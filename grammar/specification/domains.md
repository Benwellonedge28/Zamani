Zamani Domain Specification

Path: "grammar/specification/domains.md"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Branch: "main"
Specification status: Normative
Scope: Universal computational domains
Rust implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety policy: Production Rust implementation MUST use safe Rust; Rust "unsafe" is prohibited.
Primary portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: From the smallest supported computation to arbitrarily large computations, subject only to program semantics, representation limits, declared policies, implementation capacity, target capabilities, and resources actually available.

---

1. Purpose

This document defines the normative domain model of the Zamani programming language.

It specifies how Zamani represents different kinds of computation while preserving one language, one semantic model, one portability model, and one integration pipeline.

Zamani is intended to express computation across:

- classical computing;
- systems programming;
- embedded computing;
- scientific and numerical computing;
- symbolic computing;
- high-performance computing;
- parallel computing;
- distributed computing;
- heterogeneous computing;
- accelerator computing;
- quantum computing;
- quantum error-corrected computing;
- hybrid quantum-classical computing;
- hardware description;
- hardware/software co-design;
- AI and machine learning;
- tensor and data computation;
- networking;
- cryptography and security;
- edge and cloud computing;
- temporal computation;
- multi-timeline computation;
- nano-oriented computation;
- metaprogramming;
- interoperability;
- future computational paradigms.

These are domains of one language, not separate programming languages joined together by unrelated grammar extensions.

The domain architecture MUST therefore allow a single program to combine multiple domains without requiring a rewrite of the program merely because its eventual realization changes.

---

2. Normative Authority

This document is subordinate to:

1. "grammar/specification/README.md"
2. "grammar/specification/language.md"
3. "grammar/DESIGN.md"

It specializes their domain architecture.

The concrete syntax is represented by:

- "grammar/Zamani.g4"
- domain grammar contracts under "grammar/"

The executable implementation is represented by:

- "src/lexer.rs"
- "src/parser.rs"
- "src/frontend/ast/"
- semantic analysis
- canonical semantic models
- IR generation and verification
- domain IR
- compiler/lowering infrastructure
- runtime
- scheduling
- routing
- resilience
- ZQN
- HAL
- backends

This file MUST NOT become a second grammar.

It defines what a computational domain means and how it integrates, not every concrete parser production.

---

3. Existing Repository Integration

The domain specification integrates with the existing repository architecture as follows:

grammar/specification/domains.md
                │
                ▼
      Domain semantic contracts
                │
                ▼
      grammar/domain directories
                │
                ▼
          grammar/Zamani.g4
                │
                ▼
          src/lexer.rs
                │
                ▼
          src/parser.rs
                │
                ▼
       src/frontend/ast/
                │
                ▼
     Structural validation
                │
                ▼
 Type / Effect / Resource / Capability
             Analysis
                │
                ▼
      Canonical semantic model
                │
        ┌───────┼────────┐
        ▼       ▼        ▼
   Classical quantum   HDL/
      IR       ::ir    Hardware
        │       │        │
        └───────┼────────┘
                ▼
           Optimization
                │
        ┌───────┼────────┐
        ▼       ▼        ▼
     Routing Scheduling Resilience
                │
                ▼
               ZQN
                │
                ▼
               HAL
                │
                ▼
        Target realization
                │
        ┌───────┼───────────────┐
        ▼       ▼       ▼       ▼
       CPU     GPU     FPGA     QPU
        │       │       │       │
        └───────┴───────┴───────┘
                    │
                    ▼
              Future targets

A domain MUST NOT bypass this architecture merely because it is technologically specialized.

---

4. Core Domain Principle

A Zamani domain describes a kind of computation and its semantic requirements.

A domain does not automatically describe:

- a physical machine;
- a vendor;
- a device identifier;
- a fixed topology;
- a fixed resource count;
- a particular instruction set;
- a particular compiler;
- a particular runtime;
- a physical qubit;
- a specific memory bank;
- a specific GPU;
- a specific CPU;
- a specific FPGA;
- a specific network node.

The fundamental separation is:

Domain meaning
      ≠
Resource requirements
      ≠
Target capabilities
      ≠
Physical realization

For example:

quantum computation

is a domain.

requires capability("quantum.mid_circuit_measurement")

is a capability requirement.

requires qubits >= n

is a resource requirement.

prefer accelerator("quantum")

is a preference.

map logical_qubit -> physical_qubit(17)

is a physical realization.

Only the first three categories are inherently portable program semantics.

Physical realization belongs downstream.

---

5. Domain Definition

A domain is a semantic namespace describing a coherent class of computational entities, operations, types, effects, resources, capabilities, and correctness properties.

Every domain MUST have:

1. an identifier;
2. a semantic purpose;
3. a domain boundary;
4. syntax ownership;
5. type integration;
6. expression integration;
7. statement/declaration integration where necessary;
8. effect integration;
9. resource integration;
10. capability integration;
11. AST mapping;
12. semantic mapping;
13. IR mapping;
14. compiler integration;
15. runtime integration where required;
16. diagnostics;
17. testing requirements;
18. compatibility requirements;
19. scalability requirements;
20. hard-coding rules.

A domain MUST NOT exist merely because a directory or keyword exists.

---

6. Domain Lifecycle

Every domain and every domain feature has a lifecycle.

PROPOSED
   ↓
SPECIFIED
   ↓
GRAMMAR_DEFINED
   ↓
LEXER_IMPLEMENTED
   ↓
PARSER_IMPLEMENTED
   ↓
AST_MAPPED
   ↓
SEMANTICALLY_IMPLEMENTED
   ↓
IR_MAPPED
   ↓
COMPILER_INTEGRATED
   ↓
RUNTIME_INTEGRATED
   ↓
TESTED
   ↓
STABLE

A feature may also be:

EXPERIMENTAL
DEPRECATED
REMOVED

A construct MUST NOT be considered stable merely because it parses.

---

7. Universal Domain Contract

Every domain MUST satisfy the following contract.

7.1 File

The domain specification or domain contract.

7.2 Purpose

What computational meaning the domain provides.

7.3 Owns

The semantic concepts belonging uniquely to the domain.

7.4 Does Not Own

Concepts delegated to universal language infrastructure or other domains.

7.5 Inputs

The AST, types, values, effects, resources, capabilities, and metadata consumed by the domain.

7.6 Outputs

The semantic representation or canonical IR produced by the domain.

7.7 Upstream Contracts

The specifications and frontend structures consumed by the domain.

7.8 Downstream Consumers

Compiler, optimizer, router, scheduler, resilience system, ZQN, HAL, runtime, or backend components consuming the domain representation.

7.9 Grammar Contract

Concrete syntax allowed by the domain.

7.10 AST Contract

How domain syntax maps to domain-neutral AST structures.

7.11 Semantic Contract

What the syntax means.

7.12 IR Contract

How semantic meaning enters the canonical IR architecture.

7.13 Resource Contract

How resource requirements are represented without hard-coded machine limits.

7.14 Capability Contract

How required or optional capabilities are represented.

7.15 Diagnostics Contract

How invalid domain programs are reported.

7.16 Scalability Contract

How the domain scales from minimal to arbitrarily large computations.

7.17 Hard-Coding Audit

Proof that the domain does not establish artificial universal limits.

---

8. Universal Domain Invariants

All domains MUST obey the following invariants.

8.1 One Language

Domains MUST NOT create independent lexical languages unless explicitly isolated as interoperability formats.

8.2 One Source Model

Domains share the same:

