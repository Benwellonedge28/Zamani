Zamani Classical Intrinsics

Path: "grammar/classical/intrinsics.md"
Domain: Classical computation
Language: Zamani
Status: Normative
Version baseline: Rust 1.97 / Rust 1.97.1, Rust 2021
Safety: Rust "unsafe" is forbidden
Primary portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)

---

1. Purpose

This document defines the production contract for classical intrinsics in Zamani.

An intrinsic is a compiler-recognized semantic operation, property, conversion, query, or optimization boundary whose meaning is sufficiently fundamental that the compiler may need to understand it directly rather than treating it only as an opaque ordinary library function.

This document does not define a closed catalogue of every mathematical or computational operation available to Zamani.

In particular, it does not attempt to turn:

- every mathematical function;
- every numerical algorithm;
- every CPU instruction;
- every SIMD instruction;
- every GPU operation;
- every accelerator primitive;
- every vendor library function;
- every BLAS/LAPACK operation;
- every DSP primitive;
- every future computing operation

into a parser-level keyword or permanently closed intrinsic list.

The intrinsic system is an open semantic mechanism.

Its purpose is to provide a stable compiler contract while allowing the language and implementation ecosystem to grow from tiny systems to extremely large systems without making today's hardware limitations part of tomorrow's language.

---

2. Authority

The authority chain for classical intrinsics is:

grammar/specification/
        ↓
