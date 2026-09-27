Zamani Networking Grammar

Production Architecture, Ownership, Integration, Scalability, Portability and Conformance Contract

Path: "grammar/networking/README.md"
Language: Zamani
Repository: "Benwellonedge28/Zamani"
Domain: Networking, communication, service interaction, distributed communication and network capability intent
Grammar technology: ANTLR4 grammar composition
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Safety: Production Rust implementation MUST use safe Rust; "unsafe" is prohibited
Portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)
Scalability objective: From the smallest useful communication computation to arbitrarily large communication systems, constrained only by program semantics, explicitly declared requirements, actual implementation capacity, available resources, target capabilities and physical reality — never by arbitrary language-level ceilings.

---

1. Purpose

"grammar/networking/" defines the source-language syntax and composition boundary for networking and communication semantics in Zamani.

Networking is a first-class Zamani domain, but this directory is not a networking runtime.

This directory describes:

- what communication means to the program;
- who or what communicates;
- what messages are exchanged;
- what communication relationships exist;
- what protocols or protocol properties are required;
- what services are exposed;
- what communication capabilities are required;
- what logical routes or communication policies are declared;
- what streaming or distributed-compute communication intent exists.

It does not directly determine:

- which physical network is selected;
- which machine executes a service;
- which NIC is used;
- which physical port is used;
- which router is selected;
- which switch is selected;
- which transport implementation is used;
- which operating-system socket implementation is used;
- which cloud provider is used;
- which physical node participates;
- how routing is physically realized;
- how scheduling occurs;
- how bandwidth is allocated;
- how packets are transmitted;
- how serialization is implemented;
- how encryption is implemented;
- how authentication is implemented;
- how hardware is discovered;
- how runtime resources are allocated.

Those responsibilities belong to downstream semantic, resource, security, compiler, routing, scheduling, deployment, runtime, HAL, hardware and interoperability layers.

---

2. Non-Negotiable Architectural Rule

The networking grammar describes:

«portable communication intent»

It does not describe:

«a particular physical network realization»

Therefore:

Zamani source
    ↓
Lexer
    ↓
Parser
    ↓
Frontend AST
    ↓
Name / module resolution
    ↓
Type analysis
    ↓
Effect analysis
    ↓
Capability analysis
    ↓
Resource / constraint analysis
    ↓
Networking semantic model
    ↓
Canonical semantic representation / IR
    ↓
Optimization
    ↓
Routing
    ↓
Placement
    ↓
Scheduling
    ↓
Deployment
    ↓
Runtime / HAL
    ↓
Actual network realization

The networking grammar MUST NOT bypass this pipeline.

---

3. POCO-REAF

Zamani networking MUST preserve:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Run Anywhere
      ↓
Run Forever

POCO-REAF means that the source program expresses stable computational and communication semantics rather than unnecessarily encoding the characteristics of one machine, one network, one vendor or one deployment.

A networking program may express:

service compute;
message Result;
channel results;
send result through results;

without requiring:

machine 0
machine 1
interface eth0
port 8080
router 3
device 7
link 12

unless those physical properties are explicitly part of the program's semantics.

The following are distinct:

communication intent
        ≠
resource requirement
        ≠
capability requirement
        ≠
deployment policy
        ≠
physical placement
        ≠
runtime realization

---

4. Scalability Contract

The networking grammar MUST scale conceptually from:

one endpoint
one message
one request
one response
one connection
one stream
one service

to:

arbitrarily many endpoints
arbitrarily many messages
arbitrarily many requests
arbitrarily many responses
arbitrarily many connections
arbitrarily many streams
arbitrarily many services
arbitrarily large distributed systems

subject to actual resources and explicitly declared semantic requirements.

The grammar MUST NOT establish artificial universal limits such as:

MAX_ENDPOINTS
MAX_ADDRESSES
MAX_CHANNELS
MAX_MESSAGES
MAX_PROTOCOLS
MAX_REQUESTS
MAX_RESPONSES
MAX_ROUTES
MAX_SERVICES
MAX_SOCKETS
MAX_STREAMS
MAX_CONNECTIONS
MAX_NODES
MAX_NETWORK_SIZE
MAX_BANDWIDTH
MAX_THREADS
MAX_DEVICES
MAX_NETWORK_INTERFACES

or equivalent restrictions.

The prohibition also applies to hidden limits expressed indirectly through finite grammar alternatives.

For example, the grammar MUST NOT model:

endpoint0
endpoint1
endpoint2
...
endpoint1023

as the universal endpoint model.

Repeated constructs must use grammar repetition and semantic collections.

---

5. "Nothing Must Be Hard Coded" — Correct Interpretation

The prohibition on hard-coded limits does not prohibit ordinary program constants.

This is valid:

let count = 1024;

because "1024" is program data.

This is also potentially valid:

requires bandwidth >= required_bandwidth;

The following is invalid as a universal language architecture:

MAX_CONNECTIONS = 1024;

if that establishes the maximum number of connections expressible by Zamani.

Likewise:

network grammar supports at most 1024 endpoints

is prohibited.

The same principle applies to:

- endpoint counts;
- service counts;
- node counts;
- connection counts;
- stream counts;
- message counts;
- route counts;
- network dimensions;
- bandwidth;
- latency;
- address spaces;
- network devices;
- communication domains.

---

6. Current Repository Inventory

The networking directory currently contains the following authoritative component boundaries:

grammar/networking/
├── README.md
├── networking.g4
├── addresses.g4
├── endpoints.g4
├── channels.g4
├── messages.g4
├── protocols.g4
├── requests.g4
├── responses.g4
├── routing.g4
├── service-discovery.g4
├── services.g4
├── sockets.g4
├── streaming.g4
├── distributed-compute.g4
└── network-capabilities.g4

No existing networking file should be unnecessarily renamed.

The current aggregate grammar already imports these components and provides adapters for them.

The README MUST therefore describe this actual architecture rather than an older reduced inventory.

---

7. File Ownership Matrix

File| Owns| Does not own
"README.md"| networking architecture and contracts| executable grammar
"networking.g4"| networking composition and public aggregate boundary| leaf grammar implementations
"addresses.g4"| logical address syntax| physical interface allocation
"endpoints.g4"| logical communication endpoint syntax| physical endpoint allocation
"channels.g4"| networking-channel syntax| concurrency-channel semantics
"messages.g4"| message declarations| serialization implementation
"protocols.g4"| protocol declarations and intent| transport implementation
"requests.g4"| request syntax/contracts| server implementation
"responses.g4"| response syntax/contracts| runtime response generation
"routing.g4"| logical routing intent/policy| physical routing algorithm
"service-discovery.g4"| service-discovery declarations/intent| discovery runtime
"services.g4"| service declarations/contracts| service deployment
"sockets.g4"| logical socket abstraction syntax| OS socket allocation
"streaming.g4"| stream communication syntax| stream runtime
"distributed-compute.g4"| networking-facing distributed computation intent| complete distributed execution semantics
"network-capabilities.g4"| communication capability requirements| hardware capability discovery
parent "grammar/Zamani.g4"| canonical language composition| networking implementation details
"grammar/specification/"| normative language meaning| implementation
"grammar/spec/"| focused normative contracts| runtime behavior
"src/frontend/ast/"| domain-neutral AST| physical network realization
semantic layer| meaning and validation| parsing
canonical IR| computational representation| source grammar
routing| physical/logical route realization| source-language meaning
scheduling| communication ordering/timing| source grammar
runtime/HAL| execution and target realization| language syntax

No file may silently assume ownership belonging to another file.

---

8. Aggregate Grammar: "networking.g4"

"networking.g4" is the single networking-domain composition grammar.

It is not a second language.

It is not a networking runtime.

It is not an IR.

It is the aggregation boundary for all networking component grammars.

The current composition includes:

Addresses
Endpoints
NetworkingChannels
Messages
Protocols
NetworkingRequests
NetworkingResponses
NetworkingRoutes
NetworkingServiceDiscovery
NetworkingServices
Sockets
NetworkingStreaming
NetworkingDistributedCompute
NetworkCapabilities

The aggregate exposes the stable networking boundaries:

networkingUnit
networkingConstruct
networkingDeclaration
networkingElement
universalNetworking

The wider Zamani parser should consume the networking domain through this aggregate boundary rather than importing every leaf grammar independently.

---

9. Aggregate Dependency Direction

The required direction is:

addresses.g4
endpoints.g4
channels.g4
messages.g4
protocols.g4
requests.g4
responses.g4
routing.g4
service-discovery.g4
services.g4
sockets.g4
streaming.g4
distributed-compute.g4
network-capabilities.g4
        │
        ▼
networking.g4
        │
        ▼
canonical Zamani parser
        │
        ▼
frontend AST

The reverse dependency is prohibited.

Networking component grammars MUST NOT import the universal parser.

Networking component grammars MUST NOT import the root language grammar merely to obtain higher-level networking constructs.

This prevents grammar cycles.

---

10. "networkingUnit"

The standalone networking grammar must support a complete networking-domain parse boundary.

Conceptually:

networkingUnit
    : networkingConstruct* EOF
    ;

This boundary is intentionally unbounded.

"EOF" is required so that standalone networking validation does not accidentally accept a valid networking prefix followed by invalid input.

---

11. "networkingConstruct"

The aggregate dispatch must cover every existing networking component:

networkingAddress
networkingEndpoint
networkingChannel
networkingMessage
networkingProtocol
networkingRequest
networkingResponse
networkingRoute
networkingServiceDiscovery
networkingService
networkingSocket
networkingStream
networkingDistributedCompute
networkingCapability

The aggregate owns the dispatch.

Each leaf grammar owns its implementation.

No component implementation should be duplicated inside "networking.g4".

---

12. Canonical Zamani Integration

The wider Zamani language has a universal networking integration boundary.

The networking directory must preserve a stable adapter equivalent to:

networkingDeclaration
    : networkingConstruct
    ;

and:

networkingElement
    : networkingDeclaration
    ;

and:

universalNetworking
    : networkingDeclaration
    ;

These are integration adapters.

They do not create another networking syntax.

They exist so that the universal language composition can consume networking as one domain.

---

13. Root Grammar Integration

The repository's canonical architecture requires:

grammar/Zamani.g4

to remain the visible language composition root.

Any generated/internal parser grammar such as:

grammar/antlr/ZamaniParser.g4

must not become a competing specification.

The intended relationship is:

grammar/Zamani.g4
        ↓
canonical composition
        ↓
ANTLR parser representation/generated parser infrastructure
        ↓
networking.g4

If generated ANTLR artifacts are maintained under "grammar/antlr/", they are implementation artifacts, not a second source-language authority.

The networking README therefore treats the root grammar as authoritative and "networking.g4" as the networking-domain aggregate.

---

14. Endpoint Architecture — "endpoints.g4"

"endpoints.g4" owns logical communication endpoints.

An endpoint is an addressable communication participant.

It is not inherently:

- a machine;
- a process;
- a NIC;
- a physical port;
- a CPU;
- a GPU;
- an FPGA;
- a QPU;
- a cloud instance.

Endpoint syntax must therefore remain target-neutral.

Owns

- endpoint declarations;
- endpoint identity;
- endpoint references;
- endpoint attributes;
- logical endpoint relationships.

Does not own

- IP allocation;
- MAC allocation;
- NIC selection;
- physical port selection;
- DNS implementation;
- machine discovery;
- node allocation;
- routing.

Integration

endpoint syntax
    ↓
endpoint AST
    ↓
name resolution
    ↓
endpoint semantic model
    ↓
resource/capability analysis
    ↓
placement/discovery
    ↓
runtime realization

---

15. Address Architecture — "addresses.g4"

"addresses.g4" owns address syntax and address references.

Address syntax must support the abstraction of an address without forcing every address to be a particular physical network technology.

The grammar must remain open to:

- network addresses;
- logical addresses;
- service addresses;
- endpoint-relative addresses;
- names;
- qualified addresses;
- future address families;
- dialect-defined address forms.

It must not turn current protocols or address families into permanent language ceilings.

Does not own

- DNS resolver implementation;
- route selection;
- network-interface selection;
- IP assignment;
- DHCP;
- ARP;
- physical network discovery.

An address may be syntactically valid while being semantically unresolved.

That is intentional.

---

16. Networking Channels — "channels.g4"

"channels.g4" owns network communication channels.

It must remain distinct from:

grammar/concurrency/channels.g4

The two concepts are related but not identical.

Networking channel

Represents communication across a networking boundary.

Concurrency channel

Represents language-level task synchronization or communication.

The semantic layer may connect the two.

The grammar must not collapse them into one abstraction.

---

17. Message Architecture — "messages.g4"

"messages.g4" owns source-level message declarations.

A message represents information exchanged between communication participants.

It may reference:

- Zamani types;
- generic types;
- records;
- schemas;
- data structures;
- expressions;
- attributes.

It does not own:

- wire serialization;
- compression;
- packetization;
- framing;
- encryption;
- transport encoding.

The intended pipeline is:

message syntax
    ↓
message AST
    ↓
type/data analysis
    ↓
message semantic contract
    ↓
serialization selection
    ↓
wire representation

A message's physical size is not a grammar-level fixed limit.

---

18. Protocol Architecture — "protocols.g4"

Protocols must follow an open-world model.

The grammar must not become a closed enumeration of current protocols.

The language must not require permanent parser keywords for:

TCP
UDP
QUIC
HTTP
MQTT
gRPC
MPI
RDMA
InfiniBand

or any vendor-specific future technology.

These identities can be represented through:

- names;
- declarations;
- protocol types;
- capabilities;
- dialects;
- modules;
- libraries;
- semantic registries.

A new protocol should normally be introducible without modifying the core Zamani grammar merely because its name is new.

---

19. Request Architecture — "requests.g4"

"requests.g4" owns the source representation of communication requests.

A request describes an intended operation against a communication/service boundary.

It may express:

- target service;
- operation;
- parameters;
- metadata;
- requirements;
- timeout intent;
- reliability intent;
- security requirements;
- response expectations.

It does not implement:

- client runtimes;
- transport selection;
- retries;
- connection pools;
- physical sockets;
- load balancers.

Those are semantic/runtime concerns.

---

20. Response Architecture — "responses.g4"

"responses.g4" owns source-level response contracts.

It must remain symmetric with request semantics without becoming coupled to a specific runtime protocol.

A response may describe:

- returned values;
- result types;
- error outcomes;
- metadata;
- status semantics;
- streaming relationships.

It does not implement:

- packet responses;
- HTTP response generation;
- socket writes;
- transport serialization.

---

21. Routing Architecture — "routing.g4"

"routing.g4" owns logical routing intent, not physical routing algorithms.

It may express semantic policies such as:

- preferred route characteristics;
- locality;
- path constraints;
- reliability;
- latency requirements;
- topology requirements;
- communication policy;
- route preferences.

It must not decide the final physical path.

The actual pipeline is:

routing intent
    ↓
semantic validation
    ↓
resource/capability model
    ↓
topology discovery
    ↓
route planning
    ↓
optimization
    ↓
physical/network realization

No router IDs or fixed topology sizes belong in the universal grammar.

---

22. Service Discovery — "service-discovery.g4"

Service discovery syntax describes discovery intent and contracts.

It must not implement a specific discovery technology.

The grammar must remain independent of:

- DNS;
- mDNS;
- Consul;
- Kubernetes;
- cloud provider discovery;
- service mesh implementations;
- registry implementation;
- operating-system service managers.

A future discovery mechanism should be representable through the open-world capability/module/dialect architecture.

---

23. Services — "services.g4"

"services.g4" owns logical service declarations.

A service describes:

what is offered
what operations exist
what types are accepted
what types are returned
what communication requirements apply

