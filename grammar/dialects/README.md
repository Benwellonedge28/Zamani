Zamani Dialects Grammar

Path: "grammar/dialects/README.md"
Project: Zamani Universal Programming Language
Repository: "Benwellonedge28/Zamani"
Language: Zamani
Grammar technology: ANTLR4-compatible grammar
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Safety: Safe Rust only; Rust "unsafe" is prohibited
Architecture: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

The "grammar/dialects/" directory defines the source-language contract for Zamani dialects.

A dialect is a controlled, explicitly identified extension of the Zamani language.

Dialects provide a mechanism for extending Zamani without creating another language, another parser architecture, another AST architecture, another semantic universe, or another IR.

A dialect may extend or specialize Zamani for areas including:

- classical computing;
- quantum computing;
- hybrid quantum/classical computing;
- HDL;
- hardware/software co-design;
- embedded computing;
- accelerators;
- AI/ML;
- tensor computing;
- scientific computing;
- numerical computing;
- symbolic computing;
- parallel computing;
- HPC;
- distributed computing;
- networking;
- cryptography;
- security;
- data processing;
- edge/cloud computing;
- nano-oriented computing;
- temporal computation;
- future computational paradigms;
- organization-specific extensions;
- vendor-specific extensions;
- experimental language facilities.

The dialect system is deliberately open-world.

Adding a new dialect must not require modifying the core dialect grammar merely to add the dialect's name to an enumeration.

The central principle is:

«Zamani is one language. Dialects extend that language; they do not replace it.»

---

2. Architectural Position

The dialect subsystem is part of the following repository-wide pipeline:

Zamani source
    │
    ▼
canonical lexer
    │
    ▼
grammar/Zamani.g4
    │
    ▼
dialect parser boundary
    │
    ▼
grammar/dialects/dialect.g4
    │
    ├── registration.g4
    ├── namespaces.g4
    ├── versioning.g4
    ├── capabilities.g4
    ├── compatibility.g4
    ├── extension-points.g4
    ├── vendor.g4
    └── experimental.g4
    │
    ▼
domain-neutral frontend AST
    │
    ▼
structural validation
    │
    ▼
name / module / dialect resolution
    │
    ▼
type / effect / resource / capability analysis
    │
    ▼
dialect semantic model
    │
    ▼
canonical semantic representation
    │
    ├── classical representation
    ├── quantum::ir
    ├── HDL / hardware representation
    ├── data / tensor representation
    ├── AI representation
    ├── distributed representation
    └── other canonical domain representations
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
HAL / backend / deployment
    │
    ▼
target realization
    │
    ├── CPU
    ├── GPU
    ├── FPGA
    ├── ASIC
    ├── QPU
    ├── accelerator
    ├── simulator
    ├── embedded system
    ├── distributed system
    └── future target

The dialect grammar participates in the source syntax stage only.

It does not execute programs, discover hardware, allocate resources, route quantum operations, schedule workloads, perform QEC, or select physical devices.

---

3. Normative Authority

The dialect subsystem participates in the repository's single-authority model.

The authority hierarchy is:

grammar/DESIGN.md
        │
        ▼
