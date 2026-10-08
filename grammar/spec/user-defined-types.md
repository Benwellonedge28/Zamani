Zamani User-Defined Types

Path: "grammar/spec/user-defined-types.md"
Repository: "Benwellonedge28/Zamani"
Status: Normative production specification
Specification layer: "grammar/spec/"
Language: Zamani
Rust baseline: Rust 1.97.1 or later
Rust edition: Rust 2021
Rust safety: Safe Rust only; production Zamani compiler code MUST NOT use "unsafe"
Architecture: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

This document defines the normative production contract for user-defined types (UDTs) in Zamani.

User-defined types are a fundamental part of the language type system. They allow programmers and libraries to introduce domain-specific semantic types without modifying the universal Zamani grammar for every new application, algorithm, hardware platform, quantum technology, data model, or computational domain.

A Zamani user-defined type MUST be able to participate in the same universal type system as built-in, generic, classical, quantum, HDL, hardware, AI, data, distributed, networking, accelerator, and future-domain types.

The UDT system MUST therefore provide:

- nominal types;
- structural aggregate types where explicitly supported;
- records/structs;
- enumerations;
- sum/union types;
- type aliases;
- generic types;
- generic type parameters;
- type bounds;
- constraints;
- associated types;
- traits/type classes/interfaces where supported;
- recursive types;
- mutually recursive types;
- parameterized types;
- symbolic/value-parameterized types;
- refinement/contract participation;
- linear types;
- affine types;
- references and ownership;
- effect participation;
- capability participation;
- resource participation;
- policy participation;
- provenance;
- dialect extension;
- foreign/ABI type integration;
- quantum type integration;
- HDL/hardware type integration;
- deterministic semantic identity;
- target-independent representation;
- scalable realization.

The UDT system MUST NOT become a second type system separate from Zamani's canonical type system.

---

2. Normative status

The following terms are normative:

- MUST — mandatory.
- MUST NOT — prohibited.
- SHOULD — recommended unless a documented architectural reason requires otherwise.
- SHOULD NOT — normally prohibited unless justified.
- MAY — permitted.
- IMPLEMENTATION-DEFINED — determined by the implementation within the constraints of this specification.
- TARGET-DEFINED — determined by target capabilities or realization.
- DIALECT-DEFINED — determined by an explicitly registered dialect.

A feature MUST NOT be considered production-ready merely because its grammar exists.

Production readiness requires the complete path:

Specification
    ↓
Lexer
    ↓
Grammar
    ↓
AST
    ↓
Structural validation
    ↓
Name resolution
    ↓
Type resolution
    ↓
Constraint checking
    ↓
Generic/bound checking
    ↓
Ownership / linearity / affinity
    ↓
Effects
    ↓
Capabilities
    ↓
Resources
    ↓
Contracts
    ↓
Policies
    ↓
Provenance
    ↓
Canonical semantic type
    ↓
Canonical IR
    ↓
Domain lowering
    ↓
Target realization
    ↓
Tests

---

3. Architectural authority

This file is the normative specification for user-defined type semantics.

It MUST integrate with, but MUST NOT replace:

grammar/DESIGN.md
grammar/spec/type-system.md
grammar/specification/types.md
grammar/types/types.g4
grammar/declarations/declarations.g4
grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md

Authority is divided as follows.

Concern| Authority
Overall grammar architecture| "grammar/DESIGN.md"
Grammar navigation| "grammar/README.md"
Conformance status| "grammar/grammar.md"
Historical/extended grammar material| "grammar/Zamani-Grammar.md"
General type-system architecture| "grammar/spec/type-system.md"
Normative general type semantics| "grammar/specification/types.md"
UDT semantics| this document
Universal type-expression grammar| "grammar/types/types.g4"
Named type syntax| "grammar/types/named.g4"
Generic syntax| "grammar/types/generic.g4" / canonical generic owner
Record/struct syntax| corresponding declaration/type grammar
Enum syntax| corresponding declaration grammar
Sum/union syntax| corresponding declaration/type grammar
Declaration dispatch| "grammar/declarations/declarations.g4"
AST type representation| "src/frontend/ast/node/types/"
Legacy AST compatibility| existing "src/ast/" boundary until migration is complete
Semantic type resolution| semantic/type-analysis subsystem
Classical IR| canonical classical IR
Quantum IR| "quantum::ir"
Target realization| downstream compiler/backend/HAL

If two documents conflict, the conflict MUST be resolved explicitly and the affected conformance status MUST NOT be marked stable until reconciliation is complete.

---

4. Ownership contract

4.1 This document owns

This document owns:

- the meaning of a UDT;
- UDT categories;
- UDT identity;
- UDT declaration semantics;
- UDT generic semantics;
- UDT bounds;
- UDT constraints;
- UDT visibility requirements;
- UDT recursion rules;
- UDT composition rules;
- UDT type compatibility;
- UDT nominal/structural distinction;
- UDT alias distinction;
- UDT interaction with ownership;
- UDT interaction with linearity;
- UDT interaction with affinity;
- UDT interaction with effects;
- UDT interaction with capabilities;
- UDT interaction with resources;
- UDT interaction with contracts;
- UDT interaction with policies;
- UDT provenance;
- UDT dialect integration;
- UDT foreign-type integration;
- UDT quantum integration;
- UDT HDL/hardware integration;
- UDT portability requirements;
- UDT scalability requirements;
- UDT diagnostics;
- UDT conformance requirements.

4.2 This document does not own

This document does not own:

- lexer token definitions;
- token spelling;
- universal parser composition;
- implementation of "TypeExpr";
- concrete parser grammar;
- generic parser grammar;
- struct parser grammar;
- enum parser grammar;
- expression grammar;
- function grammar;
- module grammar;
- semantic resolver implementation;
- compiler optimization;
- physical memory layout;
- ABI implementation;
- quantum routing;
- quantum scheduling;
- quantum error correction;
- QPU selection;
- hardware discovery;
- backend implementation;
- ZQN implementation;
- HAL implementation;
- runtime allocation;
- operating-system behavior.

Those responsibilities remain with their existing owners.

---

5. Existing repository integration

The repository already provides a substantial type architecture.

The existing canonical type grammar identifies "typeExpression" as the universal source-level type entry point.

This specification therefore MUST NOT introduce:

userDefinedTypeExpression
udtTypeExpression
universalUserTypeExpression
secondTypeExpression

as competing universal type roots.

The correct architecture is:

UDT declaration
       ↓
declaration grammar
       ↓
domain-neutral AST
       ↓
canonical TypeExpr references
       ↓
semantic type resolver
       ↓
canonical semantic type

A user-defined type is therefore both:

1. a declaration; and
2. a type identity that can later be referenced by a normal type expression.

---

6. Canonical UDT model

A user-defined type consists conceptually of:

UserDefinedType
├── identity
├── declaration kind
├── namespace
├── visibility
├── modifiers
├── generic parameters
├── bounds
├── constraints
├── representation-independent definition
├── members / variants
├── associated items
├── contracts
├── effects
├── capabilities
├── resources
├── policies
├── provenance
└── compatibility metadata

Not every UDT requires every component.

For example:

type UserId = Integer

has an identity and target type but no aggregate members.

A record has fields.

An enum has variants.

A generic type has parameters.

A trait/interface has required associated behavior.

A resource-bearing type MAY have resource semantics.

---

7. Core principle: type meaning versus representation

A UDT describes semantic meaning.

It MUST NOT require a particular physical representation unless the type is explicitly declared as representation-constrained.

For example:

type User {
    id: Integer
    name: String
}

does not inherently specify:

- byte alignment;
- memory address;
- CPU register layout;
- cache placement;
- GPU memory;
- FPGA registers;
- ASIC cells;
- network serialization;
- quantum physical representation.

Those are downstream concerns.

This separation is required for POCO-REAF.

---

8. Nominal identity

A nominal UDT receives its identity from its declaration.

For example:

type UserId = Integer

and:

type AccountId = Integer

MUST NOT automatically become the same semantic type merely because their underlying representations are equivalent.

If the language defines these as aliases rather than nominal types, the distinction MUST be explicit.

A nominal identity MUST be based on a stable declaration identity such as:

module
namespace
qualified name
declaration identity
generic arguments where applicable

It MUST NOT be based solely on:

field order
field names
memory layout
compiler address
source-file path alone
backend type
machine architecture
physical resource

---

9. Type aliases

A type alias is a different semantic category from a new nominal type.

Conceptually:

type UserId = Integer

may mean:

alias identity
    ↓
Integer

whereas a nominal declaration means:

UserId
    ↓
distinct semantic type

The implementation MUST preserve this distinction.

Aliases MUST NOT accidentally create nominal identity.

Nominal types MUST NOT accidentally collapse into aliases.

---

10. Records and structs

UDTs MAY define product types.

Example:

type User {
    id: Integer
    name: String
}

A record/struct type consists of:

type identity
fields
field names
field types
field visibility
field attributes
field constraints
field defaults where supported
contracts
generic parameters

Field count MUST NOT have a language-level artificial maximum.

The grammar MUST NOT define:

MAX_STRUCT_FIELDS

or equivalent.

Any operational parser/compiler safety limit MUST be an explicit tool/resource policy and MUST NOT alter language semantics.

---

11. Field identity

A field MUST have a semantic identity determined by the containing type and field name or another explicitly specified declaration identity.

Field identity MUST NOT depend on:

- physical memory offset;
- machine word size;
- register width;
- target ABI;
- cache line;
- FPGA placement;
- GPU address;
- quantum hardware location.

---

12. Enumerations

UDTs MAY define enumerated types.

Conceptually:

type Result {
    Success
    Failure
    Pending
}

Each variant has a stable semantic identity.

Variant count MUST NOT have an artificial global maximum.

An enum MAY later be represented differently on different targets.

For example:

integer tag
bit pattern
branch representation
tagged object
hardware state
distributed state

provided semantic behavior remains equivalent.

---

13. Sum and union types

UDTs MAY define sum types.

Conceptually:

type Value =
    | IntegerValue(Integer)
    | FloatValue(Float)
    | TextValue(String)

