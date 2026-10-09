Zamani Grammar Authority and Ownership Specification

Canonical path: "grammar/specification/grammar-authority.md"
Status: Normative specification
Applies to: The complete "grammar/" subsystem and its language-definition interfaces with the rest of the repository
Implementation language: Rust
Minimum Rust version: Rust 1.97 or later
Rust edition: 2021
Implementation safety: Production Rust code MUST NOT use "unsafe" Rust
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: Support the smallest meaningful computation and arbitrarily large computations, subject to program semantics, available resources, implementation capabilities, target capabilities, policy, representational limits, and physical reality.

---

1. Purpose

This specification establishes the binding authority, ownership, dependency, integration, conformance, and change-control rules for the Zamani language definition.

Its purpose is to ensure that Zamani has one coherent source-language definition across:

- lexical rules and token definitions;
- concrete syntax and grammar composition;
- names, modules, declarations, expressions, statements, and types;
- domain-neutral abstract syntax trees (ASTs);
- structural and semantic validation;
- type, effect, capability, resource, contract, policy, and provenance models;
- classical, quantum, hardware-description-language (HDL), hybrid, AI, data, and other computing domains;
- canonical intermediate representations (IRs);
- compilation, optimization, lowering, and specialization;
- target discovery, capability negotiation, routing, scheduling, and execution;
- diagnostics, developer tooling, interoperability, and compatibility;
- tests, examples, conformance evidence, and release governance.

The specification prevents a grammar file, implementation detail, historical document, generated artifact, or individual domain from silently becoming a competing language authority.

It also establishes the conditions under which a file can be completed and frozen without requiring semantic rework when unrelated files or downstream implementations evolve.

A file is not production-ready merely because it exists, has extensive documentation, or contains syntactically valid grammar. It is production-ready only when its ownership, dependencies, externally visible contracts, integration mappings, compatibility behavior, tests, and acceptance criteria are complete and verified.

---

2. Normative terminology

The terms MUST, MUST NOT, REQUIRED, SHOULD, SHOULD NOT, and MAY are normative.

- Authority: The repository artifact responsible for defining a particular contract.
- Owner: The single designated authority responsible for maintaining a symbol, rule, token, schema, semantic concept, or interface.
- Normative specification: A document that defines required language behavior.
- Grammar: A formal description of valid source syntax.
- Lexer: The component that converts source characters into tokens and source spans.
- Parser: The component that converts tokens into a syntactic structure.
- AST: The structured representation of parsed source constructs.
- Semantic model: The validated meaning of a program, independent of a particular physical target.
- Canonical IR: A versioned intermediate representation with defined ownership, semantics, invariants, and compatibility rules.
- Capability: A supported operation or property of a possible execution environment.
- Resource: A quantity or service required, consumed, reserved, or supplied during computation.
- Requirement: A condition that must be satisfied for a proposed realization to be valid.
- Constraint: A restriction on allowed realizations.
- Preference: A non-mandatory selection criterion.
- Hint: Optimization guidance that MUST NOT change observable program meaning.
- Policy: A rule governing authorization, resource use, execution, adaptation, fallback, or another permitted action.
- Effect: A description of the observable or semantically relevant consequences of an operation.
- Provenance: Information recording the origin, evidence, transformations, verification, or decision history of an artifact or semantic fact.
- Dialect: A versioned, registered extension or external-language integration boundary.
- Conformance: Demonstrable satisfaction of applicable specification requirements.
- Frozen file: A file whose ownership, dependencies, exports, integrations, tests, compatibility requirements, and acceptance criteria have been approved and verified.

---

3. Fundamental authority principles

3.1 One language, multiple computational domains

Zamani is one extensible programming language. Classical computing, quantum computing, HDL, hybrid computing, AI, data processing, networking, concurrency, distributed computing, and future computational domains are domains within that language.

All domains MUST use the common language foundations wherever applicable:

- source locations and diagnostics;
- names, modules, and declarations;
- expressions and types;
- effect and capability models;
- resource requirements and constraints;
- contracts and policies;
- provenance and compatibility;
- structural validation and semantic analysis.

A domain MAY introduce specialized syntax and domain-specific semantics. It MUST NOT silently establish an incompatible language, independent universal parser, competing semantic authority, or ungoverned IR.

3.2 Separate syntax from meaning and realization

The language definition MUST preserve the following separation:

1. Syntax determines whether source conforms to the language's grammatical rules.
2. AST construction represents the parsed source without prematurely deciding target-specific meaning.
3. Structural validation checks structural invariants and relationships.
4. Semantic analysis establishes names, types, effects, capabilities, resources, contracts, policies, and other applicable meaning.
5. Domain lowering translates validated domain meaning into the appropriate canonical IR.
6. Target realization selects and applies compatible implementation strategies using actual capabilities and available resources.

A grammar production MUST NOT silently perform semantic analysis, inspect hardware, allocate physical resources, authorize privileged actions, or execute a program.

3.3 No file has implicit authority

A file's authority MUST be determined by this specification, its declared ownership, and its registered contract.

A file MUST NOT become authoritative merely because it is:

- older or larger than another file;
- named "canonical", "universal", "root", or "production";
- imported by many files;
- generated by a tool;
- referenced by an example;
- implemented in Rust;
- accepted by one parser;
- mentioned in a historical design document.

If two files appear to own the same contract, the conflict MUST be resolved explicitly. Implementations MUST NOT arbitrarily choose whichever definition is easiest to use.

3.4 Normative intent does not prove implementation

A feature may be specified before it is implemented.

However, the repository MUST distinguish specification status from implementation status. A documented or grammatically parseable feature MUST NOT be reported as semantically implemented, backend-supported, tested, or stable without corresponding evidence.

