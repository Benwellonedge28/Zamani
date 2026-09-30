Zamani Compatibility Architecture

Path: "grammar/compatibility/README.md"
Status: Normative compatibility architecture and navigation contract
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Grammar technology: ANTLR4
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety requirement: Production Zamani Rust implementation MUST use safe Rust. Rust "unsafe" MUST NOT be required or used.
Primary objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)
Scalability objective: From the smallest valid computation to arbitrarily large computation, subject only to program semantics, available resources, target capabilities, physical constraints, and implementation capacity.

---

1. Purpose

This file is the entry point and integration contract for:

grammar/compatibility/

It explains how Zamani compatibility is organized across the repository and establishes the boundaries between:

- language versions;
- source compatibility;
- lexical compatibility;
- grammar compatibility;
- parser compatibility;
- AST compatibility;
- semantic compatibility;
- type compatibility;
- effect compatibility;
- resource compatibility;
- capability compatibility;
- quantum compatibility;
- classical compatibility;
- HDL/hardware compatibility;
- hybrid compatibility;
- distributed compatibility;
- AI/data compatibility;
- networking compatibility;
- security compatibility;
- dialect compatibility;
- canonical IR compatibility;
- artifact compatibility;
- ABI compatibility;
- runtime compatibility;
- target compatibility;
- migration;
- deprecation;
- diagnostics;
- tooling;
- generated documentation;
- conformance testing.

This file is intentionally an architecture and navigation contract.

It MUST NOT become a second language specification.

It MUST NOT duplicate the complete contents of:

compatibility/versions.md
compatibility/migrations.md
compatibility/deprecated.md
compatibility/compatibility-matrix.md
spec/compatibility.md
spec/versioning.md
specification/language-version.md

Those files retain their respective ownership.

---

2. Core Principle

The fundamental compatibility rule is:

«A compatible Zamani implementation MUST NOT silently change the specified meaning of valid existing Zamani source code.»

When a change intentionally breaks compatibility, that break MUST be:

1. explicitly classified;
2. assigned to an appropriate version boundary;
3. documented;
4. diagnosable;
5. testable;
6. represented in compatibility metadata;
7. migratable where practical;
8. reflected in the repository compatibility matrix.

Implementation convenience MUST NOT silently become a language-breaking change.

Temporary hardware limitations MUST NOT become language limitations.

Backend limitations MUST NOT become source-language limitations.

Compiler implementation limitations MUST NOT become permanent semantic restrictions.

---

3. POCO-REAF Compatibility Objective

Zamani compatibility exists to preserve the long-term portability of source programs.

The objective is:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Anywhere
      ↓
Forever

POCO-REAF does not mean that one physical machine code artifact must execute unchanged on every possible machine.

It means that the specified source semantics and portable program intent remain stable while implementation-specific realization may change according to:

- available resources;
- target capabilities;
- target constraints;
- topology;
- timing;
- memory;
- accelerators;
- quantum-device capabilities;
- hardware generations;
- runtime facilities;
- scheduling;
- routing;
- resilience;
- QEC;
- ZQN;
- deployment policy.

Therefore compatibility MUST preserve the program's meaning independently from any particular:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- accelerator;
- node;
- memory bank;
- physical qubit;
- network device;
- register width;
- device count;
- hardware generation.

---

4. Authority Model

The compatibility directory participates in the repository-wide authority hierarchy.

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ├── normative language meaning
        ├── language version semantics
        └── domain semantics
        │
        ▼
grammar/spec/
        │
        ├── focused formal contracts
        ├── compatibility contract
        └── versioning contract
        │
        ▼
grammar/compatibility/
        │
        ├── versions.md
        ├── migrations.md
        ├── deprecated.md
        ├── compatibility-matrix.md
        └── README.md
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
        ├───────────────┬────────────────┐
        ▼               ▼                ▼
    classical       quantum::ir       HDL/hardware
        │               │                │
        └───────────────┼────────────────┘
                        ▼
              optimization / lowering
                        │
            ┌───────────┼───────────┐
            ▼           ▼           ▼
         routing    scheduling   resilience
                                    │
                              ┌─────┼─────┐
                              ▼     ▼     ▼
                             QEC   ZQN   other
                                    │
                                    ▼
                                   HAL
                                    │
                                    ▼
                              target/runtime

Compatibility metadata describes the relationships between these layers.

It MUST NOT redefine their ownership.

---

5. Compatibility Files and Their Responsibilities

The compatibility directory intentionally separates concerns.

File| Owns| Does not own
"README.md"| Compatibility architecture, navigation, integration boundaries| Individual version/migration/deprecation decisions
"versions.md"| Language/release version policy| Individual migration algorithms
"migrations.md"| Migration procedures and transformations| Version-number semantics
"deprecated.md"| Deprecation lifecycle and removal policy| Migration implementation
"compatibility-matrix.md"| Cross-layer compatibility relationships| Grammar syntax
"reserved.md" if present/added| Reserved identifiers/syntax| General version policy

If "reserved.md" is not yet present, it SHOULD be introduced only when reserved syntax/identifier policy requires a separately owned contract. It MUST NOT be created merely to duplicate information already owned elsewhere.

---

6. Relationship to Existing Repository Files

6.1 "grammar/DESIGN.md"

Owns the architectural principles.

Compatibility MUST conform to:

- one language;
- deterministic parsing;
- domain-neutral AST;
- canonical semantic boundaries;
- canonical "quantum::ir";
- no artificial hardware limits;
- separation of syntax, semantics, resources, capabilities, routing, scheduling, QEC, ZQN, HAL, and runtime;
- POCO-REAF.

---

6.2 "grammar/specification/"

This is the normative language specification.

It defines what Zamani means.

Compatibility policy MUST never change language semantics merely by changing compatibility metadata.

Relevant specification areas include:

grammar/specification/language.md
grammar/specification/language-version.md
grammar/specification/lexical.md
grammar/specification/syntax.md
grammar/specification/semantics.md
grammar/specification/types.md
grammar/specification/portability.md
grammar/specification/domains.md

Where an applicable file exists under the repository, compatibility documents reference that file rather than recreating its contents.

---

6.3 "grammar/spec/"

This contains focused formal contracts.

In particular:

grammar/spec/compatibility.md
grammar/spec/versioning.md

"spec/compatibility.md" defines compatibility dimensions.

"spec/versioning.md" defines cross-layer version propagation and implementation conformance.

This directory MUST remain consistent with "specification/".

---

6.4 "grammar/Zamani.g4"

"Zamani.g4" remains the canonical ANTLR composition root.

Compatibility policy MUST NOT create another root grammar.

Compatibility MUST NOT be implemented by creating:

grammar/compatibility/Zamani.g4
grammar/antlr/Zamani.g4

as competing canonical grammars.

Historical syntax may be represented through explicitly versioned compatibility rules where the specification requires it, but there MUST remain one canonical composition architecture.

---

6.5 "grammar/grammar.md"

This remains the implementation-conformance reference.

It must distinguish at least:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
EXPERIMENTAL
DEPRECATED
PLANNED
UNIMPLEMENTED

Compatibility metadata MAY be reflected there.

However:

«"grammar.md" MUST NOT decide language compatibility merely because a construct is present or absent from the generated/current grammar.»

---

6.6 "grammar/Zamani-Grammar.md"

This remains the broad historical/extended design source.

A feature appearing there does not automatically become:

- stable;
- implemented;
- compatible;
- deprecated;
- removable.

Features must pass the normal promotion path:

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
IR integration
   ↓
tests
   ↓
compatibility classification
   ↓
stable

---

7. Compatibility Is Multidimensional

Zamani MUST NOT describe something simply as "compatible" without identifying the relevant compatibility dimension.

The principal dimensions are:

source
lexical
syntactic
parser
AST
name/module
type
effect
resource
capability
semantic
determinism
IR
artifact
ABI
runtime
target
dialect
tooling
documentation

A program can therefore be:

source-compatible
but target-infeasible

or:

source-compatible
but artifact-incompatible

or:

syntax-compatible
but semantic-incompatible

or:

source-compatible
but AST-schema-incompatible

These are distinct conditions.

---

8. Source Compatibility

Source compatibility asks:

«Can the same valid Zamani source program retain its specified meaning under the new language/compiler version?»

A source-compatible change MUST NOT silently change the meaning of existing valid source.

Examples of normally compatible changes include:

- adding a non-conflicting feature;
- adding a new capability;
- adding a new target backend;
- adding a new hardware realization;
- improving optimization;
- adding a new supported QPU;
- adding a new FPGA backend;
- adding a new CPU architecture;
- adding a new accelerator;
- improving diagnostics without changing semantics.

Examples that may be source-breaking include:

- changing operator meaning;
- changing type meaning;
- removing stable syntax;
- changing evaluation semantics;
- changing ownership semantics;
- changing stable module resolution;
- changing quantum measurement semantics.

Such changes require explicit compatibility classification.

---

9. Lexical Compatibility

Lexical compatibility concerns:

- identifiers;
- keywords;
- literals;
- comments;
- operators;
- delimiters;
- Unicode handling;
- numeric forms;
- quantum literals.

The canonical relationship is:

grammar/specification/lexical.md
        ↓
grammar/lexer/
        ↓
src/lexer.rs

Lexical changes MUST be evaluated for:

- token identity;
- token precedence;
- keyword collisions;
- operator ambiguity;
- source interpretation;
- diagnostics;
- backward compatibility.

Existing duplicate or overlapping lexical concepts must not silently diverge.

Examples requiring explicit conformance checking include previously identified pairs such as:

Question / QuestionMark
Ampersand / BitAnd

and specification/lexer discrepancies involving literal forms.

---

10. Syntax and Parser Compatibility

Syntax compatibility covers:

- grammar rules;
- precedence;
- associativity;
- ambiguity;
- optional constructs;
- declaration forms;
- expression forms;
- statement forms;
- domain extensions.

The canonical relationship remains:

specification
     ↓
Zamani.g4
     ↓
parser
     ↓
AST

A grammar change MUST be checked for:

- parse changes;
- ambiguity;
- precedence changes;
- accidental keyword capture;
- previously valid programs becoming invalid;
- previously invalid programs becoming valid with unintended semantics;
- source-span preservation.

Compatibility MUST NOT be inferred merely from successful parsing.

---

11. AST Compatibility

AST compatibility is distinct from source compatibility.

The frontend AST is structural and domain-neutral.

A source-compatible grammar evolution MAY legitimately change an internal AST representation if the semantic contract remains equivalent.

However, if AST schemas are exposed to:

- plugins;
- tools;
- serialized artifacts;
- external consumers;
- compiler stages;

then that AST boundary MUST be versioned explicitly.

The required relationship is:

source
 ↓
parser
 ↓
domain-neutral AST
 ↓
semantic analysis

The AST MUST NOT become coupled to:

- LLVM;
- MLIR;
- QIR;
- OpenQASM;
- a vendor QPU;
- a particular GPU;
- a particular FPGA;
- routing;
- scheduling;
- QEC;
- ZQN;
- HAL.

---

12. Semantic Compatibility

Semantic compatibility is the highest-priority source compatibility dimension.

A migration MUST preserve specified meaning for all applicable constructs, including:

- evaluation;
- binding;
- scope;
- types;
- ownership;
- effects;
- concurrency;
- resource requirements;
- capabilities;
- deterministic behavior;
- numerical semantics;
- quantum semantics;
- hardware intent;
- distributed semantics;
- security semantics.

A syntactic transformation is not a valid migration merely because it parses.

The transformed program must retain the specified semantics.

---

13. Type Compatibility

Type compatibility includes:

- primitive types;
- generic types;
- function types;
- references;
- ownership;
- borrowing;
- arrays;
- tensors;
- resources;
- capabilities;
- quantum types;
- hardware types;
- effect types.

A type change is breaking if a previously valid program can change meaning or lose a required type guarantee.

Resource dimensions remain semantic data rather than compiler maximums.

For example:

Qubit[n]
Tensor<T, shape>
Memory<T, size>

must not be converted into fixed compiler limits.

---

14. Effect Compatibility

Effect compatibility includes:

- effect declarations;
- effect composition;
- effect handlers;
- asynchronous effects;
- I/O effects;
- quantum effects;
- resource effects;
- distributed effects;
- security effects.

An effect may be internally lowered differently while preserving its specified semantics.

A backend optimization MUST NOT silently remove a language-level effect guarantee.

---

15. Resource Compatibility

Zamani distinguishes:

resource requirement
resource constraint
resource capability
resource preference
resource hint
implementation decision

These MUST remain separate.

For example:

requires qubits >= n

is not equivalent to:

use physical qubit 17

Similarly:

requires memory >= required_memory

is not equivalent to:

use memory bank 3

Compatibility MUST preserve the distinction.

---

16. Capability Compatibility

Capabilities describe what a target can provide.

Examples include:

capability("tensor.compute")
capability("gpu.compute")
capability("quantum.measurement")
capability("quantum.mid_circuit_measurement")

Capability names and meanings MUST be versioned as semantic contracts.

A target that lacks a required capability is target-infeasible, not necessarily source-incompatible.

The compiler/runtime SHOULD produce a structured diagnostic explaining:

1. what capability is required;
2. where it is required;
3. what target lacks it;
4. whether another target or lowering strategy can satisfy it.

---

17. Quantum Compatibility

Quantum compatibility MUST preserve quantum semantics rather than a particular hardware implementation.

The canonical quantum boundary remains:

quantum::ir

Compatibility MUST preserve, where applicable:

- qubit/state meaning;
- operation meaning;
- operation ordering;
- parameters;
- controls;
- adjoints;
- measurement;
- reset;
- observables;
- classical feed-forward;
- dynamic control;
- channels;
- noise intent;
- logical/physical distinction;
- resource requirements;
- error-correction requirements.

The grammar MUST NOT turn the currently known gate set into a permanent language limit.

A construct such as:

apply H
apply custom_gate
apply vendor.operation
apply operation(parameter)

must be representable through the generic quantum-operation model when permitted by the language specification.

A historical fixed gate enumeration MUST NOT become the compatibility ceiling for future quantum operations.

---

18. Quantum IR Compatibility

The canonical downstream boundary is:

quantum::ir

Compatibility work MUST NOT create a permanent parallel frontend quantum IR merely to preserve historical syntax.

Preferred migration:

legacy quantum syntax
        ↓
compatibility parser/normalization
        ↓
current AST
        ↓
semantic quantum operation
        ↓
quantum::ir

not:

legacy syntax
        ↓
legacy quantum IR
        ↓
new quantum IR

unless a separately documented external artifact contract explicitly requires such a representation.

---

19. QEC and ZQN Compatibility

Compatibility MUST preserve the separation between language semantics and downstream quantum implementation.

The pipeline remains:

Zamani source
      ↓
AST
      ↓
semantic analysis
      ↓
quantum::ir
      ↓
optimization
      ↓
QEC / resilience / ZQN
      ↓
routing
      ↓
scheduling
      ↓
HAL
      ↓
target

QEC owns error-correction implementation and policy.

ZQN owns quantum fault/noise semantics where defined by the repository architecture.

Neither becomes a second source-language grammar.

A change in QEC implementation does not automatically constitute a Zamani language-version change.

A change in specified quantum semantics does.

---

20. Classical Compatibility

Classical computation is a first-class Zamani domain.

Compatibility MUST preserve semantic behavior across:

- scalar computation;
- integers;
- floating-point values;
- vectors;
- matrices;
- tensors;
- symbolic computation;
- numerical methods;
- statistics;
- signal processing;
- optimization;
- scientific computation;
- control computation.

Adding a CPU/GPU/vector/accelerator backend is not automatically a language compatibility change.

---

21. HDL and Hardware Compatibility

HDL and hardware intent are part of the same language.

Compatibility MUST preserve the distinction between:

hardware intent

and:

specific hardware realization

A source program MUST NOT become incompatible merely because a target changes:

- FPGA family;
- ASIC generation;
- clock frequency;
- register width;
- memory size;
- number of resources;
- interconnect topology;
- accelerator availability.

A target may be unable to satisfy a program's requirements.

That is target feasibility, not necessarily source incompatibility.

---

22. No Artificial Hardware Limits

Compatibility MUST NEVER establish universal language limits such as:

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

These names may occur in validation/audit documentation as prohibited patterns.

They MUST NOT define language capacity.

Likewise, the compatibility system MUST NOT establish fixed limits for:

- timelines;
- processes;
- channels;
- agents;
- tensor dimensions;
- vector widths;
- network nodes;
- memory;
- storage;
- quantum registers;
- physical qubits.

Scalability is bounded by actual resources and implementation capacity, not by arbitrary grammar constants.

---

23. Target Compatibility Versus Source Compatibility

These must remain separate.

A program can be:

valid Zamani
+
semantically correct
+
source-compatible

while being:

not executable on a selected target

because that target lacks:

- memory;
- compute capability;
- quantum capability;
- required topology;
- required timing;
- required precision;
- required accelerator;
- required reliability;
- required communication capability.

This MUST produce a target/resource/capability diagnostic rather than falsely declaring the language construct incompatible.

---

24. Version Compatibility

Language versions are independent of:

- Rust versions;
- compiler implementation versions;
- runtime versions;
- operating-system versions;
- CPU generations;
- GPU generations;
- FPGA generations;
- ASIC generations;
- QPU generations.

The current Rust implementation baseline is:

Rust 1.97
Rust 1.97.1
Rust edition 2021
safe Rust only

The Zamani language version MUST NOT be derived from the Rust compiler version.

For example:

Zamani 1.x
Zamani compiler x.y.z
Rust 1.97.1

is a valid independent version relationship.

---

25. Safe Rust Compatibility

The Rust implementation of the compatibility system MUST use safe Rust.

Compatibility functionality includes:

- version parsing;
- version comparison;
- feature classification;
- migration metadata;
- deprecation metadata;
- compatibility matrix handling;
- diagnostics;
- conformance checking;
- repository validation.

These MUST NOT require Rust "unsafe".

The production implementation MUST NOT introduce "unsafe" merely for:

- parser compatibility;
- versioning;
- migration;
- compatibility checking;
- AST compatibility;
- semantic compatibility;
- IR compatibility;
- diagnostics.

---

26. Migration Architecture

The canonical migration direction is:

old source
    ↓
version-aware parsing
    ↓
compatibility normalization
    ↓
current AST
    ↓
semantic validation
    ↓
canonical semantic model
    ↓
canonical IR

The governing rule is:

«Migrate representations; preserve specified semantics.»

Migration must not use target limitations to redefine semantics.

Migration MUST validate:

- source meaning;
- names;
- types;
- effects;
- resources;
- capabilities;
- quantum operations;
- HDL intent;
- diagnostics where applicable;
- deterministic behavior;
- resulting canonical IR.

Detailed migration procedures belong in:

grammar/compatibility/migrations.md

---

27. Deprecation Architecture

Deprecation is separate from migration.

The lifecycle is conceptually:

stable
   ↓
deprecated
   ↓
migration-supported
   ↓
removal-eligible
   ↓
removed

A feature MUST NOT be deprecated merely because:

- a backend does not support it;
- a vendor does not support it;
- current hardware cannot execute it;
- compiler implementation is incomplete;
- a new optimization exists.

Deprecation requires an explicit language/design reason.

Detailed lifecycle policy belongs in:

grammar/compatibility/deprecated.md

---

28. Compatibility Matrix

The repository-wide matrix is owned by:

grammar/compatibility/compatibility-matrix.md

It must be capable of representing compatibility between:

language
lexer
parser
AST
semantic model
IR
compiler
artifact
ABI
runtime
target
dialect
tooling

For each relevant feature/version combination, the matrix should identify statuses such as:

compatible
conditionally-compatible
migration-required
deprecated-but-supported
unsupported
breaking
target-infeasible
experimental
not-applicable

The exact status vocabulary MUST remain aligned with the matrix specification.

---

29. Feature-Level Compatibility Contract

Every stable feature MUST have a traceable compatibility identity.

At minimum:

feature id
feature name
language version introduced
current status
grammar owner
lexer requirements
AST representation
semantic contract
IR mapping
compatibility classification
migration path
deprecation state
tests

Where feature manifests are introduced under:

grammar/specification/features/

their compatibility metadata SHOULD be linked rather than duplicated manually.

A feature manifest SHOULD be able to answer:

What version introduced this?
What changed?
What versions accept it?
What versions reject it?
Is migration required?
Is it deprecated?
What semantics must remain stable?
Which tests prove compatibility?

---

30. Domain Compatibility

The compatibility architecture applies uniformly to:

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
embedded
scientific
accelerator
future computing domains

A domain-specific feature MUST identify:

domain
syntax
AST
semantic meaning
resource meaning
capability meaning
IR mapping
target implications
compatibility status
migration path
tests

A domain MUST NOT silently establish a second language-version system.

---

31. Dialect Compatibility

Dialects are controlled extensions.

A dialect MUST declare:

name
version
owner
base Zamani version
syntax extensions
semantic extensions
AST mapping
IR mapping
capabilities
resource requirements
compatibility rules
migration rules
deprecation rules

A dialect MUST NOT silently alter the semantics of stable core Zamani.

A dialect MAY add constructs when permitted by the dialect system.

A dialect MUST NOT redefine a core construct under the same spelling with a different meaning unless the specification explicitly defines a versioned compatibility boundary.

---

32. Interoperability Compatibility

External formats include, where supported:

- OpenQASM;
- QIR;
- LLVM-related representations;
- MLIR-related representations;
- HDL formats;
- foreign-language interfaces;
- ABI formats;
- serialization formats.

These are interoperability boundaries.

They are not automatically Zamani's canonical semantic model.

The preferred architecture is:

external format
      ↓
interop frontend
      ↓
Zamani semantic model
      ↓
canonical IR

and:

Zamani semantic model
      ↓
canonical lowering
      ↓
external format

An interoperability format changing version does not automatically change the Zamani language version.

---

33. Artifact Compatibility

Compiled artifacts MUST identify enough metadata to determine whether they are usable under the intended compatibility contract.

Where applicable, artifact metadata SHOULD include:

Zamani language version
compiler version
IR version
target-independent semantic version
dialect versions
ABI contract
runtime contract
required capabilities
resource requirements
provenance
reproducibility information

Artifacts MUST NOT claim universal portability when their contract includes target-specific requirements.

---

34. ABI Compatibility

ABI compatibility is separate from language compatibility.

ABI compatibility applies only where a stable ABI is explicitly defined.

Changing:

- internal AST representation;
- compiler optimization;
- source grammar implementation;
- internal IR representation;

does not automatically constitute an ABI break.

Conversely, changing a public ABI can be ABI-breaking even when Zamani source syntax remains unchanged.

---

35. Runtime Compatibility

Runtime compatibility concerns the ability of a runtime to execute an artifact under its declared contract.

A runtime failure MUST NOT be misclassified as a source-language incompatibility when the source remains valid and the problem is:

- missing capability;
- insufficient resources;
- unsupported artifact version;
- unsupported runtime feature;
- target-specific limitation.

Diagnostics must identify the layer responsible.

---

36. Determinism

Compatibility MUST preserve determinism where the language specification promises deterministic behavior.

Version changes MUST be checked for:

- parser determinism;
- name-resolution determinism;
- semantic-analysis determinism;
- canonicalization determinism;
- IR-generation determinism;
- migration determinism;
- compatibility-check determinism.

If multiple valid implementation choices exist, their differences MUST NOT alter specified program meaning.

---

37. Source Spans and Diagnostics

Compatibility changes MUST preserve source locations wherever the language/tooling contract requires them.

Diagnostics for compatibility problems SHOULD identify:

source location
feature
effective language version
required version
actual supported version
compatibility dimension
reason
migration path

