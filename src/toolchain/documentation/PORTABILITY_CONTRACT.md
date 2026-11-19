Zamani Toolchain — Portability Contract

Canonical path: "src/toolchain/documentation/PORTABILITY_CONTRACT.md"
Repository: "github.com/Benwellonedge28/Zamani"
Branch: "main"
Document classification: Normative toolchain portability and integration contract
Language: Zamani
Rust edition: 2021
Minimum supported Rust: 1.97 or later
Safety: "unsafe" Rust is strictly prohibited
Architecture: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Primary objective: One portable program meaning, reusable compilation specifications, multiple verified target realizations, and execution wherever valid realizations and resources are available.

Scalability objective: From the smallest meaningful computation to arbitrarily large computational systems, constrained only by actual semantic requirements, implementation capabilities, available resources, representation limits, operational budgets, security policies, and physical reality.

Implementation status: This document defines the normative contract. Its existence does not certify that every repository component already implements or satisfies it.

---

1. Purpose

This document establishes the authoritative portability contract for Zamani's toolchain.

Its purpose is to ensure that the Zamani toolchain can coordinate the development, compilation, packaging, distribution, and deployment preparation of programs across compatible computational environments without requiring developers to rewrite their programs merely because the underlying hardware or execution environment changes.

The contract applies to:

1. Classical computing.
2. Systems programming.
3. Embedded computing.
4. Parallel and concurrent computing.
5. High-performance computing.
6. Numerical and scientific computing.
7. Quantum computing.
8. Quantum simulation.
9. Quantum error-corrected computation.
10. Classical-quantum hybrid computing.
11. Hardware description language (HDL).
12. Hardware/software co-design.
13. FPGA and ASIC development.
14. Artificial intelligence and machine learning.
15. Deep learning and neural networks.
16. Tensor and accelerator computing.
17. Symbolic and probabilistic computing.
18. Causal and reasoning systems.
19. Distributed computing.
20. Networking and communication.
21. Cryptographic and secure computation.
22. Edge and cloud computing.
23. Neuromorphic, photonic, analog, reversible, and future computing technologies.
24. Any additional computational domain implementing compatible Zamani contracts.

These are computing domains within one programming-language and toolchain ecosystem.

They MUST NOT require independent project-management, dependency-management, build, or package-management architectures merely because their computational semantics differ.

The fundamental principle is:

A Zamani program defines computational meaning. The toolchain coordinates its compilation and distribution. Authoritative compiler, backend, hardware, and runtime subsystems determine how that meaning can be realized using available capabilities and resources.

The toolchain MUST preserve this separation.

---

2. Normative language

The following terms have normative meanings:

- MUST / REQUIRED: Mandatory.
- MUST NOT / PROHIBITED: Forbidden.
- SHOULD: Strong recommendation. Deviations require documented technical justification.
- SHOULD NOT: Discouraged unless a documented technical justification exists.
- MAY: Optional.
- Portable program: A program whose specified meaning is not unnecessarily dependent on a particular target.
- Portable artifact: An artifact whose portability scope has been explicitly declared and established according to the applicable evidence requirements.
- Realization: A valid target-specific implementation of the program's semantics.
- Target: A canonical compilation or execution target identified by the authoritative target subsystem.
- Provider: An implementation registered against an applicable provider contract.
- Capability: An operation or property an implementation or environment supports.
- Requirement: A mandatory condition necessary for a valid computation or realization.
- Constraint: A condition that an acceptable realization must satisfy.
- Preference: A desirable but nonmandatory selection objective.
- Hint: Nonbinding guidance.
- Feasibility: An evidence-based determination of whether a requested realization can be produced under applicable requirements, capabilities, constraints, policies, and resources.
- Availability: An observation that a capability or resource is currently available.
- Reservation: An authoritative commitment to a resource for an operation.
- Execution: Actual operation through the appropriate runtime or physical infrastructure.
- Provenance: Verifiable information about the origin, inputs, transformations, authorities, and outputs of an operation.

These terms MUST NOT be treated as interchangeable.

---

3. Architectural authority

The toolchain is an orchestration and developer-infrastructure subsystem.

It MUST NOT become a second compiler, semantic analyzer, intermediate representation, hardware abstraction layer, or runtime.

The following ownership boundaries are mandatory.

Concern| Authoritative owner| Toolchain responsibility
Source grammar| "grammar/"| Consume language contracts
Lexing and parsing| "frontend/" and grammar authorities| Invoke
AST| "frontend/ast/"| Consume
Semantic meaning| "frontend::semantic"| Preserve
Requirements and capabilities| Semantic and resource authorities| Coordinate
Classical computation| "classical/"| Coordinate
Classical IR| Canonical classical IR| Consume
Quantum semantics| "quantum/"| Coordinate
Quantum IR| "quantum::ir"| Consume
HDL semantics| "hdl/"| Coordinate
AI and tensor semantics| "ai/" and canonical tensor infrastructure| Coordinate
Hybrid semantics| "hybrid/"| Coordinate
Distributed semantics| "distributed/"| Coordinate
Universal IR| "zuir/"| Consume
Compilation| "compiler/"| Configure and orchestrate
Compilation feasibility| "compiler/planning/"| Consume authoritative results
Backend target identity| "backend/target.rs"| Adapt without redefining
Backend compatibility| "backend/compatibility.rs"| Invoke
Target code generation| "backend/"| Invoke
Artifact portability classification| "compiler/artifacts/portability.rs"| Consume
Hardware capabilities| "zhal/"| Consume
Physical hardware resources| "zhal/"| Coordinate through approved services
Application execution| "zur/"| Hand off
Runtime portability testing| "zur/testing/portability.rs"| Coordinate
Project manifest| "zamani_project_config/"| Adapt
Dependency resolution| "toolchain/dependencies/"| Own
Project and workspace management| "toolchain/workspace/"| Own
Build orchestration| "toolchain/build/"| Own
Package distribution| "toolchain/packages/" and "registries/"| Own
Deployment orchestration| "toolchain/deployment/"| Coordinate
External executable discovery| Existing backend toolchain subsystem| Adapt
External process execution| "backend/process.rs"| Reuse

The toolchain MUST consume the canonical contracts of these subsystems.

It MUST NOT introduce competing implementations of their responsibilities.

3.1 Authority hierarchy

The following dependency direction MUST be preserved:

Zamani source
      |
      v
Grammar and language specifications
      |
      v
Frontend and AST
      |
      v
Semantic validation
      |
      v
Canonical semantic model
      |
      +-------------------------+
      |                         |
      v                         v
Classical IR                quantum::ir
      |                         |
      +------------+------------+
                   |
                   v
             ZUIR and
        domain-specific IRs
                   |
                   v
          Compiler and planning
                   |
                   v
      Optimization and lowering
                   |
          +--------+---------+
          |                  |
          v                  v
     Backend              Domain-specific
     realization          realization
          |                  |
          +--------+---------+
                   |
                   v
                  ZHAL
                   |
                   v
                  ZUR
                   |
                   v
            Physical execution

This is a conceptual dependency model, not a requirement that every domain follow identical internal passes.

The toolchain coordinates the appropriate services without changing their ownership.

---

4. Meaning of POCO-REAF

POCO-REAF consists of five related guarantees.

4.1 Program Once

Program Once means that developers express the intended computation once in Zamani.

A portable program MUST describe its intended semantics without unnecessarily depending on:

- Processor architecture.
- Processor count.
- Core count.
- Thread count.
- GPU vendor.
- Accelerator identity.
- FPGA implementation.
- ASIC implementation.
- Quantum processor identity.
- Physical qubit numbering.
- Machine memory capacity.
- Deployment location.
- Network topology.
- Cluster size.
- Operating system.
- Particular backend implementation.

The program MAY explicitly declare requirements and constraints where those characteristics are genuinely part of the intended computation.

A source program SHOULD remain unchanged when the only change is the available compatible target or resource capacity.

4.2 Compile Once

Compile Once means that the same validated program meaning and reusable compilation specification can be consumed by compatible compiler implementations and realization environments without requiring source redesign for each target.

The toolchain MUST distinguish:

1. Source compilation.
2. Portable semantic representation.
3. Portable compilation artifact.
4. Target-specific specialization.
5. Target-specific lowering.
6. Target-specific artifact generation.
7. Artifact verification.
8. Runtime execution.

These stages MUST NOT be represented as equivalent operations.

The same portable program may require different target-specific artifacts.

For example:

One Zamani source
        |
        v
One validated semantic meaning
        |
        v
One reusable portable representation
        |
        +-------------------+
        |                   |
        v                   v
    CPU realization     GPU realization
        |                   |
        v                   v
    Native artifact     Accelerator artifact

A target-specific artifact is not necessarily interchangeable with another target-specific artifact.

A native executable for one architecture cannot automatically be assumed executable on every other architecture.

The toolchain MUST preserve the relationship between the common source/build identity and each target-specific realization.

Compile Once does not mean that one physical compilation operation can produce a universal executable for every conceivable architecture.

It means that the source and portable semantic contract are reusable, with target-specific compilation or specialization performed when required.

