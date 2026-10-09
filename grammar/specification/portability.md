Zamani Portability Specification

Canonical path: "grammar/specification/portability.md"
Document ID: "ZAMANI-SPEC-PORTABILITY"
Status: Normative language specification; implementation conformance must be verified independently
Language: Zamani
Architecture authority: "grammar/DESIGN.md"
Human-readable specification authority: "grammar/specification/"
Machine-contract authority: "grammar/spec/"
Implementation baseline: Rust 1.97 or later, Rust 2021 edition
Rust safety requirement: Production Rust code MUST NOT use "unsafe" Rust
Primary objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Scalability objective: From the smallest meaningful computation to arbitrarily large computations and deployments, subject to semantics, representation, available resources, implementation capabilities, policy, and physical reality
Canonical quantum semantic boundary: "quantum::ir"

---

1. Purpose

This specification defines the normative portability contract of the Zamani programming language.

Zamani MUST express computational meaning independently of unnecessary assumptions about particular processors, accelerators, quantum processing units (QPUs), programmable logic devices, application-specific integrated circuits (ASICs), operating systems, deployment environments, resource counts, and hardware topologies.

A Zamani program SHOULD be written once and remain eligible for compilation and execution across compatible environments without requiring source-code changes merely because the available hardware, machine size, deployment location, or execution strategy changes.

The portability model applies to:

- Classical and systems programming.
- Embedded and resource-constrained computing.
- Numerical, mathematical, and scientific computing.
- Parallel, concurrent, and high-performance computing.
- Distributed, cluster, edge, and cloud execution.
- Graphics, vector, tensor, and accelerator computation.
- Quantum computation, quantum simulation, and quantum error correction.
- Hybrid quantum-classical computation.
- Hardware description, hardware synthesis, and hardware/software co-design.
- Artificial intelligence, machine learning, and neural-symbolic computation.
- Data processing, streaming, networking, and secure computation.
- Interoperability with external languages, runtimes, and execution environments.
- Future computational domains that implement compatible semantic contracts.

The fundamental principle is:

«A Zamani program defines computational meaning. The compiler and execution infrastructure determine how that meaning can be realized using the capabilities and resources actually available.»

Portability is therefore a semantic and architectural property, not merely the ability to generate code for several processor families.

This specification defines required behavior and integration boundaries. It does not assert that every listed domain, backend, runtime, or target is already implemented.

2. Normative language

The terms MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, MAY, and OPTIONAL are normative.

- MUST / REQUIRED: Mandatory for conformance.
- MUST NOT / SHALL NOT: Prohibited.
- SHOULD: Recommended unless a documented technical reason justifies a deviation.
- SHOULD NOT: Discouraged unless a documented technical reason justifies a deviation.
- MAY / OPTIONAL: Permitted but not required.

A conformance claim MUST identify the specification version, applicable feature set, implementation version, and relevant tests or other evidence.

A requirement written in this document MUST NOT be treated as proof that the corresponding grammar production, semantic analysis, compiler transformation, backend, or runtime behavior already exists.

3. Authority and integration

3.1 Specification ownership

This document owns the normative portability contract.

It defines the meaning and obligations of portability, target independence, semantic preservation, resource-dependent realization, and portable compilation artifacts.

It MUST NOT create a second grammar, an independent AST, a competing semantic model, or a competing intermediate representation.

The following files have complementary responsibilities:

Repository path| Responsibility
"grammar/DESIGN.md"| Grammar architecture, ownership, dependency rules, integration boundaries, and freeze governance.
"grammar/README.md"| Navigation and implementation status orientation.
"grammar/Zamani.g4"| Canonical ANTLR grammar composition root, subject to the repository's declared parser architecture.
"grammar/antlr/ZamaniLexer.g4"| Public ANTLR lexer boundary.
"grammar/antlr/ZamaniParser.g4"| Public ANTLR parser composition boundary.
"grammar/lexer/"| Token ownership, lexical rules, literals, keywords, and lexical conformance.
"grammar/core/", "grammar/types/", "grammar/expressions/", "grammar/statements/"| Source-level constructs and their grammatical ownership.
"grammar/resources/"| Resource expressions, requirements, constraints, preferences, and related source-level contracts.
"grammar/effects/"| Effect declarations, effect identities, composition, and effect-related syntax.
"grammar/policies/"| Policy declarations and policy-related syntax.
"grammar/hardware/"| Hardware intent and hardware-description interfaces.
"grammar/classical/", "grammar/quantum/", "grammar/hdl/", "grammar/hybrid/"| Domain-specific source constructs.
"grammar/compile/"| Compilation intent and source-level compilation constructs.
"grammar/execution/"| Execution intent, adaptive execution, simulation, and related source constructs.
"grammar/dialects/", "grammar/interoperability/"| Registered language extensions and external-format boundaries.
"grammar/specification/semantic-model.md"| Language semantic model and meaning of validated programs.
"grammar/specification/execution-model.md"| Execution behavior and execution-context responsibilities.
"grammar/specification/compilation-model.md"| Compilation stages and compilation artifact responsibilities.
"grammar/specification/poco-reaf.md"| The broader POCO-REAF contract and its compilation implications.
"grammar/specification/scalability-model.md"| Detailed scalability requirements and scaling terminology.
"grammar/specification/language-version.md"| Language version identification and version evolution.
"grammar/specification/extensibility.md"| Extension and dialect governance.
"grammar/specification/grammar-authority.md"| Ownership and authority across the grammar subsystem.
"grammar/spec/portability.md"| Machine-oriented portability contract, when validated against this document.
"grammar/spec/resources.md"| Machine-oriented resource semantic contract.
"grammar/spec/determinism.md"| Determinism and reproducibility contract.
"src/lexer.rs"| Existing Rust lexical frontend implementation.
"src/parser.rs"| Existing Rust parser implementation.
"src/ast/mod.rs"| Existing AST implementation and source-structure definitions.
Canonical semantic-analysis subsystem| Type checking, effect checking, capability validation, resource validation, and semantic equivalence obligations.
Canonical Classical IR| Representation of validated classical computation.
"quantum::ir"| Canonical representation of validated quantum computation.
Canonical HDL/hardware IR| Representation of validated hardware and hardware-description intent.
Compiler, optimization, lowering, routing, and scheduling subsystems| Target realization and implementation-specific transformations.
QEC, ZQN, HAL, backend, and runtime subsystems| Quantum error correction, quantum execution/noise interfaces, hardware abstraction, target execution, and runtime services, according to their established ownership.

