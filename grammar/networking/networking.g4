/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* FILE
* ---
* grammar/networking/networking.g4
* 
* GRAMMAR
* ---
* Networking
* 
* STATUS
* ---
* CANONICAL PRODUCTION NETWORKING COMPOSITION ROOT
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file is the SINGLE CANONICAL COMPOSITION / ORCHESTRATION ROOT for
* grammar/networking/.
* 
* It does not implement networking leaf syntax.
* 
* It composes the networking component grammars and exposes stable aggregate
* parser entry points to the canonical Zamani parser.
* 
* The networking component grammars remain independently owned:
* 
* addresses.g4
* endpoints.g4
* channels.g4
* messages.g4
* protocols.g4
* requests.g4
* responses.g4
* routing.g4
* service-discovery.g4
* services.g4
* sockets.g4
* streaming.g4
* distributed-compute.g4
* network-capabilities.g4
* policies.g4
* security.g4
* 
* This file therefore acts as the NETWORKING DOMAIN ORCHESTRATOR.
* 
* ============================================================================
* IMPLEMENTATION BASELINE
* ============================================================================
* 
* Language:
* Zamani
* 
* Grammar technology:
* ANTLR4
* 
* Rust implementation:
* Rust 1.97+
* Rust 2021
* Safe Rust only
* 
* Safety:
* No unsafe Rust.
* No embedded Rust.
* No parser actions.
* No semantic predicates.
* No filesystem access.
* No network access.
* No hardware access.
* No environment inspection.
* No runtime callbacks.
* No randomness.
* 
* ============================================================================
* ARCHITECTURAL ROLE
* ============================================================================
* 
* This file owns ONLY:
* 
* 1. networking-domain grammar composition;
* 2. networking component imports;
* 3. networking-domain dispatch;
* 4. stable networking parser entry points;
* 5. aggregate-level networking classification;
* 6. integration between grammar/networking/ and ZamaniParser.g4.
* 
* This file does NOT own:
* 
* - lexical definitions;
* - identifiers;
* - qualified names;
* - general expressions;
* - general types;
* - attributes;
* - contracts;
* - policies;
* - resources;
* - effects;
* - security implementation;
* - distributed execution implementation;
* - concurrency;
* - routing algorithms;
* - topology discovery;
* - transport implementation;
* - serialization;
* - cryptography;
* - hardware realization;
* - quantum operations;
* - classical IR;
* - quantum::ir;
* - HDL IR;
* - ZQN;
* - HAL;
* - runtime execution.
* 
* ============================================================================
* CANONICAL PIPELINE
* ============================================================================
* 
* Zamani source
*      |
*      v
* grammar/antlr/ZamaniLexer.g4
*      |
*      v
* grammar/antlr/ZamaniParser.g4
*      |
*      v
* Networking
*      |
*      +--> Addresses
*      +--> Endpoints
*      +--> Channels
*      +--> Messages
*      +--> Protocols
*      +--> Requests
*      +--> Responses
*      +--> Routes
*      +--> Service Discovery
*      +--> Services
*      +--> Sockets
*      +--> Streaming
*      +--> Distributed Compute
*      +--> Network Capabilities
*      +--> Networking Policies
*      +--> Networking Security
*      |
*      v
* domain-neutral frontend AST
*      |
*      v
* structural validation
*      |
*      +--> names
*      +--> types
*      +--> expressions
*      +--> effects
*      +--> capabilities
*      +--> resources
*      +--> contracts
*      +--> policies
*      +--> provenance
*      +--> security
*      +--> distributed semantics
*      +--> portability
*      |
*      v
* canonical semantic representation
*      |
*      +--> networking semantics
*      +--> distributed semantics
*      +--> classical semantics
*      +--> hybrid semantics
*      +--> hardware communication intent
*      +--> quantum communication metadata
*      |
*      v
* canonical/domain IR
*      |
*      +--> classical IR
*      +--> quantum::ir
*      +--> HDL/hardware representation
*      +--> distributed representation
*      |
*      v
* optimization
*      |
*      +--> routing
*      +--> placement
*      +--> scheduling
*      +--> resilience
*      +--> deployment
*      |
*      v
* ZQN
*      |
*      v
* HAL
*      |
*      v
* target realization
* 
* The grammar never constructs AST nodes, semantic objects, IR, runtime
* objects, sockets, routes, or hardware resources.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Networking syntax describes LOGICAL COMMUNICATION INTENT.
* 
* It must remain usable across:
* 
* tiny systems
* embedded systems
* single-process systems
* multicore systems
* GPU systems
* FPGA systems
* ASIC systems
* accelerator systems
* QPU systems
* simulators
* HPC systems
* clusters
* distributed systems
* cloud environments
* future computational substrates
* 
* subject to semantic feasibility and available resources.
* 
* The grammar MUST NOT impose universal limits on:
* 
* endpoints
* addresses
* channels
* messages
* protocols
* requests
* responses
* routes
* services
* sockets
* streams
* connections
* participants
* nodes
* devices
* links
* providers
* transports
* bandwidth
* latency
* topology size
* protocol roles
* protocol states
* protocol transitions
* flow steps
* requirements
* capabilities
* policies
* contracts
* metadata
* 
* There are deliberately NO universal constants such as:
* 
* MAX_ENDPOINTS
* MAX_ADDRESSES
* MAX_CHANNELS
* MAX_MESSAGES
* MAX_PROTOCOLS
* MAX_REQUESTS
* MAX_RESPONSES
* MAX_ROUTES
* MAX_SERVICES
* MAX_SOCKETS
* MAX_STREAMS
* MAX_CONNECTIONS
* MAX_NODES
* MAX_NETWORK_SIZE
* MAX_BANDWIDTH
* MAX_LATENCY
* MAX_MESSAGE_SIZE
* MAX_DEVICES
* 
* Repetition is expressed through normal ANTLR repetition operators and
* recursive structures.
* 
* Practical limits are implementation/resource limits, not language
* definitions.
* 
* ============================================================================
* OPEN-WORLD NETWORKING
* ============================================================================
* 
* This composition root deliberately does NOT enumerate a closed catalogue
* of:
* 
* TCP
* UDP
* QUIC
* HTTP
* MQTT
* gRPC
* MPI
* RDMA
* InfiniBand
* vendor transports
* cloud providers
* network vendors
* network devices
* routers
* switches
* NICs
* 
* Such identities remain names, expressions, capabilities, dialect concepts,
* policies, or semantic objects owned elsewhere.
* 
* Therefore a future communication technology MUST be representable without
* requiring this aggregate grammar to gain a new universal keyword merely
* because the technology is new.
* 
* ============================================================================
* TARGET INDEPENDENCE
* ============================================================================
* 
* A networking construct is a logical source-level construct.
* 
* It is not inherently:
* 
* a process
* a thread
* a machine
* a CPU
* a GPU
* an FPGA
* an ASIC
* an accelerator
* a QPU
* an operating-system socket
* a NIC
* an IP address
* a router
* a switch
* a cloud instance
* a physical node
* 
* Any suitable target may realize a logical networking construct when its
* capabilities, resources, policies and other semantic obligations permit.
* 
* ============================================================================
* SINGLE-AUTHORITY RULE
* ============================================================================
* 
* Every networking concept MUST have exactly one active source-level grammar
* owner.
* 
* This aggregate does not become an alternative owner.
* 
* Ownership is:
* 
* addresses
*     -> addresses.g4
* 
* endpoints
*     -> endpoints.g4
* 
* channels
*     -> channels.g4
* 
* messages
*     -> messages.g4
* 
* protocols
*     -> protocols.g4
* 
* requests
*     -> requests.g4
* 
* responses
*     -> responses.g4
* 
* routing
*     -> routing.g4
* 
* service discovery
*     -> service-discovery.g4
* 
* services
*     -> services.g4
* 
* sockets
*     -> sockets.g4
* 
* streams
*     -> streaming.g4
* 
* networking-facing distributed computation
*     -> distributed-compute.g4
* 
* networking capabilities
*     -> network-capabilities.g4
* 
* networking policy integration
*     -> policies.g4
* 
* networking security integration
*     -> security.g4
* 
* This file only composes those owners.
* 
* ============================================================================
* UNIVERSAL OWNERSHIP BOUNDARIES
* ============================================================================
* 
* Canonical lexical authority:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* Canonical token registry:
* 
* grammar/lexer/tokens.g4
* 
* Canonical names:
* 
* grammar/core/names.g4
* 
* Canonical types:
* 
* grammar/types/
* 
* Canonical expressions:
* 
* grammar/expressions/
* 
* Canonical resources:
* 
* grammar/resources/
* 
* Canonical effects:
* 
* grammar/effects/
* 
* Canonical contracts:
* 
* grammar/validation/
* 
* Canonical policies:
* 
* grammar/policies/
* 
* Canonical security:
* 
* grammar/security/
* 
* Canonical concurrency:
* 
* grammar/concurrency/
* 
* Canonical distributed semantics:
* 
* grammar/distributed/
* 
* Canonical quantum semantics:
* 
* grammar/quantum/
* 
* Canonical quantum IR:
* 
* quantum::ir
* 
* Canonical HDL/hardware semantics:
* 
* grammar/hdl/
* grammar/hardware/
* 
* Canonical parser composition:
* 
* grammar/antlr/ZamaniParser.g4
* 
* Networking MUST consume these public contracts rather than redefine them.
* 
* ============================================================================
* COMPLETE NETWORKING COMPONENT INVENTORY
* ============================================================================
* 
* The aggregate imports every existing networking component:
* 
* 1.  Addresses
* 2.  Endpoints
* 3.  NetworkingChannels
* 4.  Messages
* 5.  Protocols
* 6.  NetworkingRequests
* 7.  NetworkingResponses
* 8.  NetworkingRoutes
* 9.  NetworkingServiceDiscovery
* 10. NetworkingServices
* 11. Sockets
* 12. NetworkingStreaming
* 13. NetworkingDistributedCompute
* 14. NetworkCapabilities
* 15. NetworkingPolicies
* 16. NetworkingSecurity
* 
* Every component is imported exactly once by this composition root.
* 
* ============================================================================
* IMPORT MODEL
* ============================================================================
* 
* The dependency direction is:
* 
* canonical lexer
*      |
*      v
* universal/core grammars
*      |
*      v
* networking leaf grammars
*      |
*      v
* Networking
*      |
*      v
* ZamaniParser
* 
* The following reverse dependencies are forbidden:
* 
* networking component -> Networking
* networking component -> ZamaniParser
* networking component -> Zamani root
* Networking -> runtime
* Networking -> IR
* Networking -> HAL
* 
* Leaf grammars must remain independently understandable and independently
* testable.
* 
* ============================================================================
* ANTLR COMPOSITION CONTRACT
* ============================================================================
* 
* This is a parser grammar.
* 
* The canonical lexer is:
* 
* ZamaniLexer
* 
* No lexer rules are defined here.
* 
* The aggregate therefore consumes the exact token identities established by
* the canonical lexical architecture.
* 
* No networking grammar may introduce parser-side replacements for canonical
* token names.
* 
* In particular, this file does not define aliases for:
* 
* identifiers
* strings
* numbers
* operators
* punctuation
* delimiters
* 
* ============================================================================
  */

parser grammar Networking;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* NETWORKING COMPONENT IMPORTS
* ============================================================================
* 
* IMPORTANT:
* 
* This list is the complete networking composition boundary.
* 
* Adding a new networking component requires:
* 
* 1. creating its independent grammar;
* 2. assigning ownership;
* 3. defining its public entry rule;
* 4. defining its dependency contract;
* 5. defining AST/semantic/IR integration;
* 6. defining tests;
* 7. adding ONLY that component import here;
* 8. adding ONLY its public construct to networkingConstruct.
* 
* No leaf implementation is copied into this file.
* ============================================================================
  */

import
Addresses,
Endpoints,
NetworkingChannels,
Messages,
Protocols,
NetworkingRequests,
NetworkingResponses,
NetworkingRoutes,
NetworkingServiceDiscovery,
NetworkingServices,
Sockets,
NetworkingStreaming,
NetworkingDistributedCompute,
NetworkCapabilities,
NetworkingPolicies,
NetworkingSecurity
;

/*

* ============================================================================
* PUBLIC STANDALONE NETWORKING UNIT
* ============================================================================
* 
* This entry point is used when the networking composition is parsed or tested
* independently from the complete Zamani parser.
* 
* It consumes zero or more complete networking constructs and then requires
* EOF.
* 
* No finite cardinality is encoded.
* ============================================================================
  */

