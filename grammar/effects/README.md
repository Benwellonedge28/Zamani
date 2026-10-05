Zamani Effects Grammar

Path: "grammar/effects/"
Language: Zamani
Role: Canonical source-language effect grammar subsystem
Grammar technology: ANTLR4 parser grammars
Rust baseline: Rust 1.97 / Rust 1.97.1, Edition 2021
Safety requirement: Safe Rust only; "unsafe" is prohibited
Composition root: "grammar/effects/effects.g4"

---

1. Purpose

The "grammar/effects/" subsystem defines the source-language syntax boundary for computational effects.

An effect describes computational behavior that is observable, requested, propagated, handled, declared, inferred, constrained, or otherwise relevant to semantic analysis.

Effects are a foundational part of Zamani's target-independent architecture.

They participate in:

- declarations;
- operations;
- function signatures;
- effect sets;
- effect inference;
- effect polymorphism;
- effect composition;
- effect handling;
- contracts;
- capabilities;
- requirements;
- resources;
- policies;
- security analysis;
- provenance;
- optimization legality;
- interoperability;
- classical computation;
- quantum computation;
- HDL/hardware intent;
- distributed computation;
- networking;
- simulation;
- learning;
- adaptation;
- reflection;
- foreign/native interoperability;
- execution planning.

The subsystem must therefore remain domain-neutral at the syntax layer while allowing semantic classification into arbitrary present and future computational domains.

The effect grammar describes what computational behavior exists or is requested.

It does not describe the physical mechanism used to realize that behavior.

---

2. Architectural position

The canonical pipeline is:

Zamani source
    │
    ▼
canonical lexical analysis
    │
    ▼
canonical parser
    │
    ├── core
    ├── declarations
    ├── expressions
    ├── statements
    ├── types
    └── effects
             │
             ▼
       domain-neutral AST
             │
             ▼
     structural validation
             │
             ▼
       semantic analysis
             │
       ┌─────┼───────────┬───────────────┐
       ▼     ▼           ▼               ▼
    effects capabilities resources     policies
       │     │           │               │
       └─────┴───────────┴───────────────┘
                         │
                         ▼
               canonical semantic model
                         │
             ┌───────────┼───────────────┐
             ▼           ▼               ▼
       classical      quantum::ir    HDL/hardware
             │           │               │
             └───────────┼───────────────┘
                         ▼
                    optimization
                         │
                    specialization
                         │
                    lowering
                         │
              routing / scheduling
                         │
               resilience / recovery
                         │
                    QEC / ZQN
                         │
                        HAL
                         │
                         ▼
                 target realization

The effect grammar participates in this pipeline but does not own downstream semantic or physical realization.

---

3. Core architectural rule

The effect subsystem answers:

«What computational behavior is part of this program, operation, declaration, or computation?»

It does not answer:

«Which physical machine performs it?»

It does not answer:

«How many resources are physically available?»

It does not answer:

«Which device is selected?»

It does not answer:

«Which routing or scheduling algorithm is used?»

Those questions belong to downstream semantic, resource, capability, policy, execution, and target-realization systems.

---

4. POCO-REAF requirement

Zamani is designed around:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

The effect system must preserve that property.

The same source-level effect declaration or effect usage must remain semantically meaningful when its computation is eventually realized on:

- a tiny embedded system;
- a single CPU;
- multicore CPUs;
- GPUs;
- accelerator systems;
- FPGA systems;
- ASIC implementations;
- heterogeneous systems;
- quantum processors;
- quantum simulators;
- classical/quantum hybrid systems;
- distributed systems;
- HPC systems;
- cloud systems;
- edge systems;
- future computational substrates.

An effect therefore describes portable semantic intent, not a fixed implementation.

For example:

effect quantum::measurement;

must not imply:

a particular QPU
a particular physical qubit
a fixed qubit count
a fixed topology
a particular calibration
a particular gate set
a particular vendor

Likewise:

effect networking::request;

must not imply:

a particular network interface
a particular node
a particular port
a particular protocol implementation

---

5. Non-negotiable invariants

The following are permanent architecture rules.

5.1 Effects are semantic

An effect represents computational behavior.

It is not a physical resource.

---

5.2 Effects are not capabilities

An effect describes behavior.

A capability describes something an execution environment can provide.

Therefore:

effect       = computation-side semantic behavior

capability   = environment-side ability/facility

requirement  = condition that must be satisfied

resource     = capacity/asset consumed, reserved, transferred,
               or otherwise managed

constraint   = condition limiting valid realization

preference   = non-mandatory realization preference

policy       = rule governing permitted realization/execution

These concepts must not silently collapse into one another.

---

5.3 Effects are not targets

The effect grammar must never select:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- simulator;
- accelerator;
- cloud provider;
- operating system;
- physical device;
- vendor implementation.

---

5.4 Effects are not resource declarations

The effect grammar must not define:

- memory capacity;
- processor count;
- thread count;
- qubit capacity;
- storage capacity;
- accelerator capacity;
- node count;
- network capacity.

Those belong to resource and capability systems.

---

5.5 Effects are not scheduling

The grammar must not encode:

- thread scheduling;
- task placement;
- quantum scheduling;
- instruction scheduling;
- network scheduling;
- hardware scheduling.

Scheduling is downstream.

---

5.6 Effects are not routing

The grammar must not encode:

- physical qubit routing;
- network routing;
- device placement;
- accelerator placement;
- FPGA placement;
- cluster placement.

Routing is downstream.

---

5.7 Effects are not QEC

Quantum error correction is not an effect grammar responsibility.

Quantum effects may describe semantic behavior associated with error correction, measurement, recovery, resilience, or noise.

The actual QEC representation remains downstream of "quantum::ir".

---

6. No hard-coded scalability ceilings

The effect grammar must contain no language-level finite ceiling for:

- effects;
- effect declarations;
- effect operations;
- effect references;
- effect-set entries;
- parameters;
- generic parameters;
- handlers;
- handler arms;
- nesting;
- domains;
- namespaces;
- modules;
- source size;
- program size.

The following are prohibited:

MAX_EFFECTS
MAX_EFFECT_OPERATIONS
MAX_EFFECT_SET_SIZE
MAX_EFFECT_REFERENCES
MAX_EFFECT_PARAMETERS
MAX_EFFECT_GENERICS
MAX_HANDLER_ARMS
MAX_EFFECT_DEPTH

More generally, no effect grammar rule may introduce an artificial ceiling corresponding to any machine or implementation capacity.

ANTLR repetition and recursive structures must be used where appropriate.

Any practical limit imposed by:

- parser implementation;
- memory;
- stack;
- compilation resources;
- execution resources;
- deployment environment;

