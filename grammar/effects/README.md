Zamani Effects Grammar

Status

Production specification

Path: "grammar/effects/"

Language: Zamani

Grammar technology: ANTLR

Compiler/runtime implementation language: Rust 1.97 / Rust 1.97.1

Safety requirement: Rust implementation must use "#![forbid(unsafe_code)]" and contain no "unsafe" code.

Architectural role: Source-language syntax for declaring, composing, constraining, handling, and propagating computational effects.

---

1. Purpose

The "grammar/effects/" subsystem defines the Zamani source-language syntax for effects.

An effect describes a semantically observable property of an operation, function, computation, module, or execution path that may affect how the computation is analyzed, lowered, optimized, scheduled, executed, verified, or authorized.

Effects allow Zamani to express computations involving, for example:

- input/output;
- mutation;
- allocation;
- quantum operations;
- hardware interaction;
- networking;
- distributed execution;
- synchronization;
- nondeterminism;
- external state;
- cryptographic operations;
- security-sensitive operations;
- accelerator interaction;
- custom language or domain effects.

Effects are part of the language semantics.

They are not themselves:

- hardware resources;
- hardware capabilities;
- resource requirements;
- target descriptions;
- scheduling decisions;
- routing decisions;
- optimization decisions;
- quantum IR;
- QEC algorithms;
- ZQN noise models;
- runtime implementations;
- device identifiers;
- physical topology descriptions.

The grammar therefore describes what effects a computation may have, while downstream compiler and runtime layers determine how those effects are implemented.

---

2. Core architectural principle

Zamani follows:

«Program semantics first; implementation resources second.»

An effect declaration must describe semantic behavior rather than accidentally encode a particular machine.

For example, source code may express:

effect Quantum

or:

effect IO

without implying:

- a fixed number of qubits;
- a particular QPU;
- a particular CPU;
- a particular GPU;
- a fixed network;
- a fixed memory capacity;
- a fixed topology;
- a fixed device;
- a fixed operating system.

Consequently, the effects grammar MUST NOT introduce machine-size constants or target-specific assumptions.

---

3. Ownership

The "grammar/effects/" subsystem owns the syntax of:

1. effect declarations;
2. effect names;
3. effect sets;
4. effect annotations;
5. effect application;
6. effect requirements attached to language constructs;
7. effect propagation syntax;
8. effect handling syntax;
9. effect aliases where supported;
10. effect composition;
11. effect subtraction/removal syntax where supported;
12. effect polymorphism;
13. effect parameters;
14. built-in semantic effect categories;
15. user-defined effect namespaces;
16. effect-related source metadata.

---

4. Non-ownership

"grammar/effects/" MUST NOT own the implementation or semantics of:

4.1 Capabilities

Capabilities belong to:

grammar/core/capabilities.g4
grammar/effects/capabilities.g4
grammar/resources/capabilities.g4

where the distinction between effect and capability is maintained.

An effect says:

«this computation performs or may perform X.»

A capability says:

«this execution environment can provide X.»

These must never become synonymous.

---

4.2 Requirements

Requirements belong to the requirement model.

An effect may contribute to semantic analysis of requirements, but an effect declaration must not redefine the requirement system.

---

4.3 Constraints

Constraints belong to the constraint system.

An effect may be constrained, but effect syntax must not become a general-purpose constraint language.

---

4.4 Resources

Resources belong to:

grammar/resources/

For example:

effect Quantum

does not mean:

requires 100 qubits

and:

effect GPU

does not mean:

requires device 0

---

4.5 Quantum IR

Quantum syntax eventually lowers through the repository's canonical:

quantum::ir

The effect grammar MUST NOT create an alternative quantum representation.

For example:

effect Quantum

is source-level semantic information.

It is not a quantum IR node.

---

4.6 QEC

Quantum error correction belongs to the repository's QEC subsystem.

The effects grammar may express that a computation has an error-correction-related effect where such semantics are intentionally part of the language, but it must not define:

- stabilizer algorithms;
- decoder algorithms;
- code distance;
- syndrome extraction implementation;
- correction schedules;
- decoder configuration.

---

4.7 ZQN

ZQN owns quantum noise and fault semantics.

Effects may identify that an operation interacts with noise-sensitive or fault-aware execution, but effects must not duplicate ZQN's:

- noise models;
- fault models;
- fault classification;
- correlated faults;
- leakage semantics;
- loss semantics;
- erasure semantics.

---

4.8 Scheduling

Effects may influence scheduling legality.

Effects grammar does not own:

- operation ordering;
- timing;
- resource allocation;
- ASAP/ALAP;
- critical paths;
- scheduling policies;
- pulse scheduling;
- dynamic scheduling.

Those belong to "quantum/scheduling" and related compiler infrastructure.

---

4.9 Optimization

Effects can restrict transformations.

The effects grammar does not own optimization algorithms.

Optimizers must consume semantic effect information through the appropriate AST/semantic/IR interfaces.

---

4.10 Hardware discovery

The grammar must not discover hardware.

Hardware capabilities come from the hardware abstraction and execution environment.

---

4.11 Runtime behavior

The grammar does not execute effects.

Runtime systems interpret lowered semantic representations.

---

5. Effect model

An effect is represented conceptually as:

Effect =
    Identity
    + Parameters
    + Composition
    + Optional Arguments
    + Optional Metadata

The grammar should make effect identity structurally explicit.

An effect may be:

- named;
- qualified;
- parameterized;
- generic;
- composed;
- inherited through function calls;
- declared by a function;
- attached to a module;
- attached to an operation;
- handled;
- propagated.

---

6. Effect categories

Zamani should provide a stable semantic vocabulary for common effects while permitting future extension.

The initial standard categories are:

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

These names are semantic categories.

They are not machine identifiers.

The standard effect vocabulary must remain extensible.

A future execution model must not require changing the fundamental syntax merely because a new computational substrate appears.

---

7. Effect declarations

Effect declarations are defined by:

effect-declarations.g4

A declaration introduces a named effect in the current namespace.

Conceptually:

effect MyEffect

An effect may optionally contain metadata, parameters, or a semantic body according to the language specification.

Example:

effect Logging

Example:

effect Database

Example:

effect Quantum

The declaration itself does not bind the effect to a particular implementation.

---

8. Effect namespaces

Effects may be qualified:

effect security::Audit

or referenced through an imported namespace.

The grammar must use the canonical name/path rules established by:

grammar/core/names.g4
grammar/core/paths.g4
grammar/core/qualified-names.g4
grammar/modules/

"effects/" must not create a competing identifier grammar.

---

9. Effect sets

Effect sets represent multiple effects.

Conceptually:

effects { IO, Network, Security }

The syntax must support arbitrary effect-set cardinality.

There must be:

- no fixed maximum number of effects;
- no fixed maximum nesting depth imposed by the language grammar;
- no fixed maximum effect parameters;
- no fixed number of standard effects.

Any implementation limit must be an implementation/resource concern rather than a language semantic restriction.

---

10. Effect composition

Effects can compose.

For example:

effects {
    IO,
    Quantum,
    Network
}

Composition must be semantically deterministic.

The compiler must normalize effect sets according to canonical effect identity rather than source ordering.

Thus:

effects { IO, Quantum }

and:

effects { Quantum, IO }

represent the same unordered effect set unless a future language feature explicitly gives effects ordering semantics.

---

11. Effect polymorphism

Generic functions may abstract over effects.

Conceptually:

fn compute<E: Effect>(value: T) effects { E } {
    ...
}

The exact generic syntax is owned by the functions/type systems.

"effects/" only defines the effect-side grammar needed to participate in that contract.

Effect polymorphism is essential for POCO-REAF because generic source code must not need to be rewritten for every execution environment.

---

12. Effect parameters

Effects may carry semantic parameters where required.

For example:

effect Transaction<Mode>

Parameters must be represented using the canonical generic/type/expression syntax.

Effects must not use parameters as disguised hardware constants.

Invalid design:

effect Quantum<32>

when "32" merely means "this machine has 32 qubits."

Valid semantic use would be something whose value genuinely affects program semantics, with resource capacity handled separately.

---

13. Effect application

Effect annotations may be attached to constructs such as:

- functions;
- declarations;
- operations;
- modules;
- blocks;
- expressions;
- foreign interfaces;
- hardware descriptions;
- quantum operations;
- execution boundaries.

The annotation syntax must be centralized enough to avoid incompatible annotation forms.

Where possible, reuse:

grammar/core/annotations.g4
grammar/core/attributes.g4

rather than inventing competing syntax.

---

14. Function effects

A function may declare its effects.

Conceptually:

fn read_data() effects { IO } {
    ...
}

A function with multiple effects may declare:

fn execute() effects { IO, Quantum, Network } {
    ...
}

The semantic analyzer must subsequently verify that the function body is consistent with its declared effects.

The grammar only establishes the source representation.

---

15. Effect inference

The language may permit inferred effects.

For example:

fn compute() {
    ...
}

may have its effects inferred from its body.

The grammar must not require users to enumerate every effect manually when inference is supported.

However, explicit declarations must remain available for:

- API contracts;
- verification;
- documentation;
- optimization;
- security analysis;
- compilation;
- interoperability.

---

16. Effect handling

Effect handlers are defined by:

effect-handling.g4

A handler describes how an effect is intercepted or interpreted at a semantic boundary.

Conceptually:

handle computation {
    ...
}

with effect-specific handling clauses as defined by the final semantic specification.

The handler grammar must remain independent of runtime implementation.

A handler must not directly encode:

- operating-system calls;
- vendor APIs;
- physical device addresses;
- QPU IDs;
- GPU IDs;
- fixed hardware topology.

Those belong to lower layers.

---

17. Effect propagation

Effects propagate through call relationships and composition.

For example:

caller
  |
  +-- calls function A
          |
          +-- effects { IO, Quantum }

The semantic layer determines whether the caller:

- inherits;
- handles;
- transforms;
- restricts;
- rethrows;
- discharges

those effects.

The grammar provides the constructs required to express these relationships.

---

18. Effect subtraction

Where supported by the final semantic model, an effect context may explicitly remove or discharge an effect after handling.

Conceptually:

handle IO {
    ...
}

The resulting computation may no longer expose "IO" at that semantic boundary.

The exact semantics must be defined by semantic analysis rather than by parser actions.

---

19. Built-in effects

Built-in effects should be registered by the language semantic layer rather than hard-coded throughout parser rules.

The grammar should recognize the syntactic form:

effect-name

and the semantic registry determines whether a name is:

- standard;
- user-defined;
- imported;
- vendor/dialect-defined;
- experimental;
- deprecated;
- unknown.

This prevents the parser from becoming a permanent catalogue of implementation-specific effects.

---

20. Custom effects

Users must be able to define effects.

Example:

effect Telemetry

or:

effect application::Audit

Custom effects are necessary for long-term extensibility.

The grammar must not impose a fixed finite universe of effects.

---

21. Domain-specific effects

Domain subsystems may introduce semantic effects.

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
Clock
Signal
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

These domain effects must remain semantic abstractions.

---

22. Quantum integration

Quantum effects integrate with:

grammar/quantum/
src/quantum/
quantum::ir
QEC
ZQN
scheduling
optimization
hardware
runtime

The dependency direction is:

Zamani source
    ↓
ANTLR parser
    ↓
Zamani AST
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
ZQN / hardware / runtime

Never:

effects grammar
    ↓
quantum::ir
    ↓
effects grammar

There must be no circular dependency.

---

23. Hardware integration

The following distinction is mandatory:

Effect
Capability
Requirement
Constraint
Resource
Target
Placement

are different concepts.

For example:

effect Quantum

means the computation has quantum semantics.

It does not mean:

target = specific QPU

Similarly:

effect Hardware

does not select:

FPGA
GPU
ASIC
CPU

unless an explicit target/resource/capability construct separately specifies that requirement.

---

24. Resource integration

Effect syntax may interact with resource requirements during semantic analysis.

Example conceptual source:

requires quantum
effects { Quantum }

These have different meanings.

"effects { Quantum }":

«this computation performs quantum computation.»

"requires quantum":

«execution requires an environment capable of providing quantum computation.»

The parser must preserve the distinction.

---

25. Capability integration

Capabilities describe what an environment can provide.

Effects describe what a computation does or may do.

Therefore:

Effect ⟂ Capability

conceptually.

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

The effects grammar itself must not perform this matching.

---

26. Runtime integration

The runtime receives effect information only after semantic lowering.

A possible pipeline is:

Source
 ↓
Lexer
 ↓
Parser
 ↓
AST
 ↓
Semantic Analysis
 ↓
Effect Analysis
 ↓
Canonical IR
 ↓
Compilation
 ↓
Execution Plan
 ↓
Runtime

The runtime must not parse ".g4" files.

The grammar must not depend on runtime implementation details.

---

27. Compiler integration

The compiler must expose a stable semantic representation for effects.

Conceptually:

EffectId
EffectSet
EffectDeclaration
EffectParameter
EffectContext
EffectHandler

These are compiler/AST/semantic concepts, not grammar concepts.

They should be defined in the appropriate compiler/frontend/semantic subsystem rather than inside ".g4" files.

The grammar produces parse-tree information from which those structures can be constructed.

---

28. AST contract

The AST layer must preserve:

- effect identity;
- source location;
- namespace;
- parameters;
- declaration/use distinction;
- effect-set membership;
- annotations;
- handler structure;
- generic/effect variables;
- source provenance.

The AST must not silently collapse distinct effect declarations into strings without semantic identity.

The AST must also not encode physical hardware properties merely because an effect is associated with hardware.

---

29. Diagnostics

Effect-related parser and semantic diagnostics must distinguish:

Syntax errors

Examples:

missing effect name
malformed effect set
malformed parameter list
malformed handler

Name errors

Examples:

unknown effect
unknown effect namespace
duplicate effect declaration

Type/effect errors

Examples:

effect used where a type is required
invalid effect parameter
invalid effect composition

Effect-system errors

Examples:

undeclared effect
unhandled effect
incompatible effect
effect escape
invalid effect transformation

Diagnostics must provide:

- stable error category;
- source span;
- relevant symbol;
- contextual explanation;
- actionable correction where possible.

---

30. Determinism

Parsing the same source under the same grammar version must produce the same parse structure.

Effect sets must have canonical semantic normalization.

The semantic representation must not depend on:

- hash-map iteration order;
- machine resource order;
- hardware discovery order;
- network state;
- runtime scheduling;
- provider-specific ordering.

---

31. Scalability

The grammar must support:

one effect

through arbitrarily large effect sets permitted by available compiler resources.

There must be no source-level constants such as:

MAX_EFFECTS
MAX_EFFECT_PARAMETERS
MAX_HANDLERS
MAX_EFFECT_DEPTH
MAX_QUANTUM_EFFECTS
MAX_HARDWARE_EFFECTS

unless a limit is genuinely required by the parser technology itself.

Such an implementation limit must never become a language semantic limit.

---

32. POCO-REAF compatibility

Effects are essential to POCO-REAF but must not defeat it.

The following principle applies:

Portable semantics
        ↓
Effect description
        ↓
Target-independent compilation
        ↓
Target capability matching
        ↓
Target-specific realization

A developer should not need to rewrite:

effects { Quantum, Network }

simply because execution moves between:

- simulator;
- QPU;
- embedded processor;
- accelerator;
- cluster;
- cloud;
- future architecture.

Only the realization may change.

---

33. Hardware-specific anti-patterns

The grammar MUST reject or avoid designs where effects encode physical deployment accidentally.

Bad:

effect GPU0

Bad:

effect QPU32

Bad:

effect Quantum<64>

when "64" represents physical capacity.

Bad:

effect FPGA_1

Bad:

effect NetworkNode<128>

These belong to target/resource/deployment descriptions when they have legitimate meaning.

---

34. Security

Effect declarations may identify security-sensitive semantics.

For example:

Security
Cryptography
Identity
Confidentiality
Privacy

However, declaring an effect does not grant permission.

This distinction is mandatory:

effect Security

does not mean:

permission granted

Permissions and authorization belong to the security subsystem.

---

35. Foreign-function integration

Foreign calls may carry effects.

For example, a C/C++/Python/system interface may declare effects such as:

IO
Network
External
System

The interoperability subsystem owns ABI/FFI details.

Effects provide semantic metadata consumed by interoperability and semantic analysis.

---

36. HDL integration

HDL constructs may expose effects related to:

Hardware
Clock
Timing
Signal
Memory
External

Effects do not define HDL semantics themselves.

The dependency remains:

HDL grammar
    ↓
AST
    ↓
semantic analysis
    ↓
hardware/HDL representation

rather than HDL grammar depending on runtime effects.

---

37. Distributed computing integration

Distributed effects may express semantic behavior such as:

Distributed
Network
RemoteExecution
Replication
Consistency

They must not hard-code:

- node counts;
- node IDs;
- cluster topology;
- network addresses.

Those belong to deployment/resource/hardware layers.

---

38. Effect aliases

If aliases are supported, they must be explicitly declared.

Conceptually:

effect alias ExternalIO = IO

Alias resolution must be deterministic.

Aliases must not create semantic ambiguity or cycles.

An alias graph must be validated for:

- duplicate definitions;
- cycles;
- invalid targets;
- namespace collisions.

---

39. Effect versioning

Effect semantics may evolve.

Version information must be associated with the language/dialect/effect definition rather than hidden inside parser implementation.

Effects introduced by dialects must use the dialect/versioning mechanisms.

An old program must remain parseable under its declared compatible language version according to the compatibility policy.

---

40. Dialect integration

Vendor and experimental effects must be expressible without modifying the universal core grammar every time a new platform appears.

The preferred model is:

core effect syntax
       +
qualified dialect namespace
       +
dialect registration
       +
semantic extension

For example:

vendor::effect_name

may be supported through the dialect system.

Vendor extensions must not become permanent core language requirements merely because one backend introduces them.

---

41. Effect metadata

Effect declarations may carry metadata using the canonical metadata/attribute system.

Metadata may include:

- documentation;
- version;
- deprecation;
- stability;
- provenance;
- dialect;
- semantic classification.

Metadata must not silently alter the core meaning of an effect unless explicitly defined by the semantic specification.

---

42. Error recovery

ANTLR parser rules must provide predictable recovery behavior.

Malformed effects must not cause unrelated source regions to be interpreted as effects.

Error recovery must preserve useful source locations for diagnostics.

The parser must not emit fake semantic effects to "make parsing succeed."

---

43. Lexer contract

Effect syntax depends on the canonical lexer for:

- identifiers;
- keywords;
- punctuation;
- operators;
- literals;
- comments;
- annotations.

Effect-specific tokens should only be introduced where necessary.

Do not duplicate identifier, numeric literal, string literal, or annotation lexical definitions inside "effects/".

---

44. Parser contract

The top-level grammar:

grammar/Zamani.g4

must compose the effects grammar through parser rules or the project's established ANTLR composition mechanism.

"effects/" must remain modular.

No effect rule should assume that it owns the complete Zamani compilation unit.

---

45. Grammar composition

The expected conceptual relationship is:

Zamani.g4
 ├── core
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
 └── execution

The actual ANTLR import/delegation mechanism must follow the repository's chosen grammar-authority architecture.

---

46. Required files in this directory

The directory consists of:

effects/
├── README.md
├── effects.g4
├── effect-declarations.g4
├── effect-sets.g4
├── effect-handling.g4
├── capabilities.g4
├── io.g4
├── hardware.g4
├── quantum.g4
├── distributed.g4
├── security.g4
├── network.g4
└── custom-effects.g4

Each file has a deliberately narrow responsibility.

---

47. "effects.g4"

Owns

The composition/root rules for the effect subsystem.

Does not own

Detailed declaration, handler, domain-specific, or capability rules.

Dependencies

- canonical names;
- paths;
- annotations;
- expressions;
- types.

Consumers

- "Zamani.g4";
- AST builder;
- semantic analyzer.

Completion criteria

The file must provide a complete effect grammar composition point without duplicating subordinate rules.

---

48. "effect-declarations.g4"

Owns

Syntax for:

- effect declarations;
- effect names;
- effect parameters;
- effect metadata.

Does not own

Effect handling or hardware capabilities.

Tests

- simple declarations;
- qualified declarations;
- parameterized declarations;
- duplicate declarations as semantic-negative tests;
- malformed declarations.

---

49. "effect-sets.g4"

Owns

Syntax for:

- effect sets;
- effect lists;
- effect composition;
- effect variables;
- effect expressions where defined.

Scalability

No fixed number of effects.

Tests

- empty set if permitted;
- singleton;
- multiple effects;
- arbitrarily large generated sets;
- duplicate effects;
- nested/invalid forms.

---

50. "effect-handling.g4"

Owns

Syntax for:

- handlers;
- handled effect clauses;
- effect propagation syntax;
- effect discharge syntax.

Does not own

Runtime handler implementation.

Integration

AST → semantic effect analysis → lowered handler representation.

---

51. "capabilities.g4"

This file must be handled carefully because capabilities are also represented in:

grammar/core/capabilities.g4
grammar/resources/capabilities.g4
grammar/hardware/capabilities.g4
grammar/quantum/quantum-capabilities.g4

Its purpose is only to define effect-system syntax needed to refer to or constrain effect capabilities.

It must not create a second capability model.

The semantic model must establish one canonical capability abstraction.

If repository inspection shows that the core capability grammar already provides all required syntax, this file should be merged/removed, with "effects" referencing the canonical capability rules instead.

That is preferable to duplicated capability grammars.

---

52. "io.g4"

Owns

Standard syntax for semantic IO-effect declarations or IO-specific effect forms, if such specialization is required.

Does not own

Actual filesystem/network operations.

Security

No arbitrary filesystem access is performed by parsing.

Integration

IO syntax
 ↓
AST
 ↓
IO effect semantics
 ↓
compiler/runtime

---

53. "hardware.g4"

Owns

Hardware-related semantic effect syntax.

It must express that computation interacts with hardware as a semantic category.

It must NOT encode:

- CPU count;
- GPU count;
- device ID;
- FPGA number;
- ASIC identity;
- address;
- topology;
- machine size.

Those belong to hardware/resource/target systems.

---

54. "quantum.g4"

Owns

Quantum-specific effect syntax.

It must remain backend-independent.

It must not contain:

MAX_QUBITS
q[0]
q[1]
fixed gate counts
fixed device IDs
fixed topology

Quantum effects eventually participate in semantic lowering toward:

quantum::ir

They do not define that IR.

---

55. "distributed.g4"

Owns

Distributed semantic effects.

Examples include:

Distributed
RemoteExecution
Replication
Consistency

It must not define node topology.

---

56. "security.g4"

Owns

Security-related semantic effects.

Examples:

Security
Cryptography
Privacy
Identity
Confidentiality

It does not grant permissions.

---

57. "network.g4"

Owns

Network-related semantic effect syntax.

It must not contain:

- IP addresses;
- fixed ports;
- network sizes;
- node counts;
- topology.

Those belong to networking/deployment/resource systems.

---

58. "custom-effects.g4"

Owns

Extensible user/dialect-defined effect syntax.

The grammar must permit future semantic domains without requiring modification of core effect rules.

The preferred identity mechanism is canonical qualified names.

---

59. AST integration contract

The frontend must transform parse trees into AST structures that retain semantic information.

Required conceptual structures include:

EffectId
EffectName
EffectReference
EffectDeclaration
EffectParameter
EffectSet
EffectHandler
EffectContext

Exact Rust locations are determined by repository architecture.

The grammar must not dictate ownership of these structures.

---

60. Rust safety contract

All Rust components consuming effect syntax must remain safe Rust.

The relevant crate/module must enforce:

#![forbid(unsafe_code)]

No parser integration, AST construction, effect analysis, serialization, or diagnostic implementation may require "unsafe".

Rust 1.97/1.97.1 compatibility must be tested explicitly.

---

61. Testing contract

At minimum:

grammar/tests/effects/

must test:

Positive

- declarations;
- qualified effects;
- effect sets;
- function effects;
- handlers;
- custom effects;
- quantum effects;
- hardware effects;
- distributed effects;
- security effects;
- network effects.

Negative

- malformed declaration;
- malformed effect set;
- malformed handler;
- invalid identifier;
- invalid qualification;
- malformed parameters;
- duplicate declarations;
- illegal nesting.

Boundary

- one effect;
- many effects;
- deeply nested valid semantic structures;
- large source files;
- large effect sets.

Cross-domain

- classical + quantum;
- quantum + hardware;
- quantum + distributed;
- classical + network;
- AI + accelerator;
- HDL + hardware;
- quantum + security;
- full hybrid program.

---

62. Scalability tests

Tests must explicitly prove that effect syntax does not encode arbitrary machine limits.

Examples must vary independently:

effect count
resource count
qubit count
device count
node count
thread count
memory size
program size

The grammar must continue to represent the program independently of those physical quantities.

---

63. Determinism tests

For identical source:

source
 ↓
lexer
 ↓
parser
 ↓
AST

must produce equivalent deterministic structures.

Effect-set normalization must produce deterministic semantic ordering where an ordered representation is required internally.

---

64. Round-trip tests

Where the repository provides a formatter/printer:

source
 ↓
parser
 ↓
AST
 ↓
printer
 ↓
parser

must preserve effect semantics.

Formatting may change whitespace and equivalent syntactic presentation, but must not change:

- effect identity;
- effect membership;
- handler structure;
- parameters;
- namespaces.

---

65. Compatibility

Effect syntax is part of the stable Zamani language surface.

Breaking changes require:

1. language-version declaration;
2. migration documentation;
3. compatibility tests;
4. deprecation period where appropriate;
5. explicit semantic justification.

A vendor-specific effect must not silently become a core effect with incompatible semantics.

---

66. Hard-coding audit

Before this subsystem is declared complete, search for:

MAX_EFFECT
MAX_EFFECTS
MAX_QUBITS
MAX_DEVICES
MAX_NODES
MAX_GPUS
MAX_CPUS
DEVICE_0
QPU_0
GPU_0
q[0]
q[1]
fixed topology
fixed resource counts

Any occurrence must be classified.

Allowed:

- parser implementation safeguards;
- test fixture values;
- documented examples;
- genuine semantic constants.

Forbidden:

- accidental source-language scalability limits;
- fixed hardware assumptions;
- fixed backend assumptions.

---

67. Security audit

Verify that effect syntax cannot:

- execute external commands;
- access files;
- access network resources;
- discover hardware;
- invoke runtime services;
- bypass authorization.

Parsing is declarative.

Semantic analysis is deterministic.

Execution belongs to later stages.

---

68. Repository integration

The final integration path is:

grammar/
    ↓
ANTLR lexer/parser
    ↓
frontend AST
    ↓