3.5 Physical infinity is not a language guarantee

Zamani MUST NOT impose arbitrary universal resource ceilings where its semantics do not require them.

This does not promise literal infinite memory, infinite execution, infinite physical devices, or unlimited compiler resources. Every implementation may encounter actual resource exhaustion, representation limits, target incompatibilities, or physical constraints.

When a requirement cannot be satisfied, the implementation MUST report the failure or use an explicitly permitted, semantically valid alternative. It MUST NOT silently change program meaning.

---

4. Authority hierarchy

The following hierarchy defines which artifact governs each category of decision. It is a division of responsibility, not permission for lower-level artifacts to contradict higher-level requirements.

4.1 Architectural authority: "grammar/DESIGN.md"

"grammar/DESIGN.md" owns the architecture of the grammar subsystem.

It defines:

- the architectural boundaries and invariants;
- the division of responsibility between specification, grammar, and implementation;
- the expected grammar-composition model;
- dependency direction and ownership requirements;
- AST, semantic, and IR integration principles;
- scalability and portability constraints;
- testing and freeze requirements.

This document MUST remain consistent with the repository's broader architecture.

It MUST NOT independently redefine source-level language semantics already owned by the normative language specification.

4.2 Human-readable language authority: "grammar/specification/"

The "grammar/specification/" directory owns the normative human-readable definition of Zamani.

Its documents define the language's intended source syntax, semantics, type system, execution model, portability guarantees, compatibility rules, and domain boundaries.

The principal documents include:

- "language.md"
- "language-principles.md"
- "language-scope.md"
- "language-version.md"
- "grammar-authority.md"
- "lexical.md"
- "syntax.md"
- "syntax-model.md"
- "semantic-model.md"
- "semantics.md"
- "types.md"
- "domains.md"
- "execution-model.md"
- "compilation-model.md"
- "poco-reaf.md"
- "scalability-model.md"
- "portability.md"
- "compatibility.md"
- "extensibility.md"
- "reserved-space.md"

The list is an expected integration inventory, not permission to assume that every named file exists or is complete. The manifest MUST verify actual paths and statuses before a freeze decision.

Each document MUST have a bounded purpose and explicit relationships to its neighboring specifications.

"language-scope.md" owns the breadth of language scope. This document owns authority and ownership boundaries. "poco-reaf.md" owns the detailed portability and compilation contract. "scalability-model.md" owns the detailed scalability model. "language-version.md" and "compatibility.md" own their respective versioning and compatibility contracts.

No one document should reproduce all the others as an independent source of truth.

4.3 Machine-oriented contracts: "grammar/spec/"

The "grammar/spec/" directory owns machine-oriented specification artifacts and mechanically enforceable contracts.

Depending on the repository's established structure, these may include:

- syntax and semantics contracts;
- token and feature registries;
- resource and capability schemas;
- AST and semantic mappings;
- IR mappings;
- diagnostic registries;
- compatibility metadata;
- conformance metadata.

Machine-oriented contracts MUST identify their schema or format version and validation rules.

They MUST agree with the normative human-readable specification. When a conflict is discovered, the conflict MUST be recorded and resolved through a reviewed change. A machine-readable artifact MUST NOT silently override the normative language specification simply because tooling consumes it.

Where a machine contract is generated from an authoritative source, its generated status, source, generation procedure, and validation requirements MUST be documented.

4.4 Canonical ANTLR grammar: "grammar/Zamani.g4"

"grammar/Zamani.g4" is the public ANTLR grammar entry point and composition root.

It MUST represent the normative language grammar through the approved composition hierarchy.

It MUST NOT become an independent definition of domain-specific semantics.

Its precise ownership MUST be reconciled with "grammar/antlr/ZamaniParser.g4" as specified in Section 6. The repository MUST NOT maintain two independently authoritative parser compositions.

4.5 Public ANTLR lexer boundary: "grammar/antlr/ZamaniLexer.g4"

"grammar/antlr/ZamaniLexer.g4" owns the public ANTLR lexer boundary.

The canonical lexical rules and token ownership are defined by the registered lexical subsystem, including "grammar/lexer/tokens.g4" and its documented composition.

The public boundary MUST NOT duplicate individual token definitions or create an alternative lexical vocabulary.

4.6 Public ANTLR parser boundary: "grammar/antlr/ZamaniParser.g4"

"grammar/antlr/ZamaniParser.g4" owns the designated public ANTLR parser-composition interface, subject to the single-root reconciliation required by Section 6.

It composes the approved subordinate grammar modules and exposes the parser entry points required by the language contract.

It MUST NOT duplicate domain implementations already owned by subordinate grammar modules.

4.7 Implementation-conformance reference: "grammar/grammar.md"

"grammar/grammar.md" documents the syntax and language behavior actually supported by the repository's reference implementation.

It MUST distinguish:

- what the normative specification requires;
- what the grammar can parse;
- what the AST can represent;
- what semantic analysis validates;
- what the IR and backends support;
- what is tested and stable.

It MUST NOT claim that an aspirational feature is implemented merely because it appears in another document or grammar file.

It MUST remain consistent with the implementation surfaces, including "src/lexer.rs", "src/parser.rs", "src/ast/", semantic analysis, and the applicable IR and compiler subsystems.

4.8 Extended and historical design reference: "grammar/Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" is an extended design and historical reference. It is not an independent language authority.

Each feature documented there MUST be identified using the repository's approved lifecycle vocabulary, such as:

- "PROPOSED"
- "DESIGNED"
- "SPECIFIED"
- "IMPLEMENTED"
- "PARTIALLY_IMPLEMENTED"
- "EXPERIMENTAL"
- "STABLE"
- "DEPRECATED"
- "HISTORICAL"
- "REJECTED"

