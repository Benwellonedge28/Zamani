Worked for 16s

Below is the production-ready contents for grammar/specification/language-scope.md. It is deliberately normative and establishes the scope boundary that the later grammar files must obey, rather than duplicating their concrete syntax.

Zamani Language Scope

Path: "grammar/specification/language-scope.md"
Status: Normative
Language: Zamani
Specification layer: Language foundation
Implementation baseline: Rust 1.97 / Rust 1.97.1, Rust 2021, stable Rust
Safety requirement: "unsafe" Rust is prohibited
Primary architectural objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the scope of the Zamani programming language.

It establishes:

- what Zamani is intended to express;
- which computational domains are first-class language concerns;
- which concepts belong to syntax;
- which concepts belong to semantic analysis;
- which concepts belong to canonical IR;
- which concepts belong to compilation;
- which concepts belong to execution;
- which concepts belong to targets and hardware;
- how Zamani scales from extremely small systems to arbitrarily large systems subject only to actual resource availability;
- how classical, quantum, hybrid, hardware, distributed, AI, data, networking, security, and future computation coexist without fragmenting the language;
- how the grammar remains portable and extensible;
- how existing repository implementations integrate with this scope.

This document does not define every concrete production rule.

Concrete syntax belongs to the grammar layer and must conform to the scope and architectural boundaries defined here.

---

2. Normative Language Scope

Zamani is a general-purpose universal computational language.

Its scope is not limited to a single computational model, processor architecture, execution environment, or physical implementation technology.

Zamani may express computation involving:

1. classical computation;
2. quantum computation;
3. hybrid quantum-classical computation;
4. hardware description;
5. hardware/software co-design;
6. embedded systems;
7. systems programming;
8. concurrent computation;
9. parallel computation;
10. distributed computation;
11. high-performance computing;
12. numerical computation;
13. symbolic computation;
14. scientific computation;
15. AI and machine learning;
16. data processing;
17. tensor computation;
18. accelerators;
19. networking;
20. cryptography;
21. security-sensitive computation;
22. edge computing;
23. cloud computing;
24. heterogeneous computing;
25. simulation;
26. domain-specific computation;
27. metaprogramming;
28. compile-time computation;
29. future computational paradigms not yet known when the current language version is defined.

The language therefore has one semantic foundation with multiple computational domains, rather than a collection of unrelated sublanguages.

---

3. The Core Scope Principle

The scope of Zamani is defined by computation and semantics, not by the machines currently available.

The fundamental rule is:

«Zamani describes what computation means and what properties it requires; target systems determine how that computation is realized.»

Therefore:

Zamani source
    ↓
portable program meaning
    ↓
semantic analysis
    ↓
canonical IR
    ↓
optimization / transformation
    ↓
resource and capability matching
    ↓
target realization
    ↓
execution

The language must not reverse this relationship.

A CPU, GPU, FPGA, ASIC, QPU, cluster, simulator, embedded processor, or cloud platform must not become the definition of Zamani itself.

---

4. Scope Is Not a Machine Specification

Zamani may describe requirements concerning execution resources, but it must not confuse:

- what a program means
- what a program requires
- what a target provides
- what a deployment permits
- what a scheduler chooses
- what a backend implements

These are separate concepts.

For example:

requires quantum

does not mean:

use quantum processor X

and:

requires parallel execution

does not mean:

use exactly 64 threads

Similarly:

requires hardware acceleration

does not mean:

use GPU device 0

or:

use NVIDIA GPU model X

unless the programmer explicitly makes such a target-specific decision part of the program's intended deployment semantics.

---

5. Universal Scale

5.1 Scale range

Zamani must support programs ranging from:

single expression

through:

single function
single process
single device
single embedded system
single CPU
multicore system
GPU/accelerator
FPGA
ASIC
QPU
heterogeneous machine
distributed cluster
supercomputer
cloud deployment
planetary-scale distributed computation

and future systems beyond these categories.

The language itself must not establish an arbitrary upper bound merely because a current implementation, compiler, simulator, or hardware target has a finite capacity.

---

5.2 Meaning of "infinity"

"Infinity" in the Zamani architecture means:

«The language does not impose an artificial finite machine-size ceiling.»

It does not mean that a physical computer can execute an actually infinite object.

Every concrete execution remains constrained by:

- available memory;
- available compute;
- available storage;
- address-space representation;
- execution time;
- energy;
- communication capacity;
- device capacity;
- target capabilities;
- security policies;
- compiler resources;
- runtime resources;
- operating-system constraints;
- deployment policy;
- numerical representation.

These are execution/resource constraints, not arbitrary language limits.

Therefore:

language capacity ≠ machine capacity

and:

machine capacity ≠ semantic definition of the program

---

6. No Artificial Scalability Ceiling

The grammar and language architecture must not contain arbitrary constants representing machine scale.

The language must not define syntax around assumptions such as:

MAX_QUBITS = 32
MAX_QUBITS = 64
MAX_CORES = 128
MAX_THREADS = 1024
MAX_GPUS = 8
MAX_NODES = 1024
MAX_MEMORY = ...
MAX_REGISTER_COUNT = ...
MAX_VECTOR_WIDTH = ...
MAX_TENSOR_RANK = ...

The same prohibition applies to hidden equivalents.

A parser must not reject a valid program merely because the program describes more resources than a particular implementation currently handles.

A compiler may legitimately reject an invocation because the requested target or compilation policy cannot satisfy the program.

Those are different conditions.

---

7. Resource-Bounded Execution

The language scope is unbounded by arbitrary language-level machine constants, but every execution is resource bounded.

The architecture therefore distinguishes:

Language semantics

What the program means.

Program requirements

What the program needs to execute correctly.

Target capabilities

What a target can provide.

Resource availability

What resources are currently available.

Deployment policy

What resources the execution is permitted to consume.

Scheduling

How available resources are assigned.

Backend realization

How semantic operations become target operations.

The general model is:

Program semantics
       +
Program requirements
       +
Target capabilities
       +
Available resources
       +
Deployment policy
       ↓
Feasible realization

Failure to find a feasible realization must not change the meaning of the source program.

---

8. POCO-REAF Scope

Zamani is designed around:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever»

abbreviated:

«POCO-REAF»

POCO-REAF is a language and compiler architecture principle, not a claim that one architecture-specific machine binary can literally execute unchanged on every conceivable processor.

---

8.1 Program Once

