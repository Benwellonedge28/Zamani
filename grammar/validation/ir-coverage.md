Zamani IR Coverage Specification

Path: "grammar/validation/ir-coverage.md"
Status: Normative production validation specification
Version: 1.0.0
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Rust baseline: Rust 1.97 / Rust 1.97.1
Edition: Rust 2021
Safety: Safe Rust only; Rust "unsafe" is prohibited
Primary objective: Complete, lossless, deterministic traceability from accepted Zamani language semantics to canonical intermediate representations and their downstream consumers
Portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)"
Scalability objective: From the smallest representable computation to arbitrarily large computations permitted by the program's semantics and the resources actually available to compilation, execution, and target realization.

---

1. Purpose

This document defines the normative production contract for IR coverage in the Zamani compiler and grammar architecture.

IR coverage answers a precise question:

«For every supported Zamani construct that has reached semantic validity, is there a defined, lossless, verifiable, target-independent representation at the correct canonical IR boundary, with known downstream consumers and conformance evidence?»

IR coverage is therefore not:

- the number of IR instructions;
- the number of parser rules;
- the number of tests;
- the number of backend targets;
- a percentage generated from line coverage;
- the number of enum variants in "src/ir_gen.rs".

IR coverage is semantic traceability.

The required chain is:

Zamani specification
        │
        ▼
lexical contract
        │
        ▼
grammar
        │
        ▼
lexer
        │
        ▼
parser
        │
        ▼
src/ast/
        │
        ▼
semantic analysis
        │
        ▼
canonical semantic representation
        │
        ▼
canonical IR
        │
        ▼
IR verification
        │
        ▼
optimization / lowering
        │
        ├──────────────┐
        ▼              ▼
 classical IR      quantum::ir
        │              │
        └──────┬───────┘
               ▼
        target-independent
        lowering / analysis
               │
       ┌───────┼────────┐
       ▼       ▼        ▼
    routing scheduling resilience
                       │
                       ▼
                  QEC / ZQN
                       │
                       ▼
                      HAL
                       │
                       ▼
                target realization

A construct is IR-covered only when the appropriate path exists and the representation preserves every semantic property required by downstream stages.

---

2. Actual Repository Baseline

This specification is intentionally grounded in the repository as it currently exists.

2.1 Canonical frontend AST

The repository currently contains:

src/ast/mod.rs

and the current repository does not establish "src/frontend/ast/" as an existing implementation authority.

Therefore:

«"src/ast/" is the current AST implementation authority unless and until the repository deliberately changes that authority through an explicit architecture migration.»

No second AST must be created merely to satisfy this document.

The current AST includes ordinary constructs such as:

- variables;
- constants;
- functions;
- structs;
- enums;
- traits;
- implementations;
- modules;
- imports;
- classes;
- interfaces;
- quantum constructs;
- noise models;
- fidelity checks;
- surface-code constructs;
- nano constructs;
- Sankofa constructs;
- effects;
- language declarations;
- "Unsafe";
- omniversal constructs;
- type classes;
- higher-kinded types;
- additional extended declarations.

Every such AST variant must eventually have an explicit IR disposition:

SUPPORTED
LOWERED
PRESERVED-AS-SEMANTIC-DATA
DEFERRED
EXPLICITLY-UNSUPPORTED
NOT-YET-IMPLEMENTED

It must never silently disappear.

---

2.2 Current IR implementation

The current general IR implementation is:

src/ir_gen.rs

The current structural verifier is:

src/ir_verify.rs

The current IR contains, among other things:

IrType
IrRegister
IrValue
IrInstruction
IrFunction
IrGlobal
IrModule

and instruction families including:

Alloca
Load
Store

Add
Sub
Mul
Div
Rem
Neg

And
Or
Xor
Shl
Shr
Not

Cmp

Label
Jump
CondJump
Ret
Unreachable

Call
CallIndirect

GetElementPtr
BitCast
Assign
Phi

ZExt
SExt
Trunc
FpExt
FpTrunc
SIToFP
FPToSI

QuantumGate
NanoOp
SankofaRecall
SankofaRemember
Comment

This existing IR must not be declared complete merely because these variants exist.

---

2.3 Current IR verifier

"src/ir_verify.rs" currently verifies useful structural properties including:

- duplicate functions;
- duplicate globals;
- duplicate parameters;
- duplicate labels;
- register definitions;
- undefined registers;
- basic operand type equality;
- comparison result type;
- branch targets;
- return type;
- some conversion destination types;
- phi non-emptiness;
- phi value types;
- function termination;
- basic predecessor existence.

This is a useful foundation.

It is not yet sufficient for production IR coverage.

The missing properties defined by this document are mandatory production requirements.

---

2.4 Current quantum boundary

The repository explicitly establishes:

src/quantum/ir/

as the canonical quantum semantic/IR boundary.

The quantum IR documentation explicitly defines this layer as hardware-independent semantic representation.

Therefore:

«"quantum::ir" is the only canonical quantum semantic IR boundary.»

The general-purpose "IrInstruction::QuantumGate" in "src/ir_gen.rs" must not become a competing quantum IR.

Likewise:

NanoOp
SankofaRecall
SankofaRemember

must not be allowed to become undocumented semantic black holes.

Each must either:

1. have an authoritative semantic/IR contract;
2. lower to an existing canonical representation;
3. become an explicitly documented extension boundary;
4. or be rejected as not implemented.

---

3. Authority

The following ownership model is mandatory.

Layer| Authority
"grammar/DESIGN.md"| Architecture
"grammar/specification/"| Normative human-readable language specification
"grammar/spec/"| Formal language contracts
"grammar/Zamani.g4"| Canonical ANTLR root/composition grammar
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Historical/proposed/extended design material
"src/lexer.rs"| Executable lexical implementation
"src/parser.rs"| Executable parser implementation
"src/ast/"| Current frontend AST authority
"src/semantic.rs"| Executable semantic analysis
"src/ir_gen.rs"| Current general IR generation
"src/ir_verify.rs"| Current general IR structural verification
"src/quantum/ir/"| Canonical quantum semantic IR
downstream quantum compiler| Quantum optimization/routing/scheduling/etc.
QEC| Error-correction implementation
ZQN| Quantum noise/fault/resilience semantics
HAL| Hardware realization
"grammar/validation/"| Validation policy
"grammar/tests/"| Conformance evidence

No validation document may silently establish another IR authority.

---

4. Definition of IR Coverage

A feature is IR-covered only when all applicable conditions hold:

Specification
      +
Grammar
      +
Lexer/parser
      +
AST
      +
Semantic interpretation
      +
Canonical IR mapping
      +
Information preservation
      +
IR verification
      +
Downstream ownership
      +
Diagnostics
      +
Positive tests
      +
Negative tests
      +
Boundary tests
      +
Scalability tests
      +
Determinism tests
      +
Compatibility evidence

If any mandatory element is absent, the feature is not fully covered.

---

5. Coverage Status Model

IR coverage MUST use explicit statuses.

5.1 "SPECIFIED"

The construct is defined by the normative specification.

No implementation claim is implied.

5.2 "PARSED"

The lexer/parser can construct an AST representation.

No semantic or IR implementation claim is implied.

5.3 "SEMANTIC"

The semantic analyzer defines the meaning.

No IR implementation claim is implied.

5.4 "IR_PARTIAL"

Some IR mapping exists but one or more semantic properties are not represented or verified.

5.5 "IR_COVERED"

The complete required semantic information has a canonical, verifiable IR representation.

5.6 "DOWNSTREAM_PARTIAL"

IR is complete, but required lowering/optimization/backend integration remains incomplete.

5.7 "TARGET_INCOMPATIBLE"

The program semantics are valid, but a particular target lacks required capabilities.

This is not an IR coverage failure.

5.8 "RESOURCE_UNSATISFIABLE"

The semantic requirements cannot currently be satisfied by the available resources.

This is not a parser or IR validity failure.

5.9 "UNSUPPORTED"

The feature is explicitly unsupported by the current compiler.

5.10 "UNVERIFIED"

Implementation may exist, but the required evidence has not been established.

5.11 "SEMANTIC_LOSS"

A lowering step discards required meaning.

This is a production blocker.

---

6. Core IR Coverage Invariant

For every semantically valid construct:

AST meaning
    =
semantic meaning
    =
IR meaning

subject only to explicitly documented representation transformations.

A transformation is acceptable only when:

source semantics
       ↓
representation transformation
       ↓
equivalent semantics

A transformation is invalid when it silently changes:

- value;
- type;
- ordering;
- control flow;
- ownership;
- lifetime;
- effect;
- capability requirement;
- resource requirement;
- precision;
- quantum semantics;
- measurement semantics;
- probabilistic semantics;
- timing semantics;
- hardware intent;
- security guarantees;
- provenance;
- source mapping.

---

7. No Silent Semantic Loss

The following are production errors.

Example

AST:

Tensor<T, shape>