These labels MUST be defined in the canonical status registry rather than independently redefined in each document.

4.9 Navigation: "grammar/README.md"

"grammar/README.md" is the navigation and subsystem overview.

It MUST link to the relevant authoritative specifications, grammar entry points, implementation-conformance reference, validation tools, tests, and freeze process.

It MUST NOT become a competing language specification.

---

5. Conflict resolution

When repository artifacts disagree, the following procedure is REQUIRED.

1. Identify the contract in conflict. Determine whether the disagreement concerns architecture, language meaning, machine metadata, concrete grammar, implementation behavior, or historical material.
2. Identify the designated owner. Use the authority hierarchy and ownership registry to locate the file responsible for that contract.
3. Record the conflict. State the affected paths, symbols, feature status, expected behavior, actual behavior, and compatibility consequences.
4. Determine the normative behavior. Use the applicable language specification and architecture contract. If those documents disagree, resolve that disagreement before changing dependent grammar or implementation files.
5. Update the owner first. Change the file that owns the contract and version any externally visible change as required.
6. Update registered dependents. Update grammar composition, machine contracts, implementation mappings, documentation, and tests only where their interfaces or obligations are affected.
7. Validate the complete dependency closure. Run all affected conformance checks, not merely the test for the edited file.
8. Record the outcome. Update the manifest, status, compatibility record, and freeze evidence.

A downstream implementation MUST NOT establish a new language rule merely to accommodate its current limitations.

Likewise, a normative specification MUST NOT claim support that the implementation has not demonstrated.

If a decision is unresolved, the affected contract MUST remain unfrozen.

---

6. Resolving the parser-root boundary

The repository has both "grammar/Zamani.g4" and "grammar/antlr/ZamaniParser.g4". Their relationship MUST be explicit and mechanically validated.

The production architecture MUST have one authoritative parser-composition definition, not two independently maintained dispatch trees.

The intended division is:

6.1 "grammar/Zamani.g4"

Owns the public root grammar identity and the supported root entry point, as required by the existing ANTLR build and tooling arrangement.

It SHOULD remain a thin composition facade. It MUST NOT independently implement the syntax for quantum operations, HDL modules, expressions, types, or other subordinate domains.

6.2 "grammar/antlr/ZamaniParser.g4"

Owns the canonical parser-composition rules when this file is designated as the single implementation of the parser dispatch hierarchy.

It MUST define or delegate the approved entry points for source units, declarations, statements, expressions, types, and domain dispatch through the registered subordinate grammar modules.

6.3 Required reconciliation

Before either file is frozen, the repository MUST establish and test one of the following designs:

- "Zamani.g4" is the authoritative parser root and the ANTLR parser boundary is a non-competing wrapper or generated interface; or
- "ZamaniParser.g4" is the authoritative parser composition and "Zamani.g4" is a thin facade that does not duplicate parser rules.

The chosen design MUST be compatible with the actual ANTLR toolchain, grammar import behavior, token vocabulary, generated sources, and build scripts.

The choice MUST be recorded in "DESIGN.md", the grammar manifest, and the build/conformance configuration.

Two files MUST NOT both independently define universal declaration, statement, expression, or type dispatch.

A freeze MUST be blocked if the root ownership model is ambiguous or the parser generation process cannot demonstrate which grammar is authoritative.

---

7. Lexical authority and token ownership

7.1 Single token authority

Every emitted token MUST have exactly one registered owner.

The lexical ownership contract MUST identify:

- token identifier;
- source spelling or spelling rule;
- token category;
- owning grammar or registry entry;
- reserved or contextual status;
- applicable dialect or feature gate;
- compatibility identifier;
- documentation entry;
- positive and negative tests;
- implementation mapping, where applicable.

"grammar/lexer/tokens.g4" and the approved lexical composition files MUST follow this contract.

7.2 No competing token definitions

A token MUST NOT be independently redefined in:

- "Zamani.g4";
- "antlr/ZamaniLexer.g4";
- "antlr/ZamaniParser.g4";
- a domain grammar;
- a generated file;
- a separate token registry.

A token may be referenced by many grammars, but its lexical definition and compatibility identity MUST remain unambiguous.

Names that appear duplicative, such as alternative spellings or identifiers for question marks, ampersands, closure forms, or lambda forms, MUST be consolidated or explicitly distinguished by documented lexical or syntactic meaning.

7.3 Lexer and Rust implementation agreement

The ANTLR lexer and the Rust lexer MUST be tested against a shared lexical conformance corpus.

Differences in accepted spelling, token boundaries, contextual-keyword handling, literal interpretation, escape rules, source positions, or lexical diagnostics MUST be intentional and documented.

If the implementation accepts syntax outside the specified language, that behavior MUST be classified and corrected or explicitly incorporated through the normal language-change process.

---

8. Grammar module ownership

Every ".g4" file MUST declare its purpose, ownership, boundaries, dependencies, and integration contract.

At minimum, the file's header or linked contract MUST define:

- Purpose
- Status
- Authority
- Owns
- Does not own
- Inputs
- Outputs
- Dependencies
- Imports
- Exports
- Consumers
- AST mapping
- Semantic mapping
- IR mapping, where applicable
- Diagnostic mapping
- Compatibility contract
- Scalability and portability contract
- Hard-coding audit
- Ambiguity and precedence contract
- Error-handling expectations
- Positive and negative tests
- Freeze criteria

If a field is not applicable, the contract MUST state why. Silence MUST NOT be interpreted as proof that no integration is needed.

8.1 One owner per exported rule