must be treated as an implementation/resource-policy concern, not as the semantic definition of the Zamani language.

A resource exhaustion failure must never redefine valid Zamani syntax or semantics.

---

7. Open-world effect identity

Effects must use an open-world identity model.

The core grammar must not contain a closed catalogue such as:

effectKind
    : IO
    | Network
    | Quantum
    | GPU
    | FPGA
    | QPU
    ;

That architecture would require modifying the language grammar whenever a new computational domain appears.

Instead, effect identity is represented using the canonical Zamani naming/path infrastructure.

Examples include:

IO
Storage
networking::request
quantum::measurement
quantum::reset
distributed::replication
accelerator::tensor_compute
ai::inference
learning::adaptation
security::authorization
future::domain::effect
vendor::extension::effect

The parser recognizes the symbolic identity.

Semantic analysis determines whether the identity is:

- built-in;
- imported;
- user-defined;
- dialect-defined;
- implementation-defined;
- experimental;
- deprecated;
- unavailable;
- unknown.

This allows future computational domains without requiring the core grammar to be rewritten.

---

8. Qualified names

The effect subsystem must reuse the repository's canonical qualified-name grammar.

It must not define a competing identifier or namespace system.

For example:

quantum::measurement
distributed::replication
ai::inference
security::audit
future::photonic::interaction

are names.

Their domain interpretation belongs downstream.

The effect grammar therefore depends on:

grammar/core/

for name identity and qualified-name structure.

---

9. Current repository ownership

The effect directory contains multiple specialized grammar components.

The existing filenames are retained unless a repository-wide architectural decision later proves a particular file redundant.

The subsystem currently includes, among other components:

grammar/effects/
├── README.md
├── adaptation.g4
├── capabilities.g4
├── code_generation.g4
├── custom-effects.g4
├── distributed.g4
├── effect-composition.g4
├── effect-declarations.g4
├── effect-diagnostics.g4
├── effect-diagnostics.md
├── effect-handling.g4
├── effect-operations.g4
├── effect-polymorphism.g4
├── effect-sets.g4
├── effect-types.g4
├── effects.g4
├── hardware.g4
├── io.g4
├── network.g4
├── quantum.g4
└── security.g4

The subsystem must not become a collection of unrelated mini-languages.

All files are subordinate to the same effect model.

---

10. Composition root: "effects.g4"

"grammar/effects/effects.g4" is the composition root.

It owns:

- effect grammar composition;
- effect parser entry points;
- imports;
- public effect dispatch;
- integration boundaries;
- generic effect construct selection.

It does not own detailed implementations of:

- declarations;
- effect sets;
- operations;
- handlers;
- polymorphism;
- custom-effect declarations;
- domain classifications.

Those belong to their dedicated files.

The composition root must expose stable public rules to the rest of the parser.

The public rule names should be treated as parser API.

Consumers must not depend on:

- generated token numbers;
- generated parser internals;
- ANTLR implementation details;
- generated class names where an explicit stable Zamani grammar rule exists.

---

11. "effect-declarations.g4"

Owns

- effect declarations;
- effect declaration names;
- declaration signatures;
- effect generic parameters;
- effect parameters;
- effect operation declarations;
- effect operation signatures;
- operation parameters;
- operation return types;
- declaration-local attributes/modifiers;
- declaration termination.

Does not own

- lexical token definitions;
- qualified-name definition;
- general type grammar;
- expression grammar;
- effect invocation;
- effect handling;
- capability resolution;
- resource allocation;
- hardware selection;
- runtime execution;
- IR construction.

Dependencies

core/names
core/qualified-names
core/attributes
types

Downstream integration

effect declaration
        ↓
AST declaration
        ↓
name resolution
        ↓
semantic effect identity
        ↓
effect environment

The existing Rust AST currently exposes an "EffectDeclaration" representation. The grammar-to-AST adapter must target the canonical AST rather than introduce a second effect declaration tree.

Completion requirement

This file is complete when its syntax, AST mapping, semantic contract, diagnostics, and conformance tests are independently defined.

---

12. "effect-sets.g4"

Owns

- effect references;
- qualified effect references;
- effect lists;
- effect sets;
- optional effect sets;
- effect-set composition syntax;
- empty effect sets;
- trailing separators where permitted.

Does not own

- semantic deduplication;
- canonical ordering;
- effect inference;
- capability resolution;
- resource analysis;
- handler execution.

Required property

Effect-set cardinality is open-ended.

There must be no fixed grammar-level maximum.

Example conceptual form:

effects {
    io
    networking::request
    quantum::measurement
    ai::inference
}

The exact surface syntax remains governed by the normative grammar and parser.

---

13. "effect-operations.g4"

Owns

Source-level effect operation use.

This includes:

- operation reference;
- operation invocation;
- operation arguments;
- operation modifiers where defined;
- operation result syntax where defined.

Does not own

- operation implementation;
- runtime dispatch;
- hardware dispatch;
- quantum gate lowering;
- QEC;
- routing;
- scheduling;
- target selection.

An effect operation is a source-level semantic request.

Its implementation is determined later.

---

14. "effect-handling.g4"

Owns

- effect handler syntax;
- handler arms;
- handler patterns;
- handled computations;
- handler result syntax;
- explicit effect handling constructs.

Does not own

- runtime handler implementation;
- scheduler behavior;
- operating-system calls;
- hardware dispatch;
- backend implementation.

A handler must remain representable in the domain-neutral AST.

Semantic analysis determines:

- which effects are handled;
- which effects remain unhandled;
- handler compatibility;
- handler composition;
- handler scope;
- propagation behavior.

---

15. "effect-types.g4"

This file must not become a second type system.

It may express the relationship between types and effects where Zamani's type syntax requires such qualification.

The dependency direction is:

canonical type grammar
        +
canonical effect grammar
        ↓
effect-qualified type semantics

not:

effect-types.g4
        ↓
independent type system

All type definitions remain owned by:

grammar/types/

This prevents divergence between normal types and effect-qualified types.

---

16. "effect-polymorphism.g4"

Owns

- effect variables;
- effect-variable references;
- effect bounds;
- effect substitutions;
- effect-polymorphic constraints;
- effect-polymorphic signatures.

Does not own

- ordinary type generics;
- ordinary generic parameter syntax;
- effect declaration syntax;
- general effect-set syntax.

Where the generic type infrastructure already provides reusable generic constructs, this grammar must consume them rather than duplicate them.

The semantic system must support substitution without imposing a fixed number of effect variables.

---

17. "effect-composition.g4"

This file owns source-level composition constructs where explicit composition syntax exists.

It must reuse the canonical effect-reference and effect-set rules.

