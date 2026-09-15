Zamani Language Specification

Path: "grammar/specification/README.md"
Status: Normative specification architecture
Language: Zamani
Compiler family: Zamani Compiler / ZUTC
Rust baseline: Rust 1.97 / Rust 1.97.1
Safety requirement: Safe Rust only; "unsafe" Rust is prohibited
Portability model: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability model: From the smallest representable computation to arbitrarily large computation, subject only to program semantics, available resources, target capabilities, physical constraints, and implementation capacity.

---

1. Purpose

"grammar/specification/" is the normative language-specification authority for Zamani.

It defines the language contracts that must remain stable across compiler implementations, parser implementations, hardware generations, execution environments, and future computing paradigms.

This directory defines:

- the identity and principles of Zamani;
- the language model;
- lexical and syntactic contracts;
- semantic contracts;
- type-system contracts;
- effect contracts;
- resource and capability contracts;
- portability and scalability contracts;
- classical-computing contracts;
- quantum-computing contracts;
- hybrid-computing contracts;
- HDL and hardware co-design contracts;
- distributed and parallel-computing contracts;
- AI/ML contracts;
- data and tensor-computing contracts;
- networking contracts;
- security and cryptography contracts;
- interoperability contracts;
- metaprogramming and macro contracts;
- dialect and extension contracts;
- diagnostics contracts;
- determinism and provenance contracts;
- versioning and compatibility contracts;
- feature lifecycle and conformance contracts.

This directory does not implement the lexer, parser, AST, semantic analyzer, IR, compiler, runtime, scheduler, router, QEC system, ZQN system, HAL, or backend.

Those components implement contracts defined here.

---

2. Normative Status

This document establishes the authority model for the entire "grammar/" tree.

The following distinction is mandatory:

«Specification describes what Zamani means. Implementation determines how Zamani realizes that meaning.»

A document must never be considered authoritative merely because it is larger, older, generated, or more detailed.

The specification hierarchy is:

grammar/specification/
        │
        │ normative language contracts
        ▼
grammar/Zamani.g4
        │
        │ canonical ANTLR syntax representation
        ▼
src/lexer.rs + src/parser.rs
        │
        ▼
src/frontend/ast/
        │
        ▼
semantic analysis
        │
        ▼
canonical semantic IR
        │
        ├───────────────┐
        ▼               ▼
classical IR       quantum::ir
        │               │
        └───────┬───────┘
                ▼
        optimization
                │
                ▼
       target-aware lowering
                │
       ┌────────┼─────────┐
       ▼        ▼         ▼
   scheduling routing resilience
       │        │         │
       └────────┼─────────┘
                ▼
               ZQN
                │
               HAL
                │
             backend
                │
          runtime/deployment

Every implementation layer must preserve the meaning established by the specification.

---

3. Specification Authority Model

Zamani currently contains several language-description surfaces.

They have distinct and non-overlapping responsibilities.

3.1 "grammar/specification/"

This is the normative language specification.

It answers:

- What does Zamani mean?
- What language constructs exist?
- What are their semantic guarantees?
- What are their portability guarantees?
- What are their resource semantics?
- What are their compatibility requirements?
- What is required for a feature to be considered complete?

A feature may be specified here before implementation exists.

Such a feature MUST carry an explicit lifecycle state.

Specification does not imply implementation.

---

3.2 "grammar/Zamani.g4"

"grammar/Zamani.g4" is the canonical ANTLR grammar composition root.

It is an executable representation of the language syntax.

It must conform to this specification.

It must not become an independent language authority.

It must not introduce semantics that contradict this specification.

It must not encode arbitrary hardware limits.

It must not create a second quantum semantic model.

It must not silently accept features that the specification has explicitly forbidden.

---

3.3 "grammar/grammar.md"

"grammar/grammar.md" is the implementation-conformance grammar/reference.

It describes the syntax actually supported by the reference frontend.

It must distinguish at least:

SPECIFIED
IMPLEMENTED
PARTIALLY_IMPLEMENTED
EXPERIMENTAL
DEPRECATED
UNIMPLEMENTED

It must never present planned syntax as implemented syntax.

Where generated, its generation process must be deterministic and reproducible.

---

3.4 "grammar/Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" is retained.

It is a broad language-design/reference surface and may contain historical, proposed, experimental, or aspirational constructs.

It is not automatically normative.

Every feature described there must have an explicit lifecycle status.

A feature becomes normative only after it has been promoted into the specification and passed the required implementation and conformance process.

---

3.5 "grammar/README.md"

The parent "grammar/README.md" is the navigation and architecture entry point.

It must explain:

- authority;
- directory structure;
- workflow;
- contribution process;
- feature lifecycle;
- validation;
- relationship between grammar and implementation.

It must not become another competing grammar authority.

---

3.6 "grammar/spec/"

"grammar/spec/" contains formalized, focused specification contracts.

The relationship is:

grammar/specification/
        │
        │ normative language architecture
        ▼
grammar/spec/
        │
        │ focused machine-checkable/design contracts
        ▼
grammar/Zamani.g4 + implementation

"spec/" must not contradict "specification/".

If a contradiction is found, it must be resolved through an explicit specification change rather than silently choosing whichever document was edited most recently.

---

4. Single-Language Principle

Zamani is one programming language.

