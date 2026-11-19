Zamani Toolchain — Production Build Contract

Canonical path: "src/toolchain/documentation/BUILD_CONTRACT.md"
Repository: "github.com/Benwellonedge28/Zamani"
Status: Normative implementation and integration contract
Language: Zamani
Rust edition: 2021
Minimum Rust version: 1.97
Safety: "unsafe" Rust is prohibited
Architecture: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

This document defines the authoritative production contract for the Zamani toolchain's build subsystem.

Its purpose is to establish a scalable, deterministic, extensible, secure and reproducible build architecture that coordinates the complete compilation lifecycle without duplicating existing compiler, frontend, semantic, intermediate-representation, backend, hardware, runtime or domain-specific implementations.

The toolchain MUST provide one consistent project and build experience across all supported Zamani computational domains.

It MUST support:

- Minimal classical programs.
- Large classical applications.
- Libraries and reusable components.
- Scientific and numerical applications.
- Parallel and concurrent computation.
- Embedded and resource-constrained applications.
- Quantum programs and quantum simulations.
- Quantum-classical hybrid programs.
- HDL and hardware/software co-design.
- FPGA and ASIC development workflows.
- AI, machine learning and tensor computation.
- GPU and other accelerator computation.
- Distributed and heterogeneous applications.
- Networking and cryptographic applications.
- Combinations of these domains.
- Future computational domains registered through supported extension contracts.

The architecture MUST NOT require a separate build system, package manager, dependency model or project format for each computing domain.

The build system MUST coordinate the existing authoritative implementations.

It MUST NOT create competing implementations merely to make the toolchain appear self-contained.

The central engineering objective is:

«One Zamani project, one consistent build model, reusable build specifications, and multiple verified target realizations, subject to semantic compatibility, implementation capabilities and available resources.»

---

2. Normative terminology

The terms below are normative.

- MUST / REQUIRED: Mandatory for conformance.
- MUST NOT / PROHIBITED: Forbidden behavior.
- SHOULD: Strong recommendation requiring documented justification when not followed.
- SHOULD NOT: Discouraged behavior requiring documented justification.
- MAY: Optional behavior.
- Build: The complete toolchain operation that validates project inputs, resolves dependencies, constructs and executes a build plan, invokes the compiler and required providers, validates outputs and produces a build result.
- Build request: An immutable description of the requested build operation.
- Build plan: A validated description of the work necessary to satisfy a build request.
- Build task: An independently identifiable operation within the toolchain's build orchestration.
- Build graph: The dependency graph of toolchain build tasks.
- Build artifact: A generated or transformed output associated with a verified build.
- Target realization: A concrete implementation of program semantics for a compatible target environment.
- Provider: A registered implementation of an extensible toolchain, backend, registry, artifact or deployment contract.
- Capability: An operation or property that an implementation or execution environment can provide.
- Requirement: A mandatory condition for a valid build or realization.
- Constraint: A condition that acceptable realizations MUST satisfy.
- Preference: A non-mandatory optimization objective.
- Hint: Optional implementation guidance that cannot change program semantics.
- Operational budget: A configurable resource or time policy governing an operation.
- Feasibility: An evidence-based assessment of whether the requested computation can be realized under the applicable requirements, capabilities, constraints, policies and resources.
- Provenance: Evidence describing the origin, inputs, transformations, providers, verification and outputs of a build.
- Reproducibility: The ability to establish that builds use equivalent declared inputs and satisfy the requested reproducibility guarantee.

A file's presence, module declaration, grammar production, interface definition or documentation MUST NOT be interpreted as proof that its functionality is implemented.

A build feature is production-ready only when its implementation, integration and acceptance evidence satisfy this contract.

---

3. Architectural authority and repository integration

3.1 Ownership hierarchy

The build subsystem operates downstream of the canonical language definition and upstream of target realization and execution.

The authoritative dependency direction is:

Zamani source
     |
     v
Canonical grammar and language specification
     |
     v
Frontend lexer and parser
     |
     v
Canonical AST
     |
     v
Semantic validation
     |
     v
Canonical semantic model
     |
     v
Canonical domain representations
     |
     v
Compiler services
     |
     v
Toolchain build orchestration
     |
     +-----------------------------+
     |                             |
     v                             v
Dependency and project        Build/task planning
validation                         |
     |                             |
     +--------------+--------------+
                    |
                    v
             Compiler API
                    |
                    v
             Compiler pipeline
                    |
                    v
             Backend integration
                    |
                    v
          Verified target artifacts
                    |
                    v
        Artifact and provenance validation
                    |
                    v
             Build result/report
                    |
                    v
          Runtime/deployment handoff

The toolchain coordinates the lifecycle. It does not replace the authorities participating in it.

3.2 Authoritative repository boundaries

Repository location| Authority| Build subsystem responsibility
"grammar/"| Language grammar architecture and language specifications| Consume language and compilation contracts
"grammar/specification/"| Normative language meaning| Preserve semantic requirements
"grammar/spec/"| Machine-oriented language contracts| Consume validated contracts
"grammar/OWNERSHIP.yaml"| Grammar symbol ownership| Respect canonical source ownership
"grammar/DEPENDENCIES.yaml"| Grammar dependency direction| Respect language architecture
"grammar/MANIFEST.yaml"| Grammar inventory and production gates| Use as language-definition inventory
"grammar/Zamani.g4"| Top-level grammar composition| Consume canonical source syntax
"grammar/antlr/ZamaniLexer.g4"| Public lexer boundary| Never introduce another lexer
"grammar/antlr/ZamaniParser.g4"| Public parser composition| Never introduce another parser
"grammar/lexer/"| Lexical definitions and token composition| Consume generated/canonical frontend output
"src/frontend/"| Frontend and semantic processing| Consume validated frontend results
"src/zuir/"| Universal target-independent IR| Never redefine universal IR
"src/classical/"| Classical computational representations| Coordinate through compiler integration
"src/quantum/ir/"| Canonical quantum IR| Coordinate quantum compilation
"src/hdl/"| HDL and hardware-description implementation| Coordinate hardware compilation
"src/ai/"| AI computational implementation| Coordinate AI and tensor compilation
"src/hybrid/"| Hybrid computation| Coordinate cross-domain compilation
"src/distributed/"| Distributed computation| Coordinate distributed compilation
"src/compiler/"| Compiler implementation and public compilation services| Invoke canonical compiler APIs
"src/backend/"| Target-specific backend realization| Invoke registered backend implementations
"src/zhal/"| Physical hardware abstraction and providers| Consume hardware capabilities through canonical interfaces
"src/zur/"| Runtime and execution infrastructure| Hand off validated artifacts through runtime contracts
"src/zamani_project_config/"| Canonical project configuration and manifest model| Adapt existing manifest types without duplicating parsing

The build subsystem MUST NOT redefine an authority merely because the authority does not yet expose the complete API required by the toolchain.

Missing interfaces MUST be addressed through explicit integration work in their owning subsystem.

3.3 Existing repository state relevant to this contract

The inspected repository contains:

1. "src/toolchain/mod.rs", currently empty.
2. "src/compiler/api/mod.rs", which defines a public compiler-facing API and exposes existing compilation entry points.
3. "src/compiler/incremental/mod.rs", which coordinates incremental compilation and existing cache/invalidation infrastructure.
4. "src/compiler/artifacts/mod.rs", which composes artifact management.
5. "src/backend/toolchain.rs", which owns external-tool discovery, version validation and invocation integration.
6. "src/backend/process.rs", which owns safe external-process execution, cancellation, timeout and output capture.
7. "src/backend/config.rs", which defines backend-specific configuration and operational policies.
8. "src/zamani_project_config/mod.rs", which currently parses a limited "Zamani.toml" package/dependency model.
9. "src/bin/zamani_pkg_main.rs", which contains an existing package-manager CLI.
10. "Cargo.toml", which declares the "buildsystem" feature and a "zamani-pkg" binary requiring that feature.

The current package-manager CLI explicitly describes its build invocation and some registry operations as simulated.

It imports:

use zamani_compiler::toolchain::build;
use zamani_compiler::toolchain::package_manager::PackageManager;

However, the inspected "src/toolchain/mod.rs" is empty, and the inspected "src/lib.rs" does not declare "pub mod toolchain;".

Therefore, the current package-manager interface MUST NOT be treated as a working production build API.

The existing CLI's simulated success behavior MUST be removed.

The implementation MUST reconcile the library module declaration, feature gating, toolchain exports and binary imports before the package-manager binary can be considered integrated.