It does not inherently define:

where it runs

Therefore a service should not implicitly encode:

run on node 0
run on GPU 2
run on QPU 1
run on machine X

unless those are explicitly expressed as program requirements or deployment policy.

---

24. Sockets — "sockets.g4"

Sockets are treated as a logical networking abstraction.

The grammar must not imply an operating-system implementation.

The following distinction is mandatory:

logical socket
    ≠
OS socket object
    ≠
physical interface
    ≠
physical port

"socket.g4" may express:

- socket declarations;
- socket references;
- logical socket properties;
- endpoint relationships;
- protocol relationships;
- channel relationships;
- capability requirements.

Runtime owns:

- allocation;
- opening;
- binding;
- connecting;
- closing;
- transport selection;
- OS integration.

---

25. Streaming — "streaming.g4"

"streaming.g4" owns communication-stream syntax.

A stream may represent:

- continuous data;
- asynchronous communication;
- request/response streams;
- bidirectional streams;
- event streams;
- data pipelines.

The grammar must not impose:

MAX_STREAM_ITEMS
MAX_STREAMS
MAX_STREAM_SIZE
MAX_STREAM_DURATION

as universal limits.

Actual limits belong to:

- semantic constraints;
- runtime;
- resource analysis;
- target capabilities;
- deployment policies.

---

26. Distributed Compute — "distributed-compute.g4"

"distributed-compute.g4" owns networking-facing distributed-computation intent.

It must not replace:

grammar/distributed/

The separation is:

networking/distributed-compute.g4
    =
communication-oriented distributed intent

grammar/distributed/
    =
general distributed-computing semantics

Networking may describe how computation communicates.

Distributed computing determines the broader semantics of:

- distributed execution;
- replication;
- consistency;
- distributed state;
- distributed fault tolerance;
- node membership;
- distributed scheduling;
- partitioning.

---

27. Network Capabilities — "network-capabilities.g4"

Capabilities describe what the target must be able to do.

Examples include conceptual capabilities such as:

network.connectivity
network.streaming
network.reliable
network.bidirectional
network.multicast
network.broadcast
network.secure
network.authenticated
network.confidential
network.integrity
network.low_latency
network.high_bandwidth
network.locality

These are capability identifiers, not physical-device selections.

The correct model is:

requires capability("network.streaming")

rather than:

use_network_device_7

Capability satisfaction is performed downstream.

---

28. Requirements, Constraints, Preferences and Hints

Networking must preserve the distinction between:

requirement
constraint
capability
preference
hint
resource
target
placement

For example:

requires capability("network.reliable")

is not the same as:

prefer low_latency

and neither is the same as:

place communication near data

and none is equivalent to:

use interface eth0

The first is a capability requirement.

The second is a preference.

The third is a placement constraint/policy.

The fourth is a target-specific implementation decision.

The semantic system must preserve those distinctions.

---

29. Resource Integration

Networking integrates with the repository-wide resource model.

Potential resource requirements include:

- bandwidth;
- latency;
- availability;
- reliability;
- communication locality;
- throughput;
- energy;
- cost;
- storage;
- connection capacity;
- service capacity.

The grammar must express requirements without establishing universal hardware ceilings.

For example:

requires bandwidth >= required_bandwidth

may be valid semantic intent.

But:

MAX_BANDWIDTH = 1Tbps

must not become a universal grammar limit.

---

30. Hardware Integration

Networking may eventually be realized through:

CPU
GPU
FPGA
ASIC
QPU
NIC
accelerator
shared memory
interconnect
network fabric
future substrate

The networking grammar does not own those physical resources.

The integration is:

networking intent
    ↓
resource requirements
    ↓
hardware capability discovery
    ↓
placement
    ↓
routing
    ↓
scheduling
    ↓
HAL
    ↓
target realization

No physical topology is hard-coded into the grammar.

---

31. Classical Computing Integration

Networking constructs must work with ordinary Zamani:

- functions;
- variables;
- types;
- expressions;
- modules;
- effects;
- concurrency;
- memory;
- data structures.

Networking should not require a separate classical language.

---

32. Quantum Integration

Networking must support future and current:

- quantum networking;
- distributed quantum computing;
- remote quantum services;
- quantum-classical communication;
- distributed QEC workflows;
- hybrid computation.

However, networking grammar must not own:

- qubits;
- gates;
- states;
- measurement;
- QEC codes;
- calibration;
- pulses;
- physical qubits;
- quantum topology;
- ZQN semantics.

Those remain under the quantum architecture.

The canonical path remains:

Zamani source
    ↓
frontend AST
    ↓
semantic quantum representation
    ↓
quantum::ir
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
QEC / resilience
    ↓
ZQN
    ↓
HAL
    ↓
QPU

Networking metadata may accompany the semantic representation where necessary.

Networking MUST NOT introduce a second quantum IR.

---

33. Hybrid Computing Integration

Networking must support hybrid:

classical computation
        ↓
network communication
        ↓
quantum service
        ↓
measurement/result
        ↓
network communication
        ↓
classical computation

The grammar expresses the communication contract.

Quantum semantics remain quantum-owned.

Classical semantics remain classical-owned.

The semantic layer connects them.

---

34. HDL Integration

Networking can participate in hardware/software co-design.

For example, hardware may expose a communication interface.

However, networking must not redefine:

- wires;
- clocks;
- registers;
- timing;
- state machines;
- hardware modules;
- synthesis;
- physical implementation.

Those remain owned by:

grammar/hdl/
grammar/hardware/

Networking describes the communication semantics.

HDL describes the hardware realization.

---

35. Concurrency Integration

Networking and concurrency interact frequently.

However:

networking channel

and:

concurrency channel

remain different language concepts.

Networking owns:

communication across networking boundaries

Concurrency owns:

task synchronization and language-level concurrent communication

Semantic analysis may connect the two.

The grammar must not duplicate or merge their ownership.

---

36. Data Integration

Networking messages may use:

- primitive types;
- records;
- structs;
- tuples;
- arrays;
- tensors;
- streams;
- schemas;
- generic types;
- domain data.

But networking does not own the data model.

The dependency direction is:

networking
    ↓
message contract
    ↓
canonical type/data system
    ↓
serialization semantics
    ↓
wire representation

Networking must not duplicate "grammar/data/".

---

37. Security Integration

Networking may express security requirements.

For example, a communication operation may semantically require:

authenticated communication
confidential communication
integrity-protected communication

But networking does not implement:

- cryptography;
- key management;
- identity;
- authorization;
- certificate validation;
- trust systems;
- secrets.

Those remain owned by:

grammar/security/
grammar/effects/
semantic security analysis
runtime security

This prevents networking grammar from becoming coupled to today's cryptographic algorithms.

---

38. AI and Data-Centric Computing

Networking must support communication involving:

- AI agents;
- model services;
- distributed training;
- inference services;
- distributed datasets;
- accelerators;
- tensor pipelines;
- data streams.

But framework names must not become core grammar keywords.

Framework-specific networking belongs to:

- libraries;
- modules;
- dialects;
- capabilities;
- interoperability layers.

---

39. Open-World Protocol Model

Zamani networking MUST be open-world.

The grammar must remain valid when the future introduces:

- a new transport;
- a new network architecture;
- a new service-discovery system;
- a new interconnect;
- a new accelerator fabric;
- a new quantum network;
- a new distributed protocol;
- a new communication substrate.

A new technology should normally be represented through:

name
type
capability
module
dialect
library
semantic registry
target description

rather than requiring a new global parser keyword.

---

40. Vendor Neutrality

The networking grammar must not be permanently coupled to:

- AWS;
- Azure;
- GCP;
- cloud-specific APIs;
- a particular NIC vendor;
- a particular switch vendor;
- a particular QPU vendor;
- a particular GPU vendor;
- a particular supercomputer;
- a particular FPGA vendor;
- a particular network fabric.

Provider-specific behavior belongs downstream.

The portable source should express:

requires capability("network.reliable")

rather than:

use_provider("specific_vendor")

unless provider identity is intentionally part of the program's explicit semantics.

---

41. Address and Identifier Portability

Networking identifiers should normally use the canonical Zamani name system.

Do not introduce parallel identifier systems for:

- endpoints;
- services;
- channels;
- protocols;
- sockets;
- routes.

The preferred relationship is:

canonical identifier
        ↓
qualified name
        ↓
semantic resolution

rather than each networking component inventing a separate namespace.

---

42. AST Contract

Every networking grammar construct MUST have a predetermined AST mapping before the grammar construct is considered complete.

The required direction is:

grammar rule
    ↓
parse-tree context
    ↓
domain-neutral AST node
    ↓
networking semantic model
    ↓
canonical IR representation

The grammar must not require a future redesign of the AST.

Each AST representation must preserve, where applicable:

- source span;
- construct kind;
- source ordering;
- identifier/name;
- qualified name;
- attributes;
- modifiers;
- expressions;
- types;
- nested constructs;
- requirements;
- constraints;
- capabilities;
- preferences;
- metadata.

The AST must remain domain-neutral in:

src/frontend/ast/

It must not contain:

- physical NIC objects;
- router implementations;
- vendor transport classes;
- QPU topology objects;
- runtime socket handles.

---

43. Semantic Contract

Semantic analysis owns:

- name resolution;
- endpoint validity;
- address validity;
- service resolution;
- protocol compatibility;
- message/type compatibility;
- request/response compatibility;
- route validity;
- socket compatibility;
- stream compatibility;
- capability satisfaction;
- resource analysis;
- security requirements;
- distributed compatibility;
- portability validation;
- target-independent consistency.

The parser does not perform these operations.

---

44. IR Contract

Networking grammar must never create IR directly.

The required direction is:

Networking parse tree
        ↓
Frontend AST
        ↓
Networking semantic model
        ↓
Canonical semantic representation
        ↓
IR

Networking must integrate with the repository's existing canonical IR architecture.

If a dedicated networking IR is eventually required, it must be established as a canonical semantic/IR contract rather than being invented independently inside the grammar directory.

It must not duplicate:

- canonical semantic IR;
- classical IR;
- "quantum::ir";
- HDL/hardware IR.

---

45. Routing Integration

Routing occurs after semantic analysis.

source routing intent
        ↓
semantic route requirements
        ↓
available topology/capabilities
        ↓
route planning
        ↓
optimization
        ↓
target realization

The grammar does not choose a physical route.

---

46. Scheduling Integration

Communication scheduling belongs downstream.

Networking syntax may express requirements such as:

- ordering;
- latency;
- synchronization;
- deadline;
- throughput;
- priority;
- streaming semantics.

Scheduling determines how those requirements are realized.

The grammar must not hard-code:

run communication on thread 7
use NIC 2
send at physical cycle 42

unless such information is intentionally expressed as a target-specific program requirement.

---

47. Resilience Integration

Networking may express semantic requirements such as:

reliable
fault_tolerant
retryable
recoverable

but the runtime/semantic resilience system determines how they are realized.

The networking grammar must not hard-code one recovery algorithm.

---

48. Failure-State Integration

Where the broader Zamani resilience architecture exposes states such as:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

networking grammar may reference the semantic model where appropriate.

It must not implement the state machine.

Likewise, outcomes such as:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

belong to semantic/runtime resilience contracts, not parser implementation.

---

49. Interoperability

Networking must remain interoperable with external technologies without making those technologies part of the core grammar.

Potential interoperability targets include:

- OS networking;
- sockets;
- IPC;
- shared memory;
- RPC;
- HTTP;
- QUIC;
- MPI;
- RDMA;
- message buses;
- cloud networking;
- accelerator fabrics;
- quantum networks;
- future communication substrates.

These are realizations or interoperability formats.

They are not necessarily Zamani language primitives.

---

50. Interoperability Boundary

The correct architecture is:

Zamani networking semantics
        ↓
interoperability layer
        ↓
external protocol / ABI / API
        ↓
runtime

not:

Zamani grammar
        ↓
vendor protocol implementation

This is essential for POCO-REAF.

---

51. Diagnostics Contract

Networking diagnostics must preserve source spans.

Errors should identify:

- source location;
- networking construct;
- offending name;
- relevant semantic relationship;
- expected form;
- actual form;
- applicable language version;
- applicable dialect;
- capability/resource mismatch where relevant.

Examples of semantic diagnostics include:

unknown endpoint
unknown service
incompatible message type
protocol incompatible with channel
response does not satisfy request contract
required capability unavailable
route violates declared constraint
unsupported address family
ambiguous service reference

The parser should not report runtime conditions as parse errors.

For example:

network unavailable

is not inherently a syntax error.

---

52. Determinism

The networking grammar must be deterministic with respect to the same:

- source text;
- lexer version;
- grammar version;
- grammar configuration;
- dialect configuration.

The grammar must contain:

- no runtime calls;
- no network calls;
- no filesystem access;
- no hardware discovery;
- no environment inspection;
- no randomness;
- no semantic predicates dependent on external state;
- no embedded unsafe code.

The same valid input must produce the same parse structure under the same language configuration.

---

53. Safe Rust Requirement

The grammar itself contains no Rust implementation.

Any Rust implementation connected to networking must comply with:

Rust 1.97 / Rust 1.97.1
Rust 2021
safe Rust only
no unsafe

The networking grammar must never require "unsafe" to implement its semantics.

OS/network integration may require platform-specific APIs downstream, but the Zamani production implementation must preserve the repository's explicit no-"unsafe" policy.

Where an external API would ordinarily require unsafe FFI, that responsibility belongs to a separately reviewed interoperability boundary and must not leak unsafe requirements into the grammar architecture.

---

54. Security of the Grammar

The grammar must never:

- access secrets;
- access credentials;
- inspect network interfaces;
- inspect environment variables;
- make network calls;
- perform DNS;
- access files;
- execute commands;
- contact cloud APIs;
- probe hardware.

Parsing is pure source processing.

---

55. Performance and Unboundedness

"Unbounded" means:

«no artificial language-level maximum.»

It does not mean:

«infinite memory or infinite execution time.»

ANTLR, Rust, the host operating system and the target machine necessarily have finite implementation resources.

Therefore:

language scalability

and:

implementation capacity

must remain separate concepts.

A compiler may reject a program because the implementation cannot allocate enough memory.

That is not equivalent to the language defining a permanent networking limit.

---

56. Large-System Scaling

The networking grammar must support source programs representing:

tiny embedded communication
        ↓
single-machine communication
        ↓
multi-process communication
        ↓
multi-device communication
        ↓
cluster communication
        ↓
HPC communication
        ↓
cloud communication
        ↓
edge communication
        ↓
planet-scale distributed computation
        ↓
future computational networks

The grammar does not need a different language at each scale.

Scale is determined by:

program semantics
+
resource requirements
+
available capabilities
+
compiler strategy
+
runtime strategy
+
deployment

---

57. Quantum Networking

Quantum networking must be representable without making networking grammar own quantum semantics.

Potential semantic concepts include:

quantum endpoint
quantum communication capability
remote quantum service
distributed quantum operation
entanglement communication requirement
quantum-classical communication boundary

But the networking grammar must not define:

physical qubit
quantum gate
QEC code
pulse
calibration
physical topology

Those belong to the quantum architecture.

---

58. Distributed Quantum Integration

A distributed quantum program may conceptually be:

classical orchestration
        ↓
quantum operation
        ↓
network communication
        ↓
remote quantum resource
        ↓
measurement/result
        ↓
classical decision
        ↓
network communication
        ↓
quantum operation

The networking grammar supplies the communication contract.

The quantum system supplies quantum semantics.

The distributed system supplies placement and distributed execution.

The runtime supplies realization.

---

59. HDL / Network Co-Design

Hardware may expose a network-facing interface.

