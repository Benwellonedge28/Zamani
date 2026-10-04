Zamani Grammar — Universal Language Architecture

Path: "grammar/README.md"
Repository: "Benwellonedge28/Zamani"
Language: Zamani
Edition: Rust 2021
Required Rust baseline: Rust 1.97.1 (or the compatible Rust 1.97 toolchain)
Safety: No Rust "unsafe" code
Primary objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Status: Architecture and implementation navigation contract

---

1. Purpose

The "grammar/" directory defines the source-language architecture, syntax specifications, grammar composition, conformance requirements and integration contracts of Zamani.

Zamani is intended to be one extensible, domain-neutral programming language capable of expressing:

- Classical and systems programming.
- Embedded and resource-constrained programming.
- Scientific and numerical computation.
- Symbolic computation and formal reasoning.
- Quantum computation and quantum error correction.
- Hybrid quantum-classical computation.
- Hardware description and hardware/software co-design.
- AI, machine learning and neural-symbolic computation.
- Tensor and data computation.
- Parallel, concurrent and distributed computation.
- High-performance computing.
- Networking and cryptography.
- Heterogeneous computing and accelerators.
- Edge, cloud and cluster computation.
- Temporal and multi-timeline computation.
- Metaprogramming and interoperability.
- Future computational domains.

These domains must share one coherent language foundation.

They must not become independent languages with incompatible type systems, expression models, resource models, or semantic authorities.

The grammar must support extensibility without requiring the root grammar to enumerate every future operation, device, algorithm, instruction, physical topology or application.

1.1 Scope

This directory owns the documented source-language and grammar contracts.

It does not independently implement:

- The complete Rust compiler.
- Hardware discovery.
- Physical resource allocation.
- Quantum execution.
- Machine learning algorithms.
- Runtime scheduling.
- Backend code generation.
- Operating-system functionality.
- Hardware-specific optimizations.

Those responsibilities belong to their respective repository subsystems.

The grammar must establish explicit contracts with those subsystems.

---

2. Fundamental architecture

The intended architecture is:

Zamani source
      |
      v
Source and lexical specification
      |
      v
Lexer
      |
      v
Parser / ANTLR grammar
      |
      v
Domain-neutral AST
      |
      v
Structural validation
      |
      +------------------------------+
      |                              |
      v                              v
Name and module resolution       Syntax diagnostics
      |
      v
Semantic analysis
      |
      +--------------------------------------------+
      |             |              |               |
      v             v              v               v
    Types         Effects      Capabilities      Contracts
      |             |              |               |
      +-------------+--------------+---------------+
                            |
                            v
                   Resource analysis
                            |
                            v
                    Policy analysis
                            |
                            v
                     Provenance model
                            |
                            v
                    Canonical semantic model
                            |
              +-------------+-------------+
              |                           |
              v                           v
         Classical IR                quantum::ir
              |                           |
              +-------------+-------------+
                            |
                            v
                 Target-independent optimization
                            |
                            v
                  Domain-specific lowering
                            |
                            v
                   Resource negotiation
                            |
                            v
                      Routing
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
          +-----------------+-------------------+
          |         |        |         |         |
          v         v        v         v         v
         CPU       GPU      FPGA      ASIC      QPU
          |         |        |         |         |
          +---------+--------+---------+---------+
                            |
                            v
              Embedded / HPC / cluster /
              distributed / cloud / future targets

This is an architectural contract, not a claim that every stage is already implemented.

Each stage must have an identifiable owner and a conformance status.

---

3. POCO-REAF

3.1 Definition

POCO-REAF means:

Program Once, Compile Once, Run Everywhere, Anywhere, Forever.

A program should express its computational meaning independently of a particular physical machine.

Changing the target must not require rewriting the program's algorithm merely because the available hardware has changed.

The compiler and runtime should negotiate and realize the program using available resources and target capabilities.

3.2 Portability guarantees

POCO-REAF requires:

1. Stable source-language semantics.
2. A target-independent semantic representation.
3. Explicit resource requirements.
4. Explicit capability requirements.
5. Explicit constraints and preferences.
6. Target-specific lowering outside the language's universal semantic model.
7. Versioned compiler and IR contracts.
8. Reproducible compilation where required.
9. Explicit diagnostics when a target cannot satisfy a requirement.
10. No silent change in program meaning to accommodate a target.

The following are different conditions:

Lexically valid
    !=
Syntactically valid
    !=
Semantically valid
    !=
Type correct
    !=
Compilable
    !=
Target compatible
    !=
Resource feasible
    !=
Executable

A program that requires an unavailable capability must receive an appropriate diagnostic or use an explicitly permitted fallback.

3.3 What POCO-REAF does not promise

POCO-REAF does not promise:

- Infinite physical memory.
- Infinite execution time.
- Infinite processing power.
- Infinite qubits.
- Infinite network bandwidth.
- Infinite storage.
- Identical performance across targets.
- Automatic physical feasibility.
- Identical numerical results where the language explicitly permits target-dependent numerical behavior.

Portability is a semantic guarantee, not a violation of physical constraints.

---

4. Scalability and the no-hard-coded-ceilings rule

Zamani must scale from the smallest useful computation to arbitrarily large computations, subject to available resources and explicitly defined semantics.

The language must not impose artificial universal capacity ceilings.

The following are prohibited as universal language-level limits:

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
MAX_NETWORK_LINKS
MAX_GATE_COUNT
MAX_DEVICE_COUNT
MAX_NETWORK_SIZE

This prohibition applies to grammar design, semantic architecture and compiler-wide artificial restrictions.

It does not prohibit:

