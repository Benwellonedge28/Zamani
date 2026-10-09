Zamani Scalability Model

Document ID: "ZAMANI-SPEC-SCALABILITY"
Canonical path: "grammar/specification/scalability-model.md"
Status: Proposed normative specification; pending repository conformance validation
Authority: "grammar/DESIGN.md" and the normative language specification
Applies to: All Zamani language domains, compilation stages, execution environments, targets, and future extensions
Implementation baseline: Rust 1.97 or later, Rust 2021 edition unless the repository explicitly adopts a newer edition
Memory-safety requirement: Safe Rust only; "unsafe" is prohibited
Compatibility: Versioned according to the language and artifact compatibility contracts
Primary objective: Enable Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF) without universal, hard-coded hardware-capacity ceilings.

---

1. Purpose

This document defines the normative scalability model for the Zamani programming language.

Zamani SHALL express computational meaning, requirements, constraints, preferences, capabilities, policies, and resource intent independently of the physical capacity and architecture of a particular machine.

A Zamani program SHALL be capable of expressing computations intended to execute on, or be realized across:

- Tiny and resource-constrained devices.
- Classical processors and multicore systems.
- Vector processors, GPUs, and other accelerators.
- FPGA, ASIC, and programmable hardware.
- Quantum processors and quantum simulators.
- Classical–quantum hybrid systems.
- Embedded, edge, server, high-performance computing, cluster, cloud, and distributed environments.
- Future computational substrates that preserve the language's defined semantic contracts.

The language SHALL NOT require a separate source-language design for each computational scale or hardware family.

The same source program SHALL preserve its specified meaning when compiled or realized for different supported targets, subject to its declared requirements, language semantics, numerical guarantees, effects, policies, and execution contracts.

This specification defines language-level requirements and architectural boundaries. It does not claim that every target can execute every program or that arbitrary computations can complete with finite resources.

2. Normative language

The terms MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, and MAY are normative.

- MUST / SHALL indicate requirements necessary for conformance.
- MUST NOT / SHALL NOT indicate prohibited behavior.
- SHOULD indicates a strong recommendation that may be departed from only for a documented reason.
- MAY indicates an optional behavior.

A feature is not considered implemented merely because its syntax exists. Implementation status MUST be established through the relevant AST, semantic, compiler, runtime, and conformance contracts.

3. Fundamental scalability principle

Zamani SHALL distinguish the mathematical domain of a computation from the finite resources used to realize it.

Let:

- (P) be a Zamani program.
- (M(P)) be its specified semantic meaning.
- (R) be the declared resource requirements.
- (C) be the required capabilities.
- (K) be the applicable constraints.
- (Q) be the applicable policies.
- (T) be a candidate target environment.
- (A(T)) be the resources and capabilities actually available in that environment.
- (L) be the selected realization and lowering strategy.

A target realization is admissible only when:

[
\operatorname{Admissible}(P,T)

\operatorname{RequirementsMet}(R,A(T))
\land
\operatorname{CapabilitiesMet}(C,T)
\land
\operatorname{ConstraintsMet}(K,T)
\land
\operatorname{PoliciesPermit}(Q,P,T)
]

Admissibility is necessary but not sufficient for successful execution. The implementation MUST also preserve the relevant semantic, type, effect, numerical, memory-safety, security, and execution guarantees.

The central invariant is:

«The language defines what a computation means. The compiler and runtime determine how that computation can be realized using available resources without violating its specified semantics.»

The language MUST NOT confuse the largest currently available machine with the largest computation expressible by the language.

4. Meaning of unbounded scalability

4.1 No universal finite capacity ceiling

Zamani MUST NOT define universal language-level maximums for physical or logical resource populations.

The following are prohibited as universal language limits:

- Maximum qubits.
- Maximum CPUs, cores, threads, GPUs, FPGAs, ASICs, or QPUs.
- Maximum nodes, workers, processes, devices, or network endpoints.
- Maximum memory, storage, address-space capacity, or accelerator memory.
- Maximum register width, vector width, tensor rank, collection length, or graph size.
- Maximum number of modules, functions, types, operations, channels, tasks, or distributed participants.

The prohibition includes disguised equivalents, including fixed global ceilings, implicit finite enumerations of possible hardware, and universal constants introduced solely to cap capacity.

This rule does not prohibit finite-width types, bounded algorithms, explicit resource limits, implementation budgets, protocol limits, or target-specific constraints where their meanings and scope are documented.

4.2 What unbounded means

For this specification, unbounded scalability means that Zamani's language model does not impose an arbitrary universal maximum on a resource dimension merely because an implementation currently supports a smaller value.

It does not mean that physical resources, address spaces, execution time, or mathematical quantities are literally infinite.

Every concrete implementation is subject to actual resource availability, representational limits, operational constraints, and explicitly declared budgets.

4.3 Growth must be compositional

The language and its implementation SHOULD allow a computation to grow through supported abstractions such as:

- Increasing a parameterized problem size.
- Increasing collection or tensor dimensions.
- Increasing independent work.
- Decomposing work into tasks or partitions.
- Adding compatible processors or accelerators.
- Distributing work across nodes.
- Increasing logical quantum resources when the target and execution model permit.
- Increasing generated hardware structures within the capabilities of the synthesis flow.
- Composing new supported computational domains through stable interfaces.

Growth MUST NOT require a source-language redesign merely because a resource population becomes larger.

4.4 Resource availability remains authoritative

A program requiring unavailable resources MUST NOT be treated as successfully executable merely because the language permits its expression.

The compiler or runtime MUST:

1. Evaluate declared requirements and applicable policies.
2. Determine which requirements can be established from available information.
3. Reject, defer, or explicitly negotiate requirements that cannot be satisfied.
4. Select a valid realization when one exists.
5. Report incompatibility or resource exhaustion when no permitted realization exists.