The architecture must permit:

software algorithm
        +
network communication intent
        +
hardware interface intent
        ↓
co-designed realization

but ownership remains separate:

networking/
    communication semantics

hdl/
    hardware description

hardware/
    hardware capabilities and intent

---

60. Effects

Networking operations may have effects.

Examples may include conceptual effects for:

communication
I/O
remote execution
distributed state
security-sensitive communication
resource acquisition

The networking grammar should reference the canonical effect architecture.

It must not create a second effect system.

---

61. Capabilities

Networking capabilities must integrate with the universal capability system.

Examples:

capability("network.connectivity")
capability("network.streaming")
capability("network.reliable")
capability("network.multicast")
capability("network.secure")
capability("network.authenticated")

Capability names remain open-world.

A future capability must not require a new global grammar rule solely because its identifier is new.

---

62. Resource Requirements

Networking resource requirements may include expressions involving:

bandwidth
latency
throughput
availability
reliability
locality
energy
cost
capacity

The grammar must allow these to remain expressions/contracts rather than embedding machine constants.

---

63. Topology

Topology is an important networking semantic concept but must remain abstract.

A program may require:

requires topology(...)

or equivalent semantic constructs.

It must not universally require:

router 0
router 1
router 2
switch 4
node 7
link 11

unless those identities are explicitly part of the program's intended physical target.

Topology realization belongs downstream.

---

64. Placement

Placement is distinct from networking.

Networking can express constraints relevant to placement.

Placement determines where computation and communication are realized.

Therefore:

networking
    ↓
requirements
    ↓
placement

not:

networking grammar
    ↓
physical placement

---

65. Service Discovery and Placement

Service discovery can resolve a logical service to an available realization.

The architecture is:

service reference
    ↓
semantic service contract
    ↓
discovery requirement
    ↓
service discovery runtime
    ↓
available service instance

The parser does not perform discovery.

---

66. Versioning

Networking syntax must participate in Zamani's language-version system.

Every public networking construct must have:

- introduction version;
- stability status;
- compatibility status;
- deprecation status where applicable;
- migration guidance if syntax changes.

Internal helper grammar rules may evolve without changing the public source language, provided the accepted language and parse contracts remain compatible.

---

67. Feature Lifecycle

Networking features follow:

historical
    ↓
proposed
    ↓
experimental
    ↓
specified
    ↓
implemented
    ↓
conformance-tested
    ↓
stable

A feature in "Zamani-Grammar.md" does not automatically become legal Zamani syntax.

The promotion path is:

Zamani-Grammar.md
        ↓
feature proposal
        ↓
semantic design
        ↓
AST contract
        ↓
grammar contract
        ↓
implementation
        ↓
IR contract
        ↓
tests
        ↓
compatibility validation
        ↓
stable

---

68. Generated Documentation

"grammar/grammar.md" remains the implementation-conformance reference.

This networking README does not replace it.

Networking implementation status should ultimately be represented as:

SPECIFIED
IMPLEMENTED
PARTIALLY_IMPLEMENTED
EXPERIMENTAL
PLANNED
DEPRECATED

No networking feature should be called production-ready merely because its ".g4" file exists.

---

69. Grammar-to-AST Traceability

Every public networking rule must map to a documented AST contract.

Minimum traceability:

Grammar boundary| AST responsibility| Semantic responsibility
"networkingAddress"| address syntax node| address resolution
"networkingEndpoint"| endpoint node| endpoint resolution
"networkingChannel"| channel node| channel validation
"networkingMessage"| message node| message/type validation
"networkingProtocol"| protocol node| protocol compatibility
"networkingRequest"| request node| request validation
"networkingResponse"| response node| response validation
"networkingRoute"| route-intent node| route validation
"networkingServiceDiscovery"| discovery node| discovery semantics
"networkingService"| service node| service resolution
"networkingSocket"| socket node| socket compatibility
"networkingStream"| stream node| stream validation
"networkingDistributedCompute"| distributed-compute node| distributed semantic validation
"networkingCapability"| capability requirement node| capability satisfaction

These mappings must be established before declaring the corresponding feature complete.

---

70. Semantic-to-IR Traceability

Each networking semantic construct must have a predetermined downstream mapping.

Conceptually:

address
    ↓
logical address semantic value
    ↓
target-independent communication representation

endpoint
    ↓
logical endpoint
    ↓
communication resource/reference

channel
    ↓
communication relationship
    ↓
IR communication operation

message
    ↓
typed message
    ↓
communication payload representation

protocol
    ↓
protocol contract
    ↓
transport/lowering requirement

service
    ↓
service contract
    ↓
service invocation/exposure representation

route
    ↓
route intent
    ↓
routing constraints

stream
    ↓
stream semantic model
    ↓
streaming communication operations

capability
    ↓
capability requirement
    ↓
resource/capability analysis

The exact canonical IR node names belong to the IR specification and implementation.

The grammar must not invent them.

---

71. No Second Networking IR

The networking directory must not introduce an independent IR merely because networking is complex.

If a networking semantic representation is necessary, it must be part of the repository's canonical semantic/IR architecture.

The rule is:

one semantic architecture
one canonical IR boundary
specialized domain lowering where required

not:

network grammar
    ↓
network IR

quantum grammar
    ↓
quantum IR A

quantum compiler
    ↓
quantum IR B

This principle is particularly important for preserving the canonical:

quantum::ir

boundary.

---

72. Interoperability Formats

Formats such as:

- HTTP;
- QUIC;
- QIR;
- OpenQASM;
- MPI;
- RDMA;
- WASM;
- foreign APIs;

must be treated as interoperability or target formats where appropriate.

They are not automatically the canonical Zamani semantic model.

The networking grammar remains the source-language contract.

---

73. Testing Contract

Networking testing must cover all of:

positive
negative
boundary
scalability
determinism
compatibility
diagnostics
integration
portability
hard-coding

Tests must exist at multiple levels:

lexer
parser
AST
semantic
IR
compiler
runtime
interoperability

---

74. Positive Tests

Positive tests must cover:

- one endpoint;
- multiple endpoints;
- addresses;
- channels;
- messages;
- protocols;
- requests;
- responses;
- routes;
- services;
- discovery;
- sockets;
- streams;
- distributed compute;
- capabilities;
- nested constructs;
- qualified names;
- generic types;
- attributes;
- expressions.

---

75. Negative Tests

Negative tests must include:

- malformed endpoint;
- invalid address syntax;
- malformed message;
- invalid request;
- invalid response;
- malformed service;
- invalid route;
- invalid socket;
- malformed stream;
- invalid capability expression;
- incompatible request/response;
- invalid type relationship;
- malformed qualified name.

Semantic failures must be distinguished from syntax failures.

---

76. Boundary Tests

Boundary tests must include:

- empty networking unit;
- one construct;
- many constructs;
- deeply nested declarations;
- long qualified names;
- large message definitions;
- large service definitions;
- large channel sets;
- large route sets;
- large capability sets;
- nested networking structures.

No boundary test should establish an artificial maximum.

---

77. Scalability Tests

Scalability tests must verify that the language architecture does not impose artificial limits.

The test suite should exercise generated source with increasing:

endpoint count
message count
service count
channel count
request count
response count
route count
stream count
capability count
distributed-compute constructs

The test objective is not to prove mathematical infinity.

The objective is to prove:

«no artificial language-level ceiling has been encoded into the grammar.»

---

78. Hard-Coding Audit

The networking grammar must be audited for forbidden artificial limits including:

MAX_ENDPOINTS
MAX_ADDRESSES
MAX_CHANNELS
MAX_MESSAGES
MAX_PROTOCOLS
MAX_REQUESTS
MAX_RESPONSES
MAX_ROUTES
MAX_SERVICES
MAX_SOCKETS
MAX_STREAMS
MAX_CONNECTIONS
MAX_NODES
MAX_NETWORK_SIZE
MAX_BANDWIDTH
MAX_THREADS
MAX_DEVICES

