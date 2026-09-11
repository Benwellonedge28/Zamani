

# Zamani Type System Specification

**Path:** `grammar/spec/type-system.md`  
**Language:** Zamani  
**Status:** Normative  
**Specification layer:** Static semantics / type system  
**Implementation baseline:** Rust 1.97 / Rust 1.97.1  
**Safety requirement:** `unsafe` Rust is forbidden  
**Primary goals:** Type safety, semantic portability, resource scalability, quantum correctness, deterministic compilation, and POCO-REAF

---

## 0. Status and Authority

This document defines the normative type system of the Zamani programming language.

It is part of the canonical language specification:

```text
grammar/spec/lexical.md
        │
        ▼
grammar/spec/syntax.md
        │
        ▼
grammar/spec/semantics.md
        │
        ▼
grammar/spec/type-system.md
        │
        ▼
Canonical AST
        │
        ▼
Name / Type / Effect / Resource Resolution
        │
        ▼
Canonical IR
        │
        ▼
Optimization
        │
        ▼
Target-independent lowering
        │
        ▼
Target-specific lowering
        │
        ▼
Simulator / Emulator / Hardware / QPU

No other grammar, AST, frontend, optimizer, scheduler, simulator, hardware backend, or code generator may silently redefine the meaning of a Zamani type.

In particular:

grammar/Zamani.g4 defines syntax only.

grammar/grammar.md documents implementation/conformance information.

grammar/Zamani-Grammar.md is not an independent semantic authority.

src/lexer.rs implements lexical recognition.

src/parser.rs implements parsing.

src/ast/ and src/frontend/ast/ must represent the canonical source structure without introducing contradictory type semantics.

semantic/type checking determines whether a program is valid.

src/ir_gen.rs lowers validated semantics to canonical IR.

src/ir_verify.rs verifies canonical IR.

quantum::ir is the canonical quantum semantic boundary.

optimization operates on canonical IR.

scheduling, topology, calibration, routing, ZQN, and hardware realization occur after semantic validation.



---

1. Design Goals

The Zamani type system MUST be:

1. Sound


2. Composable


3. Target-independent


4. Resource-aware


5. Quantum-safe


6. Effect-aware


7. Deterministic where semantics permit


8. Explicit about nondeterminism


9. Scalable from tiny programs to arbitrarily large programs subject only to actual resource limits


10. Independent of physical machine dimensions


11. Compatible with POCO-REAF


12. Implementable in safe Rust


13. Extensible without breaking existing type meaning


14. Representable in canonical IR


15. Verifiable before execution



The type system MUST NOT impose artificial limits such as:

MAX_QUBITS
MAX_REGISTER_SIZE
MAX_TENSOR_RANK
MAX_VECTOR_LENGTH
MAX_MEMORY
MAX_THREADS
MAX_NODES
MAX_AGENT_COUNT
MAX_TIMELINES
MAX_RECURSION_DEPTH
MAX_GENERIC_ARITY

unless a limit is an explicit implementation/resource limitation rather than a language-semantic constant.


---

2. Core Principle

A Zamani type describes what a value or computation means, not which machine currently executes it.

For example:

Qubit

does not mean:

physical qubit #0

and:

Vector<N, Float>

does not mean:

an array whose length was chosen by the compiler

Similarly:

Tensor<A, B, C, T>

does not imply a particular accelerator, SIMD width, GPU size, or memory capacity.

The type system describes the mathematical and computational contract.

The backend determines how that contract is realized.


---

3. Type-Theoretic Model

A type-checking judgment is conceptually represented as:

Γ ; Δ ; Ε ; Κ ⊢ e : T

where:

Γ = lexical/type environment

Δ = ownership/resource environment

Ε = effect environment

Κ = capability/context environment

e = expression

T = resulting type


For statements:

Γ ; Δ ; Ε ; Κ ⊢ s ✓

For declarations:

Γ ; Δ ; Ε ; Κ ⊢ d ✓

For functions:

Γ ; Δ ; Ε ; Κ ⊢ f : FunctionType

For canonical IR:

IR ⊢ valid

A type checker MUST reject a program when any required judgment cannot be established.


---

4. Type Identity

Every semantic type has a canonical identity.

A type identity MUST NOT depend on:

source formatting,

source file path,

compiler memory address,

host pointer address,

backend implementation,

hardware topology,

register number,

process ID,

thread ID.


Equivalent semantic types MUST resolve to equivalent canonical identities.

A compiler may internally assign IDs such as:

TypeId
SymbolId
GenericId
ShapeId
RegionId
CapabilityId
EffectId
ResourceId

but those IDs are implementation identifiers rather than language-level values.


---

5. Type Categories

Zamani types are divided into the following semantic categories:

Primitive
Composite
Reference
Function
Generic
Parametric
Collection
Numeric
Mathematical
Quantum
Classical
Resource
Effect
Capability
Temporal
Agent
HDL
Distributed
Proof / Verification
Opaque
Never
Unit
Dynamic / Existential, when explicitly supported

Not every category must be exposed by a keyword.

A semantic category can be represented through existing type constructs.


---

6. Primitive Types

The core primitive types are:

Bool
Char
String
Unit
Never

and implementation-independent numeric families.

Numeric families include:

SignedInteger<W>
UnsignedInteger<W>
Float<P>

where W and P are semantic widths/precisions.

The language MUST NOT silently substitute host-dependent widths.

For example:

int

MUST NOT implicitly mean:

i32 on one platform
i64 on another platform

unless int is explicitly defined as a Zamani implementation-independent type with a fixed semantic contract.


---

7. Integer Semantics

Integer operations MUST have defined behavior.

Zamani MUST NOT rely on accidental machine overflow.

For fixed-width integers, operations that overflow MUST either:

1. produce a compile-time error when provably detectable,


2. produce an explicit runtime arithmetic error,


3. use an explicitly selected wrapping operation,


4. use an explicitly selected saturating operation,


5. or use a wider/unbounded semantic representation.



Implicit wrapping is forbidden.

For example, semantic operations may distinguish:

add_checked
add_wrapping
add_saturating

rather than giving + target-dependent overflow behavior.

The implementation MUST use checked arithmetic where required to preserve semantic correctness.


---

8. Arbitrary-Precision Semantic Values

Where Zamani semantics require mathematical integers, dimensions, resource quantities, symbolic indices, counts, or cardinalities larger than host integer limits, the semantic model is conceptually based on arbitrary natural/integer values.

An implementation MAY use:

arbitrary-precision representations,

segmented representations,

symbolic representations,

compiler-managed representations.


It MUST NOT silently wrap a semantic quantity merely because the host integer representation overflowed.

usize is an implementation indexing type, not a universal Zamani cardinality.


---

9. Boolean Types

Bool has exactly two semantic values:

true
false

Boolean operators MUST have deterministic semantics.

Logical conjunction and disjunction SHOULD be short-circuiting where expressions may have effects:

a && b
a || b

The second operand MUST NOT be evaluated when short-circuit semantics determine that it is unnecessary.


---

10. Unit and Never

10.1 Unit

Unit represents a computation that completes without producing a meaningful value.

Conceptually:

()

A function returning Unit has completed successfully without a result value.

10.2 Never

Never represents computations that do not produce a normal value.

Examples include:

panic/trap
non-returning termination
infinite computation
divergent computation

Never is a bottom-like type and may coerce into an expected type where control flow proves that no value is actually produced.


---

11. Option and Result

Zamani requires explicit representation of optional and fallible computation.

Conceptual forms:

Option<T>
Result<T, E>

An optional value is either:

Some(T)
None

A fallible computation is either:

Ok(T)
Err(E)

Recoverable runtime failure MUST NOT be represented by undefined behavior.

Compiler/runtime APIs SHOULD prefer:

Result<T, E>

over unchecked failure.


---

12. No Implicit Dynamic Escape

The existence of parameters whose types may currently be omitted from source syntax MUST NOT cause them to acquire an implicit universal dynamic type.

If source syntax permits:

fn f(x) -> T

the semantic checker MUST resolve x through:

1. explicit type information,


2. valid type inference,


3. generic constraints,


4. contextual typing,


5. or a declared dynamic/opaque type if the language explicitly provides one.



If no unique valid type can be inferred:

TYPE_INFERENCE_AMBIGUOUS

MUST be produced.

Omitted type syntax MUST NOT mean:

Any

unless Any is explicitly part of the Zamani type system.


---

13. Type Inference

Zamani MAY support local type inference.

Inference MUST be:

deterministic,

context-aware,

constraint-based,

terminating under implementation-defined compiler resource limits,

independent of machine architecture.


Inference MUST NOT change program meaning merely because the program is compiled on another target.

Public APIs SHOULD expose resolved semantic types even when source-level inference is used.


---

14. Function Types

A function type contains at minimum:

parameter types
return type
effects
resource requirements/constraints
capabilities
calling semantics

Conceptually:

Fn<(T1, T2, ..., Tn), R, Effects, Resources, Capabilities>

Function type equality MUST include all semantically relevant components.

A function that performs quantum operations is not semantically equivalent to a pure classical function merely because both return the same T.


---

15. Generic Types

Zamani supports parametric abstraction conceptually through generic types and functions.

Examples:

List<T>
Option<T>
Result<T, E>
Vector<N, T>
Matrix<M, N, T>
Tensor<Shape, T>

Generic semantics MUST be independent of whether the compiler uses:

monomorphization
dictionary passing
type erasure
specialization
interpretation
JIT compilation

These are implementation strategies, not language semantics.


---

16. Generic Constraints

Generic parameters MAY have constraints.

Conceptually:

T : Numeric
T : QuantumCompatible
T : Cloneable
T : Serializable

Constraints are semantic predicates.

A generic program MUST be valid for every type satisfying its declared constraints.

The compiler MUST NOT infer undocumented hardware assumptions from generic parameters.


---

17. Type Aliases

A type alias introduces another name for an existing semantic type.

An alias MUST NOT create a distinct runtime representation unless explicitly defined as a newtype/nominal type.

Conceptually:

type UserId = Integer

means the same semantic type as the aliased type.

Where stronger distinction is required, Zamani SHOULD use a nominal type:

type UserId = new Integer

or its canonical equivalent.


---

18. Nominal and Structural Types

Zamani MAY support both nominal and structural typing.

The distinction MUST be explicit.

Nominal identity

Two types are equal because they originate from the same declared type identity.

Structural compatibility

Two types are compatible because their required structure is equivalent.

The compiler MUST NOT accidentally treat a nominal type as structurally interchangeable merely because fields happen to match.


---

19. Records, Structs, Classes, Interfaces, Traits

Existing constructs such as:

struct
record
class
interface
trait
impl

MUST map to explicit semantic categories.

They MUST NOT create several incompatible type systems.

A trait/interface describes behavioral constraints.

A struct/record/class describes data/state structure according to its declared semantics.

An implementation relationship establishes satisfaction of a declared contract.


---

20. Recursive Types

Recursive types are valid when their representation is semantically well-founded.

Examples include:

List<T>
Tree<T>
Graph<T>
Expression

The compiler MUST NOT impose an arbitrary language-level maximum recursion depth.

Compiler resource exhaustion MAY prevent compilation, but that is an implementation resource failure, not a semantic rule.


---

21. Collection Types

Zamani collections are parameterized by element type and, where relevant, shape/cardinality.

Examples:

List<T>
Set<T>
Map<K, V>
Sequence<T>
Array<N, T>
Vector<N, T>

The semantic model MUST distinguish:

logical cardinality
physical allocation
storage representation

A collection with logical cardinality N does not require the source program to know where or how its storage is physically allocated.


---

22. Shape-Parametric Types

Mathematical and scientific types MUST be shape-parametric.

Examples:

Vector<N, T>
Matrix<M, N, T>
Tensor<Shape, T>

N, M, and Shape MAY be:

compile-time constants
generic parameters
symbolic expressions
runtime values
dependent constraints

depending on the supported type system.

The type system MUST reject mathematically invalid operations.

For example:

Matrix<M, N, A>
×
Matrix<N, K, B>

is valid.

But:

Matrix<M, N, A>
×
Matrix<K, P, B>

is valid only when a declared semantic rule establishes:

N = K


---

23. No Hard-Coded Mathematical Dimensions

The type system MUST NOT impose fixed dimensions such as:

MAX_MATRIX_SIZE
MAX_TENSOR_RANK
MAX_VECTOR_LENGTH

as language semantics.

Backend implementations MAY reject an execution because available resources are insufficient.

That is different from the language declaring the program invalid.


---

24. Numeric Type Promotion

Implicit numeric conversions MUST be limited and deterministic.

The compiler MUST NOT silently perform lossy conversions.

For example:

Integer → Float

requires a defined semantic rule.

Conversions with possible loss of:

precision,

range,

sign,

information,

quantum state,

resource identity


MUST be explicit unless a language-wide rule guarantees safety.


---

25. Floating-Point Semantics

Floating-point types MUST define:

precision,

rounding behavior,

exceptional values,

NaN behavior,

infinity behavior,

comparison semantics.


Target-specific floating-point behavior MUST NOT silently alter source semantics.

Where exact cross-target reproducibility is required, Zamani MUST provide a strict numerical execution profile.

Approximate numerical semantics MUST expose the relevant tolerance/precision contract.


---

26. Reference Types

References represent access to existing values/resources.

A reference MUST NOT imply ownership unless explicitly defined.

References must preserve the language's ownership, borrowing, lifetime, or capability guarantees.

The implementation MUST prevent:

use-after-free
double ownership
invalid aliasing
dangling references

within safe Zamani semantics.


---

27. Ownership and Resource Semantics

Zamani's resource-sensitive domains require a distinction between:

ordinary values
owned resources
borrowed resources
shared resources
linear resources
affine resources

The type system MUST be capable of representing resource ownership independently of physical machine representation.

This is particularly important for:

Qubit
QRegister
QuantumState
DeviceHandle
File
Socket
Process
Agent
Timeline
HardwareResource


---

28. Linear Types

A linear value must be used exactly according to its declared linearity contract.

Conceptually:

Linear<T>

means that the resource cannot be silently duplicated or discarded.

This is essential for quantum resources.

A compiler MUST reject an operation that would duplicate a linear resource without an explicit valid transformation.


---

29. Affine Types

An affine resource may be used at most once.

Conceptually:

Affine<T>

A resource may be consumed before scope exit.

The language MAY use affine semantics for resources where explicit destruction is not mathematically required but duplication is unsafe.


---

30. Ordinary Copyable Types

A type is copyable only when its semantic definition permits duplication.

Copying a handle to a resource MUST NOT necessarily copy the underlying resource.

For example:

Qubit

cannot be copied merely because the compiler can copy a Rust struct containing an identifier.

Semantic copyability is determined by Zamani's type system, not by the host representation.


---

31. Capability Types

Capabilities represent permission to perform an operation.

Examples:

Capability<IO>
Capability<QuantumExecution>
Capability<Hardware>
Capability<Network>
Capability<Filesystem>
Capability<Clock>
Capability<Randomness>

Possession of a capability MUST be required before performing operations that require it.

Capabilities SHOULD be least-privilege.

A type system MUST NOT assume that all code has unrestricted access to external resources.


---

32. Effect-Qualified Types

Effects describe what evaluating a value or calling a function may do.

Conceptually:

Fn<(T), R> ! {IO, MUTATION}

or:

Computation<T, Effects>

Effects may include categories such as:

IO
MUTATION
QUANTUM
HARDWARE
NETWORK
TIME
RANDOM
PROCESS
FINANCE
DISTRIBUTED
AGENT
NOISE

The exact effect vocabulary is governed by the canonical semantic specification, not by isolated grammar files.


---

33. Effect Composition

If:

f : A -> B ! E1
g : B -> C ! E2

then:

g(f(x))

has effects compatible with:

E1 ∪ E2

unless the type/effect system proves that some effect is eliminated, handled, or transformed.

An optimizer MUST NOT remove or reorder observable effects without proving semantic equivalence.


---

34. Pure Types and Functions

A pure computation:

T ! {}

must not depend on hidden external state.

Pure computations SHOULD be:

deterministic,

reproducible,

cacheable,

safely optimizable.


Purity MUST NOT be inferred merely because a function has no obvious IO syntax.


---

35. Quantum Type System

Quantum computation is a first-class semantic domain.

The quantum type system MUST be target-independent.

The following are conceptual semantic types:

Qubit
QRegister<N>
QuantumArray<Shape>
QuantumState
QuantumOperation
QuantumCircuit
Measurement<T>
ClassicalResult<T>

The exact surface syntax is specified elsewhere.


---

36. Qubit Semantics

Qubit represents a logical quantum resource.

It MUST NOT be interpreted as:

physical hardware qubit index

A Qubit has:

logical identity
ownership/resource state
quantum semantics
lifetime
effect context

Physical placement belongs to later compilation stages.


---

37. Qubit Linearity

Qubits are non-copyable semantic resources unless a specific future semantic extension explicitly establishes otherwise.

The compiler MUST reject implicit duplication.

For example, conceptually invalid:

q2 = q1
q3 = q1

if those operations imply duplication of the underlying quantum resource.

Moving a quantum handle is not quantum cloning.


---

38. No-Cloning Invariant

The type system MUST preserve the no-cloning constraint.

A legal assignment such as:

q2 = move(q1)

changes ownership.

It does not create a second quantum state.

The compiler MUST distinguish:

copying a reference/identifier

from:

duplicating quantum information


---

39. Quantum Registers

A register is a logical collection of quantum resources.

Conceptually:

QRegister<N>

does not require a physical machine with exactly N physical qubits.

The compiler/backend may realize it using:

logical qubits
physical qubits
encoded qubits
photonic modes
ion states
superconducting elements
simulated state
tensor-network representation
other compatible quantum substrate

provided semantic equivalence is preserved.


---

40. Unbounded Quantum Cardinality

There MUST be no language-level:

MAX_QUBITS

A program may express a quantum resource count bounded only by:

type constraints
algorithmic constraints
declared resource budgets
available compiler resources
available execution resources

For example, a generic program may operate over:

QRegister<N>

for arbitrary valid N.


---

41. Quantum Operations Are Semantic Operations

The type system MUST NOT define the universe of quantum operations as a fixed grammar keyword list.

Operations such as:

H
X
Y
Z
S
T
CNOT
SWAP

may exist as standard library operations or predefined semantic operations.

They MUST NOT constitute the complete language-level quantum model.

A quantum operation is conceptually:

QuantumOperation<
    Parameters,
    InputResources,
    OutputResources,
    ClassicalDependencies,
    Effects,
    Constraints
>


---

42. Gate Arity

The type system MUST validate the required number and kind of operands.

An operation's arity comes from its semantic definition.

The compiler MUST NOT assume:

q[0]
q[1]

or any other fixed operand positions.

For example:

apply(op, operands)

is semantically parameterized by the operation definition.


---

43. Controlled Operations

Controlled operations are type-checked according to:

control resources
target resources
operation compatibility
control arity
target arity

The type system MUST distinguish control and target resources where required.

The physical implementation of a controlled operation is a backend concern.


---

44. Adjoint and Inverse Operations

Where an operation has an adjoint/inverse:

Adjoint<Op>
Inverse<Op>

the type system may derive the corresponding operation when the semantic contract proves that it exists.

Operations without a valid inverse MUST NOT silently acquire one.


---

45. Parameterized Quantum Operations

Quantum operations MAY be parameterized by:

classical values
symbolic values
generic parameters
compile-time constants
runtime values

For example:

Rotation<Theta>

does not imply a particular numerical representation.

The backend may lower it into supported operations.


---

46. Quantum State Types

A quantum state represents semantic quantum information.

The compiler MUST NOT assume that a state is stored as:

2^N classical floating-point amplitudes

That representation is simulator-specific.

Possible realization strategies include:

state-vector
density-matrix
stabilizer representation
tensor network
decision diagram
analog representation
physical QPU state

The type system is independent of these choices.


---

47. Entanglement

Entanglement is a semantic property of quantum state/operations.

The type system MUST NOT infer that two independently named qubits are necessarily unentangled.

Likewise, a syntactic container such as:

Entangled<A, B>

must not be treated as a magical physical guarantee unless the semantic checker can establish the required invariant.


---

48. Superposition

Superposition is a property of a quantum state.

A type such as:

Superposition<T>

must not be interpreted as a promise that every value of T is simultaneously materialized in classical memory.

Quantum semantic types describe quantum information, not simulator storage.


---

49. Measurement

Measurement converts quantum information into classical information according to the declared measurement semantics.

Conceptually:

measure : Qubit -> Measurement<T>

A measurement may produce:

ClassicalResult<T>

and modify the quantum state.

Measurement is effectful.

The type system MUST prevent a measurement from being treated as a pure read unless the semantic model explicitly defines such behavior.


---

50. Measurement and Ownership

After measurement, the quantum resource remains subject to its semantic post-measurement state.

The type system MUST not assume that:

measure(q)

automatically creates an independent classical copy of all information contained in q.

Only the information represented by the measurement result becomes classical.


---

51. Quantum-Classical Boundary

Classical control may depend on measured values:

measurement -> classical condition -> quantum operation

The reverse direction:

unmeasured quantum state -> classical Boolean

is forbidden unless a valid measurement or semantic observation exists.

A compiler MUST preserve this boundary during optimization and lowering.


---

52. Quantum Effects

Quantum operations carry the QUANTUM semantic effect.

Physical execution may additionally require:

HARDWARE
TIMING
NOISE
CALIBRATION
IO

but these must not be silently added to the ideal source semantics.


---

53. Ideal Quantum Semantics vs Physical Execution

Zamani distinguishes:

ideal semantic computation

from:

physical realization

A program's ideal semantics describe the intended quantum computation.

ZQN, calibration, routing, scheduling, and hardware layers describe physical realization.

A physical backend may introduce approximation/noise only under an explicit execution model.


---

54. Quantum Approximation

If an implementation replaces an exact operation with an approximation, the approximation MUST be represented explicitly.

Conceptually:

ApproximationBudget<ε>

or an equivalent execution contract.

The compiler MUST NOT silently convert an exact program into an approximate one without a valid declared semantic allowance.


---

55. Quantum Equivalence

Two quantum implementations are semantically equivalent when their observable behavior is equivalent according to the declared quantum semantics.

For exact transformations:

U₁ ≡ U₂

For approximate transformations:

Distance(U₁, U₂) ≤ ε

where the distance metric and tolerance are part of the declared execution contract.


---

56. Quantum Optimization

Quantum optimizations such as:

gate cancellation
peephole optimization
T-count reduction
commutation
decomposition
resynthesis

MUST operate on the canonical quantum IR.

They MUST NOT define independent semantic QuantumGate types that compete with canonical IR.

The optimizer must preserve:

type meaning
effect meaning
resource meaning
measurement behavior
classical dependencies
declared approximation bounds


---

57. QEC and Logical Quantum Types

Quantum error correction may introduce concepts such as:

LogicalQubit
EncodedQubit
CodeBlock
Syndrome
Decoder
FaultModel

These types describe logical/physical execution relationships.

They MUST NOT hard-code a particular:

code distance
number of physical qubits
decoder
hardware topology

unless explicitly declared as a target/execution constraint.


---

58. ZQN Integration

ZQN is responsible for quantum noise semantics such as:

noise channels
faults
calibration-aware execution
noise-aware execution

ZQN MUST consume canonical quantum semantics/IR.

ZQN MUST NOT redefine:

Qubit
QuantumOperation
QuantumProgram

as incompatible parallel types.

Noise is an execution-model layer.


---

59. Hardware Integration

Hardware types describe capabilities, not source-level identity.

Conceptually:

HardwareProfile
Topology
GateCapability
TimingCapability
CalibrationProfile
PrecisionProfile
ResourceProfile

These are target descriptions.

A Zamani source program MUST NOT require a particular physical topology unless the programmer explicitly declares a target constraint.


---

60. Target Constraints

A program MAY declare requirements such as:

requires capability X
requires precision >= P
requires connectivity property C
requires quantum resource >= N

These are constraints.

They are not machine-specific source semantics.

The compiler may:

accept
transform
schedule
route
decompose
reject

according to the available target profile.


---

61. Resource Types

Resource-sensitive computations MAY use explicit resource types.

Conceptually:

Resource<T>
Budget<T>
Capacity<T>
Requirement<T>
Availability<T>

Resource values describe requirements/consumption.

The language MUST distinguish:

required resource
available resource
allocated resource
consumed resource
released resource


---

62. Resource Arithmetic

Resource requirements may be symbolic.

For example:

Requires<Qubits, N>
Requires<Memory, f(N)>
Requires<Time, g(N)>

The compiler MUST preserve symbolic resource expressions rather than prematurely replacing them with fixed constants.


---

63. Resource Failure

If an execution target cannot satisfy a resource requirement, the system MUST report an explicit resource error.

Example:

RESOURCE_UNAVAILABLE

This MUST NOT become:

undefined behavior
silent truncation
automatic semantic change


---

64. Resource Scalability

A program written as:

compute<N>()

should be capable of operating at:

N = 1
N = 10
N = 10^3
N = 10^6
...

where semantically valid and physically feasible.

The type system must not encode an arbitrary upper bound merely because one implementation has a smaller capacity.


---

65. Temporal Types

Zamani temporal constructs such as zamani and sasa MUST map to semantic temporal types rather than directly to wall-clock timestamps.

Conceptual types may include:

LogicalTime
PhysicalTime
Duration
Instant
Timeline
TemporalValue<T>

Logical time MUST be distinguished from physical machine time.


---

66. Multi-Timeline Systems

If MTS is enabled, timeline identity and temporal relationships are semantic values.

A timeline MUST NOT be represented merely as:

thread
process
CPU core

unless the backend chooses such a realization.

Temporal branching, synchronization, merging, and ordering must have explicit type/effect rules.


---

67. Temporal Safety

A value belonging to one temporal context MUST NOT be used in another temporal context unless a valid conversion/relationship is established.

Conceptually:

T@Timeline<A>

is not automatically interchangeable with:

T@Timeline<B>

unless the semantic rules establish compatibility.


---

68. Mathematics Types

Mathematical types include, where supported:

Scalar<T>
Vector<N, T>
Matrix<M, N, T>
Tensor<Shape, T>
Polynomial<T>
Symbol<T>
Distribution<T>
Function<A, B>

Mathematical constructs MUST preserve mathematical meaning independently of physical execution.


---

69. Symbolic Values

A symbolic value is not the same as a runtime value.

Conceptually:

Symbol<T>

represents a value whose exact runtime realization may not yet be known.

Symbolic expressions MUST retain enough information for:

simplification
differentiation
integration
constraint solving
optimization
code generation

where those capabilities are supported.


---

70. Mathematical Validity

The type system MUST reject mathematically invalid operations where the invalidity is statically provable.

Examples:

division by a statically proven zero
invalid matrix multiplication
invalid tensor contraction
incompatible dimensions
invalid quantum operand types

When validity cannot be statically determined, a runtime check or explicit proof/constraint may be required.


---

71. Dependent and Refinement Types

Zamani may eventually expose dependent/refinement typing.

Conceptually:

Vector<N, T>

is already a form of indexed typing.

More expressive forms may represent:

x : T where P(x)

or:

Array<N, T> where N > 0

Such features MUST integrate with the same canonical type system.

They MUST NOT introduce a second incompatible type checker.


---

72. Proof-Carrying Types

Where verification features are enabled, a type may carry a proof obligation or proof witness.

Conceptually:

Verified<T, P>

means that property P has been established according to the supported proof system.

A proof annotation MUST NOT be treated as valid merely because it appears in source.

The compiler/verifier must establish the required obligation.


---

73. Session Types

If session types are supported, communication protocols become types.

Conceptually:

Send<T, Next>
Receive<T, Next>
Choice<...>
Offer<...>
End

Protocol violations MUST be rejected statically when provable.

Distributed execution must remain independent of physical network topology.


---

74. Concurrency Types

Concurrency-safe sharing must be represented through the type/effect/resource system.

Safe Zamani code MUST NOT rely on data races.

Concurrent mutation requires an explicit semantic synchronization mechanism.

The compiler may lower concurrency to:

threads
tasks
actors
processes
distributed nodes
accelerators

without changing source-level type meaning.


---

75. Distributed Types

Distributed resources MAY be represented using types such as:

Node<T>
Channel<A, B>
Remote<T>
Replicated<T>
Partitioned<T>

A Remote<T> value does not imply a particular network technology.

The backend determines the realization.


---

76. Agent Types

Agent-oriented types MAY represent:

Agent
AgentId
AgentState
AgentCapability
AgentMessage
AgentPolicy
AgentModel

Agent behavior is effectful unless explicitly proven pure.

Model execution MUST NOT be treated as deterministic merely because it is expressed as a function.


---

77. AI / Cognitive Types

Cognitive/AI abstractions may include:

Model
Embedding
Inference
Prompt
Context
Memory
Knowledge
Policy
Decision

These must have explicit contracts for:

determinism
version
resource usage
privacy
external effects
provenance

A model call is not automatically equivalent to a mathematical pure function.


---

78. Sankofa Memory Types

Sankofa memory constructs must be represented as semantic memory resources rather than ordinary mutable variables when they have specialized persistence/temporal semantics.

Conceptual forms:

Memory<T>
MemoryRef<T>
MemorySnapshot<T>
MemoryVersion

Persistence and retrieval effects must be explicit.


---

79. HDL Types

HDL constructs may represent:

Signal<T>
Clock
Reset
Wire<T>
Register<T>
Port<T>
Module
Circuit

HDL semantics MUST remain distinct from ordinary sequential program execution.

A hardware module MUST NOT be assumed to execute on a CPU merely because it appears in a Zamani source file.


---

80. Opaque Types

An opaque type hides its representation while preserving its semantic interface.

This is required for portability.

For example:

Opaque<QuantumDevice>
Opaque<BackendHandle>
Opaque<Model>

The source program can rely on the contract without relying on representation.


---

81. Dynamic / Existential Types

If Zamani supports a dynamic or existential type, it MUST be explicit.

Conceptually:

Dynamic
Any
Exists<T>

A dynamic value MUST carry enough runtime information to support safe operations.

Dynamic typing MUST NOT appear accidentally because type inference failed.


---

82. Type Erasure

Compiler type erasure is an implementation technique.

It MUST NOT change semantic type identity.

If a type is erased during compilation, the compiler must preserve every runtime property required by the semantic contract.


---

83. Variance

For parameterized reference/container types, variance MUST be explicitly defined.

Possible relationships:

covariant
contravariant
invariant

The compiler MUST NOT infer unsafe variance merely from representation.


---

84. Subtyping

If subtyping exists, it MUST define:

reflexivity
transitivity
substitution
variance interaction
effect interaction
capability interaction
resource interaction

Subtyping MUST NOT allow a program to acquire capabilities it did not possess.


---

85. Effect Subtyping

A computation requiring fewer effects may be usable where a computation allowing more effects is expected, subject to the declared effect rules.

The reverse MUST NOT happen implicitly.

For example:

pure

cannot silently become:

IO + NETWORK + HARDWARE

because a backend wants to realize it differently.


---

86. Capability Subtyping

A capability may be weakened only when the resulting capability genuinely grants no additional authority.

The type system MUST prevent privilege escalation through subtype conversion.


---

87. Resource Subtyping

A computation requiring:

<= N resources

may satisfy a context allowing:

<= M resources

when:

N <= M

is established.

The reverse requires proof or explicit handling.


---

88. Type Compatibility vs Representation Compatibility

Two types may be semantically compatible even when their representations differ.

For example:

Qubit

may be represented by:

hardware handle
simulator index
distributed identifier
opaque runtime token

These representations do not define the language type.


---

89. Type Layout

Source-level semantic types MUST NOT depend on host memory layout unless layout is explicitly part of the type contract.

A backend may choose:

AoS
SoA
packed
aligned
distributed
compressed
symbolic

representations.

For FFI/ABI-visible types, layout becomes an explicit contract.


---

90. ABI Types

ABI-facing types must be explicitly marked/defined.

The compiler must not assume that:

Zamani Integer

equals:

Rust i32

or:

C int

unless the ABI contract explicitly specifies that relationship.


---

91. Type-Level Hardware Independence

The following MUST NOT be embedded in ordinary semantic types:

CPU family
GPU model
QPU model
physical qubit numbering
native gate set
cache size
SIMD width
RAM capacity
number of CPU cores
number of network nodes

These belong to target capability descriptions.


---

92. Target Capability Types

Backends may expose target profiles such as:

TargetCapabilities
QuantumCapabilities
MemoryCapabilities
ComputeCapabilities
NetworkCapabilities
TimingCapabilities

A compiler may use these profiles for specialization.

Specialization MUST preserve source semantics.


---

93. Compile-Once Semantics

POCO-REAF requires a distinction between:

semantic compilation

and:

physical realization

The canonical compiled artifact SHOULD preserve:

resolved types
effects
resource contracts
quantum semantics
symbol identities
debug/source mappings
semantic version
IR version
optimization provenance
target-independent metadata

The artifact can then be realized on compatible targets.

A final machine binary is necessarily target-specific.

Therefore:

> “Compile Once” means that the program's semantic representation is compiled once into a canonical portable representation, after which target realization may occur without recompiling the source semantics.




---

94. Forever Semantics

“Forever” means semantic stability, not a guarantee that every physical machine in existence will remain capable of executing an artifact.

A stable artifact must identify:

language version
type-system version
semantic version
IR version
required capabilities
resource requirements
compatibility profile

Future implementations must be able to interpret or translate the artifact according to compatibility rules.


---

95. Type Serialization

Types that appear in portable artifacts MUST have canonical serialization.

Canonical serialization MUST be:

deterministic,

versioned,

architecture-independent,

endian-independent,

independent of pointer addresses,

stable under equivalent source formatting.



---

96. Type Hashing

Canonical type representations MAY be hashed.

The hash MUST be calculated from semantic content, not:

memory address
compiler process
host architecture
random map iteration order

Type hashes can be used for:

caching
incremental compilation
IR verification
dependency identity
artifact validation


---

97. Module and Import Semantics

Types imported from modules must resolve to canonical symbols.

Two modules importing the same type must not accidentally produce distinct type identities.

The resolver must distinguish:

same semantic type
same spelling
same structural shape
different nominal type


---

98. Generic Module Boundaries

Generic types crossing module boundaries must retain their constraints.

A module cannot weaken or silently change the constraints of an exported generic type.

Public APIs MUST have fully resolvable semantic signatures.


---

99. Recursive Module Types

Mutually recursive type declarations are permitted when they can be represented safely.

Runtime initialization cycles are a separate semantic problem and MUST NOT be confused with type recursion.


---

100. Pattern Matching

Pattern matching must be type checked.

The compiler SHOULD verify:

pattern type compatibility
exhaustiveness
unreachable patterns
binding types
resource ownership
quantum resource consumption

A pattern MUST NOT duplicate a linear resource.


---

101. Quantum Pattern Matching

Quantum values MUST NOT be destructured as though they were ordinary classical data unless the operation is explicitly defined.

For example, pattern matching on:

Qubit

does not reveal its unknown quantum state.

Measurement is required where classical information is needed.


---

102. Assignment

Assignment is valid only when the destination accepts the source type and all ownership/effect/resource constraints are satisfied.

Conceptually:

T_dst <- T_src

requires:

T_src <: T_dst

or a valid conversion.

Implicit conversions MUST NOT bypass resource safety.


---

103. Function Argument Checking

Every argument must satisfy:

type compatibility
ownership compatibility
effect compatibility
capability requirements
resource requirements

The compiler MUST NOT accept an argument merely because the host representation happens to fit.


---

104. Return Type Checking

A function's return expression must satisfy the declared return type.

For generic functions, all inferred substitutions must satisfy declared constraints.

A function MUST NOT return a borrowed/resource value whose lifetime/ownership is invalid.


---

105. Closure Types

Closures capture:

values
references
resources
capabilities
effects

The resulting closure type MUST reflect these requirements.

Capturing a quantum or linear resource transfers or borrows it according to the resource rules.


---

106. Async / Deferred Computation

If asynchronous computations exist, their type must distinguish:

T
Future<T>
Task<T>

or the canonical equivalent.

An asynchronous computation does not automatically execute immediately.

Its effects remain part of its semantic contract.


---

107. Cancellation

Cancellable computations require explicit cancellation semantics.

Cancellation MUST NOT silently violate resource ownership.

Quantum/hardware resources must be returned to a valid semantic state according to the execution contract.


---

108. Errors in the Type System

Type checking MUST distinguish:

unknown type
unknown symbol
type mismatch
inference failure
constraint failure
ownership violation
linearity violation
effect violation
capability violation
resource violation
shape mismatch
quantum type violation
temporal type violation
unsupported type feature

Diagnostics SHOULD include:

error code
source span
expected type
actual type
relevant constraints
origin of inferred type
suggested correction


---

109. Error Stability

Error codes SHOULD be stable.

Human-readable diagnostics may improve without changing semantic error identity.

Tools should be able to consume machine-readable diagnostic information.


---

110. Type Checking Order

A production compiler SHOULD conceptually perform:

1. Parse
2. Build source AST
3. Resolve names
4. Establish declarations
5. Resolve imports/modules
6. Collect type constraints
7. Infer types
8. Check explicit types
9. Check generics
10. Check ownership/linearity
11. Check effects
12. Check capabilities
13. Check resource constraints
14. Check quantum semantics
15. Check temporal semantics
16. Check mathematical shapes
17. Produce typed canonical representation
18. Lower to IR
19. Verify IR

Later passes MUST NOT be responsible for repairing an invalid type system.


---

111. AST Integration

The AST must represent source structure.

It MUST NOT contain target-specific representations such as:

physical QPU gate IDs
hardware qubit numbers
scheduler slots
calibration coefficients
physical routing paths

unless such information is explicitly represented as source-level declarations.

The AST should preserve:

source spans
identifiers
type expressions
generic parameters
resource/effect annotations
quantum operation intent


---

112. src/ast/ and src/frontend/ast/

The repository must not maintain two independently authoritative semantic type systems.

If both:

src/ast/
src/frontend/ast/

exist, they must have a clearly defined relationship.

Recommended model:

source/frontend AST
        │
        ▼
canonical semantic AST
        │
        ▼
typed semantic representation
        │
        ▼
IR

One canonical semantic type representation must exist.

Compatibility wrappers are acceptable.

Semantic duplication is not.


---

113. Lexer Integration

The lexer only recognizes type-related tokens.

It does not determine whether a type is semantically valid.

For example, recognizing:

Qubit
Vector
Matrix
Tensor

does not prove that their use is valid.

The semantic type resolver performs that validation.


---

114. Parser Integration

The parser creates type-expression structure.

It MUST NOT perform target-dependent semantic checks.

For example:

Vector<N, Float>

can be parsed without knowing whether the target has enough memory.

Resource validation occurs later.


---

115. Grammar Integration

The grammar may define syntax for:

type expressions
generic parameters
constraints
function signatures
quantum declarations
effect declarations
resource annotations

but grammar productions MUST NOT encode implementation limits.

Bad:

QReg '[' INTEGER_1_TO_32 ']'

Correct concept:

QReg '[' expression ']'

with semantic validation determining validity.


---

116. grammar/grammar.md Integration

grammar/grammar.md is useful as an implementation-conformance reference.

It must not independently define the semantic meaning of types.

If the implementation currently accepts untyped parameters, the semantic system must resolve them through inference or explicitly reject ambiguous cases.


---

117. grammar/Zamani.g4 Integration

Zamani.g4 must remain syntax-focused.

Its quantum gate alternatives must not become the semantic limit of quantum computing.

Its mathematical type productions must map into the canonical type model.

Its type syntax must agree with grammar/spec/syntax.md.


---

118. Zamani-Grammar.md Integration

The broad NIMBUS/Universal grammar document may serve as:

design history
future feature catalog
extension proposal source

but features are not considered implemented merely because they appear there.

Each feature must pass:

syntax specification
semantic specification
type-system specification
implementation
tests
compatibility review

before becoming stable.


---

119. Feature Status

Every advanced type feature SHOULD have an implementation status:

STABLE
IMPLEMENTED_UNSTABLE
SPECIFIED_NOT_IMPLEMENTED
EXPERIMENTAL
RESERVED
DEPRECATED

Documentation MUST NOT describe:

SPECIFIED_NOT_IMPLEMENTED

features as currently executable.


---

120. Canonical IR Integration

After type checking, src/ir_gen.rs receives a semantically validated representation.

The IR MUST contain enough type information to verify:

operand compatibility
resource usage
effect requirements
quantum operation validity
shape constraints
control/data dependencies

The IR must not depend on source syntax spelling.


---

121. src/ir_verify.rs

IR verification MUST independently verify the invariants required for safe execution.

At minimum:

types are valid
SSA/value identities are valid where applicable
resources are valid
quantum operands are valid
linear resources are not duplicated
effects are valid
control dependencies are valid
shapes are valid
references are valid

The verifier MUST NOT trust earlier compiler passes blindly.


---

122. Quantum IR Integration

quantum::ir is the canonical semantic boundary for quantum computation.

All quantum-specific optimization and execution infrastructure must consume that canonical representation.

The following must not create competing semantic gate types:

optimization/cancellation
optimization/peephole
optimization/t_gate_reduction
scheduling
hardware
ZQN
benchmarking

They should operate on or adapt canonical IR.


---

123. Scheduling Integration

Scheduling does not change types.

A scheduler transforms:

semantic operations

into:

temporally ordered executable operations

subject to target constraints.

It MUST NOT introduce a language-level fixed qubit limit.

Resource and timing contexts determine actual feasibility.


---

124. Hardware Integration

Hardware lowering may transform:

logical operation

into:

native operation sequence

but the resulting computation must preserve its semantic type/effect/resource contract.


---

125. Benchmarking Integration

Benchmarking consumes executable/canonical representations.

Benchmark measurements MUST NOT redefine type semantics.

For example, quantum volume or performance metrics do not alter the type:

Qubit

or:

QuantumCircuit


---

126. Simulator Integration

A simulator is a backend.

The simulator may represent:

Qubit

as an index.

It must not expose that index as the semantic identity of the Zamani qubit unless explicitly requested through a backend API.


---

127. Backend-Independence Invariant

For a valid program:

Program

must describe one semantic computation.

Different backends:

CPU
GPU
TPU
FPGA
QPU
simulator
distributed system
future architecture

may produce different representations but must preserve the declared semantics.


---

128. Type-Preserving Lowering

Every lowering pass must establish:

SourceType
    ↓
IntermediateType
    ↓
TargetType

with a documented semantic relation.

A lowering pass MUST NOT silently change:

precision
ownership
effect
resource meaning
quantum behavior
measurement behavior

without an explicit transformation contract.


---

129. Optimization Soundness

An optimization is valid only if:

Semantics(before) ≈ Semantics(after)

where ≈ is the appropriate equivalence relation.

For pure classical computation this may be exact observational equivalence.

For approximate numerical/quantum computation it may be bounded equivalence.

For effectful computation, observable effects must be preserved.


---

130. Constant Folding

Constant folding is valid only when evaluation of the constant expression is semantically pure or its effects are explicitly preserved.

The compiler MUST NOT execute an external effect merely because it appears syntactically constant.


---

131. Dead Code Elimination

Dead code elimination MUST NOT remove observable effects.

A computation is dead only when:

its value is unused
AND
its effects are unobservable
AND
its resource semantics permit removal


---

132. Quantum Dead Code

An apparently unused quantum result may still affect:

measurement
entanglement
resource ownership
subsequent operations
hardware timing

Therefore quantum dead-code analysis must use canonical quantum dependency information.


---

133. Type Erasure and Optimization

A type may be erased internally only when all semantic obligations have already been checked and all required runtime information is preserved.

The compiler MUST NOT erase:

ownership
effect
capability
resource
quantum linearity

information before all passes that depend on it have completed.


---

134. Monomorphization

Generic specialization may produce target-specific implementations.

Specialization MUST preserve the generic semantic contract.

A specialization cannot introduce stronger assumptions than its declared constraints.


---

135. Resource-Aware Specialization

A compiler may specialize:

Vector<N, T>

for a target supporting an optimized representation.

The specialization is valid only if it preserves:

type semantics
shape semantics
numeric semantics
effects
resource guarantees


---

136. Compile-Time Resource Limits

Compilers inevitably consume finite resources.

Examples:

RAM
disk
CPU
compilation time
cache

These are implementation constraints.

They MUST NOT be confused with semantic type limits.

A compiler failure caused by resource exhaustion should report:

COMPILER_RESOURCE_EXHAUSTED

rather than falsely claiming:

TYPE_INVALID

when the type is semantically valid.


---

137. Runtime Resource Limits

Execution targets may impose:

memory limits
qubit limits
time limits
energy limits
network limits
storage limits

These are runtime constraints.

The type/resource system may detect them before execution when enough information is available.

Otherwise the runtime must report explicit resource failure.


---

138. No Artificial Scaling Ceiling

The implementation MUST NOT introduce arbitrary constants such as:

MAX_QUBITS = 1024
MAX_VECTOR = 1_000_000
MAX_TENSOR_RANK = 32

to make implementation easier.

If a host representation requires a limit, the implementation must either:

1. use a scalable representation,


2. detect and report implementation resource exhaustion,


3. or use symbolic/lazy representation.



It MUST NOT silently redefine the language.


---

139. Lazy Types

For enormous symbolic structures, the compiler may use lazy representations.

Examples:

LazyTensor
SymbolicShape
DeferredResource
LazyQuantumOperation

Lazy representation is an implementation strategy.

The semantic type remains the same.


---

140. Infinite / Unbounded Structures

The language may describe mathematically unbounded or lazily generated structures.

Examples:

Stream<T>
Generator<T>
InfiniteSequence<T>

These do not require infinite physical memory.

Execution proceeds according to demand/resource availability.

A type representing an unbounded mathematical domain must not imply that the entire domain is allocated.


---

141. Termination

Type correctness does not guarantee program termination.

A well-typed program may:

loop forever
wait indefinitely
perform an unbounded computation
consume available resources

Termination is a separate property.


---

142. Totality

Where a total function system is enabled, totality must be explicitly established.

Ordinary Zamani functions need not be total unless the language declares that property.


---

143. Null Safety

nil/null-like values must have a defined type.

Prefer:

Option<T>

for absence.

Implicit dereferencing of a possibly absent value MUST be rejected or explicitly checked.


---

144. Type-Safe Interoperability

Interoperation with:

C
C++
Rust
WASM
HDL
quantum SDKs
external services

must use explicit ABI/type boundaries.

Foreign values MUST NOT automatically acquire Zamani semantic types without validated conversion.


---

145. FFI Safety

The Zamani compiler/runtime implementation MUST use safe Rust.

No unsafe code may be required to implement ordinary type checking.

At crate level, implementations SHOULD use:

#![forbid(unsafe_code)]

where applicable.

Foreign interfaces must be exposed through safe abstractions.


---

146. Rust Implementation Requirements

The reference implementation is constrained to:

Rust 1.97 / Rust 1.97.1

and:

unsafe Rust: forbidden

Implementation requirements include:

checked arithmetic for semantic quantities,

explicit error handling,

no reliance on undefined behavior,

deterministic data structures where deterministic output matters,

stable canonical serialization,

explicit ownership handling,

no pointer identity as semantic identity.



---

147. Panic Policy

Recoverable compiler/user errors MUST be represented using structured errors.

Production compiler code SHOULD NOT use:

unwrap()
expect()
panic!()

for ordinary malformed source, invalid types, unavailable resources, or user-controlled input.

Panics may be reserved for internal invariants that are genuinely impossible after verified checks, but production architecture should prefer explicit error propagation.


---

148. Safe Rust Resource IDs

Semantic IDs SHOULD use safe value types such as:

newtype wrappers
integer IDs
interned symbols
arena indices

They must not depend on raw pointers.


---

149. Deterministic Type Checking

Given:

same source
same language version
same semantic environment
same dependency versions
same compiler configuration

type checking should produce the same semantic result.

Map/set iteration order MUST NOT change type identities or diagnostics.


---

150. Type Inference Determinism

Inference MUST NOT depend on:

hash-map iteration order
machine architecture
thread scheduling
memory address
random compiler decisions

unless randomness is explicitly part of an experimental optimization process whose final semantic result remains deterministic.


---

151. Diagnostics and Source Mapping

Every semantic type error should retain source locations where available.

Type information should preserve:

source span
declaration origin
inference origin
constraint origin
generic instantiation origin

This is especially important for deeply generic mathematical and quantum programs.


---

152. Type Errors Across Generic Instantiation

When a generic constraint fails, diagnostics should identify:

generic declaration
instantiation
failed constraint
actual type
required type
source location

The compiler should not emit only an internal type ID.


---

153. Type Normalization

The compiler SHOULD normalize equivalent type expressions before comparison.

Examples:

alias expansion
generic substitution
canonical shape expressions
effect normalization
constraint normalization

Normalization MUST preserve nominal distinctions.


---

154. Canonical Type Printing

A compiler/toolchain SHOULD provide canonical formatting for types.

The canonical representation should be suitable for:

diagnostics
IR
debugging
cache keys
serialization
documentation


---

155. Type Versioning

The type system must be versioned independently from implementation internals.

A breaking semantic change requires a type-system compatibility update.

Examples of breaking changes:

changing integer overflow semantics
changing Qubit linearity
changing generic variance
changing effect meaning
changing type identity rules
changing quantum measurement semantics


---

156. Compatibility

A Zamani implementation should distinguish:

source compatibility
AST compatibility
semantic compatibility
IR compatibility
ABI compatibility
execution compatibility

They are not identical.

A new compiler may accept old source while producing a newer IR version.


---

157. Backward Compatibility

Adding a new type MUST NOT change the meaning of existing programs unless an explicit language-version rule says so.

Existing names MUST NOT be silently rebound to different semantic types.


---

158. Reserved Future Types

Future domains may include:

Dependent
Linear
Affine
Session
Cognitive
Biological
Nano
Temporal
Proof
Probabilistic
Quantum
Distributed

A reserved type category is not automatically implemented.

The semantic specification may define extension points without claiming current compiler support.


---

159. Extension Model

New types must integrate through:

TypeKind
TypeIdentity
TypeConstraints
EffectRules
ResourceRules
CapabilityRules
IRRepresentation
VerificationRules
BackendContract

A new domain MUST NOT create a separate type checker.


---

160. Domain Type Registration

If the compiler supports extensible semantic domains, registration should describe:

type identity
constructors
operations
conversion rules
subtyping
effects
resource behavior
serialization
IR lowering
verification

A plugin MUST NOT redefine the meaning of an existing core type.


---

161. Plugin Safety

Plugins that participate in type checking must not bypass semantic verification.

Plugin-provided types must be treated as untrusted metadata until validated.

No plugin should be able to create an invalid canonical IR node merely by claiming that a type is valid.


---

162. Typechecker Architecture

A production implementation SHOULD separate:

type parser
type resolver
type interner
constraint solver
inference engine
generic checker
ownership checker
effect checker
resource checker
quantum type checker
shape checker
diagnostic engine

These components may share infrastructure but must have clear ownership boundaries.


---

163. Type Interner

A type interner may deduplicate equivalent types.

It must preserve:

nominal identity
generic identity
source-independent canonical identity

Interning is an optimization, not a semantic rule.


---

164. Constraint Solver

Constraints may represent:

T = U
T <: U
N = M
N > 0
Effect(A) ⊆ Effect(B)
Resource(A) <= Resource(B)
Capability(A) ⊇ Capability(B)

The solver must distinguish:

satisfied
unsatisfied
unknown

Unknown constraints may require runtime checks or explicit programmer proofs.


---

165. Constraint Failure

Constraint failure must be reported deterministically.

The compiler should prefer explaining the smallest useful unsatisfied constraint rather than producing a cascade of unrelated errors.


---

166. Shape Constraints

Shape constraints are first-class semantic constraints.

Examples:

N = M
M = K + 1
Rank(T) = 3
N > 0

Shape constraints must not be represented using fixed host arrays when symbolic dimensions can be arbitrarily large.


---

167. Quantum Resource Constraints

Quantum constraints may include:

number of logical qubits
ancilla requirements
measurement capabilities
operation compatibility
connectivity requirements
error-correction requirements
precision
timing

These constraints belong to semantic/resource checking.

Physical realization remains a backend concern.


---

168. Type and Topology

Topology is NOT part of the ordinary Qubit type.

Bad:

Qubit<HardwareA, PhysicalIndex>

as a universal source-level requirement.

Better:

Qubit

plus a target capability constraint when required.

This preserves POCO-REAF.


---

169. Type and Gate Set

Native gate availability is not a source-level type property.

The type system may require:

QuantumOperation<U>

without requiring that the target directly support U.

The backend may decompose U.


---

170. Type and Scheduling

Timing requirements may be represented as resource/effect constraints.

The type system should not encode physical scheduler slot numbers.

Scheduling occurs later.


---

171. Type and Calibration

Calibration parameters are execution metadata.

They must not alter the semantic identity of a quantum operation unless the language explicitly models calibration-dependent behavior.


---

172. Type and Noise

Noise is an execution model.

The ideal type:

QuantumOperation

does not become a different source type merely because the hardware has a noise model.

Noise-aware execution may be represented as:

ExecutionProfile<NoiseModel>

or equivalent.


---

173. Type and Measurement Noise

A noisy measurement must have an explicit execution model.

The semantic result type remains distinct from the physical error process.

For example:

Measurement<Bit>

does not mean:

perfect physical measurement

unless the execution profile guarantees it.


---

174. Type and Error Correction

Error correction should preserve logical type semantics.

A logical qubit remains semantically a quantum resource even if realized by a large encoded block.

Therefore:

LogicalQubit

must not expose its physical qubit count as an intrinsic type identity.


---

175. Type and Simulation

A simulator may provide specialized types such as:

SimulationState
AmplitudeVector
DensityMatrix
StabilizerState

These are backend/simulation types unless explicitly exposed as language-level mathematical types.


---

176. Type and Benchmarking

Benchmark-specific wrappers must not alter the underlying computation type.

For example:

Benchmark<QuantumCircuit>

contains a quantum computation but does not redefine it.


---

177. Type-Level Security

Security-sensitive values may be represented using distinct types:

Secret<T>
Public<T>
Authenticated<T>
Encrypted<T>
KeyHandle

Security properties must not be inferred solely from naming.


---

178. Confidentiality and Type Conversion

A conversion from:

Secret<T>

to:

Public<T>

must require an explicit declassification rule.

Implicit information leaks through type coercion are forbidden.


---

179. Ownership Across Security Boundaries

Moving a secret resource into another computation must preserve its security constraints.

Capabilities and effects must reflect whether an operation can:

read
write
export
serialize
log
network-transfer

sensitive values.


---

180. Serialization Types

A type is serializable only when an explicit serialization contract exists.

Serialization MUST preserve semantic meaning.

Quantum state cannot automatically become serializable merely because the compiler can serialize a host-side handle.


---

181. Persistence

Persistent values must distinguish:

value semantics
storage semantics
version semantics
identity semantics

A persisted object may have a stable logical identity while changing physical storage.


---

182. Equality

Every type must define whether equality means:

value equality
identity equality
structural equality
semantic equality
approximate equality

The compiler MUST NOT silently use host pointer equality as language equality.


---

183. Quantum Equality

Quantum state equality cannot generally be implemented as classical bitwise equality.

Quantum equality operations must follow the language's declared semantic model.

No source-level operator may imply impossible access to an unknown quantum state.


---

184. Floating Equality

Exact floating-point equality and approximate numerical equality are distinct concepts.

The language/library should provide explicit operations for:

exact representation equality
approximate equality
tolerance-based equality


---

185. Hashability

A type is hashable only if its equality and hashing semantics are compatible.

Mutable/resource-dependent values must not be silently hashed by unstable physical identity.


---

186. Typeclass / Trait Semantics

Traits/interfaces may define operations over types.

Trait satisfaction must be checked semantically.

An implementation must satisfy all required:

methods
associated types
constraints
effects
resource rules
capabilities


---

187. Associated Types

If supported:

Trait<T>::Associated

must resolve deterministically.

Associated type equality must be established before dependent operations are accepted.


---

188. Coherence

Trait/interface implementations must follow a coherence rule preventing ambiguous competing implementations unless explicit specialization rules resolve them.


---

189. Type-Level Functions

If supported, type-level functions must be pure with respect to semantic type computation.

They must not access:

filesystem
network
machine state
runtime clock
physical hardware

to determine type identity.


---

190. Compile-Time Evaluation

Compile-time evaluation may calculate:

constants
shapes
resource formulas
type-level values
proof obligations

but it must obey the same semantic safety model.

It must not silently execute arbitrary external effects.


---

191. Type-Level Resource Computation

Resource formulas may be computed at compile time where possible.

For example:

Requires<Qubits, f(N)>

can be simplified if N is known.

If N is symbolic, the requirement remains symbolic.


---

192. Resource Polymorphism

A generic algorithm should be able to express:

for any N satisfying constraints

rather than:

for N <= 1024

unless the programmer explicitly declares such a target constraint.


---

193. Type-Level Machine Independence

The same type program must have the same semantic meaning on:

1-qubit simulator
100-qubit QPU
million-element CPU system
distributed cluster
future computational substrate

provided the target satisfies the required semantic capabilities.


---

194. POCO-REAF Invariant

The type system MUST support:

Program Once
Compile Once
Run Everywhere
Run Anywhere
Run Forever

by ensuring that source-level types describe semantic contracts rather than physical implementations.

The canonical artifact must therefore preserve:

type information
resource requirements
effect requirements
capability requirements
quantum semantics
version information

without baking in a particular target.


---

195. Cross-Target Type Equivalence

If:

P

is a valid Zamani program, and targets:

T1
T2
T3

all satisfy the required capabilities, then compilation to each target should preserve the same observable semantics:

Semantics(P, T1)
≈
Semantics(P, T2)
≈
Semantics(P, T3)

within explicitly declared approximation/error bounds.


---

196. Backend Rejection

A backend may reject a program if it cannot satisfy:

type requirements
capabilities
resources
precision
timing
quantum operations
ABI requirements

Backend rejection MUST NOT imply that the source program is semantically ill-typed.


---

197. Backend Adaptation

A backend SHOULD first attempt:

decomposition
specialization
routing
scheduling
representation change
simulation
distribution

before rejecting a semantically valid program, when such transformations preserve the contract.


---

198. No Hidden Type Changes

Backend adaptation MUST NOT silently change:

Integer width semantics
floating precision contract
quantum approximation budget
measurement semantics
ownership
effects
security properties
resource guarantees


---

199. Type Metadata in IR

Canonical IR should preserve enough type metadata to support:

verification
optimization
debugging
resource analysis
backend lowering
serialization
reproducibility

Type metadata may be compacted after all dependent verification passes.


---

200. IR Type Identity

IR types must have canonical identities.

Two IR types that represent the same semantic type must normalize identically.

Target-specific lowering may introduce target types after the canonical semantic boundary.


---

201. Target IR Types

Target-specific IR may define:

PhysicalQubit
NativeGate
MachineRegister
SIMDRegister
DeviceBuffer
HardwareChannel

These types MUST NOT leak backward into canonical Zamani semantic types.


---

202. Semantic Boundary

The critical boundary is:

Zamani semantic types
        │
        ▼
canonical IR
        │
        ▼
target realization

Everything before this boundary is target-independent unless explicitly declared otherwise.


---

203. Type Safety Invariant

A well-typed program MUST NOT perform an operation for which the type system has established an incompatible:

value type
resource type
ownership state
effect
capability
shape
quantum operand
temporal context


---

204. Soundness Goal

The implementation should satisfy the following conceptual property:

> If the compiler accepts a program, every operation emitted into canonical IR satisfies the type/resource/effect invariants required by the semantic specification.



This does not guarantee:

termination
performance
absence of external failure
availability of hardware

Those are separate properties.


---

205. Progress Goal

For a well-typed program operating in a valid execution environment, each operation should either:

produce a valid next semantic state

or:

produce an explicitly defined error/effect

It must never enter undefined semantic behavior.


---

206. No Undefined Type Behavior

There is no Zamani equivalent of:

undefined type behavior

If behavior is unspecified, it must be explicitly documented as:

implementation-defined
implementation-dependent
nondeterministic
environment-dependent

and the distinction must be machine-readable where practical.


---

207. Nondeterministic Types

Nondeterminism must be represented through explicit effect/resource semantics.

Sources include:

randomness
quantum measurement
distributed scheduling
external systems
AI model inference
hardware noise

A pure function cannot secretly depend on these sources.


---

208. Reproducibility

For reproducible computation, the type/effect system should allow explicit declaration of:

seed
model version
noise profile
execution profile
numerical precision
dependency versions


---

209. Dependency Type Compatibility

Imported packages may define types.

Package type identities must be versioned and namespaced.

Two packages must not accidentally make same-named types identical merely because their names match.


---

210. Package Boundaries

Danga/package management is outside the type system itself.

However, package metadata must preserve enough information to resolve:

type identities
versions
ABI contracts
semantic compatibility
IR compatibility


---

211. Type Reflection

If runtime reflection exists, reflection must expose semantic type identity rather than host memory layout.

Reflection should be safe and deterministic.


---

212. Debugging

Debug information should preserve mappings:

source expression
semantic type
IR value
target value

When target lowering changes representation, the debugger should still be able to present the source-level type.


---

213. Testing Requirements

The type system MUST be tested independently from parser tests.

Recommended structure:

grammar/tests/
├── types/
│   ├── primitives/
│   ├── functions/
│   ├── generics/
│   ├── inference/
│   ├── ownership/
│   ├── linear/
│   ├── affine/
│   ├── effects/
│   ├── capabilities/
│   ├── resources/
│   ├── quantum/
│   ├── mathematics/
│   ├── temporal/
│   ├── distributed/
│   ├── agents/
│   ├── errors/
│   └── compatibility/


---

214. Valid Type Tests

Tests MUST cover:

valid primitive types
valid generic types
valid recursive types
valid aliases
valid nominal types
valid structural types
valid function signatures
valid inferred types
valid shape constraints
valid quantum programs
valid resource-polymorphic programs


---

215. Invalid Type Tests

Tests MUST include:

unknown types
ambiguous inference
invalid conversions
ownership violations
linear duplication
affine misuse
effect violations
capability violations
resource violations
shape mismatch
quantum operand mismatch
invalid measurement usage
temporal mismatch
invalid generic constraints


---

216. Quantum Type Tests

Quantum tests MUST include:

single logical qubit
arbitrary logical register size
generic register size
multi-qubit operations
controlled operations
measurement
classical feed-forward
linear ownership
no-cloning violations
custom operations
parameterized operations
logical/physical separation
backend decomposition

There must be no test whose correctness depends on a language constant such as:

MAX_QUBITS = 32


---

217. Scaling Tests

The test suite must test scaling properties using:

symbolic sizes
small concrete sizes
progressively larger generated sizes

The goal is to verify that the implementation has no artificial semantic ceiling.

Tests SHOULD include at least:

N = 1
N = small
N = medium
N = large where CI resources permit
N = symbolic/unresolved

rather than trying to allocate an astronomically large structure merely to demonstrate syntax.


---

218. Property Tests

Property-based testing SHOULD verify:

type normalization idempotence
constraint solver consistency
generic substitution
shape unification
effect composition
resource composition
quantum operand validation
canonical serialization
type hashing


---

219. Fuzzing

The type checker should be fuzz-tested with:

random type expressions
deep generic nesting
recursive types
large symbolic expressions
malformed constraints
quantum operation sequences

Fuzz inputs must never cause:

undefined behavior
memory unsafety
uncontrolled process termination
semantic corruption


---

220. Differential Testing

Where multiple frontends exist, equivalent source programs should produce equivalent semantic representations.

For example:

Zamani source
ANTLR parse
native parser

must converge to the same canonical semantic model.

This is one of the mechanisms for eliminating grammar authority divergence.


---

221. AST-to-Type Tests

Tests must verify:

source
→ AST
→ resolved types

without depending on target hardware.


---

222. Type-to-IR Tests

Tests must verify:

typed AST
→ canonical IR

and ensure that type information is not lost incorrectly.


---

223. IR Verification Tests

Every accepted canonical IR artifact should pass:

src/ir_verify.rs

verification.

Invalid manually constructed IR must be rejected.


---

224. Cross-Backend Tests

Equivalent programs should be lowered to multiple target profiles.

Tests should verify that:

source semantics

remain equivalent even when:

gate sets differ
topologies differ
memory layouts differ
machine sizes differ
simulation strategies differ


---

225. Regression Tests

Every discovered type-system bug must receive a regression test.

The test should capture:

source
expected type result
expected error if invalid
canonical diagnostic code


---

226. Compatibility Tests

The repository should maintain tests ensuring that:

grammar/grammar.md
grammar/Zamani.g4
grammar/spec/syntax.md
parser
AST
type checker

do not drift.

Where possible, grammar artifacts should be generated from or validated against canonical definitions.


---

227. Generated Grammar Principle

If ANTLR remains part of the toolchain, generated/parser-facing grammar artifacts SHOULD have one upstream semantic/syntax authority.

Hand-editing multiple grammars independently is strongly discouraged.


---

228. No Semantic Keywords by Accident

A new keyword must not automatically create a new semantic type.

For example, adding:

quantum
nano
zamani
sasa
memory
agent

to the lexer does not automatically define the complete semantics of the corresponding domain.

The semantic specification and implementation must establish the type.


---

229. No Type by Naming Convention

The compiler must not determine types merely from identifier spelling.

For example:

qubit1
matrixA
agentX
memory

do not acquire special semantic types because of their names.


---

230. No Hidden Coercions

The compiler must not silently coerce:

quantum → classical
secret → public
linear → copyable
logical → physical
symbolic → concrete
exact → approximate
pure → effectful

without an explicit valid semantic rule.


---

231. Type Conversion Model

Conversions should be classified:

identity conversion
widening conversion
safe structural conversion
explicit semantic conversion
lossy conversion
resource-consuming conversion
effectful conversion

The compiler should expose the distinction in diagnostics and tooling.


---

232. Conversion Effects

Some conversions have effects.

For example:

quantum measurement
device acquisition
serialization
network transfer
decryption

Such conversions cannot be treated as ordinary pure casts.


---

233. Resource-Consuming Conversions

A conversion may consume or allocate resources.

Examples:

encode logical qubit
allocate device resource
materialize symbolic tensor
deserialize large object

The resource checker must account for this.


---

234. Type-Level Allocation

Types should describe allocation requirements without forcing allocation at type-check time.

For example:

QRegister<N>

is a type-level resource requirement.

It does not mean the compiler must allocate N physical qubits while compiling.


---

235. Runtime Type Checks

Runtime type checks are permitted where static information is insufficient.

They must be explicit and produce defined results.

Examples:

dynamic cast
shape check
resource availability check
capability check


---

236. Gradual Typing

If gradual typing is eventually supported, statically known types must retain their guarantees.

Dynamic regions must be isolated and explicit.

A dynamic cast failure must be a defined runtime error.


---

237. Type Contracts

A function may declare contracts such as:

requires T satisfies C
requires N > 0
requires capability X
requires resource R
ensures property P

Contracts should integrate with the type/constraint system.


---

238. Contract Checking

Contracts may be:

statically proven
partially proven
runtime checked
externally verified

The compiler must record which mechanism established the contract.


---

239. Proof Failure

A failed proof obligation is not equivalent to a type mismatch.

The compiler should distinguish:

TYPE_ERROR
CONSTRAINT_ERROR
PROOF_OBLIGATION_UNRESOLVED


---

240. Type-System Completeness

The type system is complete with respect to the features explicitly marked stable.

Future features MUST NOT be considered part of the stable type system until:

syntax
semantics
typing rules
IR mapping
verification
tests
compatibility

are implemented.


---

241. Implementation Checklist

A production implementation should contain explicit ownership for:

TypeExpr
ResolvedType
TypeId
TypeKind
TypeEnvironment
Constraint
ConstraintSolver
GenericEnvironment
InferenceContext
EffectSet
CapabilitySet
ResourceRequirement
OwnershipState
QuantumType
ShapeType
Diagnostic

Exact Rust names may differ, but semantic responsibilities must remain separated.


---

242. Suggested Rust Representation Principles

Use safe Rust value types.

Conceptually:

struct TypeId(...);
struct SymbolId(...);
struct GenericId(...);
struct ShapeId(...);
struct EffectId(...);
struct CapabilityId(...);
struct ResourceId(...);

Prefer enums for closed semantic categories:

enum TypeKind {
    Primitive,
    Function,
    Generic,
    Composite,
    Quantum,
    Mathematical,
    Resource,
    Temporal,
    Opaque,
    Never,
}

The actual implementation may use a richer architecture.

No semantic representation should require unsafe.


---

243. Avoid Host-Size Leakage

Do not use:

usize

as the semantic representation of an arbitrary Zamani cardinality unless overflow is explicitly impossible under the applicable contract.

Host indexing may use usize.

Language-level resource quantities must use a representation appropriate to their semantic range.


---

244. Error Handling in Rust

Type-checking APIs should conceptually return:

Result<T, TypeError>

or equivalent structured results.

Errors should contain stable categories rather than free-form strings only.


---

245. Thread Safety

If the compiler performs type checking concurrently, semantic results must not depend on scheduling.

Shared compiler state must use safe synchronization primitives.

No semantic cache may depend on mutable global state without synchronization.


---

246. Incremental Compilation

Type information should support incremental compilation.

A module's semantic cache key should incorporate:

source semantic hash
dependency type hashes
language version
type-system version
relevant compiler configuration


---

247. Cache Correctness

A cached type result may be reused only if all semantic inputs are unchanged.

Hardware-specific information must not invalidate target-independent type semantics unless target constraints are part of the requested compilation stage.


---

248. Reproducible Builds

Canonical type serialization and hashing should contribute to reproducible compilation.

The same semantic input should produce the same canonical type representation.


---

249. Security of Type Metadata

Serialized type metadata must be validated before use.

Malformed metadata must result in structured rejection.

It must never cause:

memory unsafety
panic-based denial of service
arbitrary code execution
semantic bypass


---

250. Trust Boundary

Treat these as untrusted inputs:

source code
ANTLR output
serialized AST
serialized type metadata
IR artifacts
package metadata
backend capability descriptions
plugin-provided metadata

All must be validated before becoming trusted semantic state.


---

251. Canonical Type Validation

Before a type enters canonical IR, it must satisfy:

identity valid
parameters valid
constraints resolved
ownership valid
effects valid
capabilities valid
resources valid
quantum rules valid
shape rules valid
serialization valid


---

252. Type-System Invariants

The implementation MUST preserve:

INV-001  No undefined type behavior
INV-002  No implicit dynamic fallback
INV-003  No implicit lossy conversion
INV-004  No implicit quantum cloning
INV-005  No linear resource duplication
INV-006  No hidden effects
INV-007  No hidden capability escalation
INV-008  No hidden resource assumptions
INV-009  No hard-coded machine size
INV-010  No hard-coded qubit count
INV-011  No target-specific canonical types
INV-012  No competing semantic type systems
INV-013  Canonical AST/IR type identity
INV-014  Deterministic type resolution
INV-015  Safe Rust implementation
INV-016  No unsafe Rust
INV-017  Explicit approximation
INV-018  Explicit nondeterminism
INV-019  Verified canonical IR
INV-020  Stable type/version metadata


---

253. Production Definition of Done

The type system is production-ready only when:

every stable surface type has a normative definition;

every stable type has a semantic identity;

type inference is deterministic;

generic constraints are validated;

ownership/linearity is validated;

effects are validated;

capabilities are validated;

resources are validated;

quantum resources are linear/non-copyable as required;

quantum operations are target-independent;

dimensions are symbolic/parametric rather than hard-coded;

canonical IR contains sufficient type information;

src/ir_verify.rs verifies type invariants;

optimization consumes canonical IR;

scheduling does not redefine types;

hardware does not redefine types;

ZQN does not duplicate the core quantum type model;

parser and ANTLR frontends converge on canonical semantics;

grammar documents do not compete as semantic authorities;

Rust implementation uses no unsafe;

semantic errors are structured;

cross-target tests exist;

scaling tests exist;

regression tests exist;

compatibility/versioning exists.



---

254. Required Repository Integration

The final architecture should converge on:

grammar/
├── README.md
├── spec/
│   ├── lexical.md
│   ├── syntax.md
│   ├── semantics.md
│   ├── type-system.md
│   └── compatibility.md
│
├── antlr/
│   ├── ZamaniLexer.g4
│   ├── ZamaniParser.g4
│   └── domain grammars as appropriate
│
└── tests/
    ├── valid/
    ├── invalid/
    ├── types/
    ├── quantum/
    ├── mathematics/
    ├── effects/
    ├── resources/
    └── compatibility/

Implementation:

src/
├── lexer.rs
├── parser.rs
├── ast/
├── frontend/
│   └── ast/
├── semantic/
│   ├── resolver/
│   ├── types/
│   ├── inference/
│   ├── constraints/
│   ├── ownership/
│   ├── effects/
│   ├── capabilities/
│   ├── resources/
│   ├── quantum/
│   └── diagnostics/
├── ir_gen.rs
├── ir_verify.rs
└── quantum/
    └── ir/

The exact directory layout may differ, but the semantic ownership boundaries MUST remain.


---

255. Canonical Type Pipeline

The complete type pipeline is:

Source
  │
  ▼
Lexer
  │
  ▼
Parser
  │
  ▼
Source AST
  │
  ▼
Name Resolution
  │
  ▼
Type Expression Resolution
  │
  ▼
Generic Constraint Collection
  │
  ▼
Type Inference
  │
  ▼
Type Checking
  │
  ├── Ownership
  ├── Linearity
  ├── Effects
  ├── Capabilities
  ├── Resources
  ├── Shapes
  ├── Quantum Semantics
  └── Temporal/Domain Constraints
  │
  ▼
Typed Canonical AST
  │
  ▼
Canonical IR
  │
  ▼
IR Verification
  │
  ▼
Optimization
  │
  ▼
Target-Independent Lowering
  │
  ▼
Target-Specific Lowering
  │
  ▼
Execution

No later phase may be responsible for discovering a basic type error that should have been rejected earlier.


---

256. Ultimate Semantic Rule

The most important rule of the Zamani type system is:

> A type describes semantic capability and guarantees, never accidental properties of the machine currently executing the program.



Therefore:

Qubit

means logical quantum resource, not physical qubit number.

Vector<N, T>

means an N-dimensional vector, not a fixed-size machine array.

Tensor<Shape, T>

means a tensor with the declared shape, not a fixed accelerator layout.

Resource<N>

means a semantic resource requirement, not a compiler-defined maximum.

Agent

means an agent computation according to its contract, not a particular model runtime.

Memory<T>

means a memory semantic abstraction, not a particular storage device.

Timeline

means a temporal semantic abstraction, not a CPU thread.


---

257. Final POCO-REAF Guarantee

Zamani's type system is designed so that a programmer can express:

what the computation is

without having to encode:

where it runs
how many physical resources exist
which gate set exists
which CPU exists
which GPU exists
which QPU exists
which topology exists
which scheduler exists
which simulator representation exists
which memory layout exists

The intended architecture is therefore:

PROGRAM
   │
   │ semantic intent
   ▼
TYPE-SAFE ZAMANI
   │
   │ canonical semantic representation
   ▼
VERIFIED IR
   │
   ├───────────────┐
   ▼               ▼
CLASSICAL IR    QUANTUM IR
   │               │
   └───────┬───────┘
           ▼
TARGET-INDEPENDENT COMPUTATION
           │
           ▼
TARGET CAPABILITY MATCHING
           │
           ▼
OPTIMIZATION / ROUTING / DECOMPOSITION / SCHEDULING
           │
           ▼
TARGET REALIZATION
           │
     ┌─────┼─────────┬─────────┐
     ▼     ▼         ▼         ▼
    CPU    GPU       FPGA      QPU
     │     │         │         │
     └─────┴─────────┴─────────┘
                 │
                 ▼
        SAME ZAMANI SEMANTICS

The type system therefore forms one of the principal foundations for:

Program Once
Compile Once
Run Everywhere
Run Anywhere
Run Forever

while preserving:

type safety
quantum correctness
resource correctness
effect correctness
capability safety
target independence
determinism
reproducibility
scalability

subject only to the actual mathematical, compiler, and execution resources available.

This should be treated as the **normative type-system layer**, with `semantics.md` defining the broader execution meaning and `syntax.md` defining what source forms are accepted. The next architectural dependency after this is `grammar/spec/compatibility.md`, because it can formally resolve the current `grammar.md` vs `Zamani.g4` vs `Zamani-Grammar.md` authority problem and define exactly how existing source programs migrate without creating a second type system.