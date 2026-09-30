Zamani Grammar Scalability Validation Specification

Path: "grammar/validation/scalability.md"
Status: Normative production validation specification
Language: Zamani
Scope: Grammar, lexer, parser, AST, semantic analysis, canonical IR, quantum IR, hardware intent, resource/capability analysis, compiler integration, runtime integration, and scalability conformance
Rust baseline: Rust 1.97 / Rust 1.97.1
Edition: Rust 2021
Safety: Safe Rust only; production implementation MUST NOT use Rust "unsafe"
Primary objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

This file defines the production validation contract for scalability throughout the Zamani grammar and every compiler layer that consumes it.

It complements:

- "grammar/validation/scalability-rules.md"
- "grammar/specification/scalability-model.md"
- "grammar/spec/portability.md"
- "grammar/DESIGN.md"
- "grammar/Zamani.g4"
- "grammar/grammar.md"
- "grammar/Zamani-Grammar.md"

It does not replace those files.

Its responsibility is narrower and more operational:

«Determine whether the grammar and its consuming implementation preserve scalability from the smallest meaningful computation to arbitrarily large computation subject only to program semantics, explicit representation rules, declared resource policies, implementation budgets, target capabilities, and resources actually available.»

The central validation question is therefore not:

«“Can the current machine execute this?”»

The central validation question is:

«“Has Zamani accidentally made the language incapable of expressing or semantically representing this computation merely because of an implementation-specific limit?”»

A production implementation MUST answer those questions separately.

---

2. Authority and Integration

The authority hierarchy is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ▼
grammar/spec/
        │
        ├── scalability
        ├── portability
        ├── type system
        ├── semantics
        ├── quantum
        ├── HDL
        └── resource/capability contracts
        │
        ▼
grammar/Zamani.g4
        │
        ▼
src/lexer.rs
        │
        ▼
src/parser.rs
        │
        ▼
src/frontend/ast/
        │
        ▼
semantic analysis
        │
        ▼
canonical semantic representation
        │
        ├───────────────┬────────────────┐
        ▼               ▼                ▼
   classical        quantum::ir     HDL/hardware
        │               │                │
        └───────────────┼────────────────┘
                        ▼
                   optimization
                        │
             ┌──────────┼──────────┐
             ▼          ▼          ▼
          routing   scheduling   resilience
             │          │          │
             └──────────┼──────────┘
                        ▼
                       ZQN
                        │
                        ▼
                       HAL
                        │
                        ▼
                 target realization

No lower layer may redefine the meaning of an upper layer.

Scalability validation MUST therefore test the complete chain rather than only "Zamani.g4".

---

3. Relationship to Existing Scalability Rules

"grammar/validation/scalability-rules.md" remains the broad normative scalability ruleset.

This file adds the validation contract.

The distinction is:

File| Responsibility
"validation/scalability-rules.md"| What scalability means and which rules apply
"validation/scalability.md"| How compliance is validated
"specification/scalability-model.md"| Language-level scalability model
"spec/portability.md"| Portability/target-independence contract
"resources/scalability.g4"| Source syntax for scalability intent
"resources/requirements.g4"| Resource requirement syntax
"hardware/*.g4"| Hardware capability/intent syntax
"quantum/resource-requirements.g4"| Quantum resource intent
"Zamani.g4"| Canonical grammar composition
"grammar.md"| Implementation conformance
"DESIGN.md"| Overall architecture

No file may create a second, contradictory definition of scalability.

---

4. Definition of Scalability

For Zamani:

«Scalability means that language expressiveness is not artificially bounded by the size or topology of the machine on which a program happens to be compiled or executed.»

The language must support the same semantic model across:

atom
    ↓
embedded
    ↓
single CPU
    ↓
multicore CPU
    ↓
many CPUs
    ↓
GPU
    ↓
many GPUs
    ↓
FPGA
    ↓
ASIC
    ↓
accelerator
    ↓
QPU
    ↓
quantum simulator
    ↓
CPU + GPU
    ↓
CPU + QPU
    ↓
FPGA + CPU
    ↓
HPC
    ↓
cluster
    ↓
distributed system
    ↓
cloud
    ↓
edge
    ↓
future execution systems

The same source semantics SHOULD remain valid whenever the target satisfies the program's semantic requirements or a legal lowering exists.

---

5. Meaning of “Infinity”

“Infinity” does not mean that a physical machine can provide infinite resources.

It means:

«Zamani MUST NOT impose an arbitrary language-level upper bound merely because current implementations or machines are finite.»

The following are distinct:

language semantics
implementation representation
compiler resource budget
host resources
target resources
target capabilities
runtime availability
deployment policy

A failure in one category MUST NOT silently become a language restriction in another.

For example:

available_qubits = 127

may be a target fact.

It MUST NOT imply:

Zamani supports at most 127 qubits

Likewise:

compiler memory budget = X

MUST NOT imply:

Zamani programs cannot describe more than X memory

---

6. Primary Invariant

The following invariant is mandatory:

Program semantics
    MUST NOT depend on accidental machine capacity.

Therefore:

same source
+
same language version
+
same semantic inputs
=
same program meaning

Changing:

- CPU count;
- GPU count;
- FPGA size;
- QPU size;
- memory availability;
- node count;
- network topology;
- accelerator count;
- physical qubit count;
- device provider;
- scheduler;
- runtime placement;

MUST NOT silently change the program's semantic meaning.

---

7. Four Kinds of Limits

Every discovered limit MUST be classified as one of:

1. semantic limit;
2. representation limit;
3. implementation/resource limit;
4. target/environment limit.

Validation MUST fail when a category-3 or category-4 limitation has accidentally been encoded as a category-1 language restriction.

---

7.1 Semantic Limits

A semantic limit is a genuine property of the language abstraction.

Examples:

- a type with explicitly defined finite range;
- a language-defined integer representation;
- an explicitly bounded external format;
- a protocol field with a specified width;
- a deliberately finite enumeration.

Such a limit MUST be documented as semantic.

---

7.2 Representation Limits

A representation limit arises from a concrete representation.

Examples:

u64
i128
f64

A representation limit MUST NOT be confused with a universal language-capacity limit.

If arbitrary precision is required by a future semantic type, it must be explicitly defined.

The lexer MUST NOT accidentally impose host integer limits merely because Rust's native integer types are convenient.

---

7.3 Implementation Limits

Examples:

maximum compiler memory
maximum diagnostic count
maximum macro expansion
maximum parser work
maximum optimization work
maximum generated artifact size
maximum compilation duration

These MAY exist.

They MUST:

- be implementation controls;
- be distinguishable from language validity;
- produce implementation/resource diagnostics;
- not redefine language semantics;
- not appear as universal grammar limits;
- be documented;
- be testable independently.

---

7.4 Target Limits

Examples:

available qubits
available memory
supported vector width
available GPU memory
FPGA resources
network capacity
device capability
clock constraints
topology constraints

These belong to target/environment analysis.

They MUST NOT be embedded into the language grammar.

---

8. Forbidden Universal Capacity Constants

The following concepts MUST NOT define universal language limits:

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

The prohibition applies to equivalent aliases as well.

Examples that MUST be detected by validation include:

MAX_QUANTUM_REGISTER
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS
MAX_GPU_COUNT
MAX_CPU_COUNT
MAX_CLUSTER_NODES
MAX_ACCELERATORS
MAX_TIMELINES
MAX_AGENTS
MAX_PROCESSES
MAX_CHANNELS
MAX_PIPELINE_STAGES

The exact spelling is not important.

The semantic role is important.

---

9. Hard-Coding Detection

The scalability validator MUST inspect:

grammar/
src/
tests/
build scripts
generation scripts
configuration used by grammar tooling

for constants or branches that establish artificial universal limits.

Detection SHOULD include:

- identifier analysis;
- numeric literal analysis;
- comparison analysis;
- array-size analysis;
- collection-size analysis;
- range analysis;
- parser production analysis;
- test fixture analysis;
- semantic validation analysis;
- IR validation analysis.

The validator MUST distinguish:

program constant

from:

compiler capacity constant

---

10. Valid Constants

The hard-coding prohibition does not prohibit ordinary program constants.

This is valid:

let n = 1024;
allocate n elements;

because "1024" is program data.

This is also potentially valid:

matrix<1024, 1024>

because the dimensions belong to that program's type/data semantics.

The following is prohibited as a universal language restriction:

MAX_MATRIX_DIMENSION = 1024

when that constant prevents valid Zamani programs from representing larger matrices.

---

11. Resource Requirement Validation

The validator MUST recognize the distinction between:

requirement
capability
constraint
preference
hint
target
placement
realization

These concepts MUST NOT collapse.

---

12. Requirements

A requirement states something necessary for a valid realization.

Conceptually:

requires qubits >= n
requires memory >= required_memory
requires capability("tensor.compute")
requires capability("gpu.compute")
requires capability("quantum.measurement")
requires topology(...)

Requirements MUST remain parameterized by program semantics.

The compiler MUST NOT convert:

requires qubits >= n

into:

n <= compiler_max_qubits

as a language rule.

---

13. Capabilities

Capabilities describe target/environment abilities.

Examples:

quantum.compute
quantum.measurement
quantum.dynamic_control
gpu.compute
fpga.synthesis
tensor.compute
distributed.execution
secure.execution
fault.tolerance

Capabilities MUST be discovered or supplied by the environment.

The grammar MUST describe capability requirements, not pretend to know which target provides them.

---

14. Constraints

Constraints describe properties that valid realizations must satisfy.

Examples:

latency <= budget
memory <= budget
energy <= budget
reliability >= required_level
noise <= tolerance

A constraint may cause compilation or deployment failure.

It MUST NOT cause parsing failure merely because the current target cannot satisfy it.

---

15. Preferences

Preferences are optional optimization guidance.

Examples:

prefer accelerator("quantum")
prefer low_latency
prefer energy_efficiency
prefer local_execution

A preference MUST NOT silently become a semantic requirement.

---

16. Hints

Hints provide optional optimization information.

A hint MUST NOT alter program meaning.

If a hint cannot be honored, the program SHOULD remain semantically valid unless the language specification explicitly defines the hint as mandatory.

---

17. Physical Realization

Physical realization belongs downstream.

The intended transformation is:

logical program
      ↓
semantic representation
      ↓
resource/capability analysis
      ↓
canonical IR
      ↓
routing
      ↓
scheduling
      ↓
target realization

The grammar MUST NOT bypass this architecture.

---

18. Collection Scalability

Repeated constructs MUST use compositional collection syntax.

Preferred:

items
    : item*
    ;

or equivalent.

Avoid finite enumeration such as:

items
    : item
    | item item
    | item item item
    ;

unless the semantic construct is deliberately bounded.

The rule applies to:

- declarations;
- parameters;
- arguments;
- imports;
- modules;
- functions;
- statements;
- expressions;
- match arms;
- quantum targets;
- quantum controls;
- HDL ports;
- signals;
- hardware resources;
- distributed nodes;
- services;
- capabilities;
- requirements;
- effects;
- tensor dimensions;
- data fields;
- timelines;
- agents.

---

19. Nesting Scalability

The language MAY permit arbitrary meaningful nesting.

Validation MUST inspect:

- recursive grammar rules;
- recursive parser functions;
- recursive AST traversals;
- recursive semantic analysis;
- recursive lowering;
- recursive optimization.

A valid language construct MUST NOT become invalid merely because a recursive Rust implementation exhausts the host call stack.

Where practical, implementations SHOULD use:

explicit stack
explicit worklist
iterative traversal
streaming
incremental processing
lazy processing

instead of unnecessary recursive Rust calls.

---

20. Operational Depth Budgets

A compiler MAY have a configurable operational depth budget.

For example:

maximum parser nesting processed in one invocation

Such a budget MUST be classified as:

implementation/resource budget

not:

language maximum nesting

The diagnostic MUST communicate the difference.

Bad:

Zamani syntax only supports 1024 nested expressions.

Good:

Compilation resource limit reached while processing nested expressions.
The source construct is valid according to the language specification.
Increase the compiler resource budget or use an incremental compilation strategy.

The exact diagnostic wording belongs to the diagnostics specification.

---

21. Source Size Scalability

The language MUST NOT establish arbitrary limits on:

- source file count;
- source file size;
- module count;
- package count;
- declaration count;
- statement count;
- expression count.

An implementation MAY have operational limits.

Such limits MUST be reported separately from syntax validity.

---

22. Quantum Scalability

Quantum constructs MUST be resource-parametric.

The language MUST NOT define universal limits on:

qubits
logical qubits
physical qubits
registers
register width
circuit width
circuit depth
operation count
control count
measurement count
classical feed-forward count

unless the bound is a property of an explicitly bounded external format.

---

23. Quantum Operation Scalability

The grammar MUST NOT become an ever-growing enumeration of every quantum operation.

Do not make the language fundamentally depend on:

H
X
Y
Z
CNOT
...

as a finite exhaustive list.

The preferred semantic model is:

operation specifier
+
parameters
+
targets
+
controls
+
modifiers
+
attributes

This permits:

apply H ...
apply custom_gate ...
apply vendor.operation ...
apply operation(parameter) ...

without modifying the fundamental grammar for every future operation.

---

24. Quantum Identity

The following distinction MUST remain explicit:

logical quantum resource

versus:

physical target resource

For example:

logical qubit q

is portable.

A binding such as:

q -> physical_qubit(17)

is target realization.

The grammar MUST NOT make physical identifiers the universal quantum model.

---

25. Canonical Quantum IR

Quantum source syntax MUST eventually lower to the repository's canonical:

quantum::ir

The grammar MUST NOT create a second competing quantum IR.

The allowed chain is:

quantum syntax
      ↓
frontend AST
      ↓
semantic quantum model
      ↓
quantum::ir

Then:

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
QEC/resilience/ZQN
      ↓
HAL
      ↓
target

Any alternative quantum representation MUST have a documented ownership reason and MUST NOT duplicate canonical semantic ownership.

---

26. Classical Scalability

Classical constructs MUST scale independently of:

CPU count
core count
thread count
register count
cache size
vector width

The language describes computation.

The compiler determines implementation.

The runtime determines available execution resources.

---

27. Tensor Scalability

Tensor syntax MUST NOT impose an arbitrary rank limit.

Invalid architectural pattern:

tensor rank <= 8

unless explicitly defined as part of a bounded type or interchange format.

Preferred:

Tensor<T, Shape>

where "Shape" is itself compositional.

The number of dimensions is semantic data.

The physical storage layout belongs downstream.

---

28. HDL Scalability

HDL syntax MUST support arbitrary meaningful composition.

There MUST NOT be grammar-wide fixed limits on:

modules
ports
signals
nets
registers
states
pipeline stages
instances
interfaces
clock domains
memory declarations
generated instances

Generate constructs MUST remain parameterized.

A hardware design may fail target realization because the target lacks resources.

That is not a grammar failure.

---

29. HDL Width Validation

The grammar MUST NOT use a fixed universal width such as:

wire [31:0]

as the language's hardware-width model.

Widths should be expressible as semantic parameters where appropriate.

For example:

width = W

means the design is parameterized by "W".

A concrete implementation may later instantiate:

W = 8
W = 32
W = 512
W = ...

subject to the selected target.

---

30. Hardware Scalability

Hardware intent MUST remain separate from target inventory.

The language may express:

requires capability("gpu.compute")
requires capability("fpga.synthesis")
requires capability("quantum.measurement")
requires topology(...)
requires memory >= required_memory

It MUST NOT silently turn these into:

GPU 0
FPGA 1
QPU 0
physical qubit 17

unless an explicitly target-bound construct requests physical realization.

---

31. Distributed Scalability

The grammar MUST NOT impose a fixed node count.

Logical participants MUST be separate from physical nodes.

Examples of valid semantic concepts:

replicas = n
participants = n
partition count = n

where "n" is program data or a runtime-selected quantity.

The runtime determines actual placement.

---

32. Concurrency Scalability

The language MUST NOT assume:

8 threads
16 workers
32 actors
64 channels

as universal limits.

Concurrency semantics must remain independent of execution resource count.

For example:

parallel for item in items

describes parallel semantics.

The runtime may realize that using:

1 worker
8 workers
64 workers
GPU execution
distributed execution

without changing program meaning.

---

33. Memory Scalability

The language MUST NOT hard-code:

maximum RAM
maximum VRAM
maximum heap
maximum stack
maximum allocation count
maximum address

as language-wide restrictions.

The following are different:

program requires memory >= M

and:

machine contains M bytes

The first belongs to program requirements.

The second belongs to target/environment state.

---

34. Resource-Intent Scalability

Resource syntax MUST remain compositional.

A resource declaration should be capable of representing:

quantity
kind
unit
requirement
constraint
capability
preference
hint
relationship
scope
lifetime
ownership
placement intent

without assuming a fixed number of resource entries.

---

35. Multi-Resource Programs

A program may simultaneously require:

CPU
GPU
QPU
FPGA
memory
network
storage
secure execution
distributed execution

The scalability validator MUST ensure that no domain assumes it is the only resource domain.

Resource requirements must compose.

---

36. Cross-Domain Scalability

The following combinations MUST be valid architectural targets:

classical + quantum
classical + HDL
classical + GPU
classical + FPGA
quantum + classical
quantum + networking
quantum + distributed
AI + GPU
AI + quantum
AI + FPGA
HDL + software
HDL + quantum
distributed + quantum
distributed + GPU
distributed + FPGA

The grammar MUST NOT create separate, incompatible universes for each domain.

---

37. Future-Domain Scalability

Adding a new computational domain MUST NOT require rewriting the fundamental scalability model.

A new domain should integrate through:

domain syntax
      ↓
universal AST
      ↓
semantic domain model
      ↓
canonical IR or existing canonical IR boundary
      ↓
resource/capability analysis
      ↓
lowering

The domain MUST document its integration contract.

---

38. Dialect Scalability

Dialects MUST NOT become hidden alternate languages.

Every dialect MUST identify:

name
version
owner
syntax additions
semantic additions
AST mapping
IR mapping
capabilities
resource model
compatibility
feature status

A dialect MUST NOT introduce a machine-size limit that silently becomes a core Zamani limit.

---

39. Macro Scalability

Macro expansion MUST remain scalable.

Validation MUST check:

- recursive expansion;
- expansion depth;
- token growth;
- AST growth;
- hygiene;
- source spans;
- diagnostics;
- cycle detection.

A macro expansion budget MAY exist.

It MUST be an implementation/resource budget.

A macro budget MUST NOT redefine valid source syntax.

---

40. Metaprogramming Scalability

Compile-time computation MUST be subject to explicit implementation budgets where required.

Validation MUST distinguish:

program cannot be represented

from:

compiler chose not to spend additional resources

The latter is an implementation policy.

---

41. Module Graph Scalability

The module system MUST NOT impose fixed limits on:

modules
imports
exports
dependency depth
package count
namespace count

The compiler MAY detect cycles and invalid dependency graphs.

Cycle detection is a semantic property, not an artificial capacity limit.

---

42. Generic Parameter Scalability

Generic constructs MUST support arbitrary meaningful parameter counts.

Do not encode:

generic parameters <= 8

unless the limit is an explicit semantic rule.

Parser and AST structures should use collections.

---

43. Expression Scalability

Expression parsing MUST remain scalable across:

operand count
operator count
call depth
index depth
member-access depth
generic nesting
type nesting
lambda nesting
match nesting

Parser implementation limits must be operational rather than semantic.

---

44. Source Span Scalability

Source spans MUST use a representation capable of addressing the supported source model without accidental truncation.

The implementation MUST NOT silently convert arbitrary source positions into a narrower machine representation merely because it is convenient.

Span representation MUST remain:

deterministic
stable
comparable
diagnostic-friendly
safe

Overflow in span construction MUST be diagnosed rather than silently wrapping.

---

45. Identifier Scalability

Identifiers MUST NOT be artificially limited by a machine-derived character or length constant unless explicitly specified by the lexical specification.

If an implementation has an operational identifier-size budget, that budget MUST be distinguishable from lexical validity.

---

46. Literal Scalability

Numeric literal processing MUST not depend on host integer overflow.

For example, the lexer must not reject a valid numeric token merely because its magnitude exceeds Rust's "u64".

The correct separation is:

lexical recognition
      ↓
literal representation
      ↓
type/semantic validation
      ↓
lowering

A literal may be lexically valid but semantically invalid for a selected type.

That is not a lexical scalability failure.

---

47. Determinism

Scalability MUST NOT compromise determinism.

For the same:

source
language version
dialect set
semantic configuration

the frontend must produce deterministic:

tokens
parse result
AST
diagnostics ordering
semantic result
canonical representation

Parallel implementation is allowed.

Nondeterministic semantics are not.

---

48. Parallel Compiler Implementation

Compiler implementation MAY use parallel processing.

However:

parallel compiler execution

MUST NOT introduce:

parallel-dependent language meaning

Any concurrency in implementation must preserve deterministic semantic results.

---

49. Streaming and Incremental Processing

Where source or intermediate data may be large, implementations SHOULD support:

- streaming;
- incremental parsing;
- incremental semantic analysis;
- incremental compilation;
- lazy expansion;
- chunked processing;
- explicit work queues.

These are implementation strategies.

They MUST NOT change the language semantics.

---

50. Out-of-Core Compilation

For sufficiently large programs, a production implementation SHOULD be architecturally capable of processing data without requiring the entire semantic workload to remain resident in one host-memory structure.

Possible strategies include:

incremental compilation
module-level caching
persistent intermediate artifacts
streamed diagnostics
lazy semantic materialization
partitioned IR

No such mechanism may introduce a language-level size limit.

---

51. Compiler Resource Budgets

Production implementations MAY expose:

compile-time budget
memory budget
expansion budget
diagnostic budget
optimization budget
IR budget
parallelism budget

Every budget MUST have:

name
unit
scope
default policy
configuration mechanism
diagnostic
failure category
recovery behavior

A budget MUST NOT be confused with language semantics.

---

52. Target Adaptation

The compiler MUST adapt target realization without changing source semantics.

Conceptually:

source
  ↓
semantic program
  ↓
resource/capability requirements
  ↓
target discovery
  ↓
legal realization

A larger target may allow more parallelism.

A smaller target may require:

serialization
tiling
partitioning
streaming
decomposition
time multiplexing
distributed execution

where these transformations preserve semantics.

---

53. Scaling Down

POCO-REAF is not only about scaling upward.

The same program should be capable of scaling down when the semantics permit.

Examples:

large machine → small machine
many workers → fewer workers
many accelerators → one accelerator
QPU → simulator
cluster → local execution
GPU → CPU

If the target cannot satisfy a hard requirement, compilation may fail.

It MUST NOT silently change the program's semantics.

---

54. Scaling Up

Scaling up MAY change:

parallelism
tiling
placement
replication
scheduling
memory hierarchy
communication strategy
device utilization

provided the semantic result remains equivalent under the language's defined execution model.

---

55. No Hidden Semantic Dependence on Scale

The following is prohibited:

if available_gpu_count > 1:
    change program meaning

unless the language explicitly defines resource-dependent semantics.

Normal scaling should alter realization, not meaning.

---

56. Numerical Scalability

Numerical behavior MUST be explicitly specified.

Changing target hardware may alter:

precision
rounding
parallel reduction order
floating-point implementation

only when the language semantics explicitly permit those differences.

If bitwise determinism is required, it must be expressed as a semantic guarantee.

Scalability MUST NOT be used as an excuse for unspecified numerical behavior.

---

57. Distributed Numerical Scalability

Parallel or distributed reduction MUST define whether:

associativity
ordering
rounding
determinism

are semantically significant.

If a program requires deterministic reduction, that requirement belongs in semantic/constraint analysis.

It must not depend on accidental node count.

---

58. Quantum Numerical/Physical Scalability

Quantum scaling MUST distinguish:

logical circuit semantics
physical noise
device topology
calibration
routing
scheduling
QEC
resilience

The source program describes the logical computation.

Physical limitations belong downstream.

---

59. QEC Integration

Quantum error correction MUST remain downstream of source syntax.

The grammar may express:

fault tolerance requirements
error correction requirements
noise constraints
logical reliability requirements

but MUST NOT encode a universal maximum based on a particular QEC implementation.

QEC remains responsible for determining a legal implementation.

---

60. ZQN Integration

ZQN MUST consume semantic quantum/resource information without redefining source syntax.

Scalability validation MUST ensure:

grammar
    ↓
quantum semantic model
    ↓
quantum::ir
    ↓
ZQN

and not:

grammar
    ↓
ZQN-specific grammar semantics

ZQN constraints must remain independent of parser capacity.

---

61. Routing Integration

Routing determines physical realization.

Scalability validation MUST verify that routing does not feed physical limits back into grammar validity.

Example:

logical program requires N qubits

is valid if N is semantically valid.

Routing may report:

target cannot realize N logical resources

That is a target/resource failure.

---

62. Scheduling Integration

Scheduling MAY determine:

time
ordering
resource occupancy
parallel execution
device utilization

It MUST NOT redefine the source program's semantic meaning merely because the target has fewer resources.

---

63. Resilience Integration

The repository's resilience model includes states such as:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

Scalability validation MUST ensure that resilience state is runtime/target state, not grammar capacity.

A degraded target may trigger:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

as defined by the execution architecture.

It MUST NOT mutate the language's definition of valid syntax.

---

64. HAL Integration

The hardware abstraction layer owns actual target information.

HAL may know:

device count
memory
topology
qubit count
supported operations
clocking
calibration
power
thermal state
reliability

The grammar must not.

---

65. Backend Integration

A backend may impose implementation constraints.

Validation MUST ensure that backend constraints remain downstream.

Backend-specific rejection MUST be classified appropriately:

TARGET_UNSUPPORTED
RESOURCE_UNAVAILABLE
CAPABILITY_MISSING
CONSTRAINT_UNSATISFIED
IMPLEMENTATION_LIMIT

rather than:

INVALID_ZAMANI_SYNTAX

when the source itself is valid.

---

66. Error Taxonomy

Scalability validation MUST distinguish at least:

LEXICAL_ERROR
SYNTAX_ERROR
AST_ERROR
TYPE_ERROR
SEMANTIC_ERROR
RESOURCE_ERROR
CAPABILITY_ERROR
CONSTRAINT_ERROR
PORTABILITY_ERROR
TARGET_ERROR
IMPLEMENTATION_LIMIT
COMPILATION_BUDGET_EXCEEDED
RUNTIME_RESOURCE_ERROR

The exact canonical diagnostic names must follow the repository's diagnostics contract.

The key invariant is that implementation capacity failure must not masquerade as source-language invalidity.

---

67. Negative Scalability Tests

The test suite MUST contain negative tests for:

- universal capacity constants;
- finite gate enumerations pretending to be exhaustive;
- fixed tensor rank;
- fixed node count;
- fixed thread count;
- fixed register width;
- fixed HDL port count;
- fixed module count;
- fixed nesting depth;
- fixed timeline count;
- physical resource identifiers as universal syntax;
- hidden target selection;
- compiler budget reported as language restriction.

---

68. Positive Scalability Tests

Positive tests MUST demonstrate:

small program
larger program
parameterized program
resource-parametric program
cross-domain program
quantum program
classical program
HDL program
hybrid program
distributed program
AI/data program

without introducing machine-derived grammar limits.

---

69. Boundary Tests

Boundary tests MUST distinguish:

minimum valid quantity
zero where semantically valid
one
large representable value
largest supported semantic representation
invalid negative value where prohibited
invalid overflow
resource unavailable
capability unavailable
target constraint unsatisfied

The boundary is semantic where possible.

It must not merely be the current host's maximum integer.

---

70. Scaling Metamorphic Tests

The validation suite SHOULD use metamorphic testing.

Examples:

same program
different available CPU count

must preserve semantics.

Likewise:

same quantum program
different available qubit capacity

must preserve source semantics.

Likewise:

same distributed program
different node availability

must preserve semantics when both environments satisfy requirements.

The resulting implementation may differ.

---

71. Resource-Availability Matrix

Scalability tests SHOULD vary resource environments.

Example conceptual matrix:

Program requirement| Target resources| Expected result
none| minimal| compile/execute
CPU| CPU| valid
GPU| GPU available| valid
GPU| no GPU| capability/resource failure
quantum| QPU available| valid
quantum| simulator available| valid if simulator satisfies semantics
quantum| no quantum capability| capability failure
N qubits| >= N| realizable
N qubits| < N| resource failure
distributed| sufficient topology| realizable
distributed| insufficient topology| resource/capability failure

The source grammar result must remain independent of these target changes.

---

72. Scaling-Up Test Families

Tests MUST cover progressively larger semantic values without defining a universal maximum.

Examples:

1
2
8
32
128
1024
larger generated values

The numbers above are test points, not language limits.

The suite MUST NOT interpret the largest test point as the maximum supported language capacity.

---

73. Scaling-Down Test Families

The same semantic program SHOULD be evaluated against smaller targets where legal transformations exist.

The expected behavior is:

same semantics
different realization

or:

resource/capability failure

if the requirements cannot be satisfied.

The expected behavior is NOT:

different program meaning

---

74. Grammar-Specific Scalability Validation

Every grammar production that represents a collection MUST be inspected for finite enumeration.

The validator SHOULD identify patterns equivalent to:

item
item item
item item item
...

and flag them when the semantic construct is intended to be unbounded.

Every domain grammar MUST be audited.

---

75. AST-Specific Scalability Validation

Every AST node representing repeated information MUST use a collection abstraction appropriate to its semantics.

Examples:

Vec<T>
slice/reference abstraction
persistent collection
iterator abstraction
domain-specific collection

The choice belongs to implementation architecture.

The AST MUST NOT use:

field_0
field_1
field_2
...
field_1023

as a universal representation.

---

76. Semantic Analysis Scalability

Semantic analysis MUST avoid:

- fixed resource tables;
- finite capability lists where extensibility is intended;
- fixed domain counts;
- fixed topology sizes;
- fixed quantum resource limits;
- hard-coded compiler capacities.

Semantic analysis MUST operate on program-defined collections and environment-provided capabilities.

---

77. IR Scalability

Canonical IR MUST preserve:

logical quantities
resource requirements
capabilities
constraints
effects
source provenance
semantic identity

without converting them prematurely into target-specific fixed resources.

IR must not accidentally lose scalability information.

---

78. Quantum IR Scalability

The canonical "quantum::ir" MUST represent:

logical qubits
operations
parameters
controls
targets
measurements
classical dependencies
resource requirements
capabilities
attributes
source provenance

without universal fixed-size arrays or compiler-defined maxima.

Implementation containers MAY be bounded by available memory.

That is an implementation resource condition, not a language restriction.

---

79. IR Traversal Scalability

IR passes SHOULD avoid recursive traversal where large IR graphs could exhaust the host stack.

Prefer explicit traversal mechanisms where necessary.

Passes MUST preserve:

semantic equivalence
source provenance
resource intent
capability requirements
determinism

---

80. Optimization Scalability

Optimization passes MUST NOT assume:

fixed operation count
fixed block count
fixed qubit count
fixed tensor size
fixed function count

They SHOULD use algorithms whose resource consumption is explicit.

Optimization MAY stop because a configured optimization budget is exhausted.

That MUST NOT make the source invalid.

---

81. Compilation Once

POCO-REAF requires a distinction between:

portable compilation artifact

and:

target-specific deployment artifact

A portable artifact SHOULD preserve enough semantic information to support legal retargeting.

The artifact should identify:

language version
semantic version
feature/dialect versions
target-independent IR identity
resource requirements
capabilities
constraints
provenance

Target-specific lowering may occur later.

---

82. Reproducibility

Given the same:

source
language version
dependencies
dialects
semantic configuration
compiler version

the frontend SHOULD produce deterministic semantic artifacts.

If target information is intentionally part of compilation, that target context must be recorded as provenance.

---

83. Retargeting

Retargeting MUST NOT require rewriting the source merely because the machine changes.

Examples:

CPU → GPU
GPU → FPGA
FPGA → ASIC
CPU → QPU
QPU → simulator
single node → cluster
cluster → cloud

The compiler may need a new target-lowering step.

That is compatible with POCO-REAF provided the source semantic program remains unchanged.

---

84. Compile-Once / Run-Everywhere Interpretation

POCO-REAF MUST be interpreted as:

one source semantic program
+
stable language semantics
+
portable intermediate representation
+
target-independent resource/capability description

rather than as a promise that one binary can execute unchanged on every physical architecture.

A target-specific binary naturally depends on its target.

The architectural goal is to prevent source-level rewriting and preserve portable semantic identity.

---

85. Future Hardware

Scalability validation MUST remain future-compatible.

A future target should be able to advertise:

new capability
new resource kind
new topology
new execution model
new accelerator
new quantum modality

without requiring the core grammar to encode every future machine in advance.

---

86. Vendor Independence

Vendor-specific features MUST NOT become universal grammar assumptions.

A vendor-specific extension belongs under:

dialects/
interoperability/
target-specific lowering

as appropriate.

Portable source should express capability rather than vendor identity.

---

87. Interoperability Formats

External formats such as:

OpenQASM
QIR
LLVM-related representations
MLIR-related representations
HDL formats
WASM
foreign language interfaces

are interoperability boundaries.

They MUST NOT silently become the canonical Zamani semantic model.

Conversion MUST preserve semantic intent or explicitly report unsupported semantics.

---

88. Generated Files

Generated grammar/reference artifacts MUST identify:

source authority
generation tool
generation version
generation timestamp where appropriate

Generated output MUST NOT be manually treated as an independent language authority.

---

89. Documentation Scalability

Every scalability-sensitive grammar file SHOULD document:

Purpose
Status
Owns
Does not own
Inputs
Outputs
Dependencies
Upstream contracts
Downstream consumers
AST mapping
Semantic mapping
IR mapping
Resource mapping
Capability mapping
Positive tests
Negative tests
Boundary tests
Scalability tests
Compatibility tests
Determinism tests
Hard-coding audit
Completion criteria

This is the repository's independent-file completion contract.

---

90. Independent-File Completion Rule

A file is not complete merely because its local syntax is written.

A scalability-sensitive file is complete only when:

syntax defined
AST contract defined
semantic contract defined
IR integration defined
resource integration defined
capability integration defined
diagnostics defined
source spans defined
positive tests defined
negative tests defined
boundary tests defined
scalability tests defined
determinism defined
compatibility defined
hard-coding audit passed
downstream consumers identified

The file must not require undocumented changes merely because another layer is later completed.

---

91. Integration Contract for "grammar/Zamani.g4"

The root grammar MUST:

- compose the domain grammars;
- expose universal program entry;
- expose declaration dispatch;
- expose statement dispatch;
- expose expression entry;
- expose type entry;
- preserve EOF handling;
- avoid duplicating domain ownership;
- avoid finite hardware enumerations;
- avoid machine-derived limits.

"Zamani.g4" remains the canonical root.

It MUST NOT be renamed.

---

92. Integration Contract for "grammar/grammar.md"

"grammar.md" MUST report actual implementation status.

It MUST distinguish:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

and, where useful:

LEXER_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
BACKEND_IMPLEMENTED
TESTED

Scalability claims MUST NOT be marked implemented merely because a specification document exists.

---

93. Integration Contract for "grammar/Zamani-Grammar.md"

"Zamani-Grammar.md" remains a broad design/historical/extended source.

It MUST NOT silently define production syntax.

Features must carry status such as:

stable
proposed
experimental
deprecated
historical
not implemented

A feature becomes production syntax only after passing the full promotion pipeline.

---

94. Feature Promotion

The mandatory feature lifecycle is:

proposal
    ↓
semantic design
    ↓
AST contract
    ↓
grammar contract
    ↓
lexer/parser implementation
    ↓
semantic implementation
    ↓
canonical IR mapping
    ↓
compiler integration
    ↓
runtime/backend integration
    ↓
positive tests
    ↓
negative tests
    ↓
boundary tests
    ↓
scalability tests
    ↓
compatibility tests
    ↓
stable

A feature MUST NOT skip scalability validation.

---

95. Feature-Level Scalability Manifest

Where feature manifests are introduced, every scalability-sensitive feature SHOULD record:

feature_id
name
status
syntax
grammar_files
lexer_tokens
ast_nodes
semantic_rules
ir_mapping
resource_requirements
capabilities
constraints
target_dependencies
scalability_properties
negative_tests
boundary_tests
compatibility_tests
determinism_requirements
hard_coding_policy

The manifest becomes the integration checklist for that feature.

---

96. Validation Categories

The production validation suite MUST contain at least:

grammar validation
lexical validation
parser validation
AST validation
semantic validation
IR validation
resource validation
capability validation
portability validation
scalability validation
determinism validation
diagnostic validation
compatibility validation

---

97. Repository-Wide Hard-Coding Audit

The scalability validator MUST inspect at least:

grammar/Zamani.g4
grammar/lexer/
grammar/core/
grammar/types/
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
grammar/distributed/
grammar/ai/
grammar/data/
grammar/networking/
grammar/security/
grammar/resources/
grammar/compile/
grammar/execution/
grammar/interoperability/
grammar/dialects/
grammar/macros/
grammar/metaprogramming/
grammar/validation/
grammar/tests/
src/

The audit MUST include both source code and documentation where documentation claims an artificial limit as normative.

---

98. Existing Repository Limits Must Be Audited

The presence of an implementation constant named something like:

max_qubits
max_nodes
max_memory

does not automatically mean it is invalid.

Validation MUST determine its role.

It is invalid when it defines a universal language restriction.

It may be valid when it represents:

target capacity
benchmark configuration
test fixture
runtime budget
deployment policy
external format limitation

The semantic classification MUST be documented.

---

99. Test Fixtures

Test fixtures MAY intentionally use fixed values.

For example:

1024 qubits
32 workers
8 GPUs

are valid test data.

The test MUST clearly identify these as fixture values.

A fixture MUST NOT establish a language-wide limit.

---

100. Benchmark Limits

Benchmarks MAY define:

workload size
maximum benchmark duration
target configuration

These are benchmark parameters.

They MUST NOT be interpreted as language constraints.

---

101. Security and Denial-of-Service Protection

Scalability does not mean accepting unbounded work without resource controls.

The compiler MUST be able to protect itself against malicious or accidental resource exhaustion.

Controls MAY include:

time budget
memory budget
expansion budget
diagnostic budget
recursion/depth budget
IR budget
parallelism budget

Such controls MUST be:

explicit
configurable
documented
diagnosable
separate from language validity

Safe Rust MUST be used.

No "unsafe" escape hatch may be introduced merely to improve capacity.

---

102. No Unsafe Rust

Production implementation of scalability validation MUST NOT use:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

This includes:

- parser;
- lexer;
- AST;
- semantic validator;
- scalability validator;
- grammar tooling;
- test infrastructure that is part of production validation.

Safe abstractions MUST be preferred.

---

103. Rust Integer Safety

Rust integer operations used by scalability validation MUST avoid accidental overflow.

Validation logic SHOULD use checked operations where arithmetic can exceed the selected representation.

Overflow MUST result in a defined validation outcome.

It MUST NOT wrap silently.

---

104. Rust Collection Safety

Collection growth MUST be treated as a resource concern.

The implementation MUST NOT assume that:

Vec<T>

can grow indefinitely.

The correct model is:

language cardinality

is logically unbounded within semantic rules, while:

actual allocation

is bounded by available implementation resources.

---

105. Validation of Numeric Resource Quantities

Resource quantities MUST be represented according to their semantic requirements.

The validator MUST avoid reducing all resource quantities to an unnecessarily narrow host integer.

If a quantity can exceed the host representation, validation must produce a defined result rather than silently wrap.

---

106. Capability Registry Scalability

Capability registries SHOULD be extensible.

The core grammar MUST NOT require a finite exhaustive list of all future capabilities.

Capability identity should support:

namespace
name
version where applicable
attributes
parameters

This permits future hardware and execution models.

---

107. Resource Registry Scalability

Resource kinds SHOULD similarly be extensible.

Do not assume the universal resource universe is:

CPU
GPU
FPGA
QPU
RAM

Future resources must be representable without redesigning the fundamental grammar.

---

108. Topology Scalability

Topology syntax MUST represent relationships compositionally.

The grammar MUST NOT enumerate:

topology_1
topology_2
topology_3
...

A topology should be represented by:

nodes
edges
relationships
constraints
properties

with cardinality determined by the program/environment.

---

109. Network Scalability

Network constructs MUST NOT hard-code:

maximum endpoints
maximum links
maximum hops
maximum nodes

as language-wide restrictions.

Actual network limits belong to the environment.

---

110. AI/Data Scalability

AI and data constructs MUST remain parameterized.

No fixed:

tensor rank
dataset size
model layer count
agent count
batch count
feature count

may become a universal grammar restriction.

Framework-specific limits belong outside the language core.

---

111. Nano/Atom-Scale Computing

If nano/atom-scale constructs are supported, they MUST use the same scalability model.

The language may represent:

atoms
molecules
materials
interactions
nano-agents

without assuming a finite universe encoded into the grammar.

Physical validity belongs to semantic/domain models.

---

112. Temporal/Multi-Timeline Scalability

If temporal or multi-timeline constructs are supported, the grammar MUST NOT impose:

maximum timelines
maximum branches
maximum events
maximum history depth

as universal language limits.

Actual execution budgets remain implementation/runtime concerns.

---

113. Sankofa Integration

Sankofa-related concepts such as:

memory
history
recall
learning
provenance
temporal state

must remain semantic constructs.

The grammar must not encode a fixed memory size, history length, or number of remembered entities.

---

114. Portability Classification

Every scalability-sensitive construct SHOULD be classifiable as:

portable
conditionally portable
implementation-defined
target-specific
non-portable

The classification must be explicit.

A target-specific construct MUST NOT masquerade as universally portable.

---

115. Portability Test

For each portable construct:

source
  ↓
target A
target B
target C

must preserve semantic meaning whenever all targets satisfy the required conditions.

Differences in:

performance
parallelism
layout
placement
scheduling
device utilization

are permitted.

Differences in required semantics are not.

---

116. Compatibility

A language version update MUST NOT silently introduce a new artificial scalability limit.

Compatibility validation must compare:

old grammar
new grammar
old AST
new AST
old semantic model
new semantic model
old IR
new IR

and identify changes affecting scalable quantities.

---

117. Backward Compatibility

Existing valid scalable programs MUST remain valid unless a documented language-version change explicitly changes their semantics.

If a previous version accidentally imposed a machine-derived limit, removing that limit is not a reason to preserve the accidental restriction.

---

118. Diagnostics

Scalability diagnostics MUST be actionable.

A diagnostic should identify:

what failed
why it failed
which layer owns the failure
whether the source itself is invalid
whether the target lacks resources
whether the compiler budget was exhausted
what can be changed

A target capacity failure MUST NOT be reported as a syntax error.

---

119. Source Spans in Scalability Errors

Every scalability-related source diagnostic SHOULD preserve the smallest useful source span.

Examples:

resource requirement
capability expression
constraint
target binding
invalid dimension
invalid resource quantity

Source span handling must remain safe and deterministic.

---

120. Completion Criteria for This File

"grammar/validation/scalability.md" is complete when:

- [ ] scope is defined;
- [ ] authority relationships are defined;
- [ ] scalability semantics are defined;
- [ ] artificial-limit policy is defined;
- [ ] implementation-limit policy is defined;
- [ ] target-limit policy is defined;
- [ ] requirement/capability/constraint/preference/hint distinctions are defined;
- [ ] quantum scalability is defined;
- [ ] classical scalability is defined;
- [ ] HDL scalability is defined;
- [ ] hardware scalability is defined;
- [ ] distributed scalability is defined;
- [ ] concurrency scalability is defined;
- [ ] memory scalability is defined;
- [ ] tensor scalability is defined;
- [ ] AI/data scalability is defined;
- [ ] future-domain scalability is defined;
- [ ] AST integration is defined;
- [ ] semantic integration is defined;
- [ ] canonical IR integration is defined;
- [ ] quantum::ir integration is defined;
- [ ] routing integration is defined;
- [ ] scheduling integration is defined;
- [ ] QEC integration is defined;
- [ ] ZQN integration is defined;
- [ ] HAL integration is defined;
- [ ] runtime integration is defined;
- [ ] deterministic behavior is defined;
- [ ] safe-Rust requirement is defined;
- [ ] hard-coding audit is defined;
- [ ] positive tests are defined;
- [ ] negative tests are defined;
- [ ] boundary tests are defined;
- [ ] metamorphic tests are defined;
- [ ] portability tests are defined;
- [ ] compatibility tests are defined;
- [ ] diagnostics are defined;
- [ ] independent-file completion rules are defined.

---

121. Production Acceptance Gate

The scalability system MUST NOT be declared production-ready until all of the following pass.

121.1 Grammar gate

Zamani.g4

contains no accidental finite machine-capacity restriction.

121.2 Lexer gate

"src/lexer.rs" has no machine-derived lexical capacity restriction.

121.3 Parser gate

"src/parser.rs" does not impose arbitrary semantic capacity limits.

121.4 AST gate

AST collections are not replaced with finite machine-shaped fields.

121.5 Semantic gate

Semantic analysis distinguishes program requirements from target capacity.

121.6 IR gate

Canonical IR preserves scalability-relevant semantic information.

121.7 Quantum gate

Quantum syntax lowers through the canonical "quantum::ir".

121.8 Hardware gate

Hardware intent does not become hardware inventory.

