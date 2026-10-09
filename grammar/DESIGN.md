Zamani Grammar Architecture and Production Design

Canonical path: "grammar/DESIGN.md"
Repository: "Benwellonedge28/Zamani"
Language: Zamani
Rust edition: 2021
Minimum Rust version: Rust 1.97.1 or later
Rust safety: Safe Rust only; production Rust code MUST NOT use "unsafe"
Primary objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Document status: Normative architectural contract
Scope: Language authority, lexical and syntactic architecture, AST and semantic integration, canonical IR boundaries, domain composition, portability, scalability, compatibility, validation, testing, and freeze governance.

---

1. Purpose

This document defines the binding architectural requirements for the Zamani "grammar/" directory.

Its purpose is to make the directory a coherent, versioned, testable language-definition system that can be frozen before the next language-development stage begins.

The architecture MUST support a unified programming language for:

- Classical and systems programming.
- Embedded and resource-constrained computing.
- Scientific and numerical computing.
- Symbolic computation and formal reasoning.
- Quantum computation, quantum error correction, and quantum simulation.
- Hybrid quantum-classical computing.
- Hardware description and hardware/software co-design.
- AI, machine learning, neural-symbolic computing, and intelligent agents.
- Data, tensor, stream, and distributed-data computation.
- Concurrency, parallel computing, and high-performance computing.
- Networking, cryptography, and secure computation.
- CPUs, GPUs, FPGAs, ASICs, QPUs, simulators, and accelerators.
- Edge, cluster, cloud, and distributed environments.
- Temporal computation, metaprogramming, interoperability, and future computational domains.

These domains MUST share a common language foundation, including names, expressions, types, effects, contracts, capabilities, resources, policies, diagnostics, provenance, and compatibility rules.

A domain MAY define specialized syntax and semantics. It MUST NOT silently create an independent language, incompatible type system, competing universal parser, or ungoverned intermediate representation.

This document specifies architecture and required behavior. It does not claim that every planned feature, backend, target, or compiler stage is already implemented.

A feature MUST NOT be described as implemented, stable, or production-ready solely because a grammar production or design document exists.

2. Normative terminology

The words MUST, MUST NOT, REQUIRED, SHOULD, SHOULD NOT, and MAY are normative.

- Language specification: The normative definition of Zamani's source-level syntax and meaning.
- Grammar: A formal description of syntactically valid source constructs.
- Lexer: Converts source characters into tokens.
- Parser: Converts tokens into a syntactic structure.
- AST: The domain-neutral abstract syntax tree representing parsed source constructs.
- Semantic model: The validated, typed, target-independent meaning of a program.
- IR: An intermediate representation used by compiler stages.
- Canonical IR: A repository-governed representation with defined semantics, invariants, versioning, and integration contracts.
- Capability: A supported operation or property of an execution environment.
- Resource: A quantity or service required, consumed, reserved, or made available by computation.
- Requirement: A condition that must hold for a selected execution or realization to be valid.
- Constraint: A restriction on permitted realizations.
- Preference: A non-mandatory selection criterion.
- Hint: Optimization guidance that cannot change observable program meaning.
- Policy: Rules governing authorization, execution, resource use, fallback, adaptation, or other permitted behavior.
- Effect: A description of observable or semantically relevant consequences of an operation.
- Contract: A precondition, postcondition, invariant, assumption, guarantee, property, or other verifiable obligation.
- Provenance: Information identifying the origin, evidence, transformation, verification, or decision history of an artifact or semantic fact.
- Realization: Mapping validated program meaning onto a concrete execution environment.
- Dialect: A versioned, explicitly registered extension or external-format boundary.
- Conformance: Evidence that an implementation satisfies its applicable requirements.
- Frozen file: A file whose ownership, interfaces, dependencies, tests, compatibility obligations, and acceptance criteria are complete and approved.

3. Fundamental invariants

Every file, grammar rule, specification, and integration contract MUST respect the following invariants.

3.1 One language

Classical, quantum, HDL, AI, distributed, networking, and future computation are domains within Zamani.

They MUST share universal lexical rules, source locations, diagnostics, semantic foundations, and compatibility governance.

3.2 One architectural authority

"grammar/DESIGN.md" is the architectural authority for the grammar subsystem.

It defines ownership, boundaries, invariants, dependency rules, integration obligations, and freeze criteria.

Other documents MUST defer to this architecture. They MUST NOT establish contradictory ownership or silently redefine its rules.

3.3 One normative language specification

"grammar/specification/" owns the human-readable normative language specification.

"grammar/spec/" owns machine-oriented contracts, schemas, registries, and mechanically enforceable specification data.

The two MUST remain consistent. A contradiction MUST be resolved through an explicit, reviewed change.

Neither a grammar file, implementation comment, example, historical reference, nor generated document can silently override the normative specification.

3.4 One canonical lexical model

"grammar/lexer/" owns the documented lexical contracts and token registry.

"grammar/antlr/ZamaniLexer.g4" defines the public ANTLR lexer boundary.

"src/lexer.rs" is an executable Rust frontend implementation where applicable.

These representations MUST be reconciled through shared conformance tests. No representation may silently define a conflicting language.

3.5 One canonical parser boundary

"grammar/Zamani.g4" is the canonical ANTLR grammar composition root.

"grammar/antlr/ZamaniParser.g4" is the canonical public parser composition boundary.

Their responsibilities MUST be reconciled explicitly. There MUST NOT be two independently maintained, competing universal grammars.

The root MUST delegate domain-specific productions to their designated owners.

3.6 One domain-neutral AST

Source syntax MUST first map to the repository's canonical frontend AST.

During any migration between "src/ast/" and "src/frontend/ast/", the implementation MUST document the canonical owner, compatibility adapter, and removal criteria for the legacy path.

A grammar feature MUST NOT introduce an independent AST that bypasses the canonical frontend.

3.7 One semantic foundation

All computational domains MUST use shared semantic contracts for:

- Names and resolution.
- Types and values.
- Effects and effect composition.
- Capabilities and requirements.
- Resources and resource constraints.
- Contracts and validation.
- Policies and authorization boundaries.
- Provenance and evidence.
- Diagnostics and source spans.
- Language and representation compatibility.

Specialized domain semantics MAY extend these foundations without redefining them.

3.8 Canonical IR boundaries

Classical computation MUST map into the repository's canonical classical IR.

Quantum computation MUST map through "quantum::ir".

HDL and hardware computation MUST use the repository's canonical hardware/HDL semantic and IR contracts.

Additional domains MAY define canonical domain IRs where required. Each MUST specify its owner, invariants, version, and mappings to the common semantic model.

No domain may bypass semantic validation by lowering source syntax directly into a vendor-specific execution representation.

3.9 Safe Rust

All production Rust implementation associated with this architecture MUST use safe Rust.

The Rust baseline is 1.97.1 or later, using edition 2021 unless an explicitly approved repository-wide edition migration supersedes that requirement.

Production code MUST NOT contain "unsafe" blocks, "unsafe" functions, or other unsafe Rust constructs. New dependencies MUST be reviewed for suitability under this policy.

Compiler correctness, parser correctness, memory safety, and resource validation MUST NOT depend on undefined behavior or unchecked assumptions.

3.10 No artificial universal capacities

The language MUST NOT define finite hardware capacities as universal source-language limits.

This prohibition includes fixed ceilings on qubits, CPUs, cores, GPUs, FPGAs, ASICs, QPUs, devices, nodes, memory, threads, registers, register width, tensor rank, topology size, or network size.

Finite values MAY appear as program data, symbolic bounds, explicit application constraints, or target-specific requirements when justified by the program's semantics.

Such values MUST NOT become hidden ceilings on the language itself.

4. Scope and responsibility boundaries

The "grammar/" directory defines source-language architecture, grammar contracts, conformance expectations, and integration boundaries.

It does not independently implement:

- Complete type checking or semantic execution.
- Hardware discovery or physical allocation.
- Quantum routing, calibration, or execution.
- Quantum error-correction algorithms.
- Operating-system services.
- Backend code generation.
- Runtime scheduling or resource reservation.
- Network transport.
- AI model training or inference algorithms.
- Physical hardware synthesis.
- The complete ZQN or HAL implementation.

Those responsibilities belong to their designated repository subsystems.

The grammar architecture MUST define the interfaces those systems consume.

It MUST NOT pretend that documenting an interface implements its consumer.

5. Authority and document ownership

The following ownership model is binding.

File or directory| Authority and responsibility
"grammar/DESIGN.md"| Architecture, ownership, boundaries, invariants, freeze rules
"grammar/specification/"| Normative human-readable language definition
"grammar/spec/"| Machine-readable contracts, schemas, registries, and validation data
"grammar/Zamani.g4"| Canonical ANTLR composition root
"grammar/antlr/ZamaniLexer.g4"| Public ANTLR lexer boundary
"grammar/antlr/ZamaniParser.g4"| Public ANTLR parser composition boundary
"grammar/lexer/"| Token registry and lexical contracts
"src/lexer.rs"| Rust lexical implementation
"src/parser.rs"| Rust parser implementation
"src/ast/" and/or "src/frontend/ast/"| Canonical AST implementation, subject to explicit migration ownership
Semantic-analysis modules| Name resolution, typing, effects, capabilities, resources, contracts, policies, provenance
Classical IR implementation| Canonical classical intermediate representation
"quantum::ir" implementation| Canonical quantum intermediate representation
HDL/hardware semantic and IR modules| Hardware-intent validation and canonical hardware representation
Compiler and execution subsystems| Optimization, lowering, negotiation, routing, scheduling, recovery, and realization
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Historical and extended design reference, with explicit feature-status labels
"grammar/README.md"| Navigation and contributor guidance only

5.1 Authority conflict resolution

When two sources disagree:

1. Identify the conflicting requirements and affected features.
2. Consult this architectural contract.
3. Consult the normative language specification and machine contracts.
4. Determine the intended language-version behavior.
5. Update the responsible authority through a reviewed change.
6. Update dependent specifications, implementation mappings, diagnostics, and tests.
7. Record compatibility implications.
8. Regenerate or correct derived documentation.
9. Re-run the relevant conformance gates.

No conflict may be resolved solely by assuming that the newest file, largest file, or currently executable implementation is automatically correct.

5.2 Historical documentation

"grammar/Zamani-Grammar.md" MAY preserve historical, proposed, experimental, deprecated, rejected, or future concepts.

Each such concept MUST have a status that prevents it from being mistaken for normative language behavior.

"grammar/grammar.md" MUST accurately report the implementation's conformance state. It MUST NOT become a competing source of language law.

6. Canonical architecture and processing pipeline

The intended architecture is:

Zamani source
     |
     v
Language specification and version selection
     |
     v
Lexical analysis
     |
     v
Parsing and source-span preservation
     |
     v
Domain-neutral AST
     |
     v
Structural validation and name/module resolution
     |
     v
Semantic analysis
     |
     +--> Types and values
     +--> Effects
     +--> Capabilities
     +--> Resource requirements and constraints
     +--> Contracts and invariants
     +--> Policies and authorization
     +--> Provenance and evidence
     |
     v
Validated target-independent semantic model
     |
     +--------------------+---------------------+
     |                    |                     |
     v                    v                     v
 Classical IR          quantum::ir         HDL/HW semantics
     |                    |                     |
     +--------------------+---------------------+
                          |
                          v
               Target-independent optimization
                          |
                          v
                Domain-specific lowering
                          |
                          v
                Capability/resource negotiation
                          |
                          v
                    Placement/routing
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
            Concrete target realization and execution

This pipeline is an architectural contract, not proof that every stage exists or is production-ready.

Each stage MUST have a documented owner, input/output contract, error model, compatibility boundary, and implementation status.

6.1 Phase separation

The following distinctions are mandatory:

- Lexical validity is not syntactic validity.
- Syntactic validity is not semantic validity.
- Semantic validity is not proof that a program is resource-feasible.
- Resource feasibility is not proof that a particular backend can compile the program.
- Successful compilation is not proof that execution will succeed.
- A valid program requirement is not a guarantee that an appropriate target exists.

The implementation MUST report failures at the earliest appropriate phase and preserve sufficient context for meaningful diagnostics.

6.2 Source spans

Source spans MUST be preserved from lexical analysis through parsing and into AST nodes and diagnostics wherever applicable.

A source-span contract MUST define:

- File or source-unit identity.
- Start and end positions.
- Offset representation and indexing convention.
- Line and column conventions.
- Handling of Unicode source text.
- Synthetic or generated source locations.
- Macro-expansion and generated-code origin tracking.
- Behavior for malformed or incomplete source.

AST construction MUST NOT silently discard source locations needed for diagnostics, provenance, debugging, or source mapping.

7. Mandatory contract for every file

Every production grammar file and every normative or machine-readable contract file MUST declare the following information, either in a standard header or in a canonical machine-readable contract referenced by the file.

1. Purpose: The exact responsibility of the file.
2. Status: Its current conformance and stabilization state.
3. Authority: The specification or contract it implements.
4. Owns: Rules, symbols, schemas, or responsibilities uniquely owned here.
5. Does not own: Explicit exclusions and neighboring responsibilities.
6. Inputs: Consumed tokens, rules, structures, or contract data.
7. Outputs: Exported rules, structures, mappings, or generated artifacts.
8. Dependencies: Exact files, interfaces, and required versions.
9. Imports: Actual grammar or module imports, where applicable.
10. Exports: Public symbols and their stability.
11. Consumers: Files and subsystems that depend on the exports.
12. AST mapping: Canonical AST representation and construction requirements.
13. Semantic mapping: Required semantic interpretation and validation.
14. IR mapping: Canonical IR destination or a documented reason no IR mapping applies.
15. Effect mapping: Applicable effects and effect-checking rules.
16. Capability mapping: Required and optional capabilities, where applicable.
17. Resource mapping: Requirements, constraints, budgets, or resource effects, where applicable.
18. Contract and policy mapping: Relevant obligations, restrictions, and authorization.
19. Provenance mapping: Origin and transformation requirements.
20. Diagnostics: Error categories, source spans, and failure behavior.
21. Compatibility: Language, grammar, AST, semantic, IR, and dialect implications.
22. Scalability: Behavior as source structures and requested computations grow.
23. POCO-REAF: Target-independence and portability obligations.
24. Hard-coding audit: Evidence that universal capacities are not fixed.
25. Ambiguity and error handling: Required behavior for conflicting or invalid input.
26. Tests: Positive, negative, boundary, integration, and regression fixtures.
27. Freeze criteria: Objective conditions required for completion.

A file MUST NOT be declared complete merely because it compiles or parses one example.

7.1 No downstream-edit requirement

Before a file is frozen, all interfaces it consumes MUST be stable enough for its contract.

A downstream implementation change MUST NOT force a frozen file to change merely because the implementation was coupled to undocumented details.

A legitimate language change MAY require a reviewed update to a frozen file. Freezing does not prohibit evolution; it requires that evolution be explicit, versioned, tested, and compatibility-controlled.

8. Ownership and dependency contracts

Each public grammar symbol MUST have exactly one canonical owner.

For example, the rule "quantumOperation" MUST have one owner. A universal dispatcher may reference it but MUST NOT redefine it.

The ownership contract MUST identify:

- Symbol name.
- Canonical owner path.
- Symbol kind.
- Public or internal visibility.
- Dependencies.
- Exporting dispatcher.
- AST mapping.
- Semantic owner.
- IR destination, where applicable.
- Test suite.
- Compatibility status.

8.1 Dependency direction

Dependencies MUST follow a directed acyclic graph wherever possible.