These findings are implementation prerequisites, not claims that all other compiler or backend components have passed runtime validation.

---

4. Core architectural responsibility

The toolchain build subsystem owns:

- Build request creation and validation.
- Project and workspace build coordination.
- Canonical manifest integration.
- Build configuration and profile selection.
- Dependency-resolution orchestration.
- Build-plan construction.
- Build-task graph construction.
- Task-provider registration and execution coordination.
- Build lifecycle state.
- Build cancellation coordination.
- Build-level operational budgets.
- Compiler API invocation.
- Backend and external-tool coordination.
- Target build matrices.
- Incremental-build coordination.
- Artifact collection and validation coordination.
- Build provenance aggregation.
- Reproducibility assessment coordination.
- Structured build diagnostics.
- Build result reporting.

The toolchain does NOT own:

- Source-language grammar.
- Lexer implementation.
- Parser implementation.
- AST construction.
- Semantic analysis.
- Type checking.
- Effect analysis.
- Capability semantics.
- Canonical domain IR definitions.
- Compiler optimization algorithms.
- Compiler transformation algorithms.
- Quantum routing.
- Quantum error correction.
- Physical resource allocation.
- Backend code-generation algorithms.
- External process execution.
- Runtime scheduling.
- Physical program execution.
- Hardware discovery algorithms.
- Cryptographic implementation algorithms.
- Package dependency graph algorithms outside the package dependency authority.

The toolchain MUST invoke the owning subsystem through an explicit, validated integration contract.

---

5. POCO-REAF build contract

5.1 Fundamental requirement

A Zamani program MUST be written independently of incidental hardware details whenever those details are not part of the program's explicit semantics.

A developer MUST be able to request different compatible realizations without rewriting the source program merely because the physical target changes.

The toolchain MUST preserve the identity of the logical source project and its declared dependency set across compatible target realizations.

The target-specific artifact may differ.

The implementation strategy may differ.

The machine code, binary representation, quantum circuit, HDL output, scheduling plan, or deployment package may differ.

The specified program semantics MUST NOT silently change.

5.2 One source, multiple realizations

The build architecture MUST distinguish:

1. Source project identity.
2. Source content identity.
3. Manifest identity.
4. Resolved dependency identity.
5. Logical build request identity.
6. Build plan identity.
7. Target realization identity.
8. Target-specific artifact identity.
9. Artifact verification evidence.
10. Build provenance identity.

These identities MUST NOT be conflated.

A multi-target build MUST NOT generate unrelated project identities merely because different targets are selected.

Instead:

One Zamani project
        |
        v
One source identity
        |
        v
One manifest identity
        |
        v
One resolved dependency identity
        |
        v
One logical build request
        |
        v
One validated build plan
        |
        +-----------------------+
        |           |           |
        v           v           v
     Target A    Target B    Target C
        |           |           |
        v           v           v
    Artifact A  Artifact B  Artifact C
        |           |           |
        +-----------+-----------+
                    |
                    v
          Shared build provenance

The target-specific artifact identities MUST remain distinct.

5.3 Meaning of Compile Once

The term Compile Once MUST NOT be interpreted as a requirement that one physical machine-code binary must execute on every architecture.

It means that the developer's logical program and validated build specification can be reused across compatible realizations.

The compiler MAY perform target-specific lowering, specialization, optimization, code generation and artifact generation.

The toolchain MUST preserve traceable relationships between all outputs and the shared source and dependency identity.

5.4 Honest portability

A successful source-level validation does not prove that every target can execute the program.

A successful compilation does not prove that the resulting artifact can run on every environment.

A conforming toolchain MUST report target-specific feasibility and implementation support accurately.

If a requested realization is unavailable, the toolchain MUST report the actual limitation.

It MUST NOT silently replace required quantum computation with classical computation, required hardware behavior with simulation, or required distributed semantics with local execution.

Fallback is permitted only when the source semantics, declared constraints, applicable policies and developer request explicitly allow it.

5.5 Future compatibility

The toolchain SHOULD preserve portable project specifications across compiler, provider, backend and hardware evolution.

A future target MUST be integrable through the canonical provider and capability contracts without requiring the project manager, package manager or universal build orchestration model to be rewritten.

---

6. Scalability and resource independence

6.1 No arbitrary universal capacity ceilings

The build subsystem MUST NOT impose arbitrary universal limits on:

- CPUs.
- Processor cores.
- Threads.
- GPUs.
- FPGAs.
- ASICs.
- QPUs.
- Quantum processors.
- Physical qubits.
- Logical qubits.
- Accelerators.
- Devices.
- Distributed nodes.
- Processes.
- Tasks.
- Workers.
- Modules.
- Functions.
- Dependencies.
- Targets.
- Build artifacts.
- Tensor dimensions.
- Tensor rank.
- Register width.
- Memory capacity.
- Storage capacity.
- Source-file size.
- Project size.
- Build graph size.
- Provider count.
- Computing domains.

The prohibition includes renamed constants, hidden fixed arrays, closed provider inventories, fixed worker counts and arbitrary collection truncation.

The implementation MUST use appropriately sized, dynamically extensible data structures and streaming or incremental processing where appropriate.

6.2 Physical and implementation limits

The absence of universal semantic ceilings does not mean that actual machines have unlimited resources.

Every concrete operation remains subject to:

- Available RAM.
- Available address space.
- Available storage.
- Available processor time.
- Available accelerator resources.
- Available network capacity.
- Available external-tool capabilities.
- Operating-system constraints.
- Provider-specific limits.
- Numeric representation limits.
- Configured operational budgets.
- Security policies.

Resource exhaustion MUST be reported explicitly.

It MUST NOT be reported as semantic invalidity unless the language's actual semantic requirements have been violated.

6.3 Operational budgets

Operational budgets are permitted and REQUIRED where appropriate.

They MUST be configurable, scoped and observable.

Examples include:

- Maximum build duration.
- Maximum memory consumption.
- Maximum retained process output.
- Maximum download size.
- Maximum archive extraction size.
- Maximum concurrent task count.
- Maximum retry count.
- Maximum cache storage.
- Maximum temporary storage.
- Maximum network transfer.
- Maximum log retention.
- Maximum individual compiler-stage duration.

The system MUST distinguish:

- No configured limit.
- A configured zero budget.
- A finite configured budget.
- An unknown or unmeasurable resource.
- An unavailable resource.
- A resource-allocation failure.

An absent budget MUST NOT be interpreted as proof of infinite capacity.

A finite budget MUST NOT become an implicit language-level limitation.

Budget enforcement belongs to the component capable of observing or constraining the relevant resource.

6.4 Tiny-to-large scaling

The build architecture MUST support both:

- Small builds with minimal orchestration overhead.
- Large builds requiring incremental compilation, task partitioning, parallel execution, streaming, distributed build providers and resumable operations where safe.

It SHOULD avoid requiring an enterprise-scale infrastructure for a minimal classical program.

It MUST NOT require a small project to initialize every domain provider.

Unused computational domains SHOULD remain inactive.

Large builds MUST be able to use registered parallel or distributed execution providers when available and permitted.

The architecture MUST NOT assume that all available resources are local to one process or machine.

6.5 Graceful resource exhaustion

When resources become unavailable during a build, the system MUST:

1. Identify the affected task and resource category.
2. Preserve completed valid work where safe.
3. Stop dependent tasks that cannot continue.
4. Cancel or terminate affected external operations through their authoritative cancellation interfaces.
5. Preserve diagnostic and provenance evidence.
6. Mark incomplete outputs as incomplete.
7. Avoid committing incomplete artifacts as successful outputs.
8. Report whether retry or resumption is safe.
9. Release resources owned by the failed operation.
10. Return a structured failure.

---

7. Build request contract

7.1 Required information

A build request MUST identify or resolve:

- Project identity.
- Workspace identity, when applicable.
- Canonical manifest.
- Source inputs.
- Selected build profile.
- Feature selections.
- Dependency-resolution policy.
- Target intent.
- Requested output behavior.
- Security policy.
- Reproducibility policy.
- Applicable operational budgets.
- Cancellation context.
- Provider requirements, when explicitly required.

Optional fields MUST have documented defaults.

Defaults MUST NOT silently introduce a particular physical architecture, vendor or hardware dependency.

7.2 Immutable requests

Once validated, a build request MUST be immutable for the lifetime of that build invocation.

Changes to source, manifest, dependency lockfile, configuration or target selection MUST produce a new logical build request or an explicitly versioned update before execution begins.

A running build MUST NOT silently observe unrelated modifications to its input files and continue using a mixture of old and new inputs.

7.3 Target intent