The audit must also detect hard-coded physical identities such as:

NIC0
NIC1
ROUTER0
ROUTER1
NODE0
NODE1
PORT0
PORT1
DEVICE0
DEVICE1

when they are incorrectly used as universal language constructs.

Legitimate user-program identifiers containing numbers are not prohibited.

The audit is specifically about compiler- or grammar-imposed limits.

---

79. Determinism Tests

The same source under the same language version and dialect configuration must produce the same parse result.

Tests should verify:

source
    ↓
lexer
    ↓
parser

is deterministic.

Networking grammar must not depend on:

- current network state;
- machine identity;
- operating system;
- environment variables;
- clock;
- random values;
- available devices.

---

80. Compatibility Tests

Networking compatibility tests must compare:

specification
    ↓
networking.g4
    ↓
lexer
    ↓
parser
    ↓
AST
    ↓
semantic model
    ↓
IR

The test suite should detect:

- specified-but-unimplemented constructs;
- implemented-but-unspecified constructs;
- parser-only constructs;
- AST gaps;
- semantic gaps;
- IR gaps;
- incompatible changes.

---

81. Portability Tests

A networking program that uses only portable semantics should not become target-specific merely because it is compiled for:

embedded
CPU
multicore
GPU
FPGA
ASIC
QPU
HPC
cluster
cloud
edge
future target

Where a target cannot satisfy requirements, compilation/runtime should report a resource/capability incompatibility, not reinterpret the source program.

---

82. Requirement vs Availability

This distinction is fundamental:

program requirement
        ≠
currently available resource

For example:

requires capability("network.streaming")

means the program requires that capability.

Whether the selected target provides it is determined later.

The grammar must not silently lower a requirement into a specific device.

---

83. Resource Failure Semantics

If a target cannot satisfy:

required capability
required bandwidth
required latency
required reliability
required topology

the appropriate downstream layer must report the incompatibility.

The grammar must not:

- silently reduce requirements;
- silently select a different protocol;
- silently alter semantics;
- silently lower correctness guarantees.

Any permitted approximation must be explicitly defined by the language semantics.

---

84. Future-Proofing

The networking architecture must remain usable when future systems introduce:

- new network architectures;
- optical networks;
- quantum networks;
- neuromorphic interconnects;
- molecular/biological communication;
- nanoscale communication;
- photonic communication;
- satellite networks;
- interplanetary networks;
- new accelerator fabrics;
- new distributed substrates;
- communication mechanisms not yet invented.

The source-language architecture should remain stable.

Only semantic registries, dialects, interoperability layers, capabilities or target backends should normally need expansion.

---

85. Nano Computing

Where Zamani eventually supports nano-scale communication, networking should provide communication semantics without embedding a fixed physical implementation.

Nano networking may interact with:

grammar/nano/
grammar/hardware/
grammar/resources/

but networking must not encode a fixed molecular, atomic or device topology.

---

86. Temporal / MTS Integration

Networking may interact with Zamani's temporal or Multi-Timeline System concepts.

However:

networking

does not own temporal semantics.

MTS remains responsible for:

- timeline semantics;
- temporal state;
- branching;
- observation;
- merge;
- rewind;
- causal semantics.

Networking can carry temporal metadata where specified.

It must not duplicate MTS.

---

87. Sankofa Integration

Networking may transport or expose Sankofa-related data.

However, networking does not implement:

- Sankofa memory;
- recall;
- learning;
- wisdom;
- historical state;
- temporal knowledge.

Those belong to their respective semantic/runtime systems.

Networking only provides communication semantics.

---

88. Macros and Metaprogramming

Macros and metaprogramming may generate networking constructs.

Generated networking syntax must still pass through:

normal parsing
normal AST construction
normal semantic analysis
normal capability/resource analysis
normal IR lowering

Macros must not bypass networking semantic validation.

Metaprogramming must not create a hidden networking runtime.

---

89. Dialects

Networking dialects are allowed where the standard language cannot reasonably encode a domain-specific extension.

A dialect must identify:

name
version
owner
syntax extensions
semantic extensions
AST mapping
IR mapping
capabilities
compatibility
feature gates

A dialect must not silently redefine standard networking semantics.

A dialect must not become an accidental second networking language.

---

90. Provider-Specific Extensions

Provider-specific networking can exist through dialects or interoperability layers.

For example:

vendor::feature
cloud::feature
fabric::feature

may be represented as semantic extensions.

The standard networking grammar must not hard-code the vendor into the universal language.

---

91. Public Grammar Stability

The following aggregate boundaries are public integration contracts:

networkingUnit
networkingConstruct
networkingDeclaration
networkingElement
universalNetworking

Leaf public rules should likewise be treated as stable only after their component contract is complete.

Internal helper rules may change if source compatibility and integration contracts remain intact.

---

92. Source Compatibility

A networking feature change must classify whether it affects:

lexical compatibility
syntax compatibility
AST compatibility
semantic compatibility
IR compatibility
runtime compatibility

A purely internal grammar refactoring should not require source migration.

A public syntax change requires a compatibility assessment.

---

93. Deprecation

Deprecated networking syntax must remain identifiable.

Deprecation should include:

feature name
introduced version
deprecated version
replacement
migration strategy
removal policy
compatibility impact

Do not silently remove networking syntax.

---

94. Error Recovery

ANTLR error recovery must remain compatible with the wider Zamani diagnostic architecture.

The networking grammar should provide enough structural boundaries to produce useful diagnostics without swallowing unrelated declarations.

Recovery must not introduce semantic behavior.

---

95. Source Spans

Every networking construct must preserve accurate source locations.

At minimum, downstream AST nodes should be able to identify:

start position
end position
construct kind
relevant identifier/property

This supports:

- compiler diagnostics;
- IDE integration;
- source navigation;
- semantic highlighting;
- refactoring;
- generated documentation;
- conformance testing.

---

96. Tooling Contract

Networking grammar should support tooling such as:

- syntax highlighting;
- completion;
- navigation;
- symbol lookup;
- documentation;
- diagnostics;
- semantic visualization;
- refactoring.

Tooling must not infer physical deployment merely from grammar syntax.

For example:

socket foo

must not cause an IDE to claim that "foo" is physically bound to a particular OS socket.

That is semantic/runtime information.

---

97. Documentation Contract

Each networking ".g4" file should document:

Purpose
Status
Owns
Does not own
Inputs
Outputs
Dependencies
Public rules
AST contract
Semantic contract
IR contract
Compiler integration
Runtime integration
Tooling integration
Positive tests
Negative tests
Boundary tests
Scalability tests
Compatibility tests
Determinism
Security
Hard-coding audit
Completion criteria

This satisfies the repository-wide requirement that a file can be completed independently without discovering missing architecture later.

---

98. Independent-First Completion Rule

A networking component is not complete merely because its grammar parses.

For every component:

syntax
✓
lexical dependencies
✓
public rules
✓
AST mapping
✓
semantic mapping
✓
IR mapping
✓
diagnostics
✓
source spans
✓
security boundary
✓
resource boundary
✓
capability boundary
✓
portability
✓
positive tests
✓
negative tests
✓
boundary tests
✓
scalability tests
✓
compatibility tests
✓
determinism
✓
hard-coding audit
✓
integration contract
✓
completion criteria

Only then is that component independently complete.

---

99. Integration Details Must Be Declared Before Implementation

Every networking component must declare its downstream consumers before implementation begins.

For example:

addresses.g4
    ↓
address AST
    ↓
address semantic model
    ↓
resource/capability analysis
    ↓
routing/discovery/runtime

and:

messages.g4
    ↓
message AST
    ↓
type/data semantic model
    ↓
serialization contract
    ↓
IR
    ↓
runtime

and:

services.g4
    ↓
service AST
    ↓
service semantic model
    ↓
discovery/placement
    ↓
runtime

This prevents the "write grammar now, decide AST later" problem.

---

100. Completion Contract for "networking.g4"

