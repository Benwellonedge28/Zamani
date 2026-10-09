Zamani Language Scope

Canonical path: "grammar/specification/language-scope.md"
Status: Normative language-scope specification
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Specification authority: "grammar/specification/"
Architecture authority: "grammar/DESIGN.md"
Implementation baseline: Rust 1.97 or later, Rust 2021 edition
Implementation safety: Production Rust code MUST NOT use "unsafe" Rust.
Primary architecture objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: From the smallest useful computation to arbitrarily large computations, constrained only by semantics, representational limits, implementation capabilities, available resources, explicit policies, and physical reality.

---

1. Purpose

This document defines the scope and boundaries of the Zamani programming language.

It establishes:

1. Which computational domains Zamani is designed to express.
2. Which concepts belong to the common language foundation and which belong to domain extensions.
3. How syntax, AST construction, semantic analysis, intermediate representations, compilation, and execution relate to one another.
4. How one source program can target different compatible machines and computational environments.
5. How programs scale without arbitrary language-imposed limits on machine size, resource counts, or computational topology.
6. How classical, quantum, hybrid, hardware-description, distributed, AI, data, and future computational domains coexist within one language.
7. How new computational domains can be added without unnecessarily redesigning the language core.
8. How the specification integrates with existing grammar files, Rust implementation modules, semantic models, IRs, backends, runtimes, and conformance tests.
9. What must be true before this scope specification and its dependent interfaces can be frozen.

This document defines language scope and architectural boundaries. It does not independently define every grammatical production, type-checking rule, domain algorithm, or backend implementation.

Concrete syntax belongs to the grammar specifications. Normative semantic meaning belongs to the semantic specifications. Concrete implementation behavior must be established through implementation-conformance evidence and tests.

2. Normative terminology

The terms MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, and MAY express normative requirements.

- MUST indicates an unconditional requirement for conformance.
- MUST NOT indicates a prohibited behavior.
- SHOULD indicates a strong recommendation that may be departed from only for a documented reason.
- MAY indicates a permitted option.

These requirements apply to the relevant language specification, grammar, lexer, parser, AST, semantic analyzer, compiler, IR, runtime, tooling, and target integrations.

A normative requirement does not establish that the current repository already implements it. Implementation status MUST be tracked separately.

3. Authority and document boundaries

3.1 Normative authority

The scope specification is governed by the repository's language-authority model.

Artifact| Responsibility
"grammar/DESIGN.md"| Governing architecture, ownership rules, dependency direction, safety, and freeze requirements
"grammar/specification/language-scope.md"| This document: language scope, domain boundaries, and integration obligations
"grammar/specification/language-principles.md"| Language-wide principles and invariants
"grammar/specification/semantic-model.md"| Meaning of valid programs
"grammar/specification/portability.md"| Portability, target independence, and resource-aware realization
"grammar/specification/poco-reaf.md"| Compile-once and execute-across-compatible-targets contract
"grammar/specification/scalability-model.md"| Scalability and resource-independent architecture
"grammar/specification/compilation-model.md"| Source-to-artifact compilation architecture
"grammar/specification/execution-model.md"| Execution, realization, adaptation, and failure behavior
"grammar/specification/domains.md"| Domain definitions, ownership, and integration requirements
"grammar/specification/grammar-authority.md"| Authority boundaries between specifications, grammars, implementation, and semantic models
"grammar/specification/language-version.md"| Language versioning, compatibility, and evolution
"grammar/spec/"| Machine-oriented contracts, schemas, and validation specifications
"grammar/Zamani.g4"| Public grammar composition root, subject to the governing architecture contract
"grammar/antlr/"| Canonical ANTLR lexer/parser composition boundaries
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Historical, extended, or design reference; not an independent normative authority
"src/lexer.rs"| Existing Rust lexer implementation
"src/parser.rs"| Existing Rust parser implementation
"src/ast/"| Existing Rust AST implementation
"src/semantic.rs"| Existing Rust semantic-analysis implementation
Canonical domain IR modules| Canonical representations for their respective computational domains
Compiler, runtime, and backend modules| Compilation, target realization, and execution implementation

No subordinate document, grammar file, generated artifact, or implementation module may silently redefine this file's scope or override a higher-level normative contract.

If existing documents disagree, the conflict MUST be recorded, resolved through the authority process, and reflected in the affected specifications and conformance tests. The conflict MUST NOT be resolved by allowing competing definitions to coexist indefinitely.

3.2 Scope ownership

This file owns:

- The overall language-domain inventory.
- The distinction between universal language foundations and domain-specific extensions.
- The scope-level portability and scalability requirements.
- The high-level separation between source meaning and target realization.
- The domain integration obligations that every feature must satisfy.
- The scope-level exclusions and freeze criteria.

This file does NOT own:

- Concrete lexer token definitions.
- Concrete ANTLR production rules.
- Parser implementation details.
- AST data structures.
- Type-checking algorithms.
- Canonical IR schemas.
- Quantum gate or operation implementations.
- Hardware discovery or device enumeration.
- Physical routing and scheduling algorithms.
- Resource allocators.
- Quantum error-correction algorithms.
- Noise-model implementations.
- Backend code generation.
- Runtime orchestration.
- Operating-system or hardware implementation details.

Those responsibilities remain with their existing authoritative specifications and implementation modules.

4. Fundamental language definition

