Zamani Language Specification

Path: "grammar/specification/language.md"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Canonical branch: "main"
Language edition: Zamani language specification, versioned independently of compiler implementation
Implementation baseline: Rust 2021, Rust 1.97 / Rust 1.97.1
Implementation safety policy: production Rust code MUST use safe Rust; "unsafe" Rust is prohibited
Primary objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: from the smallest supported computation to arbitrarily large computations, limited only by program semantics, representation limits, explicitly declared policies, compiler/runtime resources, target capabilities, and resources actually available.

---

1. Status

This document is the normative language-level specification for the meaning and architectural scope of Zamani.

It defines:

- the language identity;
- the source-program model;
- language-wide invariants;
- lexical and syntactic boundaries;
- semantic portability;
- type and effect principles;
- resource and capability semantics;
- classical computation;
- quantum computation;
- hybrid computation;
- hardware description and hardware/software co-design;
- distributed and parallel computation;
- AI/ML and data computation;
- networking;
- security and cryptography;
- interoperability;
- metaprogramming;
- dialects;
- compilation and execution intent;
- scalability;
- determinism;
- provenance;
- compatibility;
- integration with the Zamani compiler and IR architecture.

This document does not by itself claim that every described feature is currently implemented.

A feature is part of the production implementation only when its complete implementation contract exists:

language specification
        ↓
lexical contract
        ↓
grammar contract
        ↓
lexer
        ↓
parser
        ↓
domain-neutral AST
        ↓
structural validation
        ↓
name/module resolution
        ↓
type/effect/resource/capability analysis
        ↓
canonical semantic representation
        ↓
canonical IR
        ↓
compiler/lowering
        ↓
runtime/backend integration
        ↓
tests
        ↓
compatibility

The implementation-conformance status of individual features belongs in "grammar/grammar.md" and the relevant "grammar/spec/" and domain contracts.

---

2. Language Identity

Zamani is a universal, extensible programming language for expressing computational intent independently of a particular machine realization.

Zamani is intended to provide one coherent language for:

- classical computation;
- systems programming;
- embedded programming;
- scientific computing;
- numerical computing;
- symbolic computing;
- high-performance computing;
- parallel computing;
- distributed computing;
- heterogeneous computing;
- quantum computing;
- hybrid quantum-classical computing;
- quantum error-corrected computation;
- hardware description;
- hardware/software co-design;
- accelerator programming;
- AI and machine learning;
- tensor computation;
- data processing;
- networking;
- cryptography;
- security;
- edge computing;
- cloud computing;
- nano-oriented computation;
- temporal computation;
- multi-timeline computation;
- metaprogramming;
- interoperability;
- domain-specific extensions;
- future computational paradigms.

These are domains of one language, not unrelated languages joined by a collection of parser switches.

All domains share the same fundamental:

- lexical model;
- source-location model;
- identifier model;
- expression model;
- declaration model;
- type system;
- effect model;
- resource model;
- capability model;
- module model;
- diagnostic model;
- versioning model;
- compatibility model;
- semantic validation architecture.

---

3. Normative Authority

Zamani has one language.

The repository contains several representations of that language, each with a different responsibility.

3.1 Normative language specification

The normative specification is:

grammar/specification/
grammar/spec/

This file, "grammar/specification/language.md", defines the language-wide semantic and architectural contract.

Other specification files specialize individual areas without contradicting this document.

Relevant specialized specifications include:

grammar/specification/lexical.md
grammar/specification/syntax.md
grammar/specification/semantics.md
grammar/specification/types.md
grammar/specification/portability.md

and the corresponding contracts under:

grammar/spec/

No specialized document may weaken a language-wide invariant defined here without an explicit language-version change.

---

3.2 Canonical ANTLR representation

The canonical ANTLR grammar is:

grammar/Zamani.g4

It MUST remain.

It is the canonical grammar representation used for ANTLR-based parsing and grammar validation.

It MUST conform to the normative specification.

It MUST NOT silently introduce semantics absent from the specification.

It MUST NOT become a second semantic architecture.

Domain grammar files under "grammar/" may provide modular grammar contracts, but "grammar/Zamani.g4" remains the composition root.

---

3.3 Implementation-conformance reference

The existing:

grammar/grammar.md

is the implementation-conformance reference.

It describes the relationship between the normative specification and the current compiler implementation.

It MUST distinguish at least:

SPECIFIED
LEXICALLY_IMPLEMENTED
PARSED
AST_IMPLEMENTED
SEMANTICALLY_IMPLEMENTED
IR_IMPLEMENTED
COMPILER_IMPLEMENTED
RUNTIME_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL
DEPRECATED

A feature MUST NOT be called "STABLE" merely because its grammar parses.

---

3.4 Extended design reference

The existing:

grammar/Zamani-Grammar.md

is retained.

It may contain:

- historical designs;
- proposed features;
- experimental constructs;
- NIMBUS/Universal-Trinity concepts;
- Sankofa concepts;
- temporal and multi-timeline concepts;
- nano concepts;
- AI concepts;
- advanced system concepts;
- future computational models.

Presence in that document does not make a construct legal Zamani syntax.

Promotion follows:

design
  ↓
normative specification
  ↓
lexical contract
  ↓
grammar
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
semantic model
  ↓
IR
  ↓
compiler/runtime
  ↓
tests
  ↓
stable

---

4. Fundamental Semantic Principle

Zamani source describes portable computational meaning.

It may express:

- what computation must occur;
- what values and relationships exist;
- what correctness properties are required;
- what capabilities are required;
- what resources are required;
- what constraints apply;
- what preferences exist;
- what optimization hints are useful;
- what deployment properties are required.

Zamani source SHOULD NOT unnecessarily specify:

- a particular CPU;
- a particular GPU;
- a particular QPU;
- a particular FPGA;
- a physical qubit;
- a physical memory bank;
- a particular network node;
- a fixed number of cores;
- a fixed number of threads;
- a vendor-specific instruction;
- a vendor-specific topology.

The compiler, scheduler, router, runtime, HAL, deployment system, or backend determines target realization.

Therefore:

program meaning
    ≠
target realization

and:

logical intent
    ≠
physical placement

---

5. POCO-REAF

Zamani is designed around:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever.»

POCO-REAF means that a program's stable semantics are independent of a particular machine realization.

It does not mean that every target can execute every program regardless of resources.

The following states are distinct:

lexically valid
    ≠
syntactically valid
    ≠
semantically valid
    ≠
compilable
    ≠
target compatible
    ≠
resource feasible
    ≠
runtime available

For example, a valid program may require resources that a particular QPU does not possess.

The compiler MAY:

- select another target;
- distribute work;
- use logical resources;
- decompose operations;
- route operations;
- schedule operations;
- apply semantics-preserving optimization;
- apply explicitly permitted error-correction transformations;
- simulate the computation;
- defer execution;
- reject the target with a precise diagnostic.

The compiler MUST NOT silently change the program's meaning merely because a target is smaller or different.

---

6. Scalability: Tiny to Arbitrarily Large

Zamani has no universal machine-size ceiling.

The language MUST be capable of expressing computations ranging from:

one value
one operation
one function
one data item
one qubit
one processor
one device

to arbitrarily large:

programs
datasets
tensors
quantum computations
distributed systems
heterogeneous systems
hardware systems
resource graphs
execution graphs

subject to actual:

- program semantics;
- representation capabilities;
- compiler resources;
- runtime resources;
- declared policies;
- target capabilities;
- physical resources.

The phrase infinity in the architectural objective means:

«Zamani MUST NOT encode an artificial finite machine-size ceiling into the language merely because current implementations are finite.»

Every concrete compilation and execution remains finite because concrete machines, address spaces, processes, files, networks, and physical systems are finite.

---

7. Prohibition on Artificial Hard-Coding

The language MUST NOT encode universal limits such as:

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
MAX_AGENTS
MAX_DEVICES
MAX_ACCELERATORS
MAX_NETWORK_LINKS
MAX_GATE_COUNT

It MUST NOT define physical resources as a finite universal enumeration:

Qubit0
Qubit1
...
Qubit1023

The prohibition applies to artificial language or implementation ceilings.

It does not prohibit ordinary program values.

For example:

let n = 1024;
allocate n elements;

is valid.

Likewise:

Tensor<f64, [1024, 1024]>

may be valid program semantics.

What is prohibited is a language-level statement such as:

Zamani tensors cannot exceed 1024 × 1024.

Target-specific limits belong to target capability/resource policy.

---

8. Resource Semantics

Zamani MUST distinguish:

1. semantic requirement;
2. capability requirement;
3. resource constraint;
4. preference;
5. hint;
6. implementation realization.

8.1 Requirement

A requirement is semantically necessary.

Conceptually:

requires qubits >= n

8.2 Capability

A capability identifies functionality required from a target.

Conceptually:

requires capability("quantum.mid_circuit_measurement")

8.3 Constraint

A constraint restricts valid realization.

Conceptually:

constraint latency <= budget

8.4 Preference

A preference guides target selection without necessarily being mandatory.

Conceptually:

prefer accelerator("quantum")

8.5 Hint

A hint gives optimization information without changing program semantics.

Conceptually:

hint locality

8.6 Realization

A realization chooses a physical implementation.

Conceptually:

logical_qubit
    ↓
physical_qubit

Physical realization belongs downstream.

---

9. Semantic Requirements Versus Implementation Decisions

The following distinction is mandatory.

Portable semantic intent

requires qubits >= n
requires memory >= required_memory
requires capability("tensor.compute")
requires capability("quantum.measurement")
requires reliability >= required_reliability

Target-specific realization

physical_qubit(17)
gpu_device(3)
memory_bank(2)
node(14)

The first group may be part of portable Zamani semantics.

The second group is target/deployment intent and MUST NOT become a hidden universal language dependency.

If target-specific syntax is deliberately supported, it MUST be explicitly marked as target-specific and isolated from the portable semantic model.

---

10. Source Program Model

A Zamani program is a sequence of source units forming a module/dependency graph.

A source unit may contain:

- documentation;
- attributes;
- language declarations;
- package/module declarations;
- imports;
- exports;
- dialect declarations;
- declarations;
- executable statements where permitted.

Conceptually:

program
    = source_unit* ;

The exact concrete grammar belongs to "grammar/Zamani.g4" and the syntax contracts.

The semantic model MUST treat the complete source graph rather than assuming one physical source file is the entire program.

---

11. Source Locations and Provenance

Every syntactic construct that can produce a diagnostic MUST retain a source span.

The minimum source-location identity is:

file identity
+
start position
+
end position

Source locations MUST remain stable enough to support:

- diagnostics;
- IDE tooling;
- source mapping;
- semantic errors;
- generated-code provenance;
- optimization provenance;
- IR provenance;
- debugging;
- reproducibility.

Generated or transformed constructs MUST retain provenance back to their originating source where feasible.

The canonical source-map implementation remains outside the grammar specification.

---

12. Lexical Model

All native Zamani domains use one lexical model.

The lexical architecture is defined by:

grammar/specification/lexical.md
grammar/lexer/
src/lexer.rs

The lexer MUST provide:

- deterministic tokenization;
- UTF-8 source handling;
- source spans;
- stable token identity;
- deterministic operator recognition;
- literal recognition;
- structured diagnostics;
- malformed-input recovery;
- no target-dependent tokenization;
- no machine-size assumptions.

The current lexer contains a broad inventory of core, quantum, nano, Sankofa, temporal, mathematical, OOP, and advanced-system keywords. That inventory is implementation input to the lexical audit; it does not by itself establish the final normative keyword set.

---

13. Token Identity

A lexical spelling MUST have one canonical token identity unless a documented lexical distinction genuinely requires otherwise.

Existing overlapping concepts such as:

BitAnd / Ampersand
BitOr  / Pipe
Question / QuestionMark

must be resolved by the lexical contract rather than allowed to become accidental duplicate token systems.

Where one spelling has multiple semantic meanings, context should normally be resolved by parsing and semantic analysis rather than by inventing multiple indistinguishable lexical tokens.

---

14. Keywords

A concept MUST NOT become a reserved keyword merely because it exists as a library function or domain capability.

Dedicated keywords are appropriate when a construct has language-level syntax or semantics.

Generic semantic operations SHOULD use compositional forms.

This is particularly important for:

- quantum gates;
- mathematical algorithms;
- AI algorithms;
- vendor APIs;
- accelerator operations;
- networking protocols;
- cryptographic algorithms.

For example, Zamani MUST NOT require one keyword for every possible quantum gate.

---

15. Identifiers

Identifiers MUST support scalable source programs without an artificial language-level maximum length.

The lexical specification determines:

- identifier start characters;
- identifier continuation characters;
- Unicode policy;
- normalization;
- reserved-word behavior.

Identifier identity MUST be deterministic.

The semantic resolver, not the lexer, determines whether an identifier refers to:

- a value;
- type;
- function;
- module;
- resource;
- capability;
- effect;
- operation;
- domain object;
- macro;
- dialect entity.

---

16. Literals

The language may provide:

- integer literals;
- floating-point literals;
- character literals;
- string literals;
- boolean literals;
- null/unit literals;
- quantum literals;
- domain-specific literals where justified.

Literal syntax MUST NOT be confused with machine-resource limits.

An implementation may reject a literal because its selected semantic type or target cannot represent it.

That is different from establishing a universal language limit.

The current AST stores integer literals as "i64" and floating-point literals as "f64". This is an implementation constraint that MUST NOT be elevated into the normative language model. The implementation should evolve toward a representation capable of preserving the full specified literal semantics before declaring arbitrary-precision or unrestricted numeric literal support stable.

---

17. Quantum Literals

Quantum state notation MAY include forms such as:

|0⟩
|1⟩
|+⟩
|-⟩

Quantum literals describe semantic quantum state information.

They MUST NOT identify physical qubits.

Future generalized quantum notation MUST be introduced through the quantum specification rather than through an ever-growing hard-coded lexer list.

---

18. Attributes

Attributes provide metadata and declarative annotations.

An attribute MUST have a declared category:

- language-semantic;
- compiler-directed;
- tooling-only;
- diagnostic-only;
- target-specific;
- dialect-specific;
- experimental.

Attributes MUST NOT become an unrestricted escape hatch around:

- type checking;
- ownership;
- resource checking;
- capability checking;
- security validation;
- portability rules.

An attribute that changes program semantics MUST have a normative specification and AST/semantic/IR mapping.

---

19. Comments and Documentation

Comments do not alter program semantics.

The language supports line and block comments according to the lexical specification.

Documentation comments MAY be preserved for:

- documentation generation;
- IDE tooling;
- source mapping;
- reflection metadata;
- diagnostics.

Documentation syntax MUST NOT silently create executable semantics.

---

20. Modules and Packages

Modules organize the program's semantic namespace.

The language MUST support arbitrary module graph size subject to actual implementation resources.

There is no language-level maximum for:

- module depth;
- number of modules;
- number of imports;
- number of exports;
- dependency graph size.