Each exported grammar rule MUST have one canonical owner.

Composition grammars MAY reference or delegate to rules owned by subordinate grammars. They MUST NOT silently redefine the same public rule.

A duplicate-looking rule or file MUST either be consolidated or assigned a precise, tested distinction. File names alone do not establish distinct semantics.

8.2 Domain boundaries

Domain grammars MUST own only their domain-specific source constructs.

For example:

- "quantum/" owns quantum syntax, not physical qubit allocation.
- "hdl/" owns hardware-description syntax, not physical device discovery.
- "hardware/" owns hardware-intent syntax, not live hardware inventory.
- "resources/" owns resource-requirement syntax, not resource allocation.
- "effects/" owns effect syntax, not runtime effect execution.
- "policies/" owns policy syntax and references, not the authorization engine.
- "execution/" owns execution intent and applicable source constructs, not the runtime scheduler implementation.
- "dialects/" owns extension boundaries, not unregistered changes to the universal language.

These boundaries MUST be reflected in ownership and dependency contracts.

---

9. Canonical processing pipeline

The language-definition artifacts MUST support the following logical pipeline:

Zamani source
    |
    v
Normative lexical and syntax rules
    |
    v
Lexer / token stream / source spans
    |
    v
Parser
    |
    v
Domain-neutral AST
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
    +--> Types and constraints
    +--> Effects
    +--> Capabilities
    +--> Resources
    +--> Contracts
    +--> Policies and authorization obligations
    +--> Provenance and evidence
    |
    v
Validated semantic model
    |
    +--> Classical IR
    +--> quantum::ir
    +--> HDL / hardware semantic representation
    +--> Other registered domain representations
    |
    v
Target-independent transformations
    |
    v
Domain lowering and specialization
    |
    v
Capability and resource negotiation
    |
    v
Placement / routing / scheduling / resilience
    |
    v
ZQN and HAL integration, where applicable
    |
    v
Concrete target realization

This is a logical ownership and data-flow contract. It does not require every implementation to use identical internal pass names or separate binaries.

Implementations MAY fuse passes where doing so preserves the observable behavior, diagnostics, validation obligations, and versioned interfaces.

9.1 Domain-neutral AST

Parsing MUST preserve enough source information to build a domain-neutral AST, including source spans and the syntactic distinctions required by the language specification.

The AST MUST NOT prematurely commit to a physical target or replace unresolved semantic obligations with guessed values.

9.2 Semantic validation

The semantic pipeline MUST validate the applicable type, effect, capability, resource, contract, policy, and provenance requirements before claiming that a program is valid for a particular realization.

A syntactically valid program MUST NOT automatically be described as executable on every target.

9.3 Canonical IR boundaries

Each domain MUST map through validated semantic meaning to its registered IR or semantic representation.

The quantum domain MUST use the repository's canonical "quantum::ir" boundary. Grammar files MUST NOT introduce a competing universal quantum IR.

Classical and HDL/hardware processing MUST likewise use their designated repository representations and explicit mapping contracts.

An IR mapping MUST define semantics, invariants, ownership, serialization or interchange requirements where applicable, versioning, and conformance tests.

---

10. Resource, capability, and scalability authority

10.1 Resource-parametric language

The language MUST express requirements and constraints without assuming a fixed implementation capacity.

Examples of valid requirement forms include:

requires qubits >= n
requires memory >= required_memory
requires capability("quantum.measurement")
requires capability("gpu.compute")
requires capability("tensor.compute")
requires topology(...)

The concrete syntax MUST follow the canonical syntax specification. These examples illustrate the semantic model and MUST NOT be treated as permission to create duplicate syntax rules.

The language MAY also provide "prefer", "constrain", "allow", and "forbid" constructs where defined by the normative syntax and semantics.

10.2 No universal machine ceilings

The universal language grammar MUST NOT introduce finite physical implementation limits such as:

- maximum qubit or logical-qubit count;
- maximum CPU, core, thread, GPU, FPGA, QPU, or accelerator count;
- maximum node, process, task, channel, or device count;
- maximum memory or storage capacity;
- maximum register width or vector width;
- maximum tensor rank or tensor dimension;
- maximum network or topology size.

The repository MUST also audit for disguised limits, including hard-coded arrays, fixed-size registries, implicit truncation, bounded identifiers where the specification promises otherwise, or grammar alternatives that enumerate all supported devices or operations.

Finite values remain valid as ordinary program data, type parameters, constraints, or explicit target requirements. They MUST NOT become universal language-wide ceilings.

10.3 Responsibility boundary

The grammar may express a requirement. Semantic analysis may validate its meaning. A compiler or runtime may evaluate it against the environment. Hardware discovery and resource allocation belong to the responsible target or runtime subsystem.

The grammar MUST NOT attempt to determine how many devices exist, which device is selected, which physical qubits are available, or which scheduling and routing decisions should be made.

10.4 Failure semantics

If required resources or capabilities are unavailable, the implementation MUST produce a defined, observable outcome.

Permitted outcomes include a clear rejection, deferred realization, or an explicitly authorized alternative realization. The exact outcomes MUST be specified by the execution and portability contracts.

No implementation may silently weaken a requirement, omit an effect, or substitute an approximation that changes program meaning without explicit authorization.

10.5 No promise of unbounded physical execution

The absence of artificial language ceilings does not eliminate finite limits imposed by the compiler, runtime, operating system, target, representation, policy, or physical environment.

Where an implementation limit exists, it MUST be handled at the appropriate layer and reported accurately. A target-specific limitation MUST NOT be misrepresented as a universal grammar restriction.

---

11. POCO-REAF authority

POCO-REAF is a cross-layer requirement, not a property that grammar composition alone can guarantee.

