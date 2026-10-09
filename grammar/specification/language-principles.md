Zamani Language Principles

Canonical path: "grammar/specification/language-principles.md"
Status: Normative
Scope: Language-wide principles, semantic invariants, portability, scalability, safety, and architectural integration
Language: Zamani
Rust edition: 2021
Minimum Rust version: 1.97
Rust safety requirement: Safe Rust only; "unsafe" Rust is prohibited
Primary objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: From the smallest supported computation to arbitrarily large computations, subject to actual resource availability and the limits inherent in representation, implementation, semantics, and physical reality.

---

1. Purpose

This document defines the fundamental principles governing the design, implementation, evolution, and conformance of the Zamani programming language.

These principles apply to every current and future language feature, including:

- Lexical analysis and parsing.
- Declarations, expressions, statements, functions, modules, and types.
- Classical and systems programming.
- Embedded and resource-constrained computing.
- Parallel, concurrent, distributed, and high-performance computing.
- Numerical, symbolic, scientific, tensor, and data computation.
- Quantum computation, quantum simulation, and quantum error correction.
- Hybrid quantum-classical computation.
- Hardware description and hardware/software co-design.
- Artificial intelligence, machine learning, reasoning, and autonomous agents.
- Networking, cryptography, and secure computation.
- Memory management, ownership, borrowing, and resource lifetimes.
- Effects, capabilities, requirements, constraints, contracts, and policies.
- Compilation, optimization, lowering, scheduling, and execution.
- Interoperability, dialects, metaprogramming, and language evolution.
- Diagnostics, source provenance, reproducibility, and compatibility.

The purpose is not to enumerate every possible computational feature. It is to establish principles under which new features can be added without destabilizing the language or creating incompatible subsystems.

A feature MUST NOT be considered production-ready merely because its syntax has been documented, its grammar production exists, or an implementation prototype compiles.

Production readiness requires conformance with the applicable syntax, semantic, safety, integration, compatibility, and testing contracts.

The governing principle is:

«Zamani specifies computational meaning independently of accidental limitations or implementation choices of the environment that realizes it.»

The language MUST support the architectural objective:

Zamani: From Atom to Everywhere.

It MUST pursue:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).

These objectives govern architecture and language evolution. They do not constitute a claim that every possible program can execute on every physical machine.

2. Normative terminology

The terms MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, and MAY have normative meanings throughout this document.

- MUST / MUST NOT: An unconditional conformance requirement or prohibition.
- SHOULD / SHOULD NOT: A strong recommendation that may be departed from only for a documented, justified reason.
- MAY: A permitted choice that does not violate another requirement.
- Program meaning: The defined semantics of a valid program, including its observable behavior and declared obligations.
- Implementation: A lexer, parser, compiler, runtime, tool, or other system that implements part of Zamani.
- Target: An execution or realization environment.
- Resource: A quantity or service needed, consumed, reserved, or made available by a computation.
- Capability: An operation or property that an environment can provide.
- Requirement: A condition that MUST hold for a valid execution or realization.
- Constraint: A condition restricting the set of permitted realizations.
- Preference: A non-mandatory selection criterion.
- Hint: Optimization guidance that does not change program meaning.
- Semantic model: The validated, target-independent interpretation of a program.
- IR: An intermediate representation with an explicit semantic contract and ownership boundary.
- Realization: The process of mapping validated program meaning to a particular execution environment.
- Conformance: Demonstrable satisfaction of the applicable normative requirements.
- Frozen file: A file whose ownership, dependencies, interfaces, integration mappings, tests, and compatibility obligations have been approved and verified.

Where another specification defines a more specific term, that specification MAY refine the definition but MUST NOT contradict the principles established here.

3. One language, multiple computational domains

Zamani MUST remain one coherent programming language.

Classical, quantum, hybrid, hardware, AI, data, networking, and distributed computation are domains within that language. They MUST NOT become independent languages with incompatible foundational semantics.

The domains MUST share the appropriate common facilities, including:

- Names and module resolution.
- Expressions and types.
- Functions and control flow.
- Source locations and diagnostics.
- Effects and capabilities.
- Resource requirements and constraints.
- Contracts and policies.
- Provenance and compatibility.
- Semantic validation and controlled extensibility.

