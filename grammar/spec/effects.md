Zamani Effect System Specification

Path: "grammar/spec/effects.md"
Status: Normative production specification
Language: Zamani
Grammar technology: ANTLR 4
Implementation target: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Safety: "#![forbid(unsafe_code)]"
Architectural scope: Source-language effect syntax and its contracts across lexer, parser, AST, semantic analysis, canonical IR, compiler, runtime, tooling, interoperability, and domain subsystems.

---

1. Purpose

This document specifies the production effect system of the Zamani programming language.

An effect describes semantically observable computational behavior associated with a declaration, expression, operation, function, module, computation, or execution boundary.

Effects allow Zamani programs to state properties such as:

- performs input/output;
- reads or mutates state;
- allocates or releases resources;
- performs quantum computation;
- performs measurement;
- interacts with hardware;
- communicates over a network;
- performs distributed computation;
- synchronizes concurrent activities;
- uses randomness;
- produces nondeterminism;
- accesses external state;
- performs cryptographic computation;
- performs security-sensitive operations;
- interacts with accelerators;
- invokes foreign functionality;
- performs reflection or metaprogramming;
- performs compilation-time or execution-time computation;
- introduces application-defined computational behavior.

The effect system is part of the semantic type-and-behavior contract of Zamani.

It is not a hardware model, resource allocator, scheduler, router, quantum IR, QEC system, ZQN model, runtime implementation, or backend API.

The effect system must therefore remain valid across:

- tiny systems;
- large systems;
- heterogeneous systems;
- distributed systems;
- classical systems;
- quantum systems;
- hybrid systems;
- embedded systems;
- HPC systems;
- accelerators;
- FPGA/ASIC systems;
- simulators;
- future computational substrates.

---

2. Architectural contract

The authoritative conceptual pipeline is:

Zamani source
    ↓
lexer
    ↓
parser
    ↓
frontend AST
    ↓
structural validation
    ↓
name/type/effect/ownership analysis
    ↓
semantic model
    ↓
ZUIR
    ↓
domain IR
    ↓
optimization
    ↓
routing / scheduling / resilience
    ↓
ZQN / HAL / target realization
    ↓
runtime

The effect system participates primarily between:

AST
 ↓
semantic effect analysis
 ↓
semantic model / ZUIR

and is subsequently consumed by compiler and runtime phases.

The grammar must never reverse this dependency.

In particular:

grammar
  ↓
AST
  ↓
semantic effect model
  ↓
IR

is valid.

This is invalid:

grammar
  ↔
runtime

or:

grammar
  ↔
hardware

or:

effect grammar
  ↓
quantum::ir

without the semantic boundary.

---

3. Authority and file ownership

The effect system is distributed across several repository layers.

3.1 Normative specification

This file:

grammar/spec/effects.md

owns the language-level effect contract.

It defines:

- effect terminology;
- semantic distinctions;
- permitted effect forms;
- effect composition;
- effect polymorphism;
- effect propagation;
- handler semantics;
- declaration/use boundaries;
- portability requirements;
- AST contract;
- semantic contract;
- IR integration contract;
- compiler/runtime integration requirements;
- diagnostics requirements;
- conformance requirements.

It does not replace ".g4" grammar files.

---

3.2 Syntax implementation

The existing:

grammar/effects/

owns modular effect syntax.

Existing files include:

effects.g4
effect-declarations.g4
effect-sets.g4
effect-handling.g4
custom-effects.g4
capabilities.g4
distributed.g4
hardware.g4
io.g4
network.g4
quantum.g4
security.g4

These files remain independently responsible for their syntactic portions.

They must conform to this specification.

They must not silently introduce semantic rules that contradict this document.

---

3.3 ANTLR composition root

The existing:

grammar/Zamani.g4

remains the parser composition root.

It must integrate effect syntax through the canonical grammar composition mechanism.

It must not create a second effect grammar.

There must not be competing effect roots such as:

grammar/Zamani.g4
grammar/antlr/Effects.g4
grammar/effects/effects.g4

all claiming independent authority.

"grammar/antlr/" must not become a second production grammar authority.

---

3.4 Implementation-conformance reference

The existing:

grammar/grammar.md

documents what the implementation currently accepts.

It must distinguish:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
EXPERIMENTAL
DEPRECATED
PLANNED

It must not silently override this specification.

---

3.5 Extended design material

The existing:

grammar/Zamani-Grammar.md

may contain broader or aspirational effect concepts.

Such concepts are not automatically legal Zamani syntax.

A proposed effect feature becomes normative only after:

proposal
 ↓
semantic design
 ↓
AST contract
 ↓
grammar contract
 ↓
implementation
 ↓
tests
 ↓
compiler/IR integration
 ↓
promotion to stable

---

4. Core terminology

The following terms are distinct and MUST NOT be conflated.

Concept| Meaning
Effect| What a computation does or may do
Capability| What an execution environment can provide
Requirement| What must be available for execution
Constraint| A condition that must hold
Resource| A consumable/allocatable computational entity
Preference| A desired implementation characteristic
Hint| Non-binding implementation guidance
Target| A realization environment
Placement| Where something is physically/logically realized
Schedule| When operations execute
Routing| How operations/data are mapped through topology
Policy| A rule controlling implementation behavior

For example:

effects { Quantum }

means:

«this computation has quantum-related semantic behavior.»

It does not mean:

requires 32 qubits

It does not mean:

use QPU 0

It does not mean:

map q0 → physical qubit 7

It does not select a vendor.

---

5. POCO-REAF requirement

The effect system is a direct part of Zamani's:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

architecture.

Effects must describe portable semantic behavior.

They must not encode accidental properties of today's hardware.

Therefore effect syntax MUST NOT impose universal limits such as:

MAX_EFFECTS
MAX_EFFECT_PARAMETERS
MAX_EFFECT_DEPTH
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_QUBITS
MAX_NODES
MAX_MEMORY
MAX_THREADS

The language has no artificial universal effect-cardinality limit.

An implementation may have resource-protection limits, but those are compiler policies and must be represented outside the language semantics.

For example:

effects {
    IO,
    Quantum,
    Network,
    Security,
    CustomEffect
}