A sum type represents one of multiple alternatives.

The semantic type MUST preserve:

- variant identity;
- payload type;
- generic parameters;
- constraints;
- contracts;
- provenance.

A backend MAY optimize representation, but it MUST NOT change which alternatives are semantically valid.

---

14. Recursive UDTs

Recursive types are permitted where their semantic representation is well-founded.

For example:

type Node {
    value: Integer
    next: Reference<Node>
}

The language MUST distinguish:

recursive semantic type

from:

infinitely expanded concrete value

The compiler MUST represent recursive type relationships symbolically.

It MUST NOT recursively materialize an infinite type structure.

---

15. Mutually recursive types

Mutually recursive declarations MAY be supported.

Example:

type A {
    value: Reference<B>
}

type B {
    value: Reference<A>
}

Resolution MUST operate over a type-declaration graph rather than requiring declarations to be fully expanded before either can be resolved.

Cycles that are semantically legal MUST be accepted.

Cycles that violate representation, ownership, initialization, or other language rules MUST receive precise diagnostics.

---

16. Generic UDTs

UDTs MUST support generic parameters where the generic subsystem supports them.

Example:

type Pair<T, U> {
    first: T
    second: U
}

Generic parameters are semantic parameters.

They MUST NOT be treated as a finite enumerated list.

The implementation MUST NOT define:

MAX_GENERIC_PARAMETERS
MAX_GENERIC_ARGUMENTS

as language rules.

---

17. Generic parameter kinds

The architecture SHOULD distinguish at least:

type parameter
value parameter
const-like parameter
lifetime/region parameter where supported
capability parameter where supported
resource parameter where supported

The exact syntax remains owned by the generic/type grammar.

The semantic model MUST distinguish parameter kinds.

A type parameter:

T

is not equivalent to a runtime value.

A symbolic value parameter:

N

is not automatically a machine integer with a fixed width.

---

18. Generic bounds

UDTs MAY constrain generic parameters.

Conceptually:

type Buffer<T> where T: Serializable {
    data: T
}

Bounds MUST be resolved semantically.

The grammar MUST NOT enumerate all possible future bounds.

A bound MAY refer to:

- trait;
- interface;
- type class;
- capability;
- structural property;
- domain property;
- semantic constraint.

---

19. Generic constraints

A generic constraint is a semantic obligation.

Example:

T satisfies Numeric

is different from:

T is Integer

The first permits an open set of valid types.

The second identifies one type.

The implementation MUST preserve this distinction.

---

20. Associated types

Where associated types are supported, UDTs MAY participate in traits/interfaces/type classes containing associated types.

Conceptually:

Trait
├── associated type
├── required methods
└── constraints

A projection such as:

T::Item

MUST be resolved by the associated-type subsystem.

Not every "::" syntax sequence is automatically an associated-type projection.

The existing associated-type grammar remains authoritative for its syntax.

---

21. Traits, interfaces, and type classes

UDTs MAY satisfy traits/interfaces/type classes.

These constructs MUST remain semantically distinct from ordinary inheritance unless Zamani explicitly defines them as equivalent.

A trait/interface/type-class relationship establishes semantic capabilities or obligations.

It does not inherently establish:

- memory layout;
- inheritance layout;
- hardware representation;
- object identity;
- physical device binding.

---

22. Implementations

A UDT MAY have implementations associated with it.

Conceptually:

type
    ↓
implementation
    ↓
methods / associated items

Implementation validity MUST be checked by semantic analysis.

The grammar MUST NOT attempt to prove:

- trait satisfaction;
- coherence;
- method compatibility;
- generic consistency;
- ownership correctness.

---

23. Coherence

Where the language defines coherence rules, implementation resolution MUST be deterministic.

The compiler MUST NOT choose between competing implementations based on:

- filesystem traversal order;
- hash-map iteration order;
- compilation thread order;
- machine architecture;
- backend selection.

Ambiguity MUST be diagnosed.

---

24. Type identity

A UDT's semantic identity MUST be stable.

Identity MUST NOT change because the compiler selects:

- CPU;
- GPU;
- FPGA;
- ASIC;
- accelerator;
- QPU;
- simulator;
- cluster;
- distributed target.

For example:

Tensor<Float>

remains the same semantic type whether realized by:

CPU memory
GPU memory
FPGA storage
distributed shards
tensor accelerator

provided the realization satisfies its semantic contract.

---

25. Structural versus nominal compatibility

Zamani MUST explicitly distinguish:

nominal compatibility

from:

structural compatibility

A nominal type does not become compatible merely because two structures look identical unless the language explicitly permits structural compatibility in that context.

Structural types MAY be compatible based on their declared semantic structure.

The implementation MUST NOT silently switch between nominal and structural rules.

---

26. Conversion

A conversion between UDTs MUST be one of:

identity
implicit conversion
explicit conversion
constructor conversion
coercion
serialization/deserialization
foreign/ABI conversion
domain conversion

Each category MUST have defined semantics.

The compiler MUST NOT silently perform a potentially lossy or effectful conversion merely because two representations are compatible.

---

27. Representation independence

A UDT MUST NOT expose target representation accidentally.

For example:

type Matrix<T, Rows, Columns>

does not mean:

fixed machine matrix

It means a semantic matrix with symbolic or resolved dimensions.

The dimensions may be:

- compile-time constants;
- generic parameters;
- symbolic values;
- dependent values where supported;
- runtime-known values where explicitly permitted.

---

28. Symbolic dimensions

UDTs MAY contain symbolic dimensions.

Examples include:

Tensor<T, Shape>
Matrix<T, Rows, Columns>
Vector<T, N>
QRegister<N>

The type system MUST preserve symbolic information where required.

It MUST NOT force symbolic quantities into a fixed machine width prematurely.

---

29. Dependent/value-parameterized types

Dependent types are a planned advanced capability unless already implemented by the corresponding semantic subsystem.

The architecture MUST nevertheless reserve a clean semantic boundary.

A type MAY conceptually depend on a value:

Buffer<T, N>

where "N" is a semantic parameter.

This does not mean:

N <= fixed compiler maximum

The valid range is governed by language semantics and target feasibility.

---

30. Refinement and contracts

A UDT MAY participate in refinement-like semantics.

Conceptually:

type PositiveInteger = Integer
    where value > 0

The exact syntax MUST be owned by the contract/refinement subsystem.

The type system consumes the resulting semantic constraint.

The implementation MUST distinguish:

type identity

from:

proof that a value satisfies a refinement

A runtime check MAY be required if a property cannot be established statically and the language permits runtime validation.

---

31. Contracts

UDTs MUST integrate with the universal contract model.

Relevant constructs include:

requires
ensures
invariant
assume
guarantee
property
assert

A contract attached to a type or member MUST become semantic metadata.

The type grammar MUST NOT reimplement the contract language.

The contract subsystem owns:

- evaluation;
- validation;
- proof status;
- diagnostics;
- runtime enforcement where applicable.

---

32. Effects

UDTs MAY participate in effect-qualified semantics.

Effects include, where supported:

io
network
mutation
randomness
native
foreign
distributed
measurement
quantum
learning
adaptation
reflection
code_generation
simulation

A type declaration itself MUST NOT acquire an effect merely because it is user-defined.

An effect arises from a semantic operation or explicitly effectful declaration.

For example:

type definition

is not inherently a network effect.

But:

method that performs network I/O

MAY carry a network effect.

---

33. Capability integration

UDTs MAY require or expose capabilities.

For example, a domain type may semantically require:

capability("quantum.measurement")

or:

capability("tensor.compute")

Capabilities describe what a target can provide.

They do not identify a specific device.

A UDT MUST NOT contain a hard-coded physical device identity merely because it requires a capability.

---

34. Resource integration

UDTs MAY carry semantic resource requirements.

Examples:

requires qubits >= N
requires memory >= RequiredMemory
requires topology(RequiredTopology)

Resource requirements are not type identity.

The following MUST remain separate:

Type
Resource
Requirement
Capability
Constraint
Preference
Hint
Target

A type MUST NOT become invalid merely because a particular target lacks sufficient resources.

Instead:

type valid
+
target infeasible

MUST remain distinguishable from:

type invalid

---

35. No hard-coded capacity

The UDT architecture MUST NOT contain universal constants such as:

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
MAX_GENERIC_PARAMETERS
MAX_STRUCT_FIELDS
MAX_ENUM_VARIANTS
MAX_TUPLE_ARITY
MAX_TYPE_DEPTH

This prohibition applies to language semantics.

A compiler service MAY impose an explicit operational limit to prevent resource exhaustion.

Such a limit MUST be represented as:

compiler policy
resource budget
execution policy
security policy

and MUST NOT be interpreted as a language-level semantic ceiling.

---

36. "Infinity" and practical computation

The architectural requirement to scale to "infinity" means:

«Zamani MUST NOT encode an artificial finite maximum in its language semantics.»

It does not mean a physical machine has infinite resources.

Every actual compilation remains finite.

Therefore:

semantic scalability

means:

no artificial language ceiling

while:

physical feasibility

means:

the selected realization has sufficient actual resources

---

37. Quantum UDT integration

Quantum types MUST participate in the universal type system.

Examples include:

Qubit
LogicalQubit
QRegister<N>
QuantumState<T>
MeasurementResult

These are semantic concepts.

They MUST NOT directly encode:

- physical qubit IDs;
- QPU vendor;
- coupling map;
- calibration;
- pulse schedule;
- physical qubit count;
- QEC implementation.

Quantum source types MUST follow:

source UDT/type
    ↓
TypeExpr
    ↓
semantic quantum type
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

---

38. Quantum linearity

Quantum resources often require non-copying semantics.

The type system MUST therefore permit integration with:

linear types
affine types
ownership
borrowing
resource semantics

A quantum value MUST NOT automatically become copyable merely because its host-language representation is cheap to copy.

Whether a particular quantum value is:

copyable
linear
affine
move-only
borrowable

is a semantic property.

---

39. Quantum physical identity

