Zamani Toolchain — Production Readiness Matrix

Canonical path: "src/toolchain/documentation/READINESS_MATRIX.md"
Repository: "github.com/Benwellonedge28/Zamani"
Branch audited: "main"
Audit tree SHA: "6a70647f0d1e730dd2c26fdf4d304a381848f0b3"
Rust edition: 2021
Minimum supported Rust: 1.97 or later
Safety: Safe Rust exclusively. "unsafe" is prohibited.
Architecture objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Document classification: Normative implementation, integration, scalability, security, testing and production-readiness specification.

---

1. Purpose

This document defines the implementation and production-readiness requirements for the complete Zamani toolchain subsystem:

"src/toolchain/"

It establishes:

- The responsibility of every existing toolchain file.
- The complete public API and ownership boundaries.
- Dependencies between files.
- Integration contracts with other repository subsystems.
- Required implementation behavior.
- Error and failure semantics.
- Security requirements.
- Concurrency requirements.
- Scalability requirements.
- Testing requirements.
- File completion and freeze criteria.
- Implementation ordering.
- Repository-wide integration requirements.
- Production release acceptance criteria.

The objective is to make the toolchain a production-grade, extensible development and distribution infrastructure for the entire Zamani programming language.

The architecture must support:

1. Classical computing.
2. Systems programming.
3. Embedded computing.
4. Parallel and concurrent computing.
5. Scientific and numerical computing.
6. High-performance computing.
7. Quantum computing.
8. Quantum simulation.
9. Quantum-classical hybrid computing.
10. Hardware description and hardware/software co-design.
11. FPGA and ASIC development.
12. Artificial intelligence.
13. Machine learning and deep learning.
14. Tensor and accelerator computing.
15. Symbolic and probabilistic computation.
16. Causal computation and reasoning.
17. Distributed computing.
18. Networking and communication.
19. Cryptography and secure computation.
20. Neuromorphic and future computing technologies.

These domains must share one project, workspace, dependency, package, build and developer-workflow architecture.

They must not require separate package managers, project formats or competing build systems.

2. Fundamental POCO-REAF contract

The fundamental objective is:

Program Once → Compile Once → Run Everywhere → Anywhere → Forever.

POCO-REAF requires:

1. One canonical source-language meaning.
2. One consistent project identity.
3. One consistent workspace model.
4. One canonical manifest.
5. One dependency-resolution model.
6. One logical build request.
7. Reusable build specifications.
8. Multiple compatible target-specific realizations.
9. Verified artifacts.
10. Explicit compatibility evidence.
11. Explicit feasibility evidence.
12. Reproducible build identities.
13. Versioned compatibility contracts.
14. Runtime handoff through authoritative execution infrastructure.
15. Compatibility-aware evolution across future computing environments.

A single logical build may generate different target-specific artifacts.

For example, a compatible source project may generate:

- A native executable.
- A WebAssembly module.
- An accelerator-specific artifact.
- A quantum circuit.
- An HDL synthesis input.
- An AI model artifact.
- A distributed application package.

These outputs need not contain identical machine code. They must preserve the applicable source semantics and declared correctness contracts.

A target request does not prove feasibility.

A registered provider does not prove capability.

A successful build plan does not prove compilation.

A successful compilation does not prove runtime compatibility.

An artifact's existence does not prove its integrity, authenticity or portability.

Each condition must be established by its authoritative subsystem.

3. Definition of unlimited scalability

The expression "from atom to everywhere" means that Zamani must not introduce arbitrary universal finite ceilings on computational concepts, project size, hardware populations or supported computing domains.

It does not mean literal infinite physical resources.

Every implementation remains subject to:

- Available memory.
- Available storage.
- Processing capacity.
- Address-space limits.
- Rust representation limits.
- Operating-system constraints.
- Network capacity.
- Physical laws.
- Provider capabilities.
- Hardware availability.
- Semantic requirements.
- Explicit operational budgets.
- Security and authorization policies.

The following distinctions are mandatory:

Concept| Meaning
Language capability| What Zamani can express
Semantic validity| Whether a program is meaningful and valid
Implementation capability| What a particular implementation can perform
Target capability| What a target can provide
Resource availability| What resources are currently accessible
Feasibility| Whether the requested operation can be realized
Operational budget| A configurable limit governing an operation

No arbitrary maximum may be introduced for:

- CPU count.
- Processor count.
- GPU count.
- FPGA count.
- QPU count.
- Qubit count.
- Memory capacity.
- VRAM capacity.
- Register width.
- Tensor rank.
- Tensor dimensions.
- Distributed participants.
- Project size.
- Dependency count.
- Package count.
- Registry count.
- Build tasks.
- Targets.
- Providers.
- Computing domains.
- Workspace members.

Do not introduce constants such as:

"MAX_CPUS", "MAX_GPUS", "MAX_QUBITS", "MAX_NODES", "MAX_TENSOR_RANK", "MAX_MEMORY", or equivalent artificial capability limits.

A current implementation may have documented operational limitations. These must remain distinguishable from language-level limitations.

Operational protection is permitted through configurable budgets, including:

- Build time.
- Memory consumption.
- Storage consumption.
- Download size.
- Archive extraction size.
- Concurrent tasks.
- Network usage.
- Retry counts.
- Diagnostic retention.
- Provider invocation counts.

A budget must be explicit, observable and associated with the operation it governs.

An unlimited budget means that the toolchain has not imposed an application-level budget for that dimension. It does not create infinite physical resources.

4. Repository authority and ownership

Every responsibility must have one canonical owner.

The toolchain coordinates these owners rather than replacing them.

Responsibility| Canonical authority
Source grammar| "grammar/"
Lexing and parsing| "src/frontend/"
AST| "src/frontend/ast/"
Source semantics| "src/frontend/semantic/"
Classical computation| "src/classical/"
Quantum computation and quantum IR| "src/quantum/"
HDL and hardware description| "src/hdl/"
AI computation| "src/ai/"
Hybrid computation| "src/hybrid/"
Distributed computation| "src/distributed/"
Universal IR| "src/zuir/"
Compilation| "src/compiler/"
Compiler optimization and planning| "src/compiler/" and its authoritative integrations
Target-specific generation| "src/backend/"
External tool discovery| "src/backend/toolchain.rs"
External process execution| "src/backend/process.rs"
Backend provider registry| "src/backend/registry.rs"
Compiler artifacts| "src/compiler/artifacts/" and backend artifact contracts
Physical hardware integration| "src/zhal/"
Runtime and execution| "src/zur/"
Shared runtime and platform contracts| "src/stdlib/"
Project manifest| "src/zamani_project_config/"
Project and workspace orchestration| "src/toolchain/workspace/"
Dependency resolution| "src/toolchain/dependencies/"
Package lifecycle| "src/toolchain/packages/"
Registry coordination| "src/toolchain/registries/"
Build orchestration| "src/toolchain/build/"
Task orchestration| "src/toolchain/tasks/"
Developer commands| "src/toolchain/commands/"
Toolchain security integration| "src/toolchain/security/"
Toolchain diagnostics| "src/toolchain/diagnostics/"
Agent interaction| "src/toolchain/agents/"

4.1 Prohibited duplicate systems

The toolchain must not create:

- A second compiler.
- A second lexer or parser.
- A second AST.
- A second semantic model.
- A second ZUIR.
- A second quantum IR.
- A second HDL IR.
- A second tensor storage system.
- A second backend registry.
- A second hardware abstraction layer.
- A second hardware discovery engine.
- A second physical resource allocator.
- A second runtime.
- A second scheduler.
- A second external process runner.
- A second external executable discovery system.
- A second cryptographic implementation.
- A second project manifest parser.
- A second compiler artifact store.
- A second compiler incremental-compilation engine.
- A second compiler budget-accounting implementation.
- A second general-purpose authorization engine.

A new file is justified only when it introduces a genuinely distinct toolchain-owned responsibility.

5. Current repository inventory

The following is the actual toolchain directory structure found in the audited tree.