lowered to:

fixed Array<T, N>

without preserving arbitrary shape semantics.

Example

A large integer literal is accepted by the source language but silently converted to "i64".

Example

A quantum operation with arbitrary parameters is converted to an enumerated gate type that cannot preserve its operation identity.

Example

A resource requirement:

requires capability("quantum.measurement")

is discarded during lowering.

Example

A hardware-independent quantum operation is converted directly into a physical-qubit instruction before routing.

All such cases are:

SEMANTIC_LOSS

and block production readiness.

---

8. Current IR Findings Requiring Correction

The following are current implementation findings, not hypothetical recommendations.

8.1 Fixed target triple in "IrModule::new"

The current "IrModule::new" initializes a concrete target:

x86_64-pc-linux-gnu

and a concrete data layout.

This violates target-independent IR construction.

The IR module itself must not silently select the target.

Required correction

Target information must be represented as an explicit downstream compilation context or target profile.

The canonical semantic IR must remain target-neutral.

If a backend-specific LLVM module requires a target triple, that information must be attached during backend lowering.

The production invariant is:

semantic IR
    ≠
target-specific backend module

A future CPU, GPU, FPGA, QPU, simulator, accelerator, cluster, or other target must not require changing the semantic IR model.

---

9. Integer Representation Must Be Lossless

The current "IrValue" contains:

ConstInt(i64, IrType)

This is insufficient as a universal source-level representation if Zamani accepts integer literals or semantic integer values beyond signed 64-bit range.

The grammar's scalability requirement must not be defeated by an IR representation limit.

Required model

The semantic IR must distinguish:

source integer

from:

target machine integer

The canonical representation must be capable of preserving arbitrary source magnitude where the language semantics permit it.

Acceptable implementations include a canonical arbitrary-precision representation or an explicit semantic integer representation whose lowering is target-dependent.

What is prohibited:

source literal
    ↓
i64
    ↓
silent truncation

If a target requires a bounded integer representation, the compiler must report the limitation or lower through an appropriate arbitrary-precision/runtime representation.

---

10. Floating-Point Representation

Floating-point lowering must preserve the source type's declared semantics.

The IR must not silently convert:

F64

to:

F32

or otherwise narrow precision.

Any narrowing must be:

- explicit;
- semantically permitted;
- represented in IR;
- verifiable;
- diagnosable where required.

---

11. Unsigned Integer Representation

The current IR spelling maps:

U8
U16
U32
U64
U128

to LLVM integer names.

That representation is possible because LLVM integer bit width itself does not encode signedness.

However, signedness must remain semantic information.

Therefore:

U32

must not become indistinguishable from:

I32

during semantic analysis.

IR operations must carry enough information for signed/unsigned comparisons and conversions.

A textual backend representation may use the same bit width while retaining signedness in the instruction semantics.

---

12. Array Representation

The current:

IrType::Array(Box<IrType>, usize)

must not become a universal representation for all Zamani arrays.

An array count can be:

- compile-time static;
- runtime dynamic;
- symbolic;
- dependent on program data;
- target-specialized.

Therefore the IR type system must distinguish these cases.

For example:

StaticArray<T, n>
DynamicArray<T>
Slice<T>
Tensor<T, shape>

or equivalent canonical representations.

A dynamic or symbolic source array must not be silently converted into:

Array<T, 0>

because "0" was used as a placeholder.

The current "unwrap_or(0)" behavior in "IrType::from_ast_type" must therefore be treated as a production IR coverage blocker for any array whose size is not semantically known.

---

13. Tensor Representation

Tensor semantics must preserve:

- element type;
- rank;
- shape;
- symbolic dimensions;
- layout requirements;
- indexing semantics;
- ownership/lifetime where applicable;
- device/resource requirements;
- sparsity or density semantics where specified.

The IR must not encode a universal maximum tensor rank.

The compiler may use target-specific limits during lowering.

---

14. Pointer and Memory Semantics

A pointer-like representation must preserve the semantic distinction between:

- owned memory;
- borrowed memory;
- shared memory;
- immutable memory;
- mutable memory;
- address spaces;
- accelerator memory;
- distributed memory;
- persistent memory;
- opaque handles;
- foreign pointers.

The IR must not flatten all of these into:

Ptr<T>

if doing so loses semantic guarantees.

Where the semantic layer intentionally abstracts the distinction, that abstraction must be explicit.

---

15. Function and Call Coverage

The current IR verifier verifies argument values for "Call" but does not establish complete call-signature conformance.

Production IR verification must verify:

1. called function existence when statically resolvable;
2. external declaration availability;
3. argument count;
4. argument types;
5. return type;
6. indirect-call function type;
7. calling convention where applicable;
8. effect/capability requirements;
9. ABI constraints only at the backend boundary;
10. foreign-function requirements where applicable.

A call cannot be considered verified merely because all argument registers exist.

---

16. Global Coverage

Globals must be verified for:

- unique names;
- valid types;
- initializer compatibility;
- constant mutability;
- symbol visibility;
- linkage;
- alignment;
- addressability;
- string-literal ownership;
- cross-function references.

String-literal symbols must not collide with:

- functions;
- globals;
- types;
- backend-generated symbols.

---

17. Register Coverage

Every virtual register must have:

- one definition;
- one stable type;
- deterministic identity;
- a valid lifetime according to the IR model;
- no use before definition unless the IR explicitly permits block parameters or equivalent SSA semantics.

The verifier must distinguish:

definition

from:

use

and must validate dominance/use ordering according to the actual SSA model.

---

18. SSA Coverage

The current IR contains "Phi".

A production verifier must therefore validate the complete SSA contract.

For every phi node:

phi(result, [(value1, predecessor1), ...])

the verifier must establish:

1. every predecessor label exists;
2. every listed predecessor actually terminates in the phi's block;
3. every predecessor occurs exactly once unless the IR explicitly permits duplicate edges;
4. the number of incoming edges matches the CFG semantics;
5. each incoming value is valid at its predecessor;
6. each incoming value has the phi result type;
7. the phi result has one definition;
8. the phi is located at a legal block position;
9. no illegal use-before-definition exists.

The current predecessor-count check is insufficient by itself.

---

19. Control-Flow Graph Coverage

The verifier must construct a logical CFG.

Every non-external function must have:

- an entry block;
- reachable blocks identified;
- terminators identified;
- successor edges;
- predecessor edges;
- valid branch targets;
- no fall-through from a terminated block;
- no illegal instructions after a terminator.

The verifier must distinguish:

unreachable block

from:

invalid block

An unreachable block may be legal when explicitly represented.

A malformed block is not.

---

20. Terminator Coverage

Each basic block must end in exactly one legal terminator.

Legal terminators include the canonical control-flow instructions defined by the IR.

Examples:

Ret
Jump
CondJump
Unreachable

A block must not contain:

Ret
Add

or:

Jump
Store

unless the IR specification explicitly permits unreachable trailing instructions and defines their semantics.

The production default is:

«No instruction may follow a terminator in the same basic block.»

---

21. Branch Condition Coverage

Conditional branches must require a semantic boolean condition.

A non-boolean value must not be implicitly treated as truthy unless the language specification explicitly defines that conversion and the IR represents it.

---

22. Comparison Coverage

The current verifier checks operand type equality and boolean result type.

Production verification must additionally validate:

- comparison/operator compatibility;
- signed versus unsigned comparison;
- floating-point comparison legality;
- NaN semantics where relevant;
- quantum comparison restrictions;
- pointer comparison restrictions;
- opaque-type restrictions.

A comparison must not be accepted merely because two operands happen to have equal IR types.

---

23. Arithmetic Coverage

Arithmetic instructions must verify operation legality.

For example:

Add
Sub
Mul
Div
Rem

must not blindly accept:

Void
Function
Opaque
Quantum

unless the semantic contract explicitly defines those operations.

The verifier must distinguish:

same type

from:

valid operation for that type

---

24. Division and Remainder Semantics

The IR contract must define:

- signed division;
- unsigned division;
- remainder;
- division by zero;
- overflow behavior;
- floating-point division;
- exceptional behavior.

The current generic "Div" representation must not silently imply signed integer division for every numeric type.

The verifier must reject impossible combinations and require explicit operation variants where semantics differ.

---

25. Shift Coverage

"Shl" and "Shr" require:

- integral operands;
- valid shift-count semantics;
- signed/unsigned distinction for right shift;
- defined behavior for oversized shift counts.

No universal shift-count maximum may be encoded.

Target lowering may impose actual machine restrictions.

---

26. Conversion Coverage

Every conversion must specify:

source type
destination type
conversion semantics
overflow behavior
rounding behavior
sign behavior
precision behavior

The following must not be accepted solely because the destination register has the requested type:

ZExt
SExt
Trunc
FpExt
FpTrunc
SIToFP
FPToSI
BitCast

For example:

ZExt i64 -> i32

is not a valid zero extension.

Likewise:

SIToFP f64 -> i32

has the wrong destination category.

Conversion legality must be semantically verified.

---

27. BitCast Coverage

"BitCast" must verify representation compatibility.

It must not become a general-purpose semantic conversion escape hatch.

A bitcast must be allowed only when:

- source representation is compatible;
- destination representation is compatible;
- size/alignment constraints are valid;
- no semantic ownership guarantees are violated.

Target-specific reinterpretation belongs downstream.

---

28. Memory Operation Coverage

For:

Alloca
Load
Store
GetElementPtr

the verifier must establish:

- pointer/value compatibility;
- pointee compatibility;
- address-space compatibility;
- alignment requirements;
- element/index validity;
- aggregate traversal validity;
- mutability requirements;
- lifetime requirements where represented.

A "Store" must not be accepted merely because both operands exist.

---

29. "GetElementPtr" Coverage

The current verifier validates only the existence of the base and indices.

Production verification must validate the aggregate path.

For each index:

base type
   ↓
aggregate/member
   ↓
index type
   ↓
result pointer type

must be consistent.

The result register must have the correct pointer type.

---

30. Quantum IR Coverage

Quantum syntax must not be considered covered merely because:

IrInstruction::QuantumGate

exists.

The canonical path is:

Zamani quantum syntax
       ↓
src/ast/
       ↓
semantic quantum model
       ↓
quantum::ir
       ↓
optimization
       ↓
routing
       ↓
scheduling
       ↓
QEC / resilience
       ↓
ZQN
       ↓
HAL
       ↓
target

"src/ir_gen.rs" must not create a second authoritative quantum semantic model.

---

31. Quantum Operation Identity

Quantum operations must not be restricted to a fixed enum such as:

H
X
Y
Z
CNOT
...

The canonical representation must support operation identity as semantic data.

At minimum, a generic operation must preserve:

name
namespace
operands
parameters
results
attributes
modifiers
effects
capabilities
source provenance

Therefore:

apply H ...
apply custom_gate ...
apply vendor.operation ...
apply operation(parameter) ...

must be representable without changing the language architecture.

---

32. Quantum Resource Coverage

Quantum IR must preserve semantic resource requirements such as:

requires qubits >= n
requires capability("quantum.measurement")
requires capability("quantum.mid_circuit_measurement")
requires topology(...)
requires capability("quantum.error_correction")

It must not replace those requirements with physical allocation.

For example:

logical qubit

must remain distinct from:

physical qubit

until downstream realization.

---

33. Quantum Physical Mapping

Physical qubit identifiers belong downstream.

The IR coverage validator must reject designs where canonical semantic quantum IR requires:

physical_qubit(0)
physical_qubit(1)
...

as source-level universal semantics.

Physical mapping may exist in a downstream target realization IR.

---

34. Quantum Measurement

Measurement must preserve:

- measured quantum operands;
- measurement basis/observable;
- result destination;
- classical result type;
- ordering;
- effects;
- synchronization semantics;
- mid-circuit behavior;
- feed-forward dependency.

Measurement must not be reduced to an ordinary classical function call if doing so loses quantum semantics.

---

35. Quantum Classical Feed-Forward

A construct such as:

measure q -> bit
if bit {
    apply operation ...
}

must preserve the dependency:

measurement
     ↓
classical result
     ↓
control dependency
     ↓
quantum operation

The optimizer must not legally reorder the operation across this dependency.

---

36. Quantum Noise and Resilience

Noise declarations and resilience requirements must not be represented as comments.

For example:

noise model
requires fault tolerance
requires error correction

must either:

- map into canonical quantum semantic IR;
- map into an explicit resilience/QEC semantic contract;
- or be rejected as unsupported.

A comment-only lowering is not IR coverage.

---

37. QEC Boundary

QEC belongs downstream of canonical quantum semantics.

The IR coverage contract is:

source-level QEC intent
        ↓
semantic representation
        ↓
quantum::ir / resilience contract
        ↓
QEC analysis
        ↓
physical realization

The grammar must not encode:

- physical code layouts;
- fixed device capacities;
- decoder implementation;
- calibration;
- physical syndrome routing.

---

38. ZQN Boundary

ZQN must consume explicit semantic information.

IR coverage must verify that:

noise
reliability
fault
resilience
error
uncertainty

information is not silently discarded before ZQN.

---

39. HDL IR Coverage

HDL constructs must lower to a representation that preserves hardware intent.

At minimum, applicable constructs must preserve:

- module identity;
- ports;
- direction;
- signal identity;
- widths;
- parameterization;
- combinational semantics;
- sequential semantics;
- clock/reset semantics;
- timing constraints;
- state-machine semantics;
- memory intent;
- generate semantics;
- verification properties;
- synthesis intent.

No universal width must be introduced by the IR.

---

40. Hardware Intent versus Hardware Realization

The IR must distinguish:

hardware intent

from:

physical implementation

For example:

register width = W

is semantic intent.

It must not become:

32-bit register

unless "32" is actually part of the source program.

Likewise:

memory capacity

must not become a universal fixed capacity in semantic IR.

---

41. Resource and Capability IR

Resource semantics must be preserved.

The canonical distinction is:

Requirement
Constraint
Capability
Preference
Hint
Budget
Realization

These are not interchangeable.

Example:

requires capability("gpu.compute")

must not lower directly to:

use GPU 0

The latter is a downstream realization.

---

42. Distributed IR Coverage

Distributed constructs must preserve:

- process identity;
- communication relationships;
- message semantics;
- ordering;
- consistency requirements;
- replication requirements;
- partitioning intent;
- fault-tolerance requirements;
- placement constraints;
- resource requirements.

No fixed node count may be encoded.

---

43. Concurrency IR Coverage

Concurrency semantics must preserve:

- task identity;
- dependency;
- synchronization;
- ownership;
- communication;
- cancellation;
- ordering;
- determinism requirements;
- parallelism intent.

The IR must not silently replace:

parallel

with:

8 threads

or any other universal fixed implementation.

---

44. Data and Tensor IR Coverage

Data constructs must preserve:

- schema;
- element type;
- dimensions;
- shape;
- symbolic shape;
- ordering;
- ownership;
- stream semantics;
- provenance;
- serialization requirements.

The IR must not silently materialize an arbitrarily large logical dataset as a fixed-size machine array.

---

45. AI/ML IR Coverage

AI/ML constructs must preserve applicable:

- model identity;
- tensor semantics;
- parameter semantics;
- training semantics;
- inference semantics;
- differentiation;
- optimization;
- dataset relationships;
- device/resource requirements.

Framework-specific implementation must remain downstream.

The IR must not become a hidden TensorFlow/PyTorch/CUDA representation.

---

46. Networking IR Coverage

Networking constructs must preserve:

- endpoint semantics;
- protocol semantics;
- transport requirements;
- ordering;
- reliability;
- security requirements;
- streaming;
- capability requirements.

Concrete IP addresses, ports, interfaces, or physical links belong to deployment/target realization unless explicitly part of source semantics.

---

47. Security IR Coverage

Security requirements must never be lowered into comments.

For example:

requires confidentiality
requires authenticated_channel
requires capability("secure_compute")

must remain machine-readable semantic information.

Cryptographic algorithms may be represented as explicit semantic operations where the language specification requires them.

Vendor/library selection remains downstream.

---

48. Effects IR Coverage

Effect declarations and operations must preserve:

- effect identity;
- operation identity;
- effect parameters;
- handler relationships;
- ordering;
- resumability;
- capability requirements.

An effect must not disappear merely because the current backend does not directly support it.

---

49. Ownership and Resource Semantics

If semantic analysis determines that a value is:

- linear;
- affine;
- borrowed;
- mutable;
- shared;
- owned;
- consumed;

then IR lowering must preserve enough information for downstream verification.

The IR must not accidentally turn:

consume(resource)

into an ordinary unrestricted copy.

---

50. Source Provenance

Every IR construct that can produce a diagnostic, optimization decision, or runtime observation must retain source provenance where required.

The provenance chain should be:

source file
→ source span
→ AST node
→ semantic entity
→ IR entity
→ lowered entity
→ backend entity

The IR coverage validator must detect constructs that lose required source identity.

---

51. Source Spans

IR coverage depends on the source-span contract defined by:

grammar/validation/source-spans.md

Every user-visible semantic error must be traceable back to an appropriate source span.

Synthetic IR may have:

synthetic provenance

but must not fabricate a user source location.

---

52. Diagnostics

Every IR coverage failure must have a structured category.

Recommended categories:

IR_UNMAPPED_FEATURE
IR_PARTIAL_FEATURE
IR_SEMANTIC_LOSS
IR_TYPE_MISMATCH
IR_INVALID_OPERATION
IR_INVALID_CONTROL_FLOW
IR_INVALID_PHI
IR_INVALID_CALL
IR_INVALID_MEMORY_ACCESS
IR_INVALID_CONVERSION
IR_INVALID_QUANTUM_LOWERING
IR_INVALID_RESOURCE_LOWERING
IR_INVALID_CAPABILITY_LOWERING
IR_TARGET_LEAK
IR_RESOURCE_EXHAUSTION
IR_UNSUPPORTED
IR_UNVERIFIED
IR_COMPATIBILITY_FAILURE