A programmer should express the computation once at the semantic level.

Changing:

- CPU count;
- GPU count;
- FPGA size;
- quantum device;
- cluster size;
- memory capacity;
- topology;
- deployment location;
- accelerator availability;
- hardware generation;

must not inherently require rewriting the program's semantic algorithm.

---

8.2 Compile Once

Compilation should preserve a stable, target-independent semantic representation wherever the architecture permits.

The portable compilation boundary should preserve:

- computation;
- data relationships;
- control flow;
- resource requirements;
- capabilities;
- effects;
- domain semantics;
- correctness properties;
- relevant metadata.

Target-specific realization may occur later.

A target-specific code-generation step is therefore not a violation of POCO-REAF when it operates on the same portable semantic program.

---

8.3 Run Everywhere

The same semantic program may be lowered to multiple compatible targets.

Examples include:

CPU
GPU
FPGA
ASIC
QPU
simulator
embedded processor
cluster
cloud
heterogeneous system

The source semantics remain stable.

---

8.4 Run Anywhere

Execution may occur:

- locally;
- remotely;
- on embedded hardware;
- on a workstation;
- on a server;
- on a cluster;
- in a cloud;
- on quantum hardware;
- on a simulator;
- across heterogeneous resources.

Location is therefore a deployment property unless location itself is part of the program's semantics.

---

8.5 Run Forever

Long-term portability requires:

- versioned semantics;
- stable semantic identifiers;
- explicit compatibility policy;
- extensible dialects;
- reserved language space;
- target-independent representations;
- migration mechanisms;
- deterministic serialization where required;
- canonical IR boundaries;
- preservation of program meaning across target evolution.

"Forever" therefore means that Zamani must be designed so that future hardware and execution technologies can be added without invalidating the fundamental language model.

It does not mean that every historical binary or compiler implementation is guaranteed to remain executable forever.

---

9. Computational Domains

Zamani's computational domains are extensions of one language semantic model.

They are not independent languages.

---

9.1 Classical Computing

The classical scope includes:

- scalar values;
- structured values;
- arrays;
- collections;
- functions;
- generics;
- control flow;
- pattern matching;
- memory;
- ownership-related semantics;
- concurrency;
- parallelism;
- numerical computation;
- symbolic computation;
- vectors;
- matrices;
- tensors;
- systems programming;
- accelerator-oriented computation.

The classical layer must remain general enough to serve as the control and data-processing foundation for other domains.

---

10. Quantum Computing

Quantum computation is a first-class Zamani domain.

The language must be able to express quantum computation without assuming a particular QPU architecture.

The scope includes:

- logical qubits;
- physical qubits where explicitly required;
- quantum registers;
- quantum states;
- operations;
- gates;
- parameterized operations;
- controlled operations;
- arbitrary/extensible operations;
- measurement;
- observables;
- reset;
- dynamic circuits;
- mid-circuit measurement;
- classical feedback;
- quantum-classical interaction;
- quantum resources;
- quantum capabilities;
- logical computation;
- physical realization constraints;
- error-correction-related semantics;
- pulse-level semantic descriptions where supported;
- multiple quantum computational models.

---

10.1 No Fixed Qubit Limit

Zamani syntax must not encode an arbitrary maximum number of qubits.

Invalid architectural examples include:

qubit q[32];

when "32" is being used as an implementation-imposed maximum.

A valid program may express a dynamically or parametrically sized quantum resource:

qubit q[n];

where "n" is determined by program semantics, configuration, compilation, or resource negotiation as appropriate.

The actual target determines whether the requested execution is feasible.

---

10.2 Logical and Physical Qubits

Logical qubits and physical qubits have different meanings.

Logical qubit

Represents computational semantics independent of a particular physical device.

Physical qubit

Represents an explicit physical binding or target-level requirement.

A source program should default toward logical/abstract semantics unless physical binding is intentionally part of the program or deployment description.

The grammar must not silently convert logical qubits into vendor-specific physical identities.

---

11. Quantum IR Integration

The canonical quantum semantic boundary is:

src/quantum/ir/

specifically the "quantum::ir" architecture.

The grammar must not define a second quantum IR.

The dependency direction is:

Zamani source
    ↓
lexer
    ↓
parser
    ↓
AST
    ↓
semantic analysis
    ↓
quantum semantic lowering
    ↓
quantum::ir
    ↓
optimization
    ↓
routing / mapping
    ↓
scheduling
    ↓
hardware / target lowering
    ↓
execution

The "quantum::ir" implementation explicitly owns hardware-independent quantum semantic representation and does not own source parsing, target selection, physical routing, scheduling, calibration, or backend execution.

Therefore:

- "grammar/quantum/" owns source syntax;
- semantic analysis interprets it;
- "quantum::ir" owns canonical quantum meaning;
- QEC consumes the appropriate canonical semantic representation;
- ZQN consumes the appropriate canonical representation for noise-aware execution;
- optimization consumes IR;
- scheduling consumes IR;
- hardware lowering consumes IR;
- backends consume target-lowered representations.

No grammar file may create a competing canonical quantum representation.

---

12. Quantum Computational Models

The scope must not assume that all quantum computation is a gate list.

Zamani must remain extensible toward:

- circuit computation;
- dynamic circuits;
- measurement-based computation;
- analog computation;
- Hamiltonian computation;
- annealing;
- QUBO-style computation;
- continuous-variable models;
- bosonic models;
- fermionic models;
- tensor-network-oriented computation;
- logical quantum computation;
- fault-tolerant computation;
- distributed quantum computation;
- pulse-level semantic descriptions.

The existing "quantum::ir" architecture already separates universal program representation, computational models, quantum semantics, pulse semantics, resources, scheduling, metadata, validation, hashing, and dialects. The grammar must preserve those ownership boundaries rather than collapse them into one grammar-level circuit abstraction.

---

13. Hybrid Computing

Hybrid quantum-classical computation is a first-class scope.

Zamani must allow:

classical computation
        ↓
quantum computation
        ↓
measurement
        ↓
classical processing
        ↓
quantum feedback
        ↓
continued execution

within one semantic program.

The boundary between quantum and classical computation must be explicit.

The language must not require separate source programs merely because execution crosses the quantum/classical boundary.

---

14. Hardware Description and Hardware/Software Co-Design

Hardware description is part of the language scope.

Zamani may describe:

- hardware modules;
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
- interfaces;
- hardware parameters;
- hardware generics;
- hardware capabilities;
- hardware requirements;
- target constraints.

However, hardware semantics must remain distinct from physical implementation details.

---

14.1 Hardware Semantics

Hardware semantics describe what the hardware design does.

Examples:

a combinational transformation
a sequential state transition
a communication interface
a timing relationship
a memory behavior
a pipeline dependency

---

14.2 Hardware Realization

Hardware realization determines:

- technology;
- fabrication target;
- FPGA family;
- ASIC process;
- device package;
- physical placement;
- routing;
- implementation resources;
- clocking resources;
- physical pin assignments.

These must not become implicit language limits.

---

15. Embedded Computing

Zamani must be capable of targeting constrained systems including:

- microcontrollers;
- small CPUs;
- small memory systems;
- real-time systems;
- sensor systems;
- edge devices;
- hardware controllers.

A language feature must not require a large operating system, cloud service, or high-memory runtime unless that requirement is explicitly part of the feature's semantics.

Portable source should allow target realization appropriate to the available environment.

---

16. Systems Programming

The scope includes systems-level computation.

This includes:

- deterministic resource management;
- memory-aware computation;
- low-level data representation;
- concurrency;
- synchronization;
- device interaction;
- operating-system interfaces;
- embedded interfaces;
- hardware interfaces;
- foreign-function interfaces;
- ABI interaction.

The language must distinguish semantic low-level control from implementation-specific machine details.

---

17. Concurrency and Parallelism

Zamani includes:

- asynchronous execution;
- tasks;
- futures;
- actors;
- channels;
- synchronization;
- data parallelism;
- task parallelism;
- distributed parallelism;
- heterogeneous parallelism.

The grammar must not define a fixed number of:

- threads;
- workers;
- tasks;
- actors;
- execution units.

A program may express parallelism abstractly.

The runtime and scheduler determine feasible realization.

---

18. Distributed Computing

Distributed computation is within language scope.

Zamani may express:

- nodes;
- services;
- communication;
- messaging;
- remote execution;
- replication;
- consistency;
- fault tolerance;
- placement;
- distributed resources.

The language must not require a fixed cluster size.

For example, a semantic request such as:

parallel over dataset

must not inherently mean:

run on exactly 128 nodes

unless that exact number is explicitly part of the program's requirements.

---

19. AI and Machine Learning

AI/ML computation is within scope.

The language may express:

- models;
- tensors;
- datasets;
- training;
- inference;
- automatic differentiation;
- pipelines;
- agents;
- model execution;
- accelerator requirements.

AI constructs must integrate with the general type, effect, resource, memory, concurrency, data, and compilation models.

They must not establish an isolated AI programming language inside Zamani.

---

20. Data Computing

Zamani may express:

- schemas;
- records;
- collections;
- streams;
- serialization;
- transformations;
- structured and unstructured data;
- large-scale data processing.

Data size must not be limited by arbitrary grammar constants.

A grammar rule must not assume:

maximum_records = N
maximum_columns = N
maximum_tensor_elements = N

unless the bound is intrinsic to a specific semantic type.

---

21. Networking

Networking is within scope.

Zamani may express:

- endpoints;
- protocols;
- messages;
- channels;
- services;
- network capabilities;
- communication requirements.

Specific addresses, ports, protocols, or network topology should be represented only where they are semantically necessary or explicitly supplied by deployment configuration.

The grammar must not assume a fixed network architecture.

---

22. Cryptography and Security

Security is a language-wide concern.

The scope includes:

- identities;
- permissions;
- capabilities;
- cryptographic operations;
- privacy requirements;
- trust;
- security constraints;
- secure communication;
- isolation requirements.

Security semantics must not be implemented merely as comments or backend conventions.

Where a property affects program correctness or authorization, it must have an explicit semantic representation.

---

23. Resource Model

Resource semantics are central to the language scope.

Zamani must distinguish at least:

resource
requirement
constraint
capability
preference
hint
target
placement
performance
latency
energy
reliability
scalability
portability

These terms must not become synonyms.

---

23.1 Resource

A resource is something that can be consumed, reserved, assigned, shared, or otherwise used.

Examples include:

- memory;
- processors;
- accelerators;
- qubits;
- storage;
- bandwidth;
- execution units.

---

23.2 Requirement

A requirement describes something necessary for correct execution.

Example:

requires quantum capability

does not identify a particular machine.

---

23.3 Constraint

A constraint restricts permissible implementations.

Example:

maximum latency below a declared threshold

is different from specifying a particular processor.

---

23.4 Capability

A capability describes what a target or environment can provide.

Examples:

quantum
gpu
fpga
distributed
accelerated
real_time

Capabilities are discovered, declared, or negotiated.

---

23.5 Preference

A preference influences realization without necessarily being required for correctness.

---

23.6 Hint

A hint provides optimization information without becoming a semantic requirement.

---

23.7 Target

A target identifies a realization domain.

A target may identify an architecture, execution technology, or deployment class.

It must not silently change program semantics.

---

24. Capability-Based Portability

Portability is based on capabilities rather than machine identity.

Prefer:

requires capability quantum

over:

requires device "Vendor-QPU-17"

Prefer:

requires accelerator

over:

requires GPU 0

Prefer:

requires parallel execution

over:

requires exactly 64 cores

when the latter is not actually part of the program's semantics.

Explicit machine selection remains possible where required, but must be treated as an explicit target/deployment constraint rather than an implicit language assumption.

---

25. Hardware Independence

Zamani source semantics must remain independent of:

- CPU instruction sets;
- GPU vendor;
- FPGA vendor;
- ASIC technology;
- quantum vendor;
- physical topology;
- register count;
- cache structure;
- memory hierarchy;
- vector width;
- physical qubit numbering;
- device addresses.

Such information may exist in:

target descriptions
hardware descriptions
resource models
deployment configurations
compiler contexts
runtime discovery
backend metadata

but must not become an accidental universal language limit.

---

26. Grammar Scope

The grammar is responsible for recognizing valid Zamani source structure.

It may define syntax for:

- declarations;
- expressions;
- statements;
- types;
- modules;
- functions;
- effects;
- resources;
- quantum constructs;
- hardware constructs;
- concurrency;
- distributed constructs;
- AI/data constructs;
- networking;
- security;
- compilation directives;
- dialects;
- metaprogramming.

It must not encode backend implementation algorithms.

The grammar must not directly implement:

- routing;
- scheduling algorithms;
- QEC decoding;
- hardware calibration;
- GPU kernel selection;
- device discovery;
- runtime networking;
- backend authentication;
- physical placement algorithms.

Those belong to downstream systems.

---

27. Existing Frontend Integration

The current repository frontend already establishes an important architecture:

src/lexer.rs
    ↓
src/parser.rs
    ↓
src/ast/
    ↓
semantic/compiler layers

The repository's implementation-oriented "grammar/grammar.md" explicitly describes itself as being derived from "src/lexer.rs", "src/parser.rs", and "src/ast/mod.rs", and therefore reflects the currently implemented frontend rather than merely aspirational syntax.

The current lexer owns lexical recognition and source spans, while the parser consumes lexer tokens and constructs AST nodes.

The scope of this document therefore establishes the architectural direction that those implementation files must conform to; it does not duplicate their token or parser definitions.

---

28. AST Scope

"src/ast/" represents source-level structure.

The AST may represent:

- declarations;
- expressions;
- statements;
- patterns;
- types;
- quantum operations;
- effects;
- domain constructs;
- metadata.

Every source construct must have a clearly defined semantic destination.

The AST must not become a permanent backend IR.

The current AST already includes general program, statement, expression, type, pattern, quantum, effect, concurrency, and advanced system constructs.

Future additions must preserve the distinction:

AST = source structure
IR  = canonical semantic representation

---

29. Canonical Semantic Boundaries

Zamani must use canonical semantic representations rather than multiple competing representations of the same concept.

For quantum computation:

quantum::ir

is canonical.

For other domains, the same ownership principle applies:

«One semantic concept must have one authoritative canonical representation at its semantic boundary.»

Compatibility aliases may exist.

Duplicate semantic types must not.

---

30. HDL Integration Boundary

HDL syntax must lower through the normal language architecture.

HDL source
    ↓
grammar
    ↓
AST
    ↓
semantic analysis
    ↓
hardware semantic representation
    ↓
hardware compilation
    ↓
target realization

The grammar must not directly encode:

FPGA family X
ASIC process Y
physical pin Z
routing algorithm A

unless explicitly represented as target/deployment data.

---

31. Hybrid Domain Integration

Cross-domain programs are expected.

Examples include:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

The grammar must permit domains to compose through shared language primitives rather than requiring incompatible syntax universes.

---

32. Domain Extension Rule

A new computational domain must integrate with existing language concepts.

A domain extension should identify:

1. syntax;
2. AST representation;
3. type semantics;
4. effect semantics;
5. resource semantics;
6. capability semantics;
7. canonical IR;
8. compilation boundary;
9. execution boundary;
10. interoperability boundary;
11. validation;
12. testing;
13. compatibility policy.

A new domain must not introduce a second:

- type system;
- resource system;
- capability system;
- module system;
- effect system;
- compilation model;
- canonical IR architecture.

unless there is a demonstrated semantic reason and an explicit integration contract.

---

33. Future Computing

The language must reserve semantic space for future computation.

Future technologies must be integrable through:

- extensible operations;
- dialects;
- capabilities;
- resource requirements;
- target descriptions;
- semantic models;
- canonical IR extensions;
- versioning;
- compatibility rules.

The grammar must not assume that today's categories exhaust computation.

---

34. Dialects

Dialects provide controlled extension without changing the universal language core for every new technology.

A dialect may introduce:

- domain-specific operations;
- types;
- attributes;
- metadata;
- target capabilities;
- semantic models.

A dialect must identify:

- namespace;
- version;
- owner;
- capabilities;
- compatibility;
- semantic lowering;
- canonical representation.

Dialect syntax must not silently redefine core Zamani semantics.

---

35. Interoperability Scope

Zamani must be able to interoperate with external computational ecosystems.

The scope includes integration with systems such as:

- C;
- C++;
- Python;
- OpenQASM;
- Verilog and related HDL ecosystems;
- system interfaces;
- external ABIs;
- foreign functions.

Interoperability must remain an explicit boundary.

External syntax must not silently become Zamani's canonical syntax.

---

36. Compile-Time and Runtime Scope

Zamani distinguishes:

compile-time

from:

runtime

from:

deployment

from:

target realization

Compile-time constructs may determine properties that are statically knowable.

Runtime constructs may depend on information unavailable during compilation.

Deployment configuration may select resources.

Target lowering may specialize implementation.

These stages must not be collapsed into one grammar-level concept.

---

37. Compile-Time Specialization

Compile-time specialization is permitted where it preserves semantic portability.

For example:

generic algorithm
    ↓
compile-time specialization
    ↓
target-specific realization

is valid.

However:

source semantics
    ↓
hard-coded current machine limitation

is not a valid universal-language design.

---

38. Runtime Discovery

Runtime discovery is permitted for properties that genuinely cannot be known statically.

Examples include:

- available devices;
- available memory;
- current topology;
- available accelerators;
- available quantum hardware;
- current node availability;
- runtime capabilities.

Runtime discovery must produce typed semantic information rather than arbitrary hidden compiler behavior.

---

39. Scheduling Scope

Scheduling is downstream of semantic representation.

The grammar may express:

- scheduling requirements;
- ordering constraints;
- timing semantics;
- synchronization requirements;
- latency constraints;
- execution preferences.

It must not embed a particular scheduling algorithm.

For quantum computation, scheduling consumes canonical semantic information rather than redefining quantum operations.

---

40. Optimization Scope

Optimization is not grammar.

The grammar must preserve enough semantic information for optimization while remaining independent of optimization implementation.

Optimization may choose:

- equivalent transformations;
- target-specific implementations;
- resource reductions;
- parallelization;
- circuit transformations;
- accelerator mappings.

Such decisions must preserve program semantics.

---

41. Safety Scope

Zamani's language scope includes safety and correctness.

The implementation baseline requires:

Rust 1.97 / Rust 1.97.1
Rust 2021
stable Rust
no nightly dependency
no unsafe Rust

Compiler infrastructure implementing this grammar must use safe Rust.

The language may contain a source-level "unsafe" construct only if the language specification deliberately defines its semantics and safety boundary.

That source-level feature must not be interpreted as permission to use Rust "unsafe".

Therefore:

Zamani unsafe

and:

Rust unsafe

are separate concepts.

The compiler implementation remains subject to the repository's no-"unsafe" Rust requirement.

---

42. Security Boundary