src/toolchain/
├── README.md
├── mod.rs
├── errors.rs
├── ids.rs
├── result.rs
├── version.rs
│
├── agents/
│   ├── mod.rs
│   ├── context.rs
│   ├── requests.rs
│   ├── results.rs
│   ├── permissions.rs
│   ├── audit.rs
│   └── compliance.rs
│
├── api/
│   ├── mod.rs
│   ├── request.rs
│   ├── response.rs
│   ├── capabilities.rs
│   ├── compatibility.rs
│   └── versioning.rs
│
├── build/
│   ├── mod.rs
│   ├── request.rs
│   ├── plan.rs
│   ├── graph.rs
│   ├── executor.rs
│   ├── profiles.rs
│   ├── inputs.rs
│   ├── outputs.rs
│   ├── incremental.rs
│   └── reporting.rs
│
├── commands/
│   ├── mod.rs
│   ├── build.rs
│   ├── check.rs
│   ├── clean.rs
│   ├── run.rs
│   ├── test.rs
│   ├── bench.rs
│   ├── format.rs
│   ├── lint.rs
│   ├── doc.rs
│   ├── package.rs
│   ├── install.rs
│   ├── publish.rs
│   ├── target.rs
│   ├── tool.rs
│   ├── doctor.rs
│   └── deploy.rs
│
├── configuration/
│   ├── mod.rs
│   ├── config.rs
│   ├── policies.rs
│   ├── profiles.rs
│   ├── budget.rs
│   └── validation.rs
│
├── dependencies/
│   ├── mod.rs
│   ├── model.rs
│   ├── constraints.rs
│   ├── resolver.rs
│   ├── graph.rs
│   ├── lockfile.rs
│   ├── sources.rs
│   ├── features.rs
│   ├── compatibility.rs
│   └── validation.rs
│
├── deployment/
│   ├── mod.rs
│   ├── request.rs
│   ├── profiles.rs
│   ├── validation.rs
│   ├── planning.rs
│   ├── providers.rs
│   └── reporting.rs
│
├── diagnostics/
│   ├── mod.rs
│   ├── diagnostic.rs
│   ├── aggregation.rs
│   └── ordering.rs
│   └── rendering.rs
│
├── environments/
│   ├── mod.rs
│   ├── discovery.rs
│   ├── configuration.rs
│   ├── fingerprints.rs
│   └── validation.rs
│
├── external_tools/
│   ├── mod.rs
│   ├── requirements.rs
│   ├── resolution.rs
│   ├── validation.rs
│   └── reporting.rs
│
├── manifest/
│   ├── mod.rs
│   ├── adapter.rs
│   ├── loading.rs
│   ├── validation.rs
│   └── compatibility.rs
│
├── packages/
│   ├── mod.rs
│   ├── model.rs
│   ├── metadata.rs
│   ├── archive.rs
│   ├── creation.rs
│   ├── extraction.rs
│   ├── installation.rs
│   └── verification.rs
│
├── registries/
│   ├── mod.rs
│   ├── provider.rs
│   ├── registry.rs
│   ├── discovery.rs
│   ├── client.rs
│   ├── protocol.rs
│   ├── authentication.rs
│   ├── publishing.rs
│   ├── downloading.rs
│   ├── federation.rs
│   └── validation.rs
│
├── security/
│   ├── mod.rs
│   ├── policy.rs
│   ├── trust.rs
│   ├── signatures.rs
│   ├── integrity.rs
│   ├── provenance.rs
│   └── validation.rs
│
├── targets/
│   ├── mod.rs
│   ├── profiles.rs
│   ├── matrix.rs
│   ├── compatibility.rs
│   ├── selection.rs
│   └── validation.rs
│
├── tasks/
│   ├── mod.rs
│   ├── model.rs
│   ├── graph.rs
│   ├── registry.rs
│   ├── executor.rs
│   ├── dependencies.rs
│   ├── cancellation.rs
│   └── reporting.rs
│
├── workspace/
│   ├── mod.rs
│   ├── discovery.rs
│   ├── layout.rs
│   ├── members.rs
│   ├── metadata.rs
│   └── validation.rs
│
└── documentation/
    ├── ARCHITECTURE.md
    ├── INTEGRATION_CONTRACT.md
    ├── BUILD_CONTRACT.md
    ├── PACKAGE_CONTRACT.md
    ├── PORTABILITY_CONTRACT.md
    ├── SECURITY_CONTRACT.md
    ├── AGENT_CONTRACT.md
    └── READINESS_MATRIX.md

Inventory corrections:

- "configuration/budget.rs" is canonical. Do not create "budgets.rs".
- "diagnostics/ordering.rs" is canonical. Aggregation must delegate to it.
- "commands/doctor.rs" already exists.
- "agents/compliance.rs" already exists.
- "deployment/profiles.rs" already exists.
- "READINESS_MATRIX.md" is the one new file being specified here.
- No separate "package_manager/" subsystem should be created.
- No separate "projects/" subsystem should be created without demonstrating a genuinely distinct ownership requirement.

The tree above is an inventory and target documentation structure, not evidence that every implementation has passed its acceptance criteria.

6. Required contract for every Rust source file

Every toolchain Rust file must satisfy the following common requirements.

6.1 File documentation

Every production module must document:

1. Canonical path.
2. Purpose.
3. Ownership.
4. Non-ownership.
5. Public API.
6. Direct dependencies.
7. Integration consumers.
8. Integration authorities.
9. Error behavior.
10. Concurrency behavior.
11. Security requirements.
12. Scalability requirements.
13. Tests.
14. Freeze criteria.

6.2 Rust safety and quality

Every production Rust source file must:

- Compile on Rust 1.97 or later.
- Use Rust edition 2021.
- Contain no "unsafe" blocks or unsafe implementation requirements.
- Preserve thread safety where required.
- Avoid hidden global mutable state.
- Avoid panic-based error handling for recoverable failures.
- Avoid silently ignored errors.
- Avoid fabricated successful outcomes.
- Use checked arithmetic where overflow is possible.
- Use fallible allocation where an allocation failure can be handled meaningfully.
- Preserve deterministic behavior where the contract requires determinism.
- Provide structured errors.
- Have focused tests.
- Have complete public API documentation.

Use module-level safety declarations where appropriate:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

The exact lint set may be adjusted for private implementation modules, but unsafe Rust remains prohibited.

6.3 Integration freeze

A file must not be frozen merely because it compiles in isolation.

Its complete API and integration contract must be agreed before implementation.

A file is frozen only when:

- All owned types are finalized.
- All public functions and traits are finalized.
- Dependencies are known.
- Consumers are known.
- Error conversions are known.
- Serialization behavior is known where applicable.
- Concurrency requirements are known.
- Security requirements are satisfied.
- Unit tests pass.
- Integration tests pass.
- Downstream consumers compile.
- No unresolved architectural dependency remains.

A later discovery of a genuine defect requires an intentional correction to the owning contract. Freeze does not prohibit necessary maintenance.

7. Root foundation files

These files establish the shared toolchain contracts.

File| Complete responsibility| Integration and freeze requirements
"errors.rs"| Own canonical "ToolchainError", "ToolchainErrorKind", stable error codes, causal source preservation, operation context and canonical result aliases.| Must preserve frontend, compiler, backend, runtime, manifest and provider errors as causes. Cancellation, denial, incompatibility, validation failure and resource exhaustion must remain distinct. All toolchain modules consume it.
"version.rs"| Own independent toolchain contract versions, API versions, package-format versions, registry protocol versions, provider compatibility ranges and checked version evolution.| Reuse "crate::zuir::version" as the canonical generic version representation. Do not redefine language, grammar, AST, IR, compiler, firmware or runtime versions.
"ids.rs"| Own strongly typed identities for toolchain-owned projects, workspaces, packages, dependencies, sources, tasks, builds, registries, providers, environments, profiles and deployments.| Reuse backend, compiler, AST and domain identifiers rather than replacing them. Define uniqueness scope honestly. Support persistent or caller-provided identity when cross-process uniqueness is required.
"result.rs"| Expose canonical result aliases and contextual error-enrichment helpers.| Reuse "errors.rs". Never introduce a second error taxonomy or discard causal sources.
"mod.rs"| Compose and expose the toolchain modules. Establish the public root boundary and architectural invariants.| Must declare only existing modules, expose deliberate APIs and preserve the ownership hierarchy. Coordinate with "src/lib.rs" and the "buildsystem" feature.

The root module must remain a composition root, not a location for implementing services.

8. Public API subsystem

Directory: "src/toolchain/api/"

File| Complete responsibility| Integration and freeze requirements
"api/mod.rs"| Public API composition and deliberate re-exports.| Must expose stable API contracts without leaking internal implementation details.
"api/request.rs"| Typed requests for workspace, build, dependency, package, registry, test, run and deployment operations.| Consume canonical configuration, manifest, target and operation request types. Validate before dispatch.
"api/response.rs"| Structured operation responses, results, artifacts, diagnostics, provenance references and partial-operation outcomes.| Must distinguish success, failure, cancellation, denial, incomplete verification and unresolved feasibility.
"api/capabilities.rs"| Describe toolchain API capabilities and available operations.| Capability declarations must originate from actual registered implementations or authoritative providers.
"api/compatibility.rs"| Evaluate compatibility between public toolchain contracts and supported clients.| Consume "version.rs" and canonical compatibility authorities. Unknown versions must not be assumed compatible.
"api/versioning.rs"| Version public API contracts and negotiate supported API versions.| Preserve independent version domains and backward compatibility guarantees.

The API must support:

- CLI applications.
- IDE integrations.
- Automation.
- CI systems.
- Human developers.
- Authorized AI agents.
- External toolchain clients.

All clients must use the same operation semantics.

9. Configuration subsystem

Directory: "src/toolchain/configuration/"