- Ordinary numeric literals.
- Explicit program constants.
- Target-specific constraints.
- Compiler resource budgets.
- Runtime resource limits.
- Memory allocation failures.
- Physical hardware limitations.
- Configurable operational policies.

For example:

let count = 1024;

is ordinary program data.

A universal restriction preventing Zamani programs from representing more than a particular number of elements is not acceptable.

4.1 Resource-dependent scalability

The language should support symbolic and dynamic resource requirements.

Illustrative syntax:

requires memory >= required_memory;
requires qubits >= required_qubits;
requires capability("tensor.compute");
requires capability("quantum.measurement");
requires topology(required_topology);

These examples establish intended semantics. Their exact accepted syntax must be determined by the authoritative resource specification and implemented grammar.

The grammar must not determine whether a target actually possesses those resources.

That decision belongs to semantic analysis, resource negotiation, deployment and runtime.

---

5. Authority model

There must be one authoritative source for each responsibility.

No two files may independently define conflicting language semantics.

5.1 "grammar/README.md"

Owns: Navigation, architectural overview, file ownership, integration guidance and development workflow.

Does not own: Normative grammar productions, detailed language semantics or implementation claims unsupported by tests.

This document must link to the authoritative documents rather than duplicate them.

5.2 "grammar/DESIGN.md"

Owns: The normative architecture of the grammar subsystem.

It must define:

- Language authority.
- Grammar authority.
- Lexer authority.
- AST authority.
- Semantic authority.
- IR authority.
- Resource and capability boundaries.
- Effect and contract boundaries.
- Policy and provenance boundaries.
- Dialect architecture.
- Versioning.
- Compatibility.
- POCO-REAF.
- Scalability.
- Hard-coding prohibitions.
- Cross-domain integration.
- Production-readiness criteria.

All grammar documents must conform to this architecture.

5.3 "grammar/Zamani.g4"

Owns: The canonical ANTLR root grammar and composition contract.

It must remain the root grammar.

It must not become a second independent implementation of every domain's syntax.

Its responsibility is to establish the root language composition and entry points.

It must not own:

- Every expression production.
- Every type production.
- Every quantum operation.
- Every AI operation.
- Every hardware construct.
- Every resource rule.
- Physical target mapping.
- Runtime semantics.

The actual composition must remain consistent with the existing grammar files.

5.4 "grammar/antlr/ZamaniLexer.g4"

Owns: Canonical ANTLR lexical recognition.

It must agree with:

- The lexical specification.
- The Rust lexer.
- The token registry.
- The parser's expected tokens.
- Lexical conformance tests.

It must not become an independent source of incompatible token meanings.

5.5 "grammar/antlr/ZamaniParser.g4"

Owns: Canonical ANTLR parser composition.

It must consume the domain grammar through the existing grammar integration mechanism.

It must not duplicate domain-specific syntax owned by subordinate grammars.

Its public rules and imports must be stable and documented.

5.6 "grammar/grammar.md"

Owns: Implementation-conformance reporting.

It describes what the current reference implementation actually accepts.

It must distinguish:

Status| Meaning
SPECIFIED| Normative syntax or semantics have been defined
LEXER_IMPLEMENTED| The reference lexer implements the feature
PARSER_IMPLEMENTED| The reference parser implements the feature
AST_IMPLEMENTED| The AST represents the feature
SEMANTIC_IMPLEMENTED| Semantic analysis implements it
IR_IMPLEMENTED| The canonical IR integration exists
BACKEND_IMPLEMENTED| At least one relevant backend supports it
TESTED| Required conformance tests pass
STABLE| The complete stability requirements are satisfied
EXPERIMENTAL| Available but not stable
PARTIALLY_IMPLEMENTED| Some required stages remain incomplete
PLANNED| Not implemented
DEPRECATED| Scheduled for removal or replacement

A parser accepting a construct does not establish that its semantics, IR or runtime implementation exists.

5.7 "grammar/Zamani-Grammar.md"

Owns: Historical, extended and exploratory grammar reference material.

It must distinguish:

- STABLE.
- PROPOSED.
- EXPERIMENTAL.
- DEPRECATED.
- HISTORICAL.
- NOT_IMPLEMENTED.

It must not silently make an exploratory construct normative.

Promotion requires specification, implementation, tests and compatibility review.

5.8 "grammar/specification/"

Owns: Normative human-readable language specifications.

These documents define the intended meaning of language constructs.

5.9 "grammar/spec/"

Owns: Focused machine-checkable contracts, invariants, validation rules and conformance requirements.

The relationship between "specification/" and "spec/" must be explicit in "DESIGN.md".

They must not become competing specifications.

---

6. Repository integration and existing files

The grammar subsystem must expand the existing repository architecture.

Do not rename established major files merely to create a new naming convention.

Do not create duplicate owners when an appropriate file already exists.

The existing directories must be audited before introducing new directories or grammar files.

The established directories include:

grammar/
├── ai/
├── antlr/
├── classical/
├── compatibility/
├── compile/
├── concurrency/
├── core/
├── data/
├── declarations/
├── dialects/
├── distributed/
├── effects/
├── execution/
├── expressions/
├── functions/
├── hardware/
├── hdl/
├── hybrid/
├── interoperability/
├── lexer/
├── macros/
├── memory/
├── metaprogramming/
├── modules/
├── networking/
├── quantum/
├── reference/
├── resources/
├── security/
├── spec/
├── specification/
├── statements/
├── tests/
├── types/
└── validation/

The directory listing is an architectural inventory, not an instruction to duplicate existing files.

Before adding a file:

1. Search for an existing owner.
2. Read its complete contents.
3. Identify its current consumers.
4. Identify its AST and semantic representation.
5. Determine whether it can be extended.
6. Document the integration contract.
7. Add a new file only when the responsibility is genuinely distinct.

