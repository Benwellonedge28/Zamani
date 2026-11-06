Zamani Compiler — Portability Contract

Canonical path: "src/compiler/documentation/PORTABILITY_CONTRACT.md"

Repository: "github.com/Benwellonedge28/Zamani"

Subsystem: "src/compiler/"

Document type: Normative compiler architecture and integration contract

Status: Proposed production contract; implementation conformance must be demonstrated independently.

Rust baseline: Rust 1.97 or later, edition 2021. The grammar architecture specifies Rust 1.97.1 or later; the repository must maintain a consistent minimum-version policy.

Safety: Safe Rust only. No "unsafe".

Primary objective:

"Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)"

Scalability objective: Support the smallest meaningful computation through arbitrarily large finite workloads, constrained by actual available resources, program semantics, implementation capabilities and explicit operational policies, never by arbitrary universal hardware ceilings.

---

1. Purpose

This document establishes the production portability contract for the Zamani compiler.

Its purpose is to ensure that programs written in Zamani can preserve their declared semantic meaning while being compiled, transformed, optimized, packaged and realized across different computing environments.

The compiler must support a unified programming model encompassing:

- Classical computing.
- Systems programming.
- Parallel and concurrent computing.
- Distributed computing.
- Scientific and numerical computing.
- Tensor computation.
- Artificial intelligence.
- Machine learning and deep learning.
- Symbolic and probabilistic computation.
- Quantum computing.
- Hybrid quantum-classical computing.
- Quantum error correction integration.
- Hardware description and hardware/software co-design.
- FPGA and ASIC realization.
- GPU and other accelerators.
- Neuromorphic computing.
- Photonic and optical computing.
- Networking.
- Cryptographic computation.
- Embedded and edge computing.
- Cloud and high-performance computing.
- Future computational architectures.

These must remain domains of one programming language.

The compiler must not introduce separate, incompatible language semantics for different hardware classes.

The fundamental rule is:

Zamani source code describes computation and its semantic requirements. The compiler and downstream execution infrastructure determine how that computation can be realized.

The compiler must preserve that distinction throughout its entire lifecycle.

2. Normative language

The following terminology is normative:

- MUST: Mandatory.
- MUST NOT: Prohibited.
- SHOULD: Recommended unless a documented reason exists otherwise.
- SHOULD NOT: Discouraged.
- MAY: Optional.
- CAN: Describes a capability rather than an obligation.

A compiler implementation that violates a MUST or MUST NOT requirement is not conformant with this contract.

3. Architectural authorities

This document is subordinate to the repository's established language and architectural authorities.

The authority relationship is:

Authority| Responsibility
"grammar/DESIGN.md"| Grammar architecture and ownership
"grammar/specification/"| Normative language semantics
"grammar/spec/portability.md"| Formal portability semantics
"grammar/spec/resources.md"| Resource semantics
"grammar/specification/poco-reaf.md"| POCO-REAF language objective
"grammar/specification/scalability-model.md"| Language scalability semantics
"grammar/validation/portability.md"| Grammar and semantic portability validation
"grammar/validation/semantic-boundaries.md"| Semantic ownership boundaries
"grammar/Zamani.g4"| Canonical ANTLR composition root
"grammar/antlr/ZamaniLexer.g4"| Canonical lexer boundary
"grammar/antlr/ZamaniParser.g4"| Canonical parser boundary

The compiler consumes these contracts.

It MUST NOT independently redefine their language semantics.

When a conflict is discovered, the compiler implementation must not silently invent a resolution. The responsible specification authority must resolve the conflict through a reviewed and versioned change.

4. Ownership

4.1 What this document owns

This document owns the compiler-facing portability contract, including:

1. Portable compilation semantics.
2. Target-independent compilation.
3. Portable intermediate representations.
4. Portability-preserving transformations.
5. Portable compilation artifacts.
6. Target-specific realization boundaries.
7. Portability classification integration.
8. Capability and requirement preservation.
9. Cross-target semantic equivalence.
10. Portability evidence requirements.
11. Compilation compatibility.
12. Portable artifact versioning requirements.
13. Portability diagnostics.
14. Portability conformance testing.
15. Future-backend compatibility requirements.
16. POCO-REAF compiler integration.

4.2 What this document does not own

It does not own:

- Source grammar.
- Lexical rules.
- Parsing.
- AST definitions.
- Source-language semantics.
- Type-system definitions.
- Ownership semantics.
- Resource semantics.
- Capability semantics.
- Canonical IR definitions.
- Optimization algorithms.
- Physical hardware discovery.
- Device allocation.
- Physical placement.
- Runtime scheduling.
- Quantum routing.
- Quantum error correction algorithms.
- Noise modeling.
- HAL implementation.
- Backend code generation.
- Physical execution.

Those responsibilities remain with their existing repository authorities.

5. Fundamental portability definition

Portability is the preservation of the declared semantic contract of a program while its implementation realization changes.

Let:

- (P) represent the validated semantic program.
- (E) represent an execution environment.
- (R) represent a realization strategy.
- (O) represent observable program behavior.

A realization is:

[
\operatorname{Realize}(P,E,R)\rightarrow O
]

Two realizations are portable equivalents when they preserve the required observable semantics:

[
\operatorname{Realize}(P,E_1,R_1)
\equiv_{\mathrm{semantic}}
\operatorname{Realize}(P,E_2,R_2)
]

The physical implementations may differ substantially.

For example:

- One realization may use a CPU.
- Another may use a GPU.
- Another may use an FPGA.
- Another may use a quantum processor.
- Another may distribute the workload across multiple machines.
- Another may use an architecture introduced in the future.

The implementation may change.

The required program semantics must not change.

The equivalence relation must account for the language's actual observable-behavior contract, including permitted nondeterminism, floating-point rules, concurrency, effects, timing requirements and quantum measurement behavior.

The compiler must not assume that all computational domains have identical execution semantics.

6. POCO-REAF interpretation

POCO-REAF means:

Program Once → Compile Once → Run Everywhere, Anywhere, Forever.

The compiler must make this objective technically meaningful.

It requires:

1. One canonical source-language meaning.
2. Target-independent semantic representation.
3. Stable canonical IR contracts.
4. Explicit capability requirements.
5. Explicit resource requirements.
6. Portable transformations.
7. Versioned compilation artifacts.
8. Extensible backend realization.
9. Target-independent artifact metadata.
10. Reproducible compilation.
11. Cross-target conformance.
12. Preservation of observable semantics.