It must not create a second effect algebra.

Semantic normalization belongs downstream.

The semantic effect model must establish:

- identity;
- composition;
- propagation;
- subtraction/discharge where supported;
- equivalence;
- compatibility;
- normalization.

Those are semantic properties, not parser actions.

---

18. "custom-effects.g4"

Custom effects are necessary for a language intended to remain extensible.

This file owns syntax for user- or dialect-defined effect declarations/extensions where those constructs are part of the language.

It must permit future domains without modifying the generic effect grammar.

Examples may include:

research::observation
robotics::control
photonic::interaction
accelerator::tensor
scientific::simulation
future::domain::effect

These names do not become permanent core keywords merely because they are examples.

---

19. Domain effect grammars

The existing domain files:

io.g4
quantum.g4
hardware.g4
network.g4
distributed.g4
security.g4
capabilities.g4
adaptation.g4

must remain classification/integration layers over the generic effect model.

They must not create parallel effect languages.

Their role is to expose stable parser boundaries where a downstream subsystem needs to recognize a particular semantic domain.

The dependency direction is:

generic effect syntax
        ↓
domain classification
        ↓
semantic domain model

not:

domain grammar
        ↓
second effect system

---

20. "quantum.g4"

Quantum effects must remain target-independent.

Examples of semantic effect identities may include:

quantum::measurement
quantum::reset
quantum::readout
quantum::dynamic_control
quantum::state_preparation
quantum::error_detection

The effect grammar must not encode:

- physical qubit IDs;
- fixed qubit counts;
- physical coupling maps;
- calibration data;
- vendor topology;
- device IDs;
- pulse schedules;
- physical gate implementation.

The quantum pipeline remains:

source
    ↓
effect intent
    ↓
domain-neutral AST
    ↓
quantum semantic analysis
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
resilience / QEC
    ↓
ZQN
    ↓
HAL
    ↓
available realization

No second quantum effect IR may be created inside this grammar subsystem.

---

21. "hardware.g4"

Hardware effects describe semantic interaction with hardware facilities.

They must not describe a fixed machine.

Hardware identity, capability, resource, topology, placement, timing, performance, thermal properties, reliability, and physical implementation remain downstream concerns.

The grammar must remain valid for future hardware categories that do not yet exist.

---

22. "io.g4"

I/O effects represent interaction with external input/output facilities.

They must not hard-code:

- operating systems;
- filesystem implementations;
- device names;
- physical addresses;
- fixed streams;
- fixed descriptors;
- fixed buffer capacities.

Semantic I/O identity is resolved later.

---

23. "network.g4"

Networking effects represent network-related computation.

They must not encode a universal physical network configuration.

They must not define:

- fixed nodes;
- fixed ports;
- fixed interfaces;
- fixed topology;
- fixed network size.

Network capability and realization belong downstream.

Networking effects must integrate with:

grammar/networking/
grammar/resources/
grammar/security/
grammar/policies/
grammar/effects/

---

24. "distributed.g4"

Distributed effects describe semantic distributed behavior.

Examples may include:

distributed::replication
distributed::consensus
distributed::coordination
distributed::migration
distributed::communication

This grammar must not define a fixed number of:

- nodes;
- processes;
- actors;
- services;
- machines.

Distributed execution must integrate with the existing:

grammar/concurrency/
grammar/distributed/
grammar/networking/
grammar/resources/
grammar/effects/

The effect subsystem must not create a second actor/message system.

---

25. "security.g4"

Security-related effects describe security-sensitive computation.

Examples:

security::authentication
security::authorization
security::encryption
security::decryption
security::signing
security::verification
security::audit

The grammar must not implement security.

It must not:

- authenticate users;
- validate credentials;
- decrypt data;
- generate keys;
- authorize runtime operations;
- contact security providers;
- access secrets.

Those are downstream responsibilities.

Security effects must integrate with:

grammar/security/
grammar/policies/
grammar/resources/
grammar/effects/

---

26. "capabilities.g4"

This file does not own the universal capability system.

It owns only the relationship between effects and capabilities.

The distinction is:

effect
    ↓
semantic realization requirement
    ↓
capability requirement
    ↓
capability negotiation

Examples:

effect quantum::measurement
        ↓
requires capability("quantum.measurement")

or:

effect accelerator::tensor_compute
        ↓
requires capability("tensor.compute")

The effect grammar must not convert capability names into a fixed hardware catalogue.

The canonical capability system belongs elsewhere in the repository.

---

27. "adaptation.g4"

Adaptation is a semantic effect category, not unrestricted self-modification.

Adaptation may represent controlled changes to:

- strategy;
- model;
- execution plan;
- behavior;
- configuration;
- policy-selected alternatives;
- learned state.

It must participate in:

effects
contracts
capabilities
resources
policies
provenance
security
execution

Adaptation must not imply unrestricted arbitrary modification of the language, compiler, runtime, or machine.

The semantic architecture is:

adaptation request
        ↓
policy validation
        ↓
authorization
        ↓
capability validation
        ↓
resource validation
        ↓
contract validation
        ↓
provenance record
        ↓
authorized adaptation
        ↓
new semantic state/plan

Where adaptation affects quantum computation:

adaptation
    ↓
quantum semantics
    ↓
quantum::ir

No adaptation-specific quantum IR is permitted.

---

28. Learning and reasoning effects

The broader Zamani language may expose learning, reasoning, inference, deduction, knowledge, uncertainty, evidence, explanation, and adaptation.

These should integrate with the effect model rather than bypass it.

For example:

reasoning::infer
learning::learn
learning::adapt
knowledge::query
knowledge::assert
knowledge::retract
explanation::generate

These identities remain open-world semantic names.

The effect grammar must not enumerate every AI algorithm.

It must not require new grammar rules for every future:

- neural architecture;
- optimization algorithm;
- inference algorithm;
- probabilistic method;
- reasoning strategy;
- learning method.

The semantic model and libraries provide those extensions.

---

29. Effects and contracts

Effects must integrate with the repository's validation/contract system.

Conceptually:

requires
ensures
invariant
assume
guarantee
property
assert

may constrain or describe computations involving effects.

For example:

requires capability("network.request")
requires security::authorization

is not an effect declaration.

It is a requirement/contract relationship involving an effect.

Ownership remains separated:

effects/
validation/
resources/
policies/

---

30. Effects and policies

Policies govern permitted or preferred realization.

An effect can therefore participate in policies without owning the policy language.

Conceptual relationship:

effect
   ↓
policy evaluation
   ↓
permitted realization

Policies may:

- permit;
- forbid;
- constrain;
- prefer;
- require;
- authorize;
- select fallback behavior.