A physical qubit is not automatically the same thing as a source-level "Qubit".

For example:

source Qubit

may later map to:

physical qubit 7

or:

logical qubit encoded using multiple physical qubits

The source type remains semantically unchanged.

Physical identity belongs downstream.

---

40. HDL and hardware UDT integration

UDTs MUST be usable in HDL and hardware/software co-design.

Examples include:

Signal<T>
Bus<T>
Register<T>
Memory<T>
Channel<T>
Interface<T>
Accelerator<T>

These are semantic abstractions.

A source type MUST NOT silently become:

32-bit register
fixed FPGA block
fixed bus width
fixed number of lanes
fixed hardware address

unless those properties are explicitly part of the type's declared semantics.

---

41. Symbolic hardware parameters

Hardware-oriented UDTs MAY use symbolic parameters.

Examples:

Bus<T, Width>
Matrix<T, Rows, Columns>
Memory<T, Capacity>

The language MUST NOT define arbitrary universal limits for these parameters.

Hardware realization resolves them according to:

requirements
capabilities
constraints
preferences
target profiles
resource availability

---

42. AI and data UDT integration

AI/data types MUST reuse the universal UDT system.

Examples:

Tensor<T, Shape>
Dataset<T>
Model<Input, Output>
Distribution<T>
Evidence<T>
Probability<T>
Confidence<T>
Graph<Node, Edge>

The language MUST NOT create a separate AI type universe.

These types participate in:

generics
constraints
effects
resources
capabilities
contracts
provenance
policies

---

43. Reasoning and knowledge integration

UDTs MAY represent:

Fact
Claim
Evidence
Belief
Decision
Explanation
Knowledge
InferenceResult

These remain ordinary semantic types.

Reasoning operations such as:

infer
deduce
reason
query
assert
retract

belong to their appropriate semantic/expression/statement owners.

They consume UDTs through the canonical type system.

---

44. Learning and adaptation integration

Types MAY represent:

Model
Dataset
TrainingState
LearningResult
AdaptationPlan
Policy
Feedback

Learning and adaptation semantics remain owned by their respective subsystems.

A UDT MUST NOT automatically permit unrestricted self-modification.

Adaptation MUST remain subject to:

effects
capabilities
resources
authorization
policy
contracts
provenance

---

45. Uncertainty and probability

UDTs MAY represent uncertainty.

Examples:

Probability<T>
Distribution<T>
Confidence<T>
Uncertain<T>
Belief<T>

The type system MUST NOT require a single probability implementation.

Different dialects or libraries MAY provide different mathematical models provided they integrate through the canonical type system.

---

46. Provenance

UDTs MUST support provenance metadata where required.

Provenance MAY record:

declared_by
derived_from
generated_by
transformed_by
verified_by
source
evidence
decision
version

Provenance MUST NOT become part of nominal type identity unless explicitly specified.

Two values of the same semantic type MAY have different provenance.

---

47. Determinism

UDT resolution MUST be deterministic.

Given the same:

source
language version
module set
dialect configuration
type environment
semantic configuration

type resolution MUST produce the same semantic result.

The implementation MUST NOT make semantic decisions dependent on:

- hash-map iteration order;
- filesystem traversal order;
- thread scheduling;
- target discovery order;
- backend ordering.

Where unordered structures are used internally, semantic output MUST be canonicalized.

---

48. Generic specialization

Generic specialization MUST preserve semantic identity.

For:

Pair<Integer, String>

the compiler MAY generate specialized representations such as:

CPU specialization
GPU specialization
FPGA specialization
distributed specialization

but all specializations MUST represent the same source-level generic instantiation.

---

49. Monomorphization is not mandatory semantics

A compiler MAY use:

monomorphization
dictionary passing
erasure
specialization
partial evaluation
runtime generics
symbolic lowering

These are implementation strategies.

The language specification MUST NOT require one particular strategy unless required for semantic correctness.

---

50. Type erasure

If the implementation erases generic information, it MUST retain enough information to preserve all observable language semantics.

Erasure MUST NOT eliminate information required for:

- contracts;
- effects;
- capabilities;
- resources;
- ownership;
- runtime checks;
- reflection;
- provenance;
- ABI;
- serialization;
- interoperability.

---

51. Reflection

Controlled reflection MAY inspect UDTs.

Reflection MUST operate against the canonical semantic type model.

It MUST NOT construct a hidden second type system.

Reflection MUST respect:

visibility
authorization
capabilities
effects
policy
provenance

Compile-time reflection and runtime reflection MUST remain distinguishable.

---

52. Metaprogramming

Macros and metaprograms MAY generate UDT declarations.

Generated declarations MUST pass through the same:

lexical validation
syntactic validation
structural validation
type validation
semantic validation

as handwritten declarations.

Metaprogramming MUST NOT bypass type safety.

---

53. Dialect integration

A dialect MAY introduce new UDT constructors or domain-specific UDT declarations.

A dialect MUST declare:

dialect identity
version
namespace
syntax owner
semantic owner
AST representation
type representation
capabilities
effects
resources
lowering contract
compatibility
provenance

A dialect MUST NOT silently redefine the meaning of a core Zamani type.

---

54. Open-world extensibility

Adding:

new quantum technology
new accelerator
new tensor architecture
new AI model family
new network abstraction
new hardware family
new computational paradigm

MUST NOT require modifying the universal UDT semantics merely because the new technology introduces new types.

The preferred mechanism is:

named type
qualified type
generic type
dialect type
capability
resource
semantic registration

---

55. Application-specific abstractions

Application concepts SHOULD be represented as library or dialect types rather than universal keywords.

Examples include:

VisionModel
Robot
Payment
LegalDocument
Blockchain
VirtualWorld
SentimentModel
AdministrativeAction

These are not universal primitive types.

They SHOULD be ordinary user-defined/library/dialect types built on Zamani's universal foundations.

---

56. FFI and ABI integration

Foreign types MUST remain distinct from native UDT semantics.

For example:

extern type ForeignNumber

represents an interoperability boundary.

The foreign type MAY specify:

ABI
calling convention
layout
ownership
alignment
serialization
foreign effects

but those properties MUST NOT leak into ordinary portable UDTs.

FFI operations MUST participate in the effect system.

---

57. Safe Rust implementation

The reference compiler implementation MUST use:

Rust 1.97.1 or later
Rust 2021
safe Rust

Production Zamani compiler code MUST NOT use:

unsafe

or:

unsafe {}

or unsafe trait/implementation mechanisms.

The type subsystem SHOULD use safe abstractions such as:

Vec
Box
Rc
Arc
Option
Result
BTreeMap
BTreeSet
HashMap
HashSet

where appropriate.

Any foreign boundary that would traditionally require unsafe Rust MUST be isolated behind an approved safe architectural boundary and MUST NOT make the type checker depend on unchecked memory operations.

---

58. AST contract

The parser MUST produce the repository's existing domain-neutral AST.

The UDT system MUST NOT create a second AST hierarchy.

The existing architecture already provides UDT-related declaration concepts including:

Struct
Enum
Trait
Impl
TypeAlias
TypeDeclaration

The frontend AST type system already contains a canonical "TypeExpr" boundary.

The implementation MUST converge these representations rather than introducing another UDT AST.

The AST MUST preserve:

- source span;
- declaration identity;
- declaration kind;
- visibility;
- attributes;
- modifiers;
- generic parameters;
- bounds;
- constraints;
- members;
- field types;
- variant types;
- associated items;
- documentation where supported;
- source ordering.

---

59. AST versus semantic type

The AST answers:

«What did the programmer write?»

The semantic type system answers:

«What does that declaration mean?»

For example:

AST:
    named type "User"

Semantic:
    nominal type identity
    declaration reference
    generic environment
    constraints
    members
    capabilities
    resource semantics

The AST MUST NOT contain target-specific semantic decisions.

---

60. Name resolution

Name resolution MUST precede final semantic UDT resolution.

A type name MAY resolve to:

primitive type
nominal UDT
alias
generic parameter
associated type
dialect type
foreign type
domain type
type constructor

The resolver MUST distinguish these cases.

An unresolved name MUST produce a structured diagnostic.

---

61. Namespace resolution

UDTs MUST support arbitrary valid namespace/module qualification.

For example:

domain.module.Type

The number of namespace components MUST NOT have an artificial language-level maximum.

Qualified names MUST be resolved using the canonical name/module system.

---

62. Visibility

UDT declarations MUST respect the language's visibility model.

A private UDT MUST NOT become publicly usable merely because a generated or lowered representation happens to expose it.

Visibility is a source/semantic property.

---

63. Generic scope

Generic parameters are scoped to their declaration.

For:

type Pair<T, U>

"T" and "U" are valid only within the semantic scope defined by the declaration and its permitted members.

Nested declarations MUST define their own parameter scopes.

Shadowing rules MUST be explicit and deterministic.

---

64. Constraint environment

Type resolution operates in an environment containing potentially:

type bindings
generic bindings
associated-type bindings
value parameters
bounds
constraints
ownership state
effect environment
capability environment
resource environment
policy environment

The implementation MUST preserve these environments separately rather than collapsing all constraints into one untyped map.

---

65. Ownership integration

UDTs MUST integrate with the ownership model.

A UDT MAY contain:

owned values
borrowed references
shared references
linear resources
affine resources

Ownership rules apply recursively according to the semantic type structure.

A compiler MUST NOT infer that a UDT is freely copyable merely because its outer syntax is a record.

---

66. Linear types

A UDT MAY be linear.

A linear value MUST be consumed exactly according to the language's linearity rules.

The compiler MUST detect:

duplicate consumption
missing consumption
invalid duplication
invalid discard

where the language semantics prohibit them.

No global maximum on the number of linear fields is permitted.

---

67. Affine types

An affine UDT MAY be consumed at most once.

An affine value MAY be discarded when permitted.

Linear and affine semantics MUST remain distinct.

---

68. References and lifetimes

UDTs MAY contain references.

Reference validity MUST be checked by the ownership/lifetime subsystem.

A UDT containing a reference MUST NOT automatically become:

'static

or equivalent.

