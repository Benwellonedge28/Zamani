Zamani Universal Meta-Compiler

Production Integration Contract

Canonical path: "src/compiler/documentation/INTEGRATION_CONTRACT.md"

Repository: "github.com/Benwellonedge28/Zamani"

Subsystem: Universal Meta-Compiler (UMC)

Primary objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

Rust edition: 2021

Minimum Rust version: Rust 1.97 or later

Memory safety: Safe Rust only. "unsafe" is prohibited.

Scope: Compiler-wide integration, repository ownership, grammar integration, compilation orchestration, IR interoperability, optimization, heterogeneous planning, lowering, backends, artifacts, linking, incremental compilation, security, scalability, portability, testing and production acceptance.

Document classification: Normative integration contract.

Status: Production architecture contract; implementation conformance must be independently demonstrated.

---

1. PURPOSE

This document defines the authoritative integration contract for Zamani's "src/compiler/" subsystem.

Its purpose is to establish how every compiler component integrates with the existing repository without creating competing architectures, duplicate representations, duplicate semantic authorities or conflicting execution systems.

The compiler MUST enable developers to write a Zamani program once and preserve its specified meaning when compiling and realizing it across different compatible computing environments.

The architecture MUST support:

- Classical and systems computing.
- Embedded and resource-constrained computing.
- Parallel and concurrent computing.
- Scientific and numerical computing.
- High-performance computing.
- Distributed computing.
- Quantum computing.
- Quantum-classical hybrid computing.
- Quantum simulation.
- Quantum error-correction integration.
- AI and machine learning.
- Deep learning and neural networks.
- Tensor and data computation.
- Symbolic and probabilistic computation.
- Neuro-symbolic computation.
- Autonomous and multi-agent computation.
- Hardware description and hardware/software co-design.
- FPGA and ASIC realization.
- GPU and accelerator computing.
- Neuromorphic computing.
- Photonic computing.
- Networking and communication.
- Cryptographic and secure computation.
- Future computational paradigms.

These capabilities MUST remain domains of one programming language and one coordinated compilation architecture.

The compiler MUST expand existing repository capabilities rather than independently recreate them.

1.1 Fundamental objective

The desired compilation architecture is:

                 ZAMANI SOURCE
                       |
                       v
                CANONICAL GRAMMAR
                       |
                       v
                     LEXER
                       |
                       v
                    PARSER
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
                CANONICAL IRs
                       |
                       v
             IR VERIFICATION
                       |
                       v
              COMPILER ANALYSIS
                       |
                       v
             SEMANTICS-PRESERVING
                 TRANSFORMATIONS
                       |
                       v
              OPTIMIZATION PLANNING
                       |
                       v
          CAPABILITY AND FEASIBILITY
                  ANALYSIS
                       |
                       v
            HETEROGENEOUS PLANNING
                       |
                       v
              DOMAIN LOWERING
                       |
                       v
              TARGET LOWERING
                       |
                       v
               BACKEND EMISSION
                       |
                       v
             ARTIFACT VALIDATION
                       |
                       v
               LINKING/PACKAGING
                       |
                       v
             DEPLOYMENT HANDOFF
                       |
                       v
                 ZAMANI RUNTIME
                       |
                       v
                   EXECUTION

The architecture MUST permit different valid compilation routes.

For example:

- A classical program MUST NOT be forced through quantum lowering.
- A quantum program MUST use the canonical quantum representation.
- An AI program MUST integrate with the canonical AI and tensor representations.
- An HDL program MUST preserve hardware-description semantics.
- A hybrid program MUST preserve cross-domain dependencies and synchronization.
- A distributed program MUST preserve communication and consistency semantics.

The compiler determines which stages apply.

1.2 Meaning of infinity

The expression "scale to infinity" means:

No arbitrary universal finite capacity ceiling is imposed by the language or compiler architecture.

It does not mean physical resources are infinite.

The compiler MUST scale subject to:

1. Program semantics.
2. Available implementation capabilities.
3. Available memory.
4. Available processing capacity.
5. Available storage.
6. Configured operational budgets.
7. Target capabilities.
8. Representation constraints.
9. Security policies.
10. Physical reality.

Resource exhaustion MUST be reported explicitly.

The compiler MUST NOT silently truncate, discard or simplify a computation merely because it exceeds an internal arbitrary limit.

---

2. NORMATIVE LANGUAGE

The terms MUST, MUST NOT, REQUIRED, SHOULD, SHOULD NOT and MAY are normative.

- MUST: Mandatory.
- MUST NOT: Prohibited.
- SHOULD: Strongly recommended unless a documented reason justifies otherwise.
- SHOULD NOT: Normally prohibited unless a documented reason justifies otherwise.
- MAY: Optional.

A documented architecture is not proof of implementation.

A module declaration is not proof of a functioning implementation.

A generated filename is not proof of executable code generation.

A successful compilation is not proof of portability.

Production readiness requires implementation evidence and passing conformance tests.

---

3. ARCHITECTURAL AUTHORITIES

The compiler is an orchestration and transformation subsystem.

It MUST consume and coordinate existing repository authorities.

It MUST NOT become the owner of every subsystem.

3.1 Repository ownership matrix

Responsibility| Canonical owner
Language architecture| "grammar/DESIGN.md"
Language specifications| "grammar/specification/"
Machine-oriented semantic contracts| "grammar/spec/"
Source grammar| "grammar/"
Lexer| Existing frontend lexer
Parser| Existing frontend parser
AST| "src/frontend/ast/"
Semantic authority| Canonical frontend semantic subsystem
Legacy semantic compatibility| "src/semantic.rs", subject to migration
Universal IR| "src/zuir/"
Classical computation| "src/classical/"
Classical IR| "src/classical/zcir/"
Quantum computation| "src/quantum/"
Quantum IR| "src/quantum/ir/"
Quantum error correction| "src/quantum/qec/"
Quantum noise and fault semantics| "src/quantum/zqn/"
HDL and hardware IR| "src/hdl/hdlir/"
AI computation| "src/ai/"
Tensor computation| Existing tensor subsystem
Hybrid computation| "src/hybrid/"
Distributed computation| "src/distributed/"
Optimization algorithms| "src/optimizer/" and domain optimization subsystems
Backend registration and implementation| "src/backend/"
Runtime execution| "src/runtime/"
Toolchain integration| "src/toolchain/"
Source mapping| Existing source-map authority
Compiler orchestration| "src/compiler/"

The compiler owns the coordination of compilation.

It does not acquire the ownership of the systems in this table merely because it invokes them.

3.2 Semantic ownership

The semantic subsystem remains the authority for:

- Name resolution.
- Types.
- Type relationships.
- Ownership.
- Borrowing.
- Lifetimes.
- Effects.
- Resource semantics.
- Requirements.
- Capabilities.
- Constraints.
- Preferences.
- Hints.
- Contracts.
- Policies.
- Provenance of semantic facts.
- Domain-crossing validity.

The compiler MAY perform additional analyses required for transformation and realization.

However, those analyses MUST consume canonical semantic facts.

They MUST NOT establish competing source-language meaning.

In particular, compiler-side type inference and ownership modules MUST either:

1. Integrate with the canonical semantic implementation; or
2. Remain explicitly scoped to compiler-specific analysis that does not override semantic authority.

A successful compiler invocation MUST NOT be reported if required semantic validation has been skipped.

3.3 IR ownership

The compiler MUST use existing canonical representations.

It MUST NOT introduce another universal IR or another representation that silently competes with an existing domain IR.

The canonical boundaries are:

Universal computation:
    src/zuir/

Classical computation:
    src/classical/

Classical IR:
    src/classical/zcir/

Quantum computation:
    src/quantum/

Quantum IR:
    src/quantum/ir/

Hardware description:
    src/hdl/

Hardware IR:
    src/hdl/hdlir/

AI:
    src/ai/

Hybrid:
    src/hybrid/

Distributed:
    src/distributed/

A representation adapter is permitted only when its conversion contract, semantics and correctness are established.

Similar-looking structures MUST NOT be assumed to have equivalent meanings.

3.4 Backend ownership

"src/backend/" remains the authoritative backend subsystem.

"src/compiler/backends/" provides compiler-facing integration.

The compiler MUST NOT introduce a competing backend factory, registry or backend implementation framework.

3.5 Runtime ownership

"src/runtime/" remains responsible for execution.

The compiler may produce:

- Execution plans.
- Resource requirements.
- Target descriptors.
- Deployment artifacts.
- Runtime compatibility metadata.
- Execution contracts.

It MUST NOT duplicate runtime resource allocation, physical scheduling, device management or execution.

---

4. GRAMMAR INTEGRATION CONTRACT

The compiler MUST conform to the existing grammar architecture.

It MUST NOT create a second source-language grammar, a second parser or an alternative definition of program meaning.

4.1 Grammar authority hierarchy

The following hierarchy governs source-language integration.

