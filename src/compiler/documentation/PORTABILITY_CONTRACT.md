Zamani Compiler — Production Portability Contract

Canonical path: "src/compiler/documentation/PORTABILITY_CONTRACT.md"

Repository: "github.com/Benwellonedge28/Zamani"

Subsystem: "src/compiler/"

Document ID: "ZAMANI-COMPILER-PORTABILITY"

Contract version: 1.0.0

Document role: Normative compiler architecture, portability, integration and conformance contract.

Implementation status: This document establishes the required production contract. Compliance must be demonstrated by implementation and tests; documentation alone does not establish production readiness.

Rust edition: 2021.

Minimum Rust version: 1.97, consistent with the current "Cargo.toml" declaration. The crate documentation must be reconciled with this baseline.

Memory safety: Safe Rust only. "unsafe" Rust is prohibited.

Primary objective:

"Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)"

Scalability objective:

Support the smallest meaningful computation through arbitrarily large finite computations and deployments, subject to actual resource availability, implementation capabilities, representation constraints, program semantics and explicitly configured operational budgets.

The language and compiler MUST NOT impose arbitrary universal hardware-capacity ceilings.

---

1. Purpose

This document establishes the production portability contract for the Zamani compiler.

It defines how the compiler must preserve the semantic meaning of Zamani programs while supporting compilation, transformation, optimization, packaging and realization across different computational environments.

The portability contract applies to:

- Classical computing.
- Systems programming.
- Parallel and concurrent computing.
- Scientific and numerical computing.
- AI and machine learning.
- Deep learning and neural networks.
- Tensor computation.
- Symbolic and probabilistic computation.
- Quantum computing.
- Quantum simulation.
- Quantum-classical hybrid computation.
- Quantum error-correction integration.
- Hardware description languages.
- Hardware/software co-design.
- FPGA and ASIC synthesis.
- GPU and other accelerator computation.
- Neuromorphic computation.
- Photonic and optical computation.
- Distributed computing.
- Networking.
- Cryptographic computation.
- Embedded and edge computing.
- Cloud and high-performance computing.
- Future computational architectures.

These are domains of one programming language.

The compiler MUST NOT introduce separate, incompatible language semantics for different hardware families.

The fundamental principle is:

Zamani source code defines computational meaning. The compiler determines valid representations and realization strategies. The backend and runtime infrastructure realize those computations using actual compatible resources.

The compiler MUST preserve this separation throughout its entire lifecycle.

2. Normative terminology

The following terms have normative meanings.

Term| Meaning
MUST| Mandatory requirement
MUST NOT| Prohibited behavior
REQUIRED| Mandatory requirement
SHOULD| Recommended unless a documented reason justifies deviation
SHOULD NOT| Discouraged unless a documented reason justifies deviation
MAY| Permitted behavior
CAN| Capability statement
PORTABLE| Satisfies an explicitly established portability contract
TARGET-INDEPENDENT| Does not unnecessarily depend on a particular target realization
TARGET-SPECIFIC| Explicitly associated with a particular realization
REALIZATION| Implementation of validated computation for an execution environment
FEASIBILITY| Evidence-based determination that a proposed realization can satisfy its requirements
UNKNOWN| Insufficient evidence to establish a property
VERIFICATION| Validation performed by the appropriate authoritative verifier
POCO-REAF| Program Once, Compile Once, Run Everywhere, Anywhere, Forever

A requirement in this document MUST NOT be interpreted as proof that the corresponding implementation already exists.

3. Architectural authority

This document is a compiler-facing contract.

It does not replace the language specifications, grammar, semantic implementation, intermediate representations, backends or runtime.

3.1 Repository authorities

Authority| Responsibility
"grammar/DESIGN.md"| Grammar architecture and ownership
"grammar/Zamani.g4"| Canonical ANTLR composition root
"grammar/resources/portability.g4"| Source-level portability syntax
"grammar/compile/target.g4"| Source-level target intent
"grammar/compile/target-selection.g4"| Source-level target-selection intent
"grammar/specification/portability.md"| Normative language portability specification
"grammar/spec/portability.md"| Formal portability semantic contract
"grammar/spec/resources.md"| Resource semantics
"grammar/spec/determinism.md"| Determinism and reproducibility semantics
"grammar/spec/compatibility.md"| Compatibility and migration semantics
"grammar/specification/semantic-model.md"| Canonical semantic model
"grammar/specification/poco-reaf.md"| POCO-REAF language objective
"grammar/specification/scalability-model.md"| Language scalability requirements
"grammar/validation/portability.md"| Portability validation requirements
"grammar/validation/semantic-boundaries.md"| Semantic ownership boundaries
"src/frontend/"| Canonical frontend implementation
"src/frontend/semantic/"| Semantic authority
"src/zuir/"| Universal intermediate representation
"src/classical/"| Classical computation authority
"src/quantum/ir/"| Canonical quantum IR
"src/hdl/hdlir/"| Hardware-description IR
"src/ai/"| AI computation authority
"src/hybrid/"| Hybrid computation authority
"src/distributed/"| Distributed computation authority
"src/backend/"| Backend implementation authority
"src/runtime/"| Runtime and physical execution authority

The compiler integrates these authorities.

It MUST NOT recreate their responsibilities.

3.2 Conflict resolution

When a conflict is discovered:

1. Identify the authoritative subsystem responsible for the conflicting concept.
2. Record the conflicting definitions and affected interfaces.
3. Do not silently select whichever implementation is easiest.
4. Resolve the conflict through the responsible specification and architecture owners.
5. Version incompatible contract changes.
6. Update affected implementations and conformance tests.

A compiler implementation MUST NOT silently redefine source-language semantics.

4. Portability ownership

4.1 What this contract owns

This document owns the compiler-facing rules for:

- Semantic portability preservation.
- Target-independent compilation.
- Portable representation integration.
- Portability-preserving transformations.
- Portable compilation artifacts.
- Target-specific realization boundaries.
- Portability evidence requirements.
- Capability and requirement preservation.
- Cross-target semantic conformance.
- Artifact compatibility.
- Reproducible compilation requirements.
- Portability diagnostics.
- POCO-REAF compiler integration.
- Portability classification integration.
- Portability testing.
- Portability-related failure handling.

