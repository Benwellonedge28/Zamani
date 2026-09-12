Below is the complete proposed grammar/specification/scalability-model.md. It treats scalability as a language and semantic contract, not as a promise that physical hardware is infinite. “Infinity” means Zamani imposes no arbitrary finite upper bound where the semantics do not inherently require one; actual execution remains bounded by available representation, resources, implementation, physics, and policy.

# Zamani Scalability Model

**Path:** `grammar/specification/scalability-model.md`

**Status:** Normative  
**Language:** Zamani  
**Grammar subsystem:** `grammar/`  
**Primary purpose:** Define the production scalability contract for Zamani  
**Target implementation baseline:** Rust 1.97 / Rust 1.97.1  
**Implementation safety requirement:** No `unsafe` Rust  
**Architecture principle:** Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

## 1. Purpose

This document defines the normative scalability model of the Zamani programming language.

It establishes how Zamani programs scale across:

- program size;
- data size;
- memory capacity;
- processor count;
- core count;
- thread count;
- vector width;
- accelerator count;
- GPU resources;
- FPGA resources;
- ASIC resources;
- quantum resources;
- logical qubits;
- physical qubits;
- distributed nodes;
- network resources;
- storage resources;
- heterogeneous systems;
- embedded systems;
- edge systems;
- cloud systems;
- clusters;
- supercomputers;
- future computing architectures.

The central rule is:

> **Zamani source semantics MUST NOT impose arbitrary finite limits on scalable machine resources.**

Zamani describes computation, intent, requirements, constraints, capabilities, effects, and semantics.

The target environment determines how those semantics are realized within available resources.

The scalability model therefore separates:

1. **language semantics**;
2. **program requirements**;
3. **target capabilities**;
4. **resource availability**;
5. **implementation limits**;
6. **execution policy**;
7. **physical limits**.

These categories MUST NOT be conflated.

---

# 2. Normative Language

The keywords:

- **MUST**
- **MUST NOT**
- **REQUIRED**
- **SHALL**
- **SHALL NOT**
- **SHOULD**
- **SHOULD NOT**
- **MAY**

are normative.

Unless explicitly stated otherwise, a rule in this document applies to:

- the grammar;
- parser;
- AST;
- semantic analysis;
- compiler;
- canonical IR;
- runtime;
- tooling;
- target integrations;
- interoperability layers.

---

# 3. Scope

This specification owns the scalability semantics of Zamani.

It defines:

- absence of arbitrary language-level resource limits;
- scale-independent source semantics;
- resource abstraction;
- requirements;
- capabilities;
- constraints;
- preferences;
- hints;
- target realization;
- dynamic resource discovery;
- resource negotiation;
- scale adaptation;
- compile-time scalability;
- runtime scalability;
- distributed scalability;
- quantum scalability;
- hardware scalability;
- data scalability;
- concurrency scalability;
- numerical scalability;
- portability;
- artifact scalability;
- execution scalability;
- graceful failure when resources are insufficient;
- scalability testing;
- hard-coding auditing.

---

# 4. Non-Goals

This specification does NOT own:

- lexical syntax;
- parser implementation;
- AST implementation;
- canonical quantum IR;
- optimization algorithms;
- routing algorithms;
- scheduling algorithms;
- QEC algorithms;
- ZQN noise models;
- hardware discovery;
- hardware calibration;
- runtime implementation;
- memory allocator implementation;
- operating-system implementation;
- physical hardware design;
- cluster orchestration;
- compiler backend algorithms.

Those systems consume the contracts defined here.

---

# 5. Fundamental Scalability Principle

Zamani MUST follow:

```text
Program semantics
        ↓
Requirements / constraints / preferences
        ↓
Compilation
        ↓
Target capabilities
        ↓
Resource realization
        ↓
Execution

and NOT:

Program
  ↓
hard-coded machine size
  ↓
hard-coded resource assumptions
  ↓
machine-specific semantics

A program MUST describe what computation means.

A target determines how that computation is realized.


---

6. Definition of "Infinity"

Zamani uses "infinity" as a language scalability principle.

It does NOT mean that physical execution can consume infinite resources.

For this specification:

> Infinity means that Zamani does not impose an arbitrary finite upper bound on a scalable resource unless that bound is itself part of the language's semantics or representation model.



For example, Zamani MUST NOT define:

MAX_QUBITS = 32
MAX_THREADS = 1024
MAX_GPUS = 8
MAX_NODES = 256
MAX_MEMORY = 1 TB

as language-wide limits.

Actual execution may still be constrained by:

available memory;

address-space representation;

integer/index representation;

compiler implementation;

runtime implementation;

operating system;

hardware;

network;

storage;

target capabilities;

execution policy;

energy;

time;

physical laws.


Those limitations are environmental or implementation limitations rather than arbitrary language semantics.


---

7. Scale Dimensions

Scalability is multidimensional.

Zamani MUST NOT treat "scale" as only the number of processors.

The language SHALL account for at least:

Program scale
Data scale
Memory scale
Compute scale
Parallelism scale
Concurrency scale
Quantum scale
Hardware scale
Accelerator scale
Distributed scale
Network scale
Storage scale
Deployment scale
Compilation scale
Execution scale
Precision scale
Performance scale
Fault scale
Reliability scale

Future dimensions MAY be added without changing the fundamental scalability model.


---

8. Program Scale

A Zamani program MUST NOT have an arbitrary language-level maximum source size.

The implementation MAY impose practical limits caused by:

parser memory;

compiler memory;

filesystem limits;

process limits;

implementation architecture.


Such limits MUST NOT be encoded as semantic restrictions unless explicitly documented as part of the language representation.

The grammar MUST therefore avoid constructs such as:

program ::= exactly_1024_statements

or equivalent hidden restrictions.


---

9. Module Scale

Programs MAY contain arbitrarily many:

modules;

namespaces;

declarations;

functions;

types;

imports;

exports;

compilation units.


The grammar MUST use recursive or parameterized structures rather than finite enumeration.

For example, a module collection MUST conceptually behave like:

module*

rather than:

module1
module2
...
module1024


---

10. Declaration Scale

The language MUST support arbitrary numbers of declarations subject to available implementation resources.

This includes:

variables;

constants;

functions;

types;

interfaces;

implementations;

quantum declarations;

hardware declarations;

resources;

capabilities;

effects;

distributed services;

data schemas.


No grammar rule may impose an arbitrary declaration count.


---

11. Data Scale

Zamani MUST allow data structures to scale according to available resources.

This applies to:

arrays;

vectors;

matrices;

tensors;

maps;

sets;

streams;

records;

collections;

datasets;

distributed datasets;

quantum registers where semantically applicable.


A type such as:

array<T, N>

MUST NOT imply a language-wide maximum value of N.

If N is semantically part of the type, its representation MUST be sufficiently general for the supported language representation.


---

12. Resource Count Independence

The grammar MUST NOT encode fixed counts for scalable resources.

Forbidden architectural assumptions include:

cpu0
cpu1
cpu2
cpu3

as a universal machine model.

Likewise:

gpu0
gpu1
gpu2
gpu3

or:

q0
q1
q2
...
q31

MUST NOT define the universal resource space.

Identifiers MAY exist for target-specific resources, but they belong to target/deployment/resource descriptions rather than universal language semantics unless the programmer explicitly chooses a hardware-specific dialect.


---

13. Semantic Requirements Versus Resource Requirements

A critical distinction MUST be maintained.

13.1 Semantic requirement

A semantic requirement describes what the program needs for its meaning.

Examples:

requires quantum capability
requires measurement
requires coherent quantum operation
requires persistent storage
requires floating-point arithmetic
requires distributed communication

13.2 Resource requirement

A resource requirement describes a quantity or property needed for realization.

Examples:

requires available memory
requires computational capacity
requires sufficient qubit capacity
requires network bandwidth

13.3 Target requirement

A target requirement specifies an execution class or capability.

Examples:

requires target capability "quantum"
requires target capability "fpga"
requires target capability "gpu"

It MUST NOT automatically imply a specific physical device.


---

14. Requirements, Constraints, Preferences, and Hints

Zamani MUST distinguish:

Requirement
Constraint
Preference
Hint
Capability
Resource
Target
Placement

These are not interchangeable.

Requirement

A condition that MUST be satisfied.

Constraint

A condition that realization MUST obey.

Preference

A desired property that MAY be relaxed when policy permits.

Hint

Information supplied to improve implementation decisions but which does not define semantics unless explicitly declared as such.

Capability

A property supported by a target.

Resource

An available execution entity or capacity.

Target

An execution environment or class of environments.

Placement

A mapping of logical computation to physical resources.


---

15. Resource Abstraction

Resource expressions MUST be abstract enough to support:

tiny machines;

large machines;

heterogeneous systems;

distributed systems;

quantum systems;

future systems.


The source language SHOULD express properties such as:

capacity
availability
throughput
latency
precision
energy
reliability
connectivity
memory
compute capability
quantum capability
communication capability

without embedding a fixed machine inventory.


---

16. Resource Discovery

Resource discovery MUST NOT be performed by the grammar.

The grammar MAY express resource requirements.

Resource discovery belongs to:

compiler target selection;

hardware abstraction;

runtime;

deployment;

resource management.


This prevents:

grammar → hardware discovery

from becoming an architectural dependency.

Instead:

grammar
  ↓
semantic representation
  ↓
compiler/runtime
  ↓
resource discovery


---

17. Dynamic Resource Availability

A target MAY expose resources dynamically.

Examples include:

CPU availability;

GPU availability;

FPGA availability;

quantum backend availability;

node availability;

memory availability;

network availability;

accelerator availability.


A program MUST NOT change its semantic meaning merely because resource availability changes.

If resources become insufficient, the implementation MUST:

1. adapt within the permitted semantic contract;


2. defer execution;


3. select another valid realization;


4. request additional resources;


5. invoke resilience mechanisms where supported;


6. or report failure.



It MUST NOT silently change program semantics.


---

18. Scale-Invariant Program Meaning

For a portable program P:

Meaning(P)

MUST be independent of:

CPU count;

GPU count;

FPGA count;

physical qubit count;

node count;

machine topology;

deployment topology.


Execution is instead modeled conceptually as:

Execution =
    Realize(
        Meaning(P),
        Target,
        Capabilities,
        Resources,
        Constraints,
        Policy
    )

Different realizations MAY use different:

processors;

memory;

accelerators;

physical qubits;

nodes;

schedules;

routes;

optimizations.


Provided the permitted observable semantics are preserved.


---

19. Scaling Up

When additional resources become available, an implementation MAY scale execution by using:

more parallelism;

more workers;

more nodes;

more accelerators;

larger memory;

wider vectors;

more quantum resources;

greater throughput;

additional pipeline capacity.


The source program SHOULD NOT require modification merely because the target becomes larger.


---

20. Scaling Down

A program SHOULD also be capable of execution on a smaller target when its semantic requirements can still be satisfied.

Scaling down MAY involve:

serialization;

batching;

tiling;

partitioning;

scheduling;

streaming;

checkpointing;

resource sharing;

compilation;

alternate implementation strategies.


Such transformations belong to compilation/runtime systems.

They MUST preserve the semantic contract.


---

21. Elasticity

Zamani MAY support execution whose resource allocation changes during execution.

Examples:

scale up
scale down
resource join
resource leave
resource replacement
backend replacement
accelerator replacement
node replacement

The semantics of elasticity MUST be explicitly defined.

Resource elasticity MUST NOT silently alter observable program behavior where the language contract requires stability.


---

22. Classical Processor Scalability

The language MUST NOT assume:

one CPU;

one core;

fixed core count;

fixed instruction set;

fixed register count;

fixed vector width.


The compiler MAY lower a program to:

scalar execution;

SIMD;

multicore execution;

task parallelism;

data parallelism;

GPU execution;

accelerator execution;

distributed execution.


These are implementation choices unless explicitly expressed as semantic requirements.


---

23. Thread Scalability

The language MUST NOT impose a universal maximum thread count.

Concurrency abstractions SHOULD be expressed in terms of:

tasks;

actors;

logical workers;

parallel regions;

data partitions;

execution resources.


A runtime MAY choose the number of actual threads.

The programmer SHOULD NOT have to rewrite the semantic program merely because the runtime uses:

1 thread
10 threads
1000 threads
1,000,000 logical tasks


---

24. Concurrency Scalability

Concurrency semantics MUST be independent of the number of execution workers.

The language MUST define:

ordering;

synchronization;

visibility;

ownership;

communication;

cancellation;

failure behavior.


The runtime determines the physical execution width.

No grammar component may encode a fixed worker topology as a universal semantic assumption.


---

25. Distributed Scalability

Distributed programs MUST NOT assume a fixed number of nodes.

The language SHOULD support abstractions such as:

node
service
partition
replica
channel
message
distributed resource
placement requirement
consistency requirement

without requiring:

node[0..15]

as a universal model.

A deployment MAY contain:

1 node
10 nodes
10,000 nodes

or another resource count.


---

26. Distributed Topology

Physical topology MUST be separated from logical topology.

Logical relationships may express:

communication;

dependencies;

replication;

consistency;

locality requirements.


Physical realization may determine:

node placement;

network routes;

racks;

links;

regions;

availability zones.


Topology-sensitive semantics MUST only be used when topology is explicitly part of the program's meaning.


---

27. Network Scalability

Network constructs MUST NOT encode a fixed number of:

endpoints;

connections;

channels;

peers;

nodes.


The network layer MUST support target-driven realization.

Network requirements MAY express:

latency bounds;

bandwidth requirements;

reliability;

ordering;

encryption;

availability;

locality.


Actual network topology belongs to deployment/runtime systems.


---

28. Storage Scalability

Zamani MUST distinguish logical storage from physical storage.

A program MAY express:

persistent data
stream
dataset
object
record
transaction

without embedding:

disk = 1 TB

as a universal assumption.

Physical storage capacity is a resource property.


---

29. Memory Scalability

The language MUST distinguish:

logical memory requirement

from:

physical memory capacity

The language MUST NOT impose an arbitrary universal maximum memory size.

Memory implementation MAY use:

stack;

heap;

shared memory;

distributed memory;

unified memory;

accelerator memory;

persistent memory;

external storage.



---

30. Address Scalability

Physical addresses MUST NOT be part of portable language semantics by default.

An address may become semantically relevant in:

hardware-specific programming;

embedded systems;

MMIO;

device programming;

HDL;

explicit low-level dialects.


Such use MUST be explicit and isolated from portable semantics.


---

31. Quantum Scalability

Quantum scalability MUST follow the same principle as classical scalability.

The language MUST NOT impose a universal maximum number of:

qubits;

logical qubits;

physical qubits;

registers;

quantum registers;

quantum operations;

circuit depth;

measurement results.


A quantum program MUST express logical quantum computation.

Physical realization determines:

physical qubit count;

topology;

connectivity;

native gates;

coherence;

calibration;

timing;

error characteristics.



---

32. Logical and Physical Qubits

The language MUST distinguish:

logical quantum identity

from:

physical hardware identity

A portable program MUST NOT silently acquire physical qubit identities.

For example, a semantic operation equivalent to:

apply operation to logical qubit q

MUST NOT inherently mean:

apply to physical qubit 0

Routing determines the physical mapping.


---

33. Quantum Register Scalability

Quantum registers MAY contain a program-defined number of logical quantum elements.

The grammar MUST represent the collection abstractly.

It MUST NOT encode a fixed register size.

Invalid architecture:

qreg ::= q0 | q1 | ... | q31

Valid architectural principle:

quantum collection → parameterized/indexed logical elements


---

34. Quantum Measurement Scalability

Quantum measurement MUST remain explicit.

The grammar/runtime MUST NOT introduce hidden measurements merely because a target has limited resources.

In particular:

> The execution system MUST NOT automatically measure every qubit at program completion unless that behavior is explicitly part of the language/execution contract.



Measurement is an observable semantic operation.


---

35. Quantum State Scalability

Zamani MUST NOT model arbitrary quantum state as an ordinary infinitely-copyable classical value.

Quantum state semantics MUST preserve the distinction between:

references;

logical quantum state;

measurement;

reset;

transfer;

entanglement;

classical observation.


Copying a reference or symbolic handle MUST NOT imply cloning an unknown quantum state.


---

36. Quantum Simulation Scalability

A quantum simulator MAY represent quantum systems using:

state vectors;

tensor networks;

stabilizer representations;

decision diagrams;

hybrid representations;

other future representations.


The grammar MUST NOT select a simulator representation.

Simulator limitations MUST NOT become language-level quantum limits.


---

37. Quantum Error Correction Scalability

Quantum error-correction intent MAY be expressed by the language where required.

However:

QEC algorithms belong to the QEC subsystem;

physical code implementation belongs to compilation/hardware layers;

fault descriptions belong to ZQN;

execution adaptation belongs to resilience.


The grammar MUST NOT duplicate QEC algorithms.


---

38. ZQN Integration

ZQN describes:

noise;

faults;

error classes;

correlated faults;

leakage;

loss;

erasure;

fault characteristics.


The scalability model only defines how such information interacts with scalable execution.

ZQN MUST NOT impose a universal finite resource count.


---

39. quantum::ir Boundary

The canonical quantum semantic representation remains:

quantum::ir

The grammar parses quantum syntax.

Semantic lowering maps syntax into the canonical quantum representation.

Optimization, routing, scheduling, ZQN-aware processing, QEC integration, hardware lowering, and runtime systems consume the canonical representation.

The grammar MUST NOT create a second competing quantum IR.


---

40. Hardware Scalability

Hardware descriptions MUST distinguish:

logical hardware intent

from:

physical implementation

Hardware constructs MAY describe:

modules;

interfaces;

ports;

signals;

registers;

memories;

pipelines;

state machines;

timing behavior;

capabilities.


Physical synthesis determines:

placement;

routing;

gates;

cells;

physical resources.



---

41. FPGA Scalability

The language MUST NOT assume a fixed FPGA:

LUT count;

BRAM count;

DSP count;

I/O count;

clock count.


Hardware requirements MAY express capabilities.

Target descriptions provide actual resources.

Compilation determines whether the design can be realized.


---

42. ASIC Scalability

ASIC-specific resource quantities MUST remain target-specific unless explicitly part of a hardware design contract.

The language MUST support parameterized hardware descriptions rather than requiring a fixed implementation size.


---

43. Accelerator Scalability

The language SHOULD allow accelerator-independent semantics.

An algorithm MAY execute on:

CPU;

GPU;

FPGA;

ASIC;

NPU;

TPU-like accelerator;

quantum accelerator;

future accelerator.


The accelerator is selected by capability matching unless explicitly fixed by a target-specific requirement.


---

44. Tensor Scalability

Tensor dimensions SHOULD support symbolic, generic, inferred, or runtime-defined extents where semantically appropriate.

The grammar MUST NOT impose arbitrary tensor dimension ceilings.

If a dimension is semantically fixed, that fixed dimension is part of the program's type/meaning rather than a machine-size limitation.


---

45. Numerical Scalability

Numerical programs MUST distinguish:

mathematical meaning;

representation;

precision;

range;

rounding;

target implementation.


A program MAY require a particular numerical contract.

For example:

exact integer arithmetic

is different from:

use a particular CPU instruction

The latter belongs to target-specific realization unless explicitly expressed.


---

46. Precision Scaling

Targets MAY provide different precision capabilities.

The compiler MUST NOT silently reduce precision if the program requires stronger guarantees.

Permitted behavior includes:

1. preserve required precision;


2. select an equivalent higher-precision representation;


3. reject an incapable target;


4. use an explicitly permitted approximation.



Silent semantic degradation is prohibited.


---

47. Approximation

Approximation MAY be used only when allowed by the semantic contract.

An approximation policy MUST define:

allowed error;

metric;

scope;

observability;

acceptance criteria.


A compiler MUST NOT infer arbitrary tolerance merely because a target is smaller or faster.


---

48. Algorithmic Scaling

The grammar SHOULD provide mechanisms for expressing algorithms independently of physical resource count.

Examples include:

iteration;

recursion;

parameterization;

generic dimensions;

ranges;

data-dependent processing;

parallel regions;

streaming;

distributed partitioning.


The grammar MUST NOT replace these with enumerated fixed-size forms.


---

49. Parameterization

Parameterization is a primary mechanism for scalability.

Parameters MAY represent:

dimensions;

capacities;

algorithm parameters;

data sizes;

logical resources;

compile-time properties;

deployment properties.


A parameter MUST NOT be confused with a physical machine limit.


---

50. Generics

Generic programming MUST support scalable abstractions.

A generic component MAY be instantiated for:

small scale
medium scale
large scale

without modifying its semantic definition.

Generic constraints MUST express semantic requirements rather than accidental implementation limits.


---

51. Compile-Time Scaling

The compiler itself MUST be scalable.

The grammar MUST avoid constructs that inherently require:

fixed-size tables;

fixed-size resource models;

enumerated machine inventories;

bounded-depth semantic structures without justification.


Compiler implementation limits are implementation concerns.

They MUST NOT be represented as language semantics.


---

52. Runtime Scaling

Runtime scalability MUST be based on discovered capabilities and available resources.

The runtime MAY:

allocate resources;

partition workloads;

migrate work;

schedule tasks;

select accelerators;

select quantum backends;

distribute computation;

scale execution.


These decisions MUST NOT redefine program meaning.


---

53. Scheduling Integration

Scheduling belongs to the scheduling subsystem.

The grammar MAY express:

timing requirements;

deadlines;

ordering requirements;

latency constraints;

concurrency requirements;

synchronization requirements.


It MUST NOT implement the scheduling algorithm.

The scheduler may choose:

ASAP;

ALAP;

list scheduling;

resource-aware scheduling;

critical-path scheduling;

RCPSP;

future strategies.



---

54. Routing Integration

Routing belongs to the routing/compilation subsystem.

The grammar MAY express logical connectivity requirements.

It MUST NOT hard-code physical routes.

For quantum computation:

logical connectivity

is distinct from:

physical connectivity

and routing determines the latter.


---

55. Optimization Integration

Optimization belongs to the optimization subsystem.

Optimizations MAY:

fuse operations;

eliminate redundant operations;

reduce resource use;

transform data layouts;

parallelize computation;

reduce quantum depth.


The optimization result MUST preserve the required semantic contract.


---

56. Compilation Context

Compilation MAY receive:

Program
Requirements
Constraints
Preferences
Hints
Target
Capabilities
Available resources
Optimization policy
Portability policy
Security policy

The grammar MUST provide sufficient syntax to express the source-level concepts without embedding target implementation details into core grammar rules.


---

57. Target Independence

A portable source program MUST NOT depend on a target merely because that target happens to be available.

For example:

requires quantum capability

does not mean:

use backend XYZ

Likewise:

requires GPU capability

does not mean:

use GPU #3

unless explicitly requested by a target-specific program contract.


---

58. Target-Specific Programming

Zamani MAY support explicit target-specific programming.

Such programming MUST be distinguishable from portable core semantics.

Target-specific constructs SHOULD be:

namespaced;

versioned;

capability-gated;

explicitly declared;

isolated through dialects or hardware interfaces.


This allows low-level programming without contaminating the universal language model.


---

59. Deployment Independence

Deployment topology MUST NOT be embedded into portable source semantics by default.

A program MAY execute:

locally
remotely
embedded
distributed
cloud-hosted
edge-hosted
quantum-backed
heterogeneously

without rewriting the computation.

Deployment-specific information belongs to:

deployment configuration;

execution context;

target description;

resource manager;

runtime.



---

60. Resource Negotiation

When multiple targets satisfy a program's requirements, the implementation MAY negotiate among them.

Negotiation MAY consider:

performance;

cost;

latency;

energy;

reliability;

availability;

security;

locality;

precision;

quantum fidelity;

accelerator capability.


The choice MUST obey semantic constraints.


---

61. Capability Matching

Conceptually:

Requirements ⊆ Capabilities

must hold for required capabilities.

A target that lacks a mandatory capability MUST NOT be selected.

Preferences MAY be relaxed according to policy.

Hints MAY be ignored when necessary.


---

62. Resource Insufficiency

If no available realization satisfies the required contract, the system MUST NOT silently change the program.

Possible outcomes include:

defer
retry
recover
select another target
request resources
compile differently
report unsupported target
reject execution

The exact decision belongs to compilation/runtime/resilience policy.


---

63. Resilience Integration

Resilience MAY adapt execution to resource or hardware changes.

Possible actions include:

retry;

restart;

resume;

rollback;

remap;

reroute;

reschedule;

recompile;

reoptimize;

change QEC;

mitigate;

switch backend;

quarantine;

abort.


Resilience MUST preserve the semantic contract.

The grammar does not own resilience decisions.


---

64. Recovery and Scalability

Recovery mechanisms MUST distinguish among:

classical execution checkpoints;

compiled artifacts;

logical checkpoints;

measurement boundaries;

QEC-managed state;

provider-supported quantum state;

reconstructible state.


The language MUST NOT imply that arbitrary unknown quantum states can always be serialized and restored.


---

65. Checkpoint Scalability

Checkpoint systems MUST NOT assume a fixed checkpoint size.

Checkpoint representation MAY vary with:

program state;

resource model;

execution model;

quantum representation;

provider support;

distributed state.


A target may reject checkpointing if the required semantic guarantees cannot be provided.


---

66. HDL Scalability

HDL constructs MUST support parameterized hardware designs.

Examples include:

parameterized widths;

generic modules;

scalable memories;

parameterized pipelines;

configurable interfaces;

reusable state machines.


The grammar MUST NOT require a fixed hardware size.

Physical synthesis determines actual implementation.


---

67. HDL Timing

Timing has two distinct categories.

Semantic timing

Timing that is observable or explicitly required by the HDL/program.

Implementation timing

Physical clock period, routing delay, propagation delay, placement delay, and synthesis characteristics.

Only the first belongs to program semantics by default.

Implementation timing belongs to hardware/compiler/toolchain layers.


---

68. Hardware/Software Co-Design

Zamani MUST permit software and hardware components to be described in a coordinated semantic model.

The interface MUST distinguish:

software intent
hardware intent
communication contract
resource requirement
physical realization

The grammar MUST NOT force a single fixed hardware architecture.


---

69. AI/ML Scalability

AI/ML programs MUST support scalable:

datasets;

models;

tensor dimensions;

training workloads;

inference workloads;

accelerator usage;

distributed training;

parallel execution.


The grammar MUST NOT assume a fixed GPU count or fixed tensor size.


---

70. Data Scalability

Data abstractions MUST support:

small datasets;

large datasets;

streaming data;

distributed data;

partitioned data;

persistent data;

future data representations.


The implementation MAY use:

in-memory storage;

external storage;

distributed storage;

streaming systems.


These are realization choices.


---

71. Streaming Scalability

Streams MUST NOT imply a fixed number of elements.

A stream MAY be:

finite;

empty;

large;

effectively unbounded;

externally terminated.


The execution model MUST define termination and cancellation semantics without requiring a fixed element count.


---

72. Recursive and Iterative Scale

Recursive and iterative constructs MUST be semantically general.

Implementations MAY impose practical stack or recursion limits.

Such limits MUST be implementation/runtime limits, not arbitrary grammar restrictions.


---

73. Tail and Deep Execution

The language MUST NOT require a finite maximum nesting depth merely because the implementation currently uses a bounded parser or stack.

Where an implementation has a practical limit, diagnostics SHOULD identify the implementation limitation rather than claiming that the language semantics prohibit the program.


---

74. Distributed Replication Scale

Replication constructs MUST NOT assume a fixed replica count.

Replication requirements SHOULD be parameterized or policy-driven.

Physical replica placement belongs to deployment/runtime systems.


---

75. Fault and Failure Scale

A system MAY encounter arbitrarily many independent or correlated failures subject to available resources.

The scalability model MUST support:

partial failure;

resource loss;

node loss;

accelerator loss;

quantum backend failure;

network partition;

degraded capability;

recovery.


Failure handling belongs to runtime/resilience.


---

76. Security Scalability

Security policies MUST scale with:

users;

identities;

devices;

services;

resources;

nodes;

domains.


The grammar MUST NOT assume a fixed number of:

identities;

keys;

permissions;

principals;

resources.



---

77. Capability Security

Capability-based permissions SHOULD be represented independently from physical resource counts.

A capability may authorize:

use quantum resource
access accelerator
perform network operation
access storage
invoke hardware interface

The presence of a capability does not imply that a specific device exists.


---

78. Interoperability Scalability

Foreign interfaces MUST NOT impose universal fixed counts or sizes unless required by the external ABI.

Supported integrations MAY include:

C;

C++;

Python;

OpenQASM;

Verilog;

system interfaces;

future languages.


Interoperability boundaries MUST define:

ownership;

lifetime;

ABI;

data representation;

errors;

effects;

resource semantics.



---

79. Artifact Scalability

Zamani MUST distinguish:

Semantic artifact

Represents program meaning independently of a specific machine where possible.

Target artifact

Represents a realization for a target class.

Deployment artifact

Represents a concrete deployment.

Binary artifact

Represents executable code for a specific architecture/environment.

These MUST NOT be conflated.


---

80. POCO-REAF and "Compile Once"

POCO-REAF MUST NOT make the technically false assumption that one target-specific binary can execute unchanged on every future machine.

Instead:

> Compile Once means that the portable semantic/compiled representation can be reused for multiple target realizations whenever the relevant compatibility contracts permit it.



A target-specific binary MAY require recompilation.

The source program SHOULD NOT require semantic rewriting merely because recompilation for another target is necessary.


---

81. POCO-REAF Portability Levels

Zamani SHOULD distinguish:

Level 1 — Source portability

The same source program expresses the same semantics across targets.

Level 2 — Semantic artifact portability

The same target-independent semantic artifact can be consumed across targets.

Level 3 — Intermediate artifact portability

A compiled intermediate representation can be reused across compatible targets.

Level 4 — Target artifact portability

A target artifact can execute across compatible environments.

Level 5 — Binary portability

The same binary can execute unchanged across environments supporting its ABI and execution contract.

POCO-REAF primarily guarantees the first two levels and supports higher levels where technically possible.


---

82. Future Hardware

Future hardware MUST be able to participate through:

capabilities;

target descriptions;

dialects;

lowering;

resource models;

interoperability contracts.


The core grammar SHOULD NOT require modification merely because a new hardware category appears.

For example, a future accelerator SHOULD be integrable through existing abstractions when its semantics fit them.


---

83. Extensibility

Scalability and extensibility are coupled.

New computing paradigms MUST be able to introduce:

new capabilities;

new resources;

new effects;

new target classes;

new dialects;

new lowering strategies.


They MUST NOT redefine existing core semantics silently.


---

84. Dialect Scalability

Dialects MUST be:

namespaced;

versioned;

explicitly registered;

capability-aware;

compatibility-aware.


A dialect MUST NOT silently redefine a core Zamani construct.

Vendor-specific resource counts MUST remain vendor/target-specific.


---

85. No Grammar-Level Resource Discovery

The grammar MUST NOT contain logic equivalent to:

if machine_has_8_gpus

or:

if qubit_count <= 32

Resource-dependent decisions belong to:

semantic validation where requirements are checked;

compiler target selection;

runtime;

scheduling;

resource management.



---

86. Compile-Time Conditions

Compile-time conditions MAY depend on explicitly supplied compilation facts.

For example:

target capability
feature
language version
dialect availability

may be used.

However, compile-time target facts MUST NOT be confused with universal language semantics.


---

87. Runtime Conditions

Runtime conditions MAY depend on discovered:

capabilities;

resources;

availability;

health;

deployment state.


Runtime adaptation MUST obey the program's declared semantic contract.


---

88. Scale-Aware Optimization

Optimization MAY select different strategies depending on scale.

For example:

small workload → sequential implementation
large workload → parallel implementation

This is valid when both implementations preserve semantics.

Scale-aware optimization MUST NOT change observable behavior beyond permitted contracts.


---

89. Performance Portability

Performance is NOT automatically semantic.

A program may execute correctly at different:

speeds;

throughputs;

latencies;

resource utilizations.


If performance is semantically important, the program MUST express an explicit requirement or constraint.


---

90. Energy Scalability

Energy may be represented as:

requirement;

constraint;

preference;

optimization objective.


It MUST NOT become an implicit fixed hardware assumption.


---

91. Reliability Scalability

Reliability requirements MAY be expressed independently of machine size.

Examples include:

required availability
required fault tolerance
required error bounds
required execution confidence

The implementation determines how to satisfy them.


---

92. Latency Scalability

Latency requirements MUST distinguish:

semantic deadline

from:

hardware-specific execution latency

The scheduler and target system determine whether the requirement can be met.


---

93. Throughput Scalability

Throughput MAY scale through:

parallelism;

pipelining;

batching;

replication;

vectorization;

accelerators;

distributed execution.


No fixed throughput must be embedded into the grammar.


---

94. Batch Scaling

Batch sizes MAY be:

compile-time parameters;

runtime values;

inferred;

target-selected where semantics permit.


A target SHOULD be able to choose a suitable batch size when batch size is not semantically observable.


---

95. Partitioning

Large workloads MAY be partitioned.

Partitioning may occur across:

threads;

cores;

devices;

nodes;

memory domains;

quantum subprograms where semantically valid.


Partitioning MUST preserve required dependencies and semantics.


---

96. Sharding

Data may be sharded across resources.

Sharding is an implementation strategy unless explicitly observable.

The grammar SHOULD express logical data structures rather than physical shard inventories.


---

97. Placement Independence

Logical objects MUST be distinct from physical placement.

For example:

logical tensor
logical qubit
logical task
logical service

is distinct from:

physical memory
physical GPU
physical qubit
physical CPU
physical node

Placement is performed by downstream systems.


---

98. Topology Independence

Portable semantics MUST NOT assume a fixed topology.

This includes:

CPU topology;

NUMA topology;

GPU topology;

FPGA topology;

quantum connectivity;

network topology;

cluster topology;

memory topology.


Topology becomes semantic only when explicitly declared as part of the program's contract.


---

99. Explicit Topology Semantics

Some programs genuinely depend on topology.

Examples:

network topology algorithms;

hardware design;

topology-aware quantum algorithms;

routing software;

embedded hardware control.


Such topology MAY be represented explicitly.

It MUST be distinguishable from universal portable assumptions.


---

100. Resource Identity

Resource identities MUST be scoped.

Possible scopes include:

logical
module
deployment
target
device
runtime
physical

A physical identifier MUST NOT leak into portable semantics unintentionally.


---

101. Resource Lifetime

Scalable resource models MUST define:

acquisition;

ownership;

borrowing;

sharing;

release;

failure;

cancellation.


These integrate with the memory, concurrency, resource, and execution specifications.


---

102. Resource Ownership

A resource may be:

exclusively owned;

shared;

borrowed;

leased;

pooled;

dynamically assigned.


The grammar MAY express the required ownership semantics.

The runtime implements actual resource ownership.


---

103. Resource Contention

Multiple computations MAY compete for resources.

Resource contention MUST NOT change semantic meaning unless the language explicitly exposes scheduling/resource effects.

The runtime MAY:

queue;

multiplex;

migrate;

reject;

prioritize;

reschedule.



---

104. Backpressure

Streaming and distributed systems SHOULD support explicit backpressure semantics.

Backpressure is distinct from a fixed capacity.

The language MUST NOT encode an arbitrary universal queue size.


---

105. Cancellation

Cancellation MUST be scalable.

It MUST work across:

tasks;

distributed execution;

quantum execution where supported;

hardware execution where supported;

asynchronous operations.


Cancellation semantics MUST be defined independently of the number of workers.


---

106. Synchronization Scaling

Synchronization primitives MUST scale from:

single-thread

through:

multicore

to:

distributed

where their semantics permit.

The grammar defines synchronization intent.

The runtime provides implementation.


---

107. Determinism

Parsing MUST be deterministic.

Semantic analysis MUST be deterministic for identical:

source;

language version;

relevant configuration;

dialect versions.


Execution may be:

deterministic;

probabilistic;

nondeterministic;


according to the program's semantics.

Scale MUST NOT introduce accidental semantic nondeterminism.


---

108. Reproducibility

For reproducible execution, the system SHOULD preserve:

source identity;

language version;

dialect versions;

semantic artifact identity;

relevant compiler configuration;

relevant target capabilities;

random seeds where applicable;

numerical modes;

execution policy.


Machine size itself MUST NOT become an implicit semantic variable unless declared.


---

109. Quantum Reproducibility

Quantum measurement outcomes MAY differ between executions according to quantum probability.

Reproducibility therefore means preserving:

circuit semantics;

input state contract;

measurement model;

randomization policy;

execution configuration;


rather than requiring identical physical measurement samples in all circumstances.


---

110. Numerical Reproducibility

Numerical reproducibility MAY require explicit:

precision;

rounding;

numerical model;

deterministic reduction;

ordering.


Parallel execution MUST NOT silently change semantics where exact numerical reproducibility is required.


---

111. Observability

Only explicitly observable properties may become semantic dependencies.

Examples:

program output
measurement result
declared timing
declared resource usage
declared ordering

Internal:

scheduler choice
physical route
temporary buffer
device assignment

is not semantic by default.


---

112. Scaling and Observability

A program MUST NOT accidentally become non-portable because an implementation detail is observable through an undocumented side channel.

Toolchains SHOULD clearly distinguish:

program-visible observations;

diagnostic observations;

telemetry;

implementation metrics.



---

113. Resource Metrics

Runtime telemetry MAY record:

memory usage;

CPU usage;

GPU usage;

qubit usage;

node usage;

latency;

queue time;

energy;

failures;

retries.


Telemetry MUST NOT redefine program semantics.


---

114. Scalability and Diagnostics

When a program cannot execute because of resource limits, diagnostics SHOULD identify:

1. required capability/resource;


2. available capability/resource;


3. violated requirement;


4. relevant policy;


5. possible compatible targets.



The diagnostic MUST NOT incorrectly report:

language does not support more than N

when the actual issue is target capacity.


---

115. Error Classification

Scalability-related errors SHOULD distinguish:

SemanticError
RequirementUnsatisfied
CapabilityMissing
ResourceUnavailable
ResourceExhausted
TargetUnsupported
RepresentationLimit
CompilerLimit
RuntimeLimit
DeploymentLimit
PhysicalLimit
PolicyRejected

These categories MUST remain distinguishable.


---

116. Implementation Limits

Implementation limits MAY exist.

Examples:

parser memory;

compiler memory;

integer width;

internal table size;

runtime queue size;

backend API limits.


They MUST NOT be mistaken for language-level semantic limits.

Where possible, implementations SHOULD detect and report them explicitly.


---

117. Representation Limits

A representation may have finite bounds.

For example:

an integer representation;

an index representation;

an address representation.


Such bounds are part of the representation/type contract, not arbitrary machine scalability.

If a larger representation is needed, the language SHOULD provide an extensible mechanism.


---

118. Physical Limits

Physical systems impose real limits.

Examples include:

finite memory;

finite energy;

finite coherence;

finite communication bandwidth;

finite fabrication resources;

finite storage;

finite execution time.


Zamani does not claim to eliminate these.

Instead, Zamani ensures that such limitations are external to the universal semantic model.


---

119. Scalability and Physical Reality

The correct model is:

Infinite language scalability domain
            +
finite available realization
            =
best valid execution possible

not:

infinite physical hardware assumption


---

120. Tiny-to-Large Execution

The same semantic program SHOULD be capable of realization on:

tiny embedded target
        ↓
single processor
        ↓
multicore system
        ↓
accelerator system
        ↓
quantum system
        ↓
heterogeneous system
        ↓
cluster
        ↓
supercomputer
        ↓
distributed infrastructure
        ↓
future architecture

provided the target satisfies the required semantic contract.


---

121. Atom-to-Everywhere Principle

"From Atom to Everywhere" means:

the language can describe extremely small computations;

the same abstraction model can scale upward;

resource counts are not baked into syntax;

larger execution environments can provide larger realizations;

future machines can participate through capability-based integration.


The language MUST NOT require separate fundamental semantics for every scale.


---

122. Scale-Independent Syntax

Core syntax SHOULD be structurally scale-independent.

For example, the grammar should represent:

collection

rather than:

collection_of_32

and:

parallel computation

rather than:

parallel computation using exactly 8 cores

unless the latter is explicitly semantic.


---

123. Scale-Independent AST

AST nodes MUST represent logical concepts rather than physical inventories.

An AST node MAY represent:

ResourceRequirement
CapabilityRequirement
ParallelRegion
QuantumRegister
LogicalQubit
HardwareModule
DistributedService

but MUST NOT encode target-specific realization merely because it was parsed from portable source.


---

124. Scale-Independent IR

Canonical IRs MUST represent semantic intent.

For quantum computation:

Zamani syntax
      ↓
semantic analysis
      ↓
quantum::ir
      ↓
optimization
      ↓
routing
      ↓
scheduling
      ↓
hardware lowering
      ↓
execution

The IR MUST NOT become a disguised fixed-machine model.


---

125. Compiler Scaling

Compiler passes SHOULD operate on abstractions that permit:

streaming;

incremental compilation;

partitioned compilation;

parallel compilation;

lazy analysis;

caching;

reusable artifacts.


These are implementation strategies.

They MUST preserve semantics.


---

126. Incremental Compilation

Large projects SHOULD support incremental compilation.

The grammar MUST provide stable structural boundaries for:

modules;

declarations;

imports;

interfaces;

compilation units.


An implementation MAY recompile only affected components.


---

127. Distributed Compilation

Future implementations MAY distribute compilation across resources.

The language semantics MUST remain independent of whether compilation happens:

locally;

remotely;

in parallel;

incrementally;

in a cloud environment.



---

128. Runtime Scaling Policies

Runtime policies MAY include:

resource-first
performance-first
latency-first
energy-first
cost-aware
reliability-first
portability-first
deterministic

Policies MUST NOT silently violate hard semantic requirements.


---

129. Preferences and Scaling

Preferences MAY guide scaling.

For example:

prefer low latency
prefer low energy
prefer local execution
prefer accelerator
prefer distributed execution

Preferences MAY be relaxed according to policy.

Requirements MUST NOT be relaxed unless the program explicitly permits degradation.


---

130. Degraded Execution

A program MAY explicitly permit degraded execution.

A degradation contract MUST specify:

what may change;

permitted bounds;

how correctness is evaluated;

whether output is acceptable;

whether the result is marked degraded.


Without such permission, degraded execution MUST NOT silently replace required semantics.


---

131. Semantic Preservation

Any scale transformation:

small → large
large → small
single → parallel
local → distributed
logical → physical
generic → specialized

MUST preserve the program's semantic contract.

Formally:

Observable(Original)
≈
Observable(Transformed)

where ≈ is the equivalence relation defined by the program's semantic contract.

For exact semantics:

Observable(Original)
=
Observable(Transformed)

For explicitly approximate semantics, the permitted error contract applies.


---

132. Optimization Preservation

Optimization MUST preserve semantics.

An optimization MUST NOT be justified merely because:

target is small
target is large
target has fewer resources
target has more resources

Any behavior change must be permitted by the semantic contract.


---

133. Routing Preservation

Routing may change:

physical qubit placement;

inserted movement operations;

physical paths;

execution duration.


It MUST preserve the logical quantum computation under the defined execution model.


---

134. Scheduling Preservation

Scheduling may change:

operation start times;

parallel execution;

resource allocation;

ordering where dependencies permit.


It MUST preserve all semantically observable ordering and timing requirements.


---

135. Hardware Lowering Preservation

Hardware lowering may change:

instruction representation;

gate decomposition;

memory layout;

accelerator invocation;

physical resource assignment.


It MUST preserve the declared semantic contract.


---

136. Scale and Security

Scaling MUST NOT bypass security policies.

Adding resources MUST NOT automatically grant:

new privileges;

broader network access;

unrestricted hardware access;

secret access.


Capabilities remain explicit.


---

137. Scale and Isolation

Distributed or heterogeneous scaling MUST preserve declared isolation boundaries.

A workload MAY move between resources only if the destination satisfies:

security requirements;

capability requirements;

data requirements;

semantic requirements.



---

138. Scale and Provenance

Scaling decisions SHOULD be recorded in provenance where useful.

Provenance MAY include:

source identity;

semantic artifact identity;

compilation identity;

target class;

capability snapshot;

resource realization;

policy;

execution identity.


Secrets MUST NOT be embedded merely for provenance.


---

139. Versioned Scalability Contracts

Scalability semantics MUST be versioned independently from:

grammar version;

language version;

IR version;

runtime version;

target version.


A change to scalability semantics MUST be compatibility-reviewed.


---

140. Backward Compatibility

A newer implementation SHOULD continue accepting older valid programs unless a documented language evolution rule prevents it.

Changes to:

resource semantics;

requirement semantics;

capability semantics;

target semantics;


MUST follow the compatibility specification.


---

141. Forward Compatibility

The grammar SHOULD reserve extensibility points for:

new resource kinds;

new capabilities;

new target classes;

new execution environments;

new hardware paradigms;

new quantum technologies.


Unknown future target capabilities MUST NOT require rewriting the core scalability model.


---

142. Reserved Resource Namespace

Resource categories SHOULD support namespaced extension.

Conceptually:

core.resource
domain.resource
vendor.resource
experimental.resource

The exact syntax is governed by the core grammar and dialect specifications.


---

143. No Universal Machine Model

Zamani MUST NOT define one universal machine containing a fixed set of:

CPUs;

GPUs;

FPGAs;

ASICs;

qubits;

memories;

nodes.


Instead, Zamani defines a universal semantic model that can be realized by many machine models.


---

144. Heterogeneous Scalability

A program MAY combine:

CPU
GPU
FPGA
ASIC
NPU
quantum processor
distributed services

without making any one resource category the universal execution model.


---

145. Hybrid Scaling

Hybrid programs MUST preserve explicit boundaries between:

classical state;

quantum state;

hardware state;

distributed state.


Scaling one domain MUST NOT implicitly duplicate or alter another domain's state.


---

146. Quantum-Classical Scaling

A hybrid quantum-classical program may scale by:

increasing logical qubits;

increasing classical compute;

increasing measurement shots;

distributing classical post-processing;

changing quantum backend;

changing simulation method.


Each transformation MUST preserve the declared hybrid semantic contract.


---

147. Measurement-Shot Scaling

Repeated quantum execution MAY be scaled through additional shots.

The language MUST distinguish:

one quantum execution

from:

repeated sampling

where this distinction is semantically relevant.

The number of shots MUST NOT be silently introduced as an arbitrary physical workaround.


---

148. Resource-Aware Quantum Execution

Quantum execution may depend on:

available logical qubits;

physical qubits;

connectivity;

gate set;

coherence;

error rates;

calibration.


These are target capabilities/resources.

They MUST NOT become universal grammar limits.


---

149. Future Quantum Architectures

The quantum scalability model MUST accommodate future technologies such as:

different qubit modalities;

new connectivity models;

new native operations;

fault-tolerant systems;

modular quantum systems;

networked quantum systems;

future quantum computation models.


The grammar SHOULD describe semantic quantum computation rather than a particular present-day device.


---

150. Distributed Quantum Scaling

Future distributed quantum systems MAY involve:

multiple quantum processors;

quantum links;

entanglement distribution;

classical coordination;

heterogeneous quantum resources.


These MUST be represented through extensible resource/capability models rather than hard-coded topology.


---

151. HDL/Quantum Co-Design

Quantum hardware descriptions MAY combine:

HDL semantics
+
quantum semantics
+
classical control

without forcing physical resource counts into the universal grammar.

Physical realization belongs to hardware compilation.


---

152. Testing Scalability

Every scalable grammar feature MUST have tests demonstrating:

1. minimal valid scale;


2. representative scale;


3. large structural scale;


4. parameterized scale;


5. absence of artificial fixed limits.



Tests SHOULD avoid defining a "maximum supported size" as if it were a language semantic boundary.


---

153. Boundary Testing

Boundary tests MUST include cases such as:

zero-element collections where legal
one-element collections
small collections
large generated collections
empty modules where legal
large module graphs
large expressions
deep nesting
large quantum registers
large logical resource sets
large distributed resource descriptions

The test harness itself MAY have practical limits, but those limits MUST be documented as test infrastructure limits.


---

154. Scalability Test Generation

Where practical, tests SHOULD be generated rather than manually enumerated.

For example:

N = 1
N = 2
N = ...
N = generated large values

The test system MUST NOT imply that the final tested value is the language maximum.


---

155. Property-Based Testing

Scalable grammar structures SHOULD use property-based testing for:

arbitrary collection sizes;

arbitrary declaration counts;

arbitrary resource descriptions;

arbitrary parameterized dimensions;

nested structures;

large valid programs.


Properties MUST focus on semantic invariants rather than one fixed size.


---

156. Fuzz Testing

Grammar infrastructure SHOULD be fuzz-tested for:

parser robustness;

stack exhaustion;

malformed nesting;

huge identifiers;

large literals;

large collections;

large source files;

adversarial resource expressions.


Fuzz failures MUST be classified as:

language bug;

parser bug;

implementation limit;

resource exhaustion;

infrastructure failure.



---

157. Negative Scalability Tests

Tests MUST reject:

malformed resource expressions;

invalid requirements;

impossible semantic constraints;

invalid target-specific constructs;

illegal capability references;

unsupported resource kinds where appropriate.


The tests MUST distinguish syntax errors from target/resource failures.


---

158. Hard-Coding Audit

The grammar SHALL undergo a formal hard-coding audit.

Search for:

MAX_
MIN_
COUNT_
LIMIT_
QUANTUM_COUNT
QUBIT_COUNT
CPU_COUNT
GPU_COUNT
FPGA_COUNT
NODE_COUNT
THREAD_COUNT
MEMORY_SIZE
REGISTER_COUNT
DEVICE_COUNT
TOPOLOGY_SIZE

and equivalent constructs.

Every occurrence MUST be classified.


---

159. Hard-Coding Classification

Every discovered limit MUST be classified as one of:

1. semantic requirement;


2. type/representation requirement;


3. protocol requirement;


4. security requirement;


5. target-specific requirement;


6. resource constraint;


7. implementation limit;


8. runtime policy;


9. test-only limit;


10. documentation example;


11. accidental hard-coding.



Accidental hard-coding MUST be removed.


---

160. Acceptable Constants

Not every constant is forbidden.

Constants MAY exist when they are:

language syntax;

mathematical definitions;

protocol-defined;

security-defined;

representation-defined;

algorithmically required;

version identifiers;

test fixtures;

examples.


The audit MUST determine why each constant exists.


---

161. Unacceptable Constants

The following MUST NOT be universal language limits merely for convenience:

32 qubits
64 qubits
1024 threads
8 GPUs
16 nodes
1 TB memory
256 registers
32-bit machine
64-bit machine
fixed topology
fixed accelerator count


---

162. Hidden Hard-Coding

Hard-coding is prohibited even when disguised through:

parser alternatives;

finite enums;

generated code;

fixed arrays;

lookup tables;

validator branches;

token sets;

test assumptions;

documentation claims;

serialization schemas.


The audit MUST include generated artifacts and supporting tooling.


---

163. Grammar Structure and Scalability

Grammar rules SHOULD use:

recursion;

repetition;

parameterization;

generic identifiers;

extensible names;

namespaced constructs.


They SHOULD NOT enumerate scalable resources.


---

164. ANTLR Integration

ANTLR grammar design MUST avoid accidental finite limits.

ANTLR-specific implementation details MUST remain implementation concerns.

The grammar MUST remain structurally capable of parsing arbitrarily large valid structures subject to implementation resources.

ANTLR parser limits MUST NOT become documented language semantics.


---

165. Lexer Scalability

The lexer MUST support scalable:

identifier lengths;

literal lengths;

source sizes;

Unicode text;

annotations;

numeric representations;


subject to implementation representation and memory.

Fixed token-length restrictions MUST be justified if present.


---

166. Parser Scalability

The parser MUST support arbitrary valid nesting to the extent permitted by implementation resources.

Implementations SHOULD avoid unnecessary recursion-induced stack limits where practical.

If a parser implementation uses an iterative strategy, this is an implementation optimization and does not alter language semantics.


---

167. AST Scalability

AST collections MUST be dynamically sized.

The AST MUST NOT use fixed universal arrays for:

declarations;

parameters;

statements;

qubits;

resources;

modules.


Rust implementations SHOULD use safe dynamically sized structures such as:

Vec<T>
Box<T>
Option<T>

and other safe standard-library or approved abstractions as appropriate.


---

168. Rust Safety Requirement

All grammar infrastructure MUST be implementable using:

Rust 1.97;

or Rust 1.97.1.


The implementation MUST NOT use:

unsafe

or depend on unsafe custom implementation code.

Safe abstractions MUST be preferred.


---

169. Integer and Index Semantics

Indices and sizes MUST be represented using types appropriate to their semantic domain.

A parser index MUST NOT be confused with a physical resource count.

Conversions between:

semantic size
implementation index
physical resource count

MUST be explicit.


---

170. Overflow

Scalable counts MUST define overflow behavior.

An implementation MUST NOT silently wrap a semantically meaningful resource count if doing so could change program meaning.

Overflow SHOULD result in a diagnostic or a representation capable of representing the value.


---

171. Large Resource Expressions

Resource expressions SHOULD support values larger than common small-machine sizes where their semantics permit.

The grammar MUST NOT arbitrarily restrict values to values chosen merely because they fit a current implementation.


---

172. Resource Expressions and Physical Availability

A program may express:

requires capacity >= N

but N is a program requirement.

It is not a universal machine limit.

The runtime compares the requirement with actual capabilities/resources.


---

173. Generic Resource Requirements

Resource requirements SHOULD support symbolic values where useful.

For example, conceptually:

required_capacity = problem_size

rather than:

required_capacity = 32

when the required capacity naturally scales with program input.


---

174. Input-Dependent Scaling

Programs MAY require resources dependent on runtime input.

The compiler/runtime MAY:

defer resource decisions;

estimate resources;

allocate dynamically;

reject impossible execution.


The grammar MUST support expressing the requirement without assuming a fixed machine.


---

175. Data-Dependent Parallelism

Parallelism MAY depend on data size.

The language SHOULD allow:

parallel over collection

rather than:

spawn exactly 16 workers

when the latter is not semantically required.


---

176. Work-Conserving Execution

When semantics permit, runtimes SHOULD be free to use available resources efficiently.

This may include:

additional workers;

vectorization;

batching;

parallel execution;

accelerator use.


The language does not require a particular strategy.


---

177. Resource-Aware Semantics

Some programs genuinely depend on resources.

For example:

real-time control;

embedded programming;

hardware interfacing;

capacity planning;

topology algorithms.


In such cases, resource requirements become explicit program contracts.

This does not violate scalability because the requirement is semantic rather than an arbitrary global limit.


---

178. Exact Hardware Binding

Explicit hardware binding MAY be supported.

It MUST be clearly distinguishable from portable source.

Example conceptual categories:

portable requirement
target preference
target constraint
explicit physical binding

Only the last intentionally sacrifices portability.


---

179. Portability Trade-Off

A program MAY deliberately choose less portability for:

performance;

hardware control;

deterministic placement;

embedded programming;

physical experiments;

device-specific optimization.


Such trade-offs MUST be explicit.


---

180. Default Portability

Core Zamani constructs SHOULD default toward portability.

Machine-specific behavior SHOULD require an explicit mechanism.

This protects POCO-REAF.


---

181. Scalability of Effects

Effects MUST NOT imply fixed resource counts.

For example:

network effect

does not imply a fixed number of network interfaces.

Likewise:

quantum effect

does not imply a fixed number of qubits.


---

182. Effect Capability Matching

An effect may require a capability.

Conceptually:

EffectRequirement
    ↓
CapabilityCheck
    ↓
ResourceRealization

The grammar describes the requirement.

The execution environment supplies the capability.


---

183. Capability Evolution

Capabilities MUST be extensible.

A future capability MUST NOT require modifying every existing resource grammar rule merely because it is a new capability.

Capability names SHOULD support namespacing and versioning.


---

184. Scalability and Macros

Macros MUST NOT expand scalable concepts into fixed resource enumerations unless explicitly requested.

Macro systems SHOULD support parameterized expansion.

Macro expansion MUST preserve the same scalability contract as ordinary source.


---

185. Scalability and Metaprogramming

Metaprogramming MAY generate:

dimensions;

resources;

modules;

functions;

hardware structures;

quantum operations.


Generated programs remain subject to the same scalability rules.

A metaprogram MUST NOT circumvent scalability constraints by silently generating fixed limits.


---

186. Reflection

Reflection MAY inspect:

types;

capabilities;

semantic metadata;

available interfaces.


Runtime reflection MUST distinguish:

semantic program information

from:

current physical resource inventory


---

187. Compile-Time Reflection

Compile-time reflection MAY inspect target information when explicitly provided through the compilation context.

Such information MUST NOT become permanent source semantics unless explicitly captured as a requirement or constraint.


---

188. Runtime Reflection

Runtime reflection MAY observe dynamic resources.

Such observation is an execution effect.

It MUST NOT silently alter semantic guarantees.


---

189. Scale-Aware Dispatch

Dispatch systems MAY choose different implementations according to scale.

For example:

small → local
large → distributed

or:

small quantum workload → simulator
large quantum workload → hardware

provided the semantic contract permits the substitution.


---

190. Simulator-to-Hardware Portability

Quantum programs SHOULD be expressible independently of whether execution occurs on:

simulator;

emulator;

quantum hardware.


Differences in supported capabilities MUST be exposed through capability checking rather than hidden syntax limits.


---

191. Embedded Scalability

The same language architecture MUST support small embedded systems.

This does not require every feature to execute on every tiny target.

Instead:

program semantics
+
capability requirements

determine which realization is valid.

An incapable target MUST fail cleanly.


---

192. Supercomputer Scalability

The same language model MUST support large-scale HPC.

It SHOULD allow:

massive parallelism;

distributed memory;

accelerators;

vectorization;

data partitioning;

collective communication.


No fixed machine size should be embedded.


---

193. Cloud Scalability

Cloud execution MAY dynamically provision resources.

Provisioning belongs to deployment/runtime infrastructure.

Source code MUST NOT need to contain cloud-specific machine counts unless explicitly targeting such infrastructure.


---

194. Edge Scalability

Edge execution MAY use constrained resources.

The compiler/runtime MAY adapt through:

lower memory usage;

batching;

reduced parallelism;

alternate implementations.


Only permitted transformations may be applied.


---

195. Cross-Domain Scaling

A universal program may combine:

classical
+
quantum
+
hardware
+
distributed
+
AI
+
data
+
networking

Each subsystem MUST preserve the global scalability contract.

No subsystem may introduce a hidden fixed machine model.


---

196. Dependency Boundary

The scalability model depends conceptually on:

language-principles.md
language-scope.md
language-version.md
compatibility.md
grammar-authority.md
syntax-model.md
semantic-model.md
compilation-model.md
execution-model.md
poco-reaf.md
extensibility.md
reserved-space.md

It does not redefine those specifications.


---

197. Integration With semantic-model.md

semantic-model.md defines program meaning.

This document defines how that meaning remains stable across different scales.

The relationship is:

semantic-model
      ↓
scalability-model

The scalability model MUST NOT redefine core language meaning.


---

198. Integration With compilation-model.md

compilation-model.md defines compilation stages.

This specification requires compilation to preserve scale-independent semantics.

Compilation MAY specialize for a target.

Specialization MUST NOT silently rewrite semantic requirements.


---

199. Integration With execution-model.md

execution-model.md defines execution.

This document defines how execution can scale across different resource quantities and environments.

Execution consumes:

program semantics
+
capabilities
+
resources
+
policies


---

200. Integration With poco-reaf.md

poco-reaf.md defines the complete portability objective.

This document provides the scalability foundation required for:

Program Once
Compile Once
Run Everywhere
Run Anywhere
Run Forever


---

201. Integration With grammar-authority.md

grammar-authority.md determines which syntax representation is authoritative.

This specification determines what scalable semantics the authoritative grammar MUST be capable of expressing.


---

202. Integration With syntax-model.md

syntax-model.md defines syntax.

This specification constrains syntax so that scalable semantic concepts are not accidentally represented as fixed machine structures.


---

203. Integration With language-version.md

Changes to scalability semantics MUST be versioned according to language-version policy.

A new hardware capability should not automatically require a new language version if it can be represented through existing extensibility mechanisms.


---

204. Integration With compatibility.md

Compatibility MUST distinguish:

source compatibility
semantic compatibility
artifact compatibility
runtime compatibility
target compatibility

A target-specific incompatibility must not automatically imply source-language incompatibility.


---

205. Integration With resources/

The resource grammar owns syntax for:

resources;

requirements;

capabilities;

constraints;

preferences;

performance;

latency;

energy;

reliability;

scalability;

portability.


This document defines their semantic scalability relationships.


---

206. Integration With hardware/

The hardware grammar describes hardware concepts.

The scalability model ensures hardware descriptions remain parameterizable and distinguish logical design from physical realization.


---

207. Integration With quantum/

The quantum grammar describes quantum syntax.

The scalability model requires quantum constructs to remain independent of fixed machine size.


---

208. Integration With classical/

Classical grammar constructs must support scale-independent computation.

Fixed processor counts and fixed machine models are prohibited.


---

209. Integration With distributed/

Distributed constructs must represent logical distributed computation rather than fixed deployment topology.


---

210. Integration With ai/

AI grammar must support scalable data/model/tensor abstractions.

No fixed accelerator inventory may be embedded.


---

211. Integration With data/

Data grammar must support parameterized and dynamically sized data structures.


---

212. Integration With networking/

Networking syntax must distinguish logical endpoints and physical deployment.


---

213. Integration With security/

Security capabilities and permissions must scale without fixed identity/resource counts.


---

214. Integration With compile/

Compile grammar MAY express target and optimization intent.

It MUST NOT convert implementation constraints into universal semantic limits.


---

215. Integration With execution/

Execution grammar MAY express:

execution contexts;

deployment;

scheduling intent;

placement intent;

dispatch intent.


It MUST NOT hard-code a fixed execution topology.


---

216. Integration With Optimization

Optimization MAY use scale information.

It MUST preserve semantic equivalence.


---

217. Integration With Scheduling

Scheduling MAY use resource availability and scale.

It MUST NOT become a grammar-level resource limit.


---

218. Integration With Routing

Routing MAY use topology.

Topology is target realization data unless explicitly semantic.


---

219. Integration With QEC

QEC MAY adapt according to available resources and required fault tolerance.

The grammar does not implement QEC.


---

220. Integration With ZQN

ZQN may describe scale-dependent fault behavior.

The grammar only expresses relevant declarations/intents.


---

221. Integration With Hardware HAL

The hardware abstraction layer supplies actual:

capabilities;

resources;

topology;

calibration;

state.


The scalability model requires that information to remain outside universal grammar semantics.


---

222. Integration With Resource Management

Resource management maps abstract requirements to concrete resources.

This is a downstream dependency.

The grammar MUST NOT query resource managers directly.


---

223. Integration With Runtime

Runtime realizes the compiled semantic representation.

Runtime resource variation MUST NOT alter program meaning unless permitted by the execution contract.


---

224. Integration With Resilience

Resilience may react to:

resource exhaustion;

hardware changes;

degraded capabilities;

backend failure.


It MUST preserve the semantic contract or explicitly report failure/degradation.


---

225. Integration Graph

The intended architecture is:

┌──────────────────────┐
                         │ Zamani Source        │
                         └──────────┬───────────┘
                                    │
                                    ▼
                         ┌──────────────────────┐
                         │ Grammar / Parser     │
                         └──────────┬───────────┘
                                    │
                                    ▼
                         ┌──────────────────────┐
                         │ AST / Semantic Model │
                         └──────────┬───────────┘
                                    │
                    ┌───────────────┴────────────────┐
                    ▼                                ▼
          Requirements / Constraints          Semantic IR
                    │                                │
                    └───────────────┬────────────────┘
                                    ▼
                         ┌──────────────────────┐
                         │ Compilation          │
                         └──────────┬───────────┘
                                    │
                                    ▼
                         ┌──────────────────────┐
                         │ Target Capabilities  │
                         └──────────┬───────────┘
                                    │
                                    ▼
                         ┌──────────────────────┐
                         │ Resource Management  │
                         └──────────┬───────────┘
                                    │
                    ┌───────────────┼────────────────┐
                    ▼               ▼                ▼
                Routing         Scheduling       Optimization
                    │               │                │
                    └───────────────┼────────────────┘
                                    ▼
                         ┌──────────────────────┐
                         │ Hardware / Runtime   │
                         └──────────┬───────────┘
                                    │
                                    ▼
                         ┌──────────────────────┐
                         │ Execution            │
                         └──────────────────────┘

No cycle may be introduced from:

runtime → grammar
IR → grammar
hardware → grammar


---

226. Dependency Graph

The specification dependency is:

language-principles.md
        │
        ├── language-scope.md
        │
        ├── language-version.md
        │
        ├── grammar-authority.md
        │
        └── syntax-model.md
                │
                ▼
        semantic-model.md
                │
                ├── compilation-model.md
                │
                ├── execution-model.md
                │
                └── scalability-model.md
                         │
                         └── poco-reaf.md

scalability-model.md MUST NOT require implementation of a particular runtime or hardware backend to define its semantics.


---

227. Independent Completion Contract

This file MUST be independently completable.

File

grammar/specification/scalability-model.md

Purpose

Define normative scalability semantics for Zamani.

Owns

scale independence;

resource scalability;

scalability invariants;

scalability portability;

hard-coding rules;

resource abstraction relationships.


Does Not Own

syntax;

AST implementation;

IR implementation;

optimization;

routing;

scheduling;

hardware discovery;

runtime implementation.


Inputs

Conceptually:

semantic model;

compilation model;

execution model;

resource abstractions;

target/capability concepts.


Outputs

Normative scalability requirements consumed by:

grammar;

semantic analysis;

compiler;

runtime;

resource manager;

target integrations;

tests.


Dependencies

Normative specification dependencies:

language-principles.md
language-scope.md
semantic-model.md
compilation-model.md
execution-model.md
poco-reaf.md

No implementation dependency is required to define this document.

Upstream Contracts

The semantic model defines what a program means.

The compilation model defines transformation stages.

The execution model defines realization.

Downstream Consumers

lexer/parser design;

core grammar;

type system;

resource grammar;

classical grammar;

quantum grammar;

hardware grammar;

distributed grammar;

compiler;

runtime;

tests;

documentation.


Public Grammar Contract

The grammar MUST:

avoid arbitrary finite machine limits;

represent scalable collections generically;

support requirements/capabilities/constraints/preferences;

permit future resource categories;

distinguish logical from physical resources.


AST Contract

AST structures MUST represent:

logical resource intent;

requirements;

capabilities;

constraints;

preferences;

scalable collections.


AST structures MUST NOT silently encode target-specific resource realization.

Semantic Contract

Semantic analysis MUST distinguish:

semantic requirement
resource requirement
capability
constraint
preference
hint
target realization

IR Integration

Canonical IRs consume semantic results.

The scalability model MUST NOT define a competing IR.

quantum::ir remains the canonical quantum semantic boundary.

Compiler Integration

The compiler MAY specialize based on:

target;

scale;

capabilities;

resources;

policies.


Specialization MUST preserve semantic guarantees.

Runtime Integration

Runtime MAY dynamically adapt resource allocation.

It MUST NOT silently change required semantics.

Tooling Integration

Tooling MUST expose scalability diagnostics and hard-coding audits.

Cross-Domain Integration

All computing domains MUST follow the same core scalability model.

Tests

Positive, negative, boundary, scalability, cross-domain, compatibility, and determinism tests are required.

Negative Tests

Must include:

impossible requirements;

invalid capability references;

invalid target binding;

unsupported scaling semantics;

resource insufficiency.


Boundary Tests

Must cover:

minimal scale;

large scale;

parameterized scale;

generated large structures.


Compatibility Requirements

Changes MUST comply with compatibility.md.

Scalability Requirements

No arbitrary machine-size maximum may be introduced.

Hard-Coding Audit

Every fixed resource quantity MUST be justified.

Completion Criteria

This file is complete only when:

all scalable resource categories have been covered;

all domain integrations are defined;

hard-coding rules are explicit;

POCO-REAF is reconciled with realistic compilation;

physical limitations are distinguished from language limits;

quantum/classical/hardware/distributed scaling is defined;

downstream contracts are complete;

tests are specified.



---

228. Required Grammar Invariants

Every production grammar component MUST satisfy:

Invariant S1:
No arbitrary scalable resource maximum.

Invariant S2:
Program meaning is independent of target resource quantity.

Invariant S3:
Requirements are distinct from capabilities.

Invariant S4:
Capabilities are distinct from resources.

Invariant S5:
Constraints are distinct from preferences.

Invariant S6:
Logical resources are distinct from physical resources.

Invariant S7:
Physical placement is not implicit portable semantics.

Invariant S8:
Target discovery is not grammar responsibility.

Invariant S9:
Quantum physical topology is not universal grammar semantics.

Invariant S10:
No hidden resource-count assumptions.

Invariant S11:
Scale transformations preserve semantic contracts.

Invariant S12:
Implementation limits are not silently presented as language limits.

Invariant S13:
Future resource types remain extensible.

Invariant S14:
No grammar/runtime dependency cycle.

Invariant S15:
No grammar/IR dependency cycle.

Invariant S16:
No grammar/hardware discovery dependency cycle.


---

229. File-Level Hard-Coding Review

Before any grammar file is marked complete, reviewers MUST ask:

1. Does this file contain a fixed resource count?


2. Does it assume a fixed machine?


3. Does it enumerate physical devices?


4. Does it assume a fixed topology?


5. Does it impose an arbitrary maximum?


6. Does it confuse an implementation limit with semantics?


7. Does it force target-specific syntax into portable source?


8. Does it duplicate a downstream resource model?


9. Does it prevent future resource categories?


10. Does it require another grammar file to know today's machine topology?



A "yes" MUST trigger review.


---

230. Completion Gate for Each Grammar File

A grammar file MUST NOT be considered complete until:

Syntax defined
        ↓
Semantic meaning defined
        ↓
Scalability reviewed
        ↓
Resource assumptions reviewed
        ↓
Hard-coding audit passed
        ↓
AST contract defined
        ↓
IR boundary defined
        ↓
Compiler integration defined
        ↓
Runtime integration defined
        ↓
Cross-domain integration defined
        ↓
Tests complete
        ↓
Compatibility reviewed


---

231. Example: Classical Program

Conceptually:

logical computation
        ↓
problem-size-dependent workload
        ↓
compiler
        ↓
available CPU/GPU/accelerator resources
        ↓
execution

The same semantic program may be realized sequentially or in parallel.


---

232. Example: Quantum Program

Conceptually:

logical quantum program
        ↓
quantum::ir
        ↓
optimization
        ↓
routing
        ↓
scheduling
        ↓
target capabilities
        ↓
physical realization

The program MUST NOT require rewriting merely because the physical machine has a different qubit count or topology, provided the target satisfies its requirements.


---

233. Example: Hardware Program

Conceptually:

parameterized hardware behavior
        ↓
hardware compilation
        ↓
target FPGA/ASIC resources
        ↓
synthesis/place/route
        ↓
physical implementation

The source design remains parameterized where its semantics permit.


---

234. Example: Distributed Program

Conceptually:

logical distributed computation
        ↓
requirements
        ↓
deployment
        ↓
available nodes
        ↓
placement
        ↓
communication
        ↓
execution

The source does not need to enumerate every physical node.


---

235. Example: Heterogeneous Program

Conceptually:

one semantic program
        ↓
CPU work
GPU work
FPGA work
quantum work
distributed work
        ↓
capability/resource matching
        ↓
execution

Each component retains its semantic boundary.


---

236. Example: Tiny Target

A target may expose:

limited memory
single processor
no accelerator
no quantum capability

A program requiring only the available capabilities MAY execute.

A program requiring unavailable capabilities MUST fail or be transformed only where the semantic contract permits.

The target's small size does not redefine the language.


---

237. Example: Large Target

A larger target MAY expose:

many processors
many accelerators
large memory
distributed resources
quantum resources

The runtime/compiler MAY exploit them.

The source semantics remain unchanged.


---

238. Example: Future Target

A future target MAY expose a capability unknown when the source program was written.

If that capability satisfies the existing semantic contract, the program SHOULD be usable without source modification.

This is a central requirement for POCO-REAF.


---

239. Scalability and Language Evolution

Future language versions SHOULD add abstractions rather than replace scalable principles.

New syntax MUST continue to follow:

semantic intent
        ≠
physical implementation

unless a feature explicitly exists for low-level target-specific programming.


---

240. Production Readiness Requirements

The scalability model is production-ready only if:

no arbitrary universal resource limits exist;

all resource abstractions are extensible;

logical/physical boundaries are explicit;

target variation is supported;

dynamic resources are supported;

resource insufficiency is well-defined;

compiler specialization is semantics-preserving;

runtime adaptation is policy-controlled;

quantum scaling is defined;

hardware scaling is defined;

distributed scaling is defined;

classical scaling is defined;

AI/data scaling is defined;

interoperability scaling is defined;

deterministic parsing is guaranteed;

implementation limits are distinguished from language semantics;

compatibility is versioned;

provenance is supported;

security is preserved;

tests verify absence of accidental limits.



---

241. Final Scalability Contract

Zamani SHALL implement the following fundamental rule:

> A Zamani program describes a computation and its semantic requirements, not the arbitrary size of the machine currently executing it.



Therefore:

One program
     ↓
One semantic meaning
     ↓
Many valid compilation strategies
     ↓
Many target architectures
     ↓
Many resource configurations
     ↓
Many execution scales
     ↓
Future architectures

provided the required semantic contract can be satisfied.


---

242. POCO-REAF Scalability Equation

The intended model is:

Portable Program
        +
Stable Semantics
        +
Extensible Compilation
        +
Capability Matching
        +
Resource Abstraction
        +
Target Adaptation
        +
Semantic Preservation
        =
POCO-REAF


---

243. Final Architectural Rule

Zamani MUST NOT ask:

> "How many resources does today's machine have?"



when defining the meaning of a portable program.

It SHOULD ask:

> "What computation does this program mean, what does it require, and how can the available target realize that meaning?"



That distinction is the foundation of:

From Atom to Everywhere

and:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever


---

244. Final Completion Checklist

Before this specification is accepted, verify:

[ ] Scope is defined.

[ ] Non-goals are defined.

[ ] "Infinity" is formally bounded to language semantics rather than physical impossibility.

[ ] No arbitrary finite resource limits are allowed.

[ ] Requirements are separated from capabilities.

[ ] Capabilities are separated from resources.

[ ] Constraints are separated from preferences.

[ ] Hints are separated from requirements.

[ ] Logical resources are separated from physical resources.

[ ] Target identity is separated from semantic identity.

[ ] Placement is separated from semantics.

[ ] Topology is separated from portable semantics.

[ ] Classical scalability is defined.

[ ] Quantum scalability is defined.

[ ] quantum::ir remains canonical.

[ ] Physical qubit identity is not universal semantics.

[ ] Quantum measurement remains explicit.

[ ] QEC ownership remains outside grammar scalability.

[ ] ZQN ownership remains outside grammar scalability.

[ ] HDL scalability is defined.

[ ] FPGA scalability is defined.

[ ] ASIC scalability is defined.

[ ] Accelerator scalability is defined.

[ ] Distributed scalability is defined.

[ ] AI/data scalability is defined.

[ ] Networking scalability is defined.

[ ] Security scalability is defined.

[ ] Resource negotiation is defined.

[ ] Dynamic resources are defined.

[ ] Resource exhaustion is defined.

[ ] Resilience integration is defined.

[ ] Recovery semantics are defined.

[ ] Quantum checkpoint limitations are acknowledged.

[ ] Determinism is defined.

[ ] Reproducibility is defined.

[ ] Numeric precision scaling is defined.

[ ] Compiler specialization is constrained by semantic preservation.

[ ] Runtime adaptation is constrained by semantic preservation.

[ ] POCO-REAF is reconciled with target-specific binaries.

[ ] Source portability is distinguished from binary portability.

[ ] Future hardware extensibility is defined.

[ ] Dialect scalability is defined.

[ ] Macro/metaprogramming scalability is defined.

[ ] ANTLR scalability requirements are defined.

[ ] AST scalability requirements are defined.

[ ] IR boundaries are defined.

[ ] No grammar/IR cycle exists.

[ ] No grammar/runtime cycle exists.

[ ] No grammar/hardware-discovery cycle exists.

[ ] Rust 1.97/1.97.1 compatibility is required.

[ ] unsafe Rust is prohibited.

[ ] Hard-coding audit rules are defined.

[ ] Implementation limits are distinguished from language limits.

[ ] Positive tests are defined.

[ ] Negative tests are defined.

[ ] Boundary tests are defined.

[ ] Property-based testing is defined.

[ ] Fuzz testing is defined.

[ ] Cross-domain tests are defined.

[ ] Compatibility tests are defined.

[ ] File-independent completion contract is defined.

[ ] Downstream integration contracts are defined.

[ ] Production readiness criteria are defined.



---

245. Normative Final Statement

The Zamani scalability model is therefore:

No arbitrary language-imposed machine limit
+
No hidden resource-count assumptions
+
Scale-independent program semantics
+
Explicit requirements
+
Explicit constraints
+
Explicit capabilities
+
Explicit preferences
+
Target-independent logical resources
+
Target-specific physical realization
+
Dynamic resource discovery
+
Semantic-preserving compilation
+
Semantic-preserving execution
+
Extensible future targets
+
Versioned compatibility
+
Deterministic language processing
+
Safe implementation
=
Zamani scalable from tiny systems to the largest
systems that available resources and physical reality permit.

The language's upper scalability boundary is therefore not a fixed number chosen by the grammar.

It is determined by the intersection of:

program requirements
        ∩
language semantics
        ∩
representation capability
        ∩
compiler capability
        ∩
runtime capability
        ∩
target capability
        ∩
available resources
        ∩
execution policy
        ∩
physical reality

while the source-level semantic model remains stable.

That is the normative foundation on which Zamani's grammar, compiler, runtime, quantum stack, hardware stack, distributed stack, and future computing integrations MUST build.