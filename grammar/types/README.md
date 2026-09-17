Zamani Type Grammar

Production Type-System Grammar Contract

Path: "grammar/types/README.md"

Language: Zamani

Repository: "Benwellonedge28/Zamani"

Grammar technology: ANTLR4

Compiler baseline: Rust 1.97 / Rust 1.97.1, Rust 2021

Safety: safe Rust only; "unsafe" is prohibited

Primary architectural goal:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)»

Scalability principle:

«A Zamani type expresses source-level meaning, requirements, relationships, and capabilities. It must not encode arbitrary limits imposed by the machine on which the program happens to execute.»

---

1. Purpose

"grammar/types/" defines the source-level type syntax contract of the Zamani programming language.

It is a foundational grammar subsystem shared by:

- classical computing;
- quantum computing;
- hybrid quantum-classical computing;
- HDL;
- hardware/software co-design;
- systems programming;
- embedded computing;
- parallel computing;
- distributed computing;
- HPC;
- AI/ML;
- tensor computing;
- numerical computing;
- accelerators;
- networking;
- cryptography;
- scientific computing;
- data processing;
- temporal/multi-timeline computation;
- resource-aware computation;
- future computational domains.

The type grammar is responsible for answering:

«How does a programmer express this type in Zamani source syntax?»

It is not responsible for answering:

«Can the current machine execute it?»

or:

«Which physical device should execute it?»

or:

«Which physical qubit should be used?»

or:

«How many CPUs/GPUs/QPUs/nodes should be allocated?»

or:

«Which routing, scheduling, QEC, ZQN, HAL, compiler, or runtime implementation should be selected?»

Those questions belong to later layers.

---

2. Architectural Position

The complete type path is:

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
frontend AST TypeExpr
    │
    ▼
structural AST validation
    │
    ▼
name resolution
    │
    ▼
generic/type-argument resolution
    │
    ▼
type checking / inference / unification
    │
    ▼
semantic type model
    │
    ├───────────────────┬───────────────────┬─────────────────────┐
    ▼                   ▼                   ▼
classical semantics   quantum semantics   HDL/resource semantics
    │                   │                   │
    └───────────────────┴───────────────────┘
                        │
                        ▼
              canonical semantic model
                        │
             ┌──────────┼───────────┐
             ▼          ▼           ▼
        classical    quantum::ir   HDL/domain IR
             │          │           │
             └──────────┼───────────┘
                        ▼
                   optimization
                        │
             ┌──────────┼──────────────┐
             ▼          ▼              ▼
          routing    scheduling     resilience
             │          │              │
             └──────────┼──────────────┘
                        ▼
                       ZQN
                        │
                       HAL
                        │
                        ▼
                 target realization
                        │
              ┌─────────┼─────────┐
              ▼         ▼         ▼
             CPU       GPU       FPGA
              │         │         │
              └──────┬──┴──┬──────┘
                     ▼     ▼
                    QPU   future targets

The grammar therefore has a strict dependency direction:

syntax
  ↓
AST
  ↓
semantic interpretation
  ↓
canonical semantic model
  ↓
IR
  ↓
optimization/lowering
  ↓
realization

Never:

grammar → hardware → grammar
grammar → runtime → grammar
grammar → quantum backend → grammar
grammar → physical topology → grammar

---

3. Core Ownership

"grammar/types/" owns:

- type-expression syntax;
- primitive type syntax;
- named type syntax;
- qualified type paths;
- generic type syntax;
- generic argument syntax;
- tuple types;
- array types;
- slice types;
- function types;
- reference types;
- pointer types;
- optional types;
- result types;
- never type;
- unit type;
- algebraic type syntax;
- resource type syntax;
- capability type syntax;
- quantum type syntax;
- classical domain type syntax;
- hardware-domain type syntax;
- dependent/value-parameterized type syntax;
- type-level value syntax;
- type qualifiers;
- ownership/resource qualifiers;
- lifetime syntax;
- type constraints and bounds;
- future-extensible named type constructors.

---

4. This Directory Does NOT Own

The type grammar does not own:

- lexical token definitions;
- keyword spelling;
- Unicode character classification;
- identifier resolution;
- symbol tables;
- type inference;
- type unification;
- trait/interface resolution;
- overload resolution;
- ownership checking;
- borrow checking;
- capability satisfiability;
- resource discovery;
- hardware discovery;
- hardware selection;
- physical qubit allocation;
- physical topology;
- routing;
- scheduling;
- calibration;
- QEC implementation;
- ZQN noise/fault implementation;
- HAL implementation;
- optimization;
- backend selection;
- runtime layout;
- ABI layout;
- physical memory layout;
- device IDs;
- vendor-specific representations.

---

5. Non-Negotiable Architectural Rules

5.1 One public type entry point

There must be exactly one public parser composition rule:

typeExpression

Supporting files may define specialized rules, but they must not create incompatible parallel public type-expression roots.

---

5.2 One canonical AST

All type grammar constructs must lower into the existing frontend "TypeExpr".

The grammar must not introduce:

GrammarType
TypeGrammarNode
QuantumTypeIR
HardwareTypeIR
GenericTypeIR

as competing representations.

The existing frontend AST is the source-level AST boundary. The repository's frontend "TypeExpr" explicitly separates source representation from semantic types and later IR.

---

6. Existing AST Integration

The canonical source-level type representation is:

src/frontend/ast/node/types/type_expr.rs

The type grammar must map into that representation.

Representative mappings are:

Grammar construct| AST contract
named type| "TypeExpr::Identifier"
generic type| "TypeExpr::Generic"
generic parameter| "TypeExpr::GenericParameter"
tuple| "TypeExpr::Tuple"
array| "TypeExpr::Array"
slice| "TypeExpr::Slice"
function| "TypeExpr::Function"
reference| "TypeExpr::Reference"
pointer| "TypeExpr::Pointer"
optional| "TypeExpr::Optional"
result| "TypeExpr::Result"
never| "TypeExpr::Never"
unit| "TypeExpr::Unit"
quantum| canonical quantum type representation
temporal| canonical temporal representation
linear| canonical linear representation
affine| canonical affine representation
resource| canonical resource representation
dependent/value parameter| canonical source-level value representation

The exact Rust enum/field names remain owned by:

src/frontend/ast/node/types/type_expr.rs

The grammar must not silently invent a different spelling of the AST contract.

---

7. Important Generic-Argument Correction

The current type architecture must distinguish:

type argument
value argument
resource argument
capability argument

where the frontend semantic model requires that distinction.

Do not force all generic arguments through:

TypeExpr

when the semantic AST has a dedicated generic-argument representation.

The grammar should therefore conceptually support:

Generic<T>
Generic<T, U>
Vector<T, N>
Tensor<T, Rows, Columns>
Resource<Qubit, N>
Capability<C>

without assuming that every argument is a type.

The correct architecture is:

genericArgument
    ├── typeArgument
    ├── valueArgument
    ├── resourceArgument
    └── capabilityArgument

if and only if the canonical frontend AST/semantic model supports those categories.

The parser must preserve the distinction.

The semantic layer determines validity.

---

8. POCO-REAF

A type must remain semantically meaningful across different execution environments.

For example:

Tensor<Float, Shape>

must retain the same source-level meaning when compiled for:

CPU
GPU
FPGA
ASIC
QPU
cluster
cloud
embedded target
future architecture

The implementation may choose different representations.

The type does not.

---

9. No Language-Level Machine Limits

The type grammar MUST NOT contain universal limits such as:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_DEVICES
MAX_MEMORY
MAX_REGISTER_WIDTH
MAX_REGISTER_COUNT
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_ARRAY_LENGTH
MAX_TUPLE_ARITY
MAX_GENERIC_ARITY
MAX_FUNCTION_PARAMETERS
MAX_TYPE_DEPTH
MAX_RESOURCE_COUNT

Nor may an equivalent limit be hidden inside grammar alternatives.

Bad:

genericArguments
    : type
    | type COMMA type
    | type COMMA type COMMA type
    ;

Good:

genericArgumentList
    : genericArgument (COMMA genericArgument)* COMMA?
    ;

The language therefore has no arbitrary grammar-imposed finite ceiling.

---

10. What “Infinity” Means

“Scale to infinity” means:

«The language grammar does not impose an arbitrary finite machine-capacity ceiling.»

Actual execution can still be limited by:

- available memory;
- compiler resources;
- operating-system limits;
- runtime limits;
- distributed resources;
- target capabilities;
- provider limits;
- user-defined policies;
- security policies.

Those are implementation or environment constraints.

They are not type-language limits.

---

11. Type Categories

The production type system must support the following categories.

11.1 Primitive types

Examples:

void
bool
char
string
str
int
float

Additional primitive spellings may be standardized later.

Fixed-width types such as:

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

must be explicitly specified if they are reserved.

They must not accidentally become machine-independent defaults merely because a backend uses those widths.

---

12. Semantic Integer vs Fixed Width

These are different concepts:

int

and:

i64

if both exist.

"int" expresses the language's semantic integer abstraction.

"i64" expresses a specific representation width.

The grammar must not silently equate:

int == i32

unless the language specification explicitly defines it.

---

13. Named Types

Named types must support arbitrary qualified paths:

User
Tensor
QuantumState
std::collections::Map
project::module::Type
domain::resource::Capability

Canonical syntax:

namedType
    : typePath
    ;

typePath
    : typePathSegment (DOUBLE_COLON typePathSegment)*
    ;

typePathSegment
    : IDENTIFIER
    ;

Name resolution is downstream.

---

14. Generic Types

Canonical examples:

Vec<Int>
Map<String, Int>
Tensor<Float, Shape>
Quantum<State>
Resource<Qubit>
Capability<QuantumMeasurement>

Generic arity is unbounded by grammar.

The parser preserves ordered arguments.

Semantic analysis determines:

- arity;
- argument category;
- constraints;
- substitution;
- inference;
- compatibility.

---

15. Generic Arguments

The production architecture must support:

type
value
resource
capability

arguments where required by the language.

Examples:

Vector<Float, N>
Matrix<Float, Rows, Columns>
Tensor<Float, Shape>
Resource<Qubit, N>

The grammar must not convert:

N

into a machine-sized integer.

It must preserve it as a source-level value expression.

---

16. Generic Parameters

Generic declarations must support:

T
U
V

and constrained forms such as:

T: Numeric
T: Comparable
T: QuantumState
T: Sendable
T: HardwareCompatible

The grammar recognizes the syntax.

Semantic analysis determines whether a type satisfies the constraint.

The grammar must not contain a closed universe of all future traits or capabilities.

---

17. Tuple Types

Must support:

()
(T,)
(T, U)
(T, U, V)

with arbitrary source cardinality.

Do not encode tuple arity limits.

"()" is the unit type.

---

18. Array Types

The language must distinguish:

[T]

from:

[T; N]

where the specification defines them as slice and explicitly sized array respectively.

"N" remains a source-level value.

It is not interpreted by the parser as a machine limit.

Examples:

[Int; N]
[Float; Rows * Columns]
[Qubit; qubits]

---

19. Slice Types

Canonical syntax:

[T]

A slice does not imply:

- fixed pointer width;
- fixed address space;
- fixed allocation;
- fixed machine memory.

Those are downstream implementation concerns.

---

20. Function Types

Support:

fn() -> T
fn(T) -> U
fn(T, U) -> V

and, where specified:

fn<T>(T) -> T

Parameter cardinality is unbounded by grammar.

Calling convention and ABI are downstream.

---

21. Async/Effectful Function Types

Where the language specification exposes these concepts at the type level, support composition such as:

fn(T) -> U

with separately represented:

effects
capabilities
asyncness
resource requirements

Do not create a second function-type language inside "effects/".

---

22. Reference Types

Support source-level references:

&T
&mut T
&'a T
&'a mut T

Lifetime syntax is lexical/source syntax.

Borrow validity is semantic.

---

23. Pointer Types

Support:

*T
*mut T

Pointer width, address space, representation, and ABI are not grammar concerns.

---

24. Optional Types

Support:

T?

and/or an explicit constructor such as:

Option<T>

if both are standardized.

Both forms must map to the same semantic optional concept where specified.

The grammar must not create two different optional-type semantics.

---

25. Result Types

Support:

Result<T, E>

The grammar does not determine:

- error handling;
- recovery;
- runtime representation;
- ABI.

---

26. Never Type

Support:

never

or the canonical reserved spelling specified by the language.

It represents a semantic type.

---

27. Unit Type

Canonical source representation:

()

It must map to the canonical "TypeExpr::Unit".

---

28. Algebraic Types

The type grammar must integrate with:

enum
struct
record
sum
product
variant
union

where each concept is actually standardized by the language.

The type-expression grammar references named declarations.

The declaration grammar owns declaration syntax.

Do not duplicate declaration grammar inside "types/".

---

29. Dependent / Value-Parameterized Types

Zamani must be capable of expressing symbolic dimensions without making those dimensions machine limits.

Examples:

Vector<T, N>
Matrix<T, Rows, Columns>
Tensor<T, N, M, K>
Array<T, Size>

The type grammar preserves the expressions.

Semantic analysis determines:

- whether the expression is legal;
- whether it is compile-time known;
- whether it is dependent;
- whether it is satisfiable;
- whether specialization is possible.

---

30. Type-Level Values

Type-level expressions may contain:

identifier
integer literal
floating literal
qualified value path
parenthesized expression

and, where formally allowed:

+
-
*
/
%
<<
>>
&
|
^
~

The grammar must preserve source structure.

It must not evaluate the expression.

---

31. Type-Level Arithmetic

Examples:

N + 1
Rows * Columns
2 * N
Size / Block
N << 1

The grammar recognizes structure.

It does not decide whether:

N

fits in:

usize
u64
u32

or any other implementation representation.

---

32. Resource Types

Resource types are source-level declarations of resource semantics.

Examples:

Resource<T>
Resource<Qubit>
Resource<Memory>
Resource<Compute>