must remain conceptually valid regardless of the number of effects available in the program.

---

6. Meaning of "infinity"

Zamani does not claim that a finite implementation can physically execute an infinite computation.

For POCO-REAF, "infinity" means:

«The language and its effect model impose no artificial finite machine-size boundary where the underlying semantics do not require one.»

The practical execution limit is determined by:

- available memory;
- compiler resources;
- runtime resources;
- target capabilities;
- execution environment;
- physical laws;
- explicit implementation policy.

These are not language-level effect limits.

---

7. Effect identity

An effect has a semantic identity.

Conceptually:

EffectIdentity =
    namespace
    +
    name
    +
    semantic version/identity

The grammar must permit qualified names.

For example:

effect Audit;

effect security::Audit;

effect application::Telemetry;

Effect identity must use the canonical naming/path model.

Effect grammar must not invent an incompatible identifier system.

It must integrate with:

grammar/core/
grammar/modules/
grammar/types/

and the corresponding frontend AST/name-resolution infrastructure.

---

8. Effect declarations

An effect declaration introduces a source-level effect.

Conceptually:

effect Logging;

A declaration may additionally contain:

- generic parameters;
- effect parameters;
- a result type;
- metadata;
- attributes;
- documentation;
- semantic constraints where explicitly supported.

The exact declaration grammar is owned by:

grammar/effects/effect-declarations.g4

The declaration must lower to the existing source-level AST effect representation.

The existing AST already models an effect declaration using its name, generic parameter "NodeId"s, parameter "NodeId"s, and optional return type, preserving the source-level structure instead of embedding backend-specific information.

---

9. Effect declaration versus effect use

These are separate concepts.

Declaration

effect Read<T>(...) -> ...;

introduces an effect.

Use

A computation may subsequently perform or acquire the effect.

Conceptually:

perform Read(...);

The exact use syntax belongs to the corresponding effect-operation grammar.

The declaration and use must not be represented as the same AST construct.

The declaration describes an effect's source-level interface.

The use describes an occurrence of effectful computation.

---

10. Effect sets

An effect set represents zero or more effects associated with a semantic context.

Conceptually:

effects {
    IO,
    Network,
    Security
}

An effect set has set semantics unless a future language feature explicitly defines an ordered effect structure.

Therefore:

effects { IO, Quantum }

and:

effects { Quantum, IO }

represent the same semantic set.

The implementation may preserve source ordering for diagnostics and source fidelity, but semantic comparison must use canonical effect identity.

The grammar must not impose an arbitrary maximum cardinality.

---

11. Empty effect sets

The language must distinguish:

no declared effects

from:

explicitly declared empty effect set

where the language syntax provides both concepts.

This distinction is important for:

- effect inference;
- API contracts;
- overriding declarations;
- generic effect parameters;
- diagnostics;
- tooling.

If the language elects not to expose an explicit empty set syntactically, semantic representation must still be capable of representing an empty effect set.

---

12. Duplicate effects

Duplicate effect identities in one semantic set must normalize to one effect identity unless the effect model explicitly introduces multiplicity.

For example:

effects {
    IO,
    IO
}

must not accidentally represent two independent IO effects.

Diagnostics may warn about redundant declarations.

The canonical semantic representation must use stable effect identity.

---

13. Effect parameters

Effects may have semantic parameters.

For example:

effect Transaction<Mode>;

or another language-defined parameterized effect.

Effect parameters must use canonical type/expression/generic syntax.

An effect parameter must represent genuine program semantics.

It must not be a disguised hardware capacity.

Invalid conceptual design:

effect Quantum<32>;

when "32" merely means:

«the current machine contains 32 physical qubits.»

The resource system must express that requirement instead.

---

14. Effect polymorphism

Zamani must support effect-polymorphic abstractions where the type/effect system supports them.

Conceptually:

fn compute<E: Effect>(value: T) effects { E } {
    ...
}

The exact generic syntax is owned jointly by:

grammar/functions/
grammar/types/
grammar/effects/

The effect specification defines the semantic contract.

Effect polymorphism permits reusable code to abstract over execution behavior.

This is important for POCO-REAF.

A function should not need to be rewritten merely because its effect is implemented through:

- a local machine;
- a remote machine;
- an accelerator;
- a simulator;
- a QPU;
- another future target.

---

15. Effect inference

Where supported, effect information may be inferred from a computation.

For example:

fn calculate() {
    perform Something;
}

may infer an effect set containing "Something".

Inference must be semantic.

The parser must not attempt effect inference.

Explicit effect declarations remain useful for:

- API contracts;
- verification;
- security analysis;
- optimization;
- documentation;
- interoperability;
- static checking.

---

16. Effect subtyping and inclusion

If the semantic type system supports effect inclusion, the following relation must be explicit:

required effects ⊆ available effects

A computation that may perform:

{ IO, Network }

cannot be treated as effect-free merely because its caller declares:

{ IO }

unless the missing effect has been legitimately:

- handled;
- discharged;
- transformed;
- encapsulated;
- otherwise proven absent.

The grammar does not perform this proof.

The semantic analyzer does.

---

17. Effect propagation

Effects propagate through semantic dependencies.

Conceptually:

A
 |
 +-- calls B
       |
       +-- effects { IO, Quantum }

The semantic system determines the effect context of "A".

The grammar only represents explicit source declarations and handlers.

Propagation belongs to semantic analysis.

This keeps grammar parsing deterministic and inexpensive.

---

18. Effect handling

Effect handlers provide a semantic boundary at which an effect can be interpreted, intercepted, transformed, or discharged.

The syntax is owned by:

grammar/effects/effect-handling.g4

A handler must not directly encode:

- operating-system calls;
- device identifiers;
- QPU IDs;
- GPU IDs;
- physical addresses;
- vendor SDK objects;
- hardware topology;
- scheduling algorithms.

A handler expresses language-level semantics.

The implementation determines how those semantics are realized.

---

19. Resumable and non-resumable effects

The semantic system may distinguish:

resumable
non-resumable

effects.

If this distinction is exposed in source syntax, it must be represented explicitly in the effect semantic contract.

The parser must preserve the declaration.