Module resolution belongs to semantic analysis and package/build infrastructure.

Imports do not imply a particular physical deployment strategy.

A module may ultimately be:

- statically linked;
- dynamically linked;
- embedded;
- remotely resolved;
- separately compiled;
- specialized;
- distributed.

---

21. Declarations

The language provides a common declaration architecture for:

- variables;
- constants;
- functions;
- types;
- structures;
- records;
- enumerations;
- classes;
- interfaces;
- traits;
- implementations;
- modules;
- resources;
- capabilities;
- effects;
- macros;
- foreign declarations;
- domain declarations;
- data declarations;
- model declarations;
- hardware declarations;
- HDL declarations;
- quantum declarations;
- dialect declarations.

Every declaration has:

syntax
AST mapping
name-resolution rules
type rules
semantic rules
IR mapping
diagnostics
tests
compatibility status

---

22. Functions

The canonical function concept uses:

fn

A function may have:

- visibility;
- modifiers;
- name;
- generic parameters;
- parameters;
- return type;
- constraints;
- effects;
- contracts;
- body.

Conceptually:

fn name<T>(parameters) -> ReturnType

Functions are semantic entities independent of their eventual ABI.

Calling conventions, ABI details, register allocation, stack layout, and target-specific calling mechanisms belong downstream.

---

23. Types

The type system is semantic and target-independent.

It may contain:

- primitive types;
- named types;
- generic types;
- tuples;
- arrays;
- slices;
- functions;
- references;
- pointers;
- optional types;
- result types;
- never/unit types;
- resource types;
- capability types;
- effectful types;
- quantum types;
- tensor types;
- temporal types;
- hardware-intent types;
- domain-specific types.

A type describes semantic properties.

It does not mandate one physical representation unless the type contract explicitly defines such a representation.

---

24. Genericity

Generic structures MUST be scalable.

The grammar MUST NOT impose a universal maximum number of type parameters.

Conceptually:

Type<T1, T2, ... Tn>

where "n" is bounded only by the actual implementation and representation resources.

The same principle applies to:

- function parameters;
- tuple members;
- fields;
- variants;
- generic constraints;
- capabilities;
- resource requirements.

---

25. Shapes, Arrays and Tensors

Array and tensor shape information may be:

- constant;
- symbolic;
- inferred;
- runtime-dependent;
- resource-dependent.

The language MUST NOT establish a universal maximum tensor rank or dimension.

Shape semantics belong to the type/semantic system.

Physical storage layout belongs to later lowering.

For example:

Tensor<f64, shape>

expresses a semantic tensor without selecting:

- CPU;
- GPU;
- TPU;
- NPU;
- accelerator;
- memory bank;
- vector width.

---

26. Memory

Memory semantics MAY describe:

- ownership;
- borrowing;
- references;
- regions;
- address spaces;
- persistence;
- shared memory;
- distributed memory;
- accelerator memory;
- quantum memory;
- resource ownership.

The language MUST NOT assume a fixed physical memory size.

The same semantic program may be realized on:

embedded device
workstation
server
cluster
cloud
accelerator
future architecture

provided the target can satisfy its requirements or an explicitly permitted execution strategy exists.

---

27. Ownership and Resource Lifetime

Resource-bearing values MUST have explicit semantic lifetime rules.

Where ownership or linear/affine semantics are used, the language MUST distinguish:

value ownership
resource ownership
capability ownership
physical ownership

These are not automatically equivalent.

A logical resource may be owned by a program while its physical realization is selected later.

---

28. Effects

Effects describe observable computational behavior that cannot be represented solely by ordinary values.

The effect system may express:

- I/O;
- mutation;
- allocation;
- concurrency;
- nondeterminism;
- network access;
- device interaction;
- quantum measurement;
- external resource use;
- security-sensitive operations;
- foreign calls.

Effects MUST be semantically declared where required.

An effect is not itself a backend implementation.

---

29. Determinism

Zamani MUST distinguish:

- deterministic semantics;
- implementation nondeterminism;
- explicitly permitted nondeterminism;
- physical randomness;
- quantum measurement randomness;
- scheduling nondeterminism.

If a program is specified as deterministic, target-specific scheduling or optimization MUST NOT change its observable semantics.

If nondeterminism is part of the program semantics, the source MUST provide sufficient semantic information for downstream systems to preserve that property.

Reproducibility mechanisms belong to compilation, provenance, runtime, and execution specifications.

---

30. Concurrency

Concurrency is a language-level semantic concept.

The language may express:

- asynchronous functions;
- tasks;
- futures;
- actors;
- channels;
- synchronization;
- parallel regions;
- data parallelism;
- task parallelism;
- pipelines;
- reductions;
- deterministic parallelism;
- distributed concurrency.

The language MUST NOT equate:

parallel

with:

exactly N hardware threads

unless "N" is explicitly part of the program's semantic requirement.

---

31. Parallelism and Scaling

A program MAY express parallel intent without specifying the number of physical execution units.

Examples include conceptual operations such as:

parallel
map
reduce
pipeline
distribute
replicate
partition

The compiler/runtime determines an implementation based on:

- available resources;
- dependencies;
- target capabilities;
- scheduling policy;
- correctness constraints.

---

32. Classical Computing

Zamani supports general classical computation.

The classical domain includes:

- scalar computation;
- integer computation;
- floating-point computation;
- arbitrary-precision numeric semantics where implemented;
- vectors;
- matrices;
- tensors;
- symbolic computation;
- statistics;
- signal processing;
- numerical methods;
- linear algebra;
- optimization;
- scientific computing;
- control computation.

Mathematical operations SHOULD be represented through a combination of:

generic language operations
+
typed libraries/intrinsics
+
semantic capabilities

rather than turning every mathematical function into a reserved keyword.

---

33. Quantum Computing

Quantum computation is a first-class Zamani domain.

Quantum syntax MAY express:

- qubit values;
- logical quantum resources;
- registers;
- states;
- operations;
- parameterized operations;
- controlled operations;
- adjoints;
- measurement;
- reset;
- barriers;
- dynamic control;
- classical feed-forward;
- observables;
- channels;
- noise intent;
- error-correction intent;
- logical operations;
- circuits;
- kernels;
- pulse-level semantic intent;
- quantum resource requirements.

Quantum syntax MUST describe quantum semantics rather than physical hardware.

---

34. Generic Quantum Operations

Zamani MUST NOT define the entire quantum language as a fixed enumeration:

H
X
Y
Z
CNOT
...

Instead, quantum operations should be represented conceptually as:

operation
+
operation name/namespace
+
parameters
+
operands
+
results
+
modifiers
+
effects
+
capabilities
+
metadata

This permits:

- standard operations;
- user-defined operations;
- composite operations;
- parameterized operations;
- future operations;
- dialect operations;
- vendor operations;
- operations unknown to the frontend but valid within a declared semantic extension.

The semantic validator determines whether an operation is valid.

---

35. Quantum IR Boundary

The canonical quantum semantic boundary is:

quantum::ir

The language frontend MUST NOT create a competing quantum IR.

The intended direction is:

Zamani source
    ↓
lexer
    ↓
parser
    ↓
domain-neutral AST
    ↓
semantic quantum model
    ↓
quantum::ir

The existing "quantum::ir" explicitly defines itself as the hardware-independent semantic boundary and separates semantic quantum meaning from physical qubits, routing, scheduling, calibration, QEC, simulation, and backend execution.

Therefore grammar-level quantum constructs MUST integrate with that existing boundary rather than invent another representation.

---

