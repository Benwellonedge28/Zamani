/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/networking/channels.g4
 *
 * Grammar:
 *     NetworkingChannels
 *
 * Status:
 *     PRODUCTION
 *
 * Purpose:
 *     Canonical parser grammar for target-independent logical networking
 *     channels.
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     Zamani parser composition
 *          |
 *          v
 *     NetworkingChannels
 *          |
 *          v
 *     domain-neutral frontend AST
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
 *          +--> hardware communication intent
 *          +--> quantum-related semantic metadata
 *          |
 *          v
 *     canonical IR/lowering
 *          |
 *          +--> classical IR
 *          +--> quantum::ir where quantum computation participates
 *          +--> networking/distributed representation
 *          +--> HDL/hardware representation
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / placement / scheduling
 *          |
 *          v
 *     runtime / HAL / target realization
 *
 * THIS FILE NEVER CONSTRUCTS IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical networking channel declarations;
 *     - channel names;
 *     - channel body structure;
 *     - channel-scoped attributes;
 *     - channel-scoped properties;
 *     - logical source references;
 *     - logical destination references;
 *     - message/schema references;
 *     - protocol references;
 *     - direction intent;
 *     - delivery intent;
 *     - ordering intent;
 *     - reliability intent;
 *     - communication requirements;
 *     - communication constraints;
 *     - communication preferences;
 *     - capability requirements;
 *     - channel-scoped extensible metadata;
 *     - syntactic channel references.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifiers;
 *     - qualified-name syntax;
 *     - general expressions;
 *     - type syntax;
 *     - generic concurrency channels;
 *     - send/receive/select/close operations;
 *     - actor mailboxes;
 *     - endpoint declarations;
 *     - addresses;
 *     - message schema definitions;
 *     - serialization;
 *     - compression;
 *     - cryptography;
 *     - authentication;
 *     - authorization;
 *     - protocol definitions;
 *     - protocol implementations;
 *     - sockets;
 *     - ports;
 *     - IP addressing;
 *     - routing;
 *     - topology;
 *     - placement;
 *     - scheduling;
 *     - discovery;
 *     - distributed membership;
 *     - replication;
 *     - consensus;
 *     - runtime queues;
 *     - memory allocation;
 *     - hardware allocation;
 *     - CPU/GPU/FPGA/QPU selection;
 *     - physical network realization;
 *     - quantum gates;
 *     - quantum states;
 *     - QEC;
 *     - ZQN;
 *     - resilience implementation;
 *     - vendor APIs;
 *     - cloud-provider APIs;
 *     - IR construction;
 *     - runtime execution.
 *
 * ============================================================================
 * RELATED OWNERSHIP
 * ============================================================================
 *
 * Generic concurrency channels:
 *
 *     grammar/concurrency/channels.g4
 *
 * Networking endpoints:
 *
 *     grammar/networking/endpoints.g4
 *
 * Networking addresses:
 *
 *     grammar/networking/addresses.g4
 *
 * Networking messages:
 *
 *     grammar/networking/messages.g4
 *
 * Networking protocols:
 *
 *     grammar/networking/protocols.g4
 *
 * Networking services:
 *
 *     grammar/networking/services.g4
 *
 * Networking capabilities:
 *
 *     grammar/networking/network-capabilities.g4
 *
 * Networking aggregate:
 *
 *     grammar/networking/networking.g4
 *
 * Canonical names:
 *
 *     grammar/core/names.g4
 *
 * Canonical expressions:
 *
 *     grammar/expressions/expressions.g4
 *
 * Canonical types:
 *
 *     grammar/types/types.g4
 *
 * Canonical attributes:
 *
 *     grammar/core/attributes.g4
 *
 * ============================================================================
 * NETWORKING CHANNEL VS CONCURRENCY CHANNEL
 * ============================================================================
 *
 * These are deliberately different abstractions.
 *
 * A CONCURRENCY CHANNEL represents language-level concurrent communication:
 *
 *     Channel<T>
 *
 * and owns operations such as:
 *
 *     send
 *     receive
 *     select
 *     close
 *
 * Those semantics belong to:
 *
 *     grammar/concurrency/channels.g4
 *
 * A NETWORKING CHANNEL represents a logical communication relationship:
 *
 *     channel telemetry {
 *         source: sensor;
 *         destination: collector;
 *         message: telemetry::Measurement;
 *         protocol: reliable;
 *     }
 *
 * This grammar does not perform communication.
 *
 * The networking channel may eventually be realized through:
 *
 *     - in-process communication;
 *     - task communication;
 *     - IPC;
 *     - shared memory;
 *     - accelerator fabrics;
 *     - local networks;
 *     - wide-area networks;
 *     - distributed systems;
 *     - hardware interconnects;
 *     - quantum/classical networks;
 *     - future communication substrates.
 *
 * The realization is downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A networking channel expresses WHAT communication relationship is required.
 *
 * It must not permanently encode WHERE or HOW the relationship is realized.
 *
 * Therefore this grammar does NOT require:
 *
 *     - a machine;
 *     - a node;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a process;
 *     - a thread;
 *     - a socket;
 *     - a NIC;
 *     - an IP address;
 *     - a port;
 *     - a router;
 *     - a switch;
 *     - a subnet;
 *     - a cloud provider;
 *     - a geographic region;
 *     - a fixed topology;
 *     - a fixed transport;
 *     - a fixed network size.
 *
 * A logical channel may therefore survive changes in:
 *
 *     - machine size;
 *     - processor count;
 *     - accelerator count;
 *     - node count;
 *     - network topology;
 *     - transport technology;
 *     - deployment environment;
 *     - hardware generation.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level constants for:
 *
 *     MAX_CHANNELS
 *     MAX_ENDPOINTS
 *     MAX_SENDERS
 *     MAX_RECEIVERS
 *     MAX_MESSAGES
 *     MAX_CHANNEL_MEMBERS
 *     MAX_CHANNEL_PROPERTIES
 *     MAX_NETWORK_SIZE
 *     MAX_NODES
 *     MAX_CONNECTIONS
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_PAYLOAD_SIZE
 *     MAX_TOPOLOGY_SIZE
 *     MAX_PROTOCOLS
 *     MAX_SERVICES
 *
 * Repetition uses ANTLR repetition operators.
 *
 * The language therefore does not establish an artificial finite capacity.
 *
 * Practical limitations may exist in:
 *
 *     - available memory;
 *     - parser implementation;
 *     - compiler resources;
 *     - resource policy;
 *     - runtime resources;
 *     - deployment resources;
 *     - target hardware.
 *
 * Such implementation/resource limitations are NOT source-language limits.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar must never encode:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Networking-specific variants are equally prohibited:
 *
 *     MAX_CHANNELS
 *     MAX_ENDPOINTS
 *     MAX_CONNECTIONS
 *     MAX_PACKET_SIZE
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_LINKS
 *
 * Program data MAY contain numerical values.
 *
 * For example:
 *
 *     payload_limit: 4096;
 *
 * is source-level program intent.
 *
 * It must never be interpreted as a universal grammar limit.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / CAPABILITY
 * ============================================================================
 *
 * These concepts are semantically distinct even though their source structure
 * may be the same.
 *
 * Examples:
 *
 *     requires: bandwidth >= required_bandwidth;
 *     constraint: locality == preferred_region;
 *     prefers: low_latency;
 *     capability: capability("network.reliable");
 *
 * The parser preserves the property and expression.
 *
 * Semantic analysis determines:
 *
 *     requirement
 *     constraint
 *     preference
 *     capability
 *     metadata
 *     policy
 *
 * This grammar does not select hardware or a network provider.
 *
 * ============================================================================
 * OPEN-WORLD NETWORKING
 * ============================================================================
 *
 * This grammar deliberately does NOT enumerate transports:
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
 *
 * as a closed language-level channel vocabulary.
 *
 * A protocol may instead be represented by a canonical name or expression:
 *
 *     protocol: tcp;
 *     protocol: custom::transport;
 *     protocol: capability("network.transport");
 *
 * Semantic analysis determines whether the referenced protocol/capability
 * exists and whether it can satisfy the program.
 *
 * This keeps the grammar open to future communication technologies.
 *
 * ============================================================================
 * PROPERTY MODEL
 * ============================================================================
 *
 * Earlier channel grammar designs separated:
 *
 *     source
 *     destination
 *     message
 *     protocol
 *     direction
 *     delivery
 *     ordering
 *     reliability
 *     requirement
 *     constraint
 *     preference
 *     capability
 *
 * into parser alternatives that all had effectively the same structure:
 *
 *     identifier COLON expression SEMI
 *
 * That is structurally redundant and can create parser ambiguity.
 *
 * This production grammar therefore has ONE canonical property production:
 *
 *     networkChannelProperty
 *         : networkChannelPropertyName
 *           COLON
 *           expression
 *           SEMI
 *         ;
 *
 * Semantic analysis assigns the property its standardized meaning.
 *
 * This gives the language an open property namespace without requiring a new
 * parser rule every time networking gains a new semantic property.
 *
 * ============================================================================
 * STANDARD PROPERTY NAMES
 * ============================================================================
 *
 * The semantic specification may standardize properties including:
 *
 *     source
 *     destination
 *     message
 *     payload
 *     protocol
 *     direction
 *     delivery
 *     ordering
 *     reliability
 *     requires
 *     constraint
 *     constraints
 *     prefers
 *     preference
 *     capability
 *     capabilities
 *     locality
 *     latency
 *     bandwidth
 *     availability
 *     durability
 *     security
 *     priority
 *     policy
 *     metadata
 *
 * These are NOT hard-coded parser keywords.
 *
 * Their semantic meaning belongs to the networking semantic specification.
 *
 * This permits:
 *
 *     future::property
 *     vendor::extension
 *     application::metadata
 *
 * without modifying this grammar.
 *
 * Unknown properties are not automatically semantically valid.
 *
 * Semantic analysis must validate:
 *
 *     - namespace;
 *     - version;
 *     - declaration;
 *     - type;
 *     - capability;
 *     - legality;
 *     - portability;
 *     - compatibility.
 *
 * ============================================================================
 * CONTEXTUAL CHANNEL MARKER
 * ============================================================================
 *
 * The current canonical lexer architecture does not provide a dedicated
 * networking CHANNEL token.
 *
 * Therefore `channel` remains lexically represented through the canonical
 * identifier system.
 *
 * The networking parser receives the declaration in the networking grammar
 * context and the semantic layer validates that the contextual marker has the
 * required canonical spelling.
 *
 * This avoids creating a second lexer authority.
 *
 * If a future language version reserves `channel` as a global lexical keyword,
 * that migration must be performed through the canonical lexer/specification
 * compatibility process. This file's semantic channel model does not change.
 *
 * ============================================================================
 * DECLARATION FORM
 * ============================================================================
 *
 * Canonical declaration:
 *
 *     channel telemetry {
 *         source: sensor;
 *         destination: collector;
 *         message: telemetry::Measurement;
 *         protocol: reliable;
 *     }
 *
 * Empty declaration:
 *
 *     channel telemetry;
 *
 * Optional semicolon after a body:
 *
 *     channel telemetry {
 *         source: sensor;
 *     };
 *
 * No fixed number of properties is imposed.
 *
 * ============================================================================
 * CHANNEL NAME
 * ============================================================================
 *
 * Channel names use canonical identifier syntax.
 *
 * They are not:
 *
 *     - IP addresses;
 *     - sockets;
 *     - ports;
 *     - physical device identifiers;
 *     - machine identifiers.
 *
 * Semantic analysis resolves channel identity and scope.
 *
 * ============================================================================
 * CHANNEL BODY
 * ============================================================================
 *
 * A body contains zero or more channel members.
 *
 * Each member may contain:
 *
 *     - zero or more attributes;
 *     - one property.
 *
 * The body is intentionally property-oriented.
 *
 * Nested configuration can be represented through the existing expression and
 * attribute/type mechanisms where appropriate, without introducing a second
 * networking configuration language.
 *
 * ============================================================================
 * SOURCE / DESTINATION
 * ============================================================================
 *
 * Source and destination values are expressions.
 *
 * This allows logical references such as:
 *
 *     source: sensor;
 *     destination: collector;
 *
 * or:
 *
 *     source: services::sensor;
 *     destination: services::collector;
 *
 * or dynamically resolved values:
 *
 *     source: discover("sensor");
 *
 * The grammar does not determine whether a value ultimately becomes:
 *
 *     - a process;
 *     - service;
 *     - endpoint;
 *     - node;
 *     - accelerator;
 *     - quantum-control participant;
 *     - future execution resource.
 *
 * That belongs to semantic analysis and downstream realization.
 *
 * ============================================================================
 * MESSAGE / PAYLOAD
 * ============================================================================
 *
 * Message/schema references remain expressions.
 *
 * Examples:
 *
 *     message: telemetry::Measurement;
 *     payload: Measurement;
 *     message: schema::Telemetry;
 *
 * The message grammar remains owned by:
 *
 *     grammar/networking/messages.g4
 *
 * Data schemas remain owned by:
 *
 *     grammar/data/
 *
 * Serialization and wire representation remain downstream.
 *
 * ============================================================================
 * PROTOCOL
 * ============================================================================
 *
 * A channel may refer to a protocol using an expression:
 *
 *     protocol: reliable;
 *     protocol: network::reliable;
 *     protocol: selected_protocol;
 *
 * Protocol declarations remain owned by:
 *
 *     grammar/networking/protocols.g4
 *
 * The channel grammar merely records the relationship.
 *
 * ============================================================================
 * DIRECTION
 * ============================================================================
 *
 * Direction is semantic intent.
 *
 * Examples may include:
 *
 *     direction: bidirectional;
 *     direction: source_to_destination;
 *     direction: destination_to_source;
 *
 * The grammar does not enumerate these values.
 *
 * ============================================================================
 * DELIVERY
 * ============================================================================
 *
 * Delivery semantics remain open.
 *
 * Examples may include:
 *
 *     delivery: best_effort;
 *     delivery: at_most_once;
 *     delivery: at_least_once;
 *     delivery: exactly_once;
 *     delivery: custom::delivery;
 *
 * Whether a requested semantic can actually be provided is a semantic/resource
 * question, not a parser question.
 *
 * ============================================================================
 * ORDERING
 * ============================================================================
 *
 * Ordering remains an expression.
 *
 * Examples:
 *
 *     ordering: ordered;
 *     ordering: unordered;
 *     ordering: causal;
 *     ordering: total;
 *     ordering: custom::ordering;
 *
 * No finite ordering vocabulary is required by this grammar.
 *
 * ============================================================================
 * RELIABILITY
 * ============================================================================
 *
 * Reliability remains semantic intent.
 *
 * Examples:
 *
 *     reliability: reliable;
 *     reliability: resilient;
 *     reliability: best_effort;
 *     reliability: custom::reliability;
 *
 * The parser does not promise that the target can satisfy the requirement.
 *
 * ============================================================================
 * RESOURCE SEMANTICS
 * ============================================================================
 *
 * Networking properties may express resource intent:
 *
 *     latency: requirement <= bound;
 *     bandwidth: requirement >= required_bandwidth;
 *     availability: required_availability;
 *
 * These expressions are preserved for semantic/resource analysis.
 *
 * The grammar does not convert them into:
 *
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_CONNECTIONS
 *
 * ============================================================================
 * CAPABILITY SEMANTICS
 * ============================================================================
 *
 * Capability requirements may be expressed through normal expressions:
 *
 *     capability: capability("network.reliable");
 *     requires: capability("network.multicast");
 *     requires: capability("network.encryption");
 *
 * Capability declaration syntax is owned by:
 *
 *     grammar/networking/network-capabilities.g4
 *
 * Capability satisfaction is downstream.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * A channel may carry security intent:
 *
 *     security: secure;
 *     requires: capability("network.confidentiality");
 *
 * This grammar does NOT implement:
 *
 *     - cryptography;
 *     - key management;
 *     - authentication;
 *     - authorization;
 *     - certificate validation;
 *     - identity management;
 *     - trust evaluation.
 *
 * Those remain owned by the security/effects/semantic layers.
 *
 * ============================================================================
 * DISTRIBUTED COMPUTING BOUNDARY
 * ============================================================================
 *
 * A networking channel may be used by distributed computation.
 *
 * Networking owns:
 *
 *     communication relationship.
 *
 * Distributed computing owns:
 *
 *     node membership;
 *     placement;
 *     replication;
 *     partitioning;
 *     distributed execution;
 *     distributed scheduling;
 *     distributed recovery.
 *
 * This grammar does not import distributed placement semantics merely because
 * a channel can cross machine boundaries.
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * A channel may eventually be realized over:
 *
 *     CPU systems;
 *     GPU systems;
 *     FPGA systems;
 *     ASIC systems;
 *     accelerators;
 *     embedded systems;
 *     QPUs;
 *     quantum networks;
 *     HPC systems;
 *     clusters;
 *     clouds;
 *     future computational substrates.
 *
 * None of these are parser-level allocations.
 *
 * There is no:
 *
 *     cpu_channel
 *     gpu_channel
 *     qpu_channel
 *     fpga_channel
 *
 * grammar.
 *
 * Hardware capabilities are resolved downstream.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Networking may participate in hybrid quantum/classical computation.
 *
 * This grammar does NOT define:
 *
 *     Qubit
 *     LogicalQubit
 *     PhysicalQubit
 *     Gate
 *     Circuit
 *     QuantumState
 *     QuantumTopology
 *     Pulse
 *     Calibration
 *     QEC
 *     ZQN
 *
 * If communication participates in quantum computation, semantic lowering may
 * attach the appropriate information to the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * There is no second networking-specific quantum IR.
 *
 * ============================================================================
 * CONCURRENCY BOUNDARY
 * ============================================================================
 *
 * This grammar does not define:
 *
 *     send
 *     receive
 *     select
 *     close
 *     spawn
 *     await
 *
 * as networking-channel operations.
 *
 * Such operations remain ordinary Zamani language/concurrency constructs.
 *
 * A networking channel can be referenced by those constructs after semantic
 * resolution.
 *
 * ============================================================================
 * ADDRESSING BOUNDARY
 * ============================================================================
 *
 * Source and destination expressions may refer to logical addresses.
 *
 * This grammar does not define:
 *
 *     IPv4;
 *     IPv6;
 *     MAC;
 *     DNS;
 *     socket addresses;
 *     physical interfaces.
 *
 * Address declarations remain owned by:
 *
 *     grammar/networking/addresses.g4
 *
 * Physical address realization remains downstream.
 *
 * ============================================================================
 * SOCKET BOUNDARY
 * ============================================================================
 *
 * A channel is not a socket.
 *
 * Socket semantics remain owned by:
 *
 *     grammar/networking/sockets.g4
 *
 * A channel may eventually be realized through sockets, but this grammar does
 * not select or allocate them.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no embedded Rust;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime callbacks;
 *     - no randomness;
 *     - no environment-dependent parsing.
 *
 * Given the same:
 *
 *     source token stream
 *     grammar version
 *     imported grammar versions
 *
 * parsing is deterministic.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar uses the canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * It does not define lexical tokens.
 *
 * Shared syntax is imported through parser grammar components.
 *
 * Canonical concepts consumed here include:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     attribute
 *
 * No networking-specific lexer is introduced.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Names:
 *
 *     identifier
 *     qualifiedName
 *
 * Types:
 *
 *     typeExpression
 *
 * Expressions:
 *
 *     expression
 *
 * Attributes:
 *
 *     attribute
 *
 * These are imported from their canonical parser grammar components.
 *
 * This file must not duplicate those rules.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough structure for the domain-neutral frontend AST
 * to represent:
 *
 *     NetworkChannelDeclaration
 *         attributes
 *         name
 *         members
 *         source_span
 *
 *     NetworkChannelProperty
 *         attributes
 *         name
 *         value
 *         source_span
 *
 *     NetworkChannelReference
 *         qualified_name
 *         source_span
 *
 * The AST must preserve the property name and value rather than prematurely
 * converting properties into networking-specific runtime structures.
 *
 * Recommended conceptual representation:
 *
 *     ChannelDeclaration
 *         name
 *         properties[]
 *         attributes[]
 *         span
 *
 *     ChannelProperty
 *         key
 *         value
 *         attributes[]
 *         span
 *
 * The exact Rust AST type names belong to the frontend AST implementation.
 *
 * This grammar must not depend on Rust implementation details.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - validating contextual `channel` declaration syntax;
 *     - resolving channel names;
 *     - detecting duplicate channel declarations;
 *     - validating scope and visibility;
 *     - resolving source references;
 *     - resolving destination references;
 *     - resolving message/schema references;
 *     - resolving protocol references;
 *     - interpreting standardized properties;
 *     - validating property types;
 *     - validating capability requirements;
 *     - validating resource requirements;
 *     - validating constraints;
 *     - evaluating preferences;
 *     - validating security intent;
 *     - validating distributed interactions;
 *     - checking portability;
 *     - checking satisfiability;
 *     - checking compatibility;
 *     - producing diagnostics.
 *
 * Unknown properties are not automatically accepted semantically merely because
 * the grammar can parse them.
 *
 * Semantic extension mechanisms must establish:
 *
 *     namespace
 *     version
 *     owner
 *     meaning
 *     type
 *     capability requirements
 *     compatibility
 *     lowering
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file has NO direct IR dependency.
 *
 * Correct lowering is:
 *
 *     channel syntax
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic networking model
 *          |
 *          +--> networking representation
 *          +--> distributed representation
 *          +--> classical representation
 *          +--> hardware communication intent
 *          +--> quantum semantic metadata where applicable
 *          |
 *          v
 *     canonical lowering
 *
 * Where quantum computation participates, the established canonical quantum
 * boundary remains:
 *
 *     quantum::ir
 *
 * This grammar must never create:
 *
 *     NetworkingQuantumIR
 *     ChannelQuantumIR
 *     PhysicalNetworkIR
 *
 * merely to represent channel syntax.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler may use channel semantics for:
 *
 *     - communication lowering;
 *     - protocol selection;
 *     - resource analysis;
 *     - capability matching;
 *     - route selection;
 *     - placement;
 *     - scheduling;
 *     - communication optimization;
 *     - deployment planning.
 *
 * None of these decisions occur in this grammar.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime may eventually realize a channel through:
 *
 *     - local memory;
 *     - IPC;
 *     - sockets;
 *     - shared memory;
 *     - hardware fabrics;
 *     - network transports;
 *     - distributed services;
 *     - accelerator interconnects;
 *     - quantum/classical communication infrastructure.
 *
 * The grammar does not instantiate any of them.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource expressions remain symbolic source-level information.
 *
 * Examples:
 *
 *     requires: bandwidth >= required_bandwidth;
 *     requires: latency <= required_latency;
 *     requires: capability("network.reliable");
 *
 * The semantic/resource layers determine whether a realization satisfies them.
 *
 * No physical resource is assumed by the parser.
 *
 * ============================================================================
 * PORTABILITY CONTRACT
 * ============================================================================
 *
 * A valid channel declaration must not inherently depend on:
 *
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     QPU count;
 *     node count;
 *     machine identity;
 *     provider identity;
 *     topology;
 *     socket numbering;
 *     interface numbering;
 *     physical addresses.
 *
 * Portable source describes communication intent.
 *
 * Target-specific realization is downstream.
 *
 * ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * Compatible changes:
 *
 *     - adding semantic properties;
 *     - adding namespaced property meanings;
 *     - adding semantic capabilities;
 *     - adding compatible metadata;
 *     - adding new protocol implementations.
 *
 * Such additions should not require changing this grammar when they fit the
 * existing property/expression model.
 *
 * Grammar-breaking changes must follow:
 *
 *     specification
 *         ->
 *     compatibility
 *         ->
 *     grammar
 *         ->
 *     parser
 *         ->
 *     AST
 *         ->
 *     semantic implementation
 *         ->
 *     tests
 *
 * The grammar must remain synchronized with the repository's compatibility
 * policy.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Syntax errors are reported by the canonical parser/frontend diagnostic layer.
 *
 * Semantic errors include:
 *
 *     - invalid channel declaration;
 *     - unresolved channel reference;
 *     - duplicate channel;
 *     - invalid source;
 *     - invalid destination;
 *     - invalid message;
 *     - invalid protocol;
 *     - invalid property type;
 *     - unsupported capability;
 *     - unsatisfied requirement;
 *     - incompatible constraint;
 *     - invalid security policy;
 *     - incompatible portability requirement.
 *
 * These are semantic errors, not parser actions.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     - network operations;
 *     - filesystem operations;
 *     - process execution;
 *     - environment inspection;
 *     - credential access;
 *     - hardware discovery;
 *     - runtime execution.
 *
 * Generated Rust integration must remain safe Rust.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax tests must include:
 *
 *     channel telemetry;
 *
 *     channel telemetry {
 *         source: sensor;
 *         destination: collector;
 *     }
 *
 *     channel telemetry {
 *         source: sensors::primary;
 *         destination: services::collector;
 *         message: telemetry::Measurement;
 *         protocol: network::reliable;
 *     }
 *
 *     channel quantum_results {
 *         source: quantum::processor;
 *         destination: classical::controller;
 *         message: results::Measurement;
 *         requires: capability("network.reliable");
 *     }
 *
 *     channel data_stream {
 *         source: producer;
 *         destination: consumer;
 *         delivery: at_least_once;
 *         ordering: causal;
 *         reliability: resilient;
 *     }
 *
 *     channel future_transport {
 *         protocol: future::transport;
 *     }
 *
 * Namespaced properties must parse:
 *
 *     vendor::latency: requirement;
 *
 *     application::metadata: value;
 *
 * Attribute-bearing properties must parse:
 *
 *     @metadata
 *     source: sensor;
 *
 * Optional semicolon after a body must parse:
 *
 *     channel telemetry {
 *         source: sensor;
 *     };
 *
 * Empty body must parse:
 *
 *     channel telemetry {}
 *
 * Negative syntax tests must include:
 *
 *     channel;
 *
 *     channel telemetry {
 *
 *     channel telemetry {
 *         source sensor;
 *     }
 *
 *     channel telemetry {
 *         : sensor;
 *     }
 *
 *     channel telemetry {
 *         source: sensor
 *         destination: collector;
 *     }
 *
 *     channel telemetry {
 *         source:;
 *     }
 *
 *     channel telemetry {
 *         source: sensor;;
 *     }
 *
 *     channel telemetry {
 *         source: sensor
 *         destination: collector
 *     }
 *
 * The last case must follow the canonical property terminator policy.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must generate increasingly large channel declarations containing:
 *
 *     - many channels;
 *     - many properties;
 *     - many attributes;
 *     - deeply qualified names;
 *     - large expressions;
 *     - large source/destination collections;
 *     - large program-level channel sets.
 *
 * Tests must verify that this grammar contains no artificial finite networking
 * capacity.
 *
 * Tests must NOT define an expected universal maximum.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Test networking channels with:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL;
 *     hardware intent;
 *     distributed execution;
 *     AI;
 *     data;
 *     security;
 *     concurrency;
 *     effects;
 *     resources;
 *     interoperability.
 *
 * The networking grammar must remain syntactically independent of those
 * domain-specific implementations.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Identical token streams must produce equivalent parse trees repeatedly.
 *
 * Parsing must not depend on:
 *
 *     hardware;
 *     machine size;
 *     network state;
 *     filesystem state;
 *     environment;
 *     time;
 *     randomness;
 *     target availability.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the architectural hard-coding audit when it contains:
 *
 *     [x] no MAX_QUBITS
 *     [x] no MAX_CPUS
 *     [x] no MAX_GPUS
 *     [x] no MAX_FPGAS
 *     [x] no MAX_NODES
 *     [x] no MAX_MEMORY
 *     [x] no MAX_THREADS
 *     [x] no MAX_TENSOR_RANK
 *     [x] no MAX_REGISTER_WIDTH
 *     [x] no MAX_NETWORK_SIZE
 *     [x] no MAX_DEVICE_COUNT
 *     [x] no fixed transport enumeration
 *     [x] no fixed topology
 *     [x] no fixed node count
 *     [x] no physical address requirement
 *     [x] no socket allocation
 *     [x] no hardware selection
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] canonical Zamani lexer vocabulary is consumed;
 *     [x] canonical name grammar is reused;
 *     [x] canonical expression grammar is reused;
 *     [x] canonical type grammar is reusable;
 *     [x] canonical attributes are reusable;
 *     [x] networking channels remain separate from concurrency channels;
 *     [x] channel declaration syntax is defined;
 *     [x] channel body syntax is defined;
 *     [x] channel properties use one canonical structural form;
 *     [x] source is representable;
 *     [x] destination is representable;
 *     [x] message/schema references are representable;
 *     [x] protocol references are representable;
 *     [x] direction is representable;
 *     [x] delivery is representable;
 *     [x] ordering is representable;
 *     [x] reliability is representable;
 *     [x] requirements are representable;
 *     [x] constraints are representable;
 *     [x] preferences are representable;
 *     [x] capabilities are representable;
 *     [x] extensible namespaced properties are representable;
 *     [x] no fixed transport vocabulary exists;
 *     [x] no physical network assumptions exist;
 *     [x] no hardware limits exist;
 *     [x] no machine limits exist;
 *     [x] no topology limits exist;
 *     [x] no parser actions exist;
 *     [x] no semantic predicates exist;
 *     [x] no runtime behavior exists;
 *     [x] no IR is constructed;
 *     [x] quantum::ir remains downstream;
 *     [x] AST integration is defined;
 *     [x] semantic integration is defined;
 *     [x] compiler integration is defined;
 *     [x] runtime integration is defined;
 *     [x] resource integration is defined;
 *     [x] capability integration is defined;
 *     [x] portability is defined;
 *     [x] diagnostics are defined;
 *     [x] compatibility is defined;
 *     [x] scalability is defined;
 *     [x] positive tests are defined;
 *     [x] negative tests are defined;
 *     [x] cross-domain tests are defined;
 *     [x] determinism tests are defined;
 *     [x] safe-Rust integration is defined.
 *
 * ============================================================================
 */