Grammar parsing must not require:

- network access;
- filesystem traversal;
- credentials;
- external device access;
- execution of arbitrary user code.

Parsing source must remain deterministic and isolated from target execution.

Compile-time execution, macros, and metaprogramming must have explicit security boundaries.

---

43. Determinism

For a fixed:

source
+
language version
+
lexical configuration
+
grammar version

lexing and parsing must be deterministic.

The same source must not produce different AST structures because of:

- machine topology;
- available GPU;
- available QPU;
- current network;
- backend choice.

Target-dependent decisions occur after semantic parsing.

---

44. Source Fidelity

The language infrastructure must preserve source locations.

The current lexer attaches source "Span" information to tokens, and the AST similarly carries spans for source-level structures.

The scope therefore requires:

- precise diagnostics;
- stable source locations;
- useful error spans;
- preservation of source-origin information through semantic lowering where practical.

---

45. Diagnostics

Diagnostics are part of the language implementation contract.

The frontend must be capable of distinguishing:

lexical error
syntax error
name-resolution error
type error
effect error
capability error
resource requirement error
semantic error
target incompatibility
runtime/deployment failure

These errors must not be conflated.

For example:

invalid syntax

is different from:

valid program but target lacks required capability

---

46. Compatibility Scope

Language evolution must preserve existing valid programs where the compatibility policy permits it.

Before changing a construct:

1. identify existing syntax;
2. identify its parser;
3. identify its AST representation;
4. identify semantic consumers;
5. identify tests;
6. identify documentation;
7. determine whether the construct is valid;
8. preserve, migrate, deprecate, or remove it explicitly.

No valid existing feature should disappear merely because a new subsystem is introduced.

---

47. Grammar Authority Integration

The grammar scope is subordinate to the repository's formal grammar-authority policy.

The current repository contains multiple grammar representations, including:

grammar/Zamani.g4
grammar/Zamani-Grammar.md
grammar/grammar.md

They must not become independent competing languages.

The intended relationship is:

normative language specification
        ↓
canonical syntax definition
        ↓
implementation grammar
        ↓
src/lexer.rs / src/parser.rs
        ↓
AST

and, where applicable:

canonical syntax
        ↓
ANTLR representation

"grammar/Zamani.g4" must therefore remain synchronized with the canonical syntax rather than independently inventing language semantics.

"grammar/Zamani-Grammar.md" must describe the language consistently rather than silently expanding beyond the implementation contract.

"grammar/grammar.md" remains an implementation-oriented reference until the repository's explicit grammar-authority policy establishes a different canonical relationship.

The exact authority ordering belongs to:

grammar/specification/grammar-authority.md

This file establishes the scope principle; it does not duplicate the authority policy.

---

48. Existing Repository Feature Expansion

The scope requires expansion of existing functionality rather than parallel replacement systems.

Relevant repository areas include:

src/lexer.rs
src/parser.rs
src/ast/
src/quantum/
src/quantum/ir/

and downstream compiler, optimization, scheduling, hardware, QEC, ZQN, resource, and runtime systems.

The language grammar must integrate with those existing systems.

It must not create disconnected replacements such as:

grammar quantum IR

alongside:

src/quantum/ir

or:

grammar hardware resource model

alongside a separate canonical repository resource model.

---

49. Ownership Principle

Every concept must have one clear owner.

At minimum:

Concept| Primary owner
lexical recognition| lexer
concrete syntax| parser/grammar
source structure| AST
names/types/effects/capabilities| semantic analysis
canonical quantum semantics| "quantum::ir"
quantum optimization| quantum optimization subsystem
quantum scheduling| scheduling subsystem
quantum hardware mapping| hardware/routing subsystem
calibration| calibration subsystem
noise-aware execution| ZQN
error correction| QEC subsystem
target selection| compilation/target subsystem
runtime discovery| runtime
deployment| execution/deployment
interoperability| interoperability subsystem

Compatibility layers may expose aliases but must not create competing owners.

---

50. Non-Ownership of Grammar

The grammar does not own:

- target selection algorithms;
- hardware discovery;
- device scheduling;
- physical placement;
- QEC decoding;
- calibration;
- optimization algorithms;
- backend execution;
- network communication;
- runtime state;
- credentials;
- physical resource allocation.

The grammar owns only the source representation of concepts that require language-level expression.

---

51. Resource Scalability Contract

Every grammar feature that expresses a resource must support resource abstraction.

A construct must not silently assume:

one device
one CPU
one GPU
one QPU
one FPGA
one node

unless that is genuinely part of its semantics.

Similarly, collection-like language constructs must not encode fixed cardinality.

Examples:

qubits
nodes
workers
devices
ports
channels
memory regions
tensor dimensions
data records

must remain scalable.

---

52. Parametric Computation

Where a computation naturally scales with a parameter, the language should express the parameter rather than a hard-coded implementation maximum.

Conceptually:

compute(size)

rather than:

compute_64()

when "64" is merely a target limitation.

This principle applies across:

- quantum;
- classical;
- hardware;
- distributed;
- AI;
- data;
- networking.

---

53. Static Versus Dynamic Resource Information

The language may express resource information at multiple stages.

Static

Known at source or compilation time.

Parametric

Determined from generic parameters or compilation configuration.

Target-derived

Determined from target capabilities.

Runtime-derived

Discovered during execution.

Deployment-defined

Provided externally by deployment policy.

These must remain distinguishable.

A runtime-derived resource must not be disguised as a compile-time language constant.

---

54. Hardware-Specific Escape Hatch

POCO-REAF does not prohibit explicit hardware programming.

Zamani must support deliberate target-specific programming when the programmer actually requires it.

For example:

target-specific instruction
physical pin
specific accelerator
physical qubit
specific device capability

may be expressible through explicit target or dialect mechanisms.

However:

«An explicit hardware dependency must be visible as an explicit dependency.»

It must not become an accidental requirement of otherwise portable syntax.

---

55. Portability Classification

Every target-sensitive construct should conceptually fall into one of four categories:

A. Fully portable

The construct has no target dependency.

B. Capability-dependent

The construct requires a capability but not a particular target.

C. Constraint-dependent

The construct requires measurable target properties.

D. Target-specific

The construct explicitly selects or depends on a particular target.

This classification must remain available to semantic analysis and compilation.

---

56. No Hidden Target Capture

A program must not silently become target-specific merely because it was compiled on a particular machine.

