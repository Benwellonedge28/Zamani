Worked for 52s

I read the current repository material relevant to grammar/types/, including the existing type grammars, the canonical frontend TypeExpr, grammar/specification/types.md, grammar/README.md, the parser/lexer architecture, and the repository's Rust baseline.

There are several important corrections that the new README must make rather than perpetuate:

grammar/types/types.g4 and grammar/types/type.g4 currently create a potential authority split; types.g4 must remain the canonical composition boundary, while type.g4 must not become a second typeExpression authority.

The actual primitive file is primitive.g4, not primitives.g4.

TypeExpr::Generic currently stores Vec<TypeExpr>, so the documentation must not falsely claim that generic applications already have independent type/value/resource/capability argument AST variants.

TypeValueExpr currently represents identifiers, integer literals, generic symbolic values and extensions; therefore the README must not claim that arbitrary type-level arithmetic is already implemented.

TypeExpr already provides a substantial source-level vocabulary: Identifier, Generic, Tuple, Array, Slice, Function, Reference, Pointer, Optional, Result, Never, Unit, SelfType, Infer, GenericParameter, Union, Intersection, Associated, TypeApplication, Quantum, Linear, Affine, Temporal, Pi, Sigma, Identity, Hkt, and Extension.

TypeValidationPolicy provides configurable compiler-side safety/resource limits. These are not language ceilings and must never be confused with universal language restrictions.

The frontend AST is the real integration boundary; the grammar should not invent another type AST or IR.


Below is the complete replacement content I recommend for grammar/types/README.md.

Zamani Type System Grammar

Path: "grammar/types/README.md"
Repository: "Benwellonedge28/Zamani"
Language: Zamani
Grammar technology: ANTLR4
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Safety: safe Rust only; "unsafe" is prohibited
Primary architectural objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

"grammar/types/" is the source-language type-syntax subsystem of Zamani.

It defines how source programs express type information while remaining independent of:

- physical machine size;
- target architecture;
- vendor;
- device identity;
- runtime allocation;
- scheduling;
- routing;
- placement;
- calibration;
- physical quantum resources;
- backend representation;
- ABI layout;
- compiler implementation strategy.

The type subsystem must be capable of expressing the type-level abstractions required by:

- classical computation;
- systems programming;
- embedded systems;
- scientific computing;
- numerical computing;
- symbolic computation;
- AI and machine learning;
- neural-symbolic computation;
- probabilistic and uncertain computation;
- tensor computation;
- data processing;
- quantum computation;
- quantum-classical hybrid computation;
- HDL;
- hardware/software co-design;
- accelerators;
- parallel computation;
- concurrent computation;
- distributed computation;
- HPC;
- networking;
- cryptography;
- temporal computation;
- resource-aware computation;
- metaprogramming;
- interoperability;
- future computational domains.

The central question owned by this directory is:

«What type structure did the programmer express?»

The following questions are outside this directory:

«Can a particular target execute the type?»

«Which physical resource should be used?»

«Which QPU should execute it?»

«Which physical qubits should be allocated?»

«Which CPU, GPU, FPGA or accelerator should be selected?»

«How should resources be scheduled?»

«How should quantum operations be routed?»

«What physical representation should be generated?»

Those decisions belong to semantic analysis, resource/capability analysis, compilation, execution and backend layers.

---

2. Architectural position

The type subsystem participates in the repository pipeline:

Zamani source
     │
     ▼
canonical lexical system
     │
     ▼
canonical parser
     │
     ▼
typeExpression
     │
     ▼
frontend TypeExpr
     │
     ▼
structural AST validation
     │
     ▼
name resolution
     │
     ▼
generic/type-parameter resolution
     │
     ▼
type inference / unification / checking
     │
     ▼
semantic type model
     │
     ├───────────────┬────────────────┬─────────────────┐
     ▼               ▼                ▼                 ▼
 classical       quantum          HDL/hardware       AI/data
 semantics       semantics        semantics          semantics
     │               │                │                 │
     └───────────────┴────────────────┴─────────────────┘
                         │
                         ▼
                 canonical semantic model
                         │
              ┌──────────┴──────────┐
              ▼                     ▼
        classical IR           quantum::ir
              │                     │
              └──────────┬──────────┘
                         ▼
                 optimization
                         │
                         ▼
                  domain lowering
                         │
                         ▼
                resource negotiation
                         │
                         ▼
                     routing
                         │
                         ▼
                   scheduling
                         │
                         ▼
              resilience / recovery
                         │
                         ▼
                        ZQN
                         │
                         ▼
                        HAL
                         │
                         ▼
                 target realization

This document describes the type-language boundary, not an assertion that every downstream stage is already implemented.

Implementation status must be obtained from the repository's conformance documentation and tests.

---

3. Authority model

There must be exactly one owner for every language responsibility.

For the type subsystem:

Responsibility| Authority
Type syntax composition| "grammar/types/types.g4"
Canonical lexical vocabulary| "grammar/antlr/ZamaniLexer.g4" and canonical lexer/token specifications
Parser integration| "grammar/antlr/ZamaniParser.g4" and root grammar composition
Source-level type AST| "src/frontend/ast/node/types/type_expr.rs"
Type AST module organization| "src/frontend/ast/node/types/mod.rs"
Type specification| "grammar/specification/types.md"
Type conformance status| "grammar/grammar.md"
Type architecture navigation| this file
Type semantic resolution| semantic/type-analysis subsystem
Resource/capability feasibility| "grammar/resources/" plus semantic/runtime systems
Effects| "grammar/effects/" plus semantic effect analysis
Policies| policy subsystem
Quantum semantic representation| quantum semantic layer and "quantum::ir"
Hardware realization| hardware/compiler/backend subsystems

No type grammar file may silently become a second authority for another responsibility.

---

4. Canonical type-expression owner

4.1 "grammar/types/types.g4"

"grammar/types/types.g4" is the canonical type-composition boundary.

It owns the public source-level type-expression composition rule:

typeExpression

It may compose specialized type grammar components.

It must not be duplicated by another type grammar.

The canonical architecture is:

types.g4
    │
    ├── primitive
    ├── named
    ├── generic
    ├── tuple
    ├── array
    ├── slice
    ├── function
    ├── reference
    ├── pointer
    ├── option
    ├── result
    ├── algebraic
    ├── associated
    ├── dependent
    ├── linear
    ├── affine
    ├── temporal
    ├── quantum
    ├── classical
    ├── hardware
    ├── resource
    ├── capability
    └── extensibility

Specialized grammars provide reusable productions.

They do not create alternative public type-expression roots.

---

5. "grammar/types/type.g4" compatibility rule

The repository currently contains both:

grammar/types/types.g4
grammar/types/type.g4

This must not become two competing type authorities.

The production rule is:

types.g4
    │
    ▼
canonical typeExpression

"type.g4" must either:

1. become a compatibility/delegation grammar; or
2. remain transitional until its functionality is folded into the canonical composition architecture.

It must not introduce an independently authoritative "typeExpression".

No new type grammar may import the two composition grammars in a circular manner.

Forbidden architecture:

types.g4
    ↓
type.g4
    ↓
types.g4

Also forbidden:

types.g4 → typeExpression
type.g4  → typeExpression

where both rules have independent implementations.

---

6. Existing specialized grammar naming

The repository currently contains type grammar components including, among others:

grammar/types/
├── README.md
├── types.g4
├── type.g4
├── primitive.g4
├── named.g4
├── generic.g4
├── tuple.g4
├── array.g4
├── slice.g4
├── map.g4
├── map-types.g4
├── option.g4
├── optional.g4
├── result.g4
├── pointer.g4
├── reference.g4
├── never.g4
├── sum.g4
├── union.g4
├── algebraic-types.g4
├── dependent.g4
├── associated.g4
├── type-class.g4
├── constraints.g4
├── type-constraints.g4
├── linear.g4
├── affine.g4
├── temporal.g4
├── resource.g4
├── ...

The existence of multiple similarly named files is an architectural risk.

Before creating or promoting any file:

1. Search for an existing implementation.
2. Determine whether it defines syntax already owned elsewhere.
3. Determine its consumers.
4. Determine its AST mapping.
5. Determine its specification owner.
6. Determine its conformance status.
7. Either integrate it, convert it to a delegate, or explicitly mark it compatibility/historical/deprecated.

Do not create a second implementation merely because a better filename seems desirable.

Existing filenames should not be renamed merely for cosmetic consistency.

---

7. Canonical frontend AST

The canonical source-level type representation is:

src/frontend/ast/node/types/type_expr.rs

This file is the definitive source-level AST boundary.

The grammar must map source syntax into "TypeExpr".

It must not introduce:

GrammarType
UniversalType
QuantumTypeExpr
HardwareTypeExpr
AiTypeExpr
TypeGrammarNode
QuantumTypeIR
HardwareTypeIR

as competing source representations.

---

8. Current "TypeExpr" vocabulary

The current frontend type representation contains source-level forms including:

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

This existing vocabulary must be treated as the starting architectural contract.

A grammar feature is not complete merely because its corresponding conceptual name appears in this README.

It is complete only when:

specification
    ↓
lexer
    ↓
grammar
    ↓
AST
    ↓
structural validation
    ↓
semantic analysis
    ↓
IR
    ↓
required tests

is implemented as required by the feature.

---

9. TypeExpr responsibility

"TypeExpr" represents source-level structure.

It must not resolve:

- symbols;
- aliases;
- generic substitutions;
- trait/type-class satisfaction;
- overloads;
- capabilities;
- resources;
- physical hardware;
- physical quantum allocation;
- runtime representation;
- ABI layout;
- target selection.

The distinction is:

TypeExpr
    =
source-level type structure

SemanticType
    =
resolved type meaning

Canonical IR
    =
target-independent computational representation

Domain IR
    =
domain-specific implementation representation

Target representation
    =
backend-specific realization

These layers must remain distinct.

---

10. Primitive types

Primitive syntax belongs to the primitive type grammar component:

grammar/types/primitive.g4

not a newly invented "primitives.g4" authority unless the repository deliberately introduces such a file and migrates ownership.

Primitive syntax may cover language-defined scalar concepts such as:

int
float
bool
char
string
str
void
never

The exact stable set is determined by the language specification and implementation.

---

11. Semantic integer versus representation width

A semantic type:

int

must not automatically mean:

i32

or:

i64

unless the normative language specification explicitly says so.

A representation-specific type such as:

i64

has a different semantic purpose from an abstract:

int

The type grammar must preserve this distinction.

No backend implementation choice may silently redefine source semantics.

---

12. Fixed-width numeric types

If fixed-width numeric types are standardized, their meaning must be explicitly specified.

Possible examples include:

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

f16
f32
f64

These are examples of possible language-level representation types, not a statement that every spelling is currently implemented.

They must not be introduced solely because one target backend uses a particular width.

---

13. Named types

Named types are source-level names.

They must support qualified names through the canonical name/path architecture.

Conceptually:

Type
Module::Type
domain::Type
project::module::Type

The type grammar recognizes structure.

Name resolution belongs downstream.

The grammar must not resolve a source name to a compiler symbol ID.

---

14. Generic type applications

Generic applications are owned by:

grammar/types/generic.g4

under the canonical type composition grammar.

Examples:

Vec<Int>
Map<String, Int>
Result<Value, Error>
Option<T>
Tensor<Value>
Quantum<State>
Register<Qubit>

Generic application is domain-neutral.

The same syntax may be used by:

- classical libraries;
- AI models;
- tensor systems;
- data structures;
- quantum abstractions;
- HDL abstractions;
- hardware abstractions;
- distributed abstractions;
- future domains.

The generic grammar must not know what an individual generic constructor means.

---

15. Important generic-argument constraint

The current frontend "TypeExpr::Generic" representation is:

Generic {
    base: Box<TypeExpr>,
    arguments: Vec<TypeExpr>
}

Therefore the current source AST represents generic arguments as "TypeExpr".

The grammar documentation must not claim that the implementation already has separate AST categories for:

type argument
value argument
resource argument
capability argument

That distinction may be required in the future, but it requires an explicit AST/semantic design.

Until that work exists, the correct boundary is:

generic syntax
    ↓
TypeExpr arguments
    ↓
semantic interpretation

If future Zamani syntax requires generic arguments that cannot be represented faithfully as "TypeExpr", the AST contract must be deliberately extended before claiming the feature is implemented.

The extension must preserve compatibility and source meaning.

---

16. Generic arity

Generic lists must use repetition rather than finite grammar alternatives.

Correct architectural form:

genericArgumentList
    : typeExpression (COMMA typeExpression)* COMMA?
    ;

The language must not define an arbitrary universal generic arity.

Forbidden:

Generic<T>
Generic<T, U>
Generic<T, U, V>
...

with a finite endpoint.

Actual compiler resource consumption may be controlled by an explicit compiler policy, but that is not a language semantic ceiling.

---

17. Generic nesting

Nested generic applications must be supported structurally:

Vec<Option<T>>
Map<K, Vec<Result<V, E>>>
Tensor<Matrix<Value>>

No universal maximum generic nesting depth belongs in the language grammar.

Compiler-side protection against malicious or pathological input may exist as an explicit configurable validation policy.

Such protection must not alter the language's conceptual type system.

---

18. Generic parameters

Generic declarations belong to the declaration/function/type-parameter subsystem.

The type grammar consumes generic parameter references.

Examples:

T
U
Element
State
Shape

Constraints and bounds may be represented through the existing constraint/type-class architecture.

The type grammar does not decide whether:

T: Numeric
T: QuantumState
T: Sendable

is semantically satisfied.

---

19. Tuple types

Tuple syntax must support:

()
(T,)
(T, U)
(T, U, V)

and arbitrary source cardinality subject only to implementation resource availability.

The grammar must not define a finite tuple-size ceiling.

The empty tuple is the unit type where the language specification establishes that relationship.

The frontend representation is:

TypeExpr::Unit
TypeExpr::Tuple(...)

as appropriate.

---

20. Arrays

The current frontend AST explicitly supports:

TypeExpr::Array {
    element,
    length
}

The length is represented as:

Option<TypeValueExpr>

This is important for POCO-REAF.

A source program may express symbolic cardinality without forcing the parser to convert it into a host-machine integer.

Conceptually:

[T; N]

may preserve:

N

as source-level type information.

---

21. TypeValueExpr

The current frontend provides a dedicated source-level:

TypeValueExpr

with forms including:

Identifier(TypePath)
Integer(...)
Generic(TypeParameterName)
Extension(...)

This representation is specifically useful for symbolic values such as array cardinalities.

It must remain target-neutral.

The parser must not silently convert symbolic values into:

usize
u32
u64

or any target-dependent representation merely because the compiler implementation happens to run on such a machine.

---

22. Type-level arithmetic status

The repository's current "TypeValueExpr" does not establish a complete arbitrary arithmetic expression language.

Therefore this README must not claim that arbitrary expressions such as:

N + 1
Rows * Columns
2 * N
Size / Block
N << 1

are already implemented in type syntax.

If type-level arithmetic becomes a stable language feature, it must receive a complete design covering:

syntax
AST
evaluation
symbolic representation
overflow semantics
normalization
dependency tracking
diagnostics
determinism
semantic checking
serialization
IR integration
tests

Until then, symbolic type values should remain within the actually implemented "TypeValueExpr" vocabulary.

---

23. Slice types

Slice syntax is represented by:

TypeExpr::Slice(...)

A slice must not imply:

- fixed pointer width;
- fixed address width;
- fixed allocation size;
- fixed memory size;
- fixed hardware architecture.

Those are implementation concerns.

---

24. Function types

Function type syntax is represented by:

TypeExpr::Function {
    parameters,
    return_type
}

Function parameter collections are ordered and must not have an artificial language-level maximum.

Examples:

fn() -> T
fn(T) -> U
fn(T, U) -> V

Calling convention, ABI, register assignment and machine representation remain downstream.

---

25. Async and effectful functions

Function type syntax must not duplicate the effect subsystem.

Where Zamani supports effect-qualified or asynchronous function semantics, the architecture is:

function type
    +
effect information
    +
capability requirements
    +
resource requirements

Effects remain owned by:

grammar/effects/

The type grammar consumes the appropriate representation but does not redefine the effect system.

---

26. Reference types

The current AST provides:

TypeExpr::Reference {
    mutable,
    lifetime,
    inner
}

The grammar must support the source syntax standardized by the language, including lifetime and mutability information where implemented.

Examples conceptually include:

&T
&mut T
&'a T
&'a mut T

Lifetime checking is semantic.

Reference syntax must not perform borrow checking.

---

27. Pointer types

The current AST provides:

TypeExpr::Pointer {
    mutable,
    inner
}

Pointer syntax is source-level.

The grammar must not encode:

- pointer width;
- address-space width;
- physical address range;
- memory map;
- ABI layout.

Those belong downstream.

---

28. Optional types

The frontend provides:

TypeExpr::Optional(...)

If both postfix optional syntax and explicit generic syntax are standardized, they must have a single semantic optional meaning.

The grammar must not accidentally create two different optional type systems.

For example, if both are eventually accepted:

T?
Option<T>

they must converge semantically according to the specification.

---

29. Result types

The frontend provides:

TypeExpr::Result {
    ok,
    error
}

The grammar may represent:

Result<T, E>

where supported.

The type grammar does not own:

- error propagation;
- recovery;
- exception handling;
- runtime representation;
- ABI;
- backend error handling.

Those are downstream concerns.

---

30. Never and unit

The canonical source-level concepts are:

TypeExpr::Never
TypeExpr::Unit

The grammar must map their standardized source syntax to those concepts.

"Never" represents a type with no normal value.

"Unit" represents the language's unit value/type where specified.

---

31. "Self" type

The frontend supports:

TypeExpr::SelfType

The grammar may recognize the language's canonical "Self" syntax.

Whether "Self" is legal in a particular location is a semantic/contextual question.

The grammar must not attempt to implement trait/type-class context checking.

---

32. Inference placeholder

The frontend supports:

TypeExpr::Infer

This represents source-level inference syntax such as:

_

where supported.

The grammar recognizes structure.

Semantic analysis determines whether inference is permitted and whether the placeholder can be resolved.

An unresolved inference placeholder must produce a semantic diagnostic where required.

---

33. Generic parameter references

The frontend supports:

TypeExpr::GenericParameter(...)

A generic parameter reference must remain source-level until declaration and scope resolution.

The grammar does not decide whether the parameter exists.

Semantic analysis owns:

- scope;
- binding;
- shadowing;
- duplicate parameter detection;
- bounds;
- substitution.

---

34. Union types

The frontend supports:

TypeExpr::Union(...)

Union syntax is source-level.

Semantic analysis owns:

- canonicalization;
- duplicate elimination where specified;
- subtype relationships;
- exhaustiveness interactions;
- normalization.

The grammar must not hard-code a closed universe of union members.

---

35. Intersection types

The frontend supports:

TypeExpr::Intersection(...)

Intersection semantics remain downstream.

The grammar must preserve source structure without deciding:

- trait satisfaction;
- subtype relationships;
- normalization;
- implementation compatibility.

---

36. Associated types

The frontend supports:

TypeExpr::Associated {
    base,
    member
}

Conceptually:

Iterator::Item

The grammar recognizes the source structure.

Resolution belongs to the type semantic subsystem.

The grammar must not resolve an associated type to a concrete declaration.

---

37. Type application

The frontend provides:

TypeExpr::TypeApplication

