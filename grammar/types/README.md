Zamani Type System

Path: "grammar/types/README.md"
Repository: "Benwellonedge28/Zamani"
Language: Zamani
Grammar technology: ANTLR4
Rust baseline: Rust 1.97 or later
Rust edition: 2021
Implementation safety: safe Rust only
Architecture: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

"grammar/types/" is the complete source-level type-system subsystem of Zamani.

This README is the orchestrator, ownership map, dependency contract, integration contract, and completion specification for every file in this directory.

It does not replace the grammar.

The executable grammar authority is:

grammar/types/types.g4

The README answers:

- which type feature belongs to which file;
- which file owns each grammar rule;
- which files may depend on which other files;
- which files must never depend on each other;
- how every type reaches the AST;
- how every type reaches semantic analysis;
- how types interact with effects, resources, capabilities, contracts and policies;
- how classical types reach Classical IR;
- how quantum types reach "quantum::ir";
- how HDL/hardware types reach hardware semantics;
- how type information remains target-independent;
- how extensions are added without continually modifying the universal type grammar;
- how each file becomes independently complete;
- how production readiness is verified.

The central principle is:

«A type expresses program meaning. It does not select the machine that realizes that meaning.»

---

2. Production Objective

The type subsystem must support Zamani programs spanning:

- tiny embedded systems;
- ordinary classical systems;
- multicore systems;
- vector and tensor computation;
- GPUs;
- FPGAs;
- ASIC-oriented computation;
- accelerators;
- quantum simulators;
- QPUs;
- quantum-classical systems;
- HDL;
- hardware/software co-design;
- distributed systems;
- HPC;
- clusters;
- cloud systems;
- heterogeneous systems;
- future computational substrates.

The type system must therefore be resource-independent and target-neutral.

It must not encode an assumed maximum machine size.

The type system must remain capable of describing a program whose eventual realization is constrained only by:

1. the program's semantic requirements;
2. the selected language version;
3. the compiler's explicitly configured resource policy;
4. the capabilities actually available;
5. the resources actually available;
6. the target's semantic compatibility.

---

3. The Core Separation

The type subsystem owns:

"What type structure did the programmer express?"

It does not own:

"Can this particular machine execute it?"

It does not own:

"Which physical resource should execute it?"

It does not own:

"Which QPU should execute it?"

It does not own:

"Which physical qubits should be allocated?"

It does not own:

"Which GPU/FPGA/ASIC should be selected?"

It does not own:

"How should operations be routed?"

It does not own:

"How should execution be scheduled?"

It does not own:

"What ABI/layout should the backend emit?"

Those responsibilities belong downstream.

---

4. Repository Architecture

The complete language path is:

Zamani source
      │
      ▼
canonical lexer
      │
      ▼
canonical parser
      │
      ▼
grammar/types/types.g4
      │
      ▼
typeExpression
      │
      ▼
domain-neutral TypeExpr AST
      │
      ▼
structural validation
      │
      ├── name resolution
      ├── generic resolution
      ├── inference
      ├── unification
      ├── constraints
      ├── bounds
      ├── ownership
      ├── lifetimes
      ├── effects
      ├── capabilities
      ├── resources
      ├── contracts
      ├── policies
      └── provenance
      │
      ▼
canonical semantic type model
      │
      ├───────────────┬──────────────────┬─────────────────┐
      ▼               ▼                  ▼                 ▼
 Classical        Quantum              HDL              AI/Data
 semantics        semantics            semantics         semantics
      │               │                  │                 │
      └───────────────┴──────────────────┴─────────────────┘
                              │
                              ▼
                    canonical semantic IR
                              │
                    ┌─────────┴─────────┐
                    ▼                   ▼
              Classical IR         quantum::ir
                    │                   │
                    └─────────┬─────────┘
                              ▼
                        optimization
                              │
                              ▼
                           lowering
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

"grammar/types/" participates only in the source-language type portion of this pipeline.

---

5. Authority Model

There must be exactly one authority for each responsibility.

Responsibility| Authority
Language-wide grammar composition| "grammar/Zamani.g4"
Canonical parser composition| "grammar/antlr/ZamaniParser.g4"
Canonical lexical vocabulary| "grammar/antlr/ZamaniLexer.g4" / "grammar/lexer/"
Type grammar composition| "grammar/types/types.g4"
Type architecture/orchestration| "grammar/types/README.md"
Type specification| "grammar/specification/"
Type machine contracts| "grammar/spec/"
Type conformance| "grammar/grammar.md"
Source type AST| existing frontend AST
Type semantic model| semantic type subsystem
Classical representation| Classical IR
Quantum representation| "quantum::ir"
Hardware realization| hardware/backend/HAL
Resource feasibility| "grammar/resources/" + semantic/resource subsystem
Effects| "grammar/effects/" + semantic effect subsystem
Policies| policy subsystem
Provenance| compiler/frontend provenance subsystem
Type tests| "grammar/tests/" / type-specific tests

No file under "grammar/types/" may silently become the authority for a responsibility belonging elsewhere.

---

6. The Orchestrator Rule

"grammar/types/README.md" is the directory-level orchestrator.

"grammar/types/types.g4" is the grammar-level orchestrator.

They have different responsibilities.

README.md
    │
    ├── ownership
    ├── dependency graph
    ├── integration contracts
    ├── implementation status
    ├── file completion criteria
    ├── scalability policy
    ├── cross-repository contracts
    └── test matrix
             │
             ▼
        types.g4
             │
             ├── primitive
             ├── named
             ├── generic
             ├── composite
             ├── function
             ├── reference
             ├── pointer
             ├── algebraic
             ├── constrained
             ├── advanced
             ├── domain
             └── extensions

The README must never duplicate ANTLR productions.

The grammar must never contain directory-level project management prose as a substitute for actual ownership.

---

7. The One Universal Type Entry Point

The entire type subsystem has one public source-level entry point:

typeExpression

It is owned by:

grammar/types/types.g4

No specialized file may create another universal type-expression root.

Forbidden:

primitiveTypeExpression
quantumTypeExpression
hardwareTypeExpression
aiTypeExpression
universalTypeExpression
advancedTypeExpression

as competing universal roots.

Specialized grammars expose specialized constructors.

"types.g4" composes them.

---

8. Canonical Dependency Direction

The dependency direction is:

primitive
named
value-level type parameters
generic
composite
function
reference/pointer
algebraic
constraints/bounds
advanced type system
domain types
       │
       ▼
    types.g4
       │
       ▼
TypeExpr
       │
       ▼
semantic analysis

Never:

types.g4
   ▲
   │
specialized.g4

A specialized delegate must not import the orchestrator merely to obtain the type system it is supposed to contribute to.

---

9. No Circular Type-Grammar Dependencies

The following architecture is forbidden:

Types → Generic → Types
Types → Map → Types
Types → Reference → Types
Types → Effectful → Types
Types → Quantum → Types

A specialized grammar may consume rules exposed by the composition architecture where the ANTLR composition mechanism supports that structure, but it must not create a circular grammar-import graph.

The dependency graph must be acyclic.

---

10. Current "grammar/types/" Inventory

The current repository contains the following relevant entries.

Orchestrator

README.md
types.g4

Fundamental type families

primitive.g4
named.g4
generic.g4
function.g4
tuple.g4
array.g4
slice.g4
map.g4
reference.g4
pointer.g4
option.g4
optional.g4
result.g4
never.g4
record.g4
sum.g4
union.g4
algebraic-types.g4
composite-types.g4

Generic/constraint system

bounds.g4
constraints.g4
type-constraints.g4
associated.g4
associated types.g4
type-class.g4
variance.g4
functional dependencies.g4

Advanced type system

dependent.g4
existential-types.g4
higher-kinded-types.g4
path-dependent-types.g4
type-families.g4
type-providers.g4
advanced-type-level-computation.g4

Ownership/effect/resource/capability types

linear.g4
affine.g4
effectful.g4
resource.g4
capability.g4
temporal.g4

Domain types

classical.g4
quantum.g4
hardware.g4

Compatibility/overlap entries

array-types.g4...
map-types.g4
option-types.g4
optional.g4
associated types.g4
composite-types.g4

The repository also currently contains a malformed directory entry beginning with:

array-types.g4

whose path contains embedded comment text and whose size is zero.

That entry must be treated as a repository hygiene defect, not as a valid production grammar module.

---

11. Duplicate-File Policy

Several names represent overlapping concepts.

Examples:

map.g4
map-types.g4

option.g4
option-types.g4
optional.g4

associated.g4
associated types.g4

