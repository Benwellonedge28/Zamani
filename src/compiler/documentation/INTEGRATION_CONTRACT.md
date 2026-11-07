Zamani Compiler — Production Integration Contract

File: "src/compiler/documentation/INTEGRATION_CONTRACT.md"

Repository: https://github.com/Benwellonedge28/Zamani

Branch: "main"

Language: Zamani Universal Computing Language

Rust edition: 2021

Minimum supported Rust version: Rust 1.97 or later

Safety: Safe Rust only. "unsafe" is prohibited.

Architectural objective:

"Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)"

Document purpose: Normative architectural and integration contract for the Zamani compiler.

Document status: Production architecture contract. Implementation conformance must be demonstrated independently through source inspection, compilation, integration testing and release qualification.

---

1. Purpose

This document establishes the binding integration architecture for:

"src/compiler/"

Its purpose is to make Zamani's compiler a scalable, extensible, target-independent compilation system capable of coordinating all supported computational domains.

The compiler must support:

- Classical computing.
- Systems programming.
- Embedded computing.
- Scientific and numerical computing.
- Parallel and concurrent computing.
- Distributed computing.
- Quantum computing.
- Quantum-classical computing.
- Quantum simulation.
- Quantum error-correction integration.
- Hardware description language compilation.
- FPGA and ASIC realization.
- Hardware/software co-design.
- Artificial intelligence.
- Machine learning and deep learning.
- Tensor computation.
- Symbolic and probabilistic computation.
- Neural-symbolic computation.
- Neuromorphic computing.
- Networking.
- Cryptography and secure computation.
- Data and stream processing.
- Hybrid heterogeneous computing.
- Future computational domains.

These capabilities must operate within one coordinated Zamani language and compiler architecture.

The compiler must not become a collection of disconnected domain-specific compilers.

It must coordinate the canonical frontend, semantic model, intermediate representations, optimization infrastructure, target planning, backend implementations, artifact systems and runtime.

The objective is:

Write a Zamani program once, preserve its intended semantics, compile it into appropriately portable representations and artifacts, and realize it on compatible execution environments without rewriting the application for each machine.

A finite implementation cannot guarantee literal infinite computation or universal execution on physically incompatible machines.

The architecture must instead avoid arbitrary universal machine-capacity restrictions and scale according to actual available resources, explicitly configured policies, implementation capabilities and valid target constraints.

---

2. Normative terminology

The following words are normative:

- MUST: Required.
- MUST NOT: Prohibited.
- SHOULD: Strong recommendation.
- SHOULD NOT: Generally prohibited unless justified.
- MAY: Optional.

All compiler modules must follow these definitions.

2.1 Fundamental distinctions

Concept| Meaning
Requirement| What the program must have to preserve valid execution semantics.
Capability| What an implementation or execution environment supports.
Preference| What the developer would prefer among valid alternatives.
Constraint| What a realization is permitted or prohibited from doing.
Hint| Advisory information that may improve implementation decisions.
Feasibility| Evidence-based determination of whether a proposed realization is valid and achievable.
Availability| Whether a required resource is actually available at a particular point in time.
Allocation| Acquisition or reservation of actual resources.
Realization| Mapping validated computation onto an implementation or execution environment.

These concepts MUST NOT be collapsed into one target-selection flag.

A preference must never override a mandatory requirement.

A hint must never change program semantics.

Unknown capabilities must never be interpreted as unlimited support.

An unresolved requirement must never be silently treated as satisfied.

---

3. Canonical repository authorities

The compiler is an orchestration and integration subsystem.

It must consume the authoritative interfaces already established elsewhere in Zamani.

It must not create competing implementations.

3.1 Language and grammar

The language-definition authorities are:

- "grammar/DESIGN.md"
- "grammar/Zamani.g4"
- "grammar/antlr/ZamaniLexer.g4"
- "grammar/antlr/ZamaniParser.g4"
- "grammar/lexer/tokens.g4"
- "grammar/specification/"
- "grammar/spec/"
- "grammar/validation/"
- "grammar/compile/"
- Domain-specific grammar directories.

The grammar defines syntax.

The specifications define normative language meaning and contracts.

The frontend owns lexical analysis, parsing and AST construction.

The compiler MUST NOT introduce another parser or independently interpret source syntax.

3.2 Frontend and semantic authority

Canonical semantic authority:

"src/frontend/semantic/"

Important integration areas include:

- "src/frontend/semantic/mod.rs"
- Semantic types.
- Name and symbol resolution.
- Effects.
- Ownership.
- Capabilities.
- Resources.
- Requirements.
- Constraints.
- Policies.
- Contracts.
- Provenance.
- Domain validation.
- Diagnostics.

The compiler may perform additional analyses over validated semantic information.

It MUST NOT create a competing source-level semantic model.

3.3 Intermediate representations

The authoritative representation boundaries are:

Representation| Owner
Universal IR| "src/zuir/"
Classical computation| "src/classical/"
Quantum computation| "src/quantum/ir/"
Hardware description| "src/hdl/hdlir/"
AI computation| "src/ai/"
Hybrid computation| "src/hybrid/"
Distributed computation| "src/distributed/"

The compiler integrates these representations through verified adapters.

"src/compiler/ir/" MUST NOT become another universal IR implementation.

ZUIR MUST NOT replace the authoritative semantics of the domain-specific IRs.

3.4 Backend authority

Canonical backend boundary:

"src/backend/"

The compiler's "src/compiler/backends/" integrates with this subsystem.

It must not introduce another independent backend implementation or authoritative registry.

3.5 Runtime authority

Canonical execution boundary:

"src/runtime/"

The compiler prepares verified artifacts and deployment information.

The runtime owns actual execution, resource acquisition and runtime scheduling.

3.6 Additional integration authorities

The compiler must also integrate with:

- "src/stdlib/"
- "src/toolchain/"
- "src/compiler_types.rs"
- "src/source_map.rs"
- "src/ir_verify.rs"
- "src/ir_gen.rs" during legacy migration.

No subsystem may silently become a second authority for a responsibility already assigned elsewhere.

---

4. Fundamental architecture

The canonical compilation architecture is:

                  ZAMANI SOURCE
                        |
                        v
                 CANONICAL GRAMMAR
                        |
                        v
                     FRONTEND
                        |
             +----------+----------+
             |          |          |
             v          v          v
           Lexer      Parser      AST
                        |
                        v
                SEMANTIC VALIDATION
                        |
                        v
                CANONICAL SEMANTIC
                      MODEL
                        |
                        v
                SEMANTICALLY VALID
                    PROGRAM
                        |
                        v
                CANONICAL IR INPUT
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
                   OPTIMIZATION
                        |
                        v
               CAPABILITY ANALYSIS
                        |
                        v
                FEASIBILITY ANALYSIS
                        |
                        v
                RESOURCE PLANNING
                        |
                        v
              HETEROGENEOUS PLANNING
                        |
                        v
                DOMAIN LOWERING
                        |
                        v
               TARGET LEGALIZATION
                        |
                        v
                BACKEND SELECTION
                        |
                        v
               BACKEND REALIZATION
                        |
                        v
               ARTIFACT VALIDATION
                        |
                        v
                  LINKING
                        |
                        v
               PORTABLE ARTIFACTS
                        |
                        v
                DEPLOYMENT HANDOFF
                        |
                        v
                  ZAMANI RUNTIME
                        |
                        v
                 ACTUAL EXECUTION