A domain MAY define specialized syntax, types, operations, or semantic rules when those are genuinely required.

It MUST reuse existing common concepts whenever their semantics apply. It MUST NOT create duplicate universal type systems, expression systems, resource models, capability registries, effect taxonomies, policy engines, or competing canonical representations.

Domain integration MUST be explicit and testable.

A new computational domain MUST identify its syntax, AST mapping, semantic ownership, canonical IR destination, resource and capability requirements, effects, diagnostics, compatibility rules, and conformance tests before its integration is considered complete.

4. POCO-REAF is a semantic contract

POCO-REAF means Program Once, Compile Once, Run Everywhere, Anywhere, Forever.

It is an architectural objective implemented through stable semantics, portable representations, explicit requirements, extensible capabilities, compatibility rules, and target-aware realization.

It is not a keyword, a grammar trick, or a guarantee that every existing machine instruction will execute on every future processor.

4.1 Program Once

A developer SHOULD express the intended computation once, without rewriting the program merely because the available machine, accelerator, quantum processor, memory configuration, or deployment topology changes.

The same source MUST preserve its defined meaning across conforming implementations and compatible execution environments.

Source code MAY express genuine target requirements where they are part of the program's intended meaning. It MUST NOT acquire accidental dependencies on the development machine's configuration.

4.2 Compile Once

The architecture SHOULD support a portable semantic compilation artifact that can be validated, transformed, specialized, and realized for multiple compatible targets.

The artifact MUST preserve the semantic identity of the program and identify the language, semantic, IR, dialect, and compatibility contracts needed to interpret it correctly.

Compilation MUST distinguish target-independent semantic analysis from target-specific realization.

A target-specific binary MAY be produced, but its target restrictions MUST be explicit. It MUST NOT be misrepresented as universally executable.

4.3 Run Everywhere

A valid program MUST be eligible for realization on every environment that satisfies its semantic requirements, resource requirements, capabilities, constraints, policies, and other applicable obligations.

A target that cannot satisfy those obligations MUST be rejected or handled through an explicitly authorized fallback.

The compiler or runtime MUST NOT silently change program meaning to force execution on an incompatible target.

4.4 Run Anywhere

The architecture MUST permit appropriate programs to execute locally, remotely, on embedded systems, on workstations, on servers, on accelerators, on simulators, on quantum hardware, and across clusters, clouds, or distributed systems.

The source language MUST NOT assume a single deployment topology.

4.5 Run Forever

Long-term portability requires stable semantics, versioned representations, explicit compatibility contracts, reproducible artifacts where requested, migration rules, and extensible target interfaces.

Future hardware MUST be accommodated through compatible realization mechanisms rather than by requiring the original source language to enumerate every future device.

Existing programs MUST NOT have their meaning silently redefined by language, compiler, runtime, or target evolution.

5. Scalability without arbitrary architectural ceilings

Zamani MUST be designed to scale from the smallest supported computation to arbitrarily large computations.

Here, unbounded scalability means that the language architecture MUST NOT impose arbitrary finite upper limits on quantities whose size is determined by the program, its mathematical semantics, its explicit requirements, or the environment.

It does NOT mean that physical execution can consume infinite resources or that every implementation can represent every mathematically possible object.

5.1 Prohibited universal ceilings

The language MUST NOT define arbitrary universal limits on:

- Classical values, registers, or program elements.
- Logical or physical qubits.
- Quantum operations, circuit depth, or operation arity.
- CPU count, core count, thread count, or parallel workers.
- GPU, FPGA, ASIC, QPU, accelerator, or device count.
- Memory, storage, registers, vector width, or tensor dimensions.
- Tensor rank, collection size, module count, or operation count.
- Network participants, channels, distributed nodes, or cluster size.
- Hardware topology, resource identifiers, or deployment scale.

The following are examples of prohibited architectural design:

const MAX_QUBITS: usize = 32;
const MAX_CPUS: usize = 128;
const MAX_GPUS: usize = 8;
const MAX_NODES: usize = 256;
const MAX_TENSOR_RANK: usize = 16;