4.2 What this contract does not own

It does not own:

- Source grammar.
- Lexer implementation.
- Parser implementation.
- AST definitions.
- Source-level type semantics.
- Source-level effect semantics.
- Ownership semantics.
- Resource semantics.
- Capability definitions.
- Canonical IR definitions.
- Domain-specific optimization algorithms.
- Hardware discovery.
- Physical resource allocation.
- Physical placement.
- Runtime scheduling.
- Quantum routing.
- Quantum error correction.
- Noise modeling.
- ZQN implementation.
- HAL implementation.
- Backend code generation.
- Physical execution.

These responsibilities remain with their established repository authorities.

5. Fundamental portability definition

Portability is the preservation of the declared semantic contract of a program while its realization changes.

Let:

- P = Validated Zamani program.
- S(P) = Specified semantic meaning.
- E = Execution environment.
- C(E) = Capabilities available in the environment.
- R(E) = Resources available in the environment.
- K(P) = Program constraints.
- Q(P) = Applicable policies.
- L = Selected realization strategy.
- O = Observable program behavior.

A realization is:

"Realize(S(P), E, L) -> O"

A realization is valid only if it preserves the required semantic properties of the original program.

For two environments:

"Realize(S(P), E1, L1)"

and

"Realize(S(P), E2, L2)"

the realizations are portable equivalents when both satisfy the same applicable source-level semantic contract.

The physical implementations may differ.

For example:

- CPU instructions may differ.
- Memory layouts may differ.
- Parallel decomposition may differ.
- Device selection may differ.
- Quantum gate decomposition may differ.
- Physical qubit mappings may differ.
- FPGA synthesis may differ.
- Distributed placement may differ.
- Network topology may differ.
- Scheduling may differ.

However, the implementation MUST NOT silently violate the program's specified observable behavior.

The equivalence test MUST account for:

- Types.
- Effects.
- Ownership.
- Memory safety.
- Numerical semantics.
- Floating-point guarantees.
- Concurrency semantics.
- Ordering requirements.
- Communication semantics.
- Quantum state and measurement semantics.
- Hardware-description semantics.
- Explicit approximation contracts.
- Explicit nondeterminism.
- Security and correctness guarantees.

Portability does not require identical internal implementations.

6. POCO-REAF contract

6.1 Definition

POCO-REAF means:

Program Once → Compile Once → Run Everywhere, Anywhere, Forever.

The compiler must make this objective technically meaningful.

It requires:

1. One canonical source-language meaning.
2. One authoritative semantic model.
3. Stable canonical representation contracts.
4. Explicit capability requirements.
5. Explicit resource requirements.
6. Semantics-preserving transformations.
7. Portable compilation artifacts.
8. Extensible backend realization.
9. Reproducible compilation.
10. Cross-target conformance.
11. Compatibility and migration guarantees.
12. Preservation of observable semantics.

6.2 Meaning of Compile Once

Compile Once MUST NOT be interpreted as requiring one native executable binary to execute directly on every conceivable architecture.

The compiler must distinguish:

Canonical compilation

The source program is parsed, semantically validated and converted into canonical representations.

Portable compilation output

A verified representation or artifact retains sufficient semantic information for subsequent compatible realization.

Target realization

A backend may perform target-specific lowering, legalization, specialization, code generation or packaging.

The intended architecture is:

             ZAMANI SOURCE
                    |
                    v
              FRONTEND
                    |
                    v
           SEMANTIC VALIDATION
                    |
                    v
        CANONICAL SEMANTIC MODEL
                    |
                    v
          CANONICAL DOMAIN IRs
                    |
                    v
           VERIFIED PORTABLE IR
                    |
                    v
          PORTABLE ARTIFACT PACKAGE
                    |
          +---------+---------+
          |         |         |
          v         v         v
         CPU       GPU       FPGA
          |         |         |
          v         v         v
        TARGET-SPECIFIC REALIZATION
                    |
                    v
               EXECUTION

A portable artifact should be reusable for subsequent compatible target realizations without requiring developers to rewrite the original source program or unnecessarily repeat source-level semantic analysis.

Target-specific code generation may still be required.

6.3 What POCO-REAF does not promise

POCO-REAF does not promise:

- Infinite physical memory.
- Infinite processing capacity.
- Infinite execution time.
- Universal hardware capabilities.
- Identical performance on every machine.
- Identical native binaries on every architecture.
- Automatic execution of unsupported operations.
- Guaranteed availability of every accelerator.
- Guaranteed physical resource reservations.
- Automatic semantic degradation when requirements cannot be met.

A program requiring a quantum capability cannot be assumed executable on a CPU without an explicitly supported simulator or alternative realization preserving its required semantics.

An HDL artifact is not automatically a native executable.

A target-independent IR is not automatically a physical hardware implementation.

7. Portability dimensions

The compiler MUST distinguish the following portability dimensions.

Dimension| Contract
Source portability| The same source can be accepted by compatible language implementations.
Semantic portability| The specified meaning remains preserved.
IR portability| Compatible canonical representations can be transported and validated.
Artifact portability| Compilation artifacts can be consumed by compatible implementations.
Realization portability| A compatible backend can derive a valid target-specific implementation.
Execution portability| The program can execute in an environment satisfying its requirements.

These dimensions are related but independent.

A program may have source and semantic portability while requiring a particular class of execution capability.

A portable IR artifact may not be directly executable.

A target-specific executable may be valid but not portable to another architecture.

The compiler MUST NOT collapse these dimensions into one boolean.

8. Portability classifications

The existing canonical classification authority is:

"src/compiler/artifacts/portability.rs"

No second competing "PortabilityClass" enum may be introduced.

The classification model must be capable of representing:

Classification| Meaning
Portable| Target-independent within its established contract
Resource-dependent| Depends on available abstract resources
Capability-dependent| Requires specified capabilities
Scale-dependent| Realization depends on workload size
Target-constrained| Valid only under explicit target constraints
Target-specific| Bound to a particular realization
Implementation-defined| Depends on documented implementation choices
Non-portable| Cannot satisfy the requested portability boundary
Unknown| Insufficient evidence to establish portability

The implementation MUST preserve the existing public API and classification semantics unless a reviewed compatibility migration is approved.

8.1 Unknown is not portable

Unknown evidence MUST NOT be interpreted as:

- Verified portable.
- Verified compatible.
- Feasible.
- Executable.
- Safe to deploy.

Unknown and incompatible are different states.

A target may be incompatible because evidence establishes that it cannot satisfy a requirement.

A target may be unknown because insufficient information is available.

Both states must remain distinguishable.

9. Requirements, capabilities and policies

The compiler MUST preserve the distinction between six concepts.

Concept| Meaning| Authority
Requirement| What the computation needs| Frontend semantic model
Capability| What an implementation can provide| Capability authority
Preference| What the developer would prefer| Semantic/configuration policy
Constraint| What a valid realization must obey| Semantic/resource contracts
Hint| Advisory optimization information| Appropriate semantic or planning contract
Feasibility| Whether evidence establishes a valid realization| Compiler planning and authoritative evaluators

These concepts MUST NOT be merged into a single target flag or closed hardware enumeration.

Examples:

- A quantum requirement is not a vendor selection.
- A GPU preference is not a mandatory GPU requirement.
- A memory budget is not a universal language memory limit.
- A target descriptor is not proof of hardware availability.
- A registered backend is not proof of compatibility.
- A compatibility decision is not a resource reservation.

9.1 Requirements

Mandatory requirements MUST be preserved throughout compilation.

They MUST NOT silently become preferences or hints.

9.2 Capabilities

Capabilities must use extensible identities.

The compiler MUST NOT require a new central enum or compiler-root modification for every future computational capability.

Capability evidence must be obtained through authoritative registries and evaluators.

9.3 Preferences

Preferences may influence selection and optimization.

They MUST NOT override correctness, security, compatibility, feasibility or mandatory constraints.

9.4 Constraints

Hard constraints MUST be satisfied.

An unsatisfied hard constraint MUST result in explicit infeasibility or rejection.

9.5 Hints

Hints may guide implementation decisions.

They MUST NOT silently change program semantics.

9.6 Feasibility

Feasibility must be evidence-based.

The compiler MUST distinguish:

- Feasible.
- Infeasible.
- Unknown.
- Invalid evidence.
- Stale evidence.
- Evaluation failure.

10. Resource-independent scalability

Zamani must support computation from the smallest meaningful workload to arbitrarily large finite workloads.

The compiler MUST NOT introduce universal hardware ceilings.

The prohibition includes:

MAX_QUBITS
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS
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
MAX_WORKERS
MAX_MODULES
MAX_OPERATIONS
MAX_TARGETS
MAX_BACKENDS
MAX_DOMAINS

Equivalent hidden restrictions are equally prohibited.

10.1 Real implementation constraints

The prohibition does not eliminate real limitations.

The compiler may encounter:

- Host address-space limits.
- Available memory limits.
- Provider capacity.
- Operating-system limits.
- Target instruction-width limits.
- Artifact-format limits.
- Representation limits.
- Operational time budgets.
- Network limits.
- Physical hardware limits.

These MUST be represented as explicit implementation or target constraints.

They MUST NOT be presented as universal Zamani language limitations.

10.2 Operational budgets

Budgets are permitted.

Examples:

- Maximum compilation time.
- Maximum memory consumed by one compilation.
- Maximum temporary storage.
- Maximum concurrent compiler work.
- Maximum optimization effort.
- Maximum artifact transfer size.

Budgets must be configurable through the appropriate configuration and policy authorities.

They must not silently become universal semantic ceilings.

10.3 Exact quantities

The compiler must preserve exact resource quantities where the semantic model requires exactness.

The canonical resource authority is:

"src/frontend/semantic/resources/"

In particular:

- "model.rs" owns canonical resource quantities and resource records.
- "symbolic.rs" owns symbolic resource expressions.
- "requirements.rs" owns extraction from the canonical AST.
- "constraints.rs" owns semantic constraint interpretation.
- "mod.rs" owns the subsystem composition boundary.

"ExactQuantity" uses canonical decimal strings rather than imposing "u64", "u128" or "usize" limits on semantic quantities.

The compiler resource planner MUST consume these canonical types rather than defining competing quantity models.

Machine-sized indexes may be used internally where appropriate, but their implementation limits must be explicit and must not redefine source-language resource semantics.

10.4 Literal infinity

The phrase scale to infinity means:

No arbitrary universal finite capacity ceiling is imposed by the language or compiler architecture.

It does not mean that physical computation is mathematically infinite.

The compiler must support progressively larger workloads whenever the relevant representations, implementations and available resources permit them.

11. Target independence

A program should not depend unnecessarily on a particular machine.

The compiler MUST distinguish:

- Target identity.
- Target role.
- Target capability.
- Target availability.
- Target compatibility.
- Target feasibility.
- Target preference.
- Target realization.

The existing target descriptor authority is:

"src/compiler/configuration/target.rs"

It owns:

- Target identifiers.
- Target roles.
- Artifact-format identifiers.
- Extensible metadata.
- Descriptor construction.
- Structural validation.
- Legacy target adaptation.

It does NOT own:

- Physical discovery.
- Hardware inventory.
- Capability matching.
- Resource allocation.
- Backend compatibility.
- Runtime scheduling.
- Physical execution.

11.1 Open-world target identities

Target identities MUST remain extensible.

Adding a new processor, accelerator, QPU, FPGA, ASIC, simulator, photonic device or future computational substrate MUST NOT require redesigning the central compiler architecture.

Target identifiers are not proof of capability.

11.2 No implicit host target

A production compiler MUST NOT silently choose an arbitrary host architecture when a target decision is required.

A missing target must be handled according to the compilation request and portability contract.