- source spans;
- identifiers;
- names;
- modules;
- expressions;
- declarations;
- types;
- effects;
- diagnostics;
- resources;
- capabilities;
- versioning;
- compatibility model.

8.3 Domain Neutral AST

Domain syntax MUST lower into the existing domain-neutral frontend AST.

A domain MUST NOT create a parallel frontend architecture.

8.4 Semantic Separation

Domain syntax MUST NOT directly encode backend implementation details.

8.5 Canonical IR

A domain MUST integrate with an existing canonical semantic/IR boundary.

If a domain requires a genuinely distinct IR, that IR must be explicitly justified and specified.

Quantum is the established example:

quantum syntax
      ↓
domain-neutral AST
      ↓
quantum semantic model
      ↓
quantum::ir

No second quantum IR may be introduced.

---

9. Domain Composition

Domains MUST be composable.

A program MAY simultaneously contain:

classical
+
quantum
+
AI
+
data
+
networking
+
hardware
+
HDL
+
distributed

provided that the semantic interactions are valid.

For example:

data
   ↓
AI training
   ↓
classical preprocessing
   ↓
quantum kernel
   ↓
measurement
   ↓
classical decision
   ↓
network transmission

This MUST be represented as one Zamani program rather than separate languages joined through opaque foreign interfaces.

---

10. Domain Interaction Model

Domain interaction occurs through common semantic mechanisms:

Values
Types
Operations
Effects
Capabilities
Resources
Constraints
Contracts
Modules
Dataflow
Control flow

A domain MUST NOT directly depend on another domain's private implementation structures.

For example:

AI → quantum::ir

is prohibited if AI directly manipulates the private internal representation of quantum IR.

Instead:

AI semantics
     ↓
common semantic boundary
     ↓
quantum operation semantics
     ↓
quantum::ir

---

11. Domain Categories

Zamani domains are organized into the following categories.

11.1 Universal Foundations

- core;
- types;
- expressions;
- statements;
- declarations;
- functions;
- modules;
- effects;
- memory;
- concurrency.

11.2 Computational Domains

- classical;
- quantum;
- hybrid;
- AI;
- data;
- symbolic;
- scientific;
- numerical;
- signal-processing;
- optimization.

11.3 Physical and Hardware Domains

- HDL;
- hardware;
- accelerator;
- embedded;
- nano;
- hardware/software co-design.

11.4 Distributed and Networked Domains

- distributed;
- networking;
- edge;
- cloud;
- service-oriented computation.

11.5 Security Domains

- security;
- cryptography;
- secure computation;
- zero-knowledge;
- provenance and trust.

11.6 Temporal and Advanced Domains

- temporal;
- multi-timeline;
- Sankofa/memory-oriented computation;
- metaprogramming;
- macros;
- dialects.

11.7 Interoperability Domains

- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL formats;
- serialization formats;
- other explicitly defined foreign formats.

Interoperability formats MUST NOT replace Zamani's canonical semantic model.

---

12. Classical Computing Domain

Purpose

The classical domain represents general-purpose computation over conventional computational abstractions.

It includes:

- scalar computation;
- integer computation;
- floating-point computation;
- arbitrary structured data;
- arrays;
- vectors;
- matrices;
- tensors;
- symbolic computation;
- numerical computation;
- scientific computation;
- control algorithms;
- signal processing;
- optimization.

Owns

Classical semantic operations and classical computational types.

Does Not Own

- CPU identities;
- core counts;
- register allocation;
- cache topology;
- instruction selection;
- physical memory banks;
- vendor-specific CPU instructions.

Integration

classical syntax
      ↓
generic AST
      ↓
classical semantic model
      ↓
classical IR
      ↓
optimization
      ↓
target lowering

The same classical semantic program MAY lower to:

- CPU;
- GPU;
- FPGA;
- accelerator;
- distributed execution;
- embedded processor;
- future architecture.

---

13. Quantum Computing Domain

Purpose

The quantum domain represents quantum computation independently of a particular physical quantum device.

It includes:

- qubits;
- logical quantum states;
- registers;
- quantum operations;
- parameterized operations;
- controls;
- adjoints;
- measurement;
- reset;
- dynamic control;
- classical feed-forward;
- observables;
- channels;
- noise intent;
- error-correction intent;
- logical operations;
- quantum resource requirements.

Canonical Boundary

The canonical quantum semantic boundary is:

quantum::ir

The domain MUST lower to "quantum::ir".

No alternate quantum IR may be created by the grammar or frontend merely to support a new quantum feature.

Operation Model

Quantum operations MUST be generic.

The grammar MUST NOT depend on an exhaustive list such as:

H
X
Y
Z
CNOT
...

as the fundamental language model.

Instead, quantum operations conceptually consist of:

operation identity
+
parameters
+
controls
+
targets
+
modifiers
+
effects
+
capabilities
+
source provenance

This permits:

- standard operations;
- user-defined operations;
- library operations;
- vendor operations;
- future operations;
- decomposed operations;
- logical operations.

Physical Independence

Quantum source MUST NOT require:

physical_qubit(17)

to express ordinary quantum semantics.

Physical mapping belongs to routing and target realization.

Resource Scalability

The language MUST NOT define:

MAX_QUBITS

or any equivalent universal quantum ceiling.

A quantum program may require:

n qubits

where "n" is:

- constant;
- symbolic;
- generic;
- computed;
- runtime-dependent;
- resource-dependent.

The compiler determines feasibility.

Integration

quantum syntax
      ↓
domain-neutral AST Operation
      ↓
quantum semantic validation
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
QPU realization

QEC owns error correction.

ZQN owns fault/noise semantics.

Routing owns physical realization.

Scheduling owns ordering and resource timing.

HAL owns target capability/state.

The grammar owns none of those implementations.

---

14. Hybrid Quantum-Classical Domain

Purpose

The hybrid domain represents computations in which classical and quantum computation interact as one semantic program.

Examples include:

classical preprocessing
        ↓
quantum computation
        ↓
measurement
        ↓
classical decision
        ↓
quantum operation

The domain MUST support:

- host/device relationships;
- classical-to-quantum data flow;
- quantum-to-classical measurement flow;
- classical feed-forward;
- synchronization;
- hybrid functions;
- resource relationships;
- asynchronous execution where semantically valid.

Integration

Classical values MUST use the ordinary Zamani type system.

Quantum entities MUST use quantum semantic types.

The boundary between them MUST be explicit in the semantic model.

No second hybrid IR should be created solely to connect classical and quantum representations.

---

15. HDL Domain

Purpose

HDL describes hardware structure and hardware behavior as semantic hardware intent.

It may represent:

- modules;
- ports;
- signals;
- nets;
- registers;
- combinational logic;
- sequential logic;
- clocks;
- resets;
- timing;
- interfaces;
- protocols;
- state machines;
- pipelines;
- memories;
- parameterization;
- generation;
- synthesis intent;
- simulation intent;
- verification properties.

Owns

Hardware-description semantics.

Does Not Own

- one FPGA model;
- one ASIC process;
- fixed bus widths;
- fixed clock frequencies;
- physical placement;
- routing algorithms;
- transistor-level implementation unless explicitly represented by a dedicated physical-design domain.

Parameterization

Hardware widths and dimensions MAY be program semantics.

For example:

width = W

is valid.

A universal language limit such as:

MAX_WIDTH = 32

is prohibited.

Integration

HDL syntax
    ↓
domain-neutral AST
    ↓
hardware semantic model
    ↓
