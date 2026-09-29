Zamani Grammar

Path: "grammar/README.md"
Repository: "Benwellonedge28/Zamani"
Primary branch: "main"
Language: Zamani
Edition: Rust 2021
Required Rust baseline: Rust 1.97.1
Safety policy: Production compiler implementation MUST use safe Rust.
Primary portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF).
Scalability objective: From the smallest meaningful computation to arbitrarily large computations, subject only to program semantics, representational constraints, declared requirements, and resources actually available.

---

1. Purpose

The "grammar/" directory defines the source-language contract of Zamani.

It is the language architecture and conformance boundary connecting source syntax to the complete compiler and execution system.

It is not merely a collection of ANTLR grammar files.

It establishes how Zamani source code is:

1. Lexically interpreted.
2. Parsed into syntactic structures.
3. Represented in the frontend AST.
4. Structurally validated.
5. Resolved and type-checked.
6. Analyzed for effects, ownership, capabilities, and resources.
7. Converted into canonical semantic representations.
8. Lowered into intermediate representations.
9. Optimized and specialized.
10. Routed, scheduled, and made resilient where applicable.
11. Connected to ZQN, HAL, and target-specific implementations.
12. Executed across available computing environments.

The intended architecture is:

Zamani source
      |
      v
Lexical specification
      |
      v
Rust lexer / ANTLR lexical contract
      |
      v
Parser / grammar conformance
      |
      v
Frontend AST
      |
      v
Structural validation
      |
      v
Name and module resolution
      |
      v
Type / effect / ownership analysis
      |
      v
Resource / capability / constraint analysis
      |
      v
Canonical semantic model
      |
      v
Canonical IR
      |
      +-------------------+
      |                   |
      v                   v
 Classical IR         quantum::ir
      |                   |
      +---------+---------+
                |
                v
        Hardware / HDL IR
                |
                v
       Optimization and lowering
                |
                v
       Routing / scheduling
                |
                v
       Resilience / recovery
                |
                v
               ZQN
                |
                v
               HAL
                |
                v
       Target realization
                |
       +--------+---------+
       |        |         |
       v        v         v
      CPU      GPU       FPGA
                         QPU
                    Distributed
                    Embedded
                    Future targets

The grammar describes portable source-level meaning and structure.

It MUST NOT encode today's hardware as tomorrow's language.

---

2. Authority and Document Responsibilities

The existing files MUST be retained. They have different responsibilities and MUST NOT become competing authorities.

2.1 Authority hierarchy

File or subsystem| Authority and responsibility
"DESIGN.md"| Normative architectural rules for the grammar subsystem
"specification/"| Authoritative human-readable language specification
"spec/"| Formal feature contracts, invariants, and machine-checkable requirements
"lexer/"| Canonical lexical contracts and token definitions
"Zamani.g4"| Canonical ANTLR composition entry point, subject to actual ANTLR/toolchain conformance
"src/lexer.rs"| Executable Rust lexical implementation
"src/parser.rs"| Executable Rust parsing implementation
"src/ast/"| Frontend AST representation
Semantic analysis| Authoritative interpretation and validation of program meaning
Canonical IR| Authoritative compiler representation of validated computation
"quantum::ir"| Canonical quantum IR boundary
"grammar.md"| Generated implementation-conformance reference
"Zamani-Grammar.md"| Extended, historical, proposed, and experimental design reference
"reference/"| User-facing reference generated from accepted specifications
"tests/"| Executable conformance evidence

No document may silently override a higher-authority contract.

When two sources disagree, the disagreement MUST be recorded and resolved through the feature lifecycle. Implementations MUST NOT choose whichever definition is most convenient.

2.2 What each document must not become

"README.md" MUST NOT become a second normative language specification.

"DESIGN.md" MUST NOT become an inventory of every implemented token.

"Zamani.g4" MUST NOT become a second semantic analyzer.

"grammar.md" MUST NOT claim that planned syntax is implemented.

"Zamani-Grammar.md" MUST NOT automatically authorize new language syntax.

"reference/" MUST NOT introduce independent language rules.

"spec/" MUST NOT duplicate complete implementations.

The same language concept MUST have one authoritative definition and traceable references elsewhere.

---

3. Production-Readiness Definition

A grammar feature is not production-ready merely because:

- a grammar rule exists;
- the lexer recognizes a keyword;
- the parser accepts an example;
- a documentation page describes it;
- an AST variant exists;
- a backend contains a partial implementation.

Production readiness requires a complete, verified feature contract.

The required feature pipeline is:

Proposal
   |
   v
Specification
   |
   v
Lexical contract
   |
   v
AST contract
   |
   v
Grammar contract
   |
   v
Lexer implementation
   |
   v
Parser implementation
   |
   v
Structural validation
   |
   v
Semantic implementation
   |
   v
Canonical semantic representation
   |
   v
IR mapping
   |
   v
Compiler integration
   |
   v
Runtime / backend integration
   |
   v
Conformance tests
   |
   v
Compatibility validation
   |
   v
Production acceptance

Every applicable stage MUST have an identified owner, defined interface, and completion criterion.

A feature that does not reach the required downstream stages MUST be marked accordingly.

3.1 Feature status

The following statuses are mandatory:

Status| Meaning
"PROPOSED"| Suggested but not normatively accepted
"SPECIFIED"| Normative contract exists
"PARTIALLY_IMPLEMENTED"| Some implementation stages exist
"IMPLEMENTED"| Required implementation stages exist
"STABLE"| Complete conformance and compatibility requirements pass
"EXPERIMENTAL"| Explicitly provisional feature
"DEPRECATED"| Supported during a documented migration period
"PLANNED"| Accepted future work without a complete implementation
"HISTORICAL"| Retained for historical reference
"REJECTED"| Reviewed and explicitly excluded

Status MUST NOT be inferred from a directory existing or a keyword appearing in source code.

---

4. Fundamental Language Objective

Zamani is intended to express multiple computing paradigms through one coherent programming language.

The language MUST support the architectural integration of:

- Classical computing.
- Systems programming.
- Embedded computing.
- Scientific and numerical computing.
- Symbolic mathematics.
- Linear algebra.
- Vector, matrix, and tensor computation.
- High-performance computing.
- Parallel and concurrent computation.
- Distributed computing.
- Quantum computing.
- Hybrid quantum-classical computation.
- Quantum error-corrected computation.
- Hardware description.
- Hardware/software co-design.
- FPGA and ASIC-oriented computation.
- GPU and accelerator computation.
- AI and machine learning.
- Data processing.
- Networking.
- Security and cryptography.
- Edge and cloud computing.
- Nano-oriented computation.
- Temporal and multi-timeline computation.
- Sankofa memory and knowledge constructs.
- Metaprogramming.
- Interoperability.
- Domain-specific extensions.
- Future computational paradigms.

