/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/networking/requests.g4
 *
 * Grammar:
 *     NetworkingRequests
 *
 * Status:
 *     Production parser grammar contract
 *
 * Purpose:
 *     Define first-class, reusable SOURCE-LEVEL NETWORK REQUEST CONTRACTS.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     2021
 *
 * Safety:
 *     - No embedded Rust
 *     - No actions
 *     - No semantic predicates
 *     - No unsafe code
 *     - No filesystem access
 *     - No network access
 *     - No hardware access
 *     - No runtime callbacks
 *     - No randomness
 *     - No environment inspection
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
 *     Networking
 *          |
 *          v
 *     NetworkingRequests
 *          |
 *          v
 *     domain-neutral frontend AST
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
 *          +--> portability analysis
 *          +--> networking analysis
 *          +--> distributed analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> networking semantics
 *          +--> classical semantics
 *          +--> distributed semantics
 *          +--> hybrid semantics
 *          +--> hardware intent
 *          +--> quantum semantic integration
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / discovery / placement / scheduling
 *          |
 *          v
 *     runtime / HAL / target realization
 *
 * THIS GRAMMAR NEVER:
 *
 *     - sends a request;
 *     - receives a response;
 *     - opens a socket;
 *     - chooses a route;
 *     - selects a machine;
 *     - selects a CPU/GPU/FPGA/QPU;
 *     - allocates a network resource;
 *     - serializes data;
 *     - authenticates;
 *     - authorizes;
 *     - performs cryptography;
 *     - performs service discovery;
 *     - performs scheduling;
 *     - constructs IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - reusable network request declarations;
 *     - request declaration identity;
 *     - request contract bodies;
 *     - request contract properties;
 *     - request nested configuration;
 *     - request type references expressed through properties;
 *     - request source/destination intent expressed through properties;
 *     - request protocol/channel intent expressed through properties;
 *     - request requirements;
 *     - request constraints;
 *     - request capabilities;
 *     - request preferences;
 *     - request policies;
 *     - request metadata;
 *     - stable parser entry points for request constructs.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - message schemas;
 *     - message values;
 *     - endpoints;
 *     - addresses;
 *     - channels;
 *     - protocols;
 *     - services;
 *     - sockets;
 *     - transport implementations;
 *     - HTTP;
 *     - TCP;
 *     - UDP;
 *     - QUIC;
 *     - MQTT;
 *     - gRPC;
 *     - MPI;
 *     - RDMA;
 *     - serialization;
 *     - wire formats;
 *     - routing;
 *     - scheduling;
 *     - distributed placement;
 *     - topology;
 *     - node membership;
 *     - security implementation;
 *     - cryptographic implementation;
 *     - authentication;
 *     - authorization;
 *     - hardware discovery;
 *     - resource allocation;
 *     - runtime execution;
 *     - quantum gates;
 *     - quantum states;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir.
 *
 * ============================================================================
 * IMPORTANT OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * `grammar/networking/services.g4` already owns SERVICE-LOCAL request and
 * response declarations.
 *
 * Those constructs describe request/response members inside a network service.
 *
 * This file instead owns REUSABLE, FIRST-CLASS NETWORK REQUEST CONTRACTS
 * that can be referenced by services, channels, distributed computation,
 * hybrid execution, data pipelines, classical computation, quantum/classical
 * communication, and future networking domains.
 *
 * Therefore:
 *
 *     services.g4
 *         -> service-local request/response syntax
 *
 *     requests.g4
 *         -> reusable network request contracts
 *
 * Neither grammar should copy the other's implementation.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A request describes LOGICAL COMMUNICATION INTENT.
 *
 * It does not describe a particular physical realization.
 *
 * The same request may eventually be realized through:
 *
 *     - an in-process call;
 *     - shared memory;
 *     - IPC;
 *     - local messaging;
 *     - a network;
 *     - a distributed fabric;
 *     - an accelerator fabric;
 *     - CPU/GPU/FPGA communication;
 *     - quantum/classical communication;
 *     - HPC interconnects;
 *     - cloud infrastructure;
 *     - edge infrastructure;
 *     - future computational substrates.
 *
 * The source request contract remains independent of those realizations.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * This grammar deliberately DOES NOT enumerate:
 *
 *     HTTP
 *     HTTPS
 *     TCP
 *     UDP
 *     QUIC
 *     MQTT
 *     gRPC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     vendor transports
 *     cloud providers
 *     hardware vendors
 *     network devices
 *
 * These may appear as ordinary names/expressions when appropriate.
 *
 * Example:
 *
 *     protocol: http;
 *
 * or:
 *
 *     protocol: network::request_response;
 *
 * or:
 *
 *     protocol: vendor::future_transport;
 *
 * Semantic analysis determines whether such a name denotes a valid protocol
 * and whether the selected realization provides it.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE SEPARATION
 * ============================================================================
 *
 * Request properties are syntactic data.
 *
 * Semantic analysis determines whether a property represents:
 *
 *     - requirement;
 *     - constraint;
 *     - capability;
 *     - preference;
 *     - policy;
 *     - metadata;
 *     - implementation hint;
 *     - logical relationship.
 *
 * Examples:
 *
 *     requires: capability("network.reliable");
 *
 *     requires: bandwidth >= required_bandwidth;
 *
 *     prefers: locality::near;
 *
 *     constraint: latency < maximum_latency;
 *
 *     security: security::authenticated;
 *
 * The parser does not decide whether these requirements can actually be met.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO language-level limits for:
 *
 *     MAX_REQUESTS
 *     MAX_REQUEST_PROPERTIES
 *     MAX_REQUEST_MEMBERS
 *     MAX_REQUEST_ARGUMENTS
 *     MAX_ENDPOINTS
 *     MAX_CHANNELS
 *     MAX_MESSAGES
 *     MAX_PROTOCOLS
 *     MAX_SERVICES
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_NETWORK_SIZE
 *     MAX_REQUEST_SIZE
 *     MAX_PAYLOAD_SIZE
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_RETRIES
 *
 * No such constants appear in the grammar.
 *
 * Repetition uses ANTLR `*` / `+` constructs.
 *
 * Practical limitations may arise from:
 *
 *     - available memory;
 *     - parser implementation;
 *     - compiler resources;
 *     - runtime resources;
 *     - operating-system resources;
 *     - deployment resources;
 *     - target hardware.
 *
 * Those are NOT language-level limits.
 *
 * ============================================================================
 * "INFINITY" INTERPRETATION
 * ============================================================================
 *
 * "Scale from tiny to infinity" means:
 *
 *     the language does not impose an artificial finite upper bound on the
 *     size or number of request contracts or their logical relationships.
 *
 * It does NOT mean that physical machines have infinite resources.
 *
 * Execution remains bounded by resources actually available to the selected
 * realization.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     MAX_REQUESTS
 *     MAX_CHANNELS
 *     MAX_ENDPOINTS
 *     MAX_NODES
 *     MAX_CONNECTIONS
 *     MAX_MESSAGE_SIZE
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICES
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *
 * It MUST NOT encode physical resources such as:
 *
 *     cpu_0
 *     gpu_0
 *     fpga_0
 *     qpu_0
 *     node_0
 *     socket_0
 *     interface_0
 *
 * as language-level networking concepts.
 *
 * An application may use such strings/names as ordinary program data, but
 * this grammar gives them no special physical meaning.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The networking request grammar defines NO lexer rules.
 *
 * It consumes the canonical lexer vocabulary:
 *
 *     ZamaniLexer
 *
 * and canonical parser components:
 *
 *     Names
 *     Types
 *     Expressions
 *     Attributes
 *
 * The current repository intentionally uses contextual declaration markers
 * for several networking constructs where a dedicated global lexer keyword
 * is not yet authoritative.
 *
 * Therefore `request` is represented structurally by:
 *
 *     requestMarker
 *         : identifier
 *         ;
 *
 * Semantic validation determines whether the marker has the canonical
 * request spelling in the relevant declaration context.
 *
 * This avoids creating a second keyword authority.
 *
 * ============================================================================
 * PARSER DECLARATION
 * ============================================================================
 */

