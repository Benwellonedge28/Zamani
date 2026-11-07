Zamani Universal Meta-Compiler

Compilation Pipeline Contract

Canonical path: "src/compiler/documentation/PIPELINE_CONTRACT.md"

Repository: "github.com/Benwellonedge28/Zamani"

Architecture: Universal Meta-Compiler (UMC)

Primary objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

Rust edition: 2021

Minimum Rust version: 1.97 or later

Safety: Safe Rust only. "unsafe" is prohibited.

Status: Normative compiler architecture, pipeline implementation and integration contract.

---

1. Purpose

This document defines the authoritative production compilation pipeline for Zamani.

It establishes:

- The complete compilation lifecycle.
- The responsibilities of every compilation stage.
- Stage dependencies and execution requirements.
- Stage activation and conditional compilation.
- Input and output contracts.
- Semantic-preservation requirements.
- Intermediate-representation verification.
- Domain-specific compilation.
- Capability and resource feasibility.
- Heterogeneous compilation.
- Lowering and backend integration.
- Artifact generation and validation.
- Incremental and reproducible compilation.
- Failure handling and recovery.
- Cancellation and resource accounting.
- Security and provenance.
- Deterministic execution.
- Scalability requirements.
- Integration with existing repository modules.
- Testing and production-readiness criteria.

The compiler must coordinate the complete Zamani programming language.

Its architecture must accommodate:

1. Classical computing.
2. Systems programming.
3. Parallel computing.
4. Concurrent computing.
5. Distributed computing.
6. Scientific and numerical computing.
7. Quantum computing.
8. Quantum-classical computing.
9. Quantum simulation.
10. Quantum error correction integration.
11. AI and machine learning.
12. Deep learning.
13. Neural networks.
14. Transformer architectures.
15. Tensor computation.
16. Symbolic computation.
17. Probabilistic and Bayesian computation.
18. Causal computation.
19. Neuro-symbolic computation.
20. Reinforcement learning.
21. Federated computation.
22. Hardware description languages.
23. Hardware/software co-design.
24. FPGA and ASIC realization.
25. GPU and accelerator computing.
26. Neuromorphic computing.
27. Photonic computing.
28. Embedded and edge computing.
29. Networking.
30. Cryptographic computation.
31. Future computational domains.

These domains must remain extensible.

Adding a new computational domain must not require redesigning the compiler's fundamental architecture.

The central requirement is:

One Zamani program must retain its defined semantic meaning when compiled for different compatible computational environments.

The compiler must support portable program representations and multiple target realizations without requiring developers to rewrite their application whenever the available hardware changes.

However, POCO-REAF does not promise that every program can execute on every physical machine.

When a target cannot satisfy a program's requirements, the compiler must report the incompatibility explicitly.

It must never silently change the program's meaning to produce a successful compilation.

---

2. Normative language

The following terms have their conventional normative meanings:

- MUST: Mandatory.
- MUST NOT: Prohibited.
- REQUIRED: Mandatory.
- SHOULD: Recommended unless a documented reason justifies otherwise.
- SHOULD NOT: Discouraged unless a documented reason justifies otherwise.
- MAY: Optional.

All requirements expressed using MUST or MUST NOT are mandatory for a conforming implementation.

This contract governs compiler architecture and implementation.

It does not supersede the authoritative language semantic specification.

Where this document conflicts with the canonical semantic model, the semantic model takes precedence regarding program meaning.

---

3. Canonical repository architecture

3.1 Compiler ownership

The compiler owns:

- Compilation orchestration.
- Compiler subsystem composition.
- Compilation-stage contracts.
- Compiler-facing integration.
- Stage dependency coordination.
- Compilation workflow execution.
- Compiler analysis coordination.
- Transformation coordination.
- Capability and feasibility coordination.
- Resource planning coordination.
- Lowering coordination.
- Backend invocation coordination.
- Artifact validation coordination.
- Compilation provenance.
- Compilation reporting.
- Compilation lifecycle management.

The compiler does NOT own:

- Source grammar.
- Lexical rules.
- Parser implementation.
- AST definitions.
- Source-level semantic authority.
- Domain-specific semantic definitions.
- Canonical universal IR definitions.
- Classical IR definitions.
- Quantum IR definitions.
- HDLIR definitions.
- AI and tensor semantic definitions.
- Physical hardware discovery.
- Physical hardware allocation.
- Runtime scheduling.
- Runtime execution.
- Backend implementation.
- Physical quantum routing.
- Quantum error correction implementation.
- Hardware calibration.
- HAL implementation.

Those responsibilities remain with their existing authoritative subsystems.

3.2 Canonical repository authorities

Responsibility| Canonical authority
Language design| "grammar/DESIGN.md"
Complete grammar| "grammar/Zamani.g4"
Parser composition| "grammar/antlr/ZamaniParser.g4"
Lexer composition| "grammar/antlr/ZamaniLexer.g4"
Lexical vocabulary| "grammar/lexer/"
Compilation syntax| "grammar/compile/"
Program syntax| "grammar/core/"
Semantic model| "grammar/specification/semantic-model.md"
Compilation model| "grammar/specification/compilation-model.md"
Semantic boundaries| "grammar/validation/semantic-boundaries.md"
Frontend| "src/frontend/"
Semantic implementation| "src/frontend/semantic/"
AST| "src/frontend/ast/"
Lexer implementation| "src/lexer.rs" and canonical frontend integration
Parser implementation| "src/frontend/parser/"
Universal IR| "src/zuir/"
Classical computation| "src/classical/"
Quantum computation| "src/quantum/"
HDL and hardware representation| "src/hdl/"
AI computation| "src/ai/"
Hybrid computation| "src/hybrid/"
Distributed computation| "src/distributed/"
Optimization infrastructure| "src/optimizer/" and "src/zuir/pass/"
Backend implementation| "src/backend/"
Runtime execution| "src/runtime/"
Compiler orchestration| "src/compiler/"
Toolchain integration| "src/toolchain/"

These boundaries are mandatory.

A compiler module must not create a competing implementation of a subsystem already owned elsewhere.

3.3 Important distinction: semantic model versus ZUIR

The semantic subsystem defines what a Zamani program means.

ZUIR represents validated computation.

The relationship is:

Source
  |
  v
Frontend AST
  |
  v
Canonical semantic model
  |
  v
Validated computation
  |
  v
ZUIR and domain-owned IRs
  |
  v
Compiler transformations and realization

ZUIR is not the semantic authority.

The compiler must preserve the semantic information established by the frontend when integrating and transforming canonical representations.

---

4. POCO-REAF architecture

The compiler must support the following conceptual architecture:

                ZAMANI SOURCE
                      |
                      v
               SOURCE ACQUISITION
                      |
                      v
                CANONICAL LEXER
                      |
                      v
               CANONICAL PARSER
                      |
                      v
                 CANONICAL AST
                      |
                      v
              STRUCTURAL VALIDATION
                      |
                      v
              NAME / MODULE RESOLUTION
                      |
                      v
              SEMANTIC VALIDATION
                      |
          +-----------+-----------+
          |           |           |
          v           v           v
        TYPES       EFFECTS     OWNERSHIP
          |           |           |
          +-----------+-----------+
                      |
                      v
             VALIDATED PROGRAM MODEL
                      |
                      v
             CANONICAL IR INTEGRATION
                      |
                      v
                IR VERIFICATION
                      |
                      v
               PROGRAM ANALYSIS
                      |
                      v
             TRANSFORMATION PLANNING
                      |
                      v
             SEMANTICS-PRESERVING
                  OPTIMIZATION
                      |
                      v
            DOMAIN AND HYBRID ANALYSIS
                      |
                      v
            CAPABILITY NEGOTIATION
                      |
                      v
              FEASIBILITY ANALYSIS
                      |
                      v
             HETEROGENEOUS PLANNING
                      |
                      v
             RESOURCE REQUIREMENT
                  ASSESSMENT
                      |
                      v
             DOMAIN-SPECIFIC LOWERING
                      |
                      v
                  LEGALIZATION
                      |
                      v
               BACKEND REALIZATION
                      |
                      v
               LINKING / COMPOSITION
                      |
                      v
               ARTIFACT VERIFICATION
                      |
                      v
              PORTABLE / TARGET-SPECIFIC
                    ARTIFACTS
                      |
                      v
                DEPLOYMENT HANDOFF
                      |
                      v
                 ZAMANI RUNTIME
                      |
                      v
                PHYSICAL EXECUTION

