Zamani Feature-Gate Compatibility Specification

Path: "grammar/compatibility/feature-gates.md"
Status: Normative
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety requirement: Production Zamani compiler implementation MUST use safe Rust; Rust "unsafe" MUST NOT be required or used.
Primary objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Scalability objective: From the smallest supported computation to arbitrarily large computations, bounded by actual program semantics, declared resource/capability requirements, implementation resources, and target resources—not by artificial language ceilings.

---

0. Document Contract

0.1 Purpose

This document defines the normative feature-gate system for Zamani.

A feature gate controls whether a language feature, syntax extension, semantic capability, dialect facility, compatibility behavior, or experimental facility is available under a particular language/compiler configuration.

A feature gate MUST provide a deterministic compatibility mechanism without becoming:

- a second language specification;
- a second grammar;
- a hardware-limit mechanism;
- a resource-limit mechanism;
- a semantic implementation;
- a backend-selection mechanism;
- a vendor-specific language;
- a replacement for language-versioning;
- a replacement for deprecation policy.

The feature-gate system exists to answer:

«Is this already-defined feature permitted in this compilation context?»

It does not answer:

«What does the feature mean?»

Feature meaning belongs to the appropriate normative specification.

---

0.2 Core Principle

The feature-gate architecture is:

feature specification
        ↓
feature identity
        ↓
feature lifecycle/status
        ↓
language-version compatibility
        ↓
explicit feature configuration, where permitted
        ↓
lexer/parser availability
        ↓
AST availability
        ↓
semantic availability
        ↓
IR availability
        ↓
compiler/backend availability
        ↓
conformance validation

A feature MUST NOT be considered implemented merely because a gate exists for it.

Likewise, a feature MUST NOT become valid merely because an implementation happens to recognize its spelling.

---

0.3 Relationship to Existing Repository Authorities

The authority relationship is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ├── language-version.md
        ├── lexical.md
        ├── syntax.md
        ├── semantics.md
        └── domain specifications
        │
        ▼
grammar/spec/
        │
        ├── compatibility.md
        ├── versioning.md
        ├── diagnostics.md
        ├── resources.md
        └── domain contracts
        │
        ▼
grammar/compatibility/
        │
        ├── versions.md
        ├── migrations.md
        ├── deprecated.md
        ├── reserved.md
        ├── compatibility-matrix.md
        └── feature-gates.md
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
canonical semantic representation
        │
        ▼
canonical IR
        │
        ├── classical IR
        ├── quantum::ir
        └── HDL/hardware IR
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

No lower layer may silently redefine the gate semantics established by a higher layer.

---

1. Ownership

1.1 This file owns

"grammar/compatibility/feature-gates.md" owns:

- feature-gate terminology;
- feature-gate lifecycle;
- gate-state semantics;
- activation rules;
- compatibility rules for gated features;
- explicit-gate syntax policy;
- compiler gate resolution policy;
- gate diagnostics;
- gate inheritance;
- gate composition;
- gate conflicts;
- gate dependency rules;
- gate removal rules;
- gate stabilization requirements;
- gate compatibility testing;
- gate scalability rules;
- gate metadata requirements.

---

1.2 This file does not own

This file does NOT own:

- language-version numbering;
- feature semantic definitions;
- grammar production definitions;
- token definitions;
- AST node definitions;
- semantic algorithms;
- canonical IR definitions;
- quantum operation semantics;
- QEC algorithms;
- ZQN implementation;
- routing;
- scheduling;
- calibration;
- HAL implementation;
- physical hardware limits;
- runtime implementation;
- vendor APIs.

Those remain owned by their respective contracts.

---

2. Existing File Integration

2.1 "grammar/specification/language-version.md"

Owns:

- language-version meaning;
- language-version availability;
- version evolution;
- relationship between language versions and feature availability.

This file MUST NOT redefine language-version numbering.

A feature gate may reference a language-version condition but MUST NOT create a competing version system.

---

2.2 "grammar/compatibility/versions.md"

Owns:

- version-policy classification;
- release compatibility;
- language/compiler separation;
- version compatibility guarantees.

This file owns the gate-specific consequences of those version rules.

---

2.3 "grammar/compatibility/migrations.md"

Owns:

- transformations between representations;
- migration procedures;
- compatibility-preserving source migration.

A feature gate may require a migration when a feature changes state, but the migration procedure itself belongs to "migrations.md".

---

2.4 "grammar/compatibility/deprecated.md"

Owns:

- deprecation lifecycle;
- deprecation diagnostics;
- removal eligibility.

A deprecated feature MAY remain gate-visible until its removal conditions are satisfied.

---

2.5 "grammar/compatibility/reserved.md"

Owns:

- reserved syntax;
- reserved identifiers;
- reserved future namespace.

A reserved feature is not equivalent to a disabled experimental feature.

---

2.6 "grammar/compatibility/compatibility-matrix.md"

Owns explicit compatibility relationships among:

- language versions;
- compiler versions;
- AST contracts;
- semantic contracts;
- IR versions;
- dialect versions;
- artifact versions;
- runtime contracts.

Feature-gate state MUST be reflected in the matrix where it affects compatibility.

---

2.7 "grammar/spec/compatibility.md"

Owns the general compatibility model.

This file specializes that model for feature availability.

The two documents MUST remain mutually consistent.

---

2.8 "grammar/spec/versioning.md"

Owns cross-layer versioning and implementation-conformance rules.

Feature gates MUST NOT become a substitute for version identifiers.

---

2.9 "grammar/Zamani.g4"

"Zamani.g4" remains the canonical ANTLR composition root.

Feature gates MUST NOT create another root grammar.

The grammar may contain feature-associated productions, but whether those productions are legal in a particular compilation context is determined through the feature-gate and language-version contracts.

---

2.10 "grammar/grammar.md"

"grammar/grammar.md" remains the implementation-conformance reference.

A feature marked in a gate registry as:

enabled

MUST NOT automatically be reported as:

IMPLEMENTED

The implementation status must still be independently established.

---

2.11 "grammar/Zamani-Grammar.md"

"Zamani-Grammar.md" remains the historical/extended/proposed language-design source.

A feature appearing there MUST NOT become enabled merely because it is described there.

Promotion remains:

proposal
    ↓
semantic design
    ↓
AST contract
    ↓
grammar contract
    ↓
implementation
    ↓
IR integration
    ↓
tests
    ↓
compatibility decision
    ↓
stable

---

2.12 "grammar/compile/feature-selection.g4"

"grammar/compile/feature-selection.g4" owns the concrete syntax for compilation-time feature selection where that syntax is part of the canonical language.

It MUST NOT redefine:

- feature lifecycle;
- compatibility classes;
- language-version semantics;
- implementation status.

This document defines what feature selection means.

---

3. Why Feature Gates Exist

Feature gates provide controlled evolution for features that are:

- experimental;
- opt-in;
- version-specific;
- dialect-specific;
- compatibility-sensitive;
- implementation-dependent;
- staged toward stabilization;
- temporarily unavailable;
- reserved for future promotion.

They are particularly important for Zamani because the language spans:

- classical computation;
- quantum computation;
- hybrid computation;
- HDL;
- hardware/software co-design;
- distributed computation;
- AI;
- tensors;
- networking;
- security;
- metaprogramming;
- temporal computation;
- Sankofa-oriented facilities;
- nano-oriented facilities;
- future computational substrates.

The gate system allows these capabilities to evolve without fragmenting Zamani into unrelated languages.

---

4. Feature Identity

Every gated feature MUST have a globally unique stable feature identifier.

Recommended form:

zmn.<domain>.<feature>

Examples:

zmn.quantum.operations
zmn.quantum.dynamic-control
zmn.quantum.mid-circuit-measurement
zmn.quantum.logical-resources
zmn.hdl.co-design
zmn.hardware.capabilities
zmn.resources.requirements
zmn.distributed.collectives
zmn.ai.training
zmn.data.tensors
zmn.networking.streaming
zmn.security.zero-knowledge
zmn.metaprogramming.reflection

Feature IDs MUST be semantic identifiers.

They MUST NOT encode:

- physical device numbers;
- hardware generations;
- maximum resource quantities;
- vendor-specific topology;
- compiler memory addresses;
- process identifiers;
- thread identifiers.

---

5. Feature ID Stability

Once a feature ID has been published as part of a compatibility contract, it MUST NOT be silently reused for a different feature.

If a feature changes meaning incompatibly, the implementation MUST choose one of:

1. preserve the ID and introduce a language-version compatibility break;
2. create a new feature ID;
3. deprecate the old feature and introduce a replacement.

Feature IDs MUST NOT be recycled.

---

6. Feature Metadata Contract

Every production feature-gate definition MUST provide, either directly or through a machine-readable feature manifest:

id
name
status
introduced
stabilized
deprecated
removed
language_versions
compiler_support
syntax_owner
semantic_owner
ast_mapping
ir_mapping
dependencies
conflicts
required_capabilities
resource_requirements
dialects
diagnostics
migration
tests
portability
scalability
determinism
security
implementation_status

