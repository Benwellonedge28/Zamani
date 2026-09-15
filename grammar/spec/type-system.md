Zamani Type System Specification

Path: "grammar/spec/type-system.md"
Language: Zamani
Specification status: Normative
Specification layer: Static semantics / type system
Specification role: Canonical type-system contract
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Safety requirement: No "unsafe" Rust
Language objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

0. Document Contract

0.1 Purpose

This document defines the normative type system of the Zamani programming language.

It defines:

- what a Zamani type means;
- how types are formed;
- how types are identified;
- how type equality and compatibility work;
- how generic and dependent/parametric information is represented;
- how dimensions and shapes are represented;
- how ownership, linearity and resource usage interact with types;
- how quantum resources are typed;
- how classical, quantum, hybrid, HDL, distributed, AI, data, networking and other domains share one type system;
- how effects and capabilities interact with function and computation types;
- how type inference works;
- how type errors are diagnosed;
- how types lower into the canonical semantic representation and IR;
- how the type system remains independent of target hardware;
- how scalability is preserved from tiny systems to arbitrarily large systems subject only to actual resource availability and explicit implementation policies.

This document does not define:

- lexical tokenization;
- source parsing;
- AST implementation details;
- runtime object layouts;
- physical hardware topology;
- routing;
- scheduling algorithms;
- calibration;
- QEC algorithms;
- ZQN implementation;
- backend-specific register allocation;
- vendor-specific ABI behavior;
- machine-specific resource limits.

Those belong to their respective contracts.

---

1. Authority and Integration

The normative architecture is:

grammar/DESIGN.md
        │
        ▼
grammar/spec/lexical.md
        │
        ▼
grammar/spec/syntax.md
        │
        ▼
grammar/spec/type-system.md
        │
        ▼
grammar/spec/semantics.md
        │
        ├───────────────┬──────────────────┐
        ▼               ▼                  ▼
resources          effects            capabilities
        │               │                  │
        └───────────────┼──────────────────┘
                        ▼
                 Canonical AST
                        │
                        ▼
               Semantic type model
                        │
                        ▼
                  Canonical IR
                        │
          ┌─────────────┼─────────────┐
          ▼             ▼             ▼
    classical IR    quantum::ir     HDL/target IR
          │             │             │
          └─────────────┼─────────────┘
                        ▼
                   Optimization
                        │
             ┌──────────┼──────────┐
             ▼          ▼          ▼
          routing   scheduling   resilience
             │          │          │
             └──────────┼──────────┘
                        ▼
                       ZQN
                        │
                       HAL
                        │
                        ▼
               target realization

1.1 Authority hierarchy

The following hierarchy is mandatory:

1. normative language specifications;
2. normative AST contracts;
3. normative semantic-model contracts;
4. normative IR contracts;
5. implementation;
6. generated/reference documentation;
7. historical/proposed material.

Therefore:

- "grammar/spec/type-system.md" is normative for type semantics;
- "grammar/types/" owns type syntax;
- "grammar/Zamani.g4" composes syntax but does not redefine type semantics;
- "grammar/grammar.md" records implementation conformance;
- "grammar/Zamani-Grammar.md" is historical/design material unless a feature has been formally promoted;
- "src/lexer.rs" recognizes lexical forms;
- "src/parser.rs" constructs syntax;
- "src/frontend/ast/" represents source structure;
- semantic analysis validates type meaning;
- canonical IR owns lowered semantic representation;
- "quantum::ir" remains the canonical quantum semantic boundary.

No lower layer may silently redefine the meaning of a type.

---

2. Ownership Contract

2.1 This document owns

This specification owns:

- type formation;
- type identity;
- type equality;
- type compatibility;
- type inference rules;
- generic constraints;
- shape and dimensional typing;
- numeric typing;
- collection typing;
- function typing;
- ownership/linearity semantics;
- resource-sensitive types;
- capability-sensitive types;
- effect-sensitive computation types;
- quantum type semantics;
- classical type semantics;
- hardware-intent type semantics;
- distributed type semantics;
- temporal type semantics;
- proof/verification type semantics;
- type-level expressions;
- coercion rules;
- conversion rules;
- type error categories;
- type-system portability rules;
- type-system scalability rules.

2.2 This document does not own

It does not own:

- token definitions;
- parser rules;
- AST storage layout;
- runtime values;
- hardware discovery;
- physical qubit assignment;
- QEC implementation;
- ZQN implementation;
- scheduler implementation;
- router implementation;
- calibration;
- vendor-specific device models;
- target-specific machine widths.

---

3. Type-System Design Principles

The Zamani type system MUST be:

1. sound;
2. deterministic where semantics permit;
3. explicitly nondeterministic where required;
4. compositional;
5. target-independent;
6. resource-aware;
7. capability-aware;
8. effect-aware;
9. quantum-safe;
10. ownership-aware;
11. extensible;
12. representable in canonical IR;
13. independently verifiable;
14. implementable in safe Rust;
15. independent of physical machine dimensions;
16. scalable without language-level artificial limits;
17. compatible with POCO-REAF.

A Zamani type describes semantic meaning and requirements, not the accidental properties of the machine executing the program.

---

4. POCO-REAF Type-System Contract

POCO-REAF means:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

subject to:

- semantic validity;
- target capability;
- available resources;
- declared portability requirements;
- compatibility policy;
- execution feasibility.

The type system MUST NOT require source programs to encode:

- a specific CPU;
- a specific GPU;
- a specific FPGA;
- a specific QPU;
- a specific physical qubit;
- a fixed node count;
- a fixed core count;
- a fixed thread count;
- a fixed memory size;
- a fixed accelerator count;
- a fixed topology;
- a fixed SIMD width;
- a fixed register width;
- a fixed tensor size.

A type may express a requirement.

It must not accidentally become a physical placement decision.

For example:

requires capability("quantum.mid_circuit_measurement")

is semantic/resource intent.

This:

map q0 -> physical_qubit(17)

is physical realization and belongs downstream.

---

5. No Artificial Universal Limits

The type system MUST NOT define universal constants such as:

MAX_QUBITS
MAX_CLASSICAL_BITS
MAX_REGISTER_SIZE
MAX_TENSOR_RANK
MAX_VECTOR_LENGTH
MAX_MATRIX_SIZE
MAX_MEMORY
MAX_THREADS
MAX_CORES
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_AGENTS
MAX_TIMELINES
MAX_CHANNELS
MAX_GENERIC_PARAMETERS
MAX_FUNCTION_PARAMETERS
MAX_RECURSION_DEPTH
MAX_TYPE_DEPTH

as language-semantic restrictions.

An implementation may have:

- parser memory limits;
- compiler memory limits;
- compilation-time limits;
- recursion guards;
- diagnostic budgets;
- execution budgets;
- security budgets;
- target resource limits.

Those are implementation/resource policies, not type-system semantics.

A program can therefore be semantically valid while an implementation reports:

RESOURCE_EXHAUSTED

The compiler must not incorrectly report:

TYPE_ERROR

merely because the implementation lacks enough resources.

---

6. Unbounded Semantic Quantities

Zamani semantic quantities include:

- dimensions;
- cardinalities;
- resource counts;
- tensor ranks;
- symbolic indices;
- iteration domains;
- quantum-register sizes;
- distributed topology sizes;
- generic value parameters;
- compile-time quantities.

These quantities MUST be representable semantically without requiring them to fit in a host "usize".

The semantic model MUST support:

1. arbitrary finite values;
2. symbolic values;
3. constrained values;
4. dependent values where supported;
5. implementation-independent equality and ordering where mathematically defined.

A Rust implementation may use:

- arbitrary-precision integers;
- canonical symbolic expressions;
- interned semantic terms;
- DAG-based expression representations;
- segmented representations.

But a semantic quantity MUST NOT silently overflow.

"usize" is an implementation indexing type.

It is not the universal Zamani cardinality type.

---

7. Formal Typing Judgments

The core typing judgment is:

Γ ; Δ ; Ε ; Κ ; R ⊢ e : T

where:

- "Γ" = lexical/name/type environment;
- "Δ" = ownership and linear-resource environment;
- "Ε" = effect environment;
- "Κ" = capability environment;
- "R" = resource/constraint environment;
- "e" = expression;
- "T" = resulting type.

Statements:

Γ ; Δ ; Ε ; Κ ; R ⊢ s ✓

Declarations:

Γ ; Δ ; Ε ; Κ ; R ⊢ d ✓

Functions:

Γ ; Δ ; Ε ; Κ ; R ⊢ f : FunctionType

Canonical IR:

IR ⊢ valid

A type checker MUST reject a program when a required semantic judgment cannot be established.

---

8. Type Identity

Every semantic type has a canonical identity.

Type identity MUST NOT depend on:

- source formatting;
- source file location;
- compiler memory address;
- host pointer address;
- process ID;
- thread ID;
- physical device ID;
- backend-specific numbering;
- hardware topology;
- compilation order.

Equivalent semantic types MUST have equivalent canonical identities.

Internal implementation identifiers may include:

TypeId
TypeVarId
GenericId
ShapeId
EffectId
CapabilityId
ResourceId
RegionId

These are implementation identities.

They must not accidentally become source-language semantic values.

---

9. Type Categories

Zamani provides one unified type system with the following semantic categories:

Primitive
Numeric
Textual
Unit
Never
Option
Result
Tuple
Array
Slice
Sequence
Map
Set
Record
Struct
Enum
Nominal
Structural
Reference
Pointer
Function
Generic
Parametric
Dependent
Linear
Affine
Resource
Capability
Effect
Quantum
Classical
HDL
HardwareIntent
Distributed
Temporal
Agent
Model
Tensor
Proof
Opaque
Existential
Dynamic

A category does not necessarily require a keyword.

For example:

List<T>

is a collection type even if "List" is implemented as a library/standard semantic type rather than a core keyword.

---

10. Primitive Types

The minimum primitive semantic types are:

Bool
Char
String
Unit
Never

Numeric types are defined separately.

No primitive type may silently acquire host-dependent semantics.

---

11. Boolean

"Bool" has exactly two semantic values:

true
false

Logical operators have deterministic semantics.

For effectful expressions:

a && b
a || b

are short-circuiting according to the expression semantics contract.

The second operand MUST NOT execute when the first operand determines the result.

---

12. Unit

"Unit" represents successful completion without a meaningful result value.

Conceptually:

()

A function returning "Unit" completes normally without producing a semantic result.

---

13. Never

"Never" represents computation that cannot produce a normal value.

Examples:

- explicit non-returning termination;
- trap;
- unrecoverable divergence;
- infinite computation when statically known to be non-returning.

"Never" may coerce to an expected type because the computation never produces a value.

This coercion does not manufacture a value.

---

14. Option

Optional values use:

Option<T>

with semantic constructors:

Some(T)
None

"None" is not a null pointer.

It is an explicit algebraic value.

There is no implicit nullability.

---

15. Result

Fallible computations use:

Result<T, E>

with:

Ok(T)
Err(E)

Recoverable failure must not be represented through undefined behavior.

Compiler and runtime interfaces should prefer explicit result semantics over unchecked failure.

---

16. Integer Types

Zamani integer types are semantic rather than host-dependent.

Conceptual forms:

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

and arbitrary-width semantic integers:

SignedInteger<W>
UnsignedInteger<W>
Integer
Natural

where "W" is a semantic width expression.

The type:

usize

if exposed at all, represents a platform/implementation-sized integer and MUST NOT be used to define universal Zamani cardinalities.

Portable source code should use semantic integer types when width matters.

---

17. Integer Overflow

Integer arithmetic MUST have defined semantics.

Implicit target-dependent wrapping is forbidden.

A fixed-width operation that overflows must have one of these explicitly defined outcomes:

1. compile-time rejection when statically provable;
2. checked runtime failure;
3. explicit wrapping;
4. explicit saturating arithmetic;
5. operation in a wider/unbounded semantic domain.

The meaning of:

+
-
*

must not change merely because a program is compiled for another machine.

---

18. Arbitrary-Precision Integers

"Integer" and "Natural" represent mathematical integer domains.

Their semantic meaning is not limited to Rust primitive widths.

A compiler may represent them using:

- arbitrary-precision storage;
- compiler-managed big integers;
- symbolic values;
- optimized target representations.

The compiler MUST NOT silently wrap them at "u64", "u128", or "usize".

---

19. Floating-Point Types

Zamani supports semantic floating-point types such as:

f16
f32
f64
f128
Float<P>

where "P" represents semantic precision.

Floating-point semantics must define:

- precision;
- rounding;
- exceptional values;
- NaN;
- infinity;
- comparison;
- conversion;
- reproducibility profile.

Where exact reproducibility is required, a strict numerical profile must be selected.

A backend must not silently change semantic precision.

---

20. Complex Types

Complex values are represented conceptually as:

Complex<T>

where "T" is an appropriate real numeric type.

Examples:

Complex<f32>
Complex<f64>
Complex<ArbitraryPrecision>

The type system must preserve the distinction between:

Float
Complex<Float>

because they have different mathematical semantics.

---

21. Character and String

"Char" represents one semantic Unicode scalar value.

"String" represents a sequence of characters.

String representation is implementation-defined.

String type semantics must not depend on:

- machine word size;
- memory allocator;
- operating system;
- pointer width.

---

22. Tuples

For types:

(T1, T2, ..., Tn)

tuple equality is structural.

The tuple arity is a semantic property of the type.

There is no language-level maximum tuple arity.

An implementation may impose compiler resource limits.

---

23. Arrays

An array may be represented as:

Array<T, N>

where:

- "T" is the element type;
- "N" is the semantic cardinality.

"N" may be:

- a literal;
- a constant;
- a generic parameter;
- a symbolic expression;
- a dependent value;
- a runtime-known value where the selected array abstraction permits it.

The type system must distinguish:

logical length
physical allocation
storage representation

---

24. Slices and Dynamic Sequences

A slice represents a view over an existing sequence:

Slice<T>

A dynamically sized sequence may be represented as:

Sequence<T>
List<T>
Vector<T>

The semantic distinction between fixed-size and dynamically sized collections must be preserved.

A dynamic sequence must not acquire a hidden fixed maximum.

---

25. Maps and Sets

Maps:

Map<K, V>

Sets:

Set<T>

require their key/equality/hash semantics to be explicit.

The type system must not assume a particular implementation such as:

- hash table;
- tree;
- distributed map;
- accelerator memory.

Those are implementation choices.

---

26. Shape-Parametric Types

Scientific and mathematical structures use semantic shapes.

Examples:

Vector<N, T>
Matrix<M, N, T>
Tensor<Shape, T>

A shape may contain symbolic dimensions:

Shape<N, M, K>

or dependent expressions.

Shape expressions are semantic terms.

They are not Rust "usize" values by definition.

---

27. Shape Compatibility

For matrix multiplication:

Matrix<M, N, A>
×
Matrix<N, K, B>

is valid because the inner dimensions unify.

The result is:

Matrix<M, K, ResultElementType>

The operation:

Matrix<M, N, A>
×
Matrix<K, P, B>

is valid only if the type constraints establish:

N = K

This checking occurs during semantic analysis.

The parser does not perform shape reasoning.

---

28. Symbolic Dimensions

Dimensions may be symbolic.

For example:

Vector<N, Float>

does not require "N" to be a literal.

"N" may be introduced through:

- a generic parameter;
- a constant;
- a dependent value;
- a symbolic constraint;
- a runtime dimension in an appropriate dynamic type.

The type checker preserves symbolic constraints rather than prematurely converting them to machine integers.

---

29. Tensor Types

A tensor is conceptually:

Tensor<Shape, T>

where "Shape" is a semantic shape.

The type system supports:

- scalar tensors;
- vectors;
- matrices;
- arbitrary-rank tensors;
- symbolic-rank representations where the selected type abstraction supports them;
- dynamic shapes.

There is no language-level maximum tensor rank.

---

30. Generic Types

Zamani supports parametric polymorphism.

Examples:

List<T>
Option<T>
Result<T, E>
Vector<N, T>
Matrix<M, N, T>
Tensor<S, T>

Generic implementation strategy is not semantic.

The compiler may use:

- monomorphization;
- specialization;
- dictionary passing;
- type erasure;
- interpretation;
- JIT;
- AOT compilation.

The source-level meaning remains identical.

---

31. Generic Parameters

Generic parameters may represent:

Type
Value
Shape
Resource
Capability
Effect

Conceptually:

<T>
<N>
<S>
<R>
<C>
<E>

A generic parameter must have an explicit semantic category.

The compiler must not infer that a generic value corresponds to a particular physical machine property.

---

32. Generic Constraints

Generic constraints are semantic predicates.

Examples:

T : Numeric
T : Serializable
T : QuantumCompatible
T : Cloneable
N : ShapeDimension
C : Capability