HDL/Hardware IR
    ↓
synthesis / lowering / verification
    ↓
target realization

---

16. Hardware and Hardware/Software Co-Design Domain

The hardware domain represents target-independent hardware intent.

It may express:

- compute resources;
- memory requirements;
- accelerator requirements;
- communication requirements;
- topology constraints;
- timing requirements;
- power constraints;
- thermal constraints;
- reliability;
- calibration requirements;
- deployment constraints;
- capability requirements.

The programmer MAY express:

requires capability("tensor.compute")
requires capability("quantum.measurement")
requires memory >= required_memory
requires reliability >= required_reliability

The program SHOULD NOT require:

gpu_device(3)
cpu_core(7)
memory_bank(2)
physical_qubit(17)

unless the program is explicitly a target/deployment-specific program.

Target-specific declarations MUST be marked and isolated from portable semantics.

---

17. Accelerator Domain

Accelerators include, but are not limited to:

- GPUs;
- TPUs;
- NPUs;
- DSPs;
- FPGAs;
- custom accelerators;
- quantum accelerators;
- future accelerators.

The grammar MUST NOT contain a fixed accelerator catalogue.

Instead:

capability
+
resource
+
operation
+
constraint
+
preference

describe accelerator intent.

For example:

requires capability("tensor.compute")
prefer accelerator("matrix")

does not require one particular vendor or device.

---

18. Embedded Computing Domain

Embedded computation MUST be expressible without requiring a separate language.

The domain may represent:

- constrained execution;
- device I/O;
- timing;
- interrupts;
- memory regions;
- peripherals;
- power constraints;
- real-time requirements;
- hardware interfaces.

Resource scarcity is represented as a target/resource property.

It MUST NOT become a language-level reduction in semantic expressiveness.

A small embedded target may reject or transform a program because it cannot satisfy requirements; that does not redefine the language.

---

19. AI and Machine Learning Domain

The AI domain includes:

- models;
- tensors;
- datasets;
- training;
- inference;
- optimization;
- differentiable computation;
- probabilistic computation;
- neural computation;
- symbolic reasoning;
- agents;
- learning;
- model deployment;
- distributed training.

The domain MUST remain framework-independent.

The grammar MUST NOT become a collection of:

tensorflow
pytorch
jax
cuda
...

keywords.

Frameworks are interoperability/backend concerns.

AI semantics should use:

types
operations
graphs
effects
resources
capabilities
data

and lower through canonical semantic structures.

---

20. Data Domain

The data domain represents:

- collections;
- records;
- tables;
- streams;
- schemas;
- tensors;
- datasets;
- transformations;
- queries;
- pipelines;
- serialization;
- persistence;
- provenance.

Data structures MUST support arbitrary logical size subject to resources.

The grammar MUST NOT establish universal limits for:

- rows;
- columns;
- tensor dimensions;
- tensor rank;
- dataset size;
- stream length;
- schema fields.

---

21. Scientific and Numerical Domain

Scientific computation includes:

- numerical algorithms;
- linear algebra;
- statistics;
- calculus;
- differential equations;
- optimization;
- simulation;
- signal processing;
- symbolic mathematics.

Mathematical functions SHOULD normally be library/intrinsic semantic operations rather than individual reserved keywords.

For example, the language need not reserve a keyword for every mathematical algorithm.

The semantic model should support:

operation
+
typed arguments
+
constraints
+
capabilities

This allows new mathematical methods to be introduced without changing the grammar.

---

22. Symbolic Computation Domain

Symbolic computation may include:

- symbolic values;
- symbolic expressions;
- algebraic transformations;
- symbolic differentiation;
- symbolic constraints;
- theorem-oriented transformations;
- symbolic optimization.

Symbolic expressions MUST remain distinct from runtime numerical values where necessary.

The domain MUST integrate with the ordinary expression/type system rather than introduce a second expression language.

---

23. Parallel and HPC Domain

The parallel domain represents:

- data parallelism;
- task parallelism;
- pipeline parallelism;
- reductions;
- collectives;
- work distribution;
- synchronization;
- deterministic parallelism.

The language MUST NOT assume:

8 threads
16 cores
64 GPUs

as universal execution structures.

Instead:

parallel

expresses parallel semantic intent.

The runtime/compiler determines actual worker allocation.

If the programmer genuinely requires a resource quantity, that requirement belongs in the resource model.

---

24. Distributed Computing Domain

The distributed domain includes:

- processes;
- actors;
- services;
- messages;
- channels;
- replication;
- partitioning;
- placement intent;
- consistency;
- transactions;
- fault tolerance;
- collectives;
- distributed data;
- distributed execution.

The grammar MUST NOT define a fixed maximum number of nodes.

A program may describe an arbitrarily large logical distributed system.

The deployment layer determines the actual node set.

---

25. Networking Domain

Networking includes:

- endpoints;
- abstract addresses;
- protocols;
- channels;
- requests;
- responses;
- streaming;
- routing intent;
- service discovery;
- distributed communication;
- network capabilities.

The portable language MUST prefer abstract communication semantics over physical addressing.

A target-specific network address may be represented in a deployment or interoperability layer when required.

---

26. Security Domain

Security includes:

- identity;
- authorization;
- capabilities;
- policies;
- secrets;
- cryptography;
- hashes;
- signatures;
- key management;
- secure computation;
- zero-knowledge;
- trust;
- provenance.

Security semantics MUST NOT be implemented merely through parser keywords.

A security construct requires:

syntax
↓
AST
↓
semantic validation
↓
security policy
↓
compiler/runtime integration
↓
verification

A parser accepting a security keyword does not itself provide security.

---

27. Cryptography Domain

Cryptographic algorithms should generally be represented as semantic operations or library capabilities rather than permanently reserved language keywords.

The domain may specify:

cryptographic operation
+
algorithm identity
+
parameters
+
security properties
+
required capabilities

Vendor or implementation-specific cryptographic acceleration belongs downstream.

---

28. Edge and Cloud Domain

Edge and cloud execution are deployment realizations of distributed and heterogeneous computation.

They MUST NOT become separate incompatible languages.

The same program MAY be realized as:

embedded
edge
single machine
cluster
cloud
hybrid deployment

provided semantic requirements can be satisfied.

Deployment policy belongs under:

- "grammar/compile/"
- "grammar/execution/"
- "grammar/resources/"
- "grammar/hardware/"

as appropriate.

---

29. Temporal Domain

Temporal computation may represent:

- time;
- temporal values;
- deadlines;
- durations;
- schedules;
- temporal constraints;
- event ordering;
- temporal state.

Temporal syntax MUST NOT impose a fixed number of:

- timelines;
- events;
- timestamps;
- branches.

Time representation belongs to the type/semantic model.

Physical clocks belong to execution/target realization.

---

30. Multi-Timeline Domain

Multi-timeline computation may represent:

- timeline creation;
- branching;
- observation;
- speculative execution;
- fork;
- merge;
- rewind;
- temporal state.

There MUST be no universal limit on the number of timelines or branches.

The domain MUST distinguish:

logical timeline

from:

physical execution process

The compiler/runtime determines realization.

---

31. Sankofa / Memory-Oriented Domain

Sankofa-inspired concepts may include:

- remember;
- recall;
- learning;
- history;
- provenance;
- wisdom;
- temporal knowledge;
- consensus;
- inter-memory relationships.

These concepts MUST be treated as language semantics only after promotion through the standard feature lifecycle.

The grammar MUST NOT imply that the parser itself stores memory.