For example:

error: feature requires Zamani language version >= 1.2
       found: 1.0
       feature: quantum.mid_circuit_measurement
       migration: see compatibility migration guidance

Diagnostics MUST NOT claim that a construct is universally impossible merely because the selected target lacks the capability.

---

38. Compatibility Test Requirements

Every compatibility-sensitive change MUST have applicable tests.

At minimum:

positive
negative
boundary
migration
version
determinism
scalability
cross-layer

Domain-specific features additionally require relevant tests for:

classical
quantum
HDL
hardware
hybrid
distributed
AI
data
networking
security

Quantum compatibility tests SHOULD cover:

- qubit declarations;
- parameterized registers;
- generic operations;
- custom operations;
- measurement;
- reset;
- dynamic control;
- classical feed-forward;
- logical/physical distinction;
- resource requirements;
- capability requirements.

---

39. Compatibility Regression Testing

For each language version, the conformance suite SHOULD contain:

accepted-source corpus
rejected-source corpus
migration corpus
deprecated-feature corpus
AST conformance corpus
semantic conformance corpus
IR conformance corpus
diagnostic corpus
scalability corpus
determinism corpus

A compatibility regression MUST identify the first layer where behavior diverged.

Example:

source: PASS
lexer: PASS
parser: PASS
AST: PASS
semantic: FAIL

This is a semantic compatibility regression, not a grammar regression.

---

40. Scalability Compatibility

Compatibility MUST NOT impose artificial scaling ceilings.

The compatibility system MUST remain valid for:

tiny computation
embedded computation
single CPU
multicore
GPU
FPGA
ASIC
QPU
accelerator
HPC
cluster
distributed system
cloud
future architectures

No compatibility contract may require a fixed number of:

- CPUs;
- cores;
- threads;
- GPUs;
- FPGAs;
- nodes;
- devices;
- qubits;
- tensors;
- registers;
- memory units;
- timelines.

The upper bound is determined by:

program semantics
+
available resources
+
target capabilities
+
physical constraints
+
implementation capacity

not by grammar constants.

---

41. Compatibility of Resource Scaling

A program may express:

requires qubits >= n
requires memory >= required_memory
requires capability("tensor.compute")
requires capability("gpu.compute")
requires topology(...)

These statements describe intent.

Compatibility MUST preserve that intent.

The compiler MAY realize the program differently on different targets.

For example:

logical resources
        ↓
available resources
        ↓
target mapping
        ↓
physical realization

A source program MUST NOT need to be rewritten merely because the target has a different resource quantity, unless the program itself has a semantic requirement that the target cannot satisfy.

---

42. Compatibility of Hardware Mapping

Target-specific decisions belong downstream.

Examples:

physical qubit 17
GPU device 3
memory bank 2
FPGA region 4
node 14

must not become universal source-level assumptions.

When explicitly required by a target-specific deployment contract, such decisions must be clearly classified as target-specific.

Such a program is then portable only within the declared compatibility domain.

The compatibility system MUST make that distinction explicit.

---

43. Compatibility of Optimization

Compiler optimization is semantics-preserving.

Changing:

optimization strategy A

to:

optimization strategy B

does not constitute a language compatibility break if both preserve the specified semantics.

Optimization MUST NOT:

- alter observable semantics;
- alter required effects;
- remove required capabilities;
- violate resource requirements;
- alter quantum measurement semantics;
- alter HDL behavior;
- violate security guarantees.

---

44. Compatibility of Routing and Scheduling

Routing and scheduling are downstream implementation concerns.

A target can change:

routing strategy
scheduling strategy
topology mapping
resource allocation

without changing the Zamani source language.

Such changes become compatibility-relevant only when they violate a specified semantic or execution guarantee.

---

45. Compatibility of Resilience, QEC and ZQN

Changes to:

- resilience;
- QEC implementation;
- noise modeling;
- ZQN implementation;
- calibration;
- device recovery;

are not automatically language-version changes.

They become language compatibility concerns only when a language-level contract changes.

The source language remains above these implementation layers.

---

46. Repository-Wide Change Propagation

A compatibility-sensitive language change must be traced through:

specification
    ↓
spec contracts
    ↓
lexer
    ↓
Zamani.g4
    ↓
parser
    ↓
AST
    ↓
semantic analysis
    ↓
canonical IR
    ↓
compiler
    ↓
tests
    ↓
compatibility metadata
    ↓
documentation

For quantum features:

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
QEC/resilience/ZQN
    ↓
HAL

For HDL/hardware:

HDL syntax
    ↓
AST
    ↓
hardware semantics
    ↓
hardware/HDL IR
    ↓
optimization
    ↓
synthesis/lowering
    ↓
target realization

No layer may be silently skipped.

---

47. Independent File Completion Contract

A compatibility-related file is considered complete only when its ownership can be understood without requiring a later file to redefine it.

Each compatibility document SHOULD explicitly state:

Purpose
Status
Owns
Does not own
Dependencies
Upstream contracts
Downstream consumers
Compatibility dimensions
Version interaction
Migration interaction
Deprecation interaction
Grammar integration
AST integration
Semantic integration
IR integration
Compiler integration
Runtime integration
Testing requirements
Diagnostics
Scalability
Determinism
Security
Completion criteria

This prevents the situation where one file must be repeatedly rewritten after another file changes.

---

48. Completion Contract for "compatibility/README.md"

This README itself is complete when it establishes:

- compatibility authority;
- file ownership;
- relationship to "specification/";
- relationship to "spec/";
- relationship to "Zamani.g4";
- relationship to lexer/parser/AST;
- relationship to semantic analysis;
- relationship to canonical IR;
- relationship to "quantum::ir";
- relationship to QEC/ZQN;
- relationship to hardware/HAL;
- relationship to versions;
- relationship to migrations;
- relationship to deprecations;
- relationship to the compatibility matrix;
- relationship to tests;
- POCO-REAF requirements;
- scalability requirements;
- safe-Rust requirements.

It MUST NOT need to be edited merely because a particular feature is added.