semantic analysis
    ├── type checking
    ├── effect checking
    ├── capability checking
    ├── requirement checking
    └── constraint checking
    ↓
canonical IR
    ├── classical IR
    └── quantum::ir
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
hardware abstraction
    ↓
ZQN / QEC where applicable
    ↓
compiler
    ↓
runtime

The arrows represent semantic consumption.

They do not imply that every subsystem directly depends on the grammar.

---

69. Dependency restrictions

The following dependencies are prohibited:

effects → runtime implementation
effects → hardware discovery
effects → device inventory
effects → quantum::ir implementation
effects → QEC implementation
effects → ZQN implementation
effects → scheduling implementation
effects → optimization implementation

The correct relationship is:

grammar
 ↓
AST/semantic model
 ↓
canonical representations
 ↓
backend subsystems

---

70. Completion definition

"grammar/effects/" is production-ready only when:

- [ ] effect syntax has one authoritative definition;
- [ ] effect declarations are complete;
- [ ] effect sets are complete;
- [ ] effect handling is complete;
- [ ] custom effects are supported;
- [ ] standard effects have stable semantic identities;
- [ ] capabilities remain distinct from effects;
- [ ] requirements remain distinct from effects;
- [ ] constraints remain distinct from effects;
- [ ] resources remain distinct from effects;
- [ ] hardware remains distinct from effects;
- [ ] quantum effects remain distinct from "quantum::ir";
- [ ] QEC is not duplicated;
- [ ] ZQN is not duplicated;
- [ ] runtime behavior is not embedded in grammar;
- [ ] no physical machine assumptions are encoded;
- [ ] no fixed resource limits exist;
- [ ] ANTLR integration is deterministic;
- [ ] AST contracts are defined;
- [ ] semantic contracts are defined;
- [ ] compiler contracts are defined;
- [ ] runtime boundaries are defined;
- [ ] dialect integration is defined;
- [ ] versioning is defined;
- [ ] diagnostics are defined;
- [ ] positive tests exist;
- [ ] negative tests exist;
- [ ] boundary tests exist;
- [ ] scalability tests exist;
- [ ] cross-domain tests exist;
- [ ] determinism tests exist;
- [ ] round-trip tests exist where supported;
- [ ] Rust 1.97/1.97.1 integration passes;
- [ ] all Rust implementation code uses safe Rust;
- [ ] "unsafe" is forbidden;
- [ ] hard-coding audit passes;
- [ ] security audit passes;
- [ ] documentation matches the authoritative grammar;
- [ ] no circular dependency exists;
- [ ] POCO-REAF semantics are preserved.

---

71. Final invariant

The effects system must preserve the following invariant:

Effect describes computation.
Capability describes available execution ability.
Requirement describes what execution needs.
Constraint describes what execution must satisfy.
Resource describes what execution consumes or reserves.
Target describes where/how compilation may be realized.
Runtime describes how execution actually occurs.

These concepts must never be collapsed into one another.

The ultimate architecture is:

                 Zamani Source
                       │
                       ▼
                  Effect Syntax
                       │
                       ▼
                  Effect AST
                       │
                       ▼
              Effect Semantic Model
                       │
          ┌────────────┼─────────────┐
          ▼            ▼             ▼
       Types       Capabilities   Requirements
          │            │             │
          └────────────┼─────────────┘
                       ▼
                 Canonical IR
                 /          \
                /            \
       Classical IR       quantum::ir
                \            /
                 \          /
                  ▼        ▼
                 Compiler
                    │
        ┌───────────┼────────────┐
        ▼           ▼            ▼
   Optimization  Routing     Scheduling
        │           │            │
        └───────────┼────────────┘
                    ▼
             Hardware / ZQN / QEC
                    │
                    ▼
                  Runtime

Therefore:

«Zamani effects describe the semantic consequences of computation, not the accidental properties of the machine executing it.»

This is a mandatory foundation for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

and for the Zamani objective:

From Atom to Everywhere.