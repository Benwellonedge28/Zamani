Zamani POCO-REAF Specification

Canonical path: "grammar/specification/poco-reaf.md"
Specification class: Normative language and compilation contract
Language: Zamani
Implementation baseline: Rust 1.97 or later, Rust 2021 edition
Rust safety requirement: Production Rust code MUST NOT use "unsafe" Rust.
Primary objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: Support the smallest meaningful computation and arbitrarily large computations, subject to semantic requirements, available resources, implementation capabilities, representation limits, policy, and physical reality.

---

1. Purpose

This specification defines the normative meaning and engineering requirements of POCO-REAF for the Zamani programming language.

Zamani programs MUST express computational meaning independently of unnecessary assumptions about a particular processor, accelerator, quantum processing unit (QPU), field-programmable gate array (FPGA), application-specific integrated circuit (ASIC), simulator, operating system, network, cluster, or deployment environment.

A conforming implementation MUST preserve the defined meaning of a program when compiling or realizing it for compatible targets. It MUST support resource-parametric programs whose scale is determined by program inputs, semantic requirements, available capabilities, and resource availability rather than arbitrary language-wide capacity limits.

The same source program MUST be eligible for different valid realizations without requiring source modification merely because the target changes.

POCO-REAF does not mean that every program can run on every machine, that every target implements every computational domain, or that physical resources are infinite. It means that portability, scaling, and target selection are governed by explicit semantic contracts instead of accidental machine assumptions.

This specification establishes the contract consumed by the lexer, parser, AST, semantic analysis, intermediate representations, compiler, runtime, tooling, and target integrations. It does not claim that a feature is implemented merely because its syntax or specification exists.

2. Normative language

The terms MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, and MAY are normative.

- MUST / MUST NOT: Mandatory requirement or prohibition.
- SHOULD / SHOULD NOT: Strong recommendation; any deviation requires a documented technical justification.
- MAY: Permitted behavior that is not mandatory.
- Conforming implementation: An implementation with evidence that it satisfies the applicable normative requirements.
- Portable program: A program whose specified meaning is not unnecessarily tied to a particular target.
- Realization: A valid mapping of program meaning onto an execution environment.
- Resource-parametric program: A program whose supported scale is determined by its semantics and declared parameters rather than an arbitrary fixed capacity imposed by the language.

Normative requirements apply to all relevant repository components, including generated artifacts and tools that participate in the language implementation.

3. Authority and integration

This document owns the POCO-REAF contract. It MUST be read consistently with the repository’s language, grammar, scalability, portability, compilation, execution, and compatibility specifications.

3.1 Related normative documents

The following documents define complementary responsibilities:

File| Responsibility
"grammar/DESIGN.md"| Overall grammar architecture, ownership, integration boundaries, and freeze governance
"grammar/README.md"| Navigation and subsystem overview
"grammar/specification/grammar-authority.md"| Authority precedence, ownership, dependencies, conformance, and change control
"grammar/specification/language.md"| Language-wide principles and scope
"grammar/specification/language-principles.md"| Foundational language invariants
"grammar/specification/language-scope.md"| Included computational domains and scope boundaries
"grammar/specification/language-version.md"| Language version identity and evolution
"grammar/specification/scalability-model.md"| Resource-parametric scalability and limits
"grammar/specification/portability.md"| Portability semantics and target independence
"grammar/specification/compilation-model.md"| Compilation stages and meaning preservation
"grammar/specification/execution-model.md"| Execution, negotiation, realization, and runtime behavior
"grammar/specification/compatibility.md"| Compatibility across language and ecosystem versions
"grammar/specification/semantic-model.md"| Validated, target-independent program meaning
"grammar/specification/semantics.md"| Normative semantic rules
"grammar/specification/types.md"| Type-system contracts
"grammar/specification/syntax.md" and "grammar/specification/syntax-model.md"| Source syntax and syntax-model contracts
"grammar/specification/extensibility.md"| Extension and dialect boundaries
"grammar/spec/resources.md"| Machine-oriented resource semantics
"grammar/spec/portability.md"| Machine-oriented portability contract
"grammar/spec/semantics.md"| Machine-oriented semantic contract