The effect grammar must not duplicate policy grammar.

---

31. Effects and resources

An effect may cause resource requirements.

For example:

quantum::measurement

may lead semantic analysis to determine that the selected realization requires appropriate quantum resources.

But the effect grammar itself must not say:

requires 32 qubits

as a physical assumption embedded in the effect identity.

Resource requirements belong to:

grammar/resources/

and are resolved against actual execution resources.

This distinction is fundamental to POCO-REAF.

---

32. Effects and capabilities

A capability describes what the execution environment can provide.

For example:

capability("quantum.measurement")
capability("tensor.compute")
capability("distributed.consensus")

The program can express requirements against capabilities without selecting a particular implementation.

The resulting architecture is:

effect
  ↓
required capability
  ↓
available capability
  ↓
compatible realization

The effect grammar never performs capability discovery.

---

33. Effects and provenance

Effect-related source constructs must retain source provenance.

At minimum, downstream semantic analysis must be able to associate:

- source location;
- effect identity;
- declaration/use;
- operation;
- enclosing declaration;
- source ordering;
- transformations;
- semantic decisions.

Provenance belongs to the repository-wide provenance subsystem.

The effect grammar only provides source structure and source spans through the parser pipeline.

---

34. Effects and explainability

Effect analysis may contribute to explanations.

For example, tooling may explain:

why was this capability required?
why was this implementation rejected?
why was this fallback selected?
why did this function acquire an effect?
why was an optimization prohibited?
why was a quantum realization selected?

The grammar must not implement explanations.

It must preserve sufficient source structure for semantic tooling to produce them.

---

35. Effects and deterministic/reproducible execution

Parsing an effect declaration or effect use must be deterministic.

The grammar must not:

- inspect the environment;
- inspect hardware;
- discover devices;
- access the network;
- use randomness;
- perform runtime execution.

Effect semantics may later depend on an execution environment, but the grammar itself must remain deterministic and side-effect free.

---

36. Effects and simulation

Simulation is an execution strategy.

It is not a separate effect language.

The effect subsystem must allow an effect to be semantically interpreted in a simulation context.

For example:

quantum::measurement

may be realized by:

quantum simulator

or:

quantum processor

without changing the source effect.

Likewise:

networking::request

may be realized through an actual network or a simulation environment.

The realization is downstream.

---

37. Effects and classical computation

Classical computation must consume the same effect model.

A classical function may have effects associated with:

- I/O;
- mutation;
- randomness;
- networking;
- foreign calls;
- native calls;
- distributed computation;
- learning;
- simulation.

No separate classical effect language should be created.

---

38. Effects and HDL/hardware

HDL/hardware semantics may consume effect information.

For example, an effect may indicate that a computation interacts with:

- signals;
- external interfaces;
- hardware control;
- timing-sensitive operations;
- memory;
- accelerators.

But the effect grammar must not become a hardware description language.

HDL semantics remain owned by:

grammar/hdl/

and hardware semantics by:

grammar/hardware/

---

39. Effects and foreign/native interoperability

Foreign and native calls are effectful boundaries.

The effect model should therefore be capable of representing semantic effects associated with:

foreign
native
ffi
abi

without embedding ABI details into the effect grammar.

The interoperability subsystem owns:

- ABI;
- calling conventions;
- linkage;
- foreign types;
- data layout;
- external symbols.

The effect system only records the semantic fact that a foreign/native boundary exists.

---

40. Effects and reflection/metaprogramming

Reflection and code generation can be effectful.

Potential semantic identities include:

reflection::inspect
reflection::generate
metaprogramming::execute
code_generation::generate

These must remain open-world names.

The effect grammar must not implement compile-time execution or code generation.

Those belong to:

grammar/metaprogramming/
grammar/macros/
grammar/compile/

---

41. Effects and concurrency

Effects must integrate with the existing concurrency architecture.

The dependency direction is:

effect
   ↓
semantic effect
   ↓
concurrency analysis
   ↓
task/actor/channel/message semantics
   ↓
scheduler

The effect grammar must not define:

- actor lifecycle;
- task scheduling;
- channel implementation;
- worker allocation.

Those remain owned by concurrency/execution subsystems.

---

42. Effects and agents

Agent systems must reuse the existing actor/concurrency architecture where appropriate.

An agent-related effect may identify semantic behavior such as:

agent::observe
agent::decide
agent::learn
agent::adapt
agent::communicate

but the effect grammar must not create a second actor model.

The intended relationship is:

agent semantics
      ↓
existing actor/concurrency semantics
      ↓
message/task execution

---

43. Lexer integration

The lexer is the lexical authority.

The effect grammar consumes lexer tokens.

It must not define lexer rules.

The current Rust lexer already contains effect-related tokens including:

KeywordEffect
KeywordHandle
KeywordPerform

Therefore any modular grammar additions must be reconciled with the existing lexer vocabulary rather than silently inventing competing spellings.

The lexer owns:

- token identity;
- keyword recognition;
- literals;
- operators;
- punctuation;
- source spans.

The effect grammar owns:

- syntactic arrangement of those tokens.

This boundary is mandatory.

---

44. Parser integration

The repository currently has a handwritten Rust parser as well as the ANTLR grammar architecture.

The current Rust parser recognizes effect-related statements including effect declarations and handlers.

Therefore production conformance must explicitly reconcile:

normative specification
        ↓
modular ANTLR grammar
        ↓
lexer vocabulary
        ↓
Rust parser
        ↓
AST
        ↓
semantic analysis

The two parser representations must not be assumed equivalent merely because both contain an effect feature.

Any discrepancy must be classified explicitly as:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

A README claim must never falsely represent an unimplemented feature as implemented.

---

45. AST integration

The effect grammar must map to the existing domain-neutral AST.

The AST must represent source meaning rather than backend implementation.

Effect-related AST information may include:

- effect declaration;
- effect identity;
- qualified name;
- parameters;
- generic parameters;
- effect set;
- operation;
- operation arguments;
- handler;
- handler arm;
- source span;
- source ordering;
- attributes/modifiers.

The effect grammar must not introduce:

QuantumEffectIR
HardwareEffectIR
GpuEffectIR
QpuEffectIR
BackendEffectIR

or equivalent parallel IRs.

The canonical semantic model is downstream.

---

46. Semantic integration

Semantic analysis owns:

- name resolution;
- effect identity resolution;
- effect inference;
- effect normalization;
- effect compatibility;
- effect polymorphism;
- handler checking;
- effect propagation;
- effect discharge;
- capability relationships;
- resource relationships;
- policy relationships;
- security relationships;
- provenance;
- domain classification.

The grammar does not perform these operations.