These are domains of one language, not unrelated languages connected through ad hoc syntax.

They MUST share common language foundations:

- Lexical rules.
- Identifiers and names.
- Source locations.
- Expressions.
- Types.
- Declarations.
- Statements.
- Functions.
- Modules.
- Effects.
- Ownership and memory semantics.
- Resource requirements.
- Capability requirements.
- Diagnostics.
- Versioning.
- Compatibility.
- Semantic validation.
- Canonical intermediate representations.

Domain-specific syntax MUST reuse universal foundations wherever possible.

A domain MUST NOT independently redefine universal expressions, identifiers, source spans, or type semantics.

---

5. POCO-REAF: Portability Contract

POCO-REAF means:

Program Once → Compile Once → Run Everywhere → Anywhere → Forever.

The same source-level program should remain meaningful across different hardware, execution environments, scales, and future targets without requiring the programmer to rewrite the algorithm merely because the available hardware changes.

This is a language and compiler architecture objective, not a guarantee that every program can execute on every machine.

The following conditions MUST remain distinct:

Lexically valid
      !=
Syntactically valid
      !=
Semantically valid
      !=
Compilable
      !=
Target compatible
      !=
Resource feasible
      !=
Runtime available
      !=
Successfully executed

For example, a program requiring quantum measurement, a particular memory capability, or a minimum resource quantity can be valid even when a particular target cannot satisfy those requirements.

The compiler/runtime MAY:

- Select a compatible target.
- Select an alternative backend.
- Decompose operations.
- Perform legal specialization.
- Distribute work.
- Apply supported routing.
- Schedule computation.
- Apply resilience strategies.
- Use error correction.
- Use a compatible simulator.
- Request additional resources.
- Wait for resource availability.
- Reject an incompatible target with a precise diagnostic.

It MUST NOT silently alter program semantics to make an unsupported target appear compatible.

5.1 Source portability

Portable source expresses program meaning, requirements, constraints, and permitted alternatives.

It SHOULD avoid binding the algorithm to:

- Particular CPU models.
- Particular GPU models.
- Particular QPU vendors.
- Physical qubit identifiers.
- Fixed FPGA layouts.
- Specific memory-bank identifiers.
- Particular network nodes.
- Particular execution schedules.
- Vendor-specific instruction encodings.

Target-specific realization MAY exist in explicitly declared deployment, interoperability, backend, or hardware-specific contracts.

Such realization MUST NOT become an implicit universal language restriction.

5.2 Compilation portability

Compilation portability requires:

1. Stable source semantics.
2. Defined language versions.
3. Explicit target capabilities.
4. Reproducible build contracts where requested.
5. Defined specialization rules.
6. Defined fallback behavior.
7. Explicit unsupported-target diagnostics.
8. No silent semantic weakening.
9. Versioned interoperability boundaries.
10. A documented artifact compatibility contract.

A compiled artifact is portable only to the extent guaranteed by its declared artifact and runtime contract.

The phrase "Compile Once" MUST NOT be interpreted as requiring every native binary to execute on every instruction set without an appropriate runtime, virtual machine, translation layer, or compatible execution environment.

---

6. Scalability: From Atom to Everywhere

Zamani MUST NOT impose artificial universal capacity ceilings.

The architecture MUST permit computation to grow from the smallest useful unit to arbitrarily large computation, subject to actual resource availability and program semantics.

Conceptual scaling includes:

One value
   |
One operation
   |
One function
   |
One process
   |
One qubit
   |
One device
   |
Multiple devices
   |
Heterogeneous systems
   |
Clusters
   |
Distributed systems
   |
Cloud and HPC
   |
Future computational systems

This applies to:

- Source files.
- Declarations.
- Functions.
- Modules.
- Expressions.
- Quantum registers.
- Logical qubits.
- Physical resources.
- CPU cores.
- Threads.
- GPU devices.
- FPGA resources.
- QPUs.
- Processes.
- Tasks.
- Agents.
- Memory.
- Storage.
- Registers.
- Vector widths.
- Tensor ranks.
- Tensor dimensions.
- Network endpoints.
- Distributed nodes.
- Timelines.
- Hardware modules.
- Connections.
- Computational operations.

6.1 Prohibited universal limits

The grammar, specification, AST, and compiler architecture MUST NOT establish artificial universal limits such as:

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

The same prohibition applies to fixed enumerations of physical resources masquerading as universal language constructs.

For example:

Qubit0
Qubit1
...
Qubit1023

MUST NOT define the universal quantum resource model.

6.2 Resource availability is not a language limit

The implementation MAY have practical limits imposed by:

- Available memory.
- Integer representation.
- Address-space representation.
- Compiler execution budgets.
- Runtime execution budgets.
- Target capabilities.
- Backend restrictions.
- External service constraints.
- Explicit user policies.

These MUST be represented as implementation or environment constraints, not silently promoted into universal language limits.

When an implementation cannot process a valid program because of an actual limitation, it MUST report that limitation accurately.

It MUST NOT pretend that the program is invalid merely because the current machine is insufficient.

6.3 No promise of mathematical infinity

"Scale to infinity" means no arbitrary fixed language-level ceiling and no architecture that assumes today's machine size is the maximum.

It does not mean that finite hardware, finite memory, finite representations, or finite execution time can execute mathematically infinite work.

Zamani MUST distinguish unbounded language expressiveness from finite implementation resources.

---

7. Hard-Coding Policy

"Nothing must be hard coded" means no artificial implementation-specific capacity is allowed to become a universal language restriction.

It does not prohibit constants, literals, fixed mathematical definitions, or program-defined requirements.

Valid:

let n = 1024;
let data = allocate(n);

The value "1024" is program data.

Valid:

Tensor<Float, [1024, 1024]>

The dimensions are part of the declared program semantics.

Invalid architecture:

const MAX_QUBITS: usize = 1024;

when this restricts the maximum quantum computation representable by Zamani.

Likewise, the grammar MUST NOT impose an artificial maximum matrix size, register width, node count, timeline count, or tensor rank.

7.1 Required hard-coding audit

Every feature MUST be checked for:

- Fixed resource capacities.
- Fixed device counts.
- Fixed hardware identifiers.
- Fixed quantum gate lists used as universal restrictions.
- Fixed vendor assumptions.
- Fixed topology assumptions.
- Fixed register widths.
- Fixed tensor dimensions.
- Fixed network sizes.
- Fixed thread counts.
- Fixed execution-device counts.
- Fixed timeline counts.
- Fixed physical-qubit enumerations.
- Hidden assumptions in parser buffers or AST representations.
- Integer conversions that silently truncate.
- Overflow behavior.
- Allocation assumptions.
- Unbounded recursion.
- Fixed nesting limits.
- Fixed serialization sizes.

A suspicious constant is not automatically prohibited.

Its meaning MUST be classified as:

1. Language invariant.
2. Program-defined value.
3. Implementation safety bound.
4. Target capability.
5. Explicit resource policy.
6. Artificial universal restriction.

Only the last category is prohibited.

---

8. Requirement, Capability, Constraint, Preference, Hint, and Realization

These concepts MUST remain separate throughout the language.

8.1 Semantic requirement

Example:

requires qubits >= n

This expresses a program requirement.

It does not identify physical qubits.

8.2 Capability requirement

Example:

requires capability("quantum.measurement")

This requires a semantic capability, not a vendor-specific implementation.

8.3 Constraint

Example:

requires memory >= required_memory

This expresses a condition that a realization must satisfy.

8.4 Preference

Example:

prefer accelerator("quantum")

A preference may guide implementation selection but MUST NOT override correctness or mandatory requirements.

8.5 Hint

A hint may guide optimization, scheduling, or resource selection.

It MUST NOT silently change program semantics.

8.6 Implementation realization

Example:

logical operation
       |
       v
selected physical resource

Physical mapping belongs to downstream implementation and realization contracts.

It MUST NOT be confused with portable source semantics.

---

9. Canonical Repository Integration

The "grammar/" directory MUST integrate with existing repository files and directories.

It MUST expand the existing architecture rather than create parallel authorities.

9.1 Root files

Existing file| Required responsibility
"README.md"| Navigation, authority hierarchy, integration, contribution and completion rules
"DESIGN.md"| Normative architecture
"Zamani.g4"| Canonical ANTLR composition entry point
"grammar.md"| Implementation-conformance reference
"Zamani-Grammar.md"| Extended and historical design reference

No unnecessary renaming is permitted.

9.2 Existing language domains

Existing directories MUST be populated and integrated according to their ownership.

Directory| Responsibility
"antlr/"| Existing ANTLR tooling and artifacts; no competing grammar authority
"core/"| Universal language foundations
"lexer/"| Lexical contracts
"expressions/"| Universal expression syntax
"types/"| Type syntax
"declarations/"| Declaration syntax
"statements/"| Statement syntax
"functions/"| Function and callable syntax
"modules/"| Module, import, export, and package syntax
"effects/"| Effect declarations and handlers
"memory/"| Ownership, borrowing, allocation, address spaces, and memory intent
"concurrency/"| Tasks, parallelism, synchronization, and deterministic concurrency
"classical/"| Classical computation
"quantum/"| Quantum source syntax and semantic contracts
"hybrid/"| Classical/quantum integration
"hdl/"| Hardware description and verification syntax
"hardware/"| Target-independent hardware intent
"resources/"| Requirements, constraints, budgets, and preferences
"distributed/"| Distributed computation
"ai/"| AI and machine-learning syntax contracts
"data/"| Data, schemas, tensors, and provenance
"networking/"| Network and communication constructs
"security/"| Security and cryptographic intent
"compile/"| Compilation and build intent
"execution/"| Execution policies and lifecycle
"interoperability/"| External formats and language boundaries
"dialects/"| Controlled language extensions
"macros/"| Macro syntax and expansion contracts
"metaprogramming/"| Reflection and compile-time constructs
"compatibility/"| Versions and migration contracts
"validation/"| Automated conformance and architectural validation
"spec/"| Formal contracts
"specification/"| Normative human-readable specification
"reference/"| Generated language reference
"tests/"| Executable conformance tests

New subdirectories MAY be introduced when a real ownership or maintainability requirement exists.

Existing files MUST be inspected before adding replacements.

A directory MUST NOT contain duplicate files that independently claim the same authority.

---

10. ANTLR Integration

10.1 "Zamani.g4"

"Zamani.g4" is retained as the canonical ANTLR composition entry point.

Its responsibility is composition, not implementation of every language feature.

It MUST define or coordinate the root language entry, universal declarations, statements, expressions, types, and end-of-input behavior according to the selected ANTLR architecture.

It MUST NOT become a monolithic catalogue of all domain-specific operations.

It MUST NOT define a second semantic model.

It MUST NOT enumerate every possible quantum gate, mathematical operation, vendor instruction, hardware resource, or AI framework.

10.2 ANTLR composition must be verified

The actual root grammar currently documents a composition involving:

ZamaniLexer.g4
ZamaniParser.g4

and downstream domain grammars.

The existence of a documented import or composition relationship MUST NOT be treated as proof that all referenced grammar files exist, are generated, or are accepted by the configured ANTLR toolchain.

Before a grammar component is declared integrated, verify:

1. The referenced file exists.
2. The grammar declaration is valid.
3. Its imports resolve.
4. Token vocabulary is consistent.
5. No duplicate token authority exists.
6. Parser rules are reachable.
7. No unintended ambiguity exists.
8. Generated artifacts are reproducible.
9. Rust frontend behavior is conformant.
10. The relevant tests pass.

10.3 Modular grammar rules

Domain grammars MUST use shared syntax contracts.

A domain grammar MUST NOT create an independent definition of universal expression precedence, universal identifier syntax, or universal type syntax.

If ANTLR grammar composition requires generated intermediates or a combined grammar, the build configuration MUST make that relationship explicit.

The source of truth MUST remain identifiable.

---

11. Rust Frontend Integration

The grammar contract MUST be consistent with the actual Rust frontend.

The current repository contains:

src/lexer.rs
src/parser.rs
src/ast/mod.rs

These are executable implementation files, not automatically generated implementations of every rule in the broad language design.

11.1 Rust version and edition

The required development baseline is Rust 1.97.1 with Rust 2021 edition.

The repository's actual "Cargo.toml" MUST remain the authoritative build configuration.

The "rust-version" field MUST use valid Cargo syntax and MUST agree with the documented supported baseline.

CI MUST verify the selected toolchain rather than relying exclusively on this README.

11.2 Safe Rust

Production compiler implementation MUST NOT use:

unsafe

Rust's "unsafe" token or an unsafe-related language construct is a separate source-language design question.

It MUST NOT be confused with the prohibition on unsafe Rust implementation code.

If the Zamani language exposes unsafe constructs, their semantics, capability restrictions, validation, and compatibility MUST be specified independently.