File/directory| Responsibility
"grammar/DESIGN.md"| Grammar architecture and ownership authority
"grammar/specification/"| Normative language specifications
"grammar/spec/"| Domain and machine-oriented semantic contracts
"grammar/Zamani.g4"| Canonical ANTLR grammar composition
"grammar/antlr/ZamaniLexer.g4"| ANTLR lexical boundary
"grammar/antlr/ZamaniParser.g4"| ANTLR parser composition boundary
"grammar/lexer/tokens.g4"| Shared token registry
"grammar/lexer/"| Lexical contracts and supporting grammar
"grammar/core/program.g4"| Program boundary
"grammar/core/source-unit.g4"| Source-file and source-unit boundaries
"grammar/compile/compile.g4"| Compilation syntax composition
"grammar/compile/intent.g4"| Compilation intent
"grammar/compile/profiles.g4"| Source-level compilation profiles
"grammar/compile/target.g4"| Source-level target intent
"grammar/compile/lowering.g4"| Source-level lowering intent
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Historical and extended design reference

The grammar authority and specifications MUST be consulted before implementing or changing compiler behavior associated with source constructs.

4.2 Grammar versus compilation

The grammar defines source syntax.

The semantic subsystem defines source meaning.

The compiler interprets validated compilation intent.

The backend realizes valid target-specific computation.

The runtime executes the resulting artifacts.

The following separation is mandatory:

Source syntax
     |
     v
AST construction
     |
     v
Semantic validation
     |
     v
Validated compilation intent
     |
     v
Compiler configuration and planning
     |
     v
Target realization

A source-level target declaration MUST NOT be treated as proof that the target exists or possesses the requested capabilities.

4.3 Compilation grammar integration

"grammar/compile/compile.g4" owns compilation syntax composition.

Its constituent grammars describe source-level compilation concepts.

The compiler MUST consume their validated semantic interpretation.

It MUST NOT reimplement their syntax in Rust.

In particular:

- Compilation intent is not a backend implementation.
- A compilation profile is not a physical device.
- A target declaration is not hardware discovery.
- A lowering declaration is not a lowering algorithm.
- A resource requirement is not an allocation.
- A preference is not a mandatory requirement.
- A hint cannot override semantic correctness.

4.4 Domain grammar integration

The compiler MUST remain compatible with the applicable domain specifications.

Domain| Relevant grammar authority
Classical| "grammar/spec/classical.md"
Quantum| "grammar/spec/quantum.md"
Hybrid| "grammar/spec/hybrid.md" and "grammar/hybrid/"
HDL| "grammar/spec/hdl.md" and "grammar/hdl/"
AI| "grammar/spec/ai.md" and "grammar/ai/"
Distributed| "grammar/spec/distributed.md" and "grammar/distributed/"
Resources| "grammar/spec/resources.md"
Networking| "grammar/spec/networking.md"
Concurrency| "grammar/spec/concurrency.md"
Compilation| "grammar/specification/compilation-model.md"
Execution| "grammar/specification/execution-model.md"
Scalability| "grammar/specification/scalability-model.md"
Portability| "grammar/specification/poco-reaf.md"

The domain list is extensible.

It MUST NOT become a closed enumeration of every possible future computing domain.

4.5 Grammar conformance

Compiler integration tests MUST establish:

1. Grammar composition validity.
2. Lexer/parser agreement.
3. Source construct recognition.
4. Correct AST mapping.
5. Correct semantic mapping.
6. Correct diagnostics.
7. Correct compilation-intent interpretation.
8. Domain-specific compatibility.
9. Rejection of unsupported constructs.
10. Preservation of source locations.

A grammar production MUST NOT automatically be classified as an implemented compiler feature.

---

5. CURRENT COMPILER STRUCTURE AND REQUIRED INTEGRATION

The repository already contains a substantial compiler directory structure.

This contract extends that architecture.

It does not authorize replacing it with another architecture.

5.1 Existing compiler root

Canonical path:

"src/compiler/mod.rs"

Repository inspection identified this file as empty.

This is a critical integration blocker.

The root MUST become the canonical public composition boundary.

It MUST:

- Declare existing compiler submodules.
- Expose the stable compiler API.
- Preserve required compatibility exports.
- Integrate the existing configuration subsystem.
- Integrate the existing context subsystem.
- Integrate the existing pipeline.
- Integrate compiler analysis.
- Integrate IR adapters.
- Integrate passes.
- Integrate optimization.
- Integrate planning.
- Integrate lowering.
- Integrate backend invocation.
- Integrate artifacts.
- Integrate linking.
- Integrate incremental compilation.
- Integrate parallel compilation.
- Integrate diagnostics.
- Integrate security.
- Integrate profiling.
- Integrate testing.
- Expose version information.

The root MUST NOT contain the implementation of every subsystem.

It MUST remain a composition and compatibility facade.

Root integration requirements

The root MUST establish one canonical compilation entry point.

All legacy public compilation entry points MUST delegate to that implementation or to an explicitly documented compatibility adapter.

No public entry point may bypass mandatory semantic validation, IR verification, security checks or required artifact validation.

The root MUST NOT select an arbitrary concrete physical target as the universal default.

The default compilation behavior MUST preserve target independence where the requested output permits it.

A concrete target is introduced through an explicit realization request or deployment policy.

Completion condition: The root is complete when all required modules are declared, public APIs compile, compatibility behavior is tested, and the actual production pipeline is reachable through the canonical entry points.

5.2 Existing directory architecture

The current architecture contains the following major subsystem directories.

src/compiler/
│
├── mod.rs
├── api/
├── configuration/
├── context/
├── diagnostics/
├── analysis/
├── pipeline/
├── ir/
├── passes/
├── optimization/
├── planning/
├── lowering/
├── backends/
├── artifacts/
├── linking/
├── incremental/
├── parallel/
├── security/
├── profiling/
├── testing/
└── documentation/

These are existing architecture areas, not instructions to create duplicate directories.

Each directory MUST retain one clearly defined ownership boundary.

---

6. COMPILER API INTEGRATION

Canonical directory:

"src/compiler/api/"

Existing files:

api/
├── mod.rs
├── compiler.rs
├── request.rs
├── response.rs
├── session.rs
└── compatibility.rs

6.1 request.rs

Owns the immutable compilation request.

It MUST support:

- Source text.
- Source-file input.
- Logical source identity.
- Compiler configuration.
- Compilation intent.
- Applicable target-independent options.
- Optional realization requirements.

It MUST NOT own AST, IR, semantic types or backend implementations.

Integration:

User/API
   |
   v
api/request.rs
   |
   v
api/compiler.rs
   |
   v
api/session.rs
   |
   v
pipeline/orchestration.rs

6.2 response.rs

Owns the public compilation response envelope.

It MUST expose:

- Compilation outcome.
- Structured diagnostics.
- Artifact references.
- Compilation metadata.
- Failure information.
- Applicable provenance references.

It MUST NOT invent another "CompilationOutput" or another diagnostic authority.

Existing compiler types and diagnostic definitions MUST be reused.

6.3 compiler.rs

Owns the compiler service interface.

It MUST delegate actual work to the pipeline.

It MUST support repeated compilation requests without leaking state between independent sessions.

It MUST preserve thread-safety guarantees where the implementation provides concurrent compilation.

6.4 session.rs

Owns one compilation session's API lifecycle.

It MUST integrate:

- Compilation context.
- Compilation state.
- Cancellation.
- Resource-budget accounting.
- Diagnostics.
- Provenance.
- Pipeline execution.

A compilation session MUST NOT silently continue after cancellation.

6.5 compatibility.rs

Owns compatibility between the new compiler API and existing callers.

It MUST preserve valid existing public entry points where required.

Compatibility adapters MUST NOT bypass new correctness requirements.

---

7. CONFIGURATION INTEGRATION

Canonical directory:

"src/compiler/configuration/"

Existing architecture:

configuration/
├── mod.rs
├── config.rs
├── policy.rs
├── budget.rs
├── target.rs
└── validation.rs

7.1 config.rs

Owns compiler configuration composition.

It MUST integrate:

- Compilation policies.
- Operational budgets.
- Target-independent defaults.
- Optimization configuration.
- Reproducibility requirements.
- Security requirements.
- Incremental compilation configuration.
- Parallel compilation configuration.
- Artifact output configuration.

It MUST NOT define competing semantic resource requirements.

7.2 budget.rs

This is the canonical owner of compiler operational budgets.

It MUST support optional limits for:

- Source processing.
- IR processing.
- Memory accounting.
- Worker usage.
- Optimization work.
- Constant evaluation.
- Diagnostics.
- Artifact generation.
- Compilation duration.
- Extension-defined work dimensions.

Budgets MUST be configurable.

An absent budget MUST NOT be interpreted as infinite physical resources.

Budget exhaustion MUST produce structured errors.

Budget arithmetic MUST use checked operations.

Counter overflow MUST NOT silently wrap.

7.3 target.rs

Owns target descriptors and their structural validation.

It MUST support extensible target identities and metadata.

It MUST NOT own:

- Hardware discovery.
- Capability measurement.
- Resource allocation.
- Backend implementation.
- Runtime scheduling.
- Physical placement.
- Quantum routing.

The existing "TargetDescriptor" and "TargetId" contracts MUST be reused.

No new closed hardware inventory may be introduced.

7.4 policy.rs

Owns compiler-policy composition and evaluation interfaces.

