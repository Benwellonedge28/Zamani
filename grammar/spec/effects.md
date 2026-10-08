Zamani Effect System Specification

Path: "grammar/spec/effects.md"
Status: Normative production specification
Language: Zamani
Grammar technology: ANTLR 4
Rust implementation baseline: Rust 1.97 or later
Rust edition: 2021
Rust safety requirement: Safe Rust only; production Rust MUST use "#![forbid(unsafe_code)]"
Architectural model: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scope: Source-level effect syntax, effect semantics, effect inference, effect checking, effect polymorphism, effect handling, effect propagation, effect compatibility, and integration with the AST, semantic model, canonical IR, resources, capabilities, policies, contracts, provenance, compiler, runtime, quantum, classical, HDL, AI, data, distributed, networking, accelerator, interoperability, simulation, and future computational domains.

---

1. Purpose

This document defines the normative effect system of the Zamani programming language.

An effect describes computational behavior that is semantically relevant beyond the pure transformation of explicitly supplied values.

Examples include:

- input/output;
- state observation;
- state mutation;
- allocation;
- deallocation;
- randomness;
- nondeterminism;
- concurrency;
- synchronization;
- communication;
- networking;
- distributed execution;
- quantum operations;
- quantum measurement;
- hardware interaction;
- accelerator interaction;
- cryptographic operations;
- security-sensitive operations;
- foreign-function calls;
- native calls;
- reflection;
- metaprogramming;
- code generation;
- simulation;
- learning;
- adaptation;
- external-state interaction;
- persistence;
- time/environment observation;
- provenance generation;
- application-defined effects.

The effect system provides a portable semantic description of these behaviors.

It does not determine:

- which processor executes the operation;
- which accelerator executes it;
- which physical qubit is selected;
- which memory bank is selected;
- which FPGA fabric is used;
- which ASIC is selected;
- which cluster node executes a task;
- which network route is selected;
- which vendor API implements the operation;
- how resources are physically allocated;
- how scheduling is performed;
- how quantum routing is performed;
- how QEC is implemented;
- how ZQN is implemented;
- how the HAL communicates with hardware.

Those concerns belong to downstream semantic realization, resource analysis, compiler lowering, routing, scheduling, resilience, ZQN, HAL, runtime, and target-specific systems.

The effect system therefore forms a semantic contract between source-level intent and target-independent realization.

---

2. Normative terminology

The following distinctions are mandatory.

Concept| Meaning
Effect| What computational behavior a computation may perform or expose
Effect operation| A specific source-level operation associated with an effect
Effect set| A collection of effect identities
Capability| What an execution environment can provide
Resource| Something consumed, reserved, accessed, or otherwise relevant to realization
Requirement| A condition that must be satisfied
Constraint| A restriction on valid realization
Preference| A desired but non-mandatory realization property
Hint| Non-semantic implementation guidance
Policy| Rules governing permitted or preferred behavior
Contract| A semantic obligation such as requires, ensures, invariant, assume, guarantee, property, or assertion
Provenance| Information recording origin, derivation, transformation, evidence, verification, or decision history
Handler| Source-level mechanism for handling an effect
Target| A possible realization environment
Realization| Concrete mapping of semantic intent to a target
Lowering| Transformation from a higher semantic representation to a lower representation
Canonical IR| Target-independent compiler representation after semantic validation
"quantum::ir"| Canonical quantum IR boundary
Pure| A computation whose observable behavior is represented without effects
Effectful| A computation whose behavior includes one or more effects
Effect polymorphism| Abstraction over effect sets or effect variables
Effect inference| Determination of effects from program structure
Effect normalization| Canonicalization of effect identities and effect sets
Effect widening| Deliberately permitting a computation to expose a broader effect set
Effect narrowing| Restricting an implementation to a smaller declared effect set
Effect compatibility| Whether an actual effect set satisfies a declared effect contract
Effect handler| A construct that intercepts and processes specified effect operations
Dynamic effect| An effect whose concrete implementation is selected at runtime or late compilation
External effect| An effect crossing the program's ordinary semantic boundary
Target-specific effect| An explicitly non-portable effect defined by a target or dialect

---

3. Architectural authority

The effect system is distributed across several repository layers.

3.1 Normative semantic authority

This file:

grammar/spec/effects.md

owns the normative language-level effect semantics.

It defines:

- terminology;
- effect identity;
- effect sets;
- effect declarations;
- effect uses;
- effect operations;
- effect inference;
- effect propagation;
- effect polymorphism;
- effect compatibility;
- effect handlers;
- effect composition;
- effect normalization;
- effect diagnostics;
- effect safety;
- AST requirements;
- semantic-model requirements;
- IR requirements;
- downstream integration;
- conformance requirements.

This document does not replace grammar files.

---

3.2 Syntax authority

The modular grammar under:

grammar/effects/

owns effect syntax.

The existing canonical orchestrator is:

grammar/effects/effects.g4

It composes the effect grammar.

It MUST NOT become a semantic implementation.

---

3.3 Parser composition authority

The repository's parser composition root remains:

grammar/Zamani.g4

It MUST remain a composition root.

It MUST NOT independently redefine effect semantics.

---

3.4 Lexical authority

The canonical lexer remains:

grammar/antlr/ZamaniLexer.g4

with lexical ownership and token registry under:

grammar/lexer/

Effect syntax MUST consume canonical tokens.

Effect grammar files MUST NOT create competing lexical vocabularies.

---

3.5 Human specification authority

The broader normative language specifications live under:

grammar/specification/

They may define language-wide contracts that interact with effects.

When a language-wide specification and this file overlap, the specifications MUST be mutually consistent.

A contradiction MUST be resolved explicitly.

---

3.6 Machine-contract authority

Machine-readable contracts and conformance metadata belong under:

grammar/spec/

This document is the normative effect semantic contract within that layer.

---

4. Effect architecture

The effect architecture is:

                         Zamani Source
                              │
                              ▼
                           Lexer
                              │
                              ▼
                           Parser
                              │
                              ▼
                        Domain-neutral AST
                              │
                              ▼
                    Structural Validation
                              │
             ┌────────────────┼────────────────┐
             │                │                │
             ▼                ▼                ▼
           Types           Effects       Ownership/Control
             │                │                │
             └────────────────┼────────────────┘
                              │
                              ▼
                     Semantic Analysis
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
          ▼                   ▼                   ▼
      Capabilities         Resources           Policies
          │                   │                   │
          └───────────────────┼───────────────────┘
                              │
                              ▼
                         Contracts
                              │
                              ▼
                         Provenance
                              │
                              ▼
                  Canonical Semantic Model
                              │
                 ┌────────────┴────────────┐
                 │                         │
                 ▼                         ▼
           Classical IR                quantum::ir
                 │                         │
                 └────────────┬────────────┘
                              │
                              ▼
                         Optimization
                              │
                           Lowering
                              │
                     Routing / Scheduling
                              │
                    Resilience / Recovery
                              │
                           ZQN / HAL
                              │
                              ▼
                      Target Realization

The effect system primarily participates between:

AST
 ↓
effect analysis
 ↓
semantic model
 ↓
canonical IR

and provides metadata consumed by later stages.

---

5. Effect system boundary

The effect system MUST answer:

«What computational behavior is part of this program's semantic contract?»

It MUST NOT answer:

«Which physical machine realizes that behavior?»

For example:

effects {
    quantum::measurement
}

means that the computation includes quantum measurement behavior.

It does NOT mean:

use QPU 0
use physical qubit 7
use 32 qubits
use vendor X
use topology Y

Those belong to target realization.

---

6. Effect identity

An effect has a stable semantic identity.

Conceptually:

EffectIdentity =
    namespace
    +
    name
    +
    semantic identity/version information

Examples:

io::read
io::write
network::request
distributed::message
quantum::measurement
quantum::reset
hardware::signal
accelerator::compute
learning::update
adaptation::strategy
simulation::execute
foreign::call
native::call
security::authorize

The grammar MUST NOT enumerate all possible effect names.

The effect namespace is open-world.

A new effect identity MUST NOT require modification of the universal effect grammar merely because its name did not previously exist.

---

7. Qualified effect names

Effect references MUST use the repository's canonical naming system.

Examples:

io::read
quantum::measurement
distributed::consensus
accelerator::tensor
security::authorize
vendor::domain::effect
future::domain::operation

The effect grammar MUST reuse the canonical qualified-name rules.

It MUST NOT create an independent effect-specific naming system.

---

8. Effect declarations

An effect declaration introduces an effect interface.

Conceptually:

effect io::read;

or, where parameterization is supported:

effect transaction::operation<T>;

An effect declaration MAY contain:

- generic parameters;
- effect parameters;
- type parameters;
- return type information;
- attributes;
- metadata;
- documentation;
- declared semantic relationships.

An effect declaration MUST NOT contain target-specific allocation instructions.

Invalid semantic design:

effect quantum::operation<32_qubits>;

when "32_qubits" is merely a hardware capacity requirement.

That requirement belongs to the resource system.

---

9. Effect declaration ownership

The source grammar for effect declarations belongs to:

grammar/effects/effect-declarations.g4

This file owns syntax.

This specification owns meaning.

The Rust AST MUST represent effect declarations as semantic source constructs rather than as arbitrary strings.

The current frontend AST contains:

Statement::EffectDeclaration(Span, Identifier)

Therefore production implementation work MUST either:

1. extend that representation to preserve the complete declared effect structure; or
2. introduce a dedicated effect declaration node that preserves all normative source information.

The implementation MUST NOT silently discard:

- qualification;
- generic parameters;
- parameters;
- return type;
- attributes;
- source span;
- documentation/metadata where required for semantics.

---

10. Effect use

Effect declaration and effect use are distinct.

A declaration introduces an effect.

An effect use records that a computation performs or invokes effectful behavior.

Conceptually:

perform io::read(...)

or an equivalent syntax defined by the canonical effect-operation grammar.

The existing Rust AST contains:

Expression::Perform(Span, Box<Expression>)

This is currently insufficient to guarantee lossless representation of a structured effect operation unless the operand expression itself contains the required semantic information.

The production semantic pipeline MUST therefore preserve:

effect identity
operation identity
arguments
type information
source span
attributes
effects introduced
capabilities required
resources implicated
provenance

without depending on backend-specific representation.

---

11. Effect sets

An effect set represents the effects associated with a semantic context.

Conceptually:

effects {
    io::read,
    network::request,
    quantum::measurement
}

Effect sets have semantic set behavior.

Therefore:

effects {
    io::read,
    quantum::measurement
}

and:

effects {
    quantum::measurement,
    io::read
}

represent the same semantic set after normalization.

Source ordering MAY be retained for diagnostics and source fidelity.

Semantic equality MUST NOT depend on source ordering.

---

12. Empty effect sets

An empty effect set is valid:

effects {}

An omitted effect declaration and an explicit empty effect set MAY have different source-level meanings.

The AST MUST preserve enough information to distinguish them where the language specification requires that distinction.

An empty effect set MUST NOT automatically mean:

pure

unless the language-wide semantic specification explicitly establishes that equivalence.

---

13. Duplicate effects

Duplicate effect identities MAY appear syntactically.

For example:

effects {
    io::read,
    io::read
}

Semantic normalization MUST remove duplicate identities unless an explicitly defined effect system introduces multiplicity.

Multiplicity MUST NOT be inferred merely from duplicate spelling.

---

14. Effect cardinality

There is no universal source-language maximum for:

- number of effects;
- number of effect references;
- number of effect declarations;
- number of effect handlers;
- number of effect operations;
- effect-set nesting;
- effect-name depth;
- effect parameter count.

The language MUST NOT define constants such as:

MAX_EFFECTS
MAX_EFFECT_SET_SIZE
MAX_EFFECT_OPERATIONS
MAX_EFFECT_HANDLERS
MAX_EFFECT_PARAMETERS
MAX_EFFECT_DEPTH

Compiler, parser, runtime, operating-system, or deployment limits MAY exist.

Such limits MUST be implementation/resource policies and MUST NOT change the language's semantic model.

---

15. Practical scalability

"Unlimited" or "infinity" in this specification means:

«The language architecture imposes no artificial finite machine-size ceiling where the semantics themselves do not require one.»

Actual execution is necessarily bounded by available:

- memory;
- processing capacity;
- storage;
- network capacity;
- compiler resources;
- runtime resources;
- target capabilities;
- time;
- physical constraints;
- deployment policies.

These are realization constraints, not effect-language limits.

---

16. Effect inference

The compiler SHOULD infer effects wherever the language's type and semantic systems permit.

For example:

fn read_value() {
    perform io::read(...)
}

may infer:

io::read

without requiring the programmer to repeat it manually.

Inference MUST be conservative.

If a function can perform an effect, the inferred effect set MUST include that effect unless the language has a sound mechanism for proving it unreachable.

Inference MUST account for:

- direct effect operations;
- function calls;
- method calls;
- closures;
- generic instantiations;
- asynchronous operations;
- spawned tasks;
- handlers;
- foreign calls;
- native calls;
- reflection;
- metaprogramming;
- generated code;
- conditional branches;
- pattern matching;
- exception/error paths;
- distributed execution;
- quantum operations;
- simulation;
- learning;
- adaptation.

---

17. Effect inference soundness

The implementation MUST NOT infer a smaller effect set than is semantically possible.

If:

A -> B

and "B" can perform effect "E", then "A" MUST account for "E" unless:

- "A" handles "E";
- "A" transforms "E" into another explicitly represented semantic behavior;
- a sound effect-elimination proof exists;
- the operation is semantically unreachable.

Compiler optimization MUST NOT remove required effect metadata merely because the selected target happens not to expose that effect directly.