---

7. Universal language foundation

The universal language foundation must be domain-neutral.

It should provide common constructs for:

- Programs and source units.
- Identifiers and names.
- Qualified names.
- Declarations.
- Attributes and annotations.
- Modifiers.
- Expressions.
- Statements.
- Functions.
- Modules.
- Types.
- Patterns.
- Contracts.
- Requirements.
- Constraints.
- Capabilities.
- Effects.
- Policies.
- Metadata.
- Provenance.

These must be reused across all domains.

No AI-specific, quantum-specific or hardware-specific subsystem should create an incompatible alternative to a universal language concept.

---

8. UBUNTU feature integration

UBUNTU contributes useful computational capabilities to Zamani.

Its contribution must be integrated into existing Zamani architecture rather than copied as a separate language.

8.1 Reasoning

Integrate:

infer
deduce
reason

Owner:

"grammar/ai/"

Integration:

Reasoning syntax
      |
      v
Domain-neutral AST
      |
      v
Reasoning semantic model
      |
      v
Effects / capabilities / provenance
      |
      v
Canonical semantic representation

Reasoning must not be restricted to artificial intelligence.

It may also support scientific computation, symbolic computation, validation and knowledge processing.

8.2 Knowledge

Integrate:

assert
retract
query

The knowledge model must represent:

- Facts.
- Relationships.
- Assertions.
- Retractions.
- Queries.
- Metadata.
- Evidence.
- Provenance.

Knowledge operations must integrate with the existing data subsystem rather than creating a second unrelated data language.

8.3 Learning

Integrate:

learn

Learning must specify or resolve:

- Input data.
- Model.
- Objective.
- Training or adaptation operation.
- Required resources.
- Capabilities.
- Effects.
- Constraints.
- Policy.
- Provenance.

The grammar must not enumerate every machine-learning algorithm.

Algorithms belong in libraries, registered operations and semantic capability definitions.

8.4 Adaptation

Integrate:

adapt

Adaptation must not mean unrestricted self-modifying execution.

It must participate in:

- Authorization.
- Policy checking.
- Effect checking.
- Resource checking.
- Validation.
- Provenance recording.
- Reproducibility requirements.

8.5 Uncertainty

Integrate uncertainty through the type and semantic systems.

Concepts may include:

Uncertain<T>
Probability<T>
Distribution<T>
Confidence<T>

These are conceptual semantic types, not mandatory hard-coded primitive implementations.

The language must permit alternative probabilistic representations and implementations.

8.6 Evidence and explainability

Integrate:

- Evidence.
- Claims.
- Sources.
- Confidence.
- Derivations.
- Explanations.
- Decisions.
- Verification.
- Provenance.

These must be usable outside AI, including compiler optimization, resource allocation, quantum routing, security and hardware placement.

8.7 Agents

AI agents must reuse the existing concurrency and actor architecture.

The intended integration is:

AI agent
    |
    v
Actor semantics
    |
    v
Messages and channels
    |
    v
Concurrency runtime

Do not introduce a second incompatible actor lifecycle.

8.8 Neural-symbolic computation

Neural-symbolic composition must allow symbolic operations, learned models and reasoning to cooperate through common types, operations, effects and semantic representations.

It must not create a second compiler pipeline.

8.9 Contracts and policies

Integrate:

requires
ensures
invariant
assume
guarantee
property

The exact syntax and semantics must be owned by the validation and policy specifications.

A contract must not be treated as a comment.

It must have defined validation behavior and diagnostics.

8.10 Features that remain outside core syntax

The following must not become universal language keywords merely because they appeared in UBUNTU:

- Payment systems.
- Administrative interfaces.
- Legal actions.
- Copyright enforcement.
- Application-specific sentiment operations.
- Application-specific computer vision.
- Application-specific robotics.
- VR/AR.
- Blockchain application operations.
- ASI/AGI taxonomies.
- Omnipotence.
- Application-specific business workflows.

They belong in libraries, dialects, policies, services or applications.

---

9. Grammar ownership by subsystem

The following table establishes the intended responsibility boundaries.

Directory| Responsibility| Must not own
"antlr/"| ANTLR lexer/parser composition| Independent language semantics
"lexer/"| Token and lexical contracts| AST or runtime semantics
"core/"| Universal syntax and shared constructs| Physical target realization
"types/"| Type syntax and type-system contracts| Backend-specific type lowering
"expressions/"| Expression syntax| Duplicate statement ownership
"statements/"| Statement syntax| Duplicate expression ownership
"declarations/"| Declaration composition| Runtime implementation
"functions/"| Function declarations and signatures| Independent type authority
"modules/"| Module and import syntax| Build-system implementation
"resources/"| Resource requirements and abstractions| Hardware discovery
"effects/"| Effect declarations and effect syntax| Runtime effect implementation
"validation/"| Contracts and validation syntax| Proof-engine implementation
"ai/"| AI and reasoning-specific syntax| Second type or actor system
"classical/"| Classical domain syntax| Universal expression ownership
"quantum/"| Quantum domain syntax| Physical QPU implementation
"hybrid/"| Cross-domain composition| Independent quantum IR
"hdl/"| Hardware description syntax| Physical synthesis implementation
"hardware/"| Hardware intent and abstractions| Fixed universal hardware capacities
"concurrency/"| Tasks, actors, synchronization and messaging| AI-specific actor replacement
"execution/"| Execution intent and modes| Backend-specific runtime internals
"distributed/"| Distributed computation syntax| Fixed cluster limits
"networking/"| Network and communication syntax| Network transport implementation
"data/"| Data structures, queries and schemas| SQL implementation duplication
"interoperability/"| Foreign interfaces and external representations| Unrestricted native execution
"dialects/"| Optional language extensions| Changes to universal semantics
"security/"| Security declarations and constraints| General-purpose AI semantics
"metaprogramming/"| Reflection and compile-time facilities| Unrestricted runtime self-modification
"macros/"| Macro syntax and expansion contracts| Duplicate parser architecture
"memory/"| Memory-related language constructs| Artificial universal memory ceilings
"compile/"| Compilation intent and specialization contracts| Physical runtime scheduling
"compatibility/"| Versioning and migration contracts| Arbitrary language self-versioning
"specification/"| Normative specifications| Implementation status claims
"spec/"| Formal and machine-checkable contracts| Duplicate normative authority
"tests/"| Conformance and integration tests| Language specification ownership
"reference/"| Developer reference material| Competing normative definitions

