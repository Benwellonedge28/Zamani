Zamani User-Defined Types

File: "grammar/types/user-defined-types.md"
Status: Normative production design
Subsystem: "grammar/types/"
Language: Zamani
Grammar technology: ANTLR4
Implementation baseline: Rust 1.97 or later
Rust edition: 2021
Safety: Safe Rust only; production implementation must contain no "unsafe" code
POCO-REAF: Required
Target model: Target-independent and resource-scalable
Primary owner: User-defined type semantic contract
Canonical syntax owners: "grammar/declarations/" and "grammar/types/" according to construct
Canonical type-expression owner: "grammar/types/types.g4"

---

1. Purpose

This document defines the complete production contract for user-defined types in Zamani.

User-defined types allow programmers to create domain-specific semantic types while remaining inside the single Zamani type system.

Examples include:

- structures;
- records;
- enumerations;
- algebraic data types;
- tagged unions;
- aliases;
- generic user-defined types;
- constrained user-defined types;
- recursive types;
- opaque/abstract types;
- interface/trait-associated types;
- capability-aware types;
- resource-aware types;
- domain-specific types;
- classical types;
- quantum types;
- HDL/hardware intent types;
- AI/data types;
- hybrid types.

The central rule is:

«A user-defined type extends the Zamani type universe; it does not create another type universe.»

A user-defined type must therefore pass through the same:

source
  ↓
lexer
  ↓
parser
  ↓
domain-neutral AST
  ↓
name resolution
  ↓
type resolution
  ↓
constraint solving
  ↓
semantic validation
  ↓
effects / capabilities / resources / contracts / policies
  ↓
canonical semantic type
  ↓
canonical IR
  ↓
target realization

This design is required for Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF).

---

2. Architectural Principle

A user-defined type describes program meaning.

It does not describe:

- a particular CPU;
- a particular GPU;
- a particular FPGA;
- a particular ASIC;
- a particular QPU;
- a particular register width;
- a particular memory size;
- a particular node count;
- a particular address width;
- a particular accelerator;
- a particular vendor;
- a particular physical topology.

Therefore:

User-defined type
        ↓
semantic type
        ↓
target-independent representation
        ↓
target-specific realization

and never:

User-defined type
        ↓
machine-specific representation

unless that machine-specific representation has explicitly been requested through a separate target/domain mechanism.

---

3. Scope

This document owns the type-level semantics and integration contract for user-defined types.

It covers:

1. declaration identity;
2. named type identity;
3. generic user-defined types;
4. type parameters;
5. type arguments;
6. aliases;
7. nominal types;
8. structural types;
9. records;
10. structs;
11. enums;
12. tagged unions;
13. algebraic data types;
14. recursive types;
15. opaque types;
16. abstract types;
17. constrained types;
18. refinement-compatible types;
19. associated types;
20. trait/type-class relationships;
21. type-level metadata;
22. attributes;
23. visibility;
24. module qualification;
25. declaration provenance;
26. source compatibility;
27. type compatibility;
28. canonical AST integration;
29. semantic type integration;
30. canonical IR integration;
31. classical integration;
32. quantum integration;
33. HDL/hardware integration;
34. AI/data integration;
35. effects;
36. capabilities;
37. resources;
38. contracts;
39. policies;
40. provenance;
41. deterministic compilation;
42. scalability;
43. diagnostics;
44. compatibility;
45. conformance testing.

---

4. Non-Goals

This file does not own:

- the universal "typeExpression" grammar;
- primitive type syntax;
- generic argument syntax;
- function type syntax;
- array syntax;
- reference syntax;
- pointer syntax;
- quantum operation syntax;
- hardware realization;
- physical layout;
- ABI selection;
- resource discovery;
- target discovery;
- capability discovery;
- scheduling;
- routing;
- QEC;
- ZQN;
- HAL;
- runtime allocation;
- machine-specific register allocation;
- backend-specific representation.

Those responsibilities remain with their existing owners.

---

5. Repository Authority Model

The following ownership is mandatory.

Responsibility| Authority
Language composition| "grammar/Zamani.g4"
Lexer| "grammar/antlr/ZamaniLexer.g4"
Token registry| "grammar/lexer/"
Declaration composition| "grammar/declarations/declarations.g4"
User-defined declaration syntax| appropriate file under "grammar/declarations/"
Type composition| "grammar/types/types.g4"
Named type syntax| "grammar/types/named.g4"
Generic type syntax| "grammar/types/generic.g4"
Record syntax| "grammar/declarations/records.g4" / corresponding canonical owner
Struct syntax| "grammar/declarations/structs.g4"
Enum syntax| "grammar/declarations/enums.g4"
Union syntax| "grammar/declarations/unions.g4"
Alias syntax| "grammar/declarations/aliases.g4"
Trait/interface declarations| declaration/type-class subsystem
Type semantics| semantic type subsystem
Type specification| "grammar/spec/type-system.md"
General semantics| "grammar/spec/semantics.md"
Resources| "grammar/spec/resources.md"
Effects| "grammar/spec/effects.md"
Portability| "grammar/spec/portability.md"
Compatibility| "grammar/spec/compatibility.md"
Policies| "grammar/spec/policies.md"
Quantum semantics| "grammar/spec/quantum.md"
Classical semantics| "grammar/spec/classical.md"
AST| existing domain-neutral AST subsystem
Classical IR| canonical classical IR
Quantum IR| "quantum::ir"
Hardware realization| hardware/backend/HAL subsystem
Tests| "grammar/tests/" and implementation test suites

This file must not override those authorities.

---

6. Critical Ownership Correction

The path:

grammar/types/user-defined-types.md

is documentation/specification, not a second grammar implementation.

User-defined type declaration syntax belongs under:

grammar/declarations/

while user-defined type use belongs to:

grammar/types/named.g4
grammar/types/generic.g4
grammar/types/types.g4

Therefore the architecture is:

declaration syntax
        │
        ▼
grammar/declarations/
        │
        ▼
domain-neutral AST declaration
        │
        ▼
symbol/type environment
        │
        ▼
grammar/types/named.g4
grammar/types/generic.g4
grammar/types/types.g4
        │
        ▼
semantic type resolution

This prevents declaration syntax from being duplicated inside "grammar/types/".

---

7. Required File-Level Contract

This file establishes the contract that implementation files must satisfy.

Every production user-defined-type-related grammar or implementation file must define:

PURPOSE
OWNS
DOES_NOT_OWN
DEPENDS_ON
EXPORTS
CONSUMED_BY
LEXER_DEPENDENCIES
GRAMMAR_DEPENDENCIES
AST_OWNER
SEMANTIC_OWNER
TYPE_OWNER
EFFECT_OWNER
CAPABILITY_OWNER
RESOURCE_OWNER
CONTRACT_OWNER
POLICY_OWNER
PROVENANCE_OWNER
IR_OWNER
RUNTIME_OWNER
TEST_OWNER
COMPATIBILITY_OWNER
SCALABILITY_REQUIREMENTS
DETERMINISM_REQUIREMENTS
SAFETY_REQUIREMENTS
DIAGNOSTICS
COMPLETION_CRITERIA

The actual grammar file may encode a subset of this information in comments, while this document provides the normative cross-file contract.

---

8. Dependency Contract

User-defined types depend conceptually on:

canonical lexer
      ↓
declaration grammar
      ↓
named type grammar
      ↓
generic grammar
      ↓
canonical type grammar
      ↓
domain-neutral AST
      ↓
symbol/type resolution
      ↓
semantic type system

They may additionally consume:

constraints
bounds
associated types
type classes
variance
dependent types
existential types
path-dependent types
type families

where those facilities exist and are supported.

The dependency graph must remain acyclic.

---

9. Required Direction of Dependency

The architecture must be:

declarations
     ↓
AST declarations
     ↓
type expressions
     ↓
type resolution
     ↓
semantic types

not:

types
  ↓
declarations
  ↓
types

Circular semantic ownership is prohibited.

---

10. User-Defined Type Identity

Every named user-defined type must have a stable semantic identity.

A type identity must not be based solely on its source spelling.

For example:

module_a::User
module_b::User

are potentially distinct types even though both are named "User".

The semantic identity should incorporate the resolved declaration identity, namespace/module identity, and applicable generic parameters.

Conceptually:

TypeIdentity {
    declaration_identity
    namespace_identity
    generic_parameters
    declaration_version
}

The exact implementation representation belongs to the semantic type subsystem.

---

11. No String-Only Type Identity

The implementation must not use raw source strings as the sole semantic type identity.

This is insufficient for:

- aliases;
- modules;
- imports;
- generic instantiations;
- shadowing;
- incremental compilation;
- separate compilation;
- versioning;
- generated declarations;
- provenance.

Names are resolved first.

Semantic identity is then assigned.

---

12. Nominal User-Defined Types

Zamani must support nominal user-defined types.

A nominal type is identified by its declaration identity.

For example:

type Meter = ...
type Second = ...

must not automatically become interchangeable merely because their underlying structures happen to be identical.

This is essential for semantic safety.

Nominal identity is particularly important for:

- units;
- domain concepts;
- security-sensitive values;
- quantum resources;
- hardware resources;
- protocol states;
- physical quantities;
- application-specific abstractions.

---

13. Structural Compatibility

Where Zamani explicitly supports structural typing, compatibility must be determined by the canonical semantic type rules.

Structural compatibility must not be inferred by the parser.

For example, two records with equivalent fields may be structurally compatible if the type system specifies that behavior.

But:

structural equivalence

must not silently override:

nominal identity

when the type is declared nominal.

---

14. User-Defined Type Categories

The semantic system should support the following general categories.

UserDefinedType
├── Struct
├── Record
├── Enum
├── Union
├── Algebraic
├── Alias
├── Opaque
├── Abstract
├── Constrained
├── Generic
├── Recursive
└── DomainExtension

This is a semantic classification.

It must not require the universal grammar to enumerate every future user-defined type family.

---

15. Struct Types

Struct declarations define named product-like types.

Conceptually:

struct Point {
    x: Real;
    y: Real;
}

The declaration grammar owns the surface syntax.

The type system owns:

- type identity;
- field type resolution;
- generic resolution;
- compatibility;
- recursive validation;
- semantic representation.

The type system does not own:

- physical field offsets;
- ABI layout;
- alignment;
- register allocation.

---

16. Record Types

Records represent structured collections of named fields.

Conceptually:

record Person {
    name: String;
    age: Integer;
}

The semantic system must retain:

- declaration identity;
- field identity;
- field order where semantically relevant;
- field names;
- field types;
- modifiers;
- visibility;
- attributes;
- generic parameters;
- constraints.

Physical representation remains a backend concern unless explicitly specified by the language.

---

17. Field Identity

Fields must have stable semantic identities.

A field identity should not depend solely on source position.

This is required for:

- refactoring;
- diagnostics;
- serialization metadata;
- provenance;
- incremental compilation;
- compatibility checking.

---

18. Field Order

Field order must be retained if the language semantics expose order.

If field order is semantically irrelevant, semantic normalization may canonicalize it where appropriate.

Physical memory layout must not be inferred merely from source ordering.

---

19. Enum Types

Enums define a finite source-level set of named alternatives.

The semantic model must distinguish:

enum declaration identity
enum variant identity
variant payload types
variant metadata