The parser produces structure.

The semantic layer produces meaning.

---

47. Canonical IR integration

Effects do not constitute a replacement for the canonical intermediate representation.

Effect information should become semantic metadata/operations associated with the canonical semantic representation.

For quantum computation:

effect
    ↓
quantum semantic analysis
    ↓
quantum::ir

For classical computation:

effect
    ↓
classical semantic analysis
    ↓
canonical classical representation

For HDL/hardware:

effect
    ↓
hardware semantic analysis
    ↓
HDL/hardware representation

There must be no effect-specific backend IR that bypasses the canonical architecture.

---

48. Quantum boundary

Where an effect affects quantum computation, the boundary is:

effect semantics
       ↓
quantum semantics
       ↓
quantum::ir

"quantum::ir" remains the canonical quantum representation.

The effect grammar must never own:

- physical qubit IDs;
- calibration;
- coupling maps;
- pulse schedules;
- routing;
- decomposition;
- QEC;
- ZQN.

Those are downstream.

---

49. Hardware boundary

Where an effect affects hardware realization:

effect semantics
       ↓
hardware semantic model
       ↓
capability/resource analysis
       ↓
target realization

The effect grammar does not select physical hardware.

---

50. Resource negotiation

Effects may participate in resource negotiation.

The architecture is:

effect
   ↓
semantic requirements
   ↓
resource/capability requirements
   ↓
available execution context
   ↓
negotiation
   ↓
valid realization

This supports source portability across machines of different scales.

A program should not need to be rewritten merely because a realization has:

- less or more memory;
- fewer or more processors;
- different accelerator availability;
- different quantum resources;
- different topology;
- different distributed capacity.

If the requirements cannot be satisfied, the compiler/runtime must report infeasibility rather than silently changing program meaning.

---

51. No physical resource syntax in effects

The following are prohibited inside effect identities:

CPU0
GPU3
QPU7
qubit17
node8
device2
fpga_region4

unless such names are explicitly ordinary user-defined symbolic identifiers in a separate target/deployment context.

The effect system must not assign physical meaning to them.

---

52. Effect inference

Where supported, effects may be inferred from program structure.

For example, a function may acquire an effect because its body performs an effectful operation.

Conceptually:

function body
    ↓
effectful operations
    ↓
inferred effect set
    ↓
function semantic effect set

Inference must be deterministic for a fixed source program and semantic environment.

Explicit annotations remain useful for:

- API contracts;
- verification;
- security;
- optimization;
- documentation;
- interoperability;
- compilation diagnostics.

---

53. Effect polymorphism

Effect polymorphism allows generic code to abstract over effects.

Conceptually:

fn transform<E>(...) -> ... 

where "E" represents an effect parameter subject to the language's effect constraints.

The number of effect variables is not fixed by the language.

Semantic substitution belongs to the effect/type analyzer.

---

54. Effect handlers

Handlers allow a computation to intercept or transform effectful behavior.

The grammar must preserve:

handler
handler scope
handler arms
handled effects
patterns
results

Semantic analysis determines whether:

- the handled effect exists;
- the handler is compatible;
- required effects remain;
- handling is exhaustive where required;
- effects propagate correctly.

Runtime implementation remains downstream.

---

55. Effect composition

Effect composition must be mathematically and semantically defined outside the parser.

The semantic system should establish rules for:

union
composition
normalization
substitution
propagation
handling
discharge
equivalence
compatibility

The grammar only expresses source syntax.

It must never encode implementation-specific effect ordering unless source ordering has explicit semantic meaning.

---

56. Error handling and diagnostics

"effect-diagnostics.g4" and "effect-diagnostics.md" must define or document diagnostics without embedding implementation behavior into the grammar.

Diagnostics must be:

- deterministic;
- source-located;
- actionable;
- stable where compatibility requires;
- domain-aware only after semantic classification.

Errors should distinguish, where applicable:

unknown effect
invalid effect declaration
invalid effect operation
invalid effect set
invalid handler
effect mismatch
effect constraint violation
capability mismatch
resource infeasibility
policy violation
contract violation

The parser must not falsely report a semantic failure as a syntax failure when the syntax itself is valid.

---

57. Negative parsing requirements

The effect test suite must include malformed constructs such as:

- incomplete effect declarations;
- malformed qualified names;
- malformed effect sets;
- malformed operations;
- malformed handlers;
- malformed generic effect parameters;
- invalid delimiters;
- invalid nesting.

These tests verify syntax only.

Semantic-invalid programs must be tested separately so that parser and semantic diagnostics remain distinguishable.

---

58. Cross-domain testing

The effect subsystem must be tested with combinations such as:

classical + IO
classical + networking
classical + distributed
AI + learning
AI + reasoning
AI + adaptation
AI + quantum
quantum + measurement
quantum + resilience
quantum + networking
HDL + simulation
hardware + accelerator
distributed + security
foreign + security
simulation + quantum
simulation + distributed

The purpose is to verify that effects compose without creating independent language universes.

---

59. Scalability testing

Scalability tests must test structural openness rather than arbitrary fixed limits.

Examples should include:

- many effect references;
- deeply qualified effect names;
- large effect declarations;
- large effect sets;
- many handler arms;
- many generic effect variables;
- nested effect constructs;
- large cross-domain programs.

Tests must never establish a maximum as part of the language definition.

If an implementation benchmark needs a particular workload size, that is a benchmark parameter, not a language ceiling.

---

60. Determinism requirements

For identical:

source
lexer configuration
grammar version
language version
semantic environment

the parser must produce deterministic syntax results.

The grammar must not depend on:

- random values;
- wall-clock time;
- filesystem state;
- network state;
- hardware discovery;
- environment variables;
- process IDs;
- device enumeration.

---

61. Safety requirements

The grammar subsystem must require no unsafe Rust.

The grammar itself must contain:

- no embedded Rust actions;
- no embedded runtime code;
- no semantic predicates unless explicitly justified by the parser architecture;
- no filesystem access;
- no network access;
- no hardware access;
- no random behavior;
- no environment inspection.

The Rust implementation baseline is:

Rust 1.97
Rust 1.97.1
Edition 2021

The repository must compile without requiring "unsafe".

---

62. Compatibility

Effect syntax must participate in Zamani's language compatibility model.

Compatibility must account for:

- language version;
- grammar version;
- parser version;
- AST version;
- semantic model version;
- effect vocabulary version;
- dialect version.

Deprecation must be explicit.

An effect must not silently change meaning across compatible language versions.

Where an effect changes semantic meaning, a migration or compatibility mechanism must be defined.

---

63. Dialect integration

New computational domains must be extensible through dialects where appropriate.

