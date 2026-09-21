Below is the production-ready grammar/concurrency/README.md. It treats concurrency/ as a modular grammar domain, keeps concurrency.g4 as its composition boundary, prevents duplicate syntax ownership, preserves the existing token strategy, and defines integration contracts in advance so each child grammar can be completed independently.

# Zamani Concurrency Grammar

## Status

**Production architecture target:** normative  
**Domain:** concurrency, asynchronous execution, parallelism, tasks, actors, channels, synchronization, distributed concurrency  
**Language:** Zamani  
**Compiler baseline:** Rust 1.97 / Rust 1.97.1  
**Rust safety:** safe Rust only; `unsafe` is prohibited  
**Primary composition owner:** `grammar/concurrency/concurrency.g4`

This directory defines the **syntax-level concurrency domain** of the Zamani programming language.

It is designed for:

> **Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)**

Concurrency syntax describes **portable computational intent**. It must not encode assumptions about the size, topology, or implementation of the machine on which the program eventually executes.

---

# 1. Purpose

The `grammar/concurrency/` directory provides the modular grammar contracts for:

- asynchronous computation;
- futures and promises;
- tasks;
- spawning;
- awaiting;
- structured concurrency;
- parallel computation;
- data parallelism;
- task parallelism;
- pipelines;
- reductions;
- actors;
- channels;
- synchronization;
- cancellation;
- deterministic parallelism;
- distributed concurrency;
- future concurrency models.

The directory exists to prevent concurrency from becoming a monolithic collection of rules inside `Zamani.g4`.

The grammar must allow a program to express **what concurrency means** without requiring the programmer to specify **how many execution resources happen to exist**.

---

# 2. Architectural Principle

The concurrency grammar follows:

```text
Zamani source
    ↓
canonical lexer
    ↓
ANTLR parser
    ↓
domain-neutral frontend AST
    ↓
semantic analysis
    ├── types
    ├── ownership
    ├── effects
    ├── capabilities
    ├── resource requirements
    ├── determinism
    ├── synchronization
    └── distributed semantics
    ↓
canonical semantic IR
    ↓
optimization / lowering
    ↓
scheduling / placement / runtime realization
    ↓
target

The grammar is not the scheduler.

The grammar is not the runtime.

The grammar is not the thread manager.

The grammar is not the distributed deployment system.

The grammar is not the hardware topology model.


---

3. POCO-REAF Requirement

A valid Zamani concurrency program must remain meaningful when the available execution resources change.

For example, a program may describe:

parallel computation

without encoding:

run on 8 threads

The same program should be able to execute using whatever resources the compiler/runtime can legitimately provide:

one execution resource
multiple CPU cores
many CPU cores
GPU execution
accelerator execution
distributed workers
HPC resources
cloud resources
future execution models

provided the required semantic capabilities are available.

The concurrency grammar therefore has no universal limits on:

tasks;

futures;

actors;

channels;

workers;

threads;

cores;

processes;

nodes;

execution domains;

parallel branches;

pipeline stages;

distributed participants;

synchronization objects;

messages;

queues;

reductions;

timelines.



---

4. Hard-Coding Prohibition

The following must never become language-level grammar limits:

MAX_THREADS
MAX_TASKS
MAX_WORKERS
MAX_CORES
MAX_PROCESSES
MAX_ACTORS
MAX_CHANNELS
MAX_PIPELINE_STAGES
MAX_NODES
MAX_MESSAGES
MAX_PARALLEL_BRANCHES
MAX_FUTURES
MAX_QUEUES

Likewise, the grammar must never contain artificial alternatives such as:

threadCount
    : '1'
    | '2'
    | '4'
    | '8'
    | '16'
    ;

or:

workerCount
    : INTEGER /* only 1024 maximum */
    ;

A numeric literal supplied by a program is ordinary program data.

A compiler-imposed universal hardware limit is not.

For example:

parallel for i in 0..n

is valid portable intent.

A grammar rule that only permits a fixed number of parallel workers is not.


---

5. Requirement vs Implementation

Concurrency must preserve the distinction between:

Semantic requirement

What the computation requires.

requires deterministic ordering
requires atomicity
requires communication
requires capability("distributed.execution")

Capability

What the execution environment can provide.

capability("parallel.execution")
capability("async.execution")
capability("message.passing")

Constraint

A property that must be respected.

constraint ordering(...)
constraint latency(...)
constraint synchronization(...)

Preference

A non-mandatory optimization preference.

prefer parallel
prefer locality

Hint

Information supplied to aid implementation.

hint locality(...)

Implementation decision

A target-specific realization.

Examples include:

physical worker selection
CPU-core placement
GPU stream assignment
network-node placement
scheduler decisions

These belong downstream.

The grammar must not turn implementation decisions into universal concurrency semantics.


---

6. Directory Ownership

The directory has one composition boundary:

grammar/concurrency/concurrency.g4

The remaining files are specialized owners.

Recommended production structure:

concurrency/
├── README.md
├── concurrency.g4
├── async.g4
├── await.g4
├── spawn.g4
├── tasks.g4
├── futures.g4
├── actors.g4
├── channels.g4
├── synchronization.g4
├── cancellation.g4
├── parallel.g4
├── data-parallel.g4
├── task-parallel.g4
├── pipeline.g4
├── reduction.g4
├── deterministic-parallelism.g4
└── distributed-concurrency.g4

A new subdirectory should only be created when it introduces a genuinely independent maintainability boundary.

Do not create duplicate domain trees merely for organizational symmetry.


---

7. File Completion Contract

Every grammar file in this directory is complete only when the following contract has been satisfied.

Each file must have a clearly defined:

Purpose
Status
Owns
Does Not Own
Inputs
Outputs
Dependencies
Upstream Contracts
Downstream Consumers
Grammar Contract
Lexer Contract
AST Contract
Semantic Contract
IR Contract
Compiler Integration
Runtime Integration
Tooling Integration
Cross-Domain Integration
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Determinism Tests
Compatibility Tests
Diagnostics
Security
Performance
Hard-Coding Audit
Completion Criteria

This information should be documented in the file's header comments or corresponding specification document.

A file must not depend on an undocumented future edit to another grammar.


---

8. concurrency.g4

Ownership

concurrency.g4 is the concurrency composition boundary.

It is not the owner of every individual concurrency construct.

It composes the specialized grammar components.

Conceptually:

Concurrency
├── Tasks
├── Futures
├── Parallel
├── DataParallel
├── TaskParallel
├── Actors
├── Channels
├── Synchronization
├── Cancellation
├── Pipeline
├── Reduction
├── DeterministicParallelism
└── DistributedConcurrency

The exact imports must match the actual files and valid ANTLR parser-grammar declarations.

It owns

concurrency-domain composition;

the stable concurrency category;

integration of specialized concurrency grammars;

concurrency-level semantic classification.


It does not own

task leaf syntax;

future leaf syntax;

channel leaf syntax;

actor leaf syntax;

scheduler implementation;

resource allocation;

machine topology;

distributed deployment;

runtime behavior.


Important rule

Do not add sibling alternatives that cause the same concrete source text to be parseable through multiple ownership paths.

For example, if parallel.g4 owns a parallel construct, concurrency.g4 must not recreate the same construct.


---

9. async.g4

Owns

Asynchronous declaration/expression intent where asynchronous syntax is distinct from task creation or awaiting.

Does not own

future result semantics;

task spawning;

scheduler behavior;

executor implementation.


Integration

async syntax
    ↓
AST asynchronous construct
    ↓
effect / concurrency semantic analysis

The lexer remains responsible for canonical ASYNC.


---

10. await.g4

Owns

Awaiting asynchronous/future computation.

Existing asynchronous naming must remain compatible with the current frontend.

Known future-related grammar names such as:

futureExpression
futureObservationRoot
futureResultRoot

must be reconciled in their owning grammar rather than duplicated here.

Does not own

future creation;

task spawning;

executor implementation;

blocking policy;

runtime scheduling.


AST

Awaiting should map to the existing domain-neutral frontend representation, such as:

ExpressionKind::Await

rather than introducing a second concurrency IR.


---

11. spawn.g4

Owns

Explicit task/process/actor execution initiation where spawning is part of Zamani syntax.

Canonical lexer vocabulary includes:

SPAWN

Does not own

worker-count limits;

thread creation implementation;

operating-system process IDs;

CPU-core assignment;

cluster-node assignment.


A spawn describes computation creation, not physical resource allocation.


---

12. tasks.g4

Primary owner

tasks.g4 owns task syntax.

This is the authoritative location for task-specific concrete syntax.

It should cover the task abstraction required by the existing frontend and concurrency design.

Potential semantic concepts include:

task
spawn
task result
task handle
task scope
task dependency
task completion

Concrete syntax must be aligned with the existing lexer/parser rather than introducing a second vocabulary.

Does not own

generic parallel syntax;

distributed placement;

scheduling;

resource discovery;

hardware selection.



---

13. futures.g4

Owns

Future/promise-like computation syntax.

It must be reconciled with the existing future rule names.

The known existing naming discrepancy:

futureConstruct
futureStatement

versus actual future-oriented rules such as:

futureExpression
futureObservationRoot
futureResultRoot

must be resolved in this owning file.

No external file should have to compensate for inconsistent future rule names.

Completion requirement

There must be one authoritative future syntax model.


---

14. parallel.g4

Owns

Generic parallel computation syntax.

It must express semantic parallelism without imposing implementation topology.

Canonical existing token:

PARALLEL

must be retained.

Parallelism may represent:

independent operations;

parallel blocks;

parallel iteration;

parallel computation;

composable parallel expressions.


Does not own

task-specific syntax;

data-parallel reductions;

hardware placement;

worker count;

scheduler implementation.



---

15. data-parallel.g4

Owns

Data-parallel computation.

Examples of semantic concepts include:

map
transform
parallel iteration
partition-independent computation
collect
reduce

The concrete grammar must integrate with existing expression and collection syntax.

It must not create artificial limits on:

data elements;

partitions;

workers;

dimensions;

tensor size.



---

16. task-parallel.g4

Owns

Structured parallelism among tasks.

It composes with:

tasks
parallelism
dependencies
structured scopes

It must not redefine task syntax already owned by tasks.g4.

It must not define physical workers.


---

17. pipeline.g4

Owns

Pipeline composition.

A pipeline may express stages and dependencies:

stage → stage → stage

The number of stages must not be limited by the grammar.

Pipeline execution may later map to:

CPU execution;

GPU execution;

FPGA pipelines;

accelerator pipelines;

distributed pipelines;

streaming systems.


The grammar expresses the computational structure, not the implementation topology.


---

18. reduction.g4

Ownership

reduction.g4 is a semantic/concurrency adapter for reduction functionality.

The concrete reduction operation must have exactly one syntax owner.

If data-parallel.g4 owns the canonical reduction syntax, reduction.g4 must not recreate it.

For example, if the canonical syntax is already represented by:

parallel::reduce(...)

then reduction.g4 should integrate that construct rather than introduce a competing:

reduce(...)

syntax.

This prevents ambiguous parsing and competing AST mappings.


---

19. actors.g4

Owns

Actor-oriented concurrency syntax.

Concepts may include:

actor
actor state
message handling
mailbox interaction
actor lifecycle

The actor abstraction must remain independent of physical processes or machines.

One actor may eventually map to:

a local task;

a thread;

a process;

a runtime object;

a distributed service;

another execution mechanism.


The grammar does not choose the implementation.


---

20. channels.g4

Owns

Channel/message-passing syntax.

Concepts include:

channel
send
receive
select
close

Only tokens that already exist in the canonical lexer should be used without reconciliation.

Potential vocabulary such as:

SEND
RECEIVE
CHANNEL
SELECT

must be registered centrally if they are required.

Do not create local token definitions inside this parser grammar.

Channel capacity must not be interpreted as a universal machine limitation.


---

21. synchronization.g4

Owns

Language-level synchronization semantics.

Potential constructs include:

atomic
critical
synchronized
barrier
lock
condition
join

Only concepts that are actually accepted by the language specification should become syntax.

Synchronization semantics belong to semantic analysis.

The grammar does not define:

lock implementation;

OS primitives;

CPU instructions;

memory topology;

hardware coherence mechanisms.



---

22. cancellation.g4

Owns

Cancellation intent and structured cancellation syntax.

Cancellation must integrate with:

tasks
futures
async
structured concurrency
distributed concurrency

Cancellation semantics must distinguish:

request cancellation
observe cancellation
propagate cancellation
handle cancellation

The grammar must not assume that cancellation is implemented by a specific runtime primitive.


---

23. deterministic-parallelism.g4

Owns

Syntax expressing deterministic parallel computation where deterministic behavior is a language-level semantic requirement.

Examples of semantic properties include:

deterministic execution
deterministic reduction
ordered observation
reproducible parallel behavior

Determinism must be defined semantically.

It must not mean:

run on exactly N threads

or:

run on CPU core 0


---

24. distributed-concurrency.g4

Owns

Composition of concurrency with distributed computation.

It must integrate existing distributed grammar ownership without creating a second distributed language.

It may express semantic relationships such as:

distributed task
distributed actor
remote computation
message-based concurrency
collective computation
distributed synchronization

The actual distributed deployment model belongs downstream.

It must not encode fixed:

node counts
process counts
network sizes
machine IDs
topologies
worker counts
cluster dimensions


---

25. statements/concurrency.g4

This file must remain a thin statement-layer adapter.

The ownership rule is:

grammar/concurrency/
    ↓
owns concurrency syntax

grammar/statements/concurrency.g4
    ↓
adapts concurrency constructs into universal statement composition

It must not duplicate:

task
spawn
parallel
channel
actor
future

rules.

This prevents two parser paths from recognizing the same source construct.


---

26. Zamani.g4 Integration

Zamani.g4 remains the canonical ANTLR composition root.

It should not copy concurrency rules into the root grammar.

Conceptually:

Zamani.g4
    ↓
Concurrency
    ↓
specialized concurrency grammars

The root should expose the universal program/item/statement/expression/type composition.

Concurrency remains a domain module.


---

27. ZamaniParser.g4 Integration

Where the repository still contains ZamaniParser.g4, it must not become a competing language authority.

The established composition direction is:

ZamaniParser.g4
    ↓
Concurrency

while the preferred final canonical root remains:

Zamani.g4

No second independent concurrency grammar should be introduced under another root.


---

28. Lexer Integration

The lexer is the sole token authority.

Concurrency grammars consume canonical lexer tokens.

Known canonical concurrency tokens include:

ASYNC
AWAIT
SPAWN
PARALLEL

REQUIRES is also a canonical language token, but resource requirements belong to the resources domain.

Therefore concurrency grammars should not redefine:

REQUIRES

or give it concurrency-specific semantics.

Potential additional tokens such as:

ACTOR
CHANNEL
SEND
RECEIVE
SELECT
JOIN
CANCEL
ATOMIC
CRITICAL
SYNCHRONIZE
DISTRIBUTED
REMOTE

must first be reconciled with the canonical lexer registry.

Do not create parser-local lexical tokens.


---

29. Token Ownership Rule

All token definitions belong to the canonical lexer.

The hierarchy is:

lexer/
    ↓
canonical token vocabulary
    ↓
ANTLR parser grammars

Not:

concurrency grammar
    ↓
private tokens

This prevents token collisions across:

concurrency;

networking;

distributed;

hardware;

quantum;

HDL;

effects;

security.



---

30. AST Integration

Concurrency syntax must lower into the domain-neutral frontend AST.

The grammar must not define a second concurrency AST that duplicates the frontend architecture.

Examples of appropriate semantic AST categories include:

ExpressionKind::Await
Spawn
Task
Parallel
Call
Block
Operation

where those types already exist or are established by the frontend contract.

The preferred universal operation representation remains conceptually:

Operation {
    name,
    namespace,
    operands,
    parameters,
    results,
    attributes,
    modifiers,
    effects,
    capabilities,
    source
}

Concurrency-specific semantic information belongs in appropriate attributes/effects/semantic structures rather than a grammar-owned IR.


---

31. Semantic Integration

After parsing:

AST
 ↓
semantic analysis

Concurrency semantic analysis must determine:

task ownership;

lifetime;

scope;

dependency validity;

await validity;

send/receive compatibility;

synchronization correctness;

data races where applicable;

effect requirements;

cancellation propagation;

deterministic semantics;

distributed semantics;

capability requirements;

resource requirements.


The parser should not attempt to perform these checks.


---

32. Effects Integration

Concurrency constructs may introduce effects such as:

async
spawn
parallel
communication
synchronization
cancellation
distributed execution

Effects must be represented through the existing effect system.

The concurrency grammar does not become an effect system.

The pipeline is:

concurrency syntax
    ↓
AST
    ↓
effect analysis


---

33. Ownership and Memory Integration

Concurrency must integrate with the existing memory/ownership model.

The semantic layer must determine whether concurrent access is valid.

The grammar itself must not encode assumptions such as:

shared memory always exists

or:

all workers share one address space

Concurrency may eventually execute over:

shared memory
distributed memory
message passing
accelerator memory
remote memory
future memory models

without changing the source language's fundamental concurrency syntax.


---

34. Resources Integration

Resource declarations belong to:

grammar/resources/

not to the concurrency leaf grammars.

Concurrency may consume resource/capability information.

For example:

requires capability("parallel.execution")

may semantically constrain execution.

But the concurrency grammar must not turn this into:

requires 8 threads

as a universal implementation rule.

Resource negotiation belongs downstream.


---

35. Hardware Integration

Concurrency syntax must remain independent from:

CPU IDs
GPU IDs
QPU IDs
FPGA IDs
core IDs
thread IDs
memory-bank IDs
network-node IDs
device addresses

Hardware realization occurs later:

semantic concurrency
    ↓
resource analysis
    ↓
target selection
    ↓
scheduling
    ↓
placement
    ↓
HAL/runtime


---

36. Distributed Integration

Distributed concurrency must compose with:

grammar/distributed/
grammar/networking/
grammar/resources/
grammar/execution/

The concurrency grammar expresses distributed computation semantics.

The distributed subsystem determines actual realization.

For example, a logical collection of concurrent computations may eventually execute across any available number of resources.

No fixed node topology belongs in the grammar.


---

37. Quantum Integration

Concurrency may be used around or within hybrid quantum programs.

Examples include:

classical task
    ↓
quantum operation
    ↓
measurement
    ↓
classical task

or multiple independent quantum computations.

The concurrency grammar must not create a quantum execution IR.

The established quantum path remains:

Zamani syntax
    ↓
domain-neutral AST
    ↓
quantum semantic analysis
    ↓
quantum::ir
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
QEC / resilience / ZQN
    ↓
HAL

Concurrency metadata may influence semantic scheduling requirements, but the grammar does not own QEC, ZQN, routing, calibration, or HAL behavior.


---

38. HDL Integration

Concurrency concepts may interact with hardware-description and hardware/software co-design.

For example:

software computation
        +
parallel computation
        +
accelerator intent

The concurrency grammar must not assume a specific number of hardware pipelines, execution units, FPGA resources, or ASIC structures.

Those are resolved by:

HDL semantics
hardware analysis
resource analysis
compiler
synthesis
target backend


---

39. AI/Data Integration

Concurrency may compose with:

AI
data
tensor
stream
pipeline
distributed training

but must not introduce framework-specific syntax into the concurrency domain.

For example:

parallel data transformation

is a concurrency/data semantic combination.

A specific framework's executor belongs to interoperability/backend layers.


---

40. Networking Integration

Channels and distributed concurrency may use networking abstractions.

The grammar must distinguish:

logical communication

from:

physical network endpoint

The first may belong to the language.

The second is normally a deployment/runtime concern.

No universal network topology may be encoded.


---

41. Canonical IR Boundary

Concurrency grammar must not create:

ConcurrencyIR

merely because concurrency exists.

The canonical semantic representation must use the repository's established IR architecture.

Concurrency information should be represented through the canonical semantic model and corresponding IR structures owned by the compiler architecture.

The same principle applies to quantum:

DO NOT:
grammar → custom QuantumConcurrencyIR

DO:
grammar → AST → semantic concurrency/quantum analysis

and then use the existing canonical IR boundaries.


---

42. Scheduling

Scheduling is downstream.

The grammar can express semantic scheduling constraints where the language specification explicitly requires them.

It must not select:

CPU core
GPU stream
QPU
FPGA pipeline
cluster node
thread
worker

The implementation pipeline determines these choices.

This is essential for POCO-REAF.


---

43. Runtime Integration

The runtime may implement:

task execution;

futures;

async execution;

cancellation;

scheduling;

synchronization;

channels;

actors;

distributed communication;

recovery.


The grammar must remain independent of runtime implementation details.

Changing the runtime should not require rewriting the language grammar merely because a different executor is used.


---

44. Compiler Integration

The compiler must consume semantic concurrency information and lower it appropriately.

Potential targets include:

single-threaded execution
multicore CPU
GPU
FPGA
ASIC
QPU
accelerators
distributed systems
HPC
cloud
embedded systems
future execution platforms

The grammar must not encode these target choices.


---

45. Determinism

Parsing must be deterministic.

For identical source and language version:

same source
+
same grammar version
+
same lexical rules

must produce the same parse structure.

Concurrency itself may be nondeterministic at runtime when the program semantics permit it, but parser behavior must not be nondeterministic.

Where deterministic semantics are requested, semantic analysis must establish the required guarantees.


---

46. Ambiguity Prevention

Every concrete concurrency construct must have one authoritative syntax owner.

Examples:

tasks.g4          → task syntax
futures.g4        → future syntax
parallel.g4       → generic parallel syntax
data-parallel.g4  → data parallel syntax
task-parallel.g4  → task parallel syntax
actors.g4         → actor syntax
channels.g4       → channel syntax

Composition files must delegate.

They must not copy leaf rules.

This is particularly important for ANTLR imported grammars because duplicate alternatives can create:

ambiguity;

unreachable alternatives;

inconsistent parse trees;

different AST mappings for identical source;

maintenance cycles.



---

47. Circular Import Prevention

The dependency graph must remain acyclic.

Preferred direction:

leaf grammar
    ↓
domain composition
    ↓
universal grammar root

not:

Concurrency
    ↔
DistributedConcurrency

For example, if:

concurrency.g4

imports:

distributed-concurrency.g4

then distributed-concurrency.g4 must not import concurrency.g4.

It should instead depend on the specific leaf grammars it actually requires.


---

48. Cross-Domain Dependency Rule

A grammar may consume another domain's public grammar contract.

It must not silently redefine that domain.

Examples:

concurrency
    → resources

may consume resource/capability semantics.

But concurrency must not redefine the resource language.

Likewise:

distributed concurrency
    → distributed

does not mean distributed-concurrency owns distributed node syntax.


---

49. Grammar Actions

The concurrency grammars must contain no target-language semantic actions.

Do not embed Rust execution logic in .g4 files.

Avoid:

@members { ... }

for compiler/runtime behavior.

The grammar should produce parse structures.

Rust 1.97/1.97.1 code belongs in the frontend/compiler implementation.

Generated Rust must remain safe Rust.

No unsafe code is permitted.


---

50. Diagnostics

The grammar must preserve sufficient source information for diagnostics.

Every concurrency construct should ultimately be traceable to:

source file
source range
line
column
construct

Semantic diagnostics should be able to identify:

invalid await;

invalid task scope;

invalid synchronization;

invalid channel operation;

invalid cancellation;

incompatible task dependencies;

invalid distributed concurrency;

unsupported capability;

violated semantic requirement.


The grammar itself should not perform semantic diagnosis that belongs to later phases.


---

51. Security

Concurrency grammar design must avoid creating syntax that implicitly bypasses:

ownership;

authorization;

capability checking;

isolation;

effect checking;

resource policy.


Concurrency does not grant permission to access resources.

Security and capability analysis remain separate semantic concerns.


---

52. Performance

The grammar should scale with program size rather than machine size.

It must support:

tiny programs
large programs
deep task graphs
large parallel regions
large pipelines
large distributed programs
large generated programs

without artificial language limits.

Performance optimizations should target:

deterministic parsing;

avoiding duplicate alternatives;

avoiding unnecessary backtracking;

bounded local ambiguity;

reusable grammar components;

efficient tokenization.


Do not solve parser performance by imposing semantic limits on the language.


---

53. Scalability Contract

The grammar must be unbounded in the language-specification sense.

Actual execution remains constrained only by available:

memory
compute
storage
communication
runtime capabilities
compiler resources
target capabilities

The language must not impose artificial universal limits.

Therefore:

number of tasks
number of futures
number of actors
number of channels
number of workers
number of nodes
number of stages
number of messages

must be represented as grammar repetition, parameterized constructs, or ordinary program data rather than finite enumerations.


---

54. Compatibility

Concurrency syntax must be versioned through the repository's compatibility system.

Relevant integration points:

grammar/compatibility/
grammar/spec/
grammar/specification/

Compatibility must track:

language specification
lexer
parser grammar
AST
semantic analyzer
IR
compiler
runtime

A grammar change is not automatically a language-compatible change merely because ANTLR accepts it.


---

55. Deprecated Syntax

Deprecated concurrency syntax must remain explicitly documented.

Use the repository's compatibility/deprecation mechanism rather than silently accepting multiple interpretations forever.

A deprecated construct should have:

old syntax
replacement
version introduced
version deprecated
migration guidance
AST mapping
semantic compatibility
test coverage


---

56. Testing Requirements

The concurrency test suite must contain at least these categories:

tests/concurrency/
├── positive/
├── negative/
├── boundary/
├── scalability/
├── determinism/
├── compatibility/
├── async/
├── futures/
├── tasks/
├── parallel/
├── data-parallel/
├── task-parallel/
├── pipeline/
├── reduction/
├── actors/
├── channels/
├── synchronization/
├── cancellation/
├── distributed/
└── cross-domain/

The exact final directory layout should follow the existing repository instead of creating duplicates.


---

57. Positive Tests

Positive tests must verify:

valid async syntax;

valid await;

valid spawning;

valid task composition;

valid futures;

valid parallel blocks;

valid data parallelism;

valid task parallelism;

valid pipelines;

valid reductions;

valid actors;

valid channels;

valid synchronization;

valid cancellation;

valid deterministic parallelism;

valid distributed concurrency;

valid cross-domain compositions.



---

58. Negative Tests

Negative tests must verify rejection of:

malformed concurrency constructs;

invalid task syntax;

invalid await placement;

invalid channel syntax;

malformed synchronization;

malformed cancellation;

invalid distributed forms;

ambiguous constructs;

invalid resource/concurrency syntax;

unsupported syntax that is only documented as proposed.



---

59. Boundary Tests

Boundary tests must test semantic scale without establishing artificial limits.

Examples:

one task
many tasks
empty parallel region where legal
single pipeline stage
many pipeline stages
single channel
many channels
single distributed participant
many participants
deep dependency graph
large generated task graph

No test may accidentally establish a maximum such as:

1024 tasks maximum
64 channels maximum
32 workers maximum


---

60. Scalability Tests

Scalability tests must distinguish:

language scalability

from:

hardware capacity

The grammar must continue to describe larger programs without changing the language contract.

Tests should verify that no grammar rule contains machine-derived constants.


---

61. Determinism Tests

Identical input must produce identical parser results.

Tests should verify:

same source
same language version
same lexical configuration
→ same parse structure

Concurrency runtime nondeterminism must not leak into parser determinism.


---

62. Compatibility Tests

Compatibility tests must verify existing valid programs remain valid unless a documented breaking change is intentional.

Particular attention is required for:

ASYNC
AWAIT
SPAWN
PARALLEL

and any existing concurrency syntax already consumed by src/parser.rs.


---

63. Cross-Domain Tests

Concurrency must be tested with:

classical
quantum
hybrid
HDL
hardware
distributed
AI
data
networking
security
memory
effects
resources
execution

Examples include:

classical task → quantum operation
quantum measurement → classical task
parallel tensor computation
distributed data processing
async network operation
parallel accelerator computation
hardware/software co-design

These tests must validate composition rather than create duplicate domain syntax.


---

64. Grammar-to-AST Traceability

Every public concurrency grammar rule must have a predetermined AST destination.

Required mapping:

grammar rule
    ↓
AST construct
    ↓
semantic construct
    ↓
canonical IR representation

No grammar rule may be declared production-ready with:

AST mapping: TBD

or:

IR mapping: TBD

The mapping may point to an existing generic AST/semantic operation when a specialized AST node is unnecessary.


---

65. No Concurrency-Specific Duplicate IR

Do not introduce:

ConcurrencyIR
UniversalConcurrencyIR
TaskIR
ParallelIR

merely to represent parser output.

The existing compiler architecture owns IR.

Concurrency is a semantic domain that contributes information to the canonical IR pipeline.


---

66. Generic Operations

Concurrency operations that are naturally extensible should use the generic operation model rather than requiring an ever-growing parser enumeration.

This permits:

standard operation
custom operation
vendor operation
future operation
domain-specific operation

to be represented without changing the fundamental grammar every time a new backend or execution model appears.

This is particularly important for POCO-REAF.


---

67. Vendor Independence

The core concurrency grammar must not contain framework-specific or vendor-specific execution syntax such as:

CUDA thread block
specific GPU stream ID
specific CPU core ID
specific runtime executor ID
specific cluster node ID

Vendor interoperability may be provided through:

interoperability/
dialects/
capabilities/

with explicit contracts.


---

68. Dialect Integration

Concurrency dialects must declare:

dialect name
version
owner
syntax additions
semantic additions
AST mapping
IR mapping
capabilities
compatibility
feature gates

A dialect must not silently modify the meaning of core concurrency syntax.


---

69. Documentation Authority

The authority hierarchy is:

grammar/DESIGN.md
        ↓
grammar/specification/
        ↓
grammar/spec/
        ↓
canonical grammar
        ↓
frontend implementation
        ↓
grammar/grammar.md

Zamani-Grammar.md remains an extended/historical design reference.

It does not automatically create legal concurrency syntax.

grammar.md describes implementation conformance.

It is not a second independent grammar authority.


---

70. Existing Filename Preservation

Existing files must not be renamed merely to make the directory aesthetically consistent.

In particular, preserve established names such as:

concurrency.g4
tasks.g4
parallel.g4
data-parallel.g4
task-parallel.g4
reduction.g4
distributed-concurrency.g4

where they already exist.

Correct ownership and integration are preferred over unnecessary renaming.


---

71. Existing Grammar Reconciliation

Before adding new rules, each existing concurrency grammar must be checked for:

valid ANTLR parser grammar header;

tokenVocab = ZamaniLexer;

correct imports;

correct rule references;

duplicate rules;

duplicate parser paths;

undefined rules;

stale rule names;

token mismatches;

inconsistent punctuation;

source-span compatibility;

AST mapping;

semantic mapping;

IR mapping;

test coverage.


The future grammar naming mismatch is a specific example requiring correction in its owning file rather than being hidden by another grammar.


---

72. Canonical ANTLR Form

Parser grammars in this directory should follow the repository's canonical structure:

parser grammar <Name>;

options {
    tokenVocab = ZamaniLexer;
}

import <Dependency>;

The exact imported grammar list must correspond to actual owning files.

Do not invent imports merely because a future directory is planned.

Do not use a parser grammar as a lexer grammar.


---

73. Dependency Direction

The preferred dependency structure is:

lexer
  ↓
core syntax
  ↓
expressions / types / declarations
  ↓
concurrency leaf grammars
  ↓
Concurrency composition
  ↓
statement/domain composition
  ↓
Zamani.g4

Dependencies should flow toward composition.

Avoid reverse dependencies.


---

74. Independent-First Completion Strategy

The recommended implementation order inside this directory is:

1. README.md
2. token reconciliation
3. async.g4
4. await.g4
5. spawn.g4
6. tasks.g4
7. futures.g4
8. parallel.g4
9. data-parallel.g4
10. task-parallel.g4
11. actors.g4
12. channels.g4
13. synchronization.g4
14. cancellation.g4
15. pipeline.g4
16. reduction.g4
17. deterministic-parallelism.g4
18. distributed-concurrency.g4
19. concurrency.g4
20. statements/concurrency.g4 adapter
21. Zamani.g4 integration
22. frontend conformance
23. semantic/IR conformance
24. complete test matrix

The important principle is:

> Finish the contract before integrating the file.



Integration should consume an already-defined public contract rather than forcing the completed file to be redesigned later.


---

75. Definition of Done for Every .g4

A concurrency grammar file is DONE only when:

[ ] Purpose is documented.

[ ] Status is documented.

[ ] Ownership is explicit.

[ ] Non-ownership is explicit.

[ ] Inputs are defined.

[ ] Outputs are defined.

[ ] Dependencies are defined.

[ ] Import direction is acyclic.

[ ] Lexer tokens are identified.

[ ] No private lexer tokens exist.

[ ] Concrete syntax is unambiguous.

[ ] Rule names are stable.

[ ] Existing compatible names are preserved.

[ ] Duplicate syntax has been eliminated.

[ ] AST mapping is predetermined.

[ ] Semantic mapping is predetermined.

[ ] Canonical IR mapping is predetermined.

[ ] Compiler consumer is identified.

[ ] Runtime consumer is identified.

[ ] Tooling consumer is identified.

[ ] Cross-domain interactions are documented.

[ ] Diagnostics are defined.

[ ] Security implications are reviewed.

[ ] Performance implications are reviewed.

[ ] Positive tests exist.

[ ] Negative tests exist.

[ ] Boundary tests exist.

[ ] Scalability tests exist.

[ ] Determinism tests exist.

[ ] Compatibility tests exist.

[ ] Hard-coding audit passes.

[ ] No artificial machine limits exist.

[ ] No duplicate IR exists.

[ ] No unsafe Rust dependency is introduced.

[ ] Rust 1.97/1.97.1 compatibility is established.

[ ] Completion does not depend on an undocumented future edit.



---

76. Definition of Done for concurrency.g4

concurrency.g4 is complete when:

[ ] It is the sole concurrency composition boundary.

[ ] It imports the authoritative concurrency leaf grammars.

[ ] It does not duplicate leaf syntax.

[ ] It does not introduce private tokens.

[ ] It does not introduce hardware limits.

[ ] It does not introduce scheduler behavior.

[ ] It does not introduce runtime behavior.

[ ] It does not create a concurrency IR.

[ ] It exposes a stable concurrency semantic category.

[ ] It integrates with the universal statement/expression architecture.

[ ] It integrates with Zamani.g4.

[ ] It does not create an import cycle.

[ ] All imported grammar rules are valid.

[ ] All referenced rules exist.

[ ] AST mappings are known.

[ ] semantic mappings are known.

[ ] conformance tests pass.



---

77. Definition of Done for the Directory

The complete grammar/concurrency/ directory is production-ready only when:

every leaf grammar
      ↓
has one owner
      ↓
has one lexer vocabulary
      ↓
has one AST mapping
      ↓
has one semantic mapping
      ↓
has a canonical IR destination
      ↓
has tests
      ↓
is composed exactly once
      ↓
integrates into Zamani.g4

and:

no duplicate grammar authority
no duplicate tokens
no duplicate AST
no duplicate IR
no fixed hardware limits
no fixed concurrency limits
no scheduler embedded in grammar
no runtime embedded in grammar
no unsafe Rust
no undocumented compatibility break

remain.


---

78. Final Architecture

The intended final relationship is:

Zamani.g4
                         │
                         ▼
                  Concurrency.g4
                         │
       ┌─────────────────┼──────────────────┐
       │                 │                  │
       ▼                 ▼                  ▼
     Tasks            Futures            Parallel
       │                 │                  │
       │                 │          ┌───────┴────────┐
       │                 │          ▼                ▼
       │                 │     DataParallel       TaskParallel
       │                 │
       ├──────────────┬──┴───────┬──────────────┐
       ▼              ▼          ▼              ▼
    Actors         Channels   Synchronization Cancellation
       │              │          │              │
       └──────────────┴──────────┴──────────────┘
                              │
              ┌───────────────┼────────────────┐
              ▼               ▼                ▼
          Pipeline         Reduction      Determinism
                              │
                              ▼
                  DistributedConcurrency
                              │
                              ▼
                       Domain-neutral AST
                              │
                              ▼
                   Semantic Analysis
                              │
              ┌───────────────┼────────────────┐
              │               │                │
          Effects        Resources       Capabilities
              │               │                │
              └───────────────┼────────────────┘
                              ▼
                       Canonical IR
                              │
                ┌─────────────┼──────────────┐
                ▼             ▼              ▼
             Classical      Quantum          HDL
                │             │              │
                └─────────────┼──────────────┘
                              ▼
                       Optimization
                              │
                   Scheduling / Placement
                              │
                     Runtime / HAL
                              │
             ┌────────────────┼─────────────────┐
             ▼                ▼                 ▼
            CPU              GPU              FPGA
             │                │                 │
             ├────────────────┼─────────────────┤
             ▼                ▼                 ▼
            QPU          Distributed          Future
                         / HPC / Cloud        targets

The fundamental invariant is:

Zamani concurrency syntax
        ≠
machine topology
        ≠
scheduler
        ≠
runtime
        ≠
hardware

Instead:

Zamani concurrency syntax
        ↓
portable computational intent
        ↓
semantic analysis
        ↓
resource/capability resolution
        ↓
implementation selection
        ↓
execution

That separation is what allows the same concurrency program to scale from the smallest available execution environment to arbitrarily larger environments supported by the compiler, runtime, and target.


---

79. Final POCO-REAF Invariant

The concurrency subsystem must preserve this invariant:

> A Zamani program describes the computation and its semantic requirements, not the accidental size or topology of the machine currently available to execute it.



Therefore the grammar must never make today's:

CPU count
core count
thread count
GPU count
FPGA resources
QPU capacity
node count
memory capacity
network topology
accelerator count

into tomorrow's language limitations.

Resource availability determines realization.

The source program remains the portable semantic specification.

That is the concurrency-level foundation required for:

Program Once → Compile Once → Run Everywhere → Anywhere → Forever.

This README is intended to be the **contract for the whole `concurrency/` directory**, so the individual `.g4` files can subsequently be completed against fixed ownership and integration rules rather than repeatedly redesigned as other files change.