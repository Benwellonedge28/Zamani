Zamani Type System Specification

Path: "grammar/specification/types.md"
Language: Zamani
Status: Normative
Specification layer: Static type semantics and type-system architecture
Language/toolchain baseline: Rust 1.97 / Rust 1.97.1, Rust 2021
Safety requirement: No "unsafe" Rust
Primary goals: Type safety, semantic portability, resource scalability, quantum correctness, classical correctness, hardware/software co-design, deterministic compilation where specified, and POCO-REAF.

---

1. Purpose

This document defines the normative type-system contract for the Zamani programming language.

The type system is responsible for describing the meaning, structure, constraints, capabilities, ownership, effects, resources, and computational properties of values and computations.

It is not responsible for deciding how a program is physically realized on a particular machine.

The fundamental distinction is:

TYPE SEMANTICS
    =
what a program means
+
what values and computations are valid
+
what guarantees are required
+
what relationships must hold

NOT

TARGET REALIZATION
    =
which CPU
which GPU
which FPGA
which QPU
which physical qubit
which memory bank
which node
which register
which topology
which instruction

A Zamani type must therefore remain meaningful independently of the machine on which the program is eventually compiled or executed.

---

2. Authority and integration

The type-system authority is part of the following specification hierarchy:

grammar/specification/language.md
        │
        ▼
grammar/specification/syntax.md
        │
        ▼
grammar/specification/semantics.md
        │
        ▼