parser grammar NetworkingChannels;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Types, Expressions, Attributes;


/*
 * ============================================================================
 * PUBLIC CHANNEL CONSTRUCT
 * ============================================================================
 *
 * This is the stable integration point consumed by:
 *
 *     grammar/networking/networking.g4
 *
 * It intentionally exposes declaration syntax and a syntactic reference
 * boundary without performing semantic lookup.
 * ============================================================================
 */

networkChannelConstruct
    : networkChannelDeclaration
    | networkChannelReference
    ;


/*
 * ============================================================================
 * CHANNEL DECLARATION
 * ============================================================================
 */

networkChannelDeclaration
    : attribute*
      networkChannelMarker
      identifier
      networkChannelBody?
      SEMI?
    ;


/*
 * ============================================================================
 * CONTEXTUAL CHANNEL MARKER
 * ============================================================================
 *
 * The canonical lexer currently treats `channel` as an identifier.
 *
 * Semantic validation must verify that the marker's source spelling is the
 * canonical channel declaration marker.
 * ============================================================================
 */

networkChannelMarker
    : identifier
    ;


/*
 * ============================================================================
 * CHANNEL BODY
 * ============================================================================
 */

networkChannelBody
    : LBRACE
      networkChannelMember*
      RBRACE
    ;


/*
 * ============================================================================
 * CHANNEL MEMBER
 * ============================================================================
 *
 * This is deliberately ONE structural alternative.
 *
 * Source/destination/protocol/etc. are semantic property categories, not
 * different syntactic structures.
 * ============================================================================
 */

