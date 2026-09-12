Worked for 15s

Below is the production-ready normative content for grammar/specification/extensibility.md. I have treated extensibility as a language-architecture contract, not merely a mechanism for adding more grammar rules. It explicitly prevents dialects, quantum/hardware extensions, vendor features, and future domains from contaminating the portable semantic core.

The repository grammar currently already contains broad declarations for modules, functions, effects, quantum circuits, HDL, plugins, macros, mathematics, etc., so this specification is designed to govern and progressively reorganize those existing capabilities rather than discard them.

Zamani Language Extensibility Specification

Path: "grammar/specification/extensibility.md"
Status: Normative
Specification family: Zamani Language Grammar
Applies to: Grammar, lexer, parser, AST, semantic analysis, IR, compiler, runtime, dialects, tooling, and repository-wide language extensions
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust safety requirement: No Rust "unsafe" code
Primary architectural goal: POCO-REAF — Program Once, Compile Once, Run Everywhere, Anywhere, Forever

---

1. Purpose

This document defines the production extensibility architecture of the Zamani programming language.

Zamani must be extensible without allowing extensions to destabilize:

- existing valid programs;
- the canonical language semantics;
- the canonical intermediate representations;
- quantum semantics;
- classical semantics;
- hardware independence;
- HDL semantics;
- resource portability;
- compiler determinism;
- runtime compatibility;
- tooling;
- security;
- reproducibility;
- scalability.

Extensibility means that Zamani can acquire new computational capabilities without requiring the language to be redesigned whenever a new:

- processor;
- accelerator;
- quantum architecture;
- hardware technology;
- programming paradigm;
- execution model;
- communication technology;
- mathematical domain;
- AI model;
- storage technology;
- deployment environment;
- compiler optimization;
- runtime capability;
- hardware interface;
- future computational technology

appears.

The fundamental rule is:

«Extensions may add capability, but must not redefine the meaning of the portable core.»

---

2. POCO-REAF Requirement

All extensibility decisions MUST preserve:

Program Once
      ↓
Portable Zamani Semantics
      ↓
Canonical Semantic Representation
      ↓
Compile Once
      ↓
Target / Capability Adaptation
      ↓
Run Everywhere / Anywhere
      ↓
Future-Compatible Execution

A source program must not have to be rewritten merely because its execution environment changes.

For example, the same semantic program may eventually execute on:

atom-scale / microscopic systems
embedded systems
CPU
multicore CPU
GPU
FPGA
ASIC
QPU
quantum simulator
AI accelerator
heterogeneous accelerator
cluster
supercomputer
distributed system
cloud
edge
future architecture

The grammar MUST NOT encode the assumption that any particular one of these is the permanent execution model.

---

3. Core Extensibility Principle

Zamani extensions exist in layers.

                    Zamani Source
                         │
                         ▼
                    Core Grammar
                         │
             ┌───────────┴───────────┐
             │                       │
        Core Semantics          Extension Syntax
             │                       │
             │                 ┌─────┴─────┐
             │                 │           │
             │              Dialects   Domain Syntax
             │                 │           │
             └────────────┬────┴───────────┘
                          ▼
                  Semantic Analysis
                          │
                          ▼
                 Canonical Semantic IR
                          │
              ┌───────────┼────────────┐
              ▼           ▼            ▼
          Classical    quantum::ir   Hardware
              │           │            │
              └───────────┼────────────┘
                          ▼
                 Optimization
                          ▼
                    Routing
                          ▼
                   Scheduling
                          ▼
                     ZQN/QEC
                          ▼
                    Hardware HAL
                          ▼
                       Runtime

An extension MUST NOT create an independent semantic universe when an existing canonical subsystem already owns the concept.

---

4. Ownership Rules

4.1 Grammar owns

The grammar owns:

- lexical structure;
- syntactic structure;
- syntactic composition;
- source-level declarations;
- source-level extension declarations;
- source-level annotations;
- source-level capability expressions;
- source-level requirements;
- source-level constraints;
- source-level dialect references;
- source-level version declarations.

4.2 Grammar does not own

The grammar does NOT own:

- hardware discovery;
- hardware calibration;
- quantum error correction algorithms;
- noise models;
- routing algorithms;
- scheduling algorithms;
- optimization algorithms;
- runtime state;
- backend-specific machine topology;
- physical device discovery;
- execution telemetry;
- canonical quantum IR;
- physical qubit allocation;
- compiler optimization decisions.

Those belong to the appropriate repository subsystem.

---

5. Canonical Semantic Boundary

Zamani MUST maintain a strict distinction between:

Syntax
AST
Semantic Model
IR
Target Representation
Runtime Representation

The grammar MUST NOT become a second IR.

For quantum computation:

Zamani quantum syntax
        ↓
AST
        ↓
semantic validation
        ↓
quantum::ir
        ↓
optimization
        ↓
routing
        ↓
scheduling
        ↓
ZQN / QEC / hardware integration
        ↓