Diagnostics must be deterministic.

---

53. IR Verification Must Not Depend on Hardware

"src/ir_verify.rs" must verify semantic IR independently of:

- CPU model;
- GPU model;
- FPGA;
- QPU;
- accelerator;
- network topology;
- current resource availability;
- filesystem;
- network;
- environment variables.

Target compatibility is a separate validation stage.

---

54. Target Compatibility

The distinction is:

IR valid
    ≠
target compatible

For example:

valid quantum::ir

may be:

not directly supported by target A

but:

supported after decomposition by target B

or:

supported by simulation target C

This is not an IR error.

---

55. Resource Feasibility

Likewise:

semantic requirement

must be distinct from:

currently available resource

A program requiring "n" qubits remains semantically valid when a selected target has fewer than "n" usable qubits.

The compiler must report:

RESOURCE_UNSATISFIABLE

for that target rather than changing the program semantics.

---

56. POCO-REAF IR Invariant

A canonical semantic IR must not encode today's machine as the language.

The following are prohibited as universal semantic IR limits:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_TIMELINES

The following are also prohibited as universal semantic allocations:

physical_qubit_0
physical_qubit_1
gpu_0
cpu_0
node_0
device_0

unless they are explicitly part of a downstream target-realization representation.

---

57. "Infinity" and Finite Implementations

"Infinity" means:

«no artificial finite machine-size ceiling is encoded by the language or canonical IR.»

It does not mean that an actual process can allocate an infinite number of objects.

Every concrete compilation and execution remains bounded by:

- available memory;
- storage;
- compiler resources;
- target resources;
- runtime resources;
- address-space constraints;
- explicitly defined program semantics.

These are operational realities, not language-level ceilings.

---

58. Resource-Scalable IR Verification

The verifier must scale with the actual IR supplied to it.

It must not contain artificial language limits such as:

MAX_IR_INSTRUCTIONS
MAX_FUNCTIONS
MAX_BLOCKS
MAX_REGISTERS
MAX_PHI_NODES
MAX_OPERANDS
MAX_QUANTUM_OPERATIONS
MAX_QUBITS

Implementation budgets may exist externally.

For example:

validation budget
memory budget
time budget
diagnostic budget

but those are tooling/resource policies, not language limits.

---

59. Resource Exhaustion

The verifier must distinguish:

invalid IR

from:

verifier could not complete because resources were exhausted

The latter must not be reported as a semantic correctness failure.

A production API should therefore have a distinction equivalent to:

Valid
Invalid(errors)
ResourceExhausted(diagnostic)

or another representation with the same semantics.

---

60. Determinism

For identical:

repository state
compiler version
Rust toolchain
language version
grammar configuration
feature registry
IR input
validation configuration

the IR coverage result must be deterministic.

This includes:

- status;
- diagnostics;
- feature ordering;
- dependency traversal;
- generated reports;
- hashes where used.

Hash maps must not determine externally visible ordering.

Use deterministic ordering when producing reports.

---

61. Incremental Validation

IR coverage should support incremental validation.

A change to:

grammar/quantum/operations.g4

should not require semantically unrelated IR validation work when dependency information proves it is unaffected.

However, invalidation must follow the dependency graph.

For example:

grammar
   ↓
AST
   ↓
semantic
   ↓
IR

means a semantic contract change invalidates downstream IR evidence.

Incremental caching must never cause stale coverage to be reported as current.

---

62. Feature Identity

Every feature must have a stable feature identifier.

Recommended structure:

zamani.<domain>.<feature>

Examples:

zamani.core.function
zamani.types.generic
zamani.classical.tensor
zamani.quantum.operation
zamani.quantum.measurement
zamani.hdl.module
zamani.hardware.capability
zamani.resources.requirement
zamani.concurrency.parallel

Identifiers must be stable across file renames or grammar refactoring.

Existing files must not be unnecessarily renamed to establish identifiers.

---

63. Feature Manifest Contract

Where feature manifests are introduced, each feature manifest must provide:

id
name
status
version
specification
grammar
lexer
ast
semantic
ir
downstream
tests
diagnostics
compatibility
scalability
determinism

For IR specifically:

ir.authority
ir.kind
ir.operations
ir.types
ir.attributes
ir.effects
ir.resources
ir.capabilities
ir.provenance
ir.lowering
ir.verification

A feature cannot claim "IR_COVERED" without the applicable fields.

---

64. Grammar-to-IR Traceability

The validator must be able to answer:

Which grammar construct produced this AST node?
Which semantic rule interpreted it?
Which IR construct represents it?
Which verifier rule validates it?
Which downstream stage consumes it?
Which tests prove it?

Traceability must work in both directions.

Forward

grammar
→ AST
→ semantic
→ IR

Reverse

IR
→ semantic feature
→ AST feature
→ grammar/specification

An IR instruction without an identifiable semantic owner is suspicious and must be classified.

---

65. AST-to-IR Completeness

For every AST variant, the coverage registry must answer:

Can this variant be lowered?

Possible answers:

YES
PARTIAL
NO
NOT_APPLICABLE
DEFERRED

For example, an AST node representing source documentation might have no executable IR mapping.

That is valid if explicitly classified as:

NON_EXECUTABLE_METADATA

What is prohibited is silently ignoring it.

---

66. Semantic-to-IR Completeness

Every semantic concept that affects execution must have a representation.

Examples:

type
effect
ownership
resource requirement
capability
ordering
control dependency
quantum operation
measurement
hardware intent
security requirement

If a semantic concept cannot affect execution, it must be explicitly classified as metadata or compile-time-only information.

---

67. Canonical IR Boundaries

Zamani must not have one giant IR that destroys domain ownership.

The architecture is:

domain-neutral semantic model
             │
      canonical IR boundary
             │
   ┌─────────┼──────────┐
   ▼         ▼          ▼
classical  quantum     HDL/
semantic   semantic    hardware
IR         IR          intent

Quantum semantics belong to:

quantum::ir

Hardware realization belongs downstream.

---

68. No Duplicate Quantum IR

The following architecture is prohibited:

AST
 ↓
ZamaniQuantumIR
 ↓
QuantumIR
 ↓
quantum::ir

There must be one canonical quantum semantic boundary.

The general compiler may contain adapters, but adapters are not second authorities.

---

69. Quantum Adapter Contract

If "src/ir_gen.rs" needs to bridge frontend semantics to "quantum::ir", the adapter must:

1. preserve operation identity;
2. preserve operands;
3. preserve parameters;
4. preserve results;
5. preserve attributes;
6. preserve modifiers;
7. preserve effects;
8. preserve capabilities;
9. preserve resource requirements;
10. preserve source provenance;
11. reject unsupported information loss.

---

70. Classical IR

Classical computation may use the general IR where appropriate.

The general IR must support:

- scalar operations;
- control flow;
- memory;
- function calls;
- aggregate values;
- generic data;
- concurrency metadata where applicable;
- resource metadata where applicable.

It must not assume a single backend architecture.

---

71. HDL and Hardware IR

HDL/hardware semantics may require a dedicated downstream representation.

That representation must not be confused with:

general source AST

or:

quantum::ir

Hardware IR must preserve parameterized intent.

---

72. Hybrid IR

Hybrid programs must preserve relationships between:

classical
quantum
hardware
resource
control

For example:

measure
→ classical condition
→ quantum operation

must remain one dependency graph.

A lowering that splits the semantics into independent unrelated programs is not complete.

---

73. Resource and Capability Propagation

If a feature requires:

capability("x")

then its requirement must remain available to the downstream stage responsible for capability resolution.

It must not disappear after semantic analysis.

Likewise:

requires memory >= required_memory

must remain represented until resource feasibility is resolved.

---

74. Requirement Versus Allocation

The verifier must distinguish:

requirement

from:

allocation

Example:

requires qubits >= n

is portable.

Example:

map logical_q0 -> physical_q17

is a target realization.

The canonical semantic IR should preserve the former without forcing the latter.

---

75. Optimization Safety

An optimization pass may change representation.

It must not change semantics.

For each optimization, the IR coverage system must know:

input IR contract
output IR contract
preserved properties
removed properties
introduced properties
proof/evidence strategy

An optimization that discards unmodeled metadata is not production-safe.

---

76. Optimization and Quantum Semantics

Quantum optimizers must preserve:

- measurement ordering;
- entanglement dependencies;
- classical feed-forward;
- observable semantics;
- noise/resilience constraints;
- resource requirements;
- effects.

A mathematically equivalent transformation is not automatically valid if it violates explicit source-level constraints.

---

77. Optimization and Hardware Intent