These are prohibited when used to establish supposed universal language or architecture limits.

This prohibition does not prevent an algorithm from declaring a meaningful finite size. For example, a circuit containing a program-defined number of logical qubits or an array with a program-defined length is valid when the relevant type and semantic rules permit it.

A number that belongs to a program's meaning MUST NOT be confused with a limit imposed by the implementation.

5.2 Representation and implementation limits

Implementations MAY encounter genuine limitations involving address spaces, integer representations, available memory, execution budgets, parser capacity, compiler complexity, operating systems, hardware, or physical laws.

Such limits MUST:

1. Be distinguished from language-level restrictions.
2. Be documented at the layer that owns them.
3. Be reported accurately when they prevent an operation.
4. Avoid silently changing program semantics.
5. Be replaceable or extendable without requiring a language redesign whenever practical.
6. Be tested at relevant boundaries.
7. Avoid becoming accidental, undocumented ceilings in shared semantic models or public interfaces.

A resource limit imposed by an explicit security policy or execution budget is permitted when it is intentional, visible, and enforced at the appropriate boundary.

5.3 Resource availability

Resource availability determines which valid realizations are feasible. It MUST NOT retroactively redefine the meaning of the source program.

The architecture MUST distinguish:

- What a program means.
- What resources it requires.
- What capabilities an environment offers.
- Which implementations satisfy the requirements.
- Which policies authorize execution.
- Which physical realization is selected.

If no permitted realization exists, the implementation MUST provide a meaningful diagnostic or execution outcome rather than silently weakening requirements.

6. Semantic independence and target realization

Zamani MUST separate program meaning from implementation choices.

The conceptual pipeline is:

Zamani source
      |
      v
Lexical analysis and parsing
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
      +--> Types
      +--> Effects
      +--> Capabilities
      +--> Resources
      +--> Contracts
      +--> Policies
      +--> Provenance
      |
      v
Canonical semantic representation
      |
      v
Target-independent optimization
      |
      v
Capability and resource negotiation
      |
      v
Domain-specific lowering
      |
      v
Routing, placement, and scheduling
      |
      v
Resilience and recovery where required
      |
      v
ZQN / HAL / target interfaces
      |
      v
Execution

This is an architectural contract. The existence of a stage in this diagram does not establish that the corresponding implementation is already complete.

The language MUST NOT collapse parsing, semantic validation, target discovery, resource allocation, scheduling, and physical execution into one responsibility.

6.1 Requirements

Requirements state what MUST be true for a permitted execution.

Examples of the intended resource and capability model include:

requires qubits >= n
requires memory >= required_memory
requires capability("quantum.measurement")
requires capability("gpu.compute")
requires capability("tensor.compute")
requires topology(required_topology)

These examples illustrate semantic intent; their exact source syntax MUST be governed by the canonical grammar and resource specification.

A requirement MUST be validated according to its defined meaning.

6.2 Constraints

Constraints restrict the valid realization space.

A constraint MUST be enforced when the relevant realization is selected or validated. It MUST NOT be mistaken for a universal implementation limit.

6.3 Preferences

Preferences describe desirable but non-mandatory choices.

A preference MAY influence selection, ranking, placement, or optimization. Failure to satisfy a preference MUST NOT invalidate a program unless a separate normative requirement establishes that outcome.

6.4 Hints

Hints MAY guide optimization. Ignoring a hint MUST NOT change the program's defined observable behavior.

6.5 Target descriptions and observations

Target descriptions and runtime observations belong to the relevant target, compiler, deployment, or runtime interfaces.

They MUST NOT be implicitly converted into universal source-language assumptions.

7. Syntax, AST, semantics, and IR have different responsibilities

Each representation MUST have a clearly defined purpose.

7.1 Syntax

The grammar defines which source forms are structurally valid.

Grammar rules MUST NOT attempt to implement resource negotiation, hardware discovery, authorization, optimization, physical routing, scheduling, or execution.

7.2 Abstract syntax tree

The AST records the structure of parsed source constructs and the source information needed by subsequent stages.