Semantic analysis determines whether a particular handler is valid.

The runtime determines implementation behavior only after lowering.

---

20. Effect transformation

A handler or semantic boundary may transform one effect context into another.

Conceptually:

Effect A
    ↓
handler
    ↓
Effect B

For example, an abstract external effect might be implemented through a local effect or remote service effect.

Such transformations belong to semantic lowering.

The grammar must not encode backend-specific transformation tables.

---

21. Effect aliases

If aliases are supported, an alias must preserve semantic identity rather than create an unrelated effect.

Conceptually:

effect_alias Audit = security::Audit;

The exact syntax belongs to:

grammar/effects/custom-effects.g4

Alias resolution belongs to name resolution.

An alias must not create a second incompatible effect universe.

---

22. Custom effects

Users must be able to define effects.

Examples:

effect Telemetry;

effect application::Audit;

effect domain::Simulation;

Custom effects are essential for extensibility.

The standard library must therefore not be the complete universe of legal effects.

The parser must recognize the structural form.

The semantic registry determines whether an effect is:

- standard;
- imported;
- user-defined;
- dialect-defined;
- experimental;
- deprecated;
- unresolved.

---

23. Standard effect vocabulary

Zamani may provide standard semantic categories such as:

IO
State
Mutation
Allocation
Deallocation
Memory
Quantum
Classical
Hardware
Distributed
Parallel
Synchronization
Concurrency
Network
Security
Cryptography
Randomness
Nondeterminism
Time
Environment
Persistence
External
Accelerator
Device
Interrupt
System
Foreign
Reflection
Compilation
Execution

These are vocabulary-level semantic identities.

They are not mandatory closed enums in the parser.

A future computational domain must not require modifying the fundamental grammar merely because a new effect category appears.

---

24. Domain-specific effects

Domains may define specialized effects.

Examples include:

Classical

State
Mutation
IO
Randomness

Quantum

Quantum
Measurement
Reset
Entanglement

HDL

Hardware
Signal
Clock
Timing

Distributed

Distributed
Network
RemoteExecution
Replication

Security

Security
Cryptography
Identity
Confidentiality

AI

Training
Inference
Differentiation
Accelerator

These are semantic categories, not hardware implementations.

---

25. Quantum effect integration

Quantum effects must integrate with:

grammar/quantum/
src/quantum/
quantum::ir
QEC
ZQN
routing
scheduling
optimization
HAL
runtime

The correct direction is:

Zamani source
    ↓
lexer
    ↓
parser
    ↓
AST
    ↓
semantic effect analysis
    ↓
quantum semantic lowering
    ↓
quantum::ir
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
QEC / resilience / ZQN / HAL
    ↓
target realization

Effects do not replace "quantum::ir".

Effects do not create another quantum IR.

Effects do not implement QEC.

Effects do not implement ZQN.

Effects do not implement routing.

Effects do not implement scheduling.

Effects merely communicate semantic behavior.

---

26. Quantum portability

This must remain valid:

effect Quantum;

whether execution eventually occurs on:

- a quantum simulator;
- superconducting hardware;
- trapped-ion hardware;
- neutral-atom hardware;
- photonic hardware;
- another quantum technology;
- a future quantum substrate.

The effect must not contain:

QPU_ID
PHYSICAL_QUBIT
MAX_QUBITS
FIXED_TOPOLOGY

unless those concepts are explicitly part of a separate target-specific realization language.

---

27. QEC boundary

Quantum error correction belongs to the QEC subsystem.

Effect syntax may identify semantic properties relevant to error correction.

For example:

effect FaultTolerantQuantum;

could be a semantic effect if standardized.

However, effect grammar must not encode:

- code distance;
- stabilizer matrices;
- decoder algorithms;
- syndrome extraction implementation;
- correction schedules;
- physical-qubit allocation.

The pipeline remains:

effect semantics
    ↓
semantic analysis
    ↓
quantum::ir
    ↓
QEC analysis/transformation

---

28. ZQN boundary

ZQN owns fault/noise semantics.

Effects may identify that a computation has noise-sensitive or fault-aware behavior.

They must not duplicate:

- noise models;
- fault models;
- leakage;
- erasure;
- loss;
- correlated faults;
- drift;
- fault classification.

Those belong to ZQN.

The effect system may provide semantic inputs to ZQN but must not become a second ZQN.

---

29. Hardware boundary

An effect is not hardware.

Therefore:

effect GPU;

must not automatically mean:

select GPU 0

and:

effect Quantum;

must not mean:

select QPU 0

Hardware selection belongs to:

hardware/
resources/
compile/
execution/

and downstream target realization.

---

30. Capability boundary

The distinction is mandatory.

Effect:
    what the computation does.

Capability:
    what the environment can provide.

For example:

effects { Quantum }

may be combined semantically with:

requires capability("quantum.measurement")

The compiler can then determine feasibility.

The grammar must preserve both concepts independently.

---

31. Resource boundary

An effect is not a resource requirement.

For example:

effects { Quantum }

does not imply:

requires 100 qubits

Likewise:

effects { Accelerator }

does not imply:

requires 8 GPUs

Resource requirements belong to:

grammar/resources/

and hardware requirements belong to:

grammar/hardware/

---

32. Requirement/capability/effect composition

The compiler may perform:

Effects
   +
Requirements
   +
Capabilities
   +
Constraints
   ↓
Feasibility analysis

For example:

computation:
    effects = { Quantum, Network }

requirements:
    capability("quantum.measurement")

environment:
    capabilities = discovered dynamically

The result is determined downstream.

The grammar itself does not perform environment discovery.

---

33. Effect and scheduling

Effects may constrain legal transformations.

For example, two effectful operations may not be freely reordered if their semantic effects interact.

However, effects do not own scheduling.

Scheduling remains responsible for:

- operation ordering;
- resource conflicts;
- timing;
- ASAP;
- ALAP;
- critical paths;
- dynamic scheduling;
- distributed scheduling;
- quantum scheduling.

The effect system supplies semantic constraints.

---

34. Effect and optimization

Optimizers must treat effects as semantic transformation barriers where required.