The metadata MAY be represented in a feature manifest.

The canonical feature manifest location, when used, is:

grammar/specification/features/

A manifest MUST NOT contradict this document.

---

7. Feature Lifecycle

The canonical feature lifecycle is:

PROPOSED
    ↓
EXPERIMENTAL
    ↓
IMPLEMENTED_EXPERIMENTAL
    ↓
STABLE
    ↓
DEPRECATED
    ↓
REMOVED

Additional compatibility states are:

RESERVED
SPECIFIED_NOT_IMPLEMENTED
IMPLEMENTATION_DEFINED
TARGET_DEFINED
DIALECT_DEFINED

These are states, not necessarily lifecycle stages.

---

8. "PROPOSED"

A proposed feature is design material.

It:

- MAY be discussed;
- MAY appear in "Zamani-Grammar.md";
- MAY have design documents;
- MUST NOT be advertised as implemented;
- MUST NOT be required by stable programs;
- MUST NOT silently affect parsing of stable programs.

A proposed feature has no stable source compatibility guarantee.

---

9. "EXPERIMENTAL"

An experimental feature has a defined experimental contract.

It MUST:

- have a feature ID;
- have an owner;
- identify its syntax;
- identify its semantic contract;
- identify its AST mapping;
- identify its IR mapping where applicable;
- identify compatibility behavior;
- have explicit activation rules;
- have negative tests;
- have diagnostics;
- be clearly marked experimental.

Experimental syntax MUST NOT silently become active in a stable language version.

---

10. "IMPLEMENTED_EXPERIMENTAL"

This state means:

specified
+
implemented
+
not yet stable

It MUST NOT be confused with:

STABLE

The feature is available only according to its explicit gate contract.

---

11. "STABLE"

A feature may become stable only when all required production contracts exist.

Minimum completion:

Specification
    ✓
Lexical contract, if applicable
    ✓
Grammar
    ✓
Lexer
    ✓
Parser
    ✓
AST
    ✓
Semantic analysis
    ✓
Capability/resource integration
    ✓
Canonical semantic representation
    ✓
IR mapping
    ✓
Compiler integration
    ✓
Diagnostics
    ✓
Positive tests
    ✓
Negative tests
    ✓
Boundary tests
    ✓
Scalability tests
    ✓
Determinism tests
    ✓
Compatibility tests
    ✓
Migration/deprecation decision
    ✓
Documentation
    ✓

If a required item is missing, the feature MUST NOT be advertised as fully stable.

---

12. "DEPRECATED"

A deprecated feature remains recognized according to its compatibility contract.

A deprecated feature MUST identify:

- deprecation version;
- replacement;
- migration procedure;
- warning/error behavior;
- earliest permitted removal version.

Deprecation MUST NOT silently change the feature's meaning.

---

13. "REMOVED"

A removed feature is no longer valid in the applicable language contract.

Removed syntax MUST NOT be silently reinterpreted as another feature.

If encountered, it MUST produce a deterministic diagnostic when the compiler can identify the historical feature.

---

14. "RESERVED"

Reserved means:

«This feature identity or syntax space is intentionally unavailable for ordinary use.»

Reserved space exists to preserve future evolution.

A reserved feature MUST NOT accidentally become active because:

- a lexer token exists;
- an identifier happens to match;
- an ANTLR rule happens to accept it;
- a dialect imports it;
- a backend understands it.

---

15. "SPECIFIED_NOT_IMPLEMENTED"

This state means:

specified
≠
implemented

The feature may have a complete language contract but is not yet available in the implementation.

The compiler MUST produce a structured diagnostic rather than silently ignoring the construct.

---

16. Implementation-Defined Feature State

A feature may be implementation-defined only where the normative specification explicitly permits implementation choice.

The implementation MUST document its selected behavior.

Implementation-defined behavior MUST be:

- deterministic under identical configuration;
- inspectable;
- testable;
- compatible with the specification;
- independent of accidental host behavior.

---

17. Target-Defined Feature State

A feature may depend on target properties only when the language semantics explicitly permit target-defined realization.

Example:

requires capability("quantum.mid_circuit_measurement")

is a portable requirement.

Whether a target satisfies that capability is target-defined.

The feature itself is not target-defined merely because its realization differs.

---

18. Dialect-Defined Feature State

A dialect-defined feature MUST identify:

dialect name
dialect version
feature ID
syntax extension
semantic extension
AST mapping
IR mapping
compatibility rules
activation rules

A dialect MUST NOT silently redefine stable Zamani semantics.

A dialect MUST NOT create an alternate meaning for existing core syntax merely by importing a dialect.

---

19. Feature Gates Are Not Hardware Gates

This distinction is mandatory.

A feature gate may say:

zmn.quantum.dynamic-control

It MUST NOT encode:

requires 1024 qubits

as a language feature gate.

Hardware/resource feasibility belongs to:

- capability analysis;
- resource analysis;
- target descriptors;
- scheduling;
- routing;
- HAL;
- deployment.

Feature availability and resource availability are independent dimensions.

---

20. No Artificial Scalability Limits

Feature gates MUST NOT define universal maximums such as:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_TIMELINES
MAX_AGENTS

A feature gate MUST remain valid regardless of whether the actual realization contains:

- one resource;
- many resources;
- extremely large resources;
- heterogeneous resources;
- dynamically discovered resources;
- future resource classes.

The gate controls language capability.

It does not impose physical capacity.

---

21. Resource Availability Is Separate

For example:

feature:
    zmn.quantum.dynamic-control

resource requirement:
    requires capability("quantum.dynamic_control")

The feature gate answers:

Is dynamic-control syntax/semantics available?

Capability analysis answers:

Can this target provide the required capability?

Resource analysis answers:

Are sufficient resources available?

These questions MUST NOT be merged.

---

22. Requirement, Capability, Preference, Hint, Realization

Feature-gate processing MUST preserve the distinction among:

Requirement

requires capability("tensor.compute")

Resource requirement

requires memory >= required_memory

Capability

capability("quantum.measurement")

Preference

prefer accelerator("quantum")

Hint

hint locality

Realization

map logical_resource -> physical_resource

Only the first five may participate in portable source intent.

Physical realization belongs downstream.

---

23. Feature-Gate Resolution Inputs

Feature-gate resolution MAY depend on:

1. effective Zamani language version;
2. explicitly enabled experimental features;
3. explicitly disabled features;
4. compiler-supported features;
5. selected dialects;
6. dialect versions;
7. compatibility mode;
8. implementation profile;
9. target-independent compiler profile;
10. explicitly selected target contract where permitted.

Feature-gate resolution MUST NOT depend implicitly on:

- host CPU model;
- host core count;
- host memory size;
- current GPU;
- current QPU;
- current FPGA;
- current network topology;
- machine physical address;
- compilation process ID;
- thread scheduling;
- nondeterministic discovery order.

Target capability discovery belongs to later target analysis.

---

24. Effective Feature State

For every feature, the compiler MUST be able to determine an effective state.

Conceptually:

EffectiveState(feature, context)
    →
    Disabled
    Enabled
    Experimental
    Deprecated
    Reserved
    Unsupported
    Error

The actual internal Rust representation may differ.

The semantic result MUST be deterministic.

---

25. Gate Resolution Order

Gate resolution MUST follow a deterministic order.

The conceptual order is:

1. Parse configuration
2. Resolve effective language version
3. Resolve dialects
4. Resolve feature declarations
5. Load implementation-supported feature registry
6. Apply version constraints
7. Apply dialect constraints
8. Apply explicit enable/disable requests
9. Validate dependencies
10. Validate conflicts
11. Validate lifecycle state
12. Produce effective feature set
13. Make effective set available to lexer/parser/semantic analysis

The exact concrete configuration syntax belongs to the canonical grammar/specification.

---

26. No Implicit Experimental Activation

Experimental features MUST NOT become enabled merely because:

- the compiler knows about them;
- their parser rule exists;
- their lexer token exists;
- their documentation exists;
- their backend exists;
- a target supports them.

Experimental activation MUST be explicit or governed by a documented language-version contract.

---

27. No Silent Feature Downgrade

If a source program requires:

zmn.quantum.dynamic-control

and the active compiler configuration does not support it, the compiler MUST NOT silently:

- remove the operation;
- convert it to a no-op;
- ignore it;
- approximate it without permission;
- replace it with another operation;
- reinterpret it as classical control.

It MUST produce a structured diagnostic.

---

28. No Silent Feature Upgrade

Likewise, an older source program MUST NOT silently acquire new semantics merely because a newer feature exists.

Example:

old syntax

MUST retain its old specified meaning unless a language-version compatibility rule explicitly states otherwise.

---

29. Gate Dependencies

Features MAY depend on other features.

Example:

zmn.quantum.dynamic-control
    requires
zmn.quantum.measurement

Dependencies MUST be declared by feature identity.

A dependency MUST NOT be inferred from:

- source ordering;
- parser rule order;
- backend availability;
- machine topology.

If a required dependency is disabled or unsupported, the compiler MUST issue a deterministic diagnostic.

---

30. Gate Dependency Graph

