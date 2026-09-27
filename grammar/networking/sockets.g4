/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/networking/sockets.g4
 *
 * Grammar:
 *     Sockets
 *
 * Status:
 *     Production networking socket-contract grammar
 *
 * Purpose:
 *     Define the canonical Zamani source syntax for LOGICAL SOCKET
 *     COMMUNICATION CONTRACTS.
 *
 * IMPORTANT:
 *
 *     A Zamani "socket" is NOT inherently an operating-system socket.
 *
 *     It is a source-level communication binding abstraction that MAY
 *     eventually be realized as:
 *
 *         - an OS socket;
 *         - IPC;
 *         - shared memory;
 *         - message passing;
 *         - a network transport;
 *         - an accelerator fabric;
 *         - a distributed communication mechanism;
 *         - a quantum/classical communication mechanism;
 *         - a future communication substrate.
 *
 *     Physical realization is determined downstream.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     2021
 *
 * Safety:
 *     - No embedded Rust.
 *     - No unsafe code.
 *     - No semantic predicates.
 *     - No parser actions.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No environment inspection.
 *     - No runtime callbacks.
 *     - No randomness.
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
 *     Zamani parser
 *          |
 *          v
 *     Networking
 *          |
 *          v
 *     Sockets
 *          |
 *          v
 *     frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> portability analysis
 *          |
 *          v
 *     networking semantic model
 *          |
 *          +--> classical semantics
 *          +--> distributed semantics
 *          +--> quantum semantics
 *          +--> hybrid semantics
 *          +--> hardware/software co-design
 *          |
 *          v
 *     canonical IR / domain IR
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / placement / scheduling
 *          |
 *          v
 *     deployment
 *          |
 *          v
 *     runtime / HAL / target realization
 *
 * This grammar MUST NEVER construct IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical socket declarations;
 *     - socket identity;
 *     - socket configuration structure;
 *     - socket communication-contract properties;
 *     - socket references;
 *     - socket generic/type annotations where applicable;
 *     - socket lifecycle policy declarations;
 *     - socket capability/requirement property syntax;
 *     - socket endpoint/address/protocol/channel references;
 *     - socket extension blocks;
 *     - stable parser entry points for socket declarations.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - attributes;
 *     - endpoints;
 *     - addresses;
 *     - protocols;
 *     - channels;
 *     - messages;
 *     - services;
 *     - concurrency channels;
 *     - distributed nodes;
 *     - routing;
 *     - scheduling;
 *     - placement;
 *     - transport implementations;
 *     - operating-system socket APIs;
 *     - file descriptors;
 *     - IP allocation;
 *     - MAC allocation;
 *     - DNS resolution;
 *     - NIC selection;
 *     - physical topology;
 *     - serialization;
 *     - compression;
 *     - cryptography;
 *     - authentication implementation;
 *     - authorization implementation;
 *     - hardware discovery;
 *     - resource allocation;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - classical IR;
 *     - HDL IR;
 *     - runtime execution.
 *
 * ============================================================================
 * SOCKET VS ENDPOINT
 * ============================================================================
 *
 * An endpoint identifies a logical communication participant.
 *
 * A socket describes a logical communication binding/interface through which
 * that participant may communicate.
 *
 * Therefore:
 *
 *     endpoint
 *         =
 *     WHO / WHAT participates
 *
 *     socket
 *         =
 *     HOW communication is logically bound
 *
 * Neither abstraction inherently identifies a physical machine.
 *
 * ============================================================================
 * SOCKET VS ADDRESS
 * ============================================================================
 *
 * Address syntax belongs to:
 *
 *     grammar/networking/addresses.g4
 *
 * A socket MAY reference an address.
 *
 * This file MUST NOT redefine address syntax.
 *
 * Example semantic relationship:
 *
 *     socket listener {
 *         local: server_address;
 *     }
 *
 * The parser treats `server_address` as an expression.
 *
 * Address resolution occurs downstream.
 *
 * ============================================================================
 * SOCKET VS PROTOCOL
 * ============================================================================
 *
 * Protocol syntax belongs to:
 *
 *     grammar/networking/protocols.g4
 *
 * A socket MAY reference a protocol.
 *
 * The grammar MUST NOT enumerate:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     MQTT
 *     MPI
 *     RDMA
 *     InfiniBand
 *
 * as a closed socket vocabulary.
 *
 * A protocol is represented through the canonical name/expression system.
 *
 * ============================================================================
 * SOCKET VS CHANNEL
 * ============================================================================
 *
 * Networking-channel syntax belongs to:
 *
 *     grammar/networking/channels.g4
 *
 * A socket MAY bind to or expose a logical networking channel.
 *
 * This file does not duplicate channel semantics.
 *
 * ============================================================================
 * SOCKET VS CONCURRENCY CHANNEL
 * ============================================================================
 *
 * Generic concurrency channels belong to:
 *
 *     grammar/concurrency/channels.g4
 *
 * A networking socket may participate in concurrent execution, but this file
 * does not redefine concurrency-channel semantics.
 *
 * ============================================================================
 * SOCKET VS SERVICE
 * ============================================================================
 *
 * Service syntax belongs to:
 *
 *     grammar/networking/services.g4
 *
 * A service may expose or consume a socket contract.
 *
 * The socket grammar does not define service declarations.
 *
 * ============================================================================
 * SOCKET VS TRANSPORT
 * ============================================================================
 *
 * Transport is an implementation/semantic realization concern.
 *
 * A socket contract may contain a property such as:
 *
 *     transport: network::transport;
 *
 * but the grammar does not interpret that value.
 *
 * A future transport can therefore be introduced without modifying this file.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Socket syntax MUST preserve:
 *
 *     Program Once
 *         ->
 *     Compile Once
 *         ->
 *     Run Everywhere
 *         ->
 *     Run Anywhere
 *         ->
 *     Run Forever
 *
 * subject to:
 *
 *     - program semantics;
 *     - explicit requirements;
 *     - target capabilities;
 *     - available resources;
 *     - deployment constraints.
 *
 * The source MUST NOT require a particular physical socket implementation
 * merely because a logical socket is declared.
 *
 * ============================================================================
 * NO ARTIFICIAL HARDWARE LIMITS
 * ============================================================================
 *
 * This grammar MUST NOT contain:
 *
 *     MAX_SOCKETS
 *     MAX_CONNECTIONS
 *     MAX_ENDPOINTS
 *     MAX_PORTS
 *     MAX_NODES
 *     MAX_INTERFACES
 *     MAX_NETWORK_SIZE
 *     MAX_PACKET_SIZE
 *     MAX_MESSAGE_SIZE
 *     MAX_CHANNELS
 *     MAX_CLIENTS
 *     MAX_SERVERS
 *     MAX_SOCKETS_PER_NODE
 *     MAX_CONNECTIONS_PER_SOCKET
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *
 * Socket declarations use normal ANTLR repetition operators.
 *
 * Physical resource limits are downstream implementation/resource constraints.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Socket semantics MUST remain open to future communication technologies.
 *
 * The grammar therefore MUST NOT contain closed alternatives such as:
 *
 *     tcpSocket
 *     udpSocket
 *     quicSocket
 *     unixSocket
 *     rdmaSocket
 *     mpiSocket
 *     infinibandSocket
 *
 * Instead, a socket may contain semantic references:
 *
 *     protocol: network::tcp;
 *
 *     transport: network::quic;
 *
 *     capability: networking::reliable_delivery;
 *
 * These are expressions/names.
 *
 * Their meaning is determined by semantic analysis.
 *
 * ============================================================================
 * CONTEXTUAL SOCKET MARKER
 * ============================================================================
 *
 * The repository's networking grammars deliberately avoid adding a global
 * keyword for every networking concept.
 *
 * Therefore:
 *
 *     socket
 *
 * is treated as a contextual identifier here rather than requiring a new
 * SOCKET lexer token.
 *
 * This preserves the existing lexical architecture.
 *
 * It also means that:
 *
 *     socket
 *
 * remains available as an ordinary identifier outside socket-declaration
 * contexts according to the language's normal contextual-name rules.
 *
 * ============================================================================
 * DECLARATION FORMS
 * ============================================================================
 *
 * Canonical forms:
 *
 *     socket client;
 *
 *     socket client {
 *         remote: service;
 *     }
 *
 *     socket server {
 *         local: address;
 *         protocol: network::reliable;
 *     }
 *
 *     socket client : SocketType;
 *
 *     socket client : SocketType {
 *         remote: endpoint;
 *     }
 *
 *     socket client = socket_expression;
 *
 *     socket client : SocketType = socket_expression;
 *
 * The semantic layer determines whether a particular combination is valid.
 *
 * ============================================================================
 * DECLARATION SEMANTICS
 * ============================================================================
 *
 * A socket declaration:
 *
 *     - introduces a logical socket identity;
 *     - establishes a source-level communication contract;
 *     - may reference endpoints, addresses, protocols and channels;
 *     - may declare capabilities and requirements;
 *     - may provide implementation-neutral communication properties.
 *
 * It does NOT:
 *
 *     - open a socket;
 *     - bind a file descriptor;
 *     - allocate a port;
 *     - resolve DNS;
 *     - select a NIC;
 *     - connect to a remote host;
 *     - listen on an operating-system socket;
 *     - create a process;
 *     - allocate a node;
 *     - select a transport.
 *
 * ============================================================================
 * GENERIC PROPERTY MODEL
 * ============================================================================
 *
 * Socket members intentionally use an open property namespace.
 *
 * The grammar does NOT reserve parser keywords for:
 *
 *     local
 *     remote
 *     address
 *     endpoint
 *     protocol
 *     transport
 *     channel
 *     mode
 *     family
 *     security
 *     timeout
 *     reliability
 *     ordering
 *     delivery
 *     locality
 *     requires
 *     prefers
 *     provides
 *     metadata
 *
 * These are ordinary identifiers in property position.
 *
 * This allows future networking features without core grammar changes.
 *
 * ============================================================================
 * SOCKET BODY
 * ============================================================================
 *
 * A socket body contains zero or more socket members.
 *
 * Each member is one of:
 *
 *     property
 *     nested extension block
 *
 * This deliberately provides an open extension mechanism.
 *
 * Example:
 *
 *     socket compute {
 *         local: endpoint::worker;
 *         remote: service::compute;
 *         protocol: network::request_response;
 *         transport: network::reliable;
 *         mode: bidirectional;
 *
 *         security {
 *             requires: security::authentication;
 *             requires: security::integrity;
 *         }
 *     }
 *
 * ============================================================================
 * SOCKET PROPERTY
 * ============================================================================
 *
 * Canonical form:
 *
 *     key: expression;
 *
 * Examples:
 *
 *     local: server;
 *     remote: worker;
 *     protocol: network::request_response;
 *     channel: channels::results;
 *     requires: capability("network.reliable");
 *     prefers: networking::low_latency;
 *
 * Property interpretation is semantic.
 *
 * ============================================================================
 * SOCKET NESTED BLOCK
 * ============================================================================
 *
 * Canonical form:
 *
 *     name {
 *         ...
 *     }
 *
 * Nested blocks are semantic extension points.
 *
 * They do not introduce another networking language.
 *
 * ============================================================================
 * SOCKET TYPE
 * ============================================================================
 *
 * A socket may optionally refer to the canonical Zamani type system:
 *
 *     socket connection : NetworkSocket;
 *
 * The type is represented using:
 *
 *     typeExpression
 *
 * This grammar does not define a socket-specific type system.
 *
 * ============================================================================
 * SOCKET INITIALIZATION
 * ============================================================================
 *
 * Initialization uses the canonical expression grammar.
 *
 * Example:
 *
 *     socket connection = make_socket;
 *
 *     socket connection = socket_template("service");
 *
 *     socket connection = existing_socket;
 *
 * The parser does not evaluate the expression.
 *
 * ============================================================================
 * SOCKET REFERENCES
 * ============================================================================
 *
 * A socket reference is a canonical qualified name.
 *
 * Examples:
 *
 *     connection
 *     network::connection
 *     services::compute::socket
 *
 * Name resolution occurs downstream.
 *
 * ============================================================================
 * SOCKET OPERATIONS
 * ============================================================================
 *
 * Socket operations MUST NOT become a second networking expression language.
 *
 * Operations such as:
 *
 *     connect
 *     bind
 *     listen
 *     accept
 *     send
 *     receive
 *     close
 *
 * should normally be represented through the ordinary Zamani expression and
 * call/member-access system.
 *
 * Examples:
 *
 *     connection.connect(remote);
 *
 *     connection.send(message);
 *
 *     connection.receive();
 *
 *     connection.close();
 *
 * This file therefore provides reusable socket-reference rules, but does not
 * create a closed list of socket operation keywords.
 *
 * This is important for:
 *
 *     - future communication operations;
 *     - custom transports;
 *     - vendor-independent libraries;
 *     - domain-specific protocols;
 *     - quantum networking;
 *     - hardware fabrics;
 *     - future computational substrates.
 *
 * ============================================================================
 * LIFECYCLE POLICY
 * ============================================================================
 *
 * A socket declaration may express lifecycle intent using normal properties.
 *
 * Example:
 *
 *     socket worker {
 *         lifecycle: scoped;
 *     }
 *
 *     socket worker {
 *         lifecycle: persistent;
 *     }
 *
 * The parser does not enumerate lifecycle implementations.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Socket requirements may refer to capabilities:
 *
 *     requires: capability("network.reliable");
 *
 *     requires: capability("network.secure");
 *
 *     requires: capability("network.streaming");
 *
 * They may also refer to resource constraints:
 *
 *     requires: bandwidth >= required_bandwidth;
 *
 *     requires: latency <= required_latency;
 *
 * The grammar preserves these as expressions.
 *
 * It does not decide whether they are:
 *
 *     requirements;
 *     constraints;
 *     preferences;
 *     hints;
 *     capabilities.
 *
 * Semantic analysis determines their category.
 *
 * ============================================================================
 * ADDRESS INTEGRATION
 * ============================================================================
 *
 * Socket properties may reference declarations from:
 *
 *     grammar/networking/addresses.g4
 *
 * without copying address grammar.
 *
 * Example:
 *
 *     address compute = network::logical("compute");
 *
 *     socket client {
 *         remote: compute;
 *     }
 *
 * Address validation remains owned by the address semantic layer.
 *
 * ============================================================================
 * ENDPOINT INTEGRATION
 * ============================================================================
 *
 * Socket properties may reference:
 *
 *     grammar/networking/endpoints.g4
 *
 * Example:
 *
 *     endpoint worker;
 *
 *     socket worker_connection {
 *         local: worker;
 *     }
 *
 * This grammar does not validate endpoint existence.
 *
 * ============================================================================
 * PROTOCOL INTEGRATION
 * ============================================================================
 *
 * Socket properties may reference protocol declarations from:
 *
 *     grammar/networking/protocols.g4
 *
 * Example:
 *
 *     protocol RequestResponse;
 *
 *     socket connection {
 *         protocol: RequestResponse;
 *     }
 *
 * The semantic phase resolves and validates the protocol.
 *
 * ============================================================================
 * CHANNEL INTEGRATION
 * ============================================================================
 *
 * Socket properties may reference:
 *
 *     grammar/networking/channels.g4
 *
 * Example:
 *
 *     channel results;
 *
 *     socket result_connection {
 *         channel: results;
 *     }
 *
 * The socket grammar does not duplicate channel syntax.
 *
 * ============================================================================
 * SERVICE INTEGRATION
 * ============================================================================
 *
 * A socket may reference a service identity:
 *
 *     socket client {
 *         remote: service::compute;
 *     }
 *
 * Service declarations remain owned by:
 *
 *     grammar/networking/services.g4
 *
 * ============================================================================
 * MESSAGE INTEGRATION
 * ============================================================================
 *
 * A socket may carry messages defined by:
 *
 *     grammar/networking/messages.g4
 *
 * Example:
 *
 *     socket connection {
 *         message_type: Request;
 *     }
 *
 * The grammar does not redefine message schemas.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Socket use may occur inside:
 *
 *     - async functions;
 *     - tasks;
 *     - actors;
 *     - parallel computations;
 *     - pipelines;
 *     - distributed execution.
 *
 * Concurrency semantics remain owned by:
 *
 *     grammar/concurrency/
 *
 * Socket declarations do not allocate threads.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Socket declarations may participate in distributed programs.
 *
 * Distributed placement remains owned by:
 *
 *     grammar/distributed/
 *
 * A socket does not imply:
 *
 *     node;
 *     machine;
 *     process;
 *     container;
 *     VM;
 *     cluster member.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Security requirements may be expressed through normal property expressions.
 *
 * Example:
 *
 *     socket secure_connection {
 *         requires: security::authentication;
 *         requires: security::confidentiality;
 *         requires: security::integrity;
 *     }
 *
 * This file does not implement cryptography or authentication.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Socket contracts may be used for quantum/classical communication.
 *
 * Examples include:
 *
 *     - remote quantum execution;
 *     - distributed quantum computation;
 *     - quantum networking;
 *     - quantum-classical control;
 *     - distributed QEC workflows;
 *     - accelerator/QPU communication.
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     LogicalQubitId
 *     GateKind
 *     QuantumState
 *     QuantumTopology
 *     Calibration
 *     Pulse
 *     QEC
 *     ZQN
 *
 * If quantum computation is involved, semantic lowering remains connected to:
 *
 *     quantum::ir
 *
 * The socket grammar never creates a second quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A socket contract may describe communication with hardware or an accelerator.
 *
 * Example:
 *
 *     socket accelerator_link {
 *         requires: hardware::communication_fabric;
 *     }
 *
 * Hardware realization belongs to:
 *
 *     grammar/hardware/
 *     grammar/hdl/
 *     compiler/backend/HAL
 *
 * This file does not define:
 *
 *     wires;
 *     pins;
 *     registers;
 *     buses;
 *     clocks;
 *     FPGA resources;
 *     ASIC cells;
 *     physical routing.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Sockets may carry:
 *
 *     - tensors;
 *     - datasets;
 *     - model requests;
 *     - inference results;
 *     - streams;
 *     - distributed training data;
 *     - agent messages.
 *
 * Their data types remain owned by the canonical data/type system.
 *
 * ============================================================================
 * ERROR / EFFECT INTEGRATION
 * ============================================================================
 *
 * Socket operations expressed through ordinary calls must use the canonical
 * Zamani effect/error system.
 *
 * This grammar MUST NOT define an independent socket exception model.
 *
 * Semantic/runtime layers may represent:
 *
 *     unavailable endpoint;
 *     timeout;
 *     connection failure;
 *     capability mismatch;
 *     authentication failure;
 *     protocol failure;
 *     cancellation;
 *     resource exhaustion.
 *
 * ============================================================================
 * SOURCE SPAN REQUIREMENT
 * ============================================================================
 *
 * Every parser context produced by this grammar must retain the source
 * locations supplied by ANTLR.
 *
 * Frontend lowering MUST preserve:
 *
 *     declaration span;
 *     identifier span;
 *     type span;
 *     initializer span;
 *     property-key span;
 *     property-value span;
 *     nested-block span.
 *
 * This is required for deterministic diagnostics and tooling.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST representation MUST preserve at minimum:
 *
 *     - declaration kind;
 *     - socket name;
 *     - optional type;
 *     - optional initializer;
 *     - ordered socket members;
 *     - property names;
 *     - property expressions;
 *     - nested extension blocks;
 *     - source spans;
 *     - attributes.
 *
 * The AST MUST NOT contain:
 *
 *     - opened file descriptors;
 *     - resolved IP addresses;
 *     - selected NICs;
 *     - physical ports;
 *     - physical nodes;
 *     - selected transports;
 *     - routing decisions;
 *     - scheduling decisions;
 *     - hardware allocation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - socket-name resolution;
 *     - duplicate declaration checking;
 *     - type validation;
 *     - initializer validation;
 *     - endpoint reference resolution;
 *     - address reference resolution;
 *     - protocol reference resolution;
 *     - channel reference resolution;
 *     - service reference resolution;
 *     - message compatibility;
 *     - capability analysis;
 *     - resource analysis;
 *     - security analysis;
 *     - lifecycle validation;
 *     - portability validation;
 *     - target feasibility.
 *
 * The parser performs none of these operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO socket IR.
 *
 * The semantic layer may lower socket semantics into the repository's
 * canonical networking semantic representation.
 *
 * Possible downstream representations include:
 *
 *     - networking IR;
 *     - distributed communication representation;
 *     - classical communication representation;
 *     - hardware communication representation;
 *     - accelerator communication representation;
 *     - quantum-adjacent communication metadata.
 *
 * If quantum semantics are involved:
 *
 *     networking socket semantics
 *          ->
 *     semantic analysis
 *          ->
 *     quantum semantic composition
 *          ->
 *     quantum::ir
 *
 * The grammar does not choose the final target representation.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may use socket semantics to:
 *
 *     - validate target capabilities;
 *     - select communication implementations;
 *     - select transport strategies;
 *     - perform placement;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform communication optimization;
 *     - establish security obligations;
 *     - establish resource requirements.
 *
 * The compiler MUST NOT reinterpret a portable socket declaration as a
 * physical implementation merely because one implementation is available.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime owns physical realization.
 *
 * Runtime may:
 *
 *     - allocate communication resources;
 *     - resolve endpoints;
 *     - resolve addresses;
 *     - establish connections;
 *     - select transports;
 *     - negotiate capabilities;
 *     - perform service discovery;
 *     - open OS sockets;
 *     - use IPC;
 *     - use shared memory;
 *     - use specialized fabrics;
 *     - adapt to available resources.
 *
 * None of these operations are performed by this grammar.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * Tooling may use:
 *
 *     socketDeclaration
 *     socketReference
 *     socketMember
 *     socketProperty
 *     socketNestedBlock
 *
 * for:
 *
 *     - syntax highlighting;
 *     - completion;
 *     - navigation;
 *     - reference search;
 *     - diagnostics;
 *     - documentation;
 *     - semantic visualization.
 *
 * Tooling MUST NOT infer a physical socket from syntax alone.
 *
 * ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * The public rules:
 *
 *     socketConstruct
 *     socketDeclaration
 *     socketReference
 *     socketMember
 *
 * are stable integration boundaries.
 *
 * Internal helper rules may evolve provided their accepted source language
 * remains compatible with the language versioning policy.
 *
 * Introducing a new property name MUST NOT require a grammar change because
 * property names are identifiers.
 *
 * Introducing a new protocol, transport, address family, capability, provider,
 * or hardware technology MUST NOT require a grammar change solely because its
 * name is new.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * A socket declaration with an unknown property name remains syntactically
 * valid.
 *
 * Semantic analysis determines whether that property is:
 *
 *     - known;
 *     - versioned;
 *     - dialect-specific;
 *     - deprecated;
 *     - unsupported;
 *     - invalid in the current context.
 *
 * This allows future networking features without parser churn.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no predicates;
 *     - no environment access;
 *     - no I/O;
 *     - no runtime calls;
 *     - no network calls;
 *     - no hardware inspection;
 *     - no randomness.
 *
 * Identical token streams under the same grammar version therefore produce
 * deterministic parse structures.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It imports only canonical reusable parser grammars:
 *
 *     Names
 *     Types
 *     Expressions
 *     Attributes
 *
 * It does NOT import:
 *
 *     networking.g4
 *
 * because networking.g4 is the aggregate consumer of this grammar.
 *
 * This direction avoids an import cycle:
 *
 *     Networking
 *          -> Sockets
 *
 * and never:
 *
 *     Sockets
 *          -> Networking
 *
 * ============================================================================
 * PUBLIC COMPOSITION CONTRACT
 * ============================================================================
 *
 * Higher-level networking composition consumes:
 *
 *     socketConstruct
 *
 * The wider Zamani grammar consumes networking.g4, not this file directly.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar Sockets;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Types, Expressions, Attributes;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A socket construct is a declaration.
 *
 * Socket references are intentionally exposed as reusable helpers rather than
 * being accepted as standalone networking declarations. This prevents an
 * arbitrary identifier/reference expression from being mistaken for a
 * networking declaration.
 * ============================================================================
 */

