Zamani Grammar Scalability Rules

Path: "grammar/validation/scalability-rules.md"
Status: Normative production specification
Language: Zamani
Purpose: Universal, resource-parametric language scalability
Minimum Rust: Rust 1.97 / 1.97.1
Safety: "unsafe" Rust is forbidden
Primary execution model: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

This document defines the mandatory scalability rules for the Zamani grammar, parser, AST boundary, semantic validation, and their integration with the rest of the Zamani compiler.

The purpose is to ensure that Zamani can express programs ranging from the smallest computation to computations of arbitrarily large size without introducing accidental language-level limits.

The fundamental rule is:

«Zamani source describes computation and semantic intent. It MUST NOT impose arbitrary limits derived from the machine currently available.»

The grammar MUST therefore remain:

- machine-size independent;
- topology independent unless topology is explicitly part of program semantics;
- backend independent;
- resource parametric;
- deterministic;
- extensible;
- versionable;
- safe;
- compatible with Rust 1.97/1.97.1;
- free of Rust "unsafe";
- compatible with canonical IR ownership;
- suitable for quantum, classical, hybrid, HDL, hardware, distributed, AI, data, networking, security, and future domains.

This document is normative.

Words MUST, MUST NOT, REQUIRED, SHOULD, SHOULD NOT, and MAY are normative.

---

2. Fundamental Scalability Model

Zamani uses the following model:

Program
  ↓
Portable semantic meaning
  ↓
Target-independent representation
  ↓
Target capabilities
  ↓
Resource availability
  ↓
Compilation / lowering
  ↓
Execution

The source program does not become a different program merely because the execution target changes.

For example, the same semantic program MAY eventually execute on:

one CPU
many CPUs
GPU
many GPUs
FPGA
ASIC
embedded processor
quantum processor
quantum simulator
CPU + GPU
CPU + QPU
FPGA + CPU
cluster
supercomputer
cloud
edge
distributed infrastructure
future hardware

provided that the target satisfies the program's semantic requirements or the compiler can legally transform the program into an equivalent implementation.

---

3. Definition of "Infinity"

"Infinity" in Zamani means:

«There is no artificial language-defined upper bound on scalable quantities.»

It does not mean that a finite machine can physically allocate infinite resources.

Therefore:

language limit
≠
compiler implementation limit
≠
host representation limit
≠
resource availability
≠
target capability
≠
runtime budget

These MUST remain separate.

For example:

MAX_QUBITS = 1024

is prohibited as a language-wide semantic limit.

However, a particular execution environment MAY report:

available_qubits = 127

and a compilation attempt MAY fail because the requested computation cannot be mapped to that environment.

That is a target/resource failure, not a grammar failure.

---

4. Scope

These rules apply to:

grammar/Zamani.g4

grammar/lexer/*
grammar/core/*
grammar/types/*
grammar/expressions/*
grammar/statements/*
grammar/declarations/*
grammar/functions/*
grammar/modules/*
grammar/effects/*
grammar/memory/*
grammar/concurrency/*
grammar/classical/*
grammar/quantum/*
grammar/hybrid/*
grammar/hdl/*
grammar/hardware/*
grammar/distributed/*
grammar/ai/*
grammar/data/*
grammar/networking/*
grammar/security/*
grammar/resources/*
grammar/compile/*
grammar/execution/*
grammar/interoperability/*
grammar/dialects/*
grammar/macros/*
grammar/metaprogramming/*

grammar/validation/*
grammar/tests/*
grammar/examples/*

and to the compiler components that consume their output.

The rules also govern integration with:

lexer
parser
AST
semantic analysis
type system
effect system
resource system
classical IR
quantum::ir
optimization
routing
scheduling
ZQN
QEC
hardware abstraction
resilience
runtime
interoperability

---

5. Ownership Boundary

The grammar owns syntax.

The semantic layer owns meaning.

The IR owns canonical computational representation.

The target system owns physical realization.

The runtime owns actual execution conditions.

The following separation is mandatory:

Layer| Owns| MUST NOT Own
Lexer| lexical structure| machine capabilities
Parser| syntactic structure| target selection
AST| source representation| physical topology
Semantic analysis| program meaning and validity| runtime discovery
Type system| type meaning| device calibration
Capability analysis| required capabilities| arbitrary backend choice
Resource analysis| program resource requirements| machine inventory
Canonical IR| computational semantics| provider-specific execution policy
Optimization| legal transformations| source syntax
Routing| physical mapping| language meaning
Scheduling| order/timing/resource allocation| source grammar
ZQN| noise/fault semantics| parser syntax
QEC| error detection/correction| grammar ownership
Hardware HAL| target capabilities/state| source semantics
Resilience| adaptation/recovery decisions| grammar
Runtime| execution| language definition

---

6. No Machine-Derived Grammar Limits

The grammar MUST NOT contain limits derived from:

- number of CPUs;
- number of cores;
- number of threads;
- number of GPUs;
- number of FPGAs;
- number of ASIC units;
- number of QPUs;
- number of qubits;
- number of logical qubits;
- number of physical qubits;
- memory size;
- register count;
- cache size;
- vector width;
- tensor dimension;
- cluster size;
- node count;
- network size;
- topology size;
- accelerator count;
- device count;
- storage capacity;
- address-space size;
- scheduler capacity;
- backend-specific queue capacity.

Prohibited examples include:

const MAX_QUBITS: usize = 32;
const MAX_CORES: usize = 64;
const MAX_DEVICES: usize = 16;
const MAX_NODES: usize = 1024;

when these constants determine what valid Zamani programs may be represented.

---

7. What Is Allowed to Be Bounded

Not every bound is forbidden.

A bound is allowed when it is a genuine property of the relevant abstraction.

Examples include:

7.1 Language-defined finite syntax

A token can have a finite representation.

A literal can have a defined lexical structure.

An identifier can have a grammar-defined character policy.

These are syntax rules, not machine-capacity rules.

7.2 Type-defined ranges

A type MAY have a mathematically defined range.

For example:

u8
i32
f64

may have defined representation semantics.

Overflow MUST be diagnosed or handled according to the type semantics.

It MUST NOT silently wrap merely because the parser's host integer type overflowed.

7.3 Explicit program requirements

A program MAY explicitly require a resource quantity:

requires qubits >= n

or equivalent future Zamani syntax.

The value is part of program semantics.

It is not a compiler-wide maximum.

7.4 Target limits

A backend MAY report:

available_memory
available_qubits
supported_vector_width
maximum_message_size

These are target facts.

They MUST NOT be copied into the language grammar as universal limits.

7.5 Security/resource budgets

The compiler MAY expose configurable limits for:

- maximum source bytes processed in one invocation;
- maximum diagnostics emitted;
- maximum compilation time;
- maximum expansion work;
- maximum memory consumption;
- maximum macro expansion;
- maximum generated IR;
- maximum optimization work.

Such limits MUST be:

1. implementation controls;
2. externally configurable where appropriate;
3. distinguishable from language validity;
4. reported as resource/budget failures;
5. absent from the language's semantic definition.

---

8. Program Size

Zamani MUST NOT define an artificial maximum number of:

- source statements;
- declarations;
- functions;
- modules;
- expressions;
- types;
- quantum operations;
- classical operations;
- hardware instances;
- nodes;
- tasks;
- channels;
- resources;
- data records.

Grammar constructs SHOULD therefore use repetition and compositional structures rather than finite enumerations.

Conceptually:

program
    : item* EOF
    ;

is scalable.

A construct equivalent to:

program
    : item item? item? item? ...
    ;

is not an acceptable representation of scalable multiplicity.

---

9. Collection Scalability

All repeated language constructs MUST be modeled as collections whose cardinality is determined by the program.

Examples:

functions
modules
imports
parameters
arguments
qubits
operations
controls
targets
ports
signals
nodes
services
tensor dimensions
data fields
pattern alternatives
match arms
resource requirements
capabilities
effects

The grammar MUST NOT encode arbitrary finite collection sizes.

For example:

targetList
    : expression (',' expression)*
    ;

is preferred over:

targetList
    : expression
    | expression ',' expression
    | expression ',' expression ',' expression
    ;

---

10. Recursive Structure

Zamani MUST support recursively compositional language structures where semantics require them.

Examples include:

expressions
types
generic types
nested modules
blocks
patterns
functions
higher-order functions
data structures
quantum controls
hardware hierarchy
distributed topology descriptions

However, recursive grammar design MUST NOT create unnecessary parser stack exhaustion.

The implementation SHOULD distinguish:

semantic recursion

from:

implementation recursion

A recursive language structure does not require a recursively implemented Rust algorithm.

---

11. Deep-Structure Handling

For potentially unbounded nesting, implementations SHOULD use explicit worklists or stacks where practical.

For example:

AST
 ↓
explicit traversal stack
 ↓
semantic processing

rather than assuming:

AST
 ↓
unbounded Rust call stack

The parser and semantic analyzer MUST NOT introduce arbitrary language limits merely because a particular recursive implementation would overflow the host stack.

If an implementation requires an operational depth budget for safety, that budget MUST be:

- external to language semantics;
- configurable;
- documented;
- reported as an implementation/resource error;
- tested independently from semantic validity.

Rust "unsafe" MUST NOT be introduced to bypass these constraints.

---

12. Quantum Scalability

Quantum syntax MUST be resource-parametric.

There MUST be no language-level maximum such as:

MAX_QUBITS
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS
MAX_REGISTER_SIZE
MAX_CIRCUIT_WIDTH
MAX_CIRCUIT_DEPTH
MAX_GATE_COUNT

unless a limit belongs to a deliberately bounded external interchange format rather than Zamani itself.

A source program MAY express:

register[n]

where "n" is a program-defined semantic value.

The compiler MUST NOT reinterpret this as:

n <= compiler_constant

unless the compilation context explicitly imposes such a resource limit.

---

13. Quantum Identity Scalability

Quantum identifiers MUST remain logical and target-independent by default.

For example:

logical qubit

is different from:

physical qubit 17

The former is portable program semantics.

The latter is a physical target binding.

Physical identifiers MUST NOT become the default representation of quantum computation.

When physical mapping is required:

logical quantum program
    ↓
canonical quantum::ir
    ↓
routing
    ↓
physical mapping

The grammar MUST NOT bypass this boundary.

---

14. Canonical Quantum IR

The grammar MUST NOT define a second quantum IR.

The frontend MAY create syntax-specific AST nodes such as:

QuantumOperationSyntax
QuantumRegisterSyntax
QuantumMeasurementSyntax

but these are source representations.

They MUST eventually lower into the repository's canonical "quantum::ir".

The grammar MUST NOT define competing representations of:

QubitId
PhysicalQubitId
QuantumGate
QuantumOperation
Circuit
QuantumProgram

when those concepts are already owned by the canonical quantum IR.

This prevents scalability failures caused by duplicated semantic models.

---

15. Quantum Operation Vocabulary

The language MUST NOT require an ever-growing finite list of every possible quantum operation.

A small standardized vocabulary MAY exist for fundamental language constructs.

Additional operations SHOULD be expressible through:

named operations
parameterized operations
operation declarations
imports
dialects
intrinsics
libraries
semantic capabilities

rather than repeatedly modifying the core grammar.

The architecture should therefore permit:

known operation
future operation
user-defined operation
domain-specific operation
vendor operation
experimental operation

without changing the fundamental scalability model.

---

16. Classical Scalability

Classical constructs MUST scale without fixed limits on:

variables
functions
types
generic parameters
array lengths
vector lengths
matrix dimensions
tensor dimensions
tasks
threads
processes
actors
channels
modules
data structures

The grammar MUST describe these structures generically.

The compiler and runtime determine whether the target can realize them.

---

17. Tensor and Multidimensional Data Scalability

Tensor syntax MUST NOT impose an arbitrary maximum rank.

Do not encode:

rank <= 4
rank <= 8
rank <= 16

unless such a bound is an explicit semantic property of a particular type or interchange format.

Prefer:

tensor<element, shape>

where "shape" is itself a compositional semantic structure.

The number of dimensions belongs to the program's type/data semantics.

The physical representation belongs to the backend.

---

18. HDL Scalability

HDL constructs MUST scale to arbitrary hierarchy and resource counts.

The grammar MUST NOT impose fixed limits on:

modules
ports
signals
wires
registers
processes
pipeline stages
states
memory banks
interfaces
instances
clock domains
hardware parameters

For example:

module M {
    ...
}

may contain a collection of ports whose size is determined by the source program.

A compiler MAY later reject a hardware design because a target lacks sufficient resources.

That rejection MUST occur during target analysis, not because the grammar arbitrarily refuses to parse the design.

---

19. Hardware Parameters

Hardware parameters MUST be classified.

Semantic hardware parameter

A parameter changes the meaning of the hardware design.

Example:

width = N

Target parameter

A parameter describes a particular implementation.

Example:

target_frequency = ...

Calibration parameter

A parameter belongs to physical calibration.

Deployment parameter

A parameter belongs to placement or deployment.

These categories MUST NOT be conflated.

A hardware grammar construct SHOULD carry enough semantic classification to prevent a target-specific fact from becoming universal language meaning.

---

20. Distributed Scalability

Distributed constructs MUST NOT assume a fixed number of nodes.

Prohibited:

cluster<8>

as a universal semantic ceiling.

Allowed:

cluster<n>

where "n" is a program requirement or runtime-selected quantity.

The program MAY express:

replicas = desired_count

but the runtime determines actual placement subject to constraints.

Distributed syntax MUST therefore separate:

logical participants

from:

physical nodes

and:

deployment instances

---

21. Concurrency Scalability

Concurrency syntax MUST NOT assume a fixed number of:

threads
tasks
workers
actors
channels
executors
queues

The source describes concurrency semantics.

The runtime determines actual execution resources.

For example:

parallel for item in items

must not mean:

exactly 8 threads

unless that exact number is explicitly part of the program's semantics.

---

22. Parallelism and Resource Independence

The grammar MUST distinguish:

parallelism intent

from:

physical parallel resource count

Examples:

parallel

may express that iterations are independent.

It MUST NOT silently select:

4 CPU cores

or:

128 GPU lanes

The scheduler, compiler, and runtime determine an appropriate realization.

---

23. Memory Scalability

Memory constructs MUST NOT hard-code:

maximum heap size
maximum stack size
maximum object count
maximum allocation count
maximum address

The source may express semantic memory requirements.

The target determines whether those requirements can be satisfied.

Memory layout decisions belong to later compilation stages unless the language construct explicitly defines layout semantics.

---

24. Resource Model

Zamani MUST maintain a universal distinction between:

resource
requirement
constraint
capability
preference
hint
target
placement
performance
latency
energy
reliability
portability

These MUST NOT collapse into one concept.

For example:

requires quantum

means:

«the computation requires quantum capabilities.»

It MUST NOT mean:

use provider X
use device Y
use N physical qubits
use topology Z

---

25. Requirements

A requirement expresses something necessary for correct execution.

Examples include:

requires quantum
requires floating_point
requires secure_storage
requires network
requires coherent_control

Requirements MUST be evaluated against capabilities.

A requirement MUST NOT secretly become a machine selection.

---

26. Capabilities

Capabilities describe what an execution target can provide.

Examples:

quantum
gpu
fpga
distributed
secure_execution
dynamic_circuit
fault_tolerance
vectorization

Capabilities belong to the target/environment model.

They MUST NOT be hard-coded into the grammar as available machine facts.

---

27. Constraints

Constraints limit valid implementations of a program.

Examples:

latency <= L
energy <= E
memory <= M
reliability >= R

A constraint is different from a fixed machine property.

A constraint MAY cause a compilation failure if no valid implementation satisfies it.

It MUST NOT restrict the grammar to a particular machine.

---

28. Preferences

Preferences are non-mandatory optimization guidance.

Examples:

prefer low_latency
prefer energy_efficiency
prefer gpu
prefer local_execution

A preference MUST NOT be treated as a semantic requirement unless the language specification explicitly defines it as such.

---

29. Hints

Hints provide optional optimization information.

A hint MUST NOT alter program meaning.

Therefore:

hint prefer_gpu

MUST NOT make the program semantically dependent on a GPU.

If the target lacks a GPU, the compiler MAY ignore the hint.

---

30. Target Binding

Target binding is fundamentally different from portable source semantics.

A source program MAY deliberately contain a target binding in an explicitly target-bound context.

For example:

target-specific {
    ...
}

or an equivalent future mechanism.

Such constructs MUST be explicitly classified as non-portable or conditionally portable.

The compiler MUST NOT accidentally infer target binding merely from the use of a hardware-related concept.

---

31. No Hidden Target Selection

The grammar MUST NOT infer a device from:

gate name
qubit count
data type
parallelism
clock frequency
accelerator keyword
quantum keyword
HDL keyword

For example:

quantum

must not mean:

IBM device

and:

gpu

must not mean:

NVIDIA device

unless the source explicitly enters a target-binding mechanism.

---

32. POCO-REAF Invariant

The following property MUST hold:

same source
+
same language version
+
same semantic environment
=
same program meaning

Changing the target MUST NOT require rewriting the program merely because:

qubit count changed
CPU count changed
GPU count changed
memory changed
topology changed
device changed
scheduler changed

provided that the new target can satisfy the program's requirements.

---

33. Compile-Once Requirement

POCO-REAF requires a careful distinction between:

semantic compilation

and:

physical target lowering

A reusable compiled artifact SHOULD preserve:

language version
semantic identity
type information
resource requirements
capability requirements
provenance
portable IR

Target-specific artifacts MAY additionally contain:

machine code
native quantum gates
routing
scheduling
placement
calibration
deployment information

A target-specific artifact MUST NOT be confused with the portable semantic compilation result.

---

34. Target Specialization

Specialization is allowed when it preserves semantics.

For example:

portable operation
    ↓
GPU implementation

or:

portable quantum operation
    ↓
native QPU decomposition

or:

parallel algorithm
    ↓
N-worker execution

Specialization MUST be a refinement of the same semantic computation.

It MUST NOT silently change:

observable results
side effects
memory semantics
measurement semantics
quantum semantics
ordering guarantees
security guarantees

unless the language explicitly permits the behavior.

---

35. Scaling Across Quantum Hardware

The quantum compilation pipeline MUST remain:

Zamani source
    ↓
AST
    ↓
semantic analysis
    ↓
canonical quantum::ir
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
ZQN/noise-aware analysis
    ↓
hardware lowering
    ↓
runtime

The grammar MUST NOT perform:

routing
scheduling
calibration
device discovery
noise estimation

as part of parsing.

---

36. ZQN Boundary

ZQN owns noise/fault semantics.

The grammar MAY express:

noise assumptions
fault tolerance requirements
noise-related constraints

where those are language features.

The grammar MUST NOT:

- discover device noise;
- inspect calibration data;
- invent noise models;
- encode provider-specific noise values;
- perform noise-aware scheduling.

Those responsibilities belong downstream.

---

37. QEC Boundary

The grammar MAY express semantic QEC intent.

For example:

requires error_correction

or equivalent language constructs.

However, grammar MUST NOT own QEC algorithms.

It MUST NOT hard-code:

surface-code distance
specific decoder
specific stabilizer schedule
specific physical layout

unless those are explicitly expressed in a target/domain-specific program.

QEC remains a downstream semantic/compilation subsystem.

---

38. Scheduling Boundary

Timing syntax is allowed when timing has program meaning.

Examples:

deadline
ordering
synchronization
latency constraint
clock relation
temporal dependency

However, the grammar MUST NOT hard-code:

gate_duration = 20ns
clock = 5GHz
scheduler_tick = 1ns

as universal machine facts.

Hardware-specific durations belong to hardware capability/calibration data.

---

39. Routing Boundary

Logical resource relationships belong to program semantics.

Physical topology belongs to the target.

Therefore:

logical communication requirement

is portable.

physical qubit 17 connected to physical qubit 18

is target-specific.

The routing subsystem owns the transformation between them.

---

40. Resilience Boundary

Resilience is an adaptation/orchestration subsystem.

Grammar MAY express:

fault tolerance requirements
recovery policies
availability requirements
reliability constraints

where these are language semantics.

Grammar MUST NOT implement:

retry
restart
rollback
reroute
recompile
backend switching
quarantine
recovery orchestration

Those belong to resilience/runtime layers.

---

41. Mathematical Scalability

The grammar MUST avoid an ever-growing list of special mathematical keywords.

Prefer:

generic operators
generic functions
typed intrinsics
standard library functions
domain libraries
dialects

over continuously expanding:

fft
ifft
svd
eig
integrate
differentiate
...

as mandatory grammar keywords.

Mathematical expressiveness MUST therefore scale through the type system and semantic operation model.

---

42. AI/ML Scalability

AI constructs MUST NOT impose fixed:

tensor rank
batch size
model size
parameter count
layer count
accelerator count
dataset size
worker count

unless explicitly required by a type or semantic construct.

AI syntax MUST separate:

model semantics

from:

execution backend

and:

hardware accelerator

---

43. Networking Scalability

Networking constructs MUST NOT impose fixed:

node counts
connection counts
endpoint counts
message counts
network topology sizes

A network abstraction represents logical communication.

Physical network realization belongs to deployment/runtime infrastructure.

---

44. Security Scalability

Security constructs MUST NOT hard-code a finite number of:

identities
keys
permissions
principals
policies
resources
domains

Cryptographic algorithms MUST be represented through stable semantic interfaces and versioned capabilities rather than making the grammar an unbounded registry.

---

45. Genericity

Generics MUST be preferred whenever a construct varies over:

size
type
resource
architecture
domain
precision
parallelism
execution strategy

Example:

fn process<T, N>(...)

should represent a family of computations rather than requiring a separate grammar rule for each size.

---

46. Compile-Time Values

Compile-time values MAY determine program structure.

Examples:

N
M
shape
width
count
precision

However, compile-time evaluation MUST NOT introduce accidental fixed machine limits.

If:

N = 1_000_000

is valid semantic input, the parser must not reject it merely because an implementation happened to use a smaller internal representation.

Semantic integer conversion MUST be checked explicitly.

---

47. Numeric Scalability

Numeric literals MUST NOT be silently truncated.

The implementation MUST distinguish:

lexical representation
semantic numeric value
target representation

For example:

123456789012345678901234567890

must not silently become a smaller value merely because the host parser used a fixed-width integer.

Possible outcomes include:

valid arbitrary-precision semantic value

or:

explicit diagnostic that the selected semantic type cannot represent it

but never silent corruption.

---

48. Address and Identifier Scalability

Identifiers MUST be symbolic.

The grammar MUST NOT require that an identifier fit a hardware address.

Examples:

qubit
node
device
memory
resource
channel
service
accelerator

must remain semantic identifiers.

Physical addresses belong to target/deployment layers.

---

49. Generic Resource Quantities

Resource quantities SHOULD support symbolic expressions where semantically useful.

For example:

memory >= N * element_size

is preferable to requiring a literal byte count.

Likewise:

qubits >= required_qubits

is preferable to encoding a universal maximum.

This allows resource requirements to scale with the algorithm.

---

50. No Fixed Topology Assumptions

The grammar MUST NOT assume:

line topology
grid topology
ring topology
all-to-all topology
fixed cluster topology
fixed quantum connectivity

unless the topology itself is intentionally part of a target-specific or hardware-description program.

Portable programs express logical relationships.

Routing maps them to physical topology.

---

51. Hardware Description Exception

HDL may intentionally describe physical structure.

That is not accidental hard-coding if the physical structure is itself the subject of the program.

For example:

hardware {
    ...
}

may explicitly describe:

ports
wires
clock domains
registers
pipeline stages

The scalability rule is:

«A hardware design MAY specify physical structure when physical structure is the computation being described; it MUST NOT accidentally turn one implementation's resource limits into a universal language limit.»

---

52. Dialect Scalability

Dialects MUST be isolated by namespace/version/capability.

A dialect MUST NOT modify the meaning of existing core syntax silently.

A dialect MAY introduce:

new syntax
new operations
new attributes
new types
new target bindings

provided that:

- it is explicitly identified;
- its version is known;
- conflicts are detected;
- unsupported dialects produce diagnostics;
- dialect syntax cannot silently redefine core semantics.

---

53. Future-Feature Scalability

The grammar MUST reserve extensibility mechanisms for future domains.

Future computing models may include technologies not currently known.

Therefore the architecture SHOULD support:

domain declarations
dialects
capability namespaces
versioned extensions
operation declarations
type extensions
effect extensions
resource extensions

The core grammar MUST remain small enough that new domains do not require rewriting unrelated language foundations.

---

54. No Keyword Explosion

A new feature SHOULD NOT become a keyword merely because it is important.

Use identifiers and semantic registries when lexical distinction is unnecessary.

This is especially important for:

quantum gates
mathematical operations
AI operators
hardware devices
vendor technologies
future accelerators
protocols
algorithms

A finite keyword list MUST represent stable language syntax, not every available technology.

---

55. Macro Scalability

Macros MUST NOT create unbounded expansion without controlled compilation resources.

Macro systems MUST distinguish:

semantic language size

from:

expansion work

A compiler MAY impose configurable expansion budgets.

It MUST NOT claim that a source program is semantically invalid merely because a particular compilation budget was exceeded.

Macro expansion MUST be deterministic for identical controlled inputs.

---

56. Metaprogramming Scalability

Compile-time computation MAY generate arbitrarily large structures subject to available resources.

The compiler MUST provide explicit resource accounting for:

time
memory
generated nodes
generated source/IR
recursion/work depth

These are implementation budgets.

They are not language semantic ceilings.

---

57. Source File Scalability

The language MUST NOT impose an arbitrary maximum source-file size.

Tooling MAY impose configurable limits.

Large source programs SHOULD be decomposable through:

modules
packages
imports
generated code
libraries
dialects

The semantic model must remain equivalent regardless of whether a program is represented as one source file or many modules, subject to module semantics.

---

58. Module Scalability

There MUST be no fixed maximum number of:

modules
packages
imports
exports
dependencies
namespaces

Module resolution SHOULD use efficient indexed structures rather than repeatedly scanning the entire source graph.

Where deterministic ordering is required, it MUST be explicitly defined.

---

59. Compiler Algorithm Scalability

Compiler algorithms MUST avoid unnecessary complexity growth.

Implementations SHOULD prefer:

hash maps
indexed symbol tables
interning where appropriate
incremental analysis
memoization
dependency graphs
worklists
streaming processing

where these improve scalability without compromising determinism.

Avoid accidental:

O(n²)
O(n³)

behavior for operations that naturally admit indexed or graph-based solutions.

Algorithmic complexity is an implementation concern, but poor algorithms MUST NOT be compensated for by artificial language limits.

---

60. Memory Scalability of the Compiler

The compiler MUST NOT assume that:

program size <= small fixed memory

Internal representations SHOULD avoid unnecessary duplication.

The implementation SHOULD:

- retain source spans without duplicating source text unnecessarily;
- share immutable structures where appropriate;
- avoid copying large AST subtrees;
- process independent units incrementally where possible;
- release temporary representations after their consumers complete;
- avoid retaining target-specific data during target-independent phases.

---

61. Streaming and Incremental Processing

Where practical, lexer/parser/tooling components SHOULD support large inputs incrementally.

The language semantics MUST NOT depend on the complete source being resident in memory simultaneously.

However, if a semantic feature genuinely requires whole-program information, that requirement MUST be explicit.

---

62. Deterministic Scalability

Scaling MUST NOT introduce nondeterminism.

Given identical:

source
language version
dialect versions
semantic environment
compiler configuration

the following MUST be deterministic:

tokenization
parse tree
AST
name resolution
semantic diagnostics
type checking
capability analysis
resource-expression evaluation
canonical semantic identity

Map/set iteration order MUST NOT accidentally determine language behavior.

Where ordering matters, the compiler MUST use deterministic ordering.

---

63. Parallel Compiler Execution

Compiler phases MAY execute in parallel.

Parallel compilation MUST preserve deterministic results.

Therefore:

parallel execution

MUST NOT mean:

random ordering

or:

race-dependent diagnostics

Independent compilation units MAY be processed concurrently, then merged through deterministic ordering rules.

---

64. Diagnostics Scalability

Diagnostics MUST scale with source size.

A malformed large program MUST NOT cause:

unbounded diagnostic spam

The compiler MAY use configurable diagnostic budgets.

However, the primary diagnostic MUST remain deterministic and actionable.

Diagnostics SHOULD include:

stable error code
severity
source span
message
related span
help
machine-readable category

---

65. Error Classification

Scalability failures MUST be distinguished from semantic failures.

Examples:

invalid syntax

is a syntax error.

type mismatch

is a semantic/type error.

target has insufficient memory

is a target/resource error.

macro expansion budget exhausted

is an implementation/resource-budget error.

compiler internal invariant violated

is an internal/compiler error.

These MUST NOT be conflated.

---

66. No Parser-Side Resource Discovery

The parser MUST NOT:

- query hardware;
- inspect QPU topology;
- discover CPU count;
- discover GPU count;
- read calibration;
- contact a network service;
- query runtime state;
- select a device.

Parsing must remain target-independent.

---

67. No Filesystem-Dependent Semantics

Grammar parsing MUST NOT require arbitrary filesystem access.

Imports and modules MAY be resolved by the module-resolution subsystem.

The parser itself MUST NOT decide language meaning based on uncontrolled filesystem state.

Controlled source/module inputs MAY be supplied by the compiler driver.

---

68. No Network-Dependent Semantics

Parsing and semantic validation MUST NOT depend on network availability.

A package registry, remote dependency system, or distributed build system may exist outside the grammar.

Network state MUST NOT silently change the meaning of a valid source program.

---

69. No Time-Dependent Semantics

Parsing and semantic analysis MUST NOT depend on wall-clock time.

Source such as:

now()

may be a runtime operation.

It MUST NOT influence parsing or semantic identity merely because the compiler was executed at a different time.

---

70. No Randomness-Dependent Semantics

The parser MUST NOT use uncontrolled randomness.

Any explicitly language-defined compile-time random operation MUST have a deterministic seed/input model if reproducible compilation is required.

---

71. No Unsafe Rust

All grammar infrastructure MUST compile without Rust "unsafe".

This includes:

lexer
parser
AST
semantic analysis
grammar validation
tests
tooling
macro expansion
generated glue used by the grammar subsystem

Rust 1.97 / 1.97.1 compatibility MUST be maintained.

Unsafe code MUST NOT be introduced as a scalability shortcut.

---

72. Host Integer Independence

The implementation MUST distinguish:

Rust host integer width

from:

Zamani semantic integer model

Using "usize" internally is allowed where the value represents an in-memory collection index.

It MUST NOT be used to define the semantic maximum of a Zamani program.

For example:

Vec<T>

naturally has a host/platform allocation limit.

That is not equivalent to:

Zamani arrays may contain at most usize::MAX elements

as a language semantic rule.

---

73. Collection Indexing

Indices used to access host collections MAY use Rust-native indexing internally.

The compiler MUST validate conversions carefully.

A source-level index MUST NOT silently truncate during conversion to an implementation index.

Overflow MUST produce an appropriate diagnostic or controlled failure.

---

74. Size Expressions

Size expressions SHOULD be first-class semantic expressions where required.

Examples:

N
N + 1
2 * N
shape.length
resource.available

The grammar MUST not require all sizes to be literal constants.

---

75. Dynamic Sizes

Where the language supports dynamic sizes, semantic analysis MUST distinguish:

compile-time known size
runtime known size
symbolic size
unknown size

The compiler MUST NOT assume that all scalable structures are statically fixed.

---

76. Quantum Dynamic Sizes

Quantum systems require special care.

A runtime-dependent quantum resource count MUST be represented only where the target execution model supports it.

The grammar MAY express dynamic quantum control/resource semantics.

Semantic analysis and target capability checking determine whether a given target can realize them.

The grammar MUST NOT reject all dynamic quantum programs merely because one backend requires static allocation.

---

77. HDL Dynamic Parameters

HDL synthesis may require compile-time elaboration.

Therefore HDL parameters MAY need static resolution before synthesis.

That is a property of the HDL compilation model.

It MUST NOT become a global grammar limitation.

---

78. Resource Expressions Must Be Typed

Resource expressions SHOULD have semantic units where appropriate.

Examples:

memory
time
energy
bandwidth
latency
qubits
cores
storage

The compiler MUST avoid treating:

10

as universally interchangeable between different resource dimensions.

---

79. Units

Units SHOULD be represented semantically rather than through ad hoc keyword proliferation.

Examples:

10 ns
5 GHz
2 GiB
100 MB/s

The semantic layer determines dimensional correctness.

The target layer determines whether a requested quantity is achievable.

---

80. Scalability and Optimization

Optimization MUST preserve semantic meaning.

An optimizer MAY change:

operation count
instruction count
layout
parallelism
schedule
representation

but MUST NOT use optimization as an excuse to impose source-language resource limits.

Optimization failure is not grammar invalidity.

---

81. Scalability and Scheduling

Scheduling MAY adapt to:

machine width
resource availability
latency
topology
calibration
noise
power
energy

The source grammar remains unchanged.

The scheduler MUST consume semantic/resource information rather than modifying source meaning.

---

82. Scalability and Hardware Discovery

Hardware discovery belongs outside the grammar.

The hardware subsystem MAY discover:

qubit count
CPU count
GPU count
memory
topology
supported operations
clock
calibration
noise

The resulting capability model is supplied to compilation.

The grammar does not change.

---

83. Scalability and Runtime Discovery

Runtime-discovered facts MAY affect execution strategy.

They MUST NOT silently alter the language semantics.

For example:

runtime has 2 GPUs

may change execution placement.

It MUST NOT change what a Zamani expression means.

---

84. Semantic Equivalence

Two target implementations are considered valid alternatives when they preserve the relevant semantic contract.

Examples:

CPU implementation
GPU implementation
FPGA implementation
QPU implementation
distributed implementation

may differ internally while preserving observable program semantics.

The compiler MUST document exceptions where execution is intentionally nondeterministic or resource-dependent.

---

85. Observable Behavior

Scalability transformations MUST preserve:

return values
observable memory effects
defined ordering
exceptions/errors
security guarantees
quantum measurement semantics
specified timing semantics
communication semantics
resource guarantees

unless the language explicitly defines those aspects as nondeterministic or implementation-dependent.

---

86. Nondeterminism

Nondeterminism MUST be explicit.

Parallel execution, distributed execution, quantum measurement, or randomized algorithms MAY be nondeterministic according to language semantics.

Scaling MUST NOT accidentally introduce new nondeterminism into constructs that were previously deterministic.

---

87. Distributed Determinism

Changing:

node count
network topology
worker placement

MUST NOT change deterministic semantics unless the program explicitly depends on those properties.

Distributed implementation MAY produce different execution traces while producing the same specified result.

---

88. Quantum Measurement

Quantum measurement is inherently part of quantum semantics.

The grammar MUST distinguish:

measurement as program operation

from:

backend measurement implementation

Changing physical qubit layout MUST NOT change the intended measurement semantics.

---

89. Resource Failure

When resources are insufficient, the compiler/runtime MUST report a resource failure rather than rewriting the program silently.

Examples:

insufficient qubits
insufficient memory
unsupported operation
insufficient accelerator capacity
unavailable network capability
timing constraint unsatisfied

The implementation MUST NOT silently reduce program size to fit a target unless the language explicitly defines such behavior.

---

90. Graceful Degradation

If the language explicitly supports graceful degradation, it MUST be semantic and explicit.

For example:

prefer accelerator

may allow:

accelerator unavailable
→ use portable fallback

But:

require accelerator

must not silently fall back.

---

91. Compile-Time Resource Budgets

Compiler implementations SHOULD support configurable budgets for:

source bytes
tokens
AST nodes
semantic nodes
macro expansion
generated IR
optimization work
memory
time
diagnostics

These budgets are not language limits.

They are execution safeguards.

A budget failure MUST be represented distinctly from invalid source.

---

92. Denial-of-Service Protection

The compiler MAY reject pathological input when an implementation budget is exhausted.

Protection mechanisms MUST NOT redefine language semantics.

Examples include:

excessive macro expansion
pathological nesting
excessive generated code
resource-exhausting compile-time evaluation

The implementation MUST provide a deterministic failure mode.

---

93. Grammar Complexity

Grammar rules MUST be designed to avoid avoidable ambiguity and pathological parse complexity.

Scalable grammar design SHOULD:

- factor common prefixes;
- use explicit precedence;
- avoid unnecessary ambiguous alternatives;
- avoid exponential backtracking where applicable;
- keep lexical rules deterministic;
- separate syntax from semantic resolution.

Scalability is not achieved merely by removing numeric constants.

The grammar algorithms themselves must scale.

---

94. Parser Progress Invariant

Every parser recovery path MUST make progress.

No malformed input may cause:

infinite loop
infinite recovery
unbounded repeated diagnostics

This is a production invariant.

---

95. Error Recovery Scalability

Recovery SHOULD skip to meaningful synchronization points such as:

statement boundary
declaration boundary
block boundary
delimiter boundary
EOF

Recovery MUST avoid recursively reparsing arbitrarily large regions.

---

96. AST Scalability

AST nodes SHOULD be compact and structured.

AST design MUST avoid:

machine-specific fields
backend-only state
runtime handles
device pointers
calibration objects
scheduler state

AST nodes SHOULD retain:

semantic source information
source spans
syntax structure
attributes
domain information

without duplicating downstream IR.

---

97. Source Span Scalability

Source spans MUST be represented efficiently.

They SHOULD identify source ranges without copying source text into every AST node.

Diagnostics SHOULD reference source locations through:

file identity
start position
end position

or an equivalent compact representation.

---

98. Interning and Symbol Scalability

The implementation MAY intern:

identifiers
module names
type names
operation names
attribute names
dialect names

where this improves memory usage.

Interning MUST NOT alter semantic identity.

Intern tables MUST have deterministic behavior where observable.

---

99. Symbol Table Scalability

Symbol lookup SHOULD be approximately:

O(1)

average-case or an appropriate indexed complexity.

Nested scopes SHOULD be represented structurally.

The implementation MUST NOT repeatedly scan the entire program for every identifier.

---

100. Generic Instantiation Scalability

Generic instantiation MUST avoid uncontrolled duplication.

Implementations SHOULD support:

memoization
canonical generic identities
sharing
lazy instantiation
incremental instantiation

where appropriate.

A generic program's semantic size MUST remain distinct from the number of target-specific instantiations generated.

---

101. Monomorphization

If a backend uses monomorphization, that is a compiler implementation strategy.

It MUST NOT change the language's generic semantics.

A compiler MAY select:

monomorphization
specialization
erasure
dictionary passing
dynamic dispatch

depending on target requirements.

---

102. Resource-Parametric Generics

Where appropriate, Zamani MAY express algorithms parameterized by resources:

N
M
Q
workers
precision
vector_width

These values remain semantic parameters.

The compiler may specialize them for a target.

---

103. Portability Classes

Every target-related concept SHOULD belong to one of these classes:

P0 — Fully portable semantic

No target-specific information.

P1 — Capability-constrained portable

Requires a capability but does not bind a target.

P2 — Resource-constrained portable

Requires a resource quantity but not a particular provider.

P3 — Deployment-specific

Depends on deployment configuration.

P4 — Hardware-specific

Explicitly binds to a physical target.

P5 — Implementation-private

Not part of portable language semantics.

This classification SHOULD be used throughout grammar documentation and semantic analysis.

---

104. Example Classification

requires quantum

is:

P1

while:

requires qubits >= N

is:

P2

and:

use physical qubit 17

is:

P4

A compiler-internal scheduler handle is:

P5

The grammar MUST prevent accidental promotion of P4/P5 concepts into P0 semantics.

---

105. Portability Rule

A portable construct MUST NOT depend implicitly on a lower portability class.

For example:

P0 → P4

must not occur simply because the compiler implementation chooses a physical representation.

A deliberate target-binding construct MAY request such a transition.

---

106. Future Hardware Rule

A valid Zamani program MUST NOT require modification merely because a new hardware architecture appears.

If the new hardware can implement the program's semantic requirements, the compiler SHOULD be able to lower the same semantic representation to it.

This is a core POCO-REAF property.

---

107. Unknown Future Capabilities

The grammar MUST permit extensibility for capabilities not yet known.

Unknown target capabilities MUST NOT require modifying the core grammar when they can be represented through extensible capability namespaces.

For example:

capability vendor.domain.feature

may be handled through an extension mechanism.

---

108. Vendor Independence

Vendor names MUST NOT become universal language semantics.

Vendor-specific syntax belongs under:

dialects
interoperability
target bindings
hardware descriptions

Core Zamani syntax remains vendor-neutral.

---

109. Interoperability Scalability

Foreign interfaces MUST NOT impose a universal limit on:

functions
parameters
types
symbols
libraries
ABIs
foreign modules

Interoperability layers MUST preserve explicit ABI constraints without turning them into language-wide limits.

---

110. OpenQASM Integration

OpenQASM support MUST be an interoperability/frontend concern.

OpenQASM syntax MUST lower into Zamani semantic structures and ultimately canonical quantum IR.

It MUST NOT create a second permanent quantum semantic model.

Target limitations in OpenQASM backends MUST NOT become Zamani grammar limits.

---

111. Verilog/HDL Integration

Verilog and other HDL interoperability belongs to the interoperability/HDL boundary.

Imported hardware semantics MUST be mapped into Zamani's hardware semantic model where possible.

Foreign-language limitations MUST NOT become core Zamani limits.

---

112. Documentation Scalability

Every scalability-sensitive grammar construct MUST document:

semantic meaning
maximum semantic cardinality
whether a maximum exists
whether a maximum is target-dependent
whether a maximum is implementation-only

If there is no semantic maximum, documentation MUST say so explicitly.

---

113. Test Scalability

Every scalable grammar component MUST have:

Small case

The smallest valid instance.

Typical case

A normal representative instance.

Large case

A substantially larger instance.

Parametric case

A size controlled by a parameter.

Boundary case

The largest practical test supported by the test environment.

Resource failure case

A valid program that cannot execute on an intentionally constrained target.

Determinism case

Repeated compilation must produce equivalent results.

---

114. No Artificial Test Limits

Tests MUST NOT encode accidental language limits.

Bad:

test_max_qubits_is_32()

Good:

test_qubit_count_is_not_grammar_limited()

Tests MAY use finite numbers because tests execute on finite machines.

They MUST NOT claim that the test number is the language maximum.

---

115. Scaling Test Strategy

Scalability tests SHOULD use parameterized generation.

For example:

N = 1
N = small
N = medium
N = large

and, where practical:

N beyond previously problematic implementation thresholds

The goal is to detect accidental limits rather than define a new maximum.

---

116. Quantum Scaling Tests

Tests MUST include generated quantum programs with varying:

qubit count
operation count
circuit depth
control count
target count
register count
measurement count
parameter count

The grammar MUST behave consistently across these scales.

---

117. HDL Scaling Tests

Tests SHOULD vary:

module count
port count
signal count
register count
pipeline depth
state count
parameter count

No arbitrary fixed maximum should emerge from grammar rules.

---

118. Distributed Scaling Tests

Tests SHOULD vary:

logical node count
service count
communication edge count
replication count
message definitions

while maintaining the distinction between logical and physical topology.

---

119. Cross-Domain Scaling Tests

The grammar MUST test large mixed programs such as:

classical + quantum
classical + HDL
quantum + HDL
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

The presence of one domain MUST NOT introduce artificial limits into another.

---

120. Deterministic Ordering

Where collections are semantically unordered, their internal representation MUST NOT accidentally determine program behavior.

Where collections are semantically ordered, the ordering MUST be explicit.

This applies to:

imports
attributes
capabilities
effects
resource requirements
dialect declarations
macro definitions
diagnostics
generated symbols

---

121. Hashing and Identity

Hashes MUST NOT be used as semantic identity merely because they are convenient.

If hashes are used for:

caching
incremental compilation
provenance
deduplication
artifact addressing

the underlying semantic identity MUST remain well-defined.

Hash collisions MUST NOT change program meaning.

---

122. Incremental Compilation

Incremental compilation MAY reuse previously compiled semantic units.

Cache keys MUST include all semantic inputs required for correctness, including where applicable:

language version
dialect versions
source identity
semantic configuration
dependency identity
compiler semantic version

Target-specific information MUST NOT contaminate portable semantic cache identity unless target specialization is intentionally being cached.

---

123. Reproducibility

A reproducible compilation MUST NOT depend on:

machine size
map iteration order
wall clock
uncontrolled randomness
network state
filesystem ordering
thread scheduling

unless explicitly declared as an input.

---

124. Scalability and Versioning

Language evolution MUST NOT introduce hidden scalability regressions.

A new language version MUST document:

new limits
removed limits
changed limits
new resource semantics
new target-binding semantics

Any newly introduced bound MUST have an explicit justification.

---

125. Compatibility Rule

Backward-compatible source programs MUST NOT become invalid merely because they are larger than an implementation's previous practical test size.

For example, adding support for:

N > previous test maximum

is not a breaking semantic change.

Removing an artificial parser limit is normally an implementation improvement.

---

126. Generated Code

Generated Zamani code MUST obey exactly the same scalability rules as handwritten source.

The compiler MUST NOT create generated source that depends on artificial machine limits unless it is deliberately target-specialized.

---

127. Serialization

Serialized AST/IR/resource metadata MUST NOT impose smaller semantic limits than the language unless the serialization format explicitly defines such limits.

If an external format is bounded, the compiler MUST report an interoperability limitation rather than claiming that the Zamani program is invalid.

---

128. Canonical Serialization

Canonical serialization MUST be deterministic.

Equivalent semantic structures SHOULD serialize consistently.

Ordering MUST be explicit.

Machine-specific data MUST be excluded from portable semantic serialization unless intentionally represented as target binding.

---

129. Execution Context

Execution context may provide:

resources
capabilities
limits
placement
target
calibration
noise
runtime state

These are inputs to compilation/execution.

They MUST NOT alter the source grammar.

---

130. Resource Negotiation

When a program expresses a requirement, the compiler/runtime MAY negotiate among available resources.

For example:

require quantum
prefer low_latency

may allow selection among multiple capable backends.

The source remains unchanged.

---

131. Scaling and Scheduling

Scheduling MAY adapt dynamically to:

available resources
queue state
latency
topology
calibration
noise
power
energy

but the scheduler MUST NOT redefine the program.

---

132. Scaling and Resilience

Resilience MAY change execution strategy:

retry
resume
reroute
reschedule
recompile
switch backend
change mitigation

but MUST preserve the semantic contract.

If semantic preservation cannot be established, execution MUST NOT silently continue as though the result were equivalent.

---

133. Resource Availability Is Not Language Validity

A program can be:

syntactically valid
semantically valid
resource-infeasible for target A
resource-feasible for target B

This is a valid and expected outcome.

Example:

program requires 10,000 logical qubits
target A provides insufficient capacity
target B provides sufficient capacity

The program remains valid.

---

134. "Atom to Everywhere" Rule

The smallest valid computation MUST use the same semantic architecture as the largest.

There must not be a separate miniature semantic model that later becomes incompatible with large programs.

The path is always:

syntax
→ AST
→ semantics
→ canonical representation
→ target realization

regardless of program scale.

---

135. No Small-Target Special Semantics

Embedded or tiny-target compilation MAY optimize aggressively.

It MUST NOT redefine the language merely because the target is small.

A tiny target may reject a program because resources are insufficient.

It MUST NOT cause the language itself to acquire a tiny-target maximum.

---

136. No Large-Target Special Semantics

Likewise, large machines MAY provide more resources.

They MUST NOT change the meaning of source constructs.

More hardware means more possible implementations, not a different language.

---

137. Compiler Feature Detection

Compiler feature detection MUST be explicit.

The compiler MAY determine:

supports_quantum
supports_gpu
supports_dynamic_circuit
supports_hdl_synthesis
supports_distributed

but this information MUST enter through the compilation environment/capability model.

The grammar does not inspect it.

---

138. Semantic Feature Detection

Source-level conditional compilation MAY exist.

If supported, it MUST distinguish:

language feature

from:

target capability

For example:

if capability quantum { ... }

is conceptually different from:

if language_version >= ...

and both differ from physical target identity.

---

139. Avoiding Semantic Forks

Different target backends MUST NOT introduce incompatible interpretations of the same source construct.

If a backend cannot implement a construct, it must report:

unsupported capability

or:

unsatisfied constraint

rather than redefining the construct.

---

140. Grammar Ownership of Scalability

The grammar's responsibility is to provide syntax capable of representing scalable semantics.

It does NOT need to prove that a target can execute every program.

Therefore grammar validation MUST ask:

«Can this source construct be represented?»

rather than:

«Can the current machine execute this source construct?»

---

141. Semantic Analysis Ownership

Semantic analysis MUST determine:

type correctness
name correctness
effect correctness
ownership correctness
resource-expression correctness
capability requirements
domain compatibility
quantum legality
hardware-semantic legality

It MUST NOT inspect current hardware merely to decide whether syntax is valid.

---

142. Target Analysis Ownership

Target analysis determines:

resource sufficiency
capability support
native operations
topology
memory
timing
calibration
deployment

This occurs after semantic meaning has been established.

---

143. Backend Independence

Every core grammar construct MUST have a target-neutral interpretation.

Backend-specific behavior MUST be layered afterward.

This is mandatory for POCO-REAF.

---

144. Integration With "grammar/validation/grammar-validation.md"

"grammar-validation.md" owns syntax validation.

This file owns scalability validation.

The boundary is:

grammar-validation.md
    ↓
Is the syntax structurally valid?

scalability-rules.md
    ↓
Does the syntax/implementation impose an accidental scalable-resource limit?

Neither document owns semantic type checking.

---

145. Integration With "semantic-boundaries.md"

"semantic-boundaries.md" defines which subsystem owns meaning.

This document defines how those meanings scale.

The two documents MUST remain consistent.

If a scalability rule conflicts with a semantic-boundary rule, the architecture MUST be revised rather than silently accepting contradictory ownership.

---

146. Integration With "hardcoding-audit.md"

"hardcoding-audit.md" identifies hard-coded values.

This document defines whether those values are acceptable.

Every discovered constant MUST be classified as:

1. semantic constant;
2. syntax constant;
3. type-system constant;
4. target constraint;
5. resource budget;
6. implementation safeguard;
7. test fixture;
8. accidental hard-coding.

Only category 8 is automatically prohibited.

---

147. Integration With "ambiguity-rules.md"

Ambiguity rules MUST prevent scalable syntax from becoming ambiguous as domains expand.

New domain constructs MUST NOT introduce parsing ambiguity merely because the language supports more domains.

Semantic disambiguation MUST occur after parsing when lexical identity is insufficient.

---

148. Integration With "compatibility-rules.md"

Compatibility rules MUST ensure that scalability improvements do not silently break existing programs.

Removing an accidental maximum SHOULD be backward compatible.

Introducing a genuine semantic restriction requires explicit language-version treatment.

---

149. Integration With "naming-rules.md"

Naming rules MUST remain scalable.

They MUST NOT reserve an ever-growing list of machine/vendor/device names in the core language.

Vendor/domain names SHOULD use namespaces.

---

150. Integration With "Zamani.g4"

"grammar/Zamani.g4" is the executable syntax boundary.

It MUST:

- avoid fixed resource cardinalities;
- use scalable repetitions;
- avoid machine-derived predicates;
- avoid target discovery;
- avoid parser-side resource checks;
- avoid finite catalogs where extensibility is appropriate;
- preserve semantic distinctions required downstream.

---

151. Integration With Lexer

The lexer MUST NOT encode scalable domain catalogs unnecessarily.

For example, quantum operation names SHOULD NOT all become reserved keywords merely to recognize them.

The lexer MUST preserve identifiers so semantic resolution can remain extensible.

---

152. Integration With AST

AST structures MUST be able to represent arbitrary collection cardinality.

They MUST NOT contain fields such as:

qubit0
qubit1
qubit2

Instead they SHOULD use collections:

Vec<T>

or appropriate scalable representations.

Host collection limits remain implementation constraints, not language semantics.

---

153. Integration With Types

Types MUST support parameterization where scale varies.

Examples:

Array<T, N>
Tensor<T, Shape>
QubitRegister<N>

where semantically appropriate.

The type system MUST NOT turn target capacity into a type-level universal maximum.

---

154. Integration With Resources

The resource model MUST be the primary location for scalable resource requirements.

Grammar syntax expresses the requirement.

Resource analysis determines its meaning.

Target analysis determines availability.

---

155. Integration With Classical IR

Classical IR MUST remain capable of representing scalable:

control flow
data
memory
parallelism
types
calls
effects
resources

The grammar MUST NOT encode backend-specific classical representation.

---

156. Integration With "quantum::ir"

Quantum source constructs lower into canonical "quantum::ir".

No grammar component may introduce a competing semantic quantum representation.

The quantum IR remains the canonical boundary consumed by:

optimization
routing
scheduling
ZQN
QEC integration
hardware
runtime

---

157. Integration With QEC

QEC receives semantic quantum information after canonical lowering.

The grammar expresses intent.

QEC determines implementation.

No grammar-level QEC algorithm may depend on a fixed number of physical qubits.

---

158. Integration With ZQN

ZQN receives fault/noise information after semantic lowering.

The grammar does not perform hardware noise discovery.

---

159. Integration With Scheduling

Scheduling receives:

operations
dependencies
resource requirements
timing constraints
capabilities

and determines an executable schedule.

No scheduler constant becomes a grammar constant.

---

160. Integration With Hardware

Hardware provides:

capabilities
resources
topology
calibration
constraints

The grammar remains unchanged.

---

161. Integration With Runtime

Runtime receives executable artifacts and execution context.

Runtime resource availability MUST NOT change source semantics.

---

162. Integration With Resilience

Resilience may adapt execution based on changing resource availability.

Its decisions MUST preserve semantic identity and provenance.

---

163. Integration With Tooling

IDE/LSP/tooling MUST NOT impose smaller semantic limits than the compiler.

Editor diagnostics MAY be approximate for performance, but the canonical compiler remains authoritative.

Tooling SHOULD support large files incrementally.

---

164. Integration With Documentation

Documentation MUST distinguish:

semantic maximum
implementation maximum
target maximum
recommended practical maximum
test maximum

These terms MUST NOT be used interchangeably.

---

165. Integration With Examples

Examples MUST NOT accidentally establish false language limits.

An example using:

4 qubits

does not imply a four-qubit language.

An example using:

8 workers

does not imply an eight-worker maximum.

Examples SHOULD include parametrized versions where scale is relevant.

---

166. Integration With Tests

Every scalable construct MUST have:

minimum test
parametric test
large test
failure-on-resource-constrained-target test
determinism test

where applicable.

---

167. Hard-Coding Audit Procedure

The following search categories MUST be audited:

MAX_*
MIN_*
LIMIT_*
CAP_*
COUNT_*
QUANTUM_*
QUBIT_*
CORE_*
THREAD_*
DEVICE_*
NODE_*
GPU_*
FPGA_*
MEMORY_*
REGISTER_*
WIDTH_*
DEPTH_*
RANK_*

These names are not automatically forbidden.

Every occurrence must be classified.

---

168. Hard-Coding Examples

Forbidden

const MAX_QUBITS: usize = 32;

when it defines language validity.

Allowed

const DEFAULT_DIAGNOSTIC_LIMIT: usize = ...;

if it is an explicitly configurable tooling budget and does not define language semantics.

Allowed

let index: usize = ...

when "index" represents a host-memory collection index.

Forbidden

if qubit_count > 64 {
    reject_program();
}

inside semantic validation merely because the compiler implementation was designed for 64 qubits.

---

169. Fixed Constants in Grammar

Grammar constants are allowed only when they describe syntax itself.

Examples:

number of characters in an escape sequence
fixed punctuation spelling
operator spelling

They MUST NOT encode:

machine size
resource capacity
device topology
hardware count

---

170. Default Values

Defaults MUST be classified.

A default such as:

default optimization level

is implementation policy.

A default such as:

default qubit count = 8

would be dangerous unless it is explicitly part of the source construct's semantics.

Defaults MUST NOT accidentally become limits.

---

171. "Reasonable Maximum" Is Not a Valid Language Rule

The phrase:

«"No one will need more than N."»

MUST NOT justify a language maximum.

The only acceptable question is:

«Is N part of the semantic definition of the construct?»

If not, it MUST NOT be encoded as a language ceiling.

---

172. Performance Must Not Become Semantics

Compiler performance concerns MUST NOT be disguised as semantic restrictions.

Instead of:

reject programs larger than N

prefer:

configurable compiler resource budget

with a distinct diagnostic.

---

173. Scalability and Safety

Scalability MUST NOT weaken safety.

The compiler MUST NOT use:

unsafe pointer tricks
unchecked memory access
unchecked integer conversion

to handle larger programs.

Safe Rust data structures and algorithms MUST be used.

---

174. Scalability and Security

Large-input handling MUST be hardened against:

memory exhaustion
CPU exhaustion
macro expansion bombs
pathological parsing
diagnostic flooding
dependency explosions

Security controls MUST remain implementation controls rather than semantic language restrictions.

---

175. Resource Budget API

Where compiler budgets exist, they SHOULD be represented explicitly.

Conceptually:

CompilationBudget {
    time
    memory
    expansion_work
    generated_nodes
    diagnostics
}

The exact implementation belongs outside the grammar specification.

The grammar MUST only require that budget failures be distinguishable from semantic invalidity.

---

176. Scaling and Caching

Caches MAY reduce repeated work.

Cache invalidation MUST remain semantically correct.

A cached target-specific result MUST NOT be reused for an incompatible target.

A portable semantic artifact MAY be reused across targets when its contract permits.

---

177. Scaling and Provenance

Large compilation pipelines MUST preserve provenance.

Every lowered operation SHOULD remain traceable to relevant source semantics where diagnostics and verification require it.

Scaling MUST NOT cause provenance to be discarded merely to save memory unless the requested compilation mode explicitly permits it.

---

178. Scaling and Verification

Verification MUST be scalable.

Where full verification is computationally expensive, the system MAY use:

incremental verification
localized verification
proof summaries
hash-based identity
resource-aware verification

but MUST NOT silently skip mandatory semantic invariants.

---

179. Cross-Domain Scalability

Cross-domain constructs MUST remain compositional.

For example:

classical
    +
quantum

must not require a second language.

Likewise:

quantum
    +
HDL
    +
distributed

must compose through explicit semantic boundaries.

---

180. Domain Independence

Adding a new domain MUST NOT require changing unrelated scalability rules.

For example:

new accelerator domain

should not require changing:

core expression cardinality
qubit limits
module limits
parser collection limits

unless there is a demonstrated language-wide semantic reason.

---

181. Semantic Extensibility

New semantic concepts SHOULD be introduced through:

new type
new effect
new capability
new resource kind
new operation
new dialect
new IR extension

rather than modifying unrelated grammar rules.

---

182. Avoiding Grammar Fragmentation

Subdirectories MUST improve ownership and maintainability.

They MUST NOT create duplicated definitions.

For example, there MUST be one authoritative semantic definition for:

identifier
type expression
resource requirement
capability
quantum operation

even if several grammar files reference it.

---

183. Imported Grammar Fragment Rules

If ANTLR grammar fragments are split across files:

- each fragment MUST have one responsibility;
- imports MUST be acyclic;
- shared rules MUST have one owner;
- duplicate token/rule definitions MUST be prohibited;
- generated parser output MUST be deterministic.

---

184. Dependency Direction

The dependency direction MUST remain:

specification
    ↓
grammar
    ↓
lexer/parser
    ↓
AST
    ↓
semantic analysis
    ↓
canonical IR
    ↓
optimization
    ↓
routing/scheduling/resilience/ZQN
    ↓
target lowering
    ↓
runtime

Never:

grammar → runtime
grammar → hardware discovery
grammar → scheduler state
grammar → calibration

---

185. No Reverse Semantic Dependency

Backend limitations MUST NOT modify grammar meaning.

For example:

hardware supports only 127 qubits

must not cause the parser to change its accepted language.

---

186. Resource Failure Reporting

A resource failure SHOULD identify:

requested resource
available resource
required capability
target
relevant source span
possible alternatives

without rewriting the program.

---

187. Scalability and Compilation Modes

Different compilation modes MAY exist:

syntax-only
semantic-only
portable-IR
targeted
optimized
debug
verification
simulation
hardware

All modes MUST share the same language semantics.

A mode MUST NOT silently introduce a different grammar.

---

188. Fast Parsing Mode

A fast parser MAY skip optional analysis.

It MUST NOT skip syntax correctness.

It MAY defer:

name resolution
type checking
capability analysis
resource analysis

to later stages.

---

189. Full Compilation Mode

Full compilation performs complete semantic validation.

It MUST preserve the same meaning as syntax-only parsing followed by semantic analysis.

---

190. Incremental IDE Mode

IDE parsing MAY operate on incomplete source.

Incomplete source is not necessarily invalid completed source.

The IDE MUST distinguish:

incomplete
invalid
valid

and MUST NOT infer scalability limits from partial source.

---

191. Error Tolerance

Error recovery MUST NOT produce a fake semantic interpretation that later becomes authoritative.

Recovered AST nodes MUST carry appropriate error state.

---

192. Generated AST Nodes

Generated nodes MUST preserve semantic provenance where required.

They MUST NOT be indistinguishable from user-authored source when diagnostics need to explain transformations.

---

193. Serialization and Version Evolution

Large serialized representations MUST be versioned.

A newer representation MUST NOT silently reinterpret old resource semantics.

Migration MUST be explicit.

---

194. Backward Compatibility

A program valid under a previous compatible language version SHOULD remain semantically valid unless a documented breaking change exists.

Scalability improvements MUST normally be monotonic:

old valid program
+
new compiler
=
still valid

Removing an accidental limit is therefore a compatibility-preserving improvement.

---

195. Forward Compatibility

Unknown future constructs SHOULD be rejected explicitly unless an extension mechanism can safely preserve them.

The parser MUST NOT silently reinterpret unknown constructs as existing constructs.

---

196. Reserved Extension Space

Reserved keywords/namespaces MAY exist for future scalability.

Reserved space MUST be documented.

Reserved identifiers MUST NOT be confused with implemented features.

---

197. Production Readiness Requirement

A scalability implementation is production-ready only when:

- no accidental machine limits remain;
- semantic limits are documented;
- implementation budgets are separated;
- target limits are separated;
- resource requirements are explicit;
- quantum scale is unbounded by grammar;
- classical scale is unbounded by grammar;
- HDL scale is unbounded by grammar;
- distributed scale is unbounded by grammar;
- tensor rank is not artificially bounded;
- module count is not artificially bounded;
- operation count is not artificially bounded;
- parser recovery is bounded by implementation safeguards rather than semantic ceilings;
- no "unsafe" Rust exists;
- deterministic behavior is verified;
- large-input tests exist;
- cross-domain tests exist;
- canonical IR boundaries are preserved.

---

198. File Completion Contract

This file is complete only when all of the following are true.

Purpose

The file defines scalability rules for the entire grammar boundary.

Owns

This file owns:

- scalability terminology;
- scalability invariants;
- distinction between semantic and implementation limits;
- machine-independence rules;
- resource-parametric grammar requirements;
- scaling test requirements;
- integration requirements related to scalability.

Does Not Own

This file does not own:

- exact grammar productions;
- AST implementation;
- type-system implementation;
- canonical quantum IR;
- hardware discovery;
- scheduling algorithms;
- QEC algorithms;
- ZQN implementation;
- runtime implementation.

Inputs

This specification integrates with:

grammar/DESIGN.md
grammar/README.md
grammar/Zamani.g4
grammar/specification/*
grammar/validation/grammar-validation.md
grammar/validation/semantic-boundaries.md
grammar/validation/ambiguity-rules.md
grammar/validation/compatibility-rules.md
grammar/validation/naming-rules.md
grammar/validation/hardcoding-audit.md

and repository compiler/IR/backend contracts.

Outputs

This file provides normative constraints for:

grammar design
parser design
AST design
semantic analysis
resource analysis
scalability testing
compiler implementation
target integration

Upstream Contracts

The specification layer defines:

language semantics
POCO-REAF
hardware independence
resource model

This file refines those rules for scalability.

Downstream Consumers

Consumers include:

lexer
parser
AST
semantic analyzer
type checker
resource analyzer
compiler
quantum frontend
quantum::ir lowering
hardware lowering
scheduler
runtime
tests
tooling

IR Contract

No new IR is defined here.

The document requires that grammar semantics lower into existing canonical IR ownership.

For quantum semantics, the canonical boundary remains:

quantum::ir

Compiler Contract

The compiler MAY impose resource budgets.

Those budgets MUST remain distinguishable from language validity.

Runtime Contract

Runtime resource availability MUST NOT modify source semantics.

Tooling Contract

Tools MUST NOT invent smaller language limits than the compiler.

Cross-Domain Contract

All domains MUST follow the same:

semantic requirement
→ capability/resource analysis
→ target realization

model.

---

199. Required Validation Matrix

Before this document is considered implemented across the repository, validate:

Area| Required
Grammar cardinalities| No artificial maxima
Quantum qubits| No grammar maximum
Classical resources| No fixed machine maximum
HDL hierarchy| No arbitrary maximum
Tensor rank| No artificial rank limit
Modules| Unbounded by language
Functions| Unbounded by language
Operations| Unbounded by language
Distributed nodes| Logical count is parametric
Threads/tasks| Runtime controlled
Memory| Resource controlled
Devices| Target controlled
Topology| Target controlled
Calibration| Hardware controlled
Scheduling| Scheduler controlled
QEC| QEC subsystem controlled
Noise| ZQN controlled
Resilience| Resilience subsystem controlled
Diagnostics| Configurable implementation budget
Macros| Configurable expansion budget
Parser depth| No semantic ceiling
Compiler memory| Implementation budget
Compiler time| Implementation budget
Rust safety| No "unsafe"
Determinism| Required
Compatibility| Required
Provenance| Required
Cross-domain composition| Required

---

200. Required Negative Tests

The following classes MUST have negative tests where applicable:

hidden machine maximum
fixed qubit ceiling
fixed core ceiling
fixed tensor rank
fixed node ceiling
fixed device count
fixed topology
silent integer truncation
resource failure reported as syntax failure
target-specific syntax accepted as portable syntax
backend-specific limitation encoded as grammar rule
non-deterministic ordering
parser infinite recovery
macro expansion exhaustion without diagnostic
unsafe implementation workaround

---

201. Required Positive Tests

Positive tests MUST include:

minimal program
large classical program
large quantum program
parameterized quantum program
large HDL hierarchy
large tensor program
large distributed program
large module graph
cross-domain program
target-independent resource requirement
capability requirement
constraint
preference
hint
dialect extension
future capability extension

---

202. Required Boundary Tests

Boundary tests MUST verify that:

semantic limit

is not confused with:

resource limit

and that:

resource limit

is not confused with:

parser limit

and that:

parser implementation limit

is not confused with:

language validity

---

203. Required Scalability Invariants

The following invariants are mandatory:

S1 — No artificial machine ceiling

No machine capacity becomes a language maximum.

S2 — Resource parametricity

Program scale is determined by semantic/resource expressions.

S3 — Target independence

Changing hardware does not require changing portable source semantics.

S4 — Canonical representation

No duplicate semantic IR is introduced.

S5 — Determinism

Scaling does not introduce accidental nondeterminism.

S6 — Safe implementation

No Rust "unsafe".

S7 — Explicit failure

Resource failure is not semantic rewriting.

S8 — Extensibility

Future domains do not require arbitrary core grammar redesign.

S9 — Composability

Domains can be combined without conflicting scale assumptions.

S10 — Monotonicity

Removing accidental implementation limits does not invalidate previously valid programs.

---

204. Final Scalability Principle

Zamani MUST follow this rule:

«The program defines what computation means. The compilation environment defines what resources are available. The compiler defines how the computation is realized. The runtime defines where and when it executes.»

Therefore:

Program
    ↓
Meaning
    ↓
Requirements
    ↓
Capabilities
    ↓
Resources
    ↓
Implementation
    ↓
Execution

must never become:

Program
    ↓
Current machine
    ↓
Current machine limits
    ↓
Language restrictions

---

205. Final POCO-REAF Contract

The ultimate scalability invariant is:

ONE PROGRAM
     ↓
ONE SEMANTIC MEANING
     ↓
ONE PORTABLE COMPILATION MODEL
     ↓
MANY TARGETS
     ↓
MANY ARCHITECTURES
     ↓
MANY RESOURCE SCALES
     ↓
MANY EXECUTION ENVIRONMENTS
     ↓
FUTURE MACHINES

A Zamani program SHOULD therefore be written once and remain semantically stable while execution scales:

atom
→ device
→ processor
→ accelerator
→ multicore
→ cluster
→ supercomputer
→ distributed system
→ heterogeneous system
→ quantum system
→ future computational architecture

subject only to the program's actual semantic requirements and the resources/capabilities available to the implementation.

"Infinity" means no accidental language ceiling.

It does not mean ignoring finite physical resources.

That distinction is fundamental to POCO-REAF.

---

206. Final Rule

The single highest-priority scalability rule is:

«Never turn an implementation limitation into a language limitation.»

If a limit exists, the implementation MUST first determine which category it belongs to:

LANGUAGE SEMANTIC
TYPE SEMANTIC
PROGRAM REQUIREMENT
PROGRAM CONSTRAINT
TARGET CAPABILITY
TARGET RESOURCE
DEPLOYMENT LIMIT
COMPILER BUDGET
SECURITY BUDGET
TEST LIMIT
IMPLEMENTATION LIMIT
ACCIDENTAL HARD-CODING

Only the last category is automatically a defect.

The resulting architecture must preserve:

Zamani
    =
portable computation
+
semantic intent
+
resource parametricity
+
capability negotiation
+
target-independent meaning
+
canonical IR
+
safe scalable implementation

with:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

as the governing architectural objective.