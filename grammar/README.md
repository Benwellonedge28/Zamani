Zamani Grammar

Path: "grammar/README.md"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Primary branch: "main"
Edition: Rust 2021
Required compiler baseline: Rust 1.97.1
Safety requirement: production Rust code MUST use safe Rust; Rust "unsafe" is prohibited
Primary language objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: from the smallest useful computation to arbitrarily large computations, limited only by program semantics, representation limits, explicitly declared policies, and resources actually available to the compiler, runtime, and target.

---

1. Purpose

The "grammar/" directory defines the source-language contract of Zamani.

It is not merely an ANTLR grammar directory.

It is the traceable language boundary connecting:

Zamani source
    ↓
lexical contract
    ↓
grammar
    ↓
lexer
    ↓
parser
    ↓
frontend AST
    ↓
structural validation
    ↓
name/module resolution
    ↓
type/effect/ownership analysis
    ↓
resource/capability analysis
    ↓
semantic model
    ↓
canonical IR
    ↓
optimization
    ↓
routing / scheduling / resilience
    ↓
ZQN / HAL / target realization
    ↓
CPU / GPU / FPGA / QPU / distributed / embedded / future targets

The grammar therefore describes portable source-level meaning and structure.

It MUST NOT encode today's hardware as tomorrow's language.

---

2. Production Status Rule

"grammar/" is considered production-ready only when the complete contract exists.

A grammar production alone is not a completed language feature.

A feature is complete only when:

Specification
    ↓
Lexical contract
    ↓
Grammar
    ↓
Lexer
    ↓
Parser
    ↓
AST representation
    ↓
Structural validation
    ↓
Semantic validation
    ↓
Canonical semantic representation
    ↓
IR integration
    ↓
Compiler integration
    ↓
Runtime/backend integration where applicable
    ↓
Positive tests
    ↓
Negative tests
    ↓
Boundary tests
    ↓
Scalability tests
    ↓
Compatibility tests
    ↓
Diagnostics tests
    ↓
Determinism tests where applicable
    ↓
Hard-coding audit
    ↓
Production acceptance

No feature may be described as "STABLE" merely because the parser accepts it.

---

3. Core Objectives

The grammar must support one coherent language capable of expressing, without fragmenting into unrelated languages:

- general classical computation;
- systems programming;
- embedded programming;
- scientific computing;
- numerical computing;
- symbolic computing;
- high-performance computing;
- parallel computing;
- distributed computing;
- heterogeneous computing;
- quantum computing;
- hybrid quantum-classical computing;
- quantum error-corrected computation;
- hardware description;
- hardware/software co-design;
- accelerators;
- AI and machine learning;
- tensor and data computation;
- networking;
- cryptography;
- security;
- edge computing;
- cloud computing;
- nano-oriented computation;
- temporal/multi-timeline computation;
- metaprogramming;
- interoperability;
- domain-specific extensions;
- future computational paradigms.

These domains are parts of one language.

They must share the same:

- lexical system;
- identifier system;
- source locations;
- expressions;
- type system;
- declarations;
- statements;
- modules;
- effects;
- capabilities;
- resource model;
- diagnostics;
- versioning;
- compatibility model;
- semantic validation model.

---

4. The Fundamental Separation

Zamani source describes:

WHAT the program means
WHAT computation must occur
WHAT correctness guarantees are required
WHAT capabilities are required
WHAT resources are required
WHAT constraints apply
WHAT preferences exist
WHAT portability guarantees are expected

Later compiler/runtime infrastructure determines:

WHERE
WHEN
HOW
ON WHICH TARGET
USING WHICH PHYSICAL RESOURCE
USING WHICH NATIVE INSTRUCTION SET
USING WHICH DEVICE TOPOLOGY

Therefore:

Language meaning
        ≠
Target realization

and:

Portable intent
        ≠
Physical placement

This separation is the foundation of POCO-REAF.

---

5. POCO-REAF

5.1 Definition

POCO-REAF means:

«A Zamani program should be written in terms of stable program semantics rather than a particular machine realization, so that the same source-level program can be compiled and realized across different machines, environments, scales, and future targets without rewriting the algorithm merely because the available hardware changed.»

It does not mean that every program can execute on every target regardless of resource availability.

These are separate conditions:

Lexically valid
    ≠
Syntactically valid
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

A target that cannot satisfy a program's requirements must produce a precise diagnostic or invoke an explicitly permitted alternative realization.

It must not silently change program meaning.

---

6. Scalability: Tiny to Arbitrarily Large

Zamani MUST NOT impose artificial universal ceilings.

The language must conceptually support scaling from:

one value
one operation
one function
one qubit
one process
one device

to:

arbitrarily large programs
arbitrarily large datasets
arbitrarily large tensors
arbitrarily large quantum computations
arbitrarily large distributed systems
arbitrarily large heterogeneous systems

subject to:

- program semantics;
- representation limits;
- declared constraints;
- compiler resource budgets;
- runtime resource policies;
- target capabilities;
- actual available resources.

The following are prohibited as language-level universal limits:

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

Likewise, the grammar MUST NOT establish a finite physical universe such as:

Qubit0
Qubit1
...
Qubit1023

as the universal language model.

---

7. "Nothing Must Be Hard Coded"

This rule must be interpreted correctly.

The prohibition is against artificial implementation limits, not ordinary program constants.

This is valid:

let n = 1024;
allocate n elements;

because "1024" is program data.

This is not valid as a universal language architecture:

MAX_QUBITS = 1024

when that value means that Zamani cannot express a larger quantum computation.

Likewise:

Matrix<1024, 1024>

may be a perfectly valid program type.

But:

Zamani matrices may never exceed 1024 × 1024

is prohibited unless that restriction is an explicitly defined semantic restriction of a particular type or target profile rather than a universal language limitation.

The same distinction applies to:

- CPU count;
- core count;
- thread count;
- GPU count;
- FPGA count;
- QPU count;
- memory;
- storage;
- node count;
- network endpoints;
- tensor dimensions;
- register widths;
- vector widths;
- timelines;
- agents;
- processes;
- accelerators;
- physical qubits.

---

8. Requirement vs Capability vs Constraint vs Preference vs Hint vs Realization

The language MUST distinguish these concepts.

8.1 Requirement

A semantic resource or property is required.

Conceptually:

requires qubits >= n