Every supported construct MUST have a documented AST mapping. Source spans and diagnostic context MUST be preserved to the extent required by the language's source-mapping contract.

The AST MUST NOT become a substitute for validated semantic information or a competing target-specific IR.

7.3 Semantic model

Semantic analysis establishes the meaning and validity of the AST, including names, types, effects, capabilities, resources, contracts, policies, and applicable domain rules.

It MUST report invalid or unresolved semantics explicitly.

7.4 Intermediate representations

Each canonical IR MUST have a defined owner, purpose, invariant set, versioning policy, and integration contract.

Domain-specific IRs MAY coexist when their responsibilities are distinct. They MUST NOT compete to be the canonical representation of the same semantics.

The repository's established quantum boundary is "quantum::ir", implemented under "src/quantum/ir/". Quantum source constructs MUST reach that boundary through the frontend and semantic validation path.

Classical constructs MUST use the established Classical IR architecture. HDL and hardware constructs MUST use their designated semantic and representation boundaries.

A grammar feature MUST NOT create a new canonical IR merely because its domain is new.

8. Classical, quantum, hybrid, and hardware principles

8.1 Classical computation

Classical computation is a foundational domain.

The language SHOULD support the required range of scalar and structured values, functions, generic programming, control flow, ownership, memory, concurrency, parallelism, numerical computing, symbolic computing, systems programming, and accelerator computation.

Classical semantics MUST remain compatible with the shared language foundations.

8.2 Quantum computation

Quantum computation is a first-class computational domain.

Quantum syntax MUST NOT assume a permanently fixed number of qubits, a finite universal gate catalog, a particular vendor, a fixed coupling topology, or a universal calibration or noise model.

Quantum operations MUST be extensible through well-defined operation identities, parameters, targets, controls, results, attributes, modifiers, and registered extensions where appropriate.

A backend MAY have a finite native operation set. That limitation MUST remain a target property, not a universal language restriction.

Logical qubits, physical qubits, measurement, reset, dynamic circuits, feed-forward, noise, and error correction MUST have explicitly defined semantic boundaries.

Physical qubit allocation, routing, pulse selection, calibration, scheduling, and device communication MUST remain downstream responsibilities.

8.3 Hybrid computation

Classical and quantum computation MUST be able to participate in a single program when the required semantics and implementation support exist.

Data exchange, control flow, measurement results, effects, synchronization, and resource boundaries MUST be explicit.

A domain crossing MUST NOT silently discard type information, effects, resource requirements, provenance, or policy obligations.

8.4 HDL and hardware/software co-design

Zamani MUST support hardware description and hardware/software co-design through explicit syntax and semantics.

Hardware constructs MAY describe modules, ports, signals, registers, clocks, timing, state machines, pipelines, memories, interfaces, parameters, and implementation intent.

The language MUST distinguish an abstract hardware design from its concrete physical realization.

A design MUST NOT require a particular FPGA family, fixed bus width, fixed processor count, or specific device merely because one implementation uses those properties.

Where a width, timing requirement, topology, or physical property is essential to correctness, the program MAY declare it explicitly.

9. Shared semantic foundations

All domains MUST integrate with the common semantic control model.

9.1 Types

Types describe valid values, operations, relationships, and constraints.

A type MUST NOT implicitly encode a target's physical layout or capacity unless the language's defined type semantics explicitly require such a property.

Generic, linear, affine, resource-aware, quantum, and hardware-related types MUST integrate with the shared type system rather than establish incompatible parallel type universes.

9.2 Effects

Effects describe semantically relevant consequences of computation, including external interaction, mutation, randomness, measurement, network communication, native calls, learning, adaptation, reflection, and code generation where supported.

Effects MUST be explicit enough for semantic checking and policy enforcement.

Domain-specific operations MUST reuse the canonical effect model.

9.3 Capabilities

Capabilities describe operations or properties an environment can provide.

Examples include quantum measurement, tensor computation, distributed communication, accelerator execution, and secure execution.

Capability identity and interpretation MUST be governed by the canonical capability contract.

Capability availability MUST be resolved against the applicable environment, not assumed during parsing.

9.4 Resources

