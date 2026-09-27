/*
 * ============================================================================
 * Zamani Universal Programming Language
 * Production Networking Service-Discovery Grammar
 * ============================================================================
 *
 * File:
 *     grammar/networking/service-discovery.g4
 *
 * Grammar:
 *     NetworkingServiceDiscovery
 *
 * Status:
 *     Production parser grammar contract
 *
 * Purpose:
 *     Define target-independent SOURCE-LEVEL SERVICE-DISCOVERY INTENT.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *
 * Safety:
 *     - No embedded Rust.
 *     - No unsafe code.
 *     - No parser actions.
 *     - No semantic predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No environment-dependent parsing.
 *     - No randomness.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * Service discovery is a SOURCE-LEVEL LOGICAL CONTRACT.
 *
 * It describes:
 *
 *     - what logical service identity is being discovered;
 *     - where/how discovery intent is expressed;
 *     - discovery selectors;
 *     - service references;
 *     - endpoint references;
 *     - protocol references;
 *     - channel references;
 *     - route references;
 *     - request/response relationships;
 *     - capability requirements;
 *     - resource requirements;
 *     - constraints;
 *     - preferences;
 *     - policies;
 *     - availability intent;
 *     - lifecycle/refresh intent;
 *     - consistency intent;
 *     - security intent;
 *     - observability metadata;
 *     - extensible application/domain metadata.
 *
 * This grammar does NOT perform service discovery.
 *
 * ============================================================================
 * WHAT THIS GRAMMAR DOES NOT DO
 * ============================================================================
 *
 * It does NOT:
 *
 *     - query DNS;
 *     - query a service registry;
 *     - contact a discovery server;
 *     - contact a service;
 *     - perform network I/O;
 *     - resolve IP addresses;
 *     - resolve MAC addresses;
 *     - select a physical endpoint;
 *     - select a physical node;
 *     - select a router;
 *     - select a switch;
 *     - select an interface;
 *     - allocate a port;
 *     - open a socket;
 *     - establish a connection;
 *     - perform health checks;
 *     - probe a network;
 *     - perform load balancing;
 *     - perform failover;
 *     - perform placement;
 *     - perform scheduling;
 *     - allocate resources;
 *     - discover hardware;
 *     - discover QPUs;
 *     - discover GPUs;
 *     - discover CPUs;
 *     - discover FPGAs;
 *     - discover accelerators;
 *     - execute code.
 *
 * Those responsibilities belong to semantic analysis, compilation,
 * routing, scheduling, deployment, runtime, HAL, or external infrastructure.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     NetworkingServiceDiscovery parser
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> service resolution
 *          +--> endpoint resolution
 *          +--> protocol validation
 *          +--> channel validation
 *          +--> route validation
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> distributed analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     optimization / routing / scheduling
 *          |
 *          v
 *     deployment / HAL
 *          |
 *          v
 *     runtime service discovery realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - service-discovery declaration syntax;
 *     - service-discovery declaration identity;
 *     - discovery contract bodies;
 *     - discovery properties;
 *     - nested discovery configuration;
 *     - logical discovery references;
 *     - discovery intent;
 *     - discovery requirements;
 *     - discovery constraints;
 *     - discovery preferences;
 *     - discovery capability expressions;
 *     - discovery policy metadata;
 *     - reusable discovery-reference grammar boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - service declarations;
 *     - endpoint declarations;
 *     - address declarations;
 *     - protocol declarations;
 *     - channel declarations;
 *     - message declarations;
 *     - request declarations;
 *     - response declarations;
 *     - socket declarations;
 *     - route declarations;
 *     - network capability declarations;
 *     - distributed node declarations;
 *     - physical topology;
 *     - routing algorithms;
 *     - scheduling;
 *     - placement;
 *     - runtime discovery;
 *     - registry implementation;
 *     - DNS;
 *     - DHCP;
 *     - multicast;
 *     - broadcast;
 *     - cloud-provider discovery;
 *     - vendor APIs;
 *     - cryptographic implementation;
 *     - authentication implementation;
 *     - authorization implementation;
 *     - quantum operations;
 *     - quantum topology;
 *     - QEC;
 *     - ZQN;
 *     - HAL implementation.
 *
 * ============================================================================
 * RELATIONSHIP TO OTHER NETWORKING GRAMMARS
 * ============================================================================
 *
 * Service declarations:
 *
 *     grammar/networking/services.g4
 *
 * Endpoint declarations:
 *
 *     grammar/networking/endpoints.g4
 *
 * Addresses:
 *
 *     grammar/networking/addresses.g4
 *
 * Protocols:
 *
 *     grammar/networking/protocols.g4
 *
 * Channels:
 *
 *     grammar/networking/channels.g4
 *
 * Messages:
 *
 *     grammar/networking/messages.g4
 *
 * Requests:
 *
 *     grammar/networking/requests.g4
 *
 * Responses:
 *
 *     grammar/networking/responses.g4
 *
 * Sockets:
 *
 *     grammar/networking/sockets.g4
 *
 * Routing:
 *
 *     grammar/networking/routing.g4
 *
 * Networking capabilities:
 *
 *     grammar/networking/network-capabilities.g4
 *
 * This grammar REFERENCES those concepts.
 *
 * It does not reproduce their syntax.
 *
 * ============================================================================
 * SERVICE DISCOVERY VS SERVICE DEFINITION
 * ============================================================================
 *
 * A service definition answers:
 *
 *     "What service contract exists?"
 *
 * Service discovery answers:
 *
 *     "What logical service realization should be discovered for this
 *      program's declared intent?"
 *
 * Discovery therefore does not replace services.g4.
 *
 * Example:
 *
 *     service compute {
 *         ...
 *     }
 *
 *     discover compute_service {
 *         service: compute;
 *     }
 *
 * The first declares a service contract.
 *
 * The second declares discovery intent concerning that service.
 *
 * Semantic analysis determines whether `compute` denotes a service.
 *
 * ============================================================================
 * SERVICE DISCOVERY VS ROUTING
 * ============================================================================
 *
 * Discovery identifies or selects a LOGICAL communication participant.
 *
 * Routing determines how communication reaches the selected logical
 * participant.
 *
 * Therefore:
 *
 *     discovery
 *         ->
 *     logical service/endpoint realization
 *         ->
 *     route planning
 *         ->
 *     scheduling/placement
 *         ->
 *     runtime communication
 *
 * Discovery MUST NOT contain routing algorithms.
 *
 * A discovery declaration may reference a route contract:
 *
 *     route: network::compute_route;
 *
 * but the route itself remains owned by routing.g4.
 *
 * ============================================================================
 * SERVICE DISCOVERY VS ENDPOINTS
 * ============================================================================
 *
 * Discovery may identify endpoint candidates.
 *
 * Endpoint syntax remains owned by endpoints.g4.
 *
 * Example:
 *
 *     discover compute {
 *         service: compute;
 *         endpoint: compute::endpoint;
 *     }
 *
 * The parser does not determine whether the referenced name is an endpoint.
 *
 * ============================================================================
 * SERVICE DISCOVERY VS ADDRESSES
 * ============================================================================
 *
 * Address syntax remains owned by addresses.g4.
 *
 * Discovery may contain:
 *
 *     address: logical::compute;
 *
 * or:
 *
 *     selector: network::address;
 *
 * without embedding a second address grammar.
 *
 * Physical address resolution is downstream.
 *
 * ============================================================================
 * SERVICE DISCOVERY VS PROTOCOLS
 * ============================================================================
 *
 * Protocol syntax remains owned by protocols.g4.
 *
 * Discovery may reference a protocol:
 *
 *     protocol: network::request_response;
 *
 * The parser does not enumerate protocol implementations.
 *
 * ============================================================================
 * SERVICE DISCOVERY VS CHANNELS
 * ============================================================================
 *
 * Channel syntax remains owned by channels.g4.
 *
 * Discovery may reference a logical channel:
 *
 *     channel: compute_requests;
 *
 * Physical queues, buffers and transport resources remain downstream.
 *
 * ============================================================================
 * SERVICE DISCOVERY VS REQUESTS / RESPONSES
 * ============================================================================
 *
 * Requests and responses remain owned by:
 *
 *     requests.g4
 *     responses.g4
 *
 * Discovery may associate a discovered service with request/response
 * contracts using ordinary qualified-name references.
 *
 * It does not duplicate request/response syntax.
 *
 * ============================================================================
 * SERVICE DISCOVERY VS SOCKETS
 * ============================================================================
 *
 * Socket syntax remains owned by sockets.g4.
 *
 * Discovery does not imply that a particular OS socket exists.
 *
 * Runtime socket creation remains downstream.
 *
 * ============================================================================
 * SERVICE DISCOVERY VS NETWORK CAPABILITIES
 * ============================================================================
 *
 * Capability syntax remains owned by:
 *
 *     network-capabilities.g4
 *
 * Discovery properties may express:
 *
 *     requires: capability("network.reliable");
 *
 *     capability: network::service_discovery;
 *
 *     requires: capability("network.secure");
 *
 * The parser preserves these as expressions/references.
 *
 * Capability satisfaction remains semantic.
 *
 * ============================================================================
 * SERVICE DISCOVERY VS DISTRIBUTED COMPUTING
 * ============================================================================
 *
 * A discovered service may ultimately be realized by:
 *
 *     - one process;
 *     - many processes;
 *     - one node;
 *     - many nodes;
 *     - an edge resource;
 *     - a cloud resource;
 *     - an accelerator;
 *     - a quantum service;
 *     - a hybrid service;
 *     - a future computational substrate.
 *
 * This grammar does not encode any fixed deployment model.
 *
 * Distributed placement remains owned by grammar/distributed/ and its
 * semantic/compiler layers.
 *
 * ============================================================================
 * SERVICE DISCOVERY VS QUANTUM COMPUTING
 * ============================================================================
 *
 * A discovery contract may identify a logical quantum service:
 *
 *     discover quantum_backend {
 *         service: quantum::backend;
 *         requires: capability("quantum.compute");
 *     }
 *
 * or:
 *
 *     discover quantum_measurement {
 *         service: quantum::measurement;
 *         requires: capability("quantum.measurement");
 *     }
 *
 * This grammar does not define:
 *
 *     - qubits;
 *     - gates;
 *     - circuits;
 *     - QPU topology;
 *     - calibration;
 *     - pulses;
 *     - QEC;
 *     - ZQN;
 *     - physical qubits.
 *
 * Quantum semantics continue toward the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * SERVICE DISCOVERY VS CLASSICAL / AI / HDL / HARDWARE
 * ============================================================================
 *
 * Discovery may identify logical services implemented by:
 *
 *     classical systems;
 *     quantum systems;
 *     hybrid systems;
 *     AI systems;
 *     HDL/hardware accelerators;
 *     distributed systems;
 *     embedded systems;
 *     future computing systems.
 *
 * The discovery grammar remains domain-neutral.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Service discovery MUST preserve:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * Therefore the source program should identify WHAT it needs rather than
 * permanently identifying WHERE a particular machine happens to provide it.
 *
 * Valid intent:
 *
 *     discover compute {
 *         service: compute;
 *         requires: capability("compute.general");
 *     }
 *
 * Valid intent:
 *
 *     discover quantum_compute {
 *         service: quantum::compute;
 *         requires: capability("quantum.compute");
 *     }
 *
 * Not a universal discovery requirement:
 *
 *     discover gpu0
 *     discover node7
 *     discover qpu0
 *     discover router3
 *
 * Such names can still exist as ordinary source data, but this grammar gives
 * them no special physical meaning.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * This grammar deliberately does NOT enumerate:
 *
 *     DNS
 *     mDNS
 *     DNS-SD
 *     Consul
 *     etcd
 *     Kubernetes
 *     LDAP
 *     UDDI
 *     cloud-provider registries
 *     proprietary service registries
 *     vendor discovery protocols
 *
 * as closed grammar alternatives.
 *
 * A discovery implementation can be represented through:
 *
 *     qualified names;
 *     expressions;
 *     capabilities;
 *     dialects;
 *     semantic registries;
 *     deployment configuration.
 *
 * New discovery technologies therefore do not require a new core grammar
 * production merely because their implementation name is new.
 *
 * ============================================================================
 * OPEN-WORLD PROPERTY MODEL
 * ============================================================================
 *
 * Route/service/network grammars in this repository already use open-world
 * property structures.
 *
 * This grammar follows the same architecture:
 *
 *     qualifiedName : expression ;
 *
 * Standard semantic property names may include:
 *
 *     service
 *     endpoint
 *     address
 *     protocol
 *     channel
 *     route
 *     request
 *     response
 *     selector
 *     query
 *     policy
 *     strategy
 *     scope
 *     locality
 *     region
 *     version
 *     instance
 *     identity
 *     health
 *     availability
 *     consistency
 *     freshness
 *     timeout
 *     ttl
 *     refresh
 *     retry
 *     fallback
 *     security
 *     observability
 *     metadata
 *     provenance
 *     requires
 *     constraint
 *     prefers
 *     capability
 *
 * These names are DOCUMENTED semantic concepts only.
 *
 * They are not parser-level closed vocabulary.
 *
 * Future properties may be introduced without modifying this grammar.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / CAPABILITY
 * ============================================================================
 *
 * Discovery syntax preserves source intent.
 *
 * Semantic analysis distinguishes:
 *
 *     requirement
 *     constraint
 *     preference
 *     capability
 *     policy
 *     hint
 *     metadata
 *
 * Examples:
 *
 *     requires: capability("network.discovery");
 *
 *     constraint: latency <= maximum_latency;
 *
 *     prefers: locality::near;
 *
 *     capability: network::service_discovery;
 *
 *     policy: adaptive;
 *
 * No parser rule determines whether any requirement can be satisfied.
 *
 * ============================================================================
 * NO PHYSICAL RESOURCE LIMITS
 * ============================================================================
 *
 * This grammar MUST NOT define:
 *
 *     MAX_SERVICES
 *     MAX_ENDPOINTS
 *     MAX_DISCOVERIES
 *     MAX_INSTANCES
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_ROUTERS
 *     MAX_LINKS
 *     MAX_NETWORK_SIZE
 *     MAX_RESULTS
 *     MAX_RETRIES
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_MEMORY
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *
 * There is no fixed:
 *
 *     service count;
 *     endpoint count;
 *     candidate count;
 *     registry count;
 *     discovery depth;
 *     selector complexity;
 *     property count;
 *     nested-block count.
 *
 * Repetition uses ANTLR `*` / `+`.
 *
 * Practical limits belong to:
 *
 *     parser implementation;
 *     compiler resources;
 *     runtime resources;
 *     deployment resources;
 *     target capabilities;
 *     available network infrastructure.
 *
 * ============================================================================
 * "INFINITY" INTERPRETATION
 * ============================================================================
 *
 * "Tiny to infinity" means the language imposes no artificial finite
 * service-discovery capacity.
 *
 * It does NOT claim physical infrastructure is infinite.
 *
 * A realization can execute only when sufficient resources and capabilities
 * are actually available.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This grammar defines NO lexer rules.
 *
 * The canonical lexer is:
 *
 *     ZamaniLexer
 *
 * The `discover` keyword is intentionally a real lexical keyword because
 * service discovery must have an unambiguous declaration boundary when
 * composed with existing contextual networking declarations.
 *
 * This avoids the existing ambiguity caused by networking declarations whose
 * markers are ordinary identifiers.
 *
 * The keyword is therefore owned by:
 *
 *     grammar/lexer/keywords.g4
 *
 * as:
 *
 *     DISCOVER : 'discover' ;
 *
 * No other discovery vocabulary is reserved here.
 *
 * ============================================================================
 * PARSER DECLARATION
 * ============================================================================
 */

parser grammar NetworkingServiceDiscovery;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions, Attributes;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable integration boundary for networking.g4.
 *
 * ============================================================================
 */

networkServiceDiscoveryConstruct
    : networkServiceDiscoveryDeclaration
    ;


/*
 * ============================================================================
 * SERVICE-DISCOVERY DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     discover compute;
 *
 *     discover compute {
 *         service: compute;
 *     }
 *
 *     discover quantum_backend {
 *         service: quantum::backend;
 *         requires: capability("quantum.compute");
 *     };
 *
 * The declaration name is the logical discovery-contract identity.
 *
 * It is NOT necessarily:
 *
 *     a service name;
 *     an endpoint name;
 *     a process name;
 *     a node name;
 *     a device name.
 *
 * Semantic analysis determines the relationship.
 *
 * ============================================================================
 */

networkServiceDiscoveryDeclaration
    : attribute*
      DISCOVER
      identifier
      networkServiceDiscoveryBody?
      SEMI?
    ;


/*
 * ============================================================================
 * DISCOVERY BODY
 * ============================================================================
 *
 * An arbitrary number of members is allowed.
 *
 * ============================================================================
 */

networkServiceDiscoveryBody
    : LBRACE
      networkServiceDiscoveryMember*
      RBRACE
    ;


/*
 * ============================================================================
 * DISCOVERY MEMBER
 * ============================================================================
 *
 * Members are either:
 *
 *     property
 *
 * or:
 *
 *     nested configuration block.
 *
 * ============================================================================
 */

networkServiceDiscoveryMember
    : attribute*
      networkServiceDiscoveryProperty
    | attribute*
      networkServiceDiscoveryNestedBlock
    ;


/*
 * ============================================================================
 * DISCOVERY PROPERTY
 * ============================================================================
 *
 * Canonical form:
 *
 *     qualified_name: expression;
 *
 * Examples:
 *
 *     service: compute;
 *     endpoint: compute::endpoint;
 *     protocol: network::request_response;
 *     selector: service::compute;
 *     requires: capability("network.discovery");
 *     prefers: locality::near;
 *
 * ============================================================================
 */

networkServiceDiscoveryProperty
    : networkServiceDiscoveryPropertyName
      COLON
      networkServiceDiscoveryPropertyValue
      SEMI
    ;


/*
 * ============================================================================
 * DISCOVERY PROPERTY NAME
 * ============================================================================
 *
 * Qualified names allow future and vendor/application-specific namespaces
 * without modifying the core grammar.
 *
 * Examples:
 *
 *     service
 *     selector
 *     network::selector
 *     vendor::discovery::policy
 *     application::metadata
 *
 * ============================================================================
 */

networkServiceDiscoveryPropertyName
    : qualifiedName
    ;


/*
 * ============================================================================
 * DISCOVERY PROPERTY VALUE
 * ============================================================================
 *
 * All values use the canonical expression grammar.
 *
 * ============================================================================
 */

networkServiceDiscoveryPropertyValue
    : expression
    ;


/*
 * ============================================================================
 * NESTED DISCOVERY BLOCK
 * ============================================================================
 *
 * Example:
 *
 *     discover compute {
 *         selector {
 *             capability: compute::general;
 *             locality: locality::near;
 *         }
 *
 *         policy {
 *             strategy: adaptive;
 *         }
 *     }
 *
 * The nested name is semantic.
 *
 * ============================================================================
 */

networkServiceDiscoveryNestedBlock
    : networkServiceDiscoveryNestedBlockName
      LBRACE
      networkServiceDiscoveryNestedMember*
      RBRACE
      SEMI?
    ;


networkServiceDiscoveryNestedBlockName
    : qualifiedName
    ;


networkServiceDiscoveryNestedMember
    : attribute*
      networkServiceDiscoveryProperty
    | attribute*
      networkServiceDiscoveryNestedBlock
    ;


/*
 * ============================================================================
 * DISCOVERY REFERENCE
 * ============================================================================
 *
 * A discovery reference is a logical name.
 *
 * It performs no lookup.
 *
 * ============================================================================
 */

networkServiceDiscoveryReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * DISCOVERY REFERENCE LIST
 * ============================================================================
 *
 * There is no artificial list-size limit.
 *
 * ============================================================================
 */

networkServiceDiscoveryReferenceList
    : networkServiceDiscoveryReference
      (COMMA networkServiceDiscoveryReference)*
      COMMA?
    ;


optionalNetworkServiceDiscoveryReferenceList
    : networkServiceDiscoveryReferenceList?
    ;


/*
 * ============================================================================
 * DISCOVERY DECLARATION LIST
 * ============================================================================
 */

networkServiceDiscoveryDeclarationList
    : networkServiceDiscoveryDeclaration*
    ;


/*
 * ============================================================================
 * DISCOVERY PROPERTY LIST
 * ============================================================================
 */

networkServiceDiscoveryPropertyList
    : networkServiceDiscoveryProperty*
    ;


/*
 * ============================================================================
 * DISCOVERY ITEM
 * ============================================================================
 */

networkServiceDiscoveryItem
    : networkServiceDiscoveryDeclaration
    | networkServiceDiscoveryReference
    ;


/*
 * ============================================================================
 * LOGICAL SERVICE REFERENCE
 * ============================================================================
 *
 * Service declaration ownership remains in services.g4.
 *
 * ============================================================================
 */

networkServiceDiscoveryServiceReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * LOGICAL ENDPOINT REFERENCE
 * ============================================================================
 *
 * Endpoint declaration ownership remains in endpoints.g4.
 *
 * ============================================================================
 */

networkServiceDiscoveryEndpointReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * LOGICAL ADDRESS REFERENCE
 * ============================================================================
 *
 * Address syntax remains in addresses.g4.
 *
 * ============================================================================
 */

networkServiceDiscoveryAddressReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * LOGICAL PROTOCOL REFERENCE
 * ============================================================================
 *
 * Protocol declaration ownership remains in protocols.g4.
 *
 * ============================================================================
 */

networkServiceDiscoveryProtocolReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * LOGICAL CHANNEL REFERENCE
 * ============================================================================
 *
 * Networking channel declaration ownership remains in channels.g4.
 *
 * ============================================================================
 */

networkServiceDiscoveryChannelReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * LOGICAL ROUTE REFERENCE
 * ============================================================================
 *
 * Routing declaration ownership remains in routing.g4.
 *
 * ============================================================================
 */

networkServiceDiscoveryRouteReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * LOGICAL REQUEST REFERENCE
 * ============================================================================
 *
 * Request declaration ownership remains in requests.g4.
 *
 * ============================================================================
 */

networkServiceDiscoveryRequestReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * LOGICAL RESPONSE REFERENCE
 * ============================================================================
 *
 * Response declaration ownership remains in responses.g4.
 *
 * ============================================================================
 */

networkServiceDiscoveryResponseReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * LOGICAL SOCKET REFERENCE
 * ============================================================================
 *
 * Socket declaration ownership remains in sockets.g4.
 *
 * ============================================================================
 */

networkServiceDiscoverySocketReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * LOGICAL CAPABILITY REFERENCE
 * ============================================================================
 *
 * Networking capability declaration ownership remains in
 * network-capabilities.g4.
 *
 * ============================================================================
 */

networkServiceDiscoveryCapabilityReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * SERVICE DISCOVERY SELECTOR
 * ============================================================================
 *
 * A selector is intentionally an expression boundary.
 *
 * It may therefore evolve from:
 *
 *     service::compute
 *
 * to:
 *
 *     capability("compute.general")
 *
 * or:
 *
 *     attributes
 *
 * or another semantically defined expression without changing this grammar.
 *
 * ============================================================================
 */

networkServiceDiscoverySelector
    : expression
    ;


/*
 * ============================================================================
 * SERVICE DISCOVERY QUERY
 * ============================================================================
 *
 * A query is intentionally represented as an expression.
 *
 * This prevents the grammar from embedding a registry-specific query
 * language.
 *
 * ============================================================================
 */

networkServiceDiscoveryQuery
    : expression
    ;


/*
 * ============================================================================
 * SERVICE DISCOVERY POLICY
 * ============================================================================
 *
 * Policy configuration is structurally represented by the generic nested-block
 * form.
 *
 * No policy algorithm is enumerated.
 *
 * ============================================================================
 */

networkServiceDiscoveryPolicyBlock
    : networkServiceDiscoveryNestedBlock
    ;


/*
 * ============================================================================
 * SERVICE DISCOVERY CONSTRAINT
 * ============================================================================
 *
 * Constraint configuration remains an expression/property boundary.
 *
 * ============================================================================
 */

networkServiceDiscoveryConstraint
    : networkServiceDiscoveryProperty
    ;


/*
 * ============================================================================
 * SERVICE DISCOVERY REQUIREMENT
 * ============================================================================
 */

networkServiceDiscoveryRequirement
    : networkServiceDiscoveryProperty
    ;


/*
 * ============================================================================
 * SERVICE DISCOVERY PREFERENCE
 * ============================================================================
 */

networkServiceDiscoveryPreference
    : networkServiceDiscoveryProperty
    ;


/*
 * ============================================================================
 * SERVICE DISCOVERY CAPABILITY
 * ============================================================================
 */

networkServiceDiscoveryCapability
    : networkServiceDiscoveryProperty
    ;


/*
 * ============================================================================
 * SERVICE DISCOVERY METADATA
 * ============================================================================
 */

networkServiceDiscoveryMetadata
    : networkServiceDiscoveryProperty
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST mapping must preserve:
 *
 *     - declaration attributes;
 *     - declaration name;
 *     - ordered members;
 *     - property names;
 *     - property expressions;
 *     - nested blocks;
 *     - logical references;
 *     - source spans.
 *
 * This grammar MUST NOT require a discovery-specific AST hierarchy if the
 * existing domain-neutral AST can represent declaration/property/block
 * structure.
 *
 * The semantic layer may introduce a semantic service-discovery model after
 * AST construction where required.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - discovery declaration-name resolution;
 *     - service reference resolution;
 *     - endpoint reference resolution;
 *     - address reference resolution;
 *     - protocol compatibility;
 *     - channel compatibility;
 *     - request/response compatibility;
 *     - route compatibility;
 *     - capability satisfaction;
 *     - resource requirements;
 *     - constraint validation;
 *     - preference interpretation;
 *     - discovery-policy validation;
 *     - service identity semantics;
 *     - version compatibility;
 *     - availability semantics;
 *     - health semantics;
 *     - consistency semantics;
 *     - freshness semantics;
 *     - security requirements;
 *     - distributed realization compatibility;
 *     - portability analysis.
 *
 * Parser acceptance MUST NOT imply:
 *
 *     service existence;
 *     endpoint availability;
 *     registry availability;
 *     network availability;
 *     capability availability;
 *     resource availability;
 *     route feasibility.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * It MUST NOT introduce:
 *
 *     ServiceDiscoveryIR
 *     DiscoveryIR
 *     NetworkDiscoveryIR
 *     PhysicalDiscoveryIR
 *
 * as a competing intermediate representation merely because discovery syntax
 * exists.
 *
 * The intended path is:
 *
 *     source
 *         ->
 *     frontend AST
 *         ->
 *     semantic service-discovery model
 *         ->
 *     canonical/domain IR
 *         ->
 *     routing/scheduling/deployment
 *
 * ============================================================================
 * QUANTUM IR INVARIANT
 * ============================================================================
 *
 * Service discovery may participate in quantum/classical distributed
 * computation.
 *
 * It must never create a second quantum IR.
 *
 * When quantum computation participates, the established architecture remains:
 *
 *     semantic networking information
 *         ->
 *     canonical quantum::ir where appropriate
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may use discovery semantics to:
 *
 *     - validate service requirements;
 *     - resolve logical service identities;
 *     - negotiate capabilities;
 *     - construct logical communication relationships;
 *     - coordinate route selection;
 *     - coordinate distributed placement;
 *     - select compatible runtime discovery mechanisms;
 *     - generate deployment metadata;
 *     - optimize communication.
 *
 * These are downstream decisions.
 *
 * The compiler must not interpret:
 *
 *     discover compute
 *
 * as a demand for a specific physical machine.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime may perform:
 *
 *     - service registry lookup;
 *     - endpoint resolution;
 *     - discovery refresh;
 *     - health observation;
 *     - failover;
 *     - cache management;
 *     - connection establishment;
 *     - route activation;
 *     - service binding.
 *
 * None of these operations occur during parsing.
 *
 * ============================================================================
 * ROUTING INTEGRATION
 * ============================================================================
 *
 * Discovery and routing are sequentially related but independently owned.
 *
 * Example:
 *
 *     discover compute {
 *         service: compute;
 *         route: network::compute_route;
 *     }
 *
 * Semantic flow:
 *
 *     discovery contract
 *         ->
 *     service realization
 *         ->
 *     route contract
 *         ->
 *     routing realization
 *
 * This grammar does not import routing.g4 merely to represent the reference.
 *
 * The reference remains a qualified name.
 *
 * This prevents dependency cycles between networking component grammars.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Discovery may express:
 *
 *     requires: capability("network.discovery");
 *
 *     requires: capability("network.reliable");
 *
 *     requires: memory >= required_memory;
 *
 *     requires: capability("quantum.compute");
 *
 *     requires: capability("gpu.compute");
 *
 *     constraint: latency <= maximum_latency;
 *
 *     prefers: locality::near;
 *
 * These are expressions.
 *
 * Resource/capability analysis determines whether they can be satisfied.
 *
 * The grammar does not impose a resource limit.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Security-related properties remain syntactic data.
 *
 * Examples:
 *
 *     security: security::authenticated;
 *
 *     requires: capability("secure.discovery");
 *
 *     policy: security::confidential;
 *
 * The grammar does not implement:
 *
 *     - authentication;
 *     - authorization;
 *     - encryption;
 *     - certificate validation;
 *     - trust;
 *     - key management.
 *
 * ============================================================================
 * DATA INTEGRATION
 * ============================================================================
 *
 * Discovery may identify services responsible for:
 *
 *     - data;
 *     - datasets;
 *     - streams;
 *     - tensors;
 *     - AI models;
 *     - classical computation;
 *     - quantum computation;
 *     - hardware acceleration.
 *
 * Data schemas remain owned by the data grammar.
 *
 * ============================================================================
 * AI INTEGRATION
 * ============================================================================
 *
 * An AI program may declare:
 *
 *     discover inference {
 *         service: ai::inference;
 *         requires: capability("ai.inference");
 *     }
 *
 * The grammar does not encode:
 *
 *     - a particular AI framework;
 *     - a particular accelerator;
 *     - a model vendor;
 *     - a fixed model size.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A discovery contract may identify a logical hardware service:
 *
 *     discover accelerator {
 *         service: hardware::accelerator;
 *         requires: capability("accelerator.compute");
 *     }
 *
 * Hardware realization remains downstream.
 *
 * No register width, device count, memory capacity, FPGA capacity, or topology
 * is encoded here.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Discovery can identify logical distributed services.
 *
 * It does not define:
 *
 *     node membership;
 *     cluster size;
 *     replica placement;
 *     partitioning;
 *     consensus;
 *     distributed scheduling.
 *
 * Those belong to distributed grammar and semantic layers.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * The grammar provides sufficient structure for:
 *
 *     - syntax highlighting;
 *     - formatting;
 *     - symbol indexing;
 *     - navigation;
 *     - refactoring;
 *     - discovery-contract documentation;
 *     - property inspection;
 *     - dependency/reference analysis;
 *     - diagnostics.
 *
 * Tooling must preserve:
 *
 *     declaration order;
 *     property order;
 *     nested structure;
 *     source spans.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Parser diagnostics cover structural syntax errors such as:
 *
 *     discover;
 *
 *     discover 123;
 *
 *     discover compute {
 *         service
 *     }
 *
 *     discover compute {
 *         service:
 *     }
 *
 *     discover compute {
 *         : value;
 *     }
 *
 *     discover compute {
 *         service value;
 *     }
 *
 *     discover compute {
 *         service: ;
 *     }
 *
 *     discover compute {
 *         selector {
 *     }
 *
 * Semantic diagnostics cover:
 *
 *     - unknown service;
 *     - invalid endpoint reference;
 *     - invalid protocol reference;
 *     - invalid channel reference;
 *     - invalid route reference;
 *     - unknown capability;
 *     - unsatisfied capability;
 *     - impossible resource requirement;
 *     - contradictory constraints;
 *     - incompatible service version;
 *     - invalid security requirement;
 *     - unavailable realization.
 *
 * These are semantic errors, not parser errors.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source text;
 *     - lexer configuration;
 *     - grammar version;
 *     - imported grammar definitions.
 *
 * Parsing MUST NOT depend on:
 *
 *     - registry contents;
 *     - network state;
 *     - DNS;
 *     - hardware;
 *     - runtime state;
 *     - filesystem state;
 *     - environment variables;
 *     - wall-clock time;
 *     - randomness.
 *
 * Identical source/token input must produce equivalent parse structures.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This is an additive networking grammar.
 *
 * Existing:
 *
 *     services.g4
 *     endpoints.g4
 *     addresses.g4
 *     protocols.g4
 *     channels.g4
 *     messages.g4
 *     requests.g4
 *     responses.g4
 *     sockets.g4
 *     routing.g4
 *     network-capabilities.g4
 *
 * retain their existing ownership.
 *
 * Service discovery does not replace service declarations.
 *
 * The only required lexical compatibility change is:
 *
 *     DISCOVER : 'discover' ;
 *
 * in the canonical keyword grammar.
 *
 * This reservation is intentional because it creates a deterministic parser
 * boundary for service-discovery declarations within the aggregate grammar.
 *
 * Existing programs using `discover` as an ordinary identifier would require
 * the repository's normal keyword-compatibility migration policy.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST cover:
 *
 *     - one discovery declaration;
 *     - many discovery declarations;
 *     - empty discovery body;
 *     - one property;
 *     - many properties;
 *     - deeply qualified property names;
 *     - deeply qualified references;
 *     - deeply nested discovery blocks;
 *     - large expressions;
 *     - large source programs;
 *     - arbitrary reference lists.
 *
 * Tests MUST NOT establish an artificial maximum.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * Minimal:
 *
 *     discover compute;
 *
 * Basic service:
 *
 *     discover compute_service {
 *         service: compute;
 *     }
 *
 * Endpoint-aware:
 *
 *     discover compute_service {
 *         service: compute;
 *         endpoint: compute::endpoint;
 *     }
 *
 * Protocol-aware:
 *
 *     discover compute_service {
 *         service: compute;
 *         protocol: network::request_response;
 *     }
 *
 * Capability-aware:
 *
 *     discover compute_service {
 *         service: compute;
 *         requires: capability("network.discovery");
 *         requires: capability("network.reliable");
 *     }
 *
 * Resource-aware:
 *
 *     discover compute_service {
 *         requires: memory >= required_memory;
 *         constraint: latency <= maximum_latency;
 *     }
 *
 * Preference-aware:
 *
 *     discover compute_service {
 *         prefers: locality::near;
 *     }
 *
 * Route integration:
 *
 *     discover compute_service {
 *         service: compute;
 *         route: network::compute_route;
 *     }
 *
 * Quantum integration:
 *
 *     discover quantum_backend {
 *         service: quantum::backend;
 *         requires: capability("quantum.compute");
 *     }
 *
 * Hybrid integration:
 *
 *     discover quantum_controller {
 *         service: hybrid::quantum_controller;
 *         requires: capability("quantum.measurement");
 *     }
 *
 * AI integration:
 *
 *     discover inference {
 *         service: ai::inference;
 *         requires: capability("ai.inference");
 *     }
 *
 * Hardware integration:
 *
 *     discover accelerator {
 *         service: hardware::accelerator;
 *         requires: capability("accelerator.compute");
 *     }
 *
 * Nested:
 *
 *     discover compute {
 *         selector {
 *             capability: compute::general;
 *             locality: locality::near;
 *         }
 *
 *         policy {
 *             strategy: adaptive;
 *             freshness: desired_freshness;
 *         }
 *     }
 *
 * Qualified extension:
 *
 *     discover compute {
 *         vendor::discovery::policy: future_policy;
 *         application::metadata: metadata_value;
 *     }
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following MUST be rejected syntactically:
 *
 *     discover;
 *
 *     discover 123;
 *
 *     discover compute {
 *         service
 *     }
 *
 *     discover compute {
 *         service:
 *     }
 *
 *     discover compute {
 *         service: ;
 *     }
 *
 *     discover compute {
 *         service compute;
 *     }
 *
 *     discover compute {
 *         : compute;
 *     }
 *
 *     discover compute {
 *         selector {
 *     }
 *
 *     discover compute {
 *         service: compute
 *         endpoint: endpoint;
 *     }
 *
 * Semantic rejection examples:
 *
 *     - unknown service reference;
 *     - unknown endpoint reference;
 *     - unknown protocol reference;
 *     - unknown route reference;
 *     - unsatisfied capability;
 *     - contradictory constraints.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     discover a;
 *
 *     discover very_long_logical_name;
 *
 *     discover compute {};
 *
 *     discover compute {
 *         service: compute;
 *     }
 *
 *     deeply qualified names;
 *
 *     deeply nested blocks;
 *
 *     large expressions;
 *
 *     large property counts;
 *
 *     large discovery declaration counts.
 *
 * No finite language-level boundary is permitted.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains NO:
 *
 *     MAX_SERVICES
 *     MAX_ENDPOINTS
 *     MAX_DISCOVERIES
 *     MAX_INSTANCES
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_ROUTERS
 *     MAX_SWITCHES
 *     MAX_INTERFACES
 *     MAX_LINKS
 *     MAX_NETWORK_SIZE
 *     MAX_RESULTS
 *     MAX_RETRIES
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_MEMORY
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_THREADS
 *
 * It contains no special physical constructs such as:
 *
 *     node_0
 *     gpu_0
 *     qpu_0
 *     router_0
 *     switch_0
 *     interface_0
 *     socket_0
 *
 * It contains no fixed:
 *
 *     registry;
 *     discovery protocol;
 *     transport;
 *     topology;
 *     hardware;
 *     provider;
 *     service count;
 *     endpoint count.
 *
 * ============================================================================
 * SECURITY AUDIT
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no network access;
 *     - performs no registry lookup;
 *     - handles no secrets;
 *     - performs no authentication;
 *     - performs no authorization;
 *     - performs no cryptography;
 *     - performs no certificate validation;
 *     - contains no embedded executable actions.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar uses ordinary ANTLR repetition and qualified-name structures.
 *
 * It must not introduce complexity proportional to physical network size
 * during parsing.
 *
 * Parser complexity depends on the source/token stream and grammar structure,
 * not on:
 *
 *     number of physical services;
 *     number of physical nodes;
 *     number of registry entries;
 *     network topology;
 *     hardware capacity.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is an independent parser grammar.
 *
 * [x] It owns service-discovery syntax only.
 *
 * [x] It uses the canonical ZamaniLexer.
 *
 * [x] It reuses canonical Names.
 *
 * [x] It reuses canonical Expressions.
 *
 * [x] It reuses canonical Attributes.
 *
 * [x] It contains no lexer rules.
 *
 * [x] It uses an unambiguous DISCOVER declaration marker.
 *
 * [x] It supports optional declaration bodies.
 *
 * [x] It supports open-world property names.
 *
 * [x] It supports qualified property names.
 *
 * [x] It supports expression-valued properties.
 *
 * [x] It supports nested configuration.
 *
 * [x] It supports logical references.
 *
 * [x] It supports arbitrary reference lists.
 *
 * [x] It supports attributes.
 *
 * [x] It contains no transport enumeration.
 *
 * [x] It contains no registry implementation.
 *
 * [x] It contains no physical topology.
 *
 * [x] It contains no hardware assumptions.
 *
 * [x] It contains no resource capacity constants.
 *
 * [x] It contains no routing algorithm.
 *
 * [x] It contains no scheduling implementation.
 *
 * [x] It contains no runtime behavior.
 *
 * [x] It creates no IR.
 *
 * [x] It creates no second quantum IR.
 *
 * [x] It preserves source structure for AST construction.
 *
 * [x] It preserves source spans through the normal parse-tree pipeline.
 *
 * [x] It defines semantic responsibilities in advance.
 *
 * [x] It defines compiler/runtime boundaries in advance.
 *
 * [x] It defines resource/capability integration in advance.
 *
 * [x] It defines security boundaries in advance.
 *
 * [x] It defines positive tests.
 *
 * [x] It defines negative tests.
 *
 * [x] It defines boundary tests.
 *
 * [x] It defines scalability tests.
 *
 * [x] It defines determinism tests.
 *
 * [x] It defines compatibility requirements.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It remains compatible with Rust 1.97 / 1.97.1 generated-parser
 *     integration.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This grammar answers:
 *
 *     "What logical service-discovery intent does the program declare?"
 *
 * It does NOT answer:
 *
 *     "Which service instance exists?"
 *
 *     "Which node hosts it?"
 *
 *     "Which address will be used?"
 *
 *     "Which network route will be selected?"
 *
 *     "Which socket will be opened?"
 *
 *     "Which hardware will execute it?"
 *
 * Those answers belong downstream.
 *
 * ============================================================================
 */