"networking.g4" is complete only when:

- every existing networking component is imported;
- every component is imported exactly once;
- no component is duplicated;
- all public adapters are stable;
- the wider parser can consume networking through one boundary;
- no import cycle exists;
- all leaf grammars compile;
- all leaf tests pass;
- aggregate tests pass;
- "EOF" is enforced for standalone parsing;
- no physical resource limit is encoded;
- no vendor transport is hard-coded;
- no networking runtime behavior exists in the grammar;
- AST mappings are defined;
- semantic mappings are defined;
- IR mappings are defined;
- source spans are preserved;
- diagnostics are defined;
- compatibility is defined.

---

101. Completion Contract for "endpoints.g4"

Complete only when:

- logical endpoint syntax is defined;
- names use canonical Zamani naming;
- endpoint references are defined;
- attributes are defined;
- endpoint multiplicity is unbounded at language level;
- no physical endpoint allocation exists;
- AST mapping exists;
- semantic validation exists;
- resource/capability integration exists;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- portability tests exist.

---

102. Completion Contract for "addresses.g4"

Complete only when:

- logical address syntax is defined;
- qualified names are supported where applicable;
- address properties are semantically extensible;
- no fixed address-family universe is required;
- no physical interface is selected by the grammar;
- AST mapping exists;
- semantic address resolution exists;
- interoperability mapping exists;
- diagnostics exist;
- compatibility tests exist.

---

103. Completion Contract for "channels.g4"

Complete only when:

- networking channels are structurally distinct from concurrency channels;
- channel identity is defined;
- direction semantics are defined;
- participant semantics are defined;
- channel properties are defined;
- AST mapping exists;
- semantic mapping exists;
- IR mapping exists;
- no physical link is encoded;
- no channel count limit exists;
- scalability tests exist.

---

104. Completion Contract for "messages.g4"

Complete only when:

- message declarations are defined;
- message fields use canonical types;
- message metadata is defined;
- generic message types are supported where applicable;
- serialization remains downstream;
- no message-size ceiling exists in grammar;
- AST mapping exists;
- type/semantic validation exists;
- serialization integration is defined;
- interoperability integration is defined;
- tests exist.

---

105. Completion Contract for "protocols.g4"

Complete only when:

- protocol declarations are defined;
- protocol references are open-world;
- protocol properties are extensible;
- no closed protocol enumeration exists;
- protocol compatibility is semantic;
- AST mapping exists;
- interoperability mapping exists;
- capability mapping exists;
- future protocol compatibility is tested.

---

106. Completion Contract for "requests.g4"

Complete only when:

- request syntax is defined;
- target service semantics are defined;
- parameters are defined;
- requirements are defined;
- response relationships are defined;
- AST mapping exists;
- semantic validation exists;
- IR integration exists;
- no transport implementation is embedded;
- diagnostics exist;
- tests exist.

---

107. Completion Contract for "responses.g4"

Complete only when:

- response syntax is defined;
- result semantics are defined;
- response/error semantics are defined;
- request compatibility is defined;
- AST mapping exists;
- semantic mapping exists;
- IR integration exists;
- transport-independent semantics are preserved.

---

108. Completion Contract for "routing.g4"

Complete only when:

- routing intent is defined;
- constraints are defined;
- preferences are defined;
- topology references remain abstract;
- physical route selection remains downstream;
- AST mapping exists;
- routing semantic mapping exists;
- scheduler integration is defined;
- scalability tests exist.

---

109. Completion Contract for "service-discovery.g4"

Complete only when:

- service-discovery intent is defined;
- service references are canonical;
- discovery requirements are represented;
- no specific discovery provider is mandatory;
- AST mapping exists;
- semantic discovery contract exists;
- runtime integration exists;
- future discovery mechanisms can be added without core grammar churn.

---

110. Completion Contract for "services.g4"

Complete only when:

- service declarations are defined;
- operations are defined;
- parameters/types are defined;
- service contracts are defined;
- service references are defined;
- deployment is separate;
- discovery is separate;
- AST mapping exists;
- semantic mapping exists;
- IR mapping exists;
- runtime integration exists.

---

111. Completion Contract for "sockets.g4"

Complete only when:

- logical socket syntax exists;
- socket references exist;
- properties are open-world;
- physical socket allocation is absent;
- OS implementation is downstream;
- transport selection is downstream;
- AST mapping exists;
- semantic compatibility exists;
- runtime integration exists;
- no socket-count ceiling exists.

---

112. Completion Contract for "streaming.g4"

Complete only when:

- stream declaration is defined;
- stream direction is defined;
- stream lifecycle is defined;
- stream element typing is defined;
- asynchronous semantics are defined;
- backpressure/flow requirements are semantically represented where required;
- runtime stream implementation remains downstream;
- no universal stream-size/count limit exists.

---

113. Completion Contract for "distributed-compute.g4"

Complete only when:

- networking-facing distributed computation syntax exists;
- general distributed semantics remain in "grammar/distributed/";
- node identity is abstract;
- placement is separate;
- replication is separate;
- scheduling is separate;
- AST mapping exists;
- distributed semantic mapping exists;
- networking integration exists;
- no fixed node count exists.

---

114. Completion Contract for "network-capabilities.g4"

Complete only when:

- capability requirements are expressible;
- capability identifiers are open-world;
- capability satisfaction remains downstream;
- requirements can reference expressions;
- resource requirements remain distinct;
- target-specific device selection remains downstream;
- AST mapping exists;
- semantic capability mapping exists;
- portability tests exist.

---

115. Production Validation Pipeline

The networking production gate is:

Normative specification
        ↓
Networking grammar contract
        ↓
ANTLR grammar validation
        ↓
Lexer conformance
        ↓
Parser conformance
        ↓
AST coverage
        ↓
Semantic coverage
        ↓
IR coverage
        ↓
Compiler coverage
        ↓
Runtime integration
        ↓
Positive tests
        ↓
Negative tests
        ↓
Boundary tests
        ↓
Scalability tests
        ↓
Portability tests
        ↓
Compatibility tests
        ↓
Diagnostics tests
        ↓
Determinism tests
        ↓
Hard-coding audit
        ↓
Safe-Rust audit
        ↓
Production acceptance

---

116. Repository-Wide Integration Matrix

Networking integrates with the existing Zamani architecture as follows:

Subsystem| Networking relationship
"lexer/"| canonical tokens, identifiers, literals
"core/"| names, paths, attributes, modifiers
"types/"| message/service/channel types
"expressions/"| networking property values and requirements
"statements/"| networking-related operations where defined
"declarations/"| networking declarations
"modules/"| networking imports/exports
"effects/"| communication effects
"memory/"| buffers, ownership and communication memory
"concurrency/"| task/channel interaction
"classical/"| classical networking computation
"quantum/"| quantum networking metadata/interaction
"hybrid/"| classical/quantum communication
"hdl/"| hardware communication interfaces
"hardware/"| hardware capabilities and topology
"distributed/"| distributed execution
"resources/"| communication resource requirements
"execution/"| runtime execution policies
"compile/"| target selection and compilation
"data/"| message/data semantics
"security/"| authentication/confidentiality/integrity
"interoperability/"| external protocols and APIs
"dialects/"| domain/vendor extensions
"validation/"| grammar and architecture validation
"compatibility/"| version/migration policy
"reference/"| generated language documentation
"tests/"| complete conformance suite

---

117. What Networking Must Never Become

Networking must never become:

a second programming language

or:

a vendor-specific networking DSL

or:

a socket API embedded in the parser

or:

a physical topology description hidden inside source syntax

or:

a replacement for distributed computing

or:

a replacement for security

or:

a replacement for hardware description

or:

a second IR

or:

a second AST

---

118. What Networking Is

Networking is:

a first-class semantic domain of the Zamani language

with syntax for expressing:

communication participants
communication relationships
messages
protocols
requests
responses
services
discovery
routes
streams
sockets
distributed communication
capabilities
resource requirements