socketConstruct
    : socketDeclaration
    ;


/*
 * ============================================================================
 * SOCKET DECLARATION
 * ============================================================================
 *
 * Supported forms:
 *
 *     socket name;
 *
 *     socket name { ... }
 *
 *     socket name : Type;
 *
 *     socket name : Type { ... }
 *
 *     socket name = expression;
 *
 *     socket name = expression { ... }
 *
 *     socket name : Type = expression;
 *
 *     socket name : Type = expression { ... }
 *
 * The semantic layer determines whether a particular combination is valid.
 * ============================================================================
 */

socketDeclaration
    : attribute*
      socketMarker
      identifier
      socketTypeAnnotation?
      socketInitializer?
      socketBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * CONTEXTUAL SOCKET MARKER
 * ============================================================================
 *
 * `socket` is deliberately represented by the canonical identifier rule.
 *
 * No SOCKET lexer token is required.
 * ============================================================================
 */

socketMarker
    : identifier
    ;


/*
 * ============================================================================
 * SOCKET TYPE ANNOTATION
 * ============================================================================
 */

socketTypeAnnotation
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * SOCKET INITIALIZER
 * ============================================================================
 */

socketInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * SOCKET BODY
 * ============================================================================
 */

socketBody
    : LBRACE
      socketMember*
      RBRACE
    ;