The intended broad direction is:

lexical contracts
       |
       v
core names and source structure
       |
       v
types and memory contracts
       |
       v
expressions and declarations
       |
       v
statements and functions
       |
       v
effects, resources, capabilities, contracts and policies
       |
       v
computational domains
       |
       v
interoperability, dialects and controlled metaprogramming
       |
       v
universal composition and conformance

This is a conceptual ordering, not permission to create cyclic grammar imports.

Shared abstractions MUST be placed in an appropriate lower-level owner rather than creating circular dependencies between domains.

Any necessary semantic dependency cycle MUST be resolved through stable interfaces, deferred references, or semantic-analysis ordering—not an accidental grammar import cycle.

8.2 Composition versus ownership

A dispatcher MAY import or reference a subordinate rule. It MUST NOT become the second owner of that rule.

The root composition layer MUST NOT accumulate domain-specific production implementations.

9. Canonical lexical architecture

The lexical system MUST define one consistent token identity and classification model.

9.1 Token registry

"grammar/lexer/tokens.g4" and its associated machine-readable registry MUST establish canonical token identities and their ownership.

Each token registry entry MUST identify:

- Canonical token name.
- Spelling or lexical pattern.
- Token category.
- Owner.
- Reserved or contextual status.
- Applicable language versions.
- Dialect restrictions, if any.
- Deprecation or compatibility status.
- Tests.
- Downstream token or AST mappings where relevant.

The token registry MUST be mechanically checked against emitted ANTLR tokens and the Rust lexical implementation.

9.2 Lexical component files

The lexical hierarchy MAY partition its responsibilities among:

- "lexer/keywords.g4"
- "lexer/operators.g4"
- "lexer/punctuation.g4"
- "lexer/identifiers.g4"
- "lexer/literals.g4"
- "lexer/numeric-literals.g4"
- "lexer/string-literals.g4"
- "lexer/character-literals.g4"
- "lexer/boolean-literals.g4"
- "lexer/quantum-literals.g4"
- "lexer/hardware-literals.g4"
- "lexer/duration-literals.g4"
- "lexer/size-literals.g4"
- "lexer/annotations.g4"
- "lexer/comments.g4"
- "lexer/lexer-errors.g4"

These files MUST have clearly defined roles in the actual build graph.

A file that is only a design partition MUST be marked as such. It MUST NOT be mistaken for a production grammar merely because it has a ".g4" extension.

9.3 Duplicate token identities

Different token names MUST NOT represent the same lexical meaning without an explicit, documented distinction.

For example, duplicate identities for question marks, arrows, bitwise operators, or punctuation MUST be reconciled.

Compatibility aliases MAY exist inside a migration adapter, but the canonical emitted token stream MUST have one stable identity per lexical token category.

9.4 Keywords

Core keywords MUST be reserved only when required by the normative language specification.

Contextual keywords MUST have explicitly defined recognition and compatibility rules.

Adding a new keyword MUST include an identifier-compatibility analysis. A new feature SHOULD prefer a contextual or qualified form when unconditional reservation would cause unnecessary source breakage.

Application-specific vocabulary MUST NOT be promoted to universal keywords merely because one library or computational domain uses it.

9.5 Literals

Literal syntax MUST be defined independently of a particular machine's storage capacity.

The language MAY support integer, floating-point, decimal, rational, complex, Boolean, character, string, byte, duration, size, probability, confidence, symbolic quantum, and resource-related values where their semantics are specified.

Literal parsing MUST define overflow, precision, exponent, escape, encoding, and invalid-literal behavior.

A parser MUST NOT silently truncate an out-of-range value to fit a machine type.

9.6 Lexical determinism

Lexical classification MUST depend only on source characters, the selected language version, and explicitly selected lexical configuration.

It MUST NOT depend on target hardware, currently available resources, network state, filesystem state, wall-clock time, or runtime scheduling.

9.7 Lexer/parser agreement

The ANTLR and Rust frontend implementations MUST share a conformance corpus.

Any intentional differences in accepted syntax MUST be explicitly specified by language version or frontend mode.

The project MUST NOT declare lexical production readiness while these implementations silently disagree on canonical tokens, comments, literals, or source positions.

10. Core language and composition

The "core/" directory owns domain-neutral language foundations, including source units, program structure, names, qualified names, paths, visibility, versions, attributes, generic requirements, constraints, capability references, policy references, and optimization hints where present.

The root grammar MUST remain small.

It MAY own or delegate universal program entry, source-unit composition, and top-level dispatch. It MUST NOT duplicate:

- Token definitions.
- Type-system implementations.
- Expression implementations.
- Quantum operations.
- HDL implementation rules.
- AI-specific implementations.
- Resource semantics.
- Hardware discovery or realization.
- IR construction.

The composition contract MUST state exactly which root owns each public entry rule and how ANTLR imports are resolved in the actual build environment.

10.1 Hints versus meaning

An optimization hint MAY influence code generation or target selection.

A hint MUST NOT change the program's required observable behavior.

A requirement, constraint, or policy MUST NOT be treated as an optional optimization hint.

11. Types, values, and memory

The type system MUST describe program meaning rather than a fixed target layout.

It SHOULD provide a coherent model for the supported forms of:

- Primitive and named types.
- User-defined records and enumerations.
- Generic and constrained types.
- Functions, tuples, arrays, slices, maps, and collections.
- Unions, sums, options, and results.
- References, pointers, ownership, borrowing, and lifetimes.
- Linear and affine values.
- Traits, interfaces, associated types, and polymorphism.
- Resource-aware and domain-specific types.
- Quantum, hardware, temporal, and other specialized types where defined.

The grammar MUST NOT independently invent incompatible type constructors for classical, quantum, and hardware computation.

11.1 Type meaning versus physical representation

A source type MUST NOT implicitly promise a specific register width, memory location, device architecture, or physical allocation unless the language construct explicitly defines such a requirement.

For example, a parameterized vector type describes a logical collection. Its length is not a universal CPU register width.

A quantum register type describes a logical quantum resource. It does not allocate physical qubits or establish a device topology.

11.2 Generic and advanced type features

Each advanced type feature MUST define its syntax, type-checking rules, substitution behavior, diagnostics, serialization behavior where applicable, and IR mapping.

Features such as dependent types or higher-kinded types MUST remain marked as proposed, experimental, or planned until their complete semantic and implementation contracts are satisfied.

11.3 Memory model

The memory subsystem MUST separate source-level ownership, aliasing, lifetime, address-space, and persistence semantics from physical memory capacity.

It MUST document how host memory, accelerator memory, shared memory, distributed memory, persistent storage, and domain-specific resources interact with the common type and effect systems.

12. Expressions, declarations, statements, and functions

These subsystems MUST build on the canonical core and type system.

12.1 Expressions

"expressions/expressions.g4" SHOULD be the canonical expression composition point.

Every expression family MUST have one canonical owner, including literals, names, paths, unary and binary operations, calls, member access, indexing, ranges, tuples, arrays, lambdas, closures, blocks, conditionals, matching, comprehensions, queries, and domain-specific expressions.

Precedence and associativity MUST be specified explicitly.

Ambiguous duplicate families such as singular/plural closure or lambda files MUST be reconciled. They may remain separate only when their distinct responsibilities are documented and tested.

12.2 Statements

"statements/statements.g4" SHOULD be the canonical statement dispatcher.

Control flow, declarations, assertions, contracts, policies, resource requirements, simulation, reasoning, learning, adaptation, and domain-specific statements MUST be integrated through stable shared contracts.

A feature MUST NOT create a second control-flow or expression model merely because it belongs to a different domain.

12.3 Functions

The function architecture MUST define parameters, return types, generic constraints, calling conventions, closures, asynchronous behavior, compile-time functions, foreign functions, and contracts where supported.

Functions MUST share the common type, effect, resource, capability, and policy systems.