File| Complete responsibility| Integration and freeze requirements
"configuration/mod.rs"| Configuration subsystem composition and exports.| Must expose the canonical configuration contracts.
"configuration/config.rs"| Aggregate toolchain configuration, sources and defaults.| Defaults must be explicit and overrideable. Avoid implicit machine or vendor assumptions.
"configuration/policies.rs"| Configuration policies for security, dependency resolution, building, registries and deployment.| Must delegate authorization and security decisions to their authoritative systems.
"configuration/profiles.rs"| Profile identity, registration, inheritance, resolution and configuration composition.| Profiles must be extensible and must not redefine compiler optimization semantics.
"configuration/budget.rs"| Canonical toolchain-facing operational budget integration.| Reuse canonical compiler budget accounting. Support named, open-ended budget dimensions, shared accounting, checked arithmetic and configurable limits.
"configuration/validation.rs"| Validate configuration structure, compatibility, policies, profiles and budgets.| Must reject invalid or conflicting configuration before execution.

Budget-specific mandatory decisions

"budget.rs" must remain the sole toolchain budget integration file.

It must not create a competing budget engine.

The following are operational budget dimensions, not language capabilities:

- Time.
- Memory.
- Storage.
- Network transfer.
- Archive extraction.
- Build operations.
- Provider invocations.
- Diagnostic retention.
- Concurrent operations.

Custom dimensions must be supported.

The default must not introduce arbitrary application-level hardware ceilings.

10. Workspace subsystem

Directory: "src/toolchain/workspace/"

File| Complete responsibility| Integration and freeze requirements
"workspace/mod.rs"| Workspace composition and public API.| Must expose the canonical workspace services.
"workspace/discovery.rs"| Discover project and workspace roots from explicit paths and supported discovery policies.| Must not assume the current working directory is the project root. Handle symlinks and filesystem boundaries according to policy.
"workspace/layout.rs"| Interpret canonical workspace and source layout declarations.| Consume manifest and frontend contracts without introducing a competing project format.
"workspace/members.rs"| Model workspace membership, project relationships and member identities.| Reuse "ProjectId" and "WorkspaceId". Validate duplicate and conflicting membership.
"workspace/metadata.rs"| Workspace metadata and deterministic workspace fingerprints.| Reuse canonical reproducibility and hashing authorities.
"workspace/validation.rs"| Validate workspace structure, paths, member consistency and configuration.| Produce structured diagnostics and preserve incomplete validation states.

The workspace system must support small projects and large workspaces without arbitrary member-count limits.

11. Manifest integration

Directory: "src/toolchain/manifest/"

File| Complete responsibility| Integration and freeze requirements
"manifest/mod.rs"| Toolchain-facing manifest API.| Must adapt canonical project configuration types.
"manifest/adapter.rs"| Convert canonical "ZamaniToml" data into validated toolchain requests and domain-neutral configuration.| No independent TOML schema or duplicate parser.
"manifest/loading.rs"| Load a manifest from an explicitly selected project location.| Use safe filesystem access and structured errors.
"manifest/validation.rs"| Validate manifest requirements for workspace, dependency, build and package operations.| Delegate schema validation to "zamani_project_config".
"manifest/compatibility.rs"| Handle manifest-version compatibility and migration requirements.| Never silently reinterpret an unknown manifest version.

11.1 Canonical manifest authority

The existing authority is:

"src/zamani_project_config/"

The current inspected "src/zamani_project_config/mod.rs" contains:

- "ZamaniToml"
- "PackageSection"
- "DependencyConfig"
- "parse_from_str"

It currently models basic package and dependency information.

It does not yet constitute a complete production manifest implementation.

The manifest schema must be expanded in its owning subsystem.

The following manifest responsibilities must be added or integrated there:

- Package metadata.
- Dependency declarations.
- Workspace configuration.
- Feature configuration.
- Build profiles.
- Target intent.
- Toolchain requirements.
- Registry sources.
- Packaging policies.
- Testing configuration.
- Deployment intent.
- Schema versioning.
- Compatibility and migration.

The toolchain must not duplicate these definitions.

The parser must distinguish malformed values from missing values rather than silently replacing invalid fields with empty strings or default values.

12. Build subsystem

Directory: "src/toolchain/build/"

This subsystem is the central lifecycle coordinator.

File| Complete responsibility| Integration and freeze requirements
"build/mod.rs"| Build subsystem composition and public API.| Owns the build boundary and service exports, not compiler implementation.
"build/request.rs"| Immutable build requests containing project identity, source inputs, profiles, features, target intent, requirements, constraints and operational budgets.| Integrate with "api/request.rs", manifests, targets and configuration.
"build/plan.rs"| Validated build plans and target-specific feasibility evidence.| Consume compiler planning and canonical compatibility results. Unknown feasibility must not be treated as feasible.
"build/graph.rs"| Build dependency graph, node identity, edges, graph validation and deterministic ordering.| Must remain distinct from package dependency graphs and compiler IR graphs.
"build/executor.rs"| Execute validated build plans and coordinate compiler invocation, cancellation, budget accounting and artifact verification.| Use the public compiler API and authoritative artifact verification. Must not fabricate successful output.
"build/profiles.rs"| Resolve build profile configuration and applicable policies.| Reuse "configuration/profiles.rs" and compiler profile contracts.
"build/inputs.rs"| Track source, manifest, dependency, configuration, environment and provider inputs.| Reuse canonical fingerprints and reproducibility contracts.
"build/outputs.rs"| Collect verified compiler and backend artifact references.| Reuse "compiler/artifacts" and backend artifact authorities.
"build/incremental.rs"| Coordinate incremental builds and cache invalidation.| Reuse "compiler/incremental/". No second cache engine.
"build/reporting.rs"| Produce complete build reports, outcomes, diagnostics, timings and provenance references.| Integrate with diagnostics and reproducibility authorities.

12.1 Required build lifecycle

Public BuildRequest
        |
        v
Request validation
        |
        v
Workspace discovery
        |
        v
Manifest validation
        |
        v
Dependency resolution
        |
        v
Profile resolution
        |
        v
Target intent resolution
        |
        v
BuildPlan construction
        |
        v
Compiler feasibility assessment
        |
        v
AssessedBuildPlan
        |
        v
Build input fingerprinting
        |
        v
BuildGraph construction
        |
        v
Task execution
        |
        v
Public CompilerService
        |
        v
Canonical compiler pipeline
        |
        v
Backend realization
        |
        v
Artifact verification
        |
        v
BuildOutputs
        |
        v
Incremental state coordination
        |
        v
BuildReport

This sequence must be implemented through real service integrations.

A module declaration is not an implementation.

12.2 Critical existing implementation issue

The inspected "build/executor.rs" identifies the compiler API as a transitional legacy driver.

This must be resolved before production release.

The final implementation must:

1. Invoke the canonical production compiler service.
2. Pass validated compilation inputs.
3. Preserve target identity.
4. Preserve requirements and constraints.
5. Associate compilation results with their originating build.
6. Verify output metadata.
7. Verify artifact integrity through the owning authority.
8. Preserve compiler and backend errors.
9. Return a structured build result.
10. Refuse to claim success when any required stage fails.

Its test compiler and test verifier are acceptable as test doubles. They are not evidence of production compilation.

13. Dependency subsystem

Directory: "src/toolchain/dependencies/"

File| Complete responsibility| Integration and freeze requirements
"dependencies/mod.rs"| Dependency subsystem composition and public API.| Expose canonical dependency contracts.
"dependencies/model.rs"| Dependency identities, declarations, source identity and dependency metadata.| Adapt canonical manifest and package declarations.
"dependencies/constraints.rs"| Version, source, feature and compatibility constraints.| Support registered version schemes rather than one hard-coded algorithm.
"dependencies/resolver.rs"| Deterministic dependency solving and candidate selection.| Integrate registered source providers and package compatibility contracts.
"dependencies/graph.rs"| Package dependency topology, reachability, cycle analysis and dependency-first ordering.| Remain separate from compiler and build-task graphs.
"dependencies/lockfile.rs"| Versioned, deterministic lockfile serialization, verification and persistence.| Preserve source identity, selected versions, features, integrity references and resolution evidence.
"dependencies/sources.rs"| Local, path, registry and extensible package source integration.| Delegate network and filesystem operations to approved providers.
"dependencies/features.rs"| Feature activation and dependency expansion.| Use canonical package feature declarations.
"dependencies/compatibility.rs"| Package and artifact compatibility evaluation.| Consume compiler, backend, IR and package compatibility evidence.
"dependencies/validation.rs"| Validate the completed resolution before installation or building.| Reject unresolved conflicts, missing sources and invalid lockfile state.

The resolver must support:

- Transitive dependencies.
- Multiple version schemes.
- Conflicting constraints.
- Optional dependencies.
- Feature activation.
- Feature unification.
- Cyclic dependency detection.
- Multiple sources.
- Deterministic resolution.
- Reproducible lockfiles.
- Compatibility checks.
- Unavailable source handling.
- Integrity evidence.

No arbitrary dependency-count or dependency-depth language limit is permitted.

Actual resource exhaustion must be reported as resource exhaustion.

14. Package subsystem

Directory: "src/toolchain/packages/"