The feature graph MUST be acyclic unless the specification explicitly defines a cycle-safe semantic mechanism.

Preferred structure:

core
  ↓
types
  ↓
resources
  ↓
quantum
  ↓
quantum.measurement
  ↓
quantum.dynamic-control

Not:

A → B → C → A

A cyclic gate dependency MUST be rejected during feature configuration validation unless it is explicitly defined as legal.

---

31. Gate Conflicts

Features MAY declare conflicts.

Example:

feature A
    conflicts with
feature B

Conflicts MUST be semantic or syntactic conflicts, not merely implementation inconvenience.

A backend not supporting two features simultaneously is NOT sufficient reason to make them language-level conflicting features.

If both features are semantically compatible but a particular target cannot realize them together, target analysis must report the target incompatibility.

---

32. Feature Composition

Features SHOULD compose rather than create duplicate languages.

For example:

zmn.quantum.measurement
+
zmn.quantum.dynamic-control
+
zmn.classical.control-flow

should compose through existing universal Zamani semantics.

The compiler MUST NOT require a separate:

QuantumLanguage
HybridLanguage
HDLLanguage

for ordinary domain composition.

---

33. Quantum Feature Gates

Quantum gates MUST control semantic capabilities, not enumerate physical devices.

Valid feature identities include:

zmn.quantum.operations
zmn.quantum.measurement
zmn.quantum.dynamic-control
zmn.quantum.reset
zmn.quantum.observables
zmn.quantum.channels
zmn.quantum.noise
zmn.quantum.logical-resources
zmn.quantum.error-correction
zmn.quantum.pulse-intent

A feature gate MUST NOT encode:

MAX_QUBITS
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS
MAX_GATES

---

34. Quantum Operation Gate

The language MUST avoid using feature gates as a fixed universal gate list.

The preferred semantic model is:

quantumOperation
    →
operationSpecifier
    +
quantumTargetList

Therefore:

apply H to q
apply X to q
apply custom_gate to q
apply vendor.operation to q
apply operation(parameter) to q0, q1

may share the same semantic operation model.

A feature gate may control a class of quantum operations, but it MUST NOT turn the grammar into a permanently fixed list of today's gate names.

---

35. Canonical Quantum IR

Feature gates MUST NOT create a permanent second frontend quantum IR.

The integration is:

feature-gated source
        ↓
domain-neutral frontend AST
        ↓
semantic quantum operation
        ↓
quantum::ir
        ↓
optimization
        ↓
routing
        ↓
scheduling
        ↓
resilience/QEC/ZQN
        ↓
HAL
        ↓
target

This preserves the repository's canonical "quantum::ir" boundary.

---

36. Classical Feature Gates

Classical features MAY include:

zmn.classical.numeric
zmn.classical.symbolic
zmn.classical.vector
zmn.classical.matrix
zmn.classical.tensor
zmn.classical.parallel
zmn.classical.scientific

A gate MUST NOT impose:

MAX_VECTOR_WIDTH
MAX_MATRIX_SIZE
MAX_TENSOR_RANK

as universal language limits.

---

37. HDL Feature Gates

HDL features MAY include:

zmn.hdl.modules
zmn.hdl.signals
zmn.hdl.sequential
zmn.hdl.combinational
zmn.hdl.clocking
zmn.hdl.timing
zmn.hdl.pipeline
zmn.hdl.memory
zmn.hdl.verification
zmn.hdl.co-design

A feature gate MUST NOT encode:

wire [31:0]

as a universal hardware width.

Parameterized hardware intent remains semantic.

---

38. Hardware Feature Gates

Hardware features MAY expose:

zmn.hardware.capabilities
zmn.hardware.resources
zmn.hardware.topology
zmn.hardware.interconnect
zmn.hardware.accelerator
zmn.hardware.negotiation
zmn.hardware.deployment

They MUST describe hardware intent rather than force a particular physical machine.

---

39. Distributed Feature Gates

Distributed features MAY include:

zmn.distributed.processes
zmn.distributed.services
zmn.distributed.channels
zmn.distributed.collectives
zmn.distributed.replication
zmn.distributed.partitioning
zmn.distributed.consistency
zmn.distributed.fault-tolerance

The gate system MUST NOT define:

MAX_NODES
MAX_SERVICES
MAX_CHANNELS

as language ceilings.

---

40. AI/Data Feature Gates

AI/data features MAY include:

zmn.ai.models
zmn.ai.training
zmn.ai.inference
zmn.ai.differentiation
zmn.ai.agents
zmn.data.tensors
zmn.data.datasets
zmn.data.streams

Feature gates MUST NOT couple the language to a specific AI framework.

Framework implementation belongs to interoperability/backend layers.

---

41. Networking Feature Gates

Networking features MAY include:

zmn.networking.endpoints
zmn.networking.streaming
zmn.networking.protocols
zmn.networking.service-discovery
zmn.networking.distributed-compute

No universal network-size limit may be encoded.

---

42. Security Feature Gates

Security features MAY include:

zmn.security.identity
zmn.security.authorization
zmn.security.capabilities
zmn.security.cryptography
zmn.security.signatures
zmn.security.zero-knowledge
zmn.security.secure-computation

A feature gate does not itself provide a security guarantee.

Security semantics remain owned by the security specification.

---

43. Metaprogramming Feature Gates

Features such as:

zmn.metaprogramming.reflection
zmn.metaprogramming.quotation
zmn.metaprogramming.code-generation
zmn.metaprogramming.compile-time

MUST remain subject to:

- hygiene;
- capability checks;
- deterministic expansion requirements;
- resource budgets where explicitly defined;
- source provenance;
- semantic validation.

A macro or metaprogramming feature MUST NOT bypass feature-gate validation.

---

44. Feature Gates and Macros

Macro expansion MUST NOT be used to bypass disabled feature gates.

For example:

macro expansion
    ↓
generates gated syntax

MUST still result in gate validation.

The effective feature set applies to the resulting semantic program, not merely to the user's original token stream.

---

45. Feature Gates and Generated Code

Generated Zamani code MUST carry enough compatibility context to determine:

- language version;
- dialect;
- active feature contract;
- generated-source provenance where required.

Generated code MUST NOT silently activate experimental features.

---

46. Feature Gates and Modules

Modules MUST NOT implicitly activate arbitrary features.

Importing:

module quantum

does not automatically mean:

enable every experimental quantum feature

Module imports and feature activation remain separate concepts.

---

47. Feature Gates and Packages

Packages MAY declare required features.

A package manifest may conceptually state:

requires-feature = "zmn.quantum.measurement"

The exact manifest syntax is owned by the package/build specification.

Package dependency resolution MUST NOT silently convert unsupported features into supported ones.

---

48. Feature Gates and Dialects

A dialect MAY require feature gates.

For example:

dialect.example.v1
    requires:
        zmn.core.extension

The dialect MUST NOT silently enable unrelated features.

A dialect MAY bundle a defined set of features only when the dialect specification explicitly says so.

---

49. Feature Gates and Language Versions

Language versions MAY implicitly stabilize or activate features.

For example:

Zamani 1.0
    feature A = stable

Zamani 1.1
    feature B = stable

However, language-version availability remains authoritative.

The gate system records and enforces the relationship.

It MUST NOT create a contradictory version table.

---

50. Version-Gated Features

A feature may have a version interval:

introduced: 1.2.0
stable:     1.4.0
deprecated: 2.1.0
removed:    3.0.0

These values are examples of metadata structure, not a declaration that these exact releases exist.

A feature MUST NOT be considered available outside its declared compatibility interval unless a documented compatibility mode explicitly permits it.

---

51. Feature Activation

A feature can become active through one of the following mechanisms:

51.1 Stable-by-language-version

A stable feature is automatically active when the effective language version includes it.

51.2 Explicit experimental activation

An experimental feature may require explicit activation.

51.3 Dialect activation

A dialect may activate its declared feature set.

51.4 Compatibility mode

A compatibility mode may activate historical behavior where explicitly defined.

51.5 Toolchain profile

A compiler profile may expose implementation features, provided those features are not falsely represented as language-standard features.

---

52. Explicit Feature Activation

The exact syntax for source-level feature activation is owned by the canonical grammar.

This document therefore does not establish a new syntax.

Potential implementation representations such as:

feature("...")
enable("...")
@feature("...")

MUST NOT be treated as canonical merely because they appear in documentation or experiments.

The canonical syntax must be defined once in:

grammar/specification/
grammar/spec/
grammar/Zamani.g4

and implemented consistently.

---

53. Configuration-Level Activation

Compiler/build configuration MAY enable experimental features.

Such configuration MUST be:

- explicit;
- reproducible;
- recorded in build metadata where required;
- available to diagnostics;
- available to compatibility checking;
- deterministic.

An environment variable discovered accidentally at compilation time MUST NOT silently change language semantics.

---

54. Feature-Gate Precedence

When multiple mechanisms affect a feature, precedence MUST be deterministic.

The conceptual precedence is:

language-version contract
        ↓
dialect contract
        ↓
explicit feature policy
        ↓
compiler implementation availability
        ↓
