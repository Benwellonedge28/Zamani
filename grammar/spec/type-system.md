Zamani Type System Specification

Path: "grammar/spec/type-system.md"
Language: Zamani
Status: Normative
Specification role: Canonical type-system semantic contract
Implementation baseline: Rust 1.97 or later
Rust edition: 2021
Rust safety: Safe Rust only; production Zamani compiler code MUST NOT use "unsafe"
Architecture: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scope: Source types, semantic typing, genericity, constraints, ownership, linearity, affinity, dependent typing, associated types, type-level values, refinements, effects/capabilities/resource integration, quantum typing, hardware/HDL typing, interoperability, diagnostics, determinism, scalability, compatibility, and IR integration

---

0. Document Contract

0.1 Purpose

This document is the normative specification for the Zamani type system.

It defines:

- what a Zamani type means;
- how source-level type expressions are interpreted;
- how types are formed;
- how types are resolved;
- how type identity works;
- how type equality works;
- how compatibility works;
- how generic parameters work;
- how generic bounds work;
- how associated types work;
- how type classes/interfaces work;
- how type-level values work;
- how shapes and dimensions work;
- how refinement constraints work;
- how dependent types work;
- how ownership interacts with types;
- how linear and affine resources are typed;
- how references and lifetimes are typed;
- how effects interact with function types;
- how capabilities and resources constrain typed computation;
- how contracts interact with types;
- how policies constrain type-dependent operations;
- how quantum values are typed;
- how classical, HDL, hardware, distributed, networking, AI/data, and hybrid domains use the same type foundation;
- how types are represented in the canonical AST;
- how types become semantic types;
- how validated types reach canonical IR;
- how type semantics remain independent of target hardware;
- how type semantics remain scalable without artificial machine ceilings;
- how the type system is implemented safely in Rust;
- how production conformance is established.

The type system is a semantic layer.

It is not a hardware allocator, scheduler, router, runtime, calibration system, QEC implementation, or device-discovery system.

---

1. Normative Language

The following words have normative meaning.

MUST

A mandatory requirement.

MUST NOT

A prohibited behavior.

SHOULD

A strong recommendation. Deviations require documented justification.

SHOULD NOT

Normally prohibited unless documented justification exists.

MAY

Permitted behavior.

IMPLEMENTATION-DEFINED

Determined by the implementation but required to be documented and stable for the applicable implementation profile.

RESOURCE-DEPENDENT

Dependent on resources available to a compiler or execution environment.

TARGET-DEPENDENT

Dependent on the selected realization target.

SEMANTICALLY INVALID

Violates the language's type or semantic rules.

RESOURCE-UNSATISFIABLE

The program is semantically meaningful, but the requested realization cannot satisfy its resource requirements.

CAPABILITY-UNAVAILABLE

The requested realization lacks a required capability.

UNREPRESENTABLE

A valid semantic meaning cannot be represented by the selected realization under the requested constraints.

UNDEFINED BEHAVIOR

Behavior for which the language supplies no semantic definition.

Zamani MUST minimize undefined behavior.

Ordinary valid programs MUST NOT depend on undefined behavior.

---

2. Authority and Repository Integration

The authoritative relationship is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ├── language.md
        ├── syntax.md
        └── domain specifications
        │
        ▼
grammar/spec/type-system.md
        │
        ├── grammar/types/
        ├── grammar/declarations/
        ├── grammar/functions/
        ├── grammar/expressions/
        ├── grammar/resources/
        ├── grammar/effects/
        ├── grammar/validation/
        ├── grammar/policies/
        └── grammar/compatibility/
        │
        ▼
grammar/Zamani.g4
        │
        ▼
canonical lexer/parser
        │
        ▼
src/frontend/ast/
        │
        ▼
structural validation
        │
        ▼
name resolution
        │
        ▼
type resolution
        │
        ▼
constraint solving
        │
        ├── ownership
        ├── effects
        ├── capabilities
        ├── resources
        ├── contracts
        └── policies
        │
        ▼
canonical semantic model
        │
        ├── classical semantic representation
        ├── quantum semantic representation
        ├── HDL/hardware semantic representation
        └── other domain representations
        │
        ▼
canonical IR boundaries
        │
        ├── Classical IR
        └── quantum::ir
        │
        ▼
optimization
        │
        ▼
lowering
        │
        ▼
routing / scheduling / resilience / QEC
        │
        ▼
ZQN
        │
        ▼
HAL
        │
        ▼
target realization

2.1 Authority rules

This document is authoritative for type semantics.

"grammar/types/" owns type syntax.

"grammar/Zamani.g4" owns grammar composition only.

The canonical frontend "TypeExpr" owns the source-level AST representation.

Semantic analysis owns type resolution and validation.

Canonical IR owns validated lowered semantics.

"quantum::ir" remains the canonical quantum semantic boundary.

"grammar/grammar.md" records implementation/conformance status.

Historical or aspirational documentation MUST NOT silently override this document.

If two normative documents conflict, the conflict MUST be resolved explicitly through the language-versioning process.

---

3. Ownership Contract

3.1 This file owns

This specification owns:

- type meaning;
- type formation;
- type identity;
- type equality;
- type compatibility;
- type normalization;
- type inference rules;
- generic semantics;
- generic bounds;
- associated type semantics;
- type-class/interface constraints;
- variance;
- type-level values;
- shape semantics;
- refinement semantics;
- dependent type semantics;
- ownership typing;
- reference typing;
- lifetime relationships;
- linearity;
- affinity;
- resource-sensitive typing;
- quantum type semantics;
- hardware-intent type semantics;
- domain-neutral type composition;
- type-level computation requirements;
- type conversion semantics;
- coercion semantics;
- subtyping semantics where supported;
- diagnostics required for type failures;
- type-system determinism;
- type-system scalability;
- type-system compatibility.

3.2 This file does not own

This file does not own:

- lexical token definitions;
- parser implementation;
- concrete ANTLR syntax;
- AST storage implementation;
- source locations;
- runtime object layouts;
- ABI layouts;
- target instruction selection;
- register allocation;
- physical memory placement;
- physical qubit placement;
- quantum routing;
- scheduling;
- calibration;
- pulse generation;
- QEC implementation;
- ZQN implementation;
- hardware discovery;
- target discovery;
- resource inventory;
- device selection;
- runtime scheduling.

Those responsibilities belong to their respective subsystems.

---

4. Fundamental Invariant

The central type-system invariant is:

«A type describes semantic meaning and valid relationships between values and computations. A type does not prescribe the physical machine on which the computation must execute.»

Therefore:

Type
≠ Machine

Type
≠ Physical Device

Type
≠ Hardware Topology

Type
≠ Physical Address

Type
≠ Register Allocation

Type
≠ Scheduling Decision

Type
≠ Routing Decision

Type
≠ Calibration

Type
≠ QEC Implementation

Instead:

Type
=
semantic identity
+
validity conditions
+
relationships
+
constraints
+
ownership
+
effects where applicable
+
resource/capability requirements where applicable

---

5. POCO-REAF Type-System Contract

POCO-REAF means:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Forever

subject to:

- semantic validity;
- declared requirements;
- available capabilities;
- available resources;
- compatibility policy;
- target feasibility;
- implementation policy.

POCO-REAF does not mean that every physical machine can execute every program.

A target that lacks a required capability MAY reject a program.

A target MUST NOT silently change the program's type meaning merely to make execution possible.

For example:

requires capability("quantum.measurement")

is a semantic requirement.

It does not mean:

use QPU #7

Likewise:

QRegister<N>

does not mean:

allocate physical qubits 0 through N-1

Physical allocation occurs downstream.

---

6. Scalability and the Meaning of "Unbounded"

Zamani type semantics MUST be open-ended.

The language MUST NOT impose artificial finite ceilings on:

- number of type parameters;
- number of generic arguments;
- tuple arity;
- function parameter count;
- type nesting;
- array cardinality;
- tensor rank;
- tensor dimensions;
- quantum-register cardinality;
- distributed topology size;
- number of devices;
- number of nodes;
- number of actors;
- number of channels;
- memory size;
- storage size;
- network size;
- accelerator count.

"Unbounded" means:

«No artificial finite maximum is imposed by the language semantics where the underlying semantic domain has no such maximum.»

It does not mean that physical computers possess infinite resources.

A compiler MAY impose an invocation-specific resource budget for:

- memory;
- CPU time;
- diagnostic output;
- recursion;
- type-solving work;
- type-level evaluation;
- AST size;
- intermediate representation size.

Such a budget is an implementation policy.

It MUST NOT redefine the language's semantic type universe.

A semantically valid type may therefore fail compilation with:

COMPILER_RESOURCE_EXHAUSTED

without becoming:

TYPE_INVALID

---

7. No Universal Hardware Constants

The type system MUST NOT define language-semantic constants such as:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ACCELERATORS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_AGENTS
MAX_CHANNELS
MAX_GENERIC_PARAMETERS
MAX_TUPLE_ARITY
MAX_FUNCTION_PARAMETERS
MAX_TYPE_DEPTH

A number appearing in a program is program semantics.

For example:

Vector<1024, Float>

is valid type-level information.

The number "1024" is not a language capacity declaration.

---

8. Semantic Quantities

Type-level quantities include:

- cardinalities;
- dimensions;
- widths;
- precisions;
- ranks;
- symbolic indices;
- resource quantities;
- timing quantities;
- capacities;
- iteration bounds;
- generic value parameters;
- proof parameters.

A semantic quantity MUST NOT silently overflow.

The implementation MUST distinguish:

semantic quantity

from:

host implementation index

"usize" MAY be used internally for indexing Rust collections.

"usize" MUST NOT define a universal Zamani cardinality.

Where a semantic quantity can exceed host integer width, the implementation MUST use an appropriate representation such as:

- arbitrary-precision integers;
- canonical symbolic expressions;
- interned terms;
- DAG representations;
- segmented representations;
- another explicitly specified representation.