The compiler may perform target-independent compilation without selecting a physical execution target.

Target-specific realization requires an explicit target or a valid target-discovery and selection process.

The current legacy "CompilationTarget" compatibility model must not become the permanent closed-world target authority.

12. Canonical representation portability

The compiler integrates existing representation authorities.

It MUST NOT introduce another universal or domain-specific IR merely to simplify compiler implementation.

Representation| Canonical authority
Universal IR| "src/zuir/"
Classical computation| "src/classical/"
Quantum computation| "src/quantum/ir/"
HDL/hardware| "src/hdl/hdlir/"
AI computation| "src/ai/"
Hybrid computation| "src/hybrid/"
Distributed computation| "src/distributed/"

The compiler-facing integration boundary is:

"src/compiler/ir/"

This subsystem owns adapters, compatibility coordination, translation orchestration, lowering integration and verification-result propagation.

It MUST NOT own a competing canonical IR.

12.1 ZUIR

ZUIR remains the universal IR authority.

It must preserve the computational structure and semantic information required for downstream realization.

The compiler must consume and coordinate ZUIR rather than redefine its program, operation, value, region or block models.

ZUIR serialization and structural verification remain under "src/zuir/".

12.2 Classical computation

Classical computations must preserve their semantics across compatible CPU, GPU and accelerator realizations.

The compiler must not assume that every classical operation can be executed identically on every accelerator.

12.3 Quantum computation

The canonical quantum semantic boundary is:

"src/quantum/ir/"

The compiler MUST preserve:

- Quantum state semantics.
- Operation semantics.
- Measurement semantics.
- Classical control dependencies.
- Logical resource requirements.
- Required ordering.
- Applicable effects.
- Explicit approximation and error contracts.

The compiler MUST NOT confuse logical and physical qubits.

Physical qubit mapping, routing, scheduling, calibration, noise and QEC remain with their respective authoritative subsystems.

12.4 HDL and hardware description

The compiler must preserve hardware-description semantics.

It must distinguish:

- Logical hardware structures.
- Signal semantics.
- Structural connectivity.
- Timing contracts.
- Synthesis requirements.
- Target-specific physical realization.

It MUST NOT impose universal limits on signal count, register count, bus width, pipeline depth or physical resources.

Actual synthesis constraints belong to the selected synthesis implementation and target.

12.5 AI and tensor computation

AI and tensor semantics must remain under the established AI, semantic and tensor authorities.

The compiler must preserve:

- Tensor shape information.
- Symbolic and dynamic dimensions.
- Data dependencies.
- Numerical semantics.
- Model structure.
- Differentiation requirements.
- Training and inference semantics.
- Distributed and accelerator requirements.

No arbitrary universal tensor-rank, model-size or parameter-count ceilings may be introduced.

12.6 Hybrid computation

Hybrid compilation must preserve cross-domain relationships.

Examples include:

- Classical-to-quantum data dependencies.
- Quantum-to-classical measurement results.
- AI-to-classical computation.
- AI-to-quantum interfaces.
- HDL-to-software interfaces.
- Distributed-to-local execution.
- Cross-domain synchronization.

The compiler MUST NOT create a competing hybrid semantic authority.

12.7 Distributed computation

Distributed portability must preserve:

- Communication semantics.
- Data dependencies.
- Ordering.
- Consistency requirements.
- Synchronization.
- Failure contracts.
- Security boundaries.
- Provenance.
- Partitioning constraints.

The compiler may produce abstract partitioning and routing plans.

Actual resource allocation and runtime execution belong to the relevant downstream authorities.

13. Portability-preserving transformations

Every transformation that changes a program representation MUST have an explicit semantic-preservation contract.

This applies to:

- Optimization.
- Simplification.
- Specialization.
- Monomorphization.
- Instruction fusion.
- Operation decomposition.
- Outlining.
- Domain lowering.
- Target legalization.
- Code generation.
- Linking.
- Artifact composition.

A transformation must declare:

1. Its accepted input representation.
2. Its output representation.
3. Applicable semantic domains.
4. Preconditions.
5. Required analyses.
6. Legality conditions.
7. Preserved analyses.
8. Invalidated analyses.
9. Required verification.
10. Failure behavior.
11. Provenance information.
12. Determinism guarantees.

A transformation valid for classical IR MUST NOT automatically be considered valid for quantum IR or HDLIR.

A failed legality check MUST NOT be converted into successful transformation output.

13.1 Optimization

Optimization remains coordinated by:

- "src/compiler/optimization/"
- "src/compiler/passes/"

The compiler must support extensible optimization providers and domain-specific legality rules.

A closed mapping of every possible optimization or hardware domain is prohibited.

Optimization must preserve the applicable semantic, numerical, ownership, effect, concurrency and correctness contracts.

14. Capability matching and backend selection

Backend selection is not portability verification.

The compiler must preserve the following distinctions:

Backend registration
        |
        v
Backend discovery
        |
        v
Capability evaluation
        |
        v
Compatibility evaluation
        |
        v
Resource feasibility
        |
        v
Selection
        |
        v
Backend invocation

The relevant existing compiler boundary is:

"src/compiler/backends/"

The authoritative backend implementation remains:

"src/backend/"

14.1 Existing backend selection

"src/compiler/backends/selection.rs" already defines:

- Backend selection requests.
- Ordered preferences.
- Backend eligibility.
- Compatibility evaluation.
- Unknown compatibility.
- Deterministic ranking.

This contract must be preserved.

Unknown compatibility MUST NOT be treated as compatible.

Backend registration MUST NOT establish compatibility.

Compatibility MUST NOT establish resource feasibility.

Resource feasibility MUST NOT establish physical availability.

Backend selection MUST NOT be confused with execution.

14.2 Extensibility

New backends must integrate through the established registry and provider contracts.

The compiler MUST NOT require a new closed enumeration or a modification of the composition root for every new backend.

14.3 Fallback

Fallback is permitted only when the alternative realization satisfies the original semantic requirements.

The compiler MUST NOT silently replace:

- A required quantum operation with a different classical computation.
- A required hardware implementation with a software approximation.
- A required precision with lower precision.
- A required consistency model with a weaker one.

Any permitted alternative must be supported by explicit semantics and validation.

15. Resource feasibility and planning

The compiler-facing planning authority is:

"src/compiler/planning/"

The existing resource planning implementation is:

"src/compiler/planning/resource_planning.rs"

It already consumes:

- Canonical semantic resource requirements.
- Capability-matching results.
- Target-scoped resource evidence.
- Exact resource quantities.

The compiler must preserve these ownership boundaries.

Resource planning determines whether a proposed realization is supported by available evidence.

It does not allocate, reserve or execute physical resources.

15.1 Feasibility results

Planning must distinguish:

- Feasible.
- Infeasible.
- Unknown.
- Invalid evidence.
- Stale evidence.
- Evaluation failure.

Insufficient evidence MUST NOT be interpreted as feasibility.

15.2 Resource snapshots

Resource observations must carry sufficient identity and provenance to identify the inventory revision against which a plan was made.

A plan based on stale evidence must not be represented as a guarantee of current availability.

15.3 Physical allocation

Physical resource acquisition belongs to the runtime and relevant resource providers.

The compiler must hand off requirements and plans without pretending that a plan reserves hardware.

16. Artifact portability

The canonical compiler artifact subsystem is:

"src/compiler/artifacts/"

The artifact portability implementation is:

"src/compiler/artifacts/portability.rs"

This module is the single compiler-owned portability classification and assessment authority.

It must not be duplicated elsewhere.

16.1 Artifact identity

"artifacts/artifact.rs" owns artifact identity, descriptors, formats, payloads, metadata and provenance.

Artifact identity must remain distinct from:

- Source identity.
- Program identity.
- Target identity.
- Backend identity.
- Runtime identity.
- Resource identity.

16.2 Artifact manifests

"artifacts/manifest.rs" owns artifact collections and manifest relationships.

A portable manifest must preserve:

- Artifact identities.
- Dependencies.
- Representation versions.
- Semantic compatibility requirements.
- Capability requirements.
- Resource requirements.
- Target associations.
- Provenance.
- Verification evidence references.
- External payload references.
- Cross-domain dependencies.

Manifest aggregation must not hide an incompatible or unverified dependency.

16.3 Artifact metadata

"artifacts/metadata.rs" owns artifact metadata conventions.

Metadata must distinguish:

- Language information.
- Semantic-model information.
- Compiler information.
- IR information.
- Backend information.
- Target information.
- Portability information.
- Provenance information.
- Verification information.
- Security information.

Unknown namespaces must remain extensible under explicit validation policies.

Metadata alone is not proof of portability.

16.4 Portability evidence

The existing "artifacts/portability.rs" module records portability declarations and evidence references.

The following rules are mandatory:

1. A missing target reference does not prove portability.
2. A portability declaration does not prove semantic correctness.
3. An evidence reference does not automatically authenticate its source.
4. A verified portability classification requires evidence from the appropriate authoritative verifier.
5. An unverified declaration must remain distinguishable from verified evidence.
6. Conflicting declarations must be rejected or explicitly resolved.
7. Stale evidence must not silently establish current compatibility.
8. A manifest must preserve the evidence of all relevant dependencies.
9. An unsupported artifact format must not be considered valid.
10. Unknown evidence must remain unknown.

The security subsystem is responsible for trust and authenticity decisions.

The IR and semantic verification authorities are responsible for correctness evidence.

The portability subsystem records and assesses that evidence; it must not manufacture verification.

16.5 Artifact validation

"artifacts/validation.rs" owns artifact validation coordination.

Validation must distinguish:

- Valid.
- Invalid.
- Incomplete.

An incomplete validation must never be reported as fully verified.

Where applicable, validation must coordinate:

Decoding
    |
    v
Structural validation
    |
    v
Content integrity
    |
    v
Provenance validation
    |
    v
Semantic / IR verification
    |
    v
Portability assessment
    |
    v
Security validation
    |
    v
Target compatibility

The exact checks depend on the artifact kind.

A successfully decoded artifact is not automatically safe to execute.

A valid digest is not proof of semantic correctness.

16.6 Serialization

"artifacts/serialization.rs" owns the stable compiler artifact serialization boundary.

It must support:

- Versioned formats.
- Deterministic encoding.
- Strict decoding.
- Schema compatibility.
- Explicit unsupported-version errors.
- Configurable operational budgets.
- External payload references.
- Corruption detection.
- Secure input validation.

Artifact serialization versions must remain independent of:

- Language versions.
- Compiler API versions.
- Semantic-model versions.
- ZUIR versions.
- Domain IR versions.
- Backend versions.
- Runtime versions.

ZUIR serialization remains owned by "src/zuir/serialization/".

The compiler artifact serializer MUST NOT replace it.

17. Reproducibility and determinism

Portability requires controlled reproducibility.

The compiler must conform to:

"grammar/spec/determinism.md"

It must preserve deterministic behavior where required by the language and build contract.

Reproducibility must account for:

- Source identity.
- Language version.
- Grammar version.
- Semantic-model version.
- Canonical IR versions.
- Compiler version.
- Compiler configuration.
- Transformation configuration.
- Dependency fingerprints.
- Backend versions.
- Target descriptors.
- Relevant ABI information.
- Artifact serialization versions.

The compiler must distinguish target-independent compilation fingerprints from target-specific realization fingerprints.

17.1 Portable compilation fingerprints

A portable compilation fingerprint should include semantic and representation inputs that determine the canonical portable output.

It should not unnecessarily include physical device identifiers or temporary machine inventory information.

17.2 Target-specific fingerprints

A target-specific realization fingerprint must additionally include relevant:

- Target identity.
- Target descriptor version.
- Backend identity.
- Backend version.
- ABI.
- Target capabilities.
- Lowering configuration.
- Relevant toolchain versions.

17.3 Incremental compilation

The existing subsystem is:

"src/compiler/incremental/"

It must preserve:

- Correct dependency fingerprints.
- Precise invalidation.
- Cache integrity.
- Cache version compatibility.
- Reproducibility.
- Security verification.

A cache hit MUST NOT bypass required semantic, IR or security verification.

18. Compiler pipeline integration

The compiler composition root is:

"src/compiler/mod.rs"

It already establishes the intended separation between frontend, semantic analysis, IR, analysis, planning, lowering, backend integration, artifacts and runtime handoff.

The compiler pipeline must implement the following logical stages.

Stage| Authority
Source acquisition| Compiler API
Lexing| Frontend lexer
Parsing| Frontend parser
AST construction| Frontend AST
Semantic validation| Frontend semantic
Resource analysis| Frontend semantic resources
Capability analysis| Frontend semantic capabilities
Canonical IR construction| ZUIR and domain IRs
IR verification| Canonical IR authorities
Dependency analysis| Compiler analysis
Ownership and effect analysis| Frontend semantic and compiler analysis
Transformation planning| Pass manager
Optimization| Optimization subsystem
Target discovery| Planning and provider authorities
Capability matching| Planning
Resource feasibility| Resource planning
Partitioning| Planning and domain authorities
Data movement| Planning and distributed authorities
Scheduling and routing planning| Planning and relevant domain authorities
Domain-specific lowering| Canonical domain authorities and lowering
Target legalization| Lowering and backend contracts
Backend realization| "src/backend/"
Artifact validation| Artifacts and security
Linking| Compiler linking subsystem
Packaging| Artifact and deployment authorities
Runtime handoff| Runtime integration

Not every program needs every stage.

The pipeline must support domain-specific and conditional routes.

However, a required stage MUST NOT be silently skipped.

A pipeline stage must report:

- Its applicability.
- Its dependencies.
- Its input contract.
- Its output contract.
- Its verification obligations.
- Its failure conditions.
- Its provenance.
- Its resource-budget consumption.

A declared stage is not proof of an implemented stage.

19. Cross-domain portability

The compiler must preserve the semantics of each computational domain.

19.1 Classical

Required properties:

- Type and value correctness.
- Ownership and memory safety.
- Effect preservation.
- Concurrency correctness.
- Numerical guarantees.
- ABI compatibility.

19.2 Quantum

Required properties:

- Quantum state semantics.
- Gate and operation semantics.
- Measurement semantics.
- Logical resource preservation.
- Classical-quantum dependencies.
- Explicit approximation and error contracts.
- Correct quantum IR integration.

19.3 HDL

Required properties:

- Signal semantics.
- Structural relationships.
- Hardware behavior.
- Timing requirements.
- Interface compatibility.
- Synthesis constraints.
- Target-specific realization boundaries.

19.4 Hybrid

Required properties:

- Cross-domain data exchange.
- Synchronization.
- Ordering.
- Effects.
- Resource requirements.
- Capability requirements.
- Domain-specific verification.

19.5 Distributed

Required properties:

- Communication semantics.
- Ordering.
- Consistency.
- Failure behavior.
- Synchronization.
- Partition dependencies.
- Security.
- Provenance.

19.6 AI and tensor

Required properties:

- Tensor semantics.
- Shape preservation.
- Numerical behavior.
- Differentiation contracts.
- Training and inference behavior.
- Accelerator compatibility.
- Distributed computation requirements.

No domain may silently inherit another domain's transformation or execution assumptions.

20. Compiler diagnostics

Portability-related errors must use the existing compiler diagnostics subsystem:

"src/compiler/diagnostics/"

Diagnostics must integrate with the canonical frontend source-location infrastructure.

The compiler MUST NOT create a competing source-map or source-span system.

At minimum, diagnostics must distinguish:

- Invalid portability declaration.
- Missing portability evidence.
- Invalid semantic representation.
- Unsupported IR version.
- Unsupported artifact version.
- Missing capability.
- Incompatible target.
- Insufficient resources.
- Unknown resource availability.
- Stale resource evidence.
- Unsupported lowering.
- Invalid transformation.
- Failed verification.
- Incompatible backend.
- Invalid artifact dependency.
- ABI incompatibility.
- Security rejection.
- Operational budget exhaustion.
- Unsupported execution environment.

Where available, diagnostics must preserve:

- Source spans.
- Relevant AST or semantic identities.
- Target identity.
- Backend identity.
- Requirement identity.
- Evidence references.
- Failure stage.
- Recovery information.

An unsupported target must produce an explicit diagnostic rather than silently selecting an incompatible backend.

21. Security requirements

Portability must not weaken security.

The compiler security authority is:

"src/compiler/security/"

The following requirements are mandatory:

1. Validate externally supplied artifacts.
2. Validate provider identities and versions.
3. Preserve provenance.
4. Authenticate evidence where required.
5. Validate dependency integrity.
6. Reject unsupported serialization versions.
7. Prevent untrusted extensions from bypassing verification.
8. Isolate untrusted backend and provider execution.
9. Apply explicit operational budgets.
10. Protect against resource-exhaustion attacks.
11. Preserve security policies during lowering.
12. Validate generated artifacts before deployment.
13. Preserve cryptographic integrity where required.
14. Prevent cache poisoning and untrusted cache reuse.

A portability claim must never be used as a substitute for security verification.

22. Extensibility and future computational domains

The compiler must remain open to future computational technologies.

New domains and providers must integrate through explicit contracts and registries.

The architecture must not require editing a closed central enumeration whenever a new technology appears.

New integrations must identify:

- Domain identity.
- Semantic authority.
- Canonical representation.
- Capability vocabulary.
- Resource requirements.
- Verification authority.
- Transformation contracts.
- Lowering contracts.
- Backend/provider contracts.
- Artifact format.
- Compatibility rules.
- Runtime integration.
- Conformance tests.

A new domain MUST NOT introduce a competing semantic model or universal IR.

A new backend MUST NOT redefine language semantics.

A new capability MUST NOT require a redesign of the compiler's central portability model.

23. Rust safety and compatibility