For example, an optimizer cannot assume that two arbitrary effectful operations are interchangeable merely because their computed values are equal.

Effect information can therefore influence:

- common-subexpression elimination;
- code motion;
- dead-code elimination;
- inlining;
- parallelization;
- vectorization;
- quantum optimization;
- hardware lowering.

Optimization algorithms remain outside this specification.

---

35. Effect and concurrency

Effects interact with:

grammar/concurrency/

and semantic concurrency analysis.

Potential effect properties include:

thread-local
shared
synchronizing
blocking
atomic
distributed
nondeterministic

These properties must be semantic.

The effect grammar must not hard-code:

8 threads
16 workers
32 cores

or any equivalent machine limit.

---

36. Effect and distributed execution

Distributed effects may describe semantic remote behavior.

For example:

effect RemoteExecution;
effect DistributedState;

They must not identify:

node0
node1
node2

as universal language constructs.

The distributed subsystem determines actual placement and topology.

---

37. Effect and networking

Networking effects can represent semantic interaction with external communication.

Examples:

effect Network;
effect RemoteExecution;
effect Streaming;

The grammar must not require a fixed:

- number of endpoints;
- network topology;
- address space;
- transport;
- network device.

Concrete networking realization belongs downstream.

---

38. Effect and security

Security effects may identify security-sensitive behavior.

Examples:

effect Security;
effect Cryptography;
effect SecretAccess;

Security semantics must integrate with:

grammar/security/

but effect syntax must not become a duplicate cryptographic language.

The effect system does not implement cryptographic algorithms.

---

39. Effect and AI

AI-related effects may represent semantic operations such as:

Training
Inference
Differentiation
ModelExecution

The grammar must remain framework-neutral.

It must not encode:

CUDA
PyTorch
TensorFlow
specific accelerator model

as universal effect semantics.

Framework integration belongs to interoperability and backend layers.

---

40. Effect and HDL

HDL-related effects may express semantic hardware interaction:

Hardware
Signal
Clock
Timing
Simulation
Synthesis
Verification

Effects must not become a second HDL.

Actual hardware structure belongs to:

grammar/hdl/

and target-independent hardware intent belongs to:

grammar/hardware/

---

41. Effect and memory

Memory-related effects may represent:

Allocation
Deallocation
Mutation
Persistence
SharedMemory
DistributedMemory

They must not encode physical memory capacity.

Invalid universal grammar semantics:

memory = 64GB

as a language-wide limit.

Valid program semantics may contain an actual program value such as:

buffer_size = 64GB

when that quantity is genuinely part of the program's semantics.

---

42. Effect and time

Time-related effects must not assume a fixed clock width, timestamp width, or machine timer.

The effect model must remain capable of representing:

- logical time;
- physical time;
- temporal effects;
- deadlines;
- time-dependent computation;
- distributed temporal behavior.

Actual timing realization belongs downstream.

---

43. Effect and nondeterminism

Nondeterministic behavior must be distinguishable from ordinary deterministic effects.

This is important for:

- reproducibility;
- testing;
- verification;
- distributed execution;
- quantum measurement;
- randomized algorithms;
- speculative execution.

The semantic model must be able to represent whether an effect introduces nondeterminism.

The grammar must not confuse:

Randomness

with:

Nondeterminism

unless the language specification explicitly defines their relationship.

---

44. Effect and determinism

Effect analysis must cooperate with the repository's determinism/provenance infrastructure.

The semantic pipeline should be able to determine:

deterministic
conditionally deterministic
nondeterministic
externally determined

where the broader language model supports those classifications.

This must not require runtime state inside the AST.

---

45. Effect and purity

A computation with no observable effects may be classified as pure by semantic analysis.

Purity must not be inferred solely from the absence of a syntactic annotation unless the language explicitly specifies that rule.

The semantic analyzer must account for inferred effects.

A function declared effect-free must be checked against its body.

---

46. Effect contracts on functions

Functions may explicitly declare their effects.

Conceptually:

fn read_data() effects { IO } {
    ...
}

or:

fn execute() effects {
    IO,
    Quantum,
    Network
} {
    ...
}

The function grammar and effect grammar must share one canonical effect-set representation.

There must not be separate incompatible function-effect syntax and generic effect syntax.

---

47. Effect contracts on expressions and operations

Where required, effect annotations may apply to:

- expressions;
- operations;
- declarations;
- functions;
- modules;
- blocks;
- foreign calls;
- execution boundaries;
- domain constructs.

The general annotation/attribute mechanism should be reused.

Effect grammar must not create unnecessary parallel annotation syntax.

---

48. Source metadata

Effect syntax must preserve sufficient source information for:

- diagnostics;
- IDE tooling;
- formatting;
- source maps;
- refactoring;
- provenance;
- semantic tracing.

At minimum, semantic effect nodes must be traceable to:

source file
source span
effect identity
containing construct

The existing AST architecture already treats source-level effect information as source structure rather than compiler-resolved meaning.

---

49. AST contract

Every effect construct must have a predetermined AST mapping before its grammar is considered complete.

The required direction is:

grammar
    ↓
AST node
    ↓
semantic effect model
    ↓
ZUIR / canonical IR

The effect AST must preserve:

- effect name;
- namespace/path;
- generic parameters;
- effect parameters;
- return type where applicable;
- source span;
- attributes/metadata;
- declaration/use distinction;
- source ordering where required.

The existing "Effect" AST node already follows this general architecture, including "NodeId" references to generic parameters, parameters, and optional return type.

---

50. AST ownership

The effect declaration node owns references to its children.

It does not own the entire AST store.

Conceptually:

Effect
 ├── name
 ├── generic parameter NodeIds
 ├── parameter NodeIds
 └── return type NodeId

The canonical AST store owns the child nodes.

This permits independent evolution of:

- generic parameters;
- function parameters;
- types;
- expressions;
- declarations.

---

51. Semantic contract

Semantic analysis must resolve:

- effect identity;
- namespace;
- imports;
- aliases;
- generic parameters;
- effect parameters;
- parameter types;
- return types;
- effect-set membership;
- effect inclusion;
- handler compatibility;
- effect propagation;
- effect inference;
- capability implications;
- resource implications;
- domain-specific semantics;
- determinism implications;
- security implications.