A generic definition is valid for every instantiation satisfying its declared constraints.

The compiler must not add undocumented target constraints.

---

33. Where Constraints

Constraints may be attached through a "where"-style contract.

Conceptually:

where
    N = M,
    T : Numeric

The constraint set must be preserved through:

AST
→ semantic model
→ canonical IR

Constraints must never be silently discarded.

---

34. Type Aliases

An alias creates another name for the same semantic type.

Conceptually:

type UserId = Integer

does not create a new nominal identity.

Where a distinct type is required, a nominal declaration must be used.

Example:

type UserId = new Integer

or the equivalent canonical nominal-type syntax.

---

35. Nominal Types

A nominal type has its own semantic identity.

Two nominal types with identical structure are not automatically equal.

Example:

type UserId = new Integer
type ProductId = new Integer

"UserId" and "ProductId" are distinct even if both use integer representation.

This prevents accidental interchangeability.

---

36. Structural Types

Structural compatibility may be used where explicitly defined.

For a structural type, compatibility is determined by the required structure and semantic contracts.

Structural compatibility must not accidentally erase nominal identity.

The language must clearly distinguish:

nominal equality

from:

structural compatibility

---

37. Records, Structs, Classes, Interfaces and Traits

The language may expose:

record
struct
class
interface
trait
impl

These must not create independent type systems.

They map into the unified semantic model:

data structure
behavioral contract
implementation relationship
nominal identity

A trait/interface primarily establishes behavioral requirements.

A struct/record/class primarily establishes data/state structure according to its declaration semantics.

---

38. Recursive Types

Recursive types are valid when their semantic representation is well-founded.

Examples:

List<T>
Tree<T>
Graph<T>
Expression
AST

There is no language-level recursion-depth maximum.

Compiler stack/resource exhaustion is an implementation limitation, not a type-system rule.

Recursive types must be represented without requiring infinite eager expansion.

---

39. Function Types

A function type contains all semantically relevant information.

Conceptually:

Fn<
    Parameters,
    Return,
    Effects,
    Resources,
    Capabilities
>

For example:

Fn<(A, B), C, E, R, K>

Two functions with identical parameters and return type are not necessarily semantically identical if their:

- effects;
- resource requirements;
- capabilities;

differ.

---

40. Function Variance

Function compatibility must follow explicit variance rules.

For a function:

Fn<(A), R>

parameter compatibility is contravariant where the relevant type relation permits it.

Return compatibility is covariant where safe.

Effect/resource/capability constraints must not be ignored during function compatibility.

No backend may weaken these rules.

---

41. Async and Concurrent Functions

An asynchronous computation is a semantic computation type.

It must not be defined as:

Fn -> operating_system_thread

Instead, the type describes asynchronous behavior.

The runtime determines whether the computation is realized through:

- threads;
- tasks;
- fibers;
- event loops;
- accelerators;
- distributed execution;
- hardware engines.

---

42. References

References provide access to existing values/resources.

A reference does not imply ownership.

The type system must preserve:

- lifetime;
- aliasing;
- ownership;
- mutability;
- capability constraints.

Invalid references must be rejected before execution.

---

43. Ownership

Zamani distinguishes:

Owned<T>
Borrowed<T>
Shared<T>
Linear<T>
Affine<T>

where these categories are semantically relevant.

Ownership is part of the static resource model.

Ownership must not depend on the physical memory allocator.

---

44. Copyability

A type is copyable only when its semantic contract permits duplication.

Copying a resource-sensitive value must not silently duplicate an external resource.

For example, copying:

Integer

is fundamentally different from copying:

Qubit
DeviceHandle
Socket
Process
Timeline
HardwareResource

---

45. Linear Types

A linear resource must be consumed exactly according to its declared linearity contract.

Linear semantics are particularly important for:

- quantum resources;
- unique hardware handles;
- exclusive resources;
- cryptographic secrets where the chosen security model requires linear use;
- transactional resources.

A linear resource cannot be silently duplicated.

---

46. Affine Types

An affine resource may be used at most once.

An affine value may be consumed before scope exit.

Affine semantics are appropriate where duplication is forbidden but unused destruction is permitted.

---

47. Quantum Types

The quantum type system is target-independent.

Core semantic quantum types include:

Qubit
LogicalQubit
PhysicalQubit
QRegister
QuantumState
Observable
QuantumChannel
QuantumOperation
QuantumCircuit

The exact source spelling is controlled by:

grammar/quantum/
grammar/spec/quantum.md

The semantic boundary is the canonical:

quantum::ir

No second quantum semantic IR may be introduced merely because a source-level quantum construct exists.

---

48. Logical and Physical Qubits

A logical qubit represents a semantic computational resource.

A physical qubit represents a physical realization vocabulary.

These are distinct.

LogicalQubit

does not mean:

physical qubit 0

and:

PhysicalQubit

does not guarantee that a target actually provides such a resource.

Existence and availability are downstream resource/HAL concerns.

The canonical quantum IR already follows this separation and intentionally imports canonical "QubitId" and "PhysicalQubitId" rather than defining duplicate identifiers.

---

49. Quantum Resource Linearity

Qubits are resource-sensitive.

The type system must prevent semantic duplication of a qubit resource.

This does not mean the type system prevents valid quantum operations such as:

- entanglement;
- measurement;
- unitary transformation;
- controlled operations.

It means that the same unique semantic resource cannot be silently copied as though it were an ordinary integer.

---

50. No Fixed Qubit Limit

The type system MUST NOT contain:

MAX_QUBITS
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS

A type such as:

QRegister<N>

is valid for any semantically representable "N".

Actual execution feasibility is determined later from:

resource requirements
+
capabilities
+
hardware state
+
routing
+
scheduling
+
QEC
+
ZQN
+
HAL

---

51. Quantum Register Types

A register can be represented semantically as:

QRegister<N>

where "N" is a semantic cardinality.

"N" may be:

- constant;
- generic;
- symbolic;
- dynamically established.

The type system must not require physical contiguous qubits.

Logical register layout is distinct from physical topology.

---

52. Quantum State Types

Quantum state types must preserve semantic distinction between:

Qubit
QRegister<N>
QuantumState

A "QuantumState" may represent the state associated with a collection of quantum resources without exposing its physical storage layout.

The implementation may use:

- state vectors;
- stabilizer representations;
- tensor networks;
- sparse representations;
- hardware state;
- other valid representations.

The representation is not part of source type identity.

---

53. Quantum Operations

A quantum operation type expresses:

- operation identity;
- input quantum resources;
- classical parameters;
- output/resource effects;
- control conditions;
- required capabilities;
- relevant effects.

The grammar must not enumerate every possible gate as a distinct type.

This permits:

- standard operations;
- user-defined operations;
- future operations;
- vendor operations through explicit interoperability;
- composite operations.

---

54. Quantum Parameters

Angles and other quantum parameters should use semantic types where appropriate.

For example:

Angle
Phase
Frequency
Duration
Amplitude

These are semantic quantities rather than arbitrary floating-point aliases.

A backend may lower them into a supported numerical representation.

---

55. Measurement Types

Measurement is not ordinary copying.

A measurement operation may:

- consume or transform quantum state;
- produce classical information;
- introduce an explicit effect;
- change resource state.

Therefore measurement must be represented by an appropriate effect/type contract.

The result type must explicitly distinguish classical observation from quantum state.

---

56. Classical/Quantum Hybrid Types

Hybrid computation is represented in one unified type system.

A computation may have:

classical inputs
quantum resources
classical outputs
quantum outputs
effects
capabilities
resource requirements

A hybrid function is not a separate language.

Example semantic shape:

Fn<
    (ClassicalInput, QRegister<N>),
    ClassicalOutput,
    Effects,
    Resources,
    Capabilities
>

The exact source syntax belongs to the hybrid grammar.

---

57. Classical Types

Classical computation uses the same base type system for:

- integers;
- floats;
- booleans;
- records;
- arrays;
- tensors;
- functions;
- references;
- resources;
- effects.

The "classical/" grammar adds syntax where necessary but must not introduce an independent type system.

---

58. HDL Types

HDL constructs use the same semantic type system.

Examples include:

Bit
BitVector<N>
Signal<T>
Clock
Reset
Port<T>
Net<T>
Register<T>
Memory<T, Shape>

Widths are semantic parameters.

A declaration such as:

BitVector<N>

must not impose a universal "N".

Physical synthesis determines actual implementation resources.

