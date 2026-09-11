

Zamani Language Principles

Path: "grammar/specification/language-principles.md"
Status: Normative
Scope: Zamani language and grammar architecture
Minimum implementation toolchain: Rust 1.97 / Rust 1.97.1
Rust safety requirement: "unsafe" Rust is forbidden
Primary compiler: ZUTC
Primary portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the fundamental principles that govern the Zamani programming language.

These principles apply to:

- source syntax;
- lexical design;
- parsing;
- AST construction;
- semantic analysis;
- type checking;
- effect checking;
- capability checking;
- resource requirements;
- classical computation;
- quantum computation;
- hybrid computation;
- HDL;
- hardware/software co-design;
- distributed computation;
- parallel computation;
- AI and data computation;
- networking;
- security;
- compilation;
- execution;
- interoperability;
- future language extensions.

This document is normative.

A grammar rule, AST representation, semantic rule, compiler transformation, runtime interface, or backend implementation must not contradict these principles unless the language specification is explicitly versioned and the change is intentionally approved.

The fundamental objective is:

«Zamani describes computation and its meaning independently of the accidental limitations of the machine on which that computation happens to execute.»

The resulting language must support:

«Zamani: From Atom to Everywhere»

and:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).»

---

2. Fundamental Language Principle

A Zamani program represents computation, intent, semantics, requirements, constraints, capabilities, and permitted effects.

It does not primarily represent:

- a particular CPU;
- a particular QPU;
- a particular GPU;
- a particular FPGA;
- a particular ASIC;
- a particular machine topology;
- a particular memory capacity;
- a particular number of qubits;
- a particular number of cores;
- a particular deployment;
- a particular vendor implementation.

Machine-specific information belongs to target, resource, compilation, scheduling, deployment, calibration, or runtime layers unless that information is itself part of the program's intended semantics.

The distinction is mandatory:

Program semantics
        │
        ▼
Target-independent semantic representation
        │
        ▼
Resource/capability analysis
        │
        ▼
Target selection
        │
        ▼
Optimization
        │
        ▼
Scheduling / routing / placement
        │
        ▼
Target lowering
        │
        ▼
Execution

The source language must not collapse these layers into one.

---

3. POCO-REAF

POCO-REAF is a core Zamani design principle.

3.1 Program Once

A developer should express an algorithm, system, computation, circuit, hardware design, or hybrid computation once.

The semantic meaning of the source must not need to be rewritten merely because execution moves between different resource configurations.

Examples include:

one qubit → many logical qubits
one core → many cores
one device → many devices
one accelerator → many accelerators
one node → many nodes
local → distributed
CPU → GPU
CPU → FPGA
classical → hybrid quantum-classical
simulator → physical quantum hardware

The source may contain explicit semantic requirements when those requirements are genuinely part of the program.

It must not contain accidental assumptions merely because one development machine happened to have a particular configuration.

---

3.2 Compile Once

Compilation must preserve the program's semantic identity.

Where technically possible, the compiler should establish a target-independent compiled representation that can subsequently be specialized for available execution resources.

Compilation must therefore distinguish:

semantic compilation

from:

target realization

A target-specific artifact may legitimately exist, but target specialization must not redefine the source program's meaning.

The compiler architecture must not force source-level rewriting merely because target resources differ.

---

3.3 Run Everywhere

The same semantic program must be capable of being realized on different supported execution environments.

The environment may provide different:

- resources;
- capabilities;
- performance;
- topology;
- instruction sets;
- native operations;
- memory;
- timing;
- energy budgets;
- reliability;
- quantum hardware characteristics.

These differences must be handled through the compilation and execution architecture.

---

3.4 Run Anywhere

Execution may occur:

- locally;
- remotely;
- embedded;
- on a workstation;
- on a server;
- on a cluster;
- on a supercomputer;
- in a cloud environment;
- on an accelerator;
- on a simulator;
- on quantum hardware;
- on heterogeneous systems;
- on future execution systems.

The grammar must not assume a single deployment model.

---

3.5 Run Forever

"Forever" does not mean that every historical binary remains executable on every future machine without adaptation.

It means that the language semantics must remain evolvable and representable as execution technology changes.

Future targets must be able to implement existing semantic programs through new lowering mechanisms without requiring the language to encode every future machine in advance.

Therefore:

Stable semantics
+
Versioned language evolution
+
Extensible capabilities
+
Target-independent representations
=
Long-lived Zamani programs

---

4. Semantic Independence From Hardware

The language must distinguish:

Semantic requirement

What the program means.

Resource requirement

What resources are necessary to execute the program.

Capability requirement

What capabilities an execution environment must provide.

Constraint

What must remain true during compilation or execution.

Preference

What implementation is preferred but not required.

Hint

Information intended to assist optimization without changing semantics.

Target description

A description of a concrete target.

Runtime observation

A property discovered from the actual execution environment.

These concepts must never be silently conflated.

For example:

requires quantum

does not mean:

use device X

and:

requires at least N logical qubits

does not mean:

hardware must contain exactly N physical qubits

Likewise:

prefer GPU

does not mean:

the program is invalid without a GPU

---

5. No Accidental Hard-Coding

Zamani must not encode arbitrary scalability limits into language syntax or semantic structures.

The following must not be fixed by grammar design:

- maximum qubits;
- maximum logical qubits;
- maximum physical qubits;
- maximum CPU count;
- maximum core count;
- maximum thread count;
- maximum GPU count;
- maximum FPGA count;
- maximum accelerator count;
- maximum node count;
- maximum cluster size;
- maximum memory capacity;
- maximum register count;
- maximum vector width;
- maximum tensor dimension;
- maximum tensor rank;
- maximum network size;
- maximum device count;
- maximum resource identifiers;
- fixed hardware addresses;
- fixed topology sizes.

A program may explicitly contain a number because that number is part of its algorithm.

For example:

register[1024]

is not an artificial compiler limit if "1024" is semantically required by the program.

The compiler must not infer from this:

register[1024]

that:

register[1025]

is syntactically or semantically impossible.

Resource availability belongs to later stages.

---

6. Scaling Principle

Zamani must scale from the smallest useful computation to arbitrarily large computations subject only to:

- available resources;
- representational limits;
- explicit resource budgets;
- implementation-defined operational limits that are documented and not falsely represented as language limits.

The architecture must avoid unnecessary constants whose purpose is merely to make implementation convenient.

Examples of prohibited architecture:

const MAX_QUBITS: usize = 32;
const MAX_CORES: usize = 128;
const MAX_DEVICES: usize = 16;

when those values represent supposed language limits.

Implementation-specific limits may exist where unavoidable, but they must be:

1. outside language semantics;
2. explicitly documented;
3. represented as resource/implementation limits;
4. diagnostically reported;
5. replaceable without changing source semantics.

---

7. Separation of Syntax and Semantics

Grammar answers:

«What source forms are structurally valid?»

Semantic analysis answers:

«What does the valid source mean, and is that meaning valid?»

Type analysis answers:

«Are the values and operations type-compatible?»

Effect analysis answers:

«What effects may this computation perform?»

Capability analysis answers:

«What execution capabilities are required?»

Resource analysis answers:

«What resources are required?»

Compilation answers:

«How can the semantics be transformed into executable representations?»

Scheduling answers:

«When and where should operations execute?»

Backend lowering answers:

«How is the semantic computation represented on this target?»

Runtime answers:

«How is the resulting computation actually executed?»

These responsibilities must remain distinct.

---

8. Grammar Must Not Become the IR

The grammar is not an intermediate representation.

The grammar defines source syntax.

The AST represents parsed source structure.

Semantic structures represent validated meaning.

IR represents canonical executable semantics.

Backends represent target realizations.

The architecture is:

Zamani source
     │
     ▼
Lexer
     │
     ▼
Parser
     │
     ▼
AST
     │
     ▼
Name/module resolution
     │
     ▼
Type/effect/capability/resource analysis
     │
     ▼
Canonical semantic IR
     │
     ├──────── classical IR
     │
     ├──────── quantum::ir
     │
     ├──────── control/data representations
     │
     └──────── effect/resource/temporal metadata
     │
     ▼
Optimization
     │
     ▼
Scheduling / routing / resilience / ZQN
     │
     ▼
Target lowering
     │
     ▼
Execution

The grammar must not create a second semantic universe that competes with the canonical IR.

---

9. Canonical Quantum Boundary

Quantum computing is a first-class Zamani domain.

However, grammar/frontend structures must not become a competing quantum IR.

The canonical quantum semantic boundary is:

crate::quantum::ir

The repository already contains structured "quantum::ir" modules covering quantum program and semantic concerns, including qubit identities and physical qubit identities.

Quantum source syntax must therefore follow:

Quantum syntax
      ↓
Frontend AST
      ↓
Semantic validation
      ↓
quantum::ir
      ↓
Optimization
      ↓
Routing
      ↓
Scheduling
      ↓
ZQN / resilience / calibration
      ↓
Target lowering

The grammar must not introduce an alternative canonical quantum operation model.

---

10. Quantum Hardware Independence

Quantum syntax must not assume:

- a fixed number of qubits;
- a fixed native gate set;
- a fixed coupling topology;
- a fixed measurement model;
- a fixed calibration;
- a fixed pulse representation;
- a fixed error model;
- a fixed QPU vendor;
- a fixed physical qubit numbering scheme.

The language may express these properties when they are explicitly part of a program's requirements.

For example:

requires:
    logical_qubits >= N

is fundamentally different from:

use_physical_qubit(7)

The latter is a target-specific statement and must therefore remain within the appropriate target/hardware boundary unless physical identity itself is deliberately part of the program's semantics.

---

11. Quantum Operation Principle

Quantum operations must be represented according to semantic meaning rather than an arbitrary finite keyword catalog.

The language must be capable of representing:

- standard gates;
- user-defined operations;
- parameterized operations;
- controlled operations;
- multi-target operations;
- inverse/adjoint operations;
- measurement;
- reset;
- observables;
- dynamic circuits;
- mid-circuit measurement;
- classical feedback;
- logical operations;
- error-correction operations;
- future quantum operations.

A backend may have a finite native gate set.

That is a backend property.

The source language must not become limited to that native set.

The conceptual pipeline is:

Portable quantum operation
        ↓
Canonical quantum semantic representation
        ↓
Decomposition
        ↓
Target capability analysis
        ↓
Routing
        ↓
Scheduling
        ↓
Calibration/noise-aware realization
        ↓
Native target operations

---

12. Classical Computing Principle

Classical computation is a first-class foundation of Zamani.

The language must support, subject to implementation of each feature:

- scalar values;
- structured values;
- functions;
- generic programming;
- algebraic data;
- control flow;
- memory management;
- ownership/resource semantics;
- concurrency;
- parallelism;
- numerical computation;
- symbolic computation;
- vectors;
- matrices;
- tensors;
- accelerator computation;
- systems programming.

Classical computation must not be designed as a subsystem that prevents quantum, HDL, hardware, distributed, or future domains from interoperating with it.

---

13. Hybrid Computing Principle

Classical and quantum computation must be able to coexist in one semantic program.

Hybrid computation includes:

classical → quantum
quantum → classical
classical control → quantum operation
quantum measurement → classical control
classical optimization → quantum execution
quantum result → classical post-processing

The language must not require developers to split a single semantic algorithm into unrelated languages merely because it crosses computational domains.

Domain boundaries must be represented explicitly in semantics and effects rather than through incompatible language silos.

---

14. HDL and Hardware Principle

Zamani must support hardware description and hardware/software co-design.

Hardware syntax may describe:

- modules;
- ports;
- signals;
- wires;
- registers;
- clocks;
- timing;
- combinational logic;
- sequential logic;
- processes;
- state machines;
- memories;
- pipelines;
- interfaces;
- parameters;
- generics;
- hardware resources;
- hardware capabilities;
- target requirements.

However:

«Hardware semantics are not the same thing as a particular physical implementation.»

The source may describe:

a pipeline

without requiring a particular FPGA family.

It may describe:

parallel computation

without requiring a fixed number of physical execution units.

Target-specific implementation belongs to hardware lowering.

---

15. Hardware/Software Co-Design

A Zamani program may contain both:

software semantics

and:

hardware semantics

where required.

The compiler must preserve the distinction.

For example:

algorithm
    ↓
compute requirement
    ↓
hardware realization

is different from:

algorithm = one specific FPGA netlist

unless the latter is intentionally expressed as a target-specific artifact.

This distinction allows one semantic design to be realized across different hardware targets.

---

16. Resource Principle

Zamani must treat resources as dynamically describable properties.

Resources may include:

- computation;
- memory;
- storage;
- bandwidth;
- latency;
- energy;
- qubits;
- logical qubits;
- physical qubits;
- classical processors;
- accelerators;
- devices;
- network links;
- execution time;
- reliability;
- thermal constraints;
- power;
- capacity.

Resource expressions must be scalable.

Resource descriptions must not require a fixed machine model.

---

17. Capability Principle

A capability describes what an execution environment can do.

Examples include:

supports quantum computation
supports dynamic circuits
supports floating-point operation X
supports accelerator execution
supports hardware feature Y
supports required network protocol
supports a particular cryptographic primitive

Capabilities must not be confused with resources.

For example:

supports quantum computation

is a capability.

has N physical qubits

is a resource.

The compiler must be able to reason about both independently.

---

18. Constraint Principle

Constraints describe conditions that must be satisfied.

Examples:

latency <= L
energy <= E
reliability >= R
requires capability C
requires resource R

A constraint is not automatically an instruction to use a particular implementation.

The compiler may satisfy the same constraint through different realizations.

---

19. Preference Principle

Preferences are optional optimization guidance.

Examples:

prefer_gpu
prefer_low_latency
prefer_energy_efficiency
prefer_local_execution
prefer_parallelism

Failure to satisfy a preference must not automatically make a program semantically invalid.

Preferences must not be silently promoted to hard requirements.

---

20. Hint Principle