array.g4
array-types.g4...

constraints.g4
type-constraints.g4

These are not allowed to become multiple authorities.

Each overlapping file must ultimately be classified as exactly one of:

CANONICAL
DELEGATE
COMPATIBILITY
DEPRECATED
EXPERIMENTAL
HISTORICAL

Only one canonical implementation may own a particular grammar rule.

---

12. Existing Filename Preservation

Existing filenames should not be renamed merely for aesthetic consistency.

This is important because:

- downstream imports may reference them;
- tooling may discover them;
- documentation may reference them;
- historical compatibility may depend on them;
- external consumers may rely on stable paths.

If consolidation is required, retain the old file as a compatibility/deprecation surface until the migration contract is complete.

---

13. Fundamental File Contract

Every ".g4" file under this directory must have an explicit header containing:

PURPOSE
STATUS
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
SCALABILITY_CONTRACT
DIAGNOSTICS_CONTRACT
COMPLETION_CRITERIA

This makes a file independently completable.

A completed file must not need to be reopened simply because another type file is later implemented.

---

14. File Completion Principle

A type grammar file is DONE only when all of the following are already known:

syntax
ownership
dependencies
AST mapping
semantic meaning
type interactions
effect interactions
capability interactions
resource interactions
contract interactions
policy interactions
provenance interactions
IR destination
diagnostics
tests
compatibility
scalability
integration

The implementation of a downstream file may still occur later.

That does not mean the upstream file was incomplete.

Its contract must already specify how the integration occurs.

---

15. "types.g4" — Canonical Type Orchestrator

Owns

typeExpression
typeCore
typePrefix
typePostfix
typeValueExpression
type extension composition

Does not own

- primitive semantics;
- generic semantics;
- array semantics;
- quantum semantics;
- hardware realization;
- resource discovery;
- capability negotiation;
- effect checking;
- type inference;
- unification;
- substitution;
- ABI layout;
- runtime layout.

Integrates

All canonical type delegates.

Must export

typeExpression

as the only universal public type entry point.

Production requirements

"types.g4" must:

1. remain parser-only;
2. use canonical lexer tokens;
3. avoid lexer definitions;
4. avoid embedded Rust;
5. avoid target actions;
6. avoid semantic execution;
7. avoid physical resource selection;
8. avoid domain-specific implementation logic;
9. avoid finite type-family enumeration where open-world extension is intended;
10. avoid hard-coded physical limits.

Completion

"types.g4" is complete when its import graph is acyclic and every imported rule has exactly one owner.

---

16. "primitive.g4"

Owns

Language-defined primitive/scalar type syntax.

Potential categories include:

boolean
integer
floating
character
string
unit
never

Exact spellings are controlled by the language specification and lexer.

Does not own

- machine selection;
- numeric representation lowering;
- ABI width selection;
- register allocation;
- runtime layout.

Integration

primitive.g4
    ↓
types.g4
    ↓
TypeExpr
    ↓
semantic scalar type

Scalability

Primitive syntax must not silently impose machine capacity.

A semantic integer must not become a machine-specific width merely because one backend uses that width.

---

17. "named.g4"

Owns

Named and qualified source-level types.

Examples:

User
module::User
project::module::Type

Does not own

- symbol resolution;
- module loading;
- declaration lookup;
- generic substitution.

Integration

named.g4
    ↓
TypeExpr
    ↓
name resolution

Names remain symbolic until semantic analysis.

---

18. "generic.g4"

Owns

Generic type application syntax.

Examples:

Vec<T>
Map<K, V>
Result<T, E>
Tensor<T>
Model<T>

Must be open-ended

Generic argument lists use repetition/recursion.

No finite universal generic arity.

Does not own

- type inference;
- bound satisfaction;
- type-class resolution;
- generic specialization;
- monomorphization.

Integration

generic.g4
    ↓
TypeExpr
    ↓
generic resolution
    ↓
constraint solving
    ↓
specialization

---

19. Generic Scalability

The grammar must support structurally arbitrary generic nesting:

Vec<Option<T>>
Map<K, Vec<Result<V, E>>>
Tensor<Vec<Result<T, E>>>

There must be no language-level constants such as:

MAX_GENERIC_DEPTH
MAX_GENERIC_ARGUMENTS
MAX_TYPE_PARAMETERS

Compiler resource protection may exist outside the language semantics.

---

20. "function.g4"

Owns

Function type syntax.

Conceptually:

fn() -> T
fn(T) -> U
fn(T, U) -> V

Does not own

- function declarations;
- function bodies;
- calling conventions;
- ABI;
- register allocation;
- execution scheduling.

Integration

function.g4
    ↓
TypeExpr::Function
    ↓
semantic function type
    ↓
effects/capabilities/resources
    ↓
IR/backend

---

21. Function Type + Effects

Effects are not duplicated inside "types/".

If a function type can carry effect information, the type grammar consumes the effect representation owned by:

grammar/effects/

The architecture is:

function type
     +
effect information
     +
capability requirements
     +
resource requirements

The type grammar does not define the effect universe.

---

22. "tuple.g4"

Owns

Tuple type syntax.

Must support structurally:

()
(T,)
(T, U)
(T, U, V)

and arbitrary tuple length subject to implementation resources.

Does not own

- tuple layout;
- ABI layout;
- register allocation;
- machine representation.

AST

Maps to the existing tuple "TypeExpr" representation.

---

23. "array.g4"

Owns

Array type syntax.

The source-level model must preserve:

element type
array length/value parameter

when a length is part of the type.

Symbolic sizes

The grammar must support symbolic values where the language specifies them.

Conceptually:

[T; N]

must preserve "N" as source-level information.

Must not

Convert a symbolic size into an assumed host-machine integer during parsing.

---

24. Array Size Architecture

Array size belongs to the type-level value system.

The architecture is:

array.g4
    ↓
TypeExpr::Array
    ↓
TypeValueExpr
    ↓
semantic evaluation / constraint checking

The parser must not decide whether a value is physically realizable.

---

25. "slice.g4"

Owns

Slice type syntax.

Does not own

- pointer width;
- address width;
- allocation strategy;
- ownership semantics;
- runtime memory layout.

Integration

slice.g4
    ↓
TypeExpr::Slice
    ↓
reference/ownership semantics
    ↓
semantic model

---

26. "reference.g4"

Owns

Reference syntax.

Conceptually:

&T
&mut T

and lifetime syntax where standardized.

Does not own

- borrow checking;
- lifetime solving;
- alias analysis;
- memory allocation.

Integration

reference.g4
    ↓
TypeExpr::Reference
    ↓
lifetime/ownership analysis

---

27. "pointer.g4"

Owns

Pointer type syntax where Zamani exposes pointer types.

Important distinction

A pointer type is a source-level semantic abstraction.

It must not assume:

32-bit address
64-bit address
fixed register width
fixed virtual-address width

unless such a representation is explicitly part of the type's language semantics.

Integration

pointer.g4
    ↓
TypeExpr::Pointer
    ↓
semantic pointer model
    ↓
target-specific lowering

---

28. "map.g4" and "map-types.g4"

Only one file may own canonical map syntax.

Recommended production ownership:

map.g4 → canonical
map-types.g4 → compatibility/deprecated

unless repository-wide dependency analysis establishes the reverse.

Canonical map form

Conceptually:

Map<K, V>

Does not own

- hashing;
- ordering;
- storage;
- allocation;
- runtime implementation.

---

29. "option.g4", "optional.g4", "option-types.g4"

These files represent overlapping concepts.

They must be consolidated into one canonical owner.

Recommended architecture:

option.g4
    ↓
canonical option constructor

with:

optional.g4
option-types.g4

classified as compatibility/deprecated surfaces unless repository consumers require another arrangement.

Semantic distinction

Option-like types represent absence/presence.

They must not become aliases for arbitrary nullable machine pointers unless the language specification explicitly defines such equivalence.

---

30. "result.g4"

Owns

Result type syntax.

Conceptually:

Result<T, E>

Does not own

- exception handling;
- runtime error transport;
- diagnostics implementation.

Integration

Result<T, E>
    ↓
TypeExpr
    ↓
semantic error/value type

---

31. "never.g4"

Owns

Never/uninhabited type syntax.

It must integrate with:

- control-flow analysis;
- function return analysis;
- pattern exhaustiveness;
- type compatibility.

It must not define its own control-flow subsystem.

---

32. "record.g4"

Owns

Record/structural product type syntax where applicable.

It must support source-level composition without imposing:

- field-count limits;
- fixed alignment;
- fixed layout;
- ABI rules.