---

59. Hardware-Intent Types

Hardware intent is represented separately from concrete hardware realization.

Examples:

HardwareResource
ComputeResource
MemoryResource
Accelerator
Interconnect
QuantumDevice

These types represent semantic resource classes.

They do not mean:

GPU #0
CPU core #7
QPU #2
physical qubit #17

unless a downstream realization layer explicitly introduces such identities.

---

60. Resource Types

Resource types describe computational resources.

Examples:

Resource<T>
MemoryResource
ComputeResource
CommunicationResource
QuantumResource
StorageResource
EnergyResource

Resource types are connected to:

grammar/resources/
grammar/hardware/
grammar/compile/
grammar/execution/

The type system establishes semantic relationships.

Resource feasibility remains downstream.

---

61. Capability Types

A capability describes what a target or execution environment can provide.

Examples:

Capability<"quantum.measurement">
Capability<"tensor.compute">
Capability<"distributed.collective">
Capability<"hdl.synthesis">

Capabilities are not hardware identities.

A capability requirement can be satisfied by multiple implementations.

---

62. Requirement vs Capability vs Preference

The type/resource system must distinguish:

requirement
capability
constraint
preference
hint
implementation decision

For example:

requires capability("quantum.mid_circuit_measurement")

is a requirement.

prefer capability("gpu.tensor")

is a preference.

map q0 -> physical_qubit(17)

is a target-specific implementation decision.

These must not be conflated.

---

63. Effect Types

Effects describe observable computational behavior.

Examples include:

IO
State
Mutation
Allocation
Async
Concurrency
Quantum
Measurement
Randomness
Nondeterminism
Network
FileSystem
Device
Hardware
UnsafeExternal

The stable core must not provide an unrestricted "unsafe" escape hatch.

Effects must be represented explicitly where their semantics matter.

---

64. Effect Polymorphism

Functions may be generic over effects.

Conceptually:

Fn<T, E>

where "E" is an effect parameter or effect set.

This permits abstractions that remain portable across execution environments.

A function requiring no effects should not silently acquire them because of a backend.

---

65. Capability-Effect Separation

A capability is not an effect.

For example:

Capability<"quantum.compute">

means the environment can provide a capability.

An effect:

Quantum

describes what the computation does.

A program may require a capability without itself producing the corresponding effect in every control path.

The semantic analyzer must preserve this distinction.

---

66. Temporal Types

Temporal computation may introduce types such as:

Time
Duration
Instant
Interval
Timeline
Event
TemporalState

A temporal type does not prescribe a physical clock implementation.

Clock precision, synchronization and hardware realization are downstream concerns.

---

67. Distributed Types

Distributed computation may use semantic types such as:

Node
Process
Service
Actor
Channel<T>
Message<T>
Replica<T>
Partition<T>
DistributedCollection<T>

The type system must not impose:

MAX_NODES
MAX_PROCESSES

The runtime determines placement and available resources.

---

68. Agent and AI Types

AI/ML features share the common type system.

Semantic types may include:

Tensor<S, T>
Dataset<T>
Model<I, O>
Agent<I, O>
Distribution<T>
Probability<T>
Gradient<T>

Framework-specific types must remain outside the core semantic type system unless deliberately promoted into a stable Zamani abstraction.

---

69. Data Types

Data-domain constructs use:

Schema
Record
Dataset<T>
Stream<T>
Table<T>
Tensor<S, T>
Query<I, O>

Data representation may be:

- local;
- distributed;
- persistent;
- streamed;
- accelerator-resident.

The type does not prescribe storage location unless storage location is itself part of explicit semantic type intent.

---

70. Networking Types

Networking types may include:

Endpoint
Address
Protocol
Socket
Connection
Request
Response
Stream<T>

A network type must not imply a specific physical interface.

The network runtime determines realization.

---

71. Security and Cryptographic Types

Security-sensitive semantic types may include:

Secret<T>
Key
PublicKey
PrivateKey
Signature
Digest
Identity
Credential
CapabilityToken

Secret material must not be implicitly copyable if the selected security contract forbids duplication.

Cryptographic algorithms are semantic operations or library capabilities, not arbitrary type-system keywords.

---

72. Opaque Types

An opaque type hides representation while exposing semantic contracts.

This is essential for portability.

Example:

opaque Device
opaque HardwareResource
opaque ForeignHandle

An opaque type may be implemented differently on different targets while preserving the same source-level contract.

---

73. Existential Types

Where supported, existential types represent:

there exists T satisfying constraints

The hidden representation must not leak across the abstraction boundary unless permitted by the type contract.

Existential implementation is not required to use any particular runtime mechanism.

---

74. Dynamic Types

A dynamic type may exist only as an explicit language feature.

Omitted type information does not mean:

Any

If inference cannot establish a unique type, the compiler must issue an explicit inference diagnostic.

There is no implicit dynamic escape.

---

75. Type Inference

Zamani may support type inference.

Inference must be:

- deterministic;
- constraint-based;
- context-sensitive where required;
- target-independent;
- semantically stable;
- bounded by implementation resources rather than arbitrary language constants.

The same valid source program must not infer different types merely because it is compiled for:

- CPU;
- GPU;
- FPGA;
- QPU;
- simulator;
- distributed target.

---

76. Inference Failure

When inference cannot determine a unique valid type, compilation must fail with an explicit diagnostic such as:

TYPE_INFERENCE_AMBIGUOUS

The compiler must explain:

- unresolved type variable;
- relevant constraints;
- candidate types where useful;
- source span;
- suggested explicit annotation when possible.

---

77. Type Unification

The semantic analyzer uses unification or an equivalent constraint-solving mechanism.

Unification must support:

- ordinary types;
- generic variables;
- symbolic dimensions;
- shape constraints;
- effect variables;
- capability constraints;
- resource constraints where represented at type level.

The implementation must not eagerly expand recursive structures without necessity.

---

78. Subtyping

Subtyping exists only where explicitly defined.

The type system must distinguish:

type equality
type compatibility
subtyping
coercion
conversion
capability satisfaction

These concepts must not be collapsed into one operation.

---

79. Coercions

Implicit coercions must be:

- deterministic;
- semantics-preserving;
- non-lossy unless explicitly defined as safe;
- target-independent.

Potentially lossy conversions require explicit syntax.

Examples include:

Integer → Float
Float → Integer
Wide → Narrow
Exact → Approximate
Quantum → Classical
Resource-owning → Borrowed

where the conversion may lose information or change semantics.

---

80. Numeric Promotion

Numeric promotion must never depend on host architecture.

For example:

i32 + i64

must have one language-defined result.

The compiler may lower the result differently on different targets.

---

81. No Silent Semantic Narrowing

The compiler must not silently narrow:

i128 → i32

or:

Tensor<large_shape> → Tensor<smaller_shape>

or:

QRegister<N> → QRegister<M>

unless a valid explicit semantic transformation exists.

---

82. Type-Level Values

Type-level values may include:

- natural numbers;
- integers;
- symbolic dimensions;
- shapes;
- labels;
- capabilities;
- resource quantities;
- effect sets.

They must have canonical semantic representation.

A type-level value must not be confused with a runtime value.

---

83. Type-Level Arithmetic

Where supported:

N + M
N * M
N = M
N >= M

are semantic constraints.

They must not require host execution of arbitrary source programs during parsing.

The type checker evaluates or reasons about type-level expressions using a safe semantic mechanism.

---

84. Dependent Types

Zamani may support dependent types where the feature is formally promoted.

A dependent type may depend on semantic values such as:

Vector<N, T>
Matrix<M, N, T>
QRegister<N>
BitVector<N>
Tensor<S, T>

The dependent information must remain canonical through semantic lowering.

Dependent typing must not become an implicit hardware-binding mechanism.

---

85. Resource-Dependent Types

A type may express resource requirements where that requirement is semantically meaningful.

For example:

Requires<Capability<C>, T>

or an equivalent canonical representation.

However:

Requires<GPU0, T>

must not be part of the portable semantic type system unless explicitly inside a target-specific realization boundary.

---

86. Type-Level Hardware Intent

Hardware-related type information must describe intent.

Allowed concepts include:

Accelerator<T>
RequiresCapability<C>
MemoryClass<M>
ComputeClass<C>
CommunicationClass<C>

The source language must not silently bind these to physical addresses, device IDs or topology positions.

---

87. Type Equality Across Targets

For POCO-REAF:

Type(program, target A)
==
Type(program, target B)

must hold at the source semantic level unless target-specific conditional compilation or explicit target-dependent semantics are requested.

Backend lowering may differ.

Source meaning must not.

---

88. Conditional Target Specialization

Target-specific specialization may exist through explicit mechanisms.

It must be:

- declared;
- versioned;
- capability-aware;
- semantically checked;
- isolated from the portable core.

A target specialization must never silently change the meaning of the portable program.

---

89. Interoperability Types

Foreign interfaces may introduce opaque or externally defined types.

Examples:

extern type CHandle
extern type QIRValue
extern type HDLSignal

Foreign types must be mapped through an explicit interoperability contract.

They must not leak target-specific assumptions into ordinary Zamani types.

---

90. Type Representation vs Runtime Representation

A semantic type is not a promise about memory layout.

For example:

Tensor<S, Float>

does not promise:

- contiguous memory;
- row-major order;
- GPU memory;
- CPU memory;
- SIMD layout.

Representation becomes fixed only when required by an explicit ABI or interoperability contract.

---

91. ABI and Calling Convention

ABI details are downstream.

A Zamani function type describes semantic calling behavior.

It does not inherently specify:

- stack layout;
- register assignment;
- calling convention;
- binary symbol encoding;
- machine instruction ABI.

Those belong to compiler/backend/interoperability contracts.

---

92. Ownership and Effects in Function Types

Function compatibility must consider:

parameters
returns
ownership
effects
capabilities
resource requirements

For example:

pure fn f(...)

is not semantically equivalent to:

fn f(...) effects IO

even if both return "Unit".

---

93. Resource Consumption

A function may consume resources.

Resource consumption must be explicit in semantic analysis.

Examples:

Qubit
MemoryResource
DeviceHandle
File
Socket
Process
Timeline

The type system verifies ownership/linearity.

Resource availability is checked by resource analysis and target realization.

---

94. Resource Requirements vs Type Errors

A program can be type-correct but resource-infeasible.

Example:

QRegister<N>

may be perfectly valid.

A target with insufficient quantum resources may later report:

INSUFFICIENT_RESOURCE

This must not become:

TYPE_ERROR

The distinction is fundamental.

---

95. Capability Satisfaction

A capability requirement is satisfied if the selected execution environment provides a compatible capability.

The type system records the requirement.

The resource/capability layer determines whether it is satisfiable.

Hardware discovery and HAL remain outside the type checker.

---

96. Type Effects of Measurement

Measurement may transform quantum resources and produce classical values.

Therefore:

measure : Qubit → ClassicalResult

must not be treated as an ordinary pure function unless the language explicitly models the measurement effect.

The type/effect system must preserve:

- quantum effect;
- state transition;
- classical output;
- resource semantics.

---

97. Type Effects of Allocation

Allocation is an effect when allocation has observable resource semantics.

For example:

allocate<T>

may produce:

Resource<T>

and an allocation effect.

The type checker must not assume infinite resources.

It must distinguish:

semantically valid allocation

from:

execution feasibility

---

98. Type Effects of Concurrency

Concurrency introduces effects such as:

Async
Spawn
SharedState
Synchronization
Nondeterminism

The type system must preserve relevant concurrency guarantees.

No source-level type may assume a fixed number of threads.

---

99. Determinism

Type checking itself must be deterministic.

Given the same:

- source;
- specification version;
- dependency versions;
- declared compilation profile;

the semantic type result must be identical.

Hardware availability must not alter whether a purely semantic type is valid.

---

100. Nondeterministic Types

If a type or computation intentionally permits nondeterminism, that must be explicit through:

- effects;
- capability requirements;
- semantic annotations;
- domain contracts.

Nondeterminism must not enter merely because a backend happens to parallelize execution.

---

101. Type Errors

At minimum, the compiler must distinguish:

UNKNOWN_TYPE
TYPE_MISMATCH
TYPE_INFERENCE_AMBIGUOUS
UNDEFINED_TYPE
INVALID_GENERIC_ARGUMENT
UNSATISFIED_TYPE_CONSTRAINT
INVALID_CONVERSION
LOSSY_CONVERSION
INVALID_SHAPE
SHAPE_MISMATCH
INVALID_OWNERSHIP
LINEAR_RESOURCE_REUSED
AFFINE_RESOURCE_REUSED
RESOURCE_NOT_CONSUMED
INVALID_CAPABILITY
UNSATISFIED_CAPABILITY
INVALID_EFFECT
EFFECT_MISMATCH
INVALID_QUANTUM_TYPE
INVALID_QUANTUM_RESOURCE_USE
INVALID_HARDWARE_INTENT
INVALID_DEPENDENCY
RECURSIVE_TYPE_ERROR

Resource exhaustion must not be mislabeled as a type error.

---

102. Diagnostics

Every type diagnostic should provide:

- error code;
- source span;
- primary message;
- relevant type;
- expected type;
- actual type;
- constraints;
- ownership/effect information where relevant;
- related source spans;
- actionable suggestion where possible.

For shape errors:

expected inner dimension N
found K
constraint N = K cannot be established

For ownership errors:

linear quantum resource q was already consumed

---

103. Source Spans

Every semantically meaningful type expression must preserve source-span information from the AST.

This includes:

- generic arguments;
- type parameters;
- dimensions;
- constraints;
- function types;
- resource types;
- capability types.

Type errors must point to source locations rather than synthesized locations.

---

104. AST Contract

The type system consumes the domain-neutral AST.

The AST must preserve:

type syntax
generic parameters
generic arguments
constraints
shape expressions
resource annotations
capability annotations
effect annotations
source spans

The AST must not prematurely lower:

Qubit
Tensor
HardwareResource

into backend-specific representations.

---

105. Type AST vs Semantic Type

The source AST representation and semantic type representation are distinct.

Example:

Vector<N, Float>

may be represented syntactically as:

TypeApplication(
    Vector,
    [N, Float]
)

The semantic analyzer resolves it to a canonical semantic type.

This prevents parser-level assumptions from becoming type semantics.

---

106. Canonical Semantic Type Representation

The semantic model must provide a canonical representation equivalent to:

Type =
    Primitive
  | Integer
  | Float
  | Complex
  | Tuple
  | Array
  | Slice
  | Map
  | Set
  | Function
  | Reference
  | Generic
  | Parametric
  | Dependent
  | Nominal
  | Structural
  | Resource
  | Capability
  | Effect
  | Quantum
  | Tensor
  | HardwareIntent
  | Distributed
  | Temporal
  | Opaque
  | Never
  | Unit
  | Option
  | Result
  | Existential
  | Dynamic

The exact Rust enum is an implementation contract, not source grammar.

---

107. Canonical Quantum IR Integration

Quantum source types must lower into the existing canonical quantum IR type boundary.

The pipeline is:

Zamani quantum type syntax
        ↓
domain-neutral AST type
        ↓
semantic quantum type
        ↓
quantum::ir

No source grammar feature may create:

ZamaniQuantumIR

as a competing semantic layer.

"quantum::ir::core::types" already establishes itself as the canonical target-independent quantum semantic type layer and explicitly excludes hardware topology, routing, scheduling and calibration from its ownership.

---

108. Canonical Qubit Identity

The type system must not define duplicate:

QubitId
PhysicalQubitId

implementations.

The canonical quantum IR identifiers remain authoritative.

The type system defines:

Qubit
PhysicalQubit

as semantic types.

Identity belongs to the canonical quantum IR identity layer.

---

109. Classical IR Integration

Classical semantic types lower into the canonical classical IR boundary.

The type system must preserve:

- integer width;
- floating precision;
- signedness;
- shape;
- ownership;
- effects;
- capability requirements.

The lowering process may choose a target representation later.

---

110. HDL IR Integration

HDL types lower through the HDL/hardware semantic pipeline.

The type system preserves:

- bit width;
- signal semantics;
- timing-relevant types;
- memory shapes;
- interface types.

Synthesis chooses physical implementation.

---

111. Hybrid IR Integration

Hybrid types lower into coordinated classical and quantum semantic representations.

The boundary must preserve:

classical data
quantum resources
measurement results
control dependencies
effects
capabilities
resource requirements

---

112. Resource and Capability Integration

Type checking may produce semantic obligations such as:

RequiresCapability(C)
RequiresResource(R)
RequiresConstraint(C)

These are passed to:

grammar/resources/
grammar/hardware/
compiler resource analysis
HAL
deployment
runtime

The type system does not discover hardware itself.

---

