Worked for 16s

Below is the complete production specification for github.com/Benwellonedge28/Zamani/grammar/specification/semantic-model.md. I’ve treated the semantic model as the contract between syntax and the rest of Zamani: it defines meaning and invariants, but does not duplicate canonical IR, QEC, ZQN, scheduling, routing, optimization, or hardware models. This is consistent with the repository material showing explicit ownership boundaries in existing production-oriented subsystems—for example, QEC components consume canonical resource/capability contracts rather than redefining them. 

Zamani Semantic Model

Path: "grammar/specification/semantic-model.md"
Status: Production Specification
Language: Zamani
Grammar Authority: "grammar/Zamani.g4"
Semantic Authority: This specification
Implementation Target: Rust 1.97 / Rust 1.97.1
Safety: "unsafe" is forbidden
Primary Goal: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the normative semantic model of the Zamani programming language.

The semantic model specifies what a valid Zamani program means, independently of the machine, processor, accelerator, quantum device, FPGA, ASIC, operating system, deployment topology, scheduler, compiler optimization, or runtime currently used to execute it.

Zamani semantics are therefore defined at the level of:

- values;
- types;
- names;
- bindings;
- expressions;
- statements;
- functions;
- modules;
- effects;
- resources;
- capabilities;
- requirements;
- constraints;
- computation domains;
- execution boundaries;
- ownership;
- concurrency;
- communication;
- quantum operations;
- classical computation;
- hardware intent;
- observations;
- errors;
- determinism;
- provenance;
- program identity;
- semantic versions.

The semantic model MUST NOT encode accidental properties of a particular implementation.

---

2. Normative status

This document is normative for semantic meaning.

The authority hierarchy is:

Zamani source syntax
        |
        v
grammar/Zamani.g4
        |
        v
AST / syntax representation
        |
        v
THIS SEMANTIC MODEL
        |
        v
semantic analysis
        |
        v
canonical domain IRs
        |
        +--------------------+
        |                    |
        v                    v
classical IR           quantum::ir
        |                    |
        +---------+----------+
                  |
                  v
          optimization
                  |
                  v
              routing
                  |
                  v
             scheduling
                  |
                  v
        hardware / ZQN / runtime

The grammar defines syntactic validity.

This document defines semantic validity and meaning.

The IR defines canonical machine-independent representations of established semantics.

Compilation, optimization, scheduling, routing, hardware adaptation, and runtime execution MUST NOT redefine source-language meaning.

---

3. Core semantic principle

The fundamental semantic principle is:

«Zamani describes computation, intent, relationships, requirements, constraints, capabilities, and observable behavior—not arbitrary limitations of the machine currently available.»

A program therefore has one semantic meaning regardless of whether its eventual execution uses:

- one CPU;
- many CPUs;
- a GPU;
- many GPUs;
- an FPGA;
- multiple FPGAs;
- an ASIC;
- a quantum processor;
- a quantum simulator;
- a heterogeneous system;
- an embedded system;
- a cluster;
- a supercomputer;
- a distributed deployment;
- cloud infrastructure;
- future computational architectures.

The implementation MAY differ.

The semantic meaning MUST NOT silently change.

---

4. POCO-REAF semantic contract

POCO-REAF means:

Program Once
    ↓
Semantic Program Identity
    ↓
Compile Once
    ↓
Portable Semantic Artifact
    ↓
Target Capability Resolution
    ↓
Target-Specific Lowering
    ↓
Run Everywhere
    ↓
Run Anywhere
    ↓
Future-Compatible Evolution

POCO-REAF does NOT mean that every machine can execute every program.

Instead:

same source semantics
        +
available capabilities
        +
declared requirements
        +
declared constraints
        +
implementation mappings
        =
valid execution or explicit inability to execute

A target that cannot satisfy a program's semantic requirements MUST fail explicitly.

It MUST NOT silently alter the program's meaning.

---

5. Semantic portability

A program is semantically portable when its meaning is independent of target-specific implementation details.

For example:

requires quantum

means that the computation requires quantum semantics.

It MUST NOT implicitly mean:

use provider X
use processor Y
use qubit count N
use topology Z

Similarly:

requires parallel

MUST NOT imply:

use exactly 64 cores

and:

requires accelerator

MUST NOT imply:

use GPU 0

Target selection belongs to target resolution.

---

6. Semantic categories

Every semantic construct MUST belong to one or more explicitly defined categories.

The principal categories are:

1. Syntax
2. Value
3. Type
4. Binding
5. Effect
6. Capability
7. Requirement
8. Constraint
9. Preference
10. Resource
11. Observation
12. Execution
13. Hardware intent
14. Communication
15. Ownership
16. Provenance
17. Error
18. Artifact
19. Version
20. Domain semantics

These categories MUST NOT be conflated.

---

7. Requirement, capability, constraint, preference, and hint

These are distinct semantic concepts.

7.1 Requirement

A requirement states what must be available for the program to execute with its declared semantics.

Example:

requires quantum;

A requirement is mandatory.

Failure to satisfy it is an execution/compilation admission failure, not an alternative semantic interpretation.

---

7.2 Capability

A capability describes something an execution environment can provide.

Examples:

quantum
dynamic_circuit
fpga
gpu
distributed
fault_tolerant

Capabilities are supplied by the environment.

Programs MAY query or constrain capabilities through defined language mechanisms.

The grammar MUST NOT encode a fixed universe of future capabilities.

Unknown capabilities MUST be representable through extensible identifiers or qualified capability namespaces.

---

7.3 Constraint

A constraint limits acceptable implementations.

Examples:

requires quantum;
constraint coherence >= required_coherence;
constraint latency <= permitted_latency;

A constraint is stronger than a preference.

A compiler MUST NOT violate a semantic constraint merely to obtain a successful compilation.

---

7.4 Preference

A preference expresses an optimization objective without becoming semantic necessity.

For example:

prefer lower_latency;
prefer lower_energy;
prefer higher_throughput;

Failure to satisfy a preference MUST NOT invalidate otherwise semantically correct execution.

---

7.5 Hint

A hint provides optional implementation information.

Hints MUST NOT change program meaning.

Compilers and runtimes MAY ignore hints.

---

8. Semantic hierarchy

Zamani semantics MUST follow this hierarchy:

Meaning
  >
Requirements
  >
Constraints
  >
Preferences
  >
Hints
  >
Implementation choices

An implementation choice MUST NEVER override a semantic requirement.

A preference MUST NEVER be treated as a requirement.

A hint MUST NEVER become an implicit constraint.

---

9. Values

A value is a semantic result represented independently of its physical storage.

Values MAY include:

- booleans;
- integers;
- floating-point values;
- exact numeric values;
- complex values;
- characters;
- strings;
- tuples;
- arrays;
- records;
- algebraic values;
- references;
- functions;
- futures;
- streams;
- classical data;
- quantum measurement results;
- hardware-domain values;
- resource descriptions;
- capability descriptions;
- symbolic values.

The semantic model MUST NOT require a particular memory representation.

---

10. Types

Types define semantic domains and valid operations.

A type MUST describe semantic properties rather than accidental target representation.

For example:

integer

does not inherently mean:

CPU register

and:

qubit

does not inherently mean:

physical qubit #0

Type implementation MAY vary by target provided semantic guarantees remain valid.

---

11. Type identity

A type has semantic identity independent of:

- memory address;
- device ID;
- CPU ID;
- GPU ID;
- qubit index;
- FPGA location;
- physical register;
- compiler temporary;
- runtime handle.

Physical representations MAY be associated later.

---

12. Genericity

Generic types and operations MUST express relationships rather than fixed machine capacities.

For example:

vector<T, N>

may express a dimension parameter "N".

The grammar and semantic model MUST NOT impose an arbitrary global upper bound.

Validity is determined by:

- representability;
- target capability;
- declared constraints;
- compiler/resource limits;
- runtime availability.

These are separate from language semantics.

---

13. Names and identity

Semantic identity MUST be based on stable language-level names or compiler-generated stable identifiers.

Physical identifiers MUST NOT become semantic identities unless explicitly declared as part of a hardware-specific program.

Examples of implementation-only identity:

device-17
gpu-3
qpu-7
qubit-128
core-42
memory-bank-5