36. Quantum Physical Realization

The language MUST distinguish:

logical qubit

from:

physical qubit

Physical qubit identifiers, topology, connectivity, calibration, pulse implementation, and device selection belong downstream.

The canonical quantum IR already establishes the authoritative qubit identity architecture and prohibits duplicate "QubitId" implementations.

The grammar therefore MUST NOT introduce its own physical-qubit identity system.

---

37. Quantum Error Correction

The language MAY express intent regarding error correction and fault tolerance.

For example:

requires error_correction(...)
requires fault_tolerance(...)
requires reliability(...)
requires noise_budget(...)

The grammar does not implement QEC.

Responsibilities remain separated:

grammar
    → quantum intent

quantum::ir
    → canonical quantum semantics

QEC
    → error correction

ZQN
    → fault/noise semantics

routing
    → physical realization

scheduling
    → timing/order/resource scheduling

optimization
    → semantics-preserving transformations

HAL
    → target capability/state

No grammar feature may duplicate these responsibilities.

---

38. Hybrid Quantum-Classical Computing

Quantum and classical computation form one language.

A hybrid program may express:

classical computation
        ↓
quantum operation
        ↓
measurement
        ↓
classical decision
        ↓
quantum operation

The language MUST support explicit semantic boundaries between classical and quantum values.

A quantum measurement MAY produce classical information.

Classical control MAY depend on measurement results when the target and semantic model permit it.

---

39. Hardware Description

HDL is a first-class Zamani domain.

HDL syntax MAY express:

- modules;
- ports;
- signals;
- nets;
- registers;
- combinational logic;
- sequential logic;
- clocking;
- reset;
- timing;
- assertions;
- interfaces;
- protocols;
- state machines;
- pipelines;
- memories;
- parameterization;
- generate constructs;
- synthesis intent;
- simulation intent;
- verification intent;
- physical design intent;
- hardware/software co-design.

HDL semantics MUST describe hardware intent rather than silently selecting a particular chip.

---

40. Hardware/Software Co-Design

A Zamani program may combine:

software computation
+
accelerator intent
+
memory intent
+
communication
+
timing constraints
+
hardware modules
+
verification properties

The same semantic algorithm SHOULD be capable of being realized as:

- software;
- accelerator;
- FPGA implementation;
- ASIC-oriented hardware;
- heterogeneous computation.

Target-specific synthesis decisions belong downstream.

---

41. Hardware Capabilities

Hardware descriptions MAY express:

capability
resource
requirement
constraint
preference
topology
timing
power
thermal
reliability
calibration
deployment

The language MUST distinguish an abstract capability from a particular device.

For example:

requires capability("matrix.acceleration")

does not imply:

use GPU 0

---

42. Distributed Computing

Zamani supports distributed computation through semantic constructs for:

- processes;
- services;
- actors;
- messages;
- channels;
- communication;
- replication;
- partitioning;
- placement;
- consistency;
- transactions;
- fault tolerance;
- collective operations;
- distributed data;
- distributed resources.

There is no universal maximum number of nodes.

The language MUST NOT assume a particular network topology unless topology is explicitly part of the program's semantic requirement.

---

43. AI and Machine Learning

AI/ML is a language domain rather than a framework-specific language.

The language MAY represent:

- models;
- tensors;
- datasets;
- training;
- inference;
- optimization;
- differentiation;
- probabilistic computation;
- neural computation;
- symbolic computation;
- agents;
- model pipelines;
- distributed training;
- model deployment.

The grammar MUST NOT encode one framework's API as the universal language.

Framework-specific operations belong in libraries, dialects, interoperability layers, or backend integrations.

---

44. Data Computation

Data semantics MAY include:

- records;
- collections;
- tables;
- streams;
- schemas;
- datasets;
- tensors;
- transformations;
- queries;
- pipelines;
- serialization;
- persistence;
- provenance.

The language MUST distinguish semantic data structures from their physical storage.

For example:

dataset

does not imply:

one file
one database
one storage device

---

45. Networking

Networking syntax MAY express:

- endpoints;
- abstract addresses;
- protocols;
- channels;
- requests;
- responses;
- streaming;
- service discovery;
- routing intent;
- distributed computation;
- network capabilities.

A portable program SHOULD express communication intent rather than assuming a particular machine address.

Target-specific deployment configuration belongs outside portable semantic program logic unless explicitly declared as deployment intent.

---

46. Security

Security is a semantic domain.

The language MAY express:

- identities;
- capabilities;
- authorization;
- policies;
- trust;
- provenance;
- secrets;
- cryptographic operations;
- secure computation;
- signatures;
- hashes;
- key management;
- zero-knowledge intent.

Security annotations MUST NOT merely document security.

Where they claim semantic protection, the compiler/runtime/backend MUST provide the corresponding enforcement mechanism.

---

47. Cryptography

Cryptographic algorithms MAY be exposed through:

- standard library operations;
- capability declarations;
- cryptographic types;
- dialects;
- interoperability layers.

The grammar SHOULD NOT turn every cryptographic algorithm into a permanent reserved keyword.

Algorithm identity belongs to semantic resolution.

---

48. Interoperability

Zamani may interoperate with:

- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL formats;
- other standardized formats.

Interoperability formats are not the canonical Zamani semantic model.

The architecture is:

external format
    ↓
format frontend/importer
    ↓
Zamani semantic representation
    ↓
canonical IR

or:

Zamani semantic representation
    ↓
exporter
    ↓
external format

OpenQASM, QIR, LLVM, MLIR, vendor SDKs, and similar systems MUST NOT become a second canonical Zamani semantic authority.

---

49. Dialects

Dialects extend Zamani without creating unrelated languages.

Every dialect MUST declare:

name
version
owner
syntax extensions
semantic extensions
AST mapping
IR mapping
capabilities
compatibility
feature status
security properties

A dialect MUST NOT silently redefine core language semantics.

A dialect MUST NOT introduce target-specific syntax as though it were universally portable.

Dialect features must be explicitly identifiable.

---

50. Macros

Macros MAY operate at:

- token level;
- syntax level;
- declaration level;
- expression level;

where permitted by the language.

Macros MUST preserve:

- source provenance;
- diagnostics;
- hygiene;
- semantic validation;
- type checking;
- capability checking;
- security rules.

Macro expansion MUST NOT become an uncontrolled bypass around the language's semantic model.

---

51. Metaprogramming

Metaprogramming MAY provide:

- reflection;
- introspection;
- quotation;
- unquotation;
- compile-time computation;
- type-level computation;
- code generation;
- schema generation.

Compile-time computation MUST remain subject to defined resource and security policies.

It MUST NOT create an implicit unlimited escape from deterministic compilation requirements.

---

52. Temporal and Multi-Timeline Computation

Temporal and multi-timeline concepts may be supported as semantic constructs.

The language MUST NOT impose a fixed number of timelines.

Conceptually, supported semantics may include:

- timelines;
- temporal values;
- events;
- observations;
- speculative execution;
- branching;
- fork;
- merge;
- rollback;
- replay.

The exact semantics must be defined independently before such syntax becomes stable.

---

53. Sankofa-Oriented Semantics

Sankofa concepts present in the broader Zamani design may be represented through language-level semantic categories such as:

- remember;
- recall;
- learn;
- infer;
- wisdom;
- provenance;
- history;
- temporal knowledge;
- consensus.

The grammar MUST describe syntax only.

The parser MUST NOT become the owner of persistent memory, learning, knowledge storage, or runtime cognition.

Those belong to semantic/runtime subsystems.

---

54. Nano and Physical-Scale Domains