The implementation MUST NOT fabricate resource availability or silently discard a requirement.

5. Required scalability dimensions

Scalability is multidimensional. A successful implementation MUST NOT equate scalability exclusively with increasing CPU count or executing more threads.

The model covers the following dimensions.

Dimension| Required property
Problem size| Parameters and data structures can represent supported problem sizes without universal capacity ceilings.
Memory| Memory requirements are expressed independently of a particular device's capacity.
Computation| Work can be mapped to compatible execution resources.
Parallelism| Independent and dependent work are represented according to their semantics.
Distribution| Work and data can be partitioned or distributed where the computation permits it.
Heterogeneity| Multiple compatible resource classes can participate through defined interfaces.
Quantum computation| Logical quantum requirements remain distinct from physical device capacity and realization.
Hardware description| Hardware structures and dimensions are parameterized and subject to explicit synthesis constraints.
Compilation| Compiler work is managed through explicit budgets and scalable algorithms.
Runtime| Scheduling, placement, negotiation, and monitoring adapt to actual execution conditions.
Reliability| Resource loss and failures are handled according to declared resilience guarantees.
Numerical behavior| Precision, overflow, approximation, and reproducibility guarantees remain explicit.
Security| Authorization, isolation, and policy enforcement remain effective as scale changes.
Maintainability| Extensions do not require duplicating foundational semantics.
Compatibility| Versioned source and artifacts retain their declared compatibility guarantees.

A claim of scalability MUST identify the relevant workload, target environment, correctness requirements, resource assumptions, and performance or operational objectives. A single successful large-scale test does not prove scalability for every workload or target.

6. Semantic portability across scale

6.1 Preserve specified meaning

Scaling a computation MUST NOT silently change its specified meaning.

An implementation MUST preserve, as applicable:

- Type correctness.
- Observable program behavior.
- Ordering and synchronization guarantees.
- Ownership, borrowing, lifetime, and memory-safety guarantees.
- Effect and capability restrictions.
- Resource requirements and constraints.
- Contract preconditions and postconditions.
- Policy decisions and authorization requirements.
- Numerical and precision guarantees.
- Distributed consistency and transaction guarantees.
- Quantum measurement and state-transition semantics.
- HDL timing, clocking, and hardware-structure requirements.
- Provenance and reproducibility obligations.

Where exact equivalence is not required or cannot be established, the language or relevant domain specification MUST define the permitted equivalence relation.

For example, floating-point parallel reduction may produce a different result if the language permits a different reduction order. A compiler MUST NOT assume that reassociation is valid for every numerical type.

Likewise, a quantum simulator, quantum processor, classical algorithm, and hardware realization MUST NOT be assumed interchangeable without an explicit semantic contract establishing the permitted relationship.

6.2 Separate intent from realization

Source code SHOULD express the required computation and its constraints without unnecessarily naming a physical execution unit.

Where the program does not explicitly constrain placement, the implementation MAY choose among semantically compatible realizations.

Such choices MUST respect the program's declared requirements, observable behavior, effects, policies, and numerical guarantees.

Explicit placement directives MAY restrict portability. Their scope and consequences MUST be documented.

6.3 No silent semantic downgrade

The compiler or runtime MUST NOT silently:

- Replace quantum execution with a classical approximation.
- Replace an exact computation with an approximate computation.
- Relax a memory, latency, reliability, security, or precision requirement.
- Remove an unsupported effect.
- Change a consistency guarantee.
- Ignore a required hardware timing constraint.
- Drop a required capability or authorization.
- Treat an unknown capability as available.

An alternative realization is permitted only when it satisfies the program's declared contract or the program explicitly authorizes the relevant alternative.

7. Resource, capability, constraint, preference, and hint model

These concepts MUST remain distinct throughout the language architecture.

7.1 Resource requirement

A resource requirement describes a quantity or property needed for a computation to execute under its declared contract.

Examples of intended source-level expressions include:

requires memory >= required_memory;
requires qubits >= logical_qubit_requirement;
requires resource("compute.capacity") >= workload_requirement;

These examples illustrate the semantic model. They MUST NOT be copied into a grammar file until the canonical token spellings, rule names, and resource-expression interfaces have been verified.

A resource requirement MUST map to a structured AST representation and a semantic constraint. It MUST NOT directly allocate a physical device.

7.2 Capability requirement

A capability requirement states that an environment must provide a specified operation or property.

Examples include:

requires capability("quantum.measurement");
requires capability("gpu.compute");
requires capability("tensor.compute");

Capability identifiers MUST be extensible and namespaced. Unknown capability identifiers MUST NOT be treated as satisfied merely because the compiler does not recognize them.

The meaning of a capability is defined by its canonical registry or extension contract, not by a device-name enumeration in the core grammar.

7.3 Constraint

A constraint defines a condition that an admissible realization MUST satisfy.

Constraints are mandatory within their declared scope. A compiler or runtime MUST NOT relax them without explicit authorization under the governing specification.

7.4 Preference

A preference expresses a desirable realization property.

A preference MAY influence candidate ranking but MUST NOT override mandatory requirements, constraints, security rules, or policies.

If no candidate satisfies all mandatory requirements, the implementation MUST NOT claim success merely because a preferred target exists.

7.5 Hint

A hint provides optional optimization information.

A hint MUST NOT change the program's specified meaning or authorize an otherwise forbidden effect.

An implementation MAY ignore a hint and MUST remain correct when doing so.

7.6 Permission and prohibition

Permissions and prohibitions express what an execution is allowed or forbidden to do.

They MUST be evaluated through the applicable policy, security, and authorization model. They are not interchangeable with preferences.

7.7 Requirement evaluation states

The semantic and execution layers SHOULD distinguish at least:

- Satisfied.
- Unsatisfied.
- Unknown.
- Not applicable.
- Temporarily unavailable.
- Evaluation failed.

An unknown result MUST NOT automatically become satisfied.

The exact representation MUST use the repository's canonical diagnostic and semantic-result types where available.

8. Symbolic and parameterized resource expressions

8.1 Resource expressions

Resource expressions MUST support the language's defined expression forms, including symbolic quantities, named values, parameterized values, and supported arithmetic or constraint operations.

They MUST reuse the canonical expression, name, type, and resource abstractions instead of creating parallel expression languages.

The language MUST NOT require every resource quantity to be a compile-time integer literal.

8.2 Parameterized dimensions

Problem sizes, tensor dimensions, collection lengths, logical resource requirements, and hardware structure dimensions SHOULD be parameterized when appropriate.

The implementation MUST distinguish:

- A statically known dimension.
- A symbolic dimension.
- A runtime-determined dimension.
- A target-selected dimension.
- A dimension whose value is constrained by a contract.

A parameterized dimension MUST NOT be interpreted as a physical capacity declaration unless the relevant domain semantics explicitly define that relationship.

8.3 Overflow and representability

Resource arithmetic MUST have defined semantics for overflow, invalid quantities, unit conversion, and values outside an implementation's representable range.

An implementation MUST NOT silently wrap a resource requirement into a smaller value when that could change admissibility.

If a quantity cannot be represented or evaluated safely, the implementation MUST report a diagnostic or use a specified representation capable of preserving its meaning.

8.4 Units and dimensions

Resource quantities MUST use canonical unit and dimension semantics where units are applicable.

The semantic layer MUST reject incompatible comparisons, invalid conversions, and ambiguous quantities unless the language explicitly defines their interpretation.

Unit support MUST be extensible without introducing target-specific capacity constants into the universal grammar.

9. Resource discovery and negotiation

Resource discovery and negotiation belong to compiler services, target adapters, execution planning, and runtime services—not to lexical analysis or universal grammar rules.

The negotiation process MUST conceptually perform the following steps:

1. Collect the program's declared requirements, constraints, capabilities, preferences, and policies.
2. Obtain relevant target capability and resource information.
3. Establish which facts are current, unknown, or potentially stale.
4. Filter out candidates that violate mandatory constraints.
5. Evaluate preferences among the remaining candidates.
6. Select a permitted realization or report that none is available.
7. Lower and plan the computation for the selected realization.
8. Revalidate assumptions that may have changed before execution.
9. Monitor execution conditions and handle failures according to the execution contract.

A candidate's advertised capacity MUST NOT be treated as a guarantee that the capacity remains available.

The implementation MUST account for reservations, concurrent consumption, revocation, changing topology, and stale capability information where these are relevant to the target.

If requirements can change at runtime, the relevant execution model MUST define when and how they are reevaluated.

10. Execution planning and adaptive scaling

10.1 Planning responsibilities

The compiler and runtime MAY use target-independent optimization, specialization, decomposition, partitioning, placement, routing, scheduling, and adaptive execution to realize a program.

Each stage MUST operate through stable semantic and IR contracts.

No planning stage may redefine the program's meaning merely to simplify implementation.

10.2 Adaptive execution

Adaptive execution MAY respond to changing resource availability, workload, topology, performance, or reliability.

Adaptation MUST remain subject to:

- Declared policies.
- Capabilities and authorization.
- Effect restrictions.
- Resource constraints.
- Contracts and invariants.
- Provenance requirements.
- Numerical and semantic guarantees.
- Security and isolation boundaries.

Adaptation MUST NOT be treated as unrestricted modification of executable code.

Where adaptation changes a program artifact, configuration, or execution plan, the implementation MUST define the authorization, validation, versioning, and provenance requirements for that change.

10.3 Scheduling

Scheduling MUST remain a realization decision rather than a universal source-language capacity limit.

A program MAY express scheduling constraints or preferences using supported syntax, but the core grammar MUST NOT prescribe a fixed global worker count, thread count, queue size, or device population.

The scheduler MUST respect data dependencies, effect ordering, synchronization, and any applicable timing or consistency requirements.

10.4 Backpressure and admission control

When demand exceeds available resources, the runtime SHOULD use defined admission control, backpressure, queuing, cancellation, or rejection mechanisms appropriate to the execution model.

It MUST NOT accept work and then silently violate required resource or correctness guarantees.

Any bounded queue, buffer, retry policy, or admission budget MUST be represented as an explicit implementation configuration or declared contract where applicable—not as a universal language ceiling.

11. Classical computing scalability

Classical constructs MUST use the common type, expression, function, memory, effect, and resource models.

The language SHOULD support scalable representations and realizations for:

- Scalars and aggregate values.
- Vectors, matrices, and tensors.
- Collections and streams.
- Scientific and numerical computations.
- Parallel and vectorized computations.
- Data processing and transformations.
- Accelerator-backed computation.
- Symbolic and optimization workloads.

A tensor dimension or vector length MUST NOT imply a fixed processor register width.

A collection's semantic length MUST NOT be capped by a universal grammar constant.

An implementation MAY impose operational budgets for a particular target or execution session, provided that those budgets are explicit, correctly enforced, and reported through the appropriate diagnostic or runtime outcome.

12. Quantum computing scalability

Quantum scalability MUST distinguish logical quantum meaning from physical quantum realization.

The language and semantic model MUST distinguish, where applicable:

- Logical qubits and physical qubits.
- Logical operations and physical operation realizations.
- Quantum state semantics and device-specific state representations.
- Measurement semantics and device measurement capabilities.
- Abstract quantum circuits and routed physical circuits.
- Error-correction intent and concrete error-correction execution.
- Quantum resource requirements and observed hardware availability.
- Simulator capabilities and actual quantum processor capabilities.