Such identities belong to target and deployment layers.

---

14. Binding

A binding associates a name with a semantic entity.

Bindings MAY represent:

- values;
- variables;
- constants;
- types;
- functions;
- modules;
- capabilities;
- resources;
- effects;
- hardware interfaces;
- quantum entities;
- compile-time entities.

Binding lifetime and visibility are semantic properties.

Physical allocation is not.

---

15. Scope

Scope determines where a binding is visible.

Scopes MUST be determined by source semantics.

They MUST NOT depend on:

- machine size;
- number of devices;
- processor count;
- runtime topology.

---

16. Evaluation

Expressions evaluate according to language-defined semantics.

Evaluation MAY be:

- immediate;
- deferred;
- lazy where explicitly defined;
- concurrent;
- parallel;
- distributed;
- accelerator-backed;
- quantum-assisted.

An implementation MAY transform evaluation order only when semantic equivalence is preserved.

---

17. Observable behavior

Two implementations are semantically equivalent when they preserve all behavior observable through the language's defined observation model.

Observations MAY include:

- returned values;
- mutations visible through defined interfaces;
- emitted events;
- measurements;
- externally declared effects;
- communication;
- errors;
- termination;
- explicitly observable timing contracts;
- resource guarantees when those are semantic constraints.

Unspecified implementation details MUST NOT become observable accidentally.

---

18. Evaluation order

Evaluation order MUST be explicitly specified wherever it can affect observable behavior.

For operations whose ordering is semantically irrelevant, compilers MAY reorder them.

For operations with dependencies, effects, communication, synchronization, or quantum semantics, reordering is legal only when semantic preservation is proven.

---

19. Side effects

Effects describe interactions beyond pure value computation.

Effects MAY include:

- I/O;
- filesystem access;
- networking;
- process interaction;
- device interaction;
- quantum execution;
- measurement;
- hardware interaction;
- distributed communication;
- allocation;
- synchronization;
- external state;
- security-sensitive operations.

Effect declarations MUST remain distinct from implementation mechanisms.

---

20. Effect composition

Effects compose according to explicit effect rules.

An implementation MUST NOT silently erase an effect.

For example, a quantum measurement is not equivalent to an ordinary pure function call merely because both return a value.

Likewise, device I/O is not equivalent to pure computation.

---

21. Purity

A computation is pure when its result depends only on its semantic inputs and does not produce observable external effects.

Pure computation MAY be:

- constant-folded;
- memoized;
- parallelized;
- vectorized;
- accelerated;
- distributed;

provided semantic equivalence is preserved.

---

22. Determinism

Zamani distinguishes:

1. deterministic semantics;
2. nondeterministic semantics;
3. probabilistic semantics;
4. externally nondeterministic behavior;
5. unspecified ordering.

These MUST NOT be conflated.

A quantum measurement, for example, MAY be intrinsically probabilistic.

A compiler's internal hash-map ordering MUST NOT introduce accidental nondeterminism into a deterministic program.

---

23. Reproducibility

Where a program declares reproducibility requirements, implementations MUST preserve the required reproducibility contract.

Reproducibility MAY depend on:

- explicit seeds;
- deterministic scheduling;
- deterministic reduction;
- stable serialization;
- stable compiler artifacts;
- stable execution environment;
- defined random sources.

Physical randomness MUST NOT be represented as ordinary deterministic computation.

---

24. Concurrency semantics

Concurrency describes potentially overlapping computation.

Concurrency MUST NOT imply a fixed number of workers.

For example:

parallel computation

does not mean:

64 workers

The runtime MAY execute a computation with:

1

or:

many

workers when the semantics permit.

---

25. Parallel semantics

Parallel execution MUST preserve dependency and observable-effect requirements.

The language MAY expose:

- data parallelism;
- task parallelism;
- pipeline parallelism;
- collective operations;
- distributed parallelism;
- accelerator execution.

The actual execution width is target-dependent unless explicitly part of program semantics.

---

26. Synchronization

Synchronization establishes semantic ordering or coordination.

Examples include:

- barriers;
- joins;
- locks;
- channels;
- events;
- futures;
- atomics;
- domain-specific synchronization.

Synchronization MUST NOT assume a fixed number of participants.

Participant sets MUST be represented semantically.

---

27. Distributed semantics

A distributed program describes logical computation and communication.

The source program MUST NOT require a fixed machine topology unless topology is explicitly part of its semantics.

A logical node is distinct from a physical host.

A logical service is distinct from a physical process.

A logical endpoint is distinct from a network address.

Physical deployment belongs to deployment and runtime layers.

---

28. Communication

Communication has semantic properties such as:

- sender;
- receiver;
- payload;
- ordering;
- delivery guarantee;
- failure behavior;
- consistency;
- acknowledgement;
- cancellation.

Transport protocol, IP address, physical network, and routing are implementation concerns unless explicitly declared.

---

29. Memory semantics

Memory semantics describe:

- ownership;
- lifetime;
- aliasing;
- mutability;
- visibility;
- synchronization;
- allocation semantics.

They MUST NOT require a particular:

- RAM capacity;
- cache size;
- NUMA topology;
- memory-bank count;
- address width.

Resource insufficiency is a resource/compilation/runtime condition, not a semantic change.

---

30. Resource semantics

A resource is a consumable or capability-bearing execution entity.

Examples:

- memory;
- compute capacity;
- quantum capacity;
- accelerator capacity;
- bandwidth;
- energy budget;
- storage;
- execution time;
- device availability.

Resources MUST be modeled abstractly.

No grammar rule may impose an arbitrary universal maximum.

---

31. Resource quantity

A source program MAY express resource quantities where they are semantically meaningful.

Examples:

requires at_least <expression> quantum_capacity;
requires <expression> memory;

The quantity MUST be represented as a semantic expression.

The language MUST NOT define a finite universal ceiling merely because an implementation currently uses a bounded integer representation.

---

32. Resource exhaustion

Resource exhaustion MUST produce an explicit failure or negotiated execution outcome.

It MUST NOT cause silent semantic degradation.

For example:

requested computation
        ↓
resource unavailable
        ↓
explicit admission failure

is valid.

Changing the requested computation silently is not.

---

33. Capability discovery

Capability discovery belongs to the capability/environment boundary.

The grammar can express capability requirements.

The runtime/hardware system determines whether those capabilities exist.

The grammar MUST NOT contain a permanent list of every future machine capability.

---

34. Hardware independence

Hardware constructs describe hardware semantics or hardware intent.

They MUST NOT accidentally bind the entire language to one hardware architecture.

Examples of semantic hardware concepts:

- module;
- port;
- signal;
- register;
- clock;
- pipeline;
- memory;
- interface;
- combinational process;
- sequential process.

Examples of target-specific properties:

- FPGA tile coordinates;
- ASIC cell names;
- vendor-specific resource identifiers;
- physical pin numbers;
- device addresses.

The latter belong to hardware-target layers unless explicitly declared by a hardware-specific program.

---

35. HDL semantics

HDL semantics MUST preserve:

- signal meaning;
- combinational behavior;
- sequential behavior;
- clock relationships;
- state transitions;
- interface contracts;
- timing requirements;
- reset behavior;
- hardware parameterization.

Synthesis and placement MUST remain downstream transformations.

---

36. Timing

Timing has three semantic categories:

36.1 Functional timing

Timing that changes program correctness.

36.2 Contractual timing

Timing explicitly required by the program.

36.3 Optimization timing

Timing used only to improve performance.

These MUST remain distinct.

A source-level timing constraint MUST NOT be interpreted as a physical clock frequency unless the language explicitly defines that meaning.

---

37. Quantum semantics

Quantum semantics are first-class language semantics.

Quantum constructs include:

- qubits;
- logical qubits;
- quantum states;
- quantum operations;
- gates;
- controlled operations;
- parameterized operations;
- measurement;
- reset;
- observables;
- dynamic circuits;
- mid-circuit measurement;
- classical control;
- quantum-classical interaction.

The semantic model defines what these constructs mean.

It does NOT own the canonical quantum IR.

---

38. Canonical quantum boundary

The repository's canonical quantum semantic representation is:

quantum::ir

Therefore:

Zamani syntax
    ↓
AST
    ↓
semantic quantum model
    ↓
quantum::ir