An enum must not be automatically lowered to a particular machine integer width at the type-grammar level.

---

20. Tagged Unions

Tagged unions must represent alternatives and their associated payloads.

Conceptually:

union Value {
    IntegerValue(Integer),
    TextValue(String),
    ...
}

The grammar should not impose a fixed number of alternatives.

The implementation must support arbitrary alternatives subject only to available compiler resources.

---

21. Algebraic Data Types

Algebraic types combine:

- product structures;
- sum structures;
- recursive structures;
- generic structures.

They must reuse existing:

generic.g4
record.g4
sum.g4
union.g4

rather than creating a second generic/type-expression mechanism.

---

22. Aliases

An alias gives another source-level name to an existing type.

Conceptually:

type UserId = Integer;

The semantic model must distinguish:

alias

from:

nominal new type

unless the language specification explicitly makes them equivalent.

Alias expansion must be controlled by semantic resolution rather than textual substitution.

---

23. Alias Transparency

Aliases may be transparent for compatibility where specified.

However, alias identity and declaration identity should remain available for:

- diagnostics;
- documentation;
- provenance;
- tooling;
- source mapping.

---

24. Opaque Types

An opaque type exposes an abstract identity while hiding its representation.

This is important for POCO-REAF because implementation representation can change across targets without changing source semantics.

For example:

opaque type Hash;

does not prescribe:

- bit width;
- storage layout;
- CPU representation;
- GPU representation;
- FPGA representation.

---

25. Abstract Types

Abstract types allow interfaces/contracts to be expressed without exposing representation.

Their resolution may depend on:

- implementation declarations;
- generic constraints;
- type classes;
- capabilities;
- modules;
- policies.

The type system remains the semantic authority.

---

26. Generic User-Defined Types

User-defined types must support arbitrary generic parameter counts.

Conceptually:

struct Pair<A, B> {
    first: A;
    second: B;
}

and:

struct Container<A, B, C, ...> { ... }

The language must not define:

MAX_TYPE_PARAMETERS
MAX_GENERIC_ARGUMENTS
MAX_GENERIC_DEPTH

as semantic limits.

Compiler resource controls may exist outside the language.

---

27. Generic Parameter Kinds

Where supported, parameters may represent:

- types;
- compile-time values;
- lifetimes;
- capabilities;
- other type-level entities.

The canonical generic subsystem owns parameter syntax.

User-defined types merely consume the resulting generic representation.

---

28. Generic Bounds

User-defined types may impose bounds.

Conceptually:

type Container<T>
where T: Comparable

Bounds are owned by the canonical bounds/constraint system.

User-defined types must not create an independent bound language.

---

29. Type Constraints

A user-defined type may participate in:

- equality constraints;
- subtype constraints;
- trait/type-class constraints;
- associated-type constraints;
- capability constraints;
- resource constraints;
- dependent constraints.

These must be represented through the existing constraint architecture.

---

30. Associated Types

User-defined types may expose associated types where the language's trait/interface model supports them.

For example:

interface Collection {
    type Item;
}

Associated type declaration syntax belongs to the declaration/type-class subsystem.

Associated type references belong to:

grammar/types/associated.g4

or the repository's canonical associated-type owner.

---

31. Type Classes / Traits

User-defined types may implement traits/type classes.

The user-defined-type specification must not define a competing dispatch mechanism.

The relationship is:

user-defined type
       ↓
trait/type-class implementation
       ↓
constraint solving
       ↓
semantic resolution

---

32. Variance

Generic user-defined types must participate in the canonical variance system where supported.

Variance is not inferred independently inside this file.

The semantic type checker determines variance according to the canonical type rules.

---

33. Recursive User-Defined Types

Recursive types must be supported without artificial semantic depth limits.

Example:

type Node = {
    value: Value,
    next: Option<Node>
};

The grammar should parse the symbolic reference.

The semantic resolver determines whether the recursion is valid.

The parser must not attempt to recursively expand the type.

---

34. Recursive Resolution

Recursive types require a declaration environment capable of representing incomplete declarations.

Resolution should therefore use a staged model:

declare identity
      ↓
register declaration
      ↓
collect type references
      ↓
resolve dependencies
      ↓
validate recursion
      ↓
construct semantic type

This avoids infinite recursive expansion.

---

35. Recursive Type Safety

The semantic system must distinguish valid recursive structures from invalid infinitely expanding definitions.

The exact termination rules belong to the type-system specification and semantic implementation.

The grammar must remain independent of those decisions.

---

36. Type Declaration Visibility

User-defined types participate in existing visibility rules.

Possible visibility dimensions include:

- module;
- package;
- crate/project;
- public;
- private;
- restricted;
- dialect scope.

The type grammar does not redefine visibility.

---

37. Namespaces

User-defined types must support qualified names.

Examples:

math::Vector
physics::Vector
quantum::State
hardware::Signal

The semantic resolver determines identity.

The parser only represents the name structure.

---

38. Namespace Stability

Moving a declaration between namespaces may be a compatibility-breaking operation.

The compatibility subsystem must therefore be able to track:

old identity
new identity
migration
alias
deprecation

where supported.

---

39. Imports

User-defined types may be imported through the existing module/import system.

Type resolution must occur after import/module resolution has established the relevant namespace environment.

No user-defined type grammar may implement a second import mechanism.

---

40. Shadowing

If Zamani permits type-name shadowing, the semantic resolver must apply the language's canonical lexical and namespace rules.

The parser must not resolve shadowing.

Diagnostics must identify ambiguous or shadowed declarations where required.

---

41. Declaration and Use Separation

The following are different constructs:

type declaration

and:

type reference

Declaration:

struct User { ... }

Use:

User

The declaration grammar owns the first.

The canonical type grammar owns the second.

---

42. Canonical Type Reference

All user-defined type uses must ultimately become the canonical named-type representation.

Conceptually:

named.g4
    ↓
NamedTypeExpr
    ↓
resolved declaration identity
    ↓
semantic UserDefinedType

No specialized domain should bypass this path.

---

43. Generic Application

A generic user-defined type:

Vector<Real>

must use:

named type
+
generic application

rather than a dedicated grammar for every user-defined generic type.

---

44. No Generic Type Enumeration

The language must not require additions such as:

vectorType
matrixType
tensorType
modelType
graphType
quantumVectorType
hardwareVectorType

merely because a library introduces a new type.

Those should remain user-defined types.

---

45. Open-World Type Extension

The user-defined type model must be open-ended.

New libraries may introduce:

Matrix<T>
Tensor<T>
Graph<N, E>
Model<I, O>
QuantumState<T>
Signal<T>
Packet<T>

without modifying the universal type grammar.

---

46. Type Attributes

User-defined types may carry attributes.

Examples include metadata describing:

- representation;
- serialization;
- visibility;
- derivation;
- documentation;
- verification;
- capability requirements;
- resource requirements;
- domain affiliation.

Attribute syntax belongs to the canonical attribute subsystem.

The user-defined type system consumes the resulting semantic metadata.

---

47. Type Metadata

Metadata must not alter semantic type identity unless the language specification explicitly says so.

For example:

documentation metadata

must not make two otherwise identical types semantically distinct.

By contrast, an explicit representation or semantic qualifier may participate in type identity if defined by the specification.

---

48. Type-Level Contracts

User-defined types may participate in:

requires
ensures
invariant
assume
guarantee
property

but these are owned by:

grammar/validation/
grammar/spec/

The type system consumes their semantic representation.

---

49. Refinement-Compatible Types

Where refinement types are supported, a user-defined type may be constrained by predicates.

Conceptually:

type PositiveInteger = Integer
    where value > 0;

The exact refinement syntax must come from the canonical constraint/refinement system.

The user-defined type document does not invent another predicate grammar.

---

50. Dependent Types

Where dependent types are implemented, user-defined types may depend on type-level values.

Example:

Matrix<T, Rows, Columns>

The type grammar must preserve:

T
Rows
Columns

as semantic type-level arguments.

No fixed matrix dimensions may be imposed by the grammar.

---

51. Type-Level Values

A type-level value must remain symbolic until the semantic system has enough information to evaluate or constrain it.

This is particularly important for:

- array sizes;
- tensor dimensions;
- matrix dimensions;
- resource quantities;
- quantum resource counts;
- hardware parameters.

The parser must not collapse them into a machine-dependent representation.

---

52. Quantum User-Defined Types

Quantum abstractions may be expressed as user-defined types.

Examples:

type Register<T>;
type Algorithm<Input, Output>;
type QuantumState<T>;

These remain normal user-defined types.

Quantum-specific semantic meaning belongs to the quantum subsystem.

Quantum lowering ultimately crosses the canonical:

quantum::ir

boundary.

---

53. Quantum Resource Semantics

A user-defined quantum type may participate in resource requirements.

For example, a semantic type may require:

capability("quantum.measurement")

or:

qubits >= required_qubits

The type declaration itself does not select physical qubits.

Resource feasibility belongs downstream.

---

54. No Fixed Quantum Limits

User-defined quantum types must never encode assumptions such as:

MAX_QUBITS

or any fixed maximum number of quantum resources.

The semantic model must remain open-ended.

Physical feasibility is determined by available resources.

---

55. Classical User-Defined Types

Classical structures, records, numeric abstractions, collections, graphs, tensors, models and other constructs may all be user-defined types.

They use the same type architecture.

They do not require a separate classical type system.

---

56. HDL and Hardware User-Defined Types

HDL/hardware domains may define semantic types for:

- signals;
- buses;
- channels;
- interfaces;
- protocols;
- hardware resources;
- timing abstractions;
- state machines.

However, user-defined type semantics must remain distinct from physical realization.

A source type must not silently become:

wire [31:0]

or any other fixed physical width unless the language explicitly declares that width semantically.

---

57. Hardware Scalability

Hardware-oriented types must not contain universal limits such as:

MAX_BITS
MAX_LANES
MAX_CHANNELS
MAX_DEVICES
MAX_REGISTERS

Resource availability belongs to the target environment.

---

58. AI and Data User-Defined Types

AI/data domains may define:

Dataset<T>
Tensor<T, Shape>
Model<Input, Output>
Distribution<T>
Evidence<T>
Knowledge<T>
Graph<Node, Edge>

These are ordinary user-defined types from the core type-system perspective.

Their domain semantics belong to their respective semantic subsystems.

---

59. Uncertainty Types

User-defined types may wrap:

- probability;
- confidence;
- distribution;
- belief;
- uncertainty.

For example:

type Prediction<T> = {
    value: T,
    confidence: Confidence
};

The type system represents the structure.

The uncertainty subsystem defines the meaning of the uncertainty-related types.

---

60. Knowledge Types

Knowledge structures may be represented using user-defined types.

For example:

type Fact<S, R, O> = {
    subject: S,
    relation: R,
    object: O
};

The type system remains domain-neutral.

Knowledge semantics are supplied by the appropriate semantic subsystem.

---

61. Reasoning Types

Reasoning-related abstractions may be user-defined types:

Evidence<T>
Premise<T>
Conclusion<T>
Derivation<T>
Decision<T>
Explanation<T>

These should not require universal grammar changes.

---

62. Learning Types

Learning systems may define:

Dataset<T>
Model<I, O>
Objective<T>
TrainingState<T>
Prediction<T>

The learning semantics belong to the AI/learning subsystem.