All Zamani-owned compiler implementations MUST use safe Rust.

The following are prohibited:

- "unsafe" blocks.
- "unsafe" functions.
- "unsafe" implementations.
- Unnecessary raw-pointer manipulation.
- Unvalidated external data.
- Unchecked assumptions about representation sizes.

Compiler modules should establish:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

Applicable strict lint policies must be enforced at crate or module boundaries.

Broad lint suppressions must not be used to conceal incomplete implementations.

The crate's Rust minimum version must remain consistent with "Cargo.toml".

Public APIs must document:

- Ownership.
- Borrowing.
- Thread-safety.
- Error behavior.
- Compatibility guarantees.
- Resource-budget behavior.
- Versioning.

24. Exact repository integration contracts

The following integrations are mandatory.

File or subsystem| Portability responsibility
"src/compiler/mod.rs"| Compiler composition and architectural invariants
"src/compiler/api/"| Public compilation requests and responses
"src/compiler/configuration/"| Target descriptors, policies, profiles and budgets
"src/compiler/compiler_types.rs"| Legacy compatibility types and controlled migration
"src/compiler/context/"| Compilation state, dependencies and provenance
"src/compiler/analysis/"| Compiler-facing dependency, capability and feasibility analysis
"src/compiler/ir/"| Canonical IR integration and verified adapters
"src/compiler/pipeline/"| Conditional compilation stages and orchestration
"src/compiler/passes/"| Pass registration, legality and preservation
"src/compiler/optimization/"| Semantics-preserving optimization
"src/compiler/lowering/"| Domain and target-specific lowering
"src/compiler/backends/"| Backend discovery, compatibility and selection
"src/compiler/planning/"| Target, resource and heterogeneous planning
"src/compiler/parallel/"| Dependency-aware compiler parallelism
"src/compiler/incremental/"| Fingerprints, caching and invalidation
"src/compiler/artifacts/"| Artifact portability and validation
"src/compiler/linking/"| Portable module composition and target-specific linking
"src/compiler/diagnostics/"| Structured portability diagnostics
"src/compiler/security/"| Artifact, provider and evidence trust
"src/compiler/profiling/"| Measurements and optimization evidence
"src/compiler/testing/"| Portability conformance testing

24.1 Frontend integration

The compiler consumes:

- "src/frontend/"
- "src/frontend/ast/"
- "src/frontend/semantic/"
- "src/frontend/semantic/resources/"
- The canonical semantic model.

The legacy interfaces exposed through "src/lib.rs", including "src/semantic.rs" and "src/ir_gen.rs", must be migrated through explicit compatibility adapters.

They must not become competing production semantic or IR authorities.

24.2 IR integration

The compiler consumes canonical representations through:

"src/compiler/ir/"

The compiler must not duplicate:

- ZUIR.
- Classical IR.
- Quantum IR.
- HDLIR.
- AI semantic representations.
- Hybrid semantic representations.
- Distributed semantic representations.

24.3 Backend integration

The compiler coordinates backend selection and invocation.

The authoritative code-generation implementations remain in:

"src/backend/"

The compiler must not implement another backend architecture inside "src/compiler/backends/".

24.4 Runtime integration

The compiler hands validated artifacts and execution requirements to:

"src/runtime/"

The runtime owns actual resource negotiation and execution.

The compiler must not claim that a compilation plan guarantees physical execution.

24.5 Linking integration

The compiler linking authority is:

"src/compiler/linking/"

It must preserve:

- Artifact portability information.
- Semantic compatibility.
- ABI compatibility.
- Domain dependencies.
- Provenance.
- Version requirements.

It must not introduce a competing root-level linker implementation.

24.6 Grammar integration

The compiler must consume the language contracts defined by:

- "grammar/Zamani.g4"
- "grammar/resources/portability.g4"
- "grammar/compile/target.g4"
- "grammar/compile/target-selection.g4"
- "grammar/spec/portability.md"
- "grammar/spec/resources.md"
- "grammar/spec/determinism.md"
- "grammar/spec/compatibility.md"
- "grammar/specification/portability.md"
- "grammar/validation/portability.md"

The root grammar remains the canonical composition boundary.

The compiler must not create another grammar root or independently interpret source syntax.

The repository must also reconcile any documented discrepancy between the ANTLR lexer/parser import declarations and the actual grammar file inventory before declaring grammar-to-frontend portability conformance.

25. Portability conformance testing

Portability must be demonstrated through executable tests.

The compiler MUST NOT claim production portability merely because:

- A module exists.
- A trait exists.
- A grammar accepts a construct.
- A backend registers successfully.
- An artifact contains portability metadata.
- A target descriptor validates.
- A compilation stage is declared.

25.1 Required test categories

Test category| Required evidence
Source portability| Same source accepted by compatible frontend implementations
Semantic portability| Required meaning preserved
IR portability| Canonical representations verify
Artifact portability| Portable artifacts serialize, deserialize and validate
Target portability| Compatible target realization succeeds
Capability validation| Missing capabilities are detected
Resource feasibility| Insufficient resources are reported
Unknown evidence| Unknown never becomes feasible
Classical portability| Classical semantics preserved
Quantum portability| Quantum semantics preserved
HDL portability| Hardware-description semantics preserved
Hybrid portability| Cross-domain dependencies preserved
Distributed portability| Communication and consistency preserved
AI portability| Tensor and model semantics preserved
Reproducibility| Equivalent declared inputs produce reproducible outputs
Incremental compilation| Cache invalidation is correct
Security| Untrusted inputs cannot bypass validation
Scalability| No arbitrary universal hardware ceilings
Failure handling| Unsupported realizations fail explicitly

25.2 Required fixtures

The compiler conformance suite should include:

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

These fixtures must test actual compilation paths.

They must not merely test module imports or successful parsing.

25.3 Cross-target equivalence

Where multiple compatible implementations exist, tests must compare their results against the applicable semantic contract.

The comparison must use the domain's actual equivalence rules.

For example:

- Classical results may require value equivalence.
- Floating-point results may require specified numerical tolerances.
- Quantum results may require distributional or state-level equivalence.
- HDL results may require behavioral or formal equivalence.
- Distributed results may require consistency and ordering equivalence.

One universal equality test is insufficient for every computing domain.

26. Scalability qualification

The compiler must be tested progressively across increasing workload sizes.

Testing must cover:

- Small workloads.
- Increasingly large workloads.
- Large dependency graphs.
- Large artifact manifests.
- Large symbolic resource expressions.
- Large IR structures.
- Large numbers of registered providers.
- Large numbers of compilation units.
- Heterogeneous workloads.
- Distributed compilation.

Tests must evaluate:

- Memory consumption.
- Compilation time.
- Resource accounting.
- Failure behavior.
- Incremental compilation.
- Determinism.
- Parallel execution.
- Representation limits.
- Operational budget enforcement.

No fixed universal benchmark size establishes the maximum supported Zamani program.

A benchmark is evidence about a particular implementation and environment, not a language ceiling.

27. File completion and integration freeze

Every implementation file participating in portability must have an explicit completion contract.

Each file must establish:

1. Purpose.
2. Ownership.
3. Non-responsibilities.
4. Canonical authorities.
5. Public API.
6. Input and output contracts.
7. Dependencies.
8. Dependency direction.
9. Ownership and lifetime rules.
10. Error handling.
11. Determinism.
12. Concurrency.
13. Security.
14. Scalability.
15. Integration consumers.
16. Required tests.
17. Compatibility rules.
18. Completion criteria.

27.1 Freeze rule

A file is complete only when:

- Its public contract is defined.
- Its dependencies are known.
- Its downstream consumers are known.
- Its error model is agreed.
- Its integration behavior is tested.
- Its documentation matches its implementation.
- Its required tests pass.
- Its compatibility expectations are documented.

A file must not be declared frozen merely because its implementation compiles.

27.2 Contract changes

No architecture can guarantee that an implementation defect will never require a previously completed contract to change.

When an integration defect is discovered:

1. Identify the responsible authority.
2. Determine whether the defect is implementation-only or a contract defect.
3. Avoid unnecessary changes to unrelated files.
4. Record affected consumers.
5. Version incompatible contract changes.
6. Update the relevant tests.
7. Revalidate the integration chain.

This provides controlled evolution without repeated architectural redesign.

28. Production readiness requirements

The compiler may claim portability conformance only when the following conditions are demonstrated.

- One canonical source-language semantic authority.
- One canonical universal IR authority.
- Canonical domain IR integration.
- Verified semantics-preserving transformations.
- Explicit requirements and capabilities.
- Evidence-based feasibility.
- Open-world target and provider registration.
- Correct domain-specific lowering.
- Valid backend realization.
- Portable artifact contracts.
- Versioned serialization.
- Provenance preservation.
- Reproducible compilation.
- Deterministic behavior where required.
- Secure artifact validation.
- Cross-target conformance.
- Explicit unsupported-operation diagnostics.
- Correct resource exhaustion behavior.
- Incremental compilation correctness.
- No arbitrary universal hardware ceilings.
- No unsafe Rust.
- Passing integration and regression tests.

The existence of the corresponding files or modules is not sufficient evidence.

29. Required Rust quality gates

The repository must maintain these quality gates:

cargo fmt --all -- --check

cargo check --all-targets

cargo test --all-targets

cargo clippy --all-targets -- -D warnings

Additional requirements:

- Test the declared Rust minimum version.
- Test a current stable Rust toolchain.
- Validate all supported feature configurations.
- Validate public documentation.
- Verify that no "unsafe" code is introduced.
- Ensure broad lint suppressions do not hide incomplete compiler implementations.

Passing these commands is necessary but not sufficient for portability conformance.

30. Final architectural invariant

The complete Zamani portability architecture is:

                 ZAMANI SOURCE
                       |
                       v
                CANONICAL GRAMMAR
                       |
                       v
                    FRONTEND
                       |
                       v
              CANONICAL AST
                       |
                       v
              SEMANTIC AUTHORITY
                       |
                       v
           VALIDATED SEMANTIC MODEL
                       |
                       v
               CANONICAL ZUIR
                       |
              +--------+--------+
              |        |        |
              v        v        v
          CLASSICAL  QUANTUM   HDLIR
              |        |        |
              +--------+--------+
                       |
                       v
             OTHER DOMAIN IRs
                       |
                       v
                IR VERIFICATION
                       |
                       v
          SEMANTICS-PRESERVING PASSES
                       |
                       v
                  OPTIMIZATION
                       |
                       v
             CAPABILITY NEGOTIATION
                       |
                       v
              RESOURCE FEASIBILITY
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
             PORTABLE / TARGET-SPECIFIC
                  ARTIFACTS
                       |
                       v
              VALIDATION AND LINKING
                       |
                       v
                DEPLOYMENT HANDOFF
                       |
                       v
                    RUNTIME
                       |
                       v
              PHYSICAL EXECUTION

The compiler coordinates this architecture.

It does not replace the semantic authority, IR authorities, domain implementations, backend implementations or runtime.

The fundamental invariant is:

One Zamani program must retain its specified computational meaning across compatible realizations, regardless of changes in machine size, hardware architecture, execution location or available computational resources.

The compiler must support the smallest meaningful computation and scale progressively to arbitrarily large finite realizations without imposing arbitrary universal hardware ceilings.

Unknown capabilities must remain unknown.

Infeasible computations must be reported honestly.

Unsupported transformations must not silently change semantics.

Portable artifacts must retain their provenance and verification evidence.

Target-specific realizations must remain distinguishable from portable representations.

POCO-REAF is achieved through semantic preservation, extensible realization, reproducible artifacts and verified compatibility—not through pretending that every machine has identical capabilities.

---

END OF PORTABILITY CONTRACT

Canonical file: "src/compiler/documentation/PORTABILITY_CONTRACT.md"

Contract version: "1.0.0"

Rust baseline: "1.97+", edition "2021"

Safety: "#![forbid(unsafe_code)]"

Portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever"