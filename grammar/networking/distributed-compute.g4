/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/networking/distributed-compute.g4
 *
 * Grammar:
 *     NetworkingDistributedCompute
 *
 * Status:
 *     Production parser grammar contract
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     2021
 *
 * Safety:
 *     - Grammar only.
 *     - No embedded Rust.
 *     - No unsafe code.
 *     - No parser actions.
 *     - No semantic predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No randomness.
 *     - No environment inspection.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the NETWORKING-FACING SOURCE-LEVEL CONTRACT for
 * distributed computation.
 *
 * It provides a bridge between:
 *
 *     distributed computation
 *
 * and:
 *
 *     networking communication.
 *
 * It describes WHAT a distributed computation requires from communication
 * infrastructure without deciding HOW that communication is physically
 * realized.
 *
 * This grammar is intentionally narrower than:
 *
 *     grammar/distributed/
 *
 * The distributed grammar owns distributed execution semantics.
 *
 * This grammar owns only the networking-facing distributed-compute contract.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     networking
 *          |
 *          v
 *     NetworkingDistributedCompute
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> distributed analysis
 *          +--> networking analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> distributed semantics
 *          +--> networking semantics
 *          +--> classical semantics
 *          +--> quantum semantics
 *          +--> hardware semantics
 *          |
 *          v
 *     optimization
 *          |
 *          +--> partitioning
 *          +--> placement
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          |
 *          v
 *     runtime / HAL / target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - networking-facing distributed computation declarations;
 *     - logical distributed-computation identity;
 *     - communication-related distributed-computation properties;
 *     - network participation intent;
 *     - logical source/destination relationships;
 *     - route references expressed as properties;
 *     - channel references expressed as properties;
 *     - protocol references expressed as properties;
 *     - service references expressed as properties;
 *     - communication requirements;
 *     - communication constraints;
 *     - communication preferences;
 *     - communication capabilities;
 *     - networking-related distributed-compute metadata;
 *     - stable parser entry points for this networking contract.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - distributed node declarations;
 *     - distributed process declarations;
 *     - distributed actor declarations;
 *     - distributed placement algorithms;
 *     - distributed partitioning algorithms;
 *     - replication implementation;
 *     - consistency implementation;
 *     - distributed scheduling;
 *     - distributed fault-tolerance implementation;
 *     - deployment implementation;
 *     - network route calculation;
 *     - route discovery;
 *     - packet forwarding;
 *     - topology discovery;
 *     - transport implementation;
 *     - socket creation;
 *     - endpoint implementation;
 *     - address implementation;
 *     - protocol implementation;
 *     - channel implementation;
 *     - message implementation;
 *     - service implementation;
 *     - service discovery;
 *     - security implementation;
 *     - cryptographic implementation;
 *     - hardware allocation;
 *     - CPU selection;
 *     - GPU selection;
 *     - FPGA selection;
 *     - QPU selection;
 *     - physical-qubit allocation;
 *     - quantum routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * RELATIONSHIP TO grammar/distributed/
 * ============================================================================
 *
 * The repository already contains a separate distributed grammar domain:
 *
 *     grammar/distributed/
 *
 * Its composition root is:
 *
 *     grammar/distributed/distributed.g4
 *
 * That domain owns distributed-computation semantics such as:
 *
 *     nodes
 *     processes
 *     actors
 *     placement
 *     partitioning
 *     replication
 *     consistency
 *     topology
 *     deployment
 *     remote execution
 *     fault tolerance
 *     transactions
 *     collective computation
 *
 * This file MUST NOT recreate those declarations.
 *
 * Instead, a networking distributed-compute declaration may reference
 * distributed constructs through ordinary names and expressions.
 *
 * Example:
 *
 *     distributed_compute workload {
 *         computation: distributed::job;
 *         service: compute_service;
 *         channel: result_channel;
 *         route: network::compute_route;
 *     }
 *
 * Semantic analysis resolves those references.
 *
 * ============================================================================
 * RELATIONSHIP TO NETWORKING COMPONENTS
 * ============================================================================
 *
 * endpoints.g4
 *     Owns communication participants.
 *
 * addresses.g4
 *     Owns address syntax.
 *
 * protocols.g4
 *     Owns protocol declarations.
 *
 * channels.g4
 *     Owns networking channel declarations.
 *
 * messages.g4
 *     Owns networking message declarations.
 *
 * services.g4
 *     Owns networking service declarations.
 *
 * sockets.g4
 *     Owns logical socket contracts.
 *
 * requests.g4
 *     Owns reusable network request contracts.
 *
 * responses.g4
 *     Owns reusable network response contracts.
 *
 * routing.g4
 *     Owns logical route contracts.
 *
 * service-discovery.g4
 *     Owns service discovery intent.
 *
 * network-capabilities.g4
 *     Owns networking capability declarations.
 *
 * This grammar references those concepts but does not duplicate their syntax.
 *
 * ============================================================================
 * NETWORKING VS DISTRIBUTED COMPUTING
 * ============================================================================
 *
 * Networking answers:
 *
 *     "How does logical communication relate to the network?"
 *
 * Distributed computing answers:
 *
 *     "How is computation distributed?"
 *
 * This grammar provides only the bridge between those two semantic domains.
 *
 * It MUST NOT turn:
 *
 *     route
 *
 * into:
 *
 *     placement.
 *
 * It MUST NOT turn:
 *
 *     channel
 *
 * into:
 *
 *     distributed process.
 *
 * It MUST NOT turn:
 *
 *     service
 *
 * into:
 *
 *     physical machine.
 *
 * Those interpretations belong to semantic analysis and downstream
 * realization.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Distributed computation must support:
 *
 *     Program
 *         Once
 *          |
 *     Compile
 *         Once
 *          |
 *     Run
 *       Everywhere
 *          |
 *     Anywhere
 *          |
 *       Forever
 *
 * Therefore this grammar describes logical intent rather than a fixed
 * deployment.
 *
 * The same source-level contract may ultimately be realized using:
 *
 *     - one local execution resource;
 *     - multiple CPU cores;
 *     - multiple processes;
 *     - multiple machines;
 *     - edge resources;
 *     - cloud resources;
 *     - HPC resources;
 *     - heterogeneous accelerators;
 *     - GPU resources;
 *     - FPGA resources;
 *     - ASIC resources;
 *     - quantum resources;
 *     - distributed quantum resources;
 *     - hybrid quantum/classical resources;
 *     - future computational substrates.
 *
 * The source grammar does not select among these realizations.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * This grammar deliberately does NOT enumerate:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     MQTT
 *     gRPC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     Ethernet
 *     Wi-Fi
 *     cloud providers
 *     cluster vendors
 *     accelerator vendors
 *     quantum providers
 *
 * Such concepts remain:
 *
 *     identifiers;
 *     qualified names;
 *     protocol declarations;
 *     capability declarations;
 *     dialect-defined values;
 *     semantic registry entries;
 *     implementation metadata.
 *
 * A new networking or distributed technology therefore does not require a
 * new parser keyword merely because it did not exist when Zamani was designed.
 *
 * ============================================================================
 * DECLARATION MODEL
 * ============================================================================
 *
 * Canonical form:
 *
 *     distributed_compute workload {
 *         computation: distributed::job;
 *         source: producer;
 *         destination: consumer;
 *         service: compute_service;
 *         channel: result_channel;
 *         route: network::compute_route;
 *     }
 *
 * The declaration may also be empty:
 *
 *     distributed_compute workload;
 *
 * An empty declaration is syntactically valid.
 *
 * Semantic analysis determines whether an empty contract is meaningful in
 * the surrounding program.
 *
 * ============================================================================
 * OPEN-WORLD PROPERTY MODEL
 * ============================================================================
 *
 * Members intentionally use:
 *
 *     qualifiedName : expression ;
 *
 * rather than a closed list of property keywords.
 *
 * This allows properties such as:
 *
 *     computation
 *     source
 *     destination
 *     endpoint
 *     address
 *     service
 *     channel
 *     request
 *     response
 *     message
 *     protocol
 *     route
 *     topology
 *     placement
 *     partition
 *     replication
 *     consistency
 *     ordering
 *     delivery
 *     reliability
 *     latency
 *     bandwidth
 *     locality
 *     security
 *     requires
 *     constraint
 *     prefer
 *     preference
 *     capability
 *     resilience
 *     timeout
 *     retry
 *     checkpoint
 *     metadata
 *     provenance
 *
 * without turning each name into a permanent grammar keyword.
 *
 * Semantic analysis owns the meaning of each property.
 *
 * ============================================================================
 * REQUIREMENTS / CONSTRAINTS / PREFERENCES / CAPABILITIES
 * ============================================================================
 *
 * The grammar preserves these as ordinary expressions.
 *
 * Examples:
 *
 *     requires: capability("distributed.communication");
 *
 *     requires: capability("network.reliable");
 *
 *     constraint: latency <= maximum_latency;
 *
 *     constraint: bandwidth >= required_bandwidth;
 *
 *     prefer: locality::near;
 *
 *     capability: network::multicast;
 *
 * The parser does not evaluate these expressions.
 *
 * It does not determine whether a capability exists.
 *
 * It does not determine whether a resource is available.
 *
 * It does not select a target.
 *
 * ============================================================================
 * RESOURCE SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level finite limits for:
 *
 *     distributed computations;
 *     properties;
 *     nested blocks;
 *     references;
 *     endpoints;
 *     services;
 *     channels;
 *     messages;
 *     protocols;
 *     routes;
 *     participants;
 *     partitions;
 *     replicas;
 *     resources;
 *     capabilities;
 *     requirements;
 *     constraints;
 *     preferences;
 *     qualified-name depth.
 *
 * This grammar MUST NOT contain:
 *
 *     MAX_NODES
 *     MAX_PROCESSES
 *     MAX_WORKERS
 *     MAX_TASKS
 *     MAX_SERVICES
 *     MAX_CHANNELS
 *     MAX_MESSAGES
 *     MAX_ROUTES
 *     MAX_PARTITIONS
 *     MAX_REPLICAS
 *     MAX_DEVICES
 *     MAX_NETWORK_SIZE
 *     MAX_CONNECTIONS
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_BANDWIDTH
 *
 * Nor may it encode fixed identifiers such as:
 *
 *     node0
 *     node1
 *     cpu0
 *     gpu0
 *     fpga0
 *     qpu0
 *
 * as language-level resources.
 *
 * Practical limits come from available implementation, compiler, runtime,
 * deployment, and target resources.
 *
 * ============================================================================
 * "TINY TO INFINITY"
 * ============================================================================
 *
 * "Infinity" means:
 *
 *     no artificial finite language capacity.
 *
 * It does not mean:
 *
 *     infinite physical memory;
 *     infinite bandwidth;
 *     infinite processing power;
 *     infinite network capacity;
 *     infinite storage;
 *     infinite target resources.
 *
 * The semantic/resource system determines whether a realization is possible.
 *
 * ============================================================================
 * NO HARD-CODED PHYSICAL TOPOLOGY
 * ============================================================================
 *
 * This grammar must not require:
 *
 *     physical node IDs;
 *     router IDs;
 *     switch IDs;
 *     interface IDs;
 *     port IDs;
 *     link IDs;
 *     MAC addresses;
 *     fixed IP addresses;
 *     physical machine IDs;
 *     provider-specific identifiers.
 *
 * A source program may refer to a logical resource by name.
 *
 * Physical realization is downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Distributed computation may communicate with quantum computation.
 *
 * Examples:
 *
 *     distributed_compute quantum_job {
 *         computation: quantum::algorithm;
 *         service: quantum::service;
 *         route: quantum::network;
 *         requires: capability("quantum.compute");
 *     }
 *
 * This grammar does not define:
 *
 *     QubitId;
 *     PhysicalQubitId;
 *     quantum gates;
 *     native gate sets;
 *     QPU topology;
 *     calibration;
 *     pulses;
 *     QEC;
 *     ZQN.
 *
 * Quantum computation remains downstream through:
 *
 *     domain-neutral AST
 *          ->
 *     semantic analysis
 *          ->
 *     quantum::ir
 *
 * followed by:
 *
 *     optimization
 *     routing
 *     scheduling
 *     QEC / resilience
 *     ZQN
 *     HAL
 *     target realization
 *
 * ============================================================================
 * CLASSICAL / AI / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The computation referenced by this contract may denote:
 *
 *     classical computation;
 *     vector computation;
 *     tensor computation;
 *     AI/ML computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL/hardware computation;
 *     accelerator computation;
 *     future computation.
 *
 * This grammar does not create domain-specific copies of those computations.
 *
 * Semantic analysis resolves the referenced computation according to the
 * owning domain.
 *
 * ============================================================================
 * NETWORK ROUTING BOUNDARY
 * ============================================================================
 *
 * A property such as:
 *
 *     route: network::compute_route;
 *
 * is a logical reference.
 *
 * It does not select:
 *
 *     router;
 *     switch;
 *     interface;
 *     physical path;
 *     packet route;
 *     transport.
 *
 * Route realization belongs to:
 *
 *     grammar/networking/routing.g4
 *
 * and its downstream semantic/compiler/runtime layers.
 *
 * ============================================================================
 * CHANNEL BOUNDARY
 * ============================================================================
 *
 * A property such as:
 *
 *     channel: result_channel;
 *
 * references a logical networking channel.
 *
 * It does not create:
 *
 *     queue;
 *     buffer;
 *     socket;
 *     physical connection;
 *     transport stream.
 *
 * Those are downstream.
 *
 * ============================================================================
 * SERVICE BOUNDARY
 * ============================================================================
 *
 * A property such as:
 *
 *     service: compute_service;
 *
 * references a logical service contract.
 *
 * Service declaration remains owned by:
 *
 *     services.g4
 *
 * Service discovery remains owned by:
 *
 *     service-discovery.g4
 *
 * Physical service realization remains downstream.
 *
 * ============================================================================
 * MESSAGE / REQUEST / RESPONSE BOUNDARY
 * ============================================================================
 *
 * Message, request, and response contracts remain owned by:
 *
 *     messages.g4
 *     requests.g4
 *     responses.g4
 *
 * This grammar only references them through ordinary expressions.
 *
 * It does not duplicate their schemas.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Security-related properties may express intent:
 *
 *     security: secure;
 *
 *     requires: capability("network.secure");
 *
 *     constraint: trust >= required_trust;
 *
 * The grammar does not:
 *
 *     authenticate;
 *     authorize;
 *     encrypt;
 *     decrypt;
 *     generate keys;
 *     validate certificates;
 *     evaluate trust.
 *
 * Security semantics remain downstream.
 *
 * ============================================================================
 * RESILIENCE BOUNDARY
 * ============================================================================
 *
 * Distributed networking may express:
 *
 *     reliability: required_reliability;
 *
 *     resilience: policy;
 *
 *     retry: retry_policy;
 *
 *     checkpoint: checkpoint_policy;
 *
 * These are source-level declarations.
 *
 * The grammar does not execute:
 *
 *     retry;
 *     recovery;
 *     failover;
 *     rerouting;
 *     remapping;
 *     checkpointing.
 *
 * Runtime and resilience layers determine whether such policies can be
 * realized.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source text;
 *     selected grammar version;
 *     canonical lexical vocabulary;
 *     imported grammar definitions.
 *
 * Parsing must not depend on:
 *
 *     current time;
 *     random state;
 *     hardware;
 *     filesystem state;
 *     network state;
 *     target availability;
 *     environment variables;
 *     runtime state.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This file defines NO lexer rules.
 *
 * It consumes:
 *
 *     ZamaniLexer
 *
 * Contextual declaration marker:
 *
 *     distributed_compute
 *
 * remains an identifier-level spelling.
 *
 * This avoids creating a private keyword owned only by this grammar.
 *
 * Semantic analysis verifies the canonical contextual spelling when this rule
 * is used as a networking distributed-compute declaration.
 *
 * Property names are also identifiers / qualified names.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Direct grammar dependencies:
 *
 *     Names
 *     Expressions
 *     Attributes
 *
 * Names:
 *
 *     identifier
 *     qualifiedName
 *
 * Expressions:
 *
 *     expression
 *
 * Attributes:
 *
 *     attribute
 *
 * The grammar does not directly import:
 *
 *     distributed semantics;
 *     routing implementation;
 *     scheduling;
 *     hardware;
 *     quantum::ir;
 *     QEC;
 *     ZQN;
 *     HAL.
 *
 * Those remain downstream semantic/compiler dependencies.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The stable public entry point is:
 *
 *     networkingDistributedComputeConstruct
 *
 * This rule is consumed by:
 *
 *     grammar/networking/networking.g4
 *
 * No competing networking distributed-computation root should be introduced.
 *
 * ============================================================================
 */

