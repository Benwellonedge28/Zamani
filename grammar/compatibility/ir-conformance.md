Zamani IR Conformance

Path: "grammar/compatibility/ir-conformance.md"
Language: Zamani
Repository: "github.com/Benwellonedge28/Zamani"
Status: Normative production IR-conformance contract
Rust baseline: Rust 1.97.1
Rust edition: 2021
Rust safety: Safe Rust only; production Zamani compiler code MUST NOT use Rust "unsafe"
Primary objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)

---

1. Purpose

This document defines the production conformance contract between Zamani source-language meaning and the canonical Intermediate Representation used by the compiler.

It closes the compatibility boundary:

Zamani specification
        ↓
grammar/Zamani.g4
        ↓
lexer
        ↓
parser
        ↓
frontend AST
        ↓
semantic analysis
        ↓
canonical semantic representation
        ↓
canonical IR
        ↓
optimization / lowering
        ↓
routing / scheduling / resilience / QEC / ZQN
        ↓
HAL / backend
        ↓
runtime
        ↓
target realization

This document specifically owns the source-semantics → canonical-IR conformance boundary.

It defines:

- what IR conformance means;
- which representation is canonical;
- IR ownership;
- AST-to-IR requirements;
- semantic-to-IR requirements;
- classical IR requirements;
- quantum IR requirements;
- HDL/hardware IR requirements;
- hybrid IR requirements;
- resource/capability IR requirements;
- effects and concurrency;
- distributed computation;
- AI/data computation;
- interoperability;
- dialects;
- versioning;
- compatibility;
- canonicalization;
- deterministic serialization;
- hashing;
- provenance;
- source mapping;
- diagnostics;
- validation;
- resource-policy separation;
- scalability;
- target independence;
- optimization legality;
- routing/scheduling boundaries;
- QEC/ZQN boundaries;
- HAL boundaries;
- tests;
- negative tests;
- boundary tests;
- scalability tests;
- hard-coding audits;
- production completion criteria.

This document does not define:

- backend algorithms;
- physical hardware;
- physical topology;
- device calibration;
- QEC algorithms;
- routing algorithms;
- scheduling algorithms;
- compiler optimization strategies;
- runtime implementation;
- simulator implementation;
- target-specific machine instructions.

Those remain owned by their respective subsystems.

---

2. Repository Authority

The IR contract is subordinate to the existing Zamani architecture.

The authority chain is:

grammar/DESIGN.md
        ↓
grammar/specification/
        ↓
grammar/spec/
        ↓
grammar/Zamani.g4
        ↓
lexer / parser
        ↓
src/ast/
        ↓
semantic analysis
        ↓
canonical semantic model
        ↓
canonical IR

The relevant repository authorities remain:

File/subsystem| Authority
"grammar/DESIGN.md"| Grammar architecture and architectural invariants
"grammar/specification/"| Normative language specification
"grammar/spec/"| Feature contracts
"grammar/Zamani.g4"| Canonical ANTLR composition root
"grammar/grammar.md"| Implementation-conformance reference
"grammar/Zamani-Grammar.md"| Historical/proposed/experimental extended design
"grammar/compatibility/versions.md"| Language-version compatibility policy
"grammar/compatibility/migrations.md"| Migration procedures
"grammar/compatibility/deprecated.md"| Deprecation lifecycle
"grammar/compatibility/compatibility-matrix.md"| Cross-layer compatibility matrix
"grammar/compatibility/ast-conformance.md"| AST boundary
"grammar/compatibility/frontend-conformance.md"| Frontend pipeline
"grammar/compatibility/ir-conformance.md"| IR boundary defined by this document
"src/ast/"| Executable AST implementation
semantic analysis| Meaning and validity
"src/quantum/ir/"| Canonical quantum IR
compiler/lowering| IR transformation and target preparation
routing| Physical/logical realization
scheduling| Temporal/resource realization
resilience/QEC/ZQN| Fault/noise/resilience processing
HAL/backend| Target realization
runtime| Execution

No downstream file may silently redefine the meaning established at an earlier boundary.

---

3. Fundamental IR Principle

The canonical IR is a semantic representation, not a second source language and not a hardware description.

The IR MUST preserve the meaning of a semantically valid Zamani program while removing source-level syntactic ambiguity.

The fundamental invariant is:

Equivalent source meaning
        ↓
Equivalent canonical semantic meaning
        ↓
Equivalent canonical IR meaning

The IR MAY differ structurally when two representations are semantically equivalent.

However:

different IR structure

MUST NOT silently imply:

different program semantics

unless the difference is explicitly part of the semantic contract.

---

4. What the IR Owns

The canonical IR owns:

1. semantic operations;
2. values;
3. types;
4. regions;
5. blocks;
6. control flow;
7. dependencies;
8. data flow;
9. operation operands;
10. operation results;
11. semantic attributes;
12. effects;
13. capabilities;
14. resource requirements;
15. logical quantum operations;
16. classical operations;
17. hardware-independent HDL intent where represented by IR;
18. hybrid classical/quantum relationships;
19. provenance;
20. source mappings;
21. deterministic identity;
22. semantic version metadata;
23. dialect identity;
24. explicit compatibility metadata;
25. validation metadata where required.

---

5. What the IR Does Not Own

The canonical semantic IR MUST NOT silently own:

- physical CPU identifiers;
- physical GPU identifiers;
- physical FPGA locations;
- physical ASIC cells;
- physical QPU identifiers;
- physical qubit assignments;
- calibration values;
- device-specific pulse calibration;
- target-specific routing;
- final scheduling;
- physical memory-bank placement;
- machine topology unless explicitly represented as target-independent requirements;
- current machine inventory;
- runtime process handles;
- operating-system handles;
- allocator addresses;
- compiler memory addresses;
- mutable hardware state.

Those belong downstream.

---

6. Existing Repository Canonical Quantum IR

The actual repository contains:

src/quantum/ir/

with established subsystems including:

analysis/
classical/
compatibility/
control/
core/
dialect/
hashing/
metadata/
model/
program/
pulse/
quantum/
resources/
scheduling/

The existing "src/quantum/ir/mod.rs" explicitly establishes this hierarchy and identifies "quantum::ir" as the canonical quantum boundary.

That architecture MUST be preserved.

In particular:

quantum::ir

MUST remain the canonical quantum semantic IR.

A frontend quantum IR MUST NOT be introduced merely because the frontend has quantum syntax.

The required direction is:

Zamani quantum syntax
        ↓
domain-neutral AST
        ↓
semantic quantum model
        ↓
quantum::ir

not:

Zamani syntax
        ↓
frontend quantum IR
        ↓
another quantum IR
        ↓
quantum::ir

---

7. No Duplicate Quantum IR

The following are prohibited:

frontend::quantum::ir
compiler::quantum::ir
grammar::quantum::ir
parser::quantum::ir
dialect::quantum::ir

when any of them claims to be another canonical semantic representation.

Domain-specific intermediate structures MAY exist temporarily as implementation details, but they MUST:

1. have an explicit owner;
2. have a documented lifetime;
3. have a deterministic lowering path;
4. not claim canonical semantic authority;
5. not duplicate the public "quantum::ir" contract.

---

8. Canonical Quantum Boundary

The canonical quantum boundary is:

src/quantum/ir/

The parent module already provides compatibility re-exports while preserving canonical ownership.

Existing compatibility paths MUST remain aliases/re-exports rather than duplicate implementations.

Examples include canonical ownership for:

QubitId
PhysicalQubitId
Gate
Measurement
QuantumCircuit
QuantumProgram
QuantumIrLimits

The compatibility layer MUST NOT create structurally equivalent but nominally different types.

---

9. Qubit Identity

The repository explicitly establishes:

quantum::ir::qubit::QubitId
quantum::ir::qubit::PhysicalQubitId

as the authoritative logical and physical identity types.

IR conformance therefore requires:

logical identity
    ≠
physical identity

at the semantic level.

A logical qubit may later be mapped to a physical qubit.

The mapping MUST NOT be silently embedded into the source-level meaning.

Example:

logical q0

may become:

physical qubit 17

after routing.

That transformation is target realization, not source-language semantics.

---

10. Generic Operation Model

The IR MUST support extensible operations.

The semantic shape is conceptually:

Operation {
    identity
    name
    namespace
    operands
    results
    parameters
    attributes
    modifiers
    effects
    capabilities
    resource_requirements
    regions
    provenance
}

The IR MUST NOT require every operation to be known when the compiler is built.

This is essential for:

- future quantum operations;
- vendor-independent operations;
- new accelerators;
- future hardware;
- AI operations;
- tensor operations;
- new mathematical operations;
- HDL constructs;
- new dialects.

---

11. Closed Enumeration Prohibition

The IR MUST NOT make an extensible semantic domain dependent on a permanently closed enumeration.

In particular, quantum semantics MUST NOT require:

H
X
Y
Z
CNOT
SWAP
...

to be the complete universe of operations.

Known operations MAY have optimized representations.

Unknown-but-valid operations MUST remain representable through the extensible operation model.

---