Optimization may transform:

portable intent

into:

better target-independent representation

but must not prematurely select physical hardware unless the current compiler stage explicitly owns target realization.

---

78. Backend Boundary

The backend boundary is where target-specific facts may become concrete.

Examples:

target triple
data layout
register allocation
instruction set
physical topology
physical qubit
GPU architecture
FPGA fabric
ASIC primitive
accelerator instruction

These must not leak backwards into canonical source semantics.

---

79. Current "IrModule" Target Data

Because the current "IrModule::new" initializes:

target_triple = "x86_64-pc-linux-gnu"

and a concrete data layout, this must be treated as:

TARGET-COUPLED IR

rather than universal semantic IR.

Production correction:

canonical semantic IR
        ↓
target lowering context
        ↓
target-specific backend module

The constructor of a canonical semantic module must not silently select x86-64.

---

80. Backend-Specific Module

A backend module may legitimately contain:

target triple
data layout
ABI
calling convention
instruction set
address-space layout

but it must be a separate representation or explicit lowering stage.

The canonical IR coverage specification does not prohibit backend-specific IR.

It prohibits backend-specific information from becoming universal semantic truth.

---

81. IR Serialization

If IR serialization is supported, serialized IR must preserve:

- semantic identity;
- types;
- operation identity;
- attributes;
- provenance;
- capabilities;
- resources;
- effects;
- version;
- compatibility metadata.

Serialization must not depend on host pointer addresses or process-local identities.

---

82. IR Versioning

Canonical IR must have an explicit versioning strategy.

A version change must state whether it is:

compatible
conditionally compatible
migration required
breaking

A source-language version must not automatically imply an identical IR version.

---

83. IR Compatibility

Backward compatibility must be tested at:

source
AST
semantic
IR
serialized IR
backend

where applicable.

A grammar feature may remain source-compatible while its internal IR representation changes.

If so, a migration or adapter must preserve semantics.

---

84. IR Hashing and Reproducibility

If canonical IR is hashed for:

- caching;
- provenance;
- reproducible builds;
- signing;
- artifact identity;

then the hash must be based on canonical semantic representation.

It must not depend on:

- memory addresses;
- hash-map iteration order;
- machine-specific defaults;
- host architecture;
- random IDs;
- timestamps unless explicitly part of the artifact identity.

---

85. Diagnostics and Source Identity

An IR error must be able to identify:

feature
source span
semantic entity
IR entity

where applicable.

A generic:

IR error

without traceability is insufficient for production tooling.

---

86. Negative Coverage

Every feature must have tests proving that invalid lowering is rejected.

Examples:

invalid type
invalid conversion
invalid branch
undefined register
duplicate definition
invalid phi
invalid call
invalid memory operation
invalid quantum lowering
missing resource requirement
lost capability
unsupported operation
semantic information loss

---

87. Boundary Coverage

Boundary tests must include:

empty function where illegal
single instruction
single block
multiple blocks
nested control flow
large operand list
large function
large module
large type graph
large dependency graph
large quantum operation list
symbolic resource quantities
large numeric literals
dynamic dimensions

No test should establish a maximum that becomes a language limit.

---

88. Scalability Coverage

Scalability tests must test increasing sizes according to available resources.

The test framework must not define:

MAX_IR_SIZE
MAX_AST_SIZE
MAX_QUBITS

as language semantics.

Instead, tests should state:

the implementation successfully handles input size S under environment E

and allow larger S when resources permit.

---

89. Tiny-to-Large Testing

The production suite must cover at least the conceptual progression:

one value
one expression
one statement
one function
one module
many functions
many modules
large dependency graph
large IR
large quantum program
large hybrid program
large hardware design
large distributed program

The exact largest tested size is an implementation benchmark, not a language ceiling.

---

90. Deterministic IR Generation

For identical:

source
compiler version
configuration
language version
dialect set

IR generation must be deterministic unless nondeterminism is explicitly part of program semantics.

Register naming, label naming, ordering and serialization must not depend on:

- hash-map iteration;
- thread scheduling;
- random seeds;
- wall-clock time.

---

91. Safe Rust

The implementation of IR generation and verification must remain safe Rust.

The following are prohibited:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

The compiler should enforce this policy where practical:

#![deny(unsafe_code)]

No IR scalability feature may require "unsafe".

---

92. Rust Version

The repository targets:

Rust 1.97
Rust 1.97.1
Rust 2021

The current "Cargo.toml" contains:

rust-version = "1.97" or "1.97.1"

This is not valid TOML syntax and must be corrected before production validation can claim a verified Rust baseline.

The repository must select one valid value, for example:

rust-version = "1.97.1"

or another explicitly chosen supported minimum.

The IR coverage document does not silently modify "Cargo.toml"; this is an explicit repository integration prerequisite.

---

93. Compiler Safety versus Zamani Language Features

The current AST contains a Zamani:

Statement::Unsafe

This must not be confused with Rust implementation safety.

The repository requirement in this document is:

«The Rust implementation of the compiler, validator, IR generator and verifier uses no Rust "unsafe".»

If Zamani itself intentionally has an "unsafe" language construct, that feature requires a separate semantic and security contract.

It must not cause the compiler implementation to use Rust "unsafe".

---

94. Current "QuantumGate" Status

The existing:

IrInstruction::QuantumGate

must not be automatically classified as canonical quantum IR.

Its production status must be:

LEGACY/BRIDGE UNTIL PROVEN CANONICAL

unless repository evidence establishes otherwise.

The preferred architecture is:

AST
 ↓
semantic quantum operation
 ↓
quantum::ir

rather than:

AST
 ↓
general IrInstruction::QuantumGate
 ↓
another quantum representation

---

95. Current Quantum String-Based Lowering

The current general IR rendering includes a form equivalent to:

@__quantum_rt_<operation>

derived from the operation string.

This is insufficient as the canonical quantum semantic representation.

It may be valid as a backend lowering mechanism, provided that:

- the operation identity has already been validated;
- parameters are preserved;
- operands are preserved;
- effects are preserved;
- capabilities are preserved;
- resource requirements are preserved;
- backend support is explicitly checked.

The runtime symbol name must not become the source-level semantic identity.

---

96. Nano Operations

"NanoOp" must follow the same rule.

It must not be a stringly typed semantic escape hatch.

A production implementation must define:

nano operation identity
parameters
operands
results
effects
capabilities
resources
provenance
canonical lowering

or explicitly classify the feature as incomplete.

---

97. Sankofa Operations

The same applies to:

SankofaRecall
SankofaRemember

Their semantic meaning must be defined.

Comments in emitted backend text are not sufficient IR implementation.

If an operation is intentionally compile-time metadata only, that must be explicitly specified.

---

98. Comment Instructions

"Comment" is metadata.

It must never carry semantic information required for execution.

Therefore:

Comment("requires quantum measurement")

does not satisfy IR coverage.

A semantic requirement must have a machine-readable IR representation.

---

99. Feature Coverage Matrix

Every registered feature must produce a row conceptually equivalent to:

Feature| AST| Semantic| IR| Verify| Lower| Tests| Status
feature-id| yes/no| yes/no| yes/no| yes/no| yes/no| yes/no| derived

The status must be computed from evidence, not manually asserted.

---

100. No Artificial Coverage Percentage

The system must not declare:

IR coverage = 98%

and call the compiler production-ready.

A single missing semantic mapping for a critical feature can be a release blocker regardless of percentage.

Production readiness is therefore gate-based.

---

101. Critical Feature Gates

A production release must have complete IR coverage for all features classified as:

stable
core
required
security-critical
quantum-core
hardware-core
resource-core

Experimental features may remain:

EXPERIMENTAL

provided they cannot silently masquerade as stable.

---

102. Cross-Domain Coverage

The validator must test interactions, not only isolated domains.

Required classes include:

classical + quantum
classical + HDL
classical + hardware
quantum + classical
quantum + hardware
quantum + resources
quantum + resilience
HDL + hardware
AI + tensor
AI + accelerator
distributed + networking
distributed + resources
security + networking
memory + accelerator
concurrency + distributed

---

103. Hybrid Quantum/Classical Example

A complete IR trace for:

allocate quantum state
measure
store result
branch on result
apply quantum operation

must preserve:

quantum allocation
measurement
classical result
control dependency
quantum operation

No stage may flatten the measurement into an ordinary integer load.

---

104. Hardware/Software Co-Design Example

A source construct describing:

algorithm
accelerator intent
memory requirement
communication
verification property

must remain distinguishable at semantic IR.

The compiler may later lower it to:

CPU
GPU
FPGA
ASIC
accelerator

without changing the source semantics.

---

105. Distributed Example

A source program describing:

parallel computation
partition
replication
communication
fault tolerance

must not be lowered to a fixed number of nodes.

The downstream deployment system resolves actual placement.

---

106. Resource Negotiation

When source semantics contain:

requires ...
prefers ...
allows ...
hints ...

