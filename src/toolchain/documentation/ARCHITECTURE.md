Zamani Toolchain Architecture

Canonical path: "src/toolchain/documentation/ARCHITECTURE.md"
Repository: "github.com/Benwellonedge28/Zamani"
Language: Zamani
Rust edition: 2021
Minimum Rust version: Rust 1.97 or later
Safety: Safe Rust exclusively. "unsafe" is prohibited.
Architecture objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Document classification: Normative toolchain architecture and repository integration contract
Implementation status: Architecture specification; implementation conformance must be independently demonstrated.

---

1. Purpose

This document defines the authoritative architecture, responsibilities, integration boundaries, scalability requirements, security requirements, and production-readiness obligations of:

"src/toolchain/"

The Zamani toolchain is the unified developer-facing infrastructure for creating, configuring, validating, building, testing, packaging, distributing, running, and preparing deployment of Zamani projects.

It must support the entire Zamani computing ecosystem without implementing competing versions of existing repository subsystems.

The toolchain must coordinate:

- Classical computing.
- Systems programming.
- Embedded computing.
- Parallel and concurrent computing.
- High-performance computing.
- Scientific and numerical computing.
- Quantum computing.
- Quantum simulation.
- Quantum-classical hybrid computing.
- Hardware description and hardware/software co-design.
- FPGA and ASIC development.
- AI and machine learning.
- Deep learning and neural networks.
- Tensor and accelerator computing.
- Symbolic, probabilistic and causal computation.
- Distributed computing.
- Networking and communication.
- Cryptography and secure computation.
- Neuromorphic, photonic and other future computing technologies.

These are domains of one Zamani language, not independent programming languages requiring separate project-management and package-management systems.

The toolchain must support the same development architecture for an individual developer, an enterprise, a hardware manufacturer, an academic institution, an automated system, and an authorized AI agent.

1.1 Primary objective

The fundamental objective is:

Program Once, Compile Once, Run Everywhere, Anywhere, Forever.

POCO-REAF requires:

1. One canonical source-language meaning.
2. One consistent project and dependency model.
3. One coordinated logical build request.
4. Reusable compilation and build specifications.
5. Multiple compatible target-specific realizations.
6. Verified artifacts.
7. Explicit compatibility and feasibility evidence.
8. Reproducible build identities.
9. Compatibility-aware evolution.
10. Runtime handoff through authoritative execution infrastructure.

One source program may produce multiple artifacts because different machines and computational technologies require different realizations.

This does not mean that every program can run on every machine or that all targets must generate identical machine code.

It means that target changes must not require developers to redesign source semantics unnecessarily.

1.2 Meaning of infinity

The expression "scale from atom to everywhere" means:

The architecture must not impose arbitrary universal finite ceilings on computational concepts, hardware populations, project size, or supported computing domains.

It does not mean literal infinite physical resources.

Every implementation remains subject to:

- Available memory and storage.
- Processing capacity.
- Address-space and representation limits.
- Operating-system constraints.
- Provider capabilities.
- Hardware availability.
- Physical laws.
- Network and communication capacity.
- Security and authorization policies.
- Explicit operational budgets.

The toolchain must grow according to available resources and implementation capabilities.

It must not confuse a current implementation limitation with a universal limitation of the Zamani language.

---

2. Normative language and governing principles

The terms MUST, MUST NOT, REQUIRED, SHOULD, SHOULD NOT, and MAY are normative.

- MUST: Mandatory.
- MUST NOT: Prohibited.
- SHOULD: Strong recommendation requiring documented justification when not followed.
- SHOULD NOT: Generally prohibited unless justified.
- MAY: Optional.

The following principles govern every file in the toolchain.

2.1 One authoritative owner per concept

Every canonical responsibility must have one authoritative implementation.

The toolchain may coordinate an authority but must not duplicate it.

For example:

Concept| Canonical owner
Language syntax| "grammar/"
Lexical processing| "frontend/" and lexical contracts
AST| "frontend/ast/"
Source semantics| "frontend/semantic/"
Classical computation| "classical/"
Quantum semantics and IR| "quantum/ir/"
Hardware description| "hdl/"
Universal IR| "zuir/"
Compilation| "compiler/"
Optimization| "compiler/" and its authoritative optimization infrastructure
Target code generation| "backend/"
External process execution| "backend/process.rs"
External tool discovery| "backend/toolchain.rs"
Backend artifacts| "backend/" and "compiler/artifacts/"
Physical hardware integration| "zhal/"
Runtime execution| "zur/"
Distributed computation| "distributed/" and its runtime integration
Canonical project manifest| "zamani_project_config/"
Project/workspace orchestration| "toolchain/"
Package dependency resolution| "toolchain/dependencies/"
Package lifecycle| "toolchain/packages/"
Registry coordination| "toolchain/registries/"
Build orchestration| "toolchain/build/"
Developer commands| "toolchain/commands/"
Deployment coordination| "toolchain/deployment/"

A new implementation must first determine whether the required responsibility already exists.

2.2 No competing authorities

The toolchain MUST NOT introduce:

- A second compiler.
- A second frontend.
- A second AST.
- A second semantic model.
- A second universal IR.
- A second quantum IR.
- A second HDL IR.
- A second backend registry.
- A second hardware abstraction layer.
- A second physical resource allocator.
- A second runtime.
- A second external process runner.
- A second cryptographic implementation.
- A second project manifest parser.
- A second compiler artifact store.
- A second compiler incremental-compilation engine.
- A second compiler budget-accounting implementation.

Existing implementations must be expanded through their owning modules.

2.3 Open-world extensibility

New providers, domains, targets, registries, package sources, external tools, deployment environments, and commands must be introduced through stable extension contracts.

The architecture must not depend on a permanent list of hardware manufacturers or computational technologies.

A new compatible provider should not require changing the toolchain's fundamental orchestration architecture.

---

3. Repository authority and specification hierarchy

The toolchain must conform to the existing language and compiler specifications.

The relevant authority hierarchy is:

Zamani language architecture
        |
        v
grammar/DESIGN.md
        |
        +-------------------------------+
        |                               |
        v                               v
grammar/specification/              grammar/spec/
        |                               |
        +---------------+---------------+
                        |
                        v
               Canonical grammar
                        |
                        v
                Native frontend
                        |
                        v
             Canonical AST and semantics
                        |
                        v
              Canonical IR authorities
                        |
                        v
                Compiler services
                        |
                        v
              Backend and artifacts
                        |
                        v
              Toolchain orchestration
                        |
                        v
                Runtime handoff

The toolchain is a consumer of validated project and compilation contracts.

It is not an authority for changing the meaning of Zamani source programs.

3.1 Required grammar specifications

The architecture must remain consistent with:

File| Authority
"grammar/DESIGN.md"| Grammar architecture, ownership and dependency rules
"grammar/OWNERSHIP.yaml"| Canonical grammar ownership
"grammar/DEPENDENCIES.yaml"| Grammar dependency direction
"grammar/MANIFEST.yaml"| Grammar inventory and conformance gates
"grammar/Zamani.g4"| Root grammar composition
"grammar/antlr/ZamaniLexer.g4"| Public ANTLR lexer boundary
"grammar/antlr/ZamaniParser.g4"| Public ANTLR parser composition
"grammar/specification/semantic-model.md"| Source-language meaning
"grammar/specification/compilation-model.md"| Compilation stages
"grammar/specification/execution-model.md"| Execution semantics
"grammar/specification/portability.md"| Semantic portability
"grammar/specification/poco-reaf.md"| POCO-REAF
"grammar/specification/scalability-model.md"| Resource-parametric scalability
"grammar/specification/compatibility.md"| Compatibility governance
"grammar/specification/extensibility.md"| Extensibility and dialect governance

The toolchain must not interpret the existence of a grammar production as proof that its corresponding compiler or runtime implementation exists.

3.2 Grammar integration

The toolchain consumes project and source files through the canonical frontend and compiler APIs.

The intended direction is:

Zamani project
      |
      v
Toolchain project/workspace services
      |
      v
Canonical frontend
      |
      v
AST
      |
      v
Semantic validation
      |
      v
Validated semantic model
      |
      v
Canonical IR systems
      |
      v
Compiler and backend

The toolchain MUST NOT:

- Parse Zamani source using its own parser.
- Implement an independent ANTLR grammar loader as a replacement for the frontend's grammar integration.
- Duplicate token definitions.
- Interpret source syntax independently.
- Perform source-language type checking.
- Decide source-level effect validity.
- Create a competing resource semantics model.
- Introduce hardware-specific source-language syntax.

Project-manifest parsing and source-language parsing are separate responsibilities.

The former belongs to "zamani_project_config/"; the latter belongs to "frontend/".

---

4. Toolchain responsibilities

The toolchain owns the developer-facing lifecycle and coordinates the following operations.