8.2 Capability

A target must provide a capability.

Conceptually:

requires capability("quantum.mid_circuit_measurement")

8.3 Constraint

A property must remain within a specified bound.

Conceptually:

requires latency <= budget

8.4 Preference

A target characteristic is preferred but not mandatory.

Conceptually:

prefer accelerator("quantum")

8.5 Hint

Information is provided to optimization without changing program meaning.

Conceptually:

hint locality

8.6 Realization

A logical resource is eventually mapped to a physical resource.

Conceptually:

logical qubit
    ↓
physical qubit

Physical realization belongs downstream.

---

9. Repository Authority

The repository contains multiple grammar-related artifacts.

They MUST NOT become competing language authorities.

9.1 "grammar/DESIGN.md"

Authority: grammar architecture.

Owns:

- grammar architecture;
- authority model;
- integration rules;
- invariants;
- ownership boundaries;
- production-readiness criteria.

Does not own every individual grammar production.

---

9.2 "grammar/Zamani.g4"

Authority: canonical ANTLR syntax representation.

It MUST remain.

It represents the accepted Zamani syntax for ANTLR tooling and grammar conformance.

It MUST NOT independently invent features.

It MUST converge with:

grammar/specification/
grammar/spec/
src/lexer.rs
src/parser.rs
src/ast/

It is a syntax representation, not the owner of semantic behavior.

---

9.3 "grammar/grammar.md"

Authority: implementation-conformance reference.

It describes what the reference implementation currently accepts.

It MUST distinguish:

SPECIFIED
LEXER_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
BACKEND_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL
DEPRECATED

It MUST NOT silently become a second language specification.

---

9.4 "grammar/Zamani-Grammar.md"

Authority: extended language-design/reference material.

It may contain:

- historical designs;
- proposed features;
- future syntax;
- NIMBUS/Universal-Trinity concepts;
- Sankofa concepts;
- MTS concepts;
- nano concepts;
- AI concepts;
- advanced computational concepts;
- experimental features.

Presence in this file does not make syntax legal.

Promotion must follow:

Design
 ↓
Specification
 ↓
Grammar
 ↓
Lexer
 ↓
Parser
 ↓
AST
 ↓
Semantics
 ↓
IR
 ↓
Compiler/runtime
 ↓
Tests
 ↓
Stable

---

9.5 "grammar/README.md"

This file defines the operational contract of the entire grammar subsystem.

It must remain architectural rather than becoming another competing grammar.

---

10. One Canonical Zamani Language

There must be exactly one canonical Zamani language.

The repository may contain:

- ANTLR grammar;
- Rust lexer;
- Rust parser;
- AST;
- language specification;
- generated documentation;
- IDE syntax definitions;
- compatibility documents;
- domain contracts.

These are representations of one language.

They must never silently become separate languages.

---

11. Existing Repository Integration

The grammar must integrate with the existing compiler rather than create a second compiler architecture.

The current repository contains:

src/lexer.rs
src/parser.rs
src/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs
src/frontend/
src/compiler/
src/classical/
src/quantum/
src/hdl/
src/distributed/
src/ai/
...

The ownership contract is:

Component| Owns| Must not own
"grammar/"| language syntax contracts| runtime behavior
"Zamani.g4"| ANTLR syntax| target lowering
"grammar.md"| implementation grammar reference| future promises
"Zamani-Grammar.md"| design/proposal material| implementation claims
"src/lexer.rs"| tokenization| semantic validation
"src/parser.rs"| source syntax recognition| physical hardware mapping
"src/ast/"| source structure| target realization
"src/semantic.rs"| semantic validation| lexical recognition
"src/ir_gen.rs"| AST → IR lowering| hardware scheduling
"src/ir_verify.rs"| IR correctness| source parsing
"src/quantum/ir/"| canonical quantum semantics| source grammar
optimization| semantics-preserving transformation| language syntax
routing| physical realization| language definition
scheduling| timing/resource scheduling| language syntax
QEC| quantum error correction| parser/grammar
ZQN| quantum fault/noise semantics| duplicate quantum IR
HAL| target capability/state| language definition
runtime| execution| source grammar

---

12. Canonical Compiler Pipeline

The grammar architecture MUST support this pipeline:

                         Zamani Source
                              │
                              ▼
                     Source Map / File ID
                              │
                              ▼
                           Lexer
                              │
                              ▼
                        Token Stream
                              │
                              ▼
                           Parser
                              │
                              ▼
                       Frontend AST
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
       Name Resolution   Type Analysis   Effect Analysis
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                 Resource/Capability Analysis
                              │
                              ▼
                    Semantic Validation
                              │
                              ▼
                 Canonical Semantic Model
                              │
             ┌────────────────┼─────────────────┐
             ▼                ▼                 ▼
        Classical         Quantum             HDL/
        Semantics        Semantics         Hardware Intent
             │                │                 │
             └────────────────┼─────────────────┘
                              ▼
                       Canonical IR
                              │
                              ▼
                         Optimization
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
          Routing         Scheduling       Resilience
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                             ZQN
                              │
                              ▼
                             HAL
                              │
                              ▼
                       Target Lowering
                              │
          ┌───────────┬──────┼──────┬────────────┐
          ▼           ▼      ▼      ▼            ▼
         CPU         GPU    FPGA    QPU       Future Target

The grammar MUST remain above target-specific realization.

---

13. Rust and Safety Contract

The intended production baseline is:

Rust 2021
Rust 1.97.1

All production compiler implementation must use safe Rust.

Prohibited:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

The no-unsafe requirement is a repository production gate, not merely a grammar-directory convention.

The grammar README must therefore not claim the whole repository is production-ready until repository-wide unsafe usage has been removed or the relevant architecture has been replaced with safe equivalents.

The current repository contains existing unsafe runtime/stdlib paths; those are integration blockers that must be resolved separately.

---

14. Source-Language "unsafe"

The Rust implementation rule and the Zamani source-language concept must remain separate.

If the Zamani source language contains an "unsafe" construct, it requires its own:

- syntax contract;
- semantic contract;
- capability model;
- security model;
- compiler behavior;
- diagnostics;
- tests;
- compatibility policy.

Parsing the word "unsafe" does not itself provide safety guarantees.

If the final Zamani language is intended to be safe-by-default with no unrestricted unsafe execution model, the feature must eventually become either:

- explicitly capability-controlled;
- restricted;
- deprecated;
- or removed according to the compatibility policy.

No incomplete "unsafe" implementation may be presented as production safety.

---

15. Lexical Contract

"src/lexer.rs" is the executable lexical implementation.

The lexical specification belongs under:

grammar/spec/

and must be represented consistently by:

grammar/Zamani.g4
src/lexer.rs
tests/

The lexer must provide:

- deterministic tokenization;
- UTF-8-aware source handling;
- source spans;
- stable token identity;
- deterministic operator recognition;
- literal recognition;
- structured diagnostics;
- malformed-input handling;
- no target-dependent behavior;
- no hidden target state;
- no artificial machine-size limits.

---

16. Canonical Token Identity

The current lexer contains overlapping token categories, including concepts such as:

BitAnd / Ampersand
BitOr  / Pipe
Question / QuestionMark

These must be audited.

The rule is:

«One lexical spelling must have one canonical token identity unless there is a documented, mechanically justified lexical distinction.»

Context-dependent semantic meaning should normally be resolved by the parser/semantic layer rather than by inventing duplicate lexical tokens.

This applies especially to:

&
|
?
*
+
-
<
>

and other overloaded symbols.

Every token must have a documented:

spelling
token identity
lexical rule
precedence impact
parser consumers
AST mapping
diagnostic behavior
test coverage

---

17. Keyword Policy

A concept must not become a reserved keyword merely because it exists in a library or domain.

Prefer:

identifier
+
compositional syntax
+
semantic resolution

over:

one keyword for every operation

This is particularly important for:

- quantum gates;
- mathematical algorithms;
- AI algorithms;
- vendor accelerators;
- hardware devices;
- network protocols;
- cryptographic algorithms;
- future technologies.

The keyword registry must therefore be governed by:

lexer/keywords.md
spec/lexical.md
compatibility/

rather than by individual domain directories.

---

18. Literal Scalability

Literals must not inherit arbitrary host-machine limitations merely because the compiler is implemented in Rust.

The language must distinguish:

source literal
    ↓
lexical representation
    ↓
semantic numeric/value representation
    ↓
target representation

Where arbitrary precision or symbolic representation is semantically required, the grammar must permit it.

The grammar must not define artificial language limits such as:

maximum integer = machine usize
maximum tensor dimension = 32
maximum literal digits = implementation buffer size

unless such a bound is an explicitly documented lexical specification limit rather than an accidental implementation limitation.

---

19. Source Spans and Diagnostics

Every syntactic construct that can participate in diagnostics must retain sufficient source location information.

The contract is:

source file
+
file identity
+
byte/character range
+
grammar construct
=
diagnostic location

Errors must be structured enough for:

- compiler diagnostics;
- IDE/LSP tooling;
- tests;
- machine-readable output;
- future refactoring tools.

Diagnostics must identify:

- error category;
- source span;
- human-readable message;
- expected construct where known;
- actual construct where known;
- relevant feature/version;
- recovery status where applicable.

---

20. Parser Contract

The reference parser must remain:

- deterministic;
- span-aware;
- recoverable;
- safe Rust;
- target-independent;
- allocation-conscious;
- scalable to deep programs;
- free from artificial hardware limits.

The parser must make progress on malformed input.

Recovery must:

1. detect the unexpected construct;
2. emit a structured diagnostic;
3. consume enough input to make progress;
4. synchronize at a known boundary;
5. continue where safe;
6. never manufacture executable semantics from invalid input.

---

21. Deep-Program Scalability

Zamani must not assume programs are shallow.

Compiler infrastructure must avoid unnecessary recursive algorithms where deeply nested source could exhaust the host stack.

Where practical, use:

- explicit stacks;
- worklists;
- iterative traversal;
- streaming;
- incremental processing;
- lazy representations;
- configurable diagnostic budgets;
- configurable compiler resource budgets.

A program must not fail merely because an arbitrary compiler constant was chosen.

If a resource budget is necessary, it must be:

explicit
configurable
documented
diagnosed
not a language semantic limit

---

22. Grammar Modularity

The repository already contains many domain directories.

They must be populated only when each directory has a defined contract.

The intended structure is:

grammar/
├── README.md
├── DESIGN.md
├── Zamani.g4
├── Zamani-Grammar.md
├── grammar.md
│
├── specification/
├── spec/
├── lexer/
├── core/
├── types/
├── expressions/
├── statements/
├── declarations/
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
├── distributed/
├── ai/
├── data/
├── networking/
├── security/
├── resources/
├── compile/
├── execution/
├── interoperability/
├── dialects/
├── macros/
├── metaprogramming/
├── validation/
├── compatibility/
├── reference/
└── tests/

Existing directories must not be renamed merely to fit this architecture.

Empty directories should not be retained for appearance alone.

Create/populate a directory only when its ownership and integration contract are defined.

---

23. Root Grammar Composition

"grammar/Zamani.g4" is the root ANTLR composition point.

It should eventually own only composition-level rules such as:

program
item
declaration
statement
expression
typeExpression

Domain-specific syntax should be delegated through a controlled grammar composition architecture.

The root grammar must not become a permanent monolith containing every operation in every computational domain.

At the same time, modular files must not become independent competing grammars.

The model is:

                    Canonical Zamani Language
                              │
                        Zamani.g4
                              │
             ┌────────────────┼────────────────┐
             │                │                │
          Core rules      Shared rules     Domain rules
             │                │                │
             └────────────────┼────────────────┘
                              │
                         one language

---

24. "grammar/antlr/"

Do not automatically delete or rename "grammar/antlr/".

First establish whether anything consumes it.

The final rule is:

«There must be one canonical ANTLR composition root.»

If "grammar/antlr/" contains obsolete or duplicate root grammar material and has no valid consumers, it should eventually be removed rather than maintained as a second grammar authority.

Do not maintain:

grammar/Zamani.g4
grammar/antlr/Zamani.g4

as two competing canonical grammars.

---

25. Specification Architecture

"grammar/specification/" is the human-readable normative language specification.

At minimum, the target architecture is:

specification/
├── README.md
├── language.md
├── lexical.md
├── syntax.md
├── semantics.md
├── types.md
├── effects.md
├── resources.md
├── capabilities.md
├── portability.md
├── determinism.md
├── concurrency.md
├── classical.md
├── quantum.md
├── hybrid.md
├── hdl.md
├── hardware.md
├── distributed.md
├── ai.md
├── data.md
├── networking.md
├── security.md
├── interoperability.md
└── compatibility.md

