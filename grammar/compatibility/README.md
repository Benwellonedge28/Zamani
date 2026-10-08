Zamani Compatibility Architecture

Path: "grammar/compatibility/README.md"
Repository: "Benwellonedge28/Zamani"
Scope: "grammar/compatibility/"
Status: Normative compatibility architecture, ownership, navigation, and integration contract
Grammar technology: ANTLR4
Rust implementation: Rust 1.97 or later, Rust 2021 edition
Rust safety: Production Rust implementation MUST use safe Rust; production implementation MUST NOT require or use Rust "unsafe"
Primary portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)

---

1. Purpose

This document defines the architecture, ownership boundaries, integration contracts, and production requirements for:

grammar/compatibility/

It explains how compatibility is maintained across the complete Zamani language and compiler architecture without turning compatibility into a second language specification.

Compatibility covers relationships among:

- language versions;
- source syntax;
- lexical representation;
- parser behavior;
- AST schemas;
- names and modules;
- types;
- effects;
- resources;
- capabilities;
- contracts;
- policies;
- provenance;
- semantic meaning;
- classical computation;
- quantum computation;
- HDL and hardware intent;
- hybrid computation;
- AI and data computation;
- concurrency;
- distributed computation;
- networking;
- security;
- metaprogramming;
- dialects;
- canonical IR;
- quantum IR;
- artifacts;
- ABI;
- runtime;
- targets;
- tooling;
- migration;
- deprecation;
- diagnostics;
- reproducibility;
- conformance testing.

This README is intentionally not a second language specification.

It MUST NOT define the complete syntax or semantic meaning of Zamani constructs.

It MUST NOT become an alternative versioning specification.

It MUST NOT become an alternative migration specification.

It MUST NOT become an alternative deprecation registry.

It MUST NOT become an alternative IR specification.

It MUST NOT become an alternative target specification.

Instead, this document establishes:

1. who owns each compatibility concern;
2. how compatibility information flows between repository layers;
3. what must be checked when a feature changes;
4. how compatibility interacts with POCO-REAF;
5. how scalability is protected;
6. how compatibility is integrated with the Rust implementation;
7. how compatibility is validated and tested.

---

2. Fundamental Compatibility Principle

The fundamental rule is:

«A compatible Zamani implementation MUST NOT silently change the specified meaning of valid existing Zamani source code within a compatibility contract that it claims to support.»

A compatibility-affecting change MUST therefore be:

1. identified;
2. classified;
3. assigned to its owning specification;
4. associated with the applicable language/version contract;
5. reflected in compatibility metadata;
6. diagnosed when necessary;
7. tested;
8. migrated when migration is required;
9. deprecated before removal when the compatibility policy requires deprecation.

Implementation convenience MUST NOT silently become a language-breaking change.

A limitation of one compiler backend MUST NOT silently become a limitation of the language.

A limitation of one target MUST NOT silently become a limitation of portable source code.

A current hardware limitation MUST NOT silently become a universal grammar limit.

A generated document MUST NOT become authoritative merely because it is generated or easier to consume.

---

3. Compatibility Is a Contract, Not a Capability Claim

The word "compatible" is too broad to be meaningful by itself.

Compatibility MUST always identify its dimension.

Examples:

source-compatible
lexically-compatible
syntactically-compatible
AST-compatible
type-compatible
effect-compatible
resource-compatible
capability-compatible
semantically-compatible
IR-compatible
artifact-compatible
ABI-compatible
runtime-compatible
target-compatible
dialect-compatible
tool-compatible
migration-compatible

An implementation MAY be compatible in one dimension while incompatible in another.

For example:

source-compatible
+
target-infeasible

is a valid state.

Likewise:

syntax-compatible
+
semantic-incompatible

is possible and MUST NOT be reported simply as "compatible".

---

4. Compatibility Directory Ownership

The compatibility directory contains separate files with separate responsibilities.

File| Owns| Does not own
"README.md"| Architecture, ownership, navigation, integration rules| Individual compatibility decisions
"versions.md"| Language/release version policy| Migration algorithms
"migrations.md"| Migration procedures and transformations| Version numbering policy
"deprecated.md"| Deprecation lifecycle and removal policy| Migration implementation
"compatibility-matrix.md"| Cross-layer compatibility relationships| Language grammar
"reserved.md" when required| Compatibility-sensitive reserved identifiers/syntax| General version policy

A file SHOULD NOT be added merely because another file is becoming large.

A new file is justified when it provides a distinct, stable ownership boundary.

---

5. Repository Authority Model

The compatibility subsystem participates in the following repository-wide authority hierarchy:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ├── normative language meaning
        ├── language-version semantics
        └── domain semantics
        │
        ▼
grammar/spec/
        │
        ├── focused formal contracts
        ├── validation contracts
        └── compatibility/versioning contracts
        │
        ▼
grammar/compatibility/
        │
        ├── compatibility architecture
        ├── version relationships
        ├── migration relationships
        ├── deprecation relationships
        └── compatibility matrix
        │
        ▼
canonical grammar / lexer / parser
        │
        ▼
domain-neutral AST
        │
        ▼
structural + semantic validation
        │
        ▼
canonical semantic model
        │
        ├───────────────┬────────────────┐
        ▼               ▼                ▼
    classical       quantum::ir       HDL/hardware
        │               │                │
        └───────────────┼────────────────┘
                        ▼
                 optimization
                        │
                    lowering
                        │
             routing / scheduling
                        │
              resilience / recovery
                        │
                    QEC / ZQN
                        │
                       HAL
                        │
                target realization

Compatibility describes relationships across these layers.

It does not take ownership of their internal semantics.

---

6. Relationship to "grammar/DESIGN.md"

"grammar/DESIGN.md" is the higher-level architecture authority.

Compatibility MUST conform to its requirements concerning:

- one coherent Zamani language;
- domain-neutral AST;
- canonical grammar composition;
- separation of syntax and semantics;
- resource abstraction;
- capability abstraction;
- effects;
- contracts;
- policies;
- provenance;
- classical computation;
- quantum computation;
- HDL;
- hardware;
- distributed execution;
- AI/data computation;
- canonical IR boundaries;
- "quantum::ir";
- optimization;
- lowering;
- routing;
- scheduling;
- resilience;
- QEC;
- ZQN;
- HAL;
- POCO-REAF;
- scalability.

This README MUST NOT duplicate those architectural definitions unnecessarily.

When an architectural question belongs to "DESIGN.md", this file references that authority rather than redefining it.

---

7. Relationship to "grammar/specification/"

"grammar/specification/" owns normative human-readable language meaning.

Compatibility metadata cannot redefine language semantics.

For example, if:

grammar/specification/

defines the meaning of a quantum measurement operation, then:

grammar/compatibility/

may describe whether that operation is preserved between language versions.

It MUST NOT redefine what measurement means.

The same rule applies to:

- types;
- ownership;
- effects;
- contracts;
- policies;
- AI reasoning;
- learning;
- adaptation;
- uncertainty;
- classical computation;
- quantum computation;
- HDL;
- distributed semantics;
- networking;
- security;
- metaprogramming.

---

8. Relationship to "grammar/spec/"

"grammar/spec/" contains focused formal contracts.

Compatibility MUST consume and remain consistent with contracts such as:

grammar/spec/compatibility.md
grammar/spec/versioning.md

Where a contract exists there, this README identifies its integration rather than creating another competing definition.

The following relationship is mandatory:

specification/
      │
      ▼
spec/
      │
      ▼
compatibility/
      │
      ▼
implementation + tests

Contradictions MUST be resolved in favor of the repository's defined authority hierarchy.

---

9. Relationship to "grammar/Zamani.g4"

"grammar/Zamani.g4" is the canonical ANTLR grammar composition root.

Compatibility MUST NOT create another canonical root grammar.

The compatibility directory MUST NOT contain a competing:

Zamani.g4

or another file claiming to be the universal grammar root.

Historical syntax may be supported through explicitly defined compatibility mechanisms when required, but it must eventually normalize into the current canonical AST and semantic pipeline.

Preferred architecture:

legacy source
     │
     ▼
version-aware lexical/parser compatibility
     │
     ▼
current domain-neutral AST
     │
     ▼
current semantic model
     │
     ▼
canonical IR

Not:

legacy source
     │
     ▼
permanent legacy AST
     │
     ▼
permanent legacy IR
     │
     ▼
second compiler pipeline

unless an independently specified external artifact contract explicitly requires such a representation.

---

10. Relationship to the Rust Lexer and Parser

The repository currently contains Rust frontend implementation in addition to the ANTLR grammar.

Therefore:

ANTLR grammar

and:

Rust lexer/parser

MUST NOT silently accept different languages.

The compatibility architecture requires conformance between:

grammar/specification/lexical.md
        ↓
grammar/antlr/ZamaniLexer.g4
        ↓
src/lexer.rs

and:

grammar/specification/syntax.md
        ↓
grammar/Zamani.g4
        ↓
src/parser.rs

ANTLR acceptance alone does not prove Rust frontend compatibility.

Rust parser acceptance alone does not prove ANTLR compatibility.

Both implementations must be checked against shared conformance cases.

