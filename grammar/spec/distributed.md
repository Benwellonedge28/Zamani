Zamani Distributed Computing Specification

Path: "grammar/spec/distributed.md"
Language: Zamani
Specification role: Normative distributed-computing semantic, integration, scalability, portability, and conformance contract
Status: Production / Normative
Specification version: 1.0
Grammar technology: ANTLR4
Implementation baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Rust safety requirement: Safe Rust only; production Zamani implementation MUST NOT use "unsafe"
Primary portability principle: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)
Scalability principle: From the smallest supported computation to arbitrarily large realizations, limited only by actual resource availability and explicit semantic requirements
Canonical grammar composition root: "grammar/Zamani.g4"
Canonical frontend AST: "src/frontend/ast/"
Canonical quantum semantic boundary: "quantum::ir"
Distributed implementation boundary: distributed semantic analysis → canonical semantic representation → distributed execution metadata → placement/routing/scheduling/deployment/runtime

---

0. Purpose

This document defines the normative semantic contract for distributed computation in Zamani.

It establishes how Zamani represents computation that may execute across:

- one execution context;
- multiple processes;
- multiple workers;
- multiple logical nodes;
- multiple physical machines;
- clusters;
- HPC systems;
- edge systems;
- cloud systems;
- heterogeneous systems;
- accelerator systems;
- quantum/classical systems;
- quantum networks;
- embedded distributed systems;
- future computational substrates.

This specification defines distributed computational meaning and intent.

It does not implement distributed execution.

The fundamental separation is:

Zamani source
    │
    ▼
distributed semantic intent
    │
    ▼
resource / capability analysis
    │
    ▼
canonical semantic representation
    │
    ├── classical semantics
    ├── quantum semantics
    ├── HDL/hardware semantics
    ├── AI/data semantics
    └── distributed semantics
    │
    ▼
canonical/domain IR
    │
    ├── classical IR
    ├── quantum::ir
    └── appropriate hardware/domain IR
    │
    ▼
optimization
    │
    ├── placement
    ├── routing
    ├── scheduling
    ├── replication
    ├── resilience
    └── deployment planning
    │
    ▼
HAL / runtime / deployment

The language therefore describes what distributed computation means, while downstream systems determine how, where, and at what physical scale it is realized.

---

1. Normative Terminology

The following terms are normative.

- MUST — mandatory.
- MUST NOT — prohibited.
- REQUIRED — mandatory.
- SHOULD — recommended unless a documented technical reason exists otherwise.
- SHOULD NOT — normally prohibited unless justified.
- MAY — permitted.
- IMPLEMENTATION-DEFINED — determined by the implementation and documented.
- RESOURCE-DEPENDENT — dependent on available execution resources.
- CAPABILITY-DEPENDENT — dependent on capabilities supplied by a target or execution environment.
- TARGET-DEPENDENT — dependent on explicit target constraints.
- SEMANTICALLY INVALID — violates the language semantics.
- RESOURCE-UNSATISFIABLE — required resources cannot be provided.
- CAPABILITY-UNSATISFIABLE — required capabilities cannot be provided.
- UNREPRESENTABLE — the requested semantic computation cannot be represented by the selected realization.
- OBSERVABLE — potentially visible under the language semantics.
- LOGICAL — source/semantic identity independent of physical realization.
- PHYSICAL — realization-specific identity or property.
- PORTABLE — not unnecessarily tied to a particular realization.
- DISTRIBUTED — semantically capable of involving more than one execution context.
- LOCAL — constrained to one execution context by explicit semantics.
- REMOTE — crossing an execution-context boundary.
- EXECUTION CONTEXT — an abstract semantic locus in which computation or state may be realized.
- PARTICIPANT — an abstract member of a distributed computation.
- PLACEMENT — mapping semantic work or state to execution resources.
- ROUTING — determining communication paths or physical realization paths.
- SCHEDULING — determining execution order and timing subject to constraints.
- REPLICATION — maintaining multiple semantic or implementation realizations of state/work.
- CONSISTENCY — rules governing observations of shared or replicated state.
- FAULT — an execution failure or abnormal condition recognized by the semantic/runtime model.
- RECOVERY — restoration or continuation of a computation after an applicable failure.
- TOPOLOGY — structural relationships among execution contexts or resources.
- TRANSPORT — implementation mechanism used to communicate.
- POLICY — semantic or implementation preference governing realization.
- REQUIREMENT — a condition that must be satisfied.
- CONSTRAINT — a restriction on valid realization.
- CAPABILITY — an ability supplied by a realization.
- PREFERENCE — an optimization preference that does not alter correctness.
- HINT — information usable for optimization but not required for semantic correctness.

---

2. File Completion Contract

This file is complete only when all of the following contracts are defined.

2.1 Purpose

Define the normative semantics of distributed computation in Zamani.

2.2 Owns

This specification owns:

- distributed execution semantics;
- logical execution contexts;
- distributed participants;
- distributed tasks;
- distributed services;
- distributed actors;
- distributed communication semantics;
- message semantics;
- channels;
- distributed dependencies;
- remote execution semantics;
- placement semantics at the logical level;
- replication semantics;
- consistency semantics;
- partition semantics;
- distributed coordination;
- distributed collective operations;
- distributed failure semantics;
- recovery semantics;
- distributed lifecycle semantics;
- distributed state ownership;
- distributed data locality;
- distributed observability;
- distributed determinism;
- distributed resource semantics;
- distributed capability requirements;
- distributed scalability;
- distributed portability;
- distributed security boundaries;
- distributed quantum integration;
- distributed classical integration;
- distributed HDL/hardware integration;
- distributed AI/data integration;
- distributed conformance requirements.

2.3 Does Not Own

This specification does not own:

- lexer implementation;
- parser implementation;
- ANTLR mechanics;
- general identifier syntax;
- general expression syntax;
- general type syntax;
- generic memory ownership semantics;
- generic concurrency syntax;
- physical network protocols;
- physical node discovery;
- hardware discovery;
- device drivers;
- cloud-provider APIs;
- physical topology construction;
- routing algorithms;
- scheduling algorithms;
- placement algorithms;
- resource allocation algorithms;
- replication algorithms;
- consensus algorithms;
- storage engines;
- checkpoint implementation;
- runtime implementation;
- deployment implementation;
- calibration;
- QEC implementation;
- ZQN implementation;
- HAL implementation;
- "quantum::ir" definition.

Those concerns belong to their respective repository contracts.

2.4 Inputs

Distributed semantic analysis consumes:

- parsed distributed syntax;
- general expressions;
- declarations;
- types;
- effects;
- resource requirements;
- capabilities;
- security policies;
- portability constraints;
- execution policies;
- domain semantics;
- quantum semantics where applicable;
- hardware capabilities where applicable;
- target/deployment information.

2.5 Outputs

Distributed semantic analysis produces:

- normalized distributed intent;
- participant relationships;
- execution-context relationships;
- communication requirements;
- dependency relationships;
- locality requirements;
- consistency requirements;
- replication intent;
- failure/recovery requirements;
- placement constraints;
- scheduling constraints;
- resource requirements;
- capability requirements;
- portability classifications;
- security requirements;
- canonical distributed metadata;
- diagnostics.

2.6 Dependencies

This specification integrates with:

grammar/spec/lexical.md
grammar/spec/syntax.md
grammar/spec/semantics.md
grammar/spec/type-system.md
grammar/spec/effects.md
grammar/spec/resources.md
grammar/spec/portability.md
grammar/spec/compatibility.md
grammar/spec/concurrency.md
grammar/spec/networking.md
grammar/spec/security.md
grammar/spec/quantum.md
grammar/spec/hardware.md
grammar/spec/execution.md

Where one of these files is not yet present, this document defines the required integration contract but does not create a second authority for that domain.

2.7 Upstream Contracts

Syntax is supplied by:

grammar/distributed/
grammar/execution/distributed-execution.g4
grammar/memory/distributed-memory.g4
grammar/effects/distributed.g4
grammar/Zamani.g4

The existing distributed grammar is the source-level structural grammar. It intentionally uses open-world qualified names and does not encode finite node, worker, service, replica, or topology limits.

The execution grammar provides the execution-layer composition boundary.

Neither grammar is permitted to redefine this document's semantic meaning.

2.8 Downstream Consumers

Consumers include:

- frontend semantic analysis;
- name resolution;
- type checking;
- effect checking;
- resource analysis;
- capability analysis;
- distributed semantic validation;
- classical IR generation;
- "quantum::ir";
- hardware/HDL lowering;
- distributed execution planning;
- placement;
- routing;
- scheduling;
- replication;
- resilience;
- runtime;
- deployment;
- networking;
- security;
- observability;
- verification;
- testing.

---

3. Repository Authority and Integration

Zamani MUST maintain one distributed semantic model.

The architecture is:

grammar/spec/distributed.md
        │
        │ normative distributed semantics
        ▼