Nano-oriented computation may express:

- agents;
- atoms;
- molecules;
- materials;
- interactions;
- protocols;
- capabilities;
- deployment intent.

The grammar MUST NOT encode a hard-coded physical universe.

Physical constants and scientific models belong to semantic libraries and domain models.

---

55. Resource Model

Resources are abstract semantic entities.

A resource may represent:

- computation;
- memory;
- storage;
- communication;
- quantum resources;
- accelerator resources;
- energy;
- timing;
- reliability;
- bandwidth;
- capacity;
- concurrency.

A resource contract MUST distinguish:

requirement
constraint
preference
hint
capability
availability
realization

The language does not assume that all resources are physically local.

---

56. Capability Model

Capabilities describe what a target can do.

Examples:

quantum.measurement
quantum.mid_circuit_measurement
tensor.compute
parallel.execution
distributed.communication
secure.computation
hardware.synthesis

Capabilities MUST be namespaced and versionable.

A capability declaration does not itself guarantee that the current target possesses it.

Capability satisfaction is determined during semantic compilation/target analysis.

---

57. Compilation Intent

Compilation directives MAY describe:

- target independence;
- optimization goals;
- specialization;
- reproducibility;
- deterministic builds;
- caching;
- artifact generation;
- deployment requirements;
- compilation profiles.

Compilation syntax MUST distinguish:

program semantics

from:

compiler policy

Compiler policy MUST NOT silently change observable program meaning.

---

58. Execution Intent

Execution declarations MAY express:

- entry points;
- runtime policies;
- scheduling preferences;
- placement preferences;
- resilience policies;
- recovery policies;
- checkpointing;
- observability;
- tracing;
- profiling;
- lifecycle.

Execution intent MUST remain separate from physical machine topology unless explicitly target-specific.

---

59. Target Selection

A program may be compiled for a target.

The target is an external realization context.

Target selection MAY consider:

capabilities
resources
constraints
performance
energy
reliability
cost
security
availability
deployment policy

A source program MUST NOT need to be rewritten merely because the target changes.

---

60. Optimization

Optimization is semantics-preserving unless the program explicitly requests a permitted relaxation.

Optimization MAY:

- reorder independent operations;
- fuse operations;
- decompose operations;
- specialize generic operations;
- vectorize;
- parallelize;
- distribute;
- route;
- schedule;
- lower to target-specific representations.

Optimization MUST NOT become a source-language definition mechanism.

---

61. Routing

Routing determines physical realization where a domain requires it.

For quantum computation, routing may map:

logical connectivity
        ↓
physical connectivity

Routing MUST NOT modify the portable language semantics.

Routing belongs downstream of the canonical semantic representation.

---

62. Scheduling

Scheduling determines when operations execute.

Scheduling MAY consider:

- dependencies;
- resources;
- timing;
- concurrency;
- topology;
- calibration;
- target capabilities;
- reliability.

Scheduling MUST NOT define language semantics.

A scheduler MUST NOT introduce a universal maximum number of operations, qubits, threads, or devices.

---

63. Resilience

Resilience is a downstream semantic realization concern.

It may include:

- health;
- degradation;
- retry;
- recovery;
- quarantine;
- replacement;
- checkpointing;
- failover.

For quantum systems, resilience integrates with:

QEC
ZQN
HAL
routing
scheduling
optimization

The grammar exposes portable intent where necessary; the runtime/compiler implements the policy.

---

64. ZQN

ZQN remains responsible for quantum fault/noise semantics.

The language may declare noise or reliability requirements.

The grammar MUST NOT duplicate ZQN's internal fault model.

The architecture is:

Zamani source
    ↓
quantum semantics
    ↓
quantum::ir
    ↓
ZQN-aware analysis

not:

Zamani grammar
    ↓
second ZQN implementation

---

65. HAL

HAL represents target capability/state and target interaction.

The language MAY express required capabilities.

The language MUST NOT assume a particular HAL implementation.

HAL remains downstream of portable semantic compilation.

---

66. Canonical Semantic Representation

The frontend AST is a representation of source structure.

The AST MUST NOT be treated as a target IR.

The semantic pipeline is:

source
 ↓
lexer
 ↓
parser
 ↓
domain-neutral AST
 ↓
structural validation
 ↓
semantic analysis
 ↓
canonical semantic model
 ↓
canonical IR

The canonical semantic representation MUST be independent of:

- LLVM;
- MLIR;
- QIR;
- OpenQASM;
- vendor SDKs;
- physical hardware;
- routing;
- scheduling;
- QEC;
- calibration.

---

67. Existing AST Integration

The current repository AST is still an implementation surface rather than the final production semantic architecture.

The current "src/ast/mod.rs" contains generic constructs but also explicit variants such as:

QuantumOp
NanoOp
Entangle
Recall
Remember
Learn
Sasa
Unsafe

and therefore MUST be treated as an implementation that is being reconciled with the domain-neutral AST architecture rather than as the final normative definition.

The production direction is:

generic source operation
    ↓
domain-neutral AST
    ↓
semantic classification
    ↓
domain semantic model
    ↓
canonical IR

A quantum operation is therefore not required to become a parser-specific "QuantumGate" enum.

---

68. Quantum Generic Operation Contract

A generic operation SHOULD be capable of representing:

name
namespace
operands
parameters
results
attributes
modifiers
effects
capabilities
source provenance

This supports:

- standard operations;
- custom operations;
- future operations;
- dialect operations;
- vendor operations;
- composite operations.

Semantic analysis determines validity.

The AST MUST NOT need to be rewritten merely because a new quantum operation is added.

---

69. Canonical Quantum IR

The existing "src/quantum/ir/" architecture is the canonical quantum semantic boundary.

Its current architecture explicitly separates:

- quantum semantics;
- classical semantics;
- control;
- models;
- program structure;
- pulse semantics;
- resources;
- scheduling;
- metadata;
- analysis;
- validation;
- dialects;
- compatibility.

The language specification therefore requires grammar integration with these existing ownership boundaries rather than creating duplicate representations.

---

70. IR Independence

The canonical IR MUST NOT depend on:

- source grammar;
- source parser;
- vendor SDKs;
- credentials;
- filesystem execution;
- network clients;
- physical backend implementations.

The source frontend depends conceptually on the IR contract.

The IR does not depend on the source frontend.

---

71. Resource Limits Versus Language Limits

A compiler invocation MAY impose explicit resource/security limits.

For example:

compiler policy:
maximum compilation memory = available budget

or:

security policy:
maximum allowed IR expansion = policy value

These are invocation policies, not Zamani language limits.

This distinction is mandatory.

The existing quantum IR already distinguishes explicit "QuantumIrLimits" from universal IR semantics.

---

72. Resource Feasibility

A program may be semantically valid but resource-infeasible on a particular target.

For example:

program requires 1000 logical qubits
target provides insufficient resources

The correct outcome is a resource feasibility diagnostic.

The compiler MUST NOT reinterpret the program as:

program requires 100 qubits

merely because the target provides 100.

Possible permitted alternatives include:

- another target;
- distribution;
- simulation;
- decomposition;
- sequentialization;
- logical encoding;

but only when semantics remain preserved or the program explicitly permits approximation.

---

73. Approximation

Approximation MUST be explicit.

An implementation MUST NOT silently replace exact semantics with approximate semantics merely to fit a target.

Approximation policies, when supported, must specify:

- allowed error;
- metric;
- scope;
- provenance;
- reproducibility;
- target policy;
- diagnostics.

---

74. Diagnostics