Feature-specific compatibility belongs in feature contracts and the compatibility matrix.

---

49. Adding a New Language Feature

A new feature MUST follow this sequence:

1. Define semantic intent
        ↓
2. Define specification contract
        ↓
3. Define lexical requirements
        ↓
4. Define grammar
        ↓
5. Define AST mapping
        ↓
6. Define semantic mapping
        ↓
7. Define IR mapping
        ↓
8. Define compiler consumers
        ↓
9. Define runtime/target implications
        ↓
10. Define compatibility status
        ↓
11. Define migration requirements
        ↓
12. Define deprecation/removal implications
        ↓
13. Add conformance tests
        ↓
14. Validate scalability
        ↓
15. Validate determinism
        ↓
16. Update compatibility matrix
        ↓
17. Update implementation-conformance documentation

No feature becomes stable merely because its grammar parses.

---

50. Changing an Existing Feature

Before changing a stable feature:

1. identify its feature contract;
2. identify its current language version;
3. identify affected compatibility dimensions;
4. identify downstream consumers;
5. determine whether the change is compatible;
6. determine whether migration is required;
7. determine whether deprecation is required;
8. update tests;
9. update compatibility metadata;
10. update implementation conformance;
11. validate the complete pipeline.

The change MUST NOT be implemented solely in the grammar.

---

51. Removing a Feature

A feature may be removed only after:

stable
 ↓
deprecated
 ↓
migration available where practical
 ↓
documented removal target
 ↓
compatibility matrix updated
 ↓
tests updated
 ↓
implementation removal
 ↓
conformance verification

Removal MUST NOT occur merely because a current backend cannot implement the feature.

---

52. Historical Syntax

Historical syntax may remain accepted by a compatibility parser where required.

Historical syntax MUST:

- have a defined status;
- identify the version in which it was valid;
- have migration guidance where applicable;
- normalize to current semantics;
- avoid creating a permanent duplicate semantic architecture.

Historical syntax MUST NOT silently become current syntax.

---

53. Legacy Quantum Syntax

Legacy quantum constructs must be normalized into the generic quantum semantic model where possible.

For example, a historical fixed-gate representation may migrate into:

operation name
namespace
operands
parameters
results
attributes
modifiers
effects
capabilities
source span

and then:

semantic quantum operation
        ↓
quantum::ir

This permits future operations without making compatibility dependent on a fixed list of gates.

---

54. Legacy Hardware Syntax

Historical target-specific forms should be classified as:

- portable;
- target-specific;
- deprecated;
- migration-required;
- unsupported.

The compatibility system must not silently reinterpret a target-specific declaration as a portable requirement.

For example:

use GPU 0

must not silently acquire the meaning:

requires gpu.compute

unless the migration specification explicitly defines that transformation and proves that the semantics are equivalent.

---

55. Compatibility and Resource Availability

Resource availability is a runtime/target property.

Compatibility MUST distinguish:

language accepts program

from:

selected target can satisfy program

and:

runtime currently has resources available

Therefore:

valid source
+
valid semantics
+
insufficient target resources

is not automatically a language incompatibility.

It is a resource-feasibility failure.

---

56. Compatibility and Infinite Scalability

Zamani's scalability objective is conceptual rather than a promise that physical machines have infinite resources.

The language MUST NOT encode artificial finite ceilings.

The correct model is:

program size
      ↓
required resources
      ↓
available resources
      ↓
target capabilities
      ↓
implementation feasibility

A sufficiently large program may fail because no available target can satisfy its requirements.

That does not make the language itself bounded by an arbitrary grammar constant.

---

57. Compatibility Security Requirements

Compatibility machinery MUST reject or safely handle:

- malformed versions;
- ambiguous version declarations;
- invalid migration metadata;
- unsupported dialect versions;
- inconsistent compatibility metadata;
- malformed artifact metadata;
- contradictory feature states;
- invalid migration transformations.

Compatibility metadata MUST NOT provide a mechanism to bypass:

- type checking;
- effect checking;
- capability checking;
- resource checking;
- security validation;
- semantic validation.

Migration MUST NOT weaken security guarantees silently.

---

58. Compatibility and Macros

Macros are source transformations.

Compatibility MUST distinguish:

macro source

from:

expanded semantic program

A macro migration must preserve:

- hygiene;
- bindings;
- source spans where required;
- semantic meaning;
- diagnostics;
- capabilities;
- effects.

A macro must not bypass language-version compatibility checks.

---

59. Compatibility and Metaprogramming

Compile-time and reflective facilities MUST respect the active language-version contract.

Metaprogramming MUST NOT create an uncontrolled mechanism for generating syntax that bypasses:

- parser rules;
- semantic validation;
- capability checks;
- compatibility checks.

Generated source must be checked under the correct effective language version.

---

60. Compatibility and Interoperability

Foreign-language interoperability must identify the external contract separately from the Zamani language contract.

For example:

Zamani language version
+
C ABI version
+
QIR version
+
OpenQASM version

are independent dimensions.

An external format upgrade does not automatically imply a Zamani language-version upgrade.

---

61. Compatibility and Generated Files

Generated compatibility documentation MUST be deterministic.

Generated files MUST identify their source authority where applicable.

A generated file MUST NOT become a second authority merely because it is easier to read.

For example:

grammar.md

may be generated from implementation/specification information, but the normative language meaning remains owned by the specification.

---

62. Compatibility Validation

The existing validation subsystem MUST participate in compatibility validation.

Relevant repository contracts include:

grammar/validation/compatibility-rules.md
grammar/validation/ambiguity-rules.md
grammar/validation/hardcoding-audit.md
grammar/validation/scalability-rules.md
grammar/validation/semantic-boundaries.md

Compatibility validation MUST check at least:

version consistency
feature status consistency
grammar/spec consistency
lexer/parser consistency
AST coverage
semantic coverage
IR coverage
migration coverage
deprecation coverage
test coverage
hard-coding violations
scalability violations
determinism

---

63. Hard-Coding Audit

Compatibility files MUST NOT introduce artificial limits.

The validation system should detect prohibited universal capacity assumptions such as:

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

A numeric literal is not automatically prohibited.

The distinction is:

program value

versus:

language capacity limit

For example:

let n = 1024;

may be valid program semantics.

A grammar rule stating:

qubits must never exceed 1024

is an artificial language limit and is prohibited unless it is an explicitly specified semantic property of a particular target-specific construct.

---

64. Compatibility Matrix Update Rule

Whenever a compatibility-relevant feature changes, the compatibility matrix MUST be updated in the same change set whenever practical.

The matrix entry should identify:

feature
old version
new version
source status
lexical status
syntax status
AST status
semantic status
IR status
artifact status
runtime status
target status
migration status
deprecation status
tests

A change must not be considered production-complete while its compatibility status is unknown.

---

65. Version Resolution

Every compilable source unit MUST have a deterministic effective language version.

The source of that version is governed by the language-version specification.

Potential sources include:

1. explicit source declaration;
2. package/project manifest;
3. workspace declaration;
4. documented legacy default.

The exact precedence MUST be defined by the authoritative version specification.

Once resolved, the effective version MUST be available to:

lexer
parser
AST construction
semantic analysis
compatibility checks
diagnostics
artifact metadata
tooling

An unsupported version MUST produce a structured diagnostic.

It MUST NOT silently fall back to an unrelated version.

---

66. Compiler Support for Multiple Language Versions

A compiler MAY support multiple Zamani language versions.

Conceptually:

Zamani Compiler
 ├── Zamani 1.x
 ├── Zamani 2.x
 └── future versions

Each supported version MUST have explicit conformance status.

A compiler MUST NOT claim compatibility merely because it accepts the source syntax.

Semantic conformance is required.

---

67. Compiler Version Independence

Compiler implementation versions are independent from language versions.

For example:

Zamani language: 1.2.0
compiler: 0.8.x
Rust: 1.97.1

is a legitimate relationship.

The compatibility system MUST NOT confuse:

compiler upgrade

with:

language breaking change

---

68. Rust Baseline

The repository's production Rust implementation baseline is:

Rust 1.97 / Rust 1.97.1
Rust edition 2021
safe Rust

Compatibility-related implementation MUST remain valid under the declared supported Rust baseline.

The compatibility README does not own the repository's "Cargo.toml"; it merely states the integration requirement.

The actual Rust build configuration remains authoritative for the concrete compiler dependency declaration.

---

69. Completion Criteria for Production Compatibility

The compatibility architecture is production-ready only when all of the following are true:

[ ] authority hierarchy is unambiguous
[ ] language-version ownership is unambiguous
[ ] compatibility dimensions are defined
[ ] migration ownership is unambiguous
[ ] deprecation ownership is unambiguous
[ ] compatibility matrix is authoritative for relationships
[ ] source compatibility is tested
[ ] lexical compatibility is tested
[ ] syntax compatibility is tested
[ ] AST compatibility is tested where exposed
[ ] semantic compatibility is tested
[ ] type compatibility is tested
[ ] effect compatibility is tested
[ ] resource compatibility is tested
[ ] capability compatibility is tested
[ ] quantum compatibility is tested
[ ] classical compatibility is tested
[ ] HDL compatibility is tested
[ ] hardware intent compatibility is tested
[ ] distributed compatibility is tested
[ ] AI/data compatibility is tested
[ ] interoperability compatibility is tested
[ ] dialect compatibility is tested
[ ] artifact compatibility is tested where applicable
[ ] ABI compatibility is tested where promised
[ ] runtime compatibility is tested where promised
[ ] target feasibility is distinguished from source compatibility
[ ] migration paths exist where required
[ ] deprecated features have explicit lifecycle state
[ ] deterministic behavior is validated
[ ] scalability is validated
[ ] artificial hardware limits are audited
[ ] Rust unsafe is absent from the production implementation
[ ] documentation reflects actual implementation status

---

70. Definition of "Compatible"

A Zamani release or implementation MUST NOT use the unqualified term:

«compatible»

without specifying the scope.

Preferred forms are:

source-compatible
lexically-compatible
syntactically-compatible
AST-compatible
semantically-compatible
IR-compatible
artifact-compatible
ABI-compatible
runtime-compatible
target-compatible
dialect-compatible
migration-compatible

If several dimensions are guaranteed, they SHOULD be listed explicitly.

---

71. Definition of "Breaking"

A change is breaking when it intentionally invalidates a previously guaranteed contract.

A break may occur at:

source
lexical
syntax
AST
type
effect
resource
capability
semantic
IR
artifact
ABI
runtime
dialect

A target that cannot satisfy a program requirement is not automatically a language break.

A backend that cannot implement a newly specified capability is not automatically a language break.

A compiler optimization change is not automatically a language break.

---

72. Definition of "Migration Required"

Migration is required when an old representation cannot remain accepted indefinitely without:

- ambiguity;
- semantic contradiction;
- security problems;
- unresolvable parser conflict;
- incompatible language semantics;
- removal of a deliberately deprecated contract.

Migration should be avoided merely to accommodate:

- temporary implementation limitations;
- a single vendor;
- a single machine;
- a single accelerator;
- current hardware capacity.

---

73. Definition of "Target-Infeasible"

A program is target-infeasible when:

program semantics are valid
+
compatibility requirements are satisfied
+
selected target cannot satisfy required resources/capabilities/constraints

Examples:

requires qubits >= n

when the selected target lacks sufficient usable quantum resources.

Or:

requires capability("gpu.compute")

when the selected target provides no such capability.

The source program does not thereby become invalid Zamani.

---

74. Integration With Existing Quantum Architecture

The compatibility system MUST preserve:

generic quantum syntax
        ↓
domain-neutral AST
        ↓
semantic quantum operation
        ↓
quantum::ir

It MUST NOT preserve legacy quantum syntax by creating a second permanent semantic pipeline.

Quantum compatibility MUST remain independent of:

- physical qubit numbering;
- vendor gate inventories;
- topology;
- calibration;
- routing;
- QEC implementation;
- scheduler implementation.

---

75. Integration With Existing HDL Architecture

HDL compatibility follows:

HDL syntax
      ↓
AST
      ↓
hardware semantics
      ↓
canonical hardware/HDL representation
      ↓
synthesis/lowering
      ↓
target realization

A compatibility migration MUST preserve hardware behavior and intent.

It MUST NOT transform portable hardware intent into an accidental fixed device model.

---

76. Integration With Resources and Hardware

Existing contracts under:

grammar/resources/
grammar/hardware/

must remain downstream of language semantics.

Compatibility metadata should distinguish:

resource requirement
capability
constraint
preference
hint
deployment choice

This distinction is essential for POCO-REAF.

---

77. Integration With Validation

The compatibility directory integrates with:

grammar/validation/

Validation MUST verify that:

compatibility metadata
        ↔
specification
        ↔
grammar
        ↔
lexer
        ↔
parser
        ↔
AST
        ↔
semantic implementation
        ↔
IR
        ↔
tests

remain consistent.

---

78. Integration With Tests

Compatibility tests belong under the existing test architecture.

They SHOULD include:

tests/compatibility/
tests/negative/
tests/boundary/
tests/scalability/
tests/determinism/

where those directories exist or are established by the repository test architecture.

Existing "grammar/tests/mod.rs" MUST NOT become a second compatibility specification.

It is a test integration point.

---

79. Compatibility Review Checklist

Before merging a compatibility-affecting change:

[ ] Is the owning specification identified?
[ ] Is the compatibility dimension identified?
[ ] Is the language version identified?
[ ] Is the change source-compatible?
[ ] Is the lexer affected?
[ ] Is the parser affected?
[ ] Is the AST affected?
[ ] Is semantic analysis affected?
[ ] Is the canonical IR affected?
[ ] Is quantum::ir affected?
[ ] Are resources affected?
[ ] Are capabilities affected?
[ ] Are HDL/hardware semantics affected?
[ ] Are artifacts affected?
[ ] Is ABI affected?
[ ] Is runtime behavior affected?
[ ] Is target feasibility affected?
[ ] Is migration required?
[ ] Is deprecation required?
[ ] Is the compatibility matrix updated?
[ ] Are positive tests updated?
[ ] Are negative tests updated?
[ ] Are boundary tests updated?
[ ] Are scalability tests updated?
[ ] Are deterministic tests updated?
[ ] Has hard-coding been audited?
[ ] Does the implementation remain safe Rust?

---

80. Final Architectural Rule

The compatibility architecture exists to preserve this separation:

LANGUAGE MEANING
       │
       ▼
PORTABLE PROGRAM INTENT
       │
       ▼
SEMANTIC MODEL
       │
       ▼
CANONICAL IR
       │
       ├──────────────┬───────────────┐
       ▼              ▼               ▼
   classical      quantum::ir     HDL/hardware
       │              │               │
       └──────────────┼───────────────┘
                      ▼
             target-independent
                 optimization
                      │
          ┌───────────┼───────────┐
          ▼           ▼           ▼
       routing    scheduling   resilience
                                  │
                              QEC / ZQN
                                  │
                                  ▼
                                 HAL
                                  │
                                  ▼
                           target realization

Compatibility belongs across this entire chain, but each layer retains its own ownership.

The grammar MUST NOT become the hardware model.

The hardware model MUST NOT become the language specification.

The runtime MUST NOT redefine language semantics.

QEC MUST NOT become the quantum grammar.

ZQN MUST NOT become a second quantum IR.

The AST MUST remain domain-neutral.

"quantum::ir" remains the canonical quantum semantic boundary.

Resources and capabilities describe what is required or available.

Routing, scheduling, resilience, QEC, ZQN, HAL, and backends determine how valid intent is realized.

---

81. POCO-REAF Compatibility Guarantee

The ultimate compatibility contract is therefore:

«A valid Zamani program expresses computation and intent independently of the particular machine on which it will eventually execute.»

The program may scale from:

atom
embedded
single processor
multicore
GPU
FPGA
ASIC
QPU
accelerator
HPC
cluster
distributed system
cloud
future architecture

without requiring the language to encode artificial universal limits.

The compiler and runtime may adapt realization to available resources and capabilities.

The source semantics remain the stable contract.

Therefore:

Program
   ↓
One language contract
   ↓
One semantic meaning
   ↓
One canonical compilation model
   ↓
Many target realizations

is the compatibility foundation for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

---

82. File Ownership Summary

This README owns:

compatibility architecture
compatibility navigation
repository integration boundaries
compatibility terminology
compatibility workflow
cross-file ownership relationships
POCO-REAF compatibility principles

"versions.md" owns:

version policy
version numbering
release compatibility

"migrations.md" owns:

migration procedures
migration transformations
migration validation

"deprecated.md" owns:

deprecation lifecycle
deprecation status
removal policy

"compatibility-matrix.md" owns:

explicit cross-layer compatibility relationships

"spec/compatibility.md" owns:

formal compatibility dimensions

"spec/versioning.md" owns:

cross-layer version propagation
implementation conformance

"specification/language-version.md" owns:

language-level version semantics

"Zamani.g4" owns:

canonical grammar composition

"grammar.md" owns:

implementation-conformance reporting

"Zamani-Grammar.md" owns:

historical/extended/proposed design material

This separation is intentional and MUST be preserved.

---

83. Production Definition

"grammar/compatibility/" is production-ready when:

every compatibility decision
        ↓
has one identifiable owner
        ↓
has one version relationship
        ↓
has explicit affected layers
        ↓
has diagnostics
        ↓
has migration/deprecation treatment where necessary
        ↓
has conformance tests
        ↓
has scalability validation
        ↓
has deterministic behavior
        ↓
does not introduce artificial resource limits
        ↓
does not require unsafe Rust

No compatibility document may silently become a second language specification.

No implementation detail may silently become a language limit.

No target limitation may silently become a source-language incompatibility.

No historical syntax may silently become stable syntax.

No stable semantic meaning may silently change.

That is the required compatibility foundation for a production-ready, scalable Zamani grammar and compiler ecosystem.