The semantic lifetime belongs to the referenced value and its type environment.

---

69. Recursive ownership

Recursive UDTs MUST be analyzed without infinite expansion.

The implementation SHOULD use symbolic graph-based representation for recursive semantic types.

For example:

Node
 └── Reference<Node>

is one recursive type graph.

It is not an infinitely duplicated syntax tree.

---

70. Equality

The implementation MUST distinguish:

type identity equality

from:

type compatibility

and:

value equality

and:

representation equality

Two UDTs can have compatible representations without having equal nominal identities.

---

71. Ordering and hashing

If semantic UDTs participate in maps, sets, caches, or canonical hashes, equality and hashing MUST be consistent.

Semantic hashing MUST NOT include:

- memory addresses;
- pointer identity;
- filesystem-dependent temporary paths;
- machine-specific addresses;
- nondeterministic iteration state.

---

72. Type normalization

The semantic resolver MAY normalize types.

Examples:

alias expansion
qualified-name canonicalization
generic substitution
constraint normalization
associated-type resolution

Normalization MUST preserve semantic identity.

Alias expansion MUST NOT accidentally turn a nominal type into its representation.

---

73. Type substitution

Generic substitution MUST be capture-safe.

Substitution MUST preserve:

- generic scope;
- associated types;
- value parameters;
- ownership;
- effects;
- capabilities;
- resources;
- provenance.

Substitution MUST be deterministic.

---

74. Infinite generic recursion

The compiler MUST detect semantic non-termination caused by pathological recursive type constraints.

This MUST be handled as a compiler/type-resolution resource or semantic diagnostic.

It MUST NOT be implemented as a fixed language-level type-depth limit.

For example, the implementation MUST NOT define:

MAX_TYPE_DEPTH = 128

as a universal semantic rule.

---

75. Type solver resource limits

An implementation MAY provide configurable solver limits such as:

maximum analysis budget
maximum compilation time
maximum memory budget
maximum diagnostic count

These are operational policies.

They MUST NOT change the definition of whether a type is semantically valid.

---

76. Type errors versus resource errors

The compiler MUST distinguish:

invalid type

from:

valid type but analysis budget exhausted

and:

valid type but target resources unavailable

and:

valid type but backend unsupported

This distinction is essential for scalable compilation.

---

77. Contracts on UDT declarations

A UDT MAY have declaration-level contracts.

Conceptually:

type T
    requires ...
    invariant ...

The contract subsystem determines how these contracts are enforced.

Contracts MAY constrain:

- construction;
- field relationships;
- allowed values;
- generic parameters;
- associated types;
- resource properties;
- domain invariants.

---

78. Policies

A UDT may be affected by policy.

For example:

policy
    forbid foreign representation

or:

policy
    require deterministic serialization

Policies MUST be explicit.

A type declaration MUST NOT silently acquire policy from the current machine.

---

79. Provenance of generated types

Generated UDTs MUST retain provenance when required.

For example:

source declaration
    ↓
macro expansion
    ↓
generated type

The semantic system MAY record:

generated_by
source
macro
dialect
version
transformation

This is especially important for reproducibility and diagnostics.

---

80. Deterministic type IDs

If the compiler assigns internal IDs to semantic UDTs, those IDs MUST NOT affect language semantics.

Where stable identifiers are required for:

- caching;
- serialization;
- provenance;
- diagnostics;
- reproducibility;

they SHOULD be derived from canonical semantic information rather than process-local memory addresses.

---

81. Serialization

UDT semantic information MAY be serialized.

Serialization MUST preserve enough information to reconstruct:

identity
generic structure
constraints
members
associated types
visibility
version
dialect
provenance

Serialization format versioning MUST be separate from language type identity.

---

82. Reproducibility

A reproducible compilation of a UDT MUST NOT depend on:

- current time;
- random identifiers;
- machine addresses;
- filesystem enumeration order;
- backend discovery order.

If generated identifiers are necessary, they MUST be deterministically derived where reproducibility requires it.

---

83. ABI boundaries

A semantic UDT MAY lower to an ABI representation.

That representation MAY differ by target.

For example:

User

could be lowered differently on:

CPU ABI
GPU ABI
FPGA interface
foreign ABI
distributed serialization format

provided the semantic contract is preserved.

ABI layout MUST NOT redefine source type identity.

---

84. Serialization versus ABI

These MUST remain distinct:

source type
ABI representation
serialization format
wire format
memory representation

A type being serializable does not mean its serialization is its in-memory representation.

---

85. Interoperability

A foreign type MAY be wrapped in a Zamani UDT.

Example conceptual structure:

Zamani wrapper
    ↓
foreign type
    ↓
ABI boundary

The wrapper MAY enforce:

- ownership;
- lifetime;
- capability;
- effect;
- validation;
- provenance.

---

86. Data formats

JSON, XML, SQL, and similar technologies MUST NOT create separate universal Zamani type systems.

Instead:

external format
    ↓
dialect/interoperability layer
    ↓
Zamani semantic type

A UDT may represent a structured external data model.

---

87. Pattern matching

UDTs MUST integrate with pattern matching where supported.

Patterns MAY match:

struct fields
enum variants
sum alternatives
tuple members
generic structures
references

Pattern matching MUST respect:

- visibility;
- ownership;
- linearity;
- affinity;
- exhaustiveness;
- type compatibility.

---

88. Exhaustiveness

For closed sum/enum types, pattern matching SHOULD be statically checked for exhaustiveness.

For open or extensible types, the semantic rules MAY require an explicit fallback.

The compiler MUST NOT assume that an extensible type is closed unless its declaration says so.

---

89. Generic pattern matching

Pattern matching against generic UDTs MUST preserve generic substitutions.

For example:

Option<T>

must remain semantically meaningful for any valid "T".

---

90. Methods

Methods associated with UDTs MUST use the same canonical function/type system as ordinary functions.

The type system MUST NOT create a separate method-type system.

Method signatures participate in:

generic resolution
ownership
effects
capabilities
resources
contracts
policies
provenance

---

91. Associated constants

Where supported, UDTs and traits MAY contain associated constants.

Their values MUST participate in the canonical compile-time/value system.

A target-specific representation MUST NOT change their semantic value.

---

92. Type-level computation

Advanced type-level computation MAY operate on UDTs.

It MUST remain:

deterministic
bounded by explicit compiler policy
semantically validated
provenance-aware
safe

It MUST NOT silently execute arbitrary target-dependent code during type checking.

---

93. Compile-time execution

If Zamani permits compile-time execution involving UDTs, the execution environment MUST be explicitly constrained.

It MUST declare relevant:

effects
capabilities
resources
sandbox
policy
determinism

Compile-time execution MUST NOT automatically gain arbitrary filesystem, network, native, or hardware access.

---

94. Reflection and security

Reflection over UDTs MAY expose:

type name
members
generic parameters
attributes
contracts
metadata

Access MUST respect visibility and security policy.

Sensitive implementation metadata MUST NOT become public merely because a type is reflectable.

---

95. Sandbox integration

When UDT construction or inspection is performed in a sandboxed environment, the type subsystem MUST honor sandbox policy.

The type system MUST NOT bypass:

forbid effect("network")
forbid effect("native")
forbid effect("foreign")

or equivalent policies.

---

96. Adaptive execution

A UDT MAY participate in adaptive execution.

For example, a semantic tensor type may be realized through:

CPU
GPU
FPGA
accelerator
distributed system

The adaptive execution subsystem chooses the realization.

The type itself does not perform target selection.

---

97. Simulation

A UDT MUST remain meaningful under simulation when the semantic domain supports simulation.

Examples:

QuantumState
HardwareSignal
DistributedNode
Tensor

may have simulator representations.

Simulation MUST preserve semantic meaning.

A simulator representation MUST NOT become the universal source-level representation.

---

98. Distributed UDTs

UDTs MAY represent distributed values.

For example:

Distributed<T>
Shard<T>
Replica<T>
Remote<T>

These remain ordinary semantic types.

The distributed subsystem determines:

placement
replication
routing
consistency
serialization
communication
fault recovery

The UDT declaration does not encode a fixed node count.

---

99. Networking UDTs

Network-related UDTs MAY represent:

Endpoint
Message
Packet
Stream
Protocol
Channel

Network effects and capabilities remain separate from type identity.

For example:

Endpoint

does not inherently grant:

capability("network.connect")

The operation using the endpoint determines the required capability/effect.

---

100. Hardware capability types

A capability type MAY express semantic availability.

For example:

Capability<"tensor.compute">

The exact capability vocabulary belongs to the capability subsystem.

A capability type MUST NOT directly bind to a vendor or physical device unless it is explicitly declared as target-specific.

---

101. Resource types

Resource types MAY represent logical resources.

Examples:

Resource<T>
QubitResource
MemoryResource
ComputeResource

A resource type does not automatically mean the resource has been allocated.

Allocation is a runtime/execution concern.

---

102. Resource ownership

If a UDT owns a resource, ownership MUST be explicit.

For example:

ResourceHandle<T>

may be affine or linear.

Dropping such a value MAY trigger a semantic release operation if the language defines one.

The type system MUST make such behavior explicit.

---

103. Capability versus resource

These MUST remain separate.

Capability

answers:

«What can the target do?»

Resource

answers:

«What computational quantity can be consumed or used?»

Requirement

answers:

«What does this program need?»

Constraint

answers:

«What realizations are forbidden or required?»

Preference

answers:

«Which acceptable realization is preferred?»

---

104. Target independence

A UDT MUST NOT require modification when a new target is added.

For example, adding:

new QPU
new GPU
new FPGA
new accelerator
new CPU
new cluster architecture

MUST NOT require changing the semantic definition of existing portable UDTs.

---

105. Target-specific UDTs

Target-specific UDTs MAY exist.

They MUST be explicitly identified as:

target-specific
dialect-specific
foreign
non-portable
conditionally portable

A target-specific UDT MUST NOT be represented as universally portable without an explicit portability contract.

---

106. Portability classes

A UDT SHOULD be classifiable as:

portable
conditionally portable
dialect-specific
target-specific
foreign

Portable

Meaning independent of a particular target.

Conditionally portable

Portable when declared capabilities/resources are available.

Dialect-specific

Requires a registered dialect.

Target-specific

Bound to a particular target family or target profile.

Foreign

Owned by an external ABI/system.

---

107. POCO-REAF guarantee

For a portable UDT:

source type identity

MUST remain stable across compatible targets.

A target MAY change:

representation
layout
storage
parallelization
vectorization
routing
scheduling
device mapping
serialization
error correction

but MUST NOT change the source-level semantic meaning.

---

108. Classical integration

Classical UDTs use the same universal type system.

They MUST integrate with:

functions
expressions
statements
memory
concurrency
effects
resources
contracts
policies
provenance

There MUST NOT be an incompatible classical-only type system.

---

109. Quantum integration

Quantum UDTs use the same universal type system while respecting quantum-specific semantics.

They eventually converge through:

quantum::ir

No UDT grammar may directly construct physical quantum backend instructions.

---

110. HDL integration

HDL UDTs use the universal type system.

HDL semantics may add:

signal properties
timing properties
clock relationships
hardware constraints
synthesis attributes
verification properties

These remain semantic annotations/constraints, not physical implementation instructions.

---

111. Hybrid integration

A hybrid UDT may combine:

classical values
quantum values
hardware values
AI/data values
distributed values

The hybrid semantic layer determines how those domains interact.

The type system remains unified.

---

112. AI integration

AI UDTs MAY express:

Model<Input, Output>
Tensor<T, Shape>
Dataset<T>
Distribution<T>
InferenceResult<T>
Evidence<T>
Decision<T>

They remain normal UDTs.

AI-specific operations integrate through the AI semantic subsystem.

---

113. Knowledge integration

Knowledge UDTs MAY express:

Fact
Claim
Evidence
Relation
KnowledgeGraph
Decision
Explanation

They can be consumed by reasoning and query operations.

No AI-only type system is permitted.

---

114. Learning integration

Learning UDTs MAY express:

Model
Dataset
TrainingState
Objective
Feedback
LearningResult

The learning subsystem determines learning semantics.

The type system only defines the types and their relationships.

---

115. Adaptation integration

Adaptation-related UDTs MAY express:

AdaptationPlan
Policy
Strategy
Feedback
Candidate
Decision

Adaptation MUST remain governed by:

authorization
policy
effects
capabilities
resources
contracts
provenance

---

116. Evidence and explainability

A UDT may represent evidence or explanation.

For example:

Evidence<T>
Explanation<T>
Decision<T>

The compiler MAY attach provenance explaining:

where a type originated
which transformation produced it
which dialect defined it
which specialization was selected

---

117. Error handling

Invalid UDT programs MUST produce structured diagnostics.

Diagnostics SHOULD identify:

diagnostic code
severity
source span
type involved
expected type
actual type
generic environment
relevant bound
constraint
suggested correction where appropriate
provenance

---

118. Required diagnostic categories

At minimum, implementations SHOULD distinguish:

unknown type
duplicate type declaration
invalid type alias
invalid generic parameter
wrong generic arity
unsatisfied bound
unsatisfied constraint
invalid associated type
recursive type error
ownership violation
linearity violation
affinity violation
invalid conversion
invalid field
unknown field
duplicate field
unknown variant
invalid variant payload
visibility violation
foreign-type violation
dialect violation
capability violation
resource infeasibility
effect violation
contract violation
policy violation

---

119. Diagnostics must not hide resource failures

For example:

type Tensor<T, N>

may be completely valid while a selected target lacks sufficient memory.

The compiler MUST NOT report:

invalid type

when the actual problem is:

target resource infeasibility

---

120. Grammar integration

The grammar architecture MUST remain:

grammar/Zamani.g4
        ↓
declaration dispatcher
        ↓
UDT declaration owner
        ↓
typeExpression
        ↓
domain-neutral AST

The UDT specification MUST NOT require a second root grammar.

---

121. "grammar/Zamani.g4"

"grammar/Zamani.g4" remains the composition root.

It MUST NOT duplicate concrete UDT syntax.

It should only dispatch/combine the canonical declaration and type grammar.

A UDT addition MUST therefore be integrated through the appropriate declaration/type grammar owner rather than by placing a second implementation in "Zamani.g4".

---

122. "grammar/declarations/declarations.g4"

This file owns declaration-family dispatch.

It already separates:

type declarations
type aliases
structs
records
enums
unions
traits
implementations

UDT work MUST extend the existing declaration architecture rather than introduce a parallel UDT declaration dispatcher.

Concrete syntax remains with the dedicated declaration grammar.

---

123. "grammar/types/types.g4"

This remains the canonical public type-expression orchestration boundary.

It MUST own:

typeExpression
typeCore
typePrefix
typePostfix

It MUST NOT become a giant implementation of every UDT constructor.

UDT references should enter through the existing named/generic/type-expression architecture.

---

124. "grammar/types/named.g4"

Named types MUST remain the mechanism for referring to declared UDTs.

A UDT such as:

User

is referenced as a named type.

A qualified UDT such as:

account.User

is resolved through the canonical type path/name system.

"named.g4" MUST NOT need to know which UDT declarations exist.

---

125. Generic grammar integration

Generic syntax remains owned by the generic grammar.

UDT declarations consume generic parameters.

UDT type references consume generic arguments.

The generic subsystem MUST NOT require a hard-coded list of UDT names.

---

126. Struct/record grammar integration

Struct/record grammar owns:

fields
field declarations
field attributes
field visibility
field field-level syntax

It MUST use the canonical "typeExpression" for field types.

It MUST NOT enumerate:

Integer
String
Tensor
Qubit
User

or any finite type inventory.

---

127. Enum grammar integration

Enum grammar owns:

variants
variant payload syntax
variant attributes

It MUST use canonical type expressions for payloads.

---

128. Sum/union grammar integration

Sum/union grammar owns the syntax for alternatives.

Each payload MUST use canonical type syntax.

No domain-specific type list is permitted.

---

129. AST integration files

The primary implementation boundary is:

src/frontend/ast/node/types/

The existing AST already distinguishes named and generic type structures.

UDT support MUST reuse these canonical structures.

Declaration nodes already include UDT-related categories such as:

Struct
Enum
Trait
Impl
TypeAlias
TypeDeclaration

No parallel:

UserDefinedTypeAst
UniversalTypeAst
DomainTypeAst

should be created merely to represent UDTs.

---

130. Legacy AST compatibility

The repository also contains an older/general AST boundary under:

src/ast/

Until that boundary is fully migrated, compatibility code MAY preserve existing UDT declarations.

However, the architecture MUST converge on the canonical frontend AST.

The same semantic declaration MUST NOT acquire two independently maintained AST meanings.

---

131. Semantic integration

Semantic analysis MUST perform:

declaration collection
name resolution
generic binding
bound checking
constraint checking
type construction
recursive type validation
ownership analysis
linearity analysis
affinity analysis
effect analysis
capability analysis
resource analysis
contract analysis
policy analysis
provenance attachment

The grammar itself MUST NOT perform these operations.

---

132. Canonical semantic type

After semantic validation, every valid UDT MUST become one canonical semantic type representation.

Conceptually:

SemanticType
├── Primitive
├── Named
├── Generic
├── Aggregate
├── Function
├── Reference
├── Linear
├── Affine
├── Dependent
├── Associated
├── Domain
├── Foreign
└── Extension

The exact Rust enum/struct organization is implementation-defined, but semantic ownership MUST remain singular.

---

133. Classical IR integration

A classical UDT that reaches compilation MUST lower into the canonical classical IR representation.

The UDT MUST NOT define its own competing universal IR.

The lowering may choose:

aggregate
scalar
reference
opaque handle
specialized representation
distributed representation

according to semantic requirements.

---

134. Quantum IR integration

Quantum UDTs that participate in quantum computation MUST cross the canonical:

quantum::ir

boundary.

The type specification MUST NOT introduce:

UDTQuantumIR
UserTypeQIR
UniversalQuantumTypeIR

as competing representations.

---

135. HDL/hardware IR integration

Hardware UDTs lower through the existing hardware/HDL semantic pipeline.

The type system defines source semantics.

Hardware lowering determines:

signals
ports
registers
storage
interfaces
timing
placement
synthesis

---

136. Distributed lowering

Distributed UDTs MAY lower to:

local value
remote value
serialized value
sharded value
replicated value
distributed handle

The semantic type remains stable.

---

137. AI/data lowering

AI/data UDTs MAY lower to:

CPU tensor
GPU tensor
FPGA tensor
accelerator tensor
distributed tensor
symbolic representation
lazy representation

provided semantics are preserved.

---

138. Optimization

Optimization MAY specialize UDT representations.

Examples:

field elimination
layout optimization
generic specialization
constant propagation
value numbering
fusion
vectorization
parallelization
quantum optimization
hardware mapping

Optimization MUST preserve type semantics.

---

139. Target lowering

Target lowering MAY change representation.

It MUST NOT change:

type identity
contract meaning
ownership meaning
effect meaning
capability requirements
resource semantics
observable program behavior

unless the language explicitly permits a target-dependent semantic choice.

---

140. Resource negotiation

UDTs may influence resource requirements.

Example:

Tensor<Float, Shape>

may imply memory requirements based on "Shape".

The compiler SHOULD derive resource requirements symbolically whenever possible.

The type system MUST NOT convert these requirements into hard-coded universal capacities.

---

141. Capability negotiation

A UDT MAY require a capability.

Example:

QuantumState<T>

may require a quantum capability depending on its use.

Capability negotiation happens downstream.

The UDT itself does not select a machine.

---

142. Policies and specialization

Policies MAY affect which representation is selected.

Examples:

prefer locality
prefer deterministic execution
forbid remote execution
require quantum measurement
allow simulation
forbid foreign calls

Policies MUST NOT silently change type semantics.

---

143. Provenance through lowering

When a UDT is transformed:

source type
    ↓
normalized type
    ↓
specialized type
    ↓