The following are domains of one language:

- classical;
- quantum;
- hybrid quantum-classical;
- HDL;
- hardware/software co-design;
- embedded;
- systems;
- distributed;
- parallel/HPC;
- AI/ML;
- data;
- tensor;
- networking;
- cryptography;
- scientific computing;
- edge;
- cloud;
- accelerator programming;
- future computational paradigms.

They must share common foundations.

                         ZAMANI
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
      Core              Domains           Extensions
        │                  │                  │
 expressions          classical            macros
 statements           quantum              dialects
 declarations         hybrid               metaprogramming
 types                HDL                  future domains
 modules              hardware
 effects              distributed
 memory               AI
 concurrency          data
                       networking
                       security

A domain may add syntax and semantics.

It must not create an incompatible language inside Zamani.

---

5. Dependency Direction

The dependency direction is mandatory:

Specification
     ↓
Lexical Contract
     ↓
Syntax Contract
     ↓
AST Contract
     ↓
Semantic Contract
     ↓
Canonical IR Contract
     ↓
Optimization
     ↓
Target-Aware Lowering
     ↓
Routing / Scheduling / Resilience
     ↓
HAL / ZQN / Backend
     ↓
Runtime / Deployment

A downstream layer must not redefine an upstream concept.

For example:

- a backend must not redefine the meaning of a Zamani type;
- a scheduler must not redefine quantum operation semantics;
- HAL must not define source-language syntax;
- QEC must not create a second quantum language;
- ZQN must not become another quantum IR;
- hardware topology must not become a universal source-language topology;
- runtime behavior must not silently redefine grammar.

---

6. Repository Integration Contract

The specification integrates with the existing repository as follows.

Repository component| Primary responsibility
"grammar/specification/"| Normative language specification
"grammar/spec/"| Focused formal contracts
"grammar/Zamani.g4"| Canonical ANTLR syntax
"grammar/grammar.md"| Current implementation grammar/conformance
"grammar/Zamani-Grammar.md"| Broad design/reference material
"grammar/lexer/"| Lexical contracts
"grammar/core/"| Universal language syntax
"grammar/types/"| Type syntax
"grammar/expressions/"| Expression syntax
"grammar/statements/"| Statement syntax
"grammar/declarations/"| Declaration syntax
"grammar/functions/"| Function syntax
"grammar/modules/"| Module/package syntax
"grammar/effects/"| Effect syntax
"grammar/memory/"| Memory/ownership syntax
"grammar/concurrency/"| Concurrency syntax
"grammar/classical/"| Classical domain syntax
"grammar/quantum/"| Quantum domain syntax
"grammar/hybrid/"| Hybrid domain syntax
"grammar/hdl/"| HDL syntax
"grammar/hardware/"| Hardware intent/capability syntax
"grammar/resources/"| Resource requirements and constraints
"grammar/distributed/"| Distributed computing syntax
"grammar/ai/"| AI/ML syntax
"grammar/data/"| Data/tensor syntax
"grammar/networking/"| Networking syntax
"grammar/security/"| Security syntax
"grammar/compile/"| Compilation/build intent
"grammar/execution/"| Execution/runtime intent
"grammar/interoperability/"| Foreign/interoperability syntax
"grammar/dialects/"| Controlled language extensions
"grammar/macros/"| Macro syntax
"grammar/metaprogramming/"| Compile-time/reflection syntax
"grammar/validation/"| Specification and grammar validation
"grammar/compatibility/"| Versioning and compatibility
"grammar/tests/"| Conformance testing
"src/lexer.rs"| Reference lexical implementation
"src/parser.rs"| Reference parser implementation
"src/frontend/ast/"| Domain-neutral source AST
"src/semantic.rs"| Semantic analysis
"src/ir_gen.rs"| AST/semantic lowering into IR
"src/ir_verify.rs"| IR validation
"src/quantum/ir/"| Canonical quantum semantic boundary
optimization| Semantics-preserving optimization
routing| Physical realization/mapping
scheduling| Timing/resource scheduling
QEC| Quantum error correction
ZQN| Quantum fault/noise semantics
HAL| Hardware capability/state abstraction
runtime| Execution
backends| Target realization

These ownership boundaries are normative.

---

7. Feature Completion Contract

Every language feature must be independently completable.

Before implementation begins, the feature must have answers for:

Feature identity
Purpose
Normative status
Syntax
Lexical requirements
Parser rule
AST representation
Structural invariants
Semantic meaning
Type interaction
Effect interaction
Capability interaction
Resource interaction
Canonical IR representation
Compiler consumers
Optimizer consumers
Scheduler consumers
Runtime consumers
Backend consumers
Diagnostics
Source-span requirements
Serialization requirements
Interoperability requirements
Compatibility requirements
Positive tests
Negative tests
Boundary tests
Scalability tests
Determinism tests
Security tests
Hard-coding audit
Completion criteria

A feature is not production-ready until every applicable item is resolved.

---

8. Independent-File Completion Rule

Every specification file must be independently completable.

A specification file must explicitly declare:

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
IR integration
Compiler integration
Runtime integration
Tooling integration
Cross-domain integration
Diagnostics
Compatibility
Scalability
Determinism
Security
Hard-coding audit
Tests
Negative tests
Boundary tests
Completion criteria

A file must not contain unresolved statements such as:

TBD
TODO
figure this out later
implementation-defined
future AST
future IR
backend decides

unless the exact lifecycle and owner of that unresolved decision are explicitly specified.

---

9. No-Rework Integration Principle

Implementation must proceed dependency-first.

A lower-level contract must be finalized before dependent contracts are declared complete.

The intended order is:

Language principles
        ↓
Lexical specification
        ↓
Syntax model
        ↓
AST contract
        ↓
Semantic model
        ↓
IR contract
        ↓
Compiler contract
        ↓
Runtime contract
        ↓
Target integration

A new feature must integrate with existing contracts rather than forcing unrelated completed files to be rewritten.

If an established contract genuinely must change, the change must be versioned and compatibility-reviewed.

---

10. Source-to-Execution Contract

The canonical Zamani pipeline is:

Zamani source
     ↓
Lexer
     ↓
Parser
     ↓
Domain-neutral AST
     ↓
Structural validation
     ↓
Name/module resolution
     ↓
Type analysis
     ↓
Effect analysis
     ↓
Capability analysis
     ↓
Resource/constraint analysis
     ↓
Semantic model
     ↓
Canonical IR
     ↓
Domain IR
     ↓
Optimization
     ↓
Target-aware lowering
     ↓
Routing / mapping
     ↓
Scheduling
     ↓
Resilience / recovery
     ↓
ZQN where applicable
     ↓
HAL
     ↓
Backend
     ↓
Runtime
     ↓
Deployment

No stage may silently skip semantic validation.

---

11. Domain-Neutral AST Requirement

The source AST must represent source structure and portable language meaning.

It must not become:

- LLVM IR;
- QIR;
- MLIR;
- vendor IR;
- hardware topology;
- physical qubit mapping;
- QEC implementation;
- calibration state;
- routing schedule;
- runtime device state.

The AST belongs to the language frontend.

Domain lowering happens after semantic analysis.

For example:

source
  ↓
Operation {
    name
    namespace
    operands
    parameters
    results
    attributes
    modifiers
    effects
    capabilities
    source
}

must remain generic enough to represent future operations without requiring an ever-growing enumeration of every possible operation.

---

12. Quantum IR Boundary

"src/quantum/ir/" is the canonical semantic quantum boundary.

The grammar must parse quantum syntax.

The AST must represent quantum source structure.

Semantic analysis determines validity.

Quantum lowering produces canonical "quantum::ir".

The grammar must not create another quantum IR.

The pipeline is:

Quantum source
      ↓
Zamani lexer/parser
      ↓
Domain-neutral AST
      ↓
Semantic quantum model
      ↓
src/quantum/ir/
      ↓
optimization
      ↓
routing
      ↓
scheduling
      ↓
QEC
      ↓
ZQN
      ↓
HAL
      ↓
target backend

This boundary is mandatory.

---

13. Quantum Operation Design

The language must not permanently enumerate every possible quantum gate.

Avoid a grammar model equivalent to:

gate
  : H
  | X
  | Y
  | Z
  | CNOT
  | ...

Such a design does not scale to:

- future gates;
- parameterized gates;
- user-defined operations;
- logical operations;
- vendor operations;
- decomposed operations;
- fault-tolerant operations;
- pulse-derived operations;
- future quantum paradigms.

Prefer a compositional operation model.

Conceptually:

operation-specifier
        +
parameters
        +
modifiers
        +
targets
        +
attributes

The semantic layer determines what the operation means.

---

14. Quantum Resource Independence

The language must never define an artificial maximum for:

- qubits;
- registers;
- logical qubits;
- physical qubits;
- controls;
- circuit depth;
- measurement count;
- operation count.

A program may explicitly request a finite quantity.

For example:

requires qubits >= n

is semantic program intent.

A compiler implementation must not reinterpret this as:

n <= MAX_QUBITS

unless the limit is imposed by the actual target/resource environment.

---

15. Classical Computing

Classical computation is a first-class Zamani domain.

The language must support composable:

- scalar computation;
- integer computation;
- floating-point computation;
- arbitrary-precision abstractions where supported;
- vectors;
- matrices;
- tensors;
- symbolic computation;
- numerical computation;
- scientific computation;
- signal processing;
- control;
- optimization;
- parallel computation;
- accelerator computation.

The grammar must not turn every library function into a reserved keyword.

Mathematical operations should generally be represented through:

types
+
generic operations
+
intrinsics
+
libraries
+
capabilities
+
semantic constraints

rather than an unbounded keyword catalogue.

---

16. HDL and Hardware Co-Design

HDL is a first-class Zamani domain.

Zamani must be capable of expressing:

- modules;
- ports;
- signals;
- nets;
- registers;
- combinational logic;
- sequential logic;
- state machines;
- pipelines;
- memories;
- interfaces;
- timing intent;
- reset behavior;
- clocking intent;
- verification properties;
- synthesis intent;
- simulation intent;
- physical constraints;
- hardware/software co-design.

Hardware syntax describes intent.

It must not accidentally turn a particular FPGA, ASIC, CPU, GPU, accelerator, or QPU into the universal Zamani machine model.

---

17. Hardware Independence

Hardware-dependent information belongs in the hardware/capability/resource layers.

Source code should preferably express:

requires capability("tensor.compute")
requires capability("quantum.mid_circuit_measurement")
requires memory >= required_memory
requires communication >= required_bandwidth
prefer accelerator("quantum")

rather than:

use_gpu_0
use_qpu_3
use_core_7
use_qubit_42

The latter are realization decisions.

The language must preserve the distinction between portable intent and target realization.

---

18. Resource Semantics

Zamani distinguishes:

Requirement

Necessary for correctness.

requires capability("quantum")

Capability

A target property.

supports capability("quantum")

Constraint

A property that must be satisfied.

constraint latency < limit

Preference

A desirable property.

prefer low_energy

Hint

Optimization information that does not alter correctness.

hint locality

Resource

An actual execution resource.

Examples:

qubit
core
memory
device
accelerator
node
link
storage

Target

A concrete environment capable of executing the program.

These concepts must not be collapsed.

---

19. POCO-REAF

Zamani's primary portability principle is:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

This means that source semantics must not be tied unnecessarily to a particular target.

POCO-REAF does not mean that every program can execute on every physical machine regardless of capability.

The following are distinct:

Syntax validity
≠
Semantic validity
≠
Compilation validity
≠
Target compatibility
≠
Resource availability
≠
Runtime availability

A program may therefore be:

valid Zamani
+
semantically correct
+
portable
+
not executable on the current target

because the target lacks required resources or capabilities.

That is a target/resource failure, not a language failure.

---

20. Scalability and the Meaning of Infinity

Zamani must scale from tiny computation to arbitrarily large computation subject to resources.

"Infinite scalability" means:

«The language itself must not impose arbitrary finite limits merely because a current implementation or hardware generation has such limits.»

There must be no language-defined universal maximum for:

- program size;
- declarations;
- functions;
- modules;
- expressions;
- data;
- tensor dimensions;
- tensor elements;
- nodes;
- processes;
- threads;
- tasks;
- devices;
- accelerators;
- CPUs;
- GPUs;
- FPGAs;
- QPUs;
- qubits;
- memory;
- storage;
- timelines;
- execution stages;
- communication endpoints.

Real systems may have limits.

Those limits belong to:

- implementation resources;
- operating systems;
- target capabilities;
- resource budgets;
- physical constraints;
- numerical representations;
- compilation capacity.

They must not silently become grammar limits.

---

21. Absolute No-Hard-Coding Rule

The grammar and language specification must not contain arbitrary universal hardware limits.

Prohibited examples:

MAX_QUBITS = 32
MAX_QUBITS = 64
MAX_CPUS = 128
MAX_THREADS = 1024
MAX_GPUS = 8
MAX_FPGAS = 4
MAX_NODES = 1024
MAX_MEMORY = ...
MAX_TENSOR_RANK = 32
MAX_VECTOR_WIDTH = 512

Equivalent hidden restrictions are also prohibited.

The same rule applies to:

- fixed topology;
- fixed physical addresses;
- fixed device identifiers;
- fixed accelerator counts;
- fixed register widths;
- fixed memory-bank counts;
- fixed network sizes;
- fixed cluster sizes.

---

22. Constants Are Not Hardware Limits

The no-hard-coding rule does not prohibit ordinary program constants.

This is valid:

let n = 1024;

This is also valid:

matrix<1024, 1024>

if those numbers are program semantics.

What is prohibited is turning the value into an implementation-wide language limit:

MAX_MATRIX_DIM = 1024

or:

grammar only permits dimensions <= 1024

The distinction is:

Program semantics
        ≠
Implementation limitation

---

23. Hardware Realization

Target realization is downstream.

The source program describes intent.

The compiler and runtime determine how to realize that intent using:

- available hardware;
- capabilities;
- resources;
- topology;
- scheduling;
- routing;
- calibration;
- optimization;
- resilience;
- deployment policies.

A physical mapping such as:

logical q0 → physical q17

is not universal source semantics.

It belongs downstream.

---

24. Classical / Quantum / HDL Integration

The language must support programs that combine:

classical computation
        ↓
quantum operation
        ↓
measurement
        ↓
classical decision
        ↓
quantum operation
        ↓
hardware interaction

The same language model must support:

software
+
quantum
+
hardware
+
data
+
AI
+
distributed execution

without requiring separate languages.

---

25. Hybrid Computing

Hybrid computation is first-class.

The specification must permit semantic boundaries between:

- classical state;
- quantum state;
- hardware state;
- accelerator state;
- distributed state.

The language must support:

- measurement;
- classical feed-forward;
- dynamic circuits;
- host/device interaction;
- shared data;
- synchronization;
- resource transitions;
- asynchronous execution.

These are language semantics.

Physical execution is downstream.

---

26. Distributed and Parallel Computing

The language must support:

- tasks;
- processes;
- actors;
- services;
- messages;
- channels;
- collective operations;
- partitioning;
- replication;
- placement intent;
- synchronization;
- consistency requirements;
- fault tolerance;
- parallel algorithms;
- data parallelism;
- task parallelism;
- pipeline parallelism;
- distributed computation.

The grammar must not hard-code:

8 workers
16 nodes
32 processes
1024 threads

unless the number is explicitly part of program semantics.

---

27. AI and Machine Learning

AI/ML is a first-class domain.

The specification may support:

- models;
- tensors;
- datasets;
- training;
- inference;
- optimization;
- differentiation;
- probabilistic computation;
- symbolic computation;
- neural computation;
- agents;
- model pipelines;
- distributed training;
- model deployment.

The core language must not become a framework-specific language.

Frameworks and vendor systems belong to interoperability and backend layers.

---

28. Data and Tensor Computing

Data computation must support:

- collections;
- records;
- schemas;
- tables;
- streams;
- tensors;
- datasets;
- transformations;
- queries;
- pipelines;
- serialization;
- persistence;
- provenance.

Tensor shapes and data sizes must remain parameterizable.

No arbitrary universal tensor dimension or rank limit may be encoded into the language.

---

29. Networking

Networking syntax may express:

- endpoints;
- protocols;
- channels;
- services;
- messages;
- streaming;
- requests;
- responses;
- routing intent;
- service discovery;
- distributed communication.

The source language must avoid unnecessary dependence on:

- fixed IP addresses;
- fixed hardware interfaces;
- fixed node counts;
- fixed topology.

Those belong to deployment and target configuration when required.

---

30. Security

Security is part of the language architecture.

The specification may support:

- identity;
- authorization;
- capability security;
- policy;
- secrets;
- cryptographic intent;
- hashes;
- signatures;
- key management;
- secure computation;
- zero-knowledge intent;
- provenance;
- trust.

The grammar should describe semantic security intent rather than turning every cryptographic algorithm into permanent language syntax.

---

31. Effects

Effects describe computational behavior that may cross domain boundaries.

The effect system must be capable of representing concepts such as:

- I/O;
- mutation;
- concurrency;
- allocation;
- networking;
- quantum interaction;
- hardware interaction;
- nondeterminism;
- external resources.

Effects belong to language semantics.

They must not be confused with implementation-specific runtime APIs.

---

32. Memory and Ownership

The memory model must be sufficiently general for:

- ordinary memory;
- references;
- ownership;
- borrowing;
- regions;
- persistent storage;
- shared memory;
- distributed memory;
- accelerator memory;
- device memory;
- quantum resources where applicable.

The language must not encode today's physical memory hierarchy as the universal semantic model.

---

33. Determinism

Where deterministic semantics are promised, the implementation must preserve them across:

- parsing;
- semantic analysis;
- lowering;
- optimization;
- serialization;
- compilation;
- deployment metadata.

Nondeterminism must be explicit where it is semantically relevant.

Randomness must not be introduced accidentally by compiler implementation choices.

---

34. Provenance and Reproducibility

Production Zamani implementations should preserve sufficient provenance to determine:

- source version;
- language version;
- dialect versions;
- feature versions;
- compiler version;
- relevant compilation configuration;
- semantic transformations;
- target requirements;
- generated artifacts.

Provenance metadata must not change program semantics.

---

35. Diagnostics

Every syntactic and semantic feature must define diagnostics.

Diagnostics must provide, where applicable:

- stable diagnostic identity;
- severity;
- source span;
- primary message;
- contextual explanation;
- relevant notes;
- actionable remediation;
- related spans.

Diagnostics must not expose secrets.

Diagnostics must not depend on target-specific implementation details when reporting source-language semantics.

---

36. Error Recovery

Parser error recovery must:

- make progress;
- avoid infinite loops;
- preserve source locations;
- avoid inventing valid semantic constructs;
- produce deterministic diagnostics;
- permit useful recovery where supported.

Error recovery must never be mistaken for successful semantic validation.

---

37. Versioning

Every normative language feature belongs to a language version.

Feature lifecycle:

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

Possible later states:

DEPRECATED
    ↓
REMOVED

A feature must not be declared stable merely because syntax parses.

---

38. Compatibility

Compatibility must be evaluated across:

Specification
Zamani.g4
Lexer
Parser
AST
Semantic analysis
IR
Compiler
Runtime
Backends
Interoperability
Tests

A compatibility change must document:

- old behavior;
- new behavior;
- migration;
- source compatibility;
- semantic compatibility;
- binary/artifact implications where applicable;
- dialect implications;
- deprecation period;
- test coverage.

---

39. Additive Evolution

Zamani should evolve additively whenever practical.

Prefer:

existing syntax
+
generic composition
+
typed extension
+
attributes
+
capabilities
+
dialects

over:

new keyword
+
new parser branch
+
new AST enum
+
new IR special case

Every new keyword creates permanent compatibility obligations.

Therefore keywords must be justified.

---

40. Dialects

Dialects are controlled extensions of Zamani.

A dialect must declare:

name
version
owner
base language version
syntax extensions
semantic extensions
AST mapping
IR mapping
capabilities
compatibility
feature gates
tests

A dialect must not silently change the meaning of existing standard Zamani constructs.

Dialect syntax must remain identifiable.

---

41. Interoperability

Zamani must be capable of interoperating with external systems.

Examples include:

- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL formats;
- accelerator APIs;
- serialization formats.

These are interoperability mechanisms.

They are not competing canonical Zamani semantic models.

For quantum:

Zamani
  ↓
canonical quantum::ir
  ↓
OpenQASM/QIR/vendor format

is preferable to:

Zamani
  ↓
OpenQASM-specific semantic model

---

42. Macros

Macros may extend source syntax.

They must:

- preserve hygiene;
- preserve source locations;
- remain compatible with semantic validation;
- not bypass type checking;
- not bypass capability checking;
- not bypass security validation;
- not create hidden hardware dependencies.

Macro expansion is not permission to create a second language.

---

43. Metaprogramming

Metaprogramming may provide:

- reflection;
- compile-time computation;
- quotation;
- code generation;
- type-level computation;
- schema generation.

It must remain bounded by explicit compiler/security/resource policies.

Metaprogramming must not become an unrestricted mechanism for silently modifying language semantics.

---

44. Resource Negotiation

Resource negotiation is essential to POCO-REAF.

A portable program may express:

requirement
constraint
preference
hint
fallback
capability

The compiler/runtime may then select an implementation appropriate to the target.

The selection process must preserve semantic correctness.

If no valid realization exists, compilation or deployment must fail explicitly.

It must not silently change program meaning.

---

45. Fallbacks

Where a program permits alternatives, the alternatives must be explicit.

Conceptually:

requires capability A
otherwise use semantic alternative B

A fallback must preserve the required semantic contract.

A backend must not silently substitute an approximation where exact semantics were required.

---

46. Optimization Contract

Optimization may change implementation.

It must not change language meaning unless the language explicitly permits the transformation.

Optimization must preserve:

- observable semantics;
- type correctness;
- effect requirements;
- resource correctness;
- quantum semantics;
- hardware intent;
- determinism guarantees.

Optimization belongs downstream from the canonical semantic representation.

---

47. Scheduling Contract

Scheduling determines execution order and timing.

It may consider:

- dependencies;
- resources;
- topology;
- latency;
- concurrency;
- hardware capabilities;
- timing constraints.

Scheduling must not redefine the semantics of the source program.

For quantum programs, scheduling is downstream from "quantum::ir".

---

48. Routing Contract

Routing maps abstract computation to physical connectivity.

It may consider:

- topology;
- communication cost;
- device availability;
- calibration;
- resource constraints.

Routing must not introduce physical mappings into the portable source semantics.

---

49. QEC Contract

Quantum error correction is a downstream responsibility.

The language may express:

error-correction intent
fault-tolerance requirement
logical reliability requirement
noise tolerance requirement

but QEC implementation belongs to the QEC subsystem.

The grammar must not implement QEC.

---

50. ZQN Contract

ZQN represents quantum fault/noise semantics and related execution concerns.

The grammar may express noise/fault requirements.

It must not duplicate ZQN's semantic model.

The dependency remains:

source intent
    ↓
quantum::ir
    ↓
QEC / optimization / routing / scheduling
    ↓
ZQN

---

51. HAL Contract

HAL exposes target capabilities and state.

The source language may request capabilities.

HAL determines actual availability.

The source language must not assume that a particular HAL implementation, device ID, topology, calibration table, or physical qubit map exists.

---

52. Backend Contract

Backends are responsible for target realization.

A backend may specialize for:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- accelerator;
- embedded system;
- distributed system;
- future target.

A backend must preserve source semantics.

Backend limitations must be reported explicitly.

They must not be encoded retroactively as universal grammar limitations.

---

53. Safe Rust Requirement

The reference implementation baseline is:

Rust 1.97 / Rust 1.97.1

Production Zamani compiler/frontend implementation must use safe Rust.

"unsafe" Rust is prohibited.

This includes avoiding hidden unsafe requirements in:

- lexer;
- parser;
- AST;
- semantic analysis;
- grammar tooling;
- validation;
- test infrastructure.

The language specification itself must remain independent of Rust implementation details.

---

54. No Rust-Specific Language Leakage

Zamani is not Rust syntax.

Rust implementation choices must not become language semantics merely because the compiler is implemented in Rust.

For example:

- Rust enum layout is not a Zamani language rule;
- Rust ownership implementation is not automatically Zamani ownership semantics;
- Rust integer widths are not automatically Zamani semantic limits;
- Rust allocation limits are not language limits.

Where Zamani intentionally adopts a semantic concept analogous to Rust, that concept must be specified independently.

---

55. Testing Contract

Every feature requires:

positive tests
negative tests
boundary tests
scalability tests
compatibility tests
diagnostic tests
determinism tests

Where applicable:

security tests
resource tests
cross-domain tests
interoperability tests
serialization tests
compiler integration tests
runtime integration tests

A grammar-only test is insufficient for a production language feature.

---

56. Scalability Tests

Scalability tests must verify that the language does not accidentally impose finite limits.

Examples include:

- increasing program size;
- increasing declaration count;
- increasing module count;
- increasing expression depth;
- increasing tensor dimensions;
- increasing quantum register size;
- increasing distributed worker count;
- increasing task count;
- increasing device count;
- increasing topology size.

Tests must distinguish:

language limitation

from:

implementation/resource limitation

---

57. Hard-Coding Audit

Every grammar and specification change must be audited for:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_DEVICES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_TIMELINES
MAX_PROCESSES
MAX_TASKS

The audit must also identify hidden equivalent restrictions.

The absence of suspicious names is not sufficient.

Semantic analysis must verify that the grammar does not accidentally establish an artificial finite universe.

---

58. Source Span Contract

Every source construct that can produce a diagnostic or semantic transformation must retain sufficient source-location information.

Source spans must survive:

lexer
→ parser
→ AST
→ semantic analysis
→ diagnostics
→ lowering

Generated constructs must retain provenance to their originating source where practical.

---

59. Security Contract

The specification and implementation must prevent:

- hidden capability escalation;
- implicit privileged hardware access;
- secret leakage;
- uncontrolled macro expansion;
- unrestricted metaprogramming;
- malformed-input denial of service where preventable;
- ambiguous security semantics.

Security-relevant constructs must have explicit semantics.

---

60. Deterministic Build and Reproducibility Contract

Where deterministic compilation is promised, the compiler must produce reproducible semantic artifacts from identical:

source
+
language version
+
dialect versions
+
compiler version
+
relevant configuration

Target-specific realization may vary where the target is intentionally different.

Such variation must not silently alter portable program meaning.

---

61. Feature Manifests

For complex features, the repository should maintain a machine-readable feature contract under:

grammar/specification/features/

Each feature manifest should identify:

id
name
status
version
grammar
tokens
AST nodes
semantic rules
IR mapping
compiler consumers
runtime consumers
domain
capabilities
resource requirements
negative tests
boundary tests
scalability tests
compatibility
hard-coding policy

The manifest is an integration index.

It does not replace the normative prose specification.

---

62. Cross-Domain Feature Integration

A feature that crosses domains must identify every participating domain.

For example, hybrid quantum-classical execution may involve:

expressions
types
effects
quantum
classical
hybrid
resources
execution
hardware
compile

The feature contract must identify:

syntax
AST
semantic model
IR
quantum::ir integration
resource model
compiler integration
runtime integration
tests

This prevents one domain from silently creating incompatible semantics.

---

63. Grammar Composition

The root grammar:

grammar/Zamani.g4

is the composition root.

Domain grammars must ultimately compose into the same language.

Conceptually:

Zamani.g4
    │
    ├── core
    ├── lexer
    ├── types
    ├── expressions
    ├── statements
    ├── declarations
    ├── functions
    ├── modules
    ├── effects
    ├── memory
    ├── concurrency
    ├── classical
    ├── quantum
    ├── hybrid
    ├── hdl
    ├── hardware
    ├── distributed
    ├── ai
    ├── data
    ├── networking
    ├── security
    ├── resources
    ├── compile
    ├── execution
    ├── interoperability
    ├── dialects
    ├── macros
    └── metaprogramming

No second root grammar may silently compete with "Zamani.g4".

---

64. "grammar/antlr/"

An existing "grammar/antlr/" directory must not become a second grammar authority.

Before removal or retention, repository references must be checked.

If it contains required tooling, it may remain as tooling support.

If it contains obsolete duplicate grammar authority, it should eventually be removed.

The canonical language grammar remains:

grammar/Zamani.g4

No duplicate "Zamani.g4" should exist merely to create another authority.

---

65. Documentation Authority

Documentation has three distinct categories:

Normative

Defines what Zamani means.

grammar/specification/

Conformance

Describes what the current implementation accepts.

grammar/grammar.md

Design/history/reference

Contains broader or historical material.

grammar/Zamani-Grammar.md

This distinction must remain visible to every contributor.

---

66. Implementation Status

The repository must never confuse:

specified

with:

implemented

A feature may be fully specified but not yet implemented.

A feature may also exist experimentally in code without being a stable language feature.

Promotion requires conformance.

---

67. Production-Ready Definition

A language feature is production-ready only when:

Specification
        ✓
Lexical contract
        ✓
Syntax contract
        ✓
Parser implementation
        ✓
AST contract
        ✓
Semantic contract
        ✓
IR contract
        ✓
Compiler integration
        ✓
Runtime integration
        ✓
Relevant backend integration
        ✓
Diagnostics
        ✓
Positive tests
        ✓
Negative tests
        ✓
Boundary tests
        ✓
Scalability tests
        ✓
Compatibility tests
        ✓
Determinism review
        ✓
Hard-coding audit
        ✓
Security review
        ✓
Documentation
        ✓

A feature missing a required stage is not complete.

---

68. Repository-Wide Conformance

Production readiness of "grammar/" must ultimately be verified against the whole frontend/backend pipeline.

The conformance chain is:

grammar/specification/
        ↓
grammar/spec/
        ↓
grammar/Zamani.g4
        ↓
lexer
        ↓
parser
        ↓
AST
        ↓
semantic analysis
        ↓
IR
        ↓
domain IR
        ↓
optimization
        ↓
routing
        ↓
scheduling
        ↓
resilience
        ↓
ZQN
        ↓
HAL
        ↓
backend
        ↓
runtime

The grammar cannot be considered production-ready solely because ANTLR accepts it.

---

69. Canonical Semantic Boundary

Zamani should have one canonical semantic model for each genuinely distinct domain.

For quantum computation:

quantum::ir

is the canonical semantic boundary.

Frontend syntax must lower into it.

Other quantum subsystems consume it.

No second quantum IR should be introduced merely because another subsystem needs a representation.

---

70. Universal Computing Model

Zamani is intended to express computation across:

atom
      ↓
nano
      ↓
embedded
      ↓
classical
      ↓
accelerated
      ↓
quantum
      ↓
hybrid
      ↓
distributed
      ↓
HPC
      ↓
cloud
      ↓
edge
      ↓
future computational systems

The language architecture must remain open-ended.

A future computational model should be able to integrate through existing:

types
expressions
effects
capabilities
resources
operations
modules
dialects
IR boundaries

without requiring a redesign of the entire language.

---

71. Program Once

The programmer should express semantic intent once.

For example:

compute
    result
from
    data

should remain independent of whether the eventual realization uses:

CPU
GPU
FPGA
ASIC
QPU
distributed cluster
embedded system
future accelerator

where the computation's semantics permit those alternatives.

The compiler determines the realization.

---

72. Compile Once

Compilation must separate:

portable semantic compilation

from:

target realization

Where target-specific information is unavoidable, it must be represented as target configuration or deployment information rather than silently becoming source-language semantics.

---

73. Run Everywhere / Anywhere

A Zamani program should be transportable across compatible environments.

The environment determines:

capabilities
resources
topology
performance
availability

The program determines:

required semantics
correctness
constraints
acceptable alternatives

The compiler/runtime performs the negotiation.

---

74. Run Forever

"Forever" means the language is designed for long-term semantic stability.

This requires:

- explicit versioning;
- compatibility rules;
- deprecation policies;
- migration rules;
- stable semantic contracts;
- provenance;
- dialect versioning;
- preservation of portable source meaning.

It does not mean that every historical compiler or physical device remains executable forever.

---

75. What This Specification Must Never Become

This directory must never become:

- a vendor API manual;
- a hardware catalogue;
- a list of today's GPUs;
- a list of today's QPUs;
- an implementation dump;
- a backend specification;
- a runtime API reference;
- a second quantum IR;
- a list of every mathematical function;
- a list of every AI framework;
- a fixed topology description;
- an arbitrary collection of keywords.

It defines the language.

---

76. Completion Criteria for "grammar/specification/"

This specification architecture is complete only when:

- authority is unambiguous;
- every normative feature has a lifecycle;
- "Zamani.g4" has a defined relationship to the specification;
- "grammar.md" has a defined conformance role;
- "Zamani-Grammar.md" has a defined non-authoritative/reference role;
- "spec/" has a defined subordinate contract role;
- lexer ownership is defined;
- parser ownership is defined;
- AST ownership is defined;
- semantic ownership is defined;
- canonical IR ownership is defined;
- "quantum::ir" is explicitly protected as the canonical quantum semantic boundary;
- compiler integration is defined;
- runtime integration is defined;
- hardware integration is defined;
- resource semantics are defined;
- capability semantics are defined;
- portability semantics are defined;
- scalability semantics are defined;
- no arbitrary hardware limits are allowed;
- domain integration is defined;
- interoperability is defined;
- dialects are controlled;
- versioning is defined;
- compatibility is defined;
- diagnostics are defined;
- determinism is defined;
- provenance is defined;
- security boundaries are defined;
- safe Rust requirements are explicit;
- "unsafe" Rust is prohibited;
- independent-file completion requirements are explicit;
- feature completion requirements are explicit;
- repository-wide conformance requirements are explicit.

---

77. Final Architectural Contract

The central Zamani contract is:

                         ZAMANI SOURCE
                               │
                               ▼
                     Normative Specification
                               │
                               ▼
                         Zamani Grammar
                               │
                               ▼
                             Lexer
                               │
                               ▼
                            Parser
                               │
                               ▼
                     Domain-Neutral AST
                               │
                               ▼
                  Structural + Semantic Analysis
                               │
          ┌────────────────────┼────────────────────┐
          │                    │                    │
      Types/Effects       Capabilities          Resources
          │                    │                    │
          └────────────────────┼────────────────────┘
                               ▼
                   Canonical Semantic Model
                               │
          ┌────────────────────┼────────────────────┐
          │                    │                    │
     Classical IR          quantum::ir         HDL/Hardware
          │                    │                    │
          └────────────────────┼────────────────────┘
                               ▼
                         Optimization
                               │
                 ┌─────────────┼─────────────┐
                 │             │             │
              Routing      Scheduling     Resilience
                 │             │             │
                 └─────────────┼─────────────┘
                               ▼
                              ZQN
                               │
                              HAL
                               │
                         Target Backend
                               │
          ┌────────────┬───────┼───────┬────────────┐
          │            │       │       │            │
         CPU          GPU     FPGA    QPU       Future Target
          │            │       │       │            │
          └────────────┴───────┼───────┴────────────┘
                               ▼
                       Runtime / Deployment

The fundamental invariant is:

«Zamani source describes portable computational intent and semantics. It does not describe the accidental limitations of today's hardware.»

Therefore:

Program Once
       ↓
Compile Once
       ↓
Target-aware realization
       ↓
Run Everywhere
       ↓
Run Anywhere
       ↓
Run Forever

subject to:

program semantics
+
correctness requirements
+
available resources
+
target capabilities
+
physical constraints
+
explicit user constraints

and never because the grammar arbitrarily imposed a finite machine limit.

The final language must therefore scale from the smallest meaningful computation to arbitrarily large computations supported by the available resources, while remaining one language across classical, quantum, HDL, hybrid, distributed, AI, data, networking, security, accelerator, embedded, scientific, edge, cloud, and future computational domains.

That is the normative architectural contract for "grammar/specification/".