---

18. Effect propagation

Effects propagate through computation boundaries.

For a call:

f()

the caller inherits effects of "f" unless the call is:

- handled;
- transformed;
- statically proven effect-free;
- replaced by an observationally equivalent pure computation.

For sequential composition:

A;
B;

the resulting effect set is the semantic union of the effects of "A" and "B".

For branching:

if condition {
    A
} else {
    B
}

the enclosing effect set MUST conservatively account for effects reachable from either branch unless path-sensitive analysis proves a stronger result.

---

19. Effect propagation through loops

For:

while condition {
    body
}

the enclosing effect set MUST include effects of the condition and body that may execute.

Loop iteration count MUST NOT be represented as an effect count.

There is no language-level maximum number of iterations.

---

20. Effect propagation through asynchronous execution

Existing asynchronous constructs remain owned by the concurrency/asynchronous grammar.

Effect semantics MUST nevertheless account for:

- task creation;
- task execution;
- awaiting;
- cancellation;
- synchronization;
- communication;
- shared-state access;
- failure propagation.

An asynchronous function MUST NOT silently lose effects because execution is deferred.

---

21. Effect propagation through spawned computation

For:

spawn computation

the semantic model MUST record that the spawned computation may execute independently.

The effect system MUST distinguish:

- effects of spawning;
- effects of the spawned computation;
- effects required for synchronization;
- effects observed by the parent;
- effects handled inside the child.

The concurrency subsystem owns scheduling and lifecycle.

The effect system owns semantic effect information.

---

22. Effect propagation through handlers

An effect handler may intercept effects.

Given:

handle computation {
    ...
}

the handler may transform:

effect E

into another semantic behavior.

The enclosing computation MUST expose:

unhandled effects

rather than simply copying all effects blindly.

A handler MUST NOT be assumed to eliminate an effect unless its semantic contract establishes that the effect is fully handled.

---

23. Effect handlers

The source syntax for handlers belongs to:

grammar/effects/effect-handling.g4

The handler grammar owns:

- handler structure;
- handler arms;
- handler patterns;
- guards;
- handler bodies;
- source-level handling constructs.

This specification owns their semantic meaning.

A handler MUST specify, directly or through its semantic structure:

- what effect identity it handles;
- what operation it handles;
- its arguments;
- its continuation behavior where applicable;
- its result;
- any effects introduced by the handler;
- any capabilities required;
- any resources implicated;
- provenance requirements.

---

24. Handler isolation

A handler MUST NOT silently acquire authority merely because it handles an effect.

For example, handling:

network::request

does not automatically grant:

network::admin
filesystem::write
native::execute

Capabilities remain separately checked.

---

25. Handler resource semantics

A handler MAY require resources.

For example:

effect simulation::execute

may be handled by a simulator requiring memory or accelerator capacity.

The handler does not redefine resource semantics.

The resource subsystem remains authoritative.

The pipeline is:

effect
  ↓
handler
  ↓
capability/resource requirements
  ↓
semantic validation
  ↓
realization

---

26. Effect polymorphism

Zamani SHOULD support effect-polymorphic functions and types.

Conceptually:

fn compute<E>(value: T) effects { E } -> R

The exact syntax is owned jointly by:

grammar/types/
grammar/functions/
grammar/effects/

This specification defines the semantics.

An effect variable represents an abstraction over computational behavior.

It MUST NOT represent:

- a CPU identifier;
- a GPU identifier;
- a QPU identifier;
- a physical qubit;
- a memory bank;
- a cluster node;
- a target-specific device handle.

---

27. Effect bounds

Effect variables MAY have bounds.

Conceptually:

E: effect

or:

E: io

depending on the final type/effect syntax.

Effect bounds MUST describe semantic behavior.

They MUST NOT encode fixed physical capacities.

---

28. Effect substitution

When an effect-polymorphic function is instantiated, effect variables MUST be substituted consistently.

For example:

compute<E>

instantiated with:

E = { io::read, network::request }

must produce the corresponding concrete semantic effect set.

Substitution MUST preserve:

- effect identity;
- effect parameters;
- source provenance;
- capability relationships;
- resource relationships;
- policy constraints.

---

29. Effect compatibility

A declared effect contract is compatible with an implementation when the implementation's actual effects satisfy the declared contract.

For a declaration:

effects { E1, E2 }

the implementation MUST NOT perform an undeclared effect "E3" unless the language explicitly permits effect widening.

Undeclared effects MUST produce a diagnostic when the declaration is closed.

---

30. Open and closed effect contracts

The language MUST distinguish conceptually between:

closed effect set

and:

open effect set

A closed set means:

«No additional effects are permitted.»

An open effect set means:

«Additional effects may be introduced subject to the applicable effect variable, bound, policy, or contract.»

The exact surface syntax belongs to the effect grammar.

The semantic model MUST preserve the distinction.

---

31. Effect widening

Effect widening means allowing an operation to expose additional effects.

Widening MUST be explicit or justified by an open effect contract.

It MUST NOT happen silently.

Widening affects:

- callers;
- API contracts;
- type/effect checking;
- optimization;
- reproducibility;
- policy analysis;
- provenance.

---

32. Effect narrowing

An implementation may expose fewer effects than a declaration permits.

This is valid when the declared effect contract is an upper bound.

Example:

declared:
effects { io::read, io::write }

implementation:

effects { io::read }

may be valid if the language's contract semantics define declarations as upper bounds.

The implementation MUST follow the chosen language-wide variance rules consistently.

---

33. Effect subtyping

Effect subtyping, if implemented, MUST be defined independently from ordinary type subtyping.

The semantic relation MUST be explicit.

An implementation MUST NOT infer:

effect A <: effect B

merely because names share a prefix.

For example:

network::read

is not automatically a subtype of:

network

unless such a relationship is explicitly registered in the effect semantic model.

---

34. Effect hierarchy

The effect namespace MAY represent hierarchical relationships.

Example:

io::read
io::write
io::filesystem
io::device

Hierarchy is semantic metadata.

It MUST NOT require one grammar production per effect.

---

35. Effect aliases

Effect aliases MAY exist.

An alias maps one semantic identity to another.

Alias resolution MUST occur during semantic analysis.

The canonical effect identity MUST be retained after resolution.

Aliases MUST NOT create infinite recursive resolution.

Cycles MUST produce deterministic diagnostics.

---

36. Effect composition

Effect composition is semantic union plus any explicitly defined transformation.

The implementation MUST distinguish:

union

from:

handler elimination

and:

effect transformation

and:

effect masking

These are not interchangeable.

---

37. Effect transformation

A handler or semantic adapter MAY transform one effect into another.

Example:

quantum::measurement

may be transformed into:

classical::observation

after measurement.

The semantic model MUST preserve the transformation.

It MUST NOT claim that the original effect never occurred.

This is important for:

- provenance;
- auditing;
- quantum-classical interaction;
- reproducibility;
- security;
- explanation.

---

38. Effect masking

Effect masking is permitted only when the semantic model establishes that the masked effect is no longer observable outside the handling boundary.

Masking MUST NOT hide effects merely for convenience.

In particular:

native::call

MUST NOT be masked simply because the called function appears to return an ordinary value.

The external behavior remains effectful unless proven otherwise.

---

39. Effect purity

A computation is pure only when its observable semantics contain no effects outside the language's defined pure computation model.

Pure computation MUST NOT:

- read external state;
- mutate external state;
- observe nondeterministic environment state;
- perform I/O;
- perform network communication;
- perform uncontrolled randomness;
- perform external measurement;
- invoke unmodeled foreign behavior;
- perform uncontrolled reflection;
- perform uncontrolled code generation;
- mutate shared state outside its semantic ownership.

---

40. Purity is not hardware independence

A pure computation can still be executed on:

- CPU;
- GPU;
- FPGA;
- ASIC;
- accelerator;
- simulator;
- distributed system.

Hardware placement does not itself determine whether a computation is pure.

Conversely, a computation may be hardware-independent but still effectful.

---

41. Allocation effects

Allocation and deallocation MAY be represented as effects when they are semantically relevant.

The effect system MUST distinguish:

allocation behavior

from:

memory resource requirement

For example:

memory::allocate

describes behavior.

A resource requirement such as:

requires memory >= required_memory

describes feasibility.

They MUST NOT be conflated.

---

42. State effects

State effects describe observation or mutation of semantic state.

Examples:

state::read
state::write
state::atomic
state::transaction

State identity is semantic.

Physical storage location is downstream.

---

43. Mutation effects

Mutation is an effect when it changes observable program state.

The effect system MUST integrate with:

grammar/memory/
grammar/types/
grammar/concurrency/
grammar/validation/

Mutation analysis MUST respect:

- ownership;
- borrowing;
- aliasing;
- linearity;
- affinity;
- synchronization;
- concurrency semantics.

---

44. Randomness

Randomness is an effect.

Examples:

randomness::source
randomness::sample
randomness::entropy

The effect model MUST distinguish:

- deterministic pseudorandom computation;
- external entropy;
- nondeterministic sampling;
- cryptographic randomness;
- quantum randomness.

The exact implementation remains downstream.

---

45. Reproducibility

A computation requiring randomness MAY still be reproducible if the language explicitly supplies a deterministic seed or equivalent controlled source.

The effect metadata MUST preserve the distinction between:

randomness present

and:

uncontrolled nondeterminism

Reproducibility analysis belongs jointly to:

effects
execution
compile
provenance
compatibility

---

46. Nondeterminism

Nondeterminism is distinct from randomness.

Nondeterminism may arise from:

- concurrent scheduling;
- distributed message ordering;
- race-free but nondeterministic execution;
- unspecified ordering;
- external environment behavior;
- target-dependent realization.

The effect model MUST permit the semantic representation of nondeterminism.

It MUST NOT incorrectly classify all nondeterminism as randomness.

---

47. Time and environment effects

Observation of external time or environment state is effectful.

Examples:

environment::time
environment::clock
environment::configuration
environment::identity
environment::locale

Such effects MUST NOT be silently treated as pure constants unless compile-time evaluation is explicitly proven valid.

---

48. I/O effects

I/O belongs to the effect system.

Examples:

io::read
io::write
io::open
io::close
io::flush

The effect system describes the semantic behavior.

It does not select:

- a filesystem;
- a device;
- a storage controller;
- a kernel API;
- a hardware device.

Those belong to realization.

---

49. Network effects

Networking is effectful.

Examples:

network::connect
network::send
network::receive
network::request
network::stream

Networking effects MUST integrate with:

grammar/networking/
grammar/security/
grammar/resources/
grammar/policies/
grammar/distributed/

Network capability and resource requirements are separate from network effects.

---

50. Distributed effects

Distributed computation introduces effects such as:

distributed::send
distributed::receive
distributed::commit
distributed::consensus
distributed::coordination
distributed::checkpoint

These describe semantic distributed behavior.

They do not determine:

- node count;
- node identity;
- cluster topology;
- transport vendor;
- physical network;
- scheduler.

---

51. Concurrency effects

Concurrency-related behavior MAY include:

concurrency::spawn
concurrency::join
concurrency::synchronize
concurrency::communicate
concurrency::atomic

The effect system MUST integrate with:

grammar/concurrency/

The concurrency subsystem owns execution constructs.

The effect system owns their semantic effect annotations.

---

52. Foreign-function effects

Foreign calls are effectful by default unless their foreign contract explicitly establishes a soundly modeled effect set.

The interoperability subsystem owns syntax for:

grammar/interoperability/

The effect system owns the semantic requirement that foreign calls participate in effect analysis.

Foreign calls MUST carry sufficient metadata to determine:

- declared effects;
- calling boundary;
- capability requirements;
- resource requirements;
- ABI assumptions;
- provenance;
- failure behavior.

---

53. Native effects

Native execution is effectful by default.

A native function MUST NOT be assumed pure merely because it has a pure-looking signature.

A native implementation may:

- access memory;
- access devices;
- perform I/O;
- access time;
- mutate global state;
- use randomness;
- communicate;
- invoke other native operations.

Its semantic contract MUST therefore be explicitly declared or conservatively classified.

---

54. Reflection effects

Reflection may expose or modify program structure.

The effect system MUST distinguish:

reflection::inspect
reflection::modify
reflection::invoke

where such distinctions are supported.

Inspection MAY be less effectful than mutation.

The implementation MUST NOT grant mutation authority merely because inspection is permitted.

---

55. Metaprogramming effects

Compile-time computation is still computation.

The effect system MUST distinguish:

compile-time effect

from:

runtime effect

Compile-time execution MUST NOT silently access runtime-only capabilities.

Examples requiring explicit semantic treatment include:

- filesystem access;
- environment inspection;
- network access;
- code generation;
- external process invocation;
- nondeterministic data.

---

56. Code-generation effects

Code generation is effectful when it changes generated program artifacts or compilation behavior.

Example:

code_generation::emit

Generated code MUST be subject to its own:

- parsing;
- validation;
- type checking;
- effect checking;
- capability checking;
- policy checking;
- provenance tracking.

Generated code MUST NOT bypass language safety checks merely because it was generated.

---

57. Simulation effects

Simulation may itself be effectful when it:

- creates simulation state;
- consumes simulation resources;
- accesses external models;
- records simulation output;
- interacts with external simulation infrastructure.

Simulation MUST remain an execution strategy.

It MUST NOT silently become a different programming language.

---

58. Learning effects

Learning operations are effectful when they modify:

- model parameters;
- learned state;
- persistent training state;
- external datasets;
- adaptive strategies.

Examples:

learning::train
learning::update
learning::adapt
learning::feedback

The effect system MUST integrate with:

grammar/ai/
grammar/data/
grammar/execution/
grammar/policies/
grammar/provenance/

