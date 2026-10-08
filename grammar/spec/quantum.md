Zamani Quantum Language Specification

Path: "grammar/spec/quantum.md"
Status: Normative
Domain: Quantum computation and quantum-classical computation
Language: Zamani
Grammar technology: ANTLR4
Rust baseline: Rust 1.97 or later
Rust edition: 2021
Rust safety: Safe Rust only; production Rust MUST NOT use "unsafe"
Canonical quantum semantic boundary: "quantum::ir"
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document is the normative specification for the quantum domain of the Zamani programming language.

It defines the language-level contract for expressing:

- quantum computation;
- quantum resources;
- logical quantum computation;
- explicit physical-resource intent;
- quantum operations;
- parameterized operations;
- controlled operations;
- inverse/adjoint operations;
- quantum states;
- state preparation;
- measurement;
- reset;
- observables;
- channels;
- noise intent;
- dynamic circuits;
- classical feed-forward;
- quantum-classical interaction;
- quantum kernels;
- quantum learning;
- quantum inference;
- uncertainty;
- adaptive execution;
- error-correction intent;
- resilience requirements;
- quantum capabilities;
- quantum resource requirements;
- quantum policies;
- quantum provenance;
- quantum interoperability;
- quantum dialects.

This document also defines the boundaries between:

source language
    ↓
lexer
    ↓
ANTLR parser
    ↓
frontend AST
    ↓
structural validation
    ↓
semantic analysis
    ↓
canonical semantic representation
    ↓
quantum::ir
    ↓
optimization / decomposition
    ↓
resource analysis
    ↓
routing
    ↓
scheduling
    ↓
QEC / resilience / fault handling
    ↓
ZQN
    ↓
HAL
    ↓
target realization
    ↓
runtime

The quantum language describes portable computational meaning.

It does not prescribe a particular physical realization.

---

2. Normative Language

The following terms are normative:

- MUST — mandatory.
- MUST NOT — prohibited.
- SHOULD — recommended unless a documented architectural reason requires otherwise.
- SHOULD NOT — normally prohibited unless justified.
- MAY — permitted but optional.
- CAN — capability rather than requirement.
- IMPLEMENTATION LIMIT — a limit imposed by a particular compiler, runtime, target, or deployment environment rather than by the language.
- LANGUAGE LIMIT — a restriction inherent in Zamani semantics.

An implementation claiming conformance MUST distinguish language rules from implementation policies.

---

3. Architectural Authority

Zamani is one programming language.

Quantum computation is a domain of Zamani.

The quantum subsystem MUST NOT become a second language with a competing semantic authority.

The authority chain is:

language specification
        ↓
grammar/spec/quantum.md
        ↓