Integration

record.g4
    ↓
domain-neutral record AST
    ↓
semantic record type
    ↓
layout lowering

---

33. "sum.g4"

Owns

Sum/algebraic variant type syntax.

Does not own

- pattern matching;
- exhaustiveness checking;
- runtime tag representation.

Those are integrated downstream.

---

34. "union.g4"

Owns

Union type syntax where union semantics are distinct from sum types.

The specification must clearly distinguish:

sum

from:

union

The two grammars must not accidentally create two spellings for the same semantic type without an explicit specification.

---

35. "algebraic-types.g4"

This is a grouping/delegation surface for algebraic types.

It must not create a second algebraic type system.

Its canonical role is to compose:

sum
union
product/record
option
result

where appropriate.

---

36. "composite-types.g4"

This file must be classified carefully.

It may serve as:

compatibility façade

or:

composition delegate

but must not independently redefine:

arrayType
tupleType
mapType
recordType
optionType
resultType

If it currently does so, those duplicate rules must be normalized.

---

37. "linear.g4"

Owns

Linear type qualification syntax.

Semantic purpose

Linear types constrain use/consumption of values.

This is especially important for:

- quantum resources;
- handles;
- ownership-sensitive resources;
- session-like resources;
- unique hardware resources.

Does not own

The complete ownership checker.

That belongs in semantic analysis.

---

38. "affine.g4"

Owns

Affine type qualification syntax.

Affine values may be used zero or one time according to the semantic specification.

Integration

affine.g4
    ↓
TypeExpr
    ↓
ownership/usage analysis

---

39. Linear/Affine + Quantum

Quantum types may require linear or affine semantics.

The architecture is:

quantum.g4
      +
linear.g4 / affine.g4
      ↓
TypeExpr
      ↓
quantum semantic validation
      ↓
quantum::ir

"quantum.g4" must not independently implement a second ownership system.

---

40. "dependent.g4"

Owns

Dependent-type syntax.

It may express relationships between types and source-level values.

Examples conceptually:

Vector<T, N>
Matrix<T, Rows, Columns>
Tensor<T, Shape>

Does not own

- arbitrary compile-time execution;
- machine resource discovery;
- runtime evaluation;
- target selection.

Integration

dependent.g4
    ↓
TypeExpr + TypeValueExpr
    ↓
constraint/evaluation subsystem

---

41. "typeValueExpression"

The type system requires a dedicated source-level representation for values used by types.

It must remain target-independent.

Examples:

N
Rows
Columns
Shape
Batch
RequiredMemory

The parser must preserve symbolic identity.

It must not silently convert symbolic values to:

usize
u32
u64

because the compiler itself happens to execute on a particular host.

---

42. Type-Level Arithmetic

The current architecture must distinguish:

symbolic type values

from:

arbitrary type-level computation

The presence of "advanced-type-level-computation.g4" does not by itself establish that every arithmetic expression is supported.

If arbitrary type-level computation is standardized, it requires:

- syntax;
- AST;
- evaluation model;
- normalization;
- termination/resource policy;
- diagnostics;
- determinism;
- provenance;
- compatibility;
- semantic checking;
- serialization;
- tests.

No README claim should imply completion before all of those exist.

---

43. "associated.g4"

Owns

Associated-type references/projections.

Conceptually:

T::Item
T::Output

Does not own

- trait/type-class resolution;
- instance selection;
- coherence;
- specialization.

---

44. "associated types.g4"

This overlapping filename must not create another associated-type authority.

It must be classified as:

CANONICAL
COMPATIBILITY
DEPRECATED
HISTORICAL

after dependency inspection.

The canonical rule owner must remain singular.

---

45. "type-class.g4"

Owns

Type-class/trait-like type syntax where standardized.

It may express:

T: Numeric
T: Ordered
T: Serializable

or the canonical Zamani equivalent.

Does not own

- instance selection;
- trait resolution;
- coherence;
- specialization;
- method dispatch.

Those belong to semantic analysis.

---

46. "bounds.g4"

Owns

Generic/type parameter bounds.

Examples:

T: Numeric
T: QuantumState
T: Sendable

The grammar records structure.

Semantic analysis determines whether a bound is satisfied.

---

47. "constraints.g4"

Owns

General type constraints where they are distinct from parameter bounds.

It must integrate with:

type-constraints.g4
bounds.g4
dependent.g4
type-class.g4

without becoming another constraint universe.

---

48. "type-constraints.g4"

This should be the canonical owner for type-specific constraint composition if repository-wide inspection confirms that role.

It must not duplicate:

grammar/resources/
grammar/policies/
grammar/validation/

A type constraint is not automatically:

- a hardware requirement;
- a security policy;
- a runtime assertion.

The semantic layer determines how those systems interact.

---

49. "variance.g4"

Owns

Variance annotations and variance-related syntax.

Conceptually:

covariant
contravariant
invariant

or the canonical Zamani notation.

Does not own

Variance inference unless that is explicitly defined as semantic analysis.

---

50. "functional dependencies.g4"

Owns

Functional-dependency syntax if this feature is standardized.

It must remain integrated with the same type-class/constraint model.

It must not introduce an independent generic-resolution system.

---

51. "existential-types.g4"

Owns

Existential type syntax.

Conceptually:

exists T. ...

or the language's standardized equivalent.

Does not own

Runtime existential representation.

---

52. "higher-kinded-types.g4"

Owns

Higher-kinded type syntax.

The feature must be expressed as an extension of the same generic/type-parameter model.

It must not create a separate type universe.

---

53. "type-families.g4"

Owns

Type-family syntax where standardized.

Type-family evaluation is semantic.

It must be deterministic and must not depend on:

- hardware;
- time;
- randomness;
- network state;
- filesystem state.

---

54. "path-dependent-types.g4"

Owns

Path-dependent type syntax.

It must integrate with the existing:

named.g4
associated.g4
generic.g4

architecture.

It must not create another name-resolution mechanism.

---

55. "type-providers.g4"

Type providers are an advanced extension.

They must not become a hidden source of nondeterministic compilation.

If type information is obtained from external sources, the semantic/compiler architecture must explicitly define:

- source identity;
- version;
- provenance;
- reproducibility;
- failure behavior;
- security policy;
- caching;
- compatibility.

The grammar itself does not access external data.

---

56. "advanced-type-level-computation.g4"

Owns

Advanced compile-time type-level constructs.

Does not own

A general-purpose runtime execution engine.

Type-level computation must remain:

- deterministic;
- inspectable;
- resource-policy controlled;
- provenance-preserving;
- compatible with reproducible builds.

---

57. "temporal.g4"

Owns

Temporal type syntax.

It must remain separate from runtime temporal storage/execution.

Temporal semantics may integrate with:

- distributed systems;
- event systems;
- historical data;
- temporal memory;
- deterministic execution.

The type grammar only represents type structure.

---

58. "effectful.g4"

Owns

Effect-qualified type syntax.

It must consume the canonical effect representation.

It must not redefine:

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

Those effect categories belong to the effect subsystem.

---

59. "resource.g4"

Owns

Source-level resource type abstractions.

Examples conceptually:

Resource<T>

Does not own

Actual resource discovery.

Resource requirements remain in:

grammar/resources/

and semantic/resource analysis.

---

60. "capability.g4"

Owns

Source-level capability type abstractions.

Examples:

Capability<"quantum.measurement">
Capability<"gpu.compute">
Capability<"tensor.compute">

Does not own

Whether the target actually provides that capability.

The architecture is:

Capability<T>
      ↓
semantic capability identity
      ↓
capability negotiation
      ↓
target discovery

---

61. Resource and Capability Separation

Types may describe:

Resource<T>
Capability<T>

but requirements such as:

requires capability("quantum.measurement");
requires capability("gpu.compute");
requires capability("tensor.compute");
requires qubits >= N;
requires memory >= RequiredMemory;
requires topology(...);

belong to the resource/capability requirement system.

The type system may reference the resulting semantic concepts.

It must not perform negotiation.

---

62. "classical.g4"

Owns

Classical-domain type constructors that are genuinely part of the language type model.

Possible semantic families include:

scalar
numeric
complex
rational
vector
matrix
tensor

where specified.

Critical rule

Classical types must use the same universal type system.

They must not create:

ClassicalTypeExpr

as a competing AST hierarchy.

---

63. Tensor Scalability

Tensor types must not impose:

MAX_TENSOR_RANK

or a fixed universal dimension count.

Symbolic representations must remain symbolic.

Examples:

Tensor<T, Shape>
Tensor<T, N>
Tensor<T, Rows, Columns>

