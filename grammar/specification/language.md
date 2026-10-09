Zamani Language Specification

Canonical path: "grammar/specification/language.md"

Status: Normative language-wide specification

Language: Zamani

Language implementation baseline: Rust 2021; Rust 1.97 or later

Implementation safety requirement: Zamani-owned Rust production code MUST NOT use Rust "unsafe" code.

Primary portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

Scalability objective: Support computations from the smallest meaningful computation to arbitrarily large computations without imposing arbitrary language-level capacity limits, subject to program semantics, representational requirements, implementation capabilities, available resources, physical constraints, and explicitly declared policies.

---

1. Purpose

This document defines the normative identity, scope, fundamental principles, and cross-domain architectural contracts of the Zamani programming language.

It establishes the requirements that the language's specialized specifications, grammar, lexical system, parser, abstract syntax tree (AST), semantic analysis, intermediate representations (IRs), compiler, runtime, tooling, and target integrations MUST follow.

Zamani is designed as one extensible programming language for:

- Classical and systems programming.
- Embedded and resource-constrained computing.
- Scientific, numerical, and symbolic computing.
- Quantum computing and quantum error correction.
- Hybrid quantum-classical computation.
- Hardware description and hardware/software co-design.
- Parallel, concurrent, distributed, and high-performance computing.
- Artificial intelligence, machine learning, and neural-symbolic computation.
- Tensor, stream, and large-scale data processing.
- Networking and communication.
- Security and cryptography.
- Heterogeneous computing and accelerators.
- Edge, cluster, cloud, and supercomputing environments.
- Nano-oriented and physical-scale computational models.
- Temporal and multi-timeline computation.
- Metaprogramming and controlled reflection.
- Foreign-function interfaces and external-format interoperability.
- Future computational domains.

These domains MUST compose within one coherent language model. They MUST NOT create competing universal parsers, incompatible type systems, duplicated semantic authorities, or ungoverned intermediate representations.

This specification defines language requirements and architectural invariants. It does not assert that every planned construct, backend, target, or runtime service already exists.

A feature is implemented only to the extent demonstrated by its implementation, tests, and integration contracts.

2. Normative terminology

The words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, MAY, and OPTIONAL are normative.

The following terms have precise meanings throughout this specification.

Term| Meaning
Language specification| The normative definition of Zamani's syntax and meaning.
Grammar| A formal description of the source constructs accepted by a parser.
Lexer| A component that converts source text into tokens.
Parser| A component that constructs a syntactic representation from tokens.
AST| An abstract syntax tree representing parsed source constructs and source locations.
Semantic model| The validated interpretation of a program, including types, effects, contracts, resources, capabilities, and domain meaning.
IR| An intermediate representation used by one or more compiler stages.
Canonical IR| A versioned representation governed by an explicit semantic and compatibility contract.
Domain| A family of computational concepts with defined syntax, semantics, and integration boundaries.
Capability| A property or operation that an execution environment supports.
Resource| A consumable, reservable, measurable, or otherwise constrained quantity or service.
Requirement| A condition that MUST hold for a program or execution to be valid.
Constraint| A restriction on permissible implementations or realizations.
Preference| A non-mandatory selection criterion.
Hint| Optimization guidance that MUST NOT change program meaning.
Policy| An explicit rule governing permitted operations, execution, resource use, security, fallback, or adaptation.
Effect| A description of an operation's observable or semantically relevant consequences.
Contract| A verifiable precondition, postcondition, invariant, assumption, guarantee, or property.
Provenance| Information recording origin, evidence, transformation, or decision history.
Realization| The mapping of validated program meaning onto a concrete computational environment.
Target| A computational environment or execution substrate selected for a realization.
Dialect| A versioned and explicitly governed language extension or external-format boundary.
Conformance| Demonstrated satisfaction of applicable language and implementation requirements.
Frozen file| A file whose ownership, interfaces, dependencies, tests, compatibility requirements, and completion criteria are satisfied and protected by change control.

3. Normative authority

3.1 Authority hierarchy

Zamani MUST have one normative language definition.

The repository's responsibilities are separated as follows:

1. "grammar/specification/" defines the normative human-readable language contracts.
2. "grammar/spec/" contains machine-oriented contracts, schemas, registries, and mechanically verifiable invariants.
3. "grammar/DESIGN.md" defines the grammar architecture, ownership rules, integration boundaries, and freeze governance.
4. "grammar/Zamani.g4" is the canonical ANTLR grammar composition root.
5. "grammar/antlr/ZamaniLexer.g4" and "grammar/antlr/ZamaniParser.g4" define the public ANTLR lexer and parser composition boundaries.
6. The modular grammar files under "grammar/" implement the grammar contracts assigned to their respective owners.
7. "src/lexer.rs" and "src/parser.rs" are existing Rust frontend implementation points whose accepted language MUST conform to the normative syntax and semantics.
8. "src/ast/" contains the existing AST integration point. Any additional frontend AST paths MUST have an explicitly documented ownership and migration contract.
9. Semantic-analysis components define validated program meaning according to this specification and the specialized semantic contracts.
10. Canonical domain IRs represent validated computational meaning.
11. Compiler, optimization, routing, scheduling, resilience, ZQN, HAL, backend, and runtime components realize that meaning on compatible execution environments.
12. "grammar/grammar.md" documents implementation conformance and feature status.
13. "grammar/Zamani-Grammar.md" preserves extended, historical, proposed, and design-reference material. Its contents do not automatically become normative language law.

3.2 No competing authorities

No implementation file, generated document, domain grammar, dialect, example, or historical reference MAY silently override this specification.

When an inconsistency is discovered, the affected feature MUST be marked as non-conforming or unresolved until the authoritative specification, implementation, and tests agree.

A proposed change to normative behavior MUST follow the language change and compatibility process.

3.3 Normative dependencies

This document owns language-wide principles. Specialized specifications own their respective detailed rules.

Subject| Normative specification
Overall architecture and file ownership| "grammar/DESIGN.md"
Lexical behavior| "grammar/specification/lexical.md"
Syntax and grammar composition| "grammar/specification/syntax.md"
Program meaning| "grammar/specification/semantics.md"
Types and type checking| "grammar/specification/types.md"
Computational domains| "grammar/specification/domains.md"
Portability| "grammar/specification/portability.md"
POCO-REAF| "grammar/specification/poco-reaf.md"
Scalability| "grammar/specification/scalability-model.md"
Compilation| "grammar/specification/compilation-model.md"
Execution| "grammar/specification/execution-model.md"
Compatibility| "grammar/specification/compatibility.md"
Extensibility| "grammar/specification/extensibility.md"
Grammar authority| "grammar/specification/grammar-authority.md"
Machine-checkable contracts| "grammar/spec/" and registered schemas

A specialized specification MAY strengthen a requirement within its subject area. It MUST NOT weaken a language-wide invariant or introduce contradictory semantics.

If two normative documents conflict, implementation MUST NOT resolve the conflict through undocumented precedence or local interpretation. The conflict MUST be recorded, resolved by a specification change, and covered by conformance tests.

4. Language identity

Zamani is a general-purpose, extensible programming language that expresses computational meaning independently of a particular physical machine.

Its foundational model separates:

1. Source syntax.
2. Program structure.
3. Static and dynamic semantics.
4. Type and ownership rules.
5. Effects and observable behavior.
6. Requirements, constraints, and policies.
7. Capability and resource obligations.
8. Domain-specific computational meaning.
9. Compilation and realization decisions.
10. Runtime behavior and execution outcomes.

Every supported domain MUST integrate with this shared foundation.

A domain MAY introduce specialized types, operations, declarations, attributes, effects, and syntax when justified by its semantics. It MUST NOT redefine universal identifiers, source locations, ordinary expression meaning, common type rules, resource requirements, effect composition, or diagnostics without an explicitly approved language-wide change.