grammar/specification/types.md
        │
        ├── grammar/spec/type-system.md
        ├── grammar/types/*
        ├── grammar/resources/*
        ├── grammar/quantum/*
        ├── grammar/hardware/*
        └── grammar/classical/*
        │
        ▼
canonical source AST
        │
        ▼
structural validation
        │
        ▼
name/type/effect/resource/capability resolution
        │
        ▼
semantic type model
        │
        ▼
ZUIR / canonical semantic representation
        │
        ├── classical domain IR
        ├── quantum::ir
        ├── HDL/hardware domain IR
        ├── hybrid domain IR
        └── future domain IRs
        │
        ▼
optimization
        │
        ▼
routing / scheduling / resilience / ZQN
        │
        ▼
HAL / target realization

2.1 Authority rules

"grammar/specification/types.md" defines type meaning.

"grammar/types/*" defines source syntax for type constructs.

"grammar/Zamani.g4" composes the concrete grammar.

"grammar/grammar.md" documents implementation/conformance status.

"grammar/Zamani-Grammar.md" is an extended design/history/reference surface and MUST NOT silently redefine type semantics.

"src/frontend/ast/node/types/type_expr.rs" remains the canonical source-level AST representation.

The specialized AST modules:

primitive
named
generic
array
slice
tuple
function
reference
pointer
optional
result
never
unit
bounds

remain focused façades/APIs over the canonical representation rather than independent competing type systems.

Semantic analysis resolves source-level "TypeExpr" into the semantic type model.

ZUIR and domain IRs represent validated semantics.

"quantum::ir" remains the canonical quantum semantic boundary.

The type system MUST NOT create another quantum IR.

---

3. Core type-system principle

A Zamani type describes semantic identity and valid use, not physical implementation.

For example:

Qubit

means a quantum resource abstraction.

It does not mean:

physical qubit 0
physical qubit 1
QPU 0
device 7
a particular topology
a particular vendor
a fixed number of physical qubits

Similarly:

Vector<N, Float>

does not mean:

a vector limited to today's SIMD width

and:

Memory<T, N>

does not mean:

a particular physical RAM bank

The backend may map these semantic types to physical resources, but such mapping is outside the source type's identity.

---

4. POCO-REAF requirement

Zamani types MUST support:

«Program Once, Compile Once, Run Everywhere, Anywhere, Forever»

subject to the semantic requirements of the program and the resources/capabilities actually available at execution.

The type system MUST NOT contain universal language-level limits such as:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_MEMORY
MAX_STORAGE
MAX_NODES
MAX_PROCESSES
MAX_AGENTS
MAX_TIMELINES
MAX_TENSOR_RANK
MAX_VECTOR_LENGTH
MAX_MATRIX_SIZE
MAX_REGISTER_WIDTH
MAX_GENERIC_ARITY
MAX_TUPLE_ARITY
MAX_FUNCTION_PARAMETERS
MAX_TYPE_DEPTH

unless a value is genuinely part of the language's semantic definition rather than an implementation capacity.

---

5. Meaning of "unbounded"

"Unbounded" in Zamani means:

«No artificial finite ceiling is imposed by the language semantics where the mathematical or abstract domain itself has no such ceiling.»

It does not mean that physical computers have infinite memory or infinite execution time.

A program remains subject to:

available memory
available compute
available quantum resources
available storage
available communication capacity
execution time
compiler resource policies
runtime resource policies
target capabilities
physical laws

These are resource/execution constraints, not arbitrary language type limits.

For example:

Vector<N, T>

must remain legal for any semantically representable "N".

A particular compilation may fail because the selected target cannot satisfy:

N

but that does not make the type itself invalid.

---

6. Type judgments

The conceptual typing judgment is:

Γ ; Δ ; Ε ; Κ ; R ⊢ e : T

where:

Γ = lexical/name/type environment
Δ = ownership/resource environment
Ε = effect environment
Κ = capability environment
R = refinement/constraint environment
e = expression
T = resulting type

Statements use:

Γ ; Δ ; Ε ; Κ ; R ⊢ s ✓

Declarations use:

Γ ; Δ ; Ε ; Κ ; R ⊢ d ✓

Functions use:

Γ ; Δ ; Ε ; Κ ; R ⊢ f : F

where "F" is a complete function type.

A program is statically valid only when every required typing judgment can be established.

---

7. Type identity

Every semantic type has a canonical identity.

Type identity MUST NOT depend on:

- source formatting;
- whitespace;
- source file path;
- parser node address;
- compiler memory address;
- pointer address;
- process identifier;
- thread identifier;
- machine identifier;
- physical device identifier;
- hardware topology;
- backend implementation;
- vendor;
- compilation order.

The implementation MAY assign internal identifiers such as:

TypeId
TypeParameterId
SymbolId
GenericId
ShapeId
RegionId
LifetimeId
CapabilityId
EffectId
ResourceId

but these are implementation identities, not portable Zamani values.

Canonical semantic equality MUST be independent of such internal identifiers.

---

8. Type categories

Zamani supports the following semantic type categories.

8.1 Fundamental categories

Unit
Never
Boolean
Character
String
Numeric
Named
Tuple
Record
Struct
Enum
Union/Sum
Reference
Pointer
Function
Closure
Collection
Array
Slice/View
Generic
Parametric
Dependent
Refined
Opaque
Existential
Dynamic

8.2 Computational categories

Classical
Numeric
Symbolic
Tensor
Data
Stream
Temporal
Distributed
Concurrent
Agent
AI/ML

8.3 Resource categories

Owned
Borrowed
Shared
Linear
Affine
Capability
Resource
Hardware
Memory
Communication
Device
Process
Service
Timeline

8.4 Quantum categories

Qubit
QuantumRegister
QuantumState
QuantumReference
LogicalQubit
QuantumResource
Observable
QuantumChannel
Measurement
QuantumOperation
QuantumCircuit

8.5 Hardware/HDL categories

Signal
Net
Register
Port
Interface
Clock
Reset
Memory
HardwareModule
HardwareResource
TimingDomain

These categories describe semantics. They do not require separate incompatible type systems.

---

9. Canonical source representation

There MUST be exactly one authoritative source-level type representation.

The existing canonical:

TypeExpr

is the source-level representation.

Specialized structures such as:

PrimitiveType
NamedType
GenericType
ArrayType
SliceType
TupleType
FunctionType
ReferenceType
PointerType
OptionalType
ResultType
NeverType
UnitType
TypeBound

are APIs/facades over that canonical representation.

They MUST NOT introduce a second incompatible AST hierarchy.

The frontend AST already defines this architectural boundary and must remain source-level rather than hardware/backend-specific.

---

10. Primitive types

The language provides semantic primitive types including:

Bool
Char
String
Unit
Never

and numeric families.

Primitive syntax is defined by "grammar/types/types.g4" and the canonical lexer.

The specification defines semantics, not lexer-token implementation.

---

11. Boolean type

"Bool" has exactly two semantic values:

true
false

Boolean operations MUST be deterministic.

Logical operators MUST have defined evaluation order.

Where an operator is short-circuiting, the second operand MUST NOT be evaluated when its value cannot affect the result.

This behavior MUST remain independent of target architecture.

---

12. Unit type

"Unit" represents a computation with no meaningful result value.

Conceptually:

()

Unit is distinct from:

Never

and from a one-element tuple.

A function returning "Unit" completed normally and produced no meaningful result value.

---

13. Never type

"Never" represents an expression that cannot produce a normal value.

Examples include:

non-returning termination
explicit trap
unrecoverable termination
divergent computation

"Never" may participate in control-flow typing where an expression terminates control flow.

The type system MUST NOT require a value after a path whose type is "Never".

Compiler recursion or resource exhaustion MUST NOT automatically be modeled as the language-level "Never" type.

---

14. Option type

Optionality is represented semantically by:

Option<T>

with values equivalent to:

Some(T)
None

The postfix syntax:

T?

may be a source-level shorthand where defined by the canonical grammar.

The semantic representation MUST be canonicalized so that:

Option<T>

and its permitted syntactic shorthand have identical meaning.

"None" MUST NOT be represented as an invalid pointer or uninitialized memory.

---

15. Result type

Fallible computation is represented by:

Result<T, E>

with:

Ok(T)
Err(E)

Recoverable failures MUST be represented explicitly.

Undefined behavior MUST NOT be used as the semantic representation of ordinary failure.

Compiler/runtime APIs should use explicit result semantics.

---

16. Integer types

Zamani distinguishes semantic integer families from implementation indexing types.

Conceptual families include:

SignedInteger<W>
UnsignedInteger<W>

and arbitrary-precision mathematical integer domains where required.

Examples may include:

i8
i16
i32
i64
i128

u8
u16
u32
u64
u128

when these are defined by the language.

The type system MUST define their semantics independently of the host machine.

---

17. Integer overflow

Integer overflow MUST NOT silently acquire target-dependent meaning.

For fixed-width integer operations, overflow semantics must be explicit and defined.

Permitted semantic classes include:

checked
wrapping
saturating
trapping
widening
arbitrary-precision

The ordinary arithmetic operators MUST have one deterministic language-defined interpretation.

An implementation MUST NOT silently change overflow behavior because a different target architecture was selected.

---

18. Arbitrary-precision semantic integers

Some Zamani domains require integer values that exceed ordinary machine widths.

These include:

mathematical quantities
symbolic dimensions
resource quantities
cardinalities
large indices
exact counters
cryptographic integers
scientific values
compile-time values
proof parameters

The semantic model MUST support arbitrary mathematical magnitude where the relevant type requires it.

An implementation may use:

big integers
segmented representation
symbolic expressions
canonicalized expressions
compiler-managed representations

but MUST NOT silently wrap a semantic value merely because a host integer overflowed.

"usize" is an implementation indexing type and MUST NOT define the maximum semantic size of a Zamani program.

---

19. Floating-point types

Floating-point types MUST define:

- precision;
- exponent/range semantics;
- rounding;
- NaN behavior;
- infinity behavior;
- comparison behavior;
- conversion behavior.

A target MUST NOT silently change source-level numerical meaning.

Where strict reproducibility is required, Zamani execution profiles may require deterministic numerical semantics.

Where approximation is intentional, the approximation contract must be represented explicitly through types, constraints, effects, or numerical policy.

---

20. Exact numeric types

Zamani may support exact numeric types for domains requiring mathematically exact values.

Examples include:

Integer
Rational
ExactDecimal
Complex<Exact>

The type system must distinguish exact semantics from approximate floating-point semantics.

An exact type MUST NOT silently degrade into approximate representation unless an explicit conversion or declared policy permits it.

---

21. Complex numbers

Complex numbers are semantic numeric values:

Complex<T>

where "T" is a valid scalar numeric type.

For quantum computing, complex amplitudes MUST remain semantic values and MUST NOT imply a particular floating-point implementation.

---

22. Numeric promotion

Implicit conversions must be deterministic.

Conversions that may lose:

range
precision
sign
information
exactness
quantum semantics
resource identity

must be explicit unless a language-wide rule proves that the conversion is lossless.

The compiler MUST NOT choose different implicit conversions merely because a target has different native widths.

---

23. Character and string types

"Char" represents one Zamani character value according to the language's Unicode/lexical specification.

"String" represents a sequence of character data.

String representation is implementation-defined unless a stronger representation guarantee is explicitly part of a type.

Source-level string types MUST NOT expose:

pointer width
allocator address
buffer capacity
machine encoding

as part of semantic type identity.

---

24. Named types

A named type is resolved through the language's namespace and type environment.

Examples:

User
Matrix
quantum::State
hardware::Resource

The parser preserves the name.

Semantic resolution determines whether it denotes:

- a nominal type;
- a type alias;
- a type parameter;
- a generic constructor;
- a resource type;
- a capability type;
- a domain type;
- an imported type;
- an opaque type.

The parser MUST NOT decide semantic identity from spelling alone.

---

25. Type aliases

A type alias introduces an alternate name for an existing type.

Conceptually:

type UserId = Integer

does not create a new nominal identity unless explicitly declared as a nominal/newtype construct.

Aliases MUST preserve semantic equivalence.

---

26. Nominal types

A nominal type has identity determined by declaration identity.

Two nominal types are not interchangeable merely because they have identical structure.

This is required for concepts such as:

UserId
Meters
Seconds
QubitHandle
DeviceHandle
LogicalQubit
PhysicalQubit

where accidental structural equivalence would be unsafe or semantically incorrect.

---

27. Structural types

Structural compatibility may be used for explicitly structural constructs.

For example, a structural record contract can be satisfied by any type possessing the required members and semantics.

Structural compatibility MUST NOT silently override nominal identity.

---

28. Product types

Product types combine multiple typed components.

Examples:

(T1, T2)
Record
Struct

The semantic identity includes:

- component count;
- component order where ordered;
- component names where named;
- component types;
- relevant attributes;
- relevant ownership/resource semantics.

There is no language-level tuple-arity ceiling.

---

29. Sum types

Zamani may represent alternatives through enums, tagged unions, or equivalent sum types.

A sum type contains one of a finite set of declared alternatives.

Example:

enum ResultState<T, E> {
    Success(T),
    Failure(E)
}

The number of declared variants is determined by the program rather than by a universal machine limit.

---

30. Recursive types

Recursive types are valid where their semantic representation is well-founded.

Examples:

List<T>
Tree<T>
Graph<T>
Expression
AST

The language MUST NOT define an arbitrary maximum recursion depth as part of type semantics.

Compiler implementation limits may exist as configurable resource policies.

Such limits MUST produce explicit resource diagnostics rather than silently changing type meaning.

---

31. Generic types

Zamani supports parametric generic types.

Examples:

List<T>
Map<K, V>
Option<T>
Result<T, E>
Vector<N, T>
Matrix<M, N, T>
Tensor<Shape, T>

Generic semantics MUST be independent of implementation strategy.

A compiler may use:

monomorphization
specialization
dictionary passing
type erasure
JIT
interpretation

without changing program meaning.

---

32. Generic parameters

Generic parameters may represent:

types
values
shapes
lifetimes
effects
capabilities
resources
constraints

where explicitly supported.

Each parameter has a declared kind.

A type parameter MUST NOT be confused with a value parameter.

A value parameter MUST NOT silently become a machine-specific constant.

---

33. Generic constraints

Generic constraints express semantic requirements.

Examples:

T : Numeric
T : Serializable
T : Cloneable
T : QuantumCompatible
T : Ordered

A generic implementation must be valid for every argument satisfying its declared constraints.

The compiler MUST NOT infer undocumented hardware requirements from generic constraints.

---

34. Trait and interface constraints

Traits/interfaces describe capabilities or behavioral contracts.

A type satisfying a trait/interface must satisfy all required semantic members and laws defined by that contract.

Trait satisfaction is a semantic concern.

The grammar only represents the declaration/use of the constraint.

---

35. Associated types

Traits/interfaces may define associated types.

Conceptually:

trait Iterator {
    type Item;
}

Associated types are resolved by semantic analysis.

Associated type resolution MUST be deterministic.

Associated types MUST NOT embed backend-specific machine representation.

---

36. Generic specialization

Specialization may be used by the compiler for optimization.

Specialization MUST NOT change the externally visible semantics of a valid program.

A specialized implementation must preserve:

type correctness
effect semantics
resource semantics
ownership
observable behavior
determinism guarantees

---

37. Type inference

Zamani may support type inference.

Inference MUST be:

- deterministic;
- context-sensitive where specified;
- constraint-based;
- target-independent;
- semantically reproducible;
- bounded by explicit compiler resource policy rather than language type rules.

Inference MUST NOT inspect:

CPU model
GPU model
QPU model
physical memory
physical qubit count
hardware topology
vendor SDK
runtime load

to determine the source type.

If inference cannot establish one valid type:

TYPE_INFERENCE_AMBIGUOUS

or an equivalent stable diagnostic MUST be produced.

---

38. No implicit universal dynamic type

Omitted type information MUST NOT automatically mean:

Any
Dynamic
Unknown

unless the language explicitly defines that behavior for the particular construct.

Inference may use:

context
constraints
expected type
generic bounds
declared defaults

but unresolved ambiguity must remain an error.

---

39. Dynamic and existential types

Zamani may provide explicit dynamic/opaque/existential types.

These must be explicit semantic choices.

A dynamic type represents a deliberate loss of compile-time static knowledge.

It MUST NOT be introduced merely because the compiler failed to infer a type.

---

40. Function types

A function type consists of all semantically relevant properties:

parameter types
return type
generic parameters
effects
resource requirements
capabilities
ownership/linearity
calling semantics

Conceptually:

Fn<Parameters, Return, Effects, Requirements, Capabilities>

Two functions are not semantically equivalent merely because their parameter and return types match if their effects/resources/capabilities differ in ways visible to the language.

---

41. Closures

A closure has:

parameter types
return type
captured values
capture modes
effects
resource requirements
capabilities

Capture semantics MUST preserve ownership and lifetime guarantees.

A closure MUST NOT secretly capture a physical hardware object merely because it was compiled for a target.

---

42. Function parameter scalability

The language MUST NOT impose a fixed semantic limit on the number of function parameters.

Compiler implementation may apply resource policies for pathological input, but those policies are not part of type meaning.

---

43. Reference types

References represent access to another value/resource.

A reference does not imply ownership unless explicitly declared.

The type system must distinguish:

owned
borrowed
shared
mutable
immutable
linear
affine

where applicable.

Reference validity must be statically or semantically established before unsafe behavior could occur.

---

44. Lifetimes

Lifetime annotations describe validity relationships between references and owned resources.

Lifetime identity is semantic, not a machine timestamp.

A lifetime MUST NOT be represented as:

pointer address
process ID
thread ID
wall-clock timestamp

unless those are explicitly separate values in the program.

Lifetime analysis belongs to semantic analysis.

---

45. Ownership

Ownership describes which computation is responsible for a resource.

The type system must support ownership-sensitive resources such as:

File
Socket
Process
DeviceHandle
MemoryRegion
Qubit
QuantumRegister
Timeline
Agent
Service
HardwareResource

Ownership semantics MUST remain independent of physical allocation.

---

46. Linear types

A linear value must be consumed according to its declared linearity contract.

Conceptually:

Linear<T>

means that the value cannot be freely duplicated.

This is particularly important for:

Qubit
exclusive device resource
unique capability
exclusive resource handle

A program that duplicates a linear value without a valid semantic operation MUST be rejected.

---

47. Affine types

An affine value may be consumed zero or one time.

Conceptually:

Affine<T>

Affine semantics are appropriate for resources where duplication is prohibited but explicit destruction/consumption is not mandatory.

---

48. Copyable types

A type is copyable only when its semantic contract permits duplication.

Copyability MUST NOT be inferred solely from representation.

A type containing:

Qubit
exclusive resource
linear capability
unique device handle

must not automatically become copyable merely because its representation fits in a machine register.

---

49. Move semantics

Moving transfers ownership without duplicating the semantic resource.

After a moved value is consumed, its previous owner cannot use it unless the type explicitly permits reuse.

The semantic rule must remain independent of physical memory movement.

---

50. Collections

Zamani supports parameterized collections.

Examples:

List<T>
Set<T>
Map<K, V>
Sequence<T>
Array<N, T>
Vector<N, T>
Stream<T>

The type system distinguishes:

logical cardinality
physical allocation
storage layout

These are not interchangeable concepts.

---

51. Arrays

A sized array may be represented conceptually as:

Array<N, T>

or the canonical source syntax defined in "grammar/types/types.g4".

"N" is semantic cardinality.

The language MUST NOT define a universal maximum array length.

---

52. Slices and views

A slice/view represents access to part of a collection without necessarily owning the underlying storage.

Its type semantics include:

element type
access mode
lifetime/region
shape/cardinality information when known

A slice does not imply a particular pointer width or memory layout.

---

53. Shape-parametric types

Scientific and computational types must support symbolic shape.

Examples:

Vector<N, T>
Matrix<M, N, T>
Tensor<Shape, T>

Shape values may be:

compile-time constants
generic parameters
symbolic expressions
dependent values
runtime-known values

according to the supported semantic level.

---

54. Shape equality

Operations requiring compatible shapes MUST establish the required shape relationships.

For example:

Matrix<M, N, A>
×
Matrix<N, K, B>

is valid when the inner dimensions are semantically equal.

The type system must reject:

Matrix<M, N, A>
×
Matrix<K, P, B>

when no constraint establishes:

N = K

---

55. Symbolic dimensions

Symbolic dimensions must not be prematurely lowered to host integers.

For example:

N
M
K
Shape
Batch
SequenceLength

may remain symbolic until sufficient information exists.

This is essential for POCO-REAF.

---

56. Dependent/value-parameterized types

Zamani may express types parameterized by values.

Examples:

Vector<N, T>
Matrix<M, N, T>
Tensor<Shape, T>
Array<T, N>
QuantumRegister<N>

The parser preserves the value expression.

Semantic analysis determines whether the expression is valid.

The parser MUST NOT evaluate the expression.

---

57. Type-level arithmetic

Type-level value expressions may support operations such as:

+
-
*
/
%
shift
bitwise operations

where permitted.

Type-level arithmetic must have deterministic semantics.

Overflow in type-level arithmetic must not silently wrap.

If a type-level quantity cannot be represented or proven valid, semantic analysis must produce an explicit diagnostic.

---

58. Refinement types

Zamani may express types constrained by predicates.

Conceptually:

x : Int where x > 0

or equivalent syntax.

A refinement is valid only when its predicate is established according to the language's verification model.

The type system must distinguish:

syntactically declared predicate
provably established predicate
runtime-checked predicate
unproven predicate

---

59. Refinement checking

Refinement checking may use:

constant evaluation
symbolic reasoning
constraint solving
static proofs
runtime checks

The chosen method is an implementation detail provided the language-visible semantics remain equivalent.

Failure to prove a required refinement must produce an explicit diagnostic or require an explicit runtime check.

---

60. Units of measure

Zamani may provide semantic units for physical and mathematical quantities.

Examples:

Length
Time
Mass
Energy
Frequency
Voltage
Current
Temperature
Angle

Units must be represented semantically rather than by comments or naming conventions.

Invalid dimensional operations must be rejected.

For example:

Length + Time

is invalid unless an explicit conversion or domain rule exists.

---

61. Physical quantities and hardware

A type such as:

Frequency

must not imply:

CPU clock
GPU clock
QPU clock

unless explicitly constrained by a hardware/resource context.

The type system describes the quantity.

Hardware selection remains downstream.

---

62. Resource types

Resource types represent semantically consumable or constrained resources.

Examples:

Memory
Compute
Storage
Bandwidth
Energy
TimeBudget
QubitResource
QuantumMemory
Accelerator
Device
Node
Channel

A resource type describes a resource contract.

It does not identify a physical resource unless the program explicitly requests target-specific realization.

---

63. Resource quantities

Resource quantities must be parameterized or symbolic where appropriate.

For example:

Memory<Required>
Qubits<N>
Bandwidth<B>
Compute<C>

must remain target-independent.

A target may satisfy or fail the requirement later.

---

64. Requirements versus realizations

The type system must preserve the distinction between:

requirement
constraint
capability
preference
hint
realization

For example:

requires Qubits<N>

does not mean:

use physical qubit IDs 0 through N-1

Likewise:

requires capability("quantum.mid_circuit_measurement")

does not select a particular QPU.

---

65. Capability types

Capabilities represent semantic abilities.

Examples:

Numeric
Parallel
Quantum
Measurement
DynamicCircuit
TensorCompute
HDL
Network
Storage
Persistence
SecureCompute

A capability is not a device identifier.

Capability satisfaction is resolved against the compilation/execution environment.

---

66. Capability polymorphism

Generic code may require capabilities.

Conceptually:

fn compute<T : TensorCompute>(x: T) -> ...

The implementation must remain valid for every implementation satisfying the declared capability contract.

A compiler must not silently specialize the meaning of the function to one vendor.

---

67. Effect types

Effects describe observable computational behavior.

Examples include:

IO
Mutation
Async
Concurrency
Network
Storage
Randomness
Time
Quantum
Measurement
Hardware
UnsafeExternal

Effects are semantic information.

They are not equivalent to implementation APIs.

---

68. Quantum effects

Quantum computation may be represented through types plus effects.

A quantum operation may have:

Quantum

and, where appropriate:

Measurement

effects.

The type system must distinguish quantum state manipulation from ordinary classical data manipulation.

---

69. Quantum type principles

Quantum types must be:

- target-independent;
- resource-aware;
- compatible with linear/affine semantics;
- compatible with dynamic circuits;
- compatible with logical and physical realization;
- compatible with QEC;
- compatible with quantum::ir;
- independent of vendor gate sets.

The source type system must not encode a fixed gate inventory.

---

70. Qubit

"Qubit" represents a quantum resource abstraction.

It does not identify:

physical qubit index
QPU
topology
coupling map
calibration
pulse
vendor
device address

These are downstream concerns.

---

71. Quantum registers

A quantum register may be parameterized by symbolic cardinality.

Conceptually:

QuantumRegister<N>

The language MUST NOT define a maximum "N".

The compiler/runtime may reject execution if no target can satisfy the required resource.

---

72. Logical and physical quantum resources

The semantic model must distinguish:

logical qubit
physical qubit

where that distinction is meaningful.

A logical qubit is part of program semantics.

A physical qubit is part of target realization.

The source type system MUST NOT require programmers to identify physical qubit numbers for portable programs.

---

73. Quantum state

A quantum state type represents semantic quantum state.

It must not require the source program to expose:

state-vector storage size
amplitude array layout
GPU memory
QPU memory
physical qubit layout

Those are implementation details.

---

74. Quantum operations

Quantum operations are semantic operations over quantum types.

The type system must support:

single-target operations
multi-target operations
parameterized operations
controlled operations
adjoint operations
measurement
reset
dynamic control
custom operations
logical operations

without enumerating a universal fixed gate list.

The existing architecture's canonical quantum semantic destination remains "quantum::ir".

---

75. Measurement

Measurement changes the semantic information available to the classical side.

The type/effect system must preserve that boundary.

A measurement operation may produce:

classical result
measurement effect
quantum-state effect

according to the semantic model.

Measurement must not be treated as an ordinary pure classical function.

---

76. Classical feed-forward

Hybrid quantum/classical code must allow measured results to influence later classical or quantum operations.

The type system must preserve:

quantum value
classical value
measurement result
control dependency

as distinct semantic categories.

---

77. Quantum linearity

Quantum resources must not be copied using ordinary unrestricted value semantics.

For example, a program cannot create two independent owners of the same semantic qubit resource by ordinary assignment.

Valid quantum transformations must explicitly define the resulting resource semantics.

---

78. Quantum entanglement

Entanglement is a semantic relationship between quantum resources.

The type system MUST NOT assume that each qubit can always be independently represented.

A backend may use:

state vector
tensor network
stabilizer representation
measurement-based representation
analog representation
physical QPU state

without changing source-level type meaning.

---

79. Quantum error correction

The type system may express requirements or capabilities related to error correction.

Examples:

FaultTolerant<T>
LogicalQubit
ErrorCorrected<T>

where such types are formally defined.

The type system does NOT implement QEC.

QEC remains a downstream subsystem.

---

80. ZQN integration

ZQN represents fault/noise semantics downstream of source typing.

A type may express requirements such as:

requires fault tolerance
requires noise bound
requires reliability

but must not encode ZQN's internal fault model.

---

81. Quantum routing and scheduling

Routing and scheduling are not type checking.

A type can require:

N logical qubits
capability X
timing property Y

but the source type MUST NOT encode:

physical qubit 7
edge (2,3)
scheduler slot 11

unless explicitly using a target-specific dialect.

---

82. Classical computing types

Classical types cover:

scalars
arrays
vectors
matrices
tensors
records
streams
symbolic values
numeric values
scientific values
control values

They remain target-independent.

CPU/GPU/FPGA selection is downstream.

---

83. Tensor types

A tensor type may be represented conceptually as:

Tensor<Shape, Element>

where "Shape" may contain arbitrary symbolic dimensions.

The type system must not impose a fixed maximum tensor rank.

---

84. Tensor layout

Logical tensor shape is distinct from physical layout.

The type system describes:

shape
element type
semantic indexing
optional layout contract

but not necessarily:

GPU warp layout
SIMD register width
cache line
memory bank
vendor tensor core

Those belong downstream.

---

85. AI/ML types

AI/ML types may include:

Tensor
Dataset
Model
Parameter
Gradient
OptimizerState
Inference
TrainingState
Agent

These are semantic categories.

The type system must not make:

CUDA
ROCm
TensorRT
vendor accelerator

part of core type identity.

---

86. Data types

Data-oriented types include:

Dataset<T>
Table<Schema>
Record<Schema>
Stream<T>
Column<T>
Batch<T>

Schema constraints are semantic.

Physical database/storage selection is not part of source type identity.

---

87. Distributed types

Distributed computation may introduce semantic types for:

Node
Process
Actor
Service
Channel
Message
Replica
Partition
Cluster

The type system MUST NOT impose a fixed number of nodes.

A distributed type expresses semantics and contracts.

Deployment determines actual realization.

---

88. Concurrency types

Concurrency-related types may include:

Task<T>
Future<T>
Promise<T>
Channel<T>
Actor<T>
Shared<T>

The type system must define ownership and synchronization semantics.

The number of tasks/threads/cores is not a type-system constant.

---

89. Parallelism

A parallel computation expresses semantic parallelism.

The type system must not require:

8 threads
16 cores
32 workers

unless those are explicit program requirements represented through resource constraints.

The runtime may choose:

one worker
many workers
CPU
GPU
FPGA
distributed cluster

subject to semantic equivalence.

---

90. HDL types

Hardware-description types include semantic categories such as:

Signal
Net
Register
Port
Clock
Reset
Interface
Memory
HardwareModule

These describe hardware intent.

They do not require a fixed FPGA/ASIC architecture.

---

91. HDL width semantics

A hardware width is semantic when the program explicitly requires it.

For example:

Signal<Width>

may express an actual algorithmic/data-path requirement.

The language must distinguish that from a compiler-imposed universal width.

The compiler cannot declare:

MAX_SIGNAL_WIDTH = 64

as a language rule merely because one backend prefers 64 bits.

---

92. Hardware/software co-design

A type may cross the software/hardware boundary where the language explicitly supports it.

For example:

HardwareModule<Input, Output>
Accelerator<Input, Output>

may be used to express an interface contract.

The source type must remain independent of:

FPGA vendor
ASIC process
CPU model
GPU architecture
QPU vendor

unless an explicit target-specific dialect is selected.

---

93. Hardware resource types

Hardware resource types describe requirements/capabilities such as:

Compute
Memory
Interconnect
Accelerator
QuantumDevice
Signal
TimingDomain

A hardware resource type is not a physical device ID.

---

94. Interoperability types

Foreign interfaces may introduce types representing:

C-compatible values
C++ interfaces
Rust interfaces
WASM values
OpenQASM values
QIR values
HDL values

These types must explicitly identify interoperability semantics.

They must not contaminate the canonical Zamani type system with backend-specific internal representations.

---

95. Opaque types

An opaque type hides implementation representation while exposing a semantic contract.

Opaque types are useful for:

device handles
cryptographic keys
compiler-managed resources
quantum states
foreign objects
runtime services

An opaque type must not be assumed structurally compatible with its hidden representation.

---

96. Capability-safe opaque resources

Opaque resources should normally be used with explicit capabilities.

For example:

Device<T>

does not automatically grant:

read
write
execute
measure
allocate
configure

permissions.

Capabilities must be separately established.

---

97. Security-sensitive types

Security-sensitive values should use dedicated semantic types where appropriate.

Examples:

Secret<T>
PublicKey
PrivateKey
Signature
Hash
Credential
CapabilityToken

The type system must prevent accidental structural interchange where security semantics differ.

---

98. Cryptographic types

Cryptographic types must preserve algorithm/domain identity where algorithm identity is semantically important.

For example:

Hash<A>
Signature<A>
Key<A>

may carry algorithm families at the semantic level.

The grammar should not require every cryptographic algorithm to become a reserved keyword.

---

99. Temporal types

Temporal computation may use semantic types for:

Duration
Instant
Interval
Timestamp
Timeline
LogicalTime

Physical clock representation is not part of type identity unless explicitly required.

A logical timeline must not be limited by a fixed number of branches or events.

---

100. Timeline/resource semantics

If a timeline is a resource, ownership and borrowing rules apply.

Fork/merge semantics must be defined separately from ordinary value copying.

A compiler must not confuse:

timeline identity
wall-clock time
logical ordering
execution timestamp

---

101. Agent types

Agents may be modeled as stateful computational resources.

An agent type may include:

state
capabilities
effects
communication
memory
policy

Agent semantics must remain independent of the number of agents deployed.

---

102. Sankofa-compatible semantic types

Where the language provides memory/learning/history constructs, types may distinguish:

Memory
Experience
Observation
Knowledge
Inference
History
Provenance
Consensus

These are semantic abstractions.

The parser/type system does not itself execute learning, maintain runtime memory, or perform inference.

---

103. Resource ownership versus resource availability

The type system must distinguish:

I own X
I borrow X
I require X
I can use X
the target provides X
the runtime allocated X

These are different concepts.

A type MUST NOT infer resource ownership merely because a capability is available.

---

104. Resource availability

Availability is an environmental property.

For example:

Qubits<N>

may be satisfiable on one target and unsatisfiable on another.

That does not make "Qubits<N>" itself invalid.

Compilation/deployment must report:

RESOURCE_UNSATISFIED

or an equivalent stable diagnostic when no permitted realization satisfies the requirement.

---

105. Type-level resource constraints

Resource constraints may be represented through:

generic constraints
refinements
capabilities
resource contracts
effects
execution requirements

The type system must preserve them for downstream resource analysis.

---

106. Capability negotiation

When a type requires a capability, the compiler/runtime may negotiate among available targets.

The source type does not select one target unless explicitly requested.

This is essential to POCO-REAF.

---

107. Target-independent type equality

These must remain semantically equivalent across targets:

Vector<N, Float>

compiled for:

CPU
GPU
FPGA
distributed cluster
future accelerator

provided the selected realization satisfies the program's semantic requirements.

---

108. Target-dependent types

Target-specific types may exist only in explicitly target-specific contexts.

Such types must be clearly distinguishable from portable types.

A target-specific type must identify:

target/domain
version
capabilities
compatibility
lowering rules

Target-specific types must not silently leak into portable source code.

---

109. Dialect types

A dialect may introduce additional types.

Every dialect-defined type must specify:

dialect name
dialect version
type identity
syntax
semantic meaning
constraints
AST mapping
semantic mapping
IR mapping
compatibility
lowering behavior

A dialect cannot silently redefine a core Zamani type.

---

110. Subtyping

Where Zamani supports subtyping, the relation must be explicit and deterministic.

Possible relations include:

nominal subtyping
structural subtyping
trait/interface satisfaction
capability satisfaction
refinement implication

Subtyping must not depend on target hardware.

---

111. Variance

Generic variance must be explicitly defined for types where variance applies.

Possible positions:

covariant
contravariant
invariant

The compiler MUST NOT infer variance merely from implementation representation if doing so would alter type safety.

---

112. Type compatibility

Type compatibility is not identical to type equality.

Two types may be:

equal
compatible
convertible
subtypes
structurally compatible
nominally incompatible

The semantic checker must distinguish these relations.

---

113. Explicit conversions

Conversions must specify:

source type
destination type
loss semantics
failure semantics
effect semantics
resource semantics

A conversion must not silently change ownership or resource identity.

---

114. No implicit resource conversion

A value representing:

Qubit
DeviceHandle
MemoryRegion
Capability
LinearResource

must not be implicitly converted into another resource type merely because their representations happen to be compatible.

---

115. Type inference and hardware independence

Type inference MUST complete without requiring:

target discovery
device probing
hardware enumeration
runtime queries

Compilation may perform target capability analysis after source type resolution.

This separation is necessary for reproducible compilation.

---

116. Type checking and hardware independence

Ordinary source type checking must not require physical hardware.

A program can therefore be:

parsed
type checked
verified
optimized

without having a physical CPU/GPU/FPGA/QPU available.

Target realization happens later.

---

117. Type checking and resource constraints

A resource requirement may be syntactically and semantically valid while being unsatisfied by a target.

Therefore:

type validity

and:

resource feasibility

must be separate compiler stages.

---

118. Compiler resource limits

The compiler MAY enforce operational resource limits to protect itself from:

hostile input
memory exhaustion
pathological type recursion
constraint explosion
compile-time computation explosion

Such limits MUST be:

- explicit;
- configurable where appropriate;
- diagnostically visible;
- outside language semantic limits;
- incapable of changing the meaning of successfully compiled programs.

---

119. No hidden host-width semantics

The following Rust implementation types must never silently define universal Zamani semantics:

usize
isize
u64
i64
pointer width
allocation size
Vec capacity
HashMap capacity

They may be used internally where appropriate.

Their limitations must not become source-language type limits.

---

120. Type-level cardinality and "usize"

A semantic cardinality such as:

N

must not automatically be represented as "usize" if that would impose an unintended semantic limit.

Internal conversion to "usize" is valid only at a boundary where the implementation has explicitly established that the target operation requires a host index and that the conversion is safe.

Failure must be explicit.

---

121. Memory representation

The source type system does not define physical representation unless the type explicitly belongs to a representation-sensitive domain.

Representation-sensitive constructs may include:

extern
repr
ABI types
HDL types
FFI types
serialization layouts

Portable source types remain abstract.

---

122. ABI

ABI information is downstream unless the programmer explicitly requests ABI compatibility.

Function type identity must not accidentally depend on one machine's ABI.

Explicit ABI types belong to interoperability/compile contracts.

---

123. Serialization

A type may declare serialization capabilities.

Serialization format is distinct from semantic type identity.

For example:

User

may serialize as JSON, binary, CBOR, or another format without becoming a different source type.

---

124. Equality and hashing of types

Compiler-internal type identity must use canonical semantic information.

Type hashing must not depend on:

memory addresses
iteration order of unordered structures
process identity
random seeds

where canonical determinism is required.

---

125. Deterministic type resolution

Given the same:

source
language version
dialect set
feature configuration
semantic environment

type resolution must produce the same semantic result.

Target-specific optimization may occur later.

---

126. Type diagnostics

Type errors must identify:

diagnostic code
source span
primary type
expected type
actual type
relevant constraints
resource/capability requirement when applicable
suggested correction where possible

Diagnostics must not expose unstable memory addresses or backend implementation details.

---

127. Required diagnostic classes

The implementation should distinguish at least:

TYPE_UNKNOWN
TYPE_MISMATCH
TYPE_INFERENCE_AMBIGUOUS
TYPE_NOT_CALLABLE
TYPE_NOT_INDEXABLE
TYPE_NOT_ITERABLE
TYPE_PARAMETER_MISMATCH
TYPE_CONSTRAINT_UNSATISFIED
TYPE_BOUND_UNSATISFIED
TYPE_SHAPE_MISMATCH
TYPE_DIMENSION_MISMATCH
TYPE_INVALID_CONVERSION
TYPE_OWNERSHIP_VIOLATION
TYPE_BORROW_VIOLATION
TYPE_LINEARITY_VIOLATION
TYPE_AFFINITY_VIOLATION
TYPE_EFFECT_VIOLATION
TYPE_CAPABILITY_UNSATISFIED
TYPE_RESOURCE_UNSATISFIED
TYPE_QUANTUM_LINEARITY_VIOLATION
TYPE_QUANTUM_STATE_VIOLATION
TYPE_TARGET_CONSTRAINT_UNSATISFIED
TYPE_DIALECT_INCOMPATIBLE

Exact codes may be finalized by the diagnostics specification, but their semantic distinction must be preserved.

---

128. Error messages and portability

Diagnostics must not assume:

CPU
GPU
QPU
FPGA

unless the relevant compilation stage is actually reporting a target-specific issue.

A source type error must remain understandable independently of target hardware.

---

129. Type validation order

The compiler should conceptually perform:

lexical validation
        ↓
syntactic validation
        ↓
AST structural validation
        ↓
name resolution
        ↓
generic/type resolution
        ↓
type inference
        ↓
constraint solving
        ↓
ownership/resource/effect validation
        ↓
semantic type validation
        ↓
canonical semantic model
        ↓
IR lowering

The exact implementation may combine passes, but observable semantics must preserve these boundaries.

---

130. AST contract

Every source-level type construct must map to the canonical AST.

The mapping is:

source syntax
    ↓
grammar/types/*
    ↓
TypeExpr
    ↓
AST structural validation
    ↓
semantic type resolution

No grammar construct may be added without determining its AST representation in advance.

This is mandatory for independent file completion.

---

131. Grammar integration contract

The existing:

grammar/types/types.g4

owns concrete type syntax.

This specification owns meaning.

The grammar MUST NOT encode:

type checking
name resolution
generic substitution
hardware availability
QEC
routing
scheduling
calibration
runtime execution

The current grammar already separates syntax from these concerns; this specification formalizes that boundary.

---

132. Type grammar integration

The following source-level categories must remain represented through the existing "grammar/types/*" architecture:

type expression
primitive type
named type
generic type
array
slice
tuple
function
reference
pointer
optional
result
never
unit
bounds
type-level values

New type domains should extend the appropriate semantic contracts rather than introducing a second root type grammar.

---

133. Type-level value grammar integration

"grammar/types/type-constraints.g4" and related type grammar files may express:

constraints
capability satisfaction
associated-type requirements
semantic type properties

The semantic checker determines whether those constraints are true.

The grammar only represents them.

The existing repository already contains this type-constraint boundary.

---

134. AST integration

The source AST must remain domain-neutral.

It may represent:

TypeExpr

with source-level information.

It MUST NOT contain:

LLVMType
MLIRType
QIRType
VendorQPUType
CUDAType
PhysicalQubitType
HardwareTopologyType

as ordinary core type nodes.

The existing AST architecture explicitly maintains "TypeExpr" as the single authoritative source-level representation.

---

135. TypeBuilder integration

"src/frontend/ast/node/builders/type_builder.rs" is construction infrastructure.

It MUST construct the existing canonical "TypeExpr".

It MUST NOT create another type hierarchy.

It must remain independent of:

semantic analysis
ZUIR
quantum::ir
hardware
routing
scheduling
QEC
runtime
vendor SDKs
LLVM
MLIR
QIR

This matches the current repository's builder contract.

---

136. Semantic model integration

Semantic type resolution transforms:

TypeExpr

into the canonical semantic type representation.

This phase owns:

name resolution
type equality
generic substitution
constraint solving
trait resolution
associated types
subtyping
ownership
borrowing
linearity
effects
resource contracts
capability contracts
domain semantics

It does not belong in the grammar.

---

137. ZUIR integration

The type system feeds the canonical semantic representation/ZUIR.

The flow is:

TypeExpr
    ↓
semantic type
    ↓
semantic model
    ↓
ZUIR

Source AST types must not contain ZUIR implementation details merely to simplify lowering.

---

138. Classical IR integration

Classical semantic types lower to the canonical classical representation required by the existing compiler architecture.

The type system remains independent of:

LLVM
machine registers
specific CPU instruction sets
SIMD width
cache architecture

---

139. Quantum IR integration

Quantum semantic types lower to:

quantum::ir

where quantum semantics become canonical.

The type specification does not define a second quantum IR.

The flow is:

Zamani TypeExpr
    ↓
semantic quantum type
    ↓
quantum::ir
    ↓
quantum optimization
    ↓
routing
    ↓
scheduling
    ↓
QEC/resilience
    ↓
ZQN
    ↓
HAL
    ↓
target

---

140. HDL IR integration

HDL types lower to the canonical hardware/HDL semantic representation.

The source type must preserve:

signal semantics
width
timing intent
interface contract
resource intent
verification properties

without assuming one FPGA/ASIC implementation.

---

141. Hybrid integration

Hybrid types must permit the semantic model to connect:

classical value
quantum resource
measurement result
classical control
quantum operation

without creating two unrelated type systems.

---

142. Resource manager integration

Resource-aware semantic types may carry resource requirements.

The resource manager resolves actual availability later.

For example:

QuantumRegister<N>

can generate a resource requirement equivalent to:

required quantum capacity >= N

but does not allocate physical qubits.

---

143. Scheduling integration

Type semantics may establish requirements relevant to scheduling.

Examples:

timing-sensitive value
resource-bound operation
quantum resource
exclusive resource

Scheduling decides actual ordering and placement.

The type system must not encode scheduler slots.

---

144. Routing integration

A type can require:

logical connectivity property

but routing determines physical realization.

The type system must not contain a universal physical topology.

---

145. QEC integration

QEC consumes semantic quantum/resource requirements.

The type system can express:

logical resource
fault-tolerant requirement
error-correction capability

but QEC implementation belongs downstream.

---

146. ZQN integration

ZQN consumes fault/noise semantics.

The type system must not implement:

noise model
fault decoder
syndrome extraction
recovery algorithm

It only exposes semantic requirements and classifications.

---

147. HAL integration

HAL is responsible for target/device capability realization.

Types may require:

Capability<X>
Resource<X>

but must not directly depend on a HAL implementation.

---

148. Runtime integration

Runtime receives validated semantic information.

Runtime may resolve:

allocation
resource availability
execution policy
dynamic capability
device selection

subject to the semantic contracts.

Runtime must not reinterpret a source type differently from the compiler.

---

149. Compile-time versus runtime type information

A type property may be:

compile-time known
symbolically known
statically constrained
runtime known
dynamically checked
opaque

The language must explicitly distinguish these categories.

A runtime-known shape is not automatically an invalid type.

---

150. Runtime-sized structures

Zamani must support types whose cardinality becomes known only at runtime where the semantics permit it.

Examples:

Vector<N, T>

where "N" is runtime-derived.

The compiler may choose dynamic representation.

The source semantics remain valid.

---

151. Compile-time computation

Compile-time type-level computation may be used for:

dimensions
constraints
type selection
specialization
proof conditions
resource requirements

Compile-time computation must remain deterministic where required.

---

152. Compile-time resource limits

Compiler resource limits may constrain how much compile-time computation is performed.

Such limits are operational policies.

They must not redefine successful program semantics.

A limit failure must be explicitly diagnosed.

---

153. Type normalization

Equivalent types should have a canonical semantic normalization where practical.

Normalization may include:

alias expansion
canonical generic arguments
canonical path resolution
equivalent refinement normalization
effect normalization
constraint normalization

Normalization must preserve meaning.

---

154. Recursive type normalization

Recursive types must be normalized without infinite compiler loops.

The implementation must use cycle-aware algorithms.

A compiler implementation limit may terminate pathological processing, but must produce an explicit diagnostic rather than silently changing type meaning.

---

155. Type cycles

The semantic model must distinguish valid recursive declarations from invalid infinite definitions.

For example:

type List<T> = Cons(T, List<T>) | Nil

may be valid.

A type definition that requires an infinitely expanding value representation without an indirection or well-founded semantic rule must be rejected.

---

156. Type compatibility across versions

Type meaning must be versioned.

Changes that alter:

type identity
subtyping
conversion
ownership
linearity
effect semantics
resource semantics
quantum semantics

are language compatibility changes.

They must be recorded through the existing compatibility/versioning architecture.

---

157. Serialization compatibility

Changes to source AST type serialization must not silently change semantic type identity.

AST serialization versions are distinct from language type versions.

---

158. Dialect compatibility

A dialect may extend the type system but must declare compatibility.

A dialect must not silently redefine:

Int
Float
Bool
Qubit
Memory
Resource

or other core semantic types.

---

159. Target-specific extensions

Target-specific type extensions must be isolated.

For example:

cuda::...
qpu::...
fpga::...
vendor::...

may exist in a dialect/interoperability context.

They MUST NOT become universal core types merely because one backend supports them.

---

160. Generic quantum programming

Generic quantum code should be expressible without choosing a device.

Conceptually:

fn algorithm<Q : QuantumCapability>(
    q : Q
) -> ...

or an equivalent Zamani construct.

The implementation must permit different valid quantum realizations.

---

161. Generic hardware programming

Hardware-independent code should be able to express:

requires capability("vector.compute")
requires capability("tensor.compute")
requires capability("quantum.measurement")

without selecting hardware IDs.

---

162. Generic distributed programming

Distributed code should be able to express:

requires distributed
requires communication
requires replication

without declaring:

node0
node1
node2

as universal machine identities.

---

163. Type safety across domains

A value must not cross domain boundaries merely because its representation is compatible.

For example:

Qubit
Signal
Bool
Bit
ClassicalBit
PhysicalQubit
LogicalQubit

must remain semantically distinct where their meanings differ.

---

164. Classical bit versus quantum measurement result

A classical Boolean result and a measurement result may share representation but need not have identical provenance/effect semantics.

The type/effect system must preserve relevant provenance where the language requires it.

---

165. Physical qubit versus logical qubit

These are not interchangeable.

LogicalQubit

represents a logical computational resource.

PhysicalQubit

represents a target-level resource.

Portable source programs should normally use logical resources.

Mapping occurs downstream.

---

166. Resource provenance

Resource-sensitive types may carry provenance.

Provenance can identify:

source declaration
resource requirement
allocation decision
runtime resource

but source type identity must not become dependent on a particular allocation.

---

167. Type erasure

Type erasure is an implementation technique.

It must preserve all runtime-visible semantic guarantees required by the language.

Erasure must not remove:

ownership safety
capability requirements
effect requirements
resource obligations
quantum linearity

that remain relevant after compilation.

---

168. Monomorphization

Monomorphization is an implementation strategy.

It must preserve generic semantics.

The compiler may generate different target-specific representations while preserving one source-level type contract.

---

169. Type-directed optimization

Optimization may use type information.

Examples:

numeric specialization
shape specialization
effect optimization
resource optimization
quantum optimization

Optimization MUST NOT alter the semantic type contract.

---

170. Optimization and target resources

Optimization may select a realization based on target capabilities.

For example:

CPU
GPU
FPGA
QPU
distributed system

This does not change the source type.

---

171. Type-based dispatch

If Zamani supports static or dynamic dispatch, dispatch semantics must be explicit.

Dispatch cannot silently depend on target hardware unless a target-specific policy explicitly permits it.

---

172. Pattern matching and types

Pattern matching must respect:

sum types
optionality
refinements
ownership
linearity
resource semantics

A pattern that consumes a linear resource must consume it according to the resource contract.

---

173. Type exhaustiveness

For finite sum/enum types, exhaustive pattern checking should be performed statically where possible.

The checker must distinguish:

known finite variants
open/extensible variants
dynamic values

---

174. Type guards

Runtime type checks may refine a dynamic/opaque value.

A successful guard may establish a more precise type.

A failed guard must preserve defined control-flow semantics.

---

175. Type narrowing

Narrowing must be sound.

A narrowing operation must not allow the compiler to assume a stronger type without an established condition.

---

176. Nullability

Zamani should prefer explicit:

Option<T>

rather than implicit nullable references.

If null-like values exist for interoperability, they must have explicit type semantics.

Null must not be an invisible inhabitant of every reference type.

---

177. Pointer types

Pointers are representation-sensitive constructs.

Pointer syntax does not imply:

32-bit
64-bit
128-bit

The target determines representation subject to the type/ABI contract.

Raw pointer operations must not weaken the language's safety guarantees unless an explicitly defined unsafe/FFI boundary exists.

The core compiler implementation itself must remain safe Rust as required by this project.

---

178. No unsafe Rust

The Zamani implementation must use:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

where applicable.

The type-system implementation must not require:

unsafe
raw pointer dereference
unchecked indexing
transmute
manual memory reclamation

to implement its semantic model.

---

179. Rust implementation boundary

Rust implementation types are implementation details.

Examples:

Vec<T>
Option<T>
Result<T, E>
String
Box<T>
Arc<T>

may be used internally.

Their Rust-specific semantics must not accidentally become Zamani semantics.

---

180. Rust version

The implementation contract targets:

Rust 1.97
Rust 1.97.1
Rust 2021
stable Rust

No nightly-only feature may be required for the type system.

The implementation must remain compatible with the repository's declared Rust baseline.

---

181. Type-system determinism

Type checking must not depend on:

randomness
wall-clock time
hardware state
process ID
thread scheduling
memory addresses
unordered iteration

when determining language semantics.

If a type system feature intentionally involves randomness, the randomness must be explicit in the language semantics.

---

182. Parallel type checking

The compiler may type-check independent units in parallel.

Parallelization must not alter:

diagnostic meaning
type identity
constraint results
semantic type resolution

---

183. Incremental compilation

Type information may be cached.

Caches must be invalidated based on semantic dependencies rather than incidental timestamps alone.

Cached type identity must remain deterministic.

---

184. Cross-module types

Types may be imported/exported across modules.

Cross-module type identity must use canonical module/type identity.

A type must not become different merely because it was imported through an alias.

---

185. Module versioning

Imported type definitions must respect module/package version compatibility.

A module version change that changes type semantics is a compatibility event.

---

186. Visibility

Type declarations may have visibility such as:

private
module
package
public

Visibility affects accessibility, not intrinsic type identity.

---

187. Type privacy

Private implementation details of an opaque type must not become visible through structural inference.

This preserves abstraction boundaries.

---

188. Recursive generic constraints

Generic constraints may refer recursively to types where explicitly permitted.

The semantic resolver must detect unsatisfiable or infinitely recursive constraints.

---

189. Constraint solving

Constraint solving must distinguish:

satisfied
unsatisfied
unknown
inconsistent
resource-unsatisfied
target-unsatisfied

Unknown is not automatically equivalent to true.

---

190. Constraint provenance

A failed constraint should retain provenance sufficient to explain:

which declaration introduced it
which type introduced it
which generic parameter caused it
which source span required it

This improves production diagnostics.

---

191. Resource constraint solving

Resource constraints may remain symbolic until target selection.

For example:

Qubits<N>

can remain symbolic until a target capability model is available.

This allows one source program to compile against many possible targets.

---

192. Compile-once principle

The semantic type representation should be sufficiently target-independent that a compiled semantic artifact can be reused for different target realizations when the compiler architecture permits it.

The type system must therefore avoid embedding accidental target facts.

---

193. Run-anywhere principle

A program can run on a target only if that target satisfies its semantic requirements.

Therefore POCO-REAF does not mean:

«every target can execute every program.»

It means:

«the program does not need to be rewritten merely because the available realization changes, provided the target can satisfy the program's requirements or the compiler can legally transform the program to a satisfying realization.»

---

194. Compile-once limitations

A target may legitimately reject a program because:

required capability unavailable
required resource unavailable
required numerical guarantee unavailable
required quantum operation unavailable
required timing guarantee unavailable
required security property unavailable

This is a resource/capability incompatibility, not a type-system failure.

---

195. Semantic versus implementation decisions

The type system must explicitly distinguish:

Semantic

Matrix<M, N>
Qubit
LogicalQubit
Tensor<Shape, T>
Memory<N>
Capability<X>

Implementation

GPU 0
physical qubit 17
register r3
memory bank 2
thread 8
scheduler slot 14

The second category belongs downstream.

---

196. Hard-coding prohibition

The following are prohibited as universal type semantics:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_THREADS
MAX_TENSOR_RANK
MAX_VECTOR_LENGTH
MAX_REGISTER_WIDTH
MAX_TYPE_DEPTH
MAX_GENERIC_ARITY

The compiler may have configurable operational safety limits, but these MUST NOT be presented as language type limits.

---

197. Hard-coding audit

Every new type-system feature must be checked for:

machine-size assumptions
pointer-size assumptions
register-size assumptions
fixed topology
fixed device count
fixed qubit count
fixed thread count
fixed memory
fixed tensor rank
fixed generic arity
fixed tuple arity
fixed recursion depth
vendor assumptions
backend assumptions

A feature fails review if any such assumption is introduced without semantic justification.

---

198. Type-system security

The type system must reject programs that violate:

ownership
borrowing
linearity
affinity
capability requirements
resource contracts
quantum resource rules
type invariants

The type system must not rely on undefined behavior for safety.

---

199. Hostile input

Compiler resource exhaustion is a security concern.

The implementation may enforce configurable limits on:

AST size
type recursion
constraint solving
generic expansion
compile-time evaluation
diagnostic count
memory use

These are compiler protection policies.

They are not language-level semantic limits.

---

200. Error recovery

Parser recovery may construct incomplete type expressions.

Incomplete expressions must not be treated as valid semantic types.

The structural validation layer must distinguish:

valid
incomplete
malformed
ambiguous
unresolved

---

201. Source spans

Every type expression must preserve source-span information sufficient for diagnostics.

Source spans are metadata and are not part of semantic type equality.

Two identical types at different source locations remain the same semantic type.

---

202. Attributes and annotations

Attributes may provide type-related metadata.

Examples:

#[deprecated]
#[opaque]
#[repr(...)]
#[ffi(...)]
#[requires(...)]

Attributes must be classified as:

semantic
diagnostic
tooling
lowering
target-specific

and must not silently change core type meaning.

---

203. Type extensions

New core type forms require:

1. specification entry;
2. syntax contract;
3. lexer contract if new tokens are required;
4. AST mapping;
5. structural validation;
6. semantic resolution;
7. constraint behavior;
8. IR mapping;
9. compiler integration;
10. runtime integration where applicable;
11. positive tests;
12. negative tests;
13. boundary tests;
14. scalability tests;
15. compatibility entry.

A feature is not complete when only its grammar exists.

---

204. File ownership contract

This file owns:

type meaning
type categories
type identity
type compatibility
type relationships
type constraints
type resource/type boundary
type/effect boundary
type/capability boundary
type/quantum boundary
type/hardware boundary
type/IR boundary
type scalability guarantees

It does NOT own:

concrete parser implementation
lexer implementation
AST implementation
semantic implementation code
IR implementation
runtime implementation
hardware implementation
QEC implementation
routing implementation
scheduling implementation
HAL implementation

---

205. Integration contract with "grammar/types/*"

"grammar/types/*" owns concrete source syntax.

This specification owns semantic interpretation.

The integration must be:

types/*.g4
    ↓
TypeExpr
    ↓
semantic type

not:

types/*.g4
    ↓
backend type

---

206. Integration contract with "grammar/spec/type-system.md"

"grammar/spec/type-system.md" is an existing type-system specification surface.

It MUST NOT become a second semantic authority.

The repository must converge on one normative meaning for the type system.

Its role should be either:

compatibility/implementation contract

or:

cross-reference/derived specification

while this document provides the authoritative production type-system specification under "grammar/specification/".

Until that migration is completed, any discrepancy between the two documents is a production blocker and must be resolved explicitly rather than silently choosing one interpretation.

---

207. Integration contract with "grammar/specification/semantics.md"

"semantics.md" owns general evaluation/binding/control semantics.

This file owns type-specific rules.

Neither document may silently redefine the other.

---

208. Integration contract with "grammar/specification/language.md"

"language.md" defines language-wide principles such as portability and POCO-REAF.

This file instantiates those principles for types.

---

209. Integration contract with "grammar/specification/poco-reaf.md"

"poco-reaf.md" defines the program portability requirement.

This file ensures type semantics do not introduce target-specific assumptions.

---

210. Integration contract with resources

"grammar/resources/*" owns resource requirement syntax.

The type system consumes the resulting semantic resource contracts.

A resource requirement does not become a type unless its semantic identity requires it.

---

211. Integration contract with hardware

"grammar/hardware/*" owns hardware-intent syntax.

Hardware type semantics remain target-independent.

Physical realization belongs downstream.

---

212. Integration contract with quantum

"grammar/quantum/*" owns quantum source syntax.

Quantum types are resolved into semantic quantum types.

Quantum semantics then flow into:

quantum::ir

No second quantum IR is permitted.

---

213. Integration contract with classical

"grammar/classical/*" owns classical-domain syntax.

Classical types remain part of the universal Zamani type system.

There must not be a separate incompatible "classical type system."

---

214. Integration contract with HDL

"grammar/hdl/*" owns HDL syntax.

HDL semantic types integrate with the universal type system.

Software/hardware boundaries must be explicit.

---

215. Integration contract with AI

"grammar/ai/*" owns AI/ML syntax.

AI types such as tensors/models/datasets must reuse the universal type machinery for:

genericity
shape
ownership
resources
capabilities
effects

---

216. Integration contract with distributed computing

"grammar/distributed/*" may introduce distributed syntax.

Distributed semantic types must use the same:

ownership
resource
capability
effect
generic
constraint

framework.

---

217. Integration contract with interoperability

"grammar/interoperability/*" owns foreign type syntax and ABI declarations.

Foreign representations must not redefine native Zamani type meaning.

---

218. Integration contract with dialects

"grammar/dialects/*" may extend the type language.

Dialect types must declare compatibility and lowering rules.

They must not silently modify core type semantics.

---

219. Integration contract with macros

Macros may generate type syntax.

After expansion, generated type syntax must pass the same structural and semantic validation as handwritten syntax.

Macros cannot bypass type checking.

---

220. Integration contract with metaprogramming

Metaprogramming may inspect or construct type representations.

It must operate through the canonical type model.

It must not create a second hidden type system.

---

221. Compiler integration

The compiler must consume validated semantic types.

Compiler passes may use types for:

optimization
specialization
layout
resource planning
vectorization
quantum optimization
hardware lowering

but must preserve type semantics.

---

222. Runtime integration

Runtime type metadata may be retained when required.

Runtime metadata must not be confused with compile-time semantic type identity.

---

223. Verification integration

Formal verification may consume:

types
refinements
effects
resources
capabilities
ownership

The type system should expose enough semantic structure for verification without embedding a particular theorem prover.

---

224. Determinism contract

The same semantic program under the same language/specification configuration must resolve to the same type meaning.

The implementation must not use nondeterministic maps/sets in a way that affects semantic decisions.

Where unordered collections are necessary internally, results must be canonicalized before affecting observable semantic output.

---

225. Scalability contract

The type system must scale from:

tiny embedded program

through:

single CPU
single GPU
single FPGA
single QPU
multi-accelerator system
distributed system
HPC system
cloud
edge
hybrid quantum-classical system
large AI/data system
future computational substrate

without changing the fundamental type model.

---

226. Resource-adaptive execution

The same semantic type may be lowered differently depending on resources.

For example:

Tensor<A, Shape>

could be realized through:

CPU
GPU
FPGA
distributed execution
specialized accelerator

provided the semantic contract remains satisfied.

---

227. Quantum resource adaptation

Likewise:

LogicalQubit

may be realized using different:

physical qubit counts
error-correction codes
connectivity
gate decompositions
scheduling strategies

without changing the source type.

---

228. No physical resource leakage into type identity

A physical mapping must never cause:

Qubit

to become a different source type merely because it was mapped to a different physical qubit.

The mapping is metadata/lowering state, not source semantic identity.

---

229. Resource insufficiency

If no target satisfies a valid program's resource requirements, compilation/deployment must fail explicitly.

Example:

TYPE VALID
RESOURCE UNSATISFIED

must remain distinguishable from:

TYPE INVALID

---

230. Portability classification

Every type may be classified as:

portable
conditionally portable
target-specific
dialect-specific
foreign

The classification must be explicit.

---

231. Portable core

The portable core includes types whose meaning is independent of:

vendor
architecture
device
backend
operating system
machine size

Portable programs should use these types whenever possible.

---

232. Conditional portability

A type may be conditionally portable when it requires a capability.

Example:

requires capability("quantum.dynamic_circuit")

The type remains portable across all targets providing that capability.

---

233. Target-specific types

Target-specific types must not pretend to be portable.

They should be explicitly isolated through:

dialect
interop module
target profile
foreign declaration

---

234. Type feature manifests

For production traceability, each major type-system feature should eventually have a feature contract under the repository's feature/contract system.

A type feature contract should record:

Feature ID
Name
Status
Specification
Grammar
Tokens
AST representation
Semantic representation
Constraints
Effects
Resources
Capabilities
IR mapping
Compiler consumers
Runtime consumers
Positive tests
Negative tests
Boundary tests
Scalability tests
Compatibility
Hard-coding audit

This allows one feature to be completed without later discovering an undocumented downstream dependency.

---

235. Required production tests

The type system must include tests for:

primitive types
named types
aliases
nominal types
structural types
generic types
generic constraints
associated types
tuples
records
sum types
recursive types
arrays
slices
functions
closures
references
lifetimes
ownership
linear types
affine types
option
result
never
unit
numeric conversions
shape constraints
dependent values
refinements
effects
capabilities
resources
quantum types
quantum linearity
logical/physical distinction
HDL types
hardware capabilities
distributed types
AI/data types
interoperability types
dialect types

---

236. Negative tests

Negative tests must include:

unknown type
invalid generic argument
wrong generic arity
unsatisfied bound
shape mismatch
invalid conversion
invalid ownership
invalid borrow
double consumption
linear resource duplication
affine violation
invalid effect
missing capability
missing resource
invalid quantum copy
invalid logical/physical conversion
invalid refinement
incompatible dialect type

---

237. Boundary tests

Boundary tests must include:

zero-sized values where semantically valid
one-element collections
very large symbolic dimensions
nested generic types
deeply nested types
recursive types
large tuple structures
large generic argument sets
large constraint sets
large quantum cardinalities
large distributed resource requirements

The tests must not define artificial universal limits.

---

238. Scalability tests

Scalability tests must verify that type semantics remain correct for increasingly large:

generic structures
type graphs
shape expressions
constraint graphs
quantum resource counts
tensor shapes
distributed resource sets
module graphs

Failures caused by configured compiler resource policies must be reported as resource failures, not semantic type failures.

---

239. Cross-target tests

Equivalent source types should be tested against multiple semantic target profiles where available:

CPU
GPU
FPGA
QPU
distributed
simulated

The tests must verify that source type identity remains stable.

---

240. Quantum cross-target tests

Quantum tests must verify that:

logical qubit
quantum register
measurement
dynamic circuit
quantum operation

remain semantically stable across different target capability models.

---

241. No fixed-qubit tests

Tests must explicitly reject the assumption that:

MAX_QUBITS = N

is a language rule.

A symbolic or larger quantum requirement must remain syntactically and semantically representable.

---

242. No fixed-memory tests

Tests must reject compiler logic that assumes:

MAX_MEMORY = N

as a language-level type restriction.

---

243. No fixed-core tests

Tests must reject compiler logic that interprets:

parallel

as meaning a fixed number of threads/cores.

---

244. No fixed tensor-rank tests

Tests must ensure that tensor rank is governed by the type/program semantics rather than an arbitrary parser/compiler constant.

---

245. No fixed generic-arity tests

Generic argument count must be determined by the declaration and semantics, not by an arbitrary global maximum.

Operational compiler limits are separate.

---

246. No fixed tuple-arity tests

Tuple arity must be determined by source semantics.

No global tuple-size constant is permitted as a language rule.

---

247. Compatibility tests

Compatibility testing must compare:

grammar/specification/types.md
grammar/spec/type-system.md
grammar/types/*
grammar/Zamani.g4
src/lexer.rs
src/parser.rs
src/frontend/ast/node/types/*
semantic type resolver
ZUIR
domain IR
compiler
runtime

for every stabilized type feature.

---

248. Completion criteria for this specification

This document is complete when:

- type authority is unambiguous;
- type identity is defined;
- primitive types are defined;
- composite types are defined;
- generic types are defined;
- dependent/value-parameterized types are defined;
- refinement semantics are defined;
- ownership is defined;
- linearity is defined;
- affinity is defined;
- references/lifetimes are defined;
- effects are integrated;
- capabilities are integrated;
- resources are integrated;
- quantum types are integrated;
- classical types are integrated;
- HDL types are integrated;
- distributed types are integrated;
- AI/data types are integrated;
- interoperability is integrated;
- dialects are integrated;
- target independence is defined;
- POCO-REAF is defined;
- no-hard-coding policy is defined;
- AST integration is defined;
- semantic integration is defined;
- IR integration is defined;
- compiler integration is defined;
- runtime integration is defined;
- diagnostics are defined;
- testing requirements are defined;
- compatibility requirements are defined;
- safe-Rust requirements are defined.

---

249. Required implementation invariants

The implementation MUST preserve these invariants:

ONE canonical source-level TypeExpr
ONE semantic interpretation per valid source type
NO duplicate quantum type system
NO backend-specific core types
NO hardware-specific portable type identity
NO implicit dynamic escape
NO silent lossy conversion
NO implicit resource duplication
NO unsafe Rust requirement
NO machine-size language limits
NO fixed qubit language limits
NO fixed tensor language limits
NO fixed generic language limits
NO fixed tuple language limits
NO fixed distributed-node language limits
NO fixed accelerator language limits

---

250. Final canonical type pipeline

The complete type pipeline is:

Zamani source
    │
    ▼
canonical lexer
    │
    ▼
grammar/Zamani.g4
    │
    ▼
grammar/types/*
    │
    ▼
parser
    │
    ▼
canonical TypeExpr
    │
    ▼
AST structural validation
    │
    ▼
name resolution
    │
    ▼
generic/type resolution
    │
    ▼
constraint solving
    │
    ▼
ownership / lifetime / linearity
    │
    ▼
effect resolution
    │
    ▼
capability resolution
    │
    ▼
resource semantics
    │
    ▼
domain semantic types
    │
    ├──────────────┬───────────────┬──────────────┐
    ▼              ▼               ▼              ▼
 classical       quantum          HDL           hybrid
 semantic        semantic         semantic      semantic
 model           model            model         model
    │              │               │              │
    ▼              ▼               ▼              ▼
 classical       quantum::ir      HDL/target     hybrid IR
 IR
    │              │               │              │
    └──────────────┴───────────────┴──────────────┘
                       │
                       ▼
                 optimization
                       │
             ┌─────────┼─────────┐
             ▼         ▼         ▼
          routing   scheduling  resilience
             │         │         │
             └─────────┼─────────┘
                       ▼
                      ZQN
                       │
                       ▼
                      HAL
                       │
                       ▼
                target realization
                       │
       ┌───────────────┼────────────────┐
       ▼               ▼                ▼
      CPU             GPU/FPGA          QPU
       │               │                │
       └───────────────┼────────────────┘
                       ▼
                future targets

---

251. Fundamental Zamani guarantee

The type system therefore guarantees the following architectural principle:

«A Zamani type describes semantic intent and guarantees, not today's machine.»

A developer may write one program containing:

generic computation
quantum computation
classical computation
tensor computation
distributed computation
hardware intent
resource requirements
capability requirements

without rewriting the type system for every target.

The compiler may subsequently choose:

one CPU
many CPUs
GPU
many GPUs
FPGA
ASIC
QPU
hybrid CPU/QPU
distributed cluster
edge device
cloud
future computational substrate

provided the selected realization satisfies the program's semantic requirements.

---

252. POCO-REAF type-system theorem

For any valid Zamani source type "T":

T(source)

must retain the same semantic identity across all valid compilation targets.

Target realization may change:

representation
layout
storage
instruction selection
parallelization
routing
scheduling
physical mapping
error correction
device selection

but must not change the meaning of "T".

Therefore:

PROGRAM TYPE SEMANTICS
        ≠
TARGET IMPLEMENTATION

This separation is the type-system foundation of:

Program Once
Compile Once
Run Everywhere
Anywhere
Forever

subject only to the actual semantic capabilities and resources required by the program and available to the realization.

---

253. Final non-negotiable rules

Zamani's production type system MUST NEVER:

1. encode a fixed maximum qubit count;
2. encode a fixed maximum CPU count;
3. encode a fixed maximum GPU count;
4. encode a fixed maximum FPGA count;
5. encode a fixed maximum node count;
6. encode a fixed maximum memory capacity;
7. encode a fixed maximum tensor rank;
8. encode a fixed maximum vector length;
9. encode a fixed maximum generic arity;
10. encode a fixed maximum tuple arity;
11. encode a fixed maximum function parameter count;
12. encode a fixed hardware topology;
13. identify physical qubits as source-level universal types;
14. make vendor hardware part of core type identity;
15. create a second quantum IR;
16. create a second source-level AST type hierarchy;
17. silently introduce "Any"/dynamic typing when inference fails;
18. silently perform lossy conversions;
19. silently duplicate linear resources;
20. silently change type meaning based on target hardware;
21. require unsafe Rust;
22. rely on undefined behavior for type safety;
23. place QEC implementation inside the type system;
24. place routing implementation inside the type system;
25. place scheduling implementation inside the type system;
26. place calibration implementation inside the type system;
27. place HAL implementation inside the type system;
28. make "usize" define Zamani semantic cardinality;
29. make host pointer width define source type meaning;
30. make compiler implementation limits into language semantics.

---

254. Production definition of done

"grammar/specification/types.md" is the production type-system contract when every implementation layer can answer the following without inventing new semantics later:

What is this type?
Who owns its syntax?
Who owns its AST representation?
Who resolves its name?
Who resolves its generic parameters?
Who checks its constraints?
Who checks its ownership?
Who checks its effects?
Who checks its capabilities?
Who checks its resource requirements?
What is its canonical semantic identity?
How does it lower to IR?
How does it lower to quantum::ir when applicable?
Which compiler passes consume it?
Which runtime components consume it?
Which hardware components consume it?
What are its diagnostics?
What are its negative cases?
What are its boundary cases?
What are its scalability cases?
What are its compatibility rules?
What prevents target-specific hard-coding?
What prevents unsafe implementation?

No type feature is considered production-ready until these questions have defined answers.

---

255. Architectural conclusion

The Zamani type system is therefore not merely a collection of:

int
float
bool
struct
class
Qubit
Tensor

It is the portable semantic contract connecting the universal language to every computational domain.

Its responsibility ends at semantic meaning.

Its downstream consumers determine realization:

type semantics
    ↓
semantic model
    ↓
canonical IR
    ↓
optimization
    ↓
resource analysis
    ↓
routing
    ↓
scheduling
    ↓
resilience / QEC / ZQN
    ↓
HAL
    ↓
actual hardware

That boundary is what permits Zamani to scale from the smallest computational object to arbitrarily large computational systems without turning today's hardware constraints into tomorrow's language limitations.