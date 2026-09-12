Zamani Type Grammar

Production Type-System Grammar Contract

This directory defines the source-level type syntax contract for the Zamani programming language.

It is part of the authoritative Zamani grammar architecture and exists to provide a stable, extensible, hardware-independent syntax for expressing types across:

- classical computing;
- quantum computing;
- hybrid quantum-classical computing;
- hardware/software co-design;
- HDL;
- embedded systems;
- systems programming;
- parallel computing;
- distributed computing;
- HPC;
- AI/ML;
- tensor and numerical computing;
- accelerators;
- networking;
- cryptography;
- scientific computing;
- future computational domains.

The type grammar is designed around the Zamani principle:

«Zamani describes computation, intent, capabilities, constraints, and semantics—not arbitrary limitations of the machine currently available.»

The type grammar therefore participates in:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).

A type describes the meaning and requirements of a program entity. It must not accidentally encode the capabilities or limitations of one particular machine.

---

1. Scope

The "grammar/types/" directory owns source-level type syntax.

It defines how a Zamani source program spells concepts such as:

- primitive types;
- named types;
- qualified types;
- generic types;
- type arguments;
- tuples;
- arrays;
- slices;
- references;
- pointers;
- functions;
- option types;
- result types;
- algebraic types;
- resource types;
- quantum types;
- classical types;
- hardware-related types;
- type-level values;
- value-parameterized types;
- type constraints;
- type qualifiers;
- future extensible type constructors.

The directory establishes syntax.

It does not perform semantic type checking.

It does not select physical hardware.

It does not allocate resources.

It does not choose a quantum processor.

It does not choose a CPU architecture.

It does not choose a GPU.

It does not determine an FPGA layout.

It does not perform QEC.

It does not model quantum noise.

It does not perform scheduling.

It does not perform routing.

It does not perform optimization.

It does not create canonical IR.

---

2. Architectural Position

The type grammar participates in the following pipeline:

Zamani source
    |
    v
canonical lexer
    |
    v
type grammar
    |
    v
parse tree
    |
    v
frontend AST
    |
    v
structural validation
    |
    v
name resolution
    |
    v
type resolution
    |
    v
type checking
    |
    +------------------+
    |                  |
    v                  v
classical semantics   quantum semantics
    |                  |
    +--------+---------+
             |
             v
canonical semantic representation
             |
             +--------------------+
             |                    |
             v                    v
       classical IR          quantum::ir
             |                    |
             +----------+---------+
                        |
                        v
                optimization
                        |
                        v
                routing / mapping
                        |
                        v
                  scheduling
                        |
                        v
              hardware abstraction
                        |
                        v
                    runtime

The grammar must never create a reverse dependency:

grammar -> IR -> grammar

or:

grammar -> runtime -> grammar

or:

quantum type grammar -> hardware discovery -> grammar

The dependency direction is always:

syntax -> semantic interpretation -> IR -> realization

---

3. Authoritative Files

The type grammar is distributed across focused files.

The primary entry point is:

grammar/types/types.g4

Supporting grammar files define focused type families.

The current repository already contains type grammar files including:

grammar/types/types.g4
grammar/types/primitive-types.g4
grammar/types/composite-types.g4
grammar/types/function-types.g4
grammar/types/generic-types.g4
grammar/types/array-types.g4
grammar/types/map-types.g4
grammar/types/option-types.g4
grammar/types/result-types.g4
grammar/types/algebraic-types.g4
grammar/types/reference-types.g4
grammar/types/resource-types.g4
grammar/types/quantum-types.g4
grammar/types/classical-types.g4
grammar/types/hardware-types.g4

The repository's current "types.g4" already establishes the important ownership boundary: type grammar owns type-expression syntax while semantic resolution, ownership checking, resource allocation, quantum allocation, physical placement, hardware selection, scheduling, optimization, QEC, ZQN, canonical quantum IR, classical IR, runtime representation, ABI layout, and backend selection remain outside the grammar.

---

4. "types.g4" Is the Type-System Grammar Root

"types.g4" is the central parser grammar for source-level type expressions.

It owns:

- type-expression composition;
- primitive type references;
- named types;
- qualified type paths;
- generic applications;
- tuple syntax;
- arrays;
- slices;
- function types;
- references;
- pointers;
- optional syntax;
- type-level values;
- value-parameterized types;
- quantum type references;
- type qualifiers;
- recursive type composition.

It does not own the implementation semantics of those constructs.

Every supporting grammar must integrate into the public type-expression contract established by "types.g4".

Supporting files must not silently create incompatible parallel definitions of "typeExpression".

---

5. Type Grammar Ownership Model

The ownership hierarchy is:

types/
│
├── types.g4
│   └── canonical type-expression composition
│
├── primitive-types.g4
│   └── primitive source types
│
├── composite-types.g4
│   └── structural type composition
│
├── generic-types.g4
│   └── generic declarations/applications
│
├── function-types.g4
│   └── function/closure type syntax
│
├── tuple-types.g4
│   └── tuple syntax
│
├── array-types.g4
│   └── array/slice syntax
│
├── map-types.g4
│   └── map type syntax
│
├── option-types.g4
│   └── optional type syntax
│
├── result-types.g4
│   └── result type syntax
│
├── algebraic-types.g4
│   └── algebraic type syntax
│
├── reference-types.g4
│   └── references/borrows
│
├── resource-types.g4
│   └── resource-oriented type syntax
│
├── quantum-types.g4
│   └── quantum semantic type syntax
│
├── classical-types.g4
│   └── classical domain types
│
├── hardware-types.g4
│   └── hardware-domain type syntax
│
└── type-constraints.g4
    └── type constraint syntax

No file may become the owner of concepts belonging to another subsystem merely because the syntax happens to be used there.

---

6. Important Ownership Rule

A type grammar rule must answer:

«"How is this type written?"»

It must not answer:

«"Can the current machine execute this type?"»

For example:

Qubit

means that the source program requires a quantum value/resource with the semantic identity of a qubit.

It does not mean:

physical qubit 0

It does not mean:

device = some_vendor_device

It does not mean:

topology = linear

It does not mean:

maximum qubits = 32

It does not mean:

maximum qubits = 64

It does not imply any fixed physical architecture.

Those decisions belong downstream.

---

7. POCO-REAF Type Principle

Types must preserve source semantics across machines.

For example:

fn compute<T>(value: T) -> T

must not acquire a different source-level meaning merely because it executes on:

- one CPU;
- many CPUs;
- a GPU;
- an FPGA;
- an ASIC;
- a quantum-classical system;
- a cluster;
- a cloud environment;
- an embedded device;
- a future architecture.

The compiler may choose different representations.

The runtime may choose different resources.

The scheduler may choose different execution plans.

The hardware layer may choose different devices.

But the source type's semantic identity must remain stable.

---

8. No Machine-Capacity Limits

The type grammar MUST NOT encode fixed machine limits.

Forbidden examples include:

MAX_QUBITS = 32
MAX_QUBITS = 64
MAX_CORES = 16
MAX_THREADS = 1024
MAX_DEVICES = 8
MAX_NODES = 4096
MAX_MEMORY = ...
MAX_REGISTER_COUNT = ...
MAX_TENSOR_RANK = 8
MAX_GENERIC_PARAMETERS = 16

Likewise, grammar rules must not contain hidden finite restrictions equivalent to those values.

The grammar must not use a fixed number of repetitions where arbitrary semantic cardinality is intended.

Prefer recursive or unbounded grammar composition.

For example:

typeArgumentList
    : typeArgument
      (COMMA typeArgument)*
      COMMA?
    ;

rather than:

typeArgumentList
    : typeArgument
    | typeArgument COMMA typeArgument
    | typeArgument COMMA typeArgument COMMA typeArgument
    ;

The latter creates an accidental language ceiling.

---

9. "Infinity" Means No Language-Imposed Finite Ceiling

"Scale to infinity" is interpreted architecturally as:

«the grammar does not impose an arbitrary finite machine-size ceiling; actual execution is bounded only by the resources and policies of the compilation/execution environment.»

Actual implementation limits may exist because of:

- available memory;
- parser stack resources;
- compiler resource budgets;
- operating-system limits;
- runtime resources;
- target capabilities;
- distributed-system capacity;
- provider limits.

Those are not language grammar semantics.

Such limits must be represented by explicit policies or diagnostics rather than encoded as source grammar restrictions.

---

10. Type-Level Values

Type-level values are permitted where they are necessary to describe semantic dimensions.

Examples include:

Vector<T, N>
Matrix<T, Rows, Columns>
Tensor<T, N, M, K>
Array<T, N>

The grammar must preserve the expression.

It must not evaluate it.

It must not decide whether a value fits into a machine.

For example:

Vector<float, N>

does not mean:

N <= 1024

unless a downstream semantic or target-specific constraint explicitly establishes such a requirement.

---

11. Type Constraints

"type-constraints.g4" owns syntactic representation of type constraints.

It must not become a second expression grammar.

It must consume or reference the canonical expression/type grammar where appropriate.

The architectural boundary is:

type constraint syntax
        |
        v
type semantic representation
        |
        v
type checker / capability checker

not:

type constraint grammar
        |
        v
custom mini expression language

The repository's existing constraint architecture explicitly follows the principle that constraint operands should feed the expression/type semantic model and that the constraint grammar must not create a second expression language.

---

12. Constraint Categories

The language must distinguish at least:

type constraints
resource constraints
capability requirements
execution constraints
hardware constraints
placement constraints
performance constraints
security constraints
effect constraints

These concepts must not be collapsed into one grammar construct.

For example:

T: Numeric

is fundamentally different from:

requires capability(quantum)

and different again from:

requires resource(qubits)

and different again from:

constraint latency < expression

The parser preserves these distinctions.

Semantic analysis determines their validity.

---

13. Generic Type Constraints

Generic constraints must support extensible semantic predicates without encoding target assumptions.

Examples:

T: Numeric
T: Comparable
T: QuantumState
T: Sendable
T: HardwareCompatible

The grammar should permit named constraints without requiring the grammar to know every future trait, interface, capability, or domain.

This is essential for future language evolution.

---

14. Constraints Must Remain Semantic

The grammar must not determine whether a constraint is satisfiable.

For example:

T: Numeric

is syntax.

Whether a particular type satisfies "Numeric" is semantic analysis.

Likewise:

T: QuantumState

is syntax.

Whether "T" actually implements the required semantic contract is resolved downstream.

---

15. Quantum Integration

Quantum type grammar integrates with:

grammar/quantum/
src/quantum/
src/quantum/ir/
src/quantum/qec/
src/quantum/zqn/
src/quantum/scheduling/
src/quantum/optimization/
src/quantum/hardware/
src/quantum/resilience/

The grammar does not duplicate those subsystems.

The flow is:

Quantum source type
        |
        v
quantum-types.g4
        |
        v
frontend AST
        |
        v
semantic quantum type
        |
        v
quantum::ir

"quantum::ir" remains the canonical quantum semantic boundary.

The grammar must never introduce a competing quantum gate/qubit/type IR.

---

16. Quantum Cardinality

There must be no grammar-level finite cardinality limit for quantum values.

Forbidden:

Qubit[32]

as a special fixed grammar construct.

The valid abstraction is something such as:

Qubit
Qubit[]
QubitRegister<N>

where any cardinality is a semantic/type-level value rather than a grammar-imposed machine limit.

A resource requirement belongs to resource semantics.

A physical qubit count belongs to hardware realization.

A logical qubit count belongs to quantum semantic compilation.

A mapping belongs to routing.

A schedule belongs to scheduling.

---

17. Logical vs Physical Quantum Types

The source grammar may distinguish semantic concepts such as:

LogicalQubit
PhysicalQubit

when the language requires developers to explicitly express that distinction.

However, the grammar must not bind a physical type to:

- a physical index;
- device ID;
- vendor;
- topology;
- hardware address;
- calibration;
- timing;
- pulse implementation.

Physical realization belongs to hardware abstraction and routing.

---

18. QEC Integration

Type grammar may represent semantic types associated with error correction.

It must not implement QEC algorithms.

For example, a source-level logical quantum type may eventually carry semantic information consumed by QEC.

But:

surface code

must not cause the type grammar to:

- allocate physical qubits;
- generate syndrome circuits;
- select ancillas;
- schedule stabilizers;
- choose calibration;
- perform decoding.

Those are QEC/compiler responsibilities.

---

19. ZQN Integration

ZQN describes quantum noise/fault semantics.

Type grammar does not own noise models.

A type may be associated semantically with requirements concerning noise tolerance or execution characteristics, but the grammar must not embed ZQN fault semantics into type parsing.

The architectural direction remains:

type syntax
    |
    v
semantic type
    |
    +---- quantum IR
    |
    +---- resource semantics
    |
    +---- capability semantics
    |
    +---- resilience/ZQN integration

---

20. Hardware Integration

Hardware types must describe hardware semantics rather than hardware instances.

For example:

GPU
FPGA
CPU
QuantumDevice
Accelerator

may be valid semantic type categories.

But the grammar must not make:

GPU0
FPGA7
QPU42
device_address

part of ordinary type semantics.

A concrete target may be specified separately through:

target
capability
resource
deployment
placement
configuration

mechanisms.

---

21. Classical Integration

Classical types must support:

- scalar values;
- arbitrary structured data;
- vectors;
- matrices;
- tensors;
- numerical abstractions;
- symbolic values;
- accelerator-compatible types;
- systems-level types.

Machine representation is downstream.

For example:

int

must not inherently mean:

i32

unless Zamani explicitly defines "int" as such in its language specification.

If fixed-width representation is required, it should be expressed explicitly.

---

22. Width Independence

The type grammar must distinguish:

semantic integer

from:

fixed-width integer representation

where the language provides both.

Examples may include:

int
uint
i8
i16
i32
i64
i128

or future arbitrary-width forms.

The grammar must not assume that:

int = 32 bits

or:

int = 64 bits

unless this is an explicit Zamani language semantic contract.

---

23. Hardware and HDL Integration

HDL types may represent:

- signal types;
- clock-related types;
- register types;
- hardware interfaces;
- buses;
- bit vectors;
- parameterized hardware structures.

The grammar must preserve the difference between:

hardware semantic structure

and:

physical implementation.

For example:

BitVector<N>

expresses a semantic width parameter.

It must not imply a particular FPGA family or ASIC technology.

---

24. Resource Types

Resource types may represent computational resources.

Examples:

Resource<T>
Capability<T>
Device<T>
Accelerator<T>

The type grammar does not allocate the resource.

Resource allocation belongs to resource management.

The resource type can express semantic ownership or requirements, while the runtime/compiler determines realization.

---

25. Ownership and Borrowing

Reference syntax belongs to the type grammar where the language exposes it.

Examples:

&T
&mut T

Lifetime syntax may be represented syntactically.

However:

- ownership checking;
- alias analysis;
- borrow checking;
- lifetime validity;
- concurrency safety;

belong to semantic analysis.

The grammar must never encode implementation-specific Rust borrowing behavior merely because the compiler is implemented in Rust.

---

26. Rust Implementation Boundary

Zamani's compiler implementation uses:

Rust 1.97
Rust 1.97.1

as supported implementation baselines.

Compiler/frontend implementation must use:

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

where applicable.

No unsafe Rust is required by the grammar architecture.

The ANTLR grammar itself contains no Rust implementation code.

Rust version constraints are compiler implementation constraints, not source-language type semantics.

---

27. ANTLR Boundary

ANTLR is the parser-generation mechanism.

Type grammar files must follow the repository's canonical ANTLR architecture.

Lexer ownership remains outside parser grammar files.

Parser grammar files must not duplicate lexer rules.

The canonical lexer provides:

- identifiers;
- keywords;
- literals;
- operators;
- punctuation;
- comments;
- source positions.

Type parser grammars consume those tokens.

---

28. Identifier Ownership

Identifier syntax belongs to the lexer/core name system.

Type grammar must not independently define:

- Unicode normalization;
- identifier character ranges;
- maximum identifier length;
- case-folding;
- Unicode policy.

This ensures every language subsystem uses the same naming rules.

---

29. Type Paths

Type paths must be structurally compositional.

Examples:

T
User
std::Vec
std::collections::Map
quantum::State
hardware::Accelerator
future::domain::Type

There must be no fixed namespace depth.

Do not implement:

identifier
| identifier :: identifier
| identifier :: identifier :: identifier

as the complete model.

Use recursive/unbounded composition.

---

30. Generic Arity

Generic type argument lists must not impose a finite grammar-level maximum.

Valid:

A<T>
A<T, U>
A<T, U, V>
A<T, U, V, ...>

subject only to practical compiler resources and semantic validity.

The grammar must not contain:

MAX_GENERIC_ARITY

or an equivalent structural ceiling.

---

31. Nested Types

Types must be recursively composable.

Examples:

Vec<Option<Result<T, E>>>

and:

fn(Vec<Matrix<float, R, C>>) -> Result<T, E>

must be representable without special-case grammar expansion.

The same principle applies to future type constructors.

---

32. Type Constructor Extensibility

The grammar must allow future type constructors without requiring a new parser rule for every library-defined type.

For example:

Tensor<T, ...>
Matrix<T, ...>
QuantumState<T>
Resource<T>
Device<T>
Stream<T>
Future<T>
Actor<T>
Channel<T>

should be representable using the generic/named type machinery.

Only language-semantic type constructors that require special syntax should receive dedicated grammar rules.

---

33. No Vendor Lock-In

The type grammar must not contain vendor-specific hardware types as mandatory core syntax.

Vendor integrations belong under dialect/interoperability mechanisms.

For example, a vendor-specific accelerator type should be expressible through a registered dialect rather than requiring permanent modification of the universal core type grammar.

This preserves POCO-REAF.

---

34. Dialect Integration

Future/domain-specific types may be introduced through:

grammar/dialects/

and semantic dialect registration.

Dialect types must still obey:

- namespace isolation;
- versioning;
- compatibility;
- capability declaration;
- semantic validation;
- deterministic parsing;
- no accidental machine-size assumptions.

The core type grammar must remain stable while dialect space evolves.

---

35. Type Aliases

Type aliases belong to declarations/type syntax, not type checking.

Example:

type Distance = float

The type grammar must parse the referenced type.

Alias resolution happens later.

Aliases must not create a second type system.

---

36. Algebraic Types

Algebraic type syntax may represent:

- sum types;
- product types;
- tagged unions;
- recursive types;
- generic variants.

The grammar preserves structure.

It does not decide memory layout.

Layout belongs to semantic lowering/ABI/code generation.

---

37. Function Types

Function types must support arbitrary parameter lists.

The grammar must not hard-code:

fn(A)
fn(A, B)
fn(A, B, C)

as separate finite cases.

Instead:

fn(parameterType*)

must be structurally compositional.

Calling conventions, ABI, execution placement, asynchronous execution, and target-specific lowering remain downstream.

---

38. Async and Distributed Types

The type system must be extensible to types such as:

Future<T>
Task<T>
Stream<T>
Remote<T>
Actor<T>
Channel<T>
Service<T>

without baking distributed topology into the type grammar.

A type such as:

Remote<T>

does not imply:

- number of nodes;
- node identity;
- network topology;
- provider;
- region;
- cloud platform.

Those are deployment/execution concerns.

---

39. Data and Tensor Types

Tensor-like types should use parameterization rather than hard-coded dimensions.

Examples:

Tensor<T, Shape>
Matrix<T, Rows, Columns>
Vector<T, N>

The grammar must not impose a finite maximum rank.

Shape semantics belong to type checking and numerical/compiler infrastructure.

---

40. Type Constraints and Capability Constraints

A type constraint can establish semantic properties.

A capability requirement establishes execution/environment requirements.

Do not conflate:

T: QuantumState

with:

requires capability quantum

and do not conflate either with:

requires resource qubits

These must remain separate semantic categories.

---

41. Error Reporting Contract

Malformed types must produce deterministic parser diagnostics.

Diagnostics should preserve:

- source span;
- offending token;
- expected construct;
- grammar context;
- stable diagnostic category.

Semantic errors must not be disguised as syntax errors.

For example:

UnknownType

is generally a name-resolution/type-resolution error.

It should not be emitted as a parser syntax error merely because the type is unknown to the current semantic environment.

---

42. Determinism

Parsing must be deterministic.

Given identical:

source
lexer version
grammar version
parser configuration

the parser must produce equivalent parse structure.

The grammar must not depend on:

- runtime hardware;
- network state;
- device availability;
- quantum backend state;
- calibration;
- scheduling;
- random hardware discovery.

---

43. No Runtime Dependency

"grammar/types/" must not depend on runtime execution.

The grammar must be usable when:

- no hardware is present;
- no QPU is present;
- no GPU is present;
- no FPGA is present;
- no network is present;
- no cloud provider is available.

Parsing is a language operation.

Execution is a later operation.

---

44. No Hardware Discovery Dependency

The parser must never query:

CPU count
GPU count
QPU count
qubit count
memory capacity
FPGA resources
network topology
device calibration
provider availability

while parsing a type.

Such information belongs to:

capability discovery
resource management
target selection
runtime
hardware abstraction

---

45. No Scheduling Dependency

Types must not depend on scheduling.

A type does not decide:

- operation order;
- execution time;
- latency;
- resource reservation;
- gate timing;
- pulse schedule.

Scheduling consumes semantic/compiled representations downstream.

---

46. No Optimization Dependency

Type grammar does not optimize types.

Examples such as:

int -> vector register
Tensor -> GPU tensor
Qubit -> physical qubit

are compiler transformations.

They do not belong to parsing.

---

47. No QEC Dependency

QEC consumes semantic quantum information downstream.

The type grammar must not generate:

- syndrome measurements;
- stabilizer circuits;
- decoder configuration;
- correction operations;
- ancilla allocation.

---

48. No ZQN Dependency

Noise and fault semantics remain owned by ZQN.

Type grammar may expose semantic hooks through which downstream analysis can associate execution requirements with types, but it must not duplicate ZQN's fault/noise model.

---

49. No Resilience Dependency

Resilience decides how computation adapts to failures and changing execution conditions.

Types describe semantic contracts.

The type grammar must not perform:

- retry;
- recovery;
- rollback;
- rerouting;
- backend switching;
- mitigation selection;
- fault diagnosis.

---

50. AST Contract

The parser should produce a structural representation that can be lowered into the repository's frontend AST.

The AST should preserve:

- type constructor;
- type path;
- generic arguments;
- type-level values;
- qualifiers;
- references;
- lifetimes;
- tuple structure;
- array structure;
- function structure;
- quantum type identity;
- source spans.

The AST must not prematurely resolve:

- aliases;
- generic substitutions;
- hardware resources;
- physical qubits;
- backend selection.

---

51. Semantic Type Contract

After parsing, semantic analysis is responsible for determining:

- whether a type exists;
- whether a type is visible;
- whether generic arguments are valid;
- whether constraints are satisfied;
- whether type-level values are legal;
- whether dimensions are compatible;
- whether references are valid;
- whether ownership rules are satisfied;
- whether quantum semantics are valid;
- whether hardware/resource requirements are satisfiable.

---

52. IR Contract

The type grammar must never directly construct canonical IR.

The correct boundary is:

grammar
    |
    v
AST
    |
    v
semantic types
    |
    v
IR lowering

For quantum computation:

semantic quantum types
    |
    v
quantum::ir

The existing repository explicitly establishes "quantum::ir" as the canonical semantic boundary rather than allowing frontend grammar components to become a competing representation. The type grammar must preserve that architecture.

---

53. Resource Integration

Type-level resource requirements must integrate with:

grammar/resources/

and the repository's resource-management subsystem.

A type may express that a value is resource-like.

It must not allocate the resource.

The semantic layer determines the requirement.

Resource management determines availability.

Compilation determines realization.

Runtime determines execution.

---

54. Capability Integration

Type syntax may reference capabilities where the language requires it.

Capability satisfaction belongs to capability checking.