These files must not duplicate the entire grammar.

They define the language contracts that grammar and compiler implementation must satisfy.

---

26. Formal Contract Architecture

"grammar/spec/" is for concise machine-checkable or implementation-oriented contracts.

At minimum:

spec/
├── lexical.md
├── syntax.md
├── type-system.md
├── semantics.md
├── effects.md
├── resources.md
├── capabilities.md
├── portability.md
├── determinism.md
├── diagnostics.md
├── source-spans.md
├── quantum.md
├── classical.md
├── hybrid.md
├── hdl.md
├── hardware.md
├── distributed.md
├── ai.md
├── data.md
├── networking.md
├── security.md
├── interoperability.md
├── versioning.md
├── compatibility.md
└── conformance.md

Every contract should identify:

Purpose
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Syntax Contract
AST Contract
Semantic Contract
IR Contract
Compiler Integration
Runtime Integration
Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility Tests
Hard-Coding Audit
Completion Criteria

---

27. The Independent-File Completion Contract

Every grammar feature file must be independently completable.

Each feature file MUST declare:

File
Purpose
Status
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Public Grammar Contract
Token Contract
AST Contract
Semantic Contract
IR Integration
Compiler Integration
Runtime Integration
Tooling Integration
Cross-Domain Integration
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility Tests
Diagnostics
Determinism
Security
Performance
Hard-Coding Audit
Completion Criteria

This is mandatory.

The goal is:

«Once a feature file is marked complete, adding an unrelated downstream feature must not require reopening it merely to discover its missing integration contract.»

If a downstream component requires information not declared by the feature contract, the feature was not actually complete.

---

28. AST Contract

Every grammar construct must have a predetermined mapping:

Grammar rule
    ↓
AST representation
    ↓
Semantic representation
    ↓
Canonical IR

No feature may be added using:

grammar today
AST later
semantics later
IR later

because that guarantees rework.

The AST must remain a source representation.

It must not become:

- a hardware IR;
- an optimizer IR;
- a QPU topology model;
- a QEC implementation model;
- a calibration model;
- a backend-specific instruction set.

---

29. Quantum Architecture

Quantum computing is a first-class Zamani domain.

The grammar must support:

- quantum types;
- logical qubits;
- abstract registers;
- parameterized operations;
- controls;
- adjoints/inverses;
- measurements;
- resets;
- dynamic control;
- classical feed-forward;
- observables;
- channels;
- noise intent;
- error-correction intent;
- logical operations;
- circuits;
- kernels;
- quantum resource requirements;
- target-independent quantum interoperability.

---

30. Quantum Gate Scalability

The grammar MUST NOT permanently enumerate today's hardware gate catalog.

Avoid a universal grammar such as:

gate
    : H
    | X
    | Y
    | Z
    | CNOT
    | SWAP
    | ...

A scalable model should represent:

operation name
operation namespace
parameters
controls
modifiers
targets
attributes
effects
capabilities

Conceptually:

apply operation(name, parameters)
    to targets
    with controls
    modifiers

The exact surface syntax belongs to the canonical Zamani syntax specification.

The architectural rule does not change:

«Quantum syntax represents quantum intent, not a frozen hardware gate catalogue.»

---

31. Canonical Quantum IR

The repository's "quantum::ir" remains the canonical quantum semantic boundary.

The intended path is:

Zamani quantum source
        ↓
frontend AST
        ↓
semantic quantum representation
        ↓
quantum::ir
        ↓
optimization
        ↓
decomposition
        ↓
routing
        ↓
scheduling
        ↓
QEC / resilience
        ↓
ZQN
        ↓
HAL
        ↓
target realization

The grammar must never create a competing permanent quantum IR.

It must not depend on:

- physical qubit IDs;
- coupling maps;
- native gate sets;
- pulse schedules;
- calibration data;
- QPU topology;
- backend-specific instruction encodings.

---

32. Quantum Resource Semantics

Quantum programs may express resource requirements.

Examples include:

required logical qubits
required computational depth
required fidelity
required measurement capability
required error-correction capability
required coherence properties
required communication capability

The grammar parses those requirements.

Semantic/compiler infrastructure determines feasibility.

QEC owns error correction.

ZQN owns fault/noise semantics.

HAL owns target capability/state.

Routing owns physical realization.

Scheduling owns timing/order/resource scheduling.

Optimization owns semantics-preserving optimization.

The grammar owns none of those implementations.

---

33. Classical Computing

"grammar/classical/" must represent classical computation without encoding specific processors.

The architecture may cover:

- scalar values;
- integers;
- floating point;
- vectors;
- matrices;
- tensors;
- symbolic computation;
- numerical computation;
- statistics;
- signal processing;
- linear algebra;
- optimization;
- scientific computing;
- control systems;
- parallel computation.

Mathematical algorithms should generally be represented through:

generic language constructs
+
types
+
intrinsics
+
libraries
+
semantic capabilities

rather than adding a new keyword for every algorithm.

---

34. Hybrid Computing

"grammar/hybrid/" provides the language boundary between domains.

It must support concepts such as:

classical computation
        ↓
quantum computation
        ↓
measurement
        ↓
classical decision
        ↓
quantum computation

and:

software
        ↓
accelerator
        ↓
memory
        ↓
communication
        ↓
hardware

The same program model must support mixed classical/quantum/hardware execution without creating separate languages.

---

35. HDL

"grammar/hdl/" must represent hardware intent.

It should eventually cover:

- modules;
- ports;
- signals;
- nets;
- registers;
- combinational logic;
- sequential logic;
- clocking;
- reset;
- timing intent;
- assertions;
- interfaces;
- protocols;
- state machines;
- pipelines;
- memories;
- parameterization;
- generation;
- synthesis intent;
- simulation intent;
- verification intent;
- physical intent;
- hardware/software co-design.

The grammar must not turn today's FPGA/ASIC limits into universal language limits.

---

36. Hardware Intent

"grammar/hardware/" describes target-independent hardware properties.

It may express:

- capabilities;
- resource requirements;
- compute requirements;
- memory requirements;
- communication requirements;
- acceleration requirements;
- quantum-device requirements;
- topology constraints;
- timing constraints;
- power constraints;
- thermal constraints;
- reliability requirements;
- calibration requirements;
- deployment constraints;
- negotiation policies.