lowered representation

the compiler SHOULD preserve provenance relationships.

This enables:

diagnostics
debugging
reproducibility
explainability
auditing
verification

---

144. Versioning

UDT declarations participate in language and module versioning.

A change that alters semantic identity or compatibility MUST be versioned appropriately.

Changes to:

field meaning
variant meaning
generic parameter meaning
trait obligations
associated types
contracts
effects
capabilities
resource semantics

MAY be breaking changes.

---

145. Compatibility

The compatibility system MUST distinguish:

source compatibility
AST compatibility
semantic compatibility
IR compatibility
ABI compatibility
serialization compatibility
runtime compatibility

These are not interchangeable.

---

146. ABI stability

A UDT can remain source-compatible while its target ABI changes.

ABI changes MUST NOT automatically imply source-language type changes.

---

147. Serialization compatibility

A serialized UDT representation MAY evolve independently of the semantic type.

Schema versioning MUST be explicit.

---

148. Dialect version compatibility

Dialect-defined UDTs MUST include enough metadata to determine whether a consumer supports the dialect and its type semantics.

---

149. Security requirements

UDTs MUST NOT provide implicit access to:

filesystem
network
native execution
foreign execution
hardware
credentials
reflection
code generation
runtime mutation

Those capabilities belong to the appropriate effect/capability/policy systems.

---

150. No implicit dynamic escape

When type inference or type resolution fails, the compiler MUST NOT silently convert the type into an unrestricted dynamic type.

If Zamani provides a dynamic/opaque type, it MUST be explicit and governed by its own semantics.

---

151. Opaque types

Opaque types MAY be supported.

An opaque type exposes identity while hiding representation.

This is useful for:

abstraction boundaries
FFI
security
module encapsulation
hardware handles
quantum handles
resource handles

An opaque type MUST NOT accidentally reveal its representation through compiler internals.

---

152. Abstract types

Abstract UDTs MAY define semantic behavior without exposing concrete representation.

This is especially useful for:

resource abstractions
domain abstractions
hardware-independent APIs
distributed handles
quantum logical resources

---

153. Handles

A handle UDT represents an abstract reference to a resource or service.

A handle MUST NOT be interpreted as a raw machine pointer.

If a target uses pointers internally, that representation remains an implementation detail.

---

154. No pointer-width semantics

A Zamani semantic type MUST NOT acquire its meaning from:

32-bit host
64-bit host
128-bit host
future host width

Pointer width is a target property.

---

155. No register-width semantics

UDT semantics MUST NOT depend on a fixed universal register width.

A backend MAY select:

scalar register
vector register
memory representation
distributed representation

as appropriate.

---

156. No tensor-rank ceiling

Tensor UDTs MUST support symbolic and arbitrarily large rank subject to actual compiler/resource feasibility.

The language MUST NOT define a universal tensor-rank maximum.

---

157. No quantum-count ceiling

Quantum UDTs MUST support symbolic quantum cardinality.

For example:

QRegister<N>

is valid regardless of whether "N" is:

1
10
100
1000000

provided the source semantics permit it.

Whether a target can realize the value is a separate feasibility question.

---

158. No node-count ceiling

Distributed UDTs MUST NOT encode a universal maximum number of nodes.

---

159. No device-count ceiling

Hardware UDTs MUST NOT encode a universal maximum number of devices.

---

160. No memory ceiling

A type MAY imply memory requirements.

It MUST NOT encode a universal maximum memory capacity.

---

161. No generic-arity ceiling

The number of generic parameters is declaration-defined.

It MUST NOT be constrained by a universal arbitrary constant.

---

162. No field-count ceiling

The number of fields is declaration-defined.

Operational parser limits, if needed, MUST be explicit compiler policies.

---

163. No variant-count ceiling

The number of variants is declaration-defined.

---

164. No tuple-size ceiling

Tuple size is determined by source semantics.

---

165. No function-parameter ceiling

Function parameter count MUST be determined by source semantics.

---

166. No artificial type-depth rule

Nested types may be arbitrarily deep in language semantics.

An implementation MAY limit compiler resource usage through explicit policy.

---

167. Error recovery

Parser error recovery MUST NOT fabricate valid UDT declarations.

Recovered parse trees MUST remain distinguishable from validated semantic declarations.

---

168. Source spans

Every UDT declaration and relevant child element MUST retain source-location information.

At minimum:

declaration span
name span
generic parameter spans
bound spans
field spans
variant spans
type-expression spans
attribute spans

These spans feed diagnostics and provenance.

---

169. Documentation metadata

UDT declarations MAY carry documentation.

Documentation MUST remain metadata.

It MUST NOT alter type identity unless an explicit attribute system defines semantic behavior.

---

170. Attributes

UDT attributes MAY provide semantic metadata.

Attributes MUST declare whether they are:

semantic
optimization
diagnostic
documentation
interop
dialect
target
tooling

Target-specific attributes MUST NOT silently modify portable source semantics.

---

171. Attributes and portability

An attribute that affects target realization MUST be distinguishable from an attribute that defines language semantics.

For example:

prefer target capability

is not equivalent to:

require target identity

---

172. Constraints and preferences

UDT-related constraints MUST distinguish:

require
constrain
prefer
allow
forbid
hint

For example:

requires capability("tensor.compute")

is stronger than:

prefer capability("tensor.compute")

The type system consumes the resulting semantic distinction.

---

173. UDT declarations and policies

A policy MAY:

allow a type
forbid a type
require a capability
require a representation property
forbid a foreign representation
require deterministic behavior
require provenance

The policy subsystem owns enforcement.

---

174. Testing model

Every UDT feature MUST have tests at multiple layers.

Required categories:

lexical
parser
AST
structural
semantic
type
generic
ownership
linearity
affinity
effects
capabilities
resources
contracts
policies
provenance
IR
target
compatibility
determinism
scalability
negative
boundary
cross-domain

---

175. Positive tests

At minimum, test:

simple nominal UDT
type alias
struct
record
enum
sum type
union
generic UDT
nested generic UDT
recursive UDT
mutually recursive UDT
associated type
trait implementation
linear UDT
affine UDT
reference-containing UDT
dependent/value-parameterized UDT
quantum UDT
HDL UDT
hardware UDT
AI UDT
data UDT
distributed UDT
dialect UDT
foreign wrapper

---

176. Negative tests

At minimum, test:

duplicate type name
unknown type
unknown field
duplicate field
unknown variant
duplicate variant
invalid generic arity
invalid generic bound
unsatisfied bound
invalid associated type
recursive representation failure
invalid ownership
linear duplication
invalid affine use
invalid conversion
visibility violation
dialect incompatibility
foreign-type violation
capability violation
resource infeasibility
effect violation
contract violation
policy violation

---

177. Boundary tests

Test:

empty valid aggregate where allowed
single-field aggregate
large symbolic aggregate
deeply nested generic type
recursive type
mutually recursive type
symbolic dimensions
large generic parameter sets
large field sets
large variant sets
large constraint sets
large quantum cardinalities
large distributed requirements

No test may define an artificial language ceiling.

---

178. Scalability tests

The test suite MUST verify that UDT semantics remain correct as the number of:

types
fields
variants
generic parameters
generic arguments
constraints
modules
namespaces
recursive relationships
domain types
resource requirements
capability requirements

increases.

Tests MAY use increasingly large generated programs.

Failure caused by configured compiler resource budgets MUST be classified as an operational resource failure rather than a language semantic failure.

---

179. Cross-domain tests

At minimum, validate UDT composition across:

classical
quantum
hybrid
HDL
hardware
AI
data
distributed
networking
accelerators
embedded
HPC
simulation
interoperability

---

180. POCO-REAF integration tests

A portable UDT SHOULD be tested against multiple target profiles.

For example:

same source UDT
    ↓
CPU profile
GPU profile
FPGA profile
QPU profile
simulator profile
distributed profile

The semantic type identity MUST remain stable.

Only target realization may differ.

---

181. Quantum portability test

For:

QRegister<N>

test that:

N symbolic
N small
N larger

remain semantically representable.

A target with insufficient resources must produce a resource/capability diagnostic rather than a type-system failure.

---

182. Hardware portability test

For:

Bus<T, Width>

verify that "Width" remains semantic and does not silently become a fixed machine register width.

---

183. Tensor portability test

For:

Tensor<T, Shape>

verify that:

Shape

remains symbolic or semantic where required.

Do not introduce a universal rank ceiling.

---

184. Distributed portability test

For:

Distributed<T>

verify that node count is a deployment/resource concern rather than a type-system ceiling.

---

185. Determinism tests

Compile the same source repeatedly with:

same compiler version
same language version
same module set
same dialect set
same semantic configuration

and verify equivalent semantic type results.

---

186. Reproducibility tests

Where reproducible builds are enabled, verify that UDT-related:

semantic hashes
generated metadata
diagnostic ordering
serialized type representations

are deterministic.

---

187. Compatibility tests

The implementation MUST cross-check:

grammar/spec/user-defined-types.md
grammar/spec/type-system.md
grammar/specification/types.md
grammar/types/types.g4
grammar/types/named.g4
grammar/declarations/declarations.g4
src/frontend/ast/node/types/
src/frontend/ast/node/program/
semantic type resolver
canonical classical IR
quantum::ir

for every stabilized UDT feature.

---

188. Lexer integration tests

Only keywords actually required by UDT syntax should be reserved.

A user-defined type name such as:

Tensor
Robot
VisionModel
Payment
MyQuantumType

MUST remain an identifier unless the language explicitly reserves that spelling.

UDTs MUST NOT cause keyword explosion.

---

189. Grammar integration tests

Every UDT grammar production MUST use the canonical lexer vocabulary.

No UDT grammar may create a private lexical vocabulary.

---

190. AST integration tests

Every UDT declaration MUST map to exactly one canonical AST declaration representation.

Every UDT type reference MUST map to exactly one canonical type-expression representation.

---

191. Semantic integration tests

Every valid UDT MUST resolve to one canonical semantic type.

No valid UDT may remain represented only as an unvalidated syntax node after semantic analysis.

