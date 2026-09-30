Zamani AST Conformance

Path: "grammar/compatibility/ast-conformance.md"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Status: Production AST conformance contract
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety policy: Safe Rust only; production compiler implementation MUST NOT use Rust "unsafe"
Primary objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the production conformance contract between:

grammar/specification/
        ↓
grammar/spec/
        ↓
grammar/Zamani.g4
        ↓
src/lexer.rs
        ↓
src/parser.rs
        ↓
src/ast/
        ↓
semantic analysis
        ↓
canonical semantic representation
        ↓
IR
        ↓
optimization / lowering
        ↓
quantum::ir / classical / HDL / target-independent IR
        ↓
routing / scheduling / resilience / ZQN
        ↓
HAL / backend / runtime

This file specifically owns the AST boundary.

It defines:

- what the Zamani AST represents;
- what the AST owns;
- what the AST does not own;
- the mapping from syntax to AST;
- source-span requirements;
- AST identity requirements;
- domain-neutrality requirements;
- quantum AST requirements;
- classical AST requirements;
- HDL AST requirements;
- hybrid AST requirements;
- resource/capability AST requirements;
- effect AST requirements;
- memory/concurrency AST requirements;
- compatibility requirements;
- versioning requirements;
- serialization requirements where applicable;
- deterministic traversal requirements;
- scalability requirements;
- diagnostics requirements;
- AST-to-semantic integration;
- AST-to-IR integration;
- conformance tests;
- negative tests;
- boundary tests;
- hard-coding audits.

This document does not define target-specific lowering.

---

2. Authority

The AST contract is subordinate to the following architecture:

grammar/DESIGN.md
        ↓
grammar/specification/
        ↓
grammar/spec/
        ↓
grammar/Zamani.g4
        ↓
src/lexer.rs
        ↓
src/parser.rs
        ↓
src/ast/
        ↓
this file
        ↓
semantic analysis
        ↓
IR

The following files remain separate authorities:

File| Authority
"grammar/DESIGN.md"| Grammar architecture
"grammar/specification/"| Normative language specification
"grammar/spec/"| Feature contracts
"grammar/Zamani.g4"| Canonical ANTLR syntax composition
"src/lexer.rs"| Executable lexical implementation
"src/parser.rs"| Executable parsing implementation
"src/ast/"| Executable AST implementation
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Historical/proposed/experimental language design
"grammar/compatibility/ast-conformance.md"| AST conformance contract
semantic analysis| Meaning and validity
canonical IR| Compiler representation
"src/quantum/ir/"| Canonical quantum IR
routing| Physical/resource realization
scheduling| Temporal/resource scheduling
resilience/QEC/ZQN| Fault/noise/resilience processing
HAL/backend| Target realization

No file may silently supersede this authority model.

---

3. Current Implementation Baseline

The current repository contains an actual AST implementation under:

src/ast/mod.rs

The implementation currently includes, among other structures:

Program
Statement
Expression
Literal
Identifier
TypeExpr
Type
Parameter
Pattern
MatchCase
CatchArm
ClassMember
InterfaceMember
Visibility

and additional domain-oriented structures.

The current "Expression" representation includes variants such as:

Identifier
Literal
Prefix
Infix
If
Block
Match
Loop
Call
Lambda
Array
Tuple
Struct
Index
Range
MemberAccess
MethodCall
Cast
TypeAscription
Assign
CompoundAssign
Try
TryCatch
Await
Async
Spawn
New
QuantumOp
Entangle
NanoOp
Recall
Remember
Learn
Perform
Zamani
Sasa
Macro

The current "Literal" representation includes:

Integer(i64, Span)
Float(f64, Span)
String(String, Span)
Boolean(bool, Span)
Char(char, Span)
Null(Span)
Unit(Span)
Quantum(String, Span)
Nano(String, Span)
MTS(String, Span)

The current resolved "Type" representation includes fixed-width Rust-backed variants such as:

Int(IntWidth)
UInt(IntWidth)
Float(FloatWidth)
Array(Box<Type>, Option<usize>)
...

These are existing implementation facts.

This document does not pretend that these representations are already the final production model.

The production conformance work must bring them into alignment with the language architecture without unnecessarily renaming existing files.

---

4. AST Definition

The Zamani AST is a structural representation of the source program after lexical and syntactic analysis.

The AST represents:

WHAT the programmer wrote

and:

HOW that source is structurally organized

It does not represent:

WHICH physical CPU
WHICH GPU
WHICH physical qubit
WHICH FPGA resource
WHICH memory bank
WHICH network node
WHICH vendor instruction
WHICH calibration table
WHICH scheduling slot

Those belong downstream.

---

5. AST Responsibilities

The AST owns:

1. source structure;
2. source ordering;
3. declarations;
4. statements;
5. expressions;
6. patterns;
7. type syntax;
8. generic parameters;
9. attributes;
10. modifiers;
11. names;
12. source spans;
13. syntactic literals;
14. syntactic resource/capability requirements;
15. syntactic domain operations;
16. syntactic effects;
17. syntactic control flow;
18. syntactic quantum operations;
19. syntactic HDL intent;
20. syntactic hybrid computation;
21. syntactic interoperability constructs;
22. syntax-level metadata.

---

6. AST Does Not Own

The AST MUST NOT own:

- type inference results;
- physical device selection;
- physical qubit allocation;
- physical register allocation;
- CPU core assignment;
- GPU assignment;
- FPGA placement;
- ASIC cell selection;
- QPU calibration;
- pulse calibration;
- routing decisions;
- scheduling decisions;
- QEC decisions;
- fault-model realization;
- backend-specific instruction selection;
- runtime state;
- live resource availability;
- machine topology;
- hardware inventory;
- compiler caches;
- runtime handles;
- mutable global compiler state.

These belong downstream.

---

7. AST Invariant

Every AST node MUST have enough structural information for downstream semantic analysis to determine its meaning without re-parsing source text.

Semantic analysis MUST NOT need to inspect raw source text merely to understand ordinary syntax.

Source text may still be retained separately for diagnostics and tooling.

---

8. Source Span Contract

Every source-level AST node that corresponds to source syntax MUST have an associated source span.

A span identifies:

source file / source identity
start position
end position

The exact representation remains owned by the existing source-map/span implementation.

AST nodes MUST NOT invent competing span systems.

The existing "Span" type is therefore the canonical source-location mechanism.

---

9. Span Requirements

Spans MUST:

- be deterministic;
- be reproducible;
- preserve source ordering;
- identify the originating source;
- support diagnostics;
- support IDE tooling;
- support source mapping;
- survive AST transformations where possible;
- never depend on a target machine;
- never depend on runtime addresses.

For generated nodes, the implementation MUST distinguish:

source-originated
generated
synthetic
desugared

where that distinction is required by diagnostics or tooling.

---

10. AST Node Identity

AST identity MUST NOT depend on:

- memory addresses;
- pointer addresses;
- Rust object identity;
- hash-map iteration order;
- target device identifiers;
- physical resource identifiers.

Where stable node identity is required, it must be represented explicitly and deterministically.

Node IDs MUST remain valid across supported compiler stages or have an explicit remapping contract.

---

11. Program Root

The existing:

Program

remains the canonical AST root.

The program root owns:

- top-level items/statements;
- source span;
- program-level structure.

It does not own:

- compilation results;
- target configuration;
- runtime state;
- generated machine code.

A program must be representable regardless of whether it ultimately targets:

embedded
CPU
multicore
GPU
FPGA
ASIC
QPU
quantum simulator
accelerator
HPC
cluster
distributed/cloud
future target

---

12. Statements

"Statement" owns source-level control and declaration structure.

Examples include:

let/binding
expression statement
return
break
continue
if
loop
while
for
match
declaration
resource requirement
effect operation
domain operation