Target intent MUST distinguish:

- Explicitly requested targets.
- Portable target profiles.
- Target constraints.
- Target preferences.
- Required capabilities.
- Optional capabilities.
- Provider requirements.
- Unspecified target information.

An unspecified target MUST NOT automatically mean that a particular native architecture is universally valid.

A host-native target MAY be offered as a user-visible convenience when the relevant backend can establish its identity and compatibility.

It MUST NOT be silently substituted for an explicitly requested target.

---

8. Canonical manifest and workspace integration

8.1 Manifest authority

"src/zamani_project_config/" remains the authoritative owner of Zamani project-manifest parsing and data structures.

"src/toolchain/manifest/" is an adapter and orchestration boundary only.

The toolchain MUST NOT introduce a second TOML parser or competing "ZamaniToml" type.

8.2 Current manifest limitations

The inspected "src/zamani_project_config/mod.rs" currently defines:

- "ZamaniToml".
- "PackageSection".
- "DependencyConfig".
- Package metadata.
- A basic dependency map.
- "ZamaniToml::parse_from_str".

Build, feature and deployment sections are not yet implemented in that file.

Consequently, the toolchain MUST NOT assume that complete build profiles, targets, registries, workspaces, packaging policies or deployment settings are already available through the manifest API.

The canonical manifest implementation MUST be extended in its owning subsystem before those fields become production build inputs.

8.3 Manifest validation

Before dependency resolution or compilation, the toolchain MUST:

1. Resolve the explicitly selected project root.
2. Load the canonical manifest.
3. Validate required fields.
4. Validate manifest version compatibility.
5. Validate referenced paths.
6. Validate declared features.
7. Validate dependency declarations.
8. Validate target and profile intent.
9. Validate security-relevant configuration.
10. Produce structured diagnostics for invalid entries.

Missing information MUST NOT be silently replaced with fabricated values.

8.4 Workspace behavior

Workspace discovery MUST operate relative to an explicit project path or validated workspace root.

It MUST NOT assume that the process working directory is always the project root.

Workspace membership MUST be validated before build-plan construction.

Path normalization and filesystem access MUST defend against traversal, unexpected symlink resolution and unauthorized access.

---

9. Dependency-resolution contract

9.1 Ownership

The toolchain owns the coordination of project dependency resolution.

The package dependency subsystem owns dependency identity, constraints, graph algorithms, version selection and lockfile behavior.

The compiler owns compilation-unit and compiler-internal dependency relationships.

These graphs MUST remain distinct.

In particular:

- Package dependency graphs are not compiler IR graphs.
- Build-task graphs are not package dependency graphs.
- Compiler compilation graphs are not runtime execution graphs.
- Physical hardware topology graphs are not toolchain build graphs.

9.2 Resolution requirements

Dependency resolution MUST support:

- Direct dependencies.
- Transitive dependencies.
- Local path dependencies.
- Registered package sources.
- Version constraints.
- Dependency features.
- Optional dependencies.
- Source identity.
- Compatibility requirements.
- Dependency conflicts.
- Cycle detection according to package semantics.
- Deterministic resolution.
- Lockfile generation.
- Lockfile verification.

The resolver MUST NOT silently select an incompatible dependency merely because it is available.

9.3 Lockfiles

A production lockfile MUST record enough information to reproduce the resolved dependency set.

It SHOULD include:

- Lockfile schema version.
- Package identity.
- Package version.
- Package source identity.
- Immutable content identity or equivalent integrity reference.
- Resolved dependency relationships.
- Resolved feature selections.
- Relevant compatibility information.
- Integrity verification information.

A lockfile MUST be validated before it is trusted.

A stale or invalid lockfile MUST be handled through an explicit resolution or update policy.

Dependency resolution MUST be deterministic for identical canonical inputs, resolver versions and resolution policies.

---

10. Build-plan contract

10.1 Plan construction

The build planner MUST transform a validated build request into an explicit build plan.

A plan MUST describe:

- Input identities.
- Required validation stages.
- Dependency-resolution results.
- Domain activation conditions.
- Target realization requests.
- Required providers.
- Build tasks.
- Task dependencies.
- Task inputs and outputs.
- Operational budgets.
- Cache eligibility.
- Reproducibility requirements.
- Artifact validation requirements.
- Failure and cancellation behavior.

The plan MUST be inspectable before execution.

10.2 Plan validity

A build plan is valid only when:

1. Its input references are resolved.
2. Its dependency requirements are satisfied or explicitly deferred under an allowed policy.
3. Its required stages have registered implementations.
4. Its task relationships are valid.
5. Its target intent has been validated.
6. Required provider contracts are available.
7. Its security policies permit the planned operations.
8. Its resource requirements have been assessed to the level required for that stage.
9. Its output contracts are defined.

A plan MUST NOT claim physical feasibility merely because its structure is valid.

10.3 Build graph ownership

"src/toolchain/build/graph.rs" owns the graph of toolchain build operations only.

"src/toolchain/tasks/graph.rs" owns the lower-level task execution relationships if that separation is retained in the final architecture.

These files MUST have a documented distinction and MUST NOT independently implement the same graph authority.

The preferred relationship is:

BuildPlan
    |
    v
BuildTaskGraph
    |
    v
TaskExecutor

The build graph MUST NOT duplicate:

- "src/compiler/graph/".
- "src/compiler/pipeline/ordering".
- "src/compiler/parallel/".
- "src/compiler/context/dependency_context".
- "src/zuir/" dependency or IR representations.

The build graph describes toolchain operations such as resolving dependencies, compiling a target, packaging outputs and validating artifacts.

Compiler-internal operations remain inside the compiler.

---

11. Build execution lifecycle

The production build lifecycle is:

1. Request received
       |
       v
2. Request validation
       |
       v
3. Workspace discovery
       |
       v
4. Manifest loading and validation
       |
       v
5. Source and input snapshot
       |
       v
6. Dependency resolution
       |
       v
7. Lockfile verification/update
       |
       v
8. Environment and provider validation
       |
       v
9. Target capability assessment
       |
       v
10. Build-plan construction
       |
       v
11. Build graph validation
       |
       v
12. Task execution
       |
       v
13. Canonical compiler API invocation
       |
       v
14. Backend and required external-tool execution
       |
       v
15. Artifact collection
       |
       v
16. Artifact verification
       |
       v
17. Provenance and reproducibility assessment
       |
       v
18. Transactional output commit
       |
       v
19. Structured build report

Not every project requires every domain-specific stage.

A stage MAY be inactive only when its activation contract establishes that it is genuinely inapplicable.

A required stage MUST NOT be silently skipped.

A stage marked inactive MUST NOT be reported as successfully executed.

A dependent task MUST NOT consume incomplete output from a failed prerequisite.

11.1 Validation is not compilation

Successful manifest validation MUST NOT be reported as a successful build.

Successful dependency resolution MUST NOT be reported as successful compilation.

Successful compiler invocation MUST NOT automatically establish artifact verification.

Successful artifact verification MUST NOT establish that an execution environment exists.

Each lifecycle stage MUST have its own status and evidence.

11.2 Real compilation only

The toolchain MUST invoke an actual canonical compiler service.

It MUST NOT:

- Return success without invoking compilation.
- Generate fabricated successful artifacts.
- Substitute a simulated backend invocation.
- Claim that a source project was compiled when only its manifest was parsed.
- Treat a mocked provider as evidence of real hardware compatibility.
- Treat an empty output as a valid artifact without an explicit artifact contract.

If the required compiler API is unavailable or fails, the build MUST fail explicitly.

11.3 Compiler API integration

The primary integration boundary is:

"src/compiler/api/mod.rs"

The toolchain MUST consume the existing public compiler API and preserve its existing authoritative types, including where applicable:

- "CompilerConfig".
- "CompilationTarget".
- "OptimizationLevel".
- "CompilerError".
- "CompilationOutput".
- "CompilationRequest".
- "CompilationResponse".
- "CompilerService".
- Existing "compile_source".
- Existing "compile_with_config".
- Existing "compile".

The final integration MUST use the exact public methods and request/response contracts present in the compiler API after a full implementation audit.

The toolchain MUST NOT depend on private compiler implementation details.

The toolchain MUST NOT directly invoke internal compiler passes to simulate a public compiler service.

If the existing compiler API cannot express a required build input, such as an explicit target matrix, verified provider selection or complete provenance context, the compiler API MUST be extended by its owning subsystem.

The toolchain MUST NOT create a parallel compiler configuration that silently overrides canonical compiler configuration.

11.4 Domain dispatch

