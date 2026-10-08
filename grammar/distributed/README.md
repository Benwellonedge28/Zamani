Zamani Distributed Grammar

Production Architecture, Ownership, Integration, Scalability and Conformance Contract

Path: "grammar/distributed/"
Domain: Distributed, parallel, federated, remote, clustered, heterogeneous and geographically distributed computation
Grammar technology: ANTLR4 parser grammar components
Language: Zamani
Rust baseline: Rust 1.97 or later
Rust edition: Rust 2021
Safety requirement: Safe Rust only; "unsafe" is prohibited
Portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)

---

1. Purpose

"grammar/distributed/" is Zamani's distributed-computation grammar subsystem.

This README is the architectural orchestrator and integration contract for every file in this directory.

It defines:

- what the distributed grammar subsystem owns;
- what every distributed grammar file owns;
- what every distributed grammar file does not own;
- the dependency direction between files;
- the public parser boundaries;
- integration with the universal Zamani grammar;
- integration with concurrency;
- integration with networking;
- integration with resources;
- integration with effects;
- integration with capabilities;
- integration with contracts;
- integration with policies;
- integration with provenance;
- integration with execution;
- integration with classical computation;
- integration with quantum computation;
- integration with HDL and hardware;
- integration with AI and learning;
- integration with interoperability;
- integration with canonical IR;
- integration with routing;
- integration with scheduling;
- integration with resilience;
- integration with deployment;
- integration with runtime and HAL;
- scalability and hard-coding requirements;
- testing and conformance requirements;
- completion criteria for every distributed grammar file.

This README is not itself an ANTLR grammar.

It is the authoritative architectural contract for the directory.

---

2. Architectural Principle

Distributed Zamani syntax describes portable computational intent.

It must not encode a particular physical realization unless the language specification explicitly defines that information as part of program semantics.

The fundamental separation is:

Zamani source
    |
    v
Canonical lexer
    |
    v
Canonical parser
    |
    v
grammar/distributed/
    |
    v
Domain-neutral AST
    |
    +--> name resolution
    +--> type analysis
    +--> ownership/lifetime analysis
    +--> effect analysis
    +--> capability analysis
    +--> resource analysis
    +--> contract analysis
    +--> policy analysis
    +--> provenance
    +--> distributed semantic analysis
    |
    v
Canonical semantic representation
    |
    +--> Classical IR
    +--> quantum::ir
    +--> HDL / hardware representation
    +--> distributed execution metadata
    |
    v
Optimization
    |
    +--> partitioning
    +--> placement
    +--> routing
    +--> scheduling
    +--> resilience
    +--> recovery
    |
    v
ZQN / target-independent representations
    |
    v
HAL / target realization
    |
    +--> CPU
    +--> multicore
    +--> GPU
    +--> FPGA
    +--> ASIC
    +--> accelerator
    +--> QPU
    +--> simulator
    +--> HPC
    +--> cluster
    +--> federated system
    +--> cloud
    +--> edge
    +--> future target

The distributed grammar is therefore upstream of realization.

It does not select the physical realization.

---

3. The Distributed Composition Authority

There is exactly one ANTLR composition root for this directory:

grammar/distributed/distributed.g4

Its responsibility is:

compose
dispatch
expose public distributed parser boundaries

It must not become the implementation owner of every distributed construct.

The responsibilities are therefore:

grammar/distributed/README.md
    |
    | architectural authority
    v
grammar/distributed/distributed.g4
    |
    | ANTLR composition authority
    v
distributed leaf grammars
    |
    v
domain-neutral AST

No second aggregate grammar may be created.

Do not create:

distributed2.g4
distributed-main.g4
distributed-root.g4
distributed-all.g4
distributed-complete.g4

as competing composition roots.

---

4. README Authority Versus Grammar Authority

This README owns:

- architecture;
- ownership;
- dependency direction;
- integration contracts;
- completion criteria;
- scalability requirements;
- conformance requirements.

"distributed.g4" owns:

- ANTLR imports;
- public parser dispatch;
- composition adapters.

Leaf ".g4" files own:

- concrete syntax for their individual distributed feature.

Specifications own:

- normative language meaning.

The AST implementation owns:

- source representation.

Semantic analysis owns:

- meaning and validation.

IR implementations own:

- compiler representation.

Runtime/HAL owns:

- actual execution.

No layer may silently assume another layer's responsibility.

---

5. Current Distributed Directory

The current distributed subsystem contains these principal files:

grammar/distributed/
├── README.md
├── distributed.g4
├── actors.g4
├── channels.g4
├── collective.g4
├── communication.g4
├── consistency.g4
├── deployment.g4
├── fault-tolerance.g4
├── messages.g4
├── nodes.g4
├── partitioning.g4
├── placement.g4
├── processes.g4
├── remote-execution.g4
├── replication.g4
├── services.g4
├── tasks.g4
├── topology.g4
└── transactions.g4

Additional distributed functionality elsewhere in the repository may participate in this subsystem without being moved into this directory.

In particular, distributed semantics already intersect with:

grammar/concurrency/
grammar/networking/
grammar/resources/
grammar/effects/
grammar/execution/
grammar/validation/
grammar/policies/
grammar/security/
grammar/provenance/
grammar/ai/
grammar/classical/
grammar/quantum/
grammar/hybrid/
grammar/hdl/
grammar/hardware/
grammar/interoperability/
grammar/compile/
grammar/compatibility/

The distributed directory must integrate with those subsystems rather than duplicate them.

---

6. File Ownership Matrix

File| Owns| Does not own
"README.md"| Directory architecture and contracts| Executable grammar
"distributed.g4"| Distributed grammar composition and dispatch| Feature implementation
"nodes.g4"| Logical distributed node syntax| Physical node discovery
"processes.g4"| Distributed process syntax| OS process creation
"actors.g4"| Distributed actor declarations/adapters| Generic actor runtime
"services.g4"| Distributed service declarations| Service deployment/runtime
"tasks.g4"| Distributed task syntax| Scheduler implementation
"channels.g4"| Distributed communication-channel intent| Generic concurrency channel runtime
"messages.g4"| Distributed message use/declaration boundary| Network serialization runtime
"communication.g4"| Distributed communication intent| Transport protocol
"collective.g4"| Collective-operation intent| Collective implementation
"replication.g4"| Replication intent| Replication algorithm
"consistency.g4"| Consistency intent| Consensus/consistency algorithm
"partitioning.g4"| Partitioning intent| Partition-placement algorithm
"placement.g4"| Logical placement intent| Physical placement
"topology.g4"| Logical topology intent| Physical topology discovery
"remote-execution.g4"| Remote-execution intent| Remote process/runtime implementation
"deployment.g4"| Distributed deployment composition| Deployment engine
"fault-tolerance.g4"| Distributed fault/recovery intent| Runtime recovery implementation
"transactions.g4"| Distributed transaction intent| Transaction engine

---

7. Ownership Rule

Every distributed feature must have one syntactic owner.

For example:

communication syntax
    -> communication.g4

not:

communication.g4
distributed.g4
networking.g4
concurrency.g4

all independently defining communication syntax.

The same semantic concept may be consumed by several subsystems, but it must have one syntax owner.

This prevents grammar divergence.

---

8. Dependency Direction

The dependency graph must flow toward composition:

leaf grammars
    |
    v
distributed.g4
    |
    v
canonical parser
    |
    v
AST
    |
    v
semantic analysis

Leaf distributed grammars must not import the complete Zamani parser.

Leaf distributed grammars must not import "distributed.g4".

Leaf grammars must not create cycles such as:

A -> B -> C -> A

The only normal upward dependency is:

leaf -> shared lower-level grammar

and:

leaf -> distributed composition adapter

must be avoided.

---

9. Universal Grammar Authorities

Distributed grammars must reuse existing universal authorities.

Lexer

grammar/antlr/ZamaniLexer.g4
grammar/lexer/

Names

grammar/core/names.g4
grammar/core/qualified-names.g4

Expressions

grammar/expressions/

Types

grammar/types/

Statements

grammar/statements/

Declarations

grammar/declarations/

Functions

grammar/functions/

Modules

grammar/modules/

Distributed grammars must not redefine these systems.

---

10. No Distributed Lexer Explosion

Distributed concepts must not automatically become reserved keywords.

Do not add keywords merely for:

node
worker
service
actor
cluster
shard
replica
partition
federation
region
swarm
topology
consensus
migration

when ordinary Zamani identifiers or qualified names are sufficient.

Prefer semantic names such as:

distributed::node
distributed::worker
distributed::service
distributed::actor
distributed::shard
distributed::replica
distributed::federation
distributed::topology

A future concept such as:

distributed::swarm
distributed::fabric
distributed::mesh_domain
distributed::future_protocol

must remain structurally expressible without changing the universal lexical vocabulary.

Reserved words are justified only where the language specification requires deterministic syntactic boundaries.

---

11. Open-World Distributed Model

Distributed computing is intentionally open-ended.

The grammar must not contain a closed catalogue of all distributed technologies.

Semantic names may represent:

- node kinds;
- service kinds;
- process kinds;
- actor kinds;
- communication models;
- consistency models;
- replication models;
- topology models;
- scheduling policies;
- deployment models;
- federation models;
- future execution models.

Semantic analysis determines whether a name is:

defined
supported
experimental
deprecated
vendor-defined
dialect-defined
extension-defined
unknown

Parsing must not imply implementation support.

---

12. Distributed Source Meaning

A distributed source construct should answer:

«What distributed computation does the program express?»

It must not prematurely answer:

«Which physical machine executes it?»

Therefore:

logical node
    != physical machine

logical process
    != OS process

logical service
    != server instance

logical actor
    != runtime actor implementation

logical channel
    != network socket

logical topology
    != physical topology

logical placement
    != physical placement

replication intent
    != replication implementation

---

13. "nodes.g4"

Owns

"nodes.g4" owns syntax for logical distributed nodes.

A node is a semantic participant in distributed computation.

It may participate in:

- tasks;
- communication;
- services;
- processes;
- actors;
- collective operations;
- topology;
- placement;
- deployment;
- resource requirements.

Does not own

It does not own:

- machine discovery;
- CPU discovery;
- GPU discovery;
- QPU discovery;
- hardware identifiers;
- network-interface discovery;
- physical addresses;
- operating-system process creation;
- node allocation.

Integration

nodes.g4
    |
    v
domain-neutral AST
    |
    v
distributed node semantic model
    |
    +--> resources
    +--> capabilities
    +--> topology
    +--> placement
    +--> deployment
    |
    v
execution planning

Completion

The file is complete when its syntax, AST mapping, semantic ownership, resource integration, tests and scalability contract are fixed independently of downstream implementation details.

---

14. "processes.g4"

Owns

Distributed process syntax and process relationships.

Does not own

- operating-system process creation;
- process IDs;
- process scheduling;
- CPU assignment;
- machine assignment;
- process lifecycle implementation.

Integration

distributed process
    |
    v
AST
    |
    v
distributed semantic model
    |
    +--> concurrency
    +--> resources
    +--> placement
    +--> scheduling
    +--> execution

Generic process/concurrency semantics remain owned by the concurrency subsystem.

---

15. "actors.g4"

Distributed actors must integrate with the existing actor/concurrency architecture.

The distributed grammar must not create a second actor language.

The relationship is:

distributed actor
        |
        v
concurrency actor model
        |
        v
message passing
        |
        v
scheduler/runtime

"actors.g4" may provide distributed-specific declaration/context syntax.

Generic actor lifecycle and concurrency semantics remain outside this directory.

---

16. "services.g4"

Services represent logical computational interfaces.

They may express:

- service declaration;
- service interface;
- service dependency;
- invocation intent;
- availability requirements;
- capability requirements;
- placement preferences;
- deployment intent.

They must not select:

- server;
- IP;
- port;
- container runtime;
- cloud provider;
- orchestration engine;
- network transport.

Integration:

service
    |
    v
AST
    |
    +--> networking
    +--> capabilities
    +--> resources
    +--> policies
    +--> deployment
    |
    v
runtime service realization

---

17. "tasks.g4"

Tasks are logical units of distributed computation.

The grammar may express:

- named tasks;
- task parameters;
- task dependencies;
- task invocation;
- task results;
- task requirements;
- task constraints;
- task preferences;
- task hints;
- asynchronous intent;
- remote-execution intent.

It must not encode:

CPU count
thread count
worker count
queue number
scheduler identity
machine identity
execution duration
physical placement

The scheduler determines realization.

---

18. "channels.g4"

Distributed channels represent logical communication paths.

They must not automatically become:

TCP socket
UDP socket
MPI communicator
RDMA queue
QUIC stream
InfiniBand link

Those are implementation possibilities.

The architecture is:

distributed channel
    |
    v
communication semantics
    |
    v
network capability analysis
    |
    v
transport selection
    |
    v
runtime realization

Generic concurrency channels remain owned by "grammar/concurrency/".

Networking channels remain owned by "grammar/networking/".

The distributed channel grammar must define the boundary between those systems rather than duplicate either one.

---

19. "messages.g4"

Message syntax must integrate with the canonical message/data/type model.

It must preserve:

- message identity;
- payload structure;
- source span;
- type information;
- metadata;
- provenance where applicable.

It must not own:

- serialization algorithms;
- compression;
- encryption;
- network transport;
- packet framing.

Networking and interoperability determine realization.

---

20. "communication.g4"

This file owns distributed communication intent.

Examples of semantic operations include:

send
receive
broadcast
scatter
gather
reduce
all_reduce
exchange
publish
subscribe

These names are semantic operations, not transport implementations.

The grammar must not enumerate:

TCP
UDP
QUIC
MPI
RDMA
InfiniBand
Ethernet

as universal distributed communication mechanisms.

Those belong to networking, interoperability, dialects, capabilities or runtime implementation.

---

21. "collective.g4"

Collective computation represents logical operations involving multiple participants.

Examples include:

broadcast
scatter
gather
reduce
all_reduce
all_gather
exchange
barrier

The grammar must remain open to future collective operations.

It must not hard-code a finite participant count.

Integration:

collective syntax
    |
    v
distributed semantic model
    |
    +--> concurrency
    +--> resources
    +--> topology
    +--> networking
    +--> scheduling
    |
    v
target realization

---

22. "replication.g4"

Replication syntax expresses intent.

It may specify:

- what is replicated;
- replication policy;
- requirements;
- constraints;
- preferences;
- consistency relationships;
- placement relationships.

It must not implement:

- replication algorithms;
- quorum selection;
- replica placement;
- storage synchronization;
- failover.

There must be no universal:

MAX_REPLICAS

or fixed replica inventory.

A quantity is a program expression or semantic requirement, not a grammar capacity.

---

23. "consistency.g4"

"consistency.g4" owns source-level consistency intent.

It must remain open-world.

Names such as:

strong
linearizable
causal
eventual
sequential
session
monotonic_read
monotonic_write
bounded_staleness

remain semantic names unless the language specification explicitly reserves one.

The grammar does not implement:

- consensus;
- locking;
- quorum algorithms;
- CRDT algorithms;
- transaction engines;
- conflict-resolution algorithms.

It expresses requirements that downstream semantic/runtime layers realize.

---

24. "partitioning.g4"

Partitioning describes logical data/work partitioning.

It may express:

- partition intent;
- partition key;
- partition relationship;
- partition constraints;
- locality requirements;
- balancing preferences.

It does not own:

- partition placement;
- load-balancing algorithms;
- physical shard assignment;
- storage implementation.

Integration:

partitioning
    |
    +--> data
    +--> distributed
    +--> resources
    +--> topology
    |
    v
placement / scheduling / runtime

---

25. "placement.g4"

Placement expresses logical placement intent.

The distinction is:

placement requirement
placement constraint
placement preference
placement hint

These must not be conflated.

For example:

prefer locality

is not equivalent to:

requires locality

Physical placement belongs downstream.

"placement.g4" must not select:

- machine IDs;
- GPU IDs;
- QPU IDs;
- FPGA coordinates;
- cloud providers;
- physical network paths.

---

26. "topology.g4"

Topology describes logical relationships.

It must support open-ended topology semantics without enumerating every topology family.

Do not hard-code:

ring
mesh
torus
tree
star
hypercube
dragonfly
fat_tree
heavy_hex

as a closed grammar universe.

Those may be semantic identifiers.

A future topology must remain syntactically expressible.

The architecture is:

logical topology
    |
    v
semantic topology
    |
    v
resource/capability analysis
    |
    v
placement
    |
    v
routing
    |
    v
scheduling
    |
    v
physical realization

---

27. "remote-execution.g4"

Remote execution represents execution intent relative to a logical execution domain.

It must not implement:

- RPC;
- process spawning;
- SSH;
- container launch;
- cloud API;
- machine selection;
- network transport.

Those are realization mechanisms.

Remote execution must integrate with:

execution/
networking/
resources/
capabilities/
security/
policies/
deployment/
runtime/

---

28. "deployment.g4"

Distributed deployment is a composition boundary.

The existing deployment grammar should remain the canonical owner of generic deployment syntax.

"distributed/deployment.g4" must adapt rather than duplicate generic deployment rules.

The intended relationship is:

execution/deployment.g4
        |
        v
generic deployment syntax
        |
        v
distributed/deployment.g4
        |
        v
distributed deployment semantics

It must not create a competing deployment language.

---

29. "fault-tolerance.g4"

Fault-tolerance syntax describes intent and policy.

It may represent:

- retry intent;
- recovery intent;
- fallback;
- redundancy;
- degraded operation;
- failure policy;
- availability requirements;
- recovery constraints.

It must not implement recovery algorithms.

It must integrate with the repository's existing resilience vocabulary:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

and outcomes:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

These are semantic/runtime concepts, not parser algorithms.

---

30. "transactions.g4"

Transactions represent distributed transaction intent.

The grammar may express:

- transaction boundaries;
- participants;
- transactional requirements;
- consistency requirements;
- failure behavior;
- isolation intent;
- recovery intent.

It must not implement:

- two-phase commit;
- three-phase commit;
- consensus;
- lock management;
- transaction logs;
- storage engines.

Those belong downstream.

---

31. Universal Resource Integration

Distributed programs must integrate with:

grammar/resources/

The semantic distinctions are mandatory:

requirement
constraint
preference
hint
capability
resource
target
placement
allocation

They are not interchangeable.

For example:

requires memory >= required_memory;
requires capability("distributed.execution");
requires topology(required_topology);
prefer locality;

means:

required condition
required capability
required topology property
advisory preference

It does not mean that the grammar itself allocates resources.

---

32. Capability Integration

Capabilities are open-ended.

Examples include:

distributed.execution
distributed.communication
distributed.collective
network.communication
network.high_bandwidth
accelerator.compute
tensor.compute
quantum.measurement
quantum.dynamic_circuit

The distributed grammar must not maintain an exhaustive capability list.

Capability registries and semantic analysis determine availability.

A future capability such as:

future.distributed.acceleration

must remain structurally representable.

---

33. Effects Integration

Distributed constructs may produce or require effects such as:

distributed
network
io
mutation
randomness
native
foreign
simulation
measurement

Effect ownership remains:

grammar/effects/

The distributed grammar must not create a second effect system.

The semantic pipeline is:

distributed syntax
    |
    v
AST
    |
    v
effect analysis
    |
    v
effect checking

---

34. Contract Integration

Distributed constructs may participate in:

requires
ensures
invariant
assume
guarantee
property
assert

Contract ownership remains in:

grammar/validation/

Distributed files may expose contract-bearing syntax only where structurally necessary.

They must not redefine universal contract semantics.

---

35. Policy Integration

Distributed execution may be constrained by policies concerning:

- placement;
- communication;
- replication;
- consistency;
- deployment;
- security;
- resilience;
- adaptation;
- simulation;
- reproducibility;
- resource use.

Policy ownership remains:

grammar/policies/

The distributed subsystem consumes policy semantics.

It does not create a competing policy language.

---

36. Provenance Integration

Distributed compilation and execution can change:

task placement
partitioning
replication
routing
scheduling
deployment
adaptation
recovery

Those transformations should remain traceable through the universal provenance architecture.

Distributed grammars must preserve source information required for:

- source spans;
- declarations;
- relationships;
- requirements;
- policies;
- transformations.

Provenance ownership remains outside this directory.

---

37. Concurrency Integration

Distributed computation and concurrency are related but distinct.

The relationship is:

distributed computation
        |
        v
concurrency
        |
        +--> tasks
        +--> actors
        +--> channels
        +--> scheduling
        +--> synchronization
        |
        v
runtime

The distributed subsystem must not create a second:

- actor model;
- async model;
- task model;
- channel model;
- scheduler language.

Existing concurrency infrastructure remains authoritative for generic concurrency.

---

38. Networking Integration

Distributed computation describes logical computation and communication relationships.

Networking describes communication infrastructure and network semantics.

Therefore:

distributed::send(...)
        |
        v
distributed communication intent
        |
        v
network capability analysis
        |
        v
network realization

Distributed grammar must not duplicate:

addresses
endpoints
protocols
sockets
streams
network services

owned by:

grammar/networking/

---

39. Execution Integration

Distributed execution integrates with:

grammar/execution/

The execution layer owns:

- execution modes;
- adaptive execution;
- simulation;
- deployment;
- fallback;
- recovery;
- execution policy.

Distributed grammar supplies distributed intent.

It does not implement the execution engine.

---

40. Data Integration

Distributed data semantics integrate with:

grammar/data/

This includes:

- datasets;
- graphs;
- schemas;
- queries;
- partitioning;
- distributed data;
- provenance;
- data movement.

Distributed grammar must not create a second data type system.

---

41. Classical Integration

Distributed classical computation follows:

classical computation
        |
        v
distributed decomposition
        |
        v
partitioning
        |
        v
placement
        |
        v
communication
        |
        v
scheduling
        |
        v
target realization

The source computation remains semantically classical.

Distribution is a realization/context property unless explicitly part of program semantics.

---

42. Quantum Integration

Distributed computation must support quantum-classical and distributed quantum workloads without creating a second quantum language.

The boundary is:

distributed source
        |
        v
semantic analysis
        |
        v
quantum semantic model
        |
        v
quantum::ir
        |
        v
quantum optimization
        |
        +--> routing
        +--> scheduling
        +--> QEC
        +--> ZQN
        |
        v
QPU / simulator / target

The distributed grammar must not define:

QubitId
PhysicalQubitId
GateKind
QuantumTopology
Calibration
Pulse
QEC algorithm
ZQN model

Those belong to the quantum subsystem.

Distributed quantum syntax describes distributed intent, not physical quantum hardware.

---

43. Hybrid Integration

Distributed hybrid computation may combine:

classical
quantum
AI
tensor
accelerator
HDL
hardware
network

The semantic architecture remains:

distributed intent
        |
        v
hybrid semantic model
        |
        +--> Classical IR
        +--> quantum::ir
        +--> HDL/hardware representation
        |
        v
global optimization and realization

No hybrid-specific duplicate IR should be created merely because distributed execution participates.

---

44. AI Integration

AI workloads may be distributed across:

- workers;
- actors;
- services;
- accelerators;
- clusters;
- federated environments;
- heterogeneous resources.

The AI grammar remains responsible for AI semantics.

The distributed grammar supplies distributed context.

For example:

AI model
    |
    v
distributed training
    |
    +--> tasks
    +--> workers
    +--> communication
    +--> resources
    +--> capabilities
    +--> policies
    |
    v
execution

Do not create a second distributed-AI actor or task language.

---

45. Learning and Adaptation

Distributed learning and adaptation may interact with:

grammar/ai/
grammar/execution/
grammar/policies/
grammar/effects/
grammar/resources/

Adaptation must remain controlled.

The distributed grammar must never imply unrestricted self-modifying execution.

Any adaptive change must pass through:

policy
capability
effect analysis
resource analysis
authorization
provenance
validation

---

46. HDL and Hardware Integration

Distributed hardware computation may involve:

CPU
GPU
FPGA
ASIC
accelerator
QPU
future hardware

The distributed grammar does not own physical hardware syntax.

Hardware ownership remains:

grammar/hardware/
grammar/hdl/

The distributed layer expresses logical distribution and execution relationships.

Hardware realization occurs later.

---

47. Interoperability Integration

Distributed programs may interface with:

- foreign functions;
- external services;
- external data;
- protocols;
- ABI boundaries;
- dialects.

Ownership remains:

grammar/interoperability/
grammar/dialects/

Distributed grammar must not duplicate FFI or ABI syntax.

Foreign operations must participate in:

effects
capabilities
security
policies
provenance

---

48. Canonical AST Contract

Distributed grammar must produce structures that can be represented by the existing domain-neutral AST architecture.

The distributed grammar must not require a separate permanent distributed AST universe unless the AST architecture explicitly establishes one as part of the canonical domain-neutral model.

Conceptually:

DistributedNode
DistributedProcess
DistributedActor
DistributedService
DistributedTask
DistributedChannel
DistributedMessage
DistributedCommunication
DistributedCollective
DistributedReplication
DistributedConsistency
DistributedPartitioning
DistributedPlacement
DistributedTopology
DistributedRemoteExecution
DistributedDeployment
DistributedFaultTolerance
DistributedTransaction

are semantic classifications of source structures.

Their AST representation must remain compatible with the repository's canonical AST design.

Every node must preserve, where applicable:

- source span;
- source ordering;
- names;
- qualified names;
- attributes;
- modifiers;
- expressions;
- declarations;
- relationships;
- requirements;
- constraints;
- preferences;
- policies;
- provenance-relevant information.

---

49. Semantic Contract

The parser only establishes structural validity.

Semantic analysis must determine:

- name resolution;
- type validity;
- ownership validity;
- lifetime validity;
- effect validity;
- capability requirements;
- resource requirements;
- contract validity;
- policy validity;
- topology validity;
- placement validity;
- partitioning validity;
- replication validity;
- consistency validity;
- transaction validity;
- fault-tolerance validity;
- deployment validity;
- target feasibility.

Parser acceptance must never imply that execution is feasible.

---

50. Canonical IR Contract

The distributed grammar creates no distributed-specific IR merely because the syntax lives in this directory.

Distributed information must flow into the canonical compiler architecture.

For classical computation:

distributed source
    |
    v
AST
    |
    v
semantic analysis
    |
    v
Classical IR

For quantum computation:

distributed source
    |
    v
AST
    |
    v
quantum semantic analysis
    |
    v
quantum::ir

For hardware:

distributed source
    |
    v
HDL/hardware semantic representation

A "DistributedIR" must not be created merely to mirror the grammar directory.

---

51. Optimization Boundary

Optimization belongs downstream.

Distributed grammar must not implement:

- automatic partitioning algorithms;
- placement algorithms;
- routing algorithms;
- load balancing;
- scheduling;
- replication algorithms;
- consensus;
- transport selection.

The optimizer may transform the canonical representation while preserving semantic meaning.

---

52. Partitioning Boundary

Partitioning is an optimization/realization concern unless explicitly specified as program semantics.

The compiler may determine:

what to partition
how to partition
where to partition
when to partition

subject to:

requirements
constraints
capabilities
resources
policies
contracts

The grammar only expresses the source-level intent that actually belongs in the language.

---

53. Placement Boundary

Placement must be resolved after semantic analysis.

The architecture is:

source placement intent
        |
        v
semantic placement model
        |
        v
resource discovery
        |
        v
capability matching
        |
        v
topology analysis
        |
        v
placement

The source program must not need to be rewritten merely because the available machines change.

---

54. Routing Boundary

Routing is downstream.

Distributed grammar may describe:

communication relationship
topology requirement
locality requirement
route preference

but does not implement:

shortest path
adaptive routing
fault-aware routing
quantum routing
network routing

Routing consumes canonical semantic information.

---

55. Scheduling Boundary

Scheduling belongs downstream.

Distributed grammar may express:

- dependencies;
- priorities where semantically valid;
- deadlines where supported;
- ordering requirements;
- synchronization requirements;
- preferences.

It must not select a concrete scheduler.

---

56. Resilience Boundary

Distributed fault tolerance integrates with the repository's broader resilience architecture.

The distributed grammar expresses intent.

The resilience system determines realization.

For example:

failure
    |
    v
semantic policy
    |
    v
resilience analysis
    |
    +--> retry
    +--> recovery
    +--> fallback
    +--> degraded execution
    +--> escalation

---

57. Security Boundary

Distributed execution must integrate with:

grammar/security/

Security concerns include:

- authorization;
- authentication;
- trust;
- sandboxing;
- capability restrictions;
- communication restrictions;
- FFI restrictions;
- deployment restrictions;
- audit;
- provenance.

Distributed grammar does not implement cryptography or authorization engines.

---

58. Determinism

ANTLR grammar components must remain deterministic.

Distributed grammars must not contain:

- embedded Rust actions;
- "unsafe";
- semantic predicates used as runtime logic;
- filesystem access;
- network access;
- hardware discovery;
- resource discovery;
- randomness;
- mutable parser-global state;
- runtime callbacks.

Parsing must depend only on:

token stream
grammar
language version
imported grammar contracts

---

59. Rust Safety Contract

The distributed grammar subsystem is language-definition infrastructure.

The Rust implementation surrounding it must support:

Rust 1.97+
Rust 2021
safe Rust only

No distributed feature may require "unsafe".

No grammar file may embed Rust actions that require unsafe implementation.

The grammar must remain independent from Rust implementation details wherever possible.

---

60. Absolute Scalability Contract

The distributed grammar must not impose artificial physical limits.

The following are explicitly prohibited as language-level limits:

MAX_NODES
MAX_PROCESSES
MAX_WORKERS
MAX_SERVICES
MAX_ACTORS
MAX_TASKS
MAX_CHANNELS
MAX_MESSAGES
MAX_REPLICAS
MAX_SHARDS
MAX_PARTITIONS
MAX_REGIONS
MAX_DEVICES
MAX_NETWORKS
MAX_CLUSTERS
MAX_DEPLOYMENTS
MAX_RESOURCES
MAX_TOPOLOGY_SIZE
MAX_CPU_COUNT
MAX_GPU_COUNT
MAX_FPGA_COUNT
MAX_ASIC_COUNT
MAX_QPU_COUNT
MAX_THREADS
MAX_MEMORY
MAX_STORAGE
MAX_BANDWIDTH
MAX_REGISTER_WIDTH

Equivalent indirect limits are also prohibited.

---

61. No Fixed Resource Universe

The grammar must not assume:

4 nodes
8 workers
16 devices
32 threads
64 machines
1024 endpoints

as universal capacities.

Those may appear as program data or test values, but never as language limits.

For example:

let workers = 1024;

is ordinary program data.

But:

distributed grammar supports at most 1024 workers

is prohibited.

---

62. Meaning of "Infinity"

POCO-REAF does not claim physically infinite resources.

It means:

«The language and grammar do not impose an artificial finite ceiling on distributed scale.»

Actual execution remains bounded by:

- available memory;
- storage;
- compute;
- network capacity;
- compiler capacity;
- runtime capacity;
- operating-system limits;
- target capabilities;
- deployment policies;
- energy;
- physical resources.

Therefore:

language scalability
    !=
physical infinity

The correct guarantee is:

no artificial language ceiling
+
resource-aware realization

---

63. Symbolic Quantities

Distributed quantities must remain expressions wherever possible.

Examples:

requires nodes >= required_nodes;
requires workers >= workload.parallelism;
requires memory >= required_memory;
requires storage >= dataset.size;
requires bandwidth >= workload.bandwidth;
requires topology(required_topology);

The grammar parses the expression.

Semantic analysis determines:

- type;
- units;
- dependencies;
- computability;
- feasibility;
- target availability.

The grammar must not require those values to be compile-time constants.

---

64. Requirement Semantics

A requirement is mandatory.

For example:

requires capability("distributed.execution");

means the realization must provide that capability.

An unsatisfied requirement must produce a semantic feasibility failure.

The compiler must not silently convert it into:

prefer
hint
ignore

---

65. Constraint Semantics

A constraint restricts valid realizations.

It is stronger than a preference and different from a capability.

Constraints must be represented without hard-coding physical inventories.

---

66. Preference Semantics

A preference influences realization but does not necessarily determine correctness.

For example:

prefer locality;

must remain advisory unless the language specification explicitly defines a stronger meaning.

---

67. Hint Semantics

Hints are advisory information.

An implementation may ignore a hint if doing so preserves all stronger semantic obligations.

Hints must never accidentally become hidden requirements.

---

68. Capability Semantics

A capability describes what a target can provide.

A requirement describes what the program needs.

Therefore:

program
    |
    +--> required capabilities
    |
    v
target capability set
    |
    v
negotiation

The grammar must not discover the target capability set.

---

69. Resource Negotiation

Distributed realization must support:

source requirements
        |
        v
resource analysis
        |
        v
capability analysis
        |
        v
target/environment discovery
        |
        v
negotiation
        |
        v
realization

This is the central mechanism enabling POCO-REAF.

---

70. Single-Resource Degeneration

A distributed program should not require a distributed physical machine merely because its source expresses distributed-capable semantics.

A realization may collapse to:

one resource
one process
one execution domain

when semantics permit it.

Likewise, the same semantic program may expand to:

many processes
many machines
many accelerators
many regions

when resources and policies permit.

This is essential to scaling from tiny systems to very large systems.

---

71. Semantic Preservation During Scaling

Changing target scale must not silently change program meaning.

The compiler may change:

partitioning
placement
scheduling
routing
replication strategy
communication implementation
parallel execution strategy

provided semantic contracts are preserved.

It must not silently change:

types
required guarantees
observable semantics
explicit constraints
security requirements
program contracts

---

72. Distributed-to-Local Optimization

If a distributed operation can be safely realized locally, the compiler may choose local execution.

For example:

logical communication

may become:

in-process communication
shared-memory communication
local device communication

if semantics permit.

The source does not need to change.

---

73. Local-to-Distributed Expansion

Conversely, a computation may be distributed during compilation or execution if the language semantics and policies permit it.

The compiler may introduce:

partitioning
communication
replication
scheduling
placement

while preserving the semantic contract.

This is a major POCO-REAF requirement.

---

74. Distributed AI and Agents

AI agents must integrate with the existing concurrency actor model.

The relationship is:

AI agent
    |
    v
actor/concurrency model
    |
    v
distributed execution
    |
    v
scheduler/runtime

No second agent-runtime grammar should be created here.

AI-specific semantics remain under:

grammar/ai/

---

75. Federated Computation

Federated execution should be represented through generic distributed abstractions:

participants
policies
data boundaries
communication
privacy/security constraints
resource requirements
provenance

The grammar must not assume a fixed federation architecture.

A future federation model should be representable through the open semantic model.

---

76. Remote and Heterogeneous Execution

Distributed workloads may span:

CPU
GPU
FPGA
ASIC
accelerator
QPU
simulator
edge
HPC
cloud
future target

The distributed grammar describes the relationship.

Hardware and target systems determine realization.

---

77. Quantum-Classical Distributed Execution

The architecture must permit:

classical node
    |
    v
quantum node
    |
    v
measurement/result
    |
    v
classical decision

without requiring distributed grammar to know the physical quantum architecture.

Quantum semantics remain owned by:

grammar/quantum/

and canonical:

quantum::ir

---

78. Distributed HDL and Hardware

Distributed hardware workloads may involve:

hardware nodes
accelerator nodes
FPGA fabrics
ASIC systems
reconfigurable resources

The distributed grammar describes logical relationships.

"grammar/hdl/" and "grammar/hardware/" describe hardware semantics and realization.

Physical placement is downstream.

---

79. Diagnostics Contract

Syntax diagnostics belong to the grammar.

They should identify:

- malformed declarations;
- malformed statements;
- malformed expressions;
- missing delimiters;
- malformed lists;
- malformed distributed relationships;
- malformed distributed clauses.

Semantic diagnostics belong downstream.

Examples:

unsatisfied requirement
unavailable capability
incompatible topology
invalid placement
incompatible replication policy
incompatible consistency requirements
invalid transaction
unsupported target
policy violation

A physical resource shortage must never be reported merely as a syntax error.

---

80. Compatibility Contract

Distributed syntax must participate in:

grammar/compatibility/

Compatibility must distinguish:

language version
grammar version
AST version
semantic version
IR version
dialect version

A distributed feature may be:

stable
experimental
deprecated
historical
planned
partially implemented

The README must not claim implementation status merely because a grammar file exists.

Actual conformance status belongs in the repository's conformance/status system.

---

81. Specification Ownership

The normative distributed specification belongs under:

grammar/spec/
grammar/specification/

especially:

grammar/spec/distributed.md

This README is the architectural orchestrator.

It must not silently override normative semantic specifications.

If a conflict is found:

DESIGN.md
    |
    v
specification/
    |
    v
spec/
    |
    v
grammar composition

must be reconciled explicitly rather than resolved by undocumented assumptions.

---

82. Root Grammar Integration

The repository's universal language composition remains owned by:

grammar/Zamani.g4

The distributed directory must be integrated through the root composition architecture.

"distributed.g4" is the distributed-domain composition root.

The root grammar must not import every distributed leaf independently if "distributed.g4" is the established aggregate boundary.

Preferred:

Zamani.g4
    |
    v
Distributed
    |
    +--> Nodes
    +--> Processes
    +--> Actors
    +--> Services
    +--> Tasks
    +--> Channels
    +--> Communication
    +--> ...

Not:

Zamani.g4
    |
    +--> nodes
    +--> processes
    +--> actors
    +--> services
    +--> tasks
    +--> ...

The latter destroys the composition boundary.

---

83. Composition Root Contract

"distributed.g4" must expose only stable public rules needed by the wider parser.

The current architectural public boundary is conceptually:

distributedDeclaration
distributedStatement
distributedExpression
distributedConstruct

Compatibility aliases may exist where required by existing parser consumers, but they must delegate to canonical rules rather than introduce duplicate semantic constructs.

---

84. No Competing Distributed Operation Language

Historical references to:

distributedOperation
distributedInvocation
distributedCommunication
distributedTask

must not become competing semantic languages.

If compatibility aliases are required, the relationship must be:

legacy/public adapter
        |
        v
canonical distributed construct

not:

independent grammar
+
independent AST
+
independent semantics

---

85. Public Rule Stability

Once a public rule is consumed outside the directory, its contract should be treated as an API.

Changes should preserve:

- rule meaning;
- ownership;
- AST mapping;
- semantic mapping;
- compatibility.

If a breaking change is unavoidable, it must go through the compatibility/versioning system.

---

86. Per-File Contract Requirement

Every ".g4" file in this directory must document all of the following before it is considered complete:

Purpose
Owns
Does Not Own
Inputs
Outputs
Dependencies
Public Rules
Private Rules
Lexer Dependencies
Grammar Dependencies
AST Contract
Semantic Contract
Type Contract
Effect Contract
Capability Contract
Resource Contract
Contract Integration
Policy Integration
Provenance Integration
IR Contract
Compiler Integration
Runtime Integration
Networking Integration
Concurrency Integration
Quantum Integration
HDL Integration
Hardware Integration
Diagnostics
Compatibility
Scalability
Hard-Coding Audit
Positive Tests
Negative Tests
Boundary Tests
Cross-Domain Tests
Determinism Tests
Completion Criteria

This prevents a supposedly completed file from needing to be reopened merely because another subsystem was subsequently implemented.

---

87. Standard File Header Contract

Every distributed ".g4" file should have a header identifying:

File
Grammar
Status
Purpose
Owns
Does Not Own
Dependencies
Public Entry Rule
AST Contract
Semantic Contract
IR Contract
Integration Contract
Scalability Contract
Safety Contract
Completion Criteria

The header is architectural documentation.

It must remain consistent with this README.

---

88. AST Integration Checklist

Every distributed feature must answer:

What AST node represents it?
Who owns that AST node?
What source span is preserved?
What names are preserved?
What expressions are preserved?
What attributes are preserved?
What relationships are preserved?
What requirements are preserved?
What policies are preserved?
What provenance is preserved?

A grammar feature is not complete merely because ANTLR accepts it.

---

89. Semantic Integration Checklist

Every feature must answer:

What does it mean?
What types does it accept?
What types does it produce?
What effects does it have?
What capabilities can it require?
What resources can it require?
What contracts can constrain it?
What policies can constrain it?
What provenance must survive?
What invalid states must be diagnosed?

---

90. IR Integration Checklist

Every feature must answer:

Does it map to Classical IR?
Does it map to quantum::ir?
Does it map to HDL/hardware representation?
Does it become execution metadata?
Does it become optimization metadata?
Does it become resource requirements?
Does it become policy metadata?

The answer must be documented before declaring the grammar feature complete.

---

91. No Distributed IR Fragmentation

Do not create:

DistributedIR
DistributedTaskIR
DistributedNodeIR
DistributedNetworkIR
DistributedTopologyIR
DistributedReplicationIR

merely to mirror source files.

Use the canonical compiler IR architecture.

Domain-specific semantic information may exist as structured metadata or semantic models, but the grammar directory must not dictate a fragmented IR architecture.

---

92. Test Ownership

Tests should primarily live under the repository's canonical testing structure.

Recommended:

grammar/tests/distributed/

with categories:

grammar/tests/distributed/
├── lexical/
├── parser/
├── ast/
├── semantic/
├── resources/
├── capabilities/
├── effects/
├── contracts/
├── policies/
├── provenance/
├── concurrency/
├── networking/
├── quantum/
├── hybrid/
├── hardware/
├── ai/
├── positive/
├── negative/
├── boundary/
├── scalability/
├── compatibility/
└── determinism/

Do not create a second competing test hierarchy inside "grammar/distributed/" unless repository-wide test architecture explicitly requires it.

---

93. Positive Tests

Every distributed feature requires valid examples covering:

- minimal form;
- ordinary form;
- nested form;
- parameterized form;
- expression-bearing form;
- cross-domain form;
- resource-aware form;
- capability-aware form;
- policy-aware form.

---

94. Negative Tests

Every feature requires tests for:

- malformed syntax;
- missing delimiters;
- malformed expressions;
- invalid structural combinations;
- duplicate constructs where forbidden;
- illegal nesting;
- malformed lists;
- malformed references.

Semantic invalidity should be tested separately from syntactic invalidity.

---

95. Boundary Tests

Boundary tests must cover:

distributed + concurrency
distributed + networking
distributed + resources
distributed + effects
distributed + policies
distributed + contracts
distributed + provenance
distributed + classical
distributed + quantum
distributed + hybrid
distributed + HDL
distributed + hardware
distributed + AI
distributed + interoperability
distributed + execution

---

96. Scalability Tests

Scalability tests must prove architectural absence of artificial limits.

Tests should use symbolic structures and generated collections rather than treating one large number as a maximum.

Test:

one participant
many participants
deep nesting
many tasks
many relationships
many requirements
many capabilities
large dependency graphs
large topology descriptions
large message sets
large partition sets
large deployment descriptions

The test suite must verify that no finite grammar constant has accidentally become a semantic ceiling.

---

97. Determinism Tests

The same token stream must produce the same parse structure under the same language version and grammar configuration.

Tests must detect:

- ambiguous alternatives;
- unstable composition;
- duplicate ownership;
- hidden parser state;
- context-dependent nondeterminism.

---

98. Cross-Domain Integration Test

At least one integration test must combine:

distributed task
+
actor
+
service
+
message
+
communication
+
resource requirement
+
capability requirement
+
effect
+
contract
+
policy
+
topology
+
placement
+
partitioning
+
replication
+
consistency
+
fault tolerance
+
deployment
+
classical computation
+
AI computation
+
quantum computation
+
hardware realization intent

The source must remain one coherent Zamani program.

The pipeline must be traceable:

source
  |
  v
lexer
  |
  v
parser
  |
  v
AST
  |
  v
semantic analysis
  |
  +--> types
  +--> effects
  +--> capabilities
  +--> resources
  +--> contracts
  +--> policies
  +--> provenance
  |
  v
canonical semantic representation
  |
  +--> Classical IR
  +--> quantum::ir
  +--> hardware representation
  |
  v
optimization
  |
  +--> partition
  +--> placement
  +--> routing
  +--> scheduling
  +--> resilience
  |
  v
target realization

---

99. POCO-REAF Test

The distributed subsystem must eventually demonstrate that the same source semantics can be considered for:

tiny embedded target
single CPU
multicore CPU
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC
cluster
federated system
cloud
edge
heterogeneous system
future target

without requiring source-language changes solely because the realization scale changes.

---

100. Resource Failure Semantics

If the program says:

requires memory >= required_memory;

and a target does not satisfy it, the compiler must report an unsatisfied requirement.

It must not silently:

- reduce the requirement;
- change the algorithm;
- discard work;
- reduce precision;
- reduce replicas;
- change topology;
- weaken consistency;
- remove communication;
- alter quantum semantics.

Unless the program explicitly permits adaptation through its policy and semantic contracts.

---

101. Adaptive Distributed Execution

Adaptive execution is permitted only through the universal adaptation architecture.

The semantic flow is:

detect environment
        |
        v
evaluate policy
        |
        v
evaluate capabilities/resources
        |
        v
select permitted realization
        |
        v
validate semantic obligations
        |
        v
record provenance
        |
        v
execute

Adaptation must never mean unrestricted self-modification.

---

102. Simulation

Distributed computation may be simulated.

Simulation belongs to:

grammar/execution/

Distributed grammar provides the distributed workload.

Simulation may model:

- nodes;
- communication;
- topology;
- failures;
- scheduling;
- partitioning;
- resource availability;
- quantum-distributed execution;
- AI distributed execution.

Simulation is an execution strategy, not a second language.

---

103. Reproducibility

Distributed programs may be difficult to reproduce because of:

- scheduling;
- communication order;
- failures;
- resource allocation;
- adaptation;
- randomization.

The language architecture must therefore preserve compatibility with:

effects
policies
provenance
execution
compatibility

Deterministic/reproducible execution requirements belong to those semantic systems.

---

104. Security and Isolation

Distributed programs may cross trust boundaries.

The distributed grammar must therefore integrate with security policies concerning:

capabilities
authorization
sandboxing
network access
foreign calls
native calls
data access
remote execution
deployment
provenance

The grammar itself must remain declarative.

---

105. No Vendor Lock-In

The distributed grammar must not make any vendor, cloud provider, network provider, accelerator provider, operating system or hardware manufacturer part of the universal language.

Vendor-specific functionality belongs in:

dialects/
interoperability/
capabilities/
hardware/
resources/
policies/

where appropriate.

---

106. No Application-Specific Distributed Keyword Explosion

Application concepts should remain libraries, dialects or semantic extensions.

The distributed grammar should not acquire universal keywords for:

robot
payment
vision
sentiment
blockchain
legal
administration
VR
AR

The universal distributed model is:

entity
task
service
message
actor
resource
capability
policy
relationship

Applications build upon those primitives.

---

107. No Fixed Algorithm Inventory

The grammar must not enumerate every distributed algorithm.

Do not make the universal grammar require a closed list of:

consensus algorithms
replication algorithms
routing algorithms
load balancers
schedulers
partitioners
failure detectors
serialization methods

Algorithms are implementation choices or semantic names where explicitly required.

---

108. Generic Extension Mechanism

Future distributed concepts should be introduced through:

qualified names
attributes
metadata
dialects
capabilities
policies
semantic registries

where appropriate.

The preferred extension mechanism is:

new semantic concept
        |
        v
existing syntactic structure
        |
        v
semantic registry / dialect

rather than:

new concept
        |
        v
new universal keyword
        |
        v
rewrite root grammar

---

109. Completion Gate for Every Distributed File

A distributed ".g4" file is not DONE merely because:

ANTLR accepts it

It is DONE only when:

[ ] Purpose defined
[ ] Ownership defined
[ ] Non-ownership defined
[ ] Public rules defined
[ ] Dependencies defined
[ ] Dependency direction validated
[ ] Lexer authority identified
[ ] Name authority identified
[ ] Expression authority identified
[ ] Type authority identified
[ ] AST contract defined
[ ] Semantic contract defined
[ ] Effect contract defined
[ ] Capability contract defined
[ ] Resource contract defined
[ ] Contract integration defined
[ ] Policy integration defined
[ ] Provenance integration defined
[ ] IR destination defined
[ ] Compiler integration defined
[ ] Runtime boundary defined
[ ] Networking boundary defined
[ ] Concurrency boundary defined
[ ] Quantum boundary defined where applicable
[ ] HDL boundary defined where applicable
[ ] Hardware boundary defined where applicable
[ ] Diagnostics defined
[ ] Compatibility defined
[ ] Scalability defined
[ ] Hard-coding audit passed
[ ] Positive tests exist
[ ] Negative tests exist
[ ] Boundary tests exist
[ ] Cross-domain tests exist
[ ] Scalability tests exist
[ ] Determinism tests exist
[ ] Completion criteria satisfied

---

110. Completion Gate for "distributed.g4"

"distributed.g4" is complete only when:

[ ] It is the sole distributed composition root.
[ ] It imports every canonical distributed leaf grammar.
[ ] It does not duplicate leaf syntax.
[ ] It exposes stable public distributed boundaries.
[ ] It does not create a second semantic model.
[ ] It does not define lexer rules.
[ ] It does not define types.
[ ] It does not define generic expressions.
[ ] It does not implement resources.
[ ] It does not implement capabilities.
[ ] It does not implement policies.
[ ] It does not implement effects.
[ ] It does not implement runtime behavior.
[ ] It preserves source ordering.
[ ] It preserves compatibility adapters where required.
[ ] It contains no physical limits.
[ ] It contains no unsafe implementation requirement.
[ ] It passes parser tests.
[ ] It passes ambiguity tests.
[ ] It passes integration tests.

---

111. Completion Gate for the Directory

"grammar/distributed/" is production-ready only when:

specification
    |
    v
lexer
    |
    v
grammar
    |
    v
parser
    |
    v
AST
    |
    v
semantic analysis
    |
    +--> types
    +--> effects
    +--> capabilities
    +--> resources
    +--> contracts
    +--> policies
    +--> provenance
    |
    v
canonical semantic representation
    |
    +--> Classical IR
    +--> quantum::ir
    +--> HDL/hardware representation
    |
    v
optimization
    |
    +--> partitioning
    +--> placement
    +--> routing
    +--> scheduling
    +--> resilience
    |
    v
deployment
    |
    v
runtime / HAL

Every stable distributed feature must have a traceable path through this architecture.

---

112. Production-Readiness Checklist

Before declaring the directory production-ready:

ARCHITECTURE
[ ] README is the directory orchestration contract.
[ ] distributed.g4 is the sole composition root.
[ ] No competing distributed composition root exists.

LEXER
[ ] No duplicate lexer rules.
[ ] No unnecessary distributed keyword explosion.
[ ] Canonical lexer remains authoritative.

GRAMMAR
[ ] Every leaf has one owner.
[ ] No duplicated semantic grammar.
[ ] No grammar cycles.
[ ] Public rules are documented.
[ ] Compatibility aliases delegate to canonical rules.

AST
[ ] Every stable construct has an AST mapping.
[ ] Source spans are preserved.
[ ] Domain-neutral representation is maintained.

SEMANTICS
[ ] Distributed meaning is defined.
[ ] Resource semantics are separate.
[ ] Capability semantics are separate.
[ ] Effects are separate.
[ ] Policies are separate.
[ ] Contracts are separate.
[ ] Provenance is preserved.