4.3 Run Everywhere

Run Everywhere means that any compatible supported target can be considered for a valid realization of the program.

A target is eligible only when the relevant authorities establish that the requested semantics can be preserved.

Examples include:

- Embedded processors.
- Single-core and multicore processors.
- CPUs.
- GPUs.
- FPGAs.
- ASICs.
- QPUs.
- Quantum simulators.
- Heterogeneous systems.
- Distributed clusters.
- HPC environments.
- Edge devices.
- Cloud environments.
- Future registered computing technologies.

These examples do not establish that every such backend currently exists or is production-ready.

4.4 Run Anywhere

Run Anywhere means that a portable program can be considered for valid execution in different locations and deployment environments.

A program MUST NOT depend on a particular physical location unless that dependency is explicitly part of its semantics or constraints.

External conditions may include:

- Permissions.
- Network connectivity.
- Provider availability.
- Data access.
- Hardware availability.
- Deployment policies.
- Resource availability.
- Execution deadlines.
- Security requirements.

The appropriate authorities MUST evaluate these conditions.

4.5 Run Forever

Run Forever means that Zamani's language, artifacts, and toolchain are designed to evolve across future implementations and computational architectures.

It does not mean that an unchanged executable or device will operate indefinitely.

Long-term portability requires:

- Versioned language contracts.
- Versioned semantic contracts.
- Versioned IR schemas.
- Versioned artifact formats.
- Explicit provider API compatibility.
- Stable semantic behavior.
- Deprecation procedures.
- Migration procedures.
- Reproducible metadata.
- Compatibility validation.
- Provenance preservation.

Unknown or incompatible semantic versions MUST NOT be silently accepted as compatible.

---

5. Portability classification

Portability MUST be evaluated at multiple distinct levels.

A program's portability does not automatically establish the portability of every artifact derived from it.

The following classifications MUST remain distinguishable.

5.1 Semantic portability

Semantic portability concerns the preservation of the program's specified meaning independently of a particular target.

The canonical semantic authorities establish the meaning.

The toolchain MUST NOT infer semantic portability merely because:

- Source code parses.
- Compilation succeeds.
- An artifact has no target identifier.
- An artifact is labeled universal.
- A backend advertises support.
- A provider claims compatibility.

Semantic portability requires the appropriate semantic validation and verification evidence.

5.2 Representation portability

Representation portability concerns whether a semantic representation can be consumed by compatible implementations.

A representation MUST identify its applicable:

- Language version.
- Semantic contract version.
- IR schema version.
- Domain and dialect versions.
- Required features.
- Required interfaces.
- Serialization format, where applicable.
- Compatibility requirements.

A representation MUST NOT depend on undocumented compiler memory layouts or process-local state.

5.3 Artifact portability

Artifact portability concerns the scope in which a generated artifact can be used.

The authoritative implementation is:

"src/compiler/artifacts/portability.rs"

The toolchain MUST consume its classifications and evidence.

It MUST NOT implement a competing artifact-portability classifier.

An artifact MUST NOT automatically be considered portable because its manifest lacks a target association.

An explicit declaration is not equivalent to independent verification.

5.4 Target portability

Target portability concerns whether an artifact or reusable representation can be realized for a specified target.

The authoritative compiler and backend systems determine compatibility and feasibility.

The toolchain coordinates requests and records their outcomes.

5.5 Execution portability

Execution portability concerns whether equivalent required program behavior is demonstrated across actual execution environments.

The existing authority is:

"src/zur/testing/portability.rs"

The toolchain MUST consume actual execution-portability reports rather than manufacture portability claims from successful builds.

5.6 Portability status

The following conceptual status model MUST be preserved.

Status| Meaning
Declared| Producer has stated a portability intention
Validated| Applicable structural and semantic validation has completed
Evidence-backed| Relevant authoritative evidence is available
Verified| The required verification authority has established the claim
Partially supported| Some requested portability conditions are established, others are not
Indeterminate| Available evidence cannot establish the result
Incompatible| Evidence establishes that the relevant requirements cannot be satisfied
Expired or stale| Previously supplied evidence is no longer sufficiently current for the required operation

These are conceptual distinctions. Existing subsystem enums remain authoritative where they already define corresponding statuses.

The toolchain MUST NOT introduce a new universal enum merely to duplicate existing status models.

---

6. Fundamental portability invariant

The following invariant is mandatory:

The same source program MUST preserve its specified semantic meaning across every valid realization, subject to its declared semantic contracts.

Different targets may produce different:

- Machine instructions.
- Intermediate representations.
- Binary formats.
- Gate decompositions.
- Hardware descriptions.
- Execution plans.
- Memory layouts.
- Scheduling decisions.
- Communication plans.
- Physical mappings.
- Runtime implementations.

These differences are permitted when the transformations preserve the required program semantics.

A target-specific implementation MUST NOT silently change:

- Type correctness.
- Observable behavior.
- Required effects.
- Data dependencies.
- Memory safety.
- Numerical guarantees.
- Quantum measurement semantics.
- Quantum state semantics.
- HDL clocking and timing semantics.
- Distributed consistency guarantees.
- Security properties.
- Explicit determinism requirements.
- Explicit ordering requirements.

Where the language permits implementation-dependent or nondeterministic behavior, the applicable contract MUST define the permitted variation.

---

7. Requirements, capabilities, preferences, constraints and hints

The toolchain MUST preserve the five-way distinction established by the grammar and semantic model.

7.1 Requirements

A requirement expresses something necessary for valid realization.

Examples:

requires capability("quantum.measurement");
requires capability("tensor.compute");
requires memory >= required_memory;
requires qubits >= logical_qubits;

These are illustrative expressions. Their actual syntax is owned by the grammar.

An unmet mandatory requirement MUST prevent the corresponding realization from being classified as feasible.

7.2 Capabilities

A capability describes an operation or property that an implementation or environment supports.

Capability declarations and evidence belong to the canonical capability authorities.

The toolchain MUST NOT invent capabilities from:

- Vendor names.
- Device names.
- Marketing descriptions.
- Target labels.
- Artifact names.
- Provider self-description without validation.

7.3 Preferences

Preferences influence selection among otherwise valid realizations.

Examples:

- Prefer lower energy consumption.
- Prefer lower latency.
- Prefer an available accelerator.
- Prefer a trusted provider.
- Prefer a specific execution environment.

A preference MUST NOT override mandatory requirements or semantic correctness.

7.4 Constraints

Constraints restrict the set of valid realizations.

A realization violating a mandatory constraint MUST NOT be selected as valid.

7.5 Hints

Hints are nonbinding optimization guidance.

Ignoring a hint MUST NOT change the specified program meaning.

7.6 Operational budgets

Operational budgets are different from language-level capacity ceilings.

Examples include:

- Maximum build duration.
- Maximum download size.
- Maximum memory allocated to a build.
- Maximum concurrent tasks.
- Maximum retry count.
- Maximum archive extraction size.
- Maximum retained diagnostics.
- Maximum deployment cost.
- Maximum network consumption.

These budgets MUST be explicitly configured, scoped, and reported.

They MUST NOT silently become universal semantic limits.

---

8. Resource-parametric scalability

Zamani MUST support programs whose scale is determined by semantic requirements, input dimensions, capabilities and actual resource availability.

The toolchain MUST NOT impose arbitrary universal ceilings on:

- Project size.
- Workspace members.
- Source files.
- Functions.
- Modules.
- Dependencies.
- Build tasks.
- Target count.
- CPUs.
- Cores.
- Threads.
- GPUs.
- FPGAs.
- ASIC resources.
- QPUs.
- Logical qubits.
- Physical qubits.
- Memory.
- Storage.
- Register widths.
- Tensor dimensions.
- Tensor rank.
- Network participants.
- Distributed nodes.
- Processes.
- Operations.
- Hardware modules.
- Signals.
- Ports.
- Pipelines.
- Computational domains.
- Future provider populations.

The prohibition includes disguised restrictions such as:

- Fixed-size global registries.
- Arbitrary fixed arrays.
- Closed universal hardware enumerations.
- Hidden parser capacity ceilings.
- Fixed-size target matrices.
- Fixed-size dependency collections.
- Hard-coded worker populations.
- Fixed hardware topology assumptions.
- Undocumented maximum resource quantities.
- Serialization formats with arbitrary domain capacity limits.

8.1 Meaning of infinity

Infinity is an architectural scalability objective.

It means that Zamani does not impose arbitrary universal finite capacity ceilings on computational semantics.

It does not mean that physical hardware has infinite resources.

Every concrete computation remains subject to:

- Available memory.
- Available storage.
- Address-space limits.
- Representation limits.
- Processing capacity.
- Time.
- Energy.
- Network bandwidth.
- Provider capabilities.
- Physical laws.
- Security policies.
- Explicit operational budgets.

The toolchain MUST distinguish:

Language limitation
        !=
Implementation limitation
        !=
Representation limitation
        !=
Resource exhaustion
        !=
Capability incompatibility
        !=
Operational policy rejection

These conditions MUST produce appropriately classified diagnostics.

8.2 Arbitrary-precision resource quantities