Policies MUST be distinguishable from semantic requirements and preferences.

A policy MUST NOT silently change the meaning of a program.

7.5 validation.rs

Validates configuration before execution.

It MUST distinguish:

- Invalid configuration.
- Unsupported configuration.
- Conflicting options.
- Infeasible requested realization.
- Operational budget incompatibility.
- Security-policy rejection.

---

8. CONTEXT INTEGRATION

Canonical directory:

"src/compiler/context/"

Existing files:

context/
├── mod.rs
├── compilation_context.rs
├── compilation_state.rs
├── dependency_context.rs
└── provenance_context.rs

8.1 compilation_context.rs

Owns shared immutable compilation information and access to shared operational services.

It MUST integrate:

- Compiler configuration.
- Source identity.
- Budget tracking.
- Cancellation.
- Context derivation.
- Thread-safe sharing.

It MUST NOT become a semantic model.

8.2 compilation_state.rs

Owns compilation lifecycle tracking.

It MUST enforce legal state transitions.

It MUST distinguish:

- Created.
- Initialized.
- Running.
- Waiting.
- Cancelled.
- Failed.
- Completed.

The exact state model MUST remain extensible where additional lifecycle states are justified.

A failed or cancelled compilation MUST NOT become successful through an invalid state transition.

8.3 dependency_context.rs

Owns compilation-unit dependency context.

It MUST integrate canonical dependency identities and dependency relationships.

It MUST support transitive dependency analysis.

It MUST NOT duplicate the canonical dependency graph where an existing authoritative graph can be reused.

8.4 provenance_context.rs

Owns compilation-session provenance context.

It MUST preserve:

- Source identity.
- Transformation lineage.
- Relevant analysis evidence.
- Verification results.
- Artifact relationships.
- Compilation decisions.

Provenance MUST NOT be silently discarded during transformations.

---

9. PIPELINE INTEGRATION

Canonical directory:

"src/compiler/pipeline/"

Existing files:

pipeline/
├── mod.rs
├── stage.rs
├── stages.rs
├── ordering.rs
├── execution.rs
├── failure.rs
├── recovery.rs
└── orchestration.rs

This is an existing implementation and MUST remain authoritative for compiler pipeline orchestration.

9.1 stage.rs

Owns the canonical stage contract.

Every stage MUST define:

- Stable identifier.
- Descriptor.
- Input contract.
- Output contract.
- Dependencies.
- Execution behavior.
- Determinism declaration.
- Concurrency declaration.
- Failure behavior.
- Cancellation behavior.
- Budget obligations.
- Verification obligations.
- Provenance obligations.

A stage MUST NOT silently consume an incompatible input representation.

9.2 stages.rs

Owns standard stage definitions.

It MUST distinguish:

- Required stages.
- Conditional stages.
- Policy-controlled stages.
- Optional stages.

Domain-specific extensions MUST use the established stage contracts.

9.3 ordering.rs

Owns stage dependency validation and deterministic ordering.

It MUST detect:

- Missing dependencies.
- Duplicate identifiers.
- Cycles.
- Invalid stage relationships.
- Dependency inconsistencies.

Ordering MUST NOT depend on arbitrary map iteration order.

9.4 execution.rs

Owns stage execution.

It MUST:

- Supply valid inputs.
- Preserve stage outputs.
- Record execution outcomes.
- Check cancellation.
- Account for budgets.
- Propagate errors.
- Preserve deterministic behavior where required.

It MUST NOT report a skipped required stage as completed.

9.5 failure.rs

Owns structured pipeline failure representation.

Failure information MUST identify:

- Origin.
- Stage.
- Category.
- Severity.
- Relevant diagnostics.
- Retryability.
- Partial outputs.
- Failure relationships.

9.6 recovery.rs

Owns recovery-policy evaluation.

A retry MUST require replay-safety evidence.

A fallback MUST be verified against the original semantic requirements.

Cancellation MUST NOT be overridden.

A fallback that changes program meaning MUST NOT be reported as successful.

9.7 orchestration.rs

Owns the lifecycle of one pipeline invocation.

It MUST integrate the existing stage, ordering, execution, failure, recovery and context subsystems.

It MUST NOT introduce another pipeline engine.

9.8 Mandatory compilation success conditions

A successful compilation MUST establish:

1. Required source processing completed.
2. Required semantic validation succeeded.
3. Required ownership and effect checks succeeded.
4. Required IR construction completed.
5. Required IR verification succeeded.
6. Required transformations completed.
7. Required feasibility checks succeeded.
8. Required lowering completed.
9. Backend realization succeeded when requested.
10. Required artifact validation succeeded.
11. Required linking succeeded when requested.
12. Compilation lifecycle reached a successful terminal state.
13. No unresolved fatal failure remains.
14. Required output artifacts exist and are valid.

A target-independent portable compilation may legitimately finish without generating a target-specific executable.

The result MUST identify which type of compilation was completed.

---

10. ANALYSIS INTEGRATION

Canonical directory:

"src/compiler/analysis/"

The compiler analysis subsystem MUST provide facts needed by compiler transformations and planning without becoming a competing semantic authority.

The existing architecture includes:

analysis/
├── mod.rs
├── program.rs
├── dependency.rs
├── effects.rs
├── ownership.rs
├── capabilities.rs
├── requirements.rs
└── feasibility.rs

Additional analysis files MAY be introduced when a genuinely independent responsibility requires them.

10.1 program.rs

Coordinates program-level compiler analysis.

It MUST consume the validated semantic model and canonical IR.

10.2 dependency.rs

Coordinates dependency analysis.

It MUST reuse established dependency identities and graph ownership.

10.3 effects.rs

Consumes canonical effect information.

It MUST preserve effect ordering and observable behavior.

10.4 ownership.rs

Consumes canonical ownership and borrowing facts.

It MUST NOT replace the semantic borrow checker.

Compiler-specific aliasing or transformation analyses are permitted when their scope is explicit.

10.5 capabilities.rs

Consumes canonical capability requirements and target capability evidence.

It MUST NOT assume a particular processor family.

10.6 requirements.rs

Consumes canonical resource and capability requirements.

The following concepts MUST remain distinct:

Concept| Meaning
Requirement| Mandatory condition for valid realization
Capability| Supported operation or property
Constraint| Restriction on permitted realizations
Preference| Desirable realization characteristic
Hint| Optional optimization guidance
Feasibility| Whether a realization satisfies applicable conditions

10.7 feasibility.rs

Determines whether a proposed realization is admissible.

It MUST consider:

- Program semantics.
- Required capabilities.
- Resource requirements.
- Target compatibility.
- Constraints.
- Policies.
- Available resource evidence.
- Implementation support.

It MUST distinguish unknown feasibility from proven feasibility.

It MUST explicitly report infeasibility.

---

11. IR INTEGRATION

Canonical directory:

"src/compiler/ir/"

Existing files:

ir/
├── mod.rs
├── input.rs
├── adapters.rs
├── compatibility.rs
├── verification.rs
├── translation.rs
└── lowering.rs

11.1 input.rs

Owns the compiler-facing IR input contract.

It MUST accept canonical representations and their relevant metadata.

It MUST NOT manufacture new semantic IR entities.

11.2 adapters.rs

Coordinates domain-owned representation adapters.

The existing implementation identifies ZCIR/ZUIR integration through:

"crate::classical::zcir::lowering"

This existing conversion boundary MUST remain authoritative.

The current adapter MUST NOT claim unsupported conversions are implemented.

In particular, the following conversions require explicit domain-owned implementations and verification before being registered:

- Quantum IR conversions.
- HDLIR conversions.
- AI IR conversions.
- Tensor representation conversions.
- Other future domain conversions.

A conversion MUST declare:

1. Source representation.
2. Destination representation.
3. Version compatibility.
4. Semantic mapping.
5. Type mapping.
6. Effect mapping.
7. Ownership mapping.
8. Resource mapping.
9. Information preservation.
10. Information loss, if any.
11. Preconditions.
12. Postconditions.
13. Verification requirements.
14. Failure behavior.
15. Provenance.

An unsupported conversion MUST fail explicitly.

11.3 compatibility.rs

Coordinates legacy IR compatibility.

The canonical legacy integration includes:

"src/classical/zcir/lowering/legacy_adapter.rs"

Legacy representations MUST NOT be silently treated as canonical ZUIR.

Compatibility MUST be explicit and verifiable.

11.4 verification.rs

Coordinates canonical IR verification.

It MUST invoke the relevant authoritative verifiers.

It MUST verify IR before and after transformations whenever required by the transformation contract.

11.5 translation.rs

Coordinates representation translation.

It MUST delegate actual conversion to domain-owned providers.

It MUST NOT infer semantic equivalence from structural similarity.

11.6 lowering.rs

Provides the compiler-facing lowering boundary.

The canonical ZUIR lowering authority is:

"src/zuir/lowering/"

Relevant interfaces include:

- "src/zuir/lowering/api.rs"
- "src/zuir/lowering/context.rs"
- "src/zuir/lowering/legality.rs"
- "src/zuir/lowering/diagnostics.rs"
- "src/zuir/lowering/provenance.rs"