parser grammar NetworkingDistributedCompute;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions, Attributes;


/*
 * ============================================================================
 * PUBLIC CONSTRUCT
 * ============================================================================
 *
 * Canonical forms:
 *
 *     distributed_compute workload;
 *
 *     distributed_compute workload {
 *         computation: distributed::job;
 *         source: producer;
 *         destination: consumer;
 *     }
 *
 * The declaration name identifies logical computation intent.
 *
 * It is NOT a machine identifier.
 *
 * It is NOT a node identifier.
 *
 * It is NOT a process identifier.
 *
 * It is NOT a physical network identifier.
 */
networkingDistributedComputeConstruct
    : networkingDistributedComputeDeclaration
    ;


/*
 * ============================================================================
 * DECLARATION
 * ============================================================================
 */

networkingDistributedComputeDeclaration
    : attribute*
      networkingDistributedComputeMarker
      identifier
      networkingDistributedComputeBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * CONTEXTUAL MARKER
 * ============================================================================
 *
 * The spelling `distributed_compute` remains an identifier rather than a
 * private lexer token.
 *
 * Semantic validation recognizes it in this declaration position.
 *
 * This preserves the single lexical-authority rule.
 */
networkingDistributedComputeMarker
    : identifier
    ;


/*
 * ============================================================================
 * BODY
 * ============================================================================
 *
 * Zero or more members are accepted.
 *
 * No finite member count is encoded.
 */
networkingDistributedComputeBody
    : LBRACE
      networkingDistributedComputeMember*
      RBRACE
    ;


/*
 * ============================================================================
 * MEMBER
 * ============================================================================
 *
 * Members are either:
 *
 *     - property assignments;
 *     - nested named blocks.
 *
 * Bare expressions are intentionally not accepted here.
 *
 * This prevents ordinary expressions from becoming ambiguous with
 * distributed-compute declarations and keeps the contract structurally
 * deterministic.
 */
networkingDistributedComputeMember
    : attribute*
      networkingDistributedComputeProperty
    | attribute*
      networkingDistributedComputeNestedBlock
    ;


/*
 * ============================================================================
 * PROPERTY
 * ============================================================================
 *
 * Canonical form:
 *
 *     name: expression;
 *
 * Examples:
 *
 *     computation: distributed::job;
 *     source: producer;
 *     destination: consumer;
 *     service: compute_service;
 *     channel: result_channel;
 *     route: network::compute_route;
 *     protocol: network::protocol;
 *     requires: capability("network.reliable");
 *     constraint: latency <= maximum_latency;
 *     prefer: locality::near;
 *
 * Property meaning is semantic.
 */
networkingDistributedComputeProperty
    : networkingDistributedComputePropertyName
      COLON
      networkingDistributedComputePropertyValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * PROPERTY NAME
 * ============================================================================
 *
 * Open-world qualified names permit future extensions without grammar
 * modification.
 *
 * Examples:
 *
 *     computation
 *     source
 *     network::source
 *     distributed::partition
 *     vendor::extension::property
 */
networkingDistributedComputePropertyName
    : qualifiedName
    ;


/*
 * ============================================================================
 * PROPERTY VALUE
 * ============================================================================
 *
 * The canonical expression grammar owns value syntax.
 */
networkingDistributedComputePropertyValue
    : expression
    ;


/*
 * ============================================================================
 * NESTED BLOCK
 * ============================================================================
 *
 * Example:
 *
 *     distributed_compute workload {
 *         communication {
 *             source: producer;
 *             destination: consumer;
 *             route: network::preferred;
 *         }
 *
 *         requirements {
 *             capability: network::reliable;
 *             latency: maximum_latency;
 *         }
 *     }
 *
 * Nested blocks remain semantically open.
 */
networkingDistributedComputeNestedBlock
    : networkingDistributedComputeNestedBlockName
      LBRACE
      networkingDistributedComputeNestedMember*
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * NESTED BLOCK NAME
 * ============================================================================
 */

networkingDistributedComputeNestedBlockName
    : qualifiedName
    ;


/*
 * ============================================================================
 * NESTED MEMBER
 * ============================================================================
 */

networkingDistributedComputeNestedMember
    : attribute*
      networkingDistributedComputeProperty
    | attribute*
      networkingDistributedComputeNestedBlock
    ;


/*
 * ============================================================================
 * STABLE REFERENCE
 * ============================================================================
 *
 * This rule does not perform symbol lookup.
 *
 * It is provided as a stable grammar boundary for downstream grammar
 * components that need to reference a networking distributed-compute
 * declaration.
 */
networkingDistributedComputeReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * REFERENCE LIST
 * ============================================================================
 *
 * No finite list length is imposed.
 */
networkingDistributedComputeReferenceList
    : networkingDistributedComputeReference
      (COMMA networkingDistributedComputeReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * OPTIONAL REFERENCE LIST
 * ============================================================================
 */

optionalNetworkingDistributedComputeReferenceList
    : networkingDistributedComputeReferenceList?
    ;


/*
 * ============================================================================
 * DECLARATION LIST
 * ============================================================================
 */

networkingDistributedComputeDeclarationList
    : networkingDistributedComputeDeclaration*
    ;


/*
 * ============================================================================
 * PROPERTY LIST
 * ============================================================================
 */

networkingDistributedComputePropertyList
    : networkingDistributedComputeProperty*
    ;


/*
 * ============================================================================
 * SEMANTIC PROPERTY ADAPTERS
 * ============================================================================
 *
 * These rules are naming adapters only.
 *
 * They do not introduce alternate syntax.
 *
 * They exist so AST builders, documentation tooling, semantic analyzers, and
 * future compatibility tooling can refer to stable conceptual categories
 * without creating separate grammar implementations.
 *
 * Semantic analysis MUST verify the property name before assigning one of
 * these conceptual meanings.
 */


/*
 * COMPUTATION
 */
networkingDistributedComputeComputationProperty
    : networkingDistributedComputeProperty
    ;


/*
 * SOURCE
 */
networkingDistributedComputeSourceProperty
    : networkingDistributedComputeProperty
    ;


/*
 * DESTINATION
 */
networkingDistributedComputeDestinationProperty
    : networkingDistributedComputeProperty
    ;


/*
 * ENDPOINT
 */
networkingDistributedComputeEndpointProperty
    : networkingDistributedComputeProperty
    ;


/*
 * ADDRESS
 */
networkingDistributedComputeAddressProperty
    : networkingDistributedComputeProperty
    ;


/*
 * SERVICE
 */
networkingDistributedComputeServiceProperty
    : networkingDistributedComputeProperty
    ;


/*
 * CHANNEL
 */
networkingDistributedComputeChannelProperty
    : networkingDistributedComputeProperty
    ;


/*
 * MESSAGE
 */
networkingDistributedComputeMessageProperty
    : networkingDistributedComputeProperty
    ;


/*
 * REQUEST
 */
networkingDistributedComputeRequestProperty
    : networkingDistributedComputeProperty
    ;


/*
 * RESPONSE
 */
networkingDistributedComputeResponseProperty
    : networkingDistributedComputeProperty
    ;


/*
 * PROTOCOL
 */
networkingDistributedComputeProtocolProperty
    : networkingDistributedComputeProperty
    ;


/*
 * ROUTE
 */
networkingDistributedComputeRouteProperty
    : networkingDistributedComputeProperty
    ;


/*
 * TOPOLOGY
 */
networkingDistributedComputeTopologyProperty
    : networkingDistributedComputeProperty
    ;


/*
 * PLACEMENT
 */
networkingDistributedComputePlacementProperty
    : networkingDistributedComputeProperty
    ;


/*
 * PARTITIONING
 */
networkingDistributedComputePartitionProperty
    : networkingDistributedComputeProperty
    ;


/*
 * REPLICATION
 */
networkingDistributedComputeReplicationProperty
    : networkingDistributedComputeProperty
    ;


/*
 * CONSISTENCY
 */
networkingDistributedComputeConsistencyProperty
    : networkingDistributedComputeProperty
    ;


/*
 * ORDERING
 */
networkingDistributedComputeOrderingProperty
    : networkingDistributedComputeProperty
    ;


/*
 * DELIVERY
 */
networkingDistributedComputeDeliveryProperty
    : networkingDistributedComputeProperty
    ;


/*
 * RELIABILITY
 */
networkingDistributedComputeReliabilityProperty
    : networkingDistributedComputeProperty
    ;


/*
 * LATENCY
 */
networkingDistributedComputeLatencyProperty
    : networkingDistributedComputeProperty
    ;


/*
 * BANDWIDTH
 */
networkingDistributedComputeBandwidthProperty
    : networkingDistributedComputeProperty
    ;


/*
 * LOCALITY
 */
networkingDistributedComputeLocalityProperty
    : networkingDistributedComputeProperty
    ;


/*
 * SECURITY
 */
networkingDistributedComputeSecurityProperty
    : networkingDistributedComputeProperty
    ;


/*
 * REQUIREMENT
 */
networkingDistributedComputeRequirementProperty
    : networkingDistributedComputeProperty
    ;


/*
 * CONSTRAINT
 */
networkingDistributedComputeConstraintProperty
    : networkingDistributedComputeProperty
    ;


/*
 * PREFERENCE
 */
networkingDistributedComputePreferenceProperty
    : networkingDistributedComputeProperty
    ;


/*
 * CAPABILITY
 */
networkingDistributedComputeCapabilityProperty
    : networkingDistributedComputeProperty
    ;


/*
 * RESILIENCE
 */
networkingDistributedComputeResilienceProperty
    : networkingDistributedComputeProperty
    ;


/*
 * TIMEOUT
 */
networkingDistributedComputeTimeoutProperty
    : networkingDistributedComputeProperty
    ;


/*
 * RETRY POLICY
 */
networkingDistributedComputeRetryProperty
    : networkingDistributedComputeProperty
    ;


/*
 * CHECKPOINT POLICY
 */
networkingDistributedComputeCheckpointProperty
    : networkingDistributedComputeProperty
    ;


/*
 * METADATA
 */
networkingDistributedComputeMetadataProperty
    : networkingDistributedComputeProperty
    ;


/*
 * PROVENANCE
 */
networkingDistributedComputeProvenanceProperty
    : networkingDistributedComputeProperty
    ;


/*
 * ============================================================================
 * SEMANTIC REFERENCE ADAPTERS
 * ============================================================================
 *
 * These remain syntactic qualified-name references.
 *
 * They do not perform resolution.
 */


/*
 * Logical computation reference.
 */
networkingDistributedComputeComputationReference
    : qualifiedName
    ;


/*
 * Logical endpoint reference.
 */
networkingDistributedComputeEndpointReference
    : qualifiedName
    ;


/*
 * Logical service reference.
 */
networkingDistributedComputeServiceReference
    : qualifiedName
    ;


/*
 * Logical channel reference.
 */
networkingDistributedComputeChannelReference
    : qualifiedName
    ;


/*
 * Logical message reference.
 */
networkingDistributedComputeMessageReference
    : qualifiedName
    ;


/*
 * Logical request reference.
 */
networkingDistributedComputeRequestReference
    : qualifiedName
    ;


/*
 * Logical response reference.
 */
networkingDistributedComputeResponseReference
    : qualifiedName
    ;


/*
 * Logical protocol reference.
 */
networkingDistributedComputeProtocolReference
    : qualifiedName
    ;


/*
 * Logical route reference.
 *
 * Actual route syntax and route semantics remain owned by routing.g4.
 */
networkingDistributedComputeRouteReference
    : qualifiedName
    ;


/*
 * Logical topology reference.
 */
networkingDistributedComputeTopologyReference
    : qualifiedName
    ;


/*
 * Logical distributed-computation reference.
 *
 * Resolution belongs to the distributed semantic layer.
 */
networkingDistributedComputeDistributedReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [x] It has exactly one networking-facing distributed-compute declaration
 *     entry point.
 *
 * [x] It has a stable declaration rule.
 *
 * [x] It has a stable reference rule.
 *
 * [x] It uses the canonical ZamaniLexer.
 *
 * [x] It reuses canonical Names.
 *
 * [x] It reuses canonical Expressions.
 *
 * [x] It reuses canonical Attributes.
 *
 * [x] It does not define lexer rules.
 *
 * [x] It does not create a private keyword.
 *
 * [x] It does not define a second expression grammar.
 *
 * [x] It does not define a second type grammar.
 *
 * [x] It does not duplicate distributed node/process/actor syntax.
 *
 * [x] It does not duplicate routing syntax.
 *
 * [x] It does not duplicate endpoint syntax.
 *
 * [x] It does not duplicate address syntax.
 *
 * [x] It does not duplicate protocol syntax.
 *
 * [x] It does not duplicate channel syntax.
 *
 * [x] It does not duplicate message syntax.
 *
 * [x] It does not duplicate service syntax.
 *
 * [x] It does not duplicate request syntax.
 *
 * [x] It does not duplicate response syntax.
 *
 * [x] It does not implement service discovery.
 *
 * [x] It does not implement route discovery.
 *
 * [x] It does not implement scheduling.
 *
 * [x] It does not implement placement.
 *
 * [x] It does not implement partitioning.
 *
 * [x] It does not implement replication.
 *
 * [x] It does not implement consistency.
 *
 * [x] It does not implement fault tolerance.
 *
 * [x] It does not implement security.
 *
 * [x] It does not implement hardware allocation.
 *
 * [x] It does not implement quantum routing.
 *
 * [x] It does not implement QEC.
 *
 * [x] It does not implement ZQN.
 *
 * [x] It does not create IR.
 *
 * [x] It does not create a distributed IR.
 *
 * [x] It does not create a networking IR.
 *
 * [x] It preserves the canonical semantic/IR pipeline.
 *
 * [x] Quantum computation remains compatible with quantum::ir downstream.
 *
 * [x] It contains no machine-size limits.
 *
 * [x] It contains no network-size limits.
 *
 * [x] It contains no node-count limits.
 *
 * [x] It contains no process-count limits.
 *
 * [x] It contains no channel-count limits.
 *
 * [x] It contains no endpoint-count limits.
 *
 * [x] It contains no bandwidth limits.
 *
 * [x] It contains no latency limits.
 *
 * [x] It contains no topology limits.
 *
 * [x] It contains no vendor transport enumeration.
 *
 * [x] It contains no physical addresses.
 *
 * [x] It contains no physical machine identifiers.
 *
 * [x] It contains no parser actions.
 *
 * [x] It contains no semantic predicates.
 *
 * [x] It contains no Rust.
 *
 * [x] It contains no unsafe code.
 *
 * [x] It uses unbounded ANTLR repetition where repetition is required.
 *
 * [x] It is deterministic with respect to its token stream and grammar.
 *
 * [x] Source spans remain recoverable from the ANTLR parse tree.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should represent this declaration as a domain-neutral
 * declaration/contract node containing:
 *
 *     - source span;
 *     - declaration name;
 *     - attributes;
 *     - ordered members;
 *     - nested blocks;
 *     - property names;
 *     - property expressions.
 *
 * The AST MUST NOT encode:
 *
 *     physical node;
 *     physical route;
 *     physical network interface;
 *     physical device;
 *     transport implementation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     - resolve the declaration name;
 *     - resolve referenced computations;
 *     - resolve endpoints;
 *     - resolve services;
 *     - resolve channels;
 *     - resolve protocols;
 *     - resolve messages;
 *     - resolve requests/responses;
 *     - resolve route references;
 *     - validate cross-domain references;
 *     - classify requirements;
 *     - classify constraints;
 *     - classify preferences;
 *     - classify capabilities;
 *     - validate types;
 *     - validate effects;
 *     - validate security requirements;
 *     - validate portability;
 *     - validate distributed/networking relationships.
 *
 * It must NOT infer physical target identity merely from syntax.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * The resulting semantic information may contribute to:
 *
 *     - distributed semantic representation;
 *     - networking semantic representation;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL/hardware representation;
 *
 * according to the referenced computation.
 *
 * The networking grammar itself must never create:
 *
 *     DistributedIR
 *     NetworkingIR
 *     RouteIR
 *     PhysicalNetworkIR
 *
 * merely because a declaration appears here.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler may use this declaration to derive:
 *
 *     - communication requirements;
 *     - route requirements;
 *     - capability requirements;
 *     - placement inputs;
 *     - scheduling inputs;
 *     - partitioning inputs;
 *     - resilience inputs;
 *     - deployment metadata.
 *
 * Compiler stages remain responsible for selecting realizations.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime systems may consume the semantic result to establish communication
 * between distributed computation components.
 *
 * This grammar itself performs no runtime operation.
 *
 * ============================================================================
 * TOOLING INTEGRATION
 * ============================================================================
 *
 * Tooling may use:
 *
 *     networkingDistributedComputeConstruct
 *
 * for:
 *
 *     - syntax highlighting;
 *     - documentation generation;
 *     - source indexing;
 *     - navigation;
 *     - reference analysis;
 *     - diagnostics;
 *     - semantic visualization.
 *
 * Tooling must not infer physical realization from this parse rule alone.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Classical:
 *
 *     computation: classical::program;
 *
 * Quantum:
 *
 *     computation: quantum::algorithm;
 *
 * Hybrid:
 *
 *     computation: hybrid::workflow;
 *
 * AI:
 *
 *     computation: ai::training;
 *
 * HDL/hardware:
 *
 *     computation: hardware::accelerator;
 *
 * Distributed:
 *
 *     computation: distributed::workload;
 *
 * Future domains may be represented through qualified names and semantic
 * extension mechanisms without changing this structural grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     distributed_compute workload;
 *
 *     distributed_compute workload {
 *         computation: distributed::job;
 *     }
 *
 *     distributed_compute workload {
 *         source: producer;
 *         destination: consumer;
 *         service: compute_service;
 *         channel: result_channel;
 *         route: network::compute_route;
 *     }
 *
 *     distributed_compute quantum_job {
 *         computation: quantum::algorithm;
 *         requires: capability("quantum.compute");
 *         route: quantum::network;
 *     }
 *
 *     distributed_compute hybrid_job {
 *         computation: hybrid::workflow;
 *         service: hybrid::compute;
 *         channel: results;
 *     }
 *
 *     distributed_compute workload {
 *         communication {
 *             source: producer;
 *             destination: consumer;
 *             route: network::preferred;
 *         }
 *     }
 *
 *     distributed_compute workload {
 *         requirements {
 *             capability: capability("network.reliable");
 *             latency: maximum_latency;
 *             bandwidth: required_bandwidth;
 *         }
 *     }
 *
 * NEGATIVE:
 *
 *     distributed_compute;
 *
 *     distributed_compute workload {
 *         source producer;
 *     }
 *
 *     distributed_compute workload {
 *         source:;
 *     }
 *
 *     distributed_compute workload {
 *         source: producer
 *         destination: consumer;
 *     }
 *
 *     distributed_compute workload {
 *         communication {
 *             source: producer;
 *     }
 *
 *     distributed_compute workload {
 *         : invalid;
 *     }
 *
 * BOUNDARY:
 *
 *     distributed_compute x;
 *
 *     distributed_compute x {};
 *
 *     distributed_compute x {
 *         a: b;
 *     }
 *
 *     distributed_compute x {
 *         a::b::c: value;
 *     }
 *
 *     distributed_compute x {
 *         property: [arbitrarily large expression/value];
 *     }
 *
 * SCALABILITY:
 *
 *     - arbitrarily many declarations;
 *     - arbitrarily many members;
 *     - arbitrarily many nested members;
 *     - arbitrarily deep qualified names;
 *     - arbitrarily many references;
 *     - large expressions;
 *     - large source files;
 *     - tiny source files.
 *
 * DETERMINISM:
 *
 *     Identical token streams must produce structurally identical parse trees.
 *
 * COMPATIBILITY:
 *
 *     Verify compatibility with:
 *
 *         networking.g4
 *         distributed/distributed.g4
 *         distributed/remote-execution.g4
 *         networking/routing.g4
 *         networking/channels.g4
 *         networking/services.g4
 *         networking/requests.g4
 *         networking/responses.g4
 *         networking/service-discovery.g4
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * No universal hardware or networking capacity is encoded.
 *
 * No fixed:
 *
 *     node count;
 *     process count;
 *     endpoint count;
 *     service count;
 *     channel count;
 *     route count;
 *     message count;
 *     protocol count;
 *     device count;
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     QPU count;
 *     memory capacity;
 *     bandwidth;
 *     topology size
 *
 * is imposed by this grammar.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics are structural only.
 *
 * Examples:
 *
 *     missing declaration name;
 *     malformed property;
 *     missing colon;
 *     missing expression;
 *     malformed nested block.
 *
 * Semantic diagnostics remain downstream.
 *
 * Examples:
 *
 *     unknown service;
 *     unknown endpoint;
 *     unknown route;
 *     unsatisfied capability;
 *     incompatible protocol;
 *     invalid distributed computation;
 *     unsupported target.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing must not:
 *
 *     execute network operations;
 *     resolve external addresses;
 *     contact services;
 *     access credentials;
 *     perform authentication;
 *     perform authorization;
 *     access secrets;
 *     inspect target infrastructure.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * The grammar is structurally linear over the number of members for ordinary
 * contract parsing.
 *
 * Nested blocks follow source nesting depth.
 *
 * No grammar-level resource bound is introduced.
 *
 * Any parser implementation stack/resource limitation is an implementation
 * concern rather than a language semantic limit.
 *
 * ============================================================================
 */