grammar/quantum/*.g4
        ↓
canonical lexer
        ↓
frontend AST
        ↓
semantic analysis
        ↓
quantum::ir
        ↓
compiler / runtime

The following MUST NOT independently redefine Zamani quantum semantics:

- external quantum source formats;
- vendor languages;
- vendor SDKs;
- simulator APIs;
- target instruction sets;
- hardware drivers;
- QPU-specific programming models;
- optimization implementations;
- routing implementations;
- scheduling implementations;
- QEC implementations;
- calibration systems.

External formats are interoperability surfaces.

They are not language authorities.

---

4. Repository Authority

The quantum specification integrates with the repository as follows.

4.1 "grammar/DESIGN.md"

Owns global grammar architecture.

This specification MUST conform to it.

---

4.2 "grammar/Zamani.g4"

Owns root grammar composition.

It MUST NOT duplicate quantum leaf syntax.

Quantum dispatch enters the quantum domain through the canonical quantum grammar boundary.

---

4.3 "grammar/antlr/ZamaniLexer.g4"

Owns canonical lexical behavior.

Quantum-specific tokens MUST be introduced only when a construct genuinely requires reserved lexical treatment.

Quantum operation names MUST NOT become a finite lexer catalogue.

For example, the lexer MUST NOT require:

H
X
Y
Z
CNOT
CZ
SWAP
RX
RY
RZ

to be reserved quantum keywords.

They remain names unless a separate lexical rule explicitly requires otherwise.

---

4.4 "grammar/quantum/quantum.g4"

Owns quantum-domain grammar orchestration.

It MUST compose independently owned quantum parser grammars.

It MUST NOT become a second semantic specification.

---

4.5 Quantum leaf grammars

Existing repository files remain owners of their respective syntax.

Examples include:

grammar/quantum/operations.g4
grammar/quantum/measurement.g4
grammar/quantum/reset.g4
grammar/quantum/types.g4
grammar/quantum/states.g4
grammar/quantum/qubits.g4
grammar/quantum/quantum-registers.g4
grammar/quantum/circuits.g4
grammar/quantum/parameters.g4
grammar/quantum/controls.g4
grammar/quantum/adjoints.g4
grammar/quantum/observables.g4
grammar/quantum/channels.g4
grammar/quantum/noise.g4
grammar/quantum/error-correction.g4
grammar/quantum/dynamic-circuits.g4
grammar/quantum/mid-circuit-control.g4
grammar/quantum/quantum-classical.g4
grammar/quantum/quantum-resources.g4
grammar/quantum/quantum-capabilities.g4
grammar/quantum/quantum-dialects.g4
grammar/quantum/provenance.g4
grammar/quantum/kernels.g4

A feature MUST have one canonical grammar owner.

---

5. Canonical Semantic Boundary

The canonical quantum semantic boundary is:

quantum::ir

There MUST NOT be a competing canonical quantum IR.

The following concepts are therefore prohibited as alternative canonical representations:

QuantumIR
QuantumGateIR
QuantumCircuitIR
QuantumHardwareIR
QuantumFrontendIR
QuantumGrammarIR

unless they are explicitly documented as non-canonical intermediate implementation structures.

If a temporary internal representation exists, it MUST have a documented conversion into "quantum::ir".

---

6. POCO-REAF

Quantum programs MUST be designed around:

Program Once
Compile Once
Run Everywhere
Anywhere
Forever

This does not mean that one binary can execute unchanged on every physical machine.

It means that source-level computational meaning remains stable while compilation and realization adapt to available:

- resources;
- capabilities;
- target properties;
- policies;
- topology;
- execution models;
- resilience requirements;
- interoperability constraints.

The same source program MAY therefore be considered for:

tiny systems
CPU
multicore CPU
GPU
FPGA
ASIC
accelerator
QPU
quantum simulator
tensor-network simulator
distributed simulator
HPC
cluster
cloud
hybrid systems
future computational substrates

provided that the target can satisfy the program's semantic requirements.

---

7. Meaning of "Infinite Scale"

Zamani uses "infinite scale" architecturally, not physically.

It means:

«The language MUST NOT impose an artificial finite machine-size ceiling.»

It does NOT mean that:

- infinite memory exists;
- infinite qubits exist;
- infinite execution time exists;
- an infinite circuit can be physically executed;
- a compiler has infinite resources.

Every concrete compilation remains finite.

The following distinction is mandatory:

language expressiveness
        ≠
compiler capacity
        ≠
runtime capacity
        ≠
target capacity
        ≠
physical capacity

---

8. No Artificial Quantum Limits

The language MUST NOT define universal machine-size constants such as:

MAX_QUBITS
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS
MAX_REGISTERS
MAX_REGISTER_SIZE
MAX_CIRCUIT_WIDTH
MAX_CIRCUIT_DEPTH
MAX_OPERATIONS
MAX_CONTROLS
MAX_PARAMETERS
MAX_MEASUREMENTS
MAX_CLASSICAL_BITS
MAX_SHOTS
MAX_TIMELINES
MAX_QPUS
MAX_QUANTUM_MEMORY

The grammar MUST NOT encode equivalent restrictions under different names.

A compiler MAY have configurable resource policies.

Those policies are implementation constraints.

They MUST NOT redefine the language.

---

9. Program Constants Are Not Language Limits

This is valid:

qubit[1024]

because "1024" is program data.

This is not a language rule:

quantum_register_maximum = 1024

The distinction is:

program-defined value
        ≠
language-defined capacity

A program MAY intentionally request a finite number of qubits.

The language MUST remain capable of representing larger values without requiring a grammar redesign.

---

10. Quantum Resource Abstraction

The source-level quantum resource abstraction is:

qubit

A source-level "qubit" does not inherently identify:

- a vendor;
- a QPU;
- a physical qubit;
- a hardware channel;
- a simulator array position;
- a topology coordinate;
- a calibration object.

Those are downstream realization concerns.

---

11. Logical and Physical Quantum Resources

Zamani distinguishes:

source-level qubit
logical qubit
physical qubit

These MUST NOT be silently conflated.

Conceptually:

logical quantum resource
        ↓
mapping / encoding / allocation
        ↓
physical quantum resource

The mapping is downstream.

A logical qubit MUST NOT silently become a physical qubit merely because a backend is selected.

---

12. Physical Intent

Physical quantum resource intent MAY be expressed explicitly.

When physical intent is present, it MUST remain distinguishable from portable logical intent.

Examples of physical information include:

- physical qubit identity;
- physical topology;
- physical resource class;
- explicit target placement.

Such constructs are target-sensitive.

They MUST NOT silently contaminate ordinary portable quantum source.

---

13. Quantum Cardinality

Quantum cardinality MAY be:

- literal;
- symbolic;
- generic;
- parameterized;
- derived from another value;
- dependent on a resource expression;
- resolved during semantic analysis;
- resolved during compilation;
- determined by execution policy where the language permits it.

Examples:

qubit[n]
qubit[width]
qubit[2 * n]
qubit[required_width]

The parser MUST preserve the expression.

It MUST NOT:

- allocate resources;
- choose a target;
- select physical qubits;
- truncate the value;
- impose a machine-size limit.

---

14. Cardinality Representation

The source language MUST NOT prematurely require cardinality to fit a particular implementation integer type merely because an implementation happens to use:

u32
u64
usize
i32
i64

A later compiler layer MAY choose a representation appropriate to its current task.

If a concrete representation cannot represent the requested value, the compiler MUST produce an explicit resource or implementation diagnostic.

It MUST NOT reinterpret the source program.

---

15. Quantum Registers

Quantum registers are collections of quantum resources.

They MAY be:

- fixed-size;
- symbolic;
- generic;
- parameterized;
- dynamically determined where supported;
- resource-dependent.

A register MUST NOT implicitly mean:

physical qubits 0..N

unless physical placement was explicitly requested.

---

16. Quantum Types

Quantum-specific types are source-level semantic abstractions.

The quantum type system MUST integrate with the universal Zamani type system.

It MUST NOT become a separate type language.

Quantum types MAY include concepts corresponding to:

qubit
logical qubit
quantum<T>
quantum collection
quantum state
observable
quantum channel
quantum resource

The exact surface representation is owned by the corresponding grammar files.

---

17. Generic Quantum Types

Quantum types MUST participate in the general Zamani generic type system.

They MUST support the repository's applicable mechanisms for:

- generic parameters;
- type constraints;
- associated types;
- type-level values;
- bounds;
- variance where applicable.

Quantum syntax MUST NOT duplicate universal generic syntax.

---

18. Operation Model

Quantum operations MUST be open-world.

The grammar MUST NOT enumerate every quantum gate.

The semantic operation model is conceptually:

Operation {
    name,
    namespace,
    operands,
    parameters,
    results,
    attributes,
    modifiers,
    effects,
    capabilities,
    source
}

The actual AST and IR structures are owned by their respective implementation layers.

---

19. Operation Identity

An operation identity consists of semantic name information.

It MAY be:

operation
namespace::operation
library::operation
dialect::operation
vendor::operation
future::operation

Operation names are data.

The grammar MUST NOT require a parser modification whenever a new operation is introduced.

---

20. Built-In Operations

Zamani MAY provide standard quantum operations through the standard library or semantic registry.

Such operations are NOT a finite grammar catalogue.

The language MUST remain able to represent:

- standard operations;
- user operations;
- library operations;
- vendor operations;
- dialect operations;
- future operations.

---

21. Operation Parameters

Operations MAY accept zero or more parameters.

Parameters MAY be:

- literals;
- symbols;
- expressions;
- compile-time values;
- runtime values;
- generic values;
- derived values.

The grammar MUST NOT impose a universal parameter count.

Parameter validity belongs to semantic analysis.

---

22. Operation Operands

Operations MAY operate on:

- individual qubits;
- qubit collections;
- logical qubits;
- explicitly physical resources;
- classical values where appropriate;
- observables;
- states;
- domain-specific resources.

Operand arity belongs to operation semantics.

It is not a global grammar limit.

---

23. Operation Results

Quantum operations MAY produce:

- quantum values;
- classical values;
- measurement results;
- handles;
- resource information;
- domain-specific values.

Results MUST remain represented in the AST/semantic model when they affect program meaning.

They MUST NOT disappear during lowering.

---

24. Operation Modifiers

Operations MAY have structured modifiers.

Examples include:

control
adjoint
inverse
power
repeat
conditional

Modifiers MUST be represented structurally.

A parser MUST NOT flatten them into an opaque string.

Semantic analysis determines whether a modifier is valid for a particular operation.

---

25. Controlled Operations

Controlled operations MUST be compositional.

The language MUST NOT impose a universal limit such as:

one control
two controls
three controls

The number of controls is determined by the program and target feasibility.

The semantic structure MUST preserve:

base operation
controls
control polarity
targets
modifiers

---

26. Adjoint and Inverse Operations

Adjoint/inverse intent MAY be expressed explicitly.

The parser records the intent.

Semantic analysis determines whether the operation admits the requested transformation.

Optimization and lowering determine the concrete realization.

The parser MUST NOT eagerly replace an adjoint operation with a hardware-specific sequence.

---

27. User-Defined Quantum Operations

Zamani MUST support user-defined quantum operations.

A user-defined operation MAY contain:

- parameters;
- quantum operands;
- classical values;
- declarations;
- operations;
- control flow where permitted;
- resource requirements;
- capabilities;
- effects;
- contracts;
- policies;
- provenance.

A user-defined operation does not need to correspond to a built-in gate.

---

28. Operation Extensibility

Adding a new quantum operation MUST normally require:

operation declaration / library registration
        ↓
semantic definition
        ↓
optional dialect metadata
        ↓
lowering support
        ↓
tests

It MUST NOT require modifying:

ZamaniLexer.g4
quantum.g4
quantum.md

merely because the operation has a new name.

---

29. Quantum Circuits

A quantum circuit is a semantic computation structure.

It MAY contain:

- resource declarations;
- operations;
- state preparation;
- measurements;
- reset;
- classical control;
- dynamic control;
- contracts;
- resource requirements;
- capability requirements;
- policies;
- provenance.

A circuit is not inherently a physical device description.

---

30. Circuit Width and Depth

Width and depth MAY be computed as analysis properties.

They MUST NOT be language-level limits.

The compiler MAY report:

width
depth
critical path
operation count
resource pressure

These are analysis results.

They do not constrain what the language can represent.

---

31. Quantum State Semantics

Quantum state syntax expresses semantic state intent.

A state MAY be represented conceptually using:

|0>
|1>
|+>
|->
|psi>

or generalized state expressions.

State syntax MUST NOT imply a particular storage representation.

---

32. State Representation Independence

A quantum state MAY be represented downstream using:

- state vectors;
- density matrices;
- tensor networks;
- stabilizer representations;
- decision diagrams;
- symbolic forms;
- compressed forms;
- hardware-native representations;
- future representations.

The language MUST NOT mandate any one representation.

---

33. No Mandatory State-Vector Allocation

The declaration:

qubit[n]

MUST NOT imply:

allocate 2^n amplitudes

That is a simulator implementation strategy.

It is not quantum language semantics.

---

34. State Preparation

State preparation expresses computational intent.

It MUST NOT prescribe:

- pulses;
- waveform shapes;
- laser sequences;
- cooling procedures;
- physical transport;
- calibration identifiers;
- control electronics.

Those belong downstream.

---

35. Reset

Reset is a semantic quantum operation.

It expresses a requirement to place the relevant quantum resource into the specified reset state according to Zamani semantics.

The target MAY implement reset using:

- native reset;
- measurement and conditional correction;
- active cooling;
- state reinitialization;
- another semantically equivalent method.

The realization MUST preserve observable program meaning.

---

36. Measurement

Measurement is a first-class semantic operation.

A measurement MUST preserve, as applicable:

- measured resource;
- basis or observable;
- result identity;
- destination;
- metadata;
- source location;
- semantic ordering.

The parser MUST NOT insert implicit measurements merely because a backend needs them.

---

37. Measurement Results

Measurement results cross the quantum/classical boundary.

The semantic flow is:

quantum state
      ↓
measurement
      ↓
classical result
      ↓
classical computation

Measurement results MUST remain available to subsequent computation when the source program uses them.

---

38. Measurement Basis

The measurement basis MAY be:

- standard;
- parameterized;
- observable-defined;
- library-defined;
- dialect-defined.

The grammar MUST remain extensible.

A basis identifier does not inherently identify a physical measurement mechanism.

---

39. Mid-Circuit Measurement

Mid-circuit measurement is a first-class quantum capability.

A valid program MAY express:

quantum operation
measurement
classical computation
conditional quantum operation

without requiring all measurements to occur at program termination.

---

40. Dynamic Circuits

Quantum programs MAY contain runtime-dependent control flow.

Examples include:

if measurement_result { ... }

while condition { ... }

repeat until condition

The semantic representation MUST preserve dynamic dependencies.

A compiler MAY lower dynamic control to:

- QPU-native dynamic control;
- host control;
- embedded controller control;
- statically specialized control;
- hybrid execution.

The chosen strategy MUST preserve semantics.

---

41. Classical Feed-Forward

Classical feed-forward is explicit semantic control flow.

For example:

measure q -> result

if result {
    apply operation(target)
}

MUST remain distinguishable from a static circuit.

The compiler MAY choose the most appropriate target realization.

---

42. Quantum/Classical Integration

Quantum and classical computation are parts of one Zamani program.

The semantic model MUST preserve:

classical → quantum
quantum → classical
classical control → quantum
measurement → classical
classical parameter → quantum operation

There MUST NOT be a separate mandatory classical language embedded inside quantum programs.

---

43. Hybrid Computation

Hybrid programs MAY combine:

- classical computation;
- quantum computation;
- AI/ML;
- tensor computation;
- distributed execution;
- accelerators;
- HDL/hardware operations.

The quantum subsystem MUST reuse universal Zamani contracts for:

- types;
- effects;
- resources;
- capabilities;
- contracts;
- policies;
- provenance.

---

44. Observables

Observables are semantic mathematical objects.

An observable MAY be:

- named;
- composed;
- parameterized;
- tensor-structured;
- library-defined;
- dialect-defined.

The source representation MUST remain independent of the target's internal observable representation.

---

45. Quantum Channels

Quantum channels represent transformations of quantum states where the language model supports them.

They MAY represent:

- unitary transformations;
- noisy transformations;
- completely positive maps;
- measurement channels;
- environment interaction;
- library-defined transformations.

The semantic model determines validity.

The grammar does not implement the mathematics.

---

46. Noise

Noise syntax expresses noise intent or noise modelling where supported.

It MUST NOT imply that a particular physical noise process exists on every target.

Noise MAY be used for:

- simulation;
- verification;
- analysis;
- robustness testing;
- fault modelling;
- target-aware compilation.

Physical noise data belongs to target/runtime systems.

---

47. Determinism

The language MUST distinguish:

deterministic source semantics

from:

probabilistic quantum outcomes

A deterministic program may legitimately produce probabilistic measurement outcomes.

Determinism therefore means that:

- parsing is deterministic;
- semantic interpretation is deterministic;
- canonical serialization is deterministic where specified;
- compilation decisions are reproducible when the same deterministic inputs/policies are supplied.

It does not mean that quantum measurement outcomes are deterministic.

---

48. Randomness

Quantum randomness MUST be represented through the effect system where applicable.

Measurement and other nondeterministic operations MAY produce effects corresponding to randomness.

The effect MUST NOT be silently removed during lowering.

---

49. Uncertainty

Quantum programs MAY express uncertainty.

Possible concepts include:

probability
distribution
confidence
uncertainty
belief
expected value

These are semantic concepts.

They MUST NOT be tied to one probabilistic implementation.

---

50. Resource Requirements

Quantum source MAY express resource requirements.

Examples include:

requires qubits >= required_qubits;
requires memory >= required_memory;

or equivalent resource expressions defined by the universal resource specification.

A resource requirement describes:

what is needed

not:

what physical resource has been allocated

---

51. Capabilities

Quantum source MAY require capabilities.

Examples include:

requires capability("quantum.measurement");
requires capability("quantum.mid_circuit_measurement");
requires capability("quantum.dynamic_control");
requires capability("quantum.fault_tolerance");

Capability identifiers MUST remain open-ended.

The language MUST NOT define a permanently closed hardware catalogue.

---

52. Resource and Capability Separation

The following are distinct:

resource
capability
constraint
preference
allocation
target identity

For example:

requires qubits >= n

does not select a QPU.

Likewise:

requires capability("quantum.measurement")

does not select a vendor.

---

53. Preferences

A program MAY express preferences.

Examples conceptually include:

prefer capability("quantum.low_latency");
prefer capability("quantum.high_fidelity");

Preferences MUST NOT be treated as mandatory requirements unless explicitly promoted by semantic policy.

---

54. Constraints

A program MAY express constraints.

Examples include:

constrain latency <= limit;
constrain topology(required_topology);
constrain fidelity >= required_fidelity;

The exact generic resource constraint syntax is owned by the resource subsystem.

Quantum syntax MUST reuse that subsystem rather than create a duplicate constraint language.

---

55. Allow and Forbid

Where supported by the universal policy system, quantum execution MAY express:

allow ...
forbid ...

These are policy semantics.

They MUST NOT become quantum-specific copies of the universal policy model.

---

56. Resource Negotiation

Resource negotiation occurs downstream of parsing.

Conceptually:

program requirements
        +
program preferences
        +
program policies
        +
target capabilities
        +
target resources
        ↓
feasibility / realization plan

The language does not perform hardware discovery.

---

57. Resource Failure

If a valid program requires resources unavailable on a target, the compiler/runtime MUST distinguish:

invalid source

from:

valid source
but target infeasible

The latter MUST NOT cause the source semantics to be silently changed.

Possible outcomes include:

REJECT
RETRY
RECOVER
ESCALATE
DEGRADED_ACCEPT

according to the repository's resilience model.

---

58. Adaptive Quantum Execution

Quantum execution MAY adapt based on:

- measurement results;
- target state;
- resource availability;
- runtime conditions;
- resilience state;
- policies;
- learning results.

Adaptive execution MUST remain controlled by explicit semantic contracts.

It MUST NOT mean unrestricted self-modifying source code.

---

59. Adaptation

If quantum adaptation is permitted, it MUST integrate with the universal adaptation model.

An adaptation operation SHOULD preserve:

- authorization;
- policy;
- effects;
- capabilities;
- resource requirements;
- provenance;
- validation state.

Adaptation MUST NOT silently rewrite the language semantics.

---

60. Quantum Learning

Quantum learning is a semantic composition of:

learning
+
quantum computation
+
classical computation

It MUST reuse the universal learning model.

The quantum grammar MUST NOT enumerate every learning algorithm.

New algorithms SHOULD normally be introduced through:

- libraries;
- semantic registrations;
- dialects;
- compiler lowering;
- runtime implementations.

---

61. Quantum Inference and Reasoning

Quantum programs MAY participate in generic:

- inference;
- deduction;
- reasoning;
- evidence processing;
- uncertainty handling.

The quantum subsystem MUST NOT create a separate reasoning language.

The generic reasoning system remains the semantic owner.

---

62. Neural-Symbolic Quantum Computation

A Zamani program MAY combine:

symbolic reasoning
+
learned model
+
classical computation
+
quantum computation

The quantum grammar MUST preserve the boundaries required by the universal semantic model.

There MUST NOT be a second AI-specific quantum IR.

---

63. Contracts

Quantum constructs MAY participate in universal contracts:

requires
ensures
invariant
assume
guarantee
property
assert

Contracts may constrain:

- quantum state properties;
- measurement properties;
- resource requirements;
- capabilities;
- correctness;
- invariants;
- expected outcomes;
- execution conditions.

The universal validation subsystem owns contract semantics.

---

64. Evidence and Provenance

Quantum computation SHOULD preserve provenance sufficient for:

- diagnostics;
- reproducibility;
- scientific auditing;
- optimization tracing;
- transformation tracing;
- explanation;
- verification.

Provenance MAY record:

source
derived_from
generated_by
transformed_by
verified_by
reason
evidence
version
timestamp
policy
decision

The universal provenance system owns the canonical provenance model.

---

65. Explainability

Quantum transformations MAY require explanations.

An explanation MAY describe:

- why an optimization occurred;
- why a target was selected;
- why a route was selected;
- why a resource requirement failed;
- why a fallback occurred;
- why a QEC strategy was selected.

The grammar records explainability intent where syntax exists.

The compiler/runtime supplies the explanation.

---

66. Policies

Quantum execution MUST integrate with the universal policy system.

Policies MAY constrain:

- resources;
- capabilities;
- execution;
- security;
- adaptation;
- simulation;
- deployment;
- interoperability;
- resilience.

Quantum policy syntax MUST NOT create a competing policy architecture.

---

67. Sandboxing

Quantum code executing through simulation, FFI, native interfaces, plugins, or external services MAY be subject to sandbox policies.

Sandboxing MAY constrain:

- filesystem;
- network;
- native calls;
- foreign calls;
- resources;
- reflection;
- code generation;
- adaptation;
- external execution.

The quantum grammar does not implement sandboxing.

---

68. Error-Correction Intent

Quantum source MAY express QEC intent.

Examples include requirements concerning:

- fault tolerance;
- logical qubits;
- resilience;
- encoded computation;
- error model assumptions;
- correction requirements.

The source language MUST NOT implement QEC algorithms.

---

69. QEC Boundary

The grammar MUST NOT own:

- QEC decoder implementations;
- syndrome-decoding algorithms;
- physical error-correction schedules;
- calibration;
- physical-qubit allocation;
- resource accounting implementation;
- backend-specific QEC mechanisms.

These belong downstream.

---

70. QEC Resource Policies

Existing implementation concepts such as resource limits or runtime policies remain downstream.

A quantum grammar MUST NOT duplicate structures such as a runtime resource-limit object.

For example, an implementation policy corresponding to:

QecLimits

is not a grammar-level semantic type.

The source language expresses requirements.

The resource/resilience subsystem decides feasibility.

---

71. Logical QEC Semantics

The source program MAY request logical computation.

For example, the semantic intent may be:

requires capability("quantum.fault_tolerance");
requires capability("quantum.logical_qubit");

The compiler determines:

- encoding;
- code;
- distance;
- physical overhead;
- syndrome strategy;
- decoder;
- target mapping.

Those decisions MUST remain downstream.

---

72. Resilience

Quantum execution MAY participate in the repository's resilience model.

Relevant states include:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

Relevant outcomes include:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

The quantum grammar does not own those runtime states.

It preserves the source-level requirements that allow downstream resilience decisions.

---

73. Dynamic Resource Availability

A resource may become unavailable after compilation.

The system MUST distinguish:

source validity

from:

current execution feasibility

If a target becomes unavailable, the runtime MAY:

- retry;
- select another compatible target;
- recover;
- degrade where permitted;
- escalate;
- reject.

The source program MUST NOT be silently rewritten.

---

74. Routing Boundary

Routing is not grammar semantics.

The grammar expresses:

logical interaction requirements

Routing determines:

how those interactions are mapped to target resources

Therefore:

source
    ↓
quantum::ir
    ↓
routing

A quantum grammar MUST NOT encode a vendor-specific coupling map as universal language semantics.

---

75. Scheduling Boundary

Scheduling occurs after semantic lowering.

The language MAY express timing requirements or constraints where supported.

The grammar MUST NOT implement:

- scheduler algorithms;
- timing allocation;
- target queue management;
- calibration-aware scheduling.

---

76. Optimization Boundary

Optimization MUST preserve semantic equivalence.

The grammar does not decide:

- gate cancellation;
- gate fusion;
- decomposition;
- circuit rewriting;
- layout optimization;
- tensor optimization;
- target-specific transformations.

Those are compiler responsibilities.

---

77. Semantic Equivalence

A quantum optimization is valid only when the resulting representation preserves the relevant semantic contract.

The relevant contract MAY include:

- exact semantics;
- probabilistic semantics;
- observable distributions;
- measurement behavior;
- declared tolerances;
- resource constraints;
- contracts;
- effects;
- provenance requirements.

An optimization MUST NOT silently weaken a declared semantic guarantee.

---

78. Canonical Quantum IR

Every semantically valid quantum construct MUST have a defined relationship to "quantum::ir".

The required conceptual path is:

source
    ↓
frontend AST
    ↓
semantic quantum model
    ↓
quantum::ir

The exact Rust implementation is owned by the quantum IR subsystem.

---

79. Canonical IR Identity

Existing canonical IR identities MUST be reused.

The specification recognizes the repository's canonical distinction between:

QubitId
PhysicalQubitId
Gate
Measurement
ClassicalBitId
Circuit / Program structures

as applicable to the current IR.

New code MUST NOT introduce duplicate definitions merely because a new frontend feature needs the concept.

---

80. IR Operation Model

The IR operation representation MUST preserve all semantically relevant information.

At minimum, the mapping MUST account for concepts corresponding to:

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

Not every field must be physically stored in every IR node.

However, no semantically relevant information may be silently lost.

---

81. Source Provenance in IR

Quantum IR MUST retain or reference source provenance using the repository's established provenance mechanism.

At minimum, a compiler MUST be able to identify the source origin of a semantically meaningful operation sufficiently for diagnostics and auditing.

---

82. IR Verification

Before optimization or target lowering, the canonical quantum IR MUST be structurally and semantically verifiable.

Verification SHOULD cover:

- valid identities;
- valid operands;
- valid parameter relationships;
- valid result relationships;
- valid control dependencies;
- valid measurement dependencies;
- valid resource references;
- valid capability requirements;
- valid effect information;
- valid provenance references.

---

83. Classical IR Boundary

Quantum/classical hybrid programs MUST remain compatible with the repository's canonical classical semantic representation.

The architecture is:

Zamani program
       ↓
domain-neutral semantic model
       ↓
classical semantics + quantum semantics
       ↓
Classical IR + quantum::ir
       ↓
hybrid lowering

Neither domain may silently discard the other's semantic dependencies.

---

84. HDL Boundary

Quantum control hardware may interact with HDL/hardware constructs.

The semantic boundary MUST preserve:

quantum intent
classical control
hardware intent

without forcing the quantum grammar to become an HDL grammar.

Hardware realization belongs to:

HDL
hardware
HAL
backend

layers.

---

85. Accelerator Boundary

Quantum computation MAY interact with:

- GPU;
- FPGA;
- ASIC;
- tensor accelerator;
- specialized coprocessor;
- future accelerator.

The source language describes requirements and computation.

It MUST NOT encode one fixed accelerator architecture.

---

86. Distributed Quantum Computation

Quantum programs MAY participate in distributed computation.

The language MAY express:

- distributed quantum resources;
- communication requirements;
- remote operations;
- network capabilities;
- synchronization;
- distributed policies.

The distributed subsystem owns distributed semantics.

Quantum syntax MUST NOT duplicate actor/message/network semantics.

---

87. Quantum Networking

Quantum networking MAY involve:

- endpoints;
- channels;
- entanglement resources;
- communication;
- synchronization;
- security;
- policies.

The quantum grammar MAY express quantum-specific requirements.

Generic networking semantics remain owned by the networking subsystem.

---

88. Quantum Memory

Quantum memory is a semantic resource.

The language MAY express requirements for:

- persistence;
- coherence;
- storage duration;
- logical state retention;
- memory capability.

Physical storage implementation remains target-specific.

---

89. Topology

Topology is a resource/capability concern.

A source program MAY express a topology requirement when semantically meaningful.

For example:

requires topology(required_topology);

The grammar MUST NOT hard-code a universal coupling graph.

---

90. Timing

Quantum source MAY express semantic timing requirements where supported.

Examples include requirements concerning:

- latency;
- ordering;
- synchronization;
- duration;
- coherence constraints.

Physical timing realization remains downstream.

---

91. Pulse-Level Intent

Pulse-level programming MAY be supported as an explicit target-sensitive capability.

Pulse intent MUST remain distinct from ordinary portable gate/operation semantics.

It MUST NOT cause all quantum programs to become pulse programs.

---

92. Calibration

Calibration is not ordinary quantum-language semantics.

The grammar MUST NOT require:

- calibration database IDs;
- pulse calibration records;
- vendor calibration APIs;
- device calibration identifiers.

Calibration-aware compilation MAY consume target metadata later.

---

93. Hardware Identity

Portable quantum source MUST NOT require a vendor or physical machine identity.

A target-specific deployment configuration MAY specify:

target
provider
device
region
execution policy

but this configuration is not part of the universal quantum computation semantics.

---

94. Dialects

Quantum dialects are extension mechanisms.

A dialect MAY define:

- additional syntax;
- additional operation metadata;
- additional capabilities;
- additional attributes;
- target-specific semantics.

A dialect MUST:

- be explicitly identified;
- be versioned;
- declare compatibility;
- lower into the canonical semantic model;
- use "quantum::ir" where quantum semantics require it.

A dialect MUST NOT silently change stable Zamani semantics.

---

95. Dialect Isolation

A dialect MUST NOT introduce:

- a second lexer authority;
- a second universal AST;
- a second quantum IR;
- a second resource model;
- a second capability model;
- a second policy model.

Dialect-specific information MUST have an explicit ownership and lowering path.

---

96. Future Operations

A future quantum operation can be introduced without modifying the universal grammar when its invocation fits the open operation model.

This is a mandatory extensibility property.

The architecture MUST therefore support:

new operation
        ↓
operation metadata / library / dialect
        ↓
semantic validation
        ↓
quantum::ir
        ↓
lowering

without changing the core operation grammar merely because the operation name is new.

---

97. Interoperability

Zamani MAY interoperate with external quantum ecosystems.

Examples include:

- OpenQASM;
- QIR;
- Quil;
- Q# representations;
- vendor formats;
- simulator formats;
- hardware-facing representations.

Each format owns its own:

- lexical rules;
- parser;
- source AST;
- format validation;
- import/export logic.

The format MUST lower into Zamani semantics rather than replacing them.

---

98. External Format Import

Conceptually:

external quantum format
        ↓
format frontend
        ↓
Zamani semantic model
        ↓
quantum::ir

An importer MUST preserve semantics or report unsupported constructs.

It MUST NOT silently discard information.

---

99. External Format Export

Conceptually:

quantum::ir
        ↓
format-specific lowering
        ↓
external quantum format

Export MAY be lossy only when:

- the loss is explicitly detected;
- the user/toolchain is informed;
- the compatibility contract permits it.

Silent semantic loss is prohibited.

---

100. OpenQASM Boundary

OpenQASM is an interoperability format.

The repository's OpenQASM frontend is responsible for:

OpenQASM lexical analysis
        ↓
OpenQASM parsing
        ↓
OpenQASM validation
        ↓
import to Zamani quantum semantics

The OpenQASM standard library gate catalogue MUST NOT become Zamani's universal gate catalogue.

OpenQASM-specific syntax MUST remain in the interoperability subsystem.

---

101. QIR Boundary

QIR is an interoperability/lowering representation.

The direction is:

Zamani quantum semantics
        ↓
quantum::ir
        ↓
QIR lowering

Zamani grammar MUST NOT become a QIR grammar.

QIR-specific constraints MUST NOT redefine Zamani quantum semantics.

---

102. Serialization

Canonical quantum representations MAY be serialized.

Serialization MUST be deterministic where the repository requires deterministic artifacts.

Canonical serialization SHOULD preserve:

- semantic identity;
- operation ordering;
- parameters;
- operands;
- modifiers;
- attributes;
- effects;
- capabilities;
- provenance.

---

103. Hashing and Identity

Where canonical quantum IR hashing is used, hashing MUST operate on canonical semantic representation rather than arbitrary parser formatting.

Whitespace or irrelevant source formatting MUST NOT change semantic identity.

---

104. Source Formatting

Formatting is separate from semantics.

Two source programs that differ only in irrelevant formatting MUST remain semantically equivalent.

Source spans MUST remain available for diagnostics even after formatting or canonicalization.

---

105. Diagnostics

Quantum diagnostics MUST distinguish at least:

lexical error
syntax error
AST construction error
type error
name-resolution error
semantic quantum error
effect error
capability error
resource error
contract error
policy error
provenance error
compatibility error
target-feasibility error
runtime error

A target resource failure MUST NOT be reported as a syntax error.

---

106. Diagnostic Stability

Diagnostics SHOULD contain stable machine-readable identifiers.

Human-readable wording MAY evolve.

Tooling MUST NOT depend solely on diagnostic prose.

---

107. Error Recovery

Parser recovery MAY construct recovery AST nodes.

Recovery nodes MUST remain distinguishable from validated semantic constructs.

A compiler MUST NOT silently execute malformed source merely because parser recovery produced a tree.

---

108. Security

Quantum source is untrusted input unless explicitly trusted.

Production implementations MUST:

- use safe Rust;
- forbid "unsafe";
- avoid raw pointer manipulation;
- use checked arithmetic;
- avoid unchecked memory operations;
- avoid executing hardware operations during parsing;
- avoid implicit network access during parsing;
- avoid implicit filesystem access during parsing;
- avoid executing foreign code during parsing.

Recommended Rust module attributes are:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

where applicable.

---

109. Rust Baseline

Quantum-related production Rust MUST support:

Rust 1.97 or later
Rust 2021 edition
safe Rust

No quantum grammar feature may require "unsafe".

If a dependency requires unsafe internally, the Zamani source implementation MUST NOT itself introduce unsafe code merely to use the dependency unless the repository's global safety policy explicitly permits such dependency usage.

The preferred production boundary remains safe Rust throughout Zamani-owned code.

---

110. Parser Resource Protection

Implementations MAY impose protection policies against malicious or pathological input.

Examples include:

- maximum source bytes;
- parser work budget;
- recursion budget;
- AST memory budget;
- diagnostic budget;
- compilation time budget.

These are implementation policies.

They MUST be:

- explicit;
- configurable where appropriate;
- observable;
- distinguishable from language semantics.

They MUST NOT be represented as quantum language constants.

---

111. Iterative and Fallible Processing

Because quantum programs may be very large, implementations SHOULD use:

- checked arithmetic;
- fallible allocation;
- iterative traversal where recursion could overflow;
- streaming processing where appropriate;
- lazy structures;
- symbolic representations;
- bounded diagnostic accumulation.

The language MUST NOT require eager physical materialization of every semantic structure.

---

112. Symbolic Scaling

Symbolic forms SHOULD be preserved whenever concrete expansion is unnecessary.

Examples include:

qubit[n]
parameter(theta)
operation(parameter_expression)
resource >= symbolic_requirement

The compiler SHOULD defer expansion until a later stage actually requires it.

---

113. Large Program Handling

The implementation MUST be architecturally capable of processing programs whose size exceeds ordinary small examples.

Large programs MAY be limited by explicit compiler resource policies.

Such failure MUST be reported as:

implementation/resource limitation

rather than:

invalid Zamani quantum syntax

when the source is otherwise valid.

---

114. No Eager Exponential Expansion

A quantum program describing "n" qubits MUST NOT automatically be expanded into an exponential classical representation merely because the compiler can do so.

The compiler SHOULD preserve compact semantic forms until a concrete backend requires expansion.

---

115. Simulation

Simulation is an execution strategy.

The language MAY express simulation intent.

Simulation MAY target:

- state-vector simulation;
- stabilizer simulation;
- tensor networks;
- density matrices;
- trajectory methods;
- symbolic simulation;
- distributed simulation;
- accelerated simulation;
- future simulation techniques.

No single simulator representation is normative.

---

116. Simulation and Hardware Semantics

Simulation MUST distinguish:

simulated behavior

from:

physical target behavior

Where simulation models a physical noise/error model, the model MUST be explicit.

A simulator MUST NOT silently claim physical fidelity it cannot establish.

---

117. Verification

Quantum programs MAY be subjected to:

- type verification;
- contract verification;
- property verification;
- equivalence checking;
- resource verification;
- capability verification;
- formal proof;
- statistical validation.

Verification is downstream of parsing.

The grammar only needs to preserve the source intent required for verification.

---

118. Contracts and Probabilistic Semantics

A contract involving probabilistic quantum behavior MUST specify its semantic interpretation.

For example, a property MAY concern:

- probability bounds;
- expected values;
- distributions;
- exact state properties;
- measurement invariants.

A compiler MUST NOT reinterpret probabilistic contracts as deterministic guarantees.

---

119. Quantum Program Lifecycle

A quantum feature follows:

proposal
    ↓
semantic design
    ↓
specification
    ↓
grammar owner
    ↓
lexer contract
    ↓
AST contract
    ↓
semantic contract
    ↓
IR contract
    ↓
compiler contract
    ↓
runtime contract
    ↓
positive tests
    ↓
negative tests
    ↓
boundary tests
    ↓
scalability tests
    ↓
determinism tests
    ↓
compatibility tests
    ↓
stable

A ".g4" file existing is not sufficient for feature completion.

---

120. Feature Status

Quantum features MUST use the repository's established lifecycle states.

At minimum:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

Where applicable:

STABLE
EXPERIMENTAL
REMOVED
RESERVED

The status MUST reflect actual implementation state.

A specification MUST NOT claim implementation that does not exist.

---

121. Feature Manifest

Substantial quantum features SHOULD have machine-readable metadata identifying:

feature ID
feature name
status
language version
grammar owner
lexer dependencies
AST mapping
semantic owner
IR mapping
compiler consumers
runtime consumers
capabilities
resources
effects
policies
provenance
positive tests
negative tests
boundary tests
scalability tests
determinism tests
compatibility tests
hard-coding audit

This permits independent completion.

---

122. File Ownership Contract

Every quantum grammar file MUST document:

PURPOSE
OWNS
DOES_NOT_OWN
LEXER_DEPENDENCIES
GRAMMAR_DEPENDENCIES
EXPORTS
AST_CONTRACT
SEMANTIC_CONTRACT
TYPE_CONTRACT
EFFECT_CONTRACT
CAPABILITY_CONTRACT
RESOURCE_CONTRACT
CONTRACT_CONTRACT
POLICY_CONTRACT
PROVENANCE_CONTRACT
IR_CONTRACT
COMPILER_CONTRACT
RUNTIME_CONTRACT
CROSS_DOMAIN_CONTRACT
DIAGNOSTICS
POSITIVE_TESTS
NEGATIVE_TESTS
BOUNDARY_TESTS
SCALABILITY_TESTS
DETERMINISM_TESTS
COMPATIBILITY
HARD_CODING_AUDIT
COMPLETION_CRITERIA

No feature is considered independently complete until these are predetermined.

---

123. Dependency Contract

Every quantum feature file SHOULD declare:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
IR_OWNER:
SPEC_OWNER:
TEST_OWNER:

Example:

FILE:
    grammar/quantum/measurement.g4

DEPENDS_ON:
    canonical lexer
    grammar/expressions/*
    grammar/types/*
    grammar/quantum/qubits.g4

EXPORTS:
    quantumMeasurement

AST_OWNER:
    src/frontend/ast/

SEMANTIC_OWNER:
    quantum semantic layer

IR_OWNER:
    quantum::ir

SPEC_OWNER:
    grammar/spec/quantum.md

TEST_OWNER:
    grammar/tests/quantum/measurement/

The file MUST remain independently coherent when downstream implementations evolve.

---

124. One Rule, One Owner

Every grammar production MUST have one canonical owner.

A production MUST NOT be copied between:

grammar/quantum/
grammar/types/
grammar/expressions/
grammar/statements/

merely for convenience.

A public wrapper MAY delegate to an owned production.

Duplication is prohibited when it can produce divergent syntax.

---

125. Quantum Root Stability

"grammar/quantum/quantum.g4" SHOULD remain stable as quantum technology evolves.

Adding:

- an operation;
- a provider;
- a QPU;
- a simulator;
- a QEC implementation;
- a routing strategy

MUST NOT require root grammar changes when the feature already fits an existing extensibility point.

---

126. Lexer Stability

Adding a new operation MUST NOT normally require a new lexer keyword.

Adding a new quantum technology MUST NOT normally require a lexer change.

New reserved words are justified only when the construct cannot safely be represented using ordinary identifiers or existing lexical mechanisms.

---

127. AST Stability

Quantum AST construction MUST use the existing domain-neutral frontend AST architecture.

The quantum subsystem MUST NOT create a duplicate AST solely because a construct is quantum.

AST fields MUST represent source meaning, not backend choices.

---

128. AST Source Preservation

The AST MUST preserve enough information for:

- diagnostics;
- source mapping;
- IDE tooling;
- formatting;
- refactoring;
- compatibility;
- provenance;
- semantic analysis.

Source information MUST NOT be discarded merely because a backend does not need it.

---

129. AST Does Not Own Hardware

Portable AST structures MUST NOT contain backend-only fields such as:

vendor queue ID
hardware credential
calibration database key
backend session ID
device-specific internal pointer

unless the AST node explicitly represents a target/deployment configuration.

---

130. Semantic Analysis

Quantum semantic analysis MUST validate:

- operation resolution;
- operation modifiers;
- operand validity;
- parameter validity;
- type compatibility;
- qubit usage rules;
- measurement legality;
- reset legality;
- dynamic control legality;
- capability requirements;
- resource requirements;
- effects;
- contracts;
- policies;
- dialect compatibility;
- provenance requirements.

Semantic analysis is not parser syntax.

---

131. Quantum Resource Ownership and Usage

Where the quantum type system requires exclusive or constrained use of quantum resources, the semantic checker MUST enforce those rules.

This MAY integrate with:

- linear types;
- affine types;
- ownership;
- borrowing;
- lifetime analysis.

The quantum grammar MUST not duplicate the universal type/ownership model.

---

132. No Silent Quantum Duplication

A compiler MUST NOT implicitly copy a quantum resource when the semantic model prohibits copying.

Operations involving quantum resources MUST obey the applicable type/ownership rules.

---

133. Measurement and Ownership

Measurement MAY change the semantic state or ownership status of a quantum resource depending on the language semantics.

Those effects MUST be explicit in semantic analysis.

The grammar records syntax.

The semantic model defines the consequence.

---

134. Effects

Quantum constructs MUST integrate with the universal effect system.

Possible effects include:

quantum
measurement
randomness
simulation
learning
adaptation
network
distributed
foreign
native
io

The exact effect taxonomy is owned by "grammar/spec/effects.md" and its implementation.

Quantum grammar MUST NOT create a second effect system.

---

135. Capability Checking

Capability checking occurs after parsing.

Examples include:

quantum.measurement
quantum.dynamic_control
quantum.mid_circuit_measurement
quantum.logical_qubit
quantum.fault_tolerance
quantum.distributed

The list is extensible.

A capability identifier MUST NOT imply a particular vendor.

---

136. Resource Checking

Resource checking evaluates:

program requirements
+
compiler policies
+
target resources

It MUST NOT alter source semantics.

If the requirement cannot be satisfied, the compiler/runtime reports infeasibility.

---

137. Target Discovery

Target discovery is outside the grammar.

The grammar MUST NOT:

- query hardware;
- discover QPUs;
- inspect network state;
- select a backend;
- read calibration databases.

Those operations belong to compilation/runtime infrastructure.

---

138. Target Selection

Target selection is downstream.

The compiler MAY select a target according to:

- capabilities;
- resources;
- constraints;
- preferences;
- policies;
- provenance requirements;
- cost;
- performance;
- reliability;
- availability.

The language remains target-independent unless explicit target intent was requested.

---

139. Target Specialization

A compiler MAY specialize a program for a target.

Specialization MUST preserve the source semantics.

Specialization MAY select:

- gate decompositions;
- topology;
- routing;
- scheduling;
- QEC;
- numerical representation;
- simulation method;
- hardware implementation.

These decisions are not grammar semantics.

---

140. Portability Classes

Quantum programs MAY be classified according to portability.

Conceptually:

portable
capability-constrained
resource-constrained
dialect-constrained
target-specific
physical-intent

The classification MUST be explicit.

A target-specific program MUST NOT be falsely represented as universally portable.

---

141. Physical Intent and POCO-REAF

Explicit physical intent is permitted.

However:

portable source

and:

physical deployment specification

are distinct concerns.

A developer who intentionally requests a specific physical resource accepts narrower portability.

The core language remains capable of portable quantum computation.

---

142. Quantum Kernels

Quantum kernels MAY represent reusable quantum computation units.

A kernel MAY contain:

- parameters;
- quantum resources;
- operations;
- classical inputs;
- classical outputs;
- resource requirements;
- capabilities;
- effects;
- contracts.

Kernel syntax MUST reuse universal function/callable semantics where possible.

---

143. Kernel Compilation

A kernel MAY be compiled independently or as part of a larger program.

Its semantic meaning MUST remain stable.

The compiler MAY:

- inline;
- specialize;
- fuse;
- decompose;
- distribute;
- cache;
- lower.

Those transformations MUST preserve semantic equivalence.

---

144. Reproducibility

Quantum compilation SHOULD support reproducible results when the same:

- source;
- compiler version;
- language version;
- dialect versions;
- semantic configuration;
- target description;
- resource policy;
- optimization policy;
- randomness seed where applicable

are supplied.

Physical hardware outcomes may remain probabilistic.

---

145. Reproducible Compilation

The compiler SHOULD expose enough provenance to identify:

source version
language version
grammar version
AST version
semantic version
IR version
dialect versions
compiler version
target description
policy version
optimization configuration

where required by the repository's reproducibility model.

---

146. Randomized Compilation

If compiler transformations use randomness, the randomness MUST be represented by explicit compilation configuration or policy.

A compiler MUST NOT make supposedly deterministic compilation nondeterministic without recording the cause.

---

147. Quantum Measurement Reproducibility

Measurement results are not generally reproducible solely from source code.

A reproducible experiment requires relevant runtime inputs such as:

- target;
- calibration;
- noise model;
- seed where applicable;
- execution policy;
- shot count;
- environment.

The language specification MUST NOT falsely guarantee physical outcome reproducibility.

---

148. Provenance

Quantum provenance SHOULD identify semantic transformations such as:

source operation
    ↓
semantic operation
    ↓
optimization
    ↓
decomposition
    ↓
routing
    ↓
scheduling
    ↓
QEC
    ↓
target operation

The provenance chain MUST NOT imply that the target-specific result was source syntax.

---

149. Explainable Compilation

Compiler transformations MAY expose decisions such as:

operation decomposed because target lacks native operation
route selected because topology constraint
QEC selected because policy
fallback selected because capability unavailable

Such explanations are compiler metadata, not quantum syntax.

---

150. Compliance With Universal Policies

Quantum compilation MUST respect universal policies governing:

- security;
- resources;
- adaptation;
- execution;
- simulation;
- deployment;
- provenance.

A quantum backend MUST NOT bypass policy validation merely because it is quantum.

---

151. Sandbox Boundary

If quantum compilation or execution invokes:

- native code;
- foreign code;
- external processes;
- network services;
- plugins;

the appropriate effect, capability and sandbox policies MUST apply.

---

152. FFI

Quantum FFI MAY connect Zamani to:

- native quantum libraries;
- provider SDKs;
- simulator libraries;
- hardware interfaces.

FFI declarations MUST participate in:

effect analysis
capability checking
policy checking
provenance
ABI validation

FFI MUST NOT become an implicit escape from the semantic model.

---

153. ABI

ABI concerns remain in interoperability/backend layers.

The quantum grammar MUST NOT encode a universal ABI.

An ABI adapter MAY translate:

Zamani semantic quantum values
        ↓
ABI representation
        ↓
foreign implementation

---

154. Metaprogramming

Quantum code MAY participate in controlled metaprogramming.

Reflection/code generation MUST obey universal:

- effect;
- capability;
- policy;
- provenance;
- sandbox

requirements.

Metaprogramming MUST NOT silently bypass quantum semantic validation.

---

155. Generated Quantum Code

Generated quantum source or IR MUST pass the same required semantic validation as source-authored quantum code unless a formally specified trusted IR boundary applies.

Generated code MUST NOT receive implicit authority to violate resource or capability requirements.

---

156. Compile-Time Quantum Computation

Compile-time quantum computation MAY be supported through simulation or symbolic evaluation.

It MUST remain distinct from physical QPU execution.

The compiler MUST clearly distinguish:

compile-time simulation

from:

runtime quantum execution

---

157. Simulation Security

Simulation of large quantum systems can be computationally expensive.

Implementations MAY enforce simulation resource policies.

Such limits MUST NOT become quantum-language limits.

---

158. Conformance Tests

The quantum test suite MUST include at least:

quantum-minimal
quantum-types
quantum-qubits
quantum-registers
quantum-operations
quantum-custom-operations
quantum-qualified-operations
quantum-parameterized-operations
quantum-controls
quantum-adjoints
quantum-measurement
quantum-reset
quantum-states
quantum-observables
quantum-circuits
quantum-dynamic-control
quantum-mid-circuit-control
quantum-classical
quantum-channels
quantum-noise
quantum-kernels
quantum-resources
quantum-capabilities
quantum-policies
quantum-provenance
quantum-uncertainty
quantum-learning
quantum-inference
quantum-adaptation
quantum-error-correction
quantum-resilience
quantum-dialects
quantum-interoperability
quantum-openqasm
quantum-qir
quantum-scalability
quantum-determinism
quantum-security
quantum-negative
quantum-boundary

---

159. Positive Tests

Every feature MUST have valid examples proving:

- lexical acceptance;
- syntactic acceptance;
- AST construction;
- semantic validity;
- expected IR mapping where implemented.

---

160. Negative Tests

Every feature MUST have invalid examples proving that:

- malformed syntax is rejected;
- invalid types are rejected;
- invalid operands are rejected;
- invalid operations are rejected;
- unavailable required capabilities are diagnosed;
- unavailable required resources are diagnosed;
- invalid policies are diagnosed;
- invalid contracts are diagnosed.

---

161. Boundary Tests

Boundary tests MUST cover combinations such as:

classical + quantum
quantum + AI
quantum + distributed
quantum + networking
quantum + HDL
quantum + contracts
quantum + policies
quantum + provenance
quantum + simulation
quantum + adaptation
quantum + FFI

---

162. Scalability Tests

Tests MUST verify that no artificial quantum capacity is encoded.

The test suite SHOULD include progressively larger structures and symbolic structures.

Examples:

one qubit
small register
large symbolic register
large operation sequence
large parameter expression
deep semantic structure
dynamic circuit
large hybrid program

The exact test sizes are implementation benchmarks.

They MUST NOT become language limits.

---

163. Determinism Tests

The compiler/frontend MUST test that:

same source
+
same language version
+
same grammar
+
same semantic configuration
+
same deterministic policies

produces equivalent semantic results.

Parser nondeterminism is prohibited.

---

164. Compatibility Tests

Compatibility tests MUST cover:

- stable syntax;
- AST compatibility;
- semantic compatibility;
- IR compatibility;
- dialect compatibility;
- interoperability compatibility;
- deprecations;
- migrations.

---

165. Hard-Coding Audit

Quantum grammar and specification reviews MUST search for accidental constants or fixed technology assumptions.

At minimum audit for:

MAX_QUBITS
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS
MAX_REGISTER_SIZE
MAX_QPU_SIZE
MAX_CONTROLS
MAX_OPERATIONS
MAX_DEPTH
MAX_PARAMETERS
MAX_MEASUREMENTS

Also audit for hard-coded vendor/resource identity such as:

QPU0
QPU1
vendor-specific physical IDs
fixed coupling maps
fixed hardware topology

when used as universal language semantics.

---

166. Allowed Target-Specific Information

Target-specific information is allowed when explicitly represented as:

- deployment configuration;
- target description;
- dialect;
- backend metadata;
- capability description;
- resource policy;
- physical intent.

It MUST NOT leak into the portable quantum grammar.

---

167. No Vendor Lock-In

The universal quantum grammar MUST NOT depend on a particular provider.

Provider support MUST be implemented through:

capabilities
dialects
interop adapters
HAL
backend implementations
target metadata

as appropriate.

---

168. No Finite Gate Catalogue

The quantum grammar MUST NOT contain:

H | X | Y | Z | CNOT | SWAP | ...

as the universal operation universe.

A standard gate library MAY enumerate standard gates outside the universal grammar.

---

169. Standard Library Boundary

Standard quantum operations SHOULD be provided through a standard semantic/library layer.

The standard library MAY define:

- common gates;
- common measurements;
- common state preparations;
- common observables;
- common algorithms.

The language grammar remains open-world.

---

170. Library Boundary

Application-specific concepts SHOULD be implemented as libraries, dialects, capabilities or policies rather than universal quantum keywords.

Examples include:

- quantum chemistry libraries;
- optimization libraries;
- machine-learning libraries;
- scientific libraries;
- robotics libraries;
- networking libraries.

The core quantum grammar remains generic.

---

171. No Application Keyword Explosion

The quantum grammar MUST NOT become a catalogue of application concepts.

The following kinds of concepts should normally remain libraries or higher-level semantic systems rather than core quantum keywords:

specific industry workflows
specific applications
specific business actions
specific user roles
specific products
specific domain procedures

The quantum core expresses universal computational primitives.

---

172. Error Model Boundary

An error model MAY describe:

- bit-flip error;
- phase error;
- depolarization;
- correlated error;
- measurement error;
- custom noise.

The language representation describes intent.

The simulator or hardware model determines the actual implementation.

---

173. Fault Model Compatibility

A fault model MUST be associated with explicit provenance and configuration when reproducibility matters.

A backend MUST NOT silently substitute a materially different error model when the source program explicitly requires one.

---

174. Quantum Resource Contracts

A resource contract MAY combine:

requirements
constraints
preferences
capabilities
policies

Conceptually:

program
    ↓
resource contract
    ↓
target feasibility

Resource contracts do not allocate physical resources.

---

175. Capability Contracts

A capability contract specifies what the execution environment must be able to do.

It does not specify:

which machine

unless explicitly combined with target/deployment intent.

---

176. Policy Contracts

Policy contracts specify what is:

required
allowed
preferred
forbidden

The universal policy system owns their semantics.

---

177. Quantum Security

Quantum programs MAY require security capabilities.

Examples include:

secure_execution
trusted_device
authenticated_provider
isolated_execution
protected_measurement

These MUST be capabilities/policies, not hard-coded hardware assumptions.

---

178. Trusted Execution

Where a program requires trusted execution, the requirement MUST be explicit.

A compiler/runtime MUST NOT falsely claim that ordinary execution provides trusted execution.

---

179. Provenance and Security

Security-sensitive quantum operations SHOULD preserve provenance identifying:

- source;
- authorization;
- policy;
- target;
- transformation;
- execution decision.

---

180. Quantum Data

Quantum data crossing a classical boundary MUST have an explicit semantic representation.

The data layer MUST distinguish:

quantum state
measurement result
classical approximation
symbolic representation
serialized quantum representation

---

181. Serialization of Quantum State

Serialization of a quantum state is target/model dependent.

The language MUST NOT require a universal physical serialization.

A serializer MAY use:

- mathematical representation;
- symbolic representation;
- simulator representation;
- interoperability format.

---

182. Classical Approximation

If a quantum computation is approximated by a classical computation, the approximation MUST be explicit at the semantic/compiler level.

It MUST NOT silently change a quantum operation into an unrelated classical computation.

---

183. Approximate Compilation

Approximate lowering MAY be permitted where a contract or policy explicitly allows approximation.

The approximation MUST preserve the declared tolerance.

---

184. Precision

Quantum numerical parameters MAY require arbitrary or implementation-selected precision.

The grammar MUST NOT mandate a universal floating-point width.

---

185. Parameter Expressions

Quantum parameter expressions MUST use the universal expression system where possible.

The quantum grammar SHOULD add only genuinely quantum-specific parameter syntax.

---

186. Symbolic Parameters

Symbolic quantum parameters MUST remain symbolic until semantic resolution or lowering requires concretization.

The parser MUST NOT evaluate arbitrary symbolic mathematics.

---

187. Parameter Validation

Parameter validation occurs semantically.

Examples include:

angle domain
probability domain
dimension compatibility
observable compatibility
operation-specific constraints

The grammar only establishes structure.

---

188. Generic Quantum Algorithms

Quantum algorithms SHOULD be expressible through ordinary Zamani:

- functions;
- generics;
- modules;
- types;
- operations;
- control flow;
- contracts.

The language MUST NOT require a new keyword for every algorithm.

---

189. Quantum Modules

Quantum code MUST participate in the ordinary module system.

A quantum module MAY export:

- operations;
- types;
- kernels;
- constants;
- observables;
- algorithms;
- resource contracts.

The quantum subsystem MUST NOT invent a second module system.

---

190. Namespaces

Quantum names MAY be qualified.

Name resolution is owned by the universal name-resolution architecture.

The quantum grammar only preserves the structure required for resolution.

---

191. Imports

Quantum libraries SHOULD use ordinary Zamani module/import semantics.

External quantum formats MAY use format-specific import syntax within their own interoperability frontends.

---

192. Versioning

Quantum features MUST respect:

language version
grammar version
AST version
semantic version
IR version
dialect version
interop format version

Version information MUST NOT be silently reinterpreted.

---

193. Deprecation

A deprecated quantum feature MUST have:

- a stable identifier;
- migration guidance;
- compatibility status;
- replacement semantics where applicable;
- test coverage.

Removing a feature requires the repository's compatibility process.

---

194. Experimental Features

Experimental quantum features MUST be explicitly marked.

They MUST NOT silently become stable merely because they appear in a grammar file.

---

195. Reserved Features

Reserved quantum syntax MAY exist for future evolution.

Reserved syntax MUST be documented.

Reserved syntax MUST NOT acquire accidental semantics.

---

196. Grammar Evolution

The grammar SHOULD evolve by adding new leaf capabilities behind existing architectural boundaries.

The preferred change order is:

specification
    ↓
semantic contract
    ↓
AST contract
    ↓
IR contract
    ↓
leaf grammar
    ↓
root dispatch
    ↓
implementation
    ↓
tests

A root grammar change MUST be minimized.

---

197. Backward Compatibility

Existing stable quantum syntax MUST remain compatible unless the language version explicitly permits a breaking change.

A change is breaking when it changes:

- parse meaning;
- AST meaning;
- semantic meaning;
- IR meaning;
- observable program behavior.

Formatting changes alone are not semantic breaking changes.

---

198. Forward Compatibility

A compiler SHOULD reject unsupported future features explicitly.

It MUST NOT silently reinterpret unknown quantum constructs as something else.

---

199. Semantic Preservation

Every transformation MUST preserve all semantics required by the program's declared:

- types;
- effects;
- resources;
- capabilities;
- contracts;
- policies;
- provenance;
- quantum semantics.

---

200. Semantic Loss

If a downstream target cannot represent a source construct exactly, the compiler MUST choose one of:

exact lowering
supported approximation
supported fallback
explicit rejection

Silent semantic loss is prohibited.

---

201. Fallback

Fallback MAY use:

- another compatible operation;
- another target;
- simulation;
- host execution;
- alternative algorithm;
- alternative QEC strategy.

Fallback MUST be permitted by policy and preserve the declared semantic contract.

---

202. Degraded Execution

A program MAY allow degraded execution through explicit policy.

For example, a policy may allow a less capable target when a stated property remains satisfied.

The compiler/runtime MUST NOT assume degradation is acceptable unless the source or policy permits it.

---

203. Retry and Recovery

Quantum execution MAY retry or recover after runtime failure.

The retry/recovery policy belongs to the execution/resilience subsystem.

The source semantics remain unchanged.

---

204. Quantum Runtime Boundary

The runtime owns:

- execution sessions;
- target communication;
- job submission;
- result retrieval;
- runtime recovery;
- execution monitoring.

The grammar MUST NOT depend on runtime APIs.

---

205. HAL Boundary

HAL owns target abstraction.

The quantum specification requires only that HAL expose sufficient capabilities for semantic realization.

The grammar MUST NOT know the implementation details of HAL.

---

206. ZQN Boundary

ZQN is a downstream representation/protocol boundary.

Quantum grammar MUST NOT implement ZQN.

Quantum semantic information required by ZQN MUST first be represented through "quantum::ir" or the appropriate canonical semantic structures.

---

207. Compilation Pipeline

The normative conceptual pipeline is:

Zamani source
      ↓
canonical lexer
      ↓
ANTLR parser
      ↓
frontend AST
      ↓
structural validation
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
contract analysis
      ↓
policy analysis
      ↓
quantum semantic validation
      ↓
canonical semantic model
      ↓
quantum::ir
      ↓
IR verification
      ↓
optimization
      ↓
decomposition
      ↓
resource realization
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
target lowering
      ↓
runtime

No stage may silently bypass a required semantic validation stage.

---

208. Compiler Ordering

The exact compiler implementation MAY fuse passes for efficiency.

However, fused passes MUST preserve the logical dependency order.

For example, resource feasibility MUST NOT be treated as proven before required resource expressions are semantically resolved.

---

209. Backend Independence

A quantum backend MUST consume canonical semantic information.

It MUST NOT require the source grammar to know backend internals.

---

210. Future Hardware

A future quantum technology MUST be representable through the existing semantic abstractions where possible.

If genuinely new semantics are required, the extension MUST follow the feature lifecycle.

The language architecture MUST NOT assume that today's physical models exhaust future quantum computation.

---

211. Future Computational Models

The quantum subsystem MUST remain capable of integrating with future computational models such as:

- new qubit modalities;
- new quantum memories;
- new communication models;
- new accelerator architectures;
- new simulation methods;
- new logical-computation models.

The source language MUST evolve by semantic extension, not by freezing current hardware assumptions.

---

212. Quantum-Classical Resource Negotiation

A hybrid program may require both:

classical resources
quantum resources

The resource system MUST permit both requirements to be represented together.

Example conceptually:

requires memory >= required_memory;
requires qubits >= required_qubits;
requires capability("tensor.compute");
requires capability("quantum.measurement");

The compiler resolves the complete resource contract.

---

213. Cross-Domain Effects

A hybrid operation may carry multiple effects.

For example:

classical computation
+
quantum measurement
+
network transmission

The effect system MUST preserve the combined effect set.

---

214. Cross-Domain Provenance

A hybrid transformation SHOULD preserve provenance across:

classical
quantum
AI
data
HDL
distributed
networking

A quantum operation generated from classical/AI reasoning SHOULD retain enough provenance to identify its semantic origin.

---

215. Cross-Domain Policies

Policies MAY constrain hybrid computation.

Examples include:

- quantum execution only in trusted environments;
- no network access during a quantum region;
- simulation allowed as fallback;
- physical execution forbidden;
- adaptive execution requires authorization.

The universal policy system owns these semantics.

---

216. Cross-Domain Contracts

Contracts MAY span classical and quantum boundaries.

For example:

requires quantum capability
ensures classical result satisfies property

The semantic checker MUST preserve the dependency between the quantum computation and resulting classical property.

---

217. Quantum Compilation and AI

AI-based optimization MAY be used by the compiler.

The compiler MUST NOT expose such implementation strategy as quantum source semantics unless explicitly requested.

The quantum language describes computation.

The compiler decides whether optimization uses:

- deterministic algorithms;
- heuristic algorithms;
- learned models;
- symbolic methods;
- search.

---

218. Learning During Compilation

If a compiler learns during compilation, the learning process is an implementation concern unless explicitly surfaced by the language.

If surfaced, it MUST use the universal learning/effect/provenance/policy models.

---

219. Adaptation During Compilation

Compiler adaptation MUST remain distinct from program-level adaptation.

Compiler adaptation chooses a realization.

Program adaptation changes program-controlled behavior.

They MUST NOT be conflated.

---

220. Reflection

Quantum reflection MAY inspect:

- quantum type information;
- operation metadata;
- capability metadata;
- resource metadata;
- dialect metadata.

Reflection MUST respect universal reflection policies.

---

221. Compile-Time Introspection

Compile-time introspection MAY query semantic metadata.

It MUST NOT automatically query live hardware unless explicitly crossing into a target/discovery subsystem.

---

222. Runtime Introspection

Runtime introspection MAY report:

- target capabilities;
- resource availability;
- execution status;
- measurement results;
- resilience state.

Runtime introspection is not parser semantics.

---

223. Capability Discovery

Capability discovery occurs outside the grammar.

A capability name is stable semantic vocabulary.

A target's actual capability set is runtime/compiler data.

---

224. Resource Discovery

Resource discovery MAY discover:

- qubit availability;
- memory;
- topology;
- execution capacity;
- communication resources.

Discovery MUST NOT change the source language.

---

225. Quantum Topology Discovery

A backend MAY discover target topology.

The compiler uses topology for routing.

The source language remains independent unless it explicitly declares topology constraints.

---

226. Topology Constraints

A source topology constraint MUST be interpreted semantically.

It MUST NOT embed a particular backend's graph representation unless physical intent explicitly requests it.

---

227. Logical-to-Physical Mapping

The mapping:

logical qubit
        ↓
physical qubit

MUST be represented downstream.

Mapping MUST be traceable through provenance when required.

---

228. Mapping Stability

A compiler MAY choose different physical mappings on different targets.

The logical program remains unchanged.

Different mappings are valid when they preserve the program's semantic contract.

---

229. Quantum Scheduling and Timing

Scheduling MAY depend on:

- target topology;
- operation duration;
- coherence;
- concurrency;
- resource conflicts;
- calibration;
- policy.

The language remains independent of these implementation details.

---

230. Resource-Aware Optimization

Optimization MAY use resource constraints.

An optimization MUST NOT violate explicit source constraints merely to improve performance.

---

231. Capability-Aware Optimization

The compiler MAY choose different transformations based on capabilities.

For example:

target supports native operation

may avoid decomposition.

The source semantics remain identical.

---

232. Simulation Fallback

If policy permits, a quantum program MAY fall back to simulation.

The compiler/runtime MUST distinguish:

physical execution

from:

simulation

in provenance and diagnostics.

---

233. Hardware Fallback

If a target lacks a required capability, another target MAY be selected if:

- policy permits;
- resource requirements are satisfied;
- semantic requirements remain satisfied.

---

234. No Silent Target Substitution

Target substitution MUST NOT happen silently when the source explicitly requires a target identity.

For portable source, target selection remains a downstream decision.

---

235. Quantum Provider Abstraction

Provider integrations MUST implement provider-specific behavior outside the core grammar.

Provider-specific capabilities MAY be represented as capability identifiers.

---

236. Vendor Operation Abstraction

Vendor operations MAY be represented as open operation names.

Their semantics MUST be registered through an appropriate dialect/library/provider mechanism.

---

237. Vendor Operation Portability

A vendor-specific operation SHOULD be marked as target-specific when it cannot be lowered portably.

The compiler MUST not falsely classify it as universal.

---

238. Experimental Hardware

Experimental hardware MAY expose capabilities through dialects or target descriptions.

Experimental support MUST NOT destabilize stable universal syntax.

---

239. Quantum Algorithm Libraries

Algorithms such as:

- search;
- simulation;
- optimization;
- chemistry;
- cryptography;
- machine learning

SHOULD be libraries or higher-level semantic constructs.

The core grammar MUST remain small and universal.

---

240. Application Independence

The quantum language is not an application-specific DSL.

Application semantics belong in libraries and higher-level layers.

This keeps the quantum foundation usable across:

science
engineering
AI
cryptography
optimization
simulation
communication
control

without creating an unbounded keyword catalogue.

---

241. Canonical Quantum Grammar Principle

The quantum grammar is a semantic front door.

It is not:

hardware description
pulse language
vendor SDK
runtime API
resource manager
QEC implementation
scheduler
router
simulator

Those are downstream systems.

---

242. Parser Purity

The parser MUST be free of:

- hardware probing;
- network calls;
- filesystem discovery;
- target selection;
- runtime execution;
- QPU submission;
- calibration lookup;
- QEC decoding.

Parser actions MUST be structural.

---

243. Deterministic Parsing

Given identical source and lexical configuration, the parser MUST produce the same parse result.

Parsing MUST NOT depend on:

- hardware state;
- network state;
- current time;
- random numbers;
- provider availability.

---

244. Grammar Actions

Semantic actions embedded in ANTLR grammar MUST be avoided where they create architecture coupling.

The preferred pipeline is:

parse
    ↓
AST
    ↓
semantic analysis

not:

parse
    ↓
execute semantic behavior

---

245. Rust Integration

Generated ANTLR Rust code and Zamani-owned Rust integration MUST remain compatible with Rust 1.97+.

Quantum grammar files MUST NOT require unsafe runtime support.

---

246. Safe Resource Accounting

Resource accounting implementations SHOULD use:

- checked arithmetic;
- explicit overflow handling;
- fallible allocation;
- bounded diagnostics;
- symbolic representations.

Overflow MUST produce a controlled diagnostic rather than undefined behavior.

---

247. Deep Structures

Implementations MUST account for deeply nested quantum expressions and large programs.

Where recursion can exhaust the stack, iterative algorithms SHOULD be used.

---

248. Canonical Source Spans

Every quantum syntax node that can generate diagnostics SHOULD preserve source span information.

The span MUST identify the relevant source region sufficiently for tooling.

---

249. Comments and Formatting

Comments and formatting are source concerns.

If the frontend preserves trivia, quantum constructs SHOULD retain sufficient information for source tooling.

Semantic meaning MUST NOT depend on comments.

---

250. Documentation

Every stable quantum feature MUST document:

- purpose;
- syntax;
- semantics;
- types;
- effects;
- capabilities;
- resources;
- contracts;
- policies;
- provenance;
- AST;
- IR;
- diagnostics;
- compatibility;
- tests.

---

251. Integration With "grammar/quantum/README.md"

The quantum directory README is architectural navigation/documentation.

It MUST remain consistent with this specification.

If the README and this specification disagree:

normative specification

wins until the specification is formally changed.

---

252. Integration With "grammar/spec/"

Quantum specification MUST align with:

grammar/spec/resources.md
grammar/spec/effects.md
grammar/spec/provenance.md
grammar/spec/policies.md
grammar/spec/ai.md
grammar/spec/classical.md
grammar/spec/hdl.md
grammar/spec/interoperability.md
grammar/spec/syntax.md

Quantum-specific rules belong here.

Universal rules belong in the universal specification.

---

253. Integration With "grammar/compatibility/"

Quantum syntax MUST satisfy:

frontend-conformance
AST-conformance
compatibility-matrix

No quantum feature is stable until those compatibility contracts are satisfied.

---

254. Integration With "src/frontend/ast/"

Quantum grammar structures MUST map into the existing frontend AST architecture.

The quantum specification does not authorize creation of a parallel AST hierarchy merely for convenience.

---

255. Integration With "src/quantum/ir/"

All canonical quantum semantic information MUST eventually map to:

crate::quantum::ir

The specification recognizes this as the canonical downstream quantum boundary.

---

256. Integration With OpenQASM

The existing OpenQASM frontend remains an interoperability boundary.

Its structure is conceptually:

OpenQASM
    ↓
OpenQASM frontend AST
    ↓
OpenQASM semantic validation
    ↓
Zamani quantum semantics
    ↓
quantum::ir

OpenQASM source syntax MUST NOT be inserted into Zamani's universal grammar.

---

257. Integration With QIR

QIR support remains an interoperability/lowering concern.

The direction is:

Zamani quantum semantics
    ↓
quantum::ir
    ↓
QIR

QIR MUST NOT become the source-language authority.

---

258. Integration With Classical Compilation

Quantum operations that produce classical values MUST connect to the classical semantic model.

The compiler MUST preserve data dependencies across the boundary.

---

259. Integration With AI

AI-related quantum features MUST reuse universal:

learning
reasoning
inference
adaptation
uncertainty
evidence
provenance
policy

models.

The quantum subsystem MUST NOT duplicate them.

---

260. Integration With Data

Quantum data structures MUST integrate with the universal data model where meaningful.

Serialization and interchange remain data/interoperability concerns.

---

261. Integration With Networking

Quantum networking features MUST integrate with the networking subsystem.

Network effects MUST be represented through the universal effect model.

---

262. Integration With Distributed Execution

Distributed quantum computation MUST integrate with:

concurrency
actors
distributed
networking
resources
policies
resilience

There MUST NOT be a second actor model created solely for quantum agents.

---

263. Integration With HDL

Hardware interaction MUST use the HDL/hardware/HAL architecture.

Quantum grammar does not become a hardware netlist grammar.

---

264. Integration With Security

Security-sensitive quantum execution MUST integrate with:

security
capabilities
authorization
sandbox
policy
provenance
audit

---

265. Integration With Metaprogramming

Generated quantum constructs MUST ultimately pass through the same semantic contracts as source-authored constructs.

Metaprogramming MUST NOT become a semantic bypass.

---

266. Integration With Compatibility

A quantum feature MUST specify:

introduced version
stable version
deprecated version
removed version
migration path

where applicable.

---

267. Integration With Tooling

Tooling SHOULD be able to inspect:

- quantum operations;
- types;
- measurements;
- resource requirements;
- capabilities;
- contracts;
- policies;
- provenance;
- IR mapping.

This information SHOULD be exposed through machine-readable structures rather than parsing documentation.

---

268. IDE Requirements

An IDE MAY provide:

- quantum operation completion;
- operation documentation;
- capability diagnostics;
- resource diagnostics;
- measurement-flow analysis;
- visual circuit representations;
- provenance views.

IDE features MUST consume semantic information rather than becoming alternate language authorities.

---

269. Formatter Requirements

A formatter MUST preserve semantics.

It MUST NOT rewrite quantum operations into target-specific forms.

---

270. Linter Requirements

Linters MAY detect:

- unused quantum resources;
- invalid patterns;
- suspicious measurements;
- impossible requirements;
- portability issues;
- target-specific constructs.

Linting MUST be distinguishable from language validity.

---

271. Compiler Diagnostics for Portability

A compiler SHOULD be able to warn when a program is:

portable
capability-specific
resource-specific
dialect-specific
target-specific
physical-specific

This helps developers understand POCO-REAF portability.

---

272. Portability Is Semantic

Portability does not mean that every target supports every operation.

It means:

source meaning remains stable

while target realization may differ.

---

273. Physical Feasibility

Physical feasibility is a separate question from language validity.

A source program may be:

valid Zamani

but:

not currently executable on target X

That is not necessarily a language error.

---

274. Future Target Feasibility

A program MAY become executable on a future target without source modification.

This is a central POCO-REAF property.

---

275. Target Capability Growth

When a target gains a capability, previously infeasible source MAY become executable.

No language redesign should be necessary merely because hardware improved.

---

276. Resource Growth

When resources increase, a symbolic or scalable program SHOULD be capable of using the larger resource set where its semantics permit it.

The grammar MUST NOT artificially prevent this.

---

277. Resource Shrinkage

When resources decrease, a program may become infeasible.

The compiler/runtime MAY:

- reject;
- fallback;
- specialize;
- simulate;
- degrade if permitted.

The source semantics remain unchanged.

---

278. Quantum Program Scaling

Scaling MAY occur along:

qubit count
operation count
circuit depth
parameter count
classical workload
distributed nodes
quantum nodes
simulation size
resource availability

None may be globally capped by grammar design.

---

279. Algorithmic Scaling

The language MUST not require one implementation strategy for scaling.

A compiler MAY use:

- decomposition;
- parallelization;
- distribution;
- batching;
- tiling;
- lazy evaluation;
- symbolic execution;
- target-specific optimization.

---

280. Compilation Scaling

The compiler SHOULD support incremental and/or staged processing where appropriate.

The specification does not require one compiler architecture.

It requires semantic preservation.

---

281. Runtime Scaling

The runtime MAY scale quantum execution across:

- one target;
- multiple targets;
- distributed targets;
- simulators;
- accelerators.

The language remains independent of the runtime deployment strategy.

---

282. Cloud Execution

Cloud execution is a deployment/runtime concern.

A quantum program MUST NOT require cloud-specific grammar.

---

283. Embedded Execution

Embedded execution is a target concern.

The language MUST remain usable for small systems without introducing separate embedded quantum syntax.

---

284. HPC Execution

HPC execution is a target/compile concern.

The language MAY express resource requirements but MUST NOT encode a fixed HPC architecture.

---

285. Distributed Quantum Execution

Distributed execution MAY use:

- actor systems;
- message passing;
- network channels;
- collective operations.

Those systems remain separate semantic owners.

---

286. Quantum Communication

Quantum communication primitives MAY be expressed when supported.

They MUST integrate with:

- networking;
- capabilities;
- resources;
- security;
- policies;
- provenance.

---

287. Entanglement Resources

Entanglement MAY be represented as a semantic resource.

Its physical generation and maintenance are target/runtime concerns.

---

288. Quantum Memory Resources

Quantum memory requirements MAY express:

- capacity;
- retention;
- coherence;
- logical persistence;
- communication compatibility.

Physical memory implementation remains downstream.

---

289. Quantum Time

Time-related requirements MAY be represented semantically.

The grammar MUST NOT hard-code:

- fixed clock rates;
- fixed gate durations;
- fixed coherence times.

Those are target properties.

---

290. Fidelity

Fidelity requirements MAY be expressed as resource/capability constraints.

The source MUST distinguish:

required property

from:

measured target property

---

291. Error Rates

Error-rate requirements MAY be expressed through the resource/fault model.

The grammar MUST NOT assume one universal physical error rate.

---

292. Noise-Aware Compilation

Noise-aware compilation is downstream.

A compiler MAY use target noise information to choose a better realization.

The source semantics remain unchanged.

---

293. Fault-Aware Compilation

Fault-aware compilation MAY use:

- fault model;
- resilience policy;
- QEC capabilities;
- target availability.

It MUST preserve source semantics.

---

294. Quantum Benchmarking

Benchmarking is not core quantum syntax.

Benchmarking protocols belong to libraries/tooling/benchmarking subsystems.

The quantum language may express computations used by benchmarks.

---

295. Profiling

Profiling is tooling/runtime functionality.

The grammar MUST NOT embed profiler implementation details.

---

296. Observability

Runtime observability MAY expose:

- operation counts;
- timing;
- resource use;
- errors;
- measurements;
- resilience state.

Observability metadata SHOULD remain distinguishable from program semantics.

---

297. Logging

Quantum logging MAY integrate with the universal logging/observability infrastructure.

The grammar MUST NOT create a quantum-specific logging language unless genuinely required.

---

298. Auditability

Quantum execution SHOULD be auditable when policy requires it.

Audit records SHOULD include:

- source identity;
- compilation identity;
- target identity;
- policy identity;
- transformation provenance;
- execution result.

---

299. Scientific Reproducibility

Scientific quantum computation SHOULD preserve sufficient provenance to reproduce:

- source;
- compiler;
- semantic configuration;
- target description;
- noise model;
- execution policy;
- results.

---

300. Reproducibility Does Not Mean Identical Hardware

Reproducibility MAY mean reproducibility of semantic methodology rather than identical physical outcomes.

Quantum randomness and physical drift remain explicit factors.

---

301. Canonical Mathematical Meaning

Where a quantum construct represents a mathematical operation, the language semantics SHOULD be stated independently of its implementation.

Example:

operation X

means the mathematical/semantic operation defined by its library or semantic registry.

Its hardware implementation is separate.

---

302. Semantic Registries

A semantic registry MAY provide definitions for:

- operations;
- observables;
- channels;
- state constructors;
- capabilities;
- dialect features.

Registries MUST NOT require parser modification for ordinary additions.

---

303. Registry Versioning

Semantic registries MUST be versioned when their definitions affect program meaning.

A program relying on registry semantics SHOULD record the registry version in provenance.

---

304. Operation Overloading

Operation overloading MAY exist through ordinary Zamani semantic mechanisms.

Overload resolution MUST be deterministic.

---

305. Generic Operations

Quantum operations MAY be generic.

Generic resolution remains owned by the universal type system.

---

306. Operation Contracts

Quantum operations MAY declare:

requires
ensures
invariant

These contracts participate in universal validation.

---

307. Operation Effects

Operations MUST expose relevant effects.

Measurement, randomness, networking, foreign execution and adaptation MUST NOT be treated as effect-free when they are semantically effectful.

---

308. Operation Capabilities

Operations MAY require capabilities.

For example:

quantum::measurement
quantum::dynamic_control
quantum::reset

Capability requirements are semantic metadata.

---

309. Operation Resource Requirements

An operation MAY require resources.

For example:

minimum qubit resources
connectivity
memory
latency
fidelity

The exact resource expression is owned by the universal resource system.

---

310. Operation Policies

Operations MAY be affected by policies.

For example:

operation permitted only under policy X

The policy subsystem owns enforcement.

---

311. Operation Provenance

Every semantically meaningful generated operation SHOULD retain provenance identifying its origin.

This is especially important for:

- optimization;
- generated code;
- decomposition;
- routing;
- QEC.

---

312. Decomposition

A high-level operation MAY be decomposed into lower-level operations.

Decomposition MUST preserve declared semantics.

The decomposition strategy is target/compiler specific.

---

313. Native Operation Selection

A compiler MAY choose a native operation when the target supports it.

The source program remains unchanged.

---

314. Gate Synthesis

Gate synthesis is a compiler/backend responsibility.

The grammar MUST NOT encode one universal synthesis algorithm.

---

315. Approximate Synthesis

Approximate synthesis MAY be used only where the declared semantic tolerance permits it.

---

316. Circuit Transformation Provenance

Every significant circuit transformation SHOULD be traceable when provenance is enabled.

---

317. Measurement Transformation

Measurement transformations require special care because measurement may change semantics.

A compiler MUST NOT move, duplicate or remove measurement without proving semantic equivalence under the relevant model.

---

318. Dynamic Control Transformation

Dynamic control MAY be transformed into static control only when the transformation preserves semantics.

---

319. Resource Transformation

A compiler MAY transform resource usage while preserving requirements.

For example, one target may use:

more physical resources

to realize:

same logical computation

This is valid if the source requirement is satisfied.

---

320. Logical Resource Expansion

Logical-to-physical expansion MUST remain downstream.

A source program MUST NOT be forced to know the physical overhead of its logical representation.

---

321. Error-Correction Expansion

QEC expansion MAY dramatically increase resource usage.

The source program's logical requirements remain distinct from physical implementation overhead.

---

322. QEC Policy

A program MAY require:

fault tolerance
logical qubits
minimum reliability

The actual code and physical realization are selected downstream.

---

323. Resilience Policy

A resilience policy MAY define acceptable behavior under:

- degraded target;
- transient failure;
- unavailable resource;
- target failure;
- execution timeout.

---

324. Quarantine

A target may be quarantined by runtime policy.

The source program remains valid.

The runtime selects another realization only if permitted.

---

325. Retired Targets

A retired target MUST NOT be selected for new execution unless an explicit compatibility/recovery policy permits it.

---

326. Unknown Target State

An unknown target state MUST NOT be assumed healthy.

The runtime/resilience layer owns the resulting decision.

---

327. Quantum Safety

Quantum language implementation safety includes:

- memory safety;
- arithmetic safety;
- resource safety;
- effect safety;
- capability safety;
- policy safety;
- semantic safety.

Safe Rust is necessary but not sufficient for semantic safety.

---

328. No Unsafe Requirement

No quantum feature may require:

unsafe

in Zamani's own production implementation.

---

329. Dependency Safety

Third-party dependencies MUST be reviewed according to repository dependency policy.

A quantum feature MUST NOT bypass repository safety rules merely because a backend library requires low-level access.

---

330. Testing Rust Safety

CI SHOULD verify that quantum-related Rust crates/modules continue to satisfy the repository's unsafe-code prohibition.

---

331. Property-Based Testing

Quantum semantic components SHOULD use property-based tests where appropriate for:

- operation structures;
- parameter expressions;
- resource contracts;
- serialization;
- round trips;
- canonicalization.

---

332. Fuzz Testing

Quantum parser boundaries SHOULD be fuzz tested for:

- malformed syntax;
- deeply nested syntax;
- very large inputs;
- unusual identifiers;
- malformed parameter structures;
- malformed operation modifiers.

Fuzzing MUST verify that invalid input cannot cause unsafe behavior or uncontrolled resource consumption beyond configured policies.

---

333. Round-Trip Testing

Where serialization exists:

source/IR
    ↓
serialize
    ↓
deserialize

SHOULD preserve semantic structure.

---

334. Canonicalization Testing

Equivalent semantic programs SHOULD canonicalize consistently when canonicalization is defined.

---

335. Interoperability Round Trips

Where lossless:

external format
    ↓
Zamani semantics
    ↓
quantum::ir
    ↓
external format

SHOULD preserve semantics.

Lossy conversions MUST report their limitations.

---

336. Quantum Negative Compatibility

A program rejected because of unavailable hardware resources MUST remain distinguishable from a program rejected because its source syntax is invalid.

---

337. Compile Once

"Compile Once" in POCO-REAF refers to preservation of canonical semantic intent.

It does not require one immutable target-specific binary.

A semantic artifact MAY be lowered differently for different targets.

---

338. Run Everywhere

"Run Everywhere" means:

any target capable of satisfying the program contract

not:

every target regardless of capability

---

339. Anywhere

"Anywhere" means that the source semantics do not depend on one deployment location.

Deployment-specific requirements MAY narrow applicability.

---

340. Forever

"Forever" means the language architecture is designed for long-term semantic stability and extensibility.

It does not promise that:

- hardware APIs never change;
- external formats never change;
- compiler bugs never occur;
- physical technology never changes.

Compatibility and versioning systems exist to manage those changes.

---

341. Semantic Stability

Stable quantum constructs MUST preserve their semantic interpretation across compatible language versions.

---

342. Extension Stability

New operations and hardware technologies SHOULD be added through existing open-world extension points.

---

343. No Architecture Freeze

Production readiness does not mean freezing every implementation detail.

It means freezing the correct semantic contracts while allowing implementations to evolve.

---

344. Production Readiness

"grammar/spec/quantum.md" is specification-complete when it defines:

syntax authority
semantic authority
AST boundary
IR boundary
operation model
type model
state model
measurement model
dynamic execution
resource model
capability model
effect model
contract model
policy model
provenance model
QEC boundary
resilience boundary
routing boundary
scheduling boundary
simulation boundary
HAL boundary
runtime boundary
interoperability
dialects
versioning
compatibility
security
Rust safety
testing
scalability
hard-coding rules

---

345. Production Readiness of Implementation

The repository itself is quantum-production-ready only when every applicable feature satisfies:

specification
    ✓

lexer
    ✓

grammar
    ✓

AST
    ✓

semantic analysis
    ✓

type checking
    ✓

effect checking
    ✓

capability checking
    ✓

resource checking
    ✓

contract checking
    ✓

policy checking
    ✓

provenance
    ✓

quantum::ir mapping
    ✓

IR verification
    ✓

optimization integration
    ✓

routing integration
    ✓

scheduling integration
    ✓

QEC integration
    ✓

resilience integration
    ✓

ZQN integration
    ✓

HAL integration
    ✓

runtime integration
    ✓

positive tests
    ✓

negative tests
    ✓

boundary tests
    ✓

scalability tests
    ✓

determinism tests
    ✓

compatibility tests
    ✓

security tests
    ✓

---

346. Definition of Done for This File

This specification is complete when every future quantum grammar feature can answer, before implementation:

What syntax does it own?

What does it not own?

Which tokens does it consume?

Which grammar owns its productions?

Which AST node represents it?

Which semantic model represents it?

Which types does it use?

Which effects does it produce?

Which capabilities does it require?

Which resources does it require?

Which contracts constrain it?

Which policies constrain it?

Which provenance must be preserved?

How does it map to quantum::ir?

Which compiler passes consume it?

Which runtime components consume it?

Which targets may realize it?

What diagnostics are required?

What positive tests prove it?

What negative tests prove it?

What boundary tests prove it?

What scalability tests prove it?

What determinism tests prove it?

What compatibility guarantees exist?

What hard-coding audit proves scalability?

What safety audit proves safe Rust?

If these questions cannot be answered, the feature is not production-ready.

---

347. Canonical Ownership Summary

The intended ownership is:

grammar/spec/quantum.md
    normative quantum semantics

grammar/quantum/quantum.g4
    quantum grammar orchestration

grammar/quantum/operations.g4
    operation invocation syntax

grammar/quantum/types.g4
    quantum-specific type syntax

grammar/quantum/qubits.g4
    qubit syntax

grammar/quantum/quantum-registers.g4
    register syntax

grammar/quantum/states.g4
    state syntax

grammar/quantum/measurement.g4
    measurement syntax

grammar/quantum/reset.g4
    reset syntax

grammar/quantum/controls.g4
    control syntax

grammar/quantum/adjoints.g4
    adjoint/inverse syntax

grammar/quantum/parameters.g4
    quantum parameter syntax

grammar/quantum/observables.g4
    observable syntax

grammar/quantum/channels.g4
    channel syntax

grammar/quantum/noise.g4
    noise syntax

grammar/quantum/dynamic-circuits.g4
    dynamic circuit syntax

grammar/quantum/mid-circuit-control.g4
    measurement-dependent control

grammar/quantum/quantum-classical.g4
    explicit quantum/classical boundaries

grammar/quantum/error-correction.g4
    QEC intent

grammar/quantum/quantum-resources.g4
    quantum resource requirements

grammar/quantum/quantum-capabilities.g4
    quantum capability requirements

grammar/quantum/quantum-dialects.g4
    dialect declarations/extensions

grammar/quantum/provenance.g4
    quantum provenance syntax

grammar/quantum/kernels.g4
    quantum kernel syntax

grammar/spec/resources.md
    universal resources

grammar/spec/effects.md
    universal effects

grammar/spec/provenance.md
    universal provenance

grammar/spec/policies.md
    universal policies

grammar/spec/ai.md
    universal AI/reasoning/learning semantics

src/frontend/ast/
    canonical frontend AST

src/quantum/ir/
    canonical quantum semantic IR

src/quantum/frontend/formats/
    external quantum formats

compiler
    optimization/lowering/routing/scheduling

QEC/resilience
    fault and recovery realization

ZQN
    downstream quantum execution representation

HAL
    target abstraction

runtime
    actual execution

---

348. Canonical Dependency Direction

The dependency direction MUST remain:

specification
      ↓
grammar
      ↓
frontend AST
      ↓
semantic model
      ↓
quantum::ir
      ↓
compiler
      ↓
ZQN
      ↓
HAL
      ↓
runtime
      ↓
target

The reverse dependency is prohibited.

In particular:

grammar
    MUST NOT depend on runtime

grammar
    MUST NOT depend on hardware

grammar
    MUST NOT depend on routing implementation

grammar
    MUST NOT depend on scheduling implementation

grammar
    MUST NOT depend on QEC implementation

grammar
    MUST NOT depend on calibration

grammar
    MUST NOT depend on provider APIs

grammar
    MUST NOT depend on target discovery

---

349. Final Quantum Architecture

The canonical architecture is:

                         ZAMANI
                            │
             ┌──────────────┼──────────────┐
             │              │              │
         classical       quantum          HDL
             │              │              │
             └──────────────┼──────────────┘
                            │
                         hybrid
                            │
                  domain-neutral AST
                            │
                    semantic analysis
                            │
       ┌────────────────────┼────────────────────┐
       │                    │                    │
     types                effects             policies
       │                    │                    │
       └────────────────────┼────────────────────┘
                            │
                    quantum semantics
                            │
                       quantum::ir
                            │
              ┌─────────────┼─────────────┐
              │             │             │
         optimization    decomposition  analysis
              │             │             │
              └─────────────┼─────────────┘
                            │
                         routing
                            │
                       scheduling
                            │
                    QEC / resilience
                            │
                           ZQN
                            │
                           HAL
                            │
                 target-specific lowering
                            │
                         runtime
                            │
        ┌───────────┬────────┼────────┬───────────┐
        │           │        │        │           │
       CPU         GPU      FPGA     ASIC        QPU
        │           │        │        │           │
        └───────────┴────────┼────────┴───────────┘
                            │
                 simulator / HPC / cluster
                            │
                    distributed / cloud
                            │
                     future substrates

---

350. Final Normative Principle

The governing principle of Zamani quantum computation is:

«The programmer specifies portable computational intent; the compiler determines an appropriate realization; the target determines what is physically available; the runtime determines how execution occurs.»

Therefore:

PROGRAMMER
    specifies
        WHAT

COMPILER
    determines
        HOW

RESOURCE/CAPABILITY SYSTEM
    determines
        WHAT IS FEASIBLE

TARGET
    determines
        WHAT IS AVAILABLE

RUNTIME
    determines
        HOW IT EXECUTES

The language MUST NOT reverse these responsibilities.

---

351. Final POCO-REAF Rule

A Zamani quantum program SHOULD remain unchanged when moving between compatible computational substrates.

The same source semantics may therefore be lowered differently for:

tiny target
→
CPU
→
multicore
→
GPU
→
FPGA
→
ASIC
→
accelerator
→
simulator
→
QPU
→
HPC
→
cluster
→
distributed system
→
cloud
→
future computational substrate

subject to:

resources
capabilities
constraints
preferences
effects
policies
contracts
compatibility
physical feasibility

No artificial machine-size constant belongs in the language.

---

352. Final Scalability Rule

The quantum language MUST be able to express a computation of any finite size that can be represented by the language and whose execution is permitted by available resources.

Therefore:

small program
        |
        v
large program
        |
        v
symbolically scalable program
        |
        v
target-dependent realization

The language does not define the upper bound.

Available resources and physical feasibility do.

---

353. Final Extensibility Rule

A future quantum feature SHOULD be implemented by adding:

new semantic capability
        ↓
new leaf grammar if syntax is required
        ↓
AST mapping
        ↓
semantic mapping
        ↓
quantum::ir mapping
        ↓
compiler/runtime support
        ↓
tests

It SHOULD NOT require redesigning:

the universal lexer
the universal operation grammar
the quantum root
the AST architecture
the canonical quantum IR

unless the new feature genuinely introduces new language semantics.

---

354. Final Safety Rule

All Zamani quantum implementation code MUST remain safe Rust.

The baseline is:

Rust 1.97+
Rust 2021
unsafe forbidden
checked arithmetic
explicit resource policies
controlled allocation
deterministic parsing
controlled diagnostics

No quantum-language feature is permitted to bypass these requirements.

---

355. Final Canonical Rule

The single most important rule is:

«If a quantum feature can be expressed as portable semantic intent, it MUST remain portable semantic intent until a downstream compiler stage has sufficient information to choose a concrete realization.»

Consequently:

source syntax
    describes intent

semantic analysis
    determines meaning

quantum::ir
    preserves canonical quantum meaning

compiler
    determines realization

target
    provides capabilities and resources

runtime
    executes the realization

This is the contract that allows Zamani quantum computation to scale from the smallest meaningful computation to arbitrarily large computations whenever the available resources and target capabilities permit it, without freezing the language around today's hardware.