The grammar MUST NOT introduce a second permanent quantum representation that competes with "quantum::ir".

Frontend-local structures may exist temporarily during parsing or semantic analysis, but they MUST lower into the canonical quantum representation.

---

39. Qubit identity

A source-level qubit is a semantic entity.

It is NOT inherently:

physical qubit 0

or:

physical qubit 127

Logical and physical qubits MUST remain distinguishable.

Physical assignment belongs to routing/hardware compilation.

---

40. Quantum allocation

Quantum allocation expresses semantic demand.

It MUST NOT require a fixed hardware topology.

A source program MAY state that it needs a number of logical qubits derived from program semantics.

The compiler determines whether available resources can realize that requirement.

---

41. Quantum registers

A quantum register is a semantic collection of quantum entities.

Its size MAY be:

- literal;
- symbolic;
- generic;
- computed;
- parameterized.

The language MUST NOT impose an arbitrary universal register limit.

---

42. Quantum gates

Gate semantics describe mathematical/physical quantum transformations at the language abstraction level.

A gate invocation identifies:

- operation;
- operands;
- parameters;
- control conditions;
- semantic effects.

It MUST NOT require physical qubit adjacency.

Routing is downstream.

---

43. Controlled operations

Controlled operations establish semantic control relationships.

The grammar MUST permit arbitrary valid operand sets permitted by the semantic model.

There MUST be no assumption such as:

control q[0]
target q[1]

unless explicitly written by the developer.

---

44. Measurement

Measurement is an observable quantum effect.

A measurement:

- consumes or transforms quantum state according to the defined operation;
- produces classical information;
- MAY introduce probabilistic behavior;
- MAY alter future valid operations.

Measurement MUST therefore remain semantically distinct from ordinary assignment.

---

45. Mid-circuit measurement

Mid-circuit measurement is valid when supported by the language semantics.

Its classical result MAY control later computation.

This creates an explicit dependency:

quantum operation
       ↓
measurement
       ↓
classical result
       ↓
classical predicate
       ↓
future operation

Compilation and scheduling MUST preserve that dependency.

---

46. Quantum reset

Reset is an explicit quantum operation.

It MUST NOT be treated as an implicit compiler cleanup unless the semantics permit that transformation.

---

47. Quantum state representation

The language MUST NOT require source programs to expose a full physical state-vector representation.

A state may be represented semantically without requiring:

- fixed vector width;
- fixed memory size;
- fixed simulator representation.

Simulation is one possible implementation.

---

48. Quantum error correction

The language MAY express semantic intent relating to:

- logical qubits;
- fault tolerance;
- error correction requirements;
- protected operations;
- logical error guarantees.

The grammar MUST NOT implement QEC algorithms.

QEC remains owned by the repository's QEC subsystem.

---

49. ZQN boundary

ZQN owns quantum noise/fault semantics.

The language semantic model MAY express requirements concerning:

- noise tolerance;
- fault assumptions;
- error characteristics;
- mitigation intent;

but MUST NOT duplicate ZQN's canonical fault representation.

The relationship is:

Zamani semantics
      |
      | requirements / intent
      v
ZQN fault model
      |
      v
resilience / QEC / compilation decisions

---

50. Resilience boundary

Resilience is an orchestration and decision layer.

Zamani semantics MUST NOT contain resilience algorithms.

Semantic declarations MAY establish:

- correctness requirements;
- acceptable degradation;
- recovery expectations;
- fault tolerance requirements.

The resilience subsystem decides how to satisfy them.

---

51. Optimization boundary

Optimization MUST preserve semantic equivalence.

Examples:

- gate cancellation;
- peephole optimization;
- algebraic simplification;
- classical constant folding;
- tensor optimization;
- scheduling improvements.

An optimization that changes observable semantics is invalid unless explicitly permitted by a language-level approximation or relaxation contract.

---

52. Scheduling boundary

Scheduling determines execution ordering and timing after semantic dependencies are established.

The language MUST NOT encode scheduler implementation.

Scheduling MAY use:

- ASAP;
- ALAP;
- resource-aware scheduling;
- critical-path scheduling;
- dynamic scheduling;
- hardware-specific scheduling.

The semantic model only defines the dependencies and timing guarantees that scheduling must preserve.

---

53. Routing boundary

Routing maps logical resources to physical resources.

The semantic model MUST NOT assume physical topology.

For quantum computation:

logical qubit
      ↓
routing
      ↓
physical qubit

For classical/accelerator systems:

logical computation
      ↓
placement
      ↓
physical resource

---

54. Hardware abstraction boundary

Hardware abstraction provides:

- available resources;
- capabilities;
- topology;
- calibration;
- operational state;
- supported operations;
- limits.

The semantic model consumes those facts.

It MUST NOT own hardware discovery.

---

55. Calibration

Calibration data is environmental state.

Calibration MUST NOT become permanent source-language semantics unless explicitly declared as a requirement.

A compiler/runtime MAY use calibration to select an implementation.

It MUST NOT change the mathematical meaning of the program.

---

56. Backend independence

A backend is an implementation target.

Backend-specific behavior MUST be isolated behind target contracts.

The source program MUST NOT need provider-specific syntax for ordinary portable semantics.

Vendor extensions MUST use dialect/extension mechanisms with explicit namespaces and compatibility rules.

---

57. Classical computation

Classical semantics cover:

- scalar values;
- structured values;
- functions;
- recursion;
- iteration;
- pattern matching;
- numerical computation;
- symbolic computation;
- vectors;
- matrices;
- tensors;
- parallel computation;
- accelerator computation.

Classical semantics MUST remain independent of the number or architecture of CPUs.

---

58. Hybrid computation

Hybrid computation combines classical and quantum domains.

The semantic boundary MUST explicitly model transitions such as:

classical → quantum
quantum → classical
classical ↔ accelerator
classical ↔ hardware
quantum ↔ hardware

Each transition MUST identify the semantic data and effects involved.

---

59. Data movement

Data movement is semantically relevant only when observable or constrained.

An implementation MAY eliminate, fuse, copy, or relocate data when semantic equivalence is preserved.

Physical transfer between:

- CPU;
- GPU;
- FPGA;
- QPU;
- memory hierarchy;

is implementation-specific unless explicitly constrained.

---

60. AI/ML semantics

AI/ML constructs MAY describe:

- models;
- tensors;
- datasets;
- training;
- inference;
- differentiation;
- optimization;
- agents;
- pipelines.

The semantic model MUST distinguish mathematical model meaning from accelerator implementation.

A tensor operation MUST NOT inherently mean GPU execution.

---

61. Data semantics

Data constructs describe:

- schemas;
- records;
- collections;
- streams;
- transformations;
- serialization.

Serialization format is part of semantic behavior only when explicitly declared.

Otherwise it remains an interoperability concern.

---

62. Networking semantics

Networking constructs describe logical communication.

They MUST NOT hard-code:

- IP addresses;
- ports;
- physical interfaces;
- fixed node counts;
- network topology.

Such information belongs to deployment configuration or explicit system-level declarations.

---

63. Security semantics

Security semantics include:

- authority;
- identity;
- permissions;
- capabilities;
- trust;
- confidentiality;
- integrity;
- authentication;
- authorization.

Security properties MUST be preserved across lowering.

A compiler MUST NOT remove a security effect merely because the underlying operation appears semantically equivalent at the value level.

---

64. Ownership and borrowing

Where Zamani exposes ownership/borrowing semantics, they describe:

- aliasing;
- mutation;
- lifetime;
- access rights;
- resource ownership.

The semantics MUST NOT require a specific allocator or memory layout.

Rust implementation of the compiler MUST remain safe Rust.

---

65. Unsafe code

The Zamani compiler/frontend implementation MUST use:

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

No grammar semantic feature may require Rust "unsafe".

Unsafe machine operations, where needed by an eventual target backend, MUST be isolated behind explicit backend contracts and MUST NOT leak into the language's semantic model.

---

66. Error semantics

Errors MUST be categorized by semantic layer.

At minimum distinguish:

- syntax errors;
- name-resolution errors;
- type errors;
- effect errors;
- capability errors;
- requirement failures;
- constraint violations;
- resource admission failures;
- compilation failures;
- target incompatibility;
- runtime failures;
- hardware failures;
- communication failures;
- quantum execution failures.