The language MUST permit new algorithms, operations, devices, and computational models to be introduced through appropriate semantic extensions, registered dialects, libraries, capabilities, and backend integrations where possible.

New devices MUST NOT require new universal language keywords merely because they are new devices.

5. Fundamental semantic principle

Zamani distinguishes what a program means from how that meaning is realized.

The normative pipeline is:

Zamani source
      |
      v
Lexical analysis
      |
      v
Parsing
      |
      v
Domain-neutral AST + source locations
      |
      v
Structural validation
      |
      v
Name and module resolution
      |
      v
Type, ownership, effect, and contract analysis
      |
      v
Capability, resource, and policy validation
      |
      v
Provenance and semantic validation
      |
      v
Canonical semantic model
      |
      +----------------+----------------+
      |                |                |
      v                v                v
  Classical IR     quantum::ir     HDL/hardware IR
      |                |                |
      +----------------+----------------+
                       |
                       v
             Correctness-preserving
             optimization and lowering
                       |
                       v
              Target negotiation
                       |
                       v
              Placement and routing
                       |
                       v
                 Scheduling
                       |
                       v
             Resilience and recovery
                       |
                       v
                      ZQN
                       |
                       v
                      HAL
                       |
                       v
             Compatible realization

This pipeline is an architectural contract, not a claim that every stage is fully implemented in the current repository.

5.1 Separation of responsibility

The grammar defines syntax.

The AST records source structure and locations.

Semantic analysis determines whether a program is well-formed and what it means.

The canonical semantic model records validated meaning.

Domain IRs represent computational intent in their respective domains.

Compiler and runtime components determine how validated meaning can be realized.

A grammar production MUST NOT be treated as proof of semantic validity.

A valid AST MUST NOT be treated as proof that the required capabilities or resources exist.

A successful compilation MUST NOT be treated as proof that every target can execute the artifact.

A target selection MUST NOT silently alter observable program semantics.

6. POCO-REAF

POCO-REAF is the language's portability objective:

Program Once, Compile Once, Run Everywhere, Anywhere, Forever.

It requires semantic portability, not the impossible guarantee that every program executes unchanged on every conceivable physical system.

6.1 Program once

A program SHOULD express its computational purpose, types, correctness conditions, effects, capabilities, resource requirements, constraints, and permitted alternatives without encoding incidental properties of one machine.

The same source MUST NOT require modification merely because a compatible target has a different processor count, memory capacity, accelerator configuration, or physical topology.

Explicit target dependence MAY be expressed when it is part of the program's intended semantics or an explicitly accepted portability restriction.

6.2 Compile once

The portable compilation artifact SHOULD preserve the validated program meaning and the information required for later realization.

Where the implementation supports portable semantic artifacts, they MUST identify or bind:

- Language edition and semantic version.
- Relevant feature and dialect versions.
- Canonical semantic representation version.
- Type, effect, and capability requirements.
- Resource requirements and constraints.
- Applicable policies and correctness contracts.
- Required runtime services.
- Provenance and reproducibility information.
- Any target-specific assumptions that restrict portability.

Machine-specific binaries, native instructions, physical routing plans, calibration-dependent schedules, and device-bound executables MUST be distinguished from portable semantic artifacts.

A machine-specific binary MUST NOT be described as universally portable merely because it originated from Zamani source.

6.3 Run everywhere

A conforming implementation SHOULD attempt to realize a program on any compatible target satisfying its requirements, constraints, policies, and correctness obligations.

Target discovery, capability negotiation, resource negotiation, lowering, placement, and scheduling belong to the compiler/runtime realization layers.

The language MUST NOT require source-level enumeration of every future processor, accelerator, QPU, FPGA, ASIC, simulator, or distributed platform.

6.4 Run anywhere

When several valid realizations exist, implementation policy MAY select among them using declared preferences, permitted optimization criteria, availability, performance objectives, or deployment configuration.

The selected realization MUST satisfy the program's semantic requirements and applicable policies.

If no valid realization exists, the implementation MUST report the failure rather than silently inventing capabilities or weakening mandatory requirements.

6.5 Run forever

Long-term compatibility requires versioned semantics, preserved artifacts, compatibility rules, migration mechanisms, reproducibility information, and documented support boundaries.

It does not promise that a program will execute on nonexistent hardware, that discontinued services will remain available, or that an old binary will run on every future operating system.

Future implementations SHOULD preserve established semantics and SHOULD support documented migration paths when compatibility cannot be maintained.

6.6 Portability classes

Implementations SHOULD distinguish at least:

- Semantically portable: meaning does not depend on a specific target.
- Capability-constrained: execution requires declared capabilities.
- Resource-constrained: execution requires declared resource conditions.
- Policy-constrained: execution is permitted only under applicable policies.
- Target-specialized: a realization depends on target-specific properties.
- Non-portable: correctness or behavior explicitly depends on a particular environment.

Portability classification MUST NOT be confused with implementation status.

7. Scalability: from tiny to arbitrarily large

Zamani MUST NOT impose arbitrary finite language-level limits on scalable computational resources.

This applies to, but is not limited to:

- Logical and physical qubits.
- Classical processors and processor cores.
- Threads, tasks, actors, and processes.
- GPUs and other accelerators.
- FPGAs and ASIC resources.
- Distributed nodes and network participants.
- Memory and storage quantities.
- Register widths and vector widths.
- Tensor dimensions and ranks.
- Collection and stream sizes.
- Quantum circuit operations and depth.
- Hardware ports, signals, and parameterized widths.
- Graphs, datasets, and computational pipelines.

The term unbounded means that the language does not impose an arbitrary fixed ceiling on these resources where its semantics do not inherently require one.

It does not mean that every mathematical value is representable by every implementation or that physical execution can use infinite resources.

7.1 Representation independence

A source-level quantity MUST NOT acquire an undocumented fixed machine capacity merely because a current implementation uses a particular integer type, collection implementation, allocator, or backend.

If a representation imposes a practical bound, the implementation MUST document the bound and handle it according to the relevant error and compatibility contract.

Where a computation requires values beyond an implementation's supported representation, the implementation MUST report the limitation or use a conforming alternative representation where available.

It MUST NOT silently wrap, truncate, overflow, or reinterpret the value unless such behavior is explicitly defined by the relevant numeric semantics.

7.2 Scale-independent meaning

Changing a resource parameter MUST NOT change the intended meaning of a program merely because the computation becomes larger.

For example, a vector-parametric computation SHOULD be able to use a small vector, a larger vector, or a target-selected vector extent without requiring a different algorithm solely because of capacity.

Scaling may change execution time, memory usage, numerical results where explicitly permitted by numerical semantics, resource feasibility, and target realization. It MUST NOT silently change the program's specified correctness conditions.

7.3 Resource availability

Actual execution remains bounded by:

- Resources available to the implementation.
- Target capabilities.
- Representational and mathematical requirements.
- Operating-system and runtime limits.
- Explicit security and resource policies.
- Physical constraints.
- The program's own requirements and constraints.

These limits MUST be reported at the correct layer rather than embedded as arbitrary universal grammar restrictions.

8. Prohibition on arbitrary hard-coded capacity limits

No universal language rule, grammar production, type definition, or canonical semantic contract may require a fixed machine capacity solely to simplify implementation.

The following are prohibited as universal language limits:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

Equivalent disguised fixed ceilings are prohibited as well.

This prohibition does not forbid legitimate finite values in programs, explicit resource budgets, physical design parameters, safety policies, algorithmic bounds, or target-specific contracts.

8.1 Valid resource expressions

The language MUST support an extensible semantic model for requirements such as:

requires qubits >= n
requires memory >= required_memory
requires capability("quantum.measurement")
requires capability("gpu.compute")
requires capability("tensor.compute")
requires topology(...)

It SHOULD support distinct requirement, constraint, preference, hint, permission, and prohibition concepts.

The exact syntax and typing of these expressions are defined by the relevant syntax, resource, capability, and policy specifications.