runtime

"quantum::ir" remains the canonical quantum semantic boundary.

Quantum extensions may introduce syntax, declarations, annotations, and domain concepts, but they must lower into the established quantum semantic architecture.

---

6. Extension Categories

Every extension MUST belong to one or more explicitly identified categories.

6.1 Syntax extension

Adds new source syntax.

Examples:

new declaration
new expression
new statement
new attribute
new literal
new pattern

6.2 Semantic extension

Adds a new language meaning.

Examples:

new type
new effect
new resource class
new execution semantic

Semantic extensions require a defined semantic contract and cannot be accepted merely because their syntax parses.

6.3 Domain extension

Adds a computational domain.

Examples:

quantum
HDL
AI
distributed computing
scientific computing

6.4 Target extension

Adds support for a target capability without changing source semantics.

Examples:

GPU
FPGA
QPU
ASIC
CPU architecture
future accelerator

Target extensions MUST NOT require source-level rewrites when the program's semantic requirements remain satisfiable.

6.5 Dialect extension

Adds specialized syntax under an explicitly declared namespace and compatibility boundary.

6.6 Tooling extension

Adds:

- diagnostics;
- formatting;
- syntax highlighting;
- IDE support;
- language-server support;
- documentation generation;
- static analysis.

Tooling extensions must consume the authoritative grammar and semantic model rather than inventing independent syntax.

---

7. Extension Levels

Extensions are classified into the following levels.

Level 0 — Core

Stable Zamani syntax and semantics.

Level 1 — Standard

Official Zamani extensions maintained by the project.

Level 2 — Domain

Official domain extensions such as:

quantum
classical
hdl
hardware
distributed
ai
data
networking

Level 3 — Experimental

Extensions under active development.

Experimental extensions MUST be explicitly identifiable.

Level 4 — External

Third-party extensions.

External extensions MUST NOT silently redefine standard syntax or semantics.

Level 5 — Vendor

Vendor-specific features.

Vendor extensions MUST remain isolated from the portable language core.

---

8. Namespace Isolation

Every non-core extension MUST have a stable namespace.

Conceptually:

core::<name>
quantum::<name>
hdl::<name>
hardware::<name>
distributed::<name>
ai::<name>
vendor::<vendor>::<name>

The exact source syntax is determined by the authoritative grammar.

Extensions MUST NOT introduce globally ambiguous identifiers.

A vendor MUST NOT reserve an unqualified keyword merely because a device happens to use that concept.

---

9. Reserved Vocabulary

New keywords MUST NOT be introduced casually.

A proposed keyword requires:

1. semantic justification;
2. syntax justification;
3. collision analysis;
4. compatibility analysis;
5. parser ambiguity analysis;
6. tooling impact analysis;
7. migration strategy;
8. documentation;
9. tests.

Prefer contextual syntax or namespaced declarations when a globally reserved keyword is unnecessary.

This is essential for long-term POCO-REAF compatibility.

---

10. Extension Declaration Contract

An extension mechanism MUST provide enough information for the compiler to determine:

extension identity
extension namespace
extension version
language version compatibility
required capabilities
provided capabilities
semantic ownership
syntax ownership
dependencies
stability level
compatibility policy
lowering boundary

Conceptually:

extension <namespace> {
    version ...
    requires ...
    provides ...
    compatibility ...
}

The exact concrete syntax MUST be established by the authoritative grammar.

This document does not independently define a competing grammar.

---

11. Capability-Based Extension

Extensions MUST prefer capability declarations over target identifiers.

Bad:

use_device("specific_qpu")

Better:

requires {
    quantum;
    measurement;
    dynamic_control;
}

The compiler/runtime may then discover a suitable implementation.

A program may express:

requires quantum
requires capability(dynamic_measurement)
requires resource(qubits >= required)

without embedding a fixed machine.

The number of resources required by a program is a semantic/resource property when genuinely required by the computation. It must not become an arbitrary grammar maximum.

---

12. Requirements, Constraints, Preferences and Hints

These concepts MUST remain distinct.

Requirement

A property that must be satisfied.

requires quantum

Constraint

A property that must not be violated.

constraint latency < bound

Preference

A desirable property.

prefer low_latency

Hint

A non-authoritative suggestion.

hint parallel

The compiler MUST NOT interpret a hint as a mandatory physical requirement.

---

13. Resource Independence

Extensions MUST NOT introduce fixed resource ceilings.

Forbidden architectural assumptions include:

MAX_QUBITS = 32
MAX_CORES = 64
MAX_THREADS = 128
MAX_GPUS = 8
MAX_NODES = 1024

Such limits may exist internally in a particular implementation only when they represent an actual implementation boundary, and they MUST NOT become language semantics.

The source language must remain capable of representing programs whose required resources exceed the current implementation.

---

14. Quantum Extension Rules

Quantum extensions MUST preserve backend independence.

Quantum syntax may describe:

- qubits;
- logical qubits;
- physical qubits where explicitly meaningful;
- registers;
- states;
- gates;
- operations;
- measurements;
- observables;
- reset;
- dynamic circuits;
- mid-circuit measurement;
- classical control;
- parameterized operations;
- controlled operations;
- error-correction declarations;
- quantum resources;
- quantum capabilities.

However:

syntax ≠ physical allocation
syntax ≠ topology
syntax ≠ scheduling
syntax ≠ calibration
syntax ≠ noise model

A quantum extension MUST NOT assume:

q[0]
q[1]

as a universal physical topology.

If an extension needs physical qubits, it must express the semantic requirement explicitly and allow the hardware layer to perform mapping.

---

15. QEC Boundary

Grammar extensions may express QEC intent, such as:

logical qubit
code requirement
fault-tolerance requirement
syndrome-related intent

but grammar MUST NOT implement QEC algorithms.

QEC remains owned by the QEC subsystem.

The extension pipeline is:

source
→ AST
→ semantic QEC intent
→ canonical quantum representation
→ QEC subsystem

The grammar must not duplicate:

- stabilizer algorithms;
- decoder implementations;
- syndrome schedulers;
- code-distance algorithms;
- correction algorithms.

---

16. ZQN Boundary

ZQN owns fault/noise semantics.

Grammar extensions may declare relevant source-level requirements or policies, but MUST NOT duplicate ZQN's fault model.

The boundary is:

Zamani source
→ semantic intent
→ ZQN-compatible representation
→ ZQN fault/noise processing

A vendor's noise terminology MUST NOT become a universal Zamani semantic concept merely because one backend uses it.

---

17. Scheduling Boundary

Scheduling extensions may express:

ordering constraints
dependency constraints
timing requirements
latency requirements
synchronization intent

They must not implement the scheduling engine.

The scheduling subsystem owns:

- dependency scheduling;
- resource-aware scheduling;
- timing;
- ASAP/ALAP;
- critical-path analysis;
- alignment;
- dynamic scheduling;
- resource conflicts.

---

18. Routing Boundary

Source-level extensions may express placement requirements where semantically meaningful.

They must not own routing algorithms.

Routing owns:

logical → physical mapping
topology realization
connectivity constraints
placement optimization

A source program must not need to encode a machine topology simply to execute.

---

19. Hardware and HDL Extensions

HDL extensions may describe:

- modules;
- ports;
- signals;
- wires;
- registers;
- clocks;
- timing;
- processes;
- state machines;
- memories;
- pipelines;
- interfaces.

Hardware extensions may describe:

- target capabilities;
- resources;
- accelerators;
- hardware classes;
- implementation constraints.

However:

hardware semantic description

must remain distinct from:

specific machine instance

A hardware extension must not force all future hardware to conform to today's physical architecture.

---

20. Dialect Architecture

A dialect is a controlled extension of Zamani syntax and/or semantics.

Every dialect MUST define:

name
namespace
version
stability
owner
syntax additions
semantic additions
dependencies
capabilities
lowering
compatibility
deprecation policy

Dialect registration belongs conceptually under:

grammar/dialects/

with supporting specifications under:

grammar/specification/

The dialect system must not require modifying unrelated core grammar rules whenever a dialect is added.

---

21. Dialect Independence

A dialect MUST NOT directly depend on a concrete backend implementation.

Forbidden dependency:

dialect → IBM device implementation
dialect → NVIDIA device implementation
dialect → FPGA vendor implementation

Preferred:

dialect
  ↓
semantic capability
  ↓
canonical representation
  ↓
target lowering

---

22. Dialect Composition

Multiple dialects must be composable when their semantics are compatible.

Examples:

classical + quantum
classical + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

Composition MUST be validated explicitly.

The compiler MUST reject conflicting semantic definitions rather than silently choosing one.

---

23. Extension Conflicts

Potential conflicts include:

- duplicate keywords;
- duplicate namespaces;
- incompatible types;
- incompatible effects;
- conflicting ownership;
- incompatible lowering;
- ambiguous syntax;
- incompatible version requirements;
- contradictory capabilities;
- semantic reinterpretation of existing constructs.

Conflict resolution MUST be deterministic.

There must be no "whichever extension loaded first wins" semantics.

---

24. Semantic Stability

An extension MUST NOT change the meaning of an existing standard construct merely by being imported.

For example, importing a quantum dialect must not redefine:

fn
let
if
for
return
type
module

unless the language specification explicitly defines a contextual extension.

Existing standard programs must remain semantically stable.

---

25. Extension Versioning

Every stable extension MUST have a version.

Compatibility MUST distinguish:

language version
grammar version
dialect version
semantic version
IR version
backend version
runtime version

These versions MUST NOT be conflated.

A grammar change does not automatically imply an IR change.

An IR change does not automatically imply a source-language breaking change.

---

26. Backward Compatibility

Extensions MUST define whether a release is:

backward compatible
forward compatible
source compatible
AST compatible
semantic compatible
IR compatible
runtime compatible

Compatibility claims must be testable.

Deprecated syntax must remain documented for the supported migration period.

---

27. Forward Compatibility

Unknown extensions must not cause corruption.

Where possible, the parser/AST architecture should preserve extension boundaries sufficiently for tools to report:

unsupported extension
unsupported dialect
unsupported version
unsupported capability

rather than producing misleading semantic interpretations.

---

28. Extension Lowering

Every semantic extension MUST define its lowering boundary.

Example:

Source Extension
      ↓
AST
      ↓
Semantic Extension Model
      ↓
Canonical IR
      ↓
Optimization
      ↓
Target Lowering

Extensions must not directly bypass semantic validation.

A dialect that generates backend instructions directly from syntax violates the architecture unless the feature is explicitly defined as a target-specific compilation layer outside the portable language.

---

29. Canonical IR Rule

No extension may introduce an alternative canonical IR for a domain already represented by Zamani.

For quantum:

quantum syntax
→ quantum::ir

For classical computation, use the repository's canonical classical/semantic IR.

For hardware, use the repository's canonical hardware representation.

Extensions may define temporary AST or semantic structures, but these must have an explicit ownership and lowering boundary.

---

30. AST Requirements

AST extension nodes MUST contain source semantics rather than backend state.

AST nodes should preserve information such as:

source location
syntax kind
identifier
arguments
attributes
generic parameters
requirements
constraints
effects
dialect identity
extension version

AST nodes MUST NOT contain:

live hardware handles
runtime device objects
calibration state
scheduler state
backend connections
mutable execution state

---

31. Determinism

Extension discovery, registration, parsing, validation, and lowering MUST be deterministic.

Given identical:

source
grammar version
dialect versions
extension set
compiler configuration

the compiler must produce the same semantic result.

No extension may depend on:

- hash-map iteration order;
- filesystem enumeration order;
- network response ordering;
- nondeterministic plugin loading;
- machine-local incidental state.

---

32. Extension Discovery

Extension discovery must be separated from source semantics.

The language should distinguish:

extension available

from:

extension requested

A program should not silently acquire semantics merely because an extension happens to be installed.

---

33. No Network-Dependent Parsing

Parsing MUST NOT require network access.

The grammar frontend must not dynamically download grammar definitions or execute arbitrary remote code.

Extension metadata may be resolved by tooling/build infrastructure, but parsing a previously resolved source program must remain deterministic and offline-capable.

---

34. Security

Extensions must be treated as untrusted input unless explicitly trusted.

The Rust implementation MUST use safe Rust only.

Requirements:

- no "unsafe";
- no arbitrary native execution during parsing;
- no arbitrary filesystem access from grammar parsing;
- no arbitrary network access from parsing;
- no secret extraction;
- no implicit command execution;
- bounded resource handling at implementation boundaries;
- explicit diagnostics for unsupported extensions.

An extension mechanism must not become a general arbitrary-code-execution mechanism.

---

35. Macros and Metaprogramming

Macros and metaprogramming are extensions of the language, but must remain distinct from dialect registration.

Macros operate on language representations.

Dialect registration establishes language capabilities.

Metaprogramming must not silently modify the grammar itself during ordinary program compilation.

If grammar-generation capabilities are ever introduced, they must operate through an explicit compiler/build phase with deterministic inputs and versioned outputs.

---

36. Macro Hygiene

Macro extensions MUST prevent accidental capture of:

- identifiers;
- namespaces;
- bindings;
- capabilities;
- effects;
- resources.

Macro expansion must preserve source provenance.

Generated code must remain attributable to:

source location
macro identity
macro version
expansion location

---

37. Compile-Time Extensions

Compile-time features may compute:

- constants;
- generated types;
- specialized implementations;
- static metadata;
- resource expressions;
- compile-time transformations.

They MUST NOT make runtime hardware assumptions part of source semantics.

A compile-time computation must remain deterministic unless explicitly classified otherwise.

---

38. Runtime Extensions

Runtime capabilities must remain outside the grammar's semantic ownership.

Examples:

backend discovery
device selection
calibration
telemetry
scheduling
resource allocation
fault recovery

The grammar can express requirements and policies.

The runtime decides how those requirements are satisfied.

---

39. Resilience Integration

Resilience remains a decision/orchestration layer.

An extension may express:

fault tolerance requirement
availability requirement
recovery policy
semantic safety requirement

but must not implement resilience itself.

The architecture remains:

grammar
→ semantic intent
→ resilience
→ QEC / ZQN / routing / scheduling / hardware / runtime

---

40. Resource Scalability

Resource expressions MUST be mathematically and semantically extensible.

The language must support resource requirements that depend on:

- program structure;
- data size;
- algorithmic complexity;
- target capabilities;
- runtime availability;
- dynamically determined workload.

The grammar MUST NOT impose arbitrary numeric limits merely because the current compiler implementation uses a particular integer representation.