12. Quantum Operation Conformance

A quantum operation MUST preserve:

- operation name;
- namespace;
- operands;
- targets;
- controls;
- parameters;
- conditions;
- results;
- modifiers;
- attributes;
- effects;
- capability requirements;
- resource requirements;
- source provenance.

For example:

apply custom_gate

MUST remain representable even when the compiler does not have a dedicated enum member for "custom_gate".

Likewise:

apply vendor.operation(...)

MUST not require a new compiler release merely because a new vendor operation exists, provided the semantic operation model supports it.

---

13. Quantum Semantic Extensibility

The canonical quantum IR MUST be capable of representing, where supported by the semantic specification:

- gate-based computation;
- dynamic circuits;
- measurement;
- reset;
- classical feed-forward;
- controlled operations;
- adjoint operations;
- parameterized operations;
- observables;
- channels;
- noise;
- logical operations;
- error-correction intent;
- pulse intent;
- circuit models;
- analog models;
- Hamiltonian models;
- annealing;
- QUBO;
- tensor-network computation;
- continuous-variable models;
- measurement-based computation;
- distributed quantum computation;
- future quantum paradigms.

Not every target needs to implement every model.

Unsupported target capability MUST produce a target/capability diagnostic rather than changing the IR semantics.

---

14. Classical IR

Classical computation MUST use the same semantic foundation.

The canonical IR MUST be able to represent:

- scalar values;
- integer computation;
- floating-point computation;
- arbitrary structured numeric operations;
- arrays;
- vectors;
- matrices;
- tensors;
- functions;
- calls;
- control flow;
- memory operations;
- data flow;
- concurrency;
- parallelism;
- symbolic computation;
- data processing.

A mathematical function does not automatically require a dedicated IR opcode.

For example:

fft(x)

MAY remain a structured operation/call unless the language specification assigns special semantic guarantees to FFT.

---

15. Mathematical Scalability

The IR MUST NOT establish language-level mathematical limits.

For example:

Matrix<1024, 1024>

is program information.

It MUST NOT imply:

MAX_MATRIX_DIMENSION = 1024

Likewise:

Tensor<T, Shape>

MUST NOT have a universal fixed rank merely because one implementation currently uses a bounded host representation.

---

16. Numeric Literal Preservation

The existing AST implementation contains fixed-width representations such as:

Integer(i64, Span)
Float(f64, Span)

This is a compatibility/scalability concern.

IR conformance requires that the source-to-IR pipeline MUST NOT silently lose numeric information merely because an intermediate implementation uses a fixed-width Rust representation.

The architecture MUST distinguish:

source literal

from:

selected semantic numeric representation

and:

target representation

If a value cannot be represented by a particular compiler implementation or target, the implementation MUST produce a structured diagnostic.

It MUST NOT silently truncate, wrap, or redefine the source value.

---

17. Resource Policies Are Not Language Limits

The actual repository contains "QuantumIrLimits".

That module explicitly defines resource limits as policy, not as the maximum capability of Zamani.

This distinction MUST be preserved.

Conceptually:

IR semantics
        ≠
compilation policy
        ≠
hardware capacity

For example:

QuantumIrLimits
    = policy for this compilation/validation/security boundary

target capability
    = resources offered by the selected target

program requirement
    = resources the program requires

These three values MUST NOT be conflated.

---

18. Existing QuantumIrLimits

"src/quantum/ir/core/limits.rs" currently defines explicit resource categories such as:

LogicalQubits
ClassicalBits
Registers
Operations
OperandsPerOperation
ParametersPerOperation
Parameters
ExpressionNodes
Regions
Blocks
Values
Symbols
Dependencies
ControlFlowDepth
NestingDepth
CircuitDepth
Measurements
Barriers
PulseOperations
WaveformSamples
WaveformBytes
Channels
Frames
ScheduledOperations
MappingEntries
ResourceRequirements
Extensions
Diagnostics
MetadataBytes
SourceBytes
ProgramBytes
ValidationSteps
AnalysisSteps
TransformationSteps
CompilationSteps
ProcessingDepth

These are legitimate resource-policy categories.

They MUST NOT become language-level restrictions.

---

19. "unbounded()" Semantics

Where the existing IR policy provides an unbounded policy, "unbounded()" means:

«no finite application-level policy ceiling is imposed by that policy object.»

It MUST NOT mean:

«infinite physical resources exist.»

Actual limits may still arise from:

- available memory;
- process address space;
- host representation;
- allocator behavior;
- target capacity;
- runtime limits;
- network capacity;
- physical hardware;
- compiler execution time.

Those are external constraints.

---

20. No Universal "MAX_*" IR Limits

The IR contract MUST NOT introduce universal language limits such as:

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
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_TIMELINES
MAX_PROGRAM_SIZE

The presence of a bounded policy structure is permitted only when it is explicitly identified as a policy/resource/security boundary.

---

21. Requirement vs Capability vs Preference vs Realization

The IR MUST distinguish:

Requirement

requires qubits >= n

Capability

requires capability("quantum.measurement")

Constraint

requires topology(...)

Preference

prefer accelerator("quantum")

Realization

logical q0 → physical qubit 17

Only the first four belong in portable semantic requirements.

The final realization belongs downstream.

---

22. Resource Scaling

The IR MUST support programs whose required resource count is determined by program data.

For example:

n = input()
allocate qubits[n]

must not be rejected merely because "n" is not known when the grammar is compiled.

The compiler may later determine that a particular target cannot satisfy the runtime requirement.

That is a target/resource diagnostic, not a grammar incompatibility.

---

23. IR Resource Requirements

Resource requirements in canonical IR MUST preserve:

- resource kind;
- quantity;
- unit;
- symbolic expression where applicable;
- minimum/maximum semantics;
- hard/soft status;
- capability relationship;
- source provenance;
- diagnostic context.

A requirement MUST NOT be silently converted into a hard-coded implementation decision.

---

24. Capability Requirements

Capabilities MUST be represented independently from physical devices.

Examples:

quantum.measurement
quantum.mid_circuit_measurement
tensor.compute
gpu.compute
fpga.synthesis
distributed.communication
persistent.storage

The capability name is semantic metadata.

The implementation later determines which target provides it.

---

25. Type Conformance

IR types MUST be resolved semantic types, not raw source "TypeExpr".

The pipeline is:

source TypeExpr
        ↓
type resolution
        ↓
semantic Type
        ↓
canonical IR type

The IR MUST NOT require re-parsing source-level type syntax.

---

26. Generic Type Conformance

The IR MUST preserve the semantic meaning of generic parameters.

Examples:

Tensor<T, Shape>
Memory<T, Size>
Qubit<N>
Vector<T, N>

The IR MUST preserve parameters that affect program semantics.

It MUST NOT replace symbolic parameters with arbitrary compiler-selected constants merely for convenience.

---

27. Dependent and Symbolic Values

If the language permits symbolic dimensions or requirements, the IR MUST preserve them until the semantic phase that is responsible for resolving them.

For example:

Tensor<T, shape>

MUST NOT become:

Tensor<T, 1024>

unless "1024" is actually established by the program or a permitted specialization.

---

28. Effects

IR MUST represent effects where effects affect semantic correctness.

Examples include:

- I/O;
- allocation;
- mutation;
- synchronization;
- quantum measurement;
- nondeterminism;
- external calls;
- device interaction;
- network communication;
- persistence.

Effect information MUST be preserved across lowering when required to maintain semantic guarantees.

An optimizer MUST NOT eliminate an effect merely because the operation appears unused syntactically.

---

29. Ownership and Resource Semantics

Where Zamani's semantic model defines ownership, borrowing, linearity, or affine usage, the IR MUST preserve the information required to enforce those guarantees.

The IR MUST NOT turn:

linear resource

into:

ordinary unrestricted value

without a semantics-preserving transformation.

---

30. Memory

IR memory semantics MUST remain target-independent.

The IR may represent:

- allocation;
- deallocation;
- ownership;
- references;
- regions;
- address-space intent;
- shared memory;
- distributed memory;
- accelerator memory;
- persistent memory;
- quantum memory abstractions.

It MUST NOT assume:

RAM = 64 GB
VRAM = 24 GB
register = 32 bits

as universal semantics.

---

31. Concurrency

The IR MUST represent semantic concurrency rather than physical worker counts.

Valid concepts include:

parallel
spawn
async
await
task
channel
actor
pipeline
data parallel
task parallel

The number of workers is normally a target/resource realization.

An optimizer may lower:

parallel

to:

1 worker

or:

many workers

provided specified semantics remain unchanged.

---

32. Deterministic Concurrency

If the source program requests deterministic behavior, the IR MUST preserve the required ordering and synchronization semantics.

The compiler MUST NOT introduce nondeterminism merely because additional resources are available.

Where nondeterminism is permitted, it MUST be represented explicitly in semantic/effect metadata where necessary.

---

33. Distributed IR

Distributed semantics MUST distinguish:

logical distribution

from:

physical node placement

The IR may represent:

- partitioning;
- replication;
- communication;
- messages;
- collectives;
- synchronization;
- consistency;
- checkpointing;
- migration;
- fault tolerance.

It MUST NOT require a fixed node count.

---

34. Classical/Quantum Hybrid IR

Hybrid programs MUST remain one semantic program.

The canonical structure is:

classical operation
        ↓
quantum operation
        ↓
measurement
        ↓
classical condition
        ↓
quantum operation

The IR MUST preserve dependencies between classical and quantum domains.

A compiler MUST NOT treat the quantum portion as an unrelated second program.

---

35. HDL/Hardware IR

HDL/hardware source constructs may lower into hardware-intent IR.

The IR may represent:

- modules;
- ports;
- signals;
- registers;
- memories;
- state machines;
- pipelines;
- interfaces;
- protocols;
- timing intent;
- verification intent;
- synthesis intent;
- resource requirements.

It MUST NOT require a fixed:

wire [31:0]

width unless "32" is explicitly part of the program's semantics.

---

36. Hardware-Independent HDL

The semantic hardware IR MUST distinguish:

desired width

from:

physical implementation width

A program may explicitly request:

width = 32

without causing the language to impose:

maximum width = 32

---

37. AI/Data IR

AI and data computation MUST use extensible semantic operations.

The IR may represent:

- tensors;
- datasets;
- model operations;
- training;
- inference;
- differentiation;
- probabilistic operations;
- symbolic operations;
- agents;
- pipelines;
- distributed computation;
- accelerator requirements.

Framework names such as CUDA, TensorFlow, PyTorch, JAX, XLA, TVM, Triton, or vendor-specific APIs MUST NOT become the canonical Zamani semantic model.

They belong to interoperability or lowering.

---

38. Dialects

The existing quantum IR has a dialect subsystem.

Dialects MUST:

1. identify themselves;
2. identify their version;
3. declare their owner;
4. define operations;
5. define attributes;
6. define types where required;
7. define semantic invariants;
8. define lowering;
9. define compatibility;
10. declare required capabilities;
11. remain isolated from unrelated dialects.

A dialect MUST NOT silently change core Zamani semantics.

---

39. Dialect Compatibility

A dialect MUST have an explicit compatibility identity:

dialect name
dialect version
core language version
IR version

A compiler MUST reject incompatible dialects explicitly.

It MUST NOT silently reinterpret them as another dialect.

---

40. IR Version

The IR version is distinct from:

- Zamani language version;
- compiler version;
- Rust version;
- dialect version;
- runtime version;
- target version.

An IR artifact MUST carry sufficient version metadata to determine whether it can be safely interpreted.

---

41. Language Version vs IR Version

These are different:

Zamani 1.x
IR 1.x
Compiler 0.x
Rust 1.97.1

A change in IR representation does not necessarily require a language-version change.

A language semantic change may require both.

The compatibility matrix MUST record the relationship explicitly.

---

42. IR Compatibility Classes

Every IR change MUST be classified as one of:

IDENTICAL
BACKWARD_COMPATIBLE
FORWARD_COMPATIBLE
LOSSLESS_MIGRATION
LOSSY_MIGRATION
INCOMPATIBLE
IMPLEMENTATION_ONLY

"IMPLEMENTATION_ONLY" is valid only where semantic meaning is unchanged.

---

43. Lossless IR Migration

A migration is lossless only when the complete semantic information required by the receiving IR version can be reconstructed.

A transformation that drops:

- effects;
- capabilities;
- resource requirements;
- quantum controls;
- measurement semantics;
- provenance;
- type information;
- ordering constraints;

MUST NOT be called lossless.

---

44. Lossy IR Migration

A lossy migration MUST be explicit.

The migration MUST state:

- what information is lost;
- why;
- affected features;
- semantic consequences;
- affected targets;
- diagnostic requirements;
- whether execution remains permitted.

The compiler MUST NOT silently discard semantic information.

---

45. IR Canonicalization

Canonicalization MUST produce deterministic IR representation.

Equivalent canonical IR MUST have deterministic:

- operation ordering where ordering is semantically meaningful;
- attribute ordering;
- map/set serialization order;
- identifier representation;
- metadata representation;
- dialect metadata;
- serialization;
- hashing.

Hash-map iteration order MUST NOT determine canonical output.

---

46. Canonical Hashing

The actual repository provides:

src/quantum/ir/hashing/

as the canonical hashing subsystem.

The compatibility alias:

quantum::ir::hash

MUST remain an alias to the canonical hashing implementation.

No second hashing algorithm may be introduced merely for compatibility.

Hash inputs MUST include all semantic information required to prevent semantically different IR from producing an identical canonical semantic identity.

Where version identity affects interpretation, the applicable IR/version metadata MUST be included.

---

47. Serialization

Serialization belongs to the canonical IR serialization subsystem.

Serialization MUST preserve:

- semantic values;
- types;
- operations;
- operands;
- results;
- attributes;
- effects;
- resources;
- capabilities;
- provenance;
- version metadata;
- dialect metadata.

Serialization MUST be deterministic.

A serialization format MUST NOT silently omit semantic information.

---

48. Source Provenance

The existing IR architecture contains metadata/provenance facilities.

Every IR construct originating from source SHOULD preserve enough provenance to identify:

- source file;
- source span;
- originating AST node where applicable;
- generated/desugared status;
- dialect;
- transformation provenance.

Generated IR MUST remain distinguishable from source-originated IR where diagnostics require it.

---

49. Source Span Preservation

The IR does not have to preserve the exact source span for every machine-generated helper operation.

However, every source-semantic operation MUST retain sufficient provenance for:

- diagnostics;
- compiler errors;
- optimization diagnostics;
- capability errors;
- resource errors;
- quantum errors;
- target errors;
- debugging;
- IDE tooling where supported.

---

50. IR Identity

IR identities MUST NOT depend on:

- memory addresses;
- pointer addresses;
- allocation order outside the semantic contract;
- hash-map iteration;
- host process identity;
- target hardware identity.

Deterministic identity MUST be explicit where identity is observable.

---

51. Operation Ordering

The IR MUST distinguish:

semantic ordering

from:

incidental storage ordering

Operations that are independent may be reordered by optimization.

Operations whose order is semantically observable MUST retain dependency information.

Quantum operations are especially sensitive to this rule.

---

52. Quantum Ordering

Quantum IR MUST preserve all ordering constraints required by:

- state evolution;
- measurement;
- reset;
- classical feed-forward;
- control dependencies;
- synchronization;
- temporal semantics.

An optimizer MUST NOT reorder quantum operations merely because they operate on different data structures.

It must first prove that the transformation preserves semantics.

---

53. Measurement

Measurement is a semantic boundary.

The IR MUST preserve:

- measured operands;
- measurement basis;
- result identity;
- destination;
- conditions;
- ordering;
- relevant effects;
- required capabilities.

Measurement MUST NOT be lowered into a generic classical operation if doing so loses quantum semantics.

---

54. Classical Feed-Forward

The IR MUST preserve dependencies such as:

measure q
        ↓
classical result
        ↓
conditional quantum operation

The optimizer MUST preserve this dependency even if target lowering later transforms the representation.

---

55. Reset

Reset is not equivalent to arbitrary initialization unless the semantic specification establishes that equivalence.

The IR MUST preserve reset semantics.

---

56. Dynamic Quantum Control

Dynamic control must remain representable.

Examples include:

if measured_bit {
    apply operation
}

The IR MUST preserve the relationship between measurement results and later control flow.

---

57. Quantum Pulse IR

The repository contains a pulse subsystem.

Pulse IR MUST remain hardware-independent at its semantic boundary.

Pulse IR may describe:

- pulse;
- frame;
- port;
- waveform;
- capture;
- timing;
- calibration intent.

Physical calibration and device-specific control remain downstream.

---

58. Quantum Error Correction

QEC MUST remain downstream of canonical quantum semantics.

The boundary is:

quantum::ir
        ↓
QEC analysis/transformation
        ↓
fault-tolerant realization

The frontend MUST NOT encode a specific QEC implementation into the canonical source grammar.

IR conformance requires that sufficient semantic information is preserved for QEC where the source requests it.

---

59. ZQN

ZQN remains responsible for fault/noise/resilience semantics according to the existing architecture.

The canonical IR MAY carry:

- error requirements;
- noise requirements;
- resilience requirements;
- reliability requirements;
- fault-tolerance intent.

The IR MUST NOT duplicate ZQN's implementation.

ZQN MUST NOT redefine source syntax.

---

60. Routing

Routing consumes logical semantic information.

The boundary is:

logical IR
        ↓
routing
        ↓
physical realization

Routing MUST NOT modify the source meaning.

For quantum computation:

logical q0

may become:

physical q17

without changing source semantics.

---

61. Scheduling

Scheduling consumes semantic dependencies and resource information.

Scheduling MAY determine:

- execution order;
- start times;
- parallelism;
- resource occupancy;
- device-specific timing.

Scheduling MUST preserve semantic dependencies.

Scheduling is not part of the source grammar's machine model.

---

62. Optimization

Optimization is valid only when semantics are preserved.