The exact existing enum remains authoritative until a coordinated AST migration is made.

A grammar feature MUST NOT be declared production-ready until its "Statement" representation is defined.

---

13. Expressions

"Expression" is the canonical AST representation for expressions.

The current implementation already provides broad expression coverage.

The following existing structural categories are production-compatible:

Identifier
Literal
Prefix
Infix
If
Block
Match
Loop
Call
Lambda
Array
Tuple
Struct
Index
Range
MemberAccess
MethodCall
Cast
TypeAscription
Assign
CompoundAssign
Try
TryCatch
Await
Async
Spawn
New

These must remain target-neutral.

---

14. Domain-Specific Expression Rule

Existing variants such as:

QuantumOp
NanoOp
Recall
Remember
Learn
Perform
Zamani
Sasa
Macro

MUST NOT automatically be interpreted as proof that each concept deserves a permanently independent semantic execution model.

Each such variant requires a feature contract defining:

grammar
lexer
AST
semantic meaning
capabilities
effects
resource requirements
IR mapping
compiler consumers
runtime consumers
tests
compatibility

If multiple variants represent the same semantic category, they should converge through semantic lowering rather than create multiple competing IRs.

---

15. Generic Operation Principle

For extensible domains, the AST should prefer structured operation data over closed enumerations.

Conceptually:

Operation {
    name
    namespace
    operands
    parameters
    results
    attributes
    modifiers
    effects
    capabilities
    source_span
}

rather than:

Operation::H
Operation::X
Operation::Y
Operation::Z
Operation::CNOT
...

This is especially important for quantum computing.

---

16. Quantum AST Contract

The quantum AST MUST support operations whose names are not known to the compiler when the grammar is authored.

Valid semantic forms include:

apply H ...
apply X ...
apply custom_gate ...
apply vendor.operation ...
apply operation(parameter) ...

The AST therefore MUST preserve enough information to represent:

operation name
operation namespace
operands
parameters
results
attributes
modifiers
controls
targets
conditions
effects
capabilities
source span

The parser must not require a closed list of physical or vendor gates.

---

17. Quantum AST → Semantic Model

The required direction is:

Zamani syntax
    ↓
AST quantum operation
    ↓
semantic quantum operation
    ↓
quantum::ir

The frontend MUST NOT introduce a second canonical quantum IR.

The canonical boundary remains:

quantum::ir

as established by the existing repository architecture.

---

18. Quantum AST Must Not Own Physical Qubits

The AST may contain source-level concepts such as:

q[0]
q[i]
Qubit[n]
logical q

but it MUST NOT resolve these to physical hardware.

For example:

logical q0

may later become:

physical qubit 17

but:

physical qubit 17

is not part of the portable source AST unless explicitly written as a target-specific realization.

---

19. Quantum Resource Requirements

The AST may represent requirements such as:

requires qubits >= n
requires capability("quantum.measurement")
requires capability("quantum.mid_circuit_measurement")
requires topology(...)

These are semantic requirements.

They are not compiler maximums.

The AST MUST NOT contain:

MAX_QUBITS
MAX_PHYSICAL_QUBITS
MAX_QPU_SIZE

as universal language limits.

---

20. Classical AST Contract

Classical computation uses the same AST foundations.

The AST must support:

- scalar values;
- integers;
- floating point;
- arbitrary structured numeric expressions;
- arrays;
- vectors;
- matrices;
- tensors;
- functions;
- control flow;
- memory operations;
- concurrency;
- dataflow;
- symbolic operations.

A mathematical algorithm does not automatically require a new AST node.

For example, a library operation such as:

fft(x)

can remain a normal call expression unless the language gives it special semantic guarantees.

---

21. Mathematical Scalability

The AST MUST NOT impose artificial mathematical limits.

For example:

matrix<1024, 1024>

may be a valid program type/value.

But the AST contract MUST NOT define:

maximum matrix dimension = 1024

The number belongs to program semantics.

---

22. Literal Conformance

The current AST stores:

Integer(i64, Span)
Float(f64, Span)

This is a production scalability concern.

Lexical recognition and source preservation MUST NOT be constrained by "i64" or "f64".

The production architecture must distinguish:

source literal

from:

typed machine representation

Therefore the long-term canonical AST representation for numeric literals MUST preserve the exact source value sufficiently for semantic conversion.

A production-safe direction is:

IntegerLiteral {
    source or canonical arbitrary-precision representation,
    radix,
    sign,
    span
}

and:

FloatLiteral {
    source or exact canonical representation,
    radix/form,
    exponent information,
    span
}

The AST MUST NOT silently lose information during lexing/parsing.

No fixed-width Rust integer may become an accidental language maximum.

---

23. Float Semantics

The AST MUST NOT silently interpret every source floating-point literal as an "f64".

The source language may eventually support:

f16
f32
f64
f128
arbitrary precision
decimal
symbolic
target-specific floating representations

without requiring changes to lexical recognition.

The parser preserves syntax.

Semantic/type analysis chooses representation.

---

24. Character and String Literals

String and character literals must preserve:

- source span;
- escape information where needed;
- Unicode semantics;
- exact source diagnostics;
- semantic decoding state.

A parser MUST NOT silently reinterpret malformed Unicode escapes as valid source.

---

25. Quantum Literals

The AST must support quantum literal forms including:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

without restricting the number of states, amplitudes, dimensions, or qubits through AST-level constants.

The existing:

Literal::Quantum(String, Span)

can serve as a transitional representation only if semantic validation receives enough preserved information.

A future structured representation may be introduced without changing the canonical source file layout.

---

26. Nano Literals

The existing:

Literal::Nano(String, Span)

must follow the same rule.

The AST represents syntax.

It must not embed:

- physical atom limits;
- molecular database limits;
- device limits;
- simulation limits.

Those belong to semantics/runtime/tooling.

---

27. MTS Literals

The existing:

Literal::MTS(String, Span)

must be reconciled with:

grammar/lexer/
grammar/spec/
src/lexer.rs
src/parser.rs

The AST representation must not compensate for an unresolved lexer/specification mismatch.

The canonical pipeline is:

lexical specification
    ↓
lexer
    ↓
parser
    ↓
AST

If a literal is specified but not lexed, that is a conformance failure, not an AST workaround.

---

28. TypeExpr

"TypeExpr" represents source-level type syntax.

It may contain:

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
Quantum
Linear
Affine
Temporal
Pi
Sigma
Identity
Hkt

The exact existing enum may be expanded where required, but type syntax MUST remain distinct from resolved semantic types.

---

29. TypeExpr vs Type

The following separation is mandatory:

TypeExpr
    =
source type syntax

Type
    =
resolved semantic type

Do not collapse them.

For example:

Tensor<f64, shape>

first exists as source syntax.

Semantic analysis determines:

- whether "Tensor" exists;
- whether the generic arguments are valid;
- whether "shape" is valid;
- what capabilities are required;
- what representation is selected.

---

30. Resolved Type Scalability

The current implementation contains:

Array(Box<Type>, Option<usize>)

This must not become a universal scalability ceiling.

"usize" is an implementation representation tied to the compiler process architecture.

It must not silently mean:

«Zamani arrays can never be larger than the host "usize".»

The production semantic model must distinguish:

language-level dimension

from:

host representation used by one compiler pass

If a concrete compiler data structure cannot represent a dimension, it must return a resource/representation diagnostic rather than redefining the language.

---

31. Generic Types

Generic types must be structurally represented.

Examples:

Vector<T>
Matrix<T, R, C>
Tensor<T, Shape>
Memory<T, Size>
Qubit<N>
Result<T, E>

The AST must preserve generic arguments without imposing compiler-defined maxima.

No fixed generic-argument count may be introduced unless it is intrinsic to the language syntax.