6.1 Compile-once boundary

The compiler should support the following architecture:

                 ZAMANI SOURCE
                       |
                       v
                LEXICAL ANALYSIS
                       |
                       v
                    PARSING
                       |
                       v
                 CANONICAL AST
                       |
                       v
               SEMANTIC ANALYSIS
                       |
                       v
             VALIDATED SEMANTIC MODEL
                       |
                       v
                CANONICAL IR
                       |
                       v
            PORTABLE COMPILATION ARTIFACT
                       |
              +--------+---------+
              |        |         |
              v        v         v
             CPU      GPU       FPGA
              |        |         |
              +--------+---------+
                       |
                       v
               OTHER TARGETS
                       |
                       v
                FUTURE TARGETS

A portable compilation artifact must be reusable for subsequent target realization without requiring source rewriting.

However, the compiler must distinguish a portable artifact from a target-specific executable.

A portable IR package is not automatically an executable binary.

A target-specific backend may need to perform lowering, legalization, specialization or code generation.

This does not invalidate POCO-REAF.

The objective is to avoid recompiling the source-language meaning unnecessarily, not to promise that one physical machine-code binary can execute natively on every architecture.

6.2 What POCO-REAF does not promise

POCO-REAF does not promise:

- Infinite physical memory.
- Infinite processors.
- Infinite processing speed.
- Universal hardware capabilities.
- Identical performance across machines.
- Identical physical execution strategies.
- Guaranteed availability of every requested accelerator.
- Automatic realization of unsupported operations.

It promises an architecture that does not unnecessarily bind program meaning to today's physical machines.

7. Portability architecture

The compiler must implement or integrate the following logical stages.

SOURCE
  |
  v
FRONTEND
  |
  v
SEMANTIC VALIDATION
  |
  v
CANONICAL REPRESENTATIONS
  |
  v
PORTABILITY ANALYSIS
  |
  v
IR VERIFICATION
  |
  v
PORTABLE TRANSFORMATIONS
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
HETEROGENEOUS PLANNING
  |
  v
DOMAIN-SPECIFIC LOWERING
  |
  v
BACKEND REALIZATION
  |
  v
ARTIFACT VALIDATION
  |
  v
LINKING AND PACKAGING
  |
  v
DEPLOYMENT
  |
  v
RUNTIME

Each stage has a distinct responsibility.

Stage| Responsibility
Frontend| Produce canonical AST
Semantic analysis| Establish program meaning
Canonical IR| Represent computation
Portability analysis| Establish portability properties and dependencies
Verification| Validate semantic and representation invariants
Transformation| Preserve meaning while changing representation
Optimization| Improve implementation quality without changing required behavior
Capability analysis| Determine required and available capabilities
Feasibility analysis| Determine whether a realization satisfies requirements
Planning| Produce abstract realization plans
Lowering| Transform representations for a supported target
Backend| Produce target-specific outputs
Artifact validation| Verify artifact structure and integrity
Linking| Resolve dependencies and composition
Runtime| Execute and manage actual resources

The compiler must not collapse these responsibilities into one large orchestration function.

8. Repository integration contracts

The following existing files and subsystems are authoritative integration points.

8.1 Compiler composition root

File: "src/compiler/mod.rs"

Responsibilities:

- Expose the compiler's canonical module hierarchy.
- Establish compiler-level integration boundaries.
- Preserve existing public APIs.
- Coordinate compilation without duplicating domain authorities.
- Integrate artifact portability.
- Expose the compiler's portability-related contracts through the appropriate modules.

Required integration:

pub mod artifacts;

The current compiler root must explicitly expose the existing artifact subsystem.

The integration must use:

crate::compiler::artifacts

as the canonical compiler artifact boundary.

Do not create a second artifact module or portability implementation.

The compiler root must remain independent of particular hardware vendors.

8.2 Artifact portability

File: "src/compiler/artifacts/portability.rs"

This file already implements:

- Portability authority identifiers.
- Evidence references.
- Portability declarations.
- Portability classifications.
- Artifact portability assessments.
- Manifest portability assessments.
- Portability policies.
- Structured errors.

It is the canonical compiler artifact portability implementation.

Required extensions and integration:

1. Preserve its existing public API unless a reviewed compatibility migration is approved.
2. Ensure all artifact assessments distinguish declaration from verification.
3. Require appropriate evidence for verified portability.
4. Preserve explicit target dependencies.
5. Support open-ended computing domains.
6. Support future architecture identifiers.
7. Preserve resource and capability dependencies.
8. Distinguish unknown evidence from negative evidence.
9. Integrate with artifact validation.
10. Integrate with compiler-level portability diagnostics.
11. Support target-independent and target-specific artifact relationships.
12. Preserve deterministic assessment behavior.
13. Ensure policy rejection never silently converts an artifact into another portability class.

Important:

This module records evidence and classifications. It does not independently authenticate an external attestation or prove that a backend is correct.

Those responsibilities belong to their authoritative verification and trust systems.

8.3 Artifact model

File: "src/compiler/artifacts/artifact.rs"

Owns:

- Artifact identity.
- Artifact descriptors.
- Artifact kinds.
- Artifact formats.
- Artifact payloads.
- Payload references.
- Content digests.
- Target references.
- Producer identity.
- Provenance.

Required portability integration:

An artifact must be able to describe whether it represents:

- Portable canonical computation.
- Portable intermediate representation.
- Domain-specific intermediate representation.
- Target-specific intermediate representation.
- Target-specific executable.
- Hardware configuration.
- Deployment package.
- Hybrid deployment bundle.
- Other registered artifact categories.

These must be extensible identifiers, not a permanently closed inventory of hardware categories.

Artifact identity must not be confused with target identity.

Content integrity must not be confused with semantic correctness.

A valid digest does not prove portability.

8.4 Artifact manifests

File: "src/compiler/artifacts/manifest.rs"

Owns:

- Artifact collections.
- Manifest identity.
- Manifest versions.
- Dependency relationships.
- Artifact roots.
- Manifest structural validation.

Required integration:

A portable compilation manifest must be capable of describing:

- Canonical IR dependencies.
- Domain IR dependencies.
- Compiler compatibility requirements.
- Semantic-model compatibility.
- Capability requirements.
- Resource requirements.
- Artifact provenance.
- Verification evidence references.
- Optional target-specific realizations.
- External payload references.
- Cross-domain dependencies.

A manifest must not claim that every referenced artifact is executable.

The distinction between portable and realized artifacts must remain explicit.

8.5 Artifact metadata

File: "src/compiler/artifacts/metadata.rs"

Owns:

- Namespaced metadata.
- Metadata access.
- Metadata validation.
- Metadata merging.
- Configurable metadata policies.

Required integration:

Portable metadata must not silently encode physical target assumptions.

Target-specific metadata must be explicitly identifiable.

Metadata must distinguish:

- Compiler information.
- Language information.
- Semantic information.
- IR information.
- Backend information.
- Target information.
- Portability information.
- Provenance information.
- Verification information.

Unknown metadata namespaces must remain extensible subject to explicit validation policy.

8.6 Artifact validation

File: "src/compiler/artifacts/validation.rs"

Owns artifact and manifest validation.

The existing implementation distinguishes:

- Valid.
- Invalid.
- Incomplete.

These distinctions are mandatory for portability.

Required validation sequence:

DECODE
  |
  v
STRUCTURAL VALIDATION
  |
  v
CONTENT INTEGRITY
  |
  v
PROVENANCE VALIDATION
  |
  v
IR VERIFICATION
  |
  v
PORTABILITY ASSESSMENT
  |
  v
SECURITY VALIDATION
  |
  v
BACKEND / TARGET COMPATIBILITY

Only applicable checks should be required for a particular artifact kind.

A deferred check must not be represented as successful verification.

An external payload that has not been retrieved and verified must remain explicitly unresolved.

A successfully decoded artifact must not automatically be considered safe to execute.

8.7 Artifact serialization

File: "src/compiler/artifacts/serialization.rs"

Owns the stable wire representation of artifacts and manifests.

Required integration:

- Preserve serialization format versioning.
- Preserve deterministic encoding.
- Preserve strict decoding.
- Preserve configurable input and output budgets.
- Preserve external payload references.
- Reject unsupported format versions.
- Validate decoded structures.
- Maintain forward compatibility through explicit schema evolution.

The serialization format version must remain independent of:

- Zamani language version.
- Compiler version.
- Semantic-model version.
- ZUIR version.
- Domain IR versions.
- Backend version.
- Runtime version.

Large payloads should use external payload references when appropriate rather than requiring all content to be loaded into compiler memory.

8.8 Existing hybrid compiler

File: "src/compiler/hybrid_pipeline.rs"

Required integration:

The hybrid compilation path must preserve portability across domain boundaries.

It must support:

- Classical-to-quantum dependencies.
- Quantum-to-classical dependencies.
- AI-to-classical dependencies.
- AI-to-quantum dependencies.
- HDL-to-software dependencies.
- Distributed computation.
- Cross-domain data exchange.
- Synchronization requirements.
- Resource dependencies.
- Capability requirements.
- Domain-specific verification.

The compiler must not invent a second hybrid semantic model.

The authoritative hybrid subsystem and canonical IR implementations remain responsible for their respective representations.

8.9 Hybrid profiles

File: "src/compiler/hybrid_profiles.rs"

Required integration:

Profiles must not become hidden hardware ceilings.

They may describe:

- Optimization objectives.
- Compilation characteristics.
- Execution preferences.
- Expected workload properties.
- Measured performance.
- Energy characteristics.
- Domain-specific policies.

Profiles must be versioned and extensible.

A profile is not proof of target feasibility.

8.10 Hardware partitioning

File: "src/compiler/hardware_partitioner.rs"

Required integration:

The partitioner must use explicit capability and requirement information.

It must support heterogeneous partitioning rather than assume that one device category represents all computation.

It must consider:

- Operation compatibility.
- Data dependencies.
- Communication costs.
- Memory locality.
- Synchronization.
- Resource requirements.
- Capability requirements.
- Latency.
- Throughput.
- Energy.
- Feasibility.
- Alternative realizations.

It must return explicit infeasibility information.

It must not allocate physical devices or assume that a planned device is available.

Physical allocation belongs to the appropriate runtime and resource-management authorities.

8.11 Compiler linking

File: "src/compiler/linker.rs"

Required integration:

- Portable module composition.
- Symbol compatibility.
- Domain dependency validation.
- Target-specific relocation.
- ABI compatibility.
- Artifact dependency resolution.
- Hybrid bundle composition.
- Version compatibility.
- Explicit unsupported-linkage diagnostics.

Linking must not erase portability metadata or provenance.

8.12 Incremental compilation

File: "src/compiler/incremental.rs"

Required integration:

Portable cache identities must distinguish semantic inputs from target-specific realization inputs.

A portable compilation cache key should include the relevant:

- Source identity.
- Language version.
- Semantic contract version.
- Canonical IR version.
- Compiler transformation configuration.
- Dependency fingerprints.
- Relevant optimization policy.

A target-specific realization cache must additionally include the relevant:

- Target descriptor version.
- Backend identity and version.
- ABI.
- Target features.
- Lowering configuration.

A physical device identifier must not unnecessarily contaminate a target-independent cache key.

Cache reuse must never bypass required verification.

8.13 Compiler diagnostics

File: "src/compiler/diagnostics.rs"

Required integration:

Portability failures must be structured and machine-readable.

Diagnostics must distinguish:

- Semantic invalidity.
- Missing capability.
- Insufficient resources.
- Target incompatibility.
- Unsupported lowering.
- Missing portability evidence.
- Invalid portability declaration.
- Artifact incompatibility.
- Unsupported serialization version.
- Security rejection.
- Incomplete validation.
- Operational resource exhaustion.

Diagnostics must preserve source locations and provenance where available.

8.14 LLVM and WebAssembly

Existing files:

- "src/compiler/llvm_backend.rs"
- "src/compiler/wasm_backend.rs"
- "src/compiler/wasm_cfg.rs"

Required integration:

These must remain backend-specific implementations.

They must consume the compiler's established IR and portability contracts.

They must not define competing portable semantic models.

Their outputs must accurately identify whether they are:

- Intermediate representations.
- Object files.
- Executables.
- Libraries.
- WebAssembly modules.
- Other explicitly identified formats.