The compiler MUST delegate to these existing interfaces.

It MUST NOT implement a competing ZUIR lowering engine.

11.7 IR integration completion

The IR integration is complete only when:

- Every supported representation is explicitly identified.
- Every conversion has a real implementation.
- Every conversion has a verification contract.
- Unsupported conversions are rejected.
- Semantic information is preserved.
- Source provenance is preserved.
- No duplicate IR authority exists.
- All supported domain paths have integration tests.

---

12. PASS MANAGEMENT

Canonical directory:

"src/compiler/passes/"

Existing files include:

passes/
├── mod.rs
├── registry.rs
├── manager.rs
├── pipeline.rs
├── ordering.rs
├── legality.rs
└── preservation.rs

12.1 registry.rs

Owns compiler-facing pass registration.

It MUST support extensible pass providers.

It MUST NOT duplicate domain-owned pass registries unnecessarily.

12.2 manager.rs

Coordinates pass execution.

It MUST validate pass inputs and outputs.

12.3 pipeline.rs

Defines pass sequences.

It MUST support conditional and domain-specific transformations.

12.4 ordering.rs

Validates pass dependencies.

12.5 legality.rs

Determines whether a transformation is permitted.

It MUST consider semantic and representation invariants.

12.6 preservation.rs

Tracks preserved and invalidated analysis information.

A transformation MUST invalidate every analysis whose assumptions it changes.

Stale analysis results MUST NOT be reused.

12.7 Mandatory transformation contract

Every production transformation MUST declare:

1. Input representation.
2. Output representation.
3. Preconditions.
4. Postconditions.
5. Semantic-preservation obligations.
6. Required analyses.
7. Invalidated analyses.
8. Effects.
9. Resource requirements.
10. Determinism guarantees.
11. Failure behavior.
12. Verification strategy.
13. Provenance requirements.
14. Compatibility implications.

No transformation may silently change observable program meaning.

---

13. OPTIMIZATION INTEGRATION

Canonical directory:

"src/compiler/optimization/"

Existing architecture includes:

optimization/
├── mod.rs
└── policies.rs

The compiler MUST integrate with the existing:

"src/optimizer/"

It MUST NOT introduce a competing general-purpose optimizer.

13.1 policies.rs

Owns compiler-facing optimization policy integration.

It MUST distinguish:

- Optimization objectives.
- Mandatory semantic constraints.
- Numerical guarantees.
- Resource budgets.
- Performance preferences.
- Implementation hints.

13.2 Required optimization capabilities

The integrated optimizer must support, as applicable:

- Classical optimization.
- Dataflow optimization.
- Control-flow optimization.
- Memory optimization.
- Loop transformations.
- Parallelization.
- Vectorization.
- Tensor optimization.
- AI graph optimization.
- Quantum optimization.
- Quantum circuit optimization.
- HDL optimization.
- Hardware-aware optimization.
- Distributed optimization.
- Communication optimization.
- Energy-aware optimization.
- Security-preserving optimization.
- Cross-domain optimization.

These are integration requirements, not claims that all algorithms already exist.

13.3 Optimization correctness

Every optimization MUST preserve applicable:

- Types.
- Effects.
- Ownership.
- Observable behavior.
- Numerical contracts.
- Memory semantics.
- Synchronization.
- Communication semantics.
- Quantum semantics.
- Hardware-description semantics.
- Security constraints.

Optimization profitability MUST NOT override correctness.

---

14. HETEROGENEOUS PLANNING

Canonical directory:

"src/compiler/planning/"

Existing architecture includes:

planning/
├── mod.rs
├── target_discovery.rs
├── capability_matching.rs
├── resource_planning.rs
├── partitioning.rs
├── scheduling.rs
├── routing.rs
└── feasibility.rs

The exact module inventory and declarations MUST be kept consistent with the actual source tree.

14.1 target_discovery.rs

Coordinates discovery of registered target providers.

It MUST NOT maintain a hardcoded inventory of every possible processor.

It MUST distinguish registered targets from physically available devices.

14.2 capability_matching.rs

Matches validated program requirements against target capability evidence.

Unknown capabilities MUST NOT be treated as supported.

14.3 resource_planning.rs

Produces abstract resource plans.

It MUST NOT allocate physical resources.

14.4 partitioning.rs

Coordinates heterogeneous workload partitioning.

It MUST preserve:

- Data dependencies.
- Control dependencies.
- Cross-domain effects.
- Ownership.
- Synchronization.
- Communication.
- Memory consistency.
- Numerical guarantees.
- Failure semantics.

14.5 scheduling.rs

Produces abstract scheduling plans.

Physical execution scheduling remains with the appropriate execution subsystem.

14.6 routing.rs

Coordinates abstract routing requirements.

Physical routing remains with the authoritative routing or hardware subsystem.

14.7 feasibility.rs

Validates the complete proposed realization.

It MUST produce structured explanations for infeasibility.

14.8 Required heterogeneous planning model

The compiler must support:

                 VALIDATED PROGRAM
                         |
                         v
                 WORKLOAD ANALYSIS
                         |
                         v
               OPERATION REQUIREMENTS
                         |
                         v
                CAPABILITY MATCHING
                         |
                         v
                RESOURCE FEASIBILITY
                         |
                         v
              HETEROGENEOUS PARTITIONING
                         |
                         v
                DEPENDENCY ANALYSIS
                         |
                         v
                DATA-MOVEMENT PLANNING
                         |
                         v
               ABSTRACT SCHEDULING
                         |
                         v
                  LOWERING PLAN

The planning system MUST support arbitrary domain compositions.

It MUST NOT assume that all programs have one CPU, one GPU, one QPU, one FPGA or one execution device.

It MUST NOT impose universal limits on processing units, accelerators, nodes, memory or domain count.

---

15. LOWERING INTEGRATION

Canonical directory:

"src/compiler/lowering/"

Existing architecture includes:

lowering/
├── mod.rs
├── legalization.rs
├── conversion.rs
├── decomposition.rs
├── outlining.rs
└── target_lowering.rs

15.1 legalization.rs

Coordinates representation legality and legalization.

It MUST reject illegal transformations.

15.2 conversion.rs

Coordinates conversion providers.

It MUST delegate to authoritative representation owners.

15.3 decomposition.rs

Coordinates domain-owned decomposition.

Examples include quantum gate decomposition and other domain-specific lowering strategies.

It MUST NOT implement a second domain-specific decomposition authority.

15.4 outlining.rs

Coordinates outlining of eligible computations.

It MUST preserve dependencies, captures, ownership, effects and execution contracts.

15.5 target_lowering.rs

Coordinates target-specific lowering admission and delegation.

It MUST validate:

- Target identity.
- Backend compatibility.
- Input representation.
- Required capabilities.
- Lowering legality.
- Required verification.
- Artifact expectations.

Target lowering MUST NOT silently select an incompatible backend.

---

16. BACKEND INTEGRATION

Canonical directory:

"src/compiler/backends/"

Existing architecture includes:

backends/
├── mod.rs
├── registry.rs
├── discovery.rs
├── selection.rs
├── capabilities.rs
├── compatibility.rs
└── invocation.rs

The canonical backend owner is:

"src/backend/"

16.1 registry.rs

The compiler-facing registry MUST delegate to:

"crate::backend::registry"

The compiler MUST NOT create a second backend registry.

16.2 discovery.rs

Coordinates backend-provider discovery.

It MUST distinguish:

- Installed providers.
- Registered providers.
- Available providers.
- Compatible providers.
- Providers that have passed validation.

16.3 selection.rs

Coordinates selection of compatible backend providers.

Selection MUST consider:

- Validated program semantics.
- Required capabilities.
- Target descriptors.
- Constraints.
- Policies.
- Artifact compatibility.
- Resource feasibility.

A preference may rank feasible alternatives.

It MUST NOT turn an infeasible target into a feasible one.

16.4 capabilities.rs

Exposes backend capability contracts.

Capability claims MUST be supported by provider evidence.

16.5 compatibility.rs

Validates:

- IR compatibility.
- Backend API compatibility.
- Target compatibility.
- ABI compatibility.
- Artifact-format compatibility.
- Version compatibility.

16.6 invocation.rs

Invokes the canonical backend interfaces.

It MUST:

- Supply verified inputs.
- Pass appropriate configuration.
- Propagate errors.
- Capture diagnostics.
- Capture output metadata.
- Validate returned artifacts.
- Preserve provenance.

It MUST NOT treat a successful backend function return as sufficient proof that the artifact is valid.

16.7 Future backend extensibility

New backend providers MUST be registerable without modifying semantic authorities or creating new universal target enums.

The architecture MUST accommodate future computing architectures.

Examples include:

- New CPU architectures.
- GPUs.
- TPUs.
- FPGAs.
- ASICs.
- Quantum processors.
- Neuromorphic processors.
- Photonic processors.
- Distributed accelerators.
- Future execution substrates.

These examples are not a closed inventory.

---

17. ARTIFACT INTEGRATION

Canonical directory:

"src/compiler/artifacts/"

Existing architecture includes:

artifacts/
├── mod.rs
├── artifact.rs
├── metadata.rs
├── manifest.rs
└── validation.rs