File| Complete responsibility| Integration and freeze requirements
"packages/mod.rs"| Package composition and public API.| Expose package lifecycle operations without duplicating dependency resolution.
"packages/model.rs"| Canonical package identity, metadata, contents and artifact descriptions.| Reuse toolchain package IDs and canonical version contracts.
"packages/metadata.rs"| Package manifest encoding, decoding and metadata validation.| Define a versioned, deterministic package metadata schema.
"packages/archive.rs"| ".zpkg" archive framing and serialization.| Define deterministic archive structure, entry metadata and format compatibility.
"packages/creation.rs"| Construct packages from validated project contents.| Consume workspace, manifest, dependency, artifact and packaging contracts.
"packages/extraction.rs"| Safely extract package archives.| Prevent path traversal, unsafe links, malformed entries and resource-exhaustion attacks.
"packages/installation.rs"| Install verified packages into configured destinations.| Support atomic installation, rollback and explicit installation policies.
"packages/verification.rs"| Validate package structure, integrity, signatures and compatibility.| Delegate cryptography, trust and integrity to canonical security providers.

14.1 Package requirements

Packages must support:

- Source distributions.
- Libraries.
- Applications.
- Portable artifacts.
- Multiple target-specific artifacts.
- Quantum artifacts.
- HDL designs.
- AI models and datasets.
- Hybrid applications.
- Distributed applications.
- Embedded software.
- Firmware.
- Future domain artifacts.

A package must preserve logical identity independently of the physical machine.

Archive extraction must be defensive.

The archive format must be documented before publishing is implemented.

Do not publish an archive whose format, compatibility, integrity or verification contract is still changing.

15. Registry subsystem

Directory: "src/toolchain/registries/"

File| Complete responsibility| Integration and freeze requirements
"registries/mod.rs"| Registry composition and public API.| Expose registry lifecycle and provider interfaces.
"registries/provider.rs"| Extensible registry-provider contract.| Define discover, query, download and publication capabilities without vendor-specific coupling.
"registries/registry.rs"| Registry identity, configuration and lifecycle.| Use "RegistryId" and canonical configuration contracts.
"registries/discovery.rs"| Discover configured registry providers and registry instances.| Validate discovery evidence; discovery is not trust.
"registries/client.rs"| Coordinate registry client operations.| Delegate actual transport to registered implementations.
"registries/protocol.rs"| Versioned registry request/response contracts.| Use "version.rs" and reject incompatible protocol versions.
"registries/authentication.rs"| Authentication-provider integration and credential references.| Never expose secrets or implement an independent credential store.
"registries/publishing.rs"| Package publication lifecycle and publication status.| Require validated packages, authorization, signatures and integrity evidence.
"registries/downloading.rs"| Download and verify package content.| Apply safe storage and resource budgets.
"registries/federation.rs"| Coordinate multiple registry instances and compatible sources.| Preserve registry provenance and trust boundaries.
"registries/validation.rs"| Validate registry metadata, protocol, responses and provider declarations.| Fail closed on invalid or untrusted responses.

Registry support must be open-world.

Do not hard-code:

- One registry URL.
- One authentication provider.
- One transport.
- One package vendor.
- One cloud.
- One registry implementation.

Support public, private, enterprise, self-hosted and federated registries through registered providers.

16. Target subsystem

Directory: "src/toolchain/targets/"

File| Complete responsibility| Integration and freeze requirements
"targets/mod.rs"| Target-management composition and public API.| Preserve the distinction between target intent, target identity and physical realization.
"targets/profiles.rs"| Portable target profiles and associated build policies.| Reuse compiler target and configuration authorities.
"targets/matrix.rs"| Multi-target build matrix construction.| Support multiple compatible realizations without arbitrary target-count limits.
"targets/compatibility.rs"| Coordinate target and artifact compatibility evaluation.| Delegate authoritative compatibility decisions to backend and compiler authorities.
"targets/selection.rs"| Coordinate requested target-selection policy.| Consume canonical selection and feasibility contracts.
"targets/validation.rs"| Validate requested targets and report unsupported, unknown, incompatible and infeasible conditions.| Never treat an unknown target as a successful target.

16.1 Critical ownership correction

The toolchain must not own a second hardware or backend target registry.

It must integrate with:

- "compiler/configuration/target"
- "compiler/planning"
- "backend/target"
- "backend/capabilities"
- "backend/compatibility"
- "backend/validation"
- "zhal"

The toolchain owns project-facing target intent and orchestration.

The compiler owns compilation planning and feasibility assessment.

The backend owns target realization.

ZHAL owns physical hardware integration.

The runtime owns execution.

Physical placement, routing, scheduling and allocation must remain with their respective authoritative systems.

17. External tool integration

Directory: "src/toolchain/external_tools/"

File| Complete responsibility| Integration and freeze requirements
"external_tools/mod.rs"| External-tool integration facade.| Expose stable toolchain-facing interfaces.
"external_tools/requirements.rs"| Define the external executable requirements for an operation.| Support open-ended tool identities and version requirements.
"external_tools/resolution.rs"| Coordinate external-tool discovery and resolution.| Reuse "backend::toolchain".
"external_tools/validation.rs"| Apply toolchain security, policy and compatibility validation.| Reject unverified or incompatible executables.
"external_tools/reporting.rs"| Report installed, missing, incompatible and unverified tools.| Produce structured diagnostics.

Reuse:

- "backend/toolchain.rs"
- "backend/process.rs"
- "backend/config.rs"
- "backend/reproducibility.rs"

Do not implement a second executable scanner or process runner.

The integration must support future compiler, simulator, synthesis, verification and domain-specific executable providers.

18. Task subsystem

Directory: "src/toolchain/tasks/"

File| Complete responsibility| Integration and freeze requirements
"tasks/mod.rs"| Task subsystem composition and public API.| Expose canonical task contracts.
"tasks/model.rs"| Task identities, task state, inputs, outputs and lifecycle contracts.| Finalize before implementing the executor.
"tasks/graph.rs"| Task-graph access and task relationships.| Use the canonical "BuildGraph"; do not introduce a competing graph implementation.
"tasks/registry.rs"| Task-provider registration and discovery.| Use open-ended task/provider identities and invoke providers outside registry locks.
"tasks/executor.rs"| Execute validated tasks.| Apply budgets, cancellation, concurrency policies and failure propagation.
"tasks/dependencies.rs"| Resolve task prerequisites and input/output dependencies.| Coordinate with the canonical build graph.
"tasks/cancellation.rs"| Cancellation tokens, observations and propagation.| Reuse cancellation concepts from backend and runtime contracts where appropriate.
"tasks/reporting.rs"| Task outcomes, logs, timing and provenance references.| Preserve partial failures and deterministic reporting.

Task execution must support:

- Sequential execution.
- Parallel execution.
- Dependency-aware ordering.
- Cancellation.
- Failure propagation.
- Safe resumption where supported.
- Deterministic result aggregation.
- Configurable operational budgets.

It must not duplicate compiler parallel execution or runtime scheduling.

19. Developer commands

Directory: "src/toolchain/commands/"

File| Complete responsibility| Integration and freeze requirements
"commands/mod.rs"| Command registration, descriptors, dispatch, common authorization and result contracts.| One canonical command-dispatch boundary.
"commands/build.rs"| Build command adapter.| Delegate to "build/".
"commands/check.rs"| Validation and compilation-check command.| Invoke real compiler validation without falsely claiming artifact production.
"commands/clean.rs"| Remove recognized generated outputs and caches.| Use safe path validation and avoid deleting user-owned files.
"commands/run.rs"| Build and hand off a verified artifact to the runtime.| Delegate actual execution to ZUR.
"commands/test.rs"| Test command orchestration.| Integrate with canonical testing infrastructure and report actual test execution.
"commands/bench.rs"| Benchmark orchestration and reporting.| Preserve measurement provenance and comparable configuration.
"commands/format.rs"| Source-formatting command.| Reuse the canonical formatter; do not introduce another parser-based formatting engine.
"commands/lint.rs"| Lint command.| Aggregate canonical frontend, semantic, compiler and security diagnostics.
"commands/doc.rs"| Documentation generation.| Reuse documentation and source-analysis infrastructure.
"commands/package.rs"| Package creation command.| Delegate to "packages/".
"commands/install.rs"| Package installation command.| Delegate to dependencies, registry and package verification services.
"commands/publish.rs"| Package publishing command.| Require authorization, package verification and registry readiness.
"commands/target.rs"| Target inspection command.| Use canonical target and capability authorities.
"commands/tool.rs"| External tool inspection command.| Reuse "external_tools/".
"commands/doctor.rs"| Diagnose project, workspace, manifest, dependency, provider, toolchain and environment readiness.| Aggregate structured diagnostics without changing underlying error classifications.
"commands/deploy.rs"| Deployment command adapter.| Delegate to deployment planning and authorized providers.

Every command must:

- Be callable through the canonical API.
- Return structured outcomes.
- Have explicit authorization requirements.
- Support machine-readable output.
- Preserve failure semantics.
- Avoid duplicating underlying business logic.

Commands must not become separate implementations of the services they invoke.

20. Environment subsystem

Directory: "src/toolchain/environments/"

File| Complete responsibility| Integration and freeze requirements
"environments/mod.rs"| Environment composition and API.| Expose canonical environment operations.
"environments/discovery.rs"| Discover relevant development and execution environments.| Reuse operating-system and provider discovery authorities.
"environments/configuration.rs"| Apply explicit environment configuration.| Separate declared configuration from observed environment state.
"environments/fingerprints.rs"| Generate environment fingerprints.| Reuse compiler and backend reproducibility contracts.
"environments/validation.rs"| Validate environment requirements and compatibility.| Unknown environment properties must remain unresolved.

Environment fingerprints must distinguish reproducibility-relevant inputs from incidental machine information.

Do not include secrets in fingerprints or reports.

21. Deployment subsystem

Directory: "src/toolchain/deployment/"

File| Complete responsibility| Integration and freeze requirements
"deployment/mod.rs"| Deployment subsystem composition and API.| Coordinate deployment services without becoming an execution engine.
"deployment/request.rs"| Immutable deployment request and deployment intent.| Preserve project, artifact, environment and authorization identities.
"deployment/profiles.rs"| Deployment profile identity and policy composition.| Reuse canonical configuration profiles where possible.
"deployment/validation.rs"| Validate artifacts, prerequisites, security, compatibility and permissions.| Fail closed when mandatory evidence is missing.
"deployment/planning.rs"| Produce validated deployment plans.| Do not allocate physical devices or independently schedule execution.
"deployment/providers.rs"| Extensible deployment-provider contracts.| Support registered providers without vendor-specific implementation assumptions.
"deployment/reporting.rs"| Deployment outcomes and provenance.| Distinguish planned, accepted, submitted, running, completed, failed and unknown states.

Actual runtime execution belongs to ZUR and the relevant deployment providers.

Hardware allocation belongs to ZHAL and its providers.

Distributed execution belongs to distributed infrastructure and its runtime integrations.

22. Security subsystem

Directory: "src/toolchain/security/"

File| Complete responsibility| Integration and freeze requirements
"security/mod.rs"| Security subsystem composition and public API.| Expose security coordination boundaries.
"security/policy.rs"| Toolchain security policy evaluation and decision contracts.| Use the canonical policy and authorization authorities.
"security/trust.rs"| Trust identities, trust assessment and evidence references.| Reuse canonical identity and trust infrastructure.
"security/signatures.rs"| Signature-provider integration.| Do not implement cryptographic algorithms or private-key management.
"security/integrity.rs"| Coordinate integrity verification for packages, dependencies and artifacts.| Reuse canonical hashing and artifact verification.
"security/provenance.rs"| Toolchain lifecycle provenance composition.| Reuse compiler and artifact provenance rather than generating competing provenance.
"security/validation.rs"| Aggregate security validation and fail-closed operation acceptance.| Preserve unknown, denied, failed and indeterminate outcomes.

22.1 Mandatory security invariants

Security must distinguish:

- Authentication.
- Authorization.
- Trust.
- Integrity.
- Authenticity.
- Provenance.
- Capability.
- Isolation.

These concepts are not interchangeable.

The toolchain must fail closed when required verification cannot complete.

A provider's registration is not proof of trust.

A successful download is not proof of authenticity.

A valid hash is not proof of publisher identity.

A successful build is not proof of deployment authorization.

No sensitive credential may be written into ordinary diagnostics, logs or provenance metadata.

23. Diagnostics subsystem

Directory: "src/toolchain/diagnostics/"

File| Complete responsibility| Integration and freeze requirements
"diagnostics/mod.rs"| Public diagnostic composition and deliberate exports.| Re-export canonical diagnostic and aggregation types.
"diagnostics/diagnostic.rs"| Canonical toolchain diagnostic model, severity, category, phase, origin, metadata, source references, labels and fixes.| Reuse canonical frontend source spans. Do not redefine compiler diagnostic types.
"diagnostics/aggregation.rs"| Aggregate diagnostics, track statistics, deduplicate exact records, enforce retention policy and preserve completeness.| Delegate ordering to "ordering.rs". Use explicit budgets and report incomplete aggregation.
"diagnostics/ordering.rs"| Canonical deterministic comparison and sorting of toolchain diagnostics.| Sole owner of toolchain diagnostic ordering. Must be reused by aggregation and reporting.

23.1 Mandatory diagnostic corrections

The following rules are mandatory:

1. "ordering.rs" owns comparison.
2. "aggregation.rs" must not maintain a second comparator.
3. Aggregation must preserve deterministic ordering.
4. Exact structural duplicates may be deduplicated according to policy.
5. Hash collisions must not remove distinct diagnostics.
6. Floating-point metadata comparison must have a total-order policy consistent with equality.
7. Retention exhaustion must be observable.
8. Invalid diagnostics must mark the aggregate incomplete.
9. Allocation failures must not silently disappear.
10. Upstream incompleteness must propagate.
11. Dropped records must be counted.
12. Diagnostic ordering must not depend on hash-map iteration order.

An incomplete aggregate must never be reported as complete.

Human-readable rendering and machine-readable reporting must use these canonical records.

Do not create duplicate "rendering.rs" or "reporting.rs" unless a separately justified implementation is required by the actual tree. The current audited tree does not contain those files.

If rendering/reporting functionality is required, first determine whether it belongs in existing diagnostic or command APIs. Add files only after that ownership decision is documented.

24. Agent subsystem

Directory: "src/toolchain/agents/"

File| Complete responsibility| Integration and freeze requirements
"agents/mod.rs"| Agent subsystem composition and public API.| Expose the canonical agent workflow boundary.
"agents/context.rs"| Validated project, workspace, operation and provenance context for agents.| Reuse canonical API and diagnostic structures.
"agents/requests.rs"| Agent request envelope and request validation.| Adapt canonical toolchain API requests rather than defining separate operation semantics.
"agents/results.rs"| Structured agent workflow results and advisory next actions.| Preserve canonical command and toolchain outcomes.
"agents/permissions.rs"| Authorization integration for agent-requested operations.| Use the authoritative authorization system; agent claims are not credentials.
"agents/audit.rs"| Agent operation audit-event composition and audit-sink integration.| Reuse canonical audit persistence and identity infrastructure.
"agents/compliance.rs"| Agent workflow compliance evaluation.| Fail closed on required compliance failures; compliance evidence must originate from authoritative controls.

24.1 Human and AI parity

Humans and AI agents must use the same:

- Public API.
- Build request.
- Manifest validation.
- Dependency resolution.
- Build-plan validation.
- Compiler API.
- Target compatibility rules.
- Artifact verification.
- Security policies.
- Error semantics.
- Deployment services.

Agents must not receive a separate unrestricted execution path.

An agent can request a target assessment but cannot invent capability evidence or declare physical feasibility.

Agent suggestions are advisory. Authorization and compliance decisions remain authoritative.

25. Documentation subsystem

Directory: "src/toolchain/documentation/"

File| Complete responsibility| Integration and freeze requirements
"documentation/ARCHITECTURE.md"| Overall architecture, ownership and lifecycle.| Must agree with the actual module tree.
"documentation/INTEGRATION_CONTRACT.md"| Cross-subsystem interfaces and dependency direction.| Must agree with actual repository APIs and ownership.
"documentation/BUILD_CONTRACT.md"| Build lifecycle and acceptance requirements.| Must describe actual compiler invocation and artifact verification.
"documentation/PACKAGE_CONTRACT.md"| Package format, lifecycle, compatibility and security.| Must match the implemented ".zpkg" format.
"documentation/PORTABILITY_CONTRACT.md"| POCO-REAF semantic portability and target realization.| Must distinguish portable meaning from physical feasibility.
"documentation/SECURITY_CONTRACT.md"| Security boundaries, trust, authorization and supply-chain protection.| Must agree with canonical security implementations.
"documentation/AGENT_CONTRACT.md"| Human/AI parity, request handling and permission boundaries.| Must match agent API and authorization implementations.
"documentation/READINESS_MATRIX.md"| Normative per-file implementation and production acceptance matrix.| This document. It must be updated with implementation evidence, not optimistic status claims.

Documentation must not claim that a capability is production-ready solely because its file or interface exists.

26. Repository-wide integration requirements

The toolchain cannot become production-ready in isolation.

The following changes are required in other repository locations.

26.1 "src/lib.rs"

Current inspected condition:

The crate root exposes frontend, compiler, ZUIR, computational domains, backend, ZHAL, ZUR, stdlib and project configuration.

It does not declare "pub mod toolchain;".

Required correction:

Expose the toolchain using the established feature policy:

#[cfg(feature = "buildsystem")]
pub mod toolchain;

This must be implemented only after confirming that all declared modules and public dependencies compile under the feature.

The library's default feature set should remain lightweight unless a deliberate release decision changes it.

26.2 "Cargo.toml"

Current inspected condition:

- Rust edition 2021.
- "rust-version = "1.97"".
- "buildsystem = []".
- "zamani-pkg" requires "buildsystem".
- Existing dependencies are declared at package level.

Required work:

1. Preserve Rust 1.97 as the minimum.
2. Preserve edition 2021.
3. Preserve the prohibition on unsafe Rust.
4. Define deliberate feature boundaries.
5. Associate new dependencies with the features that require them where practical.
6. Avoid unnecessary dependencies.
7. Avoid duplicate libraries for capabilities already provided by the repository.
8. Ensure the package manager compiles using only supported public APIs.
9. Ensure feature combinations are tested independently.
10. Keep "Cargo.lock" consistent with deliberate dependency changes.

The "buildsystem" feature must not imply that every future domain implementation is available.

Feature activation means the relevant compiled API is available, not that every provider is installed.

26.3 "src/zamani_project_config/"

The manifest subsystem must be expanded in its authoritative location.

The implementation must provide:

- Package metadata.
- Dependency declarations.
- Workspace configuration.
- Feature declarations.
- Build configuration.
- Target intent.
- Profile declarations.
- External tool requirements.
- Registry configuration.
- Packaging policies.
- Testing configuration.
- Deployment intent.
- Schema versioning.
- Validation.
- Compatibility and migration.

Suggested file structure must be determined after inspecting the current directory tree and its consumers. Do not replace existing files blindly or duplicate existing types.

The existing "ZamaniToml", "PackageSection" and "DependencyConfig" contracts must be preserved through intentional compatibility migration.

The parser must validate actual field types and required fields. It must not silently convert malformed values into empty strings or default values.

26.4 "src/bin/zamani_pkg_main.rs"

The current CLI is not a production package-manager implementation.

Its inspected implementation:

- Parses a limited set of commands.
- Loads "ZamaniToml".
- References "toolchain::package_manager::PackageManager".
- Calls "build::compile_project".
- Prints that backend invocation is simulated.

This must be corrected.

The final CLI architecture must be:

CLI arguments
      |
      v
Command parsing
      |
      v
Canonical toolchain API
      |
      v
Command authorization
      |
      v
Command dispatch
      |
      v
Workspace / manifest / dependency services
      |
      v
Build / package / registry / deployment services
      |
      v
Structured response
      |
      v
Human or machine-readable output

The CLI must not implement its own:

- Manifest parser.
- Dependency resolver.
- Package archive implementation.
- Build executor.
- Compiler driver.
- Registry client.
- Security engine.

Remove simulated success messages.

A command must not be advertised as implemented until its actual service is integrated and tested.

The obsolete "package_manager" import must be removed or replaced by the canonical package and dependency APIs. Do not create a duplicate "package_manager" subsystem to satisfy the old import.

26.5 Compiler integration

Required authorities:

- "src/compiler/api/"
- "src/compiler/analysis/"
- "src/compiler/planning/"
- "src/compiler/artifacts/"
- "src/compiler/incremental/"
- "src/compiler/reproducibility/"
- "src/compiler/security/"

The toolchain must call the public compiler service.

It must not import private compiler implementation details or execute individual compiler passes directly.

The compiler must remain responsible for:

- Compilation semantics.
- Semantic validation.
- Capability analysis.
- Requirement analysis.
- Feasibility assessment.
- Optimization.
- Lowering.
- Compilation planning.
- Canonical IR transformations.
- Compiler artifact production.

The toolchain is responsible for coordinating project-level compilation requests and reporting their outcomes.

26.6 Backend integration

Reuse:

- "src/backend/api.rs"
- "src/backend/registry.rs"
- "src/backend/target.rs"
- "src/backend/capabilities.rs"
- "src/backend/compatibility.rs"
- "src/backend/toolchain.rs"
- "src/backend/process.rs"
- "src/backend/artifact.rs"
- "src/backend/reproducibility.rs"
- "src/backend/security.rs"
- "src/backend/validation.rs"

The toolchain must not independently generate target code or launch unmanaged external processes.

Backend output must be validated by its authoritative artifact contracts.

26.7 Hardware and runtime integration

Use:

- "src/zhal/" for hardware abstraction and physical hardware-provider integration.
- "src/zur/" for runtime and application execution.
- "src/distributed/" for distributed infrastructure and its canonical interfaces.

The toolchain must distinguish:

1. Project validated.
2. Dependencies resolved.
3. Build plan validated.
4. Feasibility assessed.
5. Compilation completed.
6. Artifact verified.
7. Execution environment validated.
8. Resources acquired.
9. Application launched.
10. Application completed.

These are different lifecycle states.

A successful compilation must not be represented as a successful execution.

27. Grammar integration and language boundaries

The grammar is the authoritative source-language specification.

The toolchain must consume the grammar's declared intent through the canonical frontend and semantic pipeline.

It must not interpret grammar productions as proof that an implementation service exists.

27.1 Grammar authority

The relevant grammar architecture includes:

grammar/
├── DESIGN.md
├── FREEZE.md
├── MANIFEST.yaml
├── OWNERSHIP.yaml
├── DEPENDENCIES.yaml
├── STATUS.yaml
├── Zamani.g4
│
├── antlr/
├── core/
├── modules/
├── compile/
├── resources/
├── hardware/
├── classical/
├── quantum/
├── hybrid/
├── hdl/
├── ai/
├── distributed/
├── execution/
├── interoperability/
├── validation/
├── compatibility/
├── specification/
└── spec/

The actual tree contains additional grammar directories and domain-specific productions. This listing identifies the principal integration areas, not a replacement grammar inventory.

27.2 Relevant source-level package and dependency grammar

The inspected files include:

- "grammar/modules/packages.g4"
- "grammar/modules/dependencies.g4"

"packages.g4" owns package declaration syntax.

"dependencies.g4" owns dependency declaration syntax, including identity, aliases, optional declarations, version constraints, sources, features, capability requirements and metadata.

Neither grammar owns:

- Package archive serialization.
- Package downloading.
- Package installation.
- Dependency solving.
- Registry authentication.
- Publishing.
- Hardware selection.
- Physical allocation.

Those operations belong to the toolchain and their authoritative providers.

The grammar's package identity must not be confused with the toolchain's package identity.

The source-level package declaration must be transformed through the canonical AST and semantic pipeline before it is used as package-management input.

27.3 Compilation grammar

The inspected compilation grammar includes:

- "grammar/compile/compile.g4"
- "grammar/compile/target.g4"
- "grammar/compile/target-selection.g4"
- "grammar/compile/cross-compilation.g4"
- "grammar/compile/profiles.g4"
- "grammar/compile/features.g4"
- "grammar/compile/reproducibility.g4"
- "grammar/compile/deterministic-builds.g4"
- "grammar/compile/artifacts.g4"
- "grammar/compile/provenance.g4"
- "grammar/compile/deployment.g4"

These grammars express source-level compilation intent.

They do not implement compilation.

The toolchain must translate validated source and manifest intent into canonical build requests without interpreting grammar syntax independently.

Target-selection grammar must not become a hardware discovery system.

Compilation grammar must not become a compiler implementation.

27.4 Core and semantic ownership

The inspected "grammar/core/README.md" establishes the common domain-neutral syntax boundary.

The inspected "grammar/compile/README.md" establishes that compilation syntax expresses intent and that the compiler determines realization.

The inspected "grammar/validation/semantic-boundaries.md" establishes the separation between source semantics and physical realization.

The toolchain must preserve these boundaries.

27.5 Hard-coding and scalability policies

The following files are normative inputs to this matrix:

- "grammar/validation/hard-coding.md"
- "grammar/validation/hardcoding-audit.md"
- "grammar/validation/scalability-rules.md"
- "grammar/validation/portability.md"
- "grammar/validation/semantic-boundaries.md"
- "grammar/specification/portability.md"
- "grammar/compatibility/versions.md"
- "grammar/OWNERSHIP.yaml"
- "grammar/DEPENDENCIES.yaml"

The toolchain must not introduce a hard-coded limit that contradicts these policies.

27.6 Grammar conformance integration

The following workflow is mandatory:

Normative language specification
          |
          v
Canonical grammar
          |
          v
Canonical lexer/parser
          |
          v
Frontend AST
          |
          v
Semantic validation
          |
          v
Canonical semantic model
          |
          v
Canonical IRs
          |
          v
Compiler planning
          |
          v
Build orchestration
          |
          v
Backend realization
          |
          v
Verified artifact

The toolchain must consume the output of this workflow.

It must not add a second source parser for build configuration or domain semantics.

TOML project configuration remains under "zamani_project_config", distinct from the source-language ANTLR grammar.

28. Domain integration requirements

The toolchain must support all computing domains through the same project and build infrastructure.

It must not create a parallel domain compiler inside "src/toolchain/".