This represents the architectural dependency flow.

It does not mean every program must execute every stage.

A classical-only program must not be forced through quantum compilation.

A quantum program must preserve quantum semantics.

An HDL program must preserve hardware-description semantics.

A hybrid program must preserve cross-domain dependencies.

A distributed program must preserve communication, ordering and consistency requirements.

Conditional stages are permitted.

Silently skipping required stages is prohibited.

---

5. Compiler ownership boundaries

The compiler owns:

1. Compilation orchestration.
2. Compiler API composition.
3. Compiler configuration integration.
4. Compiler operational budgets.
5. Compilation lifecycle.
6. Compiler diagnostics integration.
7. Compiler-facing analysis coordination.
8. IR integration and verification coordination.
9. Pass management.
10. Transformation coordination.
11. Optimization planning.
12. Target discovery integration.
13. Capability matching integration.
14. Feasibility assessment.
15. Abstract resource planning.
16. Heterogeneous workload partitioning.
17. Abstract dependency-aware scheduling.
18. Backend selection coordination.
19. Backend invocation coordination.
20. Domain lowering coordination.
21. Artifact validation.
22. Linking coordination.
23. Incremental compilation coordination.
24. Parallel compilation.
25. Reproducibility coordination.
26. Compiler security integration.
27. Compiler profiling.
28. Compilation testing and conformance.

The compiler does NOT own:

- Language syntax.
- AST construction.
- Source-level semantic meaning.
- Canonical IR definitions.
- Domain-specific IR definitions.
- Physical hardware discovery.
- Physical resource allocation.
- Device reservation.
- Runtime scheduling.
- Quantum physical routing.
- Quantum error correction implementation.
- Backend implementation.
- Network transport.
- Actual program execution.

These boundaries are mandatory.

---

6. Scalability and hard-coding prohibition

6.1 Universal scalability

The compiler must not introduce arbitrary universal ceilings on:

- CPU count.
- CPU core count.
- Thread count.
- Worker count.
- GPU count.
- FPGA count.
- ASIC count.
- QPU count.
- Accelerator count.
- Qubit count.
- Logical qubit count.
- Memory capacity.
- VRAM capacity.
- Storage capacity.
- Register width.
- Tensor rank.
- Tensor dimensions.
- Distributed participants.
- Network participants.
- Modules.
- Functions.
- Operations.
- Compilation units.
- Optimization passes.
- Pipeline stages.
- Targets.
- Providers.
- Domains.
- Artifacts.
- Dependencies.

Examples of prohibited architectural restrictions include:

const MAX_CPUS: usize = 64;
const MAX_GPUS: usize = 8;
const MAX_QUBITS: usize = 128;
const MAX_WORKERS: usize = 256;
const MAX_TENSOR_RANK: usize = 16;
const MAX_MEMORY: u64 = 64 * 1024 * 1024 * 1024;

These must not be introduced as universal compiler limits.

6.2 Real constraints

Real implementation constraints remain valid.

Examples:

- A processor has a finite instruction width.
- A QPU exposes a finite physical qubit count.
- An FPGA has finite logic resources.
- A backend supports a particular artifact format.
- An operating system imposes memory limits.
- An external toolchain supports a finite set of operations.

These limitations must be discovered, represented and reported by their authoritative owners.

They must not be transformed into universal Zamani language restrictions.

6.3 Operational budgets

Compiler budgets belong to:

"src/compiler/configuration/budget.rs"

Budgets may limit:

- Compilation time.
- Memory consumption.
- Optimization work.
- Analysis work.
- Artifact generation.
- Cache consumption.
- Parallel work.
- Diagnostic collection.
- Other explicitly accounted compiler operations.

An omitted budget is not proof of infinite physical resources.

Exhaustion must produce an explicit outcome.

The compiler must never silently truncate work and report success.

6.4 Representation limits

Representation types must be selected according to their actual responsibility.

For example:

- "usize" may be necessary for indexing an in-memory collection.
- A target ABI may require a fixed-width integer.
- A budget counter may use "u128" with explicit overflow handling.
- A semantic resource quantity may require arbitrary-precision decimal representation.

The choice must not create an accidental universal hardware ceiling.

Where an arithmetic operation overflows, the operation must fail explicitly.

No wrapping, saturating or truncating behavior may silently alter semantic quantities.

6.5 Infinite-resource distinction

The architectural objective is unbounded scalability with respect to arbitrary language-level hardware ceilings.

It is not a claim that physical resources are infinite.

The compiler must scale from the smallest supported execution environment to larger environments as resources and compatible implementations become available.

---

7. Grammar-to-compiler integration

The grammar is authoritative for source-level compilation intent.

The compiler must consume the semantic interpretation of that intent rather than parse or reinterpret grammar rules independently.

7.1 Root grammar

"grammar/Zamani.g4"

Owns the root grammar composition boundary.

It must not contain compiler implementation details, hardware discovery or backend selection logic.

7.2 Parser composition

"grammar/antlr/ZamaniParser.g4"

Owns parser composition and universal source entry points.

It must preserve the domain-neutral AST architecture.

7.3 Lexer composition

"grammar/antlr/ZamaniLexer.g4"

Owns lexical grammar integration.

"grammar/lexer/tokens.g4" remains the lexical token authority.

The compiler must consume frontend output rather than introduce independent token definitions.

7.4 Compilation intent grammar

The following existing files have distinct responsibilities.

Grammar file| Responsibility
"grammar/compile/compile.g4"| Compilation syntax composition.
"grammar/compile/intent.g4"| General compilation intent.
"grammar/compile/profiles.g4"| Named compilation profiles.
"grammar/compile/target.g4"| Source-level target intent.
"grammar/compile/target-selection.g4"| Target-selection policy.
"grammar/compile/optimization.g4"| Optimization intent.
"grammar/compile/lowering.g4"| Lowering intent.
"grammar/compile/caching.g4"| Caching intent.
"grammar/compile/reproducibility.g4"| Reproducibility intent.

Other existing compilation grammar modules must retain their respective ownership.

The compiler must not introduce competing source-level types for these constructs.

7.5 Target intent versus realization

These are separate concepts:

Target declaration
       |
       v
Target semantic intent
       |
       v
Target-selection policy
       |
       v
Capability matching
       |
       v
Feasibility assessment
       |
       v
Resource planning
       |
       v
Backend realization
       |
       v
Runtime execution

A source-level target declaration does not identify an available physical device.

A target preference does not guarantee backend compatibility.

A structurally valid target descriptor does not establish feasibility.

7.6 Semantic authority

"grammar/specification/semantic-model.md" defines the semantic contract.

"src/frontend/semantic/" implements the semantic authority.

The compiler must preserve the distinctions established by that specification.

Grammar qualification requirement: Before declaring the language-to-compiler integration production-ready, audit the grammar composition and all imported compilation grammars, validate the normative specifications, and test actual generated-parser-to-AST integration. Documentation alone is insufficient.

---

8. Compiler module integration contracts

This section defines the responsibility and integration requirements of the existing compiler subsystems.