---

192. IR integration tests

UDTs that reach compilation MUST have a defined lowering strategy.

A UDT that cannot be lowered to a selected target MUST produce an explicit lowering/target diagnostic.

---

193. Quantum IR tests

Quantum UDTs MUST verify the path:

source
→ AST
→ semantic type
→ quantum semantic model
→ quantum::ir

No second quantum IR is permitted.

---

194. Hardware tests

Hardware UDTs MUST verify:

source type
→ semantic hardware intent
→ capability/resource analysis
→ hardware lowering

Physical device selection MUST remain downstream.

---

195. FFI tests

Foreign UDT wrappers MUST verify:

native type
→ foreign declaration
→ ABI boundary
→ effect/capability checks
→ safe semantic representation

---

196. Safe-Rust tests

CI MUST reject production compiler code containing prohibited unsafe Rust.

At minimum, the implementation SHOULD enforce:

#![forbid(unsafe_code)]

at appropriate crate/module boundaries.

---

197. File-level completion contract

This specification file is considered complete when it has established:

1. UDT definition.
2. UDT ownership.
3. UDT declaration categories.
4. UDT identity.
5. Alias semantics.
6. Generic semantics.
7. Bounds.
8. Constraints.
9. Associated types.
10. Nominal/structural distinction.
11. Recursive types.
12. Ownership.
13. Linear types.
14. Affine types.
15. Effects.
16. Capabilities.
17. Resources.
18. Contracts.
19. Policies.
20. Provenance.
21. Dialects.
22. FFI/ABI.
23. Classical integration.
24. Quantum integration.
25. HDL integration.
26. AI/data integration.
27. Distributed integration.
28. Portability.
29. POCO-REAF.
30. Scalability.
31. Diagnostics.
32. Testing.
33. Compatibility.
34. Determinism.
35. Safe-Rust requirements.

---

198. File integration contract

This file:

grammar/spec/user-defined-types.md

DEPENDS_ON

grammar/DESIGN.md
grammar/spec/type-system.md
grammar/specification/types.md
grammar/types/types.g4
grammar/types/named.g4
grammar/declarations/declarations.g4

DEFINES

UDT semantic contract
UDT identity
UDT portability
UDT scalability
UDT integration rules
UDT conformance requirements

CONSUMED_BY

semantic type system
compiler implementation
AST implementation
grammar implementation
validation
testing
IR lowering
documentation/conformance tooling

AST_OWNER

src/frontend/ast/node/types/
src/frontend/ast/node/program/

with legacy compatibility through:

src/ast/

until migration is complete.

SEMANTIC_OWNER

semantic/type-analysis subsystem

CLASSICAL_IR_OWNER

canonical classical IR

QUANTUM_IR_OWNER

quantum::ir

RESOURCE_OWNER

grammar/resources/
semantic resource subsystem

CAPABILITY_OWNER

grammar/resources/
capability semantic subsystem

EFFECT_OWNER

grammar/effects/
semantic effect subsystem

CONTRACT_OWNER

grammar/validation/
contract semantic subsystem

POLICY_OWNER

grammar/policies/
security/policy subsystem

PROVENANCE_OWNER

provenance semantic/compiler subsystem

DIALECT_OWNER

grammar/dialects/

---

199. Required implementation files

The following integration should be completed without creating competing ownership.

199.1 "grammar/types/types.g4"

Must remain the universal type-expression orchestrator.

Required:

typeExpression
typeCore
typePrefix
typePostfix

UDTs enter through named/generic/dependent/associated type mechanisms.

No UDT-specific universal branch is required merely because a user declared a new type.

---

199.2 "grammar/types/named.g4"

Must own unresolved named type references and qualified type paths.

It MUST remain open-ended.

It MUST NOT enumerate declared type names.

---

199.3 Generic grammar

Must own:

generic parameters
generic arguments
parameter kinds
generic application

UDT declarations consume this subsystem.

---

199.4 Struct/record grammar

Must own aggregate field syntax.

It MUST use:

typeExpression

for field types.

---

199.5 Enum/sum/union grammars

Must own variant syntax.

Payloads MUST use canonical type expressions.

---

199.6 "grammar/declarations/declarations.g4"

Must remain the declaration dispatcher.

No second UDT declaration dispatcher may be created.

---

199.7 "grammar/spec/type-system.md"

Must remain the general type-system architecture.

This document specializes it for UDTs.

Any duplicate or conflicting UDT rule found there MUST be reconciled.

---

199.8 "grammar/specification/types.md"

Must remain the normative general type specification.

This document supplies the UDT-specific normative detail.

Cross-references MUST remain consistent.

---

199.9 "src/frontend/ast/node/types/"

Must provide the canonical domain-neutral type AST.

UDT references MUST use canonical named/generic/etc. nodes.

UDT declarations MUST use canonical declaration nodes.

---

199.10 Semantic type resolver

Must resolve:

name
namespace
generic parameters
aliases
nominal identity
bounds
constraints
associated types
recursive types
ownership
effects
capabilities
resources
contracts
policies

---

199.11 Canonical classical IR

Must receive UDTs through semantic lowering.

No UDT-specific competing IR is allowed.

---

199.12 "quantum::ir"

Must remain the canonical quantum semantic boundary.

Quantum UDTs must reach it only after semantic validation.

---

200. Required source-level examples

The conformance suite SHOULD contain examples equivalent to the following.

Simple record

type User {
    id: Integer
    name: String
}

Nominal type

type UserId {
    value: Integer
}

Alias

type UserId = Integer

Enum

type Status {
    Ready
    Running
    Complete
    Failed
}

Sum type

type Value =
    | IntegerValue(Integer)
    | FloatValue(Float)
    | TextValue(String)

Generic type

type Pair<T, U> {
    first: T
    second: U
}

Recursive type

type Node<T> {
    value: T
    next: Reference<Node<T>>
}

Symbolic type

type Vector<T, N> {
    data: Array<T, N>
}

Quantum-oriented type

type Register<N> {
    qubits: QubitRegister<N>
}

The exact surface syntax must follow the currently canonical grammar implementation; examples in this specification express semantic intent and MUST NOT override the grammar authority.

---

201. Resource example

A type MAY participate in a program containing:

requires capability("tensor.compute")
requires memory >= required_memory

The UDT system consumes these requirements semantically.

It MUST NOT introduce a constant such as:

MAX_MEMORY

to decide whether the type itself is valid.

---

202. Quantum resource example

A program may require:

requires capability("quantum.measurement")
requires qubits >= required_qubits

A UDT involving a logical quantum resource remains valid independently of whether a particular target has sufficient physical resources.

---

203. Policy example

A program may constrain a UDT realization with policies such as:

prefer local
allow simulation
forbid foreign
require deterministic

The policy system determines realization constraints.

The UDT semantic identity remains unchanged.

---

204. Provenance example

A generated UDT may retain:

source
generated_by
dialect
version
derived_from
verified_by

without changing its nominal identity.

---

205. Type-system interaction with reasoning

A reasoning operation may accept:

Evidence<T>
Claim<T>
Knowledge<T>

The reasoning subsystem does not need a separate type system.

It consumes canonical Zamani types.

---

206. Type-system interaction with learning

A learning operation may accept:

Dataset<T>
Model<Input, Output>
TrainingState<Model>

Again, these are ordinary UDTs or library/dialect types.

Learning semantics remain separate from type identity.

---

207. Type-system interaction with adaptation

An adaptation plan may be represented as:

AdaptationPlan<T>

but execution remains subject to:

policy
authorization
effect
capability
resource
provenance
contract

A type does not itself authorize adaptation.

---

208. Type-system interaction with explainability

A decision type may contain:

decision
evidence
confidence
explanation
provenance

These are ordinary semantic fields/types.

No application-specific keyword is required.

---

209. Type-system interaction with deterministic execution

A UDT may carry metadata requiring deterministic treatment.

For example:

Deterministic<T>

may be a library or semantic abstraction.

The runtime/compiler must honor deterministic execution policies where required.

---

210. Type-system interaction with simulation

The same UDT should remain usable under:

real execution
simulation
testing
verification

when the semantic domain supports those modes.

The simulator MUST NOT define a separate source type.

---

211. Type-system interaction with adaptive execution

The same type may be realized differently based on target capabilities.

For example:

Tensor<T, Shape>

may use:

CPU
GPU
FPGA
accelerator
distributed execution

without source modification.

---

212. Production conformance matrix

Every UDT feature MUST be traceable as:

Layer| Required evidence
Specification| This document
General type architecture| "grammar/spec/type-system.md"
Normative type semantics| "grammar/specification/types.md"
Lexer| canonical lexer tests
Grammar| canonical parser tests
AST| canonical AST node
Structural validation| validation test
Name resolution| resolver test
Generic resolution| generic test
Semantic type| semantic model test
Effects| effect test where applicable
Capabilities| capability test where applicable
Resources| resource test where applicable
Contracts| contract test where applicable
Policies| policy test where applicable
Provenance| provenance test where applicable
Classical IR| lowering test where applicable
"quantum::ir"| quantum lowering test where applicable
HDL| HDL lowering test where applicable
Backend| target conformance test
Runtime| runtime test where applicable
Compatibility| version test
Determinism| repeated-build test
Scalability| generated large-input test
Negative behavior| diagnostic test

A feature MUST NOT be declared "STABLE" if a required layer is missing.

---

213. Conformance states

UDT features SHOULD use the repository's conformance terminology:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

Additional implementation evidence SHOULD record:

AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL

---

214. Hard-coding audit

Every UDT implementation change MUST be checked for accidental introduction of:

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
MAX_GENERIC_PARAMETERS
MAX_STRUCT_FIELDS
MAX_ENUM_VARIANTS
MAX_TYPE_DEPTH

The audit MUST include:

grammar
AST
semantic resolver
IR
tests
documentation
compiler configuration
runtime integration

---

215. Forbidden architectural shortcuts

The implementation MUST NOT:

1. create a second universal type-expression root;
2. create a second AST for UDTs;
3. create a second semantic type system;
4. create a second quantum type system;
5. make UDTs target-specific by default;
6. make all UDTs dynamically typed;
7. silently erase nominal identity;
8. silently perform lossy conversion;
9. make generic arity globally fixed;
10. make field count globally fixed;
11. make enum variant count globally fixed;
12. make type depth globally fixed;
13. make quantum cardinality globally fixed;
14. make tensor rank globally fixed;
15. make distributed node count globally fixed;
16. make memory capacity a type-system maximum;
17. put hardware discovery in grammar;
18. put QPU routing in grammar;
19. put scheduling in grammar;
20. put QEC in grammar;
21. put calibration in grammar;
22. put backend selection in grammar;
23. require unsafe Rust;
24. allow nondeterministic type resolution;
25. let dialects silently redefine core semantics.

---

216. Production implementation sequence

UDT implementation SHOULD proceed in dependency order.

Phase 1 — specification

Complete:

grammar/spec/user-defined-types.md

and reconcile:

grammar/spec/type-system.md
grammar/specification/types.md

Phase 2 — declaration ownership

Validate:

grammar/declarations/declarations.g4

and all concrete declaration grammars.

Phase 3 — canonical type expression

Validate:

grammar/types/types.g4
grammar/types/named.g4

and the generic/type constructor delegates.

Phase 4 — AST

Validate:

src/frontend/ast/node/types/
src/frontend/ast/node/program/

against the declaration/type contracts.

Phase 5 — semantic resolution

Implement:

name resolution
generic resolution
alias resolution
nominal identity
constraint solving
recursive type handling

Phase 6 — ownership

Integrate:

ownership
references
linear
affine
lifetime

Phase 7 — universal semantics

Integrate:

effects
capabilities
resources
contracts
policies
provenance

Phase 8 — domains

Integrate:

classical
quantum
HDL
hardware
AI
data
distributed
networking
hybrid

Phase 9 — IR

Validate:

canonical classical IR
quantum::ir
domain lowering

Phase 10 — targets

Validate:

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
future targets

Phase 11 — conformance

Run:

positive
negative
boundary
scalability
determinism
compatibility
cross-domain
POCO-REAF

---

217. Definition of production-ready UDT support

Zamani has production-ready UDT support only when a developer can:

declare a type
        ↓
reference it
        ↓
compose it
        ↓
genericize it
        ↓
constrain it
        ↓
implement behavior for it
        ↓
pattern-match it
        ↓
use it in classical computation
        ↓
use it in quantum computation
        ↓
use it in HDL/hardware computation
        ↓
use it in AI/data computation
        ↓
use it in distributed computation
        ↓
use it across dialects/interoperability
        ↓
validate it
        ↓
lower it
        ↓
realize it

without requiring a new universal type-system implementation for each domain.

---

218. Final canonical architecture

The complete UDT architecture is:

                         Zamani Source
                              │
                              ▼
                    Canonical Zamani Lexer
                              │
                              ▼
                       Zamani Grammar
                              │
                              ▼
                     Declaration Grammar
                              │
               ┌──────────────┴──────────────┐
               │                             │
               ▼                             ▼
        UDT Declaration                 Type Expression
               │                             │
               └──────────────┬──────────────┘
                              ▼
                    Domain-Neutral AST
                              │
                              ▼
                    Structural Validation
                              │
                              ▼
                       Name Resolution
                              │
                              ▼
                    Generic Resolution
                              │
                              ▼
                   Constraint / Bound Check
                              │
                              ▼
                   Ownership / Linearity
                              │
                              ▼
                       Effect Analysis
                              │
                              ▼
                    Capability Analysis
                              │
                              ▼
                     Resource Analysis
                              │
                              ▼
                      Contract Analysis
                              │
                              ▼
                       Policy Analysis
                              │
                              ▼
                    Provenance Attachment
                              │
                              ▼
                   Canonical Semantic Type
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
          ▼                   ▼                   ▼
      Classical            Quantum              HDL
      Semantic             Semantic            Semantic
       Types                Types               Types
          │                   │                   │
          ▼                   ▼                   ▼
  Classical IR          quantum::ir       Hardware/HDL IR
          │                   │                   │
          └───────────────────┼───────────────────┘
                              │
                              ▼
                        Optimization
                              │
                              ▼
                       Specialization
                              │
                              ▼
                    Resource Negotiation
                              │
                              ▼
                           Routing
                              │
                              ▼
                         Scheduling
                              │
                              ▼
                    Resilience / Recovery
                              │
                              ▼
                             ZQN
                              │
                              ▼
                             HAL
                              │
         ┌────────────────────┼────────────────────┐
         │                    │                    │
         ▼                    ▼                    ▼
        CPU                  GPU                  FPGA
         │                    │                    │
         ├────────────────────┼────────────────────┤
         │                    │                    │
         ▼                    ▼                    ▼
        ASIC              Accelerator             QPU
         │                    │                    │
         └────────────────────┼────────────────────┘
                              ▼
                  Simulator / HPC / Cluster
                              │
                              ▼
                    Distributed / Cloud
                              │
                              ▼
                     Future Targets

---

219. Fundamental UDT invariant

The fundamental invariant is:

USER-DEFINED TYPE
        =
SEMANTIC PROGRAM MEANING

and never:

USER-DEFINED TYPE
        =
PHYSICAL MACHINE REPRESENTATION

A UDT describes what the programmer means.

The compiler determines how that meaning can be represented and realized.

---

220. POCO-REAF type invariant

For a valid portable UDT "T":

SemanticType(T, target₁)
=
SemanticType(T, target₂)

provided both targets satisfy the program's semantic requirements.

The realizations may differ:

layout₁ != layout₂
representation₁ != representation₂
schedule₁ != schedule₂
device₁ != device₂

without implying:

type₁ != type₂

This is essential to POCO-REAF.

---

221. Scalability invariant

The UDT system MUST scale according to available resources rather than a hard-coded language universe.

Therefore:

small program
    ↓
same type system
    ↓
large program
    ↓
same type system
    ↓
distributed program
    ↓
same type system
    ↓
quantum/hybrid program
    ↓
same type system
    ↓
future computational domain
    ↓
same type system

A new computational scale MUST NOT require a new UDT language.

---

222. Final non-negotiable rules

Zamani UDTs MUST:

1. be first-class language types;
2. use the canonical type-expression system;
3. use the domain-neutral AST;
4. have deterministic semantic identity;
5. support nominal identity where declared;
6. distinguish aliases from nominal types;
7. support generic composition;
8. support bounds and constraints;
9. integrate with associated types;
10. support recursive semantic types;
11. integrate with ownership;
12. integrate with linearity;
13. integrate with affinity;
14. integrate with effects;
15. integrate with capabilities;
16. integrate with resources;
17. integrate with contracts;
18. integrate with policies;
19. integrate with provenance;
20. integrate with dialects;
21. integrate with FFI/ABI;
22. integrate with classical computation;
23. integrate with quantum computation;
24. integrate with HDL;
25. integrate with hardware/software co-design;
26. integrate with AI/data computation;
27. integrate with distributed computation;
28. preserve source-level meaning across target realization;
29. scale without artificial language-level capacity ceilings;
30. compile through the canonical semantic pipeline;
31. use "quantum::ir" for quantum semantic lowering;
32. preserve deterministic semantics;
33. support reproducibility where configured;
34. produce precise diagnostics;
35. be implementable in Rust 1.97.1 or later;
36. use safe Rust only.

They MUST NOT:

1. create a second type system;
2. create a second universal "TypeExpr";
3. create a second AST;
4. create a second quantum IR;
5. encode fixed machine capacities;
6. encode fixed quantum capacities;
7. encode fixed tensor rank;
8. encode fixed distributed scale;
9. encode fixed generic arity;
10. encode fixed aggregate size;
11. silently change semantic type identity by target;
12. silently introduce dynamic typing;
13. silently perform unsafe conversions;
14. perform hardware discovery in grammar;
15. perform physical quantum routing in grammar;
16. perform scheduling in grammar;
17. perform QEC in grammar;
18. perform backend selection in grammar;
19. require Rust "unsafe";
20. depend on nondeterministic type resolution.

---

223. Final production criterion

"grammar/spec/user-defined-types.md" is the authoritative UDT contract when every UDT feature can be traced without ambiguity through:

UDT Specification
       ↓
Declaration Grammar
       ↓
Type Grammar
       ↓
Canonical Lexer
       ↓
Domain-Neutral AST
       ↓
Structural Validation
       ↓
Name Resolution
       ↓
Generic / Constraint Resolution
       ↓
Ownership / Linear / Affine Validation
       ↓
Effects
       ↓
Capabilities
       ↓
Resources
       ↓
Contracts
       ↓
Policies
       ↓
Provenance
       ↓
Canonical Semantic Type
       ↓
Classical IR / quantum::ir / domain IR
       ↓
Optimization
       ↓
Specialization
       ↓
Resource Negotiation
       ↓
Routing
       ↓
Scheduling
       ↓
Resilience / QEC
       ↓
ZQN
       ↓
HAL
       ↓
Target

At no point may a downstream target redefine the meaning of the source UDT.

At no point may a new hardware platform require a new universal UDT branch merely because the platform exists.

At no point may a compiler implementation limit become a language semantic limit.

At no point may a user-defined type become tied to a particular machine merely because it was compiled for that machine.

The resulting model is:

                Zamani User-Defined Type
                          │
                          ▼
                  Semantic Meaning
                          │
          ┌───────────────┼────────────────┐
          │               │                │
          ▼               ▼                ▼
       Classical        Quantum           HDL
          │               │                │
          ▼               ▼                ▼
       Classical       quantum::ir       Hardware
           IR             │               IR
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                    Target-independent
                       optimization
                          │
                          ▼
                 resource/capability
                    negotiation
                          │
                          ▼
                 routing / scheduling
                          │
                          ▼
                   target realization

That is the required foundation for user-defined types in a production Zamani language capable of expressing computation from the smallest useful computational scale through arbitrarily larger feasible systems while preserving one coherent type system and the POCO-REAF architecture.