Zamani Distributed Grammar

Path: "grammar/distributed/README.md"
Domain: Distributed, parallel, federated, remote, clustered, and heterogeneous computation
Grammar technology: ANTLR4 parser grammar components
Language: Zamani
Compiler/runtime baseline: Rust 1.97 / Rust 1.97.1, Edition 2021
Safety: Safe Rust only; "unsafe" is prohibited
Primary portability objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
Scalability objective: From the smallest supported computation to arbitrarily large distributed computation, subject only to actual semantic requirements and available resources.

---

1. Purpose

The "grammar/distributed/" subsystem defines the source-level syntax boundary for distributed computation in Zamani.

It allows a Zamani program to express distributed computational intent without making a particular machine, cluster, processor, accelerator, network, provider, topology, or deployment architecture part of the language definition.

The distributed grammar must therefore support programs that may ultimately execute across:

- one execution resource;
- multiple CPU cores;
- multiple processes;
- multiple machines;
- embedded systems;
- edge systems;
- HPC systems;
- clusters;
- supercomputers;
- clouds;
- heterogeneous systems;
- CPU/GPU/FPGA/ASIC systems;
- quantum-classical systems;
- distributed quantum systems;
- federated environments;
- remote execution environments;
- future computing architectures.

The fundamental principle is:

«Zamani describes distributed computation and intent; downstream systems determine how that intent is realized.»

---

2. POCO-REAF

Distributed syntax is part of Zamani's:

«Program Once → Compile Once → Run Everywhere → Run Anywhere → Run Forever»

architecture.

The source program must describe the computation rather than the machine on which it happens to execute.

The architecture is:

Zamani Source
     |
     v
Canonical Lexer
     |
     v
Distributed Parser Grammar
     |
     v
Frontend AST
     |
     +--> Name Resolution
     +--> Type Analysis
     +--> Effect Analysis
     +--> Capability Analysis
     +--> Resource Analysis
     +--> Ownership / Lifetime Analysis
     +--> Security Analysis
     +--> Distributed Semantic Analysis
     |
     v
Canonical Semantic Representation
     |
     +--> Classical IR
     +--> quantum::ir
     +--> HDL / Hardware Representation
     +--> Resource Requirements
     +--> Distributed Execution Metadata
     |
     v
Optimization
     |
     v
Routing / Placement / Scheduling
     |
     v
Target Lowering
     |
     v
Runtime / Deployment

The grammar must never bypass this architecture.

---

3. Relationship to the Rest of Zamani

The repository already contains distributed concepts outside this directory.

Examples include:

src/distributed/
src/runtime/distributed.rs
src/quantum/ir/model/distributed.rs
src/quantum/routing/distributed.rs
src/quantum/memory/distributed.rs
src/quantum/error_correction/distributed.rs
src/quantum/scheduling/context.rs
src/quantum/ir/resources/locality.rs
grammar/effects/distributed.g4
grammar/memory/distributed-memory.g4

The existence of those components is important.

"grammar/distributed/" must not duplicate their semantic models.

Instead:

grammar/distributed/
        |
        v
source syntax
        |
        v
frontend AST
        |
        v
semantic distributed model
        |
        +----------------+
        |                |
        v                v
classical semantics   quantum semantics
        |                |
        |             quantum::ir
        |                |
        +-------+--------+
                |
                v
       resource / placement /
       routing / scheduling
                |
                v
             runtime

The repository already describes distributed quantum concepts in canonical quantum IR structures, including distributed nodes, logical/physical references and communication links. The grammar must feed those structures rather than inventing a second IR.

---

4. Scope

This directory owns source syntax for:

- distributed computation;
- distributed scopes;
- distributed entities;
- distributed tasks;
- distributed services;
- distributed workers;
- distributed actors;
- distributed channels;
- distributed state;
- distributed communication intent;
- remote execution intent;
- dependencies;
- coordination;
- replication intent;
- consistency intent;
- migration intent;
- recovery/fault-tolerance intent;
- deployment intent;
- federation intent;
- distributed resource requirements;
- distributed constraints;
- distributed preferences;
- distributed hints;
- distributed semantic extensions.

It does not implement those mechanisms.

---

5. Ownership

This directory owns

- distributed source grammar;
- distributed syntactic composition;
- distributed declaration shapes;
- distributed statement shapes;
- distributed invocation syntax;
- distributed dependency syntax;
- distributed relationship syntax;
- distributed scope syntax;
- distributed semantic-attribute syntax where required;
- distributed extension syntax;
- source-level distributed intent.

This directory does not own

- lexical token definitions;
- identifiers;
- qualified names;
- expression precedence;
- type definitions;
- ownership checking;
- borrow checking;
- lifetime analysis;
- memory implementation;
- network protocols;
- network transport;
- node discovery;
- device discovery;
- hardware discovery;
- resource allocation;
- scheduling;
- routing;
- placement algorithms;
- load balancing;
- deployment implementation;
- replication algorithms;
- consensus algorithms;
- consistency algorithms;
- fault detection;
- resilience decisions;
- QEC;
- ZQN;
- quantum gates;
- quantum topology;
- quantum calibration;
- "quantum::ir";
- classical IR;
- hardware IR;
- runtime execution.

---

6. Critical Architectural Rule: Syntax Is Not Semantics

The distributed parser answers:

«Is this structurally valid distributed Zamani syntax?»

It must not answer:

«Can this deployment actually be executed?»

It must not decide:

- which machine is selected;
- which node executes a task;
- how many nodes exist;
- how many replicas are created;
- which network is used;
- which transport protocol is used;
- which scheduler is used;
- which placement algorithm is used;
- which cloud provider is used;
- which CPU/GPU/QPU is used;
- whether sufficient resources currently exist.

Those decisions belong downstream.

The repository's distributed-memory grammar already follows this distinction: source syntax describes logical memory intent while placement, node discovery, routing, scheduling, replication, hardware selection and runtime execution remain downstream concerns.

---

7. Canonical Distributed Grammar Entry Point

The directory must have one canonical parser composition boundary.

The intended public entry point is:

distributedDeclaration

or, if the final parser architecture requires a domain-specific composition rule:

distributedDomain

There must not be multiple competing "canonical" entry points.

Rules such as:

distributedProgram
distributedScope
distributedCompleteScope
distributedCompilationUnit
distributedMemberExpanded
distributedCompleteMember
distributedStatement

may exist only when they have a clearly documented parser-composition purpose.

They must not represent different versions of the same language construct.

---

8. Important Correction to the Existing "distributed.g4"

The existing grammar has a strong open-world design, but it currently contains substantial duplication.

Examples include overlapping concepts represented separately as:

distributedOperation
distributedCommunication
distributedPolicy
distributedReplication
distributedConsistency
distributedPlacement
distributedRemoteExecution
distributedDeployment
distributedCoordination
distributedFailureIntent
distributedInvocation

and multiple overlapping scope/member rules.

This creates a maintenance problem:

one semantic construct
        |
        +--> rule A
        +--> rule B
        +--> rule C
        +--> rule D

The production architecture must instead establish:

one structural syntax
        |
        v
one AST construct
        |
        v
semantic classification

The grammar may provide aliases for parser composition where genuinely necessary, but aliases must not create independent semantic concepts.

The current file's recursive/open-world approach is valuable and should be retained; the duplication should not be.

---

9. Open-World Distributed Model

Distributed computing must remain extensible.

The grammar must not require a new lexer keyword whenever a new distributed technology appears.

For example, these should remain syntactically representable:

distributed::node
distributed::service
distributed::worker
distributed::actor
distributed::task
distributed::channel
distributed::message
distributed::federation
distributed::region
distributed::partition
distributed::shard
distributed::replica
distributed::coordination
distributed::consensus
distributed::migration
distributed::recovery
distributed::quantum_network
distributed::future_architecture
distributed::future_protocol

The semantic registry determines whether a name is:

- defined;
- supported;
- experimental;
- deprecated;
- vendor-specific;
- extension-defined;
- unknown.

The parser must not need to be modified merely because a future distributed abstraction is introduced.

---

10. Namespace Rule

Distributed concepts should use the canonical Zamani naming system.

Examples:

distributed::task
distributed::service
distributed::send
distributed::receive
distributed::consensus
distributed::replication
distributed::migration

The grammar must consume the canonical:

identifier
qualifiedName

rules.

It must not redefine:

IDENTIFIER
identifier
qualifiedName

This is consistent with the existing distributed-effects grammar, which deliberately uses the language-wide naming system instead of introducing distributed-specific identifiers.

---

11. No Distributed-Specific Lexer Explosion

Do not create lexer keywords such as:

NODE
WORKER
SERVICE
ACTOR
SHARD
REPLICA
CLUSTER
REGION
FEDERATION
CONSENSUS
MIGRATION

merely because those concepts exist.

Prefer semantic qualified names:

distributed::node
distributed::worker
distributed::service
distributed::actor
distributed::shard
distributed::replica

This preserves language extensibility.

A future concept:

distributed::swarm

should not require changing the lexical vocabulary.

---

12. Distributed Entities

Distributed entities are logical source-level entities.

Possible semantic classifications include:

- node;
- worker;
- service;
- task;
- actor;
- channel;
- state;
- region;
- partition;
- shard;
- replica;
- endpoint;
- workflow;
- deployment;
- federation;
- execution domain.

The grammar should use a generic structure such as:

distributed-kind + entity-name + body

rather than implementing a closed list of every possible future entity.

Semantic analysis determines the entity's actual kind.

---

13. Distributed Tasks

Tasks represent units of computation.

The grammar must allow:

- named tasks;
- nested tasks;
- parameterized task bodies;
- task dependencies;
- task composition;
- task invocation;
- task result binding;
- task constraints;
- task requirements;
- task preferences;
- task hints;
- asynchronous execution intent;
- remote execution intent.

The grammar must not determine:

- CPU count;
- thread count;
- worker count;
- machine assignment;
- queue;
- scheduler;
- execution time;
- physical placement.

---

14. Distributed Services

Services represent logical computational interfaces.

The grammar may express:

service
service interface
service invocation
service dependency
service availability requirement
service placement preference

It must not select:

- server;
- IP address;
- port;
- cloud provider;
- container runtime;
- orchestration system;
- network transport.

Those belong to downstream deployment/network/runtime layers.

---

15. Distributed Actors

Actor syntax must remain separate from generic concurrency semantics.

The grammar may identify an actor declaration or actor-oriented distributed entity.

It must not implement:

- mailbox scheduling;
- actor runtime;
- message delivery;
- supervision;
- process isolation;
- failure recovery.

Generic concurrency belongs to:

grammar/concurrency/

Distributed semantics provide the distributed context.

---

16. Channels and Communication

Distributed communication syntax represents communication intent.

Examples include semantic operations equivalent to:

distributed::send(...)
distributed::receive(...)
distributed::broadcast(...)
distributed::scatter(...)
distributed::gather(...)
distributed::reduce(...)

The grammar must not choose:

TCP
UDP
QUIC
MPI
RDMA
InfiniBand
vendor transport

The repository's existing distributed grammar already establishes this principle: communication syntax describes intent while networking/runtime layers select the realization.

---

17. Networking Boundary

Distributed communication and networking are related but are not the same abstraction.

The architecture is:

distributed::send(...)
        |
        v
distributed semantic intent
        |
        v
network capability analysis
        |
        v
transport selection
        |
        v
runtime/network realization

The distributed grammar must not import or duplicate protocol definitions.

Networking owns:

- endpoints;
- protocols;
- messages;
- channels where networking-specific;
- transport capabilities;
- network realization.

Distributed owns:

- why communication occurs;
- logical participants;
- semantic communication intent;
- dependency relationships.

---

18. Distributed Dependencies

The grammar must support explicit semantic dependencies.

Conceptually:

task_a -> task_b

means:

«"task_b" depends on "task_a".»

It does not mean:

- network route;
- physical connection;
- hardware link;
- scheduler assignment;
- machine adjacency.

The dependency is later lowered into scheduling/execution structures.

---

19. Distributed Relationships

Relationships may represent:

- dependency;
- communication;
- membership;
- replication;
- coordination;
- federation;
- association;
- logical placement;
- ownership relationships.

Relationships must remain semantic.

They must not silently become physical topology.

---

20. Distributed State

Distributed state is a logical abstraction.

The grammar may express:

distributed state
distributed state initialization
distributed state access
distributed state relationship
distributed state policy

It must not automatically imply:

- persistent storage;
- replication;
- consensus;
- serialization;
- cache coherence;
- physical memory;
- a particular database.

Those meanings require semantic analysis and appropriate downstream subsystems.

---

21. Replication

Replication syntax expresses intent.

Examples include concepts equivalent to:

distributed::replicate(state, policy)
distributed::replication(state, policy)

The grammar must never hard-code a physical replica universe.

Forbidden language-level assumptions include:

replica0
replica1
replica2

or:

MAX_REPLICAS = 3

A replica count, if semantically meaningful, must be represented as a normal expression or resource requirement.

The compiler/resource system determines whether the requested replication is feasible.

---

22. Consistency

Consistency must remain distinct from replication.

The grammar may express a consistency requirement or preference.

Examples may semantically represent:

relaxed
eventual
causal
strong
sequential
transactional

But the grammar does not implement:

- Raft;
- Paxos;
- PBFT;
- CRDT algorithms;
- quorum algorithms;
- transactional engines.

Those belong to semantic/runtime implementations.

---

23. Placement

Placement is an abstract property.

The grammar may express:

placement requirement
placement constraint
placement preference
placement hint

It must not encode a physical machine unless the language specification explicitly defines physical addressing as a semantic feature.

Even when physical placement is explicitly requested, it must be represented through the resource/target model rather than silently becoming part of generic distributed syntax.

---

24. Resource Independence

Distributed syntax must integrate with the universal resource model.

The following concepts must remain distinct:

requirement
constraint
preference
hint
capability
target
placement

For example:

requires distributed computation

must not mean:

use exactly N nodes

Likewise:

prefer locality

must not become a correctness requirement.

Resource semantics belong to:

grammar/resources/

while distributed grammar provides the distributed-domain composition boundary.

---

25. Absolute Scalability Rule

No finite language-level limits may be encoded.

There must be no:

MAX_NODES
MAX_WORKERS
MAX_TASKS
MAX_SERVICES
MAX_ACTORS
MAX_CHANNELS
MAX_MESSAGES
MAX_REPLICAS
MAX_SHARDS
MAX_PARTITIONS
MAX_REGIONS
MAX_DEVICES
MAX_NETWORKS
MAX_CLUSTERS
MAX_DISTRIBUTED_DEPTH

There must also be no implicit fixed:

CPU count
GPU count
QPU count
node count
cluster size
memory size
network size
address width
topology size
device count

The grammar must use repetition and recursion:

*
+

where cardinality is unbounded by language design.

Actual limits belong to:

- parser resource policy;
- compiler resource policy;
- semantic analysis;
- resource manager;
- scheduler;
- deployment system;
- runtime;
- operating system;
- hardware.

The existing distributed grammar correctly follows this principle and explicitly avoids fixed node, worker, service, task, replica and cluster limits.

---

26. "Infinity" Means Resource-Bounded Scalability

Zamani must not falsely promise physically infinite execution.

"Scale to infinity" means:

«The language imposes no artificial finite machine-size ceiling.»

Execution remains bounded by actual:

- memory;
- storage;
- compute;
- network capacity;
- compilation capacity;
- runtime capacity;
- energy;
- hardware capability;
- operating-system limits;
- deployment policies.

Therefore:

language scalability
≠
infinite physical resources

The grammar must preserve the former.

---

27. Quantum Integration

Distributed computing must integrate with Zamani quantum computing.

Examples include:

- distributed quantum computation;
- distributed logical state;
- quantum communication intent;
- distributed measurement/control;
- quantum network abstractions;
- distributed QEC workflows;
- hybrid quantum-classical distributed execution.

The architecture must remain:

distributed syntax
        |
        v
frontend AST
        |
        v
semantic analysis
        |
        v
quantum::ir

The distributed grammar must not define:

QubitId
PhysicalQubitId
GateKind
QuantumTopology
Calibration
Pulse
QECAlgorithm
NoiseChannel

"quantum::ir" remains authoritative for canonical quantum semantics.

The repository already has distributed quantum IR and routing structures; distributed grammar must integrate with those rather than create competing representations.

---

28. QEC Boundary

Distributed grammar must not implement quantum error correction.

It may express high-level intent such as:

distributed recovery
logical-state distribution
fault-tolerant distributed computation

but:

grammar
≠
QEC

QEC owns:

- codes;
- syndrome extraction;
- decoding;
- correction;
- logical error semantics.

The grammar must not introduce physical-qubit limits or duplicate QEC structures.

---

29. ZQN Boundary

ZQN describes quantum noise/fault semantics.

Distributed grammar must not define:

- noise channels;
- leakage;
- loss;
- correlated faults;
- stochastic fault models.

If a distributed quantum execution is affected by faults:

distributed syntax
        ↓
semantic representation
        ↓
quantum::ir
        ↓
ZQN / QEC / resilience

The distributed grammar remains upstream.

---

30. Resilience Boundary

Distributed failure and recovery syntax may express intent.

It must not decide recovery.

Examples:

fault tolerance requirement
recovery policy
availability requirement
failure handling intent

Resilience owns decisions such as:

- retry;
- restart;
- rollback;
- remap;
- reroute;
- reschedule;
- recompile;
- switch backend;
- quarantine;
- abort.

The grammar must not become a distributed resilience engine.

---

31. Memory Integration

Distributed memory is separately represented by:

grammar/memory/distributed-memory.g4

The distributed grammar must not duplicate distributed-memory semantics.

The relationship is:

distributed grammar
       |
       +---- distributed computation
       |
       v
memory grammar
       |
       +---- distributed memory

The existing distributed-memory grammar explicitly establishes itself as the specialized distributed branch of the common memory foundation and forbids duplication of memory places, ownership, lifetimes, allocation and related semantics.

---

32. Effects Integration

Distributed effects are represented through:

grammar/effects/distributed.g4

That grammar owns distributed-effect reference syntax.

Therefore "grammar/distributed/" must not redefine effect sets.

For example:

distributed::send
distributed::receive
distributed::consensus

may be recognized as effect references through the generic effect system.

The existing distributed-effects grammar intentionally keeps these names open-ended and delegates generic effect sets to the generic effect grammar.

---

33. Concurrency Integration

Distributed computation and concurrency are different layers.

grammar/concurrency/

owns generic:

- tasks;
- futures;
- actors;
- channels;
- synchronization;
- parallelism;
- cancellation.

"grammar/distributed/" owns the distributed interpretation.

The same logical task model should therefore be capable of being lowered to:

- local execution;
- multicore execution;
- GPU execution;
- distributed execution;
- heterogeneous execution.

The source language must not force the programmer to rewrite semantics simply because the execution scale changes.

---

34. Hardware and HDL Integration

Distributed syntax may coexist with:

- hardware modules;
- FPGA computation;
- ASIC computation;
- accelerators;
- CPU/GPU execution;
- quantum hardware;
- hardware/software co-design.

However:

distributed grammar
≠
hardware topology

The hardware subsystem owns:

- capabilities;
- resources;
- topology;
- placement;
- target realization.

The distributed grammar only expresses logical distributed intent.

---

35. Classical Integration

Distributed classical computation must work with the canonical classical language.

Examples include:

- distributed numerical computation;
- distributed matrices;
- distributed tensors;
- distributed AI workloads;
- distributed data processing;
- distributed pipelines;
- distributed reductions.

The distributed grammar must consume canonical:

expression
type
function
statement
module

rules rather than recreating them.

---

36. AI/Data Integration

Distributed AI and data processing must be expressible without creating special machine-specific grammar.

Possible semantic combinations include:

distributed + tensor
distributed + dataset
distributed + training
distributed + inference
distributed + pipeline
distributed + accelerator

The AI/data grammars own their respective semantic syntax.

Distributed grammar provides the distributed composition layer.

---

37. Future Computing

The grammar must support future domains without requiring a rewrite of the foundational distributed syntax.

Examples:

distributed::future_architecture
distributed::federated_machine
distributed::neuromorphic_region
distributed::photonic_fabric
distributed::quantum_network
distributed::unknown_future_domain

Syntactic acceptance does not imply semantic support.

Unknown constructs must be resolved by semantic registration/capability analysis.

---

38. Source-Level Portability

A valid portable program should be expressible without embedding:

machine name
device name
node address
network address
physical port
provider identifier
cluster identifier
CPU identifier
GPU identifier
QPU identifier

unless such identity is explicitly part of the program's semantic contract.

Even then, target-specific identification should be isolated through target/resource abstractions.

---

39. Target Selection Boundary

The distributed grammar must not perform target selection.

The correct model is:

source requirement
       |
       v
capability analysis
       |
       v
resource negotiation
       |
       v
target selection
       |
       v
placement
       |
       v
scheduling
       |
       v
execution

This is essential to POCO-REAF.

---

40. Runtime Boundary

The grammar must never contain:

- runtime callbacks;
- execution commands;
- network calls;
- filesystem operations;
- hardware queries;
- environment queries;
- deployment calls;
- dynamic discovery.

ANTLR grammar files must remain declarative.

---

41. Rust Safety

The grammar itself must contain no embedded Rust actions.

The associated Zamani compiler/parser implementation must obey:

Rust 1.97
or
Rust 1.97.1

Edition 2021

unsafe: forbidden

No implementation may introduce:

unsafe
unsafe fn
unsafe impl
unsafe {}

The distributed grammar must remain independent of Rust-specific runtime behavior.

---

42. Determinism

Parsing must be deterministic.

The distributed grammar must contain:

- no randomness;
- no I/O;
- no environment queries;
- no hardware queries;
- no network access;
- no runtime callbacks;
- no mutable global parser state;
- no semantic predicates whose results depend on external state.

The same token stream must produce the same parse structure.

---

43. AST Contract

The parser must produce structural information sufficient for the frontend AST to preserve:

- source spans;
- declaration ordering;
- member ordering;
- qualified-name segments;
- argument ordering;
- expression structure;
- dependency direction;
- nesting;
- attributes;
- modifiers;
- source-level relationships.

The AST must not prematurely lower:

node -> physical machine
task -> CPU
channel -> TCP
replica -> physical copy
placement -> topology coordinate

Those transformations happen later.

---

44. Canonical Semantic Representation

The distributed AST must lower into the repository's canonical semantic representation.

The grammar must never directly create:

DistributedIR
DistributedQuantumIR
DistributedRuntimeIR
DistributedHardwareIR

unless the repository explicitly establishes such a canonical layer.

The preferred architecture is:

AST
 ↓
semantic distributed model
 ↓
canonical IR/domain IR

For quantum:

AST
 ↓
semantic analysis
 ↓
quantum::ir

---

45. Integration With Scheduling

Distributed dependencies may eventually become scheduling dependencies.

But:

grammar dependency
≠
schedule edge

The semantic/compiler layer translates source dependency intent into the appropriate scheduling representation.

Scheduling remains responsible for:

- ordering;
- timing;
- resource conflicts;
- synchronization;
- placement-aware execution;
- scheduling policies.

The grammar only provides the source-level information.

---

46. Integration With Routing

A distributed relationship must not automatically become a network route.

Routing consumes:

logical communication requirements
+
resource/capability information
+
topology

and determines realization.

This separation is particularly important for distributed quantum programs, where logical communication and physical quantum routing are distinct concerns.

---

47. Integration With Resource Management

The grammar can express:

requirement
constraint
preference
hint

but resource management determines:

- availability;
- capacity;
- feasibility;
- allocation;
- admission;
- resource lifecycle.

A source program must never have its semantics changed simply because the current machine is smaller.

---

48. Integration With Deployment

Deployment consumes distributed semantic intent and determines an actual deployment.

The grammar must not directly describe:

Kubernetes
Docker
VM
container
cloud provider
host
IP
port
region identifier

as mandatory language primitives.

Provider-specific concepts belong in interoperability/dialect/target layers.

---

49. Interoperability

Distributed programs may eventually interoperate with:

- MPI;
- RPC systems;
- actor runtimes;
- message queues;
- distributed databases;
- cloud systems;
- HPC systems;
- remote procedure systems;
- foreign languages.

Those integrations belong to:

grammar/interoperability/

and compiler/runtime adapters.

The distributed grammar remains provider-neutral.

---

50. Dialects

Vendor- or platform-specific distributed features must use the dialect mechanism.

Examples:

dialect::vendor::distributed_feature
dialect::experimental::feature

The base distributed grammar must remain stable.

This permits:

core language
+
optional dialect

without contaminating the core language with temporary vendor assumptions.

---

51. Proposed Directory

The distributed directory should ultimately be:

grammar/distributed/
├── README.md
├── distributed.g4
├── nodes.g4
├── services.g4
├── communication.g4
├── messaging.g4
├── remote-execution.g4
├── replication.g4
├── consistency.g4
├── fault-tolerance.g4
└── placement.g4

However, no file should be created merely because it appears in a conceptual tree.

Every file must have a unique ownership boundary.

---

52. File Ownership Matrix

File| Owns| Must not own
"README.md"| architecture, contracts, integration, conformance| executable grammar
"distributed.g4"| canonical distributed composition| specialized implementation semantics
"nodes.g4"| logical node/domain syntax| node discovery/topology
"services.g4"| logical service syntax| service discovery/runtime
"communication.g4"| communication intent| network transport
"messaging.g4"| message-oriented syntax| serialization implementation
"remote-execution.g4"| remote execution intent| deployment/runtime
"replication.g4"| replication intent| replication algorithm
"consistency.g4"| consistency intent| consistency algorithm
"fault-tolerance.g4"| fault-tolerance intent| resilience implementation
"placement.g4"| abstract placement intent| physical placement algorithm

---

53. "distributed.g4"

Purpose

Canonical composition grammar for the distributed domain.

Owns

- distributed declaration composition;
- distributed member composition;
- generic distributed entities;
- generic distributed operations;
- distributed relationships;
- distributed dependencies;
- distributed scopes;
- common distributed structural forms.

Does not own

Specialized semantics already owned by:

nodes.g4
services.g4
communication.g4
messaging.g4
replication.g4
consistency.g4
fault-tolerance.g4
placement.g4

unless those files are deliberately merged into it.

Dependencies

lexer
core/names
expressions
types

as supplied by the canonical parser composition system.

Consumers

- parser;
- AST;
- semantic analysis;
- distributed compiler analysis;
- tests;
- tooling.

Completion criteria

- one canonical entry point;
- no duplicate semantic rules;
- no lexer duplication;
- no fixed resource limits;
- no target-specific assumptions;
- deterministic parsing;
- complete negative tests;
- integration tests with the host grammar.

---

54. "nodes.g4"

Purpose

Source syntax for logical distributed execution domains/nodes.

Owns

- node declarations;
- logical node references;
- node relationships.

Does not own

- node discovery;
- physical node identity;
- topology;
- hardware capabilities;
- addresses;
- placement.

Scalability

No node count is encoded.

---

55. "services.g4"

Purpose

Logical distributed service syntax.

Owns

- service declarations;
- service interfaces;
- service references;
- service invocation structure.

Does not own

- discovery;
- deployment;
- networking;
- load balancing;
- provider APIs.

---

56. "communication.g4"

Purpose

Source-level communication intent.

Owns

- send intent;
- receive intent;
- broadcast intent;
- scatter/gather intent;
- reduction communication intent;
- communication relationships.

Does not own

- transport;
- routing;
- protocol;
- network address;
- network topology.

---

57. "messaging.g4"

Purpose

Message-oriented source syntax.

Owns

- logical messages;
- message creation;
- message references;
- message-oriented relationships.

Does not own

- wire encoding;
- serialization format;
- queue implementation;
- transport.

---

58. "remote-execution.g4"

Purpose

Remote computation intent.

Owns

- remote invocation;
- remote task execution;
- remote result binding;
- remote execution requirements.

Does not own

- host selection;
- deployment;
- scheduling;
- RPC protocol;
- network transport.

---

59. "replication.g4"

Purpose

Replication intent.

Owns

- replication declarations;
- replication relationships;
- replication policies as syntax;
- symbolic replica requirements.

Does not own

- replica placement;
- replica algorithm;
- physical copy creation;
- consensus.

---

60. "consistency.g4"

Purpose

Consistency requirements and preferences.

Owns

- consistency declarations;
- consistency constraints;
- consistency preferences.

Does not own

- consistency algorithm;
- distributed database implementation;
- consensus runtime.

---

61. "fault-tolerance.g4"

Purpose

Source-level distributed fault-tolerance intent.

Owns

- fault-tolerance requirements;
- recovery intent;
- availability intent;
- failure-policy references.

Does not own

- fault detection;
- recovery orchestration;
- retry implementation;
- resilience policy execution.

---

62. "placement.g4"

Purpose

Abstract placement requirements.

Owns

- locality;
- affinity;
- proximity;
- balancing;
- placement requirements;
- placement preferences;
- placement hints.

Does not own

- machine discovery;
- topology;
- placement algorithms;
- hardware allocation.

---

63. When Files Should Be Merged

Files should be merged when they have no independently meaningful ownership boundary.

For example, if "nodes.g4" only contains one generic qualified-name alias and introduces no unique structural syntax, it should not exist merely for directory symmetry.

Likewise, if "communication.g4" and "messaging.g4" inevitably produce the same AST structure, they should be merged.

The final rule is:

«One file = one independently completable responsibility.»

Not:

«One conceptual noun = one file.»

---

64. Dependency Order

Implementation must proceed in dependency order.

Phase D0 — Architecture

grammar/distributed/README.md

Complete this first.

Phase D1 — Canonical foundations

Depend only on already-established:

core/names
core/paths
expressions
types
lexer

Phase D2 — Core distributed grammar

distributed.g4

Phase D3 — Independent domain extensions

nodes.g4
services.g4
communication.g4
messaging.g4

Phase D4 — Higher-level semantics

remote-execution.g4
replication.g4
consistency.g4
fault-tolerance.g4
placement.g4

Phase D5 — Cross-domain integration

Integrate with:

memory
concurrency
effects
resources
classical
quantum
hybrid
hardware
networking
security
AI
data

Phase D6 — Compiler integration

AST
semantic analysis
capability analysis
resource analysis
IR lowering

Phase D7 — Runtime integration

routing
placement
scheduling
deployment
runtime

---

65. Dependency Graph

The intended graph is:

lexer
  |
  +--> core/names
  |
  +--> expressions
  |
  +--> types
          |
          v
   distributed.g4
          |
    +-----+-----------------------------+
    |     |             |               |
    v     v             v               v
 nodes services   communication     messaging
    |     |             |               |
    +-----+-------------+---------------+
                  |
                  v
          distributed AST
                  |
        +---------+----------+
        |         |          |
        v         v          v
    semantic   effects    resources
        |
        +-------------------+
        |                   |
        v                   v
 classical semantics   quantum semantics
                            |
                         quantum::ir
                            |
                +-----------+-----------+
                |           |           |
                v           v           v
             routing    scheduling   optimization
                |
                v
             hardware
                |
                v
             runtime

There must be no reverse dependency from these downstream systems into the source grammar.

---

66. Integration Graph

grammar/distributed/
        |
        +--> grammar/core/
        +--> grammar/types/
        +--> grammar/expressions/
        +--> grammar/effects/
        +--> grammar/memory/
        +--> grammar/concurrency/
        +--> grammar/resources/
        +--> grammar/classical/
        +--> grammar/quantum/
        +--> grammar/hybrid/
        +--> grammar/hardware/
        +--> grammar/networking/
        +--> grammar/security/
        +--> grammar/ai/
        +--> grammar/data/
        |
        v
frontend AST
        |
        v
semantic analysis
        |
        +--> resource analysis
        +--> capability analysis
        +--> security analysis
        +--> ownership/effect analysis
        |
        v
canonical semantic representation
        |
        +--> classical IR
        +--> quantum::ir
        +--> hardware/HDL representation
        |
        v
optimization
        |
        v
routing / placement
        |
        v
scheduling
        |
        v
deployment
        |
        v
runtime

---

67. Testing Requirements

Every distributed grammar component requires:

Positive tests

Valid:

- node declarations;
- service declarations;
- task declarations;
- actors;
- channels;
- messages;
- remote execution;
- dependencies;
- replication;
- consistency;
- placement;
- fault tolerance;
- nested scopes.

Negative tests

Invalid:

- malformed qualified names;
- malformed calls;
- missing delimiters;
- malformed dependencies;
- invalid nesting;
- invalid argument syntax;
- malformed declarations.

Boundary tests

Test:

- zero distributed members where legal;
- one member;
- very many members;
- deeply nested scopes;
- long qualified names;
- large argument lists;
- large dependency graphs.

The grammar must not impose artificial limits.

---

68. Cross-Domain Tests

Mandatory combinations include:

classical + distributed
quantum + distributed
hybrid + distributed
HDL + distributed
hardware + distributed
AI + distributed
data + distributed
networking + distributed
security + distributed
memory + distributed
concurrency + distributed

And combined cases:

classical + quantum + distributed
classical + HDL + distributed
quantum + hardware + distributed
quantum + networking + distributed
quantum + QEC + distributed
quantum + ZQN + distributed
AI + quantum + distributed
classical + quantum + HDL + hardware + distributed

---

69. Scalability Tests

Tests must verify that the grammar has no artificial limits on:

number of nodes
number of tasks
number of services
number of channels
number of messages
number of replicas
number of partitions
number of dependencies
number of distributed scopes
namespace depth
program size

The tests should scale parametrically rather than asserting a particular maximum.

For example, avoid tests whose architecture assumes:

N <= 1024

unless "1024" is merely an explicitly documented test-resource budget rather than a language rule.

---

70. Determinism Tests

Given identical source:

source A
source A
source A

the parser must produce equivalent structural results.

There must be no dependence on:

- machine;
- network;
- clock;
- random number generator;
- environment;
- runtime state.

---

71. Round-Trip Tests

Where the repository supports source serialization:

Source
 ↓
Lexer
 ↓
Parser
 ↓
AST
 ↓
Printer
 ↓
Parser

must preserve semantic structure.

The test must verify:

- names;
- nesting;
- arguments;
- dependency direction;
- attributes;
- distributed relationships.

---

72. AST Stability

Grammar changes must not silently change AST meaning.

If a grammar rule is renamed or consolidated:

old syntax
   ↓
migration/compatibility layer
   ↓
same semantic AST

where compatibility is promised.

AST changes must be explicitly versioned.

---

73. Compatibility

Distributed grammar evolution must distinguish:

syntax addition
syntax clarification
syntax correction
syntax deprecation
syntax removal
semantic change

Adding a new open-world qualified name should generally not require a language-version change.

Changing the structure of an existing construct may require versioning.

---

74. Diagnostics

Distributed syntax errors must preserve:

- source span;
- offending token;
- expected structure;
- diagnostic category;
- stable error identity where the compiler supports it.

The grammar must not hide errors by interpreting malformed source as arbitrary future extensions.

Open-world syntax means:

«unknown semantic construct can be syntactically representable.»

It does not mean:

«every syntactically representable construct is semantically valid.»

---

75. Hard-Coding Audit

Every distributed grammar change must be audited for:

MAX_NODES
MAX_TASKS
MAX_WORKERS
MAX_SERVICES
MAX_CHANNELS
MAX_MESSAGES
MAX_REPLICAS
MAX_SHARDS
MAX_PARTITIONS
MAX_REGIONS
MAX_DEVICES
MAX_NETWORKS
MAX_CLUSTERS

and equivalent indirect restrictions.

Also audit for:

node0
node1
device0
device1
fixed cluster
fixed topology
fixed provider
fixed network
fixed address
fixed port
fixed CPU
fixed GPU
fixed QPU

Each finding must be classified as:

1. semantic language requirement;
2. target requirement;
3. resource constraint;
4. implementation limit;
5. accidental hard-coding;
6. test-only limit;
7. documentation-only limit.

Accidental hard-coding must be removed.

---

76. Physical Identity Rule

Physical identity must never be smuggled into logical distributed syntax.

This is invalid as a universal semantic assumption:

distributed::node node0

if "node0" is intended to mean "the first physical machine."

It is acceptable only if "node0" is explicitly a programmer-defined logical name whose physical realization is resolved later.

The distinction is:

logical identity
        ≠
physical identity

---

77. Resource Rule

A source program may request resources.

It must not permanently encode the current availability of those resources.

For example:

requires distributed

is a semantic requirement.

Where a quantitative resource expression is genuinely meaningful:

requires resources(...)

the value must remain an expression or resource model rather than a grammar constant.

---

78. No Hidden Topology

The distributed grammar must never assume:

ring
mesh
star
tree
fat-tree
torus
fully-connected
linear

unless topology itself is explicitly being expressed as source-level semantic intent.

Even then, topology belongs primarily to:

grammar/hardware/
grammar/networking/
grammar/resources/

and downstream target models.

Distributed computation should remain topology-independent.

---

79. No Hidden Provider

The core grammar must not encode:

AWS
Azure
GCP
Kubernetes
MPI
Slurm
Docker
specific cloud
specific cluster
specific vendor

as universal distributed primitives.

Provider integration belongs to interoperability and dialect layers.

---

80. No Hidden Execution Strategy

The source grammar must not imply:

one process per node
one task per worker
one actor per process
one service per machine
one message per network packet
one replica per device

Those are implementation choices.

---

81. Security Integration

Distributed computation must integrate with:

grammar/security/

for:

- identity;
- permissions;
- capabilities;
- trust;
- cryptography;
- privacy.

Distributed syntax must not silently bypass security analysis.

For example:

remote execution

does not imply permission to execute remotely.

Capability/security analysis must determine whether it is authorized.

---

82. Failure Semantics

A distributed operation may syntactically express failure handling.

But semantic validation must distinguish:

failure requirement
failure policy
failure observation
recovery intent
resilience action

The grammar must not collapse these into one generic "retry" operation.

---

83. Observability

The grammar should remain compatible with downstream telemetry.

It may expose semantic metadata that allows the compiler/runtime to associate:

- task identity;
- service identity;
- logical operation identity;
- dependency identity;
- execution scope.

It must not implement telemetry itself.

---

84. Provenance

Distributed AST structures should preserve enough source information for:

source
 ↓
AST
 ↓
semantic representation
 ↓
IR
 ↓
runtime

to maintain provenance.

This is particularly important when:

- a task is replicated;
- a task is migrated;
- a logical operation is routed;
- a quantum operation is lowered;
- a distributed computation is rescheduled.

---

85. No Semantic Loss During Lowering

The compiler must preserve the distinction between:

logical distributed intent

and:

physical realization

For example:

distributed::send(a, b)

must not lose the fact that the source requested communication merely because lowering selected a particular transport.

The generated representation should retain provenance and semantic intent where the repository's IR supports it.

---

86. Repository Integration Contract

The distributed grammar must integrate with the repository as follows:

grammar/distributed/
        ↓
frontend parser
        ↓
AST
        ↓
semantic analysis
        ↓
resource/capability analysis
        ↓
canonical IR
        ↓
optimization
        ↓
routing
        ↓
scheduling
        ↓
hardware/target lowering
        ↓
runtime

It must never become:

grammar
 ↓
runtime directly

or:

grammar
 ↓
hardware directly

or:

grammar
 ↓
quantum::ir directly

without the appropriate frontend/semantic boundary.

---

87. ANTLR Integration

The grammar must:

- use the canonical lexer vocabulary;
- use parser grammar composition;
- avoid embedded actions;
- avoid semantic predicates;
- avoid lexer duplication;
- avoid parser-global mutable state;
- avoid external environment dependencies.

The existing distributed grammar uses:

parser grammar Distributed;

options {
    tokenVocab = ZamaniLexer;
}

and imports canonical naming/expression grammars. That architectural direction should be preserved, subject to consolidation of the duplicated rules.

---

88. Rust Integration

ANTLR grammar generation is a language-tooling concern; Rust is the implementation baseline for the Zamani compiler/runtime surrounding it.

The implementation must compile under:

Rust 1.97
Rust 1.97.1
Edition 2021

and must use safe Rust exclusively.

No grammar feature may require unsafe Rust.

---

89. Completion Contract for Every File

A distributed grammar file is not complete until all of these are satisfied:

- Purpose defined;
- ownership defined;
- non-ownership defined;
- dependencies defined;
- parser composition defined;
- AST contract defined;
- semantic contract defined;
- IR boundary defined;
- compiler consumers identified;
- runtime consumers identified;
- cross-domain integration identified;
- scalability audited;
- hard-coding audited;
- deterministic behavior verified;
- positive tests written;
- negative tests written;
- boundary tests written;
- cross-domain tests written where applicable;
- compatibility status documented;
- diagnostics defined;
- source-span preservation verified;
- no unsafe requirement introduced;
- no duplicate semantic model introduced.

---

90. Definition of Done for "grammar/distributed/"

The distributed grammar subsystem is production-ready only when:

[ ] One canonical distributed grammar boundary exists
[ ] No duplicate semantic grammar hierarchy exists
[ ] Canonical lexer is reused
[ ] Canonical identifiers are reused
[ ] Canonical expressions are reused
[ ] Canonical types are reused
[ ] Distributed effects integrate correctly
[ ] Distributed memory integrates correctly
[ ] Concurrency integrates correctly
[ ] Resource model integrates correctly
[ ] Classical computing integrates correctly
[ ] Quantum computing integrates correctly
[ ] quantum::ir remains canonical
[ ] QEC is not duplicated
[ ] ZQN is not duplicated
[ ] Hardware is not duplicated
[ ] HDL is not duplicated
[ ] Networking is not duplicated
[ ] Security is not bypassed
[ ] Routing remains downstream
[ ] Scheduling remains downstream
[ ] Placement remains downstream
[ ] Runtime remains downstream
[ ] No machine limits are encoded
[ ] No fixed node count exists
[ ] No fixed task count exists
[ ] No fixed replica count exists
[ ] No fixed topology exists
[ ] No fixed device count exists
[ ] No provider is embedded
[ ] No physical addresses are required
[ ] Future distributed constructs remain extensible
[ ] Parser behavior is deterministic
[ ] Diagnostics preserve source spans
[ ] Positive tests pass
[ ] Negative tests pass
[ ] Boundary tests pass
[ ] Scalability tests pass
[ ] Cross-domain tests pass
[ ] Compatibility tests pass
[ ] Round-trip tests pass where supported
[ ] Rust 1.97/1.97.1 integration passes
[ ] unsafe Rust remains prohibited

---

91. Final Architectural Principle

The distributed grammar must embody:

«Zamani describes distributed computation, relationships, requirements, capabilities, constraints, preferences, and semantic intent — not the accidental limitations of the machine currently available.»

Therefore:

One Zamani Program
        |
        v
One Stable Semantic Meaning
        |
        +-------------------+
        |                   |
        v                   v
 Local Execution      Distributed Execution
        |                   |
        +---------+---------+
                  |
                  v
       Many Architectures
                  |
                  v
       Many Hardware Configurations
                  |
                  v
       Many Scales
                  |
                  v
       Many Execution Environments
                  |
                  v
       Future Architectures

The source program should not need to be rewritten merely because:

1 node
→
10 nodes
→
1,000 nodes
→
1,000,000 nodes

or:

CPU
→
GPU
→
FPGA
→
ASIC
→
QPU
→
heterogeneous system
→
future architecture

becomes the execution environment.

The language's responsibility is to preserve semantic intent.

The compiler's responsibility is to determine a valid realization.

The resource/capability system's responsibility is to determine feasibility.

The routing/placement/scheduling systems' responsibility is to determine realization.

The runtime's responsibility is to execute that realization.

That separation is what makes the distributed portion of Zamani compatible with:

«Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)»

and with the broader Zamani objective:

«From Atom to Everywhere.»