Domain activation MUST be determined from validated semantic and compiler inputs.

It MUST NOT be determined through unreliable string matching against source code or arbitrary target-name conventions.

A project containing classical and quantum computation MUST preserve the semantic boundaries and interfaces of both domains.

A hybrid build MUST coordinate the required domain-specific compilation paths rather than flattening all computations into one artificial backend.

---

12. Multi-target build contract

12.1 Target matrix

A target matrix MUST support an extensible collection of target requests.

The architecture MUST NOT define a fixed number of targets per project or build.

Each target realization MUST have its own:

- Target identity.
- Provider identity.
- Capability assessment.
- Compatibility assessment.
- Compiler/backend configuration.
- Target-specific build inputs.
- Artifact outputs.
- Verification evidence.
- Failure status.
- Provenance information.

12.2 Shared and target-specific inputs

The system MUST distinguish common inputs from target-specific inputs.

Common inputs include:

- Source content.
- Canonical manifest.
- Resolved dependencies.
- Shared feature selections.
- Source-level semantic identity.
- Build-wide policy.

Target-specific inputs may include:

- Backend provider.
- Target profile.
- Lowering configuration.
- External toolchain fingerprints.
- Target ABI.
- Required artifact formats.
- Target-specific capabilities.
- Explicit target constraints.

A target-specific input change MUST invalidate only the affected target realizations where the canonical cache and invalidation contracts establish that the other outputs remain valid.

12.3 Partial success

A multi-target build MUST represent per-target outcomes.

For example:

Logical build: PARTIAL

Target A: SUCCESS
Target B: FAILURE
Target C: NOT ATTEMPTED

The toolchain MUST NOT report universal success because one target succeeded.

A partial build MUST be distinguishable from:

- Complete success.
- Complete failure.
- Cancelled build.
- Incomplete build.
- Unverified build.
- Deferred build.

A policy MAY require all requested targets to succeed before the overall operation is accepted.

---

13. Computing-domain integration

13.1 Classical computing

The build subsystem MUST support:

- Applications.
- Libraries.
- Modules.
- Native compilation.
- Cross-compilation.
- Debug and release configurations.
- Portable classical artifacts.
- Compatible CPU architectures.
- Embedded targets.
- Parallel compilation.
- Incremental compilation.

Integration:

- "src/classical/"
- "src/compiler/"
- "src/backend/"
- "src/compiler/api/"

The toolchain MUST NOT implement another classical IR or code generator.

13.2 Quantum computing

The build subsystem MUST support:

- Quantum source projects.
- Quantum dependency resolution.
- Quantum semantic validation.
- Canonical quantum IR integration.
- Quantum compilation profiles.
- Simulator-oriented artifacts.
- Physical QPU-oriented artifacts where providers exist.
- Quantum capability requirements.
- Quantum artifact metadata.
- Quantum verification evidence.

Integration:

- "src/quantum/"
- "src/quantum/ir/"
- Existing quantum backend and ZQN integration.
- "src/zhal/"
- "src/backend/"

The toolchain MUST NOT implement:

- Qubit allocation.
- Gate decomposition.
- Quantum routing.
- Quantum scheduling.
- QEC.
- Calibration.
- Physical QPU discovery.

Those responsibilities remain with their canonical authorities.

A simulator MUST NOT be silently substituted for a physical quantum target when the build request requires physical execution.

13.3 HDL and hardware

The build subsystem MUST support:

- HDL source projects.
- Hardware/software co-design.
- Hardware dependencies.
- Synthesis workflows.
- Simulation workflows.
- Verification workflows.
- FPGA and ASIC realization requests.
- Generated hardware artifacts.
- Timing and implementation reports when supplied by providers.

Integration:

- "src/hdl/"
- "src/hdl/hdlir/"
- "src/backend/"
- "src/zhal/"

The toolchain coordinates registered synthesis, simulation and verification providers.

It MUST NOT implement physical synthesis, placement, routing or timing closure itself.

13.4 AI and tensor computation

The build subsystem MUST support:

- AI computation projects.
- Machine-learning and deep-learning workflows.
- Model and tensor artifacts.
- Training and inference build requirements.
- Dataset references and integrity.
- Accelerator-specific realization requests.
- Edge and distributed AI deployment artifacts.
- Model metadata and provenance.

Integration:

- "src/ai/"
- Canonical tensor implementation.
- "src/compiler/".
- "src/backend/".
- "src/zur/".

The toolchain MUST NOT create a competing tensor storage system or independent AI semantic authority.

13.5 Hybrid computation

A hybrid build MUST preserve domain-specific semantics and cross-domain interfaces.

It MUST support coordinated outputs from compatible classical, quantum, AI, HDL and other domain implementations.

Cross-domain interfaces MUST have explicit compatibility contracts.

A hybrid build MUST NOT imply that every domain can execute within one physical device or one runtime.

13.6 Distributed computation

The build subsystem MUST support:

- Distributed project dependencies.
- Multi-component builds.
- Distributed artifact metadata.
- Communication and serialization contracts.
- Distributed deployment specifications.
- Multi-environment build coordination.
- Remote build providers where available.
- Reproducibility and provenance across participating providers.

Integration:

- "src/distributed/"
- "src/backend/"
- "src/zur/"
- Relevant networking and serialization authorities.

The toolchain MUST NOT become the distributed runtime scheduler.

13.7 Future domains

New computational domains MUST be integrated through open, versioned provider and capability contracts.

The core build orchestrator MUST NOT require modification merely to recognize another domain's vendor, device or implementation name.

A domain integration MUST supply:

1. Stable domain identity.
2. Semantic compatibility contract.
3. Compiler integration.
4. Target/provider requirements.
5. Build input and output contracts.
6. Artifact metadata.
7. Verification requirements.
8. Failure semantics.
9. Reproducibility behavior.
10. Conformance tests.

Unknown domains MUST be rejected or reported as unresolved when required.

They MUST NOT be silently interpreted as an existing domain.

---

14. Task execution contract

14.1 Task identity

Every build task MUST have a stable identity within its build invocation.

A task MUST declare:

- Task identity.
- Task kind.
- Input references.
- Output references.
- Prerequisites.
- Required provider.
- Operational budget.
- Cancellation context.
- Execution policy.
- Expected result type.
- Failure behavior.
- Provenance requirements.

Task identity MUST NOT depend on memory addresses or nondeterministic iteration order.

14.2 Execution

The task executor MUST:

- Validate prerequisites.
- Respect dependency ordering.
- Enforce task policies.
- Observe cancellation.
- Enforce applicable budgets.
- Preserve task results.
- Propagate failures.
- Prevent dependent tasks from consuming invalid outputs.
- Support safe parallel execution.
- Produce structured execution records.

14.3 Parallel execution

Parallel execution MUST be dependency-aware.

The system MUST NOT use an arbitrary universal worker count.

Worker availability and concurrency policy MUST be configurable or supplied by the execution provider.

Parallelism MUST preserve the required deterministic semantics of each task.

If two tasks have conflicting writes or shared mutable inputs, the executor MUST enforce an appropriate synchronization or isolation policy.

The build system MUST NOT duplicate "src/compiler/parallel/" or the compiler's parallel build scheduler.

The toolchain may execute independent toolchain tasks concurrently while delegating compiler-internal parallel work to the compiler.

14.4 Cancellation

Cancellation MUST be propagated to:

- Pending build tasks.
- Running task providers.
- Compiler requests where supported.
- External processes.
- Remote build providers where supported.

Cancellation is a distinct outcome.

A cancelled build MUST NOT be reported as an ordinary compilation failure or successful partial build.

Completed artifacts MAY be retained only if their validity and provenance are established.

14.5 Retry and recovery

Retries MUST require explicit retry policy.

A task MUST NOT be retried automatically merely because it failed.

Retry is permitted only when:

- The failure is classified as potentially transient.
- The operation is replay-safe or idempotent.
- Its inputs are unchanged.
- Its previous outputs have been invalidated or safely isolated.
- Its security policy permits the retry.
- The remaining operational budget permits it.

A non-idempotent external action MUST NOT be blindly repeated.

---

15. External toolchain integration

15.1 Existing authorities

The toolchain MUST reuse:

- "src/backend/toolchain.rs"
- "src/backend/process.rs"
- "src/backend/config.rs"

"backend/toolchain.rs" owns external executable discovery, tool identity and version validation.

"backend/process.rs" owns external process execution.

"backend/config.rs" owns backend-specific policy and configuration.

The toolchain MUST NOT duplicate these implementations.

15.2 External tool requirements