The table identifies intended integration responsibilities. It MUST NOT be interpreted as evidence that every listed file or mechanism has already passed conformance tests.

3.2 Implementation and grammar boundaries

The following repository interfaces MUST be reconciled through conformance tests:

- "grammar/Zamani.g4": grammar composition root.
- "grammar/antlr/ZamaniLexer.g4": public ANTLR lexer boundary.
- "grammar/antlr/ZamaniParser.g4": public ANTLR parser boundary.
- "grammar/lexer/": lexical definitions and token contracts.
- "src/lexer.rs" and "src/parser.rs": Rust frontend implementation, where applicable.
- "src/ast/": AST representation and construction.
- "src/semantic.rs": semantic analysis, where applicable.
- "src/ir_gen.rs" and "src/ir_verify.rs": IR generation and verification, where applicable.
- The repository’s existing Classical IR: canonical classical intermediate representation.
- "src/quantum/ir/": canonical quantum semantic IR boundary, referred to as "quantum::ir".

Actual implementation behavior MUST be verified against the normative contract. Rust code demonstrates implementation status; it MUST NOT silently redefine language semantics.

3.3 Conflict resolution

If this document conflicts with another specification, grammar, registry, generated artifact, or implementation:

1. Record the conflict and affected contracts.
2. Identify the authoritative owner under "grammar/specification/grammar-authority.md".
3. Do not silently choose whichever behavior is easiest to implement.
4. Resolve the normative language rule through an explicit, reviewed specification change.
5. Update affected machine contracts, grammars, AST mappings, semantic mappings, diagnostics, compatibility records, and tests.
6. Verify the complete affected dependency closure before freezing the change.

A machine-oriented contract MUST remain consistent with the human-readable normative specification. A grammar production alone cannot establish the complete semantics of a feature.

4. Fundamental architecture

POCO-REAF depends on a strict separation between program meaning and target realization.

The intended conceptual pipeline is:

Zamani source
    |
    v
Lexing and parsing
    |
    v
Domain-neutral AST
    |
    v
Structural validation
    |
    v
Name resolution and semantic analysis
    |
    +-- Types
    +-- Effects
    +-- Capabilities
    +-- Resources
    +-- Contracts
    +-- Policies
    +-- Provenance
    |
    v
Validated, target-independent program meaning
    |
    v
Canonical semantic IR boundaries
    |
    v
Optimization and permitted specialization
    |
    v
Capability and resource negotiation
    |
    v
Target-specific lowering and realization
    |
    v
Routing, scheduling, deployment, and execution
    |
    v
Structured outcomes, diagnostics, and provenance

This pipeline is conceptual: an implementation MAY combine stages, perform additional analyses, or execute permitted stages in a different order when doing so preserves the specified contracts.

The following invariants are mandatory:

1. Source syntax MUST NOT be treated as a substitute for semantic validation.
2. Target-independent meaning MUST be established before target-specific realization.
3. Optimization MUST preserve observable behavior within the applicable semantic and numerical contracts.
4. Target selection MUST NOT silently redefine source-level meaning.
5. Physical allocation, calibration, routing, scheduling, and device discovery MUST belong to appropriate downstream components.
6. Domain-specific processing MUST integrate with the common language foundations rather than creating competing universal language models.
7. Every transformation that affects a portable artifact MUST have a defined contract, compatibility implications, and appropriate provenance.

5. The five POCO-REAF guarantees

POCO-REAF comprises five related but distinct guarantees.

5.1 Program Once

Program Once means expressing the intended computation once in Zamani, using explicit semantics and portable abstractions instead of unnecessary target-specific assumptions.

A program MAY express:

- values, types, algorithms, and control flow;
- classical, quantum, HDL, hybrid, and other supported domain operations;
- resource requirements and constraints;
- required capabilities;
- performance, timing, energy, and reliability objectives;
- effects, permissions, contracts, and policies;
- allowed adaptations and fallback behavior;
- domain-specific requirements where they are part of the intended computation.

A portable program SHOULD remain unchanged when only the target’s resource capacity, processor architecture, accelerator configuration, or deployment topology changes.