language specification
        │
        ├── specification/*
        └── spec/*
        │
        ▼
canonical lexical contracts
        │
        ▼
canonical grammar
        │
        ▼
grammar/Zamani.g4
        │
        ▼
dialect grammar components
        │
        ▼
lexer / parser implementation
        │
        ▼
frontend AST
        │
        ▼
semantic analysis
        │
        ▼
canonical IR
        │
        ▼
compiler / runtime / target

The following ownership remains mandatory.

File| Primary responsibility
"grammar/DESIGN.md"| Overall grammar architecture and boundaries
"grammar/README.md"| Grammar navigation and authority model
"grammar/Zamani.g4"| Canonical ANTLR composition/root grammar
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Historical/extended design reference
"grammar/dialects/README.md"| Dialect architecture and contracts
"grammar/dialects/dialect.g4"| Public dialect parser boundary
"grammar/dialects/registration.g4"| Dialect registration syntax
"grammar/dialects/namespaces.g4"| Dialect namespace context
"grammar/dialects/versioning.g4"| Dialect-specific version attachment
"grammar/dialects/capabilities.g4"| Dialect capability adapters
"grammar/dialects/compatibility.g4"| Dialect compatibility declarations
"grammar/dialects/extension-points.g4"| Extension attachment points
"grammar/dialects/vendor.g4"| Explicit vendor-extension syntax
"grammar/dialects/experimental.g4"| Explicit experimental-extension syntax
"grammar/dialects/registry.md"| Dialect registry contract
"grammar/compatibility/*"| Version, migration and deprecation policy

No dialect document may silently override a higher-level normative contract.

---

4. One Language, Not Many

A dialect MUST NOT become an alternative Zamani language.

The following architecture is prohibited:

Zamani
 ├── Zamani language
 ├── Quantum language
 ├── HDL language
 ├── GPU language
 └── Vendor language

The required architecture is:

                       Zamani
                          │
             ┌────────────┼────────────┐
             │            │            │
         classical      quantum       HDL
             │            │            │
             └────────────┼────────────┘
                          │
                     hybrid semantics
                          │
             ┌────────────┼────────────┐
             │            │            │
             AI       distributed    data
             │            │            │
             └────────────┼────────────┘
                          │
                    common semantics
                          │
                  canonical IR layer

Every dialect therefore shares Zamani's:

- lexical model;
- names;
- expressions;
- declarations;
- statements;
- types;
- effects;
- ownership;
- resource model;
- capability model;
- module model;
- diagnostics;
- source spans;
- versioning;
- compatibility;
- semantic validation;
- canonical IR boundaries.

A dialect may add meaning only through an explicitly declared extension contract.

---

5. Dialect Definition

A dialect is a named, versioned semantic extension contract.

At minimum, a production dialect contract identifies:

identity
version
owner
status
dependencies
namespace
syntax extensions
semantic extensions
AST mapping
semantic mapping
IR mapping
capabilities
requirements
compatibility
feature gates
diagnostics
migration policy
provenance

A dialect MAY additionally define:

constraints
preferences
hints
extension points
lowering contracts
interoperability contracts
deprecation information
security requirements
determinism requirements
resource requirements
target-independent execution requirements

A dialect MUST NOT use registration metadata to smuggle implementation-specific behavior into source semantics.

---

6. Open-World Dialect Model

Dialect identity is symbolic.

The grammar MUST NOT contain a closed enumeration such as:

dialect
    : QUANTUM
    | OPENQASM
    | CUDA
    | VERILOG
    | VENDOR_X
    ;

That design does not scale.

Instead, a dialect identity is represented by the canonical Zamani name/path system.

Examples include:

quantum::standard
quantum::openqasm
classical::numeric
classical::scientific
hybrid::quantum_classical
hdl::rtl
hardware::fpga
distributed::messaging
ai::tensor
data::stream
organization::domain::dialect
vendor::domain::extension
future::computing::dialect

These are examples of structure, not a built-in list of approved dialects.

The grammar must not need modification when a new dialect is introduced.

Whether a dialect actually exists, is visible, is trusted, is compatible, or is usable is determined by semantic and registry infrastructure.

---

7. Dialect Identity

A dialect identity MUST be:

- explicit;
- stable;
- namespace-aware;
- versionable;
- resolvable;
- comparable;
- compatible with repository/module resolution;
- independent of physical hardware identity.

A dialect identity MUST NOT be equivalent to:

CPU model
GPU model
FPGA part number
ASIC identifier
QPU identifier
physical qubit identifier
machine address
network node identifier
memory-bank identifier
deployment location

For example:

quantum::standard

identifies a language contract.

It does not identify a particular QPU.

---

8. Dialect Ownership

Every production dialect MUST have an explicit owner.

Ownership identifies who maintains the dialect contract.

Ownership may represent:

- the Zamani project;
- an organization;
- a standards body;
- a research group;
- a vendor;
- another explicitly recognized authority.

Ownership does not grant permission to modify Zamani core semantics.

A dialect owner is responsible for:

- dialect specification;
- versioning;
- compatibility declarations;
- migration policy;
- semantic definitions;
- extension maintenance;
- conformance tests;
- diagnostics;
- provenance.

Vendor ownership MUST NOT turn vendor syntax into implicit Zamani core syntax.

---

9. Dialect Lifecycle

Every dialect has a lifecycle state.

The lifecycle vocabulary is:

proposed
experimental
stable
deprecated
historical
rejected
removed

A dialect declaration MUST NOT imply "stable" merely because the parser accepts it.

A production dialect is stable only when its complete contract exists:

specification
    ↓
lexical contract
    ↓
grammar
    ↓
AST contract
    ↓
semantic contract
    ↓
IR contract
    ↓
compiler/lowering contract
    ↓
runtime contract where applicable
    ↓
positive tests
    ↓
negative tests
    ↓
boundary tests
    ↓
scalability tests
    ↓
compatibility tests
    ↓
diagnostic tests
    ↓
determinism tests where applicable
    ↓
hard-coding audit
    ↓
production acceptance

---

10. Dialect Status Does Not Change Automatically

A dialect MUST NOT silently transition:

experimental → stable
stable → deprecated
deprecated → removed

A lifecycle transition requires an explicit compatibility decision and corresponding documentation.

The existing repository compatibility policy remains authoritative:

- "grammar/compatibility/versions.md"
- "grammar/compatibility/migrations.md"
- "grammar/compatibility/deprecated.md"

Dialect lifecycle changes MUST be represented consistently with those contracts.

---

11. Syntax Extension Contract

A dialect may introduce source syntax only through an explicitly defined extension point.

The dialect grammar MUST distinguish:

core syntax
dialect syntax
vendor syntax
experimental syntax

A dialect MUST NOT silently redefine an existing Zamani construct.

For example, a dialect must not redefine the meaning of:

if
match
fn
type
struct
module
import
requires
capability

unless the language specification explicitly permits that extension mechanism.

Where an existing Zamani construct is extended, the extension must have:

extension identity
extension point
syntax contract
AST mapping
semantic contract
compatibility contract

---

12. Syntax Extension Does Not Automatically Mean Semantic Extension

A dialect may add syntax without changing core language semantics.

For example:

dialect-specific annotation

may simply attach metadata to an existing AST node.

The dialect must declare whether its extension is:

syntax-only
metadata-only
semantic
type-system
effect-system
resource-system
capability-system
lowering
interoperability

This prevents accidental semantic coupling.

---

13. AST Contract

Every dialect construct that survives parsing MUST have a predetermined AST contract.

The required pipeline is:

dialect syntax
    ↓
ANTLR parse tree
    ↓
frontend AST
    ↓
semantic dialect model

The AST must remain domain-neutral where the repository's frontend architecture requires domain neutrality.

A dialect MUST NOT create an isolated permanent frontend AST universe unless explicitly authorized by the canonical AST architecture.

For every dialect construct, its contract must specify:

grammar rule
AST representation
fields
source spans
attributes
children
optional values
generic values
semantic identity
error representation

The AST contract must be defined before the grammar construct is considered production complete.

---

14. Semantic Contract

The parser establishes structure.

Semantic analysis establishes meaning.

Therefore:

parser accepts syntax
        ≠
dialect is semantically valid

Semantic analysis must determine:

- whether the dialect exists;
- whether it is visible;
- whether its version is valid;
- whether dependencies exist;
- whether dependencies are compatible;
- whether extensions are valid;
- whether capabilities are valid;
- whether requirements are satisfiable;
- whether conflicting dialects are present;
- whether namespace ownership is valid;
- whether feature gates permit use;
- whether the dialect has a valid lowering;
- whether the dialect's semantic constructs are supported.

The grammar must not attempt to perform these decisions.

---

15. Canonical IR Contract

A dialect MUST map into the repository's canonical semantic/IR architecture.

A dialect must not create a permanent parallel IR solely because it has a different source syntax.

The general path is:

Dialect syntax
      ↓
domain-neutral AST
      ↓
semantic dialect model
      ↓
canonical semantic representation
      ↓
canonical IR

For quantum functionality, the canonical quantum boundary remains:

quantum::ir

A dialect must not create:

QuantumDialectIR
QuantumVendorIR
QuantumGateIR
QuantumDialectIntermediateIR

as competing permanent representations.

Instead:

dialect syntax
      ↓
semantic quantum operation
      ↓
quantum::ir

The same principle applies to classical, HDL, hardware, data, AI, distributed and other canonical representations.

---

16. Dialects and Quantum Computing

Quantum dialects MUST preserve the generic quantum operation model.

A dialect must not force a fixed universal gate enumeration such as:

H
X
Y
Z
CNOT
...

into the root grammar.

The scalable model is:

operation identity
parameters
operands
results
attributes
modifiers
effects
capabilities
requirements
source information

A dialect may introduce a known operation such as:

quantum::standard::operation

but the underlying semantic operation must remain compatible with the generic quantum operation model.

For example, custom or future operations must remain representable without changing the root grammar.

The intended path is:

dialect operation
      ↓
generic semantic operation
      ↓
quantum::ir
      ↓
optimization
      ↓
routing
      ↓
scheduling
      ↓
QEC / resilience
      ↓
ZQN
      ↓
HAL
      ↓
target

Dialect syntax must not perform physical qubit mapping.

---

17. Dialects and HDL

An HDL dialect may define:

- RTL constructs;
- interfaces;
- protocols;
- timing intent;
- parameterized widths;
- state machines;
- pipelines;
- memory intent;
- verification constructs;
- synthesis intent.

It must not turn a dialect declaration into a fixed physical chip description unless that physical description is explicitly part of a downstream hardware contract.

The dialect must remain compatible with parameterized hardware intent.

For example, a width may be represented by a program parameter rather than universally hard-coded as:

[31:0]

when that width is intended to scale.

---

18. Dialects and Classical Computing

Classical dialects may add:

- numerical operations;
- scientific constructs;
- symbolic constructs;
- vector/tensor operations;
- specialized execution semantics;
- optimization intent;
- domain-specific data operations.

They must not make a particular:

CPU
ISA
register width
SIMD width
cache size
core count

part of universal Zamani syntax unless explicitly represented as a target-level contract.

---

19. Dialects and AI/Data/Tensor Computing

AI and data dialects may define semantic concepts such as:

model
dataset
training
inference
tensor
stream
pipeline
agent

but framework-specific implementations must remain outside core syntax.

The dialect must not require:

specific GPU model
specific accelerator
specific tensor-core generation
specific framework
specific vendor runtime

to establish the source-level meaning.

The compiler may later specialize the semantic representation.

---

20. Dialects and Distributed Computing

Distributed dialects may define:

- communication;
- services;
- processes;
- actors;
- channels;
- replication;
- partitioning;
- consistency;
- collective operations;
- fault tolerance;
- placement intent.

The grammar must not impose:

MAX_NODES
MAX_PROCESSES
MAX_CHANNELS
MAX_SERVICES

or any equivalent artificial ceiling.

A dialect describes distributed semantics.

The compiler/runtime determines actual placement.

---

21. Dialects and Hardware Capabilities

A dialect may declare capability requirements.

For example:

requires capability("quantum.measurement")
requires capability("quantum.mid_circuit_measurement")
requires capability("tensor.compute")
requires capability("gpu.compute")

These are semantic capability requirements.

They do not select a physical target.

The separation is:

dialect requirement
        ↓
capability resolution
        ↓
target capability inventory
        ↓
compiler realization

---

22. Requirements, Capabilities, Constraints, Preferences and Hints

These concepts MUST remain distinct.

Requirement

A property that must be satisfied.

requires qubits >= n

Capability

A semantic ability required by the program.

requires capability("quantum.measurement")

Constraint

A condition that must hold.

requires latency <= budget

Preference

A desired but non-mandatory realization.

prefer accelerator("quantum")

Hint

Compilation guidance that does not normally alter semantics.

hint locality

Realization

A downstream implementation decision.

logical resource → physical resource

A dialect MUST NOT collapse these categories.

---

23. No Hard-Coded Hardware Limits

The dialect grammar MUST NOT encode artificial universal limits.

The following are prohibited as language-level ceilings:

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
MAX_TIMELINES
MAX_DEVICES
MAX_ACCELERATORS
MAX_NETWORK_LINKS
MAX_DIALECTS
MAX_EXTENSIONS
MAX_CAPABILITIES
MAX_REQUIREMENTS
MAX_IMPORTS
MAX_NAMESPACE_DEPTH

This also applies to hidden equivalents.

For example, a parser rule that accepts only a fixed number of dialect members is prohibited merely because the number is not named "MAX_DIALECTS".

The grammar must express repetition structurally:

member*

or:

member+

where appropriate.

Practical implementation limits may exist because of:

- available memory;
- integer representation;
- process address space;
- compiler resource budgets;
- target resources;
- operating-system limits.

Such implementation limitations must not become artificial language semantics.

---

24. Tiny-to-Large Scalability

The dialect system must scale conceptually from:

one dialect
one extension
one capability
one requirement

to:

arbitrarily many dialects
arbitrarily many extensions
arbitrarily many capabilities
arbitrarily many requirements
arbitrarily large compositions

subject to actual implementation and resource availability.

No finite language-level ceiling may be inferred from today's hardware.

The same dialect contract must remain structurally meaningful on:

tiny embedded target
single CPU
multicore CPU
GPU
FPGA
ASIC
QPU
accelerator
HPC system
cluster
distributed system
cloud
future computational substrate

when the target satisfies the dialect's semantic requirements.

---

25. POCO-REAF

Dialect design is subordinate to:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever»

POCO-REAF requires that dialects describe durable source semantics rather than temporary target implementation details.

The intended model is:

Program
   ↓
stable Zamani semantics
   ↓
dialect semantics
   ↓
canonical representation
   ↓
compile
   ↓
capability/resource analysis
   ↓
target adaptation
   ↓
execution

A target change must not require rewriting the source merely because:

- the CPU changed;
- the GPU changed;
- the FPGA changed;
- the QPU changed;
- the number of available resources changed;
- topology changed;
- memory capacity changed;
- a new accelerator appeared.

When a target cannot satisfy a requirement, the compiler must report the incompatibility or use an explicitly permitted alternative realization.

It must not silently change program meaning.

---

26. Versioning

Dialect versions are semantic-contract versions.

Version syntax is owned by:

grammar/core/versioning.g4

and adapted to dialect context by:

grammar/dialects/versioning.g4

"versioning.g4" must not create a second version language.

Dialect version semantics are resolved outside the parser.

The dialect version system must support:

exact versions
version ranges
version constraints
pre-release identifiers
build metadata
version references
compatibility expressions

without imposing artificial numeric bounds.

Dialect versions must remain independent of the Rust compiler version.

Rust 1.97 / 1.97.1 is an implementation baseline, not a Zamani dialect version.

---

27. Compatibility

Dialect compatibility is layered.

At minimum:

source compatibility
lexical compatibility
syntactic compatibility
AST compatibility
semantic compatibility
capability compatibility
resource compatibility
IR compatibility
compiler compatibility
runtime compatibility
target compatibility

These must not be conflated.

Two dialect implementations may use different internal representations while remaining semantically compatible.

Conversely, two syntactically similar dialects may be semantically incompatible.

The dialect compatibility contract integrates with:

grammar/spec/compatibility.md
grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md

---

28. Dialect Migration

A dialect migration must prefer:

old syntax
    ↓
canonical semantic meaning
    ↓
current syntax/representation

over maintaining permanent parallel semantic architectures.

A migration must identify:

source changes
AST changes
semantic changes
IR changes
compiler changes
runtime changes
compatibility status

A migration must preserve meaning where the compatibility contract promises preservation.

A dialect must never be declared incompatible merely because a particular hardware target cannot currently support it.

---

29. Extension Points

"extension-points.g4" provides reusable attachment points.

An extension point identifies:

extension identity
target
kind
metadata
requirements
capabilities
compatibility
semantic/lowering descriptors

An extension point is not:

backend
IR
device
physical resource
scheduler
router
QEC implementation
ZQN implementation
runtime object

Semantic analysis determines whether the extension point is valid and usable.

---

30. Vendor Extensions

Vendor extensions are explicitly identifiable.

A vendor dialect may expose vendor-specific functionality without polluting Zamani core syntax.

For example, structurally:

vendor::organization::domain

may identify a vendor-owned dialect.

Vendor syntax MUST NOT silently become universal Zamani syntax.

Vendor extensions must specify:

vendor identity
dialect identity
version
semantic contract
capabilities
requirements
compatibility
fallback behavior where applicable
migration path

Vendor-specific physical information belongs downstream unless it is deliberately part of the declared dialect contract.

---

31. Experimental Extensions

Experimental extensions must be explicitly marked.

Experimental syntax must not silently become stable.

An experimental dialect must identify:

experimental status
version
owner
feature gate
semantic contract
known limitations
compatibility expectations
promotion path

Promotion is:

experimental
    ↓
proposal validation
    ↓
semantic design
    ↓
AST contract
    ↓
grammar
    ↓
implementation
    ↓
IR contract
    ↓
tests
    ↓
compatibility review
    ↓
stable

---

32. Feature Gates

Feature gates control whether a dialect or dialect feature is permitted in a particular language/compiler configuration.

A feature gate must be:

- explicit;
- deterministic;
- version-aware;
- diagnosable;
- independent of physical hardware identity.

A feature gate must not be used to hide a hardware capacity limit.

For example:

feature quantum_dynamic_control

is structurally different from:

feature qpu_has_32_qubits

The first describes a language capability.

The second incorrectly turns a target property into a language feature.

---

33. Registry Contract

The dialect registry is the authoritative semantic registry of known dialect contracts.

The registry is represented by:

grammar/dialects/registry.md

The registry is not:

- a grammar;
- a hardware inventory;
- a device inventory;
- an IR;
- a runtime;
- a scheduler;
- a package manager.

The registry records enough metadata for deterministic dialect resolution.

A registry entry should be able to identify:

dialect ID
owner
version
status
dependencies
capabilities
requirements
extension points
semantic contract
AST contract
IR mapping
compatibility
migration
provenance

Adding a registry entry must not require changing the root dialect grammar.

---

34. Deterministic Resolution

Dialect resolution must be deterministic.

Given the same:

source
language version
dialect declarations
dialect versions
dependency metadata
feature gates
compatibility rules
registry state

the resolver must produce the same semantic result.

The grammar itself remains deterministic parsing machinery.

Resolution belongs to semantic/compiler infrastructure.

If multiple valid resolutions exist and the language permits choice, the selection policy must be explicitly specified rather than left to implementation accident.

---

35. Dependency Model

A dialect may depend on another dialect.

Dependencies must be represented symbolically.

For example:

dialect A
    requires dialect B

The grammar records the declaration.

Semantic infrastructure determines:

- whether B exists;
- which version is selected;
- whether the requested version is compatible;
- whether dependencies form a cycle;
- whether all requirements are satisfied.

The grammar must not implement dependency resolution.

There must be no fixed maximum dependency count.

---

36. Namespace Model

Dialect namespaces must use the canonical Zamani naming model.

"namespaces.g4" owns dialect-context namespace syntax.

Canonical name resolution remains a semantic concern.

A dialect must not redefine the global identifier grammar.

Namespace depth must not be limited by an arbitrary constant.

For example, a structure such as:

organization::division::project::domain::dialect

must be representable without the grammar having to know how many namespace components exist.

---

37. Lexical Integration

Dialect grammar files must consume the canonical Zamani lexical vocabulary.

They must not create private lexical universes.

The canonical lexer owns:

- identifiers;
- keywords;
- operators;
- punctuation;
- literals;
- comments;
- Unicode handling;
- source positions.

Dialect-specific keywords must be introduced only through the repository's approved keyword-extension mechanism.

A dialect must not independently redefine an existing token with a different meaning.

Duplicate lexical concepts must be resolved centrally.

---

38. Root Grammar Integration

"grammar/Zamani.g4" remains the canonical root.

The root grammar must compose the dialect boundary rather than embedding every dialect definition.

The intended relationship is:

grammar/Zamani.g4
        ↓
dialect entry point
        ↓
grammar/dialects/dialect.g4
        ↓
registration.g4
        ↓
supporting dialect grammar contracts

The root grammar must not enumerate every known dialect.

Adding:

future::computing::new_dialect

must not require editing "Zamani.g4" merely to add its name.

---

39. Historical "dialects.g4"

If an older:

grammar/dialects/dialects.g4

exists or is referenced by tooling, it must not become a competing authority.

The preferred canonical boundary is:

grammar/dialects/dialect.g4

If historical tooling requires "dialects.g4", it may temporarily act as a compatibility wrapper.

It must not independently define a second dialect grammar.

The repository must eventually have one authoritative public dialect composition contract.

---

40. "registration.g4" Integration

"registration.g4" owns detailed registration syntax.

It is responsible for the source-level representation of:

- dialect identity;
- owner;
- version;
- dependencies;
- capabilities;
- requirements;
- extensions;
- semantic descriptors;
- lowering descriptors;
- compatibility;
- deprecation;
- metadata.

It must consume, rather than duplicate:

names
versioning
capabilities
compatibility
extension points

from their respective owners.

---

41. "capabilities.g4" Integration

"capabilities.g4" is an adapter.

The canonical capability model remains owned by the core capability contract.

This file provides dialect context.

It must not redefine:

capability identity
capability version language
capability resolution
hardware discovery
resource allocation

Capability resolution is semantic.

---

42. "versioning.g4" Integration

"versioning.g4" adapts canonical version syntax to dialect declarations.

It must not redefine:

exactVersion
versionRange
versionComparator
versionConstraint
versionIdentifier

when those are already owned by the canonical versioning grammar.

It only answers:

«Where does the canonical version expression apply in dialect syntax?»

---

43. "compatibility.g4" Integration

"compatibility.g4" owns dialect compatibility declaration syntax.

It records compatibility intent.

It does not implement:

version solving
compatibility solving
migration execution
dependency resolution

Those remain semantic/compiler responsibilities.

---

44. "extension-points.g4" Integration

"extension-points.g4" provides reusable extension attachment syntax.

It must not duplicate registration syntax.

It must not redefine:

dialectRegistration
dialectImport
dialectExport
qualifiedName
versionExpression
capabilityExpression
compatibilityExpression

The extension-point contract must remain reusable across:

classical
quantum
HDL
hardware
AI
data
distributed
networking
security
future domains

without creating separate extension languages.

---

45. "vendor.g4" Integration

"vendor.g4" owns explicit vendor-extension syntax.

It must remain an adapter over the general dialect system.

Vendor syntax must eventually map to the same:

AST
semantic model
capability model
resource model
compatibility model
IR boundary

used by ordinary dialects.

---

46. "experimental.g4" Integration

"experimental.g4" owns explicit experimental-extension syntax.

Experimental syntax must use the same:

AST
semantic
capability
compatibility
IR
diagnostic

contracts as stable syntax.

The lifecycle state differs; the architectural model does not.

---

47. "grammar.md" Integration

"grammar/grammar.md" remains the implementation-conformance reference.

Dialect features must appear there according to actual implementation status:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

A dialect described in this README is not automatically implemented.

A dialect becomes implementation-conformant only when the actual frontend and downstream pipeline support it.

---

48. "Zamani-Grammar.md" Integration

"grammar/Zamani-Grammar.md" remains a historical/extended design reference.

It may contain proposed dialect concepts.

Those concepts do not automatically become legal syntax.

The promotion path remains:

Zamani-Grammar.md
        ↓
proposal
        ↓
semantic design
        ↓
AST contract
        ↓
canonical grammar
        ↓
implementation
        ↓
IR contract
        ↓
tests
        ↓
stable

This prevents the historical design document from becoming a competing grammar authority.

---

49. Specification Integration

Dialect contracts must integrate with:

grammar/specification/
grammar/spec/

At minimum, relevant dialect semantics must trace to:

language
lexical
syntax
semantics
type system
effects
resources
capabilities
compatibility
quantum
classical
HDL
interoperability
diagnostics

A dialect-specific specification may extend those contracts, but it must not silently contradict them.

---

50. Frontend AST Integration

The frontend AST must remain the canonical structural representation.

Dialect AST mappings must specify:

node identity
fields
children
attributes
source span
semantic identity
validation rules

The dialect contract must identify exactly which AST representation is used.

No downstream file should need to reinterpret the parser tree to determine what a dialect construct means.

---

51. Semantic Integration

Dialect semantic analysis must occur after parsing.

The semantic phase owns:

dialect resolution
namespace resolution
version resolution
dependency resolution
capability resolution
resource requirement analysis
extension validation
compatibility validation
feature-gate validation
conflict detection
semantic diagnostics

The parser must not perform these operations.

---

52. Resource Integration

A dialect may express resource requirements.

Examples:

requires memory >= required_memory
requires qubits >= n
requires capability("tensor.compute")
requires capability("gpu.compute")
requires topology(...)

These expressions describe program requirements.

They do not establish compiler maxima.

Resource resolution belongs to the resource/compiler/target layers.

---

53. Target Integration

The target layer determines whether a dialect's requirements can be realized.

The dialect grammar does not select:

CPU 0
GPU 3
FPGA 2
QPU 1
node 17
qubit 42
memory-bank 5

unless such physical identity is explicitly and intentionally part of a downstream target-specific contract.

The portable source-level dialect contract must remain independent of temporary machine topology.

---

54. Compiler Integration

A dialect must identify its lowering path.

For every semantic extension:

dialect construct
      ↓
AST
      ↓
semantic representation
      ↓
canonical IR
      ↓
lowering

The dialect must document:

- required compiler pass;
- canonical IR destination;
- required capabilities;
- resource requirements;
- target-independent transformations;
- target-specific transformations;
- diagnostics when lowering is unavailable.

The dialect grammar itself must not implement compiler passes.

---

55. Runtime Integration

A dialect that has runtime semantics must define:

runtime contract
execution requirements
observable behavior
failure semantics
resource behavior
determinism requirements
compatibility

Runtime implementation remains outside "grammar/dialects/".

The dialect grammar merely establishes source structure.

---

56. Quantum Runtime Integration

For quantum dialects, runtime integration follows:

dialect syntax
      ↓
domain-neutral AST
      ↓
quantum semantic model
      ↓
quantum::ir
      ↓
optimization
      ↓
routing
      ↓
scheduling
      ↓
QEC / resilience
      ↓
ZQN
      ↓
HAL
      ↓
QPU / simulator

The dialect must not bypass this architecture.

---

57. HDL Runtime / Toolchain Integration

HDL dialects may lower toward:

HDL representation
      ↓
synthesis
simulation
verification
hardware realization

The dialect must not make the grammar responsible for synthesis algorithms.

Physical implementation belongs downstream.

---

58. Diagnostics

Every dialect must have deterministic diagnostics for at least:

unknown dialect
unknown dialect version
incompatible dialect version
missing dependency
conflicting dependency
unknown extension
invalid extension point
missing capability
unsatisfied requirement
invalid feature gate
unsupported semantic construct
invalid compatibility declaration
deprecated construct
experimental construct used without permission
invalid namespace
ambiguous dialect reference
missing lowering
unsupported target realization

Diagnostics should preserve source spans.

The parser must not invent semantic errors that belong to later phases.

---

59. Error Recovery

ANTLR parser error recovery must remain compatible with the repository's parser architecture.

Dialect syntax must not create error recovery paths that consume arbitrary unrelated source.

Malformed dialect declarations must fail locally where possible.

Examples of malformed input that require coverage:

missing dialect identity
missing version
invalid namespace
missing dependency
invalid extension
malformed capability
malformed compatibility expression
unclosed dialect declaration
unexpected dialect member

---

60. Determinism

Dialect parsing and resolution must be deterministic.

Given identical:

source
language version
dialect definitions
registry state
feature gates
compatibility metadata

the system must produce identical:

parse result
AST
semantic result
diagnostics
dialect resolution

where deterministic behavior is specified.

No iteration-order accident may affect semantic meaning.

---

61. Reproducibility

A dialect build must be reproducible from declared inputs.

Dialect resolution must not depend silently on:

machine model
CPU count
GPU count
QPU count
filesystem traversal order
network response ordering
environment-variable accidents
undocumented local state

External registry data must have explicit provenance and versioning.

---

62. Security Boundary

A dialect declaration is source data.

Parsing it must not execute arbitrary code.

Dialect registration syntax must not itself:

- load arbitrary executables;
- execute shell commands;
- access hardware;
- modify files;
- access secrets;
- alter compiler memory unsafely.

Rust implementation remains safe Rust.

No "unsafe" Rust is required or permitted by this contract.

---

63. No Runtime Plugin Execution During Parsing

Dialect support must not require arbitrary plugin execution during parsing.

The architectural boundary is:

parse
    ↓
validate
    ↓
resolve
    ↓
compile
    ↓
execute

If external dialect metadata is used, the metadata must be treated as declared data and validated according to the registry/security policy.

---

64. Rust Implementation Contract

The repository implementation baseline is:

Rust 1.97 / Rust 1.97.1
Rust 2021

Production Rust implementation MUST use safe Rust.

"unsafe" must not be required for:

- dialect parsing;
- dialect registration;
- dialect resolution;
- dialect compatibility;
- dialect metadata;
- dialect validation;
- AST conversion;
- semantic analysis.

The grammar files themselves do not contain Rust implementation code, but their contracts must remain implementable using the supported safe-Rust baseline.

---

65. Generic Data Model Principle

Dialect metadata should be represented using generic structures rather than a closed enumeration of known domains.

Avoid designs such as:

enum DialectKind {
    Quantum,
    Classical,
    HDL,
    GPU,
    FPGA,
    AI,
}

when the enum is being used to make the dialect universe closed.

Prefer a symbolic identity model capable of representing:

namespace
name
version

plus declared metadata.

Known standard dialects may be recognized semantically, but the grammar must remain open to future identities.

---

66. No Vendor Keyword Explosion

A new vendor must not require adding a new global keyword for every vendor feature.

Prefer:

vendor::organization::dialect

plus generic extension mechanisms.

This prevents:

NVIDIA_FEATURE
AMD_FEATURE
IBM_FEATURE
INTEL_FEATURE
XILINX_FEATURE
...

from becoming permanent core language vocabulary.

---

67. No Framework Lock-In

Dialect contracts must not make frameworks part of Zamani's universal grammar.

For example, framework names may be represented as:

interoperability metadata
dialect identity
package/module dependency

rather than becoming permanent core keywords.

This allows implementations to evolve without changing Zamani source semantics.

---

68. No Hardware Lock-In

A dialect MUST NOT require a particular hardware model unless its explicitly declared semantic contract is intentionally hardware-specific.

Even hardware-specific dialects should distinguish:

semantic requirement

from:

physical realization

The latter belongs downstream whenever possible.

---

69. Dialect Composition

Multiple dialects may coexist in one program.

The composition model must support:

dialect A
dialect B
dialect C

and relationships such as:

A requires B
A extends B
A interoperates with B
A conflicts with B
A is compatible with B

without imposing a fixed number of dialects.

Composition semantics must be explicit.

The compiler must detect incompatible combinations rather than relying on parser order.

---

70. Conflict Detection

Dialect conflicts may occur at:

lexical level
syntax level
namespace level
semantic level
type level
effect level
capability level
resource level
IR level
runtime level

Each conflict must be classified.

A dialect must not silently override another dialect.

Where explicit precedence is permitted, that precedence must be part of the dialect contract.

---

71. Extension Precedence

Extension precedence must never be determined accidentally by:

- file ordering;
- hash-map iteration;
- registration order;
- parser traversal order;
- filesystem order.

If precedence is meaningful, it must be explicitly declared and semantically validated.

---

72. Dialect Imports

Dialect imports must use the canonical module/name model.

Imports identify contracts.

They do not directly:

- load hardware;
- allocate resources;
- select devices;
- execute code.

Import resolution belongs to semantic/compiler infrastructure.

---

73. Dialect Exports

A dialect may expose extensions to other dialects.

Exports must identify:

export identity
version
visibility
compatibility
semantic contract

Export visibility must not bypass normal namespace and module rules.

---

74. Capability Namespaces

Capability identities should be namespaced.

Examples:

quantum.measurement
quantum.mid_circuit_measurement
tensor.compute
gpu.compute
hdl.synthesis
distributed.messaging

A capability name does not imply a physical implementation.

For example:

gpu.compute

means a capability, not a specific GPU.

---

75. Resource Semantics

Dialect resource requirements must remain symbolic where possible.

Examples:

qubits >= n
memory >= required_memory
parallelism >= desired_parallelism

A dialect must not convert these into parser constants.

The semantic resource system determines feasibility.

---

76. Capability Negotiation

A dialect may declare:

required capability
preferred capability
optional capability
fallback capability

Capability negotiation is semantic/compiler behavior.

The grammar records declarations.

It does not negotiate with hardware.

---

77. Fallbacks

Where a dialect supports alternative realizations, the fallback policy must be explicit.

For example:

preferred capability
required capability
fallback implementation

A fallback must preserve semantics unless the source explicitly permits approximation or alternative semantics.

The compiler must not silently substitute an incompatible operation.

---

78. Future-Proofing

The dialect architecture must support future computing models without changing its fundamental architecture.

Examples include:

new quantum technologies
new accelerator models
new hardware fabrics
new memory models
new distributed architectures
new AI paradigms
new simulation technologies
new computing substrates

A future domain should be able to introduce:

new dialect identity
new semantic contract
new capabilities
new extension points
new lowering

without changing the core language merely to add its name.

---

79. Dialects and Sankofa

Where Sankofa concepts are exposed through Zamani dialects, the dialect must remain a language extension rather than becoming a second language.

Sankofa-related concepts may include:

memory
history
recall
learning
wisdom
temporal information
provenance
consensus

The dialect grammar defines their source representation.

Their actual semantic/runtime implementation belongs downstream.

A Sankofa-related dialect must continue to use Zamani's:

AST
semantic model
capability model
resource model
IR architecture

rather than creating an isolated compiler universe.

---

80. Dialects and Multi-Timeline Computation

If temporal or multi-timeline facilities are represented through a dialect, the grammar must remain open-ended.

There must be no fixed:

MAX_TIMELINES
MAX_BRANCHES
MAX_FORKS
MAX_HISTORY

A dialect may describe:

timeline
fork
merge
observation
rewind
speculation

while the execution subsystem owns actual timeline management.

---

81. Dialects and Nano Computing

A nano-oriented dialect may represent:

agents
atoms
molecules
materials
interactions
protocols

but must not turn the grammar into a fixed database of physical entities.

Physical models belong to semantic/domain libraries and execution infrastructure.

The dialect remains a source-level contract.

---

82. Dialect File Completion Contract

Every dialect grammar file must be independently completable.

A file is not complete merely because its ANTLR rules compile.

The file's completion checklist is:

[ ] Purpose defined
[ ] Status defined
[ ] Ownership defined
[ ] Non-ownership defined
[ ] Inputs defined
[ ] Outputs defined
[ ] Dependencies defined
[ ] Upstream contracts identified
[ ] Downstream consumers identified
[ ] Lexer contract identified
[ ] Grammar contract complete
[ ] AST contract complete
[ ] Semantic contract complete
[ ] IR contract complete
[ ] Compiler integration defined
[ ] Runtime integration defined where applicable
[ ] Capability integration defined
[ ] Resource integration defined
[ ] Compatibility integration defined
[ ] Versioning integration defined
[ ] Diagnostics defined
[ ] Source-span requirements defined
[ ] Determinism requirements defined
[ ] Security requirements defined
[ ] Positive tests defined
[ ] Negative tests defined
[ ] Boundary tests defined
[ ] Scalability tests defined
[ ] Compatibility tests defined
[ ] Hard-coding audit passed
[ ] No duplicate ownership
[ ] No competing grammar
[ ] No fixed hardware assumptions
[ ] No artificial capacity limits
[ ] Rust implementation compatibility verified
[ ] Safe-Rust requirement verified

Once these conditions are satisfied, later work in another subsystem must not require redesigning the file merely because that subsystem was implemented.

Implementation bugs may of course require correction; architectural contracts must not depend on unfinished neighboring implementation.

---

83. Per-Feature Traceability

Every dialect feature must have a traceable chain:

Feature ID
    ↓
Specification
    ↓
Lexical contract
    ↓
Grammar rule
    ↓
AST node
    ↓
Semantic rule
    ↓
Capability/resource rules
    ↓
Canonical IR mapping
    ↓
Compiler/lowering
    ↓
Runtime behavior
    ↓
Tests

A feature without this chain is incomplete.

---

84. Feature Manifest Integration

Where machine-readable feature manifests are introduced under the specification system, a dialect feature should identify:

feature ID
dialect ID
dialect version
status
syntax
grammar
tokens
AST mapping
semantic mapping
IR mapping
compiler consumers
runtime consumers
capabilities
resource requirements
positive tests
negative tests
boundary tests
scalability tests
compatibility tests
migration
hard-coding policy

The manifest is a contract index.

It does not replace the grammar.

---

85. Testing Requirements

Dialect testing must include:

lexical tests
syntax tests
AST tests
semantic tests
capability tests
resource tests
compatibility tests
diagnostic tests
determinism tests
scalability tests

Every dialect should have:

positive/
negative/
boundary/
scalability/
compatibility/

test coverage.

---

86. Scalability Tests

Scalability tests must verify that the dialect grammar does not accidentally introduce finite limits.

Examples:

one dialect
many dialects
nested namespaces
many extensions
many capabilities
many requirements
many dependencies
large metadata sets
large source programs
large operation lists
large composition graphs

Tests must use generated or parameterized sizes where appropriate rather than choosing one arbitrary size and treating it as a language maximum.

---

87. Hard-Coding Audit

Every dialect contribution must be checked for:

MAX_*
fixed device identifiers
fixed qubit identifiers
fixed CPU identifiers
fixed GPU identifiers
fixed FPGA identifiers
fixed node counts
fixed tensor dimensions
fixed register widths
fixed memory sizes
fixed topology sizes

The audit must distinguish legitimate program constants from artificial implementation limits.

For example:

let n = 1024;

is program data.

But:

dialect accepts no more than 1024 capabilities

is an artificial language limit.

---

88. Repository-Wide Integration Matrix

A production dialect must be traceable across:

grammar/DESIGN.md
grammar/README.md
grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md

grammar/specification/*
grammar/spec/*

grammar/lexer/*
grammar/core/*
grammar/types/*
grammar/expressions/*
grammar/declarations/*
grammar/statements/*
grammar/functions/*
grammar/modules/*
grammar/effects/*
grammar/memory/*
grammar/concurrency/*

grammar/classical/*
grammar/quantum/*
grammar/hybrid/*
grammar/hdl/*
grammar/hardware/*
grammar/resources/*
grammar/distributed/*
grammar/ai/*
grammar/data/*
grammar/networking/*
grammar/security/*

grammar/compile/*
grammar/execution/*
grammar/interoperability/*
grammar/dialects/*
grammar/macros/*
grammar/metaprogramming/*

grammar/compatibility/*
grammar/validation/*
grammar/tests/*

Only the files actually relevant to a particular dialect need concrete implementation.

The point is traceability, not forcing every dialect to implement every subsystem.

---

89. What a Dialect Does Not Own

A dialect grammar does not own:

hardware discovery
device allocation
physical resource allocation
runtime execution
scheduler implementation
routing implementation
optimization implementation
QEC implementation
ZQN implementation
calibration
HAL implementation
compiler backend implementation
network transport implementation
filesystem implementation
package resolution implementation
version solver implementation
capability discovery implementation

The dialect contract may declare the requirements consumed by these systems.

It must not implement them at the grammar layer.

---

90. What a Dialect May Own

A dialect may own:

dialect identity
dialect-specific syntax
dialect-specific metadata
dialect-specific semantic concepts
dialect-specific capabilities
dialect-specific requirements
dialect-specific extension points
dialect-specific compatibility declarations
dialect-specific lowering contracts
dialect-specific diagnostics
dialect-specific migration information

provided those concepts remain within Zamani's global architecture.

---

91. Interoperability

A dialect may provide interoperability with:

OpenQASM
QIR
LLVM
MLIR
HDL formats
WASM
foreign languages
foreign ABIs
vendor ecosystems

but interoperability formats are not automatically canonical Zamani semantic representations.

The preferred path is:

external representation
      ↓
adapter
      ↓
Zamani semantic representation
      ↓
canonical IR

rather than:

external representation
      ↓
permanent parallel Zamani IR

---

92. No Duplicate Quantum IR

This rule is absolute.

The dialect subsystem must not introduce another quantum IR.

All quantum dialects ultimately converge on:

quantum::ir

The dialect layer may provide syntax and semantic adapters.

It must not create a competing quantum intermediate representation.

---

93. No Duplicate Core Type System

A dialect may introduce types, but must use the canonical Zamani type-system architecture.

It must not create a private type universe incompatible with:

grammar/types/*

Dialect types must specify:

type identity
parameters
constraints
operations
conversion rules
semantic properties
IR representation

---

94. No Duplicate Core Effect System

Dialect effects must integrate with:

grammar/effects/*

A dialect may introduce a new effect identity, but it must use the common effect architecture.

Effects must remain analyzable by the semantic system.

---

95. No Duplicate Resource System

Dialect-specific resource requirements integrate with:

grammar/resources/*

A dialect must not create:

QuantumResources
GpuResources
HdlResources

as mutually incompatible resource languages.

Domain-specific resource identities may exist, but the common resource contract remains authoritative.

---

96. No Duplicate Capability System

Dialect capabilities integrate with the canonical capability model.

A capability such as:

quantum.measurement

is a capability identity.

It is not a direct hardware handle.

---

97. Source Compatibility

Adding a dialect must not reinterpret unrelated existing Zamani source.

A dialect must have explicit activation/visibility semantics.

A program that does not activate or import a dialect must not unexpectedly acquire that dialect's syntax or semantics.

This prevents dialect additions from changing the meaning of existing programs.

---

98. Namespace Isolation

Dialect names and extensions must remain namespace-aware.

A dialect must not claim arbitrary unqualified global names unless the language specification explicitly grants that authority.

This prevents collisions among:

standard dialects
organization dialects
vendor dialects
experimental dialects
future dialects

---

99. Backward Compatibility

A dialect update must classify changes as:

compatible
conditionally compatible
source-breaking
semantic-breaking
deprecated
removed

The classification must be documented.

A patch-level dialect release must not silently introduce semantic breaking changes.

---

100. Forward Compatibility

The grammar must tolerate future dialect identities through the open-world symbolic identity model.

The language should be able to encounter a dialect that a particular compiler does not know.

The correct result may be:

unknown dialect

or:

unsupported dialect version

rather than parser corruption or reinterpretation as another dialect.

---

101. Unknown Dialects

An unknown dialect should be represented distinctly from malformed syntax.

These are different conditions:

syntactically invalid dialect reference

versus:

syntactically valid but unknown dialect

The parser handles the first.

Semantic/registry resolution handles the second.

---

102. Unsupported Dialects

A compiler may know a dialect exists but not implement it.

That state must be distinguishable from:

unknown
invalid
incompatible
deprecated

This distinction is necessary for robust tooling and long-term compatibility.

---

103. Partial Implementation

A dialect may be:

implemented
partially implemented
planned

but the implementation status must be reported accurately.

Partial parser support must not be advertised as full semantic support.

---

104. Documentation Rule

This README defines the dialect architecture.

It is not a second syntax reference.

Concrete syntax belongs to:

dialect.g4
registration.g4
namespaces.g4
versioning.g4
capabilities.g4
compatibility.g4
extension-points.g4
vendor.g4
experimental.g4

Normative language semantics belong to the specification system.

Implementation status belongs to "grammar.md".

Historical/proposed material belongs to "Zamani-Grammar.md".

---

105. Production Readiness Gate

The dialect subsystem is production-ready only when all of the following are true:

[ ] One canonical dialect parser boundary exists
[ ] No competing dialect grammar exists
[ ] Dialect identities are open-world
[ ] Dialects are explicitly versioned
[ ] Ownership is explicit
[ ] Lifecycle is explicit
[ ] Syntax extensions are explicit
[ ] Semantic extensions are explicit
[ ] AST mappings exist
[ ] Semantic mappings exist
[ ] Canonical IR mappings exist
[ ] Quantum features map to quantum::ir
[ ] Hardware is not hard-coded into the grammar
[ ] Resources are not hard-coded into the grammar
[ ] Capabilities are separate from hardware selection
[ ] Requirements are separate from preferences
[ ] Preferences are separate from implementation decisions
[ ] Vendor extensions are explicitly scoped
[ ] Experimental extensions are explicitly scoped
[ ] Feature gates are deterministic
[ ] Compatibility is versioned
[ ] Migration paths exist where required
[ ] Diagnostics exist
[ ] Source spans are preserved
[ ] Resolution is deterministic
[ ] Positive tests exist
[ ] Negative tests exist
[ ] Boundary tests exist
[ ] Scalability tests exist
[ ] Compatibility tests exist
[ ] Hard-coding audit passes
[ ] Safe Rust implementation is possible
[ ] Rust 1.97 / 1.97.1 compatibility is verified
[ ] POCO-REAF is preserved

---

106. Final Architectural Invariant

The complete dialect architecture is:

                         Zamani
                            │
                            ▼
                    canonical grammar
                            │
                            ▼
                      dialect boundary
                            │
                ┌───────────┴───────────┐
                │                       │
          dialect syntax         dialect metadata
                │                       │
                └───────────┬───────────┘
                            │
                            ▼
                       frontend AST
                            │
                            ▼
                    semantic analysis
                            │
          ┌─────────────────┼─────────────────┐
          │                 │                 │
     capabilities       resources       compatibility
          │                 │                 │
          └─────────────────┼─────────────────┘
                            │
                            ▼
                 canonical semantic model
                            │
          ┌─────────────────┼──────────────────┐
          │                 │                  │
      classical        quantum::ir       HDL/hardware
          │                 │                  │
          └─────────────────┼──────────────────┘
                            │
                            ▼
                     optimization
                            │
              ┌─────────────┼─────────────┐
              │             │             │
           routing      scheduling     resilience
              │             │             │
              └─────────────┼─────────────┘
                            │
                         QEC / ZQN
                            │
                            ▼
                           HAL
                            │
                            ▼
                    target realization
                            │
        ┌─────────┬────────┼────────┬─────────┐
        │         │        │        │         │
       CPU       GPU      FPGA     QPU      Future
        │         │        │        │       targets
        └─────────┴────────┴────────┴─────────┘

The invariant is:

«A dialect may extend Zamani's source-level language contract, but it must never fracture Zamani into competing languages or competing semantic/IR architectures.»

And:

«A dialect describes durable computation semantics; the compiler and runtime determine how those semantics are realized on whatever resources are actually available.»

Therefore the dialect layer must remain:

open-world
versioned
namespaced
capability-aware
resource-aware
compatibility-aware
AST-traceable
semantic-traceable
IR-traceable
deterministic
scalable
hardware-independent
safe-Rust implementable
POCO-REAF compatible

with no artificial universal limits on dialects, extensions, capabilities, requirements, namespaces, devices, processors, memory, qubits, nodes, tensors, threads, timelines, or other physical resources.

---

107. Definition of Done for "grammar/dialects/README.md"

This README is complete when it can be used as the integration contract for every other dialect file without requiring a later architectural rewrite.

Specifically, a developer working on any one dialect file must be able to determine from this README:

what the file owns
what it does not own
which file owns its dependencies
which lexer contract it consumes
which grammar boundary it connects to
what AST it must produce
what semantic representation it must produce
which canonical IR it must use
how capabilities integrate
how resources integrate
how versioning integrates
how compatibility integrates
how vendor/experimental status integrates
how Zamani.g4 consumes it
how grammar.md reports it
how tests validate it
how hard-coding is prohibited
how POCO-REAF is preserved
how Rust 1.97/1.97.1 implementation remains safe

No dialect file may require a second dialect architecture.

No dialect may require modifying the root grammar simply to add its identity.

No dialect may create a competing quantum IR.

No dialect may introduce an artificial hardware capacity.

No dialect may silently become part of core Zamani semantics.

The final contract is therefore:

ONE LANGUAGE
ONE CORE LEXICAL MODEL
ONE CORE AST ARCHITECTURE
ONE SEMANTIC MODEL
ONE CANONICAL IR ARCHITECTURE
ONE quantum::ir
OPEN-WORLD DIALECTS
EXPLICIT EXTENSIONS
EXPLICIT CAPABILITIES
EXPLICIT REQUIREMENTS
EXPLICIT COMPATIBILITY
NO ARTIFICIAL RESOURCE CEILINGS
NO UNSAFE RUST
POCO-REAF

Program Once → Compile Once → Run Everywhere → Anywhere → Forever.