The language specification MUST preserve target-independent program meaning wherever the program's declared semantics permit it.

The compiler and runtime MUST distinguish:

- portable source semantics;
- versioned semantic artifacts;
- target-specific lowering;
- environment-dependent capability checks;
- actual resource availability;
- target-dependent performance;
- execution behavior and compatibility guarantees.

11.1 Compile-once contract

“Compile once” MUST refer to a clearly defined portable artifact or compilation stage in the POCO-REAF contract.

It MUST NOT be interpreted as a promise that one target-specific machine-code binary will execute natively on every incompatible architecture indefinitely.

A portable artifact SHOULD record the applicable language and semantic versions, feature set, dialect versions, IR version, requirements, capabilities, constraints, policies, effects, provenance, and reproducibility information.

The artifact contract MUST define which portions are portable and which require target-specific realization.

11.2 Run-everywhere contract

A portable program MAY be considered for multiple targets without source rewriting merely because hardware has changed.

A target is eligible only when its capabilities, resources, policies, and semantics satisfy the program's requirements or an explicitly authorized alternative is valid.

11.3 Run-anywhere contract

A program MUST declare or inherit the relevant constraints and fallback rules. The compiler or runtime MUST apply them deterministically according to the language's policy and compatibility contracts.

11.4 Forever contract

Long-term portability requires versioned syntax and semantics, stable artifact contracts, migration rules, dialect governance, and compatibility policy.

It MUST NOT be represented as a guarantee that all old binaries or tools will run unchanged on every future system.

"grammar/specification/poco-reaf.md", "compilation-model.md", "portability.md", "language-version.md", and "compatibility.md" own the detailed requirements for these guarantees.

---

12. Cross-domain integration requirements

All domain grammars MUST integrate with the common language foundation.

12.1 Classical and systems computing

"classical/" MUST use the common types, expressions, functions, memory, effects, resources, and validation models.

It MUST NOT introduce an independent universal type system or assume a fixed processor architecture.

12.2 Quantum computing

"quantum/" MUST define extensible source syntax for quantum operations, parameters, targets, controls, measurements, states, channels, dynamic circuits, and other specified quantum constructs.

Quantum operation names and extensions MUST be data-driven or registered through the approved operation and dialect contracts where appropriate. The core grammar MUST NOT need to enumerate every present or future quantum operation.

Quantum syntax MUST map through the AST and semantic analysis to "quantum::ir".

The grammar MUST NOT allocate physical qubits, select a QPU, implement routing, perform calibration, schedule execution, or implement quantum error correction.

12.3 HDL and hardware/software co-design

"hdl/" MUST express the specified hardware structure and intent through parameterized, typed constructs.

It MUST NOT impose universal bus widths, register counts, memory capacities, clock counts, or device inventories.

Hardware-description semantics and their IR mappings MUST be explicit, versioned, and tested.

12.4 Hybrid computing

"hybrid/" MUST define the boundaries and coordination constructs between domains without redefining classical, quantum, or HDL semantics.

It MUST reuse the common effects, resources, capabilities, contracts, policies, and execution model.

12.5 AI, data, concurrency, and distributed computing

These domains MUST reuse the common language foundations.

Learning, inference, reasoning, data transformation, actor communication, parallel execution, and adaptation MUST have defined semantic and effect contracts.

Any adaptation or code generation that can change executable behavior MUST be governed by explicit authorization, capabilities, effects, policies, provenance, and validation.

Application-specific vocabulary SHOULD remain in libraries, registered dialects, capabilities, or policies unless a language-wide construct has been formally justified and specified.

12.6 Interoperability and dialects

"interoperability/" MUST distinguish parsing an external format, importing its semantics, invoking foreign code, and declaring an ABI from defining Zamani's own language.

"dialects/" MUST provide a governed extension boundary with identity, namespace, version, dependencies, feature status, syntax ownership, semantic mapping, compatibility rules, and conformance tests.

An external language format MUST NOT become universal Zamani syntax merely because an adapter or frontend exists.

---

13. Rust implementation and safety requirements

The repository's production Rust implementation MUST use Rust 1.97 or later and the Rust 2021 edition, consistent with the package's declared minimum supported version.

Production Rust code MUST NOT use "unsafe".

This requirement applies to lexer and parser implementations, AST construction, semantic analysis, IR translation, validation tools, generators, diagnostics, compiler integration, and any other Zamani-owned production Rust code.

13.1 Grammar-to-Rust contract

Every implemented grammar construct MUST have an explicit mapping to the applicable Rust AST or parser representation.

The mapping MUST specify:

- the source construct;
- the owning AST type or node;
- the source-span behavior;
- required child nodes and optional fields;
- validation obligations;
- diagnostics for invalid or unsupported constructs;
- semantic mapping;
- IR mapping where applicable;
- test coverage and status.

The exact Rust file path MUST be verified against the repository. A specification MUST NOT invent an AST module or claim that a particular node already exists without checking the implementation.

13.2 No grammar-only safety claims

The absence of embedded actions or unsafe code in a grammar file does not prove that the full implementation is safe.

Safety claims MUST be supported by the relevant source audit, build configuration, static checks, and tests.

If a future language feature would require unsafe Rust to implement, the implementation MUST be redesigned to comply with this policy rather than silently relaxing it.

13.3 Failure handling

Parsing, validation, semantic analysis, IR conversion, and resource negotiation MUST handle invalid input and recoverable failures through explicit results and diagnostics.

The implementation MUST NOT rely on undefined behavior, memory unsafety, silent truncation, unchecked assumptions about input size, or undocumented panic behavior as a substitute for a defined contract.

---