---

32. Dependent and Parameterized Types

Existing constructs such as:

Pi
Sigma
Identity
Hkt

must remain syntax-level structures until semantic validation determines whether the construct is valid.

The AST must preserve:

binder
domain
codomain
expression parameters

without evaluating them during parsing.

---

33. Ownership and References

Existing:

Reference(bool, Box<TypeExpr>)
Pointer(bool, Box<TypeExpr>)

represent syntax.

Ownership and aliasing rules belong to semantic analysis.

The parser MUST NOT attempt to prove:

borrow validity
aliasing validity
lifetime validity
resource uniqueness

Those are semantic checks.

---

34. Effects

Effect syntax must be represented structurally.

The AST may contain:

effect declaration
effect invocation
effect handler
effect capability

but must not execute effects.

Effect analysis belongs downstream.

---

35. Capability Requirements

Capability syntax must remain declarative.

Example:

requires capability("tensor.compute")

must become structured AST data.

It must not cause the parser to inspect installed hardware.

Hardware capability discovery belongs to compilation/deployment.

---

36. Resource Requirements

The AST may represent:

requires memory >= required_memory
requires qubits >= n
requires capability(...)
requires topology(...)
requires latency <= bound

Resource requirements are program semantics.

They are not compiler constants.

---

37. Resource Requirement vs Realization

These are distinct:

requires qubits >= n

versus:

map q0 -> physical_qubit(17)

The first is portable.

The second is a realization.

The AST must preserve this distinction.

---

38. Hardware AST Contract

Hardware syntax must represent:

- modules;
- ports;
- signals;
- nets;
- registers;
- clocks;
- resets;
- timing intent;
- protocols;
- interfaces;
- state machines;
- pipelines;
- memory intent;
- synthesis intent;
- verification intent;
- co-design intent.

It must not assume:

32-bit register
64 GB RAM
24 GB VRAM
fixed FPGA size
fixed ASIC size
fixed clock
fixed number of cores

unless explicitly written by the programmer as program semantics.

---

39. HDL Widths

A source construct such as:

wire [31:0]

may be syntactically valid when "31:0" is genuinely the programmer's requested width.

It MUST NOT establish:

Zamani universal register width = 32

Parameterized hardware syntax must remain parameterized.

---

40. Hybrid AST Contract

Hybrid programs must be represented using the same AST.

A valid conceptual flow is:

classical computation
        ↓
quantum operation
        ↓
measurement
        ↓
classical decision
        ↓
quantum operation

No second hybrid AST hierarchy should be introduced merely because two domains interact.

---

41. Concurrency

The AST may represent:

async
await
spawn
parallel
actor
channel
task
pipeline

but must not hard-code:

8 threads
16 cores
32 workers
1024 tasks

unless explicitly specified by the source program.

---

42. Distributed Computing

Distributed syntax must preserve:

logical node
logical process
service
message
channel
partition
replication
consistency
placement intent

without forcing physical node identifiers.

The AST must support arbitrarily large logical collections subject only to actual compiler/resource representation constraints.

---

43. AI/ML

AI syntax must use the same AST infrastructure.

Examples:

model
dataset
training
inference
agent
tensor
pipeline

must not require framework-specific AST types for:

PyTorch
TensorFlow
CUDA
ROCm
vendor accelerator

Framework integration belongs to interoperability/backend layers.

---

44. Data and Tensor AST

Tensor syntax must preserve:

element type
shape
layout intent
indexing
operations
constraints
attributes

without imposing a fixed tensor rank.

The AST must not define:

MAX_TENSOR_RANK

as a language constant.

---

45. Networking

Networking AST nodes may represent:

endpoint
protocol
channel
request
response
stream
service
route

without embedding machine-specific socket state.

Runtime endpoints are downstream realizations.

---

46. Security

Security AST nodes may represent:

identity
authorization
policy
capability
secret
signature
hash
key-management intent
zero-knowledge intent
trust

but the AST does not implement cryptography.

Algorithms and providers belong to semantic/interoperability/backend layers.

---

47. Macros

Macro syntax must remain distinguishable from expanded AST.

The required pipeline is:

source
 ↓
parse
 ↓
macro AST
 ↓
macro expansion
 ↓
validated AST
 ↓
semantic analysis

Macro expansion MUST NOT bypass semantic validation.

Generated nodes must preserve provenance where required.

---

48. Metaprogramming

Reflection, quotation, code generation, compile-time execution and type-level computation must not silently mutate the parser's global state.

AST transformations must be:

- deterministic where language semantics require determinism;
- explicit;
- source-trackable;
- bounded by compiler resource policies;
- safe Rust;
- free from target-specific assumptions.

---

49. AST Determinism

For identical source and identical language/compiler configuration:

source
  ↓
AST

must be deterministic.

AST construction must not depend on:

- hash-map randomization;
- thread scheduling;
- hardware topology;
- current time;
- process address space;
- physical device discovery.

If nondeterminism is part of program semantics, it must be represented explicitly.

---

50. AST Ordering

Ordered source constructs must retain source order.

For example:

declaration A
declaration B
declaration C

must not be arbitrarily reordered by AST construction.

Any later canonicalization must occur after the AST boundary and must retain semantic equivalence and source provenance.

---

51. Diagnostics

AST construction must support diagnostics for:

- unexpected syntax;
- malformed literals;
- invalid structural combinations;
- missing required components;
- duplicate structural components;
- invalid delimiters;
- invalid generic syntax;
- malformed quantum operations;
- malformed HDL constructs;
- malformed attributes.

Semantic diagnostics must not be incorrectly emitted by the parser merely because semantic information is unavailable.

---

52. Error Nodes

Error recovery may use internal error/synthetic nodes.

Such nodes MUST:

- be distinguishable from valid source constructs;
- retain source spans;
- not accidentally lower to executable semantics;
- not be accepted as production-valid AST nodes;
- permit continued diagnostic collection where safe.

An error-recovery node is not a language feature.

---

53. AST Serialization

If AST serialization is exposed to tools, IDEs, tests, caches or external consumers, serialization must have:

schema identity
schema version
language version
feature/dialect information where required
source-span representation
node-kind identity

Unknown fields must be handled according to the compatibility policy.

Serialized AST must not include:

- pointers;
- memory addresses;
- backend handles;
- physical device state;
- runtime objects.

---

54. AST Compatibility

AST compatibility has several dimensions.

54.1 Source compatibility

Existing valid source remains valid unless a documented language-breaking change occurs.

54.2 AST schema compatibility

Tooling consuming serialized ASTs must receive an explicit schema/version change when structure changes.

54.3 Semantic compatibility

Equivalent source must retain equivalent semantics unless a documented language change occurs.

54.4 IR compatibility

AST changes must not silently alter canonical IR semantics.

---

55. Versioning

AST changes must follow the repository's compatibility policy.

A change is classified as:

PATCH
MINOR
MAJOR

according to the language/versioning contract.

Examples requiring explicit compatibility treatment:

- removing an AST variant;
- changing literal representation;
- changing span semantics;
- changing generic representation;
- changing operation structure;
- changing declaration structure;
- changing source-to-AST mapping.

---

56. Deprecation

Existing AST constructs must not disappear without a migration path.

A deprecated construct must have:

deprecated status
reason
replacement
version introduced
version deprecated
migration guidance
compatibility behavior
tests

The parser may continue to accept a deprecated construct while emitting a structured diagnostic according to the language policy.

---

57. AST and "grammar.md"

"grammar/grammar.md" must report AST conformance for each accepted grammar feature.

The status model should include:

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
DEPRECATED

A grammar production is not "STABLE" merely because it parses.

---

58. AST and "Zamani-Grammar.md"

"Zamani-Grammar.md" may describe future AST ideas.