Domain-specific function execution MAY have additional rules, but those rules MUST be mapped into the common semantic model.

12.4 Controlled adaptation and reflection

Learning, reasoning, reflection, code generation, and adaptation MUST have explicitly defined semantics and effects.

An adaptation feature MUST NOT grant unrestricted authority to rewrite executable code or bypass type checking and security validation.

Where an operation changes code, policy, or execution behavior, its contract MUST specify required authorization, capabilities, effects, resource bounds or budgets where applicable, provenance, validation, and failure behavior.

13. Effects, capabilities, resources, contracts, and policies

These are shared semantic foundations, not unrelated per-domain features.

13.1 Effects

"effects/" MUST define effect identities, parameters, composition, subeffect relationships, effect polymorphism, diagnostics, and integration with capabilities, policies, and provenance.

Potential effect families include I/O, networking, mutation, randomness, native operations, foreign calls, distributed execution, quantum measurement, learning, adaptation, reflection, code generation, simulation, security-sensitive operations, and hardware access.

The registry MUST be extensible without requiring the root grammar to enumerate every future effect.

13.2 Capabilities

Capabilities describe supported behavior rather than physical inventory.

Examples include:

requires capability("quantum.measurement")
requires capability("gpu.compute")
requires capability("tensor.compute")

Capability identifiers MUST have defined naming, versioning, compatibility, discovery, and diagnostic rules.

A capability declaration MUST NOT itself perform hardware discovery or authorize an operation.

13.3 Resources

Resource syntax MUST support symbolic and parameterized requirements, including forms equivalent to:

requires qubits >= n
requires memory >= required_memory
requires topology(...)

The exact accepted surface syntax MUST be defined in the normative specification and tested against the actual parser.

The resource model MUST distinguish:

- Required minimums.
- Maximum budgets.
- Available capacity.
- Consumable resources.
- Reserved resources.
- Resource preferences.
- Resource constraints.
- Resource-derived effects.
- Resource feasibility and failure.

Resource expressions MUST be evaluated according to explicit semantic rules. Unknown values MUST NOT be silently treated as satisfied requirements.

13.4 Requirements, constraints, preferences, and hints

These concepts MUST remain distinct:

- A requirement must be satisfied.
- A constraint rules out disallowed realizations.
- A preference ranks otherwise acceptable choices.
- A hint guides optimization without changing meaning.
- An allowance explicitly permits a behavior.
- A prohibition explicitly disallows a behavior.

The implementation MUST NOT convert a hard requirement into a preference to make compilation succeed.

13.5 Contracts

The contract model MUST define preconditions, postconditions, invariants, assumptions, guarantees, assertions, and properties where supported.

Each contract category MUST specify its validation phase, failure behavior, diagnostic mapping, and relationship to runtime checks or formal verification.

13.6 Policies

"policies/" owns the common policy syntax and policy composition model.

Policies MUST integrate with security, effects, capabilities, resources, execution, adaptation, fallback, and provenance.

A policy declaration is not a substitute for enforcing that policy. Enforcement belongs to the designated semantic, compiler, or runtime component.

13.7 Provenance and evidence

Provenance MUST identify the origin and transformation history of relevant semantic facts, generated code, decisions, and artifacts.

Evidence and explainability constructs MUST specify what is recorded, how it is linked to the source, and what verification guarantees are actually available.

The presence of an evidence syntax node MUST NOT be presented as proof that an assertion is true.

14. Classical and data computation

"classical/" and "data/" MUST reuse the shared language foundation.

Classical computation MAY include numeric types, vectors, matrices, tensors, linear algebra, statistics, signal processing, symbolic operations, optimization, and scientific computation.

Data computation MAY include collections, records, schemas, tables, graphs, datasets, streams, pipelines, queries, serialization, deserialization, transformations, persistence, and provenance.

The grammar MUST distinguish logical data shape from the physical capacity of a device.

Parameterized dimensions and ranks MUST not introduce a universal maximum.

External query and serialization formats SHOULD be supported through explicit dialect or interoperability boundaries where appropriate. They MUST NOT automatically become part of the universal core language.

15. Quantum computation

The quantum subsystem MUST be integrated into the common language and semantic model.

It MUST support the architecture needed for logical quantum operations, operation parameters, targets, controls, states, measurement, reset, observables, channels, dynamic circuits, conditional execution, feed-forward, modifiers, logical resources, error correction, noise models, pulses, provenance, uncertainty, and dialects where implemented.

15.1 Generic operation model

The grammar MUST NOT depend on a permanently enumerated list of physical gate names.

A generic operation model SHOULD represent:

- Operation identity or specifier.
- Target operands.
- Optional control operands.
- Optional parameters.
- Results.
- Attributes and modifiers.
- Effects.
- Capability requirements.
- Resource requirements.
- Source provenance.

Built-in, user-defined, parameterized, vendor-specific, and future operations MUST be representable through an explicitly governed operation registry or dialect mechanism.

Adding a new operation MUST NOT require redesigning the universal grammar when its generic syntax and semantic contract already suffice.

15.2 Quantum semantic boundary

Quantum syntax MUST map through the domain-neutral AST and validated semantic model to "quantum::ir".

The grammar MUST NOT directly assign physical qubit identifiers, select a QPU, inspect calibration, perform routing, schedule pulses, or execute error correction.

Those responsibilities belong to downstream quantum compilation and execution systems.

15.3 Quantum resource semantics

Logical qubit requirements MUST be separate from physical qubit allocation.

The language MAY express requirements for qubits, measurements, topology, fidelity, connectivity, error correction, or supported operations where the semantic model defines them.

A requirement MUST be validated against the selected realization. An unavailable requirement MUST cause a meaningful failure unless a semantically valid, explicitly permitted alternative exists.

15.4 Error correction and resilience

The grammar MAY express error-correction intent, code families, logical resources, distance requirements, syndrome-related constructs, decoder requirements, and resilience contracts where specified.

The grammar MUST NOT imply that expressing a fault-tolerance requirement automatically implements it.

QEC behavior MUST map to its designated compiler, IR, runtime, and hardware contracts.

16. HDL and hardware/software co-design

The HDL subsystem MUST be a domain of Zamani rather than a competing universal language.

Its syntax and semantics MAY cover hardware modules, ports, nets, signals, registers, memories, combinational and sequential logic, clocks, resets, timing, pipelines, interfaces, protocols, state machines, assertions, verification, synthesis intent, and hardware/software co-design.

The grammar MUST express logical hardware structure and constraints without imposing universal bus widths, register counts, clock counts, pipeline depths, or device capacities.

Widths, timing parameters, and resource quantities MAY be program parameters or explicit target requirements.

HDL constructs MUST have defined validation and lowering paths into the canonical hardware/HDL representation.

Hardware synthesis, placement, timing closure, and physical realization MUST remain downstream responsibilities.

17. Hybrid computation

"hybrid/" MUST compose already-defined classical, quantum, and hardware semantics.

It owns the contracts for interactions such as:

- Classical control of quantum operations.
- Measurement results used by classical computation.
- Quantum-classical data conversion.
- Host-device communication.
- Synchronization.
- Shared data and resource requirements.
- Accelerator interoperability.
- Hybrid scheduling and execution intent.

It MUST NOT redefine classical types, quantum operation semantics, or hardware realization.

Cross-domain interactions MUST specify types, effects, capabilities, resource requirements, synchronization semantics, and failure behavior.

18. AI, reasoning, and adaptive computation

"ai/" MUST build on the shared type, effect, resource, capability, contract, policy, concurrency, and provenance models.

Supported AI features MAY include inference, learning, deduction, induction, abduction, knowledge, reasoning, evidence, uncertainty, probability, confidence, training, planning, decisions, neural-symbolic computation, agents, and adaptation.

Each feature MUST have a defined syntax-to-AST mapping, semantic contract, diagnostics, and downstream representation.