networkChannelMember
    : attribute*
      networkChannelProperty
    ;


/*
 * ============================================================================
 * CANONICAL CHANNEL PROPERTY
 * ============================================================================
 *
 * Examples:
 *
 *     source: sensor;
 *     destination: collector;
 *     message: telemetry::Measurement;
 *     protocol: network::reliable;
 *     delivery: at_least_once;
 *     ordering: causal;
 *     reliability: resilient;
 *     requires: capability("network.reliable");
 *
 * Future properties fit the same structure.
 * ============================================================================
 */

networkChannelProperty
    : networkChannelPropertyName
      COLON
      networkChannelPropertyValue
      SEMI
    ;


/*
 * ============================================================================
 * PROPERTY NAME
 * ============================================================================
 *
 * Qualified names allow namespaced extensions:
 *
 *     network::latency
 *     vendor::transport
 *     application::metadata
 *
 * The parser does not assign semantic meaning to the name.
 * ============================================================================
 */

networkChannelPropertyName
    : qualifiedName
    ;


/*
 * ============================================================================
 * PROPERTY VALUE
 * ============================================================================
 *
 * Property values use the canonical Zamani expression grammar.
 *
 * This prevents networking from creating a second expression language.
 * ============================================================================
 */

networkChannelPropertyValue
    : expression
    ;


/*
 * ============================================================================
 * SOURCE PROPERTY
 * ============================================================================
 *
 * This named wrapper exists for semantic/tooling integration.
 *
 * It delegates to the canonical property structure instead of creating a
 * second syntactic form.
 * ============================================================================
 */

networkChannelSourceProperty
    : networkChannelProperty
    ;


/*
 * ============================================================================
 * DESTINATION PROPERTY
 * ============================================================================
 */

networkChannelDestinationProperty
    : networkChannelProperty
    ;


/*
 * ============================================================================
 * MESSAGE PROPERTY
 * ============================================================================
 */

networkChannelMessageProperty
    : networkChannelProperty
    ;


/*
 * ============================================================================
 * PROTOCOL PROPERTY
 * ============================================================================
 */

networkChannelProtocolProperty
    : networkChannelProperty
    ;


/*
 * ============================================================================
 * DIRECTION PROPERTY
 * ============================================================================
 */

networkChannelDirectionProperty
    : networkChannelProperty
    ;


/*
 * ============================================================================
 * DELIVERY PROPERTY
 * ============================================================================
 */

networkChannelDeliveryProperty
    : networkChannelProperty
    ;


/*
 * ============================================================================
 * ORDERING PROPERTY
 * ============================================================================
 */

networkChannelOrderingProperty
    : networkChannelProperty
    ;