This represents the logical architecture.

The actual implementation must use the existing pipeline stage contracts and canonical repository modules.

It must not create a second pipeline implementation.

Not every program traverses every domain-specific stage.

A classical-only program does not require quantum lowering.

A quantum program must preserve quantum semantics.

An HDL program must preserve hardware-description semantics.

An AI program must integrate with canonical AI and tensor infrastructure.

A hybrid program must preserve cross-domain dependencies and synchronization.

A distributed program must preserve communication, ordering and consistency requirements.

Conditional execution is mandatory.

However, conditional execution must never become permission to omit a required stage.

---

5. Existing pipeline infrastructure

The existing "src/compiler/pipeline/" directory is the canonical pipeline implementation.

The following files must be preserved and expanded.

File| Canonical responsibility
"pipeline/mod.rs"| Pipeline composition and public exports
"pipeline/stage.rs"| Individual stage contract
"pipeline/stages.rs"| Standard stage identifiers and dependencies
"pipeline/ordering.rs"| Dependency validation and deterministic ordering
"pipeline/execution.rs"| Stage execution and output delivery
"pipeline/failure.rs"| Structured failure representation
"pipeline/recovery.rs"| Recovery decisions and policies
"pipeline/orchestration.rs"| Compilation lifecycle and orchestration

These files are not interchangeable.

Their responsibilities must remain separate.

5.1 Existing implementation qualification

The current architecture establishes several important foundations:

- Dynamic stage identifiers.
- Stage descriptors.
- Stage dependencies.
- Deterministic dependency ordering.
- Required-stage enforcement.
- Conditional stage outcomes.
- Stage input and output delivery.
- Structured failures.
- Recovery decisions.
- Compilation lifecycle coordination.

However, the presence of these foundations does not prove that the entire production compilation pipeline is connected.

The following implementation limitations must be addressed:

1. Standard stage identifiers do not prove that corresponding implementations exist.
2. Dependency ordering does not prove that stage registration is complete.
3. The current executor is sequential.
4. Parallel compilation requires a separate, verified integration.
5. Recovery decisions do not execute retries or fallback operations.
6. Operational budgets must be enforced at every applicable resource-consuming boundary.
7. Stage outputs must carry adequate validation and provenance evidence.
8. The public compiler driver must invoke the actual production pipeline.
9. Every required domain must have a demonstrable implementation path.
10. Successful orchestration must not be reported when required work remains incomplete.

These are production qualification requirements, not reasons to replace the existing architecture.

---

6. Pipeline stage contract

"src/compiler/pipeline/stage.rs" remains the canonical stage contract.

No second stage trait or execution framework may be introduced.

Every stage must have:

1. A stable stage identifier.
2. A validated descriptor.
3. Explicit dependencies.
4. An input contract.
5. An output contract.
6. A defined execution phase.
7. A concurrency declaration.
8. A determinism declaration.
9. A failure contract.
10. A cancellation contract.
11. A resource-budget contract.
12. A verification contract.
13. A provenance contract.
14. Applicable security requirements.

6.1 Stage identity

Stage identifiers must be dynamically extensible.

Standard identifiers belong in:

"src/compiler/pipeline/stages.rs"

Domain-specific extensions may introduce additional identifiers through the existing "StageId" contract.

A closed enumeration of every possible compiler stage is prohibited.

The compiler must not require modification of its root module whenever a new computational domain introduces a valid stage.

6.2 Stage inputs

Stages must consume explicitly declared inputs.

They must not rely on accidental execution order.

A stage must obtain the outputs of its declared dependencies through the existing stage-input contract.

An unexpected payload type must produce a structured integration error.

A type mismatch must never be interpreted as successful conversion.

6.3 Stage outputs

Every successful stage must return an output consistent with its declared contract.

The output must identify, where applicable:

- Representation identity.
- Representation version.
- Source relationship.
- Semantic-preservation evidence.
- Verification status.
- Dependency information.
- Provenance.
- Relevant resource-accounting information.
- Target compatibility information.

The compiler must not treat an arbitrary payload as a verified IR or a valid artifact.

6.4 Stage completion

A stage has completed successfully only when its required operation has completed and its output satisfies the declared contract.

The following states must remain distinct:

- Completed.
- Skipped.
- Blocked.
- Failed.
- Cancelled.
- Incomplete.

A skipped stage is not a completed stage.

A blocked stage is not a completed stage.

A failed stage is not a completed stage.

An incomplete required stage cannot contribute to a successful compilation result.

---

7. Canonical compilation stage groups

The following stage groups define the logical compilation responsibilities.

The actual stage identifiers must be maintained in the existing "stages.rs".

The actual implementations must be registered and connected through the existing pipeline architecture.

7.1 Source acquisition

Responsibilities:

- Accept source text or source-file input.
- Establish source identity.
- Preserve encoding information.
- Integrate with canonical source maps.
- Preserve source provenance.
- Report input errors.
- Validate applicable source-input policies.

Integration:

- "src/compiler/api/"
- "src/compiler/context/"
- "src/source_map/"
- "src/frontend/"

Source acquisition must not select physical hardware.

7.2 Lexical analysis

Responsibilities:

- Use canonical Zamani lexical rules.
- Preserve token positions.
- Preserve lexical identity.
- Report lexical errors.
- Maintain token compatibility.

Integration:

- "grammar/antlr/ZamaniLexer.g4"
- "grammar/lexer/lexer.g4"
- "grammar/lexer/tokens.g4"
- "src/lexer.rs"
- Canonical frontend lexer integration.

The compiler must not introduce another lexical vocabulary.

7.3 Parsing

Responsibilities:

- Use the canonical parser.
- Construct the canonical frontend AST.
- Preserve source spans.
- Support all registered language domains.
- Report syntax errors.
- Preserve declaration and expression relationships.

Integration:

- "grammar/Zamani.g4"
- "grammar/antlr/ZamaniParser.g4"
- "grammar/core/"
- "grammar/declarations/"
- "grammar/statements/"
- "grammar/compile/"
- "src/frontend/parser/"
- "src/frontend/ast/"

7.4 Structural validation

Responsibilities:

- Validate AST invariants.
- Validate declaration relationships.
- Detect inconsistent syntax structures.
- Preserve source locations.
- Reject malformed structures.

Integration:

- "src/frontend/ast/"
- "src/frontend/semantic/"
- "src/compiler/diagnostics/"

7.5 Name and module resolution

Responsibilities:

- Resolve identifiers.
- Resolve qualified names.
- Resolve imports.
- Resolve module dependencies.
- Detect missing declarations.
- Detect ambiguous references.
- Establish dependency relationships.

Integration:

- "src/frontend/semantic/"
- "src/compiler/analysis/"
- "src/compiler/context/dependency_context.rs"
- "src/stdlib/"
- "src/toolchain/"

The compiler must not independently redefine symbol semantics.

7.6 Semantic validation

Responsibilities:

- Type validation.
- Ownership validation.
- Borrowing validation.
- Effect validation.
- Capability requirement validation.
- Resource requirement validation.
- Constraint validation.
- Contract validation.
- Security validation.
- Domain-specific semantic validation.

Integration:

- "src/frontend/semantic/"
- "src/compiler/analysis/"
- "grammar/specification/semantic-model.md"
- "grammar/validation/semantic-boundaries.md"

The frontend semantic subsystem remains the semantic authority.

The legacy "src/semantic.rs" must be handled through an explicit compatibility migration.

It must not silently become a second production semantic authority.

7.7 Capability and requirement analysis

The compiler must preserve the distinction between:

Concept| Meaning
Requirement| What the program needs
Capability| What an implementation can support
Preference| What the developer would prefer
Constraint| What is permitted or prohibited
Hint| Information that may improve planning
Feasibility| Whether evidence supports a proposed realization

These concepts must not be collapsed into a single target-selection flag.

Integration:

- "src/frontend/semantic/"
- "src/compiler/analysis/requirements.rs"
- "src/compiler/analysis/capabilities.rs"
- "src/compiler/analysis/feasibility.rs"
- "src/compiler/planning/"

7.8 Canonical IR construction

The compiler coordinates construction and integration of canonical representations.

It does not define competing representations.

Integration:

- "src/compiler/ir/"
- "src/zuir/"
- "src/classical/"
- "src/quantum/ir/"
- "src/hdl/hdlir/"
- "src/ai/"
- "src/hybrid/"

Every adapter must declare its source representation, destination representation and preservation guarantees.

Unsupported conversions must fail explicitly.

7.9 IR verification

Every representation must be validated using its authoritative verifier.

Verification must distinguish:

- Valid.
- Invalid.
- Incomplete.
- Unsupported.
- Unverified.

These states must never be conflated.

Integration:

- "src/compiler/ir/verification.rs"
- "src/zuir/verify/"
- "src/ir_verify.rs"
- Domain-specific verification authorities.

Verification is required before accepting an IR as an input to a transformation that depends on its validity.

Verification is also required after transformations that can affect relevant invariants.

7.10 Program and dependency analysis

Responsibilities:

- Control-flow relationships.
- Data dependencies.
- Effect dependencies.
- Ownership dependencies.
- Domain dependencies.
- Communication dependencies.
- Module relationships.
- Cross-domain dependencies.

Integration:

- "src/compiler/analysis/program.rs"
- "src/compiler/analysis/dependency.rs"
- "src/compiler/analysis/effects.rs"
- "src/compiler/analysis/ownership.rs"
- "src/compiler/graph/" only if a distinct compiler workflow graph is required.

The compiler must not create a duplicate program IR.

7.11 Transformation planning

Transformations must be selected according to:

- Semantic validity.
- Representation legality.
- Required capabilities.
- Program constraints.
- Applicable optimization policies.
- Cost evidence.
- Security policies.
- Reproducibility requirements.

Integration:

- "src/compiler/passes/"
- "src/compiler/optimization/"
- "src/optimizer/"
- "src/zuir/pass/"

The compiler must integrate existing optimization authorities.

It must not create a competing optimizer.

7.12 Semantics-preserving optimization

Every transformation must establish the applicable correctness conditions.

A transformation must not silently:

- Change numerical semantics.
- Remove observable effects.
- Change quantum measurement behavior.
- Violate ownership.
- Change required synchronization.
- Remove communication dependencies.
- Alter hardware behavior.
- Violate timing semantics where timing is observable.

Approximate transformations are permitted only when the program explicitly authorizes approximation and the transformation satisfies the declared error contract.

7.13 Domain analysis

The compiler must activate applicable domain integrations.

Domain| Authority
Classical| "src/classical/"
Quantum| "src/quantum/"
HDL| "src/hdl/"
AI| "src/ai/"
Tensor| Existing canonical tensor infrastructure
Hybrid| "src/hybrid/"
Distributed| "src/distributed/"
Networking| Existing networking subsystem
Cryptography| Existing cryptographic subsystem
Embedded| Existing embedded subsystem
Future domains| Extensible domain interfaces

A domain must not be considered supported merely because an identifier exists.

Its complete compilation route must be demonstrated.

7.14 Capability discovery and compatibility

The compiler must integrate target and provider discovery.

It must establish:

- Provider identity.
- Provider version.
- Advertised capabilities.
- Capability evidence.
- Representation compatibility.
- Required operation support.
- Target compatibility.
- Applicable constraints.

Integration:

- "src/compiler/backends/"
- "src/compiler/planning/target_discovery.rs"
- "src/backend/"
- Relevant domain providers.

Unknown capability information must not be interpreted as unlimited support.

7.15 Feasibility analysis

Feasibility must be evidence-based.

It must consider:

- Mandatory requirements.
- Available capabilities.
- Resource demand.
- Resource availability evidence.
- Target restrictions.
- Representation compatibility.
- Domain constraints.
- Operational policies.

Possible outcomes must distinguish:

- Feasible.
- Infeasible.
- Conditionally feasible.
- Unknown.
- Incomplete evidence.

The compiler must not claim that a target is feasible merely because a backend is registered.

7.16 Resource planning

Resource planning determines abstract requirements and proposes realization plans.

It does not physically allocate hardware.

Integration:

- "src/compiler/planning/resource_planning.rs"
- "src/compiler/planning/feasibility.rs"
- "src/compiler/configuration/budget.rs"
- "src/frontend/semantic/"
- Relevant domain resource authorities.

The runtime and resource providers own actual resource acquisition.

7.17 Heterogeneous partitioning

The compiler must support partitioning when computation requires multiple execution domains.

Examples include:

- CPU and GPU.
- CPU and QPU.
- CPU and FPGA.
- Classical and quantum.
- CPU and AI accelerator.
- Multiple heterogeneous accelerators.
- Distributed heterogeneous environments.

Partitioning must preserve:

- Data dependencies.
- Control dependencies.
- Effects.
- Synchronization.
- Ownership.
- Communication semantics.
- Resource requirements.
- Execution contracts.

A partition boundary must not silently destroy semantic relationships.

7.18 Data movement planning

Data movement planning must consider:

- Data ownership.
- Representation compatibility.
- Memory spaces.
- Transfer requirements.
- Communication costs.
- Synchronization.
- Consistency requirements.
- Applicable resource constraints.

Physical data transfer remains a runtime/provider responsibility.

7.19 Scheduling and routing planning

The compiler may coordinate abstract scheduling and routing plans.

It must not duplicate physical scheduling and routing authorities.

Integration:

- "src/compiler/planning/scheduling.rs"
- "src/compiler/planning/routing.rs"
- "src/hybrid/"
- "src/distributed/"
- "src/quantum/"
- "src/runtime/"

Quantum routing, physical placement, QEC and device scheduling remain with their appropriate domain and realization authorities.

7.20 Domain-specific lowering

Lowering must use the authoritative domain implementation.

Examples:

- Classical lowering.
- Quantum decomposition and lowering.
- HDL lowering.
- AI graph lowering.
- Tensor lowering.
- Hybrid lowering.
- Distributed lowering.

Integration:

- "src/compiler/lowering/"
- "src/compiler/ir/"
- "src/classical/"
- "src/quantum/"
- "src/hdl/"
- "src/ai/"
- "src/hybrid/"
- "src/distributed/"

7.21 Legalization

A representation must be legalized against the intended target before backend realization.

Legalization must establish:

- Supported operations.
- Supported data representations.
- Valid operand constraints.
- Applicable precision.
- Required capabilities.
- Target compatibility.
- Preservation of required semantics.

Unsupported operations must produce explicit diagnostics.

7.22 Backend realization

The compiler must invoke the canonical backend implementation.

Integration:

- "src/compiler/backends/"
- "src/backend/"

Backend selection must use registered providers and compatibility evidence.

The compiler must not duplicate backend code generation.

Backend invocation must not be confused with physical resource allocation.

7.23 Linking and composition

Linking must preserve:

- Symbol identity.
- Module identity.
- ABI compatibility.
- Domain-specific metadata.
- Artifact dependencies.
- Representation compatibility.
- Required relocation information.

Integration:

- "src/compiler/linking/"
- "src/stdlib/"
- "src/backend/"
- "src/compiler/artifacts/"

7.24 Artifact verification

Before an artifact is accepted as a successful compilation product, the compiler must validate:

- Artifact integrity.
- Format validity.
- Representation version.
- Target compatibility.
- Required metadata.
- Dependencies.
- Provenance.
- Security information.
- Verification status.

A placeholder, empty file or fabricated success result is not a valid artifact.

7.25 Deployment handoff

The compiler may produce deployment-ready artifacts and validated deployment descriptions.

It must not claim that physical execution has occurred merely because artifact generation succeeded.

Integration:

- "src/compiler/api/"
- "src/compiler/artifacts/"
- "src/toolchain/"
- "src/runtime/"

Physical execution belongs to the runtime and its providers.

---

8. Conditional pipeline execution

The pipeline must support domain-dependent and policy-dependent stages.

A stage is activated only when its prerequisites and applicability conditions are established.

For example:

Classical program
    |
    +-- Classical IR
    +-- Classical optimization
    +-- Classical lowering
    +-- Compatible backend

Quantum program
    |
    +-- Quantum IR
    +-- Quantum verification
    +-- Quantum optimization
    +-- Quantum lowering
    +-- Required mapping and routing integration
    +-- Compatible backend

Hybrid program
    |
    +-- Classical representation
    +-- Quantum representation
    +-- Cross-domain analysis
    +-- Dependency preservation
    +-- Synchronization planning
    +-- Heterogeneous realization

The active pipeline must be derived from validated semantic information and explicit compilation policy.

The grammar alone must not activate arbitrary compiler stages.

8.1 Required versus optional stages

Every compilation request must establish which stages are mandatory.

Mandatory stages must complete successfully.

Optional stages may be skipped only when their omission is permitted by the applicable contract.

Skipping an optional optimization must not invalidate the program.

Skipping mandatory semantic validation must invalidate the compilation.

8.2 Dependency correctness

A stage may execute only after all required dependencies have completed successfully.

A skipped dependency does not satisfy a required dependency.

A blocked dependency does not satisfy a required dependency.

An incomplete dependency does not satisfy a required dependency.

The pipeline must never interpret a missing stage output as a valid empty output.

8.3 Conditional graph validation

The active stage graph must be validated before execution.

It must detect:

- Missing stages.
- Missing dependencies.
- Duplicate implementations.
- Invalid dependencies.
- Dependency cycles.
- Incompatible descriptors.
- Unavailable required implementations.
- Invalid stage contracts.

The resulting graph must be deterministic.

---

9. Pipeline ordering and execution

9.1 Ordering authority

"src/compiler/pipeline/ordering.rs" owns dependency validation and deterministic topological ordering.

It must remain independent of stage implementation.

It must not perform stage execution.

9.2 Execution authority

"src/compiler/pipeline/execution.rs" owns stage invocation and output delivery.

It must:

- Execute only validated stages.
- Respect dependency ordering.
- Preserve stage inputs.
- Preserve dependency outputs.
- Enforce required-stage completion.
- Check cancellation.
- Record stage outcomes.
- Propagate failures.
- Prevent incomplete work from becoming success.

9.3 Sequential execution

The current sequential executor is the initial correctness baseline.

It must remain available even when parallel execution is introduced.

Sequential and parallel implementations must produce equivalent semantic results under the same compilation contract.

9.4 Parallel stage execution

Parallel stage execution may be introduced through integration with the existing parallel compiler infrastructure.

It must establish:

- Dependency-safe execution.
- Explicit concurrency permissions.
- Independent stage inputs.
- Safe shared-state access.
- Correct output ownership.
- Cancellation propagation.
- Budget accounting.
- Deterministic result aggregation.
- Correct failure propagation.

Stages must not be executed concurrently merely because they occupy the same topological level.

Their concurrency contracts must permit it.

The compiler must not introduce arbitrary worker-count ceilings.

Actual parallel capacity is governed by available resources and configured operational budgets.

---

10. Source grammar integration

The compiler must respect the actual grammar hierarchy.

10.1 Complete grammar

"grammar/Zamani.g4" is the complete-language ANTLR composition root.

It composes the canonical lexical and parser boundaries.

It must not become another compiler implementation.

10.2 Lexer authority

The canonical lexical integration includes:

grammar/antlr/ZamaniLexer.g4
             |
             v
grammar/lexer/lexer.g4
             |
             v
grammar/lexer/tokens.g4
             |
             v
Canonical lexical components

Token definitions remain in their respective lexical authorities.

The compiler must not introduce a second token registry.

10.3 Parser authority

The canonical parser composition root is:

"grammar/antlr/ZamaniParser.g4"

It coordinates the source-unit, declaration, statement, expression and domain grammar boundaries.

The compiler must consume the canonical frontend output.

It must not parse source independently.

10.4 Program grammar

The program-boundary grammar is maintained under:

"grammar/core/program.g4"

The complete source structure and declaration/statement authorities remain in their existing grammar modules.

The compiler must not redefine program syntax.

10.5 Compilation grammar

The source-level compilation subsystem is maintained under:

"grammar/compile/"

Its responsibility is to describe compilation intent, profiles, target declarations, optimization intent, reproducibility, caching and deployment-related source constructs.

It does not implement the compiler pipeline.

In particular:

COMPILE declaration
        !=
Compiler Stage

A source-level target declaration is not proof of target availability.

A source-level resource requirement is not physical resource allocation.

A source-level optimization preference is not permission to violate program semantics.

A source-level compilation profile must be interpreted through the frontend and semantic authorities before influencing compiler execution.

10.6 Grammar composition and migration

The existing grammar must be audited for overlapping compilation composition roots.

In particular:

- "grammar/compile/compile.g4" remains the canonical "Compile" composition root.
- "grammar/compile/compilation.g4" must not become a competing compilation authority.
- "grammar/compile/intent.g4" must not introduce a circular composition relationship.
- Existing compatibility grammars must remain explicit migration adapters where required.

Grammar composition must be validated mechanically.

The compiler must not claim complete grammar integration merely because the grammar files exist.

10.7 Grammar conformance

The following must be tested:

1. Grammar imports resolve.
2. No circular imports exist.
3. No conflicting parser rules exist.
4. Token ownership is unambiguous.
5. The canonical parser entry point is valid.
6. Every required domain grammar is reachable.
7. Source-level compilation constructs are parsed correctly.
8. Invalid source produces appropriate diagnostics.
9. Grammar-generated behavior agrees with the canonical Rust frontend.

---

11. Semantic preservation

Semantic preservation is a non-negotiable compiler invariant.

Every transformation must preserve the observable behavior defined by the semantic model.

The compiler must maintain sufficient evidence to establish that the transformation is valid.

11.1 Classical computation

Preserve:

- Types.
- Values.
- Control flow.
- Memory semantics.
- Ownership.
- Borrowing.
- Effects.
- Concurrency semantics.
- Numerical guarantees.

11.2 Quantum computation

Preserve:

- Quantum state semantics.
- Gate and operation semantics.
- Measurement semantics.
- Entanglement relationships.
- Classical control dependencies.
- Measurement ordering.
- Required quantum effects.
- Applicable quantum resource requirements.

Classical optimization must not be applied to quantum representations without an appropriate legality and preservation contract.

11.3 HDL and hardware computation

Preserve:

- Signal identity.
- Hardware structure.
- Timing semantics where observable.
- Clock relationships.
- Reset behavior.
- State-transition semantics.
- Interface contracts.
- Structural dependencies.

The compiler must not silently transform a hardware description into ordinary sequential software.

11.4 AI and tensor computation

Preserve:

- Tensor shape contracts.
- Data types.
- Numerical precision.
- Differentiation semantics.
- Training and inference semantics.
- Model dependencies.
- Execution effects.
- Applicable approximation guarantees.

AI and tensor lowering must reuse canonical AI and tensor infrastructure.

11.5 Hybrid computation

Preserve:

- Cross-domain dependencies.
- Synchronization.
- Classical-to-quantum data flow.
- Quantum-to-classical observations.
- Measurement dependencies.
- Ownership and resource relationships.

11.6 Distributed computation

Preserve:

- Communication semantics.
- Ordering requirements.
- Consistency requirements.
- Failure contracts.
- Message dependencies.
- Synchronization.
- Data ownership.
- Security boundaries.

A distributed optimization must not silently change consistency or communication guarantees.

---

12. Requirements, capabilities and feasibility

The compiler must implement a clear separation between program intent and target realization.

12.1 Requirements

Requirements express what a valid realization must provide.

Examples include:

- Required operations.
- Required numerical precision.
- Required memory properties.
- Required quantum capabilities.
- Required communication behavior.
- Required timing behavior.
- Required security properties.

12.2 Capabilities

Capabilities express what a provider or target implementation can support.

Capability evidence must include its source and applicable version.

12.3 Preferences

Preferences influence selection but must not override mandatory requirements.

12.4 Constraints

Constraints establish what is permitted or prohibited.

12.5 Hints

Hints may improve planning but must not become mandatory semantic conditions without explicit authorization.

12.6 Feasibility

Feasibility must be evaluated using actual evidence.

The compiler must distinguish an unknown capability from a supported capability.

It must distinguish a feasible abstract plan from confirmed physical resource availability.

It must report infeasibility rather than silently modifying requirements.

---

13. Resource scalability

The compiler must scale from tiny computations to arbitrarily large computations within actual implementation and resource constraints.

The term infinity describes the absence of arbitrary universal architectural ceilings.

It does not mean that finite physical machines provide infinite memory, storage or computation.

13.1 Prohibited arbitrary ceilings

The compiler must not introduce universal fixed limits for:

- CPU count.
- Core count.
- Thread count.
- Worker count.
- GPU count.
- FPGA count.
- ASIC count.
- QPU count.
- Accelerator count.
- Distributed nodes.
- Processes.
- Devices.
- Memory capacity.
- Storage capacity.
- Register width.
- Qubit count.
- Tensor rank.
- Tensor dimensions.
- Vector width.
- Program size.
- Module count.
- Function count.
- IR operation count.
- Pipeline stage count.
- Dependency count.
- Backend provider count.
- Domain count.

13.2 Actual implementation limitations

A particular target or implementation may have finite constraints.

Examples:

- An instruction format has a defined operand width.
- A physical QPU exposes a finite number of usable qubits.
- A particular FPGA has finite logic resources.
- A process has finite address space.
- A particular backend cannot represent a particular operation.
- An operating system limits an individual allocation.

These are genuine implementation or target constraints.

They must be represented accurately and reported at the appropriate boundary.

They must not become universal language or compiler limits.

13.3 Operational budgets

Operational budgets are permitted.

They may govern:

- Compiler memory consumption.
- Compilation time.
- Optimization effort.
- Number of retries.
- Cache storage.
- Artifact storage.
- Parallel work.
- External tool invocation.
- Temporary resources.

Budgets must be configurable and explicitly accounted for.

A budget must not silently change program semantics.

Budget exhaustion must produce an explicit outcome.

13.4 Budget enforcement

"src/compiler/configuration/budget.rs" provides the existing accounting foundation.

The compiler must ensure that relevant resource-consuming operations are accounted for.

Accounting must cover, as applicable:

- Input processing.
- Analysis.
- Transformations.
- Lowering.
- Parallel work.
- Backend invocation.
- Artifact generation.
- Cache operations.

The accounting model must distinguish measured compiler consumption from physical hardware allocation.

A budget ledger does not itself enforce operating-system memory limits.

Where stronger enforcement is required, the responsible execution boundary must provide it.

13.5 Large-program architecture

The compiler should use:

- Dependency-aware processing.
- Incremental compilation.
- Content-addressed caching.
- Immutable snapshots.
- Bounded operational work.
- Streaming where appropriate.
- Parallel processing where safe.
- Modular artifacts.
- Deterministic graph processing.

No arbitrary universal size ceiling may be introduced merely for convenience.

All arithmetic, indexing, size conversions and allocations must handle overflow and allocation failure explicitly.

---

14. Determinism and reproducibility

The compiler must provide reproducible compilation under an explicitly defined reproducibility contract.

Deterministic compilation must consider:

- Source identity.
- Grammar version.
- Language version.
- Semantic-model version.
- Compiler version.
- Compiler API versions.
- IR schema versions.
- Configuration.
- Optimization policies.
- Pass identities and versions.
- Backend identity and version.
- Dependency identities.
- Relevant environment inputs.

Nondeterministic information must not silently affect reproducible output.

Where physical realization inherently depends on dynamic hardware conditions, the artifact must record the applicable realization evidence.

Determinism does not mean that every physical execution produces identical measurements or results when the language explicitly permits nondeterminism.

14.1 Integration

- "src/compiler/reproducibility/"
- "src/compiler/incremental/"
- "src/compiler/artifacts/"
- "src/compiler/context/"
- "src/compiler/security/"

A cache hit must never bypass mandatory verification or security validation.

---

15. Incremental compilation

Incremental compilation must preserve correctness.

Cache identity must account for all relevant inputs, including:

- Source content.
- Imported modules.
- Semantic dependencies.
- Compiler configuration.
- Compiler version.
- Grammar and language versions.
- IR versions.
- Optimization policies.
- Backend versions.
- Provider identities.
- Relevant target information.

Invalidation must be dependency-driven.

A changed input must invalidate every affected dependent result.

Unchanged independent results may be reused when their compatibility and provenance are established.

Cache corruption must never be treated as a successful cache hit.

Integration:

- "src/compiler/incremental/"
- "src/compiler/graph/" where needed
- "src/compiler/reproducibility/"
- "src/compiler/security/"
- "src/compiler/artifacts/"

---

16. Parallel compilation and concurrency

The compiler must distinguish three separate concerns:

1. Parallel execution of compiler stages.
2. Parallel compilation of independent source units.
3. Parallel execution of the compiled program.

These must not be implemented as one shared scheduler.

Compiler-stage parallelism belongs to compiler infrastructure.

Independent compilation work belongs to the compiler build infrastructure.

Physical program scheduling belongs to runtime and domain authorities.

16.1 Required guarantees

Parallel compiler execution must preserve:

- Dependency correctness.
- Ownership.
- Data isolation.
- Cancellation.
- Error propagation.
- Resource accounting.
- Deterministic aggregation.
- Semantic equivalence with the sequential implementation.

The compiler must not require a fixed worker count.

16.2 Integration

- "src/compiler/parallel/"
- "src/compiler/pipeline/"
- "src/compiler/incremental/"
- "src/compiler/context/"