---

9. Core Typing Judgment

The conceptual typing judgment is:

Γ ; Δ ; Ε ; Κ ; R ; C ⊢ e : T

where:

Γ = name/type environment
Δ = ownership/resource environment
Ε = effect environment
Κ = capability environment
R = resource environment
C = semantic constraints
e = expression
T = resulting type

Statements:

Γ ; Δ ; Ε ; Κ ; R ; C ⊢ s ✓

Declarations:

Γ ; Δ ; Ε ; Κ ; R ; C ⊢ d ✓

Functions:

Γ ; Δ ; Ε ; Κ ; R ; C ⊢ f : F

A program is type-valid when all required judgments can be established.

A target is feasible only after type validation and subsequent capability/resource analysis.

---

10. Type-System Phases

Type processing MUST conceptually occur in the following order.

source syntax
    ↓
AST construction
    ↓
structural AST validation
    ↓
name resolution
    ↓
type-expression resolution
    ↓
generic binding
    ↓
associated-type resolution
    ↓
constraint collection
    ↓
constraint normalization
    ↓
unification / inference
    ↓
subtyping / compatibility
    ↓
ownership / linearity / affinity
    ↓
effect checking
    ↓
capability checking
    ↓
resource checking
    ↓
contract/policy validation
    ↓
semantic type finalization
    ↓
canonical semantic representation
    ↓
IR lowering

A parser MUST NOT perform semantic type checking.

A type grammar MUST NOT perform resource discovery.

A type checker MUST NOT perform physical placement.

---

11. Canonical Source AST

There MUST be exactly one canonical source-level type-expression representation.

The repository's canonical "TypeExpr" is that representation.

The canonical representation includes source-level constructs corresponding to:

Identifier
Generic
Tuple
Array
Slice
Function
Reference
Pointer
Optional
Result
Never
Unit
SelfType
Infer
GenericParameter
Union
Intersection
Associated
TypeApplication
Quantum
Linear
Affine
Temporal
Pi
Sigma
Identity
Hkt
Extension

The semantic specification MUST describe these constructs rather than introducing another AST hierarchy.

Convenience types such as:

ArrayType
TupleType
FunctionType
ReferenceType
ResultType

MAY exist as safe APIs/facades.

They MUST NOT become competing AST authorities.

---

12. Source Type vs Semantic Type

The following distinction is mandatory.

TypeExpr
=
source-level type structure

while:

SemanticType
=
resolved and validated type meaning

and:

IR type
=
validated type information represented at an IR boundary

Therefore:

TypeExpr
≠
SemanticType
≠
IR Type

A source type may contain unresolved:

- names;
- generic parameters;
- inference variables;
- symbolic dimensions;
- associated projections;
- extensions.

Semantic analysis resolves or validates them.

---

13. Type Identity

Semantic type identity MUST NOT depend on:

- whitespace;
- comments;
- source formatting;
- source file path;
- parser node address;
- process ID;
- thread ID;
- compiler memory address;
- allocation address;
- compilation order;
- hardware ID;
- device ID;
- backend implementation;
- vendor.

Internal identifiers MAY include:

TypeId
TypeVariableId
SymbolId
GenericParameterId
ShapeId
EffectId
CapabilityId
ResourceId
RegionId
LifetimeId

These are implementation identities.

They MUST NOT become source-language identities.

---

14. Type Equality

Zamani distinguishes at least:

syntactic equality
structural equality
definitional equality
semantic equality
compatibility
subtyping
conversion

These MUST NOT be conflated.

14.1 Syntactic equality

Two source type expressions are syntactically equal when their canonical syntax trees are equivalent.

14.2 Structural equality

Two types are structurally equal when their normalized structures are equal.

14.3 Definitional equality

Two types are definitionally equal when normalization and language-defined type computation establish that they denote the same semantic type.

14.4 Semantic equality

Two resolved types are semantically equal when the language's canonical semantic model identifies them as the same type.

14.5 Compatibility

Two types are compatible when a permitted operation allows one to be used where the other is expected.

Compatibility does not imply equality.

---

15. Type Normalization

Before semantic equality is tested, the compiler MAY normalize:

- aliases;
- transparent wrappers;
- equivalent qualified names;
- type applications;
- associated projections;
- normalized unions;
- normalized intersections;
- equivalent refinement expressions.

Normalization MUST be deterministic.

Normalization MUST NOT change observable program meaning.

---

16. Primitive Types

The core primitive semantic types include:

Bool
Char
String
Unit
Never

Numeric families are specified separately.

Primitive types MUST have target-independent semantic meaning.

---

17. Boolean Type

"Bool" has exactly two semantic values:

true
false

Boolean operations MUST have deterministic semantics.

For short-circuiting operations, evaluation order is part of the language semantics.

A backend MUST preserve this behavior.

---

18. Unit Type

"Unit" represents successful completion with no meaningful result value.

It is distinct from:

Never

and from arbitrary tuple types.

A function returning "Unit" completes normally and produces no meaningful result value.

---

19. Never Type

"Never" represents a computation that cannot produce a normal value.

Examples include:

- non-returning termination;
- explicit language-defined trap;
- divergence where the language semantics classify the computation as non-returning.

"Never" MAY participate in control-flow typing.

Compiler resource exhaustion MUST NOT automatically be treated as a "Never" result.

---

20. Option Types

Optional values are represented semantically as:

Option<T>

with:

Some(T)
None

If shorthand syntax is provided, it MUST canonicalize to "Option<T>".

"None" MUST NOT be represented semantically as:

- an invalid pointer;
- uninitialized memory;
- arbitrary sentinel bits.

---

21. Result Types

Fallible computation is represented as:

Result<T, E>

with:

Ok(T)
Err(E)

Ordinary recoverable failure MUST be represented explicitly.

Undefined behavior MUST NOT be used as an ordinary failure mechanism.

---

22. Integer Types

Zamani distinguishes semantic integer types from implementation-sized integers.

Language-defined fixed-width types MAY include:

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

where defined by the language specification.

The semantic meaning of each fixed-width type is independent of host architecture.

The language MAY also provide:

Integer
Natural
SignedInteger<W>
UnsignedInteger<W>

where "W" is a semantic width.

---

23. Integer Overflow

Integer overflow behavior MUST be explicit and deterministic.

Possible semantic operations include:

checked
wrapping
saturating
trapping
widening
arbitrary-precision

The compiler MUST NOT silently change overflow semantics because a different target was selected.

---

24. Arbitrary-Precision Integers

Where the semantic type represents mathematical integers, its meaning MUST NOT be limited to a host primitive width.

The implementation MAY use:

- big integers;
- symbolic expressions;
- segmented integers;
- optimized target representations.

The implementation MUST NOT silently wrap a semantic integer because an internal machine integer overflowed.

---

25. Floating-Point Types

Floating-point types MUST specify:

- precision;
- range;
- rounding;
- NaN semantics;
- infinity semantics;
- comparison;
- conversion;
- reproducibility behavior.

Target-specific native floating-point formats MAY be used only as valid realizations of the requested semantic type.

A target MUST NOT silently change declared precision semantics.

---

26. Exact Numeric Types

Zamani MAY provide exact numerical types such as:

Integer
Natural
Rational
ExactDecimal
Complex<Exact>

Exact semantics MUST remain exact unless an explicit conversion permits approximation.

---

27. Complex Types

Complex values use:

Complex<T>

where "T" is an appropriate scalar numeric type.

Quantum amplitudes MAY use complex semantic values, but quantum semantics MUST NOT depend on a particular host floating-point representation.

---

28. Numeric Conversion

Conversions MUST be classified.

A conversion is:

lossless
potentially lossy
explicitly lossy
representation-changing
semantic-changing

Potentially lossy conversions MUST be explicit unless a language rule establishes that they are safe and lossless.

---

29. Character and String Types

"Char" represents a semantic Unicode scalar value according to the language's lexical/unicode specification.

"String" represents a sequence of characters.

String representation is implementation-defined.

String semantics MUST NOT depend on:

- pointer width;
- operating system;
- allocator;
- machine word size.

---

30. Tuples

A tuple type is:

(T1, T2, ..., Tn)

Tuple arity is semantic.

There is no language-defined maximum tuple arity.

Compiler resource budgets MAY limit an individual compilation.

Such a budget is not a type-system maximum.

---

31. Arrays

An array type is conceptually:

Array<T, N>

where:

T = element type
N = semantic cardinality

"N" MAY be:

- literal;
- constant;
- generic parameter;
- symbolic expression;
- dependent value;
- another statically validated cardinality.

Array semantics MUST distinguish:

logical cardinality
storage representation
physical allocation

Physical allocation is downstream.

---

32. Slices and Views

A slice/view represents a view over another sequence:

Slice<T>

Its semantics MUST NOT imply ownership unless explicitly qualified.

Dynamic collections MAY include:

Sequence<T>
List<T>
Vector<T>

as library or standard semantic types.

No dynamic collection may acquire a hidden universal maximum.

---

33. Maps and Sets

Maps:

Map<K, V>

Sets:

Set<T>

Their semantics depend on:

- key equality;
- value equality where applicable;
- ordering where applicable;
- hashing where applicable.

The type system MUST NOT require a particular data structure implementation.

---

34. Named and Qualified Types

Named types are resolved from qualified paths.

Examples:

Int
std::collections::Map
quantum::State
module::submodule::Type

A name in source syntax is not a resolved type until semantic name resolution succeeds.

Qualified naming MUST remain target-independent.

---

35. Nominal and Structural Typing

Zamani distinguishes nominal identity from structural compatibility.

A nominal type's identity comes from its declaration identity.

A structural type's compatibility comes from its normalized structure.

Two distinct nominal types MUST NOT become interchangeable merely because their fields happen to match.

Structural compatibility MAY be defined for explicitly structural constructs.

The compiler MUST NOT silently switch a type from nominal to structural interpretation.

---

36. Type Aliases

A transparent alias preserves the underlying semantic type.

For example:

type UserId = Integer

if declared as a transparent alias, has the same semantic type as its target.

A nominal declaration MUST instead create a distinct type identity.

The declaration syntax determines which model applies.

---

37. Newtype/Nominal Wrappers

A nominal wrapper creates a distinct type even if its representation is equivalent to another type.

This is required for:

- domain safety;
- units;
- identifiers;
- security labels;
- resource handles;
- hardware intent;
- quantum abstractions.

Representation equivalence does not imply type equality.

---

38. Generic Types

Zamani supports parametric polymorphism.

Examples:

Option<T>
Result<T, E>
Map<K, V>
Vector<N, T>
Matrix<M, N, T>
Tensor<S, T>
Model<I, O>

Generic application syntax is owned by:

grammar/types/generic.g4

Generic semantics are owned by this specification.

---

39. Generic Parameter Kinds

A generic parameter MUST have a semantic category.

Supported categories include:

Type
Value
Shape
Lifetime
Effect
Capability
Resource

A generic parameter MUST NOT silently acquire a physical-machine interpretation.

For example:

<N>

does not inherently mean:

number of physical CPU cores

It means whatever semantic parameter its declaration establishes.

---

40. Generic Declarations vs Applications

Generic declaration and generic application are different constructs.

The declaration side defines parameters:

<T>
<N>
<S>

and their constraints.

The application side supplies arguments:

Vector<N, Float>
Map<Key, Value>

"grammar/types/generic.g4" owns generic application syntax.

Generic declaration syntax remains owned by the appropriate declaration/function generic grammar.

No grammar file may create a second generic system.

---

41. Generic Bounds

Bounds constrain generic parameters.

Examples:

T : Numeric
T : Serializable
T : Ordered
T : QuantumCompatible

A bound is a semantic constraint.

A bound MUST NOT be interpreted as a machine-selection directive.

---

42. Constraint Model

Zamani distinguishes:

type equality
value equality
type compatibility
subtyping
trait/interface satisfaction
shape equality
shape inequality
resource constraints
capability constraints
effect constraints
refinement predicates
policy constraints

The compiler MUST preserve these distinctions.

Constraint solving MUST be deterministic for the same semantic input and compiler profile.

---

43. Constraint Solving

Constraint solving may involve:

- unification;
- normalization;
- substitution;
- trait/interface resolution;
- associated-type resolution;
- arithmetic reasoning;
- shape reasoning;
- refinement checking;
- capability validation;
- resource validation.

The type checker MUST distinguish:

UNSATISFIABLE_CONSTRAINT

from:

SOLVER_RESOURCE_EXHAUSTED

The latter is not automatically a language-level type error.

---

44. Inference Variables

Inference variables represent unresolved semantic quantities.

Examples:

?
T
N
S
E

Inference variables MUST NOT survive into a finalized monomorphic semantic type unless the language explicitly permits unresolved existential/dynamic semantics.

An unresolved required type variable is a diagnostic error.

---

45. Type Inference

Type inference MUST be:

- deterministic;
- scope-aware;
- constraint-driven;
- independent of target hardware;
- independent of hash-map iteration order;
- independent of backend selection.

Inference MUST NOT depend on:

- available CPU count;
- available GPU count;
- available QPU count;
- memory layout;
- compilation machine identity.

---

46. Subtyping

Zamani MAY support subtyping for explicitly defined semantic relations.

Subtyping MUST NOT be inferred merely from representation similarity.

Where subtyping exists, the specification MUST define:

reflexivity
transitivity
variance interaction
function variance
generic variance
ownership interaction
effect interaction
capability interaction

A target backend MUST NOT invent new source-level subtype relations.

---

47. Variance

For a generic constructor "F<T>", variance MUST be explicitly declared or derived according to language rules.

Supported variance classifications MAY include:

covariant
contravariant
invariant
bivariant

Variance MUST be semantic.

It MUST NOT depend on backend representation.

---

48. Function Types

A function type includes all semantically relevant information.

Conceptually:

fn(
    P1,
    P2,
    ...
) -> R

with optional semantic qualifiers for:

generics
effects
capabilities
resources
ownership
constraints
contracts

Function syntax is owned by:

grammar/types/function.g4
grammar/functions/

Function type meaning is owned here.

---

49. Function Variance

For a function:

fn(A) -> B

parameter and return compatibility MUST follow the language-defined variance rules.

A backend MUST NOT alter function type compatibility according to ABI calling conventions.

ABI adaptation is a downstream conversion.

---

50. Closures

Closure types MUST preserve the semantic types of:

- parameters;
- return value;
- captured values;
- capture ownership;
- effects;
- capabilities;
- resource usage.

A closure MUST NOT silently capture a linear resource multiple times.

---

51. References

A reference type may contain:

mutability
lifetime
referenced type

Conceptually:

&T
&mut T
&'a T
&'a mut T

Reference validity is governed by ownership and lifetime analysis.

The source type does not expose physical addresses.

---

52. Raw Pointers

Raw pointer types are distinct from safe references.

Conceptually:

*const T
*mut T

Their legality and operations are constrained by the language's memory-safety rules.

The production Rust compiler implementation MUST remain safe Rust.

Zamani raw-pointer semantics MUST NOT require "unsafe" Rust in the compiler implementation.

---

53. Ownership

Zamani ownership semantics describe responsibility for values and resources.

The type system MUST distinguish at least:

owned
borrowed
shared
mutable borrowed
linear
affine

Ownership is semantic.

It is not an allocation-address model.

---

54. Move Semantics

Moving an owned value transfers ownership.

After a move, the original binding MUST NOT be used in a way prohibited by the ownership rules.

The compiler MUST detect invalid use after move.

---

55. Borrowing

A borrowed reference does not transfer ownership.

The borrow checker MUST establish that:

- the referent remains valid;
- mutable and immutable access rules are respected;
- lifetimes are compatible;
- linear/affine resource rules remain satisfied.

---

56. Lifetimes

Lifetimes express validity relationships between references.

A lifetime is a semantic region relationship.

It MUST NOT be interpreted as a wall-clock duration.

The type system MUST NOT require lifetime names to correspond to runtime timestamps.

---

57. Linear Types

A linear type represents a value that must be consumed exactly according to the language's linearity rules.

A linear value MUST NOT be silently duplicated.

Examples include:

- certain quantum resources;
- exclusive resource handles;
- ownership tokens;
- unique capabilities.

The linearity checker is semantic.

---

58. Affine Types

An affine type represents a value that may be consumed at most once.

Unlike a linear value, an affine value MAY be dropped if the semantic type permits dropping.

The distinction between:

linear

and:

affine

MUST remain explicit.

---

59. Quantum Ownership

Quantum resources MUST be subject to explicit ownership semantics.

A logical qubit value MUST NOT be duplicated as an ordinary copyable value merely because the host machine can copy a data structure representing it.

The type system must distinguish:

logical quantum resource

from:

classical description of a quantum resource

The latter may be copyable where its type permits it.

---

60. Quantum Type Boundary

Quantum-specific source types are owned syntactically by:

grammar/quantum/
grammar/types/quantum.g4
grammar/quantum/types.g4

according to the repository's single-owner grammar contract.

This specification owns their semantics.

The semantic pipeline is:

quantum source type
        ↓
TypeExpr
        ↓
resolved quantum semantic type
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
resilience / QEC
        ↓
ZQN
        ↓
HAL

The type system MUST NOT:

- select physical qubits;
- select a QPU;
- choose a coupling map;
- choose calibration;
- choose a pulse implementation;
- choose a QEC code;
- assign device IDs.

---

61. Quantum Type Examples

Portable semantic types MAY include:

Qubit
LogicalQubit
QRegister<N>
QuantumState<S>
Observable<O>
QuantumChannel<I, O>

These names are semantic constructors, not physical inventories.

For example:

QRegister<N>

means a register whose semantic cardinality is "N".

It does not mean a particular number of physical qubits.

---

62. Quantum Type Compatibility

Quantum types MUST preserve distinctions between:

classical bit
quantum bit
logical qubit
physical qubit
quantum state
measurement result
observable
quantum operation
quantum channel

A classical "Bool" MUST NOT silently become a "Qubit".

A "Qubit" MUST NOT silently become a classical Boolean merely because measurement exists.

Measurement is a semantic operation with its own result and effect rules.

---

63. Quantum Resource Constraints

Quantum type validity and quantum resource feasibility are distinct.

This:

QRegister<N>

is a type-level semantic construct.

This:

requires qubits >= N

is a resource requirement.

This:

capability("quantum.measurement")

is a capability requirement.

This:

physical_qubit = 17

is downstream realization.

These MUST NOT be conflated.

---

64. Shape Types

Scientific, numerical, tensor, and hardware data types MAY use semantic shape parameters.

Examples:

Vector<N, T>
Matrix<M, N, T>
Tensor<S, T>

Shape expressions are semantic values.

They are not necessarily host integers.

---

65. Shape Equality

Two dimensions are equal when the semantic constraint system establishes:

A = B

A literal equality is only one way to establish this.

Equality may also arise from:

- generic parameters;
- aliases;
- normalized arithmetic expressions;
- dependent parameters;
- compile-time definitions.

---

66. Shape Constraints

A matrix multiplication constraint is:

Matrix<M, N, A>
×
Matrix<N, K, B>

The result has shape:

Matrix<M, K, Result>

A multiplication:

Matrix<M, N, A>
×
Matrix<K, P, B>

requires a proof/constraint:

N = K

If that constraint cannot be established, the operation is rejected.

The parser does not perform this reasoning.

---

67. Tensor Types

Tensor semantics support:

- scalar tensors;
- vectors;
- matrices;
- arbitrary-rank tensors;
- symbolic dimensions;
- dynamic dimensions;
- symbolic shapes.