networkingUnit
: networkingConstruct* EOF
;

/*

* ============================================================================
* PRIMARY NETWORKING DISPATCH
* ============================================================================
* 
* This is the CENTRAL ORCHESTRATION RULE for grammar/networking/.
* 
* Every networking-domain construct enters through this dispatch.
* 
* Each alternative delegates immediately to the public entry point owned by
* its component grammar.
* 
* This rule MUST remain thin.
* 
* It MUST NOT contain the implementation of any networking component.
* ============================================================================
  */

networkingConstruct
: networkingAddress
| networkingEndpoint
| networkingChannel
| networkingMessage
| networkingProtocol
| networkingRequest
| networkingResponse
| networkingRoute
| networkingServiceDiscovery
| networkingService
| networkingSocket
| networkingStream
| networkingDistributedCompute
| networkingCapability
| networkingPolicy
| networkingSecurity
;

/*

* ============================================================================
* CANONICAL ZAMANI PARSER ADAPTER
* ============================================================================
* 
* grammar/antlr/ZamaniParser.g4 consumes the networking domain through stable
* aggregate-level entry points.
* 
* The parser must not know the complete networking leaf inventory.
* 
* The stable integration path is:
* 
* ZamaniParser
*      |
*      v
* networkingElement / universalNetworking
*      |
*      v
* networkingDeclaration
*      |
*      v
* networkingConstruct
*      |
*      v
* networking leaf owner
* 
* ============================================================================
  */

networkingDeclaration
: networkingConstruct
;

/*

* ============================================================================
* DOMAIN ELEMENT ADAPTER
* ============================================================================
* 
* "networkingElement" is the stable domain-level adapter consumed by the
* canonical parser composition.
* 
* It does not introduce another networking syntax.
* ============================================================================
  */

networkingElement
: networkingDeclaration
;

/*

* ============================================================================
* UNIVERSAL NETWORKING ADAPTER
* ============================================================================
* 
* "universalNetworking" is intentionally equivalent to the aggregate
* networking declaration boundary.
* 
* It does not create a second networking construct hierarchy.
* ============================================================================
  */

universalNetworking
: networkingDeclaration
;

/*

* ============================================================================
* STABLE COMPONENT ADAPTERS
* ============================================================================
* 
* Each adapter below performs ONE job:
* 
* aggregate classification
* 
* It does not duplicate the leaf grammar.
* 
* This gives the networking root stable public names while preserving the
* independent ownership of every networking component.
* ============================================================================
  */

/*

* ---
* ADDRESS
* ---
* 
* Owner:
* grammar/networking/addresses.g4
* 
* Public owner rule:
* addressConstruct
* 
* Aggregate adapter:
* networkingAddress
* ---

*/

networkingAddress
: addressConstruct
;

/*

* ---
* ENDPOINT
* ---
* 
* Owner:
* grammar/networking/endpoints.g4
* 
* Public owner rule:
* endpointDeclaration
* ---

*/

networkingEndpoint
: endpointDeclaration
;

/*

* ---
* CHANNEL
* ---
* 
* Owner:
* grammar/networking/channels.g4
* 
* Public owner rule:
* networkChannelConstruct
* ---

*/

networkingChannel
: networkChannelConstruct
;

/*

* ---
* MESSAGE
* ---
* 
* Owner:
* grammar/networking/messages.g4
* 
* Public owner rule:
* messageConstruct
* ---

*/

networkingMessage
: messageConstruct
;

/*

* ---
* PROTOCOL
* ---
* 
* Owner:
* grammar/networking/protocols.g4
* 
* Public owner rule:
* protocolConstruct
* ---

*/

networkingProtocol
: protocolConstruct
;

/*

* ---
* REQUEST
* ---
* 
* Owner:
* grammar/networking/requests.g4
* 
* Public owner rule:
* networkRequestConstruct
* ---

*/

networkingRequest
: networkRequestConstruct
;

/*

* ---
* RESPONSE
* ---
* 
* Owner:
* grammar/networking/responses.g4
* 
* Public owner rule:
* networkResponseConstruct
* ---

*/

networkingResponse
: networkResponseConstruct
;

/*

* ---
* ROUTE
* ---
* 
* Owner:
* grammar/networking/routing.g4
* 
* Public owner rule:
* networkRouteConstruct
* ---

*/

networkingRoute
: networkRouteConstruct
;

/*

* ---
* SERVICE DISCOVERY
* ---
* 
* Owner:
* grammar/networking/service-discovery.g4
* 
* Public owner rule:
* networkServiceDiscoveryConstruct
* ---

*/

networkingServiceDiscovery
: networkServiceDiscoveryConstruct
;

/*

* ---
* SERVICE
* ---
* 
* Owner:
* grammar/networking/services.g4
* 
* Public owner rule:
* networkServiceConstruct
* ---

*/

networkingService
: networkServiceConstruct
;

/*

* ---
* SOCKET
* ---
* 
* Owner:
* grammar/networking/sockets.g4
* 
* Public owner rule:
* socketConstruct
* ---

*/

networkingSocket
: socketConstruct
;

/*

* ---
* STREAM
* ---
* 
* Owner:
* grammar/networking/streaming.g4
* 
* Public owner rule:
* networkStreamingConstruct
* ---

*/

networkingStream
: networkStreamingConstruct
;

/*

* ---
* DISTRIBUTED COMPUTE
* ---
* 
* Owner:
* grammar/networking/distributed-compute.g4
* 
* Public owner rule:
* networkingDistributedComputeConstruct
* 
* IMPORTANT:
* 
* This is only the NETWORKING-FACING distributed-computation contract.
* 
* The distributed domain remains responsible for:
* 
* nodes
* placement
* partitioning
* replication
* consistency
* distributed scheduling
* distributed recovery
* deployment
* 
* ---

*/

networkingDistributedCompute
: networkingDistributedComputeConstruct
;

/*

* ---
* NETWORK CAPABILITY
* ---
* 
* Owner:
* grammar/networking/network-capabilities.g4
* 
* Public owner rule:
* networkCapabilityConstruct
* 
* Capability identity remains open-world and is resolved semantically.
* ---

*/