Responsibility| Toolchain ownership
Project management| Own
Workspace management| Own
Manifest adaptation| Own adapter; canonical schema remains external
Dependency declarations| Consume canonical manifest data
Dependency resolution| Own
Dependency lockfiles| Own
Build orchestration| Own
Build-task coordination| Own
Incremental build coordination| Integrate existing compiler systems
Compiler invocation| Coordinate
Target intent| Coordinate canonical target contracts
Target feasibility| Consume authoritative compiler assessment
External tool discovery| Integrate backend authority
External process execution| Reuse backend
Artifact collection| Integrate canonical artifact infrastructure
Package creation| Own
Package extraction| Own
Package installation| Own
Registry coordination| Own
Package publication| Own
Developer commands| Own orchestration and dispatch
Testing workflows| Coordinate test infrastructure
Environment validation| Coordinate authoritative providers
Deployment planning| Own coordination
Deployment execution| Delegate to authorized providers
Agent access| Own integration contracts and authorization handoff
Diagnostics| Aggregate and render structured diagnostics
Security| Coordinate canonical security authorities
Runtime execution| Delegate to ZUR

---

5. Architectural ownership boundaries

These boundaries are mandatory.

5.1 Frontend

Canonical authority:

"src/frontend/"

The frontend owns:

- Lexing.
- Parsing.
- AST construction.
- Source locations.
- Source-level diagnostics.
- Frontend contracts.

The toolchain supplies source and project context and invokes the appropriate frontend services.

It must not implement another frontend.

5.2 Semantic analysis

Canonical authority:

"src/frontend/semantic/"

This subsystem owns:

- Names and symbol resolution.
- Types and values.
- Effects.
- Requirements.
- Capabilities as semantic concepts.
- Constraints.
- Preferences.
- Hints.
- Resource semantics.
- Contracts.
- Policies and semantic validation.
- Provenance of semantic facts.
- Source-level validity.

The toolchain consumes the resulting validated semantic information.

It must not determine semantic validity independently.

5.3 Compiler

Canonical authority:

"src/compiler/"

The compiler owns:

- Compiler services.
- Compilation sessions.
- Compiler configuration.
- Analysis.
- Compilation pipelines.
- Compiler passes.
- Optimization.
- Compiler planning.
- Feasibility assessment.
- Target-specific compilation planning.
- Lowering coordination.
- Compilation verification.
- Compiler diagnostics.
- Compiler reproducibility.
- Compiler incremental infrastructure.

The toolchain owns project-level orchestration, not compiler implementation.

A toolchain build request must be translated into the canonical compiler request contract.

The build executor must not call private compiler passes or reconstruct a second compilation pipeline.

5.4 Intermediate representations

Canonical authorities include:

- "src/zuir/"
- Classical IR.
- "src/quantum/ir/"
- "src/hdl/" and its canonical HDL/hardware representation.

The toolchain may identify and package IR artifacts through existing artifact descriptors.

It must not construct a new IR or reinterpret an existing representation.

5.5 Backend

Canonical authority:

"src/backend/"

The backend owns:

- Backend registrations.
- Target backend implementations.
- Artifact generation.
- Backend compatibility.
- Backend capability descriptions.
- Backend-specific validation.
- External toolchain integration.
- Linking and related target realization interfaces.

The toolchain must invoke these authorities through public interfaces.

It must not implement target-specific code generation.

5.6 External process execution

Canonical authority:

"src/backend/process.rs"

The toolchain must reuse:

- "ProcessRunner".
- "ProcessRequest".
- "ProcessOutput".
- "CancellationToken".
- Existing timeout behavior.
- Existing output-capture behavior.
- Existing structured process failures.

The toolchain must never construct shell commands by concatenating untrusted strings.

Arguments must be supplied individually.

External executable authorization must be enforced before invocation.

5.7 External tool discovery

Canonical authority:

"src/backend/toolchain.rs"

The existing implementation owns:

- "ToolchainSpec".
- "ToolVersion".
- "VersionRequirement".
- "ToolchainDescriptor".
- "ToolchainManager".
- Executable resolution.
- Version probing.
- Version validation.
- Validated tool invocation.

"src/toolchain/external_tools/" must provide a toolchain-facing orchestration facade over this implementation.

It must not redefine these types or implement another executable discovery system.

A discovered executable is not automatically trusted.

5.8 Artifacts

Canonical authorities:

- "src/compiler/artifacts/"
- "src/backend/"
- Existing backend artifact storage and validation infrastructure.

The toolchain owns build-output coordination and package composition.

It does not own an independent artifact payload store or digest implementation.

An artifact must retain its actual:

- Identity.
- Content fingerprint or digest.
- Artifact format.
- Domain.
- Target.
- Compiler compatibility.
- ABI compatibility where applicable.
- IR compatibility where applicable.
- Dependency identity.
- Verification state.
- Provenance.
- Portability evidence.

A generated file is not automatically a verified artifact.

5.9 Hardware

Canonical authority:

"src/zhal/"

ZHAL owns hardware-provider integration, hardware discovery and hardware capability evidence.

The toolchain may request or consume hardware-related information through authorized public contracts.

It must not:

- Discover physical devices independently.
- Allocate physical hardware.
- Reserve accelerators.
- Allocate qubits.
- Perform physical routing.
- Perform hardware calibration.
- Implement device drivers.
- Maintain a competing hardware inventory.

5.10 Runtime

Canonical authority:

"src/zur/"

The runtime owns execution, scheduling, resource acquisition, runtime placement and execution lifecycle.

The toolchain may prepare an execution request and hand off a validated artifact.

It must not implement a second runtime.

5.11 Distributed and hybrid systems

Canonical authorities:

- "src/distributed/"
- "src/hybrid/"
- "src/zur/"

The toolchain coordinates distributed and hybrid project builds and deployment preparation.

Actual distributed scheduling, communication execution, resource placement, domain coordination and runtime execution remain with their owning systems.

---

6. Semantic and operational distinctions

Every toolchain subsystem must preserve the following concepts.

Concept| Meaning
Requirement| What must be satisfied for valid computation
Capability| What an environment or provider can supply
Preference| A desirable but nonmandatory realization property
Constraint| A mandatory restriction on acceptable realizations
Hint| Nonbinding implementation guidance
Budget| An explicitly configured operational allowance
Feasibility| An evidence-based assessment of whether a realization is possible
Availability| Whether a resource is currently available
Allocation| Acquisition or reservation of a resource
Realization| Mapping validated computation onto an implementation
Verification| Evidence that a defined validation contract passed

These concepts must never be collapsed into one boolean field.

For example:

- A requested target is not proof of feasibility.
- A capability declaration is not proof of present availability.
- A discovered device is not a reservation.
- A preference is not a requirement.
- An exhausted build budget does not make a program semantically invalid.
- A successful validation is not successful compilation.
- A successful compilation is not successful execution.

6.1 Feasibility states

The toolchain must preserve at least these outcomes where the underlying authority provides them:

- Passed.
- Blocked.
- Unresolved.
- NotEvaluated.

Missing evidence must never be promoted to Passed.

An unresolved capability must never be treated as available.

An unsupported operation must never be silently discarded.

The authoritative compiler or provider feasibility assessment must remain the source of truth.

---

7. POCO-REAF architecture

The toolchain must support one logical project and multiple target-specific realizations.

                  ONE ZAMANI PROJECT
                          |
                          v
                  ONE SOURCE IDENTITY
                          |
                          v
                 ONE DEPENDENCY IDENTITY
                          |
                          v
                ONE LOGICAL BUILD REQUEST
                          |
                          v
                VALIDATED BUILD SPECIFICATION
                          |
                          v
                  TARGET MATRIX
                          |
            +-------------+-------------+
            |             |             |
            v             v             v
         Target A      Target B      Target C
            |             |             |
            v             v             v
        Feasibility   Feasibility   Feasibility
        assessment   assessment   assessment
            |             |             |
            v             v             v
        Realization   Realization   Realization
            |             |             |
            v             v             v
        Artifact A    Artifact B    Artifact C
            |             |             |
            +-------------+-------------+
                          |
                          v
                 BUILD RESULT AND REPORT

A logical build identity must be distinct from individual target-realization identities.

The shared logical identity must account for:

- Project identity.
- Source identity.
- Dependency identity.
- Relevant language version.
- Manifest compatibility.
- Build configuration.
- Feature selections.
- Relevant compiler configuration.
- Applicable semantic contracts.

Each target realization must additionally account for its own:

- Target identity.
- Backend identity.
- Provider/toolchain identity.
- Relevant target configuration.
- Compatibility assessment.
- Feasibility evidence.
- Artifact format.
- Artifact verification evidence.

The toolchain must not claim that different target artifacts are byte-identical or interchangeable.

Their semantic compatibility must be established through the appropriate compiler, artifact and portability authorities.

7.1 POCO-REAF guarantees

The toolchain must provide:

1. Stable logical project identity.
2. Stable source and dependency identity.
3. Reusable logical build requests.
4. Explicit target selection.
5. Multi-target build coordination.
6. Target-specific compatibility assessment.
7. Target-specific artifact production.
8. Artifact verification.
9. Build provenance.
10. Reproducibility evidence.
11. Explicit failure for unsupported realizations.
12. Compatibility-aware future migration.

It must not promise impossible universal execution.

---

8. Final toolchain directory architecture

The architecture below preserves the existing toolchain subsystem boundaries and establishes the required final composition.

It is not permission to create duplicate implementations or speculative files.