grammar/spec/*.md
        ↓
grammar/Zamani.g4
        ↓
frontend AST
        ↓
name / symbol resolution
        ↓
semantic analysis
        ↓
intrinsic resolution
        ↓
canonical semantic IR
        ↓
optimization
        ↓
scheduling / resource analysis
        ↓
target lowering
        ↓
runtime / HAL / backend

The following existing repository surfaces participate in this contract:

grammar/DESIGN.md
grammar/README.md
grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md

grammar/classical/README.md
grammar/spec/classical.md
grammar/spec/syntax.md
grammar/spec/type-system.md
grammar/spec/semantics.md
grammar/spec/resources.md
grammar/spec/compatibility.md

grammar/expressions/
grammar/types/
grammar/functions/
grammar/effects/
grammar/memory/
grammar/concurrency/
grammar/resources/
grammar/hardware/
grammar/compile/
grammar/execution/
grammar/interoperability/

src/lexer.rs
src/parser.rs
src/frontend/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs
src/quantum/ir/

"grammar/Zamani-Grammar.md" may describe proposed or historical intrinsic concepts, but it does not independently authorize syntax.

"grammar/grammar.md" describes implementation conformance and must not become a second intrinsic specification.

"grammar/classical/README.md" defines the classical grammar-domain architecture.

"grammar/spec/classical.md" defines the broader classical semantic contract.

This file defines the narrower intrinsic contract.

---

3. What an intrinsic is

A classical intrinsic is a named semantic operation or semantic primitive with compiler-visible meaning.

Conceptually:

source operation
      ↓
ordinary call / generic operation syntax
      ↓
symbol resolution
      ↓
intrinsic resolution
      ↓
intrinsic contract
      ↓
semantic validation
      ↓
canonical IR

An intrinsic MAY provide:

- well-defined mathematical semantics;
- well-defined type semantics;
- compiler-recognized purity;
- compiler-recognized effects;
- compiler-recognized determinism;
- compiler-recognized resource requirements;
- compiler-recognized capability requirements;
- compiler-recognized optimization opportunities;
- canonical lowering;
- compile-time evaluation;
- runtime evaluation;
- target-specific implementation selection.

An intrinsic MUST NOT require the parser to know the physical implementation.

---

4. Intrinsic versus ordinary function

An ordinary function can be represented semantically as:

name
+
arguments
+
generic arguments
+
return type
+
effects

An intrinsic additionally has compiler-defined semantic identity.

For example, an implementation may recognize a semantic operation corresponding to:

math.sqrt

without requiring:

sqrt

to become a special parser keyword.

The source may use ordinary function/intrinsic call syntax:

math.sqrt(x)

or another syntax defined by the canonical expression grammar.

The distinction is:

ordinary function
    ↓
library/application semantics

intrinsic
    ↓
compiler-recognized semantics

hardware instruction
    ↓
target-specific implementation

These are not interchangeable concepts.

A compiler intrinsic is not automatically a CPU instruction.

---

5. Intrinsic versus standard library

The standard library provides reusable program-level APIs.

An intrinsic provides compiler-recognized semantics.

The same operation MAY have:

Zamani source API
        ↓
standard-library interface
        ↓
intrinsic semantic identity
        ↓
multiple implementations

The intrinsic system must therefore not force users to distinguish implementation mechanisms in source code.

For example, an operation could be implemented using:

software algorithm
CPU instruction
SIMD instruction
GPU kernel
FPGA implementation
ASIC implementation
distributed algorithm
future accelerator

while retaining the same semantic operation.

---

6. Intrinsic versus hardware instruction

This distinction is mandatory.

The following are not equivalent:

intrinsic("vector.add")

and:

CPU instruction ADD

Likewise:

intrinsic("matrix.multiply")

does not mean:

execute one particular processor instruction

The compiler may lower a semantic intrinsic into:

one instruction
many instructions
a vector instruction sequence
a library call
a GPU kernel
an FPGA pipeline
a distributed operation
a software implementation

provided the required semantics are preserved.

---

7. Open-world intrinsic model

The intrinsic namespace MUST be open.

The language MUST NOT require a finite grammar rule such as:

intrinsic
    : SQRT
    | SIN
    | COS
    | EXP
    | LOG
    | FFT
    | GEMM
    | ...
    ;

for every supported classical operation.

Instead, the canonical expression/call model should permit semantic names.

Conceptually:

qualified_name
    ↓
call
    ↓
symbol resolution
    ↓
intrinsic / function / library resolution

This allows:

math.sqrt(x)
math.sin(x)
linear_algebra.matmul(a, b)
signal.fft(signal)
tensor.contract(a, b)
symbolic.differentiate(f, x)

without adding a parser keyword for each operation.

The exact call syntax remains owned by:

grammar/expressions/
grammar/functions/

not by this file.

---

8. Intrinsic identity

Every compiler-recognized intrinsic MUST have a stable semantic identity.

An intrinsic identity should contain, conceptually:

namespace
name
version
semantic signature

An implementation MAY additionally maintain:

feature identifier
domain
effect contract
capability contract
resource contract
determinism contract
lowering family
compatibility information

The semantic identity MUST NOT be derived from a physical implementation.

For example:

math.sqrt

is a suitable semantic identity.

The following are not portable intrinsic identities:

x86.sqrtss
avx512.sqrt
cuda.device_sqrt
gpu0.sqrt
qpu7.operation

Those are target/deployment implementation identities.

---

9. Namespaces

Intrinsic namespaces SHOULD be hierarchical.

Examples include:

math.*
numeric.*
linear_algebra.*
tensor.*
vector.*
matrix.*
signal.*
statistics.*
symbolic.*
complex.*
memory.*
parallel.*
atomic.*
bit.*
conversion.*
reflection.*

The namespace model is extensible.

A future domain may introduce:

future_domain.operation

without modifying the core parser merely because the operation is new.

Vendor-specific namespaces MAY exist as extension/deployment facilities, but they MUST NOT become mandatory assumptions of portable Zamani programs.

---

10. Intrinsic signature

Each intrinsic MUST have a semantic signature.

Conceptually:

IntrinsicSignature {
    identity
    generic_parameters
    parameters
    results
    constraints
    effects
    capabilities
    resource_requirements
    determinism
    evaluation_mode
}

The exact Rust representation belongs to the semantic/compiler implementation.

The grammar MUST NOT duplicate this structure as a parser-specific type system.

---

11. Generic parameters

Intrinsics MAY be generic over:

- types;
- values;
- dimensions;
- shapes;
- precisions;
- policies;
- effects;
- capabilities;
- domains.

Examples:

vector.add<T>(a, b)
matrix.multiply<T, M, N, K>(a, b)
tensor.contract<T, ShapeA, ShapeB>(a, b)

The exact syntax is governed by:

grammar/types/
grammar/functions/
grammar/expressions/

There MUST NOT be an intrinsic-level artificial generic-parameter limit.

---

12. Argument semantics

Intrinsic arguments MUST be semantically typed.

An intrinsic contract may specify:

argument type
argument ownership
argument mutability
shape
rank
value constraints
capability constraints
effect constraints

The compiler must validate those constraints during semantic analysis.

The parser should only establish the structural call.

---

13. Result semantics

An intrinsic MAY produce:

zero results
one result
multiple results
structured results
stream results
resource handles
future values

There MUST NOT be a universal one-result assumption.

An intrinsic result is a semantic value, not a physical register.

For example:

matrix.decompose(A)

could semantically produce a structured result containing multiple matrices without imposing a target-specific representation.

---

14. Side effects

Every intrinsic MUST have an explicit effect classification.

At minimum, the semantic system must be able to distinguish:

pure
read-only
stateful
mutating
I/O
nondeterministic
blocking
concurrent
resource-affecting
external

A pure mathematical operation such as a semantic square root can be classified as pure.

An intrinsic interacting with an external device cannot automatically be classified as pure.

Effect classification belongs to semantic analysis and effect infrastructure, not parser actions.

---

15. Purity

A pure intrinsic:

result = intrinsic(arguments)

must have no observable side effect beyond producing its semantic result.

Pure intrinsics MAY be:

- constant-folded;
- common-subexpression eliminated;
- reordered where legal;
- memoized where permitted;
- vectorized;
- parallelized;
- distributed;
- lowered to specialized hardware.

These transformations are valid only when the semantic contract permits them.

---

16. Determinism

Every intrinsic MUST have a defined determinism contract.

Possible semantic classifications include:

deterministic
conditionally_deterministic
implementation_deterministic
nondeterministic
externally_determined

For numerical operations, deterministic mathematical meaning does not automatically imply identical floating-point bit patterns across all targets.

The implementation must distinguish:

mathematical equivalence
numerical equivalence
bitwise reproducibility
implementation-defined behavior

This is particularly important for:

parallel reductions
floating-point transformations
distributed computation
randomness
hardware accelerators

---

17. Reproducibility

An intrinsic MAY declare reproducibility requirements.

Possible semantic levels include:

none
semantic
numerical
bitwise
environment-bound

The compiler must not silently weaken a requested reproducibility guarantee.

However, a reproducibility requirement is a semantic/compilation contract, not a fixed hardware selection.

---

18. Resource requirements

Intrinsics MAY declare resource requirements.

Examples:

requires memory >= required_memory
requires capability("vector.compute")
requires capability("tensor.compute")
requires capability("parallel.compute")

Resource requirements are evaluated after parsing.

They MUST NOT become parser-level hardware limits.

The intrinsic must never encode:

MAX_MEMORY
MAX_THREADS
MAX_CORES
MAX_GPUS
MAX_VECTOR_LENGTH
MAX_TENSOR_RANK

as universal language restrictions.

---

19. Resource availability

Resource availability is different from intrinsic semantics.

For example:

tensor.multiply(A, B)

has semantic meaning regardless of whether the target has:

CPU
GPU
FPGA
ASIC
accelerator
distributed nodes

The compiler/runtime determines whether the required computation can be realized with available resources.

If resources are insufficient, the result is a:

resource/capability failure

not a grammar failure.

---

20. Capability requirements

An intrinsic MAY require capabilities.

Examples:

requires capability("vector.compute")
requires capability("tensor.compute")
requires capability("parallel.compute")
requires capability("exact.arithmetic")

Capability names are semantic capabilities.

They must not be aliases for fixed hardware counts.

This is valid:

requires capability("tensor.compute")

This is not a universal intrinsic contract:

requires 8 GPU cores

unless the programmer has explicitly expressed that as a program-specific resource requirement rather than an intrinsic definition.

---

21. Preferences

An intrinsic or program MAY express implementation preferences separately from requirements.

Conceptually:

requires capability(...)
prefer capability(...)
allow fallback(...)

A preference MUST NOT be treated as a hard semantic dependency.

For example:

prefer accelerator("tensor")

may allow the compiler to select:

GPU
CPU vector engine
FPGA
ASIC
future accelerator

depending on capabilities and optimization policy.

---

22. Intrinsic lowering

Every stable intrinsic MUST define its canonical semantic lowering.

The conceptual pipeline is:

source call
    ↓
AST Call
    ↓
resolved intrinsic
    ↓
semantic intrinsic operation
    ↓
canonical semantic IR
    ↓
optimization
    ↓
target lowering

The intrinsic specification MUST NOT require the parser to emit target-specific IR.

---

23. No duplicate IR

"grammar/classical/intrinsics.md" MUST NOT introduce a second classical IR.

The intrinsic layer must use the repository's existing canonical semantic/IR infrastructure.

For quantum-classical interaction, the existing canonical "quantum::ir" boundary remains authoritative for quantum semantics.

Classical intrinsics that participate in hybrid operations must not create another:

classical quantum IR

or:

frontend quantum IR

just for intrinsic calls.

---

24. Existing call AST integration

The repository already contains a canonical call-expression AST boundary under:

src/frontend/ast/node/expressions/call.rs

Therefore intrinsic calls SHOULD be represented structurally as ordinary calls.

The AST should retain information necessary for:

callee
generic arguments
ordered arguments
source span

Intrinsic identity is resolved semantically.

The parser MUST NOT execute or resolve an intrinsic.

Parsing:

math.sqrt(x)

must only construct source structure.

It must never invoke "sqrt".

---

25. Call grammar integration

The canonical call grammar under:

grammar/expressions/calls.g4

owns call syntax.

This file MUST NOT redefine call syntax.

Therefore:

intrinsic_call

should not become a second parser-specific calling convention unless the language specification explicitly introduces syntax that genuinely differs from ordinary calls.

The preferred model is:

ordinary call syntax
        ↓
name resolution
        ↓
function
library
intrinsic
dialect operation
external symbol

---

26. Lexer integration

The intrinsic system MUST NOT require every intrinsic name to become a lexer keyword.

For example, these should normally remain identifiers/qualified names:

math.sqrt
math.sin
tensor.contract
linear_algebra.matmul
signal.fft

Adding an intrinsic must therefore normally require:

intrinsic registry/semantic definition

rather than:

new lexer token
new parser keyword
new expression rule

This is essential for extensibility.

It also prevents the keyword namespace from becoming an unbounded catalogue of libraries and algorithms.

---

27. Classical scalar intrinsics

Scalar intrinsics MAY cover semantic operations such as:

absolute value
minimum
maximum
sign
square root
exponential
logarithm
trigonometric functions
hyperbolic functions
rounding
classification
conversion
complex construction

These operations should normally be represented through typed semantic operations rather than dedicated grammar productions.

The intrinsic contract must define:

domain
codomain
numeric requirements
exceptional values
rounding behavior
effects
determinism
reproducibility

---

28. Integer intrinsics

Integer intrinsics MAY provide:

checked arithmetic
wrapping arithmetic
saturating arithmetic
overflow detection
bit counting
population count
leading/trailing zero count
bit extraction
bit insertion
rotation
arithmetic shifts
logical shifts
endianness conversion
integer conversion

The semantic model must distinguish operations from their target implementation.

For example:

bit.rotate_left(x, n)

does not mean:

emit a particular CPU rotate instruction

The compiler may select any correct implementation.

---

29. Arbitrary integer widths

Intrinsic semantics MUST NOT assume:

i32
i64
u32
u64

as universal representations.

Explicitly typed widths are language semantics when the type system defines them.

The target representation is a lowering concern.

An intrinsic must operate according to the declared source type semantics.

---

30. Floating-point intrinsics

Floating-point intrinsic contracts SHOULD explicitly define:

precision
rounding
exceptional values
NaN behavior
infinity behavior
signed-zero behavior
overflow behavior
underflow behavior
reproducibility

An implementation must not silently substitute a different semantic model merely because a target has different hardware floating-point behavior.

---

31. Exact numerical intrinsics

Exact numerical operations MAY include:

rational arithmetic
decimal arithmetic
fixed-point arithmetic
arbitrary-precision arithmetic
symbolic arithmetic
exact comparison
exact conversion

The implementation must preserve exactness where the intrinsic contract promises it.

A compiler may use a different physical representation provided observable semantics remain equivalent.

---

32. Complex-number intrinsics

Complex intrinsics MAY include:

real
imaginary
conjugate
magnitude
phase
complex construction
complex exponential
complex logarithm

The semantic value is:

(real, imaginary)

The physical representation may be:

scalar pair
vector
SIMD value
register pair
memory object
accelerator value
distributed value

The source language must not depend on that representation unless explicitly requested through a target/deployment contract.

---

33. Vector intrinsics

Vector intrinsics MAY include:

map
zip
reduce
scan
broadcast
shuffle
permute
gather
scatter
elementwise arithmetic
elementwise comparison
selection
vector conversion

Vector length is semantic data.

There is no universal:

MAX_VECTOR_LENGTH

and no universal SIMD width.

A vector of length "n" may be lowered using:

scalar execution
SIMD
GPU execution
FPGA pipeline
distributed execution

depending on available resources.

---

34. Matrix intrinsics

Matrix intrinsics MAY include:

transpose
multiply
solve
decompose
factor
inverse
determinant
norm
trace
reshape
broadcast
contraction

The intrinsic contract defines mathematical/semantic behavior.

It does not select:

BLAS
LAPACK
MKL
OpenBLAS
CUDA
ROCm
vendor accelerator

The backend may choose any valid implementation.

---

35. Tensor intrinsics

Tensor intrinsics MAY include:

reshape
transpose
permute
broadcast
slice
concatenate
contract
reduce
map
elementwise operations
convolution
reduction
normalization

Tensor rank MUST remain open-ended.

There is no intrinsic-level:

MAX_TENSOR_RANK

Tensor shape is semantic information.

Storage layout and execution layout are separate.

---

36. Shape-aware intrinsics

An intrinsic MAY declare shape constraints.

Examples:

A.cols == B.rows

or:

result.shape == ...

Shape constraints belong to semantic analysis.

The grammar should parse the shape expression.

The type/semantic system determines whether the constraint is satisfied.

If a shape cannot be proven statically, runtime validation MAY be required.

---

37. Symbolic intrinsics

Symbolic intrinsic operations MAY include:

simplify
differentiate
integrate
substitute
expand
factor
solve
evaluate

These are semantic operations.

The intrinsic system must not hard-code one computer algebra system.

A symbolic intrinsic may be implemented by:

compiler
standard library
symbolic engine
specialized backend
distributed service
future implementation

provided the declared semantics are preserved.

---

38. Numerical-analysis intrinsics

Numerical algorithms MAY be exposed through intrinsic semantic identities when compiler recognition provides meaningful optimization or semantic guarantees.

Examples:

numeric.integrate
numeric.solve
numeric.interpolate
numeric.fft
numeric.convolve
numeric.optimize

However, an operation SHOULD remain an ordinary library function when compiler recognition adds no stable language-level semantic value.

The guiding rule is:

«Do not make the grammar a catalogue of algorithms.»

---

39. Signal-processing intrinsics

Signal-processing operations MAY include semantic identities for:

FFT
IFFT
convolution
correlation
filtering
resampling
windowing
spectral transforms

The intrinsic must describe the operation.

It must not prescribe:

DSP model
SIMD width
sample-buffer maximum
fixed accelerator
specific vendor library

---

40. Statistical intrinsics

Statistical intrinsic semantics MAY include:

mean
variance
standard deviation
covariance
correlation
quantile
distribution functions
sampling
aggregation

Random sampling requires an explicit randomness/effect contract.

A random intrinsic must not be treated as pure unless its semantic model explicitly makes it deterministic through an explicit seed/state model.

---

41. Randomness

Randomness MUST be modeled explicitly.

The intrinsic contract should distinguish:

true external randomness
pseudo-randomness
deterministic seeded randomness
cryptographic randomness
environment-derived randomness

A random intrinsic must declare its relevant effects and reproducibility properties.

It must not silently depend on:

CPU RNG
GPU RNG
OS RNG
hardware RNG

as a universal semantic definition.

---

42. Bit and byte intrinsics

Bit-level operations MAY include:

and
or
xor
not
shift
rotate
extract
insert
mask
count
reverse
byte-swap

These operations must operate on semantic integer/bit representations.

They must not imply a particular CPU instruction set.

---

43. Memory-related intrinsics

Memory intrinsics MAY express semantic operations such as:

allocation
deallocation
copy
move
load
store
atomic access
fence
prefetch intent

However, ownership, borrowing, lifetimes, allocation policy, and memory regions remain governed by:

grammar/memory/
grammar/spec/type-system.md
grammar/spec/semantics.md
grammar/spec/resources.md

A memory intrinsic MUST NOT expose physical addresses as universal source semantics.

---

44. Atomic intrinsics

Atomic operations MAY include semantic identities such as:

atomic.load
atomic.store
atomic.exchange
atomic.compare_exchange
atomic.fetch_add

The semantic model must define:

ordering
visibility
atomicity
failure behavior

The implementation may use:

CPU atomics
GPU atomics
locks
distributed protocols
hardware primitives
software fallback

provided the contract is preserved.

---

45. Parallel intrinsics

Parallel semantic operations MAY include:

parallel.map
parallel.reduce
parallel.scan
parallel.for
parallel.collect
parallel.partition

The number of workers is never part of the intrinsic's universal semantics unless explicitly expressed as a program requirement.

The compiler/runtime may select any valid worker count.

---

46. Reduction semantics

Reduction intrinsics require special care.

For associative mathematical operations:

reduce(op, values)

parallel evaluation may reorder computation.

For floating-point operations, however, reordering may alter bitwise results.

Therefore an intrinsic MUST declare whether it promises:

mathematical associativity
numerical tolerance
bitwise reproducibility
ordered reduction
implementation-defined reduction

The compiler must not infer stronger guarantees than the intrinsic declares.

---

47. Accelerator intrinsics

Accelerator semantics must remain capability-based.

An intrinsic may require:

capability("parallel.compute")
capability("tensor.compute")
capability("vector.compute")

It must not require:

GPU 0
GPU 1
FPGA 0
accelerator 7

as portable semantic identities.

The target system determines the actual implementation.

---

48. CPU intrinsics

CPU-specific operations MAY exist in target-specific dialects or interoperability layers.

They MUST NOT be confused with portable classical intrinsics.

For example:

x86.add
arm.sve.operation
riscv.vector.operation

are target-specific operations.

A portable semantic operation should instead use a target-independent identity such as:

integer.add
vector.add

The compiler may lower those operations to CPU instructions.

---

49. SIMD semantics

SIMD is an implementation strategy unless explicitly part of the source semantic contract.

The intrinsic system must not assume:

4 lanes
8 lanes
16 lanes
32 lanes

as universal limits.

A vector semantic operation can be lowered to whatever target execution width is available.

---

50. GPU semantics

GPU execution is a capability/target concern.

A classical intrinsic may request:

capability("parallel.compute")

or:

capability("tensor.compute")

without selecting a GPU.

GPU-specific source constructs belong to explicit hardware/accelerator dialects where required.

Portable classical intrinsics must remain target-independent.

---

51. FPGA semantics

FPGA implementation MAY lower an intrinsic to:

pipeline
parallel hardware
streaming hardware
custom accelerator

but the intrinsic's semantic identity must remain independent of FPGA-specific realization.

No universal intrinsic may require:

LUT count
DSP count
BRAM count
fixed clock
fixed bus width
fixed FPGA family

unless those values are explicitly part of a target/deployment contract.

---

52. ASIC semantics

ASIC-specific optimization belongs downstream.

A semantic intrinsic may be implemented by an ASIC-specific circuit, but the intrinsic definition must not require the existence of that circuit.

---

53. Distributed intrinsics

Distributed operations MAY include:

collective
broadcast
reduce
scatter
gather
all_reduce
exchange
replicate
partition

The intrinsic contract must describe semantic behavior.

It must not impose:

MAX_NODES

or a fixed topology.

The distributed runtime determines actual placement.

---

54. Networking-related intrinsics

Networking operations may be represented semantically through:

send
receive
stream
request
response
publish
subscribe

but physical networking remains owned by:

grammar/networking/
grammar/hardware/
grammar/distributed/

An intrinsic must not silently encode:

Ethernet
InfiniBand
TCP
UDP
specific NIC

as its universal meaning.

---

55. Compile-time intrinsics

Compile-time intrinsics MAY provide semantic computation during compilation.

Examples include:

constant evaluation
type reflection
shape evaluation
compile-time arithmetic
schema computation

Compile-time evaluation must remain deterministic unless explicitly specified otherwise.

It must not bypass:

type checking
security validation
resource validation
effect checking

---

56. Runtime intrinsics

Runtime intrinsics MAY represent operations whose values cannot be determined during compilation.

Examples include:

environment query
clock query
resource query
runtime capability query
external state access

Such operations must have explicit effects.

The compiler must not treat runtime state as compile-time constant without a semantic guarantee.

---

57. Reflection intrinsics

Reflection operations MAY expose semantic information about:

types
functions
modules
capabilities
schemas
attributes

Reflection must not automatically expose:

raw memory addresses
private compiler internals
physical device registers
unsafe implementation details

unless an explicitly defined target/deployment facility owns that behavior.

---

58. Intrinsic effects and the effect system

Every intrinsic with nontrivial effects must integrate with:

grammar/effects/
grammar/spec/effects.md
grammar/spec/semantics.md

An intrinsic's effect contract must be available to:

type checking
borrow/ownership analysis
optimization
parallelization
scheduling
determinism analysis
diagnostics

The optimizer MUST NOT assume purity merely because an intrinsic looks mathematical.

---

59. Capability and resource integration

Intrinsic resolution must integrate with:

grammar/resources/
grammar/spec/resources.md
grammar/hardware/
grammar/compile/
grammar/execution/

The semantic pipeline is:

intrinsic
   ↓
required capabilities
   ↓
required resources
   ↓
available target capabilities
   ↓
implementation candidates
   ↓
lowering

This separation is essential for POCO-REAF.

---

60. Portability contract

A portable intrinsic MUST describe:

semantic meaning
type behavior
effect behavior
determinism
resource requirements
capability requirements
error behavior

It MUST NOT require knowledge of:

CPU model
GPU model
QPU model
FPGA model
memory capacity
register count
vector width
cache size
node count
physical address
device ID

unless those are explicitly part of a target-specific deployment contract.

---

61. Fallback implementations

A stable portable intrinsic SHOULD permit multiple implementation strategies.

For example:

intrinsic: matrix.multiply

could have:

scalar fallback
parallel CPU implementation
vectorized implementation
GPU implementation
FPGA implementation
distributed implementation
future accelerator implementation

The semantic intrinsic remains one operation.

This is a fundamental POCO-REAF requirement.

---

62. Intrinsic resolution order

Resolution SHOULD conceptually proceed as:

1. lexical name
2. syntactic call
3. generic argument resolution
4. lexical/module scope resolution
5. local function resolution
6. imported symbol resolution
7. standard-library resolution
8. intrinsic resolution
9. dialect resolution
10. external/interoperability resolution

The exact precedence is owned by the language's symbol-resolution specification.

The important rule is that intrinsic resolution must not be performed by the parser.

---

63. Ambiguous intrinsic resolution

If multiple candidates match, semantic analysis must report an ambiguity rather than arbitrarily selecting one.

Diagnostics should identify:

requested operation
candidate identities
argument types
generic arguments
constraints
failed requirements

The parser must not guess.

---

64. Versioning

Intrinsic identities MUST be version-aware.

A change to:

signature
type behavior
effect behavior
determinism
error behavior
resource requirements

that changes observable semantics requires an appropriate compatibility/versioning decision.

The compatibility contract belongs to:

grammar/compatibility/
grammar/spec/compatibility.md

This file defines the intrinsic-specific requirement.

---

65. Deprecation

An intrinsic MAY be deprecated.

Deprecation must provide:

intrinsic identity
deprecated version
replacement
migration guidance
compatibility period
diagnostic behavior

Deprecation must not silently change semantics.

---

66. Dialect integration

Vendor/domain-specific intrinsic extensions should use:

grammar/dialects/

rather than modifying the universal intrinsic grammar.

A dialect intrinsic must identify:

dialect
version
intrinsic identity
semantic contract
required capability
AST mapping
IR mapping
compatibility

A dialect MUST NOT silently redefine the semantics of a stable core intrinsic.

---

67. Interoperability integration

External functions may be represented through:

grammar/interoperability/

and the repository's existing external-call semantic infrastructure.

An external call is not automatically a core intrinsic.

The distinction is:

core intrinsic
    compiler-defined semantics

library function
    library-defined semantics

external call
    external symbol/ABI semantics

target intrinsic
    target-specific semantics

The semantic model must preserve those distinctions.

---

68. Quantum-classical integration

Classical intrinsics may operate on classical results produced by quantum computation.

For example:

measurement
    ↓
classical value
    ↓
classical intrinsic

A classical intrinsic MUST NOT assume that every classical value originated classically.

Conversely, a classical intrinsic must not create a second quantum representation.

Quantum semantics remain owned by:

quantum::ir

and the quantum subsystem.

---

69. HDL integration

A classical intrinsic may participate in software/hardware co-design.

For example:

algorithm
    ↓
semantic intrinsic
    ↓
hardware-intent analysis
    ↓
HDL/hardware lowering

The intrinsic itself must remain semantic.

HDL implementation belongs to:

grammar/hdl/
grammar/hardware/

---

70. AI integration

AI/ML operations may be represented as semantic operations or library functions.

Examples include:

tensor operations
automatic differentiation
probabilistic operations
model inference
optimization

The intrinsic system must not hard-code:

PyTorch
TensorFlow
JAX
CUDA
ROCm
specific accelerator

as universal semantics.

Framework integration belongs to interoperability/backend layers.

---

71. Numerical precision

Precision is semantic when explicitly selected.

Examples include:

f32
f64
decimal
fixed_point
exact
symbolic

But a semantic operation must not silently depend on the host compiler's preferred representation.

Where precision is generic:

T

the intrinsic contract must define the constraints required of "T".

---

72. Approximation

Some intrinsics MAY permit approximation.

If approximation is permitted, the intrinsic must define the semantic contract, such as:

absolute error bound
relative error bound
ULP bound
probabilistic bound
implementation-defined tolerance

An implementation must not silently turn an exact intrinsic into an approximate one.

---

73. Overflow and exceptional behavior

Each numeric intrinsic must define relevant behavior for:

overflow
underflow
division by zero
invalid values
NaN
infinity
domain errors
loss of precision

Where behavior is type-specific, the intrinsic references the relevant type contract rather than duplicating it.

---

74. Error semantics

An intrinsic may fail.

Failure must be represented through the language's canonical error/result/effect model.

The intrinsic definition must not invent a parallel error system.

Possible semantic outcomes include:

success
error
exception
optional result
result type
effect
resource failure
capability failure

The exact mechanism is owned by the general language semantics.

---

75. Resource failure

If an intrinsic requires resources unavailable on a target:

intrinsic
    ↓
resource requirement
    ↓
resource analysis
    ↓
insufficient resource

the failure must be reported as a resource/capability/deployment issue.

It must not be reported as:

invalid Zamani syntax

unless the source itself is syntactically invalid.

---

76. Resource policies are not intrinsic limits

An implementation MAY provide:

validate_with_limits(...)

or an equivalent policy mechanism.

Such limits are:

caller policy
compiler policy
CI policy
sandbox policy
runtime policy
deployment policy

They are not language semantics.

Therefore:

max_arguments = 1000

can be a compilation policy.

It must never become:

Zamani supports at most 1000 arguments

unless deliberately standardized as a language semantic restriction.

---

77. Infinite-scale requirement

"Infinity" in POCO-REAF means no artificial language ceiling.

It does not mean that a physical computer has infinite resources.

Therefore Zamani intrinsic semantics must support:

tiny input
large input
very large input
distributed input
future-scale input

subject to:

available memory
available compute
available storage
available communication
available capabilities
compiler/runtime policy
execution time
target support

A resource shortage is not a language limitation.

---

78. Complexity neutrality

An intrinsic definition must not assume that an operation will always execute within a particular complexity bound.

For example:

matrix.inverse

has semantic meaning independent of whether the backend uses:

naive algorithm
blocked algorithm
parallel algorithm
distributed algorithm
hardware accelerator

Optimization is downstream.

---

79. Semantic stability

The semantic meaning of an intrinsic must remain stable even when implementation strategy changes.

This permits:

compiler version A
    → CPU implementation

compiler version B
    → GPU implementation

compiler version C
    → distributed implementation

without changing the source-level meaning.

---

80. Compiler optimization contract

The optimizer may transform intrinsic operations when:

types remain valid
effects remain valid
resource constraints remain valid
observable behavior remains valid
determinism contract remains valid
reproducibility contract remains valid
error semantics remain valid

An optimization must not be justified solely because a target has a faster instruction.

---

81. Constant evaluation

A pure intrinsic MAY support compile-time evaluation.

The compiler must ensure:

compile-time result
==
runtime semantic result

subject to the intrinsic's exact semantics.

Compile-time evaluation must not introduce target-dependent behavior.

---

82. Memoization

A pure deterministic intrinsic MAY be memoized.

Memoization is prohibited when observable behavior depends on:

time
randomness
external state
I/O
mutable global state
resource state

unless the semantic contract explicitly permits memoization.

---

83. Vectorization

A compiler MAY transform scalar intrinsic operations into vector operations.

For example:

map(math.sqrt, values)

may become:

vectorized sqrt

if semantic requirements permit.

The intrinsic definition must remain independent of the vector width.

---

84. Parallelization

A compiler MAY parallelize intrinsic applications when:

dependencies permit it
effects permit it
ordering permits it
determinism permits it
resource policy permits it

No fixed worker count may be encoded in the intrinsic.

---

85. Fusion

Compatible intrinsic operations MAY be fused.

For example:

map(f)
then map(g)

may become one fused operation where semantics permit.

Fusion must preserve:

evaluation order where observable
effects
errors
resource semantics
numerical guarantees

---

86. Hardware specialization

The compiler may specialize an intrinsic based on discovered target capabilities.

Example:

tensor.matmul

may select:

CPU implementation
GPU implementation
FPGA implementation
ASIC implementation
distributed implementation

The source intrinsic identity remains unchanged.

---

87. No vendor lock-in

Vendor APIs must be represented below the portable semantic layer.

The architecture is:

portable intrinsic
        ↓
backend capability matching
        ↓
vendor implementation

not:

Zamani language
        ↓
vendor API

as the only implementation.

---

88. Intrinsic registry

The implementation SHOULD maintain a machine-readable intrinsic registry.

The registry is an implementation/semantic artifact, not a parser grammar.

Each entry should define at least:

id
namespace
name
version
domain
signature
effects
determinism
capabilities
resource requirements
error contract
lowering contract
compatibility
status

The registry must not contain machine-specific capacity constants as semantic ceilings.

---

89. Registry ownership

The registry must have one canonical owner.

It must not be independently duplicated in:

Zamani.g4
grammar.md
Zamani-Grammar.md
lexer.rs
parser.rs

Those components consume or validate the authoritative intrinsic metadata as appropriate.

The parser should not maintain a second intrinsic catalogue.

---

90. AST contract

Intrinsic calls should use the existing call-expression AST model.

The AST owns:

source span
callee
generic arguments
arguments

The AST must remain domain-neutral.

It should not require:

IntrinsicSqrtNode
IntrinsicMatrixMultiplyNode
IntrinsicGpuNode
IntrinsicCpuNode

for every operation.

Semantic resolution should classify the call.

---

91. Semantic contract

Semantic analysis owns:

symbol resolution
intrinsic identification
generic instantiation
type checking
shape checking
effect checking
capability checking
resource checking
determinism checking
error-contract checking

This is where a call becomes semantically identified as an intrinsic.

---

92. IR contract

The intrinsic must lower into the repository's canonical semantic/IR representation.

The intrinsic layer MUST NOT create:

ClassicalIntrinsicIR
IntrinsicIR2
VendorIntrinsicIR
FrontendIntrinsicIR

as competing semantic boundaries.

If a new canonical IR operation is genuinely required, that belongs to the IR specification and implementation, not to this grammar document.

---

93. Diagnostics

Intrinsic diagnostics MUST identify the semantic issue.

Examples:

unknown intrinsic
ambiguous intrinsic
invalid argument type
invalid generic argument
shape constraint unsatisfied
capability unavailable
resource requirement unsatisfied
unsupported intrinsic version
deprecated intrinsic
invalid effect context
non-deterministic operation in deterministic context

Diagnostics should contain source spans from the AST.

---

94. Diagnostic stability

Diagnostics must not depend on:

target CPU model
GPU model
memory size
number of cores

except when reporting target/resource availability.

The language-level diagnostic must remain understandable independently of the target.

---

95. Security

Intrinsic resolution must not provide an unrestricted escape hatch.

An intrinsic must not automatically grant:

raw memory access
arbitrary device access
filesystem access
network access
process control
privileged system access

Such operations require explicit capabilities/effects/security policy.

---

96. Safe Rust requirement

The implementation of intrinsic resolution and registry handling MUST use safe Rust.

No:

unsafe { ... }

No unsafe blocks.

No requirement for raw-pointer manipulation.

No intrinsic contract may require unsafe Rust merely because a target backend uses unsafe implementation techniques elsewhere.

Unsafe backend implementation, if ever required by another subsystem, must not leak into the language/grammar contract; for the Zamani compiler implementation itself, this contract requires safe Rust.

---

97. Rust-version compatibility

The implementation baseline is:

Rust 1.97 / Rust 1.97.1
Rust 2021

The intrinsic implementation must avoid depending on APIs unavailable to the selected minimum supported Rust version.

The grammar documentation itself must remain independent of compiler-host-specific Rust features.

---

98. Deterministic registry behavior

Intrinsic lookup must be deterministic.

Given the same:

program
language version
intrinsic registry version
imports
dialects
compiler configuration

resolution must produce the same semantic result.

Registry iteration order must not change semantic resolution.

---

99. Duplicate intrinsic identities

Two active intrinsic definitions must not claim the same canonical identity unless they are explicitly versioned/overloaded according to the language's resolution rules.

Duplicate definitions must produce a deterministic diagnostic.

---

100. Intrinsic overloads

Overloading MAY be supported.

Resolution must consider:

name
generic arguments
argument types
shape constraints
effect context
capability requirements
conversion rules

Ambiguous matches must be diagnosed.

The implementation must not resolve an overload based merely on target hardware.

---

101. Conversion rules

Intrinsic conversions must be explicit according to the type system.

The compiler must not silently convert:

exact → approximate
high precision → low precision
integer → floating point
signed → unsigned

when that conversion changes observable semantics, unless the language specification explicitly permits it.

---

102. Shape conversions

For vector/matrix/tensor intrinsics, conversions such as:

reshape
broadcast
transpose
slice
pad
truncate

must have explicit semantic definitions.

The compiler must not silently reinterpret memory layout as shape conversion.

---

103. Memory layout

Memory layout is separate from intrinsic semantic meaning.

For example:

matrix.multiply(A, B)

does not prescribe:

row-major
column-major
blocked
tiled
strided
distributed
device-local

unless the program explicitly requires a layout.

Layout selection belongs downstream.

---

104. Data locality

An intrinsic MAY expose locality requirements when locality itself is part of semantic or performance intent.

For example:

prefer locality(...)

is different from:

must use memory bank 3

The latter belongs to explicit target/deployment descriptions.

---

105. Performance hints

Performance hints MAY exist, but they must remain distinct from semantic requirements.

Examples:

prefer vectorization
prefer parallel
prefer locality
prefer accelerator

must not change program meaning.

Hints may be ignored if the target cannot satisfy them.

---

106. Requirement versus hint

The following distinction is mandatory:

requires capability(...)

means the program cannot satisfy its contract without the capability.

prefer capability(...)

means the compiler should consider the capability when available.

hint ...

provides optimization guidance.

These must never be conflated.

---

107. Intrinsic categories

The intrinsic registry MAY classify operations into semantic categories such as:

numeric
integer
floating
exact
complex
vector
matrix
tensor
symbolic
statistics
signal
memory
atomic
parallel
distributed
reflection
conversion
resource
environment

Categories are metadata.

They are not necessarily parser keywords.

---

108. Classical intrinsic families

The initial production families should cover, where required by the language specification:

scalar
integer
floating
exact
complex
vector
matrix
tensor
numeric
symbolic
statistics
signal
bit
memory
atomic
parallel
distributed
conversion
reflection
resource

This is an extensible taxonomy, not a finite list of all future operations.

---

109. Do not duplicate mathematical syntax

Existing classical grammar components such as:

scalar.g4
vector.g4
matrix.g4
tensor.g4
numerical.g4
symbolic.g4

must not become catalogues of intrinsic function names.

Their purpose is genuine language-level syntax.

Intrinsic names remain semantic identifiers.

---

110. Integration with "grammar/classical/README.md"

The existing classical README states that ordinary function/intrinsic syntax should be preferred where dedicated grammar syntax is unnecessary.

This file completes that contract.

Therefore:

classical grammar
    ↓
structural source syntax

intrinsic system
    ↓
semantic operation identity

The two layers must not be merged.

---

111. Integration with "grammar/spec/classical.md"

The classical specification establishes that:

- classical computation is one domain of the Zamani language;
- arbitrary-width semantic computation must be possible;
- hardware representation is separate from source semantics;
- mathematical algorithms should not all become parser keywords;
- intrinsic semantics may be compiler-recognized;
- intrinsic contracts require semantic integration.

This document specializes those rules for the intrinsic layer.

No rule here may contradict the normative classical specification.

---

112. Integration with "grammar/spec/type-system.md"

Intrinsic argument and result typing must use the canonical type system.

This file must not introduce:

IntrinsicType

as a competing type system.

Intrinsic constraints are expressed through the canonical type/constraint model.

---

113. Integration with "grammar/spec/semantics.md"

Intrinsic semantics are evaluated according to the general evaluation model.

An intrinsic cannot bypass:

scope
name resolution
type checking
effect checking
ownership rules
resource rules

unless an explicit language-level rule authorizes such behavior.

---

114. Integration with "grammar/spec/resources.md"

Intrinsic resource requirements are declarative.

They may describe:

memory
compute
communication
capability
storage
precision
accelerator support

but not arbitrary physical device assignments.

---

115. Integration with "grammar/compile/"

Compilation may use intrinsic metadata for:

constant folding
specialization
vectorization
parallelization
target selection
capability matching
lowering

The compile subsystem owns implementation strategy.

---

116. Integration with "grammar/execution/"

Execution may use intrinsic metadata for:

scheduling
placement
resource acquisition
runtime dispatch
fallback
recovery
observability

The intrinsic itself does not perform runtime scheduling.

---

117. Integration with "grammar/hardware/"

Hardware capability discovery belongs to the hardware subsystem.

Intrinsic metadata can request capabilities.

Hardware discovery determines whether those capabilities exist.

---

118. Integration with "grammar/concurrency/"

Parallel and atomic intrinsics must use the canonical concurrency semantics.

An intrinsic must not invent its own synchronization model.

---

119. Integration with "grammar/memory/"

Memory-related intrinsics must respect:

ownership
borrowing
lifetime
allocation
region
address-space
sharing

The intrinsic system must not bypass memory safety.

---

120. Integration with "grammar/interoperability/"

External/vendor implementations may be selected through interoperability mechanisms.

The semantic intrinsic remains the portable abstraction.

---

121. Integration with quantum

Classical intrinsic semantics may consume quantum results.

For example:

measurement
    ↓
classical value
    ↓
math operation

The classical intrinsic system must not own:

qubit allocation
gate decomposition
routing
QEC
calibration
pulse scheduling
ZQN

Those remain quantum/backend responsibilities.

---

122. Integration with HDL

Classical operations may become hardware implementation candidates.

For example:

matrix.multiply

could lower to an HDL-described accelerator.

The intrinsic contract remains independent of that implementation.

---

123. Integration with future domains

A future computing domain must be able to consume existing classical intrinsics without requiring the language to redefine them.

For example, a future accelerator may implement:

tensor.contract

without changing the source semantics.

This is a core POCO-REAF property.

---

124. No fixed hardware mapping

The following MUST NOT appear as semantic intrinsic identities:

cpu0.add
cpu1.add
gpu0.matmul
gpu1.matmul
fpga0.fft
qpu0.classical_operation
node0.reduce

Such identifiers may exist in deployment/target metadata.

They are not portable classical intrinsic identities.

---

125. No artificial cardinality limits

The intrinsic system MUST NOT impose language-level limits on:

arguments
results
generic parameters
vector length
matrix dimensions
tensor rank
threads
workers
devices
nodes
memory
accelerators

Any implementation policy limit must be explicitly represented as a policy.

---

126. Explicit policy boundaries

Compiler APIs MAY expose configurable limits.

Conceptually:

CompilationPolicy {
    resource_limits
    diagnostic_limits
    time_budget
    memory_budget
}

These are external compilation policies.

They must never be silently interpreted as language semantics.

---

127. Testing requirements

Every stable intrinsic family requires:

positive tests
negative tests
boundary tests
type tests
effect tests
determinism tests
resource tests
capability tests
portability tests
compatibility tests

Tests must verify semantic behavior rather than only parser acceptance.

---

128. Scalability tests

Scalability tests must verify that intrinsic semantics do not establish artificial ceilings.

Examples should exercise:

small vectors
large vectors
symbolic vectors

small matrices
large matrices
symbolic matrices

low-rank tensors
higher-rank tensors
symbolic-rank representations where supported

small argument sets
large argument sets

single-worker execution
multi-worker execution
distributed execution

The tests must not define the largest supported value as a language limit.

---

129. Determinism tests

Where an intrinsic is declared deterministic, tests must verify that semantic results remain deterministic across:

different execution orders
different worker counts
different supported target strategies

where the declared semantic contract requires such invariance.

---

130. Reproducibility tests

For intrinsics with reproducibility guarantees, tests must distinguish:

semantic equality
numerical equality
bitwise equality

The test must assert only the guarantee actually promised by the intrinsic.

---

131. Resource tests

Resource tests must distinguish:

semantic validity
resource availability
resource policy

A test must never encode:

Zamani supports exactly N resources

unless that number is explicitly a target-policy test.

---

132. Negative tests

Negative tests must cover:

unknown intrinsic
invalid namespace
wrong argument count
wrong argument type
invalid generic argument
invalid shape
invalid capability
invalid effect context
invalid conversion
ambiguous overload
unsupported version

---

133. Compatibility tests

Compatibility tests must verify that:

old source
+
supported intrinsic version

continues to have the same defined meaning.

Deprecated intrinsic behavior must be explicit.

---

134. Documentation generation

Intrinsic documentation SHOULD be generated from the same semantic registry used by implementation tooling.

This avoids:

documentation says X
compiler implements Y

drift.

Generated documentation may feed:

grammar/grammar.md
reference documentation
IDE/LSP metadata
diagnostics
compiler help

but those generated surfaces remain non-authoritative.

---

135. Conformance matrix

Each intrinsic should be traceable through:

Intrinsic ID
      ↓
Specification
      ↓
Grammar call syntax
      ↓
AST call node
      ↓
Semantic resolver
      ↓
Type/effect/resource validation
      ↓
Canonical IR
      ↓
Optimization
      ↓
Backend/lowering
      ↓
Runtime
      ↓
Tests

A production intrinsic is incomplete if one of these required semantic boundaries is undefined.

---

136. Intrinsic completion contract

An intrinsic family is considered production-ready only when all applicable items are complete:

[ ] stable semantic identity
[ ] namespace defined
[ ] version defined
[ ] signature defined
[ ] generic semantics defined
[ ] argument semantics defined
[ ] result semantics defined
[ ] type constraints defined
[ ] shape constraints defined where applicable
[ ] effect contract defined
[ ] determinism contract defined
[ ] reproducibility contract defined where applicable
[ ] error behavior defined
[ ] capability requirements defined
[ ] resource requirements defined
[ ] portability contract defined
[ ] AST mapping defined
[ ] semantic mapping defined
[ ] canonical IR mapping defined
[ ] optimization rules defined
[ ] target-lowering contract defined
[ ] diagnostics defined
[ ] compatibility defined
[ ] positive tests
[ ] negative tests
[ ] boundary tests
[ ] scalability tests
[ ] determinism tests
[ ] portability tests
[ ] hard-coding audit
[ ] safe-Rust implementation
[ ] no duplicate semantic authority

---

137. Hard-coding audit

Every intrinsic definition MUST pass an explicit hard-coding audit.

The following are prohibited as universal semantic limits:

MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_VECTOR_LENGTH
MAX_MATRIX_ROWS
MAX_MATRIX_COLUMNS
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_REGISTER_WIDTH
MAX_DEVICE_COUNT
MAX_ACCELERATORS

Also prohibited as universal semantic assumptions:

CPU0
GPU0
FPGA0
NODE0
REGISTER0

unless they occur exclusively in target/deployment metadata.

---

138. Machine-width audit

Intrinsic definitions must not silently assume:

32-bit pointer
64-bit pointer
32-bit integer
64-bit integer
fixed SIMD width
fixed register width
fixed cache line
fixed address space

The source type system determines semantic widths.

The target determines representation.

---

139. Memory audit

Intrinsic definitions must not assume:

RAM = fixed size
VRAM = fixed size
cache = fixed size
register file = fixed size
storage = fixed size

Resource requirements are dynamic and capability-based.

---

140. Parallelism audit

Intrinsic definitions must not assume:

one thread
8 threads
16 threads
32 threads
64 threads
one GPU
one accelerator
fixed node count

Parallelism is determined from semantic dependencies and available resources.

---

141. Tensor/vector audit

Intrinsic definitions must not assume:

rank <= N
dimension <= N
vector length <= N
matrix rows <= N
matrix columns <= N

where "N" is an implementation-derived ceiling.

---

142. Compiler-host independence

Intrinsic semantics must not depend on the architecture on which the Zamani compiler itself is running.

For example, compiling Zamani on:

x86
ARM
RISC-V

must not change the semantic meaning of an intrinsic.

---

143. Target independence

The same semantic program may be lowered differently for:

embedded CPU
CPU
multicore CPU
GPU
FPGA
ASIC
DSP
accelerator
cluster
cloud
distributed system
future machine

without rewriting the source algorithm.

---

144. Intrinsic implementation safety

The preferred implementation architecture is:

immutable intrinsic metadata
        ↓
validated lookup
        ↓
semantic resolution
        ↓
typed lowering

Use safe Rust data structures and explicit errors.

Do not use raw pointers, unchecked casts, or unsafe parser actions.

---

145. No execution during parsing

An intrinsic MUST NOT execute during:

lexing
parsing
AST construction

Parsing is structural.

Semantic evaluation occurs later.

This preserves:

determinism
security
tooling compatibility
IDE parsing
incremental compilation

---

146. IDE/LSP integration

Intrinsic metadata SHOULD support tooling for:

completion
signature help
documentation
type information
diagnostics
go-to-definition
deprecation warnings
capability diagnostics

These tools must consume semantic metadata rather than independently maintaining a second intrinsic list.

---

147. Build-system integration

Compilation artifacts should record the intrinsic identities and versions required by a compiled program where reproducibility requires that information.

This supports:

reproducible builds
dependency tracking
compatibility checks
provenance

---

148. Reproducible compilation

A compiler should be able to determine, from the program and compilation environment:

intrinsic registry version
language version
dialect versions
semantic contracts
optimization profile
target capability profile

The same source plus equivalent declared compilation conditions should produce equivalent semantic compilation results.

---

149. Caching

Pure deterministic intrinsic evaluations MAY be cached.

Cache identity must include all semantically relevant inputs.

The cache must not assume a fixed target unless target-specific semantics are explicitly part of the cache key.

---

150. Security and capability boundaries

An intrinsic requiring privileged behavior must declare an explicit capability.

Examples:

capability("system.clock")
capability("network.access")
capability("device.control")
capability("external.compute")

The capability system determines authorization.

The parser does not grant authorization merely because an intrinsic name appears in source.

---

151. External state

Intrinsics reading external state must be classified appropriately.

Examples:

time.now
environment.query
resource.available
device.status

must not be treated as pure constants unless the semantic model explicitly guarantees snapshot semantics.

---

152. Error and recovery

Intrinsic resolution errors must be recoverable at the compiler diagnostic layer where practical.

The parser should continue parsing independent constructs when possible.

Semantic errors should retain source locations.

---

153. Forward compatibility

An older compiler should be able to recognize that an intrinsic is:

known
unknown but namespaced
unsupported version
unsupported capability

without confusing all unknown operations with syntax errors.

This is especially important for future domain expansion.

---

154. Extension rule

A new classical intrinsic should normally require only:

1. semantic specification
2. intrinsic registry entry
3. implementation/lowering
4. tests
5. documentation

It should NOT normally require:

new lexer keyword
new parser token
new root grammar alternative
new AST node
new parallel IR

unless the operation genuinely introduces new language syntax or semantic structure.

---

155. When a new AST node is justified

A dedicated AST node is justified only when the operation introduces source semantics that cannot be represented by the existing generic call/operation model.

Examples might include a future language-level construct with:

special binding semantics
special evaluation semantics
special control-flow semantics
special ownership semantics

A mere mathematical operation is not sufficient justification.

---

156. When a new grammar rule is justified

A dedicated grammar rule is justified only when the operation has genuine syntax that ordinary expression/call syntax cannot represent clearly or safely.

Do not create:

sqrtExpression
fftExpression
matmulExpression
svdExpression

merely because these operations exist.

Prefer semantic resolution of generic operations.

---

157. Intrinsic lifecycle

Every intrinsic SHOULD move through:

proposed
    ↓
experimental
    ↓
specified
    ↓
implemented
    ↓
tested
    ↓
stable
    ↓
deprecated
    ↓
removed

An intrinsic in "Zamani-Grammar.md" is not automatically part of this lifecycle.

Promotion requires the normal specification and implementation process.

---

158. Experimental intrinsics

Experimental intrinsics must declare:

status = experimental

and SHOULD carry:

feature gate
version
compatibility warning

They must not silently become stable semantics.

---

159. Stable intrinsic guarantee

A stable intrinsic guarantees that its documented semantics are part of the supported language contract.

Implementation strategy remains free to evolve.

Therefore:

semantic stability
≠
implementation stability

---

160. Deprecated intrinsic guarantee

Deprecation means the semantic identity remains recognized for the compatibility period.

The compiler should provide a diagnostic pointing to the replacement.

---

161. Intrinsic examples

The following are conceptual examples of semantic identities:

math.abs
math.sqrt
math.sin
math.cos

integer.checked_add
integer.saturating_add
integer.rotate_left

vector.map
vector.reduce
vector.broadcast

matrix.transpose
matrix.multiply
matrix.solve

tensor.reshape
tensor.transpose
tensor.contract

signal.fft
signal.convolve

statistics.mean
statistics.variance

atomic.load
atomic.store
atomic.compare_exchange

parallel.map
parallel.reduce

These names are examples of semantic identities, not a requirement that all of them become stable Zamani intrinsics immediately.

---

162. What must remain library functions

Operations that have no special compiler semantic requirement SHOULD remain ordinary library APIs.

Examples may include domain-specific algorithms where:

ordinary function semantics

are sufficient.

The compiler should not recognize an operation as an intrinsic merely to make the language appear more feature-rich.

---

163. What must remain backend operations

Operations whose only purpose is target implementation should remain downstream.

Examples:

specific CPU instruction
specific GPU kernel primitive
specific FPGA primitive
specific accelerator opcode
vendor memory instruction

These belong to backend/target layers or explicit dialects.

---

164. What must remain deployment information

Physical assignments such as:

device identity
node identity
memory bank
physical accelerator
physical CPU
physical GPU
physical FPGA

belong to deployment/target configuration unless explicitly exposed through a target-specific dialect.

They must not become the portable meaning of a classical intrinsic.

---

165. Cross-domain semantic rule

An intrinsic may be consumed by multiple domains.

For example:

tensor.contract

may be used by:

classical
AI
quantum-classical hybrid
HDL acceleration
distributed computation

The intrinsic must therefore define one semantic contract rather than one implementation for each domain.

---

166. Canonical semantic identity

If several source-level APIs map to the same operation, they MAY lower to one canonical semantic identity.

For example:

library.matmul
tensor.matmul

could theoretically map to:

canonical tensor multiplication

if the language specification declares them semantically equivalent.

The canonical identity belongs to semantic analysis/IR, not grammar.

---

167. Alias handling

Intrinsic aliases must be explicit.

An alias must identify:

canonical intrinsic
alias
version
compatibility

Aliases must not create divergent semantics.

---

168. Intrinsic metadata provenance

Compiler-recognized intrinsic definitions SHOULD have provenance metadata:

specification version
definition source
implementation version
dialect
compatibility status

This helps reproducible compilation and diagnostics.

---

169. Documentation consistency

The following must agree:

grammar/spec/classical.md
grammar/classical/intrinsics.md
intrinsic registry
implementation
tests
generated reference

The authoritative semantic contract is the specification.

Generated documentation must reflect implementation status accurately.

---

170. Production integration checklist

Before declaring this file complete, verify:

[ ] grammar/classical/README.md references this intrinsic contract
[ ] grammar/spec/classical.md remains authoritative for broader classical semantics
[ ] grammar/expressions/calls.g4 remains the call-syntax owner
[ ] grammar/types/ remains the type-syntax owner
[ ] grammar/effects/ remains the effect-syntax owner
[ ] grammar/resources/ remains the resource-syntax owner
[ ] grammar/hardware/ remains the hardware-capability owner
[ ] grammar/compile/ remains the compilation-strategy owner
[ ] grammar/execution/ remains the runtime/execution owner
[ ] grammar/interoperability/ remains the external-call owner
[ ] no second intrinsic grammar exists
[ ] no second intrinsic IR exists
[ ] no intrinsic-specific AST hierarchy is unnecessarily introduced
[ ] no fixed hardware limits are introduced
[ ] no unsafe Rust is required
[ ] Rust 1.97/1.97.1 compatibility is preserved

---

171. Repository integration contract

This file is complete independently when its contract is sufficient for the following future implementation work to be performed without changing this document merely because another repository component is implemented differently:

lexer
parser
AST
semantic resolver
type checker
effect checker
resource checker
intrinsic registry
canonical IR
optimizer
compiler
runtime
backend
tests
IDE/LSP

If one of those components changes its internal implementation without changing the language semantics, this file should remain valid.

A semantic change, however, requires a specification/versioning update.

---

172. Required implementation-side contracts

The implementation should provide, conceptually:

IntrinsicId
IntrinsicNamespace
IntrinsicVersion
IntrinsicSignature
IntrinsicEffect
IntrinsicDeterminism
IntrinsicCapabilityRequirement
IntrinsicResourceRequirement
IntrinsicResolution
IntrinsicLowering

These are semantic/compiler concepts.

Their exact Rust structures belong in the implementation repository and must not be duplicated as Rust code in this grammar document.

---

173. Safe Rust implementation rule

The implementation should favor:

enum
struct
trait
Result
Option
immutable metadata
owned strings
borrowed slices
safe collections
explicit error types

and must not require:

unsafe
raw-pointer manipulation
unchecked memory access
parser-time execution

for intrinsic resolution.

---

174. Completion definition

"grammar/classical/intrinsics.md" is complete when it establishes all of the following:

intrinsic identity
        ↓
generic call syntax
        ↓
AST call
        ↓
semantic resolution
        ↓
type/effect validation
        ↓
capability/resource validation
        ↓
canonical semantic IR
        ↓
optimization
        ↓
target-independent lowering contract
        ↓
CPU/GPU/FPGA/ASIC/accelerator/distributed realization

without introducing:

a competing grammar
a competing AST
a competing type system
a competing classical IR
a competing quantum IR
hardware ceilings
vendor lock-in
unsafe Rust

---

175. Final architectural rule

The central rule of Zamani classical intrinsics is:

«An intrinsic describes a stable semantic capability, not a machine instruction, physical device, library implementation, or hardware capacity.»

Therefore:

Zamani source
      ↓
semantic intrinsic
      ↓
canonical semantic representation
      ↓
available capabilities/resources
      ↓
optimization
      ↓
target realization

The same intrinsic may therefore execute on:

atom-scale or embedded systems
CPU
multicore CPU
vector processor
GPU
FPGA
ASIC
DSP
accelerator
HPC system
cluster
distributed system
cloud
simulator
future architecture

without changing its source-level semantic identity.

The language does not promise physically infinite hardware.

It promises that the language itself does not impose artificial machine-scale ceilings.

That distinction is fundamental to POCO-REAF.

---

176. Final invariants

The following invariants are mandatory:

1. Intrinsics are semantic, not lexical.

2. Intrinsic names do not automatically become keywords.

3. Generic call syntax remains owned by "grammar/expressions/".

4. Types remain owned by "grammar/types/".

5. Effects remain owned by "grammar/effects/".

6. Resources remain owned by "grammar/resources/".

7. Hardware capabilities remain owned by "grammar/hardware/".

8. Compilation strategy remains owned by "grammar/compile/".

9. Runtime strategy remains owned by "grammar/execution/".

10. External ABI semantics remain owned by "grammar/interoperability/".

11. The frontend AST remains domain-neutral.

12. The intrinsic system does not create another IR.

13. "quantum::ir" remains the canonical quantum semantic boundary.

14. No universal hardware capacity is encoded into intrinsic semantics.

15. No fixed vector length is encoded.

16. No fixed matrix dimensions are encoded.

17. No fixed tensor rank is encoded.

18. No fixed thread count is encoded.

19. No fixed node count is encoded.

20. No fixed memory capacity is encoded.

21. No fixed device count is encoded.

22. Resource availability is distinct from language validity.

23. Capability requirements are distinct from physical device selection.

24. Preferences are distinct from requirements.

25. Hints are distinct from requirements.

26. Hardware realization is downstream.

27. Vendor implementation is downstream.

28. Intrinsic execution never occurs during parsing.

29. Intrinsic resolution is deterministic.

30. Intrinsic semantics are versioned.

31. Stable intrinsics are testable across targets.

32. Compiler implementation uses safe Rust only.

33. Rust 1.97/1.97.1 and Rust 2021 remain supported.

34. Future computing domains can consume existing intrinsic semantics without redefining the core language.

35. POCO-REAF remains the governing portability objective.

---

177. One-line contract

The complete classical intrinsic architecture can therefore be reduced to:

Generic Zamani call
    → semantic intrinsic identity
    → typed/effect/resource/capability validation
    → canonical semantic IR
    → optimization
    → target realization

and never:

Generic Zamani call
    → fixed hardware instruction
    → fixed hardware capacity
    → vendor-specific implementation

That is the production boundary required for Zamani classical intrinsics to scale from the smallest supported computation to arbitrarily large computations permitted by available resources while preserving the single-language, target-independent POCO-REAF architecture.