Optimization MUST preserve:

- values;
- control flow;
- effects;
- resource requirements;
- capability requirements;
- quantum semantics;
- measurement semantics;
- ordering constraints;
- observable behavior.

Optimization MAY change:

- instruction count;
- operation ordering where legal;
- representation;
- memory layout;
- physical mapping;
- target-specific implementation.

---

63. Optimization and Resources

An optimization MAY reduce resource consumption.

It MUST NOT silently increase a hard program requirement beyond what the program permits.

For example, if a program explicitly requires:

memory <= budget

a transformation exceeding that budget MUST be rejected or handled through an explicit policy.

---

64. Compiler Target Selection

Target selection occurs after semantic IR construction.

The compiler may inspect:

- target capabilities;
- available memory;
- topology;
- device features;
- accelerator availability;
- QPU capabilities;
- FPGA synthesis constraints;
- runtime capabilities.

The compiler MUST NOT rewrite source semantics merely because the selected target lacks a capability.

---

65. Target Incompatibility

When a target cannot execute an otherwise valid program, the compiler MUST distinguish:

SOURCE_INVALID

from:

TARGET_UNSUPPORTED

and:

RESOURCE_UNAVAILABLE

and:

CAPABILITY_UNAVAILABLE

and:

POLICY_LIMIT_EXCEEDED

These MUST NOT collapse into a generic parse error.

---

66. POCO-REAF

POCO-REAF requires:

Program Once
        ↓
Stable semantic meaning
        ↓
Compile Once
        ↓
Reusable semantic/artifact contract
        ↓
Run Everywhere
        ↓
Target realization
        ↓
Run Anywhere
        ↓
Different deployment environments
        ↓
Run Forever
        ↓
Versioned semantic/artifact migration

This does NOT promise that one physical binary can execute unchanged on every future machine.

It means the program's semantic contract remains portable and can be realized by future implementations.

---

67. Small-to-Large Scaling

The same semantic IR must be able to represent:

tiny embedded computation
        ↓
single CPU
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
distributed/cloud
        ↓
future architecture

The IR MUST NOT fork into separate semantic languages for these targets.

---

68. Atom-to-Everywhere Scaling

Zamani's semantic model must remain capable of representing computation involving:

- atomic/nano-scale abstractions;
- embedded systems;
- classical processors;
- accelerators;
- quantum systems;
- hardware descriptions;
- distributed systems;
- future computational substrates.

A target-specific implementation may reject a program because required resources or capabilities are unavailable.

That does not make the source language target-specific.

---

69. No Hardware-Size Language Semantics

The following are prohibited as universal semantic rules:

version 1 → 32 qubits
version 2 → 64 qubits
version 3 → 128 qubits

Correct:

language version
+
program requirements
+
target capabilities
+
resource availability

---

70. IR and Hardware Topology

Topology may appear in IR only when it is explicitly part of semantic resource requirements.

For example:

requires topology(...)

is valid.

But the canonical IR MUST NOT silently become a snapshot of one machine's topology.

Physical topology belongs to target realization.

---

71. IR and Memory

The IR may express:

memory requirement
memory class
address-space requirement
locality requirement
sharing requirement
persistence requirement

It MUST NOT encode current hardware capacities as universal language semantics.

---

72. IR and Register Width

Register width may be a target property or explicit program requirement.

The canonical IR MUST NOT assume:

register = 32-bit

as a universal limitation.

Likewise:

vector = 256-bit

must not become a language-wide ceiling.

---

73. IR and Tensor Rank

Tensor rank is program semantics where explicitly specified.

The IR MUST NOT impose a universal:

MAX_TENSOR_RANK

The compiler may use an implementation policy to protect resources.

That policy MUST remain separate from the language semantic contract.

---

74. IR and Distributed Scale

The IR MUST NOT encode:

MAX_NODES

as a language limit.

Distributed resource requirements must remain data-driven and capability-driven.

---

75. IR and Timelines

If Zamani's temporal/multi-timeline features are represented in IR, the number of timelines MUST be determined by program semantics and resources, not a hard-coded language maximum.

---

76. IR and Nano/Sankofa Features

Existing broader Zamani design concepts such as:

- memory;
- recall;
- temporal information;
- learning;
- provenance;
- historical state;
- consensus;
- temporal reasoning;

must lower through ordinary semantic operations, effects, resources, and dialects.

They MUST NOT create an independent IR authority merely because their syntax originated in the broader "Zamani-Grammar.md".

---

77. Interoperability

Interoperability formats such as:

- OpenQASM;
- QIR;
- LLVM-oriented representations;
- MLIR;
- HDL formats;
- WebAssembly;
- foreign-language ABIs;

are interoperability boundaries.

They MUST NOT become the canonical Zamani semantic model.

The direction is:

Zamani semantic IR
        ↕
interoperability adapter
        ↕
external representation

not:

external representation
        ↓
Zamani semantic authority

unless explicitly adopted by the specification.

---

78. OpenQASM

OpenQASM integration MUST occur through:

interoperability

and/or:

quantum frontend formats

as already established by the repository architecture.

OpenQASM syntax MUST NOT redefine "quantum::ir".

Any OpenQASM import MUST preserve semantic meaning and report unsupported constructs explicitly.

---

79. ABI

ABI information is downstream of language semantics.

The IR may carry calling-convention or ABI requirements where semantically necessary.

It MUST NOT make one ABI universal.

---

80. Foreign Functions

FFI operations MUST preserve:

- argument types;
- result types;
- ownership;
- effects;
- calling convention;
- safety contract;
- target requirements.

The IR MUST NOT treat an FFI call as pure unless its semantic contract establishes purity.

---

81. Diagnostics

Every IR conformance failure MUST have a structured diagnostic category.

At minimum:

IR_UNSUPPORTED
IR_INVALID
IR_VERSION_MISMATCH
IR_DIALECT_MISMATCH
IR_TYPE_MISMATCH
IR_EFFECT_MISMATCH
IR_RESOURCE_MISMATCH
IR_CAPABILITY_MISMATCH
IR_PROVENANCE_ERROR
IR_SERIALIZATION_ERROR
IR_CANONICALIZATION_ERROR
IR_TARGET_INCOMPATIBLE
IR_LOWERING_ERROR
IR_SEMANTIC_LOSS

Diagnostics MUST identify the earliest failing boundary.

---

82. First-Divergence Rule

When conformance fails:

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

the validator MUST identify the first divergence.

Example:

Specification accepts A
Lexer rejects A

is a lexical defect.

If:

Specification says A = X
Lexer accepts A
Parser accepts A
AST stores A
Semantic layer interprets A = Y

the semantic layer is the first divergence.

Downstream components MUST NOT compensate for upstream defects.

---

83. No Downstream Semantic Reconstruction

The IR generator MUST NOT reconstruct information that the AST or semantic layer discarded.

For example, if the AST loses:

quantum control condition

the IR generator MUST NOT guess it from source text.

The correct action is to repair the upstream contract.

---

84. No Silent Semantic Loss

The following are production defects:

AST contains capability
IR drops capability

AST contains effect
IR drops effect

AST contains quantum condition
IR drops condition

AST contains resource requirement
IR drops requirement

AST contains source span
IR loses all provenance

unless the relevant information has been formally proven irrelevant to all downstream semantics.

---

85. IR Verification

Canonical IR MUST be validated before optimization and lowering.

At minimum validation MUST cover:

- type correctness;
- operation validity;
- operand/result consistency;
- region validity;
- block validity;
- control-flow validity;
- dependency validity;
- effect consistency;
- resource consistency;
- capability consistency;
- dialect validity;
- version compatibility;
- provenance validity where required.

---

86. Existing IR Validation Boundary

The actual repository provides:

src/quantum/ir/validation/

This subsystem MUST remain responsible for whole-IR structural/semantic validation.

"grammar/compatibility/ir-conformance.md" defines the compatibility contract; it does not duplicate the validator implementation.

---

87. IR Analysis

The actual repository provides:

src/quantum/ir/analysis/

Analysis MUST be read-only with respect to canonical IR unless a transformation explicitly owns mutation.

Analysis results MUST NOT silently become semantic mutations.

---

88. Scheduling IR

The actual repository provides:

src/quantum/ir/scheduling/

Scheduling representations MUST remain semantic scheduling representations.

Scheduling algorithms remain downstream.

A schedule MUST NOT be confused with source semantics.

---

89. Resource IR

The actual repository provides:

src/quantum/ir/resources/

This subsystem owns abstract:

- resource requirements;
- capabilities;
- topology;
- resource relationships.

It MUST remain separate from actual device inventory.

---

90. Compatibility IR

The actual repository provides:

src/quantum/ir/compatibility/

This subsystem may provide compatibility representations/helpers.

It MUST NOT replace:

grammar/compatibility/ir-conformance.md

The grammar document is the normative cross-layer contract.

The Rust compatibility subsystem is the executable implementation.

---

91. Compatibility Layer Rules

Compatibility code MUST:

- preserve canonical types;
- avoid duplicate types;
- avoid duplicate operation models;
- provide explicit migration;
- identify legacy paths;
- remain deterministic;
- avoid semantic reinterpretation.

Compatibility aliases MUST point to canonical implementations.

---

92. Root Module Rule

"src/quantum/ir/mod.rs" is a composition/root API module.

It MUST NOT become a second implementation of domain logic.

Its existing pattern of:

pub mod ...
pub use ...

is appropriate.

New APIs should be added through their owning subsystem.

---

93. No Glob-Based Compatibility Ambiguity

Canonical APIs SHOULD use explicit re-exports.

Glob exports MUST NOT create accidental ownership ambiguity.

A new symbol MUST have one canonical owner.

---

94. Rust Version

Production Zamani compiler implementation MUST support:

Rust 1.97.1
Rust edition 2021

Rust 1.97 remains within the requested compatibility baseline where the toolchain exists, but the repository's manifest MUST use valid Cargo syntax.

The current repository manifest contains:

rust-version = "1.97" or "1.97.1"

This is invalid Cargo metadata.

It MUST be corrected to a valid value, preferably:

rust-version = "1.97.1"

The correction belongs in "Cargo.toml", not in this document.

This IR contract MUST NOT pretend that the invalid manifest is already production-ready.

---

95. Safe Rust

Production compiler implementation MUST use safe Rust.

The repository MUST enforce:

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

where appropriate to the crate/module architecture.

No IR compatibility feature may require "unsafe".

No grammar compatibility feature may require "unsafe".

No serialization, hashing, migration, or validation feature may require "unsafe".

---

96. Overflow Safety

Resource accounting MUST use checked arithmetic where overflow is possible.

This is consistent with the existing "QuantumIrLimits" implementation.

The compiler MUST NOT allow integer overflow to turn into:

- incorrect resource acceptance;
- incorrect IR sizes;
- incorrect schedule times;
- incorrect memory requirements;
- incorrect qubit counts.

---

97. Host Representation vs Language Capacity

A Rust representation such as:

usize
u64
u128

is an implementation representation.

It MUST NOT automatically become a language semantic ceiling.

For example:

usize

may be used internally for a collection index.

That does not imply:

Zamani maximum resource count = usize::MAX

The distinction must remain explicit.

---

98. Resource Policy vs Physical Limit

The complete distinction is:

Program requirement
        ↓
IR semantic requirement
        ↓
compiler policy
        ↓
target capability
        ↓
actual resource availability
        ↓
execution

Each layer may impose a constraint.

Only the program semantic requirement belongs to the language meaning.

---

99. IR Specialization

Specialization MAY replace symbolic values with concrete values when:

1. semantics are preserved;
2. the specialization condition is known;
3. the specialization is recorded where required;
4. fallback behavior is correct where required.

Specialization MUST NOT silently change the program's declared requirements.

---

100. Constant Folding

Constant folding is valid only when the language semantics establish that the folded expression is equivalent.

Special care is required for:

- floating-point behavior;
- overflow;
- NaN;
- infinities;
- effects;
- quantum operations;
- nondeterministic operations;
- external calls.

---

101. Quantum Optimization

Quantum optimization MAY perform:

- gate cancellation;
- decomposition;
- algebraic simplification;
- parameter simplification;
- circuit restructuring;
- measurement-aware transformations.

It MUST preserve the semantic quantum operation.

The optimizer MUST NOT assume that today's hardware gate set is the language's complete operation universe.

---

102. Hardware Lowering

Hardware lowering is downstream:

canonical IR
      ↓
target-specific lowering
      ↓
machine representation

The canonical IR MUST remain target-independent unless a target-specific dialect is explicitly selected.

---

103. Target-Specific Dialects

A target-specific dialect MAY represent:

- GPU kernels;
- FPGA primitives;
- ASIC structures;
- QPU-specific operations;
- vendor accelerator operations.

However:

target dialect

MUST NOT silently become:

core Zamani semantics

The boundary must remain explicit.

---

104. Target Capability Negotiation

Target selection SHOULD follow:

program requirements
        ↓
target capabilities
        ↓
resource availability
        ↓
legal realization

not:

target selected
        ↓
rewrite program meaning

---

105. Failure Semantics

If a target cannot satisfy a valid IR program, the compiler MUST report the reason.

Examples:

required capability unavailable
required resource unavailable
required topology unavailable
unsupported operation
unsupported dialect
unsupported IR version
unsupported effect
unsupported runtime guarantee

It MUST NOT silently remove the unsupported feature.

---

106. IR Round Trip

Where serialization supports round trips:

IR
 ↓
serialize
 ↓
deserialize
 ↓
IR

the semantic meaning MUST remain equivalent.

Canonical serialization SHOULD produce byte-stable output for equivalent canonical IR.

---

107. IR Hash Stability

For semantically identical canonical IR:

canonicalize(A)
canonicalize(B)

must produce identical canonical representation where the hash contract says they are equivalent.

Hashing MUST NOT depend on:

- process address;
- random hash seed;
- map iteration;
- machine-specific pointer values;
- wall-clock time;
- host path unless provenance semantics explicitly require it.

---

108. Deterministic Compilation

Given identical:

- source;
- language version;
- feature configuration;
- dialect versions;
- compiler version where relevant;
- deterministic compiler configuration;
- canonical dependencies;

the semantic IR MUST be deterministic.

Target-specific output MAY differ when target inputs differ.

---

109. Reproducibility

Reproducibility metadata SHOULD identify:

- language version;
- IR version;
- dialect versions;
- relevant compiler version;
- source identity;
- dependency identity;
- target identity where target output is involved;
- transformation configuration.

It MUST NOT accidentally encode volatile runtime data as semantic identity.

---

110. IR and Caching

Compiler caches may use canonical IR hashes.

Cache keys MUST include all information required to prevent incompatible semantic reuse.

A cache hit MUST NOT cause a program compiled under incompatible semantic assumptions to be reused.

---

111. IR and Provenance

Transformation provenance SHOULD be represented where required for:

- diagnostics;
- debugging;
- auditing;
- reproducibility;
- certification;
- migration;
- security.

Provenance MUST NOT become a hidden semantic input unless the language explicitly defines it as semantic.

---

112. Security Boundary

IR input may be untrusted.

Deserialization, validation, optimization, and analysis MUST defend against:

- excessive nesting;
- excessive operation counts;
- arithmetic overflow;
- malformed identifiers;
- malformed dialects;
- invalid references;
- resource exhaustion;
- malformed metadata;
- invalid dependency graphs.

Resource policies are therefore valid defensive mechanisms.

They MUST remain policies rather than language ceilings.

---

113. Denial-of-Service Protection

A compiler service MAY impose policy limits on:

- operations;
- expressions;
- metadata;
- serialized bytes;
- validation work;
- processing depth;
- waveform samples;
- symbolic expressions.

Such limits MUST be explicitly classified as:

service/security policy

and MUST NOT be presented as:

Zamani language maximum

---

114. Large Programs

The IR MUST scale with available implementation resources.

The implementation MUST avoid unnecessary fixed-size arrays where dynamic structures are appropriate.

Large programs SHOULD be processed using streaming, chunking, incremental processing, or other bounded techniques where appropriate.

A compiler implementation MAY reject a program when actual resources are insufficient.

It MUST distinguish:

resource exhaustion

from:

invalid Zamani program

---

115. Small Programs

The same IR architecture MUST remain usable for tiny programs.

The implementation MUST NOT require large-scale runtime infrastructure merely to represent:

one classical operation

or:

one qubit

The semantic architecture scales in both directions.

---

116. Incremental Compilation

Incremental compilation MAY reuse IR fragments when their semantic identities and dependencies remain valid.

Invalidation MUST be based on semantic dependencies rather than file modification time alone.

---

117. Parallel Compilation

Compilation stages may execute in parallel where semantics permit.

Parallel compilation MUST preserve deterministic results.

Thread count is an implementation resource.

It MUST NOT become a language semantic requirement.

---

118. Distributed Compilation

Distributed compiler services MAY partition IR processing.

The partitioning MUST preserve semantic dependencies.

Distributed compilation MUST NOT introduce a different semantic interpretation.

---

119. IR Feature Lifecycle

Every IR feature MUST have:

PROPOSED
    ↓
DESIGNED
    ↓
EXPERIMENTAL
    ↓
IMPLEMENTED
    ↓
CONFORMANT
    ↓
STABLE
    ↓
DEPRECATED
    ↓
REMOVED

A grammar rule is not enough to make an IR feature implemented.

---

120. Feature Completeness

A feature is IR-conformant only when:

- syntax exists;
- AST mapping exists;
- semantic meaning exists;
- IR representation exists;
- validation exists;
- diagnostics exist;
- compiler integration exists;
- required runtime integration exists;
- relevant target integration exists;
- compatibility is classified;
- tests exist;
- scalability behavior is defined.

---

121. Feature Status

"grammar/grammar.md" MUST report implementation status.

Recommended statuses:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

"Zamani-Grammar.md" remains broader design material and MUST NOT silently establish IR support.

---

122. Compatibility Matrix Integration