Learning algorithms MUST remain extensible.

The universal effect grammar MUST NOT enumerate every machine-learning algorithm.

---

59. Adaptation effects

Adaptation is an effect.

Examples:

adaptation::select
adaptation::update
adaptation::strategy
adaptation::configuration

Adaptation MUST be controlled by:

- policy;
- authorization;
- capabilities;
- effects;
- resources;
- contracts;
- provenance.

Adaptation MUST NOT mean unrestricted self-modifying code.

---

60. Adaptation safety

An adaptive computation MUST declare or infer:

what may change
why it may change
who/what authorizes the change
what capabilities are required
what resources are required
what effects may result
how the change is recorded
how the resulting state is validated

The effect system records the behavioral aspect.

Policy and security systems determine whether the behavior is permitted.

---

61. Quantum effects

Quantum effects are ordinary semantic effects identified through the open-world namespace.

Examples:

quantum::prepare
quantum::operate
quantum::measure
quantum::reset
quantum::entangle
quantum::dynamic_control
quantum::mid_circuit_measurement
quantum::logical_operation

The effect grammar MUST NOT enumerate a fixed universal quantum gate set.

---

62. Quantum effect boundary

Quantum effect syntax MUST NOT directly encode:

- physical qubit IDs;
- physical topology;
- calibration;
- pulse schedules;
- vendor-specific gate implementations;
- QEC decoder selection;
- physical routing;
- physical placement.

Those belong downstream.

The semantic pipeline is:

quantum effect
      ↓
quantum semantic model
      ↓
quantum::ir
      ↓
optimization
      ↓
routing
      ↓
scheduling
      ↓
resilience / QEC
      ↓
ZQN
      ↓
HAL

---

63. Quantum measurement

Measurement is explicitly effectful.

A measurement may produce classical information and introduce effects such as:

quantum::measurement
classical::observation

The semantic model MUST preserve the fact that measurement occurred.

Optimization MUST NOT erase measurement semantics merely because a later target represents measurement differently.

---

64. Quantum-classical interaction

Hybrid programs may transform:

classical → quantum
quantum → classical

The effect system MUST preserve both sides.

Example:

quantum::measurement

may produce classical control information.

This MUST NOT be represented as if the quantum effect never happened.

---

65. HDL and hardware effects

Hardware interaction MAY be represented through effects such as:

hardware::signal
hardware::register
hardware::memory
hardware::device
hardware::clock
hardware::io

The effect model MUST remain independent of a particular HDL or hardware family.

Hardware-specific realization belongs to:

grammar/hdl/
grammar/hardware/
HAL
backend
target dialect

---

66. Accelerator effects

Accelerator interaction MAY be represented by effects such as:

accelerator::compute
accelerator::transfer
accelerator::synchronize

The effect describes interaction.

Capability/resource systems determine whether a target provides the accelerator capability.

---

67. AI and symbolic reasoning

Reasoning operations may be effect-free or effectful depending on their inputs and semantics.

For example:

infer
deduce
reason

over immutable local values may be pure.

Reasoning that accesses:

- external knowledge;
- network data;
- mutable knowledge stores;
- learned state;
- nondeterministic models;

must carry the corresponding effects.

The effect system therefore does not classify all reasoning as inherently effectful.

---

68. Knowledge operations

Knowledge operations such as:

assert
retract
query

MUST participate in effect analysis when they access or mutate persistent/shared knowledge.

For example:

query knowledge

may carry:

knowledge::read

while:

assert fact

may carry:

knowledge::write

The exact effect identity is semantic metadata, not a fixed grammar enumeration.

---

69. Contracts and effects

Contracts interact with effects.

A contract MUST be able to express conditions involving effectful behavior without making the contract evaluator itself accidentally effectful.

For example:

requires capability("quantum.measurement")

is a requirement on realization.

It is not itself equivalent to:

quantum::measurement

Similarly:

ensures result.valid

is a contract.

It is not an effect.

---

70. Policies and effects

Policies govern effects.

A policy MAY:

- permit an effect;
- prohibit an effect;
- require a capability;
- require provenance;
- restrict a handler;
- restrict adaptation;
- restrict native execution;
- restrict network access;
- restrict reflection;
- constrain randomness;
- constrain distribution.

Policy semantics belong to:

grammar/policies/
grammar/security/

Effect semantics must remain policy-neutral except for explicit policy integration points.

---

71. Capabilities and effects

An effect does not imply a capability.

For example:

quantum::measurement

does not imply that the target has:

capability("quantum.measurement")

The compiler determines whether the effect can be realized.

The capability system determines whether the target can provide the required capability.

---

72. Resources and effects

Effects may imply resource consequences.

For example:

quantum::measurement

may require quantum resources.

But the effect itself MUST NOT encode physical resource quantities.

The separation is:

effect:
    quantum::measurement

capability:
    quantum.measurement

resource:
    target-dependent quantum resources

requirement:
    required quantum capability/resources

realization:
    actual target mapping

---

73. Effect/resource non-equivalence

The following MUST remain distinct:

effect

and:

resource requirement

Therefore:

effect quantum::compute

does not mean:

requires qubits >= fixed_number

The latter belongs to resource semantics.

---

74. Effect/capability non-equivalence

The following are distinct:

effect quantum::measurement

and:

capability("quantum.measurement")

The first describes program behavior.

The second describes target capability.

---

75. Effect/policy non-equivalence

The following are distinct:

effect network::request

and:

policy {
    allow network::request
}

The effect describes what happens.

The policy describes whether that behavior is permitted.

---

76. Effect/provenance integration

Effect analysis MUST participate in provenance.

The compiler SHOULD be able to record:

effect introduced by source
effect inferred from call
effect propagated from dependency
effect transformed by handler
effect removed by verified optimization
effect required by generated code
effect associated with foreign call
effect associated with target realization

Provenance belongs to the universal provenance subsystem.

---

77. Effect provenance identity

An inferred effect SHOULD retain enough provenance to answer:

«Why does this computation have this effect?»

Possible provenance sources include:

direct_source
called_function
generic_instantiation
handler
generated_code
foreign_declaration
native_declaration
dialect
inference_rule
semantic_lowering

---

78. Effect diagnostics

Effect diagnostics MUST identify:

- effect identity;
- source location;
- declared effect set;
- actual/inferred effect set;
- reason for mismatch;
- relevant call chain where available;
- handler information where applicable;
- policy conflict where applicable;
- capability conflict where applicable.

Diagnostics SHOULD be deterministic.

---

79. Effect mismatch

A mismatch occurs when:

actual_effects

are incompatible with:

declared_effects

The diagnostic MUST distinguish:

missing declaration

from:

forbidden effect

from:

unknown effect

from:

unresolved effect

from:

policy violation

from:

capability infeasibility

from:

resource infeasibility

These are different failures.

---

80. Unknown effect

An unknown effect identity MUST NOT automatically be treated as harmless.

A compiler MAY support open-world effect names, but before execution or stable lowering the semantic system MUST determine whether the effect is:

- declared;
- imported;
- provided by a dialect;
- provided by a standard library;
- provided by a target contract;
- explicitly permitted as an open extension.

Otherwise compilation MUST produce an appropriate diagnostic.

---

81. Unresolved effect

An unresolved effect is distinct from an unknown effect.

An effect may be known syntactically but unresolved because:

- a module is unavailable;
- a dialect is not loaded;
- a version is incompatible;
- an alias cannot be resolved;
- an effect provider is missing.

This distinction MUST be preserved for diagnostics and tooling.

---

82. Effect versioning

Effect identities MAY carry semantic versions or version constraints.

Version information MUST NOT be confused with language version.

The compatibility system MUST distinguish:

language version
grammar version
effect definition version
dialect version
AST version
IR version
target capability version

---

83. Effect compatibility

An effect definition change MUST preserve compatibility according to the repository compatibility policy.

A change may be:

additive
compatible
conditionally compatible
breaking
deprecated
removed

Changing the semantic meaning of an existing effect without a compatibility mechanism is a breaking change.

---

84. Effect deprecation

Effects MAY be deprecated.

A deprecated effect:

- remains resolvable during its compatibility window;
- produces an appropriate diagnostic;
- has a migration path;
- retains provenance;
- does not silently change meaning.

---

85. Effect extension

New effect domains SHOULD normally be introduced through:

effect identity
effect metadata
capability metadata
resource metadata
policy metadata
semantic adapter
tests

A new effect MUST NOT require a new universal keyword if an identifier can represent it.

This keeps the language open-ended.

---

86. Effect dialects

Domain-specific effect syntax MAY be supplied through dialects.

A dialect MUST define:

- dialect identity;
- version;
- syntax;
- lexer requirements;
- AST mapping;
- effect mapping;
- capability mapping;
- resource mapping;
- policy mapping;
- provenance requirements;
- compatibility;
- tests.

Dialect effects MUST eventually map into the canonical effect semantic model.

---

87. Vendor extensions

Vendor-specific effects MAY exist.

Example:

vendor::accelerator::special_operation

Vendor extensions MUST NOT redefine the meaning of standard effects.

Vendor effects MUST be isolated through:

- dialect metadata;
- capability contracts;
- interoperability;
- target-specific backend support.

---

88. Effect handlers and continuations

If handlers expose continuations, the continuation itself MUST have an effect contract.

The continuation may:

- perform additional effects;
- return a value;
- fail;
- invoke another handler;
- perform asynchronous work.

The handler grammar owns syntax.

The semantic model owns continuation effect checking.

---

89. Handler effect safety

A handler MUST NOT accidentally hide effects performed by its continuation.

The compiler MUST calculate:

handler effects
+
continuation effects
+
unhandled effects

according to the language's handler semantics.

---

90. Effect recursion

Recursive functions may recursively propagate effects.

The effect analyzer MUST converge.

It MUST NOT recursively expand an effect forever.

Recursive effect analysis SHOULD use a fixed-point algorithm over canonical effect identities.

The fixed point MUST be deterministic.

---

91. Effect analysis algorithmic contract

The semantic effect analyzer SHOULD conceptually perform:

1. resolve effect identities
2. resolve declarations
3. construct effect variables
4. collect direct effects
5. collect call dependencies
6. propagate effects
7. apply handlers
8. apply effect substitutions
9. normalize effects
10. apply policy constraints
11. apply capability/resource relationships
12. validate declared contracts
13. emit diagnostics
14. produce canonical semantic effect metadata

The implementation may use a different internal algorithm provided the observable semantics are equivalent.

---

92. Effect normalization

Normalization MUST be deterministic.

At minimum it SHOULD:

1. resolve aliases;
2. resolve qualified names;
3. canonicalize identity;
4. normalize parameters;
5. remove duplicate set members;
6. normalize nested effect expressions;
7. normalize effect-variable substitutions;
8. preserve required provenance.

Normalization MUST NOT discard semantic distinctions.

---

93. Canonical effect ordering

The semantic effect set MAY be stored in a deterministic canonical order.

The canonical order MUST be defined by semantic identity, not source formatting.

This is important for:

- reproducible builds;
- stable hashing;
- deterministic diagnostics;
- incremental compilation;
- caching;
- IR serialization;
- provenance.

---

94. Effect hashing

If effects participate in semantic hashes, cache keys, or artifact identity, the hash MUST include all semantically relevant information.

It MUST NOT include irrelevant source formatting.

The hash MUST distinguish effects whose parameters have different semantics.

---

95. Effect equality

Two effects are semantically equal when their normalized identities and all semantically relevant parameters are equal.

String similarity is insufficient.

For example:

vendor::x::read

and:

io::read

are not equal merely because both contain "read".

---

96. Effect operations

Effect operations represent executable behavior associated with an effect.

An operation SHOULD contain:

effect identity
operation identity
parameters
arguments
result types
attributes
source span
provenance

The exact syntax is owned by the modular effect-operation grammar.

---

97. Effect operation dispatch

The parser MUST NOT dispatch effect operations.

Semantic analysis resolves the operation.

Compiler lowering selects a realization.

Runtime dispatch occurs only after semantic lowering.

---

98. Effect operation extensibility

New effect operations MUST be representable without adding universal grammar alternatives whenever their syntax fits the generic operation form.

For example:

quantum::measurement
learning::update
adaptation::select
network::request

may be represented by the same generic semantic structure.

---

99. Effect operations and target specialization

An operation may be specialized for a target.

Specialization MUST preserve the source effect contract.

A target-specific implementation MUST NOT silently introduce semantically observable effects that violate the source contract.

If specialization adds an effect, the compiler MUST either:

- reject it;
- expose it through effect widening;
- prove it unobservable;
- route it through an explicit effect transformation.

---

100. Optimization and effects

Optimization MUST preserve effect semantics.

The following transformations require effect-aware validation:

- constant folding;
- common-subexpression elimination;
- dead-code elimination;
- inlining;
- loop transformations;
- vectorization;
- parallelization;
- distribution;
- fusion;
- specialization;
- hardware lowering;
- quantum circuit optimization;
- simulation substitution.

---

101. Dead-code elimination

A computation MUST NOT be removed merely because its returned value is unused if it has observable effects.

For example:

perform io::write(...)

cannot be removed solely because its result is ignored.

An operation may be removed only when its effects are proven irrelevant under the language's semantic rules.

---

102. Common-subexpression elimination

A computation with effects MUST NOT automatically be treated as referentially transparent.

For example:

read_time()

cannot be duplicated or eliminated as if it were a constant unless its effect contract permits such transformation.

---

103. Inlining

Inlining MUST preserve:

- effect identity;
- effect propagation;
- handler behavior;
- capability requirements;
- resource implications;
- provenance where required.