A target failure MUST NOT be reported as a language syntax error.

---

67. Error stability

Semantic errors MUST have stable categories and machine-readable identities.

Provider-specific error identifiers MUST NOT become universal language semantics.

The compiler MAY attach provider diagnostics as additional metadata.

---

68. Partial execution

A partially executed program MUST NOT be interpreted as a successful complete program unless the source semantics explicitly permit partial results.

For quantum computation especially:

partial execution

does not imply:

recoverable complete quantum state

State persistence and checkpoint semantics belong to the relevant runtime/QEC contracts.

---

69. Checkpoint semantics

A checkpoint is not automatically a serialized arbitrary quantum state.

Semantic checkpoint categories MUST distinguish:

- classical execution checkpoint;
- compiled-program checkpoint;
- logical checkpoint;
- measurement-boundary checkpoint;
- QEC-supported checkpoint;
- provider-supported state checkpoint;
- reconstructible computation state.

The language MUST NOT promise universal serialization of arbitrary unknown quantum states.

---

70. Provenance

Every compiled semantic artifact SHOULD retain provenance sufficient to establish:

- source identity;
- semantic version;
- grammar version;
- relevant dialects;
- compiler identity;
- semantic transformations;
- target transformations;
- optimization stages;
- execution-relevant assumptions.

Provenance MUST NOT alter program semantics.

---

71. Semantic identity

A program's semantic identity is determined from:

- normalized source semantics;
- language version;
- explicitly selected dialect versions;
- relevant semantic configuration.

It MUST NOT depend on:

- device ID;
- runtime process ID;
- memory address;
- temporary compiler identifier;
- physical qubit assignment.

---

72. Versioning

Semantic versioning MUST distinguish:

language version
grammar version
AST version
semantic-model version
IR version
artifact version
backend version
runtime version
hardware capability version

Changing a hardware backend MUST NOT automatically change language meaning.

---

73. Compatibility

Backward compatibility is required for stable semantic constructs unless explicitly deprecated.

A syntax migration MUST preserve semantic meaning where compatibility is claimed.

A compiler MUST reject incompatible constructs explicitly.

It MUST NOT silently reinterpret them.

---

74. Dialects

Dialects extend Zamani semantics through explicitly namespaced extensions.

A dialect MUST declare:

- identity;
- version;
- namespace;
- semantic additions;
- compatibility;
- required capabilities;
- lowering contract;
- reserved identifiers;
- failure behavior.

A dialect MUST NOT redefine an existing core construct with a conflicting meaning.

---

75. Vendor extensions

Vendor-specific features MUST be isolated.

For example:

vendor::<provider>::...

may represent provider-specific behavior.

Such constructs MUST NOT contaminate portable core semantics.

A program using vendor-specific semantics is portable only to targets that satisfy the extension contract.

---

76. Compile-time semantics

Compile-time computation is distinct from runtime computation.

Compile-time execution MUST have:

- explicit phase;
- deterministic behavior where required;
- bounded compiler resource consumption;
- controlled environmental access;
- reproducible inputs;
- explicit side-effect policy.

Compile-time execution MUST NOT silently depend on the machine used to compile the program when POCO-REAF semantics require portability.

---

77. Metaprogramming

Metaprogramming produces source/AST/semantic structures.

Generated constructs MUST undergo the same semantic validation as handwritten constructs.

Generated code MUST NOT bypass:

- type checking;
- effect checking;
- capability validation;
- security validation;
- resource validation;
- provenance.

---

78. Macros

Macros transform syntax or semantic structures according to defined hygiene and expansion rules.

Macro expansion MUST NOT introduce accidental identifier capture.

Macro-generated machine assumptions MUST be explicit.

---

79. Reflection

Reflection MAY expose semantic information.

It MUST NOT expose unstable implementation details as if they were portable semantics.

For example, a physical device address is not automatically a stable semantic property.

---

80. Semantic equivalence

Let:

P

be a Zamani program.

Let:

I₁(P)
I₂(P)
...
Iₙ(P)

be different valid implementations.

They are semantically equivalent when:

Observable(I₁(P)) =
Observable(I₂(P))

under the same defined inputs, environment contracts, nondeterminism model, and declared resource/accuracy guarantees.

This is the fundamental requirement for optimization and portability.

---

81. Approximate semantics

Approximate computation MUST be explicitly declared.

Examples include:

- numerical tolerance;
- approximate optimization;
- probabilistic algorithms;
- approximate quantum simulation;
- bounded-error algorithms;
- ML inference tolerances.

Approximation MUST NOT be silently introduced merely because a target is limited.

---

82. Numerical semantics

Numerical semantics MUST define:

- precision;
- range;
- rounding;
- overflow;
- underflow;
- NaN/infinity behavior where applicable;
- exact versus approximate representation.

Target-specific floating-point hardware MUST NOT silently redefine the language's mathematical contract.

---

83. Units and dimensions

Quantities with physical units SHOULD be semantically distinguishable from dimensionless numbers.

Examples:

duration
frequency
energy
power
length
mass

Unit conversions MUST be explicit and semantically valid.

This is particularly important for:

- HDL timing;
- quantum pulse timing;
- hardware constraints;
- scientific computing;
- resource specifications.

---

84. Time semantics

Time MAY be represented as:

- logical ordering;
- duration;
- timestamp;
- deadline;
- interval;
- physical time.

These concepts MUST NOT be conflated.

A logical dependency does not automatically imply a physical nanosecond duration.

---

85. Execution semantics

Execution is the realization of semantic computation.

The execution system determines:

- where computation occurs;
- when it occurs;
- how resources are allocated;
- how operations are scheduled;
- how physical resources are selected.

The source semantics remain authoritative.

---

86. Target resolution

Target resolution maps semantic requirements to available capabilities.

Conceptually:

Program Requirements
        +
Program Constraints
        +
Program Preferences
        +
Environment Capabilities
        +
Resource Availability
        ↓
Target Resolution

No target resolution is allowed to modify source meaning.

---

87. Compilation pipeline

The production pipeline MUST remain conceptually:

Source
  ↓
Lexer
  ↓
Parser
  ↓
AST
  ↓
Name Resolution
  ↓
Type Analysis
  ↓
Effect Analysis
  ↓
Capability / Requirement Analysis
  ↓
Semantic Validation
  ↓
Canonical IR
  ↓
Optimization
  ↓
Routing / Placement
  ↓
Scheduling
  ↓
Target Lowering
  ↓
Runtime / Hardware

Domain-specific IRs are introduced only at the appropriate semantic boundary.

---

88. Canonical IR ownership

The semantic model does not replace IR.

For quantum computation:

quantum::ir

is authoritative.

For classical computation, the repository's canonical classical representation is authoritative once identified and integrated.

The grammar MUST NOT create parallel permanent representations.

---

89. IR invariants

Every semantic lowering MUST preserve:

- values;
- types;
- control dependencies;
- data dependencies;
- effects;
- quantum semantics;
- measurement behavior;
- error semantics;
- required capabilities;
- semantic constraints;
- provenance.

---

90. Optimization invariant

For every valid optimization:

Semantics(before) == Semantics(after)

unless an explicit approximation/relaxation contract exists.

---

91. Scheduling invariant

Scheduling MUST preserve:

- dependency order;
- resource exclusivity;
- synchronization;
- timing constraints;
- quantum measurement/control dependencies;
- HDL clock relationships;
- communication ordering.

---

92. Routing invariant

Routing MUST preserve logical identity.

For example:

logical qubit A

MUST remain semantically the same logical qubit after mapping to:

physical qubit X

Routing may change representation, not meaning.

---

93. Resource adaptation

A program MAY adapt to available resources only through explicitly defined semantics.

Valid adaptation:

choose implementation satisfying requirement

Invalid implicit adaptation:

silently remove required computation

---

94. Scaling semantics

Zamani MUST support scale-independent semantics.

A construct is scalable when its grammar and semantic model do not impose an arbitrary maximum on its conceptual cardinality.

Examples:

many qubits
many threads
many nodes
many tensors
many modules
many signals
many devices
many processes
many data elements

are represented through collections, parameters, expressions, or resource models.

---

95. Finite implementation limits

Real implementations are finite.