grammar/distributed/*.g4
        │
        │ distributed syntax
        ▼
grammar/execution/distributed-execution.g4
        │
        │ execution composition
        ▼
grammar/Zamani.g4
        │
        ▼
lexer
        │
        ▼
parser
        │
        ▼
src/frontend/ast/
        │
        ▼
structural validation
        │
        ▼
semantic analysis
        │
        ├── types
        ├── effects
        ├── resources
        ├── capabilities
        ├── security
        ├── portability
        └── distributed semantics
        │
        ▼
canonical semantic representation
        │
        ├── classical
        ├── quantum
        ├── HDL/hardware
        ├── AI/data
        └── distributed
        │
        ▼
canonical/domain IR
        │
        ├── classical IR
        ├── quantum::ir
        └── hardware/domain IR
        │
        ▼
verification
        │
        ▼
optimization
        │
        ├── placement
        ├── routing
        ├── scheduling
        ├── replication
        └── resilience
        │
        ▼
deployment
        │
        ▼
runtime

No stage may silently replace distributed semantic intent with a different meaning.

---

4. Relationship to Existing Distributed Grammar Files

4.1 "grammar/distributed/distributed.g4"

This grammar owns distributed source syntax.

It MUST NOT become another semantic specification.

It is responsible for recognizing structural forms such as:

- distributed declarations;
- distributed operations;
- distributed bindings;
- distributed relationships;
- distributed dependencies;
- distributed blocks.

The semantic interpretation of these forms comes from this document.

4.2 "grammar/execution/distributed-execution.g4"

This grammar owns execution-level distributed syntax.

It represents execution intent such as:

- distributed execution declarations;
- logical execution domains;
- abstract participants;
- execution requirements;
- execution constraints;
- execution preferences;
- lifecycle intent;
- placement references;
- scheduling references;
- checkpoint/state intent.

It MUST NOT implement distributed execution.

4.3 "grammar/memory/distributed-memory.g4"

This grammar owns distributed-memory syntax.

Its meaning MUST conform to the distinction:

logical memory semantics
        ≠
physical memory placement

4.4 "grammar/effects/distributed.g4"

Distributed effects MUST be interpreted as semantic effects.

An effect such as communication does not select a physical transport.

4.5 "grammar/Zamani.g4"

"grammar/Zamani.g4" remains the canonical composition root.

This specification MUST NOT introduce another root grammar.

---

5. Distributed Computation Model

A distributed Zamani computation is modeled conceptually as:

D =
    Program
  + Participants
  + Computation
  + State
  + Communication
  + Dependencies
  + Consistency
  + Resource Requirements
  + Capability Requirements
  + Failure Semantics
  + Security Semantics
  + Observability

A realization is:

R =
    D
  + Available Resources
  + Available Capabilities
  + Target Constraints
  + Placement
  + Routing
  + Scheduling
  + Runtime Policy
  + Deployment Policy

The realization MUST preserve the distributed semantic contract.

Therefore:

distributed meaning
    ≠
physical cluster layout

and:

logical participant
    ≠
physical machine

and:

logical communication
    ≠
physical network route

---

6. POCO-REAF for Distributed Computing

Distributed programs MUST follow:

Program_Once
Compile_Once
Run_Everywhere
Anywhere
Forever

A developer SHOULD be able to express:

parallel computation
distributed state
communication
dependencies
replication intent
consistency requirements
failure policy
resource requirements
capabilities
placement constraints

without rewriting the computation merely because the realization changes from:

1 context
→
10 contexts
→
1,000 contexts
→
1,000,000 contexts

provided the required resources and capabilities exist.

The source MUST NOT require a new semantic program merely because:

- more nodes become available;
- fewer nodes are available;
- CPUs are replaced by GPUs;
- GPUs are replaced by other accelerators;
- a local realization becomes a cluster;
- a cluster becomes a cloud deployment;
- the network topology changes;
- the storage implementation changes;
- a different scheduling strategy is selected.

---

7. Meaning of "Distributed"

A computation is distributed when its semantic model permits its work, state, communication, or execution context to span multiple logical execution contexts.

Distributed does not necessarily mean:

- physically remote;
- networked;
- cloud-hosted;
- geographically separated.

For example, a computation may be logically distributed while ultimately being realized on one physical machine.

Conversely, a program may run on multiple machines without exposing distributed semantics to the source program.

The semantic classification therefore belongs to the program model rather than merely to physical deployment.

---

8. Execution Contexts

An execution context is an abstract semantic locus for computation.

A context MAY eventually map to:

- a process;
- a thread;
- a worker;
- a CPU;
- a GPU execution domain;
- an accelerator;
- an FPGA region;
- a QPU;
- a remote service;
- a VM;
- a container;
- an embedded controller;
- a cluster partition;
- a future computational substrate.

The language MUST NOT require any particular mapping.

A logical execution context MUST have an identity that is independent of physical realization.

---

9. Participants

A distributed participant is a logical entity participating in distributed computation.

Examples include:

node
worker
service
actor
task
process
execution_context
partition
stage
agent
device

These are semantic categories.

They MUST NOT automatically imply physical machine identities.

A participant declaration MAY be realized by:

one physical resource
many physical resources
a virtual resource
a remote service
a dynamically created execution context

depending on semantic requirements.

---

10. No Fixed Participant Limits

The language MUST NOT define:

MAX_NODES
MAX_WORKERS
MAX_SERVICES
MAX_ACTORS
MAX_TASKS
MAX_PROCESSES
MAX_PARTITIONS
MAX_EXECUTION_CONTEXTS

The grammar MUST use repetition and composition rather than finite enumeration.

For example:

participant*

or equivalent recursive/compositional structures are valid.

A compiler MAY have implementation resource limits.

Such limits are not language semantics.

If an implementation limit is reached, it MUST issue an explicit diagnostic or defined resource failure.

---

11. Logical Identity Versus Physical Identity

The following distinction is mandatory.

logical node
    ≠
hostname

logical participant
    ≠
machine ID

logical worker
    ≠
CPU ID

logical accelerator
    ≠
PCI address

logical quantum resource
    ≠
PhysicalQubitId

logical channel
    ≠
socket

logical service
    ≠
IP address

Physical identifiers MAY exist in explicit target-specific deployment metadata.

They MUST NOT silently become part of portable distributed semantics.

---

12. Distributed Tasks

A distributed task is a semantic unit of computation.

A task MAY:

- consume inputs;
- produce outputs;
- depend on other tasks;
- communicate;
- access distributed state;
- execute remotely;
- execute concurrently;
- be replicated;
- be migrated;
- be retried when explicitly permitted;
- be checkpointed when semantically checkpointable;
- fail;
- recover.

Task identity MUST be logical.

Task placement is downstream.

---

13. Task Dependencies

A dependency expresses a semantic relationship.

For:

A → B

the arrow means:

B depends on the required semantic result/state established by A.

It does NOT mean:

A is physically connected to B

or:

A executes on node 1
B executes on node 2

or:

A uses network link X

Dependency graphs MUST be allowed to scale without language-level limits.

---

14. Data Dependencies

A data dependency exists when one computation requires data produced or modified by another.

The compiler MUST preserve required dependency semantics.

The optimizer MAY:

- fuse operations;
- move operations;
- replicate data;
- cache data;
- distribute data;
- reorder independent operations;

provided semantic dependencies remain satisfied.

---

15. Communication Semantics

Distributed communication is a semantic operation.

It MAY represent:

- send;
- receive;
- request;
- response;
- publish;
- subscribe;
- broadcast;
- multicast;
- scatter;
- gather;
- reduce;
- all-reduce;
- exchange;
- stream;
- synchronization.

The semantic operation MUST NOT select a physical transport by itself.

For example:

distributed::send(channel, value, destination)

does not mean:

TCP
UDP
QUIC
MPI
RDMA
InfiniBand
HTTP
RPC

The networking subsystem determines a valid realization.

---

16. Message Semantics

A message is semantic data transferred between participants.

A message MUST have:

- logical source where applicable;
- logical destination where applicable;
- payload;
- semantic ordering properties where applicable;
- effect/security metadata where applicable.

A physical packet is not the language-level message.

A compiler MAY transform messages into:

- packets;
- RPC calls;
- shared-memory operations;
- DMA operations;
- hardware channels;
- accelerator transfers;
- other mechanisms.

---

17. Message Ordering

The language MUST distinguish:

- ordered communication;
- unordered communication;
- causal dependency;
- explicit synchronization;
- implementation-defined ordering.

If ordering is semantically observable, the implementation MUST preserve it.

If ordering is explicitly unspecified, the implementation MAY choose an ordering.

An implementation MUST NOT treat unspecified ordering as deterministic unless the semantic contract says so.

---

18. Exactly-Once, At-Least-Once, and At-Most-Once

If the distributed API exposes delivery semantics, they MUST be explicit.

Possible semantic classes include:

at_most_once
at_least_once
exactly_once
best_effort

These are semantic contracts.

They MUST NOT be treated as merely transport configuration.

For "exactly_once", the runtime must preserve the observable semantic contract even if internally it uses:

- retries;
- deduplication;
- logging;
- idempotence;
- transactions;
- checkpointing.

The implementation mechanism is not source semantics.

---

19. Remote Execution

Remote execution is a semantic request that computation may be realized outside the initiating execution context.

It MUST NOT imply a specific protocol.

A remote invocation may eventually be realized through:

- local dispatch;
- IPC;
- RPC;
- message passing;
- shared memory;
- network transport;
- accelerator invocation;
- cloud service invocation;
- another execution mechanism.

The semantic meaning remains the requested computation and its observable effects.

---

20. Remote State

Remote state is logically identified independently of its physical location.

The source language MUST NOT assume that:

remote state = physically addressable memory

Remote state may be realized using:

- distributed memory;
- replicated storage;
- remote services;
- object stores;
- databases;
- state machines;
- message-based state;
- future storage mechanisms.

---

21. State Ownership

Distributed state MUST have a semantic ownership model.

Possible models include:

single-owner
shared
replicated
partitioned
derived
immutable
append-only
transactional
event-sourced

Ownership MUST be compatible with the general Zamani memory and type systems.

The distributed specification MUST NOT redefine the general ownership model.

---

22. Shared State

Shared state is state whose semantic model permits observations or modifications from multiple participants.

Shared state MUST define the required consistency behavior.

A compiler MUST NOT infer a stronger consistency guarantee merely because a target provides one.

Likewise, a compiler MUST NOT weaken a required consistency guarantee merely because a target makes it expensive.

---

23. Immutable Distributed State

Immutable distributed data MAY be freely:

- copied;
- cached;
- replicated;
- migrated;
- partitioned;

subject to type, security, ownership, and effect constraints.

Copies MUST preserve semantic value.

---

24. Data Locality

Locality is a semantic property only when the program declares it.

Possible locality concepts include:

local
remote
near
co-located
same-context
same-domain
distributed
portable

Locality preferences SHOULD be distinguishable from locality requirements.

For example:

prefer local(data)

does not mean:

must execute locally

whereas:

require local(data)

may be a semantic constraint.

---

25. Placement

Placement is the mapping:

logical computation/state
        ↓
realization resource

Placement MUST occur downstream from source parsing.

A placement planner MAY choose:

- one context;
- multiple contexts;
- replicated contexts;
- heterogeneous resources;
- dynamically allocated contexts.

Placement MUST satisfy semantic requirements.

Placement MUST NOT alter logical identity.

---

26. Placement Constraints

A program MAY declare constraints such as:

same_context
different_context
co_located
separated
near
within_domain
requires_capability
requires_locality

The exact syntax belongs to:

grammar/distributed/placement.g4
grammar/execution/placement.g4
grammar/resources/
grammar/hardware/

This specification owns their semantic interpretation.

---

27. Dynamic Placement

Distributed programs MAY allow placement to be decided at runtime.

Dynamic placement is valid when:

- semantic requirements remain satisfiable;
- observable behavior remains valid;
- security constraints remain satisfied;
- required locality remains satisfied.

Dynamic placement MUST NOT introduce nondeterminism where the source program requires deterministic placement behavior.

---

28. Replication

Replication means that multiple realizations represent the same logical state or computation.

Replication MAY be used for:

- availability;
- fault tolerance;
- performance;
- locality;
- read scaling;
- computation scaling.

Replication factor MUST NOT be a universal compile-time ceiling.

A requested replication quantity MAY be:

- constant;
- symbolic;
- parameterized;
- data-dependent;
- resource-dependent;
- dynamically negotiated.

---

29. Replication Semantics

Replication MUST preserve logical identity.

For example:

logical state S
    ├── realization A
    ├── realization B
    └── realization C

does not create three independent source-level states unless explicitly declared.

The semantic layer MUST distinguish:

replica

from:

independent copy

---

30. Replication and Consistency

Replication MUST be interpreted together with consistency.

The following are distinct:

replication without shared mutation
replicated immutable state
eventual consistency
causal consistency
strong consistency
transactional consistency
application-defined consistency

The language MUST NOT assume that replication automatically implies a specific consistency model.

---

31. Consistency

Consistency defines which observations and updates are valid.

A distributed implementation MAY support consistency models such as:

- linearizable;
- sequential;
- causal;
- eventual;
- session;
- transactional;
- application-defined.

The semantic model MUST preserve the declared consistency contract.

A weaker realization MUST NOT silently satisfy a stronger source requirement.

---

32. Consensus

Consensus is a distributed coordination mechanism.

Consensus syntax, when present, expresses semantic intent.

The language does not mandate:

- Raft;
- Paxos;
- PBFT;
- HotStuff;
- a particular quorum algorithm;
- a particular transport;
- a particular membership algorithm.

The selected implementation is downstream.

Consensus requirements MUST be validated against available capabilities.

---

33. Partitioning

Partitioning divides logical data or work into independently addressable semantic regions.

Partitions MUST remain logical.

The compiler/runtime MAY map them to:

- nodes;
- workers;
- GPUs;
- memory domains;
- storage devices;
- QPUs;
- FPGA regions;
- other execution resources.

The source program MUST NOT need rewriting when the partition realization changes.

---

34. Sharding

Sharding is a form of partitioning where data or work is distributed across independent semantic shards.

Shard count MUST NOT be language-limited.

A program MAY express:

shard(data)

without specifying a fixed number of physical shards.

A shard count requirement, if semantically necessary, is a resource/semantic property rather than a grammar-level maximum.

---

35. Dynamic Scaling

Distributed computation MAY scale dynamically.

Scaling may change:

- participant count;
- worker count;
- replica count;
- partition count;
- execution capacity;
- memory capacity;
- communication capacity.

Scaling MUST preserve semantic correctness.

The program MUST NOT assume that additional resources automatically alter the meaning of its result.

---

36. Elasticity

Elasticity means that realization resources may expand or contract during execution.

An elastic program MAY specify:

minimum capability
preferred capability
maximum desired capability
scaling policy

The exact resource syntax belongs to the resource specification.

This file defines only the distributed semantic consequences.

---

37. Scaling to Available Resources

The distributed semantic model MUST support:

tiny workload
small workload
medium workload
large workload
very large workload
resource-maximal workload

without requiring distinct source programs.

The implementation MAY parallelize more aggressively when more resources are available.

It MUST NOT change observable semantics merely because more resources exist.

---

38. "Infinity" and Unbounded Scaling

"Infinite scale" means:

«no artificial finite limit is imposed by the language model.»

It does NOT mean:

«physical hardware is infinite.»

Therefore:

language bound → none by design
physical bound → actual available resources

The compiler MAY reject a program because:

- memory is insufficient;
- communication capacity is insufficient;
- required accelerator capability is unavailable;
- target precision is insufficient;
- execution-time constraints cannot be met;
- deployment policy prohibits the requested realization.

Such failure MUST be explicit.

---

39. No Artificial Distributed Limits

The following MUST NOT be universal language constants:

MAX_NODES
MAX_WORKERS
MAX_PROCESSES
MAX_ACTORS
MAX_SERVICES
MAX_TASKS
MAX_CHANNELS
MAX_MESSAGES
MAX_REPLICAS
MAX_SHARDS
MAX_PARTITIONS
MAX_REGIONS
MAX_NETWORKS
MAX_ENDPOINTS
MAX_EXECUTION_CONTEXTS
MAX_TIMELINES

The same rule applies to disguised finite grammar enumerations.

For example:

node0
node1
...
node127

MUST NOT be the only representable node model.

---

40. Program Constants Versus Language Limits

The prohibition on hard-coded implementation limits does not prohibit program constants.

This is valid:

let workers = 128;

if "128" is part of the program's semantic intent.

This is also valid:

replicate state 8 times;

if the program genuinely requires eight replicas.

What is prohibited is:

the compiler accepts at most 128 workers

when 128 is merely an implementation assumption.

---

41. Resource Requirements

Distributed programs MAY require abstract resources.

Examples:

requires distributed.execution
requires communication.capacity >= required_capacity
requires memory >= required_memory
requires capability("collective.reduce")
requires capability("persistent.state")

The resource system owns the formal resource taxonomy.

Distributed semantics consume it.

---

42. Requirement Versus Preference

The implementation MUST distinguish:

requirement
constraint
capability
preference
hint

For example:

require distributed.execution

is different from:

prefer distributed.execution

The first is a semantic requirement.

The second is an optimization preference.

Ignoring a preference MAY be valid.

Ignoring a requirement is not valid.

---

43. Capability Model

Capabilities are abstract.

Examples:

distributed.execution
distributed.messaging
distributed.replication
distributed.consensus
distributed.persistence
distributed.collective
distributed.dynamic_scaling
distributed.checkpoint
distributed.recovery
distributed.remote_execution
distributed.quantum_communication

Capability names MUST NOT encode vendor-specific implementation identity unless explicitly inside a target-specific extension.

---

44. Capability Negotiation

A compiler/runtime MAY negotiate capabilities with the target environment.

The negotiation process MUST preserve source semantics.

If a required capability is unavailable:

CAPABILITY-UNSATISFIABLE

MUST be reported.

The implementation MUST NOT silently substitute an incompatible capability.

---

45. Transport Independence

The source language MUST NOT require a particular transport for ordinary portable distributed communication.

The realization MAY choose:

shared memory
IPC
TCP
UDP
QUIC
RPC
MPI
RDMA
InfiniBand
custom transport
future transport

subject to semantic requirements.

Transport selection belongs downstream.

---

46. Topology Independence

The source language MUST NOT require a fixed physical topology unless explicitly expressed as a target constraint.

Portable distributed code MUST NOT depend on:

- fixed node adjacency;
- fixed number of links;
- fixed network diameter;
- fixed routing paths;
- fixed rack structure;
- fixed geographic layout.

Logical communication graphs MAY be specified.

Physical realization remains downstream.

---

47. Logical Communication Graph

A program MAY define:

A communicates with B
B depends on C
D broadcasts to group G

The semantic graph is logical.

A runtime MAY realize the graph through:

direct links
routing
relays
collective operations
replication
multicast
broadcast trees
shared memory

without changing source meaning.

---

48. Failure Model

Distributed failures MUST be represented explicitly when observable.

Possible failures include:

participant_unavailable
communication_failure
timeout
resource_exhaustion
state_loss
state_corruption
partition
capability_loss
deployment_failure
execution_failure

The exact runtime error taxonomy belongs to the runtime specification.

This specification defines how distributed semantics interact with those failures.

---

49. Failure Is Not Silent Success

A distributed operation MUST NOT silently succeed when its required semantic effect did not occur.

For example:

send(message)

MUST NOT be treated as successful merely because the compiler emitted transport code.

If the semantic contract requires delivery and delivery fails, the result MUST reflect failure according to the declared delivery semantics.

---

50. Failure Handling

A program MAY declare failure behavior such as:

retry
recover
fail
escalate
fallback
recompute
checkpoint
restore

The language semantics define the requested policy.

The resilience subsystem implements it.

This is consistent with the repository's broader separation between semantic intent and resilience implementation.

---

51. Retry Semantics

Retry is not automatically safe.

A retry MUST respect:

- idempotence;
- side effects;
- transaction semantics;
- exactly-once requirements;
- ownership;
- security;
- external effects.

The compiler MUST NOT automatically duplicate an effectful operation merely because it failed.

---

52. Recovery

Recovery restores a computation to a valid semantic state.

Recovery MAY use:

- checkpoint;
- replay;
- recomputation;
- replica;
- alternate resource;
- alternate route;
- alternate execution context.

The selected mechanism is implementation-defined.

Recovery MUST preserve source semantics.

---

53. Checkpointing

Checkpointing is semantic only when explicitly supported.

The language MUST distinguish:

checkpointable state

from:

arbitrary execution state

Not every state is serializable.

A distributed implementation MUST NOT claim successful checkpointing when required state cannot be reconstructed.

---

54. Migration

A logical computation or state MAY be migrated.

Migration MUST preserve:

- logical identity;
- required state;
- ownership;
- security;
- consistency;
- observable semantics.

Migration does not imply a particular transport or physical mechanism.

---

55. Concurrency Integration

Distributed execution is a specialization/composition of concurrency, not a replacement for it.

The distributed model MUST integrate with:

grammar/concurrency/
grammar/spec/concurrency.md

The semantic distinction is:

concurrency
    = potentially overlapping computation

distributed computation
    = computation across logical execution contexts

A distributed computation may also be concurrent.

A concurrent computation may remain entirely local.

---

56. Determinism

Distributed execution MUST distinguish:

1. deterministic computation;
2. explicit nondeterminism;
3. unspecified ordering;
4. implementation-defined scheduling.

For deterministic programs, changing:

- node count;
- worker count;
- physical placement;
- routing;
- scheduling;

MUST NOT change observable results.

Where floating-point or reduction semantics are sensitive to reassociation, the language MUST define the applicable numerical semantics before allowing the compiler to claim deterministic equivalence.

---

57. Distributed Reduction

Reductions are especially important.

A reduction MAY be:

sum
product
min
max
logical_and
logical_or
custom associative operation

The compiler MAY parallelize a reduction only when the operation's semantic properties permit it.

For floating-point operations, the implementation MUST respect the numerical semantics specified by the type/numerical contract.

---

58. Collective Operations

Collectives include:

broadcast
scatter
gather
reduce
all_reduce
all_gather
barrier
exchange

A collective is a logical operation.

It does not require a specific physical collective algorithm.

The implementation MAY select:

- tree;
- ring;
- hierarchical;
- topology-aware;
- hardware-assisted;
- future algorithms.

---

59. Synchronization

Synchronization semantics MUST be explicit.

Possible synchronization concepts include:

barrier
fence
await
join
dependency
transaction boundary
consistency boundary

A synchronization operation MUST NOT be removed by optimization if it is semantically observable.

---

60. Distributed Services

A service is a logical computational provider.

A service MUST be identified semantically rather than solely by:

- host;
- IP;
- port;
- process ID.

Service discovery belongs downstream.

A service MAY be realized locally or remotely.

---

61. Service Discovery

Service discovery is a runtime/deployment capability.

Source syntax MAY express:

requires service capability
requires service name
requires service contract

but MUST NOT require a particular discovery implementation unless explicitly target-specific.

---

62. Service Contracts

A distributed service contract MUST specify semantic properties such as:

- inputs;
- outputs;
- types;
- effects;
- errors;
- capabilities;
- security;
- consistency;
- availability requirements where semantic.

It MUST NOT require the service to be implemented in a particular programming language unless interoperability explicitly requires it.

---

63. Actors

Actors are logical participants with isolated state and message-driven interaction.

Actor semantics MUST integrate with the concurrency/effects systems.

The language MUST NOT assume:

one actor = one OS thread

or:

one actor = one machine

An actor MAY be:

- co-located;
- distributed;
- migrated;
- replicated;

when its semantic contract permits.

---

64. Distributed Dataflow

Dataflow graphs MAY represent distributed computations.

Nodes represent semantic operations.

Edges represent semantic data dependencies.

The graph MUST remain independent of physical topology.

Optimization MAY fuse or repartition graph nodes while preserving semantics.

---

65. Pipelines

A distributed pipeline is an ordered sequence of semantic stages.

Stages MAY execute:

- sequentially;
- concurrently;
- in parallel;
- across different resources.

Pipeline semantics MUST remain independent of stage placement.

---

66. Map / Reduce / Partition Computation

Zamani SHOULD support generic distributed patterns without making them machine-specific.

Conceptually:

map
partition
shuffle
reduce

These are semantic patterns.

The runtime MAY implement them using any suitable realization.

No fixed partition count is permitted.

---

67. Distributed Transactions

Where transaction semantics are supported, the language MUST define:

- transaction scope;
- atomicity requirements;
- consistency requirements;
- isolation requirements;
- durability requirements;
- failure behavior.

The grammar MUST NOT imply a particular transaction protocol.

---

68. Event-Driven Distributed Computation

Events MAY represent distributed effects.

An event has semantic identity and payload.

The implementation MAY realize event delivery using:

- queues;
- streams;
- brokers;
- direct messages;
- shared state;
- hardware event systems.

The source semantics remain independent of the implementation.

---

69. Streaming

A distributed stream is a potentially unbounded sequence of values/events.

The language MUST NOT impose a finite stream length.

A stream MAY be:

- local;
- distributed;
- replicated;
- partitioned;
- ordered;
- unordered;
- replayable.

The semantics of ordering and delivery MUST be explicit.

---

70. Backpressure

Backpressure is a resource/flow-control property.

A distributed stream MAY declare a backpressure requirement or policy.

The actual mechanism belongs to runtime/networking.

The language MUST distinguish:

semantic boundedness

from:

implementation buffer size

---

71. Distributed Memory

Distributed memory integrates with:

grammar/memory/
grammar/memory/distributed-memory.g4

The model MUST distinguish:

logical address
physical address
logical state
physical storage location

A portable program MUST NOT depend on physical addresses.

Memory capacity is a resource property.

---

72. Distributed Persistence

Persistence is an effect/capability.

A program MAY require:

persistent.storage

without selecting:

disk X
database Y
storage provider Z

unless explicitly target-specific.

---

73. Distributed Security

Distributed execution MUST integrate with:

grammar/security/
grammar/spec/security.md

Security properties may include:

- authentication;
- authorization;
- confidentiality;
- integrity;
- isolation;
- provenance;
- trust;
- secure communication;
- capability restrictions.

Security requirements MUST survive lowering.

They MUST NOT be discarded during placement or optimization.

---

74. Capability-Based Distributed Security

A distributed participant SHOULD receive only the capabilities required for its semantic role.

The source-level capability model MUST be distinct from physical credential storage.

The language MUST NOT expose secrets merely because a remote service exists.

---

75. Provenance

Distributed execution MAY generate provenance metadata.

Provenance SHOULD identify:

- logical computation;
- source version;
- semantic configuration;
- execution realization;
- relevant transformations;
- resource/capability environment.

Provenance MUST NOT require hard-coded physical topology in portable source.

---

76. Observability

Distributed programs MAY expose:

- metrics;
- tracing;
- logs;
- events;
- execution state;
- resource observations.

Observability is an effect.

Instrumentation MUST NOT alter semantics except where explicitly specified.

---

77. Quantum Integration

Distributed quantum computation MUST integrate with the existing quantum architecture.

The boundary is:

distributed semantics
        │
        ▼
quantum semantic representation
        │
        ▼
quantum::ir
        │
        ▼
routing
        │
        ▼
scheduling
        │
        ▼
QEC / resilience / ZQN
        │
        ▼
HAL
        │
        ▼
physical realization

Distributed grammar/specification MUST NOT create a second quantum IR.

---

78. Quantum State Identity

A logical quantum state MUST NOT automatically be treated as freely copyable distributed data.

Quantum state movement, teleportation, entanglement distribution, measurement, and classical feed-forward have their own semantic rules.

Distributed syntax may express intent.

Quantum semantic analysis determines legality.

"quantum::ir" remains canonical.

---

79. Distributed Quantum Communication

Distributed quantum communication MAY involve:

- logical qubit movement;
- entanglement;
- teleportation;
- distributed gates;
- measurement;
- classical feed-forward;
- quantum channels.

The distributed specification MUST NOT define:

- "QubitId";
- "PhysicalQubitId";
- gate enums;
- pulse definitions;
- calibration;
- QEC algorithms;
- ZQN noise models.

Those remain owned by the quantum subsystem.

---

80. Distributed Quantum Resources

Quantum distributed resource requirements MAY include abstract requirements such as:

quantum.communication
quantum.entanglement
quantum.measurement
quantum.remote_operation

The actual QPU count, qubit count, topology, coupling, calibration, and physical assignment belong downstream.

---

81. Classical Integration

Distributed classical computation integrates with:

grammar/classical/
grammar/concurrency/
grammar/data/
grammar/resources/

The distributed layer MUST NOT redefine:

- arithmetic;
- scalar types;
- vector types;
- tensor types;
- functions;
- loops;
- ordinary control flow.

It adds distribution semantics around those computations.

---

82. HDL and Hardware Integration

Distributed HDL/hardware computation MAY express:

- distributed accelerators;
- hardware pipelines;
- hardware nodes;
- interconnect intent;
- distributed memory;
- accelerator groups.

Physical implementation remains owned by:

grammar/hdl/
grammar/hardware/
grammar/resources/

The distributed layer MUST NOT create a competing hardware topology model.

---

83. AI and Data Integration

Distributed AI/data workloads MAY express:

- distributed training;
- distributed inference;
- parameter distribution;
- model partitioning;
- dataset partitioning;
- data parallelism;
- model parallelism;
- pipeline parallelism;
- federated computation;
- distributed agents.

Framework-specific implementations MUST remain outside the language core.

---

84. Distributed AI Scaling

A model MUST NOT require source rewriting merely because:

one accelerator
→
many accelerators
→
many nodes

provided the algorithm's semantic requirements remain satisfied.

The compiler/runtime MAY choose:

- data parallelism;
- model parallelism;
- tensor parallelism;
- pipeline parallelism;
- hybrid parallelism.

These are realization strategies unless explicitly part of the source semantic contract.

---

85. Distributed Scheduling

Scheduling belongs downstream.

Distributed source MAY express:

- dependency constraints;
- deadlines;
- ordering;
- priority;
- resource requirements;
- synchronization;
- latency requirements.

The scheduler determines actual execution order and placement.

Existing scheduling systems MUST remain responsible for scheduling rather than being duplicated by the grammar.

---

86. Distributed Routing

Routing determines how logical communication becomes physical communication.

The distributed specification provides logical communication requirements.

Routing may select:

- direct links;
- intermediate nodes;
- hierarchical routes;
- topology-aware paths;
- alternate routes.

Physical routing MUST NOT be encoded into portable source unless explicitly target-specific.

---

87. Distributed Optimization

Optimization MAY transform distributed computation through:

- task fusion;
- communication elimination;
- replication;
- caching;
- batching;
- partitioning;
- vectorization;
- accelerator offload;
- collective selection;
- topology-aware execution.

Optimization MUST preserve semantic behavior.

---

88. Distributed Resilience

Resilience consumes:

- failure requirements;
- recovery policies;
- replication semantics;
- checkpoint intent;
- retry semantics;
- capability requirements.

The distributed specification does not implement resilience.

The resilience subsystem remains responsible for orchestration.

---

89. ZQN Integration

Where distributed quantum computation encounters faults/noise:

distributed intent
    ↓
quantum::ir
    ↓
ZQN fault/noise semantics
    ↓
QEC
    ↓
routing/scheduling/resilience

Distributed syntax MUST NOT duplicate ZQN.

ZQN remains responsible for quantum fault/noise semantics.

---

90. HAL Integration

HAL remains responsible for translating abstract capability requirements into target/device capabilities.

Distributed source MAY require:

distributed.execution
quantum.communication
accelerator.compute
persistent.storage

HAL/runtime/deployment determines whether and how those capabilities are realized.

Source-level distributed semantics MUST NOT contain provider-specific HAL logic.

---

91. Resource Exhaustion

If required resources are unavailable, the implementation MUST produce an explicit result.

Possible classifications include:

resource_unsatisfiable
capability_unsatisfiable
placement_unsatisfiable
scheduling_unsatisfiable
communication_unsatisfiable
consistency_unsatisfiable
security_unsatisfiable

The implementation MUST NOT silently:

- reduce the computation;
- drop tasks;
- reduce replica count;
- weaken consistency;
- weaken security;
- reduce precision;
- reduce quantum resources;
- discard messages.

unless such adaptation is explicitly permitted by the program semantics.

---

92. Graceful Adaptation

A program MAY declare acceptable adaptation.

For example, conceptually:

prefer parallelism
allow repartitioning
allow migration
allow replication adjustment

Such adaptation is valid only within explicitly defined semantic bounds.

A preference MUST NOT be mistaken for a requirement.

---

93. Semantic Equivalence Across Scale

Suppose:

P₁ = realization of program P on one context
P₂ = realization of program P on many contexts

If the program requires deterministic semantics:

Observable(P₁) == Observable(P₂)

must hold for equivalent inputs and environments.

The implementation may differ internally.

This is the core scaling guarantee.

---

94. Distributed Nondeterminism

Nondeterminism MAY be explicit.

Sources include:

- race-free concurrent scheduling choices;
- distributed message arrival;
- external systems;
- randomness;
- quantum measurement;
- explicitly declared nondeterministic APIs.

If nondeterminism is semantically permitted, the language MUST define its observable bounds.

The implementation MUST NOT claim deterministic behavior beyond those bounds.

---

95. Race Semantics

Data races MUST NOT be silently accepted where the type/effect/concurrency system prohibits them.

Distributed races MUST be represented through the same semantic safety mechanisms as local concurrency where applicable.

A distributed deployment MUST NOT become a loophole around ownership or synchronization rules.

---

96. Memory Safety

All distributed execution semantics MUST remain compatible with safe Rust implementation.

Production implementation MUST NOT require Rust "unsafe".

The compiler/runtime MAY use safe abstractions for:

- channels;
- ownership;
- references;
- state transfer;
- serialization;
- task management;
- resource handles.

The language specification MUST NOT require unsafe implementation techniques.

---

97. Serialization

Serialization is a semantic boundary whenever data crosses a representation boundary.

Serialization MUST preserve the semantic value required by the receiving computation.

The language MUST distinguish:

semantic value

from:

wire representation

The compiler/runtime MAY choose the representation.

---

98. Serialization and Quantum State

Quantum states MUST NOT be treated as ordinary serializable classical values.

Any quantum state transfer must obey quantum semantics.

The quantum subsystem owns the legality and representation.

Distributed serialization MUST therefore delegate quantum-state semantics to the quantum subsystem.

---

99. Versioning

Distributed syntax and semantics MUST be versioned consistently with:

grammar/spec/compatibility.md

A breaking distributed semantic change requires an explicit language-version transition.

Adding a backend, transport, scheduler, or deployment provider is normally not a source-language breaking change.

---

100. Compatibility

Existing valid distributed programs MUST retain their meaning across compatible compiler versions.

A compiler MUST NOT reinterpret:

- dependency semantics;
- delivery semantics;
- consistency semantics;
- failure semantics;
- locality requirements;
- resource requirements;

without an explicit language-version transition.

---

101. Diagnostics

Distributed diagnostics MUST be:

- deterministic;
- source-located;
- structured;
- actionable;
- independent of machine topology;
- independent of hash-map iteration;
- independent of provider ordering.

Diagnostics SHOULD identify:

feature
source span
semantic requirement
unsatisfied condition
available capability
requested capability
affected participant
affected dependency
possible realization boundary

---

102. Required Diagnostic Classes

At minimum, semantic analysis SHOULD distinguish:

DISTRIBUTED_INVALID_DECLARATION
DISTRIBUTED_UNKNOWN_ENTITY
DISTRIBUTED_INVALID_DEPENDENCY
DISTRIBUTED_INVALID_COMMUNICATION
DISTRIBUTED_INVALID_CONSISTENCY
DISTRIBUTED_INVALID_REPLICATION
DISTRIBUTED_INVALID_LOCALITY
DISTRIBUTED_RESOURCE_UNSATISFIABLE
DISTRIBUTED_CAPABILITY_UNSATISFIABLE
DISTRIBUTED_PLACEMENT_UNSATISFIABLE
DISTRIBUTED_SCHEDULING_UNSATISFIABLE
DISTRIBUTED_SECURITY_VIOLATION
DISTRIBUTED_SERIALIZATION_UNSUPPORTED
DISTRIBUTED_RECOVERY_UNSUPPORTED
DISTRIBUTED_CHECKPOINT_UNSUPPORTED
DISTRIBUTED_QUANTUM_SEMANTIC_VIOLATION
DISTRIBUTED_PORTABILITY_VIOLATION

Exact diagnostic identifiers belong to the repository-wide diagnostic contract.

---

103. AST Contract

Every distributed syntax construct accepted by the parser MUST map to the domain-neutral frontend AST.

The AST MUST preserve enough information to represent:

- distributed declaration identity;
- participant identity;
- operation identity;
- qualified names;
- arguments;
- dependencies;
- clauses;
- resource requirements;
- capability requirements;
- constraints;
- preferences;
- hints;
- source spans;
- nested structure.

The AST MUST NOT prematurely encode:

- physical node IDs;
- CPU IDs;
- GPU IDs;
- QPU IDs;
- physical qubit IDs;
- transport-specific state;
- scheduler-specific state.

---

104. AST Independence

The distributed AST representation MUST remain independent of:

- LLVM;
- QIR;
- MLIR;
- vendor APIs;
- CUDA;
- ROCm;
- MPI;
- a specific cloud provider;
- a specific scheduler;
- a specific network protocol;
- physical topology.

Interoperability belongs downstream.

---

105. Semantic Model Contract

After AST construction, distributed semantic analysis MUST resolve:

names
types
effects
resources
capabilities
ownership
dependencies
communication
consistency
replication
locality
security
failure semantics
portability

The semantic model MUST be target-independent unless the source explicitly declares target dependence.

---

106. Canonical IR Integration

Distributed semantics MAY be represented in canonical/domain IR metadata.

The IR boundary MUST preserve:

- logical identity;
- dependencies;
- effects;
- resources;
- capabilities;
- communication intent;
- placement constraints;
- scheduling constraints;
- failure semantics.

No semantic information may silently disappear during lowering.

---

107. Quantum IR Integration

Quantum distributed operations MUST lower through:

quantum::ir

when they are quantum semantic operations.

The distributed subsystem MUST NOT introduce:

DistributedQuantumIR

as a competing quantum semantic boundary.

Distributed metadata MAY accompany or reference canonical quantum IR.

---

108. Classical IR Integration

Classical computation MUST lower through the repository's canonical classical semantic/IR boundary.

Distributed metadata may annotate or surround classical operations.

The distributed subsystem MUST NOT duplicate classical computation semantics.

---

109. HDL/Hardware IR Integration

Hardware computation MUST lower through the appropriate HDL/hardware semantic boundary.

Distributed metadata may describe:

- placement intent;
- communication intent;
- accelerator relationships;
- hardware partitioning.

The distributed subsystem MUST NOT become a second hardware IR.

---

110. Runtime Contract

The runtime consumes validated distributed semantics.

The runtime is responsible for:

- execution;
- communication;
- participant lifecycle;
- state management;
- failure handling;
- resource monitoring;
- recovery;
- observability.

The runtime MUST NOT infer a different source meaning merely because the deployment differs.

The existing distributed runtime implementation must remain an implementation of this semantic contract rather than a source-language authority.

---

111. Deployment Contract

Deployment transforms logical execution intent into physical realization.

Deployment MAY determine:

nodes
containers
processes
VMs
accelerators
networks
storage
regions
providers

These decisions MUST remain downstream unless explicitly requested as target-specific source semantics.

---

112. Hardware Scaling

A distributed program MAY span:

CPU
GPU
FPGA
ASIC
QPU
accelerator
embedded processor
future computational substrate

The distributed specification MUST remain hardware-neutral.

Hardware capabilities are supplied by the hardware/resource/HAL systems.

---

113. Heterogeneous Distribution

A single distributed program MAY combine:

CPU computation
GPU computation
FPGA computation
QPU computation
network computation
storage computation

The source language MUST preserve a unified semantic model.

Each domain retains ownership of its own semantics.

---

114. Hybrid Quantum-Classical Distribution

A hybrid program MAY contain:

classical distributed computation
        ↓
quantum computation
        ↓
measurement
        ↓
distributed classical decision
        ↓
quantum computation

The semantic boundaries MUST remain explicit.

Quantum state semantics remain quantum semantics.

Classical communication remains distributed/classical semantics.

The integration layer coordinates them without duplicating either domain.

---

115. Distributed HDL Co-Design

A Zamani program MAY describe:

software task
    ↓
hardware accelerator
    ↓
distributed communication
    ↓
software result

The compiler may derive a heterogeneous realization.

The source program MUST NOT need to encode physical interconnect details unless explicitly target-dependent.

---

116. Distributed AI/Data Co-Design

The same semantic model MUST support:

dataset partitioning
model partitioning
distributed training
distributed inference
distributed feature processing
distributed agents
distributed pipelines

The number of workers and accelerators is not a language limit.

---

117. Security Across Boundaries

Cross-context operations MUST preserve security semantics.

A distributed call MUST NOT automatically grant:

- filesystem access;
- memory access;
- hardware access;
- network access;
- secret access;
- privileged capabilities.

Capability and security analysis MUST happen before realization.

---

118. Side Effects

A distributed side effect MUST be represented through the effect system.

The optimizer MUST NOT duplicate an effectful distributed operation unless its semantics permit duplication.

For example:

send()

cannot automatically be transformed into:

send()
send()

unless the communication semantics explicitly permit that transformation.

---

119. Idempotence

An operation MAY be declared or inferred idempotent only under the appropriate semantic rules.

Idempotence can enable safe:

- retries;
- replication;
- speculative execution;
- recomputation.

The compiler MUST NOT assume idempotence merely because an operation is distributed.

---

120. Speculative Distributed Execution

Speculative execution MAY be used when:

- duplicated computation is semantically safe;
- side effects are controlled;
- results can be reconciled;
- resource usage remains permitted.

Speculation MUST NOT change observable behavior.

---

121. Cancellation

Distributed cancellation MUST preserve semantic guarantees.

Cancellation MAY be:

local
task-scoped
group-scoped
pipeline-scoped
computation-scoped

The runtime implements cancellation.

The semantic layer defines which effects and state transitions cancellation may interrupt.

---

122. Timeouts

Timeouts are semantic only when explicitly declared.

A timeout SHOULD be represented as a semantic constraint rather than an implementation-specific timer assumption.

The implementation MAY use:

- wall-clock timers;
- logical clocks;
- distributed deadlines;
- hardware timers.

---

123. Clocks

Distributed computation MUST NOT assume synchronized physical clocks unless explicitly required.

The semantic model SHOULD distinguish:

logical ordering

from:

physical time

Where temporal semantics are required, the time model MUST be explicit.

---

124. Ordering Without Global Time

Distributed dependencies MAY be expressed without requiring a global clock.

For example:

A happens-before B

is a semantic ordering relationship.

It does not imply that all participants share a physical timestamp.

---

125. Geographical Distribution

Geographic location MAY be a resource/constraint property.

Examples:

region
zone
distance
jurisdiction
latency domain

Portable programs SHOULD use abstract properties.

Physical coordinates belong to deployment/target-specific layers.

---

126. Edge and Cloud

The same distributed semantic model MUST support:

edge
cloud
on-premises
HPC
embedded
hybrid cloud
multi-cloud

The source program SHOULD remain unchanged when moving among compatible realizations.

Provider-specific APIs belong to interoperability/deployment layers.

---

127. Embedded Distribution

Distributed computation may exist inside embedded systems.

The semantic model MUST NOT assume:

- an operating system;
- a process model;
- virtual memory;
- a conventional network stack.

The realization may be minimal while preserving the same distributed semantic contract.

---

128. HPC Integration

Distributed HPC workloads MAY use:

- data parallelism;
- task parallelism;
- collectives;
- distributed memory;
- accelerator offload.

The language MUST NOT encode a fixed MPI rank count or machine topology as universal semantics.

---

129. Future Distributed Architectures

The distributed grammar MUST remain open to future concepts.

Examples may include:

new transport
new accelerator
new execution context
new collective
new consistency model
new topology
new storage model
new quantum communication primitive

Unknown future names MAY be syntactically representable through the existing open-world qualified-name model.

Semantic analysis determines whether a feature is:

stable
implemented
experimental
vendor-specific
deprecated
unknown

---

130. Open-World Rule

The distributed language MUST prefer:

qualified semantic names
+
structured arguments
+
typed expressions
+
capabilities

over an ever-growing closed list of keywords.

For example:

distributed::future_protocol(...)

may be syntactically representable without adding a new global keyword.

Semantic validation determines whether it is supported.

---

131. Unknown Operations

An unknown distributed operation MUST NOT silently become a no-op.

It MUST result in a structured semantic diagnostic unless the language explicitly permits dynamic dispatch for that operation.

The compiler MUST NOT emit comments as a substitute for implementing or rejecting an operation.

---

132. Dynamic Dispatch

If distributed operations support dynamic dispatch, the semantic model MUST preserve:

- operation identity;
- argument types;
- effects;
- capabilities;
- errors;
- security;
- resource requirements.

Dynamic dispatch MUST NOT become an uncontrolled semantic escape hatch.

---

133. Interoperability

Distributed systems MAY interoperate with:

- RPC;
- MPI;
- message brokers;
- databases;
- object stores;
- cloud APIs;
- operating-system services;
- foreign languages;
- hardware services.

Interoperability syntax belongs to:

grammar/interoperability/

The distributed semantic layer describes the required semantic contract.

---

134. Provider Independence

Portable distributed programs MUST NOT depend on a provider-specific service unless explicitly declared.

A provider-specific deployment MAY be selected downstream.

A source-level provider dependency MUST be represented as target-dependent semantics.

---

135. Distributed Compilation

The compiler MAY perform:

partitioning
specialization
fusion
replication
communication elimination
serialization selection
placement planning
scheduling
target selection

during compilation.

The resulting transformation MUST preserve semantic meaning.

---

136. Distributed Runtime Compilation

A runtime MAY perform additional specialization when resource availability is known only at runtime.

This does not violate POCO-REAF provided:

source semantics remain unchanged

and runtime specialization is derived from the same semantic contract.

---

137. Compile Once

The ideal compilation boundary is:

source
 ↓
frontend
 ↓
semantic analysis
 ↓
canonical representation
 ↓
portable compiled artifact

Target realization may then occur without reparsing or rewriting source.

A backend MAY specialize the artifact for a target.

---

138. Cacheability and Reproducibility

Distributed compilation SHOULD support deterministic artifacts where the semantic inputs are identical.

The following MUST NOT alter semantic compilation unexpectedly:

- hash-map iteration order;
- physical node discovery order;
- network discovery order;
- provider enumeration order;
- thread scheduling during compilation.

---

139. Provenance of Realization

A realization MAY record:

compiler version
language version
source identity
semantic configuration
target capabilities
resource environment
placement
routing
scheduling
runtime version

This metadata MUST NOT be confused with source semantics.

---

140. Formal Portability Condition

Let:

P = distributed program semantics
E₁ = execution environment 1
E₂ = execution environment 2
R₁ = realization strategy 1
R₂ = realization strategy 2

Then:

Realize(P, E₁, R₁)

and:

Realize(P, E₂, R₂)

are valid portable realizations when:

Requirements(P) ⊆ Capabilities(Eᵢ)

and:

Constraints(P, Eᵢ, Rᵢ) are satisfied

and:

ObservableSemantics(P, E₁, R₁)
    ≡
ObservableSemantics(P, E₂, R₂)

for all observables whose equivalence is required by the program.

---

141. Resource-Dependent Failure

If:

Requirements(P) ⊄ Capabilities(E)

the implementation MUST NOT pretend that the program is successfully realized.

It MUST return an explicit failure.

This distinction is fundamental:

portable

does not mean:

always executable

It means:

not unnecessarily tied to a particular realization

---

142. Semantic Preservation Under Scaling

If more resources are supplied:

E_small
→
E_large

the compiler/runtime MAY exploit those resources.

It MUST NOT silently alter:

- result meaning;
- required consistency;
- required ordering;
- security;
- required effects;
- quantum semantics;
- correctness guarantees.

---

143. Distributed Correctness

A distributed transformation is valid only if it preserves:

- values;
- effects;
- dependencies;
- ownership;
- consistency;
- communication guarantees;
- failure guarantees;
- security;
- quantum legality;
- resource requirements;
- explicit timing constraints.

---

144. Testing Contract

Distributed conformance MUST include:

tests/distributed/
tests/negative/
tests/boundary/
tests/scalability/
tests/determinism/
tests/compatibility/

Tests MUST cover:

- one participant;
- multiple participants;
- dynamically sized participant sets;
- empty/invalid participant sets where relevant;
- dependency graphs;
- communication;
- ordering;
- collectives;
- replication;
- consistency;
- partitioning;
- placement constraints;
- remote execution;
- failure;
- recovery;
- cancellation;
- checkpointing;
- dynamic scaling;
- heterogeneous resources;
- quantum integration;
- HDL integration;
- AI/data integration;
- security.

---

145. Positive Tests

Positive tests MUST include:

single-context distributed intent
multi-context intent
dynamic participant sets
logical node declarations
logical services
logical workers
task dependencies
send/receive
broadcast
scatter/gather
reduce
replication
partitioning
placement constraints
resource requirements
capability requirements
failure policies
recovery policies
streaming
remote execution
hybrid classical/quantum distribution
distributed accelerator computation

---

146. Negative Tests

Negative tests MUST include:

invalid dependency
unknown required capability
incompatible consistency
invalid replication
invalid locality
security violation
unsatisfied resource requirement
unsatisfied placement constraint
illegal quantum-state transfer
invalid serialization
invalid checkpoint requirement
invalid effect usage
invalid participant reference
invalid operation arguments

---

147. Boundary Tests

Boundary tests MUST include:

- zero-length collections where semantically valid;
- one participant;
- two participants;
- many participants;
- nested distributed scopes;
- deeply nested dependencies;
- large expressions;
- large message payload metadata;
- long qualified names;
- large dependency graphs;
- large replication descriptions;
- dynamic resource quantities.

No boundary test may establish an artificial universal maximum.

---

148. Scalability Tests

Scalability testing MUST verify that grammar and semantic design do not contain fixed machine-size assumptions.

Tests SHOULD exercise progressively larger generated workloads.

The purpose is not to claim infinite physical execution.

The purpose is to verify:

no artificial language ceiling

and:

resource-dependent failure is explicit

---

149. Hard-Coding Audit

A distributed implementation MUST be audited for:

MAX_NODES
MAX_WORKERS
MAX_PROCESSES
MAX_TASKS
MAX_SERVICES
MAX_ACTORS
MAX_REPLICAS
MAX_SHARDS
MAX_PARTITIONS
MAX_CHANNELS
MAX_MESSAGES
MAX_DEVICES
MAX_REGIONS
MAX_NETWORKS

It MUST also detect disguised equivalents such as:

node0..node127
worker0..worker255
replica0..replica7

when those constructs are intended to represent universal language limits.

---

150. Implementation Limits

Implementations MAY have limits caused by:

- address space;
- memory;
- parser stack;
- compiler resources;
- runtime resources;
- operating-system constraints;
- target constraints.

Such limits MUST be implementation-defined.

They MUST NOT be represented as universal Zamani semantic limits.

When practical, diagnostics SHOULD distinguish:

language restriction

from:

implementation resource limit

---

151. Parser Safety

The grammar MUST contain:

- no embedded Rust actions;
- no "unsafe";
- no filesystem access;
- no network access;
- no hardware access;
- no runtime callbacks;
- no randomness;
- no environment-dependent parser decisions.

Parsing MUST depend only on the supplied source/token stream.

---

152. Semantic Safety

Semantic analysis MUST remain deterministic wherever the language semantics require deterministic analysis.

It MUST NOT depend on:

- physical node availability;
- provider enumeration order;
- network discovery order;
- hash iteration;
- hardware timing.

Target feasibility analysis MAY consume target information, but this must occur after language semantics are established.

---

153. Error Recovery

Parser error recovery MAY continue parsing to collect diagnostics.

It MUST NOT turn invalid distributed syntax into a valid distributed semantic program.

Production compilation MUST reject semantically invalid programs.

---

154. Grammar-to-AST Traceability

Every distributed grammar rule MUST map to a documented AST representation.

Required traceability:

grammar/distributed/*.g4
        ↓
src/frontend/ast/
        ↓
semantic model
        ↓
distributed metadata / domain IR

A grammar rule without an AST mapping is incomplete.

---

155. AST-to-Semantics Traceability

Every distributed AST construct MUST have:

semantic meaning
validation rules
error conditions
resource implications
effect implications
security implications
portability classification
IR/lowering destination

A successfully parsed construct MUST NOT disappear during semantic analysis.

---

156. Semantics-to-IR Traceability

Every semantically valid distributed construct MUST either:

1. lower into the canonical semantic/IR representation; or
2. remain as validated metadata consumed by an explicitly identified downstream subsystem.

It MUST NOT silently disappear.

---

157. No Semantic Loss

The following is prohibited:

source
 ↓
parser accepts
 ↓
AST stores partial information
 ↓
semantic analysis ignores information
 ↓
IR loses information
 ↓
runtime behaves differently

Examples include silently dropping:

- consistency requirements;
- delivery guarantees;
- security requirements;
- locality requirements;
- resource requirements;
- failure policies;
- ordering requirements;
- quantum constraints.

---

158. Integration With "src/distributed/"

The existing distributed implementation area is the implementation consumer of this specification.

Its implementation MUST:

- preserve logical identity;
- avoid physical topology assumptions;
- remain resource-parametric;
- use typed identifiers where applicable;
- distinguish semantic IDs from physical IDs;
- return structured errors;
- avoid "unsafe";
- avoid silent no-op behavior;
- avoid hard-coded universal resource limits.

Implementation details belong in Rust rather than this specification.

---

159. Integration With Runtime

The runtime MUST consume validated distributed intent.

It MUST NOT become the authority for language syntax.

Runtime behavior MUST remain traceable to:

source
→ AST
→ semantic model
→ IR/metadata
→ runtime

---

160. Integration With Quantum Distributed Infrastructure

Existing quantum distributed components, including distributed quantum memory, distributed quantum routing, distributed quantum IR metadata, and distributed quantum capabilities, MUST remain downstream consumers of the language contract.

They MUST NOT create a second source-language distributed quantum model.

The semantic flow remains:

Zamani distributed syntax
        ↓
distributed semantic analysis
        ↓
quantum semantic analysis
        ↓
quantum::ir
        ↓
distributed quantum realization

---

161. Integration With Resource Management

The resource manager determines whether a realization can satisfy:

compute
memory
communication
storage
accelerator
quantum
timing
reliability
capacity

Distributed semantics only declare the required properties.

Resource management MUST NOT modify source semantics merely to fit available resources.

---

162. Integration With Scheduling

Scheduling receives:

tasks
dependencies
resource requirements
communication requirements
timing constraints
placement constraints

The scheduler chooses an execution schedule.

The source language does not prescribe the scheduler algorithm.

---

163. Integration With Routing

Routing receives logical communication relationships.

Routing derives physical communication paths.

It MUST NOT mutate logical participant identities.

---

164. Integration With Resilience

Resilience consumes:

failure semantics
recovery intent
replication
checkpointing
retry
escalation
resource/capability state

The resilience subsystem determines the actual recovery mechanism.

---

165. Integration With Networking

Networking consumes:

logical endpoints
communication semantics
delivery semantics
security
latency requirements
bandwidth requirements

Networking determines:

transport
route
connection
protocol
packetization

---

166. Integration With Hardware

Hardware discovery provides capabilities.

The source program describes requirements.

The realization process is:

source requirement
        ↓
capability query
        ↓
candidate resources
        ↓
placement
        ↓
routing
        ↓
scheduling
        ↓
execution

No source-level universal hardware map is permitted.

---

167. Integration With Deployment

Deployment maps logical execution entities to physical infrastructure.

The deployment layer MAY determine:

- machines;
- containers;
- processes;
- regions;
- providers;
- networks;
- accelerators.

Portable source remains independent of those decisions.

---

168. Feature Status

Every distributed feature MUST have an explicit status:

STABLE
IMPLEMENTED
SPECIFIED
EXPERIMENTAL
DEPRECATED
REMOVED
RESERVED

A feature documented here but not implemented MUST NOT be advertised as compiler-supported.

---

169. Feature Promotion

A distributed feature becomes stable only after:

proposal
    ↓
semantic specification
    ↓
grammar contract
    ↓
AST contract
    ↓
semantic implementation
    ↓
IR integration
    ↓
runtime/deployment integration where required
    ↓
positive tests
    ↓
negative tests
    ↓
boundary tests
    ↓
scalability tests
    ↓
determinism tests
    ↓
compatibility decision
    ↓
STABLE

---

170. No Phantom Distributed Features

A documented distributed feature MUST have one explicit state:

implemented
specified-only
experimental
deprecated
reserved

There MUST NOT be a hidden state:

documented but unsupported

---

171. No Phantom Syntax

The parser MUST NOT accept distributed syntax that cannot be represented correctly in the AST.

Likewise, the semantic analyzer MUST NOT accept distributed constructs that cannot be represented in the downstream canonical semantic model.

---

172. Reference Examples

The following are conceptual examples of the intended model.

172.1 Logical distributed task

distributed::task compute {
    ...
}

The implementation decides where it runs.

172.2 Logical dependency

distributed::task_a -> distributed::task_b;

The arrow means semantic dependency.

It does not select a physical route.

172.3 Communication

distributed::send(channel, value, destination);

The transport remains downstream.

172.4 Resource requirement

requires capability("distributed.execution");

The capability is abstract.

172.5 Scaling

let n = input_size();
parallel_for item in data {
    ...
}

The runtime may use an implementation-dependent number of workers.

172.6 Replication

replicate state according_to policy;

The policy determines semantic requirements.

The deployment determines physical replicas.

---

173. What Distributed Syntax Must Never Mean Implicitly

The following interpretations are prohibited:

node       → physical machine
worker     → CPU core
service    → IP address
channel    → TCP socket
message    → network packet
replica    → physical copy
partition  → machine
accelerator → specific device
quantum node → physical QPU

Such mappings require downstream realization.

---

174. Distributed Program Lifecycle

A production implementation SHOULD follow:

source
  ↓
lexical analysis
  ↓
parsing
  ↓
domain-neutral AST
  ↓
structural validation
  ↓
name resolution
  ↓
type analysis
  ↓
effect analysis
  ↓
resource analysis
  ↓
capability analysis
  ↓
distributed semantic analysis
  ↓
security analysis
  ↓
portability analysis
  ↓
canonical semantic representation
  ↓
IR lowering
  ↓
IR verification
  ↓
optimization
  ↓
placement
  ↓
routing
  ↓
scheduling
  ↓
resilience planning
  ↓
deployment
  ↓
runtime

No downstream stage may silently redefine an earlier semantic contract.

---

175. Production Readiness Criteria

"grammar/spec/distributed.md" is considered production-ready when:

- distributed terminology is defined;
- ownership boundaries are explicit;
- grammar integration is explicit;
- AST integration is explicit;
- semantic integration is explicit;
- resource integration is explicit;
- capability integration is explicit;
- portability integration is explicit;
- execution integration is explicit;
- networking integration is explicit;
- security integration is explicit;
- placement integration is explicit;
- routing integration is explicit;
- scheduling integration is explicit;
- resilience integration is explicit;
- quantum integration is explicit;
- HDL integration is explicit;
- AI/data integration is explicit;
- no artificial limits are specified;
- no physical topology is required by portable source;
- logical and physical identity are separated;
- failures are explicit;
- diagnostics are specified;
- versioning is specified;
- compatibility is specified;
- AST traceability is specified;
- IR traceability is specified;
- tests are specified;
- negative tests are specified;
- boundary tests are specified;
- scalability tests are specified;
- hard-coding audit is specified;
- safe Rust implementation is required;
- "unsafe" is prohibited.

---

176. Completion Checklist

Before marking this specification complete, verify:

Authority

- [ ] This file is the normative distributed semantic contract.
- [ ] "grammar/Zamani.g4" remains the grammar composition root.
- [ ] "grammar/grammar.md" remains implementation-conformance documentation.
- [ ] "grammar/Zamani-Grammar.md" cannot silently introduce distributed semantics.

Grammar

- [ ] "grammar/distributed/distributed.g4" consumes this semantic contract.
- [ ] "grammar/execution/distributed-execution.g4" consumes this semantic contract.
- [ ] Distributed subgrammars do not redefine general expressions.
- [ ] Distributed subgrammars do not redefine identifiers.
- [ ] Distributed subgrammars do not create physical topology syntax as universal semantics.

AST

- [ ] Every accepted distributed construct has an AST representation.
- [ ] Source spans are preserved.
- [ ] Logical identity is preserved.
- [ ] Physical identities are not introduced prematurely.

Semantics

- [ ] Requirements are distinguished from preferences.
- [ ] Capabilities are distinguished from resources.
- [ ] Logical placement is distinguished from physical placement.
- [ ] Logical communication is distinguished from physical transport.
- [ ] Replication is distinguished from independent copies.
- [ ] Consistency is explicit.
- [ ] Failure semantics are explicit.

Quantum

- [ ] "quantum::ir" remains canonical.
- [ ] No second quantum IR exists.
- [ ] No physical qubit mapping is encoded as universal distributed semantics.
- [ ] Quantum-state legality remains a quantum semantic concern.
- [ ] QEC remains a QEC concern.
- [ ] ZQN remains a ZQN concern.

Scalability

- [ ] No "MAX_NODES".
- [ ] No "MAX_WORKERS".
- [ ] No "MAX_TASKS".
- [ ] No "MAX_REPLICAS".
- [ ] No "MAX_SHARDS".
- [ ] No "MAX_CHANNELS".
- [ ] No fixed topology.
- [ ] No fixed transport.
- [ ] No fixed machine count.
- [ ] No fixed accelerator count.

Portability

- [ ] One source program can describe scalable distributed computation.
- [ ] Additional resources may be exploited without source rewriting.
- [ ] Resource exhaustion is explicit.
- [ ] Capability failure is explicit.
- [ ] Semantic meaning is preserved across valid realizations.

Safety

- [ ] Rust 1.97 / 1.97.1 compatibility is maintained.
- [ ] Rust 2021 is maintained.
- [ ] "unsafe" is prohibited.
- [ ] Grammar actions are prohibited.
- [ ] Parser I/O is prohibited.
- [ ] Parser hardware access is prohibited.
- [ ] Parser network access is prohibited.

Validation

- [ ] Positive tests exist.
- [ ] Negative tests exist.
- [ ] Boundary tests exist.
- [ ] Scalability tests exist.
- [ ] Determinism tests exist.
- [ ] Compatibility tests exist.
- [ ] Hard-coding audit exists.

---

177. Final Architectural Rule

The distributed subsystem of Zamani MUST follow this rule:

SOURCE SEMANTICS
    describe
        WHAT computation means
        WHAT communication means
        WHAT state relationships mean
        WHAT consistency means
        WHAT resources are required
        WHAT capabilities are required
        WHAT failures are acceptable
        WHAT correctness guarantees exist

DOWNSTREAM SYSTEMS
    determine
        WHERE computation executes
        HOW computation is partitioned
        HOW computation is replicated
        HOW messages are transported
        HOW data is placed
        HOW routes are selected
        HOW tasks are scheduled
        HOW failures are recovered
        WHICH hardware is selected
        WHICH provider is selected
        WHICH physical topology is used

Therefore:

Zamani distributed source
        ↓
portable semantic intent
        ↓
resource/capability resolution
        ↓
canonical semantic representation
        ↓
canonical/domain IR
        ↓
optimization
        ↓
placement
        ↓
routing
        ↓
scheduling
        ↓
resilience
        ↓
deployment
        ↓
runtime

The distributed language MUST remain:

target-independent by default
resource-parametric
capability-driven
topology-independent
transport-independent
provider-independent
scale-independent
safe
deterministic where required
open to future distributed architectures

and MUST support:

tiny → large → arbitrarily large

subject only to:

actual available resources
+
actual available capabilities
+
explicit semantic requirements
+
explicit target constraints

The fundamental invariant is:

Program Once
    ↓
Compile Once
    ↓
Derive Realization
    ↓
Run Everywhere
    ↓
Run Anywhere
    ↓
Remain Semantically Stable Forever

No distributed feature is production-ready until its complete path is defined:

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
Resources / Capabilities / Effects
  ↓
Canonical Semantic Model
  ↓
IR
  ↓
Verification
  ↓
Optimization
  ↓
Placement / Routing / Scheduling
  ↓
Resilience
  ↓
Deployment
  ↓
Runtime

A distributed feature that cannot complete this chain MUST remain "SPECIFIED", "EXPERIMENTAL", or "RESERVED"; it MUST NOT be represented as a stable production feature merely because its syntax parses.