src/toolchain/
│
├── mod.rs
├── ids.rs
├── version.rs
├── errors.rs
├── result.rs
│
├── api/
│   ├── mod.rs
│   ├── request.rs
│   ├── response.rs
│   ├── capabilities.rs
│   ├── compatibility.rs
│   └── versioning.rs
│
├── configuration/
│   ├── mod.rs
│   ├── budget.rs
│   ├── config.rs
│   ├── policies.rs
│   ├── profiles.rs
│   └── validation.rs
│
├── workspace/
│   ├── mod.rs
│   ├── discovery.rs
│   ├── layout.rs
│   ├── members.rs
│   ├── metadata.rs
│   └── validation.rs
│
├── manifest/
│   ├── mod.rs
│   ├── adapter.rs
│   ├── loading.rs
│   ├── validation.rs
│   └── compatibility.rs
│
├── dependencies/
│   ├── mod.rs
│   ├── model.rs
│   ├── constraints.rs
│   ├── features.rs
│   ├── sources.rs
│   ├── compatibility.rs
│   ├── graph.rs
│   ├── resolver.rs
│   ├── lockfile.rs
│   └── validation.rs
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
├── tasks/
│   ├── mod.rs
│   ├── model.rs
│   ├── graph.rs
│   ├── dependencies.rs
│   ├── registry.rs
│   ├── executor.rs
│   ├── cancellation.rs
│   └── reporting.rs
│
├── targets/
│   ├── mod.rs
│   ├── profiles.rs
│   ├── matrix.rs
│   ├── compatibility.rs
│   ├── selection.rs
│   └── validation.rs
│
├── external_tools/
│   ├── mod.rs
│   ├── requirements.rs
│   ├── resolution.rs
│   ├── validation.rs
│   └── reporting.rs
│
├── packages/
│   ├── mod.rs
│   ├── model.rs
│   ├── metadata.rs
│   ├── creation.rs
│   ├── archive.rs
│   ├── extraction.rs
│   ├── installation.rs
│   ├── verification.rs
│   └── validation.rs
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
├── environments/
│   ├── mod.rs
│   ├── discovery.rs
│   ├── configuration.rs
│   ├── fingerprints.rs
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
├── security/
│   ├── mod.rs
│   ├── policy.rs
│   ├── trust.rs
│   ├── signatures.rs
│   ├── integrity.rs
│   ├── provenance.rs
│   └── validation.rs
│
├── diagnostics/
│   ├── mod.rs
│   ├── diagnostic.rs
│   ├── aggregation.rs
│   ├── ordering.rs
│   ├── rendering.rs
│   └── reporting.rs
│
├── agents/
│   ├── mod.rs
│   ├── context.rs
│   ├── permissions.rs
│   ├── requests.rs
│   ├── results.rs
│   ├── audit.rs
│   └── compliance.rs
│
├── documentation/
│   ├── ARCHITECTURE.md
│   ├── INTEGRATION_CONTRACT.md
│   ├── BUILD_CONTRACT.md
│   ├── PACKAGE_CONTRACT.md
│   ├── PORTABILITY_CONTRACT.md
│   ├── SECURITY_CONTRACT.md
│   ├── AGENT_CONTRACT.md
│   └── READINESS_MATRIX.md
│
└── testing/
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

Architecture status: Existing module implementations must be retained and audited before any additions or removals. The tree identifies the intended final organization. A file absent from the inspected tree is a required addition, not an existing implementation.

The directory names are not a reason to duplicate existing code elsewhere in the repository.

---

9. Root foundation files

9.1 "mod.rs"

Ownership: Toolchain composition.

Must contain:

- Toolchain-wide safety attributes.
- Module declarations.
- Feature-gated public composition.
- Explicit public exports.
- Toolchain architecture version exposure.
- Toolchain initialization contracts, if initialization is required.
- Architectural boundary documentation.
- Root-level integration tests.

Must not contain:

- Build algorithms.
- Package algorithms.
- Registry implementations.
- Process execution.
- Compiler implementation.
- Hardware discovery.
- Runtime scheduling.

Integration:

- All child toolchain modules.
- "src/lib.rs".
- "Cargo.toml".

The composition root must be implemented after the child-module contracts have been verified.

9.2 "ids.rs"

Owns logical toolchain identities.

Must preserve strongly typed identifiers for:

- Projects.
- Workspaces.
- Packages.
- Sources.
- Tasks.
- Build operations.
- Registries.
- Providers.
- Deployment operations.
- Other toolchain-owned entities.

It must reuse canonical identifiers where another subsystem already owns the concept.

For example, it must not redefine backend "ToolchainId", compiler target identifiers or standard-library "ProviderId".

Identifiers must be stable and validated.

Identity must not depend on machine-local memory addresses.

Integration:

- Workspace.
- Dependencies.
- Packages.
- Tasks.
- Build.
- Registries.
- Deployment.
- API.

9.3 "version.rs"

Owns toolchain API version identity.

It must distinguish:

- Toolchain API version.
- Toolchain composition version.
- Manifest schema version.
- Package-format version.
- Registry-protocol version.
- Language version.
- Grammar version.
- Compiler version.
- IR schema version.
- Backend API version.
- Provider API version.

These versions must not be conflated.

The file must define compatibility policy references without implementing every subsystem's version comparison algorithm.

9.4 "errors.rs"

Owns structured toolchain-level failures.

Must distinguish:

- Invalid requests.
- Invalid configuration.
- Workspace failures.
- Manifest failures.
- Dependency failures.
- Resolution conflicts.
- Build preparation failures.
- Build execution failures.
- External-tool failures.
- Package failures.
- Registry failures.
- Authentication failures.
- Authorization failures.
- Deployment failures.
- Cancellation.
- Timeouts.
- Budget exhaustion.
- Resource exhaustion.
- Incomplete operations.
- Verification failures.
- Unknown remote outcomes.

Must preserve error causes and operation context.

Errors from compiler, backend, filesystem, network and provider authorities must not be flattened into indistinguishable strings.

9.5 "result.rs"

Defines canonical toolchain result aliases and structured operation results.

Must integrate with "errors.rs".

It must not introduce a second independent diagnostic or error taxonomy.

It must distinguish successful, partial, failed, cancelled and unresolved operations where appropriate.

---

10. Public API

The public API is the common interface for:

- CLI clients.
- IDE integrations.
- Editors.
- Automation.
- Human developers.
- AI agents.
- External development tools.
- Enterprise integrations.

"api/mod.rs"

Composes the public API.

Must expose only deliberate stable contracts.

"api/request.rs"

Defines typed requests.

Required request families include:

- Workspace discovery.
- Project validation.
- Build.
- Run.
- Test.
- Package.
- Install.
- Publish.
- Deployment.
- Toolchain inspection.

Requests must contain explicit project scope and operation intent.

A request must not imply authorization.

"api/response.rs"

Defines structured responses.

Must support:

- Operation identity.
- Correlation identity.
- Lifecycle status.
- Structured diagnostics.
- Artifact references.
- Dependency changes.
- Target-specific outcomes.
- Verification evidence.
- Partial-completion information.
- Failure causes.

"api/capabilities.rs"

Describes available toolchain operations and API capabilities.

Must distinguish toolchain API capabilities from:

- Language-semantic capabilities.
- Compiler capabilities.
- Backend capabilities.
- Hardware capabilities.
- Runtime capabilities.

"api/compatibility.rs"

Coordinates compatibility assessments across independently versioned contracts.

Must delegate actual compatibility decisions to their canonical owners.

"api/versioning.rs"

Defines API version negotiation.

Unknown incompatible versions must not be silently accepted.

---

11. Configuration

Canonical directory:

"configuration/"

This subsystem must preserve the existing singular "budget.rs" integration.

"configuration/mod.rs"

Composition root for configuration.

"configuration/budget.rs"

Integrates the existing compiler budget infrastructure.

Canonical authority:

"src/compiler/configuration/budget/"

The toolchain must share the same budget tracker for one logical operation.

It must not create a second accounting ledger.

Budget consumption must remain consistent across:

- Workspace processing.
- Dependency resolution.
- Package operations.
- Build tasks.
- Compilation.
- Artifact handling.
- Testing.
- Deployment preparation.

An operation that creates an independent tracker for the same logical budget must not silently bypass accounting.

"configuration/config.rs"

Owns aggregate toolchain configuration and namespaced configuration sections.

Must support configuration composition without owning domain-specific semantic settings.

"configuration/policies.rs"

Owns toolchain policy definitions and policy composition.

It must not replace security authorization or semantic policy authorities.

"configuration/profiles.rs"

Owns reusable toolchain configuration profiles, inheritance and resolution.

Profiles may include development, testing, production and specialized workflows.

They must remain extensible.

"configuration/validation.rs"

Coordinates configuration validation participants.

It must preserve validation outcomes and must not convert unresolved validation into success.

---

12. Workspace and manifest integration

12.1 "workspace/"

Workspace services own project discovery and workspace coordination.

"workspace/mod.rs"

Public workspace subsystem boundary.

"workspace/discovery.rs"

Discovers projects from explicitly supplied paths.

Must not assume the process working directory is the project root.

"workspace/layout.rs"

Interprets project and workspace layouts.

Must support configurable source organization rather than one mandatory directory arrangement.

"workspace/members.rs"

Maintains workspace-member identities and relationships.