Existing files must be expanded before introducing competing implementations.

8.1 Composition root

"src/compiler/mod.rs"

Owns:

- Compiler module composition.
- Public compiler namespace.
- Compiler API version.
- Architectural invariants.
- Module integration boundaries.

Must integrate:

- "api/"
- "configuration/"
- "context/"
- "diagnostics/"
- "analysis/"
- "ir/"
- "pipeline/"
- "passes/"
- "optimization/"
- "backends/"
- "lowering/"
- "planning/"
- "parallel/"
- "incremental/"
- "linking/"
- "artifacts/"
- "security/"
- "profiling/"
- "testing/" where implemented.
- Existing standalone implementations.

Must not:

- Implement a second compiler.
- Define source-language semantics.
- Define another IR.
- Instantiate physical hardware.
- Execute programs.

Every declared module must resolve to a real implementation.

Every public export must have a deliberate owner.

The composition root must not advertise unsupported functionality as available.

8.2 Public API

Directory:

"src/compiler/api/"

"api/mod.rs"

Owns public API composition and stable re-exports.

Must integrate:

- Compiler service.
- Compilation requests.
- Compilation responses.
- Compilation sessions.
- API compatibility.

"api/compiler.rs"

Owns the compiler service abstraction and production driver interface.

Must integrate:

- Configuration validation.
- Compilation context.
- Pipeline orchestration.
- Diagnostics.
- Artifacts.
- Cancellation.
- Incremental compilation.

A production driver must execute the actual canonical compilation path.

It must not delegate to an unverified legacy pipeline and claim production compilation.

"api/request.rs"

Owns compilation request data.

Must preserve:

- Source identity.
- Source input.
- Validated configuration.
- Compilation intent.
- Target preferences.
- Explicit requirements and constraints.

Must not perform parsing or semantic analysis.

"api/response.rs"

Owns the public compilation result.

Must preserve:

- Artifact references.
- Structured diagnostics.
- Compilation status.
- Verification evidence.
- Provenance.
- Portability information.
- Backend and target realization information where applicable.

A failed or incomplete compilation must never produce a successful response.

"api/session.rs"

Owns compilation session lifecycle and cancellation integration.

Must support independent sessions without leaking mutable state.

"api/compatibility.rs"

Owns API version and compatibility contracts.

Language, compiler API, IR schema, artifact format and backend ABI versions must remain distinct.

Critical existing correction

The inspected "api/mod.rs" imports:

super::CompilerError
super::CompilationOutput
super::production_config
super::compile_source
super::compile_with_config

The inspected "src/compiler/mod.rs" does not define those items.

This is an integration defect.

Before freezing "api/mod.rs":

1. Establish the actual canonical error and output types.
2. Identify the authoritative existing compilation entry points.
3. Remove references to nonexistent parent symbols.
4. Implement the production driver against the canonical pipeline.
5. Preserve legacy public entry points through explicit adapters.
6. Add API integration tests.
7. Verify all module paths through Cargo.

Do not invent replacement types when an existing authoritative type can be reused.

8.3 Configuration

Directory:

"src/compiler/configuration/"

"configuration/mod.rs"

Owns configuration composition and public re-exports.

"configuration/config.rs"

Owns construction and structural validation of the existing "CompilerConfig".

Must preserve compatibility with "src/compiler_types.rs" during migration.

"configuration/budget.rs"

Owns operational compilation budgets and shared accounting.

Must:

- Support optional limits.
- Account for actual work.
- Share accounting safely across workers.
- Report exhaustion.
- Detect arithmetic overflow.
- Avoid partial accounting changes after failed transactions.

Its "u128" accounting representation must remain an operational accounting choice, not a universal resource representation.

"configuration/target.rs"

Owns compiler-facing extensible target descriptors.

Must use open, namespaced target identities.

Must not extend the legacy "CompilationTarget" enum for every new hardware architecture.

Must distinguish descriptor validity from actual capability support.

"configuration/policy.rs"

Owns compiler-side policy contracts.

Must consume canonical semantic policy information and delegate authorization meaning to the authoritative policy subsystem.

"configuration/profile.rs"

Owns reusable compilation-profile registration and resolution.

Must preserve the source-level profile semantics established by the frontend.

A profile must not claim that a recommended optimization pass is implemented or legal without verification.

"configuration/validation.rs"

Owns aggregate configuration validation.

Must report:

- Invalid fields.
- Conflicting settings.
- Missing required configuration.
- Unsupported configuration versions.
- Invalid policy combinations.
- Invalid target descriptors.

Structural validation must not claim physical feasibility.

8.4 Compiler types

"src/compiler/compiler_types.rs"

Owns existing compiler compatibility types.

Must preserve existing callers during migration.

The legacy "CompilationTarget" must not become the authoritative open-world target system.

The extensible target system belongs to:

"src/compiler/configuration/target.rs"

Canonical semantic types belong to the frontend.

Canonical resource quantities belong to the semantic resource model.

Domain-specific types belong to their respective subsystems.

Do not introduce another type system here.

8.5 Compilation context

Directory:

"src/compiler/context/"

"context/mod.rs"

Owns context composition.

"context/compilation_context.rs"

Owns immutable compilation inputs and shared compilation services.

Must integrate:

- Validated configuration.
- Budget tracking.
- Cancellation.
- Diagnostics.
- Provenance.
- Dependency context.
- Compilation identity.

"context/compilation_state.rs"

Owns compilation lifecycle states and valid transitions.

Must distinguish:

- Created.
- Initialized.
- Running.
- Completed.
- Failed.
- Cancelled.

Invalid transitions must produce structured errors.

"context/dependency_context.rs"

Owns compiler-facing access to canonical compilation dependencies.

Must be reused by:

- "parallel/dependencies.rs"
- "parallel/scheduler.rs"
- "incremental/"
- "analysis/dependency.rs"
- "pipeline/ordering.rs"

No competing dependency graph is permitted.

"context/provenance_context.rs"

Owns compilation provenance context.

Must preserve source, configuration, compiler version, semantic identity, transformation history and verification evidence.

8.6 Diagnostics

Directory:

"src/compiler/diagnostics/"

"diagnostics/mod.rs"

Owns diagnostic subsystem composition.

Canonical authorities:

- "src/frontend/semantic/diagnostics/"
- "src/frontend/ast/node/source/span.rs"
- "src/frontend/semantic/diagnostics/ordering.rs"
- "grammar/spec/diagnostics.md"
- "grammar/spec/source-spans.md"

"diagnostics/aggregation.rs"

Owns compiler-level aggregation.

Must integrate existing diagnostic records without redefining severity, identity or source spans.

"diagnostics/rendering.rs"

Owns human-readable rendering.

Must preserve:

- Source locations.
- Related spans.
- Diagnostic severity.
- Diagnostic codes.
- Context.
- Relevant suggestions.

"diagnostics/reporting.rs"

Owns machine-readable reporting.

Must provide deterministic, versioned serialization.

The existing "zamani.diagnostics.v1" contract must be preserved or deliberately versioned.

Serialization must preserve structured diagnostic meaning rather than flattening all errors into strings.

No fixed diagnostic-count ceiling may be introduced.

8.7 Compiler analysis

Directory:

"src/compiler/analysis/"