Implementation limits must be checked at implementation boundaries and reported diagnostically.

---

41. "Infinity" and Practical Meaning

"Scale to infinity" means:

«The language imposes no artificial finite architectural ceiling on the semantic model.»

It does NOT mean that a machine with finite resources can execute an infinite computation.

Execution remains bounded by:

available memory
available compute
available storage
available quantum resources
available time
available energy
backend capabilities
runtime limits

The language must therefore distinguish:

semantic scalability

from:

physical resource availability

---

42. Extension Failure Model

An extension must fail explicitly when:

- required syntax is unsupported;
- required semantic capability is unsupported;
- dialect versions conflict;
- lowering is unavailable;
- capability requirements cannot be satisfied;
- extension dependencies are missing;
- extension semantics conflict;
- compatibility is violated.

It must never silently downgrade a semantic requirement.

---

43. Diagnostics

Extension diagnostics MUST identify:

extension
dialect
version
source location
failure category
required capability
available capability
compatibility status
suggested migration where applicable

Diagnostics must be stable enough for tooling to consume programmatically.

---

44. Tooling Integration

The extension model MUST integrate with:

lexer
parser
AST
semantic analyzer
type checker
effect checker
capability checker
formatter
language server
documentation generator
compiler
IR
tests

Tooling must obtain extension information from authoritative metadata rather than independently maintaining keyword lists.

---

45. Grammar Integration

The authoritative grammar remains:

grammar/Zamani.g4

unless the grammar architecture is deliberately split into imported grammar fragments.

If grammar fragments are introduced, the build system MUST establish a deterministic composition mechanism.

The architecture must not permit multiple competing definitions of the same production.

ANTLR grammar composition must respect ANTLR's grammar/import constraints and naming conventions.

---

46. Proposed Repository Integration

The extensibility specification integrates with:

grammar/Zamani.g4
grammar/specification/grammar-authority.md
grammar/specification/language-version.md
grammar/specification/compatibility.md
grammar/specification/semantic-model.md
grammar/specification/compilation-model.md
grammar/specification/execution-model.md
grammar/specification/scalability-model.md
grammar/specification/poco-reaf.md

grammar/dialects/
grammar/macros/
grammar/metaprogramming/
grammar/validation/
grammar/compatibility/
grammar/tests/

Domain integration includes:

src/quantum/
src/quantum/ir/
src/quantum/qec/
src/quantum/zqn/
src/quantum/scheduling/
src/quantum/optimization/
src/quantum/hardware/
src/quantum/resilience/

The grammar must consume the contracts of these subsystems; it must not duplicate their ownership.

---

47. Dependency Direction

The dependency direction MUST remain:

grammar specification
        ↓
grammar
        ↓
AST
        ↓
semantic analysis
        ↓
canonical IR
        ↓
optimization / transformation
        ↓
routing
        ↓
scheduling
        ↓
QEC / ZQN / hardware / resilience
        ↓
runtime

Never:

runtime → grammar
hardware implementation → grammar
scheduler → grammar
QEC algorithm → grammar

A compiler frontend may know about semantic contracts, but backend implementations must not redefine source grammar.

---

48. Extension Registration

The extension registry must define metadata, not arbitrary execution behavior.

A registration record should conceptually contain:

ExtensionId
Namespace
Version
Stability
LanguageCompatibility
Dependencies
ProvidedCapabilities
RequiredCapabilities
SyntaxSurface
SemanticSurface
LoweringBoundary
CompatibilityPolicy

These concepts belong to the extension model and should eventually be represented by the repository's canonical metadata/registry infrastructure.

---

49. Extension Identity

Extension identifiers MUST be stable.

An extension ID must not depend on:

- memory address;
- process ID;
- device ID;
- random value;
- machine-specific path.

If identity is persisted, it must remain stable across machines.

---

50. Extension Dependencies

Dependencies MUST be explicit.

An extension may require:

another extension
language version
capability
semantic facility
IR version

Dependencies must form a directed graph.

Cycles must be rejected unless a future specification explicitly defines a sound mutually recursive extension mechanism.

---

51. Capability Negotiation

Capability negotiation must occur after parsing and semantic identification.

Conceptually:

Program Requirements
        ↓
Required Capabilities
        ↓
Available Capabilities
        ↓
Compatibility Check
        ↓
Plan

A target is selected because it satisfies semantic requirements, not because the source happens to name that target.

---

52. Portable versus Target-Specific Features

Every feature must be classified as:

portable
conditional
target-specific
vendor-specific
experimental

A target-specific feature MUST be visibly distinguishable from portable semantics.

Target-specific syntax must never masquerade as universally portable behavior.

---

53. Vendor Extensions

Vendor extensions may provide:

- hardware-specific instructions;
- special capabilities;
- diagnostics;
- optimization hints;
- device-specific controls.

They must be isolated behind a namespace.

Vendor extensions MUST NOT alter the semantics of standard Zamani constructs.