must not require the grammar to know how large the eventual tensor is.

---

64. "quantum.g4"

Owns

Source-level quantum type constructors.

Examples may include:

Qubit
LogicalQubit
QuantumState<T>
QRegister<N>
QuantumChannel<T>

where standardized.

Does not own

- physical qubit allocation;
- QPU selection;
- coupling maps;
- calibration;
- routing;
- pulse scheduling;
- QEC implementation;
- ZQN generation.

---

65. Quantum Type Boundary

The canonical path is:

quantum.g4
     ↓
TypeExpr
     ↓
quantum semantic type
     ↓
quantum::ir
     ↓
optimization
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

There must be one canonical quantum IR.

No type grammar may create a competing quantum IR.

---

66. Quantum Type Scalability

The type grammar must never introduce:

MAX_QUBITS
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS
MAX_QREG_SIZE

A symbolic:

QRegister<N>

remains symbolic until semantic/resource analysis.

Physical feasibility is determined downstream.

---

67. "hardware.g4"

Owns

Source-level hardware abstractions.

Examples conceptually:

Hardware<T>
Accelerator<T>
Signal<T>
Memory<T>
Interconnect<T>
Device<T>

Does not own

- physical device discovery;
- vendor IDs;
- fixed bus widths;
- fixed register counts;
- physical topology;
- timing calibration;
- power management;
- placement.

---

68. Hardware Type Principle

A hardware type describes intent or semantic capability, not a physical machine.

Therefore:

Hardware<Compute>

does not mean:

specific CPU

and:

Accelerator<T>

does not mean:

specific accelerator model

unless an explicit target constraint is supplied elsewhere.

---

69. Type Extensions

New domains must not require continual growth of the universal type keyword inventory.

The preferred mechanisms are:

named types
qualified types
generic applications
associated types
type classes
value parameters
dialects
registered extensions
capabilities

Therefore future types such as:

Tensor<T>
Model<T>
Dataset<T>
Distribution<T>
Evidence<T>
QuantumState<T>
Signal<T>
Accelerator<T>
DistributedState<T>

remain expressible without adding one grammar branch per application.

---

70. Application-Specific Types

Application concepts such as:

vision
robotics
sentiment
blockchain
payments
VR
AR
legal
administration

must not become universal primitive types merely because applications use them.

They should normally be represented through:

libraries
named types
generic types
dialects
capabilities
policies
domain modules

The core type system remains universal.

---

71. AI and Knowledge-Oriented Types

The type system must be capable of representing semantic types needed by reasoning and learning systems without embedding an application-specific type inventory.

Examples:

Model<T>
Dataset<T>
Distribution<T>
Probability<T>
Confidence<T>
Evidence<T>
Decision<T>
Knowledge<T>
Agent<T>

These are library/domain types unless explicitly standardized by Zamani.

The universal grammar does not need a keyword for every concept.

---

72. Uncertainty Types

Uncertainty may be represented through types such as:

Probability<T>
Distribution<T>
Confidence<T>
Uncertain<T>

when supplied by the language/library semantic model.

The type grammar does not hard-code a particular probability implementation.

---

73. Provenance Types

Provenance-related types may represent:

Evidence<T>
Derived<T>
Observed<T>
Verified<T>
Decision<T>

where standardized.

Actual provenance records remain owned by the provenance subsystem.

---

74. Pattern Types

"pattern.g4" must integrate with the same type model used by:

expressions
statements
match
guards

Pattern typing belongs to semantic analysis.

The type grammar must not create a separate pattern type universe.

---

75. Type Extensions and Metadata

Future domain extensions should preferably be expressible through explicit extension mechanisms rather than universal keyword expansion.

The extension boundary must define:

extension identity
arguments
version
provenance
compatibility
semantic owner

The extension mechanism must remain deterministic.

---

76. Type-System Interaction with Contracts

Types participate in:

requires
ensures
invariant
assume
guarantee
property

but do not own contract semantics.

Architecture:

TypeExpr
   ↓
semantic type
   ↓
contract validation

A contract may constrain a type.

A type does not become a contract merely because it has constraints.

---

77. Type-System Interaction with Policies

Policies may constrain:

- permitted types;
- permitted effects;
- permitted capabilities;
- permitted resources;
- permitted adaptations;
- permitted foreign interfaces.

The policy subsystem owns policy semantics.

"types/" only supplies the type structure being evaluated.

---

78. Type-System Interaction with Effects

A type can carry effect information where the language defines effectful types.

The effect system remains authoritative.

No type grammar may redefine the effect vocabulary.

---

79. Type-System Interaction with Capabilities

A type may require or expose semantic capability information.

For example:

QuantumState<T>

may ultimately require quantum capabilities.

The grammar only expresses the type.

Semantic analysis determines the implications.

---

80. Type-System Interaction with Resources

A type may imply resource requirements.

For example, a type involving:

QRegister<N>

may lead to a semantic resource requirement involving "N".

The grammar does not allocate the resource.

---

81. Type-System Interaction with Provenance

The parser must preserve source locations.

The type grammar must not generate:

- timestamps;
- machine IDs;
- hashes;
- random IDs;
- environment data.

Provenance is attached downstream.

---

82. Type-System Interaction with Determinism

Parsing must depend only on:

source text
grammar version
lexer version
explicit dialect configuration

It must not depend on:

CPU availability
GPU availability
QPU availability
memory availability
filesystem state
network state
wall-clock time
randomness
environment variables
scheduler state

---

83. POCO-REAF Type Invariant

For a given Zamani source program:

source type meaning

must remain stable across target realizations.

A backend may change:

representation
layout
specialization
placement
routing
scheduling
execution strategy

but must not silently change:

type meaning
type safety
declared constraints
declared effects
declared capabilities
declared resource requirements

If a target cannot satisfy the requirements, the compiler/runtime must report or negotiate that condition.

It must not silently weaken the type.

---

84. No Hard-Coded Physical Limits

The entire directory is prohibited from defining universal constants such as:

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

Also prohibited as language-level assumptions:

64 GB RAM
24 GB VRAM
32-bit registers
8 threads
16 lanes
1024 qubits

unless such values are literal program data or an explicitly selected target constraint.

---

85. Symbolic Scaling

The preferred representation is:

N
M
Rows
Columns
Shape
Batch
RequiredMemory
RequiredWidth
Qubits
Devices
Nodes

rather than hard-coded physical capacities.

The compiler may specialize these values for a particular realization.

The source type system must remain general.

---

86. "Infinity" and Resource Availability

POCO-REAF does not mean that every finite physical machine can execute every program.

It means:

«the language does not make an arbitrary finite target capacity part of the source type system.»

Therefore the correct model is:

program requirement
       ↓
target capability
       ↓
resource availability
       ↓
negotiation
       ↓
specialization
       ↓
realization

not:

program
       ↓
hard-coded machine size

---

87. Compiler Resource Limits

A compiler may require protection against pathological input.

Examples include implementation-level controls for:

- memory exhaustion;
- excessive recursion;
- compilation time;
- parser work;
- type solver work.

These are compiler resource policies, not language semantics.

They must be:

- explicit;
- configurable where appropriate;
- documented;
- diagnosable;
- deterministic;
- separate from type meaning.

They must never be encoded as universal language capacities.

---

88. Rust Requirements

All Rust implementation associated with this subsystem must target:

Rust 1.97+
Rust 2021

and use safe Rust only.

The type grammar itself contains no Rust implementation code.

The frontend, parser integration, semantic type system, tests, and compiler components must not require unsafe Rust.

---

89. AST Boundary

Every canonical type construct must map into the repository's existing domain-neutral type AST.

The architecture is:

grammar
    ↓
TypeExpr

not:

grammar
    ↓
QuantumTypeExpr
HardwareTypeExpr
AITypeExpr
TensorTypeIR

Domain-specific semantic information is resolved after the common AST boundary.

---

90. AST Compatibility Rule

Before declaring a new type grammar feature complete, verify:

grammar rule
    ↓
parser
    ↓
AST constructor
    ↓
AST serialization/equality/debugging
    ↓
semantic resolver
    ↓
tests

A grammar feature without an AST representation is not production-ready.

An AST variant without grammar support is not production-ready.

A grammar and AST feature without semantic handling is not production-ready.

---

91. Semantic Type Boundary

After AST construction:

TypeExpr
   ↓
name resolution
   ↓
generic resolution
   ↓
inference
   ↓
unification
   ↓
bounds
   ↓
constraints
   ↓
ownership
   ↓
lifetimes
   ↓
effects
   ↓