Hints provide optimization information without defining semantic meaning.

A hint must not change the observable semantics of a valid program merely because a compiler chooses to ignore it.

This distinction is especially important for:

- scheduling;
- vectorization;
- parallelization;
- quantum routing;
- accelerator selection;
- memory placement;
- distributed placement.

---

21. Effect Principle

Effects describe computational behavior that is not adequately represented by pure value transformation.

Potential effect domains include:

- IO;
- hardware;
- quantum;
- network;
- distributed execution;
- concurrency;
- security;
- external state;
- persistent state;
- timing;
- resource consumption.

Effects must be semantically explicit.

The compiler must not infer arbitrary hidden effects merely because a parser recognized a keyword.

---

22. Safety Principle

Zamani language safety and Rust implementation safety are distinct concepts.

The Zamani language may eventually define constructs for controlled low-level operations where those operations are semantically necessary.

That does not permit the Zamani compiler implementation to use Rust "unsafe".

All Rust implementation code in the relevant production toolchain must remain safe Rust.

The implementation must target:

Rust 1.97 / Rust 1.97.1

and must not require:

unsafe { ... }

or:

unsafe fn ...

or:

unsafe trait ...

or unsafe implementations.

Where low-level behavior is required, it must be represented through safe abstractions or explicitly isolated external interfaces whose safety contract is established outside the Rust implementation.

---

23. Memory and Resource Safety

Memory/resource semantics must be explicit where required.

The language may evolve support for:

- ownership;
- borrowing;
- lifetimes;
- linear resources;
- affine resources;
- explicit allocation;
- explicit deallocation;
- shared resources;
- distributed resources.

These concepts must have one canonical semantic interpretation.

Surface syntax must not create duplicate semantic concepts merely because multiple domains need resource ownership.

---

24. Concurrency Principle

Concurrency must be expressed independently of the number of available execution units.

The language must not assume:

exactly N threads

or:

exactly N cores

A program expresses concurrency semantics.

The runtime/compiler determines how that concurrency is realized given available resources.

Potential execution strategies include:

single-threaded
multithreaded
SIMD
GPU
distributed
accelerator
heterogeneous

without requiring source rewriting.

---

25. Parallelism Principle

Parallelism is a semantic opportunity, not a fixed hardware count.

The source may express:

- independent tasks;
- data parallelism;
- task parallelism;
- reductions;
- pipelines;
- synchronization;
- distributed computation.

The compiler and runtime determine feasible execution width.

A program that can execute on one execution unit must remain semantically meaningful when more resources become available, subject to its synchronization and determinism requirements.

---

26. Determinism Principle

For identical:

- source;
- language version;
- compilation configuration;
- relevant semantic inputs;

the frontend must produce deterministic results.

Parsing must not depend on:

- machine topology;
- available CPU count;
- available GPU count;
- available qubits;
- random runtime state;
- filesystem ordering;
- network timing.

Any deliberate nondeterminism must be represented explicitly at the semantic/runtime layer.

---

27. Source Stability

Source syntax should remain stable whenever possible.

A language change must not silently change the meaning of an existing valid program.

Breaking changes require:

- version identification;
- compatibility documentation;
- migration rules;
- diagnostics;
- explicit deprecation policy where applicable.

The following must remain distinct:

language evolution

and:

target evolution

A new hardware generation must not require a new source language version merely because its implementation differs.

---

28. Versioning Principle

Language versions describe language semantics and syntax.

Target versions describe target capabilities.

Backend versions describe implementation behavior.

Runtime versions describe execution infrastructure.

These version domains must not be conflated.

The language must be capable of expressing:

language version
+
program semantics
+
required capabilities
+
resource constraints
+
target-independent intent

without embedding a complete target specification into every source file.

---

29. Extensibility Principle

Zamani must support future computing models without requiring the grammar architecture to be redesigned every time a new domain appears.

Future domains may include:

- new quantum models;
- new accelerators;
- new neuromorphic architectures;
- optical computing;
- molecular computing;
- biological computing;
- new distributed execution models;
- future hardware architectures;
- future mathematical abstractions;
- future AI execution models.

The language should therefore provide extension mechanisms based on:

- types;
- capabilities;
- effects;
- resources;
- attributes;
- dialects;
- semantic operations;
- modules;
- versioned extensions.

New domain functionality should not require uncontrolled growth of core keywords.

---

30. Keyword Discipline

A concept should become a reserved keyword only when lexical reservation provides genuine language-level value.

Domain operations that can be represented as:

identifier + type + semantic resolution

should not automatically become reserved keywords.

This is especially important for:

- quantum gates;
- mathematical functions;
- accelerator operations;
- hardware primitives;
- vendor operations.

A finite keyword catalog must not become an artificial ceiling on the language.

---

31. Mathematical Principle

Mathematics is a fundamental Zamani capability.

The language may support:

- scalar mathematics;
- vectors;
- matrices;
- tensors;
- symbolic mathematics;
- calculus;
- probability;
- statistics;
- optimization;
- signal processing;
- numerical methods;
- scientific computing.

However, mathematics should primarily use:

types
+
generic operations
+
functions
+
intrinsics
+
libraries
+
semantic interfaces

rather than turning every mathematical function into a permanent grammar keyword.

The existing broad "Zamani.g4" mathematics surface must therefore be reconciled with this principle rather than expanded indefinitely by keyword accumulation.

---

32. AI and Data Principle

AI and data computation must be expressed through composable language abstractions.

The language must be able to represent, as implementations mature:

- tensors;
- datasets;
- models;
- training;
- inference;
- automatic differentiation;
- agents;
- pipelines;
- accelerators;
- distributed data processing.

AI constructs must interoperate with classical, quantum, hardware, networking, and distributed semantics.

AI must not become a separate language embedded inside Zamani.

---

33. Distributed Computing Principle

Distributed computation must be independent of a fixed node count.

The language may express:

- nodes;
- tasks;
- services;
- messages;
- channels;
- replication;
- consistency;
- fault tolerance;
- remote execution;
- placement.

However, the source must not assume a fixed cluster size unless cluster size is explicitly part of program semantics.

The compiler/runtime must negotiate actual placement with available resources.

---

34. Networking Principle

Networking constructs must represent communication semantics rather than hard-coded network topology.

A program may require:

reliable communication

without specifying:

node A → node B → node C

unless that topology is semantically necessary.

Network-specific realization belongs to networking/runtime/deployment layers.

---

35. Security Principle

Security is part of program semantics where security properties affect correctness.

The language must support future semantic representation of:

- permissions;
- capabilities;
- identity;
- cryptographic operations;
- privacy requirements;
- trust requirements;
- security constraints.

Security semantics must not be reduced to arbitrary backend configuration.

---

36. Interoperability Principle

Zamani must interoperate with external ecosystems where necessary.

Potential boundaries include:

- C;
- C++;
- Python;
- OpenQASM;
- Verilog;
- other HDL representations;
- system interfaces;
- foreign ABIs.

Interoperability syntax describes a boundary.

It must not leak foreign language semantics into the Zamani core unless deliberately adopted.