A build plan MUST declare required external tools through stable tool identifiers and version requirements.

A tool MUST be:

- Discovered.
- Validated.
- Authorized.
- Version-checked.
- Configured.
- Invoked through the canonical process interface.
- Associated with its actual execution result.

A tool's discovery MUST NOT automatically establish trust.

15.3 Process security

External tools MUST be invoked using executable paths and argument vectors.

The toolchain MUST NOT construct shell command strings by interpolating untrusted input.

Environment inheritance MUST be an explicit policy.

Sensitive credentials MUST NOT be written to logs, diagnostics or provenance records.

Timeout, cancellation and output capture MUST be configured through the process runner.

The standard-library process runner's direct-child termination limitations MUST be acknowledged. If a tool requires process-tree termination, an approved platform-specific supervisor or isolation provider is required.

The toolchain MUST NOT claim that cancelling a direct child necessarily terminates all detached descendants.

15.4 Failure handling

A missing executable, incompatible version, non-zero exit, timeout, cancellation, invalid output or unavailable provider MUST produce an explicit structured failure.

A tool's successful exit code alone MUST NOT prove that its output is a valid Zamani artifact.

The owning backend MUST validate its output.

---

16. Incremental build and caching

16.1 Ownership

The build subsystem coordinates incremental build requests.

It MUST reuse the existing:

- "src/compiler/incremental/"
- "src/zuir/incremental/"
- "src/zuir/fingerprint/"
- "src/backend/cache.rs"
- "src/compiler/artifacts/"

The toolchain MUST NOT create another fingerprint algorithm, cache store or invalidation graph.

16.2 Cache identity

Cache identity MUST account for all output-affecting inputs.

These may include:

- Source fingerprints.
- Manifest fingerprint.
- Resolved dependency fingerprints.
- Feature selections.
- Compiler identity and version.
- Compiler configuration.
- Grammar and semantic contract versions where relevant.
- IR schema versions where relevant.
- Backend identity and version.
- External toolchain fingerprints.
- Target profile and ABI.
- Relevant environment facts.
- Security and reproducibility policies.
- Domain-specific configuration.
- Provider configuration that affects outputs.

The exact canonical fingerprint representation MUST remain owned by its established fingerprint authority.

16.3 Cache correctness

A cache hit MUST be validated against the canonical cache-key and artifact-integrity contracts.

A cache hit MUST NOT bypass mandatory verification.

A cache entry MUST NOT be accepted merely because its filename or lookup key matches.

A cache miss MUST NOT cause build failure when ordinary compilation remains possible.

A failed cache insertion MUST NOT convert a failed compilation into success or invalidate a successfully produced artifact.

16.4 Invalidation

Changes to inputs MUST invalidate every dependent output according to the canonical dependency and invalidation systems.

Invalidation MUST NOT discard unrelated valid target realizations without evidence that they depend on the changed input.

A build artifact may be marked clean only after the relevant compilation and output commit have succeeded.

---

17. Artifact contract

17.1 Artifact ownership

Artifact structures and lifecycle integration MUST reuse "src/compiler/artifacts/" and the canonical backend artifact implementations.

The toolchain MAY coordinate packaging and collection of artifact references.

It MUST NOT create a competing compiler artifact authority.

17.2 Required artifact information

A production artifact MUST carry or reference the metadata required to establish:

- Artifact identity.
- Artifact kind.
- Source project identity.
- Source content identity.
- Build request identity.
- Target realization identity.
- Compiler identity.
- Relevant compiler version.
- Backend identity and version.
- ABI or interface compatibility information where applicable.
- Dependency identity.
- Relevant schema versions.
- Content integrity information.
- Provenance information.
- Verification status.

Domain-specific metadata MUST be extensible.

The universal artifact model MUST NOT require every domain to populate irrelevant fields.

17.3 Artifact verification

Artifact verification MUST establish that the artifact satisfies its declared format and integrity requirements.

Where a target or artifact requires a verifier, the build MUST fail if that verifier is unavailable and verification is mandatory.

The toolchain MUST distinguish:

- Verified.
- Verification failed.
- Verification unavailable.
- Verification not requested.
- Verification not applicable.
- Verification incomplete.

An unverified artifact MUST NOT be represented as verified.

17.4 Output commit

Artifact publication to the selected build-output location MUST be transactional where the underlying storage permits it.

The toolchain SHOULD stage outputs before committing them.

It MUST NOT expose incomplete files as final successful artifacts.

Existing files MUST NOT be overwritten unless the configured policy explicitly permits it.

Filesystem output handling MUST defend against path traversal, symlink attacks, unexpected replacement and unsafe archive paths.

---

18. Build determinism and reproducibility

18.1 Deterministic inputs

Identical canonical build inputs, dependency resolution, provider identities, relevant environment facts and deterministic policies SHOULD produce equivalent build plans.

Where byte-for-byte reproducibility is required and supported, the resulting artifacts MUST satisfy that guarantee.

The toolchain MUST NOT claim byte-for-byte reproducibility merely because two builds used the same source file.

18.2 Deterministic ordering

The toolchain MUST establish deterministic ordering where ordering affects:

- Dependency resolution.
- Build-plan serialization.
- Task graph serialization.
- Artifact manifests.
- Provenance.
- Cache keys.
- Diagnostics intended for comparison.
- Package contents.

Parallel completion order MUST NOT silently alter semantic results or canonical build identities.

18.3 Reproducibility levels

The build system SHOULD distinguish at least:

1. Input reproducibility.
2. Dependency reproducibility.
3. Build-plan reproducibility.
4. Toolchain/environment reproducibility.
5. Artifact reproducibility.
6. Execution reproducibility where the runtime and domain support it.

A request for stronger reproducibility MUST fail explicitly or return an accurately qualified result if the required guarantee cannot be established.

18.4 Provenance

Every production build MUST produce sufficient provenance to identify:

- Source inputs.
- Manifest.
- Dependency resolution.
- Compiler.
- Backend providers.
- External toolchains.
- Target realizations.
- Build configuration.
- Relevant environment fingerprints.
- Build stages.
- Artifact identities.
- Verification outcomes.
- Failures and retries where relevant.

Provenance MUST integrate with the canonical compiler and security authorities.

The toolchain MUST NOT create a second provenance ledger.

Sensitive credentials and secret values MUST NOT be recorded.

---

19. Security contract

19.1 Input trust

All external inputs MUST be treated as untrusted until validated.

This includes:

- Manifests.
- Dependency metadata.
- Package archives.
- Registry responses.
- Provider responses.
- External-tool output.
- Build scripts.
- Generated artifacts.
- Cache entries.
- Remote build results.

19.2 Authorization

The toolchain MUST enforce operation-specific authorization for:

- Dependency retrieval.
- External-tool invocation.
- Filesystem modification.
- Package installation.
- Package publishing.
- Artifact signing.
- Remote build execution.
- Deployment handoff.

Human and AI agent requests MUST pass through the same underlying authorization and validation services.

19.3 No unsafe Rust

All production Rust implementation under the build subsystem MUST remain compatible with the crate's:

#![forbid(unsafe_code)]

No new implementation may require:

- "unsafe" blocks.
- Unsafe traits.
- Unsafe trait implementations.
- Unsafe foreign-function wrappers.
- Unsafe memory manipulation.

If an external dependency introduces unsafe internals, its use MUST be evaluated under the repository's dependency and safety policy. The Zamani implementation MUST not use unsafe Rust to compensate for missing APIs.

19.4 Secrets

Secrets MUST be represented by protected credential references or approved secret-provider interfaces.

They MUST NOT be:

- Hard-coded.
- Committed to the repository.
- Included in lockfiles.
- Included in cache keys in plaintext.
- Printed in diagnostics.
- Persisted in build logs.
- Included in provenance as secret values.

---

20. Build result and lifecycle status

20.1 Required result distinctions

A build result MUST distinguish at least:

- Request rejected.
- Input validation failed.
- Dependency resolution failed.
- Environment validation failed.
- Target unresolved.
- Target infeasible.
- Build planning failed.
- Build running.
- Build succeeded.
- Build partially succeeded.
- Build failed.
- Build cancelled.
- Build deferred.
- Artifact verification failed.
- Provenance incomplete.

These states MUST have precise transition rules.

A status MUST NOT be assigned merely because a function returned without panicking.

20.2 Build success

A build may be reported as successful only when:

1. The request was validated.
2. Required inputs were identified.
3. Required dependencies were resolved.
4. The build plan was valid.
5. All required tasks completed successfully.
6. The canonical compiler actually performed the requested compilation.
7. Required backend operations completed.
8. Required artifacts were produced.
9. Required artifact verification succeeded.
10. Required provenance was recorded.
11. Outputs were committed successfully.
12. No mandatory stage remains incomplete.

Where an explicitly permitted policy relaxes a verification or reproducibility requirement, the report MUST identify that exception and the resulting assurance level.

20.3 Execution is not build success

A successful build does not mean that the program has been executed.

The toolchain MUST distinguish:

- Project validated.
- Dependencies resolved.
- Build plan validated.
- Compilation completed.
- Artifact verified.
- Execution environment validated.
- Resources acquired.
- Program launched.
- Program completed.

Execution belongs to the runtime.

Hardware allocation belongs to the appropriate resource and hardware authorities.

Deployment belongs to the deployment and infrastructure providers.

---

21. Error handling and diagnostics

All build failures MUST be represented through structured error types.

Errors MUST preserve their originating subsystem and causal information.

The build subsystem MUST distinguish:

- Invalid project.
- Invalid manifest.
- Invalid source input.
- Dependency conflict.
- Dependency source failure.
- Lockfile failure.
- Unsupported capability.
- Infeasible target.
- Missing provider.
- Incompatible provider.
- Missing external executable.
- External-tool version mismatch.
- Process failure.
- Timeout.
- Cancellation.
- Resource exhaustion.
- Compiler failure.
- Backend failure.
- Artifact verification failure.
- Cache failure.
- Provenance failure.
- Output commit failure.
- Internal orchestration failure.

Diagnostics SHOULD include:

- Stable diagnostic identifier.
- Severity.
- Owning subsystem.
- Relevant source location, where available.
- Build/task identity.
- Target identity, where applicable.
- Failure cause.
- Suggested corrective action where justified.

The build layer MUST preserve frontend and compiler source-location information.

It MUST NOT invent source locations when they are unavailable.

Machine-readable diagnostics MUST be supported independently of terminal rendering.

---

22. Build API and command integration

22.1 Shared API

The build subsystem MUST expose a stable toolchain-facing API.

The API MUST be usable by:

- "zamani-pkg".
- The main Zamani CLI.
- IDE integrations.
- Editor integrations.
- Automation.
- Human developers.
- AI agents.
- Remote build clients.
- Future developer tools.

All clients MUST invoke the same build services.

22.2 CLI

"src/bin/zamani_pkg_main.rs" MUST become a thin presentation and argument-parsing layer.

It MUST NOT implement:

- Compilation.
- Dependency resolution algorithms.
- Package archive extraction.
- Artifact verification.
- Registry protocols.
- External process execution.
- Independent manifest parsing.
- Simulated build success.

The CLI MUST call the canonical toolchain API and render its structured results.

22.3 Public library integration

"src/lib.rs" MUST expose the toolchain according to the agreed feature policy.

The existing "buildsystem" Cargo feature MUST govern the build-system public surface.

A suitable integration boundary is:

#[cfg(feature = "buildsystem")]
pub mod toolchain;

The implementation MUST verify that all required child modules and dependencies compile under that feature.

The toolchain MUST NOT require every lightweight compiler consumer to activate the complete package/build ecosystem.

22.4 Feature behavior

The following commands and services MUST be exposed only when their implementations and dependencies are ready:

- Build.
- Check.
- Clean.
- Run.
- Test.
- Benchmark.
- Package.
- Install.
- Publish.
- Target inspection.
- External-tool inspection.
- Documentation.
- Deployment.

A command that is not implemented MUST return an explicit unsupported-operation result or remain unavailable.

It MUST NOT silently run a placeholder.

---

23. AI agent integration

AI agents MUST use the same build requests, validation, permissions, task execution, diagnostics and result contracts as human developers.

An agent MAY:

- Inspect a build plan.
- Request compilation.
- Inspect target feasibility.
- Read diagnostics.
- Request tests.
- Request packaging.
- Propose a configuration change.
- Request a permitted retry.

An agent MUST NOT:

- Bypass authorization.
- Invent provider capabilities.
- Mark an unverified artifact as verified.
- Override semantic requirements.
- Claim target feasibility without evidence.
- Publish or deploy without the required authorization.
- Modify immutable build inputs during an active build without an explicit operation.

Every agent-triggered operation MUST be auditable.

---

24. Required file-level integration contracts

The following files define the expected build implementation boundaries.

They are architectural responsibilities, not claims that all files already exist.

File| Required ownership and integration
"src/toolchain/build/mod.rs"| Public build API, build-service composition and deliberate exports. Must not contain compiler algorithms.
"src/toolchain/build/request.rs"| Immutable build requests, input identity, target intent, profiles, features, policies and budgets. Integrates API, manifest and configuration contracts.
"src/toolchain/build/plan.rs"| Validated build-plan model, required stages, provider requirements, outputs and feasibility evidence. Consumes request and canonical capability results.
"src/toolchain/build/graph.rs"| Toolchain build-task graph only. Integrates tasks and build planning without replacing compiler or package graphs.
"src/toolchain/build/executor.rs"| Coordinates actual build execution, task lifecycle, compiler invocation, cancellation and failure propagation.
"src/toolchain/build/profiles.rs"| Applies validated profile configuration without introducing a second profile authority.
"src/toolchain/build/inputs.rs"| Captures canonical source, manifest, dependency and configuration input references. Integrates fingerprint authorities.
"src/toolchain/build/outputs.rs"| Collects artifact references, verifies output completeness and coordinates output commit through artifact authorities.
"src/toolchain/build/incremental.rs"| Coordinates existing compiler incremental compilation, invalidation and backend cache systems.
"src/toolchain/build/reporting.rs"| Produces structured build results, per-target outcomes, diagnostics, timing and provenance.
"src/toolchain/tasks/mod.rs"| Task subsystem composition and public contracts.
"src/toolchain/tasks/model.rs"| Stable task identity, task inputs, outputs, state, provider and execution contract.
"src/toolchain/tasks/graph.rs"| Task prerequisites, dependency relationships and deterministic ordering.
"src/toolchain/tasks/registry.rs"| Extensible task-provider registration and compatibility validation.
"src/toolchain/tasks/executor.rs"| Task execution, concurrency, cancellation, budgets and result propagation.
"src/toolchain/tasks/dependencies.rs"| Task prerequisite validation and dependency ordering. Must not duplicate package dependency resolution.
"src/toolchain/tasks/cancellation.rs"| Cancellation propagation to task providers and delegated operations.
"src/toolchain/tasks/reporting.rs"| Task execution records, timings, diagnostics and provenance references.
"src/toolchain/targets/mod.rs"| Toolchain-facing target orchestration boundary.
"src/toolchain/targets/profiles.rs"| Extensible portable target profiles and explicit target configuration.
"src/toolchain/targets/matrix.rs"| Multi-target request and result coordination.
"src/toolchain/targets/compatibility.rs"| Delegates compatibility to canonical compiler/backend/domain contracts.
"src/toolchain/targets/selection.rs"| Coordinates explicit target intent and approved selection policies.
"src/toolchain/targets/validation.rs"| Validates target requests and records unresolved or infeasible conditions.
"src/toolchain/environments/fingerprints.rs"| Integrates canonical compiler/backend environment fingerprints. Must not invent a second fingerprint algorithm.
"src/toolchain/external_tools/requirements.rs"| Declares external executable requirements for requested operations.
"src/toolchain/external_tools/resolution.rs"| Adapts "backend::toolchain" discovery and version validation.
"src/toolchain/external_tools/validation.rs"| Applies build and security policies to validated tool descriptors.
"src/toolchain/external_tools/reporting.rs"| Reports missing, incompatible, unverified and available tools.
"src/toolchain/diagnostics/diagnostic.rs"| Structured build diagnostics and subsystem attribution.
"src/toolchain/security/provenance.rs"| Coordinates canonical provenance evidence; does not create a second ledger.
"src/toolchain/documentation/BUILD_CONTRACT.md"| This normative build contract.

24.1 File freeze rule

Each implementation file MUST have its complete contract recorded before implementation begins.

The file's specification MUST include:

1. Purpose.
2. Exact ownership.
3. Public types.
4. Public functions.
5. Required invariants.
6. Direct dependencies.
7. Consumed external contracts.
8. Downstream consumers.
9. Error behavior.
10. Concurrency behavior.
11. Cancellation behavior.
12. Security obligations.
13. Scalability obligations.
14. Unit tests.
15. Integration tests.
16. Acceptance criteria.
17. Freeze conditions.

A file MUST NOT be frozen merely because it compiles independently.

It is frozen when its agreed API, invariants, failure semantics, tests and known integration obligations are complete.

A later discovered defect MUST be corrected in the authoritative owner rather than hidden through undocumented workarounds in unrelated consumers.

---

25. Grammar integration contract

The build system MUST respect the existing grammar architecture.

The inspected repository contains these relevant authorities:

- "grammar/DESIGN.md"
- "grammar/OWNERSHIP.yaml"
- "grammar/DEPENDENCIES.yaml"
- "grammar/MANIFEST.yaml"
- "grammar/Zamani.g4"
- "grammar/antlr/ZamaniLexer.g4"
- "grammar/antlr/ZamaniParser.g4"
- "grammar/lexer/lexer.g4"
- "grammar/lexer/tokens.g4"
- "grammar/compile/compile.g4"
- "grammar/modules/dependencies.g4"
- "grammar/specification/semantic-model.md"
- "grammar/specification/poco-reaf.md"
- "grammar/specification/scalability-model.md"
- "grammar/validation/semantic-boundaries.md"

The actual lexical token composition path is:

grammar/antlr/ZamaniLexer.g4
              |
              v
grammar/lexer/lexer.g4
              |
              v
grammar/lexer/tokens.g4
              |
              v
Canonical lexical component grammars
              |
              v
Generated lexer vocabulary

The parser composition path is:

grammar/antlr/ZamaniParser.g4
              |
              v
Canonical parser/domain composition
              |
              v
Domain-neutral source structure
              |
              v
Frontend AST
              |
              v
Semantic analysis

The top-level "grammar/Zamani.g4" is the combined grammar composition root.

The build subsystem MUST use the actual canonical grammar dependency and generation contracts.

It MUST NOT assume that grammar generation is equivalent to successful semantic validation or compilation.

25.1 Compilation grammar

"grammar/compile/compile.g4" owns source-level compilation syntax and composition.

It describes developer intent, such as compilation declarations, profiles, targets, optimization intent, reproducibility and deployment-related syntax.

It does NOT own:

- Actual compiler invocation.
- Target discovery.
- Resource allocation.
- Backend generation.
- Artifact verification.
- Physical execution.

The toolchain MUST consume the validated semantic interpretation of these constructs through the canonical frontend/compiler API.

25.2 Dependency grammar

"grammar/modules/dependencies.g4" owns source-level dependency declaration syntax.

It does NOT resolve packages, access registries, download dependencies or build packages.

The toolchain dependency subsystem MUST consume validated dependency declarations rather than parsing source text independently.

25.3 Grammar conformance

The build system SHOULD provide a grammar-validation workflow that:

- Discovers the canonical grammar roots.
- Resolves grammar imports.
- Validates dependency ordering.
- Detects missing imports.
- Detects conflicting ownership.
- Generates parser artifacts through the configured ANTLR toolchain.
- Verifies generated output compatibility.
- Runs lexical and parser conformance tests.
- Compares generated behavior with the canonical Rust frontend where applicable.

Generated parser artifacts MUST NOT become independent language authorities.

The grammar inventory and dependency registries MUST be validated before their results are treated as authoritative.

The build subsystem MUST NOT infer that every file beneath "grammar/" is a production grammar or that every documented production is implemented.

---

26. Configuration and compatibility

26.1 Configuration authority

The toolchain configuration subsystem owns toolchain-level policies and operational settings.

The compiler configuration remains owned by the compiler.

The backend configuration remains owned by the backend.

The project manifest remains owned by "src/zamani_project_config/".

A single setting MUST have one authoritative owner.

Adapters MAY translate configuration between subsystem boundaries but MUST NOT create conflicting sources of truth.

26.2 Version independence

The following versions MUST remain distinct:

- Zamani language version.
- Grammar version.
- Semantic-model version.
- Manifest version.
- Toolchain API version.
- Compiler API version.
- ZUIR version.
- Domain IR versions.
- Backend ABI version.
- External tool versions.
- Artifact format version.
- Package format version.
- Runtime version.
- Provider protocol version.

Compatibility MUST be established through explicit contracts.

A matching Cargo package version MUST NOT be treated as proof that all of these interfaces are compatible.

26.3 Schema evolution

Persisted build plans, lockfiles, manifests, artifacts and provenance records MUST use versioned schemas.

Unsupported future schema versions MUST be rejected or handled through an explicitly defined compatibility adapter.

Unknown required fields MUST NOT be silently ignored when doing so could change build meaning or security.

---

27. Testing and conformance requirements

Production readiness MUST be established through real implementation and test evidence.

Tests MUST cover isolated components and complete end-to-end workflows.

Mocks are permitted for unit tests and provider-contract tests.

Mock success MUST NOT be treated as proof of actual external-tool, hardware or remote-provider compatibility.

27.1 Foundation tests

- Stable identity.
- Request immutability.
- Configuration validation.
- Error conversion.
- Version compatibility.
- Budget semantics.
- Manifest adapter correctness.
- Workspace discovery.
- Path validation.

27.2 Dependency tests

- Direct dependencies.
- Transitive dependencies.
- Version constraints.
- Conflicting versions.
- Optional features.
- Local dependencies.
- Missing packages.
- Registry failures.
- Lockfile determinism.
- Lockfile tampering.
- Dependency invalidation.

27.3 Build-plan tests

- Valid plans.
- Missing prerequisites.
- Missing providers.
- Invalid task dependencies.
- Domain activation.
- Target compatibility.
- Multi-target matrices.
- Unsupported capabilities.
- Infeasible requirements.
- Deterministic plan serialization.

27.4 Execution tests

- Actual compiler invocation.
- Successful compilation.
- Compiler failure.
- Backend failure.
- External-tool failure.
- Timeout.
- Cancellation.
- Non-zero process exit.
- Output truncation.
- Resource exhaustion.
- Parallel task dependencies.
- Safe retry.
- Incomplete output rejection.

27.5 Artifact tests

- Valid artifacts.
- Invalid artifacts.
- Corrupted artifacts.
- Missing metadata.
- Incorrect target identity.
- Incorrect ABI.
- Dependency mismatch.
- Signature verification.
- Provenance verification.
- Transactional output commit.
- Cache corruption.

27.6 Portability tests

The conformance suite MUST include:

1. Minimal classical program.
2. Multi-module classical project.
3. Library project.
4. Parallel application.
5. Cross-target application.
6. Quantum project.
7. Quantum simulation project.
8. HDL project.
9. Hardware synthesis-provider project.
10. AI and tensor project.
11. Hybrid project.
12. Distributed project.
13. Mixed-domain project.
14. Future-provider conformance fixture.

Where a real provider or target is unavailable, the result MUST be marked as provider-contract or simulated-environment testing rather than actual target conformance.

27.7 Scalability tests

Scalability testing MUST examine:

- Tiny projects.
- Growing workspaces.
- Large dependency graphs.
- Large source collections.
- Large task graphs.
- Large multi-target matrices.
- Large artifact collections.
- Memory pressure.
- Storage pressure.
- Parallel execution.
- Cancellation under load.
- Incremental invalidation.
- Cache reuse.
- Distributed-provider coordination.

Tests SHOULD measure growth in time and memory as workload size increases.

The implementation MUST NOT pass scalability tests by silently truncating inputs, dropping tasks or ignoring dependencies.

27.8 Security tests

Test:

- Malformed manifests.
- Malicious dependency metadata.
- Path traversal.
- Symlink attacks.
- Malicious archives.
- Registry response manipulation.
- Credential leakage.
- Untrusted provider output.
- Process argument injection.
- Environment manipulation.
- Cache poisoning.
- Artifact tampering.
- Unauthorized publishing.
- Unauthorized deployment.

---

28. Required Cargo and Rust validation

The implementation MUST retain:

- Rust 2021 edition.
- Rust 1.97 or later.
- Safe Rust only.
- No "unsafe".
- No ignored "Result" values where correctness depends on the operation.

The following checks MUST pass:

cargo fmt --all -- --check

cargo check --all-targets

cargo test --all-targets

cargo clippy --all-targets -- -D warnings

Build-system feature validation:

cargo check --all-targets --features buildsystem

cargo test --all-targets --features buildsystem

Package-manager executable validation:

cargo build --release \
  --features buildsystem \
  --bin zamani-pkg

The actual package-manager executable MUST be tested against a real minimal project.

Passing a default-feature build MUST NOT be considered sufficient evidence for build-system readiness.

If additional features or provider-specific dependencies are introduced, each supported feature combination MUST receive appropriate build and test coverage.

---

29. Required implementation sequence

Implementation MUST proceed from stable independent contracts toward orchestration.

The order below minimizes downstream changes.

Phase 0 — Contract and ownership freeze

Before implementation:

1. Freeze this BUILD_CONTRACT.
2. Freeze the toolchain integration contract.
3. Freeze build request and response contracts.
4. Freeze task identity and state contracts.
5. Freeze build-plan semantics.
6. Freeze target realization identity.
7. Freeze artifact and provenance integration.
8. Freeze error conversion rules.
9. Freeze cancellation semantics.
10. Freeze configuration ownership.
11. Freeze compiler API integration requirements.
12. Freeze manifest integration requirements.

No implementation file should be started with unresolved ownership or API dependencies.

Phase 1 — Independent foundations

Implement and freeze:

1. "src/toolchain/errors.rs"
2. "src/toolchain/version.rs"
3. "src/toolchain/ids.rs"
4. "src/toolchain/result.rs"
5. "src/toolchain/api/versioning.rs"
6. "src/toolchain/api/compatibility.rs"
7. "src/toolchain/api/capabilities.rs"
8. "src/toolchain/configuration/budgets.rs"
9. "src/toolchain/configuration/policies.rs"
10. "src/toolchain/configuration/profiles.rs"
11. "src/toolchain/tasks/model.rs"
12. "src/toolchain/dependencies/model.rs"
13. "src/toolchain/dependencies/constraints.rs"
14. "src/toolchain/targets/profiles.rs"
15. "src/toolchain/environments/fingerprints.rs"

These files establish the shared contracts.

They MUST NOT depend on unfinished build orchestration.

Phase 2 — Project and manifest foundations

Implement:

- Canonical manifest schema extensions in "src/zamani_project_config/".
- Toolchain manifest adapter.
- Workspace discovery.
- Workspace membership.
- Workspace metadata.
- Project validation.
- Build configuration.
- Target declarations.
- Profile resolution.

The manifest authority MUST be frozen before build request and plan implementations are frozen.

Phase 3 — Dependency foundations

Implement:

- Dependency sources.
- Version constraints.
- Dependency graph.
- Dependency resolver.
- Feature resolution.
- Compatibility validation.
- Lockfile schema.
- Lockfile verification.
- Deterministic resolution.

The dependency model and lockfile format MUST be frozen before package installation and build planning depend on them.

Phase 4 — Task infrastructure

Implement:

- Task registry.
- Task graph.
- Task prerequisite validation.
- Task cancellation.
- Task executor.
- Task reporting.
- Provider compatibility.

Task execution MUST be validated independently before build orchestration consumes it.

Phase 5 — Build orchestration

Implement:

- Build request.
- Build plan.
- Build graph.
- Build input snapshots.
- Environment validation.
- Target matrix.
- Build executor.
- Compiler API integration.
- Backend integration.
- Incremental build integration.
- Artifact collection.
- Artifact verification.
- Build reporting.

The first real milestone MUST be a complete classical compilation, not a simulated build.

Phase 6 — Domain expansion

Extend the established build architecture to:

- Classical.
- Quantum.
- HDL.
- AI and tensor.
- Hybrid.
- Distributed.
- Networking.
- Cryptography.
- Embedded.
- Accelerators.
- Future registered domains.

Each domain MUST supply its own implementation and conformance evidence.

Phase 7 — Production developer experience

Implement:

- CLI integration.
- Structured diagnostics.
- Test workflows.
- Benchmark workflows.
- Documentation workflows.
- Package creation.
- Package installation.
- Registry operations.
- Deployment handoff.
- Human and AI agent integration.

These services MUST reuse the same build API.

Phase 8 — Production release validation

Run the complete Cargo checks and real project conformance suite.

Verify:

- Real compilation.
- Dependency correctness.
- Determinism.
- Incremental correctness.
- Artifact verification.
- Security.
- Portability.
- Scalability.
- Cancellation.
- Provenance.
- API compatibility.

Only then may the build subsystem be declared production-ready.

---

30. Production acceptance criteria

The build subsystem MUST NOT be declared production-ready until all applicable criteria have implementation and test evidence.

Area| Acceptance requirement
Project| Canonical manifest parsing and validation
Workspace| Explicit-root discovery and validated membership
Dependencies| Deterministic transitive resolution
Lockfile| Versioned and integrity-checked
Request| Immutable and fully validated
Planning| Explicit validated build plan
Graph| Correct dependency ordering
Tasks| Real execution and structured results
Parallelism| Dependency-safe parallel execution
Cancellation| Correct propagation and cleanup
Compiler| Actual invocation through public compiler API
Backend| Registered provider integration
External tools| Discovery, version validation and safe execution
Targets| Extensible target matrix
Portability| Semantic preservation across compatible realizations
Quantum| Canonical quantum compiler integration
HDL| Canonical HDL and synthesis-provider integration
AI| Canonical AI/tensor integration
Hybrid| Cross-domain build coordination
Distributed| Distributed build/provider integration
Incremental| Correct invalidation and cache reuse
Artifacts| Validated artifact references and metadata
Security| Safe inputs, outputs, credentials and providers
Reproducibility| Explicit reproducibility assessment
Provenance| Traceable source-to-artifact evidence
Diagnostics| Structured human and machine-readable results
CLI| Thin adapter over the canonical toolchain API
Agents| Same authorization and execution contracts
Scalability| No arbitrary universal capacity ceilings
Rust| Rust 1.97+, edition 2021, no unsafe
Cargo| All required checks pass

An unavailable provider MUST be marked unavailable.

An unimplemented domain MUST be marked unimplemented.

A mocked integration MUST be marked as mocked.

A partially completed build MUST be marked partial.

No documentation, demonstration or release note may convert these states into a false production-readiness claim.

---

31. Final architectural invariants

The following decisions are binding for the build subsystem.

1. The toolchain orchestrates; the compiler compiles.
2. The frontend parses and constructs canonical source representations.
3. Semantic analysis owns source meaning.
4. ZUIR and domain IRs own their respective canonical representations.
5. The compiler owns compiler-internal dependency and transformation workflows.
6. The package subsystem owns package dependency resolution.
7. The toolchain owns build-task orchestration.
8. The backend owns target-specific artifact realization.
9. "backend::toolchain" owns external executable discovery and version validation.
10. "backend::process" owns safe process execution.
11. ZHAL owns physical hardware abstraction and provider integration.
12. ZUR owns runtime execution.
13. The toolchain coordinates deployment without becoming the physical infrastructure manager.
14. The manifest remains owned by "src/zamani_project_config/".
15. The grammar remains governed by "grammar/".
16. The toolchain MUST NOT create a competing compiler, IR, scheduler, hardware model, runtime, process runner or manifest parser.
17. No arbitrary universal hardware or computational capacity ceiling is permitted.
18. Operational budgets MUST be explicit, configurable and enforced by the appropriate authority.
19. Unknown capability MUST NOT be treated as unlimited capability.
20. Unsupported computation MUST NOT be reported as successful.
21. Build success MUST require actual compilation and the completion of all mandatory stages.
22. Multi-target outputs MUST preserve their shared logical source and dependency identity.
23. Target-specific artifacts MUST have independent identities and verification evidence.
24. Reproducibility and provenance MUST be evidence-based.
25. All production Rust MUST use safe Rust.

---

32. Definition of completion

The Zamani build subsystem is complete only when a developer can:

1. Create a valid Zamani project.
2. Describe its dependencies using the canonical project and language contracts.
3. Validate its workspace and manifest.
4. Resolve dependencies reproducibly.
5. Request one or multiple compatible target realizations.
6. Inspect a complete build plan.
7. Execute the build through real compiler and backend implementations.
8. Use incremental and parallel compilation where supported.
9. Receive verified artifacts.
10. Inspect the build's diagnostics and provenance.
11. Reproduce the declared build guarantees.
12. Hand off compatible artifacts to the runtime or deployment authorities.
13. Perform the same operations through the CLI, library API, IDE, automation or authorized AI agent.

All of this MUST use one coherent toolchain architecture.

The architecture MUST remain extensible as new computing domains, target architectures, backend providers, hardware systems, registries and execution environments are introduced.

Final principle:

Zamani must not make the programmer redesign a program merely because the available machine becomes larger, smaller, faster, slower, distributed, heterogeneous or fundamentally different.

The language defines the computation.

The semantic model defines its meaning.

The compiler determines valid transformations.

The toolchain coordinates the build.

The backend realizes the computation.

The hardware and runtime authorities determine what can actually execute.

The build system reports the truth about every stage.

That is the production foundation of POCO-REAF.