/*
 * ============================================================================
 * RELIABILITY PROPERTY
 * ============================================================================
 */

networkChannelReliabilityProperty
    : networkChannelProperty
    ;


/*
 * ============================================================================
 * REQUIREMENT PROPERTY
 * ============================================================================
 */

networkChannelRequirementProperty
    : networkChannelProperty
    ;


/*
 * ============================================================================
 * CONSTRAINT PROPERTY
 * ============================================================================
 */

networkChannelConstraintProperty
    : networkChannelProperty
    ;


/*
 * ============================================================================
 * PREFERENCE PROPERTY
 * ============================================================================
 */

networkChannelPreferenceProperty
    : networkChannelProperty
    ;


/*
 * ============================================================================
 * CAPABILITY PROPERTY
 * ============================================================================
 */

networkChannelCapabilityProperty
    : networkChannelProperty
    ;


/*
 * ============================================================================
 * CHANNEL REFERENCE
 * ============================================================================
 *
 * This rule performs no name lookup.
 *
 * Examples:
 *
 *     telemetry
 *     services::telemetry
 *     subsystem::network::telemetry
 *
 * Resolution belongs to semantic analysis.
 * ============================================================================
 */

networkChannelReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * CHANNEL REFERENCE LIST
 * ============================================================================
 *
 * No finite number of references is imposed.
 * ============================================================================
 */