"workspace/metadata.rs"

Produces deterministic workspace metadata and fingerprints.

"workspace/validation.rs"

Validates workspace structure, membership, path relationships and consistency.

12.2 "manifest/"

The manifest subsystem is an adapter.

Canonical owner:

"src/zamani_project_config/"

"manifest/mod.rs"

Exposes the toolchain-facing manifest interface.

"manifest/adapter.rs"

Projects canonical manifest structures into toolchain requests.

It must preserve original data and must not introduce a competing schema.

"manifest/loading.rs"

Coordinates manifest loading using the canonical parser.

Filesystem access must follow explicit path and security policies.

"manifest/validation.rs"

Validates represented manifest fields and coordinates canonical schema validation.

It must not claim that unrepresented sections have been validated.

"manifest/compatibility.rs"

Handles compatibility assessment and migration reporting for manifest versions.

It must not silently rewrite user manifests.

12.3 Required expansion of "src/zamani_project_config/"

The current inspected "ZamaniToml" model represents package metadata and dependencies, but its implementation does not yet represent the full project configuration described by the wider architecture.

The canonical manifest subsystem must be expanded before the toolchain can claim full project-management readiness.

Its future contract must accommodate:

- Package metadata.
- Dependencies.
- Workspace membership.
- Features.
- Build settings.
- Target intent.
- Profiles.
- External tool requirements.
- Registry declarations.
- Packaging policies.
- Testing configuration.
- Deployment intent.
- Version compatibility.

These must be added to the existing canonical project configuration owner.

Do not implement these as a second schema under "toolchain/manifest/".

---

13. Dependency management

Canonical directory:

"dependencies/"

The toolchain owns package dependency resolution, not compiler IR dependency analysis.

"dependencies/mod.rs"

Composes dependency management.

"dependencies/model.rs"

Owns canonical package dependency declarations and identities.

"dependencies/constraints.rs"

Represents and evaluates dependency version and compatibility constraints using registered evaluators.

"dependencies/features.rs"

Coordinates package feature activation and feature-dependent dependency expansion.

"dependencies/sources.rs"

Resolves local, path, registry and other registered dependency sources.

"dependencies/compatibility.rs"

Checks package, language, ABI and artifact compatibility using authoritative contracts.

"dependencies/graph.rs"

Owns package dependency topology, reachability, cycle detection and dependency-first ordering.

This graph must not replace:

- Compiler dependency graphs.
- Build-task graphs.
- ZUIR dependency graphs.

"dependencies/resolver.rs"

Owns dependency solving and candidate-resolution coordination.

Must support:

- Transitive dependencies.
- Version conflicts.
- Source conflicts.
- Feature unification.
- Multiple compatible package versions where the package model permits.
- Deterministic resolution.
- Explicit resolution failures.

"dependencies/lockfile.rs"

Owns deterministic lockfile creation, serialization, loading and verification.

Lockfiles must preserve the resolved dependency identity and relevant source integrity information.

"dependencies/validation.rs"

Validates resolved dependencies before build or installation.

A dependency graph must not be accepted merely because its structure can be parsed.

---

14. Build orchestration

Canonical directory:

"build/"

This is the central toolchain coordination subsystem.

It does not replace the compiler.

"build/mod.rs"

Composes the build subsystem and exposes its public boundary.

"build/request.rs"

Converts external requests into immutable validated build requests.

Must preserve:

- Project identity.
- Workspace scope.
- Target intent.
- Feature selections.
- Build profiles.
- Operational budgets.
- Relevant policies.
- Request correlation.
- Dependency identity.

"build/plan.rs"

Owns build-plan composition and association with authoritative compiler feasibility evidence.

It must not independently determine physical feasibility.

"build/graph.rs"

Owns the toolchain build dependency graph.

It must distinguish build-task dependencies from package dependencies and compiler dependencies.

"build/executor.rs"

Coordinates execution of planned build work.

It must invoke the actual compiler service and appropriate backend services.

It must not simulate compilation.

It must preserve cancellation, timeout, budget and failure semantics.

"build/profiles.rs"

Composes build profiles with canonical configuration profiles and compiler configuration.

"build/inputs.rs"

Produces immutable build-input snapshots.

Inputs must account for every relevant factor that can affect the result, including:

- Source content.
- Manifest content.
- Dependency lockfile.
- Dependency identities.
- Feature selections.
- Compiler configuration.
- Relevant toolchain fingerprints.
- Relevant environment information.
- Target-specific configuration.

The file must reuse existing canonical fingerprint authorities.

"build/outputs.rs"

Collects canonical compiler and backend artifact references.

It must not introduce a competing artifact payload representation.

"build/incremental.rs"

Integrates:

- "src/compiler/incremental/"
- Existing backend cache infrastructure.
- Canonical dependency fingerprints.
- Compiler artifact identities.

The toolchain coordinates project-level incremental builds.

The compiler retains authority over compilation invalidation and incremental compilation internals.

"build/reporting.rs"

Produces structured build reports.

Reports must distinguish:

- Preparation.
- Resolution.
- Feasibility.
- Compilation.
- Artifact generation.
- Verification.
- Partial completion.
- Failure.
- Cancellation.

A successful report must require actual successful execution of the requested build stages.

---

15. Task orchestration

Canonical directory:

"tasks/"

"tasks/mod.rs"

Composition root.

"tasks/model.rs"

Owns task definitions, identities, inputs, outputs and lifecycle contracts.

"tasks/graph.rs"

Must remain a facade over the canonical build graph where the existing implementation specifies that relationship.

It must not implement a second graph.

"tasks/dependencies.rs"

Owns task input/output dependency bindings and prerequisite validation.

"tasks/registry.rs"

Owns task-provider registration and resolution.

Multiple compatible providers must be supported.

"tasks/executor.rs"

Coordinates registered task providers.

Must not introduce an independent compiler executor or runtime scheduler.

"tasks/cancellation.rs"

Integrates shared cancellation with the canonical backend process cancellation token.

"tasks/reporting.rs"

Produces structured task execution results and evidence.

Task registration does not establish that a provider is trusted, available or production-ready.

---

16. Target management

Canonical directory:

"targets/"

Target management coordinates existing target authorities.

It does not establish a second target registry.

"targets/mod.rs"

Public composition boundary.

"targets/profiles.rs"

Coordinates target profiles with canonical compiler and backend configuration.

"targets/matrix.rs"

Coordinates multi-target build matrices.

Each target must have an independent assessment and outcome.

"targets/compatibility.rs"

Delegates authoritative compatibility evaluation to backend and compiler compatibility services.

"targets/selection.rs"

Applies configured selection policies to already assessed candidates.

It must not select unsupported targets.

"targets/validation.rs"

Coordinates target validation and readiness evidence.

It must distinguish:

- Target identity.
- Target description.
- Capability declaration.
- Compatibility.
- Feasibility.
- Resource availability.
- Resource allocation.
- Realization.

The toolchain must not hard-code a finite list of architectures or vendors.

---

17. External tools

Canonical directory:

"external_tools/"

"external_tools/mod.rs"

Toolchain-facing integration facade.

"external_tools/requirements.rs"

Defines external tool prerequisites for requested operations.

"external_tools/resolution.rs"

Coordinates resolution through "backend::toolchain".

"external_tools/validation.rs"

Applies configured toolchain requirements, compatibility and security policies.

"external_tools/reporting.rs"

Reports installed, missing, incompatible, inaccessible and unverified tools.

This subsystem must not implement executable discovery or process execution independently.

---

18. Package management

Canonical directory:

"packages/"

The toolchain owns the package lifecycle.

"packages/mod.rs"

Composition root.

"packages/model.rs"

Defines package identities and package lifecycle contracts.

"packages/metadata.rs"

Defines package metadata projections and compatibility information.

"packages/creation.rs"

Creates packages from validated project contents.

Must apply explicit inclusion and exclusion rules.

"packages/archive.rs"

Defines the versioned ".zpkg" archive format.

Must establish:

- Format identity.
- Format version.
- Entry representation.
- Metadata representation.
- Content integrity.
- Compatibility requirements.
- Deterministic serialization where supported.
- Safe decoding rules.

"packages/extraction.rs"

Extracts package contents safely.

Must prevent:

- Path traversal.
- Malicious archive paths.
- Unsafe filesystem replacement.
- Symlink attacks.
- Unexpected file types.
- Archive resource exhaustion.

"packages/installation.rs"

Installs verified packages into explicitly configured package locations.

"packages/verification.rs"

Coordinates integrity, signature, metadata and compatibility verification.

"packages/validation.rs"

Validates package completeness and package-format conformance.

Source packages and precompiled artifacts must be distinguishable.

A precompiled artifact must retain its actual target and compatibility metadata.

---

19. Registry infrastructure

Canonical directory:

"registries/"

"registries/mod.rs"

Registry composition root.

"registries/provider.rs"

Defines the extensible registry-provider contract.

"registries/registry.rs"

Owns registry identity and configuration.

"registries/discovery.rs"

Discovers explicitly configured and registered registries.

"registries/client.rs"

Coordinates registry client operations.

"registries/protocol.rs"

Defines versioned registry request and response contracts.

"registries/authentication.rs"