---

54. Future Technology Rule

A future technology must be representable by adding an extension or target implementation without requiring a rewrite of the core language.

Examples include future:

quantum architectures
photonic processors
neuromorphic systems
new accelerators
new memory architectures
new interconnects
new distributed models
new hardware description technologies
new computational paradigms

The language core must remain smaller than the set of all possible future targets.

---

55. Avoiding the "Everything Becomes a Keyword" Problem

The existing Zamani grammar already contains a very broad vocabulary spanning mathematical operations, quantum constructs, HDL, effects, plugins, macros and other domains.

Future expansion MUST NOT continue indefinitely by adding every new domain concept as a global keyword.

Prefer:

types
namespaces
modules
attributes
capabilities
declarations
generic abstractions
dialects

over uncontrolled global keyword growth.

This is essential for parser stability and long-term language evolution.

---

56. Extension and Type System

Extensions introducing types MUST define:

type identity
type parameters
type constraints
ownership semantics
layout semantics, if applicable
conversion rules
equality semantics
serialization semantics, if applicable
IR lowering

A type extension must not secretly encode a fixed hardware representation unless representation is explicitly part of the type's semantics.

---

57. Extension and Effects

Extensions introducing effects MUST identify:

effect identity
effect inputs
effect outputs
effect interactions
effect propagation
effect composition
effect handling

Hardware, quantum, network, IO, distributed and security effects must remain distinguishable.

---

58. Extension and Memory

Memory-related extensions must distinguish:

semantic storage
physical memory
allocation policy
placement
address space
coherence
distribution

A source-level data structure must not inherently imply a particular physical memory technology.

---

59. Extension and Concurrency

Concurrency extensions must express semantics such as:

parallelism
ordering
synchronization
communication
cancellation
isolation

They must not require a particular:

thread count
core count
CPU topology
GPU topology
node count

unless explicitly expressed as a target constraint.

---

60. Extension and AI/Data

AI/data extensions must preserve abstraction over:

tensor dimensions
accelerator type
memory layout
device count
distributed topology

where these are implementation details.

The semantic model must remain portable.

---

61. Extension and Networking

Networking extensions may describe:

protocol requirements
communication semantics
endpoint requirements
message schemas
delivery guarantees
security requirements
latency requirements

They must not embed a fixed network topology into portable program semantics.

---

62. Extension and Security

Security extensions must make security properties explicit.

Examples:

requires authentication
requires confidentiality
requires integrity
requires capability X
requires trusted execution

Security requirements must not be silently weakened when a target cannot satisfy them.

---

63. Compatibility Matrix

Every stable extension should be represented in the repository compatibility matrix with:

extension
version
language versions
AST versions
IR versions
runtime compatibility
target compatibility
status
deprecated status
migration path

This belongs with:

grammar/compatibility/

and must be tested.

---

64. Deprecation

An extension feature may be deprecated only when:

1. replacement exists;
2. migration is documented;
3. compatibility policy permits deprecation;
4. diagnostics identify the deprecated feature;
5. tests preserve the supported compatibility period.

Deprecation must not silently change semantics.

---

65. Removal

Removal requires:

deprecation history
compatibility analysis
migration documentation
release policy compliance
test updates
documentation updates

No extension may be removed merely because its implementation is inconvenient.

---

66. Testing Requirements

Every extension requires:

Syntax tests

Valid syntax parses.

Negative tests

Invalid syntax fails.

Semantic tests

Valid syntax receives the correct meaning.

Conflict tests

Conflicting extensions fail deterministically.

Version tests

Compatible versions succeed.

Incompatible versions fail.

Namespace tests

Collisions are rejected.

Capability tests

Requirements are validated.

Scalability tests

No artificial resource ceilings exist.

Determinism tests

Repeated compilation produces equivalent semantic output.

Cross-domain tests

At minimum:

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

67. Extension Test Independence

An extension's tests MUST NOT depend on a particular physical machine unless the test is explicitly classified as target-specific.

Portable extension tests must run against:

small resources
large resources
different target capabilities
simulators
hardware

where applicable.

---

68. Hard-Coding Audit

Every extension MUST undergo an automated and manual hard-coding audit.

Search for:

MAX_*
fixed device IDs
fixed qubit counts
fixed core counts
fixed thread counts
fixed node counts
fixed memory sizes
fixed topology
fixed addresses
fixed accelerator counts
fixed tensor dimensions

Every finding must be classified as:

semantic requirement
target requirement
resource constraint
implementation limitation
test limitation
documentation limitation
accidental hard-coding

Accidental hard-coding MUST be removed.

---

69. No Unsafe Rust

The grammar/compiler implementation governed by this specification MUST use safe Rust.

Requirements:

Rust 1.97 or Rust 1.97.1
no unsafe blocks
no unsafe functions
no unsafe traits
no unsafe implementations

The extension architecture must not require unsafe Rust.