"analysis/mod.rs"

Owns analysis subsystem composition.

"analysis/program.rs"

Owns compiler-facing program analysis.

Consumes canonical AST and IR information.

Must not create a second program representation.

"analysis/dependency.rs"

Owns compiler-facing dependency analysis.

Must integrate the canonical dependency context and ZUIR operation identities.

"analysis/effects.rs"

Consumes authoritative frontend effect information.

Must not redefine effect semantics.

"analysis/ownership.rs"

Integrates canonical frontend ownership and borrowing information.

Must not create a second ownership system.

"analysis/capabilities.rs"

Delegates capability compatibility to:

"src/frontend/semantic/capabilities/"

It must preserve unresolved and incompatible results.

"analysis/requirements.rs"

Collects and normalizes validated semantic requirements.

Must preserve requirement, constraint, preference and hint distinctions.

"analysis/feasibility.rs"

Aggregates capability, resource and realization evidence.

Must not infer feasibility from incomplete evidence.

"analysis/cost.rs"

Coordinates cost information from authoritative cost models.

Must not assume that classical operation costs apply to quantum, HDL or distributed computation.

"analysis/provenance.rs"

Preserves the evidence and origins of compiler analysis results.

8.8 IR integration

Directory:

"src/compiler/ir/"

"ir/mod.rs"

Owns compiler-facing IR integration composition.

"ir/input.rs"

Currently documents a verification-gated ZUIR input.

This is the correct initial direction.

Must consume canonical ZUIR verification results.

It must not claim automatic support for domain IRs whose adapters have not been implemented and verified.

"ir/adapters.rs"

Owns compiler-facing adapter coordination.

Must delegate actual representation conversion to the relevant domain authorities.

Required integration:

- ZUIR.
- Classical IR.
- Quantum IR.
- HDLIR.
- AI and tensor representations.
- Hybrid representations.
- Distributed representations.

Each adapter must explicitly declare supported source and destination representations.

"ir/compatibility.rs"

Owns representation and version compatibility assessment.

"ir/translation.rs"

Coordinates validated translation.

Must preserve semantic information, effects, resource requirements and provenance.

"ir/lowering.rs"

Coordinates compiler-facing lowering requests.

Must delegate actual lowering implementation to the authoritative domain and backend owners.

"ir/verification.rs"

Coordinates verification and preserves verification evidence.

Invalid or incomplete verification must not be treated as success.

8.9 Pass management

Directory:

"src/compiler/passes/"

"passes/mod.rs"

Owns pass-management composition.

"passes/registry.rs"

Owns integration with the canonical extensible pass registry.

Must not duplicate domain-owned pass registries.

"passes/manager.rs"

Coordinates pass execution.

"passes/ordering.rs"

Resolves pass dependencies and ordering.

"passes/pipeline.rs"

Composes applicable passes.

"passes/legality.rs"

Checks transformation legality against the relevant semantic and IR contracts.

"passes/preservation.rs"

Records and validates preservation obligations.

"passes/analysis_manager.rs"

Manages analysis-result validity and invalidation.

A classical transformation must not automatically be considered legal for quantum, HDL or other domains.

8.10 Optimization

Directory:

"src/compiler/optimization/"

"optimization/mod.rs"

Owns optimization subsystem composition.

"optimization/cost_model.rs"

Provides extensible cost-model integration.

"optimization/decisions.rs"

Records optimization decisions and evidence.

"optimization/planning.rs"

Plans applicable optimization strategies.

"optimization/policies.rs"

Enforces semantic-preservation and optimization policies.

"optimization/profitability.rs"

Evaluates transformation profitability.

"optimization/validation.rs"

Verifies transformation results.

All transformations must preserve the applicable program semantics or produce an explicit failure.

Existing "instruction_fusion.rs", "monomorphizer.rs" and "sro.rs" must integrate through these contracts rather than create competing optimization authorities.

8.11 Lowering

Directory:

"src/compiler/lowering/"

"lowering/mod.rs"

Owns lowering composition.

"lowering/conversion.rs"

Coordinates verified representation conversion.

"lowering/decomposition.rs"

Coordinates domain-specific operation decomposition.

"lowering/legalization.rs"

Validates target legality and performs permitted legalization.

"lowering/outlining.rs"

Extracts independently compilable regions while preserving dependencies.

"lowering/target_lowering.rs"

Coordinates target-specific lowering through registered backend interfaces.

Domain-specific decomposition remains with the respective domain authorities.

Quantum decomposition must preserve quantum semantics.

HDL lowering must preserve hardware structural and signal semantics.

AI lowering must preserve tensor and computational semantics.

8.12 Backend integration

Directory:

"src/compiler/backends/"

"backends/mod.rs"

Owns compiler-facing backend integration.

"backends/registry.rs"

Integrates the canonical registry in "src/backend/".

Must not establish another authoritative backend registry.

"backends/discovery.rs"

Coordinates backend discovery.

"backends/capabilities.rs"

Consumes registered backend capability descriptions.

"backends/compatibility.rs"

Evaluates compatibility using authoritative semantic requirements, capability evidence and feasibility reports.

"backends/selection.rs"

Selects an eligible registered backend using explicit preferences and deterministic ranking.

Must not treat registration as compatibility.

Must not treat compatibility as resource availability.

"backends/invocation.rs"

Coordinates validated invocation of the selected backend.

Must preserve:

- Backend identity.
- Input verification.
- Invocation configuration.
- Structured errors.
- Output validation.
- Provenance.

Critical backend migration

"src/backend/mod.rs" documents the transitional backend architecture.

It explicitly identifies unresolved concerns involving legacy target dispatch and misleading LLVM artifact output.

These must be corrected in the backend implementation before the compiler can be considered production-ready.

The compiler must never report successful artifact generation when the backend only returns textual IR or placeholder output.

8.13 Planning

Directory:

"src/compiler/planning/"

This subsystem is responsible for abstract compilation and realization planning.

"planning/mod.rs"

Owns planning composition.

"planning/target_discovery.rs"

Integrates registered target discovery.

"planning/capability_matching.rs"

Evaluates semantic requirements against explicit capability evidence.

"planning/resource_planning.rs"

Consumes canonical semantic resource requirements and target-scoped resource evidence.

The existing implementation uses exact decimal quantities.

Preserve this approach.

Do not convert arbitrary resource quantities into fixed-width machine integers or floating-point approximations.

It must not allocate physical resources.

"planning/feasibility.rs"

Aggregates capability and resource feasibility.

Must preserve unknown, incomplete, incompatible and infeasible states.

"planning/partitioning.rs"

Integrates:

- Canonical ZUIR operation identities.
- Capability matching.
- Resource feasibility.
- Existing distributed workload partitioning.

The existing distributed partitioner remains authoritative for its generic partitioning responsibility.

This module owns compiler-facing heterogeneous partition refinement.

It does not allocate physical devices.

"planning/data_movement.rs"

Coordinates data movement planning.

Must consume dependency, topology and transfer-cost evidence from authoritative providers.

Must not implement network transport.

"planning/routing.rs"

Coordinates abstract routing requirements.

Must distinguish compiler-level communication planning from quantum physical routing and distributed network routing.

"planning/scheduling.rs"

