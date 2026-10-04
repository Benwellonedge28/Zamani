Zamani Grammar Architecture and Production Design

Path: "grammar/DESIGN.md"
Repository: "Benwellonedge28/Zamani"
Language: Zamani
Compiler: ZUTC / Zamani compiler
Edition: Rust 2021
Minimum Rust version: Rust 1.97.1
Rust safety: Safe Rust only; production Rust implementation MUST NOT use "unsafe"
Architecture: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Status: Normative architectural specification
Scope: Grammar, lexical contracts, frontend integration, semantic ownership, domain integration, compatibility, scalability and conformance.

---

1. Purpose

This document establishes the normative architecture for the Zamani "grammar/" subsystem.

It defines how Zamani source-language syntax is specified, implemented, validated, integrated and maintained across the repository.

Its purpose is to ensure that Zamani can express classical, quantum, hybrid, hardware-description, AI, distributed, parallel, scientific, data, networking and future computational paradigms through one coherent language.

The architecture must support programs ranging from the smallest meaningful computation to arbitrarily large computations, subject to actual semantic requirements, implementation representation limits and available resources.

This document also integrates useful UBUNTU concepts into Zamani without introducing a second language, competing grammar authority or application-specific keyword explosion.

It is an architectural contract. It does not certify that every described feature is already implemented.

A design requirement is not an implementation claim.

A feature may only be called production-ready after its complete implementation and conformance requirements have been demonstrated.

2. Normative terminology

The words below have precise meanings throughout this document.

- MUST / MUST NOT: Mandatory architectural requirement.
- SHOULD / SHOULD NOT: Strong recommendation, with documented justification required for deviations.
- MAY: Optional implementation or extension.
- Specified: A normative design exists.
- Implemented: The required implementation exists and is integrated.
- Conformant: The implementation passes the applicable conformance suite.
- Stable: The feature has an approved compatibility contract.
- Experimental: The feature is explicitly non-stable.
- Deprecated: The feature remains recognizable but has an announced replacement or removal policy.
- Unsupported: The implementation cannot currently provide the feature.
- Target: A compilation or execution environment.
- Resource: A logical or physical quantity consumed or required by computation.
- Capability: An operation or property supported by an execution environment.
- Realization: The concrete implementation of logical program intent on a target.

These statuses MUST NOT be used interchangeably.

3. Existing repository baseline

This design is based on the existing repository structure and implementation, not an assumption that the repository is an empty language project.

The repository already contains:

- "grammar/Zamani.g4"
- "grammar/Zamani-Grammar.md"
- "grammar/grammar.md"
- "grammar/README.md"
- "grammar/DESIGN.md"
- multiple domain-specific grammar directories;
- "src/lexer.rs";
- "src/parser.rs";
- "src/ast/mod.rs";
- quantum-related compiler components;
- existing concurrency, effects, resource, execution and interoperability work.

The Rust frontend currently contains constructs and representations that are not necessarily identical to the ANTLR grammar.

For example, the existing AST includes dedicated quantum, nano, learning, concurrency, effect and other language constructs. The parser is a recursive-descent/Pratt implementation. The lexer contains its own token definitions and keyword recognition.

Consequently, no grammar document may assume that adding an ANTLR production automatically changes the behavior of the Rust compiler.

3.1 Repository facts that must be reconciled

The repository's "Cargo.toml" currently declares Rust 2021 and contains a "rust-version" entry that must be validated and normalized to a valid Cargo version string.

The required baseline for this architecture is:

edition = "2021"
rust-version = "1.97.1"

The actual build must be tested using that toolchain.

The existing AST also contains an "Unsafe" language node. This design distinguishes:

1. Rust implementation safety.
2. Zamani source-language unsafe operations.

Production Rust code MUST NOT use Rust "unsafe".

The presence of a Zamani "unsafe" syntax node does not authorize unsafe Rust implementation. The source-language feature must independently pass the security and semantics requirements defined below.

3.2 Repository inspection and change policy

Before changing an existing grammar or frontend file, developers MUST inspect:

- its current contents;
- its direct imports;
- its public types and rules;
- its downstream consumers;
- associated tests;
- relevant semantic and IR implementations;
- compatibility requirements.

A proposed architecture MUST NOT silently replace existing files with assumed versions.

Existing major filenames MUST be preserved unless an approved migration establishes otherwise.

4. Fundamental language objective

Zamani is intended to provide one coherent programming language for:

- classical computation;
- systems programming;
- embedded computation;
- scientific and numerical computation;
- symbolic computation;
- AI and machine learning;
- tensor and data computation;
- parallel and concurrent computation;
- distributed and high-performance computing;
- quantum computation;
- hybrid quantum-classical computation;
- quantum error correction;
- hardware description;
- hardware/software co-design;
- accelerators;
- networking;
- cryptography;
- security;
- edge and cloud computation;
- temporal computation;
- metaprogramming;
- interoperability;
- domain-specific extensions;
- future computational paradigms.

These are domains of one language, not separate languages that happen to share a filename.

They MUST share common foundations for:

- identifiers and names;
- expressions;
- statements;
- declarations;
- types;
- source locations;
- diagnostics;
- modules;
- effects;
- capabilities;
- resources;
- constraints;
- contracts;
- policies;
- provenance;
- semantic validation;
- compatibility.

Domain-specific meaning MUST be introduced through explicit semantic contracts and domain-owned extensions.

5. POCO-REAF

5.1 Definition

POCO-REAF means:

Program Once, Compile Once, Run Everywhere, Anywhere, Forever.

A Zamani program MUST express stable program meaning independently of a particular physical machine.

The same source-level program SHOULD remain usable across different target architectures, scales and future execution environments without rewriting its algorithm merely because hardware changes.

POCO-REAF is a portability architecture, not a guarantee that every target can execute every program.

The following are different conditions:

Lexically valid
    ≠
Syntactically valid
    ≠
Structurally valid
    ≠
Semantically valid
    ≠
Compilable
    ≠
Target compatible
    ≠
Resource feasible
    ≠
Runtime available
    ≠
Successfully executed

A program may be valid even when a particular machine cannot currently execute it.

The compiler MUST distinguish these conditions and report the appropriate result.

5.2 Compilation-once interpretation

Compilation-once MUST NOT be interpreted as requiring one binary to execute natively on every possible architecture.

The architecture distinguishes:

- source portability;
- semantic portability;
- canonical IR portability;
- target-specific lowering;
- executable portability;
- runtime portability;
- reproducibility.

A compilation artifact may contain target-independent information and target-specific realizations.

A target-specific binary cannot be assumed to execute on a different architecture.

The compiler MUST preserve source meaning when producing different target realizations.

5.3 Feasibility

If a target lacks a required capability or resource, the compiler/runtime MUST:

- report the unsatisfied requirement;
- identify the relevant target capability or resource;
- explain whether a permitted alternative exists;
- preserve program meaning;
- avoid silently weakening guarantees.

Possible permitted alternatives include simulation, decomposition, routing, distribution, scheduling, recovery and other explicitly authorized realizations.

Fallbacks MUST be explicit in semantic policy or compilation configuration.

6. Scalability and resource independence

6.1 General principle

Zamani MUST NOT impose arbitrary universal hardware ceilings through grammar productions, AST representations, semantic constants or compiler architecture.

The intended scale includes:

one value
one operation
one function
one logical qubit
one processor
one accelerator
one device
        |
        v
large programs
large datasets
large tensors
large quantum computations
large heterogeneous systems
large distributed systems
future computational environments

Actual feasibility is determined by semantics, available resources and explicitly declared policies.

6.2 Prohibited universal limits

The language architecture MUST NOT establish universal fixed ceilings such as:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_TIMELINES
MAX_AGENTS
MAX_DEVICES
MAX_ACCELERATORS
MAX_NETWORK_SIZE
MAX_GATE_COUNT

The same prohibition applies to equivalent renamed constants.

The grammar MUST NOT enumerate a fixed physical universe of resources as its universal model.

6.3 Finite representation is not an artificial language ceiling

All real implementations have finite representations and finite execution resources.

Zamani MUST acknowledge these facts rather than claiming literal mathematical infinity.

The following distinctions are mandatory:

Concept| Required interpretation
Language expressiveness| No arbitrary universal hardware ceiling
Integer representation| Explicitly specified and checked
Collection size| Determined by type, representation and available resources
Compiler memory| Governed by implementation and configurable resource policy
Physical qubits| Determined by target availability
Tensor dimensions| Represented according to type and implementation capabilities
Network size| Determined by declared semantics and available infrastructure
Runtime allocation| Checked against actual available resources

An implementation limit MUST be documented, observable and distinguishable from a language restriction.

When a representation limit is reached, the compiler MUST produce a diagnostic rather than overflow, truncate, wrap unexpectedly or silently corrupt program meaning.

6.4 Numeric values

Ordinary program constants remain valid.

For example:

let n = 1024;

is valid program data.

A particular matrix dimension may also be specified in a program.

What is prohibited is making such a value the universal maximum supported by Zamani.

7. Separation of responsibilities

The central rule is:

Source describes:
    meaning
    computation
    types
    correctness
    effects
    requirements
    constraints
    capabilities
    policies
    preferences

Compiler/runtime determines:
    target
    placement
    scheduling
    routing
    decomposition
    implementation
    physical resources
    native instructions
    execution strategy

Grammar MUST NOT own physical hardware realization.

The grammar describes the program's logical intent.

The compiler and runtime realize that intent where possible.

8. Universal resource model

The following concepts MUST remain distinct.

8.1 Requirement

A requirement is necessary for the program's declared semantics.

Conceptual form:

requires qubits >= required_qubits;

8.2 Capability

A capability describes an operation or property supported by a target.

requires capability("quantum.measurement");
requires capability("tensor.compute");

Capability names MUST be extensible and registered through the appropriate semantic or capability registry.

8.3 Constraint

A constraint restricts an acceptable realization.

requires latency <= latency_budget;

A constraint MUST have defined units, comparison semantics and diagnostic behavior.

8.4 Preference

A preference guides target selection without becoming a mandatory requirement.

8.5 Hint

A hint supplies optimization information without changing program meaning.

8.6 Budget

A budget expresses a permitted resource expenditure or bound.

Budgets MUST NOT become universal machine capacities.

8.7 Negotiation

Negotiation determines whether the requirements and policies can be satisfied by a candidate environment.

8.8 Realization

Physical mapping, resource allocation, topology selection and device placement belong downstream.

The logical program MUST NOT be forced to contain physical resource identifiers unless the programmer explicitly requests target-specific behavior.

9. Repository authority model

There MUST be one canonical Zamani language.

The repository may maintain multiple representations of that language, but they MUST have explicitly separated responsibilities.

9.1 "grammar/DESIGN.md"

This document.

Owns:

- architecture;
- ownership boundaries;
- integration contracts;
- invariants;
- scalability;
- compatibility principles;
- conformance requirements;
- production acceptance.

Does not own every grammar production or implementation detail.

9.2 "grammar/README.md"

Owns navigation and entry-point documentation.

It MUST identify authoritative specifications, implementation references and conformance resources.

It MUST NOT introduce independent language semantics.

9.3 "grammar/Zamani.g4"

Owns the canonical ANTLR grammar composition.

It MUST:

- remain the stable root grammar entry point;
- define the accepted composition boundary;
- include the appropriate root program rule and EOF;
- delegate domain syntax to its designated grammar owners;
- avoid duplicate lexical or semantic definitions;
- preserve compatibility deliberately.

It MUST NOT claim that ANTLR is the sole implementation authority while the Rust lexer/parser independently accept different syntax.

9.4 "grammar/Zamani-Grammar.md"

Owns historical, extended and explanatory grammar material.

Every feature MUST be classified as one of:

- stable;
- proposed;
- experimental;
- deprecated;
- historical;
- not implemented.

Presence in this document does not automatically authorize syntax.

9.5 "grammar/grammar.md"

Owns implementation conformance documentation.

It MUST describe the behavior actually implemented by the reference compiler.

Each feature MUST distinguish applicable stages:

SPECIFIED
LEXER_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
STRUCTURAL_VALIDATION_IMPLEMENTED
SEMANTIC_IMPLEMENTED
TYPE_IMPLEMENTED
EFFECT_IMPLEMENTED
RESOURCE_IMPLEMENTED
IR_IMPLEMENTED
BACKEND_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL
DEPRECATED

A status MUST be supported by implementation evidence.

9.6 "grammar/specification/"

Owns normative human-readable language specifications.

9.7 "grammar/spec/"

Owns machine-checkable contracts, schemas and validation metadata.

The two directories MUST NOT silently contradict one another.

9.8 Rust implementation

The Rust implementation owns executable behavior.

The lexer, parser, AST, semantic model and IR MUST agree with normative language specifications.

A specification change does not automatically imply an implementation change.

10. Canonical compiler pipeline

The production architecture is:

                  Zamani Source
                        |
                        v
                Source Management
                        |
                        v
                      Lexer
                        |
                        v
                   Token Stream
                        |
                        v
                      Parser
                        |
                        v
                  Domain-neutral AST
                        |
                        v
             Structural Validation
                        |
                        v
                Name/Module Resolution
                        |
             +----------+----------+
             |          |          |
             v          v          v
           Types      Effects    Contracts
             |          |          |
             +----------+----------+
                        |
                        v
          Resources / Capabilities / Policies
                        |
                        v
                 Semantic Analysis
                        |
                        v
             Canonical Semantic Model
                        |
            +-----------+-----------+
            |           |           |
            v           v           v
        Classical    Quantum       HDL/
        Semantics    Semantics   Hardware Intent
            |           |           |
            +-----------+-----------+
                        |
                        v
                 Canonical IR
                        |
                        v
                 Domain IRs
                        |
                        v
                 Optimization
                        |
                        v
                   Lowering
                        |
                        v
              Routing / Scheduling
                        |
                        v
              Resilience / Recovery
                        |
                        v
                    ZQN / HAL
                        |
                        v
                Target Realization

This is a conceptual ownership and data-flow architecture.

It does not require every domain to execute every stage or require all stages to be implemented inside "grammar/".

11. Frontend integration

11.1 "src/lexer.rs"

Owns tokenization in the current Rust implementation.

It MUST:

- recognize the specified token vocabulary;
- preserve source spans;
- distinguish identifiers from reserved keywords;
- handle lexical errors deterministically;
- support the specified Unicode policy;
- avoid silently accepting malformed literals;
- maintain compatibility according to the language version;
- expose stable token contracts to the parser.

The ANTLR lexer MUST be reconciled with this implementation.

Neither implementation may independently invent accepted syntax.

11.2 "src/parser.rs"

Owns current Rust parsing behavior.

It MUST:

- implement the normative syntax;
- maintain defined precedence and associativity;
- produce precise diagnostics;
- preserve source spans;
- recover from errors without corrupting subsequent parsing;
- reject invalid structures;
- avoid treating unsupported syntax as implemented.

The ANTLR grammar and Rust parser MUST be checked against shared conformance cases.

11.3 "src/ast/mod.rs"

Owns the current frontend AST representation.

The AST MUST represent source-level meaning without embedding target-specific implementation decisions.

It MUST NOT require:

- LLVM-specific objects;
- QIR-specific objects;
- vendor-specific physical qubit identifiers;
- calibrated physical topology;
- target-specific routing;
- hardware-specific instruction encodings;
- runtime resource handles.

Source spans and necessary semantic distinctions MUST be preserved.

11.4 AST evolution

Every AST change MUST document:

- the new or modified node;
- construction sites;
- parser integration;
- semantic consumers;
- serialization implications;
- compatibility implications;
- tests;
- downstream migration requirements.

An AST node MUST NOT be introduced merely to make a grammar production parse.

12. Canonical IR boundaries

The frontend AST and compiler IR have different responsibilities.

The AST represents source structure and source-level distinctions.

The canonical semantic representation captures validated program meaning.

Domain IRs represent domain-specific computation.

12.1 Quantum IR

"quantum::ir" is the canonical quantum IR authority.

Zamani MUST NOT create a competing quantum IR merely because a new frontend or dialect is introduced.

All quantum source formats and quantum-language integrations MUST converge on the canonical quantum semantic boundary.

The intended flow is:

Zamani source
     |
     v
Frontend AST
     |
     v
Quantum semantic validation
     |
     v
quantum::ir
     |
     v
Optimization
     |
     v
Decomposition
     |
     v
Routing
     |
     v
Scheduling
     |
     v
QEC / resilience
     |
     v
ZQN
     |
     v
HAL
     |
     v
QPU / simulator / future target

The exact implementation stages belong to the relevant compiler subsystems.

The grammar MUST NOT define physical quantum limits.

12.2 Classical IR

Classical computation MUST have a corresponding target-independent semantic boundary.

12.3 HDL boundary

HDL intent MUST be represented without confusing logical hardware declarations with physical synthesis results.

12.4 Hybrid boundary

Hybrid programs MUST preserve the relationships between classical control, quantum computation, measurement outcomes and subsequent classical decisions.

12.5 Cross-domain operations

Cross-domain constructs MUST have explicit semantic ownership.

The frontend MUST NOT silently duplicate equivalent operations in different domains.

13. Universal syntax architecture

The grammar SHOULD be organized around existing repository directories.

Existing directories MUST be populated and consolidated before creating parallel alternatives.

The principal owners are:

Directory| Responsibility
"core/"| Universal program structure, names and shared syntax
"lexer/"| Token contracts and lexical specifications
"antlr/"| ANTLR composition and generated grammar integration
"types/"| Type syntax
"expressions/"| Expression syntax
"statements/"| Statement syntax
"declarations/"| Declaration syntax
"functions/"| Functions, parameters and callable declarations
"modules/"| Modules and imports
"effects/"| Effect declarations and effect-related syntax
"resources/"| Resource requirements and capabilities
"validation/"| Contracts and validation syntax
"policies/"| Universal policy syntax and contracts
"provenance/"| Provenance syntax and metadata
"execution/"| Execution intent and modes
"concurrency/"| Tasks, actors, synchronization and messages
"classical/"| Classical domain syntax
"quantum/"| Quantum domain syntax
"hybrid/"| Cross-domain computation
"hdl/"| Hardware description
"hardware/"| Hardware intent and resource descriptions
"ai/"| AI, learning, knowledge and reasoning
"data/"| Data and query constructs
"distributed/"| Distributed computation
"networking/"| Network constructs
"security/"| Security-related constructs
"interoperability/"| Foreign interfaces and external formats
"dialects/"| Explicit language extensions
"metaprogramming/"| Compile-time and reflective facilities
"compile/"| Compilation intent and compilation metadata
"compatibility/"| Versioning and migration
"specification/"| Normative human specifications
"spec/"| Machine-checkable contracts
"tests/"| Conformance and regression tests

A directory name does not automatically grant a file ownership over syntax, AST or semantics.

Every file MUST have an explicit ownership contract.

14. UBUNTU integration

UBUNTU contributes useful language concepts, but its complete grammar MUST NOT be copied into Zamani.

The following concepts are approved for architectural integration, subject to their individual specifications and implementation status.

14.1 Reasoning

Support a unified reasoning model capable of representing:

- inference;
- deduction;
- induction;
- abduction;
- premises;
- evidence;
- conclusions;
- derivations;
- uncertainty.

"infer", "deduce" and "reason" MUST converge on a coherent semantic model.

They MUST NOT create independent reasoning languages.

14.2 Knowledge

Support:

- knowledge assertions;
- retraction;
- querying;
- relations;
- evidence;
- metadata;
- provenance.

Knowledge MUST be usable by AI and non-AI programs.

14.3 Learning

Learning MUST be a semantic operation with explicit inputs, outputs, objectives, effects, capabilities, resources and provenance.

The core grammar MUST NOT enumerate every learning algorithm.

14.4 Adaptation

Adaptation MUST be controlled.

It MUST NOT imply unrestricted self-modifying code.

An adaptation operation MUST be subject to:

- authorization;
- policy;
- declared effects;
- capability checks;
- resource constraints;
- validation;
- provenance.

14.5 Uncertainty

Support semantic representations for:

- uncertainty;
- probability;
- distributions;
- confidence;
- beliefs.

The language MUST NOT hard-code a particular probabilistic implementation.

14.6 Explainability

Explanations MUST be applicable to:

- reasoning;
- AI decisions;
- compiler transformations;
- resource allocation;
- optimization;
- quantum routing;
- hardware placement;
- security decisions.

Explainability MUST NOT be limited to an AI-only subsystem.

14.7 Evidence and provenance

A shared evidence/provenance model MUST support:

claim
evidence
source
derivation
verification
decision
transformation
provenance

The model MUST support traceability without requiring every execution to retain unlimited historical information.

Retention and detail are governed by explicit policies.

14.8 Agents

AI agents MUST integrate with the existing concurrency and actor architecture.

There MUST NOT be a competing actor runtime.

The AI domain owns agent-specific meaning.

The concurrency domain owns actor lifecycle, scheduling and message semantics.

14.9 Neural-symbolic computation

Neural and symbolic computation MUST compose through common types, operations, effects, resources and semantic interfaces.

They MUST NOT become disconnected execution systems.

14.10 Contracts

Support a coherent contract model for:

- requirements;
- preconditions;
- postconditions;
- invariants;
- assumptions;
- guarantees;
- properties;
- evidence.

Contracts MUST have defined evaluation and failure semantics.

14.11 Policies and sandboxing

Policies MUST express permissions, prohibitions, constraints, preferences and authorized fallbacks.

Sandboxing MUST integrate with effects, capabilities, resources, foreign calls, reflection and adaptation.

14.12 Simulation

Simulation is an execution strategy, not a separate language.

It may support classical, quantum, hybrid, hardware, distributed, AI and fault simulation.

The simulator MUST preserve the specified semantics or explicitly document its approximation.

14.13 Interoperability

FFI, ABI, foreign types, calling conventions and external data formats belong in interoperability specifications.

Foreign calls MUST participate in effect and capability analysis.

14.14 Excluded application-specific syntax

The following MUST NOT be added as universal core keywords merely because they appeared in UBUNTU:

- payment operations;
- administrative actions;
- user blocking;
- legal actions;
- copyright enforcement;
- sentiment-specific operations;
- computer-vision-specific operations;
- robotics-specific operations;
- VR/AR-specific operations;
- blockchain-specific operations;
- application-specific AGI/ASI labels;
- omnipotence or other non-computational claims.

These MAY be implemented through libraries, dialects, services, policies and application-level APIs.

15. Lexical architecture

15.1 One canonical token contract

The language MUST have one canonical lexical contract.

The Rust lexer, ANTLR lexer, syntax highlighters and generated references MUST derive from or be validated against that contract.

A token registry is metadata, not an independent runtime lexer.

15.2 Keyword policy

A word MUST become a reserved keyword only when the language specification requires dedicated syntax or a defined compatibility commitment.

Otherwise it SHOULD remain an identifier.

The UBUNTU concepts "infer", "deduce", "reason", "learn", "adapt", "retract", "query", "explain", "evidence", "provenance" and "policy" MUST be evaluated individually.

They MUST NOT all be reserved automatically.

15.3 Duplicate token definitions

Equivalent token names MUST be reconciled.

For example, "Question" and "QuestionMark", or "Ampersand" and "BitAnd", MUST NOT remain ambiguous duplicate representations unless their distinct meanings are specified and tested.

15.4 Literal handling

Numeric, string, character, Boolean, quantum, annotation, temporal and future literals MUST have explicit lexical contracts.

A documented literal that the Rust lexer does not recognize MUST be reported as a conformance gap.

No grammar document may mark it implemented merely because an ANTLR production exists.

15.5 Unicode

The language MUST specify:

- identifier normalization policy;
- valid Unicode categories;
- escape behavior;
- source encoding;
- invalid-sequence diagnostics;
- source-span accounting.

The lexer MUST preserve correct source locations.

16. Grammar composition

The root grammar MUST be thin.

The intended composition is:

Zamani.g4
    |
    +-- lexical contract
    |
    +-- program and item dispatch
    |
    +-- declarations
    |
    +-- statements
    |
    +-- expressions
    |
    +-- types
    |
    +-- domain dispatch
         |
         +-- classical
         +-- quantum
         +-- hybrid
         +-- HDL
         +-- AI
         +-- data
         +-- distributed
         +-- other domains

Domain grammar files MUST import shared syntax instead of copying it.

The grammar MUST avoid circular imports.

Every public parser rule MUST have a documented owner.

16.1 Generic operations

Universal grammar MUST represent extensible operations through generic operation specifications where appropriate.

It MUST NOT enumerate every possible mathematical, AI, quantum or vendor operation.

Quantum operations MUST use an extensible operation representation with parameters, operands, results, attributes and modifiers.

New operations MUST be addable through the appropriate semantic registry or extension mechanism without changing universal grammar for every new operation.

16.2 ANTLR and Rust parser agreement

ANTLR and the Rust parser MUST be tested against common valid and invalid source cases.

If the implementations intentionally support different subsets during migration, the difference MUST be explicit and versioned.

There MUST NOT be undocumented syntax that parses in one implementation but has a different meaning or is rejected in another.

17. Type-system architecture

The type system MUST be coherent across all domains.

It SHOULD support, according to staged specifications:

- primitive types;
- composite types;
- generic types;
- functions;
- tuples;
- records;
- sums;
- options;
- results;
- arrays;
- slices;
- maps;
- references;
- ownership;
- linear types;
- affine types;
- constraints;
- associated types;
- type classes or equivalent abstractions;
- refinement types;
- dependent types where implementation and decidability requirements are established;
- uncertainty-aware types;
- domain-specific types.

Potential types such as:

Uncertain<T>
Probability<T>
Distribution<T>
Confidence<T>

are semantic design candidates, not automatic reserved keywords.

Type syntax MUST NOT impose physical capacity ceilings.

Type checking MUST distinguish compile-time guarantees from runtime checks.

The specification MUST define behavior for unsatisfied constraints, undecidable properties and representation overflow.

18. Effects

The effect system MUST provide one coherent model.

Relevant effect categories may include:

- IO;
- filesystem;
- network;
- mutation;
- randomness;
- native;
- foreign;
- distributed;
- measurement;
- quantum;
- learning;
- adaptation;
- reflection;
- code generation;
- simulation;
- resource allocation.

The list MUST be extensible through controlled semantic registration.

An effect MUST have a defined meaning.

Merely introducing a keyword does not implement an effect.

Learning, adaptation, measurement, network access, reflection and foreign calls MUST be checked according to their declared effects.

Effects MUST integrate with functions, types, contracts, policies and execution.

19. Contracts and validation

Contracts MUST be domain-neutral.

The common model is:

Contract
    |
    +-- scope
    +-- preconditions
    +-- postconditions
    +-- invariants
    +-- assumptions
    +-- guarantees
    +-- properties
    +-- evidence
    +-- provenance

Contract syntax belongs in the designated validation grammar.

Contract semantics MUST define:

- evaluation timing;
- evaluation environment;
- side-effect restrictions;
- failure behavior;
- diagnostic locations;
- interaction with optimization;
- interaction with concurrency;
- interaction with probabilistic operations;
- interaction with quantum measurement;
- runtime enforcement requirements.