The compiler must not claim that a generated source file or manifest is an executable artifact.

9. Canonical IR portability

The compiler must integrate existing IR authorities.

Representation| Canonical owner
Universal IR| "src/zuir/"
Classical IR| "src/classical/"
Quantum IR| "src/quantum/ir/"
HDL/hardware IR| "src/hdl/hdlir/"
AI computation| "src/ai/"
Hybrid composition| "src/hybrid/"
Distributed computation| "src/distributed/"

The compiler must not introduce competing representations.

9.1 Universal IR

ZUIR is the universal representation boundary.

It must preserve the semantics and relationships necessary for subsequent realization.

The compiler consumes and coordinates ZUIR.

It must not create another universal IR.

9.2 Classical IR

Classical computation must use the existing classical IR authority.

CPU, GPU and accelerator specialization must not redefine classical source semantics.

9.3 Quantum IR

"src/quantum/ir/" remains the canonical quantum IR boundary.

The compiler must preserve:

- Logical quantum state semantics.
- Quantum operation semantics.
- Measurement semantics.
- Classical control dependencies.
- Quantum resource requirements.
- Required ordering.
- Applicable effects.

Physical qubit mapping, routing, scheduling, QEC and noise handling remain downstream.

The compiler must not turn physical qubit identifiers into universal source-level semantics.

9.4 HDL and hardware IR

Hardware intent must remain distinct from physical implementation.

The compiler must preserve the canonical HDL/hardware representation.

It must not impose universal limits on:

- Signal counts.
- Register counts.
- Port counts.
- Hardware widths.
- Pipeline depth.
- FPGA resources.
- ASIC resources.
- Physical topology.

Program-defined values and explicit target constraints remain valid.

9.5 AI and tensor IR

AI and tensor computations must retain their canonical semantic and IR representations.

The compiler must not introduce arbitrary universal limits on tensor rank, model size, dimensions, parameter counts or accelerator counts.

Symbolic and dynamic shapes must be preserved where supported by the semantic and IR contracts.

9.6 Hybrid and distributed representations

Hybrid and distributed computation must preserve:

- Cross-domain dependencies.
- Data exchange.
- Synchronization.
- Ordering.
- Effects.
- Resource requirements.
- Capability requirements.
- Failure contracts.
- Provenance.

The compiler must not assume that all domain operations execute on one physical machine.

10. Portability classifications

The compiler must distinguish different kinds of portability dependencies.

At minimum, the overall portability model must be capable of expressing:

Classification| Meaning
Portable| No unnecessary target-specific dependency
Resource-dependent| Depends on abstract resource availability
Capability-dependent| Requires a supported capability
Scale-dependent| Requirements vary with workload
Target-constrained| Valid under an explicit target constraint
Target-specific| Explicitly depends on a particular realization
Implementation-defined| Chosen by a downstream implementation
Non-portable| Cannot preserve the requested portability boundary
Unknown| Insufficient evidence to establish the classification

These are conceptual semantic categories.

Their representation must remain consistent with the existing "PortabilityClass" implementation in "src/compiler/artifacts/portability.rs".

Do not introduce a second competing portability enum.

An unknown classification must never be interpreted as verified portability.

11. Requirements and capabilities

The compiler must preserve the distinction between:

Concept| Meaning
Requirement| What the program needs
Capability| What an environment can provide
Constraint| What a realization must obey
Preference| What is desirable
Hint| Optional optimization guidance
Budget| An operational allowance
Feasibility| Whether a realization satisfies its obligations

Example:

requires capability("quantum.measurement");

requires memory >= required_memory;

prefers capability("tensor.acceleration");

constrains latency <= required_latency;

These are conceptual examples. Their exact syntax and semantics are governed by the canonical grammar and semantic specifications.

The compiler must not collapse all of these into one hardware-target enumeration.

Capability identifiers must be extensible.

A future capability must not require redesigning the compiler's central portability model.

12. Scalability requirements

Zamani must scale from the smallest supported computation to arbitrarily large finite realizations.

The architecture must not introduce universal finite ceilings.

The prohibition includes:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_QPUS
MAX_NODES
MAX_DEVICES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_WORKERS
MAX_MODULES
MAX_OPERATIONS

Equivalent hidden restrictions are equally prohibited.

12.1 Distinguish implementation budgets from language limits

Operational budgets are permitted.

Examples:

- Maximum compilation time selected by the user.
- Maximum memory budget selected by the build environment.
- Maximum parallel workers selected by policy.
- Maximum artifact output size selected by the caller.
- Maximum cache storage selected by configuration.

These are operational safeguards.

They must not become permanent source-language restrictions.

The compiler must distinguish:

1. Language invalidity.
2. Semantic invalidity.
3. Target incompatibility.
4. Resource infeasibility.
5. Configured budget exhaustion.
6. Actual implementation resource exhaustion.

12.2 Infinity

Infinity means the absence of artificial language-level capacity ceilings.

It does not mean infinite physical resources.

The compiler must support growth until actual constraints prevent further work, and must report those constraints accurately.

13. Portable transformation requirements

Every compiler transformation affecting a portable representation must preserve the relevant semantic contract.

A transformation must define:

1. Input representation.
2. Output representation.
3. Preconditions.
4. Required analyses.
5. Semantic-preservation obligations.
6. Effect-preservation obligations.
7. Capability implications.
8. Resource implications.
9. Provenance.
10. Invalidated analyses.
11. Determinism properties.
12. Failure behavior.
13. Verification procedure.

Examples include:

- Constant folding.
- Dead-code elimination.
- Loop transformations.
- Instruction fusion.
- Vectorization.
- Parallelization.
- Tensor optimization.
- Quantum decomposition.
- Classical/quantum boundary transformation.
- Hardware-specific lowering.
- Distributed decomposition.

A transformation must not silently:

- Remove required operations.
- Introduce undeclared effects.
- Change measurement semantics.
- Violate memory ordering.
- Change required numerical behavior.
- Discard capability requirements.
- Discard resource constraints.
- Convert an exact operation into an approximation without permission.
- Introduce a target dependency into a portable artifact without recording it.

If semantic preservation cannot be established, the compiler must reject the transformation or mark the result as insufficiently verified.

14. Capability and feasibility integration

The compiler must separate capability discovery from feasibility evaluation.

