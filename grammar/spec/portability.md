Zamani Portability Semantic Specification

Path: "grammar/spec/portability.md"
Language: Zamani
Specification role: Normative portability, target-independence, scalability, adaptation, and POCO-REAF contract
Status: Production / Normative
Specification version: 4.0
Grammar technology: ANTLR4
Implementation baseline: Rust 1.97 / Rust 1.97.1, Rust 2021
Safety requirement: Safe Rust only; production Zamani implementation MUST NOT use Rust "unsafe"
Primary objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Scalability objective: From the smallest supported computation to arbitrarily large realizations subject only to actual available resources and explicitly declared semantic requirements
Canonical quantum semantic boundary: "quantum::ir"
Canonical grammar composition root: "grammar/Zamani.g4"

---

0. Purpose

This document defines the normative semantic contract for portability in Zamani.

Portability is not merely the ability of a compiler to emit binaries for several machines.

In Zamani, portability is a property of the program's semantic representation.

A portable Zamani program describes:

- computation;
- data;
- types;
- control flow;
- effects;
- correctness;
- resource requirements;
- resource constraints;
- required capabilities;
- acceptable adaptations;
- interoperability requirements;
- execution intent;

without unnecessarily making those meanings depend on:

- a particular processor;
- a particular accelerator;
- a particular QPU;
- a particular FPGA;
- a particular ASIC;
- a particular memory device;
- a particular machine;
- a particular physical topology;
- a particular number of workers;
- a particular number of nodes;
- a particular physical qubit mapping;
- a particular deployment location.

The fundamental rule is:

«Zamani source semantics describe what the program means; implementation layers determine how and where that meaning is realized.»

This separation is the foundation for:

Program Once → Compile Once → Run Everywhere, Anywhere, Forever.

---

1. Normative Language

The keywords below have normative meaning.

- MUST — mandatory.
- MUST NOT — prohibited.
- REQUIRED — mandatory.
- SHOULD — recommended unless a documented reason exists not to do so.
- SHOULD NOT — discouraged unless a documented reason exists.
- MAY — permitted but optional.
- CAN — capability statement, not a requirement.

A compiler, frontend, runtime, backend, or tool that violates a MUST/MUST NOT rule is not conformant with this specification.

---

2. File Completion Contract

This file is considered complete only when all of the following contracts are established.

2.1 Purpose

Define the semantic meaning of portability and POCO-REAF.

2.2 Owns

This file owns:

- portability semantics;
- target independence;
- realization independence;
- portability guarantees;
- portability requirements;
- portability constraints;
- portability preferences;
- portability hints;
- portability domains;
- portability dimensions;
- semantic adaptation;
- resource-independent scaling;
- compile-once semantics;
- target-specific derivation rules;
- logical-versus-physical separation;
- portability diagnostics;
- portability conformance;
- portability compatibility requirements.

2.3 Does Not Own

This file does not own:

- lexical token definitions;
- general expression syntax;
- general type syntax;
- parser implementation;
- AST implementation;
- hardware discovery;
- physical allocation;
- scheduling algorithms;
- routing algorithms;
- optimization algorithms;
- calibration;
- QEC implementation;
- ZQN implementation;
- HAL implementation;
- vendor APIs;
- device drivers;
- physical deployment;
- canonical quantum IR;
- classical IR;
- HDL/hardware IR.

Those systems consume the contracts defined here.

2.4 Inputs

The semantic portability model consumes:

- source-level portability intent;
- resource requirements;
- resource constraints;
- capability requirements;
- execution context;
- target capabilities;
- deployment policy;
- compiler policy;
- runtime availability;
- domain semantics.

2.5 Outputs

Portability analysis produces:

- normalized portability intent;
- portability guarantees;
- target-independence classification;
- portability requirements;
- adaptation requirements;
- implementation constraints;
- portability diagnostics;
- information consumed by resource analysis, compilation, optimization, scheduling, routing, HAL, runtime, and deployment.

2.6 Dependencies

This file depends semantically on:

- "grammar/spec/lexical.md";
- "grammar/spec/syntax.md";
- "grammar/spec/semantics.md";
- "grammar/spec/type-system.md";
- "grammar/spec/effects.md";
- "grammar/spec/resources.md";
- "grammar/spec/compatibility.md";
- "grammar/specification/language.md";
- "grammar/specification/portability.md";
- "grammar/DESIGN.md".

2.7 Upstream Contracts

Upstream syntax is defined by:

- "grammar/resources/portability.g4";
- canonical resource-expression grammar;
- "grammar/Zamani.g4".

The portability grammar MUST NOT define an independent expression language.

2.8 Downstream Consumers

Consumers include:

- frontend semantic analysis;
- resource analysis;
- capability analysis;
- type/effect analysis;
- compiler;
- optimizer;
- target selection;
- specialization;
- routing;
- scheduling;
- resilience;
- QEC;
- ZQN;
- HAL;
- runtime;
- deployment;
- interoperability tooling;
- diagnostics;
- conformance tooling.

---

3. Authority and Repository Integration

The repository MUST maintain one semantic authority for portability.

The intended authority relationship is:

grammar/specification/portability.md
        │
        │ human-readable normative language semantics
        ▼
grammar/spec/portability.md
        │
        │ formal semantic contract
        ▼
grammar/resources/portability.g4
        │
        │ syntax
        ▼
grammar/Zamani.g4
        │
        │ composition
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
structural validation
        │
        ▼
semantic analysis
        │
        ├── type analysis
        ├── effect analysis
        ├── resource analysis
        ├── capability analysis
        ├── portability analysis
        └── correctness analysis
        │
        ▼
canonical semantic representation
        │
        ├── classical semantics
        ├── quantum semantics
        ├── HDL/hardware semantics
        ├── hybrid semantics
        ├── distributed semantics
        ├── AI/data semantics
        └── other domain semantics
        │
        ▼
canonical/domain IR
        │
        ├── classical IR
        ├── quantum::ir
        └── HDL/hardware IR
        │
        ▼
optimization / decomposition / lowering
        │
        ├── routing
        ├── scheduling
        ├── resilience
        ├── QEC
        └── ZQN
        │
        ▼
HAL / target realization
        │
        ▼
runtime / deployment

3.1 Relationship with "grammar/specification/portability.md"

"grammar/specification/portability.md" provides the language-wide explanatory specification.

This file provides the formal semantic contract used to integrate portability with the grammar, AST, resource system, compiler, IR, runtime, and conformance system.

Neither file may introduce contradictory meanings.

3.2 Relationship with "grammar/spec/resources.md"

"grammar/spec/resources.md" owns general resource semantics.

This file owns the portability consequences of those resources.

For example:

resources.md
    resource requirement:
        requires memory >= required_memory

portability.md
    semantic portability:
        the requirement remains meaningful across
        different memory implementations.

Resource semantics MUST NOT be duplicated here.

3.3 Relationship with "grammar/resources/portability.g4"

"grammar/resources/portability.g4" owns portability syntax.

This file owns the meaning of that syntax.

The parser MUST NOT infer hardware feasibility.

3.4 Relationship with "grammar/Zamani.g4"

"grammar/Zamani.g4" remains the canonical grammar composition root.

This document MUST NOT create another root grammar.

3.5 Relationship with "grammar/grammar.md"

"grammar/grammar.md" remains the implementation-conformance reference.

It MUST distinguish:

- specified;
- implemented;
- partially implemented;
- planned;
- deprecated.

Portability syntax MUST NOT be considered implemented merely because it appears in a grammar file.

3.6 Relationship with "grammar/Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" may contain broader historical, experimental, or aspirational portability concepts.

It MUST NOT silently promote those concepts to normative syntax.

Promotion requires:

proposal
→ semantic definition
→ AST contract
→ grammar contract
→ implementation
→ IR integration
→ conformance tests
→ compatibility decision
→ stable feature

---

4. Core Definition of Portability

Portability is the preservation of program semantics across permitted variations in realization.

Let:

P = Zamani program semantics
E = execution environment
R = realization strategy
O = observable program behavior

A valid realization is conceptually:

Realize(P, E, R) → O

Two realizations are portable equivalents when their observable behavior satisfies the same semantic contract:

Realize(P, E₁, R₁) ≡semantic Realize(P, E₂, R₂)