Zamani is a general-purpose, extensible programming language designed to express computational meaning across different computational models, resource scales, execution environments, and hardware technologies.

Zamani is one language with a shared semantic foundation and multiple interoperable computational domains.

The language is not defined by a particular processor, instruction set, device vendor, operating system, execution location, or current generation of hardware.

The fundamental invariant is:

«Zamani source code describes computation, data, control flow, types, effects, requirements, constraints, policies, and observable behavior. The compiler and runtime determine how that meaning can be realized on a compatible target.»

The language MUST support both abstract, portable computation and explicitly specialized computation.

A program MAY express target-specific requirements when those requirements are genuinely part of its intended semantics or deployment contract. Such specialization MUST be explicit and MUST NOT become an implicit assumption of otherwise portable source code.

5. Computational domains within scope

Zamani's scope includes the following domains.

5.1 Classical and general-purpose computing

The classical domain includes:

- Expressions, values, statements, and control flow.
- Functions, closures, modules, packages, and declarations.
- Primitive, compound, generic, and user-defined types.
- Pattern matching and structured data.
- Memory management, ownership, borrowing, and lifetimes where supported.
- Numerical, symbolic, scientific, and mathematical computation.
- Arrays, collections, vectors, matrices, tensors, and streams.
- Systems programming and operating-system interaction.
- Deterministic and nondeterministic computation where explicitly defined.
- Compile-time and runtime computation.
- Error handling, contracts, and formal correctness properties.

The common language foundation MUST be sufficient to support these features without requiring each computational domain to invent an incompatible type system, expression model, or function model.

5.2 Embedded and constrained computing

Zamani MUST accommodate targets ranging from small embedded systems to resource-rich machines.

Relevant applications include:

- Microcontrollers and embedded processors.
- Sensors, actuators, and controllers.
- Real-time and reactive systems.
- Edge devices.
- Resource-constrained operating environments.
- Firmware and device-facing software.
- Low-memory and low-energy execution environments.

A feature that requires substantial runtime services MUST declare those requirements through the appropriate capability, effect, resource, or execution contract.

The language MUST NOT assume that every target has an operating system, heap allocator, network connection, filesystem, GPU, or large runtime.

Conversely, support for small systems MUST NOT impose an artificial upper limit on the programs that can be expressed for larger systems.

5.3 Systems programming

The scope includes:

- Resource ownership and lifetime management.
- Memory and address-space abstractions.
- Synchronization and concurrency.
- Device and operating-system interfaces.
- Foreign-function interfaces and ABI boundaries.
- Explicit low-level data representation where required.
- System services and hardware-facing abstractions.
- Error handling and failure containment.

The distinction between language-level low-level control and Rust implementation safety is mandatory. Zamani's implementation MUST use safe Rust; a source-language construct that represents low-level or privileged behavior does not authorize unsafe Rust inside the compiler.

5.4 Parallel, concurrent, and high-performance computing

Zamani MUST support expressing:

- Task parallelism.
- Data parallelism.
- Vectorized computation.
- Asynchronous computation.
- Pipelines.
- Actors, tasks, futures, and channels.
- Shared-memory and distributed-memory computation.
- Heterogeneous parallel execution.
- High-performance numerical and scientific computation.
- Resource-aware scheduling intent.

The language MUST NOT require a program to assume a fixed number of threads, cores, workers, execution units, or accelerators.

Parallelism is a computational property. Its physical realization is an implementation decision, subject to program semantics and declared requirements.

5.5 Distributed computing

The distributed domain includes:

- Processes and services.
- Messages, channels, and communication.
- Remote execution.
- Data partitioning.
- Replication and consistency.
- Distributed transactions where supported.
- Collective computation.
- Placement and topology requirements.
- Fault detection, recovery, and resilience.
- Distributed data processing.
- Deployment and execution across heterogeneous environments.

The language MUST NOT encode a fixed cluster size or a finite universal network topology.

A program MAY specify minimum resource requirements, topology constraints, or placement requirements. These MUST be represented as explicit requirements and evaluated against the actual execution environment.

5.6 Quantum computing

Quantum computation is a first-class computational domain.

Zamani MUST provide an extensible path for expressing:

- Logical qubits and quantum registers.
- Quantum states and state transformations.
- Quantum operations and parameterized operations.
- Controlled and composed operations.
- Measurement and observables.
- Reset and dynamic circuits.
- Mid-circuit measurement and classical feed-forward.
- Quantum-classical interaction.
- Quantum simulation.
- Quantum resource requirements.
- Logical and physical realization constraints.
- Noise-aware and fault-tolerant computation.
- Quantum error-correction intent.
- Circuit, pulse, analog, and other supported quantum computational models.
- Future quantum computational models through compatible semantic extensions.

The language MUST NOT require all quantum computation to be represented as a permanently enumerated set of gate names.

New operations and compatible computational models SHOULD be introduced through extensible operation descriptors, dialects, metadata, semantic registration, and canonical IR support rather than unnecessary changes to the universal grammar.

5.7 Hybrid quantum-classical computation

Zamani MUST support a single semantic program that coordinates classical and quantum computation.

This includes:

- Classical preparation of quantum inputs.
- Quantum execution.
- Measurement results.
- Classical post-processing.
- Conditional quantum operations.
- Feedback-controlled execution.
- Iterative quantum-classical algorithms.
- Resource coordination across computational domains.
- Explicit synchronization and data-transfer boundaries.