11.3 Lexer contract

"src/lexer.rs" MUST be checked against the canonical lexical specification.

The lexer must define:

- Token identity.
- Keyword recognition.
- Identifier rules.
- Unicode behavior.
- Numeric literals.
- String literals.
- Character literals.
- Escape sequences.
- Comments.
- Operators.
- Punctuation.
- Quantum literals.
- Source spans.
- Invalid input behavior.
- Error recovery.
- Determinism.

A token MUST NOT be independently assigned incompatible meanings in the specification, ANTLR grammar, and Rust lexer.

The current lexer contains a substantial token taxonomy and keyword registry. It MUST be audited for:

- Duplicate conceptual tokens.
- Unused tokens.
- Keywords without parser support.
- Parser constructs without lexical support.
- Operator collisions.
- Inconsistent literal handling.
- Missing Unicode cases.
- Missing source-span behavior.

For example, "Question" and "QuestionMark", or "Ampersand" and "BitAnd", MUST be reconciled according to their actual distinct lexical meanings rather than retained as accidental duplicates.

11.4 Parser contract

"src/parser.rs" is currently a recursive-descent/Pratt parser.

Its actual supported constructs MUST be compared with the normative syntax specification.

The parser MUST define:

- Precedence.
- Associativity.
- Expression parsing.
- Declaration parsing.
- Statement parsing.
- Type parsing.
- Pattern parsing.
- Error recovery.
- Source spans.
- Invalid syntax behavior.
- Deterministic parsing.

A construct being present in "Zamani.g4" does not establish that the Rust parser supports it.

Likewise, a Rust parser branch does not automatically establish that the construct is a stable language feature.

11.5 Frontend AST contract

"src/ast/mod.rs" currently uses explicit Rust structures and enums, including domain-specific statement variants.

The architecture MUST evolve without prematurely requiring every new syntax construct to receive a dedicated AST enum variant.

Each feature MUST identify whether its representation belongs in:

- An existing domain-neutral AST node.
- An extended existing AST node.
- A new AST node.
- A semantic annotation.
- A canonical IR representation.

The decision MUST be documented before implementation.

AST nodes MUST preserve sufficient source information for diagnostics, tracing, transformations, and compatibility.

Domain-specific semantics MUST NOT be hidden inside arbitrary strings when structured data is necessary.

The AST MUST remain a frontend representation, not a duplicate of the canonical IR.

---

12. Canonical AST and Semantic Mapping

Every accepted grammar construct MUST have a predetermined mapping.

The required traceability chain is:

Specification section
       |
       v
Grammar rule
       |
       v
Token contract
       |
       v
Frontend AST node
       |
       v
Structural validation
       |
       v
Semantic model
       |
       v
Canonical IR operation
       |
       v
Compiler consumer
       |
       v
Runtime/backend consumer

No feature may be implemented by adding syntax first and deciding its meaning later.

12.1 Domain-neutral foundations

The frontend AST MUST preserve shared concepts across domains.

For example, a function call, a quantum operation, and a classical mathematical operation may share universal expression structure while receiving different semantic interpretation.

This avoids creating independent language frontends for every computing domain.

12.2 Semantic authority

The semantic analyzer owns:

- Name resolution.
- Type checking.
- Effect checking.
- Ownership validation.
- Capability checking.
- Resource requirement validation.
- Domain-specific semantic validation.
- Compile-time evaluation rules.
- Legality of transformations.
- Cross-domain compatibility.

The parser MUST NOT implement these responsibilities through ad hoc syntax restrictions.

12.3 Source provenance

Every AST and semantic construct that originates from source MUST preserve traceable source information.

The source-span contract MUST cover:

- File identity.
- Start position.
- End position.
- Unicode handling.
- Byte-position interpretation.
- Generated-source provenance.
- Macro expansion provenance.
- Diagnostic mapping.

The implementation MUST define how source positions remain valid through parsing and transformation.

---

13. Canonical IR Integration

The canonical IR is the boundary between frontend language meaning and backend realization.

The grammar subsystem MUST NOT create a competing canonical IR.

Quantum computation MUST use:

Zamani source
      |
      v
Domain-neutral frontend AST
      |
      v
Semantic analysis
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
Target realization

13.1 Quantum invariants

The grammar MUST NOT:

- Create a second quantum IR.
- Make a fixed gate enumeration the universal operation model.
- Define physical qubit numbering as universal language semantics.
- Encode device calibration.
- Encode physical topology as a parser invariant.
- Perform routing.
- Perform scheduling.
- Implement QEC.
- Implement noise semantics.
- Implement HAL behavior.

Quantum operations MUST support extensible semantic identity, including names, namespaces, operands, parameters, results, attributes, modifiers, effects, capabilities, and source provenance where applicable.

The syntax MUST permit extensible operations without requiring every new operation to modify the core grammar.

For example:

apply H to q
apply custom_gate to q
apply vendor.operation to q
apply operation(parameter) to q0, q1

These must be interpreted through a defined operation-specification contract rather than a universal finite gate list.

13.2 Classical and hardware representations

Classical computation and HDL/hardware intent MAY have distinct canonical IR components.

Those components MUST share the same semantic contracts for:

- Types.
- Effects.
- Source provenance.
- Resource requirements.
- Capability requirements.
- Correctness.
- Interoperability.

The existence of domain-specific IR MUST NOT fragment the source language into incompatible semantic systems.

---

14. Computing-Domain Integration

14.1 Classical computing

Classical syntax MUST support general computation without turning every library operation into a keyword.

Existing mathematical capabilities—including vectors, matrices, tensors, symbolic mathematics, calculus, statistics, numerical methods, FFT, signal processing, and optimization—MUST be retained through appropriately typed operations, intrinsics, and libraries.

14.2 Quantum computing

Quantum syntax MUST describe operations and semantic requirements independently of physical device limitations.

It MUST support the required design space for:

- Qubits and registers.
- Quantum states.
- Operations.
- Parameterized operations.
- Controls.
- Adjoint operations.
- Measurement.
- Reset.
- Barriers.
- Classical feed-forward.
- Dynamic control.
- Observables.
- Channels.
- Noise.
- Error correction.
- Logical operations.
- Pulse intent.
- Circuits.
- Kernels.
- Resource requirements.

A construct MUST NOT become stable until its AST, semantic, and "quantum::ir" contracts are complete.

14.3 Hybrid computation

Hybrid syntax MUST support the interaction between classical and quantum computation.

The architecture MUST define:

- Classical-to-quantum data flow.
- Measurement results.
- Quantum-to-classical boundaries.
- Feed-forward.
- Synchronization.
- Shared data.
- Resource requirements.
- Execution semantics.

A hybrid program MUST remain one Zamani program.

14.4 HDL and hardware/software co-design

HDL MUST express hardware structure and behavior without introducing universal hardware capacity restrictions.

The language MUST distinguish:

- Hardware intent.
- Logical design.
- Parameterized widths.
- Signals.
- Ports.
- Registers.
- Timing.
- Clocking.
- Reset.
- State machines.
- Pipelines.
- Memory behavior.
- Verification properties.
- Synthesis intent.
- Simulation intent.
- Physical realization.

A declaration of a particular width is valid when it is program semantics.

A universal maximum width imposed by the language is not.

14.5 AI and data

AI and data constructs MUST integrate with universal types, expressions, modules, effects, resource contracts, and IR.

Framework-specific implementations MUST remain outside the core language grammar.

The architecture MUST accommodate symbolic computation, neural computation, inference, training, agents, datasets, tensor computation, probabilistic computation, and provenance without making each framework operation a permanent language keyword.

14.6 Distributed and networking

Distributed syntax MUST express computation and communication semantics independently of a fixed node count or physical network.

It MUST define consistency, communication, failure, placement intent, and resource semantics where applicable.

14.7 Sankofa and temporal computation

Sankofa constructs such as remembering, recalling, learning, inference, wisdom, history, temporal reasoning, and knowledge MUST be preserved as proposed or implemented language features according to their actual conformance status.

The grammar describes syntax.

It MUST NOT itself maintain memory, perform learning, store historical state, or implement consensus.

Multi-Timeline System constructs MUST define semantics for timelines, branching, observation, rewind, and merging without fixed timeline counts or arbitrary timestamp widths.

14.8 Nano-oriented computation

Nano-oriented syntax MAY describe atoms, molecules, materials, interactions, and capabilities.

It MUST NOT embed a fixed periodic-table implementation or a particular scientific database into the universal grammar.

It MUST be integrated only when its semantic ownership and downstream implementation are defined.

---

15. Hardware, Resource, and Capability Integration

Hardware discovery and physical realization MUST remain downstream from source parsing.

The language MAY express:

requires capability("gpu.compute")
requires capability("quantum.measurement")
requires capability("tensor.compute")
requires qubits >= n

The precise syntax and semantics MUST be specified before promotion to stable status.

15.1 Ownership boundaries

Concern| Owner
Source-level requirement syntax| Grammar and specification
Requirement meaning| Semantic analysis
Capability validation| Capability analysis
Resource feasibility| Resource analysis and runtime
Hardware discovery| HAL or target infrastructure
Physical mapping| Routing and realization
Execution ordering| Scheduler
Fault/noise semantics| ZQN
Quantum error correction| QEC
Runtime recovery| Resilience/runtime
Device calibration| Device/backend infrastructure
Vendor implementation| Backend/interoperability layer

The grammar MUST NOT absorb downstream implementation responsibilities.

---

16. Effects, Memory, and Concurrency

These are universal language concerns.

16.1 Effects

Effects MUST describe observable computation semantics.

The grammar MUST distinguish effect declarations, effect operations, handlers, effect composition, and effect polymorphism where applicable.

Effects MUST NOT be used as a substitute for runtime implementation details.

16.2 Memory

The memory model MUST distinguish:

- Ownership.
- Borrowing.
- References.
- Allocation.
- Regions.
- Address spaces.
- Persistence.
- Shared memory.
- Distributed memory.
- Accelerator memory.
- Quantum memory.
- Memory capabilities.

The grammar MUST NOT assume fixed RAM, VRAM, register, or storage capacities.

16.3 Concurrency

Concurrency syntax MUST express semantic intent.

It MUST NOT require a fixed number of cores, threads, tasks, actors, or devices.

The language MUST distinguish concurrency from parallel execution and define determinism and synchronization rules.

---

17. Compatibility and Versioning

Language evolution MUST be explicit and testable.

The compatibility subsystem MUST cover:

- Language versions.
- Syntax versions.
- Lexer compatibility.
- Parser compatibility.
- AST compatibility.
- Semantic compatibility.
- IR compatibility.
- Compiler compatibility.
- Runtime compatibility.
- Artifact compatibility.
- Dialect compatibility.
- Migration procedures.
- Deprecation procedures.

A breaking change MUST identify:

1. Affected feature.
2. Previous behavior.
3. New behavior.
4. Migration path.
5. Compatibility period.
6. Required tests.
7. Relevant compiler/runtime impact.

Existing filenames MUST NOT be renamed merely to make the directory structure look cleaner.

A rename requires an actual maintenance benefit and an explicit migration plan.

---

18. Extended and Historical Grammar

"Zamani-Grammar.md" MUST be retained.

It is the extended and historical design reference for material that is not necessarily implemented or normatively accepted.

Every feature MUST have a visible status.

Its lifecycle is:

Zamani-Grammar.md
       |
       v
Feature proposal
       |
       v
Semantic design
       |
       v
AST contract
       |
       v
Canonical grammar contract
       |
       v
Implementation
       |
       v
IR integration
       |
       v
Conformance tests
       |
       v
Stable promotion

No feature may enter stable Zamani syntax solely because it appears in "Zamani-Grammar.md".

The retained material MUST NOT be silently discarded.

The feature lifecycle MUST distinguish:

- Stable.
- Proposed.
- Experimental.
- Deprecated.
- Historical.
- Not implemented.

---

19. Implementation-Conformance Reference

"grammar.md" MUST describe the actual implementation, not the intended future language.

Its implementation status MUST distinguish:

- "SPECIFIED"
- "IMPLEMENTED"
- "PARTIALLY IMPLEMENTED"
- "PLANNED"
- "DEPRECATED"

Its claims MUST be traceable to actual source files and tests.

It MUST identify discrepancies among:

specification/
Zamani.g4
lexer/
src/lexer.rs
src/parser.rs
src/ast/
semantic analysis
IR
compiler
runtime

The file SHOULD be generated or mechanically validated wherever practical.

If manual content remains, it MUST be checked against implementation evidence.

A planned feature MUST NOT be presented as parser-supported.

A parser-supported feature MUST NOT automatically be presented as semantically complete.

---

20. Feature Contracts and Independent Completion

The repository MUST support independently completable features.

A developer MUST be able to complete one feature without discovering later that another file requires redesign because its interface was never established.

Each feature MUST have an explicit contract before implementation.

Where feature manifests are introduced, they SHOULD be stored under:

grammar/specification/features/