Where quantities may exceed machine integer widths, the owning semantic, compiler or resource subsystem MUST use an appropriate representation.

The existing compiler feasibility subsystem provides an example through its arbitrary-length decimal "ResourceQuantity".

The toolchain MUST NOT convert such values into "usize", "u64" or another bounded integer without a checked conversion and explicit failure behavior.

A finite machine representation is acceptable where it is genuinely required, but its limit MUST be documented as an implementation or representation constraint rather than a language-wide capacity ceiling.

8.3 Tiny-to-large operation

The toolchain MUST accommodate:

- Minimal projects.
- Small embedded applications.
- Single-device programs.
- Multicore programs.
- Heterogeneous applications.
- Large workspaces.
- Distributed applications.
- Large-scale computational projects.
- Future computational environments.

The architecture SHOULD avoid unnecessary heavyweight services for tiny projects while permitting additional orchestration and parallelism when resources justify them.

This does not require every small project to initialize every provider or domain subsystem.

---

9. Open-world target and provider architecture

The toolchain MUST support extensible target and provider identities.

It MUST NOT encode every future architecture, vendor, processor, accelerator, QPU, operating system or deployment environment into a closed universal enumeration.

The existing target authorities are:

- "src/compiler/configuration/target.rs"
- "src/backend/target.rs"
- "src/backend/registry.rs"
- "src/backend/capabilities.rs"
- "src/backend/compatibility.rs"
- "src/compiler/planning/"

The toolchain's target subsystem adapts these authorities.

9.1 Target identity consistency

The repository currently has separate compiler-facing and backend-facing target identities.

For example:

- "src/compiler/configuration/target.rs"
- "src/backend/target.rs"

The toolchain MUST NOT assume that two target IDs are interchangeable merely because their display strings look similar.

A canonical, explicit, validated mapping MUST be used wherever the two identity systems meet.

The mapping MUST preserve:

- Source identity.
- Destination identity.
- Mapping authority.
- Mapping version.
- Compatibility information.
- Provenance.
- Validation outcome.

An absent or ambiguous mapping MUST result in an unresolved or incompatible condition, as appropriate.

The target identity boundary MUST be reconciled across:

- "src/toolchain/targets/matrix.rs"
- "src/toolchain/targets/compatibility.rs"
- "src/toolchain/targets/selection.rs"
- "src/toolchain/targets/validation.rs"
- "src/toolchain/build/plan.rs"

This is particularly important because the target matrix and build-plan code must not independently establish conflicting canonical target identities.

9.2 Provider registration

Providers MUST implement explicit versioned contracts.

A provider MUST establish:

- Its identity.
- Its supported interfaces.
- Its interface versions.
- Its declared capabilities.
- The evidence supporting those capabilities.
- Its operational requirements.
- Its compatibility behavior.
- Its security and trust requirements.
- Its error behavior.
- Its lifecycle behavior.

Provider registration MUST NOT automatically establish capability, availability, feasibility or trust.

9.3 Future targets

Adding a new target SHOULD require registering compatible implementations and contracts rather than redesigning the toolchain's core architecture.

Examples include:

- New CPU architectures.
- New accelerators.
- New quantum technologies.
- New FPGA architectures.
- New distributed environments.
- Neuromorphic devices.
- Photonic processors.
- Future computational substrates.

The toolchain MUST remain open to new targets without changing the meaning of existing portable programs.

---

10. Target selection and feasibility

Target selection and feasibility MUST remain separate.

The toolchain coordinates selection requests, but the canonical compiler, backend and hardware authorities establish the relevant facts.

The required conceptual process is:

Portable build request
        |
        v
Resolve target intent
        |
        v
Discover candidate targets
        |
        v
Obtain capability evidence
        |
        v
Evaluate requirements
        |
        v
Evaluate constraints
        |
        v
Evaluate resource feasibility
        |
        v
Apply valid preferences
        |
        v
Produce realization plan
        |
        v
Revalidate changing assumptions
        |
        v
Resource acquisition
        |
        v
Execution

10.1 Feasibility classifications

The toolchain MUST preserve at least these conceptual outcomes:

Feasible: Authoritative evidence establishes that the requested realization can satisfy the evaluated requirements under the applicable policy and assessment scope.

Infeasible: Sufficient evidence establishes that a mandatory requirement or constraint cannot be satisfied.

Indeterminate: Evidence is missing, incomplete, stale or insufficient to establish feasibility.

Partially feasible: Some requested target realizations are feasible while others are not.

The toolchain MUST NOT convert indeterminate results into feasible results.

A successful compatibility assessment MUST NOT automatically become a successful resource-feasibility assessment.

10.2 Compatibility versus feasibility

The following distinctions are mandatory:

Target identity
    !=
Target descriptor

Target descriptor
    !=
Capability declaration

Capability declaration
    !=
Capability evidence

Capability evidence
    !=
Current availability

Availability
    !=
Compatibility

Compatibility
    !=
Feasibility

Feasibility
    !=
Resource reservation

Resource reservation
    !=
Execution success

A target descriptor does not establish that a target is physically available.

A compatibility report does not establish that all required resources can be acquired.

A feasibility report does not guarantee that conditions will remain unchanged until execution.

10.3 Revalidation

Because resource availability and provider conditions can change, the runtime or resource authority MUST revalidate relevant assumptions before actual execution.

The toolchain MUST preserve the relationship between the original assessment and any later reassessment.

---

11. Portable build identity

A single logical build may produce multiple target-specific realizations.

The toolchain MUST preserve a shared logical identity across those realizations.

The identity model MUST distinguish:

1. Project identity.
2. Workspace identity.
3. Source identity.
4. Dependency identity.
5. Configuration identity.
6. Logical build identity.
7. Target identity.
8. Target-realization identity.
9. Artifact identity.
10. Execution identity.

These identities MUST NOT be conflated.

11.1 Shared identity

The common logical build identity MUST describe the validated common inputs.

These may include:

- Source contents.
- Source tree structure.
- Project manifest.
- Resolved dependencies.
- Feature selections.
- Language version.
- Semantic contract version.
- Compiler configuration.
- Build profile.
- Relevant policies.
- Required interfaces.
- Reproducibility requirements.

11.2 Target-specific realization identity

Each realization MUST additionally identify relevant target-dependent inputs.

These may include:

- Canonical target identity.
- Backend implementation identity.
- Backend version.
- Provider identity.
- Provider interface version.
- Target capability evidence.
- Target-specific compiler configuration.
- External toolchain versions.
- Relevant environment fingerprint.
- Target-specific lowering decisions.
- Relevant calibration or hardware information.
- Target-specific ABI.
- Artifact format.
- Realization provenance.

11.3 Identity invariant

The same logical build identity MUST NOT imply that different target artifacts are byte-for-byte identical.

Instead, it establishes that they derive from the same validated logical inputs under the recorded build contract.

The toolchain MUST be able to trace each target artifact back to its common source and dependency identity.

---

12. Domain portability contracts

The common toolchain MUST support all registered computational domains through one coordinated project, dependency and build architecture.

It MUST NOT create independent universal build systems for each domain.

12.1 Classical computing

The toolchain MUST support:

- Native compilation.
- Cross-compilation.
- Embedded builds.
- Single-core and multicore execution targets.
- Parallel computation.
- Scientific and numerical applications.
- Systems applications.
- Libraries and executables.
- Portable target profiles.
- Target-specific artifacts.

Integration authorities:

- "src/classical/"
- "src/classical/zcir/"
- "src/compiler/"
- "src/backend/"
- "src/zur/"

Classical computational semantics remain owned by the classical subsystem.

The toolchain MUST NOT redefine classical IR, memory semantics, numerical semantics or scheduling.

12.2 Quantum computing

The toolchain MUST support:

- Quantum source projects.
- Quantum libraries and dependencies.
- Quantum simulation workflows.
- Quantum compilation profiles.
- Quantum artifact packaging.
- Registered QASM and other supported interchange formats.
- QPU provider integration.
- Quantum capability compatibility.
- Quantum resource requirements.
- Quantum execution handoff.
- Quantum-specific provenance.

Integration authorities:

- "src/quantum/"
- "src/quantum/ir/"
- "src/quantum/zqn/"
- "src/quantum/qec/"
- "src/quantum/scheduling/"
- "src/quantum/hardware/"
- "src/backend/quantum/"
- "src/zhal/adapters/quantum.rs"
- "src/zur/quantum.rs"

The toolchain MUST NOT implement:

- Quantum gates.
- Quantum states.
- Quantum measurement semantics.
- Quantum routing.
- Qubit allocation.
- Quantum error correction.
- Quantum noise modeling.
- Quantum calibration.
- Quantum scheduling.

These remain with their authoritative quantum subsystems.

The canonical quantum semantic boundary remains "quantum::ir".

A logical quantum resource MUST NOT be treated as identical to a physical qubit.

A quantum simulator's capability MUST NOT automatically establish physical QPU capability.

12.3 HDL and hardware computing

The toolchain MUST support:

- HDL projects.
- Hardware/software co-design.
- Hardware design dependencies.
- Synthesis workflows.
- Simulation workflows.
- Verification workflows.
- FPGA workflows.
- ASIC workflows.
- Hardware artifact packaging.
- Hardware provider integration.
- Timing and verification evidence.

Integration authorities:

- "src/hdl/"
- "src/backend/hdl/"
- "src/zhal/adapters/hdl.rs"
- Existing synthesis and simulation provider contracts.

The toolchain MUST NOT implement physical synthesis, placement, routing, timing closure or bitstream generation itself.

An HDL artifact MUST carry its relevant design and toolchain compatibility information.

Hardware timing requirements MUST NOT be silently weakened to make a target appear compatible.

12.4 AI and tensor computing

The toolchain MUST support:

- AI and machine-learning projects.
- Model dependencies.
- Dataset references.
- Training workflows.
- Inference workflows.
- Tensor computation.
- Accelerator-specific artifacts.
- Model packaging.
- Edge deployment.
- Distributed training.
- Model provenance.

Integration authorities:

- "src/ai/"
- "src/classical/collections/tensor.rs"
- "src/zuir/dialects/ai/"
- Existing compiler and backend tensor infrastructure.
- "src/zhal/adapters/accelerator.rs"
- "src/zur/"

The toolchain MUST reuse canonical tensor and model representations.

It MUST NOT create a competing tensor storage model or AI semantic system.

12.5 Hybrid computing

The toolchain MUST support projects containing multiple computational domains.

Examples:

- Classical plus quantum.
- Classical plus HDL.
- Quantum plus HDL.
- Classical plus AI.
- Quantum plus AI.
- Classical plus quantum plus distributed.
- Classical plus hardware plus AI.
- Multiple domains combined in a single application.

Hybrid projects MUST preserve domain boundaries and explicit interfaces.

Integration authorities:

- "src/hybrid/"
- "src/quantum/"
- "src/classical/"
- "src/hdl/"
- "src/ai/"
- "src/zur/hybrid/"
- "grammar/hybrid/"

The toolchain MUST NOT create a competing hybrid IR.

A hybrid build may produce multiple coordinated artifacts rather than forcing all domains into one artificial executable format.

12.6 Distributed computing

The toolchain MUST support:

- Distributed applications.
- Multi-node compilation workflows.
- Distributed package dependencies.
- Cluster deployment descriptions.
- Distributed artifact management.
- Communication contracts.
- Serialization compatibility.
- Reproducibility.
- Distributed execution handoff.
- Failure and recovery reporting.

Integration authorities:

- "src/distributed/"
- "src/zur/distributed/"
- "src/zur/scheduling/"
- "src/zhal/adapters/distributed.rs"
- "grammar/distributed/"
- "grammar/execution/distributed-execution.g4"

The toolchain MUST NOT implement distributed scheduling, physical node allocation, communication routing or runtime recovery as competing subsystems.

12.7 Future domains

New computational domains MUST be integrable through open-world contracts.

The toolchain MUST NOT require changes to a universal hard-coded domain enumeration whenever a new technology is introduced.

A future domain must establish:

- Domain identity.
- Semantic contract.
- Relevant IR or interchange contract.
- Required capabilities.
- Resource requirements.
- Provider interfaces.
- Compatibility requirements.
- Artifact contract.
- Runtime or realization handoff.
- Portability tests.

---

13. Target matrices and partial success

The toolchain supports one logical build request with multiple target realizations.

The existing target-matrix implementation is:

"src/toolchain/targets/matrix.rs"

It owns matrix identity, entries, assessment association and deterministic aggregation.

The toolchain MUST preserve these responsibilities.

13.1 Matrix semantics

A target matrix MUST:

- Preserve the logical build identity.
- Preserve source identity.
- Preserve dependency identity.
- Identify every requested target.
- Preserve target-specific assessments.
- Preserve unresolved target requests.
- Preserve assessment provenance.
- Report incomplete assessment.
- Distinguish partial success from total success.
- Avoid inventing feasibility.

13.2 Partial success

Consider:

One logical build
       |
       +---- Target A: feasible
       |
       +---- Target B: infeasible
       |
       +---- Target C: indeterminate
       |
       +---- Target D: feasible

The overall result MUST preserve all four outcomes.

It MUST NOT report complete multi-target success merely because some targets succeeded.

The caller's declared success policy determines whether partial success is acceptable.

13.3 Transactional integrity

A failed target realization MUST NOT corrupt successfully verified outputs for unrelated targets.

The toolchain SHOULD support independent realization tasks, failure isolation and deterministic aggregation.

---

14. Numerical and computational semantic preservation

Portability is not simply successful compilation.

Different targets may have different numerical characteristics.

The compiler and domain authorities MUST preserve the specified numerical contract.

Relevant concerns include:

- Integer overflow.
- Integer widths.
- Floating-point precision.
- Rounding.
- Floating-point exceptions.
- Denormal behavior.
- Fused operations.
- Reduction order.
- Parallel floating-point accumulation.
- Reproducibility.
- Approximation bounds.
- Probability distributions.
- Random-number semantics.
- Nondeterministic execution.
- Quantum measurement.
- Statistical sampling.

The toolchain MUST record relevant compilation and realization requirements.

It MUST NOT silently weaken accuracy or reproducibility guarantees.

Where an optimization changes numerical results within a permitted tolerance, the applicable language or numerical contract MUST explicitly authorize that variation.

A backend's inability to meet a numerical guarantee MUST be reported as incompatibility or infeasibility.

---

15. Quantum semantic preservation

Quantum portability requires additional distinctions.

The toolchain MUST preserve the distinction between:

- Logical quantum operations.
- Physical quantum operations.
- Logical qubits.
- Physical qubits.
- Quantum states.
- Measurement outcomes.
- Noise models.
- Error-correction strategies.
- Hardware calibration.
- Routing.
- Scheduling.
- Simulation.

A quantum artifact MUST NOT be declared physically portable merely because a quantum simulator accepts it.

The toolchain MUST preserve explicit quantum execution requirements and relevant evidence.

Automatic measurement, silent gate substitution, silent approximation and unauthorized changes to quantum semantics are prohibited.

The appropriate quantum subsystem decides whether a transformation is valid.

---

16. HDL semantic preservation

HDL portability has a different realization model from ordinary executable portability.

An HDL design may be realized through:

- Simulation.
- Synthesis.
- Formal verification.
- FPGA implementation.
- ASIC implementation.
- Hardware/software co-design.

The toolchain MUST distinguish a source design from its synthesis artifact and its physical implementation.

It MUST preserve relevant:

- Clocking contracts.
- Timing requirements.
- Reset behavior.
- Signal semantics.
- Interface contracts.
- Parameterization.
- Verification requirements.
- Hardware constraints.
- Physical implementation evidence.

A successful HDL parser invocation MUST NOT be reported as successful synthesis.

A successful synthesis MUST NOT be reported as completed physical implementation.

A generated bitstream MUST NOT be assumed compatible with an unrelated FPGA.

---

17. Distributed semantic preservation

Distributed programs have observable behavior that may depend on communication and consistency semantics.

The toolchain MUST preserve:

- Message contracts.
- Serialization contracts.
- Ordering guarantees.
- Consistency requirements.
- Synchronization.
- Failure semantics.
- Recovery contracts.
- Security and authentication.
- Data provenance.
- Idempotency requirements where declared.

Changing the number of distributed participants MUST NOT silently change the program's specified meaning.

Different implementations may use different topologies and placement strategies where permitted.

A deployment with more nodes is not automatically equivalent to a deployment with fewer nodes.

The applicable distributed semantic contract determines whether the realizations are equivalent.

---

18. Reproducibility and determinism

The toolchain MUST preserve reproducibility information across target realizations.

A reproducible build MUST identify all inputs that can materially affect the result.

These may include:

- Source content.
- Manifest content.
- Resolved dependency graph.
- Dependency checksums.
- Feature selection.
- Compiler version.
- Compiler configuration.
- Backend version.
- External toolchain versions.
- Provider identities.
- Target identity.
- Target-specific configuration.
- Relevant environment properties.
- Build scripts and generated inputs.
- Relevant policies.
- Artifact format version.

18.1 Build identity versus artifact identity

The logical build identity MUST remain distinct from each artifact identity.

Two builds with equivalent source identity but different target configurations may produce different artifacts.

The toolchain MUST preserve this distinction.

18.2 Incremental compilation

The toolchain MUST reuse the existing compiler incremental compilation and artifact-cache authorities.

A cached result MUST NOT be reused when a relevant input, contract, target property or provider version invalidates its compatibility.

Cache invalidation MUST be based on complete relevant input identity rather than assumptions about the current machine.

18.3 Deterministic ordering

Where deterministic results are required, target matrices, dependency graphs, provider records and build reports MUST use stable ordering.

No result may depend accidentally on hash-map iteration order or provider registration order.

---

19. Security and trust

Portability MUST NOT weaken security.

The toolchain MUST preserve:

- Dependency integrity.
- Artifact integrity.
- Signature verification.
- Provenance.
- Provider trust.
- Authentication.
- Authorization.
- Credential isolation.
- Reproducible verification.
- Supply-chain policies.
- Isolation requirements.