This file MUST integrate with:

grammar/compatibility/compatibility-matrix.md

The matrix MUST be able to identify:

feature
language version
grammar
lexer
parser
AST
semantic analysis
IR
compiler
runtime
target
status
compatibility

A feature MUST NOT be marked fully supported when the IR column is incomplete.

---

123. AST Conformance Integration

This file consumes:

grammar/compatibility/ast-conformance.md

The AST contract establishes the structural input to semantic lowering.

The IR contract establishes the semantic output.

The IR generator MUST NOT bypass the AST contract.

---

124. Frontend Conformance Integration

This file consumes:

grammar/compatibility/frontend-conformance.md

The frontend MUST produce an AST that contains every semantic fact required by IR generation.

If it does not, the defect belongs upstream.

---

125. Specification Integration

This file integrates with:

grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/spec/semantics.md
grammar/spec/type-system.md
grammar/spec/resources.md
grammar/spec/quantum.md
grammar/spec/diagnostics.md
grammar/spec/compatibility.md

Those files own their respective semantics.

This file defines how those semantics must survive into IR.

---

126. Versioning Integration

This file integrates with:

grammar/compatibility/versions.md
grammar/specification/language-version.md

Language version determines source semantics.

IR version determines IR representation.

Compiler version determines implementation.

Target version determines target realization.

They MUST remain separate.

---

127. Migration Integration

IR migrations belong with:

grammar/compatibility/migrations.md

This file defines what an IR migration MUST preserve.

Migration tooling MUST explicitly classify:

lossless
lossy
incompatible

---

128. Deprecation Integration

Deprecated IR forms MUST be identified through:

grammar/compatibility/deprecated.md

A deprecated representation MUST NOT silently acquire new semantics.

---

129. Reserved Integration

Reserved operations/types/attributes belong to:

grammar/compatibility/reserved.md

IR implementations MUST NOT consume reserved identifiers without the corresponding feature contract.

---

130. Validation Integration

The executable validation layer should be:

grammar/validation/

and the IR implementation validation layer is:

src/quantum/ir/validation/

The two layers have different roles:

grammar/validation/
    = repository/specification conformance

src/quantum/ir/validation/
    = executable IR validation

They MUST agree.

---

131. Compiler Integration

The compiler pipeline MUST be:

source
 ↓
lexer
 ↓
parser
 ↓
AST
 ↓
semantic analysis
 ↓
canonical semantic IR
 ↓
IR validation
 ↓
optimization
 ↓
lowering
 ↓
routing / scheduling / resilience / QEC / ZQN
 ↓
HAL
 ↓
backend

The backend MUST NOT be responsible for discovering missing source semantics.

---

132. Runtime Integration

Runtime receives:

- executable representation;
- required capabilities;
- resource requirements;
- execution policies;
- required guarantees.

Runtime MUST NOT infer missing semantic information from the target.

---

133. HAL Integration

HAL is responsible for target abstraction.

It may report:

capability available
capability unavailable
resource available
resource unavailable
device degraded
device unavailable

It MUST NOT rewrite the source program's semantic meaning.

---

134. Resilience Integration

The architecture already distinguishes resilience states including:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

These are execution/target state semantics.

They MUST NOT become source-language hardware limits.

IR may preserve resilience requirements and guarantees where defined.

---

135. Execution Outcomes

Where execution semantics expose outcomes such as:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

the IR contract MUST distinguish:

program semantic result

from:

execution-management outcome

A target recovery action must not silently change the program result.

---

136. Diagnostics and Source Errors

IR generation MUST fail explicitly when semantic information required for IR is missing.

It MUST NOT generate placeholder operations and continue to successful code generation.

In particular, the implementation MUST NOT use dummy IR values to conceal unresolved semantic errors.

---

137. No Phantom IR

A feature MUST NOT claim IR support merely because:

an enum variant exists

or:

a parser rule exists

or:

a documentation section exists

Actual support requires the complete lowering and validation path.

---

138. No Parser-Only Features

The following is not production support:

grammar parses feature
        ↓
AST stores feature
        ↓
IR ignores feature

If the feature is semantically meaningful, this is a conformance defect.

---

139. No AST-Only Features

Likewise:

AST contains feature
        ↓
semantic analyzer ignores feature

is incomplete.

---

140. No Semantic-Only Phantom

Likewise:

semantic analyzer validates feature
        ↓
IR has nowhere to represent it

is incomplete.

The semantic model must have a defined IR boundary or an explicit reason why the feature is compile-time-only.

---

141. Compile-Time-Only Features

A feature MAY legitimately terminate before IR when it is purely compile-time.

Examples may include:

- compile-time assertions;
- macro expansion;
- static validation;
- source-level annotations.

Such features MUST explicitly declare:

IR mapping = none

and explain why no runtime semantic information is required.

---

142. Compile-Time Metadata

Compile-time metadata may be removed after its purpose is fulfilled.

The compiler MUST verify that removal is semantics-preserving.

---

143. Macro Expansion

Macros MUST lower to ordinary AST constructs before semantic IR generation unless a macro-specific semantic contract explicitly requires another path.

Macros MUST NOT bypass:

semantic validation

---

144. Metaprogramming

Generated code MUST enter the same semantic pipeline as ordinary source.

The IR MUST NOT trust generated code merely because it was produced by a macro or metaprogram.

---

145. Inter-Domain Composition

A single IR program MAY contain:

classical
quantum
HDL
AI
data
networking
distributed
security
memory
concurrency

where the language semantics permit composition.

Cross-domain operations MUST explicitly identify:

- operands;
- results;
- effects;
- capabilities;
- resources;
- dependencies.

---

146. Domain Isolation

A domain MUST NOT silently reinterpret another domain's operation.

For example:

quantum.measure

must not become ordinary:

read()

without a semantics-preserving contract.

Likewise:

hdl.register

must not become:

integer variable

unless the semantic specification explicitly defines that equivalence.

---

147. IR Contract for New Domains

A new computational domain MUST provide:

1. specification;
2. syntax contract;
3. AST contract;
4. semantic contract;
5. IR contract;
6. resource contract;
7. capability contract;
8. diagnostics;
9. compatibility;
10. validation;
11. tests;
12. target integration.

The new domain MUST reuse universal language foundations.

---

148. Future Hardware

A future hardware target MUST be able to consume existing semantic IR when it implements the necessary semantics/capabilities.

Adding a new:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- accelerator;
- simulator;
- distributed environment

MUST NOT require a new source language merely because the target is new.

---

149. Future Computational Paradigms

A future computational paradigm SHOULD be introduced through:

dialect
+
semantic operation model
+
capabilities
+
resource requirements
+
canonical IR lowering

rather than a fork of the core language.

---

150. Compatibility of Equivalent Programs

Two source programs are IR-compatible when they have equivalent specified semantics even if their syntax differs through:

- syntactic sugar;
- macro expansion;
- desugaring;
- equivalent type notation;
- equivalent operation spelling;
- compatible dialect syntax.

The canonical IR should remove irrelevant syntactic differences.

---

151. Canonicalization vs Optimization

Canonicalization MUST NOT be confused with optimization.

Canonicalization establishes a deterministic representation.

Optimization changes representation while preserving semantics.

The compiler SHOULD perform:

parse
 ↓
semantic analysis
 ↓
canonicalization
 ↓
validation
 ↓
optimization

where the architecture requires canonical identity before optimization.

---

152. Optimization Equivalence

After optimization:

IR_before

and:

IR_after

must be semantically equivalent according to the IR semantic model.

A changed hash does not automatically mean incompatibility.

A changed semantic hash under a contract claiming semantic identity indicates a defect.

---

153. Lowering Equivalence

Lowering from canonical IR into a target dialect MUST preserve all source-observable semantics.

If exact equivalence cannot be guaranteed, the compiler MUST identify the limitation.

---

154. Target Dialect Escape

Target-specific escape hatches MUST be explicit.

They MAY be marked:

target-specific
non-portable
experimental

as appropriate.

They MUST NOT be silently treated as portable core semantics.

---

155. Portability Classification

Every target-sensitive IR construct SHOULD have one of:

PORTABLE
CAPABILITY_DEPENDENT
RESOURCE_DEPENDENT
TARGET_DEPENDENT
DIALECT_DEPENDENT
NON_PORTABLE

This allows tooling to identify what prevents universal realization.

---

156. IR Compatibility Levels

The compatibility system SHOULD distinguish:

Level A — Source compatibility

Existing source parses and means the same thing.

Level B — Semantic compatibility

Existing semantic representation means the same thing.

Level C — IR compatibility

Existing IR can be consumed directly.

Level D — Artifact compatibility

Serialized IR can be consumed.

Level E — Runtime compatibility

The runtime can execute the artifact.

Level F — Target compatibility

The selected target can realize it.

These levels MUST NOT be conflated.

---

157. Release Gate

An IR-affecting release MUST NOT be declared production-ready until:

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
semantic analysis
    ↓
IR
    ↓
IR validation
    ↓
compiler
    ↓
runtime

has been checked for all affected features.

---