parser grammar NetworkingRequests;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Types, Expressions, Attributes;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable integration boundary for the networking aggregate.
 *
 * Only request declarations are public networking request constructs.
 *
 * Ordinary request invocation/execution is intentionally NOT represented here.
 *
 * A request invocation is ordinary expression/call syntax and belongs to the
 * canonical expression grammar.
 * ============================================================================
 */

networkRequestConstruct
    : networkRequestDeclaration
    ;


/*
 * ============================================================================
 * REQUEST DECLARATION
 * ============================================================================
 *
 * Canonical conceptual forms:
 *
 *     request GetUser;
 *
 *     request GetUser {
 *         message: UserQuery;
 *         source: client;
 *         destination: user_service;
 *         protocol: network::request_response;
 *     }
 *
 *     request Compute {
 *         message: ComputeRequest;
 *         channel: computation;
 *         requires: capability("network.reliable");
 *     };
 *
 * The grammar intentionally does not prescribe which properties are required.
 *
 * Property legality and completeness are semantic responsibilities.
 * ============================================================================
 */

networkRequestDeclaration
    : attribute*
      networkRequestMarker
      identifier
      networkRequestBody?
      SEMI?
    ;


/*
 * ============================================================================
 * CONTEXTUAL REQUEST MARKER
 * ============================================================================
 *
 * The canonical lexer currently does not require a dedicated REQUEST token
 * for this domain construct.
 *
 * Keeping this as an identifier:
 *
 *     - preserves lexical authority;
 *     - avoids keyword duplication;
 *     - permits compatibility with the current parser hierarchy;
 *     - allows future promotion of `request` to a reserved token without
 *       changing the semantic model.
 * ============================================================================
 */

