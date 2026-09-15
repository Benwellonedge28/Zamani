Zamani Grammar Hard-Coding Audit

Path: "grammar/validation/hardcoding-audit.md"
Status: Normative production specification
Language: Zamani
Grammar technology: ANTLR4-compatible grammar architecture
Implementation baseline: Rust 1.97 / Rust 1.97.1
Safety: Safe Rust only; "unsafe" MUST NOT be used
Portability model: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability model: No accidental language-level finite limits on scalable computation or resources
Primary principle: Zamani source describes computation, intent, requirements, constraints, capabilities, and semantics—not the accidental limitations of the machine currently available.

---

1. Purpose

This document defines the normative hard-coding audit for the Zamani grammar and every grammar component that can introduce an artificial limitation into the language.

The audit exists to prevent accidental coupling between:

- Zamani source programs;
- grammar syntax;
- parser implementation;
- AST representation;
- semantic analysis;
- intermediate representations;
- compiler implementation;
- optimization;
- routing;
- scheduling;
- hardware;
- runtime;
- deployment;
- resource availability;
- physical machine topology.

The central rule is:

«A property that belongs to a particular machine, deployment, backend, implementation, or resource availability MUST NOT silently become a universal Zamani language limit.»

The audit therefore verifies that Zamani can express computations ranging from the smallest practical computation to arbitrarily large computations, subject only to:

- the semantics of the program;
- explicit program requirements;
- explicit target constraints;
- available resources;
- implementation/resource budgets;
- mathematical limits;
- external standards;
- physical reality.

"Infinite scalability" means that Zamani does not introduce an arbitrary finite upper bound where the abstraction itself does not require one.

It does not mean that a finite computer can execute an actually infinite computation.

---

2. Scope

This audit applies to the complete grammar architecture, including:

grammar/
grammar/Zamani.g4
grammar/Zamani-Grammar.md
grammar/README.md
grammar/DESIGN.md

grammar/specification/
grammar/lexer/
grammar/antlr/
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
grammar/compatibility/
grammar/tests/
grammar/examples/

It also applies to grammar consumers and producers elsewhere in the repository where they can impose a language-level limitation.

The audit therefore covers integration with:

ANTLR
lexer
parser
AST
source model
semantic analysis
name resolution
type checking
effect checking
capability checking
resource analysis
classical IR
quantum::ir
QEC
ZQN
optimization
routing
scheduling
hardware abstraction
resource management
resilience
compiler
runtime
deployment
interoperability
serialization
diagnostics
tooling
tests
documentation

---

3. Normative Language

The following terms are normative:

- MUST — mandatory.
- MUST NOT — prohibited.
- REQUIRED — mandatory.
- SHOULD — recommended unless there is a documented reason not to.
- SHOULD NOT — discouraged unless explicitly justified.
- MAY — permitted.
- MAY NOT — prohibited in the specified context.

---

4. File Contract

4.1 Purpose

Define the authoritative process for detecting, classifying, eliminating, documenting, and preventing accidental hard-coding throughout Zamani's grammar architecture.

4.2 Owns

This file owns:

- hard-coding definitions;
- hard-coding classification;
- audit methodology;
- scalability-limit classification;
- machine-coupling detection;
- grammar-level hard-coding rules;
- cross-domain hard-coding rules;
- resource-limit classification;
- target-limit classification;
- implementation-limit classification;
- test-limit classification;
- documentation-limit classification;
- remediation requirements;
- audit evidence requirements;
- hard-coding regression requirements.

4.3 Does Not Own

This file does not own:

- lexical syntax;
- parser syntax;
- identifier syntax;
- type semantics;
- quantum IR;
- classical IR;
- hardware IR;
- QEC;
- ZQN;
- scheduling;
- routing;
- optimization;
- hardware discovery;
- calibration;
- runtime policy;
- deployment policy.

Those systems remain owned by their respective components.

4.4 Inputs

The audit consumes:

- grammar source;
- lexer rules;
- parser rules;
- grammar documentation;
- AST definitions;
- semantic models;
- resource models;
- target models;
- compiler configuration;
- runtime configuration;
- generated grammar artifacts;
- tests;
- examples;
- build scripts;
- documentation;
- existing constants;
- validation diagnostics.

4.5 Outputs

The audit produces:

1. identified hard-coding instances;
2. classification for each instance;
3. remediation decision;
4. ownership decision;
5. integration decision;
6. test requirement;
7. compatibility impact;
8. scalability impact;
9. completion evidence.

4.6 Dependencies

This specification depends conceptually on:

grammar/validation/scalability-rules.md
grammar/validation/naming-rules.md
grammar/validation/semantic-boundaries.md
grammar/validation/ambiguity-rules.md
grammar/validation/compatibility-rules.md
grammar/validation/grammar-validation.md
grammar/compatibility/*
grammar/specification/*
grammar/core/*

The audit MUST NOT duplicate the complete contents of those documents.

---

5. Core Architectural Principle

Zamani has four fundamental layers:

Source Syntax
      ↓
AST / Source Model
      ↓
Semantic Model / Canonical IR
      ↓
Target Realization

The hard-coding rule follows directly:

Language semantics
        ≠
target capabilities
        ≠
resource availability
        ≠
compiler implementation limits
        ≠
runtime limits
        ≠
deployment configuration

A value MUST NOT cross these boundaries implicitly.

For example:

available_qubits = 127

is a target fact.

It MUST NOT become:

MAX_QUBITS = 127

in the Zamani grammar.

Likewise:

CPU has 16 cores

MUST NOT become:

MAX_THREADS = 16

in language syntax.

---

6. Definition of Hard-Coding

A hard-coded value is problematic when a fixed implementation value is embedded into a layer that should instead represent a variable, negotiated, discovered, parameterized, or externally supplied property.

Hard-coding is therefore contextual.

The following is not automatically bad:

8
64
256
1024
4096

The audit asks:

«What does this value mean, and which architectural layer owns that meaning?»

---

7. Hard-Coding Classification

Every discovered fixed value MUST be classified as exactly one primary category.

7.1 Category A — Genuine Language Semantic Constant

A value is a genuine semantic constant when changing it would change the definition of the language or a mathematical operation.

Examples:

boolean has two logical values

or a language-defined mathematical constant.

These MAY remain fixed.

They MUST be documented.

---

7.2 Category B — External Standard Constant

A value belongs to an external standard.

Examples may include:

SHA-256
UTF-8
IEEE-defined formats
protocol-defined widths
standard instruction encodings

Such values MAY be fixed when they are genuinely part of the named external standard.

The implementation MUST NOT confuse the standard's fixed property with a general Zamani resource limit.

---

7.3 Category C — Explicit Program Semantic Requirement

A program MAY explicitly request a fixed value.

For example:

requires qubits >= 128

or an equivalent future Zamani construct.

The "128" is then part of the program's semantics.

It is not a grammar-wide maximum.

---

7.4 Category D — Target Capability

A target may expose:

available_qubits
available_memory
supported_vector_width
supported_instruction_set

These are target properties.

They MUST remain outside universal grammar limits.

---

7.5 Category E — Resource Availability

A runtime or resource manager may determine:

free_memory
available_devices
available_nodes
available_threads
available_quantum_resources

These MUST NOT be encoded into source grammar limits.

---

7.6 Category F — Compiler Resource Budget

A compiler MAY impose configurable operational budgets for safety.

Examples:

maximum compilation memory
maximum diagnostics
maximum macro expansion work
maximum optimization work
maximum source bytes accepted by one invocation

Such values MUST be:

- implementation configuration;
- externally configurable where appropriate;
- separate from language semantics;
- represented as resource/budget failures;
- documented as implementation limits.

They MUST NOT be presented as language restrictions.

---

7.7 Category G — Test Fixture Constant

A test MAY intentionally use:

4 qubits
8 workers
16 nodes

as a fixture.

The test MUST NOT establish that these are language limits.

Tests MUST include additional scale cases proving that the fixture value is not an architectural maximum.

---

7.8 Category H — Documentation Example

Documentation MAY use a convenient number.

The documentation MUST make clear that the number is illustrative when it is not normative.

---

7.9 Category I — Implementation Detail

An internal implementation MAY have bounded structures for engineering reasons.

For example:

buffer capacity
allocation chunk
parser work batch
diagnostic batch
cache size

Such values MUST NOT become source-language limits.

They MUST be treated as implementation details or configurable resource budgets.

---

7.10 Category J — Accidental Hard-Coding

A value is accidental hard-coding when it exists merely because the implementation chose a convenient fixed value even though the underlying concept is scalable.

Examples:

MAX_QUBITS = 32
MAX_CORES = 64
MAX_DEVICES = 8
MAX_NODES = 1024

when those values are not genuine language semantics.

Category J MUST be removed or moved to the correct architectural layer.

---

8. Absolute Prohibition on Accidental Machine Limits

The grammar MUST NOT contain universal limits such as:

MAX_QUBITS
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS
MAX_CORES
MAX_CPUS
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_DEVICES
MAX_NODES
MAX_MEMORY
MAX_REGISTER_COUNT
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_PORTS
MAX_SIGNALS
MAX_PIPELINE_STAGES
MAX_CHANNELS
MAX_SERVICES
MAX_REPLICAS
MAX_OPERATIONS
MAX_GATES
MAX_CIRCUIT_DEPTH
MAX_CIRCUIT_WIDTH

when these are merely implementation or target limitations.

---

9. Hidden Hard-Coding Is Also Prohibited

The audit MUST detect hard-coding even when it does not use a "MAX_*" constant.

The following patterns are equally suspect:

if qubit_count > 64

if device_id > 15

match core_count {
    1 | 2 | 4 | 8 => ...
}

for i in 0..32

q[0]
q[1]

ports[0]
ports[1]
ports[2]

node_001
node_002
node_003

supported_backends = ["cpu", "gpu", "qpu"]

when that list is incorrectly treated as exhaustive.

The audit MUST inspect behavior, not merely identifier names.

---

10. Finite Enumeration Audit

A finite enumeration is suspicious when it represents an extensible domain.

For example:

backend
    : CPU
    | GPU
    | FPGA
    | QPU
    ;

may become an architectural scalability problem if the grammar intends to support arbitrary future accelerators.

Where a domain is open-ended, prefer mechanisms such as:

qualified names
dialects
capabilities
declarations
attributes
targets
extensions

The core grammar MUST NOT need to change merely because a new hardware technology appears.

---

11. Open-World Domain Rule

Zamani domains are open-world unless the specification explicitly declares a closed semantic set.

Therefore:

quantum
classical
hardware
hdl
ai
networking
distributed
accelerator

MUST NOT automatically imply exhaustive finite vocabularies.

Future technologies MUST be representable without requiring a new global keyword whenever existing abstraction mechanisms are sufficient.

---

12. Keyword Hard-Coding

Global keywords create a form of architectural hard-coding.

Every new keyword increases:

- lexer coupling;
- parser coupling;
- compatibility surface;
- identifier collisions;
- dialect coupling;
- tooling requirements.

Therefore a domain feature SHOULD use:

qualified names
declarations
attributes
capabilities
effects
resources
dialects

before introducing a new global keyword.

The keyword list remains owned by the lexer/specification.

---

13. Quantum Hard-Coding Audit

Quantum grammar MUST be audited for:

MAX_QUBITS
MAX_REGISTER_SIZE
MAX_CIRCUIT_WIDTH
MAX_CIRCUIT_DEPTH
MAX_GATE_COUNT
MAX_CONTROL_COUNT
MAX_TARGET_COUNT
MAX_CLASSICAL_BITS
MAX_MEASUREMENT_COUNT

None may become universal language limits merely because a current QPU has such a limitation.

---

14. Quantum Index Audit

The audit MUST detect special treatment of:

q[0]
q[1]
q[2]

or any finite index list.

An indexed reference MUST be modeled compositionally.

Conceptually:

register[index]

rather than:

register[0]
register[1]
register[2]

as a finite grammar universe.

The index itself MAY be constrained by semantic type checking and resource analysis.

---

15. Logical Versus Physical Quantum Resources

The audit MUST ensure that:

logical qubit

is not accidentally represented as:

physical qubit N

by default.

The correct conceptual pipeline is:

Zamani quantum source
        ↓
quantum AST
        ↓
semantic analysis
        ↓
quantum::ir
        ↓
routing
        ↓
physical mapping
        ↓
hardware

The grammar MUST NOT bypass the canonical "quantum::ir" boundary.

---

16. Quantum Gate Enumeration

A grammar MUST NOT require every possible quantum gate to be permanently added to a global finite list.

A stable core MAY define fundamental operations.

Additional operations SHOULD be representable through:

- operation declarations;
- parameterized operations;
- qualified names;
- imports;
- dialects;
- intrinsics;
- capabilities.

This permits future quantum operations without grammar redesign.

---

17. Classical Hard-Coding Audit

The audit MUST inspect:

- integer widths;
- array sizes;
- vector sizes;
- matrix dimensions;
- tensor ranks;
- loop counts;
- thread counts;
- process counts;
- worker counts;
- stack limits;
- heap limits;
- register counts;
- accelerator counts.

A fixed value is acceptable only when its meaning belongs to the type, language, external standard, explicit program requirement, or implementation configuration.

---

18. Tensor Rank Audit

The grammar MUST NOT contain rules equivalent to:

rank <= 4
rank <= 8
rank <= 16

merely because an implementation currently supports those ranks.

Tensor rank MUST remain compositional.

The compiler/backend MAY reject a program when a selected target cannot realize the requested tensor.

That is a target/resource failure.

It is not a grammar failure.

---

19. HDL Hard-Coding Audit

The audit MUST inspect:

modules
ports
signals
wires
registers
processes
states
pipeline stages
clock domains
memory banks
interfaces
instances
hardware parameters

No arbitrary finite count may be embedded into grammar semantics.

For example:

module {
    port*
}

is scalable.

A grammar allowing only a predetermined number of ports is not.

---

20. Hardware Target Audit

Hardware names MUST NOT silently imply:

- vendor;
- model;
- device count;
- topology;
- address;
- capacity;
- clock;
- voltage;
- memory;
- bus width;
- execution unit count.

Hardware identity belongs to target and hardware layers.

---

21. Distributed Hard-Coding Audit

The audit MUST reject architecture that assumes:

N == 8 nodes
N == 16 workers
N == 32 replicas

as universal language constraints.

The source language SHOULD express:

desired replicas
minimum capacity
maximum acceptable latency
required capability
placement preference
fault tolerance requirement

rather than binding itself to a fixed machine topology.

---

22. Concurrency Hard-Coding Audit

The following MUST NOT become universal language semantics:

8 threads
16 workers
32 actors
64 tasks

Concurrency semantics describe:

independence
ordering
synchronization
communication
happens-before
ownership
cancellation
parallel intent

The scheduler/runtime chooses the physical realization.

---

23. Parallelism Audit

A statement such as:

parallel for ...

MUST NOT silently mean:

run on exactly 8 cores

unless an exact worker count is explicitly part of program semantics.

The distinction is:

parallelism intent

versus:

parallel resource allocation

---

24. Memory Audit

The grammar MUST NOT define universal limits for:

heap
stack
allocation count
object count
address space
buffer count
memory size
cache size

Memory requirements belong to semantic/resource analysis.

Actual available memory belongs to the target/runtime.

---

25. Address Audit

The source grammar MUST NOT require physical addresses unless an address is genuinely part of the semantics of the declared construct.

The following are target/deployment properties:

0x40000000
PCI address
MMIO address
device BAR
physical memory address
network endpoint address

They MUST NOT be treated as universal language identity.

If low-level programming requires an address, it must be represented explicitly as a target/system-level semantic value and remain distinguishable from ordinary resource naming.

---

26. Device Identifier Audit

Device identifiers MUST NOT become universal source-level identities.

Bad architecture:

gpu0
gpu1
gpu2

as an implicit machine model.

Better architecture:

accelerator

with:

requirement
capability
placement
target
runtime discovery

determining actual devices.

---

27. Topology Audit

The grammar MUST NOT assume:

linear topology
ring topology
grid topology
2D mesh
3D mesh
fixed QPU connectivity
fixed cluster topology
fixed network topology

unless topology is explicitly part of a program's declared semantics.

Topology-specific source constructs MUST remain explicit.

Routing and placement MUST own target realization.

---

28. Resource Count Audit

All scalable resource collections MUST be represented as dynamic/compositional collections.

Examples:

qubits
cores
threads
devices
nodes
ports
signals
channels
workers
services
replicas
accelerators
memory resources

MUST NOT be bounded by grammar enumeration.

---

29. Resource Model Boundary

The grammar/resource layer MUST distinguish:

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

These concepts MUST NOT collapse into a single "hardware configuration" concept.

For example:

requires quantum

does not mean:

requires IBM device X

and:

requires 128 qubits

does not mean:

use physical qubits 0..127

---

30. Requirement Versus Limit

This distinction is mandatory.

A requirement states what a program needs.

A limit states what the language permits.

Example:

requires qubits >= 128

is a requirement.

Zamani supports at most 128 qubits

is a language limit.

The former is permitted.

The latter is prohibited unless 128 is genuinely part of the language semantics.

---

31. Capability Versus Selection

A capability describes what a target can do.

A target selection describes which target will be used.

These MUST remain separate.

For example:

requires capability quantum

MUST NOT silently select:

device = qpu_7

Target selection belongs downstream.

---

32. Hint Versus Requirement

A hint MAY influence implementation.

It MUST NOT become a semantic requirement unless explicitly declared as one.

For example:

prefer gpu

must not be interpreted as:

must use gpu

unless the language explicitly defines that construct as a hard requirement.

---

33. Optimization Hard-Coding

Optimization code MUST NOT impose language-level validity based on the current optimization implementation.

For example:

if operation_count > 100000
    reject_program

is invalid if the number exists only because the optimizer cannot currently process larger programs.

The optimizer may expose a resource budget.

It MUST NOT redefine language validity.

---

34. Scheduling Hard-Coding

Scheduling MUST NOT leak fixed resource counts into grammar.

Invalid architecture:

scheduler assumes 8 workers
grammar assumes 8 workers

Correct architecture:

program semantics
        ↓
resource requirements
        ↓
hardware capabilities
        ↓
scheduler
        ↓
available resources

Scheduling remains downstream of semantics.

---

35. Routing Hard-Coding

Routing MUST NOT require source programs to enumerate physical connectivity unless physical topology is intentionally part of the program's semantics.

Quantum and hardware routing MUST remain target-specific.

---

36. QEC Hard-Coding

The grammar MUST NOT contain fixed QEC machine limits such as:

MAX_CODE_DISTANCE
MAX_SYNDROME_BITS
MAX_ANCILLA_COUNT
MAX_LOGICAL_QUBITS

unless such a value is genuinely part of a declared language construct or external standard.

QEC algorithms remain owned by QEC.

The grammar only expresses supported source-level concepts.

---

37. ZQN Hard-Coding

ZQN owns noise/fault semantics.

The grammar MUST NOT embed current ZQN model sizes into syntax.

For example:

MAX_NOISE_CHANNELS
MAX_FAULTS
MAX_CORRELATED_QUBITS

MUST NOT become universal grammar limits.

ZQN may report target-specific or model-specific constraints downstream.

---

38. Resilience Hard-Coding

Resilience policies MUST NOT be encoded as grammar-wide fixed values.

Avoid:

retry exactly 3 times
fidelity < 0.95 means failure
maximum recovery attempts = 5

unless explicitly declared as program policy.

Resilience owns:

- diagnosis;
- policy;
- adaptation;
- recovery;
- acceptance;
- escalation.

The grammar may represent these concepts but MUST NOT hard-code the runtime policy.

---

39. Calibration Hard-Coding

Calibration values MUST NOT be embedded as universal grammar constants.

Examples:

gate_error = 0.001
readout_error = 0.01
frequency = 5GHz
temperature = ...

may be program-declared physical parameters when explicitly meaningful.

They MUST NOT become universal defaults that define language semantics.

---

40. AI/ML Hard-Coding

The grammar MUST NOT impose arbitrary limits on:

layers
parameters
tensor rank
batch size
embedding size
model size
dataset size
sequence length
accelerator count

unless a limit is intrinsic to an explicitly defined type or external standard.

---

41. Data Hard-Coding

Data grammar MUST NOT assume:

maximum records
maximum fields
maximum columns
maximum nesting
maximum stream count
maximum collection size

when these are scalable collections.

---

42. Networking Hard-Coding

Networking grammar MUST NOT assume:

fixed number of endpoints
fixed protocol list
fixed message size
fixed number of channels
fixed number of peers
fixed topology

unless an external protocol or declared type defines such a bound.

---

43. Security Hard-Coding

Security grammar MUST NOT hard-code:

fixed identity count
fixed key count
fixed policy count
fixed trust-domain count
fixed cryptographic provider list

as universal limits.

Cryptographic standards may contain fixed parameters; these remain external-standard constants.

---

44. Interoperability Audit

FFI and interoperability are especially sensitive to hard-coding.

The audit MUST distinguish:

Zamani source name

from:

external ABI symbol

and:

provider-specific identifier

No automatic conversion may assume:

snake_case → camelCase

or similar transformations unless explicitly specified.

External symbol spelling MUST be preserved when required by ABI compatibility.

---

45. Dialect Audit

Dialects MUST NOT mutate the global language's resource limits.

A dialect MAY define:

- syntax;
- semantic constructs;
- capabilities;
- domain-specific attributes;
- external standards;
- experimental constructs.

A dialect MUST NOT secretly introduce:

MAX_DEVICES
MAX_QUBITS
MAX_PORTS

as global Zamani limitations.

Dialect-specific constraints MUST remain scoped to that dialect/semantic domain.

---

46. Vendor Extension Audit

Vendor extensions MUST remain namespaced.

A vendor MUST NOT force global language changes for every Zamani program.

Prefer:

vendor::technology::feature

or equivalent registered extension mechanisms.

Vendor hardware limits belong to the vendor target description.

They MUST NOT become universal language constraints.

---

47. Macro Hard-Coding Audit

Macros and compile-time generation are common sources of hidden limits.

The audit MUST inspect:

- expansion depth;
- expansion count;
- generated item count;
- generated token count;
- recursion;
- iteration bounds.

A compiler MAY have configurable expansion budgets.

Those budgets MUST be implementation limits, not language semantic limits.

---

48. Metaprogramming Audit

Compile-time execution MUST NOT assume:

maximum iterations = N
maximum generated declarations = N
maximum generated types = N

unless those are explicit implementation safety budgets.

The compiler MUST distinguish:

program cannot semantically terminate

from:

compiler budget exhausted

---

49. Recursive Grammar Audit

Recursive syntax is not itself hard-coding.

The audit MUST distinguish:

unbounded semantic composition

from:

finite parser implementation stack

A parser implementation MAY have operational depth protection.

Such a protection MUST NOT silently redefine the language.

---

50. Collection Grammar Audit

Scalable collections MUST use compositional repetition.

Preferred:

items
    : item*
    ;

or equivalent.

Not preferred:

items
    : item
    | item item
    | item item item
    ;

The second form introduces an artificial finite structural bound.

---

51. Range Audit

Ranges MUST NOT be expanded by grammar into finite enumerations.

Bad:

index
    : 0
    | 1
    | 2
    | 3
    ;

Good:

index
    : expression
    ;

with semantic validation determining whether the value is legal.

---

52. Numeric Literal Audit

Numeric literal handling MUST distinguish:

lexical representation

from:

host integer representation

A parser MUST NOT accidentally reject valid source integers merely because an intermediate Rust integer type cannot represent them.

If Zamani defines arbitrary-precision or semantically larger numeric literals, parsing and semantic representation MUST preserve that contract.

Host overflow MUST be reported as an implementation/resource/representation issue, not silently converted into source-language semantics.

---

53. Identifier-Length Audit

The grammar MUST NOT impose an arbitrary identifier length such as:

32 characters
64 characters
255 characters
1024 characters

unless explicitly required by a language or external-standard contract.

An implementation MAY have resource limits.

Those MUST be separately configurable and diagnosed.

---

54. Namespace-Depth Audit

Qualified names MUST NOT have an arbitrary fixed maximum depth.

For example:

a::b
a::b::c
a::b::c::d
...

must remain compositional.

The semantic layer may apply namespace-specific rules.

The parser MUST NOT encode an arbitrary maximum number of components.

---

55. AST Hard-Coding Audit

AST structures MUST NOT encode fixed resource counts.

Bad:

struct QuantumCircuit {
    q0: ...
    q1: ...
    q2: ...
    q3: ...
}

Good architectural model:

QuantumCircuit {
    operations: collection
    resources: collection
}

The exact repository type names remain owned by the AST/IR implementation.

The principle is what is normative.

---

56. IR Hard-Coding Audit

Canonical IRs MUST be checked for:

- fixed resource arrays;
- fixed operation counts;
- fixed topology arrays;
- fixed register counts;
- fixed qubit counts;
- fixed device counts.

The IR MUST represent scalable semantics compositionally.

For quantum semantics, the canonical "quantum::ir" remains authoritative.

The grammar MUST NOT introduce an alternate IR merely to work around an IR scalability issue.

---

57. String-Based Semantic Hard-Coding

The audit MUST detect semantics encoded through string comparisons such as:

if name == "gpu"
if name == "qpu"
if name == "device0"
if name == "q0"

when such strings are being used as an implicit universal hardware model.

Open-world names MUST remain open-world.

Semantic registries and typed identities should be used downstream where necessary.

---

58. Hard-Coded Provider Lists

The compiler MUST NOT assume that:

providers = [A, B, C]

is an exhaustive universe unless the specification explicitly defines a closed set.

New providers MUST be addable through the appropriate provider/plugin/dialect/target mechanism without changing the core language semantics.

---

59. Target Selection Audit

Source grammar MUST NOT force:

target = current_machine

unless the program explicitly requests a target-specific compilation.

POCO-REAF requires the default semantic model to remain target-independent.

---

60. Compilation Context Audit

Target-specific facts SHOULD enter through:

compilation context
target description
capability registry
resource model
backend configuration
deployment configuration
runtime discovery

They MUST NOT leak backward into the source grammar.

---

61. Runtime Discovery Audit

Runtime-discovered facts MUST remain runtime facts.

Examples:

available_memory
available_qubits
current_queue_depth
available_gpu_count
network_capacity
current_temperature
device_health

The grammar MUST NOT require source rewriting because these values changed.

---

62. Deployment Audit

Deployment-specific values MUST remain deployment-specific.

Examples:

node count
replica count
region
machine address
cloud instance
device allocation
network endpoint

A portable program MUST not become a different semantic program merely because deployment changes.

---

63. Scheduling/Placement Audit

The source MUST express intent where possible.

The compiler/scheduler may choose:

which core
which GPU
which QPU
which FPGA
which node
which memory region
which route
which schedule

unless the exact selection is explicitly part of the program semantics.

---

64. Physical Address Audit

Physical addresses are among the strongest indicators of target coupling.

Every address-like literal MUST be classified.

The audit asks:

1. Is it language semantics?
2. Is it an external standard?
3. Is it a target declaration?
4. Is it deployment configuration?
5. Is it a test fixture?
6. Is it accidental hard-coding?

Accidental physical addresses MUST be removed from universal grammar semantics.

---

65. Fixed Topology Audit

Any grammar or semantic rule containing:

left
right
neighbor
parent
child
next_device
previous_device

MUST be inspected for hidden topology assumptions.

Topology MUST be represented explicitly when semantically required.

Otherwise target topology belongs downstream.

---

66. Fixed Width Audit

Widths require careful classification.

A width may be:

type semantics
protocol semantics
hardware design semantics
target capability
implementation detail

For example:

u64

may be a language-defined type.

But:

all hardware integers are 64-bit

would be an unacceptable machine assumption.

---

67. Fixed Vector Width Audit

The grammar MUST NOT assume that all vector operations use one physical SIMD width.

For example:

vector<8>

may be semantic if explicitly declared.

But the compiler MUST NOT infer:

vector always means hardware width 8

unless defined by the type semantics.

Physical vectorization belongs to optimization/code generation.

---

68. Fixed Tensor Dimension Audit

Tensor dimensions may be semantic.

However:

tensor dimension = 4096

must not be treated as a universal language limit merely because a current accelerator uses that size.

Dimensions should be represented as:

- explicit semantic dimensions;
- symbolic dimensions;
- runtime dimensions;
- constraints;
- target capabilities.

---

69. Fixed Pipeline Audit

HDL pipeline stages MAY be explicit hardware semantics.

However, the grammar MUST NOT assume:

pipeline has exactly 5 stages

for all hardware.

The program can declare its design.

The compiler/backend may later map or transform it.

---

70. Fixed Clock Audit

A clock frequency is a physical property unless explicitly part of the hardware design semantics.

Therefore:

clock = 5GHz

must not become:

Zamani clock = 5GHz

as a universal rule.

---

71. Fixed Memory Audit

The language MUST distinguish:

memory semantics

from:

physical memory capacity.

For example:

shared memory

does not imply:

shared memory = 64GB

---

72. Fixed Network Audit

Network semantics MUST remain independent of:

number of nodes
bandwidth
latency
number of links
address range
packet size
provider
physical topology

unless explicitly declared.

---

73. Fixed Accelerator Audit

A universal accelerator list is prohibited unless it is explicitly defined as a closed language concept.

Prefer:

requires capability accelerator

over:

requires accelerator type #7

unless the latter is explicitly target-specific.

---

74. Fixed Device Count Audit

The following are prohibited as universal language assumptions:

one QPU
two GPUs
four FPGAs
eight accelerators
sixteen nodes

unless they are explicit program requirements.

---

75. Fixed Resource Names

Names such as:

gpu0
qpu0
node0
cpu0

are not inherently invalid identifiers.

The audit MUST determine their semantic use.

They become architectural hard-coding when the compiler interprets them as universal physical identities.

---

76. Fixed Retry/Recovery Values

Resilience values such as:

retry = 3
recovery_attempts = 5
timeout = fixed

must be classified.

They are valid when explicitly declared program policy.

They are invalid when hidden universal runtime policy.

---

77. Fixed Fidelity Thresholds

A threshold such as:

fidelity < 0.95

MUST NOT become a universal resilience or quantum validity rule unless it is explicitly defined by a semantic contract.

Thresholds should normally come from:

policy
requirements
target characteristics
application semantics
backend configuration

---

78. Fixed Error Rates

The grammar MUST NOT assume universal:

gate error
readout error
decoherence rate
network error
hardware failure rate

These belong to target characterization, ZQN, calibration, or runtime telemetry.

---

79. Fixed Scheduling Durations

The grammar MUST NOT assume universal:

gate duration
instruction duration
network latency
memory latency
clock period
device startup time

Duration syntax may exist.

The value's physical interpretation belongs to the appropriate execution/hardware layer.

---

80. Fixed Topological Coordinates

Source syntax MUST NOT silently interpret:

x = 0
y = 1
z = 2

as physical hardware coordinates unless the program explicitly declares them as such.

---

81. Fixed File-System Assumptions

Module names and qualified names MUST NOT be hard-coded to a particular filesystem layout.

The naming layer MUST distinguish:

semantic module path

from:

filesystem path

and:

URL

This is required for portability across operating systems and deployment environments.

---

82. Fixed Operating-System Assumptions

The grammar MUST NOT require:

Linux
Windows
macOS
RTOS-X

as universal execution environments.

Operating-system capabilities belong to target/runtime integration.

---

83. Fixed Architecture Assumptions

The language MUST NOT assume:

x86
ARM
RISC-V
GPU
QPU
FPGA
ASIC

as the complete set of future architectures.

These are target technologies.

Zamani must remain extensible to architectures that do not yet exist.

---

84. Fixed Instruction-Set Assumptions

Core grammar MUST NOT hard-code every machine instruction.

Low-level instruction support belongs to:

target dialect
backend
ISA layer
interoperability
hardware description

A target-specific instruction may exist without becoming a universal Zamani keyword.

---

85. Fixed Vendor Assumptions

Vendor names MUST NOT be built into universal language semantics merely because the current repository supports a vendor.

Vendor-specific support belongs to:

dialects
targets
providers
interoperability
backend registries

---

86. Fixed Backend Assumptions

The grammar MUST NOT assume a finite backend universe.

A backend can be:

classical
quantum
hardware
simulation
distributed
cloud
embedded
future

without requiring changes to the fundamental source language.

---

87. Fixed Simulator Assumptions

A quantum simulator is not the definition of quantum semantics.

The grammar MUST NOT impose simulator-specific limitations such as:

maximum simulated qubits = N

as source-language limits.

A simulator may have a resource limit.

That belongs to the simulator/backend.

---

88. Fixed Compiler Assumptions

Compiler implementation constraints MUST NOT redefine source validity.

For example:

compiler currently supports 1 million AST nodes

does not mean:

Zamani supports at most 1 million AST nodes

The compiler MUST distinguish:

invalid program

from:

implementation resource exhaustion.

---

89. Fixed Parser Assumptions

Parser implementation constraints MUST be separated from language semantics.

Examples:

maximum nesting depth
maximum token count
maximum source size
maximum parse time

may exist as operational budgets.

They MUST NOT silently become semantic restrictions.

---

90. Fixed Diagnostic Count

The compiler MAY cap emitted diagnostics to avoid resource exhaustion.

For example:

diagnostic_budget

may be configurable.

This MUST NOT imply that Zamani programs may contain only that number of errors or declarations.

---

91. Fixed Source Size

A production compiler MAY have:

maximum source bytes per invocation

for operational safety.

This is an implementation/resource limit.

It MUST NOT be described as a fundamental language limit.

---

92. Fixed Compilation Time

Compilation may be subject to an external timeout.

A timeout MUST NOT turn a valid program into syntactically invalid Zamani.

The compiler should report a resource/budget failure.

---

93. Fixed Optimization Budget

Optimization may be bounded.

The implementation SHOULD support:

optimization_budget

or equivalent configuration.

Failure to complete optimization MUST NOT imply semantic invalidity.

A valid program should remain representable without requiring optimization to succeed completely.

---

94. Fixed Runtime Budget

Runtime may have:

memory budget
time budget
energy budget
execution budget
retry budget

These are execution constraints.

They MUST NOT be embedded into the grammar as universal limits.

---

95. Explicit Constraints Are Allowed

Zamani MUST be able to express explicit constraints.

Examples:

requires capability quantum
requires memory >= required_memory
requires latency <= required_latency
requires reliability >= required_reliability

The precise final syntax is owned by:

grammar/core/requirements.g4
grammar/core/constraints.g4
grammar/resources/*

This document defines the architectural rule, not duplicate syntax.

---

96. Target-Dependent Failure

If a program is valid but cannot run on a particular target, the failure MUST be classified as target/resource incompatibility.

Examples:

insufficient qubits
insufficient memory
unsupported operation
unsupported capability
unavailable accelerator
incompatible topology

These MUST NOT be reported as grammar hard-coding failures.

---

97. POCO-REAF Requirement

Hard-coding audit MUST explicitly verify:

Program Once
    ↓
stable semantic meaning
    ↓
Compile Once
    ↓
portable representation
    ↓
target adaptation
    ↓
Run Everywhere
    ↓
Run Anywhere
    ↓
future-compatible evolution

Changing:

machine size
device count
hardware topology
provider
deployment
runtime availability

MUST NOT inherently require rewriting the source program.

---

98. Compile-Once Boundary

"Compile Once" does not mean that one binary can magically execute on every architecture.

It means that the language's semantic representation and compilation model MUST remain sufficiently target-independent that source semantics do not need to be rewritten for every target.

Target-specific lowering remains legitimate.

---

99. Future Hardware Rule

A hardware technology that does not exist today MUST be representable through Zamani's extensibility architecture without requiring the language to have predicted its exact properties.

Therefore:

new hardware

SHOULD enter through:

capability
dialect
target
resource model
hardware description
backend
interoperability

rather than modification of unrelated core grammar rules.

---

100. Hard-Coding Detection Method

The production audit MUST combine:

1. lexical search;
2. structural search;
3. semantic review;
4. dependency analysis;
5. test analysis;
6. documentation analysis;
7. generated-code inspection.

Simple text search alone is insufficient.

---

101. Required Search Patterns

The audit SHOULD search for patterns including:

MAX_
MIN_
LIMIT
COUNT
SIZE
CAPACITY
QUANTITY
WIDTH
DEPTH
RANK
DEVICE
QUBIT
CORE
THREAD
GPU
FPGA
ASIC
NODE
PORT
SIGNAL
REGISTER
MEMORY
ADDRESS
TOPOLOGY
LATENCY
TIMEOUT
RETRY
FREQUENCY
BANDWIDTH

It MUST also search for numeric literals near these concepts.

---

102. Required Numeric Audit

The audit MUST inspect suspicious numeric literals such as:

0
1
2
4
8
16
32
64
128
256
512
1024
2048
4096
8192
65536

This does not mean these values are prohibited.

It means their architectural meaning MUST be established.

---

103. Required Pattern Audit

Search for:

array[0]
array[1]
array[2]

for i in 0..N

if count > N

count == N

match count

Vec::with_capacity(N)

const N: usize = ...

[Type; N]

HashMap::from([...])

match target

if device == ...

These patterns require classification.

They are not automatically defects.

---

104. Rust-Specific Audit

All Rust implementations related to grammar validation MUST be compatible with:

Rust 1.97
Rust 1.97.1

and MUST use safe Rust.

The implementation MUST NOT introduce:

unsafe

or depend on unsafe mechanisms to circumvent scalability constraints.

---

105. Rust Host Integer Audit

Rust host integer types MUST NOT accidentally become Zamani semantic limits.

For example:

usize
u32
u64

may be implementation representations.

They MUST NOT automatically define the source-language maximum unless the language specification explicitly says so.

---

106. Rust Array Audit

Rust fixed-size arrays:

[T; N]

MUST NOT be used for semantically unbounded collections merely because a convenient fixed size was selected.

Use appropriate dynamic/compositional representations where the semantic collection is scalable.

---

107. Rust Stack Audit

Recursive Rust implementations MUST be reviewed for deep source programs.

If a language structure is semantically recursive, an implementation MUST NOT introduce an arbitrary source-language nesting limit merely because Rust recursion could exhaust the stack.

Where appropriate, use explicit worklists/stacks.

---

108. Rust Allocation Audit

Dynamic allocation is permitted in safe Rust.

Allocation strategy MUST NOT become semantic language limits.

Allocation failure must be distinguishable from invalid source.

---

109. Determinism Requirement

Hard-coding audit results MUST be deterministic.

The result MUST NOT depend on:

- hash-map iteration order;
- machine CPU count;
- available memory;
- backend ordering;
- provider ordering;
- filesystem ordering;
- network state.

Where diagnostics are emitted, ordering SHOULD be stable.

---

110. No Environment-Dependent Grammar

The grammar MUST NOT change its accepted language merely because the compiler is running on:

small machine
large machine
CPU
GPU host
QPU host
cluster
cloud
embedded system

Environment-dependent validation belongs downstream.

---

111. No Runtime-Dependent Parsing

The parser MUST NOT ask the runtime:

how many qubits are available?
how many GPUs exist?
how much memory exists?
how many nodes exist?

in order to determine whether ordinary Zamani syntax is valid.

---

112. No Hardware Discovery in Grammar

Grammar code MUST NOT perform hardware discovery.

It MUST NOT directly depend on:

- device enumeration;
- calibration;
- network discovery;
- runtime inventory;
- provider APIs.

The grammar produces syntax structures.

---

113. No Network Dependency

Naming and hard-coding validation MUST NOT require network access.

A source program MUST be parseable independently of current provider availability.

---

114. No Filesystem Dependency

Grammar validation MUST NOT require access to:

/dev
/sys
/proc
hardware device files
provider configuration
machine-specific paths

unless a separate compiler phase explicitly performs target analysis.

---

115. No Provider Dependency

The grammar MUST NOT directly import provider-specific libraries merely to parse general Zamani.

Provider integration belongs downstream.

---

116. Cross-Domain Hard-Coding Audit

Every cross-domain construct MUST be inspected.

Required combinations include:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

The audit must ensure that one domain does not impose its physical limits on another.

---

117. Quantum + Classical

A quantum program MUST be able to interact with classical computation without assuming:

fixed classical register size
fixed measurement count
fixed number of classical threads

---

118. Quantum + Hardware

Quantum syntax MUST remain logical until physical mapping.

The flow remains:

quantum source
    ↓
quantum semantics
    ↓
quantum::ir
    ↓
routing/scheduling
    ↓
hardware realization

---

119. Quantum + Distributed

Distributed quantum computation MUST NOT assume a fixed number of:

QPU nodes
links
logical processors
quantum channels

unless explicitly required.

---

120. HDL + Hardware

HDL semantics MAY describe physical constraints intentionally.

However, the grammar MUST distinguish:

declared hardware design

from:

current target hardware availability.

---

121. AI + Accelerator

AI programs MUST express accelerator requirements through capabilities/resources rather than assuming one fixed accelerator architecture.

---

122. Distributed + Runtime

The number of actual nodes MUST be runtime/deployment state unless explicitly part of the program requirement.

---

123. Validation Ownership

The hard-coding audit MUST NOT become a duplicate semantic analyzer.

Its role is:

detect architectural scalability violations

not:

resolve every semantic property.

---

124. Diagnostic Contract

Hard-coding diagnostics SHOULD use stable codes.

Recommended taxonomy:

HARD001  accidental machine limit
HARD002  fixed resource count
HARD003  fixed topology
HARD004  fixed hardware identity
HARD005  hidden finite enumeration
HARD006  fixed quantum resource limit
HARD007  fixed classical resource limit
HARD008  fixed HDL resource limit
HARD009  fixed distributed resource limit
HARD010  fixed accelerator assumption
HARD011  target fact embedded in grammar
HARD012  runtime fact embedded in grammar
HARD013  provider fact embedded in grammar
HARD014  implementation limit presented as language rule
HARD015  hidden index bound
HARD016  fixed collection cardinality
HARD017  fixed namespace depth
HARD018  fixed numeric representation
HARD019  non-deterministic scalability behavior
HARD020  prohibited unsafe workaround

The final implementation MAY refine codes, but codes MUST remain stable once published.

Provider-specific error codes MUST NOT enter this taxonomy.

---

125. Diagnostic Severity

The implementation SHOULD classify findings as:

ERROR
WARNING
INFORMATION

Suggested policy:

ERROR

A source/grammar architecture genuinely violates the scalability contract.

WARNING

A suspicious implementation pattern requires review but may be legitimate.

INFORMATION

A fixed value has been classified and documented as legitimate.

---

126. Audit Record

Each production audit finding SHOULD contain:

Location:
Symbol:
Value:
Category:
Owner:
Reason:
Impact:
Classification:
Required Action:
Compatibility Impact:
Scalability Impact:
Test:
Status:

Example:

Location:
grammar/quantum/example.g4

Symbol:
MAX_QUBITS

Value:
64

Category:
J — Accidental Hard-Coding

Owner:
grammar

Impact:
Prevents larger programs

Required Action:
Remove grammar-wide limit and move target capacity to resource analysis

Status:
FAIL

---

127. False Positive Handling

A hard-coding audit MUST permit legitimate fixed values.

Every accepted fixed value MUST have an explanation.

For example:

64

could be:

SHA-512 external standard

or:

incorrect MAX_QUBITS

The audit must distinguish them.

---

128. Fixed Value Registry

A repository-wide registry SHOULD document intentionally fixed values.

Recommended conceptual structure:

fixed-value
semantic-owner
reason
scope
source
compatibility-status
scalability-class
test

This prevents repeated reclassification of legitimate constants.

The registry MUST NOT become a catalog of machine limits.

---

129. Hard-Coding Remediation

Every finding MUST result in one of:

KEEP
PARAMETERIZE
MOVE
GENERALIZE
NAMESPACE
MAKE OPEN-WORLD
MAKE TARGET-SPECIFIC
MAKE RESOURCE-SPECIFIC
MAKE PROGRAM-SPECIFIC
MAKE CONFIGURABLE
DOCUMENT
DEPRECATE
REMOVE

---

130. KEEP

Use "KEEP" only when the value is genuinely semantic.

Evidence is required.

---

131. PARAMETERIZE

Use "PARAMETERIZE" when the value should depend on program input, generic parameters, expressions, or resource requirements.

---

132. MOVE

Use "MOVE" when a value belongs to another subsystem.

Examples:

grammar → hardware target
grammar → runtime
grammar → resource manager
grammar → scheduler
grammar → ZQN

---

133. GENERALIZE

Use "GENERALIZE" when a finite representation must become compositional.

---

134. NAMESPACE

Use "NAMESPACE" when a vendor/provider/domain-specific value should not pollute global syntax.

---

135. MAKE OPEN-WORLD

Use "MAKE OPEN-WORLD" for extensible domains such as:

hardware
accelerators
quantum operations
providers
dialects
protocols

where appropriate.

---

136. MAKE TARGET-SPECIFIC

Use when a value describes a particular machine.

---

137. MAKE RESOURCE-SPECIFIC

Use when a value describes availability.

---

138. MAKE PROGRAM-SPECIFIC

Use when a value is genuinely part of program semantics.

---

139. MAKE CONFIGURABLE

Use when the value is an implementation safety/resource budget.

---

140. REMOVE

Use when the value has no legitimate semantic purpose.

---

141. Compatibility Rule

Removing or changing a hard-coded language restriction can be a compatibility-sensitive change.

For example, expanding:

maximum accepted structure

is generally less disruptive than narrowing it.

However, changing a parser from:

fixed enumeration

to:

open-world name

may affect semantic resolution.

All such changes MUST be reviewed against:

grammar/compatibility/*
grammar/validation/compatibility-rules.md

---

142. Documentation Synchronization

If a hard-coded value is removed, all documentation MUST be audited.

Search:

README
DESIGN
Zamani-Grammar
grammar.md
examples
tests
comments

for the old limit.

Documentation MUST NOT continue claiming a limit that the implementation removed.

---

143. Generated Artifact Audit

Generated ANTLR files and other generated artifacts MUST NOT be treated as independent language specifications.

The audit MUST trace:

authoritative grammar
    ↓
generation
    ↓
generated parser/lexer

A generated artifact MUST NOT introduce a manual semantic restriction.

---

144. Source-of-Truth Rule

Hard-coding decisions MUST trace back to the authoritative language specification.

If two files disagree about a limit:

1. identify authority;
2. classify the conflicting file;
3. remove duplication;
4. update derived documentation;
5. add regression test.

---

145. Existing Repository Integration

The audit MUST integrate with the current repository rather than creating a parallel architecture.

At minimum, the following existing areas MUST be considered:

grammar/Zamani.g4
grammar/Zamani-Grammar.md
grammar/README.md
grammar/DESIGN.md
grammar/antlr/
grammar/core/
grammar/validation/
grammar/quantum/
grammar/classical/
grammar/hardware/
grammar/compile/
grammar/execution/
grammar/dialects/

The repository already contains dedicated scalability, naming, ambiguity, compatibility, semantic-boundary, and grammar-validation specifications; this document is the cross-cutting audit that verifies those rules are not violated by hidden fixed values.

---

146. Integration With "grammar/core/names.g4"

"core/names.g4" owns structural name syntax.

The hard-coding audit MUST verify that it does not contain:

finite namespace depth
finite identifier universe
finite domain names
fixed hardware names
fixed provider names

Naming policy remains owned by:

grammar/validation/naming-rules.md

---

147. Integration With Lexer

The lexer owns lexical structure.

The hard-coding audit MUST ensure that lexer rules do not accidentally encode:

finite numeric ranges
finite identifier lengths
finite Unicode subsets
finite domain identifiers

unless these are genuine lexical rules.

---

148. Integration With Types

Type grammar MUST distinguish:

semantic type bound

from:

hardware representation bound.

For example:

u64

may be a semantic type.

But a backend's preference for 64-bit registers must not redefine the language type system.

---

149. Integration With Expressions

Expressions MUST remain compositional.

The audit MUST inspect:

- expression nesting;
- argument counts;
- operand counts;
- indexing;
- ranges;
- tensor dimensions;
- collection sizes.

No arbitrary fixed count is permitted.

---

150. Integration With Statements

Statement collections MUST remain scalable.

A block MUST NOT have a fixed number of statements.

Loops MUST NOT have fixed iteration counts unless the source explicitly declares them.

---

151. Integration With Declarations

Declarations MUST remain collections.

There MUST be no finite language-wide limits on:

types
variables
constants
fields
variants
implementations
interfaces

---

152. Integration With Functions

Functions MUST support scalable:

parameters
generic parameters
arguments
local declarations
operations
nested blocks

Any implementation budget is external to language semantics.

---

153. Integration With Modules

Module systems MUST support arbitrarily compositional namespaces.

No fixed module count or namespace depth may be encoded.

---

154. Integration With Effects

Effect sets MUST be extensible.

The grammar MUST NOT assume that:

IO
Quantum
Hardware
Network
Security

are the final universe of effects.

Custom effects and dialects must remain possible.

---

155. Integration With Memory

Memory grammar MUST remain independent of:

machine RAM
address width
cache hierarchy
NUMA topology
physical allocation limits

---

156. Integration With Concurrency

Concurrency grammar MUST remain independent of:

CPU count
worker count
scheduler width
executor count

---

157. Integration With Classical

Classical grammar MUST remain scalable across:

scalar
vector
matrix
tensor
numerical
symbolic
accelerator

without physical-width assumptions.

---

158. Integration With Quantum

Quantum grammar MUST remain independent of current QPU dimensions.

Canonical quantum semantics continue downstream through:

quantum::ir

The grammar MUST NOT duplicate the canonical IR.

---

159. Integration With Hybrid

Hybrid constructs MUST NOT make either classical or quantum resource counts universal.

---

160. Integration With HDL

HDL grammar MAY express exact design dimensions when they are actual design semantics.

It MUST NOT convert those dimensions into universal hardware limits.

---

161. Integration With Hardware

Hardware grammar owns hardware-specific descriptions.

It MUST distinguish:

declared hardware

from:

available hardware

and:

deployment hardware.

---

162. Integration With Distributed

Distributed grammar MUST express logical distributed semantics independently of actual node inventory.

---

163. Integration With AI/Data

AI/data grammar MUST remain resource-parametric.

Data/model size is a program/resource property, not a parser maximum.

---

164. Integration With Networking

Networking grammar MUST distinguish protocol semantics from current network capacity.

---

165. Integration With Security

Security grammar MUST distinguish cryptographic standard constants from provider/runtime configuration.

---

166. Integration With Resources

"grammar/resources/*" is a primary integration boundary.

The correct flow is:

program requirement
        ↓
resource expression
        ↓
capability analysis
        ↓
target selection
        ↓
resource allocation

not:

grammar
        ↓
hard-coded machine limit

---

167. Integration With Compilation

Compilation MAY introduce target-specific constraints.

Those constraints MUST remain distinguishable from language syntax validity.

---

168. Integration With Execution

Execution owns:

- scheduling;
- dispatch;
- runtime capabilities;
- deployment;
- actual resource availability.

The grammar MUST NOT duplicate these decisions.

---

169. Integration With Interoperability

External technologies may have fixed constraints.

The audit MUST verify that these constraints remain scoped to their external interface and do not become universal Zamani limits.

---

170. Integration With Dialects

Dialect-specific limits MUST remain dialect-scoped.

A dialect MUST NOT silently modify core language scalability.

---

171. Integration With Macros

Macro expansion budgets are implementation limits.

The grammar MUST remain independent of those budgets.

---

172. Integration With Metaprogramming

Compile-time execution limits MUST be reported as implementation/resource constraints.

---

173. Integration With "quantum::ir"

The hard-coding audit MUST verify:

grammar
   ↓
quantum AST
   ↓
semantic lowering
   ↓
quantum::ir

and MUST reject architecture where:

grammar
   ↓
hardware-specific quantum representation

bypasses the canonical IR.

---

174. Integration With QEC

QEC consumes canonical semantic/IR structures.

QEC-specific capacities MUST remain QEC/target/resource concerns.

---

175. Integration With ZQN

ZQN describes noise/fault semantics.

Its model size and backend limitations MUST NOT become grammar-wide limits.

---

176. Integration With Optimization

Optimization consumes semantic/IR structures.

Optimizer implementation budgets MUST remain implementation constraints.

---

177. Integration With Routing

Routing consumes logical computation and target topology.

The grammar MUST NOT encode target topology merely to make routing easier.

---

178. Integration With Scheduling

Scheduling consumes operations, dependencies, resources, and timing constraints.

Scheduler capacity MUST NOT become grammar capacity.

---

179. Integration With Hardware HAL

Hardware HAL owns actual capabilities and state.

The grammar MUST remain independent of hardware discovery.

---

180. Integration With Resource Management

Resource management owns actual allocation and availability.

The grammar expresses requirements and constraints.

---

181. Integration With Resilience

Resilience owns adaptation/recovery decisions.

The grammar MUST NOT encode current resilience policy as universal semantic constants.

---

182. Integration Graph

The required architectural direction is:

                    specification
                         │
                         ▼
                       lexer
                         │
                         ▼
                       parser
                         │
                         ▼
                         AST
                         │
                         ▼
                  semantic analysis
                         │
             ┌───────────┼────────────┐
             ▼           ▼            ▼
        type system   effects     resources
             │           │            │
             └───────────┼────────────┘
                         ▼
                  canonical IRs
                   │         │
             quantum::ir   other IRs
                   │         │
          ┌────────┴─────────┴─────────┐
          ▼                            ▼
     optimization                 target analysis
          │                            │
          ▼                            ▼
       routing                    capabilities
          │                            │
          └────────────┬───────────────┘
                       ▼
                   scheduling
                       │
                       ▼
                 hardware HAL
                       │
                       ▼
                    runtime
                       │
                       ▼
                  deployment

The hard-coding audit crosses all boundaries but MUST NOT reverse this dependency direction.

---

183. Forbidden Circular Dependency

The following architecture is prohibited:

grammar
   ↓
runtime
   ↓
hardware discovery
   ↓
grammar

Also prohibited:

quantum grammar
   ↓
hardware grammar
   ↓
quantum grammar

and:

grammar
   ↓
IR
   ↓
grammar

---

184. Hard-Coding Prevention Principle

The audit is not merely retrospective.

Every new grammar feature MUST be designed so that accidental hard-coding is difficult to introduce.

Before adding a fixed value, the developer MUST answer:

What does this value mean?
Who owns that meaning?
Is it semantic?
Is it target-specific?
Is it resource-specific?
Is it an implementation budget?
Is it only a test fixture?
Can the abstraction remain compositional instead?

---

185. Required Review Questions

Every grammar PR introducing a number MUST answer:

1. Why is the number necessary?
2. Is it normative?
3. What architectural layer owns it?
4. Can the value vary per program?
5. Can the value vary per target?
6. Can the value vary at runtime?
7. Does it limit scalability?
8. Does it affect POCO-REAF?
9. Does it affect compatibility?
10. Is a test proving the distinction?

---

186. Scalability Definition

For this audit:

«Scalable means that the language does not impose an arbitrary finite maximum on a concept whose cardinality is determined by program semantics, target capabilities, resource availability, or future technology.»

Examples:

number of qubits
number of cores
number of threads
number of devices
number of nodes
number of operations
number of modules
number of ports
number of signals
number of tensor dimensions

are therefore unbounded at the language-model level unless their abstraction explicitly defines a finite semantic domain.

---

187. Practical Resource Limits

No software implementation can literally provide infinite memory, infinite parsing time, or infinite execution resources.

Therefore:

unbounded language semantics

MUST be distinguished from:

finite implementation resources.

A resource exhaustion error MUST NOT be mislabeled as:

invalid Zamani syntax

---

188. Tiny-to-Large Requirement

The same language model MUST support:

one variable

through:

very large programs

and:

one qubit

through:

large logical quantum systems

without changing the grammar's fundamental resource model.

The practical limit is determined by available resources and implementation capacity.

---

189. Future-Proofing Requirement

The hard-coding audit MUST consider technologies not currently present in the repository.

A new technology MUST be integrable without assuming that today's categories are exhaustive.

Examples of potentially future domains:

new quantum computing models
neuromorphic computing
photonic computing
optical computing
biological computing
analog computing
molecular computing
reconfigurable computing
new accelerators
new memory architectures
new networking models
new distributed models

The grammar architecture MUST remain extensible.

---

190. Example: Bad Quantum Architecture

const MAX_QUBITS: usize = 32;

register
    : 'qubits' '[' index32 ']'
    ;

Problems:

- machine limit embedded in grammar;
- finite resource model;
- future QPU incompatibility;
- simulator coupling;
- violates POCO-REAF.

Classification:

HARD001
HARD006
HARD015

---

191. Example: Correct Quantum Architecture

Conceptually:

qubit_register
    : identifier
    ;

qubit_reference
    : qualifiedName
    | identifier '[' expression ']'
    ;

Then:

semantic analysis
        ↓
resource requirements
        ↓
quantum::ir
        ↓
target analysis
        ↓
routing

The exact grammar production belongs to the quantum/core grammar.

---

192. Example: Bad Parallel Architecture

parallel_for_8

where "8" means compiler-selected worker count.

This makes a runtime implementation detail part of source semantics.

---

193. Example: Correct Parallel Architecture

parallel_for

expresses parallelism.

A resource requirement can separately express a desired or required execution property.

The scheduler determines realization.

---

194. Example: Bad Hardware Architecture

gpu0
gpu1
gpu2
gpu3

as the complete GPU universe.

---

195. Example: Correct Hardware Architecture

accelerator

plus:

capability
requirement
target
placement

Actual device identity is downstream.

---

196. Example: Bad Distributed Architecture

cluster<8>

where 8 is secretly the maximum number of nodes supported by the compiler.

---

197. Example: Correct Distributed Architecture

cluster<desired_count>

where "desired_count" is semantic or resource information.

The runtime determines actual placement.

---

198. Example: Legitimate Fixed Value

sha256

The "256" is part of the named external algorithm.

It is not a machine resource limit.

Classification:

Category B — External Standard / Algorithm Identity

---

199. Example: Legitimate Type Width

u64

may be legitimate because its width is part of the type's semantic contract.

It MUST NOT imply:

all machine registers are 64-bit

---

200. Example: Legitimate Program Requirement

required_qubits = 128

is legitimate when the value is part of the program's actual requirement.

The compiler may determine:

target has 127

and reject execution for that target.

The grammar remains valid.

---

201. Test Requirements

Every hard-coding-sensitive component MUST have tests.

Tests MUST include:

positive
negative
boundary
scalability
determinism
cross-domain
compatibility
round-trip

where applicable.

---

202. Positive Hard-Coding Tests

Tests MUST demonstrate valid scalable structures.

Examples:

many declarations
many parameters
many operations
many quantum targets
many ports
many signals
many nodes
many resources
deep qualified names
large symbolic dimensions

---

203. Negative Hard-Coding Tests

Tests MUST detect:

fixed resource maximums
illegal target binding
hidden topology assumptions
fixed enumeration
hard-coded device IDs
invalid finite collections
implementation limits presented as semantics

---

204. Boundary Tests

Boundary tests MUST include:

minimum valid structure
single-resource programs
large resource counts
large numeric values where supported
deep structures
large qualified names
large operation collections
large declarations

The test suite MUST prove that selected values are examples rather than hidden maxima.

---

205. Scalability Test Pattern

For a scalable quantity "N", tests SHOULD use multiple values:

N = 1
N = small
N = medium
N = large
N = substantially larger

The exact values MUST NOT themselves become architectural limits.

---

206. Cross-Domain Tests

At minimum test:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

The purpose is to detect hidden assumptions crossing domain boundaries.

---

207. Determinism Tests

Repeated audit runs against identical source and repository state MUST produce identical results.

The result MUST NOT depend on:

machine size
CPU count
memory availability
backend availability
hash ordering
filesystem ordering
network state

---

208. Regression Tests

Every fixed-value finding that is removed MUST gain a regression test.

The test MUST fail if the old hard-coded limit is accidentally reintroduced.

---

209. Property-Based Testing

Where practical, property-based tests SHOULD generate scalable collections and verify:

validity does not unexpectedly change at arbitrary collection size

Examples:

N declarations
N quantum operations
N ports
N nodes
N resource requirements

The implementation MUST avoid a test generator that itself imposes the architectural maximum.

---

210. Fuzzing

Grammar fuzzing SHOULD include:

- large collections;
- deeply nested expressions;
- large qualified names;
- large numeric literals;
- unusual Unicode where supported;
- large quantum operation lists;
- mixed-domain programs.

Fuzzing MUST distinguish:

parser failure

from:

resource exhaustion

---

211. Resource-Exhaustion Tests

The compiler SHOULD have separate tests for operational limits.

For example:

resource budget exhausted

must not be confused with:

source syntax invalid

This distinction is essential for POCO-REAF.

---

212. Hard-Coding Audit of Examples

Every example MUST be inspected.

An example such as:

4 qubits

is acceptable.

However, documentation MUST NOT imply:

Zamani supports exactly 4 qubits.

Examples are not specifications.

---

213. Hard-Coding Audit of Documentation

Search all grammar documentation for phrases such as:

maximum
at most
limited to
supports only
up to
fixed number
exactly N

Every occurrence MUST be classified.

---

214. Hard-Coding Audit of Comments

Comments can create hidden architectural contracts.

Comments such as:

supports 32 qubits

MUST be removed or clearly classified if they describe obsolete implementation limits.

---

215. Hard-Coding Audit of Tests

Tests are especially dangerous because test fixtures can accidentally become semantic assumptions.

A test using:

8 nodes

must not establish:

8 nodes is the maximum.

---

216. Hard-Coding Audit of Generated Code

Generated code MUST be regenerated from authoritative source after grammar changes.

Manually patched generated artifacts MUST NOT become a second source of language limits.

---

217. Hard-Coding Audit of CI

CI MUST test the scalability contract.

CI SHOULD search for suspicious constructs such as:

MAX_QUBITS
MAX_CORES
MAX_DEVICES
MAX_NODES

and equivalent hidden patterns.

Static checks MUST allow documented legitimate constants.

---

218. Hard-Coding Audit of Build Scripts

Build scripts MUST NOT select a fixed machine model and then make that model a language requirement.

Build configuration belongs outside source semantics.

---

219. Hard-Coding Audit of Tooling

Language servers, formatters, syntax highlighters, and IDE tooling MUST NOT impose semantic limits merely for convenience.

For example:

formatter supports only 32 parameters

is a tooling limitation, not a language limitation.

---

220. Hard-Coding Audit of Serialization

Serialization MUST preserve scalable collections.

It MUST NOT truncate:

operations
qubits
ports
nodes
resources
metadata

at an arbitrary fixed count.

---

221. Hard-Coding Audit of Caches

Compiler caches may be finite.

Cache capacity MUST NOT affect semantic correctness.

Cache eviction is an implementation concern.

---

222. Hard-Coding Audit of Incremental Compilation

Incremental compilation MUST NOT require fixed numbers of:

files
modules
symbols
dependencies

at the semantic level.

---

223. Hard-Coding Audit of Symbol Tables

Symbol tables MUST support scalable namespaces.

Implementation hash-map capacity is not a language limit.

---

224. Hard-Coding Audit of Name Resolution

Name resolution MUST NOT contain:

maximum namespace depth
maximum imports
maximum aliases
maximum symbols

unless those are explicitly resource budgets.

---

225. Hard-Coding Audit of Type Checking

Type checking MUST NOT contain arbitrary limits on:

generic parameters
type nesting
tuple elements
fields
variants
constraints

unless a type system rule genuinely requires them.

---

226. Hard-Coding Audit of Effect Checking

Effect sets MUST remain extensible and compositional.

---

227. Hard-Coding Audit of Capability Checking

Capabilities MUST be represented as extensible semantic properties.

A fixed list of current hardware capabilities MUST NOT become the universal future capability universe.

---

228. Hard-Coding Audit of Resource Checking

Resource checking MUST compare:

program requirement

against:

target availability

rather than compare source syntax against:

grammar constant.

---

229. Hard-Coding Audit of Runtime

Runtime limits must remain runtime limits.

A runtime may say:

not enough resources

without changing the meaning of the source program.

---

230. Hard-Coding Audit of Deployment

Deployment systems may impose:

quota
capacity
availability
region restrictions

These are deployment facts.

They MUST NOT alter language syntax.

---

231. Compatibility With Naming Rules

"naming-rules.md" owns name policy.

This audit verifies that names do not encode accidental resource limits.

For example:

Qubit64

is not automatically invalid.

It becomes problematic when "64" is used to represent a hidden language-wide maximum.

Naming validation and hard-coding validation MUST remain separate but integrated.

---

232. Compatibility With Scalability Rules

"scalability-rules.md" defines the general scalability model.

This file operationalizes that model through an audit.

No contradictory limit may be introduced here.

---

233. Compatibility With Semantic Boundaries

"semantic-boundaries.md" owns layer separation.

Hard-coding violations often indicate boundary violations.

Examples:

grammar → hardware
grammar → runtime
grammar → calibration
grammar → routing
grammar → scheduler

must be detected.

---

234. Compatibility With Ambiguity Rules

A finite enumeration introduced to avoid parser ambiguity MUST NOT be used when a scalable syntactic abstraction is possible.

If a finite keyword set is necessary for disambiguation, the reason MUST be documented.

---

235. Compatibility With Grammar Validation

Grammar validation verifies syntax correctness.

Hard-coding audit verifies scalability architecture.

The two MUST NOT be conflated.

---

236. Compatibility With Compatibility Rules

Hard-coding changes may alter accepted syntax.

Every such change MUST be assessed for:

source compatibility
AST compatibility
semantic compatibility
IR compatibility
tooling compatibility
serialization compatibility
dialect compatibility

---

237. No Silent Narrowing

A change MUST NOT silently reduce the range of programs accepted by Zamani.

If a genuine semantic restriction is introduced, it requires:

- specification change;
- compatibility assessment;
- migration documentation;
- tests;
- explicit versioning.

---

238. No Silent Widening With Semantic Change

Even removing a limit can require semantic review if the old limit accidentally affected meaning.

The implementation MUST establish that widening the representable domain does not change unrelated semantics.

---

239. Audit Workflow

The production workflow is:

1. Discover
2. Locate
3. Classify
4. Determine owner
5. Determine semantic meaning
6. Determine scalability impact
7. Determine compatibility impact
8. Choose remediation
9. Implement remediation
10. Add regression test
11. Audit documentation
12. Audit generated artifacts
13. Run cross-domain tests
14. Run deterministic audit
15. Record completion

---

240. Dependency-First Integration

This file does not require grammar implementation to depend on runtime/hardware.

Its integration order is:

specification
    ↓
lexer/core syntax
    ↓
AST
    ↓
semantic model
    ↓
resource/capability model
    ↓
canonical IR
    ↓
target analysis
    ↓
hardware/runtime

The hard-coding audit verifies that no later layer leaks backward into source syntax.

---

241. Independent Completion Contract

This file is complete when the following are defined:

Purpose
Ownership
Non-ownership
Classification
Detection
Remediation
Integration
Diagnostics
Testing
Compatibility
Scalability
Rust safety
Completion criteria

No later grammar file should need to redefine what constitutes accidental hard-coding.

---

242. Per-File Hard-Coding Contract

Every grammar file MUST be auditable using:

File:
Purpose:
Scalable Concepts:
Fixed Values:
Fixed-Value Classification:
Owner:
Non-Owner:
Target Dependencies:
Runtime Dependencies:
Resource Dependencies:
Potential Hard-Coding:
Required Tests:
Compatibility Impact:
Completion Status:

This makes each file independently reviewable.

---

243. Required File-Level Audit

Before declaring any grammar file complete:

1. Search for fixed values.
2. Search for finite enumerations.
3. Search for fixed collection cardinality.
4. Search for hardware identifiers.
5. Search for topology assumptions.
6. Search for target-specific values.
7. Search for runtime assumptions.
8. Search for hidden numeric bounds.
9. Search documentation.
10. Search tests.
11. Check cross-domain integrations.
12. Record all legitimate constants.
13. Add regression tests.
14. Confirm no accidental limits remain.

---

244. Required Directory-Level Audit

Before declaring a grammar directory complete:

all files audited
all cross-file dependencies audited
all constants classified
all domain boundaries verified
all tests passing
documentation synchronized
generated artifacts synchronized
no duplicate limits
no conflicting limits

---

245. Required Repository-Level Audit

Before declaring "grammar/" production-ready:

every grammar file audited
every validation document reconciled
every domain audited
every resource concept audited
every target concept audited
every hardware concept audited
every compiler limit classified
every runtime limit classified
every test fixture reviewed
every documentation limit reviewed
all generated artifacts verified
all cross-domain tests passing
all hard-coding regressions passing

---

246. Production CI Gate

The repository SHOULD have a CI gate conceptually equivalent to:

grammar-hardcoding-audit

The gate MUST fail on newly introduced unclassified accidental hard-coding.

It SHOULD permit approved fixed values through an explicit classification mechanism rather than blanket suppression.

---

247. No Blanket Suppression

The following policy is prohibited:

ignore all numeric literals

or:

ignore all MAX_* constants

The audit MUST classify values individually or through explicit trusted categories.

---

248. No Test-Only Escape Hatch

A hard-coded production limit MUST NOT be hidden behind:

#[cfg(test)]

or equivalent mechanisms if it changes semantic behavior under testing.

Tests must faithfully validate production scalability.

---

249. No Debug-Only Escape Hatch

Debug/release differences MUST NOT create different language semantics.

---

250. No Platform-Specific Language

The same Zamani source semantics MUST remain stable across host operating systems and architectures.

Platform-specific behavior belongs to target/runtime layers.

---

251. Safe Rust Requirement

All implementation work derived from this specification MUST use safe Rust.

Forbidden:

unsafe { ... }

Forbidden as an architectural workaround:

raw pointer tricks
unsafe transmutation
unchecked memory manipulation
unsafe FFI solely to bypass limits

Safe abstractions MUST be preferred.

---

252. Error Taxonomy

Hard-coding errors MUST remain distinct from:

syntax errors
type errors
name-resolution errors
resource errors
target errors
runtime errors
provider errors

For example:

HARD001

means the architecture itself contains an accidental scalability restriction.

It must not be used for:

target has insufficient qubits

---

253. Resource Error Example

Correct:

Program requires 128 qubits.
Selected target provides 127.

This is a target/resource incompatibility.

Incorrect:

Zamani grammar only permits 127 qubits.

---

254. Target Error Example

Correct:

Operation requires capability X.
Target does not provide X.

Incorrect:

Parser rejects operation because current backend does not support X.

---

255. Runtime Error Example

Correct:

Execution resource became unavailable.

Incorrect:

Program is syntactically invalid because a runtime resource disappeared.

---

256. Portability Rule

A source program MUST NOT require modification solely because:

CPU count changed
GPU count changed
QPU count changed
memory changed
node count changed
device topology changed
provider changed
deployment changed

unless the program explicitly made that physical property part of its semantics.

---

257. Semantic Stability Rule

Changing a target MUST preserve source semantic meaning.

The implementation may change:

layout
schedule
routing
optimization
instruction selection
device assignment
parallelization

without changing program meaning.

---

258. Resource Adaptation Rule

Resource differences SHOULD be handled through:

capability negotiation
resource analysis
lowering
optimization
routing
scheduling
partitioning
distribution
runtime adaptation
resilience

rather than source rewriting.

---

259. Hard-Coding and POCO-REAF

The audit MUST reject any architecture where:

source program
        ↓
machine-specific constant
        ↓
semantic meaning

is unavoidable.

The desired direction is:

source semantics
        ↓
requirements
        ↓
capabilities
        ↓
target mapping

---

260. "Forever" Requirement

"Forever" requires versioned extensibility.

Hard-coded current technology lists MUST NOT prevent future evolution.

The grammar MUST be able to evolve through:

versioning
dialects
namespaces
capabilities
extensions
interoperability
migration rules

without destabilizing existing semantic meaning.

---

261. Audit Completion States

Each audited component MUST have one state:

NOT_AUDITED
AUDITED
PASS
PASS_WITH_DOCUMENTED_LIMITS
REQUIRES_REMEDIATION
BLOCKED

"PASS" means no unresolved accidental hard-coding exists.

---

262. Completion Criteria for This File

"grammar/validation/hardcoding-audit.md" is complete when:

- the hard-coding definition is normative;
- legitimate fixed values are distinguishable from accidental limits;
- resource limits are separated from language limits;
- target limits are separated from language limits;
- compiler limits are separated from language limits;
- runtime limits are separated from language limits;
- quantum limits are addressed;
- classical limits are addressed;
- HDL limits are addressed;
- hardware limits are addressed;
- distributed limits are addressed;
- AI/data limits are addressed;
- networking/security limits are addressed;
- dialect/vendor limits are addressed;
- numeric values are classified;
- finite enumerations are classified;
- hidden limits are addressed;
- "quantum::ir" ownership is preserved;
- grammar/runtime/hardware dependency direction is preserved;
- Rust 1.97/1.97.1 compatibility is defined;
- "unsafe" is prohibited;
- diagnostics are defined;
- regression testing is defined;
- repository integration is defined;
- CI enforcement is defined;
- POCO-REAF is explicitly protected.

---

263. Final Hard-Coding Rule

The following rule is absolute:

«If a value represents a scalable property of the machine, hardware, resource pool, topology, deployment, backend, or runtime, it MUST NOT become a universal Zamani grammar limit merely because the current implementation happens to use that value.»

Instead, classify and place it correctly:

Program semantics
        ↓
requirements / constraints / preferences / hints
        ↓
capabilities
        ↓
target description
        ↓
resource availability
        ↓
compilation
        ↓
routing / scheduling / optimization
        ↓
hardware
        ↓
runtime

---

264. Final Architectural Principle

Zamani MUST follow:

One Program
    ↓
One Stable Semantic Meaning
    ↓
Many Compilations / Target Realizations
    ↓
Many Architectures
    ↓
Many Hardware Configurations
    ↓
Many Resource Scales
    ↓
Many Execution Environments
    ↓
Future Technologies

The grammar MUST therefore describe:

what the program means
what it requires
what it permits
what it prefers
what capabilities it needs
what constraints it imposes

rather than:

what machine happens to exist today.

---

265. Production Readiness Checklist

Before merging a grammar component:

[ ] No accidental MAX_* resource limit
[ ] No hidden numeric resource limit
[ ] No finite machine enumeration
[ ] No fixed device universe
[ ] No fixed topology
[ ] No fixed hardware address
[ ] No fixed provider universe
[ ] No fixed backend universe
[ ] No fixed qubit limit
[ ] No fixed CPU/core/thread limit
[ ] No fixed GPU/FPGA/ASIC limit
[ ] No fixed node/cluster limit
[ ] No fixed memory capacity
[ ] No fixed tensor-rank limit
[ ] No fixed HDL resource count
[ ] No fixed collection cardinality
[ ] No arbitrary namespace-depth limit
[ ] No arbitrary identifier-length limit
[ ] No parser limit presented as language semantics
[ ] No compiler limit presented as language semantics
[ ] No runtime limit presented as language semantics
[ ] No target property embedded in grammar
[ ] No hardware discovery in grammar
[ ] No network dependency in grammar validation
[ ] No filesystem dependency in grammar validation
[ ] No provider dependency in grammar validation
[ ] Canonical IR ownership preserved
[ ] quantum::ir boundary preserved
[ ] Resource/capability separation preserved
[ ] Target/resource failures distinguished from syntax failures
[ ] Deterministic behavior verified
[ ] Cross-domain tests pass
[ ] Scalability tests pass
[ ] Regression tests exist
[ ] Documentation synchronized
[ ] Compatibility impact reviewed
[ ] Rust 1.97/1.97.1 compatible
[ ] No unsafe Rust
[ ] CI hard-coding audit passes

---

266. Final Acceptance Condition

"grammar/" MUST NOT be considered production-ready until the repository can demonstrate:

No accidental hard-coded scalability limits
        +
Correct ownership of every legitimate fixed value
        +
Explicit separation of language semantics and target reality
        +
Resource-parametric grammar
        +
Open-world extensibility
        +
Canonical IR integration
        +
Deterministic validation
        +
Safe Rust implementation
        +
Repository-wide regression coverage

The resulting architecture is:

Zamani source
      ↓
portable syntax
      ↓
portable semantics
      ↓
resource/capability requirements
      ↓
canonical IR
      ↓
target-independent compilation
      ↓
target-specific realization
      ↓
runtime adaptation
      ↓
execution

Therefore:

«Zamani is not allowed to become smaller merely because today's machine is small, nor must it become a different language merely because tomorrow's machine is larger.»

The language-level model remains scalable from the smallest practical computation to arbitrarily large computations, with actual execution bounded only by the resources, capabilities, constraints, and physical realities of the selected environment.

POCO-REAF:

Program Once
Compile Once
Run Everywhere
Run Anywhere
Run Forever

is preserved by making machine-scale properties data, requirements, capabilities, constraints, targets, and runtime state—not accidental grammar constants.