A compiler MUST NOT claim a proof merely because a contract parses.

20. Policy architecture

Policies are universal semantic controls.

They MUST NOT be limited to AI or security.

The policy model SHOULD support:

- scope;
- permission;
- prohibition;
- requirement;
- constraint;
- preference;
- fallback;
- authorization;
- resource selection;
- execution;
- adaptation;
- simulation;
- deployment;
- provenance.

A policy MUST have deterministic precedence and conflict-resolution rules.

Conflicting policies MUST produce a defined result.

A policy MUST NOT silently override program semantics.

21. Provenance and evidence

Provenance MUST be a shared semantic model.

It SHOULD represent:

source
derived_from
generated_by
transformed_by
verified_by
decision
reason
evidence
version
timestamp

The representation MUST support source-to-artifact traceability.

Provenance MUST integrate with:

- compiler transformations;
- AI reasoning;
- learning;
- adaptation;
- security;
- quantum compilation;
- hardware realization;
- reproducible builds;
- diagnostics.

The implementation MUST define which provenance fields are mandatory, optional, redacted or retained under policy.

Timestamps and nondeterministic identifiers MUST NOT be allowed to make deterministic artifacts nondeterministic unless explicitly requested.

22. AI architecture

"grammar/ai/" owns AI-specific source constructs.

It does not own universal types, effects, policies, contracts, concurrency or provenance.

AI grammar MUST integrate with:

types/
expressions/
statements/
effects/
resources/
validation/
policies/
provenance/
concurrency/
data/
quantum/
hybrid/
execution/

AI features SHOULD be grouped by semantic ownership:

- inference;
- reasoning;
- knowledge;
- learning;
- adaptation;
- uncertainty;
- causality;
- explanation;
- evidence;
- decisions;
- neural-symbolic composition;
- agents;
- planning.

Application-specific algorithms MUST be provided by libraries or registered extensions rather than universal grammar productions.

23. Concurrency and distributed computation

There MUST be one coherent concurrency model.

Actors, tasks, messages, channels, synchronization, async operations and parallel execution MUST have explicit ownership.

AI agents MUST reuse the existing actor architecture.

Distributed execution MUST build on shared concurrency semantics rather than inventing an incompatible second task model.

The language MUST NOT encode fixed maximum numbers of actors, tasks, processes, nodes, messages or devices.

The runtime MAY impose documented resource policies.

Those policies MUST NOT become universal grammar restrictions.

Concurrency semantics MUST define:

- ownership;
- data races;
- ordering;
- synchronization;
- cancellation;
- failure;
- recovery;
- deterministic execution where applicable;
- distributed failure behavior.

24. Quantum architecture

Quantum syntax MUST remain target-independent.

The quantum subsystem owns logical quantum operations and quantum-specific source constructs.

It MUST NOT own physical hardware discovery, calibration, placement or backend-specific instruction encoding.

24.1 Generic operation model

A quantum operation MUST be expressible through a generic semantic structure containing applicable:

- operation identity;
- namespace;
- operands;
- parameters;
- results;
- attributes;
- modifiers;
- effects;
- capabilities;
- source information.

The grammar MUST NOT require every new gate or operation to be added as a new universal keyword.

24.2 Logical and physical resources

Logical qubits MUST be distinct from physical qubits.

The language MUST NOT assume a fixed qubit count.

Requirements MAY express resource needs symbolically.

24.3 Quantum lifecycle

Quantum source semantics MUST integrate with:

- quantum types;
- operations;
- circuits;
- measurements;
- classical control;
- dynamic circuits;
- channels;
- noise;
- error correction;
- resource analysis;
- capability negotiation;
- provenance;
- hybrid execution.

24.4 Canonical quantum boundary

All supported quantum frontends MUST converge on "quantum::ir".

There MUST NOT be a competing canonical quantum IR.

24.5 Quantum measurement

Measurement MUST be modeled as an operation with defined result semantics and appropriate effects.

Measurement results MUST integrate with classical control through explicit semantic rules.

24.6 Quantum simulation

Simulation MUST be distinguishable from physical quantum execution.

Approximation, noise assumptions and fidelity claims MUST be explicit.

25. Hybrid computation

Hybrid computation is a first-class composition of domains.

It MUST support semantic relationships between:

- classical computation and quantum operations;
- quantum measurement and classical decisions;
- AI operations and quantum operations;
- CPU execution and accelerators;
- host and device execution;
- simulation and physical realization.

The grammar MUST NOT define a separate hybrid type universe.

Hybrid constructs MUST use shared types, effects, resources, policies and provenance.

26. HDL and hardware/software co-design

HDL MUST express hardware intent without embedding arbitrary physical limits in universal syntax.

The HDL architecture MUST distinguish:

- logical hardware design;
- parameterization;
- signals;
- state;
- timing;
- constraints;
- verification;
- simulation;
- synthesis;
- physical realization.

The grammar MUST NOT require fixed universal register widths or wire widths.

A particular design MAY specify a width when it is part of that design's semantics.

Synthesis and physical implementation belong downstream.

Verification assertions MUST integrate with the common validation model where applicable.

27. Classical and data computation

Classical computation MUST remain the general-purpose foundation.

Data computation MUST share the universal type, expression, effect, resource and provenance models.

Graph operations, queries, datasets, schemas and pipelines MUST not require a second language.

SQL, JSON and XML MUST be handled through explicit dialects, parsers, libraries or interoperability boundaries.

They MUST NOT be inserted into the universal Zamani parser merely because they are useful formats.

28. Interoperability

Interoperability owns:

- FFI;
- ABI;
- foreign declarations;
- external types;
- calling conventions;
- data layouts;
- linkage;
- external functions;
- external formats.

The language MUST distinguish foreign declarations from verified native implementations.

Foreign calls MUST be subject to:

- type validation;
- effect analysis;
- capability checks;
- safety policies;
- ABI validation;
- provenance where required.

The compiler MUST NOT assume that all external functions are safe, pure or deterministic.

29. Metaprogramming

Metaprogramming MAY support:

- reflection;
- introspection;
- compile-time computation;
- syntax generation;
- quotation;
- syntax-tree manipulation;
- type-level computation;
- macros;
- DSL construction.

These facilities MUST have explicit phase boundaries.

Compile-time execution MUST have resource limits governed by compiler policy, not arbitrary language-wide semantic ceilings.

Reflection and generation MUST have defined effects and capability requirements.

Generated syntax MUST pass the same validation and compatibility rules as ordinary source.

30. Security and safe implementation

30.1 Rust implementation

All production Rust code MUST use safe Rust.

The use of Rust "unsafe" is prohibited.

This applies to:

- lexer;
- parser;
- AST;
- semantic analysis;
- IR;
- code generation;
- runtime integration;
- tests;
- utilities.