The user-defined type system supplies the generic type machinery.

---

63. Adaptation Types

Adaptive computation may represent:

Strategy<T>
Policy<T>
Adaptation<T>
Decision<T>

but type declarations do not grant adaptation authority.

Authorization, effects, capabilities and policies remain separate.

---

64. Effects

A user-defined type can appear in effectful APIs.

For example:

fn learn<T>(data: Dataset<T>) -> Model<T>

The function's effect contract belongs to the effects subsystem.

User-defined type semantics must not absorb effect semantics.

---

65. Capabilities

A type may be associated with capability requirements where the language explicitly supports capability-aware types or declarations.

For example:

type QuantumMeasurement

may have associated semantic metadata requiring:

capability("quantum.measurement")

But:

«Declaring a type must never grant a capability.»

Capabilities are authority, not type identity.

---

66. Resources

User-defined types may participate in resource requirements.

Examples include:

memory >= required_memory
qubits >= required_qubits
capability("gpu.compute")
capability("tensor.compute")

Resource expressions remain symbolic and target-independent.

The compiler determines feasibility later.

---

67. Policies

A type may be constrained by policies.

Examples:

- security policy;
- execution policy;
- serialization policy;
- adaptation policy;
- deployment policy;
- resource policy.

Policies must not be encoded as arbitrary hidden properties of the type.

The semantic policy system remains authoritative.

---

68. Provenance

Every user-defined type declaration should be capable of retaining provenance sufficient for:

- source location;
- declaration origin;
- imported module;
- generated source;
- macro expansion;
- dialect;
- library;
- compiler transformation;
- version.

This supports diagnostics, auditing, reproducibility and explanation.

---

69. Generated User-Defined Types

Macros, metaprogramming, code generation and dialects may generate user-defined types.

Generated types must enter the same declaration/type-resolution pipeline as handwritten types.

Generated code must not bypass:

- name resolution;
- type checking;
- constraints;
- effects;
- capabilities;
- resources;
- contracts;
- policies;
- provenance.

---

70. Reflection

Reflection may inspect user-defined types where permitted.

Reflection must use the canonical semantic type model.

It must not independently parse source files to reconstruct type information.

---

71. Metaprogramming

Compile-time type generation may create:

- declarations;
- generic types;
- aliases;
- constraints;
- implementations.

The generated result must be validated through the normal type pipeline.

Metaprogramming does not create a privileged type path.

---

72. FFI and ABI

A user-defined type may cross an FFI boundary.

However, source-level semantic identity must remain distinct from ABI representation.

The pipeline is:

Zamani user-defined type
        ↓
semantic type
        ↓
FFI compatibility analysis
        ↓
ABI mapping
        ↓
target representation

ABI layout does not belong in this document's type grammar.

---

73. Foreign Types

Foreign types may be represented as:

- opaque types;
- external types;
- imported declarations;
- ABI-associated types.

The semantic model must record their external origin.

Foreign type representation must not be assumed portable merely because the source type name is portable.

---

74. Serialization

User-defined types may participate in serialization.

Serialization representation is not automatically the semantic type representation.

The serialization system must define:

- schema;
- version;
- compatibility;
- encoding;
- provenance.

A serialized type must not silently redefine the semantic type.

---

75. Type Equality

The semantic type system must define distinct operations for:

identity equality
structural equality
compatibility
convertibility
assignability
subtyping
equivalence

These concepts must not be conflated.

---

76. Type Compatibility

Compatibility must be determined by semantic rules.

It must not depend on:

- current backend;
- current CPU;
- current memory size;
- current GPU;
- current QPU;
- compiler host architecture.

---

77. Type Conversion

Conversions between user-defined types must be explicit or semantically justified according to the canonical conversion rules.

The compiler must not silently reinterpret unrelated nominal types because their representations happen to match.

---

78. Newtype Safety

A nominal user-defined type should be able to provide stronger semantic separation than its underlying representation.

This is useful for:

- units;
- identifiers;
- security tokens;
- resource handles;
- quantum objects;
- physical quantities;
- protocol states.

---

79. Representation Independence

A user-defined type must permit its implementation representation to vary by target when the language semantics permit such variation.

Therefore:

source type

does not necessarily imply:

same physical layout everywhere

POCO-REAF preserves meaning, not accidental machine representation.

---

80. Determinism

Type resolution must be deterministic.

Given identical:

- source;
- language version;
- dependency graph;
- dialect set;
- semantic configuration;
- relevant policies;

the compiler must produce the same semantic type results.

---

81. Reproducibility

Type-resolution provenance should permit the compiler to explain:

why this declaration resolved to this type
why this generic argument was selected
why a bound was accepted/rejected
why an alias expanded
why an implementation was selected

---

82. Incremental Compilation

User-defined type identity must support incremental compilation.

A change to one type should invalidate only the semantic dependents affected by that change.

The implementation should use stable declaration identities rather than reparsing or re-resolving the entire program unnecessarily.

---

83. Generic Instantiation

Generic instantiation should be memoizable.

Conceptually:

GenericType<T>
       +
Argument A
       ↓
Instantiation(GenericType, A)

Equivalent semantic instantiations should be canonicalized where appropriate.

---

84. Generic Explosion Control

The compiler may enforce implementation resource limits for:

- memory;
- compilation time;
- recursion;
- instantiated types.

Those limits must not become language-level semantic ceilings.

They must be diagnostics or compilation-policy controls.

The language must remain conceptually scalable as far as available resources permit.

---

85. No Artificial Type Limits

The user-defined type system must not define:

MAX_USER_TYPES
MAX_FIELDS
MAX_VARIANTS
MAX_TYPE_PARAMETERS
MAX_GENERIC_ARGUMENTS
MAX_NESTING
MAX_TYPE_DEPTH
MAX_RECURSION
MAX_STRUCT_SIZE
MAX_ENUM_VARIANTS

as language semantics.

Only implementation-resource policies may impose practical limits.

---

86. Type Size

The semantic type model must not require every type to have a statically known physical size.

Types may be:

- dynamically sized;
- abstract;
- opaque;
- target-dependent;
- resource-dependent;
- symbolic.

Physical size is resolved where required by a backend.

---

87. Type Layout

The semantic type system may expose layout constraints only when those constraints are part of the language semantics.

Otherwise layout belongs to:

- compiler lowering;
- ABI;
- backend;
- hardware realization.

---

88. Memory Model Integration

User-defined types interact with memory semantics through the existing memory subsystem.

The type system supplies semantic structure.

The memory subsystem determines:

- ownership;
- allocation;
- lifetime;
- borrowing;
- storage class;
- addressability;
- persistence.

No competing memory model is allowed inside this file.

---

89. Linear and Affine User-Defined Types

User-defined types may participate in linear/affine semantics.

For example, a resource handle can be a nominal type whose values obey linear usage rules.

The linear/affine subsystem owns the usage analysis.

This document defines only the type identity/integration contract.

---

90. Resource Types

Resource-aware types may represent:

- handles;
- devices;
- channels;
- quantum resources;
- accelerator contexts;
- distributed resources.

The type itself does not allocate the resource.

Resource realization remains downstream.

---

91. Capability Types

Capability-bearing values may be represented by user-defined types.

However:

type identity

and:

authority

must remain conceptually distinct.

Possessing a value can carry authority only if the capability/security specification explicitly defines that behavior.

---

92. Contracts on User-Defined Types

A user-defined type may have invariants.

For example:

type Percentage
    invariant value >= 0
    invariant value <= 100;

The exact syntax belongs to the contract/refinement subsystem.

The semantic type system records the relationship.

---

93. Invariant Checking

Type invariants may be:

- statically provable;
- dynamically checked;
- conditionally discharged;
- rejected;
- preserved as runtime contracts.

The semantic contract determines which strategy applies.

---

94. Property-Based Semantics

Properties associated with user-defined types may support verification and testing.

They must remain separate from ordinary field/type syntax.

The property subsystem owns:

- property semantics;
- proof obligations;
- verification;
- test generation.

---

95. Type Provenance

A semantic user-defined type should retain provenance such as:

declared_at
module
package
source_version
generated_by
import_origin
macro_origin
dialect
compiler_transformation

where available.

---

96. Explainability

Tooling should be able to explain:

Type X resolved to declaration Y because ...

and:

Type X is incompatible with Y because ...

The explanation should consume canonical semantic information.

---

97. Diagnostics

Diagnostics must distinguish:

Syntax error

The declaration/type syntax is invalid.

Name error

The referenced type cannot be resolved.

Type error

The types are semantically incompatible.

Constraint error

A type constraint or bound is unsatisfied.

Capability error

A required capability is unavailable.

Resource error

A required resource is unavailable.

Policy error

A policy forbids the requested realization.

Target feasibility error

The selected target cannot realize the otherwise valid semantic program.

These errors must not be collapsed into one generic "type error."

---

98. Target Infeasibility

A valid user-defined type must not become invalid merely because one target cannot represent it.

For example:

Tensor<VeryLargeType>

may be semantically valid while a particular target lacks sufficient resources.

The correct result is:

semantic type valid
target realization infeasible

not:

source type invalid

unless the language semantics explicitly require target-specific behavior.

---

99. POCO-REAF

The fundamental portability model is:

                 SAME SOURCE
                      │
       ┌──────────────┼──────────────┐
       ▼              ▼              ▼
     tiny            CPU            GPU
       │              │              │
       ▼              ▼              ▼
     FPGA            ASIC           QPU
       │              │              │
       └──────────────┼──────────────┘
                      ▼
                 accelerator
                      │
                      ▼
                     HPC
                      │
                      ▼
                   cluster
                      │
                      ▼
                distributed
                      │
                      ▼
               future target

The user-defined type remains semantically stable.

Its realization may change.

---

100. "Atom to Everywhere"

The type system must permit the same semantic type model to describe programs ranging from extremely small computations to extremely large systems.

This means the language cannot assume:

- fixed memory;
- fixed register width;
- fixed address width;
- fixed processor count;
- fixed accelerator count;
- fixed quantum capacity;
- fixed tensor rank;
- fixed network size.

Available resources determine feasibility.

---

101. Resource Abstraction

User-defined types may participate in resource requirements such as:

requires memory >= required_memory;
requires qubits >= required_qubits;
requires capability("gpu.compute");
requires capability("tensor.compute");
requires capability("quantum.measurement");
requires topology(required_topology);

These requirements belong to the resource/capability system.

They are not machine-size constants.

---

102. Preferences

Type-related realization may be influenced by:

prefer ...

but preferences do not redefine type semantics.

A preference is not a requirement.

---

103. Constraints

A type may impose semantic constraints.

A target may impose realization constraints.

These must remain distinct.

type constraint
≠
resource constraint
≠
hardware constraint

---

104. Policies

Policies may restrict use or realization of a type.

For example:

- security policy;
- sandbox policy;
- deployment policy;
- data policy;
- adaptation policy.

Policies do not become hidden type fields.

---

105. Provenance and Generated Types

If a user-defined type originates from generated code, the semantic model should preserve its origin.

For example:

generated_by = macro
generated_by = dialect
generated_by = schema compiler
generated_by = metaprogram

This is especially important for diagnostics and reproducibility.

---

106. Dialects

Dialects may introduce additional user-defined types.