121.9 Resource gate

Requirements/capabilities/constraints/preferences/hints remain distinct.

121.10 Backend gate

Backend limitations remain downstream.

121.11 Safety gate

Production implementation uses safe Rust only.

121.12 Test gate

Positive, negative, boundary, scalability, deterministic, compatibility, and portability tests pass.

---

122. Required Repository-Wide Invariants

The following invariants are mandatory.

Invariant A — No universal hardware ceilings

language ≠ current machine capacity

Invariant B — No physical identity leakage

logical resource ≠ physical resource

Invariant C — No target inference

capability ≠ vendor/device selection

Invariant D — No finite domain enumeration

operation universe ≠ finite parser keyword list

Invariant E — No IR duplication

quantum source → canonical quantum::ir

Invariant F — No budget masquerading as semantics

compiler budget ≠ language limit

Invariant G — No unsafe Rust

production implementation = safe Rust

Invariant H — Deterministic semantics

same semantic input → same semantic result

Invariant I — Target adaptation

target change → realization change

not:

target change → source semantic change

Invariant J — Future extensibility

New resource, device, accelerator, topology, operation, or domain MUST be integrable without turning the core language into a finite inventory of today's hardware.

---

123. Final Scalability Model

The complete Zamani scalability model is:

                    Zamani Source
                         │
                         ▼
                    Lexical layer
                         │
                         ▼
                    Syntax layer
                         │
                         ▼
                     Frontend AST
                         │
                         ▼
                Semantic validation
                         │
        ┌────────────────┼────────────────┐
        ▼                ▼                ▼
      Types           Effects          Resources
        │                │                │
        └────────────────┼────────────────┘
                         ▼
                 Capability analysis
                         │
                         ▼
              Canonical semantic model
                         │
        ┌────────────────┼─────────────────┐
        ▼                ▼                 ▼
    Classical        quantum::ir       HDL/Hardware
        │                │                 │
        └────────────────┼─────────────────┘
                         ▼
                    Optimization
                         │
        ┌────────────────┼────────────────┐
        ▼                ▼                ▼
      Routing        Scheduling       Resilience
        │                │                │
        └────────────────┼────────────────┘
                         ▼
                        ZQN
                         │
                         ▼
                        HAL
                         │
                         ▼
                Target capabilities
                         │
                         ▼
                Resource availability
                         │
                         ▼
                 Target realization
                         │
        ┌────────────────┼────────────────────┐
        ▼                ▼                    ▼
       CPU             GPU/FPGA              QPU
        │                │                    │
        └────────────────┼────────────────────┘
                         ▼
                Distributed/HPC/Cloud
                         │
                         ▼
                  Future targets

The source remains the semantic source of truth.

The target remains the realization environment.

---

124. The POCO-REAF Invariant

The final architectural invariant is:

PROGRAM
   ↓
semantic intent
   ↓
portable representation
   ↓
capability/resource matching
   ↓
legal realization

not:

PROGRAM
   ↓
machine-specific assumptions
   ↓
fixed hardware

Therefore:

«Program Once. Compile Once. Run Everywhere. Anywhere. Forever.»

means that Zamani source expresses portable computational meaning, while compilation, optimization, routing, scheduling, resilience, QEC, ZQN, HAL, and runtime systems determine how that meaning is realized on whatever compatible resources are actually available.

The language must scale from atom to everywhere without turning the current machine into the definition of the language.

---

125. Final Rule

The strongest possible production rule is:

«If a Zamani construct is semantically valid, the grammar MUST NOT reject it merely because the compiler, host, runtime, or current target happens to be smaller than the computation.»

Instead:

invalid semantics
        → semantic error

unsupported capability
        → capability error

insufficient target resources
        → resource/target error

unsatisfied constraint
        → constraint error

compiler protection budget exhausted
        → implementation/resource-budget error

unsupported target realization
        → target/backend error

valid portable program
        → remains a valid portable program

This is the required separation that allows Zamani to scale from tiny systems to arbitrarily large systems while preserving one language, one semantic model, one canonical IR architecture, and the POCO-REAF objective.

---

126. File Integration Summary

This file integrates with the repository as follows:

grammar/validation/scalability.md
        │
        ├── validates
        │
        ├── grammar/validation/scalability-rules.md
        ├── grammar/specification/scalability-model.md
        ├── grammar/spec/portability.md
        ├── grammar/spec/type-system.md
        ├── grammar/resources/scalability.g4
        ├── grammar/resources/requirements.g4
        ├── grammar/quantum/resource-requirements.g4
        ├── grammar/hardware/*.g4
        ├── grammar/Zamani.g4
        ├── grammar/grammar.md
        ├── grammar/Zamani-Grammar.md
        ├── grammar/DESIGN.md
        ├── src/lexer.rs
        ├── src/parser.rs
        ├── src/frontend/ast/
        ├── semantic analysis
        ├── canonical IR
        ├── src/quantum/ir/
        ├── optimization
        ├── routing
        ├── scheduling
        ├── resilience
        ├── QEC
        ├── ZQN
        ├── HAL
        └── runtime

It owns validation of scalability.

It does not own:

grammar productions
AST definitions
semantic implementation
IR definitions
routing algorithms
scheduling algorithms
QEC algorithms
ZQN implementation
HAL implementation
runtime implementation

Those remain owned by their respective files and subsystems.

---

127. Definition of Done

The Zamani grammar scalability system may be marked PRODUCTION READY only when:

[PASS] No accidental universal capacity limits
[PASS] No fixed quantum capacity in grammar
[PASS] No fixed classical capacity in grammar
[PASS] No fixed HDL capacity in grammar
[PASS] No fixed hardware inventory
[PASS] No fixed distributed node count
[PASS] No fixed tensor rank
[PASS] No fixed concurrency count
[PASS] No fixed timeline count
[PASS] No hidden target selection
[PASS] Requirement/capability separation
[PASS] Constraint/preference/hint separation
[PASS] Logical/physical resource separation
[PASS] Canonical quantum::ir integration
[PASS] AST scalability
[PASS] Semantic scalability
[PASS] IR scalability
[PASS] Compiler budget separation
[PASS] Target-resource separation
[PASS] Deterministic semantics
[PASS] Safe Rust only
[PASS] Positive scalability tests
[PASS] Negative scalability tests
[PASS] Boundary scalability tests
[PASS] Metamorphic scalability tests
[PASS] Portability tests
[PASS] Compatibility tests
[PASS] Diagnostics classification
[PASS] Future-domain extensibility
[PASS] POCO-REAF architectural invariant

Only after these gates pass should scalability be considered a production property of the Zamani grammar rather than merely a design intention.