A dialect may introduce:

- new effect identities;
- semantic classifications;
- domain-specific attributes;
- domain-specific operations.

It must not redefine the universal effect model.

The relationship is:

core Zamani effect syntax
        ↓
dialect extension
        ↓
semantic registration
        ↓
domain interpretation

A dialect must not require modification of unrelated effect grammars.

---

64. Vendor extension boundary

Vendor-specific effects may exist through qualified symbolic names.

For example:

vendor::provider::effect

The core language must not hard-code vendor names.

Vendor semantics belong to:

- dialects;
- capability registries;
- target descriptions;
- interoperability layers;
- compiler plugins/tooling where supported.

This preserves source portability.

---

65. Future-domain requirement

A future computational domain must be able to introduce a semantic effect without requiring changes to the universal effect identity grammar merely because the domain is new.

For example, a future domain might define:

future::substrate::interaction

The generic effect parser must be capable of representing the identity using existing naming rules.

Only domain-specific semantic support should need to be added.

---

66. Effect metadata

Effect declarations and references may carry metadata where the canonical language supports it.

Metadata may describe:

- documentation;
- stability;
- version;
- provenance;
- attributes;
- semantic classification;
- interoperability information.

Metadata must not turn into an implicit hardware configuration mechanism.

---

67. Relationship to contracts

Effect declarations may participate in contracts.

The relationship is:

effect
    ↕
contract
    ↕
requirement / guarantee

Examples of conceptual relationships include:

requires effect-compatible capability
ensures effect is handled
invariant effect property
assume external effect property
guarantee effect behavior

Contract syntax remains owned by validation/contracts.

---

68. Relationship to policies

Effects can be evaluated under policies.

Examples include policies controlling:

network effects
foreign effects
native effects
adaptation effects
reflection effects
security effects
distributed effects
quantum effects

The effect grammar must not duplicate policy syntax.

Policy semantics remain independently owned.

---

69. Relationship to sandboxing

Sandboxing is a security/execution concern.

A sandbox may restrict effects such as:

filesystem
network
native
foreign
reflection
code_generation
adaptation

The sandbox does not redefine those effects.

Instead:

effect
   ↓
policy
   ↓
sandbox
   ↓
allowed/forbidden

This permits the same source program to execute under different authorized policies without changing its source semantics.

---

70. Relationship to provenance

Effect analysis should be traceable.

A provenance record may eventually identify:

source construct
effect identity
semantic classification
inferred effect
capability requirement
resource requirement
policy decision
optimization consequence
lowering consequence

The effect grammar itself remains responsible only for preserving source structure and spans.

---

71. Repository-wide ownership matrix

The effect subsystem must maintain the following ownership boundaries:

Concern| Owner
Tokens| "grammar/lexer/", canonical lexer
Names| "grammar/core/"
Types| "grammar/types/"
Expressions| "grammar/expressions/"
Statements| "grammar/statements/"
Effects| "grammar/effects/"
Capabilities| "grammar/core/" / "grammar/resources/" as defined by repository authority
Resources| "grammar/resources/"
Contracts| "grammar/validation/"
Policies| "grammar/policies/" when established
Security| "grammar/security/"
Concurrency| "grammar/concurrency/"
Classical semantics| classical subsystem
Quantum semantics| quantum subsystem
Quantum IR| "quantum::ir"
HDL semantics| "grammar/hdl/" / HDL semantic subsystem
Hardware realization| hardware subsystem
Networking| networking subsystem
Distributed execution| distributed subsystem
FFI/ABI| interoperability subsystem
Reflection| metaprogramming subsystem
Optimization| compiler subsystem
Routing| compiler/quantum execution subsystem
Scheduling| execution subsystem
QEC| quantum resilience subsystem
ZQN| quantum execution subsystem
HAL| hardware abstraction subsystem
Runtime| runtime subsystem

The exact repository path of a downstream semantic owner may evolve, but ownership must remain singular.

---

72. Public integration contract for every effect grammar file

Every ".g4" file under this directory must document the following before it is considered complete:

PURPOSE
OWNS
DOES_NOT_OWN
PUBLIC_RULES
PRIVATE_RULES
LEXER_DEPENDENCIES
GRAMMAR_DEPENDENCIES
AST_CONTRACT
SEMANTIC_CONTRACT
TYPE_CONTRACT
EFFECT_CONTRACT
CAPABILITY_CONTRACT
RESOURCE_CONTRACT
CONTRACT_CONTRACT
POLICY_CONTRACT
PROVENANCE_CONTRACT
QUANTUM_BOUNDARY
HDL_BOUNDARY
IR_DESTINATION
DIAGNOSTICS
POSITIVE_TESTS
NEGATIVE_TESTS
BOUNDARY_TESTS
CROSS_DOMAIN_TESTS
SCALABILITY_TESTS
DETERMINISM_TESTS
COMPATIBILITY
INTEGRATION
COMPLETION_CRITERIA

This is mandatory because a file must have a complete integration contract before it is declared finished.

---

73. File dependency declaration

Each effect grammar file should document:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
TYPE_OWNER:
EFFECT_OWNER:
CAPABILITY_OWNER:
RESOURCE_OWNER:
POLICY_OWNER:
PROVENANCE_OWNER:
SPEC_OWNER:
TEST_OWNER:
IR_DESTINATION:

This makes dependencies explicit before implementation begins.

A file must not depend on undocumented behavior from another grammar file.

---

74. Dependency direction

The intended direction is:

lexer
  ↓
core names/attributes/types
  ↓
effects
  ↓
AST
  ↓
semantic analysis
  ↓
capabilities/resources/contracts/policies/security
  ↓
canonical semantic representation
  ↓
domain IR
  ↓
optimization/lowering
  ↓
execution
  ↓
target realization

The effect grammar must never introduce reverse dependencies from syntax into:

- runtime;
- hardware;
- QPU discovery;
- scheduling;
- routing;
- resource allocation;
- target selection.

---

75. No grammar/runtime coupling

ANTLR grammar files must not:

- call Rust runtime functions;
- allocate resources;
- perform device discovery;
- execute effects;
- access external systems;
- inspect machine properties;
- invoke FFI;
- access environment variables.

Parsing must be a pure frontend operation.

---

76. No hidden semantic keywords

A word must not become a reserved keyword merely because a semantic subsystem uses it.

Where a concept can be expressed using an existing identifier/name mechanism, it should remain extensible.

Core keywords should exist only where lexical reservation provides genuine language value.

This prevents the language from becoming a catalogue of every domain feature.

---

77. No application-specific effect explosion

The effect system must not create a separate core effect keyword for every application domain.

The core language should not need dedicated effect syntax for things such as:

sentiment
computer_vision
robotics
payments
administration
legal_actions
VR
AR
blockchain

Such concepts should normally be represented through:

libraries
dialects
capabilities
policies
data models
domain APIs

when they do not constitute universal computational semantics.

This preserves Zamani as a universal programming language instead of turning the core grammar into an application DSL.

---

78. Effects as universal semantic infrastructure

The effect system should be capable of representing behavior from:

classical computation
quantum computation
hybrid computation
HDL
hardware control
AI
learning
reasoning
knowledge
uncertainty
simulation
distributed computation
networking
security
data processing
foreign execution
native execution
reflection
code generation
future computational domains

without making those domains mutually dependent.

---

79. Cross-domain semantic composition

The following must be possible at the semantic level:

AI
 └── quantum
      └── measurement
           └── classical decision
                └── distributed communication
                     └── security policy
                          └── provenance

The effect grammar does not implement that pipeline.

It ensures that each source-level effect can enter the common semantic model without creating a second language.

---

80. Effect identity versus effect implementation

An effect identity is stable semantic information.

An implementation may vary.

For example:

quantum::measurement

may be realized through:

quantum simulator
hardware quantum processor
hybrid execution
emulation
future quantum substrate

without changing the source-level effect.

Likewise:

accelerator::tensor_compute

may be realized through different accelerator architectures.

The source meaning remains independent of the implementation.

---

81. Error semantics

The effect subsystem must distinguish:

syntax invalidity
semantic invalidity
capability infeasibility
resource infeasibility
policy rejection
contract violation
target infeasibility
runtime failure

These are different stages.

For example:

valid syntax
    ↓
valid semantic effect
    ↓
required capability unavailable

must not be reported as:

invalid grammar

---

82. Feasibility and portability

POCO-REAF does not mean every program is physically executable on every machine.

It means that source semantics are not unnecessarily tied to a particular machine.

Therefore:

same source
    ↓
different target
    ↓
different realization

is valid when the target can satisfy the program's requirements.

If it cannot, the toolchain must report the specific unsatisfied requirement or capability.

It must not silently alter program meaning.

---

83. Resource-aware compilation

Effects may influence compilation decisions.

For example:

effect quantum::measurement

may influence:

- quantum lowering;
- measurement scheduling;
- resilience planning;
- capability negotiation.

But those decisions occur after semantic analysis.

The effect grammar must remain independent of the resulting decisions.

---

84. Optimization legality

Effect information may be consumed by optimization.

An optimizer may need to know whether an operation:

- has observable I/O;
- mutates state;
- performs measurement;
- communicates externally;
- invokes native code;
- performs randomness;
- performs learning;
- performs adaptation.

The effect grammar provides source information.

The semantic analyzer produces normalized effect information.

The optimizer consumes that semantic information.

---

85. Effect normalization

Semantic analysis should produce a canonical normalized effect representation.

Normalization may include:

name resolution
alias resolution
qualification
duplicate elimination
polymorphic substitution
inheritance resolution
composition
handler discharge
inference
canonical ordering where semantically appropriate

None of these should be performed by the ANTLR grammar itself.

---

86. Effect equality

Effect equality must be defined semantically.

Textual equality alone may not be sufficient when:

- aliases exist;
- imports exist;
- namespaces are resolved;
- dialects participate;
- polymorphism exists.

The grammar must preserve enough information for the semantic layer to perform canonical identity resolution.

---

87. Effect versioning

An effect may have semantic version information where the language supports it.

Versioning must not be interpreted as a physical machine version.

It may identify:

language effect definition
dialect effect definition
semantic API
interoperability contract

Target-specific versions remain outside the effect identity itself unless explicitly represented by a target/deployment subsystem.

---

88. Test architecture

The effect subsystem must have tests at multiple levels:

lexical
parser
AST
semantic
effect algebra
effect inference
effect polymorphism
handlers
capabilities
resources
contracts
policies
security
provenance
quantum
HDL
classical
distributed
networking
interoperability
simulation
scalability
determinism
compatibility
negative
boundary
cross-domain

No single parser test suite is sufficient for production readiness.

---

89. Required effect fixtures

The repository should maintain representative fixtures for:

effect-basic.zm
effect-qualified.zm
effect-set.zm
effect-operation.zm
effect-handler.zm
effect-polymorphic.zm
effect-custom.zm
effect-capability.zm
effect-resource.zm
effect-contract.zm
effect-policy.zm
effect-security.zm
effect-quantum.zm
effect-classical.zm
effect-distributed.zm
effect-network.zm
effect-hardware.zm
effect-learning.zm
effect-adaptation.zm
effect-ffi.zm
effect-simulation.zm
effect-cross-domain.zm
effect-poco-reaf.zm

The exact fixture location belongs to the repository's test organization.

---

90. Mandatory POCO-REAF effect test

A production conformance test must demonstrate that one source-level effectful program can be analyzed without embedding a target.

Conceptually:

source
  ↓
effects
  ↓
requirements
  ↓
capabilities
  ↓
resources
  ↓
policies
  ↓
semantic representation

The same source must remain target-neutral while downstream realization may differ.

The test must verify that no source-level effect construct contains:

physical device
physical qubit
fixed CPU
fixed GPU
fixed FPGA
fixed node
fixed memory
fixed topology
fixed accelerator

---

91. Mandatory quantum integration test

At least one test must verify:

source effect
    ↓
domain-neutral AST
    ↓
quantum semantic classification
    ↓
quantum::ir

The test must verify that the effect layer does not create:

physical qubit mapping
routing
scheduling
calibration
QEC
ZQN

inside the grammar subsystem.

---

92. Mandatory hybrid integration test

At least one test must combine:

classical computation
quantum operation
measurement
classical control
AI/learning or reasoning
effect declaration
capability requirement
resource requirement
contract
policy
provenance

The result must pass through the common semantic architecture.

---

93. Mandatory distributed integration test

At least one test must combine:

effect
actor/task
message
network
distributed execution
security policy
resource requirements

without encoding a fixed number of nodes.

---

94. Mandatory HDL integration test

At least one test must combine:

hardware-related effect
HDL construct
simulation
verification
resource/capability requirement

without making physical hardware capacity part of effect grammar.

---

95. Mandatory scalability test

A scalability test must verify that the grammar architecture remains structurally open for increasing:

effect count
effect-set size
qualified-name depth
handler count
generic effect count
program size
cross-domain composition

The test must not convert its test size into a language maximum.

---

96. Mandatory compatibility test

For every stabilized effect syntax:

old valid source
      ↓
new compiler
      ↓
same semantic interpretation

unless a documented breaking language change has intentionally occurred.