Quantum operations MUST use the established generic operation model and registered operation metadata. The universal grammar MUST NOT depend on a permanently enumerated list of physical quantum devices or a finite list of quantum operations.

The quantum domain MUST map through the established semantic quantum representation into the canonical "quantum::ir". It MUST NOT introduce a competing quantum IR solely to accommodate a new scale or backend.

Physical qubit assignment, device calibration, topology-dependent routing, scheduling, pulse selection, error-correction execution, and hardware admission MUST remain downstream realization responsibilities.

The system MUST reject an execution plan when its quantum requirements cannot be met and no permitted alternative exists.

A simulator MUST NOT be assumed to provide the same performance, noise, timing, or operational behavior as physical quantum hardware.

13. Hardware description language scalability

HDL constructs MUST describe parameterized hardware intent and preserve the distinction between design intent and synthesized realization.

The language SHOULD support parameterized descriptions of:

- Signals, ports, nets, and interfaces.
- Registers and memories.
- Combinational and sequential logic.
- Clocks, reset, and timing constraints.
- Pipelines and state machines.
- Generated structures and repeated components.
- Protocols and hardware interfaces.
- Verification assertions and design contracts.
- Physical implementation constraints.
- Hardware/software co-design boundaries.

Widths, depths, lane counts, pipeline dimensions, and generated instance counts MUST be parameterizable where the language defines those facilities.

The core grammar MUST NOT impose universal limits on bus widths, register populations, clock counts, memory depth, or generated structures.

Concrete synthesis tools and targets MAY have finite implementation constraints. Such constraints MUST be reported through target validation and synthesis diagnostics.

The language MUST NOT claim that an arbitrary HDL design is realizable merely because its source syntax is valid.

14. Hybrid and heterogeneous computing

Hybrid programs MUST compose the existing semantic models rather than define parallel versions of classical, quantum, and HDL semantics.

The architecture MUST define explicit boundaries for:

- Classical-to-quantum data flow.
- Quantum-to-classical measurement results.
- Host-to-device data movement.
- Synchronization and feed-forward.
- Shared data and ownership.
- Accelerator interoperability.
- Resource coordination.
- Cross-domain effects and error handling.

A hybrid execution plan MUST validate the compatibility of its component requirements and communication boundaries.

If a component cannot execute on a selected target, the planner MUST identify a permitted alternative or report incompatibility.

It MUST NOT silently replace a required component with a semantically different computation.

15. Distributed and concurrent scalability

Concurrency and distributed execution MUST build on common computation, effect, resource, and memory semantics.

The language SHOULD permit supported computations to be realized as tasks, futures, actors, pipelines, parallel work, distributed processes, or accelerator workloads when the semantic contract permits those mappings.

The implementation MUST distinguish logical concurrency from physical threads and workers.

Distributed execution MUST define the relevant semantics for:

- Communication and message delivery.
- Ordering and synchronization.
- Failure and recovery.
- Replication and partitioning.
- Transactions and consistency.
- Cancellation and retries.
- Ownership and data movement.
- Provenance and observability.

The grammar MUST NOT define a universal maximum for participants, nodes, messages, channels, workers, or partitions.

Scaling out MUST NOT be assumed to preserve semantics automatically. The compiler and runtime MUST respect the selected consistency model, ordering constraints, transaction boundaries, and failure semantics.

Retries MUST NOT silently duplicate externally observable effects where the program's contract requires at-most-once behavior or otherwise prohibits duplication.

16. Memory scalability and ownership

Memory semantics MUST remain independent of physical memory capacity.

The memory model MUST integrate with the canonical type, ownership, borrowing, reference, lifetime, allocation, and effect systems.

The language MUST distinguish, where supported:

- Logical object size and physical allocation size.
- Available memory and required memory.
- Local and distributed memory.
- Shared and private memory.
- Host and accelerator memory.
- Persistent and transient storage.
- Logical quantum state and physical quantum resources.

An implementation MUST NOT infer that an allocation will succeed merely because its size is syntactically valid.

Allocation failures MUST be handled according to the language's defined failure and error semantics.

Streaming, chunking, incremental processing, lazy evaluation, partitioning, and bounded working sets SHOULD be available through the appropriate language and library abstractions when they preserve the required semantics.

Safe Rust is mandatory for the Rust implementation. Memory scalability MUST NOT be achieved by introducing "unsafe" code.

17. Compiler scalability

The compiler is itself a resource-consuming program and MUST follow the same scalability principles.

The compiler architecture SHOULD support:

- Incremental processing where practical.
- Streaming or bounded-memory processing where appropriate.
- Avoidance of unnecessary whole-program materialization.
- Efficient indexing and symbol resolution.
- Reuse of immutable intermediate artifacts.
- Deterministic and reproducible builds.
- Parallel compilation where dependencies permit it.
- Explicit compilation budgets and cancellation.
- Structured diagnostics for resource exhaustion.
- Versioned, target-independent compilation artifacts.
- Modular validation and optimization passes.

The compiler MUST NOT introduce arbitrary universal source-language limits merely to compensate for an inefficient implementation.

Where a compiler service requires finite limits for safety or operational stability, those limits MUST be configurable or explicitly specified, scoped to that service, and reported when reached.

The compiler MUST distinguish a program that violates the language specification from a valid program that exceeds a particular compiler's available resources.

The latter MUST NOT automatically be reported as a syntax or semantic error.

Compiler optimizations MUST preserve the applicable language and IR contracts.

18. Target-independent compilation artifacts

POCO-REAF requires a stable representation of portable program meaning.

The compilation artifact model SHOULD record, as applicable:

- Source and semantic identity.
- Language and feature versions.
- AST and semantic-model versions where needed.
- Canonical IR version.
- Dialect and extension versions.
- Resource requirements.
- Capability requirements.
- Constraints and preferences.
- Effect and policy contracts.
- Numerical and reproducibility guarantees.
- Relevant provenance.
- Target-independent computation representation.
- Explicit target-specific dependencies, when present.
- Compatibility and migration information.