This does not justify hard-coding those limits into language semantics.

The distinction is:

Language semantic domain:
potentially unbounded within the abstract model

Compiler:
finite resources

Runtime:
finite resources

Hardware:
finite resources

A concrete implementation may reject a program because it cannot realize it.

That is not equivalent to changing the language's semantic maximum.

---

96. "Infinity" interpretation

"Scale to infinity" means:

«The language imposes no arbitrary finite semantic ceiling where the mathematical/domain model does not require one.»

It does NOT claim that a physical machine has infinite:

- memory;
- energy;
- execution time;
- bandwidth;
- qubits;
- processors.

Execution remains bounded by available resources.

---

97. Resource-aware compilation

The compiler MAY select different implementations depending on resources.

For example:

one available accelerator

and:

many available accelerators

may produce different execution plans while preserving semantic meaning.

This is a core POCO-REAF property.

---

98. Graceful degradation

Graceful degradation is valid only when explicitly permitted by program semantics.

Examples:

acceptable approximation

or:

optional optimization

may permit adaptation.

A mandatory correctness requirement may not be degraded.

---

99. Semantic contracts

Every semantic construct MUST define:

- inputs;
- outputs;
- preconditions;
- postconditions;
- effects;
- failure conditions;
- ownership;
- capability requirements;
- resource implications;
- determinism;
- observability.

This contract is what downstream components consume.

---

100. Semantic ownership matrix

Concern| Grammar| Semantic Model| IR| Optimization| Routing| Scheduling| Hardware| Runtime
Syntax| Own| Interpret| No| No| No| No| No| No
Meaning| No| Own| Represent| Preserve| Preserve| Preserve| Realize| Realize
Types| Parse| Own| Represent| Preserve| Preserve| Preserve| Map| Map
Quantum semantics| Parse| Own| quantum::ir| Preserve| Transform mapping| Order| Realize| Execute
QEC algorithm| No| No| No| No| No| No| No| No
ZQN fault model| No| Reference| Consume| Consume| Consume| Consume| Consume| Consume
Optimization| No| No| Consume| Own| No| No| No| No
Routing| No| No| Consume| May prepare| Own| No| Consume| No
Scheduling| No| No| Consume| May prepare| May constrain| Own| Consume| No
Hardware discovery| No| No| No| No| No| No| Own| Consume
Runtime execution| No| No| No| No| No| No| Provide| Own

---

101. Semantic dependency rule

Semantic analysis MAY depend on:

- syntax;
- declarations;
- types;
- domain contracts;
- capability definitions;
- effect definitions.

Semantic analysis MUST NOT depend on:

- a specific physical machine;
- a specific device identifier;
- runtime state that is not explicitly part of program semantics.

---

102. No reverse dependency

The following architecture is prohibited:

runtime
   ↓
grammar meaning

or:

hardware
   ↓
source-language semantics

The correct direction is:

source semantics
      ↓
implementation

---

103. Repository integration

The semantic model MUST integrate with the repository as follows.

Grammar

"grammar/Zamani.g4" defines syntax.

AST

Frontend AST structures represent parsed syntax without becoming an alternative canonical semantic IR.

Semantic analysis

Semantic analysis resolves:

- names;
- types;
- effects;
- capabilities;
- requirements;
- constraints;
- domain semantics.

Classical IR

Classical semantics lower into the repository's canonical classical representation.

"quantum::ir"

Quantum semantics lower into the canonical quantum IR.

QEC

QEC consumes quantum semantics/IR and owns correction algorithms.

ZQN

ZQN owns fault/noise semantics and supplies fault information.

Optimization

Optimization transforms canonical representations while preserving semantics.

Scheduling

Scheduling orders executable operations under dependency/resource/timing contracts.

Routing

Routing maps logical resources to physical resources.

Hardware HAL

Hardware abstraction supplies capabilities, resources, topology, calibration, and state.

Resilience

Resilience decides recovery/adaptation strategies.

Runtime

Runtime realizes the compiled execution plan.

---

104. Semantic-to-QEC boundary

The language may express:

requires fault_tolerance;

or equivalent semantic intent.

It MUST NOT select a QEC decoder merely because the syntax mentions error correction.

QEC policy remains downstream.

---

105. Semantic-to-ZQN boundary

The language may declare fault assumptions.

ZQN determines the actual fault model.

This allows:

source semantics
      ↓
fault requirements
      ↓
ZQN observations/model
      ↓
QEC/resilience/compilation

without duplicating noise representations.

---

106. Semantic-to-resilience boundary

Resilience MAY consume:

- semantic correctness requirements;
- acceptable degradation;
- recovery constraints;
- execution state;
- capability changes.

Resilience MUST NOT redefine language meaning.

---

107. Semantic-to-scheduler boundary

The semantic model supplies:

- dependencies;
- sequencing requirements;
- timing contracts;
- resource requirements;
- synchronization.

The scheduler decides actual placement in time.

---

108. Semantic-to-hardware boundary

The semantic model supplies:

- required capabilities;
- allowed targets;
- constraints;
- resource requirements.

Hardware supplies:

- actual capabilities;
- resources;
- topology;
- calibration;
- current state.

---

109. Semantic diagnostics

Diagnostics MUST distinguish:

where syntax is wrong

from:

what semantic rule was violated

from:

why a target cannot satisfy the program

from:

why runtime execution failed

Diagnostics SHOULD contain:

- stable error category;
- source span;
- semantic entity;
- violated rule;
- relevant requirement;
- available alternatives where applicable;
- provenance.

---

110. Semantic validation phases

Semantic validation SHOULD occur in the following order:

1. Lexical validity
2. Syntactic validity
3. Name resolution
4. Declaration validation
5. Type validation
6. Generic constraint validation
7. Effect validation
8. Ownership/lifetime validation
9. Capability validation
10. Requirement validation
11. Constraint validation
12. Domain semantic validation
13. Cross-domain validation
14. IR-lowering validation

Failures MUST stop the affected phase.

---

111. Cross-domain semantic validation

The semantic analyzer MUST validate interactions among:

- classical + quantum;
- classical + HDL;
- quantum + HDL;
- quantum + hardware;
- quantum + distributed;
- AI + quantum;
- AI + hardware;
- classical + quantum + distributed;
- classical + quantum + HDL + hardware.

Each domain retains ownership of its own semantics.

The cross-domain layer owns only their interfaces.

---

112. Semantic boundary objects

Cross-domain objects MUST carry explicit domain identity.

Examples:

ClassicalValue
QuantumValue
HardwareSignal
ResourceRequirement
CapabilityRequirement
MeasurementResult
ExecutionHandle

They MUST NOT be represented by ambiguous universal objects whose semantics depend on context.

---

113. No duplicate identifiers

The semantic layer MUST reuse repository canonical identifiers where they already exist.

It MUST NOT independently redefine:

- "QubitId";
- "PhysicalQubitId";
- resource identifiers;
- schedule identifiers;
- hardware identifiers.

If a frontend needs a temporary identifier, it MUST be explicitly marked as frontend-local and lowered into the canonical identifier system.

---

114. Generic hardware abstraction

Hardware-specific semantic properties SHOULD be expressed through parameterized interfaces.

Example concept:

requires capability("quantum.operation", operation);

rather than a hard-coded device list.

This allows future hardware to satisfy the same semantic contract.

---

115. Future extensibility

The semantic model MUST be open to new computational domains.

A new domain SHOULD require:

1. namespace;
2. semantic specification;
3. type integration;
4. effect integration;
5. capability integration;
6. requirement integration;
7. IR boundary;
8. lowering contract;
9. diagnostics;
10. tests.

Adding a domain MUST NOT require rewriting unrelated domains.

---

116. Reserved semantic space

The language MUST reserve extension namespaces for future computation models.

Reserved space MUST NOT create executable behavior by itself.

Unknown future constructs MUST fail explicitly unless an extension/dialect is installed.

---

117. Semantic serialization

Semantic artifacts SHOULD be serializable through versioned representations.

Serialization MUST preserve:

- semantic identity;
- types;
- requirements;
- constraints;
- effects;
- domain information;
- provenance;
- version information.

Serialization MUST NOT assume a specific machine.

---

118. Stable hashing

Where semantic hashes are used, they MUST be computed from canonical semantic representation.