Memory semantics belong to the semantic/runtime architecture.

---

32. Nano-Oriented Domain

Nano-oriented computation may express:

- atoms;
- molecules;
- materials;
- interactions;
- nano-agents;
- nanoscale processes;
- protocols.

The domain MUST describe computational or physical intent rather than hard-code a finite catalogue of physical entities.

A periodic table, material database, device catalogue, or physical simulator belongs outside the grammar.

---

33. Security and Safety Boundary

All domains MUST respect the repository-wide safety architecture.

Production compiler implementation MUST use:

Rust 1.97 / Rust 1.97.1
Rust 2021
safe Rust

No production compiler implementation may rely on:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

The existence of a source-language construct named "unsafe" is a separate language-design issue and does not authorize Rust "unsafe".

If Zamani exposes a source-level unsafe capability, it requires its own:

- specification;
- capability model;
- security model;
- AST mapping;
- semantic rules;
- compiler implementation;
- diagnostics;
- tests;
- compatibility rules.

---

34. Domain Resource Model

Every domain MUST integrate with the resource model.

Resources include, conceptually:

compute
memory
storage
communication
quantum resources
accelerator resources
energy
power
thermal budget
time
latency
bandwidth
reliability
availability

Resource quantities MUST be semantic values or constraints.

They MUST NOT become parser-level universal limits.

---

35. Resource Requirement Categories

Every domain MUST distinguish:

Requirement

Necessary for semantic validity.

requires qubits >= n

Capability

Required functionality.

requires capability("quantum.mid_circuit_measurement")

Constraint

Restriction on realization.

constraint latency <= budget

Preference

Preferred but not mandatory.

prefer accelerator("quantum")

Hint

Optimization guidance.

hint locality

Realization

Actual target mapping.

logical_resource -> physical_resource

These categories MUST NOT be conflated.

---

36. Domain Capability Model

Capabilities identify what an execution environment can do.

Examples:

quantum.measurement
quantum.mid_circuit_measurement
quantum.dynamic_control
tensor.compute
distributed.communication
hardware.reconfiguration
secure.key_management
network.streaming

Capability names are semantic identifiers.

They MUST NOT require a finite universal registry.

Future capabilities MUST be representable without modifying the entire grammar.

---

37. Domain Scaling

Every domain MUST support the principle:

smallest meaningful computation
        ↓
larger computation
        ↓
arbitrarily large logical computation

The language MUST NOT introduce artificial ceilings such as:

MAX_DOMAIN_OBJECTS
MAX_OPERATIONS
MAX_FIELDS
MAX_NODES
MAX_QUBITS
MAX_TENSORS
MAX_AGENTS
MAX_CHANNELS
MAX_MODULES

Concrete implementations may have resource budgets.

Those budgets are implementation/runtime constraints, not language semantics.

---

38. Infinity Semantics

"Infinity" in Zamani's scalability objective means:

«The language MUST NOT artificially impose a finite universal computational ceiling merely because current hardware or implementation resources are finite.»

It does not mean that a physical machine has infinite resources.

Concrete execution is always bounded by:

- available memory;
- available compute;
- storage;
- time;
- target capabilities;
- physical constraints;
- implementation representation;
- operating environment.

Therefore:

unbounded language model

does not imply:

infinite physical execution

---

39. Domain Portability

A domain is portable when its semantic meaning survives target changes.

For example:

Tensor computation

may be realized by:

CPU
GPU
FPGA
NPU
distributed cluster
future accelerator

without changing the source semantics.

Likewise:

quantum computation

may be realized by:

simulator
QPU
logical QPU
distributed quantum system
future quantum architecture

where the required semantics and capabilities are satisfied.

---

40. Domain Specialization

Specialization is permitted downstream.

The compiler may specialize:

generic operation
      ↓
target-specific implementation

provided specialization preserves the specified semantics.

Examples:

generic matrix operation
      ↓
SIMD implementation

generic tensor operation
      ↓
GPU kernel

generic quantum operation
      ↓
native pulse/decomposition

generic communication
      ↓
network protocol implementation

Specialization MUST NOT leak backward and redefine the source language.

---

41. Domain-Specific Syntax Policy

Dedicated syntax is appropriate when:

1. it has genuine language-level semantics;
2. it improves composability;
3. it cannot reasonably be represented using existing constructs;
4. it has a stable semantic contract;
5. it has an AST mapping;
6. it has an IR mapping;
7. it has tests;
8. it does not introduce artificial limits.

A concept SHOULD remain an identifier, library operation, capability, or semantic operation when dedicated syntax is unnecessary.

This prevents Zamani from becoming a catalogue of every algorithm, device, vendor, protocol, or scientific function.

---

42. Domain Grammar Integration

The conceptual grammar organization is:

grammar/
├── core/
├── types/
├── expressions/
├── statements/
├── declarations/
├── functions/
├── modules/
├── effects/
├── memory/
├── concurrency/
├── classical/
├── quantum/
├── hybrid/
├── hdl/
├── hardware/
├── distributed/
├── ai/
├── data/
├── networking/
├── security/
├── resources/
├── compile/
├── execution/
├── interoperability/
├── dialects/
├── macros/
└── metaprogramming/

These are ownership boundaries, not independent languages.

"grammar/Zamani.g4" remains the canonical ANTLR composition/root grammar.

---

43. Domain Grammar Rule

Each domain grammar file MUST specify:

Purpose
Owns
Does Not Own
Imports/Dependencies
Grammar Rules
Tokens Used
AST Mapping
Semantic Mapping
IR Mapping
Resource Mapping
Capability Mapping
Diagnostics
Tests
Compatibility
Scalability
Hard-Coding Audit
Completion Criteria

A grammar file is not complete until all of these are defined.

---

44. Domain AST Contract

The domain frontend MUST use the existing domain-neutral AST architecture.

The preferred generic model is conceptually:

Operation {
    name
    namespace
    operands
    parameters
    results
    attributes
    modifiers
    effects
    capabilities
    source
}

A domain MUST NOT create a special AST enumeration merely because the domain has many possible operations.

For example, quantum syntax SHOULD lower into generic operation structures rather than:

enum QuantumGate {
    H,
    X,
    Y,
    Z,
    CNOT,
    ...
}

This is especially important for future operations.

---

45. Domain Semantic Contract

Every domain operation MUST answer:

1. What does it mean?
2. What values does it consume?
3. What values does it produce?
4. What effects does it have?
5. What capabilities does it require?
6. What resources does it require?
7. What constraints apply?
8. Is it deterministic?
9. What errors can occur?
10. What ownership rules apply?
11. What transformations preserve its meaning?

---

46. Domain IR Contract

Every domain MUST identify its canonical semantic/IR boundary.

Examples:

classical → classical/canonical IR
quantum   → quantum::ir
HDL       → HDL/Hardware IR

A domain MUST NOT invent another IR solely for convenience.

If an existing IR is insufficient, the deficiency must be resolved explicitly through an IR architecture change.

---

47. Compiler Integration

The compiler MUST treat domain information as semantic information.

The pipeline is:

AST
 ↓
domain classification
 ↓
name resolution
 ↓
type checking
 ↓
effect checking
 ↓
resource analysis
 ↓
capability analysis
 ↓
domain semantic validation
 ↓
canonical semantic representation
 ↓
IR lowering

No domain should jump directly from parser to backend.

---

48. Runtime Integration

Runtime integration is required only when the domain has runtime semantics.

Examples:

- networking;
- dynamic resource allocation;
- device interaction;
- quantum execution;
- distributed execution;
- temporal execution;
- persistent memory.