158. Required IR Tests

The repository MUST contain tests covering:

tests/ir/
tests/compatibility/
tests/quantum/
tests/classical/
tests/hdl/
tests/hybrid/
tests/resources/
tests/determinism/
tests/scalability/
tests/negative/
tests/boundary/

where those existing directories are the appropriate owners.

New directories SHOULD NOT duplicate existing repository hierarchy unnecessarily.

---

159. Positive Tests

Positive tests MUST verify that valid source semantics become valid canonical IR.

At minimum:

- minimal classical program;
- generic program;
- quantum program;
- hybrid program;
- HDL intent;
- resource requirements;
- capability requirements;
- distributed program;
- tensor program;
- asynchronous program;
- interoperability operation;
- dialect operation where supported.

---

160. Negative Tests

Negative tests MUST verify rejection of:

- malformed IR;
- invalid operation operands;
- invalid result counts;
- invalid types;
- invalid regions;
- invalid control flow;
- incompatible dialects;
- unsupported IR versions;
- missing capabilities;
- invalid resource requirements;
- semantic loss;
- invalid quantum dependencies;
- invalid measurement dependencies;
- malformed serialized IR.

---

161. Boundary Tests

Boundary tests MUST cover:

- empty program;
- single operation;
- one qubit;
- multiple qubits;
- large symbolic resource counts;
- empty regions;
- deeply nested regions;
- many operations;
- many parameters;
- many dependencies;
- large metadata;
- large serialized IR;
- maximum configured policy values;
- policy overflow;
- arithmetic overflow.

Boundary tests MUST NOT define a false language maximum.

---

162. Scalability Tests

Scalability tests MUST verify behavior as resources grow.

Representative cases SHOULD include:

1
small
medium
large
very large
policy-unbounded

where practical.

The purpose is to prove that scaling is controlled by actual resources and explicit policy rather than hard-coded language ceilings.

---

163. Quantum Scalability Tests

Quantum conformance tests MUST cover:

1 qubit
2 qubits
many qubits
symbolic qubit count
parameterized registers
custom operations
vendor-qualified operations
multi-control operations
measurement
mid-circuit measurement
classical feed-forward
logical qubits
physical mapping metadata

The tests MUST NOT establish:

maximum supported qubits = N

as a language rule.

---

164. Classical Scalability Tests

Tests SHOULD cover:

- scalar;
- vector;
- matrix;
- tensor;
- symbolic dimensions;
- large dimensions;
- dynamic dimensions;
- parallel computation.

---

165. HDL Scalability Tests

Tests SHOULD cover:

- parameterized widths;
- parameterized arrays;
- generated structures;
- pipelines;
- memories;
- modules;
- interfaces;
- large generated designs.

No test may establish a universal maximum register width.

---

166. Distributed Scalability Tests

Tests SHOULD vary node counts as data/resource inputs.

They MUST NOT encode a language maximum.

---

167. Determinism Tests

Repeated compilation of the same semantic input under the same configuration MUST produce:

- deterministic diagnostics;
- deterministic canonical IR;
- deterministic canonical serialization;
- deterministic semantic hashes.

Hash-map iteration order MUST NOT affect results.

---

168. Differential Conformance

Where multiple frontend paths exist, they SHOULD be tested against the same canonical IR.

For example:

Zamani source

and:

interoperability import

may produce equivalent canonical semantic operations.

Differential tests SHOULD compare semantic IR rather than textual syntax.

---

169. Golden IR Tests

Golden tests MAY store canonical IR snapshots.

Golden files MUST include enough metadata to identify:

- language version;
- IR version;
- dialect versions;
- canonicalization version where applicable.

A golden test failure MUST be classified rather than blindly updated.

---

170. Snapshot Update Rule

A changed IR snapshot MUST NOT be accepted merely because tests fail.

The change must first be classified as:

intentional semantic change
representation-only change
canonicalization change
bug fix
compatibility migration

The appropriate compatibility documents MUST then be updated.

---

171. Conformance Ledger

Production conformance SHOULD maintain a machine-readable feature ledger.

Each feature entry should identify:

feature_id
language_version
grammar_rule
lexer_tokens
parser_rule
ast_node
semantic_rule
ir_operation/type
dialect
validation
compiler
runtime
targets
status
compatibility
tests

The ledger prevents "parser implemented but IR missing" situations.

---

172. Feature ID

Every production feature SHOULD have a stable feature identifier.

Example:

quantum.operation.generic
quantum.measurement
resource.requirement
capability.requirement
hdl.module
tensor.type
concurrency.parallel

Feature IDs MUST remain stable across compatible implementation changes.

---

173. IR Operation IDs

IR operation identifiers MUST be stable within their declared IR/dialect version.

Renaming an operation MUST be treated as a compatibility event.

Aliases MAY preserve compatibility.

---

174. Semantic Operation Names

Semantic operation names MUST NOT depend on vendor implementation naming where portability is intended.

Vendor-specific names belong in explicit dialect namespaces.

---

175. Namespace Rules

Operation identity SHOULD follow:

core.operation
quantum.operation
hdl.operation
tensor.operation
vendor.operation

A namespace collision MUST be diagnosed deterministically.

---

176. Attributes

Attributes MUST have explicit semantic ownership.

An optimizer MUST NOT assume that an unknown attribute is irrelevant.

Unknown attributes may affect semantics.

They MUST be preserved or rejected according to dialect rules.

---

177. Unknown Operations

Unknown operations MAY be preserved when the active dialect explicitly permits extensibility.

Otherwise they MUST produce:

IR_UNSUPPORTED

rather than being discarded.

---

178. Unknown Attributes

Likewise, unknown attributes MUST NOT silently disappear.

A dialect MUST define whether unknown attributes are:

allowed
preserved
ignored
rejected

---

179. Unknown Types

Unknown semantic types MUST NOT be lowered as an arbitrary fallback type.

The compiler MUST either:

- understand the type;
- invoke a declared dialect;
- invoke a declared interoperability boundary;
- reject it.

---

180. Semantic Preservation Rule

The strongest IR rule is:

«Every semantic fact required to reproduce the specified program behavior MUST have an explicit representation or an explicit, verified derivation.»

No semantic fact may disappear accidentally between AST and IR.

---

181. IR Completion Contract

A feature is IR complete only when:

[ ] specification exists
[ ] syntax exists
[ ] lexer contract exists
[ ] parser contract exists
[ ] AST representation exists
[ ] semantic representation exists
[ ] IR representation exists
[ ] IR validation exists
[ ] diagnostics exist
[ ] source provenance exists where required
[ ] compatibility classification exists
[ ] migration exists where required
[ ] compiler lowering exists
[ ] runtime contract exists where applicable
[ ] target contract exists where applicable
[ ] positive tests exist
[ ] negative tests exist
[ ] boundary tests exist
[ ] scalability tests exist
[ ] determinism tests exist
[ ] hard-coding audit passes
[ ] safe-Rust audit passes

Only then may the feature be reported as fully implemented.

---

182. File Completion Contract

This file itself is complete when it defines:

Purpose

IR conformance.

Owns

Cross-layer IR compatibility and semantic-preservation rules.

Does not own

Backend algorithms, hardware, routing algorithms, QEC algorithms, scheduling algorithms, runtime implementation.

Inputs

Specification, AST, semantic contracts, version contracts.

Outputs

IR conformance requirements.

Upstream contracts

Specification, grammar, frontend, AST, semantics.

Downstream consumers

IR validator, compiler, optimizer, routing, scheduling, resilience, QEC, ZQN, HAL, runtime.

Integration

Explicitly defined in this document.

Tests

Explicitly defined in this document.

Scalability

Resource-driven, not hard-coded.

Safety

Rust 1.97.1, edition 2021, safe Rust only.

---

183. Repository Files That Must Integrate With This File

The following files/subsystems are explicitly connected:

grammar/DESIGN.md

grammar/README.md

grammar/Zamani.g4

grammar/grammar.md

grammar/Zamani-Grammar.md

grammar/specification/language-version.md

grammar/spec/lexical.md

grammar/spec/syntax.md

grammar/spec/semantics.md

grammar/spec/type-system.md

grammar/spec/resources.md

grammar/spec/quantum.md

grammar/spec/compatibility.md

grammar/compatibility/versions.md

grammar/compatibility/migrations.md

grammar/compatibility/deprecated.md

grammar/compatibility/reserved.md

grammar/compatibility/compatibility-matrix.md

grammar/compatibility/ast-conformance.md

grammar/compatibility/frontend-conformance.md

grammar/validation/

src/lexer.rs

src/parser.rs

src/ast/

src/semantic.rs

src/quantum/ir/mod.rs

src/quantum/ir/core/

src/quantum/ir/quantum/

src/quantum/ir/classical/

src/quantum/ir/control/

src/quantum/ir/model/

src/quantum/ir/program/

src/quantum/ir/pulse/

src/quantum/ir/resources/

src/quantum/ir/scheduling/

src/quantum/ir/analysis/

src/quantum/ir/validation/

src/quantum/ir/compatibility/