Each manifest MUST identify:

id:
name:
status:
version:

purpose:
owns:
does_not_own:

specification:
lexical_contract:
grammar_contract:

tokens:
ast_contract:
source_span_contract:

structural_validation:
semantic_contract:
type_contract:
effect_contract:

resource_contract:
capability_contract:

canonical_semantic_model:
ir_mapping:

compiler_consumers:
runtime_consumers:
backend_consumers:

cross_domain_dependencies:
upstream_contracts:
downstream_contracts:

diagnostics:
positive_tests:
negative_tests:
boundary_tests:
scalability_tests:
compatibility_tests:
determinism_tests:

hard_coding_policy:
security_requirements:
performance_requirements:

completion_criteria:

The manifest is a contract, not another grammar implementation.

It MUST reference the authoritative files rather than duplicate their definitions.

20.1 Every file completion contract

Every independently maintained file MUST document the following where applicable:

Contract| Required information
Identity| Exact repository path
Purpose| Why the file exists
Status| Actual maturity
Ownership| What the file defines
Non-ownership| What it explicitly does not define
Inputs| Required upstream information
Outputs| Public contract produced
Dependencies| Existing required files
Upstream integration| Contracts consumed
Downstream integration| Consumers and interfaces
Syntax| Relevant language rules
Tokens| Lexical requirements
AST| Node mapping
Semantics| Meaning and validation
IR| Canonical mapping
Compiler| Lowering and optimization integration
Runtime| Execution integration
Cross-domain| Shared interfaces
Diagnostics| Errors and source locations
Positive tests| Accepted cases
Negative tests| Rejected cases
Boundary tests| Edge conditions
Scalability tests| Resource-independent behavior
Compatibility| Version and migration rules
Determinism| Required deterministic behavior
Security| Validation and safety
Performance| Complexity expectations
Hard-coding audit| Universal-limit review
Completion criteria| Evidence required to mark complete

A file MAY reference a shared contract rather than reproduce it.

However, a reference MUST identify the exact authoritative contract and MUST NOT leave an unresolved future dependency.

20.2 Integration freeze

Before implementation begins, each file MUST declare its stable interface.

Once completed:

- Its public contract MUST NOT depend on undocumented assumptions.
- Downstream consumers MUST use the declared interface.
- New features MUST extend contracts through explicit versioned changes.
- Existing files MUST NOT be repeatedly rewritten to compensate for missing architecture.
- Changes to a completed contract MUST include impact analysis and regression tests.

This provides independent-first implementation without pretending that software architecture can prohibit all future compatible evolution.

---

21. Validation and Automated Enforcement

Production readiness MUST be verified across the repository.

The validation system MUST detect:

- Missing grammar imports.
- Unreachable grammar rules.
- Ambiguous grammar rules.
- Unintended left recursion.
- Duplicate tokens.
- Keyword collisions.
- Precedence inconsistencies.
- Lexer/specification disagreement.
- Parser/specification disagreement.
- Missing AST mappings.
- Missing semantic mappings.
- Missing IR mappings.
- Missing source spans.
- Unsupported feature claims.
- Unresolved downstream dependencies.
- Compatibility regressions.
- Hard-coded universal limits.
- Integer truncation.
- Resource assumptions.
- Determinism violations.
- Invalid generated artifacts.

21.1 Required validation layers

Specification validation
          |
          v
Lexical validation
          |
          v
ANTLR validation
          |
          v
Rust lexer tests
          |
          v
Rust parser tests
          |
          v
AST coverage
          |
          v
Semantic coverage
          |
          v
IR coverage
          |
          v
Compiler integration
          |
          v
Runtime/backend integration
          |
          v
Domain conformance
          |
          v
Scalability and portability
          |
          v
Compatibility and regression

A successful parser build alone is insufficient.

21.2 Rust safety and toolchain checks

CI MUST verify:

- Rust 1.97.1 compatibility.
- Rust 2021 edition.
- Safe Rust in production compiler implementation.
- Dependency compatibility.
- Formatting.
- Static analysis.
- Unit tests.
- Integration tests.
- Documentation tests where applicable.
- Feature-specific builds.
- Reproducible generated grammar artifacts where applicable.

The implementation MUST NOT claim that Rust's safety guarantees eliminate the need for bounds checks, overflow checks, allocation failure handling, or parser input validation.

---

22. Conformance Test Requirements

Every stable feature MUST have applicable positive, negative, boundary, scalability, compatibility, and diagnostic tests.

Tests MUST verify actual behavior rather than merely search for keywords or declarations.

22.1 General test categories

tests/
├── lexical/
├── syntax/
├── expressions/
├── types/
├── declarations/
├── statements/
├── functions/
├── modules/
├── effects/
├── memory/
├── concurrency/
├── classical/
├── quantum/
├── hybrid/
├── hdl/
├── hardware/
├── resources/
├── distributed/
├── ai/
├── data/
├── networking/
├── security/
├── interoperability/
├── dialects/
├── macros/
├── metaprogramming/
├── compatibility/
├── diagnostics/
├── negative/
├── boundary/
├── scalability/
├── determinism/
└── portability/

The existing test organization MUST be inspected before adding duplicate hierarchies.

22.2 Quantum tests

Quantum conformance MUST cover, where applicable:

- One qubit.
- Multiple qubits.
- Parameterized registers.
- Symbolic resource quantities.
- Dynamic allocation.
- Generic operations.
- Custom operations.
- Namespaced operations.
- Parameterized operations.
- Multi-target operations.
- Multi-control operations.
- Measurement.
- Mid-circuit measurement.
- Classical feed-forward.
- Dynamic control.
- Logical qubits.
- Physical mapping metadata.
- Unknown operations.
- Invalid targets.
- Invalid parameters.
- Source diagnostics.
- Canonical "quantum::ir" mapping.

Tests MUST NOT establish an artificial universal upper bound.

22.3 Scalability tests

Scalability tests MUST examine behavior as input and computation sizes grow.

They MUST distinguish:

- Language expressiveness.
- Parser complexity.
- Memory consumption.
- Compilation time.
- Representation limits.
- Runtime capacity.
- Target capacity.

The absence of a fixed language ceiling MUST NOT be interpreted as permission for unbounded allocation or denial-of-service vulnerabilities.

Implementations MUST define safe failure behavior for actual resource exhaustion.

22.4 Determinism

Where deterministic behavior is required, tests MUST establish that equivalent inputs produce the documented stable results.

This includes relevant:

- Parsing.
- AST construction.
- Diagnostics.
- Name resolution.
- Semantic analysis.
- IR generation.
- Build outputs.
- Serialization.
- Feature manifest processing.