Every invalid program condition MUST produce a structured diagnostic where possible.

Diagnostics SHOULD include:

diagnostic code
severity
message
primary span
secondary spans
source context
related declarations
suggested correction where reliable
feature/version information

Diagnostics MUST distinguish:

- lexical error;
- syntax error;
- name-resolution error;
- type error;
- effect error;
- resource error;
- capability error;
- portability error;
- security error;
- target incompatibility;
- runtime availability.

---

75. Error Recovery

The parser MAY recover from malformed input to continue diagnostics.

Recovery MUST NOT cause invalid source to be reported as valid.

A recovered parse tree MUST retain sufficient error/provenance information to prevent downstream systems from treating an incomplete construct as a valid semantic construct.

---

76. Security Model

The language specification MUST treat security as a first-class concern.

The compiler MUST NOT use source parsing as a security boundary.

Security-sensitive operations require:

- explicit semantic definition;
- capability checks;
- authorization where appropriate;
- diagnostics;
- provenance;
- runtime enforcement where required.

---

77. Rust Implementation Safety

The production Zamani implementation baseline is:

Rust 2021
Rust 1.97
Rust 1.97.1
stable Rust
safe Rust

Production compiler code MUST NOT use:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

The implementation should enforce this using repository-level linting such as:

#![forbid(unsafe_code)]

where applicable.

The Rust safety rule is distinct from the existence of an "unsafe" keyword in Zamani source.

---

78. Zamani Source-Level "unsafe"

The current implementation contains an "unsafe" lexical/parser concept. The current parser explicitly has an "unsafe" parsing path, and the lexer contains "KeywordUnsafe".

This does not establish that unrestricted unsafe execution is part of the final language.

If source-level "unsafe" remains, it MUST have:

- normative syntax;
- semantic rules;
- capability rules;
- security model;
- compiler behavior;
- diagnostics;
- tests;
- compatibility policy.

If Zamani becomes fully safe-by-default without an unrestricted unsafe execution model, the existing construct MUST eventually be classified as:

DEPRECATED

or another explicit compatibility status.

It MUST NOT remain as undocumented accidental syntax.

---

79. Deterministic Compilation

Where deterministic compilation is requested, the compiler MUST make all relevant decisions reproducible.

Determinism covers:

- source parsing;
- name resolution;
- semantic analysis;
- IR construction;
- canonical serialization;
- hashing;
- optimization ordering;
- artifact generation.

Any intentionally nondeterministic optimization MUST be identified and controlled when reproducibility is required.

---

80. Provenance

Every significant transformation SHOULD retain provenance.

The conceptual chain is:

source span
    ↓
AST node
    ↓
semantic entity
    ↓
IR entity
    ↓
optimized entity
    ↓
lowered entity
    ↓
target artifact

This enables:

- debugging;
- reproducibility;
- verification;
- auditing;
- diagnostics;
- scientific provenance;
- security analysis.

---

81. Versioning

Zamani language versions are independent of:

- compiler versions;
- backend versions;
- hardware generations;
- dialect versions.

A language-version change MUST identify:

- added syntax;
- removed syntax;
- changed semantics;
- changed diagnostics;
- compatibility impact;
- migration strategy.

---

82. Compatibility

Backward compatibility MUST be deliberate.

Existing syntax may be:

STABLE
IMPLEMENTED
EXPERIMENTAL
DEPRECATED
HISTORICAL

Deprecated features require:

- documented replacement;
- migration guidance;
- compatibility period;
- tests;
- removal policy.

Historical content does not become legal merely because it exists in "Zamani-Grammar.md".

---

83. Dialect Compatibility

Dialect compatibility requires explicit:

dialect name
version
language version
semantic version
IR compatibility
feature compatibility
migration rules

A dialect MUST NOT silently change the meaning of core Zamani syntax.

---

84. Inter-Domain Composition

The following domains MUST be composable:

classical
quantum
hybrid
HDL
hardware
AI
data
distributed
networking
security
scientific
embedded
accelerator
temporal
nano

For example:

AI model
    ↓
tensor computation
    ↓
distributed execution
    ↓
accelerator
    ↓
quantum subcomputation
    ↓
measurement
    ↓
classical control

must remain one semantic program.

---

85. Domain Boundaries

Each domain owns only its semantic constructs.

For example:

quantum/
    quantum language constructs

hardware/
    hardware capability/intent

resources/
    resource semantics

compile/
    compilation intent

execution/
    runtime/execution intent

distributed/
    distributed semantics

No domain may duplicate:

- the global type system;
- source spans;
- module resolution;
- resource identity;
- canonical IR ownership.

---

86. Mathematical and Scientific Computation

Scientific computation is a language domain but SHOULD rely heavily on generic semantic abstractions.

The language may support:

- vectors;
- matrices;
- tensors;
- symbolic expressions;
- differentiation;
- integration;
- transforms;
- statistics;
- optimization;
- numerical methods.

A mathematical algorithm should generally be represented through:

typed operation
+
generic expression
+
library/intrinsic capability

rather than requiring a permanent keyword.

---

87. Hardware Topology

Topology may be expressed when it is semantically relevant.

Examples include:

- graph connectivity;
- communication requirements;
- locality;
- adjacency;
- latency;
- bandwidth.

But topology MUST NOT become a fixed universal machine model.

The compiler may discover or negotiate actual topology.

---

88. Deployment

Deployment intent may specify:

- locality;
- replication;
- availability;
- region;
- security policy;
- resource requirements;
- communication requirements;
- target class.

Portable source SHOULD avoid physical identifiers.

Deployment configuration may provide target-specific realization externally.

---

89. Embedded Computing

Embedded programs are valid Zamani programs.

The language MUST NOT require a separate embedded language.

Embedded-specific constraints may be represented as:

resource requirements
capabilities
timing constraints
memory constraints
power constraints
device interfaces

The same semantic model remains applicable to larger targets.

---

90. Future Computational Paradigms

The language architecture MUST permit future computational domains without requiring a rewrite of the core grammar.

A future domain should be able to integrate through:

domain contract
+
syntax extension
+
AST mapping
+
semantic model
+
IR mapping
+
capability/resource model
+
compiler integration
+
tests

The core language therefore remains stable while computational technology evolves.

---

91. Feature Independence

Every production language feature MUST be independently completable.

A feature contract must identify in advance:

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
Diagnostics
Security
Performance
Hard-Coding Audit
Completion Criteria

A feature is complete only when these contracts are known.

---

92. No Deferred Integration

A grammar feature MUST NOT be accepted with an unspecified downstream mapping.

The following is insufficient:

grammar rule added

The required contract is:

grammar rule
    ↓
AST node
    ↓
semantic entity
    ↓
IR entity
    ↓
compiler consumer
    ↓
runtime/backend consumer
    ↓
tests

This is specifically intended to prevent repeated re-editing of earlier files after later architectural decisions.

---

93. AST Contract

Every stable grammar construct MUST have an AST representation.

The AST representation MUST:

- preserve source structure;
- preserve source spans;
- preserve names;
- preserve attributes;
- preserve modifiers;
- preserve operands;
- preserve parameters;
- preserve relevant provenance.

The AST MUST NOT depend on target-specific realization.

---

94. Semantic Contract

Every AST construct that has meaning MUST have semantic rules defining:

- validity;
- typing;
- ownership;
- effects;
- resource requirements;
- capabilities;
- constraints;
- determinism;
- security;
- diagnostics.

No syntax-only feature may be declared stable.

---

95. IR Contract

Every stable semantic construct that reaches compiled execution MUST have a defined IR mapping.