The realizations MAY differ in:

- instructions;
- scheduling;
- placement;
- routing;
- decomposition;
- parallelism;
- memory placement;
- device selection;
- communication topology;
- accelerator selection;
- physical quantum mapping;
- execution order where ordering is not semantically constrained;
- optimization;
- deployment.

They MUST NOT differ in a way that violates required program semantics.

---

5. Portability Is Not Uniformity

Portability does not mean that every target behaves internally in exactly the same way.

A CPU, GPU, FPGA, QPU, cluster, and future accelerator may require completely different implementation strategies.

For example:

same Zamani semantic program
        │
        ├── CPU lowering
        ├── GPU lowering
        ├── FPGA lowering
        ├── QPU lowering
        ├── distributed lowering
        └── future-target lowering

These implementations MAY be radically different.

The requirement is that they preserve the source-level semantic contract.

Therefore:

portable semantics
        ≠
identical implementation

---

6. POCO-REAF

6.1 Definition

POCO-REAF means:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever.»

The goal is that developers express computation once and do not need to rewrite the program merely because its realization changes.

6.2 Required interpretation

POCO-REAF means:

1. source semantics are target-independent by default;
2. target realization is derived;
3. physical resource identities are not silently embedded into portable semantics;
4. available capabilities determine feasible realizations;
5. target-specific transformations occur downstream;
6. source rewriting is not required merely to move to another compatible realization;
7. scalability is determined by available resources rather than universal grammar ceilings.

6.3 Compile-once boundary

The architecture SHOULD permit:

source
  ↓
lex
  ↓
parse
  ↓
AST
  ↓
semantic analysis
  ↓
canonical semantic representation
  ↓
canonical IR
  ↓
portable compilation artifact
  ↓
target-specific realization

The portable artifact MAY subsequently be specialized for:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- simulator;
- distributed cluster;
- cloud;
- edge;
- embedded target;
- future architecture.

The specialization MUST remain a derived transformation.

---

7. Meaning of "Everywhere"

"Everywhere" does not mean that every program is executable on every possible device.

A program may have genuine semantic requirements.

For example:

requires capability("quantum.measurement");

cannot be executed on an environment that has no mechanism capable of satisfying that requirement.

Therefore:

«Portability means that a program does not unnecessarily depend on a target, not that impossible requirements become possible.»

A conforming implementation MUST distinguish:

portable but currently unsatisfied

from:

intrinsically target-dependent

and from:

semantically impossible

---

8. Meaning of "Forever"

"Forever" is an architectural objective, not a promise that physical hardware or compiler implementations never change.

Future implementations SHOULD be able to consume the stable semantic contract.

A future backend MUST be able to derive a realization from:

- source semantics;
- canonical IR;
- resource requirements;
- capability descriptions;
- portability constraints;
- compatibility metadata.

A future target SHOULD NOT require changes to the source program merely because it uses a different physical architecture.

---

9. Target Independence

Portable source SHOULD express:

- what computation must occur;
- what values must be produced;
- what relationships must hold;
- what capabilities are required;
- what resources are required;
- what resource constraints apply;
- what effects are permitted;
- what correctness guarantees apply;
- what performance properties are preferred;
- what portability guarantees are required.

Portable source SHOULD NOT unnecessarily encode:

- CPU IDs;
- GPU IDs;
- QPU IDs;
- FPGA IDs;
- node IDs;
- physical qubit IDs;
- physical memory addresses;
- PCI addresses;
- fixed cache assumptions;
- fixed topology;
- fixed accelerator counts;
- vendor-specific deployment identities.

---

10. Explicit Target Dependence

Zamani MAY support explicitly target-dependent source.

Such source MUST be distinguishable from portable source.

Target-dependent constructs MUST be represented as an explicit semantic boundary.

Conceptually:

portable program
        │
        ├── target-independent semantics
        │
        └── explicit target-dependent extension

A target-dependent declaration MUST NOT silently contaminate the rest of the program.

The semantic model SHOULD therefore preserve:

portable
target-constrained
target-specific
non-portable

as distinguishable classifications.

---

11. Portability Classification

Every relevant resource or implementation dependency SHOULD be classifiable as one of:

Classification| Meaning
"portable"| No target-specific dependency
"resource-dependent"| Depends on abstract resource availability
"capability-dependent"| Requires an abstract capability
"scale-dependent"| Resource quantity varies with workload
"target-constrained"| Valid only for a declared target class/property
"target-specific"| Explicitly tied to a target implementation
"implementation-defined"| Chosen downstream without changing semantics
"non-portable"| Cannot be preserved across the requested portability boundary

A portability analysis MUST NOT hide these distinctions.

---

12. Resource Independence

Resource quantities are semantic values.

They MAY be:

- constants;
- variables;
- symbolic;
- parameterized;
- input-dependent;
- data-dependent;
- compile-time derived;
- runtime derived;
- negotiated.

Example:

let n = input_size();
requires qubits >= n;

does not establish:

MAX_QUBITS = 1024

Likewise:

requires memory >= required_memory;

does not establish a universal memory ceiling.

---

13. No Artificial Resource Limits

The Zamani language MUST NOT define artificial resource ceilings such as:

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
MAX_PROCESSES
MAX_CHANNELS
MAX_DEVICES

The same prohibition applies to disguised equivalents.

For example, a parser rule that accepts only:

q0
q1
...
q127

is a hard-coded semantic limit even if it does not contain a constant named "MAX_QUBITS".

---

14. Program Constants Are Not Language Limits

The prohibition on hard-coded implementation limits does not prohibit program constants.

This is valid:

let n = 1024;

This is also valid:

tensor<float, 1024, 1024>

if those values are part of program semantics.

What is prohibited is imposing the value as a language implementation limit:

the compiler rejects every tensor dimension > 1024

when "1024" is not a semantic requirement of the language or program.

---

15. Tiny-to-Large Scaling

The same language model MUST support computation ranging from:

one value

through:

small embedded workload
single-machine workload
multicore workload
accelerated workload
heterogeneous workload
distributed workload
cluster workload
supercomputer workload
cloud workload
future large-scale workload

without introducing separate language semantics for each scale.

Scale MUST be represented through:

- data;
- resource intent;
- concurrency;
- parallelism;
- distribution;
- capabilities;
- workload properties;
- execution policies.

---

16. "Infinity" Semantics

The word "infinity" in the portability model means:

«The language architecture imposes no artificial finite upper bound on scale.»

It does not mean that physical machines contain infinite resources.

Actual execution remains bounded by:

- available hardware;
- available memory;
- available storage;
- compiler resources;
- runtime resources;
- deployment policy;
- quotas;
- energy;
- time;
- device capabilities;
- network capacity;
- other declared constraints.

Therefore:

language semantic capacity

MUST NOT be confused with:

physical execution capacity

---

17. Resource Availability

Execution feasibility is determined after semantic analysis.

Conceptually:

program requirements
        +
target capabilities
        +
resource availability
        +
deployment policy
        +
runtime state
        ↓
feasibility

A target MAY fail to satisfy a requirement.

The implementation MUST distinguish at least:

1. unsupported capability;
2. insufficient capacity;
3. temporarily unavailable resource;
4. policy-prohibited resource;
5. unavailable deployment;
6. incompatible target;
7. unsatisfiable constraint;
8. runtime resource exhaustion.

Failure MUST NOT silently change the program's semantics.

---

18. Requirements, Constraints, Capabilities, Preferences, and Hints

These categories MUST remain distinct.

Category| Meaning| Binding?| Primary consumer
Requirement| Must be satisfied| Yes| semantic/compiler/runtime
Constraint| Restricts valid realization| Yes| compiler/runtime
Capability| Ability provided by environment| Context-dependent| HAL/environment
Preference| Desired realization| No| optimizer/scheduler
Hint| Optional guidance| No| optimizer/runtime
Implementation decision| Actual realization| Downstream| compiler/backend/runtime

A preference MUST NOT silently become a requirement.

A hint MUST NOT silently become a constraint.

An implementation decision MUST NOT silently become a source-level requirement.

---

19. Capability-Based Portability

Portability SHOULD be expressed in terms of capabilities rather than vendors.

Prefer:

requires capability("quantum.mid_circuit_measurement");