A dialect must register them through the canonical declaration/type mechanisms.

A dialect must not:

- replace "typeExpression";
- create a parallel AST;
- create a parallel type checker;
- bypass semantic validation.

---

107. Application Domains

Application-specific abstractions should be represented as libraries or dialects.

Examples:

VisionTensor
RobotPose
PaymentAmount
LegalDocument
BlockchainBlock
VRScene
SentimentScore

must not require universal core type keywords.

The universal type system remains generic.

---

108. Quantum-Classical Hybrid Types

A user-defined type may combine classical and quantum semantic components.

For example:

struct HybridState<C, Q> {
    classical: C;
    quantum: Q;
}

The hybrid semantic subsystem determines how the components interact.

Quantum operations ultimately lower through:

quantum::ir

Classical computation lowers through canonical classical IR.

---

109. HDL-Software Co-Design

User-defined types may represent shared semantic structures between software and hardware.

For example:

struct Packet {
    header: Header;
    payload: Payload;
}

The source-level type remains semantic.

Hardware layout is selected later when lowering into HDL/hardware representation.

---

110. Distributed Types

Distributed applications may use user-defined message types.

For example:

struct Message<T> {
    id: Identifier;
    payload: T;
}

The distributed subsystem owns:

- serialization;
- transport;
- routing;
- consistency;
- failure handling.

The type system owns only type meaning.

---

111. Networking

Network payload types should use ordinary user-defined types.

Network representation must be governed by the interoperability/serialization subsystem.

No network-specific type grammar is required merely to define a payload structure.

---

112. Deterministic Type Resolution

Type resolution must not depend on:

- hash-map iteration order;
- filesystem traversal order;
- thread scheduling;
- host architecture;
- backend availability.

Where unordered collections are used internally, canonical ordering must be applied whenever it affects observable compiler output.

---

113. Parallel Compilation

Type resolution may be parallelized.

Parallelism must not change semantic results.

The semantic model must therefore be deterministic regardless of compilation scheduling.

---

114. Concurrency Safety

The Rust implementation must use safe concurrency mechanisms.

Production implementation must not use "unsafe".

Recommended principles include:

- ownership;
- borrowing;
- immutable shared data;
- "Arc";
- "Mutex";
- "RwLock";
- channels;
- deterministic work queues;
- arena/index-based structures where appropriate.

The exact implementation belongs to the compiler subsystem.

---

115. Rust Safety Requirement

Production Rust implementation must support:

#![forbid(unsafe_code)]

No type-resolution path may depend on:

unsafe

FFI boundaries must use safe wrappers and explicit validation.

---

116. Rust Version

The implementation baseline is:

Rust 1.97 or later
edition 2021

Newer stable Rust features may be used only where repository compatibility policy permits.

The type grammar itself must remain independent of the implementation language.

---

117. AST Contract

Every user-defined type declaration must map into the existing domain-neutral AST architecture.

The AST should preserve sufficient information for:

- declaration identity;
- name;
- namespace;
- generic parameters;
- fields/variants;
- visibility;
- attributes;
- constraints;
- source spans;
- documentation metadata;
- provenance.

The exact AST type names are determined by the existing AST subsystem.

This file must not invent a competing AST hierarchy.

---

118. AST Stability

The AST should preserve semantic information rather than prematurely lowering it into:

- machine widths;
- physical addresses;
- register layouts;
- backend-specific types.

---

119. Semantic Type Contract

After AST construction:

UserDefinedTypeDecl
        ↓
name resolution
        ↓
generic resolution
        ↓
constraint resolution
        ↓
canonical semantic type

The semantic type must distinguish at minimum:

declaration identity
type parameters
type arguments
kind/category
members
constraints
associated information
attributes
provenance

where applicable.

---

120. Canonical Type Representation

The repository should maintain one canonical semantic representation for user-defined types.

Do not create separate:

AIUserType
QuantumUserType
HDLUserType
ClassicalUserType
HardwareUserType

representations unless they are semantic extensions of the same canonical model.

---

121. Domain Extensions

A domain may attach semantic information to the canonical type.

Conceptually:

CanonicalType
   +
QuantumMetadata

or:

CanonicalType
   +
HardwareMetadata

rather than creating a second type universe.

---

122. IR Contract

User-defined types must not directly select an IR.

Instead:

semantic type
      ↓
semantic operation/declaration
      ↓
IR lowering

determines whether the result belongs in:

Classical IR
quantum::ir
HDL/domain IR

or another canonical downstream representation.

---

123. Classical IR

Classical user-defined types may lower into canonical classical representations.

The lowering may select different physical representations based on target constraints.

Semantic identity remains independent of that choice.

---

124. quantum::ir

Quantum user-defined types that participate in executable quantum semantics must eventually cross the canonical:

quantum::ir

boundary.

No alternate quantum IR should be created inside user-defined types.

---

125. HDL Boundary

Hardware-oriented user-defined types may lower into the existing HDL semantic pipeline.

This may involve:

signals
interfaces
protocols
timing
storage
hardware resources

without requiring user-defined types to encode physical implementation.

---

126. Backend Boundary

Backends may determine:

- layout;
- representation;
- alignment;
- calling convention;
- vectorization;
- device mapping;
- memory placement.

These are downstream decisions.

---

127. Optimization

Optimizations may:

- inline;
- specialize;
- eliminate aliases;
- reorder fields where semantically permitted;
- flatten representations;
- scalarize;
- vectorize;
- distribute;
- map to accelerators.

They must preserve user-defined type semantics.

---

128. Type Erasure

If a backend erases type information, the compiler must retain enough semantic metadata for:

- diagnostics;
- provenance;
- debugging;
- reflection where required;
- contracts;
- runtime checks where required.

Type erasure is an implementation transformation, not semantic deletion.

---

129. Type Metadata Across Optimization

Optimization passes must preserve metadata that remains semantically observable.

Metadata that is not observable may be removed according to compiler policy.

The distinction must be explicit.

---

130. ABI

A user-defined type may acquire a target ABI representation only after semantic analysis.

The ABI layer may determine:

- calling convention;
- parameter passing;
- return representation;
- layout;
- alignment.

It must not redefine the language type.

---

131. Runtime

Runtime systems may need metadata for:

- dynamic dispatch;
- reflection;
- serialization;
- type identification;
- memory management.

Runtime metadata must derive from canonical semantic information.

---

132. Type Versioning

A user-defined type may evolve.

Compatibility tooling should distinguish:

source compatibility
binary compatibility
semantic compatibility
serialization compatibility
API compatibility

A change in one category does not automatically imply a change in all others.

---

133. Versioned Type Identity

Type identity may need version information for:

- separately compiled libraries;
- serialized schemas;
- package dependencies;
- ABI boundaries.

The exact versioning mechanism belongs to compatibility/package tooling.

---

134. Deprecation

A user-defined type may be deprecated.

Deprecation must be represented through the canonical compatibility/deprecation mechanism.

The grammar must continue to parse deprecated types while the compatibility policy determines diagnostics.

---

135. Migration

A renamed or replaced type should be capable of a migration path.

For example:

OldType
   ↓
compatibility alias
   ↓
NewType

where semantics permit.

---

136. Error Recovery

ANTLR error recovery must not produce semantic declarations that appear valid when required type information is missing.

The parser may recover syntactically.

Semantic validation must still reject incomplete declarations.

---

137. Source Spans

User-defined type declarations and their members must preserve source spans.

Source spans support:

- diagnostics;
- IDEs;
- refactoring;
- provenance;
- explanations;
- generated-source mapping.

---

138. Documentation

Documentation comments associated with user-defined types should be preserved through AST metadata where tooling supports it.

Documentation must not alter type identity.

---

139. Reflection Contract

Reflection must expose canonical semantic information.

It should be possible, where authorized, to inspect:

- type name;
- qualified name;
- generic parameters;
- fields;
- variants;
- constraints;
- attributes;
- provenance.

Reflection authority remains separately controlled.

---

140. Security

A user-defined type must not itself grant:

- native execution;
- filesystem access;
- network access;
- hardware access;
- privileged operations;
- adaptation authority;
- reflection authority.

Security capabilities are independently checked.

---

141. Sandbox

Inside a sandbox, user-defined types remain ordinary semantic types.

A sandbox may restrict operations involving those types.

For example:

network capability unavailable

does not make a network-related data type syntactically invalid.

---

142. Learning

Learning systems may create new model types or specialized user-defined types.

Generated type declarations must undergo normal validation.

Learning cannot bypass type checking.

---

143. Adaptation

Runtime adaptation may select a different representation or implementation only when explicitly permitted.

It must not silently redefine the source type.

Adaptation requires the applicable:

- policy;
- capability;
- effect;
- resource;
- authorization;
- provenance.

---

144. Simulation

A user-defined type may be used in simulation.

Simulation must preserve semantic type behavior.

Simulation is an execution strategy, not a different type system.

---

145. Reproducibility

A build involving user-defined types should be reproducible when given the same:

- source;
- dependencies;
- language version;
- dialect versions;
- compiler version/policy;
- relevant configuration.

---

146. Cross-Target Stability

A user-defined type must preserve semantic meaning when compiled for:

embedded
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
distributed
cloud
future targets

Physical realization may differ.

---

147. Target Capability Negotiation

Target capabilities must be evaluated after semantic type resolution.

The sequence is:

user-defined type
       ↓
semantic validation
       ↓
program requirements
       ↓
target capabilities
       ↓
resource negotiation
       ↓
realization

The type grammar must not perform capability discovery.

---

148. Feasibility

A program may have:

valid type semantics

but:

target infeasibility

These are different outcomes.

This distinction is mandatory for POCO-REAF.

---

149. Fallback

If multiple realizations exist, the compiler may select a valid fallback.

Fallback must preserve semantic meaning.

It must not silently substitute an approximation unless approximation is explicitly part of the semantic contract.

---

150. Approximate Types

If a domain supports approximate computation, approximation must be explicitly represented in the semantic model.

It must not be introduced merely because a target lacks resources.

---

151. Type and Deterministic Execution

Type semantics must not depend on nondeterministic runtime behavior unless the type explicitly represents such behavior.

Examples of nondeterministic behavior belong to the effect/execution model.

---

152. Type and Contracts

Contracts may establish guarantees about values of a user-defined type.

The type system must retain enough information for downstream contract checking.

---

153. Type and Evidence

Evidence may justify a type property.

For example:

property MatrixIsSquare

may be supported by compile-time evidence.

Evidence/provenance belongs to the corresponding subsystem.

---

154. Type and Explanation

Compiler tooling should be able to explain type compatibility and resolution.

Example:

why is A compatible with B?
why was implementation X selected?
why was generic parameter T inferred as Y?

---

155. Type Inference

User-defined types may participate in type inference.

Inference must operate on semantic type expressions.

The grammar must not perform inference.

---

156. Unification

Generic user-defined types may participate in unification.

The unifier should operate on canonical semantic types.

Recursive unification must terminate according to semantic rules.

---

157. Constraint Solving

Type constraints may arise from:

- declarations;
- expressions;
- generic bounds;
- traits;
- associated types;
- contracts;
- dependent values.

They must enter the canonical constraint solver.

---

158. Constraint Diagnostics

When a constraint fails, diagnostics should identify:

type
constraint
origin
dependent declaration
available evidence
possible correction