---

11. Relationship to the Domain-Neutral AST

The AST is the boundary between source syntax and semantic analysis.

Compatibility MUST preserve source meaning, not force historical AST layouts to remain forever.

An internal AST representation may change when semantic equivalence is preserved.

However, if the AST is exposed to:

- plugins;
- external tools;
- serialized artifacts;
- compiler extensions;
- language servers;
- analysis tools;
- external consumers;

then that exposed AST contract MUST have explicit versioning.

The AST MUST remain independent of:

- a particular CPU;
- a particular GPU;
- a particular FPGA;
- a particular ASIC;
- a particular accelerator;
- a particular QPU;
- physical qubit numbering;
- routing decisions;
- scheduler state;
- QEC implementation;
- ZQN implementation;
- HAL objects;
- vendor instruction encodings.

---

12. Canonical Semantic Compatibility

Compatibility MUST ultimately preserve semantic meaning.

The relevant pipeline is:

source
  ↓
tokens
  ↓
syntax
  ↓
AST
  ↓
structural validation
  ↓
types
  ↓
effects
  ↓
resources
  ↓
capabilities
  ↓
contracts
  ↓
policies
  ↓
provenance
  ↓
semantic model

A source transformation is not a valid migration merely because the resulting program parses.

A migration is valid only when the resulting program satisfies the applicable semantic equivalence or explicitly documented semantic-change contract.

---

13. Compatibility Dimensions

Every compatibility-affecting change MUST be evaluated against applicable dimensions.

13.1 Source

Whether existing source remains valid with its specified meaning.

13.2 Lexical

Whether tokenization remains compatible.

Includes:

- identifiers;
- keywords;
- operators;
- delimiters;
- literals;
- Unicode;
- comments;
- numeric syntax;
- quantum literal syntax.

13.3 Syntactic

Whether grammar structure, precedence, associativity, and parsing remain compatible.

13.4 AST

Whether exposed AST contracts remain compatible.

13.5 Name and Module

Whether:

- names;
- paths;
- imports;
- exports;
- modules;
- namespaces

retain their specified behavior.

13.6 Type

Whether type meaning, inference, generic constraints, ownership, references, and related guarantees remain compatible.

13.7 Effect

Whether declared and inferred effects retain their specified meaning.

This includes effects such as:

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

13.8 Resource

Whether requirements, constraints, budgets, preferences, and hints retain their meaning.

13.9 Capability

Whether capability names and semantics remain valid.

13.10 Contract

Whether:

requires
ensures
invariant
assume
guarantee
property

retain their specified semantics.

13.11 Policy

Whether permissions, prohibitions, requirements, constraints, preferences, and fallback behavior remain compatible.

13.12 Provenance

Whether required provenance and audit information remains valid.

13.13 Semantic

Whether observable program meaning remains unchanged.

13.14 Determinism

Whether deterministic programs retain their deterministic guarantees.

13.15 IR

Whether an explicitly promised IR contract remains compatible.

13.16 Artifact

Whether serialized/compiler artifacts retain their promised compatibility.

13.17 ABI

Whether a promised foreign-function or binary interface remains compatible.

13.18 Runtime

Whether a promised runtime contract remains compatible.

13.19 Target

Whether a target can realize the program.

Target feasibility is deliberately separate from source compatibility.

13.20 Dialect

Whether an external dialect contract remains compatible.

---

14. Source Compatibility

A source-compatible change MUST preserve the specified meaning of previously supported source.

Usually compatible additions include:

- new independent capabilities;
- new target backends;
- additional optimization strategies;
- additional quantum hardware;
- additional FPGA targets;
- additional CPU architectures;
- additional accelerators;
- improved diagnostics;
- additional implementation strategies;
- additional dialects that do not conflict with existing syntax.

Potentially breaking changes include:

- changing operator meaning;
- changing type meaning;
- changing evaluation order;
- changing ownership semantics;
- changing module resolution;
- removing stable syntax;
- changing quantum measurement semantics;
- changing observable concurrency semantics;
- changing security guarantees.

Such changes require explicit classification.

---

15. Lexical Compatibility

Lexical compatibility MUST be evaluated whenever the token vocabulary changes.

The lexical authority chain is:

grammar/specification/lexical.md
        ↓
grammar/lexer/
        ↓
grammar/antlr/ZamaniLexer.g4
        ↓
src/lexer.rs

Changes MUST be checked for:

- token identity;
- token precedence;
- keyword/identifier collisions;
- operator ambiguity;
- literal ambiguity;
- source spans;
- Unicode handling;
- diagnostics;
- compatibility behavior.

Previously identified duplicate lexical concepts must remain explicitly reconciled.

For example:

Question / QuestionMark
Ampersand / BitAnd

must not represent accidental duplicate concepts.

If two tokens are semantically distinct, that distinction must be documented.

If they are not distinct, the repository should converge on one canonical representation.

---

16. Keyword Compatibility

A new keyword is potentially source-breaking because an existing identifier may use that spelling.

Therefore a new reserved word MUST have:

1. an owning specification;
2. a lexical classification;
3. a compatibility classification;
4. a collision analysis;
5. parser tests;
6. migration treatment where required.

Domain-specific concepts SHOULD remain identifiers or dialect-owned vocabulary when a universal reserved keyword is unnecessary.

This keeps the language extensible without turning the lexical layer into an application catalogue.

---

17. Literal Compatibility

Literal syntax is especially sensitive because a lexer change can reinterpret existing source.

Literal compatibility MUST cover:

- integer literals;
- floating-point literals;
- strings;
- characters;
- durations;
- sizes;
- hardware-related quantities;
- quantum state literals;
- future extensible literal forms.

A literal documented by the specification but not emitted by the Rust lexer is an implementation-conformance defect.

A literal emitted by the lexer but not specified by the language MUST NOT automatically be treated as stable language syntax.

---

18. AST Compatibility

AST compatibility MUST distinguish:

internal implementation representation

from:

published AST contract

Internal representations may evolve.

Published representations require explicit compatibility policy.

Every published AST evolution SHOULD identify:

old node
new node
source construction
semantic equivalence
serialization impact
consumer impact
migration
tests

No AST node should be created merely to make a parser rule compile.

Every AST node requires a semantic owner.

---

19. Semantic Compatibility

Semantic compatibility has priority over textual similarity.

A migration that changes:

syntax A → syntax B

is valid only if the specified semantics are preserved or the change is explicitly classified as semantic.

This is particularly important for:

- ownership;
- concurrency;
- effects;
- quantum operations;
- measurement;
- randomness;
- resource constraints;
- policies;
- adaptation;
- reflection;
- foreign calls;
- distributed operations.

---

20. Type Compatibility

Type compatibility covers the complete Zamani type architecture, including future extensible type facilities.

This includes:

- primitive types;
- records;
- tuples;
- arrays;
- slices;
- maps;
- functions;
- generics;
- generic bounds;
- associated types;
- ownership;
- references;
- linear types;
- affine types;
- dependent types;
- effect-related types;
- resource types;
- capability types;
- quantum types;
- tensor types;
- uncertainty/probability types.

Type compatibility MUST NOT encode fixed physical capacities.

For example:

Qubit[n]
Tensor<T, shape>
Memory<T, size>

describe program-level quantities.

They MUST NOT imply a universal compiler maximum.

---

21. Resource Compatibility

Resource semantics MUST distinguish:

requirement
constraint
capability
budget
preference
hint
realization

For example:

requires qubits >= required_qubits;

is a logical requirement.

It is not equivalent to:

use physical qubit 17;

Similarly:

requires memory >= required_memory;

is not equivalent to selecting a particular physical memory bank.

Compatibility MUST preserve this distinction.

---

22. Capability Compatibility

Capabilities describe what an environment can provide.

Examples include:

capability("quantum.measurement")
capability("quantum.mid_circuit_measurement")
capability("gpu.compute")
capability("tensor.compute")

Capability identifiers MUST have stable semantic definitions when they are part of a compatibility contract.

A target lacking a required capability is not automatically a source-language incompatibility.

The compiler SHOULD distinguish:

valid source

from:

target lacks required capability

and report the latter as a target-feasibility issue.

---

23. POCO-REAF Compatibility

POCO-REAF is a portability architecture.

It does not mean that one target-specific machine-code binary must execute natively on every architecture.

The architecture distinguishes:

source portability
semantic portability
IR portability
artifact portability
target realization
runtime portability
reproducibility

A valid program should remain semantically stable while its realization may change according to:

- target capabilities;
- available resources;
- topology;
- scheduling;
- routing;
- acceleration;
- distribution;
- simulation;
- resilience;
- recovery;
- deployment policy.

Therefore:

same source
    ↓
same specified meaning
    ↓
different valid realization

is compatible with POCO-REAF.

---

24. Target Feasibility Is Not Source Compatibility

The following states MUST remain separate:

source valid
        ↓
semantic valid
        ↓
target selectable
        ↓
resources sufficient
        ↓
runtime available
        ↓
execution successful

A program may be fully valid while a particular target cannot execute it.

Example:

requires qubits >= required_qubits;

If a selected target lacks sufficient usable quantum resources, the result is:

target-infeasible

not:

invalid Zamani

Likewise:

requires capability("gpu.compute");

on a target without GPU compute is a feasibility failure.

The implementation MUST NOT silently weaken the requirement.

---

25. Scalability and Compatibility

Compatibility MUST NOT impose artificial universal capacity limits.

The language architecture must remain capable of expressing computations from very small systems to computations constrained only by:

- program semantics;
- representation;
- available resources;
- target capabilities;
- physical constraints;
- implementation capacity.

The following MUST NOT become universal language limits:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_AGENT_COUNT
MAX_ACCELERATOR_COUNT

Equivalent renamed constants are prohibited when they have the same purpose.

---

26. Finite Implementations

POCO-REAF does not claim that physical machines have infinite resources.

Every implementation necessarily has finite:

- address spaces;
- representations;
- compiler memory;
- execution time;
- storage;
- device resources;
- network resources.

The important distinction is:

implementation limitation

versus:

language limitation

An implementation limitation MUST NOT silently become a language restriction.

If an implementation cannot represent a valid requested value, it MUST produce a deterministic diagnostic or otherwise follow the specified failure semantics.

It MUST NOT silently:

- truncate;
- corrupt;
- reinterpret;
- wrap unexpectedly;
- discard;
- downgrade

the requested semantics.

---

27. Quantum Compatibility

Quantum compatibility preserves quantum semantics rather than a fixed hardware gate inventory.

The canonical boundary is:

Zamani source
      ↓
domain-neutral AST
      ↓
quantum semantic model
      ↓
quantum::ir
      ↓
optimization
      ↓
routing / scheduling / resilience / QEC / ZQN
      ↓
HAL
      ↓
QPU or simulator

Compatibility MUST preserve applicable:

- qubit meaning;
- state meaning;
- operation meaning;
- parameters;
- targets;
- controls;
- adjoints;
- measurement;
- reset;
- observables;
- classical feed-forward;
- dynamic control;
- channels;
- noise semantics;
- logical/physical distinction;
- resource requirements;
- capability requirements;
- QEC requirements.

The language MUST NOT make today's known quantum operations a permanent compatibility ceiling.

Generic quantum operations remain the preferred extensibility mechanism.

Conceptually:

operationSpecifier
targets
parameters
results
attributes
modifiers
effects
capabilities
resources
source

New operations must be introducible through the appropriate semantic/dialect registration mechanisms without requiring a universal hard-coded gate catalogue.

---

28. Quantum IR Compatibility

"quantum::ir" is the canonical quantum semantic boundary.

Compatibility MUST NOT create a permanent parallel quantum IR merely to preserve historical source syntax.

Preferred path:

legacy syntax
      ↓
compatibility normalization
      ↓
current AST
      ↓
quantum semantic operation
      ↓
quantum::ir

A legacy syntax form MAY remain accepted for a defined compatibility period.

It MUST eventually converge into the canonical semantic model.

---

29. Classical Compatibility

Classical computation is a first-class Zamani domain.

Compatibility must preserve semantics for:

- scalar computation;
- integer computation;
- floating-point computation;
- vectors;
- matrices;
- tensors;
- symbolic computation;
- numerical computation;
- statistics;
- optimization;
- signal processing;
- scientific computing;
- control computation.

Adding a new CPU, vector, GPU, accelerator, or distributed backend is not automatically a language compatibility change.

---

30. HDL and Hardware Compatibility

HDL compatibility follows:

HDL source
    ↓
AST
    ↓
hardware semantics
    ↓
canonical hardware/HDL representation
    ↓
verification
    ↓
synthesis/lowering
    ↓
target realization

Compatibility MUST preserve hardware intent.

It MUST NOT silently transform portable intent into a fixed physical device model.

Target-specific syntax must be explicitly classified as:

- portable;
- target-specific;
- deprecated;
- migration-required;
- unsupported.

A target-specific form MUST NOT silently acquire portable semantics unless an explicit migration contract defines that transformation.

---

31. AI, Knowledge, Reasoning, Learning, and Adaptation Compatibility

The language architecture permits generic computational facilities such as:

infer
deduce
reason
assert
retract
query
learn
adapt
explain
evidence
provenance
uncertainty

Compatibility for these features follows the same universal rules as all other language facilities.

They MUST participate in:

- type checking;
- effect checking;
- capability checking;
- resource checking;
- contract validation;
- policy validation;
- provenance;
- deterministic/reproducible execution where required.

In particular, adaptation MUST NOT be treated as unrestricted self-modifying behavior.

An adaptation operation that changes execution strategy, learned state, model state, or other protected program state must remain subject to the applicable:

policy
capability
effect
resource
authorization
provenance
validation

contracts.

Compatibility metadata must preserve those guarantees.

---

32. Contract Compatibility

Contracts include constructs such as:

requires
ensures
invariant
assume
guarantee
property

A compatibility migration MUST preserve the contract's specified interpretation.

A compiler optimization MUST NOT silently remove a contract guarantee that the language promises.

If an implementation changes when a contract is checked, that change must be classified according to whether it changes observable semantics.

---

33. Policy Compatibility

Policies can constrain:

- security;
- execution;
- resources;
- capabilities;
- adaptation;
- simulation;
- deployment;
- quantum execution;
- distributed execution;
- foreign calls;
- reflection;
- code generation.

Compatibility MUST preserve the distinction between:

requirement
permission
prohibition
preference
fallback
implementation choice

A compatibility migration MUST NOT silently broaden permissions or weaken prohibitions.

---

34. Effect Compatibility

Effects are semantic contracts.

An implementation MAY lower an effect differently, but it MUST preserve the language-level effect guarantee.

Compatibility MUST consider effects involving:

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

A feature that previously required an effect cannot silently become effect-free if that difference is observable or security-relevant.

---

35. Provenance Compatibility

Where provenance is part of a specified contract, compatibility must preserve the required provenance information.

Relevant concepts include:

source
derived_from
generated_by
transformed_by
verified_by
reason
evidence
decision
version

Provenance is especially important for:

- compilation;
- optimization;
- AI reasoning;
- learning;
- adaptation;
- quantum transformations;
- hardware placement;
- security decisions;
- policy evaluation;
- scientific computation.

A migration that discards mandatory provenance is not compatible merely because the executable result appears equivalent.

---

36. Deterministic and Reproducible Compatibility

When a program requests deterministic or reproducible behavior, compatibility MUST preserve the applicable guarantee.

This includes compatibility of:

- random-state handling;
- ordering;
- numerical behavior;
- scheduling semantics where specified;
- distributed reduction semantics where specified;
- quantum simulation behavior where specified;
- compiler transformations affecting reproducibility.

Implementation-specific nondeterminism MUST NOT silently replace a language-level deterministic guarantee.

---

37. Concurrency and Distributed Compatibility

Compatibility must preserve the semantics of:

- actors;
- tasks;
- channels;
- message passing;
- asynchronous operations;
- synchronization;
- parallel computation;
- distributed computation;
- failure handling.

AI agents and multi-agent computation must integrate with the existing concurrency architecture rather than creating a competing actor model.

Conceptually:

agent semantics
      ↓
existing actor/concurrency semantics
      ↓
message/task/channel model
      ↓
scheduler/runtime

Compatibility changes must be evaluated at every affected layer.

---

38. Networking Compatibility

Networking compatibility covers:

- endpoints;
- messages;
- streams;
- services;
- protocols;
- discovery;
- security;
- network effects;
- capability requirements;
- policy constraints.

A network capability or protocol version MUST be distinguishable from the Zamani language version.

An external protocol upgrade does not automatically constitute a language-version change.

---

39. Interoperability and ABI Compatibility

External interfaces are independent compatibility dimensions.

For example:

Zamani language version
+
foreign ABI
+
external data format
+
quantum interchange format
+
dialect version

are separate contracts.

An external ABI change does not automatically constitute a Zamani language change.

FFI compatibility MUST preserve:

- calling convention;
- type layout;
- ownership expectations;
- lifetime requirements;
- error behavior;
- effects;
- capability requirements;
- security policy;
- provenance where required.

Rust implementation of FFI-related functionality MUST remain safe Rust.

---

40. Metaprogramming Compatibility

Macros, compile-time evaluation, reflection, introspection, quotation, syntax generation, and type-level computation are source transformations or compile-time facilities.

Compatibility MUST preserve:

- hygiene;
- name resolution;
- source mapping;
- semantic validation;
- capability requirements;
- effect requirements;
- language-version selection;
- diagnostics.

Generated syntax MUST NOT bypass compatibility checks.

Generated source must be interpreted under the correct effective language contract.

Reflection MUST NOT become a mechanism for silently bypassing:

- type checking;
- effect checking;
- capability checking;
- policy checking;
- compatibility validation;
- security validation.

---

41. Dialect Compatibility

Dialects extend Zamani without replacing the universal core.

A dialect MUST identify:

dialect name
dialect version
owner
syntax
semantic contract
capabilities
effects
resource requirements
AST representation
IR boundary
migration policy
deprecation policy
tests

Dialect-specific compatibility MUST remain distinct from core-language compatibility.

Application-specific facilities SHOULD normally be expressed through:

- libraries;
- dialects;
- capabilities;
- policies;
- services;
- APIs.

They SHOULD NOT become universal keywords unless they are genuinely universal language primitives.

---

42. Data Format Compatibility

Formats such as:

SQL
JSON
XML

are interoperability/data concerns unless the language specification explicitly promotes a construct into core Zamani syntax.

Their compatibility contracts belong to the appropriate dialect or interoperability subsystem.

Preferred architecture:

external format
      ↓
format-specific parser
      ↓
Zamani semantic data/query representation
      ↓
canonical IR or data model

rather than making every external format part of the universal grammar.

---

43. Resource and Hardware Compatibility

Compatibility MUST preserve the separation:

logical requirement
        ↓
capability negotiation
        ↓
resource feasibility
        ↓
realization

A program should be able to express:

requires qubits >= required_qubits;
requires memory >= required_memory;
requires capability("gpu.compute");
requires capability("tensor.compute");
requires topology(required_topology);

without embedding a physical machine identity.

The compiler/runtime may determine:

- target;
- placement;
- routing;
- scheduling;
- distribution;
- decomposition;
- accelerator use;
- simulation;
- fallback.

These decisions do not redefine the source-language semantics.

---

44. Target-Specific Compatibility

Target-specific constructs are permitted when deliberately requested by the programmer or dialect.

However, they MUST be classified.

A target-specific construct must not silently become a universal portable requirement.

For example:

use physical_device(...)

must not silently mean:

requires capability(...)

unless an explicit semantic/migration contract establishes equivalence.

---

45. Version Resolution

Every compilable source unit MUST have a deterministic effective language version.

The exact resolution rules belong to:

grammar/specification/language-version.md
grammar/spec/versioning.md

Potential sources can include:

1. explicit source declaration;
2. project/package manifest;
3. workspace configuration;
4. documented legacy default.

The authoritative specification MUST define precedence.

Once resolved, the effective version MUST be available to the stages that need it, including:

lexer
parser
AST construction
semantic analysis
compatibility validation
diagnostics
artifact metadata
tooling

An unsupported language version MUST result in a structured diagnostic.

The implementation MUST NOT silently select an unrelated version.

---

46. Compiler Version vs Language Version

These are independent.

For example:

Zamani language: X
compiler: Y
Rust implementation: Z

are separate versions.

A compiler upgrade is not automatically a language-breaking change.

A Rust toolchain upgrade is not automatically a Zamani language-version change.

A backend upgrade is not automatically a language-version change.

A QPU firmware upgrade is not automatically a language-version change.

Compatibility classification must identify the actual affected contract.

---

47. Rust Compatibility

The repository's production Rust implementation is required to support:

Rust 1.97 or later
Rust edition 2021
safe Rust

The concrete supported toolchain declaration remains owned by the repository's build configuration.

At present, the inspected "Cargo.toml" declares:

edition = "2021"
rust-version = "1.97"

The compatibility README does not replace that declaration.

The important compatibility requirement is that compatibility-related Rust implementation remains valid under the repository's declared Rust baseline.

Production implementation MUST NOT use Rust "unsafe".

This requirement applies to implementation code and does not by itself define whether a Zamani source-language construct named "unsafe" exists. Source-language semantics are owned by the language/security specifications.

---

48. Compatibility and Generated Documentation

Generated compatibility information MUST be deterministic.

Generated documentation MAY summarize implementation status.

It MUST identify its source authority when appropriate.

A generated document does not become authoritative merely because it contains more detail.

For example:

grammar/grammar.md

may report implementation status, while normative language meaning remains owned by:

grammar/specification/

---

49. Compatibility Metadata

Compatibility metadata SHOULD be machine-readable where practical.

A compatibility record should be capable of identifying at least:

feature
feature identifier
owning specification
introduced version
current status
affected compatibility dimensions
AST impact
semantic impact
IR impact
artifact impact
ABI impact
runtime impact
target impact
migration status
deprecation status
test status

The metadata model MUST be extensible.

It MUST NOT assume a finite number of language features, targets, domains, capabilities, resources, or dialects.

---

50. Compatibility Matrix

"compatibility-matrix.md" owns explicit cross-layer compatibility relationships.

A matrix entry SHOULD identify applicable fields such as:

feature
owner
old contract
new contract
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
target
dialect
migration
deprecation
tests

A compatibility-affecting change SHOULD update the matrix in the same change set.

A feature MUST NOT be considered production-complete while its compatibility impact is unknown.

---

51. Migration Ownership

"migrations.md" owns migration procedures.

A migration SHOULD define:

source state
target state
preconditions
transformation
semantic invariants
diagnostics
manual intervention
automated applicability
post-migration validation
rollback/recovery expectations
tests

This README only defines how migrations integrate with the rest of the repository.

---

52. Migration Safety

A migration MUST NOT:

- bypass type checking;
- bypass effect checking;
- bypass capability checking;
- bypass resource validation;
- bypass policy validation;
- weaken security;
- silently change quantum semantics;
- silently change ownership;
- silently change concurrency behavior;
- silently discard mandatory provenance.

The preferred migration pipeline is:

old source
    ↓
version-aware parsing
    ↓
validated old representation
    ↓
semantic migration
    ↓
current AST
    ↓
current semantic validation
    ↓
current canonical IR

A textual replacement alone is insufficient when semantics are involved.

---

53. Deprecation Ownership

"deprecated.md" owns:

- deprecation status;
- deprecation rationale;
- replacement;
- compatibility period;
- removal criteria;
- removal version;
- migration reference.

Deprecation MUST NOT be inferred solely from the presence of a "deprecated" token or annotation.

A stable feature is deprecated only when its owning specification and deprecation contract say so.

---

54. Historical Syntax

Historical syntax may be documented in:

grammar/Zamani-Grammar.md

and compatibility materials.

Historical documentation does not automatically make syntax valid.

A historical feature must be explicitly classified as appropriate:

historical
deprecated
unsupported
migration-required
stable
experimental

Historical syntax MUST NOT silently become stable syntax merely because an old parser still accepts it.

---

55. Compatibility and the Extended Grammar Reference

"grammar/Zamani-Grammar.md" remains an extended/historical design reference.

It may contain concepts that are:

- stable;
- proposed;
- experimental;
- deprecated;
- historical;
- not implemented.

Compatibility status must not be inferred merely from presence in that document.

Promotion remains:

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

56. Compatibility and "grammar/grammar.md"

"grammar/grammar.md" reports implementation/conformance status.

It MUST distinguish specification from implementation.

Applicable statuses include:

SPECIFIED
LEXER_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
STRUCTURAL_VALIDATION_IMPLEMENTED
SEMANTIC_IMPLEMENTED
TYPE_IMPLEMENTED
EFFECT_IMPLEMENTED
RESOURCE_IMPLEMENTED
IR_IMPLEMENTED
BACKEND_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL
DEPRECATED
PLANNED
UNIMPLEMENTED

A feature MUST NOT be marked implemented merely because grammar text exists.

A feature MUST NOT be marked stable merely because implementation exists.

---

57. Compatibility and Validation

Compatibility integrates with:

grammar/validation/

Where the following contracts exist or are established, they should participate in validation:

compatibility-rules.md
ambiguity-rules.md
hardcoding-audit.md
scalability-rules.md
semantic-boundaries.md

Compatibility validation MUST detect inconsistencies among:

specification
    ↕
formal contracts
    ↕
compatibility metadata
    ↕
lexer
    ↕
parser
    ↕
AST
    ↕
semantic implementation
    ↕
IR
    ↕
tests

---

58. Compatibility and Hard-Coding Audits

Compatibility work MUST participate in the repository hard-coding audit.

The following are prohibited as universal language capacity constants:

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

Equivalent renamed constants are subject to the same rule.

A numeric value is not automatically a violation.

For example:

let n = 1024;

is ordinary program data.

A universal rule such as:

the language supports at most 1024 qubits

is a prohibited artificial language ceiling unless it is explicitly part of a genuinely target-specific contract.

---

59. Compatibility and Resource Availability

Compatibility MUST distinguish:

program validity

from:

target feasibility

and:

runtime availability

Therefore:

valid source
+
valid semantics
+
insufficient resources

is not automatically a compatibility failure.

The compiler/runtime must report the appropriate resource-feasibility condition.

---

60. Compatibility and Scalability

The compatibility architecture must remain scale-independent.

It MUST NOT assume fixed:

- machine size;
- memory size;
- processor count;
- accelerator count;
- device count;
- qubit count;
- tensor rank;
- network size;
- agent count;
- node count.

The conceptual model is:

program intent
      ↓
required resources
      ↓
target capabilities
      ↓
available resources
      ↓
constraints/policies
      ↓
realization

This allows the same semantic program to be considered for:

tiny systems
embedded systems
single processors
multicore systems
GPUs
FPGAs
ASICs
accelerators
QPU systems
simulators
HPC systems
clusters
distributed systems
cloud systems
future architectures

without requiring the compatibility subsystem to encode a fixed physical universe.

---

61. Compatibility and Adaptive Execution

Adaptive execution may select among valid realizations according to:

- capabilities;
- resources;
- policies;
- resilience state;
- performance information;
- availability;
- explicit fallback rules.

Compatibility MUST preserve the semantic contract rather than the exact realization.

For example:

portable computation
       ↓
GPU realization

and:

portable computation
       ↓
CPU realization

may both be compatible if they satisfy the same language semantics.

If a fallback changes specified observable behavior, the fallback requires an explicit semantic contract.

---

62. Compatibility and Simulation

Simulation is an execution strategy.

Compatibility MUST distinguish:

program semantics

from:

simulation implementation

The same logical computation may be realized through:

- direct execution;
- classical simulation;
- quantum simulation;
- hardware simulation;
- distributed simulation;
- fault simulation;
- performance simulation.

A simulation backend change does not automatically constitute a source-language compatibility change.

---

63. Compatibility and Resilience

The compatibility architecture must integrate with the existing resilience model.

Where applicable, states include:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

and outcomes include:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

Compatibility metadata MUST NOT redefine those states.

It only records whether a language/runtime contract involving them remains compatible.

---

64. Compatibility and QEC/ZQN

Quantum error correction and ZQN remain downstream implementation/semantic layers.

Compatibility MUST preserve their ownership boundaries.

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

An internal QEC implementation change does not automatically require a language-version change.

A change to specified quantum semantics does.

ZQN MUST NOT become a competing source-language grammar or second quantum IR.

---

65. Compatibility and Provenance of Compilation

For compatibility-sensitive compilation, the compiler should be capable of identifying relevant provenance such as:

language version
compiler version
grammar contract
AST schema
semantic contract
IR contract
dialect versions
target information
capability information
migration information

The exact artifact schema is owned outside this README.

The compatibility requirement is that any promised provenance remains internally consistent and deterministic.

---

66. Compatibility and Compiler Caching

If compiler caches or incremental compilation artifacts are exposed as stable artifacts, their compatibility contract MUST be explicit.

The compiler MUST NOT assume that:

same source text

automatically means:

same compiler artifact

because relevant inputs may include:

- language version;
- dialect versions;
- semantic configuration;
- capability assumptions;
- resource assumptions;
- compiler version;
- target;
- optimization policy;
- external dependencies.

Cache compatibility is therefore an artifact/tooling concern, not merely source compatibility.

---

67. Compatibility and Reproducible Builds

Where reproducible builds are promised, compatibility metadata must account for all relevant semantic inputs.

Build reproducibility MUST NOT depend on undocumented:

- machine identifiers;
- physical device numbering;
- incidental scheduling;
- unstable iteration order;
- uncontrolled randomness;
- timestamps where prohibited by the reproducibility contract.

The exact reproducibility mechanism belongs to the build/compiler specification.

---

68. Compatibility and Security

Compatibility machinery MUST NOT create a bypass around:

- authorization;
- capabilities;
- effects;
- resource restrictions;
- sandboxing;
- policy enforcement;
- type checking;
- semantic validation;
- provenance requirements.

Compatibility files themselves MUST NOT introduce mechanisms that allow old syntax to evade current security requirements.

A legacy construct that is retained for compatibility remains subject to current mandatory security validation unless an explicit specification says otherwise.

---

69. Compatibility and Sandboxing

Sandbox compatibility must preserve:

effect restrictions
capability restrictions
resource restrictions
network restrictions
foreign-call restrictions
native-call restrictions
reflection restrictions
adaptation restrictions
code-generation restrictions

A migration MUST NOT silently broaden sandbox authority.

A sandboxed program migrated to a newer representation must remain subject to an equivalent or explicitly documented security policy.

---

70. Compatibility and Learning/Adaptation

Learning and adaptation features must retain their effect, policy, resource, capability, and provenance contracts across versions.

For example:

learn

must not silently become an unrestricted persistent mutation operation.

Likewise:

adapt

must not silently acquire permissions that were absent from the original contract.

Compatibility must preserve the distinction between:

learned state
program source
generated code
runtime state
persistent state

where those distinctions are part of the language/runtime contract.

---

71. Compatibility and Knowledge/Reasoning

Knowledge constructs such as:

assert
retract
query
infer
deduce
reason

must preserve their semantic contract.

Compatibility must account for:

- binding;
- scope;
- evaluation;
- evidence;
- provenance;
- uncertainty;
- confidence;
- effects;
- determinism;
- persistence;
- authorization.

The compatibility subsystem must not decide the internal reasoning algorithm.

---

72. Compatibility and Uncertainty

Uncertainty-related constructs may include:

probability
distribution
confidence
belief
uncertainty

Compatibility concerns include:

- representation;
- mathematical meaning;
- normalization;
- precision guarantees;
- propagation;
- deterministic/reproducible behavior;
- serialization.

A backend implementation may change while the specified mathematical meaning remains stable.

---

73. Compatibility and Explainability

Where "explain", evidence, decisions, or provenance are part of a language contract, compatibility must preserve their specified output obligations.

The implementation may improve explanations without breaking compatibility.

However, removing required evidence or provenance is compatibility-affecting.

Explanation data may cover:

- AI decisions;
- compiler transformations;
- resource selection;
- quantum routing;
- target selection;
- optimization;
- policy decisions;
- security decisions.

---

74. Compatibility and Application-Specific Features

Application-specific facilities should normally remain outside the universal lexical core.

Examples include application domains such as:

- vision;
- language processing;
- robotics;
- payments;
- administration;
- legal workflows;
- immersive systems;
- specialized business processes.

They should be represented through appropriate:

libraries
dialects
capabilities
policies
services
APIs

unless a future specification demonstrates that a facility is genuinely a universal language primitive.

Compatibility therefore does not require maintaining a universal keyword for every application domain.

---

75. Reserved Vocabulary Compatibility

A reserved-word registry MAY be maintained when the repository needs an explicit compatibility-sensitive inventory.

If:

grammar/compatibility/reserved.md

is introduced, it owns only:

- reserved identifiers;
- reserved future syntax;
- compatibility rationale;
- reservation lifecycle.

It MUST NOT become a general language keyword specification.

The lexer remains authoritative for actual tokenization.

---

76. Compatibility Test Architecture

Compatibility tests SHOULD be organized according to the repository's existing test architecture.

Relevant areas include:

grammar/tests/
tests/compatibility/
tests/negative/
tests/boundary/
tests/scalability/
tests/determinism/

The exact physical test location is owned by the repository's testing architecture.

Compatibility tests MUST cover applicable layers rather than merely checking parser acceptance.

---

77. Required Compatibility Test Categories

Every compatibility-affecting feature SHOULD have applicable:

lexical tests
parser tests
AST tests
semantic tests
type tests
effect tests
resource tests
capability tests
contract tests
policy tests
provenance tests
IR tests
artifact tests
ABI tests
runtime tests
target-feasibility tests
migration tests
deprecation tests
determinism tests
scalability tests
negative tests
boundary tests
cross-domain tests

Not every feature requires every category.

The test plan must explicitly state which categories apply.

---

78. Compatibility Test Independence

A compatibility test MUST identify the contract it verifies.

A test should not be considered sufficient merely because:

the parser accepts the input

For semantic compatibility, the test should verify the relevant semantic invariant.

For quantum compatibility, it should verify quantum semantics.

For resource compatibility, it should verify resource meaning.

For policy compatibility, it should verify policy enforcement.

For provenance compatibility, it should verify required provenance.

---

79. Cross-Domain Compatibility

The language is intentionally multi-domain.

Therefore compatibility tests must include combinations where appropriate, such as:

classical + quantum
classical + HDL
classical + AI
quantum + AI
quantum + distributed
AI + concurrency
AI + resources
AI + policies
AI + provenance
HDL + simulation
quantum + simulation
quantum + QEC
distributed + networking
FFI + effects
metaprogramming + compatibility
contracts + quantum
contracts + hardware
policies + adaptation

A domain feature is not production-ready merely because it works in isolation when its contract crosses other language subsystems.

---

80. Compatibility Boundary Tests

Boundary tests MUST examine transitions such as:

lexer → parser
parser → AST
AST → semantics
semantics → IR
semantic IR → quantum::ir
IR → optimization
optimization → lowering
lowering → routing
routing → scheduling
scheduling → resilience
resilience → ZQN/QEC
ZQN/QEC → HAL
HAL → target

Compatibility problems frequently occur at boundaries rather than inside individual components.

---

81. Negative Compatibility Tests

Negative tests must verify that incompatible programs are rejected for the correct reason.

Examples include:

unsupported language version
invalid migration
removed stable syntax
conflicting syntax
missing capability
insufficient resource
invalid effect
violated policy
invalid contract
unsupported dialect version
invalid ABI
invalid artifact version

The diagnostic must identify the relevant compatibility dimension.

---

82. Diagnostics

Compatibility diagnostics SHOULD be structured.

Where possible they should identify:

diagnostic code
severity
feature
source location
effective language version
affected compatibility dimension
expected contract
actual condition
migration availability
suggested action

A diagnostic MUST NOT claim source incompatibility when the actual problem is target feasibility.

For example:

required capability unavailable

is preferable to:

invalid Zamani program

when the source is semantically valid.