The IR mapping must specify:

- canonical owner;
- inputs;
- outputs;
- invariants;
- provenance;
- resource information;
- compatibility.

Quantum constructs map toward:

quantum::ir

rather than a frontend-specific quantum IR.

---

96. Compiler Contract

Compiler integration defines:

- lowering;
- optimization;
- specialization;
- target selection;
- resource feasibility;
- routing;
- scheduling;
- resilience;
- backend selection.

The compiler MUST preserve semantics.

---

97. Runtime Contract

Runtime integration defines:

- resource acquisition;
- execution;
- scheduling;
- observability;
- failure;
- recovery;
- lifecycle;
- target interaction.

The runtime MUST NOT become a parser or language-specification authority.

---

98. Tooling Contract

Tooling includes:

- formatter;
- syntax highlighter;
- IDE integration;
- diagnostics;
- documentation;
- language server;
- grammar validation;
- conformance testing.

Tooling MUST consume the canonical language specification/grammar rather than defining independent syntax.

---

99. Testing Contract

Every stable feature requires:

positive tests
negative tests
boundary tests
scalability tests
compatibility tests
diagnostic tests
determinism tests where applicable

Domain-specific tests MUST exist for:

- classical;
- quantum;
- hybrid;
- HDL;
- hardware;
- distributed;
- AI;
- data;
- networking;
- security;
- interoperability.

---

100. Scalability Testing

Scalability tests MUST verify absence of artificial language limits.

Tests should vary:

- collection size;
- module count;
- expression depth;
- generic parameter count;
- tensor dimensions;
- quantum resource count;
- operation count;
- distributed node count;
- concurrency;
- data volume;
- hardware resource descriptions.

The purpose is not to prove literal mathematical infinity.

The purpose is to prove that no artificial fixed ceiling is encoded by the language.

---

101. Boundary Testing

Boundary tests MUST include:

- empty program;
- minimal valid program;
- maximal parser nesting supported by configured resources;
- very large identifiers;
- large literals;
- large generic structures;
- large expressions;
- large resource declarations;
- large quantum programs;
- large HDL descriptions;
- large distributed graphs.

Failures must be classified as:

language semantic restriction
representation limitation
compiler resource limitation
runtime resource limitation
target limitation

and must never be mislabeled.

---

102. Hard-Coding Audit

Production validation MUST search for accidental universal limits.

Suspicious constructs include:

MAX_QUBITS
MAX_CPUS
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_TIMELINES
MAX_AGENTS
MAX_DEVICES

and finite resource enumerations such as:

Qubit0
Qubit1
...

The audit MUST distinguish legitimate:

program constants
test fixtures
target profiles
security policies
compiler budgets

from prohibited universal language restrictions.

---

103. Resource Negotiation

When a program declares portable resource requirements, the compiler/runtime MAY negotiate realization.

Conceptually:

program requirement
        ↓
available target capabilities
        ↓
resource feasibility
        ↓
realization strategy

A negotiation failure MUST be explicit.

The system MUST NOT silently lower the program's semantic requirements.

---

104. Target Independence

The language is target-independent by default.

Target-specific constructs require explicit target/deployment context.

A target-specific construct MUST NOT contaminate the portable semantic model.

This allows:

same source
    ↓
CPU
GPU
FPGA
QPU
cluster
cloud
embedded system
future target

without rewriting the program's algorithm.

---

105. Representation Limits

No language can guarantee physically infinite representation.

Therefore:

«"Unbounded" or "infinite" in Zamani architecture means absence of an artificial language-level finite ceiling.»

Concrete implementation limits may arise from:

- available memory;
- address space;
- compiler resource budget;
- integer representation;
- file size;
- target resources;
- operating-system limits;
- physical constraints.

Such limits MUST be identified as implementation/target limitations rather than semantic language limits.

---

106. Canonical Compiler Architecture

The complete architecture is:

                       Zamani Source
                              │
                              ▼
                     Source Map / File ID
                              │
                              ▼
                           Lexer
                              │
                              ▼
                        Token Stream
                              │
                              ▼
                           Parser
                              │
                              ▼
                       Domain-Neutral AST
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
       Name Resolution   Type Analysis   Effect Analysis
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                 Resource/Capability Analysis
                              │
                              ▼
                    Semantic Validation
                              │
                              ▼
                 Canonical Semantic Model
                              │
             ┌────────────────┼─────────────────┐
             ▼                ▼                 ▼
        Classical         Quantum             HDL/
        Semantics        Semantics         Hardware Intent
             │                │                 │
             └────────────────┼─────────────────┘
                              ▼
                       Canonical IR
                              │
                              ▼
                         Optimization
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
          Routing         Scheduling       Resilience
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                             ZQN
                              │
                              ▼
                             HAL
                              │
                              ▼
                       Target Lowering
                              │
          ┌───────────┬──────┼──────┬────────────┐
          ▼           ▼      ▼      ▼            ▼
         CPU         GPU    FPGA    QPU       Future
                                             targets

The grammar exists at the top of this pipeline.

It does not own the lower layers.

---

107. Existing Repository Integration Map

The language specification integrates with the existing repository approximately as follows:

Layer| Authority
Language architecture| "grammar/DESIGN.md"
Language semantics| "grammar/specification/"
Formal contracts| "grammar/spec/"
ANTLR syntax| "grammar/Zamani.g4"
Implementation reference| "grammar/grammar.md"
Historical/proposed language material| "grammar/Zamani-Grammar.md"
Lexing| "src/lexer.rs"
Parsing| "src/parser.rs"
Source AST| "src/ast/"
Semantic analysis| semantic-analysis subsystem
IR generation| "src/ir_gen.rs"
IR validation| "src/ir_verify.rs"
Canonical quantum semantics| "src/quantum/ir/"
Quantum optimization| quantum optimization subsystem
Routing| quantum routing subsystem
Scheduling| scheduling subsystem
QEC| QEC subsystem
ZQN| ZQN subsystem
HAL| hardware-abstraction subsystem
Runtime| runtime subsystem

The existing repository already establishes "quantum::ir" as the canonical hardware-independent quantum semantic boundary.

---

108. Current Implementation Reconciliation

The current parser already supports a broad set of constructs including:

- functions;
- structures;
- enums;
- traits;
- implementations;
- classes;
- interfaces;
- modules;
- imports;
- quantum circuits;
- noise models;
- surface-code constructs;
- nano/agent constructs;
- Sankofa constructs;
- effects;
- handlers;
- type aliases;
- "unsafe";
- advanced/omniversal constructs.

This specification does not automatically declare every such parser path stable.

Each must be reconciled against:

normative syntax
AST
semantics
IR
compiler
runtime
tests
compatibility

The implementation-conformance status belongs in "grammar/grammar.md".

---

109. Existing Lexer Reconciliation

The current lexer contains a very broad token inventory, including:

- core language keywords;
- OOP keywords;
- quantum keywords;
- nano keywords;
- Sankofa keywords;
- temporal concepts;
- mathematical symbols;
- advanced-system keywords;
- additional expansion concepts.

The production lexical specification MUST classify these into:

stable keyword
stable token
contextual keyword
experimental keyword
dialect keyword
deprecated keyword
historical token
implementation-only token

A token MUST NOT remain permanently reserved merely because it once appeared in a design document.

---

110. Existing AST Reconciliation

The current AST is broad and carries source spans, which is a useful foundation.

However, production architecture requires that source structure remain domain-neutral.

The migration direction is:

generic AST structure
        ↓
semantic classification
        ↓
domain semantic model
        ↓
canonical IR

rather than:

one AST enum variant
        ↓
one backend-specific representation

This is especially important for quantum, hardware, AI, networking, and future domains.

---

111. No Duplicate Quantum IR

The following architecture is prohibited:

Zamani Quantum AST
        ↓
Zamani Quantum IR A
        ↓
quantum::ir B

The required architecture is:

Zamani AST
        ↓
semantic quantum classification
        ↓
quantum::ir

This maintains one canonical quantum semantic boundary.

---

112. No Backend Leakage

The language specification MUST NOT depend directly on:

- LLVM;
- MLIR;
- QIR;
- CUDA;
- ROCm;
- vendor FPGA languages;
- vendor QPU languages;
- vendor CPU instructions.

These may be downstream implementation targets or interoperability formats.

---

113. No Physical Topology Leakage

The source language MUST NOT require physical topology unless topology itself is explicitly part of program semantics.

Examples of prohibited accidental coupling:

qubit 0 connected to qubit 1
GPU 0
core 7
node 12
memory bank 3

Portable alternatives express:

requires connectivity(...)
requires locality(...)
requires bandwidth(...)
requires capability(...)

and allow downstream realization.

---

114. No Fixed Accelerator Model

Zamani MUST NOT assume that an accelerator is necessarily:

GPU
TPU
NPU
FPGA
QPU

The semantic model uses:

accelerator capability

and allows future accelerator types.

---

115. No Fixed Quantum Model

Quantum computation MUST NOT be reduced to gate circuits.

The language architecture must permit semantic models including, where implemented:

- gate circuits;
- dynamic circuits;
- measurement-based computation;
- Hamiltonian models;
- analog computation;
- annealing;
- QUBO;
- continuous-variable computation;
- logical computation;
- tensor-network computation;
- distributed quantum computation;
- pulse-level semantic intent.

The existing quantum IR architecture explicitly provides model, program, pulse, classical, control, resources, and quantum subsystems to support this breadth.

---

116. Hardware-Independent Pulse Semantics

Pulse-level syntax MAY exist.

Pulse semantics MUST describe:

- waveform intent;
- frames;
- ports;
- timing;
- calibration intent;
- capture;
- control semantics.

It MUST NOT directly encode a vendor's physical control API into the core language.

The canonical pulse semantic layer remains under "quantum::ir::pulse".

---

117. Canonical Ownership Rule

Every semantic concept MUST have exactly one canonical owner.

Examples:

source syntax
    → grammar

tokenization
    → lexer

source structure
    → AST

semantic validity
    → semantic analysis

quantum semantics
    → quantum::ir

physical quantum mapping
    → routing

timing
    → scheduling

error correction
    → QEC

fault/noise semantics
    → ZQN

target capability/state
    → HAL

execution
    → runtime

Compatibility aliases MAY exist.

Duplicate implementations MUST NOT.

---

118. File Integration Contract

Every new specification or grammar file MUST declare in its header or associated contract:

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
Diagnostics
Security
Hard-Coding Audit
Completion Criteria

This is required to make each file independently completable.

---

119. Feature Completion

A feature is "STABLE" only when:

- normative semantics exist;
- lexical rules exist;
- syntax exists;
- AST mapping exists;
- semantic mapping exists;
- IR mapping exists where required;
- compiler integration exists;
- runtime/backend integration exists where required;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- compatibility tests exist;
- diagnostics are tested;
- determinism is tested where applicable;
- security is addressed;
- hard-coding audit passes.

---

120. Production Acceptance Criteria

"grammar/" is production-ready only when:

1. There is one normative language authority.
2. "Zamani.g4" is the canonical ANTLR composition root.
3. "grammar.md" is implementation-conformance documentation.
4. "Zamani-Grammar.md" cannot silently define syntax.
5. The lexical model is canonical.
6. Token duplication is resolved.
7. AST contracts exist.
8. Semantic contracts exist.
9. IR contracts exist.
10. Domain boundaries are explicit.
11. "quantum::ir" remains the canonical quantum semantic boundary.
12. No duplicate quantum IR exists.
13. No universal hardware limits exist.
14. Resource requirements are separate from realization.
15. Capabilities are separate from physical devices.
16. Requirements, constraints, preferences, and hints are distinct.
17. Classical and quantum semantics coexist in one language.
18. HDL and software co-design coexist in one language.
19. AI/data/distributed/network/security domains use common language foundations.
20. Dialects are explicitly versioned.
21. Interoperability formats do not become language authorities.
22. Source provenance exists.
23. Diagnostics are structured.
24. Compatibility is explicit.
25. Scalability tests exist.
26. Negative tests exist.
27. Boundary tests exist.
28. Hard-coding audits exist.
29. Rust 1.97/1.97.1 compatibility is maintained.
30. Production Rust contains no "unsafe".
31. Stable features have complete downstream integration.
32. Generated documentation does not become a competing authority.

---

121. Final Language Principle

Zamani MUST describe:

WHAT the computation means
WHAT must be true
WHAT capabilities are required
WHAT resources are required
WHAT constraints apply
WHAT properties are preferred
WHAT correctness guarantees exist

and defer:

WHERE it runs
WHEN it runs
HOW it is physically realized
WHICH CPU is selected
WHICH GPU is selected
WHICH FPGA is selected
WHICH QPU is selected
WHICH physical qubits are selected
WHICH memory bank is selected
WHICH node is selected
WHICH instruction set is selected

to later compilation and execution layers.

Therefore the fundamental architecture is:

                 ZAMANI SOURCE
                       │
                       ▼
             LANGUAGE SPECIFICATION
                       │
                       ▼
                  ZAMANI.G4
                       │
                       ▼
                    LEXER
                       │
                       ▼
                    PARSER
                       │
                       ▼
             DOMAIN-NEUTRAL AST
                       │
                       ▼
             SEMANTIC ANALYSIS
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
       CLASSICAL     QUANTUM       HDL
          │            │            │
          └────────────┼────────────┘
                       ▼
              CANONICAL SEMANTICS
                       │
                       ▼
                  CANONICAL IR
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
      OPTIMIZATION   ROUTING    SCHEDULING
          │            │            │
          └────────────┼────────────┘
                       ▼
                    RESILIENCE
                       │
                       ▼
                      ZQN
                       │
                       ▼
                      HAL
                       │
                       ▼
               TARGET REALIZATION
                       │
        ┌──────────────┼──────────────┐
        ▼              ▼              ▼
       CPU            GPU            QPU
        │              │              │
       FPGA        DISTRIBUTED      FUTURE
        │              │           TARGETS
        └──────────────┴──────────────┘
                       │
                       ▼
                    RUNTIME

The governing invariant is:

«Zamani source expresses portable computational intent; downstream systems determine realization.»

The governing scalability invariant is:

«No artificial finite machine-size limit may be encoded as a universal property of the language.»

The governing quantum invariant is:

«"quantum::ir" is the canonical quantum semantic boundary; grammar and frontend code must not create a competing quantum IR.»

The governing safety invariant is:

«The production Zamani compiler is implemented in Rust 1.97/1.97.1, Rust 2021, using safe Rust only.»

The governing compatibility invariant is:

«Existing filenames and established architecture are preserved unless a demonstrated technical reason requires change; extensions are integrated into the existing authority model rather than creating parallel languages.»

The governing completion invariant is:

«No language feature is complete until its specification, syntax, AST, semantic, IR, compiler, runtime, tooling, testing, scalability, compatibility, and hard-coding contracts are known and satisfied.»

This is the normative foundation on which the remaining "grammar/" files can be completed independently without later architectural rework.