An artifact is not trusted merely because it is portable.

A provider is not trusted merely because it is registered.

A capability claim is not authenticated merely because it appears in a provider response.

A successful build does not establish that an artifact is safe to execute.

The toolchain MUST use existing security and cryptographic authorities rather than implementing competing cryptographic mechanisms.

Untrusted artifacts and provider responses MUST be validated before consumption.

---

20. Artifact and package portability

The toolchain MUST distinguish source packages, portable semantic artifacts, target-specific compilation artifacts and deployment packages.

Each artifact MUST carry or reference sufficient information to establish its compatibility scope.

Depending on artifact type, relevant metadata includes:

- Artifact identity.
- Artifact format and schema version.
- Source/build identity.
- Language version.
- Semantic contract version.
- Domain and dialect versions.
- Target identity, where applicable.
- ABI, where applicable.
- Compiler identity.
- Backend identity.
- Provider identity.
- Dependency identity.
- Required capabilities.
- Required runtime interfaces.
- Provenance.
- Integrity information.
- Signature information, where required.
- Portability declaration.
- Verification evidence references.

The exact schema remains owned by the relevant artifact and package-format authorities.

The toolchain MUST NOT invent a second serialization format to duplicate those authorities.

20.1 Package portability

A source package SHOULD remain usable across compatible environments.

A target-specific binary package MUST identify its compatibility requirements.

An installation operation MUST verify the package's applicable requirements before declaring installation success.

A successful package download MUST NOT automatically establish successful installation or execution.

---

21. Versioning and long-term compatibility

The toolchain MUST distinguish:

- Zamani language version.
- Grammar version.
- Semantic contract version.
- Compiler API version.
- Compiler implementation version.
- IR schema version.
- Artifact format version.
- Package format version.
- Backend API version.
- Provider API version.
- Runtime API version.
- Hardware firmware version.

These versions MUST NOT be conflated.

The existing toolchain version authorities include:

- "src/toolchain/version.rs"
- "src/toolchain/api/versioning.rs"
- "src/toolchain/api/compatibility.rs"

The toolchain MUST use those authorities.

21.1 Unknown versions

An unknown major semantic or artifact-format version MUST NOT be silently interpreted as compatible.

Compatibility MUST be established through explicit version policies.

21.2 Migration

When an artifact or project requires migration:

- The original identity MUST be preserved.
- The migration procedure MUST be explicit.
- The resulting identity MUST be recorded.
- The migration authority MUST be identified.
- Semantic preservation MUST be verified where required.
- Migration diagnostics MUST be available.

21.3 Forever compatibility

Zamani MUST prioritize stable semantics and explicit migration over promising that every historical binary will remain executable indefinitely.

---

22. Toolchain portability workflow

The canonical toolchain workflow is:

Zamani project
      |
      v
Workspace discovery
      |
      v
Canonical manifest loading
      |
      v
Manifest validation
      |
      v
Dependency resolution
      |
      v
Portable build request
      |
      v
Build-plan construction
      |
      v
Target resolution
      |
      v
Authoritative feasibility assessment
      |
      v
Build-input identity
      |
      v
Build-task graph
      |
      v
Compiler API
      |
      v
Target-specific realization
      |
      v
Backend and provider integration
      |
      v
Artifact validation
      |
      v
Portability assessment
      |
      v
Build report and provenance
      |
      v
Package / install / deploy / run

The toolchain MUST preserve each stage's actual outcome.

It MUST NOT skip required validation stages merely to produce a successful-looking result.

---

23. Execution handoff

The toolchain does not own application execution.

Execution belongs to "src/zur/" and its authoritative providers.

The handoff MUST distinguish:

1. Project validation succeeded.
2. Dependencies resolved.
3. Build plan validated.
4. Feasibility established.
5. Compilation succeeded.
6. Artifact generation succeeded.
7. Artifact integrity verified.
8. Execution environment validated.
9. Resources acquired or reserved.
10. Execution submitted.
11. Execution started.
12. Execution completed.
13. Required postconditions verified.

These are separate outcomes.

The toolchain MUST NOT report execution success when it has only compiled an artifact.

It MUST NOT report resource acquisition merely because a resource was observed as available.

---

24. Required integration with existing toolchain files

The following integration requirements apply to the current repository architecture.

The contract does not require creating duplicate files where the relevant implementation already exists.

Existing file| Required portability integration
"src/toolchain/mod.rs"| Expose the toolchain's deliberately selected public modules and APIs. Preserve the established ownership boundaries.
"src/toolchain/api/request.rs"| Preserve portable target intent, logical build identity inputs and explicit operational budgets.
"src/toolchain/api/response.rs"| Report structured outcomes without claiming universal portability from compilation success.
"src/toolchain/api/capabilities.rs"| Adapt canonical capability information without creating a second capability registry.
"src/toolchain/api/compatibility.rs"| Coordinate version and contract compatibility.
"src/toolchain/api/versioning.rs"| Maintain independent version authorities.
"src/toolchain/configuration/config.rs"| Configure portability-related policies and defaults without machine-derived semantic ceilings.
"src/toolchain/configuration/policies.rs"| Define explicit acceptance, fallback, trust and portability policies.
"src/toolchain/configuration/profiles.rs"| Allow reusable, extensible portability and realization profiles.
"src/toolchain/configuration/budget.rs"| Represent configurable operational budgets, not universal capacity limits.
"src/toolchain/workspace/"| Preserve common project identity across workspace members and target realizations.
"src/toolchain/manifest/adapter.rs"| Adapt the canonical project manifest without redefining its schema.
"src/toolchain/build/request.rs"| Preserve immutable build intent and avoid reinterpreting semantic requirements.
"src/toolchain/build/plan.rs"| Preserve logical build identity and authoritative feasibility evidence.
"src/toolchain/build/graph.rs"| Coordinate realization tasks without introducing compiler scheduling semantics.
"src/toolchain/build/executor.rs"| Invoke actual compiler and backend services. Never fabricate successful outputs.
"src/toolchain/build/inputs.rs"| Track all relevant portable and target-specific build inputs.
"src/toolchain/build/outputs.rs"| Collect verified output references and retain their compatibility scope.
"src/toolchain/build/incremental.rs"| Reuse compiler incremental compilation and cache invalidation.
"src/toolchain/build/reporting.rs"| Preserve per-target outcomes, partial success and provenance.
"src/toolchain/targets/profiles.rs"| Define reusable target profiles without closing the target universe.
"src/toolchain/targets/matrix.rs"| Coordinate one logical build across multiple target-specific assessments.
"src/toolchain/targets/compatibility.rs"| Delegate compatibility evaluation to backend authorities.
"src/toolchain/targets/selection.rs"| Preserve explicit selection policy and never equate selection with feasibility.
"src/toolchain/targets/validation.rs"| Validate requests, target mappings and applicable compatibility contracts.
"src/toolchain/environments/fingerprints.rs"| Produce deterministic environment fingerprints without embedding transient data unnecessarily.
"src/toolchain/environments/discovery.rs"| Coordinate environment discovery through authoritative providers.
"src/toolchain/external_tools/"| Reuse backend external-tool discovery and process execution.
"src/toolchain/deployment/"| Coordinate deployment intent, feasibility and provider handoff.
"src/toolchain/security/"| Reuse cryptographic, integrity and trust authorities.
"src/toolchain/diagnostics/"| Preserve distinctions between semantic, compatibility, feasibility, resource and execution failures.
"src/toolchain/agents/"| Give authorized AI agents the same portability contracts and operation permissions as human developers.

24.1 Critical target-identity integration

The current repository has both compiler-facing and backend-facing target identities.

The target-matrix and build-plan implementations must not independently invent their own identity mapping.

Before freezing these files, the implementation MUST establish one explicit validated integration contract for:

- "compiler::configuration::target"
- "backend::target"
- "toolchain::targets::matrix"
- "toolchain::targets::compatibility"
- "toolchain::targets::selection"
- "toolchain::build::plan"

This is a required integration correction, not an optional enhancement.

24.2 Critical root-module integration

The inspected current "src/toolchain/mod.rs" is empty.

The existence of implementation files in subdirectories does not mean they are integrated into the crate.

The toolchain root and "src/lib.rs" MUST be reconciled before the toolchain can be considered a usable public subsystem.

The appropriate module and feature-gating policy must be established in:

- "src/toolchain/mod.rs"
- "src/lib.rs"
- "Cargo.toml"

The build-system feature must not accidentally expose incomplete or duplicate APIs.

---

25. Required integration with compiler and backend

25.1 Compiler API

The toolchain MUST use the compiler's public compilation service.

Relevant existing files include:

- "src/compiler/api/request.rs"
- "src/compiler/api/response.rs"
- "src/compiler/api/compiler.rs"
- "src/compiler/api/session.rs"
- "src/compiler/artifacts/"
- "src/compiler/analysis/"
- "src/compiler/planning/"

The toolchain MUST NOT call private compiler passes directly as a substitute for the compiler service.

The compiler remains responsible for establishing compilation results.

25.2 Feasibility