A source-level change MAY be necessary when the desired behavior itself changes, a required capability is absent, or the programmer explicitly chooses a different algorithm or execution policy.

5.2 Compile Once

Compile Once means that a program can be compiled into a versioned, portable semantic artifact that compatible downstream implementations can validate, transform, specialize, and realize without requiring the programmer to rewrite the source for each target.

A conforming portable compilation workflow MUST distinguish:

1. Source compilation.
2. Construction of a portable semantic artifact.
3. Target-specific specialization or lowering.
4. Production of a target-specific executable or deployment artifact.

These are related stages, not interchangeable promises.

A target-independent artifact MUST preserve enough information to establish its meaning and realization requirements. Depending on the artifact contract, this includes:

- language and specification version identifiers;
- required feature and dialect versions;
- semantic and IR contract versions;
- validated types and domain meaning, or a defined representation from which they can be verified;
- effects and capability requirements;
- resource requirements, constraints, and preferences;
- relevant contracts and policies;
- permitted fallback behavior;
- numerical, timing, determinism, and reproducibility requirements where applicable;
- provenance and integrity metadata;
- compatibility information;
- any required external interfaces or referenced artifacts.

The exact serialization schema belongs to the artifact-format contract, not to this document alone.

A target-independent artifact MUST NOT depend on an undocumented in-memory representation, compiler process state, host pointer, or accidental implementation detail.

Compile Once does not guarantee that a single native machine-code binary can execute on every architecture indefinitely. Native executables, target-specific binaries, and deployment packages may require rebuilding or specialization when their target contracts are incompatible.

The portable semantic artifact is the primary mechanism for preserving reusable program meaning across compatible environments.

5.3 Run Everywhere

Run Everywhere means that a conforming implementation can consider a portable program for execution on any supported target that satisfies its semantic requirements and whose implementation provides a valid realization.

A target may include:

- a small embedded system;
- a single CPU or a multicore system;
- a GPU or another accelerator;
- an FPGA or ASIC;
- a QPU or quantum simulator;
- a heterogeneous CPU/GPU/QPU environment;
- an HPC system, cluster, or distributed environment;
- a cloud or edge deployment;
- a future computational architecture.

These are examples, not a claim that every backend exists or is conformant.

A target MUST be accepted only when the implementation can establish that the required semantics are supported. If it cannot, the implementation MUST report a structured incompatibility or another appropriate failure outcome.

5.4 Run Anywhere

Run Anywhere extends portability across different locations, deployments, and resource configurations.

A program MUST NOT require a specific deployment location, network topology, device identity, or physical allocation unless that requirement is explicitly part of its semantics or declared constraints.

Execution MAY depend on external conditions such as authorization, connectivity, device availability, policy, timing, and data access. Such dependencies MUST be represented and evaluated through the appropriate contracts.

5.5 Run Forever

Run Forever means that the language and its artifacts are designed for controlled evolution across future implementations and computational architectures.

It does not promise that an unchanged compiler, binary, device driver, service, or hardware target will work indefinitely.

Long-term portability requires:

- versioned language and semantic contracts;
- explicit AST, IR, and artifact-format compatibility;
- governed dialect and external-interface versions;
- stable meaning for features declared stable;
- documented deprecation and migration procedures;
- reproducible build metadata where required;
- validation of artifacts before reuse;
- clear behavior for unsupported or unknown versions and features.

A conforming implementation MUST reject, migrate, or handle incompatible artifacts according to the applicable versioning contract. It MUST NOT silently interpret unknown semantic requirements as though they were understood.

6. Scalability from tiny to arbitrarily large computations

Zamani MUST support scale-independent semantics wherever the underlying computation permits them.

The language MUST NOT impose arbitrary finite ceilings on:

- qubits or logical qubits;
- classical values, registers, or data structures;
- processors, cores, threads, or workers;
- GPUs, FPGAs, ASICs, QPUs, or accelerators;
- memory, storage, or communication resources;
- tensor dimensions or ranks;
- network participants, distributed nodes, or devices;
- operation counts, circuit depth, module counts, or deployment size.

This prohibition applies equally to disguised constants, implicit fixed arrays, fixed-size global registries, hidden parser caps, undocumented maximum counts, and serialization schemas that arbitrarily limit a scalable domain.

It does not prohibit finite limits that are intrinsic to a specified representation, security policy, implementation, target, or physical system. Such limits MUST be explicit, appropriately owned, and reported rather than presented as universal language semantics.

6.1 Meaning of unbounded scalability

“Infinity” is an architectural objective: Zamani MUST NOT impose arbitrary finite capacity limits where the language semantics do not require them.

It is not a claim of infinite physical execution.

Every concrete process, file, compilation, artifact, memory allocation, device, and execution is subject to actual constraints, which can include:

- available memory and storage;
- representation and address-space limits;
- compiler and runtime capabilities;
- available hardware and software services;
- time, energy, latency, and cost budgets;
- security policy and authorization;
- physical and mathematical constraints;
- explicitly declared requirements.

A conforming implementation MUST distinguish a genuine semantic incompatibility from resource exhaustion or a documented implementation limitation.

6.2 Resource-parametric programs

Where the algorithm and semantics allow, programs SHOULD express scale through values, symbolic dimensions, input sizes, generic parameters, and resource requirements.

For example, a conceptual requirement may express:

requires memory >= required_memory
requires qubits >= n
requires capability("quantum.measurement")
requires capability("gpu.compute")
requires capability("tensor.compute")
requires topology(...)

These examples illustrate semantic intent; their precise source syntax MUST be established by the authoritative syntax and resource specifications.

The language MUST NOT require the programmer to encode a universal machine capacity to express these requirements.

6.3 Scale must preserve meaning

Scaling MUST NOT silently change:

- type correctness;
- required effects;
- ordering guarantees;
- synchronization semantics;
- numerical accuracy guarantees;
- quantum measurement semantics;
- HDL timing or clocking contracts;
- distributed consistency guarantees;
- security properties;
- externally observable behavior.

If a larger scale requires a different algorithm, approximation, numerical precision, fault model, or consistency model, the change MUST be permitted by the program’s declared semantics and policies.

7. Resource and capability model

Resources and capabilities are separate concepts.

A resource is a quantity, service, or constrained facility that a computation requires, consumes, reserves, or uses.

A capability is an operation or property that an environment supports.

For example, sufficient memory does not prove that a target supports quantum measurement; a quantum measurement capability does not prove that sufficient qubits or memory are available.

The language and implementation MUST preserve this distinction.

7.1 Requirements

A requirement states a condition necessary for a valid realization. Requirements MUST be evaluated against the semantic contract and the candidate environment.

An unmet mandatory requirement MUST prevent execution under that realization unless a separately declared and semantically valid transformation satisfies the requirement.

7.2 Constraints

A constraint restricts the set of valid realizations. A target that violates a mandatory constraint MUST NOT be selected as a valid realization.

7.3 Preferences and hints

A preference influences selection but is not mandatory. A hint provides optimization guidance.

Preferences and hints MUST NOT override mandatory requirements, authorization, safety, contracts, or semantic correctness. A hint MUST NOT silently change observable meaning.

7.4 Negotiation

Target and resource negotiation SHOULD follow a deterministic, inspectable process:

1. Discover candidate targets and execution environments.
2. Determine available capabilities and resource information.
3. Evaluate mandatory requirements.
4. Apply constraints and policies.
5. Rank eligible realizations using preferences and declared objectives.
6. Select a valid realization or return a structured failure.
7. Record relevant decisions and provenance.
8. Revalidate assumptions that may have changed before execution.

The actual discovery, reservation, negotiation, and placement mechanisms belong to the compiler, runtime, deployment system, or target adapters. The grammar expresses intent; it MUST NOT perform physical allocation.

Resource availability can change between compilation and execution. Implementations MUST handle stale resource assumptions according to the execution and compatibility contracts.

8. Target-independent semantic boundaries

A Zamani program’s portable meaning is established by the language’s common semantic model and the relevant domain specifications.

All domains MUST reuse the common concepts for types, effects, capabilities, resources, contracts, policies, diagnostics, provenance, and compatibility where applicable.

The following boundaries are normative:

Concern| Owning layer
Valid source forms| Lexical and grammar specifications
Parsed source structure| AST contract and implementation
Type and operation meaning| Normative semantic specifications and semantic analysis
Effects and their composition| Effect model
Capability requirements| Capability model and semantic validation
Resource requirements and constraints| Resource model
Authorization and permitted adaptation| Security and policy contracts
Target discovery| Target and runtime infrastructure
Physical device selection| Target realization
Quantum logical-to-physical mapping| Quantum lowering and routing infrastructure
Scheduling and placement| Compiler/runtime/deployment infrastructure
Calibration and device control| Hardware and target integrations
Canonical intermediate representation| The designated IR owner for the domain

8.1 Classical computing

Classical computation MUST use the shared language foundations and the repository’s existing Classical IR. Scaling from scalar computation to vectors, tensors, parallel execution, or distributed workloads MUST NOT require a competing universal type or semantic system.

8.2 Quantum computing

Quantum syntax and semantics MUST integrate with the existing "quantum::ir" boundary. This specification MUST NOT define another quantum IR.

Portable quantum meaning MUST remain distinct from physical qubit allocation, routing, device calibration, pulse realization, and QPU selection.

Quantum operation extensibility SHOULD use the repository’s registered operation and dialect contracts rather than requiring a permanent core grammar enumeration of every possible operation or vendor instruction.

Quantum simulation MAY be used only when it satisfies the declared semantics and policies. A simulator MUST NOT be assumed to be an equivalent replacement for a physical quantum execution when the program requires physical effects or capabilities.

8.3 HDL and hardware design

HDL and hardware/software co-design MUST integrate through their designated semantic and IR contracts. Source-level hardware intent may include widths, timing, interfaces, clocking, protocols, and constraints where defined by the language.

A program-defined width or design parameter is not the same as a universal implementation limit. The language MUST NOT hard-code a global maximum bus width, register count, clock count, or device capacity merely because a particular target has a finite implementation.

8.4 Hybrid and other domains

Hybrid classical/quantum, AI, data, tensor, networking, concurrency, distributed, embedded, and future domains MUST use shared semantic foundations and explicit integration contracts.

Application-specific concepts SHOULD be provided through libraries, registered dialects, capability identifiers, policies, and application interfaces rather than being permanently embedded in universal grammar solely to name a use case.

9. Meaning-preserving compilation and optimization

Compilation and optimization MUST preserve all observable behavior required by the source-language semantics and applicable contracts.

Permitted transformations MAY include:

- changing internal representations;
- selecting compatible instructions;
- parallelizing independent work;
- distributing computation;
- choosing alternative valid memory layouts;
- specializing generic parameters;
- mapping logical quantum operations to supported target operations;
- scheduling or routing operations;
- selecting a permitted implementation of an abstract operation.

A transformation MUST NOT violate required effects, ordering, synchronization, numerical accuracy, timing, security, quantum, hardware, or distributed-system guarantees.

Where equivalence depends on explicit assumptions, those assumptions MUST be represented in the relevant semantic or target contract.

9.1 Specialization

Target-specific specialization MAY generate a different executable or deployment artifact from the same portable semantic artifact.

Specialization MUST record or otherwise preserve the requirements, assumptions, and compatibility information necessary to validate the result.

A specialization MUST NOT be treated as portable to a different target merely because it originated from a portable program.

9.2 Determinism and reproducibility

The language, compilation, and execution specifications MUST define when deterministic behavior is guaranteed and when nondeterminism is permitted.

If reproducibility is required, implementations MUST preserve the applicable reproducibility contract and record the metadata needed to verify it.

The presence of parallelism, concurrency, random-number generation, quantum measurement, external input, distributed execution, or adaptive execution MUST NOT be treated as an automatic guarantee of deterministic results.

10. Explicit fallback and adaptation

A conforming implementation MUST distinguish a valid fallback from an unapproved semantic substitution.

Fallback behavior MUST be explicitly permitted by the program’s requirements, contracts, and policies or by another applicable normative rule.

Examples of possible decisions include:

- use a permitted quantum simulator when physical quantum execution is not required;
- use a different compatible accelerator;
- select a different valid scheduling strategy;
- distribute work across a different topology;
- reject execution when mandatory capabilities are absent.

These examples are not blanket permission to substitute implementations.

A fallback MUST preserve mandatory semantics and guarantees. An approximation, relaxed precision, changed consistency model, changed fault model, or changed computational domain MUST NOT be silently substituted where it alters the required result.

If no valid realization exists, the implementation MUST report the incompatibility or failure through structured diagnostics or execution outcomes defined by the relevant contracts.

10.1 Adaptive execution

Adaptive execution MAY respond to changes in resource availability, load, topology, or target condition when the program and policies permit it.

Adaptation MUST be governed by explicit authorization, effects, capabilities, resources, contracts, and provenance as applicable.

Adaptation MUST NOT mean unrestricted modification of the program’s semantics or arbitrary self-modifying behavior.

Every permitted adaptation MUST preserve mandatory invariants. If preservation cannot be established, the implementation MUST stop, reject the adaptation, or follow an explicitly specified recovery policy.

11. Security and safe implementation

Production Rust code implementing Zamani MUST use safe Rust and MUST NOT use "unsafe" Rust.

This requirement applies to Zamani-owned production implementation code, including frontend components, validators, artifact handling, semantic analysis, IR processing, compiler infrastructure, and supporting tooling.

The implementation MUST NOT bypass semantic validation, resource checks, authorization, policy enforcement, artifact verification, or compatibility checks merely to enable portability or performance.

Portable artifacts MUST be treated as untrusted until the required parsing, integrity checks, version checks, semantic validation, and policy checks have succeeded.

The implementation MUST define appropriate failure behavior for malformed, unsupported, corrupted, incompatible, or unauthorized artifacts. Failures MUST NOT silently result in execution under weaker semantics.

This Rust implementation restriction is independent of any separately specified source-language construct. A source-level feature MUST have its own explicit semantics and safety contract; it MUST NOT justify introducing "unsafe" Rust into the implementation.

12. Versioning and compatibility

POCO-REAF depends on explicit compatibility contracts.

The following version identities MUST be distinguished where applicable:

- language version;
- specification revision;
- grammar and lexical contract version;
- AST version;
- semantic-model version;
- IR version;
- portable artifact-format version;
- dialect and extension versions;
- target, ABI, and backend contract versions.

A change to one version identity MUST NOT automatically be treated as a change to every other identity. The compatibility specification determines which identities must change and which combinations remain valid.

12.1 Stable behavior

Features declared stable MUST have documented semantics and compatibility expectations.

A change that alters the meaning of valid existing programs MUST be handled as a compatibility-affecting language change under the versioning policy.

12.2 Unknown features and versions

An implementation MUST NOT silently ignore an unknown mandatory feature, effect, capability requirement, policy, IR operation, or semantic contract.

It MUST reject the artifact, request an explicitly defined migration, or use another behavior authorized by the applicable compatibility specification.

12.3 Forever does not mean no maintenance

Long-term portability requires maintained specifications, artifact readers, migration procedures, and compatible implementations. It does not guarantee that every future device or software environment will support every historic feature.

13. Diagnostics, outcomes, and provenance

A POCO-REAF implementation MUST make incompatibilities observable and actionable.

Where relevant, diagnostics SHOULD identify:

- the failed requirement or unsupported capability;
- the relevant source span or artifact location;
- the target or execution environment involved;
- whether the issue is semantic, compatibility-related, policy-related, or resource-related;
- whether another permitted realization may be available;
- which contract or version must be satisfied.

The implementation MUST distinguish at least the following conceptual cases:

1. Invalid source or invalid program semantics.
2. Unsupported syntax, feature, dialect, or artifact version.
3. Missing mandatory capability.
4. Insufficient resources.
5. Violated constraint or policy.
6. Failure to find a valid realization.
7. Failure during compilation, deployment, or execution.
8. Recovery or retry, where permitted by the execution contract.

The exact diagnostic identifiers and structured outcome schema belong to the repository’s diagnostic and execution contracts.

Implementations SHOULD record provenance for artifact construction, semantic transformations, target selection, permitted fallback, and other decisions required for auditability or reproducibility.