The artifact subsystem MUST support portable and realized outputs.

17.1 artifact.rs

Owns the common compiler artifact abstraction.

It MUST distinguish artifact identity, representation, format and content.

17.2 metadata.rs

Owns artifact metadata contracts.

Metadata MUST support:

- Artifact identity.
- Artifact version.
- Producer information.
- Source provenance.
- IR representation.
- Domain information.
- Required capabilities.
- Compatibility information.
- Dependencies.
- Verification evidence.

17.3 manifest.rs

Owns artifact manifest composition.

It MUST describe the artifacts and their relationships.

A manifest MUST NOT claim an artifact exists when it has not been generated.

17.4 validation.rs

Validates:

- Artifact integrity.
- Artifact format.
- Artifact version.
- Required metadata.
- Dependency consistency.
- Compatibility.
- Provenance.
- Verification evidence.
- Security requirements.

17.5 Portable artifact

A portable artifact may contain:

- Verified target-independent IR.
- Domain representations.
- Semantic contracts.
- Effect information.
- Capability requirements.
- Resource requirements.
- Dependencies.
- Provenance.
- Compatibility metadata.

A portable artifact is not necessarily directly executable.

17.6 Realized artifact

A realized artifact may contain:

- Native executable.
- Native library.
- WebAssembly module.
- GPU binary.
- FPGA bitstream.
- ASIC implementation package.
- Quantum execution package.
- Hybrid deployment bundle.
- Distributed deployment package.
- Other registered artifact types.

A generated manifest is not proof of a working executable.

A production artifact MUST be validated using the appropriate format and provider-specific validation.

---

18. LINKING INTEGRATION

Canonical directory:

"src/compiler/linking/"

Existing architecture includes:

linking/
├── mod.rs
├── symbols.rs
├── relocation.rs
├── composition.rs
└── validation.rs

18.1 symbols.rs

Coordinates symbol identity and resolution.

It MUST preserve the canonical symbol semantics and avoid collisions.

18.2 relocation.rs

Coordinates relocation requirements.

It MUST validate relocation compatibility with the destination artifact and target.

18.3 composition.rs

Coordinates composition of compatible artifacts.

It MUST preserve dependencies and domain boundaries.

18.4 validation.rs

Validates the linked result.

It MUST reject:

- Unresolved required symbols.
- Incompatible ABI contracts.
- Invalid relocations.
- Missing dependencies.
- Incompatible artifact versions.
- Invalid cross-domain interfaces.

Link-time optimization MUST integrate with the canonical pass-management and optimizer systems.

It MUST NOT introduce a competing optimizer.

---

19. INCREMENTAL COMPILATION INTEGRATION

Canonical directory:

"src/compiler/incremental/"

Existing architecture includes:

incremental/
├── mod.rs
├── cache.rs
├── fingerprints.rs
└── invalidation.rs

Incremental compilation MUST be sound.

A cache hit is valid only when every relevant input and compatibility dependency remains valid.

Fingerprints MUST account for relevant:

- Source content.
- Grammar version.
- Frontend version.
- Semantic model version.
- IR version.
- Pass versions.
- Pass configuration.
- Target descriptors, where applicable.
- Backend versions, where applicable.
- Artifact format.
- Compilation policies.
- Relevant dependencies.
- Relevant toolchain versions.

The compiler MUST NOT reuse an artifact whose assumptions have become invalid.

Canonical dependency and fingerprint authorities MUST be reused where available.

Existing integrations include:

- "src/zuir/incremental/"
- "src/zuir/fingerprint/"
- "src/backend/cache.rs"

The compiler incremental subsystem coordinates these authorities.

It MUST NOT duplicate their underlying algorithms unnecessarily.

Cache invalidation MUST be transitive where dependency semantics require it.

A failed compilation MUST NOT publish incomplete or invalid artifacts as successful cache entries.

---

20. PARALLEL COMPILATION INTEGRATION

Canonical directory:

"src/compiler/parallel/"

Existing architecture includes:

parallel/
├── mod.rs
├── scheduler.rs
├── workers.rs
├── dependencies.rs
└── determinism.rs

Parallel compilation MUST be resource-configurable.

It MUST NOT impose arbitrary universal worker or processor ceilings.

The parallel subsystem MUST:

- Respect dependency ordering.
- Respect shared compilation budgets.
- Support cancellation.
- Propagate failures.
- Avoid unsafe shared-state assumptions.
- Preserve artifact consistency.
- Support deterministic result collection.
- Avoid data races through safe ownership and synchronization.
- Avoid deadlocks and uncontrolled resource growth.

Parallel execution MUST NOT change program meaning.

Parallel and sequential compilation MUST produce equivalent required outputs under equivalent deterministic configurations.

A configurable worker budget is permitted.

A fixed maximum number of workers as a universal architecture restriction is not.

---

21. SECURITY INTEGRATION

Canonical directory:

"src/compiler/security/"

Existing architecture includes:

security/
├── mod.rs
├── policy.rs
├── verification.rs
└── audit.rs

The compiler security subsystem MUST integrate with existing security authorities.

It MUST consider:

- Source and dependency trust.
- Semantic security properties.
- IR operation legality.
- Memory safety.
- Capability authorization.
- Data-flow constraints.
- Backend trust.
- Artifact integrity.
- Supply-chain provenance.
- External tool invocation.
- Serialization and deserialization.
- Deployment compatibility.

Security verification MUST cover the actual representations and artifacts that require verification.

A function-name inspection or a source-level scan MUST NOT be represented as complete IR or artifact security verification.

Untrusted artifacts MUST NOT be accepted merely because their manifest is structurally valid.

External compiler tools MUST be invoked through controlled, validated interfaces.

No security failure may be silently ignored.

The security subsystem MUST NOT duplicate cryptographic primitive implementations or runtime authorization systems.

---

22. DIAGNOSTICS INTEGRATION

Canonical directory:

"src/compiler/diagnostics/"

Existing architecture includes:

diagnostics/
├── mod.rs
├── reporting.rs
└── rendering.rs

The compiler MUST integrate diagnostics from:

- Lexer.
- Parser.
- AST construction.
- Semantic analysis.
- Ownership analysis.
- Type checking.
- Effect analysis.
- IR construction.
- IR verification.
- Pass management.
- Optimization.
- Capability analysis.
- Feasibility planning.
- Lowering.
- Backend invocation.
- Linking.
- Artifact validation.
- Security.
- Runtime compatibility.

Diagnostics MUST retain source mapping where applicable.

Each diagnostic SHOULD identify:

- Stable code.
- Severity.
- Message.
- Source location.
- Relevant labels.
- Related diagnostics.
- Notes.
- Suggestions.
- Responsible subsystem.
- Compilation stage.

Diagnostics MUST distinguish language errors from target-realization errors and operational resource failures.

Diagnostic collection MUST NOT silently truncate information because of an arbitrary universal count limit.

---

23. PROFILING AND OBSERVABILITY

Canonical directory:

"src/compiler/profiling/"

Existing architecture includes:

profiling/
├── mod.rs
├── events.rs
└── measurements.rs

Profiling MUST capture relevant compilation information.

Examples:

- Stage execution duration.
- Pass execution.
- IR transformation counts.
- Backend invocation.
- Resource-budget consumption.
- Cache hits and misses.
- Compilation failures.
- Verification results.
- Planning decisions.

Compiler measurements MUST be distinguished from runtime measurements.

Runtime measurements MAY inform later optimization decisions, but they MUST NOT override correctness requirements.

Profiling MUST be optional where its cost is unnecessary.

Its overhead MUST be measurable.

---

24. HYBRID COMPUTING INTEGRATION

Hybrid integration is one of the central requirements of POCO-REAF.

The compiler MUST integrate with:

- "grammar/spec/hybrid.md"
- "grammar/hybrid/"
- "src/hybrid/"
- "src/quantum/ir/"
- "src/classical/"
- "src/ai/"
- "src/hdl/"
- "src/distributed/"
- "src/runtime/"

The compiler MUST support arbitrary valid combinations of computational domains.

24.1 Hybrid semantics

The compiler MUST preserve:

- Domain identity.
- Domain-crossing dependencies.
- Data dependencies.
- Control dependencies.
- Ownership.
- Effects.
- Measurement dependencies.
- Synchronization.
- Data movement.
- Memory consistency.
- Timing contracts.
- Cancellation.
- Failure behavior.
- Transactional guarantees where applicable.

24.2 Classical-quantum integration

Quantum computation MUST use:

"src/quantum/ir/"

Classical computation MUST use the canonical classical representations.

The compiler MUST NOT introduce:

- Another quantum IR.
- Another quantum circuit model.
- Another classical IR.
- Another qubit identity system.

Quantum-to-classical and classical-to-quantum transitions MUST preserve their semantic contracts.

Measurement-driven feedback MUST retain its dependencies.

24.3 AI and tensor integration

AI operations MUST integrate with:

"src/ai/"

Tensor semantics MUST use the canonical tensor subsystem.

The compiler MUST support symbolic and dynamic dimensions where the domain implementation permits them.

AI optimization MUST preserve model and numerical contracts.