The compiler feasibility subsystem is authoritative for compiler-facing feasibility assessments.

Relevant existing files include:

- "src/compiler/analysis/feasibility.rs"
- "src/compiler/analysis/requirements.rs"
- "src/compiler/analysis/capabilities.rs"
- "src/compiler/planning/feasibility.rs"
- "src/compiler/planning/resource_planning.rs"
- "src/compiler/planning/target_discovery.rs"

The toolchain MUST preserve unknown and indeterminate outcomes.

It MUST NOT create its own competing feasibility algorithm.

25.3 Backend

Relevant backend authorities include:

- "src/backend/target.rs"
- "src/backend/capabilities.rs"
- "src/backend/requirements.rs"
- "src/backend/compatibility.rs"
- "src/backend/registry.rs"
- "src/backend/selection.rs"
- "src/backend/validation.rs"
- "src/backend/toolchain.rs"
- "src/backend/process.rs"
- "src/backend/reproducibility.rs"
- "src/backend/artifact.rs"

The toolchain MUST reuse these contracts.

Backend compatibility and backend artifact generation MUST remain distinct.

25.4 Artifact portability

The authoritative artifact portability implementation remains:

"src/compiler/artifacts/portability.rs"

The toolchain MUST delegate artifact classification and assessment to it.

The toolchain may associate the resulting evidence with logical builds, packages and target matrices, but MUST NOT duplicate the underlying classifier.

---

26. Required integration with ZHAL and ZUR

26.1 ZHAL

The authoritative hardware integration boundary is:

"src/zhal/"

Relevant components include:

- "src/zhal/capabilities/"
- "src/zhal/devices/"
- "src/zhal/providers/"
- "src/zhal/resources/"
- "src/zhal/model/"
- "src/zhal/adapters/"
- "src/zhal/security/"
- "src/zhal/observability/"

The toolchain MUST consume canonical hardware evidence and provider contracts.

It MUST NOT implement its own hardware inventory or physical resource allocator.

Capability evidence MUST retain its origin, verification classification, freshness and provenance.

26.2 ZUR

The authoritative runtime boundary is:

"src/zur/"

Relevant components include:

- "src/zur/contracts/"
- "src/zur/domains/"
- "src/zur/execution/"
- "src/zur/resources/"
- "src/zur/scheduling/"
- "src/zur/distributed/"
- "src/zur/hybrid/"
- "src/zur/testing/"
- "src/zur/platforms/"

The toolchain MUST hand off verified artifacts and validated execution intent through the public runtime contracts.

It MUST NOT independently implement runtime scheduling, physical resource allocation, execution dispatch or distributed recovery.

26.3 Runtime portability testing

"src/zur/testing/portability.rs" owns runtime portability testing.

It MUST be used to establish execution-portability evidence.

A successful build on two targets is not, by itself, proof that their observable behavior is equivalent.

---

27. Grammar integration

The toolchain MUST preserve the existing grammar ownership model.

The relevant authorities include:

27.1 Root and parser composition

- "grammar/Zamani.g4"
- "grammar/antlr/ZamaniLexer.g4"
- "grammar/antlr/ZamaniParser.g4"

The root grammar composes the language.

The toolchain MUST NOT create a second grammar or parse Zamani source independently.

27.2 POCO-REAF and portability specifications

- "grammar/specification/poco-reaf.md"
- "grammar/specification/portability.md"
- "grammar/specification/scalability-model.md"
- "grammar/specification/compilation-model.md"
- "grammar/specification/execution-model.md"
- "grammar/specification/domains.md"
- "grammar/specification/semantic-model.md"

These documents establish language-level semantics and portability obligations.

This toolchain document defines their toolchain-facing application.

It MUST NOT redefine their language semantics.

27.3 Target intent

- "grammar/compile/target.g4"
- "grammar/compile/target-selection.g4"

"target.g4" owns source-level target intent.

"target-selection.g4" owns source-level target-selection policy.

The toolchain consumes their validated semantic output.

Neither grammar performs physical target discovery, allocation or execution.

27.4 Resource and capability intent

- "grammar/resources/requirements.g4"
- "grammar/resources/capabilities.g4"
- "grammar/core/requirements.g4"
- "grammar/core/capabilities.g4"

These establish source-level resource and capability intent.

The toolchain MUST NOT reinterpret a preference as a mandatory requirement or a resource declaration as proof of availability.

27.5 Domain boundaries

Relevant grammar areas include:

- "grammar/classical/"
- "grammar/quantum/"
- "grammar/hdl/"
- "grammar/hybrid/"
- "grammar/distributed/"
- "grammar/ai/"
- "grammar/execution/"
- "grammar/hardware/"
- "grammar/resources/"
- "grammar/interoperability/"

For example:

"grammar/hybrid/classical-quantum-boundary.g4"

owns the source-level classical/quantum boundary.

"grammar/distributed/deployment.g4"

composes canonical deployment intent for distributed computing without defining a second deployment language.

The toolchain MUST preserve these single-owner boundaries.

27.6 Grammar validation

The following validation documents MUST be respected:

- "grammar/validation/semantic-boundaries.md"
- "grammar/validation/hard-coding.md"
- "grammar/validation/hardcoding-audit.md"
- "grammar/validation/scalability-rules.md"
- "grammar/validation/compatibility-rules.md"
- "grammar/validation/grammar-validation.md"
- "grammar/validation/semantic-coverage.md"

Grammar acceptance does not establish semantic validity or target feasibility.

---

28. Hard-coding policy

The toolchain MUST NOT contain universal machine-derived capacity limits.

The following examples are prohibited as universal toolchain semantics:

const MAX_CPUS: usize = 256;
const MAX_GPUS: usize = 64;
const MAX_QUBITS: usize = 1024;
const MAX_NODES: usize = 10_000;
const MAX_MEMORY: usize = 64 * 1024;
const MAX_TENSOR_RANK: usize = 32;
const MAX_TARGETS: usize = 128;
const MAX_BUILD_TASKS: usize = 10_000;

These examples are prohibited when used as arbitrary universal capability or capacity ceilings.

A particular operation may have an explicit configurable budget or a documented representation constraint.

Such limits MUST:

1. Have an identified owner.
2. Have a documented purpose.
3. Be configurable where operationally appropriate.
4. Be enforced at the correct subsystem boundary.
5. Produce structured diagnostics.
6. Avoid becoming universal language restrictions.
7. Be included in the relevant provenance when they affect results.

A security limit is not automatically a language limitation.

A host representation limit is not automatically a language limitation.

A backend limitation is not automatically a language limitation.

---

29. Failure and diagnostic semantics

Portability failures MUST be explicit.

The toolchain MUST distinguish at least the following conceptual failure classes:

Failure| Meaning
Invalid source| Source cannot satisfy grammar or frontend requirements
Semantic failure| Source violates language semantics
Manifest failure| Project configuration is invalid
Dependency failure| Required dependency cannot be resolved or verified
Capability failure| Required capability is not established
Compatibility failure| Required contract or target is incompatible
Feasibility failure| Mandatory conditions cannot be satisfied
Indeterminate assessment| Evidence is insufficient
Resource exhaustion| Available resources cannot complete the operation
Policy rejection| Applicable policy prohibits the operation
External tool failure| Required external implementation failed
Compilation failure| Compiler could not produce the requested output
Artifact verification failure| Output integrity or required evidence is invalid
Runtime failure| Execution infrastructure failed
Execution failure| The submitted program did not complete as required
Semantic portability failure| Required behavior was not preserved
Deployment failure| Deployment prerequisites or operations failed

Original error causes MUST be preserved.

The toolchain MUST NOT convert all failures into generic strings or report them as successful builds.

A diagnostic SHOULD identify:

- Operation identity.
- Project identity.
- Build identity.
- Target identity, where applicable.
- Relevant provider.
- Failed requirement or constraint.
- Evidence status.
- Failure stage.
- Suggested corrective action, where justified.
- Provenance references.

---

30. Concurrency and scaling of the toolchain itself

The toolchain must scale its own operation without hard-coded worker populations.

Build tasks, target realizations, dependency operations and provider requests MUST be managed using explicit execution policies and available resources.

The architecture SHOULD support:

- Sequential operation.
- Parallel operation.
- Incremental operation.
- Independent target realization.
- Cancellation.
- Failure isolation.
- Resumability where safe.
- Deterministic aggregation.
- Resource-aware concurrency.
- Distributed build execution where supported.

Concurrency MUST NOT alter program semantics.

The task graph MUST remain distinct from:

- Compiler IR control flow.
- Runtime scheduling.
- Quantum scheduling.
- Distributed application semantics.
- Hardware scheduling.

Operational concurrency budgets MUST be configurable.

A worker-count default is an operational policy, not a language-level maximum.

---

31. Testing and conformance requirements

Portability claims MUST be supported by actual evidence.

The toolchain MUST include tests that validate the complete chain from portable project input to target-specific realization.

The tests MUST distinguish mocked provider behavior from real provider behavior.

A mock provider may establish that orchestration behaves correctly.

It does not establish that actual hardware, external tools or runtime providers are compatible.