Existing files take precedence over newly proposed filenames when their responsibilities already match.

---

10. UBUNTU integration destinations

UBUNTU capability| Zamani owner| Required integration
infer| "ai/"| Reasoning semantics
deduce| "ai/"| Deduction model
reason| "ai/"| Unified reasoning model
assert| "ai/", "validation/"| Knowledge assertions and contracts distinguished
retract| "ai/"| Knowledge lifecycle
query| "data/", "ai/"| Shared query semantics
learn| "ai/"| Learning operation and effects
adapt| "ai/", "execution/"| Controlled adaptation
match| "expressions/", "statements/"| Pattern matching
guards| "expressions/"| Guard expressions
actors| "concurrency/"| Existing actor model
contracts| "validation/"| Preconditions, postconditions, invariants
uncertainty| "types/", "ai/"| Shared uncertainty semantics
evidence| "validation/", "ai/"| Universal evidence abstraction
explain| "ai/", provenance| Explainability
policies| "core/", "security/", "execution/"| Shared policy model
sandbox| "security/"| Effects and capability restrictions
simulation| "execution/"| Execution mode
multi-agent| "ai/", "concurrency/"| Actor reuse
neural-symbolic| "ai/"| Unified computation
FFI| "interoperability/"| ABI, effects and capabilities
reflection| "metaprogramming/"| Controlled reflection
dependent types| "types/"| Type-system extension
linear types| "types/"| Ownership/resource semantics
SQL| "dialects/", "interoperability/"| External query dialect
JSON/XML| "data/", "interoperability/"| Interchange representations
quantum ML| "ai/", "hybrid/", "quantum/"| Canonical quantum boundary
adaptive execution| "execution/"| Resource-aware execution
causal reasoning| "ai/"| Generic causal model
transfer learning| "ai/"| Library/semantic operation
explainability| provenance and "ai/"| Universal explanations

The exact syntax must be specified before adding new grammar productions.

---

11. Resource and capability architecture

Resource abstractions are essential to POCO-REAF.

Zamani must distinguish:

Concept| Meaning
Requirement| Something necessary for valid execution
Capability| Something a target or environment can provide
Constraint| A condition that must be respected
Budget| An explicitly declared allocation or limit
Preference| A desirable but non-mandatory property
Hint| Optimization information that does not alter meaning
Negotiation| Determining whether a realization satisfies requirements
Realization| Mapping logical intent onto actual resources

The language must support resource requirements without enumerating every possible future device.

Examples:

requires capability("gpu.compute");
requires capability("quantum.measurement");
requires memory >= required_memory;
requires qubits >= required_qubits;
requires topology(required_topology);

These are illustrative until their syntax is confirmed by the normative grammar.

Resource resolution must happen downstream:

Source requirement
      |
      v
Semantic validation
      |
      v
Capability discovery
      |
      v
Resource negotiation
      |
      v
Execution planning
      |
      v
Target realization

The grammar must not inspect physical hardware.

---

12. Effects, contracts, policies and provenance

These four concepts must remain distinct but composable.

12.1 Effects

Effects describe computational behavior that must be accounted for.

Potential categories include:

- Mutation.
- Input/output.
- Network access.
- Foreign calls.
- Native operations.
- Randomness.
- Distributed communication.
- Measurement.
- Quantum operations.
- Learning.
- Adaptation.
- Reflection.
- Code generation.
- Simulation.

The effect specification determines the actual effect system.

Do not add categories without defining their semantics and interactions.

12.2 Contracts

Contracts express correctness conditions.

They must have:

- Defined scope.
- Defined evaluation point.
- Defined failure behavior.
- Defined interaction with types.
- Defined interaction with effects.
- Defined interaction with policies.
- Defined diagnostics.
- Defined testing requirements.

12.3 Policies

Policies govern whether and how operations may be performed.

They may govern:

- Resource allocation.
- Execution.
- Security.
- Adaptation.
- Simulation.
- Deployment.
- Networking.
- Quantum execution.
- Distributed execution.
- Foreign calls.

A policy must not silently redefine the semantics of an operation.

12.4 Provenance

Provenance records the origin and derivation of information and transformations.

It should support:

source
derived_from
generated_by
transformed_by
verified_by
reason
evidence
decision
version
timestamp

The semantic provenance model must be shared by the compiler, AI, quantum, security, scientific and execution subsystems.

---

13. Quantum integration

Quantum syntax must be generic, extensible and independent of physical hardware.

The grammar must not enumerate a fixed universal collection of quantum operations.

The preferred conceptual model is:

operationSpecifier
quantumTargetList
parameters
results
attributes
modifiers

An operation may be:

- Built-in.
- Custom.
- Vendor-provided.
- Parameterized.
- Composite.
- Defined through an extension or dialect.