Constructs dependency-respecting abstract schedules.

The existing implementation uses deterministic topological scheduling.

It must preserve:

- Canonical operation identities.
- Dependency relationships.
- Explicit scheduling parallelism.
- Configurable operational budgets.
- Cycle detection.
- Deterministic ordering.
- Explicit scheduling failures.

It must not become the runtime scheduler.

Planning invariant

Requirements
     |
     v
Capability evidence
     |
     v
Resource evidence
     |
     v
Feasibility
     |
     v
Partitioning
     |
     v
Abstract scheduling
     |
     v
Lowering and backend realization
     |
     v
Runtime revalidation
     |
     v
Actual resource acquisition

The runtime must revalidate availability because planning evidence may become stale.

8.14 Parallel compilation

Directory:

"src/compiler/parallel/"

The existing modular subsystem is authoritative for compiler-side parallel work.

"parallel/mod.rs"

Owns subsystem composition.

"parallel/dependencies.rs"

Consumes the canonical dependency context and produces dependency-ready work.

"parallel/workers.rs"

Owns configurable compiler worker execution.

Must not establish arbitrary worker-count ceilings.

Must support cancellation, error propagation and controlled resource consumption.

"parallel/scheduler.rs"

Coordinates dependency-aware work execution.

Must not duplicate the canonical dependency graph.

"parallel/determinism.rs"

Validates deterministic result collection and integrates canonical determinism policy.

Critical correction

The earlier proposed "src/compiler/parallel_build.rs" must not be reintroduced as a competing parallel compilation subsystem.

The existing "src/compiler/parallel/" architecture must be used.

Any remaining callers of the obsolete path must be migrated through explicit compatibility adapters.

8.15 Incremental compilation

Directory:

"src/compiler/incremental/"

"incremental/mod.rs"

Coordinates incremental compilation.

"incremental/cache.rs"

Integrates canonical artifact caching.

"incremental/fingerprints.rs"

Coordinates stable fingerprints.

"incremental/invalidation.rs"

Integrates canonical dependency invalidation.

"incremental/persistence.rs"

Owns compiler-facing persistence coordination.

Must use the canonical cache and invalidation authorities.

Must account for:

- Source content.
- Semantic identity.
- IR schema versions.
- Transformation versions.
- Configuration.
- Policies.
- Backend identity.
- Toolchain versions.
- Relevant capability evidence.
- Required verification.
- Artifact format.

A cache hit must never bypass required verification or security checks.

Critical correction

"incremental/mod.rs" currently documents "src/compiler/parallel_build.rs" as a dependency.

That reference must be corrected to the actual modular parallel infrastructure and canonical dependency context.

The existing canonical authorities include:

- "src/zuir/incremental/dependencies.rs"
- "src/zuir/incremental/invalidation.rs"
- "src/zuir/incremental/cache_keys.rs"
- "src/zuir/fingerprint/stable_hash.rs"
- "src/backend/cache.rs"

The compiler must coordinate these authorities rather than recreate them.

8.16 Pipeline

Directory:

"src/compiler/pipeline/"

"pipeline/mod.rs"

Owns pipeline composition.

"pipeline/stage.rs"

Defines stage contracts, identifiers, inputs, outputs, determinism, concurrency and structured errors.

"pipeline/stages.rs"

Defines standard stage identifiers and dependencies.

It must not pretend that a declared stage automatically has an implementation.

"pipeline/ordering.rs"

Validates dependencies and constructs deterministic execution order.

"pipeline/execution.rs"

Executes registered stages and preserves dependency outputs.

Required stages must complete successfully.

"pipeline/failure.rs"

Provides structured pipeline failure representation.

"pipeline/recovery.rs"

Evaluates explicitly permitted recovery policies.

Recovery must not change semantic meaning or bypass verification.

"pipeline/orchestration.rs"

Owns complete compilation workflow coordination.

It must connect actual stage implementations rather than merely execute demonstration stages.

It must coordinate:

- Compilation lifecycle.
- Required stages.
- Conditional stages.
- Cancellation.
- Diagnostics.
- Budget tracking.
- Failure propagation.
- Partial execution reports.
- Recovery decisions.
- Final completion status.

Critical production requirement

A pipeline is production-ready only when the complete supported source-to-artifact path is demonstrably connected.

A stage registry and executor alone do not establish end-to-end compilation.

8.17 Artifacts

Directory:

"src/compiler/artifacts/"

"artifacts/mod.rs"

Owns artifact subsystem composition.

"artifacts/artifact.rs"

Owns artifact identity and representation contracts.

"artifacts/manifest.rs"

Owns artifact manifest metadata and dependencies.

"artifacts/metadata.rs"

Preserves compiler, language, IR, backend and toolchain information.

"artifacts/portability.rs"

Classifies portability and records compatibility evidence.

"artifacts/serialization.rs"

Provides validated, versioned serialization.

"artifacts/validation.rs"

Validates artifact integrity, structure, semantics and compatibility.

Artifacts must distinguish:

- Source-portable representations.
- Target-independent verified representations.
- Target-specific intermediate artifacts.
- Executable artifacts.
- Hardware-description artifacts.
- Quantum artifacts.
- Deployment packages.

A portable artifact is not automatically executable on every machine.

8.18 Linking

Directory:

"src/compiler/linking/"

"linking/mod.rs"

Owns linking composition.

"linking/symbols.rs"

Coordinates canonical symbol resolution.

"linking/composition.rs"

Coordinates compatible module and artifact composition.

"linking/relocation.rs"

Coordinates target-specific relocation.

"linking/validation.rs"

Validates symbols, ABI compatibility, dependencies and artifact compatibility.

Must integrate "src/stdlib/" and the canonical backend/linking interfaces.

Must not introduce a competing language symbol or type system.

8.19 Security

Directory:

"src/compiler/security/"

"security/mod.rs"

Owns security subsystem composition.

"security/policy.rs"

Integrates canonical policy evaluation.

"security/verification.rs"

Coordinates verification evidence.

"security/supply_chain.rs"

Coordinates dependency, provider, toolchain and artifact supply-chain assurance.

"security/audit.rs"

Integrates auditable compilation operations.

Must preserve:

- Input validation.
- Provider trust.
- Dependency integrity.
- Artifact authenticity.
- Secure serialization.
- Authorization.
- Provenance.
- Resource exhaustion protection.
- Extension trust boundaries.

Security checks must not be optional merely because a cache hit occurred.

8.20 Profiling

Directory:

"src/compiler/profiling/"

"profiling/mod.rs"

Owns profiling composition.

"profiling/events.rs"

Owns compiler-specific profiling events.

It must consume canonical pipeline stage identities.

"profiling/measurements.rs"

Owns typed measurement records and provenance.

Measurements must distinguish stable and nondeterministic observations.

"profiling/aggregation.rs"

Owns streaming aggregation.

Must use configurable aggregation budgets and canonical observability infrastructure.

Profiling must not modify compilation semantics.

Nondeterministic measurements must not unexpectedly change reproducible compilation results.

8.21 Existing standalone compiler implementations

These existing files must be integrated rather than duplicated.