The sequential executor remains the reference correctness implementation until parallel execution has been qualified.

---

17. Failure handling and recovery

Failures must be explicit and structured.

The compiler must distinguish:

- Source errors.
- Syntax errors.
- Semantic errors.
- IR verification errors.
- Unsupported operations.
- Capability incompatibility.
- Resource infeasibility.
- Budget exhaustion.
- Backend failures.
- Artifact validation failures.
- Linking failures.
- Security failures.
- Cancellation.
- Internal compiler errors.
- Incomplete work.

17.1 Failure propagation

A failed required stage must prevent dependent stages from reporting successful completion.

Failures must retain:

- Stage identity.
- Failure category.
- Error details.
- Relevant source location.
- Dependency context.
- Provenance.
- Recovery eligibility.

17.2 Recovery

"src/compiler/pipeline/recovery.rs" owns recovery decisions and policy evaluation.

It does not itself execute retry or fallback operations.

Actual recovery execution must be coordinated by orchestration and the relevant stage/provider.

A retry is permitted only when:

- The original failure permits retry.
- Replay safety is established.
- The applicable policy authorizes it.
- Operational budget remains.
- The retry cannot violate correctness.

A fallback is permitted only when:

- Semantic equivalence is verified.
- Mandatory requirements are preserved.
- Constraints are preserved.
- The fallback implementation is verified.
- The realization is compatible.
- Provenance is available.

A fallback must never silently replace mandatory quantum, HDL, numerical, security or distributed semantics with a weaker implementation.

17.3 Cancellation

Cancellation must propagate through the pipeline.

A cancellation request must not be converted into success.

A cancelled compilation must not publish incomplete artifacts as completed outputs.

---

18. Backend integration

The compiler must use the canonical backend subsystem.

"src/compiler/backends/" provides compiler-facing discovery, compatibility, selection and invocation coordination.

"src/backend/" owns backend implementations.

The compiler must not create a competing backend implementation framework.

Backend providers must expose sufficient information to establish:

- Identity.
- Version.
- Supported representations.
- Supported operations.
- Required capabilities.
- Output artifact types.
- Compatibility.
- Applicable restrictions.
- Verification interfaces.

Adding a new backend must not require changing a large closed compiler enumeration.

Backend selection must be based on explicit requirements, constraints, capabilities and preferences.

Registration alone is not proof of compatibility.

Compatibility alone is not proof of physical resource availability.

---

19. Artifact contract

Every produced artifact must have an identifiable contract.

Artifact metadata must preserve, where applicable:

- Artifact identity.
- Artifact type.
- Artifact version.
- Language version.
- Compiler version.
- Semantic-model version.
- Source provenance.
- IR representation identity.
- IR version.
- Backend identity.
- Backend version.
- Target compatibility.
- Dependencies.
- Verification evidence.
- Security metadata.
- Reproducibility information.

The compiler must distinguish:

1. Portable semantic artifacts.
2. Portable intermediate artifacts.
3. Target-specific intermediate artifacts.
4. Executable artifacts.
5. Hardware-description artifacts.
6. Deployment artifacts.
7. Runtime-managed artifacts.

A portable artifact does not automatically constitute an executable artifact.

An executable artifact does not automatically constitute proof of successful execution.

Integration:

- "src/compiler/artifacts/"
- "src/compiler/linking/"
- "src/compiler/security/"
- "src/compiler/reproducibility/"
- "src/backend/"
- "src/runtime/"

---

20. Security and trust boundaries

The pipeline must protect against malformed inputs, malicious extensions, invalid representations and compromised providers.

Required security properties include:

- Input validation.
- IR validation.
- Artifact integrity.
- Provider identity validation.
- Dependency integrity.
- Extension admission.
- Backend isolation where required.
- Secure serialization.
- Supply-chain verification.
- Provenance validation.
- Resource-exhaustion protection.
- Diagnostic safety.

Untrusted provider output must never be treated as trusted merely because the provider is registered.

A provider's claimed verification result must be validated according to the applicable trust policy.

Integration:

- "src/compiler/security/"
- "src/compiler/extensions/" where required
- "src/compiler/artifacts/"
- "src/compiler/backends/"
- "src/compiler/pipeline/"

---

21. Public compiler API integration

The compiler API must provide a genuine production entry point.

The canonical public boundary is:

"src/compiler/api/"

The production driver must coordinate:

1. Compilation request validation.
2. Compilation context construction.
3. Frontend invocation.
4. Semantic validation.
5. IR integration.
6. Pipeline construction.
7. Stage implementation binding.
8. Dependency validation.
9. Stage execution.
10. Required-stage enforcement.
11. Artifact validation.
12. Compilation reporting.

The public driver must not simply delegate to an unrelated legacy compiler.

21.1 Legacy migration

"src/lib.rs" exposes legacy compilation infrastructure.

The legacy compilation entry point must be treated as a compatibility boundary during migration.

It must not remain an independent production compilation architecture.

The migration must:

- Preserve supported existing public APIs.
- Introduce a controlled delegation path.
- Avoid duplicate semantic analysis.
- Avoid duplicate IR construction.
- Avoid incompatible diagnostics.
- Establish explicit deprecation and migration rules.

The canonical production compiler must ultimately own the compilation workflow.

---

22. Compiler module integration map

The following table defines the expected integration contracts for the existing compiler directory.

Compiler module| Responsibility| Integrates with
"api/"| Public compilation service| Pipeline, context, artifacts
"analysis/"| Compiler-facing analysis| Frontend, semantic model, IR
"artifacts/"| Artifact contracts| Backend, linking, security
"backends/"| Backend coordination| "src/backend/", planning
"configuration/"| Configuration and budgets| Context, planning, pipeline
"context/"| Compilation-wide state| API, pipeline, diagnostics
"diagnostics/"| Diagnostics integration| Frontend, source maps, pipeline
"incremental/"| Incremental compilation| Graph, artifacts, context
"ir/"| IR integration| ZUIR and domain IR authorities
"linking/"| Linking coordination| Backend, artifacts, stdlib
"lowering/"| Lowering coordination| IR and domain authorities
"optimization/"| Optimization coordination| Optimizer and pass infrastructure
"parallel/"| Parallel build coordination| Pipeline, context
"passes/"| Compiler pass coordination| ZUIR pass infrastructure
"pipeline/"| Compilation workflow| All required compiler stages
"planning/"| Feasibility and realization planning| Semantic requirements, capabilities, providers
"profiling/"| Compilation profiling| Analysis and optimization
"security/"| Compiler security| Extensions, backends, artifacts
"testing/"| Compiler qualification| All compiler and domain modules

No module may duplicate another module's canonical responsibility.

---

23. Extension and future-domain integration

Zamani must support future computational domains without redesigning the compiler.

The architecture must support extensibility through existing or appropriately established:

- Domain interfaces.
- Stage contracts.
- Backend provider contracts.
- Capability registries.
- Pass registries.
- Representation adapters.
- Artifact contracts.
- Verification interfaces.

A new domain must provide:

1. A defined semantic integration.
2. An authoritative representation.
3. A verification contract.
4. Capability descriptions.
5. Applicable requirements and constraints.
6. Lowering interfaces.
7. Compatible backend integration.
8. Artifact compatibility.
9. Conformance tests.

A domain is not production-supported merely because it can register a stage.

The compiler must not create a closed enumeration of all future computational domains.

---

24. Compilation result contract

A compilation result must accurately describe what happened.

It must distinguish:

- Successful compilation.
- Successful portable artifact generation.
- Successful target-specific realization.
- Successful linking.
- Successful packaging.
- Successful deployment preparation.
- Successful physical execution.

These are different outcomes.

A successful compilation result must establish that all mandatory compilation stages completed and all required verification and artifact checks passed.

It must not claim runtime execution unless runtime execution was actually performed and reported by the runtime.

The result must include:

- Compilation identity.
- Final status.
- Diagnostics.
- Completed stage records.
- Skipped optional stages.
- Required-stage completion.
- Artifact references.
- Verification evidence.
- Provenance.
- Relevant compatibility information.
- Resource-accounting summary.

Incomplete results must remain explicitly incomplete.

---

25. Testing and production qualification

The compiler must be qualified through actual end-to-end compilation.

Module import tests are insufficient.

A successful documentation build is insufficient.

A stage-registration test is insufficient.

A compiler that only produces placeholder artifacts is not production-ready.

25.1 Core test fixtures

The canonical fixture suite must include:

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

Each fixture must test real frontend, semantic, IR and compilation integration.

25.2 Pipeline tests

Required tests include:

- Valid dependency ordering.
- Dependency cycle rejection.
- Missing dependency rejection.
- Duplicate stage implementation rejection.
- Missing required stage rejection.
- Conditional stage activation.
- Optional stage skipping.
- Required stage skipping rejection.
- Dependency output delivery.
- Invalid payload rejection.
- Cancellation propagation.
- Failure propagation.
- Recovery policy enforcement.
- Deterministic execution.
- Correct compilation lifecycle.
- Resource-budget enforcement.

25.3 Domain tests

Classical:

- Correct lowering.
- Correct optimization.
- Valid target artifacts.

Quantum:

- Canonical quantum IR verification.
- Quantum semantic preservation.
- Measurement correctness.
- Required routing and lowering integration.

HDL:

- Structural validity.
- Signal and timing preservation.
- Valid hardware-description artifacts.

Hybrid:

- Cross-domain dependency preservation.
- Synchronization correctness.
- Classical/quantum integration.

Distributed:

- Communication preservation.
- Dependency correctness.
- Consistency and ordering requirements.

AI and tensor:

- Tensor shape preservation.
- Numerical contracts.
- Differentiation correctness.
- Model and graph integration.

25.4 Scalability tests

The compiler must test:

- Tiny programs.
- Large programs.
- Deep dependency graphs.
- Wide dependency graphs.
- Large modules.
- Large IR representations.
- High dependency fan-in.
- High dependency fan-out.
- Incremental rebuilds.
- Parallel compilation.
- Memory-budget exhaustion.
- Compilation-time budget exhaustion.
- Allocation failures.
- Overflow handling.

Tests must verify graceful handling of actual resource exhaustion rather than relying on arbitrary artificial limits.

25.5 Determinism tests

Equivalent compilation inputs under equivalent declared environments must produce equivalent deterministic results.

Test:

- Repeated builds.
- Different stage registration orders.
- Different valid execution schedules.
- Incremental versus clean builds.
- Cache hits and cache misses.
- Equivalent provider discovery orders.

Where output identity is intentionally nondeterministic, the relevant nondeterministic fields must be specified and excluded from inappropriate equivalence comparisons.

25.6 Security tests

Test:

- Malformed input.
- Invalid IR.
- Invalid artifacts.
- Corrupt cache entries.
- Untrusted provider metadata.
- Invalid extension manifests.
- Resource exhaustion.
- Invalid serialized data.
- Provenance tampering.

---

26. Rust implementation requirements

All compiler implementation must use:

- Rust 2021 edition.
- Rust 1.97 or later.
- Safe Rust only.

The compiler hierarchy must prohibit unsafe code.

Required crate-level or module-level enforcement:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

New compiler modules must follow the existing safety requirements.

Public APIs must document:

- Purpose.
- Ownership.
- Lifetimes.
- Error behavior.
- Concurrency.
- Determinism.
- Compatibility.
- Integration requirements.

The compiler must avoid unnecessary cloning of large representations.

Ownership and borrowing must be explicit.

Shared state must have an appropriate synchronization model.

Errors must not be silently discarded.

Arithmetic and allocation must handle failure explicitly.

---

27. Production quality gates

The following checks are mandatory:

cargo fmt --all -- --check

cargo check --all-targets

cargo test --all-targets

cargo clippy --all-targets -- -D warnings

Additionally:

- Grammar composition validation must pass.
- Frontend conformance tests must pass.
- IR verification tests must pass.
- Pipeline integration tests must pass.
- Cross-domain tests must pass for every claimed supported domain.
- Artifact validation must pass.
- Reproducibility tests must pass.
- Security tests must pass.
- Scalability tests must pass.

Broad lint allowances must not be used to conceal unfinished compiler implementation.

---

28. Required implementation and integration work

This section defines the implementation work required to make the existing pipeline production-ready.

It deliberately expands existing files rather than introducing duplicate subsystems.

File| Required production work
"pipeline/mod.rs"| Preserve canonical exports, enforce module boundaries, document stable public APIs and versioning.
"pipeline/stage.rs"| Complete stage contracts, input/output validation, cancellation, determinism, concurrency, failure and provenance obligations.
"pipeline/stages.rs"| Complete standard stage identities and dependency declarations; ensure identifiers remain extensible.
"pipeline/ordering.rs"| Validate complete active graphs, missing dependencies, cycles, deterministic ordering and dependency levels.
"pipeline/execution.rs"| Enforce required stages, validate inputs and outputs, account for budgets, propagate cancellation and preserve stage records.
"pipeline/failure.rs"| Normalize all pipeline failures without losing their original causes or evidence.
"pipeline/recovery.rs"| Preserve conservative recovery decisions and connect actual recovery execution through orchestration.
"pipeline/orchestration.rs"| Connect the real stage implementations, construct active graphs, enforce lifecycle transitions and produce truthful compilation results.
"configuration/budget.rs"| Complete operational budget accounting and ensure all applicable consumers propagate exhaustion.
"context/compilation_context.rs"| Integrate configuration, cancellation, budgets, provenance, shared services and controlled compilation state.
"context/compilation_state.rs"| Enforce valid lifecycle transitions and prohibit incomplete success.
"context/dependency_context.rs"| Integrate canonical dependency evidence.
"context/provenance_context.rs"| Preserve source, transformation, provider and artifact provenance.
"analysis/requirements.rs"| Consume authoritative semantic requirements without redefining their meaning.
"analysis/capabilities.rs"| Integrate provider capability evidence.
"analysis/feasibility.rs"| Produce explicit evidence-based feasibility results.
"ir/input.rs"| Accept only appropriately validated canonical representations.
"ir/adapters.rs"| Integrate authoritative domain adapters.
"ir/verification.rs"| Preserve valid, invalid, incomplete and unverified distinctions.
"backends/registry.rs"| Integrate canonical provider registration without creating competing registries.
"backends/compatibility.rs"| Verify representation and capability compatibility.
"backends/invocation.rs"| Validate backend requests and propagate actual invocation results.
"planning/resource_planning.rs"| Keep resource demand assessment separate from physical allocation.
"planning/partitioning.rs"| Preserve cross-domain dependencies and semantics.
"planning/scheduling.rs"| Produce abstract plans without duplicating runtime scheduling.
"lowering/"| Integrate domain-owned transformations and target legalization.
"optimization/"| Coordinate existing optimizers and verify semantic preservation.
"passes/"| Integrate canonical pass infrastructure and validate dependencies.
"parallel/"| Implement safe dependency-aware parallel compilation with deterministic aggregation.
"incremental/"| Establish complete content-based cache identity and precise invalidation.
"artifacts/"| Implement complete artifact identity, metadata, integrity and portability validation.
"linking/"| Implement symbol, ABI, dependency and cross-domain composition validation.
"security/"| Establish provider, extension, artifact and supply-chain trust boundaries.
"api/compiler.rs"| Connect the production compiler driver to the complete pipeline.
"api/request.rs"| Validate immutable compilation requests.
"api/response.rs"| Return complete and truthful compilation results.
"testing/"| Establish end-to-end conformance, security, determinism and scalability tests.

No new generic compiler subsystem should be created when an existing subsystem already owns the responsibility.

Any additional directory must have a distinct ownership boundary, documented integration contract and independently testable purpose.

---

29. Dependency and implementation order

Implementation must proceed in dependency order.

The goal is to define stable contracts before downstream modules consume them.

Phase 0: Architectural contract freeze

Review and align:

src/compiler/documentation/ARCHITECTURE.md
src/compiler/documentation/INTEGRATION_CONTRACT.md
src/compiler/documentation/PORTABILITY_CONTRACT.md
src/compiler/documentation/PIPELINE_CONTRACT.md

Freeze:

- Canonical ownership.
- Dependency direction.
- Error conventions.
- API versioning.
- Representation authority.
- Provenance rules.
- Budget rules.
- Stage input/output contracts.
- Determinism requirements.

Phase 1: Independent foundations

Complete and freeze:

configuration/budget.rs
configuration/target.rs
configuration/policy.rs
configuration/profile.rs
configuration/config.rs
configuration/validation.rs
diagnostics/reporting.rs
diagnostics/rendering.rs
diagnostics/aggregation.rs
compiler_types.rs

The exact order must follow actual type dependencies.

Phase 2: Evidence and identity

analysis/provenance.rs
context/provenance_context.rs
context/dependency_context.rs
analysis/requirements.rs
analysis/capabilities.rs
analysis/cost.rs

Phase 3: Compilation context and analysis

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

ir/compatibility.rs
ir/input.rs
ir/adapters.rs
ir/verification.rs
ir/translation.rs
ir/lowering.rs
ir/mod.rs

Phase 5: Backend integration

backends/registry.rs
backends/capabilities.rs
backends/discovery.rs
backends/compatibility.rs
backends/selection.rs
backends/invocation.rs
backends/mod.rs

Phase 6: Planning

planning/target_discovery.rs
planning/capability_matching.rs
planning/resource_planning.rs
planning/feasibility.rs
planning/data_movement.rs
planning/partitioning.rs
planning/routing.rs
planning/scheduling.rs
planning/mod.rs

Phase 7: Pipeline

pipeline/stage.rs
pipeline/stages.rs
pipeline/ordering.rs
pipeline/failure.rs
pipeline/execution.rs
pipeline/recovery.rs
pipeline/orchestration.rs
pipeline/mod.rs

These files already exist and must be audited and expanded rather than blindly rewritten.

Phase 8: Transformations

passes/
optimization/
lowering/
monomorphizer.rs
instruction_fusion.rs
macro_engine.rs

Phase 9: Parallel and incremental compilation

parallel/
incremental/
graph/

Only introduce the additional graph directory if existing dependency authorities cannot adequately represent compiler workflow dependencies.

Phase 10: Artifacts and linking

artifacts/
linking/

Phase 11: Security and reproducibility

security/
reproducibility/
extensions/

Add the latter directories only when their responsibilities cannot be fulfilled by existing modules without violating ownership boundaries.

Phase 12: Production API

Complete:

api/

Connect the canonical frontend, production pipeline and backend integration.

Migrate legacy entry points through controlled compatibility adapters.

Phase 13: Cross-domain qualification

Demonstrate complete compilation paths for every claimed supported domain.

---

30. File completion and freeze requirements

Every pipeline implementation file must have a complete contract before implementation begins.

Each file must establish:

1. Purpose.
2. Responsibilities.
3. Non-responsibilities.
4. Canonical authorities.
5. Public types.
6. Public methods.
7. Exact dependencies.
8. Input contracts.
9. Output contracts.
10. Ownership and borrowing.
11. Error variants.
12. Error propagation.
13. Concurrency guarantees.
14. Determinism guarantees.
15. Cancellation behavior.
16. Budget obligations.
17. Security requirements.
18. Integration details.
19. Unit tests.
20. Integration tests.
21. Completion criteria.

A file is frozen only when:

- Its public contract is complete.
- Its dependencies are implemented or represented by stable interfaces.
- Its integration tests pass.
- Its error behavior is tested.
- Its scalability requirements are satisfied.
- Its documentation agrees with the implementation.
- No unresolved ownership conflict exists.

A frozen file must not be modified merely because a downstream developer failed to follow its contract.

If a genuine defect in the contract is discovered, the change must be explicit, justified, versioned where necessary and tested against all affected consumers.

No architecture can honestly guarantee that a defective contract will never need correction. The objective is to eliminate avoidable downstream redesign.

---

31. Production-readiness acceptance criteria

The Zamani compiler pipeline may be declared production-ready only when all the following conditions are met.

Architecture

- One canonical pipeline exists.
- All ownership boundaries are respected.
- No competing semantic or IR authority exists.
- All required stages have actual implementations.
- Stage registration and activation are complete.
- Dependency validation is enforced.

Correctness

- Semantic preservation is established.
- Canonical IR verification is enforced.
- Transformation legality is enforced.
- Required stages cannot be silently skipped.
- Incomplete work cannot become success.
- Failure propagation is complete.

Domain support

- Classical compilation works end to end.
- Quantum compilation works end to end.
- HDL compilation works end to end.
- AI and tensor compilation works for claimed supported features.
- Hybrid compilation preserves cross-domain semantics.
- Distributed compilation preserves communication contracts.

Portability

- Source portability is tested.
- Semantic portability is tested.
- IR compatibility is tested.
- Artifact portability is tested.
- Execution compatibility is established through actual runtime/provider integration.

Scalability

- No arbitrary universal hardware ceilings exist.
- Operational budgets are configurable.
- Resource exhaustion is handled explicitly.
- Large dependency graphs are tested.
- Large programs are tested.
- Incremental compilation is correct.
- Parallel compilation is safe.

Reliability

- Determinism requirements are tested.
- Reproducibility is established.
- Cache invalidation is correct.
- Cancellation is propagated.
- Recovery is safe.
- Artifacts are validated.
- Provenance is preserved.

Security

- Inputs are validated.
- IRs are verified.
- Extensions follow trust policies.
- Providers are validated.
- Artifacts are protected.
- Resource exhaustion cannot silently corrupt compilation.

Rust quality

- Rust 1.97 or later.
- Rust 2021.
- No unsafe code.
- All required Cargo quality gates pass.

A documented architecture is not sufficient evidence of production readiness.

The actual implementation must demonstrate these properties.

---

32. Final architectural law

The Zamani compiler must follow this law:

One language. One semantic authority. Canonical domain representations. One coordinated compilation architecture. Extensible realization providers. Verified portable artifacts. Runtime-owned physical execution.

The compiler must preserve the meaning of a Zamani program while allowing its realization to change according to available capabilities and resources.

It must not impose arbitrary universal machine-size limits.

It must not confuse an abstract resource plan with physical resource allocation.

It must not confuse backend registration with backend compatibility.

It must not confuse successful compilation with successful execution.

It must not confuse a source-level compilation declaration with an implementation-level compiler stage.

It must not silently sacrifice correctness to achieve portability.

POCO-REAF therefore means:

Write the program once. Establish its meaning once. Preserve that meaning through compilation. Produce appropriately portable and verifiable artifacts. Realize the program wherever compatible capabilities and resources exist, now and in future computing environments.

The only limitations must be genuine semantic, implementation, representation, security, operational or physical constraints—not arbitrary architectural ceilings introduced by the compiler.