where available.

---

159. Type Classes

Type-class resolution involving user-defined types must be deterministic.

Ambiguous implementations must be rejected according to the canonical resolution rules.

---

160. Coherence

Where trait/type-class coherence is part of Zamani's semantics, user-defined types must obey the global coherence rules.

No domain may introduce a private dispatch interpretation that conflicts with canonical type-class resolution.

---

161. Higher-Kinded User Types

Where higher-kinded types are supported, user-defined generic types may themselves take type constructors.

Example conceptually:

Container<F, T>

The generic/type-kinding subsystem owns the semantics.

---

162. Existential Types

Where supported, user-defined types may hide implementation type parameters behind existential interfaces.

The existential type subsystem owns the semantic rules.

---

163. Path-Dependent Types

Where supported, associated/path-dependent type references must use the canonical path-dependent type subsystem.

User-defined types only establish the declaration members from which such types can be derived.

---

164. Type Families

Where supported, type families may compute associated types from user-defined types.

The type-family subsystem owns evaluation and normalization.

---

165. Type Providers

Where supported, externally generated type declarations must enter the normal declaration and semantic pipeline.

A provider must not inject unchecked types directly into the compiler.

---

166. Domain-Specific Type Extensions

Future domains must be able to define types without modifying this document every time a new domain appears.

The extension model is:

domain
  ↓
declaration
  ↓
canonical type expression
  ↓
semantic extension metadata
  ↓
domain semantic analysis

---

167. No Application Keyword Explosion

Application concepts should remain types, libraries, dialects or semantic abstractions.

The universal language must not gain a keyword for every possible:

- application;
- industry;
- algorithm;
- model;
- hardware vendor;
- protocol;
- business process.

---

168. Type Registry

If the compiler maintains a registry of known semantic type constructors, the registry must be extensible.

It must not require a fixed enumeration of every future type.

Registry entries should carry stable identities and metadata.

---

169. Type Constructor Identity

A generic constructor such as:

Vector

must have a stable semantic identity independent of a particular instantiation:

Vector<Real>
Vector<Integer>
Vector<Model>

Each instantiation is resolved through the canonical generic system.

---

170. Canonicalization

Equivalent semantic type expressions should be canonicalized where required.

For example:

alias A = B

may normalize to the same semantic target where aliases are transparent.

Canonicalization must not erase nominal distinctions.

---

171. Type Normalization

Normalization may:

- expand transparent aliases;
- resolve associated types;
- normalize generic applications;
- simplify equivalent constraints;
- resolve type families.

Normalization must preserve semantic identity and provenance.

---

172. Type Caching

The implementation should cache resolved semantic types where safe.

Cache keys must include every semantic input that affects resolution.

Caches must not introduce nondeterministic behavior.

---

173. Memory Scalability

The implementation must be able to represent large type graphs subject to available compiler resources.

It must not use fixed-size arrays or fixed-count registries for semantic type declarations.

---

174. Deep Type Graphs

Nested user-defined types must be represented iteratively or through safe recursion strategies where necessary.

The implementation must avoid stack-overflow assumptions for arbitrarily deep valid source programs.

Practical compiler limits may exist as resource controls, not language semantics.

---

175. Large Declaration Sets

Large projects may contain extremely many user-defined types.

The compiler architecture should support:

- indexed lookup;
- incremental invalidation;
- lazy resolution;
- dependency graphs;
- deterministic parallel analysis.

No semantic maximum declaration count is defined.

---

176. Generic Graph Scalability

Generic instantiations can form large graphs.

The compiler should use:

- memoization;
- canonical instantiation identities;
- dependency tracking;
- incremental invalidation.

No language-level generic-count ceiling is allowed.

---

177. Recursive Graph Scalability

Recursive types must be represented as graphs rather than repeatedly expanded trees.

This prevents exponential or infinite representation.

---

178. Type Environment

The semantic implementation should maintain a type/declaration environment capable of representing:

modules
namespaces
declarations
generic parameters
imports
aliases
visibility
traits
associated types
constraints
provenance

The exact Rust data structures belong to the semantic subsystem.

---

179. Safe Rust Data Model

The implementation should prefer stable identifiers such as:

DeclarationId
TypeId
ModuleId
GenericParameterId
FieldId
VariantId
ConstraintId

rather than raw pointers.

This supports safe Rust and stable graph relationships.

---

180. No Unsafe Graph Representation

Semantic type graphs must not depend on:

unsafe

pointer manipulation.

Use safe indexed arenas, immutable structures, reference-counted graphs or other safe abstractions as appropriate.

---

181. Serialization of Type Metadata

If type metadata is serialized, the schema must include:

- schema version;
- language version;
- type identity;
- declaration identity;
- generic information;
- relevant metadata;
- provenance where required.

Unknown metadata must follow compatibility rules.

---

182. Type Metadata Compatibility

A consumer must not silently reinterpret incompatible type metadata.

It must:

accept
adapt
reject

according to the compatibility contract.

---

183. Source Compatibility

Adding a new unrelated user-defined type should not break existing programs.

Changing an existing type may be breaking depending on:

- field changes;
- variant changes;
- generic changes;
- constraints;
- visibility;
- nominal identity;
- serialization schema.

---

184. Binary Compatibility

Binary compatibility belongs to the ABI/compiler compatibility subsystem.

Changing a source type does not automatically mean binary incompatibility, and vice versa.

---

185. Serialization Compatibility

Adding a field, variant or changing representation may affect serialized data.

Serialization compatibility must be evaluated separately from semantic type compatibility.

---

186. Version Migration

Type migration should support explicit mappings where feasible:

OldType
    ↓
migration
    ↓
NewType

The migration must preserve or explicitly transform semantics.

---

187. Testing Contract

Every user-defined type feature must have:

1. lexical tests;
2. parser tests;
3. AST tests;
4. name-resolution tests;
5. semantic tests;
6. generic tests;
7. constraint tests;
8. negative tests;
9. boundary tests;
10. scalability tests;
11. deterministic tests;
12. compatibility tests;
13. provenance tests where applicable;
14. cross-domain tests.

---

188. Positive Tests

At minimum:

struct Point { ... }

record Person { ... }

enum State { ... }

union Value { ... }

type UserId = Integer;

type Pair<T, U> = ...;

type Node<T> = ...;

must be represented through the canonical architecture.

---

189. Generic Tests

Test:

Pair<Integer, String>
Pair<Vec<Integer>, Option<String>>
Map<String, Result<Value, Error>>

and deeply nested generic structures.

Tests must not encode arbitrary maximum generic depth.

---

190. Recursive Tests

Test valid recursive forms:

Node
Tree
Graph
List

including generic recursion.

Also test invalid recursive declarations according to the semantic rules.

---

191. Nominal Tests

Verify that:

type A = ...
type B = ...

remain distinct when nominal semantics require distinct identities even when their structures match.

---

192. Alias Tests

Verify:

alias compatibility
alias normalization
alias diagnostics
alias provenance
alias cycles

---

193. Namespace Tests

Test:

a::Thing
b::Thing
a::nested::Thing

and verify identity resolution.

---

194. Import Tests

Verify user-defined types across:

- modules;
- packages;
- dependency boundaries;
- separately compiled units.

---

195. Visibility Tests

Test all supported visibility modes.

A private type must not become accessible merely because its name is known.

---

196. Trait/Type-Class Tests

Test:

- implementation selection;
- ambiguous implementations;
- associated types;
- generic bounds;
- coherence;
- specialization where supported.

---

197. Constraint Tests

Test:

satisfied constraint
unsatisfied constraint
dependent constraint
generic constraint
associated constraint

---

198. Contract Tests

Test type invariants and refinement-compatible types.

Verify that contracts do not bypass ordinary type checking.

---

199. Resource Tests

Test that a valid semantic type remains valid when a target lacks resources.

Expected distinction:

type valid
resource realization unavailable

rather than:

type invalid

---

200. Capability Tests

Verify:

type declaration
≠
capability grant

and ensure required capabilities are checked downstream.

---

201. Effect Tests

Verify user-defined types used in effectful APIs.

Ensure effect information is preserved through:

- generic instantiation;
- specialization;
- optimization;
- lowering.

---

202. Provenance Tests

Verify that diagnostics can identify the origin of:

- declarations;
- generated types;
- aliases;
- imported types;
- macro-generated types.

---

203. Quantum Tests

Test user-defined types participating in:

- quantum state abstractions;
- quantum resources;
- hybrid computation;
- measurement results;
- quantum-classical data.

Verify eventual convergence into "quantum::ir".

---

204. HDL Tests

Test user-defined types used in:

- signals;
- interfaces;
- hardware structures;
- simulation;
- verification;
- synthesis intent.

Verify no fixed hardware capacity is introduced.

---

205. AI/Data Tests

Test:

Tensor<T>
Dataset<T>
Model<I,O>
Evidence<T>
Prediction<T>
Graph<N,E>

as user-defined types without requiring new universal keywords.

---

206. Distributed Tests

Test user-defined message types and verify:

type
→ serialization
→ transport

without conflating type semantics with network realization.

---

207. FFI Tests

Test:

opaque types
foreign types
ABI-compatible structures
unsupported layouts

and ensure semantic types remain distinct from ABI representations.

---

208. Determinism Tests

Compile identical programs repeatedly and verify identical semantic type results.

Run with:

- different compilation thread counts;
- different target order;
- different filesystem ordering;
- different cache states.

The semantic result must remain deterministic.

---

209. Scalability Tests

The test suite must include generated type graphs of increasing size.

The tests must verify that the compiler:

- remains correct;
- does not use fixed semantic limits;
- scales according to available resources;
- reports resource exhaustion clearly when resources are genuinely exhausted.

---

210. Boundary Tests

Cross-domain examples must combine:

user-defined types
+
generics
+
effects
+
capabilities
+
resources
+
contracts
+
policies
+
provenance
+
classical computation
+
quantum computation
+
HDL
+
distributed execution

where applicable.

---

211. POCO-REAF Integration Test

A mandatory integration test should define one semantic user-defined type used across:

tiny/classical realization
GPU realization
FPGA realization
quantum-hybrid realization
HPC realization
distributed realization

The source declaration remains unchanged.

Only target realization changes.

---

212. Example POCO-REAF Type

Conceptually:

type WorkItem<T> = {
    payload: T,
    metadata: Metadata
};

The type does not specify:

CPU
GPU
FPGA
QPU
node count
memory size
register width

Those are resolved downstream.

---

213. Integration With "grammar/types/types.g4"

"types.g4" must remain the single universal type-expression composition point.

It consumes the canonical named/generic representation needed to reference user-defined types.

It must not duplicate user-defined declaration syntax.

Required relationship:

types.g4
   ├── primitive
   ├── named
   ├── generic
   ├── composite
   ├── function
   ├── reference
   ├── advanced
   └── domain extensions

---

214. Integration With "grammar/types/named.g4"

"named.g4" owns references such as:

User
module::User
package::module::User

The semantic resolver resolves those references to declaration identities.

---

215. Integration With "grammar/types/generic.g4"

"generic.g4" owns:

<T>
<A, B>

and generic application structure.

User-defined types consume this mechanism.

No user-defined type may implement its own generic argument syntax.

---

216. Integration With "grammar/declarations/types.g4"