They do not allocate anything.

They do not select devices.

They do not perform scheduling.

---

33. Capability Types

Capabilities may be represented through named/generic type syntax:

Capability<C>
Capability<QuantumMeasurement>
Capability<TensorCompute>

The grammar does not decide whether the current environment provides the capability.

That belongs to capability analysis and resource resolution.

---

34. Quantum Types

Quantum type syntax must remain extensible.

The grammar may reserve canonical source-level quantum types such as:

Qubit

and support open named/generic forms such as:

LogicalQubit
QuantumState<T>
QRegister<N>
QuantumResource<T>

where those names are not reserved keywords.

The critical rule is:

«Do not enumerate today's quantum hardware vocabulary into the core type grammar.»

---

35. Quantum Cardinality

No grammar-level finite quantum cardinality exists.

Do not define:

Qubit32
Qubit64
Qubit128

as universal type categories.

A program may explicitly express:

QRegister<N>

where "N" is semantic program information.

The grammar must not impose:

N <= 32

or:

N <= 1024

or any other universal maximum.

---

36. Logical vs Physical Quantum Types

If the language distinguishes:

LogicalQubit
PhysicalQubit

the distinction is semantic.

The grammar must not bind a physical type to:

- physical index;
- vendor;
- QPU ID;
- topology;
- calibration;
- pulse;
- timing;
- hardware address.

Those belong downstream.

---

37. Canonical Quantum IR

The type grammar must integrate with:

src/quantum/ir/

through semantic analysis.

The architecture is:

quantum source type
        ↓
frontend TypeExpr
        ↓
semantic quantum type
        ↓
quantum::ir

"quantum::ir" remains the canonical quantum semantic boundary.

The grammar must never create:

QuantumTypeIR
QuantumGateIR
QuantumQubitIR

as another quantum IR.

---

38. QEC Boundary

The type grammar may describe semantic information consumed by QEC.

It must not implement:

- syndrome generation;
- decoding;
- ancilla placement;
- stabilizer scheduling;
- physical-qubit allocation;
- error-correction routing.

The direction remains:

type
 ↓
semantic quantum model
 ↓
quantum::ir
 ↓
QEC

---

39. ZQN Boundary

ZQN owns quantum fault/noise semantics.

Type syntax may express a requirement or semantic property associated with resilience, but type parsing does not implement ZQN.

Do not add noise models to the type grammar merely because quantum types may eventually interact with them.

---

40. Hardware Types

Hardware-domain types may express semantic categories such as:

CPU
GPU
FPGA
ASIC
QPU
Accelerator
Memory
Interconnect

only when the language specification requires them.

They must not imply concrete instances.

Do not make:

GPU0
GPU1
QPU0
QPU1
CPU0
node0

special type syntax.

Those may be ordinary values/identifiers in target-specific programs.

---

41. Hardware/Software Co-Design

A type can express semantic compatibility with hardware/resource abstractions.

Example:

Accelerator<T>
Memory<T>
Compute<T>

But the type grammar must not determine:

- placement;
- mapping;
- synthesis;
- scheduling;
- physical implementation.

That belongs to:

grammar/hardware/
grammar/resources/
grammar/compile/
grammar/execution/

and their downstream compiler systems.

---

42. Classical Types

Classical types must cover:

- scalar values;
- integers;
- floating point;
- booleans;
- characters;
- strings;
- vectors;
- matrices;
- tensors;
- records;
- tuples;
- symbolic values;
- numerical abstractions;
- system-level abstractions.

The type grammar must not assume a particular CPU architecture.

---

43. Tensor Types

Tensor syntax must permit symbolic shapes.

Examples:

Tensor<Float, Shape>
Tensor<Float, N, M>
Tensor<Float, Batch, Height, Width, Channels>

No universal tensor-rank ceiling belongs in the grammar.

No universal dimension ceiling belongs in the grammar.

---

44. AI/ML Integration

AI types must remain semantic and framework-neutral.

The grammar must not require:

PyTorchTensor
TensorFlowTensor
JAXArray
CUDAArray

as core language types.

Framework interoperability belongs under:

grammar/interoperability/

AI-specific semantic constructs belong under:

grammar/ai/

---

45. HDL Integration

HDL source types may include semantic concepts for:

- signals;
- buses;
- registers;
- memories;
- interfaces;
- hardware values;
- clocks;
- timing;
- hardware resources.

But HDL type syntax must not silently encode a particular FPGA/ASIC implementation.

For example:

BitVector<N>

is a portable semantic abstraction.

A hard-coded physical register bank is not.

---

46. Distributed Types

Distributed types may describe:

Node<T>
Process<T>
Service<T>
Channel<T>
Message<T>
Partition<T>
Replica<T>

The grammar must not impose:

MAX_NODES
MAX_REPLICAS
MAX_CHANNELS

---

47. Networking Types

Networking types may express:

Endpoint
Address
Channel<T>
Protocol<P>
Message<T>
Stream<T>

The grammar must not assume a fixed address width or topology unless that is explicitly part of a source-level type.

---

48. Security Types

The type system may integrate semantic security concepts such as:

Secret<T>
Public<T>
Key<T>
Signature<T>
Capability<T>
Identity<T>

but cryptographic implementation remains outside the grammar.

---

49. Temporal / MTS Types

Where the existing MTS architecture is retained, types may represent temporal values such as:

MTS<T>

and future temporal constructs.

The grammar must not impose a maximum number of timelines, branches, histories, or observations.

---

50. Linear and Affine Types

Existing:

linear
affine

syntax remains valuable.

The grammar only recognizes the qualifiers.

Semantic analysis determines:

- use count;
- ownership;
- consumption;
- duplication legality;
- borrowing;
- resource semantics.

---

51. Effects

Type syntax may be associated with effect information.

But effects remain owned by:

grammar/effects/

The type grammar must not create a second effect language.

---

52. Type Constraints

Type constraints must remain distinct from:

resource requirements
capability requirements
hardware constraints
performance constraints
security constraints
execution constraints

For example:

T: Numeric

is a type constraint.

Whereas:

requires capability("quantum.measurement")

is a capability requirement.

And:

requires resource(qubits)

is a resource requirement.

The parser preserves these distinctions.

---

53. Constraint Extensibility

The grammar must support named semantic constraints without hard-coding every future constraint.

Examples:

T: Numeric
T: Comparable
T: Sendable
T: QuantumState
T: HardwareCompatible

A new semantic trait must not require changing the lexer merely because its name is new.

---

54. Lexer Contract

All lexical tokens used by the type grammar must come from the canonical lexer vocabulary.

The repository currently identifies:

grammar/antlr/ZamaniLexer.g4

as the canonical lexer.

The modular lexical documentation is under:

grammar/lexer/

The existing lexer architecture explicitly requires modular lexical documentation to integrate with the canonical lexer rather than creating a competing lexer.

---

55. Required Type-Grammar Token Inventory

The following token classes are required for complete type grammar integration.

55.1 Identifiers

Required:

IDENTIFIER

Identifiers must support the repository's Unicode identifier policy.

---

55.2 Generic/type delimiters

Required:

LESS_THAN
GREATER_THAN
COMMA
DOUBLE_COLON

Used for:

Map<K, V>
module::Type

---

55.3 Grouping

Required:

LPAREN
RPAREN

Used for:

()
(T, U)
fn(T) -> U

---

55.4 Array/slice delimiters

Required:

LBRACKET
RBRACKET

Used for:

[T]
[T; N]
Vector<T>[N]

---

55.5 Array separator

Required:

SEMI

for:

[T; N]

---

55.6 Reference/operator tokens

Required:

AMPERSAND
STAR

for:

&T
&mut T
*T
*mut T

---

55.7 Optional marker

Required:

QUESTION_MARK

for:

T?

---

55.8 Function token

Required:

FN

for:

fn(T) -> U

---

55.9 Function return arrow

Required:

THIN_ARROW

for:

-> T

---

55.10 Ownership/resource qualifiers

Required where standardized:

LINEAR
AFFINE
MUT

---

55.11 Lifetime punctuation

Required:

APOSTROPHE

for:

'a

---

55.12 Primitive type tokens

The canonical lexer must provide the reserved tokens required by the language specification, including the existing vocabulary where retained:

VOID
INT
FLOAT_TYPE
BOOL_TYPE
STR_TYPE
STRING_TYPE
CHAR_TYPE

Do not introduce duplicate spellings for the same primitive type without a compatibility specification.

---

55.13 Result token

Required if "Result<T,E>" is a reserved constructor:

RESULT

If the language instead treats "Result" as an ordinary type name, it must remain:

IDENTIFIER

and the parser must not require a keyword.

This decision must be consistent everywhere.

---

55.14 Never token

Required if "never" is reserved:

NEVER

---

55.15 Quantum type token

If the language formally reserves "Qubit":

QUBIT

Other quantum type names should remain extensible identifiers unless explicitly standardized.

This prevents the type grammar from becoming a closed list of quantum hardware types.

---

55.16 Temporal token

If the language formally reserves "MTS":

MTS

The semantic meaning belongs downstream.

---

56. Required Type-Level Expression Tokens

Where type-level values support arithmetic, the canonical lexer must provide:

PLUS
MINUS
STAR
SLASH
MODULO
AMPERSAND
PIPE
CARET
TILDE
LEFT_SHIFT
RIGHT_SHIFT

plus:

INTEGER
FLOAT
IDENTIFIER

and:

LPAREN
RPAREN
DOUBLE_COLON

The type grammar must consume the canonical token vocabulary.

It must not create a private expression lexer.

---

57. Keywords Required by Type Integration

The type grammar should reserve only words that require syntactic reservation.

The relevant categories are:

Core type keywords

fn
mut
linear
affine
never

Primitive type keywords

void
int
float
bool
str
string
char

where these are formally reserved.

Semantic-domain keywords

Only if the language specification requires reserved syntax:

qubit
MTS

Other names such as:

Tensor
Vector
Matrix
LogicalQubit
PhysicalQubit
GPU
FPGA
QPU
CPU
Accelerator

should remain ordinary identifiers unless there is a demonstrated parser-level reason to reserve them.

This is important for POCO-REAF and future extensibility.

---

58. Keywords That Must NOT Be Added Merely for Types

Do not reserve every domain term as a keyword.

In particular, do not automatically add:

GPU
FPGA
ASIC
QPU
CPU
Tensor
Matrix
Vector
LogicalQubit
PhysicalQubit
QuantumState
Resource
Capability
Accelerator

as keywords.

They can remain identifiers and therefore be extended by libraries, dialects, packages, and future domains.

---

59. Lexer Files That Must Be Kept Synchronized

The type subsystem depends on the following lexical documentation/contracts:

grammar/lexer/README.md
grammar/lexer/tokens.g4
grammar/lexer/keywords.g4
grammar/lexer/identifiers.g4
grammar/lexer/literals.g4
grammar/lexer/numeric-literals.g4
grammar/lexer/string-literals.g4
grammar/lexer/character-literals.g4
grammar/lexer/boolean-literals.g4
grammar/lexer/quantum-literals.g4
grammar/lexer/hardware-literals.g4
grammar/lexer/duration-literals.g4
grammar/lexer/size-literals.g4
grammar/lexer/annotations.g4
grammar/lexer/operators.g4
grammar/lexer/punctuation.g4
grammar/lexer/comments.g4
grammar/lexer/unicode.g4
grammar/lexer/lexer-errors.g4

These must remain documentation/specification components of the canonical lexer architecture rather than independent competing lexers.

---

60. Existing Type Files

The repository already contains numerous type grammar files. They must not all become independent roots.

The canonical architecture is:

grammar/types/types.g4
        │
        ├── primitive types
        ├── named types
        ├── generic types
        ├── composite types
        ├── function types
        ├── tuple types
        ├── arrays/slices
        ├── references/pointers
        ├── option/result
        ├── algebraic types
        ├── dependent types
        ├── resource types
        ├── capability types
        ├── quantum types
        ├── classical types
        ├── hardware types
        ├── temporal types
        └── constraints

The current repository already contains files such as:

affine.g4
algebraic-types.g4
array.g4
array-types.g4
capability.g4
classical.g4
composite-types.g4
dependent.g4
effectful.g4
function.g4
generic.g4
hardware.g4

and additional specialized type files.

These must be assigned ownership rather than renamed unnecessarily.

---

61. Duplicate Filename Policy

Where both:

array.g4
array-types.g4

exist, do not immediately rename either file.

Instead classify them as one of:

canonical implementation
compatibility adapter
legacy/reference
duplicate requiring consolidation

The classification must be recorded in:

grammar/types/README.md
grammar/specification/grammar-authority.md
grammar/compatibility/

No two files may independently define incompatible versions of the same public rule.

---

62. Recommended Existing-File Ownership

Use the following ownership model.

File| Role
"types.g4"| canonical type composition root
"primitive-types.g4"| primitive type syntax
"named.g4"| named/qualified types
"generic.g4"| generic syntax
"generic-types.g4"| compatibility/reference wrapper if retained
"composite-types.g4"| structural composition
"tuple.g4" / "tuple-types.g4"| tuple syntax; one canonical owner
"array.g4"| canonical array/slice syntax if already integrated
"array-types.g4"| compatibility/reference unless promoted
"slice.g4"| slice syntax
"function.g4"| canonical function type syntax
"function-types.g4"| compatibility/reference unless promoted
"reference.g4" / "reference-types.g4"| reference syntax
"pointer.g4"| pointer syntax
"option-types.g4"| optional syntax
"result-types.g4"| result syntax
"never.g4"| never type
"unit.g4"| unit type
"algebraic-types.g4"| algebraic type composition
"dependent.g4"| dependent/value-parameterized syntax
"resource.g4" / "resource-types.g4"| resource type syntax
"capability.g4" / "capability-types.g4"| capability type syntax
"quantum.g4" / "quantum-types.g4"| quantum type syntax
"classical.g4" / "classical-types.g4"| classical type syntax
"hardware.g4" / "hardware-types.g4"| hardware semantic type syntax
"affine.g4"| affine qualifier syntax
"linear.g4"| linear qualifier syntax
"effectful.g4"| effect/type integration only

The exact promotion must be verified against actual references before deleting or rewriting anything.

---

63. "types.g4" Contract

"types.g4" is the public type composition boundary.

It owns:

typeExpression
typeQualifier
typeCore
typePostfix

and composes specialized type rules.

It must not duplicate every specialized rule.

It must not contain implementation semantics.

It must not contain lexer rules.

It must not contain Rust actions.

It must not contain hardware selection.

It must not contain QEC logic.

It must not contain runtime code.

---

64. Canonical Public Rule

The canonical public rule is:

typeExpression
    : typeQualifier* typeCore typePostfix*
    ;

The exact implementation may evolve to resolve ambiguity, but the architectural invariant remains:

«There is one canonical public source-level type-expression entry point.»

---

65. Postfix Types

Postfix type constructors must be explicitly controlled.

Current canonical optional syntax:

T?

Future postfix constructs must not be added casually.

Each must define:

- lexical token;
- precedence;
- associativity;
- AST mapping;
- semantic mapping;
- ambiguity behavior;
- compatibility;
- diagnostics.

---

66. Type Grammar and Expression Grammar

The type grammar must not silently become a second expression grammar.

Type-level values are a deliberately constrained subset.

The canonical architecture is:

typeValueExpression
        ↓
typeValue AST
        ↓
semantic dependent/value analysis

not:

type grammar
        ↓
entire ordinary expression grammar

This avoids parser ambiguity and keeps the type boundary independently maintainable.

---

67. Parenthesized Types

Support:

(T)

only where required to disambiguate nested type syntax.

The parser must distinguish:

(T)

from:

(T, U)

and:

()

according to the canonical tuple/unit rules.

---

68. Recursive Types

Recursive source types must be representable without language-level finite depth.

Examples:

List<T>
Tree<T>
Node<T>
Option<Box<Node<T>>>

The Rust AST may use:

Box<TypeExpr>

where necessary for recursive enum representation.

That is an implementation requirement of Rust representation, not a Zamani language limit.

---

69. Rust Safety

All Rust integration must remain:

Rust 1.97
Rust 1.97.1
edition 2021
safe Rust
no unsafe

The repository's frontend "TypeExpr" explicitly follows this model and uses no unsafe implementation.

The grammar itself must never require unsafe Rust parser actions.

---

70. No Embedded Semantic Actions

ANTLR grammar files must not perform:

- allocation;
- hardware discovery;
- type inference;
- QEC;
- routing;
- scheduling;
- backend selection;
- I/O;
- network access;
- filesystem access.

The grammar produces syntax.

---

71. Diagnostics Contract

Every type syntax error must be diagnosable with:

- source span;
- offending token;
- expected category;
- stable diagnostic identifier where the repository's diagnostic system provides one;
- human-readable message;
- recovery behavior where parser recovery is possible.

Examples:

unterminated generic argument list
expected type after &
expected type after *
expected type after ->
expected generic argument
expected closing >
expected closing ]
expected type after :

Diagnostics must not reveal implementation-specific machine limits as language restrictions.

---

72. Error Recovery

Parser recovery must preserve enough source structure for IDEs and diagnostics.

Malformed:

Vec<

must not cause the entire compilation unit to become unrecoverable if normal parser recovery can continue.

Recovery is parser responsibility.

Semantic validity remains a separate phase.

---

73. Determinism

Given identical source and parser configuration:

source → token stream → parse tree

must be deterministic.

Type grammar must not depend on:

- hardware;
- runtime state;
- network state;
- random values;
- system time;
- device availability.

---

74. Source Locations

Every type grammar construct must preserve source locations sufficiently for:

- diagnostics;
- IDE tooling;
- formatting;
- refactoring;
- semantic errors;
- source mapping;
- provenance.

The grammar must not discard source structure needed by the frontend AST.

---

75. Formatting

The grammar must preserve enough structure for a formatter to distinguish:

Map<K,V>
Map<K, V>

without changing semantic meaning.

Formatting belongs to tooling.

---

76. Serialization

If "TypeExpr" is serialized, serialization belongs to the AST/schema layer.

The grammar must not define its own serialization format.

The type grammar must remain compatible with:

src/frontend/ast/node/types/type_expr.rs

and its schema/versioning policy.

---

77. Compatibility

Every type syntax change must specify:

introduced version
stable version
deprecated version
removed version
migration

where applicable.

Never silently change:

T?

from one semantic meaning to another.

---

78. Backward Compatibility

Existing valid Zamani programs should continue to parse unless the language specification deliberately introduces a breaking change.

If syntax conflicts arise:

1. preserve existing meaning;
2. introduce explicit disambiguation;
3. provide compatibility guidance;
4. add positive and negative tests.

---

79. Forward Compatibility

Unknown named types must remain representable.

For example:

FutureQuantumType<T>
FutureAccelerator<T>
FutureDomain::Type

should not require the lexer or parser to know the future semantic meaning.

This is one of the key mechanisms that lets Zamani grow without rewriting the core grammar.

---

80. Domain Extensibility

A future domain should normally be able to introduce:

FutureDomain::Type
FutureDomain::Resource<T>
FutureDomain::Capability<C>

without modifying the universal type grammar.

The universal grammar should recognize the structural form.

Semantic/domain registries determine meaning later.

---

81. Hardware Independence

The type grammar must never depend on:

CPU model
GPU model
FPGA family
QPU vendor
ASIC implementation
device count
node count
memory capacity
network topology

A hardware-specific type may exist as a domain extension, but its source-level meaning must remain explicit and its realization must be downstream.

---

82. Resource Requirements Are Not Types

Do not confuse:

Qubit

with:

requires resource(qubits)

or:

requires capability("quantum.measurement")

These are different layers.

Type:

Qubit

Resource:

resource requirement

Capability:

capability requirement

Constraint:

constraint

Preference:

preference

Implementation decision:

target realization

The grammar architecture must preserve these distinctions.

---

83. Type vs Representation

The type:

int

is not the same thing as:

i64

The type:

Qubit

is not the same thing as:

physical_qubit_17

The type:

Tensor<Float, N>

is not the same thing as:

GPU tensor allocation

The type:

Memory<T>

is not the same thing as:

DDR5 bank 3

The type grammar must maintain these boundaries.

---

84. Type-to-IR Contract

The complete integration is:

grammar/types/
       │
       ▼
frontend TypeExpr
       │
       ▼
semantic type
       │
       ├── classical semantic model
       ├── quantum semantic model
       ├── HDL semantic model
       ├── resource semantic model
       └── future domain semantic models
       │
       ▼
canonical semantic IR
       │
       ├── classical IR
       ├── quantum::ir
       └── domain IR

The grammar does not directly lower to target IR.

---

85. Quantum Type-to-IR Contract

Quantum source types follow:

quantum type syntax
        ↓
TypeExpr
        ↓
semantic quantum type
        ↓
quantum::ir

Then:

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
   ↓
target

This keeps the canonical "quantum::ir" boundary intact.

---

86. Classical Type-to-IR Contract

Classical types follow:

type syntax
   ↓
TypeExpr
   ↓
semantic classical type
   ↓
classical semantic IR
   ↓
optimization
   ↓
target lowering

---

87. HDL Type-to-IR Contract

HDL-related types follow:

HDL type syntax
   ↓
TypeExpr
   ↓
HDL semantic model
   ↓
HDL/domain IR
   ↓
synthesis/lowering
   ↓
target realization

The grammar must not contain synthesis algorithms.

---

88. Resource Type Integration

Resource types feed:

semantic analysis
        ↓
resource requirements
        ↓
resource manager
        ↓
capability discovery
        ↓
target selection
        ↓
deployment

The grammar does not discover resources.

---

89. Scheduling Boundary

Types may influence semantic resource requirements.

Types do not schedule operations.

Scheduling belongs to:

grammar/execution/
src/quantum/scheduling/

and related compiler systems.

---

90. Routing Boundary

Types may distinguish logical abstractions from physical representations.

They do not choose physical mappings.

Routing owns:

logical → physical realization

---

91. Calibration Boundary

No calibration data belongs in type grammar.

A type can describe a semantic requirement that eventually interacts with calibration.

Actual calibration belongs downstream.

---

92. Runtime Boundary

Types do not allocate runtime memory or devices.

Runtime representation is determined after semantic analysis and lowering.

---

93. ABI Boundary

Type syntax must not encode an ABI unless the language explicitly provides an ABI/type-interop construct.

ABI belongs under interoperability/backend/compiler layers.

---

94. Foreign Languages

C/C++/Rust/Python/QIR/OpenQASM/HDL interoperability must not redefine Zamani's core type grammar.

Foreign types must lower through explicit interoperability contracts.

For example:

extern type CType

is semantically different from making all C types native Zamani types.

---

95. Dialects

A dialect may extend type syntax.

A dialect must declare:

name
version
syntax additions
semantic additions
AST mapping
IR mapping
compatibility
feature gates

A dialect must not silently modify the meaning of a stable core type.

---

96. Macros

Macros may generate type syntax.

Macro expansion must produce ordinary Zamani type syntax that goes through the same structural and semantic validation pipeline.

Macros must not bypass type checking.

---

97. Metaprogramming

Metaprogramming may inspect or generate types where permitted.

It must not create a second type system.

Generated types must enter the same canonical:

TypeExpr → semantic type

pipeline.

---

98. Tests Required

The type subsystem is not complete until tests cover:

positive/
negative/
boundary/
scalability/
compatibility/
diagnostics/
determinism/

---

99. Required Positive Tests

At minimum:

int
bool
float
string
char
void
never
()

T
module::T

Vec<T>
Map<K, V>

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
&'a T
&'a mut T

*T
*mut T

T?
Option<T>

Result<T, E>

Qubit
LogicalQubit
QuantumState<T>
QRegister<N>

Tensor<T, N>
Tensor<T, Rows, Columns>

Resource<T>
Capability<C>

linear T
affine T

where each construct is formally standardized.

---

100. Negative Tests

Must reject malformed forms such as:

Vec<
Vec<>
Vec<T
Vec<T>>
[T
[T;]
[T; N
fn(
fn(T
&T
&mut
*
*mut
Result<T>
Result<>
(T
(T,

according to the intended parser/recovery rules.

---

101. Boundary Tests

Test:

one type argument
many type arguments
one tuple member
many tuple members
one dimension
many dimensions
deeply nested types
deeply nested generics
long qualified paths
large symbolic expressions
Unicode identifiers

No test should accidentally define a language ceiling.

---

102. Scalability Tests

The tests must demonstrate that grammar structure does not impose artificial limits on:

generic arity
tuple arity
type nesting
path depth
array dimensions
symbolic expressions
function parameter count
quantum cardinality expressions
resource dimensions
tensor dimensions

Test generation should use scalable parameterized fixtures rather than a hard-coded “maximum supported” number.

---

103. POCO-REAF Tests

The same source type program must produce structurally identical frontend AST semantics regardless of target configuration.

For example:

Tensor<Float, N>

must not parse differently because the target changes from:

CPU
GPU
FPGA
QPU
cluster

---

104. Hard-Coding Audit

Every type grammar change must be scanned for:

MAX_
LIMIT_
CAPACITY
QUBIT_0
QUBIT_1
CPU0
GPU0
FPGA0
QPU0
NODE0

and equivalent hidden finite assumptions.

A literal such as:

Vector<Int, 1024>

is valid program data.

A grammar rule such as:

dimension: 1..1024

is not acceptable as a universal language rule.

---

105. Lexer Hard-Coding Audit

The lexer must not turn implementation limits into lexical restrictions.

For example, it must not reject:

18446744073709551617

merely because a particular Rust integer type cannot represent it during lexing.

The lexer recognizes numeric syntax.

Semantic analysis determines representability.

---

106. Source-Span Contract

Every AST type node must retain sufficient location information through the parser/frontend architecture.

This is required for:

- diagnostics;
- IDE;
- formatter;
- refactoring;
- provenance;
- compatibility tooling.

---

107. Documentation Contract

Every new type grammar file must document:

File
Purpose
Status
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Public Grammar Contract
AST Contract
Semantic Contract
IR Integration
Compiler Integration
Runtime Integration
Tooling Integration
Cross-Domain Integration
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility
Diagnostics
Determinism
Security
Performance
Hard-Coding Audit
Completion Criteria

A file is not considered independently complete until all of these are known.

---

108. Independent Completion Rule

A developer working on:

grammar/types/generic.g4

must be able to finish it without waiting for an undocumented future change to another file.

Before completion, the developer must already know:

lexer tokens
parser entry point
AST representation
generic argument representation
semantic interpretation
diagnostic behavior
IR consequences
downstream consumers
tests
compatibility

This directly implements the requirement:

«Finish one file without having to reopen it merely because another file was subsequently designed.»

---

109. Feature Contract

For every significant type feature, define:

feature ID
syntax owner
lexer tokens
parser rule
AST node
semantic model
IR mapping
compiler consumers
runtime consumers
tests
compatibility

The type subsystem must not accept syntax whose AST/semantic destination is undefined.

---

110. Feature Lifecycle

Every new type feature follows:

proposal
  ↓
specification
  ↓
lexer contract
  ↓
grammar contract
  ↓
AST contract
  ↓
semantic contract
  ↓
IR contract
  ↓
compiler integration
  ↓
tests
  ↓
compatibility
  ↓
stable

No feature becomes stable merely because its grammar parses.

---

111. Completion Criteria for "grammar/types/"

The type subsystem is production-ready only when:

- [ ] one canonical "typeExpression" exists;
- [ ] all type files have explicit ownership;
- [ ] duplicate files are classified;
- [ ] no competing type grammar exists;
- [ ] lexer vocabulary is canonical;
- [ ] all required tokens exist;
- [ ] keyword policy is explicit;
- [ ] Unicode identifiers work;
- [ ] generic arguments are represented correctly;
- [ ] type/value/resource/capability arguments are distinguished where required;
- [ ] recursive types work;
- [ ] symbolic dimensions work;
- [ ] dependent/value-parameterized types work;
- [ ] no artificial machine limits exist;
- [ ] quantum types are target-independent;
- [ ] hardware types are target-independent;
- [ ] classical types are target-independent;
- [ ] HDL types integrate cleanly;
- [ ] resource types integrate cleanly;
- [ ] capability types integrate cleanly;
- [ ] temporal types integrate cleanly;
- [ ] linear/affine types integrate cleanly;
- [ ] references and pointers integrate cleanly;
- [ ] function types integrate cleanly;
- [ ] option/result/never/unit integrate cleanly;
- [ ] AST mapping is complete;
- [ ] semantic mapping is complete;
- [ ] IR mapping is documented;
- [ ] quantum integration terminates at "quantum::ir";
- [ ] QEC is downstream;
- [ ] ZQN is downstream;
- [ ] HAL is downstream;
- [ ] routing is downstream;
- [ ] scheduling is downstream;
- [ ] runtime is downstream;
- [ ] ABI is downstream;
- [ ] no unsafe Rust is required;
- [ ] diagnostics are complete;
- [ ] positive tests exist;
- [ ] negative tests exist;
- [ ] boundary tests exist;
- [ ] scalability tests exist;
- [ ] compatibility tests exist;
- [ ] determinism tests exist;
- [ ] hard-coding audit passes.

---

112. Required Repository Integration

The type grammar must be integrated with, but not coupled to, at least:

grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/DESIGN.md

grammar/lexer/
grammar/core/
grammar/expressions/
grammar/statements/
grammar/declarations/
grammar/functions/
grammar/modules/
grammar/effects/
grammar/memory/
grammar/concurrency/

grammar/classical/
grammar/quantum/
grammar/hybrid/
grammar/hdl/
grammar/hardware/
grammar/resources/
grammar/distributed/
grammar/ai/
grammar/data/
grammar/networking/
grammar/security/

grammar/compile/
grammar/execution/
grammar/interoperability/
grammar/dialects/
grammar/macros/
grammar/metaprogramming/

grammar/spec/
grammar/specification/
grammar/validation/
grammar/compatibility/
grammar/tests/

src/frontend/ast/
src/quantum/ir/
src/quantum/
compiler
runtime
HAL
resource management
scheduling
routing
optimization
QEC
ZQN

---

113. Authority Rules

The authority hierarchy for types is:

language specification
        ↓
canonical grammar composition
        ↓
frontend AST contract
        ↓
semantic type contract
        ↓
IR contract
        ↓
implementation
        ↓
tests

"Zamani-Grammar.md" cannot silently introduce stable syntax.

"grammar/grammar.md" cannot independently define syntax.

Generated documentation cannot override the specification.

Legacy grammar files cannot silently override "types.g4".

---

114. "grammar/grammar.md"

"grammar/grammar.md" should document what the current implementation accepts.

It is not the semantic authority.

Its type section must be consistent with:

types.g4
frontend TypeExpr
lexer vocabulary

---

115. "grammar/Zamani-Grammar.md"

This remains a broad design/reference document.

Features described there become production language features only after passing:

specification
→ grammar
→ AST
→ semantics
→ IR
→ implementation
→ tests

This prevents aspirational syntax from becoming accidental language authority.

---

116. "grammar/Zamani.g4"

"Zamani.g4" remains the top-level grammar composition root.

It should delegate type parsing to the canonical type grammar.

It must not define a second incompatible "typeExpression".

---

117. Lexer Integration

The type grammar must consume tokens from:

grammar/antlr/ZamaniLexer.g4

through the repository's established ANTLR architecture.

The modular files under:

grammar/lexer/

document and organize the lexical vocabulary.

They must not create a second lexer authority.

---

118. Existing Lexer Architecture Correction

The repository currently contains both:

grammar/lexer/

and:

grammar/antlr/ZamaniLexer.g4

and the repository's lexer documentation explicitly recognizes "ZamaniLexer.g4" as the canonical lexer.

Therefore:

grammar/lexer/

must be treated as the lexical specification/modular contract.

grammar/antlr/ZamaniLexer.g4

must remain the canonical ANTLR implementation/composition artifact until the repository deliberately changes that architecture.

Do not introduce another competing lexer.

---

119. Type Grammar Must Not Import Runtime

The type grammar must not depend on:

runtime
HAL
QEC
ZQN
scheduler
router
device drivers

The dependency direction is always downstream.

---

120. Type Grammar Must Not Import Hardware

The grammar must not query:

CPU
GPU
FPGA
QPU
memory
network
device

while parsing.

Compilation and runtime may later query resources.

---

121. Type Grammar Must Not Perform Capability Discovery

A type such as:

QuantumState

does not mean that a QPU exists.

A type such as:

GPUBuffer<T>

does not mean a GPU exists.

Capability discovery happens later.

---

122. Type Grammar Must Not Perform Resource Allocation

The parser never allocates:

qubits
memory
threads
cores
GPUs
nodes
accelerators

It only represents source syntax.

---

123. Type Grammar Must Not Perform QEC

The parser never:

encodes logical qubits
chooses codes
allocates ancillas
generates syndromes
decodes

Those belong to quantum/compiler subsystems.

---

124. Type Grammar Must Not Perform Scheduling

The parser does not determine:

when
where
in what order
on which physical resource

operations execute.

---

125. Type Grammar Must Not Perform Routing

The parser does not map logical entities to physical topology.

---

126. Type Grammar Must Not Perform Optimization

A type describes source meaning.

Optimization may later change implementation without changing type semantics.

---

127. Security

The type grammar must:

- perform no I/O;
- perform no network access;
- execute no source code;
- invoke no external programs;
- access no device;
- require no unsafe Rust;
- avoid parser actions with arbitrary side effects.

Malformed source must produce diagnostics rather than panics wherever the frontend architecture permits recovery.

---

128. Performance

Grammar design must avoid unnecessary ambiguity.

Prefer:

qualified path
generic application
postfix constructors

over large closed alternatives.

Do not enumerate every library type.

Do not enumerate every hardware type.

Do not enumerate every quantum gate.

Do not enumerate every AI model.

Do not enumerate every future accelerator.

This keeps grammar growth sublinear with respect to ecosystem growth.

---

129. Future-Proofing

A future type should ideally be expressible as:

domain::Type
domain::Type<T>
domain::Resource<T>
domain::Capability<C>

without changing the universal grammar.

This is a major requirement for:

«From Atom to Everywhere.»

---

130. What Must Be Added to "grammar/lexer/"

The type subsystem requires the following lexical vocabulary to be explicitly documented and validated.

Token categories

IDENTIFIER

INTEGER
FLOAT

LPAREN
RPAREN
LBRACKET
RBRACKET

LESS_THAN
GREATER_THAN

COMMA
SEMI

DOUBLE_COLON

QUESTION_MARK

AMPERSAND
STAR

PLUS
MINUS
SLASH
MODULO

PIPE
CARET
TILDE

LEFT_SHIFT
RIGHT_SHIFT

APOSTROPHE

THIN_ARROW

Type-related reserved tokens

Where formally reserved:

FN
MUT
LINEAR
AFFINE
VOID
INT
FLOAT_TYPE
BOOL_TYPE
STR_TYPE
STRING_TYPE
CHAR_TYPE
NEVER
RESULT
QUBIT
MTS

The exact token spelling must follow the canonical lexer.

No duplicate token definitions should be introduced merely to make individual type files convenient.

---

131. Tokens That Must Remain Generic

The following should normally remain:

IDENTIFIER

rather than become an enormous reserved keyword set:

Tensor
Vector
Matrix
LogicalQubit
PhysicalQubit
GPU
CPU
FPGA
ASIC
QPU
Accelerator
Resource
Capability
QuantumState
FutureType

This is intentional.

It permits ecosystem growth without lexer churn.

---

132. Type-Specific Literal Requirements

The type grammar may require lexical support for:

integer literals
floating literals
symbolic identifiers
qualified value paths

It must not introduce a second literal system.

---

133. No Fixed Numeric Representation

The lexer identifies:

INTEGER
FLOAT

The semantic layer determines:

- signedness;
- precision;
- exactness;
- representability;
- compile-time evaluation;
- target representation.

---

134. No Fixed Generic Count

The grammar must accept:

G<T>
G<T, U>
G<T, U, V>
...

subject only to implementation resource availability.

---

135. No Fixed Tuple Count

The grammar must accept:

(T,)
(T, U)
(T, U, V)
...

without a language-defined finite maximum.

---

136. No Fixed Function Parameter Count

The grammar must accept:

fn()
fn(T)
fn(T, U)
fn(T, U, V)
...

without a grammar-defined maximum.

---

137. No Fixed Tensor Rank

The grammar must accept symbolic tensor dimensions without a language-defined rank maximum.

---

138. No Fixed Quantum Cardinality

The grammar must accept:

QRegister<N>

without imposing a maximum "N".

---

139. No Fixed Resource Cardinality

Resource types must not encode a maximum number of:

cores
GPUs
QPUs
FPGAs
nodes
devices
memory units

---

140. Testing Matrix

The final test matrix must contain:

types/
├── primitive/
├── named/
├── qualified/
├── generic/
├── generic-type-arguments/
├── generic-value-arguments/
├── generic-resource-arguments/
├── generic-capability-arguments/
├── tuple/
├── array/
├── slice/
├── function/
├── reference/
├── pointer/
├── optional/
├── result/
├── never/
├── unit/
├── algebraic/
├── dependent/
├── type-values/
├── linear/
├── affine/
├── resource/
├── capability/
├── classical/
├── quantum/
├── hardware/
├── temporal/
├── diagnostics/
├── negative/
├── boundary/
├── scalability/
├── compatibility/
└── determinism/

---

141. Integration Tests

At least one complete end-to-end test must prove:

source
 ↓
lexer
 ↓
parser
 ↓
TypeExpr
 ↓
structural validation
 ↓
semantic type resolution
 ↓
IR

for:

classical type
quantum type
hybrid type
HDL type
resource type
generic type
dependent type

---

142. Cross-Domain Tests

The following combinations must be tested:

classical + quantum
classical + HDL
quantum + hardware
quantum + resource
AI + tensor
AI + accelerator
distributed + resource
HDL + hardware
quantum + resilience
type + capability
type + effect
type + memory
type + concurrency

The grammar must remain modular.

---

143. Acceptance Criterion

A type feature is not complete when:

ANTLR accepts it

It is complete only when:

ANTLR
+
lexer
+
AST
+
structural validation
+
semantic analysis
+
IR mapping
+
compiler integration
+
tests
+
diagnostics
+
compatibility

all agree.

---

144. Final Type Architecture

The intended final architecture is:

                    Zamani Type System
                           │
                    typeExpression
                           │
          ┌────────────────┼────────────────┐
          │                │                │
       named           generic          primitive
          │                │                │
          ├────────┬───────┼───────┬────────┤
          │        │       │       │        │
       tuple     array   function reference pointer
          │        │       │       │        │
          ├────────┴───────┴───────┴────────┤
          │                                 │
       optional/result/never/unit       dependent
          │                                 │
          ├─────────────────────────────────┤
          │                                 │
       classical                         quantum
          │                                 │
       hardware                         resource
          │                                 │
       capability                        temporal
          │                                 │
          └──────────────┬──────────────────┘
                         ▼
                    TypeExpr
                         ▼
                 semantic type model
                         ▼
                  canonical semantic IR
                         │
             ┌───────────┼────────────┐
             ▼           ▼            ▼
        classical     quantum::ir    HDL/domain
             │           │            │
             └───────────┼────────────┘
                         ▼
                    optimization
                         ▼
                  routing/scheduling
                         ▼
                     resilience
                         ▼
                        ZQN
                         ▼
                        HAL
                         ▼
                  target realization

---

145. Final Non-Negotiable Rule

The type grammar exists to describe what a program means, not to describe what today's machine happens to support.

Therefore:

Type
≠
hardware instance

Type
≠
resource allocation

Type
≠
physical topology

Type
≠
runtime representation

Type
≠
ABI

Type
≠
QEC implementation

Type
≠
routing

Type
≠
scheduling

The correct relationship is:

SOURCE TYPE
    ↓
SOURCE SEMANTICS
    ↓
RESOURCE / CAPABILITY REQUIREMENTS
    ↓
CANONICAL IR
    ↓
OPTIMIZATION
    ↓
ROUTING
    ↓
SCHEDULING
    ↓
RESILIENCE / QEC / ZQN
    ↓
HAL
    ↓
TARGET

That separation is what permits:

Program Once
Compile Once
Run Everywhere
Anywhere
Forever

without turning the grammar into a catalog of today's machines.

---

146. Definition of Done

"grammar/types/README.md" is itself complete when it is sufficient for a developer to implement or audit every type grammar file without needing an undocumented architectural decision from another future file.

The type subsystem is complete when:

Specification
     ↓
Lexer
     ↓
types.g4
     ↓
specialized type grammar
     ↓
TypeExpr
     ↓
semantic types
     ↓
IR
     ↓
compiler
     ↓
runtime

is traceable for every supported type construct.

No unsupported type syntax may be silently accepted.

No supported type syntax may lack an AST contract.

No AST type may lack a semantic interpretation.

No semantic type may lack a defined downstream integration boundary.

No type may encode an arbitrary machine capacity.

No type grammar may introduce a competing IR.

No type grammar may require unsafe Rust.

No type grammar may depend on hardware availability.

No type grammar may depend on runtime state.

No type grammar may silently create a second lexer.

Only when all of those conditions hold is "grammar/types/" production-ready.