Inlining MUST NOT change the externally visible effect contract.

---

104. Parallelization

Parallelization may introduce or transform concurrency effects.

The compiler MUST preserve semantic equivalence.

It MUST account for:

- synchronization;
- ordering;
- shared-state access;
- communication;
- nondeterminism;
- resource constraints.

---

105. Distribution

Distribution may transform local effects into distributed effects.

For example:

state::read

may require a distributed consistency mechanism after lowering.

The compiler MUST preserve the original semantic intent and record the transformation.

---

106. Quantum optimization

Quantum optimization MUST preserve quantum effect semantics.

It MUST NOT:

- remove required measurement;
- change measurement ordering;
- change externally observable randomness;
- introduce uncontrolled classical effects;
- change required synchronization;

unless the transformation is proven semantically equivalent.

---

107. HDL optimization

HDL optimization MUST preserve hardware-visible semantic effects such as:

- signal behavior;
- observable timing contracts where specified;
- state transitions;
- memory interactions;
- synchronization;
- external interfaces.

Target-specific optimization remains downstream.

---

108. Effect and determinism

Effect analysis itself MUST be deterministic.

Given identical:

- source;
- language version;
- dialect set;
- effect registry;
- semantic environment;
- compiler configuration;

the effect analysis result MUST be identical.

---

109. Effect and reproducible compilation

Effect metadata MUST participate in reproducible compilation when it influences:

- semantic validity;
- optimization;
- target selection;
- capability negotiation;
- resource planning;
- policy decisions;
- generated artifacts.

---

110. Effect and incremental compilation

Effect analysis SHOULD support incremental compilation.

A change to one function's effects SHOULD invalidate only semantic dependents whose effect contracts depend on the changed information.

Effect fingerprints MAY be used for dependency tracking.

---

111. Effect and caching

Compiler caches MUST distinguish semantically different effect environments.

A cached result from:

effects { io::read }

MUST NOT be reused for:

effects { io::read, network::request }

unless the cached artifact is proven valid for the broader effect environment.

---

112. Effect and modules

Effect declarations are module-visible semantic entities.

The module system MUST support:

- declaration;
- import;
- export;
- visibility;
- aliasing;
- versioning;
- compatibility.

Effect names MUST obey the canonical module/name-resolution system.

---

113. Effect and generics

Generic functions MAY depend on effect variables.

Effect substitution MUST occur consistently with type substitution.

A generic instantiation MUST NOT lose effects because the generic implementation is instantiated in a different domain.

---

114. Effect and traits/type classes

Where traits/type classes exist, effect requirements MAY form part of method contracts.

An implementation MUST satisfy the effect contract of the abstraction it implements.

An implementation MUST NOT silently acquire prohibited effects.

---

115. Effect and linear/affine types

Linear and affine semantics MAY affect effect behavior.

For example:

linear resource acquisition

may introduce allocation/release effects.

The effect system MUST cooperate with ownership analysis.

It MUST NOT independently redefine linearity.

---

116. Effect and dependent types

Dependent types MAY cause effect-relevant predicates to depend on values or types.

The effect analyzer MUST preserve soundness.

A dependent expression MUST NOT be assumed pure merely because its type-level representation appears static.

---

117. Effect and contracts

The effect checker MUST run consistently with:

requires
ensures
invariant
assume
guarantee
property
assert

A contract MUST NOT be used to erase an effect unless the contract system provides a formally valid semantic proof mechanism.

---

118. Effect and assumptions

An "assume" construct may permit conditional effect reasoning.

An assumption MUST be recorded in semantic provenance.

If an assumption cannot be proven or is invalidated, the compiler/runtime MUST follow the contract semantics rather than silently trusting it.

---

119. Effect and guarantees

A guarantee MAY constrain allowed effects.

For example, a computation may guarantee:

no network communication

The semantic effect checker MUST validate that guarantee.

---

120. Effect and properties

A property may constrain an effect set.

Examples include:

property deterministic
property no_external_io
property no_native_calls

The exact property system belongs to validation/specification.

Effect analysis supplies the semantic evidence.

---

121. Effect and security

Security-sensitive effects SHOULD include explicit security metadata.

Examples:

security::authorize
security::credential
security::secret
security::privileged

The effect system MUST NOT itself implement authorization.

Authorization belongs to:

grammar/security/
grammar/policies/
runtime security

---

122. Effect and sandboxing

A sandbox MAY prohibit effects.

For example:

forbid effect network::request

The sandbox system owns the policy.

The effect analyzer supplies the effect facts.

This creates:

effect analysis
      ↓
sandbox policy evaluation
      ↓
accept / reject / constrain

---

123. Effect and provenance requirements

Some effects MAY require provenance.

Examples:

learning::update
adaptation::strategy
code_generation::emit
foreign::call
native::call
security::decision

Whether provenance is mandatory is controlled by policy and effect metadata.

---

124. Effect and explainability

The compiler SHOULD be able to explain effect decisions.

For example:

function f has effect network::request because:
    f calls g
    g calls h
    h performs network::request

This explanation is semantic provenance.

It is not merely a textual compiler diagnostic.

---

125. Effect and evidence

Effect analysis may produce evidence such as:

direct declaration
direct operation
call graph path
verified optimization
handler elimination
policy validation

Evidence MAY be consumed by:

- contracts;
- provenance;
- security;
- diagnostics;
- tooling.

---

126. Effect and policy enforcement

Policy enforcement MUST distinguish:

effect exists

from:

effect is permitted

A program may validly contain:

network::request

but be rejected by a policy forbidding network effects.

This is a policy failure, not a grammar failure.

---

127. Effect and capability negotiation

Capability negotiation occurs after semantic effect determination.

The sequence is:

effect required
      ↓
capability requirement derived
      ↓
target capabilities inspected
      ↓
feasibility determined
      ↓
realization selected

The parser MUST NOT perform capability negotiation.

---

128. Effect and resource negotiation

The same separation applies to resources:

effect
      ↓
resource implications
      ↓
requirements
      ↓
target feasibility
      ↓
realization

The effect grammar MUST NOT allocate resources.

---

129. Effect and target selection

Effects may influence target selection.

For example:

quantum::measurement

may eliminate targets that cannot realize quantum measurement.

The compiler MAY select a simulator when simulation is explicitly permitted by policy.

It MUST NOT silently replace a required physical computation with simulation when that changes program semantics.

---

130. Effect and simulation fallback

Simulation is not automatically equivalent to physical execution.

A policy MAY explicitly allow:

physical execution
or
simulation

The provenance system MUST record the realization mode where required.

---

131. Effect and resilience

Runtime resilience may retry or recover effectful operations.

The semantic effect system MUST distinguish:

retrying an effect

from:

performing a different effect

Retries MUST obey the operation's idempotency/transaction contract where applicable.

---

132. Effect and failure

An effect operation may fail.

Failure behavior is part of the semantic contract when observable.

The effect model SHOULD distinguish:

effect occurred
effect failed before occurrence
effect partially occurred
effect completed
effect was compensated

This is especially important for:

- I/O;
- networking;
- distributed systems;
- transactions;
- hardware interaction;
- foreign calls.

---

133. Effect and transactions

Transactional effects MAY provide stronger guarantees.

A transaction may introduce:

transaction::begin
transaction::commit
transaction::rollback

The effect model MUST not assume that all effects are rollbackable.

Non-rollbackable effects MUST be identified where relevant.

---

134. Effect and idempotence

Effect metadata MAY declare whether an operation is:

idempotent
non_idempotent
conditionally_idempotent
unknown

This metadata is valuable for:

- retries;
- distributed execution;
- resilience;
- scheduling.

It MUST NOT be inferred merely from the operation name.

---

135. Effect and ordering

Some effects have ordering requirements.

Examples:

io::write
network::send
quantum::measurement
state::write

The semantic model MAY carry ordering constraints.

Optimization MUST preserve observable ordering.

---

136. Effect commutativity

Two effects MAY commute only if the semantic model proves that reordering them preserves observable behavior.

The compiler MUST NOT assume that effects commute simply because their namespaces differ.

---

137. Effect isolation

A pure computation may be isolated from effectful computation.

This may permit:

- parallelization;
- caching;
- memoization;
- vectorization;
- constant folding.

The compiler MUST establish effect safety before applying such transformations.

---

138. Effect capability metadata

Each effect definition MAY specify:

required_capabilities

These are metadata relationships.

The effect grammar MUST NOT embed a target-specific capability list.

---

139. Effect resource metadata

Each effect definition MAY specify semantic resource implications.

For example:

quantum::measurement

may imply a need for quantum measurement capability.

It does not specify:

exact physical qubit count

unless a separate resource requirement explicitly does so.

---

140. Effect policy metadata

Effect definitions MAY specify default policy classifications.

Examples:

sensitive
privileged
external
nondeterministic
nonrollbackable

Policy remains authoritative for actual enforcement.

---

141. Effect classification

An effect MAY have metadata categories such as:

pure
observable
mutating
external
nondeterministic
blocking
asynchronous
distributed
quantum
hardware
security_sensitive
resource_sensitive

Classification is metadata.

It MUST NOT replace the canonical effect identity.

---

142. Effect annotations

Effect-related attributes MAY be attached to declarations and operations.

Examples:

@deterministic
@idempotent
@external
@nonrollbackable

The attribute system owns syntax.

The effect specification defines how relevant attributes affect effect semantics.

Unknown attributes MUST NOT silently change effect meaning.

---

143. Effect operation attributes

Effect operation attributes MAY include semantic metadata such as:

- idempotence;
- ordering;
- determinism;
- latency class;
- retry safety;
- provenance requirements;
- security sensitivity.

Target-specific performance values belong to target metadata, not universal effect semantics.

---

144. Effect and performance

Performance is not itself an effect.

For example:

GPU execution

is not automatically an effect.

It may be a realization preference or capability.

Similarly:

fast

is not an effect.

Performance constraints belong to resources, policies, preferences, and target realization.

---

145. Effect and topology

Topology is not an effect.

For example:

requires topology(...)

is a resource/target constraint.

An operation may have a network or quantum effect whose realization depends on topology.

The two concepts MUST remain separate.

---

146. Effect and memory

Memory use is not automatically an effect.

The distinction is:

memory allocation/access behavior

may be effectful,

while:

requires memory >= required_memory

is a resource requirement.

---

147. Effect and capacity

Effect semantics MUST NOT encode fixed capacity.

Forbidden examples include:

quantum::measurement<32>
gpu::compute<24GB>
cpu::execute<8_threads>

when these numbers are merely target capacities.

---

148. Effect and scalable data

Effect operations MUST support arbitrary-size values where the type system permits them.

No effect grammar rule may restrict:

- tensor rank;
- collection length;
- qubit count;
- node count;
- message count;
- effect-set cardinality.

---

149. Effect and infinite/large structures

Recursive and streaming effect models SHOULD permit theoretically unbounded computation subject to runtime resources.

The language MUST NOT impose arbitrary finite semantic ceilings.

Implementations MAY protect themselves with resource budgets.

Such budgets MUST remain distinguishable from language semantics.

---

150. Effect and streaming

Streaming operations may carry effects such as:

stream::open
stream::read
stream::write
stream::close

A stream's size is not a language-level constant.

---

151. Effect and generators

Generators may suspend and resume effectful computations.

The effect model MUST account for effects performed across suspension boundaries.

A generator MUST NOT be considered pure merely because each suspension point returns a value.

---

152. Effect and coroutines

Coroutine execution may carry:

- scheduling effects;
- synchronization effects;
- communication effects;
- cancellation effects.

The effect model MUST integrate with the coroutine/concurrency subsystem.

---

153. Effect and exceptions

Error propagation does not automatically constitute a computational effect unless the language semantics classify it as one.

However, effect operations MUST preserve failure behavior.

The implementation MUST distinguish:

effect identity

from:

error type

A network error is not itself the same semantic entity as the network effect.

---

154. Effect and result types

An operation may encode ordinary failure through:

Result<T, E>

without eliminating the underlying effect.

For example:

network::request

returning:

Result<Response, NetworkError>

remains a network effect.

---

155. Effect and optionality

Optional execution does not automatically remove an effect.

If:

maybe_perform_io()

can perform I/O, the enclosing effect set must reflect that possibility.

---

156. Effect and pattern matching

Pattern matching may evaluate effectful expressions.

The effect analyzer MUST account for effects of:

- scrutinee;
- guards;
- pattern extraction;
- branch bodies.

The pattern grammar owns pattern syntax.

The effect system owns effect propagation.

---

157. Effect and guards

Guards may be effectful.

For example:

match value {
    x if query_external_state(x) => ...
}

must account for the effect of the guard.

The compiler MUST NOT treat guards as automatically pure.

---

158. Effect and closures

Closures capture effect environments.

The semantic representation MUST preserve the effects of the closure body.

Closure conversion MUST preserve the effect contract.

---

159. Effect and higher-order functions

A higher-order function that receives a callback MUST account for the callback's effects.

For example:

map(f)

may be effectful if "f" is effectful.

Effect polymorphism is the preferred mechanism for expressing this without hard-coding particular effects.

---

160. Effect and function pointers

Function values MUST carry or permit recovery of their effect contract when the language's type system requires it.

Calling a function pointer with unknown effects MUST not be treated as pure.

---

161. Effect and dynamic dispatch

Dynamic dispatch may obscure concrete effects.

The compiler MUST use the declared interface effect contract when the concrete implementation is not statically known.

If the effect contract is open, the dynamic call must retain the open effect information.

---

162. Effect and plugins

Plugins may introduce effects.

Plugin boundaries MUST declare:

- effect identities;
- capability requirements;
- resource requirements;
- security policy;
- ABI contract;
- provenance.

A plugin MUST NOT silently obtain arbitrary authority.

---

163. Effect and external services

External services are effectful.

Examples:

service::request
service::response
service::subscription

The service boundary MUST integrate with:

networking
security
policies
resources
provenance

---

164. Effect and persistent storage

Persistent storage operations are effectful.

Examples:

storage::read
storage::write
storage::commit
storage::query

Data semantics remain owned by:

grammar/data/

Effect semantics describe the external interaction.

---

165. Effect and queries

A query may be pure if it operates entirely over immutable local data.

A query over external state is effectful.

The compiler MUST determine this from semantic context rather than the word "query" alone.

---

166. Effect and databases

Database interactions MUST be represented through effect semantics plus data/query semantics.

SQL syntax belongs to an interoperability/dialect subsystem.

The universal effect system does not become a database language.

---

167. Effect and serialization

Serialization of an in-memory value may be pure.

Serialization involving external storage or communication may be effectful.

The effect classification MUST follow actual semantic behavior.

---

168. Effect and deserialization

Deserialization may be pure over a provided byte sequence.

External input acquisition is separate and effectful.

This distinction enables deterministic processing of already acquired data.

---

169. Effect and cryptography

Pure cryptographic computation over explicit values may be pure.

Operations involving:

- external keys;
- hardware security modules;
- entropy;
- secure storage;
- authorization;

may be effectful.

The effect model MUST represent the actual boundary.

---

170. Effect and secrets

Secret access is effectful when it crosses a semantic security boundary.

Examples:

security::secret_read
security::credential
security::key_access

Policy determines whether the operation is permitted.

---

171. Effect and authorization

Authorization decisions may themselves be effectful if they query external state.

The effect:

security::authorize

does not automatically imply success.

The result and effect remain separate.

---

172. Effect and audit

Security-sensitive effects MAY require audit provenance.

Audit requirements belong to policy/security.

Effect analysis supplies the effect facts required for policy evaluation.

---

173. Effect and logging

Logging is generally effectful when it produces externally observable output.

Example:

observability::log

A compiler MUST NOT remove logging merely because its return value is unused unless the language explicitly permits log elision under a policy.

---

174. Effect and telemetry

Telemetry is an external effect when data leaves the semantic computation boundary.

Telemetry policy MUST be able to restrict it.

---

175. Effect and diagnostics

Compiler diagnostics are not automatically runtime effects.

Compile-time diagnostics are compiler behavior.

Runtime diagnostics are program/runtime behavior and may be effectful.

The two MUST remain distinct.

---

176. Effect and compile-time execution

Compile-time execution is semantically separate from runtime execution.

A compile-time operation MUST declare or infer its own effect set.

Compile-time effects MUST NOT automatically become runtime effects.

Runtime effects MUST NOT automatically become compile-time permissions.

---

177. Effect and source generation

Generated source MUST enter the normal compiler pipeline.

The generator MUST NOT bypass:

lexer
parser
AST validation
type checking
effect checking
policy checking
resource checking
provenance

---

178. Effect and reflection

Reflection over metadata may be pure if metadata is immutable and embedded in the compilation artifact.

Reflection over external runtime state is effectful.

Reflection that modifies executable semantics requires explicit mutation/code-generation effects.

---

179. Effect and self-modification

Unrestricted self-modifying code is not a primitive effect exemption.

Any semantic adaptation of executable behavior MUST be represented through:

adaptation
reflection
code_generation
policy
provenance

as applicable.

---

180. Effect and reproducible adaptation

An adaptive program MAY be reproducible if all adaptive inputs and decisions are controlled and recorded.

Otherwise the semantic environment MUST identify the source of nondeterminism.

---

181. Effect and decision records

Decision-producing computations MAY emit provenance records containing:

decision
inputs
effects
evidence
policy
capabilities
resources
realization
result

The effect system does not own the entire decision record.

It contributes the effect component.

---

182. Effect and explainability

Explainability SHOULD be available for effect inference and transformations.

For example:

why was this effect required?
why was this target rejected?
why was simulation selected?
why was this optimization prohibited?

The answer SHOULD be derived from semantic evidence rather than ad hoc text.

---

183. Effect and target rejection

If a target cannot satisfy a capability/resource requirement associated with an effect, the compiler MUST distinguish:

program semantically invalid

from:

program valid but target infeasible

This distinction is fundamental to POCO-REAF.

---

184. Effect portability

An effect itself is portable when its semantic meaning is target-independent.

For example:

quantum::measurement

is portable as an intent.

Its realization may vary between:

- QPU;
- simulator;
- hybrid device;
- future quantum substrate.

---

185. Effect portability does not guarantee feasibility

A portable effect does not guarantee that every target can execute it.

The correct result may be:

valid source
target infeasible

rather than:

source rewritten

---

186. Effect and target-specific syntax

Target-specific effects MAY exist through target dialects.

They MUST be explicitly marked as target-specific.

They MUST NOT masquerade as universal effects.

---

187. Effect and deployment

Deployment configuration may impose effect policies.

For example:

production:
    forbid native::call

The deployment system owns the policy.

The effect system remains the semantic source of effect facts.

---

188. Effect and runtime

Runtime behavior MUST implement the semantic effect contract.

Runtime MUST NOT redefine language-level effect identity.

Runtime may:

- dispatch;
- schedule;
- retry;
- recover;
- monitor;
- enforce policies;
- collect provenance.

It does not redefine source semantics.

---

189. Effect and HAL

The HAL translates semantic requirements into target-specific mechanisms.

The HAL MUST NOT become the authority for effect meaning.

The relationship is:

semantic effect
      ↓
lowered operation
      ↓
HAL mapping
      ↓
target API

---

190. Effect and canonical IR

The canonical IR MUST preserve all effect information required for semantic correctness.

Effect metadata MAY be attached to:

- functions;
- operations;
- blocks;
- calls;
- regions;
- modules;
- executable units.

The exact IR schema belongs to the IR subsystem.

---

191. Effect and classical IR

Classical IR MUST preserve classical effects.

Examples:

io::read
state::write
network::request
randomness::sample

The effect representation MUST remain target-independent until lowering.

---

192. Effect and "quantum::ir"

Quantum semantic operations MUST carry the effect information necessary for:

- measurement;
- state interaction;
- dynamic control;
- external quantum interaction;
- classical feedback.

The effect system does not create "quantum::ir".

The quantum subsystem owns that representation.

---

193. Effect and HDL IR

HDL/hardware IR MUST preserve effects necessary for:

- externally visible signals;
- state;
- hardware I/O;
- timing contracts where semantically specified;
- synchronization;
- external interfaces.

The effect system does not create HDL IR.

---

194. Effect and optimization metadata

Effect metadata MAY be used by optimization passes.

Examples:

pure
idempotent
deterministic
commutative
nonblocking
read_only

Such metadata MUST be trusted only when validated.

---

195. Effect metadata trust

Effect metadata from external sources MUST be validated according to the trust model.

An untrusted foreign declaration claiming:

pure

MUST NOT automatically receive the same trust as compiler-verified source code if the trust model requires stronger evidence.

---

196. Effect contracts at ABI boundaries

ABI declarations MUST expose effect information where necessary.

A foreign function boundary without an effect contract MUST receive a conservative effect classification.

The implementation MUST NOT assume purity by default for arbitrary foreign code.

---

197. Effect and FFI

FFI integration belongs to:

grammar/interoperability/

Effect integration requires:

FFI declaration
      ↓
effect contract
      ↓
capability requirements
      ↓
resource requirements
      ↓
policy validation
      ↓
provenance

---

198. Effect and C/C++ interoperability

C/C++ calls are external effects unless their contracts are explicitly modeled.

The effect system MUST NOT depend on a particular C/C++ ABI.

ABI-specific information belongs to interoperability.

---

199. Effect and vendor APIs

Vendor APIs MUST be treated as realization mechanisms.

A vendor API MUST NOT define universal effect semantics.

Vendor adapters MUST map:

vendor behavior
      ↓
Zamani effect semantics

and preserve the source contract.

---

200. Effect and custom effects

User-defined effects are first-class semantic extensions.

A user-defined effect MUST have:

- canonical identity;
- declaration;
- semantic metadata;
- optional parameters;
- compatibility identity;
- capability relationships where applicable;
- resource relationships where applicable;
- policy classification where applicable;
- provenance requirements where applicable.

---

201. Custom effect isolation

A custom effect MUST NOT silently modify unrelated language semantics.

For example, defining:

effect application::telemetry

does not modify:

- arithmetic;
- memory semantics;
- quantum semantics;
- type checking;
- resource allocation.

---

202. Effect registries

The implementation SHOULD provide a registry for effect definitions.

A registry entry SHOULD contain:

identity
version
documentation
classification
parameters
operations
subeffects
capabilities
resources
policies
provenance requirements
compatibility

The registry is semantic metadata.

It MUST NOT become a hard-coded finite list of all possible effects.

---

203. Standard effects

The language/runtime MAY define standard effect identities.

Standard effects MUST remain extensible.

Adding a standard effect MUST NOT require rewriting generic effect-set grammar if the generic syntax already supports its identity.

---

204. Effect namespace collision

Two independently defined effects with the same fully qualified identity MUST NOT silently coexist with different semantics.

The compiler MUST detect:

- duplicate definitions;
- incompatible versions;
- conflicting providers.

---

205. Effect imports

Effects MAY be imported through modules.

Imports MUST obey normal module visibility and version rules.

An imported effect MUST resolve to one canonical semantic identity.

---

206. Effect alias collisions

Aliases that resolve to incompatible effect identities MUST produce diagnostics.

The compiler MUST NOT silently choose one.

---

207. Effect cycles

Effect inheritance, aliasing, composition, or transformation cycles MUST be detected.

Cycles that are semantically legal MUST have a well-defined fixed-point interpretation.

Illegal cycles MUST produce deterministic diagnostics.

---

208. Effect graph

The semantic analyzer MAY represent effects as a graph:

effect identity
      │
      ├── operations
      ├── subeffects
      ├── capabilities
      ├── resources
      ├── policies
      └── provenance

The graph MUST remain open-world.

---

209. Effect analysis termination

Effect analysis MUST terminate for all finite source programs accepted by the implementation unless explicitly documented resource exhaustion occurs.

Recursive dependencies MUST be handled through:

- memoization;
- fixed-point computation;
- strongly connected components;
- equivalent terminating techniques.

---

210. Effect-analysis resource limits

An implementation MAY impose compiler resource limits.

Examples:

- memory budget;
- compilation time;
- recursion depth;
- graph size;
- cache size.

Such limits MUST produce resource diagnostics.

They MUST NOT be encoded as language-level effect constants.

---

211. Effect analysis failure

If effect analysis cannot complete because of compiler resource exhaustion, the compiler MUST NOT claim:

program is effect-free

It MUST report the analysis failure.

---

212. Effect analysis soundness priority

When exact analysis cannot be completed, the compiler SHOULD prefer a conservative result over an unsoundly narrow effect set.

---

213. Effect analysis and diagnostics stability

Diagnostics SHOULD be deterministic for identical semantic input.

Diagnostic ordering SHOULD use stable source locations and canonical semantic ordering.

---

214. Effect analysis and parallel compilation

Effect analysis MAY be parallelized.

Parallel analysis MUST produce the same semantic result as serial analysis.

---

215. Effect analysis and distributed compilation

Distributed compilation MAY partition effect analysis.

The final result MUST be deterministic and equivalent to a valid single-process semantic analysis.

---

216. Effect serialization

Effect metadata serialized into compiler artifacts MUST include sufficient version information to detect incompatibility.

Serialized effect identities MUST NOT depend on:

- memory addresses;
- process IDs;
- nondeterministic ordering;
- machine-specific pointers.

---

217. Effect artifact compatibility

An artifact containing effect metadata MUST be rejected or migrated when the effect schema is incompatible.

Silent reinterpretation is prohibited.

---

218. Effect schema version

The effect semantic model MUST have an independently trackable schema version.

This is separate from:

language version
grammar version
AST version
IR version
dialect version

---

219. Effect migration

A breaking effect-model change MUST define a migration path when backward compatibility is promised.

Migration MAY transform:

old effect identity
→
new effect identity

but MUST preserve semantic intent where compatibility claims require it.

---

220. Effect deprecation lifecycle

A standard lifecycle is:

proposed
    ↓
experimental
    ↓
stable
    ↓
deprecated
    ↓
removed

The compatibility subsystem owns lifecycle policy.

This specification owns effect semantics during each phase.

---

221. Effect testing requirements

Every stable effect feature MUST have:

- lexical tests where applicable;
- parser tests;
- AST tests;
- semantic tests;
- effect-inference tests;
- compatibility tests;
- negative tests;
- boundary tests;
- scalability tests;
- deterministic-analysis tests.

---

222. Effect positive tests

Positive tests MUST cover:

single effect
multiple effects
qualified effects
empty effect set
trailing comma
custom effect
parameterized effect
effect operation
effect handler
effect polymorphism
effect inference
effect propagation

---

223. Effect negative tests

Negative tests MUST cover:

undeclared closed effect
invalid effect identity
invalid alias
conflicting effect declaration
illegal handler
effect-policy violation
effect-capability conflict
effect-resource conflict
invalid effect substitution
invalid effect variance
invalid recursive effect definition

---

224. Effect boundary tests

Boundary tests MUST cover interactions between:

effects × types
effects × generics
effects × contracts
effects × policies
effects × resources
effects × capabilities
effects × provenance
effects × concurrency
effects × distributed
effects × quantum
effects × HDL
effects × AI
effects × FFI
effects × metaprogramming
effects × simulation

---