External representations must lower into Zamani's canonical semantic model.

---

37. AST Completeness Principle

Every accepted source construct must have an AST representation that preserves all semantically relevant information.

The AST must not:

- discard meaningful attributes;
- discard source locations needed for diagnostics;
- collapse distinct semantic constructs accidentally;
- encode target-specific behavior prematurely;
- lose resource/capability information.

If a grammar construct has no complete semantic representation, it must not be presented as production-ready.

---

38. Semantic Completeness Principle

A syntactically valid construct is not automatically semantically valid.

The implementation must distinguish:

recognized syntax

from:

implemented semantics

and:

implemented target support

A feature must not be documented as fully supported merely because its parser recognizes its syntax.

The repository's current grammar reference already identifies examples where syntax is planned or partially implemented, such as the declared-but-not-yet-emitted MTS literal.

Such distinctions must remain explicit.

---

39. Diagnostics Principle

Invalid programs must produce structured diagnostics rather than crashes.

Diagnostics should provide, where available:

- source location;
- source span;
- error code;
- severity;
- message;
- relevant context;
- expected syntax;
- actual syntax;
- semantic explanation;
- suggested correction where reliable.

The compiler must not silently ignore unsupported syntax.

---

40. Error Recovery Principle

The lexer/parser must:

- avoid infinite loops;
- make progress after recoverable errors;
- preserve useful source positions;
- report multiple independent errors where safe;
- avoid silently interpreting invalid source as a different valid program.

Error recovery must never alter the semantics of successfully parsed regions without explicit diagnostics.

---

41. Deep-Structure Principle

Zamani must be able to represent deeply nested programs subject to available resources rather than arbitrary small parser constants.

Implementation may use:

- iterative parsing strategies;
- explicit worklists;
- explicit stacks;
- bounded diagnostic resources;
- configurable operational budgets.

Such implementation mechanisms must not become language-level semantic restrictions.

---

42. Resource Exhaustion Principle

"Unlimited" in the language architecture means:

«No artificial fixed machine-size ceiling is encoded into language semantics.»

It does not mean that physical machines have infinite resources.

Execution can legitimately fail because:

- memory is unavailable;
- execution time exceeds a configured budget;
- a target lacks a required capability;
- required hardware is unavailable;
- an explicitly requested resource exceeds the environment;
- a representation cannot be materialized within available resources.

Such failures must be represented as resource/target/execution failures, not as evidence that the language itself has an arbitrary fixed limit.

---

43. Compilation Principle

Compilation must preserve semantic equivalence.

A transformation is valid only if the resulting representation preserves the defined observable behavior of the source program.

Optimization must not:

- change quantum measurement semantics;
- violate synchronization;
- violate resource ownership;
- violate effect constraints;
- violate security properties;
- silently remove required operations;
- silently replace unsupported operations with different computations.

Target-specific optimization is permitted only after semantic meaning has been established.

---

44. Optimization Principle

Optimization may exploit available resources.

For example:

more cores
more memory
GPU
QPU
accelerator
parallel hardware

may permit better execution.

However, optimization must not require the source program to be rewritten merely because the available resources change.

This is central to POCO-REAF.

---

45. Scheduling Principle

Scheduling is downstream of semantic representation.

The scheduler determines:

- ordering;
- timing;
- placement;
- resource use;
- synchronization;
- target-specific execution arrangements.

Scheduling must not redefine program semantics.

Quantum scheduling must consume canonical "quantum::ir" rather than create a competing semantic representation.

The repository already separates "quantum::ir" from scheduling infrastructure, including dedicated scheduling/IR boundaries.

---

46. ZQN and Noise Principle

ZQN is a target/execution concern, not a replacement for the language's quantum semantics.

The language may express noise-related requirements where those requirements are semantically meaningful.

Noise models, calibration data, execution conditions, and noise-aware transformations belong downstream.

ZQN must consume canonical quantum semantics and relevant target/resource information rather than become a second language-level quantum model.

The repository's existing ZQN architecture already keeps canonical quantum resource identities in "quantum::ir", reinforcing this separation.

---

47. Error-Correction Principle

Quantum error correction may be represented at language and semantic levels where necessary.

However, the distinction must remain clear between:

logical quantum semantics

and:

physical error-correction implementation

The source may specify logical requirements.

Target-specific physical encoding, syndrome scheduling, calibration, and hardware realization belong to downstream layers.

---

48. Target Independence

The source language must not require knowledge of every target that may exist in the future.

A new backend should be able to implement existing semantic operations by defining:

- capabilities;
- lowering rules;
- resource mappings;
- scheduling;
- target constraints.

The grammar should not need modification merely because a new target backend is added.

This is a key production criterion.

---

49. Target-Specific Extensions

Target-specific constructs are permitted only when target specificity is genuinely part of the requested semantics.

They must be isolated through:

- target modules;
- dialects;
- explicit annotations;
- capability declarations;
- interoperability boundaries;
- target specifications.

A target extension must not silently modify core Zamani semantics.

---

50. Dialect Principle

Dialect extensions must be:

- namespaced;
- versioned;
- capability-aware;
- explicitly registered;
- compatible with the core semantic model.

A dialect may extend syntax, but it must eventually map to canonical semantics.

A dialect must not redefine the meaning of existing core constructs without explicit language-version rules.

---

51. Future-Proofing Principle

Future-proofing must come from abstraction rather than speculative syntax.

Do not create syntax solely because a hypothetical future machine might need it.

Instead provide extensible semantic mechanisms for:

- new types;
- new operations;
- new capabilities;
- new resources;
- new effects;
- new targets;
- new dialects.

This minimizes language churn.

---

52. One Meaning Principle

A Zamani construct must have one canonical semantic meaning.

Different surface forms may exist as syntactic sugar.

For example:

surface syntax A
surface syntax B

may both lower to:

canonical semantic construct

They must not create two unrelated semantic concepts.

---

53. No Duplicate Domain Models

A concept must have one owner.

Examples:

Qubit identity
    → quantum::ir

Quantum operation semantics
    → canonical quantum semantic representation

Hardware capability
    → hardware/capability model

Resource requirement
    → resource model

Scheduling
    → scheduling subsystem

Noise model
    → ZQN

Calibration
    → calibration subsystem

Grammar syntax references these concepts but must not duplicate their authoritative implementations.

---

54. Repository Integration Principle

The grammar work must be performed against the entire repository.

At minimum, integration analysis must cover:

- "grammar/Zamani.g4";
- "grammar/grammar.md";
- "grammar/Zamani-Grammar.md";
- "grammar/DESIGN.md";
- lexer implementation;
- parser implementation;
- AST;
- semantic analysis;
- IR generation;
- "quantum::ir";
- quantum optimization;
- quantum scheduling;
- QEC/resilience;
- ZQN;
- hardware;
- calibration;
- resource management;
- benchmarking;
- tests;
- examples;
- compiler tooling.

The grammar must expand existing repository functionality rather than create isolated replacements.

---

55. Grammar Authority Principle