For example:

compile on GPU machine

must not silently imply:

program permanently requires that GPU

unless the source or explicit compilation/deployment policy says so.

---

57. Numerical and Tensor Scalability

Numerical and tensor constructs must not assume fixed dimensions merely because current hardware has fixed vector or matrix units.

The language should distinguish:

mathematical shape

from:

physical execution width

A compiler may tile, vectorize, shard, or distribute a tensor without requiring source changes.

---

58. Memory Scalability

The language must support memory abstractions that scale with available resources.

The grammar must not define arbitrary maximum:

memory blocks
heap size
stack size
address count
allocation count

Actual limits belong to the target and runtime.

---

59. Execution Environment Independence

Zamani source must not intrinsically assume:

- operating system;
- filesystem;
- network;
- shell;
- cloud provider;
- hypervisor;
- container;
- accelerator runtime.

When such environments are required, the requirement must be explicit.

This enables the same semantic program to target:

bare metal
embedded
OS process
VM
container
cluster
cloud
specialized runtime

where compatible.

---

60. Inter-Domain Resource Unification

Quantum resources, CPU resources, GPU resources, FPGA resources, memory, network bandwidth, and storage must ultimately participate in a coherent resource model.

The language must not create unrelated concepts such as:

quantum-only resource syntax
GPU-only resource syntax
distributed-only resource syntax

with incompatible semantics.

Domain-specific syntax may exist, but resource meaning must remain composable.

---

61. Effects and Capabilities

Effects describe observable computational behavior or required execution privileges.

Examples include:

IO
HARDWARE
QUANTUM
NETWORK
DISTRIBUTED
SECURITY

Effects and capabilities are related but not identical.

An effect may describe what an operation does.

A capability may describe what an environment permits.

The grammar may represent both, but semantic analysis must keep them distinct.

---

62. Memory and Ownership Scope

The language scope includes safe memory semantics appropriate to:

- classical systems;
- embedded systems;
- concurrent systems;
- accelerators;
- distributed memory;
- heterogeneous memory.

The implementation must remain compatible with safe Rust requirements.

No grammar feature may require the compiler implementation to use Rust "unsafe".

---

63. Metaprogramming Scope

Metaprogramming may generate:

- source structures;
- declarations;
- types;
- domain operations;
- specialized implementations.

However, generated code must remain subject to the same semantic rules as ordinary source.

Metaprogramming must not become a mechanism for bypassing:

- type checking;
- capability checking;
- resource validation;
- security;
- compatibility.

---

64. Macro Scope

Macros are language extensions, not unrestricted compiler modification.

Macros must have explicit:

- input model;
- expansion model;
- hygiene;
- diagnostics;
- evaluation boundary;
- security boundary;
- compatibility behavior.

Macro expansion must produce valid Zamani semantic structures.

---

65. Compile-Time Execution Scope

Compile-time execution is permitted only within the explicitly defined compile-time model.

It must not make parsing dependent on uncontrolled:

- network state;
- filesystem state;
- hardware discovery;
- credentials;
- external services.

If external information is needed, it must be represented through explicit compilation inputs or configuration.

---

66. Version Scope

Language scope evolves through explicit versions.

A version may:

- add syntax;
- add semantics;
- add dialects;
- add capabilities;
- deprecate constructs;
- introduce compatibility mappings.

Versioning must not silently change the meaning of existing source.

The authoritative versioning rules belong to:

grammar/specification/language-version.md
grammar/specification/compatibility.md

---

67. Reserved Scope

The language must reserve namespace and semantic space for future extensions.

Reserved space must not be treated as implemented functionality.

A reserved keyword or namespace must have a documented reason.

The language must avoid permanently consuming generic identifiers for speculative concepts that have no defined semantics.

---

68. Error and Failure Scope

A portable program may fail to execute on a target because the target cannot satisfy its requirements.

This is not necessarily a language error.

For example:

valid Zamani program
        ↓
requires quantum capability
        ↓
CPU-only target
        ↓
no feasible realization

The correct outcome is a capability/target feasibility failure, not reinterpretation of the program.

---

69. Feasibility Is Not Semantics

The compiler must not rewrite program meaning merely to make an unsupported target accept the program.

For example, if a program requires genuine quantum semantics, silently replacing the computation with an unrelated classical approximation is not a valid realization unless the program explicitly permits such substitution.

Similarly:

requires exact arithmetic

must not silently become:

floating-point approximation

without an explicit semantic contract permitting it.

---

70. Determinism and Reproducibility

Where the language semantics require deterministic behavior, compilation and execution infrastructure must preserve it.

Target-dependent optimization may vary implementation while preserving defined observable semantics.

Canonical representations should support reproducibility where required.

This is particularly important for:

- quantum programs;
- numerical computation;
- distributed systems;
- cryptography;
- scientific computation;
- testing;
- content-addressed compilation.

---

71. Testing Scope

Language-scope conformance must include:

Positive tests

Every supported scope category must have valid examples.

Negative tests

Unsupported combinations must fail clearly.

Boundary tests

Very small and very large source structures must be accepted until actual implementation/resource limits are reached.

Cross-domain tests

At minimum:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

Scalability tests

Tests must verify that no grammar rule introduces accidental fixed limits.

Compatibility tests

Existing valid Zamani syntax must remain valid unless explicitly changed by language-version policy.

---

72. Hard-Coding Audit

Every scope-related implementation must be audited for:

- fixed qubit counts;
- fixed register counts;
- fixed core counts;
- fixed thread counts;
- fixed GPU counts;
- fixed FPGA counts;
- fixed node counts;
- fixed memory sizes;
- fixed vector widths;
- fixed tensor dimensions;
- fixed device IDs;
- fixed hardware addresses;
- fixed topology;
- fixed network sizes;
- fixed accelerator counts.

Each discovered limit must be classified as:

1. semantic requirement;
2. explicit target requirement;
3. resource policy;
4. implementation limit;
5. accidental hard-coding;
6. test limitation;
7. documentation limitation.

Only accidental hard-coding must be removed automatically; genuine resource and policy limits must remain explicit and correctly owned.

---

73. Repository Integration Contract

This file establishes the following integration contracts.

"grammar/specification/README.md"

Provides the specification index and document relationships.

This file supplies the detailed scope.

---

"grammar/specification/language-principles.md"

Defines the principles that govern this scope.

This file must conform to those principles.

---

"grammar/specification/language-version.md"

Owns version identity and evolution policy.

This file must not independently define version numbering.

---

"grammar/specification/grammar-authority.md"

Owns the authoritative relationship between:

Zamani.g4
Zamani-Grammar.md
grammar.md
src/lexer.rs
src/parser.rs
src/ast/

This file establishes scope but does not supersede that authority document.

---

"grammar/specification/syntax-model.md"

Owns the formal syntax model.

This file defines what domains are in scope, while "syntax-model.md" defines how source syntax represents them.

---

"grammar/specification/semantic-model.md"

Owns detailed semantic interpretation.

This file establishes the domain boundaries that semantic modeling must implement.

---

"grammar/specification/compilation-model.md"

Owns the formal compilation pipeline.

This file establishes why the language must remain target-independent.

---

"grammar/specification/execution-model.md"

Owns execution semantics.

This file establishes that resource availability and target realization occur after language semantics.

---

"grammar/specification/scalability-model.md"

Owns detailed scalability rules.

This file establishes the scope requirement that there be no artificial language-imposed machine ceiling.

---

"grammar/specification/poco-reaf.md"

Owns the detailed POCO-REAF model.

This file establishes the scope-level requirement for portable semantic programs.

---

74. Integration With Lexer

The lexer owns:

- character processing;
- tokenization;
- lexical errors;
- source spans;
- keyword recognition.

The scope document does not define individual token names.

The lexer must remain independent of:

- runtime hardware;
- target selection;
- quantum device discovery;
- backend execution.

The current implementation already represents lexical tokens and source spans through "TokenType", "Token", and lexer errors.

---

75. Integration With Parser

The parser owns concrete syntactic structure.

The scope document requires the parser to support the domains described here but does not dictate implementation algorithms.

The current parser uses recursive descent and Pratt-style expression precedence and already dispatches quantum, noise, surface-code, nano, effect, and advanced system constructs.

Future grammar expansion must preserve parser determinism and clear semantic boundaries.

---

76. Integration With AST

The AST must be sufficient to preserve source meaning without becoming target-specific.

Every grammar construct must map to:

AST node

and subsequently:

semantic representation

or an explicitly documented compile-time-only construct.

No grammar construct may be added without defining its AST ownership.

---

77. Integration With Quantum Systems

Quantum source constructs flow into:

semantic analysis
        ↓
quantum::ir

The downstream quantum stack then determines:

- optimization;
- routing;
- scheduling;
- hardware mapping;
- calibration;
- execution;
- simulation;
- QEC;
- noise-aware behavior.

The canonical quantum IR explicitly excludes source parsing, physical machine choice, routing, scheduling, calibration, backend execution, simulation, and QEC decoding from its own ownership.

The grammar must respect that boundary.

---

78. Integration With QEC

Quantum error correction is downstream of language parsing and semantic lowering.

The grammar may express QEC-related semantic intent where required.

It must not implement:

- syndrome decoding;
- decoder algorithms;
- physical error models;
- hardware calibration.

Those remain owned by QEC/ZQN/hardware subsystems.

---

79. Integration With ZQN

ZQN owns quantum noise-related semantics and execution concerns.

The grammar may express source-level noise models or noise-related intent.

It must not create an alternative noise IR that competes with the repository's existing quantum semantic architecture.

---

80. Integration With Optimization

Optimization consumes canonical semantic representations.

The grammar must preserve information necessary for optimization but must not encode optimization decisions as syntax unless those decisions are explicitly semantic.

---

81. Integration With Scheduling

Scheduling consumes semantic operations and resource/timing information.

No grammar rule may establish a universal:

MAX_QUBITS
MAX_THREADS
MAX_DEVICES

Scheduling limit.

Resource constraints belong to scheduling/resource policy.

---

82. Integration With Hardware

Hardware subsystems consume hardware requirements, capabilities, semantic hardware descriptions, and target information.

The grammar provides source representation.

Hardware subsystems provide realization.

The dependency must remain:

grammar → semantic representation → hardware subsystem

not:

grammar ↔ hardware subsystem

---

83. Integration With Runtime

Runtime systems may discover:

- resources;
- capabilities;
- devices;
- topology;
- availability.

The grammar must not depend on runtime state for parsing.

Runtime information may influence execution but must not change the meaning of already parsed source.

---

84. Integration With Tooling

Tooling must be able to use the grammar for:

- parsing;
- syntax highlighting;
- formatting;
- linting;
- documentation;
- IDE integration;
- code navigation;
- refactoring;
- diagnostics;
- source transformation.

Tooling must consume the same language definitions rather than creating private incompatible grammars.

---

85. Integration With Documentation

Documentation must distinguish:

implemented
planned
reserved
deprecated
experimental
target-specific

A conceptual future construct must not be presented as an implemented language feature.

---

86. Integration With Examples

Examples under:

grammar/examples/

must be executable or explicitly classified as illustrative.

Examples must demonstrate:

- tiny programs;
- scalable programs;
- classical computation;
- quantum computation;
- hybrid computation;
- HDL;
- hardware;
- distributed computation;
- AI;
- data;
- cross-domain programs;
- POCO-REAF.

Examples must not establish semantics merely by existing.

---

87. Dependency Direction

The required architecture is:

language scope
       ↓
language principles
       ↓
syntax specification
       ↓
grammar
       ↓
lexer/parser
       ↓
AST
       ↓
semantic analysis
       ↓
canonical IR
       ↓
optimization
       ↓
scheduling
       ↓
hardware/target lowering
       ↓
runtime
       ↓
execution

There must be no cycle such as:

grammar → IR → grammar

or:

grammar → runtime → grammar

or:

quantum grammar → hardware grammar → quantum grammar

---

88. Independent File Completion Contract

This file is considered complete only when:

- the complete language scope is defined;
- all major computational domains are identified;
- domain boundaries are explicit;
- quantum scope is defined;
- classical scope is defined;
- HDL scope is defined;
- hardware scope is defined;
- distributed scope is defined;
- AI/data scope is defined;
- networking/security scope is defined;
- future extensibility is defined;
- POCO-REAF scope is defined;
- scalability semantics are defined;
- resource/capability/constraint distinctions are defined;
- repository integration contracts are defined;
- ownership boundaries are defined;
- no artificial resource ceiling is introduced;
- "quantum::ir" remains the canonical quantum semantic boundary;
- the lexer/parser/AST relationship is defined;
- target-specific behavior is separated from language semantics;
- Rust 1.97/1.97.1 and no-"unsafe" implementation constraints are recorded;
- compatibility and evolution boundaries are defined;
- testing requirements are defined;
- hard-coding audit requirements are defined.