113. Scheduling Integration

Types do not schedule operations.

The type system may establish resource relationships needed by scheduling.

Scheduling subsequently determines:

- ordering;
- timing;
- resource sharing;
- parallel execution;
- delays;
- placement.

---

114. Routing Integration

Types do not route quantum operations.

A type such as:

QRegister<N>

contains no physical topology assumption.

Routing maps logical operations to available physical resources downstream.

---

115. QEC Integration

The type system does not implement quantum error correction.

It may carry semantic information such as:

LogicalQubit
FaultTolerant<T>
ErrorCorrected<T>

where those abstractions are formally defined.

QEC determines the actual encoding and correction strategy.

---

116. ZQN Integration

ZQN remains responsible for fault/noise semantics.

The type system may preserve:

NoiseAware<T>
FaultTolerant<T>
ReliabilityConstraint

when formally defined.

It must not duplicate ZQN's fault model.

---

117. HAL Integration

HAL is responsible for target/device capability and state.

The type system may express:

RequiresCapability<C>

but must not resolve:

physical device = X

during ordinary source type checking.

---

118. Compiler Integration

The compiler must consume validated semantic types.

Compilation stages include:

parse
→ AST
→ structural validation
→ name resolution
→ type inference
→ type checking
→ effect checking
→ resource/capability analysis
→ semantic lowering
→ IR verification
→ optimization
→ target lowering

A backend must never reinterpret an invalid type as valid merely because it can implement it.

---

119. Runtime Integration

Runtime types must correspond to validated semantic contracts.

Runtime may report:

RESOURCE_UNAVAILABLE
CAPABILITY_UNAVAILABLE
DEVICE_FAILURE
EXECUTION_FAILURE

when a semantically valid program cannot currently execute.

Those are not necessarily type errors.

---

120. Serialization

Canonical semantic types used across serialization boundaries must have stable representations.

Serialization must not depend on:

- pointer addresses;
- hash-map iteration order;
- compiler memory layout;
- process-local IDs.

Versioned type schemas must be used where persistent representation is required.

---

121. Type Versioning

Existing type meanings must not silently change.

Breaking type changes require:

- language-version change;
- compatibility declaration;
- migration rule;
- diagnostics;
- explicit deprecation where appropriate.

Additive type features should not alter existing type equality.

---

122. Generic Compatibility

A generic type parameter's semantic meaning must remain stable.

Adding a new implementation strategy must not alter the generic contract.

For example:

Tensor<S, T>

must retain the same semantic shape/type meaning whether implemented on:

- CPU;
- GPU;
- FPGA;
- distributed cluster;
- quantum-classical accelerator;
- future target.

---

123. Type Hashing and Interning

Implementations may intern types for efficiency.

If type interning is used:

- IDs must be compiler-local;
- canonical semantic equality must remain independent of allocation order;
- hash randomization must not change semantic equality;
- serialized semantic identity must not use pointer identity.

---

124. Recursion and Cycles

The semantic type graph may contain cycles.

The implementation must represent recursive types through stable references or equivalent indirection.

It must not recursively allocate an infinite structure.

No universal "MAX_TYPE_DEPTH" may be encoded into language semantics.

---

125. Compiler Resource Limits

Compiler limits are permitted.

Examples:

maximum memory available to compiler
maximum diagnostic count
maximum inference work budget
maximum compilation time
maximum recursion guard

These must be represented as implementation policies.

They must not alter the semantic definition of the type system.

---

126. Scalability Contract

The type system must scale conceptually from:

one value

to:

one qubit

to:

large quantum systems

to:

large tensor systems

to:

large distributed systems

to:

heterogeneous systems

to:

future computational substrates

subject only to actual semantic and resource constraints.

No artificial universal maximum may be introduced.

---

127. Tiny-System Contract

A minimal program must not require heavyweight type declarations.

Examples:

let x = 1;

let q = qubit();

when supported by the corresponding source contracts.

Inference should resolve simple programs without unnecessary annotations.

---

128. Large-System Contract

Large programs must not require changing type semantics merely because:

- the number of resources grows;
- tensor dimensions grow;
- node count grows;
- quantum register size grows;
- module count grows;
- generic instantiation count grows.

The same semantic type rules apply at every scale.

---

129. Infinite vs Unbounded

Zamani must distinguish:

unbounded semantic domain

from:

actually infinite runtime allocation

For example:

Integer

may represent an unbounded mathematical integer domain.

This does not mean a machine has infinite memory.

Likewise:

QRegister<N>

may have arbitrary finite "N".

It does not mean a runtime can allocate infinitely many qubits.

The language provides scalable semantics; resources determine realizability.

---

130. Compile-Once Principle

A source program should not need type rewrites merely because it moves from:

small CPU

to:

large CPU

or:

CPU → GPU
GPU → FPGA
FPGA → QPU
QPU → simulator
single node → distributed system

provided the target satisfies the semantic requirements.

---

131. Compile-Time Specialization

Specialization may optimize a type for a target.

It must preserve semantic equivalence.

For example:

Tensor<S, F64>

may become:

SIMD
GPU tensor
distributed tensor
accelerator tensor

internally.

The source type remains:

Tensor<S, F64>

---

132. Type-Level Capability Negotiation

Capability negotiation is downstream of core type formation.

A type can establish:

requires capability C

but the target resolver determines:

provided by target A
provided by target B
not available

This supports POCO-REAF.

---

133. Hardware Growth

If a machine gains:

- more CPUs;
- more GPUs;
- more QPUs;
- more memory;
- more nodes;
- more accelerators;

the type system must not require source changes merely to use the additional capacity.

Scaling decisions belong to:

- compiler;
- scheduler;
- runtime;
- deployment;
- resource manager.

---

134. Hardware Shrinkage

Likewise, moving to a smaller target must not cause type-system corruption.

The result should be one of:

valid execution

or:

insufficient resources

or:

missing capability

rather than a false type error.

---

135. Security

The type system must preserve security boundaries.

Sensitive resources may be:

- non-copyable;
- affine;
- linear;
- opaque;
- capability-controlled.

Type checking must not expose secrets through implicit conversions.

---

136. No Unsafe Escape Hatch

The core stable language must not require an unrestricted unsafe type operation.

Rust implementation code for the type checker and semantic model must use:

#![forbid(unsafe_code)]

where applicable.

The implementation baseline is:

Rust 1.97
Rust 1.97.1
Rust 2021

No nightly-only feature is required by this specification.

---

137. Safe Rust Implementation Contract

The type-system implementation must use safe Rust abstractions.

Allowed implementation mechanisms include:

- enums;
- structs;
- traits;
- generics;
- "Arc";
- "Box";
- "Vec";
- "BTreeMap";
- "BTreeSet";
- safe indexing/access patterns;
- checked arithmetic;
- explicit error types;
- immutable semantic structures.

Unsafe pointer manipulation is prohibited.

---

138. Semantic Arithmetic in Rust

Rust implementation code must distinguish:

host indexing arithmetic

from:

semantic arithmetic

For semantic values requiring arbitrary size, the implementation must not rely on unchecked native integer arithmetic.

Any fixed-width implementation field must have a documented semantic bound or be used only as an implementation identifier.

---

139. Existing Quantum IR Correction Requirement

The current canonical quantum IR type implementation is architecturally sound in its separation of semantic types from hardware, and it explicitly prohibits unsafe code.

However, semantic variants equivalent to:

Arbitrary(u64)

must not be interpreted as "infinite precision."

They mean only:

arbitrary within a u64-encoded width descriptor

if retained as an implementation representation.

For true unbounded semantic dimensions, the canonical representation should instead use one of:

SymbolicNat
Natural
ShapeExpr
ConstNat
TypeLevelValue

or the repository's equivalent canonical representation.

This correction applies to:

- integer widths;
- vector lengths;
- bit widths;
- tensor dimensions;
- quantum register sizes;
- resource counts.

---

140. Existing Platform-Sized Types

The current quantum IR contains platform-sized concepts such as "Size".

Such types are acceptable only when explicitly documented as:

implementation/platform representation

They must never become:

universal semantic cardinality

For example:

usize

may index a Rust vector internally.

It must not define the maximum semantic number of qubits.

---

141. No Duplicate Qubit IDs

The type system must not introduce another:

QubitId
PhysicalQubitId

The canonical quantum IR identity layer owns those identifiers.

This preserves the existing repository architecture.

---

142. Type-to-Grammar Integration

The type grammar under:

grammar/types/

owns syntax such as:

TypeExpression
GenericArguments
TypeParameters
FunctionType
ReferenceType
ArrayType
TupleType
ResourceType
CapabilityType
QuantumType
ShapeType

The grammar does not determine semantic validity.

For example:

Matrix<A, B, Float>

can be syntactically valid while semantic checking determines whether "A" and "B" are valid dimension expressions.

---

143. Type-to-Expression Integration

The type system consumes expressions when they occur in:

- generic value parameters;
- array dimensions;
- shape expressions;
- constraints;
- dependent types.

The expression grammar owns expression syntax.

The type system owns whether an expression is valid in a type-level position.

---

144. Type-to-Declaration Integration

Declarations introduce:

- named types;
- aliases;
- generic parameters;
- constraints;
- implementations;
- traits/interfaces.

The declaration grammar owns syntax.

The type system owns:

- binding;
- identity;
- compatibility;
- implementation satisfaction.

---

145. Type-to-Function Integration

Function declarations must lower to complete function types containing all semantically relevant:

parameters
returns
generics
effects
resources
capabilities
ownership
constraints

The function grammar must not independently redefine function type semantics.

---

146. Type-to-Effects Integration

Effects are attached to computation types.

The effect grammar defines source syntax.

The effect system determines:

effect identity
effect compatibility
effect polymorphism
effect propagation

The type checker verifies effect requirements.

---

147. Type-to-Resources Integration

Resource syntax belongs under:

grammar/resources/

Type semantics consume the resulting resource contracts.

The type system determines whether a value is:

resource-sensitive
linear
affine
owned
borrowed
shared

Resource availability remains downstream.

---

148. Type-to-Hardware Integration

Hardware types express target-independent intent.

They must not expose physical implementation accidentally.

The hardware grammar owns syntax for:

- capabilities;
- resource classes;
- constraints;
- topology intent;
- deployment intent.

The type system ensures those constructs are type-consistent.

---

149. Type-to-Quantum Integration

Quantum syntax comes from:

grammar/quantum/

Quantum semantics come from:

grammar/spec/quantum.md

Quantum types lower into:

quantum::ir

No duplicate quantum semantic boundary is permitted.

---

150. Type-to-HDL Integration

HDL syntax comes from:

grammar/hdl/

Hardware intent comes from:

grammar/hardware/

Type semantics remain unified.

An HDL signal type must remain distinguishable from an ordinary software integer even if both are represented using bits internally.

---

151. Type-to-Distributed Integration

Distributed syntax comes from:

grammar/distributed/

The type system provides:

Node
Process
Actor
Channel<T>
Distributed<T>
Replica<T>

where semantically required.

Physical placement remains downstream.

---

152. Type-to-AI Integration

AI syntax comes from:

grammar/ai/

The type system provides general semantic constructs such as:

Tensor<S, T>
Model<I, O>
Dataset<T>
Agent<I, O>
Distribution<T>

Framework-specific details must remain outside the language core.

---

153. Type-to-Data Integration

Data syntax comes from:

grammar/data/

The type system supplies the common foundation.

A dataset may be:

Dataset<T>

without requiring a specific database engine.

---

154. Type-to-Networking Integration

Networking syntax comes from:

grammar/networking/

The type system provides semantic network types.

The implementation may lower them to:

- sockets;
- RPC;
- message queues;
- RDMA;
- accelerator links;
- future transports.

The source type remains portable.

---

155. Type-to-Security Integration

Security syntax comes from:

grammar/security/

Security-sensitive values must preserve their ownership and capability requirements.

No backend may silently weaken a security type.

---

156. Type-to-Interoperability Integration

Interoperability types come from:

grammar/interoperability/

Foreign representations must be explicitly marked.

Canonical Zamani types must not accidentally become ABI-dependent.

---

157. Type-to-Dialect Integration

A dialect may introduce new types.

Every dialect-defined type must declare:

type identifier
version
semantic definition
source syntax
AST representation
compatibility
IR mapping
capabilities
effects
resource requirements

A dialect must not redefine the meaning of an existing stable core type.

---

158. Type-to-Macro Integration

Macros may generate type syntax.

Macro expansion must occur before final semantic type checking.

Generated type syntax is subject to the same type rules as handwritten source.

Macros must not bypass:

- ownership;
- type checking;
- effect checking;
- capability checking;
- resource checking.

---

159. Type-to-Metaprogramming Integration

Metaprogramming may inspect or construct types.

The semantic type identity must remain canonical.

Reflection must not depend on compiler memory addresses or backend implementation details.

---

160. Type Feature Completion Contract

A type-system feature is not complete until all of the following exist:

LEXICAL SUPPORT
      ↓
SYNTAX
      ↓
AST REPRESENTATION
      ↓
TYPE SEMANTICS
      ↓
TYPE INFERENCE
      ↓
DIAGNOSTICS
      ↓
CANONICAL SEMANTIC REPRESENTATION
      ↓
IR LOWERING
      ↓
IR VERIFICATION
      ↓
COMPILER INTEGRATION
      ↓
RUNTIME/TARGET INTEGRATION
      ↓
POSITIVE TESTS
      ↓
NEGATIVE TESTS
      ↓
BOUNDARY TESTS
      ↓
SCALABILITY TESTS
      ↓
DETERMINISM TESTS
      ↓
COMPATIBILITY TESTS

A grammar rule alone does not complete a type feature.

---

161. Required Tests

The type system must have tests covering:

Primitive

- boolean;
- character;
- string;
- unit;
- never.

Numeric

- signed integers;
- unsigned integers;
- floating point;
- complex;
- overflow;
- conversion;
- promotion.

Generic

- generic functions;
- generic types;
- generic constraints;
- generic inference.

Shapes

- vectors;
- matrices;
- tensors;
- symbolic dimensions;
- mismatched dimensions;
- dynamic dimensions.

Ownership

- borrowing;
- move;
- copy;
- linear;
- affine;
- resource consumption.

Quantum

- logical qubit;
- physical qubit;
- registers;
- measurement;
- quantum operations;
- resource uniqueness;
- hybrid computation.

Hardware

- capabilities;
- resource requirements;
- target-independent intent;
- unavailable capability.

Distributed

- channels;
- nodes;
- processes;
- distributed collections;
- arbitrary topology sizes.

AI/data

- tensors;
- datasets;
- models;
- agents;
- distributions.

---

162. Negative Tests

The following must fail:

integer assigned to incompatible quantum type

matrix with incompatible dimensions

linear quantum resource copied

affine resource consumed twice

missing generic constraint

ambiguous inferred type

invalid conversion

invalid capability requirement

invalid effect requirement

nominal types used interchangeably without conversion

invalid dependent constraint

---

163. Boundary Tests

Boundary tests must include:

zero-dimensional semantic cases where permitted
one-element structures
large finite dimensions
very large integer widths
large generic structures
deeply nested valid types
recursive types
large quantum registers
large tensor shapes
large distributed resource descriptions

The test suite must not define a fake maximum merely because a fixture uses one.

---

164. Scalability Tests

Scalability tests must verify that the semantic model does not contain artificial limits.

Examples:

large Vector<N, T>
large Matrix<M, N, T>
large Tensor<S, T>
large QRegister<N>
large distributed topology
large generic instantiation set
large nested type graph

The tests may use configured compiler resource budgets.

Those budgets must be reported as implementation limits, not language limits.

---

165. Determinism Tests

Given identical source and specification configuration:

type checking result
type identities
constraint results
diagnostic ordering
semantic type hashes

must be deterministic.

Hash-map iteration order must not affect semantic results.

---

166. Cross-Target Tests

The same source type program should be checked against:

CPU
GPU
FPGA
QPU
simulator
distributed target
future/opaque target

where supported.

The source semantic type must remain stable.

Only target feasibility and lowering may differ.

---

167. Compatibility Tests

Compatibility tests must verify:

old valid program remains valid

unless a deliberate breaking language version says otherwise.

A change to:

type equality
generic compatibility
ownership
quantum resource semantics
effect semantics

is a potentially breaking change and must be versioned.

---

168. Hard-Coding Audit

The type-system implementation and specification must be automatically audited for accidental machine limits.

Suspicious semantic constructs include:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_TENSOR_RANK
MAX_VECTOR_LENGTH
MAX_REGISTER_SIZE
MAX_TIMELINES

A fixed value is permitted only when it is explicitly:

- a language semantic constant;
- a test fixture;
- a diagnostic budget;
- an implementation limit;
- a target-specific resource description.