Purely compile-time domains may terminate at compilation/lowering.

The domain specification MUST explicitly state whether runtime integration is required.

---

49. Optimization Boundary

Optimization belongs downstream of domain semantics.

An optimizer MAY transform:

domain operation A

into:

domain operation B

only when semantic equivalence is proven or explicitly permitted.

Optimization MUST NOT redefine domain meaning.

---

50. Quantum Optimization Boundary

Quantum optimization may include:

- decomposition;
- cancellation;
- commutation;
- gate synthesis;
- layout optimization.

It MUST occur after semantic quantum representation.

The grammar MUST NOT encode optimizer algorithms.

---

51. Routing Boundary

Routing determines physical realization.

This is especially important for:

- quantum;
- distributed;
- networking;
- accelerator;
- hardware domains.

The domain specification MUST NOT hard-code physical topology.

For quantum:

logical qubit
      ↓
routing
      ↓
physical realization

For distributed computation:

logical process
      ↓
placement
      ↓
physical node

---

52. Scheduling Boundary

Scheduling determines:

- ordering;
- timing;
- resource allocation;
- concurrency;
- dependencies;
- synchronization.

Scheduling MUST NOT be encoded as fixed hardware topology in the grammar.

A source program may express timing requirements or constraints.

The scheduler determines the concrete schedule.

---

53. Resilience Boundary

Resilience belongs to the execution/compiler architecture.

The grammar may express:

requires reliability >= R
requires fault_tolerance(...)

but MUST NOT implement:

- retries;
- recovery;
- QEC;
- health monitoring;
- quarantine;
- repair;
- device replacement.

Those remain runtime/resilience responsibilities.

---

54. QEC Boundary

Quantum error correction is not grammar semantics.

The grammar may express QEC intent or requirements.

QEC implementation belongs to the existing quantum resilience architecture.

The pipeline remains:

quantum semantics
      ↓
quantum::ir
      ↓
QEC analysis/transformation
      ↓
routing/scheduling
      ↓
ZQN
      ↓
HAL

No grammar feature may create a second QEC subsystem.

---

55. ZQN Boundary

ZQN represents fault/noise semantics.

Domain syntax may declare relevant properties.

The grammar MUST NOT implement:

- noise simulation;
- calibration;
- fault injection;
- physical error models.

Those belong downstream.

---

56. HAL Boundary

HAL owns target capabilities and target state.

The grammar may express:

requires capability(...)

but it must not embed a hardware catalogue.

HAL determines whether a concrete target supports the required capability.

---

57. Target Selection

Target selection is not domain semantics.

The compiler/runtime MAY select:

CPU
GPU
FPGA
QPU
cluster
edge device
cloud
future accelerator

according to:

- semantic requirements;
- capabilities;
- resources;
- constraints;
- preferences;
- deployment policies.

---

58. Target-Specific Domains

Target-specific syntax MAY exist for:

- deployment;
- embedded control;
- HDL;
- foreign interfaces;
- backend directives.

However, target-specific constructs MUST be explicitly isolated.

They MUST NOT silently become universal Zamani semantics.

---

59. Interoperability Domains

Interoperability formats include:

- OpenQASM;
- QIR;
- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- HDL formats;
- serialization formats.

These are boundaries around Zamani.

The canonical direction is:

foreign format
      ↓
Zamani semantic representation

or:

Zamani semantic representation
      ↓
foreign format

depending on the interoperability operation.

A foreign format MUST NOT replace the canonical Zamani semantic model.

---

60. Dialects

Dialects extend Zamani in controlled ways.

A dialect MUST declare:

name
version
owner
syntax extensions
semantic extensions
AST mapping
IR mapping
capabilities
compatibility
feature gates

A dialect MUST NOT silently redefine core Zamani semantics.

A dialect that introduces target-specific behavior MUST identify that behavior explicitly.

---

61. Future Computational Domains

Zamani MUST be extensible to computational paradigms not currently known.

A future domain should integrate through:

types
+
operations
+
effects
+
capabilities
+
resources
+
constraints
+
modules
+
semantic model
+
canonical IR boundary

The addition of a future domain SHOULD NOT require redesigning:

- the lexer architecture;
- the parser architecture;
- the module system;
- the entire type system;
- the entire resource model;
- existing domain semantics.

---

62. Domain Discovery

The compiler may classify an operation through:

namespace
name
type
capabilities
effects
attributes
module
dialect

The compiler MUST NOT require a finite global list of all future domain operations.

This permits extensibility.

---

63. Domain Namespace

Domain entities SHOULD use explicit namespaces when ambiguity is possible.

Conceptually:

quantum.measure
tensor.matmul
network.send
crypto.hash
hardware.compute

Namespace resolution belongs to semantic analysis.

The lexer should not need a separate keyword for every namespace member.

---

64. Domain Effects

Domains may introduce effects.

Examples:

quantum.measurement
network.io
device.io
allocation
distributed.communication
mutation
randomness
temporal.control

Effects MUST be integrated with the universal effect system.

A domain MUST NOT invent an incompatible effect mechanism.

---

65. Domain Determinism

Every domain MUST specify whether operations are:

- deterministic;
- nondeterministic;
- externally nondeterministic;
- physically random;
- quantum-random;
- scheduler-dependent.

Quantum measurement randomness MUST be distinguished from accidental compiler nondeterminism.

Network timing nondeterminism MUST be distinguished from semantic nondeterminism.

Optimization MUST preserve deterministic observable semantics.

---

66. Domain Provenance

Domain transformations MUST preserve source provenance.

For example:

source quantum operation
       ↓
optimized operation
       ↓
decomposed operation
       ↓
scheduled operation

must remain traceable where feasible.

This is required for:

- diagnostics;
- debugging;
- reproducibility;
- verification;
- profiling;
- provenance;
- IDE tooling.

---

67. Domain Diagnostics

Every domain MUST provide structured diagnostics for:

- invalid syntax;
- invalid types;
- missing capabilities;
- insufficient resources;
- incompatible operations;
- invalid domain composition;
- unsupported target;
- invalid effect;
- invalid ownership;
- invalid resource lifetime;
- ambiguous operation;
- invalid target-specific usage.

Diagnostics MUST distinguish:

language error
semantic error
resource infeasibility
target incompatibility
runtime unavailability

These are not interchangeable.

---

68. Resource Failure Semantics

If a target lacks resources, the compiler MUST NOT silently alter semantics.

For example:

requires qubits >= n

on a target with insufficient qubits may result in:

target infeasible

The compiler MAY use an explicitly permitted strategy such as:

- distribution;
- simulation;
- decomposition;
- logical resource mapping;
- alternative accelerator;
- deferred execution.

The strategy MUST preserve specified semantics.

---

69. Domain Composition Example

A single program may conceptually perform:

data ingestion
      ↓
distributed preprocessing
      ↓
tensor computation
      ↓
AI model
      ↓
quantum kernel
      ↓
measurement
      ↓
classical decision
      ↓
network transmission
      ↓
hardware accelerator

This MUST remain one semantic program.

Each domain contributes its own semantics while sharing:

types
values
effects
resources
capabilities
modules
diagnostics
provenance
IR integration

---

70. POCO-REAF Domain Guarantee

For a portable domain feature:

source semantics

MUST remain stable across compatible targets.

The target may change:

CPU → GPU
GPU → FPGA
FPGA → accelerator
accelerator → QPU
single node → cluster
cluster → cloud