The grammar should recognize the general structure without requiring a grammar modification for every new operation.

13.1 Canonical quantum pipeline

Zamani source
      |
      v
Domain-neutral AST
      |
      v
Quantum semantic analysis
      |
      v
quantum::ir
      |
      v
Quantum optimization
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

"quantum::ir" is the canonical quantum IR.

Do not introduce another independent quantum frontend IR.

The grammar must not own:

- Physical qubit identifiers.
- Calibration data.
- Device topology.
- Gate routing decisions.
- Hardware scheduling decisions.
- QEC decoder implementation.
- Physical measurement implementation.

13.2 Quantum scalability

The language must not impose universal limits on:

- Logical qubits.
- Physical qubits.
- Quantum operations.
- Circuit size.
- Quantum states.
- Channels.
- Measurements.
- QPUs.
- Quantum devices.

The implementation must distinguish representation limits from language-level limits.

---

14. Classical, hybrid and HDL integration

14.1 Classical

Classical constructs must use the universal:

- Type system.
- Expression system.
- Function system.
- Effect system.
- Resource model.
- Capability model.
- Contract model.
- Policy model.
- Provenance model.

14.2 Hybrid

Hybrid computation must express transitions between computational domains.

Examples include:

classical -> quantum
quantum -> classical
classical control -> quantum operation
measurement -> classical decision
AI -> quantum
quantum -> AI
CPU -> accelerator
host -> device

Hybrid constructs must lower into the appropriate canonical semantic representations.

They must not create a separate language or quantum IR.

14.3 HDL

HDL syntax must represent hardware intent, not a universal physical hardware ceiling.

The specification must address:

- Signals.
- Connections.
- Interfaces.
- Timing.
- Clocks.
- State.
- Memory.
- Parameterization.
- Simulation.
- Verification.
- Synthesis intent.
- Hardware/software boundaries.

A fixed bus width may be valid when explicitly specified by a particular program or hardware interface.

It must not become a universal limitation of Zamani.

---

15. AI architecture

The AI subsystem must integrate with the universal language rather than create a separate compiler.

Its syntax and semantics must support the relevant concepts:

- Reasoning.
- Deduction.
- Inference.
- Knowledge.
- Learning.
- Adaptation.
- Uncertainty.
- Probability.
- Distributions.
- Evidence.
- Explanations.
- Decisions.
- Causality.
- Planning.
- Policies.
- Agents.
- Neural-symbolic composition.

The AI subsystem must consume shared:

- Types.
- Expressions.
- Effects.
- Capabilities.
- Resources.
- Contracts.
- Policies.
- Provenance.
- Concurrency.
- Execution.

AI operations must have defined semantic and effect contracts.

No AI construct should be marked stable based solely on a grammar production.

---

16. Interoperability and dialects

External languages and formats must not become competing Zamani authorities.

Examples:

- SQL.
- JSON.
- XML.
- OpenQASM.
- Foreign function interfaces.
- Foreign ABI descriptions.
- External data schemas.

Their integration must follow:

External representation
        |
        v
Dedicated parser / dialect
        |
        v
Validation
        |
        v
Zamani semantic model
        |
        v
Canonical IR

The dialect architecture must define:

- Registration.
- Versioning.
- Namespace isolation.
- Token conflicts.
- Extension conflicts.
- Compatibility.
- Semantic validation.
- Error reporting.
- Security boundaries.

FFI must explicitly participate in capability and effect checking.

---

17. Lexer and token authority

The lexer must have one coherent lexical contract.

Every reserved token must have:

- Canonical name.
- Canonical spelling.
- Category.
- Owning specification.
- Reserved status.
- Context.
- Compatibility status.
- Conformance tests.

Avoid duplicate token names unless their distinction is semantically justified.

For example, "Question" and "QuestionMark" must not represent the same lexical construct without an explicit reason.

The same applies to "Ampersand" and "BitAnd".

The token registry must not become a second independently maintained token implementation.

Its relationship with the ANTLR lexer and Rust lexer must be specified.

17.1 Keyword policy

A word should be reserved only when the language requires it.

Application-specific names should normally remain identifiers.

Do not add a keyword solely because it appeared in UBUNTU.

New keywords require:

1. A specification.
2. A lexical definition.
3. Parser integration.
4. Compatibility review.
5. Rust lexer integration.
6. AST integration.
7. Tests.
8. Documentation.

---

18. AST and Rust frontend integration

The Rust frontend is part of the language conformance boundary.

The existing files include:

src/lexer.rs
src/parser.rs
src/ast/
src/source_map.rs

The actual ownership of each AST node must be documented in the corresponding specification.

The AST must preserve:

- Source locations.
- Program structure.
- Expression structure.
- Declaration structure.
- Type structure.
- Pattern structure.
- Relevant attributes and metadata.

It must remain domain-neutral.

The AST must not embed:

- LLVM-specific types.
- Vendor-specific QPU topology.
- Physical qubit mappings.
- Hardware calibration.
- Backend scheduling decisions.
- Target-specific resource allocations.

Quantum operations must enter "quantum::ir" through semantic lowering.

The AST should represent source meaning, not downstream implementation details.

18.1 Rust safety and version

All Rust implementation work associated with grammar integration must:

- Target Rust 1.97.1 or a compatible Rust 1.97 toolchain.
- Use Rust 2021.
- Avoid "unsafe".
- Avoid requiring "unsafe" in dependencies for Zamani's own implementation.
- Use explicit error handling.
- Preserve source spans.
- Avoid unchecked assumptions about input size or parser state.
- Handle malformed input without panicking where practical.
- Avoid artificial universal capacity limits.