The artifact MUST distinguish portable semantic content from target-specific compiled output.

A portable artifact MUST NOT be treated as universally executable merely because it can be serialized or transported.

At execution time, the implementation MUST validate target compatibility and required capabilities.

Target-specific binaries MAY be generated and cached as derived artifacts. Their compatibility MUST be established independently of the portable semantic artifact.

19. POCO-REAF contract

19.1 Program once

A program expresses its computation and declared contracts once, using the common language and supported domain abstractions.

The source SHOULD avoid unnecessary assumptions about particular processor models, device identifiers, or machine capacities.

Explicit target-specific programming MAY be used when required, but its portability implications MUST be documented.

19.2 Compile once

The portable compilation product captures stable semantic meaning and the information required to realize that meaning on compatible targets.

This does not require a single native machine-code binary to execute unchanged on every architecture.

Target-specific lowering, specialization, linking, packaging, and validation MAY occur later without requiring the source program to be rewritten.

19.3 Run everywhere

A program can run only where its mandatory requirements, capabilities, constraints, policies, and execution contracts can be satisfied.

The compiler or runtime MUST select a compatible realization or explain why none is available.

Portability MUST NOT be claimed for targets that have not been validated to satisfy the program's requirements.

19.4 Run anywhere

The system MAY negotiate among compatible environments, including local processors, accelerators, simulators, and distributed resources.

Placement MUST respect authorization, data residency, security, privacy, resource, and execution policies.

An implementation MUST NOT move computation or data to an otherwise prohibited environment merely because it has greater capacity.

19.5 Run forever

Long-term compatibility requires versioned specifications, explicit compatibility rules, artifact evolution, migration procedures, and stable semantic contracts.

No implementation can guarantee literally perpetual execution in the absence of available resources, supported runtimes, required external services, compatible hardware, and valid authorization.

The language MUST preserve its documented compatibility promises rather than relying on an undefined promise of eternal compatibility.

20. Fallback and alternative realizations

Fallback behavior MUST be explicit and semantically justified.

A program or applicable policy MAY permit alternatives such as:

- A compatible simulator when physical hardware is unavailable.
- A different compatible processor or accelerator.
- A permitted partitioning or scheduling strategy.
- A compatible implementation of the same abstract operation.
- A classical implementation of a computation when the specified contract establishes equivalence or the program explicitly authorizes the alternative.

The implementation MUST verify the alternative against the relevant requirements and contracts.

An approximation MUST be identified as such and MUST satisfy the declared error, confidence, or quality constraints.

If no alternative meets the contract, execution MUST be rejected, deferred, or terminated according to the defined execution model.

Fallback MUST NOT be a mechanism for silently weakening program requirements.

21. Failure, recovery, and changing resources

Resource availability may change after compilation or planning.

The execution model MUST define behavior for relevant events, including:

- Resource exhaustion.
- Capability revocation.
- Device loss.
- Network partition.
- Target incompatibility.
- Stale capability information.
- Compilation or scheduling budget exhaustion.
- Interrupted execution.
- Failed recovery.
- Invalid or incompatible checkpoints.

The implementation SHOULD use the existing resilience and recovery contracts rather than introducing a separate scalability-specific failure model.

Recovery MUST preserve the applicable state, effect, transaction, provenance, and consistency guarantees.

A failed attempt MUST NOT be represented as a successful result.

If a computation can resume on a different target, the runtime MUST validate the restored state and new target before continuing.

22. Effects, security, and policy enforcement

Scaling MUST NOT weaken the language's effect or security model.

Every realization MUST preserve applicable restrictions concerning:

- I/O and networking.
- Mutation and shared state.
- Native and foreign calls.
- Distributed execution.
- Quantum measurement.
- Hardware access.
- Learning and adaptation.
- Reflection and code generation.
- Simulation and other controlled operations.

Resource negotiation and placement MUST remain subject to the canonical capability and policy models.

A target's availability does not grant permission to use it.

A policy decision MUST NOT be overridden by an optimization hint or a preference for higher performance.

Provenance SHOULD record material decisions about target selection, specialization, fallback, adaptation, and execution when required by the program's contract or applicable policy.

23. Compatibility and versioning

The scalability model MUST evolve through the repository's established compatibility process.

The following contracts MUST remain distinguishable:

- Language specification version.
- Lexer and grammar compatibility.
- AST schema compatibility.
- Semantic-model compatibility.
- Canonical IR compatibility.
- Quantum IR compatibility.
- Dialect and extension compatibility.
- Compilation artifact compatibility.
- Runtime and target-adapter compatibility.

A change to resource-expression semantics, capability interpretation, fallback behavior, portability guarantees, or scalability invariants MUST receive an explicit compatibility assessment.

An extension MUST NOT redefine an existing resource, capability, or constraint in a way that silently changes the meaning of previously valid programs.

Unknown or incompatible artifact versions MUST be rejected or handled through an explicitly defined migration procedure.

24. Normative architectural boundaries

The scalability model depends on strict separation of responsibilities.