without requiring source changes when the semantic requirements permit the transformation.

---

71. Compile-Once Guarantee

"Compile Once" does not mean one physical machine code binary is guaranteed to execute on every future architecture.

It means the portable semantic compilation artifact MUST preserve program meaning independently of target-specific realization.

Target-specific lowering may occur later through:

portable representation
+
target capabilities
+
resource information
+
deployment configuration

The distinction MUST remain explicit.

---

72. Run-Everywhere Guarantee

A program is portable when the target can satisfy:

semantic requirements
+
capabilities
+
resource constraints
+
correctness requirements

A target may reject a program when it cannot satisfy them.

Rejection is not a portability violation.

Silently changing the meaning is.

---

73. Run-Forever Guarantee

"Forever" means long-term semantic durability.

This requires:

- explicit language editions;
- compatibility rules;
- migration rules;
- deprecation;
- provenance;
- stable semantic contracts;
- dialect versioning;
- canonical IR boundaries;
- preservation of source meaning.

It does not promise that every historical physical device or compiler remains executable forever.

---

74. Hard-Coding Prohibition

No domain may introduce universal implementation limits such as:

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
MAX_MODULES
MAX_OPERATIONS

Nor finite universal enumerations such as:

Qubit0
Qubit1
...
Qubit1023

unless they occur as ordinary user data.

---

75. Program Constants Are Not Hard-Coding

The prohibition does not forbid program constants.

Valid:

let n = 1024;
allocate n elements;

Valid:

Tensor<f64, [1024, 1024]>

Potentially valid:

requires qubits >= 1024;

Invalid architecture:

Zamani supports at most 1024 qubits.

The distinction is:

program semantics

versus:

language implementation ceiling

---

76. Domain Scalability Tests

Every domain MUST include tests for:

Minimal

Smallest valid semantic construct.

Normal

Representative programs.

Large

Large number of entities.

Symbolic

Symbolic dimensions/resource counts.

Dynamic

Runtime-determined quantities where supported.

Boundary

Values close to implementation resource limits.

Resource exhaustion

Insufficient target resources.

Cross-domain

Interaction with at least one other domain.

Portability

Different target capabilities.

Determinism

Repeated compilation/execution produces stable semantics where required.

Deep structure

Deeply nested domain constructs.

Wide structure

Large numbers of sibling entities.

No scalability test may establish an artificial universal maximum.

---

77. Domain Compatibility

Every domain feature MUST define compatibility behavior.

Compatibility includes:

- source compatibility;
- AST compatibility;
- semantic compatibility;
- IR compatibility;
- compiler compatibility;
- runtime compatibility;
- dialect compatibility.

A grammar change that parses differently but has the same spelling MUST still be reviewed for semantic compatibility.

---

78. Domain Versioning

Each stable domain feature MUST have a versioned semantic contract.

Versioning MUST distinguish:

language version
domain version
feature version
dialect version
implementation version

A compiler version is not automatically a language version.

---

79. Domain Feature Manifest Integration

Each substantial domain feature SHOULD have a machine-readable feature manifest under the repository's feature-contract mechanism.

A manifest should identify:

id
name
status
version
domain
syntax
grammar
lexer_tokens
ast_nodes
semantic_rules
effects
capabilities
resources
ir_mapping
compiler_consumers
runtime_consumers
tests
compatibility
hard_coding_policy

This makes a feature independently completable.

---

80. "Done Means Done"

A domain file MUST be considered complete only when all required integration contracts have already been decided.

For every domain feature:

Specification              ✓
Grammar                    ✓
Lexical contract           ✓
AST mapping                ✓
Semantic mapping           ✓
Type integration           ✓
Effect integration         ✓
Resource integration       ✓
Capability integration     ✓
IR mapping                 ✓
Compiler integration       ✓
Runtime integration        ✓ if required
Diagnostics                ✓
Positive tests             ✓
Negative tests             ✓
Boundary tests             ✓
Scalability tests          ✓
Cross-domain tests         ✓
Compatibility tests        ✓
Determinism review         ✓
Hard-coding audit          ✓
Security review            ✓
Documentation              ✓

Only then is the feature complete.

---

81. Required Domain Directory Integration

The following repository directories are the intended domain ownership boundaries.

grammar/classical/
    Classical computation

grammar/quantum/
    Quantum computation

grammar/hybrid/
    Quantum-classical and heterogeneous hybrid computation

grammar/hdl/
    Hardware description

grammar/hardware/
    Hardware intent and hardware/software co-design

grammar/distributed/
    Distributed computation

grammar/ai/
    Artificial intelligence and machine learning

grammar/data/
    Data, tensors, streams, schemas and pipelines

grammar/networking/
    Network computation and communication

grammar/security/
    Security and trust

grammar/resources/
    Resource and capability semantics

grammar/compile/
    Compilation and deployment intent

grammar/execution/
    Execution and runtime intent

grammar/interoperability/
    Foreign representations

grammar/dialects/
    Controlled language extensions

These directories MUST integrate with this specification.

---

82. Existing Files Must Not Be Unnecessarily Renamed

The following existing authority files remain:

grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/DESIGN.md
grammar/specification/README.md
grammar/specification/language.md
grammar/specification/portability.md
grammar/spec/type-system.md

This file adds the missing domain-level normative contract.

No rename is required merely to satisfy this specification.

---

83. Relationship to Zamani.g4

"grammar/Zamani.g4" remains the canonical ANTLR composition root.

It MUST represent the accepted syntax defined by this specification.

Domain rules MUST NOT silently appear in "Zamani.g4" without corresponding normative specification.

Conversely, a feature specified as stable MUST eventually be represented by the canonical grammar if it requires source syntax.

The relationship is:

domains.md
     ↓
domain specifications
     ↓
domain grammar contracts
     ↓
Zamani.g4

not:

Zamani.g4
     ↓
whatever happens to parse

---

84. Relationship to grammar.md

"grammar/grammar.md" remains the implementation-conformance reference.

It MUST report domain status such as:

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

A domain may therefore be fully specified but not yet implemented.

That state is legitimate and MUST be visible.

---

85. Relationship to Zamani-Grammar.md

"grammar/Zamani-Grammar.md" remains the broader design/reference document.

It may contain:

- future domains;
- Sankofa;
- MTS;
- nano;
- advanced AI;
- speculative computation;
- NIMBUS;
- Universal-Trinity;
- experimental syntax.

However:

«Presence in "Zamani-Grammar.md" does not make a domain part of stable Zamani.»

Promotion follows the standard lifecycle.

---

86. Relationship to "grammar/spec/"

"grammar/spec/" provides subordinate contracts.

Relevant contracts include:

spec/lexical.md
spec/syntax.md
spec/type-system.md
spec/semantics.md
spec/resources.md
spec/portability.md
spec/compatibility.md

This file MUST NOT duplicate those specifications.

Instead:

domains.md
     ↓
domain-specific application
     ↓
general contract in spec/

If a domain exposes a contradiction with a general specification, the contradiction MUST be resolved rather than silently overriding the general rule.

---

87. Relationship to Types

Domain types MUST use the universal type architecture.

Examples:

Qubit
Qubit[n]
Tensor<T, Shape>
Resource<T>
Capability<T>
HardwareIntent<T>

These are semantic types.

They MUST NOT directly imply:

physical address
register width
device ID
memory bank
hardware topology

unless explicitly defined as target-specific types.

---

88. Relationship to Effects

Domain-specific effects MUST use the universal effect architecture.

For example:

quantum.measurement
network.io
device.io
distributed.communication
randomness

must be representable through the existing effect model.

No domain may create a second incompatible effect system.

---

89. Relationship to Resources

Domain resource requirements MUST use the common resource architecture.

Examples:

quantum resources
memory
compute
communication
energy
latency
reliability
availability

A domain-specific resource MUST identify its semantic unit and compatibility behavior.

---

90. Relationship to Capabilities

Domain capabilities MUST be composable.

For example:

quantum.measurement
tensor.compute
network.streaming
hardware.reconfiguration

may coexist in one program.

Capability resolution occurs against the target environment.

---

91. Relationship to Modules

Domains MUST work inside the ordinary module system.

A module may contain multiple domains.

For example:

module scientific.quantum.ml

may contain:

- classical computation;
- tensor operations;
- AI operations;
- quantum operations.

No domain-specific module system should be invented.

---

92. Relationship to Concurrency

All domains that support concurrency MUST use the universal concurrency architecture.

Examples:

- distributed;
- networking;
- AI training;
- accelerator execution;
- hybrid quantum-classical computation;
- HDL simulation.

The domain may add semantic constraints but MUST NOT invent another task model unless explicitly specified.

---

93. Relationship to Memory

Domain memory models must integrate with:

ownership
borrowing
references
regions
address spaces
persistence
shared memory
distributed memory
accelerator memory
quantum memory

A domain MUST distinguish logical memory from physical memory.

---

94. Domain Security

Every domain must undergo a security review when it introduces:

- external communication;
- device access;
- code generation;
- dynamic loading;
- foreign calls;
- secrets;
- persistent state;
- remote execution;
- privileged capabilities.

No domain may bypass security checks through syntax.

---

95. Domain Determinism and Reproducibility

Where deterministic semantics are promised:

same program
+
same semantic inputs
+
same specified environment assumptions

must produce equivalent observable semantics regardless of optimization or target realization.

Where nondeterminism is intentional, the domain MUST specify its source.

---

96. Domain Interoperability With Future Hardware

A domain MUST be able to survive new hardware generations.

For example:

quantum

must not depend on today's QPU gate sets.

tensor

must not depend on today's accelerator instruction sets.

networking

must not depend on today's network hardware.

HDL

must not depend on one FPGA family.

This is essential for POCO-REAF.

---

97. Domain Negotiation

When a program is compiled for a target, the compiler/runtime may negotiate:

requirements
        +
capabilities
        +
resources
        +
constraints
        +
preferences
        +
hints
        ↓
target realization

The negotiation MUST NOT change the program's specified semantics.

---

98. Domain Fallback

Where explicitly permitted, a domain may provide alternative realizations.

Examples:

GPU tensor computation
        ↓
CPU tensor computation

or:

QPU quantum computation
        ↓
quantum simulator

or:

accelerator operation
        ↓
software implementation

Fallback MUST be explicitly allowed by semantic policy.

A fallback MUST NOT silently weaken correctness guarantees.

---

99. Domain Failure

Domain failure must be classified.

Possible classes include:

InvalidSyntax
InvalidType
InvalidEffect
MissingCapability
InsufficientResource
InvalidConstraint
UnsupportedTarget
UnavailableRuntime
InvalidInterop
SecurityViolation
UnsupportedDialect

The exact diagnostic types belong to the diagnostics architecture.

---

100. Domain Testing Architecture

Every domain MUST have:

tests/
├── positive/
├── negative/
├── boundary/
├── scalability/
├── determinism/
├── portability/
├── compatibility/
└── integration/

Cross-domain tests are mandatory for domains that interact.

Examples:

quantum + classical
quantum + AI
AI + data
AI + accelerator
HDL + hardware
distributed + networking
security + networking
data + distributed
classical + hardware

---

101. Domain Conformance Matrix

The repository should maintain a conformance matrix conceptually like:

Domain| Spec| Grammar| Lexer| Parser| AST| Semantics| IR| Compiler| Runtime| Tests| Stable
Classical| required| required| required| required| required| required| required| required| as needed| required| conditional
Quantum| required| required| required| required| required| required| "quantum::ir"| required| required| required| conditional
Hybrid| required| required| required| required| required| required| required| required| required| required| conditional
HDL| required| required| required| required| required| required| HDL/Hardware IR| required| as needed| required| conditional
Hardware| required| required| required| required| required| required| hardware semantic boundary| required| required| required| conditional
Distributed| required| required| required| required| required| required| required| required| required| required| conditional
AI| required| required| required| required| required| required| required| required| as needed| required| conditional
Data| required| required| required| required| required| required| required| required| as needed| required| conditional
Networking| required| required| required| required| required| required| required| required| required| required| conditional
Security| required| required| required| required| required| required| required| required| required| required| conditional

"Stable" MUST never be inferred merely from "Grammar".

---

102. Domain Hard-Coding Audit

Before a domain becomes stable, inspect:

grammar
lexer
parser
AST
semantic analyzer
IR
compiler
runtime
tests

for artificial constants involving:

qubits
cores
CPUs
GPUs
FPGAs
QPUs
nodes
threads
memory
storage
tensor dimensions
tensor rank
vector widths
register widths
agents
timelines
devices
accelerators
network links

Any such constant MUST be classified as one of:

1. ordinary program data;
2. target capability;
3. runtime resource budget;
4. implementation resource budget;
5. semantic requirement.

If it is an artificial language ceiling, it MUST be removed.

---

103. Repository-Wide Scalability Requirement

The domain specification is only successful if the entire implementation respects the same scalability principle.

Therefore, domain work MUST NOT introduce a limit in:

grammar/
src/lexer.rs
src/parser.rs
src/frontend/ast/
semantic analysis
IR
compiler
runtime

that contradicts the domain specification.

For example, removing "MAX_QUBITS" from grammar while retaining an equivalent hard-coded limit in the semantic analyzer does not satisfy POCO-REAF.

---

104. Representation Limits

Implementations necessarily have finite representations.

For example, an implementation may currently use a particular integer representation.

Such implementation constraints MUST remain implementation constraints until the language specification explicitly adopts them.

The implementation MUST NOT silently promote its current representation limit into a language semantic limit.

---

105. Safe Rust Requirement

All repository implementation work associated with these domain contracts MUST remain compatible with:

Rust 1.97
Rust 1.97.1
Rust 2021
safe Rust

No domain integration may require Rust "unsafe".

If a dependency requires unsafe internally, that dependency must be evaluated under the repository's safety policy before being accepted as a production dependency.

---

106. Domain Performance

Performance optimization MUST NOT alter semantic meaning.

The implementation SHOULD use:

- iterative algorithms where deep structures are possible;
- explicit worklists;
- streaming;
- incremental processing;
- lazy evaluation where semantically appropriate;
- resource-aware compilation;
- deterministic caching where required.

Performance budgets MUST NOT become language-level domain limits.

---

107. Domain Memory Safety

Domain implementations MUST preserve:

- ownership;
- lifetime;
- resource lifetime;
- capability lifetime;
- deterministic cleanup;
- safe concurrency.

Physical resource lifetime MUST remain distinct from logical ownership.

---

108. Domain Extensibility

A new domain may be added without changing existing domain semantics if it can integrate through:

existing lexical model
existing syntax architecture
existing AST
existing types
existing effects
existing resources
existing capabilities
existing module system
existing diagnostics
existing semantic architecture
existing IR architecture
existing compatibility model

If it cannot, the architectural change must be specified first.

---

109. New Domain Checklist