It must not force source programs to select a particular:

CPU
GPU
FPGA
QPU
device ID
physical qubit
memory bank
network node

unless explicit target-specific source syntax is intentionally being used through a separately governed interoperability/target dialect.

---

37. Resource Model

"grammar/resources/" must distinguish:

requirement
capability
constraint
budget
preference
hint
negotiation
placement
scaling policy
portability requirement

For example:

requires 1000 logical qubits

is fundamentally different from:

map logical q0 -> physical q17

The first is portable program intent.

The second is a realization decision.

---

38. Distributed Computing

"grammar/distributed/" must support:

- processes;
- services;
- actors;
- messages;
- channels;
- communication;
- placement intent;
- replication;
- partitioning;
- consistency;
- transactions;
- collective operations;
- fault tolerance;
- deployment.

It must not encode a fixed number of nodes.

A distributed program should scale according to:

program semantics
+
resource availability
+
deployment policy

rather than:

N = 8 nodes forever

---

39. Concurrency and Parallelism

"grammar/concurrency/" must support:

- asynchronous execution;
- tasks;
- spawning;
- awaiting;
- actors;
- channels;
- synchronization;
- parallel regions;
- data parallelism;
- task parallelism;
- pipelines;
- reductions;
- deterministic parallelism;
- distributed concurrency.

The source should express:

parallel

when that is the semantic requirement.

It should not silently encode:

run on exactly 8 threads

as a universal implementation model.

---

40. AI and ML

"grammar/ai/" may represent:

- models;
- tensors;
- datasets;
- training;
- inference;
- optimization;
- differentiable computation;
- probabilistic computation;
- neural computation;
- symbolic reasoning;
- agents;
- pipelines;
- distributed training;
- model deployment.

The grammar must not make a particular framework such as a vendor library the language itself.

Frameworks belong in libraries, interoperability, capabilities, or target-specific lowering.

---

41. Data

"grammar/data/" may represent:

- collections;
- streams;
- records;
- tables;
- schemas;
- tensors;
- datasets;
- queries;
- transformations;
- pipelines;
- serialization;
- persistence;
- provenance.

Data syntax must remain independent of one particular database engine.

---

42. Networking

"grammar/networking/" may cover:

- endpoints;
- addresses;
- protocols;
- channels;
- requests;
- responses;
- streams;
- routing intent;
- service discovery;
- distributed computation;
- network capabilities.

Network addresses should remain abstract whenever the semantics do not require a physical address.

---

43. Security

"grammar/security/" may cover:

- identity;
- authorization;
- capabilities;
- policies;
- secrets;
- cryptographic intent;
- hashes;
- signatures;
- key management;
- secure computation;
- zero knowledge;
- provenance;
- trust.

The grammar should express security intent and structure.

It must not become a dictionary of every cryptographic implementation.

---

44. Compile and Execution Domains

"grammar/compile/" owns source-level compilation intent, including:

- target selection intent;
- compilation profiles;
- optimization policies;
- specialization intent;
- cross-compilation;
- reproducibility;
- deterministic builds;
- artifact intent;
- deployment intent;
- provenance.

"grammar/execution/" owns source-level execution intent, including:

- entry points;
- runtime environments;
- scheduling policy;
- placement policy;
- resilience;
- recovery;
- checkpointing;
- observability;
- tracing;
- profiling;
- lifecycle.

These must express policy and intent, not hard-coded target topology.

---

45. Interoperability

"grammar/interoperability/" owns controlled interaction with:

- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL formats;
- serialization formats;
- other foreign interfaces.

External representations are interoperability boundaries.

They are not automatically the canonical Zamani semantic model.

In particular:

OpenQASM
QIR
LLVM
MLIR
vendor IRs
HDL formats

must not silently become competing canonical Zamani IRs.

---

46. Dialects

"grammar/dialects/" must prevent dialect fragmentation.

Every dialect must declare:

name
version
owner
feature status
syntax additions
syntax restrictions
semantic additions
AST mapping
IR mapping
capabilities
compatibility
feature gates
migration policy

A dialect must not redefine the meaning of existing Zamani syntax without an explicit compatibility rule.

---

47. Macros

"grammar/macros/" may support:

- declarations;
- invocations;
- parameters;
- token expansion;
- syntax-tree expansion;
- hygiene;
- diagnostics.

Macros MUST NOT bypass:

- lexical validity;
- structural validation;
- type checking;
- capability checking;
- resource checking;
- security validation.

Expansion must have deterministic source mapping.

---

48. Metaprogramming

"grammar/metaprogramming/" may support:

- reflection;
- introspection;
- quotation;
- unquotation;
- code generation;
- compile-time computation;
- type-level computation;
- schemas.

Metaprogramming must not provide an unrestricted escape hatch around compiler correctness or security.

---

49. Sankofa, Temporal, MTS, Nano and Other Advanced Concepts

Existing advanced concepts must not be discarded merely because they are unusual.

However, they must follow the same promotion path.

For example:

Sankofa
MTS
nano
temporal
memory
learning
wisdom
recall
agents
omniversal concepts

must be treated as language features only when their:

syntax
AST
semantics
capabilities
resource behavior
IR behavior
runtime behavior
security
tests
compatibility

are defined.

The grammar parses their syntax.

It does not itself become the runtime, memory system, learning engine, scheduler, or device manager.

---

50. Feature Manifests

For complex or cross-domain features, the preferred future mechanism is a machine-readable feature contract.

Recommended location:

grammar/specification/features/

Only create this directory when the manifest schema itself has been specified.

Each feature manifest should contain at least:

id
name
status
version
domain
syntax
tokens
grammar_rules
ast_mapping
semantic_rules
capabilities
resource_requirements
ir_mapping
compiler_consumers
runtime_consumers
tooling_consumers
positive_tests
negative_tests
boundary_tests
scalability_tests
compatibility
diagnostics
hard_coding_policy
completion_criteria

This is the mechanism that makes a feature independently completable.

---

51. Feature Lifecycle

Every feature must have a lifecycle.

Recommended states:

IDEA
PROPOSED
DESIGNED
SPECIFIED
GRAMMAR_READY
LEXER_READY
PARSER_READY
AST_READY
SEMANTIC_READY
IR_READY
COMPILER_READY
RUNTIME_READY
TESTED
STABLE
DEPRECATED
REMOVED

