Zamani Effects Grammar

Path: "grammar/effects/"
Language: Zamani
Status: Production architecture and conformance contract
Grammar technology: ANTLR4
Rust implementation baseline: Rust 1.97 / Rust 1.97.1, Edition 2021
Safety: Safe Rust only; "unsafe" is prohibited
Primary composition grammar: "grammar/effects/effects.g4"

---

1. Purpose

The "grammar/effects/" subsystem defines the source-language grammar boundary for computational effects in Zamani.

An effect describes a computational behavior, observable semantic consequence, or execution property that can participate in:

- function contracts;
- operation declarations;
- effect inference;
- effect checking;
- effect polymorphism;
- effect composition;
- effect handling;
- semantic analysis;
- capability analysis;
- resource analysis;
- security analysis;
- optimization legality;
- lowering;
- interoperability;
- execution planning.

The effect subsystem must remain independent of the machine on which a program is eventually executed.

The architecture is therefore:

Zamani source
    │
    ▼
lexical analysis
    │
    ▼
parser
    │
    ▼
domain-neutral AST
    │
    ▼
effect semantic analysis
    │
    ├── effect identity
    ├── effect sets
    ├── effect inference
    ├── effect polymorphism
    ├── handlers
    ├── capabilities
    ├── requirements
    ├── constraints
    └── resource analysis
    │
    ▼
canonical semantic representation / IR
    │
    ├── classical semantics
    ├── quantum::ir where quantum semantics are involved
    ├── HDL/hardware semantics
    ├── distributed semantics
    └── other domain representations
    │
    ▼
optimization / lowering
    │
    ├── routing
    ├── scheduling
    ├── resilience
    ├── QEC
    ├── ZQN
    └── HAL
    │
    ▼
target realization

The grammar describes what the program means.

It does not describe how a particular machine happens to realize it.

---

2. POCO-REAF requirement

Zamani is designed around:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever»

The effects subsystem is a foundational part of this architecture.

An effect must remain meaningful independently of whether the computation ultimately runs on:

- a tiny embedded processor;
- one CPU;
- many CPUs;
- a GPU;
- many accelerators;
- an FPGA;
- an ASIC;
- a simulator;
- a QPU;
- a hybrid classical/quantum system;
- a distributed system;
- an HPC system;
- a cloud deployment;
- an edge system;
- a future computational substrate.

The source language therefore expresses semantic intent rather than accidental properties of a particular target.

For example:

effect Quantum;

describes an effect identity.

It does not mean:

use QPU 0
use 32 physical qubits
use topology X
use vendor Y

Likewise:

effect Network;

does not select:

node 0
port 8080
interface eth0

Those are downstream realization concerns.

---

3. Non-negotiable architectural invariants

The following rules apply permanently.

3.1 Effects are semantic, not physical

An effect describes computational behavior.

It must not encode:

- CPU counts;
- core counts;
- thread counts;
- GPU counts;
- FPGA counts;
- accelerator counts;
- QPU counts;
- qubit counts;
- physical qubit identifiers;
- memory capacities;
- register widths;
- vector widths;
- tensor hardware dimensions;
- node counts;
- network topology;
- device IDs;
- physical addresses.

3.2 Effects are not capabilities

An effect describes what a computation does or may do.

A capability describes what an execution environment can provide.

Effect      = computation-side semantic behavior
Capability  = environment-side ability

They must not be collapsed.

3.3 Effects are not requirements

A requirement describes something execution must provide.

For example:

effect Quantum

and:

requires capability("quantum.measurement")

have different meanings.

3.4 Effects are not constraints

A constraint describes a condition that must be satisfied.

Effects may participate in constraint analysis but must not become the universal constraint language.

3.5 Effects are not resources

A resource describes something consumed, reserved, transferred, or otherwise managed.

An effect does not itself represent:

- memory;
- qubits;
- cores;
- devices;
- network links;
- storage;
- accelerator capacity.

3.6 Effects are not targets

An effect must not select:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- simulator;
- vendor;
- cloud provider;
- physical device.

3.7 Effects are not implementation decisions

Effect syntax must not encode routing, placement, scheduling, calibration, backend selection, or physical mapping.

---

4. Actual repository ownership

The effect directory currently contains the following established files:

grammar/effects/
├── README.md
├── capabilities.g4
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

These filenames should be retained unless a future repository-wide architectural decision demonstrates that a file is genuinely redundant.

No unnecessary rename is required.

---

5. Effect subsystem composition root

"grammar/effects/effects.g4"

This is the single composition root for the effect grammar subsystem.

It owns:

- composition;
- imports;
- dispatch integration;
- the effect subsystem boundary.

It does not redefine the detailed rules owned by the subordinate grammars.

The intended ownership is:

effects.g4
    │
    ├── effect-declarations.g4
    ├── effect-sets.g4
    ├── effect-operations.g4
    ├── effect-handling.g4
    ├── effect-types.g4
    ├── effect-polymorphism.g4
    ├── effect-composition.g4
    └── custom-effects.g4

Domain-specific files remain extensions of the semantic effect vocabulary rather than replacements for the generic effect model.

---

6. File responsibilities

6.1 "effect-declarations.g4"

Owns