The hybrid domain MUST reuse the common type, effect, capability, resource, policy, and execution models.

It MUST NOT redefine classical or quantum semantics.

5.8 Hardware description and hardware/software co-design

Zamani's scope includes hardware description and hardware/software co-design.

The language MAY express:

- Hardware modules and components.
- Ports, signals, nets, wires, and registers.
- Memories and interfaces.
- Combinational and sequential logic.
- Clocks, reset behavior, and timing relationships.
- State machines and pipelines.
- Parameterized hardware descriptions.
- Verification properties and assertions.
- Hardware/software interfaces.
- Synthesis intent and physical-design constraints.
- Hardware capabilities and abstract resource requirements.

Hardware descriptions MUST distinguish the logical behavior of a design from its eventual physical realization.

Synthesis, technology mapping, physical placement, routing, fabrication, timing closure, and device-specific resource allocation belong to downstream tools and target contracts.

The language MUST NOT impose universal fixed widths, register counts, lane counts, clock counts, or device capacities.

Widths and dimensions MAY be explicitly specified where they are meaningful to the program or design. Such values are program parameters or design constraints, not universal language limits.

5.9 AI, machine learning, and computational reasoning

The AI and learning domain includes:

- Model inference.
- Training and optimization.
- Differentiable computation.
- Statistical and probabilistic computation.
- Deduction, induction, and abduction.
- Knowledge representation and querying.
- Evidence and provenance.
- Uncertainty and confidence.
- Planning and decision-making.
- Neural-symbolic computation.
- Agents and multi-agent coordination.
- Controlled adaptation.
- Model and dataset processing.
- Reproducible experimentation.

These features MUST use the common language foundations for types, functions, effects, resources, capabilities, policies, contracts, and provenance.

AI functionality MUST NOT assume that every execution environment has a GPU, a particular model runtime, or a fixed tensor capacity.

Application-specific concepts SHOULD be provided through libraries, domain modules, or dialects unless a distinct core-language construct is demonstrably necessary.

Adaptation, learning, reflection, and code generation MUST obey their applicable authorization, effect, capability, policy, resource, and provenance contracts. They MUST NOT grant unrestricted permission to mutate executing programs or bypass semantic validation.

5.10 Data and tensor computation

The data domain includes:

- Structured and unstructured values.
- Arrays, collections, tables, records, and graphs.
- Schemas and schema validation.
- Data transformations and pipelines.
- Streams and incremental processing.
- Queries and filtering.
- Serialization and deserialization.
- Persistence and storage abstractions.
- Tensor and multidimensional computation.
- Data lineage and provenance.
- Statistical and scientific data processing.

Data shape, dimensionality, collection size, and resource needs MUST be represented using the language's normal type and resource mechanisms.

The implementation MUST NOT impose arbitrary language-wide maximum collection sizes or tensor ranks.

External formats and query languages SHOULD be integrated through interoperability interfaces and dialects rather than incorporated wholesale into the universal core syntax.

5.11 Networking, cryptography, and security

The scope includes:

- Network endpoints and communication channels.
- Requests, responses, messages, and protocols.
- Streams and services.
- Routing and discovery abstractions.
- Cryptographic operations.
- Authentication and authorization.
- Identity, permissions, and capabilities.
- Sandboxing and isolation.
- Privacy-preserving computation.
- Audit records and provenance.
- Secure foreign interfaces.
- Security policies and constraints.

Syntax alone MUST NOT be treated as proof of authorization, isolation, cryptographic correctness, or runtime security.

Security enforcement belongs to the appropriate semantic checks, verified libraries, policy engine, compiler, runtime, and target environment.

5.12 Simulation and modeling

Zamani MAY express simulations and models of classical, quantum, physical, mathematical, distributed, and hardware systems.

The scope includes:

- Model definitions.
- Simulation configuration.
- Initial conditions.
- Parameters and constraints.
- Time evolution.
- Event-driven simulation.
- Probabilistic and stochastic models.
- Quantum simulation and noise models.
- Validation and comparison of simulation results.
- Reproducibility metadata.

A simulator is one possible execution realization. Its internal capacity MUST NOT define the universal language's limits.

5.13 Metaprogramming and compile-time computation

The scope includes:

- Macros.
- Syntax-tree quotation and transformation.
- Compile-time functions.
- Controlled reflection and introspection.
- Code generation.
- Generic specialization.
- Type-level computation where supported.
- Dialect and extension registration.

Metaprogramming MUST preserve source locations and diagnostic quality where applicable. Generated syntax MUST pass the same applicable parsing, type, effect, policy, capability, and semantic validation requirements as ordinary source.

Compile-time execution MUST have an explicit resource and termination policy. A lack of arbitrary language-level machine limits does not require the compiler to execute a nonterminating computation or consume unlimited resources.

5.14 Future computational domains

The domain list is extensible and MUST NOT be interpreted as a closed enumeration of all possible future computation.

A new domain MAY be integrated without redesigning the universal language core if it:

1. Defines a distinct semantic purpose.
2. Establishes explicit ownership.
3. Reuses common types, effects, capabilities, resources, contracts, policies, and provenance where applicable.
4. Defines its syntax-to-AST mapping.
5. Defines semantic validation and diagnostics.
6. Maps to an appropriate canonical representation.
7. Defines compatibility and versioning behavior.
8. Provides conformance tests.
9. Does not introduce arbitrary universal resource ceilings.
10. Does not create a competing language authority.