---

83. Compatibility Status Vocabulary

The repository SHOULD use precise status terminology.

Recommended statuses include:

SPECIFIED
IMPLEMENTED
PARTIALLY_IMPLEMENTED
EXPERIMENTAL
STABLE
DEPRECATED
PLANNED
UNIMPLEMENTED
UNSUPPORTED
MIGRATION_REQUIRED

These statuses have different meanings.

In particular:

specified ≠ implemented
implemented ≠ stable
stable ≠ target-supported
target-supported ≠ resource-feasible

---

84. Production Readiness

A compatibility feature is production-ready only when all applicable contracts are established.

At minimum:

specification
    ↓
version classification
    ↓
compatibility dimensions
    ↓
implementation
    ↓
migration/deprecation analysis
    ↓
tests
    ↓
diagnostics
    ↓
matrix entry

A feature cannot be considered production-ready merely because the grammar parses.

---

85. Definition of a Compatibility-Affecting Change

A change is compatibility-affecting if it can change any promised behavior involving:

- accepted source;
- tokenization;
- parsing;
- AST contracts;
- names;
- types;
- effects;
- resources;
- capabilities;
- contracts;
- policies;
- semantics;
- deterministic behavior;
- IR;
- artifacts;
- ABI;
- runtime;
- dialects;
- tooling.

The change author MUST identify affected dimensions.

---

86. Compatibility Review Contract

Before merging a compatibility-affecting change, reviewers SHOULD verify:

[ ] owning specification identified
[ ] owning implementation identified
[ ] language version identified
[ ] compatibility dimensions identified
[ ] source impact assessed
[ ] lexical impact assessed
[ ] syntax impact assessed
[ ] AST impact assessed
[ ] semantic impact assessed
[ ] type impact assessed
[ ] effect impact assessed
[ ] resource impact assessed
[ ] capability impact assessed
[ ] contract impact assessed
[ ] policy impact assessed
[ ] provenance impact assessed
[ ] IR impact assessed
[ ] quantum::ir impact assessed where applicable
[ ] artifact impact assessed
[ ] ABI impact assessed where applicable
[ ] runtime impact assessed where applicable
[ ] target feasibility distinguished from source compatibility
[ ] migration assessed
[ ] deprecation assessed
[ ] compatibility matrix updated
[ ] diagnostics updated
[ ] positive tests updated
[ ] negative tests updated
[ ] boundary tests updated
[ ] scalability tests updated
[ ] deterministic tests updated
[ ] hard-coding audit completed
[ ] safe-Rust requirement preserved

---

87. Independent-File Completion Contract

Compatibility files must be independently completable.

Before editing a compatibility file, its integration contract must identify:

Purpose
Owner
Does Not Own
Inputs
Outputs
Dependencies
Consumers
Specification Owner
Implementation Owner
AST Impact
Semantic Impact
IR Impact
Artifact Impact
Migration Impact
Deprecation Impact
Diagnostics
Tests
Scalability
Security
Determinism
Completion Criteria

This ensures that a file can be completed without waiting for another file to be redesigned later.

If another file must subsequently be changed, that dependency must have already been identified in the contract.

---

88. File-Specific Integration Contracts

"grammar/compatibility/README.md"

Owns:

- architecture;
- navigation;
- ownership;
- compatibility terminology;
- integration rules;
- production criteria.

Does not own:

- version numbering;
- migration algorithms;
- deprecation records;
- language syntax;
- semantic definitions.

---

"grammar/compatibility/versions.md"

Owns:

- language version policy;
- release relationships;
- version compatibility;
- version lifecycle.

Depends on:

grammar/specification/language-version.md
grammar/spec/versioning.md

Consumed by:

migrations.md
deprecated.md
compatibility-matrix.md
compiler/tooling
tests

---

"grammar/compatibility/migrations.md"

Owns:

- source migration procedures;
- transformations;
- semantic validation;
- migration diagnostics;
- migration tests.

Consumes:

versions.md
deprecated.md
specification/
spec/

Produces:

migration contracts
migration metadata
migration test requirements

---

"grammar/compatibility/deprecated.md"

Owns:

- deprecation lifecycle;
- deprecation status;
- replacement;
- removal policy;
- removal version;
- migration references.

Does not own:

- migration implementation.

---

"grammar/compatibility/compatibility-matrix.md"

Owns:

- explicit feature-to-layer compatibility relationships.

It must not become a syntax specification.

---

"grammar/compatibility/reserved.md"

If introduced, owns only compatibility-sensitive reservation policy.

The actual lexer remains owned by the lexer architecture.

---

89. Integration With "grammar/lexer/"

Compatibility integrates with the lexer through:

specification/lexical.md
        ↓
lexer/tokens.g4
        ↓
antlr/ZamaniLexer.g4
        ↓
src/lexer.rs

The token registry should identify:

name
spelling
category
owner
reserved status
context
compatibility
tests

Compatibility changes to keywords must be evaluated for identifier collisions.

Compatibility changes to operators must be evaluated for precedence and ambiguity.

Compatibility changes to literals must be evaluated for lexical reinterpretation.

---

90. Integration With "grammar/core/"

Universal constructs should remain compatibility-neutral wherever possible.

Core compatibility includes:

- identifiers;
- names;
- attributes;
- annotations;
- modifiers;
- requirements;
- constraints;
- capabilities;
- metadata.

Compatibility must preserve their generic semantics.

Domain-specific extensions must not silently change core constructs.

---

91. Integration With "grammar/types/"

Compatibility must integrate with future type-system expansion.

Relevant types may include:

generics
bounds
associated types
linear
affine
dependent
uncertain
probability
distribution
confidence
quantum
tensor
resource
capability

New type capabilities must not require compatibility rules based on fixed machine sizes.

---

92. Integration With "grammar/effects/"

All compatibility-affecting effect changes must flow through the existing effects architecture.

The compatibility layer records the change.

The effects specification defines the meaning.

The implementation enforces it.

The tests verify it.

---

93. Integration With "grammar/resources/"

Compatibility must integrate with:

requirements
capabilities
constraints
budgets
preferences
hints
negotiation
scalability

The compatibility system MUST NOT convert these into physical hardware constants.

---

94. Integration With "grammar/policies/"

Where the policy subsystem exists or is established, compatibility must preserve:

scope
permissions
prohibitions
requirements
constraints
preferences
fallbacks

A policy migration must preserve authorization semantics unless an explicit security migration says otherwise.

---

95. Integration With "grammar/validation/"

Compatibility validation consumes the repository validation architecture.

The compatibility subsystem must not duplicate validation logic unnecessarily.

The preferred relationship is:

compatibility metadata
        ↓
validation rules
        ↓
implementation checks
        ↓
conformance tests

---

96. Integration With "grammar/ai/"

Compatibility must cover generic computational intelligence features without making application-specific concepts universal language primitives.

Relevant compatibility dimensions include:

- reasoning;
- knowledge;
- learning;
- adaptation;
- uncertainty;
- evidence;
- explanation;
- provenance;
- decisions;
- agents.

Each feature remains subject to the common type/effect/resource/capability/policy architecture.

---

97. Integration With "grammar/quantum/"

Quantum compatibility must converge on:

domain-neutral AST
        ↓
quantum semantic model
        ↓
quantum::ir

The compatibility layer must not create a second quantum semantic architecture.

---

98. Integration With "grammar/hybrid/"

Hybrid compatibility covers transitions such as:

classical → quantum
quantum → classical
classical control → quantum operation
measurement → classical decision
AI → quantum
quantum → AI
host → accelerator
accelerator → host

Compatibility must preserve the semantic relationship rather than the physical implementation.

---

99. Integration With "grammar/hdl/" and "grammar/hardware/"

Compatibility must preserve:

logical hardware intent

independently from:

physical realization

Changes to synthesis, placement, timing infrastructure, FPGA implementation, ASIC implementation, or hardware targets do not automatically imply language incompatibility.

---

100. Integration With "grammar/concurrency/"

The compatibility system must use the existing concurrency model.

It must not create another actor model solely for intelligent/multi-agent computation.

Where AI agents use concurrency:

agent
  ↓
existing actor/task model
  ↓
messages/channels
  ↓
scheduler/runtime

---

101. Integration With "grammar/distributed/"

Compatibility must distinguish:

distributed semantics

from:

current cluster size

No compatibility contract may encode a fixed universal node count.

---

102. Integration With "grammar/networking/"

Compatibility must preserve:

- protocol semantics;
- endpoint semantics;
- message semantics;
- network effects;
- security requirements;
- capability requirements.

Protocol version and language version remain independent.

---

103. Integration With "grammar/interoperability/"

External languages and formats have independent contracts.

Examples include:

C ABI
C++
external data formats
quantum interchange formats
vendor interfaces
foreign runtimes

Compatibility must identify the external contract separately from the Zamani language contract.

---

104. Integration With "grammar/metaprogramming/"

Generated syntax must be checked under the effective language contract.

Reflection and generation must not bypass compatibility validation.

A macro or generator version change is compatibility-affecting when it changes generated semantics or published expansion contracts.

---

105. Compatibility and Canonical IR

