Zamani Quantum Language Specification

Path: "grammar/spec/quantum.md"
Language: Zamani
Specification role: Normative quantum-domain syntax, semantics, portability, scalability, AST, IR, compiler, runtime, interoperability, diagnostics, validation, and compatibility contract
Rust baseline: Rust 1.97 / Rust 1.97.1, Edition 2021
Rust safety requirement: "unsafe" Rust is prohibited
Execution principle: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability principle: No artificial language-level machine-size ceiling

---

1. Status

This document is the normative specification for the quantum domain of the Zamani programming language.

It defines what quantum constructs mean at the language level and establishes the contracts connecting:

Zamani source
    ↓
canonical lexical specification
    ↓
parser / Zamani.g4
    ↓
frontend AST
    ↓
structural validation
    ↓
semantic analysis
    ↓
canonical semantic IR
    ↓
quantum::ir
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
resilience
    ↓
HAL / target capabilities
    ↓
target lowering
    ↓
runtime
    ↓
actual computational substrate

This specification is authoritative for quantum-language meaning.

It is not an implementation specification for:

- quantum simulators;
- physical QPUs;
- hardware drivers;
- pulse-control systems;
- routing algorithms;
- scheduling algorithms;
- QEC decoders;
- calibration systems;
- vendor SDKs;
- runtime APIs.

Those systems consume the semantic contracts established here.

---

2. Architectural Authority

Zamani has one language.

Quantum computing is a domain of Zamani, not a second programming language embedded inside Zamani.

The following are therefore prohibited as competing semantic authorities:

Quantum Zamani
OpenQASM Zamani
QIR Zamani
Vendor-QPU Zamani
Quantum-ANTLR Zamani
Quantum-Rust-parser Zamani

External quantum formats may be supported as interoperability formats, but they do not redefine Zamani.

The authority hierarchy is:

1. Versioned Zamani language specification
2. grammar/spec/quantum.md
3. grammar/quantum/*.g4
4. canonical Zamani lexer/token specification
5. frontend AST contract
6. semantic quantum model
7. canonical quantum::ir contract
8. compiler/lowering contracts
9. runtime/backend implementations
10. external/interoperability formats

Historical or aspirational documents, including "grammar/Zamani-Grammar.md", cannot independently establish quantum syntax.

"grammar/grammar.md" describes implementation conformance and cannot silently create quantum semantics that are absent from this specification.

"grammar/Zamani.g4" is the canonical ANTLR composition root and must conform to this document.

---

3. Existing Repository Integration

This specification is designed around the existing repository architecture.

Relevant existing boundaries include:

grammar/Zamani.g4
grammar/quantum/quantum.g4
grammar/quantum/quantum-types.g4
grammar/quantum/quantum-states.g4
grammar/quantum/quantum-capabilities.g4
grammar/quantum/...
grammar/lexer/...
grammar/types/...
grammar/expressions/...
grammar/statements/...
grammar/resources/...
grammar/execution/...
grammar/compile/...
grammar/compatibility/...
grammar/validation/...
grammar/tests/...

The quantum orchestration grammar already defines the intended separation between quantum composition and specialized constructs. It explicitly places parsing before semantic analysis and downstream quantum compilation stages, and prohibits the grammar from depending directly on routing, scheduling, QEC, ZQN, calibration, or runtime implementation.

The existing quantum type grammar similarly establishes that quantum types describe source-level computational resources rather than physical machines.

The frontend "TypeExpr" is already intended to remain source-level and target-independent rather than becoming a hardware or semantic IR.

The canonical quantum IR explicitly establishes "quantum::ir" as the quantum semantic boundary and prohibits duplicate quantum IR implementations.

The compatibility specification requires that syntax accepted by the grammar, lexer, parser, AST, semantic layer, and IR remain coherent and prohibits silent semantic loss.

---

4. Core Quantum Principle

Zamani quantum syntax describes computational intent.

It must not prematurely describe a particular physical realization.

The source program should be able to express:

what computation is required
what quantum resources are required
what capabilities are required
what correctness properties matter
what constraints matter
what preferences exist
what semantic guarantees are required

without requiring:

which physical qubit
which QPU
which vendor
which topology
which coupling map
which control electronics
which scheduler
which routing algorithm
which calibration
which simulator
which physical gate decomposition

Those decisions belong to later compilation and execution stages.

---

5. POCO-REAF Contract

The quantum language MUST support:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever.»

This means that a valid source-level quantum program is not intrinsically tied to one machine configuration.

The same semantic program may be lowered to:

CPU simulation
GPU simulation
tensor-network simulation
distributed simulation
superconducting QPU
trapped-ion QPU
neutral-atom QPU
photonic substrate
spin-based substrate
future quantum substrate
hybrid classical/quantum systems

provided the target satisfies the semantic requirements.

The compiler may make target-specific decisions later.

---

6. No Artificial Quantum Limits

The grammar MUST NOT define universal constants such as:

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

The language must not contain an architectural assumption equivalent to:

quantum programs may contain at most N qubits

or:

quantum registers have a maximum size of N

or:

circuits may contain at most N operations

unless such a limit is explicitly supplied as an implementation/resource policy.

---

7. Infinite-Scale Meaning

"Scale to infinity" means:

«The language does not impose a finite machine-derived ceiling.»

It does not mean that every physical computer can execute an infinite computation.

A concrete compilation may be limited by:

- available memory;
- available storage;
- compiler resources;
- target capacity;
- target topology;
- target execution time;
- target availability;
- runtime policy;
- user/provider quotas;
- physical laws;
- communication capacity;
- energy;
- thermal constraints;
- fault tolerance requirements.

These are not language-level quantum limits.

The distinction is:

Language capability
    ≠
Compiler capacity
    ≠
Target capacity
    ≠
Physical capacity

---

8. Quantum Resource Abstraction

The fundamental quantum resource abstraction is:

qubit

A "qubit" represents a source-level quantum computational resource.

It does not inherently represent:

physical qubit
QPU index
hardware channel
device identifier
vendor resource
simulator array position

The source-level abstraction can therefore remain stable across target changes.

---

9. Logical Qubits

Zamani may distinguish:

qubit
logical qubit
physical qubit

but these are different semantic concepts.

A logical qubit represents a fault-tolerant or encoded computational abstraction.

It does not by itself specify:

- QEC code;
- code distance;
- physical-qubit overhead;
- decoder;
- syndrome extraction schedule;
- hardware topology.

Those are determined by later compilation and resilience layers.

---

10. Physical Qubits

Physical qubit references may exist when a program explicitly expresses physical-level intent.

However, a physical qubit reference must never silently replace a logical qubit reference.

The canonical distinction is:

QubitId
    =
logical/source-level quantum identity

PhysicalQubitId
    =
physical-target identity

The repository's canonical quantum IR already requires logical and physical identities to remain distinct and requires consumers to reuse the canonical definitions rather than creating duplicates.

Therefore:

logical → physical

is a lowering/mapping decision.

It is not an implicit parser transformation.

---

11. Quantum Types

The canonical source-level quantum types include:

qubit
logical qubit
qubit[N]
logical qubit[N]
quantum<T>
quantum::Type
quantum::Type<T>

The existing "grammar/types/quantum-types.g4" already establishes these as source-level abstractions and explicitly rejects encoding hardware characteristics into the type grammar.

Type syntax must remain extensible through Zamani's ordinary type-system mechanisms.

It must not become an enumeration of every future quantum technology.

---

12. Quantum Cardinality

Quantum cardinality is source-level information.

Examples:

qubit[n]

qubit[N + 1]

qubit[2 * n]

qubit[algorithm_width]

logical qubit[required_width]

The parser must preserve the cardinality expression.

The parser must not:

evaluate it
allocate it
convert it to usize
select a QPU
select physical qubits
impose a maximum

Semantic analysis determines whether the cardinality is valid.

Resource analysis determines whether it can be supported.

Compilation determines how it is realized.

---

13. Cardinality and Host Integer Types

A source-level quantum cardinality must not be prematurely restricted to:

u32
u64
usize
i32
i64

merely because those are convenient host representations.

If an implementation requires a bounded representation at a later stage, that is an implementation/resource boundary.

The source-language semantic value remains independent of that representation.

---

14. Quantum Registers

Quantum registers are collections of quantum resources.

They may be:

statically sized
symbolically sized
parameterized
generic
runtime-sized
resource-dependent

where supported by the semantic type system.

A register declaration must not implicitly mean:

physical qubits 0..N

unless physical mapping is explicitly requested and accepted by the appropriate target-specific layer.

---

15. Quantum Operations

Zamani MUST NOT define the quantum operation language as a closed enumeration such as:

H
X
Y
Z
S
T
CNOT
SWAP
RX
RY
RZ
...

The repository's quantum grammar architecture already identifies operation syntax as belonging to a specialized operation grammar rather than the quantum orchestration layer.

The semantic model must support:

standard operations
parameterized operations
controlled operations
adjoint/inverse operations
custom operations
user-defined operations
library operations
dialect operations
future operations
target-provided operations

without requiring a new grammar rule for every new operation name.

---

16. Operation Identity

An operation consists conceptually of:

operation name
namespace
operands
parameters
results
modifiers
attributes
effects
capabilities
source location

The operation name is semantic data.

For example:

H
X
CNOT
RX
custom_operation
my::library::operation
future::quantum::operation

must not require a hard-coded gate enumeration in the parser.

---

17. Operation Namespaces

Quantum operations may be qualified.

Examples:

quantum::H
quantum::measurement
library::operation
domain::operation
dialect::operation

Name resolution is semantic.

The grammar must not assume that every operation belongs to a globally predefined gate registry.

---

18. Parameters

Operations may have zero or more parameters.

Parameters may be:

literal
constant
symbol
expression
generic parameter
compile-time value
runtime value
resource-dependent value

The grammar must not impose an artificial universal parameter count.

Semantic analysis determines which parameter forms an operation accepts.

---

19. Operands

Operations may consume:

single qubits
multiple qubits
registers
slices
logical qubits
resource groups
classical values
observables
quantum states
domain-specific quantum resources

Operand count is an operation semantic property, not a universal grammar limit.

---

20. Controlled Operations

Controlled operations must be compositional.

The language must support arbitrary valid control structures rather than hard-coding:

one control
two controls
three controls

as the only possibilities.

The operation representation should preserve:

base operation
control operands
target operands
control polarity
modifiers

without lowering directly to physical gates.

---

21. Adjoint / Inverse Operations

The source language may express inverse or adjoint intent.

Examples conceptually include:

adjoint operation
inverse operation

The grammar records the modifier.

Semantic analysis determines whether the operation supports that transformation.

Optimization/lowering determines how it is realized.

The parser must not replace the operation with a physical gate sequence.

---

22. Operation Modifiers

Quantum operations may support compositional modifiers such as:

controlled
adjoint
inverse
power
repeat
conditional

The exact modifier vocabulary is defined by the specialized grammar and syntax specification.

Modifiers must remain structured AST information.

They must never be silently discarded during lowering.

The compatibility specification explicitly prohibits semantic information such as quantum modifiers from being parsed and then lost.

---

23. User-Defined Operations

Zamani must permit user-defined quantum operations.

A user-defined operation may contain:

parameters
quantum operands
classical parameters
local declarations
quantum operations
measurement where semantically permitted
control flow where permitted
effects
resource requirements
capabilities

The compiler must validate the definition semantically.

The grammar must not require the operation to correspond to a built-in gate.

---

24. Quantum Circuits

A circuit is a source-level computational structure.

A circuit may express:

resource declarations
state preparation
operations
measurement
classical feed-forward
control flow
resource requirements
capabilities
effects
verification constraints

A circuit is not inherently tied to:

a QPU
a topology
a native gate set
a scheduler
a pulse representation

---

25. Circuit Width and Depth

Circuit width and depth are semantic properties that may be computed or estimated.

They are not universal parser limits.

The language must not contain:

MAX_CIRCUIT_WIDTH
MAX_CIRCUIT_DEPTH

as language rules.

A compiler may calculate:

required width
estimated depth
critical path
resource pressure

as analysis results.

---

26. Quantum State Expressions

Quantum states may be represented using the quantum-state grammar.

Examples may include:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

and future generalized state expressions.

State notation describes source-level mathematical/semantic state intent.

It does not require the runtime to materialize a state vector.

A state may later be represented using:

state vector
density matrix
tensor network
stabilizer representation
decision diagram
symbolic representation
hardware-native state

depending on target and semantics.

---

27. State Representation Independence

The grammar must never imply:

N qubits → allocate 2^N host elements

as a language requirement.

That may be one simulator strategy.

It is not the meaning of a quantum program.

This distinction is essential for POCO-REAF and for large-scale quantum computation.

---

28. Initialization

Quantum initialization expresses semantic state preparation.

It does not prescribe:

- physical pulses;
- calibration;
- waveform;
- transport;
- optical pumping;
- cooling;
- hardware reset mechanism.

Those belong to target realization.

---

29. Reset

Reset is a semantic operation.

It may express:

reset q
reset register

where supported.

Reset does not imply a particular physical implementation.

The reset grammar owns syntax.

The quantum semantic layer owns meaning.

The backend owns realization.

---

30. Measurement

Measurement must be a first-class semantic construct.

It must preserve:

measured quantum resource
measurement basis
measurement mode
destination
measurement metadata
classical result identity
source span

where applicable.

The parser must not automatically insert measurements.

In particular, a quantum program must not be changed semantically merely because the selected backend requires measurement.

---

31. Measurement Basis

The language should support abstract measurement bases.

Examples include:

Z
X
Y
custom observable
operator-defined basis

The language must remain extensible.

A basis name does not necessarily identify a physical measurement implementation.

---

32. Measurement Results

Measurement results are classical semantic values.

The language must support the quantum-to-classical boundary explicitly.

Conceptually:

quantum state
    ↓
measurement
    ↓
classical result
    ↓
classical computation

The result must not be represented as an implicit side effect that disappears from the AST.

---

33. Mid-Circuit Measurement

Mid-circuit measurement is a first-class capability.

A program may semantically perform:

quantum operation
measurement
classical decision
quantum operation

without requiring the entire circuit to be statically measured at the end.

This must survive:

parser
→ AST
→ semantic model
→ quantum::ir
→ target lowering

where supported.

---

34. Dynamic Circuits

Dynamic quantum programs may contain runtime-dependent control flow.

Examples:

measurement
if result ...
while condition ...
repeat until ...
conditional operation

The grammar must represent this structure.

The compiler determines whether:

static lowering
dynamic lowering
hybrid execution
host/device synchronization

is required.

The grammar must not assume that every quantum program is a static circuit.

---

35. Quantum/Classical Boundary

Quantum and classical computation are parts of one Zamani program.

The semantic boundary must explicitly preserve:

classical → quantum
quantum → classical
classical control → quantum operation
measurement → classical value
classical parameter → quantum operation

The language must not require a separate programming language for classical orchestration.

---

36. Classical Feed-Forward

Classical feed-forward is semantic control flow.

For example:

measure q -> result

if result {
    apply operation to another_q
}

must remain semantically visible.

The compiler may lower it into:

dynamic QPU control
host control
embedded controller control
static specialization

depending on target capabilities.

---

37. Observables

The language may express observables as semantic mathematical objects.

An observable may be:

named
composed
parameterized
tensor/product structured
library-defined
dialect-defined

Observable syntax must not be tied to a particular simulator representation.

---

38. Quantum Channels and Noise

The language may express abstract channels/noise semantics.

Examples conceptually include:

noise model
channel
decoherence requirement
error channel
fault model

These are semantic constructs.

The grammar must not implement a simulator's noise engine.

---

39. Noise Is Not Hardware Discovery

A source program expressing:

requires fidelity ...
requires noise tolerance ...
requires fault tolerance ...

does not mean:

select QPU X

The target analysis layer determines whether the available hardware satisfies the requirement.

---

40. Error Correction

Quantum error correction is a semantic domain.

The grammar may express:

error-correction intent
fault-tolerance requirement
logical-qubit requirement
code-family intent
distance requirement
syndrome-related intent
reliability requirement

where supported by the language specification.

But the grammar must not implement:

decoder
syndrome extraction algorithm
lookup table
recovery algorithm
QEC scheduling
physical code layout

Those belong to the QEC/compiler layers.

---

41. QEC and Resource Separation

The distinction must remain:

source requirement
    ↓
semantic QEC intent
    ↓
QEC analysis
    ↓
resource estimation
    ↓
physical realization

For example:

requires fault_tolerance

is different from:

use physical qubits 0..999

The first is portable intent.

The second is target-specific realization.

---

42. ZQN Boundary

ZQN is the repository's fault/noise semantic layer.

The grammar may express source-level fault/noise requirements.

It must not implement ZQN.

The intended flow is:

Zamani source
    ↓
quantum semantic model
    ↓
quantum::ir
    ↓
ZQN interpretation
    ↓
fault/noise analysis

The grammar must never import or depend upon a ZQN implementation.

---

43. Resilience Boundary

Resilience is a runtime/compiler orchestration concern.

The language may express requirements or policies such as:

fault tolerance
recovery
retry policy
degradation tolerance
reliability requirements

but the grammar does not decide whether the runtime should:

retry
recover
reroute
reschedule
replace backend
degrade
escalate
reject

Those decisions belong to the resilience subsystem.

---

44. Capabilities

Quantum programs may require capabilities.

Examples conceptually include:

quantum computation
mid-circuit measurement
dynamic control
fast classical feed-forward
fault tolerance
logical qubits
entanglement
specific observable support
specific precision
specific measurement capability

Capabilities describe what a target must provide.

They do not select a particular vendor or device.

---

45. Requirements, Constraints, Preferences, Hints

Zamani must distinguish:

requirement
constraint
preference
hint
implementation decision

For example:

requires capability("mid_circuit_measurement")

is a requirement.

prefer capability("fast_feed_forward")

is a preference.

hint minimize_latency

is a hint.

map logical_q0 -> physical_q17

is an implementation decision.

These must not be conflated.

---

46. Hardware Independence

Quantum source syntax must not contain universal dependencies on:

IBM
IonQ
Quantinuum
Rigetti
specific QPU names
device IDs
physical qubit numbers
vendor coupling maps
vendor pulse formats
vendor calibration IDs

Vendor interoperability belongs under "grammar/interoperability/" and dialect mechanisms.

---

47. Physical Intent

Physical-level programming may be necessary for specialized applications.

Zamani must therefore support an explicit physical-intent boundary rather than pretending all programs are purely abstract.

Physical intent may express:

physical resource requirement
topology preference
native-operation preference
timing requirement
calibration requirement
placement constraint

But physical intent must remain explicitly marked as such.

It must not silently contaminate the portable semantic model.

---

48. Logical-to-Physical Mapping

The mapping:

logical qubit
    ↓
physical qubit

belongs to routing/placement.

The grammar may express constraints such as:

prefer locality
requires connectivity
requires topology property

but it must not implement the mapping algorithm.

The canonical IR already establishes that logical and physical qubit identity are distinct and that logical-to-physical conversion belongs downstream.

---

49. Routing

Routing consumes semantic quantum operations and target capabilities.

Conceptually:

quantum::ir
    ↓
routing
    ↓
physical realization

Routing may:

insert swaps
change placement
choose paths
decompose operations
adapt to topology

None of those transformations should be encoded into source-level quantum grammar.

---

50. Scheduling

Scheduling determines:

operation order
parallelism
resource conflicts
timing
dependencies
latency

The grammar may express timing/resource intent.

It must not embed one scheduler's implementation.

There must be no grammar rule such as:

run_on_8_qubits
run_on_16_threads
schedule_for_device_X

as universal semantics.

---

51. Optimization

Quantum optimization may transform:

operation sequences
circuit structure
parameter representations
decompositions
resource usage

provided semantic equivalence is preserved.

Optimization must operate after semantic interpretation.

The grammar must not encode optimization algorithms as syntax merely because an optimizer currently supports them.

---

52. Pulse-Level Programming

Zamani may expose pulse-level intent where required.

Pulse syntax must describe semantic intent such as:

pulse
duration
amplitude
frequency
phase
frame
waveform

when those are genuinely part of the language domain.

It must not prescribe:

DAC
ADC
sample rate
physical channel ID
control electronics
vendor waveform format

Those belong to hardware lowering.

---

53. Timing

Quantum timing values must remain semantically typed.

A source program may express:

duration
latency
deadline
synchronization
timing constraint

without assuming one physical clock implementation.

The compiler resolves timing against target capabilities.

---

54. Quantum Effects

Quantum effects may describe properties such as:

measurement
state mutation
resource consumption
entanglement
classical interaction
timing
hardware interaction

Effects must survive into semantic analysis where they affect correctness.

They must not be parsed and silently discarded.

---

55. Quantum Resource Ownership

Quantum resources may require linear or affine semantics.

The language must be capable of expressing and validating:

ownership
consumption
borrowing where permitted
measurement-induced state transitions
reset requirements
resource lifetime

The frontend AST records syntax.

Semantic analysis enforces legality.

The grammar must not attempt to perform ownership checking.

---

56. No-Cloning and Semantic Validation

The grammar does not need a special parser rule for every no-cloning restriction.

Instead:

syntax
    ↓
AST
    ↓
quantum resource/type analysis

determines whether an operation illegally duplicates a quantum resource.

This keeps quantum correctness in the semantic layer rather than making the grammar unnecessarily rigid.

---

57. Measurement State Transitions

Measurement may change the semantic state/lifetime of a quantum resource.

That information must survive into semantic analysis.

For example:

measure q
use q

must be accepted or rejected according to the quantum type/resource semantics.

The grammar itself only records the source structure.

---

58. Generic Quantum Operations

Generic operations must be supported where semantically meaningful.

A generic operation may abstract over:

qubit type
register size
parameter type
operation implementation
resource capability
target specialization

This allows source programs to remain portable across scale.

---

59. Quantum Generics

Quantum generic syntax must integrate with the existing Zamani generic type/function system.

The quantum grammar must not create a second generic system.

For example, conceptual forms such as:

fn algorithm<Q>(register: Q)

must use the ordinary generic mechanism.

Quantum-specific constraints belong to semantic type constraints/capabilities.

---

60. Quantum Compile-Time Parameters

Quantum programs may depend on compile-time symbolic values:

N
WIDTH
DEPTH
PRECISION
TOLERANCE

The grammar must preserve those symbolic expressions.

The compiler determines which values can be resolved at compile time.

The language must not force all quantum resource sizes into host integer constants.

---

61. Runtime-Dependent Quantum Resources

Where the language permits dynamic allocation, the source may express runtime-dependent resource requirements.

Examples conceptually include:

allocate according to n
allocate according to capability
allocate according to runtime workload

Semantic analysis must determine whether the selected execution model supports this.

---

62. Resource Discovery

The quantum program may express capability requirements.

The compiler/runtime may discover actual resources.

The source program must not need to know:

number of available QPUs
physical qubit count
current topology
backend queue
device location
vendor

unless it intentionally requests such information through an explicit introspection interface.

---

63. Resource Availability

A semantic requirement may fail at deployment.

For example:

requires logical_qubits >= required_width

does not make every machine satisfy the requirement.

The correct behavior is:

program remains valid
target feasibility fails or alternate target is selected

rather than changing the program's meaning.

---

64. Target Negotiation

Target negotiation belongs downstream.

The system may compare:

program requirements
target capabilities
target resource availability
target constraints
execution policies

and select an appropriate realization.

This enables POCO-REAF.

---

65. Canonical AST Contract

Every valid quantum syntax construct must have a defined mapping into the canonical frontend AST.

The required direction is:

quantum source
    ↓
parser
    ↓
existing generic AST structures
    ↓
semantic quantum model

The quantum grammar must not introduce a duplicate AST merely because a construct is quantum.

The existing frontend AST explicitly defines "TypeExpr" as a source-level representation rather than a semantic/hardware representation.

---

66. Generic Operation AST

Quantum operations should use the repository's generic operation model rather than creating:

enum QuantumGate {
    H,
    X,
    Y,
    ...
}

as the universal representation.

The preferred semantic shape is:

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

Quantum-specific semantic validation determines whether the operation is valid.

---

67. AST Source Preservation

Quantum AST nodes must preserve sufficient source information for:

- diagnostics;
- IDE tooling;
- formatting;
- refactoring;
- compatibility;
- source-to-source transformations;
- semantic error reporting.

Source spans must not be discarded merely because the backend does not require them.

---

68. AST Does Not Own Hardware

The quantum AST must not contain fields whose only purpose is:

QPU vendor
physical device ID
physical topology
calibration database ID
backend queue ID
hardware credential

unless the construct is explicitly a target/deployment configuration rather than portable quantum semantics.

---

69. Canonical IR Boundary

All quantum semantics must ultimately lower through the canonical:

quantum::ir

boundary.

There must not be:

grammar quantum IR
frontend quantum IR
OpenQASM quantum IR
Zamani quantum IR
QEC quantum IR

as competing canonical representations.

The existing "src/quantum/ir/quantum/mod.rs" explicitly establishes this architecture and says that the quantum domain facade must reuse the canonical IR types rather than duplicate them.

---

70. Canonical Quantum IR Responsibilities

"quantum::ir" owns semantic quantum representation.

It may represent concepts such as:

QubitId
PhysicalQubitId
Gate
Measurement
ClassicalBitId
Channel
Pulse
Frame
Waveform
Instruction
Program
Quantum resources

as already established by the repository.

It does not own:

routing algorithms
scheduling algorithms
QEC decoders
calibration databases
backend APIs
source parsing

The existing canonical IR contract explicitly separates these concerns.

---

71. IR Identity Rule

There must be exactly one canonical definition of:

quantum::ir::qubit::QubitId
quantum::ir::qubit::PhysicalQubitId
quantum::ir::gate::Gate
quantum::ir::measurement::Measurement

New code must reuse these types.

No grammar or frontend specification may introduce a duplicate semantic identity type.

---

72. IR Integration Contract

Every quantum feature must define:

Grammar rule
    ↓
AST node
    ↓
Semantic representation
    ↓
quantum::ir representation

If a construct cannot yet lower to canonical IR, it must be explicitly classified as:

specified
experimental
unsupported
future

It must not be silently accepted and dropped.

The compatibility specification explicitly prohibits semantic information from disappearing between parser, AST, semantic analysis, and IR generation.

---

73. Compiler Integration

The compiler must process quantum programs in this conceptual order:

parse
→ structural validation
→ name resolution
→ type validation
→ effect validation
→ resource validation
→ quantum semantic validation
→ canonical IR lowering
→ IR verification
→ optimization
→ QEC/resource analysis
→ routing
→ scheduling
→ resilience/ZQN processing
→ target lowering

The exact implementation may combine stages, but responsibilities must remain distinguishable.

---

74. Quantum IR Verification

Before target lowering, quantum IR must be verified.

Verification must check applicable invariants including:

valid identifiers
valid operand references
valid operation parameters
valid resource relationships
valid measurement references
valid classical/quantum boundaries
valid control dependencies
valid quantum resource ownership
valid operation structure

Verification must not depend on one particular hardware topology.

---

75. Routing Integration

Routing consumes canonical quantum semantics.

The quantum grammar does not invoke routing.

The compiler may supply:

logical qubits
operation dependencies
connectivity requirements
topology constraints
capabilities
resource requirements

to routing.

Routing produces target realization.

---

76. Scheduling Integration

Scheduling consumes the routed/optimized representation and target constraints.

The quantum grammar may specify semantic timing constraints.

It does not determine:

ASAP
ALAP
list scheduling
resource-constrained scheduling
dynamic scheduling

algorithms.

---

77. Optimization Integration

Optimization may transform the canonical quantum representation while preserving semantic equivalence.

Optimization must not:

change measurement semantics
discard required effects
discard resource requirements
discard capabilities
change observable meaning

without an explicitly valid semantic transformation.

---

78. Hardware HAL Integration

HAL exposes actual target capabilities.

The language does not need to know how HAL discovers them.

Conceptually:

quantum program
    ↓
requirements
    ↓
HAL capabilities
    ↓
feasibility
    ↓
target realization

HAL must not become a dependency of the grammar.

---

79. Runtime Integration

Runtime consumes compiled quantum representations.

Runtime may handle:

execution
resource acquisition
measurement results
classical feed-forward
checkpointing
recovery
observability
telemetry

but none of these implementation mechanisms should be required for the parser to understand ordinary quantum syntax.

---

80. Determinism

Parsing must be deterministic.

For the same:

source
language version
grammar version
lexical configuration

the parser must produce the same AST.

Semantic analysis must be deterministic unless nondeterminism is explicitly part of the language semantics.

Quantum optimization may have implementation choices, but reproducible compilation must be possible under a deterministic compilation policy.

---

81. Source Ordering

Quantum source ordering must be preserved where semantically significant.

The compiler must not rely on hash-map iteration order to determine:

operation ordering
operand ordering
parameter ordering
diagnostic ordering
resource declaration ordering

---

82. Diagnostics

Quantum diagnostics must be structured and source-located.

Examples include:

unknown quantum operation
invalid quantum operand
invalid operation arity
invalid parameter
invalid measurement target
invalid quantum type
invalid resource requirement
unsupported capability
invalid quantum/classical conversion
illegal quantum resource reuse
invalid dynamic control
unsupported target requirement

Diagnostics must not be implemented by emitting comments and continuing as though compilation succeeded.

This follows the repository's compatibility contract.

---

83. Error Recovery

Parser recovery may continue to collect diagnostics.

However:

invalid source

must not silently become:

valid quantum program

in production compilation.

Recovery AST nodes must remain clearly distinguishable from validated semantic constructs.

---

84. Security

Quantum grammar processing must be safe for untrusted source.

The implementation must:

- use safe Rust;
- forbid unsafe code;
- avoid raw pointer operations;
- avoid unchecked memory manipulation;
- avoid implicit filesystem access;
- avoid implicit network access;
- avoid executing quantum operations during parsing;
- avoid invoking hardware during parsing.

The Rust baseline is:

Rust 1.97 / Rust 1.97.1
Edition 2021
stable Rust
no unsafe

Rust implementations should enforce:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

where applicable.

---

85. Parser Resource Limits

The language must not impose machine-derived quantum limits.

However, implementations may expose explicit compiler resource policies for protection against denial-of-service inputs.

For example:

maximum parser recursion
maximum source size
maximum AST memory
maximum compilation time
maximum diagnostic count

These must be:

1. implementation policies;
2. explicitly configurable;
3. externally observable;
4. not part of quantum language semantics;
5. never confused with "MAX_QUBITS".

---

86. Resource Policy Separation

A compiler may reject a source file because:

compiler memory budget exhausted

without implying:

Zamani cannot represent this quantum program.

This distinction must remain explicit in diagnostics.

---

87. Scalability Requirements

Quantum grammar components must support:

single qubit
small circuits
large circuits
parameterized circuits
symbolically sized circuits
large quantum programs
distributed quantum programs
future quantum substrates

without changing the source language solely because the scale increases.

The architecture must not require a new grammar for:

1 qubit
10 qubits
10,000 qubits
10^6 qubits

---

88. Structural Recursion

Where the language structure is recursive, the grammar must use recursive definitions rather than manually enumerating fixed depths.

Examples:

nested expressions
nested types
qualified names
operation modifiers
controlled operations
blocks
generic arguments
composed states
composed observables

This is necessary for scalability.

---

89. No Fixed Namespace Depth

Qualified quantum names must not impose:

quantum::foo

as the maximum.

The grammar should support:

a::b::c::d::...

subject only to implementation resource policy.

---

90. No Fixed Generic Arity

Quantum generic constructs must not impose a universal maximum number of arguments.

Semantic rules may constrain the arguments of a particular type or operation.

The grammar itself should remain compositional.

---

91. No Fixed Operation Arity

The grammar must not assume:

one operand
two operands
three operands

as universal limits.

An operation's semantic signature determines valid arity.

---

92. No Fixed Control Count

Controlled operations must support arbitrary valid control collections.

The grammar must not have separate universal rules such as:

oneControlledGate
twoControlledGate
threeControlledGate

as the only possible representations.

---

93. No Fixed Circuit Size

A circuit is an ordered/composed semantic structure.

The grammar must not impose a fixed number of:

operations
blocks
registers
measurements
parameters

---

94. No Fixed QPU Size

No source-level grammar rule may assume:

QPU has N qubits

or:

target supports exactly N operations

Target capability information belongs downstream.

---

95. Quantum Dialects

Quantum dialects may extend Zamani for:

hardware-specific features
research features
specialized mathematical models
new quantum substrates
future quantum technologies

A dialect must declare:

name
version
syntax extensions
semantic extensions
AST mapping
IR mapping
capabilities
compatibility
feature status

A dialect must not silently modify stable core semantics.

---

96. Dialect Isolation

A dialect must not create a second quantum IR.

Dialect constructs must lower into:

canonical Zamani semantic model

and then:

canonical quantum::ir

where applicable.

---

97. Interoperability

Zamani may interoperate with:

OpenQASM
QIR
Quil
Q#
other quantum IRs
vendor formats
simulation formats
hardware description formats

These are interoperability surfaces.

They must not replace Zamani's semantic authority.

---

98. OpenQASM Integration

OpenQASM is an external source/interoperability format.

Its frontend should follow:

OpenQASM
    ↓
OpenQASM frontend representation
    ↓
Zamani semantic model
    ↓
quantum::ir

It must not become:

OpenQASM AST
    ↓
OpenQASM IR

inside the canonical Zamani compiler.

---

99. QIR Integration

QIR is an interoperability/target representation.

The canonical direction is:

Zamani semantic quantum model
    ↓
quantum::ir
    ↓
QIR lowering

not:

Zamani grammar
    ↓
QIR AST

QIR-specific implementation details must remain outside the grammar.

---

100. External Gate Sets

External gate sets may be represented as operation names or dialect/library semantics.

The grammar must not need to be modified every time an external ecosystem adds an operation.

This is essential for future-proofing.

---

101. Standard Library Boundary

Quantum standard-library operations may provide:

algorithms
state preparation
linear algebra
observables
measurement utilities
quantum arithmetic
quantum communication

but library APIs are not automatically language keywords.

A standard-library addition should not require modifying the core grammar unless it introduces genuinely new language syntax.

---

102. Algorithms

Quantum algorithms belong primarily to libraries/semantic capabilities rather than the core parser.

Examples:

Grover
QFT
phase estimation
amplitude estimation
simulation
optimization

must not automatically become keywords.

The language should provide the expressive primitives required to implement or invoke them.

---

103. Mathematical Integration

Quantum programs must integrate with the universal Zamani mathematical/type system.

Quantum parameters may use:

numeric expressions
symbolic expressions
matrices
vectors
tensors
complex values
probabilities
angles
units

where supported.

Quantum grammar must not duplicate the mathematical expression language.

---

104. Complex Numbers

Complex-number syntax must be owned by the universal numeric/math grammar where possible.

Quantum syntax consumes the resulting expression.

It must not invent a separate complex-number language.

---

105. Tensor and Matrix Integration

Quantum state/operator expressions may use universal:

vector
matrix
tensor

semantic structures.

The quantum grammar should only introduce syntax that is genuinely quantum-specific.

---

106. Hybrid Computing

Quantum and classical computation may coexist in one function, module, or program.

The compiler must preserve:

classical computation
quantum computation
quantum/classical dependencies

without forcing manual source-level synchronization that is an implementation artifact.

---

107. Distributed Quantum Computing

The language must support distributed quantum intent where the wider distributed grammar supports it.

Conceptual requirements may include:

distributed quantum resource
remote quantum operation
entanglement resource
communication capability
partitioning
replication
coordination

The grammar must not hard-code:

2 nodes
8 nodes
N fixed QPUs

as universal semantics.

---

108. Quantum Networking

Quantum networking constructs should integrate with:

networking/
distributed/
resources/
security/

rather than creating a second network language.

Quantum networking syntax may express semantic intent such as:

entanglement resource
quantum channel
quantum communication
remote operation
network capability

while actual routing belongs to networking/runtime layers.

---

109. Quantum Security

Quantum security constructs must integrate with the universal security grammar.

The quantum language must distinguish:

quantum computation
post-quantum cryptography
quantum communication
quantum identity

These are related but not identical domains.

Cryptographic algorithms should remain library/semantic capabilities rather than being forced into parser keywords.

---

110. Quantum Simulation

Simulation is a target/backend concern.

A source program must not need to be rewritten merely to run on:

state-vector simulator
density-matrix simulator
stabilizer simulator
tensor-network simulator
distributed simulator
hardware

The same semantic program should be usable where the target supports its requirements.

---

111. Simulation-Specific Syntax

Simulation-specific controls may exist under an explicit simulation/dialect layer.

They must not become requirements of portable quantum programs.

For example:

use statevector simulator

must be target/deployment intent rather than a universal quantum semantic requirement.

---

112. Calibration

Calibration is not grammar semantics.

A source program may express:

requires calibrated capability
requires fidelity threshold
requires timing accuracy

but the calibration system decides:

which calibration
when calibration occurs
how calibration is performed
which control parameters are used

---

113. Backend Selection

Backend selection is not part of portable quantum syntax unless explicitly expressed through the deployment/target configuration domain.

The core quantum language must remain backend-independent.

---

114. Deployment

Deployment may specify:

target class
provider constraints
region
availability
cost preference
latency preference
security requirement
resource requirement

but deployment configuration must remain separate from the semantic meaning of the quantum algorithm.

---

115. Source Portability

A quantum source program is portable when its semantic meaning does not depend on a target-specific implementation detail.

Target-specific syntax must therefore be explicitly isolated.

---

116. Compatibility

Every quantum construct must have a compatibility status:

STABLE
IMPLEMENTED
SPECIFIED
EXPERIMENTAL
DEPRECATED
REMOVED
RESERVED

These statuses follow the repository's compatibility specification.

---

117. Quantum Feature Lifecycle

A new quantum feature follows:

proposal
    ↓
semantic design
    ↓
grammar specification
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
compatibility tests
    ↓
stable

No feature becomes stable merely because its ".g4" file exists.

---

118. Feature Manifest

Every substantial quantum feature should have a machine-readable feature manifest under the repository's feature/specification infrastructure.

The manifest must identify:

feature ID
name
status
language version
grammar owner
lexer dependencies
AST mapping
semantic mapping
IR mapping
compiler consumers
runtime consumers
capabilities
resource requirements
positive tests
negative tests
boundary tests
scalability tests
compatibility tests
hard-coding audit

This allows the feature to be independently completed.

---

119. File Ownership

The quantum grammar ownership model is:

grammar/quantum/quantum.g4
    orchestration only

grammar/quantum/qubits.g4
    qubit declarations/references

grammar/quantum/quantum-registers.g4
    register syntax

grammar/quantum/quantum-types.g4
    quantum type syntax

grammar/quantum/quantum-states.g4
    state syntax

grammar/quantum/operations.g4
    operation invocation

grammar/quantum/controlled-operations.g4
    controlled operation syntax

grammar/quantum/parameterized-operations.g4
    parameterized operations

grammar/quantum/measurement.g4
    measurement syntax

grammar/quantum/reset.g4
    reset syntax

grammar/quantum/observables.g4
    observable syntax

grammar/quantum/dynamic-circuits.g4
    dynamic circuit syntax

grammar/quantum/mid-circuit-control.g4
    measurement-dependent control

grammar/quantum/quantum-classical.g4
    explicit quantum/classical boundaries

grammar/quantum/logical-qubits.g4
    logical-qubit source syntax

grammar/quantum/physical-qubits.g4
    explicit physical intent

grammar/quantum/error-correction.g4
    QEC intent

grammar/quantum/quantum-resources.g4
    quantum resource requirements

grammar/quantum/quantum-capabilities.g4
    capability requirements

grammar/quantum/quantum-dialects.g4
    dialect extension syntax

The existing orchestration file already documents this ownership model.

---

120. One Rule, One Owner

Every grammar production must have exactly one owner.

A rule must not be duplicated across:

grammar/quantum
grammar/types
grammar/effects
grammar/antlr
Zamani.g4

unless one occurrence is explicitly a composition reference rather than a second definition.

---

121. "grammar/antlr/"

The repository currently contains an older quantum grammar under:

grammar/antlr/Quantum.g4

This must not become a competing quantum grammar authority.

The canonical quantum grammar must be the modular grammar under:

grammar/quantum/

composed through:

grammar/Zamani.g4

If "grammar/antlr/Quantum.g4" is still required by tooling, it must become a compatibility/generated artifact with an explicit status.

If no tooling consumes it, it should eventually be removed rather than maintained as a second grammar authority.

No unnecessary rename is required.

---

122. Lexer Contract

Quantum grammar files must consume the canonical lexer vocabulary.

They must not declare duplicate lexer rules.

The lexical authority is:

grammar/lexer/

and the executable lexer implementation remains in the Rust frontend.

The compatibility specification requires the Rust lexer and ANTLR lexical representation to agree.

---

123. Existing Token Naming Discrepancies

The repository currently contains quantum grammar material using older token naming conventions alongside the newer "K_*" token vocabulary.

For example, the quantum type grammar documents the migration from names such as:

QUBIT
INT
FLOAT_TYPE
BOOL_TYPE

toward:

K_QUBIT
K_INT
K_FLOAT
K_BOOL

and identifies "ZamaniTokens" as the intended vocabulary.

This specification therefore requires:

«Quantum specification files must describe semantic token meaning, while the canonical lexer/token vocabulary owns actual token spelling and token identifiers.»

Do not add token aliases independently to every quantum grammar file.

The migration must be centralized.

---

124. Grammar Integration

"grammar/Zamani.g4" remains the root composition grammar.

It should compose:

core
types
expressions
statements
declarations
modules
effects
memory
concurrency
classical
quantum
hybrid
hdl
hardware
distributed
ai
data
networking
security
resources
compile
execution
interoperability
dialects
macros
metaprogramming

The quantum specification does not replace the root grammar.

It defines what the quantum portion means.

---

125. No Direct IR Dependency from Grammar

No ".g4" file may import Rust quantum IR types.

Grammar source has no dependency on:

quantum::ir
QEC
ZQN
HAL
routing
scheduling
runtime

Those are downstream consumers.

---

126. No Semantic Implementation in Grammar

The grammar must not implement:

gate matrices
state evolution
measurement probabilities
QEC decoding
routing
scheduling
calibration
noise simulation
backend selection

It only recognizes source structure.

---

127. Semantic Contract

For every accepted quantum construct:

syntax

must map to:

well-defined semantic meaning

If semantic meaning does not exist, the construct must not be classified as stable.

---

128. No Silent Semantic Loss

The following is forbidden:

source
 ↓
parser
 ↓
AST
 ↓
information discarded
 ↓
IR

Examples include losing:

operation modifier
measurement basis
resource requirement
capability requirement
logical/physical distinction
quantum/classical dependency
source span
effect
parameter

---

129. Quantum Semantic Equivalence

Two quantum programs may be optimized or lowered differently while retaining the same semantics.

For example:

source operation sequence

may become:

optimized operation sequence

or:

native target operation sequence

provided observable program semantics are preserved.

---

130. Observational Semantics

Quantum transformations must preserve applicable:

measurement distributions
classical outputs
observable values
resource correctness requirements
error/fault semantics
side effects
required effects

unless the program explicitly permits approximation or nondeterministic behavior.

---

131. Approximate Quantum Computation

If approximation is supported, it must be explicit.

A compiler must not silently replace an exact operation with an approximate one without a valid semantic policy.

Approximation may be expressed through:

precision
error tolerance
fidelity requirement
approximation policy

as resource/semantic requirements.

---

132. Precision

Precision requirements must not be tied to:

float32
float64

unless the source explicitly chooses those types.

The quantum language should be able to express semantic precision requirements.

Target lowering determines representation.

---

133. Probabilities and Results

Quantum measurement probabilities are semantic values.

Their source representation must not imply a particular runtime numeric format.

The compiler may choose an appropriate representation.

---

134. Resource Estimation

The compiler may estimate:

qubit count
logical qubit count
physical resource count
gate count
depth
communication
memory
execution time
error budget

These are compiler analysis outputs.

They are not universal grammar limits.

---

135. Resource Failure

If a target cannot satisfy a quantum resource requirement, the compiler/runtime must report a structured feasibility failure.

It must not:

silently reduce the number of qubits
drop operations
change measurement semantics
select a weaker algorithm

unless the program explicitly permits adaptation.

---

136. Adaptive Compilation

Adaptive compilation may be supported.

For example:

requires capability X
prefer capability Y
allow fallback Z

The compiler may select a valid implementation.

Adaptation must remain semantically constrained.

---

137. Quantum Program Portability Classes

Quantum programs may be classified as:

fully portable
capability-constrained
resource-constrained
target-constrained
physical-intent
hardware-specific

The classification must be explicit.

A hardware-specific program must not accidentally be represented as universally portable.

---

138. Physical-Specific Boundary

If a program deliberately uses:

physical qubit IDs
specific topology
specific pulse constraints
vendor operation

it enters a target-specific domain.

That does not invalidate the language.

It simply changes the portability classification.

---

139. Quantum/HDL Co-Design

Quantum syntax may interact with HDL/hardware intent.

The semantic boundary must remain:

quantum computation
    ↓
hardware intent
    ↓
HDL/hardware representation

Quantum grammar must not absorb HDL grammar.

---

140. Quantum/AI Integration

Quantum machine learning is a hybrid domain.

It should combine:

quantum
classical
AI
data

without creating a separate quantum-AI grammar.

The same operation/type/resource/capability mechanisms should be reused.

---

141. Quantum/Data Integration

Quantum programs may operate on:

classical datasets
quantum states
observables
tensor structures
streams
measurement results

The data domain owns generic data syntax.

The quantum domain owns genuinely quantum-specific syntax.

---

142. Quantum/Distributed Integration

Distributed quantum programs may use the distributed domain's:

processes
services
channels
messages
placement
replication
fault tolerance

with quantum semantic resources.

The quantum grammar must not duplicate distributed syntax.

---

143. Quantum/Networking Integration

Quantum networking must use common networking concepts where possible.

Quantum-specific constructs should extend the generic model rather than create a second endpoint/channel language.

---

144. Quantum/Security Integration

Security constraints must integrate with:

security/
resources/
capabilities/

Quantum-specific security should not create a duplicate identity or authorization system.

---

145. Versioning

Quantum syntax is versioned as part of the Zamani language version.

A new quantum feature must not silently reinterpret old source.

Breaking changes require explicit language-version handling according to "grammar/spec/compatibility.md".

---

146. Reserved Syntax

Future quantum syntax may be reserved.

Reserved syntax must:

not be accepted as an implemented feature
produce deterministic diagnostics when appropriate
not silently change meaning

---

147. Experimental Syntax

Experimental quantum constructs must be:

explicitly marked
versioned
feature-gated where appropriate
covered by tests
isolated from stable semantics

---

148. Deprecation

Deprecated quantum syntax must define:

deprecated version
replacement
migration path
removal policy

No deprecated construct may silently change meaning.

---

149. Conformance Requirements

A quantum feature is production complete only when all applicable layers exist:

[ ] normative specification
[ ] lexical contract
[ ] grammar contract
[ ] AST contract
[ ] semantic contract
[ ] quantum::ir mapping
[ ] IR verification
[ ] compiler integration
[ ] runtime integration
[ ] interoperability mapping where applicable
[ ] diagnostics
[ ] positive tests
[ ] negative tests
[ ] boundary tests
[ ] scalability tests
[ ] determinism tests
[ ] compatibility tests
[ ] hard-coding audit
[ ] documentation

---

150. Positive Tests

Quantum tests must cover at least:

qubit
logical qubit
registers
symbolic cardinality
states
operations
parameterized operations
controlled operations
adjoint/inverse operations
custom operations
measurement
reset
observables
dynamic circuits
mid-circuit control
quantum/classical interaction
capabilities
resources
QEC intent
dialects
interoperability

---

151. Negative Tests

Negative tests must cover:

invalid quantum type
invalid operand
invalid operation arity
invalid parameter
invalid measurement
invalid resource reference
invalid quantum/classical conversion
invalid control
invalid state expression
invalid capability
invalid QEC declaration
invalid physical intent
invalid dialect
unknown operation where semantic resolution requires one

---

152. Boundary Tests

Boundary tests must cover:

empty quantum block
single qubit
single operation
single measurement
nested quantum blocks
nested generic quantum types
deep qualified names
large parameter lists
large operand lists
symbolic cardinalities
large source expressions
dynamic control boundaries
logical/physical transitions

---

153. Scalability Tests

Scalability tests must verify that the language does not introduce artificial limits.

At minimum test structurally:

1 qubit
2 qubits
many symbolic qubits
large symbolic registers
large operation sequences
large operation operand lists
deeply nested expressions
large quantum programs
distributed quantum structures

The exact test sizes are implementation benchmarks, not language limits.

---

154. Hard-Coding Audit

The quantum grammar must be audited for accidental constants.

Forbidden universal constructs include patterns equivalent to:

MAX_QUBITS
MAX_REGISTER_SIZE
MAX_QPU_SIZE
MAX_CONTROLS
MAX_OPERATIONS
MAX_DEPTH
MAX_PARAMETERS

Also audit for accidental target identity:

QPU0
QPU1
IBM
IonQ
Rigetti
physical_qubit_0
physical_qubit_1

as universal grammar semantics.

---

155. Allowed Constants

The prohibition on hard-coding does not prohibit source-program constants.

This is valid:

qubit[1024]

when "1024" is program semantics.

This is not valid as a language implementation restriction:

quantum_register_maximum = 1024

The distinction is:

program data
    ≠
language limitation

---

156. Compiler Resource Limits

Implementation limits must be explicitly represented as policies.

For example:

QuantumCompilerPolicy
QuantumIrLimits
CompilationResourcePolicy
ExecutionResourcePolicy

may exist downstream.

They must never redefine the semantic quantum language.

---

157. No Unsafe Rust

All Rust components implementing this specification must use safe Rust.

Required baseline:

Rust 1.97
Rust 1.97.1
Edition 2021
stable
no unsafe

Recommended module-level enforcement:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

No quantum grammar feature should require unsafe Rust.

---

158. Memory Safety

Quantum program sizes can be extremely large.

Implementations must therefore use:

checked arithmetic
fallible allocation where appropriate
explicit resource policies
streaming representations where appropriate
lazy structures where appropriate
symbolic representations where appropriate

rather than assuming that all quantum state or circuit data can be materialized eagerly.

---

159. Lazy and Symbolic Representation

Where possible, the compiler should preserve:

symbolic cardinality
symbolic parameters
lazy ranges
structural operation sequences

until a concrete representation is actually required.

This is important for scaling from tiny systems toward extremely large symbolic programs.

---

160. No Mandatory State Materialization

The language must never imply that a quantum state must be represented by a full state vector.

For example:

qubit[N]

does not imply:

allocate 2^N amplitudes

That is a simulator/backend implementation decision.

---

161. Quantum IR Scalability

The canonical quantum IR must preserve the same scalability principle.

It must not define a language-level maximum for:

qubits
operations
registers
parameters
program size

The existing quantum IR contract already explicitly distinguishes semantic scalability from concrete "QuantumIrLimits" and target/resource constraints.

---

162. Runtime Scalability

Runtime may impose actual resource limits.

Those limits must be observable as runtime/resource failures rather than being confused with language invalidity.

---

163. Diagnostics for Resource Exhaustion

A diagnostic should distinguish:

invalid quantum program

from:

valid quantum program but insufficient target resources

and:

valid quantum program but compiler resource policy exceeded

These are different failure classes.

---

164. Reproducibility

Quantum compilation should support reproducible results under a deterministic policy.

The compilation record should be able to identify:

language version
grammar version
compiler version
feature set
target capability snapshot
optimization policy
resource policy
dialects
interoperability formats

without embedding those values into the source language itself.

---

165. Provenance

Quantum compilation may preserve provenance for:

source operation
AST node
semantic operation
IR operation
optimized operation
routed operation
scheduled operation
target instruction

This enables debugging and verification.

---

166. Verification

Quantum compilation must support verification at the appropriate stages.

Verification may include:

type correctness
resource correctness
operation validity
IR invariants
semantic preservation
target capability satisfaction

The grammar itself is not a verifier.

---

167. Formal Separation

The following distinction is normative:

Grammar
    answers:
    "Can this source structure be parsed?"

AST
    answers:
    "What structure did the programmer write?"

Semantic analysis
    answers:
    "What does it mean?"

IR
    answers:
    "What canonical computation does it represent?"

Optimization
    answers:
    "What equivalent representation is preferable?"

Routing
    answers:
    "How can it map to target resources?"

Scheduling
    answers:
    "When should target operations execute?"

QEC
    answers:
    "How should quantum errors be managed?"

ZQN
    answers:
    "What fault/noise semantics apply?"

HAL
    answers:
    "What does the target provide?"

Runtime
    answers:
    "How is it executed?"

No layer should silently assume another layer's responsibilities.

---

168. Required Integration Table

Every quantum grammar file must document this contract:

Contract| Requirement
File| Exact path
Purpose| Why file exists
Owns| Exact syntax owned
Does not own| Explicit exclusions
Inputs| Grammar/token dependencies
Outputs| Parser productions
Upstream| Lexer/core/type/expression contracts
Downstream| AST/semantic consumers
AST| Exact mapping
Semantics| Exact meaning
IR| "quantum::ir" mapping
Compiler| Lowering consumers
Runtime| Runtime consumers
Cross-domain| Classical/HDL/AI/etc. integration
Diagnostics| Required errors
Positive tests| Required acceptance cases
Negative tests| Required rejection cases
Boundary tests| Edge cases
Scalability| No artificial limits
Compatibility| Version/status
Hard-coding audit| Machine assumptions
Completion| Conditions for done

---

169. Definition of Done for a Quantum Grammar File

A quantum ".g4" file is not complete when it merely parses examples.

It is complete only when:

syntax
✓

ownership
✓

lexer contract
✓

AST mapping
✓

semantic mapping
✓

IR mapping
✓

compiler integration
✓

runtime integration
✓

diagnostics
✓

positive tests
✓

negative tests
✓

boundary tests
✓

scalability tests
✓

compatibility
✓

hard-coding audit
✓

documentation
✓

has been satisfied.

---

170. Definition of Done for This Specification

"grammar/spec/quantum.md" is complete when:

✓ quantum syntax authority defined
✓ quantum semantic authority defined
✓ POCO-REAF defined
✓ scalability defined
✓ no-hard-coding rule defined
✓ quantum type contract defined
✓ operation contract defined
✓ state contract defined
✓ measurement contract defined
✓ dynamic-circuit contract defined
✓ quantum/classical contract defined
✓ QEC boundary defined
✓ ZQN boundary defined
✓ routing boundary defined
✓ scheduling boundary defined
✓ optimization boundary defined
✓ HAL boundary defined
✓ runtime boundary defined
✓ AST contract defined
✓ quantum::ir contract defined
✓ interoperability defined
✓ dialect model defined
✓ diagnostics defined
✓ compatibility defined
✓ Rust 1.97/1.97.1 defined
✓ unsafe prohibition defined
✓ testing requirements defined
✓ feature lifecycle defined
✓ ownership model defined
✓ integration contract defined

---

171. Required Downstream Files

This specification integrates with, rather than replaces, the following contracts:

grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/spec/semantics.md
grammar/spec/type-system.md
grammar/spec/resources.md
grammar/spec/effects.md
grammar/spec/compatibility.md

grammar/quantum/quantum.g4
grammar/quantum/quantum-types.g4
grammar/quantum/quantum-states.g4
grammar/quantum/qubits.g4
grammar/quantum/quantum-registers.g4
grammar/quantum/operations.g4
grammar/quantum/controlled-operations.g4
grammar/quantum/parameterized-operations.g4
grammar/quantum/measurement.g4
grammar/quantum/reset.g4
grammar/quantum/observables.g4
grammar/quantum/dynamic-circuits.g4
grammar/quantum/mid-circuit-control.g4
grammar/quantum/quantum-classical.g4
grammar/quantum/logical-qubits.g4
grammar/quantum/physical-qubits.g4
grammar/quantum/error-correction.g4
grammar/quantum/quantum-resources.g4
grammar/quantum/quantum-capabilities.g4
grammar/quantum/quantum-dialects.g4

src/frontend/ast/
src/quantum/frontend/
src/quantum/ir/
src/quantum/qec/
src/quantum/routing/
src/quantum/scheduling/
src/quantum/resilience/
src/quantum/...
src/runtime/quantum.rs
src/stdlib/quantum.rs

The exact existing filenames must be retained wherever they already provide the correct ownership boundary. New files should be created only where an actual ownership gap exists.

---

172. Required Integration Direction

The dependency direction is normative:

grammar/spec/quantum.md
        ↓
grammar/quantum/*.g4
        ↓
grammar/Zamani.g4
        ↓
lexer/parser
        ↓
frontend AST
        ↓
semantic quantum model
        ↓
quantum::ir
        ↓
optimization
        ↓
QEC / resource analysis
        ↓
routing
        ↓
scheduling
        ↓
ZQN / resilience
        ↓
HAL
        ↓
target lowering
        ↓
runtime

The reverse dependency is prohibited.

In particular:

grammar
    MUST NOT depend on
runtime

grammar
    MUST NOT depend on
hardware

grammar
    MUST NOT depend on
routing

grammar
    MUST NOT depend on
scheduling

grammar
    MUST NOT depend on
QEC implementation

grammar
    MUST NOT depend on
ZQN implementation

---

173. Canonical Quantum Principle

The final architecture is:

                 ZAMANI
                    │
        ┌───────────┼────────────┐
        │           │            │
    classical     quantum       HDL
        │           │            │
        └───────────┼────────────┘
                    │
                 hybrid
                    │
             semantic model
                    │
               canonical IR
                    │
              quantum::ir
                    │
       ┌────────────┼────────────┐
       │            │            │
   optimize       route       schedule
       │            │            │
       └────────────┼────────────┘
                    │
               QEC / ZQN
                    │
                resilience
                    │
                   HAL
                    │
            target realization
                    │
       ┌────────────┼────────────┐
       │            │            │
      CPU          GPU          QPU
       │            │            │
      FPGA      accelerator   future

The quantum grammar is therefore a portable semantic front door, not a hardware programming contract.

---

174. Final Normative Rule

The following rule governs every future quantum-language decision:

«If a quantum feature can be expressed as portable semantic intent, it must remain portable semantic intent until the compiler has enough information to choose a concrete realization.»

Therefore:

programmer specifies WHAT
compiler determines HOW
target determines WHAT IS AVAILABLE
runtime determines HOW IT EXECUTES

and never:

programmer specifies today's hardware limits
grammar freezes those limits
compiler inherits those limits forever

The purpose of the quantum language is not merely to describe today's quantum computers.

It is to provide a stable semantic language capable of expressing quantum computation across changing computational substrates, while allowing the same source program to scale from the smallest meaningful quantum computation toward arbitrarily large computations whenever the available resources and target capabilities permit it.

That is the quantum-language portion of Zamani's:

Program Once → Compile Once → Run Everywhere, Anywhere, Forever.