src/quantum/ir/dialect/

src/quantum/ir/metadata/

src/quantum/ir/hashing/

compiler/lowering

routing

scheduling

resilience

QEC

ZQN

HAL

runtime

Each remains the owner of its implementation.

This file defines their compatibility relationship.

---

184. Existing "Cargo.toml" Production Issue

The actual repository manifest currently contains an invalid Rust-version expression:

rust-version = "1.97" or "1.97.1"

Cargo does not accept that syntax.

The manifest MUST be corrected independently to one valid value.

For this production baseline:

rust-version = "1.97.1"

is the required representation.

This IR-conformance file MUST NOT be edited later merely because that manifest correction is made.

The contract is already complete: Rust 1.97.1 is the required baseline.

---

185. No Unsafe Integration Dependency

IR conformance MUST remain implementable entirely with safe Rust.

The following MUST NOT require "unsafe":

- IR construction;
- IR validation;
- IR hashing;
- IR serialization;
- IR migration;
- IR compatibility;
- IR canonicalization;
- quantum semantic lowering;
- classical semantic lowering;
- HDL semantic lowering;
- resource validation;
- capability validation.

---

186. Production Acceptance Gate

The IR subsystem is production-conformant only when:

Specification
     ↓
Grammar
     ↓
Lexer
     ↓
Parser
     ↓
AST
     ↓
Semantic Analysis
     ↓
Canonical IR
     ↓
IR Validation
     ↓
Optimization
     ↓
Routing / Scheduling / Resilience / QEC / ZQN
     ↓
HAL
     ↓
Backend
     ↓
Runtime

has no unclassified semantic divergence.

---

187. Production Hard-Coding Audit

The repository compatibility validator MUST search for suspicious universal limits including:

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
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_TIMELINES
MAX_PROGRAM_SIZE

and universal physical identifiers such as:

Qubit0
Qubit1
Qubit2
...

when used as the language's fundamental resource model.

A policy constant is permitted only when its ownership and purpose are explicitly documented as:

security policy
service policy
compiler resource policy
target capability

rather than:

language maximum

---

188. No Artificial Scale Ceiling

The final architecture MUST allow:

tiny
    ↓
small
    ↓
medium
    ↓
large
    ↓
very large
    ↓
resource-bounded arbitrary scale

without changing the source language merely because the resource quantity increases.

"Infinity" in POCO-REAF means no artificial language ceiling.

It does not claim that finite physical machines have infinite resources.

---

189. Final Semantic Boundary

The complete architecture is:

                         ZAMANI SOURCE
                              │
                              ▼
                    LANGUAGE SPECIFICATION
                              │
                              ▼
                        Zamani.g4
                              │
                              ▼
                           LEXER
                              │
                              ▼
                          PARSER
                              │
                              ▼
                       FRONTEND AST
                              │
                              ▼
                     SEMANTIC ANALYSIS
                              │
              ┌───────────────┼────────────────┐
              │               │                │
            Types          Effects        Resources
              │               │                │
              └───────────────┼────────────────┘
                              │
                              ▼
                  CANONICAL SEMANTIC MODEL
                              │
                              ▼
                     CANONICAL IR
                              │
          ┌───────────────────┼──────────────────┐
          │                   │                  │
      Classical          quantum::ir       HDL/Hardware
          │                   │                  │
          └───────────────────┼──────────────────┘
                              │
                              ▼
                        IR VALIDATION
                              │
                              ▼
                        OPTIMIZATION
                              │
             ┌────────────────┼────────────────┐
             │                │                │
          Routing         Scheduling       Resilience
             │                │                │
             └────────────────┼────────────────┘
                              │
                             QEC
                              │
                             ZQN
                              │
                             HAL
                              │
                              ▼
                     TARGET REALIZATION
                              │
          ┌────────────┬─────┼─────┬─────────────┐
          │            │     │     │             │
         CPU          GPU   FPGA   QPU       Future Target
          │            │     │     │             │
          └────────────┴─────┼─────┴─────────────┘
                              │
                              ▼
                           RUNTIME
                              │
                              ▼
                          EXECUTION

The central invariant is:

SOURCE SEMANTICS
       MUST
     SURVIVE
         ↓
LEXING
         ↓
PARSING
         ↓
AST
         ↓
SEMANTIC ANALYSIS
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
TARGET REALIZATION
         ↓
EXECUTION

The implementation may change.

The IR representation may evolve.

The compiler may change.

The runtime may change.

The target may change.

The hardware may change.

The topology may change.

The resource scale may change.

The computational paradigm may expand.

The canonical semantic meaning of a compatible program MUST NOT silently change.

---

190. Production Completion Checklist

This file is complete when all of the following remain true:

Authority

- [ ] "grammar/DESIGN.md" remains architectural authority.
- [ ] "grammar/specification/" remains normative language specification.
- [ ] "grammar/spec/" remains feature-contract authority.
- [ ] "grammar/Zamani.g4" remains canonical grammar composition root.
- [ ] "grammar/grammar.md" remains implementation-conformance reference.
- [ ] "grammar/Zamani-Grammar.md" remains historical/proposed/experimental material.
- [ ] this file is the canonical cross-layer IR-conformance contract.

Frontend

- [ ] lexer semantics are traceable.
- [ ] parser semantics are traceable.
- [ ] AST semantics are traceable.
- [ ] no semantic information is silently discarded.

IR

- [ ] canonical IR exists.
- [ ] IR validation exists.
- [ ] canonicalization is deterministic.
- [ ] serialization is deterministic.
- [ ] hashing is deterministic.
- [ ] provenance is preserved where required.
- [ ] version metadata is explicit.
- [ ] dialect metadata is explicit.
- [ ] compatibility is explicit.

Quantum

- [ ] "quantum::ir" remains canonical.
- [ ] no duplicate quantum semantic IR exists.
- [ ] generic quantum operations are representable.
- [ ] measurement semantics are preserved.
- [ ] classical feed-forward is preserved.
- [ ] logical/physical qubit identity remains distinct.
- [ ] QEC remains downstream.
- [ ] ZQN remains downstream.
- [ ] routing remains downstream.
- [ ] scheduling remains downstream.
- [ ] pulse semantics remain hardware-independent.

Classical

- [ ] classical computation uses the common IR foundation.
- [ ] numeric semantics are preserved.
- [ ] symbolic values are preserved where required.
- [ ] no artificial mathematical limits exist.

HDL/Hardware

- [ ] hardware intent is representable.
- [ ] physical realization is downstream.
- [ ] no universal register-width limit exists.
- [ ] no universal device-size limit exists.

Resources

- [ ] requirements are separate from capabilities.
- [ ] capabilities are separate from realization.
- [ ] policy limits are separate from language limits.
- [ ] actual hardware limits are target constraints.
- [ ] no artificial "MAX_*" language ceiling exists.

Compatibility

- [ ] language version is distinct from IR version.
- [ ] IR version is distinct from compiler version.
- [ ] dialect version is distinct from language version.
- [ ] target version is distinct from language version.
- [ ] migrations are explicit.
- [ ] lossy migrations are identified.
- [ ] deprecated forms are identified.
- [ ] unsupported versions fail explicitly.
- [ ] no silent semantic fallback exists.

Safety

- [ ] Rust 1.97.1 is supported.
- [ ] Rust 2021 is used.
- [ ] no "unsafe" is required.
- [ ] overflow-sensitive accounting is checked.
- [ ] untrusted IR is validated.
- [ ] resource exhaustion is distinguished from invalid source.

Testing

- [ ] positive tests exist.
- [ ] negative tests exist.
- [ ] boundary tests exist.
- [ ] scalability tests exist.
- [ ] deterministic tests exist.
- [ ] compatibility tests exist.
- [ ] quantum tests exist.
- [ ] classical tests exist.
- [ ] HDL tests exist.
- [ ] hybrid tests exist.
- [ ] resource/capability tests exist.
- [ ] serialization round-trip tests exist.
- [ ] canonical hashing tests exist.
- [ ] migration tests exist.

---

191. Final Invariant

The canonical Zamani IR exists to preserve meaning, not to encode the limitations of today's machines.

Therefore:

ONE PROGRAM
    ↓
ONE STABLE SEMANTIC MEANING
    ↓
CANONICAL IR
    ↓
MANY OPTIMIZATIONS
    ↓
MANY LOWERINGS
    ↓
MANY TARGETS
    ↓
MANY HARDWARE CONFIGURATIONS
    ↓
MANY RESOURCE SCALES
    ↓
FUTURE COMPUTATIONAL SUBSTRATES

subject to:

semantic correctness
+
available capabilities
+
available resources
+
implementation support
+
physical reality

and never through an artificial grammar or IR ceiling.

The definitive rule is:

«Zamani IR describes what the program means; it does not decide how large the machine must be.»

That separation is the IR foundation required for:

From Atom → Embedded → CPU → Multicore → GPU → FPGA → ASIC
→ QPU → Accelerator → HPC → Cluster → Distributed/Cloud
→ Future Computing

and for:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

End of "grammar/compatibility/ir-conformance.md".