There is no universal maximum tensor rank.

Compiler resource budgets remain implementation policies.

---

68. Dependent Values

A dependent type may contain a type-level value.

Examples:

Array<T, N>
Vector<N, T>

where "N" is a validated semantic value.

A dependent value MUST be distinguishable from an ordinary runtime value when compile-time proof is required.

---

69. Type-Level Values

The source AST already supports a type-level value representation.

A type-level value MAY represent:

- integer literal;
- symbolic identifier;
- generic value parameter;
- registered extension value.

The type-level value system MUST remain separate from ordinary runtime expression evaluation unless an explicit compile-time evaluation bridge is defined.

---

70. Compile-Time Evaluation

Type-level evaluation MUST be:

- deterministic;
- side-effect controlled;
- resource bounded by compiler policy;
- independent of target hardware;
- reproducible under the same semantic profile.

Compile-time evaluation MUST NOT perform unrestricted:

- network access;
- filesystem access;
- device discovery;
- hardware mutation;
- secret access;
- uncontrolled code generation.

Any permitted external capability requires an explicit metaprogramming/effect contract.

---

71. Dependent Pi Types

A dependent function type is conceptually:

Π(x : A). B(x)

It represents a function whose result type depends on a parameter value.

The canonical AST representation is:

TypeExpr::Pi

The type checker MUST validate:

1. the parameter name;
2. the parameter type;
3. the scope of the parameter;
4. the body type;
5. substitution;
6. dependency validity.

---

72. Dependent Sigma Types

A dependent pair is conceptually:

Σ(x : A). B(x)

The canonical AST representation is:

TypeExpr::Sigma

The semantic type represents a value containing:

x : A

and a second component whose type depends on "x".

---

73. Type Identity Propositions

The canonical source representation supports identity propositions:

Identity<A, B>

Identity propositions MUST be interpreted as semantic equality propositions.

They MUST NOT be treated as arbitrary runtime Boolean values unless an explicit proposition-to-value conversion is defined.

---

74. Refinement Types

A refinement type represents:

T where P

where "P" is a predicate over values of "T".

The predicate MUST be semantically well-formed.

A refinement MAY be discharged through:

- compile-time proof;
- constraint solving;
- trusted verifier;
- explicit runtime validation.

If the compiler cannot establish the refinement statically and runtime checking is permitted, the runtime check MUST remain explicit in the semantic model.

---

75. Proof Obligations

Type checking may generate proof obligations.

Examples:

N = M
N > 0
index < length
capability satisfies requirement
resource budget satisfies requirement

A proof obligation is not automatically true merely because it is syntactically present.

Unproven required obligations MUST cause a diagnostic unless the language explicitly permits runtime validation or another declared proof mode.

---

76. Type Classes / Interfaces

Zamani MAY express semantic capabilities of types through type classes, interfaces, traits, or equivalent constraint constructs.

A constraint such as:

T : Numeric

means that "T" satisfies the semantic contract named "Numeric".

It does not imply a particular machine implementation.

---

77. Type-Class Resolution

Resolution MUST be:

- deterministic;
- scope-aware;
- version-aware;
- ambiguity-detecting;
- independent of hardware;
- independent of hash-map iteration.

If multiple unrelated implementations satisfy the same required constraint and no resolution rule selects one, compilation MUST report an ambiguity.

---

78. Associated Types

Associated types represent type members attached to a type-level abstraction.

Conceptually:

Iterator::Item

The canonical source representation is:

TypeExpr::Associated

Associated-type resolution MUST occur during semantic analysis.

An unresolved required associated type is a type error.

---

79. Associated-Type Equality

Constraints may establish:

Iterator::Item = T

Such equality is a semantic constraint.

The compiler MUST NOT compare only textual names.

---

80. Higher-Kinded Types

The type system MAY support type constructors as values at the type level.

A constructor may have a kind conceptually analogous to:

Type -> Type
Type -> Type -> Type

Kinds MUST be checked before type application.

A type constructor and a fully applied type are distinct semantic entities.

---

81. Type Application

The canonical AST supports:

TypeApplication

This exists to represent type-level constructor application beyond ordinary nominal generic syntax.

The semantic checker MUST distinguish:

generic type application

from:

type-level function application

where their semantics differ.

---

82. Type Extensions

The canonical AST supports an extensible type form.

An extension type MUST identify:

namespace
name
arguments
attributes
version/registration identity where required

An extension MUST declare:

- semantic meaning;
- compatibility;
- type parameters;
- constraints;
- effects if relevant;
- capabilities if relevant;
- resource requirements if relevant;
- IR mapping.

An extension MUST NOT silently redefine an existing stable core type.

---

83. Open-World Type Extension

The core type system MUST be open to future computational domains.

A new domain SHOULD be expressible through:

- named types;
- qualified types;
- generic applications;
- associated types;
- type classes;
- extensions;
- dialect registration.

The addition of a new domain MUST NOT require adding a new universal enum branch merely because the domain introduces a new semantic type family, unless the type genuinely requires core language semantics unavailable through existing mechanisms.

---

84. Resource-Aware Types

A type MAY carry semantic resource information.

Examples include:

Resource<T>
Linear<T>
Affine<T>
Capability<T>

The type system verifies the structural resource relationship.

The resource subsystem determines actual availability.

Therefore:

type validity

and:

resource availability

are distinct checks.

---

85. Capability-Aware Types

Capabilities represent permission or ability to perform a class of computation.

Examples:

Capability<"quantum.measurement">
Capability<"gpu.compute">
Capability<"tensor.compute">
Capability<"network">

Capability identifiers are semantic identifiers.

They do not identify physical devices.

Actual capability satisfaction belongs to capability analysis and target realization.

---

86. Effects and Types

Effects are not automatically types.

The effect subsystem owns effect vocabulary and effect semantics.

A function type MAY carry an effect set or effect row when the language's function-type model supports it.

Conceptually:

fn(A) -> B ! {io, network}

The exact source spelling is owned by the effect grammar.

This document specifies only that:

- effectful function compatibility must account for effects;
- effect information must not be silently discarded;
- effect polymorphism must be represented where required;
- target selection must not alter effect meaning.

---

87. Effect Polymorphism

A generic computation MAY quantify over effects.

Conceptually:

F<E>

where "E" is an effect parameter.

Effect substitution MUST preserve the semantic meaning of the function.

An implementation MUST NOT erase an effect that is required for safety, security, determinism, or policy enforcement.

---

88. Capability Constraints

A type-dependent computation may require a capability.

For example:

requires capability("quantum.measurement")

The type system records the relationship where the capability is part of the semantic contract.

Capability discovery remains outside the type checker.

---

89. Resource Constraints

A type may introduce a resource requirement.

Examples:

QRegister<N>

may induce:

requires qubits >= N

and:

Tensor<Shape, T>

may induce resource requirements based on the selected realization.

The type system MAY generate resource obligations.

It MUST NOT decide physical allocation.

---

90. Contracts

Type semantics integrate with:

requires
ensures
invariant
assume
guarantee
property
assert

The validation subsystem owns contract syntax and verification semantics.

The type system consumes contracts when they constrain type validity.

For example:

Array<T, N>

combined with:

requires N > 0

may establish a refinement needed for a later operation.

---

91. Policies

Policies may constrain type-dependent operations.

Examples include:

- permitted conversions;
- permitted resource use;
- permitted capabilities;
- allowed effects;
- adaptation permissions;
- foreign calls;
- reflection.

A policy MUST NOT silently change type equality.

It may instead make an otherwise type-valid operation disallowed in a given execution context.

---

92. Provenance

Type resolution MAY produce provenance describing:

source type
resolved declaration
generic substitution
constraint evidence
associated-type resolution
refinement proof
conversion

Provenance MUST NOT alter type identity.

Provenance belongs to the semantic/provenance subsystem.

---

93. Determinism

For identical:

source
language version
dialect set
semantic configuration
type environment

type checking MUST produce deterministic:

- type resolution;
- constraint results;
- type equality results;
- generic substitutions;
- diagnostics ordering;
- semantic type identity.

Hash-map iteration order MUST NOT influence semantic results.

---

94. Error Classification

The compiler MUST distinguish at least:

TYPE_INVALID
TYPE_NAME_UNRESOLVED
TYPE_ARITY_MISMATCH
TYPE_KIND_MISMATCH
TYPE_INFERENCE_FAILED
TYPE_CONSTRAINT_UNSATISFIED
TYPE_SUBTYPE_MISMATCH
TYPE_CONVERSION_INVALID
TYPE_COERCION_INVALID
TYPE_ASSOCIATED_UNRESOLVED
TYPE_AMBIGUOUS
TYPE_REFINEMENT_UNPROVEN
TYPE_OWNERSHIP_VIOLATION
TYPE_LINEARITY_VIOLATION
TYPE_AFFINITY_VIOLATION
TYPE_LIFETIME_VIOLATION
TYPE_EFFECT_MISMATCH
CAPABILITY_UNAVAILABLE
RESOURCE_UNAVAILABLE
POLICY_VIOLATION
TARGET_UNSUPPORTED
COMPILER_RESOURCE_EXHAUSTED
IR_INVALID

These MUST NOT be collapsed into one generic type error.

---

95. Resource Exhaustion During Type Checking

If the compiler runs out of configured resources while solving a valid or potentially valid type problem, the compiler MUST distinguish:

solver did not finish

from:

constraint proven false

For example:

TYPE_CONSTRAINT_UNSATISFIED

means the constraint is known to be false.

COMPILER_RESOURCE_EXHAUSTED

means the implementation could not complete the requested analysis under its configured budget.

This distinction is essential for scalable semantics.

---

96. Type-Level Evaluation Limits

Type-level computation MUST be resource-bounded operationally.

A compiler MAY configure:

type evaluation budget
constraint solving budget
memory budget
diagnostic budget
recursion budget

Such budgets are invocation policies.

They MUST NOT become language-level semantic maxima.

---

97. Recursive Types

Recursive types are valid where the language's declaration rules permit them.

Examples include:

List<T>
Tree<T>
Graph<T>

Recursive semantic definitions MUST be represented without imposing a language-defined maximum recursion depth.

The implementation MAY use iterative algorithms, memoization, graph interning, or explicit worklists to avoid unnecessary host-stack limitations.

---

98. Type-Graph Scalability

The semantic type model MUST conceptually support arbitrary finite type graphs.

Implementations SHOULD avoid algorithms whose correctness depends on:

- fixed recursion depth;
- fixed collection capacity;
- fixed generic arity;
- fixed type nesting depth.

Where a compiler safety policy imposes a budget, the failure MUST identify the budget as an implementation limit.

---

99. No Silent Host Integer Narrowing

A semantic quantity MUST NOT be silently converted:

BigInt → u64
BigInt → u32
BigInt → usize

unless the conversion is explicitly checked and proven valid.

This applies to:

- dimensions;
- cardinalities;
- resource quantities;
- generic values;
- array lengths;
- quantum counts;
- topology sizes.

---

100. No Hidden Target Specialization

The type checker MUST NOT change:

T

into a target-specific type merely because a target happens to be available.

Specialization occurs downstream.

For example:

Tensor<S, Float>

may later lower to:

CPU implementation
GPU implementation
FPGA implementation
accelerator implementation

without changing the source-level type.

---

101. Classical Domain Integration

Classical computation consumes the universal type system.

Classical types MUST participate in:

- generics;
- ownership;
- effects;
- capabilities;
- resources;
- contracts;
- policies;
- provenance.

Classical numeric and data types MUST NOT create a separate incompatible type system.

---

102. Quantum Domain Integration

Quantum types use the universal type foundation.

Quantum semantics add:

- quantum resource identity;
- measurement semantics;
- quantum state semantics;
- quantum operation compatibility;
- quantum ownership;
- quantum effects;
- quantum resource requirements.

Quantum types MUST converge through:

quantum::ir

after semantic validation.

---

103. Hybrid Classical/Quantum Integration

Hybrid computation uses one type system.

Examples include:

classical value
    ↓
quantum parameter

quantum measurement
    ↓
classical value

classical control
    ↓
quantum operation

quantum result
    ↓
classical decision

The type checker MUST validate the boundary conversions explicitly.

---

104. HDL Integration

HDL types remain semantically distinct from ordinary software values where their meanings differ.

Examples include:

Signal<T>
Net<T>
Register<T>
Port<T>
Clock
Reset
Interface<T>

An HDL signal containing bits is not automatically equivalent to an ordinary integer merely because both have a bit representation.

Timing and hardware realization remain downstream.

---

105. Hardware Intent Integration

Hardware intent types describe:

- computation capability;
- storage;
- interconnect;
- signal structure;
- accelerator intent;
- timing requirements;
- resource classes.

They MUST NOT encode vendor-specific devices as universal core types.

Hardware-specific properties belong to:

capabilities
resources
dialects
target descriptions
interoperability
backend contracts

---

106. Distributed Type Integration

Distributed semantic types MAY include:

Node<T>
Process<T>
Actor<T>
Channel<T>
Replica<T>
Distributed<T>

These types describe distributed semantics.

They do not select physical nodes.

Topology realization is downstream.

---

107. Networking Type Integration

Networking types MAY include:

Endpoint<T>
Channel<T>
Stream<T>
Message<T>
Protocol<P>
Service<I, O>

The type system validates semantic compatibility.

The networking subsystem determines the transport realization.

---

108. Data and AI Integration

The universal type system supports:

Tensor<S, T>
Dataset<T>
Model<I, O>
Distribution<T>
Agent<I, O>
Graph<N, E>
Schema<S>

These are semantic abstractions.

Application-specific algorithms remain libraries or registered semantic extensions.

---

109. Probabilistic and Uncertain Types

The type system MAY support semantic constructs such as:

Probability
Distribution<T>
Uncertain<T>
Confidence<T>

The type system MUST distinguish:

value
probability
distribution
confidence
uncertainty metadata

A confidence value MUST NOT automatically imply truth.

---

110. Security-Sensitive Types

Security-sensitive values MAY be represented by nominal types or capability-qualified types.

Examples include:

Secret<T>
Credential<T>
Key<T>
Token<T>

Security policies MUST NOT be bypassed by type conversion.

A backend MUST NOT silently weaken a security type.

---

111. Foreign and ABI Types

Foreign types are explicitly marked as interoperability types.

The type system MUST preserve the distinction between:

portable Zamani type

and:

foreign representation

FFI/ABI compatibility belongs to:

grammar/interoperability/

and downstream implementation contracts.

The type system validates declarations but does not choose a vendor ABI.

---

112. Type Conversion

A conversion is permitted only when a language-defined rule exists.

Conversions are categorized as:

identity
widening
narrowing
representation
nominal
structural
ownership
effect
capability
resource
foreign

Every non-identity conversion MUST have a defined semantic meaning.

---

113. Coercions

A coercion is an implicit conversion permitted by the language.

Coercions MUST be:

- deterministic;
- limited to explicitly defined relations;
- non-ambiguous;
- target-independent.

A compiler MUST NOT introduce a target-specific coercion merely because a backend can implement it.

---

114. Explicit Conversion

Potentially lossy or semantically significant conversions SHOULD require explicit source syntax.

Examples:

Float → Integer
Approximate → Exact
Classical → Quantum
Quantum → Classical
Nominal A → Nominal B
Portable → Foreign ABI

where applicable.

---

115. Quantum Measurement Conversion

Quantum measurement is not an ordinary type coercion.

Conceptually:

Qubit
   ↓
measurement operation
   ↓
classical result

The conversion is an operation with defined quantum effects.

The type checker MUST NOT model measurement as an implicit cast.

---

116. Type Compatibility and Resources

A type may be semantically valid while a target lacks the resources necessary to realize it.

For example:

QRegister<N>

may be type-valid.

A target may still fail:

RESOURCE_UNAVAILABLE

because it cannot satisfy:

qubits >= N

This MUST NOT be reported as:

TYPE_INVALID

unless the type itself violates a semantic rule.

---

117. Capability Negotiation

Capability negotiation occurs after type semantics are established.

The conceptual flow is:

typed program
    ↓
required capabilities
    ↓
capability environment
    ↓
target negotiation
    ↓
realization

Type checking does not perform device discovery.

---

118. Resource Negotiation

Resource negotiation follows the same separation:

type/resource intent
    ↓
resource requirements
    ↓
available resources
    ↓
feasibility
    ↓
specialization
    ↓
realization

The source type remains unchanged.

---

119. Target Independence

The type system MUST NOT depend on:

CPU ISA
GPU architecture
FPGA vendor
ASIC vendor
QPU vendor
specific physical qubit
specific memory address
specific node
specific network topology
specific operating system
specific ABI

Target-specific realization MUST be represented downstream.

---

120. Dialect Types

A dialect may define additional types.

Every registered dialect type MUST specify:

identifier
version
namespace
parameters
constraints
semantic meaning
compatibility
AST mapping
semantic resolution
IR mapping
capabilities
effects
resources
provenance

A dialect MUST NOT silently redefine stable core type semantics.

---

121. Type Versioning

Changes to type semantics are language compatibility events.

Potentially breaking changes include:

- changing equality;
- changing subtyping;
- changing ownership;
- changing generic compatibility;
- changing numeric overflow semantics;
- changing quantum resource semantics;
- changing effect compatibility;
- changing associated-type resolution;
- changing dependent-type normalization.

Such changes MUST be versioned according to the repository compatibility specification.

---

122. Serialization and Type Identity

Serialized semantic types MUST carry enough schema/version information to prevent accidental interpretation under an incompatible schema.

Internal IDs MUST NOT be interpreted as portable semantic identities.

A serialized type MUST remain meaningful only under a compatible language/type schema.

---

123. AST Schema Compatibility

The canonical source AST schema and the language semantic version are distinct.

An AST schema change does not automatically mean a language semantic change.

Conversely, a semantic change MUST NOT be hidden merely by preserving the same AST structure.

---

124. Grammar Integration

The type grammar hierarchy is:

grammar/types/types.g4
        │
        ├── primitive
        ├── named
        ├── generic
        ├── function
        ├── tuple
        ├── array
        ├── slice
        ├── reference
        ├── pointer
        ├── option/result
        ├── bounds/constraints
        ├── associated types
        ├── linear/affine
        ├── dependent
        ├── classical
        ├── quantum
        ├── hardware
        ├── resource/capability
        └── extensions

Only "typeExpression" is the universal public type entry point.

No specialized grammar may define another universal type-expression rule.

---

125. Generic Grammar Integration

"grammar/types/generic.g4" owns generic application syntax.

It MUST integrate with:

typeExpression

without creating:

Types → Generic → Types

as a circular grammar-import architecture.

Generic declaration syntax belongs to the declaration/function generic subsystem.

---

126. Bounds Grammar Integration

"grammar/types/bounds.g4" owns source syntax for bounds.

The type system interprets the resulting constraints.

A bound MUST resolve to an existing semantic constraint mechanism.

It MUST NOT introduce a parallel type-class system.

---

127. Associated-Type Grammar Integration

Associated type syntax belongs to its designated grammar.

Its AST representation is:

TypeExpr::Associated

Semantic resolution belongs to type analysis.

No duplicate associated-type AST is permitted.

---

128. Linear and Affine Grammar Integration

"grammar/types/linear.g4" and "grammar/types/affine.g4" own source syntax.

Their semantics are:

Linear<T>
Affine<T>

or the repository's canonical equivalent.

The ownership checker consumes the resolved semantic qualifiers.

These grammars MUST NOT allocate resources.

---

129. Dependent-Type Grammar Integration

Dependent syntax is owned by:

grammar/types/dependent.g4

The canonical AST representation uses:

Pi
Sigma
Identity
Type-level values

The type checker owns:

- scope;
- substitution;
- normalization;
- equality;
- proof obligations.

Dependent types MUST NOT be confused with ordinary runtime generics.

---

130. Effects Integration

Effects are defined by:

grammar/effects/
grammar/spec/effects.md

The type system consumes effect information but does not define the effect vocabulary.

Effect checking MUST occur before final semantic type commitment for computations whose validity depends on effects.

---

131. Resources Integration

Resources are defined by:

grammar/resources/
grammar/spec/resources.md

The type system can emit resource obligations.

Resource availability is resolved downstream.

A resource shortage MUST NOT invalidate a source type.

---

132. Capabilities Integration

Capabilities are defined by:

grammar/resources/
grammar/security/
grammar/spec/resources.md

and related contracts.

Type semantics MAY require capabilities.

Capability availability is not type identity.

---

133. Contract Integration

Contracts are owned by:

grammar/validation/
grammar/specification/

The type system consumes type-relevant contract facts.

Contract syntax MUST NOT be duplicated inside type grammars.

---

134. Policy Integration

Policies are owned by:

grammar/policies/
grammar/security/
grammar/execution/

A policy can prohibit an operation without changing the type of the values involved.

---

135. Provenance Integration

Provenance is owned by the provenance subsystem.

Type analysis MAY emit provenance events for:

resolution
inference
substitution
conversion
constraint solving
proof discharge
extension resolution

The provenance mechanism MUST NOT change semantic type identity.

---

136. Macro Integration

Macros MAY generate type syntax.

Macro expansion MUST complete before final semantic type checking.

Generated type syntax is subject to exactly the same type rules as handwritten syntax.

Macros MUST NOT bypass:

- type checking;
- ownership;
- effects;
- capabilities;
- resources;
- contracts;
- policies.

---

137. Reflection and Metaprogramming

Reflection MAY inspect type information.

Metaprogramming MAY construct type expressions.

Neither may bypass semantic validation.

Reflective type identity MUST remain canonical and independent of:

- compiler memory addresses;
- backend IDs;
- physical device IDs.

---

138. Canonical Semantic Model

After type checking, the compiler produces a resolved semantic type model.

The semantic model MUST contain enough information for downstream consumers to determine:

- identity;
- parameters;
- constraints;
- ownership;
- effects;
- capabilities;
- resource obligations;
- domain semantics;
- provenance where required.

It MUST NOT contain accidental backend decisions.

---

139. Canonical IR Integration

The type system feeds canonical IR only after semantic validation.

The relationship is:

TypeExpr
    ↓
SemanticType
    ↓
validated semantic operation/value
    ↓
canonical IR

The type system MUST NOT directly construct backend-specific machine instructions.

---

140. Classical IR Integration

Classical semantic values and computations MUST lower to the repository's canonical Classical IR boundary.

Type information retained in the IR MUST preserve semantic distinctions necessary for:

- correctness;
- optimization;
- ownership;
- effects;
- resources;
- contracts;
- interoperability.

---

141. Quantum IR Integration

Quantum types MUST lower through:

quantum::ir

There MUST NOT be a second competing quantum IR introduced by the type system.

The type system MUST NOT depend on:

OpenQASM
QIR
CUDA
LLVM
MLIR
vendor QPU APIs

as canonical type semantics.

Those are interoperability or lowering technologies.

---

142. HDL and Hardware IR Integration

HDL/hardware semantic types may lower to domain-specific IR where necessary.

Such IRs are downstream representations.

They MUST preserve the source-level semantic type contract.

---

143. IR Type Preservation

Lowering MUST preserve all type properties that remain semantically observable.

This includes, where applicable:

- signedness;
- precision;
- shape;
- ownership;
- linearity;
- affinity;
- quantum identity;
- effect obligations;
- capability requirements;
- resource obligations;
- refinement guarantees.

A lowering pass MUST NOT discard a property merely because its selected target does not directly represent it.

The pass must either:

1. preserve it;
2. discharge it with valid evidence;
3. lower it into an equivalent mechanism;
4. reject the lowering.

---

144. Optimization and Types

Optimization MUST preserve type semantics.

An optimization MAY change representation.

It MUST NOT change:

- type identity;
- ownership guarantees;
- effect semantics;
- quantum legality;
- refinement guarantees;
- contract meaning.

An optimization may remove type information only when that information has been proven unnecessary for all remaining semantic obligations.

---

145. Specialization

Specialization may instantiate generic types using known parameters.

Specialization MUST preserve generic semantics.

Specialization is a compiler operation.

It does not redefine the source generic type.

---

146. Monomorphization

An implementation MAY use monomorphization.

It MUST NOT make monomorphization part of source-language semantics.

A compiler MAY instead use:

- dictionary passing;
- type erasure;
- interpretation;
- JIT;
- AOT;
- specialization;
- hybrid strategies.

The observable semantics MUST remain equivalent.

---

147. Representation Independence

Two implementations may represent the same semantic type differently.

For example:

Vector<N, Float>

may become:

CPU memory
GPU buffer
FPGA stream
accelerator tile
distributed partition

The representation does not define type identity.

---

148. Memory and Type Semantics

The type system describes logical values.

Physical memory layout is downstream.

A type MAY specify alignment, layout, or representation only when those properties are explicitly part of the type's semantics.

Otherwise:

type

MUST NOT imply:

physical address

---

149. ABI and Type Semantics

ABI compatibility is a separate concern.

A source-level type may have multiple valid ABI representations.

An explicit foreign/ABI type may constrain representation.

The compiler MUST NOT make portable source types ABI-specific merely because one backend uses a particular calling convention.

---

150. Interoperability

External representations such as:

- JSON;
- XML;
- SQL;
- OpenQASM;
- other data formats;
- foreign APIs;

must map into the canonical Zamani type system through explicit interoperability contracts.

External syntax MUST NOT become the universal type authority.

---

151. Type Safety and Safe Rust

The reference compiler implementation MUST use safe Rust.

The relevant Rust modules MUST include:

#![forbid(unsafe_code)]

where appropriate at crate/module boundaries.

Production code MUST NOT use:

unsafe
unsafe {}
unsafe fn
unsafe trait
unsafe impl

as part of the compiler implementation.

The type-system implementation SHOULD prefer:

- ownership;
- "Box";
- "Arc";
- "Rc" where appropriate;
- "Vec";
- slices;
- enums;
- pattern matching;
- explicit worklists;
- checked arithmetic;
- fallible APIs;
- "Result";
- deterministic maps/sets where semantic ordering matters.

---

152. Safe FFI Boundary

If a foreign subsystem requires unsafe operations internally, the Zamani type-system implementation MUST NOT expose that unsafety into the semantic type checker.

The safe compiler layer MUST consume a validated safe abstraction.

Unsafe implementation details are outside this type-system specification.

---

153. Panic Policy

Normal invalid source programs MUST produce diagnostics rather than compiler panics.

Internal invariants may use assertions during development where appropriate, but production compiler paths SHOULD return structured errors for recoverable invalid input.

Untrusted source must not cause undefined behavior.

---

154. Arithmetic Safety

Semantic quantities MUST use checked operations.

A compiler implementation MUST NOT silently wrap:

- dimensions;
- cardinalities;
- resource counts;
- type-level integers;
- diagnostic counters;
- constraint solver quantities.

Where an implementation index uses "usize", conversions MUST be checked.

---

155. Memory Safety

Recursive semantic structures MUST use safe ownership.

The implementation MAY use:

Box<T>
Arc<T>
Vec<T>

or equivalent safe structures.

No semantic rule may depend on raw pointer identity.

---

156. Deterministic Collections

Where collection iteration affects:

- diagnostics;
- canonical serialization;
- type identity;
- constraint ordering;
- semantic hashes;

the implementation MUST use a deterministic ordering strategy.

Unordered map iteration MUST NOT determine semantic output.

---

157. Type Hashing

If semantic type hashes are used, the hash input MUST be canonical.

The hash MUST NOT depend on:

- memory address;
- process ID;
- random hash seed;
- allocation order;
- compilation order.

If cryptographic stability is required, the relevant hash algorithm and encoding MUST be specified by the repository compatibility/provenance contract.

---

158. Source Locations

Source locations are metadata.

They MUST NOT affect semantic type identity.

They are retained for diagnostics and provenance.

Two identical semantic types from different source files remain semantically identical unless their declarations create distinct nominal identities.

---

159. Diagnostics

Every type diagnostic SHOULD identify:

error category
primary source span
expected type
actual type
relevant constraint
resolution context
candidate alternatives where applicable
actionable explanation

Diagnostics MUST distinguish type failure from resource or target failure.

---

160. Generic Diagnostic Example

A diagnostic should conceptually distinguish:

expected: Matrix<M, N, T>
found:    Matrix<M, K, T>
required constraint: N = K
status: TYPE_CONSTRAINT_UNSATISFIED

from:

type: QRegister<N>
status: RESOURCE_UNAVAILABLE
requirement: qubits >= N

These are different failures.

---

161. Type-System Tests

Every type feature MUST have:

positive tests
negative tests
boundary tests
scalability tests
determinism tests
compatibility tests
cross-domain tests
IR integration tests

where applicable.

---

162. Primitive Tests

Tests MUST cover:

- Bool;
- Char;
- String;
- Unit;
- Never;
- integer families;
- floating-point families;
- exact numeric types where implemented;
- complex types.

---

163. Generic Tests

Tests MUST cover:

- generic declaration;
- generic application;
- generic inference;
- bounds;
- multiple parameters;
- nested generic types;
- symbolic value parameters;
- associated types;
- variance;
- ambiguous constraints;
- unsatisfied constraints.

---

164. Ownership Tests

Tests MUST cover:

- move;
- borrow;
- mutable borrow;
- lifetime compatibility;
- linear consumption;
- affine consumption;
- invalid duplication;
- invalid use after move;
- invalid lifetime escape.

---

165. Dependent-Type Tests

Tests MUST cover:

- "Pi";
- "Sigma";
- identity;
- symbolic dimensions;
- substitution;
- normalization;
- equality;
- unsatisfied proof obligations;
- valid dependent specialization.

---

166. Shape Tests

Tests MUST cover:

Vector<N, T>
Matrix<M, N, T>
Tensor<S, T>

including:

- equal dimensions;
- symbolic dimensions;
- dependent dimensions;
- compatible dimensions;
- incompatible dimensions;
- dynamic dimensions where supported.

---

167. Quantum Type Tests

Tests MUST cover:

- logical qubit;
- quantum register;
- symbolic quantum cardinality;
- linear quantum resources;
- measurement result typing;
- quantum/classical boundaries;
- hybrid computations;
- resource requirements;
- unavailable quantum capabilities.

---

168. HDL Tests

Tests MUST cover:

- signals;
- ports;
- registers;
- interfaces;
- timing-related type constraints;
- hardware intent;
- type distinction between HDL and ordinary software values.

---

169. Distributed Tests

Tests MUST cover:

- actor types;
- channels;
- distributed values;
- replicas;
- symbolic topology requirements;
- resource-dependent placement.

---

170. Interoperability Tests

Tests MUST cover:

- foreign types;
- ABI declarations;
- explicit conversions;
- serialization types;
- data-format mappings;
- incompatible foreign representations.

---

171. Negative Tests

The following MUST fail where semantically invalid:

incompatible integer assignment
invalid generic arity
unsatisfied generic bound
ambiguous type inference
invalid associated type
invalid type-level application
invalid shape multiplication
linear value duplicated
affine value consumed twice
invalid lifetime
invalid quantum/classical conversion
missing required capability
invalid effect requirement
invalid refinement
invalid dependent equality
invalid nominal conversion
invalid foreign conversion

---

172. Resource Failure Tests

The compiler MUST distinguish:

type invalid

from:

resource unavailable

Tests MUST demonstrate that a valid type can remain valid even when a selected target cannot realize it.

---

173. Scalability Tests

Tests MUST include progressively larger semantic structures without encoding a universal maximum.

Examples include:

large generic applications
large tuples
deep type graphs
large symbolic dimensions
large tensor shapes
large quantum cardinalities
large distributed descriptions
large associated-type graphs
large constraint sets

The test harness MAY choose concrete sizes.

Those sizes MUST NOT become language limits.

---

174. Compiler-Budget Tests

Tests MUST verify that configured compiler budgets produce:

COMPILER_RESOURCE_EXHAUSTED

rather than false semantic errors.

Examples include:

- type solver work budget;
- type-level evaluation budget;
- memory budget;
- diagnostic budget.

---

175. Determinism Tests

Given identical:

source
language version
dialects
type environment
semantic configuration

the implementation MUST produce deterministic:

- resolved types;
- constraint results;
- diagnostics;
- canonical semantic serialization;
- semantic type identity.

---

176. Cross-Target Tests

The same source type program SHOULD be analyzed against:

CPU
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC
distributed target
future/opaque target

where those target profiles exist.

Type meaning MUST remain unchanged.

Only feasibility and realization MAY differ.

---

177. Compatibility Tests

Compatibility tests MUST verify that:

- existing valid programs remain valid;
- deliberate breaking changes are versioned;
- type equality remains stable;
- generic compatibility remains stable;
- ownership remains stable;
- quantum type semantics remain stable;
- effect compatibility remains stable.

---

178. Hard-Coding Audit

The type-system specification and implementation MUST be audited for accidental machine ceilings.

Suspicious constructs include:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

A fixed value is permitted only when it is explicitly:

- semantic by language definition;
- a program value;
- a test fixture;
- a compiler resource budget;
- a diagnostic budget;
- a security policy;
- a target-specific resource property.

It MUST NOT masquerade as a universal type-system limit.

---

179. Important Distinction: Fixed Semantic Widths

A fixed-width type such as:

u32

is not an artificial hardware limit.

The width is the semantic definition of that type.

This is valid:

u32

This is not a universal type rule:

QRegister supports at most 1024 qubits

This is valid:

QRegister<1024>

because "1024" is program-level type information.

---

180. Implementation-Policy Boundary

The compiler MAY impose:

maximum compilation memory
maximum compilation time
maximum solver steps
maximum diagnostic volume
maximum serialized artifact size

for operational safety.

Such policies MUST be:

- configurable where appropriate;
- documented;
- externally distinguishable from semantic invalidity;
- absent from the language's universal type meaning.

---

181. Type-System Security

The type checker MUST treat source input as untrusted.

It MUST:

- validate names;
- validate recursive structures;
- prevent integer overflow;
- avoid unchecked indexing;
- avoid unsafe memory access;
- avoid uncontrolled external execution;
- avoid uncontrolled filesystem access;
- avoid uncontrolled network access.

Type checking MUST be deterministic unless an explicitly declared semantic mechanism says otherwise.

---

182. No Backend Leakage

The type system MUST NOT depend on:

LLVM
MLIR
CUDA
ROCm
OpenQASM
QIR
vendor CPU ISA
vendor GPU ISA
vendor FPGA API
vendor QPU API
specific machine topology
specific device ID

as semantic authorities.

Such systems MAY be used in downstream lowering or interoperability.

---

183. No Hardware Type Explosion

The core language MUST NOT create types such as:

SpecificVendorGpu
SpecificVendorQpu
SpecificVendorFpga
SpecificCpuGeneration
SpecificPhysicalQubit

as universal portable types.

Hardware-specific semantics belong in:

capabilities
resources
dialects
target descriptions
interoperability
backend contracts

---

184. No Duplicate Quantum Type System

There MUST be one semantic quantum type system.

Source grammar may be distributed among the repository's quantum/type grammar files according to ownership contracts.

But semantic meaning MUST converge on one representation.

No second quantum type AST or IR may be introduced merely because another domain needs quantum types.

---

185. No Duplicate Generic System

There MUST be one generic semantic model.

These must not become competing systems:

generic
type-class
associated
higher-kinded
dependent
dialect generic
domain generic

They may be different constructs, but they must share:

parameter identity
constraint model
substitution
scope
kind checking
resolution
compatibility

---

186. No Duplicate Constraint Systems

All type-relevant constraints MUST eventually use the canonical semantic constraint model.

This includes:

- generic bounds;
- associated-type equality;
- shape constraints;
- refinements;
- dependent equality;
- capability requirements where type-dependent;
- resource requirements where type-dependent.

The syntax may live in different grammar files.

The semantic constraint representation must remain unified.

---

187. File Completion Contract

A type-system-related file is not complete merely because its grammar parses.

Before a file is marked complete, the following must already be defined:

PURPOSE
OWNS
DOES NOT OWN
DEPENDS_ON
EXPORTS
CONSUMED_BY
LEXER_DEPENDENCIES
GRAMMAR_DEPENDENCIES
AST_OWNER
SEMANTIC_OWNER
IR_OWNER
SPEC_OWNER
TEST_OWNER
COMPATIBILITY_OWNER
DIAGNOSTICS_CONTRACT
SCALABILITY_CONTRACT
RESOURCE_CONTRACT
CAPABILITY_CONTRACT
EFFECT_CONTRACT
CONTRACT_INTEGRATION
POLICY_INTEGRATION
PROVENANCE_INTEGRATION
COMPLETION_CRITERIA

A downstream implementation can therefore be added later without requiring the upstream specification file to be redesigned.

---

188. "grammar/types/types.g4" Integration Contract

"grammar/types/types.g4" owns:

typeExpression
typeCore
typePrefix
typePostfix
typeValueExpression
typeExtension

It MUST:

- remain parser-only;
- use the canonical lexer;
- avoid lexer rules;
- avoid embedded actions;
- avoid target selection;
- avoid semantic evaluation;
- avoid resource discovery;
- avoid hardware selection.

It MUST expose the single universal type entry point.

---

189. "grammar/types/generic.g4" Integration Contract

Owns:

genericTypeArguments
genericArgumentList
genericTypeApplicationSuffix

It MUST:

- represent generic applications;
- preserve argument ordering;
- support arbitrary semantic arity;
- avoid declaring generic parameters;
- avoid defining type inference;
- avoid defining substitution;
- avoid defining type-class resolution.

AST destination:

TypeExpr::Generic

or the canonical equivalent.

---

190. "grammar/types/bounds.g4" Integration Contract

Owns source syntax for generic bounds.

Semantic destination:

TypeConstraint

It MUST NOT define a second trait/type-class system.

---

191. "grammar/types/linear.g4" Integration Contract

Owns linear source syntax.

Semantic destination:

linear ownership qualifier

The ownership checker consumes it.

No resource allocation occurs here.

---

192. "grammar/types/affine.g4" Integration Contract

Owns affine source syntax.

Semantic destination:

affine ownership qualifier

The ownership checker consumes it.

---

193. "grammar/types/dependent.g4" Integration Contract

Owns syntax for:

Pi
Sigma
Identity
type-level dependent values

AST destination:

TypeExpr::Pi
TypeExpr::Sigma
TypeExpr::Identity
TypeValueExpr

Semantic destination:

dependent semantic type
constraint/proof obligations

---

194. "grammar/types/quantum.g4" Integration Contract

Owns source syntax for quantum type constructs that are genuinely type-specific.

Semantic destination:

quantum semantic type

IR destination:

quantum::ir

It MUST NOT:

- assign physical qubits;
- enumerate hardware;
- choose a QPU;
- encode QEC implementation.

---

195. "grammar/types/hardware.g4" Integration Contract

Owns hardware-intent type syntax.

It MUST express portable intent.

It MUST NOT encode universal vendor inventories.

Hardware capability and resource resolution occur downstream.