- effect declarations;
- effect declaration names;
- effect generic parameters;
- effect parameters;
- effect signatures;
- effect operation declarations;
- declaration-local attributes.

Does not own

- effect use;
- invocation;
- handlers;
- effect-set normalization;
- capability resolution;
- resource allocation;
- hardware selection.

Integration

effect-declarations.g4
        ↓
AST effect declaration
        ↓
name resolution
        ↓
semantic effect identity

The native AST already contains a dedicated source-level effect declaration representation. The grammar must map into that structure rather than inventing another declaration model.

---

6.2 "effect-sets.g4"

Owns

- effect references;
- effect lists;
- effect sets;
- effect-set syntax;
- effect-set membership syntax.

Does not own

- semantic deduplication;
- canonical semantic ordering;
- effect inference;
- capability checking.

Required property

Effect-set cardinality must be open-ended.

There must be no grammar-level:

MAX_EFFECTS
MAX_EFFECT_SET_SIZE

---

6.3 "effect-operations.g4"

Owns

Source-level use/invocation syntax for effect operations.

It may represent:

- operation references;
- effect operation invocation;
- arguments;
- operation use;
- operation calls.

Does not own

- operation implementation;
- runtime dispatch;
- hardware calls;
- device selection;
- quantum gate lowering;
- QEC;
- ZQN.

An effect operation is source syntax.

Its implementation is determined later.

---

6.4 "effect-handling.g4"

Owns

- "handle" syntax;
- effect handler syntax;
- handler arms;
- handler patterns;
- handled computations;
- handler results;
- effect propagation/discharge syntax where supported.

Does not own

- runtime handler implementation;
- operating-system calls;
- device dispatch;
- scheduler behavior;
- backend behavior.

The AST must preserve handler structure for semantic analysis.

---

6.5 "effect-types.g4"

Owns

Effect qualification of existing type expressions.

It must not become a second type grammar.

It consumes the canonical type system and effect-set representation.

Conceptually:

typeExpression
    +
effect qualification

rather than:

effectTypes.g4
    → independent type language

This prevents divergence from "grammar/types/".

---

6.6 "effect-polymorphism.g4"

Owns

- effect variables;
- effect-polymorphic parameters;
- effect bounds;
- effect substitutions;
- effect-polymorphic constraints;
- effect-variable references.

Does not own

- ordinary generic type syntax;
- ordinary effect references;
- effect-set syntax.

It must reuse the canonical generic/type/effect infrastructure.

---

6.7 "effect-composition.g4"

Owns

Effect composition syntax.

It must delegate effect membership syntax to "effect-sets.g4".

It must not create a second effect algebra.

Semantic normalization belongs downstream.

---

6.8 "custom-effects.g4"

Owns

Extensible user/dialect-defined effect declarations or extension relationships.

Custom effects are essential for a language intended to survive future computational domains.

A future domain must not require modifying a closed enum inside the core grammar.

---

7. Domain-specific effect grammars

The repository also contains:

io.g4
quantum.g4
hardware.g4
network.g4
distributed.g4
security.g4
capabilities.g4

These must remain subordinate to the universal effect model.

They must not create parallel effect systems.

The generic grammar should support qualified effect identity such as:

quantum::measurement
hardware::signal
distributed::replication
networking::request
security::authorization
accelerator::compute
ai::inference
data::transform
future::domain::effect

The names are semantic identities.

They are not required to be hard-coded into the generic parser.

---

8. Open-world effect identity

The effect language must be open-world.

Do not make the fundamental grammar:

effectKind
    : IO
    | Network
    | Quantum
    | GPU
    | FPGA
    | QPU
    ;

That would require the grammar to be modified whenever Zamani gains:

- a new quantum technology;
- a new accelerator;
- a new security model;
- a new distributed model;
- a new AI execution model;
- a new hardware abstraction;
- a new scientific domain;
- a future computational substrate.

Instead, effect identity is represented through the language's canonical name/path system.

The semantic registry determines whether an effect is:

- standard;
- user-defined;
- imported;
- dialect-defined;
- experimental;
- deprecated;
- unknown.

---

9. Relationship to the actual Rust lexer

The repository's current Rust lexer defines effect-related lexical tokens, including:

KeywordEffect
KeywordHandle
KeywordPerform

The effect grammars must use the canonical lexer vocabulary rather than inventing competing token spellings.

The lexer owns:

- tokenization;
- keyword identity;
- punctuation;
- operators;
- source spans.

The effect grammar owns:

- arrangement of those tokens into effect syntax.

This distinction is mandatory.

---

10. Relationship to the actual Rust parser

The current "src/parser.rs" explicitly dispatches effect-related statements, including:

KeywordEffect → parse_effect_decl()
KeywordHandle  → parse_handle()

Therefore the effects grammar is not merely theoretical.

However, the current Rust parser and the modular ANTLR grammar are not automatically identical merely because both contain effect functionality.

The production conformance chain must be:

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

Any difference must be classified as:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

It must not be hidden by documentation.

---

11. AST integration

The native frontend already contains an effect AST subsystem, including effect declaration and effect-kind structures.

The effect grammar must preserve enough information to construct the existing AST contract.

At minimum, source information must survive for:

- effect declaration;
- effect identity;
- namespace/path;
- generic parameters;
- parameters;
- return type;
- effect references;
- effect sets;
- handlers;
- source spans;
- source ordering;
- provenance.

The grammar must not invent a second effect AST.

The direction is:

ANTLR parse tree
       ↓
existing frontend AST
       ↓
semantic effect model

not:

ANTLR
 ↓
new grammar-specific EffectIR
 ↓
another effect representation

---

12. Source AST versus semantic model

The source AST must preserve source structure.

Semantic analysis owns:

- name resolution;
- effect identity;
- effect inheritance;
- effect inference;
- effect compatibility;
- effect algebra;
- handler semantics;
- polymorphic substitution;
- capability relationships;
- resource requirements;
- security properties;
- domain interpretation.

The AST must not be mutated into a hardware/backend representation.

Semantic information that is derived from the AST should preferably be associated through the repository's existing semantic structures and stable node identities.

---

13. Effect declaration versus effect use

These concepts must remain distinct.

Declaration:

effect Logging;

Use/reference:

Logging

Invocation:

perform Logging(...)

Handling:

handle ...

Type qualification:

Type with effect { Logging }

Polymorphism:

E

where "E" represents an effect variable under the language's effect-polymorphism rules.

These must not collapse into one grammar rule or one AST node.

---

14. Effect inference

Zamani may infer effects from computation.

For example, a function may omit an explicit effect declaration when inference is supported:

fn compute() {
    ...
}

The semantic analyzer may derive the resulting effect set from the body.

Explicit effect declarations remain valuable for:

- API contracts;
- verification;
- security analysis;
- optimization;
- documentation;
- interoperability;
- static checking.

The grammar itself does not perform inference.

---

15. Effect polymorphism

Effect polymorphism is required for generic, reusable programs.

Conceptually:

computation<T, E>

may be parameterized by an effect variable.

This enables reusable source semantics without requiring one copy of the program for every execution environment.

For example, a generic algorithm should not have to be rewritten separately for:

CPU
GPU
FPGA
QPU
distributed cluster
future accelerator

simply because its effect realization differs.

The effect grammar therefore provides the syntax for abstraction, while semantic analysis determines the actual effect relationships.

---

16. Effect handling

A handler changes how an effect is interpreted at a semantic boundary.

The grammar must represent the structure of:

computation
    ↓
handler
    ↓
handled effect
    ↓
result

The handler must not directly encode:

- OS system calls;
- vendor APIs;
- GPU identifiers;
- QPU identifiers;
- FPGA identifiers;
- physical addresses;
- network interfaces;
- device queues.

Those are implementation concerns.

---

17. Effect composition

Effect composition must be deterministic.

For semantic effect sets:

{ IO, Quantum }

and:

{ Quantum, IO }

must have the same meaning when the effect algebra defines sets as unordered.

The parser should preserve source order for diagnostics and source fidelity.

Semantic normalization may establish canonical identity/order later.

The parser must not depend on:

- hash-map iteration order;
- machine state;
- target state;
- hardware discovery;
- runtime scheduling.

---

18. Effect identity and namespaces

Effect names must use the canonical Zamani naming/path system.

Examples:

IO
security::Audit
quantum::Measurement
hardware::Signal
distributed::Replication
future::photonic::Interaction
vendor::extension::Effect

"grammar/effects/" must not create another qualified-name grammar.

The canonical source of:

- identifiers;
- qualified names;
- paths;
- namespaces

remains the shared grammar infrastructure.

---

19. Capabilities

The current repository contains an effect-local "capabilities.g4".

This file requires special treatment.

There must be one canonical semantic capability model across:

grammar/core/
grammar/resources/
grammar/hardware/
grammar/effects/
grammar/quantum/

The effect subsystem may reference capabilities.

It must not create a second capability type system.

The semantic relationship is:

Effect
   │
   │ computation performs
   ▼
semantic effect
   │
   ├──────────────┐
   │              │
   ▼              ▼
Requirement    Capability
   │              │
   └──────┬───────┘
          ▼
   feasibility analysis

That matching happens downstream.

---

20. Resources

Effect syntax must never silently become resource syntax.

For example:

effect Quantum

does not mean:

requires N qubits

Likewise:

effect Accelerator

does not mean:

requires one GPU

Resource requirements belong to the resource/requirement system.

This distinction is essential for POCO-REAF.

---

21. Requirements, constraints, preferences and hints

Zamani must preserve the following distinction:

Concept| Meaning
Effect| What the computation does/may do
Capability| What the environment can provide
Requirement| What execution needs
Constraint| What execution must satisfy
Resource| What execution consumes/reserves
Preference| What realization is preferred
Hint| Optimization guidance
Target| Intended realization domain
Placement| Concrete realization decision

The effects grammar must not merge these concepts.

---

22. Quantum integration

Quantum-related effects may appear in the effect system.

Examples conceptually include:

quantum::measurement
quantum::reset
quantum::state
quantum::dynamic_control

These are source-level semantic identities.

They must not encode:

physical qubit 0
physical qubit 1
QPU 0
32-qubit machine
specific coupling map
specific gate set
specific calibration

The correct downstream path remains:

Zamani source
    ↓
effect-aware AST
    ↓
semantic analysis
    ↓
