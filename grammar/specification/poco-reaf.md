Worked for 1m 13s

I inspected the repository's current grammar architecture and related quantum/compilation boundaries before defining this file. In particular, the existing design establishes Rust 1.97/1.97.1, Safe Rust only, POCO-REAF, the frontend → semantic analysis → canonical IR pipeline, and quantum::ir as the canonical quantum semantic boundary.  The repository also already contains substantial quantum::ir structure and explicitly separates it from scheduling, ZQN, and resilience.

I also found that POCO-REAF is already referenced across the grammar specifications, including grammar/specification/README.md, lexical, syntax, semantic, type-system, language-scope, and compatibility documents.

The GitHub write operation itself was unavailable from this session, so below is the complete contents for grammar/specification/poco-reaf.md ready to place at that exact path.

Zamani POCO-REAF Specification

Path: "grammar/specification/poco-reaf.md"
Status: Normative production specification
Language: Zamani
Minimum implementation Rust: Rust 1.97 / 1.97.1
Safety: Safe Rust only; Rust "unsafe" is forbidden
Primary objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the normative meaning of Zamani's Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF) model and the contracts required from the grammar, frontend, semantic layers, IR, compiler, target layers, and runtime.

POCO-REAF is a portability and semantic-stability property.

It is not a promise that every program can execute on every machine.

A target may lack required:

- capabilities;
- resources;
- memory;
- processing capacity;
- quantum resources;
- timing guarantees;
- precision;
- connectivity;
- supported execution modes;
- security capabilities;
- runtime services.

Such failure MUST be explicit, diagnosable, and MUST NOT silently change the program's meaning.

The central rule is:

«Zamani source describes computation and portable intent. Target-specific systems determine how that computation is realized under the capabilities, constraints, and resources actually available.»

The language MUST therefore scale from the smallest supported computation to arbitrarily large computations subject only to representational, implementation, declared-resource, and execution-resource limits.

No artificial machine-size ceiling may be introduced into the source grammar.

---

2. Normative Scope

This specification applies to:

- "grammar/Zamani.g4"
- "grammar/specification/*"
- "grammar/spec/*"
- "grammar/grammar.md"
- "grammar/Zamani-Grammar.md"
- lexer implementation
- parser implementation
- AST construction
- semantic analysis
- type analysis
- effect analysis
- capability analysis
- resource analysis
- canonical semantic IR generation
- "quantum::ir"
- classical/control/data IR
- optimization
- routing
- scheduling
- ZQN
- QEC integration
- resilience
- hardware abstraction
- target lowering
- runtime
- interoperability
- grammar tests
- compiler tests
- compatibility tests

This document does not own:

- target-specific lowering algorithms;
- hardware discovery;
- calibration;
- QEC algorithms;
- scheduling algorithms;
- optimization algorithms;
- routing algorithms;
- runtime implementation;
- physical device management;
- backend-specific machine code.

It defines the portability contract those systems MUST satisfy.

---

3. Normative Language

The terms MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, and MAY are normative.

A conforming implementation:

1. MUST preserve source semantics across supported targets.
2. MUST separate source semantics from target realization.
3. MUST represent target requirements explicitly.
4. MUST NOT introduce fixed machine-size limits into the language grammar.
5. MUST NOT silently substitute a semantically different operation because a target lacks a capability.
6. MUST use safe Rust only.
7. MUST provide structured diagnostics for unsupported, impossible, or resource-insufficient execution.
8. MUST preserve sufficient provenance to explain target-specific lowering decisions.
9. MUST distinguish guaranteed semantics from performance preferences and optimization hints.
10. MUST preserve language-version compatibility according to the compatibility policy.

---

4. Definition of POCO-REAF

4.1 Program Once

A Zamani program is written against the language's semantic model rather than against a particular machine.

Source MAY express:

- computational intent;
- types;
- effects;
- concurrency semantics;
- quantum semantics;
- hardware semantics;
- HDL semantics;
- resource requirements;
- capability requirements;
- correctness constraints;
- timing requirements;
- precision requirements;
- security requirements;
- placement constraints;
- portability constraints;
- performance preferences.

Source MUST NOT need to be rewritten merely because the target has a different:

- number of cores;
- number of qubits;
- number of devices;
- memory capacity;
- topology;
- instruction set;
- native gate set;
- accelerator layout;
- communication fabric;
- timing grid;
- calibration state;
- execution provider.

---

4.2 Compile Once

Compilation produces a canonical, versioned semantic representation and, where supported, a reusable target-independent compiled artifact.

"Compile once" means that the semantic compilation result can be reused across compatible target realizations without requiring the source program to be rewritten for every target.

It does not mean that one immutable machine-code binary must execute on every ISA.

Target-specific realization MAY occur through:

- specialization;
- capability negotiation;
- routing;
- scheduling;
- optimization;
- decomposition;
- resource allocation;
- target lowering;
- code generation;
- runtime dispatch.