Those ideas are not accepted until they have:

specification
 ↓
grammar
 ↓
lexer
 ↓
parser
 ↓
AST
 ↓
semantics
 ↓
IR
 ↓
tests

No AST implementation may be generated solely because a construct appears in "Zamani-Grammar.md".

---

59. AST and "Zamani.g4"

Every accepted grammar production that produces a meaningful semantic construct must have a documented AST mapping.

Conceptually:

grammar rule
    ↓
parser production
    ↓
AST constructor

The mapping must be deterministic.

No grammar rule may produce an undocumented AST shape.

---

60. AST and Lexer

The lexer owns tokenization.

The AST must not reconstruct lexical meaning from raw strings when a canonical token representation already exists.

For example:

&

must have one canonical lexical identity even if semantic context differs.

The AST then records the syntactic operation.

---

61. AST and Parser

The parser owns:

token → structure

The parser constructs AST nodes.

The parser does not own:

type inference
resource availability
hardware discovery
QEC
routing
scheduling
backend selection

---

62. AST and Semantic Analysis

The required boundary is:

AST
 ↓
semantic analysis

Semantic analysis determines:

- name resolution;
- scope;
- types;
- generic constraints;
- ownership;
- effects;
- capabilities;
- resources;
- domain legality;
- portability;
- semantic invariants;
- contract validity.

The AST remains source structure.

---

63. AST and Canonical IR

The AST must not become a second IR.

The required boundary is:

AST
 ↓
semantic model
 ↓
canonical IR

The IR may normalize:

- operations;
- control flow;
- resources;
- types;
- effects;
- domains;
- quantum operations;
- hardware intent.

The AST should remain useful for diagnostics, tooling, source mapping and source-level transformations.

---

64. Quantum IR Boundary

Quantum constructs must eventually enter:

quantum::ir

The AST must not create:

frontend_quantum_ir
parser_quantum_ir
grammar_quantum_ir

as competing canonical representations.

Existing repository modules under:

src/quantum/ir/

remain the canonical quantum IR boundary.

---

65. Quantum IR Identity

The existing architecture identifies canonical quantum identities such as:

quantum::ir::qubit::QubitId
quantum::ir::qubit::PhysicalQubitId

The AST must not duplicate these types.

Source-level qubit identity and physical qubit identity are different abstraction levels.

---

66. Hardware Mapping

The required boundary is:

AST
 ↓
semantic model
 ↓
IR
 ↓
optimization
 ↓
routing
 ↓
scheduling
 ↓
resilience/QEC/ZQN
 ↓
HAL
 ↓
physical target

The AST must stop before physical realization.

---

67. QEC Integration

Quantum error correction is downstream of AST.

The AST may preserve source intent such as:

requires error_correction(...)
requires fault_tolerance(...)
requires reliability(...)

but MUST NOT implement QEC.

QEC modules consume validated semantic/IR information.

The AST must not duplicate "QecLimits".

Resource policies remain owned by the canonical QEC/resource architecture already present in the repository.

---

68. ZQN Integration

ZQN is downstream.

The AST may express source-level fault/noise intent.

The AST does not own:

- physical noise models;
- realized fault models;
- calibration state;
- error probabilities;
- backend fault state.

Those belong to the ZQN/resilience architecture.

---

69. Routing Integration

Routing consumes semantic/IR representations.

The AST must not decide:

physical path
physical qubit placement
network route
hardware interconnect route

unless the source language explicitly defines a target-specific realization construct.

Even then, the construct must remain distinguishable from portable intent.

---

70. Scheduling Integration

The AST may represent:

ordering constraints
latency requirements
timing intent
dependency constraints

but actual scheduling belongs downstream.

No AST rule may require a fixed machine clock.

---

71. HAL Integration

HAL is the boundary where actual target capabilities become known.

The AST must not query HAL.

The required direction is:

source AST
    ↓
semantic requirements
    ↓
IR
    ↓
HAL capability discovery
    ↓
target realization

Never:

parser
 ↓
HAL

---

72. POCO-REAF AST Rule

For POCO-REAF:

same source
     ↓
same language semantics
     ↓
different target

must not require a different source AST solely because the target is:

CPU
GPU
FPGA
ASIC
QPU
simulator
cluster
cloud
future accelerator

Target-specific lowering occurs after semantic validation.

---

73. Target-Specific Dialects

Dialect syntax may introduce target-specific constructs.

Such syntax must explicitly declare:

dialect name
dialect version
capabilities
semantic extensions
AST extensions
IR mapping
compatibility policy

A dialect must not silently modify the meaning of core Zamani constructs.

---

74. AST Extension Rule

New AST nodes should only be added when the construct has semantics that cannot be represented adequately by existing generic structures.

Before adding:

NewAstNode

the implementation must answer:

1. Why is existing syntax insufficient?
2. Why is an existing AST node insufficient?
3. What semantic information is unique?
4. What IR construct consumes it?
5. What diagnostics require it?
6. What tests require it?
7. What compatibility policy applies?

If the answer is merely:

«"This domain has a new keyword"»

then a new AST node is normally not justified.

---

75. Generic Operation Representation

Operations that are semantically extensible should use a structured generic representation where practical.

Required fields may include:

name
namespace
arguments
operands
results
parameters
attributes
modifiers
effects
capabilities
requirements
span

This supports:

classical operation
quantum operation
AI operation
accelerator operation
HDL operation
future operation

without continually changing the parser's closed enum.

---

76. AST Resource Requirements

A resource requirement node should represent intent, for example:

ResourceRequirement {
    resource
    quantity
    constraint
    condition
    span
}

The exact Rust structure is implementation-owned.

The semantic contract is:

requirement ≠ realization

---

77. AST Capability Requirements

A capability requirement should preserve:

capability name
arguments
constraints
source span

Example:

requires capability("quantum.measurement")

The AST does not determine whether the current target supports it.

---

78. AST Preferences

Preferences must remain distinguishable from requirements.

For example:

prefer accelerator("gpu")

must not become equivalent to:

requires capability("gpu.compute")

This distinction is essential for portability.

---

79. AST Constraints

Constraints represent program-level conditions.

Examples:

latency <= bound
memory >= requirement
precision >= required_precision
reliability >= threshold

The AST stores syntax.

Semantic analysis determines whether the constraint is meaningful and valid.

---

80. AST Hints

Hints must not silently change semantics.

For example:

hint locality

may influence optimization.

It must not make an otherwise invalid program valid.

---

81. AST Attributes

Attributes must be:

- structurally represented;
- source-located;
- semantically validated;
- namespace-aware where needed;
- versionable.

Attributes must not become a hidden second programming language.

---

82. AST Namespaces

Identifiers must retain their source spelling and span.

Qualified names must preserve component order.

For example:

vendor.operation

must remain structurally distinguishable from:

operation

without requiring the parser to know what "vendor" means.

---

83. Identifier Scalability

The AST must not impose artificial identifier-length limits as language semantics.

Compiler resource exhaustion may still produce a compilation error.

That is a resource failure, not a language-level semantic maximum.

---

84. Collection Scalability

AST collections such as:

Vec<Statement>
Vec<Expression>
Vec<Parameter>
Vec<Pattern>
Vec<TypeExpr>

are implementation containers.

Their host representation must not be documented as language-level maximums.

If allocation fails or compiler budgets are exceeded, the compiler may report resource exhaustion.

It must not claim:

«Zamani supports only N statements.»

---

85. Deep AST Scalability

Deeply nested source structures may exhaust recursive compiler traversal.

Production implementations should use iterative traversal where recursion can create avoidable stack exhaustion.

Examples:

nested expressions
nested types
nested blocks
nested generic types
nested patterns
nested macro expansions

must have explicit stress tests.

---

86. AST Traversal Contract