This is distinct from ordinary generic application.

It provides a foundation for higher-order type-level abstractions where standardized.

The grammar must not introduce a second type-level language.

Type-level application must eventually converge on the same semantic type-resolution infrastructure.

---

38. Higher-kinded compatibility

The frontend also contains:

TypeExpr::Hkt

This existing representation must be treated as compatibility/source vocabulary, not as permission to create an unrelated higher-kinded type subsystem.

If higher-kinded types become fully normative, the repository must define:

kind syntax
kind checking
constructor application
higher-kinded parameters
constraints
substitution
inference
serialization
diagnostics
IR interaction

before declaring the feature complete.

---

39. Dependent types

The frontend provides:

TypeExpr::Pi
TypeExpr::Sigma

These are source-level dependent type structures.

They must not be confused with runtime allocation.

The grammar may represent dependent type syntax.

Semantic analysis owns:

- dependent binding;
- substitution;
- normalization;
- definitional equality;
- type checking;
- decidability policy;
- diagnostics.

No hardware capacity should be embedded in dependent types.

---

40. Identity types

The frontend provides:

TypeExpr::Identity

Identity/equality propositions remain semantic constructs.

The grammar represents the source relationship.

The type checker determines whether the proposition is valid under Zamani's type rules.

The grammar must not attempt theorem proving.

---

41. Algebraic types

Existing grammar components include algebraic, sum and union-related files.

Their responsibilities must remain separated:

declaration grammar
    ↓
declaration structure

type-expression grammar
    ↓
type reference/application

semantic type system
    ↓
algebraic meaning

Do not duplicate "struct", "enum", record or variant declaration syntax inside the type-expression grammar.

---

42. Type constraints and bounds

Existing components include:

grammar/types/constraints.g4
grammar/types/type-constraints.g4

These files must not become competing constraint authorities.

The canonical constraint architecture must establish one owner for:

- type bounds;
- generic bounds;
- type relationships;
- equality constraints;
- capability constraints where type-level;
- refinement relationships.

Constraints must integrate with:

grammar/validation/
grammar/resources/
grammar/effects/
grammar/policies/

where appropriate.

---

43. Type classes and interfaces

The repository contains:

grammar/types/type-class.g4

This syntax must remain distinct from semantic type-class/trait resolution.

The grammar recognizes source structure.

Semantic analysis determines:

- whether a type satisfies a type class;
- implementation selection;
- associated types;
- associated constants;
- method availability;
- coherence rules;
- ambiguity.

The type grammar must not contain a closed list of all future type classes.

---

44. Linear types

The frontend supports:

TypeExpr::Linear(...)

Linear type syntax represents source-level resource/usage semantics.

It must integrate with:

grammar/resources/
grammar/memory/
grammar/effects/

Semantic analysis owns actual linearity checking.

The grammar does not allocate or track physical resources.

---

45. Affine types

The frontend supports:

TypeExpr::Affine(...)

Affine semantics must remain separate from linear semantics while using the common type architecture.

The grammar must not duplicate the ownership checker.

---

46. Temporal types

The frontend supports:

TypeExpr::Temporal(...)

Temporal type syntax must remain source-level.

It must not embed:

- wall-clock assumptions;
- processor frequency;
- physical clock count;
- device timing;
- scheduler policy.

Those belong to temporal semantics and execution.

---

47. Quantum types

The quantum type system must remain open and extensible.

The frontend currently provides:

TypeExpr::Quantum(...)

This is a source-level quantum abstraction.

It does not mean:

- a physical qubit;
- a particular QPU;
- a vendor-specific qubit;
- a fixed number of qubits;
- a physical topology;
- a calibrated device;
- a pulse;
- a routing assignment.

The correct architecture is:

quantum source type
       │
       ▼
TypeExpr
       │
       ▼
semantic quantum type
       │
       ▼
quantum::ir
       │
       ▼
optimization
       │
       ▼
decomposition
       │
       ▼
routing
       │
       ▼
scheduling
       │
       ▼
resilience / QEC
       │
       ▼
ZQN
       │
       ▼
HAL

"quantum::ir" remains the canonical quantum IR boundary.

---

48. Quantum cardinality

No universal finite quantum cardinality may be encoded in the type grammar.

Do not create language-level ceilings such as:

Qubit32
Qubit64
Qubit128

as substitutes for scalable type semantics.

A symbolic form such as:

QRegister<N>

must remain source-level information when supported by the generic/type architecture.

The grammar must not define:

N <= 32

or any other universal hardware limit.

---

49. Logical and physical quantum abstractions

If Zamani distinguishes:

LogicalQubit
PhysicalQubit

they must remain semantically distinct.

The type grammar must not attach physical identifiers such as:

QPU0
QPU1
qubit0
qubit1

to the universal type system.

Physical mapping belongs to later compilation and execution.

---

50. Quantum learning and intelligent computation

The type system must not create a separate type system for machine learning.

Conceptual source types may include library or domain-defined forms such as:

Model<Input, Output>
Dataset<T>
Distribution<T>
Evidence<T>
Knowledge<T>
QuantumState<T>

provided their declarations exist.

The type grammar only represents the type structure.

Learning, reasoning, uncertainty, knowledge and adaptation are semantic/operation-level concerns.

They must reuse the common type system.

---

51. Uncertainty

Uncertainty may be expressed through normal generic or named types.

Conceptual examples:

Uncertain<T>
Probability<T>
Distribution<T>
Confidence<T>
Belief<T>

These names must not become a closed primitive keyword inventory unless explicitly standardized.

The type grammar should allow extensible domain types rather than enumerating every future probabilistic abstraction.

Probabilistic semantics belong downstream.

---

52. Evidence and provenance

Evidence and provenance should use normal source types where appropriate.

Examples:

Evidence<T>
Claim<T>
Provenance<T>
Decision<T>

These are not inherently special type syntax.

The type subsystem must remain usable by:

- AI;
- scientific computing;
- compiler transformations;
- security;
- quantum compilation;
- resource negotiation;
- hardware placement;
- data systems.

---

53. Resource types

The frontend supports source-level extensibility and existing resource-oriented type grammar components.

A resource type can express an abstract resource relationship.

Examples conceptually include:

Resource<T>
Resource<Qubit>
Resource<Memory>
Resource<Compute>

The type grammar must not allocate resources.

Resource feasibility belongs to:

grammar/resources/

and downstream semantic/resource systems.

---

54. Capability types

Capability-oriented type abstractions may be represented through ordinary named or generic types.

Examples:

Capability<T>
Capability<QuantumMeasurement>
Capability<TensorCompute>

The type grammar does not determine whether a target actually possesses a capability.

That belongs to capability resolution.

---

55. Hardware types

Hardware-related source types must describe abstractions, not physical machines.

Conceptual examples:

Accelerator<T>
Compute<T>
Memory<T>
Interconnect<T>
Signal<T>

Names such as:

CPU
GPU
FPGA
ASIC
QPU

should only be reserved type vocabulary when the language specification genuinely requires them.

The type grammar must not encode:

GPU0
GPU1
QPU0
CPU0
node0

as special universal type forms.

---

56. HDL integration

HDL types must use the same type foundation.

The HDL subsystem may express abstractions involving:

- signals;
- values;
- interfaces;
- modules;
- timing;
- storage;
- hardware intent;
- resource relationships.

The type grammar must not hard-code a universal hardware width.

Forbidden universal type assumptions include:

wire [31:0]
register = 32-bit
MAX_REGISTER_WIDTH

unless such syntax is explicitly part of an HDL dialect and its width is explicitly supplied by the program.

The distinction is:

source-declared width
    ≠
universal language maximum

---

57. Tensor types

Tensor abstractions should be represented through normal type construction.

Conceptual forms include:

Tensor<T>
Tensor<T, Shape>
Tensor<T, N>

where the corresponding constructors and semantic forms exist.

The grammar must not impose:

MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION

or equivalent finite ceilings.

Tensor rank and dimensions are source/semantic information.

Backend layout belongs downstream.

---

58. Data structures

The common type system must support reusable data abstractions such as:

Array
Slice
Map
Tuple
Record
Option
Result
Graph
Dataset
Stream

where standardized by the language/library architecture.

Application-specific data models must remain library-defined rather than becoming universal keywords.

---

59. Pattern matching

Pattern matching is primarily owned by:

grammar/expressions/
grammar/statements/

The type subsystem supplies type information to pattern checking.

It does not create a second pattern language.

Type semantics must support:

- pattern compatibility;
- exhaustiveness analysis;
- binding types;
- generic patterns;
- algebraic data;
- guards where supported.

---

60. Contract integration

Types interact with:

requires
ensures
invariant
assume
guarantee
property

but the type grammar does not own those constructs.

The relationship is:

type syntax
    ↓
TypeExpr
    ↓
semantic type
    ↓
contract checking

Type checking may be required to validate contract expressions.

Contract syntax and semantics remain owned by the validation subsystem.

---

61. Policy integration

Types can occur in policy-governed constructs.

Examples include policies controlling:

- resource usage;
- execution;
- security;
- adaptation;
- deployment;
- interoperability;
- simulation.

The type grammar does not authorize or deny anything.

Policy evaluation remains downstream.

---

62. Effect integration

Type syntax itself must not fabricate runtime effects.

For example:

Network<Packet>

does not perform network I/O.

Likewise:

Qubit

does not perform measurement.

Effects arise from executable operations and constructs.

The effect system owns effect semantics.

---

63. Capability integration

Type information may participate in capability analysis.

The direction is:

source type
    ↓
TypeExpr
    ↓
semantic type
    ↓
capability requirements
    ↓
capability resolution

The type grammar does not inspect the machine to decide whether a capability exists.

---

64. Resource integration

Type-level resource relationships may be consumed by the resource subsystem.

The direction is:

type
    ↓
semantic type
    ↓
resource requirements
    ↓
resource negotiation
    ↓
execution plan

A type must never directly allocate a CPU, GPU, FPGA, QPU, node, memory region or device.

---

65. POCO-REAF contract

A Zamani type must retain its source meaning across target environments.

For example, a source abstraction such as:

Tensor<T, Shape>

must remain the same source-level type when considered for:

embedded CPU
multicore CPU
GPU
FPGA
ASIC
accelerator
HPC system
cluster
distributed system
cloud
QPU
simulator
future target

The compiler may choose different representations.

The type itself does not change merely because the target changes.

---

66. No machine-size ceilings

The type grammar must not contain universal limits for:

qubits
CPUs
cores
threads
GPUs
FPGAs
QPUs
nodes
memory
storage
registers
tensor rank
tensor dimensions
tuple arity
generic arity
function parameters
devices
agents
network size

The following names and equivalent hidden restrictions are prohibited:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_TENSOR_RANK
MAX_REGISTER_WIDTH
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

The prohibition applies equally to disguised grammar forms.

---

67. Repetition instead of enumeration

For arbitrary collections, use parser repetition.

Preferred:

items
    : item (COMMA item)* COMMA?
    ;

Forbidden:

items
    : item
    | item COMMA item
    | item COMMA item COMMA item
    ;

The first form preserves scalability.

---

68. What "infinite scalability" means

POCO-REAF does not claim that physical machines have infinite resources.

It means that the language does not establish an arbitrary finite machine-capacity ceiling.

A program remains expressible subject to:

- available resources;
- compiler resources;
- runtime resources;
- operating-system constraints;
- target capabilities;
- provider limits;
- explicit user policy;
- security policy;
- physical feasibility.

These constraints are environmental or implementation-level.

They must not silently become language semantics.

---

69. Compiler validation limits

The repository's frontend "TypeValidationPolicy" may provide configurable safety limits.

These are legitimate because they are:

- explicit;
- configurable;
- implementation-level;
- diagnosable;
- separate from source-language meaning.

Examples include configurable limits on:

identifier size
collection size
extension namespace size
extension name size
attribute value size

These are not equivalent to:

MAX_TYPE_DEPTH
MAX_GENERIC_ARITY
MAX_QUBITS

as universal language rules.

A compiler may reject pathological input because of an explicit validation policy while the language itself remains conceptually unbounded.

---

70. Type validation policy

The existing frontend contains:

TypeValidationPolicy

and related structural validation errors.

The policy belongs to compiler safety and input validation.

It must not be encoded into grammar productions.

The default policy currently permits no additional collection/identifier limits unless configured.

Any future policy must preserve the distinction:

language semantics
        ≠
implementation safety policy

---

71. Determinism

Type parsing must be deterministic.

Given identical:

source
language version
lexical configuration
grammar configuration

the parser must produce the same structural type representation.

Type grammar behavior must not depend on:

- current time;
- randomness;
- network state;
- filesystem state;
- hardware availability;
- target device;
- runtime state.

Deterministic ordering must be preserved.

The frontend currently uses ordered collections such as "Vec" for source-order-sensitive structures.

---

72. Source spans

Every production that participates in user-visible diagnostics should preserve source locations through the parser/AST integration.

Source spans are required for:

- compiler diagnostics;
- IDE/LSP navigation;
- formatting;
- refactoring;
- error recovery;
- provenance;
- source mapping;
- compatibility tooling.

A type grammar feature is incomplete if it parses correctly but destroys the location information required to diagnose it.

---

73. Diagnostics

The type grammar must distinguish syntax errors from semantic errors.

Syntax errors

Examples:

Vec<
Map<T,, U>
Result<T
(T, U

These belong to parsing.

Semantic errors

Examples:

unknown type
unknown generic parameter
wrong generic arity
unsatisfied type bound
invalid associated type
invalid dependent type
invalid capability relationship
invalid resource relationship

These belong downstream.

The parser must not attempt to resolve symbols merely to decide whether a syntactically valid type exists.

---

74. Error recovery

ANTLR parser integration must preserve useful error recovery without inventing semantic types for invalid source.

Error recovery must not:

- allocate physical resources;
- query hardware;
- resolve QPU topology;
- invoke runtime code;
- perform backend selection.

Recovered parser structures must remain subject to structural validation before semantic use.

---

75. Security

The type grammar performs no:

- file I/O;
- network access;
- code execution;
- hardware access;
- runtime allocation;
- native calls.

Rust implementation requirements:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

remain required wherever Rust implementation code is added.

No type-system implementation may require Rust "unsafe".

---

76. Rust baseline

All compiler/frontend code integrated with this grammar must support:

Rust 1.97
Rust 1.97.1
Rust 2021
stable Rust
safe Rust

No nightly-only language feature may become a production dependency.

No:

unsafe

code is permitted.

---

77. Serialization

The frontend "TypeExpr" representation uses the repository's serialization architecture.

Type syntax and serialized AST representation must remain version-aware.

The existing:

TYPE_EXPR_SCHEMA_VERSION

must remain distinct from:

language version
compiler version
grammar version
IR version
backend version

Changing serialized type representation requires compatibility review.

---

78. Compatibility

Type grammar evolution must preserve source compatibility where possible.

Changes must distinguish:

addition
clarification
extension
deprecation
breaking change
historical syntax
experimental syntax

A source construct must not silently change meaning merely because a new backend or domain has been introduced.

Compatibility behavior belongs to:

grammar/compatibility/

and the language-version architecture.

---

79. Extension architecture

The current frontend provides:

TypeExpr::Extension
TypeExtension
TypeValueExtension

This is an important extensibility mechanism.

Future domains should prefer namespaced extensions over adding endless universal type keywords.

Conceptually:

extension namespace
    +
extension name
    +
arguments
    +
attributes

This permits new computational domains without requiring the universal type grammar to enumerate every future abstraction.

Extensions must still be:

- specified;
- validated;
- versioned;
- capability-aware where required;
- semantically resolved;
- tested;
- compatibility-controlled.

Extensibility does not mean unrestricted arbitrary semantics.

---

80. Domain neutrality

The type grammar must not become an AI grammar, quantum grammar, HDL grammar or hardware grammar.

Instead:

universal type foundation
        │
        ├── classical
        ├── quantum
        ├── hybrid
        ├── HDL
        ├── hardware
        ├── AI
        ├── data
        ├── distributed
        ├── networking
        └── future domains

All domains consume the same source-level type foundation.

---

81. Reasoning and knowledge integration

Reasoning constructs such as:

infer
deduce
reason

belong to the reasoning/semantic subsystem.

Knowledge constructs such as:

assert
retract
query

belong to knowledge/data/reasoning subsystems.

Their values, models, evidence and results use ordinary Zamani types.

The type grammar must not create an AI-only type hierarchy.

---

82. Learning and adaptation integration

Learning and adaptation may consume types such as:

Model<T>
Dataset<T>
State<T>
Policy<T>
Evidence<T>

where those are ordinary source-level types.

The type grammar does not own:

learn
adapt

semantics.

Those operations must integrate with:

effects
capabilities
resources
policies
contracts
provenance
execution

without changing the type authority.

---

83. Agents

Agent types must reuse normal type syntax.

The architecture is:

Agent<State>
    ↓
normal Zamani type
    ↓
agent semantics
    ↓
actor/concurrency model

The type subsystem must not create a second actor type system.

---

84. Neural-symbolic computation

Neural-symbolic systems should combine:

learned model types
+
symbolic types
+
knowledge types
+
reasoning results

through the common type architecture.

No separate compiler type system should be created for neural-symbolic computation.

---

85. Data and query integration

Data/query systems may introduce domain-specific type constructors through normal named and generic types.

Examples:

Dataset<Row>
Table<Row>
Query<Result>
Graph<Node, Edge>

The type grammar does not contain SQL, JSON or XML grammar.

Those belong to their respective dialect/interoperability systems.

Their semantic results must lower into the common Zamani type architecture.

---

86. FFI and ABI integration

Foreign interfaces consume type information.

The architecture is:

Zamani type
     ↓
semantic type
     ↓
FFI compatibility analysis
     ↓
ABI representation
     ↓
foreign target

The type grammar must not encode a specific ABI.

ABI representation belongs to:

grammar/interoperability/

and compiler/backend systems.

---

87. Reflection and metaprogramming

Reflection and type-level metaprogramming may inspect or construct type syntax.

They must operate through the canonical:

TypeExpr

representation.

They must not introduce:

ReflectionType
MetaType
QuantumMetaType
HardwareMetaType

as competing universal type representations.

Reflection must respect:

- capability policy;
- security policy;
- compilation phase;
- determinism;
- provenance;
- compatibility.

---

88. Type-level metaprogramming

Type-level computation must remain explicitly separated into:

source syntax
type-level representation
evaluation/checking
semantic type

The grammar only recognizes source syntax.

If compile-time evaluation is introduced, its resource policy must be explicit and separate from language semantics.

Compile-time computation must not be allowed to create hidden hardware assumptions.

---

89. Resource-aware types

Resource-aware type semantics may express relationships involving:

memory
compute
qubits
accelerators
communication
storage

but type syntax must not determine actual availability.

The correct direction is:

source type
    ↓
semantic type
    ↓
resource requirement
    ↓
resource negotiation
    ↓
target realization

This supports POCO-REAF.

---

90. Capability-aware types

Capability information may participate in semantic type checking.

For example:

Capability<QuantumMeasurement>
Capability<TensorCompute>

may represent source-level capability abstractions.

But the grammar must not query a machine to determine whether those capabilities exist.

The compiler/runtime performs capability negotiation.

---

91. Hardware independence

No type production may depend on:

CPU vendor
GPU vendor
FPGA vendor
QPU vendor
ASIC family
instruction set
physical device identifier
physical topology
register file size
cache size
memory bus width

unless such information is explicitly represented by a target-specific dialect or interoperability boundary.

Universal source syntax must remain portable.

---

92. Target-specific dialects

Target-specific types may exist through:

grammar/dialects/

or the repository's established interoperability architecture.

A target-specific type must not silently become a universal Zamani type.

The dialect must declare:

- ownership;
- version;
- syntax;
- semantic meaning;
- target constraints;
- capability requirements;
- compatibility;
- lowering;
- diagnostics.

---

93. Classical integration

Classical types use the same type foundation.

The classical subsystem must consume:

TypeExpr

rather than creating another type AST.

Classical numerical, symbolic and data computation must remain compatible with:

- generics;
- functions;
- arrays;
- tensors;
- contracts;
- effects;
- resources;
- capabilities;
- policies.

---

94. Quantum integration

Quantum types must use:

TypeExpr

at the source boundary and then enter the quantum semantic system.

No quantum grammar may create a parallel universal type system.

The canonical downstream boundary is:

quantum::ir

---

95. Hybrid integration

Hybrid types may combine ordinary and quantum types:

Model<State>
Tensor<Value>
QuantumState<T>
Hybrid<State, Classical>

where the corresponding declarations/constructors exist.

The type grammar must not create a separate hybrid type universe.

---

96. HDL and hardware/software co-design

Hardware/software co-design must use common type syntax.

A source abstraction such as:

Signal<T>
Buffer<T>
Accelerator<T>

must remain source-level.

Synthesis, placement, timing closure, physical resource selection and device realization remain downstream.

---

97. Distributed computation

Distributed types may express source abstractions such as:

Node<T>
Message<T>
Channel<T>
Distributed<T>
Replica<T>

where defined by the language/library.

The grammar must not encode a fixed number of nodes.

Distributed placement belongs downstream.

---

98. Networking

Network-oriented types must use normal type construction.

Examples:

Packet<T>
Stream<T>
Endpoint<T>
Message<T>

Network access is an effect/capability concern.

A network-related type does not itself perform network I/O.

---

99. Cryptography

Cryptographic abstractions should generally be represented as named/generic types and operations.

Examples conceptually:

Key<T>
Ciphertext<T>
Signature<T>
Hash<T>

The type grammar must not enumerate every cryptographic algorithm.

Algorithms belong in libraries, dialects and semantic capabilities.

---

100. Simulation

Simulation types may be represented through normal source types.

Simulation itself is an execution strategy.

The type grammar must not create a second simulation language.

The architecture remains:

type
    ↓
semantic program
    ↓
simulation execution strategy

---

101. Adaptive execution

Adaptive execution may consume types representing:

State<T>
Policy<T>
Capability<T>
Resource<T>
Decision<T>

The type grammar only represents their source structure.

Adaptive execution semantics belong to:

grammar/execution/
grammar/resources/
grammar/policies/

and downstream compiler/runtime systems.

---

102. Reproducibility

Types must have deterministic structural representation.

Where type serialization or compiler artifacts are persisted, the representation must include sufficient version information to distinguish incompatible schema versions.

Type syntax must not depend on runtime randomness.

---

103. Provenance

The type subsystem must preserve enough source structure and spans for downstream provenance.

Provenance may record:

source type
source location
type resolution
generic substitution
specialization
transformation
derived representation
verification

The grammar itself does not create provenance records.

---

104. Diagnostics and explainability

Type-related compiler diagnostics should be capable of explaining:

- which source type failed;
- where it occurred;
- which type was expected;
- which type was found;
- which constraint failed;
- which generic argument caused the failure;
- which capability/resource requirement was unsatisfied;
- which semantic rule was violated.

The type grammar supplies structure and source locations.

Semantic analysis supplies the meaning of the diagnostic.

---

105. No application-specific keyword explosion

The universal type grammar must not add special types solely because an application domain has become popular.

Do not add universal type keywords for concepts such as:

sentiment
vision
robotics
blockchain
VR
AR
payments
administration
legal actions

Those concepts belong to:

libraries
dialects
services
domain models
policies
applications

unless a concept genuinely requires universal language semantics.

---

106. No domain keyword explosion

Likewise, the type grammar must not enumerate every:

- quantum gate;
- machine-learning algorithm;
- tensor operator;
- CPU architecture;
- GPU model;
- FPGA family;
- ASIC;
- network protocol;
- cryptographic algorithm;
- scientific model.

Use:

named types
generic types
extensions
dialects
registered capabilities
semantic operations
libraries

where appropriate.

---

107. Type extension principle

A new domain should ideally be able to introduce a new type abstraction without modifying the universal type grammar.

Preferred:

DomainType<T>
Domain::Type<T>
extension(domain, type, ...)

rather than adding:

DOMAIN_TYPE_1
DOMAIN_TYPE_2
DOMAIN_TYPE_3
...

to the universal grammar.

The extension must still have a defined semantic contract.

---

108. Integration with declarations

Declarations own declaration syntax.

For example:

type Foo = ...
struct Foo ...
enum Foo ...

The declaration subsystem invokes the canonical:

typeExpression

for type positions.

The type grammar must not duplicate declaration grammar.

---

109. Integration with functions

Function parameters and return types consume the canonical type grammar.

Relevant repository consumers include:

grammar/functions/
grammar/declarations/

and frontend AST nodes.

The integration is:

function declaration
    ↓
parameter type
    ↓
typeExpression
    ↓
TypeExpr

Function semantics remain outside the type grammar.

---

110. Integration with modules

Modules provide namespaces.

The type grammar consumes qualified names.

Module resolution determines whether:

module::Type

exists.

The type grammar does not resolve modules.

---

111. Integration with expressions

Expressions may contain:

- type ascriptions;
- casts;
- generic applications;
- constructors;
- type tests;
- type-level forms.

The expression subsystem consumes the canonical type grammar rather than redefining it.

---

112. Integration with patterns

Pattern matching depends on type information.

The architecture is:

pattern syntax
    ↓
expression/statement grammar
    ↓
TypeExpr
    ↓
semantic pattern checking

No second type grammar should exist inside pattern grammar.

---

113. Integration with macros

Macros may consume or produce type syntax.

They must use the canonical type grammar/AST.

Macro expansion must preserve:

- source spans;
- type structure;
- hygiene;
- diagnostics;
- provenance.

Macros must not create a second semantic type representation.

---

114. Integration with compile-time computation

Compile-time computation may construct types or type-level values.

It must use the canonical source-level representations.

The compiler must distinguish:

source type syntax
compile-time evaluation
semantic type

Compile-time evaluation must not silently query hardware and rewrite source meaning.

---

115. Integration with resources

"grammar/resources/" owns:

requirements
capabilities
constraints
budgets
preferences
hints
negotiation
scalability

The type subsystem supplies type information.

The resource subsystem determines feasibility.

Example intended architecture:

TypeExpr
   ↓
semantic type
   ↓
resource requirement
   ↓
requires capability(...)
   ↓
capability negotiation
   ↓
execution plan

---

116. Integration with effects

"grammar/effects/" owns effects.

Types may be used by effect declarations but must not redefine effects.

Examples of semantic effects may include:

io
network
mutation
randomness
native
foreign
distributed
measurement
learning
adaptation
reflection
code generation
simulation

The exact stable effect vocabulary belongs to the effect specification.

---

117. Integration with validation

"grammar/validation/" owns contracts and validation syntax.

The type system supplies:

TypeExpr

for type-related conditions.

The validation system may use type information to verify:

requires
ensures
invariant
assume
guarantee
property

---

118. Integration with policies

Policy syntax belongs outside the type grammar.

Policies may constrain:

- types;
- resources;
- capabilities;
- effects;
- execution;
- adaptation;
- deployment;
- security.

The type grammar must remain policy-neutral.

---

119. Integration with security

Security policy may restrict types or type-related operations.

For example, a security subsystem may forbid certain foreign or native representations.

That does not change the source type grammar.

The architecture is:

type
 ↓
semantic type
 ↓
security policy analysis

---

120. Integration with interoperability

Interoperability systems consume semantic type information for:

FFI
ABI
foreign functions
foreign types
data exchange
serialization
external formats

The type grammar remains target-neutral.

---

121. Integration with canonical IR

Type syntax must eventually become semantic type information consumed by the canonical IR architecture.

The type grammar must not directly emit:

LLVM
QIR
MLIR
vendor IR
physical quantum IR
hardware-specific IR

The direction is:

source type
    ↓
TypeExpr
    ↓
semantic type
    ↓
canonical semantic model
    ↓
IR

For quantum semantics:

source quantum type
    ↓
TypeExpr
    ↓
semantic quantum type
    ↓
quantum::ir

---

122. Integration with ZUIR

The repository's canonical universal IR architecture must remain downstream from the source AST/semantic model.

"TypeExpr" is not ZUIR.

Type syntax must not embed:

- register allocation;
- physical topology;
- QEC routing;
- calibration;
- machine instruction selection.

---

123. Integration with QEC and resilience

Type syntax may carry semantic information consumed by quantum resilience analysis.

It must not implement:

- syndrome generation;
- decoder selection;
- physical-qubit allocation;
- QEC scheduling;
- stabilizer execution;
- calibration.

Those belong downstream of "quantum::ir".

---

124. Integration with ZQN

ZQN is downstream from source-level type syntax.

The type subsystem may supply semantic information consumed by resilience analysis.

It must not encode noise models as universal type grammar rules.

---

125. Integration with HAL

HAL is a target-realization boundary.

The type grammar must never directly select HAL implementations.

The direction remains:

TypeExpr
   ↓
semantic type
   ↓
IR
   ↓
lowering
   ↓
target plan
   ↓
HAL

---

126. Source compatibility

A type syntax change must be evaluated against:

lexer
parser
AST
semantic resolver
type checker
generic checker
declaration parser
function parser
expression parser
macro system
serialization
IR generation
IR verification
LSP
formatter
tests

A change is not complete when only ANTLR accepts it.

---

127. Conformance status

Every type feature must have an explicit implementation status.

Use the repository's conformance vocabulary:

SPECIFIED
LEXER_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
BACKEND_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL
PARTIALLY_IMPLEMENTED
PLANNED
DEPRECATED

Do not infer "STABLE" from parser acceptance.

---

128. Production feature definition

A type feature is production-ready only when all required stages are complete:

specification
     ↓
lexical support
     ↓
grammar support
     ↓
AST representation
     ↓
structural validation
     ↓
semantic checking
     ↓
diagnostics
     ↓
serialization compatibility
     ↓
canonical IR integration
     ↓
cross-domain integration
     ↓
tests
     ↓
compatibility review

Not every type construct requires a backend-specific implementation.

However, every stable construct requires a defined semantic destination.

---

129. Required file contract

Every type grammar file must document:

PURPOSE
OWNS
DOES NOT OWN
INPUTS
OUTPUTS
LEXER DEPENDENCIES
GRAMMAR DEPENDENCIES
PUBLIC RULES
PRIVATE RULES
AST CONTRACT
SEMANTIC CONTRACT
TYPE CONTRACT
EFFECT CONTRACT
CAPABILITY CONTRACT
RESOURCE CONTRACT
CONTRACT/VALIDATION CONTRACT
POLICY CONTRACT
PROVENANCE CONTRACT
IR CONTRACT
QUANTUM BOUNDARY
HDL BOUNDARY
INTEROPERABILITY BOUNDARY
DIAGNOSTICS
COMPATIBILITY
SCALABILITY
DETERMINISM
SECURITY
POSITIVE TESTS
NEGATIVE TESTS
BOUNDARY TESTS
CROSS-DOMAIN TESTS
SCALABILITY TESTS
COMPLETION CRITERIA

This is mandatory for production grammar components.

---

130. Dependency declaration

Each production type grammar component must explicitly identify:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
LEXER_OWNER:
AST_OWNER:
SEMANTIC_OWNER:
IR_OWNER:
SPEC_OWNER:
TEST_OWNER:
COMPATIBILITY_OWNER:

Example:

DEPENDS_ON:
    canonical Zamani lexer
    canonical typeExpression

EXPORTS:
    arrayType

CONSUMED_BY:
    grammar/types/types.g4
    declarations
    functions
    expressions

AST_OWNER:
    src/frontend/ast/node/types/type_expr.rs

SEMANTIC_OWNER:
    semantic type subsystem

IR_OWNER:
    canonical semantic IR

SPEC_OWNER:
    grammar/specification/types.md

TEST_OWNER:
    grammar/tests/

The purpose is to make file completion independently verifiable.

---

131. Independent-file-first implementation rule

Type grammar development must proceed from foundational files outward.

Recommended order:

1. README.md
2. canonical type specification
3. lexical/token dependencies
4. canonical types.g4
5. primitive.g4
6. named.g4
7. generic.g4
8. tuple.g4
9. array.g4
10. slice.g4
11. function type integration
12. reference.g4
13. pointer.g4
14. option/optional convergence
15. result.g4
16. algebraic/sum/union
17. associated.g4
18. constraints/type-constraints convergence
19. type-class.g4
20. dependent.g4
21. linear.g4
22. affine.g4
23. temporal.g4
24. resource.g4
25. quantum integration
26. classical integration
27. hardware integration
28. extension architecture
29. cross-domain conformance

A component must define its downstream integration contract before being marked complete.

---

132. Existing-file integration before new-file creation

Do not create a new file merely because a conceptual feature is missing.

First inspect:

grammar/types/
grammar/core/
grammar/declarations/
grammar/functions/
grammar/expressions/
grammar/resources/
grammar/effects/
grammar/validation/
grammar/metaprogramming/
grammar/interoperability/
grammar/quantum/
grammar/hardware/
grammar/hdl/

Then inspect the corresponding frontend AST and semantic implementation.

Only create a new grammar file if the responsibility is genuinely independent.

---

133. Duplicate-file policy

The following kinds of duplication require explicit reconciliation:

types.g4
type.g4

primitive.g4
primitives.g4

optional.g4
option.g4

map.g4
map-types.g4

constraints.g4
type-constraints.g4

sum.g4
algebraic-types.g4
union.g4

Similarity alone does not determine which file is canonical.

Ownership must be established by:

grammar composition
AST integration
specification
consumers
tests
compatibility

The canonical owner must be documented.

---

134. No silent compatibility break

Existing source syntax must not be removed merely because a cleaner grammar is designed.

If a legacy syntax must be removed:

legacy syntax
    ↓
deprecated status
    ↓
compatibility diagnostic
    ↓
migration path
    ↓
removal according to language version

Compatibility belongs to the compatibility subsystem.

---

135. Scalability tests

Type grammar tests must include generated structures that exercise increasing:

- generic arity;
- tuple cardinality;
- nesting;
- path length;
- symbolic cardinality;
- type composition;
- domain composition.

Tests must verify that the grammar does not contain artificial finite ceilings.

Compiler safety policies may intentionally reject extreme input, but such rejection must identify the implementation policy rather than claiming the source language has a smaller type universe.

---

136. Negative tests

Every type component requires invalid-input tests.

Examples:

<>
Vec<>
Map<,>
Map<T,, U>
Map<T U>
Result<T
(T, U
[T;
&T
&mut

The exact expected diagnostic belongs to the implemented syntax.

Negative tests must distinguish:

syntax error

from:

semantic error

---

137. Semantic-error tests

Valid syntax with invalid meaning must parse before semantic rejection.

Examples include:

UnknownType
Unknown<T>
Result<T>
MissingAssociatedType::Item
InvalidBound<T>

The semantic test suite must verify that the parser does not incorrectly reject these merely because the semantic environment does not contain the referenced declaration.

---

138. Cross-domain tests

The type suite must contain cross-domain combinations such as:

Register<Qubit>
Tensor<Value>
Signal<Value>
Accelerator<Model>
Buffer<Packet>
Dataset<Record>
Resource<Qubit>
Capability<QuantumMeasurement>
Model<QuantumState>

provided the referenced names/types are actually declared in the relevant test environment.

These tests demonstrate that one type architecture serves multiple domains.

---

139. Quantum cross-domain tests

Required conceptual coverage includes:

QuantumState<T>
QRegister<N>
Register<Qubit>
Tensor<QuantumState<T>>
Model<QuantumState<T>>

where supported by the actual language vocabulary.

The tests must verify that quantum source types remain target-independent.

---

140. HDL cross-domain tests

Examples:

Signal<T>
Register<T>
Buffer<T>
Module<T>
Accelerator<T>

should be tested where the corresponding types are defined.

No test may rely on a universal fixed bus/register width unless that width is explicitly source-defined.

---

141. AI/data cross-domain tests

Examples:

Model<Input, Output>
Dataset<Row>
Distribution<Value>
Evidence<Claim>
Knowledge<Fact>

should use ordinary type construction.

The type grammar must not require a special parser path merely because a type participates in reasoning or learning.

---

142. Generic semantic tests

Generic tests must separately verify:

syntax
argument preservation
argument ordering
nested arguments
trailing commas
arity checking
generic parameter resolution
bound checking
substitution
specialization
serialization
diagnostics

The grammar owns only the syntactic portion.

---

143. Determinism tests

For identical input, verify identical:

parse structure
TypeExpr
argument order
path order
source spans
serialization

No type parser behavior may depend on:

hardware
time
randomness
network
filesystem
environment

---

144. Property-based and generated testing

Where repository infrastructure permits, generated tests should exercise:

type nesting
generic structures
qualified names
symbolic values
recursive type composition
extension payloads

Generated tests must remain resource-aware.

A test generator must not accidentally create an artificial language limit.

---

145. Fuzzing

Type parsing should be fuzz-tested for:

- malformed delimiters;
- deep nesting;
- long names;
- repeated generic arguments;
- invalid separators;
- Unicode identifiers where supported;
- malformed extension syntax;
- malformed symbolic values.

The parser must not panic on malformed untrusted source.

---

146. Compiler safety

Compiler implementation must use safe Rust.

Fuzzing and validation must not rely on "unsafe" code.

The type subsystem must be robust against:

- malformed source;
- adversarial nesting;
- excessive collections;
- malformed serialized AST;
- invalid extensions.

Explicit validation policies may protect compiler resources.

---

147. Resource limits versus language limits

This distinction is mandatory.

A compiler may have:

TypeValidationPolicy

that says:

max_collection_items = Some(...)

for a particular invocation.

That is an implementation policy.

It is fundamentally different from defining:

MAX_GENERIC_ARITY

as a universal Zamani language restriction.

The first protects an implementation.

The second changes the language.

Only the former is permitted as an implementation safety mechanism.

---

148. Memory scalability

The type grammar must use structural repetition and recursive composition rather than fixed enumerations.

The compiler implementation may use:

Vec
Box
Arc

and other safe Rust structures.

The current "TypeExpr" architecture already uses recursive boxed structures and ordered vectors.

The implementation must avoid introducing unnecessary machine-sized semantic assumptions into source representation.

---

149. Identifier scalability

Identifiers must remain source-level names.

The AST currently represents type names using an immutable string representation.

The grammar must not impose arbitrary language-level identifier lengths.

Compiler safety policies may impose configurable limits when required to protect resources.

---

150. Path scalability

Qualified paths use ordered path segments.

The grammar must not define a maximum number of path segments.

Semantic resolution may fail if a path is invalid.

That is not a grammar capacity restriction.

---

151. Generic scalability

Generic applications must preserve arbitrary source structure subject to implementation resources.

The grammar must not define:

MAX_GENERIC_ARITY
MAX_GENERIC_DEPTH

or equivalent hidden limits.

---

152. Tuple scalability

Tuple syntax must not contain a finite enumeration of tuple sizes.

Use repeated grammar constructs and the existing vector-based AST representation.

---

153. Array scalability

Array cardinality must remain symbolic where source syntax provides a symbolic value.

The parser must not require a host integer representation.

---

154. Quantum scalability

The type grammar must never assume a maximum number of:

qubits
quantum registers
quantum states
quantum resources
quantum devices

A target may report insufficient resources later.

That is target feasibility, not source-language failure.

---

155. Hardware scalability

The grammar must never enumerate a finite universal inventory of:

CPUs
GPUs
FPGAs
ASICs
QPUs
accelerators
nodes
devices

Hardware discovery and selection remain outside the type grammar.

---

156. Domain scalability

Adding a new computational domain should preferably require:

new semantic/domain module
+
possibly a dialect
+
type declarations/extensions
+
tests

rather than rewriting the universal type system.

A new domain must reuse:

TypeExpr
generic system
constraints
resources
capabilities
effects
policies
provenance

where applicable.

---

157. Compiler specialization

Generic types may eventually be specialized or monomorphized.

The type grammar does not perform specialization.

The direction is:

generic source type
      ↓
TypeExpr
      ↓
semantic generic type
      ↓
specialization decision
      ↓
specialized semantic representation
      ↓
IR

Specialization must not alter the source program's declared meaning.

---

158. Target lowering

Target lowering may choose:

memory representation
register representation
vector representation
device representation
quantum representation
HDL representation

The type grammar must not make those choices.

---

159. Reproducible compilation

Type structures must be reproducible from identical source and compiler configuration.

Generic argument order, path order and type structure must not depend on unordered map iteration.

Serialized forms must be versioned.

---

160. Type equality

Source-level structural equality and semantic type equality are different concepts.

For example:

A

and:

module::A

may require semantic name resolution before equality can be established.

The grammar preserves source structure.

The semantic type system determines meaning.

---

161. Type aliases

Type aliases belong to the declaration subsystem.

Their right-hand side consumes:

typeExpression

Alias expansion/resolution belongs to semantic analysis.

The type grammar must not duplicate alias declaration syntax.

---

162. Recursive types

Recursive type structure must be representable where the language semantics permit it.

The grammar must not impose arbitrary recursive type depth.

Semantic checking determines whether recursion is legal.

Compiler validation may protect against pathological input.

---

163. Type extensions

"TypeExpr::Extension" allows future source-level constructs to remain extensible.

Extensions must include sufficient namespace/name/argument information to prevent collisions.

An extension must have:

owner
namespace
version
syntax
semantic definition
AST representation
validation
diagnostics
compatibility
tests

An extension is not automatically stable merely because the parser accepts it.

---

164. Extension security

Extensions must not automatically gain:

- native execution;
- filesystem access;
- network access;
- hardware access;
- reflection privileges;
- code-generation privileges.

Those capabilities must be explicitly authorized through the repository's capability/security architecture.

---

165. Type attributes

The frontend supports type-level attributes through "TypeAttribute" and extension attributes.

Attributes must remain:

namespaced
structured
source-preserving
semantically defined

Unknown attributes may be accepted only where the language specification permits them.

The grammar must not invent semantics for an unknown attribute.

---

166. Type attributes and provenance

Attribute source information should remain available to downstream tooling.

Attributes may participate in:

- semantic analysis;
- optimization;
- code generation;
- documentation;
- provenance;
- tooling.

They must not silently override core type semantics.

---

167. Type system and contracts

Contracts may constrain types.

For example, semantic analysis may verify that a value satisfying:

requires ...

has an appropriate type.

The contract system owns the condition.

The type system owns the type.

This separation prevents circular grammar ownership.

---

168. Type system and reasoning

Reasoning systems may reason about type relationships.

The type grammar still only provides source structure.

A reasoning engine must consume semantic type information rather than parsing types independently.

---

169. Type system and learning

Learning systems may use types to express:

input
output
model
dataset
state
parameter
distribution

The learning subsystem owns learning semantics.

The type system supplies reusable type structure.

---

170. Type system and adaptation

Adaptation may change model or strategy state, but it must not mutate the language's type meaning.

Any adaptation affecting program representation must pass through explicit:

policy
capability
effect
provenance
validation

mechanisms.

---

171. Type system and explainability

Type-related transformations should be explainable through compiler diagnostics/provenance.

For example:

source type
    ↓
alias resolution
    ↓
generic substitution
    ↓
semantic type

can be represented in diagnostic/provenance tooling.

The type grammar itself does not implement explanation.

---

172. Type system and agents

Agent state types must use ordinary Zamani types.

The concurrency subsystem owns:

actor
message
channel
task
scheduler

AI semantics may define agent behavior.

The type system remains shared.

---

173. Type system and sandboxing

A sandbox may constrain which types or operations are allowed in a context.

The type grammar does not implement the sandbox.

The architecture is:

type
 ↓
semantic type
 ↓
security/policy analysis
 ↓
sandbox decision

---

174. Type system and simulation

Simulation-specific types must remain ordinary types or dialect types.

Simulation execution does not require a second type system.

---

175. Type system and interoperability formats

JSON, XML, SQL and other external representations are not universal type grammar constructs.

They belong to:

grammar/dialects/
grammar/interoperability/

and their semantic adapters.

They must eventually map into Zamani's common type/data architecture.

---

176. No hidden backend semantics

The type grammar must never contain code or grammar branches that mean:

if GPU then ...
if QPU then ...
if FPGA then ...
if vendor == ...

Target-dependent behavior belongs downstream.

Source type meaning must remain target-independent.

---

177. No hidden physical topology

The type grammar must not encode:

grid size
mesh size
qubit adjacency
network topology
device coordinates
physical qubit IDs

Topology requirements belong to resource/target semantics.

---

178. No hidden register assumptions

The type grammar must not assume:

32-bit registers
64-bit registers
fixed register counts
fixed vector widths

unless explicitly represented as source-level target/dialect information.

---

179. No hidden memory assumptions

The type grammar must not assume:

64 GB
24 GB
fixed stack size
fixed heap size
fixed address space

as universal language properties.

Memory requirements belong to resources and target realization.

---

180. Type system and POCO-REAF failure

If a target cannot satisfy a type-associated requirement, the compiler/runtime must not silently rewrite the program's meaning.

The correct outcomes are things such as:

target compatible
target incompatible
requirement unsatisfied
capability unavailable
fallback selected
simulation selected
explicit alternative selected

according to the program's policies.

Source semantics must remain stable.

---

181. Completion criteria for "grammar/types/"

The type subsystem may be considered production-ready only when:

Authority

- [ ] One canonical "typeExpression" authority exists.
- [ ] "types.g4" is the canonical composition boundary.
- [ ] "type.g4" cannot become a competing type authority.
- [ ] Duplicate type grammar responsibilities have explicit owners.
- [ ] Legacy components have compatibility/deprecation status.

Lexer

- [ ] All consumed tokens originate from the canonical lexer.
- [ ] No type grammar defines lexer rules.
- [ ] Punctuation is not duplicated under incompatible token names.

AST

- [ ] Every stable type construct maps to "TypeExpr".
- [ ] No competing universal type AST exists.
- [ ] Source spans are preserved.
- [ ] Serialization is versioned.
- [ ] "TypeValueExpr" is used only within its actual implemented vocabulary.

Semantics

- [ ] Name resolution is downstream.
- [ ] Generic substitution is downstream.
- [ ] Type inference is downstream.
- [ ] Type equality is semantically defined.
- [ ] Bounds are semantically checked.
- [ ] Dependent types have defined semantic rules.
- [ ] Linear/affine semantics have defined checking.
- [ ] Resource/capability semantics remain downstream.

Quantum

- [ ] Quantum types use the common "TypeExpr".
- [ ] No physical qubit allocation occurs in the grammar.
- [ ] No universal quantum cardinality exists.
- [ ] "quantum::ir" remains the canonical quantum IR boundary.
- [ ] QEC/routing/scheduling remain downstream.

Classical

- [ ] Classical types use the common type system.
- [ ] Numerical abstractions remain target-neutral.
- [ ] Fixed representation widths are explicit rather than assumed.

HDL/hardware

- [ ] HDL types use the common type system.
- [ ] No universal hardware widths are imposed.
- [ ] Physical placement remains downstream.
- [ ] Hardware target selection remains downstream.

AI/data/reasoning

- [ ] AI concepts use normal type abstractions.
- [ ] Knowledge types use normal type abstractions.
- [ ] Learning types use normal type abstractions.
- [ ] Uncertainty uses normal type abstractions.
- [ ] Evidence/provenance types use normal type abstractions.
- [ ] Agents reuse the common concurrency/type architecture.
- [ ] No application-specific keyword explosion has entered the type grammar.

POCO-REAF

- [ ] No universal machine-capacity ceilings exist.
- [ ] No physical device identifiers are embedded in universal types.
- [ ] Symbolic resource/cardinality information can remain symbolic where supported.
- [ ] Target feasibility remains downstream.
- [ ] Source type meaning remains target-independent.

Safety

- [ ] Rust implementation uses no "unsafe".
- [ ] Rust 1.97 compatibility is tested.
- [ ] Rust 1.97.1 compatibility is tested.
- [ ] Malformed input cannot cause parser/compiler panics.
- [ ] Explicit validation policies are separated from language semantics.

Testing

- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Semantic-error tests exist.
- [ ] Boundary tests exist.
- [ ] Cross-domain tests exist.
- [ ] Quantum tests exist.
- [ ] HDL tests exist.
- [ ] AI/data tests exist.
- [ ] Generic tests exist.
- [ ] Dependent-type tests exist where implemented.
- [ ] Determinism tests exist.
- [ ] Serialization tests exist.
- [ ] Compatibility tests exist.
- [ ] Fuzz/property tests exist where repository infrastructure supports them.
- [ ] Scalability tests exist.

---

182. Required integration matrix

Every stable type feature must be traceable through:

SPECIFICATION
    ↓
LEXER
    ↓
GRAMMAR
    ↓
AST
    ↓
STRUCTURAL VALIDATION
    ↓
NAME RESOLUTION
    ↓
TYPE SEMANTICS
    ↓
CONSTRAINT CHECKING
    ↓
EFFECT ANALYSIS
    ↓
CAPABILITY ANALYSIS
    ↓
RESOURCE ANALYSIS
    ↓
CONTRACT ANALYSIS
    ↓
POLICY ANALYSIS
    ↓
PROVENANCE
    ↓
CANONICAL SEMANTIC MODEL
    ↓
IR
    ↓
OPTIMIZATION
    ↓
LOWERING
    ↓
TARGET REALIZATION

For quantum types, the relevant branch includes:

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
resilience/QEC
    ↓
ZQN
    ↓
HAL

---

183. Required integration with repository files

The type subsystem must remain compatible with the repository's existing major components.

At minimum, integration must be maintained with:

grammar/README.md
grammar/DESIGN.md
grammar/grammar.md
grammar/Zamani-Grammar.md

grammar/antlr/ZamaniLexer.g4
grammar/antlr/ZamaniParser.g4
grammar/Zamani.g4

grammar/core/
grammar/declarations/
grammar/functions/
grammar/expressions/
grammar/statements/
grammar/resources/
grammar/effects/
grammar/validation/
grammar/policies/
grammar/compatibility/
grammar/compile/
grammar/execution/
grammar/interoperability/
grammar/metaprogramming/
grammar/classical/
grammar/quantum/
grammar/hybrid/
grammar/hdl/
grammar/hardware/
grammar/ai/
grammar/data/
grammar/distributed/
grammar/networking/

grammar/specification/types.md

src/frontend/ast/node/types/
src/parser.rs
src/lexer.rs

The exact semantic and IR files must follow the current repository architecture rather than being invented by this README.

---

184. AST integration contract

The canonical AST owner is:

src/frontend/ast/node/types/type_expr.rs

The type grammar must not modify the AST merely because another domain grammar has been added.

If a genuinely new stable type concept cannot be represented by the current AST:

1. document the semantic requirement;
2. update the type specification;
3. design the AST extension;
4. define serialization compatibility;
5. implement the AST;
6. update semantic consumers;
7. update tests;
8. only then promote grammar syntax.

This prevents repeated reopening of completed grammar files.

---

185. Semantic integration contract

The type grammar produces source structure.

The semantic subsystem must determine:

what the type means
whether it is valid
which declaration it refers to
whether generic arguments are valid
whether constraints are satisfied
whether dependent values are legal
whether ownership rules are satisfied
whether resource/capability requirements are satisfied

No semantic decision should be hidden inside parser alternatives.

---

186. IR integration contract

The type grammar must not directly know the final machine representation.

Its IR contract is:

TypeExpr
    ↓
semantic type
    ↓
canonical semantic model
    ↓
appropriate IR type information

The exact IR representation is owned by the IR subsystem.

---

187. Quantum IR integration contract

Quantum type semantics must converge on:

quantum::ir

There must not be another independent quantum IR hidden inside:

grammar/types/

or:

src/frontend/ast/node/types/

Source-level quantum types remain source-level.

---

188. HDL integration contract

HDL source types must eventually feed HDL semantic representation and downstream HDL/domain IR.

The type grammar does not perform:

synthesis
placement
routing
timing closure
physical mapping

---

189. Resource integration contract

A type may contribute information to resource analysis.

The type grammar itself does not perform:

resource discovery
resource allocation
resource scheduling
device selection

This distinction is mandatory for POCO-REAF.

---

190. Capability integration contract

Capability relationships are resolved downstream.

The type grammar may parse a capability-oriented type expression.

It must not determine whether a machine provides that capability.

---

191. Policy integration contract

Policies may constrain types.

The type grammar remains policy-neutral.

Policy evaluation occurs after source parsing and semantic construction.

---

192. Provenance integration contract

The type subsystem must provide enough source information for provenance systems to answer:

Where did this type originate?

Which generic arguments were supplied?

Which aliases were resolved?

Which transformations changed the representation?

Which compiler decision depended on the type?

The grammar itself does not own the provenance database.

---

193. Compatibility integration contract

Any grammar migration must update:

grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/specification/types.md
grammar/compatibility/
tests

where relevant.

No feature may be called stable merely because the new grammar accepts it.

---

194. Documentation authority

This README is the navigation/integration contract for "grammar/types/".

It does not replace:

grammar/specification/types.md

for detailed normative language semantics.

It does not replace:

grammar/grammar.md

for implementation status.

It does not replace:

src/frontend/ast/node/types/type_expr.rs

for exact Rust enum/field names.

It does not replace semantic implementation files for semantic behavior.

---

195. Source of truth for exact AST names

When this README and the frontend AST disagree about an exact enum/field name:

src/frontend/ast/node/types/type_expr.rs

is the implementation authority until the architecture is deliberately changed.

The grammar documentation must then be corrected to match the canonical AST.

---

196. Source of truth for implemented syntax

When:

Zamani-Grammar.md

claims syntax that the current parser does not implement, the construct must not be presented as implemented merely because it appears in the historical/extended document.

Implementation status must be recorded in:

grammar/grammar.md

and verified by tests.

---

197. Source of truth for intended semantics

Normative type meaning belongs to:

grammar/specification/types.md

If a type feature is ambiguous, its semantics must be settled there before being declared stable.

---

198. Source of truth for compiler safety

Compiler-side validation policies belong to implementation code and its documented contracts.

They must not be converted into universal grammar restrictions.

---

199. Final architecture invariant

The entire type subsystem must preserve:

                    SOURCE TYPE
                         │
                         ▼
                   typeExpression
                         │
                         ▼
                      TypeExpr
                         │
                         ▼
                structural validation
                         │
                         ▼
                 semantic type system
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
      classical       quantum       domain types
                         │
                         ▼
                     quantum::ir
                         │
                         ▼
                  canonical IR
                         │
                         ▼
                    optimization
                         │
                         ▼
                     lowering
                         │
                         ▼
                resource negotiation
                         │
                         ▼
                     scheduling
                         │
                         ▼
                 target realization

The type grammar describes meaningful source-level type structure.

It does not describe the physical machine.

---

200. Final POCO-REAF invariant

A Zamani type must be written once at the source level.

The same type semantics must remain valid when the program is considered for different target scales.

The realization may vary:

tiny system
embedded system
CPU
multicore
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC
cluster
distributed system
cloud
future architecture

but the source-level type meaning must not be rewritten merely because the target changes.

---

201. Final scalability invariant

The language must scale according to available resources.

Therefore:

language abstraction
        ≠
physical capacity

and:

source cardinality
        ≠
machine capacity

and:

compiler safety policy
        ≠
language ceiling

and:

target capability
        ≠
type syntax

These distinctions are mandatory.

---

202. Final ownership invariant

There must be:

ONE
canonical
typeExpression

and:

ONE
canonical
source-level
TypeExpr

and:

ONE
canonical
semantic
type system

and:

ONE
canonical
quantum
IR boundary

Specialized grammar files are delegates/components, not competing languages.

---

203. Final extensibility invariant

Adding a future computational domain must not require the universal type grammar to enumerate every new type.

Prefer:

named types
qualified names
generic types
symbolic type values
extensions
dialects
semantic registration
capabilities
resources

over a permanently growing closed keyword inventory.

---

204. Definition of DONE for this README

This README itself is complete when:

- [x] It identifies the canonical type grammar authority.
- [x] It identifies the canonical frontend AST authority.
- [x] It distinguishes grammar from semantics.
- [x] It distinguishes source semantics from physical realization.
- [x] It incorporates the existing frontend "TypeExpr" vocabulary.
- [x] It does not falsely claim unsupported generic argument AST categories.
- [x] It does not falsely claim arbitrary type-level arithmetic is already implemented.
- [x] It accounts for the actual "primitive.g4" naming.
- [x] It addresses the "types.g4" / "type.g4" authority conflict.
- [x] It defines integration with declarations and functions.
- [x] It defines integration with expressions and patterns.
- [x] It defines integration with resources and capabilities.
- [x] It defines integration with effects, validation and policies.
- [x] It defines classical integration.
- [x] It defines quantum integration.
- [x] It preserves "quantum::ir" as the quantum IR boundary.
- [x] It defines HDL/hardware integration.
- [x] It defines AI/data/reasoning integration without creating another type system.
- [x] It defines FFI/interoperability boundaries.
- [x] It defines metaprogramming boundaries.
- [x] It defines provenance and compatibility requirements.
- [x] It prohibits artificial machine-capacity ceilings.
- [x] It distinguishes compiler validation policies from language limits.
- [x] It requires safe Rust.
- [x] It requires Rust 1.97/1.97.1 compatibility.
- [x] It requires deterministic behavior.
- [x] It defines positive, negative, semantic, boundary, cross-domain and scalability testing.
- [x] It provides per-file ownership and integration requirements.
- [x] It supports the POCO-REAF architecture.

The grammar files themselves are not automatically complete merely because this README is complete. Their implementation status must be verified through the repository's grammar, AST, semantic and test layers.This version is deliberately stricter than the existing document: it records what the repository actually has today instead of documenting planned facilities as if they already existed. In particular, the TypeExpr/TypeValueExpr distinction is important; otherwise the type README would promise generic value/resource arguments and arbitrary type-level arithmetic that the current frontend AST cannot faithfully represent.