Layer| Owns| Must not own
Lexical system| Token recognition and lexical validity| Resource discovery or physical allocation
Grammar| Valid source structure and composition| Runtime capacity decisions
Domain-neutral AST| Structured representation of source meaning| Device selection or physical routing
Structural validation| Structural consistency and required mappings| Runtime resource availability
Type and semantic analysis| Type meaning, effects, contracts, domain semantics| Physical device allocation
Resource semantics| Requirement, constraint, preference, and resource-expression meaning| Live hardware discovery
Capability model| Capability identity and requirement semantics| Unverified claims about available devices
Policy and security| Authorization and permitted actions| Silent policy relaxation
Classical semantic model| Classical computation meaning| Backend-specific machine layout as universal semantics
Quantum semantic model| Quantum operation and resource meaning| Physical qubit assignment
"quantum::ir"| Canonical quantum intermediate representation| A duplicate backend-specific language IR
HDL and hardware semantics| Hardware intent, structure, and constraints| Claims that every design is physically realizable
Compiler and planner| Optimization, lowering, specialization, and plan construction| Changes to specified program meaning
Target adapters| Target capabilities, limits, and implementation interfaces| Redefinition of the core language
Runtime| Discovery, negotiation, placement, execution, monitoring, and recovery| Silent violation of declared contracts
Validation and conformance| Mechanical enforcement and evidence of compliance| Replacing normative specification authority

These boundaries are mandatory even when several layers are implemented in the same Rust crate.

25. Repository integration contract

This document is normative for scalability. It does not supersede the repository's general language-authority model.

25.1 Authority and language specification

"grammar/DESIGN.md"

MUST define the overall grammar architecture, ownership rules, composition boundaries, and relationship between syntax and implementation.

It MUST remain consistent with this document's scalability invariants.

"grammar/specification/language-principles.md"

MUST establish portability, compositionality, explicit contracts, safe implementation, and separation of semantics from realization as language principles.

"grammar/specification/language-scope.md"

MUST define which computational domains are part of the language and how extensions enter the architecture.

"grammar/specification/portability.md"

MUST define portability guarantees, target compatibility, capability matching, and limitations on claims of universal execution.

"grammar/specification/poco-reaf.md"

MUST define the complete POCO-REAF contract, including portable artifacts, target-specific realization, fallback authorization, and long-term compatibility.

"grammar/specification/compilation-model.md"

MUST define the compilation pipeline, intermediate artifacts, lowering, specialization, and the distinction between portable compilation products and target-specific outputs.

"grammar/specification/execution-model.md"

MUST define runtime placement, scheduling, execution, failure handling, and the lifecycle of resource-dependent plans.

"grammar/specification/compatibility.md" and "grammar/specification/language-version.md"

MUST define versioning, compatibility guarantees, and migration requirements.

25.2 Machine-oriented specifications

The corresponding machine-oriented contracts under "grammar/spec/" MUST agree with the normative documents.

In particular:

- "spec/portability.md" MUST represent the same portability invariants.
- "spec/resources.md" MUST define resource requirements and constraints consistently.
- "spec/semantics.md" MUST preserve the separation between program meaning and target realization.
- "spec/quantum.md" MUST preserve logical/physical resource separation.
- "spec/hdl.md" MUST preserve parameterized hardware intent and target-specific validation.

The actual repository paths MUST be confirmed against the manifest before these files are declared complete.

25.3 Core and lexical grammar

The canonical token registry and lexer MUST define resource-related spellings consistently.

Core grammar rules MUST reuse canonical names, expressions, attributes, capabilities, and policy references.

The root grammar MUST compose the appropriate rules without defining independent resource semantics or physical capacity limits.

The grammar MUST NOT encode target discovery, live negotiation, or allocation as parsing behavior.

25.4 Resources and hardware

The canonical resource grammar MUST own the shared syntax for resource declarations and requirements.

The resource-expression grammar MUST own the corresponding expression payloads and reuse canonical expression and name rules.

Specialized resource grammars MUST extend shared resource semantics without duplicating common syntax.

The hardware grammar MUST express abstract target requirements and implementation constraints without introducing universal device counts or fixed capacities.

25.5 Classical, quantum, and HDL domains

The classical domain MUST consume common types, expressions, effects, and resource contracts.

The quantum domain MUST map source constructs through the shared AST and semantic model into "quantum::ir".

The HDL domain MUST map source constructs into the established hardware semantic and IR interfaces.

The hybrid domain MUST compose these domain contracts rather than redefining them.

25.6 Compiler and runtime integration

The compilation and execution specifications MUST define how resource and capability requirements flow into planning, lowering, negotiation, and runtime validation.

Target adapters MUST report actual capabilities and relevant limitations through canonical interfaces.

Scheduling, routing, resilience, and recovery MUST consume established contracts rather than introducing alternative resource definitions.

25.7 Validation and implementation status

The grammar manifest, ownership registry, dependency graph, status registry, and conformance suite MUST identify the relevant files and their actual implementation status.

The implementation conformance document MUST distinguish specified, implemented, partially implemented, planned, experimental, and deprecated behavior.

A normative requirement MUST NOT be marked implemented solely because a corresponding file exists.

26. Rust implementation requirements

The Rust implementation MUST use Rust 1.97 or later and the repository's declared edition and dependency policies.

26.1 Memory safety

All implementation code MUST use safe Rust.

The project MUST prohibit "unsafe" blocks, "unsafe" functions, and other prohibited unsafe constructs through repository linting and CI enforcement.

The implementation MUST NOT circumvent this requirement through generated source or unchecked third-party integration code without an explicitly approved repository-wide policy change.

26.2 Data modeling

Resource requirements, capabilities, constraints, preferences, and hints SHOULD have distinct typed representations.

The implementation SHOULD avoid using unvalidated strings or loosely typed maps where a canonical typed model exists.

Extensible identifiers SHOULD use a validated, namespaced representation that does not require editing a closed enum whenever a new external capability is introduced.

The implementation MUST validate external and serialized input before using it in resource planning or execution.

26.3 Arithmetic and allocation

Resource arithmetic MUST handle overflow explicitly.

Collection allocation, parsing, compilation, and deserialization MUST account for resource exhaustion and malformed inputs.

The implementation SHOULD use checked arithmetic, fallible allocation interfaces where appropriate, and bounded processing of untrusted data.

A fallible allocation strategy MUST NOT be represented as a guarantee that the allocator can recover from every process-level out-of-memory condition.