The compatibility subsystem must preserve the distinction between:

source representation
AST
semantic representation
canonical IR
target-specific representation

A compatibility migration should normally normalize old source into the current semantic pipeline.

The preferred direction is:

legacy source
      ↓
current AST
      ↓
current semantics
      ↓
canonical IR

rather than maintaining permanent legacy pipelines.

---

106. Compatibility and Optimization

Compiler optimization is permitted to change implementation strategy while preserving specified semantics.

Compatibility MUST therefore distinguish:

semantic result

from:

optimization strategy

An optimization change is not automatically a language break.

It becomes compatibility-affecting when it changes a promised observable property.

---

107. Compatibility and Lowering

Lowering may select:

- instructions;
- kernels;
- circuits;
- device operations;
- distributed tasks;
- hardware structures;
- runtime mechanisms.

Compatibility remains concerned with preserving the source semantic contract.

Target-specific lowering details do not become language semantics merely because they exist in an implementation.

---

108. Compatibility and Routing

Routing is downstream from semantic intent.

Quantum routing, network routing, data placement, and hardware placement must remain separate from source semantics unless the programmer explicitly expresses target-specific requirements.

Compatibility must preserve the logical operation rather than a particular route.

---

109. Compatibility and Scheduling

Scheduling may change due to:

- target;
- load;
- resources;
- topology;
- policy;
- resilience;
- optimization.

A scheduling change is compatible when it preserves specified observable semantics.

---

110. Compatibility and HAL

The HAL is a target-realization boundary.

Compatibility MUST NOT make the HAL a language authority.

HAL changes may affect:

- target support;
- artifact compatibility;
- runtime compatibility;
- backend compatibility.

They do not automatically change the Zamani language.

---

111. Compatibility and Future Hardware

The compatibility architecture must not assume today's hardware is the final hardware model.

New target classes must be representable through:

capabilities
resources
constraints
policies
dialects
semantic operations
HAL/backends

rather than requiring universal grammar rewrites for every new machine category.

---

112. Compatibility and Future Computational Domains

The same rule applies to future computational domains.

A new domain should preferably integrate through:

common syntax foundations
+
domain semantics
+
capabilities
+
resources
+
effects
+
policies
+
canonical semantic model
+
IR/lowering

rather than adding a new universal keyword for every future concept.

Compatibility must therefore be extensible.

---

113. No Artificial Compatibility Ceiling

The compatibility architecture MUST NOT contain a finite enumeration of all possible:

- devices;
- architectures;
- accelerators;
- QPUs;
- domains;
- protocols;
- algorithms;
- AI models;
- quantum operations;
- tensor shapes;
- network sizes.

Compatibility is based on contracts and extensible identifiers, not an exhaustive physical catalogue.

---

114. Compatibility and Capability Registries

Capability identifiers SHOULD be registry-backed.

A capability registry should identify:

capability name
semantic meaning
owner
version
requirements
effects
resource implications
compatibility status
tests

The registry must be extensible.

A capability name MUST NOT be treated as proof that a target actually provides it.

Target discovery remains responsible for determining availability.

---

115. Compatibility and Resource Negotiation

Resource negotiation should operate after source semantics have been established.

Conceptually:

source requirement
        ↓
semantic requirement
        ↓
candidate target
        ↓
capability/resource evaluation
        ↓
policy evaluation
        ↓
realization

Compatibility does not itself perform target negotiation.

It ensures that changes to requirement semantics are correctly classified.

---

116. Compatibility and Fallbacks

Fallback behavior must be explicit.

Potential realizations include:

direct execution
simulation
decomposition
distribution
alternate accelerator
CPU fallback
GPU fallback
quantum simulation
recovery
retry

A fallback that preserves semantics is an implementation strategy.

A fallback that weakens semantics requires explicit language/policy authorization.

Compatibility MUST preserve this distinction.

---

117. Compatibility and Error Handling

Compatibility failures must be deterministic.

The implementation must distinguish at least:

unsupported version
invalid source
invalid migration
unsupported feature
missing capability
insufficient resource
policy rejection
ABI incompatibility
artifact incompatibility
runtime unavailability
target infeasibility

These conditions must not collapse into a generic "incompatible" error when a more precise classification is possible.

---

118. Compatibility and Source Spans

Migration and diagnostics must preserve source locations where required.

At minimum, compatibility processing SHOULD preserve:

file
line
column
byte/character span
origin
generated-source relationship

Generated code should retain source provenance where supported.

---

119. Compatibility and Unicode

Lexical compatibility must account for the repository's Unicode policy.

Changes to identifier acceptance must be reviewed for:

- normalization;
- identifier collision;
- source encoding;
- diagnostic positions;
- parser behavior;
- compatibility with existing source.

Unicode support must not introduce an arbitrary identifier-length ceiling.

---

120. Compatibility and Namespaces

Adding new reserved vocabulary may collide with existing namespaces.

Before reserving a new word, the implementation must assess:

existing identifiers
module names
field names
function names
type names
dialect names
macro names

Migration should be provided where a stable identifier is affected.

---

121. Compatibility and Comments

Comment syntax changes can affect source interpretation if comment delimiters overlap with operators or literals.

Therefore changes to:

line comments
block comments
documentation comments

must be lexically tested.

Comments that are preserved for tooling must retain their compatibility contract where tooling depends on them.

---

122. Compatibility and Diagnostics

Diagnostics themselves may have compatibility contracts when consumed by tools.

Where structured diagnostic codes are published, changing their meaning requires compatibility analysis.

Human-readable wording may evolve without being a language break if the structured diagnostic contract remains stable.

---

123. Compatibility and Tooling

Tooling includes:

- language servers;
- formatters;
- syntax highlighters;
- documentation generators;
- analyzers;
- migration tools;
- IDE integrations;
- compiler tooling.

Tool compatibility must not be confused with language compatibility.

However, published tool protocols require their own compatibility contracts.

---

124. Compatibility and Package/Project Metadata

Package and project manifests may participate in language-version resolution.

Their semantics are owned by the package/build specification.

Compatibility must preserve the distinction between:

language version
package format version
compiler version
dependency version
dialect version
artifact version

---

125. Compatibility and Dependencies

A program may depend on components with independent versions.

Compatibility analysis should be able to distinguish:

language
compiler
library
dialect
ABI
runtime
target
external protocol

A dependency upgrade does not automatically imply a language compatibility change.

---

126. Compatibility and Artifact Metadata

Where artifacts contain version metadata, that metadata should be sufficient to determine relevant compatibility contracts.

Potential metadata includes:

language version
compiler version
AST schema version where applicable
semantic model version where applicable
IR version
dialect versions
target contract
ABI contract
runtime contract
provenance

The exact schema belongs to artifact/build specifications.

---

127. Compatibility and Serialization

Serialized language structures must have explicit version contracts when they cross a stable boundary.

This may include:

- AST serialization;
- semantic metadata;
- IR;
- compiler artifacts;
- package metadata;
- diagnostics;
- provenance.

Internal temporary serialization need not become a permanent public compatibility contract unless exposed.

---

128. Compatibility and ABI Evolution

ABI changes must be classified independently from language syntax.

An ABI migration must account for:

calling convention
data layout
alignment
ownership
lifetime
error model
symbol naming
linkage
effects
capabilities

A language feature may remain source-compatible while a foreign ABI changes.

The compiler must report the correct compatibility dimension.

---

129. Compatibility and Runtime Evolution

Runtime changes must preserve the runtime contracts promised by the language.

Runtime implementation improvements are compatible when they preserve those contracts.

Runtime behavior that changes observable semantics requires explicit compatibility classification.

---

130. Compatibility and Target Evolution

Adding a target should normally be additive.

For example, support for a new:

CPU
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC system
distributed platform

is not automatically a language compatibility change.

The target must satisfy the semantic contract of the program.

---

131. Compatibility and Target Removal

Removing a target backend is not automatically a language break, but it may be a:

target compatibility break

if the project explicitly promised support for that target.

Such removal must be documented and classified separately from source-language compatibility.

---

132. Compatibility and Dialect Removal

A dialect removal is a dialect compatibility break.

It does not automatically imply that core Zamani source compatibility has been broken.

The dialect must have its own migration/deprecation lifecycle.

---

133. Compatibility and External Standards

External standards have independent versions.

For example:

external format version
protocol version
ABI version
hardware interface version

must not be conflated with Zamani language versions.

The interoperability layer owns the external standard mapping.

Compatibility records the relationship.

---

134. Compatibility and the One-Language Principle

Zamani must remain one coherent language.

Compatibility mechanisms must not produce permanent sublanguages that have incompatible semantic foundations.

Domain extensions must converge on the common:

types
operations
effects
resources
capabilities
contracts
policies
provenance
semantic model
IR

architecture.

---

135. Compatibility and Extensibility

The compatibility model must be open-ended.

It must allow future additions without rewriting the compatibility architecture itself.

Examples include future:

- quantum operations;
- accelerator classes;
- hardware technologies;
- AI models;
- reasoning mechanisms;
- data systems;
- network protocols;
- distributed mechanisms;
- execution strategies;
- dialects.

New items should enter through extensible contracts rather than fixed universal enumerations.

