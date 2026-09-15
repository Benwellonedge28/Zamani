

# Zamani Execution Grammar

## `grammar/execution/`

Production-ready execution-language grammar architecture for the Zamani Universal Programming Language.

> **Zamani — From Atom to Everywhere**
>
> **POCO-REAF**
>
> **Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever**

---

# 1. Purpose

`grammar/execution/` defines the **source-level language for expressing execution intent** in Zamani.

Execution syntax answers questions such as:

- What computation should be executed?
- Under what semantic requirements?
- With what capabilities?
- Subject to what constraints?
- With what preferences?
- With what implementation hints?
- Against what abstract target?
- With what resource requirements?
- With what placement intent?
- With what scheduling intent?
- With what dispatch intent?
- With what synchronization semantics?
- With what lifecycle semantics?
- With what failure/recovery policy?
- With what deployment intent?

It does **not** perform execution.

The execution grammar is therefore a declarative boundary between the Zamani program and the mechanisms that eventually realize that program.

---

# 2. Fundamental Principle

The execution grammar must express:

```text
WHAT the program means
+
WHAT execution conditions are required
+
WHAT realizations are acceptable
+
WHAT realizations are preferred
+
WHAT implementation guidance is provided

It must not unnecessarily encode:

HOW a particular machine happens to execute it

Therefore execution grammar must remain independent of:

CPU model;

GPU model;

QPU model;

FPGA model;

ASIC model;

vendor;

device identifier;

physical address;

fixed machine topology;

fixed network topology;

fixed number of nodes;

fixed number of devices;

fixed number of qubits;

fixed number of cores;

fixed number of threads;

fixed memory capacity;

fixed accelerator count;

fixed queue capacity;

fixed schedule;

fixed routing;

fixed backend;

fixed simulator;

fixed deployment topology.



---

3. POCO-REAF Contract

Execution grammar is a critical component of Zamani's POCO-REAF architecture.

The intended lifecycle is:

Zamani source
    |
    v
Lexing
    |
    v
Parsing
    |
    v
AST
    |
    v
Name resolution
    |
    v
Type analysis
    |
    v
Effect analysis
    |
    v
Capability analysis
    |
    v
Resource analysis
    |
    v
Target resolution
    |
    v
Canonical semantic representation
    |
    +------------------+
    |                  |
    v                  v
Classical IR       quantum::ir
    |                  |
    +--------+---------+
             |
             v
Optimization
             |
             +--> Routing
             |
             +--> Scheduling
             |
             +--> Resilience
             |
             +--> Hardware realization
             |
             v
Compilation / lowering
             |
             v
Dispatch
             |
             v
Deployment
             |
             v
Runtime

The source program remains the semantic authority.

The physical execution environment is a realization of those semantics.


---

4. Scope

The execution grammar covers source-level execution constructs for:

classical programs;

quantum programs;

hybrid programs;

HDL/hardware programs;

accelerator programs;

AI/ML workloads;

distributed programs;

parallel programs;

embedded programs;

systems programs;

scientific programs;

data-processing programs;

networking programs;

heterogeneous programs;

future execution domains.


It must remain sufficiently generic that a future execution model can be introduced without redesigning the fundamental grammar.


---

5. Ownership

5.1 execution/ owns

The execution grammar subsystem owns syntax for:

execution declarations;

execution requests;

execution contexts;

execution requirements;

execution constraints;

execution preferences;

execution hints;

execution capability intent;

execution resource intent;

abstract target intent;

placement intent;

scheduling intent;

dispatch intent;

synchronization intent;

lifecycle intent;

result binding;

failure policy;

retry policy;

recovery intent;

deployment intent;

execution properties;

execution metadata;

execution-specific composition.



---

6. Non-Ownership

grammar/execution/ does not own:

Lexical semantics

Owned by the canonical lexer.

General expressions

Owned by the core/expression grammar.

General types

Owned by the type system.

Functions

Owned by the function grammar.

Modules

Owned by the module grammar.

Classical IR

Owned by the compiler/IR subsystem.

Quantum IR

Owned by:

quantum::ir

The execution grammar must never become a second quantum IR.

Quantum gates

Owned by the quantum grammar and semantic/IR layers.

QEC

Owned by the QEC subsystem.

Execution grammar may express QEC-related execution requirements or policies, but does not implement QEC.

ZQN

Owned by the Zamani Quantum Noise subsystem.

Execution grammar may express fault/noise-aware execution intent, but does not define noise models.

Optimization

Owned by optimization.

Routing

Owned by routing.

Scheduling algorithms

Owned by scheduling.

Hardware discovery

Owned by the hardware abstraction layer.

Resource discovery

Owned by resource management.

Calibration

Owned by hardware/calibration subsystems.

Resilience algorithms

Owned by resilience.

Runtime implementation

Owned by runtime.

Deployment implementation

Owned by deployment/runtime infrastructure.


---

7. Existing Execution Grammar

The execution subsystem already contains the following grammar components:

grammar/execution/
├── execution.g4
├── execution-context.g4
├── dispatch.g4
├── placement.g4
├── scheduling.g4
├── synchronization.g4
├── runtime-capabilities.g4
├── parallel-execution.g4
└── deployment.g4

The repository's execution directory therefore represents a modular grammar architecture rather than one monolithic grammar.

Each specialized grammar must retain a single responsibility.


---

8. Composition Root

The canonical composition root is:

grammar/execution/execution.g4

It defines the top-level execution language boundary.

It must not absorb every specialized execution grammar into one enormous grammar.

Instead:

execution.g4
       |
       +--> execution-context.g4
       |
       +--> dispatch.g4
       |
       +--> placement.g4
       |
       +--> scheduling.g4
       |
       +--> synchronization.g4
       |
       +--> runtime-capabilities.g4
       |
       +--> parallel-execution.g4
       |
       +--> deployment.g4

Specialized grammars must remain independently understandable and independently testable.


---

9. Canonical Dependency Direction

The dependency direction is:

ZamaniLexer
     |
     v
Core
     |
     v
ExecutionContext
     |
     +------------------------------+
     |                              |
     v                              v
Execution                  Specialized execution grammars
     |                              |
     +---------------+--------------+
                     |
                     v
              Canonical Parser
                     |
                     v
                    AST

The dependency direction must never become:

runtime -> execution grammar

or:

IR -> execution grammar

or:

hardware -> execution grammar

The grammar describes source syntax.


---

10. Grammar Technology Contract

The grammar must remain compatible with the repository's ANTLR architecture.

The intended implementation environment is:

Rust 1.97
or
Rust 1.97.1

Rust edition: 2021

unsafe Rust: prohibited

ANTLR grammars must remain parser/lexer definitions.

They must not embed Rust execution logic.

Do not use semantic actions to:

discover hardware;

allocate resources;

execute programs;

contact a runtime;

query a device;

access a filesystem;

access a network;

select a backend;

perform optimization;

perform scheduling;

perform routing.



---

11. Lexer Boundary

The execution grammars consume the canonical Zamani lexer.

They must not create a competing execution lexer.

The canonical lexer remains responsible for:

keywords;

identifiers;

literals;

operators;

punctuation;

comments;

lexical diagnostics.


Execution grammars may consume existing execution-related tokens.

They must not introduce a new keyword merely because a new execution property was needed.

Open semantic concepts should generally remain representable through the canonical identifier/name system.

This is important for long-term extensibility.

For example, Zamani should not require a new lexer token every time a future accelerator technology appears.


---

12. Open-Ended Semantic Vocabulary

Execution syntax must not depend on a permanently closed list of concepts such as:

cpu
gpu
qpu
fpga
asic
cluster
cloud
device_1
device_2

Instead, semantic names should be capable of representing future concepts.

For example:

capability: quantum
capability: tensor
capability: realtime
capability: distributed
capability: photonic
capability: neuromorphic
capability: future::capability

The parser recognizes the structure.

Semantic analysis determines the meaning.


---

13. Requirement vs Constraint vs Preference vs Hint

This distinction is mandatory.

Requirement

A requirement is mandatory.

requires quantum;

means the execution realization must satisfy the semantic requirement.

Failure to satisfy it is not merely a preference miss.


---

Constraint

A constraint restricts the set of valid realizations.

For example:

resource.capacity >= required_capacity;

The grammar does not decide whether the constraint can be satisfied.


---

Preference

A preference is desirable but not necessarily mandatory.

Failure to satisfy a preference must not automatically invalidate execution.


---

Hint

A hint is advisory implementation information.

A backend must not silently promote a hint into a semantic requirement.


---

14. Capability Semantics

Capabilities represent what an execution environment can provide.

Examples include:

quantum
classical
tensor
vector
realtime
distributed
parallel
accelerator
secure
persistent
fault_tolerant
dynamic_circuit

These are semantic capabilities, not device IDs.

The grammar must permit future capabilities without requiring a parser redesign.


---

15. Resource Semantics

Resource syntax represents resource requirements or resource-related intent.

Examples:

resource memory;
resource quantum;
resource accelerator;
resource bandwidth;
resource storage;
resource compute;

Quantities must remain expressions.

For example, a semantic requirement may depend on:

problem_size
input_size
algorithmic_requirement
available_capability

rather than a fixed physical maximum.


---

16. No Fixed Resource Limits

The grammar must never introduce:

MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_DEVICES
MAX_JOBS
MAX_STAGES
MAX_RETRIES

or equivalent parser-level limitations.

Repeated structures must use grammar repetition.

Actual limits belong to:

semantic analysis;

resource management;

compiler policy;

runtime policy;

operating-system constraints;

target capabilities;

deployment configuration;

user-declared constraints.



---

17. Target Semantics

A target is an abstract realization context.

A target may represent:

a language-level target class;

a compilation target;

an execution environment;

a capability profile;

a deployment environment;

a target expression;

a logical execution domain.


A target must not inherently mean:

physical device X

unless the program explicitly requests a concrete physical binding and that binding is semantically meaningful.


---

18. Target vs Hardware

This distinction is critical.

target

means:

> the desired realization category or environment.



Whereas:

hardware

describes:

> actual physical capabilities and state.



Execution grammar may reference target intent.

Hardware grammar/HAL determines what actually exists.


---

19. Placement

Placement expresses where execution is intended or constrained to occur.

It must not perform placement.

The grammar may represent concepts such as:

locality
affinity
anti_affinity
co_location
distribution
isolation
proximity

But it must not hard-code:

node 1
node 2
GPU 0
GPU 1
QPU 3

unless a physical identifier is explicitly part of the user's semantic program.

Even then, physical identity must remain an explicit portability trade-off rather than an implicit grammar assumption.


---

20. Scheduling

Scheduling syntax expresses scheduling intent.

It must not implement scheduling.

The execution grammar must not perform:

ASAP scheduling;

ALAP scheduling;

list scheduling;

critical-path scheduling;

RCPSP;

resource allocation;

dependency analysis;

pulse scheduling;

clock scheduling.


Those belong to the scheduling subsystem.

Execution syntax may express:

deadline
priority
ordering
latency preference
throughput preference
timing requirement
scheduling policy

The scheduler decides how to realize it.


---

21. Routing

Execution grammar must not implement routing.

This is especially important for quantum programs.

The grammar may express placement/routing intent.

It must not:

map logical qubits;

select physical qubits;

insert SWAP operations;

construct hardware topology;

select communication paths;

bind physical locations.


Those belong to routing and hardware-aware compilation.


---

22. Quantum Integration

Quantum execution is integrated through:

quantum grammar
       |
       v
semantic analysis
       |
       v
quantum::ir
       |
       v
optimization
       |
       v
routing
       |
       v
scheduling
       |
       v
execution

The execution grammar must not redefine:

qubits;

quantum registers;

gates;

operations;

circuits;

measurements;

observables;

quantum states;

quantum IR.


It may describe execution intent surrounding them.


---

23. Quantum Scalability

Execution grammar must support programs whose resource requirement scales with:

problem size
algorithm
input
logical structure
runtime capability

rather than:

fixed parser limit

The grammar must not assume:

32 qubits
64 qubits
127 qubits
1000 qubits

or any other finite universal machine size.


---

24. QEC Integration

QEC belongs to the QEC subsystem.

Execution grammar may express intent such as:

requires error_correction;

or structured execution policy referring to an error-correction capability.

But it must not define:

stabilizer algorithms;

surface-code algorithms;

decoding;

syndrome processing;

logical-qubit implementations;

correction circuits.


The execution layer says what resilience/correction property is desired.

QEC determines how correction is performed.


---

25. ZQN Integration

ZQN owns:

noise;

fault models;

fault classification;

channels;

correlated faults;

leakage;

loss;

erasure;

calibration/noise semantics;

noise-aware execution.


Execution grammar may express noise/fault-related requirements or policies.

It must not duplicate ZQN's semantic model.


---

26. Resilience Integration

Resilience sits above the execution mechanisms.

Conceptually:

execution intent
       |
       v
execution realization
       |
       v
runtime observations
       |
       v
resilience
       |
       +--> retry
       +--> recover
       +--> reroute
       +--> reschedule
       +--> recompile
       +--> change QEC
       +--> mitigate
       +--> switch backend
       +--> quarantine
       +--> abort

Execution grammar may express failure/retry/recovery policy intent.

It must not implement resilience algorithms.


---

27. Parallel Execution

parallel-execution.g4 owns specialized syntax for parallel execution semantics.

It must integrate with:

concurrency;

classical parallelism;

distributed execution;

accelerators;

scheduling;

resources.


It must not assume a fixed number of workers.

This is invalid as a universal language assumption:

parallel on 8 cores

unless 8 is explicitly a user-level constraint.

The grammar itself must not impose such a number.


---

28. Distributed Execution

Distributed execution must distinguish:

logical distribution

from:

physical node topology

Execution syntax may describe:

remote execution;

distribution intent;

communication requirements;

consistency requirements;

replication intent;

locality;

service execution.


The distributed subsystem decides how those requirements map onto actual infrastructure.


---

29. Runtime Capabilities

runtime-capabilities.g4 represents runtime capability requirements and intent.

It must not perform runtime discovery.

The runtime discovers actual capabilities.

Semantic analysis determines whether:

required capability

is satisfied by:

available capability


---

30. Deployment Boundary

Deployment is related to execution but remains a distinct concern.

Execution answers:

> execute this computation under these conditions.



Deployment answers:

> make this executable workload available in this deployment environment.



Deployment grammar must therefore remain distinct from:

hardware discovery;

resource discovery;

routing;

scheduling;

runtime dispatch.



---

31. Dispatch Boundary

Dispatch represents the transition toward execution.

It must not become the runtime itself.

Conceptually:

semantic program
      |
      v
compiled realization
      |
      v
dispatch intent
      |
      v
runtime

Dispatch syntax may identify the desired execution handoff semantics.

It must not call a backend from the grammar.


---

32. Synchronization

Synchronization grammar expresses semantic synchronization requirements.

It may represent:

ordering;

barriers;

completion;

dependency;

coordination;

visibility;

synchronization scopes.


It must not implement:

mutexes;

OS scheduling;

distributed consensus;

hardware synchronization algorithms.


Those belong to their appropriate semantic/runtime subsystems.


---

33. Lifecycle

Execution lifecycle syntax may represent:

start
run
pause
resume
stop
complete
cancel
restart
recover

The grammar describes intent.

The runtime owns lifecycle implementation.


---

34. Failure and Recovery

Failure policy must remain declarative.

Possible semantic policies include:

retry
recover
resume
restart
rollback
reroute
reschedule
recompile
reoptimize
switch_backend
quarantine
abort

The grammar must not contain retry algorithms.

In particular, this must not be hard-coded:

retry exactly 3 times

unless 3 is explicitly provided by the user as program policy.

The language infrastructure itself must not impose a universal retry count.


---

35. Checkpoint Semantics

Execution grammar must not imply that arbitrary quantum states can always be serialized.

Checkpoint semantics must distinguish:

classical execution state;

compiled program state;

logical checkpoint;

measurement boundary;

QEC-supported checkpoint;

reconstructible state;

provider-supported state.


Quantum state persistence is therefore a semantic/runtime capability rather than a universal parser assumption.


---

36. Expressions

Execution contexts should reuse the canonical expression grammar.

They must not create a second expression language.

Therefore execution values can ultimately depend on:

literals;

names;

qualified names;

calls;

arithmetic;

comparisons;

logical expressions;

ranges;

collections;

symbolic expressions;

domain-specific expressions.


This permits resource requirements to depend on program semantics.


---

37. Examples of Scalable Intent

Conceptually valid:

execute computation with {
    requires quantum;
    capability quantum;
}

A more resource-aware program may express:

execute computation with {
    resource quantum: required_qubits;
    resource memory: required_memory;
    target: compatible_target;
}

The important property is that:

required_qubits

can be derived from the program or input rather than being a grammar-level maximum.


---

38. What Execution Grammar Must Never Do

The execution grammar must never:

1. Allocate a device.


2. Discover hardware.


3. Select a backend implicitly.


4. Contact a runtime.


5. Execute a program.


6. Schedule an operation.


7. Route a circuit.


8. Optimize an IR.


9. Construct quantum::ir.


10. Construct classical IR.


11. Perform QEC.


12. Generate noise.


13. Diagnose faults.


14. Query calibration.


15. Access the filesystem.


16. Access the network.


17. Contain Rust semantic actions.


18. Depend on machine-specific limits.


19. Depend on a fixed topology.


20. depend on a particular vendor.




---

39. AST Contract

Execution grammar must lower into an execution-oriented AST representation.

The AST should preserve:

ExecutionDeclaration
    subject
    context
        requirements
        constraints
        preferences
        hints
        capabilities
        resources
        target
        placement
        scheduling
        dispatch
        synchronization
        lifecycle
        result
        failure
        retry
        deployment
        properties
        metadata

The AST must preserve source information necessary for diagnostics:

source span;

file/module identity;

syntax position;

declaration identity;

relevant attributes.


The AST must not prematurely resolve physical resources.


---

40. Semantic Analysis Contract

Semantic analysis consumes the execution AST and determines:

name validity;

type validity;

requirement validity;

capability validity;

resource validity;

constraint validity;

preference validity;

hint validity;

target compatibility;

execution-policy compatibility;

effect compatibility;

cross-domain compatibility.


Semantic analysis must reject contradictory or impossible declarations where the language specification requires rejection.

The parser itself should not attempt to perform these checks.


---

41. Resource Analysis

Resource analysis determines whether an execution request can be satisfied.

The distinction is:

grammar:
    resource requirement syntax

semantic analysis:
    resource meaning

resource manager:
    available resources

target resolver:
    possible target realization

scheduler:
    temporal resource realization

runtime:
    actual resource state

This separation is mandatory for scalability.


---

42. Capability Analysis

Capability analysis evaluates:

required capabilities
        vs
available capabilities

It must support capability sets that grow over time.

The grammar must not require a closed universe of capabilities.


---

43. Compiler Integration

Execution grammar feeds semantic execution intent into compilation.

The compiler may use execution intent to determine:

applicable lowering;

target compatibility;

feature selection;

optimization policy;

code generation;

backend selection;

compilation strategy.


However, execution grammar itself does none of these.


---

44. Runtime Integration

The runtime consumes an already validated execution representation.

Runtime responsibilities include:

actual resource acquisition;

runtime capability inspection;

dispatch;

lifecycle management;

execution;

telemetry;

failures;

recovery coordination.


Runtime must never need to parse source grammar as part of normal execution.


---

45. Hardware Integration

Hardware abstraction provides actual capabilities.

Examples:

available qubits
available compute
memory
accelerators
connectivity
timing
supported operations
supported execution modes

These are runtime/target facts.

They must not become universal grammar constants.


---

46. Scheduling Integration

The scheduler consumes:

semantic execution intent
+
operation dependencies
+
resource requirements
+
hardware capabilities
+
timing constraints
+
scheduling preferences

The grammar merely supplies the source-level intent.


---

47. Optimization Integration

Optimization consumes canonical representations.

Execution grammar does not define optimization passes.

Examples such as:

optimization preference

may be represented syntactically.

But passes such as:

cancellation;

peephole optimization;

T-gate reduction;

circuit optimization;

classical optimization;


remain compiler responsibilities.


---

48. Interoperability

Execution grammar must support execution of programs originating from:

Zamani;

foreign functions;

C;

C++;

Python;

OpenQASM;

Verilog;

other supported dialects.


Interoperability grammars own the syntax of the foreign representation.

Execution grammar owns the common execution intent surrounding the resulting semantic program.


---

49. Dialect Integration

Vendor and experimental execution features must not contaminate the universal core.

Dialect-specific features should be represented through:

dialect namespace
+
version
+
capability
+
explicit extension

rather than silently becoming universal assumptions.


---

50. Versioning

Execution grammar must participate in Zamani language versioning.

Compatibility must distinguish:

source syntax compatibility
AST compatibility
semantic compatibility
IR compatibility
runtime compatibility
target compatibility

A runtime implementation may evolve without changing the meaning of valid source programs.


---

51. Forward Compatibility

The execution subsystem must be designed so that future execution models can be introduced without redesigning the core execution declaration.

Future possibilities include:

new accelerators;

new quantum technologies;

photonic computation;

neuromorphic systems;

molecular/biological computation;

optical computing;

future distributed architectures;

future hardware fabrics;

unknown execution environments.


Open semantic names and extensible context structures are therefore preferred over fixed enumerations.


---

52. Determinism

Parsing must be deterministic.

For identical canonical token streams:

same input
    ->
same parse structure

There must be no:

random behavior;

hardware discovery;

runtime calls;

filesystem access;

network access;

time-dependent parsing;

global mutable state.



---

53. Diagnostics

Execution grammar must support high-quality syntax diagnostics.

Diagnostics should identify:

unexpected token;

missing delimiter;

malformed execution context;

malformed property;

invalid expression placement;

malformed nested context;

malformed execution clause.


Semantic diagnostics belong to later phases.

The parser must not silently convert malformed execution intent into comments or ignored syntax.


---

54. Error Recovery

ANTLR error recovery must not produce an apparently valid execution AST from malformed syntax without an explicit diagnostic.

In particular:

invalid execution syntax

must never silently become:

ignored execution property

because doing so could cause a program to execute with different semantics from what the developer wrote.


---

55. Security

Execution grammar must not permit the grammar itself to become a security bypass.

Security-related intent belongs to:

grammar/security/

Execution may reference security requirements.

Examples:

requires trusted_execution;
requires secure_environment;

Actual authorization remains a semantic/runtime responsibility.

Credentials and secrets must never be embedded as required grammar constants.

References to secrets should be represented through appropriate secure configuration mechanisms.


---

56. Scalability Model

The grammar must scale structurally.

This means that grammar complexity must not depend on:

number of CPUs
number of GPUs
number of qubits
number of nodes
number of devices
memory size
network size

Instead, source constructs scale through:

repetition
nesting
expressions
generics
symbolic values
resource descriptions
capabilities
constraints
runtime discovery


---

57. "Infinity" Interpretation

"Infinity" is an architectural scalability objective, not a promise of physically infinite hardware.

Zamani must support:

> any finite workload that can be represented and executed within the available computational, memory, storage, communication, compiler, runtime, and target resources.



Therefore:

language scalability

must not be confused with:

physical hardware capacity

The grammar must impose no arbitrary finite ceiling where none is semantically necessary.


---

58. Hard-Coding Audit

Every execution grammar file must be audited for:

MAX_*
fixed counts
fixed device IDs
fixed topology
fixed addresses
fixed vendors
fixed backend names
fixed qubit counts
fixed core counts
fixed thread counts
fixed memory limits
fixed retry counts
fixed schedule lengths
fixed deployment sizes

Each finding must be classified as:

1. language semantic requirement;


2. target-specific requirement;


3. resource constraint;


4. implementation limitation;


5. accidental hard-coding;


6. test-only limitation;


7. documentation-only limitation.



Accidental hard-coding must be removed.


---

59. Testing Strategy

Every execution grammar component requires:

Positive tests

Valid execution declarations.

Negative tests

Malformed execution declarations.

Boundary tests

Very small and very large execution contexts.

Cross-domain tests

Examples combining:

classical + execution
quantum + execution
hybrid + execution
HDL + execution
hardware + execution
distributed + execution
AI + execution
quantum + distributed + execution
classical + quantum + HDL + execution

Scalability tests

Verify no parser-level machine-size restrictions.

Determinism tests

Repeated parsing must produce equivalent structures.

Round-trip tests

Where a canonical printer exists:

source
  -> lexer
  -> parser
  -> AST
  -> printer
  -> parser

must preserve semantic intent.


---

60. Execution-Specific Test Matrix

At minimum test:

execute expression;

execute block;

execute expression with context;

execute classical computation;

execute quantum computation;

execute hybrid computation;

execute hardware computation;

execute distributed computation;

execute parallel computation;

execute accelerator computation;

execute AI workload;

execute with requirements;

execute with constraints;

execute with preferences;

execute with hints;

execute with capabilities;

execute with resources;

execute with target;

execute with placement;

execute with scheduling;

execute with dispatch;

execute with synchronization;

execute with lifecycle;

execute with failure policy;

execute with retry policy;

execute with deployment policy;

nested execution context;

nested property object;

expression-valued resource requirement;

expression-valued capability requirement;

future/unknown capability name;

namespaced execution property;

large context;

deep context nesting;

large expression;

large resource expression.


---

61. Negative Test Matrix

Tests must reject or diagnose:

unterminated execution context;

missing execution subject;

malformed property;

missing property value;

invalid operator;

invalid nesting;

malformed list;

malformed argument;

invalid separator;

unexpected token;

unclosed block;

invalid execution declaration;

Semantic tests should additionally verify rejection of:

contradictory mandatory requirements;

impossible constraints;

invalid capability references;

invalid resource expressions;

invalid target references;

invalid cross-domain combinations;


---

62. Cross-Domain Integration Tests

The execution subsystem must eventually test combinations such as:

classical -> execution
quantum -> execution
hybrid -> execution
HDL -> execution
hardware -> execution
distributed -> execution
AI -> execution
networking -> execution
security -> execution
resource -> execution
compile -> execution
deployment -> execution
resilience -> execution

The tests must ensure that the execution grammar does not redefine concepts belonging to these domains.


---

63. File Independence Contract

Every execution grammar file must be independently completable.

Before implementing a file, its integration contract must already identify:

Purpose
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
Tests
Negative Tests
Boundary Tests
Compatibility Requirements
Scalability Requirements
Hard-Coding Audit
Completion Criteria

No file should be declared complete until all of these are known.


---

64. File Contracts

execution.g4

Purpose

Composition root for source-level execution declarations.

Owns

execution declaration;

execution request;

execution subject;

top-level execution composition.


Does not own

Specialized scheduling, dispatch, placement, deployment, or runtime algorithms.

Depends on

canonical lexer;

Core;

execution-context definitions.


Consumers

canonical Zamani parser;

frontend AST;

semantic analyzer.


Completion criteria

no duplicate execution declaration;

no hardware-specific limits;

deterministic parsing;

correct integration with canonical parser;

no semantic actions;

all execution context forms represented consistently.



---

execution-context.g4

Purpose

Reusable execution-context syntax.

Owns

context;

context entries;

keys;

values;

nested objects;

structured properties;

context composition.


Does not own

execution itself;

hardware discovery;

resource allocation;

scheduling;

routing.


Depends on

Core;

canonical expression/name grammar.


Consumers

execution;

dispatch;

deployment;

distributed execution;

future execution extensions.


Completion criteria

open semantic vocabulary;

expression reuse;

arbitrary context size;

nested context support;

deterministic syntax;

no physical limits.



---

scheduling.g4

Purpose

Scheduling intent syntax.

Owns

scheduling policies;

timing intent;

ordering requirements;

deadlines;

priorities;

latency/throughput intent.


Does not own

scheduling algorithms;

actual timing;

resource allocation.


Integrates with

resources;

hardware;

quantum;

concurrency;

execution.



---

placement.g4

Purpose

Placement intent.

Owns

locality;

affinity;

distribution;

placement constraints.


Does not own

actual placement;

routing;

hardware discovery.



---

dispatch.g4

Purpose

Execution handoff intent.

Owns

dispatch mode;

execution handoff;

dispatch policies;

dispatch properties.


Does not own

runtime execution.



---

synchronization.g4

Purpose

Synchronization semantics.

Owns

synchronization intent;

ordering;

completion;

barriers;

dependencies.


Does not own

runtime synchronization implementation.



---

runtime-capabilities.g4

Purpose

Runtime capability intent.

Owns

runtime capability requirements;

capability preferences;

capability conditions.


Does not own

capability discovery.



---

parallel-execution.g4

Purpose

Parallel execution intent.

Owns

parallel execution structures;

parallelism declarations;

parallel execution policies.


Does not own

fixed worker counts;

runtime thread allocation;

scheduling algorithms.



---

deployment.g4

Purpose

Deployment intent.

Owns

deployment configuration;

lifecycle;

availability;

rollout;

persistence;

deployment policies.


Does not own

hardware discovery;

runtime allocation;

resource discovery;

routing;

scheduling.



---

65. Required Future File: distributed-execution.g4

If distributed execution requires a distinct execution-language composition layer, create:

grammar/execution/distributed-execution.g4

It should own:

distributed execution intent;

remote execution composition;

distributed execution policies.


It should integrate with:

grammar/distributed/

but must not duplicate:

nodes;

communication;

messaging;

replication;

consistency;

fault tolerance;

physical topology.



---

66. No Empty Files

No file should exist merely because it appears in an architectural tree.

A file must exist only when it has:

a distinct responsibility;

a stable public grammar contract;

real consumers;

tests;

documentation;

a reason not to be merged into another grammar.


If two grammar files cannot maintain independent ownership boundaries, they should be merged.


---

67. Avoiding Grammar Fragment Explosion

Modularity is valuable, but excessive fragmentation is harmful.

Do not create one .g4 file for every keyword.

A new file is justified when it represents:

1. an independently evolving semantic domain;


2. an independently testable grammar contract;


3. a significant integration boundary;


4. a domain with multiple consumers;


5. a domain that would otherwise make another grammar unmaintainably large.




---

68. Canonical Parser Integration

The canonical parser must compose execution grammar without creating duplicate entry points.

There should be one authoritative path from:

source

to:

executionDeclaration

Specialized execution grammars should expose reusable rules rather than competing top-level language grammars.


---

69. AST Stability

Execution AST nodes should not contain target-specific fields such as:

gpu_count
qpu_count
cpu_count
fixed_node_count
fixed_device_id

unless those values are explicitly represented as source semantics.

Prefer:

resource requirement
capability requirement
target expression
constraint
preference

which can later be resolved against actual resources.


---

70. Semantic Preservation

The compiler must preserve the meaning of:

program
+
execution intent

when moving between:

source
AST
semantic representation
IR
compiled representation
runtime plan

Optimization and hardware realization may change implementation.

They must not silently change semantic execution requirements.


---

71. Portability Rule

An execution declaration should remain portable unless the developer explicitly introduces a portability-restricting requirement.

For example:

requires quantum;

is portable across compatible quantum environments.

Whereas:

target: specific_physical_environment;

may intentionally reduce portability.

The compiler must distinguish those cases.


---

72. Portability Diagnostics

Tooling should be capable of warning when source syntax introduces unnecessary portability restrictions.

Examples:

hard-coded physical target
hard-coded device identity
hard-coded topology
hard-coded resource count
vendor-specific execution requirement

These may be valid programs.

They should not silently be treated as universally portable.


---

73. Capability Negotiation

POCO-REAF requires capability negotiation.

Conceptually:

program requirements
        |
        v
candidate target capabilities
        |
        v
constraint evaluation
        |
        v
acceptable realization

If multiple realizations satisfy the requirements, the implementation may select among them according to:

constraints;

preferences;

hints;

resource availability;

runtime policy.


The grammar must not choose one implicitly.


---

74. Resource Negotiation

Resource requests should follow:

program
  |
  v
resource requirement
  |
  v
resource analysis
  |
  v
available resources
  |
  v
candidate realization

This is what allows the same program to scale from a tiny environment to a very large one.


---

75. Semantic Requirements vs Implementation Decisions

This distinction must be preserved everywhere.

Semantic requirement

requires quantum;

Implementation decision

use backend X

Semantic resource requirement

requires resources sufficient for problem;

Implementation decision

allocate device 4

Semantic timing requirement

deadline <= expression

Implementation decision

schedule operation at physical time T

The grammar must not accidentally collapse these layers.


---

76. Execution and Compile Separation

Compilation and execution are distinct.

compile

describes production of a realization.

execute

describes execution of a semantic/compiled workload.

A compilation target must not automatically become an execution target.

A program may be compiled once and subsequently executed in multiple compatible environments.

This separation is fundamental to POCO-REAF.


---

77. Compile-Once Semantics

POCO-REAF does not mean that one binary is guaranteed to execute unchanged on every possible architecture.

Instead, the semantic representation must remain portable and reusable.

Therefore the architecture should distinguish:

source semantics

from:

target-specific realization

and from:

runtime execution state


---

78. Forever Semantics

"Forever" means the semantic language representation must support evolution.

Therefore execution grammar requires:

explicit versioning;

compatibility rules;

reserved namespace space;

dialect mechanisms;

deprecation policy;

migration mechanisms;

stable AST semantics;

extensible execution properties.


Future hardware must not require rewriting historical programs merely because a new device category appeared.


---

79. Documentation Contract

The execution subsystem documentation must stay synchronized with:

grammar/execution/*.g4

and the language specification.

Documentation must distinguish:

syntax
semantic meaning
compiler behavior
runtime behavior
hardware realization

Never describe a runtime implementation detail as if it were grammar semantics.


---

80. Repository Integration

The execution subsystem integrates with:

grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
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
grammar/interoperability/
grammar/dialects/
grammar/macros/
grammar/metaprogramming/

The integration rule is:

execution consumes their semantic intent

not:

execution redefines their language


---

81. Repository Runtime Integration

Execution semantics eventually connect to:

AST
  |
  v
semantic analysis
  |
  v
IR
  |
  v
optimization
  |
  v
routing
  |
  v
scheduling
  |
  v
resilience
  |
  v
hardware HAL
  |
  v
dispatch
  |
  v
runtime

No grammar file should directly depend on runtime implementation.


---

82. Dependency-First Implementation Order

The execution subsystem should be implemented in this order:

1. Core expression/name contracts
       |
       v
2. execution-context.g4
       |
       v
3. execution.g4
       |
       +--> placement.g4
       |
       +--> scheduling.g4
       |
       +--> synchronization.g4
       |
       +--> runtime-capabilities.g4
       |
       +--> dispatch.g4
       |
       +--> parallel-execution.g4
       |
       +--> deployment.g4
       |
       +--> distributed-execution.g4 (if retained)
       |
       v
4. Canonical parser integration
       |
       v
5. AST integration
       |
       v
6. Semantic analysis
       |
       v
7. Cross-domain tests
       |
       v
8. Round-trip and compatibility tests

No downstream grammar should require fundamental redesign of a completed upstream grammar.


---

83. Completion Gate

An execution grammar file is not complete until:

syntax is defined;

ownership is defined;

non-ownership is defined;

dependencies are fixed;

upstream contracts are fixed;

downstream contracts are fixed;

AST mapping is defined;

semantic mapping is defined;

IR integration is defined;

compiler integration is defined;

runtime integration is defined;

cross-domain integration is defined;

positive tests exist;

negative tests exist;

boundary tests exist;

scalability tests exist;

determinism tests exist;

hard-coding audit passes;

documentation exists;

compatibility impact is documented;

canonical parser integration succeeds.



---

84. Production-Readiness Checklist

Architecture

[ ] Single execution architecture.

[ ] No circular dependencies.

[ ] Clear ownership.

[ ] Clear non-ownership.

[ ] Specialized grammars remain modular.

[ ] No duplicate semantic representations.


Syntax

[ ] Execution declarations supported.

[ ] Contexts supported.

[ ] Requirements supported.

[ ] Constraints supported.

[ ] Preferences supported.

[ ] Hints supported.

[ ] Capabilities supported.

[ ] Resources supported.

[ ] Targets supported.

[ ] Placement supported.

[ ] Scheduling supported.

[ ] Dispatch supported.

[ ] Synchronization supported.

[ ] Lifecycle supported.

[ ] Failure/recovery intent supported.

[ ] Deployment intent supported.


Scalability

[ ] No fixed qubit limit.

[ ] No fixed CPU limit.

[ ] No fixed core limit.

[ ] No fixed thread limit.

[ ] No fixed GPU limit.

[ ] No fixed FPGA limit.

[ ] No fixed node limit.

[ ] No fixed device limit.

[ ] No fixed memory limit.

[ ] No fixed topology.

[ ] No fixed retry limit.

[ ] No fixed schedule size.


Quantum

[ ] Quantum syntax remains owned by quantum grammar.

[ ] quantum::ir remains canonical.

[ ] No duplicate quantum IR.

[ ] No physical-qubit assumptions.

[ ] QEC remains separate.

[ ] ZQN remains separate.

[ ] Routing remains separate.

[ ] Scheduling remains separate.


Runtime

[ ] No runtime calls from grammar.

[ ] No hardware discovery from grammar.

[ ] No resource allocation from grammar.

[ ] No backend selection from grammar.

[ ] No deployment execution from grammar.


Safety

[ ] No Rust semantic actions.

[ ] No unsafe Rust requirement.

[ ] Rust 2021 compatible.

[ ] Rust 1.97 compatible.

[ ] Rust 1.97.1 compatible.


Testing

[ ] Positive tests.

[ ] Negative tests.

[ ] Boundary tests.

[ ] Cross-domain tests.

[ ] Scalability tests.

[ ] Determinism tests.

[ ] Round-trip tests.

[ ] Compatibility tests.


Documentation

[ ] Architecture documented.

[ ] Ownership documented.

[ ] Dependency graph documented.

[ ] AST contract documented.

[ ] Semantic contract documented.

[ ] Compiler contract documented.

[ ] Runtime contract documented.

[ ] Compatibility policy documented.



---

85. Final Architecture

The execution subsystem must ultimately implement this model:

ZAMANI SOURCE
                       |
                       v
                CANONICAL LEXER
                       |
                       v
              CANONICAL PARSER
                       |
                       v
              EXECUTION GRAMMAR
                       |
                       v
             EXECUTION INTENT AST
                       |
          +------------+------------+
          |            |            |
          v            v            v
      capability    resource     target
      analysis      analysis     analysis
          |            |            |
          +------------+------------+
                       |
                       v
              SEMANTIC PROGRAM
                       |
          +------------+-------------+
          |            |             |
          v            v             v
     classical IR  quantum::ir   HDL/hardware
          |            |             |
          +------------+-------------+
                       |
                       v
                  COMPILATION
                       |
          +------------+-------------+
          |            |             |
          v            v             v
      optimize      routing      scheduling
          |            |             |
          +------------+-------------+
                       |
                       v
                   RESILIENCE
                       |
                       v
                 HARDWARE HAL
                       |
                       v
                   DISPATCH
                       |
                       v
                  DEPLOYMENT
                       |
                       v
                    RUNTIME

The critical invariant is:

GRAMMAR DESCRIBES INTENT.
SEMANTIC ANALYSIS INTERPRETS INTENT.
IR REPRESENTS SEMANTICS.
COMPILERS REALIZE SEMANTICS.
SCHEDULERS ORDER EXECUTION.
ROUTERS MAP EXECUTION.
HARDWARE PROVIDES CAPABILITIES.
RESILIENCE ADAPTS EXECUTION.
RUNTIME EXECUTES.

Never reverse these responsibilities.


---

86. Ultimate Zamani Execution Principle

The execution grammar must preserve:

> One program → one semantic meaning → many possible realizations.



Therefore:

one source program
        |
        +--> tiny machine
        |
        +--> CPU
        |
        +--> multicore
        |
        +--> GPU
        |
        +--> FPGA
        |
        +--> ASIC
        |
        +--> QPU
        |
        +--> simulator
        |
        +--> accelerator
        |
        +--> cluster
        |
        +--> supercomputer
        |
        +--> distributed environment
        |
        +--> cloud
        |
        +--> edge
        |
        +--> future architecture

without requiring the programmer to rewrite the semantic program merely because the realization environment changes.

The environment determines how the program is realized.

The Zamani program determines what computation means.

That separation is the foundation of:

Zamani — From Atom to Everywhere

and:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

POCO-REAF

.