Provenance MUST NOT be represented as proof that an unverified transformation is correct; verification remains governed by the relevant semantic and compiler contracts.

14. Conformance requirements

A POCO-REAF implementation MUST demonstrate compliance through executable tests and verifiable contracts.

Documentation alone is insufficient.

14.1 Source and semantic conformance

Tests MUST establish, as applicable, that:

- source is parsed according to the declared grammar version;
- AST nodes map to documented semantic constructs;
- types, effects, capabilities, resources, contracts, and policies are validated;
- domain-specific operations map to their designated semantic and IR boundaries;
- invalid or unsupported constructs produce the specified diagnostics.

14.2 Portability conformance

Tests MUST establish, for supported target combinations, that:

- target-independent artifacts retain their declared meaning;
- target selection evaluates mandatory requirements and constraints;
- preferences cannot override mandatory requirements;
- incompatible targets are rejected with appropriate diagnostics;
- specialization preserves the required semantic contract;
- fallback occurs only when permitted;
- unsupported semantic features are not silently discarded.

14.3 Scalability conformance

Tests MUST establish that:

- no arbitrary language-wide capacity ceiling is introduced by the grammar, AST contract, semantic model, IR contract, or artifact format;
- scalable dimensions are represented according to their declared semantics;
- resource limitations are distinguished from semantic invalidity;
- increasing program or resource scale does not silently alter required behavior;
- documented representation or implementation limits are reported accurately.

Tests SHOULD include small, ordinary, and progressively larger parameterized cases, plus symbolic or generated cases where appropriate.

A test using one finite large value does not prove support for infinite execution. Conformance is established by the absence of unjustified fixed limits, validated parameterized behavior, and correct handling of actual resource constraints.

14.4 Reproducibility and compatibility conformance

Tests MUST cover the applicable artifact versions, stable feature contracts, unsupported versions, migration behavior, deterministic-build requirements, and compatibility failures.

14.5 Safety conformance

Continuous integration MUST enforce the prohibition on "unsafe" Rust in Zamani-owned production code. Any exception would violate this specification and require a formal specification change before implementation; performance alone is not sufficient justification.

14.6 Evidence and status

A feature MUST NOT be labeled "IMPLEMENTED", "STABLE", or production-ready solely because:

- its grammar file exists;
- an example parses;
- an implementation stub exists;
- a design document describes it;
- a target name appears in a registry.

Implementation status MUST be supported by the evidence required in "grammar/specification/grammar-authority.md" and the repository’s conformance and status contracts.

15. Integration contract for individual files

Any grammar, specification, registry, schema, or implementation file that participates in POCO-REAF MUST define its integration obligations before it is frozen.

The applicable contract MUST identify:

1. Purpose and authority.
2. Owned concepts and explicit non-ownership.
3. Inputs and outputs.
4. Dependencies, imports, and exported interfaces.
5. Downstream consumers.
6. AST mapping, where applicable.
7. Semantic mapping, where applicable.
8. Canonical IR mapping, where applicable.
9. Resource and capability interactions.
10. Effects, contracts, policies, and provenance interactions, where applicable.
11. Diagnostics and failure behavior.
12. Compatibility and versioning obligations.
13. Scalability and hard-coding audit.
14. Positive and negative conformance tests.
15. Freeze and acceptance criteria.

Files MUST depend on stable contracts, not on undocumented implementation details of downstream components.

A frozen file MUST NOT require semantic rework merely because an unrelated target, backend, device, or domain is added. If a new integration genuinely changes language semantics, it is a language evolution requiring explicit review and compatibility analysis—not a reason to conceal the change in an implementation patch.

The manifest, ownership registry, dependency graph, feature-status registry, and conformance records MUST be kept consistent with the authority rules established by "grammar/specification/grammar-authority.md".

16. Responsibilities outside this specification

This document defines the POCO-REAF requirements; it does not implement every mechanism necessary to satisfy them.

The following remain the responsibility of their designated repository owners:

- grammar and token composition;
- AST design and construction;
- name resolution and semantic validation;
- type and effect systems;
- capability and resource models;
- policy and security enforcement;
- canonical IRs and their verification;
- target discovery and negotiation;
- lowering and backend code generation;
- quantum routing, scheduling, and error-correction execution;
- HDL synthesis and hardware realization;
- deployment, runtime adaptation, and recovery;
- artifact serialization and compatibility tooling;
- diagnostics, conformance fixtures, and automated audits.

Each mechanism MUST have a named owner, a defined interface, and appropriate tests before it can be claimed as implemented.

17. Production acceptance criteria

This specification is ready to serve as the normative POCO-REAF contract when its requirements are reflected consistently across the language and implementation specifications.

The overall POCO-REAF implementation MUST NOT be declared production-ready until the applicable acceptance criteria below have passed.

Language and authority

- [ ] POCO-REAF semantics are consistent with "grammar/specification/grammar-authority.md".
- [ ] Portability, scalability, compilation, execution, and compatibility specifications do not contradict this contract.
- [ ] Grammar and implementation status accurately reflect demonstrated behavior.
- [ ] Every normative rule has an identifiable owner.

Semantic and compilation architecture

- [ ] The AST and semantic model preserve target-independent program meaning.
- [ ] Canonical IR ownership is explicit and consistent.
- [ ] Classical computation uses the designated Classical IR.
- [ ] Quantum computation uses "quantum::ir".
- [ ] HDL and other domains integrate through designated semantic and IR contracts.
- [ ] Compilation and optimization preserve required semantics.

Resource and target independence

- [ ] Requirements, constraints, preferences, and hints have distinct semantics.
- [ ] Capability negotiation is separate from resource sufficiency.
- [ ] No arbitrary universal machine-capacity limits are encoded in grammar, AST, semantic contracts, IR, or artifact formats.
- [ ] Target discovery, physical allocation, routing, scheduling, and calibration remain outside universal source semantics.
- [ ] Incompatible targets fail explicitly.
- [ ] Fallback and adaptation are permitted only under explicit, semantically valid contracts.

Artifacts and evolution

- [ ] Portable artifacts have versioned semantic and compatibility contracts.
- [ ] Unknown mandatory features and unsupported versions are handled explicitly.
- [ ] Target-specific binaries are distinguished from target-independent semantic artifacts.
- [ ] Migration and reproducibility requirements are documented and tested.

Implementation and tests

- [ ] Production Rust implementation uses Rust 1.97 or later and Rust 2021 edition.
- [ ] No "unsafe" Rust is used in Zamani-owned production code.
- [ ] Conformance tests cover source, AST, semantics, resources, capabilities, IRs, portability, scalability, diagnostics, and compatibility as applicable.
- [ ] Negative tests prove that invalid requirements and impermissible fallbacks are rejected.
- [ ] Automated checks detect fixed-capacity assumptions and undocumented implementation ceilings.
- [ ] Every feature reported as implemented has supporting implementation and test evidence.

A checked box represents verified evidence, not an intention or a documentation-only declaration.

18. Final invariant

POCO-REAF MUST preserve this separation:

PROGRAM MEANING
    +
TYPES AND EFFECTS
    +
CAPABILITIES AND RESOURCE REQUIREMENTS
    +
CONTRACTS AND POLICIES
    +
PROVENANCE AND VERSIONED SEMANTICS
    =
PORTABLE PROGRAM CONTRACT

A realization is valid only when that contract is satisfied by the selected implementation and environment:

PORTABLE PROGRAM CONTRACT
    +
TARGET CAPABILITIES
    +
AVAILABLE RESOURCES
    +
COMPATIBLE IMPLEMENTATION
    +
PERMITTED TRANSFORMATIONS
    =
VALID REALIZATION

Zamani MUST remain open to future computational architectures without embedding arbitrary machine-size ceilings into its universal language semantics.

The language expresses what a computation means. Its validated semantic model preserves that meaning. Versioned canonical representations allow it to be reused. Compiler and runtime systems determine how it can be realized. Explicit requirements, capabilities, resources, policies, and compatibility contracts determine whether that realization is valid.

That is the normative foundation of Program Once, Compile Once, Run Everywhere, Anywhere, Forever.