Integrates authentication providers and credential references.

Secrets must not be stored in ordinary manifest or diagnostic metadata.

"registries/publishing.rs"

Coordinates package publication and publication-state transitions.

"registries/downloading.rs"

Coordinates package downloading, integrity verification and storage.

"registries/federation.rs"

Coordinates compatible federated registries.

"registries/validation.rs"

Validates registry responses, metadata and trust requirements.

Public, private, enterprise, local and self-hosted registries must be supported through provider contracts.

No single registry URL or vendor may be permanently hard-coded.

---

20. Developer commands

Canonical directory:

"commands/"

Commands must be thin adapters over toolchain services.

"commands/mod.rs"

Owns:

- Command identity.
- Command descriptors.
- Command registration.
- Command-handler contracts.
- Command dispatch.
- Authorization integration.
- Audit integration.
- Structured command responses.

It must not implement the operations themselves.

Individual commands

File| Responsibility
"build.rs"| Coordinate actual build preparation and compiler execution
"check.rs"| Invoke frontend, semantic and compiler validation services
"clean.rs"| Remove recognized generated outputs safely
"run.rs"| Build and hand validated artifacts to ZUR
"test.rs"| Coordinate configured test execution
"bench.rs"| Coordinate benchmarking
"format.rs"| Integrate canonical formatting services
"lint.rs"| Coordinate compiler, frontend and security diagnostics
"doc.rs"| Coordinate documentation generation
"package.rs"| Create validated packages
"install.rs"| Resolve and install verified packages
"publish.rs"| Coordinate authorized package publication
"target.rs"| Inspect registered target information
"tool.rs"| Inspect and validate external tools
"doctor.rs"| Diagnose project and environment readiness
"deploy.rs"| Coordinate deployment preparation and authorized provider handoff

Every command must:

1. Validate its request.
2. Establish authorization through the canonical authority.
3. Apply applicable policies and budgets.
4. Execute the owning service.
5. Preserve structured results.
6. Record required audit information.
7. Distinguish complete from partial success.

A command must not report success simply because its handler returned without a panic.

A command whose implementation or required provider is unavailable must not be advertised as fully operational.

---

21. Environments

Canonical directory:

"environments/"

"environments/mod.rs"

Composition root.

"environments/discovery.rs"

Coordinates discovery of explicitly relevant environments through authoritative providers.

"environments/configuration.rs"

Applies environment configuration.

"environments/fingerprints.rs"

Produces environment fingerprints using existing compiler and backend fingerprint contracts.

"environments/validation.rs"

Validates environment requirements and compatibility.

An environment fingerprint is not proof of trust or physical availability.

---

22. Deployment

Canonical directory:

"deployment/"

Deployment orchestration must be separate from deployment execution.

"deployment/mod.rs"

Composes deployment requests, profiles, validation, planning, providers and reporting.

"deployment/request.rs"

Adapts canonical API deployment requests.

"deployment/profiles.rs"

Coordinates deployment profiles with generic configuration profiles.

"deployment/validation.rs"

Owns evidence-based deployment validation orchestration.

It must preserve the outcomes:

- Passed.
- Blocked.
- Unresolved.
- NotEvaluated.

Structural validity and deployment readiness must remain distinct.

Validation must not:

- Authorize deployment.
- Reserve hardware.
- Allocate resources.
- Execute providers.
- Claim that deployment completed.
- Manufacture provenance.

"deployment/planning.rs"

Produces side-effect-free deployment plans.

A plan is not a reservation or execution result.

"deployment/providers.rs"

Owns extensible deployment-provider registration and resolution.

Provider registration is not provider trust or authorization.

"deployment/reporting.rs"

Produces structured deployment reports and evidence references.

Actual provider execution and physical resource acquisition remain with authorized provider and runtime authorities.

22.1 Deployment execution contract

The deployment lifecycle must distinguish:

Request
   |
   v
Structural validation
   |
   v
Configuration validation
   |
   v
Environment evidence
   |
   v
Feasibility assessment
   |
   v
Deployment planning
   |
   v
Authorization
   |
   v
Apply-time revalidation
   |
   v
Authorized provider invocation
   |
   v
Provider receipt
   |
   v
Deployment report

A successful plan is not a successful deployment.

A provider acceptance response is not necessarily successful completion.

A partial deployment must remain explicitly partial.

An unknown remote outcome must not be reported as failure-free success.

---

23. Security and supply-chain integrity

Canonical directory:

"security/"

Security must integrate with the repository's existing security and cryptographic authorities.

"security/mod.rs"

Composition root.

"security/policy.rs"

Coordinates toolchain security policy.

"security/trust.rs"

Coordinates trust decisions with authoritative trust providers.

"security/signatures.rs"

Integrates existing signing and signature-verification infrastructure.

It must not implement a second cryptographic signature algorithm.

"security/integrity.rs"

Coordinates package and artifact integrity verification.

"security/provenance.rs"

Coordinates provenance records and references to authoritative provenance evidence.

It must not fabricate provenance or replace compiler/backend provenance.

"security/validation.rs"

Coordinates security validation before sensitive operations.

Security requirements include:

- Input validation.
- Dependency integrity.
- Package integrity.
- Signature verification.
- Provider trust.
- Executable authorization.
- Credential protection.
- Filesystem containment.
- Archive safety.
- Network security.
- Artifact verification.
- Audit requirements.
- Reproducibility evidence.

A digest is not a signature.

A signature is not authorization.

A registered provider is not automatically trusted.

A successful security audit report is not itself proof that every security requirement has passed.

---

24. Diagnostics

Canonical directory:

"diagnostics/"

"diagnostics/mod.rs"

Public diagnostics composition.

"diagnostics/diagnostic.rs"

Defines structured toolchain diagnostics.

"diagnostics/aggregation.rs"

Aggregates diagnostics from workspace, manifest, dependencies, compiler, backend, package and deployment services.

"diagnostics/ordering.rs"

Provides deterministic diagnostic ordering.

"diagnostics/rendering.rs"

Provides human-readable presentation.

"diagnostics/reporting.rs"

Provides machine-readable output.

Diagnostics must preserve canonical source spans where available.

The toolchain must not invent source locations or flatten infrastructure failures into source-code errors.

---

25. Human and AI agent integration

Canonical directory:

"agents/"

Human developers and AI agents must use the same underlying toolchain services.

"agents/mod.rs"

Composition root.

"agents/context.rs"

Provides structured project and operation context.

"agents/permissions.rs"

Integrates the canonical authorization authority.

Agent-supplied identity must not be treated as authenticated identity.

"agents/requests.rs"

Defines structured agent requests.

"agents/results.rs"

Defines structured agent outcomes and advisory next actions.

"agents/audit.rs"

Coordinates audit recording through the canonical audit infrastructure.

"agents/compliance.rs"

Coordinates agent compliance checks.

It must not implement authentication, authorization, cryptographic verification or durable audit storage.

25.1 Human/AI parity

An AI agent must not receive a separate, weaker execution pathway.

Both human and AI requests must use:

- The same public API.
- The same validation.
- The same authorization authority.
- The same security policies.
- The same compiler.
- The same backend.
- The same budget accounting.
- The same artifact verification.
- The same audit requirements.

An agent may suggest a build plan.

It cannot independently declare a target feasible.

An agent may request deployment.

It cannot bypass deployment authorization.

Agent suggestions are advisory until accepted through the appropriate service.

---

26. Domain integration

The toolchain must remain domain-neutral.

It must not create a separate package manager or build engine for each computing domain.

26.1 Classical computing

Integrate with:

- "src/classical/"
- "src/compiler/"
- "src/backend/"
- "src/compiler/artifacts/"

Support native compilation, cross-compilation, libraries, executables, debugging profiles, optimization profiles and portable artifact management.

Classical computing is the initial practical production milestone, not a restriction on the architecture.

26.2 Quantum computing

Integrate with:

- "src/quantum/"
- "src/quantum/ir/"
- Existing quantum hardware and simulator interfaces.
- Backend and artifact infrastructure.
- ZHAL and ZUR.

Support quantum source projects, simulation workflows, provider-specific toolchains, quantum artifacts and compatible deployment preparation.

The toolchain must not implement:

- Quantum semantics.
- Gate semantics.
- Qubit allocation.
- Quantum routing.
- Quantum scheduling.
- Calibration.
- QEC.
- ZQN.

Those remain with their respective authorities.

26.3 HDL and hardware

Integrate with:

- "src/hdl/"
- Canonical HDLIR.
- Hardware realization.
- "src/zhal/"
- Backend toolchain providers.

Support synthesis and simulation workflows, hardware/software co-design, hardware artifact packaging and verification.

Physical placement, routing, synthesis and timing closure must remain with actual implementation providers.

26.4 AI and tensor computing

Integrate with:

- "src/ai/"
- Canonical tensor infrastructure.
- Compiler.
- Backend.
- Runtime.

Support model projects, datasets, training and inference dependencies, accelerator compatibility, model artifacts and edge deployment.

The toolchain must not implement another tensor engine or AI semantic system.

26.5 Hybrid computing

Integrate with:

- Classical computation.
- Quantum computation.
- AI and tensor computation.
- HDL and hardware.
- "src/hybrid/"
- Compiler and runtime.