while leaving physical realization to downstream infrastructure.

---

119. POCO-REAF Example

A portable source program should be able to express the intent:

service compute {
    request Compute(input: Data) -> Result;
}

channel results;

requires capability("network.reliable");
requires capability("network.streaming");

without forcing:

CPU count
GPU count
NIC number
machine ID
router ID
switch ID
port number
cloud provider
physical topology

The compiler can subsequently determine a valid realization based on:

target
capabilities
resources
topology
deployment
runtime

---

120. Scaling Example

The same semantic program can conceptually move through:

one process
        ↓
two processes
        ↓
many processes
        ↓
one machine
        ↓
many machines
        ↓
cluster
        ↓
HPC system
        ↓
cloud
        ↓
edge
        ↓
heterogeneous accelerator system
        ↓
quantum/classical distributed system
        ↓
future computational network

without requiring a different networking language.

Only the realization changes.

---

121. Compiler Responsibility

The compiler must determine, where permitted:

- target capabilities;
- available communication mechanisms;
- placement;
- routing;
- scheduling;
- serialization;
- transport;
- interoperability;
- resource allocation;
- optimization.

It must preserve program semantics.

The compiler must not reinterpret:

requirement

as:

preference

or:

preference

as:

mandatory hardware choice

without an explicit language rule.

---

122. Runtime Responsibility

Runtime owns actual communication.

Runtime may:

- resolve endpoints;
- resolve services;
- allocate communication resources;
- select transports;
- establish connections;
- open sockets;
- use IPC;
- use shared memory;
- use specialized fabrics;
- negotiate capabilities;
- perform service discovery;
- recover communication;
- monitor communication;
- adapt to available resources.

None of those operations occur during grammar parsing.

---

123. Hardware / HAL Responsibility

HAL/hardware infrastructure owns:

- physical devices;
- interfaces;
- topology;
- device capabilities;
- hardware state;
- physical constraints;
- vendor implementation details;
- target-specific realization.

The networking grammar remains above that layer.

---

124. No Physical Assumptions

The grammar must never assume:

RAM = 64 GB
VRAM = 24 GB
register = 32 bit
NIC count = N
network nodes = N
port count = N

Likewise, it must not assume:

MAX_NETWORK_SIZE

or equivalent.

Physical availability is discovered and validated downstream.

---

125. Future Hardware

A networking source program should remain semantically meaningful if the target changes from:

CPU

to:

GPU

or:

FPGA

or:

ASIC

or:

QPU

or:

future accelerator

provided the target satisfies the program's semantic requirements.

---

126. Grammar Purity

The networking grammar must remain a pure syntactic layer.

It must not:

- allocate resources;
- inspect hardware;
- inspect network state;
- make network calls;
- perform DNS;
- open sockets;
- access credentials;
- execute programs;
- construct IR;
- schedule operations.

---

127. Production-Readiness Checklist

"grammar/networking/README.md" is complete when this architecture is accepted.

The networking directory is production-ready only when:

Architecture

- [x] one networking aggregate exists;
- [x] all existing networking components have explicit ownership;
- [x] no unnecessary file renames occur;
- [x] no competing networking root grammar exists;
- [x] dependency direction is defined;
- [x] root language composition remains authoritative.

Scalability

- [x] no artificial endpoint limit;
- [x] no artificial connection limit;
- [x] no artificial node limit;
- [x] no artificial message limit;
- [x] no artificial service limit;
- [x] no artificial stream limit;
- [x] no artificial bandwidth limit;
- [x] no artificial network-size limit.

Portability

- [x] physical topology is downstream;
- [x] provider selection is downstream;
- [x] transport implementation is downstream;
- [x] placement is downstream;
- [x] scheduling is downstream;
- [x] runtime realization is downstream.

Integration

- [x] AST boundary defined;
- [x] semantic boundary defined;
- [x] resource boundary defined;
- [x] capability boundary defined;
- [x] IR boundary defined;
- [x] routing boundary defined;
- [x] scheduling boundary defined;
- [x] runtime boundary defined;
- [x] HAL boundary defined.

Domain integration

- [x] classical;
- [x] quantum;
- [x] hybrid;
- [x] HDL;
- [x] hardware;
- [x] distributed;
- [x] AI;
- [x] data;
- [x] security;
- [x] concurrency;
- [x] interoperability;
- [x] dialects.

Safety

- [x] no grammar actions;
- [x] no runtime calls;
- [x] no network calls;
- [x] no hardware calls;
- [x] no environment access;
- [x] no unsafe Rust requirement.

Validation

- [x] positive tests defined;
- [x] negative tests defined;
- [x] boundary tests defined;
- [x] scalability tests defined;
- [x] portability tests defined;
- [x] determinism tests defined;
- [x] compatibility tests defined;
- [x] diagnostics tests defined;
- [x] hard-coding audit defined.

---

128. Definition of Done

A networking file is DONE only when its own contract can be closed without waiting for another networking file to be redesigned.

For every networking component:

Purpose
✓

Ownership
✓

Non-ownership
✓

Syntax
✓

Lexer dependencies
✓

Public rules
✓

AST mapping
✓

Semantic mapping
✓

IR mapping
✓

Compiler integration
✓

Runtime integration
✓

Resource integration
✓

Capability integration
✓

Security boundary
✓

Portability
✓

Source spans
✓

Diagnostics
✓

Positive tests
✓

Negative tests
✓

Boundary tests
✓

Scalability tests
✓

Compatibility tests
✓

Determinism
✓

Hard-coding audit
✓

Versioning
✓

Migration/deprecation
✓

Completion criteria
✓

A file that lacks any of these contracts is not yet production-complete.

---

129. Final Architectural Invariant

The fundamental invariant of Zamani networking is:

Zamani Source
     ↓
Portable Networking Intent
     ↓
Domain-Neutral AST
     ↓
Semantic Networking Model
     ↓
Canonical IR
     ↓
Resource + Capability Analysis
     ↓
Optimization
     ↓
Routing
     ↓
Placement
     ↓
Scheduling
     ↓
Deployment
     ↓
Runtime / HAL
     ↓
Actual Network

Never:

Zamani Source
     ↓
Physical Network

and never:

Networking Grammar
     ↓
Vendor Runtime

and never:

Networking Grammar
     ↓
Second IR

---

130. Final POCO-REAF Guarantee

The networking grammar exists to make the following architectural promise possible:

                 ONE ZAMANI PROGRAM
                         │
                         ▼
                ONE SOURCE SEMANTICS
                         │
                         ▼
                  ONE LANGUAGE MODEL
                         │
             ┌───────────┼───────────┐
             ▼           ▼           ▼
          Classical    Quantum      HDL
             │           │           │
             └───────────┼───────────┘
                         ▼
                 Hybrid / Distributed
                         │
                         ▼
                Resource + Capability
                         │
                         ▼
                  Canonical IR
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
       Routing        Scheduling      Resilience
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                        ZQN
                         │
                        HAL
                         │
       ┌─────────┬───────┼───────┬─────────┐
       ▼         ▼       ▼       ▼         ▼
      CPU       GPU     FPGA     QPU     Future
       │         │       │       │         │
       └─────────┴───────┴───────┴─────────┘
                         │
                         ▼
                 NETWORK REALIZATION

The networking grammar therefore describes communication meaning, not today's network hardware.

Its scalability is determined by the semantic model and actual resources rather than arbitrary parser limits.

Its portability is determined by separating intent from realization.

Its future-proofing is achieved through open-world names, capabilities, types, dialects and interoperability rather than continuously adding vendor/protocol keywords.

Its production readiness is achieved only when the complete chain:

Specification
→ Grammar
→ Lexer
→ Parser
→ AST
→ Semantics
→ Resources
→ Capabilities
→ IR
→ Routing
→ Scheduling
→ Runtime
→ Tests
→ Compatibility

is traceable.

That is the networking-domain contract required for Zamani to participate in the broader:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

architecture.