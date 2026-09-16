Zamani Networking Specification

Path: "grammar/spec/networking.md"
Domain: Networking, communication, networked computation, distributed communication, communication-aware hardware/software systems
Language: Zamani
Specification status: Normative
Grammar technology: ANTLR4 composition
Compiler baseline: Rust 1.97 / Rust 1.97.1
Safety baseline: Safe Rust only; "unsafe" is prohibited
Portability objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability objective: From the smallest supported communication computation to arbitrarily large communication systems, constrained only by program semantics, available resources, implementation capabilities, and explicitly declared requirements/constraints.

---

1. Purpose

This specification defines the normative networking-language contract for Zamani.

Networking is a first-class computational domain of Zamani.

Networking syntax MUST allow a program to express communication semantics including, where applicable:

- communication participants;
- logical endpoints;
- messages;
- channels;
- protocols;
- services;
- communication patterns;
- streams;
- requests and responses;
- event communication;
- capabilities;
- requirements;
- constraints;
- preferences;
- hints;
- locality;
- quality-of-service intent;
- reliability;
- ordering;
- delivery semantics;
- communication security requirements;
- data movement;
- distributed communication;
- heterogeneous communication;
- accelerator communication;
- quantum/classical communication;
- hardware/software communication;
- future communication mechanisms.

The specification deliberately separates:

WHAT the program means
        from
HOW communication is realized
        from
WHERE communication executes
        from
WHICH physical resources perform it

The networking language therefore describes portable communication intent, not a particular operating system, network interface, machine, provider, topology, transport implementation, or runtime.

---

2. Authority and Integration

Networking syntax is governed by the repository's overall grammar authority model.

The relevant architecture is:

grammar/spec/networking.md
        │
        │ normative networking semantics
        ▼
grammar/networking/*.g4
        │
        │ syntax implementation
        ▼
grammar/Zamani.g4
        │
        ▼
Lexer
        │
        ▼
Parser
        │
        ▼
src/frontend/ast/
        │
        ▼
structural validation
        │
        ▼
name / type / effect / capability analysis
        │
        ▼
networking semantic representation
        │
        ├── classical semantics
        ├── distributed semantics
        ├── quantum semantics
        ├── hybrid semantics
        ├── hardware semantics
        ├── data semantics
        ├── security semantics
        └── execution semantics
        │
        ▼
canonical IR / domain IR
        │
        ▼
optimization
        │
        ▼
placement / routing / scheduling
        │
        ▼
deployment
        │
        ▼
runtime / hardware realization

Networking specification files MUST NOT create a parallel compiler architecture.

Networking syntax MUST integrate with the existing frontend AST architecture rather than creating an independent networking AST that competes with the canonical Zamani AST.

The repository's frontend AST is explicitly structured as a canonical node subsystem, with domain/resource/capability/validation components available for integration.

---

3. Ownership

This file owns the normative language-level networking contract.

It owns:

- networking concepts;
- networking semantic distinctions;
- networking syntax requirements;
- networking portability requirements;
- networking scalability requirements;
- networking resource semantics;
- networking capability semantics;
- networking cross-domain contracts;
- networking AST requirements;
- networking semantic-model requirements;
- networking IR integration requirements;
- networking compiler/runtime boundaries;
- networking conformance requirements.

It does not own:

- parser implementation;
- lexer implementation;
- frontend AST implementation;
- networking runtime implementation;
- socket implementation;
- operating-system networking;
- routing algorithms;
- scheduling algorithms;
- hardware discovery;
- physical network configuration;
- cryptographic implementation;
- serialization implementation;
- cloud-provider implementation;
- network-driver implementation;
- QEC implementation;
- ZQN implementation;
- quantum IR implementation;
- distributed-runtime implementation.

Those responsibilities belong to their respective repository layers.

---

4. Existing File Integration

The networking specification MUST remain aligned with these existing repository surfaces.

4.1 "grammar/networking/"

Current networking grammar components include:

grammar/networking/
├── README.md
├── networking.g4
├── endpoints.g4
├── channels.g4
├── messages.g4
├── protocols.g4
├── services.g4
└── network-capabilities.g4

The existing directory already defines these as separate responsibilities rather than one monolithic networking grammar.

Their responsibilities are:

File| Owns
"networking.g4"| networking composition and public grammar boundary
"endpoints.g4"| logical endpoint syntax
"channels.g4"| networking-channel syntax
"messages.g4"| message declarations
"protocols.g4"| protocol declarations and intent
"services.g4"| service declarations
"network-capabilities.g4"| networking capability requirements

This specification defines what those files MUST mean.

---

5. Existing "networking.g4"

"grammar/networking/networking.g4" is the networking composition boundary.

The current grammar explicitly describes itself as an aggregate/composition grammar and identifies the component grammars above.

Its responsibilities remain:

- compose networking grammar components;
- expose networking parser entry points;
- dispatch networking constructs;
- provide the stable networking-domain grammar boundary.

It MUST NOT become a second specification.

It MUST NOT duplicate component rules.

It MUST NOT contain runtime behavior.

It MUST NOT contain target-specific limits.

It MUST NOT perform semantic analysis.

It MUST NOT construct IR.

---

6. Existing "networking/README.md"

"grammar/networking/README.md" is the architectural and implementation-facing contract for the networking grammar directory. It already establishes the networking-domain boundary, component ownership, POCO-REAF objective, and separation from distributed, quantum, hardware, security, and runtime concerns.

This specification is the normative semantic counterpart.

The relationship is:

grammar/spec/networking.md
        ↓
normative meaning
        ↓
grammar/networking/*.g4
        ↓
syntactic realization

Neither file should silently introduce semantics absent from the other.

---

7. Existing "grammar/grammar.md"

"grammar/grammar.md" describes the implementation-facing grammar accepted by the current frontend.

It MUST NOT silently become the normative authority for networking semantics.

The authority relationship is:

grammar/spec/networking.md
        =
normative specification

grammar/networking/*.g4
        =
grammar implementation

grammar/grammar.md
        =
implementation/conformance reference

If these disagree, the discrepancy MUST be recorded and resolved through the repository's compatibility/conformance process.

---

8. Existing "grammar/DESIGN.md"

Networking MUST obey the repository-wide grammar architecture defined by "grammar/DESIGN.md".

The networking domain therefore inherits:

- one language;
- one frontend architecture;
- one AST architecture;
- semantic/domain separation;
- canonical IR boundaries;
- resource/capability separation;
- portability requirements;
- no artificial hardware limits;
- deterministic parsing;
- source-span preservation;
- compatibility requirements.

The repository explicitly identifies networking as one of the domains that must coexist with classical, quantum, HDL, hybrid, AI, security, data, and other computation.

---

9. Core Networking Model

Zamani networking is based on logical communication.

The fundamental model is:

Participant
    │
    ▼
Endpoint
    │
    ▼
Channel
    │
    ▼
Message / Stream / Event
    │
    ▼
Protocol
    │
    ▼
Service / Communication Operation

These are semantic abstractions.

They are not required to correspond one-to-one with:

- processes;
- threads;
- machines;
- sockets;
- ports;
- NICs;
- routers;
- links;
- network cards;
- containers;
- virtual machines;
- physical nodes.

A compiler/runtime MAY map them to those mechanisms where appropriate.

---

10. Endpoint Semantics

An endpoint represents a logical participant in communication.

An endpoint MAY represent:

- a local computation;
- a task;
- an actor;
- a service;
- a process;
- a device;
- an accelerator;
- a remote computation;
- a quantum processor;
- a hardware component;
- a virtual endpoint;
- a future computational substrate.

The endpoint abstraction MUST NOT require physical realization.

For example:

endpoint worker;

does not inherently mean:

machine 0
process 1
interface eth0
port 8080

Those are realization details.

---

11. Endpoint Identity

Zamani MUST distinguish:

logical identity

from:

physical identity

Logical identity belongs to program semantics.

Physical identity belongs to deployment/runtime realization.

A program MAY explicitly require a physical property when that property is genuinely part of its semantics.

For example, a program may require:

capability("authenticated-communication")

without selecting:

interface("eth0")

The latter is a deployment-specific decision unless explicitly required by the program.

---

12. Endpoint Collections

Endpoint collections MUST be unbounded at the language level.

The grammar MUST NOT define:

MAX_ENDPOINTS
MAX_CLIENTS
MAX_SERVERS
MAX_PEERS
MAX_PARTICIPANTS
MAX_NODES

A collection is represented through normal language constructs such as:

- declarations;
- arrays;
- iterables;
- ranges;
- comprehensions;
- generated entities;
- dynamic discovery;
- distributed placement semantics.

The implementation may impose practical resource limits.

Those limits MUST NOT become language-level semantic limits.

---

13. Channel Semantics

A networking channel represents a communication relationship.

It may provide semantics including:

- direction;
- ordering;
- reliability;
- delivery;
- flow;
- streaming;
- backpressure;
- buffering requirements;
- multiplexing;
- communication scope;
- security requirements;
- quality-of-service requirements.

A networking channel is distinct from a concurrency channel.

The repository contains "grammar/concurrency/channels.g4".

Therefore:

networking channel

means communication across a networking boundary or networking abstraction.

Whereas:

concurrency channel

means a language-level synchronization/communication primitive.

The semantic layer MAY connect them.

The grammar MUST NOT collapse their distinct ownership.

---

14. Message Semantics

A message represents information communicated between participants.

A message MAY contain:

- scalar values;
- structures;
- records;
- collections;
- tensors;
- streams;
- opaque values;
- references where permitted;
- metadata;
- provenance;
- security metadata;
- application-defined fields.

Messages integrate with the repository's data/type system.

The networking grammar MUST NOT implement serialization.

The semantic pipeline is:

message syntax
    ↓
frontend AST
    ↓
type/data analysis
    ↓
message semantic model
    ↓
serialization/lowering

Not:

message grammar
    ↓
wire-format implementation

---

15. Message Size

The language MUST NOT impose a universal maximum message size.

It MUST NOT define:

MAX_MESSAGE_SIZE
MAX_PACKET_SIZE
MAX_PAYLOAD
MAX_FIELDS

as universal networking-language limits.

A program MAY explicitly require or constrain message size.

For example:

requires message_size <= expression;

is a semantic requirement.

It is fundamentally different from a compiler constant such as:

MAX_MESSAGE_SIZE = 65536

The former belongs to program semantics.

The latter would be an implementation limitation and MUST NOT become the universal language contract.

---

16. Streaming

Networking MUST support communication whose extent is not known statically.

The semantic model MUST support:

- finite streams;
- potentially unbounded streams;
- asynchronous streams;
- synchronous streams;
- event streams;
- incremental messages;
- backpressured streams;
- stream transformations;
- stream composition.

No fixed stream length belongs in the grammar.

A stream MAY continue until:

- completion;
- cancellation;
- failure;
- resource exhaustion;
- explicit termination;
- external condition;
- program-defined condition.

---

17. Protocol Semantics

A protocol defines communication behavior or requirements.

A protocol MAY describe:

- ordering;
- handshake;
- request/response;
- streaming;
- event delivery;
- reliability;
- acknowledgement;
- retry semantics;
- idempotency;
- session behavior;
- state transitions;
- flow control;
- capability requirements.

The grammar MUST use an open-world model.

It MUST NOT require the language grammar to enumerate every networking protocol that exists.

---

18. Open-World Protocol Model

Protocols such as:

TCP
UDP
QUIC
HTTP
MQTT
gRPC
MPI
RDMA
InfiniBand

MAY be represented as names, libraries, declarations, dialects, capabilities, or interoperability contracts.

They MUST NOT be the fundamental closed vocabulary of Zamani networking.

A future protocol MUST be expressible without requiring a new Zamani core grammar keyword solely because its name is new.

This is essential for POCO-REAF.

---

19. Transport Independence

The source language describes communication semantics independently of transport realization.

The same logical communication may be realized using:

- shared memory;
- local IPC;
- sockets;
- message queues;
- TCP;
- UDP;
- QUIC;
- RDMA;
- optical links;
- wireless communication;
- accelerator interconnects;
- specialized fabrics;
- quantum communication;
- future transports.

Transport selection is downstream.

The compiler/runtime MAY choose a realization according to:

- target capabilities;
- declared requirements;
- deployment policy;
- performance objectives;
- security requirements;
- resource availability;
- runtime conditions.

---

20. Service Semantics

A service is a logical communication interface.

A service MAY define:

- operations;
- inputs;
- outputs;
- messages;
- events;
- protocols;
- capabilities;
- security requirements;
- availability requirements;
- resource requirements;
- communication policies.

A service MUST describe behavior rather than physical placement.

This is valid:

service compute;

The declaration MUST NOT inherently imply:

machine 0
GPU 2
QPU 1
port 8080
interface eth0

unless such properties are explicitly declared as semantic requirements.

---

21. Request/Response Semantics

Networking MUST support request/response communication as a semantic pattern.

The semantic model MUST permit:

request
    →
processing
    →
response

including:

- synchronous requests;
- asynchronous requests;
- streaming responses;
- partial responses;
- cancellation;
- timeout requirements;
- retry policies;
- idempotency requirements;
- error responses.

Timeouts and retries MUST remain semantic policies rather than fixed runtime constants.

---

22. Event-Driven Communication

Networking MUST support event-oriented communication.

Events MAY be:

- local;
- remote;
- multicast;
- broadcast;
- service-generated;
- hardware-generated;
- sensor-generated;
- quantum-measurement-derived;
- distributed.

Event semantics MUST remain independent of the underlying event transport.

---

23. Publish/Subscribe

The networking model SHOULD support publish/subscribe semantics.

The language model MUST distinguish:

publication intent
subscription intent
delivery semantics
transport realization

A publish operation MUST NOT inherently require a particular broker, server, machine, or provider.

---

24. Multicast and Broadcast

The semantic model MUST support communication patterns beyond point-to-point communication.

Where supported by the target, networking may include:

- multicast;
- broadcast;
- fan-out;
- fan-in;
- all-to-all;
- one-to-many;
- many-to-one;
- many-to-many.

The grammar MUST NOT impose fixed participant counts.

For example:

broadcast message;

must not imply:

broadcast to 8 nodes;

unless the program explicitly specifies eight as its semantic requirement.

---

25. Collective Communication

Networking MUST be capable of representing collective communication used by:

- distributed computing;
- HPC;
- AI training;
- tensor computation;
- scientific computing;
- accelerator systems;
- quantum distributed computing.

Collective operations may include:

- broadcast;
- gather;
- scatter;
- reduce;
- all-reduce;
- all-gather;
- exchange;
- barrier communication.

The number of participants MUST remain data/resource dependent.

---

26. Networking and Distributed Computing

Networking and distributed computing are related but separate domains.

"grammar/networking/" owns:

- communication;
- endpoints;
- channels;
- messages;
- protocols;
- services;
- networking capabilities.

"grammar/distributed/" owns distributed execution semantics such as:

- node membership;
- placement;
- replication;
- partitioning;
- distributed state;
- consistency;
- distributed fault tolerance;
- distributed scheduling.

Therefore:

networking ≠ distributed execution

A distributed program may use networking, but networking MUST NOT redefine distributed execution semantics.

The repository's distributed grammar explicitly treats communication syntax as intent while networking/runtime layers select its realization.

---

27. Networking and Concurrency

Networking operations may execute concurrently.

Networking MUST integrate with:

- tasks;
- async operations;
- actors;
- futures;
- streams;
- synchronization;
- cancellation;
- structured concurrency;
- deterministic concurrency.

Networking grammar MUST NOT duplicate concurrency semantics.

The relationship is:

concurrency
    +
networking
    ↓
semantic composition

not:

networking grammar
    =
concurrency grammar

---

28. Networking and Classical Computing

Classical programs may use networking for:

- RPC;
- distributed computation;
- services;
- data transfer;
- storage;
- control;
- telemetry;
- synchronization;
- scientific workloads;
- HPC;
- cloud computation.

Networking MUST remain a language capability rather than a separate language.

---

29. Networking and Quantum Computing

Networking MUST support communication involving quantum computation without defining quantum semantics itself.

Possible use cases include:

- distributed quantum computation;
- remote quantum execution;
- quantum-classical control;
- quantum networking;
- entanglement-related communication;
- distributed QEC workflows;
- hybrid quantum services.

However, networking MUST NOT define:

- "QubitId";
- physical qubit identifiers;
- quantum gates;
- quantum states;
- circuits;
- QEC codes;
- calibration;
- pulse schedules;
- ZQN faults.

Quantum semantics remain owned by the quantum domain and canonical "quantum::ir".

The repository already identifies quantum networking and distributed quantum computing as valid quantum-domain applications.

---

30. Networking and "quantum::ir"

Networking MUST NOT create a second quantum IR.

The correct direction is:

networking syntax
        ↓
frontend AST
        ↓
semantic analysis
        ↓
quantum/network semantic composition
        ↓
quantum::ir

where quantum computation genuinely participates.

Networking metadata may accompany or relate to quantum semantic operations where the IR architecture permits it.

But:

grammar/networking
        ↓
independent quantum IR

is prohibited.

---

31. Networking and Hardware

Networking may involve:

- CPUs;
- GPUs;
- FPGAs;
- ASICs;
- QPUs;
- accelerators;
- embedded devices;
- memory systems;
- communication fabrics;
- future computational substrates.

The networking grammar MUST NOT define physical hardware topology.

Hardware realization belongs downstream.

The architecture is:

networking intent
        ↓
requirements/capabilities
        ↓
hardware analysis
        ↓
placement
        ↓
routing
        ↓
scheduling
        ↓
runtime realization

---

32. No Hard-Coded Hardware Limits

The networking language MUST NOT contain universal limits for:

MAX_NODES
MAX_ENDPOINTS
MAX_CHANNELS
MAX_MESSAGES
MAX_SERVICES
MAX_CONNECTIONS
MAX_LINKS
MAX_ROUTERS
MAX_INTERFACES
MAX_NETWORK_SIZE

It MUST NOT assume:

8 nodes
16 endpoints
32 channels
64 connections
1000 messages

as universal implementation limits.

The only valid fixed quantities are those that are genuine program semantics.

---

33. Address Abstraction

Networking MAY represent addresses when addresses are semantically necessary.

The language SHOULD distinguish:

logical address
service identity
resource identity
deployment identity
physical address

A physical address such as an IP address MUST NOT be required merely to express logical communication.

The implementation MAY resolve a logical identity to an address at compile time or runtime.

---

34. Provider Independence

Zamani networking MUST NOT depend on a specific:

- cloud provider;
- operating system;
- network stack;
- service mesh;
- container runtime;
- accelerator vendor;
- hardware vendor;
- network driver.

Provider-specific integration belongs under interoperability/dialect/backend layers.

---

35. Network Capability Model

Capabilities express what a communication realization can provide.

Examples include:

reliable communication
ordered delivery
secure communication
authenticated communication
confidential communication
integrity protection
multicast
broadcast
streaming
low-latency communication
high-bandwidth communication
local communication
remote communication
persistent communication
fault-tolerant communication
quantum communication

Capability names are semantic identifiers.

They MUST NOT directly select a particular machine.

---

36. Requirement vs Capability

These concepts MUST remain distinct.

requirement

means:

«The program requires a property.»

capability

means:

«A target/resource can provide a property.»

For example:

requires reliable_communication;

is a program requirement.

A runtime may discover:

capability reliable_communication

on a particular resource.

The compiler/runtime determines whether the capability satisfies the requirement.

---

37. Requirement vs Constraint

A requirement describes something necessary.

A constraint restricts valid realizations.

For example:

requires secure communication

and:

constrain communication to an allowed trust domain

are not equivalent.

The semantic model MUST preserve the distinction.

---

38. Requirement vs Preference

A preference is not necessarily mandatory.

The language MUST allow the semantic layer to distinguish:

require
prefer
allow
avoid
prohibit
hint

A preference MUST NOT accidentally become an absolute hardware requirement.

---

39. Requirement vs Implementation Decision

This distinction is central to POCO-REAF.

These are different:

requires reliable communication

and:

use interface eth0

The first describes semantic intent.

The second identifies a realization.

The latter belongs downstream unless explicitly required by the program.

---

40. Quality of Service

Networking MAY express semantic quality requirements such as:

- latency;
- throughput;
- bandwidth;
- jitter;
- reliability;
- availability;
- ordering;
- delivery;
- loss tolerance;
- energy;
- locality.

These MUST be expressions or semantic constraints where appropriate.

The language MUST NOT assume fixed universal values.

For example:

requires latency <= L;

is portable if "L" is a program-level requirement.

It does not mean every machine must have the same latency.

---

41. Resource-Aware Networking

Networking integrates with the repository's resource model.

Relevant resource dimensions may include:

- communication capacity;
- bandwidth;
- memory;
- energy;
- compute;
- storage;
- accelerator capacity;
- network locality;
- reliability;
- availability.

Networking MUST express resource intent without owning resource allocation.

Resource allocation belongs downstream.

---

42. Dynamic Resource Availability

A networking program MUST remain valid when the available realization changes.

For example:

program
    requires communication capability C

may execute on:

- a small local system;
- a larger machine;
- a distributed cluster;
- a cloud deployment;
- an accelerator system;
- a future communication substrate.

Provided the target satisfies the program's semantic requirements.

---

43. Scaling Semantics

Networking programs MUST be capable of scaling along dimensions such as:

- endpoint count;
- message count;
- channel count;
- service count;
- data volume;
- communication volume;
- node count;
- topology size;
- concurrency;
- geographic distribution.

Scaling MUST be represented using program semantics rather than fixed grammar limits.

---

44. Tiny-to-Large Principle

The same source language MUST support:

single-process communication

through:

multi-threaded communication

through:

multi-device communication

through:

multi-node communication

through:

large distributed communication systems

without requiring a different networking language.

The source describes the computation.

The available resources determine the realization.

---

45. "Infinity" and Practical Limits

"Infinity" is a scalability objective, not a claim that physical machines have infinite resources.

The language MUST NOT encode a finite artificial ceiling merely because current implementations have finite resources.

Practical execution is bounded by:

- available memory;
- available compute;
- network capacity;
- runtime limits;
- operating-system limits;
- deployment limits;
- hardware capabilities;
- explicitly declared program constraints.

These limits are external to the language's universal semantic model.

---

46. Backpressure

Streaming communication SHOULD support backpressure semantics.

Backpressure expresses:

producer ↔ consumer capacity relationship

rather than specifying:

buffer = exactly N bytes

unless an exact buffer requirement is part of program semantics.

Backpressure policies MAY include:

- block;
- await;
- drop;
- reject;
- buffer;
- slow producer;
- shed load;
- transform;
- retry.

---

47. Reliability

Networking MUST permit programs to express reliability requirements.

Reliability may concern:

- delivery;
- ordering;
- persistence;
- availability;
- recovery;
- duplicate suppression;
- idempotency;
- fault tolerance.

The grammar MUST NOT implement reliability.

The runtime/backend chooses an implementation capable of satisfying the semantic requirement.

---

48. Failure Semantics

Networking operations may fail because of:

- unavailable endpoint;
- unavailable service;
- lost communication;
- rejected communication;
- resource exhaustion;
- timeout;
- cancellation;
- capability mismatch;
- security failure;
- protocol failure.

Failures MUST be representable through Zamani's normal error/effect/control-flow architecture.

Networking MUST NOT create an unrelated error system.

---

49. Retry Semantics

Retry behavior MUST be explicit semantic intent.

A retry policy MAY include:

- retryability;
- retry count as a program requirement;
- delay policy;
- backoff policy;
- idempotency requirement;
- failure classification.

The networking grammar MUST NOT silently retry every failed operation.

Runtime behavior MUST derive from semantic policy.

---

50. Cancellation

Networking operations MUST integrate with the repository's cancellation/effect/concurrency architecture.

Cancellation MUST be composable with:

- async operations;
- streams;
- requests;
- services;
- distributed execution;
- runtime lifecycle.

A networking grammar rule MUST NOT create an independent cancellation model.

---

51. Ordering

Networking semantics MUST distinguish:

- ordered communication;
- partially ordered communication;
- unordered communication.

Ordering guarantees MUST be semantic.

A transport may implement those guarantees through different mechanisms.

---

52. Delivery Semantics

The semantic model SHOULD distinguish properties such as:

at-most-once
at-least-once
exactly-once
best-effort
durable
non-durable

These are communication semantics, not necessarily transport names.

The compiler/runtime is responsible for determining whether and how a target can satisfy them.

---

53. Security Integration

Networking MUST integrate with:

grammar/security/
grammar/effects/

and the repository's security semantics.

Networking may express:

- authentication requirements;
- confidentiality requirements;
- integrity requirements;
- authorization requirements;
- trust requirements;
- secure-channel requirements.

Networking MUST NOT implement:

- cryptographic algorithms;
- key generation;
- certificate verification;
- key storage;
- authentication engines;
- authorization engines;
- trust engines.

---

54. Secret Handling

Networking syntax MUST NOT require secrets to be embedded directly into source code.

Credentials, private keys, tokens, certificates, and other secrets MUST be represented through secure resource/reference mechanisms where needed.

The networking grammar MUST NOT define plaintext-secret storage semantics.

---

55. Data Integration

Networking messages MUST integrate with the universal Zamani data/type model.

Relevant data domains may include:

- scalar values;
- records;
- collections;
- tensors;
- datasets;
- streams;
- binary data;
- structured data;
- opaque data.

The networking domain does not own those types.

---

56. Serialization Boundary

Serialization is a downstream concern.

The semantic pipeline is:

Zamani value
    ↓
type/data semantics
    ↓
message semantics
    ↓
serialization selection
    ↓
wire representation

The grammar MUST NOT hard-code one wire format.

Potential realizations may include:

- text;
- binary;
- structured formats;
- zero-copy representations;
- shared-memory representations;
- vendor formats;
- future formats.

---

57. Zero-Copy Communication

The language MAY express zero-copy or ownership-related communication requirements.

However, the grammar MUST distinguish:

zero-copy is required

from:

this specific memory address must be used

The first is semantic intent.

The second is target realization.

Memory ownership must integrate with the repository's memory/ownership model.

---

58. Networking and Ownership

Networking operations MUST respect Zamani ownership and lifetime semantics.

The networking domain MUST NOT silently create:

- memory leaks;
- invalid references;
- dangling communication resources;
- unsound aliasing.

The exact enforcement belongs to semantic analysis and runtime.

---

59. Networking and Effects

Network communication is inherently capable of observable effects.

Networking operations SHOULD integrate with the effect system for properties such as:

- I/O;
- remote communication;
- nondeterminism;
- failure;
- authentication;
- resource acquisition;
- cancellation;
- external observation.

Networking MUST NOT create an isolated effect system.

---

60. Networking and Determinism

Parsing MUST be deterministic.

The networking grammar MUST contain:

- no semantic predicates;
- no embedded code;
- no environment-dependent parsing;
- no filesystem access;
- no network access;
- no hardware inspection;
- no randomness.

The same source/token stream MUST produce the same parse result.

Runtime networking can naturally be nondeterministic; that is a semantic/runtime concern rather than a parser concern.

---

61. Source Spans

Every networking AST construct MUST preserve source-location information.

At minimum, semantic consumers MUST be able to associate diagnostics with:

- declaration;
- endpoint;
- channel;
- message;
- protocol;
- service;
- operation;
- capability;
- requirement;
- constraint;
- malformed networking syntax.

Networking grammar MUST NOT discard source information required by frontend diagnostics.

---

62. AST Contract

Every networking grammar construct MUST map deterministically into the canonical frontend AST.

Conceptually:

grammar construct
        ↓
AST node
        ↓
semantic networking node

The networking AST SHOULD use generic language concepts where possible.

It MUST NOT create a vendor-specific AST hierarchy merely to represent networking.

A useful conceptual model is:

Declaration
Operation
Reference
Type
Attribute
Capability
Requirement
Constraint
Effect

with networking-specific semantic attributes.

---

63. No Networking IR in the Grammar

The grammar MUST NOT contain IR construction logic.

It MUST NOT:

- instantiate compiler IR;
- select backend instructions;
- select physical routes;
- allocate sockets;
- assign ports;
- assign NICs;
- assign nodes;
- construct packets;
- schedule communication.

The parser ends at syntax.

---

64. Semantic Networking Model

The semantic layer SHOULD represent concepts such as:

Endpoint
Channel
Message
Protocol
Service
CommunicationOperation
CommunicationRequirement
CommunicationCapability
CommunicationConstraint
CommunicationPreference
CommunicationEffect
CommunicationSecurityRequirement
CommunicationResourceRequirement

These semantic entities are not parser constructs unless syntax requires them.

Their implementation belongs outside this specification.

---

65. IR Integration

Networking semantics may lower into:

- canonical semantic IR;
- communication IR;
- distributed IR;
- classical IR;
- hardware communication IR;
- accelerator communication representation;
- quantum-associated communication metadata.

The specific IR architecture remains owned by the compiler/IR subsystem.

Networking specification MUST NOT invent a competing canonical IR.

---

66. Classical IR Integration

Classical computations that communicate may lower communication operations into the appropriate classical/domain representation.

Examples include:

send
receive
request
response
stream
collective

The networking specification does not prescribe the machine instruction representation.

---

67. Quantum IR Integration

Where networking interacts with quantum computation:

networking semantics
        +
quantum semantics
        ↓
canonical semantic composition
        ↓
quantum::ir and/or associated communication representation

Quantum semantic ownership remains with "quantum::ir".

---

68. Distributed IR Integration

Networking operations used by distributed computation MUST integrate with distributed semantic representation.

Networking owns communication.

Distributed owns:

- placement;
- replication;
- consistency;
- distributed state;
- membership;
- distributed execution.

---

69. Hardware Communication Integration

Networking semantics MAY lower to:

- buses;
- interconnects;
- NoCs;
- network fabrics;
- accelerator links;
- DMA;
- shared memory;
- specialized communication units.

The hardware domain determines realization.

Networking grammar does not encode hardware topology.

---

70. Compiler Integration

The compiler MUST be able to:

1. parse networking constructs;
2. construct AST nodes;
3. resolve names;
4. validate types;
5. validate effects;
6. validate capabilities;
7. validate requirements;
8. validate constraints;
9. build communication semantics;
10. integrate with distributed analysis;
11. integrate with hardware analysis where necessary;
12. integrate with quantum semantics where necessary;
13. lower into canonical/domain IR;
14. optimize;
15. perform placement;
16. perform routing;
17. perform scheduling;
18. produce deployment artifacts.

Networking syntax itself performs none of steps 4–18.

---

71. Placement

Placement answers:

«Where should this communication endpoint or participating computation be realized?»

Placement MUST remain separate from networking semantics.

Placement may depend on:

- resources;
- topology;
- locality;
- performance;
- reliability;
- security;
- cost;
- energy;
- deployment policy.

---

72. Routing

Routing answers:

«How should communication travel through the available realization?»

Routing is downstream.

Networking grammar MUST NOT contain physical route selection.

A source-level logical path MAY express semantic constraints, but physical route selection remains an implementation concern.

---

73. Scheduling

Scheduling answers:

«When should communication occur relative to other operations?»

Networking may expose timing or ordering requirements.

Scheduling decides actual execution order subject to:

- program semantics;
- resource availability;
- dependencies;
- target capabilities.

---

74. Runtime Integration

The runtime may:

- resolve endpoints;
- discover resources;
- establish connections;
- send/receive data;
- manage streams;
- handle failures;
- perform service discovery;
- execute communication policies;
- enforce resource limits.

None of these are parser responsibilities.

---

75. Existing "src/stdlib/net.rs"

The current standard-library networking implementation is conceptual and currently contains concrete example abstractions such as "TcpListener", "TcpStream", "UdpSocket", HTTP types, and a "SecureChannel"; it also contains example loopback addresses and conceptual return values.

This specification therefore establishes the following boundary:

grammar/spec/networking.md
        ↓
language semantics

grammar/networking/*.g4
        ↓
syntax

src/frontend/ast/
        ↓
AST

semantic/compiler layers
        ↓
portable networking semantics

src/stdlib/net.rs
        ↓
library/runtime-facing facilities

"src/stdlib/net.rs" MUST NOT be treated as the authority for:

- language syntax;
- universal networking semantics;
- protocol vocabulary;
- hardware limits;
- endpoint limits;
- transport availability;
- network topology.

Its current conceptual implementation MUST NOT be allowed to create an implicit language contract.

---

76. TCP/UDP/HTTP

TCP, UDP, HTTP and similar technologies MAY be exposed through libraries/interoperability/dialects.

They MUST NOT define the complete Zamani networking language.

This prevents the language from becoming obsolete when communication technology changes.

---

77. Quantum Networking

Quantum networking is a cross-domain capability.

Networking MAY describe:

remote quantum service
quantum communication endpoint
quantum communication capability
hybrid quantum-classical communication

but MUST defer quantum semantics to the quantum subsystem.

The repository's quantum architecture already identifies quantum networking as a supported domain interaction.

---

78. AI/ML Integration

Networking MUST support:

- distributed training;
- model serving;
- parameter exchange;
- tensor streaming;
- dataset distribution;
- inference services;
- agent communication;
- accelerator communication.

The AI grammar owns AI-specific semantics.

Networking owns communication.

This follows the repository's cross-domain architecture, where networking is expected to interact with AI without becoming an AI framework grammar.

---

79. HPC Integration

Networking MUST support communication patterns required by high-performance computing.

Examples include:

- collectives;
- reductions;
- synchronization;
- data exchange;
- distributed tensors;
- pipeline communication;
- accelerator communication.

The language MUST NOT require MPI, RDMA, InfiniBand, or a specific HPC fabric.

Those may be backend realizations.

---

80. Embedded and Edge Computing

Networking MUST support constrained systems without defining separate syntax for every hardware class.

The same networking semantics may execute on:

- tiny embedded targets;
- edge devices;
- gateways;
- servers;
- clusters;
- accelerators;
- cloud systems.

Resource requirements determine whether a target can satisfy the program.

---

81. Cloud Independence

Cloud deployment is not networking semantics.

A Zamani networking program MUST NOT inherently depend on:

- a particular cloud;
- a particular region;
- a particular availability zone;
- a particular load balancer;
- a particular service mesh.

Cloud-specific deployment belongs to deployment/interoperability layers.

---

82. Service Discovery

Service discovery MAY be expressed semantically.

The source program may request:

service("compute")

rather than:

connect to 192.0.2.1:8080

when logical service identity is sufficient.

Runtime/deployment resolves the service identity.

---

83. Name Resolution

Networking names MUST integrate with the universal name/module/path system.

The networking grammar MUST NOT invent a second name-resolution mechanism.

Resolution may occur through:

- module resolution;
- service discovery;
- deployment metadata;
- runtime discovery;
- explicit address resolution.

---

84. Locality

Networking MAY express locality requirements.

Examples:

prefer local communication
require same execution domain
prefer low-distance communication
avoid remote communication

Locality is a semantic/resource property.

It is not equivalent to:

use machine 3

---

85. Topology Independence

The source program MUST NOT assume a particular physical topology unless topology itself is part of the program's semantic requirement.

The same program may execute over:

bus
star
mesh
torus
tree
fat-tree
ring
fully connected
wireless
optical
shared memory
future topology

subject to capability and performance requirements.

---

86. Topology Requirements

A program MAY express a topology requirement where the topology is semantically meaningful.

For example:

requires communication topology satisfying T;

The exact physical topology is then a downstream realization problem.

The language MUST distinguish:

required topology property

from:

selected physical links

---

87. Address Stability

Physical addresses MUST NOT be assumed stable across execution environments unless explicitly required.

Logical service identities SHOULD be preferred for portable programs.

---

88. Portability

Networking syntax MUST remain portable across target classes whenever semantic requirements can be satisfied.

A portable program should not need source changes merely because:

- endpoint count changes;
- node count changes;
- network topology changes;
- transport changes;
- provider changes;
- machine size changes;
- hardware changes;
- accelerator count changes.

---

89. POCO-REAF

POCO-REAF is a language architecture principle.

Networking MUST preserve:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

subject to:

1. language compatibility;
2. semantic correctness;
3. target capability satisfaction;
4. resource availability;
5. explicit program constraints;
6. supported compiler/runtime contracts.

POCO-REAF does not mean every target can satisfy every program.

It means that target-specific realization MUST NOT unnecessarily contaminate the portable program's semantic source.

---

90. Compile-Once Semantics

Compile-once MUST NOT be interpreted as:

«one binary contains a preselected physical machine layout forever.»

Instead, the compiler/runtime architecture SHOULD preserve enough target-independent semantic information for later realization where the repository's deployment model permits it.

Target-specific lowering MAY occur where necessary.

The networking source model itself remains target independent.

---

91. Dynamic Capability Discovery

Where runtime adaptation is supported, a program MAY query or require capabilities dynamically.

The language MUST distinguish:

capability requirement

from:

implementation discovery

Discovery belongs to runtime/resource layers.

---

92. Graceful Scaling

A networking program MAY scale by:

- increasing endpoint population;
- increasing data;
- increasing communication parallelism;
- distributing services;
- increasing available resources.

The language MUST NOT require a new grammar construct merely because the scale changes.

---

93. No Fixed Participant Counts

The grammar MUST NOT define fixed forms such as:

send_to_8_nodes
broadcast_to_16
connect_32
cluster_of_64

unless these are ordinary program-level values expressed through general language constructs.

---

94. Generic Iteration

Large communication structures SHOULD be expressible through generic language facilities.

For example, conceptually:

for endpoint in endpoints {
    send message through endpoint;
}

rather than a grammar with separate rules for each possible participant count.

This keeps scale data-driven.

---

95. Generated Networking Structures

Networking structures MAY be generated from:

- program data;
- compile-time computation;
- metaprogramming;
- runtime discovery;
- resource information.

Generated structures MUST still undergo normal semantic validation.

---

96. Metaprogramming Boundary

Networking metaprogramming MUST NOT bypass:

- type checking;
- capability checking;
- security checking;
- resource checking;
- portability validation.

Generated networking syntax is still Zamani syntax and must enter the normal compilation pipeline.

---

97. Dialects

Provider-specific or experimental networking constructs SHOULD be represented through the dialect mechanism where they cannot be expressed through the core language.

A dialect MUST declare:

- name;
- version;
- syntax extensions;
- semantic extensions;
- AST mapping;
- IR mapping;
- compatibility;
- capability requirements.

A dialect MUST NOT silently modify core networking semantics.

---

98. Interoperability

Interoperability may expose:

- sockets;
- C networking;
- C++;
- Rust;
- Python;
- WebAssembly;
- operating-system APIs;
- vendor APIs;
- protocol libraries;
- network fabrics.

These are interoperability concerns.

They MUST NOT redefine Zamani's networking language.

---

99. Foreign ABI

ABI details belong to interoperability/compiler/backend layers.

The networking grammar MUST NOT require programmers to know:

- calling conventions;
- register assignments;
- stack layouts;
- ABI-specific packet structures;
- machine-specific addresses.

Unless explicitly writing an interoperability declaration.

---

100. Error Classification

Networking diagnostics MUST distinguish at least:

Syntax errors

Examples:

- malformed endpoint declaration;
- malformed channel;
- malformed message;
- malformed protocol;
- malformed service;
- invalid networking syntax.

Name errors

Examples:

- unknown endpoint;
- unknown service;
- unknown protocol.

Type errors

Examples:

- incompatible message type;
- invalid endpoint type.

Semantic errors

Examples:

- impossible communication requirement;
- incompatible delivery semantics.

Capability errors

Examples:

- required capability unavailable.

Resource errors

Examples:

- insufficient runtime resources.

Security errors

Examples:

- unsatisfied authentication requirement.

Deployment errors

Examples:

- no valid placement.

The parser MUST NOT report downstream semantic failures as syntax errors.

---

101. Diagnostics

Diagnostics SHOULD include:

- source span;
- error code;
- human-readable explanation;
- relevant networking construct;
- expected semantic category;
- actionable context;
- related declaration where appropriate.

The grammar MUST NOT embed Rust error-generation code.

---

102. Security Boundary

Networking parsing MUST remain side-effect free.

Parsing MUST NOT:

- open sockets;
- connect to hosts;
- resolve network addresses through live networking;
- access credentials;
- read secrets;
- inspect hardware;
- contact services.

---

103. Resource Safety

Networking syntax MUST NOT allocate runtime resources during parsing.

Parsing creates syntax information only.

Runtime resources are acquired after semantic analysis and compilation.

---

104. Safe Rust Requirement

Networking-related Rust implementation must remain compatible with:

Rust 1.97
Rust 1.97.1

and MUST use safe Rust.

No networking compiler/frontend implementation may require:

unsafe

The grammar itself contains no Rust implementation code.

---

105. Memory Safety

Networking semantic/runtime implementations MUST preserve Rust memory safety without "unsafe".

Ownership, borrowing, lifetimes, synchronization, and resource cleanup MUST use safe abstractions.

The networking specification does not authorize unsafe exceptions.

---

106. Concurrency Safety

Networking runtime implementations MUST avoid data races and unsafe shared mutable state.

The language-level concurrency model remains governed by the concurrency/effects/runtime architecture.

---

107. Resource Lifetime

Communication resources MUST have explicit semantic lifetime.

Relevant objects may include:

- endpoints;
- channels;
- streams;
- service handles;
- requests;
- subscriptions.

The runtime MUST release resources according to lifecycle semantics.

---

108. Channel Lifecycle

The semantic model SHOULD distinguish:

declared
available
opening
open
closing
closed
failed
cancelled

where such lifecycle states are semantically required.

The exact runtime state machine belongs downstream.

---

109. Service Lifecycle

Services may be:

- declared;
- available;
- starting;
- running;
- degraded;
- unavailable;
- stopping;
- stopped.

The networking grammar expresses service intent.

Runtime lifecycle remains outside the grammar.

---

110. Fault Tolerance

Networking MUST integrate with the repository's broader resilience architecture.

Networking MAY express requirements for:

- redundancy;
- retry;
- failover;
- durability;
- availability;
- recovery.

The networking domain MUST NOT duplicate the resilience subsystem.

---

111. ZQN Boundary

If quantum networking interacts with ZQN:

networking semantics
        ↓
quantum/distributed semantic integration
        ↓
ZQN

Networking MUST NOT define ZQN fault semantics.

ZQN remains responsible for its own fault/noise semantics.

---

112. QEC Boundary

Networking MUST NOT define QEC algorithms.

Where networking carries or coordinates quantum error-correction information, the semantic model may refer to QEC requirements.

QEC implementation remains owned by the quantum resilience/QEC subsystem.

---

113. Routing Boundary

Networking declares communication intent.

Routing determines physical realization.

Therefore:

networking
    ≠
routing

A route may depend on:

- topology;
- capacity;
- latency;
- congestion;
- reliability;
- security;
- energy;
- target hardware.

---

114. Scheduling Boundary

Networking declares communication dependencies and requirements.

Scheduling determines timing.

The networking specification MUST NOT prescribe a global scheduler.

---

115. Optimization Boundary

Compiler optimization may:

- combine communication;
- eliminate redundant communication;
- reorder safe operations;
- batch messages;
- fuse transfers;
- choose communication strategies.

Optimization MUST preserve observable networking semantics.

---

116. Observability

Networking MAY integrate with:

- tracing;
- metrics;
- profiling;
- logging;
- provenance;
- telemetry.

Observability MUST NOT alter program semantics unless explicitly requested.

---

117. Provenance

Networking operations may participate in provenance tracking.

Provenance may identify:

- source operation;
- message origin;
- service origin;
- transformation;
- execution context;
- deployment realization.

The grammar SHOULD expose semantic provenance hooks through generic attributes rather than hard-coding a single tracing backend.

---

118. Time

Networking MAY express timing requirements.

Examples:

- deadlines;
- timeouts;
- latency constraints;
- ordering;
- scheduling relationships.

Time units and representations must integrate with the universal Zamani type/temporal system.

Networking MUST NOT impose one machine clock model.

---

119. Clock Independence

The source program MUST NOT assume that distributed machines share an identical physical clock.

Distributed timing semantics must distinguish:

- logical ordering;
- local time;
- physical time;
- deadlines;
- elapsed duration.

---

120. Network Timeouts

A timeout is a semantic requirement when explicitly declared.

A timeout MUST NOT silently become a universal compiler constant.

---

121. Flow Control

Networking MAY express flow-control requirements.

Flow control remains separate from transport implementation.

A runtime may implement it through:

- credit-based mechanisms;
- windows;
- buffering;
- backpressure;
- scheduling;
- transport-specific facilities.

---

122. Congestion

Congestion management is generally a runtime/network realization concern.

The language may express a requirement such as:

avoid communication beyond specified resource conditions

but MUST NOT embed one congestion-control algorithm into the core grammar.

---

123. Network Policies

Networking policies MAY include:

- reliability;
- security;
- locality;
- resource usage;
- latency;
- bandwidth;
- retry;
- routing preference;
- service availability.

Policies must remain semantic declarations and integrate with the repository's general policy/effect/resource systems.

---

124. Network Hints

Hints are non-mandatory implementation guidance.

A hint MUST NOT be treated as a semantic guarantee.

For example:

prefer locality

does not mean:

must use local machine

unless declared as a requirement.

---

125. Negotiation

Networking may involve capability negotiation.

Negotiation MUST distinguish:

program requirements
target capabilities
negotiated realization

A failed negotiation is a semantic/runtime/deployment failure, not necessarily a parser failure.

---

126. Version Negotiation

Protocols and services may declare compatibility ranges.

Version compatibility MUST integrate with:

grammar/compatibility/

and MUST NOT require a new core grammar rule for every protocol version.

---

127. Backward Compatibility

Networking language changes MUST preserve source compatibility where the repository's compatibility policy requires it.

Breaking changes MUST be:

- versioned;
- documented;
- diagnosable;
- migratable where possible.

---

128. Forward Compatibility

The open-world model MUST allow future:

- protocols;
- transports;
- network technologies;
- hardware;
- communication patterns;
- quantum networking technologies;
- accelerator interconnects.

A future name MUST NOT require a core grammar change solely because the name is new.

---

129. Extensibility

Networking extensions SHOULD prefer:

generic constructs
+
names
+
attributes
+
capabilities
+
types
+
effects
+
dialects
+
interoperability

over adding one keyword for every technology.

---

130. Generic Operation Model

Networking operations SHOULD use a generic semantic model where appropriate.

Conceptually:

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

This is consistent with the repository's broader frontend architecture and avoids creating a giant closed enumeration of networking operations.

---

131. Avoid Keyword Explosion

The core networking grammar SHOULD NOT grow into a list containing every:

- protocol;
- cloud service;
- network device;
- vendor;
- transport;
- application framework;
- communication algorithm.

Language-level keywords should represent stable semantic concepts.

---

132. Networking Grammar Composition

The intended composition is:

networking.g4
    ├── endpoints.g4
    ├── channels.g4
    ├── messages.g4
    ├── protocols.g4
    ├── services.g4
    └── network-capabilities.g4

The existing composition grammar already establishes this architecture.

No component should redefine another component's public concepts.

---

133. Expression Integration

Networking expressions MUST use the universal Zamani expression grammar.

Examples of values may include:

- endpoint references;
- message values;
- addresses;
- capability values;
- resource expressions;
- timeout expressions;
- size expressions;
- policy expressions.

Networking MUST NOT create a parallel expression language.

---

134. Type Integration

Networking types MUST integrate with universal Zamani types.

Potential semantic categories include:

Endpoint
Channel
Message
Stream
Service
Protocol
Address
Capability

These should be represented through the type system rather than parser-specific ad hoc typing.

---

135. Module Integration

Networking declarations MUST participate in the normal module/import/export system.

A networking service can therefore be:

- defined in a module;
- imported;
- exported;
- namespaced;
- versioned;
- composed.

Networking MUST NOT create an independent module system.

---

136. Generic Networking

Networking constructs SHOULD support generic programming where useful.

For example:

Service<T>
Message<T>
Channel<T>
Stream<T>

may be semantic constructs where the type system permits them.

The grammar must not impose fixed payload types.

---

137. Tensor and Large Data Networking

Networking MUST support transfer of:

- vectors;
- matrices;
- tensors;
- datasets;
- model parameters;
- streams.

The grammar MUST NOT define a fixed tensor size or network payload ceiling.

Data shape remains a type/program concern.

---

138. Accelerator Communication

Networking may connect:

- CPU ↔ GPU;
- CPU ↔ FPGA;
- CPU ↔ ASIC;
- CPU ↔ QPU;
- GPU ↔ GPU;
- accelerator ↔ accelerator;
- host ↔ device.

The grammar describes communication intent.

Hardware/resource layers determine the realization.

---

139. Memory-Centric Communication

Some targets may communicate using shared memory rather than packets.

Therefore:

communication

MUST NOT inherently mean:

socket

A compiler may lower communication into shared memory where semantically valid.

---

140. Local and Remote Transparency

A logical communication abstraction MAY be realized:

within one address space

or:

between processes

or:

between machines

without requiring source rewriting, provided the program's semantics permit it.

This is a core POCO-REAF property.

---

141. Location Transparency

Logical services SHOULD be location-transparent unless the program explicitly requires location semantics.

The compiler/runtime may determine where the service executes.

---

142. Network Partitioning

Distributed/networked applications may experience partitions.

The networking semantic model MAY expose partition-sensitive policies.

Distributed consistency semantics remain owned by "grammar/distributed/".

---

143. Consistency Boundary

Networking MUST NOT define distributed consistency models such as:

- strong consistency;
- eventual consistency;
- causal consistency;

as networking implementation details.

When required, those semantics belong to distributed/data systems and may be referenced by networking.

---

144. Transactions

Networked transactions may cross communication boundaries.

Transaction semantics belong to the broader data/distributed/effects architecture.

Networking provides communication primitives needed by those systems.

---

145. Persistence

Networking may transport persistent data.

Persistence semantics belong to data/storage systems.

Networking MUST NOT make all messages persistent.

---

146. Reliability vs Durability

The language MUST distinguish:

communication reliability

from:

data durability

Successful message delivery does not necessarily mean durable storage.

---

147. Security vs Reliability

Security and reliability are independent dimensions.

For example:

reliable

does not imply:

secure

and:

secure

does not imply:

reliable

The semantic model MUST preserve this distinction.

---

148. Privacy

Networking may express privacy/confidentiality requirements.

Privacy semantics must integrate with security/data policy.

Networking does not implement privacy mechanisms.

---

149. Authentication

Authentication is a semantic security requirement.

Networking may require:

authenticated peer

but authentication implementation belongs to security/runtime layers.

---

150. Authorization

Networking may require authorization before communication.

Authorization policy belongs to security.

Networking only expresses the communication-side requirement.

---

151. Trust

Networking MAY reference trust requirements.

Trust evaluation is downstream.

The networking grammar MUST NOT implement trust scoring or trust databases.

---

152. Cryptography

Cryptographic algorithms are not core networking grammar.

They belong to:

grammar/security/

or interoperability/dialect/library layers.

---

153. Network Identity

Identity should be abstract.

A networking service may identify itself through:

- logical name;
- service identity;
- capability identity;
- authenticated identity.

Physical interface identity remains a deployment concern unless explicitly required.

---

154. Multi-Tenant Systems

Networking MAY support tenant isolation requirements.

Tenant semantics belong to resource/security/distributed layers.

Networking can express the communication requirement without implementing isolation.

---

155. Communication Domains

The semantic model may distinguish:

- local;
- process-local;
- host-local;
- cluster-local;
- regional;
- global;
- private;
- public;
- secure;
- isolated.

These are semantic categories, not physical implementation guarantees.

---

156. Global Communication

"Global" MUST NOT imply a fixed geographic topology.

It means the semantic scope permits communication across the relevant execution domain.

Deployment determines the physical realization.

---

157. Network Namespace

Networking names SHOULD be namespace-aware.

A protocol/service/channel name may be qualified.

The networking grammar should rely on the universal namespace/path system rather than inventing one.

---

158. Versioned Services

Services MAY expose versions.

Version syntax must integrate with general versioning/compatibility rules.

The grammar MUST NOT hard-code a maximum number of service versions.

---

159. Service Interfaces

A service interface MUST define semantic operations and data contracts.

It MUST NOT prescribe:

- socket APIs;
- operating-system APIs;
- physical ports;
- network interfaces.

---

160. Interface Compatibility

Service compatibility SHOULD consider:

- operation names;
- input types;
- output types;
- effects;
- capabilities;
- security requirements;
- protocol requirements;
- version compatibility.

---

161. Message Compatibility

Message evolution MUST integrate with the compatibility system.

Changes such as adding optional fields may be compatible where semantics allow.

The networking grammar does not decide wire compatibility alone.

---

162. Schema Evolution

Schema evolution belongs to the data/message semantic layer.

Networking carries schemas but does not own all schema semantics.

---

163. Protocol Composition

Protocols may be composed.

For example:

security
+
transport
+
application protocol

may form a communication stack.

The grammar MUST support protocol references/composition without embedding one fixed protocol stack.

---

164. Protocol Properties

Protocol properties SHOULD be represented semantically.

Examples:

ordered
reliable
streaming
authenticated
encrypted
idempotent
transactional

These are properties.

They are not necessarily protocol names.

---

165. Transport Selection

Transport selection may be based on:

capabilities
requirements
constraints
preferences
target
deployment

The source program should not need to specify a transport when semantics are sufficient.

---

166. Automatic Adaptation

Where multiple valid realizations exist, the compiler/runtime MAY select among them.

Selection MUST preserve:

- program semantics;
- security;
- correctness;
- explicit requirements.

---

167. Resource Exhaustion

Resource exhaustion is not a syntax error.

Examples:

- insufficient bandwidth;
- insufficient memory;
- endpoint capacity exhausted;
- service capacity exhausted.

Such failures belong to semantic/resource/runtime layers.

---

168. Runtime Resource Limits

Runtime implementations may impose operational limits.

Those limits MUST be:

- documented;
- externally configurable where appropriate;
- distinguishable from language limits;
- incapable of changing the grammar's semantic definition.

---

169. Parser Resource Limits

Parser implementations naturally have finite machine resources.

The grammar MUST NOT convert those implementation constraints into artificial language semantics.

A parser may reject due to resource exhaustion, but that is not equivalent to:

Zamani supports at most N networking constructs.

---

170. Compiler Resource Limits

Likewise, a compiler may have resource budgets.

Compiler failure due to resource exhaustion MUST NOT become a networking language limit.

---

171. Runtime Scaling

Runtime networking should scale according to:

- available resources;
- target capabilities;
- declared constraints;
- communication patterns.

The source language remains unchanged.

---

172. Parallel Communication

Networking MUST support parallel communication where semantics permit.

Parallelism belongs to the concurrency/distributed architecture.

The networking layer provides communication operations.

---

173. Pipeline Communication

Networking may participate in pipeline computation.

A pipeline may contain:

compute
→ communicate
→ compute
→ communicate

without requiring fixed machine counts.

---

174. Dataflow Communication

Networking may participate in dataflow programs.

Dataflow edges remain logical until lower layers choose physical communication mechanisms.

---

175. Actor Communication

Networking may carry actor messages.

Actor semantics remain owned by concurrency/distributed domains.

Networking carries communication.

---

176. RPC

RPC is a communication pattern.

It may be expressed through:

service
request
response
protocol

without making RPC a fundamental transport.

---

177. Remote Procedure Semantics

A remote call MUST preserve the language's type/effect/error semantics.

The runtime handles transport and dispatch.

---

178. Streaming RPC

Streaming RPC MAY support:

- request streams;
- response streams;
- bidirectional streams.

No fixed stream size belongs in the grammar.

---

179. Network Functions

Networking MAY expose reusable functions for:

- sending;
- receiving;
- connecting;
- accepting;
- publishing;
- subscribing;
- requesting;
- serving;
- streaming.

These should use normal Zamani function semantics.

---

180. Generic Network Operations

A new transport or communication operation SHOULD be expressible through generic operation mechanisms before a new keyword is considered.

This protects language stability.

---

181. Domain Composition

Networking MUST compose with:

classical
quantum
hybrid
HDL
hardware
distributed
AI
data
security
concurrency
effects
resources
execution

The repository's core architecture explicitly intends networking to coexist with these domains.

---

182. Networking + HDL

Networking may describe communication between hardware modules.

HDL owns:

- modules;
- signals;
- clocks;
- timing;
- hardware structures.

Networking owns logical communication semantics.

Hardware lowering determines the physical implementation.

---

183. Networking + Hardware/Software Co-Design

A single Zamani program may express:

algorithm
+
communication
+
accelerator intent
+
memory intent
+
timing constraints

without hard-coding a particular accelerator.

---

184. Networking + Data

Data pipelines may use networking for:

- distributed ingestion;
- processing;
- streaming;
- replication;
- transformation.

Data semantics remain owned by the data subsystem.

---

185. Networking + Security

Security requirements accompany networking operations.

Security analysis may reject a networking operation when required guarantees cannot be satisfied.

---

186. Networking + Effects

Communication effects MUST remain visible to effect analysis where relevant.

This supports:

- purity analysis;
- deterministic reasoning;
- capability enforcement;
- auditing.

---

187. Networking + Resources

Resource requirements MUST be analyzable independently from communication syntax.

For example:

communication requires capability C

must not mean:

select device D

---

188. Networking + Execution

Execution policy may determine:

- lifecycle;
- scheduling;
- retry;
- recovery;
- checkpointing;
- observability.

Networking supplies communication semantics.

---

189. Networking + Deployment

Deployment may determine:

- service placement;
- endpoint realization;
- provider;
- machine;
- network;
- address;
- transport.

These are not part of portable networking syntax unless explicitly requested.

---

190. Networking + Interoperability

External networking technologies are integrated through interoperability rather than by polluting the core grammar.

---

191. Networking + Dialects

Vendor/provider-specific networking should be isolated in dialects where appropriate.

---

192. Networking + Macros

Macros may generate networking declarations and operations.

Generated syntax must undergo ordinary validation.

Macros MUST NOT bypass networking semantic rules.

---

193. Networking + Metaprogramming

Compile-time networking metadata generation is permitted where the metaprogramming system permits it.

Compile-time code MUST NOT perform prohibited external network access merely because the generated program contains networking.

---

194. Compile-Time vs Runtime Networking

The language MUST distinguish:

describe networking

from:

perform live networking during compilation

Ordinary parsing/semantic analysis MUST remain deterministic and side-effect free.

---

195. No Live Network During Parsing

The compiler MUST NOT need to contact a service to determine whether a networking construct is syntactically valid.

---

196. Offline Compilation

Networking programs MUST be representable and compilable without live network access when all required source/dependency information is locally available.

---

197. Reproducibility

Networking compilation SHOULD be reproducible.

Network-dependent discovery MUST NOT silently change parsing or semantic interpretation.

External discovery results should enter the compilation/deployment process through explicit inputs.

---

198. Deterministic Builds

A networking program's grammar interpretation MUST be deterministic.

Compiler artifacts should record relevant networking capability/target inputs when target-dependent specialization occurs.

---

199. Provenance of Realization

When deployment chooses:

transport
route
endpoint
provider
machine
network

the resulting artifact SHOULD preserve provenance where repository infrastructure supports it.

The source program remains portable.

---

200. Conformance Matrix

Every networking feature MUST have traceability:

Specification
    ↓
Grammar
    ↓
Lexer
    ↓
Parser
    ↓
AST
    ↓
Semantic analysis
    ↓
IR
    ↓
Compiler
    ↓
Runtime
    ↓
Tests

A feature is not production complete if one of these required layers is undefined.

---

201. Per-File Contract

Each networking grammar/specification file MUST have:

File
Purpose
Status
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
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility Tests
Determinism Tests
Hard-Coding Audit
Diagnostics
Security
Performance
Completion Criteria

This allows a file to be completed independently without discovering missing architectural dependencies later.

---

202. "endpoints.g4" Completion Contract

"endpoints.g4" is complete only when:

- logical endpoint syntax is defined;
- endpoint identity is defined;
- endpoint references are defined;
- expressions integrate;
- names integrate;
- types integrate;
- source spans are preserved;
- physical identity is not assumed;
- endpoint multiplicity has no artificial limit;
- negative cases exist;
- boundary cases exist;
- scalability cases exist;
- AST mapping exists;
- semantic mapping exists.

---

203. "channels.g4" Completion Contract

"channels.g4" is complete only when:

- networking channels are defined;
- channel direction is defined;
- participant relationships are defined;
- lifecycle semantics are defined;
- networking/concurrency distinction is preserved;
- no channel-count limit exists;
- AST mapping exists;
- semantic mapping exists;
- resource/capability integration exists;
- negative/boundary/scalability tests exist.

---

204. "messages.g4" Completion Contract

"messages.g4" is complete only when:

- message declarations exist;
- fields integrate with types/data;
- metadata is defined;
- generic payloads are supported;
- no universal message-size limit exists;
- serialization remains downstream;
- schema evolution is addressed;
- AST/semantic/IR integration exists;
- positive/negative/boundary/scalability tests exist.

---

205. "protocols.g4" Completion Contract

"protocols.g4" is complete only when:

- protocol declarations exist;
- protocol references exist;
- protocol composition exists;
- protocol properties exist;
- open-world extensibility exists;
- transport names are not a closed grammar list;
- versioning integrates;
- dialects integrate;
- interoperability integrates;
- AST/semantic mapping exists;
- tests exist.

---

206. "services.g4" Completion Contract

"services.g4" is complete only when:

- service declarations exist;
- service operations exist;
- service contracts exist;
- endpoint association exists;
- protocol association exists;
- capability requirements exist;
- service placement remains downstream;
- deployment remains downstream;
- versioning exists;
- AST/semantic integration exists;
- tests exist.

---

207. "network-capabilities.g4" Completion Contract

"network-capabilities.g4" is complete only when:

- capability syntax exists;
- requirements are distinguishable from capabilities;
- requirements are distinguishable from preferences;
- requirements are distinguishable from implementation choices;
- capability names are open-world;
- hardware selection is not embedded;
- resource integration exists;
- security integration exists;
- semantic validation exists;
- tests exist.

---

208. Grammar-Level Hard-Coding Audit

The networking grammar MUST fail review if it introduces universal constants or fixed alternatives representing:

maximum endpoints
maximum nodes
maximum channels
maximum services
maximum messages
maximum payload
maximum connections
maximum topology size
maximum bandwidth
maximum latency
maximum network devices
maximum protocols

It MUST also reject accidental machine-specific constructs such as:

machine0
machine1
node0
node1
gpu0
qpu0
eth0
router0
port8080

when these are presented as universal networking language concepts.

Such names may appear as ordinary user-defined identifiers where syntactically legal; the prohibition concerns compiler-imposed universal assumptions.

---

209. Hard-Coding Audit of Existing Runtime Examples

The current "src/stdlib/net.rs" contains example loopback addresses and fixed example ports in its conceptual implementations.

Those examples MUST NOT be promoted into:

- grammar rules;
- language constants;
- universal networking semantics;
- portability guarantees.

If the runtime eventually implements actual networking, those values must become runtime/deployment inputs or explicit program values where appropriate.

---

210. Negative Tests

Networking conformance MUST reject malformed constructs such as:

- incomplete endpoint declarations;
- malformed channel declarations;
- invalid message fields;
- malformed protocol declarations;
- malformed service operations;
- invalid capability syntax;
- invalid type usage;
- invalid references;
- invalid attribute syntax.

Semantic tests MUST separately reject:

- impossible requirements;
- unknown required capabilities;
- invalid endpoint references;
- incompatible message types;
- incompatible service contracts;
- prohibited security combinations.

---

211. Boundary Tests

Boundary tests MUST cover:

- empty communication collections;
- one endpoint;
- one channel;
- one message;
- one service;
- zero-length streams where permitted;
- maximum representable program values;
- nested communication structures;
- deeply composed protocols;
- large generic types;
- large service interfaces.

No test may turn an implementation boundary into a language semantic ceiling.

---

212. Scalability Tests

Scalability tests MUST verify that networking semantics remain valid as program scale increases.

Test dimensions include:

endpoint count
channel count
message count
service count
protocol count
communication volume
data volume
concurrency
distributed participants
collective participants
stream length

Tests should use generated/parameterized inputs rather than embedding a single artificial maximum.

---

213. Cross-Domain Tests

Required integration tests include:

classical + networking
quantum + networking
hybrid + networking
HDL + networking
hardware + networking
distributed + networking
AI + networking
data + networking
security + networking
concurrency + networking
effects + networking
resources + networking

---

214. Quantum Cross-Domain Tests

At minimum:

classical computation
    ↓
quantum operation
    ↓
measurement
    ↓
network communication
    ↓
classical decision

must be representable where the corresponding language domains support it.

---

215. Distributed Cross-Domain Tests

At minimum:

logical service
    ↓
multiple logical participants
    ↓
communication
    ↓
distributed placement

must be representable without hard-coding node count.

---

216. Hardware Cross-Domain Tests

At minimum:

computation
+
communication requirement
+
hardware capability

must lower without requiring a fixed hardware topology.

---

217. AI Cross-Domain Tests

At minimum:

model
+
distributed computation
+
tensor communication

must be representable without requiring a specific accelerator vendor.

---

218. Security Cross-Domain Tests

At minimum:

service
+
authenticated communication
+
confidentiality requirement

must be expressible through the networking/security composition.

---

219. Determinism Tests

Tests MUST verify that parsing is independent of:

- host machine;
- network availability;
- filesystem state;
- current time;
- random state;
- hardware topology.

---

220. Portability Tests

The same source-level networking semantics SHOULD be tested against different abstract resource configurations.

For example:

small target
medium target
large target
distributed target
heterogeneous target
quantum-capable target

without changing source semantics.

---

221. Resource-Availability Tests

Tests MUST distinguish:

program invalid

from:

program valid but current target cannot satisfy requirements

This is essential for POCO-REAF.

---

222. Capability-Failure Tests

A valid networking program may fail compilation/deployment because the selected realization lacks a required capability.

That failure is not evidence that the networking grammar is invalid.

---

223. Runtime-Failure Tests

Runtime tests should separately cover:

- unavailable service;
- lost endpoint;
- failed channel;
- timeout;
- cancellation;
- security failure;
- resource exhaustion;
- protocol incompatibility.

---

224. Performance

Networking grammar parsing SHOULD remain proportional to the source/token structure and MUST avoid unnecessary semantic computation.

Performance optimization MUST NOT change language semantics.

---

225. Parser Complexity

The grammar should avoid pathological ambiguity and unnecessary backtracking.

Validation must check:

- unreachable rules;
- ambiguous alternatives;
- duplicate tokens;
- precedence conflicts;
- left-recursion problems;
- excessive parser complexity.

---

226. Memory Scalability

Compiler/frontend implementations MUST avoid unnecessary duplication of large message/service/networking structures.

Large source programs should be processed according to available resources.

The language itself must remain conceptually unbounded.

---

227. Streaming Compiler Inputs

Where supported by the frontend architecture, networking specifications SHOULD remain compatible with incremental parsing/analysis.

No networking feature should inherently require the complete networked deployment to be known during parsing.

---

228. Incremental Compilation

Changes to one networking declaration SHOULD not require reprocessing unrelated source when compiler architecture supports incremental compilation.

---

229. Caching

Compiler/runtime caching may cache:

- protocol metadata;
- service schemas;
- capability analysis;
- semantic results.

Caching MUST NOT alter language semantics.

---

230. Reproducible Deployment

Deployment artifacts SHOULD record target-dependent decisions without modifying the original portable source program.

---

231. No Source Rewriting for Scaling

Scaling from:

1 endpoint

to:

many endpoints

MUST NOT require rewriting the networking language constructs merely because the scale changes.

---

232. No Source Rewriting for Hardware

Moving from:

CPU

to:

GPU

or:

FPGA

or:

QPU

MUST NOT require networking source changes when communication semantics remain unchanged.

---

233. No Source Rewriting for Topology

Changing physical topology MUST NOT require source rewriting unless topology is explicitly part of the program's semantics.

---

234. No Source Rewriting for Provider

Changing deployment provider MUST NOT require changes to portable networking semantics.

---

235. No Source Rewriting for Transport

Changing transport implementation MUST NOT require changes to source semantics where the required communication guarantees remain satisfied.

---

236. Explicit Physical Requirements

Zamani MAY support explicit physical requirements for programs where physical realization is genuinely part of the intended computation.

Examples may include:

- hardware testing;
- networking research;
- embedded systems;
- deterministic topology experiments;
- hardware/software co-design.

These requirements MUST be explicit and isolated from universal language assumptions.

---

237. Semantic vs Realization Layer

The complete separation is:

PROGRAM SEMANTICS
    │
    ├── communication meaning
    ├── data
    ├── protocols
    ├── requirements
    └── correctness
    │
    ▼
REALIZATION
    │
    ├── endpoints
    ├── transports
    ├── routes
    ├── topology
    ├── hardware
    └── runtime

The compiler bridges these layers.

---

238. What Networking Grammar Must Never Become

Networking MUST NOT become:

a socket language
a TCP language
a cloud-provider language
a Linux networking language
a Kubernetes language
an MPI language
an RDMA language
a hardware-NIC language
a quantum-network vendor language

It is the networking component of one universal programming language.

---

239. What Networking Grammar Must Be

It MUST be:

logical
typed
effect-aware
capability-aware
resource-aware
security-aware
distributed-aware
quantum-compatible
hardware-neutral
transport-neutral
provider-neutral
extensible
deterministic
scalable
portable
versioned

---

240. Feature Lifecycle

Networking features follow:

proposed
    ↓
specified
    ↓
grammar implemented
    ↓
AST implemented
    ↓
semantic implementation
    ↓
IR integration
    ↓
compiler integration
    ↓
runtime integration
    ↓
conformance tests
    ↓
stable

A feature MUST NOT be declared stable merely because a grammar rule exists.

---

241. Existing Design Documents

"Zamani-Grammar.md" may contain broader or experimental networking concepts.

Those concepts MUST NOT become normative merely by appearing there.

They must pass the feature lifecycle above.

---

242. Historical/Experimental Features

Experimental networking features MUST be clearly marked.

Possible statuses:

stable
experimental
proposed
deprecated
historical
implemented
partially implemented
planned

This prevents design material from silently becoming production syntax.

---

243. Compatibility

Networking syntax must integrate with:

grammar/spec/compatibility.md
grammar/compatibility/

Compatibility decisions must include:

- grammar;
- lexer;
- parser;
- AST;
- semantics;
- IR;
- compiler;
- runtime.

---

244. Migration

If a networking construct changes, migration guidance MUST identify:

old syntax
new syntax
semantic difference
AST difference
compatibility impact
migration strategy

---

245. Deprecation

Deprecated networking syntax MUST remain parseable only as long as the language compatibility policy requires.

Diagnostics SHOULD identify the replacement.

---

246. Version Independence of Protocols

Protocol versioning MUST remain separate from Zamani language versioning.

For example:

Zamani language version

and:

service/protocol version

are independent dimensions.

---

247. Feature Gating

Experimental networking syntax MAY use feature gates.

Feature gating must integrate with the universal language feature-gate system.

---

248. Diagnostics for Unsupported Features

If a compiler recognizes syntax but the target lacks the required capability, the diagnostic should say that the realization is unsupported rather than claiming the source syntax is invalid.

---

249. Tooling

Networking tooling SHOULD support:

- syntax highlighting;
- completion;
- navigation;
- symbol lookup;
- service/schema inspection;
- capability inspection;
- protocol inspection;
- diagnostics;
- documentation;
- formatting.

Tooling MUST derive information from the same language authority.

---

250. Formatting

Formatting MUST NOT change networking semantics.

It should use the canonical grammar/parser rather than a separate networking parser.

---

251. Language Server Integration

The language server SHOULD expose:

- endpoint symbols;
- service symbols;
- protocol symbols;
- message types;
- channel types;
- capability requirements;
- semantic diagnostics.

---

252. Documentation Integration

Documentation generation should be able to extract:

- service interfaces;
- message schemas;
- protocol contracts;
- capability requirements.

Generated documentation MUST remain derived output.

---

253. Testing Architecture

Networking tests MUST exist at multiple levels:

lexical
syntax
AST
semantic
IR
compiler
runtime
integration
compatibility
scalability
security

---

254. Golden Tests

Golden parser/AST tests SHOULD preserve:

- source;
- parse result;
- AST result;
- diagnostics.

Golden files must be regenerated only through the authoritative toolchain.

---

255. Fuzz Testing

Networking parser fuzzing SHOULD cover:

- endpoint syntax;
- channel syntax;
- messages;
- protocols;
- services;
- capabilities;
- nested constructs;
- large generated programs.

Fuzzing MUST verify parser safety and termination.

---

256. Property Tests

Property tests SHOULD verify:

parse(format(parse(source))) ≡ parse(source)

where canonical formatting is defined.

They should also verify source-span preservation and semantic invariants.

---

257. Security Testing

Security tests MUST include:

- malformed network syntax;
- malicious identifiers;
- oversized generated constructs;
- deeply nested communication declarations;
- invalid security requirements;
- capability confusion;
- privilege-boundary violations.

---

258. No Unsafe Requirement

The networking grammar and compiler architecture MUST NOT depend on unsafe Rust.

This requirement applies to networking implementation code as well as any future supporting compiler/runtime modules.

---

259. Integration with "src/frontend/ast/"

Networking syntax maps into the existing canonical AST subsystem.

The repository has explicit domain/resource/capability/validation AST boundaries.

Therefore networking MUST use those generic mechanisms rather than create a disconnected AST hierarchy.

---

260. Generic AST Principle

Networking AST nodes SHOULD capture:

what was written
where it was written
what names were referenced
what expressions were supplied
what attributes/modifiers were supplied

They should not capture:

which socket was opened
which machine was selected
which route was chosen
which NIC was used

Those are downstream decisions.

---

261. Semantic Analysis

Networking semantic analysis MUST perform:

- name resolution;
- type checking;
- capability checking;
- effect checking;
- requirement validation;
- constraint validation;
- security validation;
- protocol compatibility;
- service compatibility;
- resource validation.

---

262. Semantic Error Examples

Examples:

unknown endpoint
unknown service
unknown protocol
incompatible message type
missing capability
incompatible security requirements
unsatisfied latency requirement
invalid channel direction
invalid service contract

These are semantic errors.

---

263. Runtime Adaptation

Where supported, runtime may adapt communication to changing resources.

Examples:

- service migration;
- route change;
- transport fallback;
- endpoint relocation;
- load redistribution.

Adaptation MUST preserve semantic guarantees.

---

264. Failure of Adaptation

If no valid realization satisfies the program requirements, execution may fail.

That does not invalidate the language program.

It means the current environment cannot satisfy the declared requirements.

---

265. Explicit Resource Budgets

Programs may explicitly specify resource budgets.

Examples:

maximum communication cost
maximum latency
maximum energy
maximum bandwidth consumption

These are program semantics.

They must not be converted into universal grammar constants.

---

266. Communication Cost

Communication cost MAY be expressed semantically.

Cost models belong to resource/optimization systems.

Networking grammar does not define one universal cost model.

---

267. Energy-Aware Networking

Networking may express energy preferences/requirements.

This is important for:

- embedded;
- edge;
- mobile;
- scientific;
- accelerator;
- future computational systems.

Energy realization belongs to hardware/runtime layers.

---

268. Thermal/Physical Constraints

Where relevant, communication may interact with:

- thermal constraints;
- power constraints;
- physical limits.

Those are hardware/resource semantics, not networking grammar implementation.

---

269. Future-Proofing

The networking language MUST remain open to future:

- communication media;
- protocols;
- architectures;
- network topologies;
- quantum communication technologies;
- photonic communication;
- molecular/nano communication;
- neuromorphic communication;
- biological computation communication;
- future computational substrates.

Future technologies must be representable through stable semantic abstractions.

---

270. Nano/Future Networking

If future nano or other computational domains use networking-like communication, they SHOULD consume the generic networking model rather than requiring an entirely separate communication language.

---

271. Communication as a Universal Capability

Networking is not restricted to computers connected through today's Internet.

Communication can mean exchange between computational entities across any supported substrate.

The language therefore models communication abstractly.

---

272. Semantic Closure

A networking feature is semantically complete only when all of these are known:

syntax
AST representation
typing
effects
capabilities
requirements
constraints
security
resource interaction
IR mapping
compiler consumer
runtime consumer
diagnostics
compatibility
tests

---

273. Independent Completion Principle

No networking grammar file should require future re-editing merely because another subsystem later becomes implemented.

Its contracts MUST be defined now.

For example:

"messages.g4" must already specify that serialization is downstream.

"protocols.g4" must already specify that transports are open-world.

"endpoints.g4" must already specify that physical addresses are not inherent.

"network-capabilities.g4" must already specify capability/requirement separation.

This is the required independently-completable architecture.

---

274. Dependency Direction

The dependency direction MUST remain:

core
 ↓
expressions/types
 ↓
networking syntax
 ↓
frontend AST
 ↓
semantic analysis
 ↓
IR
 ↓
compiler
 ↓
runtime

Not:

networking grammar
 ↓
runtime
 ↓
hardware

---

275. Circular Dependency Prohibition

Networking grammar MUST NOT create circular dependencies between:

- networking;
- distributed;
- quantum;
- hardware;
- runtime;
- compiler.

Cross-domain relationships should be represented through shared semantic contracts.

---

276. Shared Concepts

The following concepts should normally be shared through core/resource/type/effect systems:

Name
Type
Expression
Attribute
Capability
Requirement
Constraint
Effect
Resource
Target
SourceSpan
Diagnostic

Networking should reuse them.

---

277. Domain-Specific Concepts

Networking-specific concepts include:

Endpoint
Channel
Message
Protocol
Service
Communication
Stream
Subscription

These should not be duplicated in unrelated domains.

---

278. Channel Naming Conflict

The existence of:

grammar/concurrency/channels.g4

and:

grammar/networking/channels.g4

is intentional.

They MUST retain distinct semantic ownership.

The frontend semantic layer may normalize or relate them where necessary.

---

279. Message Naming Conflict

If data/domain systems also contain message-like constructs, networking MUST distinguish:

data/message schema

from:

communication/message

A communication message carries a value; it does not necessarily own the underlying data type.

---

280. Service vs Module

A service is a communication-facing abstraction.

A module is a source organization abstraction.

They are not equivalent.

---

281. Endpoint vs Process

An endpoint is a communication abstraction.

A process is an execution abstraction.

They are not equivalent.

---

282. Channel vs Socket

A channel is a logical communication abstraction.

A socket is one possible runtime realization.

They are not equivalent.

---

283. Protocol vs Transport

A protocol defines communication semantics.

A transport is one mechanism used to realize communication.

They are not equivalent.

---

284. Network vs Topology

A network is a semantic communication environment.

A physical topology describes realization.

They are not equivalent.

---

285. Capability vs Device

A capability describes what can be provided.

A device is one possible provider.

They are not equivalent.

---

286. Resource vs Address

A resource describes something available to computation.

An address identifies a communication destination.

They are not equivalent.

---

287. Program Meaning

The source program's meaning MUST be independent of implementation choices not explicitly included in its semantics.

This is the primary networking portability rule.

---

288. Implementation Freedom

Compiler/runtime implementations are free to choose among valid realizations.

Examples:

shared memory
socket
RDMA
accelerator fabric
wireless
optical
quantum
future transport

provided the semantic contract remains satisfied.

---

289. Observational Equivalence

Different networking implementations are valid when they preserve the observable semantics required by the program.

Optimization and adaptation must preserve:

- values;
- ordering guarantees;
- errors;
- security guarantees;
- explicit resource constraints;
- externally observable effects.

---

290. No Hidden Semantic Changes

A runtime MUST NOT silently weaken:

reliability
security
ordering
delivery
consistency

requirements merely because the selected transport cannot provide them.

It must reject, adapt, or choose another valid realization.

---

291. Capability Negotiation Failure

If no available target satisfies:

requires capability C

the compiler/deployer/runtime must report unsatisfied capability rather than silently violating the requirement.

---

292. Preference Failure

A preference may be ignored if no target satisfies it.

The semantic distinction between preference and requirement MUST be preserved.

---

293. Hint Failure

A hint may be ignored without making the program invalid.

---

294. Constraint Failure

A hard constraint must not be violated by a valid realization.

---

295. Requirement Satisfaction

Requirements may be satisfied through:

- direct capability;
- composition of capabilities;
- compiler transformation;
- runtime adaptation;
- hardware support.

The language does not prescribe the mechanism.

---

296. Communication Composition

Communication operations MUST be composable.

Examples:

service
    + protocol
    + channel
    + message
    + security
    + capability

should form one semantic communication contract.

---

297. No Duplicate Semantic Ownership

If another subsystem already owns:

security
resource
effect
type
distributed placement
quantum semantics
hardware capability

networking references those concepts instead of redefining them.

---

298. Production Readiness Gate

"grammar/spec/networking.md" is considered integrated only when:

- networking semantics are specified;
- grammar mappings are defined;
- AST mappings are defined;
- semantic mappings are defined;
- IR boundary is defined;
- compiler boundary is defined;
- runtime boundary is defined;
- distributed boundary is defined;
- quantum boundary is defined;
- hardware boundary is defined;
- security boundary is defined;
- resource/capability separation is defined;
- POCO-REAF rules are defined;
- hard-coding audit exists;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- compatibility tests exist;
- determinism tests exist.

---

299. Final Networking Architecture

The complete architecture is:

                    ZAMANI SOURCE
                          │
                          ▼
                 Universal Lexer
                          │
                          ▼
                 Universal Parser
                          │
                          ▼
                  Frontend AST
                          │
                          ▼
        ┌─────────────────────────────────┐
        │      Semantic Networking        │
        │                                 │
        │ Endpoint                        │
        │ Channel                         │
        │ Message                         │
        │ Protocol                        │
        │ Service                         │
        │ Stream                          │
        │ Requirements                    │
        │ Capabilities                    │
        │ Constraints                     │
        │ Effects                         │
        └─────────────────────────────────┘
                          │
             ┌────────────┼─────────────┐
             │            │             │
             ▼            ▼             ▼
        Classical      Quantum       Distributed
             │            │             │
             │       quantum::ir        │
             │            │             │
             └────────────┼─────────────┘
                          │
                    Canonical IR
                          │
                          ▼
                     Optimization
                          │
              ┌───────────┼───────────┐
              ▼           ▼           ▼
           Placement    Routing    Scheduling
              │           │           │
              └───────────┼───────────┘
                          ▼
                    Deployment
                          │
                          ▼
                  Runtime Realization
                          │
       ┌──────────────────┼───────────────────┐
       │                  │                   │
    Shared memory      Network fabric      Future
       │                  │                substrate
       │                  │                   │
       └──────────────────┼───────────────────┘
                          │
                   Actual resources

---

300. Final Invariants

The following invariants are mandatory.

Invariant 1 — One Language

Networking is part of Zamani.

It is not a separate language.

Invariant 2 — One Grammar Authority

"grammar/Zamani.g4" remains the canonical grammar composition root.

Invariant 3 — One Frontend AST

Networking constructs lower into the existing canonical frontend AST.

Invariant 4 — No Second Quantum IR

Quantum participation ultimately integrates through the canonical "quantum::ir" architecture.

Invariant 5 — Open-World Networking

New protocols, transports, devices, providers and technologies must not require core grammar keyword additions merely because their names are new.

Invariant 6 — No Artificial Limits

There is no universal maximum for:

nodes
endpoints
channels
messages
services
connections
payloads
protocols
topology
bandwidth
latency

Invariant 7 — Resource-Driven Scaling

Actual scale is determined by:

program semantics
+
available resources
+
target capabilities
+
explicit constraints

Invariant 8 — Semantic/Realization Separation

The language says what communication means.

The compiler/runtime determines how it happens.

Invariant 9 — Requirement/Preference Separation

Requirements MUST be preserved.

Preferences MAY be optimized.

Hints MAY be ignored.

Invariant 10 — Hardware Independence

Networking syntax MUST NOT encode today's machine topology as universal language semantics.

Invariant 11 — Deterministic Parsing

Parsing depends only on the supplied source/token stream and grammar.

Invariant 12 — Safe Rust

Networking compiler/runtime implementation MUST support Rust 1.97/1.97.1 without "unsafe".

Invariant 13 — Cross-Domain Composition

Networking MUST compose with:

classical
quantum
hybrid
HDL
hardware
distributed
AI
data
security
concurrency
effects
resources
execution

without duplicating their semantic ownership.

Invariant 14 — Independent Completion

Every networking component must have its syntax, AST, semantic, IR, compiler, runtime, test, compatibility, scalability and integration contracts defined before it is declared complete.

Invariant 15 — POCO-REAF

Networking MUST preserve the architectural goal:

PROGRAM ONCE
      ↓
COMPILE ONCE
      ↓
RUN EVERYWHERE
      ↓
RUN ANYWHERE
      ↓
RUN FOREVER

subject to semantic correctness, compatibility, explicitly declared requirements, and the capabilities/resources available to the realization.

---

301. Completion Checklist

This specification is complete when all of the following are true:

- [x] networking ownership defined;
- [x] grammar ownership defined;
- [x] AST ownership defined;
- [x] semantic ownership defined;
- [x] IR boundary defined;
- [x] compiler boundary defined;
- [x] runtime boundary defined;
- [x] resource boundary defined;
- [x] capability boundary defined;
- [x] security boundary defined;
- [x] distributed boundary defined;
- [x] quantum boundary defined;
- [x] hardware boundary defined;
- [x] concurrency boundary defined;
- [x] data boundary defined;
- [x] interoperability boundary defined;
- [x] dialect boundary defined;
- [x] POCO-REAF defined;
- [x] open-world protocol model defined;
- [x] no-hard-coded-limit policy defined;
- [x] scalability model defined;
- [x] deterministic parsing requirements defined;
- [x] source-span requirements defined;
- [x] diagnostics requirements defined;
- [x] compatibility requirements defined;
- [x] positive-test requirements defined;
- [x] negative-test requirements defined;
- [x] boundary-test requirements defined;
- [x] scalability-test requirements defined;
- [x] cross-domain test requirements defined;
- [x] safe-Rust requirement defined;
- [x] existing repository networking files integrated;
- [x] existing standard-library networking boundary explicitly defined;
- [x] no competing networking authority introduced;
- [x] no physical network implementation placed in grammar;
- [x] no second networking AST/IR authorized;
- [x] future networking technologies remain representable.

---

302. Normative Summary

Zamani networking is a portable semantic communication system, not a fixed socket API.

Its fundamental abstraction is:

logical communication intent

rather than:

physical network configuration

The programmer describes:

who communicates
what is communicated
how communication must behave
what guarantees are required
what capabilities are needed
what constraints apply
what resources are preferred

The compiler and runtime determine:

where
when
through which transport
over which topology
using which hardware
using which resources

The networking language therefore remains capable of scaling from tiny systems to arbitrarily large systems supported by available resources, without turning today's hardware, network technologies, providers, or topology into permanent language limits.

The authoritative relationship is:

grammar/spec/networking.md
        ↓
normative meaning

grammar/networking/*.g4
        ↓
syntax

grammar/Zamani.g4
        ↓
language composition

src/frontend/ast/
        ↓
canonical AST

semantic analysis
        ↓
networking meaning

canonical/domain IR
        ↓
optimization

placement/routing/scheduling
        ↓
realization

runtime/hardware
        ↓
actual communication

That separation is the networking-specific foundation required for Zamani's broader Program Once, Compile Once, Run Everywhere, Anywhere, Forever architecture.