They MUST NOT include:

- memory addresses;
- process IDs;
- temporary paths;
- nondeterministic map ordering;
- physical device IDs unless explicitly part of the artifact contract.

---

119. Deterministic canonicalization

Canonical semantic representations MUST provide deterministic ordering for semantically unordered collections.

Rust implementations SHOULD prefer deterministic collections or explicit canonical sorting.

The implementation MUST NOT depend on accidental iteration ordering.

---

120. Resource limits and implementation safety

The language semantic model remains scalable.

The implementation may still need defensive resource limits for:

- parser input;
- recursion;
- AST size;
- compiler memory;
- compile time;
- generated artifact size;
- diagnostic size.

Such limits are implementation safety limits.

They MUST NOT be represented as language semantic maxima.

---

121. No hidden limits

Production implementation MUST audit for hidden constants such as:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_DEVICES
MAX_NODES
MAX_REGISTER_COUNT
MAX_TENSOR_RANK
MAX_PIPELINE_DEPTH

Any such constant must be classified as:

- semantic requirement;
- implementation safety limit;
- target capability;
- resource policy;
- test limit.

It MUST NOT accidentally become a universal language restriction.

---

122. Example: scalable quantum semantics

The following conceptual program:

quantum computation {
    qubits q[count];

    for i in 0..count {
        prepare q[i];
    }

    measure q;
}

has semantics parameterized by "count".

The semantic model does not say:

count <= 32

or:

count <= 64

The actual compiler/runtime determines whether the selected target can satisfy the resource requirement.

---

123. Example: scalable parallel semantics

parallel for item in dataset {
    process(item);
}

does not specify a fixed worker count.

A runtime may execute it using:

one worker

or:

many workers

provided the observable semantics remain valid.

---

124. Example: scalable hardware semantics

hardware module Processor {
    input data;
    output result;

    process {
        result = transform(data);
    }
}

defines hardware behavior.

It does not require:

FPGA device X

or:

N LUTs

or:

M clock regions

unless those properties are explicitly declared as constraints.

---

125. Example: capability-driven execution

Conceptually:

requires {
    capability quantum;
    capability dynamic_circuit;
}

prefer {
    lower_latency;
}

means:

must support quantum
must support dynamic circuits
prefer lower latency

It does not select a specific provider.

---

126. Example: target-specific program

A target-specific declaration MAY exist when physical identity is genuinely part of the program's semantics.

For example, hardware-development code may intentionally identify a physical interface.

Such a declaration MUST be explicitly target-bound.

It MUST NOT contaminate portable semantics.

---

127. Semantic portability classes

Programs SHOULD be classifiable as:

Universal

No target-specific semantic requirements.

Capability-portable

Requires classes of capabilities but not particular providers.

Constraint-portable

Requires capabilities satisfying declared constraints.

Architecture-specific

Requires a particular architecture class.

Device-specific

Requires a particular device or physical configuration.

Deployment-specific

Requires a particular deployment topology.

This classification makes portability explicit.

---

128. Portability is not universality of hardware support

A program can be portable while requiring a capability unavailable on some target.

For example:

requires fault_tolerant_quantum;

may not execute on a small noisy device.

That is a valid target failure.

The source semantics remain portable.

---

129. Semantic preservation under scale

If the same source program is compiled at different scales:

P → Target₁
P → Target₂
P → Target₃

the implementation MUST preserve semantic identity wherever all required contracts are satisfied.

Differences may occur in:

- execution time;
- physical placement;
- parallelism;
- resource consumption;
- instruction selection;
- routing;
- scheduling;
- optimization;
- hardware mapping.

Those are implementation differences.

---

130. Semantic preservation under hardware replacement

Replacing:

hardware A

with:

hardware B

MUST NOT require source changes solely because:

- qubit topology differs;
- CPU count differs;
- GPU count differs;
- FPGA resources differ;
- memory capacity differs;
- scheduling differs;
- calibration differs.

Source changes are required only when semantic requirements themselves differ.

---

131. Semantic preservation under compiler optimization

A compiler MAY transform:

source

into radically different machine instructions.

That is valid when semantic equivalence is maintained.

The source language is not an instruction encoding language.

---

132. Semantic preservation under scheduling

Scheduling may alter:

- order where dependencies allow;
- start times;
- resource assignments;
- concurrency.

It MUST preserve all semantic dependencies.

---

133. Semantic preservation under routing

Routing may introduce:

- swaps;
- data movement;
- communication;
- placement transformations.

Such operations are valid only when they preserve logical semantics.

---

134. Semantic preservation under resilience

Resilience may:

- retry;
- restart;
- recover;
- reroute;
- reschedule;
- recompile;
- switch compatible backends;
- change implementation strategy.

Such adaptation is valid only if the resulting computation remains within the declared semantic contract.

---

135. Acceptance semantics

Execution systems SHOULD distinguish:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

These are execution outcomes.

They MUST NOT be confused with source-level semantic types.

---

136. Semantic degradation

A degraded result is valid only when degradation is permitted by the source contract.

Otherwise:

degraded execution

must not be presented as:

successful exact execution

---

137. Security of semantic interpretation

Semantic interpretation MUST reject:

- ambiguous privileged constructs;
- undeclared capability escalation;
- hidden effects;
- invalid authority transitions;
- malformed dialect declarations;
- incompatible semantic versions.

Parsing a construct MUST NOT grant execution authority.

---

138. No arbitrary environment access during parsing

Parsing MUST remain independent of:

- filesystem state;
- network state;
- hardware discovery;
- device credentials;
- runtime state.

Semantic resolution may consume explicitly supplied environment/capability contexts at the appropriate compiler phase.

---

139. Compiler context

Semantic analysis MAY receive an explicit compilation context containing:

- language version;
- dialect registry;
- capability environment;
- target requirements;
- feature flags;
- resource policy;
- security policy.

The context MUST be explicit.

Global mutable machine-dependent state MUST NOT silently influence semantics.

---

140. Runtime context

Runtime context contains environmental facts such as:

- available resources;
- actual hardware;
- calibration;
- topology;
- runtime health;
- queue state;
- deployment state.

Runtime context MUST NOT redefine source semantics.

---

141. Semantic context immutability

Where practical, semantic contexts SHOULD be immutable after construction.

Changes in hardware state should create new environmental observations rather than mutate the meaning of an already validated program.

---

142. Semantic snapshots

A compiled semantic artifact SHOULD be associated with a semantic snapshot containing:

- language version;
- dialect versions;
- semantic assumptions;
- required capabilities;
- constraints;
- provenance.

This enables deterministic replay and compatibility analysis.

---

143. Replay

Replay semantics MUST distinguish:

- source replay;
- compilation replay;
- execution replay;
- measurement replay;
- randomized replay.

Quantum execution cannot universally guarantee identical physical measurement outcomes.

Where probabilistic behavior is intrinsic, replay means reproduction of the defined execution conditions/trace contract, not necessarily identical physical random outcomes.

---

144. Observability

Only explicitly observable properties become semantic.

Internal:

- cache behavior;
- register allocation;
- physical routing;
- temporary IDs;
- scheduler internals;

are not semantic unless exposed by the language contract.

---

145. ABI boundary

ABI details belong to interoperability/backend layers.

Semantic function identity MUST NOT depend on one ABI.

A function may be lowered to:

- native calling conventions;
- C ABI;
- C++;
- FFI;
- accelerator ABI;
- quantum backend ABI;

without changing its source semantics.

---

146. Foreign functions

Foreign functions are semantic boundaries.

They MUST declare enough information to determine:

- input types;
- output types;
- effects;
- ownership;
- ABI;
- safety;
- failure behavior.

Unknown foreign behavior MUST NOT be assumed pure.

---

147. Error and exception semantics

Exceptions/errors are semantic control-flow events when the language defines them.

Compiler transformations MUST preserve:

- whether errors can occur;
- which handlers can observe them;
- ordering of observable effects.

---

148. Cancellation

Cancellation is an execution effect.

A cancellation-aware operation MUST define what happens when cancellation occurs.

For distributed/quantum/hardware operations, cancellation MUST NOT imply that an already executed physical operation can necessarily be undone.

---

149. Atomicity

Atomicity is semantic only when explicitly defined.