225. Effect scalability tests

Scalability tests MUST demonstrate that the implementation does not rely on fixed semantic limits.

Tests SHOULD include generated effect sets of progressively larger size.

The test design MUST avoid repository-defined artificial constants such as:

MAX_EFFECTS

as language semantics.

---

226. Effect determinism tests

Given identical input and semantic environment:

analysis(source) == analysis(source)

must hold.

Serialized effect metadata MUST also be stable where deterministic serialization is promised.

---

227. Effect property tests

Property-based testing SHOULD validate:

normalization(normalization(E)) == normalization(E)

and:

union(E, empty) == E

and:

union(E, E) == E

where the semantic model defines ordinary set semantics.

---

228. Effect handler property tests

Where handler semantics define elimination:

handle(E, handler) 

must produce the expected remaining effect set.

The implementation MUST test both:

handled effect

and:

effects introduced by handler

---

229. Effect inference property tests

For a function body whose direct effect set is known:

inferred_effects(body)

must contain every semantically reachable direct effect.

---

230. Effect optimization tests

Each optimization that changes effectful structure MUST have equivalence tests.

At minimum:

before effects
=
after effects

unless the transformation has an explicitly defined effect transformation.

---

231. Effect lowering tests

Lowering MUST preserve the effect contract.

For each lowering:

source semantic effects

must correspond to:

lowered semantic effects

after applying the formally defined transformation.

---

232. Effect runtime tests

Runtime tests MUST verify:

- effect dispatch;
- handler behavior;
- policy enforcement;
- capability failures;
- resource failures;
- effect ordering;
- retry behavior where applicable;
- provenance.

---

233. Effect security tests

Security tests MUST verify that an effect cannot obtain unauthorized capabilities.

Examples:

network effect
without network capability

must fail appropriately.

native effect
inside sandbox

must obey sandbox policy.

---

234. Effect provenance tests

Provenance tests MUST verify that effect origins survive:

- inference;
- generic substitution;
- optimization;
- lowering;
- code generation;
- target specialization.

---

235. Effect compatibility tests

Compatibility tests MUST cover:

- old effect declarations;
- renamed effects;
- deprecated effects;
- version constraints;
- dialect effects;
- serialized effect metadata;
- compiler artifacts.

---

236. Effect and source spans

Every AST-level effect construct MUST retain source location information sufficient for diagnostics.

At minimum:

start position
end position
source identity

The current AST uses "Span".

Production effect nodes MUST integrate with that mechanism.

---

237. Effect AST contract

The domain-neutral AST SHOULD represent effect semantics using dedicated structures rather than encoding all information in arbitrary strings.

A production representation should be capable of representing conceptually:

EffectId {
    namespace
    name
    version
}

EffectReference {
    identity
    arguments
    span
}

EffectSet {
    entries
    openness
    span
}

EffectDeclaration {
    identity
    parameters
    return_type
    attributes
    span
}

EffectOperation {
    effect
    operation
    arguments
    results
    attributes
    span
}

EffectHandler {
    handled_effects
    operations
    body
    effects_introduced
    span
}

Exact Rust types are owned by the AST implementation.

The semantic information MUST nevertheless be preserved.

---

238. Current AST integration requirement

The current AST contains:

Statement::EffectDeclaration(Span, Identifier)

and:

Expression::Perform(Span, Box<Expression>)

These nodes are acceptable as transitional representations only if the compiler can recover all required semantics without ambiguity.

For production completeness, the AST implementation MUST evolve toward structured effect information where necessary.

The specification does not permit a parser to accept richer syntax and then silently discard that information.

---

239. AST ownership

Effect AST nodes MUST be owned by the AST subsystem.

The grammar MUST NOT construct Rust AST structures directly.

The parser produces parse trees.

The frontend converts parse trees into AST.

The semantic analyzer resolves AST effect identities.

---

240. Semantic effect model

The semantic model MUST contain enough information to support:

effect identity
operation identity
effect set
effect variables
effect substitutions
effect handlers
effect transformations
capability relations
resource relations
policy relations
provenance
diagnostics

---

241. Canonical semantic effect representation

The semantic representation SHOULD normalize effects into a canonical structure.

Conceptually:

SemanticEffect {
    identity
    parameters
    classification
    operations
    capabilities
    resources
    policy_metadata
    provenance
}

The actual implementation type is owned by the semantic subsystem.

---

242. Effect metadata ownership

The semantic effect registry owns:

effect identity
effect semantics
effect classification
operation definitions
relationships

The resource registry owns:

resource identity
capacity
availability
allocation

The capability registry owns:

capability identity
provider semantics

The policy registry owns:

permission
prohibition
preference
fallback

The provenance system owns:

origin
derivation
evidence
decision
verification

No subsystem may silently take ownership of another subsystem's semantics.

---

243. Canonical IR effect contract

Canonical IR MUST be capable of preserving:

effects
effect operations
effect ordering where required
effect attributes
effect transformations
effect provenance references

IR MUST NOT replace semantic effect identity with vendor-specific instructions before the appropriate lowering boundary.

---

244. IR ownership

The effect specification does not own the IR schema.

IR files own IR representation.

This specification defines the minimum effect information that IR must preserve.

---

245. Quantum IR ownership

"quantum::ir" owns quantum intermediate representation.

Effect metadata may accompany quantum IR operations.

The effect subsystem MUST NOT create a competing quantum representation.

---

246. Classical IR ownership

The canonical classical IR owns classical operation representation.

Effect metadata is attached according to IR contracts.

---

247. HDL ownership

HDL IR owns hardware-level representation.

Effects remain semantic metadata.

---

248. Compiler pass ownership

Compiler passes MUST consume effect information rather than reimplementing effect semantics independently.

There MUST be one semantic effect analysis authority.

---

249. Runtime ownership

Runtime MUST consume lowered effect metadata and enforce applicable policies.

Runtime MUST NOT silently reinterpret source effect identity.

---

250. Target ownership

Targets provide:

- capabilities;
- resource inventory;
- implementation mappings;
- constraints;
- performance characteristics;
- availability;
- failure state.

Targets do not redefine effect semantics.

---

251. Effect realization

The realization process is:

source effect
      ↓
semantic effect
      ↓
effect requirements
      ↓
capabilities/resources/policies
      ↓
feasibility
      ↓
lowering
      ↓
target implementation

---

252. Effect realization substitution

Two target implementations may realize the same semantic effect differently.

For example:

quantum::measurement

may be realized through:

- physical measurement;
- logical measurement;
- simulation;
- another target-supported mechanism.

The substitution is valid only when semantic equivalence is established.

---

253. Effect equivalence

Two implementations are effect-equivalent when they preserve the declared observable semantics.

Physical similarity is not required.

---

254. Effect realization and provenance

When an effect is lowered to a target-specific implementation, provenance SHOULD record:

source effect
target realization
lowering rule
backend
target identity/version
policy
capabilities

where required by the provenance policy.

---

255. Effect and target migration

A program may be migrated from one target to another.

Effect semantics MUST remain stable.

Only realization may change.

---

256. Effect and fallback

Fallback is a realization decision.

Examples:

QPU unavailable
→ simulator permitted

or:

accelerator unavailable
→ CPU realization permitted

Fallback MUST be explicitly permitted by policy or source constraints.

It MUST NOT silently change program meaning.

---

257. Effect and recovery

Recovery is runtime/realization behavior.

A recovered operation remains subject to the original effect contract.

---

258. Effect and retry

Retry semantics MUST respect operation metadata.

Non-idempotent effects MUST NOT be blindly retried.

The effect specification supplies the semantic metadata required for this decision.

---

259. Effect and cancellation

Cancellation may itself be effectful.

The implementation MUST distinguish:

cancel requested
operation canceled
operation completed before cancellation
operation partially completed

where the underlying effect requires those distinctions.

---

260. Effect and compensation

Compensating an effect is not equivalent to erasing the effect from history.

For example:

write
rollback

does not mean the write never happened.

Provenance MUST preserve such distinctions where required.

---

261. Effect and distributed consistency

Distributed effect semantics MUST cooperate with consistency models.

A network effect does not automatically imply a particular consistency model.

Consistency requirements belong to:

distributed
contracts
policies
resources

---

262. Effect and actor model

Actor syntax belongs to:

grammar/concurrency/actors.g4

AI or multi-agent semantics MAY use actors.

An actor message may introduce effects.

The effect system MUST integrate with actor semantics rather than create a second actor system.

---

263. Effect and agent systems

An agent may have:

reasoning
learning
communication
adaptation
memory
tool invocation

Each behavior MUST contribute its appropriate effects.

There is no special universal "agent effect".

---

264. Effect and neural-symbolic computation

Neural-symbolic systems may combine:

learning
reasoning
knowledge
inference

The effect system MUST compose their effects normally.

No separate effect universe is permitted.

---

265. Effect and uncertainty

Uncertainty itself is not necessarily an effect.

A probability distribution represented as immutable data may be pure.

Sampling from a random source is effectful.

The effect system MUST distinguish:

uncertain value

from:

random sampling operation

---

266. Effect and confidence

Confidence is data/semantic metadata.

Producing confidence from a pure computation may be pure.

Obtaining confidence from an external service may be effectful.

---

267. Effect and evidence

Evidence may be:

local immutable data

or:

external observation

The effect classification follows the actual semantic source.

---

268. Effect and provenance

Provenance data itself may be pure when constructed locally.

Persisting provenance externally is effectful.

---

269. Effect and observability

Observability operations such as tracing and metrics are effectful when externally visible.

Optimization MUST respect observability policy.

---

270. Effect and debugging

Debug instrumentation may add effects.

Production builds MAY remove debug-only effects only when the language/build policy permits it.

---

271. Effect and conditional compilation

Conditional compilation may change effects.

Each compilation configuration MUST perform effect analysis independently.

A configuration MUST NOT reuse incompatible effect assumptions from another configuration.

---

272. Effect and feature gates

Feature gates may enable effect definitions.

Feature configuration is part of the semantic environment.

An unavailable effect definition MUST produce a deterministic diagnostic.

---

273. Effect and dialect loading

Dialect loading occurs before semantic effect resolution.

A dialect may provide:

effect definitions
operations
capabilities
resources
policies

The dialect MUST map them into canonical semantic structures.

---

274. Effect and application libraries

Application-specific behavior SHOULD normally be represented by:

- libraries;
- effect definitions;
- capabilities;
- policies;
- dialects;
- APIs.

The core effect grammar MUST remain universal.

---

275. Effect keyword policy

A new effect concept SHOULD use an identifier before introducing a new reserved keyword.

A keyword is justified only when:

- the syntax cannot be expressed clearly otherwise;
- lexical ambiguity requires it;
- the construct is language-wide;
- compatibility has been considered.

Domain-specific effect names SHOULD NOT become universal keywords.

---

276. Effect lexer policy

Effect names SHOULD normally remain identifiers.

The lexer MUST NOT enumerate every effect.

Examples such as:

quantum::measurement
network::request
learning::update

must remain representable without adding a new lexer token for each operation.

---

277. Effect grammar policy

The grammar SHOULD use generic structures:

qualifiedName
operationName
argumentList
attributeList
effectSet

rather than enumerating domain operations.

This preserves open-world extensibility.

---

278. Effect grammar safety

Effect grammar files MUST contain:

- parser rules only where applicable;
- no embedded Rust;
- no semantic predicates that perform computation;
- no filesystem access;
- no network access;
- no environment inspection;
- no hardware discovery;
- no runtime execution;
- no random behavior.

---

279. Rust implementation safety

All Rust implementation components related to effects MUST support:

#![forbid(unsafe_code)]

The effect subsystem MUST use safe Rust.

No "unsafe" block, unsafe trait implementation, unsafe function, or unsafe foreign abstraction is permitted in the production effect implementation.

FFI may exist at an explicit interoperability boundary, but the production Zamani effect implementation itself MUST remain safe Rust.

---

280. Rust version

The minimum supported Rust version for the implementation is:

Rust 1.97 or later

The implementation MUST NOT depend on language/library features newer than the declared minimum without updating the compatibility contract.

---

281. Rust ownership model

Effect analysis SHOULD use owned/borrowed structures consistent with safe Rust.

The implementation SHOULD prefer:

- immutable data where possible;
- explicit ownership;
- deterministic collections;
- stable identifiers;
- structured error types;
- bounded recursion where practical.

---

282. Rust error handling

Effect analysis errors SHOULD use structured diagnostics rather than panics.

Expected semantic failures SHOULD be represented through appropriate "Result"-based APIs.

Compiler bugs may panic only where the repository's global compiler error policy explicitly permits it.

---

283. Effect implementation determinism

Effect registries and semantic analysis MUST avoid nondeterministic iteration when output ordering is observable.

Stable maps/sets or explicit sorting SHOULD be used where deterministic output matters.

---

284. Effect implementation memory behavior

The implementation MAY use dynamic data structures.

It MUST NOT assume a fixed number of effects.

Memory exhaustion MUST be treated as an implementation/resource failure rather than a language semantic limit.

---

285. Effect implementation recursion

Recursive semantic analysis SHOULD use iterative/fixed-point approaches where unbounded source nesting could otherwise cause stack exhaustion.

If recursion is used, resource exhaustion MUST produce a controlled compiler diagnostic where practical.

---

286. Effect implementation parallelism

Parallel effect analysis is permitted.

The implementation MUST remain deterministic.

No data race or unsafe synchronization mechanism is permitted.

---

287. Effect implementation caching

Caches MUST use semantic identities rather than source pointer addresses.

Cache keys SHOULD include:

effect definition version
language/semantic environment
source identity
relevant generic substitutions
relevant dialect versions

---