Those transformations MUST preserve canonical semantics or explicitly report that the selected target cannot satisfy them.

---

4.3 Run Everywhere

A program can run on every target for which:

1. the target satisfies the program's semantic requirements;
2. the target satisfies required capabilities;
3. sufficient resources are available;
4. a conforming lowering exists;
5. execution is permitted by security and policy constraints.

"Everywhere" means across supported execution classes.

It does not mean that an incapable machine must somehow execute an impossible computation.

---

4.4 Run Anywhere

The same semantic program MAY execute:

- locally;
- remotely;
- embedded;
- distributed;
- simulated;
- emulated;
- accelerated;
- classically;
- quantumly;
- on heterogeneous hardware;
- on future target classes.

Target identity MUST NOT become part of program meaning unless explicitly declared as such.

---

4.5 Run Forever

The semantic contract MUST be versioned and evolvable so that new targets can interpret existing valid programs without requiring historical source to encode future hardware assumptions.

"Forever" is therefore a language-evolution objective.

It does not claim that every historical implementation or physical device will remain available forever.

Backward compatibility is governed by:

- "grammar/specification/language-version.md";
- "grammar/specification/compatibility.md";
- "grammar/compatibility/*".

---

5. Fundamental Separation: Meaning vs Realization

Zamani MUST distinguish:

Semantic requirement

What must be true for the program to be correct.

Capability requirement

What a target must be able to provide.

Constraint

What realizations are forbidden.

Preference

What realization is preferred but not required.

Hint

Information supplied to guide optimization without changing semantics.

Budget

An explicitly selected limit on resource consumption.

Target fact

What the selected execution environment actually provides.

Target facts MUST NOT automatically become source semantics.

For example:

requires quantum

does not mean:

use device X
use topology Y
use exactly N physical qubits
use native gate set Z

Likewise:

requires memory >= M

expresses a resource requirement.

It does not establish a universal machine memory size.

---

6. Scaling Model

6.1 General Rule

Every scalable quantity MUST be represented:

- symbolically;
- parametrically;
- dynamically;
- through an explicit program requirement;
- through a resource expression;
- through a runtime-discovered capability.

The following MUST NOT receive arbitrary language-level maxima:

- qubits;
- logical qubits;
- physical qubits;
- quantum registers;
- CPUs;
- cores;
- threads;
- GPUs;
- FPGAs;
- ASIC instances;
- devices;
- nodes;
- processes;
- tasks;
- channels;
- memories;
- tensor dimensions;
- tensor rank;
- vector lengths;
- matrix dimensions;
- network endpoints;
- distributed partitions;
- storage capacity;
- address spaces;
- program size;
- circuit depth;
- timeline count;
- hardware modules.

---

6.2 Legitimate Limits

A limit is legitimate only when it represents one of:

1. A fundamental language semantic restriction.
2. A documented representation constraint.
3. An explicitly selected resource budget.
4. A target capability.
5. A user-selected execution policy.
6. A security or safety policy.
7. A finite host-resource limitation.

A limit MUST NOT be disguised as a grammar constant.

---

6.3 No Artificial Ceiling

The language MUST NOT define universal semantics equivalent to:

MAX_QUBITS = 32
MAX_CORES = 64
MAX_DEVICES = 16
MAX_TENSOR_RANK = 8

Such constants MAY exist only as:

- test fixtures;
- implementation safeguards;
- explicit user-selected execution budgets;
- target-specific capability values.

They MUST NOT define the semantic capacity of Zamani.

---

7. Program Shape and Parametric Scaling

Programs SHOULD express scale through program semantics rather than machine identity.

Examples include:

for each element in data {
    ...
}

allocate n qubits

parallelize over available resources

require capability quantum

require memory >= required_memory

A compiler/runtime MAY instantiate those semantics at different scales.

A program whose algorithm explicitly requires 1024 qubits is portable to every target capable of satisfying that requirement.

It is not equivalent to a language that universally supports only 1024 qubits.

---

8. Source-to-Execution Contract

The canonical pipeline is:

Zamani Source
    │
    ▼
Source Identity / Source Map
    │
    ▼
Lexer
    │
    ▼
Parser
    │
    ▼
Frontend AST
    │
    ▼
Name / Module / Import Resolution
    │
    ▼
Type + Effect + Capability + Resource Analysis
    │
    ▼
Canonical Semantic Representation
    ├───────────────────────┬─────────────────────┐
    ▼                       ▼                     ▼
Classical IR           quantum::ir       Effect/resource/
                                          temporal metadata
    │                       │                     │
    └───────────────────────┴─────────────────────┘
                            │
                            ▼
                  Target-independent analysis
                            │
                            ▼
                 Optimization / Planning
                            │
                            ▼
             Resilience / Routing / Scheduling / ZQN
                            │
                            ▼
                      Target Lowering
                            │
                            ▼
          CPU / GPU / FPGA / ASIC / QPU /
          Simulator / Distributed / Future Target
                            │
                            ▼
                          Runtime