Third-party dependencies MUST be assessed for security, maintenance and safety.

Where a dependency internally uses unsafe Rust, the project MUST document the dependency boundary and evaluate its safety implications.

The project MUST NOT represent such a dependency as proving that the entire dependency graph contains no unsafe code.

30.2 Zamani source-level unsafe

The language-level "unsafe" construct is separate from Rust's "unsafe".

Its status MUST be explicitly specified.

It MUST NOT be accepted as a general escape hatch around memory safety, effect checks, resource checks or capability checks.

Any source-level unsafe feature MUST define:

- authorized operations;
- safety obligations;
- required capabilities;
- effects;
- diagnostics;
- verification boundaries;
- compatibility;
- runtime behavior.

If those requirements are not implemented, the construct MUST be marked unsupported or experimental.

30.3 Sandbox

Sandboxing MUST integrate with:

- effects;
- capabilities;
- resource policies;
- filesystem access;
- network access;
- foreign calls;
- reflection;
- code generation;
- adaptation.

The grammar describes sandbox intent.

Enforcement belongs to the security/runtime implementation.

A parsed sandbox declaration MUST NOT be represented as an enforced security boundary without runtime evidence.

31. Execution, simulation and resilience

Execution grammar MUST express portable execution intent.

The runtime owns actual scheduling and recovery.

Adaptive execution MUST NOT silently change the observable semantics of a program.

Possible states and outcomes MUST be defined in the execution/resilience specification.

The established quantum/runtime resilience vocabulary may include:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

and:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

These are domain/runtime states, not automatically universal source-language keywords.

The architecture MUST define how they are represented and exchanged.

Simulation MUST preserve semantics or identify approximation.

Fallback MUST be policy-authorized.

32. Compatibility and versioning

Zamani MUST have an explicit compatibility policy.

Relevant version boundaries include:

- language version;
- grammar version;
- lexer/token contract version;
- AST version;
- semantic model version;
- IR version;
- dialect version;
- backend interface version.

A change to syntax MUST be assessed for:

- source compatibility;
- lexical compatibility;
- parser compatibility;
- AST compatibility;
- semantic compatibility;
- tooling compatibility;
- generated documentation;
- existing programs.

New syntax MUST NOT silently reinterpret existing valid programs.

Deprecated syntax MUST have a migration policy.

The language version MUST NOT be arbitrarily rewritten by a program at runtime.

33. File-level completion contract

Every independently maintained grammar or specification file MUST declare its responsibilities before implementation.

The required contract is:

FILE:
STATUS:
PURPOSE:
OWNS:
DOES_NOT_OWN:
PUBLIC_RULES:
PRIVATE_RULES:
LEXER_DEPENDENCIES:
GRAMMAR_DEPENDENCIES:
DEPENDENCY_DIRECTION:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
TYPE_OWNER:
EFFECT_OWNER:
RESOURCE_OWNER:
POLICY_OWNER:
PROVENANCE_OWNER:
IR_OWNER:
SPEC_OWNER:
TEST_OWNER:
COMPATIBILITY:
INTEGRATION:
COMPLETION_CRITERIA:

Not every field requires a separate implementation file.

Where a concept is not applicable, the file MUST explicitly state "NOT_APPLICABLE" and explain why.

33.1 Feature contract

Every feature MUST additionally define:

1. Purpose.
2. Syntax.
3. Lexical behavior.
4. Valid forms.
5. Invalid forms.
6. AST representation.
7. Semantic meaning.
8. Type behavior.
9. Effect behavior.
10. Capability behavior.
11. Resource behavior.
12. Contract behavior.
13. Policy behavior.
14. Provenance behavior.
15. IR destination.
16. Backend boundary.
17. Diagnostics.
18. Positive tests.
19. Negative tests.
20. Boundary tests.
21. Scalability tests.
22. Compatibility tests.
23. Determinism requirements.
24. Integration dependencies.
25. Completion criteria.

33.2 Independent completion

A file is independently complete when:

- its ownership is unambiguous;
- its dependencies are specified;
- its public interface is stable;
- its semantic contract is complete;
- its AST and IR boundaries are known;
- its integration points are declared;
- its tests exist;
- its downstream consumers can be implemented without changing the file's public contract.

Independent completion does not prohibit a later versioned extension.

It prohibits avoidable rework caused by undocumented dependencies or incomplete ownership.

33.3 Dependency changes

If another file changes, consumers MUST NOT need to be rewritten merely because an internal implementation detail changed.

If a public contract must change, the change MUST include:

- a documented version or compatibility decision;
- impact analysis;
- migration guidance;
- updated tests;
- affected consumer updates.

No architecture can guarantee that public interfaces never change. It can guarantee that changes are explicit, controlled and traceable.

34. Integration ownership matrix

Concern| Primary owner| Consumers
Language architecture| "grammar/DESIGN.md"| All grammar/specification files
Navigation| "grammar/README.md"| Developers and tooling
ANTLR root| "grammar/Zamani.g4"| ANTLR tooling
Implemented syntax status| "grammar/grammar.md"| Developers and conformance tooling
Extended history| "grammar/Zamani-Grammar.md"| Reference readers
Tokenization| "src/lexer.rs" and lexical specification| Parser and tooling
ANTLR lexical representation| ANTLR lexer| ANTLR parser
Parsing| "src/parser.rs"| Frontend
AST| "src/ast/"| Semantic analysis
Types| Type specification and semantic implementation| Compiler
Effects| Effects specification and semantic implementation| Compiler/runtime
Resources| Resources specification and implementation| Planner/runtime
Policies| Policy specification and implementation| Security/planner/runtime
Contracts| Validation specification and implementation| Compiler/runtime
Provenance| Provenance specification and implementation| Compiler/tooling/runtime
Quantum IR| "quantum::ir"| Quantum frontends and backends
Classical IR| Canonical classical IR owner| Classical backends
HDL semantics| HDL semantic owner| Verification/synthesis
Target realization| Backend/HAL| Execution environments

A file may consume another owner's public contract but MUST NOT redefine it.

35. UBUNTU migration ownership

UBUNTU integration MUST be tracked in:

"grammar/specification/ubuntu-integration.md"

The migration matrix MUST contain:

Field| Meaning
UBUNTU concept| Original feature
Zamani destination| Canonical owner
Syntax status| Proposed/stable/etc.
Lexer status| Actual implementation
Parser status| Actual implementation
AST representation| Exact node or expression
Semantic owner| Responsible subsystem
Effect requirements| Applicable effects
Capability requirements| Applicable capabilities
Resource requirements| Applicable resources
IR destination| Canonical representation
Tests| Conformance test locations
Compatibility| Version implications
Decision| Adopt, adapt, defer or reject

The integration document MUST prevent duplicate ownership.

36. Specification organization

The following normative specifications SHOULD be maintained under existing directories:

grammar/specification/
    language-authority.md
    poco-reaf.md
    frontend.md
    types.md
    ai.md
    quantum.md
    hybrid.md
    hdl.md
    resources.md
    effects.md
    contracts.md
    policies.md
    provenance.md
    compatibility.md
    interoperability.md
    ubuntu-integration.md

This list defines intended specification responsibilities, not a claim that all these files already exist.

Machine-checkable contracts belong under:

grammar/spec/

Machine-checkable files MUST have a documented schema/version and validation procedure.

The human specification MUST remain understandable without requiring developers to infer semantics from generated metadata.

37. Testing architecture

Testing MUST cover the complete language pipeline.

The conformance suite MUST distinguish:

- lexical tests;
- parser tests;
- AST tests;
- structural validation;
- semantic tests;
- type tests;
- effect tests;
- resource tests;
- capability tests;
- contract tests;
- policy tests;
- provenance tests;
- IR tests;
- backend integration tests;
- compatibility tests;
- diagnostics tests.

37.1 Required test categories

Every stable feature MUST have applicable:

1. Positive tests.
2. Negative tests.
3. Boundary tests.
4. Scalability tests.
5. Cross-domain tests.
6. Compatibility tests.
7. Determinism tests.
8. Diagnostics tests.

Tests MUST verify behavior rather than merely search source files for keywords.

37.2 Grammar implementation agreement

ANTLR and Rust parser implementations MUST share conformance fixtures.

The test suite MUST detect:

- syntax accepted by one parser but rejected by the other;
- different precedence;
- different associativity;
- different keyword interpretation;
- inconsistent literal handling;
- inconsistent error handling.

37.3 Scalability tests

Scalability tests MUST distinguish:

- grammar recursion;
- parser stack usage;
- AST memory usage;
- semantic analysis complexity;
- IR construction;
- compiler resource exhaustion;
- runtime resource exhaustion.

Tests MUST NOT require literal infinity.

They MUST demonstrate the absence of arbitrary fixed ceilings and verify controlled behavior at implementation boundaries.

37.4 Cross-domain tests

The suite MUST include integrated programs combining relevant domains, including:

- classical and quantum;
- classical and HDL;
- AI and symbolic reasoning;
- learning and adaptation;
- AI and quantum;
- distributed and resource-aware computation;
- effects and policies;
- contracts and provenance;
- simulation and execution;
- FFI and security.

37.5 End-to-end POCO-REAF test

At least one integrated program MUST exercise:

source
  |
  v
lexer
  |
  v
parser
  |
  v
AST
  |
  v
semantic validation
  |
  v
type/effect/resource/capability analysis
  |
  v
contracts/policies/provenance
  |
  v
canonical semantic representation
  |
  +------ classical IR
  |
  +------ quantum::ir
  |
  +------ HDL intent
  |
  v
execution planning
  |
  v
target feasibility

The test MUST verify that a change of target does not silently change source-level meaning.

38. Hard-coding audit

Every production milestone MUST include a hard-coding audit.

The audit MUST search for:

- universal capacity constants;
- fixed resource arrays;
- fixed device enumerations;
- assumptions about physical qubit indices;
- fixed tensor rank limits;
- fixed network sizes;
- fixed thread counts;
- fixed memory ceilings;
- hidden parser nesting limits;
- undocumented integer truncation;
- hard-coded backend assumptions;
- hard-coded quantum gate catalogs;
- domain-specific keyword duplication.

A match is not automatically a defect.

The reviewer MUST determine whether it represents:

1. program data;
2. a target-specific property;
3. a documented implementation limit;
4. a configurable policy;
5. an accidental universal ceiling.

The audit MUST record its decision.

39. Determinism and reproducibility

Where the language or build configuration promises deterministic behavior, the implementation MUST define:

- stable ordering;
- deterministic name resolution;
- deterministic diagnostics where applicable;
- reproducible semantic transformations;
- reproducible compilation inputs;
- controlled randomness;
- provenance handling;
- target-specific nondeterminism.

Learning, probabilistic computation, concurrency and physical quantum execution may introduce nondeterminism.

Such behavior MUST be explicitly represented rather than falsely described as deterministic.

Reproducibility MUST distinguish:

- identical source;
- identical semantic meaning;
- identical compiler inputs;
- identical compilation artifact;
- identical execution result.

These are not equivalent guarantees.

40. Rust implementation requirements

The compiler MUST support Rust 1.97.1 as its declared baseline.

The repository MUST validate:

rustc --version
cargo --version
cargo check --all-targets
cargo test --all-targets
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

Where the project has unsupported feature combinations, the CI matrix MUST document and test the supported combinations instead of claiming universal feature coverage.

Production Rust MUST NOT use:

unsafe

The project SHOULD enforce this through code review, CI checks and dependency audits.

The Rust version MUST be declared in a valid Cargo manifest format.

The language's source-level "unsafe" syntax MUST NOT be confused with Rust implementation safety.

41. Performance and resource behavior

The grammar and frontend MUST be designed for predictable resource consumption.

The implementation SHOULD avoid:

- pathological grammar ambiguity;
- uncontrolled backtracking;
- exponential parsing behavior;
- unbounded diagnostic accumulation;
- unnecessary AST cloning;
- uncontrolled recursive descent;
- repeated semantic work;
- accidental quadratic processing.

Resource exhaustion MUST produce controlled errors where recoverable.

The compiler MUST distinguish a resource budget imposed by configuration from a semantic limit imposed by the language.

Incremental parsing and analysis MAY be implemented where beneficial.

Performance optimizations MUST preserve specified semantics.

42. Diagnostics

Diagnostics MUST be structured and source-aware.

They SHOULD identify:

- source file;
- span;
- error category;
- relevant construct;
- expected condition;
- actual condition;
- related location;
- suggested correction where reliable.

The compiler MUST distinguish syntax errors from semantic, type, capability, resource, policy and target-feasibility errors.

A missing target capability MUST NOT be reported as a grammar error.

An unsupported feature MUST NOT be reported as valid merely because its syntax parses.

43. Production readiness gates

A feature is production-ready only when all applicable gates pass.

Gate A — Authority

- One normative specification exists.
- Ownership is unambiguous.
- No competing language definition exists.

Gate B — Lexical and grammar

- Token behavior is specified.
- Grammar behavior is specified.
- Rust and ANTLR behavior is reconciled.
- Invalid syntax is rejected.

Gate C — Frontend

- AST representation exists.
- Source locations are preserved.
- Diagnostics are implemented.
- Structural validation exists.

Gate D — Semantics

- Meaning is defined.
- Types are checked.
- Effects are checked.
- Resources and capabilities are checked where applicable.
- Policies and contracts are integrated where applicable.

Gate E — IR

- Canonical semantic representation exists.
- Domain IR boundary is documented.
- Quantum constructs reach "quantum::ir".
- No competing canonical IR is introduced.

Gate F — Execution

- Backend/runtime integration exists where required.
- Target feasibility is checked.
- Failure and fallback semantics are defined.