networkRequestMarker
    : identifier
    ;


/*
 * ============================================================================
 * REQUEST BODY
 * ============================================================================
 *
 * A request body is a finite source sequence syntactically represented with
 * repetition. No language-level cardinality limit is imposed.
 * ============================================================================
 */

networkRequestBody
    : LBRACE
      networkRequestMember*
      RBRACE
    ;


/*
 * ============================================================================
 * REQUEST MEMBER
 * ============================================================================
 *
 * There is deliberately ONE canonical property structure.
 *
 * This avoids ambiguous alternatives such as:
 *
 *     requestSourceProperty
 *     requestDestinationProperty
 *     requestProtocolProperty
 *     requestChannelProperty
 *     requestCapabilityProperty
 *
 * all separately matching:
 *
 *     identifier COLON expression SEMI
 *
 * Semantic analysis determines the meaning of the property name.
 * ============================================================================
 */

networkRequestMember
    : attribute*
      networkRequestProperty
    | attribute*
      networkRequestNestedBlock
    ;


/*
 * ============================================================================
 * REQUEST PROPERTY
 * ============================================================================
 *
 * Canonical form:
 *
 *     key: expression;
 *
 * Examples:
 *
 *     message: UserQuery;
 *
 *     source: client;
 *
 *     destination: service;
 *
 *     protocol: network::request_response;
 *
 *     channel: telemetry;
 *
 *     timeout: duration;
 *
 *     requires: capability("network.reliable");
 *
 *     prefers: locality::near;
 *
 *     constraint: latency < maximum_latency;
 *
 * Qualified property names permit open-world extensions:
 *
 *     network::timeout: value;
 *     vendor::feature: value;
 *     application::metadata: value;
 * ============================================================================
 */

networkRequestProperty
    : networkRequestPropertyName
      COLON
      networkRequestPropertyValue
      SEMI
    ;


/*
 * ============================================================================
 * REQUEST PROPERTY NAME
 * ============================================================================
 */

networkRequestPropertyName
    : qualifiedName
    ;


/*
 * ============================================================================
 * REQUEST PROPERTY VALUE
 * ============================================================================
 *
 * The value is delegated to the canonical expression grammar.
 *
 * Networking therefore does not create:
 *
 *     - a second literal grammar;
 *     - a second arithmetic grammar;
 *     - a second boolean grammar;
 *     - a second call grammar;
 *     - a second collection grammar.
 * ============================================================================
 */

networkRequestPropertyValue
    : expression
    ;


/*
 * ============================================================================
 * REQUEST NESTED BLOCK
 * ============================================================================
 *
 * Nested blocks are useful for structured policies and future extensibility
 * without introducing a separate grammar for every networking concept.
 *
 * Example:
 *
 *     request Compute {
 *         retry {
 *             policy: adaptive;
 *             limit: retries;
 *         }
 *     }
 *
 * The grammar preserves structure.
 * Semantic analysis decides whether the nested key is valid.
 * ============================================================================
 */

networkRequestNestedBlock
    : networkRequestNestedBlockName
      LBRACE
      networkRequestNestedMember*
      RBRACE
      SEMI?
    ;


networkRequestNestedBlockName
    : qualifiedName
    ;


networkRequestNestedMember
    : attribute*
      networkRequestProperty
    | attribute*
      networkRequestNestedBlock
    ;


/*
 * ============================================================================
 * REUSABLE REQUEST NAME REFERENCE
 * ============================================================================
 *
 * This rule is intentionally NOT exposed as a top-level construct.
 *
 * It exists for other networking grammar components that need to reference
 * a previously declared request contract.
 *
 * Example:
 *
 *     request: api::GetUser;
 *
 * The semantic layer verifies that the referenced symbol actually denotes a
 * network request.
 * ============================================================================
 */

networkRequestReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * REQUEST PROPERTY REFERENCE WRAPPERS
 * ============================================================================
 *
 * These wrappers provide stable semantic/tooling names without duplicating
 * syntax.
 *
 * They MUST NOT be used to create separate parser alternatives in the request
 * body because all of them intentionally share the same structural syntax.
 * ============================================================================
 */

networkRequestMessageProperty
    : networkRequestProperty
    ;


networkRequestSourceProperty
    : networkRequestProperty
    ;


networkRequestDestinationProperty
    : networkRequestProperty
    ;


networkRequestProtocolProperty
    : networkRequestProperty
    ;


networkRequestChannelProperty
    : networkRequestProperty
    ;


networkRequestEndpointProperty
    : networkRequestProperty
    ;


networkRequestRequirementProperty
    : networkRequestProperty
    ;


networkRequestConstraintProperty
    : networkRequestProperty
    ;


networkRequestCapabilityProperty
    : networkRequestProperty
    ;


networkRequestPreferenceProperty
    : networkRequestProperty
    ;


networkRequestPolicyProperty
    : networkRequestProperty
    ;


networkRequestMetadataProperty
    : networkRequestProperty
    ;


/*
 * ============================================================================
 * COMMON SEMANTIC PROPERTY CATEGORIES
 * ============================================================================
 *
 * These are documentation/integration wrappers only.
 *
 * The grammar intentionally does NOT enumerate the corresponding property
 * spellings.
 *
 * Examples that semantic analysis may recognize include:
 *
 *     message
 *     source
 *     destination
 *     endpoint
 *     channel
 *     protocol
 *     requires
 *     constraint
 *     capability
 *     prefers
 *     policy
 *     security
 *     timeout
 *     priority
 *     reliability
 *     ordering
 *     delivery
 *     metadata
 *
 * Future properties remain syntactically valid without changing this file.
 * ============================================================================
 */


/*
 * ============================================================================
 * REQUEST SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis MUST resolve:
 *
 *     - request name;
 *     - request properties;
 *     - referenced message;
 *     - source;
 *     - destination;
 *     - endpoint references;
 *     - channel references;
 *     - protocol references;
 *     - capabilities;
 *     - requirements;
 *     - constraints;
 *     - preferences;
 *     - security policies;
 *     - resource implications;
 *     - portability requirements.
 *
 * The parser MUST NOT perform these resolutions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * MESSAGE OWNERSHIP
 * ============================================================================
 *
 * A request MAY reference a message declaration.
 *
 * Message schema ownership remains:
 *
 *     grammar/networking/messages.g4
 *
 * This file does NOT redefine:
 *
 *     messageDeclaration
 *     messageField
 *     messageValue
 *     messageGenericParameters
 *
 * Example:
 *
 *     request GetUser {
 *         message: UserQuery;
 *     }
 *
 * `UserQuery` is resolved semantically as a message/type reference.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ENDPOINT OWNERSHIP
 * ============================================================================
 *
 * Endpoint declaration syntax remains owned by:
 *
 *     grammar/networking/endpoints.g4
 *
 * A request may reference endpoints:
 *
 *     request GetUser {
 *         source: client;
 *         destination: user_service;
 *     }
 *
 * The parser does not determine whether those names are endpoints.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CHANNEL OWNERSHIP
 * ============================================================================
 *
 * Channel declaration syntax remains owned by:
 *
 *     grammar/networking/channels.g4
 *
 * A request may reference a channel:
 *
 *     request GetUser {
 *         channel: user_requests;
 *     }
 *
 * Physical queueing, buffering, routing and transport are downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PROTOCOL OWNERSHIP
 * ============================================================================
 *
 * Protocol declaration syntax remains owned by:
 *
 *     grammar/networking/protocols.g4
 *
 * A request may refer to a protocol:
 *
 *     request GetUser {
 *         protocol: network::request_response;
 *     }
 *
 * The request grammar does not enumerate protocol implementations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SERVICE OWNERSHIP
 * ============================================================================
 *
 * Service declaration syntax remains owned by:
 *
 *     grammar/networking/services.g4
 *
 * Services may reference reusable request contracts through ordinary
 * expressions/properties.
 *
 * This prevents services and requests from becoming competing grammar
 * authorities.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SOCKET OWNERSHIP
 * ============================================================================
 *
 * Socket syntax remains owned by:
 *
 *     grammar/networking/sockets.g4
 *
 * A request may eventually be realized through a socket, but this grammar
 * does not require or imply socket semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A request may participate in distributed execution.
 *
 * Distributed semantics remain owned by:
 *
 *     grammar/distributed/
 *
 * This grammar therefore does NOT define:
 *
 *     node
 *     worker
 *     cluster
 *     replica
 *     partition
 *     consensus
 *     placement
 *     distributed scheduler
 *
 * Such information can be expressed through properties and interpreted by
 * the semantic/distributed layers.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CLASSICAL / QUANTUM / HYBRID INTEGRATION
 * ============================================================================
 *
 * Requests may carry data generated by:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     AI computation;
 *     HDL/hardware computation;
 *     distributed computation.
 *
 * This grammar does not create domain-specific request variants.
 *
 * For example:
 *
 *     request QuantumResult {
 *         message: QuantumMeasurement;
 *         source: quantum_controller;
 *         destination: classical_host;
 *         requires: capability("quantum.measurement");
 *     }
 *
 * Quantum semantic processing remains connected to:
 *
 *     quantum::ir
 *
 * when quantum computation is involved.
 *
 * No second quantum IR is created here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Request properties may express resource intent:
 *
 *     requires: memory >= required_memory;
 *
 *     requires: capability("network.reliable");
 *
 *     requires: capability("network.streaming");
 *
 *     requires: topology(required_topology);
 *
 * The grammar treats these as expressions.
 *
 * Semantic/resource analysis decides:
 *
 *     - whether the expression is valid;
 *     - whether it is satisfiable;
 *     - which capabilities are required;
 *     - which resources are necessary;
 *     - whether the selected realization satisfies them.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Request security requirements remain semantic properties.
 *
 * Examples:
 *
 *     security: authenticated;
 *
 *     requires: capability("secure.communication");
 *
 *     policy: security::confidential;
 *
 * The grammar does NOT implement:
 *
 *     - cryptography;
 *     - key management;
 *     - authentication;
 *     - authorization;
 *     - certificate verification;
 *     - trust evaluation.
 *
 * Those remain downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PERFORMANCE / QoS INTEGRATION
 * ============================================================================
 *
 * Request properties may express:
 *
 *     latency;
 *     throughput;
 *     bandwidth;
 *     reliability;
 *     ordering;
 *     delivery;
 *     priority;
 *     availability;
 *     locality;
 *     energy;
 *     cost;
 *     deadline;
 *     timeout.
 *
 * These are source-level intent.
 *
 * They are NOT fixed machine parameters.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * A request may cross:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     embedded system
 *     distributed system
 *     future compute substrate
 *
 * The request grammar does not identify a particular physical device.
 *
 * Hardware selection belongs downstream to:
 *
 *     resource analysis;
 *     capability analysis;
 *     target selection;
 *     routing;
 *     scheduling;
 *     deployment;
 *     HAL.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar must lower into the EXISTING domain-neutral frontend AST.
 *
 * It must NOT introduce a competing networking AST hierarchy merely because
 * this grammar exists.
 *
 * The AST must preserve, at minimum:
 *
 *     - declaration source span;
 *     - request name;
 *     - declaration attributes;
 *     - ordered body members;
 *     - property names;
 *     - property values;
 *     - nested block structure;
 *     - nested property order;
 *     - source spans for every member.
 *
 * The AST must preserve the distinction between:
 *
 *     property
 *
 * and:
 *
 *     nested block
 *
 * without assigning semantic meaning prematurely.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC LOWERING CONTRACT
 * ============================================================================
 *
 * The intended pipeline is:
 *
 *     networkRequestDeclaration
 *             |
 *             v
 *     domain-neutral AST
 *             |
 *             v
 *     semantic request contract
 *             |
 *             +--> message resolution
 *             +--> endpoint resolution
 *             +--> channel resolution
 *             +--> protocol resolution
 *             +--> capability resolution
 *             +--> resource analysis
 *             +--> security analysis
 *             +--> portability analysis
 *             |
 *             v
 *     canonical networking semantic representation
 *             |
 *             +--> classical IR
 *             +--> distributed representation
 *             +--> hardware communication representation
 *             +--> quantum::ir integration where required
 *             |
 *             v
 *     optimization / routing / scheduling / deployment
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * THIS FILE DOES NOT define an IR.
 *
 * In particular, it must NOT introduce:
 *
 *     RequestIR
 *     NetworkRequestIR
 *     QuantumRequestIR
 *     PhysicalRequestIR
 *
 * merely because request syntax exists.
 *
 * The semantic layer determines the appropriate canonical representation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime request execution is downstream.
 *
 * This grammar never:
 *
 *     - creates a connection;
 *     - selects a socket;
 *     - sends bytes;
 *     - receives bytes;
 *     - retries;
 *     - redirects;
 *     - queues;
 *     - serializes;
 *     - deserializes;
 *     - authenticates;
 *     - encrypts;
 *     - decrypts.
 *
 * Those operations are compiler/runtime responsibilities.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source text;
 *     - lexer output;
 *     - grammar version;
 *     - imported grammar definitions.
 *
 * There are no:
 *
 *     - semantic predicates;
 *     - parser actions;
 *     - random choices;
 *     - filesystem queries;
 *     - network queries;
 *     - hardware queries;
 *     - environment queries.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This grammar introduces no replacement for:
 *
 *     service-local request declarations
 *
 * already owned by:
 *
 *     grammar/networking/services.g4
 *
 * It adds a reusable first-class networking request declaration.
 *
 * Existing source programs therefore do not need to reinterpret their
 * service-local request syntax merely because this grammar is added.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * The grammar must provide sufficient structure for:
 *
 *     - syntax highlighting;
 *     - formatting;
 *     - symbol indexing;
 *     - request-contract documentation;
 *     - navigation;
 *     - refactoring;
 *     - diagnostics;
 *     - dependency/reference analysis;
 *     - semantic property inspection.
 *
 * Source ordering and source spans must be preserved by the frontend.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * request GetUser;
 *
 * request GetUser {
 *     message: UserQuery;
 *     source: client;
 *     destination: user_service;
 *     protocol: network::request_response;
 * };
 *
 * request Compute {
 *     message: ComputeRequest;
 *     channel: compute_requests;
 *     requires: capability("network.reliable");
 *     prefers: locality::near;
 * };
 *
 * request QuantumMeasurement {
 *     message: QuantumMeasurement;
 *     source: quantum_controller;
 *     destination: classical_host;
 *     requires: capability("quantum.measurement");
 * };
 *
 * request DistributedCompute {
 *     message: ComputeRequest;
 *     source: worker_group;
 *     destination: coordinator;
 *     requires: capability("distributed.communication");
 *     constraint: latency < required_latency;
 * };
 *
 * QUALIFIED EXTENSIONS
 * --------------------
 *
 * request Example {
 *     network::timeout: timeout_value;
 *     vendor::transport: vendor::future_transport;
 *     application::metadata: metadata_value;
 * };
 *
 * NESTED
 * ------
 *
 * request ReliableCompute {
 *     message: ComputeRequest;
 *
 *     retry {
 *         policy: adaptive;
 *         limit: retry_limit;
 *     }
 *
 *     security {
 *         policy: security::authenticated;
 *     }
 * };
 *
 * ATTRIBUTES
 * ----------
 *
 * @portable
 * request PortableRequest {
 *     message: RequestMessage;
 * };
 *
 * NEGATIVE
 * --------
 *
 * request;
 *
 * request 123;
 *
 * request Example {
 *     message;
 * };
 *
 * request Example {
 *     message:
 * };
 *
 * request Example {
 *     source:
 * };
 *
 * request Example {
 *     broken
 * };
 *
 * request Example {
 *     property: value
 *     another: value;
 * };
 *
 * The semantic layer must additionally reject:
 *
 *     - unknown message references where a message is required;
 *     - invalid endpoint references;
 *     - invalid channel references;
 *     - invalid protocol references;
 *     - unsatisfied capabilities;
 *     - impossible resource requirements;
 *     - contradictory constraints.
 *
 * Those are semantic errors, not grammar errors.
 *
 * BOUNDARY
 * --------
 *
 * request A;
 *
 * request VeryLongRequestName {
 *     property: value;
 * };
 *
 * deeply qualified property names;
 *
 * deeply nested policy structures;
 *
 * large property expressions;
 *
 * empty request bodies;
 *
 * large request bodies.
 *
 * SCALABILITY
 * ----------
 *
 * Tests MUST demonstrate:
 *
 *     - one request;
 *     - many requests;
 *     - many properties;
 *     - many nested blocks;
 *     - deeply qualified property names;
 *     - large expressions;
 *     - large source programs;
 *     - very small source programs.
 *
 * No test may establish an artificial maximum.
 *
 * DETERMINISM
 * -----------
 *
 * The same token stream and grammar version MUST produce equivalent parse
 * structure regardless of:
 *
 *     - machine size;
 *     - target hardware;
 *     - network availability;
 *     - runtime state;
 *     - environment variables.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is an independent parser grammar.
 * [x] It uses the canonical lexer vocabulary.
 * [x] It reuses Names.
 * [x] It reuses Types.
 * [x] It reuses Expressions.
 * [x] It reuses Attributes.
 * [x] It owns reusable network request contracts.
 * [x] It does not duplicate message schemas.
 * [x] It does not duplicate endpoint syntax.
 * [x] It does not duplicate channel syntax.
 * [x] It does not duplicate protocol syntax.
 * [x] It does not duplicate service syntax.
 * [x] It does not duplicate socket syntax.
 * [x] It does not duplicate distributed syntax.
 * [x] It does not enumerate transport implementations.
 * [x] It does not enumerate hardware.
 * [x] It does not encode machine capacities.
 * [x] It does not encode networking limits.
 * [x] It does not create a second expression grammar.
 * [x] It does not create a second type grammar.
 * [x] It does not create an IR.
 * [x] It does not create a second quantum IR.
 * [x] It preserves extensibility.
 * [x] It preserves source structure.
 * [x] It supports nested configuration.
 * [x] It supports qualified property names.
 * [x] It supports attributes.
 * [x] It is deterministic.
 * [x] It contains no actions.
 * [x] It contains no predicates.
 * [x] It contains no unsafe code.
 * [x] It is compatible with Rust 1.97 / 1.97.1 generated-parser integration.
 * [x] It provides an explicit AST contract.
 * [x] It provides an explicit semantic contract.
 * [x] It provides an explicit IR boundary.
 * [x] It provides an explicit runtime boundary.
 * [x] It provides positive/negative/boundary/scalability/determinism tests.
 *
 * ============================================================================
 */