31.1 Minimal portability tests

The smallest meaningful projects MUST be tested.

Examples:

- Minimal classical application.
- Minimal quantum computation.
- Minimal HDL module.
- Minimal AI/tensor operation.
- Minimal hybrid program.
- Minimal distributed application.

31.2 Increasing-scale tests

The test harness MUST generate configurable workloads with increasing:

- Source size.
- Module count.
- Dependency count.
- Build graph size.
- Target count.
- Data size.
- Tensor dimensions.
- Quantum resource requirements.
- HDL signals and ports.
- Hardware resources.
- Distributed participants.
- Domain combinations.

Test limits are test budgets, not language limits.

31.3 Cross-target tests

Where actual implementations exist, test:

- Multiple compatible processor targets.
- Multiple operating systems.
- Different memory configurations.
- Different accelerator providers.
- Quantum simulator and hardware pathways.
- HDL synthesis and simulation.
- Heterogeneous environments.
- Distributed environments.

31.4 Cross-domain tests

Required combinations include:

- Classical and quantum.
- Classical and HDL.
- Classical and AI.
- Quantum and AI.
- Quantum and hardware.
- Quantum and distributed.
- Classical, quantum and distributed.
- Classical, quantum, HDL and AI.
- Hybrid and distributed.

Only claim successful combinations where the relevant implementations actually exist and have passed the applicable tests.

31.5 Semantic equivalence tests

Test the specified observable behavior, not just textual or binary equality.

Depending on the domain, equivalence may require:

- Exact output equality.
- Numerical tolerance comparison.
- Deterministic replay.
- Statistical comparison.
- Quantum measurement-distribution comparison.
- HDL timing and functional verification.
- Distributed consistency verification.
- Explicitly defined approximation bounds.

The comparison policy MUST be appropriate to the domain's semantics.

31.6 Negative tests

Test rejection of:

- Unknown required capabilities.
- Incompatible target versions.
- Invalid target identity mappings.
- Missing resource evidence.
- Stale evidence.
- Insufficient resource capacity.
- Invalid artifact portability declarations.
- Invalid semantic contracts.
- Unsupported domain combinations.
- Unauthorized fallback.
- Invalid provider contracts.
- Incompatible runtime interfaces.

Unknown MUST NOT become success.

31.7 Required integration test files

Integrate portability tests into the repository's existing test architecture.

Recommended coverage:

tests/
└── toolchain/
    ├── portability.rs
    ├── targets.rs
    ├── target_identity.rs
    ├── build_identity.rs
    ├── target_matrix.rs
    ├── compiler_integration.rs
    ├── artifact_portability.rs
    ├── capability_evidence.rs
    ├── feasibility.rs
    ├── reproducibility.rs
    ├── numerical_portability.rs
    ├── quantum_portability.rs
    ├── hdl_portability.rs
    ├── hybrid_portability.rs
    ├── distributed_portability.rs
    ├── security.rs
    ├── scalability.rs
    └── downstream/
        └── vimbai.rs

These files are proposed additions only where corresponding test coverage does not already exist.

Existing authoritative tests MUST be reused rather than duplicated.

---

32. Required repository documentation integration

This contract MUST be integrated with the following existing documents.

Document| Required relationship
"src/toolchain/documentation/ARCHITECTURE.md"| Establish the toolchain's overall architecture and ownership
"src/toolchain/documentation/INTEGRATION_CONTRACT.md"| Define cross-subsystem integration
"src/toolchain/documentation/BUILD_CONTRACT.md"| Define build lifecycle and target-specific realization
"src/toolchain/documentation/PACKAGE_CONTRACT.md"| Define package compatibility and distribution
"src/toolchain/README.md"| Provide navigation to the authoritative toolchain contracts
"grammar/specification/poco-reaf.md"| Establish language-level POCO-REAF
"grammar/specification/portability.md"| Establish language-level portability
"grammar/specification/scalability-model.md"| Establish language-level scalability
"grammar/specification/compilation-model.md"| Establish compilation semantics
"grammar/specification/execution-model.md"| Establish execution semantics
"grammar/validation/semantic-boundaries.md"| Preserve ownership boundaries
"grammar/validation/hard-coding.md"| Enforce no-hard-coding rules

A new "src/toolchain/documentation/READINESS_MATRIX.md" SHOULD be added to record implementation and conformance evidence.

This contract MUST NOT duplicate the detailed implementation responsibilities of the architecture, build or package contracts.

It owns the toolchain-facing portability requirements and their acceptance criteria.

---

33. Implementation sequence and file-freeze requirements

The contract must be established before downstream implementation files are frozen.

The following sequence is mandatory.

Stage 1 — Contract reconciliation

First reconcile:

1. "grammar/specification/poco-reaf.md"
2. "grammar/specification/portability.md"
3. "grammar/specification/scalability-model.md"
4. "grammar/specification/compilation-model.md"
5. "grammar/specification/execution-model.md"
6. "grammar/specification/semantic-model.md"
7. "grammar/validation/semantic-boundaries.md"
8. "src/toolchain/documentation/ARCHITECTURE.md"
9. "src/toolchain/documentation/INTEGRATION_CONTRACT.md"
10. "src/toolchain/documentation/BUILD_CONTRACT.md"

Resolve contradictions before implementing new portability APIs.

Stage 2 — Independent portability foundations

Review and freeze the existing foundational authorities:

- "src/compiler/artifacts/portability.rs"
- "src/compiler/analysis/feasibility.rs"
- "src/backend/target.rs"
- "src/backend/capabilities.rs"
- "src/backend/requirements.rs"
- "src/backend/compatibility.rs"
- "src/zhal/capabilities/evidence.rs"
- "src/zhal/capabilities/requirements.rs"
- "src/zhal/capabilities/compatibility.rs"
- "src/zur/domains/compatibility.rs"
- "src/zur/testing/portability.rs"

These files must not be modified merely to satisfy toolchain convenience.

Their canonical ownership and public contracts must be established first.

Stage 3 — Toolchain identity and API

Freeze:

- "src/toolchain/ids.rs"
- "src/toolchain/version.rs"
- "src/toolchain/errors.rs"
- "src/toolchain/result.rs"
- "src/toolchain/api/versioning.rs"
- "src/toolchain/api/compatibility.rs"
- "src/toolchain/api/capabilities.rs"
- "src/toolchain/api/request.rs"
- "src/toolchain/api/response.rs"

Establish the public API and compatibility contracts.

Stage 4 — Target identity and portability coordination

Freeze:

- "src/toolchain/targets/profiles.rs"
- "src/toolchain/targets/compatibility.rs"
- "src/toolchain/targets/selection.rs"
- "src/toolchain/targets/matrix.rs"
- "src/toolchain/targets/validation.rs"

Resolve the compiler/backend target identity mapping before dependent build files are frozen.

Stage 5 — Build integration

Freeze:

- "src/toolchain/build/request.rs"
- "src/toolchain/build/plan.rs"
- "src/toolchain/build/inputs.rs"
- "src/toolchain/build/graph.rs"
- "src/toolchain/build/executor.rs"
- "src/toolchain/build/outputs.rs"
- "src/toolchain/build/incremental.rs"
- "src/toolchain/build/reporting.rs"

All files must consume the established portability and target contracts.

Stage 6 — Runtime and artifact integration

Validate:

- Compiler artifact portability.
- Backend artifact validation.
- Hardware capability evidence.
- Runtime provider compatibility.
- Runtime portability testing.
- Deployment handoff.

Stage 7 — Domain conformance

Validate the same toolchain contracts across classical, quantum, HDL, AI, hybrid, distributed and future registered domains.

---

34. File completion and freeze criteria

Every implementation file involved in this contract MUST have the following recorded in the toolchain readiness matrix:

1. Canonical path.
2. Purpose.
3. Exclusive ownership.
4. Explicit non-ownership.
5. Public types.
6. Public functions.
7. Imported contracts.
8. Exact integration dependencies.
9. Downstream consumers.
10. Error behavior.
11. Compatibility requirements.
12. Thread-safety requirements.
13. Concurrency behavior.
14. Security requirements.
15. Scalability requirements.
16. Determinism requirements.
17. Serialization requirements, where applicable.
18. Unit tests.
19. Integration tests.
20. Conformance tests.
21. Completion evidence.
22. Freeze criteria.

A file MUST NOT be declared complete merely because it compiles.

It is complete only when:

- Its ownership is unambiguous.
- Its public API is established.
- Its dependencies are known.
- Its consumers are known.
- Its error behavior is complete.
- Its scalability requirements are satisfied.
- Its security requirements are satisfied.
- Its tests pass.
- Its integrations compile and behave correctly.
- No known downstream redesign is required.

If a later integration exposes a genuine defect, the owning contract must be corrected deliberately, with affected consumers and compatibility implications identified.

Unrelated files MUST NOT accumulate undocumented workarounds.

---

35. Rust implementation requirements

All new Rust implementation work associated with this contract MUST use:

- Rust 1.97 or later.
- Rust 2021 edition unless the repository explicitly adopts a newer edition.
- Safe Rust exclusively.

The implementation MUST NOT use:

unsafe

All relevant implementation modules SHOULD retain:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

Where appropriate, retain the repository's existing:

#![deny(unused_must_use)]
#![deny(missing_docs)]

The implementation MUST also:

- Preserve structured errors.
- Avoid global mutable state.
- Avoid implicit hardware discovery.
- Avoid hidden filesystem or network access.
- Avoid fabricated provider results.
- Validate untrusted serialized inputs.
- Use checked numeric conversions.
- Avoid arbitrary universal collection capacities.
- Avoid undocumented fixed-size buffers.
- Preserve deterministic behavior where required.
- Avoid introducing dependency cycles.

---

36. Production-readiness acceptance criteria

The toolchain may claim portability conformance only when the applicable requirements have implementation and test evidence.

36.1 Architecture

- [ ] Toolchain owns orchestration, not compilation.
- [ ] Compiler remains authoritative.
- [ ] Backend remains authoritative for target-specific generation.
- [ ] ZHAL remains authoritative for hardware integration.
- [ ] ZUR remains authoritative for execution.
- [ ] Canonical domain IRs are reused.
- [ ] No competing portability engine exists.
- [ ] Target identity mapping is explicit and validated.
- [ ] Public toolchain module integration is complete.

36.2 POCO-REAF

- [ ] Program Once semantics are preserved.
- [ ] Compile Once is defined without promising one universal binary.
- [ ] Multiple target realizations are supported.
- [ ] Run Everywhere is conditional on verified compatibility.
- [ ] Run Anywhere respects execution and deployment requirements.
- [ ] Run Forever uses versioned contracts and migration.
- [ ] Source identity is preserved.
- [ ] Dependency identity is preserved.
- [ ] Logical build identity is preserved.
- [ ] Target-specific artifact identity is preserved.

36.3 Scalability

- [ ] No arbitrary universal hardware ceilings.
- [ ] No fixed universal domain enumeration.
- [ ] No hidden target-matrix capacity ceilings.
- [ ] No arbitrary project-size ceiling.
- [ ] No arbitrary dependency-count ceiling.
- [ ] No arbitrary worker-count semantic ceiling.
- [ ] Operational budgets are configurable.
- [ ] Representation limitations are explicit.
- [ ] Resource exhaustion is classified correctly.
- [ ] Tiny projects are supported.
- [ ] Large projects are stress-tested.
- [ ] Unknown capacity is never treated as unlimited capacity.

36.4 Domain portability

- [ ] Classical portability is tested.
- [ ] Quantum portability is tested.
- [ ] HDL portability is tested.
- [ ] AI/tensor portability is tested.
- [ ] Hybrid portability is tested.
- [ ] Distributed portability is tested.
- [ ] Cross-domain combinations are tested.
- [ ] Future domains can register compatible providers.

36.5 Feasibility and execution

- [ ] Requirements and capabilities remain distinct.
- [ ] Constraints and preferences remain distinct.
- [ ] Unknown evidence remains indeterminate.
- [ ] Compatibility is not treated as feasibility.
- [ ] Feasibility is not treated as reservation.
- [ ] Reservation is not treated as execution.
- [ ] Compilation success is not treated as execution success.
- [ ] Partial target success is reported accurately.
- [ ] Runtime assumptions are revalidated.

36.6 Artifacts and security

- [ ] Artifact portability declarations are validated.
- [ ] Artifact verification evidence is preserved.
- [ ] Artifact integrity is checked.
- [ ] Provenance is preserved.
- [ ] Package compatibility is checked.
- [ ] Provider trust is established.
- [ ] External toolchains are validated.
- [ ] Credentials are protected.
- [ ] Untrusted inputs are validated.
- [ ] Cache reuse is compatibility-aware.

36.7 Reproducibility

- [ ] Build identities are deterministic.
- [ ] Dependency resolution is reproducible.
- [ ] Environment fingerprints are recorded.
- [ ] Target-specific fingerprints are recorded.
- [ ] Compiler and backend versions are recorded.
- [ ] Relevant provider versions are recorded.
- [ ] Incremental build invalidation is correct.
- [ ] Artifact provenance is verifiable.

36.8 Testing

- [ ] Unit tests pass.
- [ ] Integration tests pass.
- [ ] Cross-target conformance tests pass.
- [ ] Cross-domain tests pass.
- [ ] Negative tests pass.
- [ ] Security tests pass.
- [ ] Scalability tests pass.
- [ ] Reproducibility tests pass.
- [ ] Runtime portability tests pass.
- [ ] Actual provider testing is distinguished from mock testing.

---

37. Required Rust validation

The repository must pass its established Rust validation commands:

cargo fmt --all -- --check

cargo check --all-targets

cargo test --all-targets

cargo clippy --all-targets -- -D warnings

The build-system feature must also be validated independently:

cargo check --all-targets --features buildsystem

cargo test --all-targets --features buildsystem

The actual package-manager executable must be tested:

cargo build --release \
    --features buildsystem \
    --bin zamani-pkg

Where additional feature combinations are supported, their compatibility must also be tested.

Passing these commands establishes Rust build and test evidence only. It does not independently establish universal portability, hardware support or production readiness across all domains.

---

38. Absolute portability invariants

The following invariants are mandatory.

Invariant 1 — Semantic preservation

A target change MUST NOT silently redefine the meaning of a valid portable program.

Invariant 2 — No arbitrary universal ceilings

A current machine's capacity MUST NOT become a universal Zamani language or toolchain capacity limit.

Invariant 3 — Honest feasibility

Unknown capability or resource evidence MUST NOT be interpreted as proof of feasibility.

Invariant 4 — Open-world extensibility

A new target or computational domain MUST be integrable without requiring a universal hard-coded enumeration to be redesigned.

Invariant 5 — Canonical ownership

The toolchain MUST reuse the compiler, backend, ZHAL, ZUR and domain authorities rather than implementing competing subsystems.

Invariant 6 — Artifact honesty

A portability declaration MUST NOT be presented as verified portability without the required evidence.

Invariant 7 — Execution honesty

Compilation success MUST NOT be presented as proof of runtime execution success.

Invariant 8 — Resource honesty

Available resources MUST NOT be confused with reserved resources.

Invariant 9 — Compatibility honesty

A compatible interface MUST NOT automatically be interpreted as proof that every workload is executable.

Invariant 10 — Long-term compatibility

Unknown future semantic or artifact versions MUST NOT be silently accepted.

Invariant 11 — Safe Rust

No implementation associated with this contract may use unsafe Rust.

Invariant 12 — Physical reality

The architecture MUST remain scalable without claiming physically infinite resources.

---

39. Final POCO-REAF architecture

The intended Zamani portability architecture is:

                 ONE ZAMANI PROGRAM
                         |
                         v
                 ONE SOURCE IDENTITY
                         |
                         v
                ONE SEMANTIC MEANING
                         |
                         v
             ONE REUSABLE BUILD SPECIFICATION
                         |
                         v
              PORTABLE SEMANTIC ARTIFACT
                         |
                         v
             REQUIREMENTS AND CONSTRAINTS
                         |
                         v
             CAPABILITY AND FEASIBILITY
                    ASSESSMENT
                         |
                         v
                LOGICAL BUILD IDENTITY
                         |
            +------------+-------------+
            |            |             |
            v            v             v
          CPU           GPU           FPGA
            |            |             |
            v            v             v
       Realization A Realization B Realization C
            |            |             |
            +------------+-------------+
                         |
             +-----------+-----------+
             |           |           |
             v           v           v
           QPU         Cluster      Future
        Realization    Realization  Realization
             |           |           |
             +-----------+-----------+
                         |
                         v
             VERIFIED TARGET ARTIFACTS
                         |
                         v
              COMPATIBLE EXECUTION
                         |
                         v
                VERIFIED OUTCOMES

The source program and its specified semantics remain the common authority.

The physical realization may differ.

The toolchain preserves the identities, contracts, evidence and provenance connecting those realizations.

---

40. Final engineering principle

Zamani MUST be designed for computational scale beyond the machines available during its initial development.

A program MUST NOT need to be rewritten merely because:

- One processor becomes many processors.
- One machine becomes a cluster.
- A CPU becomes a heterogeneous system.
- A local device becomes an edge or cloud environment.
- A simulator becomes a physical quantum processor.
- A software computation becomes a hardware implementation.
- A current accelerator is replaced by a future accelerator.
- A current computational domain is extended by a new domain.

Such changes may require different valid realizations, compilation, specialization, deployment configuration or operational policies.

They MUST NOT silently change the program's meaning.

The toolchain must make those differences explicit, verifiable and reproducible.

The ultimate architectural invariant is:

A physical limitation must never become a universal language limitation merely because a current implementation has that limitation.

Likewise:

A backend implementation detail must never become universal source semantics merely because one backend currently requires it.

And:

A new computational architecture must not require rewriting a portable Zamani program merely because its physical realization differs.

Zamani's objective is:

From Atom to Everywhere.

Program Once → Compile Once → Run Everywhere → Anywhere → Forever.

End of "src/toolchain/documentation/PORTABILITY_CONTRACT.md".