over:

requires vendor_qpu_X;

Prefer:

requires capability("tensor.compute");

over:

requires gpu_device_3;

Prefer:

requires capability("distributed.communication");

over:

requires node_17;

Capability namespaces MUST remain extensible.

The grammar MUST NOT need to be rewritten merely because a future capability is introduced.

---

20. Logical Resources Versus Physical Resources

The semantic model MUST distinguish:

logical resource

from:

physical resource

Examples:

logical_qubit

is not inherently:

physical_qubit(17)

Likewise:

logical_worker

is not inherently:

node(8)

Likewise:

accelerator

is not inherently:

GPU(3)

Likewise:

memory

is not inherently:

address(0x...)

Physical mapping belongs downstream.

---

21. Resource Adaptation

A portable program MAY permit implementation adaptation.

Examples include:

- changing parallelism;
- changing memory placement;
- changing accelerator selection;
- changing communication topology;
- changing task partitioning;
- changing scheduling;
- changing instruction selection;
- changing quantum decomposition;
- changing physical qubit mapping;
- changing HDL implementation strategy.

An adaptation is valid only if it preserves mandatory semantics.

---

22. Semantic Preservation

A compiler MAY optimize or transform a program.

It MUST preserve all semantics that are not explicitly declared adaptable.

These include, where applicable:

- observable outputs;
- declared side effects;
- type meaning;
- ownership guarantees;
- ordering guarantees;
- synchronization guarantees;
- quantum measurement semantics;
- quantum/classical interaction semantics;
- HDL timing semantics;
- security requirements;
- resource requirements;
- explicit correctness constraints.

An optimization MUST NOT be justified merely by saying:

the target cannot implement the original semantics

unless the program explicitly permitted that adaptation.

---

23. Determinism and Portability

Portability MUST NOT silently introduce nondeterminism.

If a program requires deterministic behavior, target-specific adaptation MUST preserve that requirement.

If a program explicitly permits nondeterministic execution, the permitted nondeterminism MUST be represented semantically.

Therefore:

different target

does not automatically mean:

different valid result

The distinction belongs to the language semantics.

---

24. Reproducibility

A portable build SHOULD preserve provenance sufficient to identify:

- source version;
- language version;
- feature versions;
- dialects;
- semantic assumptions;
- resource requirements;
- target-independent compiler stages;
- target-specific realization choices;
- relevant optimization decisions.

A target-specific artifact MUST remain traceable to the portable semantic program from which it was derived.

---

25. Classical Computing Portability

Classical programs MUST NOT inherently depend on:

- processor count;
- core count;
- thread count;
- register count;
- register width;
- cache size;
- SIMD width;
- instruction-set extension;
- physical memory layout.

For example:

parallel compute(data)

MAY execute using:

one worker

or:

many workers

provided the declared semantics remain satisfied.

The compiler MAY choose:

- scalar execution;
- vectorization;
- multithreading;
- task parallelism;
- GPU acceleration;
- distributed execution.

Those choices are downstream.

---

26. Quantum Portability

Quantum programs MUST be designed around logical semantics rather than physical device identity.

Portable quantum semantics MAY express:

- logical qubits;
- logical registers;
- operations;
- parameters;
- controls;
- adjoints;
- measurements;
- resets;
- observables;
- channels;
- noise intent;
- error-correction requirements;
- fault-tolerance requirements;
- capability requirements;
- resource requirements.

They MUST NOT silently assume:

- fixed physical qubit IDs;
- fixed topology;
- all-to-all connectivity;
- fixed gate set;
- fixed coherence time;
- fixed measurement technology;
- fixed calibration;
- fixed device count.

---

27. Quantum Operation Portability

Quantum syntax MUST support extensible operation semantics.

The architecture MUST NOT depend on a permanently closed grammar such as:

X | Y | Z | H | CNOT | ...

A portable quantum operation model MUST be capable of representing:

- named operations;
- parameterized operations;
- controlled operations;
- adjoint operations;
- composed operations;
- custom operations;
- domain-defined operations;
- future operations;
- interoperable operations.

The semantic path is:

quantum source
    ↓
domain-neutral AST
    ↓
semantic quantum operation
    ↓
quantum::ir
    ↓
optimization/decomposition
    ↓
routing
    ↓
scheduling
    ↓
QEC/resilience
    ↓
ZQN
    ↓
HAL
    ↓
physical realization

"quantum::ir" remains the canonical quantum semantic boundary.

Portability MUST NOT introduce another quantum IR.

---

28. Quantum Physical Mapping

Physical quantum realization belongs to:

- routing;
- scheduling;
- HAL;
- backend;
- runtime;
- device-specific deployment.

The source language MUST NOT silently transform:

logical q[0]

into:

physical q[17]

unless an explicitly target-dependent contract requests such behavior.

The compiler MUST NOT infer a universal physical mapping from source syntax.

---

29. Quantum QEC and ZQN Boundaries

Portability MAY express requirements such as:

requires capability("quantum.error_correction");
requires capability("quantum.fault_tolerance");

or equivalent resource/capability intent.

Portability does not implement QEC.

Responsibilities remain:

Subsystem| Responsibility
Grammar| Syntax
Portability semantics| Target-independent intent
"quantum::ir"| Canonical quantum semantics
Optimization| Implementation improvement
Routing| Physical realization
Scheduling| Timing/order/resources
QEC| Error detection/correction
ZQN| Fault/noise semantics
HAL| Device capability/state
Resilience| Recovery/orchestration

---

30. HDL and Hardware Portability

HDL/co-design programs SHOULD describe hardware intent rather than unnecessarily binding the program to one physical implementation.

Portable hardware semantics MAY describe:

- interfaces;
- signals;
- timing;
- state machines;
- pipelines;
- memory behavior;
- communication;
- accelerator intent;
- parameterized widths;
- resource requirements;
- synthesis constraints;
- verification properties.

They SHOULD NOT impose universal hardware limits such as:

MAX_PORTS = 32
MAX_BITS = 64
MAX_LANES = 16
MAX_MODULES = 128

unless such a value is explicitly part of the language's semantic model.

A target-specific FPGA/ASIC constraint belongs in the target realization layer.

---

31. Distributed Portability

Distributed programs MUST NOT require a fixed number of nodes unless the number itself is semantic program intent.

The language SHOULD permit:

workers >= required_workers

or equivalent abstract resource semantics.

It MUST distinguish:

requires at least N workers

from:

execute on node 17

Partitioning, placement, replication, routing, and node selection belong downstream.

---

32. AI and Data Portability

AI/data programs SHOULD describe:

- model semantics;
- tensor semantics;
- data semantics;
- training/inference intent;
- precision requirements;
- accelerator capabilities;
- memory requirements;
- parallelism;
- distribution;
- reproducibility requirements.

The grammar MUST NOT make a particular AI framework or accelerator vendor the semantic authority.

For example:

requires capability("tensor.compute");

is portable.

A hard-coded dependency on a particular vendor runtime is target-specific.

---

33. Networking Portability

Networking semantics SHOULD separate:

logical endpoint

from:

physical address

and:

logical channel

from:

physical link

A portable program may require:

- bandwidth;
- latency;
- reliability;
- protocol capability;
- secure communication;
- ordering;
- delivery guarantees.

Physical routing remains downstream.

---

34. Memory Portability

Memory semantics MUST distinguish:

- logical storage;
- address spaces;
- allocation;
- ownership;
- persistence;
- locality;
- capacity;
- bandwidth;
- latency;
- physical placement.

Portable source MUST NOT assume:

- a particular RAM size;
- a particular VRAM size;
- a fixed memory bank;
- a fixed address;
- a fixed cache hierarchy.

Compiler and runtime MAY choose:

- RAM;
- VRAM;
- accelerator memory;
- distributed memory;
- persistent storage;
- unified memory;
- other future memory systems.

---

35. Numeric Portability

Mathematical meaning MUST be distinguished from machine representation.

The implementation MAY select:

- scalar representation;
- vector representation;
- arbitrary precision;
- fixed precision;
- floating point;
- software emulation;
- accelerator representation.

Such selection MUST preserve declared semantics.

The language MUST NOT silently introduce target-dependent overflow behavior.

Where numeric behavior is target-dependent by design, the program MUST explicitly permit it.