If "declarations/types.g4" remains a compatibility or composition surface, its ownership must be explicitly reconciled with:

structs.g4
records.g4
enums.g4
unions.g4
aliases.g4

There must be one authoritative owner for each declaration production.

Duplicate declaration rules must be removed, delegated or marked compatibility-only.

---

217. Integration With "grammar/declarations/structs.g4"

"structs.g4" owns struct declaration syntax.

It must export a declaration representation containing sufficient information for semantic user-defined type construction.

It must not construct a backend layout.

---

218. Integration With "grammar/declarations/records.g4"

"records.g4" owns record declaration syntax.

Its field types must consume canonical "typeExpression".

It must not define another type-expression rule.

---

219. Integration With "grammar/declarations/enums.g4"

"enums.g4" owns enum declaration syntax.

Variant payloads must consume canonical type expressions.

---

220. Integration With "grammar/declarations/unions.g4"

"unions.g4" owns union declaration syntax.

Payload types must consume canonical type expressions.

---

221. Integration With "grammar/declarations/aliases.g4"

"aliases.g4" owns alias declaration syntax.

The aliased type must be parsed using canonical "typeExpression".

Alias normalization belongs to semantic analysis.

---

222. Integration With "grammar/declarations/traits.g4"

Trait/type-class declarations may reference and constrain user-defined types.

User-defined types may implement those traits.

There must be no circular grammar ownership.

---

223. Integration With "grammar/types/bounds.g4"

Bounds apply to generic parameters and constrained types.

User-defined type declarations consume the canonical bound representation.

---

224. Integration With "grammar/types/constraints.g4"

Constraints are semantic obligations.

The user-defined type system records the applicable constraints but does not implement constraint solving.

---

225. Integration With "grammar/types/associated.g4"

Associated types resolve against the canonical declaration/type model.

No user-defined type-specific associated-type syntax is allowed.

---

226. Integration With "grammar/types/type-class.g4"

User-defined types participate in canonical type-class/trait resolution.

The type class subsystem owns dispatch semantics.

---

227. Integration With "grammar/types/linear.g4"

A user-defined type can be linear.

The linear type subsystem determines usage requirements.

---

228. Integration With "grammar/types/affine.g4"

A user-defined type can be affine.

The affine subsystem determines consumption rules.

---

229. Integration With "grammar/types/dependent.g4"

Dependent user-defined types may depend on compile-time values.

The dependent type subsystem owns the value/type dependency semantics.

---

230. Integration With "grammar/types/existential-types.g4"

Existential user-defined abstractions use the canonical existential mechanism.

---

231. Integration With "grammar/types/higher-kinded-types.g4"

Higher-kinded user-defined types use the canonical kind system.

---

232. Integration With "grammar/types/path-dependent-types.g4"

Path-dependent references use the canonical path-dependent type rules.

---

233. Integration With "grammar/types/type-families.g4"

Type-family computation operates over canonical user-defined type identities.

---

234. Integration With "grammar/types/type-providers.g4"

Generated/provided types must enter the ordinary declaration/type-resolution pipeline.

---

235. Integration With Effects

A user-defined type may appear in APIs carrying effects.

The effect system remains independent.

Required relationship:

Type
 +
Effect
 +
Capability
 +
Resource
 +
Policy

must converge in semantic analysis.

---

236. Integration With Resources

Resource requirements must remain symbolic.

Examples:

requires memory >= required_memory;
requires qubits >= required_qubits;
requires capability("tensor.compute");

The type does not determine the target.

---

237. Integration With Contracts

Type invariants and contracts must be represented using canonical contract semantics.

---

238. Integration With Policies

Policies may restrict type use or realization.

Policies must not mutate type identity.

---

239. Integration With Provenance

All significant type transformations should preserve provenance.

Example:

source declaration
   ↓
alias
   ↓
generic substitution
   ↓
normalized type
   ↓
lowered representation

---

240. Integration With Security

Security-sensitive user-defined types may represent:

- tokens;
- handles;
- identities;
- credentials;
- protected resources.

Security semantics belong to the security subsystem.

A type does not automatically confer privilege.

---

241. Integration With Compatibility

Compatibility analysis must inspect changes to:

- type identity;
- generic parameters;
- fields;
- variants;
- constraints;
- visibility;
- associated types;
- representation contracts.

---

242. Integration With Modules

The module subsystem owns:

- imports;
- exports;
- module identity;
- visibility.

The type subsystem consumes the resulting environment.

---

243. Integration With Packages

Package identity and dependency resolution are outside the type grammar.

The semantic type resolver receives a resolved package/module environment.

---

244. Integration With Macros

Macros can generate type declarations.

Expanded declarations must be treated as normal declarations.

---

245. Integration With Metaprogramming

Type-level computation must produce canonical semantic types.

Generated type information cannot bypass normal validation.

---

246. Integration With Interoperability

Schema-derived types from JSON/XML/SQL or other formats should be generated as normal user-defined types.

The interchange format remains a dialect/interoperability concern.

---

247. SQL Integration

A SQL schema may produce:

record Customer { ... }

but SQL grammar does not become part of the core type grammar.

---

248. JSON/XML Integration

JSON/XML schemas may generate user-defined types.

Generated types still follow:

declaration
→ AST
→ semantic type

---

249. Neural-Symbolic Integration

A user-defined type can represent symbolic structures used alongside learned models.

The type system does not distinguish "neural" and "symbolic" at the universal grammar level.

---

250. Evidence Integration

Evidence types may carry proof or confidence information.

The evidence subsystem determines the semantics.

---

251. Decision Integration

Decision records may use user-defined types.

Decision semantics and provenance remain outside the basic type declaration.

---

252. Adaptation Integration

Adaptive systems may instantiate or select user-defined types.

Adaptation must not mutate the type system at runtime.

Runtime adaptation changes program state/strategy according to policy; it does not redefine the language's semantic type universe.

---

253. Dynamic Type Generation

If runtime dynamic type generation is ever supported, generated types must be represented through an explicit runtime type mechanism.

They must not mutate compile-time type declarations invisibly.

---

254. Compile-Time Type Generation

Compile-time generation must produce ordinary AST/declaration nodes before semantic analysis completes.

---

255. Runtime Reflection

Runtime reflection must consume compiled type metadata rather than source grammar.

---

256. No Hidden Semantic Mutation

A compiler, runtime, macro, dialect or adaptive execution mechanism must not silently modify the meaning of an existing user-defined type.

Changes require explicit versioning or a new type identity.

---

257. Type Evolution

If a type evolves through a program transformation, the compiler must distinguish:

same type

from:

new type derived from old type

---

258. Type Derivation

Derived types may be generated from:

- schema;
- generic instantiation;
- macro expansion;
- compiler transformation;
- domain dialect.

The derivation should be represented in provenance.

---

259. Type Name Hygiene

Generated declarations must use hygienic naming rules where macros/metaprogramming require them.

Generated names must not accidentally capture unrelated user declarations.

---

260. Source Mapping

Generated user-defined types must retain source mapping back to their generating construct when possible.

---

261. Diagnostics for Generated Types

Errors involving generated types should identify both:

generated type

and:

generation origin

where possible.

---

262. Type Documentation Generation

Documentation tools should be able to enumerate user-defined types from semantic metadata.

They must not parse ".g4" files.

---

263. IDE Integration

IDE tooling should obtain type information from the canonical semantic model.

Supported operations should include:

- go to type;
- find implementations;
- find fields;
- inspect generic parameters;
- show constraints;
- show provenance;
- rename;
- refactor;
- show compatibility.

---

264. Language Server

The language server must use the same semantic type resolver as the compiler where practical.

It must not implement a second incompatible type checker.

---

265. Build-System Integration

Build tooling must invalidate affected type dependents when declarations change.

---

266. Cache Integration

Semantic type caches must be invalidated based on semantic dependency changes.

File timestamps alone are insufficient when generated or dependency-provided types are involved.

---

267. Compiler Pipeline Integration

The complete integration is:

Zamani source
       ↓
canonical lexer
       ↓
ANTLR parser
       ↓
declaration AST
       ↓
type-expression AST
       ↓
symbol resolution
       ↓
user-defined type resolution
       ↓
generic/bound/constraint solving
       ↓
type checking
       ↓
effect checking
       ↓
capability checking
       ↓
resource checking
       ↓
contract checking
       ↓
policy checking
       ↓
provenance
       ↓
canonical semantic model
       ↓
Classical IR / quantum::ir / domain IR
       ↓
optimization
       ↓
lowering
       ↓
routing
       ↓
scheduling
       ↓
resilience / recovery
       ↓
ZQN
       ↓
HAL
       ↓
target

---

268. Canonical AST Requirement

There must be exactly one semantic AST authority.

Do not create:

UserTypeAST
QuantumUserTypeAST
AIUserTypeAST
HardwareUserTypeAST

as competing roots.

Domain-specific information must extend the canonical model.

---

269. Canonical Semantic Requirement

There must be exactly one canonical semantic type representation.

Different domains may attach semantic information.

They must not create incompatible parallel type systems.

---

270. Canonical IR Requirement

User-defined types do not create an IR.

Their semantics are consumed by existing IR generation.

---

271. Optimization Invariant

Any optimization involving user-defined types must preserve:

- type identity;
- observable invariants;
- contracts;
- effects;
- capability semantics;
- resource requirements;
- provenance where required.

---

272. Lowering Invariant

Lowering may change representation.

It must not change semantic meaning.

---

273. Routing Invariant

Routing is not a type-system concern.

Quantum routing, network routing and hardware routing occur downstream.

---

274. Scheduling Invariant

Scheduling must not redefine type semantics.

---

275. Resilience Invariant

Recovery/fallback must preserve the semantic contract of user-defined types.

---

276. QEC Boundary

Quantum error correction is not part of user-defined type syntax.

Quantum semantic types may carry information needed by QEC, but QEC itself belongs downstream.

---

277. ZQN Boundary

ZQN consumes lowered semantic operations and associated metadata.

User-defined types do not directly emit ZQN.

---

278. HAL Boundary

HAL maps target-independent semantics to actual target capabilities.

User-defined types remain target-independent until appropriate lowering.

---

279. Production Grammar Requirements

The eventual declaration grammar must:

- use canonical tokens;
- avoid duplicated lexical rules;
- avoid embedded target logic;
- avoid embedded Rust actions;
- avoid physical limits;
- avoid hard-coded domain enumerations;
- preserve source spans;
- expose canonical declaration structures;
- remain composable with the canonical type grammar.

---

280. ANTLR Requirements

ANTLR grammar modules must maintain an acyclic import/delegation architecture.

Every imported rule must have one authoritative owner.

ANTLR's grammar composition mechanism should be used for syntax composition, not semantic resolution.

ANTLR itself supports grammar imports and rule ownership through grammar composition; Zamani's architecture should therefore preserve one clear owner per rule rather than duplicating equivalent alternatives.

---

281. Lexer Requirements

No user-defined type requires a new keyword merely because programmers can declare a new type.

The type name itself should normally be an identifier.

For example:

Tensor
RobotPose
PaymentAmount
QuantumState

remain identifiers.

This is essential for extensibility.

---

282. Keyword Requirements

Only language-level constructs that genuinely require reserved syntax should consume canonical keywords.

User-defined type names must not become global lexer vocabulary.