Domain| Canonical integration| Toolchain responsibility
Classical| "src/classical/"| Project/build coordination and compatible artifact packaging
Systems| "src/classical/", "src/compiler/", "src/backend/"| Native and cross-compilation workflows
Embedded| "src/classical/", "src/zhal/", "src/backend/"| Embedded target profiles and deployment coordination
Quantum| "src/quantum/"| Quantum build requests, simulator/QPU provider coordination and artifact packaging
Quantum simulation| "src/quantum/", "src/backend/", "src/zur/"| Simulation build and execution handoff
HDL| "src/hdl/"| HDL build, synthesis and verification orchestration
FPGA/ASIC| "src/hdl/", "src/zhal/", "src/backend/"| Synthesis and hardware artifact workflow coordination
AI/ML| "src/ai/"| Model, training and inference build orchestration
Tensor| Canonical tensor infrastructure and "src/ai/"| Tensor-aware artifact and dependency integration
Hybrid| "src/hybrid/"| Cross-domain build composition and interface validation
Distributed| "src/distributed/"| Distributed dependency and deployment coordination
Networking| Networking domain authority| Network-aware project and provider integration
Cryptography| Cryptographic domain and canonical security authority| Secure package and artifact workflows
Future domains| Registered domain/provider contracts| Extensible discovery, compatibility and orchestration

28.1 Hybrid integration

A hybrid project must not be forced into one artificial target.

One project may contain:

- Classical components.
- Quantum components.
- HDL components.
- AI components.
- Distributed components.

The toolchain must preserve domain-specific artifact identities and cross-domain dependency relationships.

The compiler and domain authorities determine semantic compatibility.

28.2 Distributed integration

The toolchain may coordinate distributed build tasks and deployment requests.

It must not create a second distributed scheduler, membership service, transport implementation or execution engine.

29. Missing architectural elements and justified additions

The following are not present as dedicated files in the audited "src/toolchain/" tree.

They must not be added automatically.

First determine whether the existing files can own the responsibility without becoming overloaded.

Potential addition| Decision| Justification
"testing/"| Add only if needed after integrating repository test infrastructure| Toolchain conformance tests must exist, but a second test framework is unnecessary.
"observability/"| Do not add initially| Reuse diagnostics, task reporting, compiler profiling and existing telemetry.
"cache/"| Do not add| Reuse "compiler/incremental/" and existing artifact/cache authorities.
"package_manager/"| Prohibited as a competing subsystem| Packages and dependencies already own the relevant responsibilities.
"hardware/"| Prohibited as a competing subsystem| Reuse ZHAL, backend and compiler planning.
"compiler/"| Prohibited| The compiler already exists.
"runtime/"| Prohibited| ZUR and runtime infrastructure own execution.
"providers/"| Do not create a duplicate global registry| Reuse the individual canonical provider registries.
"projects/"| Do not create initially| Workspace and manifest already cover project orchestration.
"rendering.rs" / "reporting.rs" in diagnostics| Defer| The audited tree does not contain them. Determine whether current command/API reporting can own these responsibilities first.

Any addition must have:

1. A documented responsibility.
2. A documented owner.
3. A complete API contract.
4. Known dependencies.
5. Known consumers.
6. Integration tests.
7. A justified location.
8. No duplicated authority.

30. Implementation ordering

Implementation must begin with independent foundations.

The order below is a dependency order, not a requirement to implement every subsystem in one release.

A file must be fully specified before implementation begins.

Phase 0 — Contract reconciliation

Before changing implementation code:

1. Reconcile "ARCHITECTURE.md".
2. Reconcile "INTEGRATION_CONTRACT.md".
3. Reconcile "BUILD_CONTRACT.md".
4. Reconcile "PACKAGE_CONTRACT.md".
5. Reconcile "PORTABILITY_CONTRACT.md".
6. Reconcile "SECURITY_CONTRACT.md".
7. Reconcile "AGENT_CONTRACT.md".
8. Establish this "READINESS_MATRIX.md".
9. Confirm every module declaration against the actual tree.
10. Confirm the canonical public APIs of compiler, backend, ZHAL, ZUR and project configuration.
11. Confirm grammar ownership and versioning.
12. Record unresolved integration contracts.

No implementation should begin against a guessed API.

Phase 1 — Independent foundation

Implement and freeze:

1. "errors.rs"
2. "version.rs"
3. "ids.rs"
4. "result.rs"
5. "configuration/budget.rs"
6. "configuration/policies.rs"
7. "configuration/profiles.rs"
8. "tasks/model.rs"
9. "dependencies/model.rs"
10. "dependencies/constraints.rs"
11. "packages/model.rs"
12. "registries/protocol.rs"
13. "registries/provider.rs"
14. "targets/profiles.rs"
15. "diagnostics/diagnostic.rs"
16. "diagnostics/ordering.rs"

These contracts establish the types and invariants required by downstream services.

Do not start executors or large orchestration services before their foundational contracts are frozen.

Phase 2 — Configuration, workspace and manifest

Implement:

1. "configuration/config.rs"
2. "configuration/validation.rs"
3. "workspace/discovery.rs"
4. "workspace/layout.rs"
5. "workspace/members.rs"
6. "workspace/metadata.rs"
7. "workspace/validation.rs"
8. "manifest/loading.rs"
9. "manifest/adapter.rs"
10. "manifest/validation.rs"
11. "manifest/compatibility.rs"

Integrate the canonical "zamani_project_config" implementation.

Freeze the manifest and workspace contracts before building dependency resolution on top of them.

Phase 3 — Dependency resolution

Implement:

1. "dependencies/sources.rs"
2. "dependencies/features.rs"
3. "dependencies/graph.rs"
4. "dependencies/compatibility.rs"
5. "dependencies/resolver.rs"
6. "dependencies/validation.rs"
7. "dependencies/lockfile.rs"

The resolver must produce deterministic, verifiable resolution results.

Phase 4 — Package infrastructure

Implement:

1. "packages/metadata.rs"
2. "packages/archive.rs"
3. "packages/verification.rs"
4. "packages/creation.rs"
5. "packages/extraction.rs"
6. "packages/installation.rs"

The archive and verification contracts must be frozen before registry publishing begins.

Phase 5 — Registry and external-tool integration

Implement:

1. "registries/registry.rs"
2. "registries/discovery.rs"
3. "registries/authentication.rs"
4. "registries/validation.rs"
5. "registries/client.rs"
6. "registries/downloading.rs"
7. "registries/publishing.rs"
8. "registries/federation.rs"
9. "external_tools/requirements.rs"
10. "external_tools/resolution.rs"
11. "external_tools/validation.rs"
12. "external_tools/reporting.rs"

Phase 6 — Targets and environments

Implement:

1. "targets/compatibility.rs"
2. "targets/selection.rs"
3. "targets/matrix.rs"
4. "targets/validation.rs"
5. "environments/discovery.rs"
6. "environments/configuration.rs"
7. "environments/fingerprints.rs"
8. "environments/validation.rs"

These must consume canonical compiler and backend capabilities.

Phase 7 — Task and build orchestration

Implement:

1. "tasks/dependencies.rs"
2. "tasks/graph.rs"
3. "tasks/registry.rs"
4. "tasks/cancellation.rs"
5. "tasks/reporting.rs"
6. "tasks/executor.rs"
7. "build/request.rs"
8. "build/profiles.rs"
9. "build/inputs.rs"
10. "build/plan.rs"
11. "build/graph.rs"
12. "build/outputs.rs"
13. "build/executor.rs"
14. "build/incremental.rs"
15. "build/reporting.rs"

A real classical compiler invocation must be demonstrated before declaring the first build milestone complete.

Phase 8 — Security, diagnostics and agents

Implement:

- Security policy and provider integration.
- Package and artifact verification.
- Provenance.
- Diagnostic aggregation.
- Deterministic ordering.
- Agent authorization.
- Agent compliance.
- Audit integration.
- Human/AI API parity.

These services must be integrated into the canonical command lifecycle rather than left as isolated modules.

Phase 9 — Developer commands and deployment

Implement and integrate every command through the canonical dispatcher.

No command may report a successful result when its service is simulated, unavailable or unimplemented.

Phase 10 — Domain expansion

Expand the shared toolchain through real providers and existing subsystem authorities.

Start with the classical computing workflow, then add verified quantum, HDL, AI, hybrid and distributed workflows.

The shared project, dependency, package and build architecture must remain unchanged.

Phase 11 — Production release

Perform the full repository and downstream acceptance process.

31. Required testing architecture

The existing tree has tests distributed throughout the individual toolchain modules.

Keep focused unit tests with their owning modules.

Add integration tests under the repository's existing "tests/" structure rather than automatically creating a duplicate test framework.

Recommended structure:

tests/
└── toolchain/
    ├── api.rs
    ├── workspace.rs
    ├── manifest.rs
    ├── configuration.rs
    ├── dependencies.rs
    ├── packages.rs
    ├── registries.rs
    ├── targets.rs
    ├── external_tools.rs
    ├── tasks.rs
    ├── build.rs
    ├── commands.rs
    ├── deployment.rs
    ├── security.rs
    ├── diagnostics.rs
    ├── agents.rs
    ├── portability.rs
    ├── reproducibility.rs
    └── downstream/
        ├── classical.rs
        ├── quantum.rs
        ├── hdl.rs
        ├── ai.rs
        ├── hybrid.rs
        └── distributed.rs

