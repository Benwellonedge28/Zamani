Zamani Universal Meta-Compiler Architecture

Canonical path: "src/compiler/documentation/ARCHITECTURE.md"

Repository: "github.com/Benwellonedge28/Zamani" (https://github.com/Benwellonedge28/Zamani)

Subsystem: Universal Meta-Compiler (UMC)

Rust edition: 2021

Minimum Rust version: Rust 1.97 or later

Memory safety: Safe Rust only. "unsafe" is forbidden.

Primary architectural objective:

"Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)"

Scalability objective: From the smallest meaningful computation to arbitrarily large computations, subject to program semantics, available resources, implementation capabilities, representation limits, policies, and physical reality.

Document classification: Normative compiler architecture and integration contract.

Scope: Compiler architecture, ownership, compilation pipeline, intermediate representation integration, transformations, optimization, hybrid computing, hardware planning, target realization, backends, artifacts, portability, scalability, security, compatibility, testing, and implementation governance.

---

1. Purpose

This document defines the production architecture of the Zamani Universal Meta-Compiler.

The compiler must enable developers to write a Zamani program once, preserve its meaning, compile it into portable representations, and realize it across compatible computing environments without requiring source-code modifications merely because the underlying hardware changes.

The compiler must support the complete intended Zamani computing ecosystem, including:

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
- Hardware description and hardware/software co-design.
- AI, machine learning, deep learning, and neural networks.
- Tensor and data computation.
- Symbolic and probabilistic computation.
- Autonomous and multi-agent computation.
- Neuromorphic computing.
- Photonic computing.
- Cryptographic and secure computation.
- Networking and communication.
- Accelerators and specialized processors.
- Future computational architectures.

These capabilities must operate as domains of one programming language.

The compiler must not become a collection of unrelated compilers that happen to accept the same source file.

It must coordinate the repository's existing authoritative subsystems through stable, explicit interfaces.

1.1 Core objective

The intended architecture is:

                    ZAMANI SOURCE
                          |
                          v
                  LANGUAGE FRONTEND
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
                 CANONICAL IR SYSTEMS
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
             CAPABILITY AND FEASIBILITY ANALYSIS
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
                 PORTABLE/TARGET ARTIFACTS
                          |
                          v
                 LINKING AND PACKAGING
                          |
                          v
                 DEPLOYMENT AND RUNTIME

The architecture must allow different valid compilation routes depending on the program's computational domains and the requested output.

For example, a purely classical program should not be forced through quantum processing.

A hybrid quantum-classical program must preserve the relationships between classical operations, quantum operations, measurements, control flow, synchronization, and data movement.

An HDL program must preserve hardware-intent semantics rather than treating hardware descriptions as ordinary CPU instructions.

1.2 Meaning of infinity

The phrase "scale to infinity" describes the absence of arbitrary universal capacity ceilings.

It does not mean that physical memory, computation time, storage, processors, or devices are infinite.

The compiler must:

1. Avoid artificial language-wide hardware limits.
2. Support dynamically sized and resource-parametric computations.
3. Use available resources efficiently.
4. Support configurable operational budgets.
5. Report actual resource exhaustion.
6. Preserve program meaning when changing execution strategies.
7. Avoid silent truncation and artificial success.
8. Support extensible hardware and backend registration.

---

2. Architectural authority

The compiler must respect the existing repository's ownership boundaries.

It is an orchestration and transformation subsystem, not the owner of every computing concept.

The following ownership model is mandatory.

Responsibility| Canonical owner
Language syntax| "grammar/"
Lexical implementation| "src/lexer.rs" and canonical frontend integration
Parsing| Canonical frontend parser
AST| "src/frontend/ast/"
Semantic meaning| "src/frontend/semantic/"
Legacy semantic compatibility| "src/semantic.rs"
Universal IR| "src/zuir/"
Classical computation| "src/classical/"
Quantum computation| "src/quantum/"
Quantum IR| "src/quantum/ir/"
HDL and hardware representation| "src/hdl/"
AI computation| "src/ai/"
Hybrid composition| "src/hybrid/"
Distributed computation| "src/distributed/"
Optimization implementation| "src/optimizer/" and domain optimization owners
Target-specific backend implementation| "src/backend/"
Compiler orchestration| "src/compiler/"
Physical execution| "src/runtime/"
Project and build integration| "src/toolchain/"
Source locations| "src/source_map/"

The compiler may define adapters, registries, orchestration contracts, and transformation coordination.

It must not create competing definitions of these responsibilities.

2.1 Semantic authority

"src/frontend/semantic/" owns validated source-level meaning.

The compiler must consume semantic results including, where applicable:

- Resolved names.
- Types and type relationships.
- Effects.
- Ownership and borrowing information.
- Capabilities.
- Requirements.
- Constraints.
- Preferences.
- Hints.
- Resources.
- Contracts.
- Policies.
- Provenance.
- Domain-specific validation results.

The compiler must not implement a second semantic type system.

It must not independently reinterpret source-level meaning.

The legacy "src/semantic.rs" remains a migration compatibility concern until its consumers are reconciled with the native semantic implementation.

2.2 IR authority

The compiler must integrate existing representations:

- ZUIR.
- Classical IR and ZCIR.
- "quantum::ir".
- HDLIR.
- Tensor and AI representations.
- Other explicitly registered domain representations.

The compiler must not create another universal IR to avoid integrating ZUIR.

It must not redefine quantum operations or hardware semantics.

It must establish explicit, verified adapters between representations.

2.3 Runtime authority

The compiler may produce abstract resource plans, execution dependencies, scheduling constraints, and deployment artifacts.

It must not take ownership of:

- Physical device allocation.
- Device leases.
- Runtime resource reservations.
- Actual execution.
- Runtime task scheduling.
- Resource release.
- Physical quantum execution.
- Runtime recovery.

Those responsibilities remain with the appropriate runtime and hardware subsystems.

---

3. Language and grammar integration

The compiler must conform to the language authority established in "grammar/".

The following documents are essential:

File| Authority
"grammar/DESIGN.md"| Grammar architecture and ownership
"grammar/specification/grammar-authority.md"| Language authority and conflict resolution
"grammar/specification/semantic-model.md"| Source-language meaning
"grammar/specification/compilation-model.md"| Compilation semantics
"grammar/specification/scalability-model.md"| Scalability requirements
"grammar/specification/poco-reaf.md"| POCO-REAF requirements
"grammar/specification/execution-model.md"| Execution and realization semantics
"grammar/specification/compatibility.md"| Compatibility requirements
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Historical and extended design reference
"grammar/Zamani.g4"| Canonical grammar composition root
"grammar/antlr/ZamaniLexer.g4"| Public ANTLR lexer boundary
"grammar/antlr/ZamaniParser.g4"| Public ANTLR parser boundary
"grammar/lexer/tokens.g4"| Token registry

The exact implementation status of individual grammar productions must be established through conformance testing.

3.1 Mandatory processing order

The conceptual frontend sequence is:

Zamani source
    |
    v
Grammar and language-version validation
    |
    v
Lexical analysis
    |
    v
Parsing
    |
    v
Canonical AST construction
    |
    v
Structural validation
    |
    v
Name and module resolution
    |
    v
Semantic analysis
    |
    v
Validated semantic program

The compiler must preserve source spans, source identity, diagnostics, and provenance through the pipeline.

3.2 Grammar must not determine hardware

The grammar describes computational intent.

It must not encode accidental properties of the host machine.

The compiler must distinguish:

- A semantic requirement.
- A target capability.
- An available resource.
- A resource budget.
- A target constraint.
- An optimization preference.
- A non-binding implementation hint.

A grammar production must not silently allocate hardware or select a physical device.

3.3 Grammar conformance

Every supported source construct must have a traceable path:

Grammar production
       |
       v
Lexer/parser representation
       |
       v
Canonical AST node
       |
       v
Semantic validation
       |
       v
Canonical semantic fact
       |
       v
Domain IR representation
       |
       v
Compiler transformation
       |
       v
Backend or explicit unsupported result

A grammar feature is not considered implemented merely because it can be parsed.

---

4. Compiler directory architecture

The existing compiler subdirectories must be retained and integrated rather than replaced with a competing architecture.

The canonical architectural structure is:

src/compiler/
│
├── mod.rs
│
├── api/
│   ├── mod.rs
│   ├── compiler.rs
│   ├── request.rs
│   ├── response.rs
│   └── compatibility.rs
│
├── configuration/
│   ├── mod.rs
│   ├── config.rs
│   ├── budget.rs
│   ├── policy.rs
│   ├── profile.rs
│   ├── target.rs
│   └── validation.rs
│
├── context/
│   ├── mod.rs
│   ├── compilation_context.rs
│   ├── compilation_state.rs
│   ├── dependency_context.rs
│   └── provenance_context.rs
│
├── pipeline/
│   ├── mod.rs
│   ├── stage.rs
│   ├── stages.rs
│   ├── ordering.rs
│   ├── execution.rs
│   ├── failure.rs
│   ├── recovery.rs
│   └── orchestration.rs
│
├── analysis/
│   ├── mod.rs
│   ├── program.rs
│   ├── dependency.rs
│   ├── effects.rs
│   ├── ownership.rs
│   ├── capabilities.rs
│   ├── requirements.rs
│   ├── feasibility.rs
│   ├── cost.rs
│   └── provenance.rs
│
├── ir/
│   ├── mod.rs
│   ├── input.rs
│   ├── adapters.rs
│   ├── verification.rs
│   ├── translation.rs
│   ├── lowering.rs
│   └── compatibility.rs
│
├── passes/
│   ├── mod.rs
│   ├── registry.rs
│   ├── manager.rs
│   ├── pipeline.rs
│   ├── ordering.rs
│   ├── preservation.rs
│   ├── legality.rs
│   └── analysis_manager.rs
│
├── optimization/
│   ├── mod.rs
│   ├── planning.rs
│   ├── cost_model.rs
│   ├── policies.rs
│   ├── decisions.rs
│   ├── profitability.rs
│   └── validation.rs
│
├── planning/
│   ├── mod.rs
│   ├── target_discovery.rs
│   ├── capability_matching.rs
│   ├── resource_planning.rs
│   ├── partitioning.rs
│   ├── scheduling.rs
│   ├── routing.rs
│   ├── data_movement.rs
│   └── feasibility.rs
│
├── lowering/
│   ├── mod.rs
│   ├── legalization.rs
│   ├── conversion.rs
│   ├── decomposition.rs
│   ├── outlining.rs
│   └── target_lowering.rs
│
├── backends/
│   ├── mod.rs
│   ├── registry.rs
│   ├── discovery.rs
│   ├── selection.rs
│   ├── capabilities.rs
│   ├── compatibility.rs
│   └── invocation.rs
│
├── artifacts/
│   ├── mod.rs
│   ├── artifact.rs
│   ├── manifest.rs
│   ├── metadata.rs
│   ├── portability.rs
│   ├── validation.rs
│   └── serialization.rs
│
├── linking/
│   ├── mod.rs
│   ├── symbols.rs
│   ├── relocation.rs
│   ├── composition.rs
│   └── validation.rs
│
├── incremental/
│   ├── mod.rs
│   ├── cache.rs
│   ├── fingerprints.rs
│   ├── invalidation.rs
│   └── persistence.rs
│
├── parallel/
│   ├── mod.rs
│   ├── build.rs
│   ├── scheduler.rs
│   ├── workers.rs
│   ├── dependencies.rs
│   └── determinism.rs
│
├── security/
│   ├── mod.rs
│   ├── policy.rs
│   ├── verification.rs
│   ├── supply_chain.rs
│   └── audit.rs
│
├── profiling/
│   ├── mod.rs
│   ├── events.rs
│   ├── measurements.rs
│   └── aggregation.rs
│
├── testing/
│   ├── mod.rs
│   ├── fixtures.rs
│   ├── conformance.rs
│   ├── differential.rs
│   ├── determinism.rs
│   ├── scalability.rs
│   └── regression.rs
│
└── documentation/
    ├── ARCHITECTURE.md
    ├── PIPELINE_CONTRACT.md
    ├── INTEGRATION_CONTRACT.md
    └── PORTABILITY_CONTRACT.md

This is the intended integrated architecture.

It is not a claim that every listed file already exists or that every implementation is complete.

Existing files must be reused when they already own the required responsibility.

No duplicate implementation may be introduced merely to match this directory layout.

---

5. Compiler root: "src/compiler/mod.rs"

The compiler root is the central integration point.

It must be a composition facade, not a place where every compiler algorithm is implemented.

The current repository snapshot returns empty content for this file. This is a critical integration issue.

The root must declare and expose the actual implemented compiler modules.

Its responsibilities are:

1. Declare compiler subsystem modules.
2. Expose the stable compiler API.
3. Preserve compatibility exports.
4. Define the canonical compilation entry points.
5. Coordinate compilation sessions.
6. Invoke the canonical pipeline.
7. Coordinate error propagation.
8. Return complete compilation outcomes.
9. Enforce compiler-wide invariants.
10. Expose version information.

The root must not:

- Implement semantic analysis.
- Define another AST.
- Define another universal IR.
- Implement another optimizer.
- Implement hardware discovery.
- Allocate physical resources.
- Implement target-specific code generation.
- Execute generated programs.

5.1 Root module integration

The root must register the existing submodules:

pub mod api;
pub mod configuration;
pub mod context;
pub mod pipeline;
pub mod analysis;
pub mod ir;
pub mod passes;
pub mod optimization;
pub mod planning;
pub mod lowering;
pub mod backends;
pub mod artifacts;
pub mod linking;
pub mod incremental;
pub mod parallel;
pub mod security;
pub mod profiling;
pub mod testing;

Additional legacy modules must only be declared when their files actually exist and their compatibility responsibilities have been established.

All declarations must be checked against the real filesystem before integration.

The root must expose the public interfaces deliberately. Avoid wildcard exports that create ambiguous ownership.

5.2 Compiler entry points

The existing compatibility API must be preserved until all consumers have been migrated.

Relevant interfaces include:

- "compile()".
- "compile_source()".
- "compile_with_config()".
- "production_config()".

Their signatures and semantics must be audited before changing them.

The new API under "api/" must delegate to the canonical compiler driver.

There must be exactly one authoritative compilation orchestration path.

---

6. Configuration architecture

Existing owner:

"src/compiler/configuration/"

This directory provides the foundational configuration contracts.

6.1 "configuration/config.rs"

Owns:

- Construction of the existing "CompilerConfig".
- Configuration validation.
- Compatibility adapters.
- Configuration validation modes.
- Structured configuration errors.

It must integrate with "src/compiler_types.rs" without defining competing configuration or target types.

6.2 "configuration/budget.rs"

Owns compiler operational budgets.

Budgets may cover:

- Source processing.
- IR processing.
- Memory accounting.
- Worker usage.
- Optimization work.
- Constant evaluation.
- Diagnostics.
- Generated artifact size.
- Compilation duration.
- Extension-defined work dimensions.

Budgets must be configurable and optional.

An absent budget does not mean physically unlimited resources.

Budget exhaustion must be explicit.

A stage must never silently truncate a program and return successful compilation.

6.3 "configuration/policy.rs"

Owns compiler-side policy representation and evaluation contracts.

It must distinguish:

- Mandatory requirements.
- Constraints.
- Preferences.
- Hints.
- Optimization policies.
- Fallback policies.
- Security policies.

Policy evaluation must be deterministic where required.

6.4 "configuration/profile.rs"

Owns named compilation profiles.

Profiles may define:

- Optimization policies.
- Verification requirements.
- Debugging configuration.
- Artifact preferences.
- Compilation behavior.
- Operational budgets.

Profiles must be extensible.

A profile must not silently override mandatory semantic requirements.

6.5 "configuration/target.rs"

Owns structural target descriptors.

A descriptor may include:

- Target identity.
- Architecture.
- Vendor.
- Operating system.
- Execution environment.
- ABI.
- Data layout.
- Target triple.
- Artifact formats.
- Extensible properties.

It does not own physical hardware discovery or allocation.

An absent target property means unknown or unspecified, not unlimited.

6.6 "configuration/validation.rs"

Performs aggregate configuration validation.

It must validate compatibility between configuration, policies, profiles, budgets, and target descriptions.

It must not independently implement hardware feasibility.

---

7. API architecture

Existing owner:

"src/compiler/api/"

7.1 "api/request.rs"

Owns the compilation request contract.

It must support:

- In-memory source.
- Filesystem source.
- Explicit configuration.
- Source identity.
- Optional compilation metadata.
- Cancellation and budget integration through established session/context contracts.

7.2 "api/response.rs"

Owns the public compilation response.

It must distinguish:

- Successful compilation.
- Portable artifact production.
- Target-specific artifact production.
- Partial diagnostic results.
- Unsupported target realization.
- Failed compilation.

It must not claim successful execution merely because an artifact was generated.

7.3 "api/compiler.rs"

Owns the compiler service interface.

It must delegate to a canonical compilation driver.

The existing compatibility adapter must preserve "compile_source()" and other established entry points.

7.4 "api/compatibility.rs"

Owns compatibility adapters between existing callers and the new compiler service.

It must not introduce a second compiler pipeline.

---

8. Compilation context

Existing owner:

"src/compiler/context/"

This directory must establish all shared information required by compilation stages.

8.1 "compilation_context.rs"

Must provide:

- Compilation identity.
- Source identity.
- Configuration snapshot.
- Pipeline identity.
- Diagnostic access.
- Budget access.
- Cancellation access.
- Provenance access.
- Immutable shared compilation information.

8.2 "compilation_state.rs"

Must define compilation lifecycle states and legal transitions.

Possible states include:

- Created.
- Configured.
- FrontendValidated.
- SemanticallyValidated.
- IRVerified.
- Optimized.
- Planned.
- Lowered.
- BackendRealized.
- ArtifactValidated.
- Completed.
- Failed.
- Cancelled.

These are lifecycle classifications, not a closed list of computational domains.

Incomplete compilation must never be represented as completed compilation.

8.3 "dependency_context.rs"

Must provide immutable dependency information to stages.

It integrates with:

- Incremental compilation.
- Module dependency resolution.
- IR dependencies.
- Backend dependencies.
- Artifact dependencies.

It must not duplicate the incremental fingerprinting implementation.

8.4 "provenance_context.rs"

Must maintain compilation provenance.

It should identify:

- Original source.
- Language version.
- Semantic version.
- IR version.
- Transformations.
- Optimization decisions.
- Backend identity.
- Artifact lineage.
- Verification evidence.

Provenance is not a substitute for artifact hashing or signature verification.

---

9. Compilation pipeline

Existing owner:

"src/compiler/pipeline/"

The pipeline coordinates independently implemented compilation stages.

9.1 "stage.rs"

Defines the common stage contract.

Each stage must specify:

- Stable stage identity.
- Inputs.
- Outputs.
- Preconditions.
- Postconditions.
- Dependencies.
- Required analyses.
- Resource requirements.
- Budget consumption.
- Failure categories.
- Cancellation behavior.
- Determinism properties.

9.2 "stages.rs"

Defines standard compiler stage identities and extension points.

It must support domain-specific stages without requiring modifications to a universal enum every time a new computational domain is added.

9.3 "ordering.rs"

Validates stage dependencies and establishes legal execution ordering.

It must detect dependency cycles.

It must not reorder transformations when ordering changes program meaning.

9.4 "execution.rs"

Executes pipeline stages.

It must provide:

- Structured results.
- Error propagation.
- Cancellation.
- Budget enforcement.
- Stage records.
- Diagnostic preservation.
- Provenance updates.

9.5 "failure.rs"

Defines structured pipeline failures.

It must distinguish:

- Source errors.
- Semantic errors.
- Invalid IR.
- Unsupported operations.
- Infeasible requirements.
- Resource exhaustion.
- Backend errors.
- Artifact errors.
- Infrastructure failures.
- Cancellation.

9.6 "recovery.rs"

Defines permitted retry and fallback evaluation.

Recovery must never silently change semantics.

A fallback is valid only if the program's declared requirements and semantic contracts remain satisfied.

9.7 "orchestration.rs"

Owns the complete pipeline lifecycle.

It must coordinate:

1. Frontend integration.
2. Semantic validation.
3. Ownership validation.
4. IR construction and verification.
5. Analysis.
6. Optimization.
7. Capability and feasibility planning.
8. Lowering.
9. Backend realization.
10. Artifact validation.
11. Linking and packaging where requested.

Stages must be conditional.

A computation must not pass through irrelevant domain stages.

---

10. Analysis architecture

Existing owner:

"src/compiler/analysis/"

The compiler analysis subsystem consumes authoritative semantic and IR information.

It must not become a second semantic analyzer.

10.1 "program.rs"

Provides program-level compilation analysis.

10.2 "dependency.rs"

Analyzes computational dependencies.

It must support:

- Data dependencies.
- Control dependencies.
- Effect dependencies.
- Synchronization dependencies.
- Cross-domain dependencies.
- Communication dependencies.

10.3 "effects.rs"

Consumes semantic effect information and provides it to transformation and optimization stages.

It must not redefine the semantic effect system.

10.4 "ownership.rs"

Coordinates ownership and aliasing information with semantic analysis.

A compiler ownership check must never return success simply because the integration is not implemented.

The native semantic ownership authority must be used where available.

An incomplete ownership analysis must be reported as incomplete or unsupported.

10.5 "capabilities.rs"

Consumes canonical capability descriptions.

It must not create a second hardware capability registry.

10.6 "requirements.rs"

Consumes semantic requirements and provides them to planning.

It must preserve the difference between requirements, constraints, preferences, and hints.

10.7 "feasibility.rs"

Coordinates feasibility analysis.

It must distinguish:

- Semantic validity.
- Capability compatibility.
- Resource feasibility.
- Backend availability.
- Lowering feasibility.
- Execution feasibility.

10.8 "cost.rs"

Coordinates cost estimates.

Cost estimates may include:

- Execution time.
- Energy.
- Communication.
- Memory movement.
- Compilation time.
- Resource utilization.
- Reliability.

Unknown cost must not be represented as zero.

10.9 "provenance.rs"

Tracks analysis evidence and decisions.

---

11. IR integration

Existing owner:

"src/compiler/ir/"

The compiler IR subsystem is an integration boundary, not another IR authority.

11.1 "ir/input.rs"

Accepts validated canonical representations.

It must identify the representation type and version.

11.2 "ir/adapters.rs"

Owns explicit adapters between supported IR representations.

Required integrations include:

- ZUIR.
- Classical IR/ZCIR.
- Quantum IR.
- HDLIR.
- Tensor and AI representations.

Adapters must preserve semantic properties.

A conversion that cannot preserve a required property must fail explicitly.

11.3 "ir/verification.rs"

Coordinates authoritative IR verification.

Verification must happen:

- Before transformations.
- After transformations where required.
- Before lowering.
- Before backend emission.
- After artifact generation where applicable.

The compiler must not accept invalid IR merely because a backend can process it.

11.4 "ir/translation.rs"

Coordinates explicit representation translations.

Each translation must identify:

- Source representation and version.
- Destination representation and version.
- Mapping rules.
- Preserved properties.
- Unsupported constructs.
- Verification evidence.
- Provenance.

11.5 "ir/lowering.rs"

Integrates with established ZUIR lowering.

The canonical lowering implementation must remain in its designated owner.

11.6 "ir/compatibility.rs"

Maintains legacy IR compatibility during migration.

It must not make "crate::ir_gen::IrModule" a permanent competing universal representation.

---

12. Compiler pass management

Existing owner:

"src/compiler/passes/"

The pass manager must provide extensible, verifiable transformation execution.

12.1 "passes/registry.rs"

Registers available passes.

Each pass must have a stable identifier, version, input/output contracts, and declared requirements.

12.2 "passes/manager.rs"

Coordinates pass execution.

12.3 "passes/pipeline.rs"

Defines configurable transformation sequences.

12.4 "passes/ordering.rs"

Validates transformation dependencies and ordering constraints.

12.5 "passes/preservation.rs"

Tracks semantic properties and analyses preserved by transformations.

12.6 "passes/legality.rs"

Determines whether a transformation is permitted.

12.7 "passes/analysis_manager.rs"

Caches analysis results and invalidates them when required.

12.8 Transformation contract

Every transformation must declare:

1. Input IR.
2. Output IR.
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

No transformation may silently discard a semantic requirement.

---

13. Optimization architecture

Existing owner:

"src/compiler/optimization/"

Integration:

"src/optimizer/"

The compiler optimization directory coordinates optimization decisions. It must not duplicate the existing optimizer.

13.1 "planning.rs"

Builds legal optimization plans.

13.2 "cost_model.rs"

Provides extensible cost-model interfaces.

13.3 "policies.rs"

Defines configurable optimization policies.

13.4 "decisions.rs"

Records optimization decisions and their justification.

13.5 "profitability.rs"

Evaluates whether an optimization is beneficial under the selected objective.

13.6 "validation.rs"

Validates semantic preservation and transformation correctness.

13.7 Optimization domains

The architecture must support optimization across:

- Classical IR.
- Quantum IR.
- HDL IR.
- Tensor and AI computation.
- Hybrid computation.
- Distributed computation.
- Networking.
- Cryptographic computation.
- Future domains.

Optimization algorithms remain owned by their appropriate subsystems.

Cross-domain optimization must use explicit contracts and verified transformations.

---

14. Heterogeneous planning

Existing owner:

"src/compiler/planning/"

This subsystem determines how validated computation may be realized.

It does not allocate physical hardware.

14.1 "target_discovery.rs"

Discovers registered target descriptions and available providers.

Actual hardware discovery may be delegated to backend, runtime, or hardware providers.

14.2 "capability_matching.rs"

Matches program operations and requirements against target capabilities.

Matching must consider:

- Required operations.
- Supported types.
- Numerical semantics.
- Memory models.
- Effects.
- Synchronization.
- Communication.
- Precision.
- Timing.
- Domain compatibility.
- Required interfaces.

14.3 "resource_planning.rs"

Produces abstract resource requirements and plans.

It must not reserve or allocate physical devices.

14.4 "partitioning.rs"

Partitions workloads across compatible execution providers.

It must support:

- Classical and quantum partitioning.
- AI and tensor partitioning.
- HDL and hardware boundaries.
- Distributed partitions.
- Cross-domain dependencies.
- Data locality.
- Communication cost.
- Synchronization.
- Explicit infeasibility.

Partitioning must preserve observable program behavior.

14.5 "scheduling.rs"

Produces abstract dependency-aware execution schedules.

Physical scheduling remains with the appropriate runtime and hardware authorities.

14.6 "routing.rs"

Coordinates abstract routing requirements.

Quantum physical routing, networking routes, and hardware placement must be delegated to their authoritative systems.

The compiler must not duplicate quantum topology, QEC, or runtime routing algorithms.

14.7 "data_movement.rs"

Plans data movement between computational domains and execution environments.

It must account for:

- Data representation.
- Conversion.
- Copying.
- Transfer cost.
- Memory locality.
- Synchronization.
- Ownership.
- Data lifetime.

14.8 "feasibility.rs"

Produces structured feasibility outcomes.

A result must distinguish:

- Feasible.
- Conditionally feasible.
- Infeasible.
- Unknown.
- Unsupported.
- Incomplete.

Unknown is not equivalent to feasible.

---

15. Lowering architecture

Existing owner:

"src/compiler/lowering/"

Lowering converts validated representations into forms accepted by subsequent stages.

15.1 "legalization.rs"

Validates and resolves representation-level legality requirements.

15.2 "conversion.rs"

Coordinates representation conversion.

It must use canonical IR adapters.

15.3 "decomposition.rs"

Coordinates decomposition of operations that cannot be represented directly by a destination.

Decomposition must preserve the required semantics.

15.4 "outlining.rs"

Coordinates extraction of independent kernels, functions, or computational regions.

15.5 "target_lowering.rs"

Coordinates target-specific lowering.

It must delegate to registered backend providers.

Target-specific lowering must not contaminate portable semantic representations.

---

16. Backend integration

Existing owner:

"src/compiler/backends/"

Authoritative implementation:

"src/backend/"

The compiler backend directory is an integration facade.

It must not become a competing backend implementation.

16.1 "backends/registry.rs"

Coordinates registered backend providers.

A provider must declare:

- Provider identity.
- Version.
- Supported IR representations.
- Target compatibility.
- Supported capabilities.
- Artifact formats.
- Toolchain dependencies.
- Required configuration.
- Security requirements.
- Failure behavior.

16.2 "backends/discovery.rs"

Discovers providers.

Discovery must not assume that every backend is installed.

16.3 "backends/selection.rs"

Selects compatible providers using verified capability and feasibility information.

It must not silently substitute a different target.

16.4 "backends/capabilities.rs"

Adapts canonical backend capability contracts.

16.5 "backends/compatibility.rs"

Validates:

- IR compatibility.
- Target compatibility.
- ABI compatibility.
- Artifact format compatibility.
- Backend version compatibility.

16.6 "backends/invocation.rs"

Invokes the authoritative backend implementation.

It must validate inputs and preserve structured failures.

16.7 Backend extensibility

The architecture must allow additional providers without requiring central compiler rewrites.

Examples include:

- LLVM.
- WebAssembly.
- Native CPU backends.
- GPU backends.
- FPGA backends.
- ASIC synthesis backends.
- Quantum execution backends.
- Quantum simulators.
- Neuromorphic backends.
- Future accelerators.

These are extension possibilities, not claims of existing production support.

---

17. Artifact architecture

Existing owner:

"src/compiler/artifacts/"

This subsystem owns compilation artifact contracts.

It must distinguish portable artifacts from target-specific realizations.

17.1 "artifact.rs"

Defines common artifact identity and representation.

17.2 "manifest.rs"

Defines artifact contents, dependencies, requirements, and compatibility metadata.

17.3 "metadata.rs"

Defines:

- Compiler identity.
- Language identity.
- Semantic version.
- IR identity.
- Backend identity.
- Provenance.
- Build information.

17.4 "portability.rs"

Defines artifact portability classifications.

Examples:

- Target-independent semantic artifact.
- Portable IR artifact.
- Domain-specific portable artifact.
- Target-specific artifact.
- Deployment bundle.

Portability must not be inferred merely from a file extension.

17.5 "validation.rs"

Validates artifact integrity, required metadata, representation compatibility, and verification status.

17.6 "serialization.rs"

Defines stable serialization contracts.

Serialization must be versioned.

Deserialization must validate the format before interpreting the content.

17.7 Portable artifact

A portable artifact should preserve:

- Program identity.
- Semantic contracts.
- Canonical IR.
- Domain modules.
- Required capabilities.
- Requirements and constraints.
- Effects.
- Provenance.
- Language version.
- IR versions.
- Dependency information.
- Compatibility information.

It must not depend unnecessarily on physical device identifiers.

17.8 Realized artifact

A target-specific artifact may include:

- Native executable.
- Native library.
- GPU binary.
- FPGA bitstream.
- ASIC implementation package.
- Quantum execution package.
- WebAssembly module.
- Hybrid deployment bundle.

The artifact must be validated before being represented as successfully generated.

A portable IR package must not be falsely described as a native executable.

---

18. Linking

Existing owner:

"src/compiler/linking/"

This is the canonical compiler linking subsystem.

18.1 "symbols.rs"

Owns symbol resolution and symbol compatibility.

18.2 "relocation.rs"

Owns relocation contracts and validated relocation application.

18.3 "composition.rs"

Owns composition of compatible compilation units and artifacts.

18.4 "validation.rs"

Owns link-time validation.

18.5 Linking requirements

Linking must support:

- Multiple modules.
- Cross-domain references.
- External symbols.
- Versioned dependencies.
- Portable artifacts.
- Target-specific artifacts.
- Hybrid bundles.
- Explicit unresolved dependencies.

It must integrate with "src/backend/" and "src/toolchain/".

It must not create a second linker elsewhere.

---

19. Incremental compilation

Existing owner:

"src/compiler/incremental/"

The incremental compiler must support sound dependency tracking and invalidation.

Its contracts must cover:

- Compilation fingerprints.
- Semantic dependencies.
- IR dependencies.
- Transformation dependencies.
- Backend dependencies.
- Artifact dependencies.
- Cache compatibility.
- Persistent cache metadata.
- Cache invalidation.

Cache keys must incorporate every relevant input that can change the compilation result.

These include configuration, language version, semantic version, IR version, pass configuration, backend identity, and relevant dependencies.

A cache hit must never bypass required verification or reuse an incompatible artifact.

Incremental compilation must integrate with "parallel/" without duplicating job execution.

---

20. Parallel compilation

Existing owner:

"src/compiler/parallel/"

The parallel subsystem must support:

- Dependency-aware compilation.
- Configurable concurrency.
- Independent compilation units.
- Cancellation.
- Failure propagation.
- Deterministic result collection.
- Resource-budget accounting.
- Incremental build integration.

Parallel execution must not introduce arbitrary worker-count ceilings.

Available worker capacity must be discovered or explicitly configured.

Parallel and sequential compilation must produce equivalent results wherever deterministic compilation is required.

The compiler must not assume that a larger worker count is always beneficial.

---

21. Security architecture

Existing owner:

"src/compiler/security/"

The existing security subsystem must integrate with:

- "src/compiler/ir/verification.rs".
- "src/compiler/backends/".
- "src/compiler/artifacts/".
- "src/compiler/analysis/".
- "src/backend/".
- Existing "src/compiler/safety_guard.rs", where retained.

Security verification must examine more than function names.

It must address applicable:

- IR operations.
- Effects.
- Data flows.
- Memory accesses.
- External calls.
- Toolchain invocations.
- Backend output.
- Artifact integrity.
- Dependency provenance.
- Signing and attestation.
- Authorization policies.
- Untrusted extensions.

Security verification must produce structured evidence.

An unavailable security check must not be represented as a successful check.

The compiler must not execute arbitrary generated backend source merely because it was successfully generated.

---

22. Profiling and observability

Existing owner:

"src/compiler/profiling/"

The profiling subsystem must record:

- Compilation-stage durations.
- Stage outcomes.
- Resource-accounting measurements.
- Optimization decisions.
- Backend invocation measurements.
- Artifact-generation measurements.
- Failure and cancellation information.

Profiling must not alter program semantics.

Measurements must include their source and reliability information.

An unavailable measurement must not be interpreted as zero.

Existing hybrid profiling functionality must be integrated without creating competing telemetry systems.

---

23. Hybrid computing

Hybrid computing is a central requirement of the Zamani compiler.

The compiler must support composition of multiple computational domains in one program.

Examples:

Classical → Quantum
Quantum → Classical
Classical → HDL
AI → Quantum
Quantum → AI
AI → GPU
Classical → FPGA
Quantum → Distributed
Classical → Quantum → Distributed
Classical → AI → Quantum → Hardware

The compiler must preserve the meaning of transitions between domains.

23.1 Hybrid integration

The compiler must integrate:

- "src/hybrid/".
- "src/frontend/semantic/domains/".
- "src/quantum/ir/".
- Classical IR.
- HDLIR.
- AI representations.
- Distributed representations.
- "src/runtime/hybrid_kernel.rs".
- Existing hybrid compiler compatibility interfaces where they remain present.

23.2 Domain boundaries

Every cross-domain transition must specify:

- Source domain.
- Destination domain.
- Input representation.
- Output representation.
- Type compatibility.
- Data conversion.
- Ownership and lifetime.
- Effects.
- Synchronization.
- Resource requirements.
- Error behavior.
- Provenance.

23.3 Hybrid synchronization

The compiler must preserve:

- Dependency ordering.
- Measurement dependencies.
- Classical control dependencies.
- Completion conditions.
- Cancellation semantics.
- Failure propagation.
- Data availability.
- Synchronization requirements.

Actual synchronization execution remains a runtime responsibility.

23.4 Quantum integration

The compiler must consume "quantum::ir" as the canonical quantum IR.

It must integrate with quantum optimization, scheduling, routing, hardware, QEC, and ZQN authorities.

It must not introduce a second quantum semantic representation.

The compiler may establish abstract requirements and legal transformation plans.

Physical qubit allocation, topology mapping, calibration, pulse generation, and QEC execution belong to their designated subsystems.

23.5 HDL integration

The compiler must integrate with the canonical HDLIR and hardware subsystem.

It must preserve distinctions between:

- Software execution.
- Hardware description.
- Hardware synthesis.
- Hardware/software co-design.
- Physical implementation.

An HDL artifact must not be presented as a synthesized physical device unless the actual synthesis and validation stages have completed.

---

24. AI and tensor integration

The compiler must support AI and tensor computation as first-class computational domains.

Integration must use:

- "src/ai/".
- Existing tensor representations.
- ZUIR AI dialects.
- Existing classical tensor facilities.
- Domain semantic validators.
- Appropriate backend providers.

Supported computational categories should include:

- Machine learning.
- Deep learning.
- Neural networks.
- Transformers.
- Generative computation.
- Symbolic reasoning.
- Probabilistic computation.
- Bayesian inference.
- Causal computation.
- Reinforcement learning.
- Federated learning.
- Distributed training.
- Agents and multi-agent systems.
- Differentiation and autodiff.
- Neuro-symbolic computation.
- Quantum machine learning.
- Neuromorphic computation.

The compiler owns coordination, not AI algorithm implementation.

Tensor shapes must support symbolic and dynamic dimensions where defined by the authoritative tensor model.

No arbitrary universal tensor-rank or accelerator-count ceiling may be introduced.

---

25. Distributed and networking integration

The compiler must integrate distributed computation through:

- "src/distributed/".
- Existing networking interfaces.
- Canonical semantic requirements.
- ZUIR distributed dialects.
- Runtime deployment and execution.

Compiler responsibilities include:

- Distributed dependency analysis.
- Communication requirements.
- Partition compatibility.
- Data movement planning.
- Failure-contract validation.
- Artifact composition.
- Deployment metadata.

Runtime responsibilities include:

- Actual node allocation.
- Communication execution.
- Distributed scheduling.
- Resource leases.
- Failure detection.
- Recovery.
- Resource release.

The compiler must not duplicate the distributed runtime.

---

26. Scalability and hard-coding policy

This is a mandatory compiler-wide invariant.

No arbitrary hardware-capacity ceilings may be embedded in the compiler.

The prohibition includes fixed universal maximums for:

- CPUs.
- Cores.
- Threads.
- Workers.
- GPUs.
- FPGAs.
- ASICs.
- QPUs.
- Qubits.
- Logical qubits.
- Distributed nodes.
- Devices.
- Memory.
- Storage.
- Registers.
- Register width.
- Tensor dimensions.
- Tensor rank.
- Hardware topology.
- Network size.
- IR operation count.
- Compilation-unit count.
- Domain count.

26.1 Acceptable limits

A finite operational limit is acceptable when it is:

1. Explicitly configured.
2. Justified by an operational or security requirement.
3. Associated with a defined unit.
4. Enforced consistently.
5. Reported through structured errors.
6. Distinct from source-language semantics.
7. Not presented as a universal hardware maximum.

For example, a user may configure a maximum compilation duration.

That must not become a language-wide maximum execution duration.

26.2 Integer representation

Use appropriate checked numeric representations.

Do not assume that "usize" represents every possible semantic resource quantity.

Where wider accounting is required, use an appropriate fixed-width or extensible representation and report overflow.

A numeric representation limit is an implementation constraint, not a claim that the language itself has the same semantic limit.

26.3 Scalability behavior

When resources become unavailable:

- Stop the affected operation safely.
- Preserve completed work where valid.
- Report the resource failure.
- Do not claim complete compilation.
- Do not silently discard required computation.
- Offer a valid alternative only when the semantic contract permits it.

26.4 Streaming

Large workloads should use streaming interfaces where practical.

Avoid unnecessary materialization of:

- Entire source collections.
- Complete IR graphs.
- Full diagnostic histories.
- All optimization candidates.
- All compilation jobs.
- Complete artifact contents.

The implementation must use memory proportionally to the work actually required, subject to its algorithms and representation.

---

27. Portability and POCO-REAF

The compiler must preserve the following invariant:

One source program → one semantic meaning → multiple valid realizations.

Portability must be based on semantic preservation, not identical machine instructions.

27.1 Program Once

Developers express computation and its requirements without unnecessary target-specific assumptions.

27.2 Compile Once

The compiler produces portable, verified representations and artifacts.

Target-specific compilation may occur later when a particular realization is required.

This distinction is important: a portable compilation artifact is not necessarily a native executable.

27.3 Run Everywhere

A program can be realized on compatible targets without rewriting its source.

27.4 Run Anywhere

The compiler and deployment architecture support different execution environments, provided their capabilities and policies satisfy the program contract.

27.5 Forever

Compatibility must be governed through explicit versioning.

The compiler must preserve:

- Language identity.
- Semantic identity.
- IR compatibility.
- Artifact compatibility.
- Backend compatibility.
- Dependency compatibility.
- Provenance.

Future compiler versions must not silently reinterpret existing portable artifacts.

27.6 Explicit feasibility

POCO-REAF does not guarantee that every computation can run on every machine.

If a target cannot satisfy a required capability or resource requirement, the compiler must report that fact.

It must not silently remove the requirement or substitute different semantics.

---

28. Error and diagnostic architecture

All compiler stages must use structured diagnostics.

Diagnostics must identify:

- Stable error code.
- Severity.
- Compilation stage.
- Source location.
- Affected representation.
- Explanation.
- Related locations.
- Suggested correction where available.
- Provenance.
- Recovery status.

The architecture must distinguish:

- Invalid source.
- Invalid AST.
- Semantic failure.
- Invalid IR.
- Illegal transformation.
- Unsupported operation.
- Capability mismatch.
- Resource infeasibility.
- Lowering failure.
- Backend failure.
- Linking failure.
- Artifact validation failure.
- Security failure.
- Infrastructure failure.
- Cancellation.

Errors must preserve their original classification through API and compatibility adapters.

An incomplete stage must never be converted into a successful compilation result.

---

29. Determinism and reproducibility

The compiler must define determinism at each applicable stage.

For identical:

- Source.
- Language version.
- Semantic configuration.
- Compiler version.
- Pass configuration.
- Backend version.
- Relevant dependency snapshots.

the compiler should produce reproducible outputs wherever the applicable compilation contract requires determinism.

Deterministic behavior must not depend on:

- Hash-map iteration order.
- Worker completion order.
- Randomized optimization decisions.
- Unstable plugin registration order.
- Unrecorded environmental inputs.

Nondeterministic operations must be explicitly identified and governed.

---

30. Compatibility and migration

The compiler must support controlled migration from existing implementations.

The current repository contains both older and newer architectural surfaces.

In particular:

- "src/compiler_types.rs" remains an existing configuration authority.
- "src/ir_gen.rs" remains a legacy IR integration point.
- "src/semantic.rs" remains a legacy semantic integration point.
- "src/backend/" retains compatibility with legacy backend implementations.
- The newer frontend semantic and ZUIR subsystems must be integrated through explicit adapters.

Migration must follow this order:

1. Identify existing consumers.
2. Identify the authoritative replacement.
3. Define compatibility adapters.
4. Add behavior-preservation tests.
5. Migrate callers.
6. Verify that the old implementation no longer has independent production ownership.
7. Remove obsolete compatibility paths only after migration evidence exists.

Do not delete functioning code merely because a new architecture has been proposed.

Do not preserve two competing production authorities indefinitely.

---

31. Integration with the repository root

The compiler must be integrated through "src/lib.rs".

The current repository root must be reconciled with the newer subsystem architecture.

The existing "src/lib.rs" exposes legacy modules including:

pub mod backend;
pub mod compiler;
pub mod compiler_types;
pub mod ir_gen;
pub mod ir_verify;
pub mod optimizer;
pub mod parser;
pub mod runtime;
pub mod semantic;

The newer architecture also requires appropriate integration of:

- "frontend".
- "zuir".
- "classical".
- "hdl".
- "hybrid".

These declarations must be added only after the actual module roots and child dependencies are verified.

The compiler must not reference modules that are inaccessible through the crate module tree.

The integration must be tested using the actual Rust compiler.

31.1 Backend migration

"src/backend/mod.rs" already identifies a transitional compatibility boundary.

The migration must reconcile:

- "src/backend.rs".
- "src/backend/mod.rs".
- "src/backend/legacy.rs".
- Existing target implementations.
- Compiler backend adapters.

The target-dispatch defect and misleading LLVM artifact behavior identified in the earlier COMPILER audit must be corrected in the owning backend implementation.

The compiler must never report a successful executable artifact when the backend only produced descriptive text or an unvalidated intermediate representation.

---

32. Testing architecture

Existing owner:

"src/compiler/testing/"

The compiler must have a complete conformance and integration testing system.

Required test categories:

Test category| Purpose
Unit| Verify individual contracts
Integration| Verify subsystem composition
Grammar conformance| Verify language implementation
Semantic conformance| Verify program meaning
IR verification| Reject invalid representations
Transformation| Verify semantic preservation
Differential| Compare equivalent realizations
Portability| Test cross-target behavior
Hybrid| Test cross-domain dependencies
Incremental| Verify correct invalidation
Parallel| Verify deterministic collection
Security| Verify unsafe or unauthorized operations are rejected
Artifact| Verify serialization and integrity
Scalability| Test increasing workloads
Resource exhaustion| Verify explicit failure
Regression| Preserve previously verified behavior
Compatibility| Verify old and new interfaces

32.1 Required fixtures

The compiler testing architecture must include representative fixtures:

minimal.zm
classical.zm
quantum.zm
hybrid.zm
hdl.zm
ai.zm
distributed.zm
networking.zm
cryptographic.zm
poco-reaf.zm

Cross-domain fixtures must include:

classical_quantum.zm
classical_hdl.zm
classical_ai.zm
quantum_ai.zm
quantum_distributed.zm
ai_hardware.zm
classical_quantum_hdl.zm
multi_domain.zm

32.2 Portability testing

A successful compiler invocation alone does not establish portability.

Tests must compare observable program behavior across compatible realizations.

The testing system must distinguish:

- Same semantics.
- Target-specific numerical variation permitted by specification.
- Nondeterministic behavior explicitly permitted by specification.
- Unsupported capability.
- Infeasible resource requirements.
- Actual compilation defects.

32.3 Scalability testing

Tests must progressively increase workload sizes.

They must test configurable budgets and actual exhaustion.

A small fixed test maximum must not be presented as proof of universal scalability.

---

33. Documentation architecture

The documentation directory owns compiler architectural documentation.

33.1 "documentation/ARCHITECTURE.md"

This document.

It owns:

- Compiler architecture.
- Architectural authorities.
- Module responsibilities.
- Pipeline boundaries.
- Integration rules.
- Scalability principles.
- Portability principles.
- Migration governance.
- Production requirements.

33.2 "documentation/PIPELINE_CONTRACT.md"

Must define the complete pipeline stage contract.

For every stage, document:

- Purpose.
- Inputs.
- Outputs.
- Dependencies.
- Preconditions.
- Postconditions.
- Error categories.
- Resource budgets.
- Cancellation.
- Determinism.
- Verification.
- Completion criteria.

33.3 "documentation/INTEGRATION_CONTRACT.md"

Must define the integration contracts between compiler and:

- Frontend.
- Semantic layer.
- ZUIR.
- Classical IR.
- Quantum IR.
- HDLIR.
- AI.
- Hybrid.
- Distributed.
- Backend.
- Runtime.
- Toolchain.

33.4 "documentation/PORTABILITY_CONTRACT.md"

Must define:

- Portable artifact semantics.
- Target-independent representations.
- Capability requirements.
- Target realization.
- Backend compatibility.
- Artifact versioning.
- Cross-target conformance.
- POCO-REAF guarantees and limitations.

These documents must not create competing normative language or semantic specifications.

---

34. Every-file completion and freeze contract

Every production compiler file must have a complete implementation contract before implementation begins.

The contract must identify:

1. Purpose.
2. Ownership.
3. Non-responsibilities.
4. Public types.
5. Public functions.
6. Public traits.
7. Dependencies.
8. Upstream interfaces.
9. Downstream consumers.
10. Integration paths.
11. Invariants.
12. Error behavior.
13. Configuration.
14. Resource-budget behavior.
15. Scalability.
16. Determinism.
17. Security.
18. Compatibility.
19. Unit tests.
20. Integration tests.
21. Acceptance criteria.
22. Freeze criteria.

34.1 Freeze requirements

A file may be declared frozen only when:

- Its complete implementation exists.
- Its dependencies are established.
- Its public API is complete.
- Its integration contracts are verified.
- Its tests pass.
- Its error behavior is complete.
- Its scalability requirements are satisfied.
- Its security boundaries are established.
- No unresolved production placeholder remains.
- Its completion criteria are objectively satisfied.

A frozen file may still change when a legitimate language or interface evolution requires it.

Such changes must be explicit, reviewed, versioned, and tested.

Freezing is not permission to preserve an incorrect implementation.

---

35. Implementation order

Implementation must follow dependency order.

The following sequence is mandatory unless a verified dependency requires an adjustment.

Phase 0 — Repository reconciliation

Before implementation:

1. Recursively inventory "src/compiler/".
2. Identify every existing file and nested directory.
3. Inspect every module declaration.
4. Inspect direct callers and dependencies.
5. Identify duplicate implementations.
6. Verify the current compiler root.
7. Reconcile "src/lib.rs".
8. Reconcile the legacy backend.
9. Read the grammar authority and specifications.
10. Identify current compilation failures.
11. Produce an authoritative dependency graph.

Deliverable: verified repository inventory and defect register.

Phase 1 — Independent contracts

Implement and freeze:

1. "configuration/budget.rs"
2. "configuration/target.rs"
3. "configuration/policy.rs"
4. "configuration/profile.rs"
5. "configuration/config.rs"
6. "configuration/validation.rs"
7. "configuration/mod.rs"

Also establish independent artifact metadata, diagnostic, provenance, and requirement/capability contracts using the existing authoritative owners.

Do not introduce duplicate semantic types.

Phase 2 — API and context

Implement and freeze:

1. "api/request.rs"
2. "api/response.rs"
3. "context/compilation_state.rs"
4. "context/compilation_context.rs"
5. "context/dependency_context.rs"
6. "context/provenance_context.rs"
7. "api/compiler.rs"
8. "api/compatibility.rs"
9. "api/mod.rs"
10. "context/mod.rs"

Phase 3 — Pipeline

Implement:

1. "pipeline/stage.rs"
2. "pipeline/stages.rs"
3. "pipeline/ordering.rs"
4. "pipeline/failure.rs"
5. "pipeline/recovery.rs"
6. "pipeline/execution.rs"
7. "pipeline/orchestration.rs"
8. "pipeline/mod.rs"

Phase 4 — IR integration

Implement:

1. "ir/input.rs"
2. "ir/adapters.rs"
3. "ir/verification.rs"
4. "ir/translation.rs"
5. "ir/lowering.rs"
6. "ir/compatibility.rs"
7. "ir/mod.rs"

Verify the integration against the actual ZUIR and domain IR implementations.

Phase 5 — Analysis and pass management

Complete:

- Analysis integration.
- Ownership integration.
- Effect integration.
- Capability matching interfaces.
- Requirement integration.
- Dependency analysis.
- Pass registry.
- Pass manager.
- Pass legality.
- Analysis preservation.
- Pass ordering.

Phase 6 — Optimization

Integrate the canonical optimizer and domain optimization providers.

Complete:

- Optimization planning.
- Cost models.
- Policies.
- Decisions.
- Profitability.
- Validation.

Phase 7 — Heterogeneous planning

Complete:

- Target discovery.
- Capability matching.
- Resource planning.
- Workload partitioning.
- Scheduling plans.
- Routing plans.
- Data movement.
- Feasibility reporting.

Verify classical, quantum, AI, HDL, and distributed integration.

Phase 8 — Lowering and backend integration

Complete:

- Legalization.
- Conversion.
- Decomposition.
- Outlining.
- Target lowering.
- Backend discovery.
- Backend registry integration.
- Backend selection.
- Backend invocation.

Verify real backend output.

Phase 9 — Artifacts and linking

Complete:

- Artifact contracts.
- Manifest.
- Metadata.
- Portability classification.
- Validation.
- Serialization.
- Linking.
- Relocation.
- Composition.

Phase 10 — Incremental and parallel compilation

Complete:

- Fingerprinting.
- Invalidation.
- Persistence.
- Dependency snapshots.
- Parallel scheduling.
- Worker execution.
- Deterministic result collection.

Phase 11 — Security and production validation

Complete:

- IR security verification.
- Backend security.
- Supply-chain verification.
- Artifact integrity.
- Security audit.
- Differential testing.
- Portability testing.
- Scalability testing.
- Regression testing.
- Fuzzing.

Phase 12 — Release qualification

The entire repository must pass the production quality gates.

---

36. Production quality gates

The compiler must use the repository's Rust baseline:

- Rust 1.97 or later.
- Rust 2021 edition.
- No unsafe Rust.

Required commands:

cargo fmt --all -- --check

cargo check --all-targets

cargo test --all-targets

cargo clippy --all-targets -- -D warnings

Additional requirements:

1. No arbitrary universal hardware ceilings.
2. No duplicate semantic authority.
3. No duplicate canonical IR.
4. No competing optimizer.
5. No competing backend registry.
6. No competing runtime.
7. No fake backend output.
8. No silent semantic fallback.
9. No successful compilation after a required stage was skipped.
10. No unverified portability claims.
11. No ignored production errors.
12. No unresolved production placeholders.
13. No invalid artifact presented as executable.
14. No unverified generated code.
15. No unsafe Rust.

The compiler root must also be tested through actual external crate consumers to verify its public API.

---

37. Production-readiness definition

The compiler is production-ready only when all of the following are demonstrated.

Area| Acceptance condition
Language integration| Grammar, parser, AST and semantic conformance
Semantic correctness| One authoritative validated program meaning
Ownership| Real ownership and borrowing integration
IR| Verified canonical representations
Transformations| Explicit legality and semantic preservation
Optimization| Validated optimization decisions
Hybrid computing| Correct cross-domain composition
Hardware planning| Capability and feasibility analysis
Backends| Real, validated artifact generation
Portability| Cross-target semantic conformance
Linking| Correct dependency and symbol resolution
Incremental| Sound cache invalidation
Parallel| Correct and deterministic execution where required
Security| Complete verification and evidence
Scalability| Resource-parametric processing
Diagnostics| Structured and actionable failures
Compatibility| Versioned public contracts
Testing| Unit, integration, differential and regression coverage
Build| All production quality gates pass

A directory structure, architecture document, or large number of source files is not proof of production readiness.

Evidence must come from working implementations, successful builds, tests, and actual target integrations.

---

38. Final architectural principles

The Zamani Universal Meta-Compiler must obey these permanent rules.

Rule 1 — One program meaning

The compiler must preserve one validated semantic meaning across different target realizations.

Rule 2 — No duplicate authority

The compiler integrates semantic, IR, optimization, backend, hardware, and runtime authorities rather than replacing them.

Rule 3 — No artificial universal capacity limits

Operational budgets are configurable and separate from language semantics and physical capabilities.

Rule 4 — Explicit feasibility

An impossible or unsupported realization must be reported honestly.

Rule 5 — Extensible architecture

New computational domains, target descriptions, passes, and backend providers must integrate through stable contracts.

Rule 6 — Verified transformations

Every transformation must preserve the required semantic properties.

Rule 7 — Portable artifacts

Portable representations must not be confused with target-specific executables.

Rule 8 — Runtime separation

The compiler plans and generates; the runtime allocates and executes.

Rule 9 — Safe Rust

No unsafe Rust is permitted.

Rule 10 — Integration before claims

A feature is not production-ready until its complete integration has been demonstrated.

---

39. Ultimate POCO-REAF architecture

The intended final system is:

                    ONE ZAMANI PROGRAM
                            |
                            v
                  ONE CANONICAL MEANING
                            |
                            v
                  VERIFIED PORTABLE IR
                            |
                            v
                 OPTIMIZED REPRESENTATIONS
                            |
                            v
               CAPABILITY-AWARE REALIZATION
                            |
             +--------------+--------------+
             |              |              |
             v              v              v
          CLASSICAL       QUANTUM          HDL
             |              |              |
             +--------------+--------------+
                            |
             +--------------+--------------+
             |              |              |
             v              v              v
             AI          DISTRIBUTED     ACCELERATORS
             |              |              |
             +--------------+--------------+
                            |
                            v
                  HYBRID EXECUTION BUNDLE
                            |
                            v
                  TARGET-SPECIFIC ARTIFACTS
                            |
                            v
                    RUNTIME EXECUTION
                            |
                            v
                 OBSERVABLE PROGRAM RESULTS

The same source semantics must remain valid across:

Atom
  |
  v
Embedded computing
  |
  v
Single processor
  |
  v
Multicore computing
  |
  v
GPU and accelerators
  |
  v
FPGA and ASIC
  |
  v
Quantum computing
  |
  v
Hybrid computing
  |
  v
Clusters
  |
  v
Supercomputers
  |
  v
Distributed infrastructure
  |
  v
Cloud and edge environments
  |
  v
Future computing architectures

These are different realization environments, not different programming languages.

The developer should not need to rewrite a program merely because the available hardware changes.

The compiler must determine which valid realization is possible, preserve the program's semantic contracts, and explicitly report when no valid realization exists.

The ultimate Zamani compiler principle is:

«One program. One semantic meaning. One portable compilation foundation. Multiple verified realizations. Any compatible hardware. Any scale supported by available resources. Forever, through controlled compatibility and evolution.»

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.