File| Integration responsibility
"compiler_types.rs"| Shared compatibility types.
"borrow_checker.rs"| Frontend ownership integration.
"monomorphizer.rs"| Generic specialization and domain-aware legality.
"instruction_fusion.rs"| Semantics-preserving fusion.
"macro_engine.rs"| Macro expansion and canonical AST integration.
"safety_guard.rs"| Compiler safety inspection.
"audit_engine.rs"| Compilation audit evidence.
"fuzzing_harness.rs"| Source, IR, pipeline and artifact fuzzing.
"sro.rs"| Telemetry-informed optimization integration.
"ssbe.rs"| Specialized static backend emission integration.

No standalone implementation may silently create a competing compiler, IR, optimizer or backend.

---

9. Cross-repository integration requirements

The following table establishes the required integration boundaries.

Repository path| Compiler responsibility| Required integration
"src/frontend/"| Frontend input| Canonical AST and parser output.
"src/frontend/semantic/"| Semantic analysis| Types, effects, capabilities, resources, requirements and provenance.
"src/zuir/"| IR input| Verified universal representation.
"src/classical/"| Classical lowering| Classical computation semantics.
"src/quantum/ir/"| Quantum lowering| Quantum semantics and verification.
"src/hdl/hdlir/"| HDL lowering| Hardware-description semantics.
"src/ai/"| AI lowering| AI and tensor computation semantics.
"src/hybrid/"| Hybrid planning| Cross-domain dependencies and coordination.
"src/distributed/"| Distributed planning| Communication and logical partitioning.
"src/backend/"| Backend realization| Actual target-specific generation.
"src/runtime/"| Deployment handoff| Physical resource acquisition and execution.
"src/stdlib/"| Linking| Standard-library symbols and ABI compatibility.
"src/toolchain/"| Developer tooling| CLI, LSP and build integration.
"src/source_map.rs"| Diagnostics| Canonical source locations.
"src/ir_verify.rs"| Legacy verification migration| Explicit compatibility and conformance.
"src/ir_gen.rs"| Legacy IR migration| Explicit migration to canonical IR.

Every integration must be demonstrated with actual code and tests.

A module existing in the repository does not establish that the compiler supports it.

---

10. Domain-specific compilation contracts

10.1 Classical

Must preserve:

- Type semantics.
- Memory semantics.
- Ownership.
- Effects.
- Control flow.
- Function and module interfaces.
- ABI requirements.

Must integrate with the canonical classical representation and existing backend providers.

10.2 Quantum

Must preserve:

- Quantum state semantics.
- Gate and operation semantics.
- Measurement semantics.
- Entanglement.
- Reversibility obligations.
- Quantum resource requirements.
- Measurement effects.

Physical qubit mapping, quantum routing, calibration, scheduling and QEC remain with their authoritative downstream subsystems.

A classical fallback must never silently replace required quantum semantics.

10.3 HDL

Must preserve:

- Signal identity.
- Hardware structure.
- Timing semantics.
- Clock domains.
- Hardware interfaces.
- Structural dependencies.
- Parameterization.
- Synthesis constraints.

HDL lowering must not be reduced to ordinary executable code generation.

Vendor-specific synthesis remains a backend/provider responsibility.

10.4 AI and tensor

Must preserve:

- Tensor shape and layout semantics.
- Numeric precision requirements.
- Autodifferentiation semantics.
- Model and graph dependencies.
- Training and inference requirements.
- Randomness and reproducibility contracts.
- Accelerator capability requirements.

Must reuse canonical tensor and AI representations.

The compiler must not introduce another tensor type system.

10.5 Hybrid

Must preserve cross-domain boundaries.

Examples include:

- Classical-to-quantum data transfer.
- Quantum measurement feeding classical control.
- AI inference controlling classical execution.
- HDL interfaces connected to software.
- Distributed classical and quantum workloads.

Dependencies, synchronization, effects, ownership and transfer requirements must remain explicit.

10.6 Distributed

Must preserve:

- Communication dependencies.
- Ordering.
- Consistency requirements.
- Failure semantics.
- Data movement.
- Identity.
- Security.
- Provenance.

Compiler planning must not claim that a logical distributed schedule guarantees actual network availability.

---

11. Portability contracts

POCO-REAF requires explicit portability guarantees.

The compiler must distinguish:

Level| Guarantee
Source portability| The same source is accepted by compatible implementations.
Semantic portability| Program meaning is preserved.
IR portability| A compatible canonical representation can be transported and verified.
Artifact portability| A compiled artifact can be consumed by a compatible environment.
Execution portability| The program can execute on an available compatible target.

These guarantees must be represented in artifact metadata.

The compiler must not claim execution portability when only source or IR portability has been established.

11.1 Portable compilation

The compiler must preserve target-independent semantics before target-specific realization.

Target-specific compilation must not irreversibly contaminate a supposedly universal artifact unless explicitly represented as a target-specific artifact.

11.2 Unsupported realizations

When a target cannot satisfy a mandatory requirement, the compiler must:

1. Identify the unsatisfied requirement.
2. Preserve the relevant evidence.
3. Report the incompatible target.
4. Avoid silently changing program semantics.
5. Return an explicit infeasibility or unsupported-realization result.

Permitted alternatives may only be used when the program's semantics and explicit policies authorize them.

---

12. Determinism and reproducibility

Equivalent compilation inputs must produce equivalent results under the declared determinism contract.

The relevant inputs include:

- Source contents.
- Semantic model version.
- Grammar and language version.
- Compiler configuration.
- Compilation policies.
- Pass versions.
- IR schemas.
- Backend versions.
- Toolchain versions.
- Relevant provider metadata.
- Explicit target intent.
- Reproducibility settings.

The compiler must not depend silently on:

- Host machine identity.
- Unrecorded environment variables.
- Uncontrolled random values.
- Wall-clock timestamps.
- Unstable registry enumeration.
- Nondeterministic iteration order.

Where nondeterminism is explicitly permitted, it must be declared and recorded.

Artifact fingerprints must include every input that can materially affect the result.

---

13. Concurrency and cancellation

Compiler concurrency must be safe and deterministic according to the applicable contract.

Requirements:

1. Independent compilation work may execute concurrently.
2. Dependency ordering must always be respected.
3. Shared budget accounting must be synchronized.
4. Cancellation must propagate.
5. Failed prerequisites must block dependent work.
6. Worker failures must remain visible.
7. Partial results must not be represented as complete results.
8. Parallel execution must not change semantic meaning.
9. Worker availability must be discovered or explicitly configured.
10. No arbitrary universal worker-count ceiling may be introduced.

Use the existing "src/compiler/parallel/" implementation.

Do not create another worker pool.

---

14. Security and safety

The entire compiler subsystem must enforce:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

No "unsafe" implementation is permitted.

The compiler must validate all externally supplied:

- Source inputs.
- Configuration.
- Provider metadata.
- Capability evidence.
- Resource evidence.
- IR.
- Artifact data.
- Serialized objects.
- Backend results.
- Extension metadata.

Untrusted provider output must not be treated as trusted compilation evidence.

Backend invocation must preserve the trust boundary.

No security check may be silently skipped.

The compiler must fail explicitly when a required security property cannot be established.

---

15. Public API and legacy migration

The current repository has legacy compilation entry points in "src/lib.rs".

These include:

"compile(source)"