Gate G — Tests

- Positive tests pass.
- Negative tests pass.
- Boundary tests pass.
- Scalability tests pass.
- Cross-domain tests pass.
- Compatibility tests pass.
- Diagnostics are verified.

Gate H — Safety and maintainability

- No Rust "unsafe" is introduced.
- Hard-coding audit passes.
- Dependency contracts are complete.
- Public interfaces are documented.
- Compatibility impact is reviewed.

A feature MUST NOT be marked stable until all applicable gates pass.

44. Implementation order

Implementation MUST follow dependency order, not alphabetical order.

Phase 0 — Authority and baseline

- "grammar/DESIGN.md"
- "grammar/README.md"
- "grammar/grammar.md"
- "grammar/Zamani-Grammar.md"
- existing specifications;
- actual frontend inventory;
- Rust toolchain and build baseline.

Phase 1 — Lexical contracts

- token registry;
- keyword policy;
- operators;
- delimiters;
- identifiers;
- literals;
- comments;
- Unicode;
- diagnostics;
- Rust/ANTLR conformance.

Phase 2 — Universal syntax

- core;
- types;
- expressions;
- statements;
- declarations;
- functions;
- modules;
- patterns;
- guards.

Phase 3 — Universal semantics

- resources;
- capabilities;
- effects;
- contracts;
- policies;
- provenance;
- semantic validation.

Phase 4 — Execution foundations

- concurrency;
- memory;
- execution;
- resilience;
- simulation;
- security.

Phase 5 — Classical and data

- classical;
- numerical;
- tensor;
- data;
- graph;
- query;
- scientific computation.

Phase 6 — Quantum and hybrid

- quantum;
- quantum operations;
- measurement;
- dynamic circuits;
- quantum resources;
- quantum::ir integration;
- hybrid semantics;
- QEC boundary.

Phase 7 — HDL and hardware

- HDL intent;
- hardware resources;
- verification;
- simulation;
- synthesis boundary;
- hardware/software co-design.

Phase 8 — UBUNTU-derived AI

- reasoning;
- knowledge;
- inference;
- learning;
- adaptation;
- uncertainty;
- causality;
- explanation;
- evidence;
- agents;
- neural-symbolic composition.

Phase 9 — Distributed and networking

- actors;
- distributed computation;
- networking;
- protocols;
- fault tolerance;
- topology abstractions.

Phase 10 — Interoperability and dialects

- FFI;
- ABI;
- SQL;
- JSON;
- XML;
- external formats;
- dialect registration.

Phase 11 — Metaprogramming

- reflection;
- compile-time computation;
- generation;
- quotation;
- syntax-tree operations.

Phase 12 — Conformance and production

- complete test matrix;
- cross-domain integration;
- hard-coding audit;
- reproducibility;
- compatibility;
- performance;
- security;
- production acceptance.

A phase MAY proceed independently only when its public dependencies are specified.

45. Required architectural invariants

The following invariants are mandatory.

Invariant 1: Zamani is one language, not a collection of incompatible domain languages.

Invariant 2: Grammar defines syntax, not physical hardware realization.

Invariant 3: The Rust frontend and ANTLR representation MUST agree on the normative language.

Invariant 4: AST is domain-neutral and target-independent.

Invariant 5: Domain-specific semantics MUST have explicit ownership.

Invariant 6: Quantum frontends converge on "quantum::ir".

Invariant 7: No artificial universal hardware ceiling is permitted.

Invariant 8: Actual resource limits MUST be observable and correctly diagnosed.

Invariant 9: UBUNTU features are integrated as shared semantic capabilities, not copied as a competing grammar.

Invariant 10: Application-specific functionality belongs in libraries, dialects, services or policies.

Invariant 11: Effects, capabilities, resources, contracts and policies MUST have distinct meanings.

Invariant 12: Learning and adaptation MUST have explicit authorization and effects.

Invariant 13: Simulation MUST NOT be confused with physical execution.

Invariant 14: Parsing does not prove semantic correctness.

Invariant 15: Syntax acceptance does not establish backend support.

Invariant 16: Rust production implementation MUST NOT use "unsafe".

Invariant 17: Every stable feature MUST have conformance tests.

Invariant 18: Every public file contract MUST declare dependencies and integration points.

Invariant 19: Public contract changes MUST be explicit and versioned.

Invariant 20: No feature may be described as implemented without implementation evidence.

46. Definition of architectural completion

The grammar architecture is ready for production implementation when:

1. Every existing grammar directory has an explicit responsibility.
2. Existing files have been inventoried and their ownership reconciled.
3. The normative specification hierarchy is unambiguous.
4. The Rust lexer/parser and ANTLR grammar have a shared conformance model.
5. AST and semantic ownership are documented.
6. Classical, quantum, HDL, hybrid and AI boundaries are explicit.
7. The canonical quantum IR boundary is preserved.
8. UBUNTU migration is traceable.
9. Resource and capability semantics are independent of physical target enumeration.
10. No universal hardware ceilings exist.
11. Every public file has a dependency and integration contract.
12. The production test architecture exists.
13. Rust 1.97.1 builds and tests are verified.
14. The Rust implementation satisfies the no-"unsafe" requirement.
15. Compatibility and versioning rules are defined.
16. Production readiness is measured by implementation evidence rather than documentation volume.

47. Final architectural principle

Zamani MUST describe computation in terms of stable meaning.

It MUST NOT force programmers to rewrite programs whenever a machine, accelerator, quantum processor, topology, memory system or execution environment changes.

Its universal foundation is:

                    ZAMANI
                       |
          +------------+------------+
          |                         |
     UNIVERSAL CORE            DOMAIN SYSTEMS
          |                         |
     Types / Values            Classical
     Operations                Quantum
     Expressions               Hybrid
     Effects                   HDL
     Resources                 AI
     Capabilities              Data
     Contracts                 Distributed
     Policies                  Networking
     Provenance                Hardware
          |                         |
          +------------+------------+
                       |
                Semantic Model
                       |
                Canonical IR
                       |
              Domain-specific IR
                       |
                 Optimization
                       |
                   Lowering
                       |
             Routing / Scheduling
                       |
              Resilience / Recovery
                       |
                   ZQN / HAL
                       |
               Target Realization
                       |
          +------------+------------+
          |            |            |
         CPU          GPU          FPGA
          |            |            |
         ASIC         QPU       Accelerator
          |            |            |
          +------------+------------+
                       |
            HPC / Distributed / Cloud
                       |
                Future Targets

The same source-level meaning must survive this process.

The language must scale from the smallest supported computation to the largest computation that its semantics, implementation and available resources can support.

POCO-REAF is achieved through semantic stability, explicit resource negotiation, target-independent representations, controlled specialization, verified compiler integration and compatibility—not through an infinitely large grammar or a promise that finite hardware has infinite capacity.

This document is the architectural authority for those requirements. Individual specifications and implementations MUST conform to it.