target/resource feasibility

A lower layer MUST NOT override a higher semantic contract.

For example:

target supports feature

cannot activate a feature prohibited by the language version.

---

55. Disabled Feature Behavior

When disabled syntax is encountered, the compiler SHOULD distinguish:

known feature, currently disabled

from:

unknown syntax

This allows useful diagnostics.

Preferred diagnostic structure:

feature is disabled

feature: zmn.quantum.dynamic-control
status: EXPERIMENTAL
required action: enable the feature under the supported feature policy
language version: ...

The exact diagnostic identifier belongs to "grammar/spec/diagnostics.md".

---

56. Unsupported Feature Behavior

If a feature is known but unsupported by the compiler:

feature recognized
+
specification exists
+
implementation unavailable

the compiler MUST report a structured unsupported-feature diagnostic.

It MUST NOT:

- ignore it;
- remove it;
- convert it to a no-op;
- silently lower it incorrectly.

---

57. Removed Feature Behavior

If a feature is removed:

feature recognized historically
+
feature removed from current contract

the compiler SHOULD report a removal diagnostic where historical recognition is possible.

The diagnostic SHOULD identify:

- feature ID;
- removed version;
- replacement;
- migration reference.

---

58. Reserved Feature Behavior

Reserved syntax MUST produce a reserved-feature diagnostic when the compiler can identify the reserved construct.

Reserved syntax MUST NOT be silently treated as an ordinary identifier where that would undermine the reserved-space contract.

---

59. Feature Gate Diagnostics

Diagnostics MUST be deterministic.

They SHOULD contain:

diagnostic code
feature ID
feature status
effective language version
required version
required dependencies
conflicting features
activation state
migration reference
source span

Diagnostics MUST NOT expose unstable implementation memory addresses.

---

60. Feature-Gate Diagnostic Codes

Recommended code family:

ZMN-FEATURE-UNKNOWN
ZMN-FEATURE-DISABLED
ZMN-FEATURE-EXPERIMENTAL
ZMN-FEATURE-DEPRECATED
ZMN-FEATURE-REMOVED
ZMN-FEATURE-RESERVED
ZMN-FEATURE-UNSUPPORTED
ZMN-FEATURE-DEPENDENCY
ZMN-FEATURE-CONFLICT
ZMN-FEATURE-VERSION
ZMN-FEATURE-DIALECT
ZMN-FEATURE-CONFIGURATION
ZMN-FEATURE-IMPLEMENTATION

These identifiers remain subject to the canonical diagnostics registry.

---

61. Feature-Gate Determinism

Given identical:

source
language version
dialect configuration
feature configuration
compiler feature registry

feature resolution MUST produce the same effective feature set.

It MUST NOT depend on:

- hash-map iteration order;
- thread scheduling;
- host CPU;
- memory addresses;
- machine hostname;
- current time;
- random state;
- filesystem enumeration order;
- network response order.

---

62. Feature-Gate Reproducibility

A reproducible compilation must record enough information to reconstruct the effective feature environment.

At minimum, reproducibility metadata SHOULD identify:

language version
dialect versions
feature states
compiler version
relevant specification versions
relevant IR versions
target-independent compiler profile

Target-specific realization metadata belongs to artifact/deployment contracts.

---

63. Feature Gates and POCO-REAF

Feature gates MUST preserve the distinction:

language capability
        ≠
target capability
        ≠
resource availability
        ≠
physical realization

A feature being enabled means the program may express its semantics.

It does not mean every target can execute it.

For example:

feature:
    zmn.quantum.mid-circuit-measurement

may be enabled.

A particular target may then report:

missing capability:
    quantum.mid_circuit_measurement

The program remains semantically valid.

The target is incompatible.

---

64. Scalability and Infinity

"Infinity" in the POCO-REAF objective is a scalability principle, not a requirement that a compiler represent mathematically infinite physical resources.

Feature gates MUST support arbitrarily large finite semantic programs and resource quantities where the language specification permits them.

The limiting factors may be:

- actual available memory;
- compilation resources;
- execution resources;
- target resources;
- declared resource policies;
- mathematical constraints;
- implementation representation limits.

Those limits MUST NOT be converted into artificial language ceilings.

---

65. Large Resource Values

Feature-gate metadata MUST NOT use fixed-width implementation values as the semantic definition of scalable resources.

For example, a feature requiring a quantity MUST conceptually support:

n

where "n" may be:

- a literal;
- a symbolic value;
- a compile-time expression;
- a constrained value;
- a dependent value.

The implementation MUST reject overflow rather than silently wrap.

---

66. Rust Implementation Requirement

The feature-gate implementation MUST be compatible with:

Rust 1.97
Rust 1.97.1
Rust 2021

Production implementation MUST use safe Rust.

The gate subsystem MUST NOT require:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

Feature-gate resolution SHOULD use ordinary safe Rust structures such as:

- enums;
- structs;
- owned strings;
- slices;
- maps;
- sets;
- immutable registries;
- validated configuration structures;
- deterministic graph algorithms.

The exact implementation remains outside this grammar document.

---

67. Feature-Gate Registry

The implementation SHOULD maintain one canonical registry.

Conceptually:

FeatureRegistry
    ├── feature ID
    ├── metadata
    ├── lifecycle state
    ├── version constraints
    ├── dependencies
    ├── conflicts
    └── activation policy

There MUST NOT be separate incompatible feature registries for:

- lexer;
- parser;
- AST;
- semantic analysis;
- compiler;
- quantum subsystem;
- HDL subsystem.

Subsystems may cache derived views, but the semantic feature identity MUST remain canonical.

---

68. Registry Validation

At startup or build-time, the registry SHOULD validate:

- unique feature IDs;
- valid version ranges;
- valid lifecycle transitions;
- dependency existence;
- conflict existence;
- dependency cycles;
- contradictory states;
- invalid removal intervals;
- invalid dialect references;
- invalid syntax-owner references;
- invalid semantic-owner references;
- invalid IR references.

A malformed registry MUST fail deterministically.

---

69. Lifecycle Transition Rules

Allowed transitions are:

PROPOSED
    → EXPERIMENTAL

EXPERIMENTAL
    → IMPLEMENTED_EXPERIMENTAL
    → DEPRECATED
    → REMOVED

IMPLEMENTED_EXPERIMENTAL
    → STABLE
    → DEPRECATED
    → REMOVED

STABLE
    → DEPRECATED

DEPRECATED
    → REMOVED

A feature MUST NOT transition directly from:

PROPOSED → STABLE

without the required implementation/conformance evidence.

A feature MUST NOT transition from:

REMOVED → STABLE

using the same feature ID.

Reintroduction requires an explicit compatibility decision.

---

70. Stable Promotion Gate

Before promotion to "STABLE", the feature owner MUST demonstrate:

Specification

- normative meaning;
- ownership;
- non-ownership;
- compatibility.

Syntax

- canonical grammar;
- no unintended ambiguity;
- lexical compatibility.

Frontend

- lexer;
- parser;
- AST.

Semantics

- name resolution;
- type rules;
- effect rules;
- resource rules;
- capability rules.

IR

- semantic representation;
- canonical IR mapping;
- verification.

Backend

- compiler integration;
- required lowering;
- target integration where applicable.

Diagnostics

- invalid syntax;
- disabled feature;
- unsupported implementation;
- incompatible version;
- dependency failure.

Testing

- positive;
- negative;
- boundary;
- scalability;
- determinism;
- compatibility.

---

71. Stable Features and Experimental Dependencies

A stable feature MUST NOT depend permanently on an experimental feature unless the stable specification explicitly declares that dependency and provides a stable compatibility contract.

Preferred:

stable A
    ↓
stable B

Avoid:

stable A
    ↓
experimental B

because the experimental feature could change without preserving A.

If such a relationship is temporarily required during development, A MUST remain non-stable.

---

72. Stable Features and Target Dependencies

A stable language feature MUST NOT become target-dependent merely because its first implementation targets one device.

Example:

zmn.quantum.measurement

must not become:

only supported on current QPU X

as a language-semantic rule.

Target support belongs to target capability analysis.

---

73. Compiler Profiles

Compiler profiles MAY define feature availability.

Profiles MUST be explicit.

A profile MUST NOT silently alter the meaning of stable source.

Examples of legitimate profile distinctions:

standard
experimental
compatibility
development
conformance

A profile is not a new language version.

---

74. Conformance Profile

A conformance profile SHOULD be capable of enabling only features whose implementation status is fully established.

It SHOULD reject:

- unregistered features;
- undocumented experimental features;
- contradictory feature sets;
- missing dependencies;
- unsupported stable requirements.

This provides a production validation path.

---

75. Feature Gates and "grammar.md"

"grammar.md" MUST expose feature status consistently.

For each feature, implementation conformance SHOULD distinguish:

SPECIFIED
LEXICALLY_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
SEMANTICALLY_IMPLEMENTED
IR_IMPLEMENTED
BACKEND_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL
DEPRECATED
REMOVED

A gate state and an implementation state are different dimensions.

For example:

Gate:
    ENABLED