26.4 Concurrency

Concurrent resource discovery, negotiation, planning, and execution MUST preserve the synchronization and ownership contracts.

Shared mutable state MUST be managed through safe Rust abstractions and the repository's established concurrency model.

The implementation MUST NOT assume that a successful resource query reserves the queried resources unless the underlying interface explicitly guarantees a reservation.

26.5 Error handling

Resource and scalability failures MUST use the repository's canonical diagnostic and error conventions.

Errors SHOULD distinguish invalid source, unsatisfied requirements, unsupported capabilities, unavailable resources, incompatible artifacts, policy rejection, and implementation-budget exhaustion.

The compiler and runtime MUST NOT silently convert an error into success.

27. Diagnostics

The language implementation MUST provide actionable diagnostics for violations of this specification.

The exact diagnostic identifiers MUST be registered in the canonical diagnostic registry rather than independently invented in individual grammars.

The following are required diagnostic categories; the names below are descriptive labels, not reserved source keywords.

Category| Required meaning
Invalid resource expression| A resource expression is syntactically or semantically invalid.
Incompatible resource units| A resource comparison uses incompatible dimensions or units.
Resource arithmetic failure| A required calculation overflows or cannot be represented safely.
Unsatisfied resource requirement| No admissible realization satisfies a mandatory resource requirement.
Unsupported capability| A required capability is unavailable or unsupported.
Unknown capability| The capability cannot be established as supported.
Constraint violation| A candidate realization violates a mandatory constraint.
Policy rejection| An applicable policy prohibits the proposed action or realization.
Invalid fallback| A fallback is unauthorized or fails the required semantic contract.
Target incompatibility| The target cannot meet the program's declared contract.
Stale execution plan| A relevant planning assumption is no longer valid.
Implementation budget exhausted| A compiler or runtime budget has been reached.
Artifact incompatibility| An artifact cannot be safely consumed under the current compatibility contract.
Unsupported scalability feature| A feature is specified but is not implemented by the current toolchain.

Diagnostics SHOULD identify the violated requirement, its source location, the evaluation stage, and relevant remediation options.

Diagnostics MUST NOT disclose sensitive hardware, security, or infrastructure details to callers who are not authorized to receive them.

28. Scalability conformance testing

Conformance testing MUST evaluate both language-level invariants and implementation behavior.

28.1 Static architecture checks

Automated validation MUST check for:

- Universal hard-coded resource ceilings.
- Duplicate ownership of resource or capability symbols.
- Duplicate or competing resource-expression rules.
- Unresolved grammar imports.
- Circular grammar dependencies.
- Undocumented grammar exports.
- Missing AST mappings.
- Missing semantic mappings.
- Missing required IR mappings.
- Domain leakage into the core grammar.
- Invalid or undocumented token ownership.
- Conflicting normative definitions.
- Unsafe Rust constructs.
- Unregistered diagnostics.
- Missing conformance tests for production features.

The audit MUST distinguish legitimate target-specific limits from prohibited universal language limits.

28.2 Resource model tests

Tests MUST cover:

- Symbolic resource requirements.
- Parameterized quantities.
- Valid and invalid unit conversions.
- Arithmetic overflow.
- Unknown capabilities.
- Unsatisfied constraints.
- Preference ranking.
- Policy rejection.
- Resource exhaustion.
- Stale capability information.
- Explicit fallback.
- Incompatible targets.
- Invalid artifact versions.

28.3 Cross-domain tests

The conformance suite MUST include representative classical, quantum, HDL, hybrid, accelerator, and distributed workloads.

Tests MUST verify that each domain uses the common resource, capability, policy, effect, and compatibility contracts where applicable.

28.4 Scale-parameter tests

The same source program SHOULD be tested with multiple problem-size parameters and different available resource profiles.

The test suite MUST distinguish:

1. The program's valid semantic domain.
2. The target's supported domain.
3. The resources available during the test.
4. The implementation's operational budgets.
5. The result required by the program's contract.

A test MUST NOT interpret a failure caused by intentionally insufficient resources as evidence that the language has a universal capacity limit.

Conversely, a successful small-scale execution MUST NOT be treated as proof that all larger executions are supported.

28.5 Differential and semantic-equivalence tests

Where multiple targets or realization strategies claim equivalent behavior, tests SHOULD compare their observable results under the specified equivalence contract.

For floating-point, probabilistic, quantum, distributed, and timing-sensitive computations, tests MUST use the domain's defined comparison and guarantee model rather than assuming universal bit-for-bit equality.

28.6 Property-based and fuzz testing

The implementation SHOULD use property-based testing and fuzzing for resource expressions, parameterized structures, capability metadata, artifact decoding, and negotiation inputs.

Malformed or adversarial input MUST NOT cause undefined behavior, unchecked resource arithmetic, or silent policy bypass.

29. Hard-coding audit

The following rule is mandatory:

«No finite implementation capacity may be presented as a universal language limit merely because a current compiler, runtime, target, or test environment supports only a finite range.»

The audit MUST inspect more than literal names.

It MUST identify equivalent restrictions hidden in:

- Grammar alternatives.
- Repeated fixed declarations.
- Closed device enumerations.
- Fixed array lengths used as universal capacities.
- Fixed-width counters used without overflow handling.
- Arbitrary validation thresholds.
- Unconfigurable global constants.
- Backend-specific assumptions embedded in shared semantics.
- Silent truncation or clamping.
- Defaults that accidentally become mandatory limits.

Legitimate finite values MAY exist when they represent actual language semantics, explicit user constraints, protocol-defined bounds, safety budgets, or target-specific implementation limits.

Each such value MUST have a documented owner, scope, purpose, enforcement point, and compatibility consequence.

30. Observability and operational evidence