the IR must preserve their category.

A preference must not become a requirement.

A hint must not become a semantic guarantee.

A resource requirement must not become a physical allocation.

---

107. IR Coverage of Requirements

For every resource requirement:

source
 ↓
AST
 ↓
semantic requirement
 ↓
IR requirement
 ↓
resource analysis

must exist.

If the requirement is intentionally consumed before IR, the contract must explicitly state why and prove that no downstream consumer needs it.

---

108. IR Coverage of Capabilities

For every capability:

source capability
 ↓
semantic capability
 ↓
IR capability requirement
 ↓
target capability negotiation

must be traceable.

---

109. IR Coverage of Constraints

Constraints such as:

latency
memory
precision
reliability
topology
ordering

must not disappear.

If a constraint is compile-time-only, its consumption stage must be documented.

---

110. IR Coverage of Preferences

Preferences may legally be consumed during optimization.

However, their consumption must be deterministic and traceable.

A preference must never be treated as an absolute semantic requirement without an explicit semantic rule.

---

111. Error Recovery

IR generation must not manufacture fake valid IR after an unrecoverable semantic error merely to make a pipeline appear complete.

The correct flow is:

semantic failure
      ↓
diagnostic
      ↓
IR generation skipped or explicitly partial

If recovery is supported, recovered IR must be marked as diagnostic/recovery IR and must not be treated as production-valid IR.

---

112. No Placeholder IR

The following are not acceptable as production implementation:

NoOp
Comment
dummy register
default zero
empty operation
placeholder string

when used to silently replace a semantic construct.

Placeholders may exist during development but must be classified:

DEVELOPMENT_ONLY

and must prevent a feature from receiving "IR_COVERED".

---

113. Dummy Values

The current IR generator contains patterns where fallback values may be produced when errors occur.

Production rules:

«A fallback value must never cause an invalid or unsupported construct to appear valid.»

A fallback register or literal used solely to continue diagnostic collection must carry an error state that prevents successful IR validation.

---

114. Error-State Propagation

IR generation should conceptually return:

Valid IR

or:

IR + diagnostics

but it must never turn:

error

into:

valid semantics

The verifier must therefore be able to reject IR produced from unrecoverable generation errors.

---

115. Canonical IR Verification Contract

"src/ir_verify.rs" must eventually verify at least:

Module

- unique symbols;
- valid globals;
- valid functions;
- valid module metadata;
- version;
- semantic/target boundary.

Function

- unique parameters;
- valid parameter types;
- valid blocks;
- valid control flow;
- valid terminators;
- valid return type.

SSA

- single definitions;
- valid uses;
- dominance;
- phi predecessor correspondence;
- type consistency.

Memory

- pointer validity;
- load/store compatibility;
- aggregate traversal;
- address-space correctness.

Calls

- function existence;
- signature;
- arguments;
- return type;
- calling convention.

Arithmetic

- operation/type legality;
- signedness;
- overflow semantics.

Conversion

- legal source/destination categories;
- representation compatibility.

Quantum

- canonical quantum boundary;
- operation identity;
- operands;
- parameters;
- semantic metadata.

Resources

- requirements;
- capabilities;
- constraints;
- preferences.

Provenance

- source identity where required.

---

116. CFG Dominance

Because the current IR contains SSA "Phi", production verification must eventually establish dominance relationships.

A use of a register must be dominated by its definition unless it is a valid phi incoming value from the corresponding predecessor.

This must be verified without imposing artificial CFG-size limits.

---

117. Phi Validation

A phi node must reference actual incoming CFG edges.

It is insufficient to check only:

label exists

The verifier must check:

predecessor block
      ↓
terminator edge
      ↓
phi block

and ensure the incoming value is valid for that edge.

---

118. Unreachable Code

Unreachable code must be explicitly represented as unreachable.

The verifier should report unreachable blocks separately from malformed blocks.

Optimizers may remove unreachable code later.

IR verification must not silently interpret unreachable code as executable.

---

119. Function Entry

Every non-external function must have a well-defined entry block.

The entry block must not require an incoming predecessor.

All other reachable blocks must be reachable from the entry.

---

120. External Functions

External functions require explicit declarations.

Their signatures must be available to call verification.

Backend-specific ABI details remain downstream unless part of the external declaration contract.

---

121. Calling Conventions

Calling conventions are semantic only when the language explicitly exposes them.

Otherwise:

calling convention
ABI
register assignment
stack layout

belong downstream.

The IR coverage system must not hard-code one ABI as universal semantics.

---

122. Target Data Layout

Target data layout belongs to backend lowering.

The canonical IR must not assume:

x86_64
Linux
specific pointer width
specific alignment
specific vector width

unless the IR itself is explicitly target-specific.

---

123. Target-Specific IR

Target-specific IR is allowed.

It must have an explicit identity such as:

LLVM backend IR
CUDA backend IR
SPIR-V backend IR
FPGA synthesis IR
QPU target IR

and must not be mistaken for the canonical semantic IR.

---

124. Canonical Semantic IR

The canonical semantic IR must remain:

target independent
vendor independent
hardware independent
resource-limit independent
deterministic
versioned
verifiable
traceable

---

125. IR Lowering Contracts

Each lowering pass must document:

Input IR
Output IR
Preserved semantics
Consumed metadata
Introduced metadata
Target assumptions
Failure modes
Diagnostics
Tests

A pass without this contract cannot be considered production-complete.

---

126. Pass Ordering

The compiler pipeline must not perform target-specific transformations before required semantic validation.

The conceptual order is:

semantic validation
      ↓
canonical IR
      ↓
IR verification
      ↓
target-independent optimization
      ↓
target capability analysis
      ↓
target-dependent lowering
      ↓
routing / scheduling / resilience
      ↓
backend

Quantum compilation may specialize the ordering of routing, scheduling, QEC and ZQN according to their established subsystem contracts, but the ownership boundaries must remain explicit.

---

127. IR Coverage and QEC

The IR coverage validator must verify that all information required by QEC remains available.

Examples:

logical qubit identity
operation sequence
measurement
noise model
error budget
fault-tolerance requirements

must not be discarded before QEC analysis.

---

128. IR Coverage and Routing

Routing needs:

logical operands
interaction relationships
topology requirements
connectivity constraints

The canonical quantum IR should provide semantic information.

The routing subsystem determines physical placement.

---

129. IR Coverage and Scheduling

Scheduling requires:

dependencies
ordering
duration information where semantically available
resource conflicts
synchronization

The IR must preserve these where required.

The scheduler determines actual execution order.

---

130. IR Coverage and HAL

HAL consumes target-specific information.

The canonical IR must not depend on a particular HAL implementation.

---

131. Backend Failure

If a backend cannot lower a valid canonical IR feature, the result is:

TARGET_UNSUPPORTED

or:

TARGET_INCOMPATIBLE

not:

invalid source

unless the source semantic contract itself was invalid.

---

132. Compatibility

IR coverage must be checked against:

grammar/compatibility/
grammar/spec/compatibility.md

A source-compatible feature must continue to produce semantically compatible IR unless a documented migration exists.

---

133. Historical Features

Features appearing only in:

grammar/Zamani-Grammar.md

must not receive "IR_COVERED" status unless their implementation evidence exists.

Historical/proposed syntax is not automatically an IR requirement.

---

134. "grammar/grammar.md"

"grammar/grammar.md" must distinguish:

specified
lexically implemented
parser implemented
AST implemented
semantic implemented
IR implemented
backend implemented
tested
stable
experimental
planned
deprecated

IR coverage must not be inferred merely from parser acceptance.

---

135. Validation Dependency Graph

The IR validator depends on:

specification
    ↓
AST coverage
    ↓
semantic coverage
    ↓
IR coverage

Therefore:

IR_COVERED

cannot be asserted when:

AST mapping

or:

semantic mapping

is absent.

---

136. Required Companion Validation Documents

This file integrates with:

grammar/validation/grammar-validator.md
grammar/validation/ast-coverage.md
grammar/validation/semantic-coverage.md
grammar/validation/source-spans.md
grammar/validation/determinism.md
grammar/validation/compatibility-rules.md
grammar/validation/ambiguity.md
grammar/validation/duplicate-tokens.md

Where an existing filename differs, the existing authoritative filename must be retained rather than unnecessarily renamed.

---

137. Validation Evidence

Every "IR_COVERED" feature must provide evidence for:

specification
grammar
AST
semantic
IR
verification
lowering
tests

Evidence must identify:

repository path
feature identifier
rule/node/instruction
test identifier
status

---

138. Evidence Must Be Current

Evidence must be associated with a repository revision, generated report, or equivalent reproducible validation context.

Stale evidence must not be treated as current.

---

139. No Manual "Covered" Flags

A feature must not be manually marked:

IR_COVERED

if the implementation evidence fails.

Status should be derived from validation results.

Manual declarations may identify intended status but cannot override evidence.