---

23. Generated Files and Source of Truth

Generated artifacts MUST have an identifiable source of truth.

The repository MUST distinguish:

1. Hand-authored normative contracts.
2. Hand-authored implementation.
3. Generated grammar artifacts.
4. Generated documentation.
5. Test fixtures.
6. Historical artifacts.

Generated files MUST NOT be edited manually when regeneration is the intended workflow.

Generation MUST be reproducible under a documented toolchain.

A generated file MUST identify its source inputs and generation procedure where practical.

The build MUST fail clearly when required generated artifacts are absent or inconsistent.

---

24. Interoperability and Dialects

External formats and implementation technologies MUST NOT become the canonical Zamani semantic model.

Examples include:

- OpenQASM.
- QIR.
- LLVM.
- MLIR.
- HDL formats.
- WebAssembly.
- C.
- C++.
- Python.
- Rust.
- Vendor-specific hardware languages.

These belong at defined interoperability boundaries.

Each integration MUST specify:

- Input/output format.
- Version.
- Semantic mapping.
- Type mapping.
- Ownership mapping.
- Error behavior.
- Source provenance.
- Unsupported constructs.
- Compatibility.
- Tests.

Dialects MUST declare:

- Name.
- Version.
- Owner.
- Syntax extensions.
- Semantic extensions.
- AST mapping.
- IR mapping.
- Compatibility.
- Feature gates.
- Diagnostics.

A dialect MUST NOT silently create a separate language or bypass semantic validation.

---

25. Security and Robustness

The grammar and frontend are security-sensitive input-processing components.

They MUST handle malformed, adversarial, and excessively large inputs safely.

Required considerations include:

- Invalid UTF-8 handling according to the source contract.
- Unicode normalization policy.
- Identifier spoofing policy.
- Integer overflow.
- Numeric literal magnitude.
- Deep nesting.
- Recursive parser behavior.
- Large token streams.
- Excessive diagnostics.
- Allocation failure.
- Macro expansion growth.
- Compile-time execution limits.
- External format validation.
- Resource exhaustion.
- Malformed generated artifacts.

Resource safety limits MAY be imposed by implementations as explicit policies.

They MUST NOT silently become universal language restrictions.

Diagnostics MUST avoid exposing secrets or sensitive implementation state.

---

26. Contribution and Feature Promotion Process

Every new language feature MUST follow the same process.

Step 1 — Inspect

Inspect:

- Existing specification.
- Existing grammar.
- Existing lexer.
- Existing parser.
- Existing AST.
- Existing semantic implementation.
- Existing IR.
- Existing tests.
- Existing downstream consumers.

Do not create duplicate files before checking existing ownership.

Step 2 — Establish ownership

Define:

- Purpose.
- Non-ownership.
- Inputs.
- Outputs.
- Dependencies.
- Upstream contracts.
- Downstream consumers.

Step 3 — Design the complete contract

Specify:

- Syntax.
- Tokens.
- AST.
- Semantics.
- Source spans.
- IR mapping.
- Diagnostics.
- Compatibility.
- Scalability.
- Negative cases.
- Cross-domain integration.

Step 4 — Resolve dependencies

All required interfaces MUST be defined before implementation.

An unresolved dependency MUST be recorded as a blocker rather than concealed by a temporary incompatible interface.

Step 5 — Implement independently

Implement the feature against its frozen contract.

Do not expand unrelated grammar components.

Step 6 — Integrate

Integrate with the root grammar, lexer, parser, AST, semantic analysis, IR, compiler, and runtime as applicable.

Step 7 — Validate

Run the relevant tests and repository-wide conformance checks.

Step 8 — Promote

Only complete and verified features may be promoted to "STABLE".

---

27. Recommended Implementation Order

Implementation MUST begin with independent contracts before dependent grammar expansion.

The order below is a dependency order, not a claim that all listed files already exist.

Phase A — Architectural contracts

1. "DESIGN.md"
2. "README.md"
3. "specification/README.md"
4. "specification/language.md"
5. "specification/lexical.md"
6. "specification/syntax.md"
7. "specification/semantics.md"
8. "specification/portability.md"
9. "spec/compatibility.md"
10. "spec/type-system.md"

Completion condition: authority, ownership, interfaces, and feature lifecycle are resolved.

Phase B — Lexical contracts

1. "lexer/README.md"
2. "lexer/tokens.md"
3. "lexer/keywords.md"
4. "lexer/operators.md"
5. "lexer/identifiers.md"
6. "lexer/literals.md"
7. "lexer/comments.md"
8. "lexer/unicode.md"
9. "lexer/quantum-literals.md"
10. "lexer/conformance.md"

Completion condition: every token has one canonical lexical identity and implementation mapping.

Phase C — Universal syntax

1. "core/"
2. "expressions/"
3. "types/"
4. "declarations/"
5. "statements/"
6. "functions/"
7. "modules/"

Completion condition: universal syntax, precedence, AST, and diagnostics contracts are established.

Phase D — Universal semantics

1. "effects/"
2. "memory/"
3. "concurrency/"
4. "resources/"
5. "hardware/"

Completion condition: shared semantic contracts exist before domain-specific expansion.

Phase E — Computing domains

1. "classical/"
2. "quantum/"
3. "hybrid/"
4. "hdl/"
5. "distributed/"
6. "ai/"
7. "data/"
8. "networking/"
9. "security/"

Completion condition: domain syntax has defined AST, semantic, and IR integration.

Phase F — Advanced facilities

1. "compile/"
2. "execution/"
3. "interoperability/"
4. "dialects/"
5. "macros/"
6. "metaprogramming/"
7. Sankofa-related constructs.
8. Multi-Timeline System.
9. Nano-oriented constructs.

Completion condition: each facility has explicit safety, compatibility, and semantic ownership.

Phase G — Composition and implementation conformance

1. Verify modular grammar components.
2. Verify ANTLR imports and generation.
3. Integrate "Zamani.g4".
4. Align "src/lexer.rs".
5. Align "src/parser.rs".
6. Align "src/ast/".
7. Verify semantic mappings.
8. Verify canonical IR mappings.
9. Update "grammar.md" from implementation evidence.

Completion condition: no undocumented disagreement among the language specification and executable frontend.

Phase H — Production validation

1. Grammar validation.
2. Lexer conformance.
3. Parser conformance.
4. AST coverage.
5. Semantic coverage.
6. IR coverage.
7. Compiler integration.
8. Runtime/backend integration.
9. Domain tests.
10. Negative tests.
11. Boundary tests.
12. Scalability tests.
13. Determinism tests.
14. Compatibility tests.
15. Security and robustness tests.