14. Dependency and integration contracts

Every authoritative file MUST declare its dependency direction and the contracts it consumes.

The machine-readable manifest and dependency registry MUST be the mechanically validated source for file-level dependency information. Prose documentation MAY explain the graph but MUST NOT silently maintain a contradictory graph.

The dependency graph MUST be acyclic unless a specific toolchain requires a cycle that is explicitly documented and proven safe. Circular grammar imports or circular authority relationships MUST be treated as errors.

14.1 Required integration mapping categories

Each applicable language feature MUST be traceable through the following stages:

Stage| Required mapping
Specification| Normative rule and version
Lexical system| Tokens and lexical constraints
Grammar| Owning rule and composition path
AST| Node or explicit non-applicability
Validation| Structural invariants and errors
Semantics| Meaning, types, effects, capabilities, resources, and policies as applicable
IR| Canonical representation or explicit non-applicability
Compiler/runtime| Realization contract or explicit unsupported status
Diagnostics| Stable error identifiers and source locations
Tests| Positive, negative, and relevant integration cases
Compatibility| Versioning, migration, and stability classification

A feature MUST NOT be marked complete while a required mapping is unknown.

A mapping to a not-yet-implemented component MAY be recorded as planned, but the feature MUST NOT be reported as fully implemented or frozen for production conformance.

14.2 Integration details must precede implementation

Before a file is considered complete, its contract MUST already identify its consumers and integration points.

A later backend, runtime, or domain extension MUST NOT require editing a frozen grammar file merely to add an implementation of an already-defined stable interface.

If a genuinely new source-language construct or semantic obligation is required, that is a language change and MUST go through the normal specification and compatibility process. It is not an accidental integration fix.

---

15. Machine-readable ownership and dependency governance

The repository MUST maintain a validated inventory of authoritative grammar and specification files.

The exact manifest path MUST be established in "DESIGN.md" and "README.md"; the project MUST NOT maintain multiple competing inventories.

The inventory MUST record, at minimum:

- canonical path;
- file kind;
- owner;
- authority level;
- lifecycle status;
- public exports;
- dependencies and imports;
- consumers;
- AST mapping;
- semantic mapping;
- IR mapping, where applicable;
- diagnostic mapping;
- test locations;
- compatibility classification;
- hard-coding audit status;
- last verified conformance state.

Where separate registries are useful, such as ownership, dependencies, exports, feature status, diagnostics, or compatibility, they MUST have a single documented source of truth and deterministic consistency checks.

The repository MUST NOT allow a file to disappear from the inventory while remaining an active grammar dependency.

15.1 Inventory accuracy

The manifest MUST reflect the actual repository state, not an aspirational directory tree.

Missing files, renamed files, malformed paths, orphaned grammars, undocumented exports, duplicate owners, and stale dependency entries MUST be reported as validation failures or explicitly recorded migration items.

No file count or completion percentage may be treated as evidence of production readiness without the associated audit results.

---

16. Diagnostics and conformance reporting

Every public syntax construct and every registered grammar module MUST have defined error behavior.

Where a construct can be recognized syntactically but rejected semantically, the specification MUST distinguish syntax errors from structural, type, effect, capability, resource, policy, compatibility, and realization errors.

Diagnostics SHOULD include:

- a stable diagnostic identifier;
- severity;
- source span;
- concise description;
- relevant context;
- an actionable correction where feasible;
- the validation stage responsible;
- stable or versioned compatibility behavior.

The parser MUST NOT pretend that syntactic acceptance proves semantic validity.

The implementation-conformance reference MUST report the distinction between specified, parsed, represented in the AST, semantically validated, lowered to IR, supported by a backend, tested, and stable.

---

17. Compatibility and language evolution

Any change to a public token, grammar rule, AST mapping, semantic contract, IR mapping, dialect interface, or diagnostic behavior MUST be evaluated for compatibility impact.

The responsible change MUST identify:

- affected language versions;
- whether existing source remains valid;
- whether program meaning changes;
- whether AST or IR consumers are affected;
- whether migration is required;
- whether a feature is introduced, modified, deprecated, or removed;
- which conformance tests must change;
- which portable artifacts may be affected.

A breaking change MUST NOT be introduced by quietly editing a subordinate grammar file.

Compatibility decisions belong to "language-version.md" and "compatibility.md", with machine-readable records where required.

Generated grammar files, token IDs, parser artifacts, or IR schemas MUST NOT be treated as stable merely because their current numeric identifiers or layouts happen to remain unchanged.

---

18. Required validation gates

Before a file or group of files is frozen, automated checks MUST verify all applicable items below.

18.1 Authority and ownership

- [ ] The file has one designated owner.
- [ ] Its authority level is declared.
- [ ] Its purpose and non-ownership boundaries are explicit.
- [ ] Public exports have unique owners.
- [ ] Its consumers are recorded.
- [ ] No competing authority defines the same contract.

18.2 Dependency integrity

- [ ] All declared imports and dependencies exist.
- [ ] All dependency paths are canonical.
- [ ] The dependency graph is valid.
- [ ] There are no unapproved cycles.
- [ ] There are no orphaned public rules or unregistered consumers.
- [ ] The manifest agrees with the file.

18.3 Lexical and grammar correctness

- [ ] Tokens have unique definitions and documented spellings.
- [ ] The lexical conformance corpus passes.
- [ ] Grammar generation succeeds with the supported toolchain.
- [ ] Rule references resolve.
- [ ] Unreachable rules and unintended recursion are detected.
- [ ] Ambiguities and precedence rules are reviewed.
- [ ] Error recovery and invalid-input behavior are tested.
- [ ] Source locations remain available to downstream stages.