Future domains are therefore expected to extend the architecture through stable interfaces rather than by repeatedly expanding a monolithic grammar.

6. One language, shared foundations

Every computational domain MUST integrate with a common language foundation.

The common foundation includes:

- Lexical rules and source locations.
- Names, identifiers, modules, and visibility.
- Declarations, expressions, statements, and functions.
- Types, generic constraints, and type relationships.
- Effects and effect composition.
- Capabilities and authorization requirements.
- Resource expressions and requirements.
- Contracts, invariants, and properties.
- Policies and execution constraints.
- Error and diagnostic conventions.
- Provenance and relevant source metadata.
- Versioning and compatibility.

Domains MAY define specialized types, statements, expressions, declarations, or semantic constructs when necessary.

However, each extension MUST declare how it integrates with the common foundation. It MUST NOT silently introduce a second incompatible type system, resource system, capability model, effect model, or source-location model.

7. Scale and the meaning of infinity

7.1 Scale-independent language design

Zamani MUST be designed so that the language itself does not impose arbitrary finite ceilings on the scale of computation.

The architecture MUST NOT hard-code universal maxima for:

- CPUs, cores, threads, or workers.
- GPUs, FPGAs, ASICs, or other accelerators.
- QPUs, logical qubits, or physical qubits.
- Memory, storage, or address-space capacity.
- Register width or vector width.
- Tensor rank or collection size.
- Network size, cluster size, or node count.
- Devices, operations, modules, tasks, or execution units.

The prohibition also applies to disguised fixed-capacity assumptions embedded in shared data structures, public APIs, semantic representations, or grammar rules.

This does not prohibit legitimate representation limits, explicitly parameterized program values, or implementation limits that are documented and correctly reported.

7.2 Meaning of infinity

For Zamani, infinity is an architectural openness principle, not a promise of physically infinite execution.

It means the language must not impose an arbitrary finite upper bound where no such bound is required by the language's semantics.

Concrete executions remain limited by the resources and representations available to the relevant implementation.

The following distinction is mandatory:

- Language scope: what can be expressed.
- Program semantics: what the computation means.
- Representation: how the computation is encoded.
- Implementation capacity: what a particular compiler, parser, runtime, or backend can process.
- Resource availability: what is available for a particular execution.
- Target capability: what a target can actually do.
- Physical reality: what the underlying system can support.

A limit in one category MUST NOT be silently reclassified as a universal language limit.

7.3 Tiny-to-large execution

The same semantic program SHOULD remain valid as the available resources change, provided that its requirements and other semantic conditions can still be satisfied.

For example, a program may use a symbolic size parameter and allow its execution realization to select suitable resources.

It MUST NOT need a different algorithm merely because the target has more cores, more memory, additional accelerators, or a larger compatible quantum resource.

Where the program's mathematical result, numerical behavior, ordering, timing contract, or other observable properties depend on execution strategy, those dependencies MUST be defined explicitly. Resource scaling MUST NOT silently weaken correctness guarantees.

7.4 Representation and resource failures

An implementation MAY report that it cannot represent, compile, load, or execute a particular program because of a genuine implementation or resource limitation.

It MUST:

- Report the failure through an appropriate diagnostic or execution result.
- Identify the relevant limitation when reasonably possible.
- Distinguish implementation limits from language-invalid syntax.
- Avoid silently truncating values, dropping operations, or reducing requirements.
- Preserve the source program's semantic identity.
- Permit a compatible implementation to support a larger range without changing the language's meaning.

An implementation limitation MUST NOT be described as a universal semantic restriction unless the normative language specification explicitly requires that restriction.

8. POCO-REAF portability contract

POCO-REAF means Program Once, Compile Once, Run Everywhere, Anywhere, Forever.

It is a language and toolchain architecture objective, not a guarantee that one architecture-specific executable will run unchanged on every conceivable machine.

8.1 Program once

Developers SHOULD express computational meaning once, independently of unnecessary target-specific details.

Changing the number or type of available compatible resources MUST NOT inherently require rewriting the algorithm.

Explicit target-specific requirements remain permitted, but reduce portability to the targets that satisfy them.

8.2 Compile once

The compilation architecture MUST define a stable, versioned, target-independent semantic artifact or equivalent portable compilation boundary wherever the selected language features permit it.

The artifact SHOULD preserve:

- Program meaning and identity.
- Relevant type and control-flow information.
- Domain semantics.
- Effects and capabilities.
- Resource requirements and constraints.
- Policies and contracts.
- Required provenance and source metadata.
- Language and feature versions.
- Canonical IR or serialization versions.
- Dialect dependencies and compatibility information.

A portable artifact may subsequently undergo target-specific specialization, lowering, code generation, linking, or packaging.

Such work MUST preserve the artifact's semantic contract.

"Compile once" MUST NOT be interpreted as requiring every target-specific machine instruction to be generated in advance or as requiring an architecture-specific binary to be universally executable.

8.3 Run everywhere and anywhere

A semantically valid program MAY be realized on any compatible environment that satisfies its required capabilities, resource requirements, constraints, and policies.

Potential targets include:

- Embedded systems.
- CPUs and multicore systems.
- GPUs and other accelerators.
- FPGAs and ASICs.
- Quantum processors and quantum simulators.
- Heterogeneous systems.
- Distributed clusters and supercomputers.
- Edge and cloud environments.
- Future computational substrates.

The target list is illustrative, not exhaustive.

A target that cannot satisfy the program's requirements MUST produce an explicit, appropriate failure or follow an explicitly permitted alternative realization.

8.4 Run forever

Long-term portability requires:

- Versioned language semantics.
- Stable semantic and artifact contracts.
- Explicit compatibility rules.
- Versioned domain and dialect interfaces.
- Reproducibility support where required.
- Migration rules.
- Stable canonical IR boundaries.
- Preservation of historical program meaning.
- Documented deprecation and removal policies.

"Forever" means the language architecture must support continued evolution without unnecessarily invalidating portable programs. It does not guarantee that every historical binary, external service, device, or compiler remains available indefinitely.

9. Requirements, capabilities, resources, constraints, preferences, and hints

These concepts MUST remain distinct throughout the language architecture.

9.1 Requirements

Requirements state what must be true for a program or operation to be validly realized.

Examples include:

- A required capability.
- A minimum resource quantity.
- A required topology property.
- A semantic correctness condition.
- A supported computational model.

A requirement that cannot be satisfied MUST NOT be silently ignored.

9.2 Capabilities

Capabilities describe supported operations or properties of an execution environment.

Examples include:

- "quantum.measurement"
- "gpu.compute"
- "tensor.compute"
- A supported hardware-description feature.
- A supported cryptographic operation.
- A required communication or execution facility.

Capability identifiers MUST be extensible and namespaced where appropriate.

The grammar MUST NOT enumerate every future device, vendor, capability, or implementation.

9.3 Resources

Resources represent quantities or properties that may be required, available, reserved, allocated, or consumed.

Examples include memory, storage, compute capacity, logical qubits, physical qubits, communication capacity, and execution time budgets.

Resource expressions MUST use the language's appropriate expression and quantity models. They MUST NOT introduce universal maximum constants.

9.4 Constraints

Constraints restrict the set of acceptable realizations.

A constraint is mandatory when its declaration specifies mandatory behavior.

An unsatisfied mandatory constraint MUST result in rejection, a documented failure, or another explicitly defined outcome.

9.5 Preferences

Preferences rank otherwise valid realizations.

A preference MUST NOT override a mandatory requirement, security restriction, semantic constraint, or authorization decision.

9.6 Hints

Hints provide nonbinding guidance to a compiler, optimizer, scheduler, or runtime.

A hint MAY be ignored if the implementation cannot apply it.

A hint MUST NOT alter the program's required semantics.

9.7 Resource negotiation

The language architecture MUST permit requirements and capabilities to be evaluated against resources available at compilation, deployment, or execution time, as appropriate.

The general model is:

Source program
    |
    v
Syntax and AST
    |
    v
Semantic validation
    |
    v
Requirements, constraints, capabilities and policies
    |
    v
Portable semantic representation
    |
    v
Target and resource discovery
    |
    v
Feasibility evaluation
    |
    +---- infeasible ----> explicit failure or permitted alternative
    |
    v
Target selection and realization
    |
    v
Execution

The actual division between compile-time and runtime work depends on the relevant contract. The grammar itself MUST NOT perform hardware discovery, resource allocation, or physical target selection.

10. Fallback and adaptive realization

Zamani MAY support alternative execution strategies when they are explicitly permitted and preserve the applicable semantic contract.

Examples include:

- Choosing among compatible CPU and accelerator implementations.
- Using a simulator when a program explicitly permits simulation.
- Selecting another compatible target.
- Deferring execution until required resources become available.
- Applying an optimization that preserves observable behavior.
- Using an explicitly permitted approximation with declared error bounds.

Fallback MUST NOT silently weaken correctness, precision, security, authorization, fault-tolerance guarantees, or other required properties.

A quantum program MUST NOT silently become a classical approximation merely because a quantum processor is unavailable.

A fallback that changes the mathematical model or guarantees MUST require explicit permission and an appropriately defined semantic contract.

Adaptive execution MUST respect:

- Types and effects.
- Required capabilities.
- Resource requirements.
- Contracts and invariants.
- Policies and authorization.
- Provenance and audit requirements.
- Determinism and reproducibility requirements.
- Compatibility guarantees.

11. Semantic architecture and processing pipeline

Zamani's intended integration pipeline is:

Zamani source
    |
    v
Lexical analysis
    |
    v
Parsing
    |
    v
Domain-neutral AST
    |
    v
Structural validation
    |
    +---- names and declarations
    +---- types and constraints
    +---- effects and capabilities
    +---- resources and policies
    +---- contracts and provenance
    |
    v
Domain semantic analysis
    |
    +---- classical semantics
    +---- quantum semantics
    +---- HDL/hardware semantics
    +---- hybrid semantics
    +---- AI and data semantics
    +---- distributed and other domain semantics
    |
    v
Canonical domain representations
    |
    +---- Classical IR
    +---- quantum::ir
    +---- appropriate HDL/hardware IR
    +---- other domain IRs where required
    |
    v
Target-independent transformations
    |
    v
Target-aware specialization and lowering
    |
    v
Capability and resource negotiation
    |
    v
Routing, scheduling and resilience where applicable
    |
    v
Target adapters and execution

This is an architectural model, not a claim that every stage is already implemented or that every program must use every stage.

Implementations MAY combine stages internally when their interfaces and observable contracts remain consistent.

11.1 Lexer and parser integration

The language specification defines intended syntax. The canonical lexer/parser boundary defines how that syntax is processed.

The repository currently contains "src/lexer.rs" and "src/parser.rs", as well as ANTLR grammar files under "grammar/".

The implementation MUST establish a documented conformance relationship between these paths.

If both parsing paths are maintained, they MUST NOT silently accept incompatible interpretations of the same language version. Shared conformance fixtures SHOULD verify equivalent lexical and syntactic behavior.

"grammar/Zamani.g4" and the canonical ANTLR composition boundaries MUST follow "grammar/DESIGN.md" and "grammar/specification/grammar-authority.md".

11.2 AST integration

Every implemented source construct MUST map to an identified AST representation or an explicitly documented intermediate syntax representation.

AST nodes MUST preserve sufficient source-location and diagnostic information for applicable downstream checks.

Domain-specific AST nodes MAY exist, but they MUST integrate with the shared program structure and semantic analysis architecture.

The AST MUST NOT become a second canonical domain IR.

11.3 Semantic integration

Semantic analysis determines whether syntax is meaningful and valid under the language rules.

It is responsible for the applicable:

- Name resolution.
- Type checking.
- Constraint checking.
- Effect analysis.
- Capability checking.
- Resource validation.
- Contract checking.
- Policy checking.
- Domain-specific validation.
- Compatibility checks.

The semantic model MUST be independent of accidental properties of a particular backend or physical device.

11.4 Canonical IR integration

A domain MUST use its designated canonical semantic representation.

The common architecture permits multiple canonical domain IRs. It does not require all domains to be collapsed into one universal IR.

The classical compiler pipeline MUST reuse its established canonical representation.

The quantum compiler pipeline MUST use "quantum::ir" as its canonical quantum semantic boundary.

HDL and hardware compilation MUST use the appropriate canonical hardware representations defined by their owning subsystems.

Additional domain representations MAY be introduced where they are necessary, but their ownership, purpose, versioning, and integration MUST be explicit.

No grammar file may independently define a competing canonical IR.

12. Quantum integration boundary

The canonical quantum semantic boundary is "quantum::ir", implemented within "src/quantum/ir/".

The grammar subsystem owns quantum source syntax, not the canonical quantum IR.

The intended direction is:

Quantum source syntax
    |
    v
AST
    |
    v
Quantum semantic validation
    |
    v
quantum::ir
    |
    v
Optimization and target-independent transformations
    |
    v
Routing and scheduling
    |
    v
Target lowering and calibration
    |
    v
Quantum hardware or simulator

The grammar MUST NOT:

- Assign physical qubit identities implicitly.
- Define vendor-specific physical topology as universal syntax.
- Impose a universal maximum number of qubits.
- Reimplement quantum semantic IR.
- Own routing, calibration, scheduling, or device execution.
- Assume every quantum target supports the same operations or computational models.

Logical qubits and physical qubits MUST remain distinct.

A logical quantum resource describes program-level computation. A physical qubit describes a physical realization or an explicit physical-target requirement.

Quantum operations SHOULD be extensible through generic operation descriptors, parameters, targets, controls, results, attributes, and compatible semantic registrations. A closed, permanently hard-coded gate list MUST NOT be the architectural basis for quantum extensibility.

Quantum error correction, noise modeling, ZQN, optimization, and hardware execution MUST consume the appropriate canonical representations through their established interfaces. They MUST NOT require source syntax to encode implementation-specific resource limits.

13. Hardware and HDL integration boundary

HDL and hardware descriptions MUST integrate with the shared language foundation while preserving hardware-specific semantics.

The architecture MUST distinguish:

1. Hardware behavior and design intent.
2. Abstract hardware capabilities and requirements.
3. Technology-specific implementation.
4. Physical resource allocation and placement.
5. Synthesis, routing, timing closure, and fabrication.

The grammar MAY express parameterized widths, timing requirements, interfaces, memory behavior, state transitions, and other hardware properties.

It MUST NOT convert current implementation capacities into universal language ceilings.

A source-level hardware constraint MAY be exact when the program genuinely requires it. Such a constraint must remain distinguishable from a limit imposed by a compiler or tool.

HDL semantic lowering MUST map to the canonical hardware representation and must not bypass type checking, validation, or compatibility checks.

14. Extensibility and dialects

Zamani MUST support extension without treating every new domain, algorithm, operation, device, vendor, or external format as a new universal-language feature.

Extensions MAY be implemented through:

- Libraries and modules.
- Domain-specific grammar components.
- Versioned dialects.
- Capability registrations.
- Semantic extensions.
- Compiler plugins or adapters where supported.
- Foreign-function and interoperability interfaces.
- Target-specific lowering and backend integrations.

Each extension MUST define:

- A stable identity and namespace.
- Its owner and non-ownership boundaries.
- Its dependencies and exported interfaces.
- Its syntax and lexical dependencies, if any.
- Its AST mapping.
- Its semantic validation.
- Its canonical representation or explicit no-IR rationale.
- Its diagnostics.
- Its compatibility and versioning behavior.
- Its resource and capability requirements.
- Its conformance tests.