This table establishes intended integration responsibilities. The repository manifest and implementation contracts MUST verify actual file existence, canonical owners, and current implementation status before any production freeze.

3.2 Conflict resolution

The following rules apply if repository artifacts disagree:

1. "grammar/DESIGN.md" governs grammar architecture and subsystem ownership.
2. The applicable normative documents under "grammar/specification/" govern source-language meaning within their declared scopes.
3. Machine-oriented contracts under "grammar/spec/" MUST agree with the normative documents and MUST be validated mechanically where possible.
4. Grammar productions MUST conform to the source-language specification.
5. Rust implementations MUST conform to the specified language behavior.
6. Generated documentation, examples, historical references, and implementation comments MUST NOT silently override normative requirements.

A conflict MUST be resolved through an explicit, reviewed specification change. Implementations MUST NOT arbitrarily select whichever definition is easiest to implement.

"grammar/Zamani-Grammar.md" remains a reference or historical design document unless a specific section has been explicitly assigned normative authority. "grammar/grammar.md" MUST distinguish specified behavior from implemented, partially implemented, planned, deprecated, and tested behavior.

3.3 No duplicate ownership

Each portability-related symbol, semantic contract, resource concept, capability model, and artifact format MUST have a canonical owner.

Related documents MAY reference a contract, summarize it, or define a narrower extension. They MUST NOT establish independent competing definitions of the same contract.

4. Definitions

4.1 Program

A program is the computation and associated contracts expressed by Zamani source code.

Its meaning may include values, control flow, types, effects, contracts, resource requirements, capability requirements, policies, domain-specific operations, and externally observable behavior.

A program does not inherently identify a physical machine.

4.2 Portable semantic meaning

Portable semantic meaning is the validated meaning of a program independent of any particular compatible target realization.

It includes the program's required behavior and all semantic distinctions that an implementation must preserve, including relevant ordering, effects, ownership, type guarantees, domain semantics, and declared correctness conditions.

4.3 Target

A target is a computational environment for which the compiler or execution infrastructure can construct a realization.

Examples include CPUs, GPUs, FPGAs, ASICs, QPUs, simulators, embedded devices, accelerators, clusters, and heterogeneous or distributed environments.

A target is not necessarily a single physical device. It MAY represent a composite execution environment.

4.4 Capability

A capability identifies an operation or property supported by an execution environment.

Examples include:

- "quantum.measurement"
- "quantum.mid_circuit_measurement"
- "quantum.dynamic_control"
- "gpu.compute"
- "tensor.compute"
- "parallel.execution"
- "distributed.communication"
- "hardware.synthesis"
- "secure.execution"

Capability identifiers are open-world semantic identifiers. A new capability MUST NOT require changing a universal finite enumeration of all possible hardware or future technologies.

A capability name alone does not prove that the current target implements it. The execution environment MUST supply trustworthy capability evidence.

4.5 Resource

A resource is a quantity or service required, consumed, reserved, or supplied by computation.

Resources MAY include compute capacity, memory, storage, bandwidth, communication capacity, quantum resources, timing capacity, energy, concurrency, reliability, or other explicitly modeled properties.

4.6 Requirement

A requirement is a condition that MUST hold for a realization to be valid.

If a required capability or resource cannot be satisfied, the implementation MUST NOT silently treat the requirement as optional.

4.7 Constraint

A constraint restricts the set of permitted realizations.

A constraint MUST have a defined interpretation and MUST NOT be confused with a preference or optimization hint.

4.8 Preference

A preference expresses a desirable but non-mandatory realization characteristic.

A preference MAY be relaxed when necessary, provided the resulting realization still satisfies all requirements, constraints, policies, and semantic guarantees.

4.9 Hint

A hint provides optimization guidance.

A hint MUST NOT change observable program meaning, authorize an otherwise prohibited action, or weaken a correctness requirement.

4.10 Realization

A realization is a validated mapping of portable program meaning onto a concrete target or execution environment.

It MAY include machine code, accelerator code, a quantum circuit, synthesized hardware, a distributed execution plan, a schedule, or another target-specific artifact.

4.11 Portable compilation artifact

A portable compilation artifact is a versioned representation that preserves the information necessary to validate, reuse, lower, or realize a program on compatible targets.

It is not necessarily a native executable and MUST NOT be assumed to be directly executable on every architecture.

4.12 Scale

Scale refers to the amount of computation, data, resources, parallelism, quantum state or circuit resources, hardware structure, or deployment capacity involved in a program.

A scale-dependent value MUST NOT acquire an arbitrary language-wide maximum merely because an implementation or target has a finite capacity.

5. Fundamental portability invariants

A conforming implementation MUST observe the following invariants.

5.1 Meaning precedes placement

Source-level meaning MUST be established before physical target placement.

The parser MUST NOT assign physical processors, GPU identifiers, physical qubits, memory banks, network routes, or target-specific instruction addresses as an implicit consequence of parsing source code.

5.2 Target changes do not imply source changes

Changing the target MUST NOT require rewriting source code solely to accommodate differences in processor count, memory capacity, device topology, instruction sets, or hardware availability.

This guarantee is subject to the program's declared requirements, language semantics, available implementations, and compatibility contracts.

5.3 No universal physical assumptions

The language MUST NOT assume that every target provides the same:

- processor architecture;
- memory hierarchy;
- instruction set;
- execution model;
- parallelism;
- accelerator family;
- quantum operation set;
- physical qubit layout;
- clocking model;
- communication topology;
- storage capacity;
- timing characteristics.

5.4 No artificial universal capacity limits

The language and its universal grammar MUST NOT impose arbitrary finite upper bounds on scalable quantities, including:

- qubits and logical qubits;
- processors, cores, threads, and workers;
- GPUs and other accelerators;
- FPGAs, ASIC resources, and devices;
- memory and storage;
- register and vector widths;
- tensor dimensions and rank;
- modules, declarations, and program constructs;
- nodes, channels, processes, and network participants;
- hardware ports, registers, and generated structures;
- timelines, tasks, and concurrent operations.

