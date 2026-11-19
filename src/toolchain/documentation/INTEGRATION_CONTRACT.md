Zamani Toolchain — Integration Contract

Canonical path: "src/toolchain/documentation/INTEGRATION_CONTRACT.md"
Repository: "Benwellonedge28/Zamani"
Branch audited: "main"
Language: Zamani
Rust edition: 2021
Minimum supported Rust version: 1.97
Safety: Unsafe Rust is forbidden
Architecture: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Document classification: Normative cross-subsystem integration contract
Status: Proposed authoritative contract; implementation conformance must be demonstrated independently.

---

1. Purpose

This document establishes the mandatory integration contracts between:

- "src/toolchain/"
- "src/frontend/"
- "src/classical/"
- "src/quantum/"
- "src/hdl/"
- "src/ai/"
- "src/hybrid/"
- "src/distributed/"
- "src/zuir/"
- "src/compiler/"
- "src/backend/"
- "src/zhal/"
- "src/zur/"
- "src/runtime/"
- "src/stdlib/"
- "src/zamani_project_config/"
- "grammar/"
- Repository-level CLI applications and integration tests.

Its purpose is to ensure that the Zamani toolchain can coordinate the complete software development lifecycle without creating competing implementations of existing repository responsibilities.

The toolchain must provide one coherent development, building, testing, packaging, distribution and deployment-preparation architecture for the entire Zamani ecosystem.

It must support:

1. Classical computing.
2. Systems programming.
3. Embedded computing.
4. Parallel and concurrent computing.
5. Scientific and numerical computing.
6. Quantum computing.
7. Quantum simulation.
8. Quantum-classical hybrid computing.
9. HDL and hardware/software co-design.
10. FPGA and ASIC development.
11. Artificial intelligence.
12. Machine learning and deep learning.
13. Tensor and accelerator computing.
14. Symbolic and probabilistic computing.
15. Distributed computing.
16. Networking and communication.
17. Cryptographic and secure computation.
18. Neuromorphic, photonic, analog and future computing technologies.

These are not separate project-management systems or independent programming languages.

They must participate in the same toolchain architecture through their existing semantic, compiler, IR, backend, hardware and runtime authorities.

This document specifies integration contracts. It does not certify that every listed capability has already been implemented.

2. Normative terminology

The following terminology is mandatory.

- MUST: Required.
- MUST NOT: Prohibited.
- SHOULD: Strong recommendation requiring justification when not followed.
- SHOULD NOT: Generally prohibited unless justified.
- MAY: Optional.

The following terms have specific architectural meanings:

Term| Meaning
Project| A logical Zamani development unit
Workspace| A collection of related projects
Manifest| Canonical project configuration
Build request| A caller's requested build operation
Build plan| Validated and immutable build orchestration description
Build task| One independently identifiable build operation
Target| A canonical compiler-recognized target
Provider| A registered implementation of an integration contract
Capability| An established implementation capability
Requirement| A property a computation needs
Preference| A desirable but nonmandatory property
Constraint| A condition that an acceptable realization must obey
Hint| Nonbinding information that may guide planning
Feasibility| An evidence-based determination of whether a realization is possible
Realization| A target-specific implementation of program semantics
Artifact| A compilation or packaging output
Provenance| Verifiable information describing the origin and transformations of an artifact

These concepts must not be treated as interchangeable.

3. Architectural authority and precedence

Every subsystem must have exactly one canonical owner for each concept.

The following authority hierarchy governs integration.

Authority| Owns
"grammar/"| Source-language syntax and language specifications
"frontend/"| Lexing, parsing, AST and source-language semantic integration
"frontend::semantic"| Source-level names, types, effects, capabilities, requirements, constraints and semantic contracts
"zuir/"| Universal intermediate representation
"classical/"| Classical computational representations and operations
"quantum::ir"| Canonical quantum intermediate representation
"hdl/hdlir"| Hardware-description representation
"ai/"| AI computation interfaces and domain integration
"hybrid/"| Hybrid computational integration
"distributed/"| Distributed-computation infrastructure
"compiler/"| Compilation orchestration, analysis, planning and compilation services
"backend/"| Backend registration, target-specific generation and external tool execution integration
"zhal/"| Physical hardware abstraction and hardware-provider integration
"zur/"| Runtime and execution infrastructure
"runtime/"| Existing runtime implementation and integration
"stdlib/"| Shared provider, capability, context and common contracts
"zamani_project_config/"| Canonical project manifest and configuration schema
"toolchain/"| Project, workspace, dependency, package, build, task, developer workflow and deployment orchestration

The toolchain may adapt existing types and services, but it MUST NOT create another authoritative owner for them.

3.1 Dependency direction

The intended integration architecture is:

             Zamani source
                   |
                   v
             grammar/
                   |
                   v
               frontend
                   |
                   v
          Semantic authority
                   |
                   v
          Canonical domain IRs
                   |
                   v
                compiler
                   |
          +--------+--------+
          |                 |
          v                 v
      optimizer          planning
          |                 |
          +--------+--------+
                   |
                   v
                 backend
                   |
                   v
                  ZHAL
                   |
                   v
                  ZUR
                   |
                   v
                runtime

The toolchain coordinates the development lifecycle around this architecture:

             Zamani project
                   |
                   v
                Toolchain
                   |
       +-----------+-----------+
       |           |           |
       v           v           v
   Workspace   Manifest   Dependencies
       |           |           |
       +-----------+-----------+
                   |
                   v
              Build request
                   |
                   v
              Build planning
                   |
                   v
              Build execution
                   |
                   v
                Compiler
                   |
                   v
             Verified artifacts
                   |
          +--------+--------+
          |        |        |
          v        v        v
       Package  Test/Run  Deployment
          |        |        |
          v        v        v
       Registry   ZUR    Deployment
                           providers

The toolchain MUST NOT become an alternative compiler pipeline or hardware execution system.

4. POCO-REAF integration contract

The primary objective is:

Program Once, Compile Once, Run Everywhere, Anywhere, Forever.

POCO-REAF must be implemented as a semantic portability and reproducible realization contract.

It requires:

1. One canonical source program.
2. One consistent semantic interpretation.
3. One logical project identity.
4. One reproducible dependency description.
5. One logical build request.
6. Reusable build specifications.
7. Multiple compatible target-specific realizations.
8. Verified target-specific artifacts.
9. Explicit compatibility evidence.
10. Explicit feasibility results.
11. Reproducible build identities.
12. Versioned compatibility contracts.
13. Runtime handoff through authoritative runtime infrastructure.

A single logical build request may produce several artifacts.

For example:

One Zamani source project
          |
          v
    One build request
          |
          v
    Target resolution
          |
     +----+-----+--------+
     |          |        |
     v          v        v
   Native      WASM     Accelerator
     |          |        |
     v          v        v
  Artifact   Artifact  Artifact

The artifacts may differ substantially in representation.

They must preserve the applicable source-language semantics.

POCO-REAF does not promise that:

- Every program executes on every physical machine.
- Every provider implements every computational domain.
- Every target generates identical machine code.
- A successful build guarantees successful execution.
- Hardware resources are infinite.

An incompatible or infeasible realization must be reported explicitly.

Source changes must not be demanded merely because a compatible execution environment has changed.

5. Scalability contract

5.1 Fundamental rule

The toolchain MUST NOT introduce arbitrary universal finite ceilings on computational capacity.

Forbidden examples include:

MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_QPUS
MAX_QUBITS
MAX_NODES
MAX_WORKERS
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_DEVICES
MAX_TARGETS
MAX_PROVIDERS
MAX_PACKAGES
MAX_DEPENDENCIES
MAX_BUILD_TASKS
MAX_PROJECT_SIZE

Renaming these restrictions does not make them acceptable.

The prohibition applies to universal application-level limits imposed without a legitimate representation, protocol, security or operational reason.

5.2 Resource-parametric operation

The architecture must support workloads that grow according to:

- Program requirements.
- Explicit input dimensions.
- Available processing resources.
- Available memory and storage.
- Available hardware capabilities.
- Provider implementation capacity.
- Distributed infrastructure.
- Configured operational budgets.
- Security and authorization policies.