The legacy function uses the lexer, parser, semantic analyzer and IR generator.

The canonical native frontend is:

"src/frontend/"

The canonical semantic subsystem is:

"src/frontend/semantic/"

The legacy and native paths must not become permanent competing semantic authorities.

Migration must follow this sequence:

1. Establish the canonical frontend API.
2. Establish the canonical semantic result contract.
3. Establish canonical IR construction.
4. Verify the native source-to-IR path.
5. Integrate the production compiler driver.
6. Adapt legacy public entry points.
7. Add compatibility and differential tests.
8. Deprecate legacy paths only after verified migration.
9. Remove obsolete implementations only after all consumers are migrated.

Existing public callers must not be broken accidentally.

The public API must not claim to use the canonical production pipeline while actually using the legacy path.

---

16. Compilation failure and error contracts

Every compiler subsystem must distinguish:

- Source-language errors.
- Configuration errors.
- Semantic errors.
- Verification failures.
- Capability incompatibility.
- Resource infeasibility.
- Missing providers.
- Backend incompatibility.
- Backend invocation failures.
- Artifact validation failures.
- Linking failures.
- Security failures.
- Cancellation.
- Budget exhaustion.
- Internal compiler errors.

Errors must be structured.

They must preserve causal information.

No stage may convert an error into an empty successful result.

No failed verification may be reported as successful.

No incomplete analysis may certify a program as valid.

No unavailable target may be silently substituted.

---

17. Required testing architecture

Testing must establish actual integration rather than merely test module imports.

17.1 Compiler unit tests

Every module must test:

- Valid input.
- Invalid input.
- Boundary conditions.
- Overflow.
- Missing dependencies.
- Incompatible versions.
- Cancellation.
- Resource exhaustion.
- Determinism.
- Concurrency where applicable.

17.2 Integration tests

Required integration paths include:

Grammar
  -> Frontend
  -> Semantic Model
  -> ZUIR
  -> Verification
  -> Analysis
  -> Optimization
  -> Planning
  -> Lowering
  -> Backend
  -> Artifact Validation

And:

ZUIR
  -> Classical IR
  -> Classical Backend
  -> Artifact

ZUIR / Quantum Domain Input
  -> Quantum IR
  -> Quantum Verification
  -> Quantum Lowering
  -> Compatible Backend

HDL Input
  -> HDLIR
  -> HDL Validation
  -> HDL Lowering
  -> Synthesis Backend

Hybrid Input
  -> Cross-domain Analysis
  -> Partitioning
  -> Dependency Planning
  -> Domain Lowering
  -> Compatible Backends

Distributed Input
  -> Dependency Analysis
  -> Partitioning
  -> Abstract Scheduling
  -> Deployment Handoff

Every path must be tested using actual registered implementations.

A mock backend may be used for isolated unit testing, but must never count as evidence of production backend support.

17.3 Canonical fixtures

The testing architecture must provide fixtures for:

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

Fixtures must be valid against the actual grammar and semantic specifications.

17.4 Scalability testing

Test:

- Increasing source size.
- Increasing module count.
- Increasing operation count.
- Increasing dependency graph size.
- Increasing target-provider count.
- Increasing resource dimensions.
- Increasing domain composition.
- Increasing parallel work.
- Large artifact sets.
- Large incremental dependency graphs.

Tests must establish behavior under configured budgets and actual process resources.

No benchmark may be used to justify a universal hardware ceiling.

17.5 Differential testing

Where independent reference implementations exist, compare semantic and compilation results.

Differences must be classified and explained.

A differential mismatch must never be silently accepted.

17.6 Fuzzing

Use the existing:

"src/compiler/fuzzing_harness.rs"

Fuzz:

- Source input.
- Parser-to-AST boundaries.
- Semantic input.
- IR adapters.
- IR verification.
- Optimization transformations.
- Stage execution.
- Backend input.
- Artifact serialization.
- Deserialization.
- Extension metadata.

All fuzzing results must preserve reproducible failure information.

---

18. Build and quality gates

The minimum quality gates are:

cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets -- -D warnings

The minimum supported Rust version must be tested.

The latest stable Rust version should also be tested.

The repository must progressively remove broad lint allowances from "src/lib.rs", including:

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(clippy::all)]

These allowances must not hide unfinished compiler implementations.

The crate root must enforce the safe-Rust requirement.

The grammar build and parser-generation process must be tested independently and integrated into CI.

All declared compiler modules must compile.

All declared public interfaces must resolve.

All supported end-to-end compilation paths must pass.

---

19. File completion and freeze contract

Every compiler source file must have a documented completion contract.

Before implementation begins, the following must be established.

Section| Required definition
Purpose| Exactly what the file owns.
Non-responsibilities| What the file must not implement.
Canonical authorities| Existing repository owners it consumes.
Public API| All required types, functions and traits.
Dependencies| Exact upstream and downstream modules.
Ownership| Borrowing, mutation, cloning and lifetime rules.
Scalability| No arbitrary universal hardware limits.
Errors| Complete structured error behavior.
Determinism| Required output and ordering guarantees.
Concurrency| Synchronization and thread-safety rules.
Cancellation| Cancellation behavior.
Security| Trust boundaries and validation.
Integration| Exact consuming and consumed files.
Compatibility| Existing public API obligations.
Tests| Unit, integration, failure and regression tests.
Completion| Objective freeze conditions.

19.1 Freeze rules

A file may be considered frozen when:

1. Its ownership is unambiguous.
2. Its public interfaces are complete.
3. All dependencies are known.
4. All consumers are identified.
5. Its errors are structured.
6. Its invariants are tested.
7. Its scalability requirements are satisfied.
8. Its security contract is established.
9. Its compatibility obligations are satisfied.
10. Its integration tests pass.
11. It does not depend on unfinished speculative APIs.
12. Its implementation matches its documentation.

Freezing a file does not mean it can never change.

A discovered architectural defect must be corrected through an explicit contract change.

Silent breaking changes are prohibited.

---

20. Implementation sequence

Implementation must proceed from independent foundations toward dependent orchestration.

The order below establishes contracts before their consumers.

Phase 0 — Architecture and authority audit

Files:

- "src/compiler/documentation/INTEGRATION_CONTRACT.md"
- "src/compiler/mod.rs"
- "src/compiler_types.rs"
- "src/lib.rs"
- "grammar/DESIGN.md"
- "grammar/specification/semantic-model.md"

Establish:

- Canonical authorities.
- Public APIs.
- Dependency direction.
- Error contracts.
- Versioning.
- Safe Rust.
- Scalability rules.
- Legacy migration.

Resolve conflicting or contaminated specifications before implementation.

Phase 1 — Independent foundations

Start with:

- "compiler_types.rs"
- "configuration/budget.rs"
- "configuration/target.rs"
- "configuration/config.rs"
- "configuration/policy.rs"
- "configuration/profile.rs"
- "configuration/validation.rs"

Then:

- "diagnostics/aggregation.rs"
- "diagnostics/rendering.rs"
- "diagnostics/reporting.rs"

These files establish reusable contracts for downstream subsystems.

Phase 2 — Context and evidence

Complete:

- "context/compilation_state.rs"
- "context/dependency_context.rs"
- "context/provenance_context.rs"
- "context/compilation_context.rs"
- "analysis/requirements.rs"
- "analysis/capabilities.rs"
- "analysis/cost.rs"
- "analysis/provenance.rs"

Phase 3 — Analysis and IR

Complete:

- "analysis/program.rs"
- "analysis/dependency.rs"
- "analysis/effects.rs"
- "analysis/ownership.rs"
- "analysis/feasibility.rs"
- "ir/compatibility.rs"
- "ir/input.rs"
- "ir/adapters.rs"
- "ir/verification.rs"
- "ir/translation.rs"
- "ir/lowering.rs"

Phase 4 — Pass management and optimization

Complete:

- "passes/registry.rs"
- "passes/manager.rs"
- "passes/ordering.rs"
- "passes/pipeline.rs"
- "passes/legality.rs"
- "passes/preservation.rs"
- "passes/analysis_manager.rs"
- "optimization/cost_model.rs"
- "optimization/decisions.rs"
- "optimization/planning.rs"
- "optimization/policies.rs"
- "optimization/profitability.rs"
- "optimization/validation.rs"

Integrate existing optimization implementations.

Phase 5 — Backend and planning foundations

Complete:

- "backends/registry.rs"
- "backends/capabilities.rs"
- "backends/discovery.rs"
- "backends/compatibility.rs"
- "planning/target_discovery.rs"
- "planning/capability_matching.rs"
- "planning/resource_planning.rs"
- "planning/feasibility.rs"
- "backends/selection.rs"
- "backends/invocation.rs"

Phase 6 — Heterogeneous planning

Complete:

- "planning/partitioning.rs"
- "planning/data_movement.rs"
- "planning/routing.rs"
- "planning/scheduling.rs"

Integrate existing distributed and domain-specific authorities.

Phase 7 — Pipeline

Complete:

- "pipeline/stage.rs"
- "pipeline/stages.rs"
- "pipeline/ordering.rs"
- "pipeline/execution.rs"
- "pipeline/failure.rs"
- "pipeline/recovery.rs"
- "pipeline/orchestration.rs"

Prove actual end-to-end stage registration and execution.

Phase 8 — Parallel and incremental compilation

Complete:

- "parallel/dependencies.rs"
- "parallel/workers.rs"
- "parallel/scheduler.rs"
- "parallel/determinism.rs"
- "incremental/cache.rs"
- "incremental/fingerprints.rs"
- "incremental/invalidation.rs"
- "incremental/persistence.rs"

Reuse canonical dependency and cache authorities.

Phase 9 — Artifacts and linking

Complete:

- "artifacts/artifact.rs"
- "artifacts/manifest.rs"
- "artifacts/metadata.rs"
- "artifacts/portability.rs"
- "artifacts/serialization.rs"
- "artifacts/validation.rs"
- "linking/symbols.rs"
- "linking/composition.rs"
- "linking/relocation.rs"
- "linking/validation.rs"

Phase 10 — Security, profiling and assurance

Complete:

- "security/policy.rs"
- "security/verification.rs"
- "security/supply_chain.rs"
- "security/audit.rs"
- "profiling/events.rs"
- "profiling/measurements.rs"
- "profiling/aggregation.rs"

Phase 11 — Public compiler integration

Complete:

- "api/request.rs"
- "api/response.rs"
- "api/session.rs"
- "api/compatibility.rs"
- "api/compiler.rs"
- "api/mod.rs"
- "compiler/mod.rs"

Integrate the canonical frontend and remove competing legacy compilation paths through controlled migration.

Phase 12 — Production qualification

Complete cross-domain conformance, security, reproducibility, scalability, backend and runtime integration testing.

Do not declare unsupported domains production-ready.

---

21. New directories: controlled extensibility

The existing compiler architecture already contains registries, dependency contexts and graph-related infrastructure.

Therefore, do not automatically introduce competing directories such as:

- "compiler/core/"
- "compiler/engine/"
- "compiler/ir/" as a new IR implementation.
- "compiler/graph/" as a competing dependency graph.
- "compiler/parallel_build.rs" as a second parallel system.

Only introduce a new directory when an actual architectural gap is demonstrated.

A future "extensions/" subsystem may be justified if the existing provider and pass registries do not provide sufficient common lifecycle, versioning, validation and isolation contracts.

Before creating it, audit:

- "backends/registry.rs"
- "passes/registry.rs"
- Existing backend provider contracts.
- Existing extension and plugin authorities.
- "security/supply_chain.rs"

Any new subsystem must reuse those authorities.

No duplicate registry is permitted.

---

22. Production acceptance criteria

The compiler is not production-ready merely because its files exist or their modules compile independently.

The following conditions are mandatory.

22.1 Architectural correctness

- One canonical source-language semantic authority.
- One canonical universal IR authority.
- Authoritative domain IRs.
- Explicit integration contracts.
- No competing compiler architectures.
- No hidden legacy production path.

22.2 Functional correctness

- Real frontend-to-IR compilation.
- Verified IR.
- Semantics-preserving optimization.
- Actual target feasibility assessment.
- Real backend invocation.
- Valid artifact generation.
- Correct linking.
- Correct failure propagation.

22.3 POCO-REAF

- Target-independent semantic representation.
- Portable artifact contracts.
- Open-world target and backend registration.
- Extensible capability matching.
- Explicit infeasibility.
- Correct target-specific lowering.
- Runtime revalidation.
- Reproducibility.
- No arbitrary universal hardware ceilings.

22.4 Domain correctness

- Classical compilation qualification.
- Quantum compilation qualification.
- HDL compilation qualification.
- AI and tensor qualification.
- Hybrid compilation qualification.
- Distributed compilation qualification.
- Cross-domain integration qualification.

22.5 Engineering correctness

- Rust 1.97+.
- Edition 2021.
- No unsafe Rust.
- No hidden lint suppression.
- Complete tests.
- Deterministic outputs where required.
- Secure artifact handling.
- Documented compatibility.
- Verified end-to-end integration.

22.6 Release decision

A domain is production-ready only when its complete implementation path has been verified.

A planned feature, declared module, grammar production, mock backend or documentation entry is not sufficient evidence.

---

23. Final architectural invariant

The Zamani compiler must remain an extensible, target-independent compiler coordination system.

Its responsibility is to transform validated language meaning into verified, portable and appropriately realized compilation artifacts.

It must not confuse:

- Syntax with semantics.
- Semantics with IR.
- IR with target representation.
- Requirements with capabilities.
- Preferences with requirements.
- Feasibility with availability.
- Planning with allocation.
- Scheduling plans with runtime scheduling.
- Backend registration with compatibility.
- Artifact generation with successful execution.
- Portability with universal physical compatibility.

The architecture must permit Zamani programs to scale from tiny execution environments to increasingly large heterogeneous systems as resources, capabilities and compatible implementations become available.

No arbitrary universal machine-capacity ceiling may be introduced.

All unsupported capabilities, unavailable resources, representation limitations, budget exhaustion and failed verification must remain explicit.

The final objective is:

Program Once. Compile Once. Preserve Meaning. Realize Everywhere Compatible. Run Anywhere Supported. Remain Extensible Forever.

"Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever"

This document establishes the integration contract. Production readiness must be demonstrated through actual implementation, integration, conformance and release testing.