A feature must never skip from:

IDEA

to:

STABLE

because syntax was added.

---

52. File-Level Definition of Done

A grammar file is complete only if:

Ownership

- its purpose is explicit;
- ownership is explicit;
- non-ownership is explicit.

Dependencies

- upstream contracts are known;
- downstream consumers are known;
- no hidden dependency exists.

Syntax

- productions are defined;
- ambiguity is addressed;
- precedence is addressed;
- associativity is addressed;
- lexical dependencies are documented.

AST

- every accepted construct has an AST mapping;
- every AST mapping is source-span aware.

Semantics

- semantic meaning is defined;
- invalid combinations are defined;
- resource semantics are defined;
- capability semantics are defined.

IR

- canonical IR mapping is predetermined;
- no duplicate domain IR is introduced.

Compiler/runtime

- compiler consumers are known;
- runtime consumers are known;
- target realization ownership is explicit.

Tests

- positive tests;
- negative tests;
- boundary tests;
- scalability tests;
- compatibility tests;
- diagnostics tests;
- determinism tests where applicable.

Hard-coding

- no artificial hardware limit;
- no fixed resource universe;
- no fixed target topology;
- no hidden compiler ceiling.

Completion

- no known downstream contract is missing.

---

53. Testing Architecture

"grammar/tests/" must test the language as a language, not merely test parser happy paths.

Target architecture:

tests/
├── README.md
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
├── diagnostics/
├── negative/
├── boundary/
├── scalability/
├── determinism/
├── portability/
└── compatibility/

Do not create empty directories merely to make this tree look complete.

Create each directory when its first independently complete test contract exists.

---

54. Quantum Test Requirements

Quantum grammar tests must cover at minimum:

single qubit
multiple qubits
symbolic register sizes
parameterized registers
dynamic resource expressions where supported
generic operations
custom operations
parameterized operations
controlled operations
adjoint operations
measurement
mid-circuit measurement
reset
classical feed-forward
logical qubits
error-correction intent
noise intent
resource requirements
capability requirements
target-independent programs

Tests must not encode an artificial maximum qubit count.

---

55. Scalability Tests

Scalability tests must verify that language infrastructure does not accidentally introduce fixed limits.

Examples:

deep nesting
large expression sequences
large declarations
large modules
large generic parameter lists
large arrays
large tensors
large symbolic dimensions
large quantum registers
large quantum operation lists
large distributed descriptions
large data pipelines
large dependency graphs

Where resource exhaustion is intentionally tested, the test must distinguish:

resource exhaustion

from:

language limitation

---

56. Hard-Coding Audit

The grammar validation system should detect suspicious universal limits.

Examples:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_DEVICES
MAX_TIMELINES
QUBIT_0
QUBIT_1
QUBIT_2

Detection alone is not enough.

Every flagged item must be classified as:

VALID PROGRAM CONSTANT
VALID TARGET PROFILE
VALID RESOURCE POLICY
VALID TEST FIXTURE
INVALID LANGUAGE LIMIT

A test fixture may use a fixed number.

The universal language architecture may not.

---

57. Determinism

Where language semantics require deterministic behavior, the grammar/compiler boundary must define it explicitly.

Determinism applies to:

- lexical tokenization;
- parsing;
- diagnostics ordering where promised;
- macro expansion where promised;
- semantic resolution where promised;
- reproducible compilation where promised;
- source-to-IR lowering where promised.

Nondeterministic runtime behavior must not be accidentally introduced by grammar interpretation.

---

58. Compatibility

Language compatibility must be explicit.

Every breaking syntax change must define:

old syntax
new syntax
version
diagnostic
migration
deprecation period
compatibility mode if applicable
AST impact
semantic impact
IR impact
tooling impact

Existing files:

grammar/compatibility/versions.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md

must remain authoritative for compatibility policy.

---

59. Compatibility Matrix

The grammar compatibility system must eventually be able to answer:

Feature| Specification| Lexer| Parser| AST| Semantics| IR| Compiler| Runtime| Tests
Feature X| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓| ✓
Feature Y| ✓| ✓| ✓| ✓| —| —| —| —| partial
Feature Z| proposed| —| —| —| —| —| —| —| —

No feature should be marked production/stable while a required stage is missing.

---

60. Generated Artifacts

Generated artifacts must be clearly identified.

The repository must distinguish:

AUTHORITATIVE SOURCE

from:

GENERATED OUTPUT

Generated parser/token/reference documentation must never be edited manually as if it were the language authority.

Every generated artifact must document:

generator
input authority
generation command
version
validation command

---

61. No Duplicate Semantic Boundaries

The grammar must not create parallel representations of concepts already owned elsewhere.

Examples:

grammar quantum IR
+
src/quantum/ir

is prohibited.

Likewise:

grammar hardware semantic model
+
HAL semantic model

should not duplicate ownership.

The grammar describes source intent.

Semantic infrastructure interprets it.

IR represents computation.

Backends realize computation.

---

62. Domain Integration Rule

Every domain must integrate through shared language infrastructure.

The required pattern is:

shared lexical system
        ↓
shared syntax foundations
        ↓
shared AST
        ↓
shared semantic analysis
        ↓
domain semantic model
        ↓
canonical domain IR
        ↓
shared optimization/lowering infrastructure

Not:

classical language
quantum language
HDL language
AI language
nano language

as independent languages.

---

63. Backend Independence

The grammar must not know whether a program eventually executes on:

x86
ARM
RISC-V
GPU
TPU
FPGA
ASIC
QPU
simulator
distributed cluster
edge device
cloud infrastructure
future architecture

A target may impose capabilities or resource constraints.

Those are discovered and validated downstream.

---

64. Resource Negotiation

POCO-REAF requires a distinction between:

program requirement

and:

target offer

Conceptually:

Program
  │
  ├── requirements
  ├── capabilities needed
  ├── constraints
  └── preferences
          │
          ▼
Target discovery
          │
          ▼
Capability matching
          │
          ▼
Resource planning
          │
          ▼
Lowering

The grammar should provide syntax for portable intent where required.

The actual negotiation mechanism belongs to compiler/runtime/resource infrastructure.

---

65. Target-Specific Syntax

Target-specific constructs are permitted only when explicitly identified as:

target-specific

or:

interoperability

or:

dialect

They must not contaminate the portable core language.

A vendor-specific feature must declare:

target
version
capability
fallback behavior
portability classification
compatibility

---

66. Error Handling

Grammar errors must be distinguishable from:

lexical errors
semantic errors
type errors
capability errors
resource errors
target compatibility errors
runtime errors

For example:

invalid syntax

must not be reported as:

target lacks 64 qubits

and:

target lacks required capability

must not be reported as:

invalid Zamani syntax

This separation is essential for POCO-REAF.

---

67. Security Boundary

The grammar must not bypass repository security policy.

Macros, metaprogramming, FFI, target directives, deployment constructs, and external resources must have explicit capability/security contracts.

The grammar itself must not:

- execute external commands;
- perform network I/O;
- access arbitrary files;
- access secrets;
- invoke hardware;
- mutate compiler global state.

The parser parses.

The semantic/compiler layers decide what is permitted.

---

68. Performance

Grammar design must support large source programs without introducing unnecessary quadratic behavior.

Validation must consider:

- tokenization complexity;
- parser complexity;
- precedence handling;
- nested structures;
- large lists;
- large modules;
- diagnostics;
- memory use;
- incremental parsing potential;
- generated parser performance where applicable.

Performance optimizations must never change language semantics.

---

69. Repository-Wide Integration Gate

"grammar/" cannot be declared production-ready independently of the repository.

The production gate must validate at least:

grammar
lexer
parser
AST
semantic analysis
IR generation
IR verification
compiler
quantum subsystem
classical subsystem
HDL subsystem
resource model
runtime
backend interfaces
tests
documentation
CI

The grammar README may define the contract, but implementation status must be determined from actual repository evidence.

---

70. Required Repository Consistency Checks

Before declaring the grammar production-ready, CI should verify:

cargo fmt --check
cargo check
cargo test
cargo clippy

plus grammar-specific validation.

The exact commands must be finalized against the repository's actual build tooling.

The Rust toolchain declaration must be valid and exact.

Do not use an invalid expression such as:

rust-version = "1.97" or "1.97.1"

A package manifest must declare one valid Rust version requirement.

---

71. No-Unsafe CI Gate

The final repository production pipeline must contain a no-unsafe gate.

The gate should reject production Rust source containing:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe {

with documented exceptions only for text fixtures or language-source examples where the word is not Rust implementation code.

The preferred implementation-level enforcement is:

#![forbid(unsafe_code)]

at appropriate crate/module boundaries, together with repository-wide CI auditing.

Existing unsafe runtime/stdlib implementations must be migrated before the repository can claim complete compliance.

---

72. Existing Filenames Must Be Preserved

The following existing files must not be unnecessarily renamed:

grammar/README.md
grammar/DESIGN.md
grammar/Zamani.g4
grammar/Zamani-Grammar.md
grammar/grammar.md

Their roles must instead be clarified and enforced.

Likewise, existing populated domain files should be integrated rather than renamed merely for aesthetic consistency.

Renaming is justified only when:

- the existing name creates an unavoidable semantic conflict;
- compatibility has been considered;
- all consumers have been migrated;
- the rename provides a real architectural benefit.

---

73. Do Not Create Empty Structure

Directories should be created when needed.

Do not create dozens of empty ".g4" or ".md" files merely to demonstrate an intended architecture.

A directory/file becomes justified when:

1. its ownership is defined;
2. its dependencies are known;
3. its downstream consumers are known;
4. its contract can be completed independently;
5. tests can be associated with it.

This keeps the grammar maintainable rather than turning the directory tree into architecture theatre.

---

74. Recommended Implementation Order

Implementation must begin with independent contracts.

Phase 1 — Authority

Complete and stabilize:

grammar/DESIGN.md
grammar/README.md
grammar/specification/README.md
grammar/specification/language.md
grammar/specification/lexical.md
grammar/specification/syntax.md
grammar/specification/semantics.md
grammar/specification/portability.md
grammar/spec/type-system.md
grammar/spec/compatibility.md

Phase 2 — Lexical contracts

Then:

grammar/lexer/tokens.md
grammar/lexer/keywords.md
grammar/lexer/operators.md
grammar/lexer/identifiers.md
grammar/lexer/literals.md
grammar/lexer/comments.md
grammar/lexer/unicode.md
grammar/lexer/quantum-literals.md

Phase 3 — Shared syntax

Then:

core
types
expressions
statements
declarations
functions
modules
effects
memory
concurrency

Phase 4 — Domains

Then:

classical
quantum
hybrid
hdl
hardware
resources
distributed
ai
data
networking
security

Phase 5 — Advanced mechanisms

Then:

compile
execution
interoperability
dialects
macros
metaprogramming

followed by governed integration of:

Sankofa
MTS
nano
temporal
other advanced paradigms

Phase 6 — Composition

Only after the independent contracts exist should:

grammar/Zamani.g4

be finalized as the composition root.

Phase 7 — Conformance

Then synchronize:

grammar/grammar.md
src/lexer.rs
src/parser.rs
src/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs

Phase 8 — Production validation

Finally run:

positive tests
negative tests
boundary tests
scalability tests
compatibility tests
determinism tests
diagnostic tests
hard-coding audit
no-unsafe audit
repository integration tests

---

75. What Must Never Happen

The following architecture is prohibited:

new feature
    ↓
add keyword
    ↓
add parser branch
    ↓
make up AST later
    ↓
make up semantics later
    ↓
make up IR later
    ↓
discover backend requirements
    ↓
rewrite grammar

Instead:

feature contract
    ↓
syntax
    ↓
lexer
    ↓
parser
    ↓
AST
    ↓
semantics
    ↓
IR
    ↓
compiler/runtime consumers
    ↓
tests

The second model is the required production workflow.

---

76. Definition of a Production Grammar

"grammar/" is production-ready only when all of the following are true:

- there is one canonical language;
- grammar authority is unambiguous;
- "Zamani.g4" has one defined role;
- "grammar.md" is implementation-conformance documentation;
- "Zamani-Grammar.md" cannot silently introduce syntax;
- lexical authority is defined;
- token identity is canonical;
- parser behavior is deterministic and recoverable;
- source spans are preserved;
- AST mappings are complete;
- semantic mappings are complete;
- IR mappings are complete;
- quantum syntax reaches "quantum::ir";
- no second quantum IR exists;
- classical, quantum and HDL domains share language foundations;
- hardware is described through portable intent;
- resources and capabilities are separate from physical realization;
- no artificial hardware limits are encoded;
- no universal fixed qubit/core/GPU/FPGA/node/tensor/timeline limits exist;
- target realization is downstream;
- domain-specific syntax does not become uncontrolled keyword proliferation;
- macros and metaprogramming cannot bypass semantic validation;
- dialects cannot silently fork the language;
- interoperability formats remain interoperability boundaries;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- compatibility tests exist;
- diagnostics are tested;
- determinism is tested where required;
- hard-coding audits pass;
- no-unsafe production requirements pass;
- Rust toolchain metadata is valid;
- repository-wide compiler integration passes;
- documentation agrees with implementation;
- CI verifies the complete contract.

---

77. Final Architecture

The intended Zamani architecture is:

                         ZAMANI SOURCE
                              │
                              ▼
                    ┌───────────────────┐
                    │ grammar/          │
                    │ language contract │
                    └─────────┬─────────┘
                              │
                              ▼
                           LEXER
                              │
                              ▼
                           PARSER
                              │
                              ▼
                      DOMAIN-NEUTRAL AST
                              │
              ┌───────────────┼────────────────┐
              │               │                │
              ▼               ▼                ▼
        Name Resolution   Type System     Effect System
              │               │                │
              └───────────────┼────────────────┘
                              ▼
                  Resource/Capability Model
                              │
                              ▼
                     Semantic Validation
                              │
                              ▼
                 Canonical Semantic Model
                              │
            ┌─────────────────┼──────────────────┐
            │                 │                  │
            ▼                 ▼                  ▼
       Classical          quantum::ir       HDL/Hardware
            │                 │                  │
            └─────────────────┼──────────────────┘
                              │
                              ▼
                         CANONICAL IR
                              │
                              ▼
                         OPTIMIZATION
                              │
              ┌───────────────┼────────────────┐
              │               │                │
              ▼               ▼                ▼
           ROUTING        SCHEDULING       RESILIENCE
              │               │                │
              └───────────────┼────────────────┘
                              ▼
                             ZQN
                              │
                              ▼
                             HAL
                              │
                              ▼
                      TARGET REALIZATION
                              │
        ┌─────────────┬───────┼────────┬─────────────┐
        ▼             ▼       ▼        ▼             ▼
       CPU           GPU     FPGA      QPU        FUTURE
        │             │       │        │          TARGETS
        └─────────────┴───────┴────────┴─────────────┘

The language boundary remains portable.

The implementation boundary remains typed and semantic.

The IR boundary remains canonical.

The hardware boundary remains target-specific.

The resource boundary remains dynamic.

The quantum semantic boundary remains "quantum::ir".

The compiler remains responsible for realization.

The runtime remains responsible for execution.

---

78. The Governing Rule

The entire grammar subsystem should be governed by one principle:

«Zamani source code expresses computation and portable intent. It does not encode the accidental limits of today's machines.»

Therefore:

Program Once
        ↓
Compile Once
        ↓
Discover capabilities
        ↓
Plan resources
        ↓
Optimize
        ↓
Route
        ↓
Schedule
        ↓
Apply resilience/QEC where required
        ↓
Lower through ZQN/HAL/backend infrastructure
        ↓
Run on the available target

The same source program should remain meaningful when moving:

tiny → large
small CPU → large CPU
CPU → GPU
GPU → FPGA
CPU/GPU → QPU
single machine → cluster
edge → cloud
current hardware → future hardware

provided the target can satisfy the program's semantic requirements or an explicitly supported alternative realization exists.

That is the grammar-level foundation of POCO-REAF.

---

79. Completion Criterion for This README

This README is complete when it serves as the stable architectural contract for "grammar/" and when every new grammar feature can answer, before implementation:

What syntax do I own?
What tokens do I consume?
What AST represents me?
What semantics do I introduce?
What resources/capabilities do I require?
What IR represents me?
Who consumes that IR?
What compiler components consume me?
What runtime components consume me?
What tests prove me?
What compatibility guarantees do I provide?
What scalability guarantees do I provide?
What hard-coding risks exist?
What do I explicitly NOT own?

If those questions cannot be answered, the feature is not ready to be added to the canonical grammar.

---

80. Non-Negotiable Final Rules

1. Do not create a second canonical Zamani grammar.
2. Do not rename "Zamani.g4", "grammar.md", or "Zamani-Grammar.md" unnecessarily.
3. Do not allow "Zamani-Grammar.md" to silently define implemented syntax.
4. Do not let "grammar.md" become a competing specification.
5. Do not encode physical hardware limits into grammar.
6. Do not encode fixed qubit/core/GPU/FPGA/node/tensor/timeline limits.
7. Do not enumerate every quantum gate as a permanent grammar alternative.
8. Do not create a second quantum IR.
9. Keep "quantum::ir" as the canonical quantum semantic boundary.
10. Do not put QEC implementation into the grammar.
11. Do not put routing implementation into the grammar.
12. Do not put scheduling implementation into the grammar.
13. Do not put calibration implementation into the grammar.
14. Do not put HAL implementation into the grammar.
15. Do not make vendor APIs the core language.
16. Do not make AI frameworks the core language.
17. Do not make every mathematical function a keyword.
18. Do not let dialects silently become separate languages.
19. Do not let macros bypass semantic validation.
20. Do not let metaprogramming bypass safety/capability rules.
21. Do not create empty directories/files merely for appearance.
22. Every completed feature must have its AST, semantic and IR contracts defined in advance.
23. Every completed feature must have downstream consumers defined in advance.
24. Every completed feature must have positive, negative, boundary and scalability tests.
25. Every completed feature must pass a hard-coding audit.
26. Production Rust must use no "unsafe".
27. The Rust toolchain declaration must be valid and exact.
28. The repository must pass grammar + lexer + parser + AST + semantic + IR + compiler + runtime integration.
29. A syntax feature is not a production feature until its complete pipeline exists.
30. The language must remain target-independent while the compiler remains target-aware.

This document is therefore the operational contract for turning the existing "grammar/" skeleton into a single, scalable, traceable, production-grade Zamani language boundary without repeatedly redesigning completed files when later subsystems are integrated.