Foreign-function boundaries must be isolated behind safe abstractions whose implementation remains compliant with the project's no-"unsafe" requirement.

---

70. Performance and Scalability

Extensibility must not require linear or otherwise unnecessary global work for every extension.

The compiler should be able to determine:

which extensions are relevant
which namespaces are active
which dialects are required
which capabilities are required

without processing unrelated extension definitions unnecessarily.

Data structures must scale with the actual program and active extension set.

No fixed global extension count is part of the language.

---

71. Parser Scalability

Parser architecture must not impose artificial limits on:

source-file size
declaration count
module count
expression count
quantum operation count
hardware declarations
resource expressions
extension count
dialect count

Any implementation limits must be explicit resource safeguards rather than language semantics.

---

72. Error Recovery

Parser error recovery must remain deterministic.

An unsupported extension must not cause unrelated source to be interpreted as another language construct.

Diagnostics should identify the nearest valid extension boundary.

---

73. Source Provenance

Extension-generated AST and semantic structures must preserve provenance.

At minimum:

source file
source range
extension/dialect
extension version
macro expansion, if applicable
generated/original status

This is required for:

- diagnostics;
- debugging;
- auditing;
- reproducibility;
- IDE tooling;
- semantic verification.

---

74. Reproducibility

An extension build must be reproducible from declared inputs.

Inputs include:

source
grammar version
language version
extension versions
dialect versions
compiler version
relevant configuration

Undeclared extension behavior is prohibited.

---

75. Extension Documentation Contract

Every production extension MUST provide:

README
syntax specification
semantic specification
version
compatibility
dependencies
examples
positive tests
negative tests
integration tests
migration information

Experimental extensions must additionally state their instability.

---

76. Completion Contract for an Extension

An extension is NOT complete merely because its grammar parses.

It is complete only when:

syntax
+
AST
+
semantic model
+
validation
+
canonical lowering
+
compatibility
+
diagnostics
+
tests
+
documentation
+
tooling integration

are complete.

---

77. Completion Contract for a Grammar File

Every grammar file implementing extensibility MUST document:

File
Purpose
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
Tests
Negative Tests
Boundary Tests
Compatibility Requirements
Scalability Requirements
Hard-Coding Audit
Completion Criteria

A file is not complete until all of these are known.

---

78. Integration With "grammar/dialects/"

The following responsibilities belong under the dialect subsystem:

dialects.g4
registration.g4
namespaces.g4
versioning.g4
capabilities.g4
vendor.g4
experimental.g4
compatibility.g4

These files MUST consume this specification.

They MUST NOT redefine:

- the canonical type system;
- quantum IR;
- QEC;
- ZQN;
- scheduling;
- routing;
- hardware discovery.

---

79. Integration With "grammar/macros/"

Macro files own:

macro declarations
macro invocation
hygiene
expansion

They must consume the core syntax and produce valid Zamani syntax/AST structures.

Macros must not silently register new language keywords.

---

80. Integration With "grammar/metaprogramming/"

Metaprogramming owns:

compile-time execution
reflection
generation
specialization

It must remain separate from runtime execution and hardware discovery.

---

81. Integration With "grammar/validation/"

Validation must enforce this specification through:

ambiguity checks
semantic boundary checks
scalability checks
compatibility checks
hard-coding checks
naming checks

The validation system becomes the enforcement layer for the extensibility contract.

---

82. Integration With "grammar/compatibility/"

Compatibility owns:

versions
migrations
deprecations
reserved space
compatibility matrix

Extensibility depends on these contracts rather than duplicating them.

---

83. Integration With Existing Zamani Grammar

The current grammar already includes broad constructs for modules, imports/exports, functions, effects, quantum declarations, HDL declarations, plugins, macros, packages, and mathematical constructs.

Therefore migration must proceed by:

inventory
→ classify
→ preserve valid constructs
→ establish ownership
→ split oversized grammar areas where necessary
→ eliminate duplication
→ introduce extension contracts
→ add compatibility tests

Existing valid syntax must not be silently removed.

---

84. Existing "unsafe" Syntax

The existing grammar contains an "unsafe" modifier/block concept.

This specification does not equate that language-level concept with Rust "unsafe".

The repository's implementation requirement is:

«Rust implementation code MUST contain no "unsafe".»

If Zamani's language-level "unsafe" feature is retained, its semantic meaning must be specified independently in the language specification and must not require Rust "unsafe".

If the language-level feature is removed, that removal requires the normal deprecation and compatibility process.

---

85. Repository-Wide Integration Principle

The grammar is one layer of Zamani, not the entire compiler.

The complete architecture remains:

Zamani Source
    ↓
Lexer
    ↓
Parser
    ↓
AST
    ↓
Semantic Analysis
    ↓
Capability / Effect / Resource Validation
    ↓
Canonical IR
    ↓
Optimization
    ↓
Routing
    ↓
Scheduling
    ↓
ZQN / QEC / Resilience
    ↓