/*
 * ============================================================================
 * SOCKET MEMBER
 * ============================================================================
 *
 * A member is either:
 *
 *     property
 *
 * or:
 *
 *     nested extension block
 *
 * The property namespace is open.
 * ============================================================================
 */

socketMember
    : socketProperty
    | socketNestedBlock
    ;


/*
 * ============================================================================
 * SOCKET PROPERTY
 * ============================================================================
 *
 * Canonical form:
 *
 *     name: expression;
 *
 * Example:
 *
 *     remote: compute;
 *     protocol: network::request_response;
 *     requires: capability("network.reliable");
 * ============================================================================
 */

socketProperty
    : socketPropertyName
      COLON
      expression
      SEMI
    ;


/*
 * ============================================================================
 * SOCKET PROPERTY NAME
 * ============================================================================
 *
 * Qualified property names permit extension namespaces without introducing
 * global keywords.
 * ============================================================================
 */

socketPropertyName
    : identifier
    | qualifiedName
    ;


/*
 * ============================================================================
 * SOCKET NESTED BLOCK
 * ============================================================================
 *
 * Canonical form:
 *
 *     name {
 *         ...
 *     }
 *
 * The nested block name is intentionally open.
 * ============================================================================
 */

socketNestedBlock
    : socketNestedBlockName
      socketBody
    ;