AST visitors/walkers must provide deterministic traversal.

Traversal order must be documented.

A canonical order is generally:

parent
then children in source order

unless a visitor contract explicitly states otherwise.

Traversal must not depend on hash-map iteration order.

---

87. AST Transformation Contract

AST transformations must preserve:

- semantics unless explicitly documented otherwise;
- source provenance where possible;
- spans;
- node relationships;
- deterministic ordering.

Transformations must not silently introduce target-specific information into portable AST.

---

88. Desugaring

Desugaring should occur after parsing and before the semantic/IR boundary where appropriate.

Example:

syntactic sugar
    ↓
normalized AST
    ↓
semantic analysis

Desugaring must not erase source provenance required for diagnostics.

---

89. AST Normalization

Normalization may canonicalize structurally equivalent forms.

It must not:

- alter observable semantics;
- introduce hardware assumptions;
- assign physical resources;
- silently specialize to the current machine.

---

90. Generic Type Parameters

"TypeParameter" currently contains:

name
bounds

This is a valid foundation.

Future additions such as:

capability bounds
resource bounds
effect bounds
lifetime constraints
shape constraints
quantum constraints

must be represented structurally rather than encoded as arbitrary strings where semantic structure is required.

---

91. Patterns

Existing patterns include:

Wildcard
Identifier
Literal
Tuple
Struct
Enum
Or
Range
Ref

Pattern matching must remain source-level.

Exhaustiveness belongs to semantic analysis.

The AST must preserve all pattern structure necessary for semantic checking.

---

92. Match Cases

A "MatchCase" currently contains:

pattern
guard
body
span

This remains a valid foundation.

The AST must preserve:

pattern
optional guard
body
source span

without performing exhaustiveness analysis during parsing.

---

93. Function Parameters

Existing "Parameter" information includes:

name
type
default
is_self
is_mutable

Future additions must remain semantic attributes rather than target-specific ABI decisions.

For example:

calling convention
register
physical memory bank

must not become ordinary parameter properties unless explicitly defined by a target-specific dialect.

---

94. Calls

Call expressions must preserve:

callee
arguments
span

Named arguments, generic arguments and other extensions should be structurally represented when supported.

The AST must not resolve overloads during parsing.

---

95. Member Access

Member access must remain structural:

object.member

Semantic analysis resolves:

field
method
property
module
namespace
associated item

The parser must not need symbol-table knowledge.

---

96. Method Calls

Method-call syntax must preserve:

receiver
method name
arguments
span

Dispatch resolution belongs to semantic analysis.

---

97. Assignment

Assignment AST nodes must preserve:

target
value
operator
span

Semantic analysis determines:

- mutability;
- ownership;
- type compatibility;
- resource effects;
- side effects.

---

98. Async/Spawn

Existing:

Await
Async
Spawn

are source constructs.

The AST does not determine:

thread
CPU core
GPU stream
distributed worker
hardware queue

Those are downstream realization decisions.

---

99. New/Object Construction

Existing:

New

represents source construction.

It does not guarantee a particular allocation strategy.

Allocation strategy belongs to semantic/runtime lowering.

---

100. Macro AST

Existing:

Macro

must carry enough structure to distinguish:

macro invocation
macro name
arguments
span

Macro expansion must occur in a controlled compiler phase.

A macro cannot bypass AST validation.

---

101. AST Safety

The Rust AST implementation MUST use safe Rust.

Production AST code MUST NOT use:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

The AST implementation must rely on safe:

struct
enum
Vec
Box
String
Option
Result
Arc
Rc
HashMap
BTreeMap

or other safe standard/library abstractions as appropriate.

---

102. No Unsafe Dependency Through AST

A dependency that internally uses unsafe Rust is not automatically equivalent to source-level unsafe in Zamani.

However, the Zamani compiler itself must not require an unsafe block or unsafe API for ordinary AST operation.

Dependency review remains part of production supply-chain validation.

---

103. Thread Safety

AST structures should be safe to pass between compiler phases where ownership permits.

No global mutable AST state is permitted.

Parallel compilation must not change AST semantics.

---

104. No Global AST State

The AST must not depend on:

global mutable symbol tables
global current target
global current QPU
global hardware inventory
global compiler phase
global current source file

All required contextual information must be supplied explicitly to the appropriate phase.

---

105. AST Context

When semantic context is required, use explicit context objects outside the AST.

Conceptually:

AST
 +
SemanticContext
 =
semantic validation

not:

AST contains global compiler state

---

106. Source File Identity

AST nodes that span multiple files must retain sufficient source identity to distinguish:

main.zm
module.zm
dependency.zm
generated.zm

Source identity belongs to the source-map layer.

The AST references it through canonical spans.

---

107. Imports and Modules

"UsePath" and related module constructs must remain source structures.

Existing:

UseKind::Single
UseKind::Glob
UseKind::Named

must be resolved semantically.

The AST does not load modules.

---

108. Module Resolution

Module resolution occurs after parsing.

Therefore:

use foo::bar

may produce an AST even when "foo" does not exist.

The semantic phase reports the unresolved module.

---

109. AST and Interoperability

FFI constructs must preserve:

foreign name
foreign type syntax
calling convention syntax
ABI declaration
attributes
span

without making the AST dependent on one foreign language.

---

110. OpenQASM Integration

OpenQASM is an interoperability format.

It must map into Zamani semantic quantum constructs.

It must not become the canonical AST.

The preferred architecture remains:

OpenQASM
   ↓
format frontend
   ↓
Zamani semantic model / quantum::ir

rather than:

OpenQASM AST
+
Zamani quantum AST
+
another quantum IR

as competing representations.

---

111. QIR Integration

Likewise:

QIR

is an interoperability/backend representation.

It must not become the canonical source AST.

---

112. HDL Interoperability

Verilog/SystemVerilog/VHDL/other HDL formats are interoperability targets or frontends.

They must map through explicit conversion contracts.

They must not redefine the core Zamani AST.

---

113. Canonical AST vs Backend AST

A backend may internally construct a target-specific intermediate representation.

That representation must not be confused with the source AST.

The boundary is:

Zamani AST
    ↓
semantic model
    ↓
canonical IR
    ↓
backend IR

---

114. AST Feature Completeness

A feature is not AST-complete merely because an enum variant exists.

A feature is AST-complete only when all are defined:

[ ] source syntax
[ ] lexical tokens
[ ] parser production
[ ] AST representation
[ ] source spans
[ ] AST invariants
[ ] semantic mapping
[ ] capability/resource mapping
[ ] effects mapping
[ ] canonical IR mapping
[ ] diagnostics
[ ] positive tests
[ ] negative tests
[ ] boundary tests
[ ] scalability tests
[ ] compatibility tests
[ ] deterministic traversal
[ ] hard-coding audit
[ ] documentation

---

115. AST Conformance Matrix

Every production feature should be represented in a matrix equivalent to:

Feature| Grammar| Lexer| Parser| AST| Semantic| IR| Tests| Status
identifiers| ✓| ✓| ✓| ✓| ✓| N/A| required| verify
literals| ✓| ✓| ✓| ✓| required| required| required| verify
generics| ✓| ✓| ✓| ✓| required| required| required| verify
functions| ✓| ✓| ✓| ✓| required| required| required| verify
effects| ✓| ✓| ✓| required| required| required| required| verify
classical operations| ✓| ✓| ✓| ✓| required| required| required| verify
quantum operations| ✓| ✓| ✓| required| required| "quantum::ir"| required| verify
hybrid operations| ✓| ✓| ✓| required| required| required| required| verify
HDL| ✓| ✓| ✓| required| required| hardware/IR| required| verify
resources| ✓| ✓| ✓| required| required| required| required| verify
capabilities| ✓| ✓| ✓| required| required| required| required| verify
distributed| ✓| ✓| ✓| required| required| required| required| verify
AI| ✓| ✓| ✓| required| required| required| required| verify
interoperability| ✓| ✓| ✓| required| required| required| required| verify