24.4 HDL integration

HDL computation MUST integrate with:

"src/hdl/hdlir/"

Hardware descriptions MUST retain hardware semantics.

The compiler MUST NOT reduce an HDL design to ordinary CPU instructions.

Simulation and synthesis MUST remain distinguishable.

24.5 Distributed integration

Distributed computation MUST integrate with:

"src/distributed/"

The compiler may analyze partitioning, communication and placement requirements.

Actual node allocation, communication execution and distributed resource management remain downstream.

24.6 Synchronization

Cross-domain synchronization MUST be explicit in the validated semantic and IR models.

The compiler MUST NOT invent synchronization behavior merely to make an execution plan appear feasible.

---

25. REQUIREMENTS, CAPABILITIES AND RESOURCE NEGOTIATION

The compiler MUST distinguish program meaning from implementation availability.

The negotiation process is:

Validated program
       |
       v
Extract requirements
       |
       v
Discover target providers
       |
       v
Obtain capability evidence
       |
       v
Check mandatory requirements
       |
       v
Evaluate constraints
       |
       v
Apply authorization policies
       |
       v
Evaluate resource feasibility
       |
       v
Rank valid alternatives
       |
       v
Select a valid realization
       |
       v
Record decision provenance

The following rules are mandatory.

1. Requirements MUST be satisfied.
2. Constraints MUST be respected.
3. Policies MUST authorize the realization.
4. Capabilities MUST be supported by evidence.
5. Preferences MAY rank valid alternatives.
6. Hints MAY influence optimization.
7. Infeasible alternatives MUST NOT be selected.
8. Unknown information MUST NOT be treated as proof of support.
9. Decisions MUST be recorded.
10. Assumptions that can change MUST be revalidated at the appropriate later boundary.

The compiler MUST distinguish:

- Unsupported computation.
- Missing capability.
- Insufficient resources.
- Incompatible backend.
- Invalid constraint.
- Security rejection.
- Unknown feasibility.
- Implementation limitation.

---

26. SCALABILITY AND HARD-CODING POLICY

This section is mandatory for every compiler file.

26.1 Forbidden universal ceilings

The compiler MUST NOT introduce universal maximums for:

- CPUs.
- CPU cores.
- Threads.
- Workers.
- GPUs.
- TPUs.
- FPGAs.
- ASICs.
- QPUs.
- Qubits.
- Logical qubits.
- Distributed nodes.
- Devices.
- Accelerators.
- Memory.
- Storage.
- Register width.
- Tensor rank.
- Tensor dimensions.
- Collection size.
- Graph size.
- Functions.
- Modules.
- Operations.
- Hardware topology.
- Domain combinations.
- Artifact count.

Examples of prohibited assumptions include:

const MAX_CPUS: usize = 64;
const MAX_GPUS: usize = 16;
const MAX_QUBITS: usize = 1024;
const MAX_THREADS: usize = 256;
const MAX_MEMORY: usize = 64 * 1024;
const MAX_TENSOR_RANK: usize = 16;

These must not be introduced as universal architectural restrictions.

26.2 Configurable operational budgets

The following are permitted:

pub struct CompilationBudget {
    pub memory_budget: Option<u64>,
    pub worker_budget: Option<usize>,
    pub artifact_budget: Option<u64>,
    pub time_budget: Option<std::time::Duration>,
}

This is illustrative.

The implementation MUST reuse the existing budget authority rather than introducing another budget type.

A budget represents an operational policy.

It MUST NOT redefine the language's semantic capacity.

26.3 Representation limits

Rust and external formats have finite representation limits.

The compiler MUST handle these honestly.

It MUST distinguish:

- Language-level restrictions.
- Semantic restrictions.
- Representation restrictions.
- Implementation restrictions.
- Operational budgets.
- Physical resource limits.

For example, a host address space may not represent a requested allocation.

That is an implementation or resource failure, not a reason to impose a smaller universal language limit.

26.4 Integer safety

All relevant arithmetic MUST use checked operations where overflow is possible.

The compiler MUST NOT:

- Silently wrap resource counters.
- Truncate operation counts.
- Overflow artifact sizes.
- Convert invalid dimensions into valid ones.
- Silently saturate values where exact accounting is required.

26.5 Memory and streaming

Large compilation workloads SHOULD support:

- Incremental processing.
- Streaming.
- Bounded working sets.
- Reusable immutable analysis.
- Memory-aware scheduling.
- Incremental artifact production where valid.
- Explicit resource accounting.

Streaming MUST preserve ordering and semantic correctness.

26.6 No false infinity claims

Unlimited configured budgets do not guarantee infinite execution capacity.

The compiler must report actual exhaustion.

---

27. PORTABILITY AND POCO-REAF

POCO-REAF is the central portability requirement.

It does not mean every physical machine can execute every program.

It means hardware changes alone should not require developers to rewrite a program whose declared semantics remain realizable.

27.1 Program Once

The language defines one source-level program meaning.

The compiler MUST NOT require separate source programs merely because hardware differs.

27.2 Compile Once

The compiler MUST support producing a portable compilation artifact where the requested compilation mode permits it.

That artifact MUST contain sufficient information for later compatible realization.

27.3 Run Everywhere

The same portable artifact SHOULD be realizable on compatible targets.

The compiler MUST validate capabilities, requirements and constraints.

27.4 Run Anywhere

The architecture MUST support different execution environments, including:

- Embedded devices.
- Edge systems.
- Workstations.
- Servers.
- HPC systems.
- Clusters.
- Cloud environments.
- Accelerators.
- Quantum systems.
- Hybrid systems.
- Future environments.

27.5 Run Forever

Long-term compatibility MUST be supported through:

- Versioned language semantics.
- Versioned IR.
- Versioned artifact formats.
- Versioned backend interfaces.
- Explicit compatibility rules.
- Migration mechanisms.
- Provenance.
- Reproducible compilation where required.

Forever does not mean that software can never require maintenance or compatibility migration.

---

28. DETERMINISM AND REPRODUCIBILITY

The compiler MUST distinguish deterministic compilation from nondeterministic program execution.

Where deterministic compilation is required, equivalent inputs and equivalent configurations MUST produce equivalent required outputs.

Determinism MUST cover:

- Stage ordering.
- Pass ordering.
- Symbol resolution.
- Artifact composition.
- Serialization.
- Provenance ordering where applicable.
- Parallel result collection.
- Cache fingerprints.

Nondeterministic algorithms MUST declare their behavior.

Randomness MUST NOT be introduced silently into compilation decisions.

A reproducible build MUST identify relevant toolchain and provider versions.

---

29. ERROR AND FAILURE CONTRACT

The compiler MUST distinguish the following failure classes:

Failure| Meaning
Syntax failure| Source cannot be parsed
Semantic failure| Program meaning is invalid
Type failure| Type contracts are violated
Ownership failure| Ownership or borrowing is invalid
Effect failure| Effect constraints are violated
IR failure| Representation is invalid
Verification failure| Required verification failed
Optimization failure| A transformation cannot be performed
Capability failure| Required capability is unavailable
Feasibility failure| No valid realization satisfies the requirements
Resource failure| Available resources are insufficient
Lowering failure| Required conversion cannot be performed
Backend failure| Backend realization failed
Linking failure| Artifact composition failed
Artifact failure| Generated artifact is invalid
Security failure| Security policy or verification rejected the operation
Cancellation| Compilation was cancelled
Internal failure| Compiler invariant or implementation failed

Failures MUST preserve useful partial information where safe.

A failed compilation MUST NOT return a successful artifact status.

An infeasible realization MUST NOT be disguised as a successful compilation.

---

30. COMPATIBILITY AND MIGRATION

The compiler MUST preserve existing valid public APIs where required.

However, compatibility MUST NOT preserve incorrect behavior.

30.1 Existing compiler entry points

All existing public compilation entry points MUST be identified through repository-wide reference analysis.

They MUST be classified as:

- Canonical.
- Compatibility adapter.
- Deprecated.
- Internal.
- Removed.

No legacy entry point may bypass mandatory pipeline stages.

30.2 Legacy IR

Legacy IR MUST be handled through:

"src/classical/zcir/lowering/legacy_adapter.rs"

Legacy IR MUST NOT silently become canonical ZUIR.

30.3 Existing compiler modules

Previously discussed root-level modules such as:

- "hybrid_pipeline.rs"
- "hardware_partitioner.rs"
- "parallel_build.rs"
- "jit.rs"
- "safety_guard.rs"
- "ssbe.rs"

MUST NOT be blindly recreated from historical architecture descriptions.

Their existence, removal, relocation and current callers MUST be verified against the active branch before any migration.

The current architecture's directory-based implementations take precedence where they already own the responsibility.

A JIT, if provided, MUST be distinguished from an interpreter or IR execution engine.

A backend must not claim native JIT compilation unless it genuinely emits and executes native code.

---

31. FILE-LEVEL INTEGRATION REQUIREMENTS

Every existing and future production compiler file MUST have a complete implementation contract.

The contract MUST be established before implementation begins.

31.1 Mandatory file contract

Every file MUST document:

1. Canonical path.
2. Purpose.
3. Implementation status.
4. Architectural authority.
5. Responsibilities it owns.
6. Responsibilities it does not own.
7. Inputs.
8. Outputs.
9. Public API.
10. Dependencies.
11. Upstream integrations.
12. Downstream integrations.
13. Data ownership.
14. Concurrency behavior.
15. Error behavior.
16. Cancellation behavior.
17. Budget behavior.
18. Determinism guarantees.
19. Security requirements.
20. Provenance requirements.
21. Compatibility requirements.
22. Scalability behavior.
23. Hard-coding audit.
24. Unit tests.
25. Integration tests.
26. Acceptance criteria.
27. Freeze criteria.

31.2 No downstream-edit requirement

A file SHOULD be designed so its public contract does not need redesign merely because a downstream file is subsequently implemented.

Before implementation, establish:

- All required types.
- All required traits.
- All error categories.
- All integration interfaces.
- All ownership boundaries.
- All versioning obligations.
- All test obligations.

Dependent files MUST adapt to the frozen contract.

A frozen file may still require a justified defect correction or security correction.

Freeze does not excuse preserving an incorrect design.

31.3 Dependency order

Files MUST be implemented in dependency order.

The recommended order is:

1. Independent foundational contracts.
2. Configuration.
3. Context.
4. API.
5. Pipeline.
6. Analysis.
7. IR integration.
8. Pass management.
9. Optimization.
10. Planning.
11. Lowering.
12. Backend integration.
13. Artifacts.
14. Linking.
15. Incremental compilation.
16. Parallel compilation.
17. Security.
18. Profiling.
19. Compiler root integration.
20. Repository-wide conformance.

Existing files MUST be reused rather than recreated.

---

32. CROSS-REPOSITORY INTEGRATION MATRIX

The following table defines the required repository-wide integration boundaries.

Repository path| Compiler responsibility
"src/frontend/"| Consume canonical frontend outputs
"src/frontend/ast/"| Consume AST without redefining it
"src/frontend/parser/"| Consume parser outputs
"src/frontend/semantic/"| Consume validated semantic information where present
"src/semantic.rs"| Preserve explicit legacy compatibility until migration is resolved
"src/zuir/"| Canonical universal IR integration
"src/zuir/verify/"| IR verification
"src/zuir/lowering/"| Canonical lowering
"src/zuir/pass/"| Canonical ZUIR pass integration
"src/classical/"| Classical domain integration
"src/classical/zcir/"| Classical IR integration
"src/quantum/ir/"| Canonical quantum IR
"src/quantum/optimization/"| Quantum optimization
"src/quantum/scheduling/"| Quantum scheduling integration
"src/quantum/qec/"| Quantum error-correction integration
"src/quantum/zqn/"| Quantum noise/fault integration
"src/hdl/hdlir/"| Hardware IR
"src/ai/"| AI computation
Tensor subsystem| Tensor computation
"src/hybrid/"| Hybrid domain integration
"src/distributed/"| Distributed computation
"src/optimizer/"| Optimization algorithms
"src/backend/"| Backend registration and code generation
"src/runtime/"| Execution compatibility
"src/toolchain/"| Build and packaging integration
"src/source_map/"| Source mapping where available
"src/ir_verify.rs"| Legacy verification compatibility, subject to migration
"src/ir_gen.rs"| Legacy IR compatibility, not a competing canonical IR
"Cargo.toml"| Rust and dependency compatibility

The compiler MUST NOT make these subsystems depend on compiler orchestration merely to access their own foundational abstractions.

Dependency direction MUST remain acyclic at the architectural level.

---

33. REQUIRED PRODUCTION TESTING

Testing MUST establish actual behavior rather than merely prove that public types exist.

33.1 Unit tests

Every implementation file MUST test its own invariants and failure conditions.

33.2 Pipeline integration tests

Test:

- Required-stage execution.
- Conditional-stage execution.
- Invalid stage dependencies.
- Missing required stages.
- Stage failures.
- Cancellation.
- Recovery.
- Budget exhaustion.
- Successful completion.
- Partial results.

33.3 IR integration tests

Test:

- Canonical IR input.
- ZCIR/ZUIR integration.
- Quantum IR boundaries.
- HDLIR boundaries.
- AI integration.
- Unsupported conversions.
- Invalid IR.
- Transformation verification.
- Provenance preservation.

33.4 Hybrid integration tests

Test:

- Classical-only computation.
- Quantum-only computation.
- Classical-quantum computation.
- AI-quantum computation.
- HDL-classical computation.
- Distributed hybrid computation.
- Multi-domain dependency graphs.
- Measurement-driven feedback.
- Cross-domain synchronization.
- Data movement.
- Ownership.
- Failure propagation.

33.5 Backend tests

Test:

- Backend discovery.
- Registration.
- Duplicate registration rejection.
- Capability matching.
- Compatibility.
- Invocation.
- Actual artifact production.
- Artifact validation.
- Backend failure.
- Unsupported target rejection.

33.6 Portability tests

Compile the same source program for multiple compatible target providers.

Compare observable behavior against the program's semantic contract.

Comparing filenames or confirming that two compilation calls returned successfully is insufficient.

33.7 Scalability tests

Test:

- Small programs.
- Large source files.
- Large dependency graphs.
- Large IR modules.
- Many compilation units.
- Configurable worker budgets.
- Configurable memory budgets.
- Dynamic tensor dimensions.
- Symbolic resource quantities.
- Large artifact collections.
- Incremental builds.
- Parallel builds.
- Resource exhaustion.

Tests MUST NOT require arbitrary fixed machine capacities.

33.8 Security tests

Test:

- Invalid artifacts.
- Invalid manifests.
- Malformed serialized IR.
- Untrusted providers.
- Dependency integrity.
- Unauthorized operations.
- Invalid capability claims.
- External tool failures.
- Supply-chain verification.
- Cancellation during security verification.

33.9 Determinism tests

Equivalent deterministic inputs MUST produce equivalent required outputs.

Sequential and parallel execution MUST preserve equivalent results under equivalent conditions.

---

34. REQUIRED TEST FIXTURES

The compiler should use shared language fixtures.

Recommended location:

tests/compiler/
├── fixtures/
│   ├── minimal.zm
│   ├── classical.zm
│   ├── quantum.zm
│   ├── hybrid.zm
│   ├── hdl.zm
│   ├── ai.zm
│   ├── distributed.zm
│   ├── networking.zm
│   └── poco-reaf.zm
│
├── api/
├── pipeline/
├── semantic/
├── ir/
├── passes/
├── optimization/
├── planning/
├── lowering/
├── backends/
├── artifacts/
├── linking/
├── incremental/
├── parallel/
├── security/
├── portability/
├── hybrid/
└── scalability/

These are proposed test directories.

Existing test locations MUST be inspected before creating duplicates.

Each fixture MUST specify:

- Expected parsing result.
- Expected semantic result.
- Expected IR representation.
- Expected verification result.
- Expected compilation stages.
- Expected artifact type.
- Expected failure behavior, where applicable.
- Applicable portability guarantees.

---

35. RUST AND MEMORY-SAFETY REQUIREMENTS

The compiler MUST target:

- Rust 1.97 or later.
- Rust edition 2021.
- Safe Rust only.

"unsafe" is prohibited.

Production code MUST NOT contain:

- "unsafe" blocks.
- "unsafe fn".
- "unsafe impl".
- Unnecessary FFI escape mechanisms.
- Unverified unsafe dependencies introduced solely to bypass architecture constraints.

Every relevant compiler module SHOULD enforce:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

Where the module's established lint policy permits it, missing documentation and other relevant lints SHOULD also be denied.

All external dependencies MUST be justified.

The compiler MUST NOT introduce dependencies that duplicate an existing repository authority without a documented migration reason.

---

36. BUILD AND VALIDATION GATES

The following commands are mandatory repository quality gates:

cargo fmt --all -- --check

cargo check --all-targets

cargo test --all-targets

cargo clippy --all-targets -- -D warnings

Additional validation MUST include:

1. Grammar composition validation.
2. Parser generation or equivalent grammar checks.
3. Public API compatibility checks.
4. IR conformance checks.
5. Backend integration tests.
6. Artifact validation.
7. Security tests.
8. Portability tests.
9. Incremental compilation tests.
10. Parallel compilation tests.
11. Scalability tests.
12. Reproducibility tests.

A compiler documentation file MUST NOT claim these gates have passed without actual execution evidence.

The Rust version declared by the repository MUST be checked against the required baseline.

---

37. PRODUCTION ACCEPTANCE CRITERIA

The compiler is production-ready only when all mandatory conditions below are satisfied.

37.1 Architecture

- [ ] One canonical compiler root exists.
- [ ] One canonical compilation pipeline exists.
- [ ] Repository ownership boundaries are respected.
- [ ] No competing semantic authority exists.
- [ ] No duplicate universal IR exists.
- [ ] No duplicate backend registry exists.
- [ ] No duplicate runtime exists.
- [ ] Dependency direction is valid.
- [ ] Existing public APIs are accounted for.

37.2 Frontend and semantics