Names such as "MAX_QUBITS", "MAX_CPUS", "MAX_GPUS", "MAX_FPGAS", "MAX_NODES", "MAX_MEMORY", "MAX_THREADS", "MAX_REGISTER_WIDTH", "MAX_TENSOR_RANK", "MAX_NETWORK_SIZE", and "MAX_DEVICE_COUNT" MUST NOT define universal language ceilings.

Equivalent disguised constants are prohibited when they serve the same purpose.

This prohibition does not ban finite, explicitly specified values used as ordinary program data, protocol fields, format requirements, target properties, or genuine mathematical constants.

5.5 Real limits remain real

Implementations MAY encounter limits due to:

- actual memory or storage exhaustion;
- compiler or runtime budgets;
- representable integer ranges;
- target architecture restrictions;
- execution quotas;
- security policy;
- provider restrictions;
- physical laws;
- explicit source-level constraints;
- finite hardware and energy resources.

Such limits MUST be represented and reported at the appropriate layer. They MUST NOT be misrepresented as universal Zamani language restrictions.

6. POCO-REAF semantics

6.1 Program once

A programmer SHOULD express the intended computation, correctness requirements, domain semantics, permitted effects, resource requirements, capabilities, and policies in a reusable source program.

The source MUST NOT need to encode incidental target-specific implementation choices unless the programmer explicitly requires such a dependency.

6.2 Compile once

Zamani MUST support the architectural objective of compiling source into a reusable, versioned, target-independent semantic artifact from which compatible target realizations can be derived.

The compiler MAY produce target-specific artifacts from that portable representation.

The phrase "compile once" MUST NOT be interpreted as a guarantee that a single native machine-code binary can execute directly on every processor, QPU, FPGA, ASIC, simulator, or future computational substrate.

6.3 Run everywhere and anywhere

The same source program or compatible portable artifact MUST be eligible for execution on every environment that can satisfy its semantic requirements and has a conforming realization path.

If a target cannot satisfy a required capability, resource, correctness condition, or policy, the implementation MUST report the incompatibility or follow an explicitly authorized alternative.

It MUST NOT silently weaken the program's requirements.

6.4 Forever

Long-term portability MUST be supported through versioned language semantics, artifact schemas, compatibility rules, stable identifiers, migration procedures, and reproducibility metadata.

No specification can promise that every future machine will support every historical feature or that every old native binary will execute forever.

Instead, the language and artifact contracts MUST enable future implementations to identify, validate, migrate, preserve, or explicitly reject older programs and artifacts.

6.5 POCO-REAF invariant

Across every stage, the following relationship MUST hold:

Source meaning → portable semantic artifact → compatible target realization

Target-specific transformations MAY change representation and implementation strategy. They MUST preserve the program's defined semantics unless an explicitly declared, validated contract permits a particular difference.

7. The portability pipeline

The intended compilation and execution architecture is:

Zamani source
    |
    v
Lexical analysis
    |
    v
Parsing and source spans
    |
    v
Domain-neutral AST
    |
    v
Structural validation
    |
    v
Semantic analysis
    |
    +--> name and type validation
    +--> effects and capabilities
    +--> resources and constraints
    +--> contracts and policies
    +--> provenance and compatibility
    |
    v
Canonical semantic model
    |
    +--> Classical IR
    +--> quantum::ir
    +--> canonical HDL/hardware IR
    +--> other governed domain IRs
    |
    v
Target-independent optimization
    |
    v
Capability and resource negotiation
    |
    v
Target-specific lowering and specialization
    |
    v
Placement, routing, scheduling, and resilience
    |
    +--> QEC where applicable
    +--> ZQN interfaces where applicable
    |
    v
HAL and backend
    |
    v
Runtime, device, simulator, or deployment environment

This diagram defines responsibilities, not a requirement that every program execute every stage. An implementation MAY omit irrelevant domain stages or perform valid compiler transformations in a different order, provided the required semantic dependencies and observable guarantees are preserved.

7.1 Lexer and parser

The lexer and parser MUST establish lexical and grammatical validity and preserve the source information required for diagnostics and downstream analysis.

They MUST NOT discover hardware, reserve resources, authorize privileged actions, execute quantum operations, or determine runtime placement.

7.2 AST

The AST MUST represent parsed constructs using the repository's canonical frontend model.

Every portability-relevant construct MUST have a defined mapping from syntax to AST fields or nodes.

Source spans and diagnostics MUST be retained sufficiently to explain invalid requirements, incompatible features, unsupported constructs, and failed semantic validation.

The portability specification MUST NOT introduce an alternative AST that competes with "src/ast/" or any explicitly designated canonical replacement.

7.3 Semantic analysis

Semantic analysis MUST distinguish valid portable intent from invalid or target-dependent intent.

It MUST validate relevant types, effects, capability requirements, resource expressions, constraints, contracts, policies, and domain-specific semantics before a realization is accepted.

A program that parses successfully MUST NOT automatically be considered semantically valid or executable.

7.4 Canonical IRs

Domain-specific constructs MUST pass through their designated semantic representations and canonical IR boundaries.

Classical operations MUST use the repository's canonical Classical IR.

Quantum operations MUST pass through "quantum::ir".

HDL and hardware operations MUST use their designated canonical hardware/HDL representation.

An implementation MUST NOT introduce a second quantum IR or bypass semantic validation by translating source syntax directly into a vendor-specific representation.

7.5 Target realization

Target discovery, negotiation, placement, physical mapping, routing, scheduling, resource acquisition, device calibration, and runtime dispatch MUST remain outside the universal grammar.

The responsible subsystem MUST establish that its proposed realization satisfies the semantic contract.

8. Resource-parametric computation

8.1 General rule

Programs SHOULD express resource needs through semantic values, symbolic expressions, parameters, data-dependent requirements, capabilities, and abstract constraints.

For example:

requires qubits >= required_qubits
requires memory >= required_memory
requires capability("quantum.measurement")
requires capability("gpu.compute")
requires capability("tensor.compute")
requires topology(required_topology)

These are illustrative contract expressions. Their exact syntax MUST conform to the canonical grammar and resource specification.

They MUST NOT be interpreted as allocating physical resources or establishing universal capacity limits.

8.2 Symbolic and dynamic quantities

Resource quantities MAY depend on:

- program inputs;
- compile-time parameters;
- type-level values;
- derived resource expressions;
- input data size;
- algorithmic requirements;
- runtime discovery;
- explicitly negotiated resource profiles.

The language MUST define how each supported quantity is typed, evaluated, validated, and diagnosed.

An implementation MUST NOT silently replace a symbolic requirement with a fixed implementation constant.

8.3 Resource arithmetic

Resource arithmetic MUST define the relevant units, conversions, comparisons, and failure behavior.

When arithmetic exceeds the representable range of the selected semantic type, the implementation MUST apply the language's defined overflow or range-error rules.

It MUST NOT silently wrap a resource requirement into a smaller value or treat arithmetic overflow as evidence that the requirement is satisfied.

8.4 Requirements, constraints, preferences, and hints

These concepts MUST remain distinct.

Concept| Meaning| May be ignored?
Requirement| Necessary condition for a valid realization.| No.
Constraint| Restriction on permitted realizations.| No, while active and applicable.
Preference| Desirable realization characteristic.| Yes, if permitted.
Hint| Optimization guidance without semantic force.| Yes.
Policy| Rule governing permitted actions or choices.| Only according to its defined authority and scope.

For example, a program may require a capability, constrain a precision or latency property, prefer a certain execution family, and provide an optimization hint.

An implementation MUST NOT silently convert a preference into a mandatory requirement or weaken a requirement into a preference.

8.5 Resource availability is not language validity

A program MAY be syntactically and semantically valid but impossible to execute in a particular environment because that environment lacks sufficient resources.

The implementation MUST distinguish at least:

- invalid source syntax;
- invalid program semantics;
- unsatisfied target capability;
- insufficient available resources;
- implementation or representation limit;
- policy rejection;
- temporary execution failure.

The diagnostic category and available recovery information SHOULD identify the relevant stage and failed contract.

9. Capability negotiation and target selection

9.1 Open-world capabilities

Capability identifiers MUST be extensible without requiring the universal grammar to enumerate every processor, device, vendor, operation, or future architecture.

A capability MUST have defined semantics, including its identity, relevant parameters, evidence requirements, compatibility behavior, and validation rules.

A new capability MAY require an extension, registry entry, semantic adapter, backend, or runtime implementation. It MUST NOT require unrelated changes to the universal grammar solely to accommodate a new device model.

9.2 Negotiation procedure

The realization system SHOULD perform the following logical steps:

1. Determine the program's required capabilities and resources.
2. Discover or obtain evidence for the available execution environment.
3. Reject candidates that fail mandatory requirements or constraints.
4. Apply security and execution policies.
5. Evaluate compatible realization strategies.
6. Rank valid candidates using declared preferences and optimization criteria.
7. Construct a realization plan.
8. Validate the plan before committing resources or execution.
9. Execute, monitor, and report the outcome.

Implementations MAY combine or reorder steps where dependencies permit, but MUST NOT accept an invalid realization merely because it is convenient.

9.3 Negotiation failure

If no valid realization exists, the implementation MUST report that fact.

It MAY offer diagnostic information about missing capabilities, unavailable resources, incompatible constraints, unsupported semantics, or possible explicitly permitted alternatives.

It MUST NOT report successful portable execution when the selected target cannot meet the required contract.

9.4 Capability claims

A target's advertised capabilities MUST NOT be assumed to guarantee successful execution in all circumstances.

The implementation MUST distinguish capability presence from current resource availability, authorization, health, calibration, and operational readiness where these distinctions affect correctness.

10. Tiny-to-arbitrarily-large scalability

10.1 Meaning of unbounded language scale

Zamani MUST NOT impose an arbitrary language-defined finite maximum on the size of a computation where the underlying semantic concept is scalable.

The intended scale includes:

single value
    -> small embedded computation
    -> single processor
    -> multicore and parallel computation
    -> accelerator computation
    -> quantum or hybrid computation
    -> distributed computation
    -> cluster and HPC workload
    -> large heterogeneous deployment
    -> future compatible computational environments

This progression does not imply that every computation can be mapped to every step or that every architecture supports every domain.

10.2 No literal infinity guarantee

The word "infinity" MUST NOT be interpreted as a promise of infinite physical memory, unlimited execution time, infinite energy, unlimited parallelism, or completion of arbitrary computations.

It means that Zamani does not impose arbitrary universal capacity ceilings merely because a target or implementation is finite.

Actual computation remains subject to resource availability, algorithmic complexity, termination, representational constraints, policies, and physical reality.

10.3 Resource-sensitive realization

A compiler or runtime MAY select different valid strategies at different scales.

Examples include:

- sequential execution instead of parallel execution;
- bounded-memory streaming instead of materializing an entire dataset;
- partitioning work across compatible devices;
- distributing independent computations;
- using a simulator where an explicitly permitted simulation is appropriate;
- generating hardware structures from parameterized descriptions;
- selecting a different quantum decomposition for a compatible target.

Every selected strategy MUST preserve the program's required semantics and declared correctness guarantees.

10.4 No automatic guarantee of speedup

Portability and scalability MUST NOT be conflated with performance.

A program may remain semantically portable while experiencing different performance, costs, resource utilization, or numerical behavior within the guarantees explicitly defined by the language.

The compiler SHOULD exploit additional resources when doing so is valid and beneficial, but MUST NOT promise linear speedup, unlimited parallelism, or automatic performance improvement.

10.5 Complexity and feasibility

The compiler and runtime SHOULD communicate relevant feasibility constraints when they can be determined.

They MUST NOT claim that a program will complete merely because the language imposes no artificial upper capacity limit.

Resource negotiation and planning MUST account for the requirements that are actually relevant to the selected algorithm and execution model.

11. Portability across computational domains

11.1 Classical computing

Classical operations MUST use the common type, expression, memory, effect, and semantic models.

Integer widths, floating-point behavior, alignment, calling conventions, vectorization, and machine layouts MUST be governed by the appropriate language and target contracts.

A target's native register width MUST NOT redefine a portable source type unless the type's explicit semantics specify a target-dependent representation.

11.2 Quantum computing

Quantum source syntax MUST express quantum intent, logical operations, targets, controls, parameters, measurements, results, and applicable resource or correctness requirements.

Quantum semantic analysis MUST establish whether the requested operations and control flow are valid under the language's quantum semantics.

The pipeline MUST preserve "quantum::ir" as the canonical quantum semantic representation.

The following responsibilities MUST remain separate:

- Source syntax: "grammar/quantum/".
- Quantum semantic validation: the designated semantic subsystem.
- Canonical quantum representation: "quantum::ir".
- Quantum optimization and decomposition: the designated compiler subsystem.
- Physical qubit mapping and routing: the appropriate target-realization subsystem.
- Scheduling: the appropriate scheduling subsystem.
- Error-correction planning and execution: QEC interfaces.
- Quantum execution and noise-related integration: ZQN interfaces.
- Device capabilities and hardware access: HAL and backend interfaces.

A program MUST NOT depend implicitly on a fixed number of physical qubits or on a particular physical qubit numbering scheme.

A logical qubit is not inherently a physical qubit. Logical-to-physical mapping MUST be performed by the appropriate realization layer.

A quantum program MUST NOT be silently replaced by a classical approximation or simulation merely because a QPU is unavailable.

Simulation, approximation, decomposition, and alternative quantum implementations MUST obey the declared semantic and policy contracts.

11.3 Hybrid quantum-classical computing

Hybrid programs MUST use the common semantic model for their classical and quantum components.

Classical control, quantum operations, measurement results, synchronization, and feed-forward MUST have explicitly defined interactions.

A hybrid realization MUST preserve the required data dependencies, ordering, measurement semantics, effects, and declared correctness conditions.

The hybrid grammar MUST NOT redefine the underlying classical or quantum semantic models.

11.4 HDL and hardware/software co-design

HDL source MUST express parameterized hardware intent, interfaces, signals, state, timing requirements, constraints, and verification properties through the designated language contracts.

Hardware synthesis and physical implementation MAY be target-specific.

A source-level width, clock, latency, or structural parameter MUST be interpreted according to its declared semantics rather than treated as a universal hardware capacity.

Physical timing, electrical constraints, clocking rules, and target-specific synthesis restrictions MUST be validated by the relevant hardware and synthesis subsystems.

A hardware design that cannot satisfy its physical constraints MUST be diagnosed. Portability MUST NOT be achieved by silently discarding timing or correctness requirements.

11.5 AI, data, and numerical computation

AI, machine-learning, tensor, data, and scientific-computing constructs MUST use the shared type, effect, resource, capability, policy, and provenance contracts.

A model, tensor, dataset, or workload MUST NOT be assumed to fit in a particular memory capacity or accelerator.

A target-specific representation MAY differ, provided the transformation preserves the declared numerical, statistical, reproducibility, precision, and semantic guarantees.

Training, inference, randomness, distributed updates, and approximate algorithms MUST follow their applicable effect and determinism contracts.

11.6 Concurrency and distributed computing

Source semantics MUST distinguish logical computation, synchronization, communication, ordering, and failure behavior from the number and identity of physical workers.

A task, actor, pipeline, or parallel computation MUST NOT inherently mean one fixed operating-system thread or a particular device.

Distributed realization MAY partition, replicate, relocate, or schedule work when those transformations preserve the program's contracts.

Consistency, ordering, transactions, failure handling, and communication semantics MUST be explicit where they affect observable behavior.

11.7 Networking and external services

Network addresses, service endpoints, protocols, remote execution, and external interfaces MUST use the relevant networking, security, policy, and interoperability contracts.

A portable program MUST NOT silently depend on an undeclared network service, physical address, credential, or provider-specific capability.

Deployment-dependent details MUST be supplied through validated configuration, capability discovery, explicit bindings, or registered integrations.

11.8 Future domains

A new computational domain MUST integrate with the common language architecture.

It MUST declare its syntax ownership, semantic mapping, capabilities, effects, resource contracts, diagnostics, canonical representation where applicable, compatibility rules, and conformance tests.

A new domain MUST NOT require a redesign of the universal language merely to register new operation names, target families, or capabilities.

12. Explicit fallback and adaptation

12.1 Fallback is a semantic decision

Fallback MUST NOT be treated as an unconditional permission to change the algorithm, domain, precision, correctness guarantee, or observable behavior.

A fallback is valid only if it is explicitly permitted and preserves the guarantees that remain mandatory.

12.2 Examples of possible fallback

Depending on the program's declared semantics and policies, an implementation MAY consider:

- another compatible processor;
- another supported accelerator;
- a different valid parallelization strategy;
- a compatible simulator;
- an alternative implementation of the same abstract operation;
- a semantically equivalent distributed plan.

Each candidate MUST be validated against the original requirements.

12.3 Approximation and degraded operation

Approximation, reduced precision, probabilistic guarantees, degraded service, and other deliberate changes MUST be explicitly declared and validated against the relevant contract.

A preference for speed or lower cost MUST NOT authorize an approximation.

If the requested guarantee cannot be maintained, the implementation MUST reject the alternative or request an explicit, authorized change to the program's contract.

12.4 Adaptation

Runtime adaptation MAY change implementation strategy, placement, scheduling, or resource allocation when allowed by the program and applicable policies.

Adaptation MUST NOT silently modify source semantics, bypass type or effect checking, weaken security, or evade resource authorization.

Any adaptation that changes observable guarantees MUST be governed by an explicit contract and appropriate provenance.

12.5 Failure to find a fallback

If no authorized, semantically valid fallback exists, the implementation MUST report failure rather than inventing a compatible realization.

13. Effects, contracts, policies, and provenance

13.1 Effects

Portability MUST preserve declared effects and their semantic relationships.

An optimization or target-specific lowering MUST NOT silently introduce forbidden I/O, network access, mutation, randomness, foreign calls, measurement, adaptation, code generation, or other prohibited effects.

Target implementations MAY realize an effect differently only when the resulting behavior conforms to the applicable effect contract.

13.2 Contracts

Preconditions, postconditions, invariants, assumptions, guarantees, and other correctness properties MUST remain attached to the computation they govern.

A realization MUST satisfy the applicable contracts or report why it cannot.

An implementation MUST NOT remove a correctness condition solely because the selected target cannot enforce it.

13.3 Policies and authorization

Portability MUST NOT bypass policies, sandboxing, authorization, or trust boundaries.

A target's capability does not automatically grant permission to use it.