Support one logical project containing coordinated domain-specific artifacts and interfaces.

Cross-domain dependencies must be explicit and validated.

26.6 Distributed computing

Integrate with:

- "src/distributed/"
- "src/zur/"
- Compiler.
- Backend.
- Deployment providers.

Support distributed project builds, dependency identity, multi-environment deployment descriptions and distributed artifact management.

Actual scheduling, communication and physical resource allocation remain outside the toolchain.

26.7 Future domains

A future computational domain must be able to participate through:

- Stable identifiers.
- Canonical semantic contracts.
- Applicable IR contracts.
- Provider registration.
- Target capability evidence.
- Artifact descriptors.
- Compatibility contracts.
- Build-task providers.
- Deployment integration where applicable.

Adding a domain must not require modifying unrelated domain implementations.

---

27. No-hard-coding and scalability contract

The following are prohibited as universal application or language ceilings.

Area| Prohibited arbitrary ceiling
CPUs| Maximum processor/core/thread count
GPUs| Maximum accelerator count
Quantum| Maximum qubit or QPU count
HDL| Maximum register/bus width or device count
Memory| Universal RAM or VRAM ceiling
Storage| Universal storage ceiling
Distributed| Maximum cluster participant count
Tensors| Maximum tensor rank or dimensions
Networking| Maximum topology size
Tasks| Arbitrary fixed task-count ceiling
Dependencies| Arbitrary dependency-count ceiling
Workspaces| Arbitrary member-count ceiling
Packages| Arbitrary package-count ceiling
Providers| Fixed provider inventory
Targets| Closed target inventory
Domains| Closed computing-domain inventory
Registries| One permanently hard-coded registry

Equivalent renamed constants are equally prohibited.

The implementation must use extensible identifiers, registries, providers, symbolic requirements and canonical capability contracts.

27.1 Permitted operational budgets

Finite operational budgets are permitted and required where appropriate.

Examples:

- Maximum build duration.
- Maximum download size.
- Maximum archive extraction size.
- Maximum concurrent operations.
- Maximum retained diagnostic output.
- Maximum memory allocated to a particular operation.
- Maximum retry count.
- Maximum network usage.
- Maximum disk usage.
- Maximum package cache usage.

These must be explicitly configured or supplied by an applicable policy.

A budget must not silently become a semantic capability limit.

An exhausted budget must produce an explicit operation outcome.

27.2 Representation limits

Rust collections and integer types have finite representational limits.

The architecture must not pretend otherwise.

Where representation limits matter, the implementation must:

- Validate conversions.
- Detect overflow.
- Avoid silent truncation.
- Avoid wrapping arithmetic for capacity accounting.
- Report allocation failures where the API permits.
- Use suitable integer or symbolic representations.
- Preserve the distinction between mathematical quantities and implementation counters.

No finite implementation can guarantee literal infinity.

---

28. Determinism and reproducibility

The toolchain must provide deterministic behavior wherever the relevant contract requires it.

This includes:

- Dependency resolution.
- Lockfile serialization.
- Workspace metadata.
- Build-input identity.
- Build graph ordering.
- Package metadata ordering.
- Package serialization where required.
- Registry metadata normalization.
- Diagnostics ordering.
- Artifact association.
- Build reports.

The toolchain must reuse existing compiler and backend fingerprint infrastructure.

It must not create an unrelated fingerprint algorithm.

Every build input capable of affecting the result must be represented in the relevant identity.

A fingerprint does not prove correctness, security, authorization or successful execution.

28.1 Reproducibility levels

The architecture should distinguish:

- Deterministic dependency resolution.
- Deterministic build planning.
- Deterministic build execution where supported.
- Reproducible artifact content.
- Reproducible package serialization.
- Reproducible deployment configuration.

A reproducibility claim must identify exactly which guarantee was established.

An external tool that does not provide deterministic output must not be described as reproducible merely because its invocation was recorded.

---

29. Security and failure semantics

All external inputs must be treated as untrusted until validated.

This includes:

- Project manifests.
- Dependency metadata.
- Registry responses.
- Package archives.
- Provider descriptors.
- External tool output.
- Deployment responses.
- Agent requests.
- Serialized artifacts.

29.1 Filesystem security

All project and package filesystem operations must:

- Validate paths.
- Enforce configured containment.
- Avoid path traversal.
- Handle symlinks according to explicit policy.
- Avoid unsafe overwrite behavior.
- Preserve existing user data.
- Use atomic commit patterns where required.
- Report partial operations.

29.2 Network security

Network operations must use explicit providers and security policies.

Credentials must be protected.

Network responses must be structurally validated.

Retries must respect configured policies and operation idempotency.

29.3 Failure preservation

The following must never be silently converted to success:

- Missing executables.
- Failed version probes.
- Incompatible tools.
- Failed dependency resolution.
- Invalid packages.
- Invalid signatures.
- Incomplete compilation.
- Failed artifact verification.
- Cancellation.
- Timeout.
- Budget exhaustion.
- Resource exhaustion.
- Provider failure.
- Partial deployment.
- Unknown remote outcome.

The toolchain must preserve causal errors and partial-completion evidence.

---

30. Integration outside "src/toolchain/"

The architecture requires coordinated changes elsewhere in the repository.

These are integration obligations, not permission to duplicate the owning subsystems.

30.1 "src/lib.rs"

The current inspected library root does not declare the toolchain module.

It must expose the subsystem under the build-system feature.

Required integration:

#[cfg(feature = "buildsystem")]
pub mod toolchain;

The library root must retain:

#![forbid(unsafe_code)]

The toolchain must compile with the appropriate feature enabled.

The public API must not accidentally require the full aspirational computing feature set merely to use basic project/build functionality.

30.2 "Cargo.toml"

The existing manifest declares:

[features]
default = []
lsp = []
buildsystem = []
full = []
omniversal = []

The build-system feature must control the toolchain surface.

Dependencies must be added only when justified by an established implementation contract.

Optional integrations should be feature-gated where practical.

The "zamani-pkg" binary must continue to require "buildsystem".

The declared Rust minimum version must remain consistent with the repository's normative Rust baseline.

The grammar architecture currently specifies Rust 1.97.1 or later in places, while Cargo declares Rust 1.97. This discrepancy must be explicitly reconciled before release. Do not claim a verified MSRV until the actual toolchain matrix has been tested.

30.3 "src/zamani_project_config/"

Expand the canonical project configuration owner.

Do not create another manifest parser in the toolchain.

The manifest must eventually represent the full project configuration needed by the build and package lifecycle.

All new sections require:

- Schema ownership.
- Parsing.
- Validation.
- Version compatibility.
- Migration policy.
- Tests.
- Toolchain adapter integration.

30.4 "src/compiler/"

The toolchain must use public compiler services.

The compiler integration must support:

- Validated compilation requests.
- Compiler configuration.
- Relevant budgets.
- Target intent.
- Compilation outcomes.
- Diagnostics.
- Feasibility evidence.
- Artifact references.
- Provenance.

The toolchain must not call private compiler implementation details.

30.5 "src/backend/"

The toolchain must integrate the actual backend public interfaces.

It must reuse:

- Backend target descriptions.
- Capability evidence.
- Compatibility evaluation.
- Backend validation.
- External tool discovery.
- Process execution.
- Artifact generation.
- Artifact verification.

The existing backend implementation must be audited for target-dispatch correctness and artifact validity.

A documented backend architecture is not proof that every backend is correct.

30.6 "src/compiler/artifacts/"

The toolchain must consume canonical artifact descriptors and verification contracts.

Package creation must not bypass artifact validation.

30.7 "src/zhal/"

Hardware requirements and capability evidence must be obtained through canonical providers.

The toolchain must not make its own hardware availability claims.

30.8 "src/zur/"

The "run" and "deploy" command pathways must use canonical runtime interfaces.

Compilation success must not imply runtime success.

30.9 "src/bin/zamani_pkg_main.rs"

The package-manager executable must become a thin command-line adapter.

Its responsibilities are:

1. Parse command-line arguments.
2. Construct typed toolchain requests.
3. Invoke the public toolchain API.
4. Render structured results.
5. Return appropriate process exit codes.

It must not:

- Parse TOML independently.
- Resolve dependencies independently.
- Compile through simulated functions.
- Choose an implicit universal native target.
- Extract packages independently.
- Implement registry communication.
- Bypass authorization.
- Claim successful operations that were not executed.

The existing simulated build message must be removed before any production-readiness claim.

30.10 Tests

The existing "tests/toolchain_integration_tests.rs" must be audited.

Its causality and theorem-prover tests must be assigned to their actual semantic/formal-verification owners if they are unrelated to toolchain behavior.

The toolchain integration suite must test actual toolchain workflows.

Recommended external test organization:

tests/
└── toolchain/
    ├── api.rs
    ├── workspace.rs
    ├── manifest.rs
    ├── dependencies.rs
    ├── build.rs
    ├── tasks.rs
    ├── targets.rs
    ├── external_tools.rs
    ├── packages.rs
    ├── registries.rs
    ├── commands.rs
    ├── deployment.rs
    ├── agents.rs
    ├── security.rs
    ├── determinism.rs
    ├── reproducibility.rs
    ├── portability.rs
    └── downstream/

