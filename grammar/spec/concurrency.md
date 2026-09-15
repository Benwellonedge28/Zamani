Zamani Concurrency Specification

Path: "grammar/spec/concurrency.md"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Branch: "main"
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust edition: 2021
Rust safety requirement: production implementation uses safe Rust only; no Rust "unsafe"
Primary objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Scalability objective: from the smallest meaningful computation to arbitrarily large computations, constrained only by program semantics, representation limits, declared requirements, and resources actually available to the compiler/runtime/target.

---

1. Status

This document is the normative specification for the semantic concurrency contract of Zamani.

It defines:

- what concurrency means in Zamani;
- what concurrency syntax may express;
- what the concurrency grammar owns;
- what it explicitly does not own;
- task concurrency;
- asynchronous computation;
- futures and awaitables;
- actors;
- message passing;
- channels;
- synchronization;
- structured concurrency;
- cancellation;
- task parallelism;
- data parallelism;
- general parallelism;
- deterministic parallelism;
- resource-aware execution;
- distributed concurrency;
- heterogeneous concurrency;
- quantum/classical concurrency;
- HDL/hardware concurrency;
- effects and capabilities;
- memory and ownership interaction;
- type-system interaction;
- compiler integration;
- canonical IR integration;
- runtime integration;
- scheduling integration;
- portability;
- scalability;
- determinism;
- diagnostics;
- testing;
- compatibility;
- security and safety;
- completion criteria.

This document is not itself the executable grammar.

The executable grammar remains distributed across the existing concurrency grammar files and the canonical grammar composition mechanism established by "grammar/DESIGN.md".

---

2. Authority and ownership

Zamani has one language.

Concurrency must therefore have one semantic specification even though its syntax may be partitioned across multiple files.

The authority relationship is:

grammar/DESIGN.md
        │
        ▼
grammar/spec/concurrency.md
        │
        ├── lexical contract
        │
        ├── syntax contracts
        │
        ├── AST contracts
        │
        ├── semantic contracts
        │
        └── integration contracts
        │
        ▼
grammar/concurrency/*.g4
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
canonical semantic representation / IR
        │
        ├── classical IR
        ├── quantum::ir
        └── HDL/hardware representation
        │
        ▼
optimization
        │
        ├── routing
        ├── scheduling
        └── resilience
        │
        ▼
runtime / HAL / backend

No concurrency grammar file may become a second semantic authority.

---

3. Existing files covered by this specification

The existing concurrency directory contains:

grammar/concurrency/
├── README.md
├── actors.g4
├── cancellation.g4
├── channels.g4
├── concurrency.g4
├── data-parallel.g4
├── futures.g4
├── parallel.g4
├── synchronization.g4
├── task-parallel.g4
└── tasks.g4

These filenames are retained.

No rename is required by this specification.

Their normative ownership is:

File| Owns
"concurrency.g4"| concurrency composition/entry points
"tasks.g4"| task construction and task-oriented concurrency
"futures.g4"| future/awaitable syntax
"actors.g4"| actor syntax
"channels.g4"| channel/message-passing syntax
"synchronization.g4"| explicit synchronization syntax
"parallel.g4"| general parallel intent
"data-parallel.g4"| data-parallel intent
"task-parallel.g4"| task dependency/parallel intent
"cancellation.g4"| cancellation semantics syntax
"README.md"| navigation and implementation guidance
"grammar/spec/concurrency.md"| normative semantic contract

The files must not duplicate one another's ownership.

---

4. Duplicate ANTLR grammar authority

The repository also contains:

grammar/antlr/Concurrency.g4

This is an architectural risk because it can become a second concurrency grammar authority.

It MUST NOT independently define a second Zamani language.

The final architecture must establish exactly one canonical route:

Zamani.g4
   │
   ▼
canonical concurrency grammar
   │
   ├── tasks
   ├── futures
   ├── actors
   ├── channels
   ├── synchronization
   ├── parallel
   ├── data parallel
   ├── task parallel
   └── cancellation

If "grammar/antlr/Concurrency.g4" contains reusable rules that are still required, those rules must be explicitly incorporated into the canonical grammar architecture.

If it is obsolete and unused, it may eventually be removed.

It must never silently remain a competing authority.

No rename is required as part of this specification.

---

5. Definition of concurrency

Concurrency is the semantic property that multiple computations may make progress independently or with explicitly defined coordination relationships.

Concurrency does not mean:

- multiple CPU threads;
- multiple CPU cores;
- multiple operating-system processes;
- multiple machines;
- multiple GPUs;
- multiple accelerators;
- simultaneous physical execution.

Those are possible implementations.

The semantic model is:

logical computation
        │
        ├── dependency
        ├── ordering
        ├── communication
        ├── synchronization
        ├── cancellation
        └── resource requirements

The compiler/runtime determines how the logical computation is realized.

---

6. Fundamental POCO-REAF rule

A concurrency program must not have to be rewritten merely because the execution target changes.

The same source program may be realized using:

one execution resource
many execution resources
CPU
GPU
FPGA
QPU
distributed nodes
heterogeneous resources
remote resources
future computational resources

provided the target can satisfy the program's semantic requirements.

The source describes:

- computation;
- dependencies;
- permitted concurrency;
- required ordering;
- synchronization semantics;
- communication semantics;
- cancellation semantics;
- resource requirements;
- capability requirements;
- correctness constraints.

The compiler/runtime determines:

- placement;
- worker allocation;
- scheduling;
- queue implementation;
- thread implementation;
- device selection;
- topology;
- transport;
- execution strategy.

---

7. No artificial concurrency limits

The language MUST NOT impose universal fixed limits on:

tasks
futures
actors
channels
messages
parallel regions
workers
threads
cores
devices
nodes
queues
executors
execution contexts
data partitions
task dependencies
distributed participants

The grammar MUST NOT define:

MAX_TASKS
MAX_THREADS
MAX_WORKERS
MAX_ACTORS
MAX_CHANNELS
MAX_NODES
MAX_CORES
MAX_DEVICES
MAX_PARALLEL_REGIONS

as language ceilings.

Nor may the compiler implement hidden equivalents.

---

8. Meaning of scalability to infinity

"Infinity" is an architectural scalability requirement, not a claim that physical computers possess infinite resources.

The correct model is:

language limit
    =
no artificial finite concurrency ceiling

actual execution limit
    =
available resources + representation constraints + semantic constraints

Therefore:

task { ... }

and a program containing a dynamically determined number of tasks are both valid language concepts.

Whether a particular target can realize the requested concurrency is a separate resource-feasibility question.

A target may:

- execute concurrently;
- serialize independent work;
- distribute it;
- batch it;
- stream it;
- checkpoint it;
- defer it;
- reject the target because resources are insufficient.

It must not silently change program semantics.

---

9. Concurrency dimensions

Zamani MUST distinguish at least the following dimensions:

asynchrony
task concurrency
data parallelism
task parallelism
actor concurrency
message passing
structured concurrency
distributed concurrency
heterogeneous concurrency
reactive/event concurrency
pipeline concurrency
speculative concurrency
deterministic concurrency

These dimensions may compose.

For example:

distributed
    +
task parallel
    +
data parallel
    +
asynchronous

is valid as a semantic combination.

No single concurrency model is assumed to be universal.

---

10. Concurrency is not scheduling

The grammar expresses:

may execute concurrently
must execute before
must complete before
may overlap
must synchronize
may be cancelled
depends on
communicates with

The scheduler determines:

when
where
on which execution resource
with which priority
using which queue
with which placement

The grammar MUST NOT contain scheduler algorithms.

Examples of downstream scheduling strategies include:

- FIFO;
- priority scheduling;
- work stealing;
- dependency scheduling;
- static scheduling;
- dynamic scheduling;
- distributed scheduling;
- accelerator scheduling;
- quantum scheduling.

These are implementation policies.

---

11. Concurrency is not hardware topology

The concurrency grammar MUST NOT encode:

CPU topology
GPU topology
FPGA topology
QPU topology
NUMA topology
network topology
cluster topology
accelerator topology

Topology belongs to target/resource/hardware layers.

A source-level statement such as:

parallel {
    compute_a()
    compute_b()
}

must not imply:

core 0 → compute_a
core 1 → compute_b

---

12. Requirement/capability/preference distinction

Concurrency specifications MUST preserve the distinction between:

12.1 Requirement

A semantic condition that must hold.

requires capability("parallel_execution")

12.2 Capability

Something a target can provide.

capability("distributed_execution")

12.3 Constraint

A property that must remain within a bound.

requires latency <= budget

12.4 Preference

A non-mandatory optimization preference.

prefer locality

12.5 Hint

Information that may improve compilation without changing semantics.

hint independent

12.6 Realization

A downstream mapping decision.

logical_task -> physical_execution_resource

The source language must not confuse these categories.

---

13. Task semantics

A task represents a logical unit of computation whose execution may be independently scheduled subject to its dependencies and semantic constraints.

A task may have:

- inputs;
- outputs;
- captures;
- effects;
- resource requirements;
- capability requirements;
- cancellation state;
- completion state;
- dependencies.

A task does not inherently represent:

- an OS thread;
- a CPU core;
- an OS process;
- a GPU stream;
- a QPU;
- a network node.

---

14. Task identity

Task identity is logical.

The language may provide handles, references, futures, task IDs, or structured task scopes.

However, a logical task identifier MUST NOT be interpreted as:

CPU ID
thread ID
process ID
machine ID
device ID

unless an explicitly target-specific construct has been introduced downstream.

---

15. Task dependencies

Tasks may have dependency relationships.

Conceptually:

A
│
├── B
└── C

B ──► D
C ──► D

The compiler must preserve these relationships.

A scheduler may execute "B" and "C" concurrently if their semantic requirements permit it.

"D" cannot execute before all required predecessors have satisfied their completion conditions.

---

16. Dynamic task creation

Task counts must not need to be known statically.

A program may derive task creation from:

- runtime input;
- collection size;
- stream size;
- dataset size;
- distributed state;
- quantum measurement results;
- hardware capability;
- external events.

The language must therefore support logically unbounded task creation subject to runtime resources.

No parser-level maximum may exist.

---

17. Futures and awaitables

A future/awaitable represents a computation whose result may become available later.

The semantic contract is:

Future<T>
    =
eventual availability of a value or failure of type T

A future is not inherently:

- a thread;
- an OS event;
- a network request;
- an executor job;
- a particular runtime object.

The implementation is free to choose the appropriate mechanism.

---

18. Await semantics

"await" semantically requests the result of an awaitable computation.

It does not require:

- blocking an OS thread;
- suspending a physical processor;
- using an event loop;
- using a particular executor.

The implementation may suspend the logical computation and release its physical execution resource.

---

19. Async semantics

An asynchronous function represents computation that may suspend while retaining its logical continuation.

The implementation may use:

- stackless coroutines;
- stackful coroutines;
- fibers;
- state machines;
- event loops;
- work queues;
- runtime tasks;
- other safe mechanisms.

The source semantics remain independent of implementation.

---

20. Structured concurrency

Structured concurrency is the preferred semantic model for parent/child task relationships.

A structured scope establishes:

parent lifetime
      │
      ├── child task
      ├── child task
      └── child task

The scope defines the relationship between:

- creation;
- completion;
- failure;
- cancellation;
- cleanup;
- result propagation.

A child task MUST NOT silently outlive its owning structured scope unless the language explicitly provides a separate detached-concurrency construct with a complete lifetime contract.

---

21. Detached concurrency

Detached/background computation may be supported only with an explicit semantic lifetime model.

A detached computation must define:

- ownership;
- lifetime;
- error handling;
- cancellation;
- resource retention;
- shutdown behavior;
- observation;
- failure reporting.

The grammar must not provide a casual syntax that silently creates immortal background work.

---

22. Actor model

Actors are logical concurrent entities that:

- own isolated state;
- receive messages;
- process messages;
- may produce messages;
- may create further actors;
- may terminate.

An actor is not synonymous with:

OS process
OS thread
CPU core
network node

Actor placement is downstream.

The existing actor language-specification implementation must be reconciled with this specification rather than defining an incompatible actor model.

---

23. Actor state isolation

Actor state must be protected by the language's canonical memory/type/effect model.

An actor MUST NOT silently permit arbitrary shared mutable state.

If shared state is permitted, its semantics must be explicitly represented through:

- synchronization;
- ownership transfer;
- immutable sharing;
- channels;
- capabilities;
- transactional semantics;
- another formally specified mechanism.

---

24. Message passing

Message passing is a semantic communication mechanism.

Messages may be:

- values;
- references where safe;
- immutable structures;
- ownership-transferred values;
- serialized values;
- domain-specific messages.

Message transport is implementation-specific.

The same source-level message interaction may be realized:

in-process
cross-thread
cross-device
cross-process
cross-node
cross-network

without changing the source semantics.

---

25. Channels

A channel provides an ordered or otherwise explicitly specified communication mechanism between producers and consumers.

A channel type must define:

- message type;
- send semantics;
- receive semantics;
- closure semantics;
- failure semantics;
- cancellation interaction;
- ordering semantics;
- capacity semantics where capacity is semantically observable.

Channel capacity MUST NOT be silently interpreted as a universal machine constant.

---

26. Unbounded versus bounded channels

The language may distinguish:

unbounded logical channel
bounded logical channel

where the distinction has semantic consequences.

A bounded channel must define the behavior when capacity is exhausted, such as:

- suspension;
- backpressure;
- failure;
- cancellation;
- explicit rejection.

The implementation may realize a logically unbounded channel using segmented storage, spilling, remote storage, or another mechanism.

No fixed universal capacity is permitted.

---

27. Channel ownership

Channel endpoints must integrate with the canonical ownership/type system.

Closing an endpoint must have specified semantics.

Dropping a sender/receiver must not create unspecified behavior.

The grammar itself does not own ownership rules.

Ownership belongs to:

grammar/spec/type-system.md
memory model
semantic analyzer

---

28. Select / multiplexing

A select-like construct may wait for one of several communication or asynchronous events.

Its semantics must define:

- eligible operations;
- readiness;
- fairness;
- deterministic selection requirements;
- cancellation;
- timeout/deadline behavior;
- no-ready-case behavior.

A runtime must not accidentally make program semantics depend on hash-map order, memory address, thread timing, or other unspecified implementation artifacts.

---

29. Synchronization

Synchronization establishes an explicit ordering or coordination relationship.

Possible semantic categories include:

join
wait
barrier
latch
notification
condition
atomic coordination
phase synchronization
collective synchronization

The grammar describes the operation.

It does not define the implementation primitive.

---

30. Synchronization versus mutual exclusion

These concepts must remain distinct.

Mutual exclusion means:

at most one eligible operation enters a critical region at a time

Synchronization may instead mean:

A must complete before B

or:

all participants must reach phase P

or:

wait until condition C

The language must not collapse all synchronization into a generic mutex abstraction.

---

31. Memory-model integration

Concurrency is inseparable from memory semantics.

The concurrency specification therefore depends on the canonical memory/type system for:

- ownership;
- borrowing;
- aliasing;
- mutation;
- lifetimes;
- shared state;
- immutable state;
- atomic state;
- synchronization;
- data races.

Concurrency grammar must not introduce a second ownership model.

---

32. Data races

The language implementation must define whether a particular shared-memory operation is:

- statically prohibited;
- dynamically checked;
- explicitly synchronized;
- atomic;
- immutable;
- otherwise semantically safe.

The grammar alone cannot guarantee race freedom.

A syntactically valid concurrent program may still fail semantic analysis.

---

33. Effect integration

Concurrency constructs may produce effects including:

async
concurrent
parallel
communication
synchronization
cancellation
distributed
remote
nondeterministic

The effect system owns the representation of these effects.

Concurrency syntax merely identifies their source.

A concurrency construct must therefore map to the existing effect model rather than defining an independent effect system.

---

34. Deterministic concurrency

Zamani must support deterministic concurrent programs.

Determinism means that, under the specified semantic inputs and permitted execution model, observable results do not depend on arbitrary scheduling choices.

The language must distinguish:

deterministic
nondeterministic
schedule-dependent
externally nondeterministic

where required.

The compiler must not accidentally introduce nondeterminism into a construct specified as deterministic.

---

35. Explicit nondeterminism

If a construct intentionally permits nondeterministic selection, that must be part of its semantic contract.

For example, selecting one ready communication operation may be:

unspecified
fair
randomized
priority ordered
source ordered

The choice must be explicitly specified.

"Whatever the runtime happens to do" is not a sufficient language semantic.

---

36. Fairness

Fairness is not automatically guaranteed.

A construct requiring fairness must explicitly state:

- fairness domain;
- fairness condition;
- progress guarantee;
- cancellation behavior.

A runtime may implement stronger fairness than required, but it must not violate required fairness.

---

37. Progress and liveness

The language must distinguish:

safety
liveness
termination
progress
fairness
deadlock freedom
starvation freedom

A syntax construct must not claim stronger guarantees than its semantic analysis can establish.

For example:

parallel

does not automatically mean:

deadlock free

or:

always concurrently executed

---

38. Deadlock semantics

The compiler may detect statically provable deadlocks.

Runtime systems may detect certain dynamic deadlocks.

The language must define diagnostics for known violations.

However, not every deadlock can necessarily be detected statically in a general-purpose language.

The specification must not make an impossible static guarantee.

---

39. Cancellation

Cancellation is a semantic request to stop eligible computation.

Cancellation is cooperative unless a stronger semantic guarantee is explicitly defined.

Cancellation MUST NOT automatically mean:

kill OS thread
kill process
send SIGTERM
reset device
destroy QPU job
terminate network connection

Those are implementation mechanisms.

---

40. Cancellation propagation

Structured concurrency must define cancellation propagation.

Possible relationships include:

parent cancelled
    ↓
children become cancellation-requested

and:

child failure
    ↓
scope cancellation
    ↓
remaining children cancelled

The exact behavior must be part of the construct's semantic contract.

---

41. Cancellation safety

A cancellable operation must define its cancellation points or cancellation safety.

Operations that hold resources must not be left in an invalid semantic state.

Cancellation must integrate with:

- ownership;
- transactions;
- locks;
- channels;
- I/O;
- distributed operations;
- quantum operations;
- hardware operations.

---

42. Timeouts and deadlines

Timeouts and deadlines are semantic temporal constraints.

A timeout must not be confused with a particular OS timer.

A deadline may be represented abstractly and propagated through nested computations.

The runtime may realize deadlines using:

- monotonic clocks;
- distributed clock models;
- hardware timers;
- scheduler timers.

The grammar must remain target-independent.

---

43. Resource-aware concurrency

Concurrency must integrate with:

grammar/resources/
grammar/hardware/
grammar/compile/
grammar/execution/

The program may express:

requires capability(...)
requires resource(...)
requires latency(...)
requires throughput(...)
requires memory(...)
prefer locality
prefer parallelism

but physical realization remains downstream.

---

44. Resource elasticity

A concurrent program may scale with available resources.

For example:

parallel for item in data {
    process(item)
}

does not mean:

one OS thread per item

The implementation may use:

one worker
many workers
dynamic work stealing
GPU kernels
distributed workers
streaming execution

provided semantics are preserved.

---

45. Work decomposition

The language may expose logical work decomposition.

The compiler may then choose:

chunking
tiling
partitioning
vectorization
batching
fusion
fission
distribution
streaming

These transformations belong to optimization/lowering, not grammar semantics.

---

46. Data parallelism

Data parallelism means applying compatible computation across a logical collection or index domain.

It must remain independent of:

SIMD width
vector register width
GPU warp width
GPU block size
GPU count
accelerator count

Those are target properties.

A data-parallel program may execute on a scalar CPU or a massively parallel accelerator.

---

47. Task parallelism

Task parallelism represents multiple logical tasks with potentially different computations.

The language may express:

task A
task B
task C
A -> D
B -> D
C -> D

The scheduler determines how those tasks are realized.

No worker count is implied.

---

48. General parallel blocks

A parallel block means that contained computations are eligible for concurrent execution subject to semantic dependencies.

It does not promise physical simultaneity.

Therefore:

parallel {
    A()
    B()
}

means:

A and B may overlap if permitted

not:

A and B must occupy two physical cores

---

49. Nested parallelism

Parallel constructs may be nested.

The language must not impose a fixed nesting depth.

The compiler may flatten, fuse, serialize, distribute, or otherwise transform nested parallel regions when semantics permit.

---

50. Dynamic parallelism

Parallel work may be generated dynamically.

Examples include:

- recursive divide-and-conquer;
- dynamic graph traversal;
- adaptive numerical methods;
- data-dependent workloads;
- dynamic quantum-classical control;
- distributed discovery;
- AI agent spawning.

No static task-count assumption is permitted.

---

51. Pipeline concurrency

Pipelines may represent stages such as:

input
  ↓
stage A
  ↓
stage B
  ↓
stage C

Multiple items may be in different stages concurrently.

The implementation may choose:

- buffering;
- streaming;
- batching;
- fusion;
- hardware pipelines;
- distributed stages.

The grammar expresses the semantic pipeline relationship only.

---

52. Reactive/event concurrency

Event-driven constructs may represent:

event
handler
subscription
notification
stream
reaction

The event runtime determines dispatch mechanisms.

The language must not require:

one event = one thread

---

53. Speculative concurrency

Speculative computation may be supported when explicitly specified.

A speculative computation must define:

- what is speculative;
- what commits;
- what rolls back;
- what observations are allowed;
- how side effects are controlled;
- cancellation behavior;
- determinism requirements.

Speculation must integrate with effects and transactions rather than bypassing them.

---

54. Distributed concurrency

Concurrency and distribution are separate dimensions.

A computation may be:

concurrent + local

or:

concurrent + distributed

or:

concurrent + heterogeneous

Distributed semantics belong to:

grammar/distributed/

Concurrency provides the logical computation relationships.

Distribution determines:

- node placement;
- replication;
- communication;
- consistency;
- remote execution;
- failure domains.

---

55. Network communication

A channel may eventually use:

shared memory
local queue
IPC
RDMA
network transport
future communication substrate

The source-level channel semantics remain independent of transport.

Networking grammar owns protocol/address/endpoint semantics.

Concurrency grammar owns the logical communication relationship.

---

56. Actor distribution

An actor may execute:

locally
remotely
on another process
on another machine
on an accelerator

without changing its language-level identity.

The distributed subsystem determines placement.

---

57. Heterogeneous concurrency

Concurrency may combine:

CPU
GPU
FPGA
QPU
specialized accelerator
network resource
future computational substrate

The grammar must not encode vendor-specific execution assumptions.

Hardware/resource capabilities determine feasible realization.

---

58. Quantum-classical concurrency

Quantum and classical computations may participate in one concurrency graph.

Example semantic flow:

classical preparation
        ↓
quantum computation
        ↓
measurement
        ↓
classical analysis
        ↓
quantum computation

Concurrency may also coordinate independent quantum computations.

However, concurrency grammar does NOT own:

- qubits;
- logical qubits;
- physical qubits;
- quantum gates;
- circuits;
- quantum operations;
- QEC;
- ZQN;
- quantum routing;
- quantum scheduling;
- quantum IR.

The canonical quantum semantic boundary remains:

quantum syntax
    ↓
semantic quantum representation
    ↓
quantum::ir

Concurrency metadata may be attached to the semantic operation graph without creating a second quantum IR.

---

59. Quantum resource scalability

The concurrency grammar MUST NOT contain:

MAX_QUBITS
MAX_QUANTUM_TASKS
MAX_QPU_WORKERS
MAX_QPU_DEVICES

Quantum resource feasibility belongs to the quantum/resource/HAL pipeline.

A program requiring many independent quantum computations remains valid even if a particular QPU cannot currently execute all of them concurrently.

The compiler may:

- serialize;
- batch;
- distribute;
- queue;
- simulate;
- map to multiple devices;
- use logical resources;
- reject a target with a resource diagnostic.

---

60. HDL and hardware concurrency

HDL concurrency has different semantics from software concurrency.

Examples include:

concurrent signal assignment
hardware processes
clocked processes
pipeline stages
state machines

These belong to:

grammar/hdl/
grammar/hardware/

The concurrency specification must not redefine them.

Shared concepts such as dependencies and parallel intent may be represented in the canonical semantic model.

---

61. Clock semantics

A hardware clock is not equivalent to a software scheduler.

Clock syntax belongs to HDL/hardware semantics.

Concurrency may describe logical relationships between hardware computations, but:

clock
cycle
frequency
phase
timing

are owned by hardware/HDL specifications.

---

62. AI/agent concurrency

AI agents may be concurrent.

Actor-like agents, asynchronous inference, distributed training, data-parallel training, and agent communication may use the concurrency model.

The AI subsystem owns:

- model semantics;
- tensor semantics;
- training semantics;
- inference semantics;
- model capabilities.

Concurrency owns the execution relationships.

---

63. Effects of concurrency on types

Potential concurrency-aware types include:

Future<T>
Task<T>
Channel<T>
Sender<T>
Receiver<T>
ActorHandle<T>

These are semantic types, not necessarily runtime implementation types.

Their final type-system definitions belong to:

grammar/spec/type-system.md
grammar/types/

This specification defines only their concurrency meaning.

---

64. Ownership and Sendability

Where the type system requires it, values crossing concurrency boundaries must satisfy the appropriate transfer/share contract.

Possible categories include:

movable
shareable
immutable
synchronized
atomic
non-sendable

The exact terminology belongs to the canonical type/memory system.

The concurrency analyzer consumes those properties.

It does not define another ownership system.

---

65. Capability-based concurrency

Concurrency operations may require capabilities such as:

async_execution
parallel_execution
message_passing
distributed_execution
cancellation
synchronization
remote_execution
accelerator_execution
quantum_execution

Capabilities describe target/environment properties.

They do not identify specific physical devices.

---

66. Semantic resource requirements

A program may require a semantic amount of:

compute
memory
communication
bandwidth
latency
energy
reliability
parallelism
quantum capability
accelerator capability

Resource quantities must remain representable without compiler-imposed universal ceilings.

The grammar may represent a value such as:

required_parallelism = n

without defining a maximum "n".

---

67. Resource adaptation

The compiler may adapt physical realization to available resources.

For example:

logical parallelism = 100000
available workers = 32

may result in batched execution.

The compiler must preserve semantic behavior.

The source must not need to be rewritten merely because physical parallelism changes.

---

68. Backpressure

Streaming/channel/pipeline concurrency may require backpressure.

Backpressure is semantic when it affects observable behavior.

The specification must distinguish:

producer may continue
producer must wait
producer may drop
producer must fail
producer may spill

The implementation determines how that behavior is realized.

---

69. Failure semantics

Concurrent operations may fail independently.

The language must specify:

- failure propagation;
- sibling cancellation;
- parent cancellation;
- result collection;
- error aggregation;
- partial completion;
- cleanup.

Failure semantics must be compatible with structured concurrency.

---

70. Error aggregation

When multiple concurrent children fail, the semantic model must not arbitrarily discard errors unless the construct explicitly specifies that behavior.

Possible semantic models include:

first failure
all failures
primary failure + suppressed failures
aggregated failure
custom error policy

The selected model must be defined by the relevant construct.

---

71. Exception/panic integration

Concurrency must integrate with the canonical error model.

A task failure must not silently become:

process crash

unless the program/runtime contract explicitly specifies that outcome.

The language must define what happens when:

await task

observes a failed task.

---

72. Resource cleanup

Concurrency constructs must preserve resource cleanup semantics.

This includes:

- memory;
- file handles;
- channels;
- device handles;
- network resources;
- accelerator resources;
- quantum resources;
- locks;
- transactional state.

Cancellation and failure must not create unspecified resource leaks.

---

73. Compiler pipeline

The production compiler integration is:

Zamani source
    ↓
lexer
    ↓
parser
    ↓
frontend AST
    ↓
name resolution
    ↓
type analysis
    ↓
effect analysis
    ↓
ownership/memory analysis
    ↓
concurrency analysis
    ↓
resource/capability analysis
    ↓
semantic validation
    ↓
canonical semantic model
    ↓
canonical IR
    ↓
optimization
    ↓
placement/routing
    ↓
scheduling
    ↓
resilience
    ↓
target lowering
    ↓
runtime/backend

The grammar is complete only when its constructs have a defined destination through this pipeline.

---

74. AST contract

Every concurrency syntax construct must map to a domain-neutral AST representation.

The AST must preserve at least:

construct kind
source span
body
operands
dependencies
attributes
modifiers
nested expressions/statements

where applicable.

The AST must not require:

physical thread ID
physical core ID
GPU ID
QPU ID
machine ID
node ID
worker ID

for ordinary portable concurrency.

---

75. Generic operation model

Concurrency operations should fit the repository's generic operation architecture where appropriate.

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
    source
}

Concurrency-specific semantics may be attached through typed attributes/effects rather than a giant enumeration of every possible runtime primitive.

---

76. Canonical IR integration

Concurrency must not create a second independent universal IR.

Concurrency information should be represented in the canonical semantic/IR layer using concepts such as:

task
dependency
region
effect
communication
synchronization
resource requirement
capability requirement
cancellation
ordering

The exact concrete IR type belongs to the existing IR architecture.

The grammar must not dictate an implementation-specific IR layout.

---

77. Quantum IR boundary

Quantum concurrency must lower into the existing "quantum::ir".

It must not create:

concurrency::quantum_ir
quantum_concurrency_ir
frontend_quantum_ir

as competing semantic representations.

Concurrency describes relationships between computations.

"quantum::ir" describes quantum computation semantics.

---

78. Optimization integration

Concurrency-aware optimization may perform:

fusion
fission
task elimination
task coalescing
dependency simplification
parallel region transformation
pipeline transformation
vectorization
batching
tiling
work decomposition
communication reduction

provided observable semantics are preserved.

The grammar does not own these transformations.

---

79. Scheduling integration

Scheduling consumes:

logical operations
dependencies
resource requirements
capabilities
timing constraints
effect constraints
memory constraints
communication constraints
target capabilities

It determines physical execution order and placement.

A parallel source construct does not require the scheduler to allocate a matching number of workers.

---

80. Runtime integration

The repository already contains a runtime concurrency subsystem.

Runtime implementation may use:

tasks
futures
coroutines
workers
queues
event loops
threads
distributed workers
accelerator execution

The grammar MUST NOT depend on a particular implementation.

The dependency direction is:

grammar
    ↓
AST
    ↓
semantic model
    ↓
IR
    ↓
compiler
    ↓
runtime

Never:

grammar
    ↓
runtime implementation

---

81. Standard-library integration

The repository also contains concurrency-related standard-library functionality.

The language grammar must not duplicate standard-library APIs.

Distinction:

language construct
    =
syntax + semantic meaning

library API
    =
reusable abstraction implemented using the language/runtime

A library function must not become a keyword merely because it performs concurrency.

---

82. Safe Rust implementation requirement

All compiler and grammar tooling implementing this specification MUST use safe Rust.

The following are prohibited in production implementation:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

This includes:

- lexer;
- parser;
- AST;
- semantic analyzer;
- concurrency analysis;
- grammar validation;
- compiler integration;
- runtime integration code maintained by this project.

The specification itself does not require Rust syntax.

---

83. Rust 1.97 / 1.97.1 compatibility

The implementation must compile against:

Rust 1.97
Rust 1.97.1
Edition 2021

Concurrency grammar design must not require newer Rust language features merely to represent the specification.

Repository dependency versions remain governed by the repository's existing package metadata and compatibility policy.

---

84. No runtime-sized parser limits

Parser implementation must avoid turning ordinary concurrency cardinality into fixed parser constants.

The grammar may naturally encounter implementation limits such as:

- input size;
- recursion depth;
- memory exhaustion;
- integer representation;
- operating-system resource limits.

Those are implementation/resource failures, not language-level semantic limits.

They must be diagnosed accurately.

---

85. No recursion-only scalability assumption

Concurrency constructs may be nested deeply.

The implementation must not assume that all valid concurrency structures fit into a small fixed nesting depth.

Where practical, compiler implementation should use iterative/data-structure approaches for potentially unbounded graphs rather than unnecessary recursion that creates artificial stack limits.

---

86. Graph scalability

The semantic concurrency model should be able to represent large dependency graphs.

The implementation must not assume:

small number of tasks
small number of edges
small number of actors
small number of channels

Graph algorithms should be selected with scalability in mind.

Memory exhaustion is an implementation/resource condition, not a semantic grammar ceiling.

---

87. Deterministic compilation

Given the same:

source
compiler version
language version
dependency graph
compilation configuration
target description
resource policy

the compiler should produce deterministic semantic results unless nondeterminism is explicitly part of the compilation process.

Concurrency analysis must not depend on:

- hash iteration order;
- thread scheduling;
- pointer addresses;
- unspecified filesystem ordering;
- random process timing.

---

88. Runtime determinism

If a program is specified as deterministic, runtime execution must not expose accidental scheduler nondeterminism.

Examples include:

- unordered message delivery when ordering is required;
- arbitrary reduction ordering where floating-point semantics require a defined result;
- race-dependent mutation;
- unstable task selection.

Where nondeterminism is intentionally permitted, it must be documented.

---

89. Floating-point parallelism

Parallel floating-point reductions may produce different rounding if operation order changes.

Therefore the language must distinguish:

mathematically associative operation

from:

machine-level deterministic reduction

A deterministic numeric operation must specify an appropriate reduction order or deterministic semantic contract.

The compiler must not claim bitwise determinism merely because an operation is mathematically associative.

---

90. Atomic semantics

Atomic operations belong to the canonical memory/concurrency semantic model.

The grammar may identify atomic constructs.

It does not define hardware instructions such as:

x86 lock
ARM LDAXR
GPU atomic instruction

Those belong to backend lowering.

---

91. Lock-free and wait-free claims

Claims such as:

lock-free
wait-free
obstruction-free

must be treated as semantic/performance contracts only when formally specified.

A source annotation must not merely be documentation pretending to provide a guarantee.

The compiler/backend must either:

- verify the guarantee;
- preserve an established guarantee;
- or issue a diagnostic if the target cannot satisfy it.

---

92. Priority

Priority is not inherently semantic.

A priority construct may be supported when a program genuinely requires priority ordering.

Otherwise:

priority = implementation hint

must remain distinct from:

priority = correctness requirement

The grammar must preserve that distinction.

---

93. Affinity

Physical affinity belongs to target realization.

Portable source should generally express:

prefer locality
prefer data affinity
prefer communication locality

rather than:

run_on_core(7)

A target-specific dialect may provide explicit affinity, but it must be isolated and must not contaminate the portable concurrency model.

---

94. Accelerator concurrency

Accelerator work may be concurrent with host work.

The language may represent:

submit computation
await computation
synchronize

without assuming:

GPU 0
stream 3
device 2

The hardware/resource subsystem resolves the physical accelerator.

---

95. Distributed failure

Distributed concurrency must account for:

node failure
network failure
partition
message loss
timeout
duplicate delivery
reordering
resource exhaustion

These semantics belong jointly to concurrency, distributed, networking, and resilience specifications.

Concurrency must not silently assume a perfect network.

---

96. Structured distributed concurrency

A distributed structured scope must define:

- ownership;
- child lifetime;
- cancellation;
- failure propagation;
- communication;
- recovery;
- completion.

The implementation may place children across different machines.

The source semantics remain logical.

---

97. Resilience integration

Concurrency interacts with the existing resilience architecture.

Resilience may:

retry
recover
restart
reassign
quarantine
escalate

but must not silently change semantic results.

For quantum computation, resilience remains integrated through:

QEC
ZQN
HAL
routing
scheduling

rather than being implemented in the concurrency grammar.

---

98. Observability

Concurrent programs may expose:

tracing
metrics
profiling
task state
dependency state
resource usage

Observability must not alter program semantics unless explicitly specified.

Instrumentation must not introduce races into otherwise valid programs.

---

99. Debugging semantics

The implementation should support inspection of:

logical task graph
dependencies
task states
channel states
cancellation state
synchronization relationships
resource requirements

without requiring the programmer to reason in terms of physical worker IDs.

Physical execution details may be provided as diagnostics.

---

100. Security

Concurrency must not bypass:

- capability checks;
- ownership;
- authorization;
- isolation;
- secret handling;
- sandboxing.

A concurrent child must inherit only the capabilities explicitly permitted by the semantic model.

Detached concurrency must not become a capability-escalation mechanism.

---

101. Data confidentiality

Message passing must respect data ownership and security classification.

A compiler/runtime must not automatically move protected data to:

remote node
GPU
QPU
network
external service

unless the semantic/security model permits that movement.

---

102. Concurrency and cryptography

Cryptographic operations may be parallelized.

The concurrency model must not assume that cryptographic operations are interchangeable with ordinary computations.

Security-sensitive operations must retain their required ordering, constant-time, isolation, or side-channel properties where specified.

---

103. Concurrency and persistence

Persistent/temporal systems such as Sankofa may participate in concurrent computation.

Concurrency must define:

- synchronization;
- consistency;
- conflict handling;
- transaction semantics

through the relevant persistence/data specifications.

Concurrency itself must not invent a separate persistence model.

---

104. Concurrency and temporal computation

Temporal/MTS computations may execute concurrently.

If temporal semantics require:

ordered observation
snapshot consistency
causal ordering
timeline isolation

those requirements must be represented explicitly.

Concurrency must not assume ordinary wall-clock time is sufficient.

---

105. Concurrency and metaprogramming

Macros and compile-time execution may themselves be parallelized internally.

However, source-level compile-time evaluation must preserve the language's deterministic compilation contract where required.

Runtime concurrency constructs must not accidentally execute during compilation.

---

106. Concurrency and macros

Macros may generate concurrency syntax.

After expansion, generated constructs must undergo the same:

parsing
structural validation
type checking
effect checking
ownership checking
resource analysis
concurrency analysis

as ordinary source.

Macros must not bypass concurrency safety checks.

---

107. Concurrency and dialects

Dialect-specific concurrency may exist.

A dialect must declare:

dialect identity
version
syntax additions
semantic additions
AST mapping
IR mapping
capability requirements
compatibility

A dialect must not silently redefine core concurrency semantics.

---

108. Interoperability

Foreign concurrency models may be represented through interoperability layers.

Examples:

C/C++ threading
Rust async
OpenMP
MPI
GPU execution models
OpenCL
CUDA
HIP
SYCL
OpenQASM/QIR-associated execution
HDL simulation

These are interoperability targets or dialects.

They are not automatically the Zamani concurrency semantics.

---

109. Vendor independence

The portable concurrency specification MUST NOT contain vendor-specific assumptions.

Examples of forbidden core semantics:

CUDA block
ROCm wavefront
Intel thread
NVIDIA stream
TPU core
specific QPU worker
specific FPGA fabric

Such concepts belong in target-specific interoperability/dialect layers.

---

110. Compiler target adaptation

A compiler may lower the same logical concurrency graph differently for different targets.

For example:

CPU:
task graph → thread/task scheduler

GPU:
data parallel graph → kernels

FPGA:
parallel graph → hardware pipeline

distributed:
task graph → distributed execution

QPU:
quantum/classical dependency graph → quantum scheduling

The source program remains unchanged.

---

111. Compilation once versus execution many times

POCO-REAF does not require a single binary to contain every possible backend.

It requires that source semantics remain portable and target realization remain downstream.

A compiled artifact may contain:

- portable IR;
- multiple target variants;
- target-independent metadata;
- capability requirements;
- resource requirements;
- provenance.

The exact packaging model belongs to the compiler/deployment specifications.

---

112. Semantic preservation

Every concurrency lowering must preserve:

observable values
observable effects
required ordering
required synchronization
communication semantics
failure semantics
cancellation semantics
resource constraints
security constraints
determinism requirements

Optimization is invalid if it changes these without an explicitly permitted semantic relaxation.

---

113. Semantic relaxation

If Zamani later supports explicit relaxed semantics, such as:

eventually consistent
best effort
approximate
nondeterministic
relaxed ordering

those must be explicit program semantics.

The compiler must never introduce semantic relaxation merely because the target is constrained.

---

114. Completion of "tasks.g4"

"tasks.g4" is complete only when:

- task syntax is defined;
- task body semantics are defined;
- task inputs/outputs are defined;
- task dependencies are representable;
- task lifetime is defined;
- task failure is defined;
- task cancellation is defined;
- task ownership is defined;
- task AST mapping is defined;
- semantic mapping is defined;
- IR mapping is defined;
- no physical worker assumptions exist;
- no fixed task count exists;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- compatibility tests exist.

---

115. Completion of "futures.g4"

"futures.g4" is complete only when:

- future creation semantics are defined;
- result semantics are defined;
- failure semantics are defined;
- await semantics are defined;
- cancellation interaction is defined;
- ownership is defined;
- AST mapping is defined;
- type mapping is defined;
- effect mapping is defined;
- IR mapping is defined;
- no executor implementation is assumed;
- no thread implementation is assumed;
- tests cover nested futures and large dependency graphs.

---

116. Completion of "actors.g4"

"actors.g4" is complete only when:

- actor declaration is defined;
- actor state is defined;
- message handling is defined;
- actor lifecycle is defined;
- actor failure is defined;
- actor cancellation is defined;
- actor ownership is defined;
- actor message typing is defined;
- distribution interaction is defined;
- AST mapping is defined;
- semantic mapping is defined;
- IR mapping is defined;
- no OS process assumption exists;
- no fixed actor count exists.

---

117. Completion of "channels.g4"

"channels.g4" is complete only when:

- channel creation is defined;
- sender/receiver semantics are defined;
- message typing is defined;
- ordering is defined;
- closure is defined;
- capacity semantics are defined;
- backpressure is defined;
- cancellation is defined;
- select integration is defined;
- ownership is defined;
- AST/semantic/IR mappings exist;
- no fixed channel capacity is imposed.

---

118. Completion of "synchronization.g4"

It is complete only when:

- every synchronization construct has a semantic definition;
- ordering guarantees are defined;
- memory visibility is defined through the memory model;
- cancellation interaction is defined;
- failure behavior is defined;
- AST/semantic/IR mappings exist;
- hardware lock implementations are not encoded in syntax.

---

119. Completion of "parallel.g4"

It is complete only when:

- parallel-region semantics are defined;
- dependency behavior is defined;
- nested parallelism is defined;
- deterministic behavior is defined;
- resource scaling is defined;
- AST mapping exists;
- IR mapping exists;
- no worker count is hard-coded;
- no physical topology is assumed.

---

120. Completion of "data-parallel.g4"

It is complete only when:

- logical data-domain semantics are defined;
- iteration semantics are defined;
- reduction semantics are defined;
- ordering is defined;
- determinism is defined;
- mutation restrictions are defined;
- vectorization is downstream;
- GPU mapping is downstream;
- no fixed vector width is encoded.

---

121. Completion of "task-parallel.g4"

It is complete only when:

- logical tasks are representable;
- dependencies are representable;
- task groups are representable;
- joining is defined;
- failure is defined;
- cancellation is defined;
- dynamic task creation is supported;
- no worker count is encoded.

---

122. Completion of "cancellation.g4"

It is complete only when:

- cancellation request is defined;
- cancellation propagation is defined;
- cancellation points are defined;
- cleanup semantics are defined;
- resource ownership is preserved;
- task/future/channel integration exists;
- distributed cancellation behavior is defined;
- quantum/hardware cancellation is delegated correctly;
- no OS-specific termination is encoded.

---

123. Completion of "concurrency.g4"

"concurrency.g4" is complete only when it:

- exposes canonical concurrency entry points;
- composes all dedicated concurrency domains;
- introduces no duplicate rules;
- introduces no second type system;
- introduces no second effect system;
- introduces no second resource model;
- introduces no runtime implementation;
- introduces no hardware assumptions;
- has explicit imports/composition;
- has parser integration;
- has AST integration;
- has semantic integration;
- has IR integration;
- has complete test coverage.

---

124. Cross-file integration contract

Every concurrency grammar file MUST declare its dependencies before implementation.

The dependency graph is:

core
 ├── expressions
 ├── types
 ├── declarations
 ├── statements
 └── effects
       │
       ▼
concurrency composition
       │
       ├── tasks
       ├── futures
       ├── actors
       ├── channels
       ├── synchronization
       ├── parallel
       ├── data-parallel
       ├── task-parallel
       └── cancellation
       │
       ▼
semantic analysis
       │
       ├── memory
       ├── type system
       ├── effects
       ├── resources
       └── capabilities
       │
       ▼
canonical IR
       │
       ├── classical
       ├── quantum::ir
       └── HDL/hardware
       │
       ▼
optimization
       │
       ├── routing
       ├── scheduling
       └── resilience
       │
       ▼
runtime / HAL / backend

No file may assume a dependency that is not documented here or in the relevant authoritative specification.

---

125. Upstream contracts

Concurrency depends on:

grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/spec/type-system.md
grammar/spec/semantics.md
grammar/spec/effects.md
grammar/spec/resources.md
grammar/spec/portability.md

It also depends on:

grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/memory/
grammar/effects/
grammar/resources/
grammar/compile/
grammar/execution/
grammar/distributed/
grammar/hardware/

These contracts must exist before concurrency syntax is considered stable.

---

126. Downstream consumers

Concurrency semantics are consumed by:

src/parser.rs
src/frontend/ast/
semantic analysis
type/effect analysis
resource analysis
canonical IR
optimizer
routing
scheduler
resilience
runtime
HAL
backend
tooling
diagnostics
tests

No downstream component should need to inspect raw concurrency grammar text.

---

127. Public grammar contract

The public concurrency grammar must remain compositional.

Preferred conceptual form:

concurrency_construct
    : task_construct
    | future_construct
    | actor_construct
    | channel_construct
    | synchronization_construct
    | parallel_construct
    | data_parallel_construct
    | task_parallel_construct
    | cancellation_construct
    ;

The exact ANTLR composition syntax is determined by the canonical grammar architecture.

The semantic contract is independent of the exact grammar-file organization.

---

128. No grammar-level runtime API

The grammar MUST NOT encode:

spawn_thread()
create_os_thread()
pthread_create()
cudaStreamCreate()
process_fork()
device_0
core_7

as the core concurrency model.

These are implementation mechanisms.

Portable syntax must express semantic intent.

---

129. No fixed resource topology

The following must never become universal grammar assumptions:

one task per core
one actor per process
one queue per worker
one worker per task
one GPU per parallel region
one QPU per quantum region

A compiler may choose any valid implementation.

---

130. Concurrency and scheduling/resource negotiation

When a program expresses:

requires parallel_execution

the compiler/runtime should negotiate against available capabilities.

The outcome may be:

parallel execution
batched execution
serialized execution
distributed execution
alternative accelerator execution
target rejection

provided semantics are preserved.

---

131. Resource infeasibility

If no target can satisfy a mandatory concurrency requirement, the compiler must report a precise resource/capability diagnostic.

It must not:

- silently remove concurrency;
- silently remove tasks;
- silently change ordering;
- silently drop messages;
- silently ignore cancellation;
- silently change determinism.

---

132. Error categories

Concurrency diagnostics should distinguish:

ConcurrencySyntaxError
ConcurrencyTypeError
ConcurrencyEffectError
ConcurrencyOwnershipError
ConcurrencyDependencyError
ConcurrencyDeadlockRisk
ConcurrencyCapabilityError
ConcurrencyResourceError
ConcurrencyCancellationError
ConcurrencyDeterminismError
ConcurrencyCompatibilityError

The concrete Rust error types belong to the compiler's existing diagnostic/error architecture.

These names are semantic categories, not mandatory public Rust types.

---

133. Source spans

Every concurrency AST construct must retain its source location.

Diagnostics must identify:

- source file;
- span;
- construct;
- relevant dependency;
- relevant resource/capability;
- suggested correction where appropriate.

A concurrency error must not merely say:

invalid concurrency

without location/context.

---

134. Negative examples

The conformance suite must reject or diagnose cases such as:

await non_awaitable_value

send incompatible_type to channel

receive from closed channel

when the semantics prohibit it.

Also:

parallel {
    conflicting_unsynchronized_mutation()
}

when the memory/effect model proves it invalid.

And:

cancel impossible_target_specific_operation

when cancellation semantics prohibit cancellation at that point.

---

135. Boundary cases

Tests must include:

one task
many tasks
zero dynamically generated tasks
deep dependency graph
wide dependency graph
nested parallelism
nested async scopes
large channels
empty channels
closed channels
multiple simultaneous failures
cancellation during await
cancellation during communication
task creation during cancellation
actor termination
distributed failure
large data-parallel domain
single-worker target
many-worker target
heterogeneous target

No artificial fixed maximum may be encoded into these tests.

---

136. Scalability tests

Scalability tests must verify that the language model remains valid for progressively larger logical structures.

Examples:

1 task
10 tasks
100 tasks
1,000 tasks
larger generated workloads

The exact test sizes are test parameters, not language limits.

Tests should also include:

deep dependency graphs
wide dependency graphs
large communication graphs
large actor sets
large data domains
large nested parallel structures

The tests must verify semantic correctness rather than merely throughput.

---

137. Resource-scaling tests

The same logical program should be evaluated under different resource configurations:

minimal resources
moderate resources
large resources
heterogeneous resources
distributed resources

Expected result:

same semantics
different physical realization

unless the program has explicitly declared a resource-dependent semantic requirement.

---

138. Determinism tests

Tests must include:

repeated execution
different scheduler ordering
different worker counts
different target configurations

For deterministic programs:

observable result must remain equivalent

For intentionally nondeterministic programs:

permitted result set must remain within specification

---

139. Concurrency fuzzing

The compiler should eventually fuzz:

task graphs
nested scopes
channel operations
select operations
cancellation
dependency graphs
actor messages
parallel regions

The fuzzing harness must ensure that malformed source cannot crash the compiler.

Compiler safety is required even when the input program is invalid.

---

140. Parser ambiguity testing

Concurrency syntax must be tested against neighboring constructs involving:

blocks
functions
closures
expressions
generics
operators
channels
comparisons
arrows
async
await
spawn

Ambiguous constructs must have one deterministic parse.

---

141. Keyword policy

Concurrency keywords must be coordinated with the canonical lexer.

A parser rule must not silently turn an identifier into a reserved keyword.

Potential vocabulary includes:

async
await
spawn
task
parallel
actor
channel
select
cancel

but a word becomes a reserved keyword only after:

language specification
lexer contract
parser contract
AST contract
semantic contract
compatibility analysis
tests

have been completed.

---

142. Generic operations over keyword explosion

Not every concurrency library operation should become a keyword.

Prefer compositional syntax and semantic operations for:

timeouts
retry policies
executor policies
scheduling hints
work decomposition
custom synchronization

when they do not require unique language semantics.

This prevents the grammar from becoming a catalog of runtime APIs.

---

143. Versioning

Concurrency features must have language-version status:

proposed
experimental
stable
deprecated
removed

A stable concurrency construct must not silently change semantics in a minor compatibility update.

Breaking changes require migration rules.

---

144. Compatibility

Compatibility testing must compare:

specification
Zamani.g4
lexer
parser
AST
semantic analysis
IR
compiler
runtime

for each concurrency feature.

A feature is not stable merely because "Zamani.g4" parses it.

---

145. Feature lifecycle

Every concurrency feature follows:

proposal
   ↓
semantic design
   ↓
specification
   ↓
lexical contract
   ↓
grammar
   ↓
AST
   ↓
semantic implementation
   ↓
IR integration
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

---

146. Feature manifest integration

Where the repository adopts feature manifests, concurrency features should have machine-readable contracts such as:

grammar/specification/features/concurrency-*.yaml

Each manifest should identify:

feature ID
name
status
language version
grammar file
lexer tokens
AST mapping
semantic rules
effect mapping
resource mapping
capability mapping
IR mapping
compiler consumers
runtime consumers
tests
negative tests
boundary tests
scalability tests
compatibility
hard-coding audit

The manifest must not become a second semantic authority; it is a traceability contract.

---

147. Hard-coding audit

Every concurrency-related file must be checked for:

MAX_THREADS
MAX_TASKS
MAX_WORKERS
MAX_ACTORS
MAX_CHANNELS
MAX_NODES
MAX_DEVICES
MAX_CORES
MAX_PARALLELISM

and equivalent disguised constants.

The audit must also detect assumptions such as:

task == thread
actor == process
worker == core
channel == fixed queue
parallel == multiple CPU cores

Any such assumption must be justified as implementation-specific or removed from the language contract.

---

148. What constants are allowed

Ordinary program values are allowed.

For example:

let workers = 32;

is valid if "32" is program data or an explicit semantic policy.

What is prohibited is:

the Zamani language only supports 32 workers

The distinction is:

program constant
    ≠
language ceiling

---

149. No source rewrite for scale

A source program should not need separate variants such as:

program_small.z
program_cpu.z
program_gpu.z
program_cluster.z
program_qpu.z

merely because available concurrency resources differ.

The goal is:

one semantic program
        ↓
different valid realizations

---

150. Portability definition

Concurrency portability means preserving the program's specified semantics across targets.

It does NOT require:

- identical performance;
- identical physical scheduling;
- identical worker count;
- identical latency;
- identical topology.

The source-level contract is semantic portability.

---

151. Performance portability

Performance is not automatically semantic.

The compiler may optimize concurrency differently for different targets.

A program may express explicit performance constraints where they are semantically meaningful:

latency <= X
throughput >= Y

Such constraints must integrate with resource analysis.

---

152. Energy-aware concurrency

Where the resource model supports energy constraints, concurrency may express:

energy <= budget
prefer energy efficiency

The compiler/runtime determines how to satisfy those constraints.

No particular CPU/GPU/QPU power model belongs in the grammar.

---

153. Real-time concurrency

Real-time constraints may be represented through the execution/resource specifications.

Possible semantic properties include:

deadline
period
latency bound
jitter bound

The scheduler/backend must determine whether the target can satisfy them.

The grammar must not promise real-time guarantees merely because a deadline syntax parses.

---

154. Embedded concurrency

Embedded targets may have extremely limited resources.

The same concurrency semantics must remain valid.

The compiler may lower:

logical concurrency

into:

cooperative execution
interrupt-driven execution
static scheduling
hardware state machines

when semantics permit.

---

155. HPC concurrency

High-performance computing may exploit:

data parallelism
task parallelism
distributed parallelism
collectives
accelerators
vectorization

The concurrency grammar remains target-independent.

MPI/OpenMP/CUDA/etc. are implementation/interoperability concerns.

---

156. Scientific computing

Scientific workloads often involve:

- reductions;
- pipelines;
- iterative computation;
- large arrays;
- distributed datasets;
- deterministic numerical requirements.

The concurrency model must support these without imposing fixed data sizes or worker counts.

---

157. Dataflow concurrency

A dataflow model may represent:

producer
    ↓
transform
    ↓
transform
    ↓
consumer

The compiler may pipeline, fuse, distribute, or batch the graph.

The source semantics remain unchanged.

---

158. Streaming

Streams may be logically unbounded.

The language must not require a finite stream size.

Execution may use:

bounded buffering
backpressure
spill
distributed partitioning
windowing
checkpointing

according to the relevant semantic contracts.

---

159. Infinite or long-lived computations

A concurrent computation may intentionally run indefinitely.

The language must distinguish:

finite computation
long-lived computation
reactive computation
streaming computation
non-terminating computation

Cancellation and lifecycle semantics become especially important.

---

160. Progress guarantees

The language must never infer:

parallel ⇒ progress

or:

async ⇒ nonblocking

without semantic definitions.

A target may lack the resources needed for immediate progress.

The runtime must expose resource starvation appropriately.

---

161. Scheduler starvation

Scheduler starvation is an implementation concern unless the program explicitly requests a progress/fairness guarantee.

Where a guarantee is requested, the compiler/runtime must validate or enforce it.

---

162. Priority inversion

If priority is a semantic requirement, synchronization/resource systems must account for priority inversion.

The grammar does not prescribe a particular mitigation mechanism.

---

163. Memory ordering

Memory ordering belongs to the canonical memory/atomic model.

Concurrency grammar may reference memory-ordering constructs but must not duplicate their definitions.

Possible implementation targets include:

CPU memory barriers
GPU synchronization
distributed consistency

but those are backend realizations.

---

164. Transactional concurrency

If transactions are supported, transaction semantics must define:

begin
commit
abort
conflict
isolation
rollback

Concurrency may compose transactions with tasks and actors.

Transaction semantics belong to the appropriate semantic specification, not to the grammar's runtime implementation.

---

165. Lock ownership

If locks are exposed at the language level, ownership must be represented by the canonical type/memory system.

The grammar must not create a separate lock lifecycle model.

---

166. Async resource lifetime

An asynchronous computation may outlive the lexical expression that created it only when the language's ownership/lifetime model explicitly permits that.

This prevents hidden lifetime/resource leaks.

---

167. Channel lifetime

Channel lifetime must be tied to the canonical ownership model.

The specification must define what happens when:

all senders disappear
all receivers disappear
a task is cancelled while holding an endpoint

---

168. Actor lifetime

Actor lifecycle must explicitly define:

creation
running
stopping
stopped
failed
restarting

if restart semantics are supported.

Actor restart must not silently duplicate externally visible effects.

---

169. Exactly-once semantics

Exactly-once message processing is not assumed.

If exactly-once semantics are required, they must be explicitly defined and supported by the distributed/runtime layers.

Otherwise, the model should distinguish:

at-most-once
at-least-once
best effort
exactly-once

where applicable.

---

170. Ordering

Message/task ordering must be explicitly specified.

Possible guarantees:

program order
channel order
causal order
total order
unordered

A runtime must not accidentally expose stronger or weaker semantics than specified.

---

171. Causality

Distributed concurrency may require causal relationships.

Causality is a semantic relationship, not a particular clock implementation.

The runtime may use:

- logical clocks;
- vector clocks;
- timestamps;
- dependency graphs;
- other mechanisms.

---

172. Synchronization across domains

Concurrency synchronization may cross:

classical
quantum
HDL
distributed
AI
data
accelerator

domains.

Each domain remains responsible for its own semantics.

The synchronization mechanism must preserve domain-specific correctness.

---

173. Quantum synchronization

Quantum/classical synchronization may require waiting for:

quantum operation completion
measurement result
calibration state
device availability

The concurrency layer expresses the dependency.

Quantum runtime/HAL determines how it is realized.

---

174. HDL synchronization

HDL synchronization must respect clock and event semantics.

Software task synchronization must not be naively mapped onto HDL clocks.

The HDL semantic layer owns the correct interpretation.

---

175. Runtime resource exhaustion

When runtime resources are exhausted, the runtime must produce a defined failure or backpressure behavior.

It must not:

drop arbitrary tasks
corrupt messages
change ordering
silently cancel work

unless the language contract explicitly permits those behaviors.

---

176. Compiler resource exhaustion

Compiler memory/time exhaustion is not a language semantic error.

The compiler should report a resource diagnostic rather than claiming the source program is invalid merely because the compiler ran out of resources.

---

177. Runtime resource discovery

Resource discovery belongs downstream.

Concurrency syntax must not inspect:

number_of_cores()
number_of_gpus()
number_of_qpus()

as an implicit language mechanism for determining semantic meaning.

Such queries may exist as explicitly specified runtime APIs, but their results must not redefine core concurrency semantics.

---

178. Adaptive execution

Adaptive runtime behavior is permitted.

For example:

available parallelism changes

may cause:

different batching
different placement
different worker count

provided the program's semantic contract is preserved.

---

179. Self-scaling

A program may express logical work proportional to input/resource domains.

The runtime/compiler may determine physical parallelism.

This is a core mechanism for POCO-REAF.

---

180. No implicit physical identifiers

Portable concurrency syntax must not require:

thread_id
core_id
gpu_id
qpu_id
node_id
device_id
worker_id

as part of ordinary task identity.

Explicit target-specific identifiers belong to target-specific dialects.

---

181. Target-specific dialect boundary

If an application genuinely requires physical affinity, a dialect may express it.

The dialect must clearly mark the construct as target-specific.

Such a construct cannot be used as evidence that the portable core language itself is target-independent.

---

182. Source compatibility

Adding a new concurrency capability must not change the meaning of existing valid programs.

If a previously ordinary identifier becomes reserved, compatibility tooling must provide migration information.

---

183. Parser recovery

Malformed concurrency syntax should produce recoverable diagnostics where practical.

Examples:

missing task body
missing channel type
missing await expression
malformed select arm
malformed cancellation scope

Recovery must not generate a misleading valid AST.

---

184. Security of parser recovery

Malformed concurrent source must not crash the compiler.

No unsafe memory behavior may be introduced by parser recovery.

This is particularly important for fuzzing and untrusted source input.

---

185. Tooling integration

Tooling should understand:

task boundaries
async boundaries
await points
actor declarations
channel endpoints
parallel regions
synchronization
cancellation
dependencies

Tooling must derive these from AST/semantic information rather than parsing ".g4" files independently.

---

186. IDE integration

An IDE may visualize:

task graph
dependency graph
actor graph
channel graph
parallel regions
resource requirements

but these are views over canonical semantic data.

They are not alternate language definitions.

---

187. Documentation integration

"grammar/concurrency/README.md" must remain the navigation document.

This file is the normative semantic specification.

"grammar/grammar.md" describes implementation conformance.

"grammar/Zamani-Grammar.md" remains the broader design/proposal/history document.

"grammar/DESIGN.md" remains the architecture authority.

These documents must not contradict one another.

---

188. Existing concurrency README integration

The existing concurrency README correctly establishes:

syntax → lexer → parser → AST → semantic analysis → effect/resource analysis → IR → optimization → scheduling → runtime

This specification formalizes that architecture and extends it with:

- deterministic semantics;
- cancellation;
- resource negotiation;
- distributed semantics;
- quantum integration;
- HDL integration;
- actor lifecycle;
- channel lifecycle;
- failure semantics;
- scalability;
- feature completion criteria.

---

189. Existing grammar file integration

The existing dedicated grammar files must be audited against this specification.

The audit must answer for every rule:

What does it parse?
What AST node does it produce?
What semantic construct does it represent?
What effect does it produce?
What resource/capability information does it carry?
What IR construct receives it?
Which compiler component consumes it?
Which runtime component consumes the lowered representation?
Which tests prove it?

A rule without these answers is not production-complete.

---

190. Existing runtime integration

The existing runtime concurrency implementation must consume semantic/IR representations.

It must not need to understand:

grammar/concurrency/*.g4

directly.

The runtime may implement:

task scheduling
async execution
inter-task communication

but these must be driven by compiled representations.

---

191. Existing standard library integration

The existing synchronization/concurrency library remains a library layer.

It should provide reusable abstractions rather than becoming the definition of language semantics.

Where language syntax directly represents a library operation, the relationship must be documented.

---

192. Existing actor specification integration

"src/compiler/language_spec/concurrency_actors.rs" must be reconciled with this document.

It must not independently define actor semantics that contradict:

grammar/spec/concurrency.md

The source-level actor specification should become an implementation/conformance representation of this semantic contract.

---

193. Canonical source of truth for actor semantics

The intended authority should become:

grammar/spec/concurrency.md
        ↓
grammar/concurrency/actors.g4
        ↓
AST
        ↓
semantic actor model
        ↓
IR
        ↓
compiler/runtime

not:

actors.g4
        +
concurrency_actors.rs
        +
README

with potentially different meanings.

---

194. Canonical source of truth for channels

Likewise:

grammar/spec/concurrency.md
        ↓
channels.g4
        ↓
AST
        ↓
type/effect/ownership analysis
        ↓
IR
        ↓
runtime

Channel implementation details remain downstream.

---

195. Canonical source of truth for parallelism

Likewise:

grammar/spec/concurrency.md
        ↓
parallel.g4
data-parallel.g4
task-parallel.g4
        ↓
AST
        ↓
semantic parallel model
        ↓
IR
        ↓
optimizer
        ↓
scheduler

---

196. Required conformance matrix

For every concurrency construct, maintain a matrix:

Construct| Spec| Grammar| Lexer| Parser| AST| Semantics| Effects| Resources| IR| Compiler| Runtime| Tests
task| required| required| required| required| required| required| required| required| required| required| required| required
future| required| required| required| required| required| required| required| required| required| required| required| required
actor| required| required| required| required| required| required| required| required| required| required| required| required
channel| required| required| required| required| required| required| required| required| required| required| required| required
synchronization| required| required| required| required| required| required| required| required| required| required| required| required
parallel| required| required| required| required| required| required| required| required| required| required| required| required
data parallel| required| required| required| required| required| required| required| required| required| required| required| required
task parallel| required| required| required| required| required| required| required| required| required| required| required| required
cancellation| required| required| required| required| required| required| required| required| required| required| required| required

A row is not "complete" merely because the grammar column is complete.

---

197. Positive test requirements

Each construct must have tests for:

minimal valid form
normal form
nested form
generic form
parameterized form
large form
cross-domain form

---

198. Negative test requirements

Each construct must test:

missing operands
wrong types
invalid ownership
invalid dependencies
invalid cancellation
invalid channel operations
invalid synchronization
invalid resource requirements
invalid capability requirements
ambiguous syntax
malformed nesting

---

199. Boundary test requirements

Tests must cover:

empty scope
single task
single actor
single channel
one producer
one consumer
many producers
many consumers
deep nesting
wide dependency graph
dynamic creation
cancellation races
simultaneous failures
large data domains

---

200. Cross-domain test requirements

Concurrency tests must eventually cover:

classical + concurrency
quantum + concurrency
hybrid + concurrency
HDL + concurrency
AI + concurrency
distributed + concurrency
networking + concurrency
data + concurrency
accelerator + concurrency
security + concurrency

---

201. Quantum conformance tests

At minimum:

concurrent classical computations around quantum operations
independent quantum operations
measurement-driven classical branching
quantum task dependencies
multiple logical quantum computations
dynamic quantum-classical dependencies
resource-constrained quantum concurrency

No test may establish an artificial maximum qubit/task count.

---

202. HDL conformance tests

At minimum:

concurrent hardware processes
clocked process interaction
pipeline concurrency
software/hardware boundary
hardware resource requirements

The tests must distinguish software concurrency from HDL concurrency.

---

203. Distributed conformance tests

At minimum:

local concurrency
two logical nodes
many logical nodes
message dependency
failure propagation
cancellation propagation
retry/recovery
partition behavior
ordering
causal dependency

The number of nodes must remain a test parameter.

---

204. Scalability conformance

A valid implementation must demonstrate that logical concurrency scales without a language-defined ceiling.

The implementation may eventually encounter:

memory exhaustion
compiler time limits
runtime capacity
target capacity

Those are resource conditions.

They must not appear as grammar restrictions.

---

205. Completion definition

This specification is considered integrated only when:

Specification

- concurrency semantics are normative;
- all concurrency domains have defined ownership;
- resource/capability semantics are defined;
- determinism is defined;
- cancellation is defined;
- failure semantics are defined.

Grammar

- every syntax construct has a grammar owner;
- no duplicate grammar authority exists;
- "Zamani.g4" composition is explicit;
- concurrency grammar files have no hidden dependencies.

Lexer

- all reserved words are centrally defined;
- punctuation/operator ownership is clear;
- no concurrency grammar silently creates keywords.

Parser

- one canonical concurrency parse path exists;
- malformed syntax has diagnostics;
- ambiguity is tested.

AST

- every construct maps to a domain-neutral AST representation;
- source spans are retained;
- no physical-resource IDs are required.

Semantics

- type/effect/ownership/resource/capability checks exist;
- concurrency safety rules are defined.

IR

- concurrency has one canonical semantic representation;
- no duplicate universal concurrency IR exists;
- quantum concurrency integrates with "quantum::ir".

Compiler

- optimization preserves semantics;
- scheduling consumes semantic dependencies;
- resource negotiation is downstream.

Runtime

- runtime consumes compiled representations;
- runtime implementation is independent of grammar files;
- resource exhaustion has defined behavior.

Tests

- positive;
- negative;
- boundary;
- scalability;
- determinism;
- compatibility;
- cross-domain;
- fuzzing

coverage exists.

---

206. Final architectural invariant

The complete concurrency architecture is:

                    Zamani Program
                          │
                          ▼
                    Concurrency Syntax
                          │
                          ▼
                         AST
                          │
          ┌───────────────┼────────────────┐
          ▼               ▼                ▼
        Types           Effects         Ownership
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                Concurrency Semantics
                          │
          ┌───────────────┼────────────────┐
          ▼               ▼                ▼
       Resources       Capabilities     Constraints
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                    Canonical IR
                          │
       ┌──────────────────┼──────────────────┐
       ▼                  ▼                  ▼
  Classical           quantum::ir        HDL/Hardware
       │                  │                  │
       └──────────────────┼──────────────────┘
                          ▼
                     Optimization
                          │
          ┌───────────────┼────────────────┐
          ▼               ▼                ▼
       Routing        Scheduling       Resilience
          │               │                │
          └───────────────┼────────────────┘
                          ▼
                         ZQN
                          │
                         HAL
                          │
                    Target lowering
                          │
       ┌──────────────────┼──────────────────┐
       ▼                  ▼                  ▼
      CPU          GPU/FPGA/QPU       Distributed/Future

The invariant is:

Concurrency syntax
        ≠
physical execution

and:

logical scale
        ≠
physical resource count

and:

program meaning
        ≠
target realization

Therefore a Zamani concurrency program can be written once and subsequently realized on different amounts and kinds of hardware without changing its semantic source merely to accommodate different resource counts.

The implementation is allowed to scale down through serialization, batching, streaming, or other semantics-preserving transformations when resources are scarce, and scale up through parallel, distributed, accelerator, or heterogeneous execution when resources are available.

No artificial concurrency ceiling belongs in the language.

---

207. Final "done with this file" checklist

"grammar/spec/concurrency.md" is complete when the following are all true:

- [x] Purpose defined.
- [x] Scope defined.
- [x] Authority defined.
- [x] Existing concurrency filenames retained.
- [x] Duplicate "grammar/antlr/Concurrency.g4" issue identified.
- [x] Lexer ownership defined.
- [x] Parser ownership defined.
- [x] AST contract defined.
- [x] Type integration defined.
- [x] Memory/ownership integration defined.
- [x] Effect integration defined.
- [x] Resource integration defined.
- [x] Capability integration defined.
- [x] Compiler integration defined.
- [x] Canonical IR integration defined.
- [x] "quantum::ir" boundary preserved.
- [x] Scheduling integration defined.
- [x] Runtime integration defined.
- [x] HAL/backend boundary defined.
- [x] Task semantics defined.
- [x] Future/await semantics defined.
- [x] Actor semantics defined.
- [x] Channel semantics defined.
- [x] Synchronization semantics defined.
- [x] Structured concurrency defined.
- [x] Cancellation defined.
- [x] Parallelism defined.
- [x] Data parallelism defined.
- [x] Task parallelism defined.
- [x] Distributed concurrency defined.
- [x] Heterogeneous concurrency defined.
- [x] Quantum/classical integration defined.
- [x] HDL integration defined.
- [x] Determinism defined.
- [x] Failure semantics defined.
- [x] Scalability defined.
- [x] POCO-REAF integration defined.
- [x] No-hard-coded-resource rule defined.
- [x] Rust 1.97/1.97.1 requirement defined.
- [x] No-Rust-"unsafe" requirement defined.
- [x] Diagnostics defined.
- [x] Compatibility defined.
- [x] Positive-test requirements defined.
- [x] Negative-test requirements defined.
- [x] Boundary-test requirements defined.
- [x] Scalability-test requirements defined.
- [x] Cross-domain-test requirements defined.
- [x] Feature completion criteria defined.
- [x] Cross-file integration dependencies defined.
- [x] Downstream consumers defined.
- [x] Hard-coding audit defined.

No subsequent grammar file should need to redefine any of these fundamental concurrency semantics. Subsequent files should implement their assigned syntax/implementation contracts against this specification.