Implementation:
    PARSER_IMPLEMENTED
    SEMANTIC_NOT_IMPLEMENTED

does not make the feature production-ready.

---

76. Feature Gates and AST

Every syntax-bearing gated feature MUST have an AST contract before it can become stable.

The mapping is:

feature-gated syntax
        ↓
canonical AST node
        ↓
semantic interpretation

A parser MUST NOT accept gated syntax that cannot be represented structurally.

---

77. Feature Gates and Semantic Analysis

Semantic analysis MUST revalidate the feature state.

This prevents feature bypass through:

- macro expansion;
- generated AST;
- deserialization;
- imported modules;
- transformed syntax;
- interoperability input.

The semantic layer MUST NOT assume that parser acceptance alone proves feature authorization.

---

78. Feature Gates and IR

Every gated feature that reaches IR MUST define:

feature
    ↓
semantic meaning
    ↓
IR representation

If no valid lowering exists, compilation MUST fail before producing an artifact that falsely represents the feature as implemented.

---

79. Feature Gates and Quantum IR

Quantum features MUST map through:

frontend AST
    ↓
semantic quantum operation
    ↓
quantum::ir

The feature-gate system MUST NOT create:

experimental quantum IR
stable quantum IR
vendor quantum IR

as competing semantic authorities.

Interoperability formats such as OpenQASM or QIR remain interchange mechanisms.

---

80. Feature Gates and HDL

HDL feature gates control language facilities such as:

- module descriptions;
- signals;
- timing;
- sequential behavior;
- combinational behavior;
- state machines;
- verification;
- co-design.

They do not control physical synthesis limits.

For example:

feature enabled:
    zmn.hdl.pipeline

does not mean:

maximum pipeline stages = N

---

81. Feature Gates and Resource Analysis

Resource requirements MUST be evaluated separately.

Example:

feature enabled:
    zmn.quantum.error-correction

program requirement:
    requires capability("quantum.error_correction")

target:
    capability unavailable

The result is:

target/resource incompatibility

not:

feature disabled

---

82. Feature Gates and Capability Negotiation

Capabilities MAY be negotiated after feature resolution.

The architecture is:

feature availability
        ↓
semantic validation
        ↓
capability requirements
        ↓
target capability discovery
        ↓
resource feasibility
        ↓
realization

This preserves POCO-REAF.

---

83. Feature Gates and Preferences

Preferences MUST NOT activate mandatory semantics.

For example:

prefer accelerator("gpu")

does not enable a GPU-only feature.

A feature requiring GPU capability must explicitly declare that requirement.

---

84. Feature Gates and Compilation

The compiler MAY perform:

- specialization;
- decomposition;
- optimization;
- vectorization;
- parallelization;
- routing;
- scheduling;
- hardware mapping;
- quantum decomposition;
- QEC lowering.

These are downstream transformations.

Feature gates MUST NOT encode backend algorithms.

---

85. Feature Gates and Runtime

Runtime feature availability MUST be distinguished from source-language feature availability.

A runtime may lack support for a realization while the source feature remains valid.

The runtime MUST return a structured execution capability/error result rather than changing program semantics.

---

86. Feature Gates and Interoperability

Imported external representations MUST be mapped into the Zamani feature model.

Examples include:

- OpenQASM;
- QIR;
- HDL formats;
- WebAssembly;
- C/C++;
- Python;
- Rust;
- other IRs.

An external format MUST NOT automatically define a new Zamani language feature.

---

87. Feature Gates and Vendor Extensions

Vendor extensions MUST use explicit namespaces.

Conceptually:

vendor.example.feature

They MUST NOT silently redefine core Zamani feature IDs.

Vendor support MUST NOT become a dependency of portable core semantics.

---

88. Feature Gate Namespacing

Reserved namespace classes SHOULD include:

zmn.core.*
zmn.classical.*
zmn.quantum.*
zmn.hybrid.*
zmn.hdl.*
zmn.hardware.*
zmn.resources.*
zmn.distributed.*
zmn.ai.*
zmn.data.*
zmn.networking.*
zmn.security.*
zmn.compile.*
zmn.execution.*
zmn.interoperability.*
zmn.macros.*
zmn.metaprogramming.*
zmn.sankofa.*
zmn.nano.*

Vendor/dialect namespaces MUST remain distinct from "zmn.*" unless explicitly authorized by the language specification.

---

89. Feature Gates and Reserved Syntax

Feature-gated syntax MUST not accidentally consume identifiers needed for future evolution.

The lexer remains governed by:

grammar/spec/lexical.md
grammar/lexer/

A domain-specific word MUST NOT become a globally reserved keyword solely because an experimental feature uses it.

Where practical, experimental features SHOULD prefer syntax that minimizes collisions with stable language syntax.

---

90. Feature Gate Compatibility Classes

Every feature SHOULD identify its compatibility class:

STABLE
COMPATIBLE_EXTENSION
EXPERIMENTAL
DEPRECATED
REMOVED
RESERVED
IMPLEMENTATION_DEFINED
TARGET_DEFINED
DIALECT_DEFINED
SPECIFIED_NOT_IMPLEMENTED

The classification must agree with "grammar/spec/compatibility.md".

---

91. Gate and Version Compatibility Matrix

A feature manifest SHOULD be expressible conceptually as:

Feature:
    id: zmn.quantum.dynamic-control

Lifecycle:
    status: EXPERIMENTAL

Availability:
    introduced: <version>
    stable: <version or none>
    deprecated: <version or none>
    removed: <version or none>

Activation:
    explicit: required

Dependencies:
    zmn.quantum.measurement

Conflicts:
    none

Implementation:
    lexer: implemented/required
    parser: implemented/required
    ast: required
    semantic: required
    ir: quantum::ir
    backend: target-dependent

Portability:
    portable: yes

Scalability:
    no artificial resource ceiling

Safety:
    safe Rust implementation

Exact version values MUST come from the actual language-version contract.

---

92. No Version Guessing

This file MUST NOT invent language versions for features.

A feature's:

introduced
stable
deprecated
removed

versions MUST be sourced from the actual versioning decision.

An unspecified version MUST be represented as unspecified rather than guessed.

---

93. Feature Configuration Validation

Before compilation, the compiler SHOULD validate:

all requested features known?
all states legal?
all version constraints satisfied?
all dialects valid?
all dependencies enabled?
all conflicts absent?
all requested stable features implemented?
all experimental features explicitly permitted?

Failure MUST be deterministic.

---

94. Feature Configuration Must Be Closed

A production compilation should operate on a resolved feature set.

Conceptually:

RequestedFeatureSet
        ↓
ValidatedFeatureSet
        ↓
EffectiveFeatureSet

After resolution, downstream compiler stages SHOULD consume the effective set rather than independently reinterpreting configuration.

This prevents:

lexer thinks feature enabled
parser thinks feature disabled
semantic layer thinks feature experimental

---

95. Feature-State Consistency

All compiler stages MUST agree on the effective state.

For every feature:

lexer_state
parser_state
ast_state
semantic_state
ir_state

must be derived from the same resolved feature identity and configuration.

A stage MUST NOT independently invent a feature state.

---

96. Gate State and Parsing

There are two valid implementation strategies:

Strategy A — parse then reject

The parser recognizes the stable syntax shape and semantic/feature validation rejects disabled features.

Strategy B — gate-aware parser

The parser conditionally recognizes the production.

Either is acceptable if:

- diagnostics remain deterministic;
- disabled syntax cannot silently change meaning;
- AST construction remains valid;
- parser ambiguity is controlled.

The canonical implementation should choose one consistent architecture.

---

97. Gate State and Lexer

Feature-specific lexing MUST be used cautiously.

A feature SHOULD NOT require globally changing token meaning unless the language specification requires it.

Prefer stable lexical categories where possible.

This reduces compatibility hazards.

---

98. Gate State and Macros

Macro expansion cannot bypass feature gates.

Validation MUST occur after expansion where generated semantics are introduced.

A macro requiring an experimental feature MUST declare or inherit that requirement according to the macro specification.

---

99. Gate State and Imports

Importing a module that contains experimental functionality MUST NOT silently activate that functionality.

The module's feature requirements must be resolved explicitly.

---

100. Gate State and Generated Artifacts

Artifacts SHOULD contain feature compatibility metadata sufficient to determine whether they can be consumed.

At minimum where relevant:

language version
feature contract/version
dialect versions
IR version
artifact format version

An artifact MUST NOT falsely claim portability across incompatible feature contracts.

---

101. Gate State and Serialization

Serialized AST/IR/artifacts MUST identify the relevant feature contract when feature semantics affect interpretation.

Deserialization MUST reject incompatible feature contracts rather than silently dropping unknown semantic information.

---

102. Unknown Features

An unknown feature identifier MUST NOT be treated as enabled.

Correct behavior:

UNKNOWN
    ↓
diagnostic

not:

UNKNOWN
    ↓
assume enabled

and not:

UNKNOWN
    ↓
ignore

---

103. Unknown Feature Forward Compatibility

An implementation MAY preserve unknown feature metadata for tooling or pass-through scenarios.