Hardware Abstraction
    ↓
Runtime

The extension system must fit inside this architecture rather than bypass it.

---

86. What Extensions Must Never Do

An extension MUST NOT:

- redefine standard semantics silently;
- introduce hidden machine limits;
- select a physical device merely because it is installed;
- embed physical topology into portable semantics;
- create a duplicate canonical quantum IR;
- implement QEC algorithms in grammar;
- implement ZQN noise models in grammar;
- implement routing in grammar;
- implement scheduling in grammar;
- implement hardware discovery in grammar;
- execute arbitrary code during parsing;
- require network access to parse source;
- introduce nondeterministic semantics;
- silently downgrade requirements;
- depend on undocumented compiler behavior;
- use unsafe Rust.

---

87. Extension Lifecycle

Every extension follows:

Proposal
   ↓
Design
   ↓
Namespace Allocation
   ↓
Syntax Definition
   ↓
Semantic Definition
   ↓
Compatibility Analysis
   ↓
AST Contract
   ↓
IR Contract
   ↓
Implementation
   ↓
Validation
   ↓
Tests
   ↓
Documentation
   ↓
Experimental
   ↓
Stable
   ↓
Maintenance
   ↓
Deprecation
   ↓
Removal

Skipping these stages is permitted only for explicitly experimental internal work.

---

88. Production Readiness Checklist

An extension is production-ready only when all are true:

- [ ] Namespace defined.
- [ ] Identifier defined.
- [ ] Version defined.
- [ ] Stability defined.
- [ ] Syntax defined.
- [ ] Semantic meaning defined.
- [ ] Ownership defined.
- [ ] Non-ownership defined.
- [ ] AST contract defined.
- [ ] Type interactions defined.
- [ ] Effect interactions defined.
- [ ] Capability interactions defined.
- [ ] Resource interactions defined.
- [ ] IR lowering defined.
- [ ] Canonical IR boundary respected.
- [ ] Compiler integration defined.
- [ ] Runtime integration defined.
- [ ] Tooling integration defined.
- [ ] Compatibility defined.
- [ ] Migration defined.
- [ ] Diagnostics defined.
- [ ] Positive tests implemented.
- [ ] Negative tests implemented.
- [ ] Boundary tests implemented.
- [ ] Cross-domain tests implemented.
- [ ] Determinism tests implemented.
- [ ] Scalability tests implemented.
- [ ] Hard-coding audit passed.
- [ ] Security audit passed.
- [ ] No Rust "unsafe".
- [ ] Documentation complete.

---

89. Definition of Done for "extensibility.md"

This specification is considered integrated when:

1. "grammar/Zamani.g4" is governed by a single extensibility model.
2. Dialects have explicit ownership and namespaces.
3. Extensions cannot silently redefine core semantics.
4. Extension versions are explicit.
5. Extension dependencies are explicit.
6. Capability negotiation is separated from syntax.
7. Resource requirements are separated from hardware implementation.
8. Quantum extensions lower toward "quantum::ir".
9. QEC remains owned by QEC.
10. ZQN remains owned by ZQN.
11. Routing remains owned by routing.
12. Scheduling remains owned by scheduling.
13. Hardware discovery remains owned by hardware/HAL.
14. Runtime execution remains owned by runtime.
15. Extensions cannot impose arbitrary machine limits.
16. Extension behavior is deterministic.
17. Extension errors are explicit.
18. Extension provenance is preserved.
19. Tooling can discover extension metadata.
20. Compatibility is versioned.
21. Extensions can compose safely.
22. Future domains can be added without redesigning the core language.
23. Rust implementation remains entirely safe Rust.
24. The architecture supports POCO-REAF.

---

90. Final Principle

Zamani's extensibility architecture MUST preserve the following invariant:

                         EXTENSIBILITY
                              │
             ┌────────────────┼────────────────┐
             │                │                │
          Domains          Dialects          Targets
             │                │                │
             └────────────────┼────────────────┘
                              │
                       Portable Semantics
                              │
                       Canonical IR
                              │
                    Target Adaptation
                              │
                       Runtime Execution

The extension system exists to make Zamani capable of evolving indefinitely without making today's hardware, compiler implementation, vendor ecosystem, or execution environment the permanent definition of the language.

Therefore:

«Zamani extensions may expand what the language can express, but they must never unnecessarily constrain where, how, or at what scale that computation can execute.»

The ultimate invariant is:

ONE PROGRAM
    ↓
ONE PORTABLE SEMANTIC MEANING
    ↓
MANY EXTENSIONS
    ↓
MANY COMPILATION STRATEGIES
    ↓
MANY HARDWARE CONFIGURATIONS
    ↓
MANY EXECUTION ENVIRONMENTS
    ↓
FUTURE COMPUTING SYSTEMS

while preserving:

Program Once
Compile Once
Run Everywhere
Run Anywhere
Run Forever

POCO-REAF is therefore not a backend feature. It is an architectural constraint on the language itself.