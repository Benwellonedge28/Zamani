Worked for 17s

I inspected the repository’s current grammar/specification surfaces and the relevant compiler boundaries before drafting this. In particular, the current repository has multiple grammar authorities, a hand-written Rust parser/lexer path, src/ast/, src/semantic.rs, classical IR generation/verification, and a substantial src/quantum/ir/ hierarchy. The repository already documents quantum::ir as the canonical quantum semantic boundary, so this specification preserves that ownership rather than creating a competing quantum representation.

The following is the proposed complete contents of grammar/specification/grammar-authority.md.

Zamani Grammar Authority

Path: "grammar/specification/grammar-authority.md"
Status: Normative
Language: Zamani
Specification layer: Language architecture / grammar authority
Compiler baseline: Rust 1.97 / Rust 1.97.1
Safety requirement: Safe Rust only; Rust "unsafe" is prohibited
Primary portability model: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability model: From the smallest representable computation to arbitrarily large computation, constrained only by program semantics, implementation resources, target capabilities, and physical reality.

---

1. Purpose

This document establishes the single authority model for Zamani language syntax and defines how every grammar representation in the repository relates to:

- the Zamani language specification;
- lexical analysis;
- parsing;
- AST construction;
- semantic analysis;
- type checking;
- effect analysis;
- capability analysis;
- resource analysis;
- classical IR;
- "quantum::ir";
- quantum error correction;
- ZQN;
- optimization;
- scheduling;
- hardware abstraction;
- compilation;
- runtime;
- interoperability;
- tooling;
- compatibility;
- testing.

This document exists to eliminate ambiguity between the repository's existing grammar and specification surfaces.

The repository currently contains multiple language-description surfaces, including:

- "grammar/Zamani.g4";
- "grammar/Zamani-Grammar.md";
- "grammar/grammar.md";
- "grammar/specification/";
- "grammar/spec/";
- the Rust lexer;
- the Rust parser;
- the AST;
- semantic analysis;
- IR generation.

These artifacts do not have equal authority.

Their responsibilities are defined by this document.

---

2. Fundamental Authority Rule

Zamani is one language.

There must be one coherent language model rather than independent incompatible languages for:

- classical computing;
- quantum computing;
- HDL;
- hardware;
- AI;
- distributed computing;
- networking;
- data;
- accelerators;
- embedded systems;
- future computational domains.

The language architecture is:

                    Zamani Language
                          │
             ┌────────────┼────────────┐
             │            │            │
           Syntax       Semantics    Evolution
             │            │            │
          Lexer        AST/analysis  Compatibility
             │            │
          Parser         IR
             │            │
             └──────┬─────┘
                    │
          Domain Semantic Lowering
                    │
        ┌───────────┴───────────┐
        │                       │
   Classical IR            quantum::ir
        │                       │
        └───────────┬───────────┘
                    │
          Target-independent
             transformations
                    │
                    ▼
       Target-aware compilation
                    │
        ┌───────────┼───────────┐
        ▼           ▼           ▼
       CPU         GPU         QPU
        │           │           │
        └───────────┼───────────┘
                    ▼
             Runtime / Deploy

No domain is permitted to create a second incompatible language.

---

3. Authority Hierarchy

The following authority order is mandatory.

3.1 Level 1 — Normative Language Specification

The authoritative semantic and architectural specification is:

grammar/specification/

This directory establishes:

- language principles;
- syntax model;
- semantic model;
- type-system rules;
- effect rules;
- resource model;
- capability model;
- compilation model;
- execution model;
- portability;
- compatibility;
- language evolution;
- domain integration;
- conformance requirements.

A specification document may describe a feature that is not yet implemented.

However, every unimplemented feature MUST carry an explicit lifecycle state.

---

3.2 Level 2 — Canonical Concrete Syntax

The canonical executable grammar representation is:

grammar/Zamani.g4

It represents Zamani's concrete syntax for ANTLR-based tooling and grammar validation.

"Zamani.g4" MUST conform to the normative specification.

It MUST NOT independently invent language semantics.

It MUST NOT introduce target-specific machine restrictions.

ANTLR syntax is a representation of Zamani syntax, not a separate language.

---

3.3 Level 3 — Reference-Implementation Conformance

The file:

grammar/grammar.md

documents syntax currently accepted by the repository's reference lexer/parser implementation.

It therefore answers:

«What does the current implementation actually accept?»

It does not answer:

«What will Zamani eventually support?»

The distinction is mandatory.

The repository currently describes the implementation chain as including:

src/lexer.rs
src/parser.rs
src/ast/

and "grammar/grammar.md" must remain consistent with that implemented frontend.

---

3.4 Level 4 — Extended Design Material

The file:

grammar/Zamani-Grammar.md

may contain broad language-design material, historical material, proposed constructs, and universal-language concepts.

It is not independently authoritative.

Every feature appearing there MUST have a lifecycle state.

Allowed lifecycle states are:

PROPOSED
DESIGNED
SPECIFIED
LEXICALLY_SUPPORTED
PARSED
AST_SUPPORTED
SEMANTIC_SUPPORTED
IR_SUPPORTED
BACKEND_SUPPORTED
TESTED
STABLE
DEPRECATED
REMOVED

Presence in "Zamani-Grammar.md" alone MUST NEVER be interpreted as implementation.

---

3.5 Level 5 — Navigation Documentation

The file:

grammar/README.md

is the navigation and architecture entry point.

It MUST:

- explain the grammar architecture;
- link to the authoritative specification;
- link to the concrete grammar;
- link to implementation-conformance documentation;
- link to tests;
- explain feature lifecycle.

It MUST NOT become a competing grammar authority.

---

4. Existing Repository Reconciliation

The current repository contains more than one grammar/specification representation.

The following relationship is mandatory:

Artifact| Authority| Responsibility
"grammar/specification/"| Normative| Language architecture and contracts
"grammar/Zamani.g4"| Canonical syntax representation| ANTLR concrete syntax
"grammar/grammar.md"| Implementation conformance| Currently accepted syntax
"grammar/Zamani-Grammar.md"| Design/reference| Extended language concepts
"grammar/README.md"| Navigation| Explains the system
"grammar/spec/"| Specialized normative specifications| Topic-specific contracts
"src/lexer.rs"| Executable implementation| Tokenization
"src/parser.rs"| Executable implementation| Parsing
"src/ast/"| Executable structural model| Source representation
"src/semantic.rs"| Executable semantic analysis| Meaning validation
"src/ir_gen.rs"| Executable lowering| AST → IR
"src/ir_verify.rs"| Executable IR validation| IR invariants
"src/quantum/ir/"| Canonical quantum semantic boundary| Quantum representation

The existence of multiple files does not imply multiple languages.

---

5. No Circular Authority

The following dependency direction is mandatory:

Normative specification
        │
        ▼
Concrete syntax
        │
        ▼
Lexer
        │
        ▼
Parser
        │
        ▼
AST
        │
        ▼
Semantic analysis
        │
        ▼
Canonical IR
        │
        ├──────────────┐
        ▼              ▼
Classical IR       quantum::ir
        │              │
        └──────┬───────┘
               ▼
       Optimization
               │
               ▼
          Scheduling
               │
               ▼
       Hardware mapping
               │
               ▼
            Runtime

No downstream implementation may redefine an upstream concept solely because it requires additional information.

---

6. Grammar Does Not Own Semantics

The grammar owns:

- token sequences;
- syntactic structure;
- precedence;
- associativity;
- syntactic composition;
- source-level declarations;
- source-level expressions;
- source-level statements.

The grammar does not own:

- type validity;
- resource availability;
- hardware compatibility;
- physical-qubit mapping;
- scheduling;
- optimization;
- execution;
- quantum noise;
- error-correction implementation;
- target realization.

For example:

requires quantum;

may be syntactically valid.

Whether the program's quantum requirement is satisfiable is a semantic/resource question.

---

7. AST Boundary

The parser MUST produce the repository's canonical AST representation.

The grammar MUST NOT require downstream consumers to reconstruct syntax from token streams.

The architectural boundary is:

Source
  ↓
Lexer
  ↓
Parser
  ↓
AST

The AST is responsible for preserving:

- structural meaning;
- identifiers;
- declarations;
- expressions;
- statements;
- types;
- source spans;
- domain constructs;
- syntactic metadata required by diagnostics.

The AST must not contain backend-specific execution decisions merely because the parser encountered domain syntax.

---

8. Semantic Boundary

After parsing:

AST
 ↓
name resolution
 ↓
type analysis
 ↓
effect analysis
 ↓
capability analysis
 ↓
resource analysis
 ↓
domain validation
 ↓
IR lowering

The semantic layer determines whether syntactically valid programs are semantically valid.

This includes distinctions such as:

syntax validity
≠
type validity
≠
resource validity
≠
target compatibility
≠
runtime availability

---

9. Canonical Quantum Boundary

Quantum syntax is owned by Zamani's source language.

Quantum semantics are lowered into:

src/quantum/ir/

The repository's "quantum::ir" is the canonical quantum semantic boundary.

The grammar MUST NOT create a second quantum IR.

The quantum source pipeline is:

Zamani quantum syntax
        ↓
Zamani AST
        ↓
semantic validation
        ↓
quantum semantic lowering
        ↓
quantum::ir
        ↓
optimization
        ↓
QEC / ZQN / scheduling / mapping
        ↓
hardware backend

The existing quantum IR hierarchy includes dedicated areas for canonical quantum representation, control, pulse, validation, hashing, and other semantic components. Those remain downstream consumers rather than grammar authorities.

---

10. Quantum Grammar Ownership

The grammar MAY express:

- qubits;
- registers;
- logical qubits;
- physical references;
- states;
- operations;
- gates;
- parameterized operations;
- controlled operations;
- adjoints/inverses;
- circuits;
- measurement;
- reset;
- observables;
- dynamic control;
- mid-circuit measurement;
- classical conditions;
- quantum/classical interaction;
- logical computation;
- error-correction intent;
- resource requirements;
- capability requirements.

The grammar MUST NOT impose:

MAX_QUBITS = ...
MAX_GATES = ...
MAX_CIRCUIT_DEPTH = ...
MAX_DEVICES = ...

or equivalent hidden limits.

---

11. Qubit Identity

The grammar MUST distinguish abstract quantum resources from physical placement.

For example, source-level concepts may express:

logical qubit
physical qubit reference
qubit register
qubit collection

but source syntax MUST NOT assume that a program's universe is:

q[0]
q[1]
q[2]
...
q[63]

A concrete number of qubits is a semantic or resource property when required by a program.

Physical assignment belongs downstream.

---

12. QEC Boundary

Quantum error correction is a semantic/compilation concern.

The grammar may express intent, such as:

logical qubit
error correction requirement
fault tolerance requirement
code family requirement

The grammar MUST NOT duplicate the canonical QEC implementation model.

The pipeline is:

source intent
    ↓
AST
    ↓
semantic validation
    ↓
quantum::ir
    ↓
QEC analysis / transformation
    ↓
physical realization

A grammar construct must never become a second QEC IR.

---

13. ZQN Boundary

ZQN owns quantum noise-related execution semantics.

The grammar MAY express declarative intent such as:

noise-aware
noise requirement
noise constraint
noise model selection

but MUST NOT duplicate ZQN's canonical:

- channels;
- faults;
- calibration semantics;
- execution semantics;
- noise-aware transformation model.

The direction is:

Zamani source
      ↓
AST
      ↓
semantic model
      ↓
quantum::ir
      ↓
ZQN

ZQN MUST NOT depend on source grammar rules to define its internal semantic model.

---

14. Classical Boundary

Classical syntax belongs to Zamani's common language model.

Classical constructs include:

- scalar values;
- vectors;
- matrices;
- tensors;
- structured values;
- functions;
- generics;
- control flow;
- concurrency;
- parallelism;
- numerical computation;
- symbolic computation;
- accelerator operations.

Classical operations MUST remain composable with quantum and hardware domains.

The grammar MUST NOT make classical computation subordinate to quantum computation.

---

15. HDL Boundary

HDL syntax is part of Zamani's universal language surface.

It may describe:

- modules;
- ports;
- signals;
- wires;
- registers;
- clocks;
- timing;
- combinational logic;
- sequential logic;
- processes;
- state machines;
- memories;
- pipelines;
- hardware interfaces;
- parameters;
- generics.

However:

hardware semantics

must remain distinct from:

physical implementation

A hardware description may specify the required behavior without specifying the exact physical implementation.

---

16. Hardware Boundary

Hardware constructs MAY express:

- targets;
- capabilities;
- resources;
- topology requirements;
- placement constraints;
- accelerator requirements;
- timing constraints;
- energy preferences;
- reliability requirements.

They MUST NOT convert temporary hardware characteristics into permanent language limitations.

For example:

requires capability quantum;

is fundamentally different from:

use physical_device "vendor-specific-device";

The former expresses portable intent.

The latter is a target-specific realization.

---

17. Universal Resource Model

Zamani MUST distinguish the following concepts:

Requirement

Required for correctness.

requires quantum;

Capability

A property a target provides.

supports quantum;

Constraint

A condition that must hold.

constraint latency < T;

Preference

A desirable but negotiable property.

prefer low_energy;

Hint

Optimization information that does not affect correctness.

hint locality;

Resource

An actual execution resource.

Examples:

core
memory
qubit
accelerator
device
node
network

Target

A concrete execution environment.

These concepts MUST NOT be collapsed.

---

18. POCO-REAF Authority

Zamani's portability model is:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

POCO-REAF does not mean:

«every program can execute on every machine regardless of resources.»

It means:

«source semantics do not need to be rewritten merely because the execution environment changes.»

Therefore:

Program meaning
        ≠
Machine realization

The compiler may select different:

- algorithms;
- layouts;
- schedules;
- instruction sets;
- memory strategies;
- quantum mappings;
- hardware mappings;
- deployment strategies;

while preserving the program's declared semantics.

---

19. Compile-Once Boundary

"Compile Once" MUST be interpreted as preservation of a stable, target-independent semantic representation wherever technically possible.

A target-specific executable is not necessarily the canonical compiled representation.

The architecture should permit:

Source
 ↓
Canonical semantic representation
 ↓
Portable compiled artifact
 ↓
Target-specific realization

Target-specific information MUST NOT contaminate portable semantics unless explicitly declared as part of program intent.

---

20. Run-Everywhere Boundary

Execution environments may differ in:

- processor architecture;
- memory;
- accelerators;
- quantum hardware;
- FPGA resources;
- network topology;
- operating system;
- runtime;
- scheduling capabilities;
- precision;
- available libraries;
- device capabilities.

The source program should remain semantically stable.

Differences are resolved through:

capabilities
requirements
constraints
preferences
hints
resource discovery
target lowering

---

21. Run-Forever Requirement

Language evolution MUST preserve the possibility that future targets exist which are unknown today.

Therefore no permanent grammar rule may assume that the set of computational substrates is closed.

The grammar MUST be extensible to future:

- processors;
- accelerators;
- quantum architectures;
- optical systems;
- neuromorphic systems;
- biological computing;
- reversible systems;
- distributed systems;
- unknown future computational substrates.

Future targets should be represented through capability and dialect mechanisms rather than by repeatedly redesigning the core language.

---

22. Absolute Scalability Rule

Zamani has no arbitrary language-level finite maximum for:

- program size;
- number of modules;
- number of declarations;
- number of functions;
- number of types;
- number of expressions;
- number of statements;
- number of qubits;
- number of classical resources;
- number of hardware resources;
- number of nodes;
- number of devices;
- number of concurrent activities;
- tensor dimensions;
- data volume;
- execution stages.

"Infinity" means:

«the language imposes no arbitrary finite ceiling merely because a current implementation is convenient to implement that way.»

It does not assert infinite physical resources.

---

23. Hard-Coding Prohibition

The following are prohibited in the grammar architecture unless they are genuine semantic values supplied by the program:

MAX_QUBITS = 32
MAX_QUBITS = 64
MAX_CORES = 128
MAX_THREADS = 1024
MAX_DEVICES = 16
MAX_NODES = 1024
MAX_MATRIX_DIM = 4096
MAX_TENSOR_RANK = 32

Equivalent hidden restrictions are also prohibited.

This includes:

- fixed parser branches for finite resource universes;
- fixed arrays representing all devices;
- fixed topology assumptions;
- fixed accelerator counts;
- fixed register universes;
- fixed deployment counts.

Implementation limits are permitted only when they are implementation limits rather than language semantics.

They MUST be represented and diagnosed as such.

---

24. Resource-Parametric Syntax

Where a quantity varies with the program or target, grammar constructs should be parameterized.

Prefer:

register q[n];

where "n" is semantically determined.

Do not define:

register q[64];

as the language's universal quantum model.

Likewise:

parallel_for item in data

must not mean:

run on exactly 8 threads

unless the program explicitly requires eight resources.

---

25. Machine Independence

The grammar MUST NOT require source programs to encode:

- CPU model;
- GPU model;
- FPGA model;
- ASIC identifier;
- QPU identifier;
- memory address;
- physical device identifier;
- fixed node topology;
- fixed cluster size.

Such information belongs to:

target descriptions
deployment specifications
resource contexts
capability contexts
backend configuration
runtime discovery

unless the information is intentionally part of the program's semantics.

---

26. Semantic Requirements vs Implementation Choices

Every feature MUST distinguish:

Semantic requirement

What must be true for correctness.

Implementation choice

How a compiler or runtime chooses to satisfy it.

For example:

requires parallelism

does not imply:

spawn 16 threads

and:

requires quantum

does not imply:

use QPU-7

and:

requires accelerator

does not imply:

use GPU-0

---

27. Target-Specific Syntax

Target-specific syntax is permitted only when explicitly marked as target-specific.

It MUST NOT silently become portable Zamani syntax.

Target-specific extensions MUST be represented through:

- dialects;
- namespaces;
- explicit target declarations;
- attributes;
- capability constraints;
- interoperability boundaries.

A target-specific feature MUST identify:

owner
version
capability
compatibility
lowering path
fallback behavior

---

28. Dialect Authority

Dialect extensions may introduce domain-specific syntax.

They MUST NOT redefine:

- identifiers;
- source locations;
- basic expressions;
- core type semantics;
- module semantics;
- effect semantics;
- resource semantics;
- compatibility rules;

without an explicit language-version change.

Dialect namespaces must prevent collisions.

A dialect is an extension of Zamani, not a replacement language.

---

29. Keyword Policy

Zamani MUST avoid uncontrolled keyword growth.

A new keyword should be introduced only when:

1. ordinary identifiers cannot express the required distinction;
2. compositional syntax is insufficient;
3. the construct is fundamental to language semantics;
4. compatibility impact has been evaluated;
5. lexer/parser/AST/semantic/IR ownership is defined;
6. tests are defined before implementation.

Prefer compositional constructs such as:

operation(...)
resource(...)
requirement(...)
capability(...)
constraint(...)
attribute(...)

over adding permanent keywords for every hardware or algorithmic concept.

---

30. Grammar Modularity

The grammar MUST be modular enough that domains can evolve without modifying unrelated language foundations.

Conceptual dependency direction:

lexer
  ↓
core
  ↓
types
  ↓
expressions
  ↓
statements
  ↓
declarations
  ↓
functions
  ↓
modules
  ↓
effects/capabilities
  ↓
memory/concurrency
  ↓
classical
  ↓
quantum
  ↓
hybrid
  ↓
HDL
  ↓
hardware
  ↓
distributed
  ↓
AI/data/network/security
  ↓
resources
  ↓
compile/execution
  ↓
interoperability
  ↓
dialects/metaprogramming

Actual dependencies MUST be validated against the repository before implementation.

---

31. Existing Rust Frontend Integration

The current repository uses a Rust lexer/parser architecture.

The parser is a hand-written recursive-descent/Pratt implementation and currently covers constructs including:

- "let";
- "const";
- "fn";
- structs;
- enums;
- traits;
- implementations;
- classes;
- interfaces;
- modules;
- imports;
- loops;
- match;
- closures;
- async/await;
- quantum;
- effects;
- macros;
- type aliases;
- and other Zamani-specific constructs.

Therefore "Zamani.g4" MUST NOT be treated as though it were already the sole executable parser.

The production architecture must establish one canonical language contract and then require both parser representations to conform to it.

---

32. ANTLR Integration

"grammar/Zamani.g4" MUST:

- parse all stable concrete syntax;
- have deterministic entry points;
- avoid semantic actions;
- avoid embedded target-specific execution logic;
- avoid machine-specific limits;
- preserve source locations;
- expose syntax suitable for parser validation;
- remain testable independently.

ANTLR grammar actions MUST NOT become the source of semantic truth.

ANTLR-specific implementation behavior belongs to tooling.

---

33. Hand-Written Parser Integration

"src/parser.rs" MUST implement the same language contract as "Zamani.g4" for stable syntax.

A syntax feature is not complete merely because:

Zamani.g4

accepts it.

It is also not complete merely because:

src/parser.rs

accepts it.

Stable syntax requires conformance between:

specification
Zamani.g4
lexer
parser
AST
semantic analysis
tests

---

34. Lexer Authority

The lexer owns:

- character classification;
- token boundaries;
- keyword recognition;
- identifier recognition;
- literals;
- operators;
- punctuation;
- comments;
- source locations.

The lexer MUST NOT decide semantic validity.

For example, the lexer can recognize:

quantum
qubit
requires
capability

but cannot determine whether a particular quantum operation is semantically legal.

---

35. Parser Authority

The parser owns:

- grammatical structure;
- precedence;
- associativity;
- declaration structure;
- statement structure;
- expression structure;
- type syntax;
- domain syntax.

The parser MUST NOT decide:

- whether a target has sufficient qubits;
- whether a gate is physically calibrated;
- whether an FPGA has sufficient LUTs;
- whether a GPU exists;
- whether a network is available;
- whether a resource requirement can be satisfied.

---

36. Semantic Analyzer Integration

"src/semantic.rs" owns semantic validation.

The current implementation already performs symbol management, type inference/checking, scope handling, and semantic diagnostics.

Future grammar features MUST therefore define their semantic contract before the syntax is considered complete.

Every new feature requires:

syntax
→ AST representation
→ semantic rule
→ diagnostic behavior
→ IR representation

---

37. Type-System Boundary

Grammar defines the syntax of types.

The semantic/type subsystem defines whether those types are valid and how they behave.

The grammar must support extensible type construction without hard-coding every future data domain.

The architecture must support, where semantically justified:

- primitive types;
- composite types;
- generic types;
- function types;
- tuples;
- arrays;
- maps;
- options;
- results;
- algebraic types;
- references;
- resources;
- quantum types;
- hardware types;
- classical types;
- future domain types.

---

38. Effect Boundary

Effects are semantic declarations of observable or required computational behavior.

Examples include:

- I/O;
- hardware access;
- quantum operations;
- networking;
- distributed execution;
- security-sensitive behavior.

The grammar may express effects.

Semantic analysis determines whether effect declarations are valid.

Runtime and backends implement them.

The grammar MUST NOT encode runtime implementation details.

---

39. Capability Boundary

Capabilities represent what an execution environment can provide.

Capability syntax MUST remain declarative.

For example:

requires capability quantum;

does not imply a particular vendor or physical device.

Capabilities must be discoverable or supplied through compilation/runtime contexts.

---

40. Constraint Boundary

Constraints express conditions that must hold.

Examples:

latency < limit
energy <= budget
fidelity >= threshold

The grammar represents the expression.

Semantic/resource analysis determines meaning.

Schedulers and backends attempt to satisfy constraints.

Failure to satisfy a target constraint MUST be a target/resource diagnostic rather than a grammar error.

---

41. Preference Boundary

Preferences are negotiable.

Examples:

prefer low_energy;
prefer locality;
prefer throughput;
prefer latency;

Preferences MUST NOT be interpreted as correctness requirements unless explicitly promoted to a requirement or constraint.

---

42. Hint Boundary

Hints are optimization information.

A compiler may ignore a hint while preserving correctness.

A hint MUST NOT change the semantic meaning of a program unless the language explicitly defines it as semantic.

---

43. Classical/Quantum Integration

Hybrid programs must use shared language constructs.

The intended model is:

classical computation
        │
        ├──── classical values
        │
        ▼
 quantum operation
        │
        ▼
 measurement
        │
        ▼
 classical result
        │
        ▼
 classical control

The grammar MUST allow this composition without creating a separate hybrid language.

---

44. Quantum/HDL Integration

Quantum hardware descriptions may coexist with quantum source programs.

The boundary is:

quantum semantics
        ↓
quantum::ir
        ↓
hardware realization

HDL describes hardware behavior and interfaces.

Quantum IR describes quantum computation.

Neither becomes the owner of the other's semantics.

---

45. Classical/HDL Integration

Classical control may configure or interact with hardware.

Hardware modules may expose typed interfaces.

The language must distinguish:

software computation

from:

hardware realization

while allowing explicit interoperability.

---

46. Distributed Integration

Distributed syntax may express:

- nodes;
- services;
- communication;
- messaging;
- replication;
- consistency;
- placement;
- fault tolerance.

It MUST NOT require a fixed number of nodes.

For example:

nodes = available;

is conceptually different from:

nodes = 8;

The latter is valid only when eight is an explicit program requirement.

---

47. AI and Data Integration

AI/data syntax must use common Zamani foundations for:

- types;
- functions;
- tensors;
- resources;
- capabilities;
- execution;
- effects.

AI syntax MUST NOT create a separate resource universe.

A tensor's shape may be semantically fixed where required, but the grammar must not impose an arbitrary maximum rank or size merely because a current backend does.

---

48. Interoperability

Foreign interfaces belong downstream of the language's core semantics.

Supported or planned interoperability may include:

- C;
- C++;
- Python;
- OpenQASM;
- Verilog;
- system interfaces;
- ABI/FFI boundaries.

An interoperability grammar describes external syntax or declarations.

It MUST NOT redefine Zamani's internal semantic model.

---

49. Version Authority

Language versioning is centralized.

No individual grammar file may invent incompatible version semantics.

The following MUST remain coordinated:

grammar/specification/language-version.md
grammar/specification/compatibility.md
grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
lexer
parser
AST
semantic analyzer
tests

A syntax-breaking change requires an explicit compatibility decision.

---

50. Feature Lifecycle

Every language feature MUST have a lifecycle:

PROPOSED
   ↓
DESIGNED
   ↓
SPECIFIED
   ↓
LEXICALLY_SUPPORTED
   ↓
PARSED
   ↓
AST_SUPPORTED
   ↓
SEMANTIC_SUPPORTED
   ↓
IR_SUPPORTED
   ↓
BACKEND_SUPPORTED
   ↓
TESTED
   ↓
STABLE

Features may additionally become:

DEPRECATED
   ↓
REMOVED

A feature MUST NOT skip lifecycle states merely because syntax exists.

---

51. Independent-File Completion Contract

Every grammar/specification file MUST be independently completable.

Before implementation begins, its contract MUST define:

Purpose
Scope
Normative status
Owns
Does not own
Inputs
Outputs
Dependencies
Upstream contracts
Downstream consumers
Syntax contract
AST contract
Semantic contract
IR contract
Compiler integration
Runtime integration
Tooling integration
Cross-domain integration
Compatibility requirements
Scalability requirements
Hard-coding requirements
Tests
Negative tests
Boundary tests
Completion criteria

A file MUST NOT rely on an undefined future architectural decision.

---

52. No-Re-Editing Principle

The dependency-first rule is mandatory.

Before a file is implemented, all contracts it depends on MUST already be established.

For example:

grammar-authority.md
        ↓
language-principles.md
        ↓
syntax-model.md
        ↓
lexer contract
        ↓
parser contract
        ↓
AST contract
        ↓
semantic contract
        ↓
IR contract

A completed file should not need fundamental redesign merely because another later file is implemented.

If a genuine language change becomes necessary, it must proceed through the versioning process.

---

53. File Ownership Contract

The following ownership is normative:

Component| Owns| Does not own
"grammar/specification/"| Language contracts| Runtime implementation
"grammar/Zamani.g4"| Concrete ANTLR syntax| Semantic validity
"grammar/grammar.md"| Current parser conformance| Future promises
"grammar/Zamani-Grammar.md"| Extended design material| Implementation claims
"src/lexer.rs"| Tokenization| Semantic interpretation
"src/parser.rs"| Syntax recognition| Hardware realization
"src/ast/"| Source structure| Backend execution
"src/semantic.rs"| Semantic validity| Lexing
"src/ir_gen.rs"| AST → IR lowering| Target scheduling
"src/ir_verify.rs"| IR invariants| Source parsing
"src/quantum/ir/"| Canonical quantum semantics| Source grammar
QEC| Error-correction semantics/transformation| Source grammar
ZQN| Quantum noise/execution concerns| Duplicate quantum IR
Optimization| Semantics-preserving transformation| Language definition
Scheduling| Timing/resource scheduling| Language definition
Hardware| Target realization/capabilities| Language definition
Runtime| Execution| Source syntax
Backends| Target implementation| Portable semantics

---

54. Integration Contract for Every New Grammar Feature

Before a feature is accepted, the following questions MUST have documented answers:

1. What syntax introduces it?
2. What tokens are required?
3. Which grammar rule owns it?
4. Which AST node represents it?
5. What source-span information is retained?
6. What semantic rules apply?
7. What types are involved?
8. What effects are involved?
9. What capabilities are involved?
10. What resource requirements exist?
11. What constraints exist?
12. What IR represents the meaning?
13. Which optimizer consumes it?
14. Which scheduler consumes it?
15. Which hardware/backend layer consumes it?
16. Which runtime component consumes it?
17. Which interoperability boundary consumes it?
18. Which tests prove it?
19. What compatibility rules apply?
20. What scalability limits are forbidden?
21. What happens if the target cannot satisfy it?

If any answer is undefined, the feature is not production-ready.

---

55. Hard-Coding Audit

Every grammar change MUST be audited for accidental fixed limits.

Audit for:

MAX_*
LIMIT_*
COUNT_*
SIZE_*
CAPACITY_*
DEVICE_*
QUBIT_*
CORE_*
THREAD_*
NODE_*
GPU_*
FPGA_*
REGISTER_*
MEMORY_*

Also audit for equivalent semantic restrictions that do not use those names.

Every discovered limit must be classified as:

1. Language semantic requirement
2. Explicit program requirement
3. Target-specific requirement
4. Resource constraint
5. Implementation limitation
6. Test-only limitation
7. Documentation-only limitation
8. Accidental hard-coding

Only accidental hard-coding must be removed immediately.

Implementation limits must be represented as implementation/resource limitations rather than silently becoming language limits.

---

56. Determinism

For identical source input, grammar processing MUST produce deterministic:

- tokenization;
- parsing;
- AST structure;
- diagnostics ordering where ordering is specified.

Target-dependent behavior must not alter syntactic interpretation.

---

57. Diagnostics

Grammar-related diagnostics MUST retain sufficient source location information.

At minimum, diagnostics should be capable of identifying:

- source file;
- span;
- offending token;
- expected syntax;
- actual syntax;
- relevant grammar context.

Semantic errors must not be disguised as parser errors.

For example:

missing '}'

is a syntax error.

Whereas:

target lacks required quantum capability

is a capability/resource error.

---

58. Error Recovery

Parser error recovery MUST NOT silently manufacture valid semantics.

Recovery may be used for:

- IDE parsing;
- diagnostics;
- interactive tooling.

Production compilation must distinguish recovered syntax from successfully parsed source.

A recovered AST MUST NOT automatically be considered semantically valid.

---

59. Testing Authority

Every stable grammar feature MUST have tests at multiple levels.

Required categories:

positive
negative
boundary
cross-domain
scalability
determinism
compatibility
round-trip
implementation-conformance

Tests belong in the appropriate grammar/compiler test locations.

---

60. Positive Tests

Positive tests prove valid syntax.

Every feature must have:

- minimal valid example;
- representative example;
- compositional example;
- realistic example.

---

61. Negative Tests

Negative tests prove that invalid syntax is rejected.

Examples include:

- malformed declarations;
- malformed types;
- malformed quantum operations;
- invalid hardware declarations;
- invalid module structures;
- invalid effect syntax;
- malformed constraints;
- malformed capabilities.

Semantic invalidity should be tested separately from syntax invalidity.

---

62. Boundary Tests

Boundary tests MUST verify that grammar architecture does not impose arbitrary finite limits.

Examples include:

- large declaration sets;
- deeply nested valid expressions;
- large module graphs;
- large collections;
- large quantum resource declarations;
- large hardware descriptions;
- large distributed descriptions;
- large tensor specifications.

Tests must distinguish language limits from host-resource exhaustion.

---

63. Cross-Domain Tests

The grammar MUST test combinations including:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

The objective is to prove that domains compose rather than merely coexist.

---

64. Round-Trip Tests

Where a canonical printer/serializer exists:

Source
 ↓
Lexer
 ↓
Parser
 ↓
AST
 ↓
Printer
 ↓
Parser

must preserve semantic structure.

Formatting differences are acceptable.

Semantic differences are not.

---

65. Conformance Testing

The repository must maintain a conformance suite that compares:

normative specification
        ↓
Zamani.g4
        ↓
reference lexer/parser
        ↓
AST

A stable construct accepted by one representation but rejected by another constitutes a conformance defect unless the lifecycle explicitly says the feature is implementation-incomplete.

---

66. Rust Requirements

The grammar infrastructure and supporting implementation MUST target:

Rust 1.97 / Rust 1.97.1

The repository's current "Cargo.toml" declares Rust 1.97 / 1.97.1 as its baseline.

Production implementation MUST use:

safe Rust only

Rust "unsafe" is prohibited.

This includes avoiding hidden unsafe dependencies in newly introduced grammar infrastructure where practical and ensuring the Zamani implementation itself does not introduce "unsafe" blocks.

---

67. Dependency Policy

Grammar infrastructure should depend on the smallest necessary set of components.

The current repository already uses "antlr-rust", "serde", "serde_json", "thiserror", "anyhow", logging infrastructure, and related dependencies.

A grammar feature MUST NOT introduce a dependency merely to express syntax that can be represented by existing infrastructure.

---

68. Repository Integration

The grammar authority must integrate with:

grammar/
src/lexer.rs
src/parser.rs
src/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs
src/quantum/
tests/
examples/
documentation
tooling

No grammar feature is considered complete if its downstream integration is undefined.

---

69. Non-Dependencies

The grammar MUST NOT directly depend on:

- runtime state;
- physical devices;
- scheduler state;
- backend implementation;
- calibration data;
- network state;
- current machine topology;
- QPU availability;
- GPU availability.

Those belong downstream.

---

70. Quantum Scheduling Boundary

Quantum scheduling consumes semantic/IR representations.

The grammar may express timing or scheduling intent where timing is part of source semantics.

It MUST NOT hard-code scheduling decisions such as:

gate A starts at cycle 12

unless explicit low-level hardware programming is the intended semantics.

The scheduler determines feasible execution ordering and timing.

---

71. Hardware Mapping Boundary

Mapping abstract resources to physical resources is downstream.

For example:

logical qubit

may later map to:

physical qubit 17

The number "17" must not become part of portable source semantics unless explicitly requested by the program.

---

72. Optimization Boundary

Optimization MUST preserve language semantics.

The optimizer may choose:

- different algorithms;
- gate decompositions;
- layouts;
- memory strategies;
- parallelization;
- instruction selection.

It MUST NOT change the language's meaning.

---

73. Runtime Boundary

Runtime components execute already validated semantic/compiled representations.

Runtime MUST NOT parse source syntax as a substitute for compiler semantics except in explicitly defined tooling/interpreter modes.

---

74. Future Computing Boundary

The grammar must remain capable of incorporating future computing models.

Future domains should normally enter through:

existing core syntax
+
types
+
effects
+
capabilities
+
resources
+
dialects

rather than requiring redesign of the entire grammar.

---

75. Source-Level Portability Rule

A portable Zamani source program should primarily encode:

what

rather than:

where

and:

how on today's machine

unless the latter is explicitly part of the program's semantics.

---

76. Explicit Targeting

When a programmer genuinely needs a specific target, Zamani must permit explicit target intent.

Such intent must be distinguishable from ordinary portable semantics.

Conceptually:

portable semantic program
        +
explicit target constraint

rather than silently treating every source-level resource mention as target locking.

---

77. Compatibility Rule

Existing valid syntax MUST NOT be silently broken.

Before modifying syntax:

1. identify existing users;
2. identify tests;
3. identify AST consumers;
4. identify semantic consumers;
5. identify IR consumers;
6. determine compatibility impact;
7. provide migration if necessary;
8. update documentation and conformance tests.

---

78. Deprecation Rule

Deprecated syntax must remain documented until its removal policy is complete.

A deprecated construct must specify:

deprecated version
replacement
migration guidance
warning behavior
planned removal
compatibility impact

---

79. Removal Rule

A grammar construct may be removed only when:

- its status is "REMOVED";
- compatibility analysis exists;
- tests are updated;
- documentation is updated;
- parser/lexer support is removed consistently;
- AST/semantic/IR consumers no longer depend on it.

---

80. Authority Conflict Resolution

If two grammar artifacts disagree, resolution order is:

1. Current normative specification
2. Explicit compatibility policy
3. Stable language version
4. Canonical concrete grammar
5. Reference implementation
6. Extended design documents

However, an implementation discrepancy must never be silently ignored.

It must become a tracked conformance issue.

---

81. Implementation-First vs Specification-First Conflicts

If the implementation accepts syntax that is not specified:

implementation ≠ automatic language feature

If the specification defines syntax that the implementation does not accept:

specification ≠ automatic implementation

The lifecycle state must expose the discrepancy.

This prevents accidental language evolution through undocumented parser behavior.

---

82. Canonical Syntax Rule

For stable syntax, the following must converge:

grammar/specification/*
        │
        ▼
grammar/Zamani.g4
        │
        ├──── lexer
        │
        └──── parser
                │
                ▼
              AST
                │
                ▼
            semantics

No permanent divergence is permitted.

---

83. Grammar Generation Rule

If modular ".g4" files are introduced, they must have an explicitly defined ownership model.

There must still be one canonical generated/assembled grammar contract.

Generated artifacts MUST NOT be manually edited.

The source of generated grammar material must be documented.

---

84. Generated-File Rule

Any generated grammar/parser artifact must state:

GENERATED FILE
SOURCE:
GENERATOR:
VERSION:
DO NOT EDIT:

Generated output must be reproducible.

---

85. Source-of-Truth Rule for Modular Grammar

If syntax is split into:

grammar/lexer/
grammar/core/
grammar/types/
grammar/quantum/
grammar/hdl/
...

then modular files are implementation components of one language.

They are not independent language authorities.

Cross-file ownership MUST be explicitly documented.

---

86. No Domain-Owned Core Syntax

Quantum, HDL, AI, hardware, or other domain directories MUST NOT redefine core:

- identifiers;
- literals;
- expressions;
- declarations;
- types;
- modules;
- effects;
- capabilities;
- source locations.

They extend shared syntax.

---

87. Source Location Contract

Every syntax construct that reaches the AST MUST preserve sufficient source-location information for diagnostics.

This applies equally to:

- classical constructs;
- quantum constructs;
- HDL constructs;
- hardware constructs;
- distributed constructs;
- AI constructs;
- interoperability constructs.

---

88. Serialization Contract

If AST or semantic structures are serialized:

- version identifiers must be explicit;
- schema evolution must be defined;
- unknown future fields must be handled according to compatibility policy;
- serialization must not encode target-specific assumptions into portable semantics.

---

89. Resource Availability Rule

Resource availability is not grammar validity.

A program may be:

syntactically valid
semantically valid
resource-incompatible with current target

This is expected and must be diagnosable.

For example, a target may lack:

- sufficient memory;
- required accelerator;
- required qubit count;
- required connectivity;
- required precision;
- required timing;
- required capabilities.

Those failures belong to resource/target analysis.

---

90. Physical Reality Rule

POCO-REAF does not override physical constraints.

The language must permit programs to describe arbitrary scalable computations.

Execution remains subject to:

- finite memory;
- finite energy;
- finite time;
- target capability;
- physical laws;
- implementation limits.

The compiler must distinguish these facts from grammar restrictions.

---

91. Completion Definition

"grammar/specification/grammar-authority.md" is complete only when:

- authority hierarchy is defined;
- every grammar artifact has an owner;
- implementation/specification discrepancies have a lifecycle;
- "Zamani.g4" has a defined role;
- "grammar.md" has a defined role;
- "Zamani-Grammar.md" has a defined role;
- "grammar/specification/" has normative authority;
- lexer ownership is defined;
- parser ownership is defined;
- AST ownership is defined;
- semantic ownership is defined;
- IR ownership is defined;
- "quantum::ir" ownership is protected;
- QEC boundary is defined;
- ZQN boundary is defined;
- hardware boundary is defined;
- scheduling boundary is defined;
- runtime boundary is defined;
- interoperability boundary is defined;
- resource/capability/constraint/preference/hint distinctions are defined;
- POCO-REAF is defined;
- arbitrary hard-coded scalability limits are prohibited;
- Rust 1.97/1.97.1 compatibility is defined;
- Rust "unsafe" is prohibited;
- versioning is defined;
- compatibility is defined;
- testing requirements are defined;
- independent-file completion is defined;
- no-re-edit dependency ordering is defined.

---

92. Integration Contract With Other Specification Files

This document is the authority-of-authorities document.

It establishes relationships but does not duplicate every domain specification.

The following files must refine this document:

grammar/specification/language-principles.md
grammar/specification/language-scope.md
grammar/specification/language-version.md
grammar/specification/compatibility.md
grammar/specification/syntax-model.md
grammar/specification/semantic-model.md
grammar/specification/compilation-model.md
grammar/specification/execution-model.md
grammar/specification/scalability-model.md
grammar/specification/poco-reaf.md
grammar/specification/extensibility.md
grammar/specification/reserved-space.md

Topic-specific specifications under "grammar/spec/" may provide additional detailed contracts.

They MUST NOT contradict this authority document.

---

93. Integration Contract With "grammar/Zamani.g4"

"grammar/Zamani.g4" MUST:

1. implement the stable syntax defined by the specification;
2. avoid arbitrary machine limits;
3. avoid semantic actions;
4. preserve extensibility;
5. support grammar conformance testing;
6. remain version-aware;
7. integrate with the repository's AST/parser contract;
8. never become a second semantic IR.

---

94. Integration Contract With "grammar/grammar.md"

"grammar/grammar.md" MUST:

1. describe the reference implementation's accepted syntax;
2. identify implementation-only gaps;
3. identify unsupported specified features;
4. remain synchronized with lexer/parser changes;
5. never silently redefine normative syntax.

---

95. Integration Contract With "grammar/Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" MUST:

1. retain useful broad language-design material;
2. mark aspirational features;
3. avoid claiming implementation solely from syntax descriptions;
4. reference normative specifications;
5. avoid creating a competing language authority.

---

96. Integration Contract With "src/lexer.rs"

The lexer MUST:

- tokenize according to the canonical lexical contract;
- avoid semantic interpretation;
- preserve source spans;
- avoid fixed resource universes;
- maintain keyword compatibility;
- reject malformed lexical structures deterministically.

---

97. Integration Contract With "src/parser.rs"

The parser MUST:

- recognize canonical stable syntax;
- construct the canonical AST;
- preserve source spans;
- provide deterministic syntax errors;
- avoid target/resource validation;
- avoid fixed machine limits.

---

98. Integration Contract With "src/ast/"

The AST MUST:

- represent source semantics structurally;
- remain independent of physical targets;
- support domain composition;
- preserve source locations;
- remain extensible.

---

99. Integration Contract With "src/semantic.rs"

Semantic analysis MUST:

- resolve names;
- validate types;
- validate effects;
- validate capabilities;
- validate resource requirements;
- validate domain rules;
- distinguish syntax failures from semantic failures;
- avoid embedding target-specific grammar.

---

100. Integration Contract With "src/ir_gen.rs"

IR generation MUST:

- consume validated AST/semantic information;
- lower source semantics;
- preserve program meaning;
- avoid target-specific scheduling;
- route quantum semantics toward "quantum::ir".

---

101. Integration Contract With "src/ir_verify.rs"

IR verification MUST:

- verify IR invariants;
- reject malformed IR;
- remain independent from source grammar;
- avoid reconstructing source-language syntax.

---

102. Integration Contract With "src/quantum/ir/"

The quantum IR MUST remain the canonical semantic boundary for quantum computation.

Grammar changes MUST be integrated through:

source syntax
→ AST
→ semantic validation
→ quantum lowering
→ quantum::ir

Never:

source syntax
→ grammar-specific quantum IR
→ quantum::ir

The latter would create an unnecessary second representation.

---

103. Integration Contract With Optimization

Optimization consumes canonical IR.

It must not require grammar-level knowledge except through semantic metadata explicitly carried into IR.

Optimization cannot redefine language semantics.

---

104. Integration Contract With Scheduling

Scheduling consumes resource-aware semantic/IR representations.

It determines:

- ordering;
- timing;
- placement;
- concurrency;
- resource utilization.

It must not impose scheduling assumptions on the grammar.

---

105. Integration Contract With Hardware

Hardware subsystems provide:

- capabilities;
- target descriptions;
- resources;
- topology;
- calibration;
- placement information.

These are downstream from portable source semantics.

---

106. Integration Contract With Runtime

Runtime consumes compiled representations and runtime capability/resource information.

Runtime must not become a grammar authority.

---

107. Integration Contract With Tests

Every stable syntax rule must be represented by tests.

Tests are evidence of conformance, not an independent language specification.

---

108. Integration Contract With Tooling

LSPs, formatters, syntax highlighters, package tools, documentation generators, and other tools must consume the canonical syntax contract.

Tooling must not invent syntax independently.

---

109. Repository-Wide Invariant

The following invariant is mandatory:

One Language
One Semantic Model
One Authority
Many Representations
Many Targets
Many Execution Environments

Not:

Many Grammars
Many Meanings
Many Compilers
Many Incompatible Languages

---

110. Production-Readiness Gate

The grammar authority architecture is production-ready only when the repository can answer, for every syntax construct:

Where is it specified?
Where is it lexed?
Where is it parsed?
Where is it represented in the AST?
Where is it semantically validated?
Where is its canonical IR?
Who owns its optimization?
Who owns its scheduling?
Who owns its target mapping?
Who owns its runtime behavior?
How is it tested?
How is it versioned?
What are its scalability rules?
What happens when the target cannot satisfy it?

No "unknown owner" is permitted for a stable feature.

---

111. Final Architectural Law

The ultimate rule governing Zamani grammar is:

«Zamani describes computation, intent, capabilities, constraints, resources, and semantics—not arbitrary limitations of the machine currently available.»

Therefore:

One Program
     ↓
One Semantic Meaning
     ↓
Many Compilation Strategies
     ↓
Many Architectures
     ↓
Many Hardware Configurations
     ↓
Many Resource Scales
     ↓
Many Execution Environments
     ↓
Future Platforms

The source language must remain stable while the implementation universe expands.

---

112. POCO-REAF Final Contract

Zamani MUST target:

Program_Once
Compile_Once
Run_Everywhere
Run_Anywhere
Run_Forever

subject to:

program semantics
+
declared requirements
+
declared constraints
+
target capabilities
+
available resources
+
physical reality

The language itself MUST NOT impose arbitrary finite limits on the computational universe.

Thus the governing relationship is:

                         Zamani
                           │
                           ▼
                  Portable Semantics
                           │
             ┌─────────────┼─────────────┐
             ▼             ▼             ▼
        Classical       Quantum          HDL
             │             │             │
             └─────────────┼─────────────┘
                           ▼
                    Canonical IRs
                           │
                           ▼
                 Resource/Capability
                      Resolution
                           │
                           ▼
                  Target Compilation
                           │
          ┌────────────────┼────────────────┐
          ▼                ▼                ▼
        Tiny             Large            Future
       Machine           Machine          Machine
          │                │                │
          └────────────────┼────────────────┘
                           ▼
                     Execution

The grammar defines the language.

The AST defines source structure.

Semantic analysis defines validity.

IR defines canonical computation.

Optimization transforms computation without changing meaning.

Scheduling determines feasible execution.

Hardware determines physical realization.

Runtime performs execution.

Resources determine what is possible now.

The language does not inherit arbitrary limits from any one of them.

This is the authoritative grammar architecture for Zamani's From Atom to Everywhere and POCO-REAF model.This file deliberately makes the authority relationship explicit rather than declaring Zamani.g4, grammar.md, or Zamani-Grammar.md independently authoritative. That is important because the current repository already documents those surfaces as materially different, while the Rust frontend and quantum::ir have their own concrete implementation ownership.

It also preserves the existing repository rule that quantum::ir is the canonical quantum semantic boundary rather than allowing the grammar to become a second quantum IR. 