This is a proposed test organization, not an assertion that these files already exist.

31.1 Unit tests

Every module must test:

- Valid construction.
- Invalid construction.
- Boundary conditions.
- Overflow.
- Allocation failures where testable.
- Serialization and deserialization.
- Version compatibility.
- Error preservation.
- Determinism.
- Thread-safety contracts.
- Cancellation.
- Resource-budget exhaustion.
- Unknown and unsupported states.

31.2 Integration tests

Integration tests must verify actual service interactions.

Mocks may be used for isolated contract tests.

Mocks must not be presented as proof of real compiler, hardware, network, registry or runtime support.

31.3 Scalability tests

Test increasing workloads without introducing arbitrary architectural ceilings.

Examples:

- Small and large projects.
- Large workspaces.
- Deep and broad dependency graphs.
- Many registered providers.
- Large build graphs.
- Multiple target realizations.
- Large package archives.
- Extensive diagnostic collections.
- Parallel task execution.
- Distributed workflows.

Measure:

- Time.
- Memory.
- Storage.
- Throughput.
- Allocation behavior.
- Cancellation latency.
- Failure recovery.
- Determinism.

Resource exhaustion must produce an honest, structured result.

31.4 Security tests

Test:

- Malicious manifests.
- Malformed dependency specifications.
- Path traversal.
- Malicious archives.
- Symlink attacks.
- Archive resource exhaustion.
- Registry spoofing.
- Invalid signatures.
- Invalid credentials.
- Unauthorized publication.
- Provider impersonation.
- Credential leakage.
- Incomplete provenance.
- Interrupted installations.
- Failed rollback.
- Malicious external-tool paths.

32. Production-readiness gates

A toolchain release must satisfy every applicable mandatory gate.

Gate| Required evidence
Architecture| Ownership and dependency contracts are frozen
Public API| External consumers compile against documented interfaces
Manifest| Canonical parser and schema validation work
Workspace| Real discovery and membership validation work
Dependencies| Deterministic resolution and lockfiles work
Packages| Archive creation, verification, extraction and installation work
Registries| Real provider integration and authenticated operations work
Targets| Canonical compatibility and feasibility evidence are used
External tools| Real executable discovery and safe process execution work
Tasks| Dependency-aware execution, cancellation and failure propagation work
Build| Real compiler invocation and artifact verification work
Incremental| Canonical compiler cache integration works
Commands| Advertised commands execute actual services
Security| Required checks fail closed
Diagnostics| Structured and deterministic results work
Agents| Human/AI operation parity and authorization work
Portability| Multiple compatible realizations preserve semantics
Reproducibility| Declared build inputs and reproducibility evidence are verified
Scalability| Increasing workloads scale according to available resources
Documentation| Contracts match actual implementation
Downstream| Real consumer projects work

A failed mandatory gate prevents a production-ready designation.

33. POCO-REAF conformance

The toolchain must preserve the following invariants.

33.1 Source identity

The same source project must retain a stable logical identity across compatible target realizations.

33.2 Dependency identity

Dependency resolution must preserve the exact selected dependency identities and source evidence.

33.3 Build identity

A logical build must retain its identity across its target-specific operations.

Target-specific outcomes must remain separately identifiable.

33.4 Semantic preservation

Target-specific compilation must preserve the applicable program semantics.

A target that cannot satisfy mandatory requirements must be rejected or reported as infeasible.

33.5 Artifact provenance

Every production artifact must be traceable to:

- Source identity.
- Project identity.
- Dependency resolution.
- Build configuration.
- Compiler identity.
- Backend identity.
- Target identity.
- Relevant provider versions.
- Verification evidence.

33.6 Future compatibility

New computing domains and hardware implementations must be integrated through open provider contracts.

Adding a new domain must not require redesigning the project and package management architecture.

34. Definition of done for an individual file

A file may be marked complete only when all applicable conditions are satisfied.

- [ ] Canonical responsibility is documented.
- [ ] No duplicate subsystem ownership exists.
- [ ] Public API is fully defined.
- [ ] Dependencies are known.
- [ ] All consumers are identified.
- [ ] All required integrations are identified.
- [ ] Error semantics are complete.
- [ ] Failure semantics are complete.
- [ ] Serialization contracts are complete where applicable.
- [ ] Concurrency semantics are complete where applicable.
- [ ] Security requirements are satisfied.
- [ ] Scalability requirements are satisfied.
- [ ] No arbitrary universal hardware limits exist.
- [ ] Rust 1.97+ compatibility is verified.
- [ ] Rust 2021 edition compatibility is verified.
- [ ] No unsafe Rust is used.
- [ ] Unit tests pass.
- [ ] Integration tests pass.
- [ ] Downstream consumers compile.
- [ ] Documentation is complete.
- [ ] Freeze evidence is recorded.

The readiness matrix must record evidence, not merely a checkbox.

35. Mandatory build and release commands

The minimum repository verification commands are:

cargo fmt --all -- --check

cargo check --all-targets

cargo test --all-targets

cargo clippy --all-targets -- -D warnings

Build-system feature verification:

cargo check --all-targets --features buildsystem

cargo test --all-targets --features buildsystem

Package-manager verification:

cargo build --release \
  --features buildsystem \
  --bin zamani-pkg

Additional verification must include:

- Feature-combination testing.
- Public API compatibility.
- Downstream compilation.
- Cross-platform builds.
- Reproducibility.
- Package-format compatibility.
- Archive security.
- Registry integration.
- External-tool failures.
- Cancellation.
- Resource exhaustion.
- Multi-target compilation.
- Domain-specific provider integration.
- Release artifact verification.

The project must also verify that no unsafe Rust is introduced.

36. Final architectural decisions

Question| Binding decision
Does toolchain own compilation?| No. It orchestrates the compiler.
Does toolchain own projects and workspaces?| Yes.
Does toolchain own manifest parsing?| No. "zamani_project_config" owns it.
Does toolchain own dependency resolution?| Yes.
Does toolchain own packages?| Yes.
Does toolchain own registries?| Yes, through extensible providers.
Does toolchain own build orchestration?| Yes.
Does toolchain own task orchestration?| Yes, within its build workflows.
Does toolchain own compiler scheduling?| No.
Does toolchain own external process execution?| No.
Does toolchain own external tool discovery?| No.
Does toolchain own target-specific code generation?| No.
Does toolchain own physical hardware discovery?| No.
Does toolchain own physical resource allocation?| No.
Does toolchain own runtime execution?| No.
Does toolchain own cryptographic algorithms?| No.
Does toolchain own security coordination?| Yes.
Does toolchain support human and AI clients?| Yes, through the same canonical services.
Does toolchain support future computing domains?| Yes, through extensible contracts.
Does toolchain impose arbitrary hardware ceilings?| No.
Does unlimited budget mean infinite physical resources?| No.
Does every program have to run on every device?| No. Compatibility and feasibility must be established.

37. First implementation milestone

The first milestone must not attempt to implement the entire computing ecosystem.

It must establish a genuinely functioning toolchain foundation.

The milestone consists of:

Deliverable A — Frozen contracts

- Root error and result contracts.
- Version contracts.
- Identity contracts.
- Budget contracts.
- Public API contracts.
- Ownership and integration contracts.

Deliverable B — Real project management

- Manifest integration.
- Workspace discovery.
- Workspace validation.
- Project metadata.
- Configuration validation.

Deliverable C — Reproducible dependencies

- Dependency models.
- Constraint evaluation.
- Source integration.
- Dependency graph.
- Resolver.
- Lockfile.

Deliverable D — Real package infrastructure

- Package model.
- Metadata format.
- Archive format.
- Verification.
- Safe extraction.
- Installation.

Deliverable E — Actual classical build

- Real compiler service.
- Validated build request.
- Build plan.
- Build graph.
- Task execution.
- Backend integration.
- Verified artifact.
- Build report.

The milestone is complete only when a real Zamani classical project can be built successfully through the public toolchain API and CLI, with actual artifact verification.

A simulated compiler, mock backend or fabricated successful build does not satisfy this milestone.

38. Final production-readiness principle

The Zamani toolchain must be one unified development and distribution ecosystem.

It must allow developers to create one logical project, express portable computational intent, resolve reproducible dependencies, build through canonical compiler services, obtain compatible target-specific realizations, package verified outputs and hand them to authorized execution infrastructure.

It must scale according to available resources without turning current hardware characteristics into permanent language limitations.

It must be extensible without introducing duplicate systems.

It must preserve semantic meaning while allowing implementation and physical realization to evolve.

The governing architectural principle is:

One language. One semantic authority. One project ecosystem. One coordinated toolchain. Multiple compatible realizations. Open-ended computing domains. Honest feasibility. Verified execution.

POCO-REAF is achieved through semantic portability, reproducible build specifications, extensible realization providers, verified artifacts and compatibility-aware evolution—not through a false promise that every computation can execute on every physical machine.

Production readiness is established by implementation and test evidence, never by directory structure, module declarations, grammar productions or architectural documentation alone.

---

End of READINESS_MATRIX.md