---

36. Semantic Quantities Must Not Depend on Host "usize"

Resource quantities, dimensions, counts, capacities, and other semantic values MUST NOT acquire language-level limits merely because an implementation uses a host-sized representation.

The implementation MUST NOT treat:

usize

as the semantic definition of a resource quantity.

Where values exceed the implementation's direct representation, the implementation MUST use an appropriate checked, arbitrary-precision, symbolic, or otherwise semantically correct representation.

The implementation MUST NOT:

- truncate;
- wrap;
- silently saturate;
- reinterpret;
- reject solely because of a host integer width,

unless the language semantics explicitly require that behavior.

---

37. Safe Rust Requirement

Production implementation components associated with this contract MUST support:

- Rust 1.97 or Rust 1.97.1;
- Rust 2021;
- safe Rust only.

"unsafe" Rust MUST NOT be used.

Where practical, implementation crates SHOULD enforce:

#![forbid(unsafe_code)]

Resource and portability analysis MUST use:

- checked arithmetic;
- explicit overflow handling;
- deterministic data structures where ordering matters;
- explicit error propagation;
- no pointer-width assumptions;
- no host-layout assumptions.

The semantic specification itself MUST remain independent of Rust implementation details.

---

38. Portability and Effects

Effects are part of semantic portability.

A target transformation MUST preserve declared effects.

For example, if a program declares an effect requiring:

- deterministic execution;
- external I/O;
- persistent state;
- synchronization;
- measurement;
- secure execution;

a backend MUST NOT eliminate or change that effect merely to improve portability.

Effects and portability are integrated through semantic analysis, not by making portability a replacement for the effect system.

---

39. Portability and Types

Types define semantic constraints.

Portability analysis MUST preserve type meaning across targets.

A target MAY use different machine representations when the representations remain semantically equivalent.

Examples:

integer
tensor<T>
qubit
logical_qubit
resource<T>

must retain their language-defined meanings regardless of target representation.

Target-specific representations MUST NOT redefine the source type.

---

40. Portability and Concurrency

Concurrency portability requires separation between:

semantic concurrency

and:

execution mechanism

A program may express:

parallel

without requiring:

exactly 8 threads

unless exactly eight workers are semantically required.

The implementation MAY map semantic concurrency to:

- threads;
- tasks;
- actors;
- processes;
- vector units;
- GPU workgroups;
- distributed workers;
- quantum parallel operations;
- future execution models.

---

41. Portability and Scheduling

Scheduling is downstream.

Portability MAY specify:

- latency requirements;
- deadlines;
- ordering;
- throughput requirements;
- resource constraints;
- scheduling preferences.

Portability MUST NOT select exact execution times or machine slots unless explicitly requested as a target-specific contract.

The scheduling subsystem determines the realization.

---

42. Portability and Routing

Routing is downstream.

A portable program may require:

communication capability

or:

connectivity property

without specifying a physical route.

For quantum computation:

logical operation

MUST be separable from:

physical qubit route

For distributed computation:

logical communication

MUST be separable from:

physical network path

---

43. Portability and Optimization

Optimization MAY change implementation strategy.

It MUST NOT change mandatory semantics.

An optimizer MAY:

- vectorize;
- parallelize;
- fuse;
- tile;
- partition;
- distribute;
- replicate;
- decompose;
- reorder operations where permitted;
- select accelerators;
- select representations.

Optimization MUST honor:

- requirements;
- constraints;
- capabilities;
- effects;
- determinism;
- security;
- correctness;
- domain semantics.

---

44. Portability and Specialization

Specialization is permitted.

A specialization is valid when it is a derived realization of the same semantic program.

Examples:

generic algorithm
    ↓
CPU specialization

generic tensor computation
    ↓
GPU specialization

generic quantum algorithm
    ↓
QPU-specific decomposition

generic hardware design
    ↓
FPGA implementation

Specialization MUST preserve the source contract.

---

45. Compile-Time Versus Runtime Portability

Portability information MAY be resolved:

- at parse time;
- at semantic-analysis time;
- at compile time;
- at deployment time;
- at runtime.

The phase MUST NOT change the semantic category.

For example:

requires capability(...)

remains a requirement regardless of whether capability checking occurs during:

compile

or:

runtime

---

46. Dynamic Adaptation

Programs MAY permit dynamic adaptation.

Examples:

- acquire more workers;
- reduce parallelism;
- migrate workloads;
- change memory placement;
- select another accelerator;
- use another compatible execution strategy;
- recover from a failed device.

Dynamic adaptation MUST preserve all mandatory semantics.

If adaptation would violate a requirement, the runtime MUST report failure rather than silently changing the computation.

---

47. Graceful Degradation

A program MAY explicitly define acceptable degradation.

For example:

preferred capability
fallback capability
minimum acceptable capability

The distinction MUST be explicit.

A backend MUST NOT invent fallback semantics.

Conceptually:

required capability
        │
        ├── available → execute
        │
        └── unavailable
              │
              ├── declared fallback → attempt fallback
              │
              └── no fallback → fail

---

48. Portability and Interoperability

Interoperability formats such as:

- OpenQASM;
- QIR;
- HDL formats;
- foreign function interfaces;
- vendor formats;
- other future interchange formats;

are representations at boundaries.

They MUST NOT become the canonical Zamani semantic model.

The conversion path is:

Zamani semantics
    ↓
canonical/domain IR
    ↓
interoperability representation
    ↓
external system

not:

external representation
    ↓
new competing Zamani semantic model

---

49. Dialect Portability

Dialects MAY introduce additional semantics.

A dialect MUST declare:

- name;
- version;
- ownership;
- syntax extensions;
- semantic extensions;
- AST mapping;
- IR mapping;
- compatibility rules;
- portability impact.

A dialect MUST identify whether a feature is:

- portable;
- capability-dependent;
- target-constrained;
- target-specific;
- non-portable.

A dialect MUST NOT silently redefine core Zamani portability semantics.

---

50. Portability Boundaries

Every target-specific boundary SHOULD explicitly identify:

what remains portable
what becomes target-dependent
what information is introduced
what assumptions are introduced
what semantic guarantees remain

For example:

portable semantic program
        ↓
target specialization boundary
        ↓
target-specific realization

The boundary MUST be traceable.

---

51. Provenance

Every target-specific transformation SHOULD preserve provenance linking it back to:

- source span;
- AST node;
- semantic construct;
- canonical IR entity;
- portability contract;
- resource requirement;
- target capability;
- compiler transformation.

This permits diagnostics such as:

target cannot satisfy requirement declared here

rather than opaque backend failure.

---

52. Diagnostics

Portability diagnostics MUST distinguish semantic failure from implementation failure.

Recommended diagnostic categories include:

PORTABILITY_REQUIREMENT_UNSATISFIED
PORTABILITY_CONSTRAINT_UNSATISFIED
PORTABILITY_CAPABILITY_UNAVAILABLE
PORTABILITY_TARGET_INCOMPATIBLE
PORTABILITY_ADAPTATION_FORBIDDEN
PORTABILITY_NON_PORTABLE_DEPENDENCY
PORTABILITY_PHYSICAL_BINDING_REQUIRED
PORTABILITY_RESOURCE_UNAVAILABLE
PORTABILITY_RESOURCE_EXHAUSTED
PORTABILITY_FALLBACK_UNAVAILABLE
PORTABILITY_SEMANTIC_PRESERVATION_FAILED
PORTABILITY_DETERMINISM_VIOLATION
PORTABILITY_EFFECT_VIOLATION
PORTABILITY_TYPE_SEMANTICS_VIOLATION
PORTABILITY_PROVENANCE_MISSING
PORTABILITY_VERSION_INCOMPATIBLE
PORTABILITY_DIALECT_INCOMPATIBLE
PORTABILITY_TARGET_POLICY_REJECTED

Actual error identifiers MUST be reconciled with the repository's canonical diagnostic system rather than duplicated if equivalent identifiers already exist.

Every diagnostic SHOULD preserve:

- source span;
- semantic category;
- failed condition;
- affected resource/capability;
- target context where relevant;
- whether the failure is static or dynamic;
- remediation information.

---

53. Parser Responsibilities

The parser MUST:

- parse valid portability syntax;
- preserve source structure;
- preserve source spans;
- reject syntactically invalid constructs;
- remain independent of hardware availability.

The parser MUST NOT:

- query hardware;
- choose targets;
- allocate resources;
- perform routing;
- perform scheduling;
- perform QEC;
- perform ZQN analysis;
- inspect physical topology.

---

54. AST Contract

Every portability construct MUST have a predetermined mapping into the domain-neutral frontend AST before the grammar construct is considered complete.

The AST MUST preserve, where applicable:

- source span;
- portability classification;
- expression;
- domain;
- dimension;
- condition;
- scope;
- modifiers;
- attributes;
- requirement/constraint/preference/hint distinction;
- explicit target dependence;
- nested portability relationships;
- provenance.

The AST MUST remain domain-neutral.

It MUST NOT contain:

- vendor-specific hardware objects;
- physical qubit maps;
- routing schedules;
- QEC implementation;
- calibration state;
- device driver state.

---

55. Semantic Model Contract

After parsing, portability constructs MUST be normalized into a semantic model.

The semantic model SHOULD distinguish:

PortabilityIntent
PortabilityRequirement
PortabilityConstraint
PortabilityPreference
PortabilityHint
PortabilityGuarantee
PortabilityAdaptation
PortabilityDomain
PortabilityDimension
TargetDependency

These are semantic concepts, not a requirement to introduce identically named Rust structs if the repository already has equivalent canonical types.

The semantic model MUST be canonical enough that downstream consumers do not need to re-interpret source syntax.

---

56. IR Integration

Portability MUST NOT become a competing universal IR.

Instead:

portability semantics
        ↓
semantic annotations / requirements / contracts
        ↓
canonical domain representation

For classical computation:

portability
    ↓
classical semantic model
    ↓
classical IR

For quantum computation:

portability
    ↓
quantum semantic model
    ↓
quantum::ir

For HDL/hardware:

portability
    ↓
hardware semantic model
    ↓
HDL/hardware IR

Resource and portability information MUST remain available to downstream compilation without becoming a second semantic program representation.

---

57. Compiler Integration

The compiler MUST process portability information before committing to target-specific realization.

The compiler SHOULD perform:

1. normalization;
2. requirement collection;
3. capability matching;
4. constraint validation;
5. portability classification;
6. target-independent optimization;
7. specialization;
8. target-specific lowering;
9. realization validation.

A compiler MUST NOT silently convert:

requirement

into:

preference

or:

hint

because a target cannot satisfy the requirement.

---

58. Runtime Integration

The runtime MAY consume dynamic portability conditions.

It MAY:

- discover resources;
- inspect capabilities;
- negotiate resources;
- monitor availability;
- adapt placement;
- adapt concurrency;
- recover from failures;
- select declared fallbacks.

It MUST preserve semantic requirements.

A runtime MUST NOT silently alter:

- required outputs;
- required effects;
- required correctness;
- required resource guarantees;
- declared security requirements;
- declared deterministic behavior.

---

59. HAL Integration

HAL is responsible for exposing target capabilities and state.

Portability consumes HAL information indirectly.

The portability layer MUST NOT:

- implement HAL;
- enumerate devices itself;
- select physical devices;
- maintain calibration;
- perform hardware control.

HAL answers:

what does this target provide?

Portability answers:

what does this program require or permit?

The compiler/runtime resolves:

can this program be realized here?

---

60. Resource Manager Integration

The resource manager owns actual resource lifecycle.

Portability expresses:

what resource properties are semantically required.

The resource manager determines:

what resources can actually be supplied.

The two systems MUST NOT be conflated.

---

61. Scaling Contract

A conforming implementation MUST be able to distinguish:

semantic resource requirement

from:

implementation capacity.

For example:

requires qubits >= n

may fail on a target with too few physical/logical resources.

That does not make the source program invalid.

The implementation SHOULD report:

target/resource unsatisfied

rather than:

language resource limit exceeded

unless the program violates an actual language semantic rule.

---

62. Hard-Coding Audit

Portability implementation MUST be audited for:

Forbidden universal assumptions

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
MAX_DEVICES
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_TIMELINES
MAX_PROCESSES

Forbidden implicit physical identities

q0 → physical 0
q1 → physical 1
gpu → device 0
worker → node 0
memory → address 0x...

Forbidden parser-level implementation assumptions

if target_count > constant reject
if qubits > constant reject
if nodes > constant reject
if tensor_rank > constant reject

unless the condition is genuinely part of the language semantics.

---

63. Security and Portability

Portability MUST NOT weaken security semantics.

A target adaptation MUST preserve:

- authentication requirements;
- authorization requirements;
- isolation requirements;
- secret-handling rules;
- trusted-execution requirements;
- cryptographic requirements;
- provenance requirements.

A target lacking a mandatory security capability MUST fail the relevant requirement.

It MUST NOT silently downgrade security.

---

64. Reliability and Resilience

A portability contract MAY require:

- minimum reliability;
- fault tolerance;
- recovery;
- redundancy;
- checkpointing;
- availability.

These are semantic requirements when explicitly declared.

The implementation MAY choose the mechanism.

For example:

requires reliability >= requirement

does not prescribe:

- replication;
- QEC;
- checkpointing;
- redundant hardware;
- failover topology.

Those remain implementation decisions.

---

65. Energy and Thermal Portability

Energy and thermal constraints MAY be semantic.

Examples include:

energy <= budget
power <= limit
thermal <= constraint

A compiler MAY choose an implementation satisfying them.

It MUST NOT assume that the same physical power characteristics exist on every target.

---

66. Performance Portability

Performance requirements MUST distinguish:

- hard performance requirements;
- preferences;
- hints.

For example:

latency <= budget

is different from:

prefer low latency

and different again from:

hint locality

The compiler MUST preserve that distinction.

---

67. Performance Is Not Automatically Semantics

A target may execute the same semantic program faster or slower.

Performance differences alone do not invalidate portability unless the program explicitly declares a performance requirement or the performance difference causes another semantic contract to be violated.

---

68. Portability Across Compilation Strategies

A portable program MAY be compiled through different strategies:

AOT
JIT
static specialization
dynamic specialization
interpretation
simulation
hardware synthesis
quantum compilation
distributed compilation

The strategy MAY vary.

The source semantics MUST remain stable.

---

69. Portability Across Execution Models

Zamani MAY target:

- synchronous execution;
- asynchronous execution;
- task execution;
- actor execution;
- dataflow;
- streaming;
- distributed execution;
- hardware pipelines;
- quantum execution;
- hybrid execution.

A source construct MUST only assume the execution guarantees explicitly required by its semantics.

---

70. Portability Across Time

A portable program SHOULD remain semantically understandable across future compiler versions.

This requires:

- language versioning;
- feature versioning;
- compatibility metadata;
- deprecation policy;
- migration rules;
- stable semantic identifiers.

Portability therefore includes temporal portability, not merely hardware portability.

---

71. Version Compatibility

Portability metadata MUST integrate with:

- "grammar/spec/compatibility.md";
- "grammar/compatibility/versions.md";
- "grammar/compatibility/migrations.md";
- "grammar/compatibility/deprecated.md";
- feature compatibility manifests.

A future implementation MUST be able to determine:

language version
feature version
dialect version
semantic contract version

before interpreting portability metadata.

---

72. Dialect Compatibility

A portability contract involving a dialect MUST identify the dialect.

A dialect MUST NOT silently claim universal portability.

A dialect SHOULD declare:

portable across:
target classes
capability classes
version ranges
execution models

where applicable.

---

73. Interoperability Portability

Foreign representations MAY have stronger target assumptions than Zamani.

When importing or exporting them, the implementation MUST preserve the distinction between:

Zamani semantic portability

and:

foreign representation limitations.

For example, a foreign format that requires a fixed topology does not automatically make Zamani's semantic model topology-dependent.

---

74. Explicit Non-Portability

Zamani MAY intentionally support non-portable operations.

Examples:

- direct physical device binding;
- device-specific control;
- physical address access;
- vendor-specific extensions;
- exact hardware timing;
- exact physical qubit binding.

Such constructs MUST be explicitly marked or semantically classified as target-dependent.

They MUST NOT silently masquerade as portable constructs.

---

75. Portability of Physical Bindings