networkChannelReferenceList
    : networkChannelReference
      (COMMA networkChannelReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * OPTIONAL CHANNEL REFERENCE LIST
 * ============================================================================
 */

optionalNetworkChannelReferenceList
    : networkChannelReferenceList?
    ;


/*
 * ============================================================================
 * CHANNEL DECLARATION LIST
 * ============================================================================
 *
 * Unbounded by language grammar.
 * ============================================================================
 */

networkChannelDeclarationList
    : networkChannelDeclaration*
    ;


/*
 * ============================================================================
 * CHANNEL PROPERTY LIST
 * ============================================================================
 *
 * Unbounded by language grammar.
 * ============================================================================
 */

networkChannelPropertyList
    : networkChannelProperty*
    ;


/*
 * ============================================================================
 * CHANNEL ITEM
 * ============================================================================
 */

networkChannelItem
    : networkChannelDeclaration
    | networkChannelReference
    ;


/*
 * ============================================================================
 * CHANNEL EXPRESSION
 * ============================================================================
 *
 * This wrapper allows downstream grammar components to explicitly identify
 * a channel reference as a networking-domain expression boundary without
 * introducing a networking-specific expression language.
 * ============================================================================
 */

networkChannelExpression
    : networkChannelReference
    ;


/*
 * ============================================================================
 * CHANNEL TYPE ANNOTATION
 * ============================================================================
 *
 * This is intentionally a generic property-shaped annotation.
 *
 * Example:
 *
 *     payload_type: telemetry::Measurement;
 *
 * The type itself belongs to the canonical type system.
 * ============================================================================
 */

networkChannelTypeAnnotation
    : networkChannelPropertyName
      COLON
      typeExpression
      SEMI
    ;


/*
 * ============================================================================
 * END
 * ============================================================================
 */