A source operation marked atomic MUST preserve its atomic contract regardless of target implementation.

The implementation MAY use:

- locks;
- transactions;
- hardware atomics;
- distributed protocols;

without changing meaning.

---

150. Transaction semantics

Transactions define:

- begin;
- commit;
- rollback;
- isolation;
- consistency;
- failure behavior.

Hardware-specific implementation is downstream.

---

151. State machines

State-machine semantics define:

- states;
- transitions;
- guards;
- actions;
- entry/exit behavior.

HDL synthesis and runtime implementation remain separate.

---

152. Streams

Streams describe potentially unbounded sequences.

The semantic model MUST NOT impose an arbitrary maximum stream length.

Runtime resource exhaustion remains an implementation condition.

---

153. Infinite computations

The language MAY represent conceptually unbounded computations.

An implementation may terminate due to:

- explicit cancellation;
- resource exhaustion;
- runtime failure;
- target limitations.

Such termination MUST be distinguishable from normal program completion.

---

154. Recursive semantics

Recursion MUST be defined independently of machine stack size.

An implementation may reject execution because of stack/resource limitations.

This does not create a language-level recursion-depth maximum.

---

155. Collection semantics

Collections MAY be:

- finite;
- dynamically sized;
- lazily generated;
- distributed;
- streamed;
- symbolic.

The semantic model MUST NOT require a fixed cardinality unless the type explicitly represents one.

---

156. Parameterized semantics

Program parameters MUST be evaluated within declared type and constraint rules.

Parameterization is a primary mechanism for scalable source programs.

The semantic model SHOULD prefer:

parameter
generic
expression
requirement
constraint
capability

over fixed target constants.

---

157. Resource-polymorphic programs

A resource-polymorphic program is one whose implementation may adapt to available resources while preserving semantics.

Examples:

parallel
distributed
accelerated
quantum
hardware

Resource polymorphism is central to POCO-REAF.

---

158. Target polymorphism

Target polymorphism allows the same semantic program to lower into different implementations.

Conceptually:

one semantic program
        |
        +--> CPU
        +--> GPU
        +--> FPGA
        +--> ASIC
        +--> QPU
        +--> simulator
        +--> distributed system
        +--> future target

---

159. Semantic contracts for future targets

A future target need not be known when the language is designed.

It must only implement the relevant semantic contracts.

This is the core mechanism by which Zamani remains future-extensible.

---

160. Domain addition protocol

Adding a new computational domain requires:

domain namespace
        ↓
domain syntax
        ↓
domain semantic model
        ↓
domain types
        ↓
domain effects
        ↓
domain capabilities
        ↓
domain requirements
        ↓
canonical domain IR
        ↓
lowering contract
        ↓
tests

No unrelated domain should require modification merely because a new domain was added.

---

161. Grammar integration

"grammar/Zamani.g4" MUST express the syntax necessary to represent semantic constructs defined here.

It MUST NOT contain semantic validation that requires runtime hardware discovery.

Grammar predicates MUST remain limited to syntax-level requirements.

---

162. AST integration

The AST MUST preserve all information necessary for semantic analysis:

- source locations;
- names;
- literals;
- declarations;
- expressions;
- types;
- effects;
- domain constructs;
- annotations;
- requirements;
- constraints;
- dialect identity.

The AST MUST NOT prematurely perform hardware mapping.

---

163. Semantic analysis integration

Semantic analysis transforms:

AST

into:

validated semantic representation

It MUST perform:

- name resolution;
- type checking;
- effect checking;
- capability validation;
- requirement validation;
- constraint validation;
- domain validation;
- cross-domain validation.

---

164. Lowering integration

Lowering transforms validated semantics into canonical IR.

It MUST reject any construct for which semantic preservation cannot be established.

Lowering MUST preserve provenance.

---

165. Tooling integration

Tooling MUST be able to consume semantic metadata for:

- diagnostics;
- syntax highlighting;
- completion;
- navigation;
- refactoring;
- documentation;
- static analysis;
- dependency analysis;
- capability inspection.

Tooling MUST NOT invent semantic meanings not present in this specification.

---

166. Documentation integration

"grammar/specification/syntax-model.md" defines syntactic organization.

This document defines semantic meaning.

Other specifications SHOULD therefore follow:

language-principles.md
    ↓
syntax-model.md
    ↓
semantic-model.md
    ↓
compilation-model.md
    ↓
execution-model.md

No document may silently redefine a semantic rule owned here.

---

167. Compatibility integration

"grammar/specification/compatibility.md" governs compatibility policy.

This document governs semantic meaning.

When a compatibility decision changes semantics, the semantic version MUST reflect the change according to the language versioning policy.

---

168. Scalability integration

"grammar/specification/scalability-model.md" governs explicit scalability guarantees.

This document defines the semantic principle underlying those guarantees:

«semantic domains are not artificially bounded by current hardware capacity.»

---

169. POCO-REAF integration

"grammar/specification/poco-reaf.md" defines the POCO-REAF lifecycle.

This document defines the semantic invariant required by it:

same program
    =
same semantic intent

across valid compilation/execution environments.

---

170. Extensibility integration

"grammar/specification/extensibility.md" governs how new syntax and semantic extensions are introduced.

New extensions MUST use the ownership model defined here.

---

171. Reserved-space integration

"grammar/specification/reserved-space.md" governs reserved identifiers and extension namespaces.

Reserved syntax MUST NOT imply reserved semantic behavior unless explicitly specified.

---

172. Validation integration

"grammar/validation/semantic-boundaries.md" MUST verify that grammar constructs do not cross ownership boundaries.

Examples of violations:

grammar defining scheduler algorithms
grammar defining QEC algorithms
grammar defining hardware discovery
grammar defining physical topology
grammar defining backend credentials
grammar defining runtime state

---

173. Hard-coding audit integration

"grammar/validation/hardcoding-audit.md" MUST classify every fixed value discovered in grammar/semantic infrastructure.

The semantic model rejects accidental semantic hard-coding of:

- qubit counts;
- processor counts;
- device counts;
- node counts;
- memory capacities;
- topology;
- addresses;
- accelerator counts.

---

174. Test requirements

Semantic tests MUST include:

Basic semantics

- values;
- types;
- names;
- bindings;
- scopes.

Effects

- pure;
- I/O;
- quantum;
- hardware;
- distributed.

Requirements

- capability satisfied;
- capability unavailable;
- constraint satisfied;
- constraint violated.

Quantum

- arbitrary register sizes;
- logical/physical separation;
- measurement;
- reset;
- dynamic circuits;
- classical control.

Classical

- generic computation;
- vectors;
- matrices;
- tensors;
- parallel computation.

HDL

- combinational;
- sequential;
- timing;
- interfaces;
- parameterized hardware.

Cross-domain

- classical + quantum;
- quantum + hardware;
- classical + HDL;
- quantum + HDL;
- distributed + quantum;
- AI + quantum.

---

175. Negative semantic tests

Tests MUST verify rejection of:

- invalid types;
- invalid ownership;
- invalid capability requirements;
- impossible constraints;
- illegal quantum operations;
- invalid measurement use;
- illegal cross-domain conversions;
- undeclared effects;
- incompatible dialects;
- invalid resource expressions;
- semantic ambiguity.

---

176. Scalability tests

Tests MUST verify that semantic validity does not depend on arbitrary limits.

Examples SHOULD parameterize:

qubit count
register size
collection size
tensor dimension
parallel task count
node count
hardware resource count
program size
module count

The tests MUST distinguish:

semantic validity

from:

implementation resource availability

---

177. Determinism tests

For deterministic source programs:

same source
+
same semantic context
=
same semantic result

regardless of:

- map iteration order;
- compilation thread order;
- incidental host state.

---

178. Round-trip tests

Where serialization exists:

source
 ↓
AST
 ↓
semantic representation
 ↓
serialized artifact
 ↓
deserialized artifact

must preserve semantic identity.

---

179. Provenance tests

Every semantic artifact SHOULD be traceable back to:

- source;
- language version;
- dialects;
- semantic transformations.

---

180. No unsafe compiler implementation

Production semantic-analysis code MUST compile under:

Rust 1.97

and:

Rust 1.97.1

without "unsafe".