288. Effect implementation serialization

Serialization MUST be versioned and deterministic where reproducibility requires it.

Serialized effects MUST NOT depend on:

- memory addresses;
- hash-map iteration order;
- process-local identifiers;
- machine-local pointer values.

---

289. Effect implementation diagnostics

Diagnostics MUST include source spans.

Where possible, diagnostics SHOULD include:

actual effect
expected effect
declaration
origin
call chain
policy
capability/resource consequence
suggested remediation

---

290. Effect diagnostic severity

The implementation SHOULD distinguish:

error
warning
note
help

according to repository diagnostic conventions.

A warning MUST NOT silently alter semantic meaning.

---

291. Effect linting

The compiler MAY provide lints for:

- redundant effects;
- duplicate effects;
- unnecessary effect widening;
- unused effect declarations;
- deprecated effects;
- suspicious masking;
- unhandled external effects;
- missing provenance;
- non-idempotent retry;
- overly broad effect contracts.

Lints MUST remain separate from normative semantic errors.

---

292. Effect documentation

Every standard effect SHOULD document:

identity
purpose
operations
parameters
results
classification
capabilities
resources
policies
determinism
failure
retry behavior
provenance
compatibility

---

293. Effect ownership contract

The following ownership is mandatory.

Concern| Owner
Effect semantic meaning| "grammar/spec/effects.md"
Effect declaration syntax| "grammar/effects/effect-declarations.g4"
Effect set syntax| "grammar/effects/effect-sets.g4"
Effect operation syntax| effect operation grammar
Effect handler syntax| "grammar/effects/effect-handling.g4"
Effect orchestration| "grammar/effects/effects.g4"
Lexical tokens| "grammar/antlr/ZamaniLexer.g4" / "grammar/lexer/"
Names| "grammar/core/" / name-resolution subsystem
Types| "grammar/types/" / type subsystem
Capabilities| "grammar/resources/" / capability subsystem
Resources| "grammar/resources/"
Policies| "grammar/policies/" / "grammar/security/"
Contracts| "grammar/validation/"
Provenance| provenance subsystem
AST representation| Rust AST subsystem
Effect semantic analysis| Rust semantic/compiler subsystem
Canonical IR representation| IR subsystem
Quantum IR| "quantum::ir" subsystem
Target realization| backend/HAL
Runtime execution| runtime
Compatibility| "grammar/compatibility/"
Conformance tests| "grammar/tests/" and compiler tests

---

294. "grammar/effects/effects.g4" integration contract

"grammar/effects/effects.g4" remains the effect grammar orchestrator.

It MUST:

- compose effect grammar modules;
- expose stable public effect rules;
- preserve acyclic dependencies;
- reuse canonical shared syntax;
- avoid enumerating effect identities;
- avoid semantic implementation;
- avoid target selection;
- avoid capability discovery.

It MUST NOT redefine the semantics specified here.

---

295. "grammar/effects/effect-sets.g4" integration contract

This file owns:

effectReference
effectReferenceList
effectSet
optionalEffectSet

It MUST:

- use canonical qualified names;
- allow arbitrary cardinality;
- support empty sets where specified;
- support trailing commas where specified;
- avoid effect enumeration;
- avoid semantic duplicate rejection;
- avoid capability/resource logic.

Semantic normalization belongs downstream.

---

296. "grammar/effects/effect-handling.g4" integration contract

This file owns handler syntax.

It MUST NOT:

- perform effect dispatch;
- allocate resources;
- inspect hardware;
- select a target;
- implement continuation runtime;
- implement security authorization.

It MUST expose sufficient syntax for the AST to preserve handler semantics.

---

297. Effect-operation grammar integration

The effect-operation grammar MUST provide a generic operation representation.

It MUST support:

effect identity
operation identity
arguments
attributes
results

without enumerating all future operations.

---

298. "grammar/effects/custom-effects.g4" integration contract

Custom effects MUST map into the same semantic effect model.

This grammar MUST NOT create a second effect declaration universe.

Custom effects MUST remain compatible with:

effect registry
capability registry
resource registry
policy registry
provenance
compatibility

---

299. Resource integration

Effect analysis consumes resource semantics but does not own them.

The relationship is:

effect
  ↓
semantic resource implications
  ↓
resource requirements
  ↓
capability/resource feasibility

Resource requirements MUST remain target-independent until realization.

---

300. Capability integration

Effect definitions MAY identify required capabilities.

Capability resolution is performed by the capability subsystem.

The parser MUST NOT inspect available capabilities.

---

301. Policy integration

Policies MAY:

allow
forbid
require
constrain
prefer
fallback

effects.

Policy evaluation occurs after semantic effect information is available.

---

302. Contract integration

Contracts MAY constrain effects.

Effect analysis provides evidence to contract checking.

Contracts do not redefine effect identity.

---

303. Provenance integration

Every effect transformation that can affect user-visible semantics SHOULD be provenance-traceable.

At minimum, provenance SHOULD distinguish:

source effect
inferred effect
transformed effect
handled effect
lowered effect
target realization

---

304. Compatibility integration

Effect definitions and effect metadata MUST participate in compatibility checks.

The compatibility subsystem MUST distinguish:

language compatibility
grammar compatibility
AST compatibility
effect-model compatibility
IR compatibility
dialect compatibility
target capability compatibility

---

305. Testing integration

Effect tests SHOULD be organized under a structure such as:

grammar/tests/effects/
├── lexical/
├── parser/
├── ast/
├── semantic/
├── inference/
├── polymorphism/
├── handlers/
├── operations/
├── resources/
├── capabilities/
├── policies/
├── contracts/
├── provenance/
├── quantum/
├── hdl/
├── ai/
├── distributed/
├── interoperability/
├── execution/
├── compatibility/
├── scalability/
├── determinism/
├── positive/
└── negative/

Existing repository test organization MAY differ, but ownership must remain unambiguous.

---

306. Required conformance matrix

Every effect feature SHOULD be tracked using:

Feature| Specification| Lexer| Grammar| AST| Semantics| Effects| Capabilities| Resources| Policies| Contracts| Provenance| IR| Runtime| Tests| Compatibility
Effect declaration| required| required if keyword| required| required| required| required| optional| optional| optional| optional| required| required| optional| required| required
Effect set| required| required| required| required| required| required| optional| optional| optional| optional| required| required| optional| required| required
Effect operation| required| required if needed| required| required| required| required| required where applicable| required where applicable| required where applicable| optional| required| required| required| required| required
Effect handler| required| required if needed| required| required| required| required| optional| optional| required where applicable| required where applicable| required| required| required| required| required
Effect inference| required| no| no| required| required| required| derived| derived| required| required| required| required| optional| required| required
Effect polymorphism| required| required if syntax requires| required| required| required| required| derived| derived| required| required| required| required| optional| required| required
Custom effects| required| preferably no new keyword| required| required| required| required| optional| optional| optional| optional| required| required| optional| required| required

---

307. Production completion criteria

The effect subsystem is NOT production-ready merely because ANTLR accepts the syntax.

It is production-ready only when:

Specification
      ✓
Lexer integration
      ✓
Parser integration
      ✓
AST representation
      ✓
Name resolution
      ✓
Effect declaration resolution
      ✓
Effect operation resolution
      ✓
Effect normalization
      ✓
Effect inference
      ✓
Effect propagation
      ✓
Effect polymorphism
      ✓
Effect handler semantics
      ✓
Type integration
      ✓
Capability integration
      ✓
Resource integration
      ✓
Policy integration
      ✓
Contract integration
      ✓
Provenance integration
      ✓
Canonical IR integration
      ✓
Quantum IR integration
      ✓
Classical integration
      ✓
HDL/hardware integration
      ✓
AI/data integration
      ✓
Distributed/network integration
      ✓
FFI integration
      ✓
Metaprogramming integration
      ✓
Simulation integration
      ✓
Runtime integration
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
Determinism tests
      ✓
Compatibility tests
      ✓
Safe-Rust audit
      ✓
Hard-coding audit
      ✓

---

308. Hard-coding audit

The effect implementation MUST NOT contain semantic constants such as:

MAX_EFFECTS
MAX_EFFECT_SET_SIZE
MAX_EFFECT_OPERATIONS
MAX_EFFECT_HANDLERS
MAX_EFFECT_PARAMETERS
MAX_EFFECT_DEPTH
MAX_QUANTUM_EFFECTS
MAX_CLASSICAL_EFFECTS
MAX_HARDWARE_EFFECTS

No fixed machine capacity may be encoded into effect semantics.

The implementation MAY contain operational safeguards, but they MUST be:

- configurable;
- documented;
- separate from language semantics;
- reported as implementation/resource failures;
- incapable of changing the meaning of valid programs.

---

309. Open-world extension invariant

A future effect such as:

future::photonic::operation

must be representable without modifying:

grammar/effects/effect-sets.g4

or introducing a new universal keyword.

The semantic registry and relevant dialect/backend may be extended independently.

---

310. Future hardware invariant

A future computational substrate may introduce entirely new effect categories.

The architecture MUST permit:

new effect
+
new capability
+
new resource
+
new policy
+
new realization

without redesigning the universal effect grammar.

---

311. Future computational model invariant

The effect system MUST remain usable for computational models not currently known to the language.

A new model must be able to express:

what behavior occurs
what capability is required
what resources are implicated
what policies apply
what contracts apply
what provenance is required

without requiring the language to predict that model in advance.

---

312. Cross-domain integration invariant

The effect system MUST be common across:

classical
quantum
HDL
AI
data
distributed
networking
accelerator
embedded
HPC
simulation
future domains

A domain MUST NOT create a competing effect system.

---

313. Domain integration pattern

Every domain should follow:

domain operation
      ↓
canonical effect identity
      ↓
semantic effect model
      ↓
capabilities/resources/policies
      ↓
canonical IR
      ↓
domain lowering

---

314. Quantum domain pattern

quantum operation
      ↓
quantum effect
      ↓
quantum semantic validation
      ↓
quantum::ir
      ↓
optimization
      ↓
routing
      ↓
scheduling
      ↓
resilience/QEC
      ↓
ZQN
      ↓
HAL

---

315. Classical domain pattern

classical operation
      ↓
classical effect
      ↓
semantic validation
      ↓
canonical classical IR
      ↓
optimization
      ↓
target lowering

---

316. HDL domain pattern

HDL operation
      ↓
hardware effect
      ↓
hardware semantic validation
      ↓
HDL IR
      ↓
simulation / synthesis / verification
      ↓
hardware realization

---

317. AI domain pattern

learning/reasoning/adaptation operation
      ↓
canonical effect
      ↓
policy/capability/resource validation
      ↓
semantic model
      ↓
canonical IR
      ↓
target realization

---

318. Distributed domain pattern

distributed operation
      ↓
distributed effect
      ↓
network/capability/resource/policy analysis
      ↓
canonical semantic model
      ↓
IR
      ↓
scheduling/routing
      ↓
runtime

---

319. Interoperability domain pattern

foreign operation
      ↓
foreign effect
      ↓
ABI validation
      ↓
capability/policy analysis
      ↓
canonical semantic model
      ↓
IR
      ↓
backend

---

320. Effect semantic invariant

The fundamental invariant is:

EFFECT
  describes
BEHAVIOR

CAPABILITY
  describes
PROVIDER ABILITY

RESOURCE
  describes
REALIZATION MATERIAL

REQUIREMENT
  describes
NECESSARY CONDITION

CONSTRAINT
  describes
ALLOWED REALIZATION SPACE

POLICY
  describes
PERMITTED BEHAVIOR/REALIZATION

CONTRACT
  describes
SEMANTIC OBLIGATION

PROVENANCE
  describes
ORIGIN AND DERIVATION

None of these concepts may silently become another.

---

321. POCO-REAF invariant

The effect system exists to preserve source-level computational meaning while allowing implementation variation.

Therefore:

                         SAME SOURCE
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
            tiny            CPU/GPU          QPU
              │               │               │
              ▼               ▼               ▼
          embedded          FPGA             ASIC
              │               │               │
              └───────────────┼───────────────┘
                              ▼
                         accelerator
                              │
                              ▼
                            HPC
                              │
                              ▼
                          cluster
                              │
                              ▼
                       distributed system
                              │
                              ▼
                         future target

The physical realization may differ.

The source-level effect semantics remain stable.

---

322. Effect portability invariant

A program MUST NOT require source rewriting merely because:

- CPU count changes;
- GPU count changes;
- memory size changes;
- QPU capacity changes;
- FPGA family changes;
- accelerator changes;
- node count changes;
- topology changes;
- target vendor changes.

Those are realization concerns unless the program explicitly requests target-specific behavior.

---

323. Effect feasibility invariant

A target that cannot satisfy the semantic requirements of an effectful program MUST produce a feasibility result such as:

capability unavailable
resource unavailable
policy prohibits realization
effect unsupported
target incompatible

It MUST NOT silently change the source program's meaning.

---

324. Effect preservation invariant

The following transformations MUST preserve effect semantics unless an explicit semantic transformation is defined:

optimization
inlining
specialization
parallelization
vectorization
distribution
simulation
hardware mapping
quantum routing
scheduling
recovery
target migration

---

325. Effect transparency invariant

The programmer describes:

what the computation does

rather than:

which machine performs it

unless target-specific behavior is explicitly requested.

---

326. Effect realization invariant

The compiler determines:

whether
where
how
and under which constraints

an effect can be realized.

The runtime executes the selected realization.

The HAL mediates target interaction.

---

327. Effect safety invariant

No effect may grant authority merely by being named.

An effect declaration does not grant:

- filesystem access;
- network access;
- hardware access;
- native execution;
- foreign execution;
- privileged security operations;
- adaptation authority;
- reflection authority.