The conceptual flow is:

PROGRAM REQUIREMENTS
        |
        v
CANONICAL CAPABILITY REQUIREMENTS
        |
        v
TARGET CAPABILITY DESCRIPTORS
        |
        v
RESOURCE AVAILABILITY
        |
        v
CONSTRAINT EVALUATION
        |
        v
FEASIBILITY REPORT
        |
        v
REALIZATION PLAN

A capability descriptor must not automatically imply that a target is currently available.

A resource descriptor must not automatically imply that the resource can be allocated.

A feasible plan must not automatically imply successful physical execution.

These are distinct guarantees.

The compiler must be able to report multiple feasible alternatives when the planning architecture supports them.

It must report explicit infeasibility when no acceptable realization can be established.

15. Target-independent and target-specific artifacts

The compiler must distinguish two fundamental artifact categories.

15.1 Portable compilation artifact

A portable artifact should preserve:

- Canonical IR.
- Domain IR dependencies.
- Semantic contracts.
- Effects.
- Resource requirements.
- Capability requirements.
- Portability classification.
- Dependency information.
- Provenance.
- Verification evidence.
- Compatibility metadata.
- Version information.

It must not unnecessarily depend on a specific physical device.

15.2 Target-specific realization artifact

A target-specific artifact may contain:

- Target-specific IR.
- Native object files.
- Executables.
- GPU binaries.
- FPGA bitstreams.
- ASIC synthesis outputs.
- Quantum execution packages.
- WebAssembly modules.
- Device-specific configuration.
- Deployment bundles.

It must explicitly identify its target and compatibility requirements.

A target-specific artifact must not be presented as universally executable.

15.3 Artifact relationship

The architecture must preserve the derivation:

PORTABLE ARTIFACT
        |
        v
TARGET REALIZATION
        |
        v
TARGET-SPECIFIC ARTIFACT

The realization must retain provenance linking it to its portable input.

16. Portable artifact validation

The existing artifact validation system must remain authoritative.

The compiler must integrate the following independent validation responsibilities:

Validation| Authority
Artifact structure| "artifacts/validation.rs"
Payload integrity| Artifact validation and digest implementation
Manifest structure| "artifacts/manifest.rs"
Metadata policy| "artifacts/metadata.rs"
Portability assessment| "artifacts/portability.rs"
Wire-format validation| "artifacts/serialization.rs"
Semantic correctness| Frontend semantic authority
IR correctness| Canonical IR verifiers
Security| Existing security subsystem
Backend compatibility| Backend and target contracts
Execution feasibility| Appropriate planning/runtime authorities

Validation must distinguish:

- Valid.
- Invalid.
- Incomplete.

A missing verification step must never be treated as a successful verification.

17. Serialization and compatibility

Portable artifacts must remain usable across supported compiler versions and future target implementations.

The compatibility system must independently track:

- Zamani language version.
- Grammar version.
- Semantic-model version.
- Compiler API version.
- Canonical IR version.
- Domain IR versions.
- Artifact model version.
- Manifest schema version.
- Serialization format version.
- Backend interface version.
- Target ABI.
- Runtime compatibility version.

These versions must not be collapsed into one number.

An artifact must provide sufficient information for a consumer to determine compatibility.

Unsupported versions must produce explicit diagnostics.

Incompatible formats must not be silently interpreted as compatible.

18. Deterministic and reproducible compilation

Where deterministic compilation is required, identical semantic inputs and compilation configuration must produce reproducible outputs.

The reproducibility contract must consider:

- Source content.
- Language version.
- Semantic version.
- IR version.
- Compiler version.
- Transformation configuration.
- Optimization policy.
- Backend version.
- Relevant target descriptors.
- Dependency versions.
- Serialization format.

Physical timestamps, random identifiers, device IDs and environmental information must not accidentally contaminate target-independent artifact identities.

When a compilation operation is intentionally nondeterministic, the compiler must record the relevant nondeterminism and provenance.

19. Security and trust boundaries

Portability is not a security guarantee.

A portable artifact may still be:

- Malformed.
- Corrupted.
- Incompatible.
- Untrusted.
- Unsafe to execute.
- Produced by an untrusted compiler.
- Associated with unverified external content.

The compiler must not infer security from portability.

Required security integration includes:

1. Artifact integrity.
2. Provenance verification.
3. External dependency validation.
4. Backend trust policy.
5. Compilation policy.
6. Supply-chain verification.
7. Execution authorization.
8. Target-specific security constraints.

No untrusted artifact should become executable merely because its serialization succeeded.

All Rust implementation must remain safe Rust.

20. Failure and recovery contract

A portability-aware compilation process must have explicit failure categories.

Failure| Required behavior
Invalid source| Frontend diagnostic
Invalid semantics| Semantic diagnostic
Invalid IR| IR verification failure
Missing capability| Capability diagnostic
Insufficient resources| Resource feasibility diagnostic
Incompatible target| Target compatibility diagnostic
Unsupported lowering| Backend diagnostic
Invalid artifact| Artifact validation failure
Incomplete evidence| Incomplete validation
Unsupported serialization version| Compatibility diagnostic
Resource exhaustion| Explicit operational failure
Security rejection| Security diagnostic

Fallbacks are permitted only when they preserve the required semantics and satisfy applicable policies.

The compiler must not silently convert an unsupported operation into a different operation.

An approximation must be explicitly permitted by the semantic contract.

A failed realization must not be reported as successful compilation.

21. Repository-wide integration map

The following integrations are mandatory.

Repository path| Portability responsibility
"src/frontend/"| Canonical AST and semantic input
"src/frontend/semantic/"| Validated source meaning and semantic portability
"src/semantic.rs"| Legacy compatibility where still required
"src/zuir/"| Canonical universal representation
"src/classical/"| Classical representation and lowering
"src/quantum/ir/"| Canonical quantum representation
"src/hdl/hdlir/"| Canonical hardware representation
"src/ai/"| AI computation
"src/hybrid/"| Cross-domain composition
"src/distributed/"| Distributed computation semantics
"src/backend/"| Target-specific code generation
"src/runtime/"| Resource acquisition and execution
"src/optimizer/"| Canonical optimization
"src/ir_verify.rs"| IR verification integration
"src/source_map/"| Source mapping and diagnostics
"src/toolchain/"| Compilation, packaging and deployment