None of these semantic operations belong in ANTLR parser actions.

---

52. Semantic side tables

Resolved semantic information should be associated with stable AST identities.

Conceptually:

NodeId
   ↓
semantic effect information

rather than mutating the source AST into a backend-specific representation.

This maintains the separation:

source AST
    ≠
semantic model
    ≠
IR
    ≠
backend

---

53. ZUIR integration

Effects must lower into the repository's canonical universal semantic representation.

The required conceptual boundary is:

Effect AST
    ↓
semantic effect model
    ↓
ZUIR
    ↓
domain IR

Effects must not make the AST into ZUIR.

Effects must not introduce a parallel "effect IR" that competes with the canonical IR architecture.

An implementation may have an internal semantic effect data structure, but it must remain an implementation of the semantic contract rather than a second source-level language model.

---

54. Quantum IR integration

Quantum effects must eventually interact with:

quantum::ir

through semantic lowering.

The effect system does not own "quantum::ir".

The dependency must remain:

effect semantics
    ↓
quantum semantic lowering
    ↓
quantum::ir

and never:

effect grammar
    ↓
custom quantum IR

This preserves the repository's canonical quantum semantic boundary.

---

55. Compiler integration

The compiler should conceptually expose semantic structures equivalent to:

EffectId
EffectSet
EffectDeclaration
EffectParameter
EffectContext
EffectHandler

These structures belong in compiler/frontend/semantic infrastructure, not in ".g4" grammar files.

The grammar creates parse structures.

The AST stores source structure.

The semantic layer resolves meaning.

The compiler consumes the semantic model.

---

56. Runtime integration

The runtime must never parse ".g4" files.

The runtime consumes lowered semantic/IR information.

The runtime may use effect information to implement:

- dispatch;
- permissions;
- resource acquisition;
- effect handlers;
- instrumentation;
- tracing;
- execution policies;
- fault handling.

Those are runtime responsibilities.

The grammar must remain independent of runtime implementation.

---

57. Hardware and HAL integration

Effects may contribute to target feasibility analysis.

The eventual chain may be:

effect semantics
    +
resource requirements
    +
capability requirements
    +
target constraints
    ↓
HAL/environment discovery
    ↓
feasible realization

The HAL determines actual device state/capability.

The effect grammar does not.

---

58. Routing integration

Effects may prevent certain semantic transformations if reordering would change observable behavior.

Routing remains responsible for physical realization.

For quantum computation:

effect semantics
 ↓
quantum::ir
 ↓
routing

Effects must not contain physical qubit assignments.

---

59. Scheduling integration

Scheduling may consume effect information to determine dependencies and legal execution ordering.

The effect grammar does not own scheduling algorithms.

This preserves:

effects → semantic constraints
scheduling → execution ordering

rather than:

effects → scheduling implementation

---

60. Resilience integration

Effects may identify semantic properties relevant to resilience.

However:

QEC
ZQN
resilience
HAL
routing
scheduling

remain independent subsystems.

The effect system provides semantic information only.

For example:

effect FaultSensitiveQuantum;

could inform downstream analysis.

It must not itself implement recovery.

---

61. Capability negotiation

Effect semantics may participate in capability negotiation.

For example:

effects { Quantum, Measurement }

may lead semantic analysis to determine that the computation requires capabilities corresponding to those behaviors.

The negotiation process belongs to:

grammar/resources/
grammar/hardware/
compile/
execution/
HAL

The effect grammar must not contain environment-discovery logic.

---

62. Interoperability

Foreign interfaces may have effects.

For example:

foreign_call(...)

may be semantically classified as:

Foreign
External
IO

or another appropriate effect.

The interoperability layer owns ABI and foreign-language details.

The effect system merely records semantic behavior.

OpenQASM, QIR, LLVM, MLIR, C, C++, Rust, Python and vendor interfaces remain interoperability/target concerns rather than effect-language authorities.

---

63. Dialects

Dialects may define additional effects.

A dialect-defined effect must have:

dialect identity
effect identity
version
semantic contract
AST mapping
IR mapping
compatibility contract
feature status

A dialect must not silently modify the meaning of a standard effect.

Dialect syntax must remain compatible with:

grammar/dialects/
grammar/compatibility/

---

64. Effect registry

The semantic implementation should maintain a registry capable of resolving:

standard effects
user effects
imported effects
dialect effects
experimental effects
deprecated effects

The registry must not require changing parser grammar rules whenever an effect is added.

This is a critical extensibility requirement.

The parser recognizes effect identifiers structurally.

The semantic registry gives those identifiers meaning.

---

65. No closed effect enum in the grammar

The parser must not require a rule such as:

effectName
    : IO
    | Quantum
    | GPU
    | Network
    | ...

for the complete effect universe.

Such a closed enumeration would make the language unable to evolve.

A generic identifier/path rule should provide effect identity.

Semantic validation determines whether the referenced effect exists.

---

66. Effect ordering

Effect sets are semantically unordered.

Source ordering must nevertheless be retained where needed for:

- diagnostics;
- formatting;
- source fidelity;
- tooling;
- deterministic serialization.

Canonical semantic comparison must normalize ordering.

This provides deterministic compiler behavior without changing source representation.

---

67. Effect normalization

Semantic normalization should:

1. resolve names;
2. resolve aliases;
3. canonicalize namespaces;
4. remove redundant duplicates;
5. normalize parameters;
6. normalize effect sets;
7. preserve source provenance.

Normalization must be deterministic.

Equivalent source programs must produce equivalent semantic effect sets.

---

68. Diagnostics

Effect diagnostics must distinguish at least:

unknown effect
duplicate effect
invalid effect parameter
invalid effect declaration
effect mismatch
unhandled effect
invalid handler
effect escaping scope
effect contract violation
invalid effect alias
deprecated effect
experimental effect
incompatible dialect effect
effect capability mismatch
effect requirement mismatch

Diagnostics must include source spans.

Diagnostics must not expose backend-specific implementation details unless the compiler has reached a later target-specific phase.

---

69. Error recovery

ANTLR error recovery must remain parser-level.