8.2 Meaning of a requirement

A resource requirement expresses a condition that must be satisfied for a specified operation or realization.

It does not allocate a physical resource, identify a device, prove that a target exists, or guarantee successful execution.

A capability requirement expresses a semantic dependency on a supported operation or property. It does not assert that every target implements that capability.

8.3 Audit requirement

All grammar and implementation changes MUST be checked for arbitrary capacity limits, including indirect limits hidden in enumerations, array sizes, type definitions, parser assumptions, IR layouts, generated code, and runtime configuration.

Every exception MUST have a documented semantic or physical justification and an explicit scope.

9. Requirements, constraints, preferences, hints, and policies

These concepts MUST remain distinct.

9.1 Requirements

A requirement is mandatory. If it cannot be satisfied, the implementation MUST reject that realization or report that execution is unavailable.

9.2 Constraints

A constraint excludes otherwise possible realizations that violate an explicitly stated condition.

Constraints MUST be checked at the appropriate semantic, compilation, deployment, or execution stage.

9.3 Preferences

A preference ranks otherwise valid realizations. It MUST NOT override a mandatory requirement or constraint.

9.4 Hints

A hint suggests a possible optimization or realization strategy. It MUST NOT change program meaning or override mandatory constraints.

An implementation MAY ignore a hint.

9.5 Policies

Policies govern permitted behavior, including resource use, security, adaptation, fallback, deployment, and execution.

A policy MUST NOT silently convert an unauthorized action into an authorized one.

9.6 No category substitution

An implementation MUST NOT treat a preference as a requirement, a hint as a constraint, or a policy prohibition as optional advice.

When conflicts arise, the implementation MUST apply the precedence and conflict-resolution rules defined by the relevant specifications. Undocumented resolution is non-conforming.

10. Source-program model

A Zamani program consists of source units and their referenced declarations, modules, packages, dialects, and external dependencies.

The source-program model MUST support:

- Multiple source files.
- Module and package organization.
- Qualified names and imports.
- Declarations and definitions.
- Expressions and statements.
- Types and generic parameters.
- Attributes and modifiers.
- Contracts and effect declarations.
- Resource and capability requirements.
- Domain-specific constructs.
- Explicit interoperability boundaries.
- Versioned language features and dialects.

The exact syntax belongs to "grammar/specification/syntax.md" and the canonical modular grammar.

10.1 Program identity

Implementations SHOULD provide stable identities for source units, modules, and build inputs.

Identity MUST NOT depend solely on a physical device or transient runtime allocation.

10.2 Compilation units

A compilation unit MUST have a well-defined boundary, version context, dependency set, and diagnostic context.

The meaning of an imported unit MUST be resolved according to language and package compatibility rules, not by silently selecting whichever implementation happens to be installed.

10.3 External inputs

External source, generated code, foreign libraries, dialect definitions, schemas, and compile-time inputs MUST be explicitly identified when they can affect program meaning or reproducibility.

An implementation MUST NOT silently treat an untracked external input as part of a reproducible build.

11. Source locations, diagnostics, and provenance

Every parsed construct SHOULD retain a source span or a documented synthetic-origin record.

Diagnostics MUST identify the relevant source location whenever one is available.

Diagnostics SHOULD include:

- A stable diagnostic identifier.
- A severity.
- A source location or generated-origin location.
- A clear explanation.
- A relevant rule or contract.
- An actionable correction where possible.
- Related locations when useful.
- Provenance when a generated or transformed construct is involved.

Diagnostic wording MAY improve without changing language semantics. Stable identifiers and machine-readable fields SHOULD be compatibility-controlled.

Generated AST nodes and transformed IR operations MUST retain sufficient origin information for meaningful diagnostics and audit trails.

Provenance MUST distinguish facts derived from source code, facts supplied by configuration, facts discovered from the environment, and decisions made during realization.

12. Lexical model

The language has one normative lexical model.

The lexical specification and token registry define the accepted token vocabulary, token identities, literal rules, comments, whitespace handling, and keyword behavior.

The lexer implementation MUST conform to those rules.

The ANTLR lexer and the existing Rust lexer MUST NOT independently define incompatible token meanings.

12.1 Token ownership

Every stable token MUST have one authoritative owner and a documented spelling, category, compatibility status, and conformance test.

Equivalent tokens MUST NOT be duplicated without a defined semantic distinction.

Contextual keywords, reserved words, dialect keywords, deprecated tokens, and experimental vocabulary MUST have explicit classifications.

12.2 Keyword extensibility

New domain operations SHOULD use registered operation names, namespaces, attributes, or dialect metadata when dedicated keywords are unnecessary.

Adding a new hardware device, quantum operation, AI algorithm, or vendor feature MUST NOT require a new universal keyword by default.

12.3 Literal semantics

Literal spelling and parsing are defined by the lexical and syntax specifications. Literal meaning is defined by semantic and type specifications.

Resource quantities, symbolic dimensions, physical quantities, probabilities, and domain-specific literals MUST NOT silently encode implementation capacity limits.

12.4 Source encoding

The accepted source encoding, identifier character rules, Unicode handling, escape processing, and normalization rules MUST be defined by the lexical specification.

Implementations MUST NOT silently interpret the same source text differently because of undocumented encoding behavior.

13. Names, declarations, modules, and functions

Zamani uses one common model for identifiers, qualified names, declarations, modules, visibility, functions, and imports.

The relevant files under "grammar/core/", "grammar/modules/", "grammar/declarations/", and "grammar/functions/" own the corresponding grammar details.

13.1 Names

Name resolution MUST be deterministic under the specified module, import, visibility, and shadowing rules.

The same name MUST NOT resolve to different declarations merely because the compiler selected a different target, unless target-dependent name resolution is explicitly specified and the program's portability classification reflects that dependency.

13.2 Declarations

Declarations introduce entities such as values, types, functions, modules, effects, contracts, resources, and domain-specific objects.

Each declaration kind MUST define its ownership, visibility, scope, type behavior, effect behavior, and compatibility requirements.

13.3 Functions

The function model MUST integrate with common parameter, return-type, generic, constraint, effect, contract, and calling-convention rules.

Domain-specific functions MAY expose classical, quantum, hardware, distributed, AI, or foreign operations, but MUST use the shared semantic model.

13.4 Modules and packages

Modules and packages MUST have explicit identity and versioning behavior.

Package resolution MUST NOT silently substitute an incompatible dependency.

Reproducible builds SHOULD record the resolved dependency identities and relevant configuration.

14. Types and genericity

Zamani has one common type-system authority.

The detailed rules belong to "grammar/specification/types.md" and the corresponding type-system contracts.

The type system MUST describe semantic values and permitted operations, not accidental implementation layouts.

It MUST provide a defined integration path for:

- Primitive and named types.
- Numeric and symbolic types.
- Product and sum types.
- Records, structures, and enumerations.
- Functions, closures, and callable values.
- Arrays, slices, collections, and tensors.
- References, pointers, ownership, and lifetimes.
- Generic and constrained types.
- Associated types and trait or interface constraints.
- Linear and affine types.
- Resource-aware types.
- Capability-aware types.
- Effect-aware types.
- Quantum and hardware-domain types.
- Refinement and value-parameterized types where implemented.

A feature MUST NOT be declared supported merely because its syntax parses.

14.1 Genericity

Generic parameters and constraints MUST be expressed independently of fixed target capacities wherever the semantics permit.

A type such as "Vector<T, N>" MUST NOT inherently mean that "N" equals a machine register width or another fixed physical capacity.

14.2 Symbolic dimensions

Symbolic dimensions MUST retain their declared meaning until the relevant semantic or specialization stage resolves them.

Implementations MUST NOT silently replace a symbolic dimension with an arbitrary constant.

If a dimension cannot be represented or realized, the implementation MUST report the relevant limitation.

14.3 Numeric behavior

Integer range, overflow, floating-point behavior, exact arithmetic, numeric conversion, and cross-target reproducibility MUST be defined by the type and semantic specifications.

The language MUST NOT imply arbitrary-precision behavior for an implementation that silently truncates values to a narrower representation.

Where multiple conforming numerical implementations are permitted, the specification MUST define which results are exact, bounded, implementation-dependent, or explicitly nondeterministic.

15. Memory, ownership, and resource lifetime

Zamani MUST define coherent ownership, borrowing, reference, allocation, lifetime, and resource-management rules.

The source-level memory model MUST remain distinct from physical memory placement and available capacity.

The language MUST permit implementations to realize valid memory operations using appropriate target mechanisms, including host memory, accelerator memory, shared memory, distributed memory, persistent storage, and domain-specific memory models where supported.

15.1 Ownership

Ownership and borrowing rules MUST be checked according to the type and semantic specifications.

Linear or affine resources MUST NOT be duplicated or discarded contrary to their declared semantics.

15.2 Allocation

Allocation requests express semantic requirements or operations. They do not guarantee that the requested capacity exists.

Allocation failure, resource exhaustion, and permitted recovery behavior MUST be specified.

15.3 Address independence

Physical addresses, device addresses, and target-specific memory locations MUST NOT be treated as universally portable identities.

Code that depends on concrete addresses MUST declare the applicable assumptions and portability restrictions.

15.4 Resource release

The lifetime and release behavior of owned resources MUST be defined.

Resource cleanup and recovery mechanisms MUST NOT violate observable program semantics or domain-specific correctness conditions.

16. Effects and observable behavior

Zamani MUST have one common effect model.

Effects describe consequences that are relevant to program meaning, such as:

- Input and output.
- Mutation.
- Randomness.
- Network communication.
- Foreign calls.
- Distributed communication.
- Security-sensitive operations.
- Quantum measurement.
- Hardware interaction.
- Learning and adaptation.
- Reflection and code generation.
- Simulation and external services.

This list is extensible and MUST NOT be treated as a closed universal inventory.

Every effect category MUST define its identity, composition, relevant parameters, typing rules, authorization requirements, diagnostics, and compatibility behavior.

16.1 Effect checking

A program MUST satisfy the effect constraints applicable to its declarations, execution context, contracts, and policies.

An optimization MUST NOT remove or reorder observable effects unless the relevant semantic rules explicitly permit it.

16.2 Effect polymorphism

Where effect polymorphism is supported, it MUST compose with ordinary types, generic constraints, capabilities, and contracts.

Domain-specific effect systems MUST NOT silently bypass the common effect authority.

16.3 Failure behavior

Errors, exceptions, failed requirements, unavailable resources, cancellation, and recovery MUST have defined semantic behavior.

A failure MUST NOT be converted into a successful result without an explicitly permitted recovery or fallback contract.

17. Determinism and reproducibility

Zamani MUST distinguish deterministic behavior from nondeterministic behavior.

A computation is deterministic only to the extent guaranteed by its semantic contract and execution conditions.

17.1 Deterministic compilation

When deterministic compilation is promised, equivalent inputs, relevant configuration, dependency versions, toolchain identity, and declared environment must produce equivalent artifacts under the documented equivalence relation.

Where byte-for-byte reproducibility is promised, the relevant metadata and output ordering MUST also be controlled.

17.2 Runtime nondeterminism

Concurrency, distributed execution, random operations, quantum measurement, external services, and hardware behavior MAY introduce nondeterminism where their semantics permit it.

The language MUST NOT silently promise bitwise-identical results across targets for operations whose specified semantics do not guarantee them.

17.3 Reproducibility metadata

Reproducible builds and executions SHOULD record relevant source identities, compiler versions, dependency identities, dialect versions, configuration, policies, random seeds where applicable, and target information when it affects results.

17.4 Optimization correctness

An optimization MUST preserve the semantics that the program declares observable.

Where floating-point reassociation, approximation, probabilistic behavior, or nondeterministic scheduling is allowed, the corresponding permission MUST be explicit in the applicable semantic contract.

18. Concurrency, parallelism, and distributed computation

Zamani MUST support a coherent semantic model for concurrent, parallel, heterogeneous, and distributed computation.

The language MUST distinguish logical concurrency from the physical execution resources chosen by an implementation.

A task MAY be realized using a thread, actor, event loop, accelerator queue, distributed worker, or another compatible mechanism.

The source program SHOULD NOT need to name the physical execution unit unless that distinction is semantically relevant.

18.1 Concurrency correctness

The memory model, synchronization rules, ordering, communication, cancellation, and failure behavior MUST be defined.

A compiler MUST NOT introduce data races or violate required happens-before relationships through an invalid transformation.

18.2 Parallel execution

Parallel execution MUST preserve the program's required semantics.

Where reduction order, floating-point accumulation, or execution order affects results, the applicable specification MUST define the permitted behavior.

18.3 Distributed execution

Distributed nodes, channels, services, and participants MUST be represented through abstract identities and declared requirements.

A program MUST NOT assume a fixed universal node count, network size, or communication capacity.

18.4 Partial failure

Distributed and concurrent operations MUST define how failures, timeouts, cancellation, retries, duplicate messages, and partial completion affect program meaning where applicable.

Retries MUST NOT silently duplicate non-idempotent effects.

19. Classical computation

Classical computation MUST use the common language foundation for types, expressions, functions, memory, effects, resources, capabilities, contracts, and policies.

It encompasses ordinary computation, systems programming, scientific computation, numerical algorithms, symbolic algorithms, signal processing, linear algebra, tensors, optimization, and accelerator-based computation.

These categories are extensible.

19.1 Numerical portability

A numerical operation MUST have defined mathematical or machine-representation semantics.

Where exact equivalence across targets is impossible, the specification MUST state the permitted differences and any applicable precision, error, or reproducibility guarantees.

19.2 Data-parallel computation

Data-parallel and tensor operations SHOULD express shape, element type, transformation, and resource requirements independently of a fixed physical vector width or device count.

19.3 Classical IR

Classical lowering MUST use the repository's canonical classical representation and its defined contracts.

The language specification MUST NOT introduce a second universal classical IR merely to accommodate another backend.

20. Quantum computation

Quantum computation is a first-class computational domain of Zamani.

Quantum syntax and semantics MUST integrate with the shared language model, including types, effects, resources, capabilities, policies, contracts, provenance, diagnostics, and compatibility.

20.1 Generic quantum operations

The grammar MUST provide an extensible operation model rather than requiring a permanently enumerated grammar production for every quantum gate or vendor-specific operation.

A generic quantum operation MUST be capable of representing, as applicable:

- Operation identity or qualified operation name.
- Target operands.
- Control operands.
- Parameters.
- Results.
- Attributes.
- Modifiers.
- Relevant effects.
- Capability requirements.
- Resource requirements.
- Source provenance.

The operation registry and semantic contracts determine whether a referenced operation is known, supported, well-typed, and realizable.

20.2 Operation identity

A quantum operation MUST have a stable semantic identity independent of the physical instruction used by a particular QPU.

New operations MAY be registered through a versioned operation registry or dialect, provided that their semantic mapping, validation rules, compatibility, and realization requirements are defined.

An unknown operation MUST NOT be silently interpreted as an unrelated operation.

20.3 Canonical quantum IR

The existing "src/quantum/ir/" hierarchy is the canonical quantum semantic IR boundary.

Quantum source syntax MUST map through the AST and semantic analysis into that representation or a documented migration-compatible path into it.

A new quantum grammar MUST NOT create a competing canonical quantum IR.

The canonical quantum IR describes computational meaning. It MUST NOT own hardware discovery, physical-qubit assignment, routing algorithms, calibration, pulse synthesis, simulator execution, or device communication.

20.4 Logical and physical resources

Logical qubits and other source-level quantum resources MUST be distinguished from physical qubits and concrete device resources.

A source-level logical qubit MUST NOT implicitly identify a physical qubit.

Physical realization is the responsibility of the relevant compiler, routing, scheduling, runtime, and hardware integration layers.

20.5 Measurement and dynamic circuits

Measurement, reset, classical feed-forward, mid-circuit control, and dynamic circuits MUST define their effects, result types, ordering, and resource requirements.

Quantum nondeterminism MUST be represented according to the declared quantum semantics. It MUST NOT be silently replaced with an arbitrary deterministic result.

20.6 Quantum error correction

Quantum error-correction intent MUST be expressed through defined domain semantics and resource/capability requirements.

The source language MAY declare code families, fault-tolerance requirements, logical-resource requirements, or other supported QEC intent.

Physical code layout, syndrome acquisition, decoder selection, routing, and device-specific execution MUST remain downstream realization concerns.

20.7 Quantum scalability

The language and canonical quantum IR MUST NOT impose arbitrary fixed limits on qubit counts, operation counts, circuit depth, topology size, or gate arity.

Concrete realizations remain subject to target capabilities, available resources, physical constraints, and explicit policies.

21. Hybrid quantum-classical computation

Hybrid computation composes classical and quantum operations within one program model.

The hybrid domain MUST define the semantic boundary between classical values, quantum resources, measurement results, control flow, and cross-domain communication.

It MUST reuse the common type, effect, resource, capability, policy, and provenance models.

Hybrid syntax MUST NOT redefine the semantics of classical computation or quantum operations.

Where a classical decision depends on a quantum measurement, the program MUST represent that dependency explicitly according to the language's control-flow and measurement rules.

22. Hardware description and co-design

Hardware description and hardware/software co-design are first-class domains.

Their source constructs MAY describe:

- Modules and components.
- Ports and interfaces.
- Signals and nets.
- Registers and memories.
- Combinational and sequential logic.
- Clocking and reset behavior.
- State machines.
- Timing and pipeline intent.
- Parameterized widths and structures.
- Protocols and hardware assertions.
- Verification and synthesis intent.
- Software-visible hardware interfaces.

The detailed syntax belongs to the HDL and hardware grammar owners.

22.1 Parameterized hardware

Hardware widths, array extents, port counts, and structural parameters MUST be represented through the relevant type, shape, parameter, and semantic contracts.

The language MUST NOT impose arbitrary universal fixed widths or counts.

A particular hardware design MAY intentionally specify a finite width, clock frequency, number of ports, or structural parameter. Such a value is a program or design requirement, not a universal language maximum.

22.2 Synthesis and simulation

Simulation, synthesis, physical design, and deployment are distinct realization modes.

A source construct MUST define which modes it permits and which semantic guarantees apply.

The language MUST NOT imply that every hardware description can be synthesized on every target.

22.3 Hardware IR

Hardware semantics MUST map through a defined semantic boundary into the repository's governed HDL/hardware representation.

Vendor-specific synthesis instructions and physical implementation details MUST NOT become universal source-language semantics without an explicitly approved language extension.

23. Hardware capabilities and heterogeneous execution

A hardware target is a possible realization environment, not the definition of the program.

Targets MAY include CPUs, GPUs, FPGAs, ASICs, QPUs, simulators, specialized accelerators, embedded systems, and future computational substrates.

A program MUST express relevant requirements through capabilities, resources, constraints, and policies.

The implementation is responsible for determining whether an available target can satisfy them.

23.1 Capability negotiation

Capability negotiation MUST distinguish:

- Required capabilities.
- Optional capabilities.
- Preferred capabilities.
- Prohibited capabilities.
- Explicit fallback capabilities.

A target lacking a mandatory capability MUST NOT be treated as compatible merely because it can execute some related operation.

23.2 Heterogeneous programs

A program MAY be realized across multiple compatible targets when its semantics and execution model permit that arrangement.

The implementation MUST define the relevant data movement, synchronization, memory visibility, failure behavior, and security boundaries.

23.3 Hardware discovery

Hardware discovery and resource enumeration MUST remain outside the universal source grammar.

A discovered device is an environment fact. It MUST NOT alter the normative meaning of the source program.

24. Artificial intelligence and learning

AI, machine learning, reasoning, symbolic inference, probabilistic computation, neural-symbolic computation, and agent-based systems are domains of the same language.

They MUST reuse the shared type, effect, data, resource, capability, policy, contract, and provenance models.

Supported operations MAY include inference, deduction, induction, abduction, learning, querying, planning, uncertainty handling, and decision recording.

The feature inventory is extensible; this list does not create a closed set of language operations.

24.1 Evidence and provenance

Where a program uses evidence, confidence, uncertainty, or decision records, the semantic model SHOULD preserve the distinction between observations, assumptions, inferred results, and verified facts.

A confidence value MUST NOT automatically be treated as a proof of correctness.

24.2 Controlled adaptation

Adaptation or learning that changes model state, execution plans, policies, or generated artifacts MUST be governed by explicit effect, capability, resource, authorization, provenance, and policy contracts.

A program MUST NOT gain unrestricted permission to modify its own executable meaning merely because it declares an adaptation operation.

24.3 Application-specific concepts

Application concepts SHOULD be provided through libraries, modules, registered dialects, and capabilities where dedicated universal syntax is not justified.

The existence of an application domain MUST NOT automatically reserve its vocabulary as core language keywords.

25. Data computation

Data computation MUST use the common type, effect, resource, capability, and provenance models.

It MAY include:

- Collections and structured records.
- Tables and schemas.
- Graphs and datasets.
- Streams and pipelines.
- Queries and transformations.
- Serialization and deserialization.
- Persistent storage.
- Statistical and analytical operations.
- Data validation and lineage.

Data quantities and shapes MUST NOT be restricted by arbitrary universal grammar constants.

External data formats and query languages MUST be integrated through explicit interoperability or dialect contracts rather than silently becoming additional competing language authorities.

26. Networking and communication

Networking constructs MUST define their source-level meaning independently of a specific network device or deployment topology.

The domain MAY provide endpoints, messages, channels, requests, responses, services, streams, routing intent, and protocol abstractions.

Network requirements, security, serialization, effects, failure behavior, and resource constraints MUST use the common semantic model.

The grammar MUST NOT impose universal fixed limits on participants, channels, endpoints, message counts, or network size.

Concrete addresses and protocols MAY be specified when required, but their portability implications MUST be documented.

27. Security and cryptography

Security is part of the language's semantic and execution contracts.

Security-sensitive operations MUST integrate with effects, capabilities, policies, authorization, provenance, and error handling.

The language MAY provide constructs for identity, permissions, sandboxing, secure execution, signatures, hashes, cryptographic operations, and other governed security mechanisms.

27.1 Authorization

The presence of a syntactic construct MUST NOT imply that the caller is authorized to perform it.

Authorization MUST be evaluated by the applicable semantic, policy, or runtime mechanisms.

27.2 Secrets and sensitive data

Secrets and sensitive data MUST be handled according to defined type, access, storage, and policy contracts where such facilities are provided.

Diagnostics and provenance MUST NOT unintentionally disclose protected values.

27.3 Safe adaptation and reflection

Reflection, generated code, native calls, adaptation, and other sensitive operations MUST be subject to their declared effects, capabilities, policies, and security boundaries.

A sandbox declaration alone MUST NOT be treated as proof that the implementation provides effective isolation.

28. Interoperability and foreign interfaces

Zamani MAY interoperate with external languages, libraries, runtimes, data formats, and execution systems.

Interoperability MUST be explicit and versioned where external contracts affect program meaning.

The relevant integration must distinguish:

- Foreign source syntax.
- ABI and calling conventions.
- Data layout and representation.
- Linkage and symbol resolution.
- Foreign types and ownership.
- Error and exception translation.
- Serialization and data interchange.
- Runtime and deployment requirements.
- Security and capability requirements.

Parsing a foreign format MUST NOT make that format part of Zamani's universal source-language authority.

A foreign operation MUST have a defined semantic, effect, resource, error, and compatibility contract.

29. Dialects and extensibility

Zamani MUST support controlled extension without requiring the core grammar to enumerate every future operation or computational domain.

A dialect MUST identify, as applicable:

- Namespace and identity.
- Version.
- Owning provider.
- Syntax ownership.
- Semantic mapping.
- Type and effect integration.
- Capability and resource requirements.
- Compatibility contract.
- Dependencies.
- Registration and availability rules.
- Diagnostics and conformance tests.
- Security and provenance requirements.

29.1 No implicit syntax authority

A dialect MUST NOT silently override a core production or change existing language semantics.

Conflicting dialect definitions MUST be diagnosed according to explicit registration and compatibility rules.

29.2 Deterministic parsing

Parsing MUST NOT require uncontrolled network access, hidden environment discovery, or nondeterministic plugin loading.

Dialect metadata affecting parsing MUST be explicitly available and versioned.

29.3 Generic operations

Where the operation's identity can be represented as data or a qualified name, the grammar SHOULD use a generic operation form instead of enumerating every operation as a dedicated production.

The semantic registry determines operation meaning, validation, and supported realization.

30. Macros, reflection, and metaprogramming

Metaprogramming MAY provide syntax transformation, quotation, unquotation, compile-time computation, controlled reflection, code generation, and specialization.

These mechanisms MUST preserve source provenance and remain subject to the language's semantic and security rules.

30.1 Macro hygiene

Macro expansion MUST have defined name-resolution and hygiene behavior.

Expansion MUST NOT silently capture or redefine names outside the documented rules.

30.2 Semantic validation

Generated constructs MUST undergo the required structural and semantic validation.

A macro MUST NOT bypass type checking, effect checking, capability requirements, resource constraints, policy enforcement, or compatibility checks.

30.3 Compile-time execution

Compile-time operations MUST have explicit effect and resource boundaries.

A build MUST NOT silently depend on undeclared external state when reproducibility is required.

30.4 Controlled reflection

Reflection MUST obey the applicable capability, policy, and visibility rules.

The availability of metadata MUST NOT automatically grant permission to mutate protected program state or generate arbitrary executable behavior.

31. Temporal and multi-timeline computation

Temporal or multi-timeline constructs MAY be supported as a specialized computational domain.

Their semantics MUST define the relevant time model, ordering, synchronization, causality, state transitions, and interaction with ordinary computation.

The existence of temporal syntax MUST NOT implicitly guarantee access to a particular clock, scheduler, simulator, or physical time source.

Where a timeline is simulated or abstract, its meaning MUST remain distinct from the wall-clock behavior of the target.

32. Nano-oriented and physical-scale domains

Nano-oriented or physical-scale computational constructs MAY express models involving physical processes, small-scale structures, or related domain-specific computations.

Their semantics MUST distinguish:

- An abstract computational model.
- A mathematical or physical simulation.
- A hardware description.
- An actual physical execution or manufacturing process.

The presence of a source construct MUST NOT imply that the runtime can directly control a physical process or that a target has the required capability.

Physical constraints and resource requirements MUST be represented explicitly.

33. Canonical semantic representation

All supported source constructs MUST map to a validated semantic representation before target-specific realization.

The representation MUST preserve enough information to establish:

- Program meaning.
- Type and ownership correctness.
- Effects.
- Capability requirements.
- Resource requirements and constraints.
- Contracts and policies.
- Domain semantics.
- Provenance.
- Relevant determinism and reproducibility properties.
- Compatibility and version information.

A feature MUST NOT be considered fully integrated if it has syntax but no defined semantic mapping.

33.1 AST integration

The existing "src/ast/" tree is an integration point for parsed language constructs.

Changes to the AST MUST be coordinated with the parser, source spans, structural validation, semantic analysis, diagnostics, and downstream consumers.

Domain-specific AST extensions MUST NOT create duplicate representations for concepts already owned by the common AST unless a documented migration or façade contract requires them.

33.2 Semantic integration

The semantic analyzer MUST determine whether parsed constructs satisfy the language's rules.

It MUST distinguish invalid program meaning from unavailable target resources.

Semantic validation MUST NOT depend on an accidental backend choice where target-independent meaning is required.

33.3 Canonical IR integration

Each domain MUST identify its canonical IR owner and mapping contract.

Classical, quantum, and HDL/hardware representations MAY differ because their semantics differ. They MUST NOT become competing definitions of the same domain's meaning.

For quantum computation, "src/quantum/ir/" remains the canonical quantum semantic IR boundary.

33.4 Lowering

Lowering MUST preserve required semantic properties.

Target-specific lowering MAY specialize a program when specialization preserves the declared behavior and satisfies all applicable contracts.

If correct lowering is unavailable, the implementation MUST report that limitation rather than silently changing the computation.

34. Compilation intent

The source language MAY express compilation profiles, specialization conditions, optimization preferences, target-independent requirements, and explicitly target-dependent requirements.

Compilation intent MUST be distinguished from the compiler's internal strategy.

34.1 Target-independent compilation

The compiler SHOULD preserve target-independent semantic information until target-dependent decisions are necessary.

34.2 Specialization

Specialization MAY use known types, symbolic dimensions, capabilities, resource information, or declared constants.

Specialization MUST preserve the source program's semantics under the conditions for which it is valid.

34.3 Optimization

Optimization MUST preserve required observable behavior, effects, contracts, and domain-specific correctness properties.

Performance preferences MUST NOT override correctness or mandatory resource and security constraints.

34.4 Compilation artifacts

Portable artifacts and target-specific artifacts MUST be distinguishable.

An artifact MUST identify the versions and assumptions necessary for its correct interpretation.

35. Execution intent and target realization

The language MAY express execution intent, placement constraints, resource preferences, resilience requirements, fallback policies, and permitted adaptation.

The runtime and backend architecture determines how to realize that intent.

The source grammar MUST NOT be responsible for enumerating actual hardware inventory or selecting concrete physical resources.

35.1 Realization procedure

A conforming realization process SHOULD perform the following steps as applicable:

1. Load and validate the artifact.
2. Resolve modules, dependencies, and dialects.
3. Validate semantic and policy obligations.
4. Discover available target capabilities and resources.
5. Determine whether mandatory requirements can be satisfied.
6. Apply constraints and reject invalid realizations.
7. Rank permissible alternatives using preferences.
8. Select a valid realization.
9. Perform target-dependent lowering.
10. Place, route, and schedule the computation as required.
11. Execute under the applicable policy and runtime contract.
12. Observe results and report failures.
13. Recover or adapt only when explicitly permitted.
14. Release resources and finalize provenance.

This procedure is an architectural expectation; the detailed behavior belongs to the compilation and execution specifications.

35.2 Unavailable resources

If required resources or capabilities are unavailable, the implementation MUST report the reason and applicable failure category.

It MAY select an explicitly permitted alternative.

It MUST NOT silently relax a mandatory requirement.

35.3 Runtime adaptation

Runtime adaptation MUST preserve the declared semantic contract.

Changes that alter precision, approximation, security properties, effect behavior, or correctness guarantees MUST require explicit permission under the relevant specification and policy.

36. Fallback and graceful degradation

Fallback is permitted only when the program or its governing policy allows it.

A fallback MUST define:

- The condition that triggers it.
- The alternative realization or behavior.
- The semantic guarantees preserved.
- Any changed guarantees.
- The resource and capability requirements.
- The observable failure or degradation behavior.
- The provenance recorded for the decision.

A quantum implementation MAY use a permitted simulator if the program allows that realization.

A program MAY permit a classical alternative where a valid semantic mapping exists.

Neither substitution is automatically valid merely because the preferred target is unavailable.

The implementation MUST reject an unauthorized or semantically invalid fallback.

Approximation MUST NOT be introduced silently.

37. Resource feasibility and implementation limits

Resource feasibility is a property of a particular compilation or execution context.

The implementation MUST distinguish at least:

- Invalid source semantics.
- Unsupported language features.
- Missing required capabilities.
- Insufficient available resources.
- Policy denial.
- Unsupported lowering.
- Target incompatibility.
- Transient runtime failure.
- Representation or implementation limits.

These failures MUST NOT be collapsed into an ambiguous success state.

Where a resource requirement is statically provable, the implementation SHOULD check it before execution.

Where availability can change over time, the implementation MAY defer the check to deployment or runtime.

A successful feasibility check MUST NOT be represented as a permanent guarantee that a changing environment will remain available.

38. Compatibility and versioning

Zamani MUST version language semantics independently from incidental implementation details.

The compatibility system MUST account for:

- Language edition and semantic version.
- Lexical and grammar behavior.
- AST and semantic representation changes.
- Type and effect rules.
- Domain operation registries.
- Dialects and extensions.
- Canonical IR formats.
- Portable and target-specific artifacts.
- ABI and runtime interfaces.
- Diagnostics where tooling compatibility matters.
- Migration and deprecation behavior.

38.1 Breaking changes

A change is breaking when it invalidates previously conforming source, changes specified program meaning, breaks a stable artifact contract, or violates an established compatibility guarantee.

Breaking changes MUST follow the language's versioning and migration policy.

38.2 Deprecated behavior

Deprecated syntax or behavior MUST have a documented status and migration path where applicable.

A deprecated construct MUST NOT silently acquire unrelated semantics.

38.3 Forward compatibility

Implementations MAY preserve or reject unknown future features according to the applicable format and compatibility contract.

They MUST NOT silently reinterpret unknown semantics as known operations.

39. Safe implementation requirements

The Zamani compiler and associated production Rust implementation MUST use Rust 2021 and Rust 1.97 or later, consistent with the repository's declared toolchain requirements.

Zamani-owned production Rust code MUST NOT use Rust "unsafe" code.

This requirement applies to compiler, lexer, parser, AST processing, semantic analysis, IR processing, tooling, and runtime code owned by the project.

External dependencies MUST be evaluated according to the project's dependency and security policies.

The Rust implementation safety requirement does not by itself determine whether a Zamani source-language construct named "unsafe" is valid, deprecated, or rejected. That question MUST be answered by the normative source-language, semantic, and security specifications.

No source construct may bypass the language's safety, ownership, capability, policy, or validation rules merely because the implementation represents it with a dedicated AST variant.

40. Diagnostics and failure guarantees

Every implementation MUST provide meaningful diagnostics for failures within its supported conformance scope.

Diagnostics SHOULD distinguish:

- Lexical errors.
- Syntax errors.
- Name-resolution errors.
- Type errors.
- Ownership and lifetime errors.
- Effect violations.
- Contract violations.
- Capability failures.
- Resource infeasibility.
- Policy and authorization failures.
- Unsupported domain operations.
- Invalid or unavailable lowering.
- Target incompatibility.
- Runtime and recovery failures.
- Version and compatibility failures.

Errors MUST NOT be silently discarded where doing so could produce an incorrect program or misrepresent conformance.

A compiler MAY continue analysis after recoverable errors to report additional diagnostics, provided that error recovery does not cause invalid input to be treated as a successfully validated program.

41. Conformance and implementation status

The repository MUST distinguish language design from implementation evidence.

A feature's presence in a specification or grammar file does not establish that the feature is implemented.

Implementation status MUST be recorded in "grammar/grammar.md" and the applicable machine-readable contracts under "grammar/spec/".

The status vocabulary MUST be consistent across the repository. Where the existing registry defines statuses such as "STABLE", "IMPLEMENTED", "PARTIAL", "PROPOSED", "EXPERIMENTAL", "PLANNED", "DEPRECATED", "HISTORICAL", and "REJECTED", those meanings MUST be used consistently.

A feature MUST NOT be marked "STABLE" until its required implementation, tests, compatibility contract, and integration evidence are complete.

41.1 Required evidence

For each stable source construct, the implementation MUST provide applicable evidence for:

1. Normative syntax and semantic rules.
2. Lexical and parser conformance.
3. AST representation and source locations.
4. Name resolution and structural validation.
5. Type, ownership, and effect validation.
6. Capability and resource analysis.
7. Policy and contract validation where applicable.
8. Domain semantic mapping.
9. Canonical IR mapping where applicable.
10. Diagnostics and failure behavior.
11. Compatibility and versioning.
12. Positive and negative conformance tests.
13. Relevant portability and scalability tests.

Not every feature requires a unique runtime backend or a dedicated IR. The manifest MUST state which integration obligations apply and why.

41.2 Shared conformance corpus

The ANTLR grammar and existing Rust lexer/parser MUST be tested against the same applicable normative conformance corpus.

Where their current capabilities differ, the discrepancy MUST be recorded rather than hidden.

A parser passing its own isolated tests is not sufficient evidence that it conforms to the whole language specification.

42. Scalability, portability, and hard-coding audits

The repository MUST enforce a hard-coding audit as part of language-feature acceptance and freeze.

The audit MUST detect:

- Arbitrary machine-capacity constants.
- Fixed resource ceilings hidden in grammar or AST structures.
- Domain-specific leakage into universal syntax.
- Duplicated token or rule ownership.
- Unresolved grammar dependencies.
- Missing semantic or IR mappings.
- Inconsistent resource representations.
- Silent truncation or overflow.
- Unspecified target assumptions.
- Invalid fallback behavior.
- Missing version or compatibility metadata.
- Unbounded recursion or allocation assumptions that could cause implementation failure without diagnostics.

The audit MUST distinguish arbitrary language restrictions from justified finite program values, mathematical constraints, explicit budgets, physical design requirements, and documented implementation limits.

Every finding MUST be either resolved or recorded as an explicit, reviewed exception with an owner, scope, rationale, and compatibility implications.

43. Testing requirements

The language MUST be validated through a layered conformance strategy.

The test suite SHOULD include:

- Token and lexical tests.
- Parser and grammar tests.
- AST and source-span tests.
- Name-resolution and module tests.
- Type, ownership, and effect tests.
- Contract, policy, capability, and resource tests.
- Classical semantic tests.
- Quantum semantic and IR tests.
- HDL and hardware-intent tests.
- Hybrid-domain tests.
- AI and data tests.
- Concurrency and distributed tests.
- Networking and security tests.
- Dialect and interoperability tests.
- Determinism and reproducibility tests.
- Compatibility and migration tests.
- Positive and negative diagnostics tests.
- Fuzzing and malformed-input tests.
- Hard-coding and dependency audits.
- Portability and scalability tests.
- End-to-end compiler and runtime tests.

43.1 Portability tests

The same source-level semantic test SHOULD be exercised against multiple compatible realization profiles where available.

Tests MUST distinguish semantic equivalence from bitwise identity where the language permits numerical or operational differences.

43.2 Scalability tests

Scalability tests SHOULD vary symbolic dimensions, data sizes, resource availability, target capabilities, and execution configurations without rewriting the source solely to accommodate different capacities.

Tests MUST NOT claim infinite execution merely because no fixed grammar limit was found.

43.3 Failure tests

Tests MUST verify that missing resources, unsupported capabilities, invalid policies, and incompatible targets produce the required diagnostics or runtime outcomes.

A failed realization MUST NOT be reported as a successful execution.

44. Integration contract for every grammar file

Every grammar file and normative contract MUST declare enough information to permit independent completion and later integration without redesigning the file merely because another domain is added.

Each applicable file MUST identify:

- Canonical path and purpose.
- Normative or non-normative status.
- Authority and owning subsystem.
- Constructs and symbols it owns.
- Constructs and symbols it explicitly does not own.
- Inputs and outputs.
- Dependencies and import requirements.
- Exported rules, symbols, or schemas.
- Consumers and integration points.
- AST mapping.
- Semantic mapping.
- Type and effect integration.
- Capability and resource integration.
- Policy, contract, and provenance integration where applicable.
- Canonical IR mapping where applicable.
- Compiler and runtime integration where applicable.
- Diagnostic requirements.
- Test ownership and required conformance cases.
- Compatibility and versioning behavior.
- Portability and scalability obligations.
- Hard-coding audit requirements.
- Error handling and ambiguity rules.
- Completion and freeze criteria.

A field that genuinely does not apply MUST be explicitly marked "NOT_APPLICABLE" with a rationale. Omitting the field MUST NOT be interpreted as proof that the integration has been considered.

44.1 Stable interfaces

A file MUST depend on declared interfaces rather than undocumented implementation details of downstream consumers.

Downstream changes MUST NOT force a frozen file to change merely because a new backend, target, or domain has been added.

If a genuine language-wide semantic change requires modification, the file MUST be reopened through the formal change-control process. A freeze is controlled stability, not immunity to legitimate language evolution.

45. Existing repository integration map

This specification integrates with the current repository as follows.

Repository path| Required responsibility
"grammar/DESIGN.md"| Overall architecture, ownership, dependency, and freeze rules.
"grammar/README.md"| Navigation and architecture overview.
"grammar/Zamani.g4"| Canonical ANTLR grammar composition root.
"grammar/antlr/ZamaniLexer.g4"| Public ANTLR lexer composition boundary.
"grammar/antlr/ZamaniParser.g4"| Public ANTLR parser composition boundary.
"grammar/lexer/"| Token registry and modular lexical rules.
"grammar/core/"| Shared names, source units, attributes, requirements, and core constructs.
"grammar/types/"| Type syntax and type-family composition.
"grammar/expressions/"| Expression syntax and composition.
"grammar/statements/"| Statement syntax and composition.
"grammar/declarations/"| Declaration syntax.
"grammar/functions/"| Function syntax and composition.
"grammar/modules/"| Module and import syntax.
"grammar/memory/"| Memory and ownership-related syntax.
"grammar/effects/"| Effect syntax and contracts.
"grammar/resources/"| Resource requirements and constraints.
"grammar/policies/"| Policy syntax and integration.
"grammar/security/"| Security-related syntax and contracts.
"grammar/classical/"| Classical computational constructs.
"grammar/quantum/"| Quantum source constructs.
"grammar/hdl/"| Hardware-description constructs.
"grammar/hybrid/"| Cross-domain composition.
"grammar/hardware/"| Hardware intent and capability-related constructs.
"grammar/ai/"| AI and learning constructs.
"grammar/data/"| Data and tensor-related constructs.
"grammar/concurrency/"| Concurrency syntax and contracts.
"grammar/distributed/"| Distributed computation.
"grammar/networking/"| Networking constructs.
"grammar/compile/"| Compilation intent.
"grammar/execution/"| Execution intent and runtime-related syntax.
"grammar/interoperability/"| Foreign interfaces and external formats.
"grammar/dialects/"| Extension and dialect syntax.
"grammar/macros/"| Macro syntax and contracts.
"grammar/metaprogramming/"| Reflection and compile-time generation.
"grammar/compatibility/"| Grammar-level compatibility contracts.
"grammar/specification/"| Normative language specifications.
"grammar/spec/"| Machine-checkable contracts.
"grammar/validation/"| Grammar, ownership, dependency, portability, and hard-coding validation.
"grammar/tests/"| Grammar and integration tests.
"grammar/grammar.md"| Implementation-conformance reference and feature status.
"grammar/Zamani-Grammar.md"| Extended and historical design reference.
"src/lexer.rs"| Existing Rust lexer implementation.
"src/parser.rs"| Existing Rust parser implementation.
"src/ast/"| Existing AST implementation.
"src/semantic.rs"| Existing semantic-analysis integration point.
"src/quantum/ir/"| Canonical quantum semantic IR.

The table assigns architectural responsibilities. It does not assert that each path is complete, independently conforming, or already connected end to end.

The manifest and ownership contracts MUST record actual paths, actual symbols, and current implementation status. Where the repository's current structure differs from this intended model, the discrepancy MUST be tracked explicitly.

46. Change control

A language change MUST include a written proposal that identifies:

1. The problem being solved.
2. The intended semantic behavior.
3. The affected normative specifications.
4. The owning grammar files and exported symbols.
5. Token and keyword consequences.
6. AST and source-location consequences.
7. Type, effect, resource, capability, and policy consequences.
8. Canonical semantic and IR mappings.
9. Compiler and runtime integration requirements.
10. Diagnostics and failure behavior.
11. Compatibility and migration impact.
12. Portability and scalability implications.
13. Required positive, negative, and regression tests.
14. The hard-coding and ambiguity audit.
15. Completion and freeze criteria.

A change MUST NOT be accepted merely because its syntax can be parsed.

A feature MUST NOT be promoted to a stable language guarantee before its normative semantics and applicable integration contracts are complete.

47. Freeze criteria for this specification

"grammar/specification/language.md" may be frozen when all of the following conditions are satisfied:

- Its authority relative to the other normative specifications is explicit.
- Its terminology is consistent with the shared architecture.
- POCO-REAF is defined without claiming physically impossible guarantees.
- The scalability and hard-coding rules are explicit.
- Resource requirements are separated from physical realization.
- Common type, effect, policy, contract, and capability principles are established.
- Classical, quantum, hybrid, HDL, AI, data, distributed, networking, and future domains share the same language foundation.
- "src/quantum/ir/" remains the canonical quantum semantic IR boundary.
- Existing AST and Rust frontend integration paths are documented without falsely claiming full conformance.
- ANTLR and the hand-written Rust frontend have a shared conformance obligation.
- Compatibility, diagnostics, provenance, testing, and change control are defined.
- No contradiction with "grammar/DESIGN.md", "syntax.md", "semantics.md", "types.md", "domains.md", "portability.md", "poco-reaf.md", or the relevant machine contracts remains unresolved.
- The specification passes the normative-document and cross-reference audits.

If a related specification or implementation has not yet been brought into conformance, the discrepancy MUST be recorded as an outstanding integration task. The language-wide document MUST NOT conceal it by declaring the entire repository production-ready.

48. Final language invariant

The defining invariant of Zamani is:

«A Zamani program describes computational meaning and its required guarantees. The language does not impose arbitrary fixed hardware capacities. Compatible implementations determine how that meaning can be realized using the capabilities, resources, policies, and physical systems actually available.»

Consequently:

                         ZAMANI SOURCE
                               |
                               v
                       PORTABLE MEANING
                               |
             +-----------------+-----------------+
             |                 |                 |
             v                 v                 v
           TYPES             EFFECTS          CONTRACTS
             |                 |                 |
             +-----------------+-----------------+
                               |
             +-----------------+-----------------+
             |                 |                 |
             v                 v                 v
        CAPABILITIES        RESOURCES          POLICIES
             |                 |                 |
             +-----------------+-----------------+
                               |
                               v
                    CANONICAL SEMANTIC MODEL
                               |
             +-----------------+-----------------+
             |                 |                 |
             v                 v                 v
        CLASSICAL IR       quantum::ir       HDL/HARDWARE IR
             |                 |                 |
             +-----------------+-----------------+
                               |
                               v
                 OPTIMIZATION AND LOWERING
                               |
                               v
                   TARGET NEGOTIATION
                               |
                               v
                  ROUTING AND SCHEDULING
                               |
                               v
                    RESILIENCE / RECOVERY
                               |
                               v
                              ZQN
                               |
                               v
                              HAL
                               |
                               v
                   COMPATIBLE REALIZATION

This architecture MUST permit new targets and computational technologies to be integrated without redesigning the fundamental language merely because the physical execution environment has changed.

The language specification is complete only when its rules, its dependent specifications, its machine-checkable contracts, and its conformance evidence agree.