A physical binding SHOULD be represented as a downstream relationship:

logical entity
    ↓
target-specific binding
    ↓
physical entity

not:

source logical entity == physical entity

The source-to-physical mapping SHOULD preserve provenance.

---

76. Portability and Deployment

Deployment information belongs in deployment/target layers.

Portable source MAY specify:

- deployment constraints;
- availability requirements;
- geographic policy;
- security policy;
- resource requirements;
- environmental requirements.

It SHOULD NOT require a physical deployment identity unless explicitly intended.

---

77. Portability and Cloud/Edge

A portable program MAY move between:

edge
cloud
cluster
workstation
embedded

provided its declared requirements are satisfied.

The compiler/runtime MAY alter:

- partitioning;
- replication;
- communication;
- placement;
- caching;
- acceleration.

The source program remains unchanged.

---

78. Portability and Embedded Computing

The same semantic language model MUST work for small embedded environments.

A target with fewer resources may legitimately reject a program requiring more resources.

That is not a grammar scalability failure.

The grammar MUST remain capable of representing the program.

Therefore:

language acceptance

and:

target feasibility

MUST remain separate.

---

79. Portability and Very Large Systems

Likewise, a large machine MAY execute a program with a larger realization.

The compiler/runtime MAY scale:

- workers;
- memory;
- storage;
- communication;
- accelerator use;
- parallelism;
- distribution.

The source language MUST NOT require a separate "large machine" dialect merely because scale increases.

---

80. Portability and Resource Negotiation

When resource quantities are not known statically, the implementation MAY negotiate them.

Conceptually:

program
  ↓
minimum requirement
  ↓
acceptable range
  ↓
available resources
  ↓
negotiated realization

Negotiation MUST respect:

- hard requirements;
- hard constraints;
- security;
- correctness;
- effects;
- determinism;
- explicit fallback rules.

Preferences MAY guide negotiation.

Hints MAY guide optimization.

Neither may override a mandatory requirement.

---

81. Portability and Failure

Failure MUST be explicit.

If no valid realization exists:

NO_VALID_REALIZATION

or an equivalent canonical diagnostic SHOULD be produced.

The implementation MUST NOT:

- silently lower correctness;
- silently drop operations;
- silently change resource requirements;
- silently substitute a weaker capability;
- silently change physical semantics.

---

82. Portability and Fallbacks

A fallback MUST be explicitly declared.

For example:

required capability
    ↓
fallback capability

The fallback MUST have a known semantic relationship to the original operation.

A backend MUST NOT invent a fallback simply because it has another implementation available.

---

83. Portability and Approximation

Approximation is a semantic change unless explicitly permitted.

A compiler MUST NOT replace:

exact computation

with:

approximate computation

merely because a target lacks resources.

Approximation MUST be represented explicitly through the language's semantic model.

---

84. Portability and Numerical Precision

Precision reduction is likewise a semantic transformation.

A compiler MAY lower precision only when:

- the type permits it;
- the program declares it;
- the semantics guarantee acceptable behavior;
- or an explicit approximation contract permits it.

The compiler MUST NOT silently reduce precision merely because a target lacks high-precision support.

---

85. Portability and Memory Pressure

Memory pressure MAY cause:

- tiling;
- streaming;
- spilling;
- recomputation;
- distribution;
- compression;

when such transformations preserve semantics and are permitted.

Memory pressure MUST NOT silently cause:

- data loss;
- precision loss;
- ordering violations;
- lifetime violations;
- ownership violations.

---

86. Portability and Parallelism

Parallelism MAY scale dynamically.

The language MUST distinguish:

parallelism as semantic intent

from:

exact worker count.

If exact worker count is not semantically required, the implementation MAY choose any supported degree of parallelism.

---

87. Portability and Distributed Execution

Distributed realization MAY change:

- partitioning;
- placement;
- replication;
- communication routes;
- number of workers;
- fault domains.

The compiler/runtime MUST preserve distributed semantics such as:

- consistency;
- ordering;
- delivery;
- synchronization;
- transactional guarantees.

---

88. Portability and Hardware Generation

A future hardware generation SHOULD be able to consume existing semantic programs without requiring source rewrites when it provides compatible capabilities.

Hardware-generation-specific improvements belong downstream.

---

89. Portability and Future Computing Paradigms

The resource and portability model MUST remain extensible to future computational paradigms.

A new paradigm SHOULD be expressible through:

domain
capability
resource
constraint
semantic operation
IR mapping
target realization

without requiring the entire portability model to be redesigned.

This includes future:

- quantum architectures;
- photonic systems;
- neuromorphic systems;
- molecular/nano systems;
- optical accelerators;
- biological computing;
- novel memory systems;
- future heterogeneous architectures.

The grammar MUST NOT claim that today's list is exhaustive.

---

90. Cross-Domain Portability

A single program MAY combine:

classical
quantum
HDL
hardware
AI
data
distributed
networking
security

Portability MUST apply across the combined semantic program.

For example:

classical control
      ↓
quantum computation
      ↓
measurement
      ↓
classical analysis
      ↓
accelerator execution
      ↓
distributed aggregation

Each domain retains its own canonical semantics.

Portability provides the cross-domain contract.

---

91. Hybrid Quantum-Classical Portability

A hybrid program MUST NOT become non-portable merely because its quantum and classical components execute on different physical systems.

The compiler/runtime MAY select:

- local QPU;
- remote QPU;
- simulator;
- heterogeneous accelerator;
- future quantum architecture.

The hybrid semantic boundary MUST remain explicit.

---

92. Source Spans and Provenance

Every portability-related semantic object SHOULD retain source provenance.

At minimum:

source file
source span
feature identity
semantic category

SHOULD remain available.

Where target realization is involved, provenance SHOULD extend to:

source
→ AST
→ semantic model
→ IR
→ transformation
→ target realization

This is necessary for production diagnostics and reproducibility.

---

93. Deterministic Normalization

Portability normalization MUST be deterministic.

Equivalent source programs MUST normalize consistently.

Where ordering is semantically irrelevant, implementation ordering MUST NOT accidentally become semantic.

Maps, sets, capabilities, requirements, and constraints MUST be handled in a way that does not introduce nondeterministic compiler behavior.

---

94. Performance of Portability Analysis

Portability analysis MUST scale with program size and resource-expression complexity.

The implementation MUST NOT introduce fixed semantic limits solely for convenience.

If implementation limits are unavoidable for safety or operational reasons, they MUST be:

- implementation-level;
- documented;
- diagnosable;
- separate from language semantics;
- configurable where appropriate;
- incapable of changing the language's theoretical resource model.

---

95. Error Handling

Portability analysis MUST use explicit error propagation.

Production Rust implementation MUST NOT use "unsafe" to bypass:

- resource validation;
- portability validation;
- ownership;
- bounds checking;
- error handling.

A portability failure MUST be represented through the repository's canonical diagnostic/error infrastructure.

---

96. Testing Contract

A production implementation is incomplete until portability is tested at all relevant boundaries.

96.1 Positive tests

Must include:

- portable classical computation;
- portable quantum computation;
- portable hybrid computation;
- portable HDL intent;
- portable distributed computation;
- portable AI/data workloads;
- resource requirements;
- capability requirements;
- portability guarantees;
- preferences;
- hints;
- adaptation;
- dynamic resource requirements;
- future/unknown capability names where syntax permits them.

96.2 Negative tests

Must include:

- invalid portability syntax;
- unsatisfied mandatory requirements;
- contradictory constraints;
- illegal physical bindings;
- invalid fallback;
- invalid portability guarantee;
- incompatible dialect;
- incompatible language version;
- unsupported target dependency;
- semantic preservation failure.

96.3 Boundary tests

Must include:

- zero/empty where semantically legal;
- minimum valid quantity;
- very large quantities;
- symbolic quantities;
- nested portability contracts;
- many constraints;
- many capabilities;
- deeply composed resource expressions;
- large domain lists;
- large portability contracts.

No test may establish an artificial universal maximum.

96.4 Scalability tests

Must verify that the system can represent progressively larger:

- resource quantities;
- resource sets;
- requirements;
- capabilities;
- domains;
- dimensions;
- workloads;
- worker counts;
- node counts;
- qubit counts;
- tensor dimensions;