---

196. "grammar/types/resource.g4" Integration Contract

Owns source syntax for resource-related type constructs.

It MUST consume the canonical resource model.

It MUST NOT independently define:

resource availability

or:

hardware discovery

---

197. "grammar/types/capability.g4" Integration Contract

Owns source syntax for capability-qualified type constructs where applicable.

Capability identity comes from the canonical capability model.

Capability satisfaction occurs downstream.

---

198. "grammar/types/effectful.g4" Integration Contract

Owns source syntax for effect-qualified types where applicable.

Effect identity comes from:

grammar/effects/

No duplicate effect vocabulary may be defined here.

---

199. "src/frontend/ast/node/types/type_expr.rs" Integration Contract

This file is the canonical source-level "TypeExpr" representation.

It MUST remain:

- source-oriented;
- target-neutral;
- deterministic;
- safe Rust;
- free of hardware allocation;
- free of backend selection.

It MAY expose safe convenience APIs around "TypeExpr".

It MUST NOT become the semantic type checker itself.

---

200. Semantic Type Implementation Contract

The semantic type implementation must provide:

resolution
normalization
equality
compatibility
subtyping where supported
substitution
unification
kind checking
constraint solving
ownership qualifiers
effect integration
capability integration
resource integration

It MUST operate independently of backend-specific target selection.

---

201. Suggested Semantic Implementation Dependency Order

The implementation should be completed in this dependency order:

type identity
      ↓
type paths / names
      ↓
primitive semantic types
      ↓
type constructors
      ↓
generic parameters
      ↓
generic application
      ↓
kind checking
      ↓
type substitution
      ↓
type normalization
      ↓
type equality
      ↓
constraints
      ↓
unification
      ↓
associated types
      ↓
variance/subtyping
      ↓
ownership
      ↓
linear/affine
      ↓
lifetimes
      ↓
dependent values
      ↓
refinement/proof obligations
      ↓
effects
      ↓
capabilities
      ↓
resources
      ↓
domain types
      ↓
semantic finalization
      ↓
IR lowering

No later subsystem should need to reinterpret the meaning of an earlier completed type contract.

---

202. Type-System Feature Completion

A feature is production-ready only when:

SPECIFICATION
    ↓
LEXER
    ↓
SYNTAX
    ↓
AST
    ↓
STRUCTURAL VALIDATION
    ↓
NAME RESOLUTION
    ↓
TYPE SEMANTICS
    ↓
INFERENCE
    ↓
CONSTRAINT SOLVING
    ↓
OWNERSHIP
    ↓
EFFECTS
    ↓
CAPABILITIES
    ↓
RESOURCES
    ↓
CONTRACTS/POLICIES
    ↓
SEMANTIC MODEL
    ↓
IR
    ↓
IR VERIFICATION
    ↓
LOWERING
    ↓
TARGET INTEGRATION
    ↓
TESTS

where the relevant stages apply.

---

203. Cross-Domain Completion Test

At least one integrated conformance program MUST combine:

generic types
+
symbolic dimensions
+
classical computation
+
tensor computation
+
quantum resource
+
quantum operation
+
measurement
+
linear ownership
+
effects
+
capabilities
+
resource requirements
+
contracts
+
policies
+
provenance
+
distributed computation
+
HDL/hardware intent

The pipeline MUST successfully demonstrate:

source
 ↓
AST
 ↓
type resolution
 ↓
constraint solving
 ↓
ownership checking
 ↓
effect checking
 ↓
capability checking
 ↓
resource checking
 ↓
contract/policy validation
 ↓
semantic type model
 ↓
Classical IR / quantum::ir / domain IR

---

204. POCO-REAF Conformance

The same semantic source program SHOULD be testable against multiple target profiles without modifying the source type declarations.

For example:

CPU
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC
cluster
distributed
future target

The type semantics MUST remain stable.

Only:

capability satisfaction
resource feasibility
specialization
lowering
routing
scheduling
resilience

may differ.

---

205. Future Computational Domains

A future computational domain may add:

types
generic constructors
constraints
effects
capabilities
resources
dialect types
lowering rules

without changing the semantics of existing types.

The extension MUST provide:

type identity
syntax
AST mapping
semantic definition
constraints
compatibility
effects
capabilities
resources
IR mapping
tests
versioning

---

206. Production Readiness Checklist

"grammar/spec/type-system.md" is production-ready only when all of the following are true:

- [ ] Type authority is unambiguous.
- [ ] Type syntax is delegated to "grammar/types/".
- [ ] "TypeExpr" is the sole canonical source type representation.
- [ ] Semantic types are distinct from source "TypeExpr".
- [ ] Generic application is distinct from generic declaration.
- [ ] Generic constraints have one semantic model.
- [ ] Associated types have one semantic model.
- [ ] Type-level values have one semantic model.
- [ ] Dependent types have defined semantics.
- [ ] Refinement types have defined semantics where implemented.
- [ ] Ownership semantics are defined.
- [ ] Linear semantics are defined.
- [ ] Affine semantics are defined.
- [ ] Lifetime semantics are defined.
- [ ] Effects integrate without duplicate ownership.
- [ ] Capabilities integrate without duplicate ownership.
- [ ] Resources integrate without duplicate ownership.
- [ ] Contracts integrate without duplicate ownership.
- [ ] Policies integrate without duplicate ownership.
- [ ] Provenance integrates without changing type identity.
- [ ] Quantum types have a single semantic boundary.
- [ ] "quantum::ir" remains the canonical quantum IR boundary.
- [ ] Classical types integrate with canonical Classical IR.
- [ ] HDL/hardware types remain target-independent.
- [ ] Distributed types remain placement-independent.
- [ ] Networking types remain transport-independent.
- [ ] Foreign types are explicitly distinguished.
- [ ] ABI semantics do not leak into portable types.
- [ ] Type equality is deterministic.
- [ ] Constraint solving is deterministic.
- [ ] Compiler resource exhaustion is distinct from semantic invalidity.
- [ ] Semantic quantities do not silently narrow to host integers.
- [ ] No universal hardware capacity is hard-coded.
- [ ] No backend becomes the type-system authority.
- [ ] Safe Rust implementation is possible.
- [ ] Production Rust uses no "unsafe".
- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Boundary tests exist.
- [ ] Scalability tests exist.
- [ ] Determinism tests exist.
- [ ] Cross-target tests exist.
- [ ] Compatibility tests exist.
- [ ] IR integration tests exist.
- [ ] Every participating file has an explicit ownership/integration contract.

---

207. Final Semantic Invariants

The following invariants are mandatory.

Invariant 1 — Type is not hardware

Type ≠ physical machine

Invariant 2 — Type is not allocation

Type ≠ physical allocation

Invariant 3 — Resource feasibility is separate

Type validity ≠ resource availability

Invariant 4 — Capability feasibility is separate

Type validity ≠ capability availability

Invariant 5 — Target realization is downstream

semantic type
    ↓
canonical IR
    ↓
target realization

Invariant 6 — Quantum semantics have one canonical boundary

quantum source semantics
    ↓
quantum::ir

Invariant 7 — Source AST has one authority

TypeExpr

is the canonical source-level representation.

Invariant 8 — Genericity is open-ended

New generic abstractions MUST NOT require a finite universal inventory.

Invariant 9 — Semantic quantities do not depend on host width

semantic cardinality ≠ usize

Invariant 10 — Compiler limits are not language limits

compiler budget
≠
type-system maximum

Invariant 11 — Safe implementation

The production compiler MUST remain safe Rust.

Invariant 12 — Determinism

Identical semantic inputs MUST produce identical type-analysis results under the same language and implementation profile.

Invariant 13 — No silent semantic weakening

A backend MUST NOT silently weaken:

- type guarantees;
- ownership;
- linearity;
- affinity;
- effects;
- capabilities;
- contracts;
- refinements;
- quantum correctness.

Invariant 14 — Open-world computation

A new hardware platform, computational domain, accelerator, quantum technology, or future execution model MUST be able to consume existing semantic types without redefining their meaning.

---

208. Final Architecture

The complete type-system architecture is:

                    ZAMANI SOURCE
                         │
                         ▼
                  canonical lexer
                         │
                         ▼
                  canonical parser
                         │
                         ▼
                     TypeExpr
                         │
                         ▼
              structural AST validation
                         │
                         ▼
                   name resolution
                         │
                         ▼
                 type resolution
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
      generics       constraints      shapes
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                  type normalization
                         │
                         ▼
                    unification
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
      ownership       effects      capabilities
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                     resources
                         │
                         ▼
                  contracts/policies
                         │
                         ▼
                 semantic type model
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
      Classical       Quantum        HDL/Hardware
          │              │              │
          ▼              ▼              ▼
   Classical IR     quantum::ir     domain IR
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                     optimization
                         │
                         ▼
                      lowering
                         │
                ┌────────┼────────┐
                ▼        ▼        ▼
             routing  scheduling resilience
                                  │
                                  ▼
                               QEC/ZQN
                                  │
                                  ▼
                                 HAL
                                  │
                                  ▼
                         target realization

The resulting invariant is:

                         TYPE
                          │
          ┌───────────────┼────────────────┐
          ▼               ▼                ▼
       meaning         constraints       guarantees
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                 semantic type model
                          │
                          ▼
                    canonical IR
                          │
                          ▼
              realization-independent
                    optimization
                          │
          ┌───────────────┼────────────────┐
          ▼               ▼                ▼
       CPU/GPU        FPGA/ASIC        QPU/Simulator
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                  HPC / distributed
                          │
                          ▼
                   future targets

A Zamani type therefore remains a semantic contract, not a machine prescription.

That is the required foundation for a type system capable of scaling from the smallest meaningful computation to arbitrarily large finite computations supported by the actual available resources, while preserving portability, type safety, quantum correctness, hardware neutrality, deterministic semantics, safe Rust implementation, and the POCO-REAF architecture.