18.4 Semantic integration

- [ ] AST mappings are complete or explicitly inapplicable.
- [ ] Semantic mappings are complete or explicitly inapplicable.
- [ ] Type, effect, capability, resource, contract, policy, and provenance integration is documented where relevant.
- [ ] IR mappings are complete or explicitly inapplicable.
- [ ] Unsupported features fail clearly rather than receiving guessed meaning.
- [ ] Diagnostics have stable identifiers or an explicit versioning plan.

18.5 Portability and scalability

- [ ] The hard-coding audit passes.
- [ ] No arbitrary physical capacity has become a universal grammar limit.
- [ ] Resource requirements remain distinct from actual resource availability.
- [ ] Target-specific realization remains outside the universal syntax authority.
- [ ] Fallback and approximation behavior is explicit.
- [ ] Capability or resource failure behavior is tested.
- [ ] Large inputs and parameterized constructs are tested within practical test budgets.
- [ ] Implementation limits are documented and not misrepresented as language-wide limits.

18.6 Rust safety and implementation conformance

- [ ] The supported Rust baseline is respected.
- [ ] Production Rust code uses no "unsafe".
- [ ] The relevant Rust frontend mappings exist.
- [ ] Error paths are tested.
- [ ] No undocumented truncation, unchecked input-size assumptions, or silent semantic substitutions are introduced.
- [ ] The implementation's actual support status is recorded accurately.

18.7 Compatibility and documentation

- [ ] Language and feature status is registered.
- [ ] Compatibility impact is recorded.
- [ ] Required documentation links are correct.
- [ ] Examples conform to the relevant specification status.
- [ ] Generated artifacts are distinguished from authoritative sources.
- [ ] The change has a reviewable conformance record.

A gate MUST fail if a REQUIRED condition is unmet. A known limitation MAY be accepted only when the relevant authority explicitly permits it, its impact is documented, and the file is not falsely labeled fully production-ready.

---

19. Freeze rules

19.1 A file cannot freeze ahead of its interfaces

A file MUST NOT be frozen until the contracts for the interfaces it consumes are stable enough to validate it.

This does not require every downstream implementation to be finished. It requires the interfaces and expected behavior to be defined.

19.2 Freeze means contract stability, not immutability

A frozen file is stable with respect to its declared contract. It is not immune to legitimate language evolution, defect correction, security correction, or compatibility-preserving maintenance.

Changes to a frozen file MUST follow the approved change process and rerun all affected gates.

19.3 Downstream implementation must not cause accidental rework

If a backend or runtime is added later, its integration MUST conform to the frozen interface.

If integration reveals that the interface is insufficient, the project MUST document the deficiency and process a deliberate contract change. It MUST NOT hide the issue through an undocumented edit to a downstream or unrelated file.

19.4 Required freeze record

Every frozen file MUST have a verifiable record of:

- the exact path and version or revision;
- owner and authority;
- dependency and export contracts;
- AST, semantic, and IR mappings as applicable;
- diagnostic and compatibility obligations;
- test commands and results;
- hard-coding and portability audit results;
- known limitations;
- approval and freeze status.

The word "STABLE" MUST NOT be assigned solely because a file's author considers the work complete.

---

20. Recommended dependency-ordered freeze sequence

The precise order MUST follow the verified repository dependency graph. The following sequence is the default plan, not a substitute for inspecting actual dependencies.

Phase 0 — Inventory and audit

Establish the canonical manifest, ownership registry, dependency registry, feature-status registry, and validation procedure.

Identify duplicate rules, duplicate token ownership, missing dependencies, malformed paths, conflicting authorities, unregistered features, and stale references.

Do not declare the grammar frozen during this phase.

Phase 1 — Architecture and language authority

Complete and reconcile:

- "grammar/DESIGN.md"
- "grammar/specification/grammar-authority.md"
- "grammar/specification/language.md"
- "grammar/specification/language-principles.md"
- "grammar/specification/language-scope.md"
- "grammar/specification/language-version.md"
- "grammar/specification/poco-reaf.md"
- "grammar/specification/scalability-model.md"
- "grammar/specification/portability.md"
- "grammar/specification/compatibility.md"

Validate the authority hierarchy and ensure that overlapping specifications delegate to each other rather than defining competing contracts.

Phase 2 — Lexical foundation

Freeze the token registry, lexical rules, literal contracts, public lexer boundary, and shared lexical conformance tests.

Validate agreement between the ANTLR lexical system and the Rust lexer.

Phase 3 — Core syntax and types

Freeze core names, identifiers, source units, modules, declarations, visibility, attributes, type definitions, generic constraints, and the common type model in dependency order.

Phase 4 — Expressions, statements, and functions

Freeze the canonical expression and statement dispatchers, operator precedence, function model, control flow, patterns, and their AST mappings.

Resolve duplicate-looking rule names and files before freezing their exports.

Phase 5 — Shared semantic contracts

Freeze effects, capabilities, resources, contracts, policies, security boundaries, provenance, and diagnostics.

These contracts must be stable before dependent computational domains are frozen.

Phase 6 — Classical, data, concurrency, and distribution

Freeze the relevant common-domain grammars and integration contracts against the shared semantic foundation.

Phase 7 — Hardware, execution, and compilation

Freeze target intent, abstract resource descriptions, execution requirements, compilation intent, and the interfaces for capability negotiation and target realization.

Phase 8 — Quantum and HDL

Freeze the quantum and HDL syntax and semantic mappings against the shared types, effects, resources, policies, execution contracts, and designated IR boundaries.

Phase 9 — Hybrid and other advanced domains

Freeze cross-domain integration only after its underlying domain contracts are stable.