capabilities
   ↓
resources
   ↓
contracts
   ↓
policies
   ↓
provenance
   ↓
canonical semantic type

The semantic type model must be the authoritative resolved representation.

---

92. Canonical IR Boundary

Types do not directly emit machine-specific IR.

The architecture is:

TypeExpr
   ↓
semantic type
   ↓
canonical semantic model
   ↓
Classical IR / quantum::ir / other canonical semantic representations

Quantum semantics must use the existing canonical:

quantum::ir

boundary.

No type grammar may create a second quantum IR.

---

93. Classical Integration

Classical types may eventually influence:

Classical IR

through semantic lowering.

Examples:

numeric types
vector types
matrix types
tensor types
record types
function types

The type grammar itself remains representation-independent.

---

94. Quantum Integration

Quantum types may influence:

quantum::ir

after semantic validation.

The type grammar does not:

- route qubits;
- select gates;
- choose QPU topology;
- perform QEC;
- choose pulse implementations.

---

95. HDL Integration

HDL-oriented types integrate through:

type grammar
    ↓
HDL semantic analysis
    ↓
HDL representation
    ↓
synthesis/lowering

The type grammar must not hard-code:

bus width
register count
pipeline depth
clock count
device count

as universal limits.

---

96. Hardware Integration

Hardware types represent source-level hardware abstractions.

Hardware realization occurs later through:

hardware semantic analysis
    ↓
resource/capability analysis
    ↓
placement
    ↓
lowering
    ↓
backend/HAL

---

97. Distributed Integration

Distributed types may express abstractions such as:

Node<T>
Channel<T>
Service<T>
Message<T>
DistributedState<T>

but the type system must not define:

MAX_NODES
MAX_CHANNELS
MAX_DEVICES

Topology and capacity are downstream concerns.

---

98. Networking Integration

Network-related types may express:

Endpoint<T>
Stream<T>
Message<T>
Protocol<T>

but network availability and topology are external semantic/resource concerns.

---

99. Interoperability Integration

FFI/ABI types must remain connected to:

grammar/interoperability/

The type system may describe:

foreign type
extern type
ABI-compatible type

where standardized.

It does not select a calling convention unless the language specification explicitly makes it source semantics.

---

100. Metaprogramming Integration

Type reflection and compile-time type computation must integrate with:

grammar/metaprogramming/

rather than creating a second reflection language inside "types/".

The type grammar represents type-level syntax.

Compile-time execution remains downstream and controlled.

---

101. Dialect Integration

A dialect may introduce new types without modifying the universal type grammar when those types can be expressed through:

named types
qualified types
generic types
extensions
capabilities

A dialect requiring new core syntax must provide:

dialect identity
version
lexer contract
parser contract
AST contract
semantic contract
compatibility contract
tests

---

102. Canonical Lexer Contract

All tokens consumed by "grammar/types/" originate from the canonical lexical system.

No type grammar may define:

IDENTIFIER
INTEGER_LITERAL
STRING_LITERAL
LESS_THAN
GREATER_THAN
COMMA

or equivalent tokens locally.

The lexical authority remains:

grammar/antlr/ZamaniLexer.g4
grammar/lexer/

---

103. Token Naming Rule

Type grammars must use canonical token names.

Do not introduce duplicate lexical concepts such as:

Question
QuestionMark
BitAnd
Ampersand

unless they represent genuinely different lexical/semantic constructs.

---

104. Dependency Manifest

Every type file must conceptually declare:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
IR_OWNER:
SPEC_OWNER:
TEST_OWNER:
COMPATIBILITY_OWNER:

Example:

grammar/types/quantum.g4

DEPENDS_ON:
    canonical lexer
    generic type syntax
    named type syntax
    type-value syntax

EXPORTS:
    quantumType

CONSUMED_BY:
    types.g4

AST_OWNER:
    frontend TypeExpr

SEMANTIC_OWNER:
    quantum semantic type system

IR_OWNER:
    quantum::ir

SPEC_OWNER:
    grammar/specification/quantum.md

TEST_OWNER:
    grammar/tests/types/quantum/

COMPATIBILITY_OWNER:
    grammar/compatibility/

---

105. Integration Contract for Every File

Every file must answer these questions before completion:

Purpose

What does this file exist to express?

Owns

Which exact grammar rules belong here?

Does Not Own

Which rules explicitly belong elsewhere?

Dependencies

Which grammar/token files must exist?

Exports

Which rules are consumed by other files?

AST

Which AST representation is required?

Semantic

What does the construct mean?

Type interactions

How does it compose with other types?

Effects

Can the construct carry or imply effects?

Capabilities

Can semantic analysis derive capability requirements?

Resources

Can semantic analysis derive resource requirements?

Contracts

Can contracts constrain it?

Policies

Can policies constrain it?

Provenance

What source information must remain traceable?

IR

Where does the semantic meaning eventually go?

Diagnostics

What errors are required?

Tests

What positive, negative, boundary and scalability tests are required?

Compatibility

How does versioning/deprecation work?

Completion

What proves the file is finished?

---

106. Dependency-First Completion Order

The type directory must be implemented in dependency order rather than alphabetical order.

Stage 1 — orchestration

README.md
types.g4

Stage 2 — lexical contract

canonical lexer/token definitions

Stage 3 — fundamental types

primitive.g4
named.g4

Stage 4 — type parameters

generic.g4
bounds.g4
constraints.g4
type-constraints.g4

Stage 5 — structural types

tuple.g4
array.g4
slice.g4
map.g4
record.g4

Stage 6 — algebraic/result types

option.g4
result.g4
never.g4
sum.g4
union.g4
algebraic-types.g4

Stage 7 — callable/reference types

function.g4
reference.g4
pointer.g4

Stage 8 — ownership/type semantics

linear.g4
affine.g4
associated.g4
type-class.g4
variance.g4
functional dependencies.g4

Stage 9 — advanced types

dependent.g4
existential-types.g4
higher-kinded-types.g4
path-dependent-types.g4
type-families.g4
type-providers.g4
advanced-type-level-computation.g4

Stage 10 — semantic modifiers

temporal.g4
effectful.g4
resource.g4
capability.g4

Stage 11 — domain types

classical.g4
quantum.g4
hardware.g4

Stage 12 — normalization

Resolve overlapping/legacy files.

Stage 13 — cross-repository integration

Integrate:

AST
semantic analysis
effects
resources
policies
contracts
provenance
IR
quantum::ir
HDL
hardware

Stage 14 — conformance

Run the complete type test matrix.

---

107. Current Duplicate/Legacy Normalization Matrix

The following files require explicit ownership classification before final production generation.

File| Required role
"types.g4"| canonical orchestrator
"primitive.g4"| canonical primitive delegate
"named.g4"| canonical named-type delegate
"generic.g4"| canonical generic delegate
"function.g4"| canonical function-type delegate
"tuple.g4"| canonical tuple delegate
"array.g4"| canonical array delegate
malformed "array-types..." entry| repository hygiene defect; remove/fix
"slice.g4"| canonical slice delegate
"map.g4"| canonical map delegate
"map-types.g4"| compatibility/deprecated unless proven otherwise
"option.g4"| canonical option delegate
"optional.g4"| compatibility/deprecated unless proven otherwise
"option-types.g4"| compatibility/deprecated unless proven otherwise
"result.g4"| canonical result delegate
"record.g4"| canonical record delegate
"sum.g4"| canonical sum delegate
"union.g4"| canonical union delegate
"algebraic-types.g4"| algebraic composition/delegation
"composite-types.g4"| composition/compatibility; no duplicate constructors
"reference.g4"| canonical reference delegate
"pointer.g4"| canonical pointer delegate
"linear.g4"| canonical linear qualifier
"affine.g4"| canonical affine qualifier
"dependent.g4"| canonical dependent-type delegate
"associated.g4"| canonical associated-type delegate
"associated types.g4"| compatibility/deprecated unless proven otherwise
"type-class.g4"| canonical type-class delegate
"existential-types.g4"| canonical existential delegate
"bounds.g4"| canonical bounds delegate
"constraints.g4"| constraint composition/compatibility
"type-constraints.g4"| canonical type-constraint delegate
"variance.g4"| canonical variance delegate
"functional dependencies.g4"| canonical functional-dependency delegate
"higher-kinded-types.g4"| canonical advanced generic delegate
"path-dependent-types.g4"| canonical path-dependent delegate
"type-families.g4"| canonical type-family delegate
"type-providers.g4"| controlled advanced extension
"advanced-type-level-computation.g4"| canonical advanced type-level computation
"temporal.g4"| canonical temporal delegate
"effectful.g4"| canonical effectful-type delegate
"resource.g4"| canonical resource-type delegate
"capability.g4"| canonical capability-type delegate
"classical.g4"| canonical classical-domain delegate
"quantum.g4"| canonical quantum-domain delegate
"hardware.g4"| canonical hardware-domain delegate