/*
 * ============================================================================
 * SOCKET NESTED BLOCK NAME
 * ============================================================================
 */

socketNestedBlockName
    : identifier
    | qualifiedName
    ;


/*
 * ============================================================================
 * SOCKET REFERENCE
 * ============================================================================
 *
 * A socket reference is a canonical qualified name.
 *
 * This rule performs no semantic lookup.
 * ============================================================================
 */

socketReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * SOCKET REFERENCE LIST
 * ============================================================================
 *
 * No language-level finite limit is imposed.
 * ============================================================================
 */

socketReferenceList
    : socketReference
      (COMMA socketReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * OPTIONAL SOCKET REFERENCE LIST
 * ============================================================================
 */

optionalSocketReferenceList
    : socketReferenceList?
    ;


/*
 * ============================================================================
 * SOCKET PROPERTY LIST
 * ============================================================================
 */

socketPropertyList
    : socketProperty*
    ;


/*
 * ============================================================================
 * SOCKET MEMBER LIST
 * ============================================================================
 */

socketMemberList
    : socketMember*
    ;


/*
 * ============================================================================
 * SOCKET DECLARATION LIST
 * ============================================================================
 */

socketDeclarationList
    : socketDeclaration*
    ;


/*
 * ============================================================================
 * SOCKET EXPRESSION REFERENCE
 * ============================================================================
 *
 * Allows higher-level grammar components to explicitly identify an expression
 * as a socket reference without creating a socket-specific expression
 * language.
 * ============================================================================
 */

socketExpressionReference
    : socketReference
    ;


/*
 * ============================================================================
 * SOCKET PROPERTY VALUE
 * ============================================================================
 *
 * Dedicated reusable expression boundary.
 * ============================================================================
 */

socketPropertyValue
    : expression
    ;


/*
 * ============================================================================
 * SOCKET ENDPOINT REFERENCE
 * ============================================================================
 *
 * Endpoint identity remains represented by the canonical qualified-name
 * system. Semantic validation determines whether the name resolves to an
 * endpoint.
 * ============================================================================
 */

socketEndpointReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * SOCKET ADDRESS REFERENCE
 * ============================================================================
 *
 * Address identity remains represented by the canonical qualified-name system.
 * ============================================================================
 */

socketAddressReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * SOCKET PROTOCOL REFERENCE
 * ============================================================================
 *
 * Protocol identity remains represented by the canonical qualified-name
 * system.
 * ============================================================================
 */

socketProtocolReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * SOCKET CHANNEL REFERENCE
 * ============================================================================
 *
 * Networking-channel identity remains represented by the canonical
 * qualified-name system.
 * ============================================================================
 */

socketChannelReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * SOCKET SERVICE REFERENCE
 * ============================================================================
 *
 * Service identity remains represented by the canonical qualified-name system.
 * ============================================================================
 */

socketServiceReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * SOCKET MESSAGE TYPE REFERENCE
 * ============================================================================
 *
 * Message types are resolved through the canonical type/name system.
 * ============================================================================
 */

socketMessageTypeReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * SOCKET CAPABILITY REFERENCE
 * ============================================================================
 *
 * Capability identity is intentionally open-world.
 * ============================================================================
 */

socketCapabilityReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * SOCKET CAPABILITY REFERENCE LIST
 * ============================================================================
 */

socketCapabilityReferenceList
    : socketCapabilityReference
      (COMMA socketCapabilityReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * SOCKET DECLARATION REFERENCE
 * ============================================================================
 *
 * Useful to aggregate grammars that need to distinguish a socket declaration
 * boundary without copying its implementation.
 * ============================================================================
 */

socketDeclarationReference
    : socketDeclaration
    ;


/*
 * ============================================================================
 * SOCKET CONFIGURATION
 * ============================================================================
 *
 * Reusable configuration boundary.
 * ============================================================================
 */

socketConfiguration
    : socketBody
    ;


/*
 * ============================================================================
 * SOCKET PROPERTY ASSIGNMENT
 * ============================================================================
 *
 * Reusable boundary for higher-level networking grammars.
 * ============================================================================
 */

socketPropertyAssignment
    : socketPropertyName
      COLON
      socketPropertyValue
      SEMI
    ;


/*
 * ============================================================================
 * SOCKET INITIALIZATION EXPRESSION
 * ============================================================================
 */

socketInitializationExpression
    : expression
    ;


/*
 * ============================================================================
 * SOCKET TYPE REFERENCE
 * ============================================================================
 */

socketTypeReference
    : typeExpression
    ;


/*
 * ============================================================================
 * SOCKET TYPE EXPRESSION
 * ============================================================================
 *
 * Explicit wrapper for consumers that need a socket-related type boundary.
 * ============================================================================
 */

socketTypeExpression
    : typeExpression
    ;


/*
 * ============================================================================
 * SOCKET MEMBER NAME
 * ============================================================================
 */

socketMemberName
    : identifier
    | qualifiedName
    ;


/*
 * ============================================================================
 * SOCKET QUALIFIED NAME
 * ============================================================================
 *
 * Explicit networking-facing wrapper around the canonical name system.
 * ============================================================================
 */

socketQualifiedName
    : qualifiedName
    ;


/*
 * ============================================================================
 * SOCKET LIST
 * ============================================================================
 *
 * Generic reusable list of socket references.
 * ============================================================================
 */

socketReferences
    : socketReferenceList?
    ;


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It has one ownership boundary.
 * [x] It defines logical socket syntax rather than OS socket implementation.
 * [x] It uses the canonical Zamani lexer.
 * [x] It uses canonical names.
 * [x] It uses canonical expressions.
 * [x] It uses the canonical type system.
 * [x] It accepts arbitrary property namespaces.
 * [x] It supports nested extensions.
 * [x] It supports initialization expressions.
 * [x] It supports type annotations.
 * [x] It has no transport enumeration.
 * [x] It has no vendor enumeration.
 * [x] It has no hardware enumeration.
 * [x] It has no physical socket identifiers.
 * [x] It has no fixed endpoint limit.
 * [x] It has no fixed connection limit.
 * [x] It has no fixed network-size limit.
 * [x] It has no parser actions.
 * [x] It has no semantic predicates.
 * [x] It has no unsafe Rust.
 * [x] It creates no IR.
 * [x] It creates no runtime behavior.
 * [x] It preserves the canonical quantum::ir boundary.
 * [x] It supports future communication substrates.
 * [x] It permits target-independent source programs.
 *
 * Remaining repository-level integration is deliberately owned by the
 * aggregate networking grammar and normative networking specification:
 *
 *     grammar/networking/networking.g4
 *     grammar/networking/README.md
 *     grammar/spec/networking.md
 *
 * Those files must reference this component as an imported grammar rather than
 * duplicating its rules.
 *
 * ============================================================================
 * END
 * ============================================================================
 */