The compiler must coordinate these systems without taking over their ownership.

22. Grammar integration contract

The compiler's portability implementation must remain consistent with the actual grammar architecture.

The canonical source composition is:

grammar/Zamani.g4
        |
        +---- ZamaniParser.g4
        |
        +---- ZamaniLexer.g4

The root grammar owns complete-program composition.

The subordinate parser and lexer hierarchies own their respective language rules.

Portability-specific source syntax is owned by:

"grammar/resources/portability.g4"

The formal portability semantics are owned by:

"grammar/spec/portability.md"

The human-readable normative specification is maintained under:

"grammar/specification/portability.md"

The grammar validation contract is:

"grammar/validation/portability.md"

22.1 Grammar responsibilities

The compiler must consume the parsed and semantically validated representation.

It must not:

- Parse portability syntax independently.
- Redefine portability expressions.
- Create another expression language.
- Introduce a competing resource model.
- Perform hardware discovery during parsing.
- Perform resource allocation during parsing.
- Treat grammar acceptance as proof of target feasibility.

22.2 Grammar-to-compiler traceability

Every stable portability-sensitive language construct must have a traceable mapping:

GRAMMAR RULE
    |
    v
AST NODE
    |
    v
SEMANTIC CONTRACT
    |
    v
CANONICAL IR MAPPING
    |
    v
COMPILER PORTABILITY ANALYSIS
    |
    v
ARTIFACT METADATA / EVIDENCE
    |
    v
TARGET REALIZATION

A construct accepted by the grammar but discarded before IR is an incomplete implementation.

A construct accepted by the parser must not automatically be advertised as production-supported.

23. Required compiler directory integration

The compiler should organize portability responsibilities without creating duplicate authorities.

The existing artifact subsystem must be reused.

Recommended organization:

src/compiler/
|
+-- mod.rs
|
+-- documentation/
|   |
|   +-- ARCHITECTURE.md
|   +-- PIPELINE_CONTRACT.md
|   +-- INTEGRATION_CONTRACT.md
|   +-- PORTABILITY_CONTRACT.md
|
+-- artifacts/
|   |
|   +-- mod.rs
|   +-- artifact.rs
|   +-- manifest.rs
|   +-- metadata.rs
|   +-- portability.rs
|   +-- validation.rs
|   +-- serialization.rs
|
+-- analysis/
|   |
|   +-- portability.rs
|   +-- capabilities.rs
|   +-- requirements.rs
|   +-- feasibility.rs
|
+-- ir/
|   |
|   +-- portability.rs
|   +-- verification.rs
|   +-- adapters.rs
|
+-- planning/
|   |
|   +-- target_discovery.rs
|   +-- capability_matching.rs
|   +-- resource_planning.rs
|   +-- feasibility.rs
|
+-- backends/
|   |
|   +-- compatibility.rs
|   +-- selection.rs
|   +-- invocation.rs
|
+-- testing/
    |
    +-- portability.rs
    +-- cross_target.rs
    +-- semantic_equivalence.rs
    +-- scalability.rs

This is the desired integration structure, not a claim that every proposed file already exists.

The existing files must be inspected before creating new ones.

In particular:

- "artifacts/portability.rs" remains the canonical artifact portability implementation.
- A future "analysis/portability.rs" may analyze semantic and IR portability, but must delegate artifact classification to the existing artifact subsystem.
- A future "ir/portability.rs" may verify representation portability, but must not define another IR.
- A future "planning/feasibility.rs" must not allocate devices.
- A future "backends/compatibility.rs" must integrate with "src/backend/".

24. Portability analysis contract

The compiler must analyze portability before target-specific lowering.

The analysis should identify:

1. Semantic target dependencies.
2. Resource dependencies.
3. Capability dependencies.
4. Explicit target constraints.
5. Non-portable operations.
6. Domain interoperability dependencies.
7. Required adaptations.
8. Unsupported transformations.
9. Evidence requirements.
10. Potential realization alternatives.

The analysis must produce structured results.

It must not mutate the canonical semantic model merely to make a target appear compatible.

Portability analysis must distinguish:

- Proven portability.
- Declared portability.
- Conditional portability.
- Unknown portability.
- Target-specific behavior.
- Failed portability verification.

The precise representation must reuse the existing artifact portability contract where the subject is an artifact.

25. Cross-domain portability

The compiler must preserve the semantics of all supported computing domains.

25.1 Classical

The same classical semantics must be realizable on compatible processors and accelerators.

25.2 Quantum

Logical quantum semantics must remain distinct from physical mapping.

"quantum::ir" remains canonical.

Routing, scheduling, QEC and ZQN must preserve the applicable logical contract.

25.3 Hybrid

Classical and quantum operations must preserve dependencies, synchronization and data exchange.

25.4 AI

AI computation must preserve model, tensor, numerical and execution contracts.

25.5 HDL

Hardware intent must remain distinct from a particular physical implementation.

25.6 Distributed

Distributed semantics must remain independent of a permanently fixed number of nodes.

25.7 Networking and cryptography

Network protocols, cryptographic semantics, security requirements and interoperability contracts must survive realization changes.

25.8 Future domains

A new computational domain must be able to participate through registered semantic and IR contracts.

The compiler must not require changes to its fundamental portability model merely because a new domain or architecture appears.

26. Future hardware and backend extensibility

The compiler must support open-ended backend registration.

The architecture must not depend on a closed list of:

- Processor manufacturers.
- GPU vendors.
- Quantum hardware vendors.
- FPGA vendors.
- Accelerator models.
- Device architectures.
- Execution providers.

Each backend must establish:

1. Supported IR representations.
2. Supported operations.
3. Supported capabilities.
4. Target requirements.
5. Resource requirements.
6. Lowering contracts.
7. ABI compatibility.
8. Artifact formats.
9. Verification requirements.
10. Failure behavior.
11. Version compatibility.
12. Portability implications.

Adding a backend must not require changing source semantics.

Adding a new target must not require rewriting a portable program.

27. Portability test architecture

The repository must maintain portability tests.

Recommended organization:

tests/
|
+-- portability/
    |
    +-- positive/
    +-- negative/
    +-- boundary/
    +-- semantic_equivalence/
    +-- cross_target/
    +-- artifacts/
    +-- serialization/
    +-- capabilities/
    +-- resources/
    +-- classical/
    +-- quantum/
    +-- hybrid/
    +-- ai/
    +-- hdl/
    +-- distributed/
    +-- security/
    +-- compatibility/
    +-- determinism/
    +-- scalability/
    +-- diagnostics/

Reuse existing test locations where equivalent infrastructure already exists.

Do not duplicate test frameworks.

27.1 Required fixtures

The following fixtures should be available where supported by the language:

minimal.zm
classical.zm
quantum.zm
hybrid.zm
ai.zm
hdl.zm
distributed.zm
poco-reaf.zm

27.2 Semantic equivalence tests

Test the same semantic computation through multiple realizations.

Examples:

Same classical program
       |
       +---- CPU
       |
       +---- GPU
       |
       +---- Other compatible accelerator

Verify equivalent observable behavior within the applicable numerical and concurrency contracts.

Quantum:

Same logical quantum program
       |
       +---- Quantum simulator
       |
       +---- QPU A
       |
       +---- QPU B

Verify the applicable logical and statistical semantic contract, accounting for the declared noise and measurement model.

HDL:

Same hardware intent
       |
       +---- Simulation
       |
       +---- FPGA
       |
       +---- ASIC flow

Verify equivalence under the supported hardware semantics.

Distributed:

Same distributed computation
       |
       +---- Smaller deployment
       |
       +---- Larger deployment

Verify that changing deployment scale does not violate the declared computation contract.

28. Hard-coding audit

The compiler and artifact subsystem must be audited for hidden capacity restrictions.

Search for constructs such as:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_WORKERS
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_DEVICES

Also investigate equivalent restrictions expressed through:

- Fixed-size arrays.
- Closed target enums.
- Fixed-size identifier tables.
- Arbitrary collection truncation.
- Hardcoded worker counts.
- Hardcoded parser limits.
- Fixed metadata lengths.
- Fixed artifact counts.
- Fixed domain registries.

A restriction must be classified before being removed.

Some operational safeguards are legitimate.

For example, an explicitly configured artifact byte budget is acceptable.

A hidden universal artifact-size ceiling is not.

The audit must distinguish:

Restriction| Policy
Universal hardware capacity ceiling| Prohibited
Universal language-level workload ceiling| Prohibited
Configurable resource budget| Permitted
Target-specific capability limit| Permitted
Security safeguard| Permitted when justified
Actual operating-system limitation| Must be reported
Implementation allocation failure| Must be reported
Program-defined numeric constant| Permitted

29. Performance and resource behavior

The compiler must support both small and large workloads efficiently.

It should provide:

- Lazy processing where appropriate.
- Incremental compilation.
- Dependency-aware caching.
- Parallel compilation.
- Streaming artifact processing where appropriate.
- Avoidance of unnecessary materialization.
- Configurable concurrency.
- Explicit cancellation.
- Resource-aware compilation.
- Memory-conscious transformations.
- Deterministic output collection where required.

The compiler must not promise that every workload can fit in memory.

Where the implementation supports streaming or external storage, it should use those mechanisms for large artifacts.

Where it does not, it must report the actual limitation rather than silently truncate data.

30. Production quality requirements

The implementation must satisfy the repository's Rust requirements.

cargo fmt --all -- --check

cargo check --all-targets

cargo test --all-targets

cargo clippy --all-targets -- -D warnings

Additionally:

- Rust 1.97 or later.
- Rust 2021 edition.
- No unsafe Rust.
- No artificial universal hardware ceilings.
- No duplicate IR authorities.
- No duplicate artifact authorities.
- No fabricated backend output.
- No silent semantic fallback.
- No unverified portability claims.
- No successful compilation result when a required stage failed.
- No unverified external payloads represented as validated.
- No target-specific artifact represented as universally portable.

Where the repository's grammar architecture requires Rust 1.97.1 or later, that stronger minimum must be preserved.

31. File integration and freeze requirements

This contract must be implementable without repeatedly redesigning its shared interfaces.

The following contracts must be established before dependent implementation work is frozen.

File| Required contract
"compiler/mod.rs"| Canonical compiler composition and artifact export
"compiler/artifacts/mod.rs"| Canonical artifact public API
"compiler/artifacts/artifact.rs"| Artifact identity and payload
"compiler/artifacts/manifest.rs"| Artifact dependencies
"compiler/artifacts/metadata.rs"| Metadata schema and policy
"compiler/artifacts/portability.rs"| Canonical artifact portability assessment
"compiler/artifacts/validation.rs"| Validation states and evidence requirements
"compiler/artifacts/serialization.rs"| Stable wire compatibility
"compiler/diagnostics.rs"| Portability diagnostic integration
"compiler/analysis/portability.rs"| Semantic and IR portability analysis
"compiler/ir/portability.rs"| IR portability verification
"compiler/planning/feasibility.rs"| Capability and resource feasibility
"compiler/backends/compatibility.rs"| Backend compatibility
"compiler/testing/portability.rs"| Shared portability test contracts

The last five paths are proposed integration points, not assertions that the files already exist.

Every implementation file must define:

1. Purpose.
2. Ownership.
3. Non-ownership.
4. Public API.
5. Dependencies.
6. Upstream integrations.
7. Downstream integrations.
8. Data contracts.
9. Invariants.
10. Errors.
11. Resource behavior.
12. Scalability behavior.
13. Determinism.
14. Security.
15. Compatibility.
16. Tests.
17. Freeze criteria.

A file is frozen only after its implementation and integrations satisfy these obligations.

A legitimate later semantic or compatibility change may require a reviewed modification. Freezing must never preserve a known defect.

32. Implementation sequence

The implementation must follow dependency order.

Phase 0 — Repository audit

Verify:

- Existing compiler modules.
- Existing artifact subsystem.
- Existing grammar authorities.
- Existing IR authorities.
- Existing backend interfaces.
- Existing tests.
- Existing module exports.
- Existing compatibility obligations.

Deliverable: authoritative dependency and integration map.

Phase 1 — Artifact integration

Complete:

- "src/compiler/artifacts/mod.rs"
- "src/compiler/artifacts/artifact.rs"
- "src/compiler/artifacts/manifest.rs"
- "src/compiler/artifacts/metadata.rs"
- "src/compiler/artifacts/portability.rs"
- "src/compiler/artifacts/validation.rs"
- "src/compiler/artifacts/serialization.rs"