The architecture must work from tiny embedded systems to large distributed environments.

It must not require rewriting source semantics to accommodate a larger compatible machine.

5.3 Legitimate finite limits

The following remain legitimate when explicitly configured:

- Build execution deadlines.
- Download budgets.
- Archive extraction budgets.
- Memory budgets.
- Storage budgets.
- Network budgets.
- Maximum retained log output.
- Concurrent task budgets.
- Retry budgets.
- Provider request budgets.
- Cancellation deadlines.

These are operational policies.

They must not be silently converted into universal language or hardware limitations.

An absent budget means that the corresponding configuration does not impose a limit. It does not guarantee unlimited operating-system or physical resources.

5.4 Representation limits

Finite representation types are permitted when justified.

For example, identifiers, timestamps, version components and in-memory indexes have finite representational domains.

Such limits must be documented, validated and reported through explicit errors.

They must never be misrepresented as universal computational-capacity limits.

5.5 Memory and algorithmic scalability

Every toolchain component must document:

- Its memory-complexity characteristics.
- Its time-complexity characteristics where material.
- Whether it retains complete input collections.
- Whether streaming or incremental processing is supported.
- Whether operations can be cancelled.
- Whether it supports bounded-memory operation.
- Whether it creates copies of large artifacts.
- Whether its data structures are deterministic.

Large packages and build outputs should be processed incrementally or through streaming interfaces where practical.

The implementation must avoid unnecessary duplication of complete source trees, dependency graphs, package archives and artifacts.

No arbitrary fixed-size buffers may silently truncate valid input.

6. Rust and safety integration

Every production implementation under "src/toolchain/" MUST satisfy:

Requirement| Contract
Minimum Rust| 1.97
Edition| 2021
Unsafe Rust| Forbidden
Error handling| Structured "Result"
Public API| Documented
Concurrency| Explicit ownership and synchronization
Serialization| Validated and versioned
Compatibility| Explicitly evaluated
Resource handling| Configurable and observable

Use:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

Apply "#![deny(missing_docs)]" to public API modules where appropriate.

The crate root already establishes an unsafe-code prohibition. Child modules must preserve that guarantee.

No implementation may introduce unsafe Rust to bypass ownership, synchronization or API-design problems.

All public functions must document:

- Purpose.
- Arguments.
- Return values.
- Errors.
- Side effects.
- Resource behavior.
- Cancellation behavior.
- Security assumptions.
- Thread-safety guarantees.

Public APIs should use non-exhaustive enums where forward-compatible extension is required.

7. Root toolchain foundation integration

The existing foundation files must remain the canonical foundation.

7.1 "src/toolchain/errors.rs"

Sole owner of:

- "ToolchainError".
- "ToolchainErrorKind".
- Stable toolchain error classifications.
- Error codes.
- Error context.
- Causal error preservation.
- Canonical toolchain result aliases.

Other toolchain modules must consume this error contract.

It must preserve the original causal error when adapting errors from:

- Frontend.
- Compiler.
- Backend.
- Runtime.
- Project configuration.
- Filesystem operations.
- Package providers.
- Registry providers.

Do not translate every error into a generic build failure.

Cancellation, incompatibility, validation failure, resource exhaustion, authorization failure and provider failure must remain distinguishable.

"result.rs" must not introduce another error taxonomy.

7.2 "src/toolchain/result.rs"

Owns:

- Result-facing public re-exports.
- Contextual result enrichment.
- Result composition helpers.

It must reuse "errors.rs".

All toolchain services must use the same canonical result semantics.

"Ok" must mean the operation completed according to its documented contract.

Partial success must be represented explicitly rather than silently treated as total success.

7.3 "src/toolchain/ids.rs"

Owns toolchain-specific identities, including:

- Project identity.
- Workspace identity.
- Package identity.
- Source identity.
- Task identity.
- Build identity.
- Registry identity.
- Provider identity where applicable.
- Environment identity.
- Profile identity.
- Deployment identity.

The exact canonical identity types must remain consistent with the existing implementation.

Backend IDs remain owned by "backend::ids".

Compiler target IDs remain owned by compiler target configuration.

AST, semantic and domain IR identities remain owned by their respective subsystems.

Physical device and qubit IDs remain owned by their hardware and domain authorities.

Identity uniqueness must not be falsely advertised as globally guaranteed when allocation is only local to an allocator instance.

Distributed or persistent identity requires a suitable persistent allocation authority.

7.4 "src/toolchain/version.rs"

Owns toolchain-specific version contracts:

- Toolchain API versions.
- Manifest schema compatibility versions.
- Package archive format versions.
- Registry protocol versions.
- Provider API versions.
- Task protocol versions.

It must reuse the canonical version representation from "zuir::version" where appropriate.

It must not become the owner of:

- Zamani language versions.
- Grammar versions.
- AST versions.
- Semantic model versions.
- IR versions.
- Compiler release versions.
- Backend executable versions.
- Hardware firmware versions.
- Runtime versions.

A matching version number alone does not prove compatibility.

Every compatibility decision must identify the exact contracts being compared.

Unknown versions must fail closed unless an explicit compatibility rule establishes otherwise.

8. Public API integration

Authority:

"src/toolchain/api/"

Existing files:

api/
├── mod.rs
├── request.rs
├── response.rs
├── capabilities.rs
├── compatibility.rs
└── versioning.rs

8.1 "api/request.rs"

Defines external toolchain operation requests.

Requests must express caller intent without embedding compiler, hardware or runtime implementations.

They must preserve:

- Project and workspace references.
- Operation identity.
- Build intent.
- Target-selection intent.
- Profile references.
- Feature selections.
- Dependency intent.
- Operational budgets.
- Cancellation information.
- Security context references.
- API version information.

8.2 "api/response.rs"

Defines structured operation responses.

Responses must distinguish:

- Request accepted.
- Request rejected.
- Validation completed.
- Dependency resolution completed.
- Build completed.
- Artifact generated.
- Artifact verified.
- Package created.
- Package published.
- Deployment prepared.
- Runtime execution handed off.

A request submission must not be reported as a successful build.

8.3 "api/capabilities.rs"

Describes capabilities of the toolchain API itself.

It must not invent hardware capability evidence.

Hardware and backend capability information must come from the canonical backend, compiler and ZHAL authorities.

8.4 "api/compatibility.rs"

Coordinates compatibility decisions across independently versioned contracts.

It must not infer compatibility from matching version strings alone.

8.5 "api/versioning.rs"

Owns API-version negotiation and version-specific request handling.

It must protect consumers from accidentally interpreting an unsupported request version.

8.6 "api/mod.rs"

Composes the public API.

CLI, IDE, automation and authorized AI-agent clients must use the same underlying services.

No client may bypass validation, authorization, artifact verification or provider policies.

9. Configuration integration

Authority:

"src/toolchain/configuration/"

Actual existing structure:

configuration/
├── mod.rs
├── config.rs
├── budget.rs
├── policies.rs
├── profiles.rs
└── validation.rs

9.1 "configuration/config.rs"

Owns canonical toolchain configuration composition.

It must integrate:

- Manifest configuration.
- User configuration.
- Workspace configuration.
- Build configuration.
- Registry configuration.
- Security policies.
- Environment configuration.
- Profile configuration.

Configuration precedence must be explicitly documented.

Silent configuration overrides are prohibited.

9.2 "configuration/budget.rs"

Owns operational budget contracts and accounting.

Other modules must reuse its budget definitions and tracking mechanisms.

Do not create independent memory, time, concurrency or storage budget implementations in every subsystem.

Budget consumption must be observable.

Budget exhaustion must produce an explicit result.

9.3 "configuration/policies.rs"

Owns toolchain-level policy configuration.

It must integrate with the existing security policy authority rather than redefining source-language security semantics.

9.4 "configuration/profiles.rs"

Owns generic toolchain profile definitions and registration.

Specialized profiles must extend the generic profile system.

It must remain the only generic toolchain profile registry.

9.5 "configuration/validation.rs"

Validates complete configuration composition.

Validation must distinguish:

- Structurally valid configuration.
- Semantically valid configuration.
- Compatible configuration.
- Authorized configuration.
- Operationally feasible configuration.

These are different results.

9.6 "configuration/mod.rs"

Composes the configuration subsystem and exposes deliberate public APIs.

10. Grammar and language integration

The toolchain MUST consume the canonical language and grammar authorities.

It MUST NOT create another Zamani source language, build language or deployment language.

10.1 Normative grammar authorities

The relevant existing documents include:

grammar/
├── DESIGN.md
├── FREEZE.md
├── CONFORMANCE.md
├── MANIFEST.yaml
├── OWNERSHIP.yaml
├── DEPENDENCIES.yaml
├── STATUS.yaml
├── Zamani.g4
├── specification/
├── spec/
├── validation/
├── compile/
├── execution/
└── policies/

The toolchain must follow the authority and ownership contracts established there.

In particular:

- "DESIGN.md" governs grammar architecture.
- "specification/" governs normative source-language meaning.
- "spec/" provides machine-oriented language contracts.
- "OWNERSHIP.yaml" governs conceptual ownership.
- "DEPENDENCIES.yaml" governs grammar dependency relationships.
- "STATUS.yaml" records implementation and conformance status.
- "FREEZE.md" governs grammar freeze.
- "CONFORMANCE.md" governs grammar conformance.

The historical "Zamani-Grammar.md" must not override normative specifications.

10.2 Compilation profiles

Canonical source syntax:

"grammar/compile/profiles.g4"

This file owns compilation-profile syntax only.

It does not own:

- Profile registry implementation.
- Hardware profiles.
- Target discovery.
- Optimization algorithms.
- Physical resource allocation.
- Runtime configuration.
- Build execution.

The frontend must map the source syntax into the canonical AST and semantic model.

The toolchain consumes validated profile references through its configuration and build APIs.

10.3 Deployment syntax

Canonical deployment intent:

"grammar/execution/deployment.g4"

Policy/deployment composition:

"grammar/policies/deployment.g4"

Compilation-layer deployment integration:

"grammar/compile/deployment.g4"

These files must remain adapters and compositions around their canonical grammar authorities.

The toolchain must consume validated deployment intent.

It must not reinterpret deployment syntax or create a competing deployment language.

10.4 Semantic separation

The semantic authority remains:

"src/frontend/semantic/"

The following concepts must retain their original ownership:

- Names.
- Types.
- Effects.
- Requirements.
- Capabilities.
- Preferences.
- Constraints.
- Hints.
- Resource requirements.
- Provenance.
- Semantic compatibility.

The toolchain may transport these validated contracts into build requests.

It must not create another semantic analyzer or independently decide what source constructs mean.

10.5 No parser-time hardware decisions

The lexer and parser must not:

- Select a CPU.
- Select a GPU.
- Allocate a QPU.
- Select physical qubits.
- Select an FPGA.
- Assign cluster nodes.
- Perform resource scheduling.
- Perform deployment.
- Determine physical feasibility.

These decisions belong to downstream authorities.

11. Manifest and workspace integration

11.1 Canonical manifest authority

"src/zamani_project_config/" owns the Zamani project manifest.

The toolchain must not create another TOML parser.

Current implementation limitation:

"src/zamani_project_config/mod.rs" currently models basic package information and dependencies.

It must be expanded through its canonical owner before the toolchain assumes complete support for workspaces, profiles, targets, registries, testing, packaging and deployment configuration.

The required manifest evolution must be coordinated with the existing project configuration API.

Potential canonical schema components include:

- Package.
- Dependencies.
- Workspace.
- Features.
- Build.
- Targets.
- Profiles.
- External tools.
- Registries.
- Packaging.
- Testing.
- Deployment.

These sections must be added to the canonical manifest authority, not redefined inside "toolchain/manifest/".

11.2 "manifest/adapter.rs"

Adapts canonical manifest data into toolchain-facing contracts.

It must not:

- Parse TOML independently.
- Create duplicate package schemas.
- Resolve dependencies.
- Discover targets.
- Allocate hardware.
- Execute builds.

11.3 "manifest/loading.rs"

Coordinates manifest loading through the canonical project configuration parser.

It must preserve file and parsing errors.

11.4 "manifest/validation.rs"

Validates toolchain-facing manifest requirements.

It must use canonical schema validation and preserve structured diagnostics.

11.5 "manifest/compatibility.rs"

Handles supported manifest versions and explicit migrations.

Migration must never silently discard unknown configuration or alter semantic meaning.

11.6 "manifest/mod.rs"

Exposes the manifest adapter boundary.

11.7 Workspace

Authority:

"src/toolchain/workspace/"

The workspace subsystem owns project relationships and workspace discovery.

It must not duplicate manifest parsing or dependency resolution.

Workspace operations must use canonical ProjectId and WorkspaceId.

Relative paths must be interpreted against explicitly identified project or workspace roots, not assumed process working directories.

Filesystem traversal must defend against:

- Path traversal.
- Symlink escapes.
- Invalid path components.
- Unexpected file types.
- Concurrent filesystem changes.
- Unauthorized workspace access.

12. Dependency management integration

Authority:

"src/toolchain/dependencies/"

Existing files:

dependencies/
├── mod.rs
├── model.rs
├── constraints.rs
├── resolver.rs
├── graph.rs
├── lockfile.rs
├── sources.rs
├── features.rs
├── compatibility.rs
└── validation.rs

12.1 Dependency model

"model.rs" owns toolchain-facing dependency identities and declarations.

It must reuse canonical package and source identities.

12.2 Constraints

"constraints.rs" owns dependency constraint representation.

Version requirements must be validated without confusing version compatibility with semantic compatibility.

12.3 Resolver

"resolver.rs" owns deterministic dependency resolution.

It must support:

- Direct dependencies.
- Transitive dependencies.
- Version conflicts.
- Multiple dependency sources.
- Feature selection.
- Compatibility constraints.
- Resolution failures.
- Reproducible results.

Unknown package metadata must not be interpreted as compatibility evidence.

12.4 Dependency graph

"graph.rs" owns package dependency relationships.

It must remain distinct from:

- Compiler dependency graphs.
- ZUIR dependency graphs.
- Build task graphs.
- Incremental invalidation graphs.

12.5 Lockfile

"lockfile.rs" owns reproducible dependency-resolution snapshots.

A lockfile must record enough information to reproduce dependency selection, including source identity, selected package versions, content integrity and relevant resolution metadata.

Lockfile updates must be atomic.

Unverified lockfiles must not be treated as trusted.

12.6 Sources

"sources.rs" owns dependency-source abstractions.

It must support local and registered remote sources through extensible providers.

It must not hard-code a single registry or repository vendor.

12.7 Features

"features.rs" owns package-feature resolution and integration.

Feature activation must be deterministic and compatible with canonical manifest declarations.

12.8 Compatibility

"compatibility.rs" coordinates package, manifest, compiler, ABI, IR and provider compatibility evidence.

12.9 Validation

"validation.rs" validates resolved dependency state before downstream build operations.

12.10 Composition

"dependencies/mod.rs" exposes the dependency service boundary.

13. Package management integration

Authority:

"src/toolchain/packages/"

Existing files:

packages/
├── mod.rs
├── model.rs
├── archive.rs
├── creation.rs
├── extraction.rs
├── installation.rs
├── verification.rs
└── metadata.rs

13.1 Package identity

"model.rs" defines the toolchain-facing package model.

It must use canonical package identities.

Package identity must remain distinct from:

- Artifact identity.
- Project identity.
- Registry identity.
- Build identity.

13.2 Archive format

"archive.rs" owns the versioned package archive contract.

The archive format must define:

- Canonical metadata.
- Package contents.
- Dependency metadata.
- Artifact references.
- Content integrity.
- Format version.
- Compatibility metadata.
- Provenance references.
- Signature references where applicable.

Archive format versioning must remain independent of language, IR and compiler versions.

13.3 Creation

"creation.rs" creates packages from validated project and artifact data.

It must use explicit package inclusion policies.

It must not accidentally package:

- Credentials.
- Secrets.
- Temporary build files.
- Unapproved local files.
- Unverified artifacts.

13.4 Extraction

"extraction.rs" must safely extract untrusted package archives.

It must prevent:

- Path traversal.
- Absolute-path writes.
- Symlink escapes.
- Malicious archive entries.
- Duplicate-path ambiguity.
- Unbounded resource consumption.
- Partial installation being reported as success.

13.5 Installation

"installation.rs" coordinates verified package installation and dependency integration.

Installation must be transactional where practical.

A failed installation must not leave a misleading valid package state.

13.6 Verification

"verification.rs" coordinates integrity, signature, compatibility and provenance checks.

It must reuse the canonical cryptographic implementation.

13.7 Metadata

"metadata.rs" owns package-level metadata projection.

It must not create a competing semantic or artifact metadata system.

13.8 Composition

"packages/mod.rs" coordinates the package lifecycle.

14. Registry integration

Authority:

"src/toolchain/registries/"

Existing files:

registries/
├── mod.rs
├── provider.rs
├── registry.rs
├── discovery.rs
├── client.rs
├── protocol.rs
├── authentication.rs
├── publishing.rs
├── downloading.rs
├── federation.rs
└── validation.rs

The registry architecture must be open-ended.

It must support public, private, enterprise, institutional, self-hosted and federated registries.

It must not hard-code one registry URL, vendor, authentication system or network protocol.

14.1 Provider contract

"provider.rs" defines the registry-provider integration boundary.

It must integrate with canonical provider contracts from "stdlib" where applicable.

Providers must declare their capabilities and supported protocol versions.

14.2 Registry identity

"registry.rs" owns registry configuration and lifecycle descriptions.

14.3 Discovery

"discovery.rs" discovers configured and registered registries.

Discovery does not establish trust.

14.4 Client

"client.rs" coordinates registry requests.

It must use explicit timeouts, cancellation and configurable network budgets.

14.5 Protocol

"protocol.rs" owns the toolchain registry wire-contract definitions.

Protocol evolution must be versioned.

Unrecognized mandatory protocol semantics must fail closed.

14.6 Authentication

"authentication.rs" integrates authentication providers.

Credentials must never be stored in ordinary package metadata, build fingerprints, diagnostic output or logs.

14.7 Publishing

"publishing.rs" coordinates package publication.

Publication must distinguish:

- Upload started.
- Upload completed.
- Registry validation completed.
- Signature accepted.
- Publication committed.
- Publication failed.

A timeout after submission must not be blindly interpreted as proof that publication did not happen. Reconciliation must use an idempotent publication identity or registry status inquiry.

14.8 Downloading

"downloading.rs" coordinates downloads and content verification.

Downloads must not be trusted merely because an HTTP request succeeded.

14.9 Federation

"federation.rs" coordinates multiple registries.

It must define conflict handling, trust boundaries, source identity and package-origin verification.

14.10 Validation

"validation.rs" validates registry metadata and provider responses.

15. Target and external-tool integration

15.1 Targets

Authority:

"src/toolchain/targets/"

Existing files:

targets/
├── mod.rs
├── profiles.rs
├── matrix.rs
├── compatibility.rs
├── selection.rs
└── validation.rs

The toolchain does not own the canonical target registry.

Target identities remain with:

"compiler::configuration::target"

Target descriptions and backend implementations remain with:

"backend/"

Target discovery and feasibility remain with:

"compiler::planning/"

Physical hardware evidence remains with:

"zhal/"

Target selection must preserve:

- Explicit developer intent.
- Requirements.
- Capabilities.
- Preferences.
- Constraints.
- Hints.
- Feasibility evidence.

An empty preference list must not imply a universal native target.

A target profile is not a target descriptor.

A target descriptor is not proof of current availability.

A requested target is not proof of feasibility.

15.2 External tools

Authority:

"src/toolchain/external_tools/"

Existing files:

external_tools/
├── mod.rs
├── requirements.rs
├── resolution.rs
├── validation.rs
└── reporting.rs

This subsystem must reuse:

- "backend::toolchain"
- "backend::process"
- "backend::config"
- "backend::ids"
- "backend::reproducibility"

"backend::toolchain" owns executable discovery and version validation.

"backend::process" owns process execution.

"external_tools/" owns toolchain-facing orchestration and reporting.

It must not implement another process runner, executable discovery engine or shell-command execution system.

External processes must receive argument vectors, not interpolated shell commands.

Tool execution must be authorized, auditable and cancellable.

Discovering an executable does not establish that it is trusted.

A version match does not establish that a tool produces correct output.

Actual output validation belongs to the relevant compiler/backend artifact authority.

16. Task integration

Authority:

"src/toolchain/tasks/"

Existing files:

tasks/
├── mod.rs
├── model.rs
├── graph.rs
├── registry.rs
├── executor.rs
├── dependencies.rs
├── cancellation.rs
└── reporting.rs

16.1 Task model

"model.rs" owns task identity and lifecycle contracts.

A task must describe:

- Identity.
- Operation kind.
- Inputs.
- Dependencies.
- Expected outputs.
- State.
- Cancellation.
- Budget requirements.
- Execution result.
- Provenance references.

16.2 Task graph

"graph.rs" owns the toolchain task dependency graph.

It must not become another compiler IR or compiler dependency graph.

Graph ordering must be deterministic where the contract requires determinism.

16.3 Registry

"registry.rs" owns task-provider registration and resolution.

The registry must support extension without requiring edits to a closed list of domains or vendors.

16.4 Dependencies

"dependencies.rs" owns task prerequisite relationships.

16.5 Cancellation

"cancellation.rs" coordinates cancellation across tasks and build operations.

Cancellation must propagate through dependent operations where appropriate.

The task executor must distinguish:

- Cancelled.
- Completed.
- Failed.
- Blocked.
- Resource-exhausted.
- Partially completed.

16.6 Executor

"executor.rs" owns task execution orchestration.

It must reuse shared budget accounting.

It must not create another compiler scheduler, runtime scheduler or distributed cluster scheduler.

The executor must distinguish independent task execution from actual distributed computation.

16.7 Reporting

"reporting.rs" owns task-level structured results.

16.8 Composition

"tasks/mod.rs" exposes the task execution boundary.

17. Build integration

Authority:

"src/toolchain/build/"

Existing files:

build/
├── mod.rs
├── request.rs
├── plan.rs
├── graph.rs
├── executor.rs
├── profiles.rs
├── inputs.rs
├── outputs.rs
├── incremental.rs
└── reporting.rs

This subsystem is the central toolchain orchestration boundary.

17.1 Build request

"request.rs" owns the immutable build request contract.

It must preserve:

- Project identity.
- Workspace identity.
- Source identity.
- Dependency intent.
- Feature selection.
- Profile references.
- Target intent.
- Operational budgets.
- Cancellation.
- Build options.
- Reproducibility requirements.

17.2 Build plan

"plan.rs" owns build-plan construction and validation.

It must coordinate the canonical compiler planning API.

A build plan must retain feasibility evidence and its source.

Unresolved feasibility must remain unresolved.

17.3 Build graph

"graph.rs" owns build-task dependencies.

It must remain separate from package dependency and compiler IR graphs.

17.4 Build inputs

"inputs.rs" owns immutable logical build-input identity.

It must integrate:

- Source fingerprints.
- Manifest identity.
- Dependency lockfile identity.
- Build profile identity.
- Compiler contract version.
- Relevant configuration.
- Relevant provider/toolchain fingerprints.

Every input that can affect the output must be represented.

Machine-local incidental data must not contaminate portable identities unless it genuinely affects the realization.

17.5 Build profiles

"profiles.rs" integrates generic toolchain profiles with validated compiler configuration.

It must not redefine source-language compilation-profile semantics.

17.6 Build executor

"executor.rs" owns orchestration of an assessed build plan.

It must:

1. Validate the plan.
2. Verify target feasibility evidence.
3. Validate source and dependencies.
4. Construct compiler requests.
5. Apply configured operational budgets.
6. Invoke the public compiler API.
7. Preserve compiler diagnostics.
8. Validate compiler responses.
9. Verify required artifacts.
10. Produce structured results.

The executor must never fabricate successful compilation.

Important existing limitation:

The current executor documents that the compiler API uses a transitional legacy compilation driver.

Therefore, successful invocation of the current public compiler API is not sufficient evidence that the complete compiler pipeline is production-ready.

The compiler integration must be upgraded and independently verified.

17.7 Build outputs

"outputs.rs" owns toolchain-facing output references and collections.

Artifact payloads and artifact verification remain with their authoritative artifact subsystem.

The toolchain must not create a second artifact storage system.

17.8 Incremental build

"incremental.rs" integrates with:

- "compiler::incremental"
- "zuir::incremental"
- "backend::cache"
- Canonical fingerprinting.

It must not create another incremental compilation dependency engine.

A cache hit is not proof of correctness unless the complete input identity and artifact integrity have been verified.

Cache rejection must never turn a failed compilation into success.

17.9 Reporting

"reporting.rs" produces structured build reports.

It must distinguish target-specific outcomes within a multi-target build.

17.10 Build composition

"build/mod.rs" exposes the complete build service.

18. Compiler integration

The compiler remains the sole owner of compilation.

The toolchain must integrate through:

"src/compiler/api/"

and the canonical compiler service.

The toolchain may submit:

- Validated source-project information.
- Compilation configuration.
- Profile selections.
- Target intent.
- Dependency fingerprints.
- Reproducibility requirements.
- Operational budgets.
- Cancellation information.

The compiler owns:

- Parsing integration.
- Semantic compilation.
- IR construction.
- IR verification.
- Optimization.
- Compilation planning.
- Capability evaluation.
- Feasibility analysis.
- Domain-specific lowering.
- Backend coordination.
- Compilation diagnostics.

The toolchain must not invoke individual private compiler passes to simulate a public compilation service.

18.1 Compiler integration requirements

Before declaring this integration production-ready, establish:

1. One stable public compiler service.
2. Complete request and response contracts.
3. Structured compilation diagnostics.
4. Explicit target and feasibility information.
5. Explicit artifact references.
6. Explicit artifact-verification outcomes.
7. Cancellation behavior.
8. Operational budget behavior.
9. Deterministic input identity.
10. Compatibility with incremental compilation.
11. Actual end-to-end classical compilation.
12. Multi-target compilation evidence.

The compiler must not claim target compatibility without appropriate evidence.

19. Backend, artifact and reproducibility integration

19.1 Backend

The backend remains the authority for target-specific code generation.

The toolchain must use the compiler-facing backend integration and canonical backend registry.

It must not create a second backend registry.

19.2 Process execution

"backend/process.rs" remains the sole process-execution authority for backend external tools.

It must not be duplicated by:

- "toolchain/external_tools/"
- "toolchain/tasks/"
- "toolchain/build/"
- "toolchain/commands/"

19.3 Artifact authority

Use the existing compiler artifact subsystem and backend artifact contracts.

The toolchain may aggregate artifact references and attach build identity.

It must not redefine artifact payloads, serialization or content-verification algorithms.

19.4 Reproducibility

Integrate with:

- "backend/reproducibility.rs"
- "backend/cache.rs"
- "compiler/incremental/"
- "zuir/fingerprint/"

Build identity must include all relevant compilation inputs.

Identical fingerprints do not prove identical external compiler behavior unless the external compiler's determinism contract is established.

A reproducible build must identify the actual guarantee:

- Deterministic dependency resolution.
- Deterministic compilation inputs.
- Deterministic build planning.
- Deterministic artifact generation.
- Byte-for-byte reproducible artifacts.

These are separate guarantees.

20. Hardware and domain integration

20.1 Classical computing

Canonical authorities:

- "classical/"
- "compiler/"
- "backend/"
- "zhal/"
- "zur/"

The toolchain coordinates classical project builds and packaging.

It does not implement CPU selection, memory allocation or processor scheduling.

20.2 Quantum computing

Canonical authorities:

- "quantum/"
- "quantum::ir"
- Quantum compilation and planning.
- Quantum backend implementations.
- Quantum hardware integration.
- ZHAL.
- ZUR.

The toolchain must preserve canonical quantum requirements and artifacts.

It must not create a competing quantum IR, qubit identity system, quantum scheduler, routing engine or QEC subsystem.

It must support quantum simulator and physical QPU providers through their canonical interfaces.

Unknown quantum capability evidence must never be interpreted as unlimited qubit availability.

20.3 HDL and hardware

Canonical authorities:

- "hdl/"
- "hdl::hdlir"
- "hdl::realization"
- "backend/"
- "zhal/"

The toolchain coordinates synthesis, simulation, verification and hardware-artifact workflows through registered providers.

It does not implement placement, routing, synthesis or physical timing closure.

20.4 AI and tensor computing

Canonical authorities:

- "ai/"
- Existing tensor infrastructure.
- "zuir/dialects/ai/"
- Compiler planning.
- Backend providers.
- Runtime.

The toolchain coordinates AI project dependencies, model artifacts, datasets and deployment metadata.

It must not create another tensor storage system or AI semantic model.

20.5 Hybrid computing

The toolchain must coordinate multiple domain-specific artifacts under one logical project and build identity.

It must not force every domain into one artificial target.

Cross-domain interfaces must be validated by their canonical authorities.

20.6 Distributed computing

Canonical authorities:

- "distributed/"
- "zur/distributed/"
- Runtime.
- Networking.
- Backend and provider integrations.

The toolchain may prepare distributed build and deployment descriptions.

It must not implement a second distributed runtime, cluster scheduler, communication protocol or node allocator.

20.7 Future domains

Future computational domains must integrate through the same extension architecture.

Adding a domain must not require changing the fundamental project, dependency, package, build or API model merely to recognize its existence.

Domain-specific semantic and realization behavior remains owned by the appropriate domain subsystem.

21. Environment integration

Authority:

"src/toolchain/environments/"

Existing files:

environments/
├── mod.rs
├── discovery.rs
├── configuration.rs
├── fingerprints.rs
└── validation.rs

The environment subsystem must distinguish:

- Declared environment.
- Discovered environment.
- Observed environment.
- Validated environment.
- Authorized environment.
- Currently available environment.

Discovery is not proof of availability.

An environment fingerprint must include the relevant information necessary for the reproducibility contract.

Environment fingerprints must not include secrets.

22. Deployment integration

Authority:

"src/toolchain/deployment/"

Existing files:

deployment/
├── mod.rs
├── request.rs
├── profiles.rs
├── validation.rs
├── planning.rs
├── providers.rs
└── reporting.rs

22.1 Deployment request

"request.rs" owns the toolchain-facing deployment request.

It must consume validated deployment intent.

22.2 Deployment profiles

"profiles.rs" must reuse:

- "configuration::profiles::ProfileDefinition"
- "configuration::profiles::ProfileRegistry"
- "configuration::profiles::ResolvedProfile"
- "ids::ProfileId"
- "configuration::config::ToolchainConfig"
- "configuration::config::ConfigSectionKey"

There must be one generic profile registry.

Deployment-specific settings must be stored through the existing namespaced configuration-section architecture.

Do not create another deployment profile identity system or registry.

22.3 Deployment validation

"validation.rs" validates deployment requirements, compatibility, authorization evidence and prerequisites.

Structural validity is not deployment feasibility.

22.4 Deployment planning

"planning.rs" creates deployment plans from validated intent and provider evidence.

It must not allocate physical resources or perform deployment itself.

22.5 Deployment providers

"providers.rs" defines or adapts the deployment-provider integration boundary.

It must reuse canonical provider contracts where appropriate.

Provider registration must not imply provider trust.

22.6 Deployment reporting

"reporting.rs" records structured deployment outcomes.

It must distinguish:

- Planned.
- Validated.
- Authorized.
- Accepted by provider.
- Provisioning.
- Deployed.
- Failed.
- Cancelled.
- Indeterminate.

22.7 Runtime handoff

Actual program execution remains with ZUR and the appropriate runtime authorities.

A deployment-ready artifact is not automatically a running application.

23. Security integration

Authority:

"src/toolchain/security/"

Existing files:

security/
├── mod.rs
├── policy.rs
├── trust.rs
├── signatures.rs
├── integrity.rs
├── provenance.rs
└── validation.rs

The toolchain must reuse the repository's canonical cryptographic and security implementations.

It must not create another cryptographic library or competing trust authority.

23.1 Supply-chain security

The toolchain must support:

- Dependency integrity.
- Package integrity.
- Artifact integrity.
- Package signatures.
- Artifact signatures where required.
- Provider trust.
- Source provenance.
- Build provenance.
- Deployment provenance.
- Credential protection.
- Authorization.
- Auditability.

23.2 Trust boundaries

The following are untrusted until validated:

- Registry responses.
- Downloaded packages.
- External compiler output.
- External executable metadata.
- Provider responses.
- Environment metadata.
- Agent-supplied requests.
- Serialized project metadata.

23.3 Authorization

Authentication and authorization must remain separate.

An authenticated caller is not automatically authorized to perform every operation.

AI-agent requests must use the same authorization and security policies as human requests.

23.4 Filesystem security

Package extraction, installation, workspace discovery, cache storage and cleanup must protect against path traversal, symlink attacks, races and unauthorized filesystem access.

23.5 Credential security

Credentials must not enter:

- Package archives.
- Lockfiles.
- Build fingerprints.
- Artifact metadata.
- Diagnostic output.
- Build logs.
- Agent context unless explicitly authorized and strictly necessary.

24. Diagnostics integration

Authority:

"src/toolchain/diagnostics/"

Existing files:

diagnostics/
├── mod.rs
├── diagnostic.rs
├── aggregation.rs
├── ordering.rs
└── rendering.rs

The diagnostics subsystem owns toolchain diagnostic composition.

It must preserve source locations and original diagnostic classifications from the frontend and compiler.

It must support:

- Human-readable diagnostics.
- Machine-readable diagnostics.
- JSON-compatible structured results.
- Stable diagnostic codes.
- Severity.
- Source locations.
- Related diagnostics.
- Causal errors.
- Provider attribution.
- Target attribution.
- Build and task correlation.

Diagnostic ordering must be deterministic where the same inputs produce the same diagnostic set.

Rendering must not alter diagnostic meaning.

25. Human and AI agent integration

Authority:

"src/toolchain/agents/"

Existing files:

agents/
├── mod.rs
├── context.rs
├── permissions.rs
├── requests.rs
├── results.rs
├── audit.rs
└── compliance.rs

Human developers and authorized AI agents must use the same toolchain service APIs.

Agent-specific interfaces may provide contextual assistance, but they must not introduce different compilation or execution semantics.

The agent subsystem must:

1. Validate incoming requests.
2. Authenticate through the appropriate authority.
3. Obtain authorization.
4. Bind authorization to the requested operation.
5. Apply operational policies.
6. Execute through canonical toolchain services.
7. Preserve request correlation.
8. Produce structured results.
9. Record audit information.
10. Validate completion and compliance evidence.

The existing "agents/compliance.rs" must be retained.

It must not replace authentication, authorization, trust evaluation or audit persistence.

An agent's claim that a build succeeded is not evidence of successful compilation.

An agent's suggested action must not automatically authorize or execute that action.

26. Command-line integration

Authority:

"src/toolchain/commands/"

Existing files:

commands/
├── mod.rs
├── build.rs
├── check.rs
├── clean.rs
├── run.rs
├── test.rs
├── bench.rs
├── format.rs
├── lint.rs
├── doc.rs
├── package.rs
├── install.rs
├── publish.rs
├── target.rs
├── tool.rs
├── doctor.rs
└── deploy.rs

Each command must be a thin adapter over the canonical toolchain services.

Commands must not independently implement:

- Manifest parsing.
- Dependency resolution.
- Compilation.
- Package creation.
- Package extraction.
- Registry operations.
- Target discovery.
- External process execution.
- Runtime execution.
- Deployment.

Each command must provide:

- Validated input.
- Structured result.
- Appropriate exit status.
- Diagnostics.
- Cancellation.
- Authorization.
- Audit integration where applicable.

26.1 Existing package-manager CLI

"src/bin/zamani_pkg_main.rs" must be reconciled with the current public toolchain API.

The current CLI contains simulated-build behavior and legacy references to:

- "toolchain::build::compile_project"
- "toolchain::package_manager::PackageManager"

These interfaces must not be assumed to exist merely because the binary imports them.

The CLI must be migrated to the actual build and package services.

It must not fabricate successful compilation.

A command must not become available merely because its name has been registered.

It must become available only when its implementation and required dependencies are ready.

26.2 Command availability

The CLI must distinguish:

- Implemented.
- Available.
- Configured.
- Authorized.
- Supported for the requested operation.
- Operationally ready.

These are different conditions.

27. Testing integration

The toolchain needs a dedicated testing architecture.

The earlier proposed "src/toolchain/testing/" directory is not present in the current repository.

It should be added only as an orchestration and conformance subsystem if the existing test infrastructure does not already own the required functionality.

Recommended eventual structure:

src/toolchain/testing/
├── mod.rs
├── fixtures.rs
├── conformance.rs
├── integration.rs
├── determinism.rs
├── reproducibility.rs
├── portability.rs
├── scalability.rs
├── security.rs
└── regression.rs

These files must coordinate testing without duplicating the Rust test framework or compiler testing infrastructure.

Tests must distinguish mocks from real provider execution.

Mock-provider success is not evidence of real hardware compatibility.

27.1 Required project fixtures

At minimum, integration coverage must include:

- Minimal classical project.
- Multi-module classical project.
- Classical library.
- Parallel application.
- Multi-target application.
- Quantum project.
- HDL project.
- AI and tensor project.
- Hybrid project.
- Distributed project.
- Project using external tools.
- Project with incompatible dependencies.
- Project with unavailable target capabilities.
- Project with exhausted operational budgets.
- Project with malicious package input.

27.2 End-to-end requirements

A complete test must exercise:

Project
  |
  v
Manifest
  |
  v
Workspace
  |
  v
Dependencies
  |
  v
Build request
  |
  v
Build plan
  |
  v
Compiler
  |
  v
Backend
  |
  v
Artifact verification
  |
  v
Package
  |
  v
Runtime handoff

The test must validate the actual operation, not merely the existence of module declarations.

28. Cross-subsystem interface contracts

The following matrix is mandatory.

Toolchain component| Upstream authority| Downstream consumer
"ids.rs"| Toolchain identity contract| All toolchain modules
"version.rs"| "zuir::version"| API, manifests, packages and registries
"errors.rs"| Standard error contracts| All toolchain services
"result.rs"| "errors.rs"| All toolchain services
"manifest/"| "zamani_project_config"| Workspace, dependencies and build
"workspace/"| Filesystem and manifest contracts| Build, dependencies and commands
"dependencies/"| Manifest and registry contracts| Build and packages
"packages/"| Artifact and security contracts| Registries and installation
"registries/"| Provider and security contracts| Dependencies and packages
"targets/"| Compiler target and backend authorities| Build planning
"external_tools/"| Backend toolchain and process| Build and task execution
"tasks/"| Shared task and budget contracts| Build and commands
"build/"| Compiler public API| Commands, packages and reporting
"environments/"| ZHAL, ZUR and backend observations| Build and deployment
"deployment/"| Validated semantic intent and runtime contracts| Deployment providers
"security/"| Canonical security and cryptography| All sensitive operations
"diagnostics/"| Frontend, compiler and toolchain errors| API, CLI and agents
"agents/"| API, authorization and audit| Human/agent workflows
"commands/"| Public toolchain services| CLI applications

An integration must not introduce reverse ownership dependencies.

29. Cross-layer state and success semantics

The toolchain must preserve distinct operation states.

The following must never be conflated:

Request received
      |
      v
Request validated
      |
      v
Authorized
      |
      v
Dependencies resolved
      |
      v
Target assessed
      |
      v
Build planned
      |
      v
Compilation completed
      |
      v
Artifact generated
      |
      v
Artifact verified
      |
      v
Package created
      |
      v
Deployment validated
      |
      v
Resources acquired
      |
      v
Program launched
      |
      v
Program completed

Each stage must have a structured outcome.

A failure at one stage must not be converted into success at another.

For operations with multiple targets or packages, partial success must be represented explicitly.

Indeterminate external outcomes must remain indeterminate until reconciled.

30. Determinism and reproducibility

The toolchain must provide deterministic behavior wherever the contract requires it.

Required areas include:

- Dependency resolution.
- Lockfile generation.
- Feature resolution.
- Workspace metadata.
- Build-plan construction.
- Task dependency ordering.
- Configuration fingerprints.
- Artifact identity.
- Package metadata.
- Registry metadata normalization.
- Diagnostic ordering.
- Build reporting.

Unordered collections must be canonicalized before fingerprinting or serialization when ordering is not semantically meaningful.

Ordered collections must preserve their defined order.

Timestamps, random values, machine-local paths, credentials and transient network information must not enter reproducibility identities unless their inclusion is explicitly required by the contract.

A build fingerprint is not a digital signature.

A hash is not proof of semantic correctness.

A cache hit is not proof that a target is currently available.

31. Versioning and compatibility

Every externally consumed contract must have an explicit versioning policy.

The following version authorities must remain independent:

Contract| Owner
Zamani language| Grammar specification
Grammar| Grammar authority
AST| Frontend
Semantic model| Frontend semantic authority
ZUIR| ZUIR
Domain IR| Domain subsystem
Compiler API| Compiler
Backend API| Backend
Toolchain API| Toolchain
Manifest schema| Project configuration
Package archive format| Toolchain packages
Registry protocol| Toolchain registries
Provider API| Canonical provider authority
Hardware firmware| Hardware provider
Runtime API| ZUR

Breaking changes require an explicit compatibility or migration policy.

Unknown mandatory features must not be silently ignored.

Old package formats must be handled through explicit compatibility rules.

Migration must preserve package content, dependency meaning and source semantics.

32. Concurrency and distributed operation

Concurrency must be introduced according to explicit ownership and synchronization contracts.

The architecture must support:

- Sequential execution.
- Concurrent independent tasks.
- Parallel builds.
- Incremental builds.
- Distributed build providers.
- Remote artifact retrieval.
- Federated registries.
- Cancellation propagation.

No arbitrary worker count must be embedded as a universal architecture ceiling.

A configured concurrency budget may restrict the actual number of simultaneously executing tasks.

Parallel execution must preserve dependency ordering.

Concurrent operations must not create duplicate or conflicting package publications.

Distributed operations must preserve:

- Stable request identities.
- Correlation IDs.
- Idempotency contracts.
- Cancellation state.
- Provenance.
- Structured failure information.
- Authentication and authorization.
- Consistent artifact identity.

The toolchain must not create a second distributed execution scheduler.

33. Transactionality and failure recovery

Filesystem, registry and package operations must define their transaction boundaries.

The following operations require explicit failure semantics:

- Package extraction.
- Package installation.
- Lockfile updates.
- Cache insertion.
- Artifact publication.
- Registry publishing.
- Configuration updates.
- Deployment submission.

Where atomic operations are unavailable, use explicit staging and commit states.

A partially completed operation must not be reported as completely successful.

Retries must be based on operation semantics and idempotency.

A failed network response does not necessarily prove that a remote operation did not commit.

Recovery must reconcile actual state before retrying non-idempotent operations.

34. Root module and Cargo integration

The current "src/lib.rs" declares the canonical top-level repository modules but does not expose "toolchain".

The "Cargo.toml" file already contains:

[features]
default = []
lsp = []
buildsystem = []
full = []
omniversal = []

The "zamani-pkg" binary requires "buildsystem".

The integration must reconcile these existing contracts.

Recommended module boundary:

#[cfg(feature = "buildsystem")]
pub mod toolchain;

This is the recommended feature-gating decision, subject to confirming that all consumers and the existing library feature policy are compatible.

The toolchain root must expose its modules through deliberate public exports.

It must not use indiscriminate wildcard re-exports.

The "buildsystem" feature must become an actual documented capability boundary rather than an empty feature declaration.

Dependencies must only be added after identifying their precise ownership and integration requirements.

Do not add duplicate process, archive, cryptographic, serialization or networking libraries merely because another subsystem has not yet exposed the required interface.

The actual dependency versions and feature combinations must be tested against Rust 1.97.

35. File ownership and freeze contract

Every toolchain file must have a complete implementation contract before implementation begins.

Each file must document:

1. Canonical path.
2. Purpose.
3. Ownership.
4. Explicit non-ownership.
5. Public types.
6. Public functions.
7. Public traits.
8. Dependencies.
9. Exact upstream integration contracts.
10. Exact downstream consumers.
11. Error behavior.
12. Validation behavior.
13. Serialization behavior, where applicable.
14. Concurrency behavior.
15. Cancellation behavior.
16. Resource-budget behavior.
17. Security requirements.
18. Scalability requirements.
19. Unit tests.
20. Integration tests.
21. Freeze criteria.

A file is frozen only when its contract and dependencies are complete.

35.1 Dependency closure

A file must not be declared complete while it depends on an undefined downstream contract.

Its upstream interfaces must be frozen first.

The complete dependency graph must be acyclic, except for deliberately designed runtime interaction protocols that do not create compile-time module cycles.

35.2 Freeze rule

Once a file is frozen:

- Adding a new consumer must not require changing its semantic contract.
- Adding a new computing domain must not require changing unrelated foundational contracts.
- Adding a new hardware provider must not require changing generic project or package identities.
- Increasing machine size must not require changing the file.
- Adding a new registry must not require rewriting dependency resolution.
- Adding a new target must not require modifying the public build-request semantics.

If an actual defect is discovered, correct the owning contract and document the compatibility impact.

Do not hide defects through undocumented adapters in unrelated modules.

36. Required implementation order

The implementation sequence must follow dependency closure.

Phase 0 — Contract freeze

Complete and reconcile:

- "documentation/ARCHITECTURE.md"
- "documentation/INTEGRATION_CONTRACT.md"
- "documentation/BUILD_CONTRACT.md"
- "documentation/PACKAGE_CONTRACT.md"
- "documentation/PORTABILITY_CONTRACT.md"
- "documentation/SECURITY_CONTRACT.md"
- "documentation/AGENT_CONTRACT.md"
- "documentation/READINESS_MATRIX.md"

The existing ARCHITECTURE.md must be reconciled with this contract.

Contradictory or duplicated ownership declarations must be corrected before implementation.

Phase 1 — Independent foundations

The existing files must first be audited and frozen:

1. "errors.rs"
2. "ids.rs"
3. "version.rs"
4. "result.rs"
5. "configuration/budget.rs"
6. "configuration/policies.rs"
7. "configuration/profiles.rs"
8. "api/versioning.rs"
9. "api/compatibility.rs"
10. "api/capabilities.rs"

These files must establish complete reusable contracts.

Phase 2 — Public API and configuration

Complete:

- "api/request.rs"
- "api/response.rs"
- "api/mod.rs"
- "configuration/config.rs"
- "configuration/validation.rs"

Phase 3 — Project and manifest infrastructure

Complete:

- Canonical "zamani_project_config/" evolution.
- "manifest/loading.rs"
- "manifest/adapter.rs"
- "manifest/validation.rs"
- "manifest/compatibility.rs"
- "manifest/mod.rs"
- Workspace subsystem.

Phase 4 — Dependency and package infrastructure

Complete:

- Dependency models.
- Constraints.
- Sources.
- Graph.
- Feature resolution.
- Resolver.
- Lockfile.
- Compatibility.
- Package model.
- Archive format.
- Package creation.
- Extraction.
- Verification.
- Installation.

Phase 5 — Security and registry infrastructure

Complete:

- Security contracts.
- Trust integration.
- Integrity.
- Signatures.
- Provenance.
- Registry providers.
- Registry protocol.
- Authentication.
- Downloading.
- Publishing.
- Federation.

Publishing must follow package-format and security verification.

Phase 6 — Target and external-tool integration

Complete:

- Target profiles.
- Target compatibility.
- Target selection.
- Target matrices.
- External tool requirements.
- External tool resolution.
- External tool validation.

Reuse the canonical backend and compiler authorities.

Phase 7 — Task and build orchestration

Complete:

- Task model.
- Task graph.
- Task registry.
- Task dependencies.
- Cancellation.
- Task executor.
- Build request.
- Build plan.
- Build graph.
- Build inputs.
- Build outputs.
- Incremental integration.
- Build executor.
- Reporting.

At the end of this phase, an actual classical project must compile through the real compiler and backend.

Phase 8 — Domain integration

Extend the existing build system to:

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

Do not create parallel domain-specific toolchain architectures.

Phase 9 — Commands and deployment

Complete the developer commands and deployment orchestration.

Integrate the runtime through ZUR.

Phase 10 — Agent integration and production verification

Complete agent authorization, compliance, audit and reporting.

Run full end-to-end testing.

37. Production-readiness gates

A module is not production-ready merely because it compiles.

Each integration must pass the applicable gates.

Gate| Required evidence
Ownership| One canonical owner per concept
API| Complete public contracts
Dependency| Valid dependency direction
Errors| Structured and preserved failures
Validation| Invalid input rejected
Security| Trust boundaries enforced
Scalability| No arbitrary universal ceilings
Determinism| Stable behavior where required
Compatibility| Versioned and tested
Concurrency| Correct synchronization
Cancellation| Defined cancellation semantics
Integration| Actual consumer/provider tests
Reproducibility| Verified identity and output behavior
Documentation| Complete ownership and integration documentation

37.1 Repository build checks

The repository must pass:

cargo fmt --all -- --check

cargo check --all-targets

cargo test --all-targets

cargo clippy --all-targets -- -D warnings

Feature-specific verification:

cargo check --all-targets --features buildsystem

cargo test --all-targets --features buildsystem

Package-manager executable:

cargo build --release \
    --features buildsystem \
    --bin zamani-pkg

These checks must be run on the actual supported Rust baseline and on the current compatible stable toolchain.

Passing Cargo checks does not establish complete toolchain functionality.

37.2 Production integration tests

The following must be independently verified:

- Actual project discovery.
- Actual manifest parsing.
- Actual dependency resolution.
- Lockfile reproducibility.
- Actual classical compilation.
- Real backend invocation.
- Artifact verification.
- Package archive creation.
- Safe package extraction.
- Package installation.
- Registry download.
- Registry publication.
- Authentication and authorization.
- Multi-target build orchestration.
- Compiler cancellation.
- Build budget enforcement.
- Incremental invalidation.
- Build reproducibility.
- Deployment-provider integration.
- Runtime handoff.
- Human/agent authorization parity.

37.3 Scalability testing

Test increasingly large workloads without encoding arbitrary application-level ceilings.

Measure:

- Time complexity.
- Memory consumption.
- Storage consumption.
- Dependency graph behavior.
- Task graph behavior.
- Incremental invalidation.
- Package processing.
- Multi-target orchestration.
- Cancellation latency.
- Failure recovery.
- Concurrent execution.

The test suite must distinguish:

- Application-defined limits.
- Configured operational budgets.
- Operating-system limits.
- Representation limits.
- Provider implementation limitations.
- Actual hardware limitations.

38. Explicitly prohibited architectural duplication

The following duplication is prohibited.

Do not create in toolchain| Existing authority
Another lexer| "frontend/"
Another parser| "frontend/"
Another AST| "frontend/ast/"
Another semantic analyzer| "frontend/semantic/"
Another ZUIR| "zuir/"
Another quantum IR| "quantum::ir"
Another HDLIR| "hdl/hdlir"
Another tensor storage implementation| Canonical tensor infrastructure
Another compiler| "compiler/"
Another optimizer| Existing compiler/optimizer authorities
Another backend registry| "backend/registry"
Another process runner| "backend/process.rs"
Another external tool discovery engine| "backend/toolchain.rs"
Another artifact store| Existing backend/compiler artifact authorities
Another hardware abstraction layer| "zhal/"
Another runtime| "zur/" and "runtime/"
Another distributed scheduler| Existing distributed/runtime infrastructure
Another cryptographic implementation| Existing cryptographic authorities
Another provider identity system| Canonical provider contracts
Another generic profile registry| "configuration/profiles.rs"
Another manifest parser| "zamani_project_config/"

The toolchain owns coordination, project lifecycle, dependency resolution, package management and developer workflow orchestration.

39. Final integration acceptance criteria

The complete toolchain must eventually satisfy the following conditions.

Project infrastructure

- One canonical manifest authority.
- Complete workspace support.
- Deterministic dependency resolution.
- Versioned lockfiles.
- Reproducible project identity.

Build infrastructure

- One public compiler integration.
- Immutable build requests.
- Validated build plans.
- Dependency-aware build graphs.
- Configurable execution budgets.
- Cancellation.
- Incremental compilation.
- Multi-target orchestration.
- Verified artifacts.

Package ecosystem

- Versioned package format.
- Safe package extraction.
- Package integrity.
- Package signatures.
- Installation.
- Publishing.
- Multiple registries.
- Federated sources.

Computing portability

- Classical integration.
- Quantum integration.
- HDL integration.
- AI and tensor integration.
- Hybrid integration.
- Distributed integration.
- Future-provider integration.

Developer experience

- Unified CLI.
- Public API.
- Structured diagnostics.
- Testing.
- Formatting.
- Linting.
- Documentation.
- Deployment preparation.
- Agent integration.

Security and reliability

- Authentication.
- Authorization.
- Trust verification.
- Integrity.
- Provenance.
- Secret protection.
- Transactional operations.
- Cancellation.
- Failure recovery.
- Reproducibility.

40. Final architectural decisions

The following decisions are binding for the implementation unless changed through an explicitly documented architectural revision.

Question| Decision
Does toolchain own compilation?| No
Does toolchain own project lifecycle?| Yes
Does toolchain own dependency resolution?| Yes
Does toolchain own package management?| Yes
Does toolchain own build orchestration?| Yes
Does toolchain own backend implementation?| No
Does toolchain own external process execution?| No
Does toolchain own hardware discovery?| No
Does toolchain own physical resource allocation?| No
Does toolchain own runtime scheduling?| No
Does toolchain own physical execution?| No
Does toolchain own deployment orchestration?| Yes
Does toolchain own infrastructure provisioning?| No
Does toolchain support multiple domains?| Yes
Does toolchain support extensible providers?| Yes
Does toolchain support human and AI clients?| Yes
Does toolchain impose arbitrary hardware ceilings?| No
Does toolchain guarantee infinite physical capacity?| No
Does toolchain treat unknown feasibility as success?| No
Does toolchain reuse canonical repository authorities?| Yes

41. Definition of completion

This document is complete as an architectural contract only when:

1. Its ownership boundaries agree with the canonical repository authorities.
2. Existing implementation files conform to their assigned contracts.
3. Missing contracts are implemented.
4. Public interfaces are stable.
5. Dependency directions are validated.
6. Integration tests establish real operation.
7. Security requirements are demonstrated.
8. Scalability requirements are demonstrated.
9. No duplicate subsystem authority has been introduced.
10. The readiness matrix records evidence for every completed component.

The existence of this document does not itself make the toolchain production-ready.

The toolchain is production-ready only when the implementation satisfies these requirements and the complete release evidence has been independently verified.

---

42. Governing architectural principle

The Zamani toolchain must be the unified development, building, packaging and distribution infrastructure of the Zamani programming language.

It must not become another compiler, hardware abstraction layer or runtime.

The language defines what a program means.

The semantic authority validates that meaning.

The canonical IRs represent the computation.

The compiler determines valid compilation and realization strategies.

The backend generates target-specific artifacts.

ZHAL integrates physical hardware.

ZUR provides runtime and execution infrastructure.

The toolchain coordinates the entire development and distribution lifecycle around these authorities.

One language. One project ecosystem. One coherent toolchain. Canonical domain authorities. Open-ended computing capabilities. Resource-aware realization. Preserved semantics.

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.