networkingCapability
: networkCapabilityConstruct
;

/*

* ---
* NETWORKING POLICY
* ---
* 
* Owner:
* grammar/networking/policies.g4
* 
* Public owner rule:
* networkingPolicyConstruct
* 
* IMPORTANT:
* 
* policies.g4 is itself only a networking adapter.
* 
* The canonical policy language remains owned by:
* 
* grammar/policies/
* 
* This aggregate MUST NOT import or reproduce universal policy rules directly.
* ---

*/

networkingPolicy
: networkingPolicyConstruct
;

/*

* ---
* NETWORKING SECURITY
* ---
* 
* Owner:
* grammar/networking/security.g4
* 
* Public owner rule:
* networkingSecurityConstruct
* 
* IMPORTANT:
* 
* security.g4 is an adapter to the canonical security subsystem.
* 
* The canonical security language remains owned by:
* 
* grammar/security/
* 
* This aggregate MUST NOT reproduce security syntax.
* ---

*/

networkingSecurity
: networkingSecurityConstruct
;

/*

* ============================================================================
* OWNERSHIP MATRIX
* ============================================================================
* 
* Aggregate construct             Canonical networking owner
* 
* networkingAddress           addresses.g4
* networkingEndpoint          endpoints.g4
* networkingChannel           channels.g4
* networkingMessage           messages.g4
* networkingProtocol          protocols.g4
* networkingRequest           requests.g4
* networkingResponse          responses.g4
* networkingRoute             routing.g4
* networkingServiceDiscovery  service-discovery.g4
* networkingService           services.g4
* networkingSocket            sockets.g4
* networkingStream            streaming.g4
* networkingDistributedCompute
*                             distributed-compute.g4
* networkingCapability        network-capabilities.g4
* networkingPolicy            policies.g4
* networkingSecurity          security.g4
* 
* No aggregate adapter is permitted to become a second source-level authority.
* 
* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* DEPENDS_ON:
* 
* grammar/antlr/ZamaniLexer.g4
* grammar/lexer/tokens.g4
* grammar/core/*
* grammar/types/*
* grammar/expressions/*
* grammar/resources/*
* grammar/effects/*
* grammar/validation/*
* grammar/policies/*
* grammar/security/*
* grammar/concurrency/*
* grammar/distributed/*
* networking component grammars imported above
* 
* TRANSITIVE DEPENDENCIES:
* 
* All dependencies declared by the imported networking component grammars.
* 
* EXPORTS:
* 
* Networking
* networkingUnit
* networkingConstruct
* networkingDeclaration
* networkingElement
* universalNetworking
* networkingAddress
* networkingEndpoint
* networkingChannel
* networkingMessage
* networkingProtocol
* networkingRequest
* networkingResponse
* networkingRoute
* networkingServiceDiscovery
* networkingService
* networkingSocket
* networkingStream
* networkingDistributedCompute
* networkingCapability
* networkingPolicy
* networkingSecurity
* 
* CONSUMED_BY:
* 
* grammar/antlr/ZamaniParser.g4
* grammar/Zamani.g4
* networking parser/conformance tests
* networking parser tooling
* 
* AST_OWNER:
* 
* domain-neutral Zamani frontend AST
* 
* SEMANTIC_OWNER:
* 
* networking semantic subsystem
* plus canonical owners of cross-domain semantics
* 
* TYPE_OWNER:
* 
* grammar/types/
* canonical type semantic implementation
* 
* EFFECT_OWNER:
* 
* grammar/effects/
* canonical effect semantic implementation
* 
* RESOURCE_OWNER:
* 
* grammar/resources/
* canonical resource analysis
* 
* CAPABILITY_OWNER:
* 
* grammar/core/capabilities.g4
* networking/network-capabilities.g4 for networking-facing capability
* syntax
* 
* POLICY_OWNER:
* 
* grammar/policies/
* networking/policies.g4 for networking adapter syntax
* 
* SECURITY_OWNER:
* 
* grammar/security/
* networking/security.g4 for networking adapter syntax
* 
* CONTRACT_OWNER:
* 
* grammar/validation/
* 
* PROVENANCE_OWNER:
* 
* canonical provenance subsystem
* 
* IR_OWNER:
* 
* canonical semantic/IR subsystem
* 
* Quantum-related computation:
*     quantum::ir
* 
* TEST_OWNER:
* 
* grammar/tests/networking/
* 
* SPEC_OWNER:
* 
* grammar/spec/networking.md
* 
* ARCHITECTURAL_SPEC_OWNER:
* 
* grammar/DESIGN.md
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* This grammar creates NO AST.
* 
* The frontend AST layer must preserve, as applicable:
* 
* source span
* source ordering
* construct kind
* declaration identity
* references
* expressions
* types
* attributes
* nested members
* requirements
* constraints
* capabilities
* preferences
* policies
* effects
* contracts
* metadata
* 
* The AST MUST NOT contain parser-selected:
* 
* transport
* router
* network interface
* machine
* CPU
* GPU
* FPGA
* ASIC
* QPU
* physical route
* physical address
* runtime socket
* allocated resource
* 
* unless those values are explicit source-level program data rather than
* parser decisions.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Parsing establishes structure only.
* 
* Semantic analysis is responsible for:
* 
* name resolution
* reference resolution
* type compatibility
* message compatibility
* protocol compatibility
* request/response compatibility
* endpoint compatibility
* channel compatibility
* service compatibility
* stream compatibility
* route validity
* discovery semantics
* capability satisfaction
* resource requirements
* effect requirements
* contract validation
* policy validation
* security validation
* distributed compatibility
* portability analysis
* target feasibility
* 
* A parser success MUST NOT be interpreted as semantic correctness.
* 
* ============================================================================
* RESOURCE CONTRACT
* ============================================================================
* 
* Networking constructs may express symbolic or expression-based resource
* requirements.
* 
* Examples of semantic intent include:
* 
* requires memory >= required_memory;
* 
* requires capability("network.communication");
* 
* requires capability("network.reliable");
* 
* requires topology(required_topology);
* 
* Such requirements are passed to semantic/resource analysis.
* 
* This grammar does NOT:
* 
* discover resources;
* allocate resources;
* reserve bandwidth;
* measure latency;
* choose interfaces;
* choose routes;
* determine node counts;
* determine device counts.
* 
* ============================================================================
* CAPABILITY CONTRACT
* ============================================================================
* 
* Capability identity is OPEN-WORLD.
* 
* Networking may express references such as:
* 
* networking::reliable_delivery
* 
* networking::ordered_delivery
* 
* networking::low_latency
* 
* networking::high_throughput
* 
* security::confidentiality
* 
* hardware::communication_fabric
* 
* quantum::communication
* 
* future::networking::new_capability
* 
* These are semantic references.
* 
* This aggregate does not enumerate or validate their physical existence.
* 
* Capability resolution belongs downstream.
* 
* ============================================================================
* EFFECT CONTRACT
* ============================================================================
* 
* Networking source constructs may semantically imply effects such as:
* 
* network
* io
* distributed
* foreign
* native
* randomness
* measurement
* 
* Effect classification belongs to the canonical effect subsystem.
* 
* This grammar does not perform effect checking.
* 
* ============================================================================
* CONTRACT CONTRACT
* ============================================================================
* 
* Networking constructs may participate in canonical contracts:
* 
* requires
* ensures
* invariant
* assume
* guarantee
* property
* 
* Contract semantics remain owned by the validation subsystem.
* 
* This aggregate does not define a second contract language.
* 
* ============================================================================
* POLICY CONTRACT
* ============================================================================
* 
* Networking policy syntax is exposed through:
* 
* networkingPolicyConstruct
* 
* but actual policy semantics remain owned by:
* 
* grammar/policies/
* 
* Networking policy integration may constrain:
* 
* communication
* discovery
* routing
* resource use
* security
* adaptation
* execution
* deployment
* simulation
* 
* The parser does not evaluate policy.
* 
* ============================================================================
* SECURITY CONTRACT
* ============================================================================
* 
* Networking security integration is exposed through:
* 
* networkingSecurityConstruct
* 
* but canonical security syntax and semantics remain owned by:
* 
* grammar/security/
* 
* Networking MUST NOT become the owner of:
* 
* identity
* authentication implementation
* authorization implementation
* cryptography
* key management
* trust enforcement
* credential handling
* secret storage
* 
* Security semantics are evaluated downstream.
* 
* ============================================================================
* PROVENANCE CONTRACT
* ============================================================================
* 
* Networking constructs must remain source-traceable.
* 
* The frontend and semantic pipeline may record:
* 
* source declaration
*      ->
* networking construct
*      ->
* semantic interpretation
*      ->
* capability/resource/policy validation
*      ->
* optimization
*      ->
* routing/planning
*      ->
* lowering
*      ->
* realization
* 
* This grammar does not generate provenance records.
* 
* ============================================================================
* QUANTUM CONTRACT
* ============================================================================
* 
* Networking may participate in quantum/classical computation.
* 
* Examples include communication involving:
* 
* quantum services
* quantum-classical hybrid computation
* entanglement-related communication
* quantum measurement results
* quantum-aware distributed computation
* 
* This aggregate does NOT define:
* 
* qubits
* physical qubits
* logical qubits
* quantum gates
* quantum states
* quantum topology
* calibration
* QEC
* QPU allocation
* 
* If networking semantics participate in quantum computation, the established
* quantum semantic path remains:
* 
* domain-neutral AST
*      ->
* quantum semantic analysis
*      ->
* quantum::ir
* 
* No networking-specific quantum IR is permitted.
* 
* ============================================================================
* CLASSICAL CONTRACT
* ============================================================================
* 
* Networking may participate in classical computation through:
* 
* functions
* tasks
* data pipelines
* actors
* services
* distributed computation
* accelerators
* 
* Classical semantics remain owned by the classical subsystem.
* 
* Networking does not create a competing classical IR.
* 
* ============================================================================
* HDL / HARDWARE CONTRACT
* ============================================================================
* 
* Networking may describe logical communication intent relevant to:
* 
* hardware/software co-design
* accelerators
* FPGA systems
* ASIC systems
* communication fabrics
* DMA-like capabilities
* hardware interfaces
* 
* This aggregate does not define:
* 
* pins
* wires
* fixed bus widths
* clock implementation
* physical interfaces
* placement
* routing implementation
* synthesis
* 
* Those belong to HDL/hardware semantics.
* 
* ============================================================================
* DISTRIBUTED CONTRACT
* ============================================================================
* 
* Networking and distributed execution are related but distinct.
* 
* Networking owns:
* 
* communication intent
* communication contracts
* network-facing distributed contracts
* 
* Distributed execution owns:
* 
* nodes
* placement
* partitioning
* replication
* consistency
* distributed scheduling
* distributed recovery
* deployment
* distributed membership
* 
* The networking aggregate MUST NOT duplicate the distributed grammar.
* 
* ============================================================================
* CONCURRENCY CONTRACT
* ============================================================================
* 
* Networking is not the owner of language-level concurrency primitives.
* 
* Existing concurrency constructs such as:
* 
* actors
* tasks
* channels
* async
* synchronization
* 
* remain owned by grammar/concurrency/.
* 
* Networking channels are distinct from generic concurrency channels.
* 
* Cross-domain semantic integration occurs downstream.
* 
* ============================================================================
* ROUTING CONTRACT
* ============================================================================
* 
* routing.g4 owns source-level routing intent.
* 
* It does not implement:
* 
* route discovery
* shortest path
* packet forwarding
* topology probing
* interface selection
* congestion control
* 
* The aggregate merely exposes the routing construct.
* 
* Actual routing realization occurs downstream.
* 
* ============================================================================
* SERVICE DISCOVERY CONTRACT
* ============================================================================
* 
* service-discovery.g4 owns logical discovery intent.
* 
* It does not:
* 
* query DNS;
* contact registries;
* inspect runtime services;
* discover hardware;
* probe networks.
* 
* Those are runtime/compiler responsibilities.
* 
* ============================================================================
* SOCKET CONTRACT
* ============================================================================
* 
* sockets.g4 defines logical socket contracts.
* 
* A source-level socket is not inherently an operating-system socket.
* 
* It may eventually be realized as:
* 
* in-process communication
* shared memory
* IPC
* network transport
* accelerator fabric
* distributed communication
* future communication substrate
* 
* Physical realization is downstream.
* 
* ============================================================================
* STREAMING CONTRACT
* ============================================================================
* 
* streaming.g4 defines logical streams.
* 
* Streams may represent arbitrarily long or incremental data flows without
* imposing a grammar-level message-count or stream-length limit.
* 
* Runtime buffering, storage, scheduling and transport constraints are
* implementation/resource concerns.
* 
* ============================================================================
* PROTOCOL CONTRACT
* ============================================================================
* 
* protocols.g4 defines logical communication protocol intent.
* 
* It does not define a closed physical protocol catalogue.
* 
* Protocol implementations, transport bindings, routing and scheduling are
* downstream concerns.
* 
* ============================================================================
* MESSAGE CONTRACT
* ============================================================================
* 
* messages.g4 owns logical message schemas.
* 
* This aggregate does not redefine:
* 
* message fields
* message values
* message types
* serialization
* wire formats
* 
* ============================================================================
* REQUEST / RESPONSE CONTRACT
* ============================================================================
* 
* requests.g4 owns reusable first-class request contracts.
* 
* responses.g4 owns reusable first-class response contracts.
* 
* services.g4 may own service-local request/response relationships.
* 
* These are deliberately separate ownership domains.
* 
* The aggregate composes them without merging their syntax.
* 
* ============================================================================
* ADDRESS / ENDPOINT CONTRACT
* ============================================================================
* 
* addresses.g4 owns address syntax.
* 
* endpoints.g4 owns logical communication participant syntax.
* 
* The aggregate does not reinterpret either as physical allocation.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This grammar creates NO IR.
* 
* Networking semantics may eventually contribute to:
* 
* canonical semantic representation
* networking representation
* distributed representation
* classical representation
* HDL/hardware intent
* deployment representation
* 
* If quantum computation is involved:
* 
* quantum::ir
* 
* remains the canonical quantum IR boundary.
* 
* No:
* 
* NetworkingIR
* NetworkIR
* QuantumNetworkingIR
* 
* may be introduced by this grammar.
* 
* ============================================================================
* BACKEND CONTRACT
* ============================================================================
* 
* Backend selection is determined downstream from:
* 
* semantic requirements
* capabilities
* resources
* effects
* contracts
* policies
* deployment context
* target availability
* 
* This grammar MUST NOT select:
* 
* CPU
* GPU
* FPGA
* ASIC
* accelerator
* QPU
* NIC
* router
* provider
* transport
* machine
* node
* 
* ============================================================================
* RUNTIME CONTRACT
* ============================================================================
* 
* Runtime/compiler infrastructure may perform:
* 
* endpoint binding
* address resolution
* service discovery
* transport selection
* serialization
* routing
* scheduling
* placement
* resource allocation
* authentication
* authorization
* key resolution
* monitoring
* retries
* recovery
* failover
* migration
* adaptation
* 
* None of these operations occur during parsing.
* 
* ============================================================================
* DETERMINISM CONTRACT
* ============================================================================
* 
* Parsing must depend only upon:
* 
* source
* canonical token stream
* selected grammar
* explicitly supplied language configuration
* 
* Parsing MUST NOT depend upon:
* 
* current network state
* DNS state
* filesystem state
* hardware availability
* runtime state
* environment variables
* wall-clock time
* randomness
* scheduler state
* service availability
* credentials
* 
* Identical input under identical grammar configuration must yield equivalent
* parse structure and source spans.
* 
* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* The aggregate uses unbounded grammar repetition:
* 
* networkingConstruct*
* 
* and delegates cardinality to component grammars.
* 
* There is no grammar-level finite limit on the number of:
* 
* networking declarations
* endpoints
* addresses
* channels
* messages
* protocols
* requests
* responses
* routes
* services
* sockets
* streams
* capability declarations
* policies
* security integrations
* 
* Practical limits may arise from:
* 
* source size
* parser memory
* parser implementation
* AST memory
* semantic-analysis resources
* compiler configuration
* runtime resources
* deployment policy
* target capability
* 
* Such limits MUST remain distinguishable from language semantics.
* 
* ============================================================================
* "INFINITY" CONTRACT
* ============================================================================
* 
* "Infinity" in the scalability requirement means:
* 
* no artificial universal language ceiling.
* 
* It does not mean:
* 
* infinite physical memory
* infinite network bandwidth
* infinite nodes
* infinite execution time
* infinite compiler resources
* 
* A finite implementation may reject a program because of actual resource
* exhaustion or an explicitly configured implementation budget.
* 
* Such rejection MUST NOT be represented as a language-level networking
* cardinality rule.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This file MUST NOT contain:
* 
* transport enumerations
* provider enumerations
* vendor enumerations
* machine enumerations
* device enumerations
* node enumerations
* physical interface enumerations
* universal topology definitions
* universal bandwidth limits
* universal latency limits
* message-size ceilings
* connection ceilings
* endpoint ceilings
* service ceilings
* protocol ceilings
* 
* It MUST NOT introduce:
* 
* MAX_*
* 
* constants for networking capacity.
* 
* Numeric literals used by programs remain program values.
* 
* They are not networking capacity declarations unless interpreted by a
* downstream semantic construct whose specification explicitly gives them
* that meaning.
* 
* ============================================================================
* EXTENSIBILITY CONTRACT
* ============================================================================
* 
* A new networking technology should normally be introduced through:
* 
* names
* capabilities
* metadata
* dialects
* libraries
* policies
* semantic registrations
* target integrations
* 
* rather than by extending this aggregate grammar.
* 
* This means the networking composition root should remain stable as the
* networking ecosystem evolves.
* 
* A new source-level networking construct DOES require a new component grammar
* when its syntax cannot be represented by an existing owner.
* 
* In that case:
* 
* new component
*      ->
* independent grammar
*      ->
* ownership contract
*      ->
* AST contract
*      ->
* semantic contract
*      ->
* tests
*      ->
* one import here
*      ->
* one dispatch alternative here
* 
* Nothing else in this aggregate should be duplicated.
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* Existing public aggregate entry points are preserved:
* 
* networkingUnit
* networkingConstruct
* networkingDeclaration
* networkingElement
* universalNetworking
* 
* Existing component adapters are preserved:
* 
* networkingAddress
* networkingEndpoint
* networkingChannel
* networkingMessage
* networkingProtocol
* networkingRequest
* networkingResponse
* networkingRoute
* networkingServiceDiscovery
* networkingService
* networkingSocket
* networkingStream
* networkingDistributedCompute
* networkingCapability
* 
* New aggregate adapters are:
* 
* networkingPolicy
* networkingSecurity
* 
* These expose already-existing networking adapter grammars:
* 
* policies.g4
* security.g4
* 
* They do not create new policy or security syntax.
* 
* ============================================================================
* ERROR BOUNDARY
* ============================================================================
* 
* Syntax errors belong to the parser.
* 
* Examples:
* 
* malformed declaration
* malformed delimiter
* incomplete construct
* malformed nested structure
* 
* Semantic errors belong downstream.
* 
* Examples:
* 
* unknown endpoint
* incompatible protocol
* invalid message type
* unsatisfied capability
* insufficient resource
* forbidden policy
* invalid security requirement
* invalid route
* unavailable target
* 
* A target-capability failure MUST NOT be reported as a networking grammar
* syntax failure.
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* This aggregate must be tested independently and through the complete parser.
* 
* REQUIRED TEST CATEGORIES
* ---
* 
* lexical conformance
* parser conformance
* AST conformance
* semantic conformance
* negative syntax
* diagnostics
* boundary
* scalability
* determinism
* compatibility
* cross-domain
* resource/capability
* policy
* security
* 
* ============================================================================
* MINIMUM POSITIVE COVERAGE
* ============================================================================
* 
* The networking test suite must prove parsing of:
* 
* address
* endpoint
* channel
* message
* protocol
* request
* response
* route
* service discovery
* service
* socket
* stream
* networking distributed compute
* networking capability
* networking policy integration
* networking security integration
* 
* ============================================================================
* MINIMUM CROSS-DOMAIN COVERAGE
* ============================================================================
* 
* The aggregate must be tested with:
* 
* classical computation
* quantum computation
* hybrid computation
* HDL
* hardware intent
* AI/data computation
* concurrency
* distributed execution
* security
* resources
* capabilities
* effects
* contracts
* policies
* provenance
* simulation
* interoperability
* 
* Cross-domain tests must verify that networking remains a communication
* domain rather than becoming a competing semantic universe.
* 
* ============================================================================
* SCALABILITY TEST CONTRACT
* ============================================================================
* 
* Tests must vary structural size without introducing a language maximum.
* 
* Relevant dimensions include:
* 
* networking construct count
* endpoint count
* address count
* channel count
* message count
* protocol count
* request count
* response count
* route count
* service count
* socket count
* stream count
* capability count
* policy count
* security declaration count
* qualified-name depth
* nested construct depth
* 
* The tests must distinguish:
* 
* language validity
* 
* from:
* 
* implementation resource exhaustion.
* 
* ============================================================================
* DETERMINISM TEST CONTRACT
* ============================================================================
* 
* Repeated parsing of identical source under identical grammar configuration
* must produce equivalent:
* 
* token sequence
* parse structure
* source spans
* diagnostics
* 
* No network state or target state may affect parsing.
* 
* ============================================================================
* NEGATIVE TEST CONTRACT
* ============================================================================
* 
* The aggregate must reject malformed networking syntax rather than accepting
* arbitrary prefixes.
* 
* Examples include:
* 
* incomplete declarations
* malformed component bodies
* missing required names
* missing delimiters
* malformed references
* malformed nested constructs
* 
* Component-specific negative syntax remains owned by the component grammar.
* 
* ============================================================================
* INTEGRATION CONTRACT
* ============================================================================
* 
* UPSTREAM
* ---
* 
* grammar/lexer/
* grammar/antlr/ZamaniLexer.g4
* grammar/core/
* grammar/types/
* grammar/expressions/
* grammar/resources/
* grammar/effects/
* grammar/validation/
* grammar/policies/
* grammar/security/
* 
* COMPONENTS
* ---
* 
* grammar/networking/addresses.g4
* grammar/networking/endpoints.g4
* grammar/networking/channels.g4
* grammar/networking/messages.g4
* grammar/networking/protocols.g4
* grammar/networking/requests.g4
* grammar/networking/responses.g4
* grammar/networking/routing.g4
* grammar/networking/service-discovery.g4
* grammar/networking/services.g4
* grammar/networking/sockets.g4
* grammar/networking/streaming.g4
* grammar/networking/distributed-compute.g4
* grammar/networking/network-capabilities.g4
* grammar/networking/policies.g4
* grammar/networking/security.g4
* 
* AGGREGATOR
* ---
* 
* This file.
* 
* ROOT PARSER
* ---
* 
* grammar/antlr/ZamaniParser.g4
* 
* ROOT COMPOSITION
* ---
* 
* grammar/Zamani.g4
* 
* DOWNSTREAM
* ---
* 
* domain-neutral AST
* structural validation
* semantic analysis
* type analysis
* effect analysis
* capability analysis
* resource analysis
* contract validation
* policy validation
* security validation
* provenance
* networking semantics
* distributed semantics
* classical semantics
* hybrid semantics
* quantum semantics
* HDL/hardware semantics
* canonical IR
* quantum::ir
* optimization
* lowering
* routing
* scheduling
* resilience
* ZQN
* HAL
* target realization
* 
* ============================================================================
* INTEGRATION INVARIANTS
* ============================================================================
* 
* 1. This file is the only networking composition root.
* 
* 2. Every networking leaf has one source-level owner.
* 
* 3. The aggregate never duplicates leaf syntax.
* 
* 4. The aggregate never defines lexer rules.
* 
* 5. The aggregate never creates AST nodes.
* 
* 6. The aggregate never creates IR.
* 
* 7. The aggregate never performs semantic analysis.
* 
* 8. The aggregate never performs target discovery.
* 
* 9. The aggregate never performs runtime network operations.
* 
* 10. The aggregate never chooses a transport.
* 
* 11. The aggregate never chooses a provider.
* 
* 12. The aggregate never chooses hardware.
* 
* 13. The aggregate never allocates resources.
* 
* 14. The aggregate never performs routing.
* 
* 15. The aggregate never performs scheduling.
* 
* 16. The aggregate never performs service discovery.
* 
* 17. The aggregate never performs authentication.
* 
* 18. The aggregate never performs authorization.
* 
* 19. The aggregate never performs cryptography.
* 
* 20. The aggregate never defines physical topology.
* 
* 21. The aggregate never defines finite networking capacity.
* 
* 22. The aggregate preserves open-world capability identities.
* 
* 23. The aggregate preserves canonical policy ownership.
* 
* 24. The aggregate preserves canonical security ownership.
* 
* 25. Quantum semantics retain the quantum::ir boundary.
* 
* 26. Distributed execution remains owned by the distributed subsystem.
* 
* 27. Concurrency primitives remain owned by the concurrency subsystem.
* 
* 28. Hardware realization remains downstream.
* 
* 29. Rust implementation remains safe Rust.
* 
* 30. No unsafe Rust is required by this grammar.
* 
* ============================================================================
* FILE COMPLETION CONTRACT
* ============================================================================
* 
* This file is DONE when ALL of the following are true:
* 
* AUTHORITY
* ---
* 
* [ ] This file is the sole composition root for grammar/networking/.
* 
* [ ] No networking leaf duplicates another networking leaf's ownership.
* 
* [ ] Every networking component has a documented owner.
* 
* LEXICAL
* ---
* 
* [ ] All tokens originate from ZamaniLexer.
* 
* [ ] No lexer rules exist in this file.
* 
* [ ] No parser-side token aliases are introduced.
* 
* GRAMMAR
* ---
* 
* [ ] All networking component grammars are imported exactly once.
* 
* [ ] Every imported component exposes the expected public construct rule.
* 
* [ ] networkingConstruct dispatches to every networking component.
* 
* [ ] networkingUnit consumes complete input through EOF.
* 
* [ ] Stable parser integration rules are preserved.
* 
* INTEGRATION
* ---
* 
* [ ] ZamaniParser.g4 consumes the aggregate rather than networking leaves.
* 
* [ ] Zamani.g4 does not duplicate networking rules.
* 
* [ ] Component grammars do not import this aggregate.
* 
* [ ] Component grammars do not import ZamaniParser.g4.
* 
* [ ] Component grammars do not create competing networking roots.
* 
* SEMANTICS
* ---
* 
* [ ] AST mapping is downstream.
* 
* [ ] Semantic validation is downstream.
* 
* [ ] Type checking is downstream.
* 
* [ ] Effect checking is downstream.
* 
* [ ] Capability checking is downstream.
* 
* [ ] Resource checking is downstream.
* 
* [ ] Contract validation is downstream.
* 
* [ ] Policy validation is downstream.
* 
* [ ] Security validation is downstream.
* 
* [ ] Provenance is preserved downstream.
* 
* IR
* --
* 
* [ ] No IR is constructed here.
* 
* [ ] Networking semantics have a documented canonical semantic destination.
* 
* [ ] Quantum participation preserves quantum::ir.
* 
* [ ] No competing quantum networking IR exists.
* 
* SCALABILITY
* ---
* 
* [ ] No universal networking capacity constant exists.
* 
* [ ] No finite transport catalogue exists.
* 
* [ ] No provider catalogue exists.
* 
* [ ] No hardware enumeration exists.
* 
* [ ] No node limit exists.
* 
* [ ] No endpoint limit exists.
* 
* [ ] No connection limit exists.
* 
* [ ] No bandwidth limit exists.
* 
* [ ] No latency limit exists.
* 
* [ ] No message-size limit exists.
* 
* SAFETY
* ---
* 
* [ ] No embedded Rust exists.
* 
* [ ] No unsafe Rust exists.
* 
* [ ] No parser action exists.
* 
* [ ] No semantic predicate exists.
* 
* [ ] No filesystem access exists.
* 
* [ ] No network access exists.
* 
* [ ] No hardware access exists.
* 
* [ ] No runtime callback exists.
* 
* DETERMINISM
* ---
* 
* [ ] Parsing is independent of runtime network state.
* 
* [ ] Parsing is independent of hardware state.
* 
* [ ] Parsing is independent of environment state.
* 
* [ ] Parsing is independent of randomness.
* 
* [ ] Determinism tests pass.
* 
* TESTING
* ---
* 
* [ ] Positive tests pass.
* 
* [ ] Negative tests pass.
* 
* [ ] Boundary tests pass.
* 
* [ ] Scalability tests pass.
* 
* [ ] Cross-domain tests pass.
* 
* [ ] Compatibility tests pass.
* 
* [ ] Diagnostics tests pass.
* 
* [ ] ANTLR generation succeeds.
* 
* [ ] Rust parser integration succeeds.
* 
* [ ] Rust 1.97+ compilation succeeds.
* 
* [ ] Rust implementation contains no unsafe code.
* 
* ============================================================================
* FINAL ARCHITECTURAL INVARIANT
* ============================================================================
* 
* grammar/networking/networking.g4 defines:
* 
* HOW NETWORKING GRAMMARS ARE COMPOSED.
* 
* It does NOT define:
* 
* HOW NETWORKS ARE PHYSICALLY IMPLEMENTED.
* 
* The complete architecture is:
* 
* source
*   ->
* canonical lexer
*   ->
* canonical parser
*   ->
* Networking aggregate
*   ->
* networking component
*   ->
* domain-neutral AST
*   ->
* semantic validation
*   ->
* capability/resource/policy/effect analysis
*   ->
* canonical semantic representation
*   ->
* optimization
*   ->
* lowering
*   ->
* routing
*   ->
* scheduling
*   ->
* resilience
*   ->
* ZQN
*   ->
* HAL
*   ->
* available target
* 
* Therefore a single source program can preserve its source-level networking
* meaning while its realization changes across:
* 
* tiny
* embedded
* classical
* accelerator
* quantum
* HDL
* HPC
* cluster
* distributed
* cloud
* future
* 
* subject to:
* 
* program semantics
* declared requirements
* capabilities
* resources
* effects
* contracts
* policies
* security requirements
* target feasibility
* 
* This is the networking composition mechanism required for:
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* without turning the networking grammar into a catalogue of today's hardware,
* transports, providers, or physical network limits.
* 
* ============================================================================
  */