Completion condition: the feature and repository satisfy their defined acceptance criteria.

---

28. Definition of Done

A file is complete only when its responsibilities are fulfilled and its integration contract is verified.

A feature is complete only when all applicable conditions are true:

- [ ] Purpose and ownership are documented.
- [ ] Non-ownership is explicit.
- [ ] Dependencies are resolved.
- [ ] Upstream contracts are identified.
- [ ] Downstream consumers are identified.
- [ ] Syntax is defined.
- [ ] Lexical requirements are defined.
- [ ] Ambiguity is resolved.
- [ ] AST mapping exists.
- [ ] Source-span behavior is defined.
- [ ] Structural validation exists.
- [ ] Semantic behavior is defined.
- [ ] Canonical IR mapping exists.
- [ ] Compiler integration exists.
- [ ] Runtime/backend integration exists where applicable.
- [ ] Cross-domain behavior is defined.
- [ ] Positive tests pass.
- [ ] Negative tests pass.
- [ ] Boundary tests pass.
- [ ] Scalability tests pass.
- [ ] Compatibility tests pass.
- [ ] Diagnostics are tested.
- [ ] Determinism is verified where required.
- [ ] Security requirements are satisfied.
- [ ] Hard-coding audit passes.
- [ ] Documentation status matches implementation evidence.
- [ ] Required Rust toolchain checks pass.
- [ ] No unsafe Rust has been introduced into production compiler implementation.

A checkbox MUST NOT be marked complete without evidence.

---

29. Explicit Non-Goals of the Grammar

The grammar MUST NOT implement or own:

- Physical hardware discovery.
- Device calibration.
- Physical qubit allocation.
- Hardware topology discovery.
- Quantum routing.
- Quantum scheduling.
- QEC algorithms.
- ZQN noise/fault semantics.
- HAL implementation.
- Runtime resource allocation.
- Backend machine-code generation.
- Vendor-specific execution mechanisms.
- Actual persistent memory storage.
- AI model training.
- Network transport.
- Cryptographic execution.
- Distributed consensus implementation.

The grammar may expose appropriate source-level syntax and contracts for these capabilities.

Their implementation belongs to the responsible downstream subsystem.

---

30. Final Architecture Invariants

The following rules are mandatory across the repository.

Invariant 1 — One language

Classical, quantum, HDL, hybrid, AI, distributed, and future computing domains belong to one coherent language.

Invariant 2 — One architectural authority

"DESIGN.md" defines grammar architecture. "specification/" defines normative language meaning. Implementation files define actual executable behavior.

Invariant 3 — No silent feature promotion

Historical and proposed syntax cannot become stable merely by appearing in a document.

Invariant 4 — No universal hardware ceilings

No artificial fixed capacity may restrict the language's representable computation.

Invariant 5 — Explicit resource semantics

Requirements, capabilities, constraints, preferences, hints, and implementation decisions remain distinct.

Invariant 6 — Domain-neutral AST foundations

Every computing domain reuses universal source representation where appropriate.

Invariant 7 — Canonical quantum IR

Quantum frontend constructs flow into "quantum::ir"; no competing frontend quantum IR is introduced.

Invariant 8 — Separation of concerns

Grammar describes syntax. Semantic analysis establishes meaning. IR represents validated computation. Backends realize computation.

Invariant 9 — Complete feature contracts

Syntax, AST, semantics, IR, implementation, diagnostics, tests, and integration are designed before implementation begins.

Invariant 10 — Safe Rust

Production compiler implementation uses safe Rust and supports the declared Rust 1.97.1 baseline.

Invariant 11 — Observable failure

Unsupported targets, unavailable resources, invalid syntax, and invalid semantics produce explicit, appropriately classified diagnostics.

Invariant 12 — Compatibility is deliberate

Language evolution is versioned, tested, and accompanied by migration rules.

Invariant 13 — Actual implementation evidence

Documentation MUST NOT claim implementation capabilities that are not supported by source code and tests.

Invariant 14 — Existing repository first

Existing files and directories MUST be expanded and integrated before introducing parallel structures.

Invariant 15 — Future extensibility

New domains and hardware targets must integrate through defined extension points rather than requiring universal grammar redesign.

---

31. The Final POCO-REAF Guarantee

The language architecture MUST allow developers to describe:

WHAT computation means
WHAT must be computed
WHAT correctness guarantees are required
WHAT capabilities are required
WHAT resources are required
WHAT constraints apply
WHAT alternatives are permitted

The compiler and execution infrastructure determine:

WHERE computation occurs
WHEN computation occurs
HOW computation is realized
WHICH compatible target is selected
WHICH physical resources are used
WHICH implementation strategy is applied

The resulting architecture is:

                    Zamani source
                          |
                          v
                   Specification
                          |
                          v
                    Zamani.g4
                          |
                          v
                  Rust lexer/parser
                          |
                          v
                    Frontend AST
                          |
                          v
                Structural validation
                          |
                          v
                Semantic analysis
                          |
             +------------+------------+
             |            |            |
             v            v            v
           Types        Effects     Resources
             |            |            |
             +------------+------------+
                          |
                          v
                 Canonical semantic model
                          |
                          v
                   Canonical IR
                          |
            +-------------+-------------+
            |             |             |
            v             v             v
        Classical     quantum::ir     HDL/Hardware
            |             |             |
            +-------------+-------------+
                          |
                          v
                  Optimization/lowering
                          |
            +-------------+-------------+
            |             |             |
            v             v             v
         Routing      Scheduling    Resilience
            |             |             |
            +-------------+-------------+
                          |
                          v
                          ZQN
                          |
                          v
                          HAL
                          |
                          v
                 Target realization
                          |
             +------------+------------+
             |            |            |
             v            v            v
            CPU          GPU          FPGA
                          |
                          v
                          QPU
                          |
                          v
                  Distributed systems
                          |
                          v
                   Future targets

The fundamental guarantee is not that every physical machine can execute every program.

It is that the language's meaning does not depend on the size, identity, or limitations of today's hardware.

The same source-level program should remain valid and semantically stable as compatible compilation and execution environments evolve.

That is the foundation of:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).

---

Document maintenance rule: This README owns repository navigation, authority, integration, contribution, and completion policy. Normative grammar details belong in "specification/"; architectural invariants belong in "DESIGN.md"; implementation evidence belongs in "grammar.md"; extended historical material remains in "Zamani-Grammar.md". Changes to these responsibilities require an explicit architecture review rather than silently duplicating their content.