Implementations SHOULD expose sufficient structured information to explain scalability decisions.

Subject to security and privacy policy, this information SHOULD include:

- Evaluated resource requirements.
- Capability-match results.
- Rejected candidate reasons.
- Selected realization and relevant alternatives.
- Applied constraints and preferences.
- Resource-budget exhaustion.
- Fallback decisions.
- Material plan changes.
- Relevant recovery events.
- Conformance and reproducibility information.

Observability MUST NOT change program semantics or grant additional permissions.

The runtime MUST distinguish actual measurements from estimates, predictions, cached information, and declared requirements.

31. Extensibility

New computational domains, capabilities, resources, target adapters, and dialects MUST integrate through established extension contracts.

Adding a new supported target SHOULD require implementation of the appropriate capability discovery, resource reporting, lowering, execution, and validation interfaces—not modification of universal grammar rules merely to add the target's name.

Adding a new operation SHOULD use the established operation metadata and semantic mapping contracts.

An extension MUST declare its dependencies, version, compatibility requirements, semantic mapping, applicable effects, capabilities, resource requirements, diagnostics, and conformance tests.

No extension MAY bypass type checking, structural validation, policy enforcement, or the canonical semantic and IR boundaries.

32. Explicit non-goals

This specification does not guarantee:

- Literally infinite physical memory or execution time.
- That every mathematical computation terminates.
- That every target supports every language feature.
- That every valid HDL design can be synthesized.
- That every quantum algorithm can run on currently available quantum hardware.
- That adding processors always improves performance.
- That distributed execution preserves ordering or consistency without an appropriate model.
- That all numerical implementations produce identical bit patterns.
- That one native binary runs unchanged on incompatible instruction sets.
- That every resource requirement can be satisfied.
- That all future hardware can be predicted in advance.

These limitations do not weaken the language's obligation to avoid arbitrary universal capacity ceilings or to preserve its documented portability contracts.

33. Required invariants

A conforming Zamani implementation MUST preserve all of the following invariants:

1. Semantic independence: Program meaning is not defined by a particular machine's capacity.
2. No universal capacity ceilings: The language does not impose arbitrary global maximums on resource populations.
3. Explicit requirements: Mandatory resource and capability needs are represented and evaluated explicitly.
4. Unknown is not satisfied: Unverified capabilities and resource facts cannot be assumed available.
5. Constraints are mandatory: Optimization and preference ranking cannot override required constraints.
6. Hints are non-semantic: Hints cannot change the specified meaning of a program.
7. Policy preservation: Resource availability cannot override authorization or policy.
8. Target validation: An implementation must establish target compatibility before claiming a valid realization.
9. No silent downgrade: Alternatives cannot silently weaken the program's contract.
10. Canonical semantic model: All domains use the established common semantic architecture.
11. Canonical quantum IR: Quantum lowering uses "quantum::ir".
12. Hardware separation: Physical allocation, routing, calibration, and scheduling remain realization responsibilities.
13. Safe Rust: Implementation code contains no prohibited unsafe constructs.
14. Explicit failure: Unmet requirements and exhausted budgets are reported through defined outcomes.
15. Versioned compatibility: Source and artifacts follow declared version and migration contracts.
16. Auditable extensibility: New capabilities and targets integrate through versioned interfaces.
17. Evidence-based conformance: Production readiness is demonstrated through automated checks and tests, not file presence alone.

34. Freeze criteria

This document MUST NOT be marked stable until all of the following have been completed.

Specification

- [ ] The normative language-authority model is established.
- [ ] The scalability, portability, compilation, and execution specifications agree.
- [ ] Resource, capability, constraint, preference, hint, and policy meanings are unambiguous.
- [ ] Fallback and compatibility guarantees are explicit.
- [ ] Domain-specific scalability boundaries are documented.

Repository integration

- [ ] All referenced files have been verified against the repository manifest.
- [ ] Every normative cross-reference resolves to an authoritative contract.
- [ ] Resource and capability concepts have one canonical owner.
- [ ] Grammar, AST, semantic, IR, compiler, and runtime responsibilities are mapped.
- [ ] The canonical "quantum::ir" boundary is preserved.
- [ ] Implementation status is accurate and evidence-based.

Validation

- [ ] The hard-coding audit passes.
- [ ] The dependency and ownership audits pass.
- [ ] The grammar generation and parser tests pass.
- [ ] The resource and capability conformance suite passes.
- [ ] Cross-domain tests pass for implemented features.
- [ ] Portability and scale-parameter tests pass.
- [ ] Negative tests confirm that invalid fallbacks and unmet requirements are rejected.
- [ ] Rust checks enforce the required safe-code policy.
- [ ] Diagnostics and compatibility behavior are tested.
- [ ] The document's claims are supported by reproducible validation results.

Freeze decision

The file MAY be marked "STABLE" only when the normative requirements are complete, repository integration has been verified, and the required conformance evidence is available.

A specification can be frozen while some future capabilities remain planned, provided that their status is explicit and no unsupported implementation claim is made.

A later change to a normative invariant MUST trigger compatibility review, affected-contract analysis, and the repository's prescribed unfreezing procedure.

35. Final contract

Zamani SHALL define a language capable of expressing computations independently of any one machine's finite capacity.

Its implementations SHALL realize those computations using compatible resources, preserve the declared semantic and operational guarantees, and report when required conditions cannot be met.

Its architecture SHALL allow new scales, targets, capabilities, and computational domains to be introduced through stable interfaces rather than through arbitrary universal ceilings or repeated redesign of the language core.

The governing principle is:

«Express meaning once. Preserve that meaning across scale. Negotiate realization against actual capabilities and resources. Reject incompatible execution rather than silently weakening the contract. Extend the implementation without imposing artificial limits on the language.»

That is the normative scalability foundation for POCO-REAF in Zamani.