Preserve the existing canonical implementations.

Phase 2 — Compiler root integration

Complete the artifact integration in:

"src/compiler/mod.rs"

Preserve the compiler's existing public module hierarchy.

Phase 3 — Semantic and IR portability

Implement the compiler-facing portability analysis and verification contracts.

Integrate canonical semantic and IR authorities.

Phase 4 — Transformation portability

Integrate pass legality, semantic preservation, analysis invalidation and provenance.

Phase 5 — Capability and feasibility

Integrate capability matching, resource planning and feasibility reporting.

Phase 6 — Heterogeneous planning

Integrate the existing hardware partitioner and hybrid pipeline.

Phase 7 — Backend compatibility

Integrate existing backend implementations through established backend contracts.

Phase 8 — Artifact realization

Integrate target-specific artifact generation, validation, serialization and linking.

Phase 9 — Incremental and parallel compilation

Ensure portable compilation and target-specific realization use sound dependency identities.

Phase 10 — Production conformance

Run cross-target, hybrid, semantic-equivalence, security, scalability and regression tests.

33. Acceptance criteria

The compiler is conformant with this portability contract only when the following conditions are demonstrated.

Language and semantics

- [ ] One canonical Zamani language.
- [ ] One canonical grammar composition root.
- [ ] One canonical AST architecture.
- [ ] One semantic authority.
- [ ] Portable semantics remain target-independent.
- [ ] Requirements and capabilities remain distinct.
- [ ] Logical and physical resources remain distinct.

Intermediate representations

- [ ] ZUIR remains canonical.
- [ ] Classical IR remains canonical.
- [ ] "quantum::ir" remains canonical.
- [ ] HDL/hardware IR remains canonical.
- [ ] AI and tensor integration does not duplicate existing authorities.
- [ ] Hybrid dependencies are preserved.
- [ ] Distributed semantics are preserved.

Portability

- [ ] Portable artifacts are distinguishable from target-specific artifacts.
- [ ] Portability declarations are distinguishable from verified evidence.
- [ ] Unknown evidence is not treated as successful verification.
- [ ] Target-specific requirements are recorded.
- [ ] Future backend extensions do not require source rewriting.
- [ ] Cross-target semantic equivalence is tested.
- [ ] Unsupported operations produce explicit diagnostics.

Scalability

- [ ] No artificial universal hardware ceilings.
- [ ] No hidden device-count ceilings.
- [ ] No fixed worker-count ceiling.
- [ ] No arbitrary tensor-rank ceiling.
- [ ] No universal memory ceiling.
- [ ] Configurable operational budgets work.
- [ ] Actual resource exhaustion is reported.
- [ ] Large artifact processing is supported by the available implementation strategy.

Artifacts

- [ ] Artifact integrity is validated.
- [ ] Manifest dependencies are validated.
- [ ] Serialization versions are enforced.
- [ ] Portability assessment is integrated.
- [ ] Provenance is preserved.
- [ ] Incomplete validation remains incomplete.
- [ ] Target-specific artifacts identify their compatibility requirements.

Safety and reliability

- [ ] Safe Rust only.
- [ ] Rust baseline is verified.
- [ ] Deterministic behavior is tested where required.
- [ ] Security validation is integrated.
- [ ] No fabricated compilation success.
- [ ] No silent semantic changes.
- [ ] Required quality gates pass.

34. Final architecture invariant

The following invariant is mandatory:

The same Zamani program must retain its declared semantic meaning when its realization changes, provided the new realization satisfies the program's requirements and constraints.

The compiler may:

- Optimize.
- Specialize.
- Decompose.
- Lower.
- Legalize.
- Partition.
- Select compatible backends.
- Produce multiple target artifacts.

But it must not silently redefine the program.

35. Final POCO-REAF model

                 ONE ZAMANI PROGRAM
                         |
                         v
                 ONE SOURCE MEANING
                         |
                         v
                ONE CANONICAL MODEL
                         |
                         v
                PORTABLE IR / PACKAGE
                         |
                         v
                 VERIFIED ARTIFACT
                         |
              +----------+----------+
              |          |          |
              v          v          v
             CPU        GPU        FPGA
              |          |          |
              +----------+----------+
                         |
              +----------+----------+
              |          |          |
              v          v          v
             QPU     DISTRIBUTED   CLOUD
              |          |          |
              +----------+----------+
                         |
                         v
                  FUTURE TARGETS

The source semantics remain stable.

The implementation adapts to the environment.

The compiler must not turn today's hardware limitations into permanent language limitations.

36. Final definitions

Portability

Preservation of a program's declared semantic contract across permitted changes in implementation realization.

Scalability

The ability of the same language and semantic architecture to represent and process workloads of varying sizes without arbitrary universal capacity ceilings.

Portable compilation artifact

A versioned artifact containing validated target-independent representations and the information necessary for subsequent compatible realization.

Target-specific realization

A derived implementation of portable semantics for a particular target or execution environment.

POCO-REAF

The ability to express computation once, establish its semantic meaning once, preserve that meaning in canonical portable representations, and reuse it across compatible present and future execution environments without rewriting the source merely because the underlying hardware or execution topology changes.

---

37. Completion and conformance statement

This document defines the compiler portability contract.

Its existence does not prove that the compiler, backends, runtime or all computing domains already satisfy it.

The repository must maintain separate evidence for:

Component| Required status
Portability contract| Defined and reviewed
Grammar integration| Conformant
Semantic integration| Implemented and verified
IR integration| Implemented and verified
Artifact portability| Implemented and tested
Capability integration| Implemented and tested
Feasibility integration| Implemented and tested
Backend integration| Verified per provider
Cross-target behavior| Conformance demonstrated
Runtime realization| Verified per execution environment

The overall portability claim must reflect the actual demonstrated implementation status.

No subsystem may claim universal portability solely because its interface or specification exists.

Final principle:

Zamani must not solve today's hardware limitations by turning them into tomorrow's programming-language limitations.

The language defines computation.

The semantic layer establishes meaning.

The canonical IR preserves that meaning.

The compiler coordinates transformations and realization.

The backends produce target-specific implementations.

The runtime executes those implementations.

The same program semantics remain the source of truth.

Program Once. Compile Once. Run Everywhere. Anywhere. Forever.