---

140. Coverage Failure Severity

Recommended severity levels:

BLOCKER
ERROR
WARNING
INFO

BLOCKER

Examples:

- semantic loss;
- duplicate canonical IR;
- fixed target in canonical IR;
- missing required quantum mapping;
- invalid SSA;
- invalid call signature;
- invalid type representation;
- silent truncation;
- missing critical resource semantics.

ERROR

Feature cannot be considered covered.

WARNING

Feature remains valid but has non-critical evidence limitations.

INFO

Informational status.

---

141. Production Blockers in the Current Repository

Based on the current implementation, the following must be resolved before claiming full production IR coverage:

1. Remove the universal x86-64 target assumption from canonical IR construction.

2. Establish a target-neutral canonical IR module model.

3. Resolve lossless integer representation.

4. Resolve dynamic/symbolic array representation.

5. Complete call-signature verification.

6. Complete CFG/SSA verification.

7. Complete phi predecessor validation.

8. Complete conversion legality validation.

9. Complete memory-operation type validation.

10. Establish explicit quantum-to-"quantum::ir" mapping.

11. Prevent "QuantumGate" from becoming a duplicate quantum IR.

12. Establish semantic contracts for "NanoOp".

13. Establish semantic contracts for "SankofaRecall" and "SankofaRemember".

14. Preserve resource/capability semantics through IR.

15. Preserve source provenance.

16. Establish deterministic IR identity and ordering.

17. Establish canonical IR versioning.

18. Correct the invalid "rust-version" entry in "Cargo.toml".

19. Ensure the IR verifier and generator compile on Rust 1.97/1.97.1.

20. Enforce no Rust "unsafe".

These are release gates, not optional documentation improvements.

---

142. Production IR Coverage Tests

The repository must eventually contain tests for at least:

tests/ir/
tests/ir/core/
tests/ir/types/
tests/ir/control-flow/
tests/ir/memory/
tests/ir/functions/
tests/ir/classical/
tests/ir/quantum/
tests/ir/hybrid/
tests/ir/hdl/
tests/ir/hardware/
tests/ir/resources/
tests/ir/distributed/
tests/ir/ai/
tests/ir/security/
tests/ir/interoperability/
tests/ir/negative/
tests/ir/boundary/
tests/ir/scalability/
tests/ir/determinism/
tests/ir/compatibility/

These directories must only be added where the repository's existing test organization requires them; existing files should not be unnecessarily renamed.

---

143. Required Core IR Tests

At minimum:

valid_empty_external_function
valid_simple_function
valid_return
valid_branch
valid_conditional_branch
valid_phi
valid_call
valid_indirect_call
valid_memory_operation
valid_conversion
valid_aggregate_access

and negative cases:

duplicate_function
duplicate_global
duplicate_parameter
duplicate_register
undefined_register
undefined_label
invalid_return_type
invalid_branch_condition
invalid_phi_predecessor
invalid_phi_type
invalid_call_signature
invalid_conversion
invalid_memory_access
instruction_after_terminator
missing_terminator

---

144. Quantum IR Tests

At minimum:

generic_quantum_operation
custom_quantum_operation
parameterized_operation
qualified_operation
multi-operand_operation
measurement
mid-circuit_measurement
classical_feedforward
resource_requirement
capability_requirement
logical_qubit
physical_mapping_boundary
noise_requirement
resilience_requirement

The tests must demonstrate that arbitrary valid operation names are not restricted to a fixed gate enum.

---

145. Quantum Scalability Tests

The suite must test:

one qubit
small register
parameterized register
large symbolic register
many operations
large operation parameters
large circuit

The test suite must not define a maximum qubit count as language semantics.

---

146. HDL IR Tests

At minimum:

module
parameterized width
port
signal
combinational logic
sequential logic
clock
reset
state machine
memory
generate
timing intent
verification property

Widths must be source semantics when explicitly supplied, not compiler defaults.

---

147. Resource IR Tests

Test:

requirement
capability
constraint
preference
hint
budget
realization boundary

and ensure they remain distinguishable.

---

148. Cross-Domain Tests

At minimum:

classical → quantum
quantum → classical
software → accelerator
HDL → hardware
AI → tensor
distributed → resource
security → networking
concurrency → distributed

---

149. Semantic Preservation Tests

For every lowering:

source semantic model

must be compared with:

IR semantic model

The test must verify that required properties are preserved.

This may use:

- structural comparison;
- canonical serialization;
- semantic fingerprints;
- explicit property checks;
- interpreter/reference execution where appropriate.

The chosen method must be deterministic.

---

150. Round-Trip Tests

Where serialization exists:

IR
 ↓
serialize
 ↓
deserialize
 ↓
IR

must preserve canonical semantics.

A serialized/deserialized IR must remain equivalent.

---

151. Reproducibility Tests

Given identical input:

source
configuration
toolchain
feature registry

the generated canonical IR must be identical or canonically equivalent according to the IR specification.

---

152. Target Independence Tests

Compile the same canonical semantic IR against multiple backend profiles.

The test must establish that:

canonical IR

does not change merely because the target changes.

Target-specific lowering may differ.

---

153. Resource Independence Tests

A valid program must retain the same semantic IR when compiled under different resource availability, provided the semantic requirements themselves do not change.

Example:

available QPU A

versus:

available QPU B

must not cause the source semantic meaning to change.

---

154. Target Capability Tests

A target lacking a capability must produce:

TARGET_INCOMPATIBLE

or equivalent.

It must not produce:

INVALID_PROGRAM

when the program is semantically valid.

---

155. Resource Exhaustion Tests

Tests must distinguish:

invalid semantic IR

from:

insufficient compiler resources

and:

insufficient target resources

---

156. No Silent Truncation Tests

Explicit tests must prove that:

large integer
large dimension
large resource quantity
large tensor shape

cannot be silently truncated merely because a backend representation is narrower.

---

157. No Hard-Coded Ceiling Tests

Repository validation must scan for universal limits including:

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

The audit must distinguish:

program constant

from:

implementation ceiling

Only the latter is prohibited.

---

158. Safe-Rust Audit

The production CI must reject Rust implementation code containing prohibited unsafe constructs.