AI syntax MUST NOT create an alternative universal type system, resource system, or authorization model.

Application concepts such as vision, sentiment analysis, robotics, payments, legal workflows, and immersive environments SHOULD be provided by libraries, registered dialects, capabilities, policies, or application modules rather than hard-coded into the universal grammar.

19. Concurrency, distribution, and networking

Concurrency MUST provide a coherent computational model that can be realized through tasks, futures, actors, threads, pipelines, data parallelism, distributed processes, or accelerator work.

The source-level meaning MUST NOT unnecessarily depend on a particular physical execution unit.

"distributed/" MUST define the syntax and semantics of distributed tasks, messages, channels, collectives, placement intent, partitioning, replication, consistency, transactions, remote execution, and fault tolerance where implemented.

"networking/" MUST define network-facing constructs such as endpoints, channels, messages, requests, responses, protocols, streams, routing intent, service discovery, and security-related requirements where supported.

Neither directory may hard-code a universal count of nodes, actors, processes, messages, channels, or endpoints.

Network access and distributed execution MUST participate in the common effect, capability, policy, and provenance systems.

20. Hardware, execution, and compilation

20.1 Hardware

"hardware/" defines target-independent hardware descriptions, capabilities, resource categories, topology requirements, memory characteristics, timing and power constraints, reliability intent, and other declared properties where supported.

It MUST NOT make the source language depend on a particular CPU, GPU, FPGA, ASIC, or QPU.

20.2 Execution

"execution/" defines execution intent and the contracts used by downstream systems for contexts, placement, scheduling intent, adaptive execution, simulation, checkpointing, recovery, lifecycle, observability, and resilience where implemented.

Physical discovery, allocation, routing, scheduling, and runtime recovery MUST be owned by their designated implementation layers.

20.3 Compilation

"compile/" defines compilation-related source contracts, including target intent, cross-compilation, specialization, conditional compilation, feature selection, reproducibility, artifacts, deployment metadata, and compilation provenance where supported.

The source program MUST NOT require algorithmic rewrites merely because the compiler selects a different compatible target.

20.4 Portable artifacts

The compilation architecture MUST define a versioned artifact contract containing the information required to preserve program meaning across supported compilation and execution environments.

Depending on the artifact stage, this includes:

- Source or semantic identity.
- Language and feature versions.
- AST, semantic-model, and IR versions as applicable.
- Dialect and extension versions.
- Capability requirements.
- Resource requirements and constraints.
- Effects, contracts, and policies.
- Permitted fallback behavior.
- Provenance and reproducibility metadata.

The exact fields MUST be specified by the artifact contract and validated by tests.

21. POCO-REAF: portability, scaling, and feasibility

POCO-REAF is a language and toolchain design objective, not a promise of unlimited physical resources.

21.1 Program once

A program expresses its algorithm and computational meaning independently of a particular device where the language semantics permit it.

21.2 Compile once

The architecture MUST define what is meant by compiling once.

A portable semantic artifact or target-independent IR can be compiled once and subsequently lowered or specialized for compatible targets.

A target-specific executable is not automatically portable to incompatible instruction sets, hardware architectures, runtimes, or execution environments.

The language specification MUST distinguish portable artifacts from target-specific artifacts.

21.3 Run everywhere

The compiler and runtime MUST evaluate requirements and negotiate capabilities before selecting a realization.

A realization process SHOULD follow this sequence:

discover available capabilities
           |
           v
evaluate hard requirements
           |
           v
apply constraints and policies
           |
           v
rank permitted preferences
           |
           v
select a compatible realization
           |
           v
lower, route, and schedule
           |
           v
execute with defined failure behavior

A program MUST NOT silently execute with missing required capabilities.

21.4 Anywhere

The language MUST allow implementations to support multiple compatible execution environments.

A program MAY declare permitted fallbacks, alternative algorithms, simulator use, or other strategies where the specification defines their semantics.

A fallback MUST preserve the required observable behavior or be explicitly authorized as an approximation with defined error bounds and result semantics.

An implementation MUST NOT silently substitute an approximate algorithm, classical computation, or simulation for a required quantum operation when the substitution changes the program's specified meaning.

21.5 Forever

Long-term compatibility requires versioned language semantics, grammar behavior, AST contracts, semantic models, IRs, dialects, and artifacts.

The architecture MUST define backward compatibility, forward-compatibility expectations, deprecation, migration, and reproducibility policies.

No document may promise that an arbitrary future implementation will execute every historical binary unchanged forever.

21.6 Scaling from tiny to large

The language MUST permit computational structures to grow according to program parameters and available resources rather than arbitrary universal grammar ceilings.

This includes scaling across:

- Tiny embedded systems.
- Single-core and multicore CPUs.
- GPUs and other accelerators.
- FPGAs and ASICs.
- Quantum devices and simulators.
- Large-memory and high-performance computing systems.
- Clusters and distributed environments.
- Future compatible targets.

The same source-level semantics MUST be preserved wherever a compatible realization exists.

The language does not promise that every algorithm scales efficiently, that every target supports every domain, or that infinite computation is physically achievable.

21.7 No universal capacity constants

The architecture MUST reject universal ceilings such as fixed maximum qubit counts, processor counts, memory capacities, device counts, tensor ranks, thread counts, or topology sizes.

The validation system MUST check source files, machine-readable contracts, grammar rules, generated metadata, and relevant implementation constants for violations or disguised equivalents.

Legitimate limits imposed by an actual target, protocol, external format, runtime, or algorithm MAY exist, but they MUST be scoped, documented, and validated at the appropriate layer.

22. Explicit fallback and failure semantics

Every fallback-capable construct MUST specify:

- Which original requirement cannot be met.
- Which alternatives are permitted.
- Whether the alternative preserves exact semantics.
- Whether approximation is allowed.
- Required error bounds, if applicable.
- Required capabilities and resources for each alternative.
- The policy authorizing the fallback.
- Provenance and audit requirements.
- Behavior when no permitted alternative is feasible.

The implementation MUST distinguish at least these outcomes conceptually:

- Accepted.
- Accepted with an explicitly permitted degraded realization.
- Retry or replan.
- Recover.
- Escalate for external intervention.
- Rejected.

These outcomes MUST be mapped to the repository's actual diagnostic, compiler, or runtime result types rather than creating duplicate competing status enums.

23. Dialects, interoperability, macros, and metaprogramming

23.1 Dialects

"dialects/" owns extension registration and compatibility contracts.

A dialect MUST declare its identity, namespace, version, dependencies, syntax boundary, semantic mapping, required capabilities, compatibility, and status.

New dialects MUST NOT silently modify the meaning of existing core syntax.

23.2 Interoperability

"interoperability/" owns foreign source formats, ABI and API boundaries, FFI, linkage, calling conventions, data-layout agreements, foreign types, serialization, and external interfaces where implemented.

Parsing an external format MUST NOT automatically make that format part of Zamani's universal syntax.

Foreign calls MUST participate in the common effect, capability, security, and provenance models.

23.3 Macros and metaprogramming

"macros/" and "metaprogramming/" MUST define expansion, hygiene, syntax-tree generation, quotation, unquotation, reflection, introspection, compile-time execution, and specialization where supported.

Generated syntax MUST pass the required parsing, structural, type, semantic, effect, capability, resource, contract, and policy checks.

Macros MUST NOT bypass semantic validation.

Reflection and code generation MUST be governed by the applicable effects, permissions, sandboxing, and provenance contracts.

24. Compatibility and versioning

"compatibility/" MUST define the relationships between:

- Language version.
- Specification version.
- Lexical and grammar version.
- Parser/frontend version.
- AST version.
- Semantic-model version.
- Classical IR version.
- Quantum IR version.
- HDL/hardware IR version.
- Dialect and extension versions.
- Artifact and backend compatibility.

Every change to a public token, grammar rule, AST mapping, semantic rule, or IR contract MUST have an explicit compatibility assessment.

The compatibility process MUST distinguish:

- Additive compatible changes.
- Changes requiring a new language feature gate.
- Deprecations.
- Breaking changes requiring a version transition.
- Historical syntax accepted only in compatibility modes.
- Unsupported or rejected constructs.

Stable source syntax MUST NOT be reinterpreted silently in a way that changes observable program meaning.

25. Machine-readable contracts and registries

The architecture SHOULD use canonical machine-readable registries for file ownership, dependencies, feature status, exports, AST mappings, semantic mappings, IR mappings, diagnostics, and compatibility.

The exact authoritative paths MUST be established in the repository before freeze.

Where present, the following names are recommended for the canonical contract set:

grammar/MANIFEST.yaml
grammar/OWNERSHIP.yaml
grammar/DEPENDENCIES.yaml
grammar/STATUS.yaml

grammar/contracts/
    ownership.yaml
    dependencies.yaml
    exports.yaml
    ast-mappings.yaml
    semantic-mappings.yaml
    ir-mappings.yaml
    diagnostics.yaml
    compatibility.yaml
    feature-status.yaml

If equivalent registries already exist, they MUST be reconciled rather than duplicated under new names.

Each registry MUST define its schema, version, owner, validation rules, and generated outputs.

A registry MUST NOT become a second contradictory source of truth for information already owned elsewhere.

25.1 Manifest completeness

The canonical manifest MUST enumerate every relevant grammar, specification, registry, test fixture, generator input, and generated artifact.

For each file, it SHOULD record:

- Path and kind.
- Owner.
- Status.
- Authority.
- Imports and dependencies.
- Exports.
- AST, semantic, and IR mappings.
- Diagnostic contract.
- Compatibility contract.
- Tests.
- Generated or authoritative status.
- Freeze state.

Missing entries and unresolved ownership MUST be reported as validation failures.

26. Validation and architectural enforcement

"validation/" MUST act as an enforceable boundary for grammar integrity and cross-layer consistency.

It MUST detect, where applicable:

1. Duplicate token identities.
2. Duplicate ownership of exported rules.
3. Missing or unresolved imports.
4. Circular grammar dependencies.
5. Unreachable or unreferenced rules.
6. Accidental left recursion.
7. Unresolved parser ambiguity.
8. Operator precedence conflicts.
9. Malformed or inconsistent file paths.
10. Duplicate-looking files without documented distinctions.
11. Orphaned grammars.
12. Missing AST mappings.
13. Missing semantic mappings.
14. Missing IR mappings where required.
15. Missing diagnostics.
16. Missing conformance tests.
17. Undocumented emitted tokens.
18. Universal capacity ceilings.
19. Domain-specific implementation leakage into the universal grammar.
20. Contradictions between normative and machine-readable specifications.
21. Inconsistent status declarations.
22. Unapproved compatibility changes.
23. Missing source-span behavior.
24. Uncontrolled dialect or feature registration.
25. Unsafe Rust in production code governed by this architecture.

26.1 Semantic-boundary validation

The architecture MUST preserve the following separation:

Concern| Grammar| AST| Semantic analysis| Compiler/runtime
Token and syntax rules| Owns| Receives structure| Consumes| Consumes artifacts
Type meaning| Expresses syntax| Represents type syntax| Validates meaning| Uses validated types
Resource requirement| Expresses declaration| Represents requirement| Validates and resolves| Negotiates realization
Capability requirement| Expresses reference| Represents requirement| Checks semantic contract| Discovers available support
Physical allocation| Does not own| Does not allocate| May validate intent| Allocates or rejects
Quantum routing| Does not own| Does not route| Validates operation constraints| Routes operations
Scheduling| Does not own| Does not schedule| Validates constraints| Schedules execution
Calibration| Does not own| Does not calibrate| Validates declared constraints| Uses target calibration
Authorization| Expresses applicable constructs| Represents declarations| Enforces semantic checks| Applies runtime policy
Provenance| Expresses relevant syntax| Preserves source information| Builds semantic records| Records execution evidence

26.2 Hard-coding audit

The hard-coding audit MUST distinguish:

- Universal language restrictions.
- Explicit program constraints.
- External protocol limits.
- Target-specific constraints.
- Algorithmic limits.
- Implementation safety limits.

Only the first category is categorically forbidden when it imposes artificial finite hardware capacities on the universal language.

A target-specific limit MUST be validated at the appropriate layer and reported with a meaningful diagnostic.

27. Diagnostics and error recovery

Every production grammar family MUST define its relevant diagnostic categories.

Diagnostics SHOULD include:

- Stable diagnostic identifier.
- Severity.
- Primary source span.
- Related source spans where relevant.
- Clear explanation.
- Expected or valid alternatives where feasible.
- Applicable feature or language version.
- Recovery behavior.
- Machine-readable classification.
- Compatibility implications where relevant.

The lexer MUST define behavior for invalid characters, malformed literals, and unterminated constructs.

The parser MUST define error recovery sufficiently to avoid uncontrolled cascades while preserving useful diagnostics.

Semantic analysis MUST distinguish invalid syntax from unknown names, type errors, missing capabilities, resource infeasibility, policy violations, and unsupported target realization.

The frontend MUST NOT report a successful parse as proof of semantic validity.

28. Testing and conformance

Every production feature MUST have tests at the levels relevant to its responsibility.

The conformance suite MUST cover:

1. Token registry validation.
2. Lexical classification.
3. Lexer error handling.
4. Parser acceptance.
5. Parser rejection.
6. Precedence and associativity.
7. AST structure and source spans.
8. Structural validation.
9. Name and module resolution.
10. Type checking.
11. Effect checking.
12. Capability validation.
13. Resource requirements and constraints.
14. Contracts and invariants.
15. Policy and authorization boundaries.
16. Provenance.
17. Domain semantics.
18. Canonical IR mapping.
19. Diagnostics.
20. Compatibility and migration.
21. Dialect registration.
22. Portability and fallback.
23. Scalability.
24. Fuzzing and robustness.
25. Compiler/runtime integration where available.

28.1 Positive and negative fixtures

Each feature MUST include valid examples and invalid examples that demonstrate its boundaries.

Tests MUST verify not only that valid input is accepted, but also that invalid or unsupported input is rejected at the correct phase.

28.2 Differential conformance

Where the ANTLR grammar and Rust frontend both implement the same language surface, they MUST be compared against a shared corpus.

Differences MUST be classified as:

- Intended and versioned.
- Known but unresolved.
- Defects.

Unresolved differences MUST prevent the affected feature from being declared stable.

28.3 Golden tests

Golden outputs SHOULD cover tokens, parser results, AST representations, diagnostics, and semantic or IR results where stable serialization contracts exist.

Golden files MUST be updated through reviewed changes, not automatically rewritten to conceal regressions.

28.4 Fuzzing and robustness

The frontend SHOULD be fuzz-tested for malformed source, nested expressions, large declarations, Unicode, invalid literals, and adversarial input.

Fuzzing MUST respect test-runner resource budgets. Such budgets are test-execution safeguards, not universal language ceilings.

29. Scalability testing

Scalability MUST be tested independently from syntax acceptance.

The same parameterized source program SHOULD be exercised with progressively larger logical workloads and, where practical, multiple target profiles.

Tests MUST distinguish:

- Source expressiveness.
- Frontend resource consumption.
- Compiler resource consumption.
- Semantic feasibility.
- Backend compatibility.
- Runtime feasibility.
- Numerical or domain-specific equivalence.

A larger workload may legitimately fail because resources are unavailable. Such a failure MUST NOT imply that the source language has an undocumented universal size ceiling.

Where a compatible target can execute the program, its observable behavior MUST conform to the same source semantics.

29.1 Complexity contracts

Critical frontend and compiler algorithms SHOULD document expected complexity and resource behavior.

Implementations MUST avoid accidental quadratic behavior, unbounded recursion, uncontrolled memory growth, and unchecked integer arithmetic where these can be addressed safely.

Any defensive limit introduced for an implementation MUST be explicit, configurable where appropriate, and reported as an implementation constraint rather than a universal language restriction.

30. Canonical examples

The grammar directory SHOULD maintain versioned, executable conformance examples for:

grammar/examples/
    minimal.zm
    classical.zm
    quantum.zm
    hybrid.zm
    hdl.zm
    ai.zm
    distributed.zm
    networking.zm
    accelerator.zm
    resource-aware.zm
    policy-aware.zm
    adaptive.zm
    poco-reaf.zm

These paths are recommended targets, not proof that the files currently exist.

Existing examples MUST be inventoried and reconciled before adding duplicates.

Each example MUST declare its expected language version, feature requirements, and test role.

Examples intended to test parsing MUST not imply that every downstream semantic or runtime stage is implemented.

Examples used for end-to-end conformance MUST have explicit expected results and documented execution prerequisites.

31. File completion and freeze contract

A file may be frozen only when its required dependencies and interfaces are stable.

The following checklist applies to every production grammar or normative contract file.

- [ ] Purpose and authority are explicit.
- [ ] Ownership is unique.
- [ ] Non-ownership boundaries are explicit.
- [ ] Inputs and outputs are documented.
- [ ] Imports and dependencies are resolved.
- [ ] Public exports are declared.
- [ ] AST mapping is complete where applicable.
- [ ] Semantic mapping is complete where applicable.
- [ ] IR mapping is complete where applicable.
- [ ] Effects, capabilities, and resource interactions are documented where applicable.
- [ ] Contract and policy interactions are documented where applicable.
- [ ] Provenance obligations are documented where applicable.
- [ ] Diagnostics and source-span behavior are defined.
- [ ] Compatibility impact is assessed.
- [ ] Scalability and POCO-REAF requirements are satisfied.
- [ ] The hard-coding audit passes.
- [ ] Ambiguity and dependency checks pass.
- [ ] Positive and negative tests pass.
- [ ] Required integration tests pass.
- [ ] Manifest and registries are consistent.
- [ ] Generated artifacts are reproducible where applicable.
- [ ] Documentation matches the implementation status.
- [ ] Completion criteria are objectively verifiable.
- [ ] No known downstream dependency requires an unplanned interface change.

A file MUST NOT be marked stable while a mandatory item remains unresolved.

A feature that is designed but not fully integrated MUST remain "PROPOSED", "EXPERIMENTAL", "PLANNED", or "PARTIAL", as appropriate.

31.1 Freeze is not immutability

A freeze establishes a stable contract for a defined language or architecture version.

It does not prohibit future improvements.

Changes after freeze MUST go through the documented change process, update affected versioned contracts, and pass the relevant compatibility and conformance gates.

32. Required feature statuses

Every feature MUST have one explicit status from a controlled vocabulary.

Status| Meaning
"STABLE"| Normative, implemented, tested, and compatibility-controlled
"IMPLEMENTED"| Implemented and tested but not yet declared stable
"PARTIAL"| Some required behavior exists; integration or conformance remains incomplete
"PROPOSED"| Designed but not yet accepted as an implemented language feature
"EXPERIMENTAL"| Available behind an explicitly documented experimental boundary
"PLANNED"| Intentionally scheduled but not implemented
"DEPRECATED"| Retained for compatibility but discouraged
"HISTORICAL"| Preserved for reference, not current language law
"REJECTED"| Explicitly excluded from the language

The implementation-conformance reference MUST additionally report relevant implementation dimensions, such as lexer support, parser support, AST support, semantic support, IR support, tests, and stability.

A single status MUST NOT conceal missing stages of the implementation pipeline.

33. Change management

Every language-affecting change MUST include:

1. Motivation and intended behavior.
2. The canonical owner.
3. A specification update.
4. Grammar and lexer impact.
5. AST and semantic impact.
6. IR and downstream integration impact.
7. Effects, capability, resource, policy, and provenance impact where applicable.
8. Diagnostics and error-handling changes.
9. Compatibility and migration analysis.
10. Positive and negative tests.
11. Hard-coding and portability review.
12. Updated manifests and generated documentation where applicable.
13. A clear decision on feature status and freeze eligibility.

New ".g4" files MUST NOT be added merely to make the directory look more complete.

A new file is justified only when it establishes a distinct responsibility with a stable contract that cannot be maintained more clearly within an existing owner.

Before adding a new file, contributors MUST inspect the existing tree for duplicate or equivalent responsibilities.

34. Recommended directory responsibilities

The existing directory tree MUST be audited and normalized before files are moved or renamed.

The intended responsibility map is:

Directory| Primary responsibility
"antlr/"| Public ANTLR lexer and parser boundaries
"lexer/"| Canonical token registry and lexical contracts
"core/"| Universal source structure, names, attributes, requirements, and constraints
"modules/"| Module and import/export composition
"declarations/"| Domain-neutral declaration families
"types/"| Shared type-system syntax
"memory/"| Ownership, references, lifetimes, and memory semantics
"expressions/"| Canonical expression families
"functions/"| Function, parameter, generic, and calling contracts
"statements/"| Canonical statement dispatch and implementations
"effects/"| Shared effect system
"resources/"| Resource requirements, constraints, budgets, and negotiation intent
"policies/"| Shared policy syntax and composition
"security/"| Security, authorization, trust, and sandbox contracts
"validation/"| Cross-file and cross-layer conformance enforcement
"classical/"| Classical computational domain
"data/"| Data structures, schemas, streams, queries, and transformations
"concurrency/"| Tasks, futures, actors, and concurrency semantics
"distributed/"| Distributed execution intent and contracts
"networking/"| Networking constructs and capabilities
"hardware/"| Hardware descriptions, capabilities, and intent
"execution/"| Execution intent, resilience, simulation, and adaptation contracts
"compile/"| Compilation intent and artifact contracts
"quantum/"| Quantum syntax and semantic intent
"hdl/"| Hardware description and verification syntax
"hybrid/"| Classical, quantum, and hardware integration
"ai/"| Reasoning, learning, evidence, and AI computation
"interoperability/"| Foreign formats, FFI, ABI, and external interfaces
"dialects/"| Governed language extensions and format boundaries
"macros/"| Macro declaration, invocation, and expansion contracts
"metaprogramming/"| Reflection and controlled compile-time generation
"compatibility/"| Versions, feature gates, migrations, and deprecation
"specification/"| Normative human-readable language specification
"spec/"| Machine-readable contracts and validation data
"tests/"| Automated grammar and integration tests

Additional directories MAY be introduced when they establish a distinct, necessary responsibility.

Existing files MUST NOT be moved, renamed, or deleted solely to match this proposed map. Every structural change requires an ownership, dependency, compatibility, and build-impact audit.

35. Freeze sequence

The freeze process MUST be dependency-first.

Freeze 0 — Inventory and audit

Establish the complete file manifest and identify:

- Missing and duplicate files.
- Duplicate token and rule ownership.
- Orphaned grammars.
- Malformed paths.
- Contradictory specifications.
- Missing AST, semantic, and IR mappings.
- Missing tests.
- Unresolved dependencies.
- Hard-coded capacity violations.
- Generated files without declared inputs.
- Unverified implementation claims.

No broad feature expansion should precede this audit.

Freeze 1 — Authority and normative specification

Freeze the architectural and language authority contracts, including:

- "DESIGN.md"
- "specification/grammar-authority.md"
- "specification/language.md"
- "specification/language-principles.md"
- "specification/language-scope.md"
- "specification/language-version.md"
- "specification/portability.md"
- "specification/poco-reaf.md"
- "specification/scalability-model.md"
- "specification/compatibility.md"

These filenames MUST be reconciled against the actual repository. Existing equivalent documents MUST be reused rather than duplicated.