quantum semantics
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
target

The effects subsystem must not create a second quantum IR.

---

23. Quantum gate/operation independence

The effect subsystem must not become a closed catalogue of quantum gates.

It must not encode:

H
X
Y
Z
CNOT
...

as the fundamental effect vocabulary.

Quantum operation identity belongs to the quantum operation model and eventually to canonical quantum semantics.

Effect syntax may express that a computation has quantum behavior without enumerating every operation supported by every present or future device.

---

24. QEC boundary

Quantum error correction remains owned by the QEC subsystem.

Effects may communicate semantic information relevant to resilience or error handling.

They must not implement:

- code distance;
- syndrome extraction;
- decoder algorithms;
- correction schedules;
- physical layout;
- decoder-specific state;
- QEC resource allocation.

The direction is:

effect semantics
      ↓
semantic analysis
      ↓
quantum::ir / resilience metadata
      ↓
QEC

not:

effect grammar
      ↓
QEC implementation

---

25. ZQN boundary

ZQN owns quantum noise/fault semantics.

The effects grammar must not duplicate:

- noise models;
- fault models;
- leakage;
- loss;
- erasure;
- correlated faults;
- drift;
- fault classification;
- mitigation implementation.

Effect syntax may identify a semantic relationship with noise/fault-sensitive computation, but ZQN remains the authority for those semantics.

---

26. Hardware boundary

"hardware.g4" must remain a semantic extension.

An effect such as:

hardware::interaction

does not identify:

CPU 0
GPU 1
FPGA 2
device address X

Hardware discovery belongs downstream.

Hardware capabilities are supplied by the execution environment.

---

27. Distributed computing boundary

"distributed.g4" may provide effect identities such as:

distributed::remote_execution
distributed::replication
distributed::consistency
distributed::communication

but must not encode a universal machine topology.

No:

node[0]
node[1]
node[2]
MAX_NODES

as universal language limitations.

The source describes distributed semantics; deployment determines realization.

---

28. Networking boundary

"network.g4" may represent networking-related semantic effects.

It must not make:

- IP addresses;
- ports;
- interfaces;
- routing tables;
- topology;
- link counts

part of the generic effect model.

Concrete network realization belongs to networking, deployment, resource, and runtime layers.

---

29. Security boundary

"security.g4" may represent semantic effects associated with:

- authorization;
- authentication;
- confidentiality;
- integrity;
- cryptographic operations;
- identity;
- secure computation.

It must not silently grant authority.

An effect such as:

security::authorization

does not itself authorize an operation.

Authorization remains a semantic/security decision.

---

30. IO boundary

"io.g4" describes IO-related semantic syntax where specialized syntax is required.

It must not perform IO.

The parser must never:

- open a file;
- inspect a filesystem;
- read environment variables;
- contact a network;
- invoke an external command.

The grammar is declarative.

---

31. No runtime behavior in grammar

The effect grammars must contain no embedded runtime behavior.

Prohibited:

filesystem access
network access
hardware discovery
environment inspection
process execution
device enumeration
randomness
runtime callbacks

The grammar produces syntax.

Semantic analysis interprets it.

The runtime executes lowered semantics.

---

32. Determinism

Parsing must be deterministic.

Given:

same source
+
same grammar version
+
same lexical configuration
+
same explicit dialect configuration

the parser must produce equivalent syntax structures.

Parsing must not depend on:

- time;
- randomness;
- hardware availability;
- filesystem state;
- network state;
- environment state;
- target selection;
- runtime scheduling.

---

33. Scalability

The effect grammar must scale from a single effect to arbitrarily large programs and effect sets subject only to actual implementation resources.

There must be no language-level limits such as:

MAX_EFFECTS
MAX_EFFECT_OPERATIONS
MAX_EFFECT_PARAMETERS
MAX_EFFECT_GENERICS
MAX_HANDLER_ARMS
MAX_EFFECT_DEPTH
MAX_QUANTUM_EFFECTS
MAX_HARDWARE_EFFECTS

A compiler may have configurable resource-protection policies.

Those policies must not become language semantics.

This distinction is essential:

language expressiveness
        ≠
compiler resource budget

---

34. "Infinity" interpretation

"Infinity" in POCO-REAF does not mean that finite hardware literally contains infinite memory or infinite execution capacity.

It means the language introduces no artificial machine-size ceiling.

The effective limit is determined by:

program size
+
compiler representation
+
available memory
+
available compute
+
target resources
+
execution resources

rather than by arbitrary grammar constants.

---

35. Hard-coding audit

Every effect grammar file must pass a hard-coding audit.

Search for suspicious constructs including:

MAX_EFFECT
MAX_EFFECTS
MAX_EFFECT_OPERATIONS
MAX_EFFECT_PARAMETERS
MAX_EFFECT_GENERICS
MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_ACCELERATORS
MAX_NODES
MAX_MEMORY
MAX_DEVICES
DEVICE_0
GPU_0
QPU_0
CPU_0
NODE_0
q[0]
q[1]

An occurrence is acceptable only when it is clearly:

- documentation;
- a test fixture;
- an example value;
- a diagnostic;
- an explicit compiler resource policy;
- a genuine source-language semantic constant.

It is forbidden when it creates an artificial universal language limit.