subject only to actual implementation/resource capacity.

96.5 Determinism tests

Repeated analysis of identical input MUST produce deterministic semantic output.

96.6 Cross-domain tests

Must cover:

classical + resources
quantum + resources
quantum + classical + resources
HDL + hardware + resources
AI + accelerator + resources
distributed + networking + resources
security + deployment + resources

---

97. Compatibility Testing

Portability features MUST be tested against:

grammar/Zamani.g4
lexer
parser
AST
semantic analysis
resource analysis
compiler
IR
runtime

A feature is not complete merely because its ".g4" file parses independently.

---

98. Feature Manifest Integration

Portability features SHOULD have corresponding machine-readable feature contracts under the repository's feature-manifest mechanism.

A portability feature manifest SHOULD identify:

id
name
status
version
syntax
grammar
lexer_tokens
ast_nodes
semantic_rules
ir_mapping
compiler_consumers
runtime_consumers
domain
capabilities
resource_requirements
negative_tests
boundary_tests
scalability_tests
compatibility
hard_coding_policy

The manifest MUST identify the portability classification of the feature.

---

99. Integration Matrix

Repository area| Portability responsibility
"grammar/DESIGN.md"| Overall architectural authority
"grammar/Zamani.g4"| Canonical grammar composition
"grammar/specification/portability.md"| Human-readable normative portability semantics
"grammar/spec/portability.md"| Formal portability semantic contract
"grammar/spec/resources.md"| General resource semantics
"grammar/spec/semantics.md"| General language semantics
"grammar/spec/type-system.md"| Type portability
"grammar/spec/effects.md"| Effect portability
"grammar/spec/compatibility.md"| Version/compatibility rules
"grammar/resources/portability.g4"| Portability syntax
"grammar/resources/requirements.g4"| Resource requirements
"grammar/resources/constraints.g4"| Resource constraints
"grammar/resources/capabilities.g4"| Capability syntax
"grammar/resources/preferences.g4"| Preferences
"grammar/resources/scalability.g4"| Scaling intent
"grammar/hardware/"| Hardware capability/realization contracts
"grammar/compile/"| Compilation and target-selection intent
"grammar/execution/"| Runtime/deployment intent
"grammar/quantum/"| Quantum semantic syntax
"grammar/classical/"| Classical semantics
"grammar/hdl/"| HDL/co-design semantics
"grammar/hybrid/"| Cross-domain semantics
"grammar/distributed/"| Distributed semantics
"grammar/ai/"| AI/ML semantics
"grammar/data/"| Data semantics
"grammar/networking/"| Communication semantics
"grammar/security/"| Security portability
"grammar/dialects/"| Explicit extensions
"grammar/compatibility/"| Version/migration compatibility
"grammar/validation/"| Static conformance and hard-coding audits
"grammar/tests/"| Conformance evidence
"src/lexer.rs"| Lexical implementation
"src/parser.rs"| Parsing implementation
"src/frontend/ast/"| Domain-neutral structural representation
semantic layer| Portability/resource/capability analysis
"quantum::ir"| Canonical quantum semantic boundary
optimizer| Target-specific implementation improvement
routing| Physical realization
scheduling| Timing/order/resource scheduling
QEC| Error correction
ZQN| Fault/noise semantics
HAL| Device capability/state
runtime| Dynamic realization

---

100. Non-Circular Dependency Rule

The portability semantic model MUST remain upstream of implementation-specific realization.

Allowed:

portability
    ↓
resource semantics
    ↓
compiler
    ↓
hardware

Allowed:

portability
    ↓
quantum semantics
    ↓
quantum::ir
    ↓
routing

Forbidden:

portability
    ↓
physical device
    ↓
portability meaning

Also forbidden:

portability grammar
    ↓
runtime scheduler implementation

The semantic model MUST be stable without requiring a particular backend implementation.

---

101. What Counts as Portable

A construct is portable when its semantic meaning can remain valid across different compatible realizations without source rewriting.

Examples:

logical computation
logical qubit
abstract accelerator
resource requirement
capability requirement
symbolic worker count
parameterized tensor
abstract memory
logical communication channel
portable algorithm

---

102. What Is Not Automatically Portable

The following are target-dependent unless explicitly abstracted:

physical address
physical qubit ID
specific PCI device
specific GPU ID
specific CPU core
specific memory bank
specific network interface
vendor-specific instruction
exact hardware timing
device-specific calibration
physical topology
device-specific register

The language MAY support them through explicit interoperability/target-specific mechanisms.

---

103. Portability Guarantee

A portability guarantee means that the implementation promises to preserve specified semantics across a declared portability boundary.

A guarantee MUST be:

- semantically defined;
- analyzable;
- traceable;
- testable.

Parser acceptance MUST NOT be treated as proof of a guarantee.

For example:

portability guarantees semantic_equivalence;

requires semantic verification, not merely parsing.

---

104. Portability Contract Scope

Portability contracts MAY apply to:

- a declaration;
- expression;
- statement;
- function;
- module;
- component;
- domain;
- resource;
- complete program.

The semantic scope MUST be explicit.

Nested contracts MUST compose according to the general semantics rules.

---

105. Composition Rules

When multiple portability constraints apply:

program
+
module
+
function
+
domain
+
target

the effective contract is the composition of applicable constraints.

A weaker nested contract MUST NOT silently weaken a stronger enclosing requirement.

Conflicts MUST produce diagnostics when they cannot be resolved according to the language's normal constraint-composition rules.

---

106. Portability and Generic Programming

Generic programs SHOULD be able to express resource and capability parameters.

For example:

algorithm<T, ResourcePolicy>

may remain target-independent while allowing the compiler to specialize the realization.

Generic resource parameters MUST NOT require physical identities.

---

107. Portability and Metaprogramming

Compile-time metaprograms MUST NOT bypass portability analysis.

Generated code MUST undergo the same:

syntax
→ AST
→ semantic analysis
→ resource analysis
→ portability analysis
→ IR

pipeline applicable to ordinary source.

---

108. Portability and Macros

Macros MUST NOT be used to smuggle target-specific assumptions into otherwise portable syntax.

Expanded code MUST remain subject to:

- portability analysis;
- resource analysis;
- type analysis;
- effect analysis;
- security analysis;
- IR validation.

---

109. Portability and Diagnostics from Generated Code

When a portability failure originates in generated code, diagnostics SHOULD retain:

- generated source span;
- macro/metaprogram provenance;
- original invocation span;
- portability construct responsible.

---

110. Portability and Source Compatibility

Existing portable programs SHOULD remain portable across compatible language revisions.

A language revision MUST NOT introduce a new universal hardware limitation.

A new backend MAY introduce new capabilities without changing the source semantics.

---

111. Portability and Deprecation

Deprecated portability constructs MUST remain identifiable.

A deprecated construct MUST NOT silently change from:

portable

to:

target-specific

without an explicit compatibility decision.

---

112. Portability and Security of Target Selection

Target selection MUST NOT allow untrusted source-level portability metadata to bypass security policy.

For example:

prefer secure_device

cannot override:

security policy prohibits device

Security policy remains authoritative where the program is executed in a controlled environment.

---

113. Portability and Resource Exhaustion

A compiler/runtime MAY protect itself against resource exhaustion.

Such protections are implementation safeguards.

They MUST NOT become universal language semantics.

For example:

implementation refuses a compilation requiring excessive host memory

does not imply:

Zamani programs may never describe larger computations.

The diagnostic SHOULD identify implementation resource exhaustion rather than falsely reporting a language semantic violation.

---

114. Portability and Host Resource Limits

Host limitations such as:

- address space;
- process limits;
- filesystem limits;
- compiler memory;
- runtime quotas;

MUST remain implementation/deployment constraints.

They MUST NOT be encoded as grammar-level universal limits.

---

115. Portability and Simulation

Simulation is a realization strategy.

A quantum program MAY be simulated on a classical target when the simulator can satisfy the program's declared semantics and resource requirements.

Simulation MUST NOT be silently treated as equivalent to physical quantum execution when the program explicitly requires physical quantum capabilities.

---

116. Portability and Verification

A portability transformation SHOULD be verifiable.

Verification MAY include:

- semantic equivalence;
- type preservation;
- effect preservation;
- resource-contract preservation;
- capability satisfaction;
- correctness proofs;
- deterministic replay;
- provenance.

The verification mechanism is downstream of the portability contract.

---

117. Portability and Resilience

A program MAY declare resilience requirements.

The implementation MAY satisfy them through:

- replication;
- recovery;
- checkpointing;
- QEC;
- redundancy;
- failover;
- retry;
- migration.

The selected mechanism MUST preserve the declared semantics.

Portability itself does not implement resilience.

---

118. Portability and Calibration

Calibration information belongs to hardware/HAL/backend layers.

Portable source MAY require a capability or quality property.

It SHOULD NOT directly depend on a particular calibration record unless explicitly target-specific.

---

119. Portability and Topology

Topology MAY be a semantic constraint when the algorithm genuinely requires a topology property.

The program SHOULD express an abstract property such as:

requires connectivity(...)

rather than hard-coding a physical graph unless physical topology is genuinely part of the program's semantics.

Routing determines physical realization.

---

120. Portability and Placement

Placement is normally an implementation decision.

The program MAY express:

- locality requirements;
- affinity;
- anti-affinity;
- co-location;
- separation;
- memory locality;
- communication constraints.

The actual placement remains downstream.

---

121. Portability and Scheduling

The program MAY express:

- deadlines;
- latency;
- ordering;
- synchronization;
- throughput;
- resource budgets.

Exact scheduling remains downstream unless exact scheduling is explicitly part of the semantic contract.

---

122. Portability and Physical Timing

Exact physical timing is target-dependent unless explicitly declared.

A portable program SHOULD prefer semantic timing constraints:

latency <= bound

rather than target-specific:

execute at physical cycle 37

unless the latter is intentionally a hardware-design contract.

---

123. Portability and HDL Timing

HDL may legitimately have exact timing semantics.

Therefore HDL portability MUST distinguish:

language-level timing semantics

from:

target-specific clock implementation.

A timing contract MAY be portable even when its physical implementation differs.

---

124. Portability and Hardware Co-Design

A co-designed Zamani program MAY contain:

software intent
+
hardware intent
+
resource intent
+
communication intent
+
timing constraints

The compiler MAY derive:

CPU code
GPU code
FPGA design
ASIC design
QPU control
distributed deployment

without requiring the algorithmic source to be rewritten for every realization.

---

125. Portability and ABI/FFI

FFI/ABI boundaries are inherently more target-sensitive.

An FFI declaration MUST clearly identify:

- external ABI;
- calling convention;
- representation requirements;
- portability classification.

A portable semantic wrapper MAY hide an ABI-specific implementation behind a stable Zamani interface.

---

126. Portability and Vendor Extensions

Vendor extensions MAY exist.

They MUST be:

- explicitly identified;
- versioned;
- capability-described;
- compatibility-described;
- classified for portability.

A vendor extension MUST NOT silently become part of the universal Zamani language.

---

127. Portability and Future Backends

A backend added after a program was written SHOULD be able to consume existing canonical semantic/IR representations.

A new backend MUST NOT require modifications to source syntax merely to express an already-supported semantic concept.

---

128. Production Conformance

A portability implementation is production-conformant only when all of the following hold:

- syntax is defined;
- lexer integration exists;
- parser integration exists;
- AST mapping exists;
- semantic mapping exists;
- resource integration exists;
- capability integration exists;
- IR integration exists;
- compiler integration exists;
- runtime integration exists where required;
- diagnostics exist;
- provenance exists;
- compatibility exists;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- determinism tests exist;
- cross-domain tests exist;
- hard-coding audit passes;
- Rust 1.97/1.97.1 compatibility passes;
- no "unsafe" implementation is used.

---

129. Definition of Done for This File

"grammar/spec/portability.md" is complete when:

Architecture

- [x] Portability is defined as semantic portability.
- [x] POCO-REAF is formally defined.
- [x] Target realization is separated from source semantics.
- [x] Logical and physical resources are separated.
- [x] Capability-based portability is defined.
- [x] Resource/constraint/preference/hint categories are separated.
- [x] Quantum portability is integrated.
- [x] Classical portability is integrated.
- [x] HDL/hardware portability is integrated.
- [x] Hybrid portability is integrated.
- [x] Distributed portability is integrated.
- [x] AI/data portability is integrated.
- [x] Networking/security portability is integrated.
- [x] Future-domain extensibility is defined.

Scalability

- [x] No artificial universal resource ceiling is allowed.
- [x] Tiny-to-large scaling is defined.
- [x] "Infinity" is defined as absence of language-imposed finite ceilings.
- [x] Host integer width is not a semantic limit.
- [x] Dynamic resource availability is supported.
- [x] Resource negotiation is supported.
- [x] Dynamic adaptation is supported.

Quantum

- [x] "quantum::ir" remains canonical.
- [x] Physical qubit IDs are not universal semantics.
- [x] Physical topology is downstream.
- [x] Routing is downstream.
- [x] Scheduling is downstream.
- [x] QEC remains downstream.
- [x] ZQN remains downstream.
- [x] HAL remains downstream.
- [x] Generic/future quantum operations are supported semantically.

Compiler

- [x] AST contract defined.
- [x] Semantic contract defined.
- [x] IR contract defined.
- [x] compiler integration defined.
- [x] runtime integration defined.
- [x] provenance defined.
- [x] deterministic normalization defined.

Safety

- [x] Rust 1.97/1.97.1 specified.
- [x] Rust 2021 specified.
- [x] "unsafe" prohibited.
- [x] overflow/truncation concerns addressed.
- [x] implementation limits separated from language limits.

Verification

- [x] positive testing defined.
- [x] negative testing defined.
- [x] boundary testing defined.
- [x] scalability testing defined.
- [x] determinism testing defined.
- [x] cross-domain testing defined.
- [x] compatibility testing defined.
- [x] hard-coding audit defined.

---

130. Required Repository Changes After This Contract

This file is intentionally designed so that it can be completed independently. The following files should consume its contract rather than redefine it.

"grammar/resources/portability.g4"

Must implement only the syntax corresponding to this semantic model.

It MUST NOT:

- introduce a second portability expression language;
- encode hardware limits;
- enumerate finite target types as the universal portability model;
- perform target selection.

"grammar/resources/scalability.g4"

Must implement scaling intent while delegating semantics to:

- this file;
- "grammar/spec/resources.md".

"grammar/resources/requirements.g4"

Must represent requirements without turning them into target bindings.

"grammar/resources/constraints.g4"

Must distinguish constraints from preferences and hints.

"grammar/resources/capabilities.g4"

Must permit extensible capability identifiers.

"grammar/compile/target.g4"

Must treat target selection as realization rather than redefining portability.

"grammar/execution/deployment.g4"

Must consume portability/resource intent without making deployment identity part of universal semantics.

"grammar/hardware/"

Must describe hardware capabilities and realization contracts rather than universal source-language limitations.

"grammar/quantum/"

Must preserve logical quantum semantics and integrate with "quantum::ir".

"grammar/validation/"

Must validate portability and detect hard-coded universal limits.

"grammar/compatibility/"

Must track portability behavior across language and feature versions.

"grammar/tests/"

Must provide the conformance matrix described above.

---

131. Final Architectural Rule

The entire portability model can be reduced to one rule:

«A Zamani program describes semantic intent and the conditions under which that intent may be realized. It does not, by default, prescribe the physical machine on which that intent must execute.»

Therefore:

PROGRAM
   ↓
SEMANTIC INTENT
   ↓
RESOURCE + CAPABILITY REQUIREMENTS
   ↓
PORTABILITY CONTRACT
   ↓
CANONICAL SEMANTIC MODEL
   ↓
CANONICAL IR
   ↓
TARGET-INDEPENDENT OPTIMIZATION
   ↓
TARGET REALIZATION
   ↓
CPU / GPU / FPGA / ASIC / QPU /
DISTRIBUTED / CLOUD / EDGE / FUTURE

The source program remains the stable semantic origin.

The machine is a realization.

The available resources determine scale.

The capabilities determine feasibility.

The compiler determines implementation.

The runtime determines execution.

The hardware determines physical realization.

The language must not confuse those layers.

That separation is the foundation required for Zamani to scale from atom to everywhere while preserving the intended Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF) model.