Phase 10 — Dialects, interoperability, macros, and metaprogramming

Freeze external-format boundaries, dialect registration, macro expansion, reflection, code generation, and their safety, policy, and compatibility contracts.

Phase 11 — Compatibility and end-to-end conformance

Validate language evolution, artifact compatibility, migration behavior, conformance examples, and the full dependency closure.

Phase 12 — Root composition and directory freeze

Freeze the reconciled parser root, public parser boundary, grammar build configuration, and final composition tests.

A root composition file MUST NOT be frozen while its subordinate grammar contracts remain unresolved.

---

21. Change-control process

Any change affecting a public grammar or specification contract MUST include:

1. The purpose and rationale.
2. The authoritative file being changed.
3. The exact symbols, rules, and interfaces affected.
4. The dependency closure that must be revalidated.
5. AST, semantic, and IR mapping changes where applicable.
6. Diagnostics and negative-test changes.
7. Scalability, portability, and hard-coding audit results.
8. Compatibility classification and migration requirements.
9. Updated conformance evidence.
10. Manifest and status updates.

A change MUST NOT be approved solely because the grammar generator succeeds.

For a change to an existing stable construct, the reviewer MUST determine whether the intended effect is compatible with the existing language meaning. If it is not, the change MUST follow the documented breaking-change policy.

The project SHOULD automate as many of these checks as practical so that ownership, dependency, token, and compatibility errors are caught before review or release.

---

22. Definition of production-ready grammar authority

This specification and the surrounding grammar authority model are ready for freeze only when all of the following are true:

- The authority hierarchy is implemented consistently across the repository.
- The parser-root relationship is unambiguous and verified by the actual build.
- The lexical authority and token registry are singular and validated.
- The manifest accurately lists the real repository files and dependencies.
- Every public rule and token has one owner.
- All active grammar dependencies resolve.
- Every applicable feature has a declared AST, semantic, and IR integration contract.
- Machine-oriented contracts agree with the normative language specification.
- Implementation-conformance status is distinguishable from specification status.
- Scalability and portability rules are testable and enforced.
- No universal grammar imposes arbitrary physical resource ceilings.
- Rust implementation policy explicitly requires Rust 1.97 or later and prohibits "unsafe".
- Diagnostics, compatibility, and change-control requirements are documented.
- The required positive, negative, integration, and portability tests pass.
- Freeze records exist for the files being declared stable.

Until these conditions are demonstrated, the correct status is in progress, partially conformant, or another accurate registered status—not production-ready.

---

23. Required consistency with related files

This specification requires the following integration relationships:

File or area| Contract relationship
"grammar/DESIGN.md"| Architecture, ownership, dependency, and freeze rules
"grammar/README.md"| Navigation and explanation of authority
"grammar/specification/language.md"| Overall normative language model
"grammar/specification/language-principles.md"| Language-wide invariants
"grammar/specification/language-scope.md"| Included domains and scope boundaries
"grammar/specification/language-version.md"| Language and feature versioning
"grammar/specification/poco-reaf.md"| Portable artifact and target-realization guarantees
"grammar/specification/scalability-model.md"| Resource-parametric scaling rules
"grammar/specification/portability.md"| Target independence and portability behavior
"grammar/specification/compatibility.md"| Compatibility and migration rules
"grammar/spec/"| Machine-oriented contracts consistent with normative requirements
"grammar/Zamani.g4"| Public ANTLR root composition
"grammar/antlr/ZamaniLexer.g4"| Public lexer boundary
"grammar/antlr/ZamaniParser.g4"| Canonical parser composition, reconciled with the root
"grammar/lexer/"| Token ownership and lexical contracts
"grammar/core/", "types/", "expressions/", "statements/"| Common syntax and AST integration
"grammar/effects/", "resources/", "policies/", "security/", "validation/"| Shared semantic constraints and validation
"grammar/classical/", "quantum/", "hdl/", "hybrid/"| Domain-specific syntax and semantic mappings
"grammar/hardware/", "execution/", "compile/"| Target intent, realization interfaces, and compilation contracts
"grammar/dialects/", "interoperability/"| Extension and external-format boundaries
"grammar/compatibility/"| Feature and artifact compatibility
"grammar/tests/" and registered conformance suites| Evidence that contracts are satisfied
"src/lexer.rs", "src/parser.rs", "src/ast/"| Rust frontend conformance
Semantic-analysis modules| Validated language meaning
Classical IR and "quantum::ir"| Canonical domain representation boundaries
Compiler, runtime, and backend modules| Target realization and execution contracts

The repository MUST verify actual paths and symbols before treating this table as an implementation inventory. A relationship listed here is an integration obligation, not proof that the corresponding implementation is complete.

---

24. Final invariant

The governing invariant of Zamani grammar authority is:

«Every source-language construct has one normative meaning, one designated owner for each public contract, one traceable path through parsing and semantic validation, and an explicit integration boundary to the compiler and runtime. No target-specific limitation may silently become a universal language restriction, and no implementation detail may silently override the normative language specification.»

The grammar subsystem must define a stable, versioned, extensible source language whose meaning is independent of a particular machine wherever the program's semantics allow it.

The implementation must then establish that meaning through the AST and semantic model, map it to the designated canonical representations, and realize it using the capabilities and resources actually available.

That is the authority model required to support POCO-REAF, scalable classical and quantum computing, HDL, hybrid systems, and future computational domains without turning the language definition into a collection of competing grammars.

Freeze decision: This file may be marked "STABLE" only after its requirements are reconciled with the actual repository manifest, architecture document, language specifications, parser build, implementation-conformance reference, and automated validation gates. The existence of this specification alone does not certify the entire "grammar/" directory as frozen.