The relevant crates/modules SHOULD enforce:

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

This matches the repository's existing production-oriented Rust components, which explicitly enforce those restrictions.

---

181. Memory safety

All semantic-analysis data structures MUST use safe Rust abstractions.

Preferred mechanisms include:

- ownership;
- borrowing;
- "Arc";
- "Rc" where appropriate;
- immutable structures;
- checked indexing;
- explicit error propagation.

Panics MUST NOT be used for ordinary invalid user programs.

---

182. Error propagation

Semantic failures MUST use structured errors.

Production code SHOULD prefer:

Result<T, E>

over:

panic!

for user-controlled invalid input.

---

183. Deterministic data structures

Where semantic ordering matters, implementations SHOULD use:

- deterministic vectors;
- ordered maps;
- ordered sets;
- canonical sorting.

The existing repository's production-oriented QEC implementation demonstrates the importance of deterministic collections and deterministic tie-breaking.

---

184. Semantic resource accounting

Resource accounting MUST remain external to the semantic meaning of the program.

The semantic layer may state:

requires R

The resource manager determines:

available R

The scheduler determines:

when R is used

The runtime determines:

whether R remains available

---

185. Semantic cancellation

Cancellation MUST preserve semantic truth.

A cancelled computation is not equivalent to a completed computation.

Execution APIs MUST distinguish:

completed
failed
cancelled
partially completed

---

186. Semantic lifecycle

A program passes through:

Parsed
  ↓
Resolved
  ↓
Typed
  ↓
EffectChecked
  ↓
CapabilityChecked
  ↓
ConstraintChecked
  ↓
SemanticallyValidated
  ↓
Lowered
  ↓
Optimized
  ↓
Placed/Routed
  ↓
Scheduled
  ↓
Compiled
  ↓
Dispatched
  ↓
Executed
  ↓
Observed

Each state has distinct ownership.

---

187. Invalid state transitions

A program MUST NOT be considered:

Compiled

if semantic validation failed.

It MUST NOT be considered:

Executed

if dispatch failed.

It MUST NOT be considered:

Correct

merely because hardware returned a result.

Verification remains necessary where the program contract requires it.

---

188. Verification

Verification MAY include:

- type correctness;
- IR invariants;
- semantic equivalence;
- resource compliance;
- quantum correctness;
- hardware constraints;
- execution result validation.

Verification is downstream from semantic definition but MUST preserve semantic contracts.

---

189. Formal semantic model

Conceptually, define:

⟦P⟧_E

as the meaning of program "P" under semantic environment "E".

The semantic environment contains only information allowed by the language contract.

For valid target implementations:

Implement(P, T)

must realize:

⟦P⟧

subject to declared:

- nondeterminism;
- approximation;
- resource constraints;
- effects;
- environmental contracts.

---

190. Target realization theorem

For a valid implementation target "T":

Capabilities(T) ⊨ Requirements(P)

and:

Constraints(P, T) = satisfied

then:

Execute(Compile(P, T))

MUST realize the semantics of:

⟦P⟧

within the program's declared execution contract.

---

191. Failure theorem

If:

Capabilities(T) ⊭ Requirements(P)

then compilation/execution MUST fail explicitly.

It MUST NOT silently produce a different program.

---

192. Optimization theorem

For an optimization transformation:

O(P) = P'

the optimizer MUST establish:

⟦P⟧ = ⟦P'⟧

unless an explicit approximation contract permits a controlled difference.

---

193. Routing theorem

For routing:

R(P) = P'

the logical semantics MUST satisfy:

LogicalMeaning(P) = LogicalMeaning(P')

Physical topology may differ.

---

194. Scheduling theorem

For scheduling:

S(P) = P'

the execution order may change only where dependencies and observable semantics permit.

---

195. Hardware substitution theorem

For hardware targets "H1" and "H2":

H1 ⊨ Requirements(P)
H2 ⊨ Requirements(P)

then both valid implementations MUST realize the same declared program semantics, even if:

H1 != H2

in architecture, capacity, topology, calibration, or implementation.

---

196. Future-machine theorem

A future machine need not be known when Zamani source is written.

If the future machine provides the capabilities required by the semantic program, an implementation may be developed later without changing the program's semantic meaning.

This is the semantic foundation of:

Run Forever

in POCO-REAF.

---

197. Semantic anti-patterns

The following are prohibited:

semantic MAX_QUBITS
semantic MAX_CORES
semantic MAX_GPUS
semantic fixed_topology
semantic fixed_device_id
semantic fixed_memory_capacity
semantic fixed_cluster_size
semantic fixed_address

unless explicitly part of a target-specific language construct.

Also prohibited:

grammar → hardware discovery
grammar → runtime state
grammar → QEC algorithm
grammar → scheduler algorithm
grammar → optimizer implementation
grammar → physical routing

---

198. Correct architecture

The required architecture is:

                 Zamani Source
                       |
                       v
                  Zamani Grammar
                       |
                       v
                       AST
                       |
                       v
               Semantic Analysis
                       |
          +------------+------------+
          |            |            |
          v            v            v
       Classical    Quantum       HDL
       Semantics    Semantics    Semantics
          |            |            |
          v            v            v
    Classical IR   quantum::ir   HDL/Semantic IR
          |            |            |
          +------------+------------+
                       |
                       v
                 Optimization
                       |
                       v
             Routing / Placement
                       |
                       v
                   Scheduling
                       |
          +------------+-------------+
          |            |             |
          v            v             v
       Hardware       ZQN       Resilience
          |            |             |
          +------------+-------------+
                       |
                       v
                     Runtime
                       |
                       v
                    Machine

---

199. Completion criteria

"semantic-model.md" is complete when:

- semantic authority is unambiguous;
- syntax and semantics are separated;
- semantic ownership is defined;
- quantum semantics integrate with "quantum::ir";
- QEC is not duplicated;
- ZQN is not duplicated;
- scheduling is not duplicated;
- routing is not duplicated;
- optimization is not duplicated;
- hardware discovery is not duplicated;
- runtime behavior is not duplicated;
- resource semantics are target-independent;
- capability semantics are extensible;
- requirement/constraint/preference/hint semantics are distinct;
- physical resources are not hard-coded;
- scalability semantics are explicit;
- POCO-REAF is formally supported;
- classical semantics are covered;
- quantum semantics are covered;
- HDL semantics are covered;
- hybrid semantics are covered;
- distributed semantics are covered;
- AI/data/accelerator semantics have extensibility contracts;
- security semantics are preserved;
- determinism rules exist;
- error semantics exist;
- provenance rules exist;
- versioning rules exist;
- dialect rules exist;
- cross-domain rules exist;
- compiler integration is defined;
- runtime integration is defined;
- test requirements are defined;
- Rust implementation constraints are defined;
- "unsafe" is prohibited;
- no arbitrary machine-size semantic limit exists.

---

200. Final semantic invariant

The most important invariant of Zamani is:

SOURCE SEMANTICS
        ≠
CURRENT HARDWARE

and:

SOURCE SEMANTICS
        >
TARGET IMPLEMENTATION

Therefore:

One Program
    ↓
One Semantic Meaning
    ↓
Many Compilers / Compilation Strategies
    ↓
Many IR Implementations
    ↓
Many Schedules
    ↓
Many Physical Mappings
    ↓
Many Machines
    ↓
Many Scales
    ↓
Many Execution Environments
    ↓
Future Machines

without requiring the programmer to rewrite the computation merely because the available machine changed.

The implementation may change.

The representation may change.

The optimization may change.

The schedule may change.

The routing may change.

The hardware may change.

The resource allocation may change.

The runtime may change.

The semantic meaning of the program does not change unless the language program itself explicitly changes its semantic contract.

That invariant is the semantic foundation of:

Zamani — From Atom to Everywhere

and:

POCO-REAF

Program Once → Compile Once → Run Everywhere → Run Anywhere → Run ForeverThis artifact is deliberately a semantic contract, not another grammar or IR. That separation is important given the repository's existing pattern of explicit ownership boundaries: for example, QEC's distributed subsystem explicitly says it does not own partition geometry, decoder mathematics, network transport, checkpoint serialization, or global scheduling policy.  Likewise, existing QEC code treats resource limits as an authoritative external policy rather than introducing another production ceiling. 