Semantic effect errors must not be encoded as parser recovery rules.

The parser should preserve as much valid structure as possible.

Malformed source must not cause:

- unsafe behavior;
- arbitrary code execution;
- filesystem access;
- network access;
- backend execution.

---

70. Security requirements

The effect grammar and semantic representation must:

- use no "unsafe";
- perform no I/O;
- perform no network access;
- execute no source program;
- execute no user-provided effect handler during parsing;
- avoid raw pointers;
- avoid memory-address semantics;
- avoid hidden global mutable state;
- avoid backend discovery.

The Rust implementation must use:

#![forbid(unsafe_code)]

and target Rust 1.97 / 1.97.1 stable, edition 2021.

---

71. Determinism

Effect parsing and semantic normalization must be deterministic.

Given the same:

source
grammar version
language version
dialect versions
semantic environment

the parser and semantic effect normalization must produce the same result.

There must be no dependence on:

- memory addresses;
- hash iteration order where observable;
- wall-clock time;
- random numbers;
- machine-specific ordering.

---

72. Parallel compilation

Effect analysis should be designed so independent declarations can be analyzed concurrently where the surrounding compiler infrastructure permits it.

The effect model must not require global mutable parser state.

No effect count should be limited merely to make parallel compilation convenient.

Actual compiler resource limits remain explicit implementation policy.

---

73. Scalability

The effect system must scale from:

one effect

to:

many effects

and to programs containing arbitrarily many effectful constructs subject only to implementation resources.

Avoid fixed-size representations such as:

Effect[8]

or:

EffectSet8

as language-level semantics.

Ordered collections may be represented using dynamically sized structures such as Rust "Vec" where appropriate.

---

74. Memory/resource protection

A compiler may impose configurable limits for denial-of-service protection.

Examples:

maximum source size
maximum AST nodes
maximum nesting depth
maximum parser work
maximum semantic-analysis work
maximum effect expansion

These must be:

- implementation policies;
- configurable where appropriate;
- documented;
- independent of language semantics;
- distinguishable from language limits.

They must never be presented as universal Zamani language restrictions.

---

75. Hard-coding audit

The following are prohibited as universal effect-system limits:

MAX_EFFECTS
MAX_EFFECT_PARAMETERS
MAX_EFFECT_NESTING
MAX_CPUS
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QUBITS
MAX_QPU_COUNT
MAX_NODES
MAX_MEMORY
MAX_DEVICES

Also prohibited as universal effect semantics:

effect GPU0
effect QPU0
effect CPU0
effect physical_qubit_0

unless such constructs are explicitly confined to a target-specific realization language and never presented as portable effect semantics.

---

76. Semantic constants versus implementation limits

This distinction is mandatory.

Valid:

let n = 1024;

because "1024" may be program data.

Valid:

requires qubits >= n;

because the value is a semantic requirement.

Invalid:

ZAMANI_MAX_EFFECTS = 32;

when used as a universal language restriction.

Invalid:

grammar accepts at most 32 effects

merely because today's compiler implementation has that capacity.

---

77. Effect grammar modularization

The existing effect files should remain specialized.

Recommended responsibility:

effects.g4
    generic effect composition entry

effect-declarations.g4
    declarations

effect-sets.g4
    sets and references

effect-handling.g4
    handlers

custom-effects.g4
    user-defined/extended effects

capabilities.g4
    effect/capability boundary syntax where required

io.g4
    IO-specific composition

quantum.g4
    quantum-specific effect composition

hardware.g4
    hardware-related semantic effect composition

distributed.g4
    distributed effects

network.g4
    network effects

security.g4
    security effects

No file may silently become a second effect authority.

---

78. Integration with "grammar/effects/README.md"

"grammar/effects/README.md" is the subsystem navigation and implementation contract.

This specification is the normative semantic contract.

The README must link conceptually to this document.

It must not redefine contradictory semantics.

The existing README already establishes that effects describe semantic behavior and explicitly separates them from capabilities, resources, QEC, ZQN, scheduling, hardware discovery and runtime behavior.

---

79. Integration with "grammar/spec/type-system.md"

The type-system specification owns the relationship between:

types
generics
effect polymorphism
effect constraints

This document owns effect semantics.

Neither specification may duplicate the other.

The shared contract is:

type system
    ↔
effect system

through well-defined effect variables, bounds and constraints.

---

80. Integration with "grammar/spec/semantics.md"

The semantics specification owns general evaluation and language meaning.

This file specializes that contract for effects.

The relationship is:

semantics.md
    ↓
general semantic model

effects.md
    ↓
effect-specific semantic model

---

81. Integration with "grammar/spec/resources.md"

The resource specification owns:

- requirements;
- resource quantities;
- resource budgets;
- capabilities;
- preferences;
- constraints.

Effects may produce semantic requirements.

They do not redefine the resource model.

---

82. Integration with "grammar/spec/quantum.md"

The quantum specification owns quantum-language semantics.

This document owns the effect dimension.

The two must meet through:

Quantum source semantics
        +
Quantum-related effects
        ↓
semantic model
        ↓
quantum::ir

No second quantum IR is permitted.

---

83. Integration with "grammar/spec/compatibility.md"

Every effect feature must be traceable through:

specification
    ↓
Zamani.g4
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
compiler
    ↓
runtime

Compatibility status must be recorded.

---

84. Integration with "src/frontend/ast/"

The AST is the source-level structural boundary.

Effect syntax must lower into the existing effect AST infrastructure.

The existing effect AST implementation deliberately keeps the representation independent of:

- CPU/GPU/FPGA/ASIC/QPU architecture;
- physical qubits;
- routing;
- scheduling;
- calibration;
- QEC;
- ZQN;
- runtime dispatch;
- vendor SDKs;
- LLVM/MLIR/QIR.

This specification preserves that design.

---

85. Integration with "src/quantum/"

The effect subsystem must not import quantum implementation modules merely because an effect is named "Quantum".

Quantum-specific semantics are resolved after parsing.

This keeps the dependency direction clean.

---

86. Integration with compiler phases

The compiler should consume effect information in phases such as:

name resolution
 ↓
type checking
 ↓