It MUST NOT claim semantic support for unknown features.

For source compilation, unknown semantic features MUST result in an appropriate diagnostic.

---

104. Feature Gate Security

Feature activation MUST NOT create an unrestricted security bypass.

Features affecting:

- code generation;
- native interop;
- external execution;
- filesystem;
- networking;
- secrets;
- cryptography;
- reflection;
- code generation;
- dynamic loading;

MUST remain subject to their respective capability/security contracts.

A feature gate is not itself a security boundary.

---

105. Feature Gates and Safe Rust

Feature activation MUST NOT require Rust "unsafe".

A feature may be semantically experimental while its compiler implementation remains entirely safe Rust.

The absence of "unsafe" is an implementation safety requirement and MUST be independently audited.

---

106. Feature Gate Audit

Production validation SHOULD search the repository for:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

and equivalent artificial ceilings.

Any occurrence MUST be classified as one of:

1. legitimate implementation/resource guard;
2. test fixture;
3. diagnostic limit;
4. security budget;
5. target-specific constraint;
6. prohibited language-level ceiling.

A legitimate implementation limit MUST NOT be represented as a language feature gate.

---

107. Feature Gates Must Not Encode Physical IDs

The feature-gate system MUST NOT contain universal identifiers such as:

gpu0
gpu1
qpu0
qpu1
qubit0
qubit1
node0
node1
core0
core1

as semantic feature identities.

Physical IDs belong to target realization.

---

108. Feature Gates Must Not Encode Fixed Widths

The gate registry MUST NOT use:

32-bit
64-bit
128-bit
1024-bit

as universal feature limits.

Widths may be semantic program parameters.

Target realization determines implementation.

---

109. Feature Gates and Numeric Literals

A feature gate may affect whether a numeric construct is available.

It MUST NOT restrict ordinary numeric literals to current machine widths merely because the implementation currently stores them in a machine integer.

Semantic quantities MUST avoid silent overflow.

---

110. Feature Gates and Tensor Shapes

A tensor feature may be enabled independently of target tensor capacity.

For example:

zmn.data.tensors

does not mean:

MAX_TENSOR_RANK = ...

Shape feasibility is determined separately.

---

111. Feature Gates and Quantum Resource Counts

A quantum feature may be enabled independently of available qubits.

For example:

zmn.quantum.logical-resources

does not establish:

maximum logical qubits

A program may require:

requires qubits >= n

and target analysis determines whether that requirement can be satisfied.

---

112. Feature Gates and Distributed Scale

A distributed feature may be enabled regardless of the number of nodes.

The language expresses:

distributed computation

not:

maximum nodes = N

The deployment system determines realization.

---

113. Feature Gates and HDL Scale

HDL feature activation must not establish fixed:

- register widths;
- memory capacities;
- pipeline lengths;
- device counts;
- module counts.

These are parameters or target constraints.

---

114. Feature Gates and Deterministic Compilation

Feature configuration MUST be part of deterministic compilation inputs.

Given identical:

source
specification
language version
feature configuration
dialect configuration
compiler version
relevant toolchain contracts

the compiler MUST resolve the same effective feature set.

---

115. Feature Gates and Reproducible Builds

Reproducible builds MUST record feature configuration sufficiently to reproduce:

source semantics
+
feature semantics
+
dialect semantics
+
language-version semantics

Backend-specific target metadata may be recorded separately.

---

116. Feature Gates and Compatibility Modes

Compatibility modes MAY preserve historical source behavior.

A compatibility mode MUST:

- identify the historical contract;
- be explicit;
- be deterministic;
- be documented;
- have migration guidance;
- not silently alter new source semantics.

Compatibility mode is not permission to introduce arbitrary legacy behavior.

---

117. Feature Gates and Migrations

When a feature is replaced:

old feature
    ↓
deprecated
    ↓
migration
    ↓
new feature

The migration MUST preserve semantics where the replacement is claimed compatible.

The migration procedure belongs to:

grammar/compatibility/migrations.md

This file only records the gate-state transition.

---

118. Feature Gates and Deprecation

Deprecation requires coordination among:

feature-gates.md
deprecated.md
migrations.md
versions.md
compatibility-matrix.md

No feature may be removed from implementation merely because its gate was changed to deprecated.

Removal requires the complete removal contract.

---

119. Feature Gates and Reserved Space

When a feature is removed, its identifier MAY become reserved.

It MUST NOT immediately be reused for unrelated semantics.

This preserves long-term compatibility and avoids confusing historical source.

---

120. Feature Gate Change Procedure

Any feature-gate change MUST follow:

1. Identify feature ID
2. Identify current state
3. Identify requested new state
4. Identify language-version impact
5. Identify source compatibility impact
6. Identify AST impact
7. Identify semantic impact
8. Identify IR impact
9. Identify dialect impact
10. Identify migration impact
11. Identify diagnostic impact
12. Update compatibility matrix
13. Update tests
14. Update implementation conformance
15. Update documentation
16. Validate repository-wide consistency

---

121. Feature Gate Completion Contract

A feature-gate entry is complete only when it specifies:

Purpose
Owns
Does not own
Feature ID
Lifecycle
Language-version relationship
Activation mechanism
Dependencies
Conflicts
Syntax owner
AST owner
Semantic owner
IR owner
Compiler integration
Runtime/target integration
Diagnostics
Migration
Deprecation behavior
Scalability
Portability
Determinism
Security
Positive tests
Negative tests
Boundary tests
Scalability tests
Compatibility tests
Completion criteria

This satisfies the repository requirement that a file be independently complete and already contain its downstream integration contract.

---

122. Production Feature Checklist

Before marking a feature "STABLE", verify:

[ ] Unique feature ID
[ ] Normative specification exists
[ ] Owner identified
[ ] Non-owner boundaries identified
[ ] Language-version relationship defined
[ ] Syntax contract exists
[ ] Lexer contract exists where required
[ ] Parser contract exists
[ ] AST contract exists
[ ] Semantic contract exists
[ ] Type/effect integration defined
[ ] Capability/resource integration defined
[ ] Canonical semantic representation defined
[ ] IR mapping defined
[ ] quantum::ir mapping defined where quantum-related
[ ] HDL/hardware mapping defined where applicable
[ ] Compiler integration defined
[ ] Runtime integration defined where applicable
[ ] Target integration defined where applicable
[ ] Dependencies defined
[ ] Conflicts defined
[ ] Diagnostics defined
[ ] Migration defined where needed
[ ] Deprecation behavior defined
[ ] Compatibility matrix updated
[ ] Positive tests exist
[ ] Negative tests exist
[ ] Boundary tests exist
[ ] Scalability tests exist
[ ] Determinism tests exist
[ ] Compatibility tests exist
[ ] No artificial hardware ceiling
[ ] No artificial resource ceiling
[ ] No silent semantic loss
[ ] No silent fallback
[ ] Safe Rust implementation
[ ] Documentation complete
[ ] grammar.md conformance updated

---

123. Feature-Gate Test Categories

Every feature gate SHOULD have:

tests/
    positive/
    negative/
    boundary/
    scalability/
    compatibility/
    determinism/

Where applicable also:

    dialect/
    migration/
    diagnostics/
    security/
    interoperability/

---

124. Positive Tests

Positive tests prove:

- valid feature configuration;
- valid syntax;
- valid semantics;
- valid feature composition;
- valid version selection;
- valid dialect activation.

---

125. Negative Tests

Negative tests MUST cover:

- unknown feature;
- disabled feature;
- unsupported feature;
- missing dependency;
- conflicting features;
- invalid version;
- invalid dialect;
- removed feature;
- reserved feature;
- malformed configuration.

---

126. Boundary Tests

Boundary tests SHOULD include:

- first supported language version;
- last compatible language version;
- transition into experimental status;
- transition into stable status;
- transition into deprecated status;
- transition into removed status;
- empty feature set;
- maximum configured feature set supported by the implementation;
- deeply composed but semantically valid feature dependency graphs.

The tests MUST NOT define artificial universal language limits.

---

127. Scalability Tests

Scalability tests SHOULD vary:

- number of enabled features;
- dependency graph size;
- module count;
- domain count;
- quantum resource expressions;
- tensor dimensions;
- distributed resource descriptions;
- HDL parameter dimensions.

Tests must distinguish:

semantic scalability

from:

available implementation resources

A resource-exhaustion failure must not be reported as a feature incompatibility.

---

128. Determinism Tests

For identical configuration:

effective feature set

must be identical.

Test:

same input
same version
same feature configuration
same dialect configuration
→ same resolution

Repeated runs must not depend on ordering accidents.

---

129. Compatibility Tests

Compatibility tests MUST verify:

old source
    +
compatible compiler
    →
same specified semantics

when compatibility is promised.

They MUST also verify that incompatible changes produce diagnostics rather than silent reinterpretation.

---

130. Cross-Domain Feature Tests

At least one integration test should exercise combinations such as:

classical
+
quantum
+
resource requirement
+
capability requirement

and:

classical
+
HDL
+
hardware capability

and:

quantum
+
classical feed-forward
+
dynamic control

The purpose is to verify that feature gates compose through the universal language rather than producing independent sublanguages.

---

131. Quantum Integration Test

A production quantum gate test should conceptually verify:

feature:
    zmn.quantum.dynamic-control

source:
    measurement
        ↓
    classical condition
        ↓
    quantum operation

frontend:
    AST

semantic:
    quantum operation + classical control

IR:
    quantum::ir

downstream:
    optimization
    routing
    scheduling
    resilience/QEC/ZQN
    HAL

No fixed gate enumeration or physical-qubit limit belongs in the feature-gate test.

---

132. HDL Integration Test

A production HDL gate test should verify:

feature:
    zmn.hdl.co-design

source:
    algorithm
    +
hardware intent

AST:
    domain-neutral representation

semantic:
    computation + hardware intent

IR:
    canonical hardware/co-design representation

downstream:
    synthesis/lowering/target realization

The test MUST NOT encode a universal fixed register width or memory capacity.

---

133. POCO-REAF Integration Test

At least one compatibility suite SHOULD verify the same semantic source program against multiple target descriptions.

Conceptually:

one Zamani source
        │
        ▼
canonical semantic model
        │
        ├── CPU target
        ├── GPU target
        ├── FPGA target
        ├── QPU target
        ├── simulator target
        └── future-target descriptor

The feature gate remains unchanged.

Target realization changes.

---

134. Feature Gates and Future Hardware

A new hardware class MUST NOT require a new language merely because the hardware is new.

The preferred process is:

new hardware
    ↓
target descriptor
    ↓
capability declaration
    ↓
resource model
    ↓
lowering/backend

A new feature gate is required only when the programming model itself introduces a new language capability.

---

135. Feature Gates and Future Computational Models

The same principle applies to future computation:

new computation model
        ↓
semantic capability
        ↓
feature specification
        ↓
feature gate
        ↓
AST/semantic/IR integration

The feature must integrate with the universal Zamani language rather than create an unrelated parser.

---

136. No Backend-Driven Feature Gates

A backend MUST NOT create a language feature merely because it needs an internal switch.

Internal compiler flags belong to implementation configuration.

Language feature gates are only for externally observable language contracts.

---

137. No Optimization Feature Leakage

An optimization such as:

vectorization
fusion
gate cancellation
instruction scheduling

is not automatically a language feature.

If optimization changes only implementation while preserving semantics, it does not require a source-language feature gate.

---

138. Semantic Optimization Gates

A gate is appropriate when the programmer can observe or rely upon a semantic property.

For example:

strict floating-point semantics
deterministic execution mode
specific numerical semantics

may require explicit language/semantic configuration.

Ordinary backend optimizations do not.

---

139. Feature Gates and Deterministic Semantics

If a feature changes determinism, the gate metadata MUST state that explicitly.

Examples:

deterministic
nondeterministic
implementation-defined
target-defined

The compiler MUST NOT silently switch a deterministic feature into nondeterministic behavior.

---

140. Feature Gates and Resource Exhaustion

If a feature is enabled but actual resources are insufficient:

feature:
    ENABLED

resource:
    INSUFFICIENT

The compiler/runtime MUST preserve this distinction.

Correct result:

resource/capability diagnostic

not:

feature disabled

---

141. Feature Gates and Retry/Recovery

Runtime recovery behavior such as:

RETRY
RECOVER
DEGRADED_ACCEPT
ESCALATE
REJECT

belongs to resilience/runtime contracts.

A feature gate may enable a resilience semantic capability but MUST NOT implement runtime recovery itself.

---

142. Feature Gates and ZQN

ZQN remains responsible for quantum fault/noise semantics.

A feature gate may enable a ZQN-related language facility.

It MUST NOT duplicate ZQN semantics inside the compatibility subsystem.

---

143. Feature Gates and QEC

QEC remains responsible for quantum error correction.

A gate such as:

zmn.quantum.error-correction

controls language availability.

The QEC subsystem determines implementation.

---

144. Feature Gates and Routing

Routing is downstream realization.

A feature gate MUST NOT hard-code:

topology = ...
qubit adjacency = ...

The program may declare topology requirements.

Routing determines physical realization.

---

145. Feature Gates and Scheduling

Scheduling is downstream realization.

A feature gate MUST NOT encode fixed:

clock count
cycle count
thread count
execution slots

unless such quantities are genuinely part of language semantics.

---

146. Feature Gates and Calibration

Calibration is target/device state.

Feature gates MUST NOT encode:

- calibration constants;
- physical device identifiers;
- current device parameters.

Those belong to target/HAL/runtime metadata.

---

147. Feature Gates and Provenance

Feature resolution SHOULD be included in provenance for reproducible builds.

The provenance record SHOULD allow reconstruction of:

which feature contract was active
which language contract was active
which dialect contract was active

without requiring knowledge of a particular physical machine.

---

148. Feature-Gate File Format

This document defines the semantic contract.

If machine-readable manifests are introduced, they SHOULD be stored under:

grammar/specification/features/

The manifest format MUST be stable and versioned independently from source-language syntax.

YAML, TOML, JSON, or another format MAY be selected by the repository tooling.

The choice of file format MUST NOT change feature semantics.

---

149. Feature Manifest Minimum Schema

A machine-readable feature manifest SHOULD contain:

id
name
status
description

introduced
stable
deprecated
removed

language_versions

activation
explicit
implicit
dialect

dependencies
conflicts

syntax
lexer
parser
ast
semantic
ir

compiler
runtime
target

capabilities
resources

diagnostics
migration

portable
deterministic
scalable

tests

Optional fields MUST NOT change the meaning of required fields.

---

150. Feature Manifest Example

Illustrative structure:

id: zmn.quantum.dynamic-control

name: Quantum Dynamic Control

status: experimental

activation:
  explicit: true

dependencies:
  - zmn.quantum.measurement

conflicts: []

syntax_owner:
  - grammar/quantum/

ast_owner:
  - src/frontend/ast/

semantic_owner:
  - semantic quantum subsystem

ir:
  - quantum::ir

capabilities:
  - quantum.dynamic_control

resources: []

portable: true
deterministic: true
scalable: true

implementation:
  safe_rust: true

This is a metadata example only.

The actual feature registry remains authoritative.

---

151. Feature Manifest Must Not Become a Second Grammar

A manifest MUST NOT contain arbitrary grammar productions as its semantic definition.

It may reference:

grammar/quantum/...
grammar/Zamani.g4

but grammar ownership remains with the canonical grammar.

---

152. Feature Manifest Must Not Become a Second Semantic Specification

A manifest may summarize semantics.

It MUST reference the authoritative semantic specification rather than redefining it.

---

153. Feature Gate and Source Version

Every source compilation MUST have an effective language-version context.

Feature resolution without language-version resolution is invalid unless the language specification explicitly defines a versionless mode.

The compiler MUST NOT guess a version from arbitrary feature usage.

---

154. Legacy Source

Legacy source without an explicit version MAY use the documented legacy-default mechanism.

That mechanism belongs to:

grammar/specification/language-version.md
grammar/compatibility/versions.md

This document merely applies feature-gate rules after the effective version is determined.

---

155. Feature Gates and Source Spans

Every diagnostic involving source syntax SHOULD preserve:

- file identity;
- start position;
- end position;
- feature ID;
- relevant version.

Feature-gate errors MUST integrate with the repository's source-span model.

---

156. Feature Gates and Error Recovery

Parser recovery MUST NOT turn disabled feature syntax into unrelated valid syntax merely to continue parsing.

For example:

disabled quantum construct

must not be recovered as:

ordinary identifier expression

if doing so changes the interpretation of the source.

---

157. Feature Gates and Ambiguity

Adding a gated feature MUST undergo ambiguity testing against all active syntax.

A disabled feature MUST NOT introduce parse ambiguity into stable syntax.

An experimental feature SHOULD be designed so its activation does not silently change parsing of existing stable programs.

---

158. Lexical Compatibility

A gated feature MUST NOT require a globally reserved keyword unless the lexical contract explicitly permits it.

If a new keyword would break existing identifiers, the change MUST undergo compatibility review.

The lexer contract remains authoritative.

---

159. Feature Gates and Operator Compatibility

New gated operators MUST be checked for:

- lexical collision;
- precedence collision;
- associativity;
- parsing ambiguity;
- existing source compatibility;
- dialect compatibility.

A feature gate MUST NOT hide an operator conflict.

---

160. Feature Gates and Type Compatibility

A gated type feature MUST identify:

type syntax
type semantics
generic interaction
conversion rules
ownership rules
effect rules
resource rules
IR representation
compatibility behavior

The feature gate only controls availability.

---

161. Feature Gates and Effects

A gated effect feature MUST define its relationship with the effect system.

It MUST NOT silently introduce effects unknown to semantic analysis.

---

162. Feature Gates and Capabilities

A feature may require language-level capabilities.

Those capabilities are semantic requirements.

They are not physical device identities.

Example:

requires capability("tensor.compute")

is valid architectural intent.

---

163. Feature Gates and Resource Expressions