For example:

T: QuantumCapable

is a type-level contract.

Whether the target satisfies the associated execution capability is not decided by the grammar.

---

55. Compatibility Contract

Type grammar evolution must preserve source compatibility wherever possible.

Breaking changes require:

1. documented language-version change;
2. migration guidance;
3. compatibility tests;
4. deprecated syntax period where appropriate;
5. explicit semantic rationale.

Never silently reinterpret an existing type construct in a way that changes program meaning.

---

56. Reserved Space

Future type-system expansion must reserve namespace and syntax deliberately.

Reserved areas may include:

future type constructors
future type qualifiers
future dependent types
future effect types
future ownership models
future quantum abstractions
future hardware abstractions
future computational domains

Reserved syntax must be documented rather than accidentally consumed.

---

57. Security

Type syntax must not provide implicit access to:

- arbitrary filesystem resources;
- network resources;
- hardware addresses;
- privileged devices;
- secret material;
- credentials;
- private keys.

Type names and type constraints are declarations.

Security authorization belongs to the security/capability subsystem.

---

58. Scalability Requirements

The type grammar must scale with:

- number of source types;
- number of generic parameters;
- nesting depth;
- number of modules;
- number of domains;
- number of quantum values;
- number of hardware resources;
- number of tensor dimensions;
- number of distributed resources;
- future dialects.

No arbitrary grammar constant may restrict these dimensions.

Compiler resource exhaustion must be handled through explicit compiler resource policies and diagnostics rather than silently changing language semantics.

---

59. File-Level Completion Contract

Each file in "grammar/types/" is complete only when all of the following are established:

Purpose

The file has one clearly defined syntactic responsibility.

Ownership

Every rule has a documented owner.

Non-ownership

The file explicitly excludes semantic responsibilities owned elsewhere.

Inputs

Every token or parser rule consumed is identified.

Outputs

Every parser rule exported to other grammar files is identified.

Upstream contract

The source of every consumed rule/token is known before implementation.

Downstream contract

Every consumer of exported rules is known before implementation.

AST contract

The intended AST representation is defined.

Semantic contract

The intended semantic interpretation is defined.

IR contract

The lowering boundary is identified.

Runtime contract

Any runtime relevance is explicitly downstream.

Cross-domain contract

Quantum/classical/HDL/hardware/resource interactions are defined where relevant.

Tests

Positive, negative, boundary, scalability, determinism, and integration tests are defined.

Hard-coding audit

No accidental machine-size assumptions remain.

Compatibility

Versioning and migration behavior are documented.

Completion criteria

The file can be considered finished without waiting for another grammar file to reveal a missing fundamental abstraction.

---

60. Required Test Classes

The type subsystem must test at least:

Primitive

int
float
bool
char
string

Named

User
module::User
a::b::c::User

Generic

Vec<int>
Map<string, int>
Result<T, E>

Nested

Vec<Option<Result<T, E>>>

Functions

fn() -> int
fn(int) -> int
fn(int, float, bool) -> string

Quantum

Qubit
Qubit?
Register<Qubit>

Parameterized

Vector<float, N>
Matrix<float, Rows, Columns>
Tensor<float, N, M, K>

References

&T
&mut T
&'a T

Pointers

*T
*mut T

Tuples

()
(T,)
(T, U)
(T, U, V)

Arrays

[T]
[T; N]

---

61. Negative Tests

The type grammar must reject malformed constructs such as:

<
>
Vec<
Vec<int
fn(
fn() ->
[T
[T;
&T
&mut

Semantic invalidity must be tested separately from syntax invalidity.

For example, an unknown type name should generally be tested at semantic analysis rather than incorrectly classified as parser failure.

---

62. Scalability Tests

The test suite must generate type expressions with:

- many nested generic applications;
- many generic arguments;
- many namespace components;
- many tuple elements;
- many function parameters;
- many type-level parameters;
- deeply nested composite types;
- large quantum type structures.

The purpose is to prove that no artificial grammar maximum exists.

Tests must not encode a false promise that every implementation can process literally unlimited input.

They must prove that the grammar itself has no arbitrary finite semantic ceiling.

---

63. Cross-Domain Tests

At minimum test combinations involving:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

Example conceptual types:

QuantumState<T>
Tensor<float, N, M>
Accelerator<T>
Remote<QuantumState<T>>

The parser must preserve these structures without selecting hardware.

---

64. Determinism Tests

For every representative type:

source
 -> lexer
 -> parser

must produce stable syntax.

The same source must not parse differently because:

- a GPU exists;
- a QPU exists;
- a machine has more memory;
- hardware calibration changed;
- runtime state changed;
- a network provider is available.

---

65. Round-Trip Tests

Where the repository provides a canonical formatter/serializer:

source
    |
    v
lexer
    |
    v
parser
    |
    v
AST
    |
    v
printer
    |
    v
parser

must preserve semantic type structure.

Whitespace and formatting may change.

Semantic type identity must not.

---

66. Documentation Integration

Every public type construct must be documented in the authoritative language specification.

Documentation must distinguish:

syntax
semantic meaning
implementation representation
target realization

Do not document implementation representation as language semantics.

---

67. Repository Integration Matrix

Subsystem| Type Grammar Role
Lexer| supplies canonical tokens
Core grammar| supplies names/paths/metadata
AST| receives parsed type structure
Type checker| resolves semantic types
Generics| consumes generic syntax
Effects| may reference type/effect contracts
Memory| consumes reference/resource semantics
Classical| consumes classical types
Quantum| consumes quantum types
"quantum::ir"| canonical downstream quantum semantic boundary
QEC| consumes semantic quantum information
ZQN| consumes downstream noise/fault semantics
Optimization| consumes lowered semantic/IR representation
Scheduling| consumes executable semantic representation
Hardware| resolves target realization
Resources| resolves requirements/capabilities
Resilience| adapts execution after faults
Runtime| executes realized representation
Interoperability| maps external type/ABI representations
Dialects| extends type namespace/semantics
Tests| validates grammar and contracts

---

68. Explicit Non-Dependencies

"grammar/types/" must not directly depend on:

hardware discovery
runtime state
QPU calibration
QPU topology
scheduler state
optimizer state
ZQN state
QEC decoder state
resilience state
network availability
cloud provider availability
device IDs
physical addresses

Those dependencies would violate the architecture.

---

69. Integration With "grammar/core"

The type grammar may consume core constructs for:

- identifiers;
- qualified names;
- metadata;
- capabilities;
- constraints;
- annotations.

Core owns the generic language infrastructure.

Types specialize that infrastructure for type syntax.

---

70. Integration With Expressions

Type-level expressions must not become a second general expression language.

Where a type requires a value expression:

Vector<T, N>

the grammar should use the language's canonical expression/type-value abstraction.

The semantic layer decides which expressions are permitted in type positions.

---

71. Integration With Declarations

Declarations consume type syntax.

Examples:

let x: T
fn f(x: T) -> U
type Alias = T
struct S<T>

The declaration grammar owns declaration structure.

The type grammar owns the "T"/"U" structures.

Neither should duplicate the other's responsibilities.

---

72. Integration With Functions

Function declarations consume function types and parameter types.

Function grammar owns:

- function declarations;
- parameter declarations;
- bodies;
- modifiers.

Type grammar owns:

- parameter type expressions;
- return type expressions;
- function type expressions.

---

73. Integration With Modules

Module grammar owns module/import/export syntax.

Type paths may refer to module-qualified names.

Module resolution remains semantic.

---

74. Integration With HDL

HDL grammar consumes hardware-specific type forms.

The type system must permit hardware types without making hardware implementation part of the universal type core.

For example:

Signal<Bit>
Register<T, Width>

can express semantic hardware structures.

Physical FPGA/ASIC implementation remains downstream.

---

75. Integration With Distributed Computing

Distributed type wrappers may express semantic execution modes:

Remote<T>
Replicated<T>
Stream<T>
Service<T>

but must not encode fixed node counts or topology.

---

76. Integration With AI/ML

AI grammar may consume parameterized types such as:

Tensor<T, Shape>
Model<Input, Output>
Dataset<T>

The type grammar must remain domain-neutral.

AI-specific semantics belong to the AI subsystem.

---

77. Future-Proofing

The type system must remain extensible without changing fundamental parser architecture every time a new computational paradigm appears.

Future examples may include:

- neuromorphic computing;
- optical computing;
- molecular computing;
- biological computing;
- probabilistic computing;
- reversible computing;
- analog computing;
- photonic computing;
- memristive computing;
- novel accelerators.

A future type should be able to enter the language through generic/named/dialect mechanisms unless it genuinely requires new core syntax.

---

78. Forbidden Architecture

Do not implement:

type -> hardware
type -> device
type -> runtime
type -> scheduler
type -> QPU
type -> calibration
type -> physical address

as direct grammar dependencies.

Also forbidden:

type grammar -> quantum IR

as a direct parser-level dependency.

The correct architecture remains:

type syntax
    ↓
AST
    ↓
semantic type
    ↓
canonical IR
    ↓
target realization

---

79. Hard-Coding Audit Checklist

Before accepting any file under "grammar/types/", search for:

MAX_
LIMIT_
CAPACITY_
QUANTUM_COUNT
QUBIT_COUNT
CORE_COUNT
THREAD_COUNT
DEVICE_COUNT
GPU_COUNT
FPGA_COUNT
NODE_COUNT
REGISTER_COUNT
MEMORY_SIZE
FIXED_
32
64
128
1024

Each occurrence must be classified.

It is acceptable for a number to appear in:

- an example;
- a specification of a fixed-width type;
- a test fixture;
- documentation explaining a target-specific example.

It is not acceptable for such values to silently constrain the universal grammar.

---

80. Generated Files

Generated parser artifacts must not become independent sources of truth.

The authoritative source remains the grammar source files.

Generated artifacts must be reproducible from:

grammar
+
ANTLR version/tool configuration

Generated files must not be manually edited.

---

81. Versioning

The type grammar must carry explicit language-version compatibility.

A grammar change that modifies accepted syntax must be classified as:

additive
clarifying
deprecated
migration-required
breaking

Compatibility rules belong to:

grammar/compatibility/

while this README defines how those rules apply to the type subsystem.

---

82. Completion Definition

The "grammar/types/" subsystem is production-ready only when:

- all type grammar files have defined ownership;
- there is one canonical type-expression architecture;
- no duplicate type systems exist;
- no duplicate expression language exists;
- all parser dependencies are known;
- all downstream semantic consumers are known;
- quantum types integrate through semantic lowering;
- "quantum::ir" remains canonical;
- resource requirements remain separate from type syntax;
- hardware realization remains downstream;
- generic arity is not artificially bounded;
- namespace depth is not artificially bounded;
- tuple arity is not artificially bounded;
- function parameter count is not artificially bounded;
- quantum cardinality is not artificially bounded;
- tensor dimensions are not artificially bounded;
- no machine-specific hardware is embedded into core types;
- parser behavior is deterministic;
- semantic errors are separated from syntax errors;
- diagnostics are stable;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- cross-domain tests exist;
- round-trip tests exist where supported;
- compatibility tests exist;
- hard-coding audits pass;
- ANTLR generation succeeds;
- Rust compiler integration succeeds under Rust 1.97/1.97.1;
- no unsafe Rust is required;
- repository-wide integration succeeds.

---

83. Definition of Done for Individual Type Files

A type grammar file is DONE only if:

[ ] Single responsibility documented
[ ] Ownership documented
[ ] Non-ownership documented
[ ] All tokens identified
[ ] All upstream parser dependencies identified
[ ] All downstream consumers identified
[ ] AST representation defined
[ ] Semantic representation defined
[ ] IR boundary defined
[ ] Quantum boundary defined where applicable
[ ] Resource boundary defined where applicable
[ ] Hardware boundary defined where applicable
[ ] No runtime dependency
[ ] No hardware-discovery dependency
[ ] No scheduling dependency
[ ] No optimizer dependency
[ ] No QEC implementation
[ ] No ZQN implementation
[ ] No resilience implementation
[ ] No fixed machine-size assumptions
[ ] No arbitrary grammar cardinality limit
[ ] Positive tests complete
[ ] Negative tests complete
[ ] Boundary tests complete
[ ] Scalability tests complete
[ ] Determinism tests complete
[ ] Cross-domain tests complete
[ ] Compatibility tests complete
[ ] Documentation complete
[ ] ANTLR generation passes
[ ] Repository integration passes

Only then may the file be considered complete.

---

84. Implementation Order

The type subsystem should be completed in dependency order.

Recommended order:

1. types/README.md
       |
       v
2. canonical lexer/token contract
       |
       v
3. core names/paths contract
       |
       v
4. types.g4
       |
       +-----------------------------+
       |                             |
       v                             v
5. primitive-types.g4        6. type-level value contract
       |                             |
       +-------------+---------------+
                     |
                     v
7. composite-types.g4
                     |
        +------------+-------------+
        |            |             |
        v            v             v
8. tuple        9. array       10. reference
        |            |             |
        +------------+-------------+
                     |
                     v
11. generic-types.g4
                     |
                     v
12. function-types.g4
                     |
          +----------+----------+
          |          |          |
          v          v          v
13. option       14. result   15. algebraic
          |          |          |
          +----------+----------+
                     |
                     v
16. classical-types.g4
                     |
                     v
17. resource-types.g4
                     |
                     v
18. quantum-types.g4
                     |
                     v
19. hardware-types.g4
                     |
                     v
20. type-constraints.g4
                     |
                     v
21. full type-system validation
                     |
                     v
22. repository-wide integration

This ordering prevents later type-domain files from redefining foundational type semantics.

---

85. Final Architectural Rule

The type grammar must preserve the following invariant:

ONE SOURCE TYPE
       |
       v
ONE STABLE SEMANTIC MEANING
       |
       +----------------+
       |                |
       v                v
CLASSICAL REALIZATION  QUANTUM REALIZATION
       |                |
       +--------+-------+
                |
                v
      HARDWARE-INDEPENDENT
          SEMANTICS
                |
                v
        TARGET REALIZATION

Therefore:

«A Zamani type describes what a value, computation, resource, or interface means—not the accidental characteristics of the machine on which it happens to execute.»

This is necessary for:

Zamani: From Atom to Everywhere

and:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).

The "grammar/types/" subsystem is consequently a portable semantic-syntax boundary, not a hardware description, scheduler, optimizer, runtime, QEC engine, ZQN model, or second IR.