Before adding a new domain, answer:

What computation does it represent?
What does it own?
What does it not own?
Which universal types does it use?
Which operations does it introduce?
Which effects does it introduce?
Which resources does it require?
Which capabilities does it require?
How does it compose with existing domains?
What is its AST representation?
What is its semantic representation?
What is its IR boundary?
Does it require runtime support?
Does it require target support?
Does it require scheduling?
Does it require routing?
Does it require resilience?
Does it interact with ZQN?
Does it interact with HAL?
How does it scale?
What are its failure modes?
How is it versioned?
How is it tested?
How is hard-coding prevented?

---

110. Domain Completion Criteria

A domain is production-ready only when:

[ ] Purpose defined
[ ] Ownership defined
[ ] Non-ownership defined
[ ] Normative specification complete
[ ] Syntax contract complete
[ ] Lexer contract complete
[ ] Parser contract complete
[ ] AST contract complete
[ ] Type contract complete
[ ] Effect contract complete
[ ] Resource contract complete
[ ] Capability contract complete
[ ] Semantic contract complete
[ ] IR contract complete
[ ] Compiler integration complete
[ ] Runtime integration complete where required
[ ] Backend integration complete where required
[ ] Diagnostics complete
[ ] Positive tests complete
[ ] Negative tests complete
[ ] Boundary tests complete
[ ] Scalability tests complete
[ ] Cross-domain tests complete
[ ] Portability tests complete
[ ] Determinism review complete
[ ] Compatibility review complete
[ ] Security review complete
[ ] Hard-coding audit complete
[ ] Documentation complete

Only then may the feature be marked:

STABLE

---

111. Final Domain Architecture

The final Zamani domain architecture is:

                         ZAMANI
                            │
             ┌──────────────┼──────────────┐
             │              │              │
          Universal     Domain          Extension
         Foundations   Computation       Mechanisms
             │              │              │
      ┌──────┼──────┐   ┌───┼───────┐   ┌─┼─────────┐
      │      │      │   │   │       │   │ │         │
    Types  Effects Memory Classical Quantum HDL    Dialects
      │      │      │     │     │       │     Macros
      │      │      │     │     │       │     Metaprogramming
      └──────┼──────┴─────┼─────┼───────┘
             │            │
             ▼            ▼
        Capabilities   Resources
             │            │
             └──────┬─────┘
                    ▼
          Canonical Semantic Model
                    │
       ┌────────────┼─────────────┐
       ▼            ▼             ▼
 Classical      quantum::ir    HDL/Hardware
    IR             │               IR
       │            │               │
       └────────────┼───────────────┘
                    ▼
              Optimization
                    │
       ┌────────────┼────────────┐
       ▼            ▼            ▼
    Routing      Scheduling   Resilience
       │            │            │
       └────────────┼────────────┘
                    ▼
                   ZQN
                    │
                   HAL
                    │
             Target realization
                    │
       ┌────────────┼──────────────┐
       ▼            ▼              ▼
      CPU          GPU            FPGA
       │            │              │
       └────────────┼──────────────┘
                    ▼
                   QPU
                    │
                    ▼
             Future targets

---

112. Fundamental Invariant

The central invariant of the entire domain architecture is:

«Zamani source code describes computational meaning, requirements, guarantees, capabilities, constraints, preferences, and permitted execution strategies. It does not unnecessarily describe the physical machine on which that meaning is realized.»

Therefore:

PROGRAM
   ↓
SEMANTIC INTENT
   ↓
CAPABILITIES + RESOURCES + CONSTRAINTS
   ↓
CANONICAL SEMANTIC MODEL
   ↓
CANONICAL IR
   ↓
OPTIMIZATION
   ↓
ROUTING / SCHEDULING / RESILIENCE
   ↓
ZQN
   ↓
HAL
   ↓
TARGET REALIZATION

This is the mechanism that protects:

Program Once
Compile Once
Run Everywhere
Anywhere
Forever

while allowing the same language to scale from:

atom
nano
embedded
single value
single operation
single processor
single qubit
single accelerator

through:

classical
quantum
hybrid
HPC
AI
distributed
cloud
edge
heterogeneous
hardware/software co-design

to:

arbitrarily large logical computations

subject only to the actual semantics, representation capacity, compiler/runtime resources, target capabilities, physical constraints, and resources available.

---

113. Non-Negotiable Rules

The following rules are normative:

1. One Zamani language.
2. One canonical language specification.
3. "grammar/Zamani.g4" remains the canonical ANTLR composition root.
4. "grammar/grammar.md" remains the implementation-conformance reference.
5. "grammar/Zamani-Grammar.md" remains non-authoritative design/reference material.
6. Domain directories are ownership boundaries, not separate languages.
7. All domains share the universal frontend architecture.
8. The AST remains domain-neutral.
9. Quantum semantics terminate at "quantum::ir".
10. No second quantum IR.
11. QEC remains a QEC responsibility.
12. ZQN remains the fault/noise semantic subsystem.
13. Routing remains responsible for physical realization.
14. Scheduling remains responsible for execution ordering/resource timing.
15. HAL remains responsible for target capabilities/state.
16. No domain may hard-code universal machine limits.
17. Resource requirements are not physical mappings.
18. Capabilities are not device identifiers.
19. Preferences are not requirements.
20. Hints do not change semantics.
21. Target realization remains downstream.
22. Frameworks and vendors do not become universal language keywords merely because they exist.
23. New operations should normally use generic semantic operation mechanisms rather than ever-growing keyword lists.
24. Every domain feature requires an AST contract before implementation is considered complete.
25. Every domain feature requires an IR contract before it is production-ready.
26. Every domain feature requires positive, negative, boundary, scalability, portability, determinism, and compatibility tests.
27. Safe Rust only.
28. Rust 1.97 / 1.97.1 and Rust 2021 remain the implementation baseline.
29. No Rust "unsafe" in production compiler implementation.
30. "Infinity" means no artificial language ceiling, not physically infinite hardware.
31. A target that lacks sufficient resources may reject a program; it must not silently change its semantics.
32. Future computational domains must be able to integrate through the existing semantic architecture.
33. No domain may create an unnecessary parallel compiler, parser, AST, effect system, resource system, or semantic model.
34. A feature is complete only when its entire downstream integration contract is complete.
35. The language must remain extensible without becoming a catalogue of today's hardware, vendors, algorithms, or frameworks.

---

114. Completion Statement

"grammar/specification/domains.md" is complete when it serves as the normative domain integration contract for the entire Zamani language and when every domain directory can be implemented against this contract without needing to reinterpret the meaning of:

- domain ownership;
- AST integration;
- semantic integration;
- IR integration;
- resource semantics;
- capability semantics;
- portability;
- scalability;
- target realization;
- compiler integration;
- runtime integration;
- compatibility;
- safety.

No individual domain specification should need to redefine these principles.

The resulting architecture is therefore:

ONE LANGUAGE
      +
ONE SPECIFICATION AUTHORITY
      +
ONE DOMAIN-NEUTRAL FRONTEND
      +
ONE SHARED TYPE/EFFECT/RESOURCE/CAPABILITY MODEL
      +
CANONICAL DOMAIN IR BOUNDARIES
      +
TARGET-INDEPENDENT SEMANTICS
      +
TARGET-AWARE LOWERING
      +
NO ARTIFICIAL RESOURCE CEILINGS
      +
SAFE RUST
      +
FULL CONFORMANCE TESTING
      =
POCO-REAF

End of "grammar/specification/domains.md".