The test suite must use real implementations for end-to-end acceptance.

Mocks are appropriate for isolated contract tests, not for claiming real provider compatibility.

---

31. Dependency direction

The toolchain dependency direction must be:

Canonical language and frontend
              |
              v
Canonical semantic model
              |
              v
Canonical compiler services
              |
              v
Canonical IR and backend services
              |
              v
Canonical artifact contracts
              |
              v
Toolchain build and package orchestration
              |
              v
ZUR / authorized deployment providers

The toolchain may coordinate several authorities, but those authorities must not depend on toolchain implementation internals.

The toolchain must not become a prerequisite for using the core compiler library.

The dependency graph must remain acyclic.

Within the toolchain:

Foundation contracts
       |
       v
Public API and configuration
       |
       v
Workspace and manifest adapters
       |
       v
Dependency and package foundations
       |
       v
Target and external-tool integration
       |
       v
Task and build orchestration
       |
       v
Commands and deployment
       |
       v
CLI / IDE / agent clients

This is a conceptual dependency order. Actual Rust module imports must be recorded explicitly and must not create circular dependencies.

---

32. Initialization and side-effect policy

Importing the toolchain must not automatically:

- Scan the filesystem.
- Discover external executables.
- Connect to registries.
- Access credentials.
- Discover physical hardware.
- Initialize providers.
- Allocate resources.
- Start background workers.
- Launch processes.
- Run builds.
- Deploy applications.

All side-effecting operations must be explicit.

Services must receive their dependencies through explicit construction or dependency injection.

No hidden global mutable service registry should be introduced.

Provider registry locks must not be held while invoking provider code.

---

33. Concurrency and task execution

The toolchain must support scalable execution without imposing a fixed universal worker count.

It must not define a permanent worker ceiling as a language or hardware limit.

Concurrency must be governed by:

- Explicit operational budgets.
- Provider concurrency contracts.
- Operating-system capabilities.
- Available memory.
- Build dependencies.
- Resource availability.
- Cancellation.
- Security policies.

The task subsystem owns toolchain task orchestration.

The compiler owns compilation concurrency.

ZUR owns runtime scheduling.

Distributed infrastructure owns distributed execution coordination.

These must not become competing schedulers.

Operations must define:

- Cancellation behavior.
- Timeout behavior.
- Partial-completion behavior.
- Retry safety.
- Idempotency.
- Failure propagation.
- Concurrent access guarantees.

---

34. Compatibility and API evolution

The toolchain must preserve independent compatibility identities for:

- Language.
- Grammar.
- Manifest.
- Toolchain API.
- Compiler API.
- IR.
- Backend API.
- Package format.
- Registry protocol.
- Provider API.
- Runtime interface.

Compatibility must be assessed explicitly.

A matching version string alone is insufficient proof of compatibility.

Breaking changes must include:

1. Identified affected contracts.
2. Explicit compatibility classification.
3. Migration requirements.
4. Tests.
5. Release documentation.
6. Defined deprecation and removal policy.

The toolchain must not silently reinterpret old manifests or lockfiles.

---

35. Production-readiness requirements

A module is not production-ready because:

- Its directory exists.
- Its file compiles in isolation.
- Its public types are defined.
- Its documentation describes the intended behavior.
- A mock provider returns success.
- A command prints a success message.

Production readiness requires evidence from real integrations.

35.1 Project infrastructure

- Canonical manifest parsing.
- Complete manifest validation.
- Workspace discovery.
- Workspace membership.
- Project configuration.
- Multi-project workspace support.
- Compatibility-aware manifests.

35.2 Dependency infrastructure

- Deterministic dependency resolution.
- Transitive dependency support.
- Conflict detection.
- Feature resolution.
- Lockfile creation.
- Lockfile verification.
- Source integrity.
- Dependency compatibility.

35.3 Build infrastructure

- Actual compiler service invocation.
- Dependency-aware build plans.
- Validated target intent.
- Authoritative feasibility assessment.
- Task-provider registration.
- Task cancellation.
- Shared budget accounting.
- Actual backend invocation.
- Verified artifact outputs.
- Incremental compilation integration.
- Structured build reports.

35.4 Package infrastructure

- Versioned archive format.
- Deterministic package metadata.
- Safe extraction.
- Integrity verification.
- Signature verification.
- Installation.
- Compatibility checks.
- Package-format migration policy.

35.5 Registry infrastructure

- Provider registration.
- Registry authentication.
- Downloading.
- Publishing.
- Metadata validation.
- Integrity verification.
- Private registry support.
- Federated registry support.
- Failure and retry semantics.

35.6 Target and domain infrastructure

- Classical integration.
- Quantum integration.
- HDL integration.
- AI and tensor integration.
- Hybrid integration.
- Distributed integration.
- Extensible provider contracts.
- Target-specific feasibility.
- Verified target-specific artifacts.

35.7 Developer experience

- Public toolchain API.
- CLI integration.
- Structured diagnostics.
- Check, build and run workflows.
- Testing.
- Benchmarking.
- Formatting.
- Linting.
- Documentation.
- Doctor diagnostics.
- Deployment coordination.

35.8 Security

- Dependency trust.
- Package integrity.
- Artifact verification.
- Credential protection.
- Provider authorization.
- Safe filesystem operations.
- Safe archive handling.
- External-tool authorization.
- Audit integration.
- Agent permission parity.
- Provenance preservation.

---

36. Implementation sequence and file-freezing policy

The architecture must be implemented in dependency order.

A file must not be started before its required upstream contracts are established.

The goal is to complete each file against a complete agreed interface so downstream integration does not force arbitrary redesign.

However, genuine defects discovered during integration must be corrected in the authoritative owning file. No architecture can guarantee that future integration will never uncover a defect.

Phase 0 — Architecture freeze

Establish:

- "documentation/ARCHITECTURE.md"
- "documentation/INTEGRATION_CONTRACT.md"
- "documentation/BUILD_CONTRACT.md"
- "documentation/PACKAGE_CONTRACT.md"
- "documentation/PORTABILITY_CONTRACT.md"
- "documentation/SECURITY_CONTRACT.md"
- "documentation/AGENT_CONTRACT.md"
- "documentation/READINESS_MATRIX.md"

Before implementation, define:

- Public APIs.
- Ownership.
- Dependency direction.
- Error contracts.
- Versioning.
- Security boundaries.
- Budget accounting.
- Thread-safety.
- Tests.
- Freeze criteria.

Phase 1 — Independent foundations

Start with:

1. "errors.rs"
2. "version.rs"
3. "ids.rs"
4. "result.rs"
5. "api/versioning.rs"
6. "api/compatibility.rs"
7. "api/capabilities.rs"
8. "configuration/budget.rs" integration audit
9. "configuration/policies.rs"
10. "configuration/profiles.rs"
11. "dependencies/model.rs"
12. "dependencies/constraints.rs"
13. "packages/model.rs"
14. "packages/metadata.rs"
15. "registries/protocol.rs"
16. "registries/provider.rs"
17. "tasks/model.rs"
18. "environments/fingerprints.rs"

These files establish stable contracts for later orchestration.

Existing implementations must be audited before replacing or recreating them.

Phase 2 — API and configuration

Complete:

- Public API composition.
- Request validation.
- Response validation.
- API compatibility.
- Configuration composition.
- Policy resolution.
- Profile inheritance.
- Shared operational budgets.
- Configuration validation.

Phase 3 — Workspace and manifest

Complete:

- Canonical manifest expansion.
- Manifest adapter.
- Manifest loading.
- Manifest validation.
- Compatibility.
- Workspace discovery.
- Layout.
- Membership.
- Metadata.
- Workspace validation.

Phase 4 — Dependencies and packages

Complete:

- Dependency source providers.
- Constraint evaluation.
- Feature resolution.
- Compatibility.
- Dependency graph.
- Resolver.
- Lockfile.
- Dependency validation.
- Package creation.
- Archive format.
- Extraction.
- Verification.
- Installation.

Phase 5 — External tools and target integration

Complete:

- External tool requirements.
- Backend toolchain integration.
- Tool validation.
- Target profiles.
- Target compatibility.
- Target matrices.
- Selection policies.
- Target validation.

Phase 6 — Task and build infrastructure

Complete:

- Task dependency bindings.
- Task registry.
- Task execution.
- Cancellation.
- Reporting.
- Build requests.
- Build plans.
- Build graph.
- Build executor.
- Build-input snapshots.
- Build-output integration.
- Incremental integration.
- Build reporting.

At the end of this phase, the toolchain must execute an actual classical build through the canonical compiler and backend services.

Phase 7 — Commands and registries

Complete:

- Command registration.
- Command dispatch.
- Command authorization integration.
- CLI adapters.
- Registry client.
- Downloading.
- Publishing.
- Federation.
- Registry validation.

Phase 8 — Deployment and agents

Complete:

- Environment validation.
- Deployment validation.
- Deployment planning.
- Provider registration.
- Authorized deployment execution.
- Deployment reporting.
- Agent context.
- Agent permissions.
- Agent requests.
- Agent results.
- Agent audit.
- Compliance.

Phase 9 — Domain expansion

Extend the same toolchain through canonical domain providers.

Recommended order:

1. Classical.
2. Embedded and systems.
3. Distributed.
4. AI and tensor.
5. HDL and hardware.
6. Quantum.
7. Hybrid.
8. Further registered domains.

This implementation order does not make the architecture classical-only. It prioritizes a practical initial production workflow.

Phase 10 — Production qualification

Run full tests, downstream integration, security validation, reproducibility testing and scalability testing.

No module should be labelled stable without the required evidence.

---

37. File-freezing requirements

Every implementation file must have a record in:

"documentation/READINESS_MATRIX.md"

Each record must contain:

1. Canonical path.
2. Purpose.
3. Authoritative ownership.
4. Responsibilities.
5. Explicit non-responsibilities.
6. Public types.
7. Public functions.
8. Inputs.
9. Outputs.
10. Dependencies.
11. Exact upstream contracts consumed.
12. Exact downstream consumers.
13. Error behavior.
14. Security requirements.
15. Concurrency guarantees.
16. Budget behavior.
17. Determinism requirements.
18. Scalability requirements.
19. Unit tests.
20. Integration tests.
21. Compatibility requirements.
22. Acceptance evidence.
23. Freeze criteria.

37.1 Definition of a frozen file

A file is frozen when:

- Its responsibility is unambiguous.
- Its API is complete for the agreed scope.
- Its dependencies are documented.
- Its upstream contracts are stable.
- Its downstream consumers compile.
- Its errors are structured.
- Its security requirements are met.
- Its tests pass.
- Its integration tests pass.
- No duplicate authority exists.
- No arbitrary universal capacity ceiling exists.
- No unsafe Rust exists.
- Its compatibility obligations are documented.

Freeze means that the contract is stable, not that the file can never receive a justified correction.

---

38. Testing and verification

The toolchain must be tested at multiple levels.

38.1 Unit tests

Test each file's complete contract independently.

38.2 Integration tests

Exercise real connections between:

- Manifest and workspace.
- Workspace and dependencies.
- Dependencies and packages.
- Build and compiler.
- Compiler and backend.
- Backend and artifacts.
- External tools and process execution.
- Deployment and providers.
- Commands and public API.
- Agents and authorization.

38.3 Conformance tests

Validate the toolchain against:

- POCO-REAF.
- Scalability.
- Portability.
- Determinism.
- Reproducibility.
- Security.
- Compatibility.

38.4 Domain fixtures

Use separate fixtures for:

Fixture| Validation
Minimal classical project| Basic project and build lifecycle
Multi-module classical project| Workspace and dependency management
Library project| Package and compatibility
Parallel application| Task coordination
Multi-target project| Target matrix and portability
Quantum project| Quantum provider integration
HDL project| Synthesis and simulation integration
AI project| Model and tensor integration
Hybrid project| Cross-domain build
Distributed project| Multi-environment build and deployment

Each fixture must use actual available implementations for any production-readiness claim.

Unsupported domains must be reported as unsupported, not simulated as successful.

38.5 Scalability testing

Test progressively larger finite workloads.

Measure:

- Memory use.
- Processing time.
- Build graph construction.
- Dependency resolution.
- Artifact processing.
- Package operations.
- Concurrency.
- Cancellation responsiveness.
- Failure recovery.

Test small systems as well as large systems.

A toolchain that only works on powerful development machines does not satisfy the scalability objective.

---

39. Required Rust verification

The repository baseline is Rust 2021 with Rust 1.97 or later.

No unsafe Rust is permitted.

The following checks are required:

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

Also verify:

- Default-feature compilation.
- Build-system-only compilation.
- Relevant feature combinations.
- Documentation tests.
- Downstream API consumption.
- Platform-specific external-tool integrations.
- Cross-compilation workflows where supported.

The repository must not claim the Rust baseline is verified until the tests have actually run on the declared minimum supported toolchain.

---

40. Production release gates

A production release requires all applicable gates to pass.

Gate| Required evidence
Architecture| Ownership and integration contracts satisfied
Module composition| All declared modules resolve
Feature integration| Correct "buildsystem" gating
Public API| Stable, validated request/response contracts
Manifest| Complete canonical schema and validation
Workspace| Real discovery and membership
Dependencies| Deterministic resolution and lockfiles
Build| Actual compiler execution
Backend| Actual compatible backend execution
Artifacts| Validated artifact references
Tasks| Real provider execution and cancellation
Packages| Safe archive creation and extraction
Registries| Real download and publishing workflows
Targets| Evidence-based compatibility and feasibility
Deployment| Authorized provider integration
Security| Verified security controls
Agents| Human/AI authorization parity
Portability| Semantic preservation evidence
Reproducibility| Verified applicable guarantees
Scalability| Measured finite-workload testing
Rust safety| No unsafe Rust
Documentation| Complete integration and operational contracts

A missing implementation must remain visibly incomplete.

A planned capability must not be described as an available production capability.

---

41. Final architectural decisions

The following decisions are binding for the toolchain architecture.

Question| Decision
Does toolchain own compilation?| No
Does toolchain own build orchestration?| Yes
Does toolchain own package management?| Yes
Does toolchain own dependency resolution?| Yes
Does toolchain own project/workspace management?| Yes
Does toolchain own canonical manifest parsing?| No
Does toolchain own manifest adaptation?| Yes
Does toolchain own compiler incremental internals?| No
Does toolchain coordinate incremental builds?| Yes
Does toolchain own backend code generation?| No
Does toolchain own external process execution?| No
Does toolchain own external tool discovery?| No; it coordinates the backend authority
Does toolchain own hardware discovery?| No
Does toolchain own resource allocation?| No
Does toolchain own runtime execution?| No
Does toolchain own deployment orchestration?| Yes
Does toolchain own physical deployment execution?| No
Does toolchain own package registries?| Yes, through extensible providers
Does toolchain support multiple computing domains?| Yes
Does toolchain support future providers?| Yes
Does toolchain support AI agents?| Yes
Must agents use the same authorization?| Yes
Are operational budgets allowed?| Yes
Are arbitrary universal hardware ceilings allowed?| No
Is literal infinite computation promised?| No
Is unsupported execution allowed to be reported as success?| No

---

42. Immediate repository corrections

Before beginning implementation of additional toolchain functionality, the following corrections must be addressed.

Priority| File| Required correction
Critical| "src/toolchain/mod.rs"| Implement the real composition root
Critical| "src/lib.rs"| Expose the feature-gated toolchain
Critical| "Cargo.toml"| Verify buildsystem feature and dependency boundaries
Critical| "src/bin/zamani_pkg_main.rs"| Remove simulated build success and use actual public services
Critical| "src/zamani_project_config/"| Expand the canonical manifest schema
Critical| "src/toolchain/build/"| Verify complete actual compiler integration
Critical| "src/toolchain/commands/build.rs"| Verify real preparation, execution and artifact verification
High| "src/toolchain/targets/"| Verify use of canonical compiler/backend feasibility
High| "src/toolchain/external_tools/"| Verify delegation to "backend::toolchain" and "backend::process"
High| "src/toolchain/configuration/budget.rs"| Verify shared canonical budget accounting
High| "src/toolchain/deployment/"| Verify evidence-based validation and authorized execution boundaries
High| "src/toolchain/packages/"| Verify actual archive and extraction security
High| "src/toolchain/registries/"| Verify real provider, download and publication workflows
High| "tests/toolchain_integration_tests.rs"| Audit ownership and replace unrelated tests with real toolchain tests

No additional duplicate subsystem should be introduced to solve an existing implementation defect.

---

43. Final architectural statement

"src/toolchain/" must become the unified development, build, package, distribution and deployment coordination infrastructure of the Zamani programming language.

Its architecture must allow a developer to express a project once and use the same project model, dependency model, build specification and developer workflow across compatible computational environments.

It must coordinate the repository's canonical authorities rather than compete with them.

The fundamental architecture is:

              ONE ZAMANI LANGUAGE
                       |
                       v
               CANONICAL FRONTEND
                       |
                       v
               SEMANTIC AUTHORITY
                       |
                       v
              CANONICAL IR SYSTEMS
                       |
                       v
                COMPILER SERVICES
                       |
                       v
               TOOLCHAIN SERVICES
                       |
          +------------+------------+
          |            |            |
          v            v            v
       Projects     Dependencies   Targets
          |            |            |
          +------------+------------+
                       |
                       v
                 BUILD SYSTEM
                       |
                       v
                 TASK SYSTEM
                       |
                       v
              VERIFIED ARTIFACTS
                       |
             +---------+---------+
             |                   |
             v                   v
          PACKAGES            DEPLOYMENT
             |                   |
             v                   v
          REGISTRIES       AUTHORIZED PROVIDERS
                                 |
                                 v
                           ZUR / EXECUTION

The architecture must remain extensible, deterministic where required, secure, observable, and resource-parametric.

It must support the smallest meaningful computation and arbitrarily large finite computations, subject to actual available resources and implementation capabilities.

The ultimate guarantee is not that every physical machine can execute every program. It is that Zamani does not unnecessarily tie program meaning, project architecture or development workflows to the limitations of a particular machine, vendor, hardware generation or deployment environment.

That is the toolchain foundation required to make POCO-REAF practical and sustainable.