The actual "Cargo.toml" remains authoritative for build configuration.

A README cannot correct an invalid Cargo manifest.

The compiler baseline must be verified against the repository's actual build configuration and CI.

---

19. Per-file completion contract

Every grammar file must be independently understandable and have its integration requirements specified before implementation.

A completed file must not require reopening merely because a downstream consumer was added.

This requires stable interfaces, not an unrealistic promise that files will never need changes when their own contract changes.

19.1 Required feature contract

Every grammar feature must document:

Field| Required information
Purpose| Exact responsibility
Status| Proposed, experimental, partial or stable
Owns| Rules and concepts exclusively owned
Does not own| Explicitly excluded responsibilities
Public rules| Exported grammar rules
Private rules| Internal grammar rules
Inputs| Consumed tokens and imported rules
Outputs| AST and semantic representations
Dependencies| Exact prerequisite files
Consumers| Known downstream files
AST contract| Node representation
Semantic contract| Meaning and validation
Type contract| Type interactions
Effect contract| Produced and required effects
Capability contract| Required capabilities
Resource contract| Resource interactions
Contract contract| Preconditions and postconditions
Policy contract| Policy interactions
Provenance contract| Information preserved
IR contract| Canonical IR destination
Quantum boundary| "quantum::ir" interaction, where applicable
HDL boundary| HDL semantic integration, where applicable
Backend boundary| Downstream realization
Diagnostics| Required errors and source spans
Positive tests| Accepted constructs
Negative tests| Rejected constructs
Boundary tests| Edge conditions
Scalability tests| Symbolic and large-input behavior
Compatibility| Versioning and migration
Integration| Exact upstream/downstream files
Completion criteria| Verifiable definition of DONE

Not every field requires a separate implementation in every file.

Where a field does not apply, explicitly state "NOT APPLICABLE" and explain why.

19.2 Dependency declaration

Each grammar feature must declare:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
IR_OWNER:
SPEC_OWNER:
TEST_OWNER:
COMPATIBILITY_OWNER:

These declarations must refer to real repository paths.

Do not invent paths or claim that an unimplemented file already exists.

19.3 Stable integration interfaces

A feature is independently complete when:

1. Its public grammar rules are documented.
2. Its token dependencies are documented.
3. Its AST contract is stable.
4. Its semantic contract is stable.
5. Its downstream interfaces are documented.
6. Its diagnostics are defined.
7. Its tests exist.
8. Its compatibility policy is defined.
9. Its integration points are verified.
10. Its conformance status is truthful.

A downstream consumer should implement against the declared contract instead of requiring arbitrary changes to the original grammar.

If the contract changes, the change must be versioned and reviewed.

---

20. Integration testing

The grammar test system must validate the complete language rather than isolated grammar fragments alone.

Maintain the existing examples where present and expand the suite with the following conceptual categories:

minimal.zm
classical.zm
generic.zm
quantum.zm
hybrid.zm
hdl.zm
poco-reaf.zm

ubuntu-reasoning.zm
ubuntu-learning.zm
ubuntu-adaptation.zm
ubuntu-knowledge.zm
ubuntu-contracts.zm
ubuntu-provenance.zm
ubuntu-policy.zm
ubuntu-uncertainty.zm
ubuntu-agents.zm
ubuntu-neural-symbolic.zm
ubuntu-sandbox.zm
ubuntu-simulation.zm
ubuntu-ffi.zm
ubuntu-patterns.zm
ubuntu-qml.zm
ubuntu-hybrid-ai-quantum.zm

These are proposed conformance fixtures.

Do not claim they exist until verified.

20.1 Test categories

tests/
├── lexical/
├── parser/
├── ast/
├── semantic/
├── types/
├── contracts/
├── resources/
├── capabilities/
├── effects/
├── provenance/
├── policies/
├── classical/
├── quantum/
├── hybrid/
├── hdl/
├── ai/
├── concurrency/
├── distributed/
├── networking/
├── interoperability/
├── metaprogramming/
├── sandbox/
├── simulation/
├── scalability/
├── portability/
├── compatibility/
├── diagnostics/
├── negative/
├── boundary/
└── integration/

Only create missing directories when the corresponding tests are ready to be owned and maintained.

20.2 Mandatory test classes

Every stable feature requires applicable:

- Positive tests.
- Negative tests.
- Boundary tests.
- AST tests.
- Semantic tests.
- Diagnostic tests.
- Compatibility tests.
- Scalability tests.
- Cross-domain tests.
- Determinism tests.

A test must verify behavior, not merely file existence or token recognition.

---

21. Mandatory cross-domain test

The final integration suite must include a program combining:

- Classical computation.
- Generic types.
- Tensor operations.
- Reasoning.
- Learning.
- Controlled adaptation.
- Knowledge.
- Evidence.
- Uncertainty.
- AI agents.
- Concurrency.
- Quantum operations.
- Measurement.
- Hybrid control.
- Resource requirements.
- Capability requirements.
- Effects.
- Contracts.
- Policies.
- Provenance.
- Simulation.
- Hardware intent.

The expected validation path is:

Source
  |
  v
Lexer
  |
  v
Parser
  |
  v
AST
  |
  v
Structural validation
  |
  v
Semantic model
  |
  v
Type / effect / capability analysis
  |
  v
Resource analysis
  |
  v
Contract and policy analysis
  |
  v
Provenance
  |
  +--------------------+
  |                    |
  v                    v
Classical IR       quantum::ir
  |                    |
  +----------+---------+
             |
             v
     Execution planning
             |
             v
     Target negotiation
             |
             v
       Lowering and routing
             |
             v
        Scheduling and
        resilience planning

Every stage must be verified against actual implementation.