- [ ] Canonical grammar integration works.
- [ ] Parser integration works.
- [ ] AST integration works.
- [ ] Semantic validation is actually invoked.
- [ ] Ownership validation is actually invoked.
- [ ] Type and effect validation are complete for supported features.
- [ ] Source locations are preserved.
- [ ] Unsupported features are rejected explicitly.

37.3 IR

- [ ] Canonical ZUIR integration works.
- [ ] Classical IR integration works.
- [ ] Quantum IR integration works for declared supported operations.
- [ ] HDLIR integration works for declared supported operations.
- [ ] AI and tensor integration works for declared supported operations.
- [ ] All registered conversions are implemented.
- [ ] All required verifiers are invoked.
- [ ] Semantic preservation is demonstrated.

37.4 Pipeline

- [ ] Required stages execute.
- [ ] Conditional stages execute correctly.
- [ ] Stage dependencies are validated.
- [ ] Failure propagation works.
- [ ] Cancellation works.
- [ ] Recovery is safe.
- [ ] Budget accounting works.
- [ ] Successful completion cannot bypass required stages.

37.5 Planning and lowering

- [ ] Target discovery works.
- [ ] Capability matching works.
- [ ] Resource feasibility works.
- [ ] Heterogeneous partitioning works for supported domains.
- [ ] Data-movement planning works where required.
- [ ] Lowering uses canonical authorities.
- [ ] Unsupported realizations fail explicitly.

37.6 Backends and artifacts

- [ ] Backend registration is functional.
- [ ] Backend selection is evidence-based.
- [ ] Actual backend emission works.
- [ ] Artifacts are validated.
- [ ] Linking works.
- [ ] Portable artifacts are distinguishable from realized artifacts.
- [ ] Compatibility is verified.

37.7 Scalability

- [ ] No arbitrary universal hardware ceilings exist.
- [ ] Operational budgets are configurable.
- [ ] Resource exhaustion is reported.
- [ ] Arithmetic is checked.
- [ ] Dynamic workloads are supported where specified.
- [ ] Incremental compilation is sound.
- [ ] Parallel compilation is correct.
- [ ] No silent truncation occurs.

37.8 Production quality

- [ ] Security verification is complete for supported compilation paths.
- [ ] Provenance is preserved.
- [ ] Diagnostics are structured.
- [ ] Determinism guarantees are tested.
- [ ] Portability is tested against real providers.
- [ ] Rust 1.97+ compatibility is verified.
- [ ] No unsafe Rust exists.
- [ ] All mandatory build gates pass.
- [ ] No unresolved production placeholders remain.
- [ ] Integration tests demonstrate actual execution or valid artifact realization.

A checked box requires evidence.

Documentation alone MUST NOT be used as evidence.

---

38. IMPLEMENTATION GOVERNANCE

This document establishes the integration rules for all subsequent compiler implementation work.

38.1 Change control

Any proposed change to a canonical integration boundary MUST identify:

- Affected files.
- Existing owner.
- New or changed responsibility.
- Public API implications.
- Dependency implications.
- Compatibility implications.
- Security implications.
- Scalability implications.
- Required tests.

Changes MUST be reviewed against:

- "src/compiler/documentation/ARCHITECTURE.md"
- "src/compiler/documentation/PIPELINE_CONTRACT.md"
- "src/compiler/documentation/INTEGRATION_CONTRACT.md"
- "grammar/DESIGN.md"
- "grammar/specification/compilation-model.md"
- "grammar/specification/semantic-model.md"
- "grammar/specification/execution-model.md"
- "grammar/specification/scalability-model.md"
- "grammar/specification/poco-reaf.md"

Where domain semantics are affected, the relevant domain specification MUST also be consulted.

38.2 Authority conflicts

If two files define conflicting ownership or behavior:

1. Identify the affected contracts.
2. Identify their authoritative owners.
3. Consult the repository's normative authority hierarchy.
4. Resolve the conflict through an explicit reviewed change.
5. Update dependent contracts.
6. Update implementations.
7. Update tests.
8. Verify the affected dependency closure.
9. Record compatibility implications.

A developer MUST NOT resolve an architecture conflict by silently introducing a second implementation.

38.3 File freeze

A file may be declared frozen only when:

- Its responsibility is complete.
- Its public API is stable.
- Its dependencies are established.
- Its upstream and downstream integrations are known.
- Its tests pass.
- Its errors are handled.
- Its compatibility requirements are satisfied.
- Its scalability obligations are satisfied.
- Its ownership boundaries are verified.
- Its acceptance criteria are met.

A file may be reopened for a genuine defect, security issue or necessary compatibility correction.

---

39. REQUIRED IMPLEMENTATION ORDER

The following order minimizes architectural rework.

Phase 0 — Repository verification

- Pin the exact repository commit.
- Inventory every file under "src/compiler/".
- Inspect actual module declarations.
- Inspect all compiler callers.
- Inspect repository-wide dependencies.
- Verify grammar composition.
- Verify existing public APIs.
- Identify duplicate or legacy implementations.
- Establish the actual baseline build status.

Phase 1 — Foundational contracts

- Configuration.
- Budgets.
- Target descriptors.
- Policies.
- Diagnostics.
- Provenance.
- Artifact identity and metadata.
- Requirement and capability integration.

Phase 2 — Context and API

- Compilation context.
- Compilation state.
- Dependency context.
- Provenance context.
- Requests.
- Responses.
- Sessions.
- Compiler service.
- Compatibility adapters.

Phase 3 — Pipeline

- Stage contracts.
- Standard stages.
- Dependency ordering.
- Execution.
- Failure handling.
- Recovery.
- Orchestration.

Phase 4 — Analysis and IR

- Program analysis.
- Dependency analysis.
- Effects.
- Ownership integration.
- Capability analysis.
- Feasibility.
- IR input.
- IR adapters.
- Verification.
- Translation.
- Lowering integration.

Phase 5 — Pass management

- Registration.
- Pass manager.
- Pass ordering.
- Legality.
- Analysis preservation.
- Transformation verification.

Phase 6 — Optimization

- Existing optimizer integration.
- Cost models.
- Optimization policies.
- Classical optimization.
- Quantum optimization.
- AI/tensor optimization.
- HDL optimization.
- Hybrid optimization.

Phase 7 — Planning

- Target discovery.
- Capability matching.
- Resource planning.
- Heterogeneous partitioning.
- Abstract scheduling.
- Routing integration.
- Feasibility.

Phase 8 — Lowering and backends

- Legalization.
- Conversion.
- Decomposition.
- Outlining.
- Target lowering.
- Backend discovery.
- Backend selection.
- Backend invocation.

Phase 9 — Artifacts and linking

- Portable artifacts.
- Realized artifacts.
- Metadata.
- Manifests.
- Validation.
- Linking.
- Packaging.

Phase 10 — Incremental and parallel compilation

- Fingerprints.
- Cache integration.
- Invalidation.
- Dependency-aware builds.
- Parallel execution.
- Determinism.

Phase 11 — Security and observability

- Security policy.
- Verification.
- Supply-chain assessment.
- Auditing.
- Profiling.
- Diagnostics.

Phase 12 — Compiler root and production acceptance

- Complete "src/compiler/mod.rs".
- Restore and validate all required exports.
- Connect canonical entry points.
- Run repository-wide tests.
- Run grammar conformance.
- Run real backend tests.
- Run portability tests.
- Run scalability tests.
- Verify all acceptance criteria.

This order MUST be adjusted only when the actual dependency graph demonstrates a different prerequisite order.

---

40. FINAL ARCHITECTURAL INVARIANT

The Zamani compiler MUST implement the following model:

                    ONE SOURCE PROGRAM
                            |
                            v
                   ONE PROGRAM MEANING
                            |
                            v
                  VALIDATED SEMANTIC FACTS
                            |
                            v
                   CANONICAL IR SYSTEMS
                            |
                            v
                  VERIFIED TRANSFORMATIONS
                            |
                            v
                  MULTIPLE VALID PLANS
                            |
                            v
                  MULTIPLE REALIZATIONS
                            |
              +-------------+-------------+
              |             |             |
              v             v             v
            CLASSICAL      QUANTUM        HDL
              |             |             |
              +-------------+-------------+
                            |
                            v
                      HYBRID SYSTEMS
                            |
                            v
                 DISTRIBUTED/ACCELERATED
                       REALIZATION
                            |
                            v
                   VALIDATED ARTIFACTS
                            |
                            v
                    ZAMANI RUNTIME
                            |
                            v
                       EXECUTION

The same architecture MUST accommodate future computational domains without requiring a redesign of the language or compiler foundation.

The compiler must not assume that today's hardware is the final form of computing.

It must not assume that today's available memory, processor count, accelerator count, register width or machine topology defines the maximum size of a Zamani program.

It must distinguish program meaning from the resources required to realize that meaning.

It must report infeasibility rather than fabricate success.

It must preserve semantics rather than silently compromise correctness.

The ultimate invariant is:

«One Zamani program. One defined program meaning. One coordinated compilation architecture. Multiple verified and compatible representations. Multiple feasible hardware realizations. Consistent observable behavior under the program's specified contracts.»

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.

---

END OF INTEGRATION CONTRACT