---

36. Existing grammar hard-coding must be removed

The current effect grammar files contain extensive documentation around maximum-cardinality prohibitions, which is useful, but production readiness requires that the actual parser rules and semantic contracts also obey those rules.

For example, "effect-operations.g4" must not introduce fixed operation counts or argument limits merely because documentation says not to.

The implementation must be checked, not merely documented.

---

37. Dependency direction

The effect grammar dependency graph must remain acyclic.

The intended direction is:

Shared lexical vocabulary
        ↓
Core parser grammar
        ↓
Types / Expressions / Declarations
        ↓
Effects
        ↓
Domain-neutral AST
        ↓
Semantic effect system
        ↓
Canonical semantic representation
        ↓
Compiler / runtime / domain backends

Within the effect subsystem:

Effects
 ├── declarations
 ├── sets
 ├── operations
 ├── handling
 ├── types
 ├── polymorphism
 ├── composition
 └── custom effects

Subordinate grammars must not import the composition root in a way that creates cycles.

---

38. Domain extension rule

A new computational domain must not require changing the generic effect syntax merely because the domain is new.

For example, future domains could introduce:

photonic::interaction
neuromorphic::spike
molecular::reaction
optical::transform
biological::signal
future::computation

using the existing open-world identity model.

The generic effect subsystem should remain unchanged unless the language semantics themselves require a genuinely new effect-system construct.

---

39. Dialect integration

Dialects may introduce additional effect identities.

A dialect must declare:

- dialect name;
- version;
- namespace;
- effect identities;
- syntax extensions, if any;
- semantic interpretation;
- AST mapping;
- compatibility policy;
- feature status.

A dialect must not silently redefine the meaning of a stable core effect.

A dialect also must not become an independent programming language hidden inside "grammar/effects/".

---

40. AST integration contract

For every effect construct, the mapping must be predetermined:

grammar rule
    ↓
AST representation
    ↓
semantic effect construct
    ↓
canonical representation / IR

Examples:

effect declaration
    ↓
Effect AST node
    ↓
semantic effect declaration

effect reference
    ↓
effect reference AST
    ↓
resolved EffectId

effect set
    ↓
effect-set AST
    ↓
normalized semantic effect set

handler
    ↓
handler AST
    ↓
handler semantic model

The grammar file must not require a future redesign of the AST to become semantically meaningful.

---

41. Canonical effect semantic model

The semantic layer should conceptually support:

EffectId
EffectName
EffectReference
EffectDeclaration
EffectOperation
EffectSet
EffectVariable
EffectConstraint
EffectContext
EffectHandler
EffectSubstitution

These are semantic/compiler concepts.

They must not be duplicated as independent grammar-specific IR structures.

---

42. Source spans

Every effect construct must retain sufficient source-location information for diagnostics.

At minimum, diagnostics must be able to identify:

- declaration span;
- effect name span;
- effect reference span;
- effect-set span;
- operation span;
- handler span;
- parameter span;
- relevant related source locations.

The grammar must not discard source structure needed by the frontend AST.

---

43. Diagnostics integration

The existing:

grammar/effects/effect-diagnostics.g4
grammar/effects/effect-diagnostics.md

must remain separate from the grammar's semantic ownership.

"effect-diagnostics.g4" may describe syntax needed for diagnostic-related grammar constructs if such constructs genuinely exist.

"effect-diagnostics.md" defines the diagnostic contract.

Diagnostics must distinguish at least:

lexical
syntax
name resolution
effect identity
effect typing
effect composition
effect polymorphism
effect handling
capability
requirement
constraint
resource
security
compatibility
implementation

Diagnostics must not confuse:

unknown effect

with:

unsupported hardware

Those are different failures at different architectural layers.

---

44. Diagnostic stability

Diagnostic identifiers must be stable.

Do not rely on source-file line numbers or list position as diagnostic identity.

A diagnostic should conceptually contain:

stable code
severity
primary span
message
related spans
structured data
optional suggestion

Presentation belongs to tooling.

The semantic diagnostic identity must remain stable across:

- CLI;
- IDE;
- LSP;
- JSON;
- machine-readable output;
- human-readable output.

---

45. Error recovery

Parser error recovery must not turn malformed effect syntax into silently valid semantics.

For example, malformed:

effect Foo {

must produce a recoverable syntax error while preserving enough source structure for subsequent diagnostics where possible.

Recovery must not invent:

- effect names;
- effect parameters;
- capabilities;
- resources;
- handlers.

---

46. Positive test contract

The effect subsystem requires positive tests for:

Declarations

effect IO;
effect Quantum;
effect security::Audit;

Generic declarations

effect Read<T>;

Effect sets

{ IO }
{ IO, Network }
{ Quantum, Security, Network }

Custom effects

application::Telemetry
future::domain::Effect
vendor::extension::Effect

Handlers

Valid handler forms defined by the canonical handler grammar.

Polymorphism

Valid effect variables and effect bounds.

Type qualification

Valid effect-qualified types.

Cross-domain use

Examples combining:

classical + quantum
quantum + hardware
quantum + distributed
classical + networking
AI + accelerator
HDL + hardware
quantum + security

---

47. Negative test contract

Negative tests must cover:

- missing effect names;
- malformed names;
- malformed qualified names;
- malformed effect sets;
- malformed parameters;
- malformed generic parameters;
- invalid handler syntax;
- invalid effect qualification;
- invalid polymorphic syntax;
- illegal composition;
- malformed operation invocation;
- invalid delimiters;
- unexpected tokens;
- invalid nesting.

Semantic negative tests must separately cover:

- unknown effect;
- duplicate declaration;
- unresolved effect;
- invalid effect parameter;
- incompatible effect composition;
- invalid handler;
- unsatisfied effect constraint;
- illegal effect escape;
- invalid effect substitution.

---

48. Boundary tests

Boundary tests must include:

- zero effects where the grammar permits an empty set;
- one effect;
- many effects;
- deeply qualified names;
- long identifiers;
- many parameters;
- many generic parameters;
- deeply nested generic types;
- deeply nested handler structures;
- large source files;
- large effect sets.

The test suite must not use a small number such as "8", "16", "32", or "1024" as an implicit language limit.

Those values may be test fixtures, but tests must also scale beyond them.

---

49. Scalability tests

Scalability tests must vary independently:

program size
effect-set size
number of declarations
number of handlers
number of generic parameters
number of operations
number of modules
resource requirements
qubit counts
device counts
node counts
thread counts
memory requirements

The effect grammar must remain independent of all physical quantities.

For example, changing:

1 qubit

to:

many qubits

must not require changing the effect grammar.

---

50. Determinism tests

For identical source and grammar configuration:

lex(source)

must be deterministic.

Then:

parse(source)

must be deterministic.

Then:

build_ast(parse_tree)

must preserve equivalent effect structure.

Semantic normalization must also be deterministic.

No effect result may depend on:

- hash-map ordering;
- machine identity;
- hardware discovery order;
- current time;
- randomness;
- network responses.

---

51. Round-trip tests

Where formatter/printer support exists:

source
  ↓
parse
  ↓
AST
  ↓
format
  ↓
parse

must preserve effect semantics.

The round trip must preserve:

- effect identity;
- qualification;
- effect membership;
- generic parameters;
- operation identity;
- handler structure;
- semantic effect relationships.

Formatting may change whitespace and equivalent presentation.

---

52. Compatibility tests

Effect syntax must participate in the repository-wide compatibility system.

Compatibility must compare:

specification
    ↕
Zamani.g4 / parser composition
    ↕
lexer
    ↕
Rust parser
    ↕
AST
    ↕
semantic model
    ↕
IR

Every stable effect feature must have a compatibility status.

Breaking changes require:

1. version identification;
2. migration documentation;
3. compatibility tests;
4. deprecation where appropriate;
5. semantic justification.

---

53. Feature lifecycle

An effect feature must follow:

proposal
   ↓
semantic design
   ↓
AST contract
   ↓
canonical grammar
   ↓
Rust/frontend implementation
   ↓
semantic implementation
   ↓
IR integration
   ↓
compiler integration
   ↓
runtime/backend integration
   ↓
positive tests
   ↓
negative tests
   ↓
boundary/scalability tests
   ↓
compatibility tests
   ↓
stable

"Zamani-Grammar.md" may document a proposal.

That does not automatically make it legal Zamani syntax.

---

54. Implementation status

The repository must distinguish:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

The effects README must never imply that all documented grammar constructs are already accepted by the Rust frontend.

In particular, the modular grammar and the current recursive-descent parser must be checked for conformance feature-by-feature.

---

55. Rust 1.97 / 1.97.1 contract

All Rust code consuming this grammar must target:

Rust 1.97
Rust 1.97.1
Edition 2021

No nightly-only language feature may be required.

No "unsafe" implementation is permitted.

The consuming crate/module must enforce safe Rust, preferably at the crate boundary with:

#![forbid(unsafe_code)]

where consistent with the existing crate architecture.

The grammar itself must contain no embedded Rust actions requiring unsafe behavior.

---

56. Security requirements

Parsing effect syntax must be safe against untrusted source.

The grammar must not:

- execute source code;
- execute shell commands;
- access files;
- access secrets;
- access network services;
- discover devices;
- inspect hardware;
- invoke runtime APIs;
- load arbitrary plugins;
- perform target selection.

Effect parameters are syntax until semantic analysis determines their meaning.

---

57. Resource exhaustion

The language must not encode arbitrary limits merely to protect one implementation.

However, compilers may implement explicit resource-protection policies for hostile or enormous inputs.

Those policies must be:

implementation policy

not:

language semantics

For example:

compiler invocation --max-parse-memory ...

is conceptually different from:

Zamani language supports at most 1024 effects

The first protects an implementation.

The second changes the language.

---

58. No hidden target dependence

The effect grammar must produce equivalent source semantics regardless of whether the compiler eventually discovers:

CPU
GPU
FPGA
ASIC
QPU
cluster
cloud
edge
future hardware

The target can influence downstream realization.

It must not alter the meaning of the source effect.

---

59. Effect-aware optimization

Effects provide semantic information that can restrict transformations.

For example, an optimizer may not freely reorder two operations when their effects establish an observable ordering relationship.

The grammar does not implement the optimizer.

The integration is:

effect syntax
      ↓
effect AST
      ↓
effect semantics
      ↓
optimization legality information
      ↓
optimizer

This allows optimization without coupling the grammar to a specific optimizer.

---

60. Effect-aware concurrency

Effects may interact with:

- async execution;
- parallelism;
- synchronization;
- shared state;
- distributed execution.

The grammar should expose only the semantic constructs required to express the relationship.

Concurrency scheduling remains owned by the concurrency/execution/compiler layers.

---

61. Effect-aware hardware/software co-design

Effects are useful for describing semantic interaction between software and hardware.

For example:

hardware::interaction
accelerator::compute
hdl::event
memory::shared

can describe semantic behavior.

They must not decide:

FPGA #3
GPU #7
memory bank #2
physical address 0x...

Those are realization decisions.

---

62. Effect-aware AI/data computation

Effects may represent semantic properties such as:

ai::training
ai::inference
data::stream
data::persistence
accelerator::compute

but the grammar must not become a framework-specific language for:

- CUDA;
- ROCm;
- PyTorch;
- TensorFlow;
- a particular accelerator vendor.

Framework interoperability belongs under the interoperability/dialect layers.

---

63. Interoperability

Effect semantics may be translated to external representations.

Examples include:

QIR
OpenQASM
LLVM-based representations
MLIR-based representations
HDL
foreign function interfaces
runtime APIs

Those are interoperability targets.

They are not the canonical Zamani effect model.

---

64. Canonical quantum boundary

Whenever an effect participates in quantum computation, the canonical semantic boundary remains:

quantum::ir

No effect-specific quantum IR may be introduced.

The correct relationship is:

Effect AST
      ↓
semantic effect model
      ↓
quantum semantic analysis
      ↓
quantum::ir

The effect grammar must not import or depend directly on the implementation of "quantum::ir".

---

65. No circular architecture

Forbidden:

grammar/effects
    ↓
quantum::ir
    ↓
grammar/effects

Forbidden:

effects grammar
    ↓
runtime
    ↓
effects grammar

Forbidden:

effects grammar
    ↓
hardware discovery

The dependency direction must always proceed toward later semantic realization.

---

66. Completion contract for each file

Every effect grammar file is considered complete only when all of the following have been established.

Purpose

What exact syntax does the file own?

Non-ownership

What syntax and semantics must remain elsewhere?

Inputs

Which canonical grammar rules/tokens does it consume?

Outputs

Which parse structures does it produce?

AST contract

Which existing AST structures consume those parse structures?

Semantic contract

What semantic analysis consumes them?

IR contract

What canonical representation receives the resulting semantics?

Compiler integration

Which compiler stages consume the semantic information?

Runtime integration

Which runtime/backend stage may eventually consume it?

Cross-domain integration

How does it interact with classical, quantum, HDL, distributed, AI, data, networking, and security domains?

Diagnostics

What syntax and semantic failures can originate from the construct?

Tests

It must have:

- positive tests;
- negative tests;
- boundary tests;
- scalability tests;
- determinism tests;
- compatibility tests;
- cross-domain tests where applicable.

Hard-coding audit

The file must not introduce universal machine limits.

Security audit

The file must remain declarative and safe.

Completion criteria

The complete integration chain must be known before the file is considered complete.

This is the mechanism that allows a file to be completed independently without waiting for another file to be redesigned later.

---

67. Independent-first implementation order

The effect subsystem should be completed in dependency order.

Stage 1 — shared contracts

Confirm:

lexer vocabulary
core names
qualified names
attributes
types
expressions
source spans
diagnostic model

Stage 2 — effect identity

Complete:

effect-declarations.g4
effect-sets.g4

These establish the foundation.

Stage 3 — effect use

Complete:

effect-operations.g4

Stage 4 — effect handling

Complete:

effect-handling.g4

Stage 5 — effect qualification

Complete:

effect-types.g4

Stage 6 — polymorphism

Complete:

effect-polymorphism.g4

Stage 7 — composition

Complete:

effect-composition.g4

Stage 8 — extension

Complete:

custom-effects.g4

Stage 9 — domain extensions

Validate:

io.g4
quantum.g4
hardware.g4
network.g4
distributed.g4
security.g4
capabilities.g4

against the generic effect model.

Stage 10 — composition

Finalize:

effects.g4

against all independently completed contracts.

Stage 11 — diagnostics

Finalize:

effect-diagnostics.g4
effect-diagnostics.md

Stage 12 — repository conformance

Validate:

Zamani.g4
lexer
Rust parser
AST
semantic analysis
IR
compiler
runtime
tests

---

68. Definition of production readiness

The effects subsystem is production-ready only when all of these are true:

- [ ] "effects.g4" is the single effect composition root.
- [ ] Every subordinate grammar has one clear owner.
- [ ] No duplicate effect grammar authority exists.
- [ ] Effect declarations are defined.
- [ ] Effect references are defined.
- [ ] Effect sets are defined.
- [ ] Effect operations are defined.
- [ ] Effect handling is defined.
- [ ] Effect qualification is defined.
- [ ] Effect polymorphism is defined.
- [ ] Effect composition is defined.
- [ ] Custom effects are defined.
- [ ] Domain effects use the generic effect model.
- [ ] Effect identity is open-world.
- [ ] Qualified names use canonical repository naming rules.
- [ ] The lexer is the sole lexical authority.
- [ ] The Rust parser has a conformance mapping.
- [ ] The AST mapping is defined.
- [ ] Semantic mapping is defined.
- [ ] IR mapping is defined.
- [ ] Capability semantics remain separate.
- [ ] Requirement semantics remain separate.
- [ ] Constraint semantics remain separate.
- [ ] Resource semantics remain separate.
- [ ] Target selection remains separate.
- [ ] Hardware topology remains downstream.
- [ ] Quantum semantics remain backend-independent.
- [ ] "quantum::ir" remains the canonical quantum boundary.
- [ ] QEC is not duplicated.
- [ ] ZQN is not duplicated.
- [ ] Routing is not duplicated.
- [ ] Scheduling is not duplicated.
- [ ] HAL is not duplicated.
- [ ] Runtime implementation is not embedded in grammar.
- [ ] No universal machine-size limit is encoded.
- [ ] No fixed qubit limit is encoded.
- [ ] No fixed CPU/GPU/FPGA/QPU limit is encoded.
- [ ] No fixed distributed-node limit is encoded.
- [ ] No fixed memory or tensor limit is encoded.
- [ ] No device identifiers are required by generic effects.
- [ ] Parsing is deterministic.
- [ ] Source spans are preserved.
- [ ] Diagnostics are structured and stable.
- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Boundary tests exist.
- [ ] Scalability tests exist.
- [ ] Determinism tests exist.
- [ ] Cross-domain tests exist.
- [ ] Round-trip tests exist where formatting support exists.
- [ ] Compatibility tests exist.
- [ ] Dialect integration is defined.
- [ ] Interoperability is defined.
- [ ] Rust 1.97/1.97.1 compatibility is tested.
- [ ] Edition 2021 compatibility is maintained.
- [ ] "unsafe" is prohibited.
- [ ] Security review passes.
- [ ] Hard-coding audit passes.
- [ ] No circular dependency exists.
- [ ] Documentation matches actual implementation status.
- [ ] POCO-REAF invariants remain intact.

---

69. Final architecture

The completed effects subsystem fits into Zamani as follows:

                         ZAMANI SOURCE
                              │
                              ▼
                         Zamani Lexer
                              │
                              ▼
                         Zamani Parser
                              │
                              ▼
                    Domain-Neutral AST
                              │
                              ▼
                    Effect AST Structures
                              │
                              ▼
                    Effect Semantic Model
                              │
             ┌────────────────┼────────────────┐
             │                │                │
             ▼                ▼                ▼
           Types        Capabilities      Requirements
             │                │                │
             └────────────────┼────────────────┘
                              │
                              ▼
                    Constraint Analysis
                              │
                              ▼
                    Canonical Semantic IR
                              │
                 ┌────────────┼────────────┐
                 │            │            │
                 ▼            ▼            ▼
            Classical      quantum::ir   HDL/Hardware
                 │            │            │
                 └────────────┼────────────┘
                              │
                              ▼
                         Optimization
                              │
                 ┌────────────┼────────────┐
                 │            │            │
                 ▼            ▼            ▼
              Routing     Scheduling   Resilience
                 │            │            │
                 └────────────┼────────────┘
                              │
                    ┌─────────┴─────────┐
                    │                   │
                    ▼                   ▼
                   QEC                 ZQN
                    │                   │
                    └─────────┬─────────┘
                              │
                              ▼
                             HAL
                              │
                              ▼
                       Target Realization
                              │
          ┌──────────┬────────┼────────┬──────────┐
          ▼          ▼        ▼        ▼          ▼
         CPU        GPU      FPGA     QPU     Distributed
          │          │        │        │          │
          └──────────┴────────┴────────┴──────────┘
                              │
                              ▼
                           Runtime

The effect subsystem therefore sits at the semantic boundary, not at the physical-machine boundary.

---

70. Final architectural invariant

The following distinction is permanent:

Effect
    = what the computation does or may do

Capability
    = what the environment can provide

Requirement
    = what execution needs

Constraint
    = what execution must satisfy

Resource
    = what execution consumes, reserves, or manages

Preference
    = what realization is preferred

Hint
    = information supplied to improve realization

Target
    = intended realization domain

Placement
    = concrete realization decision

Runtime
    = actual execution

No effect grammar file may collapse these concepts.

---

71. Final POCO-REAF statement

The purpose of "grammar/effects/" is not to describe today's computers.

It is to provide a stable semantic language for describing computational effects that can survive changes in:

- machine size;
- processor architecture;
- accelerator architecture;
- quantum technology;
- hardware topology;
- memory architecture;
- distributed topology;
- runtime;
- compiler;
- backend;
- vendor;
- deployment environment;
- future computational substrate.

The fundamental invariant is:

Zamani source
    ↓
portable semantic intent
    ↓
effect analysis
    ↓
canonical semantic representation
    ↓
target-independent optimization
    ↓
target-aware realization

Therefore:

«Effects describe computation, not the accidental properties of the machine executing it.»

That is the required foundation for:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)»

and for the broader Zamani objective:

«Scale from atom to everywhere, subject to the resources actually available, without turning today's physical limits into tomorrow's language limits.»