The audit must cover at least:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe {

Generated third-party code may be governed by dependency policy, but Zamani's own production implementation must remain safe Rust.

---

159. Rust 1.97 Compatibility

The CI matrix must verify:

cargo +1.97 check
cargo +1.97 test

and:

cargo +1.97.1 check
cargo +1.97.1 test

when both are maintained as supported baselines.

The exact project MSRV policy must be declared consistently in "Cargo.toml" and repository documentation.

---

160. Clippy and Formatting

Production validation should include:

cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features

using the supported Rust baseline.

No unsafe code may be introduced to satisfy lint or performance requirements.

---

161. IR Coverage Report

The validator must generate a deterministic report containing:

repository revision
compiler version
Rust version
language version
feature registry version
IR version
feature status
AST mapping
semantic mapping
IR mapping
verification status
downstream status
test status
compatibility status
scalability status
determinism status
diagnostics
blockers

---

162. Machine-Readable Output

The validator should provide a machine-readable form such as:

JSON

with stable schema versioning.

Human-readable Markdown remains useful but must not be the only machine-consumable evidence.

---

163. Human-Readable Output

The report must provide:

feature
status
missing stage
failure reason
source location
recommended owning file

without requiring developers to inspect compiler internals manually.

---

164. Independent File Completion Contract

This file is complete independently when it defines:

- what IR coverage means;
- what it owns;
- what it does not own;
- authority;
- statuses;
- traceability;
- semantic preservation;
- quantum boundary;
- hardware boundary;
- resource boundary;
- verification requirements;
- scalability requirements;
- determinism requirements;
- compatibility requirements;
- test requirements;
- production gates;
- current repository blockers.

It must not need to be rewritten merely because another grammar or implementation file is subsequently expanded.

Other files must integrate to this contract, not redefine it.

---

165. Integration Contract for "src/ir_gen.rs"

"src/ir_gen.rs" owns:

AST/semantic → current IR construction

It must not own:

hardware selection
routing
scheduling
QEC
HAL
target discovery

It must:

1. map every supported AST construct;
2. preserve required semantic information;
3. reject unsupported lowering;
4. avoid silent placeholders;
5. preserve provenance;
6. preserve resource/capability metadata;
7. delegate canonical quantum semantics to "quantum::ir";
8. remain safe Rust;
9. remain Rust 1.97/1.97.1 compatible.

---

166. Integration Contract for "src/ir_verify.rs"

"src/ir_verify.rs" owns:

structural and type-level IR verification

It must:

1. verify every instruction variant;
2. verify every type combination;
3. verify CFG;
4. verify SSA;
5. verify calls;
6. verify memory operations;
7. verify conversions;
8. verify module symbols;
9. verify canonical metadata;
10. verify quantum boundary invariants;
11. return deterministic diagnostics;
12. avoid target-specific assumptions.

It must not perform target routing or hardware discovery.

---

167. Integration Contract for "src/ast/mod.rs"

"src/ast/mod.rs" owns source representation.

Each AST variant must have an IR disposition.

Adding an AST variant without an IR coverage entry is incomplete for executable constructs.

The AST must not contain backend-specific implementation decisions merely to simplify IR generation.

---

168. Integration Contract for "src/semantic.rs"

"src/semantic.rs" owns semantic validity.

It must establish:

type
scope
name resolution
resource semantics
capability semantics
effect semantics
quantum semantics
domain semantics

where applicable.

IR generation must not invent semantic rules that semantic analysis failed to define.

---

169. Integration Contract for "src/quantum/ir/"

This remains the canonical quantum semantic IR.

The module must remain:

hardware independent
vendor independent
routing independent
scheduling independent
QEC implementation independent
HAL independent

It owns the semantic meaning of quantum computation.

---

170. Integration Contract for "grammar/Zamani.g4"

"Zamani.g4" owns grammar composition.

It must not:

- define IR;
- define hardware resources;
- define target selection;
- enumerate physical qubits;
- define routing;
- define scheduling;
- define QEC implementation.

---

171. Integration Contract for "grammar/grammar.md"

"grammar.md" reports actual implementation conformance.

It must not independently claim:

IR_COVERED

without evidence from this contract and actual implementation.

---

172. Integration Contract for "grammar/Zamani-Grammar.md"

This document remains historical/extended design material.

Features appearing there do not become IR requirements until promoted through the repository's feature lifecycle.

---

173. Integration Contract for "grammar/specification/"

The normative specification defines semantics.

IR coverage consumes those definitions.

If the specification is ambiguous, IR coverage must report:

SEMANTIC_UNDERSPECIFIED

rather than inventing an IR meaning.

---

174. Integration Contract for Tests

Tests are evidence.

Tests do not redefine semantics.

A passing test cannot legitimize a representation that contradicts the specification.

---

175. Production Feature Lifecycle

A feature progresses:

PROPOSED
   ↓
SPECIFIED
   ↓
GRAMMAR
   ↓
AST
   ↓
SEMANTIC
   ↓
IR_PARTIAL
   ↓
IR_COVERED
   ↓
DOWNSTREAM_COVERED
   ↓
STABLE

A feature may instead become:

DEPRECATED
UNSUPPORTED
HISTORICAL

without passing through full implementation.

---

176. Required Definition of Done for an IR Feature

A feature is done only when:

[ ] specification exists
[ ] grammar mapping exists
[ ] lexer mapping exists where required
[ ] parser mapping exists
[ ] AST mapping exists
[ ] semantic mapping exists
[ ] canonical IR mapping exists
[ ] all semantic information is preserved
[ ] IR verifier understands it
[ ] source provenance is preserved
[ ] diagnostics exist
[ ] downstream consumer is identified
[ ] lowering contract exists
[ ] positive tests exist
[ ] negative tests exist
[ ] boundary tests exist
[ ] scalability tests exist
[ ] determinism tests exist
[ ] compatibility tests exist
[ ] hard-coding audit passes
[ ] safe-Rust audit passes
[ ] Rust 1.97/1.97.1 compatibility passes

Only then may the feature receive:

IR_COVERED

---

177. Production Readiness Gate

The IR subsystem is production-ready only when all mandatory stable features satisfy:

Specification       PASS
Grammar             PASS
Lexer               PASS
Parser              PASS
AST                 PASS
Semantic             PASS
IR mapping           PASS
IR verification      PASS
Semantic preservation PASS
Source provenance    PASS
Downstream contract  PASS
Tests                PASS
Determinism          PASS
Scalability          PASS
Compatibility        PASS
Hard-coding audit    PASS
Safety audit         PASS
Rust baseline        PASS

A single blocker means:

NOT PRODUCTION READY

No percentage may override a blocker.

---

178. Production Readiness Is Not Target Availability

A valid IR implementation may be production-ready even when:

a particular GPU
a particular FPGA
a particular QPU
a particular accelerator

is unavailable.

Target availability is external runtime/deployment state.

The compiler must distinguish:

language correctness
IR correctness
target capability
resource feasibility

---

179. Production Readiness Is Not Infinite Hardware

POCO-REAF means:

no artificial machine-size ceiling

It does not promise infinite physical resources.

The compiler must therefore remain honest:

unbounded language model
+
finite concrete execution resources

is the required architecture.

---

180. Final Canonical Pipeline

The final production architecture is:

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
                     src/ast/
                          │
                          ▼
                  Semantic Analysis
                          │
                          ▼
             Canonical Semantic Model
                          │
              ┌───────────┼───────────┐
              │           │           │
              ▼           ▼           ▼
          Classical   quantum::ir   HDL/Hardware
              │           │           │
              └───────────┼───────────┘
                          │
                          ▼
                    IR Verification
                          │
                          ▼
                Target-independent
                    optimization
                          │
             ┌────────────┼────────────┐
             │            │            │
             ▼            ▼            ▼
          resource      capability    effect
          analysis      analysis      analysis
             │            │            │
             └────────────┼────────────┘
                          │
                          ▼
                 Target capability
                      analysis
                          │
             ┌────────────┼─────────────┐
             ▼            ▼             ▼
          routing      scheduling    resilience
             │            │             │
             └────────────┼─────────────┘
                          │
                         QEC
                          │
                         ZQN
                          │
                         HAL
                          │
                          ▼
                  Target realization
                          │
          ┌───────────────┼────────────────┐
          ▼               ▼                ▼
         CPU             GPU              FPGA
          │               │                │
          ▼               ▼                ▼
         ASIC            QPU          Accelerator
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                    Future targets

---

181. Non-Negotiable Architectural Rules

The following are absolute:

1. "src/ast/" remains the current AST authority unless explicitly migrated.

2. "src/quantum/ir/" remains the canonical quantum semantic IR boundary.

3. No second quantum frontend IR may become authoritative.

4. "src/ir_gen.rs" must not encode target selection into canonical semantic IR.

5. "src/ir_verify.rs" must verify the complete instruction/type/control-flow contract.

6. No semantic information may be silently discarded.

7. No fixed hardware maximum may become a language or canonical IR limit.

8. Resource requirements must remain distinct from physical allocations.

9. Capability requirements must remain distinct from target realization.

10. Quantum logical resources must remain distinct from physical resources until downstream realization.

11. Target triples and data layouts belong to target-specific lowering, not universal semantic IR.

12. Arbitrary program values must not be silently truncated to machine-sized representations.

13. Dynamic/symbolic resource and data dimensions must not be converted to placeholder fixed sizes.

14. IR placeholders must not masquerade as implemented features.

15. IR coverage must be evidence-based.

16. Coverage percentages must not override production blockers.

17. Validation must be deterministic.

18. Validation must distinguish invalid input from validator resource exhaustion.

19. Compiler/validator implementation must use safe Rust only.

20. Rust 1.97/1.97.1 compatibility must be continuously tested.

21. Existing filenames must not be unnecessarily renamed.

22. Existing domain directories must be populated and integrated rather than replaced by parallel hierarchies.

23. "Zamani-Grammar.md" must not silently become executable language authority.

24. "grammar.md" must report actual implementation status rather than aspirational syntax.

25. Every stable executable feature must have a complete specification → AST → semantic → IR → verification → downstream → test chain.

---

182. Completion Statement

"grammar/validation/ir-coverage.md" is considered complete as an independent specification when it establishes the contract above.

Its implementation dependencies are deliberately downstream:

this specification
       ↓
feature manifests
       ↓
AST coverage
       ↓
semantic coverage
       ↓
IR coverage implementation
       ↓
IR verifier
       ↓
compiler integration
       ↓
tests

The document itself must not require redefinition when another feature is added.

New features integrate by satisfying this contract.

New IR instructions integrate by declaring their semantic owner, type rules, verifier rules, provenance rules, lowering rules, downstream consumer and tests.

New targets integrate downstream of canonical semantic IR.

New quantum operations integrate through "quantum::ir".

New hardware integrates through capability/resource/target realization.

New future computational domains integrate through the same universal semantic and IR coverage model.

The resulting invariant is:

                         ONE LANGUAGE
                              │
                              ▼
                     ONE SEMANTIC MODEL
                              │
                              ▼
                    CANONICAL IR BOUNDARIES
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
         classical        quantum::ir       HDL/hardware
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                    TARGET-INDEPENDENT
                         MEANING
                              │
                              ▼
                    TARGET REALIZATION
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
            tiny            large            future
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                         POCO-REAF

The fundamental guarantee is therefore not:

«"The compiler supports today's largest machine."»

It is:

«The language and canonical IR contain no artificial machine-size ceiling, preserve program semantics independently of target size, and allow concrete compilation and execution to scale according to the actual resources and capabilities available at realization time.»

That is the required IR foundation for production Zamani and for "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever".