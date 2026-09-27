/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/networking/networking.g4
 *
 * Grammar:
 *     Networking
 *
 * Status:
 *     Production networking-domain composition grammar
 *
 * Purpose:
 *     Provide the single canonical parser-composition boundary for every
 *     networking construct currently owned by grammar/networking/.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     ANTLR4
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
 *     - No environment inspection.
 *     - No runtime callbacks.
 *     - No randomness.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file is an AGGREGATE / COMPOSITION grammar.
 *
 * It owns:
 *
 *     - the networking-domain composition boundary;
 *     - networking construct dispatch;
 *     - stable networking parser entry points;
 *     - integration with ZamaniParser.g4;
 *     - aggregation of every networking component grammar;
 *     - networking-domain classification at the parser boundary.
 *
 * It does NOT own the implementation of:
 *
 *     - endpoints;
 *     - addresses;
 *     - channels;
 *     - messages;
 *     - protocols;
 *     - requests;
 *     - responses;
 *     - routes;
 *     - service discovery;
 *     - services;
 *     - sockets;
 *     - streams;
 *     - distributed computation;
 *     - network capabilities.
 *
 * Those constructs remain owned by their existing component files.
 *
 * ============================================================================
 * CANONICAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     grammar/antlr/ZamaniParser.g4
 *          |
 *          v
 *     Networking
 *          |
 *          +--> Addresses
 *          +--> Endpoints
 *          +--> Channels
 *          +--> Messages
 *          +--> Protocols
 *          +--> Requests
 *          +--> Responses
 *          +--> Routes
 *          +--> Service Discovery
 *          +--> Services
 *          +--> Sockets
 *          +--> Streaming
 *          +--> Distributed Compute
 *          +--> Network Capabilities
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> distributed analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> networking semantics
 *          +--> distributed semantics
 *          +--> classical semantics
 *          +--> hybrid semantics
 *          +--> hardware communication intent
 *          +--> quantum-related metadata where applicable
 *          |
 *          v
 *     canonical IR / domain IR
 *          |
 *          v
 *     optimization
 *          |
 *          +--> routing
 *          +--> scheduling
 *          +--> placement
 *          +--> resilience
 *          +--> deployment
 *          |
 *          v
 *     runtime / HAL / target realization
 *
 * The grammar NEVER constructs IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Networking syntax describes logical communication intent.
 *
 * It MUST NOT impose artificial language-level limits on:
 *
 *     - endpoints;
 *     - addresses;
 *     - channels;
 *     - messages;
 *     - protocols;
 *     - requests;
 *     - responses;
 *     - routes;
 *     - services;
 *     - sockets;
 *     - streams;
 *     - distributed computations;
 *     - nodes;
 *     - devices;
 *     - network topology;
 *     - bandwidth;
 *     - latency;
 *     - connections;
 *     - providers;
 *     - transports.
 *
 * There are deliberately no:
 *
 *     MAX_ENDPOINTS
 *     MAX_ADDRESSES
 *     MAX_CHANNELS
 *     MAX_MESSAGES
 *     MAX_PROTOCOLS
 *     MAX_REQUESTS
 *     MAX_RESPONSES
 *     MAX_ROUTES
 *     MAX_SERVICES
 *     MAX_SOCKETS
 *     MAX_STREAMS
 *     MAX_CONNECTIONS
 *     MAX_NODES
 *     MAX_NETWORK_SIZE
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *
 * in this grammar.
 *
 * Repetition is expressed using ANTLR repetition operators.
 *
 * Actual resource limitations belong to:
 *
 *     - compiler implementation;
 *     - resource analysis;
 *     - capability analysis;
 *     - runtime;
 *     - deployment;
 *     - operating-system resources;
 *     - target hardware;
 *
 * Those limitations MUST NOT become universal grammar ceilings.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * A logical networking construct is not inherently:
 *
 *     - a machine;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a process;
 *     - a thread;
 *     - an operating-system socket;
 *     - a NIC;
 *     - an IP address;
 *     - a router;
 *     - a switch;
 *     - a cloud instance;
 *     - a physical node.
 *
 * A logical construct MAY be realized by any suitable implementation
 * satisfying its semantic requirements.
 *
 * This is required for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * (POCO-REAF).
 *
 * ============================================================================
 * OPEN-WORLD NETWORKING
 * ============================================================================
 *
 * The aggregate grammar deliberately does NOT enumerate:
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
 *     vendor transports
 *     cloud providers
 *     network devices
 *     hardware vendors
 *
 * as a closed universal grammar list.
 *
 * Such identities belong to the canonical name/expression/capability system
 * and are resolved semantically.
 *
 * A future transport MUST therefore be representable without requiring the
 * core aggregate grammar to gain a new transport keyword merely because the
 * technology is new.
 *
 * ============================================================================
 * DOMAIN OWNERSHIP
 * ============================================================================
 *
 * Networking owns communication intent.
 *
 * Distributed computing owns distributed execution semantics.
 *
 * Hardware owns physical hardware intent and capabilities.
 *
 * Security owns security semantics and implementation.
 *
 * Concurrency owns language-level concurrency primitives.
 *
 * Data owns data schemas and transformations.
 *
 * Quantum owns quantum semantics and the canonical quantum::ir boundary.
 *
 * The aggregate grammar MUST NOT duplicate those domains.
 *
 * ============================================================================
 * COMPONENT GRAMMAR INVENTORY
 * ============================================================================
 *
 * Existing files intentionally retained:
 *
 *     grammar/networking/addresses.g4
 *     grammar/networking/endpoints.g4
 *     grammar/networking/channels.g4
 *     grammar/networking/messages.g4
 *     grammar/networking/protocols.g4
 *     grammar/networking/requests.g4
 *     grammar/networking/responses.g4
 *     grammar/networking/routing.g4
 *     grammar/networking/service-discovery.g4
 *     grammar/networking/services.g4
 *     grammar/networking/sockets.g4
 *     grammar/networking/streaming.g4
 *     grammar/networking/distributed-compute.g4
 *     grammar/networking/network-capabilities.g4
 *
 * No existing component is renamed.
 *
 * ============================================================================
 * IMPORT MODEL
 * ============================================================================
 *
 * Each component grammar remains independently maintainable.
 *
 * This aggregate imports every networking component exactly once.
 *
 * The wider parser imports ONLY this aggregate networking grammar.
 *
 * Therefore the dependency direction is:
 *
 *     component grammar
 *          |
 *          v
 *     Networking
 *          |
 *          v
 *     ZamaniParser
 *
 * The reverse direction is forbidden.
 *
 * Networking components MUST NOT import:
 *
 *     ZamaniParser
 *     Zamani
 *     runtime grammars
 *     IR grammars
 *     hardware implementation grammars
 *
 * ============================================================================
 */

grammar Networking;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * NETWORKING COMPONENT IMPORTS
 * ============================================================================
 *
 * These are the complete networking-domain component boundaries currently
 * present in the repository.
 *
 * General lexical/name/type/expression ownership remains delegated to the
 * canonical component grammars.
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
    NetworkCapabilities
    ;


/*
 * ============================================================================
 * PUBLIC NETWORKING UNIT
 * ============================================================================
 *
 * This entry point is useful when the networking grammar is generated or
 * validated independently.
 *
 * It is intentionally unbounded at the language level.
 *
 * EOF guarantees that successful parsing represents the complete supplied
 * networking input rather than an accepted prefix.
 * ============================================================================
 */

networkingUnit
    : networkingConstruct* EOF
    ;


/*
 * ============================================================================
 * PUBLIC NETWORKING CONSTRUCT
 * ============================================================================
 *
 * Every networking component enters through this one aggregate dispatch rule.
 *
 * The alternatives are intentionally references to component-owned public
 * rules. Their implementations remain in the component grammars.
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
    ;


/*
 * ============================================================================
 * CANONICAL ZAMANI PARSER INTEGRATION
 * ============================================================================
 *
 * grammar/antlr/ZamaniParser.g4 currently consumes:
 *
 *     networkingElement
 *     universalNetworking
 *
 * and therefore requires the networking domain to expose a stable
 * `networkingDeclaration` rule.
 *
 * This rule is the official adapter between the networking aggregate and the
 * universal parser.
 *
 * No second networking declaration syntax is introduced.
 * ============================================================================
 */

networkingDeclaration
    : networkingConstruct
    ;


/*
 * ============================================================================
 * DOMAIN ELEMENT INTEGRATION
 * ============================================================================
 *
 * Networking source-level domain constructs are declarations/contracts.
 *
 * Networking does not currently introduce an independent standalone
 * expression grammar or statement grammar.
 *
 * Ordinary Zamani expressions and statements remain owned by:
 *
 *     grammar/expressions/
 *     grammar/statements/
 *
 * Networking operations are represented through the networking declarations,
 * request/response contracts, protocol flows, service contracts, stream
 * contracts, and their nested canonical expressions.
 *
 * ============================================================================
 */

networkingElement
    : networkingDeclaration
    ;


/*
 * ============================================================================
 * STABLE COMPONENT ADAPTERS
 * ============================================================================
 *
 * These rules are intentionally thin.
 *
 * They provide stable names at the aggregate boundary without copying or
 * redefining component grammar implementations.
 * ============================================================================
 */


/*
 * --------------------------------------------------------------------------
 * ADDRESS
 * --------------------------------------------------------------------------
 */

networkingAddress
    : addressConstruct
    ;


/*
 * --------------------------------------------------------------------------
 * ENDPOINT
 * --------------------------------------------------------------------------
 */

networkingEndpoint
    : endpointDeclaration
    ;


/*
 * --------------------------------------------------------------------------
 * CHANNEL
 * --------------------------------------------------------------------------
 */

networkingChannel
    : networkChannelConstruct
    ;


/*
 * --------------------------------------------------------------------------
 * MESSAGE
 * --------------------------------------------------------------------------
 */

networkingMessage
    : messageConstruct
    ;


/*
 * --------------------------------------------------------------------------
 * PROTOCOL
 * --------------------------------------------------------------------------
 */

networkingProtocol
    : protocolConstruct
    ;


/*
 * --------------------------------------------------------------------------
 * REQUEST
 * --------------------------------------------------------------------------
 */

networkingRequest
    : networkRequestConstruct
    ;


/*
 * --------------------------------------------------------------------------
 * RESPONSE
 * --------------------------------------------------------------------------
 */

networkingResponse
    : networkResponseConstruct
    ;


/*
 * --------------------------------------------------------------------------
 * ROUTE
 * --------------------------------------------------------------------------
 */

networkingRoute
    : networkRouteConstruct
    ;


/*
 * --------------------------------------------------------------------------
 * SERVICE DISCOVERY
 * --------------------------------------------------------------------------
 */

networkingServiceDiscovery
    : networkServiceDiscoveryConstruct
    ;


/*
 * --------------------------------------------------------------------------
 * SERVICE
 * --------------------------------------------------------------------------
 */

networkingService
    : networkServiceConstruct
    ;


/*
 * --------------------------------------------------------------------------
 * SOCKET
 * --------------------------------------------------------------------------
 */

networkingSocket
    : socketConstruct
    ;


/*
 * --------------------------------------------------------------------------
 * STREAMING
 * --------------------------------------------------------------------------
 */

networkingStream
    : networkStreamingConstruct
    ;


/*
 * --------------------------------------------------------------------------
 * DISTRIBUTED COMPUTE
 * --------------------------------------------------------------------------
 */

networkingDistributedCompute
    : networkingDistributedComputeConstruct
    ;


/*
 * --------------------------------------------------------------------------
 * NETWORK CAPABILITY
 * --------------------------------------------------------------------------
 */

networkingCapability
    : networkCapabilityConstruct
    ;


/*
 * ============================================================================
 * UNIVERSAL NETWORKING CONTRACT
 * ============================================================================
 *
 * This adapter is intentionally equivalent to `networkingDeclaration`.
 *
 * grammar/antlr/ZamaniParser.g4 contains:
 *
 *     universalNetworking
 *         : networkingDeclaration
 *         ;
 *
 * Keeping this contract here means the universal parser does not need to know
 * about any networking leaf grammar.
 * ============================================================================
 */

universalNetworking
    : networkingDeclaration
    ;


/*
 * ============================================================================
 * AST BOUNDARY
 * ============================================================================
 *
 * This grammar produces only ANTLR parse-tree contexts.
 *
 * It MUST NOT:
 *
 *     - create AST nodes;
 *     - resolve names;
 *     - resolve addresses;
 *     - resolve services;
 *     - select protocols;
 *     - select transports;
 *     - discover endpoints;
 *     - discover hardware;
 *     - allocate resources;
 *     - create sockets;
 *     - perform network I/O;
 *     - create IR;
 *     - schedule communication.
 *
 * The frontend AST must preserve:
 *
 *     - source span;
 *     - construct kind;
 *     - source ordering;
 *     - names;
 *     - expressions;
 *     - types;
 *     - attributes;
 *     - nested declarations;
 *     - requirements;
 *     - constraints;
 *     - capabilities;
 *     - preferences;
 *     - metadata.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * After parsing:
 *
 *     networking parse tree
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *
 * Semantic analysis owns:
 *
 *     - name resolution;
 *     - endpoint validity;
 *     - address validity;
 *     - protocol compatibility;
 *     - message/type compatibility;
 *     - request/response compatibility;
 *     - route validity;
 *     - service resolution;
 *     - socket compatibility;
 *     - stream compatibility;
 *     - capability satisfaction;
 *     - resource requirements;
 *     - security requirements;
 *     - distributed compatibility;
 *     - portability validation.
 *
 * None of these are parser responsibilities.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RESOURCE / CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Networking source may express semantic requirements such as:
 *
 *     requires capability("network.streaming")
 *     requires capability("network.reliable")
 *     requires bandwidth(...)
 *     requires latency(...)
 *     requires reliability(...)
 *     requires locality(...)
 *
 * These are expressions/contracts.
 *
 * The grammar MUST NOT convert them into universal hardware constants.
 *
 * In particular, this grammar MUST NOT encode:
 *
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_CONNECTIONS
 *     MAX_NODES
 *     MAX_NETWORK_SIZE
 *
 * or equivalent restrictions.
 *
 * Capability satisfaction occurs downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Networking constructs may reference security intent.
 *
 * The networking grammar does not implement:
 *
 *     - authentication;
 *     - authorization;
 *     - encryption;
 *     - certificate validation;
 *     - key management;
 *     - trust evaluation;
 *     - secret storage.
 *
 * Security semantics belong to the security/effects/semantic layers.
 *
 * Parsing must never access credentials, secrets, certificates, or external
 * security infrastructure.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DISTRIBUTED COMPUTING BOUNDARY
 * ============================================================================
 *
 * `distributed-compute.g4` provides only networking-facing distributed
 * communication intent.
 *
 * The general distributed domain remains owned by:
 *
 *     grammar/distributed/
 *
 * Networking MUST NOT become the owner of:
 *
 *     - cluster membership;
 *     - distributed scheduler implementation;
 *     - replication implementation;
 *     - consensus implementation;
 *     - distributed recovery implementation.
 *
 * Those systems consume networking semantic information downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CONCURRENCY BOUNDARY
 * ============================================================================
 *
 * Networking channels and concurrency channels are distinct concepts.
 *
 * Networking:
 *
 *     grammar/networking/channels.g4
 *
 * owns logical communication-network channel contracts.
 *
 * Concurrency:
 *
 *     grammar/concurrency/channels.g4
 *
 * owns language-level concurrent communication primitives.
 *
 * This aggregate MUST NOT merge them into one grammar abstraction.
 *
 * A semantic integration may connect them downstream, but ownership remains
 * separate.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DATA BOUNDARY
 * ============================================================================
 *
 * Messages may refer to canonical Zamani types and data structures.
 *
 * Networking does not own:
 *
 *     - schemas;
 *     - serialization formats;
 *     - compression;
 *     - databases;
 *     - data transformations;
 *     - binary layouts.
 *
 * Those remain downstream/data/interoperability responsibilities.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * Networking can participate in communication involving:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     embedded systems
 *     distributed systems
 *     future computational substrates
 *
 * However, this grammar does not select or identify physical hardware.
 *
 * Hardware capability and physical realization remain downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Networking can participate in hybrid classical/quantum computation and
 * quantum networking.
 *
 * This grammar does NOT define:
 *
 *     - qubits;
 *     - quantum gates;
 *     - quantum states;
 *     - physical qubits;
 *     - quantum topology;
 *     - pulse schedules;
 *     - calibration;
 *     - QEC;
 *     - ZQN.
 *
 * Where networking participates in quantum computation, semantic lowering
 * remains connected to the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * There is no networking-specific quantum IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ROUTING BOUNDARY
 * ============================================================================
 *
 * `routing.g4` expresses source-level routing intent.
 *
 * It does not choose the physical route.
 *
 * The actual route is determined downstream by:
 *
 *     semantic analysis
 *         ->
 *     routing
 *         ->
 *     topology/capability analysis
 *         ->
 *     scheduling
 *         ->
 *     target realization
 *
 * The aggregate grammar does not perform routing.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SERVICE DISCOVERY BOUNDARY
 * ============================================================================
 *
 * `service-discovery.g4` expresses discovery intent.
 *
 * Parsing does not:
 *
 *     - query registries;
 *     - query DNS;
 *     - contact services;
 *     - resolve endpoints;
 *     - perform network I/O.
 *
 * Discovery occurs only downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SOCKET BOUNDARY
 * ============================================================================
 *
 * `sockets.g4` describes logical socket contracts.
 *
 * A logical socket MAY later be realized through:
 *
 *     - OS sockets;
 *     - IPC;
 *     - shared memory;
 *     - accelerator fabrics;
 *     - message passing;
 *     - distributed communication;
 *     - quantum/classical communication;
 *     - future communication substrates.
 *
 * The grammar does not require OS socket semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * STREAMING BOUNDARY
 * ============================================================================
 *
 * `streaming.g4` describes logical stream contracts.
 *
 * It does not select:
 *
 *     - TCP;
 *     - QUIC;
 *     - HTTP;
 *     - a vendor fabric;
 *     - a physical link;
 *     - a machine;
 *     - a NIC.
 *
 * Stream realization remains downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * REQUEST / RESPONSE BOUNDARY
 * ============================================================================
 *
 * Requests and responses are source-level communication contracts.
 *
 * They do not themselves imply:
 *
 *     - HTTP;
 *     - RPC;
 *     - sockets;
 *     - a particular transport;
 *     - a particular serialization format.
 *
 * The implementation may choose an appropriate realization satisfying the
 * semantic contract.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ADDRESS BOUNDARY
 * ============================================================================
 *
 * `addresses.g4` deliberately does not make IP/MAC/vendor-specific address
 * syntax the universal language model.
 *
 * An address is a logical address reference/value whose concrete interpretation
 * is determined downstream.
 *
 * This permits:
 *
 *     local addresses
 *     logical addresses
 *     service addresses
 *     symbolic addresses
 *     physical addresses
 *     future addressing systems
 *
 * without creating a closed address vocabulary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * OPEN-WORLD EXTENSION CONTRACT
 * ============================================================================
 *
 * New networking technologies SHOULD normally be represented through:
 *
 *     - canonical names;
 *     - expressions;
 *     - capabilities;
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - attributes;
 *     - dialects;
 *     - modules;
 *     - semantic registries;
 *     - interoperability contracts.
 *
 * A new technology MUST NOT require a new core keyword merely because the
 * technology has a new name.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is a pure function of:
 *
 *     token stream
 *     +
 *     grammar version
 *
 * It must not depend on:
 *
 *     - current time;
 *     - current network state;
 *     - DNS;
 *     - filesystem state;
 *     - environment variables;
 *     - machine topology;
 *     - hardware availability;
 *     - service registries;
 *     - random state.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Parser errors:
 *
 *     malformed syntax;
 *     missing delimiters;
 *     malformed expressions;
 *     malformed declarations;
 *     invalid grammar structure.
 *
 * Semantic errors:
 *
 *     unknown endpoint;
 *     unknown address;
 *     incompatible protocol;
 *     unknown service;
 *     invalid request/response relation;
 *     unsatisfied capability;
 *     impossible resource requirement;
 *     invalid route;
 *     unsupported realization.
 *
 * Runtime errors:
 *
 *     connection failure;
 *     unavailable service;
 *     runtime timeout;
 *     link failure;
 *     runtime resource exhaustion.
 *
 * These categories MUST remain separate.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar is structurally open-ended.
 *
 * The following all use ordinary ANTLR repetition or component-defined
 * repetition and therefore have no language-level finite maximum:
 *
 *     networkingConstruct*
 *     component declaration lists
 *     reference lists
 *     member lists
 *     property lists
 *     protocol roles
 *     protocol messages
 *     protocol states
 *     protocol transitions
 *     request properties
 *     response properties
 *     routes
 *     services
 *     streams
 *     distributed communication declarations
 *     capability declarations
 *
 * "Infinity" here means:
 *
 *     no artificial language-defined finite ceiling.
 *
 * It does not mean that memory, compiler time, runtime capacity, or physical
 * network resources are infinite.
 *
 * If an implementation cannot represent a requested program because of an
 * implementation resource limit, that must remain an implementation/resource
 * diagnostic rather than becoming a hidden grammar restriction.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The aggregate rule:
 *
 *     networkingConstruct
 *
 * maps to a domain-neutral AST networking construct carrying its concrete
 * construct kind and source span.
 *
 * The aggregate grammar MUST NOT require a networking-specific semantic AST
 * hierarchy that duplicates the repository's canonical frontend AST.
 *
 * Suggested semantic classification:
 *
 *     Address
 *     Endpoint
 *     Channel
 *     Message
 *     Protocol
 *     Request
 *     Response
 *     Route
 *     ServiceDiscovery
 *     Service
 *     Socket
 *     Stream
 *     DistributedCommunication
 *     NetworkCapability
 *
 * Exact Rust AST ownership belongs to the frontend AST contract, not this
 * grammar file.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Every networking construct follows:
 *
 *     syntax
 *       ->
 *     AST
 *       ->
 *     name/type/effect/capability/resource analysis
 *       ->
 *     canonical semantic representation
 *
 * The grammar must never require target-specific semantic choices.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Networking syntax does NOT directly lower to a target backend.
 *
 * Correct direction:
 *
 *     Networking grammar
 *         ->
 *     frontend AST
 *         ->
 *     semantic networking model
 *         ->
 *     canonical/domain IR
 *         ->
 *     optimization
 *         ->
 *     routing
 *         ->
 *     scheduling
 *         ->
 *     placement
 *         ->
 *     target realization
 *
 * Where a networking operation participates in quantum computation:
 *
 *     semantic networking model
 *         ->
 *     quantum semantic integration
 *         ->
 *     quantum::ir
 *
 * remains the canonical quantum boundary.
 *
 * No second quantum IR is permitted here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler responsibilities include:
 *
 *     - capability matching;
 *     - resource analysis;
 *     - protocol validation;
 *     - communication optimization;
 *     - routing;
 *     - scheduling;
 *     - placement;
 *     - target specialization;
 *     - portability validation.
 *
 * The grammar provides source structure only.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime responsibilities include:
 *
 *     - connection establishment;
 *     - endpoint resolution;
 *     - service discovery;
 *     - address resolution;
 *     - transport selection;
 *     - route realization;
 *     - message transmission;
 *     - stream execution;
 *     - recovery;
 *     - runtime resource management.
 *
 * This grammar performs none of these actions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * Tooling may use this aggregate grammar for:
 *
 *     - syntax highlighting;
 *     - parser diagnostics;
 *     - source navigation;
 *     - formatter structure;
 *     - language-server parsing;
 *     - static-analysis input;
 *     - documentation generation;
 *     - grammar conformance tests.
 *
 * Tools MUST consume the grammar as syntax rather than reverse-engineering
 * runtime behavior.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing valid networking syntax must remain valid unless the repository's
 * explicit compatibility policy permits a breaking change.
 *
 * Changes must be reflected through:
 *
 *     grammar/compatibility/
 *     grammar/spec/
 *     grammar/grammar.md
 *     grammar/Zamani-Grammar.md
 *
 * and, where implementation-facing changes are required:
 *
 *     src/lexer.rs
 *     src/parser.rs
 *     src/ast/
 *     semantic implementation
 *     IR implementation
 *     tests
 *
 * The aggregate grammar itself must not silently create a networking language
 * fork.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * networking.g4 requires tests for:
 *
 * POSITIVE:
 *
 *     - addresses;
 *     - endpoints;
 *     - channels;
 *     - messages;
 *     - protocols;
 *     - requests;
 *     - responses;
 *     - routes;
 *     - service discovery;
 *     - services;
 *     - sockets;
 *     - streams;
 *     - distributed compute;
 *     - capabilities;
 *     - mixed networking programs.
 *
 * NEGATIVE:
 *
 *     - malformed constructs;
 *     - incomplete constructs;
 *     - invalid delimiters;
 *     - malformed expressions;
 *     - malformed nesting;
 *     - invalid declarations.
 *
 * BOUNDARY:
 *
 *     - one construct;
 *     - many constructs;
 *     - deeply nested constructs;
 *     - large property sets;
 *     - large protocol flows;
 *     - large message definitions;
 *     - large service definitions.
 *
 * SCALABILITY:
 *
 *     - many endpoints;
 *     - many channels;
 *     - many messages;
 *     - many protocols;
 *     - many requests;
 *     - many responses;
 *     - many routes;
 *     - many services;
 *     - many streams;
 *     - many distributed computations.
 *
 * DETERMINISM:
 *
 * Identical token streams under the same grammar version must produce
 * equivalent parse structures.
 *
 * CROSS-DOMAIN:
 *
 *     classical + networking
 *     quantum + networking
 *     hybrid + networking
 *     HDL + networking
 *     hardware + networking
 *     distributed + networking
 *     AI + networking
 *     data + networking
 *     security + networking
 *     concurrency + networking
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden universal networking limits include:
 *
 *     MAX_ENDPOINTS
 *     MAX_ADDRESSES
 *     MAX_CHANNELS
 *     MAX_MESSAGES
 *     MAX_PROTOCOLS
 *     MAX_REQUESTS
 *     MAX_RESPONSES
 *     MAX_ROUTES
 *     MAX_SERVICES
 *     MAX_SOCKETS
 *     MAX_STREAMS
 *     MAX_CONNECTIONS
 *     MAX_NODES
 *     MAX_NETWORK_SIZE
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *
 * This aggregate contains none of them.
 *
 * Numeric literals appearing inside component expressions remain ordinary
 * program values and MUST NOT be interpreted as universal implementation
 * ceilings.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete as the networking composition boundary when:
 *
 * [x] Every existing networking component grammar is imported.
 *
 * [x] No networking component is renamed.
 *
 * [x] No component grammar is duplicated here.
 *
 * [x] `networkingConstruct` is the single aggregate construct dispatcher.
 *
 * [x] `networkingDeclaration` exists for ZamaniParser.g4 integration.
 *
 * [x] `networkingElement` exists for ZamaniParser.g4 integration.
 *
 * [x] `universalNetworking` exists for the universal parser contract.
 *
 * [x] The grammar remains target-independent.
 *
 * [x] No physical network topology is hard-coded.
 *
 * [x] No transport implementation is hard-coded.
 *
 * [x] No vendor implementation is hard-coded.
 *
 * [x] No machine-size limit is hard-coded.
 *
 * [x] No resource allocation occurs during parsing.
 *
 * [x] No network I/O occurs during parsing.
 *
 * [x] No hardware discovery occurs during parsing.
 *
 * [x] No runtime behavior occurs during parsing.
 *
 * [x] No IR is constructed by the grammar.
 *
 * [x] The canonical quantum::ir boundary remains downstream.
 *
 * [x] POCO-REAF remains a semantic/compiler/runtime property.
 *
 * [x] Safe Rust 1.97 / 1.97.1 remains sufficient for the implementation.
 *
 * [x] Existing component files retain ownership of their syntax.
 *
 * [x] The aggregate has one-way dependency direction.
 *
 * [x] The aggregate can be independently grammar-tested.
 *
 * Remaining repository-wide completion belongs to the corresponding:
 *
 *     AST contracts
 *     semantic implementation
 *     IR contracts
 *     compiler implementation
 *     runtime implementation
 *     conformance tests
 *
 * and is intentionally NOT duplicated inside this grammar.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "Which networking source constructs belong to the Zamani networking
 *      domain, and how are their existing component grammars composed?"
 *
 * It does NOT answer:
 *
 *     "Which machine?"
 *     "Which CPU?"
 *     "Which GPU?"
 *     "Which FPGA?"
 *     "Which QPU?"
 *     "Which NIC?"
 *     "Which IP?"
 *     "Which router?"
 *     "Which provider?"
 *     "Which physical route?"
 *     "Which transport implementation?"
 *     "How many nodes?"
 *     "How many connections?"
 *
 * Those decisions remain downstream.
 *
 * ============================================================================
 * POCO-REAF FINAL PRINCIPLE
 * ============================================================================
 *
 *     One Zamani program
 *             |
 *             v
 *     one stable networking meaning
 *             |
 *             v
 *     many valid realizations
 *             |
 *             +--> local
 *             +--> embedded
 *             +--> single machine
 *             +--> multicore
 *             +--> accelerator
 *             +--> distributed
 *             +--> HPC
 *             +--> cloud
 *             +--> edge
 *             +--> quantum/classical
 *             +--> future substrate
 *
 * subject only to the program's actual semantic requirements, available
 * capabilities, resources, and implementation correctness.
 *
 * ============================================================================
 */