The grammar is authoritative for syntax.

The grammar MUST NOT bypass semantic analysis by directly encoding backend decisions.

---

9. Grammar Contract

9.1 Grammar Owns

The grammar owns:

- lexical structure;
- syntactic structure;
- valid source forms;
- precedence;
- associativity;
- syntactic declarations;
- syntactic resource expressions;
- syntactic capability expressions;
- syntactic quantum constructs;
- syntactic hardware constructs;
- syntactic HDL constructs;
- syntactic effects;
- syntactic annotations;
- source-level version markers.

---

9.2 Grammar Does Not Own

The grammar does not own:

- physical device discovery;
- calibration values;
- native hardware gate sets;
- routing algorithms;
- schedule construction;
- QEC algorithms;
- noise models;
- runtime retry policy;
- optimization algorithms;
- physical placement decisions;
- device identifiers as semantic identities;
- backend-specific machine code.

---

9.3 Open-Ended Operations

Domain operations that do not require distinct syntax SHOULD remain:

- identifiers;
- calls;
- expressions;
- intrinsics;
- registered operations;
- dialect operations;
- library functions.

The grammar MUST NOT become a closed enumeration of all possible:

- quantum gates;
- mathematical operations;
- accelerators;
- processors;
- GPUs;
- FPGA primitives;
- HDL primitives;
- devices;
- future hardware operations.

This is essential for:

- scalability;
- extensibility;
- future hardware;
- dialect support;
- POCO-REAF.

---

10. Quantum POCO-REAF

Quantum computing is a first-class Zamani domain.

Quantum source MUST remain hardware-independent unless hardware specificity is explicitly part of the program's declared intent.

---

10.1 Canonical Quantum Boundary

"quantum::ir" is the canonical quantum semantic boundary.

The grammar/frontend MUST NOT create a competing permanent quantum IR.

The frontend MAY construct temporary AST nodes for:

- quantum operation expressions;
- qubit references;
- quantum register declarations;
- measurement;
- reset;
- controlled operations;
- parameter expressions;
- observables;
- quantum regions.

Those nodes MUST lower into the canonical quantum semantic representation.

---

10.2 Qubit Identity

The grammar MUST NOT define competing semantic versions of:

- "QubitId";
- "PhysicalQubitId";
- logical qubit identity;
- physical qubit identity.

Canonical quantum identity belongs to the quantum IR/hardware boundary.

A source-level qubit reference expresses program semantics.

It is not automatically a physical device address.

---

10.3 No Fixed Qubit Count

The grammar MUST NOT restrict programs to a fixed number of qubits.

Forbidden as language semantics:

MAX_QUBITS = 32
MAX_QUBITS = 64
MAX_QUBITS = 1024

A program-specific register size is valid when it is part of the algorithm.

That size is program semantics.

It is not a universal compiler limit.

---

10.4 Quantum Operation Lowering

The required model is:

Zamani Quantum Intent
        ↓
Quantum AST
        ↓
Semantic Validation
        ↓
quantum::ir
        ↓
Optimization
        ↓
Routing / Mapping
        ↓
Scheduling
        ↓
ZQN / Calibration-Aware Analysis
        ↓
Target Lowering
        ↓
Native Execution

A backend MUST NOT silently replace an operation with a semantically different operation because its native gate set differs.

If exact realization is impossible, compilation MUST produce a structured diagnostic.

If approximation is permitted by language semantics, that approximation MUST be explicit and its correctness/error contract MUST be available to verification.

---

10.5 Quantum Control

Where implemented, the semantic model MUST be capable of representing:

- single-target operations;
- multi-target operations;
- controls;
- negative controls;
- parameterized operations;
- inverses/adjoints;
- measurement;
- mid-circuit measurement;
- reset;
- classical feed-forward;
- dynamic control flow;
- barriers/fences;
- logical-to-physical mapping;
- logical resources;
- error-correction metadata;
- noise-aware execution;
- pulse-level lowering.

These MUST be represented semantically rather than through an arbitrary closed gate list.

---

10.6 QEC Boundary

The grammar MAY express:

- QEC intent;
- logical resource requirements;
- error-correction requirements;
- fault-tolerance constraints.

The grammar MUST NOT implement QEC algorithms.

QEC owns:

- detection;
- decoding;
- correction;
- code-specific algorithms;
- syndrome processing;
- logical error handling.

---

10.7 ZQN Boundary

ZQN owns quantum:

- noise;
- faults;
- fault classification;
- fault location;
- correlated faults;
- leakage;
- loss;
- erasure;
- calibration-related fault information.

POCO-REAF integration is:

Program Semantics
       ↓
Requirements / Correctness Constraints
       ↓
quantum::ir
       ↓
ZQN Fault / Noise Information
       ↓
Planning / Adaptation

The grammar MUST NOT duplicate ZQN noise models.

---

10.8 Scheduling Boundary

Scheduling owns:

- operation ordering;
- dependency scheduling;
- resource conflicts;
- timing;
- alignment;
- delays;
- execution slots;
- scheduling policies.

The grammar MAY express semantic timing requirements.

The grammar MUST NOT hard-code:

- a device timing grid;
- gate durations;
- channel counts;
- pulse widths;
- backend timing tables.

Those belong to target information and scheduling.

---

10.9 Resilience Boundary

Resilience decides when and how to adapt execution in response to:

- faults;
- degradation;
- resource changes;
- backend failure;
- execution failure;
- changing target conditions.

The grammar MUST NOT own:

- retry algorithms;
- rollback algorithms;
- remapping;
- rerouting;
- rescheduling;
- backend switching;
- mitigation algorithms.

Where source-level resilience policy is required, grammar expresses policy intent.

The resilience subsystem implements the decision mechanism.

---

11. Classical POCO-REAF

Classical computation MUST remain fully expressive without establishing a fixed hardware scale.

The language MAY express:

- scalar computation;
- structured data;
- arrays;
- vectors;
- matrices;
- tensors;
- functions;
- generics;
- concurrency;
- parallelism;
- distributed computation;
- accelerator intent;
- numerical computation;
- symbolic computation.

A classical construct MUST NOT imply a particular:

- CPU;
- ISA;
- register count;
- cache hierarchy;
- SIMD width;
- core count;
- memory capacity.

---

12. HDL and Hardware POCO-REAF

HDL syntax represents hardware semantics such as:

- modules;
- ports;
- signals;
- registers;
- clocks;
- timing requirements;
- combinational behavior;
- sequential behavior;
- state machines;
- memories;
- pipelines;
- interfaces;
- parameters.

A hardware parameter that is part of the design is semantic.

A physical implementation detail is not automatically semantic.

For example, a parametrized data width is valid when the width is part of the hardware design.

A grammar rule that only permits 32-bit hardware is not.

Hardware source SHOULD be capable of lowering to different implementations when target toolchains support those realizations.

---

13. Hybrid Computing

Zamani MUST support classical, quantum, hardware, accelerator, and distributed semantics in one program.

Cross-domain boundaries MUST be explicit.

Examples include:

classical → quantum
quantum → classical measurement
classical → accelerator
HDL → software control
quantum → hardware execution
classical → distributed service

Conversions MUST be:

- type checked;
- effect checked;
- capability checked;
- resource checked where applicable.

A hybrid program MUST preserve semantic meaning when resources change, subject to its declared requirements.

---

14. Resource and Capability Model

POCO-REAF depends on separating what a program needs from what a machine happens to have.

The language/resource model MUST distinguish:

- requirement;
- capability;
- constraint;
- preference;
- hint;
- budget;
- target fact.

Examples of semantic intent include:

requires capability quantum;
requires capability fpga;
requires memory >= M;
requires qubits >= Q;
requires precision >= P;
prefer latency <= L;
prefer energy <= E;

The exact surface syntax is governed by the canonical grammar and resource specifications.

Semantic analysis determines the meaning.

The runtime reports actual target facts.

---

15. Target Selection

Target selection MUST be external to source semantics unless the developer explicitly declares a target requirement as part of the program's meaning.

Target selection MAY consider:

- declared capabilities;
- resource availability;
- correctness requirements;
- security policy;
- performance preferences;
- energy preferences;
- reliability requirements;
- placement constraints;
- interoperability requirements.

Target selection MUST NOT reinterpret a preference as a requirement.

For example:

prefer GPU

MUST NOT mean:

program is invalid on CPU

unless the program explicitly requires a GPU capability.

---

16. Compilation Reuse

A POCO-REAF implementation SHOULD preserve reusable artifacts at multiple levels:

1. Parsed source representation.
2. Typed/validated AST or semantic representation.
3. Canonical semantic IR.
4. Target-independent optimized IR.
5. Target-specialized representation.
6. Target-specific executable representation.

Each artifact MUST record its relevant assumptions.

An artifact depending on target facts MUST NOT be represented as universally target-independent.

---

17. Determinism

For identical:

- source;
- language version;
- source inputs;
- compilation configuration;
- dependency versions;
- deterministic target-independent policy;

parsing and semantic interpretation MUST be deterministic.

Target selection MAY differ when available targets differ.

The selected target MUST still satisfy the same source requirements.

Nondeterministic optimization or scheduling decisions MUST NOT alter semantic correctness.

---

18. Provenance

Target-dependent transformations SHOULD retain provenance sufficient to determine:

- which source construct caused an operation;
- which requirement caused resource allocation;
- which capability caused lowering;
- which optimization transformed representation;
- which routing decision changed placement;
- which scheduling decision changed timing;
- which ZQN information influenced planning;
- which resilience decision altered execution.

Provenance MUST NOT require:

- secrets;
- credentials;
- private keys;
- backend authentication data.

---

19. Failure Semantics

POCO-REAF MUST NOT permit silent failure.

The implementation MUST distinguish:

- syntax failure;
- semantic failure;
- type failure;
- capability failure;
- resource insufficiency;
- target incompatibility;
- unsupported lowering;
- runtime failure;
- transient execution failure;
- permanent execution failure;
- verification failure.

A target that cannot satisfy a requirement MUST produce a structured diagnostic.

The implementation MUST NOT silently:

- reduce qubit count;
- drop operations;
- change precision;
- change arithmetic semantics;
- remove synchronization;
- replace quantum computation;
- ignore hardware requirements;
- weaken security constraints.

If approximation is supported, approximation MUST be explicitly represented by the language/compilation contract.

---

20. Resource Exhaustion

Resource exhaustion is not automatically a grammar error.

The implementation MUST distinguish:

program is invalid

from:

target cannot currently satisfy valid program

Examples include:

- valid large quantum computation on a target with insufficient qubits;
- valid tensor computation on insufficient memory;
- valid distributed computation when the required node count is unavailable;
- valid FPGA design when the selected FPGA lacks resources.

The diagnostic SHOULD identify:

- unsatisfied requirement;
- available capability;
- relevant target;
- resource deficit;

where disclosure is safe.

---

21. Runtime Adaptation

Runtime MAY select among semantically equivalent realizations using current target facts such as:

- available CPU cores;
- GPU availability;
- QPU capacity;
- queue state;
- calibration state;
- network topology;
- energy budget;
- memory availability.

Runtime adaptation MUST NOT change program semantics.

Adaptation MUST respect:

- declared constraints;
- correctness requirements;
- security requirements;
- resource policies.

---

22. Optimization Boundary

Optimization MAY change implementation while preserving semantics.

Optimization owns transformations such as:

- algebraic simplification;
- operation cancellation;
- gate synthesis;
- common-subexpression elimination;
- target-independent optimization;
- target-aware optimization after target information is introduced.

The grammar MUST NOT encode optimizer implementation details as language semantics.

Optimization preferences are hints unless explicitly declared as requirements.

---

23. Routing Boundary

Routing maps logical computation to physical resources.

Quantum routing MAY include:

- logical-to-physical qubit mapping;
- connectivity realization;
- movement/swap insertion.

Distributed routing MAY include:

- node placement;
- communication mapping.

Accelerator routing MAY include:

- kernel placement;
- resource assignment.

Routing MUST consume canonical semantic representations and target capabilities.

Source MUST NOT encode target topology unless topology is explicitly part of the intended design.

---

24. Scheduling Boundary

Scheduling determines when executable operations occur subject to:

- dependencies;
- resources;
- timing constraints;
- synchronization;
- target capabilities;
- execution policy.

Source-level timing semantics MUST be distinguished from target scheduling decisions.

A source deadline is semantic when it affects correctness.

A physical gate duration is a target fact.

---

25. Resilience Boundary

Resilience is an orchestration and decision layer.

It MAY select:

- retry;
- restart;
- resume;
- rollback;
- remap;
- reroute;
- reschedule;
- recompile;
- reoptimize;
- change QEC;
- mitigate;
- switch backend;
- quarantine resource;
- abort.

The grammar MUST NOT implement these actions.

Source-level resilience policy MAY constrain acceptable adaptations.

Acceptance MUST remain verification-driven.

A recovered execution MUST NOT be accepted merely because recovery completed.

---

26. Checkpoint and State Semantics

POCO-REAF MUST NOT imply that arbitrary quantum state can always be serialized and restored.

Checkpoint semantics MUST distinguish:

- classical execution state;
- compiled program state;
- logical checkpoint state;
- measurement-boundary state;
- QEC-supported state;
- provider-supported state;
- reconstructible algorithmic state.

An implementation MUST NOT advertise arbitrary quantum checkpoint/restart semantics unless the underlying target actually supports them.

---

27. Security

Portability MUST NOT weaken security.

The following MUST remain valid across target selection:

- permissions;
- capability requirements;
- identity requirements;
- cryptographic requirements;
- privacy constraints;
- trust constraints.

Secrets MUST NOT become portable source semantics merely because a backend requires credentials.

The Zamani compiler MUST use Safe Rust.

Rust "unsafe" is forbidden.

---

28. Versioning and Forever Compatibility

Every POCO-REAF artifact MUST be associated with:

- language version;
- relevant grammar version;
- relevant semantic/IR version where applicable.

Language evolution SHOULD prefer:

1. backward-compatible additions;
2. explicit versioned extensions;
3. migration tooling;
4. deprecation periods;
5. explicit breaking-version changes.

Existing valid semantics MUST NOT silently acquire different meaning because new hardware technologies appear.

---

29. Dialects and Future Computing

Zamani MAY support dialects for:

- specialized domains;
- vendors;
- experimental technologies;
- future architectures.

A dialect MUST:

- have explicit identity;
- have version information;
- declare capabilities;
- define semantic lowering;
- avoid conflicting core syntax;
- define compatibility;
- avoid hidden target assumptions.

Vendor dialects MUST NOT redefine core semantic identities.

A future accelerator SHOULD be addable without changing the semantics of existing core programs.

---

30. Interoperability

Interoperability with:

- C;
- C++;
- Python;
- OpenQASM;
- Verilog;
- SystemVerilog;
- foreign ABIs;
- external runtimes;

MUST be represented as explicit boundaries.

Foreign code is not automatically POCO-REAF portable.

The interoperability layer MUST record target assumptions introduced by foreign interfaces.

A foreign function declaration MUST NOT cause the core language to inherit foreign platform limits.

---

31. Existing Grammar Integration

The repository already contains multiple grammar/specification surfaces.

They MUST NOT remain competing authorities.

The intended hierarchy is:

Normative Language Specification
            │
            ▼
Canonical Zamani.g4
            │
            ▼
Lexer / Parser
            │
            ▼
AST
            │
            ▼
Semantic Analysis
            │
            ▼
Canonical IR

Integration rules:

- "grammar/Zamani.g4" is the canonical ANTLR grammar after reconciliation.
- "grammar/specification/*" contains normative language contracts.
- "grammar/spec/*" contains detailed/legacy specification material that MUST remain consistent.
- "grammar/grammar.md" is an implementation/conformance reference and MUST NOT become a competing language authority.
- "grammar/Zamani-Grammar.md" remains historical/design material unless constructs are formally promoted.
- "grammar/DESIGN.md" defines the broader frontend architecture.
- "grammar/specification/README.md" defines the specification-directory organization.
- compatibility documents govern evolution.

Conflicts MUST be resolved through the language authority and compatibility process.

---

32. Repository Integration Contract

32.1 Lexer

The lexer MUST tokenize portable syntax without target-dependent interpretation.

32.2 Parser

The parser MUST produce deterministic AST structure and preserve source spans.

32.3 AST

The AST MUST preserve all semantic information needed for POCO-REAF.

It MUST NOT collapse portable constructs into target-specific identities prematurely.

32.4 Semantic Analysis

Semantic analysis determines:

- types;
- effects;
- capabilities;
- resources;
- constraints;
- ownership/resource correctness;
- domain validity.

32.5 Canonical IR

Canonical IR is the semantic bridge between language and implementation.

Quantum constructs MUST lower to "quantum::ir".

They MUST NOT create an independent frontend quantum IR.

32.6 Optimization

Optimization MUST preserve:

- semantics;
- correctness;
- required provenance.

32.7 Routing

Routing consumes canonical semantic information and target capabilities.

32.8 Scheduling

Scheduling consumes:

- dependencies;
- resources;
- timing constraints;
- target facts.

32.9 ZQN

ZQN supplies:

- noise;
- faults;
- target information;

without owning language syntax.

32.10 QEC

QEC supplies detection/correction mechanisms.

It does not own source grammar.

32.11 Hardware HAL

Hardware abstraction exposes capabilities and state.

Source semantics MUST NOT depend on one concrete hardware implementation.

32.12 Resilience

Resilience orchestrates adaptation and recovery while preserving semantic correctness.

32.13 Runtime

Runtime executes verified realizations and reports actual execution state and failures.

---

33. Ownership Matrix

Concern| Grammar| Semantic| Canonical IR| Backend| Runtime
Syntax| Owns| Consumes| No| No| No
Types| Declares| Owns| Represents| Consumes| Consumes
Effects| Declares| Owns| Represents| Consumes| Enforces
Resource requirements| Declares| Validates| Represents| Satisfies| Reports
Target capabilities| Names| Checks| Represents metadata| Owns facts| Reports
Quantum semantics| Declares| Validates| "quantum::ir"| Lowers| Executes
QEC algorithms| No| No| No| No| QEC subsystem
Noise model| No| No| Metadata only| ZQN| Backend/runtime
Routing| No| No| Input| Owns realization| Executes
Scheduling| No| No| Input| Owns realization| Executes
Resilience policy| Syntax only| Validates| Metadata| Consumes| Resilience/runtime
Machine size| Never universal| Requirement| Represents requirement| Reports capacity| Reports availability

---

34. Diagnostics Contract

POCO-REAF diagnostics MUST be structured.

Where applicable, diagnostics SHOULD include:

- diagnostic code;
- severity;
- source span;
- source construct;
- failed requirement;
- available capability;
- relevant target;
- remediation guidance;
- provenance.

Diagnostics MUST NOT expose secrets.

"Unsupported target" is insufficient when a more precise capability failure can be identified.

---

35. Testing Requirements

POCO-REAF requires tests at every architectural boundary.

35.1 Grammar Tests

Test:

- valid portable programs;
- invalid programs;
- boundary syntax;
- nested structures;
- generic programs;
- quantum programs;
- HDL programs;
- hybrid programs;
- distributed programs;
- future/dialect syntax.

35.2 Scalability Tests

Tests MUST demonstrate that the language does not impose artificial limits on:

- qubits;
- cores;
- threads;
- nodes;
- devices;
- tensor dimensions;
- circuit size;
- module count;
- program size.

Tests SHOULD use generated/parameterized cases instead of treating an arbitrary fixed maximum as language capacity.

35.3 Cross-Domain Tests

Required combinations include:

- classical + quantum;
- classical + HDL;
- quantum + HDL;
- quantum + hardware;
- quantum + distributed;
- AI + quantum;
- AI + hardware;
- classical + quantum + distributed;
- classical + quantum + HDL + hardware.

35.4 Target Variation Tests

The same semantic source MUST be tested against multiple target capability profiles.

Tests MUST distinguish:

- successful equivalent realization;
- explicit capability failure;
- explicit resource failure;
- prohibited semantic substitution.

35.5 Determinism Tests

Identical source and compilation context MUST produce identical parsing and semantic results.

35.6 Round-Trip Tests

Where printers/serializers exist:

source
  → lexer
  → parser
  → AST
  → canonical printer
  → parser

must preserve intended semantics.

---

36. Hard-Coding Audit

Every grammar/frontend change MUST be checked for accidental machine assumptions.

Search for:

- "MAX_QUBITS";
- fixed register sizes;
- fixed device counts;
- fixed core counts;
- fixed node counts;
- fixed memory capacities;
- fixed topology indices;
- fixed hardware IDs;
- fixed gate catalogs presented as complete;
- fixed timing grids;
- fixed accelerator counts;
- fixed tensor ranks;
- fixed deployment layouts.

Every discovered constant MUST be classified as:

1. Language semantic requirement.
2. Explicit user resource budget.
3. Target capability.
4. Implementation safeguard.
5. Test fixture.
6. Accidental hard-coding.

Category 6 MUST be removed.

Categories 2–5 MUST NOT leak into universal language semantics.

---

37. No Unsafe Rust

The Zamani implementation MUST use:

- Rust 1.97 or 1.97.1;
- Safe Rust only.

Rust "unsafe" is forbidden.

This applies to:

- lexer;
- parser;
- AST;
- semantic analysis;
- IR lowering;
- compiler infrastructure;
- grammar tooling;
- production test infrastructure.

Performance optimization MUST NOT weaken the safety boundary.

---

38. Performance and Scalability

POCO-REAF requires scalability without pretending that physical resources are infinite.

The implementation SHOULD:

- avoid unnecessary source-to-AST duplication;
- use compact representations where appropriate;
- use iterative worklists for deeply generated structures;
- stream large inputs where practical;
- avoid quadratic scans where scalable indexing is possible;
- avoid fixed-capacity semantic resource collections;
- avoid recursive algorithms where input depth can be generated arbitrarily;
- make resource budgets explicit;
- preserve deterministic behavior where required.

"Infinity" in POCO-REAF means:

«No artificial language-level ceiling.»

It does not mean infinite physical memory, infinite processing power, or infinite execution time.

Actual execution remains bounded by:

- physical resources;
- virtual resources;
- implementation representation;
- security policy;
- execution policy;
- explicitly declared budgets.

---

39. What POCO-REAF Does Not Promise

POCO-REAF does NOT promise:

1. Every program runs on every target.
2. Every target has every capability.
3. Every target satisfies every resource requirement.
4. A single machine-code binary works on every ISA.
5. Arbitrary quantum state can always be checkpointed.
6. Arbitrary timing guarantees can always be preserved.
7. Performance is identical across targets.
8. Hardware-specific features work without declaring requirements.
9. Unsupported operations can be silently approximated.
10. A target can execute a program with insufficient resources.

POCO-REAF DOES promise:

«Target differences are handled explicitly without forcing unnecessary source rewrites or semantic drift.»

---

40. Conformance Levels

Implementations SHOULD identify conformance at these levels.

Level 0 — Syntax Conformance

Lexer/parser correctly implement canonical syntax.

Level 1 — Semantic Conformance

AST and semantic analysis preserve source meaning and enforce:

- type rules;
- effect rules;
- capability rules;
- resource rules.

Level 2 — Canonical IR Conformance

Programs lower to canonical semantic representations.

Quantum programs use "quantum::ir".

Level 3 — Target Conformance

A backend realizes canonical semantics without prohibited substitutions.

Level 4 — Runtime Conformance

Execution, diagnostics, provenance, resource reporting, and failure behavior satisfy the runtime contract.

Level 5 — POCO-REAF Conformance

The implementation demonstrates reusable semantic artifacts and target-independent source semantics across substantially different target profiles.

---

41. File Integration Contract

This document is independently complete as the normative POCO-REAF policy.

Adding or changing downstream implementation files MUST NOT require redefining the principles established here.

Integration is fixed in advance.

File/subsystem| Required integration
"grammar/Zamani.g4"| Implements syntax consistent with this portability model.
"grammar/specification/README.md"| Lists this document as the POCO-REAF specification.
"grammar/specification/scalability-model.md"| Provides detailed scalability rules.
"grammar/specification/compilation-model.md"| Defines compilation behavior under POCO-REAF.
"grammar/specification/execution-model.md"| Defines execution semantics.
"grammar/specification/compatibility.md"| Defines language evolution.
"grammar/specification/hardware-independence.md"| Defines target/resource separation where present.
"grammar/spec/semantics.md"| Preserves semantic separation.
"grammar/spec/syntax.md"| Does not introduce target-specific core semantics.
"grammar/spec/type-system.md"| Preserves portable type meaning.
"src/lexer.rs"| Target-independent tokenization.
"src/parser.rs"| Portable AST construction.
"src/ast/"| Preservation of POCO-REAF semantic information.
"src/semantic.rs"| Validation of types/effects/capabilities/resources.
"src/ir_gen.rs"| Lowering to canonical semantic representations.
"src/quantum/ir/"| Canonical quantum semantic identity.
"src/quantum/zqn/"| Quantum noise/fault/target information.
"src/quantum/scheduling/"| Schedule realization.
"src/quantum/resilience/"| Adaptation and recovery decisions.
Optimization| Semantics-preserving implementation transformation.
Hardware HAL| Target capabilities and state.
Runtime| Verified execution and actual-state reporting.

No downstream subsystem may reinterpret POCO-REAF as permission to silently weaken source semantics.

---

42. Completion Criteria

This file is complete when all of the following are true:

- POCO-REAF is precisely defined.
- Program semantics are separated from target realization.
- Compile-once semantics are distinguished from universal machine-code binaries.
- Resource requirements and target capabilities are separated.
- Quantum portability is defined.
- "quantum::ir" remains the canonical quantum semantic boundary.
- QEC ownership is separated from grammar ownership.
- ZQN ownership is separated from grammar ownership.
- Scheduling ownership is separated from grammar ownership.
- Resilience ownership is separated from grammar ownership.
- HDL/hardware portability is defined.
- Cross-domain execution is defined.
- Artificial machine-size limits are prohibited.
- Resource exhaustion is distinguished from invalid source.
- Target incompatibility is explicit.
- Silent semantic substitutions are prohibited.
- Versioning is defined.
- Provenance requirements are defined.
- Security boundaries are defined.
- Testing requirements are defined.
- Hard-coding audit requirements are defined.
- Rust 1.97/1.97.1 is specified.
- Rust "unsafe" is prohibited.
- File-level integration contracts are defined.

The remaining implementation work belongs to the referenced grammar, specification, compiler, IR, backend, runtime, and test files.

It does not require weakening or redefining this POCO-REAF contract.

---

43. Canonical Principle

Zamani follows this rule:

«Write the computation once. Preserve its meaning once. Compile its semantics once. Realize it according to available capabilities and resources. Never make today's machine limitations tomorrow's language semantics.»

Therefore:

                    ONE PROGRAM
                         │
                         ▼
                 ONE SEMANTIC MEANING
                         │
             ┌───────────┼───────────┐
             ▼           ▼           ▼
          Classical   Quantum       HDL
             │           │           │
             └───────────┼───────────┘
                         ▼
                  CANONICAL IR
                         │
       ┌─────────────────┼─────────────────┐
       ▼                 ▼                 ▼
   CPU/GPU/FPGA        QPU            Distributed
       │                 │                 │
       └─────────────────┼─────────────────┘
                         ▼
                  TARGET REALIZATION
                         │
                         ▼
                       RUNTIME

POCO-REAF is achieved when the semantic program remains stable while realization scales from atom to everywhere, limited only by explicit semantics, implementation capabilities, and resources actually available.This version deliberately corrects the most important architectural ambiguity: “compile once” does not mean one machine-code binary for every ISA/QPU/FPGA; it means the program's canonical semantic compilation remains reusable while target realization is performed below the semantic boundary. That keeps POCO-REAF compatible with quantum::ir, routing, scheduling, ZQN, resilience, hardware HAL, and future targets rather than forcing the grammar to encode today's machines. 