A successful parser test alone is insufficient.

---

22. Scalability and adversarial input testing

Scalability must be tested without introducing a new artificial language ceiling.

The test system must evaluate:

- Large source files.
- Deeply nested expressions.
- Large declaration sets.
- Large generic types.
- Large symbolic resource requirements.
- Large quantum operation sequences.
- Large tensor descriptions.
- Large module graphs.
- Large patterns.
- Extensive diagnostic output.
- Malformed and adversarial input.
- Incremental parsing where supported.

Tests must distinguish:

1. Language-level restrictions.
2. Implementation representation limits.
3. Available memory.
4. Configurable resource budgets.
5. Target capabilities.
6. Operational limits.

A resource budget must produce an explicit diagnostic.

It must not silently change the accepted language or truncate semantics.

No finite test suite can prove infinite scalability. Production claims must therefore be based on architecture, complexity analysis, configurable resource policies and progressively larger empirical tests.

---

23. Determinism and reproducibility

Grammar processing must be deterministic for the same:

- Source input.
- Language version.
- Grammar version.
- Lexical configuration.
- Dialect configuration.

Parsing must not depend on:

- Hardware availability.
- Wall-clock time.
- Randomness.
- Network state.
- Filesystem state.
- Runtime state.
- Target selection.

Where semantic analysis requires environmental information, that information must be explicitly supplied and represented in the compilation context.

Reproducibility requirements for compilation and execution must be defined separately from deterministic parsing.

---

24. Compatibility and evolution

Every public language construct requires a compatibility policy.

The grammar architecture must define:

- Language edition.
- Grammar version.
- Lexer version.
- AST version.
- Semantic model version.
- IR version.
- Dialect version.
- Deprecation.
- Migration.
- Compatibility diagnostics.

A new construct must not silently change the meaning of an existing program.

Experimental syntax must be distinguishable from stable syntax.

Historical syntax must not become accepted merely because it remains documented.

Language self-versioning must be controlled by the language/toolchain compatibility model.

A program must not arbitrarily rewrite the language version under which it is compiled.

---

25. Documentation hierarchy

The intended documentation hierarchy is:

grammar/README.md
    |
    +-- DESIGN.md
    |      |
    |      +-- Architecture and ownership
    |
    +-- Zamani.g4
    |      |
    |      +-- Canonical ANTLR root
    |
    +-- antlr/
    |      |
    |      +-- Lexer and parser composition
    |
    +-- specification/
    |      |
    |      +-- Normative language specifications
    |
    +-- spec/
    |      |
    |      +-- Formal contracts and invariants
    |
    +-- grammar.md
    |      |
    |      +-- Implementation conformance
    |
    +-- Zamani-Grammar.md
    |      |
    |      +-- Extended and historical reference
    |
    +-- Domain directories
    |      |
    |      +-- Feature-specific syntax
    |
    +-- tests/
           |
           +-- Executable conformance evidence

The documentation must distinguish intended architecture from implemented functionality.

---

26. Implementation workflow

Work must proceed dependency-first and independent-file-first.

Do not implement hundreds of files without establishing their ownership contracts.

Phase 0 — Repository audit and authority

1. Inspect every existing grammar authority.
2. Inspect the lexer and parser.
3. Inspect the AST.
4. Inspect semantic analysis.
5. Inspect canonical IR ownership.
6. Inspect the quantum IR.
7. Inspect existing tests.
8. Identify conflicting or duplicated constructs.
9. Establish the authoritative specifications.
10. Record actual implementation status.

Files:

grammar/README.md
grammar/DESIGN.md
grammar/grammar.md
grammar/Zamani-Grammar.md

Phase 1 — Lexical foundation

Establish:

grammar/lexer/
grammar/antlr/ZamaniLexer.g4
src/lexer.rs

Complete token ownership, reserved words, operators, literals, diagnostics and conformance.

Phase 2 — Universal grammar foundation

Establish or strengthen:

grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/declarations/
grammar/functions/
grammar/modules/
grammar/antlr/ZamaniParser.g4

No domain-specific extension may establish a competing universal construct.

Phase 3 — Semantic contracts

Strengthen:

grammar/resources/
grammar/effects/
grammar/validation/
grammar/security/
grammar/spec/
grammar/specification/

Establish requirements, capabilities, effects, contracts, policies and provenance.

Phase 4 — Concurrency and execution

Strengthen:

grammar/concurrency/
grammar/execution/
grammar/memory/
grammar/compile/

Integrate actors, tasks, scheduling intent, simulation and controlled adaptation.

Phase 5 — Classical and data

Strengthen:

grammar/classical/
grammar/data/
grammar/types/

Phase 6 — Quantum and hybrid

Strengthen:

grammar/quantum/
grammar/hybrid/

Integrate through the existing canonical "quantum::ir".

Phase 7 — HDL and hardware

Strengthen:

grammar/hdl/
grammar/hardware/

Phase 8 — UBUNTU-derived AI

Strengthen:

grammar/ai/
grammar/concurrency/
grammar/data/
grammar/validation/

Integrate reasoning, learning, knowledge, uncertainty, explainability, evidence, provenance and agents.

Phase 9 — Distributed and networking

Strengthen:

grammar/distributed/
grammar/networking/

Phase 10 — Interoperability and dialects

Strengthen:

grammar/interoperability/
grammar/dialects/
grammar/compatibility/

Phase 11 — Metaprogramming

Strengthen:

grammar/macros/
grammar/metaprogramming/

Phase 12 — Conformance and production acceptance

Complete:

grammar/tests/
grammar/validation/
grammar/compatibility/

The phase order describes dependency priorities. Independent work may proceed in parallel only after its contracts are established.