The exact classification must be verified against imports and consumers before modifying any grammar.

---

108. No Silent Duplicate Rule Ownership

If two files currently define the same rule:

ruleName

that is a defect.

The production resolution must be:

one canonical owner
+
zero or more compatibility façades

not:

two canonical owners

---

109. Compatibility Façade Rule

A compatibility grammar may expose an old entry point, but it must delegate to the canonical implementation.

It must not copy the complete implementation.

This prevents future divergence.

---

110. Deprecated File Rule

A deprecated grammar file must document:

DEPRECATED
replacement
reason
migration path
compatibility period/version
test coverage

It must not continue introducing new syntax.

---

111. Historical File Rule

Historical grammar files may preserve prior designs for reference.

They must not be imported into the production grammar.

---

112. Experimental File Rule

Experimental type features must be explicitly marked.

They must not accidentally become reachable through the stable universal type grammar.

---

113. No Application Keyword Explosion

The type system must not grow a keyword for every domain/application.

Do not add universal type keywords merely for:

vision
sentiment
robotics
blockchain
payments
VR
AR
administration
legal

Use:

libraries
named types
generic types
dialects
capabilities
policies

instead.

---

114. Reasoning and Learning Integration

The type system must be capable of supporting semantic types used by:

reasoning
inference
deduction
learning
adaptation
knowledge
evidence
uncertainty
decisions
agents

without making these concepts universal primitive type constructors.

For example:

Evidence<T>
Model<T>
Distribution<T>
Decision<T>
Agent<T>

can be ordinary library/domain types.

---

115. Adaptive Computation

An adaptive program may have types whose semantic requirements differ depending on a selected execution strategy.

The type system must preserve the invariant:

same source meaning

while downstream execution may select:

CPU
GPU
FPGA
ASIC
accelerator
QPU
simulator
distributed

based on declared requirements and available capabilities.

---

116. Deterministic Type Checking

Type checking must not depend on:

- current hardware;
- random choices;
- current time;
- external network responses;
- nondeterministic iteration;
- undeclared environment state.

Where external information is genuinely part of a type-provider or dialect mechanism, the compiler must make the dependency explicit and preserve provenance/versioning.

---

117. Reproducibility

The same:

source
language version
grammar version
dialect versions
dependency versions
type-provider inputs
compiler configuration

must produce semantically equivalent type analysis.

Type resolution must not silently change because a machine happens to have different resources.

---

118. Diagnostics Contract

Every type grammar must support diagnostics for:

- malformed syntax;
- missing arguments;
- malformed generic application;
- malformed tuple;
- malformed array;
- malformed function type;
- malformed reference;
- malformed pointer;
- malformed type projection;
- invalid type extension syntax.

Semantic diagnostics must separately handle:

- unknown type;
- unsatisfied bound;
- invalid generic argument;
- wrong arity;
- incompatible types;
- invalid associated type;
- invalid constraint;
- invalid dependent relation;
- unavailable capability;
- unavailable resource;
- policy violation.

The parser must not attempt to solve semantic errors.

---

119. Error Ownership

Syntax error
    → parser/grammar

Unknown identifier
    → name resolution

Wrong type
    → semantic type checker

Unsatisfied bound
    → constraint/type-class subsystem

Unavailable capability
    → capability subsystem

Insufficient resource
    → resource subsystem

Policy violation
    → policy subsystem

Invalid effect
    → effect subsystem

Invalid quantum realization
    → quantum semantic/resource/backend subsystem

This separation prevents diagnostic duplication.

---

120. Type Test Architecture

The type subsystem requires tests at multiple levels.

grammar/tests/types/
├── lexical/
├── parser/
├── ast/
├── semantic/
├── generics/
├── composites/
├── algebraic/
├── references/
├── ownership/
├── dependent/
├── advanced/
├── classical/
├── quantum/
├── hardware/
├── resources/
├── capabilities/
├── effects/
├── contracts/
├── policies/
├── provenance/
├── compatibility/
├── diagnostics/
├── scalability/
├── determinism/
└── cross-domain/

---

121. Required Positive Tests

At minimum:

int
bool
string
User
module::User

Vec<T>
Map<K, V>
Option<T>
Result<T, E>

()
(T,)
(T, U)
(T, U, V)

[T]
[T; N]

fn() -> T
fn(T) -> U
fn(T, U) -> V

&T
&mut T

Pointer<T>

linear T
affine T

Tensor<T>
Matrix<T, Rows, Columns>

Qubit
QRegister<N>
QuantumState<T>

Hardware<T>
Resource<T>
Capability<T>

Exact syntax must follow the canonical grammar and specification.

---

122. Required Nested Tests

Test compositions such as:

Vec<Option<T>>
Map<K, Vec<Result<T, E>>>
Tensor<Vec<T>>
Result<QuantumState<T>, E>
Vec<Hardware<T>>
Model<Tensor<T>>
QRegister<N>

and every other semantically legal cross-family composition.

---

123. Required Negative Tests

At minimum:

<>
<T,
<T>>
[T;
[T;]
fn(
fn(T
&T
&mut
Map<K,
Map<,V>
Result<T,>

plus all malformed constructs discovered during grammar generation.

---

124. Required Semantic Negative Tests

Examples:

unknown type
wrong generic arity
invalid type argument
unsatisfied bound
invalid associated type
invalid projection
invalid variance
invalid dependent constraint
invalid ownership qualifier
invalid capability type
invalid resource type

---

125. Required Cross-Domain Tests

At minimum:

classical + quantum
classical + HDL
classical + AI
quantum + AI
quantum + hardware
tensor + quantum
distributed + resource
networking + capability
hardware + resource
effect + type
policy + capability
contract + type
provenance + type

---

126. Required POCO-REAF Test

A mandatory integration test must demonstrate a single source program whose type structure can be analyzed without changing the source for different target classes:

tiny
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
future target

The test must prove that the type meaning remains invariant.

Target-specific realization is permitted to differ.

---

127. Resource-Scaling Tests

Tests must use symbolic requirements rather than repository constants.

For example:

T
N
Rows
Columns
Shape
RequiredMemory
RequiredWidth
Qubits
Nodes

The test infrastructure must verify that larger values do not require grammar modification.

---

128. Generated Scalability Tests

The test suite should generate type structures programmatically.

Examples:

nested generics
nested tuples
nested records
large qualified names
large generic argument lists
symbolic dimensions
large type compositions

The tests may use implementation-controlled workloads.

They must not redefine those workloads as language limits.

---

129. Determinism Tests

Repeated parsing/type-checking of identical source with identical configuration must produce equivalent:

AST
diagnostics
semantic type results
constraint results
provenance structure

without dependence on host resource ordering.

---

130. Compatibility Tests

Every stable type feature must be tested against:

current grammar version
supported previous versions
deprecation rules
dialect versions
AST compatibility
semantic compatibility
IR compatibility

where those compatibility guarantees exist.

---

131. Rust Integration Tests

Rust-side tests must run on:

Rust 1.97+
Rust 2021

and must use safe Rust only.

The type subsystem must not require compiler-version-specific unsafe workarounds.

---

132. ANTLR Generation Tests

CI must verify:

grammar/types/types.g4

can be composed with the canonical parser without:

- import cycles;
- undefined rules;
- duplicate rule ownership;
- undefined tokens;
- token conflicts;
- ambiguous composition introduced by a delegate;
- invalid grammar imports.

---

133. Grammar Import Audit

CI should inspect the type directory and reject:

delegate → Types

where that would create a circular import.

CI should also detect multiple definitions of the same canonical public rule.

---

134. Hard-Coding Audit

CI must scan the type subsystem for forbidden capacity patterns.

At minimum:

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

It should also flag suspicious target-specific constants for human review.

---

135. Domain-Neutrality Audit

No type grammar should contain hard-coded:

CPU model
GPU model
FPGA model
ASIC model
QPU model
vendor
physical qubit identifier
coupling map
calibration
machine topology

unless the information is explicitly part of a user-declared target/dialect abstraction rather than universal type semantics.

---

136. Quantum Audit

The type directory must never:

- enumerate every quantum gate;
- select physical qubits;
- construct schedules;
- perform routing;
- select QEC codes;
- generate ZQN;
- select a QPU;
- create a second quantum IR.

Quantum type semantics terminate at the semantic/"quantum::ir" boundary.

---

137. HDL Audit

The type directory must not impose universal:

bus width
register count
pipeline depth
memory capacity
clock count
device count

unless explicitly represented as source-level program semantics.

---

138. Resource Audit

A type may imply a resource requirement.

A type must not itself discover or reserve resources.

Correct:

type
 ↓
semantic requirement
 ↓
resource analysis

Incorrect:

type
 ↓
allocate GPU

---

139. Capability Audit

A type may participate in capability reasoning.

It must not directly query hardware.

Correct:

type
 ↓
capability implication
 ↓
capability analysis

Incorrect:

type
 ↓
query physical device

---

140. Effect Audit

A type may carry effect information where standardized.

It must not execute effects.

---

141. Policy Audit

A type may be subject to policies.

It must not evaluate authorization itself.

---

142. Provenance Audit

Type syntax must preserve source location through the frontend.

Semantic provenance must be attached by the compiler pipeline.

No grammar file should invent machine/time/environment provenance.

---

143. Integration with "grammar/expressions/"

Expressions consume:

typeExpression

from the canonical type grammar.

They must not redefine types.

Expression typing occurs during semantic analysis.

---

144. Integration with "grammar/statements/"

Statements consume type syntax where declarations/annotations require it.

They must import the canonical type hierarchy.

They must not create alternate type rules.

---

145. Integration with "grammar/declarations/"

Declarations consume:

typeExpression

for:

- variables;
- fields;
- parameters;
- aliases;
- associated types;
- generic declarations.

---

146. Integration with "grammar/functions/"

Function declarations consume:

functionType

or its canonical constituent rules.

Function declarations do not redefine function-type syntax.

---

147. Integration with "grammar/modules/"

Qualified names used in types must use the same name/path architecture as modules.

There must not be separate module-name and type-name path grammars unless their semantic distinction is explicitly required.

---

148. Integration with "grammar/memory/"

Reference/pointer/ownership types interact with memory semantics.

Memory grammar owns memory operations.

Type grammar owns type structure.

---

149. Integration with "grammar/concurrency/"

Concurrency constructs may use types such as:

Message<T>
Channel<T>
Task<T>
Future<T>
Actor<T>

These should normally be library/domain types rather than special universal primitive types.

---

150. Integration with "grammar/ai/"

AI constructs consume the universal type system.

Examples:

Model<T>
Dataset<T>
Distribution<T>
Evidence<T>
Decision<T>
Agent<T>

should use generic/named/extension mechanisms where possible.

---

151. Integration with "grammar/data/"

Data types must use:

records
tuples
maps
arrays
generics
named types
schemas

rather than creating a second type system.

---

152. Integration with "grammar/distributed/"

Distributed constructs consume the same types.

A distributed semantic model may add:

location
consistency
replication
ownership
availability

but these are semantic properties, not reasons to create another type AST.

---

153. Integration with "grammar/networking/"

Networking constructs consume:

Message<T>
Stream<T>
Endpoint<T>
Protocol<T>

through ordinary type composition.

Network realization remains downstream.

---

154. Integration with "grammar/security/"

Security may constrain types through:

- capability labels;
- confidentiality/integrity properties;
- policy types;
- authorization markers.

Security semantics remain owned by security/policy systems.

---

155. Integration with "grammar/resources/"

This is the primary downstream consumer for resource-related type implications.

Example conceptual path:

QRegister<N>
     ↓
semantic quantum type
     ↓
resource implication
     ↓
requires qubits >= N

The type grammar does not implement the requirement solver.

---

156. Integration with "grammar/effects/"

Example:

effectful type
     ↓
effect semantic model
     ↓
effect checking

No duplicate effect vocabulary.

---

157. Integration with "grammar/validation/"

Validation consumes type information for:

assertions
contracts
properties
requires
ensures
invariants
assumptions
guarantees

The type grammar only supplies the type expression.

---

158. Integration with "grammar/policies/"

Policies may constrain:

type
effect
capability
resource
execution

The policy engine consumes semantic types.

---

159. Integration with "grammar/compile/"

Compilation consumes resolved type information for:

specialization
optimization
lowering
target selection
reproducibility

The type grammar itself does not choose a target.

---

160. Integration with "grammar/execution/"

Execution consumes resolved type and resource semantics.

The type grammar does not select:

runtime
scheduler
device
QPU
cluster

---

161. Integration with "grammar/interoperability/"

Interoperability may introduce:

foreign types
ABI types
external types

but must use the canonical type model.

---

162. Integration with "grammar/metaprogramming/"

Reflection and compile-time type computation must consume the canonical type AST/semantic model.

They must not create a second type representation.

---

163. Integration with "grammar/dialects/"

Dialects may extend types through registered constructs.

A dialect must identify:

dialect
version
type constructor
AST mapping
semantic owner
compatibility

---

164. Integration with AST

Every stable type construct requires:

grammar
AST
semantic
tests

A type feature is incomplete if only the ".g4" file exists.

---

165. Integration with Semantic Analysis

Semantic analysis must consume the domain-neutral "TypeExpr".

It is responsible for:

resolution
substitution
inference
unification
bounds
constraints
ownership
lifetimes
effect compatibility
capability implications
resource implications
policy compatibility
contract compatibility

---

166. Integration with Classical IR

Types required by classical computation must lower through the canonical semantic model to Classical IR.

The type grammar does not emit Classical IR directly.

---

167. Integration with "quantum::ir"

Quantum types must lower through semantic quantum analysis to:

quantum::ir

There is no type-specific quantum IR.

---

168. Integration with HDL

HDL semantic types eventually participate in:

HDL semantic representation
↓
synthesis/lowering
↓
hardware realization

The source type remains target-neutral.

---

169. Integration with Hardware Abstraction

Hardware type semantics eventually participate in:

resource discovery
capability negotiation
placement
routing
scheduling
HAL

No hardware allocation happens in the grammar.

---

170. Integration with QEC and Resilience

Quantum type semantics may influence resource/resilience planning.

The path remains:

quantum type
 ↓
semantic quantum model
 ↓
quantum::ir
 ↓
resilience/QEC

The type grammar must not define QEC implementation details.

---

171. Integration with ZQN

Types do not generate ZQN.

Resolved semantic operations and resources eventually reach:

ZQN

through the compiler pipeline.

---

172. Integration with HAL

The HAL realizes target-specific representations.

The type grammar must remain completely unaware of:

CPU model
GPU model
FPGA family
ASIC process
QPU vendor
simulator implementation
cluster provider

---

173. Independent File Completion Template

Every type file should use this template in its own header/documentation:

FEATURE CONTRACT

Purpose:
    ...

Owns:
    ...

Does Not Own:
    ...

Depends On:
    ...

Exports:
    ...

Consumed By:
    ...

Lexer Dependencies:
    ...

Grammar Dependencies:
    ...

AST Contract:
    ...

Semantic Contract:
    ...

Type Interaction Contract:
    ...

Effect Contract:
    ...

Capability Contract:
    ...

Resource Contract:
    ...

Contract/Validation Contract:
    ...

Policy Contract:
    ...

Provenance Contract:
    ...

IR Contract:
    ...

Quantum Boundary:
    ...

HDL Boundary:
    ...

Backend Boundary:
    ...

Diagnostics:
    ...

Positive Tests:
    ...

Negative Tests:
    ...

Boundary Tests:
    ...

Scalability Tests:
    ...

Determinism Tests:
    ...

Compatibility:
    ...

Hard-Coding Audit:
    ...

Completion Criteria:
    ...

---

174. File Completion Rule

A file can be declared complete even if downstream implementation is scheduled later, provided its integration contract is already explicit.

For example:

quantum.g4

can be complete once it specifies exactly how its syntax maps to:

TypeExpr
→ semantic quantum type
→ quantum::ir

even if a later compiler phase still needs implementation work.

This prevents repeated redesign.

---

175. No Cross-File Surprise Rule

After a file is declared complete:

A later file may:

- implement its documented extension point;
- consume its documented exports;
- implement its downstream semantic owner;
- add tests prescribed by its contract.

A later file must not require changing the completed file merely because the architecture was not specified.

If such a change is necessary, that indicates a missing contract and requires an explicit architecture review.

---

176. Production Readiness Matrix

The type subsystem is production-ready only when every canonical feature has:

Layer| Required
Specification| Yes
Lexer| Yes, where tokens are needed
Grammar| Yes
AST| Yes
AST validation| Yes
Semantic analysis| Yes
Type checking| Yes
Constraint checking| Yes
Effect integration| Yes, where applicable
Capability integration| Yes, where applicable
Resource integration| Yes, where applicable
Contract integration| Yes, where applicable
Policy integration| Yes, where applicable
Provenance| Yes
Canonical IR integration| Yes
Tests| Yes
Diagnostics| Yes
Compatibility| Yes
Scalability| Yes
Determinism| Yes
Hard-coding audit| Yes

---

177. Production Definition of Done

"grammar/types/" is production-ready only when:

1. "types.g4" is the sole universal type composition root.
2. "typeExpression" has one owner.
3. Every specialized constructor has one canonical owner.
4. No circular grammar imports exist.
5. Duplicate files are explicitly classified.
6. Compatibility façades do not duplicate canonical implementations.
7. Every stable type maps to the canonical AST.
8. Every stable type has semantic handling.
9. Every relevant type has IR integration.
10. Quantum types use "quantum::ir".
11. No second quantum IR exists.
12. Resource semantics remain outside grammar.
13. Capability semantics remain outside grammar.
14. Effect semantics remain outside grammar.
15. Policy semantics remain outside grammar.
16. Contract semantics remain outside grammar.
17. Provenance remains outside grammar.
18. Hardware realization remains outside grammar.
19. No universal physical capacity constants exist.
20. Symbolic quantities remain symbolic.
21. Generic arity is open-ended.
22. Composite structures are recursively composable.
23. Domain extensions do not require unnecessary core keyword additions.
24. Parser behavior is deterministic.
25. Semantic analysis is deterministic under identical inputs/configuration.
26. Rust implementation remains compatible with Rust 1.97+.
27. Rust implementation remains safe Rust only.
28. Positive tests pass.
29. Negative tests pass.
30. Boundary tests pass.
31. Cross-domain tests pass.
32. Scalability tests pass.
33. Compatibility tests pass.
34. ANTLR generation succeeds.
35. Repository-wide grammar integration succeeds.
36. AST integration succeeds.
37. semantic type integration succeeds.
38. Classical IR integration succeeds where applicable.
39. "quantum::ir" integration succeeds where applicable.
40. HDL/hardware integration succeeds where applicable.
41. resource/capability integration succeeds where applicable.
42. effects integration succeeds where applicable.
43. policy/contract integration succeeds where applicable.
44. provenance integration succeeds.
45. hard-coding audit passes.
46. duplicate-rule audit passes.
47. import-cycle audit passes.
48. generated parser conformance passes.
49. repository type examples pass.
50. the POCO-REAF integration test passes.

---

178. Final Ownership Model

The entire "grammar/types/" directory must converge on this model:

                         grammar/types/README.md
                                  │
                    DIRECTORY ORCHESTRATION CONTRACT
                                  │
                                  ▼
                         grammar/types/types.g4
                                  │
                       CANONICAL TYPE COMPOSITION
                                  │
          ┌───────────────────────┼────────────────────────┐
          │                       │                        │
          ▼                       ▼                        ▼
     Fundamental             Composite              Advanced
       Types                  Types                   Types
          │                       │                        │
          │                       │                        │
 primitive                 tuple/array/map          dependent
 named                     option/result            existential
 generic                   record/sum               HKT
 function                  union                    associated
 reference                 algebraic                type families
 pointer                   etc.                     type-level
          │                       │                        │
          └───────────────────────┼────────────────────────┘
                                  │
                                  ▼
                           Domain Types
                                  │
                   ┌──────────────┼───────────────┐
                   ▼              ▼               ▼
               Classical       Quantum         Hardware
                   │              │               │
                   └──────────────┼───────────────┘
                                  │
                                  ▼
                              TypeExpr
                                  │
                                  ▼
                         Semantic Type Model
                                  │
          ┌───────────────┬───────┼────────┬────────────────┐
          ▼               ▼       ▼        ▼                ▼
       Effects        Resources Capabilities Contracts    Policies
          │               │       │        │                │
          └───────────────┴───────┼────────┴────────────────┘
                                  ▼
                             Provenance
                                  │
                                  ▼
                       Canonical Semantic Model
                                  │
                     ┌────────────┴────────────┐
                     ▼                         ▼
                Classical IR              quantum::ir
                     │                         │
                     └────────────┬────────────┘
                                  ▼
                           Optimization
                                  ▼
                              Lowering
                                  ▼
                              Routing
                                  ▼
                            Scheduling
                                  ▼
                         Resilience / QEC
                                  ▼
                                 ZQN
                                  ▼
                                 HAL
                                  ▼
                          Target Realization

---

179. The Most Important Architectural Rule

The entire type subsystem must preserve this distinction:

TYPE
≠
RESOURCE

TYPE
≠
CAPABILITY

TYPE
≠
EFFECT

TYPE
≠
POLICY

TYPE
≠
CONTRACT

TYPE
≠
HARDWARE

TYPE
≠
IR

TYPE
≠
RUNTIME REPRESENTATION

Instead:

TYPE
 │
 ├── participates in
 │
 ├── effects
 ├── capabilities
 ├── resources
 ├── contracts
 ├── policies
 └── provenance
 │
 ▼
SEMANTIC TYPE
 │
 ▼
CANONICAL SEMANTIC MODEL
 │
 ├── Classical IR
 └── quantum::ir
 │
 ▼
TARGET-INDEPENDENT COMPILATION
 │
 ▼
TARGET REALIZATION

This is what permits Zamani to remain one language instead of becoming a collection of unrelated domain languages.

---

180. Final Scalability Principle

The type system must scale by composition and symbolic description, not by enumerating hardware.

Correct:

Tensor<T, Shape>
QRegister<N>
Matrix<T, Rows, Columns>
Vec<T>
Map<K, V>
Resource<T>
Capability<T>
Hardware<T>

Incorrect:

TensorRank32
QRegister1024
GPU24GB
CPU8Core
FPGA100kLUT

The former describes program meaning.

The latter embeds a particular realization.

---

181. Final POCO-REAF Principle

The goal is not:

«one source program that magically executes on every physically possible machine without checking feasibility.»

The goal is:

«one source program whose semantic type system does not need to be rewritten merely because the available computational substrate changes.»

Therefore:

same source
    ↓
same type meaning
    ↓
same semantic requirements
    ↓
different target realization

is valid.

While:

same source
    ↓
different type meaning because hardware changed

is invalid.

---

182. Final Repository Contract

"grammar/types/" is complete only when it can be treated as a stable contract between:

source language
        │
        ▼
parser
        │
        ▼
AST
        │
        ▼
semantic type system
        │
        ├── effects
        ├── capabilities
        ├── resources
        ├── contracts
        ├── policies
        └── provenance
        │
        ▼
canonical semantic model
        │
        ├── Classical IR
        └── quantum::ir
        │
        ▼
compiler
        │
        ▼
runtime / execution
        │
        ▼
hardware abstraction
        │
        ▼
target

No file under "grammar/types/" should need to know the implementation details of a future CPU, GPU, FPGA, ASIC, accelerator, QPU, simulator, cluster, or other computational substrate.

No future computational domain should require the type system to be redesigned merely because it introduces a new named type.

No type feature should be considered complete merely because its ANTLR rule parses.

Every feature must have a complete path:

SPECIFICATION
    ↓
LEXER
    ↓
GRAMMAR
    ↓
AST
    ↓
VALIDATION
    ↓
SEMANTICS
    ↓
TYPE CHECKING
    ↓
CONSTRAINTS
    ↓
EFFECTS / CAPABILITIES / RESOURCES
    ↓
CONTRACTS / POLICIES
    ↓
PROVENANCE
    ↓
CANONICAL IR
    ↓
LOWERING
    ↓
TARGET REALIZATION

That is the production contract for the Zamani type system.

---

183. Completion Statement

When all requirements in this document are satisfied, "grammar/types/" provides:

one type language
one universal type composition root
one domain-neutral AST boundary
one semantic type model
one canonical integration architecture
open-ended composition
symbolic resource quantities
target-independent quantum types
target-independent hardware types
classical/quantum/HDL interoperability
resource/capability integration
effect integration
contract integration
policy integration
provenance integration
deterministic parsing
reproducible semantic analysis
safe Rust implementation
Rust 1.97+ compatibility
no universal physical capacity assumptions
no duplicate type authorities
no competing quantum IR
no application-specific keyword explosion
POCO-REAF compatibility

The result is a type system capable of serving computation from the smallest supported realization to arbitrarily larger realizations as resources and capabilities permit, without making physical scale part of the universal language grammar.