Capabilities and policies remain independently enforced.

---

328. Effect trust invariant

Source declarations, libraries, dialects, generated code, and foreign interfaces may have different trust levels.

The compiler/security system MUST preserve those distinctions where required.

---

329. Effect provenance invariant

An effect introduced by:

source
library
generated code
dialect
foreign function
compiler transformation
target lowering

SHOULD remain distinguishable in provenance where the policy requires it.

---

330. Effect semantic stability

The effect model MUST be stable enough that downstream systems can rely on it.

Changing the meaning of an existing effect requires:

- specification update;
- compatibility analysis;
- AST impact analysis;
- semantic impact analysis;
- IR impact analysis;
- test updates;
- migration strategy where applicable.

---

331. No hidden effects

A compiler transformation MUST NOT introduce an observable effect without:

- preserving it;
- proving it unobservable;
- representing it through an allowed transformation.

Hidden effects are prohibited.

---

332. No erased effects

The compiler MUST NOT erase an effect merely because:

- the return value is unused;
- the target implements it differently;
- an optimization pass cannot easily represent it;
- the runtime does not currently support it.

Unsupported effects require explicit diagnostics or supported fallback.

---

333. No invented effects

The compiler MUST NOT claim an effect occurred when no semantic behavior supports that claim.

Target metadata may provide implementation details, but source effect semantics must remain accurate.

---

334. Effect analysis and source fidelity

The compiler SHOULD retain source-level effect information sufficiently for:

- diagnostics;
- IDE tooling;
- explanations;
- provenance;
- refactoring;
- compatibility analysis.

---

335. Effect tooling

Tooling SHOULD expose:

show effects
show inferred effects
show effect origins
show effect dependencies
show effect handlers
show policy conflicts
show capability requirements
show resource consequences
show target feasibility

Tooling MUST consume canonical semantic data rather than reparsing source independently.

---

336. Effect documentation generation

Documentation tooling MAY generate API effect contracts from semantic metadata.

Generated documentation MUST use the canonical effect identity.

---

337. Effect API stability

Public APIs SHOULD expose effect contracts explicitly where effect behavior is relevant.

Changing a public API's effects may be a compatibility-breaking change.

---

338. Effect dependency tracking

The compiler SHOULD track dependencies between:

function
  ↓
callee
  ↓
effect

This allows incremental invalidation.

---

339. Effect call graph

Effect inference MAY construct a call graph.

The call graph MUST account for:

- direct calls;
- indirect calls;
- dynamic dispatch;
- function values;
- callbacks;
- recursion;
- generated calls;
- foreign calls.

---

340. Unknown dynamic calls

If a dynamic call cannot be resolved, the compiler MUST use the declared dynamic effect contract.

If no contract exists, it MUST use the repository's conservative external-call policy.

It MUST NOT assume purity.

---

341. Effect and generic recursion

Generic recursive functions MUST converge under effect inference.

The implementation SHOULD memoize effect instantiations.

---

342. Effect and specialization

Specialization MAY narrow effects when the specialization proves fewer effects.

The resulting specialization MUST retain a correct effect contract.

---

343. Effect and monomorphization

Monomorphization MUST substitute effect variables along with type variables.

No effect information may be lost.

---

344. Effect and trait dispatch

Trait/type-class dispatch MUST use the declared effect contract of the selected implementation or the conservative contract of the abstraction.

---

345. Effect and macro expansion

Macro expansion may introduce effects.

After expansion, generated constructs MUST undergo normal effect analysis.

Macros MUST NOT bypass effect checking.

---

346. Effect and declarative macros

Declarative macros that only transform syntax may be effect-free as compiler transformations.

Generated program behavior remains subject to normal effect analysis.

---

347. Effect and procedural macros

Procedural compile-time code may itself have compile-time effects.

Those effects MUST be governed by the compile-time execution policy.

---

348. Effect and generated IR

Compiler-generated IR may contain effect metadata.

Generated IR MUST conform to canonical IR effect contracts.

---

349. Effect and serialization formats

Effect metadata serialized into external formats MUST have a documented schema.

Unknown effect fields MUST follow compatibility rules.

---

350. Effect and version negotiation

When communicating effect metadata between compiler components, each component MUST agree on the applicable semantic schema version.

---

351. Effect and plugin compatibility

A plugin declaring an effect schema incompatible with the compiler MUST be rejected or explicitly adapted.

Silent reinterpretation is prohibited.

---

352. Effect and future extension

The effect system is deliberately designed so that future domains can add:

new effect identity
new operation
new capability
new resource relationship
new policy
new backend

without changing the universal effect grammar when generic syntax suffices.

---

353. File completion contract

"grammar/spec/effects.md" is complete only when it provides unambiguous answers for:

- what an effect is;
- what an effect is not;
- how effects are named;
- how effects are declared;
- how effects are used;
- how effect sets work;
- how effects are inferred;
- how effects propagate;
- how effects are handled;
- how effects compose;
- how effect polymorphism works;
- how effects interact with types;
- how effects interact with capabilities;
- how effects interact with resources;
- how effects interact with policies;
- how effects interact with contracts;
- how effects interact with provenance;
- how effects interact with canonical IR;
- how effects interact with "quantum::ir";
- how effects interact with classical computation;
- how effects interact with HDL;
- how effects interact with AI;
- how effects interact with data;
- how effects interact with distributed execution;
- how effects interact with networking;
- how effects interact with FFI;
- how effects interact with simulation;
- how effects interact with adaptation;
- how effects interact with reflection;
- how effects interact with code generation;
- how effects remain scalable;
- how effects remain deterministic;
- how effects remain compatible;
- how effects are tested;
- how effects are implemented safely in Rust.

No downstream grammar file should need to invent the semantic meaning of these concepts.

---

354. Required implementation dependency order

The implementation should proceed in this dependency order:

1. Effect semantic specification
        ↓
2. Effect identity model
        ↓
3. Effect registry/model
        ↓
4. Effect AST representation
        ↓
5. Effect declaration resolution
        ↓
6. Effect-set normalization
        ↓
7. Effect operation model
        ↓
8. Effect inference
        ↓
9. Effect propagation
        ↓
10. Effect handler analysis
        ↓
11. Effect polymorphism
        ↓
12. Capability integration
        ↓
13. Resource integration
        ↓
14. Policy integration
        ↓
15. Contract integration
        ↓
16. Provenance integration
        ↓
17. Canonical IR integration
        ↓
18. Quantum IR integration
        ↓
19. Compiler optimization integration
        ↓
20. Runtime integration
        ↓
21. Conformance tests
        ↓
22. Compatibility tests

---

355. Required file-level integration contracts

Every implementation file that owns effect behavior SHOULD declare:

PURPOSE:
OWNS:
DOES_NOT_OWN:
DEPENDS_ON:
EXPORTS:
AST_OWNER:
SEMANTIC_OWNER:
CAPABILITY_OWNER:
RESOURCE_OWNER:
POLICY_OWNER:
CONTRACT_OWNER:
PROVENANCE_OWNER:
IR_OWNER:
RUNTIME_OWNER:
TEST_OWNER:
COMPATIBILITY_OWNER:
SCALABILITY_REQUIREMENTS:
DETERMINISM_REQUIREMENTS:
SAFETY_REQUIREMENTS:
COMPLETION_CRITERIA:

This prevents one file from silently assuming semantics owned by another file.

---

356. Required effect grammar header contract

Every production effect ".g4" file SHOULD document:

Purpose
Owns
Does Not Own
Lexer Dependencies
Grammar Dependencies
Exported Rules
AST Contract
Semantic Contract
Capability Contract
Resource Contract
Policy Contract
Contract Contract
Provenance Contract
IR Contract
Diagnostics
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility
Completion Criteria

---

357. No circular ownership

The following cycles are prohibited:

grammar ↔ runtime
grammar ↔ hardware
effects ↔ resource allocation
effects ↔ capability discovery
effects ↔ target selection
effects ↔ quantum::ir
effects ↔ scheduling
effects ↔ QEC

Correct direction:

grammar
 ↓
AST
 ↓
semantic effects
 ↓
capability/resource/policy/contract analysis
 ↓
canonical semantic model
 ↓
IR
 ↓
lowering
 ↓
target realization

---

358. Final architectural model

The complete effect architecture is:

                    SOURCE PROGRAM
                          │
                          ▼
                       EFFECTS
                          │
            ┌─────────────┼─────────────┐
            ▼             ▼             ▼
          TYPES      CAPABILITIES    RESOURCES
            │             │             │
            └─────────────┼─────────────┘
                          ▼
                       POLICIES
                          │
                       CONTRACTS
                          │
                      PROVENANCE
                          │
                          ▼
                  SEMANTIC EFFECT MODEL
                          │
             ┌────────────┴────────────┐
             ▼                         ▼
       CLASSICAL IR                quantum::ir
             │                         │
             └────────────┬────────────┘
                          ▼
                     OPTIMIZATION
                          │
                       LOWERING
                          │
                  ROUTING/SCHEDULING
                          │
                 RESILIENCE / RECOVERY
                          │
                         ZQN
                          │
                         HAL
                          │
                          ▼
                   TARGET REALIZATION

---

359. Final production invariants

The following are mandatory:

1. There is one semantic effect model.
2. Effect identities are open-world.
3. Effect names are not hard-coded into the universal grammar.
4. Effect sets have no artificial universal cardinality.
5. Effect parameters have no artificial universal cardinality.
6. Effect handlers have no artificial universal cardinality.
7. Effect recursion has no artificial semantic depth limit.
8. Compiler resource limits are not language semantics.
9. Effects are separate from capabilities.
10. Effects are separate from resources.
11. Effects are separate from requirements.
12. Effects are separate from constraints.
13. Effects are separate from preferences.
14. Effects are separate from policies.
15. Effects are separate from contracts.
16. Effects are separate from provenance.
17. Effects are separate from target realization.
18. Effects are separate from scheduling.
19. Effects are separate from routing.
20. Effects are separate from QEC.
21. Effects are separate from ZQN.
22. Effects are separate from HAL.
23. Effect inference is sound.
24. Effect normalization is deterministic.
25. Effect propagation is sound.
26. Effect handlers cannot silently grant capabilities.
27. Effect masking requires semantic justification.
28. Foreign calls are not assumed pure by default.
29. Native calls are not assumed pure by default.
30. Generated code cannot bypass effect checking.
31. Reflection cannot bypass effect checking.
32. Adaptation cannot bypass policy or provenance.
33. Learning cannot bypass effect analysis.
34. Quantum measurement remains semantically visible.
35. Quantum effects converge through "quantum::ir".
36. Classical effects converge through the canonical classical IR.
37. HDL effects remain target-independent until hardware lowering.
38. Distributed effects remain separate from physical topology.
39. Network effects remain separate from network resources.
40. Hardware effects remain separate from physical hardware identity.
41. Effect optimization preserves semantics.
42. Effect lowering preserves semantics.
43. Target migration preserves source effect meaning.
44. Fallback is explicit.
45. Simulation is not silently substituted for physical execution.
46. Retry respects effect semantics.
47. Non-idempotent operations are not blindly retried.
48. Provenance can explain effect origins.
49. Diagnostics distinguish semantic errors from target infeasibility.
50. Effect metadata is versioned.
51. Effect compatibility is explicit.
52. Effect registries remain extensible.
53. New computational domains do not require redesigning the universal effect model.
54. New hardware does not require redesigning the universal effect grammar.
55. New quantum operations do not require enumerating new grammar alternatives.
56. No source-level machine-size constants exist in effect semantics.
57. The implementation supports Rust 1.97 or later.
58. Production Rust uses no "unsafe".
59. Production Rust MUST support "#![forbid(unsafe_code)]".
60. Effect analysis is deterministic.
61. Effect serialization is deterministic where required.
62. Effect analysis remains scalable with available compiler resources.
63. Effect semantics remain stable across target realization.
64. Every production effect feature has positive tests.
65. Every production effect feature has negative tests.
66. Every production effect feature has boundary tests.
67. Every production effect feature has scalability tests.
68. Every production effect feature has determinism tests.
69. Every production effect feature has compatibility tests.
70. Every production effect implementation has an explicit integration contract.

---

360. Final POCO-REAF effect principle

The fundamental rule is:

A Zamani effect describes what the computation semantically does.

It does not prescribe which machine must do it.

Therefore:

                 PROGRAM
                    │
                    ▼
              EFFECT INTENT
                    │
       ┌────────────┼────────────┐
       ▼            ▼            ▼
   CAPABILITY    RESOURCE      POLICY
       │            │            │
       └────────────┼────────────┘
                    ▼
               FEASIBILITY
                    │
                    ▼
             CANONICAL SEMANTICS
                    │
          ┌─────────┴─────────┐
          ▼                   ▼
     Classical IR         quantum::ir
          │                   │
          └─────────┬─────────┘
                    ▼
                LOWERING
                    │
             ROUTING/SCHEDULING
                    │
             RESILIENCE/RECOVERY
                    │
                   ZQN
                    │
                   HAL
                    │
                    ▼
             ACTUAL TARGET

The same source-level effect contract can therefore be realized on a tiny target, a large classical system, an accelerator, FPGA, ASIC, QPU, simulator, HPC system, cluster, distributed system, or future computational substrate, provided the target satisfies the required semantic conditions.

The language does not define an artificial maximum machine size.

The implementation does not define an artificial maximum effect count.

The compiler does not silently replace program meaning when a target is insufficient.

The runtime does not redefine effect semantics.

The hardware does not define the language.

The source program defines portable computational intent.

That is the production effect-system foundation required for POCO-REAF.