Freeze 2 — Lexical contracts

Freeze the token registry, keyword rules, operators, punctuation, identifiers, literals, comments, lexical errors, and public lexer boundary after reconciling them with the Rust lexer.

Freeze 3 — Core and names

Freeze source units, program composition, names, paths, visibility, attributes, version declarations, generic requirements, and shared constraints.

Freeze 4 — Types and memory

Freeze the common type constructors, generic rules, type constraints, ownership, borrowing, and memory-related contracts.

Freeze 5 — Expressions

Freeze the canonical expression algebra, precedence, associativity, and AST mapping.

Freeze 6 — Declarations, statements, and functions

Freeze declaration dispatch, statement dispatch, functions, parameters, returns, and generic integration.

Freeze 7 — Semantic control plane

Freeze effects, capabilities, resources, contracts, policies, security boundaries, provenance, and cross-layer validation contracts.

Freeze 8 — Classical and data

Freeze the classical and data domains against the common semantic foundation.

Freeze 9 — Concurrency, distribution, and networking

Freeze concurrency and distributed execution contracts, including network-facing effects and capabilities.

Freeze 10 — Hardware, execution, and compilation

Freeze target intent, resource negotiation interfaces, execution contracts, compilation artifacts, and reproducibility requirements.

Freeze 11 — Quantum

Freeze quantum syntax and semantic mappings through "quantum::ir", including resource and capability integration.

Freeze 12 — HDL

Freeze HDL syntax, hardware semantics, and canonical representation mappings.

Freeze 13 — Hybrid

Freeze cross-domain integration after its dependencies are stable.

Freeze 14 — AI

Freeze AI syntax and semantic mappings through the common type, effect, resource, policy, and provenance models.

Freeze 15 — Interoperability and dialects

Freeze external-format boundaries, dialect registration, and version compatibility.

Freeze 16 — Macros and metaprogramming

Freeze controlled generation and reflection against the stable AST and semantic model.

Freeze 17 — Compatibility and migration

Freeze compatibility rules against the completed feature inventory and supported language versions.

Freeze 18 — Universal root composition

Freeze "grammar/Zamani.g4", "grammar/antlr/ZamaniLexer.g4", and "grammar/antlr/ZamaniParser.g4" only after their subordinate imports, dispatch rules, token vocabulary, and ownership contracts have been verified.

Freeze 19 — Full conformance and directory closure

Run the complete validation suite, verify all statuses and mappings, and approve the grammar directory for the next development stage.

The sequence is dependency-driven. A stage MAY proceed in parallel with another only where their contracts are independent and that independence is documented.

36. Production-readiness acceptance gate

The "grammar/" directory MUST NOT be declared production-ready until all mandatory conditions below are satisfied.

Architecture and authority

- [ ] One architectural authority.
- [ ] One normative language authority.
- [ ] One canonical lexical model.
- [ ] One canonical parser composition boundary.
- [ ] One canonical domain-neutral AST.
- [ ] Shared semantic foundations.
- [ ] Canonical IR boundaries and versioning.
- [ ] Unique ownership for public grammar symbols.
- [ ] Dependency closure with no unresolved cycles.

Grammar and lexer

- [ ] Every required grammar import resolves.
- [ ] All root and domain dispatch rules are unambiguous.
- [ ] Every emitted token has one canonical identity.
- [ ] Contextual keywords and compatibility behavior are specified.
- [ ] Precedence and associativity are tested.
- [ ] Duplicate-looking files have been reconciled.
- [ ] All supported grammar entry points are documented.

Semantics and integration

- [ ] Required AST mappings are complete.
- [ ] Required semantic mappings are complete.
- [ ] Required IR mappings are complete.
- [ ] Effects and capabilities are integrated.
- [ ] Resources, constraints, and negotiation intent are integrated.
- [ ] Contracts and policies have defined validation behavior.
- [ ] Provenance and source spans are preserved.
- [ ] Diagnostics distinguish syntax, semantic, and realization failures.
- [ ] No domain bypasses the canonical semantic pipeline.

Scalability and portability

- [ ] No universal finite hardware-capacity ceilings.
- [ ] Resource requirements are distinct from available capacity.
- [ ] Capability negotiation is specified.
- [ ] Target-independent semantics are tested.
- [ ] Fallback behavior is explicit and semantics-aware.
- [ ] Portable and target-specific artifacts are distinguished.
- [ ] Versioning and migration contracts exist.
- [ ] Large-input and resource-exhaustion behavior is documented and tested.

Safety and testing

- [ ] Production Rust code uses safe Rust only.
- [ ] The required Rust baseline is verified in the build configuration.
- [ ] Lexer and parser conformance suites pass.
- [ ] AST, semantic, and IR tests pass where applicable.
- [ ] Positive and negative tests cover every stable feature.
- [ ] Fuzzing and robustness checks exist for critical frontend paths.
- [ ] Hard-coding, ownership, dependency, and status validators pass.
- [ ] No known unresolved issue is concealed by a stable status.

Governance

- [ ] Manifest and registries agree with the actual tree.
- [ ] Historical and proposed features are clearly labeled.
- [ ] Generated documents can be reconciled with their authoritative inputs.
- [ ] Every frozen file has complete contracts and acceptance evidence.
- [ ] All required integration and compatibility reviews are complete.

An unchecked mandatory item is a production-readiness blocker, not a documentation-only task.

37. Definition of a frozen grammar directory

The grammar directory is frozen for a declared language version only when:

1. Every relevant file is inventoried.
2. Every public symbol has one owner.
3. All dependencies and composition paths are resolved.
4. The normative specification agrees with the accepted syntax and semantics.
5. The Rust frontend and ANTLR representations satisfy their required conformance contracts.
6. AST, semantic, diagnostic, and IR mappings are complete for all stable features.
7. Classical, quantum, HDL, hybrid, AI, data, concurrency, distributed, and interoperability boundaries are governed by explicit contracts.
8. Resource and capability requirements remain target-independent.
9. No artificial universal hardware-capacity ceilings remain.
10. Compatibility and versioning are explicit.
11. Automated validation and applicable tests pass.
12. Each stable feature has evidence of implementation and integration.
13. Remaining work is clearly labeled as proposed, experimental, partial, planned, or otherwise non-stable.

A frozen directory MAY contain documented planned features. It MUST NOT present them as completed production functionality.

38. Final architectural contract

Zamani's language architecture MUST preserve this separation:

SOURCE PROGRAM
      |
      v
CANONICAL LANGUAGE MEANING
      |
      +--> Types and values
      +--> Effects
      +--> Capabilities
      +--> Resources and requirements
      +--> Contracts and policies
      +--> Provenance and evidence
      |
      v
CANONICAL SEMANTIC MODEL
      |
      +--> Classical IR
      +--> quantum::ir
      +--> HDL/hardware representation
      +--> Governed additional domain IRs
      |
      v
TARGET-INDEPENDENT OPTIMIZATION
      |
      v
LOWERING AND REALIZATION
      |
      v
CAPABILITY/RESOURCE NEGOTIATION
      |
      v
ROUTING, SCHEDULING, AND RECOVERY
      |
      v
ZQN / HAL / TARGET EXECUTION

The language defines what a program means. The semantic model validates that meaning. The compiler and runtime determine how it can be realized with the available capabilities and resources.

The architecture MUST permit the same source program to scale across compatible environments without introducing a new language or hard-coded machine-size ceilings. It MUST also report honestly when a requested computation is infeasible, unsupported, unauthorized, or incompatible with the selected target.

The immediate engineering priority is Freeze 0: inventory the actual repository, establish unique ownership and dependency closure, reconcile the specification with the Rust frontend and ANTLR grammars, and record implementation status. Feature expansion and permanent freezing should follow only after that evidence exists.

This contract is the foundation for freezing "grammar/" as a coherent, versioned language subsystem before building the next stage of Zamani.