Resources describe quantities or services required, available, consumed, or reserved.

Resource kinds and quantities MUST be extensible. The resource model MUST support abstract and symbolic requirements without imposing arbitrary universal maxima.

9.5 Contracts

Preconditions, postconditions, invariants, assumptions, guarantees, and properties MUST have defined validation and execution responsibilities.

A contract MUST NOT be considered satisfied merely because its syntax parsed successfully.

9.6 Policies and authorization

Policies govern which operations, adaptations, fallbacks, deployments, and resource uses are permitted.

Recognizing a source construct MUST NOT itself authorize its execution.

Operations that alter behavior or generate code MUST undergo the applicable semantic, capability, effect, resource, authorization, and provenance checks.

9.7 Provenance

The implementation MUST preserve the source and transformation information required for diagnostics, auditing, reproducibility, explainability, and compatibility.

Provenance MUST remain available across the applicable compilation and domain boundaries.

10. Determinism, correctness, and reproducibility

Zamani MUST define which behaviors are deterministic, nondeterministic, implementation-dependent, target-dependent, or unspecified.

These categories MUST NOT be conflated.

10.1 Semantic preservation

A compiler transformation MUST preserve the defined program meaning unless the language explicitly authorizes a transformation that changes a declared approximation, precision, or execution contract.

Target selection MUST NOT silently change correctness requirements.

10.2 Floating-point and numerical behavior

Numerical operations MUST follow the applicable type and numerical semantics.

Differences in precision, rounding, parallel reduction order, hardware behavior, or accelerator support MUST be handled according to explicit language and execution contracts.

A compiler MUST NOT claim exact equivalence where the specified numerical model permits observable differences.

10.3 Quantum and probabilistic behavior

Quantum measurement, probabilistic computation, stochastic algorithms, and nondeterministic execution MUST be distinguished from implementation defects.

Where exact reproducibility is requested, the implementation MUST define which environmental and execution conditions are necessary to provide it.

10.4 Reproducibility

Where reproducible builds or executions are required, the artifact and applicable contracts MUST identify the relevant language, semantic, IR, dialect, compiler, configuration, and provenance information.

Reproducibility MUST NOT be promised beyond the guarantees actually defined and enforced.

11. Safety and secure implementation

The Zamani implementation MUST use Rust 2021 and support Rust 1.97 or later, consistent with the repository's declared toolchain baseline.

Production Rust code MUST NOT use "unsafe".

This prohibition includes unsafe blocks, unsafe functions, unsafe traits, unsafe implementations, and other uses of Rust's unsafe facilities.

The implementation MUST use safe Rust abstractions for memory management, concurrency, data access, parsing, semantic analysis, IR construction, and resource handling.

If an external component has a safety contract, that contract MUST be documented and validated at the applicable boundary. It MUST NOT be used to justify introducing unsafe Rust into the prohibited implementation.

11.1 Memory and concurrency safety

The implementation SHOULD use Rust's ownership, borrowing, type system, and safe concurrency facilities to prevent memory unsafety and data races.

Language constructs for low-level systems programming do not grant the Rust implementation permission to violate this requirement.

11.2 Input validation

Source files, manifests, serialized artifacts, dialect registrations, external inputs, and target descriptions MUST be validated before they are trusted.

Malformed or adversarial inputs MUST produce controlled errors rather than unbounded resource consumption where enforceable protections are available.

11.3 Controlled adaptation and reflection

Adaptation, reflection, metaprogramming, and code generation MUST follow explicit authorization, capability, effect, policy, resource, provenance, and validation contracts.

They MUST NOT bypass type checking, semantic validation, sandboxing, or security policy.

12. Extensibility without a closed universe

Zamani MUST be extensible without requiring every future operation, device, algorithm, application, or computational domain to become a permanent core-language keyword.

12.1 Core language

Core syntax MUST contain only concepts whose universality justifies their inclusion in the language foundation.

Application-specific functionality SHOULD be provided through libraries, modules, dialects, registered operations, capabilities, and policies as appropriate.

12.2 Dialects and extensions

An extension MUST identify its namespace, version, dependencies, registration mechanism, syntax, semantic mapping, compatibility requirements, and applicable tests.