The repository must ultimately have one normative language authority.

The architecture must distinguish:

Normative specification

Defines what Zamani means.

Canonical grammar

Defines the formal syntax corresponding to that specification.

Reference implementation

Implements the grammar and semantics.

Implementation snapshot

Documents what the current implementation actually accepts.

Historical/aspirational documentation

Records designs that have not yet been promoted into the language.

The current repository contains multiple grammar surfaces, including "grammar/Zamani.g4", "grammar/grammar.md", and "grammar/Zamani-Grammar.md".

They must not remain independent authorities.

---

56. Specification-to-Implementation Principle

Every production language feature must have a traceable path:

Specification
     ↓
Grammar
     ↓
Lexer/parser
     ↓
AST
     ↓
Semantic analysis
     ↓
Canonical representation
     ↓
Compiler
     ↓
Tests
     ↓
Documentation

A feature missing one of these necessary layers is incomplete.

---

57. Implementation Independence Principle

Each grammar/specification component should be completable against predefined contracts.

Before implementing a grammar file, its:

- inputs;
- outputs;
- dependencies;
- ownership;
- non-ownership;
- AST contract;
- semantic contract;
- IR contract;
- downstream consumers;
- tests;
- compatibility rules

must already be known.

This prevents:

finish file A
→ implement file B
→ discover A was architecturally wrong
→ rewrite A

The intended workflow is:

architecture
→ contracts
→ dependency order
→ implementation
→ validation
→ completion

---

58. Dependency Direction Principle

The architecture must maintain a one-way dependency direction.

The intended conceptual direction is:

Specification
      ↓
Grammar
      ↓
Frontend
      ↓
AST
      ↓
Semantic analysis
      ↓
Canonical IR
      ↓
Optimization
      ↓
Scheduling
      ↓
Hardware/runtime

Not:

grammar ↔ runtime
grammar ↔ hardware
grammar ↔ quantum backend
grammar ↔ scheduler

Circular dependencies must be avoided.

---

59. Quantum Dependency Direction

Quantum syntax must not depend on a particular backend.

Correct:

quantum grammar
      ↓
quantum AST
      ↓
quantum semantic analysis
      ↓
quantum::ir
      ↓
backend

Incorrect:

quantum grammar
      ↓
IBM-like backend
      ↓
quantum grammar

or:

quantum grammar
      ↓
native gate set

as the semantic authority.

---

60. Hardware Dependency Direction

Hardware syntax may describe hardware semantics.

It must not require a specific hardware backend merely to parse.

Correct:

HDL grammar
    ↓
hardware AST
    ↓
hardware semantics
    ↓
hardware IR / lowering
    ↓
FPGA / ASIC / simulator / future target

---

61. Testing Principle

Every language feature must have tests at the appropriate layers.

Tests must include:

Positive tests

Valid programs.

Negative tests

Invalid syntax.

Semantic-negative tests

Syntactically valid but semantically invalid programs.

Boundary tests

Very small and very large valid structures.

Cross-domain tests

Examples combining domains.

Compatibility tests

Programs from supported language versions.

Determinism tests

Repeated compilation/parsing produces stable results.

Round-trip tests

Where a canonical printer exists:

source
→ lexer
→ parser
→ AST
→ printer
→ parser

must preserve intended semantics.

---

62. Cross-Domain Principle

The language must support composition of domains rather than isolated domain silos.

Important integration combinations include:

classical + quantum
classical + HDL
classical + hardware
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

Cross-domain semantics must be defined before individual domain implementations are declared complete.

---

63. Portability Principle

Portability means preserving program meaning, not pretending all targets are identical.

Different targets may have:

- different performance;
- different precision;
- different capabilities;
- different timing;
- different memory;
- different native operations;
- different quantum connectivity;
- different reliability.

The compiler must account for those differences while preserving semantics wherever the target is capable of implementing the required program.

---

64. Graceful Capability Failure

If a target cannot satisfy a program's semantic requirements, the compiler/runtime must produce a structured failure.

It must not:

- silently change the algorithm;
- silently remove required operations;
- silently change precision;
- silently reduce qubit count;
- silently ignore hardware requirements.

The failure must identify the unsatisfied requirement or capability.

---

65. Explicit Specialization

A programmer may intentionally specialize a program for a target.

Such specialization must be explicit.

For example:

generic semantic program

and:

target-specific implementation

must be distinguishable.

Specialization must not contaminate the portable semantic core unnecessarily.

---

66. Compile-Time and Runtime Information

Compile-time information includes:

- source syntax;
- static types;
- statically known constants;
- compile-time constraints;
- compile-time capabilities.

Runtime information may include:

- available memory;
- available devices;
- runtime topology;
- runtime calibration;
- runtime load;
- runtime capabilities.

The language architecture must not force runtime information into compile-time syntax when it is inherently dynamic.

---

67. Reflection and Discovery

Where runtime discovery is needed, the language may expose controlled mechanisms for discovering:

- capabilities;
- resources;
- target properties;
- execution state.

Discovery must not turn runtime-specific information into permanent source-level assumptions.

---

68. Resource Negotiation

For portable execution, resource requirements may need negotiation.

The conceptual model is:

program requirement
        ↓
available capabilities
        ↓
available resources
        ↓
constraint solving
        ↓
feasible execution plan

The grammar expresses the requirement.

The resource/compiler/runtime layers determine feasibility.

---

69. Observability Principle

Resource and target adaptation must not silently change observable semantics.

For example, increasing available parallel resources may improve performance.

It must not change the result unless the language explicitly defines nondeterminism.

Likewise, changing QPU hardware must not change logical program semantics merely because native gate decomposition differs.

---

70. Temporal Principle

Where Zamani represents time, temporal semantics must be distinct from wall-clock implementation details.

A semantic timestamp, duration, ordering requirement, or temporal relation must not automatically imply:

- a particular processor clock;
- a particular timer implementation;
- a particular physical clock frequency.

Temporal semantics are portable.

Physical timing is target-specific unless explicitly part of the program's meaning.

---

71. Precision Principle

Numerical precision must be represented semantically.

The language must distinguish:

required precision

from:

target implementation precision

A backend may use a different representation only when the semantic correctness requirements permit it.

---

72. Numeric Scalability

Numerical literals and data structures must not contain hidden machine-width assumptions.

The grammar must support representations that can be semantically validated against types.

Overflow must result in a diagnostic where the language requires a value that cannot be represented.

The lexer/parser must not silently truncate values merely to fit host-machine types.

---

73. Tensor and Shape Principle

Tensor shapes may be:

- statically known;
- dynamically known;
- symbolic;
- generic;
- runtime-derived.

A grammar must not impose fixed tensor dimensions merely because a backend implementation has fixed hardware dimensions.

Shape requirements belong to type/semantic/resource analysis.

---

74. Generic Programming Principle

Generic constructs must describe algorithms over abstractions rather than machine sizes.

For example:

fn transform<T>(...)

may operate over many types.

Likewise, generic resource-aware computations should be able to adapt to available scale without requiring duplicated source programs.

---

75. Library Versus Language Principle

Not every useful operation belongs in core syntax.

Prefer:

core language
+
standard library
+
semantic intrinsics
+
domain libraries

over:

ever-growing keyword catalog

A feature belongs in the core language when it affects fundamental semantics, syntax, typing, effects, resource semantics, or compilation behavior.

---

76. Macro and Metaprogramming Principle

Metaprogramming must not allow arbitrary syntax to bypass semantic invariants.

Generated syntax must still pass through:

- parsing;
- validation;
- semantic analysis;
- type checking;
- effect checking;
- capability/resource validation.

Macros must not become an escape hatch from language correctness.

---

77. Security of Compilation

Compilation must treat source input as untrusted data.

The frontend must not execute arbitrary source-controlled behavior merely because syntax was parsed.

Compile-time execution must have an explicit semantic boundary and controlled resource model.

---

78. Reproducibility Principle

Where deterministic compilation is required, compilation must be reproducible with the same:

- source;
- language version;
- relevant compiler version;
- compilation configuration;
- semantic inputs.

Machine-specific resource discovery must not unexpectedly alter semantic output.

Target-specific artifacts may differ when target selection is intentionally different.

---

79. Documentation Principle

Documentation must distinguish:

implemented

from:

specified

from:

experimental

from:

planned

from:

deprecated

from:

historical

No aspirational construct may be described as production functionality until its implementation and semantic contracts are complete.

---

80. Compatibility Principle

Backward compatibility should be preserved whenever possible.

When compatibility cannot be maintained:

1. identify the breaking change;
2. identify affected constructs;
3. provide diagnostics;
4. provide migration guidance;
5. update compatibility tests;
6. version the language rule.

Compatibility must be deliberate rather than accidental.

---

81. Reserved Space Principle

The language should reserve namespace and syntax space for future evolution without pretending that unimplemented constructs already exist.

Reserved constructs must be explicitly documented.

A reserved keyword or syntax form must not be silently interpreted as a different semantic feature.

---

82. Repository Consistency Principle

The repository must not contain contradictory definitions of the same language construct.

If:

Zamani.g4

and:

grammar/grammar.md

disagree, the discrepancy must be resolved explicitly.

The implementation snapshot must not silently become a second language.

The repository's existing "grammar/DESIGN.md" already establishes this authority problem and calls for reconciliation among the grammar/specification surfaces.

---

83. Production Readiness Principle

A feature is production-ready only when all applicable layers are complete:

Specification
+
Grammar
+
Lexer
+
Parser
+
AST
+
Semantic analysis
+
Type/effect/capability analysis
+
Canonical IR
+
Compiler integration
+
Runtime/target integration
+
Tests
+
Documentation

Parser recognition alone is not production readiness.

---

84. File Ownership Principle

Every grammar/specification file must have explicit ownership.

For each file, the project must be able to answer:

What does this file define?
What does it not define?
What consumes it?
What may depend on it?
What must never depend on it?
What tests prove it correct?
What completion criteria make it finished?

A file must not depend on an implementation decision that has not already been specified by its upstream contract.

---

85. No Re-Edit Completion Principle

A completed file must be complete against its predefined integration contract.

If another downstream file later requires fundamental changes to an already completed file, that indicates a missing architectural contract rather than normal implementation flow.

The correct sequence is:

define contracts
      ↓
resolve dependencies
      ↓
implement independent foundations
      ↓
implement dependents
      ↓
integrate
      ↓
validate

not:

implement everything
      ↓
discover architecture conflicts
      ↓
rewrite everything

---

86. Independent-First Implementation Principle

Implementation should begin with the least dependent artifacts.

The broad dependency order is:

language principles
        ↓
language scope/version/authority
        ↓
syntax model
        ↓
semantic model
        ↓
lexical foundation
        ↓
core names and source units
        ↓
types
        ↓
expressions
        ↓
statements
        ↓
declarations
        ↓
functions
        ↓
modules
        ↓
effects/capabilities
        ↓
memory/concurrency
        ↓
classical computation
        ↓
quantum computation
        ↓
hybrid computation
        ↓
HDL
        ↓
hardware
        ↓
distributed computing
        ↓
AI/data
        ↓
networking/security
        ↓
resource model
        ↓
compilation
        ↓
execution
        ↓
interoperability
        ↓
dialects/macros/metaprogramming
        ↓
validation
        ↓
cross-domain tests

Actual implementation order must be reconciled with repository dependencies before coding.

---

87. Rust Implementation Principle

The grammar specification itself is language-independent, but the repository implementation governed by this specification must target:

Rust 1.97 / Rust 1.97.1

and:

unsafe Rust forbidden

Production code must prefer:

- safe ownership;
- explicit data structures;
- checked conversions;
- structured errors;
- deterministic algorithms;
- explicit resource budgets;
- safe concurrency;
- iterative algorithms where deep nesting threatens stack exhaustion.

---

88. No Host-Machine Leakage

The compiler must not accidentally make the host machine part of language semantics.

Examples of prohibited leakage include:

host pointer width determines Zamani integer syntax
host CPU count determines legal parallelism
host memory determines valid source syntax
host GPU presence determines whether syntax parses
host QPU presence determines whether a quantum program parses

The host environment may affect compilation feasibility, but not the definition of the language.

---

89. Compiler Resource Limits

Compiler implementation may have configurable limits for operational safety.

Such limits must be:

- explicit;
- documented;
- diagnosable;
- configurable where practical;
- separate from language semantics.

For example:

compiler diagnostic budget
compiler memory budget
compiler execution budget

must not be confused with:

maximum legal Zamani program size

unless the language specification deliberately defines such a semantic limit.

---

90. Runtime Resource Limits

Runtime limits are execution concerns.

A runtime may report:

insufficient memory
insufficient qubits
insufficient accelerator capacity
insufficient execution time
unsupported capability

without implying that the language itself has those fixed limits.

---

91. Semantic Portability

A program is portable when its semantics can be preserved across compatible targets.

Portability does not require identical implementation.

For example:

same algorithm
    ↓
CPU implementation
GPU implementation
FPGA implementation
QPU implementation
distributed implementation

may use radically different implementations while preserving the same intended semantics.

---

92. Future Hardware Principle

Future hardware must be treated as an extension of the target space rather than a reason to redesign the language core.

The language should be able to represent future execution capabilities through:

new capability
new resource
new semantic operation
new dialect
new lowering

where appropriate.

The core language should not attempt to enumerate every future architecture.

---

93. Atomic-to-Universal Principle

"From atom to everywhere" means that the semantic architecture must work at radically different scales.

At the smallest scale:

single operation
single value
single device

At larger scales:

parallel program
heterogeneous system
distributed system
quantum/classical system
cluster
cloud
future execution environment

The same semantic architecture must remain valid across these scales.

---

94. Composition Principle

Every major language domain must be composable.

A domain should not assume that it is the only computation occurring in a program.

For example:

quantum operation
+
classical computation
+
network communication
+
resource constraint
+
security requirement

must be representable as one coherent semantic program.

---

95. Canonical Identity Principle

Every semantic entity that crosses subsystem boundaries must have a stable canonical identity.

This is particularly important for:

- qubits;
- classical values;
- resources;
- operations;
- modules;
- types;
- capabilities;
- devices;
- source entities.

Frontend-local identifiers must not become competing identities once a canonical subsystem owns the entity.

The existing quantum architecture explicitly treats canonical quantum identifiers as belonging to "quantum::ir"; downstream systems such as ZQN must consume those identities rather than redefine them.

---

96. Observable Semantics Principle

The specification must define what is observable.

Compilation transformations are permitted only when they preserve required observable behavior.

Observable behavior may include:

- returned values;
- state changes;
- IO;
- communication;
- measurement results;
- ordering;
- synchronization;
- security guarantees;
- explicitly specified timing/resource behavior.

An optimization that changes a required observable behavior is invalid even if it appears faster.

---

97. Quantum Observability

Quantum programs require particular care because measurement changes what is observable.

The compiler must preserve the defined semantics of:

- state preparation;
- operations;
- measurement;
- classical control;
- probabilities;
- observables;
- reset;
- dynamic circuits.

A backend transformation must not alter the defined quantum computation merely because a different native implementation is available.

---

98. Hardware Observability

Hardware descriptions may have observable properties including:

- signal behavior;
- timing;
- clock relationships;
- state transitions;
- interface protocols.

Hardware optimizations must preserve the specified hardware semantics.

Physical implementation details may vary where the specification permits variation.

---

99. Explicit Undefined/Unspecified Behavior

The language specification must explicitly distinguish:

defined behavior

from:

implementation-defined behavior

and:

unspecified behavior

and:

invalid program

No compiler implementation may accidentally convert unspecified behavior into an undocumented permanent language guarantee.

---

100. Principle for Standard Library Growth

As Zamani expands, functionality should preferentially be added through:

standard libraries
semantic intrinsics
generic abstractions
domain libraries
dialects

rather than through permanent expansion of the core grammar.

The core grammar should remain conceptually compact even if Zamani's capabilities become enormous.

---

101. Principle for Examples

Examples must demonstrate semantic portability.

Production examples should include:

- minimal classical program;
- generic computation;
- scalable data processing;
- concurrent computation;
- distributed computation;
- quantum program;
- hybrid quantum-classical program;
- HDL program;
- hardware/software co-design;
- accelerator program;
- AI/data program;
- cross-domain program;
- POCO-REAF program.

Examples must not accidentally establish implementation limits.

---

102. Principle for Validation

Validation must operate at multiple levels:

lexical validation
syntax validation
AST validation
name resolution
type validation
effect validation
capability validation
resource validation
semantic validation
IR validation
target validation
runtime validation

A parser must not attempt to perform all of these responsibilities.

---

103. Principle for Diagnostics Across Layers

Errors should identify the layer responsible.

Examples:

syntax error
type error
effect error
capability error
resource error
quantum semantic error
hardware semantic error
target incompatibility
runtime resource exhaustion

This prevents target limitations from being misreported as language limitations.

---

104. Principle for Repository Evolution

When an existing feature is changed:

1. Identify the existing syntax.
2. Identify its implementation.
3. Identify consumers.
4. Identify its intended semantics.
5. Determine whether it is correct.
6. Preserve it if valid.
7. Migrate it if necessary.
8. Deprecate it when appropriate.
9. Remove it only with explicit compatibility justification.
10. Update all affected tests and documentation.

No valid existing feature should disappear silently.

---

105. Principle for Existing Grammar Reconciliation

The current broad ANTLR grammar and the current implementation grammar must be reconciled.

The repository currently has:

grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md

with materially different roles and maturity levels.

Therefore:

- "Zamani.g4" must not silently claim syntax that the implementation cannot support;
- "grammar.md" must not become a permanent competing language authority;
- aspirational constructs must be clearly marked;
- implemented constructs must be promoted through the specification process;
- tests must establish actual support.

---

106. Principle for "grammar/DESIGN.md"

"grammar/DESIGN.md" provides the repository's current production architecture for the relationship among:

- grammar;
- lexer;
- parser;
- AST;
- semantic analysis;
- IR;
- optimization;
- scheduling;
- ZQN;
- hardware;
- runtime.

This document complements that architecture by defining the language principles that those implementation layers must obey.

Where this document defines a semantic principle, downstream implementation documents must conform to it.

Where implementation details change, this document should remain stable unless the underlying language principle itself changes.

---

107. Principle for "grammar/grammar.md"

"grammar/grammar.md" is an implementation-conformance artifact.

It must describe what the current implementation actually accepts.

It must not be used to silently introduce new language semantics.

When implementation changes:

specification
→ grammar
→ implementation
→ tests
→ grammar.md

must remain synchronized.

---

108. Principle for "grammar/Zamani.g4"

"grammar/Zamani.g4" is the canonical ANTLR grammar representation after reconciliation.

It must:

- implement the normative syntax;
- avoid semantic actions that create hidden compiler state;
- avoid target-specific assumptions;
- avoid hard-coded resource limits;
- remain deterministic;
- preserve source structure needed by downstream AST construction;
- remain compatible with the repository's Rust frontend architecture.

ANTLR grammar structure must not become a second semantic implementation.

---

109. Principle for Specification Subdocuments

The following documents under "grammar/specification/" refine this document:

language-scope.md
language-version.md
compatibility.md
grammar-authority.md
syntax-model.md
semantic-model.md
compilation-model.md
execution-model.md
scalability-model.md
poco-reaf.md
extensibility.md
reserved-space.md

They must not contradict these principles.

Their responsibilities are:

language-principles.md
    → why and what the language fundamentally guarantees

language-scope.md
    → what domains belong to the language

language-version.md
    → language versioning

compatibility.md
    → compatibility guarantees

grammar-authority.md
    → source-of-truth rules

syntax-model.md
    → formal syntax architecture

semantic-model.md
    → meaning of constructs

compilation-model.md
    → compilation stages

execution-model.md
    → execution semantics

scalability-model.md
    → resource-independent scaling

poco-reaf.md
    → formal POCO-REAF contract

extensibility.md
    → future evolution

reserved-space.md
    → intentionally reserved syntax/semantic space

---

110. Principle for Cross-Repository Integration

This document governs grammar decisions, but it must integrate with the repository rather than duplicate repository architecture.

The key boundaries are:

grammar
  → lexer/parser

lexer/parser
  → AST

AST
  → semantic analysis

semantic analysis
  → canonical IR

quantum semantics
  → quantum::ir

quantum::ir
  → optimization/routing/scheduling/resilience/ZQN

hardware semantics
  → hardware abstraction/lowering

resource semantics
  → resource management

execution semantics
  → runtime

target semantics
  → backend

The grammar must not invert these dependencies.

---

111. Principle for No Hidden Semantic State

Parsing must not mutate hidden runtime state.

Syntax such as:

remember
recall
learn
infer
quantum
nano
zamani
sasa

must produce explicit semantic structures.

The parser must not secretly execute:

- memory operations;
- quantum operations;
- hardware operations;
- network operations;
- AI inference;
- temporal state transitions.

Those belong to later semantic/runtime layers.

---

112. Principle for Domain-Neutral Core

The core language should remain domain-neutral enough to support multiple computation models.

The same core should be able to express:

values
types
functions
control flow
resources
effects
capabilities
modules
composition

and then compose those with:

classical
quantum
hardware
distributed
AI
data
network
security
future domains

without forcing the core language to become a collection of unrelated domain-specific languages.

---

113. Principle for Semantic Lowering

Surface syntax may be expressive.

Lowering must converge toward canonical semantic concepts.

For example:

surface quantum syntax
        ↓
quantum semantic operation
        ↓
quantum::ir

and:

surface hardware syntax
        ↓
hardware semantic representation
        ↓
hardware lowering

and:

surface resource requirement
        ↓
resource semantic representation
        ↓
resource analysis

The source syntax must not determine the eventual target representation prematurely.

---

114. Principle for Backend Freedom

Backends are free to choose implementation strategies provided they preserve semantics.

A CPU backend may use:

- scalar execution;
- SIMD;
- multithreading.

A GPU backend may use:

- kernels;
- vectorization;
- device memory.

A quantum backend may use:

- native gates;
- decomposition;
- routing;
- pulse schedules.

An FPGA backend may use:

- synthesized logic;
- pipelines;
- memory structures.

The source program must not have to know which strategy was selected unless the strategy itself is part of the program's explicit semantics.

---

115. Principle for Resource Availability

Available resources determine feasibility and optimization.

They do not redefine the language.

Therefore:

small machine

may execute a smaller feasible workload,

while:

large machine

may execute a larger workload,

without either machine changing the definition of the source language.

---

116. Principle for "Infinity"

The phrase "infinity" in Zamani's scalability objective is a semantic design goal, not a claim that physical machines have infinite resources.

The correct formal interpretation is:

«No artificial finite machine-size ceiling is encoded into the language merely for implementation convenience.»

Actual execution remains bounded by available:

- memory;
- compute;
- storage;
- network;
- quantum resources;
- energy;
- time;
- target capabilities;
- compiler/runtime representation.

---

117. Principle for Permanent Semantics

Only concepts that deserve long-term language meaning should become permanent core semantics.

Temporary implementation mechanisms must remain implementation details.

Vendor-specific details must remain target-specific unless intentionally standardized.

Experimental features must remain versioned/experimental until promoted.

This protects POCO-REAF from being broken by short-lived hardware assumptions.

---

118. Principle for Language Minimality

A universal language does not need a keyword for every possible operation.

Universal capability comes from composability.

A relatively small semantic core can represent a large computation space through:

types
+
functions
+
genericity
+
effects
+
capabilities
+
resources
+
operations
+
modules
+
extensions

This is preferable to an indefinitely growing list of specialized grammar productions.

---

119. Principle for Completeness

Zamani language completeness is measured by semantic coverage, not by the number of grammar productions.

A grammar with thousands of keywords is not necessarily more universal than a grammar with a smaller, well-composed semantic core.

Production readiness therefore requires:

semantic completeness
+
implementation completeness
+
integration completeness
+
test completeness

rather than maximum grammar size.

---

120. Final Language Contract

The Zamani language must satisfy the following contract:

Zamani source
    ↓
describes computation and intent
    ↓
without accidental machine assumptions
    ↓
with explicit semantics
    ↓
with explicit effects/capabilities/resources where required
    ↓
lowered to canonical semantic representations
    ↓
with quantum semantics crossing through quantum::ir
    ↓
optimized and scheduled independently of source syntax
    ↓
adapted to available resources and target capabilities
    ↓
lowered to the selected execution environment

Therefore:

One Program
      ↓
One Semantic Meaning
      ↓
Many Compilers / Targets
      ↓
Many Architectures
      ↓
Many Hardware Configurations
      ↓
Many Scales
      ↓
Many Execution Environments
      ↓
Future Platforms

The language must never require:

one program per CPU
one program per GPU
one program per FPGA
one program per QPU
one program per cluster
one program per machine size

unless the programmer has deliberately requested target-specific semantics.

---

121. Production Completion Criteria

This document is considered integrated only when all of the following are true:

- The grammar has one clearly defined normative authority.
- "Zamani.g4" is reconciled with the normative specification.
- "grammar.md" is an implementation-conformance artifact rather than a competing authority.
- "Zamani-Grammar.md" is clearly classified as historical, design, or formally promoted specification material.
- Lexer semantics conform to the language principles.
- Parser semantics conform to the language principles.
- AST representations preserve semantic information.
- Semantic analysis owns semantic validation.
- Type analysis owns type correctness.
- Effect analysis owns effect correctness.
- Capability analysis owns capability requirements.
- Resource analysis owns resource requirements.
- "quantum::ir" remains the canonical quantum semantic boundary.
- Quantum frontend code does not introduce duplicate canonical quantum IR.
- Scheduling consumes canonical quantum semantics.
- Optimization consumes canonical semantic representations.
- ZQN consumes canonical quantum semantics and target/noise information.
- Hardware layers own hardware-specific realization.
- Runtime layers own runtime resource discovery.
- No artificial fixed machine-size limit exists in language semantics.
- No fixed qubit maximum exists in grammar semantics.
- No fixed core/thread/device/node maximum exists in grammar semantics.
- Target-specific facts are represented through target/resource/capability mechanisms.
- Rust implementation remains compatible with Rust 1.97 / 1.97.1.
- Rust "unsafe" is not required.
- Diagnostics are structured.
- Parsing is deterministic.
- Unsupported syntax is not silently accepted.
- Existing valid features are preserved or explicitly migrated.
- Cross-domain integration is tested.
- Scalability is tested.
- Compatibility is tested.
- The complete language pipeline is documented.
- Every downstream grammar specification has an explicit relationship to these principles.

---

122. Non-Negotiable Final Principle

Zamani must not be designed around today's machine.

It must be designed around computation itself.

The language therefore follows:

«Semantics before hardware.»

«Intent before implementation.»

«Capabilities before assumptions.»

«Resources before fixed limits.»

«Canonical IR before optimization.»

«"quantum::ir" before quantum backend realization.»

«Portability before vendor dependence.»

«Composition before domain isolation.»

«Explicit constraints before hidden assumptions.»

«Safe implementation before unsafe shortcuts.»

«Versioned evolution before accidental incompatibility.»

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever.»

The ultimate Zamani contract is:

WRITE ONCE
    ↓
DEFINE MEANING ONCE
    ↓
COMPILE ONCE
    ↓
MAP TO AVAILABLE CAPABILITIES
    ↓
ADAPT TO AVAILABLE RESOURCES
    ↓
EXECUTE ANYWHERE
    ↓
PRESERVE SEMANTICS ACROSS SCALE
    ↓
REMAIN EXTENSIBLE FOR FUTURE COMPUTING

Zamani: From Atom to Everywhere.

POCO-REAF is a language architecture principle, not merely a deployment feature.