The exact status must be generated from repository evidence rather than manually claiming completion.

---

116. Current Known AST Conformance Risks

The current implementation must specifically resolve or formally document these areas.

116.1 Fixed-width integer literal

Current:

Integer(i64, Span)

Risk:

large source integers cannot be represented without loss.

Required resolution:

preserve arbitrary source magnitude through AST/semantic conversion.

---

116.2 Fixed-width floating literal

Current:

Float(f64, Span)

Risk:

source precision/form may be lost.

Required resolution:

preserve exact literal information until semantic typing.

---

116.3 Host-sized array dimensions

Current:

Array(Box<Type>, Option<usize>)

Risk:

host representation can accidentally become a language limit.

Required resolution:

separate language-level dimensions from implementation representation.

---

116.4 Domain-specific expression variants

Current variants include:

QuantumOp
NanoOp
Recall
Remember
Learn
Perform
Zamani
Sasa
Macro

Risk:

domain syntax can become a collection of parallel semantic models.

Required resolution:

each variant requires a complete semantic/IR contract, and generic operation structures should be preferred where specialized structure is unnecessary.

---

116.5 Quantum operation closure

A fixed enum of quantum gates must not become the canonical model.

Required resolution:

generic operation identity plus structured operands/parameters.

---

116.6 Physical resource leakage

AST must not acquire physical hardware identity merely because downstream modules use:

PhysicalQubitId

Physical IDs remain downstream realization data.

---

117. AST Hard-Coding Audit

The following are prohibited as universal AST constraints:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_TIMELINES
MAX_AGENTS
MAX_DEVICES
MAX_NETWORK_LINKS
MAX_ACCELERATORS

The audit must search both:

source code
documentation
tests

for accidental universal limits.

---

118. Allowed Constants

The following are allowed when they are actual language/program semantics:

let n = 1024;
array[1024];
matrix<1024, 1024>;
requires qubits >= 1024;
timeout = 1000ms;

The distinction is:

program-defined value

versus:

compiler-defined maximum

Only the latter is prohibited.

---

119. Compiler Resource Limits

A production compiler may require resource budgets for safety.

For example:

maximum compilation memory
maximum diagnostic count
maximum macro expansion work
maximum compilation time
maximum recursion budget

These are implementation/resource policies.

They MUST NOT be represented as language semantic limits.

Diagnostics must identify them as compiler/resource failures.

---

120. Infinity and Practical Representation

"Infinity" in POCO-REAF means:

no artificial language ceiling

It does not mean:

every compiler can physically allocate infinite memory

Therefore:

language capacity

and:

implementation/resource capacity

must remain separate.

---

121. AST Boundary Principle

The AST is the boundary where:

source syntax

becomes:

structured source meaning

It is not the boundary where:

source

becomes:

hardware

---

122. AST-to-Semantic Contract

For every AST node:

AST node
    ↓
semantic interpretation

must be defined.

The semantic layer must know:

- valid contexts;
- required types;
- effects;
- capabilities;
- resources;
- ownership;
- domain restrictions;
- diagnostics;
- IR representation.

---

123. AST-to-IR Contract

For every semantically meaningful AST construct:

AST
 ↓
semantic representation
 ↓
IR

must be documented.

No AST feature may silently disappear during lowering.

If a construct is compile-time-only, the semantic contract must explicitly say so.

---

124. Compile-Time-Only Nodes

Some nodes may disappear after semantic processing.

Examples may include:

macro invocation
type-level computation
compile-time assertion
compile-time metadata
source-only attributes

The disappearance must be intentional and documented.

---

125. Source Provenance

When an AST node lowers to multiple IR operations, each generated IR operation should retain provenance where diagnostics/tooling require it.

Example:

one source operation
       ↓
decomposition
       ↓
many IR operations

All generated operations should remain traceable to the original source construct.

---

126. Quantum Provenance

Quantum decomposition must preserve:

source operation
logical operation
decomposed operation
physical realization

as separate provenance layers.

The AST only owns the first.

---

127. HDL Provenance

HDL lowering may generate:

netlist
logic
pipeline
memory
clocking
verification structures

but source provenance must remain traceable to the Zamani AST.

---

128. Diagnostics Across Lowering

Errors detected downstream must be able to reference source spans.

Examples:

unsupported capability
insufficient target resources
unsupported quantum operation
invalid HDL timing requirement
unrepresentable tensor shape

must be traceable back to the originating AST node.

---

129. AST Testing Strategy

AST tests must be separated into:

lexical
parsing
AST construction
AST shape
AST spans
semantic mapping
IR mapping
negative
boundary
scalability
determinism
compatibility

A parser acceptance test alone is insufficient.

---

130. AST Positive Tests

Positive tests must verify:

source
 ↓
expected AST structure

They should compare structural fields rather than implementation-specific memory addresses.

---

131. AST Negative Tests

Negative tests must verify that invalid syntax does not produce an apparently valid AST.

Examples:

malformed generic
malformed quantum target
malformed attribute
invalid range
invalid parameter list
invalid pattern
invalid HDL declaration

---

132. AST Boundary Tests

Boundary tests include:

empty program
single expression
single declaration
deep nesting
large identifier
large literal
large collection
large generic structure
large quantum operation list
large HDL module

No boundary test may establish an artificial maximum.

---

133. AST Scalability Tests

Scalability tests must test increasing input sizes parametrically.

Conceptually:

N = 1
N = larger
N = much larger

rather than:

maximum supported N = fixed constant

The test harness must distinguish:

passed
resource exhausted
compiler budget exhausted
invalid program

from:

language does not support larger N

---

134. Quantum Scalability Tests

Required cases include:

1 qubit
2 qubits
many qubits
parameterized qubit count
logical qubits
dynamic quantum resources
large operation lists
large parameter lists
custom operations
namespaced operations
measurement
mid-circuit measurement
classical feed-forward

No fixed maximum should appear in the language contract.

---

135. Classical Scalability Tests

Required cases include:

small arrays
large arrays
large tensors
deep expressions
large function bodies
large modules
large dependency graphs
large data pipelines

Compiler resource failure must not be confused with language rejection.

---

136. HDL Scalability Tests

Required cases include:

small module
parameterized module
large module
large signal set
large state machine
large generated structure
large memory declaration
large pipeline

No fixed universal width or resource count may be encoded.

---

137. Distributed Scalability Tests

Required cases include:

1 logical node
multiple logical nodes
large logical node sets
dynamic placement
replication
partitioning
collective operations

No fixed node count may be part of the AST language model.

---

138. Determinism Tests

For identical input:

parse(source)

must produce structurally equivalent ASTs.

Tests must verify:

- node order;
- spans;
- identifiers;
- literal representation;
- generic arguments;
- attributes;
- operation structure.

---

139. Compatibility Tests

Compatibility tests must verify:

old valid source
        ↓
new compiler
        ↓
same AST semantics

where compatibility is promised.

AST changes must have migration tests where required.

---

140. AST Schema Golden Tests

Where serialized AST is supported, maintain golden tests for:

minimal program
classical program
generic program
quantum program
hybrid program
HDL program
resource-constrained program
distributed program

Golden files must include schema/version metadata.

---

141. Test Naming

Recommended names:

ast_minimal
ast_classical
ast_generic
ast_quantum
ast_hybrid
ast_hdl
ast_resources
ast_distributed
ast_large_literal
ast_large_array
ast_deep_expression
ast_custom_quantum_operation
ast_quantum_measurement
ast_ast_determinism
ast_ast_compatibility

The existing repository's tests should be reused rather than duplicated unnecessarily.