Resource acquisition, remote execution, hardware access, adaptation, and privileged operations MUST follow the relevant authorization rules.

13.4 Provenance

Implementations SHOULD record sufficient provenance to explain:

- source and semantic artifact identity;
- language and artifact versions;
- applicable dialects and extensions;
- transformations and optimization decisions;
- selected target capabilities;
- relevant resource-negotiation outcomes;
- policy decisions;
- fallback or approximation decisions;
- compilation diagnostics and conformance evidence.

Provenance MUST NOT be treated as proof of correctness unless the relevant evidence is independently validated.

Sensitive information MUST be handled according to the security and privacy contracts.

14. Determinism and reproducibility

14.1 Deterministic semantics

Where Zamani specifies deterministic behavior, target changes and compiler transformations MUST preserve that behavior.

Where the language specifies nondeterminism, randomness, probabilistic behavior, or quantum measurement, implementations MUST preserve the applicable probability, ordering, and observation contracts rather than pretending that all executions are identical.

14.2 Reproducible compilation

The compiler SHOULD support reproducible compilation when the required inputs, toolchain versions, dependencies, configuration, and deterministic-build conditions are controlled.

The artifact MUST identify the relevant inputs and versions sufficiently to determine whether a later reproduction is expected to be equivalent.

14.3 Target-dependent differences

Target-dependent numerical behavior, floating-point contraction, precision, rounding, quantum noise, timing, scheduling, and concurrency MAY differ where the language contract permits those differences.

An implementation MUST NOT silently broaden the permitted behavior beyond the specified contract.

14.4 Nondeterministic environments

If an environment prevents a requested reproducibility guarantee, the implementation MUST identify the relevant limitation rather than claiming deterministic behavior without evidence.

15. Portable compilation artifacts

15.1 Required artifact contract

The portable compilation architecture MUST define a versioned artifact format containing, or securely referencing, the information needed to validate and reuse the compiled program's meaning.

Depending on the artifact's purpose, this includes:

- language and specification version;
- source identity or an appropriate source digest;
- canonical AST or semantic artifact version;
- canonical IR version or versions;
- domain and feature metadata;
- dialect identities and versions;
- type and semantic contract information;
- effect declarations and relevant constraints;
- capability requirements;
- resource requirements and constraints;
- applicable policies and authorization references;
- fallback and adaptation permissions;
- numerical and determinism guarantees;
- provenance and reproducibility metadata;
- compatibility requirements;
- integrity information.

A specific artifact MAY omit fields that are not applicable, but omission MUST be unambiguous and MUST NOT cause a consumer to assume a stronger guarantee than the artifact establishes.

15.2 Portable artifact versus native executable

A portable artifact MUST be distinguished from:

- a native executable;
- a target-specific object file;
- a device program;
- a quantum circuit;
- a synthesized hardware image;
- a deployment manifest;
- a runtime execution plan.

These artifacts may have different compatibility, validation, and execution requirements.

A portable semantic artifact MAY be reused to derive multiple target-specific realizations without reparsing or redefining the original source program.

A target-specific executable MUST NOT be assumed to be portable beyond the environments and compatibility conditions it supports.

15.3 Artifact integrity

Consumers MUST validate artifact structure, version, integrity, dependencies, and applicable compatibility conditions before trusting it.

Malformed, incompatible, corrupted, or unauthorized artifacts MUST be rejected with appropriate diagnostics.

15.4 Artifact evolution

Changes to artifact schemas MUST follow the versioning and compatibility rules defined in "grammar/specification/language-version.md" and the applicable machine contracts.

A consumer MUST NOT silently reinterpret an unknown required field, unsupported semantic feature, or incompatible IR version as though it were understood.

16. Compatibility and long-term evolution

16.1 Versioned contracts

Portability MUST be defined against explicit language, semantic, artifact, IR, and dialect contracts.

A language version identifies a defined language contract. It MUST NOT identify a fixed physical resource capacity or imply support for every future target.

16.2 Backward compatibility

An implementation claiming compatibility with an earlier language version MUST preserve that version's specified behavior within the compatibility guarantees it advertises.

If an older program cannot be supported, the implementation MUST report the incompatibility and identify the relevant version or feature where possible.

16.3 Forward compatibility

A consumer MUST NOT assume that an unknown future construct has known semantics.

Unknown required features MUST be rejected or handled according to an explicit extension contract. They MUST NOT be silently ignored if doing so could change program meaning.

16.4 Migration

Migration tools MAY transform source code or artifacts between versions.

A migration MUST document semantic changes, unsupported features, assumptions, and any required user decisions.

A migration MUST NOT claim semantic preservation when it has introduced an unapproved behavior change.

16.5 Future hardware

A future target SHOULD be integrated through capabilities, resource descriptions, domain extensions, canonical IR lowering, HAL, and backend interfaces.

Adding a compatible target MUST NOT require changing the universal grammar solely to add a processor model, device identifier, physical capacity, or vendor-specific operation name.

17. Implementation safety and resource handling

17.1 Rust baseline

Zamani-owned production Rust implementation code governed by this specification MUST use Rust 1.97 or later and the Rust 2021 edition, unless an explicitly approved repository-wide edition migration supersedes the edition requirement.

Production Rust code MUST NOT use "unsafe" blocks, "unsafe" functions, or other Rust "unsafe" constructs.

The implementation MUST NOT rely on undefined behavior to achieve scalability, portability, parsing performance, resource handling, or target interoperability.

17.2 Checked resource processing

Resource expressions, input sizes, dimensions, counts, offsets, lengths, and arithmetic MUST be validated according to their declared types and semantic rules.

The implementation MUST distinguish malformed input, arithmetic overflow, insufficient resources, and internal failures.

It MUST NOT silently truncate values or wrap arithmetic where doing so could invalidate a resource requirement or change program meaning.

17.3 Dynamic data structures

Where the implementation represents variable-sized resource sets, declarations, devices, or execution plans, it SHOULD use data structures whose capacities follow the actual data and available memory rather than arbitrary language-wide constants.

A library or target MAY impose genuine documented implementation limits. Those limits MUST be reported as implementation or target constraints, not universal language restrictions.

17.4 Dependency review

Dependencies used in the lexer, parser, semantic model, compiler, runtime, or target integrations MUST be reviewed for compatibility, maintenance, resource behavior, and compliance with the project's safety requirements.

A dependency's use of unsafe internals MUST be assessed under the repository's dependency policy; Zamani-owned production code MUST NOT introduce unsafe Rust to work around a dependency limitation.

18. Diagnostics and failure behavior

A conforming implementation MUST distinguish failures according to the layer responsible for them.

Relevant categories include:

- lexical or syntax error;
- invalid semantic construct;
- unsupported required language feature;
- type or effect violation;
- unsatisfied capability;
- insufficient resources;
- incompatible target;
- incompatible artifact or IR version;
- policy or authorization denial;
- unsupported domain operation;
- failed lowering or realization;
- resource acquisition failure;
- runtime failure;
- explicitly permitted degraded execution;
- provenance or integrity verification failure.

Diagnostics SHOULD include the source span where applicable, the failed requirement or contract, the relevant target or artifact context, and actionable remediation information where available.

The implementation MUST NOT report a target-resource failure as a language-level syntax restriction.

It MUST NOT report successful execution when the required semantic contract has not been satisfied.

Error recovery MUST NOT silently erase mandatory resource requirements, effects, contracts, capability checks, or policies.

19. Testing and conformance

Portability is not production-ready merely because the specification is written or the grammar accepts representative examples.

The repository MUST maintain a conformance suite that tests portability across lexical, syntactic, semantic, artifact, compiler, and runtime boundaries.

19.1 Specification consistency tests

Tests MUST verify consistency between:

- "grammar/specification/portability.md";
- "grammar/specification/poco-reaf.md";
- "grammar/specification/scalability-model.md";
- "grammar/specification/compilation-model.md";
- "grammar/specification/semantic-model.md";
- "grammar/specification/execution-model.md";
- "grammar/specification/language-version.md";
- "grammar/specification/grammar-authority.md";
- "grammar/spec/portability.md";
- "grammar/spec/resources.md";
- "grammar/spec/determinism.md";
- the grammar manifest and relevant ownership/dependency contracts.

Any generated or machine-readable representation MUST be checked against its declared authoritative source.

19.2 Lexical and parser tests

Tests MUST cover accepted and rejected syntax for portability-related constructs, including resource requirements, capabilities, constraints, preferences, topology expressions, target-independent execution intent, and domain-specific requirements.

The actual syntax in each fixture MUST match the canonical grammar and lexer contract.

19.3 Semantic tests

Tests MUST establish that:

- requirements are not treated as preferences;
- preferences are not treated as mandatory requirements;
- hints cannot change semantics;
- resource expressions are validated correctly;
- unsupported capabilities are diagnosed;
- policies and effects remain enforceable;
- target-dependent facts do not leak into portable semantic meaning;
- unknown required features are not silently ignored.

19.4 Scaling tests

The conformance suite MUST include small, parameterized, and large-input tests.

Where practical, the same source program MUST be exercised with different input sizes and compatible execution environments.

Tests SHOULD cover:

- increasing input dimensions;
- variable memory requirements;
- parameterized tensor and array sizes;
- increasing logical quantum resource requirements;
- variable hardware parameters;
- increasing task and distributed-workload sizes;
- different worker counts;
- different capability profiles;
- insufficient-resource and overflow conditions.

Tests MUST NOT depend on a single arbitrary maximum being the definition of language validity.

A test harness MAY use bounded values for practical test execution, but those bounds MUST be identified as test parameters, not universal language capacities.

19.5 Cross-target tests

For each supported target family, tests SHOULD establish that the same source program or compatible semantic artifact preserves its required observable behavior.

Target families MAY include CPUs, accelerators, GPUs, FPGAs, ASICs, QPUs, simulators, and distributed systems, according to actual implementation support.

A target MUST NOT be marked supported solely because its name appears in documentation.

19.6 Quantum and hybrid tests

Quantum tests MUST validate the canonical "quantum::ir" mapping, required capability handling, resource negotiation, and preservation of quantum semantics.

Hybrid tests MUST validate classical/quantum data dependencies, measurement behavior, synchronization, and explicit fallback rules.

Simulation MUST NOT be treated as proof that an actual QPU backend exists.

19.7 HDL tests

HDL tests MUST validate parameterized designs, semantic mapping, applicable timing and structural constraints, synthesis integration, and clear diagnostics for unsupported target requirements.

19.8 Negative tests

The suite MUST include cases where:

- a mandatory capability is missing;
- available resources are insufficient;
- a target cannot satisfy a constraint;
- a required operation is unsupported;
- an artifact version is incompatible;
- a proposed fallback changes forbidden semantics;
- a policy denies execution;
- a resource expression overflows;
- a target-specific implementation limit is reached;
- a program is valid in the language but infeasible on a selected target.

These tests MUST verify that failure is reported at the correct layer.

19.9 Determinism and reproducibility tests

Where deterministic behavior is specified, tests MUST compare the relevant semantic results and reproducibility properties across compatible realizations.

Where nondeterminism is specified, tests MUST validate the appropriate statistical, ordering, or probabilistic contract rather than demand identical execution traces without justification.

19.10 Fuzzing and robustness

Lexer, parser, artifact deserialization, resource-expression evaluation, capability negotiation, and relevant semantic-validation components SHOULD be fuzz-tested.

Malformed input MUST NOT cause undefined behavior, silently bypass validation, or be interpreted as a satisfied mandatory requirement.

20. File-level integration contract

This section specifies the obligations that MUST be settled before this document can be frozen.

20.1 Purpose and ownership

- Owns: The normative language-wide portability contract, semantic portability invariants, the meaning of POCO-REAF within portability, resource-independent language limits, and cross-target semantic preservation.
- Does not own: Concrete grammar productions, token definitions, AST structure, resource-model implementation, target discovery, device allocation, compiler optimization algorithms, routing algorithms, scheduling algorithms, QEC algorithms, ZQN implementation, HAL implementation, or backend code.

20.2 Inputs

This document consumes the architecture, language semantics, type and effect contracts, resource model, capability model, policies, execution model, compilation model, determinism requirements, and compatibility contracts established by the repository.

It also depends on the actual implementation's declared capabilities and supported features when determining conformance claims.

20.3 Outputs

This document defines the requirements consumed by:

- source-language and grammar maintainers;
- frontend and AST maintainers;
- semantic-analysis maintainers;
- resource and capability subsystem maintainers;
- Classical IR, quantum IR, and HDL/hardware IR maintainers;
- compiler and target-lowering maintainers;
- routing, scheduling, resilience, and QEC maintainers;
- ZQN, HAL, backend, and runtime maintainers;
- artifact-format and compatibility maintainers;
- testing, diagnostics, and developer-tooling maintainers.

20.4 Dependencies

The document MUST remain consistent with the repository's actual versions of:

- "grammar/DESIGN.md";
- "grammar/specification/language.md";
- "grammar/specification/language-principles.md";
- "grammar/specification/language-scope.md";
- "grammar/specification/language-version.md";
- "grammar/specification/grammar-authority.md";
- "grammar/specification/semantic-model.md";
- "grammar/specification/semantics.md";
- "grammar/specification/compilation-model.md";
- "grammar/specification/execution-model.md";
- "grammar/specification/poco-reaf.md";
- "grammar/specification/scalability-model.md";
- "grammar/specification/extensibility.md";
- "grammar/spec/portability.md";
- "grammar/spec/resources.md";
- "grammar/spec/determinism.md".

The repository manifest MUST determine whether each referenced path exists and what status it has. A referenced path MUST NOT be presumed implemented merely because this document names it.

20.5 Integration requirements

Before this file is frozen:

1. Its normative rules MUST be reconciled with the POCO-REAF and scalability specifications.
2. The machine-oriented portability contract MUST be checked for contradictory rules.
3. Resource and capability terminology MUST match the canonical resource and capability contracts.
4. The semantic model MUST define the observable behavior that portable transformations preserve.
5. The compilation model MUST define the portable artifact and target-specific artifact boundary.
6. The execution model MUST define how resource negotiation, fallback, and failure behave.
7. The versioning model MUST define compatibility expectations for language and artifact changes.
8. The canonical AST and IR owners MUST confirm their mappings and responsibilities.
9. Diagnostics and conformance tests MUST cover the requirements defined here.
10. The manifest, ownership registry, and dependency registry MUST reference this document consistently.

These are integration prerequisites, not claims that the corresponding work has already been completed.

20.6 Stability rule

Once frozen, downstream changes to a compiler backend, runtime, HAL, target, QPU, GPU, FPGA, ASIC, scheduler, or deployment system MUST NOT require revising this document merely to accommodate that implementation.

A change that alters the language's definition of portability, fallback, semantic equivalence, artifact compatibility, or resource requirements MUST undergo the applicable normative specification review.

20.7 Compatibility rule

Changes to this document MUST be version-controlled and reviewed for semantic impact.

Any change that weakens a portability guarantee, changes observable behavior, modifies fallback permissions, or alters the meaning of requirements MUST be treated as a semantic compatibility change.

21. Production freeze criteria

This document MUST NOT be marked fully frozen until all applicable criteria below are verified.

Authority

- [ ] Its ownership agrees with "grammar/DESIGN.md".
- [ ] Its normative scope does not conflict with neighboring specifications.
- [ ] "grammar/spec/portability.md" is consistent with this document.
- [ ] All referenced canonical owners and repository paths have been verified.
- [ ] The manifest and dependency registry identify this file correctly.

Semantics

- [ ] Portable program meaning is clearly defined.
- [ ] Target-specific realization is separated from source semantics.
- [ ] Requirements, constraints, preferences, hints, policies, and effects remain distinct.
- [ ] Resource and capability failures have defined outcomes.
- [ ] Fallback and approximation rules prohibit unauthorized semantic changes.
- [ ] Quantum, classical, hybrid, and HDL boundaries are consistent with their canonical semantic models.

Scalability

- [ ] No artificial universal hardware-capacity ceiling is introduced.
- [ ] Resource arithmetic and representation failures are defined.
- [ ] Symbolic and parameterized requirements have defined validation responsibilities.
- [ ] Test-only bounds are distinguished from language-wide limits.
- [ ] The specification makes no promise of physically infinite resources or guaranteed termination.

Compilation and compatibility

- [ ] The portable artifact contract is defined and integrated with compilation.
- [ ] Portable artifacts are distinguished from target-specific executables.
- [ ] Language, semantic, IR, and dialect compatibility responsibilities are defined.
- [ ] Determinism and reproducibility guarantees are explicit.
- [ ] Migration and unsupported-feature behavior are addressed.

Implementation and verification

- [ ] The Rust baseline and prohibition on "unsafe" are consistent with repository policy.
- [ ] Required AST, semantic, and IR mappings are identified.
- [ ] Positive, negative, boundary, and scaling tests are registered.
- [ ] Unsupported implementation features are not represented as implemented.
- [ ] Diagnostics distinguish language invalidity from target infeasibility.
- [ ] The conformance suite passes for the implementation's claimed support level.
- [ ] Reviewers approve the document's normative scope and version.

A checked box MUST represent verified evidence, not an intention or an assumption.

22. Final portability guarantee

Zamani's portability contract is:

«A conforming Zamani implementation MUST preserve the specified meaning of a program across compatible realizations, subject to the program's declared semantics, requirements, constraints, effects, policies, compatibility contracts, and the resources and capabilities actually available.»

The language MUST support the architectural possibility of scaling a program from a small computation to arbitrarily large workloads without imposing arbitrary universal resource ceilings.

The implementation MUST remain honest about physical limits, unsupported domains, finite representations, target incompatibilities, resource exhaustion, and execution failure.

The complete relationship is:

One Zamani source program
        |
        v
Portable semantic meaning
        |
        v
Versioned semantic artifact
        |
        v
Capability and resource negotiation
        |
        v
Semantically valid realization
        |
        +--> Classical IR and compatible targets
        +--> quantum::ir and compatible quantum targets
        +--> HDL/hardware IR and compatible hardware targets
        +--> Hybrid and distributed realizations
        +--> Future compatible computational domains
        |
        v
Validated lowering, routing, scheduling, and execution

The intended result is one language, one coherent semantic foundation, governed canonical IR boundaries, explicit compatibility contracts, and extensible target realization.

POCO-REAF is achieved architecturally when source meaning remains reusable and target adaptation is governed by explicit contracts—not when every physical machine is assumed to be identical or unlimited.

End of "grammar/specification/portability.md".