A feature may make resource-expression syntax available.

For example:

requires memory >= required_memory

The feature gate controls the language facility.

The resource system determines meaning.

---

164. Feature Gates and Hardware Topology

A topology feature may allow:

requires topology(...)

The gate MUST NOT hard-code a topology.

Target analysis determines whether the topology requirement is satisfiable.

---

165. Feature Gates and Device Negotiation

Device negotiation occurs after semantic validation.

The architecture is:

feature gate
    ↓
program semantics
    ↓
capability requirement
    ↓
resource requirement
    ↓
target negotiation
    ↓
realization

---

166. Feature Gates and Cloud/Distributed Targets

Cloud execution may provide different target realizations.

A cloud target MUST NOT change feature semantics merely because it is remote.

Feature availability remains a language/compiler contract.

---

167. Feature Gates and Embedded Targets

Embedded targets may have constrained resources.

The compiler MUST distinguish:

feature available

from:

target cannot satisfy resource requirement

A constrained embedded device does not define a smaller Zamani language.

---

168. Feature Gates and Future Targets

A future target can implement existing stable features without modifying the source language merely because its hardware is new.

This is essential to POCO-REAF.

---

169. Feature-Gate Governance

Every stable feature SHOULD have:

feature owner
specification owner
implementation owner
compatibility owner
test coverage

Ownership metadata MUST NOT change semantic authority.

---

170. Repository-Wide Integration Checklist

When adding or changing a feature gate, inspect at minimum:

grammar/DESIGN.md
grammar/README.md

grammar/specification/language-version.md

grammar/spec/compatibility.md
grammar/spec/versioning.md
grammar/spec/diagnostics.md
grammar/spec/resources.md

grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/compatibility/reserved.md
grammar/compatibility/compatibility-matrix.md
grammar/compatibility/feature-gates.md

grammar/compile/feature-selection.g4

grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md

grammar/lexer/
grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/declarations/

affected domain grammar

src/lexer.rs
src/parser.rs
src/frontend/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs

affected IR subsystem
affected compiler subsystem
affected runtime/HAL subsystem

grammar/validation/
grammar/tests/

Only the affected downstream files need modification, but the compatibility impact must be evaluated across this chain.

---

171. Feature-Gate Change Record

Every change SHOULD record:

feature ID
old state
new state
reason
language-version impact
source compatibility
semantic compatibility
AST impact
IR impact
dialect impact
migration
tests

This can be represented in release notes, migration metadata, or repository change records.

---

172. What Counts as a Breaking Gate Change

The following are potentially breaking:

- changing stable syntax meaning;
- disabling stable syntax;
- changing stable semantics;
- changing stable type meaning;
- changing stable effect meaning;
- changing stable resource meaning;
- changing stable quantum meaning;
- changing stable HDL meaning;
- changing feature activation required for previously valid stable source.

Such changes require compatibility review.

---

173. What Does Not Normally Count as a Language Break

The following do not normally require a language-version break when semantics remain unchanged:

- new backend;
- new CPU target;
- new GPU target;
- new FPGA target;
- new QPU target;
- new simulator;
- new accelerator;
- optimization improvements;
- routing improvements;
- scheduling improvements;
- QEC implementation improvements;
- ZQN implementation improvements;
- HAL implementation improvements;
- new target descriptors.

---

174. Feature Gate and Optimization Independence

Optimization MAY be independently enabled or disabled without becoming a source-language feature gate when it preserves semantics.

For example:

optimization:
    gate-cancellation

need not become:

language feature:
    zmn.quantum.gate-cancellation

unless the source language exposes observable semantics dependent on it.

---

175. Feature Gates and Compiler Internal Flags

Internal flags MUST remain separate from public language feature IDs.

Do not expose implementation switches such as:

use_new_parser
enable_internal_pass_7
backend_fast_path

as language features unless they have a documented semantic contract.

---

176. Feature-Gate Stability Rule

A feature gate is stable only when its behavior is stable.

A stable feature gate MUST NOT mean:

enabled today
maybe different tomorrow

It means the feature's language contract is stable.

Implementation may improve while preserving semantics.

---

177. Long-Term Evolution

Zamani is intended to evolve indefinitely.

Therefore feature gates MUST optimize for:

- namespace stability;
- semantic stability;
- migration;
- extensibility;
- backward compatibility;
- forward compatibility where explicitly supported;
- interoperability;
- deterministic behavior;
- target independence.

The system MUST NOT depend on temporary hardware generations.

---

178. Production Invariants

The following are mandatory invariants:

I1  One canonical Zamani language.
I2  One canonical feature identity per feature.
I3  No feature ID reuse.
I4  No silent activation of experimental features.
I5  No silent semantic downgrade.
I6  No silent semantic upgrade.
I7  No silent removal.
I8  No feature bypass through macros.
I9  No feature bypass through generated AST/IR.
I10 No artificial hardware limits.
I11 No artificial resource limits.
I12 No target-specific semantics in portable feature gates.
I13 No duplicate quantum semantic IR.
I14 quantum features integrate through quantum::ir.
I15 Stable features require complete semantic integration.
I16 Gate resolution is deterministic.
I17 Feature configuration is reproducible.
I18 Unknown features are rejected.
I19 Missing dependencies are rejected.
I20 Conflicting features are rejected.
I21 Removed features are diagnosed.
I22 Reserved features remain unavailable.
I23 Rust implementation uses safe Rust.
I24 Language version and compiler version remain separate.
I25 Feature availability and target capability remain separate.
I26 Resource feasibility and feature availability remain separate.
I27 grammar.md reports implementation status rather than inventing language authority.
I28 Zamani-Grammar.md does not silently promote features.

---

179. Production Readiness Gate

"grammar/compatibility/feature-gates.md" is considered integrated only when:

[ ] grammar/specification/language-version.md agrees
[ ] grammar/spec/compatibility.md agrees
[ ] grammar/spec/versioning.md agrees
[ ] grammar/compatibility/versions.md agrees
[ ] grammar/compatibility/migrations.md agrees
[ ] grammar/compatibility/deprecated.md agrees
[ ] grammar/compatibility/reserved.md agrees
[ ] grammar/compatibility/compatibility-matrix.md agrees
[ ] grammar/compile/feature-selection.g4 agrees
[ ] grammar/Zamani.g4 has one canonical composition path
[ ] grammar/grammar.md can report feature implementation status
[ ] Zamani-Grammar.md cannot silently activate features
[ ] lexer behavior is deterministic
[ ] parser behavior is deterministic
[ ] AST mappings exist
[ ] semantic mappings exist
[ ] IR mappings exist
[ ] quantum features use quantum::ir
[ ] resource/capability analysis remains separate
[ ] target realization remains downstream
[ ] diagnostics are defined
[ ] migration behavior is defined
[ ] positive tests exist
[ ] negative tests exist
[ ] boundary tests exist
[ ] scalability tests exist
[ ] determinism tests exist
[ ] compatibility tests exist
[ ] no artificial scalability ceilings exist
[ ] production Rust implementation remains safe Rust

---

180. Final Architectural Contract

The production feature-gate architecture is:

                         Zamani Source
                              │
                              ▼
                    Effective Language Version
                              │
                              ▼
                       Feature Resolution
                              │
             ┌────────────────┼────────────────┐
             │                │                │
         Core Gates      Domain Gates      Dialect Gates
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                       Canonical Grammar
                              │
                              ▼
                            Lexer
                              │
                              ▼
                           Parser
                              │
                              ▼
                       Domain-Neutral AST
                              │
                              ▼
                    Semantic Validation
                              │
             ┌────────────────┼────────────────┐
             │                │                │
           Types           Effects       Capabilities
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                       Resource Analysis
                              │
                              ▼
                  Canonical Semantic Model
                              │
             ┌────────────────┼────────────────┐
             │                │                │
        Classical        quantum::ir        HDL/Hardware
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                         Optimization
                              │
             ┌────────────────┼────────────────┐
             │                │                │
          Routing         Scheduling       Resilience
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                             ZQN
                              │
                             HAL
                              │
                              ▼
                       Target Realization
                              │
          ┌───────────┬──────┼──────┬────────────┐
          │           │      │      │            │
         CPU         GPU    FPGA    QPU      Future Target

The feature-gate subsystem therefore controls language capability availability, while the rest of the Zamani architecture determines meaning, resources, capabilities, optimization, realization, and execution.

The governing rule is:

«A feature gate may restrict when a defined language capability is available; it must never turn today's hardware, compiler, backend, or resource limits into tomorrow's language limits.»

The resulting separation is:

FEATURE AVAILABILITY
        ≠
SEMANTIC MEANING
        ≠
CAPABILITY AVAILABILITY
        ≠
RESOURCE AVAILABILITY
        ≠
TARGET REALIZATION
        ≠
PHYSICAL HARDWARE

That separation is what allows Zamani to preserve its intended:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

architecture while still permitting controlled experimental evolution, stable language releases, dialects, compatibility modes, quantum/classical/HDL integration, future computational models, and indefinite scaling subject to actual available resources.