Extensions MUST NOT silently redefine established core syntax or semantics.

12.3 Quantum and hardware extensibility

A new quantum operation or hardware capability SHOULD be introduced through its declared operation or capability interface and semantic mapping.

It MUST NOT require redesigning the universal parser merely because the underlying hardware or operation is new.

12.4 External formats

External languages and formats MUST be handled through explicit interoperability or dialect boundaries.

Supporting an external format MUST NOT imply that the format has become part of Zamani's universal source grammar.

13. Compatibility and evolution

The language MUST evolve through explicit, versioned contracts.

Compatibility MUST cover the applicable dimensions of:

- Source syntax and lexical behavior.
- Language semantics and type rules.
- AST contracts.
- Semantic-model contracts.
- Classical and quantum IR.
- HDL and hardware representations.
- Dialects and extension interfaces.
- Compiler and runtime artifacts.
- Diagnostics and tooling.
- Resource and capability registries.

A change MUST be classified according to its actual compatibility impact.

Breaking changes MUST be versioned and accompanied by migration guidance where appropriate.

Deprecated syntax MUST have an explicit status and migration policy.

Experimental or planned features MUST NOT be represented as stable merely because they are described in the specification.

Historical documentation MUST NOT silently override the normative language specification.

14. Repository authority and file ownership

The repository MUST maintain a single, explicit authority model.

The intended responsibilities are:

File or directory| Responsibility
"grammar/DESIGN.md"| Overall grammar architecture, ownership, dependency rules, integration, and freeze governance.
"grammar/specification/"| Normative human-readable language and semantic specifications.
"grammar/spec/"| Machine-oriented contracts, schemas, registries, and formal requirements.
"grammar/Zamani.g4"| Canonical ANTLR composition boundary; no competing domain-specific authority.
"grammar/antlr/ZamaniLexer.g4"| Public ANTLR lexer boundary, composed from the canonical lexical system.
"grammar/antlr/ZamaniParser.g4"| Public ANTLR parser composition boundary.
"grammar/lexer/"| Canonical token and lexical definitions.
"grammar/grammar.md"| Reference for syntax and behavior implemented by the current frontend.
"grammar/Zamani-Grammar.md"| Historical and extended design reference, with explicit feature status.
"src/lexer.rs" and "src/parser.rs"| Existing Rust frontend implementation and its conformance obligations.
"src/ast/"| Existing AST implementation and source-structure contracts.
"src/semantic.rs" and related semantic modules| Semantic validation and analysis.
Classical IR modules| Canonical classical representation under their established ownership.
"src/quantum/ir/"| Canonical quantum semantic representation.
Compiler, execution, hardware, runtime, and backend modules| Target-independent transformations and target-specific realization according to their contracts.

These responsibilities MUST be reconciled with the repository's actual implementation during the inventory and integration stages. A proposed architectural boundary MUST NOT be described as already implemented unless verified.

No document or source file may silently establish a competing authority.

Where a contradiction is found, the conflict MUST be documented, resolved by the responsible maintainers, and reflected in the appropriate normative and implementation-conformance contracts.

15. Independent-file completion and integration

Every new or modified language file MUST be independently completable before integration.

Its contract MUST be defined before implementation is declared complete.

At minimum, the file's accompanying specification or ownership record MUST establish:

1. Purpose and normative status.
2. Exactly one owner for every exported symbol or concept.
3. Explicit non-ownership boundaries.
4. Inputs, outputs, imports, exports, and dependencies.
5. Dependency direction and cycle restrictions.
6. Lexer and token dependencies where applicable.
7. AST mapping and source-location requirements.
8. Semantic interpretation and validation responsibilities.
9. Type, effect, capability, resource, contract, policy, and provenance mappings where applicable.
10. Canonical IR destination where applicable.
11. Compiler, runtime, tooling, and downstream integration points.
12. Diagnostic behavior.
13. Compatibility and versioning obligations.
14. Positive, negative, boundary, and regression tests.
15. Portability and scalability tests where applicable.
16. A hard-coding audit.
17. Explicit completion and freeze criteria.

A file MUST NOT depend on unspecified future behavior or on private implementation details of another subsystem.

Stable interfaces MUST be agreed upon in advance.

If a downstream implementation later changes, the integration MUST adapt through the agreed interface wherever possible. A frozen specification or grammar file MUST NOT require semantic modification merely because another subsystem's implementation has changed.

A change that genuinely alters the language contract MUST undergo explicit review and versioning; it MUST NOT be disguised as routine integration work.

16. Production conformance and testing

Every normative principle MUST be supported by an appropriate conformance mechanism.

The complete test strategy SHOULD include the following layers:

1. Token and lexical conformance.
2. Parser and grammar-generation conformance.
3. AST and source-span conformance.
4. Name resolution and structural validation.
5. Type and semantic conformance.
6. Effects, capabilities, resources, contracts, and policy validation.
7. Domain-specific classical, quantum, hybrid, and HDL tests.
8. Canonical IR mapping and invariant tests.
9. Compiler transformation and target-lowering tests.
10. Runtime and execution-boundary tests.
11. Portability and compatibility tests.
12. Scalability and resource-boundary tests.
13. Diagnostics and negative tests.
14. Fuzzing and malformed-input tests where appropriate.
15. Determinism and reproducibility tests where required.
16. Cross-domain integration tests.

Tests MUST distinguish implemented behavior from planned behavior.

A specification test MUST NOT be marked as passing merely because its source example is documented.

16.1 Hard-coding audit

Automated validation MUST search for arbitrary resource ceilings and equivalent disguised restrictions in grammar files, specifications, semantic structures, registries, and integration code.

The audit MUST examine both obvious constants and constraints encoded through array sizes, enumerations, fixed device lists, finite operation catalogs, fixed-width assumptions, or special-case branches.

A fixed number MAY be legitimate when it is intrinsic to the language definition, a particular algorithm, a representation contract, or an explicitly configured implementation limit. The reason and ownership MUST be documented.

The audit MUST distinguish such legitimate uses from universal architectural restrictions.

16.2 Grammar validation

Grammar validation MUST check, as applicable:

- Resolved imports and dependencies.
- Unique ownership of exported rules and tokens.
- Duplicate or conflicting definitions.
- Unreachable rules and accidental left recursion.
- Ambiguities and precedence conflicts.
- Correct parser and lexer composition.
- AST and semantic mappings.
- Diagnostic and compatibility contracts.
- Conformance of implemented syntax with the normative specification.

16.3 Scalability validation

Scalability tests MUST establish that the same semantic program can be considered for different resource configurations without source rewriting solely because the available resources change.

Tests SHOULD cover small and large parameter values, symbolic resource requirements, insufficient resources, incompatible capabilities, and explicitly permitted fallbacks.

They MUST NOT claim that every backend or every physical target is supported when no such implementation exists.

17. Fallbacks and graceful failure

Fallback is permitted only when the program's semantics and policies authorize it.

A fallback MAY use another compatible backend, a simulator, a different decomposition, or another explicitly permitted realization.

A fallback MUST preserve the required program meaning and obligations.

An approximation MAY be used only when the relevant semantic contract and policy explicitly permit it.

If a requirement cannot be satisfied and no permitted fallback exists, the compiler or runtime MUST report the failure clearly.

The implementation MUST NOT silently convert a required capability into a preference, weaken a constraint, remove an effect, or substitute an approximation.

18. Required architectural invariants

The following invariants apply to every present and future Zamani language feature:

1. Zamani remains one coherent programming language.
2. Every language concept has an explicit authority and owner.
3. Syntax and semantics have distinct responsibilities.
4. The AST represents source structure rather than physical execution state.
5. Semantic analysis validates meaning before realization.
6. Canonical IR ownership remains explicit.
7. Quantum semantics use the established "quantum::ir" boundary.
8. Classical semantics use the established Classical IR boundary.
9. HDL and hardware semantics use their designated representation boundaries.
10. Requirements, capabilities, resources, constraints, preferences, and hints remain distinct.
11. Effects and policies are shared semantic contracts.
12. Physical device selection is not implicit.
13. Hardware discovery does not occur during parsing.
14. Routing, placement, and scheduling remain downstream realization responsibilities.
15. New operations and targets remain extensible without a closed universal catalog.
16. No arbitrary universal resource ceiling is introduced.
17. Real implementation limits are documented and diagnosed honestly.
18. Target variation does not silently redefine program meaning.
19. Fallbacks preserve semantics or are explicitly authorized to use a different declared contract.
20. Compatibility and language evolution are versioned.
21. Rust implementation code remains safe; "unsafe" is prohibited.
22. Every production feature has an integration contract and conformance evidence.
23. Planned, experimental, partial, and implemented features are clearly distinguished.
24. No feature is considered frozen without satisfying its completion criteria.
25. POCO-REAF remains an architectural objective enforced through semantic portability, stable representations, and explicit realization contracts.

19. Definition of production readiness

The language architecture is ready to be frozen only when the following conditions have been demonstrated.

Authority

- [ ] Normative specification ownership is unambiguous.
- [ ] The root grammar and parser composition boundaries are documented.
- [ ] Implemented behavior is distinguished from planned behavior.
- [ ] Conflicting or duplicate authorities have been resolved or explicitly classified.

Semantics and integration

- [ ] Every supported construct has an AST mapping.
- [ ] Every supported construct has a defined semantic interpretation.
- [ ] Shared types, effects, capabilities, resources, contracts, policies, and provenance are consistently integrated.
- [ ] Canonical IR ownership is explicit.
- [ ] Domain-specific representations do not compete for the same semantic authority.
- [ ] Compiler, runtime, and target integration contracts are documented.

Scalability and portability

- [ ] No arbitrary universal hardware or resource ceilings exist.
- [ ] Requirements and resource quantities can be expressed without assuming a fixed machine size.
- [ ] Target feasibility is separated from source-language validity.
- [ ] Insufficient resources and missing capabilities produce explicit outcomes.
- [ ] Fallback behavior is controlled and documented.
- [ ] Scalability and portability tests cover the implemented feature set.

Safety and validation

- [ ] Rust 2021 and Rust 1.97 or later are supported.
- [ ] The implementation complies with the prohibition on "unsafe" Rust.
- [ ] Grammar and import validation pass.
- [ ] Ownership and dependency audits pass.
- [ ] Positive, negative, boundary, and regression tests pass.
- [ ] Compatibility and diagnostic contracts are established.
- [ ] The documented implementation status matches verified behavior.

A checkbox MUST NOT be marked complete without evidence from the relevant specification, source implementation, automated validation, or test results.

20. Final principle

Zamani MUST describe what a computation means, what it requires, which capabilities it needs, which constraints govern its realization, and which effects and policies apply.

The compiler and runtime architecture MUST determine how that computation can be realized in a compatible environment.

The intended relationship is:

ONE ZAMANI PROGRAM
        |
        v
DEFINED PROGRAM MEANING
        |
        v
SHARED SEMANTIC CONTRACTS
        |
        v
CANONICAL REPRESENTATIONS
        |
        v
CAPABILITY AND RESOURCE NEGOTIATION
        |
        v
TARGET-SPECIFIC REALIZATION
        |
        v
EXECUTION WITH DEFINED GUARANTEES

The same source may be realized on a small embedded system, a CPU, a multicore system, a GPU, an FPGA, an ASIC, a quantum processor, a simulator, a heterogeneous machine, a cluster, or a future computational platform when the applicable requirements can be satisfied.

The language MUST NOT impose artificial architectural ceilings merely because current implementations or hardware are limited.

At the same time, it MUST report genuine limitations accurately and preserve the distinction between semantic validity and execution feasibility.

The fundamental Zamani principle is therefore:

«A Zamani program defines computational meaning; available resources and compatible implementations determine how that meaning can be realized.»

This principle governs language design, grammar composition, AST construction, semantic analysis, canonical IRs, compilation, execution, compatibility, and future extensions.

It is the foundation for POCO-REAF and for scaling Zamani from the smallest supported computation to arbitrarily large computational systems without requiring a redesign of the language whenever the underlying technology evolves.