An extension MUST NOT redefine existing core syntax or semantics without an explicit, versioned language change.

Application-specific vocabulary SHOULD remain in libraries, domain modules, or dialects unless there is a documented reason to promote a construct into the language core.

15. Implementation language and safety

The repository's implementation baseline is Rust 1.97 or later using the Rust 2021 edition.

Production Rust code MUST NOT use "unsafe".

This requirement applies to new code and to any existing code that is brought into a production-conformance claim under this specification.

The implementation MUST NOT use Rust "unsafe" to bypass ownership, memory-safety, type-safety, or abstraction requirements merely to achieve scale or performance.

Where a dependency contains unsafe implementation details internally, its use MUST be assessed under the repository's dependency and safety policy; it does not authorize unsafe blocks in Zamani's own production code.

The language specification MUST distinguish:

- Source-language syntax.
- Source-language semantics.
- The safety of the Rust implementation.
- Any explicitly supported source-level low-level or unsafe construct.
- The authorization and safety requirements for such source-level behavior.

A source-language construct that describes unsafe operations MUST NOT be confused with permission to use unsafe Rust in the compiler.

Scalability MUST be pursued through sound abstractions, appropriately sized representations, checked conversions, resource-aware algorithms, incremental processing where suitable, and explicit error handling.

16. Compatibility and language evolution

The language scope is intended to remain extensible across future implementations and computational technologies.

Changes to the scope MUST follow the repository's authority and versioning process.

A scope change MUST identify:

- The affected normative specifications.
- The affected grammar and token contracts.
- The AST and semantic mappings.
- The canonical IR boundaries.
- The compiler and runtime integration points.
- The compatibility impact.
- The required diagnostics.
- The conformance tests.
- The effect on POCO-REAF and scalability guarantees.

A future target or backend SHOULD NOT require a scope change merely because its hardware characteristics differ.

A genuine new source-language semantic feature MAY require a versioned language change.

Historical, proposed, experimental, partially implemented, and stable features MUST be distinguishable in implementation-conformance documentation. Presence in this scope document alone MUST NOT imply that a feature is implemented or stable.

17. Diagnostics and failure behavior

When an implementation cannot process a program or realize its requirements, it MUST report an appropriate failure rather than silently changing the program.

Failures MAY arise from:

- Invalid syntax.
- Invalid types or constraints.
- Unsupported language features.
- Unsupported domain operations.
- Unsatisfied capabilities.
- Insufficient resources.
- Conflicting requirements.
- Policy or authorization rejection.
- Incompatible artifact or dialect versions.
- Implementation limits.
- Unavailable targets.
- Runtime failures.

The implementation SHOULD distinguish these cases so that users can determine whether the source program is invalid, the requested realization is unavailable, or the current implementation lacks support.

Diagnostics SHOULD identify relevant source spans, affected requirements, and actionable recovery options where possible.

Failure to realize a program on one target MUST NOT redefine the program's language-level meaning.

18. Testing and conformance obligations

The scope is production-conformant only when its requirements are reflected in the repository's test strategy.

The conformance suite MUST cover, as applicable:

1. Lexical and parser agreement.
2. AST construction and source spans.
3. Type and semantic validation.
4. Effects and capabilities.
5. Resource requirements and constraints.
6. Policies, contracts, and provenance.
7. Classical computation.
8. Quantum computation and "quantum::ir" mapping.
9. HDL and hardware representation.
10. Hybrid domain boundaries.
11. AI, data, and tensor computation.
12. Concurrency and distributed execution.
13. Dialects and interoperability.
14. Compatibility and versioning.
15. Diagnostics and failure handling.
16. Portable artifact behavior.
17. Hard-coded capacity detection.
18. Scaling and resource-negotiation behavior.

Tests MUST include both valid and invalid cases.

Scalability tests SHOULD vary program parameters and available resource descriptions while preserving the same source semantics. They SHOULD verify that a larger resource description is not rejected solely because it exceeds an arbitrary implementation constant.

Portability tests MUST verify semantic equivalence where the relevant language contract promises equivalence. They MUST NOT assume that all targets have identical performance, numerical behavior, timing, or capabilities unless the specification explicitly guarantees those properties.

A conformance result MUST identify the tested implementation, feature set, and relevant version. A passing syntax test alone does not prove semantic or runtime conformance.

19. File-level integration contract

This file MUST be independently maintainable after its normative boundaries and dependencies are agreed upon.

19.1 Required upstream contracts

This document depends on the governing principles in:

- "grammar/DESIGN.md"
- "grammar/specification/language-principles.md"
- "grammar/specification/grammar-authority.md"
- "grammar/specification/semantic-model.md"
- "grammar/specification/language-version.md"

These files establish the language-wide architecture, authority, semantics, and evolution rules.

19.2 Required downstream integration

The following specifications MUST remain consistent with this scope:

- "grammar/specification/portability.md"
- "grammar/specification/poco-reaf.md"
- "grammar/specification/scalability-model.md"
- "grammar/specification/compilation-model.md"
- "grammar/specification/execution-model.md"
- "grammar/specification/domains.md"
- "grammar/specification/types.md"
- "grammar/specification/semantics.md"
- "grammar/specification/effects.md"
- "grammar/specification/resources.md"
- "grammar/specification/quantum.md"
- "grammar/specification/hdl.md"
- "grammar/specification/extensibility.md"
- "grammar/specification/compatibility.md"