effect checking
 ↓
ownership/resource checking
 ↓
capability/requirement analysis
 ↓
IR lowering
 ↓
optimization
 ↓
target realization

Effect analysis must be complete before transformations that rely upon effect purity or observability.

---

87. Integration with runtime

Runtime effect handlers must be selected from lowered semantic information.

The runtime must not infer language semantics by reading source grammar files.

This guarantees that the runtime remains independent of ANTLR.

---

88. Tooling contract

IDE/tooling implementations must be able to query:

effect declaration
effect use
effect set
effect handler
effect source span
effect identity
effect status

Tooling must not need to understand backend-specific realization merely to provide source-level effect navigation.

---

89. Documentation contract

Documentation generated from the effect specification must clearly distinguish:

syntax
semantics
capabilities
requirements
implementation

Documentation must never describe:

effect Quantum

as meaning a specific quantum processor.

---

90. Testing contract

The effect subsystem is not production-ready with only positive parsing tests.

Tests must cover:

lexical
syntax
AST
semantic
diagnostics
negative
boundary
scalability
determinism
compatibility
portability

---

91. Required positive tests

At minimum:

effect IO;
effect Quantum;
effect application::Audit;
effect security::Audit;
effect Generic<T>;

Effect sets:

effects {};
effects { IO };
effects { IO, Quantum };
effects { Quantum, IO, Network, Security };

Function contracts:

fn f() effects { IO } { ... }

Custom effects:

effect application::Telemetry;

Handlers:

handle ... with ...;

using the final handler grammar.

---

92. Required negative tests

At minimum:

unknown effect reference
malformed effect declaration
duplicate conflicting declaration
invalid parameter
invalid generic parameter
invalid handler
unhandled effect
effect contract mismatch
invalid alias
invalid qualified effect name

---

93. Boundary tests

Boundary testing must include:

zero effects
one effect
many effects
deeply nested valid effect contexts
large parameter lists
large generic effect sets
large source spans
large qualified names

No boundary test may establish an artificial universal maximum.

---

94. Scalability tests

Scalability tests must verify that the grammar and semantic model do not introduce fixed machine-size assumptions.

Test progressively larger:

effect declarations
effect sets
effect parameters
generic effect variables
nested handlers
call graphs
distributed effect graphs
quantum effect graphs
hybrid effect graphs

until constrained by test-environment resources.

The test suite must distinguish:

implementation resource exhaustion

from:

language rejection

---

95. Quantum scalability tests

Quantum effect tests must include:

single-qubit semantic program
parameterized qubit count
large symbolic qubit count
dynamic quantum resource requirements
mid-circuit measurement
classical feed-forward
logical operations
fault-aware semantics
custom quantum operations

No test may assume a maximum number of qubits.

---

96. Distributed scalability tests

Distributed effect tests must include:

one execution location
multiple locations
large symbolic location sets
dynamic placement
remote effects
replication effects
fault-tolerant effects

No fixed node count may be encoded.

---

97. Determinism tests

Equivalent effect sets:

{ IO, Quantum }

and:

{ Quantum, IO }

must normalize equivalently.

Repeated compilation of the same source and semantic environment must produce deterministic effect metadata.

---

98. Compatibility tests

For every effect construct, verify:

specification
    ↔
grammar
    ↔
lexer
    ↔
parser
    ↔
AST
    ↔
semantic model
    ↔
IR
    ↔
compiler
    ↔
runtime

A feature is not complete if only the grammar parses it.

---

99. Feature manifest contract

Each substantial effect feature should eventually have a machine-readable feature manifest under the repository's feature-contract system.

Conceptually:

id:
name:
status:
version:
domain:
syntax:
grammar:
lexer_tokens:
ast_nodes:
semantic_rules:
ir_mapping:
compiler_consumers:
runtime_consumers:
capabilities:
resource_requirements:
positive_tests:
negative_tests:
boundary_tests:
scalability_tests:
determinism_tests:
compatibility:
hard_coding_policy:

The manifest makes the feature independently completable.

---

100. "Done means done" contract

An effect grammar file is complete only when all relevant contracts are predetermined.

For every effect grammar component:

✓ purpose
✓ ownership
✓ non-ownership
✓ lexical dependencies
✓ grammar dependencies
✓ AST mapping
✓ source-span behavior
✓ semantic mapping
✓ effect identity
✓ effect-set behavior
✓ diagnostics
✓ IR integration
✓ compiler integration
✓ runtime integration
✓ tooling integration
✓ interoperability
✓ domain integration
✓ positive tests
✓ negative tests
✓ boundary tests
✓ scalability tests
✓ determinism tests
✓ compatibility tests
✓ portability audit
✓ hard-coding audit
✓ security audit
✓ Rust 1.97/1.97.1 compatibility
✓ no unsafe implementation requirement

A file must not be considered complete merely because its parser rules compile.

---

101. No re-edit-after-integration principle

The effect subsystem must be designed so that an individual file can be completed against already-defined contracts.

Before implementing a new effect grammar file, its integration points must already be known:

tokens
 ↓
grammar rule
 ↓
AST
 ↓
semantic model
 ↓
IR
 ↓
compiler
 ↓
runtime
 ↓
tests

If a later file requires changing the meaning of an earlier file, that indicates that the contract was incomplete.

The correct response is to fix the contract before marking the earlier file complete.

---

102. Versioning

Effect semantics are versioned with the Zamani language specification.

Changes must distinguish:

additive
clarifying
behavior-changing
breaking
deprecated
experimental

A new effect may normally be additive.

Changing the meaning of an existing effect requires compatibility analysis.

---

103. Deprecation

Deprecated effects must remain parseable for the compatibility period defined by the language version.

The semantic layer should emit diagnostics.

The grammar must not silently reinterpret an old effect as a new effect.

Migration information belongs in:

grammar/compatibility/

---

104. Experimental effects

Experimental effects must be explicitly identifiable by the language/tooling feature-status mechanism.

Experimental status must not require a second grammar.

Feature gating belongs to the compatibility/dialect mechanism.

---

105. Future computational substrates

A future computational technology must be capable of introducing new effects without modifying the universal effect architecture.