---

142. Canonical Example Set

The repository's existing examples such as:

minimal.zm
classical.zm
generic.zm
quantum.zm
hybrid.zm
hdl.zm
poco-reaf.zm

should have AST expectations.

For each:

source
 ↓
lexer
 ↓
parser
 ↓
AST
 ↓
semantic validation
 ↓
IR

must be tested.

---

143. "minimal.zm"

Must prove:

program root
basic declaration
basic expression
source spans
deterministic AST

---

144. "classical.zm"

Must prove:

classical types
functions
expressions
control flow
collections
numeric operations

---

145. "generic.zm"

Must prove:

generic declarations
generic functions
type parameters
bounds
nested generic types

---

146. "quantum.zm"

Must prove:

qubit declaration
quantum operation
parameterized operation
measurement
custom operation
logical resource intent
quantum::ir lowering

It must not depend on a fixed gate enumeration.

---

147. "hybrid.zm"

Must prove:

classical → quantum
quantum → measurement
measurement → classical
classical control → quantum

---

148. "hdl.zm"

Must prove:

hardware module
ports
signals
parameterization
timing intent
state/pipeline structures

without fixed hardware assumptions.

---

149. "poco-reaf.zm"

Must prove that source syntax describes portable intent.

It should include requirements such as:

requires capability(...)
requires memory >= ...
requires qubits >= ...

without selecting a specific physical target.

---

150. AST Review Checklist

A reviewer must verify:

[ ] AST is source-structural
[ ] every accepted syntax has an AST mapping
[ ] every AST node has a documented owner
[ ] every node has source provenance
[ ] spans are preserved
[ ] no hardware state enters AST
[ ] no runtime state enters AST
[ ] no physical resource allocation enters AST
[ ] no artificial language ceilings
[ ] no duplicate quantum IR
[ ] no fixed quantum gate universe
[ ] no target-specific semantic assumptions
[ ] safe Rust only
[ ] deterministic construction
[ ] deterministic traversal
[ ] diagnostics supported
[ ] compatibility documented
[ ] tests exist

---

151. Completion Contract for This File

This file is complete when it defines the contract for:

grammar
lexer
parser
AST
semantic analysis
IR
quantum::ir
hardware
resources
compiler
runtime
compatibility
tests

It does not require this file to be edited whenever an implementation detail changes.

Implementation-specific changes belong in their owning files.

If a new feature changes the architecture, its own feature contract must be updated first and this document only changes if the AST boundary itself changes.

---

152. Completion Contract for an Individual AST Feature

A feature is independently complete when:

Specification
    ✓

Lexical contract
    ✓

Grammar
    ✓

Parser
    ✓

AST
    ✓

Source spans
    ✓

Semantic rules
    ✓

Capabilities
    ✓

Resources
    ✓

Effects
    ✓

Canonical IR mapping
    ✓

Backend/lowering owner
    ✓

Diagnostics
    ✓

Positive tests
    ✓

Negative tests
    ✓

Boundary tests
    ✓

Scalability tests
    ✓

Determinism tests
    ✓

Compatibility tests
    ✓

Hard-coding audit
    ✓

Only then is the feature production-ready.

---

153. Required Integration Changes Outside This File

This document defines the contract; it does not silently modify unrelated files.

The following integration work must be completed in their owning locations.

153.1 "grammar/Zamani.g4"

Ensure every accepted production has an AST mapping.

Do not add another AST-specific grammar hierarchy.

---

153.2 "src/lexer.rs"

Ensure lexical values are preserved without artificial magnitude limits.

Resolve duplicate token concepts according to:

grammar/lexer/tokens.md

---

153.3 "src/parser.rs"

Ensure every parser production creates the canonical AST representation.

Do not create a parallel semantic model inside the parser.

---

153.4 "src/ast/mod.rs"

Bring the current implementation into conformance with this document.

In particular, review:

Literal::Integer(i64, Span)
Literal::Float(f64, Span)
Type::Array(..., Option<usize>)
QuantumOp
NanoOp
Recall
Remember
Learn
Sasa
Macro

against the generic, scalable AST rules defined here.

Do not unnecessarily rename existing public files.

Prefer additive, compatibility-preserving changes where practical.

---

153.5 "src/semantic.rs"

Own:

type resolution
name resolution
effect validation
resource validation
capability validation
ownership validation
domain semantics

It must consume the AST rather than reparse source text.

---

153.6 "src/ir_gen.rs"

Own AST/semantic lowering.

It must not invent a second canonical quantum IR.

Quantum lowering must converge on:

quantum::ir

---

153.7 "src/ir_verify.rs"

Validate canonical IR.

It must not redefine source AST validity.

---

153.8 "src/quantum/ir/"

Remain the canonical quantum IR boundary.

Do not add frontend quantum IR merely to accommodate grammar constructs.

---

153.9 "grammar/grammar.md"

Record implementation status.

Do not mark a feature implemented merely because the parser recognizes it.

---

153.10 "grammar/compatibility/versions.md"

Record AST schema/version implications for compatibility-relevant changes.

---

153.11 "grammar/compatibility/migrations.md"

Provide migrations when an AST-affecting source construct changes incompatibly.

---

153.12 "grammar/compatibility/deprecated.md"

Record deprecated AST/source constructs and their replacements.

---

153.13 "grammar/spec/compatibility.md"

Define language-level compatibility rules consumed by this AST contract.

---

154. Prohibited AST Architectures

The following architectures are prohibited:

grammar
 ↓
AST
 ↓
hardware-specific AST
 ↓
quantum-specific IR
 ↓
another quantum IR

or:

Zamani.g4
 ↓
one AST

Zamani-Grammar.md
 ↓
another AST

src/parser.rs
 ↓
third AST

or:

AST
 ↓
physical hardware

The canonical architecture is:

one language
    ↓
one canonical frontend AST
    ↓
one semantic analysis boundary
    ↓
canonical IRs
    ↓
target realization

---

155. No AST Vendor Lock-In

The AST MUST NOT encode vendor names as universal semantic types merely because a vendor currently exists.

For example, vendor operations may be represented as:

namespace + operation

where appropriate.

Vendor-specific meaning is resolved by interoperability/backend capability contracts.

---

156. Future-Proof AST Rule

The AST must accommodate future computing paradigms without requiring the language core to enumerate every future device.

The design must support:

new operation
new capability
new resource
new accelerator
new computational model
new domain

through structured extension points.

A new machine should not require:

new fundamental AST architecture

unless it introduces genuinely new source semantics.

---

157. "Scale From Atom to Everywhere"

This phrase is interpreted architecturally as:

smallest computation
       ↓
embedded
       ↓
CPU
       ↓
multicore
       ↓
GPU
       ↓
FPGA
       ↓
ASIC
       ↓
QPU
       ↓
accelerator
       ↓
HPC
       ↓
cluster
       ↓
distributed
       ↓
cloud
       ↓
future computational substrate

The AST remains above this target realization hierarchy.

---

158. Resource Availability Rule

A program can be:

AST-valid
semantic-valid

while being:

not currently executable on target X

because target X lacks required resources/capabilities.

This is not an AST failure.

The diagnostic belongs to target compatibility/resource analysis.

---

159. No Silent Downscaling

If a program requires more resources than a target provides, the compiler must not silently change its semantics merely to make it execute.

For example:

requires qubits >= n

must not silently become:

requires qubits >= smaller_n

The compiler may:

decompose
route
distribute
simulate
wait
select another target

only where the semantic contract permits it.

---

160. No Silent Upscaling Assumption

Likewise, a program must not assume that additional hardware automatically changes its observable semantics.

Scaling should improve available execution resources without changing the program's defined meaning.

---

161. Deterministic AST Hashing

If AST hashing is implemented, hashes must be based on semantic/source structure rather than:

memory address
pointer value
hash-map random seed
physical target
runtime state

Hash schema/version changes must be documented.

---

162. Equality

AST equality must distinguish:

structural equality

from:

semantic equivalence

Two ASTs can be semantically equivalent without being structurally identical.

Do not use AST structural equality as a replacement for semantic equivalence checking.

---

163. Clone and Ownership

AST structures must use safe Rust ownership.

Deep cloning must be explicit.

Compiler phases should avoid unnecessary cloning for scalability, while maintaining clear ownership boundaries.

The implementation must not use unsafe pointer manipulation merely to optimize AST traversal.

---

164. Memory Efficiency

AST memory usage should be considered during production validation.

Optimization techniques may include:

- interning;
- compact IDs;
- shared immutable metadata;
- arenas implemented safely;
- source-span compression;
- structural sharing;

provided they preserve the AST contract and do not introduce unsafe Rust.

Such optimizations are implementation details, not language semantics.

---

165. Incremental Compilation

If incremental compilation is supported, AST nodes must have stable enough identity/provenance to support invalidation.

A source edit should invalidate the smallest necessary compilation region.

Incremental caches must not alter AST semantics.

---

166. Parallel AST Processing

AST analysis may be parallelized where dependencies allow.

Parallel processing must produce deterministic results.

The AST itself must not contain synchronization primitives merely because the compiler may process it concurrently.

---

167. IDE Integration

The AST should support:

go to definition
find references
hover
rename
formatting
code completion
diagnostics
semantic highlighting
refactoring

through source spans and structured nodes.

---

168. Formatter Integration

A formatter should operate from the AST or canonical syntax representation.

Formatting must not modify semantics.

The AST must retain enough information for formatting decisions that are required by the language.

---

169. Syntax Highlighting

Syntax highlighting may use lexer tokens.

It must not create a second AST authority.

---

170. Documentation Generation

Documentation tools may consume AST declarations.

Generated documentation must reflect the accepted language, not merely proposals in "Zamani-Grammar.md".

---

171. Security Review

AST processing must be robust against:

malformed input
deep nesting
large literals
large collections
pathological generic structures
pathological macro expansion
ambiguous input
repeated invalid tokens
resource exhaustion

The parser must not panic on ordinary malformed source.

---

172. Panic Policy

Production parsing should return structured errors rather than panic for expected invalid user input.

Panics may only represent programmer invariants that cannot be triggered by ordinary untrusted source, subject to the repository's broader error-handling policy.

---

173. No Unsafe Recovery

Error recovery must use safe Rust.

No unsafe pointer manipulation is permitted merely to recover from malformed syntax.

---

174. Fuzzing

AST conformance must include fuzz testing for:

lexer
parser
AST construction
AST serialization if present
AST traversal

Fuzz inputs must include:

- Unicode;
- malformed literals;
- deeply nested expressions;
- malformed quantum syntax;
- malformed HDL syntax;
- random delimiters;
- very large identifiers;
- very large literals.

---

175. Differential Testing

Where ANTLR and Rust parser implementations coexist, differential tests should compare their accepted/rejected syntax and normalized AST expectations.

The objective is:

ANTLR representation
        ≈
reference Rust parser

for the accepted language.

Neither should silently define a different language.

---

176. AST Conformance CI

Production CI should contain stages equivalent to:

grammar validation
lexer tests
parser tests
AST tests
semantic tests
IR tests
quantum IR tests
compatibility tests
scalability tests
determinism tests
fuzz tests
safe-Rust audit
hard-coding audit

A grammar-only green build is insufficient.

---

177. Safe-Rust CI Audit

CI should reject production compiler changes containing new:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe {

in the compiler/AST implementation unless explicitly exempted by an approved repository-wide policy.

The current requirement is:

NO UNSAFE

---

178. Hard-Coding CI Audit

CI should scan for prohibited universal limit patterns.

The audit should cover:

src/
grammar/
tests/

and detect suspicious additions of:

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

The audit must distinguish legitimate test values and program constants from universal implementation ceilings.

---

179. AST Production Gate

An AST feature may be marked:

PRODUCTION

only if:

grammar
+
lexer
+
parser
+
AST
+
semantic analysis
+
IR
+
tests

are all complete.

Otherwise it must remain:

SPECIFIED
PARTIALLY_IMPLEMENTED
EXPERIMENTAL
PLANNED
DEPRECATED

as appropriate.

---

180. Final Conformance Standard

The Zamani AST is production-conformant when:

✓ One canonical AST exists
✓ Source structure is represented deterministically
✓ Every accepted grammar feature has an AST mapping
✓ Every AST node has a defined owner
✓ Every source node has source provenance
✓ Spans are preserved
✓ Lexer and parser remain separate from semantics
✓ AST remains separate from IR
✓ Quantum syntax lowers to quantum::ir
✓ No duplicate quantum IR exists
✓ Classical/quantum/HDL/hybrid share common AST foundations
✓ Resource requirements remain target-independent
✓ Capabilities remain target-independent
✓ Physical realization remains downstream
✓ No artificial hardware ceilings exist
✓ Numeric literals do not silently lose source magnitude/precision
✓ Host-size representations do not become language limits
✓ Deep programs are tested
✓ Large programs are tested
✓ Quantum scaling is tested
✓ HDL scaling is tested
✓ Distributed scaling is tested
✓ AST construction is deterministic
✓ AST transformations preserve provenance
✓ Compatibility is versioned
✓ Deprecated constructs have migration paths
✓ Diagnostics are source-mappable
✓ Fuzzing exists
✓ Differential testing exists where multiple parser representations exist
✓ Production compiler code uses safe Rust
✓ No unsafe Rust is required
✓ No hidden global compiler state exists in AST
✓ No vendor-specific target model leaks into core AST
✓ POCO-REAF remains intact

---

181. Canonical Production Boundary

The final architecture is:

                         Zamani Source
                              │
                              ▼
                    grammar/Zamani.g4
                              │
                              ▼
                            Lexer
                              │
                              ▼
                           Parser
                              │
                              ▼
                      ┌───────────────┐
                      │ Canonical AST │
                      └───────────────┘
                              │
                              ▼
                    Semantic Analysis
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
       Classical           Quantum              HDL
       semantics          semantics           semantics
          │                   │                   │
          └───────────────────┼───────────────────┘
                              │
                              ▼
                     Canonical Semantic
                         Representation
                              │
                              ▼
                         Canonical IR
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
      Classical          quantum::ir          HDL/
          │                   │              Hardware IR
          └───────────────────┼───────────────────┘
                              │
                              ▼
                         Optimization
                              │
                ┌─────────────┼─────────────┐
                │             │             │
             Routing      Scheduling     Resilience
                │             │             │
                └─────────────┼─────────────┘
                              │
                             ZQN
                              │
                             HAL
                              │
                       Target realization
                              │
          ┌─────────┬─────────┼─────────┬─────────┐
          │         │         │         │         │
         CPU       GPU       FPGA      QPU      Future
          │         │         │         │       targets
          └─────────┴─────────┴─────────┴─────────┘

The AST boundary is therefore intentionally target-independent.

---

182. Final Rule

The production Zamani AST must answer:

«"What did the programmer express?"»

It must not answer:

«"Which machine should execute it?"»

That distinction is what allows one Zamani program to remain valid as the available execution substrate changes from a tiny embedded system to a multicore CPU, GPU, FPGA, ASIC, QPU, simulator, accelerator, HPC system, distributed cluster, cloud environment, or future computational architecture.

The AST therefore provides the stable source-level contract required for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

while leaving:

resource discovery
capability matching
optimization
routing
scheduling
resilience
QEC
ZQN
calibration
HAL
backend lowering
deployment

to the compiler and execution layers that own those responsibilities.