---

283. No Hard-Coded Domain List

The user-defined type system must not contain a list such as:

allowedTypeDomains = [
    classical,
    quantum,
    hdl,
    ai,
    robotics,
    ...
]

as a semantic limitation.

Domains are extensible.

---

284. No Hard-Coded Hardware List

Similarly, types must not enumerate:

CPU
GPU
FPGA
ASIC
QPU

as the complete set of possible targets.

Those are target examples, not the universal universe.

---

285. Future Hardware

A new computational substrate must be able to consume existing semantic types without requiring the type system to be redesigned.

---

286. Future Domains

A future domain should be able to define user types through:

library
dialect
extension
semantic plugin

using the canonical type architecture.

---

287. Compatibility Classification

A user-defined type-related file may be:

STABLE
EXPERIMENTAL
DEPRECATED
HISTORICAL
COMPATIBILITY

but there must be only one canonical semantic owner.

---

288. Duplicate File Resolution

The repository currently contains overlapping type files.

Before declaring the subsystem production-ready, each must be classified.

Examples include:

map.g4
map-types.g4

option.g4
option-types.g4
optional.g4

associated.g4
associated types.g4

constraints.g4
type-constraints.g4

The classification must be recorded in "grammar/types/README.md".

---

289. Compatibility Files

A compatibility file may remain temporarily.

It must not define a competing semantic grammar.

It should either:

- delegate;
- document migration;
- preserve legacy syntax;
- expose compatibility metadata.

---

290. Repository Hygiene

Malformed or accidental files under "grammar/types/" must not be treated as language features.

In particular, any path containing accidental embedded comment text or an invalid filename must be removed or corrected as repository hygiene before production release.

---

291. Documentation Synchronization

This document must remain synchronized with:

grammar/types/README.md
grammar/spec/type-system.md
grammar/spec/semantics.md
grammar/spec/portability.md
grammar/spec/compatibility.md
grammar/spec/resources.md
grammar/spec/effects.md
grammar/spec/policies.md

The synchronization rule is contractual rather than duplicative.

---

292. Specification Authority

If this file conflicts with:

grammar/specification/

the normative specification authority must be resolved explicitly.

No implementation should silently choose whichever document is convenient.

---

293. Machine Contract Authority

"grammar/spec/" provides machine-oriented contracts.

This file provides the detailed user-defined-type contract.

The two must remain semantically consistent.

---

294. Historical Material

"Zamani-Grammar.md" may contain older or aspirational user-defined type ideas.

Those ideas must not become language law until promoted through:

proposal
↓
semantic design
↓
specification
↓
grammar
↓
AST
↓
implementation
↓
tests
↓
stable

---

295. Conformance Status

"grammar/grammar.md" should track user-defined type features through:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

and implementation dimensions:

AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL

---

296. Required Completion Matrix

User-defined types are not production-ready merely because the grammar parses.

Production readiness requires:

Layer| Required
Specification| Yes
Lexer| Yes
Declaration grammar| Yes
Type grammar| Yes
AST| Yes
Name resolution| Yes
Generic resolution| Yes
Constraints| Yes
Semantic type model| Yes
Diagnostics| Yes
Effects| Yes
Capabilities| Yes
Resources| Yes
Contracts| Yes
Policies| Yes
Provenance| Yes
Classical IR| Where applicable
"quantum::ir"| Where applicable
HDL integration| Where applicable
Backend integration| Yes
Tests| Yes
Compatibility| Yes
Determinism| Yes
Scalability| Yes
Safe Rust| Yes

---

297. Definition of DONE for This Document

This document is complete when it specifies, without requiring another file to be redesigned:

1. ownership;
2. non-ownership;
3. declaration/type boundary;
4. type-expression boundary;
5. AST contract;
6. semantic contract;
7. generic contract;
8. constraint contract;
9. effect contract;
10. capability contract;
11. resource contract;
12. contract-system contract;
13. policy contract;
14. provenance contract;
15. classical IR boundary;
16. "quantum::ir" boundary;
17. HDL boundary;
18. backend boundary;
19. compatibility contract;
20. scalability contract;
21. deterministic compilation contract;
22. testing contract;
23. Rust safety contract;
24. cross-repository integration contract.

---

298. Definition of DONE for the Grammar Implementation

The corresponding implementation is complete when:

all declarations have one owner
        ↓
all type references use canonical typeExpression
        ↓
all generic applications use generic.g4
        ↓
all names use named.g4
        ↓
all declarations reach the canonical AST
        ↓
all types resolve semantically
        ↓
all constraints are checked
        ↓
all effects are checked
        ↓
all capabilities are checked
        ↓
all resources are checked
        ↓
all contracts are checked
        ↓
all policies are checked
        ↓
provenance is retained
        ↓
canonical IR lowering succeeds
        ↓
target feasibility is evaluated downstream

---

299. Required Negative Invariants

The implementation must reject or prevent:

duplicate semantic type identities
ambiguous type resolution
undeclared type references
invalid generic arity
invalid generic bounds
illegal recursive definitions
incompatible nominal assignments
invalid alias cycles
invalid visibility
conflicting trait implementations
invalid associated types
invalid type-family normalization
invalid dependent type expressions

according to the features actually enabled by the language version.

---

300. Forbidden Architectural Shortcuts

The implementation must not:

1. duplicate "typeExpression";
2. create a second AST;
3. create a second semantic type system;
4. embed backend selection into types;
5. hard-code hardware capacities;
6. hard-code generic limits;
7. assume fixed integer widths;
8. assume fixed address widths;
9. assume fixed memory;
10. assume fixed processor count;
11. assume fixed GPU count;
12. assume fixed QPU capacity;
13. assume fixed FPGA resources;
14. use application keywords for ordinary user-defined types;
15. use target names as universal type categories;
16. use string-only semantic type identity;
17. use unchecked generated types;
18. allow macros to bypass type checking;
19. allow reflection to bypass security;
20. allow adaptation to bypass policies;
21. use unsafe Rust.

---

301. Safe-Rust Production Contract

The compiler implementation must be compatible with:

#![forbid(unsafe_code)]

All user-defined type processing must therefore use safe Rust.

This includes:

- parsing;
- AST construction;
- declaration indexing;
- name resolution;
- generic resolution;
- type inference;
- unification;
- constraint solving;
- caching;
- serialization;
- diagnostics;
- provenance.

---

302. Memory Safety

No type declaration may cause:

- unchecked pointer dereference;
- use-after-free;
- data race;
- invalid aliasing;
- unchecked memory layout assumptions.

Safe Rust is mandatory for the compiler implementation.

---

303. Compiler Resource Exhaustion

The compiler may detect resource exhaustion.

For example:

type analysis exceeded configured compiler budget

This is an implementation/resource diagnostic.

It must not become:

Zamani has a maximum of N user-defined types

as language semantics.

---

304. Infinite Semantic Structures

The semantic model must detect non-terminating type computation.

For example:

type A = B
type B = A

may be invalid depending on alias semantics.

The compiler must detect it without imposing arbitrary source-level limits.

---

305. Deep Generic Structures

Very deep generic structures must be handled through safe compiler mechanisms.

If a configured compiler recursion/resource limit is reached, the compiler reports resource exhaustion rather than redefining the language's generic semantics.

---

306. Deterministic Error Reporting

Where multiple independent type errors exist, diagnostic ordering should be deterministic.

The implementation must not rely on nondeterministic parallel traversal.

---

307. Type Fingerprints

The semantic subsystem may derive stable fingerprints for user-defined types.

A fingerprint should include all semantic information relevant to the chosen compatibility identity.

It must not depend on:

- memory address;
- process ID;
- hash-map iteration order;
- host architecture.

---

308. Incremental Fingerprints

Changing irrelevant documentation metadata should not necessarily invalidate semantic type consumers unless the build system intentionally includes that metadata.

Semantic and non-semantic metadata should remain distinguishable.

---

309. Provenance Hashing

Where provenance participates in reproducibility, it should use deterministic representations.

---

310. Cross-Version Type Compatibility

A compiler must not assume two types with identical source text are compatible across incompatible language/specification versions.

Version information may affect semantic interpretation.

---

311. Library Boundary

A library exposing public user-defined types should expose sufficient metadata for consumers to perform semantic checking without requiring the library's source grammar to be reparsed.

---

312. Separate Compilation

Separate compilation must preserve:

type identity
generic contracts
trait/type-class contracts
associated types
constraints
visibility
provenance
compatibility metadata

where applicable.

---

313. Public API Stability

Public user-defined types form part of a library's API.

Changes must be classified for compatibility.

---

314. Type Documentation Stability

Documentation names must not be used as semantic identity.

Renaming documentation does not change the type.

---

315. Source Refactoring

Refactoring tools must preserve semantic type identity when performing a purely syntactic rename.

---

316. Semantic Rename

A declaration rename may preserve semantic identity if the language/package system defines rename compatibility.

Otherwise it may create a new identity.

This decision belongs to compatibility tooling.

---

317. Type Aliasing Across Modules

Cross-module aliases must retain provenance showing:

alias source
target type
module
version

---

318. Type Re-Exports

A module may re-export a user-defined type.

The re-export must not accidentally create a new semantic type identity.

---

319. Type Imports

Imports create access paths, not automatically new types.

---

320. Generic Type Aliases

Generic aliases must use canonical generic syntax and semantic substitution.

No specialized generic-alias grammar is needed unless the language specification requires one.

---

321. Recursive Generic Types

The implementation must support:

Node<T>
Tree<T>
Graph<N, E>

without fixed recursion limits in language semantics.

---

322. Mutually Recursive Types

Mutually recursive declarations should be supported where the type-system rules permit them.

Example:

type A = Option<B>;
type B = Option<A>;

The semantic resolver must construct a dependency graph before validation.

---

323. Declaration Ordering

Forward references should be supported if permitted by the declaration/module semantics.

Type declarations should not require textual ordering unless explicitly specified.

---

324. Forward Declaration

If forward declaration syntax exists, it must resolve into the same declaration identity as the eventual definition.

---

325. Type Completeness

The semantic checker must distinguish:

known declaration

from:

complete representation

Opaque/abstract types may intentionally remain incomplete.

---

326. Generic Completeness

A generic type declaration may be semantically valid without all type arguments being known until instantiation.

---

327. Partial Information

IDE and incremental compilation may operate with incomplete type information.

The compiler must distinguish:

unknown because analysis is incomplete

from:

invalid type

---

328. Error Type

If the existing semantic architecture uses an error/recovery type, it must not be confused with a valid user-defined type.

Error types exist for compiler recovery.

They must not leak into valid program semantics.

---

329. Unknown Type

An unresolved type must remain explicitly unresolved until diagnostics are emitted.

It must not silently become a valid fallback type.

---

330. No Silent "Unknown"

The compiler must not turn unresolved user-defined types into:

Any
Object
Dynamic
Unknown

unless the language specification explicitly defines such behavior.

---

331. Type Inference and User-Defined Types

Inference may infer a user-defined type when sufficient evidence exists.

The inferred result must be the canonical semantic type identity.

---

332. Ambiguous Inference

Ambiguous inference must produce a deterministic diagnostic.

The compiler must not arbitrarily choose one user-defined type.

---

333. Defaulting

If Zamani provides type defaulting, default rules must be explicit and documented.