For example, a future substrate might introduce:

effect FutureCompute;

or a qualified effect:

effect future::Computation;

The grammar must already be structurally capable of representing it.

The semantic registry, AST mapping and dialect/standard-library contracts provide the extension mechanism.

---

106. What the effect system must never become

The effect system must never become:

hardware description language
resource allocator
device manager
quantum router
quantum scheduler
QEC implementation
ZQN implementation
runtime
vendor API
ABI
compiler backend
cryptographic implementation
AI framework
network implementation

Those systems may consume effect information.

They do not belong inside the effect grammar.

---

107. Canonical dependency graph

The complete dependency direction is:

                         ┌──────────────┐
                         │   Source     │
                         └──────┬───────┘
                                ↓
                         ┌──────────────┐
                         │    Lexer     │
                         └──────┬───────┘
                                ↓
                         ┌──────────────┐
                         │    Parser    │
                         └──────┬───────┘
                                ↓
                         ┌──────────────┐
                         │     AST      │
                         └──────┬───────┘
                                ↓
                   ┌────────────────────────┐
                   │ Semantic Effect Model  │
                   └────────────┬───────────┘
                                ↓
                            ┌───────┐
                            │ ZUIR  │
                            └───┬───┘
                                ↓
             ┌──────────────────┼──────────────────┐
             ↓                  ↓                  ↓
       Classical IR       quantum::ir       HDL/Hardware IR
             │                  │                  │
             └──────────────────┼──────────────────┘
                                ↓
                         Optimization
                                ↓
                    ┌───────────┼───────────┐
                    ↓           ↓           ↓
                 Routing    Scheduling   Resilience
                    │           │           │
                    └───────────┼───────────┘
                                ↓
                         ZQN / HAL
                                ↓
                      Target realization
                                ↓
                            Runtime

The effect system participates at the semantic layer.

---

108. Canonical effect lifecycle

Every effect should follow:

Declare
   ↓
Parse
   ↓
AST
   ↓
Resolve
   ↓
Validate
   ↓
Normalize
   ↓
Infer/propagate
   ↓
Check handlers
   ↓
Check capabilities/requirements
   ↓
Lower
   ↓
Optimize subject to effect semantics
   ↓
Execute

---

109. Production invariants

The following invariants are mandatory.

Invariant 1 — No hardware leakage

Effect syntax cannot require knowledge of a physical machine.

Invariant 2 — No fixed effect universe

User-defined and future effects are possible.

Invariant 3 — No duplicate IR

Effects do not create a competing semantic IR.

Invariant 4 — Canonical AST

Effect constructs use the existing frontend AST architecture.

Invariant 5 — Semantic separation

Effect meaning is resolved after parsing.

Invariant 6 — Capability separation

Effect ≠ capability.

Invariant 7 — Resource separation

Effect ≠ resource.

Invariant 8 — Requirement separation

Effect ≠ requirement.

Invariant 9 — Target separation

Effect ≠ target.

Invariant 10 — Determinism

Equivalent source semantics normalize deterministically.

Invariant 11 — Safe implementation

No "unsafe" Rust.

Invariant 12 — Scalability

No artificial machine-size limits.

Invariant 13 — Extensibility

New computational domains can introduce effects without redesigning the effect architecture.

Invariant 14 — Traceability

Every syntax construct has a predetermined AST → semantic → IR path.

Invariant 15 — Runtime independence

The runtime never needs to parse grammar files.

---

110. Completion criteria for "grammar/spec/effects.md"

This specification is complete when:

✓ Effect terminology is defined
✓ Effect/capability/resource/requirement boundaries are explicit
✓ Effect declaration semantics are defined
✓ Effect-use semantics are separated
✓ Effect sets are defined
✓ Effect composition is defined
✓ Effect normalization is defined
✓ Effect polymorphism is defined
✓ Effect inference is defined
✓ Effect propagation is defined
✓ Effect handling is defined
✓ Effect transformation is defined
✓ Custom effects are supported
✓ Qualified effects are supported
✓ Domain effects are extensible
✓ Quantum integration is defined
✓ QEC boundary is defined
✓ ZQN boundary is defined
✓ scheduling boundary is defined
✓ routing boundary is defined
✓ hardware boundary is defined
✓ resource boundary is defined
✓ capability boundary is defined
✓ compiler integration is defined
✓ runtime integration is defined
✓ AST contract is defined
✓ ZUIR/IR contract is defined
✓ interoperability is defined
✓ dialect integration is defined
✓ diagnostics are defined
✓ determinism is defined
✓ scalability is defined
✓ hard-coding policy is defined
✓ security policy is defined
✓ compatibility is defined
✓ testing contract is defined
✓ Rust 1.97/1.97.1 compatibility is defined
✓ unsafe Rust is prohibited
✓ feature lifecycle is defined
✓ file authority is defined
✓ integration ownership is defined

---

111. Final normative rule

The fundamental rule of the Zamani effect system is:

«An effect describes computational behavior, not the machine that happens to realize that behavior.»

Therefore:

Program semantics
       ↓
Effects
       ↓
Capabilities / requirements / constraints
       ↓
Semantic IR
       ↓
Optimization
       ↓
Routing / scheduling / resilience
       ↓
ZQN / HAL
       ↓
Actual hardware

not:

Program
 ↓
hard-coded hardware assumptions

A Zamani program must be capable of expressing its computational behavior once and allowing later compilation and execution layers to determine how that behavior is realized on the resources actually available.

That separation is a foundational requirement for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

while preserving the existing Zamani architecture, the canonical "quantum::ir" boundary, target-independent AST design, extensible domain model, and safe Rust implementation.

Normative status: This document is the effect-system semantic contract.
Syntax authority: the canonical modular grammar under "grammar/effects/", composed by "grammar/Zamani.g4".
Source-structure authority: the frontend AST.
Semantic authority: the semantic-analysis layer.
Quantum semantic authority: "quantum::ir".
Hardware realization authority: HAL/target/backend layers.
Runtime authority: runtime execution infrastructure.
Compatibility authority: "grammar/compatibility/".

No downstream implementation may reinterpret this separation merely to simplify one backend.