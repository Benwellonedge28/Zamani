Zamani Universal Meta-Compiler (UMC)

Production Architecture and Repository Integration Specification

Canonical path: "src/compiler/documentation/ARCHITECTURE.md"

Repository: "Benwellonedge28/Zamani" (https://github.com/Benwellonedge28/Zamani)

Branch inspected: "main"

Inspection date: 7 November 2026

Language: Zamani

Rust edition: 2021

Minimum supported Rust version: Rust 1.97

Memory safety: Safe Rust exclusively. "unsafe" is prohibited.

Primary architectural objective:

"Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)"

Scalability objective: Support the smallest meaningful computation and arbitrarily large finite computations, subject to program semantics, available resources, implementation capabilities, representation limits, security policies and physical reality.

Document classification: Normative compiler architecture and repository integration specification.

Implementation status: Architectural requirements. This document does not certify that every existing subsystem or end-to-end compilation path is production-ready.

---

1. Purpose

This document establishes the authoritative architecture for Zamani's Universal Meta-Compiler.

The compiler must enable developers to write a Zamani program once, preserve its specified meaning, compile it into appropriately portable representations and realize it across compatible computing environments without requiring developers to rewrite their applications merely because the available hardware changes.

The compiler must coordinate the entire Zamani computational ecosystem.

This includes:

1. Classical computing.
2. Systems programming.
3. Embedded computing.
4. Parallel computing.
5. Concurrent computing.
6. Distributed computing.
7. High-performance computing.
8. Scientific and numerical computing.
9. Quantum computing.
10. Quantum simulation.
11. Quantum-classical hybrid computing.
12. Quantum error-correction integration.
13. Hardware description languages.
14. Hardware/software co-design.
15. FPGA and ASIC realization.
16. GPU and accelerator computing.
17. AI and machine learning.
18. Deep learning and neural networks.
19. Tensor computation.
20. Symbolic and probabilistic computation.
21. Bayesian and causal computation.
22. Autonomous agents and multi-agent systems.
23. Neuro-symbolic computation.
24. Neuromorphic computing.
25. Photonic computing.
26. Networking and communication.
27. Cryptographic and secure computation.
28. Data-intensive computation.
29. Future computational architectures.

These are computational domains within one programming language and one coordinated compiler architecture.

The compiler MUST expand the existing repository's capabilities rather than introduce competing implementations of existing subsystems.

1.1 Fundamental POCO-REAF objective

The intended architecture is:

                 ONE ZAMANI PROGRAM
                         |
                         v
                 CANONICAL FRONTEND
                         |
                         v
                   CANONICAL AST
                         |
                         v
                SEMANTIC VALIDATION
                         |
                         v
               VALIDATED PROGRAM MODEL
                         |
                         v
               CANONICAL IR ECOSYSTEM
                         |
                         v
               VERIFICATION AND ANALYSIS
                         |
                         v
              SEMANTICS-PRESERVING PASSES
                         |
                         v
                 OPTIMIZATION PLANNING
                         |
                         v
             CAPABILITY AND FEASIBILITY
                         |
                         v
               HETEROGENEOUS PLANNING
                         |
                         v
                DOMAIN-SPECIFIC LOWERING
                         |
                         v
                 BACKEND REALIZATION
                         |
                         v
                VALIDATED ARTIFACTS
                         |
                         v
                LINKING AND PACKAGING
                         |
                         v
                  RUNTIME HANDOFF
                         |
                         v
                  ACTUAL EXECUTION

Different programs may follow different valid routes through this architecture.

A classical program does not need quantum lowering.

A quantum program must preserve quantum semantics.

An HDL program must preserve hardware-description semantics.

A hybrid program must preserve cross-domain dependencies.

A distributed program must preserve communication and consistency semantics.

A program using several computational domains must preserve the relationships between them.

The compiler must not force every program through every computational domain.

1.2 Meaning of infinity

The architectural expression "scale to infinity" means that Zamani must not impose arbitrary universal capacity ceilings.

It does not promise infinite physical memory, storage, computation time, processors or hardware.

The compiler MUST:

- Avoid arbitrary language-wide resource ceilings.
- Support dynamically sized and resource-parametric computations.
- Support symbolic resource requirements where necessary.
- Use available resources according to explicit policies.
- Support configurable operational budgets.
- Scale its internal algorithms according to available capacity.
- Report resource exhaustion explicitly.
- Preserve semantics when changing compilation strategies.
- Avoid silent truncation.
- Avoid pretending that an unsupported computation succeeded.
- Support extensible targets and computing domains.

Actual limits must originate from real implementation constraints, target capabilities, available resources, representation formats or explicit policies.

---

2. Normative authority and precedence

The compiler must follow the existing repository's authority hierarchy.

The following sources have different responsibilities.

Authority| Responsibility
"grammar/DESIGN.md"| Overall grammar architecture
"grammar/OWNERSHIP.yaml"| Grammar ownership
"grammar/MANIFEST.yaml"| Grammar composition inventory
"grammar/Zamani.g4"| Root grammar composition
"grammar/antlr/ZamaniLexer.g4"| Public lexer boundary
"grammar/antlr/ZamaniParser.g4"| Public parser boundary
"grammar/lexer/"| Lexical vocabulary and components
"grammar/specification/"| Normative language specifications
"grammar/validation/"| Language and semantic boundaries
"src/frontend/"| Canonical frontend implementation
"src/frontend/semantic/"| Source-level semantic authority
"src/zuir/"| Universal intermediate representation
"src/classical/"| Classical computation and classical IR
"src/quantum/ir/"| Canonical quantum IR
"src/hdl/hdlir/"| Hardware-domain IR
"src/ai/"| AI computation subsystem
"src/hybrid/"| Hybrid computation
"src/distributed/"| Distributed computation
"src/optimizer/"| Existing optimization infrastructure
"src/backend/"| Canonical backend implementation and registry
"src/runtime/"| Runtime execution and physical resource management
"src/toolchain/"| Toolchain and project integration
"src/compiler/"| Compiler orchestration and integration

The compiler must conform to the normative grammar and semantic specifications.

If a compiler implementation conflicts with the language specification, the implementation must be corrected.

If an architecture document conflicts with an authoritative specification, the architecture document must be corrected.

The compiler MUST NOT silently redefine language semantics to accommodate an incomplete backend.

2.1 Semantic authority

"src/frontend/semantic/" is the canonical source-level semantic authority.

It owns:

- Name resolution.
- Semantic types.
- Type relationships.
- Effects.
- Ownership and borrowing semantics.
- Capabilities.
- Requirements.
- Constraints.
- Preferences.
- Hints.
- Semantic resource descriptions.
- Contracts.
- Policies.
- Provenance.
- Domain-specific semantic validation.

The compiler consumes these results.

"src/compiler/analysis/" may coordinate, aggregate and derive compiler-specific analysis evidence.

It MUST NOT implement a competing semantic analyzer or independent semantic type system.

"src/semantic.rs" remains a legacy migration concern until its consumers have been reconciled with the native frontend semantic implementation.

2.2 ZUIR authority

"src/zuir/" owns the universal intermediate representation.

ZUIR is not the semantic authority.

The frontend semantic subsystem establishes program meaning.

ZUIR represents validated computation for subsequent compilation.

The compiler integrates ZUIR through:

"src/compiler/ir/"

It MUST NOT introduce another universal IR.

2.3 Domain IR authorities

The canonical ownership boundaries are:

Domain| Authority
Universal computation| "src/zuir/"
Classical computation| "src/classical/"
Classical ZCIR integration| "src/classical/" and its existing ZUIR integration
Quantum computation| "src/quantum/ir/"
Hardware description| "src/hdl/hdlir/"
AI computation| "src/ai/"
Tensor storage| Existing canonical tensor subsystem
Hybrid computation| "src/hybrid/"
Distributed computation| "src/distributed/"
Networking| Existing networking subsystem
Cryptographic computation| Existing cryptographic subsystem

The compiler may coordinate adapters between these representations.

It must not recreate their identities, types, operation definitions or verification authorities.

A conversion must be explicitly implemented and verified by the appropriate domain owner.

Similar-looking structures do not establish semantic compatibility.

2.4 Backend authority

"src/backend/" owns the canonical backend interfaces, registration and implementation.

"src/compiler/backends/" provides compiler-facing integration.

The compiler MUST NOT create a competing authoritative backend registry.

The compiler-facing registry must continue delegating to the canonical backend registry.

2.5 Runtime authority

"src/runtime/" owns actual execution and physical resource management.

The compiler may produce:

- Resource requirements.
- Feasibility reports.
- Abstract partitioning plans.
- Data movement plans.
- Scheduling constraints.
- Routing constraints.
- Target realization descriptions.
- Deployment artifacts.

The compiler does not own:

- Actual device allocation.
- Physical device reservations.
- Runtime resource leases.
- Physical scheduling.
- Device communication.
- Execution.
- Resource release.
- Runtime recovery.

A compilation plan does not guarantee that a physical resource will remain available.

The runtime must validate availability when execution is requested.

---

3. Grammar and frontend integration

The compiler MUST integrate the actual grammar architecture.

It must not create another parser, lexer, AST or language specification.

3.1 Canonical grammar composition

The current grammar architecture is:

grammar/
    DESIGN.md
    OWNERSHIP.yaml
    MANIFEST.yaml
    CONFORMANCE.md
    FREEZE.md
    Zamani.g4

    antlr/
        ZamaniLexer.g4
        ZamaniParser.g4

    lexer/
        tokens.g4
        lexer.g4
        lexical component grammars

    core/
    types/
    expressions/
    declarations/
    statements/
    functions/
    modules/

    classical/
    quantum/
    hdl/
    hardware/
    hybrid/
    distributed/
    ai/
    data/
    networking/
    security/
    resources/
    concurrency/
    execution/
    interoperability/
    compile/
    policies/
    validation/

    specification/
    validation/
    tests/

This represents the existing modular grammar organization, not a proposal to recreate it.

3.2 Root grammar responsibilities

"grammar/Zamani.g4" owns the root grammar composition boundary.

It must not duplicate the grammar of individual domains.

"grammar/antlr/ZamaniLexer.g4" owns the public ANTLR lexer boundary.

"grammar/lexer/tokens.g4" coordinates the lexical token vocabulary.

"grammar/antlr/ZamaniParser.g4" owns the public parser composition boundary.

The domain grammar files own their respective syntactic productions.

The compiler consumes the resulting frontend structures.

3.3 Normative grammar specifications

The compiler must conform to:

- "grammar/DESIGN.md"
- "grammar/specification/grammar-authority.md"
- "grammar/specification/semantic-model.md"
- "grammar/validation/semantic-boundaries.md"
- "grammar/specification/compilation-model.md"
- "grammar/specification/portability.md"
- "grammar/specification/extensibility.md"
- "grammar/specification/scalability-model.md"
- "grammar/specification/poco-reaf.md"

The applicable domain specifications under "grammar/spec/" and "grammar/specification/" must also be consulted when implementing domain-specific compilation.

These specifications have distinct ownership responsibilities.

The compiler MUST NOT reinterpret source syntax independently.

3.4 Frontend integration

The compiler consumes the canonical frontend APIs:

src/frontend/
    ast/
    lexer/
    parser/
    semantic/
    diagnostics/

The intended flow is:

Source
  |
  v
Frontend lexer
  |
  v
Frontend parser
  |
  v
Canonical AST
  |
  v
Structural validation
  |
  v
Semantic analysis
  |
  v
Validated semantic results
  |
  v
Compiler

The compiler must preserve source identity, source spans and diagnostics.

Incomplete semantic analysis must never be interpreted as successful semantic validation.

A budget-exhausted analysis cannot certify a program as valid.

---

4. Compiler ownership and boundaries

The compiler owns the coordination of compilation.

It does not own all the implementations that participate in compilation.

Its responsibilities are:

1. Compilation requests.
2. Compilation configuration association.
3. Compilation contexts.
4. Compilation lifecycle.
5. Pipeline composition.
6. Stage registration and orchestration.
7. Compiler-specific analysis integration.
8. Pass coordination.
9. Optimization coordination.
10. IR integration.
11. Target compatibility coordination.
12. Feasibility aggregation.
13. Abstract resource planning.
14. Lowering coordination.
15. Backend invocation.
16. Artifact validation coordination.
17. Linking coordination.
18. Compilation diagnostics.
19. Compilation provenance.
20. Reproducibility coordination.
21. Incremental compilation.
22. Security enforcement at compiler boundaries.
23. Testing and conformance.

The compiler MUST NOT own:

- A second frontend.
- A second AST.
- A second semantic type system.
- A second universal IR.
- A second quantum IR.
- A second HDLIR.
- A second tensor storage system.
- A second optimizer.
- A second backend registry.
- A second physical resource manager.
- A second runtime scheduler.
- Physical execution.

---

5. Current compiler directory

The current compiler directory already contains the following architecture.

src/compiler/
    mod.rs

    compiler_types.rs

    audit_engine.rs
    borrow_checker.rs
    fuzzing_harness.rs
    instruction_fusion.rs
    macro_engine.rs
    monomorphizer.rs
    safety_guard.rs
    sro.rs
    ssbe.rs

    api/
    analysis/
    artifacts/
    backends/
    configuration/
    context/
    diagnostics/
    documentation/
    incremental/
    ir/
    linking/
    lowering/
    optimization/
    parallel/
    passes/
    pipeline/
    planning/
    profiling/
    security/
    testing/

The architecture must extend these existing modules.

It must not create duplicate generic subsystems simply to make the directory appear more complete.

No additional generic "core/", "engine/", "universal_ir/" or competing backend subsystem should be created.

New directories are justified only when an actual missing responsibility cannot be accommodated cleanly by the existing architecture.

---

6. File-by-file architectural responsibilities

The following table establishes the responsibility and integration contract of every existing compiler component.

The descriptions define the required production responsibility. They do not certify the current implementation as complete.

6.1 Composition root

"src/compiler/mod.rs"

Owns:

- Compiler module composition.
- Public compiler exports.
- Architectural invariants.
- Compiler API version association.
- Compiler subsystem integration.

Integrates:

- All compiler child modules.
- Frontend.
- Semantic subsystem.
- ZUIR.
- Domain IRs.
- Backend.
- Runtime.
- Toolchain.

Must not implement:

- Parsing.
- Semantic analysis.
- IR construction algorithms.
- Backend code generation.
- Runtime execution.

The existing module already contains a substantial architectural contract.

Preserve its valid content.

Any changes must reflect the actual public module tree and must compile.

It must not claim that a declared pipeline stage has a completed implementation merely because its module exists.

6.2 Shared compiler types

"src/compiler/compiler_types.rs"

Owns transitional compiler-wide compatibility types.

Its existing responsibilities include legacy compiler targets, optimization-level compatibility and shared compiler configuration types.

It must:

- Preserve required public compatibility.
- Identify legacy types explicitly.
- Avoid becoming a second semantic type system.
- Avoid becoming the canonical target-capability registry.
- Avoid introducing universal hardware ceilings.
- Avoid duplicate diagnostic ownership.
- Provide documented conversion boundaries to newer canonical types.

Canonical integration:

- "configuration/target.rs"
- "configuration/config.rs"
- "frontend/semantic/"
- "backend/target"
- "backend/capabilities"
- "backend/compatibility"

The legacy "CompilationTarget" must not become the authority for all future targets.

New target identities must remain extensible.

6.3 API

Directory: "src/compiler/api/"

File| Responsibility and integration
"mod.rs"| Public compiler API composition and exports.
"compiler.rs"| Compiler service and driver contract. Must connect to the actual production driver.
"request.rs"| Validated compilation requests, source identity and configuration association.
"response.rs"| Structured compilation results, artifacts, diagnostics and provenance.
"session.rs"| Compilation session lifecycle, cancellation and concurrent invocation.
"compatibility.rs"| Public API version and compatibility guarantees.

The public service must not expose a fake success merely because an older compiler returned a string.

The production driver must connect to the canonical frontend and actual compilation pipeline.

6.4 Analysis

Directory: "src/compiler/analysis/"

File| Responsibility
"mod.rs"| Analysis subsystem composition.
"program.rs"| Compiler-facing structural program analysis.
"dependency.rs"| Data, control, effect, communication and cross-domain dependency analysis.
"effects.rs"| Consume authoritative semantic effects and derive compiler-specific evidence.
"ownership.rs"| Integrate canonical ownership and borrowing information.
"capabilities.rs"| Capability evidence integration.
"requirements.rs"| Requirement collection and normalization.
"feasibility.rs"| Aggregate feasibility evidence without replacing its canonical authorities.
"cost.rs"| Cost estimation and comparison.
"provenance.rs"| Analysis evidence and source-to-artifact relationships.

Mandatory distinction:

Analysis must not redefine source semantics.

The compiler may derive new implementation evidence, but it must retain the source semantic authority.

6.5 Artifacts

Directory: "src/compiler/artifacts/"

File| Responsibility
"mod.rs"| Artifact subsystem composition.
"artifact.rs"| Artifact identity, representation and lifecycle.
"manifest.rs"| Dependencies, target information and artifact relationships.
"metadata.rs"| Language, compiler, IR and backend metadata.
"portability.rs"| Explicit portability classification and evidence.
"serialization.rs"| Versioned artifact serialization and validation.
"validation.rs"| Artifact integrity, structure, trust and compatibility validation.

Generated text must not automatically be represented as an executable artifact.

Artifacts must distinguish:

- Generated output.
- Structurally validated output.
- Semantically verified output.
- Target-compatible output.
- Linked output.
- Executable or deployable output.

These are different states.

6.6 Backends

Directory: "src/compiler/backends/"

File| Responsibility
"mod.rs"| Compiler-facing backend composition.
"registry.rs"| Adapter to canonical "src/backend/registry.rs".
"capabilities.rs"| Compiler-facing capability evidence.
"discovery.rs"| Backend and provider discovery integration.
"compatibility.rs"| Compatibility evaluation integration.
"selection.rs"| Deterministic backend selection.
"invocation.rs"| Validated backend construction and invocation.

Important:

"backends/registry.rs" MUST NOT become a second backend registry.

Backend selection must not rely on vendor-specific hard-coded matching.

Registration does not prove capability.

Capability does not prove feasibility.

Feasibility does not prove physical availability.

Selection does not execute a program.

The actual backend implementations remain under "src/backend/".

6.7 Configuration

Directory: "src/compiler/configuration/"

File| Responsibility
"mod.rs"| Configuration composition.
"budget.rs"| Operational work budgets and accounting.
"config.rs"| Canonical compiler configuration.
"policy.rs"| Compilation, security and transformation policies.
"profile.rs"| Reusable compilation profiles and preferences.
"target.rs"| Extensible target descriptors and metadata.
"validation.rs"| Structural and cross-field configuration validation.

Configuration must distinguish:

- Semantic requirements.
- Capabilities.
- Preferences.
- Constraints.
- Hints.
- Operational budgets.
- Security policies.

Operational budgets must never become semantic hardware ceilings.

A target descriptor must not imply that a device exists or is available.

6.8 Context

Directory: "src/compiler/context/"

File| Responsibility
"mod.rs"| Context composition.
"compilation_context.rs"| Shared immutable compilation inputs and services.
"compilation_state.rs"| Compilation lifecycle and valid state transitions.
"dependency_context.rs"| Compilation dependency evidence.
"provenance_context.rs"| Source, toolchain and transformation provenance.

All context contracts must agree on:

- Ownership.
- Thread safety.
- Cancellation.
- Lifetimes.
- Error propagation.
- Configuration immutability.
- Provenance.
- Shared budget accounting.

No hidden global mutable compiler state should be introduced.

6.9 Diagnostics

Directory: "src/compiler/diagnostics/"

File| Responsibility
"mod.rs"| Diagnostics composition.
"aggregation.rs"| Collection, grouping and deterministic ordering.
"rendering.rs"| Human-readable and machine-readable rendering.
"reporting.rs"| Structured compiler diagnostic reports.

Integrate with the canonical frontend diagnostics and source spans.

Do not create an incompatible source-location system.

Distinguish:

- Source errors.
- Semantic errors.
- Unsupported operations.
- Infeasible targets.
- Backend errors.
- Infrastructure errors.
- Security failures.
- Cancellation.
- Resource exhaustion.
- Incomplete analysis.

6.10 Incremental compilation

Directory: "src/compiler/incremental/"

File| Responsibility
"mod.rs"| Incremental subsystem composition.
"cache.rs"| Validated content-addressed compilation cache.
"fingerprints.rs"| Deterministic input and dependency fingerprints.
"invalidation.rs"| Precise dependency-driven invalidation.
"persistence.rs"| Versioned persistence and corruption handling.

Cache identity must account for every input capable of changing compilation results.

This includes:

- Source contents.
- Imported dependencies.
- Language version.
- Grammar and semantic specification versions.
- Compiler implementation version.
- Compiler configuration.
- Optimization policies.
- Pass implementations.
- Relevant extension manifests.
- Backend versions.
- IR schema versions.
- Target descriptors.
- Toolchain identities.
- Relevant security policies.

A cache hit must never bypass required verification or security validation.

6.11 IR integration

Directory: "src/compiler/ir/"

File| Responsibility
"mod.rs"| Compiler-facing IR integration.
"input.rs"| Validated canonical IR inputs.
"adapters.rs"| Integration with existing authoritative adapters.
"compatibility.rs"| Representation and schema compatibility.
"translation.rs"| Verified translation coordination.
"lowering.rs"| Compiler-facing lowering coordination.
"verification.rs"| Canonical verification coordination.

This directory must not implement another IR.

Existing "adapters.rs" supports the existing ZCIR/ZUIR identity boundary.

It must not advertise unsupported quantum, HDL, AI or tensor conversions as completed.

For every future conversion, the responsible domain owner must establish:

1. Source representation.
2. Destination representation.
3. Version compatibility.
4. Identity mapping.
5. Type mapping.
6. Value mapping.
7. Control-flow preservation.
8. Effect preservation.
9. Ownership preservation.
10. Resource-requirement preservation.
11. Provenance preservation.
12. Unsupported constructs.
13. Verification requirements.
14. Semantic-preservation evidence.

An unsupported conversion must fail explicitly.

6.12 Linking

Directory: "src/compiler/linking/"

File| Responsibility
"mod.rs"| Linking integration.
"symbols.rs"| Symbol resolution and identity.
"composition.rs"| Compatible module and artifact composition.
"relocation.rs"| Relocation integration with target-specific authorities.
"validation.rs"| Symbol, ABI, dependency and artifact compatibility.

The compiler must preserve domain-specific information during linking.

It must not treat incompatible quantum, HDL and classical artifacts as interchangeable object files.

6.13 Lowering

Directory: "src/compiler/lowering/"

File| Responsibility
"mod.rs"| Lowering coordination.
"conversion.rs"| Validated representation conversions.
"decomposition.rs"| Decomposition of operations using authoritative domain implementations.
"legalization.rs"| Target legality and legalization coordination.
"outlining.rs"| Extraction of independently compilable regions.
"target_lowering.rs"| Target-specific lowering integration.

Domain-specific conversion and decomposition algorithms must remain with their domain owners.

Examples:

- Classical lowering: classical IR and backend authorities.
- Quantum lowering: quantum IR and quantum lowering authorities.
- HDL lowering: HDLIR and synthesis authorities.
- AI lowering: AI and tensor authorities.
- Distributed lowering: distributed computation authorities.

The compiler coordinates these systems.

6.14 Optimization

Directory: "src/compiler/optimization/"

File| Responsibility
"mod.rs"| Optimization integration.
"cost_model.rs"| Extensible domain-aware cost models.
"decisions.rs"| Recorded decisions and supporting evidence.
"planning.rs"| Optimization planning.
"policies.rs"| Transformation permissions and semantic-preservation policies.
"profitability.rs"| Profitability assessment.
"validation.rs"| Transformation verification coordination.

Existing optimization implementations under "src/optimizer/", "src/zuir/" and domain-owned subsystems must be reused.

Do not create another optimizer.

A classical transformation must not automatically be considered valid for quantum or HDL computations.

Every transformation must establish its applicable semantic-preservation contract.

6.15 Parallel compilation

Directory: "src/compiler/parallel/"

File| Responsibility
"mod.rs"| Parallel compilation composition.
"build.rs"| Parallel build orchestration.
"dependencies.rs"| Dependency-aware task relationships.
"determinism.rs"| Deterministic result collection and ordering.
"scheduler.rs"| Compiler-work scheduling.
"workers.rs"| Worker execution and lifecycle.

This subsystem schedules compiler work.

It does not replace the runtime scheduler.

It must use available parallelism, configured budgets and operating-system capabilities.

No arbitrary fixed worker ceiling may be introduced.

Parallel execution must not silently change program semantics.

6.16 Pass management

Directory: "src/compiler/passes/"

File| Responsibility
"mod.rs"| Pass subsystem composition.
"analysis_manager.rs"| Analysis caching and invalidation.
"legality.rs"| Transformation legality.
"manager.rs"| Pass lifecycle and coordination.
"ordering.rs"| Pass dependency ordering.
"pipeline.rs"| Pass pipeline composition.
"preservation.rs"| Analysis preservation contracts.
"registry.rs"| Extensible pass registration.

Pass registration must be extensible.

A domain must not be required to modify a universal closed list of operations or transformations.

The pass manager must not replace the authoritative pass implementations under ZUIR or other domains.

6.17 Pipeline

Directory: "src/compiler/pipeline/"

File| Responsibility
"mod.rs"| Pipeline composition.
"stage.rs"| Stage interface and input/output contracts.
"stages.rs"| Standard stage identities and dependencies.
"ordering.rs"| Dependency validation and ordering.
"execution.rs"| Stage execution and output delivery.
"orchestration.rs"| Compilation lifecycle orchestration.
"failure.rs"| Structured failure classification.
"recovery.rs"| Explicit recovery policy.

The pipeline must support conditional domain-specific routes.

A skipped stage must never be represented as successfully completed.

A required stage must never be silently bypassed.

Stage declarations must be matched to actual implementations.

The production pipeline must establish a demonstrable connection between:

- Canonical frontend.
- Semantic validation.
- Canonical IR.
- Verification.
- Analysis.
- Pass management.
- Optimization.
- Planning.
- Lowering.
- Backend invocation.
- Artifact validation.
- Linking.
- Runtime handoff.

6.18 Planning

Directory: "src/compiler/planning/"

File| Responsibility
"mod.rs"| Planning subsystem composition.
"target_discovery.rs"| Target descriptor discovery.
"capability_matching.rs"| Requirement-to-capability matching.
"feasibility.rs"| Evidence-based feasibility assessment.
"resource_planning.rs"| Abstract resource demand and capacity assessment.
"partitioning.rs"| Computation partitioning.
"data_movement.rs"| Abstract data transfer planning.
"routing.rs"| Abstract routing constraints and plans.
"scheduling.rs"| Abstract scheduling plans.

The resource planner must distinguish resource demand from physical allocation.

The planning subsystem must not:

- Reserve hardware.
- Allocate physical qubits.
- Acquire devices.
- Perform physical scheduling.
- Claim guaranteed availability.
- Replace quantum routing.
- Replace distributed execution.
- Replace runtime resource management.

A feasibility result is evidence about a proposed realization, not an execution guarantee.

6.19 Profiling

Directory: "src/compiler/profiling/"

File| Responsibility
"mod.rs"| Profiling composition.
"events.rs"| Structured profiling events.
"measurements.rs"| Measurement contracts and provenance.
"aggregation.rs"| Measurement aggregation.

Profiling evidence may inform optimization.

It must not silently make reproducible builds depend on uncontrolled environmental measurements.

6.20 Security

Directory: "src/compiler/security/"

File| Responsibility
"mod.rs"| Security subsystem composition.
"policy.rs"| Compiler security policy integration.
"verification.rs"| Trust and verification evidence.
"audit.rs"| Auditable compiler operations.
"supply_chain.rs"| Dependency, provider and artifact supply-chain verification.

Security requirements include:

- Malformed source handling.
- Untrusted IR validation.
- Extension validation.
- Backend process isolation.
- Artifact integrity.
- Dependency integrity.
- Cryptographic provenance.
- Secure deserialization.
- Resource exhaustion protection.
- Reproducible build verification.
- Explicit authorization boundaries.

Security must not be disabled merely because an optimization or compilation budget has been exhausted.

6.21 Testing

Directory: "src/compiler/testing/"

The existing testing subsystem must progressively cover:

- Unit tests.
- Integration tests.
- Pipeline tests.
- IR conformance.
- Backend conformance.
- Cross-domain conformance.
- Semantic preservation.
- Regression tests.
- Determinism.
- Differential testing.
- Incremental compilation.
- Scalability.
- Security.
- Portability.
- Reproducibility.

Tests must exercise actual behavior, not just module imports or documentation claims.

6.22 Standalone compiler files

File| Responsibility
"audit_engine.rs"| Compiler audit coordination and evidence.
"borrow_checker.rs"| Integration with canonical ownership and borrowing semantics.
"fuzzing_harness.rs"| Fuzzing source input, IR, transformations, artifacts and extensions.
"instruction_fusion.rs"| Semantics-preserving instruction fusion through applicable domain contracts.
"macro_engine.rs"| Macro expansion, hygiene and canonical AST integration.
"monomorphizer.rs"| Generic specialization and polymorphic lowering.
"safety_guard.rs"| Compiler safety policy enforcement.
"sro.rs"| Existing specialized optimization responsibility; audit for overlap with canonical optimization owners.
"ssbe.rs"| Existing specialized backend-emission integration; audit for overlap with canonical backend implementations.

No standalone file should become a second owner of an existing repository subsystem.

---

7. Compilation pipeline

The compiler must establish one coordinated, verifiable compilation pipeline.

The pipeline is conditional and domain-aware.

The following is the canonical conceptual sequence.

Order| Responsibility| Authority
1| Source acquisition| Compiler API and frontend
2| Lexing| Frontend lexer
3| Parsing| Frontend parser
4| AST construction| Frontend AST
5| Structural validation| Frontend
6| Name and module resolution| Frontend semantic
7| Type validation| Frontend semantic
8| Effect validation| Frontend semantic
9| Ownership validation| Frontend semantic
10| Capability and requirement validation| Frontend semantic
11| Validated program model| Frontend semantic
12| Canonical IR construction| ZUIR and domain owners
13| IR verification| Canonical IR authorities
14| Compiler dependency analysis| Compiler analysis
15| Transformation planning| Pass management
16| Semantics-preserving optimization| Existing optimization authorities
17| Target discovery| Planning and providers
18| Capability matching| Planning
19| Feasibility assessment| Planning
20| Resource planning| Planning
21| Computation partitioning| Domain planning authorities
22| Data movement planning| Relevant domain authorities
23| Routing and scheduling planning| Relevant domain authorities
24| Domain-specific lowering| Domain owners and compiler adapters
25| Target legalization| Lowering and backend contracts
26| Backend selection| Canonical backend integration
27| Backend realization| "src/backend/"
28| Artifact validation| Compiler artifacts and security
29| Linking| Canonical linking integration
30| Packaging| Toolchain and artifact authorities
31| Runtime handoff| Runtime
32| Physical execution| Runtime and actual providers

These stages are logical responsibilities, not an instruction to create one source file per stage.

Existing implementations should be reused.

7.1 Conditional compilation routes

A classical program may follow:

Frontend
  -> Semantic validation
  -> Classical representation
  -> Verification
  -> Classical optimization
  -> Classical lowering
  -> Backend
  -> Artifact

A quantum program may follow:

Frontend
  -> Semantic validation
  -> Quantum IR
  -> Verification
  -> Quantum analysis
  -> Quantum transformations
  -> Quantum planning
  -> Quantum lowering
  -> Compatible backend
  -> Artifact

A hybrid program may follow:

Frontend
  -> Semantic validation
  -> Canonical IR integration
  -> Domain dependency analysis
  -> Hybrid partitioning
  -> Cross-domain data movement planning
  -> Domain-specific lowering
  -> Backend realization
  -> Validated artifacts

An HDL program must follow the applicable hardware-description and synthesis authorities.

A distributed program must preserve distributed communication, consistency and failure semantics.

No program may silently skip a mandatory semantic or verification stage.

---

8. Capability, requirement and target model

The compiler must use an open-world architecture.

It must not assume that every future computing technology can be predicted and added to a closed enumeration.

The following concepts must remain distinct.

Concept| Meaning
Requirement| What the program needs.
Capability| What an implementation supports.
Preference| What the developer prefers.
Constraint| What is permitted or prohibited.
Hint| Optional implementation guidance.
Feasibility| Whether available evidence supports a proposed realization.
Availability| Whether a required resource is actually available.
Authorization| Whether its use is permitted.

8.1 Requirements

Requirements originate from canonical semantic information.

They are mandatory conditions for preserving the declared computation.

An unsatisfied requirement must result in an explicit incompatibility.

8.2 Capabilities

Capabilities must be discoverable and extensible.

They must not be inferred from a vendor name or architecture string.

An unknown capability is not evidence of support.

8.3 Preferences

Preferences guide selection and optimization.

An unmet preference must not automatically invalidate an otherwise correct compilation.

8.4 Constraints

Constraints restrict valid realizations.

The compiler must never silently violate them.

8.5 Hints

Hints may improve planning.

They must never redefine program meaning.

8.6 Feasibility

Feasibility must be based on explicit evidence.

Possible outcomes must distinguish:

- Feasible.
- Infeasible.
- Unknown.
- Incomplete.
- Unsupported.
- Blocked by policy.

An incomplete discovery operation must not be presented as complete proof of infeasibility or feasibility.

---

9. Scalability and resource independence

POCO-REAF requires the compiler to avoid arbitrary universal resource ceilings.

The following must not be encoded as universal architectural limits:

- Maximum CPU count.
- Maximum core count.
- Maximum worker count.
- Maximum GPU count.
- Maximum FPGA count.
- Maximum QPU count.
- Maximum qubits.
- Maximum memory.
- Maximum VRAM.
- Maximum register width.
- Maximum tensor rank.
- Maximum distributed nodes.
- Maximum network participants.
- Maximum functions.
- Maximum modules.
- Maximum operations.
- Maximum computational domains.
- Maximum backend providers.
- Maximum optimization passes.

9.1 Real target limitations

A concrete target may have finite limitations.

For example:

- An instruction encoding may have a defined operand width.
- A processor may have a finite address space.
- An FPGA may have finite logic resources.
- A QPU may expose a finite number of physical qubits.
- A file format may define a bounded field.
- An operating system may impose process or thread limits.

These are real implementation constraints.

They must be represented as target capabilities, representation limits, resource evidence or explicit operational policies.

They must not become universal language limitations.

9.2 Operational budgets

Budgets are permitted.

Examples:

- Maximum compilation time.
- Maximum compiler memory consumption.
- Maximum analysis work.
- Maximum parallel workers.
- Maximum artifact output.
- Maximum cache consumption.
- Maximum external-tool execution time.

These must be configurable.

An omitted budget must not be interpreted as an infinite-resource guarantee.

Budget exhaustion must produce an explicit incomplete, aborted or failed outcome.

Incomplete analysis must never certify correctness.

9.3 Data structures and algorithms

Scalability is not achieved merely by removing constants.

The compiler must also avoid algorithms that become impractical as program size grows.

Implementations should prefer:

- Incremental analysis.
- Dependency-driven invalidation.
- Streaming where appropriate.
- Lazy evaluation where semantically valid.
- Sparse representations.
- Hierarchical data structures.
- Chunked processing.
- Bounded working sets when explicitly configured.
- Parallelizable independent operations.
- Efficient graph algorithms.
- Content-addressed storage.
- Reusable analysis results.

Algorithms must document their time and space complexity where material.

No optimization may sacrifice correctness merely to process larger inputs.

9.4 Numeric representation

Resource quantities must not be prematurely restricted to machine-sized integers.

Symbolic and arbitrary-precision quantities should be retained where the semantic and resource contracts require them.

Where a concrete representation has finite precision, overflow or loss of precision must be reported explicitly.

A wide integer type is not mathematical infinity.

---

10. Heterogeneous computing

The compiler must support programs containing multiple computational domains.

It must preserve their dependencies and semantics.

10.1 Classical computing

Integrate:

- "src/classical/"
- Existing classical IR.
- Existing ZCIR integration.
- "src/backend/"
- "src/runtime/"

Do not recreate classical IR or classical runtime scheduling.

10.2 Quantum computing

Integrate:

- "src/quantum/ir/"
- Existing quantum analysis and optimization.
- Existing quantum routing.
- Existing scheduling.
- Existing QEC.
- Existing ZQN.
- Existing hardware abstractions.
- Existing quantum backends.

"quantum::ir" remains the canonical semantic quantum IR.

The compiler must not introduce its own physical qubit identities, gate semantics, coupling maps or quantum execution models.

10.3 HDL and hardware

Integrate:

- "src/hdl/"
- "src/hdl/hdlir/"
- Existing hardware abstractions.
- Existing synthesis and backend authorities.

The compiler must preserve the distinction between logical hardware descriptions and physical realization.

It must not create a second HDLIR or place vendor-specific pin and placement semantics into universal compiler types.

10.4 AI and tensor computing

Integrate:

- "src/ai/"
- Existing tensor storage and operations.
- "src/classical/" tensor infrastructure where applicable.
- ZUIR AI and tensor dialects.
- Existing differentiation and optimization authorities.
- Existing accelerator providers.

The compiler must not introduce a competing tensor storage implementation or semantic type system.

Symbolic and dynamic tensor dimensions must remain representable where supported.

10.5 Hybrid computing

Integrate:

- "src/hybrid/"
- ZUIR.
- Classical IR.
- Quantum IR.
- HDLIR where applicable.
- AI and tensor subsystems.
- Distributed computation.
- Existing runtime interfaces.

Hybrid compilation must preserve:

- Cross-domain dependencies.
- Data types.
- Effects.
- Control dependencies.
- Synchronization.
- Communication.
- Ownership.
- Resource requirements.
- Failure semantics.
- Provenance.

A hybrid program is not merely a sequence of independent classical and quantum binaries.

10.6 Distributed computing

Integrate:

- "src/distributed/"
- Existing networking.
- Existing communication contracts.
- Existing distributed runtime.
- Relevant domain IRs.

The compiler may plan partitioning and communication.

It must not claim that an abstract partitioning plan constitutes successful physical deployment.

---

11. Backend realization and artifact integrity

The canonical backend authority is "src/backend/".

The compiler-facing backend modules must coordinate it.

The backend migration must be completed without creating a permanent duplicate backend architecture.

The current "src/backend/mod.rs" documents a transitional relationship with the legacy backend implementation.

That migration must be resolved deliberately.

11.1 Backend correctness

Every backend must establish:

1. Accepted input representation.
2. Supported operations.
3. Target capabilities.
4. Compatibility requirements.
5. Lowering behavior.
6. Generated output format.
7. Error behavior.
8. Toolchain dependencies.
9. Artifact validation.
10. Reproducibility behavior.
11. Security boundaries.
12. Conformance tests.

A backend registration is not proof of production readiness.

11.2 Target dispatch

Target dispatch must be verified against the actual requested target.

The existing backend documentation identifies unresolved target-dispatch concerns.

The architecture must prohibit:

- Selecting an incompatible backend.
- Silently substituting another target.
- Returning successful output for an unimplemented target.
- Claiming executable output when only textual IR was produced.

11.3 Artifact lifecycle

The compiler must distinguish:

Requested
   |
   v
Generated
   |
   v
Structurally validated
   |
   v
Semantically verified
   |
   v
Target compatible
   |
   v
Linked
   |
   v
Packaged
   |
   v
Deployment eligible

Not every artifact follows every state.

Every state transition must have explicit validation evidence.

---

12. Portability and POCO-REAF guarantees

Portability must be represented explicitly.

The following levels are required.

Level| Guarantee
Source portability| The same source can be accepted by compatible implementations.
Semantic portability| The defined computation meaning is preserved.
IR portability| Compatible canonical representations can be transferred and validated.
Artifact portability| An artifact can be consumed by compatible environments.
Execution portability| A compatible environment can actually execute the computation.

These are different guarantees.

A program requiring quantum semantics cannot automatically execute on an ordinary CPU without a compatible simulator or an explicitly permitted alternative implementation.

An HDL artifact cannot automatically be treated as a CPU executable.

A program with hardware-specific timing requirements cannot be assumed to preserve those requirements on every target.

The compiler must report the exact portability guarantee established by its evidence.

12.1 Meaning of Compile Once

A single compilation request must be able to produce appropriately reusable and portable artifacts.

Target-specific artifacts may require separate lowering and backend realization.

The compiler must distinguish:

- Source compilation.
- Target-independent compilation.
- Target-specific lowering.
- Backend emission.
- Linking.
- Deployment.

The term Compile Once must not be used to hide necessary target-specific realization.

12.2 Future compatibility

Future targets must be accommodated through extensible descriptors, capabilities, providers, dialects and versioned interfaces.

Adding a new hardware architecture should not require redesigning the source language or replacing the compiler composition root.

---

13. Determinism and reproducibility

Equivalent compilation inputs must produce equivalent results wherever deterministic behavior is required by the contract.

Reproducibility inputs include:

- Source content.
- Dependencies.
- Language and grammar versions.
- Semantic specification version.
- Compiler version.
- Compiler configuration.
- Optimization policies.
- Pass implementations.
- Extension identities and versions.
- IR versions.
- Backend versions.
- Target descriptors.
- Toolchain identities.
- Relevant environment inputs.

The compiler must record the inputs that influence an artifact.

Nondeterministic optimization and profiling must be controlled through explicit policy.

A reproducibility failure must not be hidden by a successful compilation status.

---

14. Security and trust

The compiler is a processing boundary for potentially untrusted source, dependencies, IR, extensions and backend output.

It must:

- Validate inputs.
- Validate IR before transformations.
- Revalidate after transformations where required.
- Reject malformed serialized data.
- Authenticate trusted dependencies and providers.
- Preserve provenance.
- Avoid shell-command injection.
- Isolate untrusted external tools.
- Enforce configured operational budgets.
- Protect artifact integrity.
- Validate backend outputs.
- Prevent untrusted extensions from bypassing required verification.

Compiler extensions must not gain unrestricted authority merely because they are registered.

No unsafe Rust is permitted in the compiler hierarchy.

The crate must enforce this requirement through source-level linting and CI checks.

---

15. Error and failure model

Failures must be structured and distinguishable.

At minimum, compiler interfaces must preserve the distinction between:

- Invalid source.
- Lexical failure.
- Parsing failure.
- Semantic failure.
- Invalid IR.
- Unsupported representation conversion.
- Unsupported operation.
- Capability mismatch.
- Constraint violation.
- Target infeasibility.
- Incomplete discovery.
- Backend incompatibility.
- Backend execution failure.
- Artifact validation failure.
- Linking failure.
- Security violation.
- Budget exhaustion.
- Cancellation.
- Internal compiler failure.

A failure in one stage must not be converted into success by a downstream stage.

Recovery must be explicit and permitted by policy.

A fallback is valid only when it preserves the required semantics and all relevant constraints.

---

16. Public API and legacy migration

The compiler must expose one canonical production compilation entry point.

The existing legacy entry point in "src/lib.rs" must be treated as a migration concern.

The current library exposes:

"compile(source: &str) -> Result<ir_gen::IrModule, Vec<String>>"

This route uses the older lexer, parser, semantic analyzer and IR generator.

It must not remain an independent production compiler architecture.

Migration must be staged:

1. Identify every existing consumer.
2. Define the new canonical compiler service contract.
3. Establish compatibility adapters.
4. Integrate the canonical frontend.
5. Integrate canonical semantic analysis.
6. Integrate ZUIR and domain IRs.
7. Establish verified pipeline execution.
8. Migrate callers.
9. Deprecate obsolete interfaces.
10. Remove obsolete implementations only after their consumers have been migrated and tested.

Do not break public APIs prematurely.

Do not maintain two independent semantic authorities indefinitely.

16.1 Lint policy

The current "src/lib.rs" contains broad lint allowances, including:

- "unused_imports"
- "unused_variables"
- "unused_mut"
- "dead_code"
- "clippy::all"

These weaken production quality enforcement.

They must be progressively removed.

The compiler must enforce strict local lint policies and must not rely on broad crate-wide allowances to conceal incomplete implementations.

---

17. Versioning and compatibility

The following versions must remain distinct:

- Zamani language version.
- Grammar version.
- Semantic specification version.
- Compiler API version.
- Compiler implementation version.
- ZUIR version.
- Classical IR version.
- Quantum IR version.
- HDLIR version.
- Artifact format version.
- Backend API version.
- Provider version.
- External toolchain version.

A change to one version must not silently imply compatibility with every other version.

Every serialized artifact must identify the representation versions it depends on.

Incompatible versions must be rejected or handled through explicitly validated migration adapters.

---

18. Testing and production qualification

Production readiness requires executable evidence.

The compiler must be tested as an integrated system.

18.1 Required language fixtures

The canonical cross-domain conformance suite must progressively include:

minimal.zm
classical.zm
quantum.zm
hdl.zm
hybrid.zm
distributed.zm
ai.zm
tensor.zm
networking.zm
crypto.zm
embedded.zm
multi_target.zm

Each fixture must exercise actual parsing, semantic validation, canonical IR construction, verification and the applicable compilation route.

A fixture that merely imports a module is insufficient.

18.2 Required correctness tests

Test| Required evidence
Frontend integration| Canonical AST and source information preserved.
Semantic conformance| Normative semantic rules enforced.
IR verification| Invalid representations rejected.
IR translation| Explicit conversions preserve required semantics.
Optimization| Transformations preserve specified behavior.
Quantum conformance| Quantum semantics preserved.
HDL conformance| Hardware intent preserved.
Hybrid conformance| Cross-domain dependencies preserved.
Distributed conformance| Communication and consistency semantics preserved.
Backend conformance| Generated output valid for its declared target.
Linking| Symbol and ABI compatibility validated.
Artifact integrity| Invalid or corrupted artifacts rejected.
Incremental compilation| Correct invalidation demonstrated.
Determinism| Defined reproducibility guarantees satisfied.
Security| Untrusted inputs cannot bypass verification.
Scalability| No arbitrary universal architectural ceilings.
Cancellation| Cancellation cannot produce false success.
Recovery| Recovery obeys semantic and security policies.
Portability| Appropriate portability claims supported by evidence.

18.3 Scaling tests

Scaling tests must cover progressively larger workloads, subject to the test environment.

They must measure:

- Compilation time.
- Peak memory consumption.
- IR construction cost.
- Verification cost.
- Dependency analysis cost.
- Optimization cost.
- Lowering cost.
- Cache efficiency.
- Parallel execution efficiency.
- Artifact generation cost.

The objective is to establish scalability behavior, identify algorithmic bottlenecks and detect accidental fixed limits.

No finite benchmark establishes literal infinite scalability.

18.4 Rust quality gates

The required project quality gates are:

cargo fmt --all -- --check

cargo check --all-targets

cargo test --all-targets

cargo clippy --all-targets -- -D warnings

Additionally:

- Test the minimum supported Rust version.
- Test supported feature combinations.
- Verify no unsafe code exists.
- Verify all declared modules compile.
- Verify generated grammar integrations.
- Verify actual backend outputs.
- Verify cross-domain fixtures.
- Verify artifact serialization compatibility.

---

19. Dependency direction

The compiler architecture must avoid circular ownership.

The intended dependency direction is:

Grammar
   |
   v
Frontend
   |
   v
Semantic authority
   |
   v
Canonical IR authorities
   |
   v
Compiler analysis and orchestration
   |
   v
Passes and optimization coordination
   |
   v
Planning and lowering
   |
   v
Canonical backend
   |
   v
Artifacts and linking
   |
   v
Runtime and deployment

This is a conceptual authority and data-flow model.

Some integration interfaces may be implemented through stable shared contracts or adapters.

No low-level subsystem should depend on the compiler composition root merely to access its own sibling implementation.

The compiler must not introduce circular dependencies between:

- Frontend and compiler.
- Semantic analysis and compiler.
- ZUIR and compiler.
- Domain IRs and compiler.
- Backend and compiler.
- Runtime and compiler.

Where existing Rust module dependencies violate the intended direction, the implementation must be refactored through appropriately owned interfaces.

---

20. Compiler documentation integration

The existing documentation directory contains:

src/compiler/documentation/
    ARCHITECTURE.md
    INTEGRATION_CONTRACT.md
    PIPELINE_CONTRACT.md
    PORTABILITY_CONTRACT.md

These documents have distinct responsibilities.

Document| Authority
"ARCHITECTURE.md"| Overall compiler architecture and ownership
"INTEGRATION_CONTRACT.md"| Repository-wide component integration
"PIPELINE_CONTRACT.md"| Compilation stage contracts and execution
"PORTABILITY_CONTRACT.md"| POCO-REAF and portability guarantees

This document must not duplicate the complete detailed pipeline contract.

The documents must remain consistent.

When an interface changes, all affected contracts must be updated deliberately.

No document may claim production readiness without corresponding implementation and test evidence.

---

21. Implementation and integration governance

The compiler must be implemented contract-first.

The objective is to complete each file with its dependencies and integration details established in advance.

A file must not be declared frozen merely because its source code compiles independently.

Its consumers and dependencies must also be known.

21.1 Required file completion contract

Every compiler implementation file must document:

1. Purpose.
2. Ownership.
3. Non-responsibilities.
4. Canonical authorities.
5. Public API.
6. Public types.
7. Function and trait contracts.
8. Dependencies.
9. Dependency direction.
10. Data ownership.
11. Lifetimes.
12. Thread safety.
13. Error model.
14. Determinism.
15. Scalability.
16. Operational budgets.
17. Security.
18. Compatibility.
19. Exact integration points.
20. Tests.
21. Completion criteria.

21.2 Freeze requirements

A file may be frozen only when:

- Its responsibility is unambiguous.
- Its canonical dependencies are established.
- Its public API is complete.
- Its errors are defined.
- Its consumers are identified.
- Its integration contracts are agreed upon.
- Its tests pass.
- Its documentation agrees with the implementation.
- No duplicate subsystem has been introduced.

A later change may still be necessary if a genuine defect is discovered.

Such a change must be deliberate, reviewed and accompanied by updated compatibility and integration tests.

The objective is to eliminate avoidable downstream redesign, not to prohibit necessary corrections.

---

22. Recommended implementation order

The implementation order must follow actual dependencies rather than arbitrary file numbering.

The following sequence is the intended progression.

Phase 0: Repository and authority freeze

Audit and reconcile:

- "grammar/"
- "src/frontend/"
- "src/frontend/semantic/"
- "src/zuir/"
- "src/classical/"
- "src/quantum/ir/"
- "src/hdl/hdlir/"
- "src/ai/"
- "src/hybrid/"
- "src/distributed/"
- "src/backend/"
- "src/runtime/"
- "src/compiler/"

Confirm every ownership boundary.

Phase 1: Independent foundational contracts

Begin with:

src/compiler/configuration/
    budget.rs
    target.rs
    config.rs
    policy.rs
    profile.rs
    validation.rs

src/compiler/compiler_types.rs

src/compiler/diagnostics/
    reporting.rs
    rendering.rs
    aggregation.rs

The precise order must follow the actual type dependencies.

These files establish stable inputs for downstream compiler components.

Phase 2: Identity, provenance and evidence

Complete:

analysis/provenance.rs
context/provenance_context.rs
context/dependency_context.rs
analysis/requirements.rs
analysis/capabilities.rs
analysis/cost.rs

Phase 3: Context and analysis

Complete:

context/compilation_state.rs
context/compilation_context.rs
context/mod.rs

analysis/program.rs
analysis/dependency.rs
analysis/effects.rs
analysis/ownership.rs
analysis/feasibility.rs
analysis/mod.rs

Phase 4: IR integration

Complete:

ir/compatibility.rs
ir/input.rs
ir/adapters.rs
ir/verification.rs
ir/translation.rs
ir/lowering.rs
ir/mod.rs

Do not advertise unsupported conversions.

Phase 5: Backend integration

Complete:

backends/registry.rs
backends/capabilities.rs
backends/discovery.rs
backends/compatibility.rs
backends/selection.rs
backends/invocation.rs
backends/mod.rs

Resolve canonical backend migration and target-dispatch defects.

Phase 6: Planning

Complete:

planning/target_discovery.rs
planning/capability_matching.rs
planning/resource_planning.rs
planning/feasibility.rs
planning/data_movement.rs
planning/partitioning.rs
planning/routing.rs
planning/scheduling.rs
planning/mod.rs

Integrate domain-owned physical planning authorities.

Phase 7: Pipeline

Complete:

pipeline/stage.rs
pipeline/stages.rs
pipeline/ordering.rs
pipeline/failure.rs
pipeline/execution.rs
pipeline/recovery.rs
pipeline/orchestration.rs
pipeline/mod.rs

Establish actual stage implementations and end-to-end registration.

Phase 8: Passes, optimization and lowering

Complete the existing:

passes/
optimization/
lowering/

Integrate canonical domain implementations.

Phase 9: Parallel and incremental compilation

Complete:

parallel/
incremental/

Verify deterministic concurrency and dependency-driven cache invalidation.

Phase 10: Artifacts and linking

Complete:

artifacts/
linking/

Verify artifact integrity, compatibility, provenance and reproducibility.

Phase 11: Security

Complete the existing security integration and verify all trust boundaries.

Phase 12: Public compiler service

Complete:

api/

Connect the production driver to the canonical frontend and verified pipeline.

Migrate legacy entry points without breaking compatibility prematurely.

Phase 13: Cross-domain qualification

Demonstrate actual end-to-end compilation for the supported domains.

Only capabilities with passing conformance tests may be advertised as production-supported.

---

23. Existing defects and outstanding repository obligations

The following concerns must remain visible in the implementation backlog.

Area| Required correction
"src/lib.rs"| Migrate the legacy compile entry point to the canonical compiler service.
"src/lib.rs"| Remove broad lint allowances progressively.
"src/compiler/compiler_types.rs"| Reconcile legacy target and configuration types with extensible canonical descriptors.
"src/compiler/ir/adapters.rs"| Keep unsupported domain conversions explicitly unsupported until implemented.
"src/compiler/pipeline/stages.rs"| Prove actual stage implementation and registration.
"src/compiler/pipeline/execution.rs"| Verify real stage output contracts and failure propagation.
"src/compiler/pipeline/orchestration.rs"| Verify the complete production pipeline lifecycle.
"src/compiler/backends/registry.rs"| Preserve delegation to the canonical backend registry.
"src/compiler/backends/selection.rs"| Verify compatibility and feasibility evidence before selection.
"src/compiler/planning/"| Verify complete domain-specific integration.
"src/backend/"| Resolve legacy migration, target dispatch and output-validity concerns.
"src/ai/"| Verify that domain integration is implemented rather than merely declared.
"src/hybrid/"| Verify actual cross-domain compilation and composition.
"src/distributed/"| Verify actual communication and execution integration.
Grammar integration| Mechanically verify the complete grammar composition and generated parser compatibility.

These are outstanding verification and implementation obligations, not permission to create replacement subsystems.

The current repository must be audited recursively before any additional files are introduced.

---

24. Production acceptance criteria

The compiler may be declared production-ready only when all applicable conditions below have demonstrable evidence.

Architecture

- One canonical frontend.
- One semantic authority.
- One canonical ZUIR.
- Canonical domain IR ownership.
- One authoritative backend registry.
- No competing runtime or physical resource manager.
- Explicit dependency direction.
- Versioned public contracts.

Correctness

- Actual end-to-end compilation.
- Semantic preservation.
- Canonical IR verification.
- Valid transformations.
- Valid lowering.
- Correct target dispatch.
- Valid backend outputs.
- Artifact validation.
- Correct linking.
- Explicit failure behavior.

Portability

- Source portability.
- Semantic portability.
- IR portability.
- Artifact portability.
- Execution compatibility evidence.
- Future target extensibility.

Scalability

- No arbitrary universal hardware ceilings.
- Configurable operational budgets.
- Explicit resource exhaustion.
- Efficient algorithms.
- Incremental compilation.
- Deterministic parallel compilation.
- Large-workload testing.

Security

- No unsafe Rust.
- Strict input validation.
- Secure artifact handling.
- Extension trust boundaries.
- Dependency integrity.
- Backend isolation.
- Supply-chain verification.

Quality

- Minimum Rust version tested.
- Formatting checks pass.
- All targets compile.
- All tests pass.
- Clippy passes with warnings denied.
- Cross-domain conformance passes.
- Reproducibility guarantees are tested.
- Documentation matches actual implementations.

A compiler module existing in the repository is not evidence that the capability is production-ready.

A successful build is not evidence that every backend is correct.

A successful compilation is not evidence that a program can execute on every possible target.

---

25. Final architectural invariants

The following invariants are mandatory for all future compiler development.

Invariant 1 — One language

Zamani remains one programming language across computational domains.

Invariant 2 — One semantic authority

"src/frontend/semantic/" owns source-level semantic meaning.

Invariant 3 — One universal IR authority

"src/zuir/" owns the universal intermediate representation.

Invariant 4 — Domain ownership

Classical, quantum, HDL, AI, hybrid and distributed subsystems retain their respective authoritative representations and implementations.

Invariant 5 — No duplicate backend registry

"src/backend/registry.rs" remains authoritative.

Invariant 6 — No duplicate runtime

Physical execution and resource acquisition belong to runtime and relevant providers.

Invariant 7 — No arbitrary universal hardware ceilings

Actual constraints must be discovered, represented or explicitly configured.

Invariant 8 — Explicit feasibility

Unknown evidence must never become proof of compatibility.

Invariant 9 — Semantic preservation

Optimization, transformation, lowering and backend realization must preserve all required program semantics.

Invariant 10 — Extensibility

New computational domains, providers and backends must integrate through versioned contracts rather than closed universal enumerations.

Invariant 11 — Reproducibility

Compilation inputs, transformations and artifact provenance must be recorded sufficiently to establish the applicable reproducibility guarantees.

Invariant 12 — Safe Rust

Unsafe Rust is prohibited.

Invariant 13 — Explicit failure

Unsupported, incomplete, infeasible and unsuccessful operations must never be silently converted into success.

Invariant 14 — Evidence-based production readiness

No subsystem may claim production support without implementation, integration and conformance evidence.

---

26. Final objective

Zamani's compiler must make the following programming model possible:

                 DEVELOPER
                     |
                     v
             ONE ZAMANI PROGRAM
                     |
                     v
            ONE SEMANTIC MEANING
                     |
                     v
            CANONICAL REPRESENTATIONS
                     |
                     v
          PORTABLE COMPILATION CONTRACT
                     |
                     v
          CAPABILITY-DRIVEN REALIZATION
                     |
          +----------+-----------+
          |          |           |
          v          v           v
        CPU/GPU     QPU       FPGA/ASIC
          |          |           |
          +----------+-----------+
                     |
                     v
            HYBRID / DISTRIBUTED
                     |
                     v
              FUTURE HARDWARE

The developer should not have to rewrite the program merely because the available computational infrastructure changes.

The implementation must select valid realizations based on semantic requirements, available capabilities, constraints, preferences, policies and actual resource availability.

When a compatible realization exists, the compiler and runtime should make it possible.

When no compatible realization exists, they must report the reason explicitly.

The ultimate Zamani objective is:

"Program Once → Compile Once → Run Everywhere → Anywhere → Forever."

Not by pretending that all machines are identical, but by preserving the program's meaning while allowing its implementation to evolve across compatible computing technologies and generations of hardware.

The compiler is the coordinating mechanism that makes this possible.