Where the repository uses a different existing filename for a responsibility, the established file MUST be located and reused rather than creating a competing specification solely to match this list.

19.3 Grammar integration

The grammar implementation MUST follow the boundaries established here:

- "grammar/Zamani.g4" remains the public composition root under the architecture contract.
- "grammar/antlr/" provides the canonical ANTLR composition boundaries.
- "grammar/lexer/" owns lexical grammar components and token contracts.
- "grammar/core/" owns shared language foundations.
- Domain directories own their domain-specific syntax.
- "grammar/spec/" carries machine-oriented contracts and validation inputs.
- "grammar/grammar.md" records implementation conformance rather than replacing normative specifications.

A domain grammar MUST NOT directly own target discovery, physical resource allocation, backend-specific scheduling, or canonical IR schemas.

19.4 Rust implementation integration

The existing implementation includes:

- "src/lexer.rs"
- "src/parser.rs"
- "src/ast/"
- "src/semantic.rs"
- "src/quantum/ir/"

The scope MUST be integrated with these components through explicit implementation mappings and conformance tests.

The implementation MUST NOT be assumed to support a feature merely because a grammar production or specification mentions it.

Any mismatch between intended scope and implemented behavior MUST be recorded in the implementation-conformance reference and tracked to completion.

19.5 Change-control rule

A downstream file changing MUST NOT automatically require this scope document to change.

This file MUST be revised only when the normative scope or one of its owned architectural boundaries genuinely changes.

Changes to implementations, algorithms, target adapters, physical devices, resource discovery, scheduling, or code generation SHOULD be handled by their owning components when the established language semantics remain unchanged.

If a change genuinely affects scope, the change MUST identify the affected dependencies and compatibility implications before integration is considered complete.

20. Production freeze criteria

This file may be declared FROZEN only when all applicable criteria below are satisfied.

Authority and scope

- [ ] The authority hierarchy is consistent with "grammar/DESIGN.md".
- [ ] Scope, semantic meaning, implementation status, and target realization are clearly distinguished.
- [ ] All major existing computational domains are covered.
- [ ] Future domains can be added through explicit extension contracts.
- [ ] No competing authority is introduced.

Portability and scalability

- [ ] POCO-REAF is defined as a semantic and compilation architecture contract.
- [ ] Resource requirements, capabilities, constraints, preferences, and hints are distinct.
- [ ] No arbitrary universal machine-size ceilings are introduced.
- [ ] Genuine representation and implementation limits remain permitted and accurately classified.
- [ ] Fallback and adaptation cannot silently change required semantics.
- [ ] Unavailable capabilities and insufficient resources have defined failure behavior.

Semantic and IR boundaries

- [ ] Grammar, AST, semantics, and IR responsibilities are distinct.
- [ ] The classical IR integration is preserved.
- [ ] "quantum::ir" remains the canonical quantum semantic boundary.
- [ ] HDL and hardware descriptions map through their designated semantic representations.
- [ ] No competing canonical domain representation is created without an explicit architecture decision.

Implementation and compatibility

- [ ] Rust 1.97 or later and Rust 2021 are documented as the implementation baseline.
- [ ] Production Rust code is required to avoid "unsafe".
- [ ] Existing lexer, parser, AST, and semantic implementation paths are acknowledged.
- [ ] Language feature status is not inferred from specification presence.
- [ ] Compatibility and versioning responsibilities are identified.

Validation and integration

- [ ] The dependent specifications have been checked for contradictions.
- [ ] Existing repository paths have been reused where appropriate.
- [ ] Machine-checkable contracts and implementation mappings have been identified.
- [ ] The conformance suite covers scope-level invariants.
- [ ] Hard-coded capacity checks are included in the validation strategy.
- [ ] Required changes are assigned to their actual owning files.
- [ ] No unresolved ownership conflict remains for a concept owned by this document.

A checkbox MUST NOT be marked complete solely because the requirement has been written down. Completion requires evidence appropriate to the requirement.

21. Final normative statement

Zamani MUST remain a single, extensible programming language whose computational meaning is not defined by the capacity or architecture of any particular machine.

Its scope MUST encompass classical, quantum, hybrid, hardware-description, embedded, systems, concurrent, parallel, distributed, AI, data, networking, security, simulation, and future computational domains through a shared semantic foundation and explicit domain interfaces.

The language MUST NOT impose arbitrary finite ceilings on scalable resources. It MUST distinguish program meaning from resource requirements, target capabilities, implementation limitations, execution policies, and physical constraints.

The architecture MUST support portable semantic artifacts, compatible target realization, explicit capability and resource negotiation, versioned evolution, and clear failure behavior.

All domains MUST integrate through their established syntax, AST, semantic, and canonical representation boundaries. In particular, quantum syntax MUST lower through the existing "quantum::ir" architecture rather than create a competing quantum IR.

The required implementation baseline is Rust 1.97 or later, Rust 2021 edition, with no "unsafe" Rust in production code.

The acceptance criterion for this scope is that adding a compatible computational domain, target architecture, or larger resource environment does not require redefining the language's fundamental meaning merely to accommodate that addition.

That is the scope-level foundation for POCO-REAF: write the program's meaning once, preserve it through compilation, and realize it wherever the declared semantics and requirements can be satisfied.