IR
[ ] No accidental DistributedIR fragmentation.
[ ] Classical workloads reach Classical IR.
[ ] Quantum workloads reach quantum::ir.
[ ] Hardware workloads reach the hardware representation.

EXECUTION
[ ] Partitioning is downstream.
[ ] Placement is downstream.
[ ] Routing is downstream.
[ ] Scheduling is downstream.
[ ] Resilience is downstream.
[ ] Deployment is downstream.

PORTABILITY
[ ] No vendor lock-in.
[ ] No fixed hardware assumptions.
[ ] No fixed machine assumptions.
[ ] No fixed network assumptions.
[ ] No fixed node count.

SCALABILITY
[ ] No MAX_NODES.
[ ] No MAX_WORKERS.
[ ] No MAX_TASKS.
[ ] No MAX_REPLICAS.
[ ] No MAX_PARTITIONS.
[ ] No MAX_CHANNELS.
[ ] No MAX_MESSAGES.
[ ] No MAX_DEVICES.
[ ] No MAX_NETWORK_SIZE.
[ ] No MAX_MEMORY.
[ ] No MAX_THREADS.
[ ] No equivalent indirect limits.

SAFETY
[ ] Rust 1.97+ supported.
[ ] Rust 2021 supported.
[ ] No unsafe Rust required.
[ ] Grammar has no runtime actions.
[ ] Grammar has no filesystem access.
[ ] Grammar has no network access.
[ ] Grammar has no hardware discovery.
[ ] Grammar has no mutable parser-global state.

TESTING
[ ] Positive tests.
[ ] Negative tests.
[ ] Boundary tests.
[ ] Cross-domain tests.
[ ] Scalability tests.
[ ] Determinism tests.
[ ] Compatibility tests.
[ ] End-to-end integration tests.

---

113. Final Distributed Architecture

The complete architecture is:

                         ZAMANI SOURCE
                              |
                              v
                       CANONICAL LEXER
                              |
                              v
                       CANONICAL PARSER
                              |
                              v
                  DISTRIBUTED COMPOSITION ROOT
                    grammar/distributed/
                              |
        +----------+----------+----------+----------+
        |          |          |          |          |
        v          v          v          v          v
      nodes     processes    actors    services    tasks
        |          |          |          |          |
        +----------+----------+----------+----------+
                              |
        +----------+----------+----------+----------+
        |          |          |          |          |
        v          v          v          v          v
    channels  communication messages  collective replication
        |          |          |          |          |
        +----------+----------+----------+----------+
                              |
        +----------+----------+----------+----------+
        |          |          |          |          |
        v          v          v          v          v
 consistency partitioning placement topology transactions
        |          |          |          |          |
        +----------+----------+----------+----------+
                              |
        +----------+----------+----------+----------+
        |          |          |          |          |
        v          v          v          v          v
 remote     deployment   fault tolerance
 execution
                              |
                              v
                       DOMAIN-NEUTRAL AST
                              |
        +----------+----------+----------+----------+
        |          |          |          |          |
        v          v          v          v          v
       types    effects   capabilities resources policies
        |          |          |          |          |
        +----------+----------+----------+----------+
                              |
                              v
                       CONTRACT ANALYSIS
                              |
                              v
                         PROVENANCE
                              |
                              v
                   SEMANTIC DISTRIBUTED MODEL
                              |
             +----------------+----------------+
             |                |                |
             v                v                v
       Classical IR       quantum::ir      HDL/HW model
             |                |                |
             +----------------+----------------+
                              |
                              v
                         OPTIMIZATION
                              |
             +----------------+----------------+
             |                |                |
             v                v                v
       partitioning       placement         routing
             |                |                |
             +----------------+----------------+
                              |
                              v
                         SCHEDULING
                              |
                              v
                        RESILIENCE
                              |
                              v
                         DEPLOYMENT
                              |
             +----------------+----------------+
             |                |                |
             v                v                v
           CPU/GPU        FPGA/ASIC        QPU
             |                |                |
             +----------------+----------------+
                              |
             +----------------+----------------+
             |                |                |
             v                v                v
          simulator          HPC           cluster
             |                |                |
             +----------------+----------------+
                              |
             +----------------+----------------+
             |                |                |
             v                v                v
          federated         cloud          edge
                              |
                              v
                       FUTURE TARGETS

---

114. The Core POCO-REAF Invariant

The permanent distributed-language invariant is:

«One Zamani program expresses one semantic computation. The realization may scale, partition, place, route, schedule, replicate, recover and execute that computation differently according to requirements, capabilities, resources, policies and available targets.»

Therefore:

ONE PROGRAM
    |
    +--> tiny execution
    |
    +--> single process
    |
    +--> multicore
    |
    +--> GPU
    |
    +--> FPGA
    |
    +--> ASIC
    |
    +--> accelerator
    |
    +--> QPU
    |
    +--> simulator
    |
    +--> HPC
    |
    +--> cluster
    |
    +--> federated system
    |
    +--> cloud
    |
    +--> edge
    |
    +--> heterogeneous system
    |
    +--> future computational substrate

without making today's physical realization into tomorrow's language restriction.

---

115. Final Responsibility Invariant

The permanent separation of responsibilities is:

README
    architecture and integration contracts

LEXER
    lexical vocabulary

GRAMMAR
    source syntax

AST
    source structure

SEMANTICS
    meaning

TYPE SYSTEM
    type correctness

EFFECT SYSTEM
    effect correctness

CAPABILITY SYSTEM
    required/provided capabilities

RESOURCE SYSTEM
    resource requirements and feasibility

CONTRACT SYSTEM
    formal obligations

POLICY SYSTEM
    permitted realization choices

PROVENANCE
    derivation and transformation history

CLASSICAL IR
    classical computational representation

quantum::ir
    quantum computational representation

HDL/HARDWARE REPRESENTATION
    hardware semantics

OPTIMIZER
    semantic-preserving transformation

PARTITIONER
    computation/data partitioning

PLACER
    realization placement

ROUTER
    communication/path realization

SCHEDULER
    execution ordering

RESILIENCE
    failure/recovery realization

DEPLOYMENT
    environment realization

RUNTIME
    execution

HAL
    target interface

HARDWARE/TARGET
    physical capabilities

No responsibility may silently move upward into the grammar.

---

116. Final Definition

"grammar/distributed/" is production-ready when it provides a single, deterministic, open-world, target-independent distributed syntax architecture whose every feature can be traced from:

specification
    ->
lexer
    ->
distributed grammar
    ->
domain-neutral AST
    ->
semantic analysis
    ->
types/effects/capabilities/resources/contracts/policies/provenance
    ->
canonical IR
    ->
partitioning/placement/routing/scheduling/resilience
    ->
deployment
    ->
runtime/HAL
    ->
target

while preserving:

Program_Once
Compile_Once
Run_Everywhere
Run_Anywhere
Run_Forever

and while imposing no artificial finite machine, node, worker, process, task, service, channel, message, replica, partition, topology, device, memory, network or hardware ceiling in the language grammar.

The decisive rule is:

«Resource availability may determine whether and how a distributed program can be realized, but resource availability must never become an accidental limit on what the Zamani language can express.»

The decisive ownership rule is:

«"grammar/distributed/README.md" orchestrates the directory, "distributed.g4" composes its grammars, each leaf grammar owns exactly one syntactic responsibility, and all meaning and realization remain downstream.»

The decisive portability rule is:

«Distributed scale is a realization property, not a source-language limit.»