Completion of later files must not require reopening this document merely to establish any of these fundamental scope boundaries.

Later documents may refine a scope area, but they must not contradict this document without an explicit language-specification change.

---

89. Anti-Scope: What Zamani Is Not

Zamani is not:

- a CPU-only language;
- a GPU-only language;
- a quantum-only language;
- an HDL-only language;
- a classical-only language;
- a cloud-only language;
- an embedded-only language;
- a fixed-size quantum language;
- a fixed-topology distributed language;
- a vendor-specific programming language;
- a collection of unrelated DSLs;
- a backend instruction language;
- a hardware device registry;
- a runtime scheduler;
- a QEC decoder;
- a calibration database;
- a compiler optimization algorithm.

Those systems may integrate with Zamani but do not define the language itself.

---

90. Anti-Hardcoding Rule

The following architectural pattern is prohibited:

language feature
    ↓
hard-coded machine assumption

Examples:

quantum → 32 qubits
parallel → 64 threads
GPU → device 0
distributed → 128 nodes
FPGA → fixed number of LUTs
memory → fixed size
tensor → fixed accelerator dimensions
network → fixed topology

The correct pattern is:

language feature
    ↓
semantic requirement
    ↓
capability/resource negotiation
    ↓
target realization

---

91. Anti-Duplication Rule

No new grammar subsystem may duplicate an existing canonical repository subsystem.

For example:

grammar/quantum/

may define quantum source syntax.

It must not create:

grammar/quantum/QuantumIR

as a second canonical semantic representation when "src/quantum/ir/" already owns that responsibility.

The same principle applies to:

- resources;
- scheduling;
- optimization;
- QEC;
- ZQN;
- hardware;
- runtime.

---

92. Anti-Circularity Rule

No grammar component may require downstream execution infrastructure merely to parse source.

Therefore:

grammar

may depend conceptually on:

language specification

but not on:

running QPU
GPU driver
FPGA toolchain
network
runtime device discovery

for ordinary parsing.

---

93. Scale-Invariant Program Model

A central Zamani invariant is:

same source semantics
        ↓
different available resources
        ↓
different valid realizations

For example:

Program P

may be realized as:

P → one CPU
P → many CPUs
P → GPU
P → FPGA
P → QPU
P → cluster
P → heterogeneous system

without requiring the algorithm's semantic source to be rewritten solely because the machine changed.

---

94. Atom-to-Everywhere Principle

"From Atom to Everywhere" means that the same language architecture must accommodate radically different scales.

At the smallest scale:

single value
single instruction-like semantic operation
tiny embedded computation

At larger scales:

parallel program
heterogeneous system
distributed computation
quantum/classical system
hardware/software system
global deployment

The language must not require a separate semantic foundation for each scale.

---

95. One Program, Many Realizations

The desired model is:

                   ┌── CPU
                   ├── GPU
                   ├── FPGA
Zamani Program ────┼── ASIC
                   ├── QPU
                   ├── simulator
                   ├── embedded
                   ├── cluster
                   ├── cloud
                   └── future target

The common semantic source is the stable point.

Target realization is downstream.

---

96. Scope Invariants

The following invariants are mandatory.

Invariant 1 — Semantic portability

Changing hardware must not inherently change program semantics.

Invariant 2 — Resource neutrality

Language syntax must not impose arbitrary physical resource limits.

Invariant 3 — Canonical ownership

Each semantic concept has one authoritative implementation.

Invariant 4 — Quantum IR authority

"quantum::ir" is the canonical quantum semantic boundary.

Invariant 5 — Domain composability

Computational domains can compose within one language.

Invariant 6 — Target separation

Target realization is downstream from semantic representation.

Invariant 7 — Deterministic parsing

Parsing is independent of hardware and runtime state.

Invariant 8 — Safe compiler implementation

The Rust implementation uses no "unsafe".

Invariant 9 — Extensibility

Future computational paradigms can be added without redesigning the core language.

Invariant 10 — Compatibility

Existing valid language functionality is preserved or explicitly migrated.

Invariant 11 — Explicit constraints

Hardware-specific requirements are visible and typed.

Invariant 12 — Resource-bounded execution

Physical limitations remain explicit execution/resource constraints rather than hidden grammar limits.

---

97. Production-Readiness Definition

The language scope is production-ready when:

every supported domain
        ↓
has a defined semantic boundary
        ↓
has a defined ownership boundary
        ↓
has a defined syntax integration point
        ↓
has a defined AST integration point
        ↓
has a defined IR integration point
        ↓
has a defined compilation boundary
        ↓
has a defined execution boundary
        ↓
has tests
        ↓
has compatibility rules

and when no domain introduces an accidental scalability ceiling.

---

98. Final Scope Contract

The Zamani language shall follow this model:

                         ZAMANI
                           │
             ┌─────────────┴─────────────┐
             │                           │
       Portable Semantics          Explicit Requirements
             │                           │
             └─────────────┬─────────────┘
                           │
                    Semantic Analysis
                           │
                    Canonical IR
                           │
             ┌─────────────┼─────────────┐
             │             │             │
        Classical       Quantum        Hardware
             │             │             │
             └─────────────┼─────────────┘
                           │
                Optimization / Lowering
                           │
                Resource / Capability
                     Negotiation
                           │
                Scheduling / Mapping
                           │
                     Target System
                           │
                       Execution

The essential invariant is:

«The program expresses computation and intent. The available machine determines realization.»

Therefore Zamani shall support:

one program
    ↓
one semantic meaning
    ↓
many compilation contexts
    ↓
many target architectures
    ↓
many hardware configurations
    ↓
many execution environments
    ↓
many scales
    ↓
future computational technologies

subject only to the actual requirements, capabilities, policies, correctness conditions, and resources involved.

The grammar must never turn a temporary limitation of today's hardware into a permanent limitation of tomorrow's language.

Zamani: From Atom to Everywhere.

POCO-REAF: Program Once, Compile Once, Run Everywhere, Anywhere, Forever.This file is intentionally the scope contract, so later files such as scalability-model.md, poco-reaf.md, semantic-model.md, and compilation-model.md can add precise mechanisms without redefining what Zamani is allowed to encompass.