---

136. Compatibility and Program Meaning

The most important invariant is:

program meaning

must remain independent of:

physical machine identity

when the program uses portable semantics.

This is the central compatibility property supporting POCO-REAF.

---

137. Production Compatibility Pipeline

A compatibility-affecting feature must be traceable through:

SPECIFICATION
      ↓
VERSION CONTRACT
      ↓
COMPATIBILITY CLASSIFICATION
      ↓
LEXER
      ↓
PARSER
      ↓
AST
      ↓
STRUCTURAL VALIDATION
      ↓
TYPE VALIDATION
      ↓
EFFECT VALIDATION
      ↓
RESOURCE VALIDATION
      ↓
CAPABILITY VALIDATION
      ↓
CONTRACT VALIDATION
      ↓
POLICY VALIDATION
      ↓
PROVENANCE
      ↓
SEMANTIC MODEL
      ↓
CANONICAL IR
      ↓
QUANTUM::IR where applicable
      ↓
OPTIMIZATION
      ↓
LOWERING
      ↓
ROUTING
      ↓
SCHEDULING
      ↓
RESILIENCE
      ↓
QEC / ZQN where applicable
      ↓
HAL
      ↓
TARGET
      ↓
CONFORMANCE TESTS

Not every feature traverses every stage.

The feature contract must identify the applicable path.

---

138. Compatibility Completion Criteria

The compatibility architecture is production-ready only when:

[ ] authority hierarchy is unambiguous
[ ] every compatibility concern has an owner
[ ] language version ownership is unambiguous
[ ] compatibility dimensions are defined
[ ] source compatibility is tested
[ ] lexical compatibility is tested
[ ] syntax compatibility is tested
[ ] AST compatibility is tested where applicable
[ ] type compatibility is tested
[ ] effect compatibility is tested
[ ] resource compatibility is tested
[ ] capability compatibility is tested
[ ] contract compatibility is tested
[ ] policy compatibility is tested
[ ] provenance compatibility is tested
[ ] semantic compatibility is tested
[ ] determinism is tested where promised
[ ] quantum compatibility is tested
[ ] classical compatibility is tested
[ ] HDL compatibility is tested
[ ] hardware compatibility is tested
[ ] hybrid compatibility is tested
[ ] concurrency compatibility is tested
[ ] distributed compatibility is tested
[ ] networking compatibility is tested
[ ] AI/data compatibility is tested
[ ] interoperability compatibility is tested
[ ] dialect compatibility is tested
[ ] artifact compatibility is tested where promised
[ ] ABI compatibility is tested where promised
[ ] runtime compatibility is tested where promised
[ ] target compatibility is tested where promised
[ ] target feasibility is distinguished from source compatibility
[ ] migration paths exist where required
[ ] deprecation lifecycle exists where required
[ ] diagnostics identify compatibility dimensions
[ ] compatibility matrix is complete
[ ] hard-coding audit passes
[ ] scalability audit passes
[ ] no artificial universal resource ceilings exist
[ ] generated metadata is deterministic
[ ] implementation status matches reality
[ ] Rust implementation uses no unsafe
[ ] declared Rust baseline is supported

---

139. Definition of "Breaking"

A change is breaking when it invalidates a previously guaranteed contract.

A break may occur at:

source
lexical
syntax
AST
name/module
type
effect
resource
capability
contract
policy
semantic
IR
artifact
ABI
runtime
target
dialect
tooling

The change must identify the affected dimension.

A target that cannot satisfy a program requirement is not automatically a source-language break.

A backend that cannot implement a capability is not automatically a language break.

An optimization change is not automatically a language break.

---

140. Definition of "Migration Required"

Migration is required when an old representation cannot remain supported without unacceptable:

- ambiguity;
- semantic contradiction;
- security weakness;
- parser conflict;
- incompatibility with a new guaranteed semantic contract;
- deliberate removal of a deprecated contract.

Migration should not be required merely because:

- a backend changed;
- hardware changed;
- one vendor changed;
- one machine changed;
- a target has insufficient resources;
- an implementation was temporarily incomplete.

---

141. Definition of "Target-Infeasible"

A program is target-infeasible when:

program is semantically valid
+
compatibility requirements are satisfied
+
selected target cannot satisfy required resources/capabilities/constraints

Examples:

requires qubits >= required_qubits;

when insufficient usable quantum resources exist.

Or:

requires capability("gpu.compute");

when the selected target does not provide the required capability.

The source program remains a valid Zamani program.

---

142. Definition of "Production-Ready"

"grammar/compatibility/" is production-ready when:

every compatibility decision
        ↓
has an identifiable owner
        ↓
has a defined compatibility dimension
        ↓
has a language/version relationship where applicable
        ↓
has affected-layer analysis
        ↓
has diagnostics where required
        ↓
has migration/deprecation treatment where required
        ↓
has conformance tests
        ↓
has deterministic behavior where promised
        ↓
passes scalability validation
        ↓
passes hard-coding validation
        ↓
does not weaken security
        ↓
does not require Rust unsafe

---

143. Final Architectural Invariants

The following invariants are mandatory.

Invariant 1 — One language

There is one coherent Zamani language.

Invariant 2 — One canonical grammar composition

"grammar/Zamani.g4" remains the canonical ANTLR composition root.

Invariant 3 — One lexical authority

The lexical architecture must converge on the repository's canonical lexer contract.

Invariant 4 — Domain-neutral AST

The AST must represent source meaning without embedding physical target decisions.

Invariant 5 — Canonical quantum boundary

"quantum::ir" remains the canonical quantum semantic boundary.

Invariant 6 — Semantic ownership

Compatibility documents must not redefine language semantics.

Invariant 7 — Version ownership

Language-version semantics remain owned by the language-version specification.

Invariant 8 — Migration ownership

Migration procedures remain owned by "migrations.md".

Invariant 9 — Deprecation ownership

Deprecation lifecycle remains owned by "deprecated.md".

Invariant 10 — Matrix ownership

Cross-layer compatibility relationships remain owned by "compatibility-matrix.md".

Invariant 11 — Resource independence

Compatibility must not encode artificial universal hardware limits.

Invariant 12 — Capability independence

Target capability must remain distinct from source validity.

Invariant 13 — Resource feasibility independence

Insufficient target resources do not automatically make source code invalid.

Invariant 14 — Policy preservation

Compatibility mechanisms must not silently weaken security or policy guarantees.

Invariant 15 — Effect preservation

Compatibility mechanisms must preserve specified effect semantics.

Invariant 16 — Provenance preservation

Required provenance must survive compatibility transformations.

Invariant 17 — Quantum extensibility

Compatibility must not turn a finite known quantum operation set into a permanent language ceiling.

Invariant 18 — Hardware independence

Portable source semantics must not depend on a fixed physical machine model.

Invariant 19 — Safe Rust

The production Rust implementation must not use Rust "unsafe".

Invariant 20 — Extensibility

Future computational domains must be incorporable without redesigning the universal compatibility architecture.

Invariant 21 — Deterministic compatibility

Version resolution, compatibility classification, migration applicability, and diagnostics must be deterministic.

Invariant 22 — Explicit incompatibility

Incompatibility must be reported rather than silently repaired in a way that changes semantics.

Invariant 23 — No hidden second compiler

Compatibility must normalize legacy representations into the current semantic pipeline whenever practical.

Invariant 24 — No hidden second language

Compatibility syntax must not become an accidental permanent sublanguage.

---

144. Final POCO-REAF Compatibility Contract

The ultimate compatibility guarantee is:

Zamani source
      ↓
stable language meaning
      ↓
portable program intent
      ↓
domain-neutral semantic model
      ↓
canonical IR
      ↓
target-independent optimization
      ↓
target-aware realization

The same program can therefore be considered across environments ranging from:

minimal computation
embedded system
single processor
multicore processor
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC system
cluster
distributed system
cloud environment
future computational architecture

subject to:

program semantics
+
required capabilities
+
resource availability
+
declared constraints
+
policies
+
physical feasibility
+
implementation support

The language does not need to know in advance how many processors, GPUs, FPGAs, QPUs, nodes, devices, threads, tensor dimensions, memory units, or other resources the future contains.

The source describes computation and intent.

The semantic system establishes meaning.

The resource system describes requirements.

The capability system describes what a target provides.

Policies constrain permissible realization.

The compiler determines valid realization strategies.

Optimization, lowering, routing, scheduling, resilience, QEC, ZQN, HAL, and backends determine how that intent is realized.

Compatibility protects the contracts between these layers.

Therefore the fundamental architecture is:

PROGRAM
   ↓
LANGUAGE CONTRACT
   ↓
PORTABLE MEANING
   ↓
SEMANTIC MODEL
   ↓
CANONICAL IR
   ↓
TARGET-INDEPENDENT COMPILATION
   ↓
TARGET-AWARE REALIZATION

with:

compatibility

preserving the contracts between every layer without becoming the owner of any layer's semantics.

This is the compatibility foundation required for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

while preserving extensibility, scalability, quantum/classical/HDL/hybrid computation, intelligent computation, interoperability, deterministic behavior, security, and safe-Rust implementation.