Deprecated syntax must have an explicit status and migration path.

---

97. Completion criteria for "grammar/effects/"

The effects subsystem is not production-ready merely because its ".g4" files exist.

It is production-ready only when:

[x] Effect ownership is explicit.
[x] Capability ownership is distinct.
[x] Resource ownership is distinct.
[x] Contract ownership is distinct.
[x] Policy ownership is distinct.
[x] Target realization is downstream.
[x] Effect identity is open-world.
[x] No machine-size ceiling is encoded.
[x] No physical device identity is encoded.
[x] No fixed quantum capacity is encoded.
[x] No fixed classical capacity is encoded.
[x] No fixed network capacity is encoded.
[x] No fixed distributed capacity is encoded.
[x] No effect-specific backend IR exists.
[x] quantum::ir remains the quantum boundary.
[x] Parsing is side-effect free.
[x] Safe Rust is sufficient.

The remaining repository implementation gates are complete only when:

[ ] Every public grammar rule has an AST mapping.
[ ] Every AST mapping has a semantic mapping.
[ ] Every executable semantic construct has an IR destination.
[ ] Rust lexer and modular grammar vocabulary agree.
[ ] Rust parser and modular grammar conformance is verified.
[ ] Effect declarations are semantically resolved.
[ ] Effect sets are normalized.
[ ] Effect inference is implemented where specified.
[ ] Effect polymorphism is implemented where specified.
[ ] Effect handlers are semantically checked.
[ ] Capability relationships are checked.
[ ] Resource relationships are checked.
[ ] Contract relationships are checked.
[ ] Policy relationships are checked.
[ ] Security relationships are checked.
[ ] Provenance is preserved.
[ ] Quantum effects reach quantum::ir where applicable.
[ ] Classical effects reach canonical classical semantics.
[ ] HDL effects reach HDL/hardware semantics.
[ ] Distributed effects reach distributed semantics.
[ ] Networking effects reach networking semantics.
[ ] FFI/native effects reach interoperability semantics.
[ ] Adaptation effects reach controlled execution semantics.
[ ] Positive tests pass.
[ ] Negative tests pass.
[ ] Boundary tests pass.
[ ] Cross-domain tests pass.
[ ] Scalability tests pass.
[ ] Determinism tests pass.
[ ] Compatibility tests pass.

---

98. Definition of DONE for an individual effect file

An individual file under this directory is considered DONE only when all of these are independently established:

PURPOSE
    The file's responsibility is unambiguous.

OWNS
    Every rule owned by the file is explicitly listed.

DOES NOT OWN
    Adjacent responsibilities are explicitly excluded.

DEPENDENCIES
    Every imported rule and semantic dependency is documented.

EXPORTS
    Every public rule is documented.

LEXER CONTRACT
    Every consumed token is known.

AST CONTRACT
    Every public parser construct maps to the canonical AST.

SEMANTIC CONTRACT
    Every construct has defined semantic meaning.

TYPE CONTRACT
    Type interactions are specified.

EFFECT CONTRACT
    Effect propagation/composition behavior is specified.

CAPABILITY CONTRACT
    Capability interaction is specified where relevant.

RESOURCE CONTRACT
    Resource interaction is specified where relevant.

CONTRACT CONTRACT
    Validation interaction is specified where relevant.

POLICY CONTRACT
    Policy interaction is specified where relevant.

PROVENANCE CONTRACT
    Source/semantic provenance requirements are specified.

IR CONTRACT
    Downstream representation is identified.

DOMAIN BOUNDARIES
    Quantum/HDL/hardware/classical/distributed boundaries are explicit.

DIAGNOSTICS
    Syntax and semantic diagnostic responsibilities are separated.

TESTS
    Positive, negative, boundary, cross-domain, scalability,
    determinism and compatibility coverage exists.

SAFETY
    No unsafe Rust is required.

SCALABILITY
    No artificial language-level ceiling exists.

COMPATIBILITY
    Version/deprecation behavior is defined.

INTEGRATION
    Upstream and downstream consumers are identified.

COMPLETION
    No undocumented dependency on another unfinished grammar file exists.

Once these conditions are met, modifying another effect grammar file should not require reopening the completed file merely to discover missing ownership or integration information.

---

99. What this README does not claim

This README is an architectural contract.

It does not falsely claim that every downstream component is already implemented.

In particular, the existence of grammar rules does not prove that:

AST
semantic analysis
effect inference
capability analysis
resource analysis
policy analysis
provenance
IR lowering
runtime execution

are already complete.

Implementation status must be tracked by the repository's conformance/status system.

---

100. Final architecture

The complete effect architecture is:

                 ZAMANI SOURCE
                       │
                       ▼
                CANONICAL LEXER
                       │
                       ▼
               EFFECT GRAMMAR
                       │
          ┌────────────┼─────────────┐
          │            │             │
          ▼            ▼             ▼
     declarations     sets       operations
          │            │             │
          └────────────┼─────────────┘
                       ▼
                    handlers
                       │
                       ▼
               domain-neutral AST
                       │
                       ▼
              semantic effect model
                       │
        ┌──────────────┼──────────────┐
        ▼              ▼              ▼
   capabilities     resources       policies
        │              │              │
        └──────────────┼──────────────┘
                       ▼
                   contracts
                       │
                       ▼
                   security
                       │
                       ▼
                  provenance
                       │
                       ▼
             canonical semantics
                       │
       ┌───────────────┼────────────────┐
       ▼               ▼                ▼
  classical       quantum::ir       HDL/hardware
       │               │                │
       └───────────────┼────────────────┘
                       ▼
                  optimization
                       │
                  specialization
                       │
                    lowering
                       │
              routing / scheduling
                       │
              resilience / recovery
                       │
                    QEC/ZQN
                       │
                      HAL
                       │
                       ▼
              AVAILABLE REALIZATION

---

101. Final POCO-REAF invariant

The effect subsystem must preserve this fundamental rule:

SOURCE SEMANTICS
        ≠
PHYSICAL REALIZATION

A Zamani effect describes the computational behavior required by the program.

The compiler and execution architecture determine how that behavior can be realized using the resources and capabilities available at compilation or execution time.

Therefore:

Program
   ↓
Effect intent
   ↓
Semantic meaning
   ↓
Requirements
   ↓
Capabilities
   ↓
Resources
   ↓
Policies
   ↓
Target-independent realization plan
   ↓
Target-specific realization

The source language must not become a catalogue of today's hardware.

The effect subsystem must remain extensible enough for tomorrow's hardware, execution models, scientific domains, AI systems, quantum systems, distributed systems, and computational substrates.

That is the required production architecture for "grammar/effects/".