---

27. Required integration details before a file is started

Before implementing a file, establish its complete dependency contract.

For example, a quantum measurement grammar file must identify:

FILE:
    grammar/quantum/measurement.g4

PURPOSE:
    Define quantum measurement source syntax.

DEPENDS_ON:
    Existing canonical quantum grammar.
    Existing lexer tokens.
    Existing expression grammar.
    Quantum normative specification.

EXPORTS:
    Its documented public measurement rule.

CONSUMED_BY:
    Existing quantum grammar composition.
    Canonical parser composition.

AST_OWNER:
    Existing frontend AST.

SEMANTIC_OWNER:
    Quantum semantic analysis.

IR_OWNER:
    quantum::ir.

SPEC_OWNER:
    Quantum specification.

TEST_OWNER:
    Quantum measurement conformance tests.

DOES_NOT_OWN:
    Physical measurement hardware.
    Calibration.
    Device routing.
    Physical qubit assignment.
    Backend scheduling.
    QEC implementation.

The exact paths must be verified before being committed.

This prevents speculative integration contracts from becoming false repository documentation.

---

28. Production acceptance criteria

The grammar directory must not be declared production-ready merely because its directory tree looks complete.

Production acceptance requires all applicable conditions below.

28.1 Authority

- One canonical language architecture.
- One canonical ANTLR root.
- One canonical lexer composition.
- One canonical parser composition.
- No contradictory normative specifications.
- No undocumented competing syntax authorities.

28.2 Frontend

- Lexer conformance.
- Parser conformance.
- AST conformance.
- Source-span preservation.
- Error recovery.
- Stable diagnostics.
- Domain-neutral AST boundaries.

28.3 Semantics

- Defined type behavior.
- Defined effect behavior.
- Defined resource behavior.
- Defined capability behavior.
- Defined contract behavior.
- Defined policy behavior.
- Defined provenance behavior.

28.4 Domain integration

- Classical.
- Quantum.
- Hybrid.
- HDL.
- Hardware.
- AI.
- Data.
- Concurrency.
- Distributed.
- Networking.
- Interoperability.

Each domain must have a truthful implementation status.

28.5 Canonical IR

- Classical semantic operations have a documented IR destination.
- Quantum operations lower into "quantum::ir".
- No duplicate quantum frontend IR is introduced.
- IR versioning is defined.
- Unsupported constructs produce diagnostics.

28.6 Safety

- Rust 2021.
- Rust 1.97.1-compatible.
- No "unsafe" in Zamani's implementation.
- No grammar actions executing arbitrary Rust.
- No parser-driven filesystem or network operations.
- No unbounded uncontrolled resource consumption.
- Explicit resource-budget diagnostics.

28.7 Scalability

- No artificial universal hardware ceilings.
- No fixed domain enumeration.
- Symbolic resource requirements.
- Configurable implementation budgets.
- Tested behavior at progressively larger scales.
- Explicit representation-limit behavior.

28.8 Compatibility

- Versioned specifications.
- Stable interfaces.
- Deprecation rules.
- Migration requirements.
- Regression tests.

28.9 Evidence

- Positive tests.
- Negative tests.
- Boundary tests.
- Cross-domain tests.
- Scalability tests.
- Compatibility tests.
- Determinism tests.
- Hard-coding audit.

Only after these conditions are satisfied should a feature or subsystem receive a production-stable status.

---

29. Hard-coding audit

Every feature implementation must be audited for artificial limits.

Search for:

- Fixed resource counts.
- Fixed device counts.
- Fixed quantum operation enumerations.
- Fixed hardware widths.
- Fixed tensor ranks.
- Fixed network sizes.
- Fixed thread limits.
- Fixed module counts.
- Fixed source-size assumptions.
- Unbounded recursion.
- Unchecked integer conversions.
- Silent truncation.
- Arbitrary parser depth limits.
- Hidden fallback behavior.

A configurable safety budget is permitted when its purpose, scope and diagnostic behavior are documented.

A configurable operational budget must not be confused with a universal language limit.

A physical target's actual limit is valid target information, not a universal language ceiling.

---

30. Definition of DONE for this README

This README is complete as a navigation and architecture contract when:

- The authority hierarchy is explicit.
- Existing canonical files are preserved.
- The UBUNTU integration boundaries are explicit.
- Every domain has a defined owner.
- The quantum IR boundary is explicit.
- The resource and capability architecture is explicit.
- Rust and safety requirements are explicit.
- Scalability requirements are explicit.
- Integration requirements are explicit.
- Production acceptance criteria are explicit.
- The document does not claim unimplemented functionality is complete.
- Links to normative documents resolve to actual repository files.
- Proposed files are not represented as existing files.

This README does not certify the grammar directory as production-ready. It establishes the conditions under which production readiness can be demonstrated.

---

31. Final architectural principle

Zamani must describe computational meaning, not the limitations of today's hardware.

The source program expresses:

Intent
Types
Operations
Requirements
Capabilities
Constraints
Effects
Contracts
Policies
Provenance

The compiler and runtime determine:

Feasibility
Specialization
Resource negotiation
Lowering
Routing
Scheduling
Recovery
Target realization

UBUNTU contributes reasoning, knowledge, learning, adaptation, uncertainty, evidence, explainability, contracts, policies, agents and neural-symbolic composition.

These capabilities must share Zamani's universal semantic foundation.

Quantum computation, classical computation, HDL, AI, distributed computing and future domains must remain interoperable within one language.

The objective is not a grammar that knows every possible machine. The objective is a language whose meaning survives changes in machines, scales, technologies and computational paradigms.

That is the foundation of Zamani POCO-REAF.