Defaulting must not depend on target hardware.

---

334. Numeric User Types

A user-defined numeric type may wrap or abstract numeric primitives.

Its mathematical semantics remain independent from backend representation.

---

335. Units of Measure

Units can be modeled as nominal user-defined types or type-level parameters.

This is preferable to relying on raw primitive values where dimensional safety matters.

---

336. Physical Quantities

Types such as:

Length
Mass
Time
Temperature
Voltage
Current

may be library/domain-defined types.

They do not require universal language keywords.

---

337. Scientific Types

Scientific data structures should use user-defined types and generic facilities.

The core language need not enumerate scientific domains.

---

338. Security Types

Security abstractions should use nominal user-defined types where semantic separation is useful.

---

339. Protocol State Types

Protocol state machines can use distinct nominal types to prevent invalid transitions.

---

340. Hardware Resource Handles

Hardware resource handles can be represented as opaque or capability-associated user-defined types.

The runtime/hardware layer owns actual resources.

---

341. Quantum Handles

Quantum handles may be represented as opaque/resource-aware types.

Physical qubit allocation remains downstream.

---

342. Distributed Handles

Distributed resources may be represented by opaque types.

Network location and resource realization remain downstream.

---

343. Type and Topology

A user-defined type may carry semantic metadata related to topology requirements.

Topology realization remains owned by resources/distributed/hardware systems.

---

344. Type and Performance

Performance annotations may be attached as metadata or policies.

They must not change semantic type identity unless explicitly defined.

---

345. Type and Optimization Hints

Optimization hints should remain distinct from type meaning.

For example:

prefer vectorization

must not transform the type itself.

---

346. Type and Simulation

Simulation may use a different physical representation of the same semantic type.

The source type remains unchanged.

---

347. Type and Adaptive Execution

Adaptive execution may select different implementations of the same type.

The semantic type remains stable.

---

348. Type and Reproducibility

Changing target hardware should not change the source-level user-defined type.

If physical nondeterminism is introduced, it must be represented by the effect/execution semantics.

---

349. Type and Explainability

The compiler should be able to explain target-specific type lowering without claiming that lowering changed the source type.

---

350. Type and Provenance

A type transformation should be representable as:

source type
   ↓
semantic normalization
   ↓
generic specialization
   ↓
optimization
   ↓
lowering

with provenance edges where required.

---

351. Type and Evidence

Type properties may be backed by evidence.

Evidence does not become part of nominal identity unless specified.

---

352. Type and Contracts

Contracts may strengthen guarantees around a type.

They do not automatically alter nominal identity.

---

353. Type and Policies

Policies constrain use/realization.

They do not redefine the underlying type.

---

354. Type and Capabilities

Capabilities constrain authority.

They do not automatically redefine the underlying type.

---

355. Type and Resources

Resources constrain feasibility.

They do not automatically redefine the underlying type.

---

356. Type and Effects

Effects describe behavior.

They do not automatically redefine the underlying type.

---

357. Universal Semantic Separation

The complete separation is:

TYPE
  ↓
what values/structures mean

EFFECT
  ↓
what computation does

CAPABILITY
  ↓
what authority exists

RESOURCE
  ↓
what physical/logical capacity exists

REQUIREMENT
  ↓
what must be available

CONSTRAINT
  ↓
what must be satisfied

POLICY
  ↓
what is permitted/preferred/forbidden

CONTRACT
  ↓
what must hold

PROVENANCE
  ↓
where the information came from

IR
  ↓
how semantic computation is represented for lowering

User-defined types participate in all of these systems without owning them.

---

358. Final Cross-Domain Architecture

                  USER-DEFINED TYPE
                         │
                         ▼
                  canonical TypeExpr
                         │
                         ▼
                 declaration identity
                         │
                         ▼
                 semantic type model
                         │
        ┌────────────────┼────────────────┐
        ▼                ▼                ▼
     Classical        Quantum            HDL
        │                │                │
        ▼                ▼                ▼
   Classical IR      quantum::ir      HDL/domain IR
        │                │                │
        └────────────────┼────────────────┘
                         ▼
                    optimization
                         │
                      lowering
                         │
                  routing/scheduling
                         │
                 resilience/recovery
                         │
                        ZQN
                         │
                        HAL
                         │
              ┌──────────┼───────────┐
              ▼          ▼           ▼
             CPU        GPU         FPGA
              │          │           │
              └──────────┼───────────┘
                         ▼
                     ASIC/QPU
                         │
                         ▼
                 accelerator/HPC
                         │
                         ▼
                cluster/distributed
                         │
                         ▼
                   future target

---

359. Production Invariants

The following are mandatory.

1. There is exactly one canonical semantic type model.
2. There is exactly one universal "typeExpression" entry point.
3. User-defined declaration syntax belongs to the declaration subsystem.
4. User-defined type references belong to the canonical type subsystem.
5. Generic syntax has exactly one owner.
6. Named type syntax has exactly one owner.
7. Every declaration has one semantic identity.
8. Names are not semantic identities by themselves.
9. Nominal types remain nominal.
10. Structural compatibility follows canonical rules.
11. Aliases are distinct from nominal new types.
12. Recursive types are represented safely.
13. Generic arity is not universally capped.
14. Generic nesting is not universally capped.
15. Field counts are not universally capped.
16. Variant counts are not universally capped.
17. User-defined type counts are not universally capped.
18. No fixed machine capacities exist in type semantics.
19. No fixed quantum capacity exists in type semantics.
20. No fixed hardware capacity exists in type semantics.
21. Type semantics are target-independent.
22. Resource feasibility is downstream.
23. Capability negotiation is downstream.
24. Target selection is downstream.
25. ABI selection is downstream.
26. Physical layout is downstream unless explicitly semantic.
27. Quantum lowering uses "quantum::ir".
28. Classical lowering uses canonical classical IR.
29. HDL lowering uses the existing hardware/HDL pipeline.
30. Domain extensions use the canonical type model.
31. Application-specific concepts remain outside the universal type grammar.
32. Generated types undergo normal semantic validation.
33. Macros cannot bypass type checking.
34. Reflection cannot bypass semantic/security rules.
35. Adaptation cannot silently mutate type meaning.
36. Learning cannot bypass type validation.
37. FFI types retain provenance.
38. Foreign types do not automatically become portable.
39. Type resolution is deterministic.
40. Generic resolution is deterministic.
41. Constraint solving is deterministic.
42. Type caching cannot change semantics.
43. Parallel compilation cannot change semantics.
44. Type metadata is versionable.
45. Compatibility is explicit.
46. Provenance is preserved where required.
47. Diagnostics distinguish semantic errors from target infeasibility.
48. Compiler resource limits are not language semantics.
49. The implementation uses Rust 1.97 or later.
50. Production implementation uses safe Rust only.
51. Production implementation supports "#![forbid(unsafe_code)]".
52. No unsafe implementation is required for type graphs.
53. Type graphs can scale with available compiler resources.
54. Recursive semantic structures do not require infinite expansion.
55. New hardware does not require changing the universal type model.
56. New domains do not require changing the universal type model.
57. New libraries do not require new core type keywords.
58. New generic constructors do not require changes to "generic.g4".
59. New user-defined types do not require changes to "types.g4".
60. Every feature has explicit AST integration.
61. Every feature has explicit semantic integration.
62. Every feature has explicit IR integration where applicable.
63. Every feature has explicit diagnostics.
64. Every feature has positive tests.
65. Every feature has negative tests.
66. Every feature has boundary tests.
67. Every feature has scalability tests.
68. Every feature has determinism tests.
69. Every feature has compatibility tests.
70. Every production file has a documented ownership contract.

---

360. Final File Integration Contract

The final integration of this document is:

grammar/types/user-defined-types.md
             │
             ├── grammar/types/README.md
             │
             ├── grammar/types/types.g4
             │
             ├── grammar/types/named.g4
             │
             ├── grammar/types/generic.g4
             │
             ├── grammar/types/bounds.g4
             │
             ├── grammar/types/constraints.g4
             │
             ├── grammar/types/associated.g4
             │
             ├── grammar/types/type-class.g4
             │
             ├── grammar/declarations/types.g4
             │
             ├── grammar/declarations/structs.g4
             │
             ├── grammar/declarations/records.g4
             │
             ├── grammar/declarations/enums.g4
             │
             ├── grammar/declarations/unions.g4
             │
             ├── grammar/declarations/aliases.g4
             │
             ├── grammar/spec/type-system.md
             │
             ├── grammar/spec/semantics.md
             │
             ├── grammar/spec/portability.md
             │
             ├── grammar/spec/resources.md
             │
             ├── grammar/spec/effects.md
             │
             ├── grammar/spec/policies.md
             │
             ├── grammar/spec/compatibility.md
             │
             ├── domain-neutral AST
             │
             ├── semantic type resolver
             │
             ├── canonical Classical IR
             │
             ├── quantum::ir
             │
             ├── HDL/hardware semantic pipeline
             │
             ├── compiler lowering
             │
             ├── ZQN
             │
             ├── HAL
             │
             └── grammar/tests/

The critical ownership rule is:

DECLARATION
    ↓
declaration grammar

TYPE REFERENCE
    ↓
types.g4 / named.g4 / generic.g4

TYPE MEANING
    ↓
semantic type system

PHYSICAL REALIZATION
    ↓
compiler/backend/resource system

---

361. Final Production Principle

The production user-defined-type architecture is therefore:

                  ZAMANI SOURCE
                       │
                       ▼
                 USER TYPE DECL
                       │
                       ▼
                DOMAIN-NEUTRAL AST
                       │
                       ▼
                 TYPE RESOLUTION
                       │
        ┌──────────────┼──────────────┐
        ▼              ▼              ▼
     GENERICS      CONSTRAINTS     TRAITS
        │              │              │
        └──────────────┼──────────────┘
                       ▼
                SEMANTIC TYPE
                       │
        ┌──────────────┼──────────────┐
        ▼              ▼              ▼
      EFFECT       CAPABILITY       RESOURCE
        │              │              │
        └──────────────┼──────────────┘
                       ▼
                    POLICY
                       │
                   CONTRACT
                       │
                  PROVENANCE
                       │
                       ▼
              CANONICAL SEMANTICS
                       │
             ┌─────────┴─────────┐
             ▼                   ▼
        Classical IR         quantum::ir
             │                   │
             └─────────┬─────────┘
                       ▼
                  OPTIMIZATION
                       │
                    LOWERING
                       │
               ROUTING/SCHEDULING
                       │
              RESILIENCE/RECOVERY
                       │
                      ZQN
                       │
                      HAL
                       │
                       ▼
             ACTUAL COMPUTATIONAL
                  SUBSTRATE

The defining rule is:

«A user-defined type expresses reusable semantic structure. It does not prescribe the machine that realizes that structure.»

That is what makes user-defined types compatible with Zamani's POCO-REAF objective.

The same source-level type can therefore participate in a tiny embedded computation, classical computation, GPU/FPGA/ASIC computation, accelerator computation, quantum-classical computation, QPU execution, simulation, HPC, cluster execution, distributed execution, or future computational substrates without introducing artificial limits into the language.

The compiler determines feasibility from actual capabilities, resources, policies and target semantics. The type system preserves the meaning of the program.

That is the production boundary this file must enforce.