It must not masquerade as a universal type-system limit.

---

169. What Counts as a Hard-Coded Limit

This is valid:

u32

because the width is the semantic definition of the type.

This is not valid:

QRegister supports at most 1024 qubits

because that is a hardware/resource constraint.

This is valid:

Matrix<1024, 1024, Float>

because "1024" is program data/type-level intent.

This is invalid:

Matrix dimensions may never exceed 1024

as a universal language rule.

---

170. Error Classification

The compiler must distinguish at least:

TYPE_INVALID
TYPE_INFERENCE_FAILED
TYPE_CONSTRAINT_UNSATISFIED
TYPE_CONVERSION_INVALID
RESOURCE_UNAVAILABLE
CAPABILITY_UNAVAILABLE
EFFECT_UNSUPPORTED
TARGET_UNSUPPORTED
IR_INVALID

These errors must not be collapsed into one generic type failure.

---

171. Documentation Integration

"grammar/grammar.md" should document the implementation status of the type system.

"grammar/Zamani-Grammar.md" may contain proposed type constructs.

Neither may silently redefine this specification.

"grammar/types/README.md" should map syntax files to this normative specification.

"grammar/specification/types.md", if retained as a broader human-readable specification, should reference this file rather than defining conflicting rules.

---

172. Implementation File Integration

The semantic type implementation should be independently completable.

The type-system implementation must establish its contracts before consumers are updated.

Recommended conceptual dependency order:

type identity
    ↓
semantic scalar types
    ↓
type expressions
    ↓
generic parameters
    ↓
shape expressions
    ↓
composite types
    ↓
ownership/resource qualifiers
    ↓
effects
    ↓
capabilities
    ↓
function types
    ↓
quantum types
    ↓
domain types
    ↓
type compatibility
    ↓
inference
    ↓
semantic validation
    ↓
IR lowering

No consumer should need to reinterpret an already-completed type definition.

---

173. Stable Public Semantic Contracts

Once a semantic type is consumed by another IR module, its meaning becomes a compatibility contract.

Existing variants must not be silently repurposed.

New variants must be:

- additive;
- versioned where required;
- documented;
- tested;
- lowered to canonical IR.

---

174. No Backend Leakage

The type system must not import or depend on:

LLVM
MLIR
QIR
CUDA
ROCm
OpenQASM
vendor QPU APIs
FPGA vendor APIs
specific CPU ISA
specific GPU architecture
specific physical topology

except through explicitly defined interoperability/target-lowering contracts.

OpenQASM and QIR are interoperability representations, not the canonical Zamani type system.

---

175. No Quantum IR Duplication

The type system must not create a second quantum IR merely to accommodate source syntax.

The architecture remains:

Zamani source
    ↓
domain-neutral AST
    ↓
semantic quantum types
    ↓
quantum::ir

This preserves the repository's existing canonical quantum boundary.

---

176. No Hardware Type Explosion

The type system must not create:

NvidiaGpuType
AmdGpuType
IntelGpuType
IBMQubitType
RigettiQubitType
FPGA_X_Type
CPU_X_Type

as core portable types.

Hardware-specific capabilities belong to:

capability
resource
hardware intent
interoperability
dialect
backend

as appropriate.

---

177. Future-Proofing

The type system must be extensible to future computational paradigms.

A future domain must be able to introduce:

new semantic types
new capabilities
new effects
new resources
new lowering contracts

without modifying the meaning of existing core types.

The extension must declare its integration contracts.

---

178. Production Readiness Criteria

This file is complete only when:

- [ ] type authority is unambiguous;
- [ ] type syntax is delegated to "grammar/types/";
- [ ] AST integration is defined;
- [ ] semantic representation is defined;
- [ ] IR integration is defined;
- [ ] quantum integration is defined;
- [ ] resource integration is defined;
- [ ] capability integration is defined;
- [ ] effect integration is defined;
- [ ] compiler integration is defined;
- [ ] runtime integration is defined;
- [ ] interoperability is defined;
- [ ] generic semantics are defined;
- [ ] shape semantics are defined;
- [ ] ownership semantics are defined;
- [ ] linear/affine semantics are defined;
- [ ] inference is defined;
- [ ] conversion rules are defined;
- [ ] diagnostics are defined;
- [ ] scalability rules are defined;
- [ ] hard-coding rules are defined;
- [ ] no artificial hardware limits exist;
- [ ] positive tests exist;
- [ ] negative tests exist;
- [ ] boundary tests exist;
- [ ] scalability tests exist;
- [ ] determinism tests exist;
- [ ] compatibility tests exist;
- [ ] safe Rust implementation is possible;
- [ ] "unsafe" Rust is prohibited;
- [ ] canonical quantum IR remains the quantum semantic boundary.

---

179. Final Type-System Invariant

The central invariant of Zamani is:

«A type describes what a computation means and what semantic guarantees it requires; it does not prescribe the machine on which that computation must run.»

Therefore:

Type
  ≠
Machine

Type
  ≠
Hardware topology

Type
  ≠
Physical allocation

Type
  ≠
Scheduling decision

Type
  ≠
Routing decision

Type
  ≠
Calibration

Type
  ≠
QEC implementation

Type
  ≠
ZQN implementation

Instead:

Type
  =
semantic contract
+
constraints
+
ownership
+
effects
+
capabilities
+
resource intent

and:

semantic contract
        ↓
canonical IR
        ↓
optimization
        ↓
routing / scheduling / resilience
        ↓
QEC / ZQN where applicable
        ↓
HAL
        ↓
target realization

This is the type-system foundation required for Zamani to scale from the smallest useful computation to arbitrarily large finite computations subject only to actual resources, while preserving the Program Once, Compile Once, Run Everywhere, Anywhere, Forever objective.

---

180. Integration Summary

This file integrates with the repository as follows:

Area| Owner| Relationship to this file
Lexical syntax| "grammar/spec/lexical.md" / "grammar/lexer/"| Supplies canonical tokens
General syntax| "grammar/spec/syntax.md"| Supplies type-expression structure
Type syntax| "grammar/types/"| Owns source grammar
Expressions| "grammar/expressions/"| Supplies type-level expressions
Declarations| "grammar/declarations/"| Introduces named types
Functions| "grammar/functions/"| Defines function syntax
Effects| "grammar/effects/"| Defines effect syntax
Resources| "grammar/resources/"| Defines resource intent
Hardware| "grammar/hardware/"| Defines target-independent hardware intent
Classical| "grammar/classical/"| Uses common type system
Quantum| "grammar/quantum/"| Uses quantum semantic types
Hybrid| "grammar/hybrid/"| Combines classical and quantum types
HDL| "grammar/hdl/"| Uses common type system
Distributed| "grammar/distributed/"| Uses distributed semantic types
AI| "grammar/ai/"| Uses tensor/model/agent types
Data| "grammar/data/"| Uses collection/tensor/schema types
Networking| "grammar/networking/"| Uses endpoint/channel types
Security| "grammar/security/"| Uses capability/secret/key types
Interoperability| "grammar/interoperability/"| Maps foreign types
Dialects| "grammar/dialects/"| Adds controlled extensions
AST| "src/frontend/ast/"| Represents source type structure
Parser| "src/parser.rs"| Produces AST syntax
Semantic analysis| semantic/type checking| Implements this contract
Canonical quantum semantics| "quantum::ir"| Canonical quantum boundary
Optimization| compiler/IR passes| Consumes validated types
Routing| quantum routing| Physical realization
Scheduling| scheduling| Timing/resource realization
QEC| resilience/QEC| Error-correction realization
ZQN| ZQN subsystem| Fault/noise semantics
HAL| hardware abstraction| Capability/device realization
Runtime| execution/runtime| Executes lowered program

---

181. Final Rule

No future Zamani type feature is considered production-ready merely because a type can be written in source code.

It is production-ready only when:

SPECIFICATION
    ↓
SYNTAX
    ↓
LEXER
    ↓
PARSER
    ↓
AST
    ↓
TYPE SEMANTICS
    ↓
INFERENCE
    ↓
OWNERSHIP / EFFECT / CAPABILITY ANALYSIS
    ↓
RESOURCE ANALYSIS
    ↓
CANONICAL SEMANTIC MODEL
    ↓
IR
    ↓
IR VERIFICATION
    ↓
COMPILER
    ↓
RUNTIME / TARGET
    ↓
TESTS

all agree on exactly the same meaning.

That contract is mandatory for the Zamani type system.