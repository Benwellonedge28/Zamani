/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/networking/streaming.g4
 *
 * Grammar:
 *     NetworkingStreaming
 *
 * Status:
 *     Production-target canonical networking streaming grammar.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL LOGICAL STREAM CONTRACTS.
 *
 * A stream represents an ordered or semantically related potentially
 * incremental sequence of values/events/data units communicated between
 * logical computational participants.
 *
 * A stream is a SOURCE-LEVEL COMMUNICATION ABSTRACTION.
 *
 * It is not:
 *
 *     - a socket;
 *     - a TCP connection;
 *     - a UDP flow;
 *     - an HTTP connection;
 *     - a QUIC stream;
 *     - a file descriptor;
 *     - a physical network link;
 *     - a hardware DMA queue;
 *     - a GPU stream;
 *     - a CUDA stream;
 *     - a QPU transport;
 *     - an operating-system resource.
 *
 * Physical realization is determined downstream.
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
 *     NetworkingStreaming
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
 *          +--> networking analysis
 *          +--> distributed analysis
 *          +--> data-flow analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> networking semantics
 *          +--> data semantics
 *          +--> classical semantics
 *          +--> distributed semantics
 *          +--> hybrid semantics
 *          +--> hardware intent
 *          +--> quantum/classical communication metadata
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
 * THIS GRAMMAR NEVER:
 *
 *     - opens a stream;
 *     - closes a stream;
 *     - sends data;
 *     - receives data;
 *     - selects a transport;
 *     - selects a network interface;
 *     - selects a machine;
 *     - selects a CPU/GPU/FPGA/QPU;
 *     - selects a route;
 *     - allocates bandwidth;
 *     - allocates buffers;
 *     - performs flow control;
 *     - performs backpressure;
 *     - performs scheduling;
 *     - performs routing;
 *     - performs serialization;
 *     - performs authentication;
 *     - performs authorization;
 *     - constructs IR;
 *     - accesses hardware.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical stream declarations;
 *     - stream declaration identity;
 *     - stream type intent;
 *     - stream properties;
 *     - stream nested configuration;
 *     - stream references;
 *     - source/sink relationships expressed as stream properties;
 *     - stream protocol relationships expressed as properties;
 *     - stream channel relationships expressed as properties;
 *     - stream request/response relationships expressed as properties;
 *     - stream requirements;
 *     - stream constraints;
 *     - stream capabilities;
 *     - stream preferences;
 *     - stream policies;
 *     - stream metadata;
 *     - stream-level semantic configuration syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - endpoints;
 *     - addresses;
 *     - messages;
 *     - protocols;
 *     - channels;
 *     - requests;
 *     - responses;
 *     - services;
 *     - sockets;
 *     - transport implementations;
 *     - routing;
 *     - scheduling;
 *     - distributed placement;
 *     - hardware topology;
 *     - network topology;
 *     - serialization;
 *     - compression implementation;
 *     - cryptographic implementation;
 *     - authentication;
 *     - authorization;
 *     - runtime stream objects;
 *     - concurrency channels;
 *     - distributed channels;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL.
 *
 * ============================================================================
 * RELATIONSHIP TO EXISTING NETWORKING FILES
 * ============================================================================
 *
 * This grammar complements:
 *
 *     endpoints.g4
 *     addresses.g4
 *     messages.g4
 *     protocols.g4
 *     channels.g4
 *     requests.g4
 *     responses.g4
 *     services.g4
 *     sockets.g4
 *     network-capabilities.g4
 *
 * Ownership is intentionally separated:
 *
 *     endpoints.g4
 *         logical communication participants
 *
 *     addresses.g4
 *         logical/address syntax
 *
 *     messages.g4
 *         message schemas and message values
 *
 *     protocols.g4
 *         protocol contracts
 *
 *     channels.g4
 *         logical networking channels
 *
 *     requests.g4
 *         reusable request contracts
 *
 *     responses.g4
 *         reusable response contracts
 *
 *     services.g4
 *         service contracts
 *
 *     sockets.g4
 *         logical socket contracts
 *
 *     streaming.g4
 *         logical stream contracts
 *
 *     network-capabilities.g4
 *         networking capability declarations
 *
 * Streaming may reference all of those concepts through ordinary names and
 * expressions, but does not duplicate their syntax.
 *
 * ============================================================================
 * STREAM VS CHANNEL
 * ============================================================================
 *
 * A networking channel and a stream are related but distinct concepts.
 *
 * A CHANNEL describes a logical communication relationship.
 *
 * A STREAM describes a logical sequence/data-flow contract that may be
 * realized through a channel, service, socket, shared memory, IPC,
 * distributed fabric, accelerator fabric, or another future substrate.
 *
 * Therefore:
 *
 *     stream
 *         may use
 *             channel
 *
 * but:
 *
 *     stream != channel
 *
 * This file MUST NOT redefine networking-channel syntax.
 *
 * ============================================================================
 * STREAM VS CONCURRENCY CHANNEL
 * ============================================================================
 *
 * Zamani also contains:
 *
 *     grammar/concurrency/channels.g4
 *
 * That grammar owns language-level concurrency channels.
 *
 * This file owns networking stream semantics.
 *
 * A networking stream MAY eventually be implemented using a concurrency
 * primitive, but that realization is downstream.
 *
 * ============================================================================
 * STREAM VS DISTRIBUTED COMMUNICATION
 * ============================================================================
 *
 * A stream can participate in distributed execution.
 *
 * This grammar does not own:
 *
 *     - node membership;
 *     - process placement;
 *     - replication;
 *     - distributed scheduling;
 *     - distributed recovery;
 *     - cluster membership.
 *
 * Those remain downstream/distributed responsibilities.
 *
 * ============================================================================
 * STREAM VS DATA
 * ============================================================================
 *
 * Streaming may carry:
 *
 *     - scalar values;
 *     - records;
 *     - messages;
 *     - tensors;
 *     - datasets;
 *     - events;
 *     - quantum/classical metadata;
 *     - hardware data;
 *     - future data structures.
 *
 * This grammar does not define their schemas.
 *
 * Data/message types are referenced through the canonical type/name system.
 *
 * ============================================================================
 * STREAM VS QUANTUM
 * ============================================================================
 *
 * Streaming may participate in quantum/classical systems.
 *
 * Examples include:
 *
 *     measurement-result streams;
 *     control streams;
 *     telemetry streams;
 *     hybrid computation streams;
 *     distributed quantum-classical communication.
 *
 * This grammar MUST NOT define:
 *
 *     Qubit;
 *     QuantumState;
 *     Gate;
 *     Circuit;
 *     QEC;
 *     ZQN;
 *     physical qubit;
 *     quantum topology.
 *
 * Quantum semantic lowering remains downstream and retains the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A stream expresses logical communication intent.
 *
 * It must not permanently bind the program to:
 *
 *     - one machine;
 *     - one operating system;
 *     - one network;
 *     - one provider;
 *     - one transport;
 *     - one interface;
 *     - one CPU;
 *     - one GPU;
 *     - one FPGA;
 *     - one QPU;
 *     - one node;
 *     - one topology.
 *
 * The same source stream contract may therefore be realized as:
 *
 *     in-process data flow;
 *     shared memory;
 *     IPC;
 *     local messaging;
 *     network communication;
 *     distributed communication;
 *     accelerator communication;
 *     HPC communication;
 *     cloud communication;
 *     edge communication;
 *     future communication substrates.
 *
 * This supports:
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
 * subject to actual semantic requirements, implementation capabilities and
 * available resources.
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
 *     HTTP/2
 *     HTTP/3
 *     MQTT
 *     WebSocket
 *     gRPC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     CUDA
 *     ROCm
 *     vendor transports
 *
 * Such concepts may be represented as ordinary names/expressions.
 *
 * Examples:
 *
 *     protocol: network::stream;
 *
 *     protocol: vendor::future_transport;
 *
 *     capability: capability("network.streaming");
 *
 * Semantic analysis determines whether a referenced protocol or capability
 * exists and whether a realization can satisfy it.
 *
 * ============================================================================
 * PROPERTY MODEL
 * ============================================================================
 *
 * Stream properties intentionally use an open-world structure:
 *
 *     name: expression;
 *
 * This means future streaming concepts do not require parser changes merely
 * because a new property name is introduced.
 *
 * Common semantic property names may include:
 *
 *     type
 *     source
 *     destination
 *     producer
 *     consumer
 *     channel
 *     protocol
 *     request
 *     response
 *     message
 *     ordering
 *     delivery
 *     reliability
 *     durability
 *     replay
 *     retention
 *     buffering
 *     backpressure
 *     flow_control
 *     rate
 *     throughput
 *     latency
 *     timeout
 *     partitioning
 *     affinity
 *     locality
 *     requires
 *     constraint
 *     prefer
 *     capability
 *     security
 *     metadata
 *     observability
 *     provenance
 *
 * These names are NOT hard-coded parser categories.
 *
 * Their semantic meanings are resolved downstream.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE / IMPLEMENTATION SEPARATION
 * ============================================================================
 *
 * Source syntax may express:
 *
 *     requires: capability("network.streaming");
 *
 *     requires: throughput >= required_throughput;
 *
 *     constraint: latency <= maximum_latency;
 *
 *     prefer: locality::near;
 *
 *     capability: capability("network.backpressure");
 *
 *     hint: implementation::buffered;
 *
 * These remain source-level declarations.
 *
 * They do not cause hardware discovery or resource allocation.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes NO language-level maximum on:
 *
 *     streams;
 *     stream members;
 *     nested stream blocks;
 *     qualified-name depth;
 *     source/sink relationships;
 *     stream declarations;
 *     stream references;
 *     stream types;
 *     payload dimensions;
 *     payload size;
 *     event count;
 *     throughput;
 *     rate;
 *     buffer capacity;
 *     topology size;
 *     node count;
 *     endpoint count;
 *     channel count;
 *     device count;
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     QPU count;
 *     memory;
 *     tensor rank.
 *
 * Repetition uses ANTLR `*` / `+`.
 *
 * Any practical limit comes from the implementation, compiler, runtime,
 * deployment environment or available resources. Such limits are not encoded
 * as Zamani language grammar limits.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     MAX_STREAMS
 *     MAX_STREAM_MEMBERS
 *     MAX_STREAM_DEPTH
 *     MAX_STREAM_ELEMENTS
 *     MAX_STREAM_SIZE
 *     MAX_BUFFER_SIZE
 *     MAX_THROUGHPUT
 *     MAX_BANDWIDTH
 *     MAX_LATENCY
 *     MAX_CONNECTIONS
 *     MAX_ENDPOINTS
 *     MAX_CHANNELS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *
 * It also MUST NOT encode:
 *
 *     cpu_0
 *     gpu_0
 *     fpga_0
 *     qpu_0
 *     node_0
 *     interface_0
 *     socket_0
 *     stream_0
 *
 * as special language-level resources.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The canonical declaration marker is:
 *
 *     stream
 *
 * It MUST be represented by the shared lexer vocabulary as:
 *
 *     STREAM
 *
 * This is a language-level syntactic word, not a transport implementation.
 *
 * `Stream` remains available as an ordinary case-sensitive type/name spelling
 * unless separately reserved by the language specification.
 *
 * The grammar defines NO lexer rules.
 *
 * ============================================================================
 * PARSER CONTRACT
 * ============================================================================
 */

parser grammar NetworkingStreaming;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Types, Expressions, Attributes;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The networking aggregate consumes:
 *
 *     networkStreamingConstruct
 *
 * This is the stable composition boundary.
 * ============================================================================
 */

networkStreamingConstruct
    : networkStreamDeclaration
    | networkStreamReference
    ;


/*
 * ============================================================================
 * STREAM DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     stream telemetry;
 *
 *     stream telemetry {
 *         type: Measurement;
 *         source: sensor;
 *         destination: analyzer;
 *     }
 *
 *     stream measurements: Stream<Measurement> {
 *         message: Measurement;
 *         protocol: network::stream;
 *         requires: capability("network.streaming");
 *     }
 *
 * The stream name is qualified-name compatible with the repository naming
 * architecture.
 * ============================================================================
 */

networkStreamDeclaration
    : attribute*
      STREAM
      qualifiedName
      networkStreamTypeAnnotation?
      networkStreamBody?
      SEMI?
    ;


/*
 * ============================================================================
 * STREAM TYPE ANNOTATION
 * ============================================================================
 *
 * The type is owned by the canonical type grammar.
 *
 * This grammar does not define Stream<T> itself.
 *
 * Examples:
 *
 *     stream values: Stream<Value>;
 *
 *     stream measurements: Stream<Measurement>;
 *
 *     stream tensors: Stream<Tensor>;
 *
 * Generic and dependent type semantics remain downstream.
 * ============================================================================
 */

networkStreamTypeAnnotation
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * STREAM BODY
 * ============================================================================
 */

networkStreamBody
    : LBRACE
      networkStreamMember*
      RBRACE
    ;


/*
 * ============================================================================
 * STREAM MEMBER
 * ============================================================================
 *
 * A stream member is either:
 *
 *     property
 *
 * or:
 *
 *     nested configuration block
 *
 * Both structures remain open-world.
 * ============================================================================
 */

networkStreamMember
    : attribute*
      networkStreamProperty
    | attribute*
      networkStreamNestedBlock
    ;


/*
 * ============================================================================
 * STREAM PROPERTY
 * ============================================================================
 *
 * Canonical structure:
 *
 *     name: expression;
 *
 * Examples:
 *
 *     source: sensor;
 *
 *     destination: processor;
 *
 *     message: Measurement;
 *
 *     protocol: network::stream;
 *
 *     channel: telemetry_channel;
 *
 *     ordering: ordered;
 *
 *     delivery: reliable;
 *
 *     requires: capability("network.streaming");
 *
 *     constraint: latency <= target_latency;
 *
 *     prefer: locality::near;
 *
 * Qualified names are accepted so extensions can be introduced without
 * modifying this grammar.
 * ============================================================================
 */

networkStreamProperty
    : networkStreamPropertyName
      COLON
      networkStreamPropertyValue
      SEMI
    ;


/*
 * ============================================================================
 * STREAM PROPERTY NAME
 * ============================================================================
 */

networkStreamPropertyName
    : qualifiedName
    ;


/*
 * ============================================================================
 * STREAM PROPERTY VALUE
 * ============================================================================
 *
 * All property values use the canonical expression language.
 *
 * This avoids creating a second:
 *
 *     - literal language;
 *     - arithmetic language;
 *     - boolean language;
 *     - comparison language;
 *     - function-call language;
 *     - collection language.
 * ============================================================================
 */

networkStreamPropertyValue
    : expression
    ;


/*
 * ============================================================================
 * NESTED STREAM BLOCK
 * ============================================================================
 *
 * Nested blocks permit extensible structures such as:
 *
 *     stream telemetry {
 *         buffering {
 *             policy: adaptive;
 *             capacity: buffer_capacity;
 *         }
 *
 *         flow_control {
 *             mode: dynamic;
 *         }
 *     }
 *
 * The grammar preserves structure.
 *
 * Semantic validation determines whether the nested configuration is legal.
 * ============================================================================
 */

networkStreamNestedBlock
    : networkStreamNestedBlockName
      LBRACE
      networkStreamNestedMember*
      RBRACE
      SEMI?
    ;


networkStreamNestedBlockName
    : qualifiedName
    ;


networkStreamNestedMember
    : attribute*
      networkStreamProperty
    | attribute*
      networkStreamNestedBlock
    ;


/*
 * ============================================================================
 * STREAM REFERENCE
 * ============================================================================
 *
 * A reference names an existing stream declaration.
 *
 * It does not create or open a runtime stream.
 *
 * Example:
 *
 *     network::telemetry
 *
 * Semantic analysis verifies that the referenced symbol is a stream.
 * ============================================================================
 */

networkStreamReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * STABLE SEMANTIC REFERENCE ADAPTERS
 * ============================================================================
 *
 * These wrappers allow downstream networking grammars to depend on stable
 * semantic rule names without copying qualified-name syntax.
 *
 * They add no new syntax.
 * ============================================================================
 */

networkStreamSourceReference
    : qualifiedName
    ;


networkStreamDestinationReference
    : qualifiedName
    ;


networkStreamChannelReference
    : qualifiedName
    ;


networkStreamProtocolReference
    : qualifiedName
    ;


networkStreamMessageReference
    : qualifiedName
    ;


networkStreamRequestReference
    : qualifiedName
    ;


networkStreamResponseReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * STREAM TYPE REFERENCE
 * ============================================================================
 *
 * A stream type reference is a canonical type expression.
 *
 * This adapter exists for downstream tooling and semantic mapping.
 * ============================================================================
 */

networkStreamTypeReference
    : typeExpression
    ;


/*
 * ============================================================================
 * OPTIONAL STREAM BODY
 * ============================================================================
 *
 * Stable wrapper for consumers that need an explicit optional-body boundary.
 * ============================================================================
 */

optionalNetworkStreamBody
    : networkStreamBody?
    ;


/*
 * ============================================================================
 * OPTIONAL STREAM TYPE
 * ============================================================================
 */

optionalNetworkStreamTypeAnnotation
    : networkStreamTypeAnnotation?
    ;


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This grammar is complete when:
 *
 * [x] It has one clear streaming ownership boundary.
 *
 * [x] It uses the canonical Zamani lexer vocabulary.
 *
 * [x] It uses the canonical STREAM token.
 *
 * [x] It imports canonical Names.
 *
 * [x] It imports canonical Types.
 *
 * [x] It imports canonical Expressions.
 *
 * [x] It imports canonical Attributes.
 *
 * [x] It does not define lexer rules.
 *
 * [x] It does not duplicate identifier syntax.
 *
 * [x] It does not duplicate qualified-name syntax.
 *
 * [x] It does not duplicate expression syntax.
 *
 * [x] It does not duplicate type syntax.
 *
 * [x] It does not duplicate attribute syntax.
 *
 * [x] It does not duplicate channel syntax.
 *
 * [x] It does not duplicate message syntax.
 *
 * [x] It does not duplicate protocol syntax.
 *
 * [x] It does not duplicate endpoint syntax.
 *
 * [x] It does not duplicate service syntax.
 *
 * [x] It does not duplicate request syntax.
 *
 * [x] It does not duplicate response syntax.
 *
 * [x] It does not duplicate socket syntax.
 *
 * [x] It remains open-world for stream properties.
 *
 * [x] It supports qualified stream names.
 *
 * [x] It supports typed streams.
 *
 * [x] It supports optional stream bodies.
 *
 * [x] It supports nested stream configuration.
 *
 * [x] It supports arbitrary property values through expressions.
 *
 * [x] It supports attributes.
 *
 * [x] It supports stream references.
 *
 * [x] It does not enumerate network transports.
 *
 * [x] It does not enumerate providers.
 *
 * [x] It does not enumerate devices.
 *
 * [x] It does not enumerate hardware.
 *
 * [x] It does not enumerate stream operations.
 *
 * [x] It does not impose resource limits.
 *
 * [x] It contains no MAX_* resource constants.
 *
 * [x] It contains no physical resource identifiers.
 *
 * [x] It contains no parser actions.
 *
 * [x] It contains no semantic predicates.
 *
 * [x] It contains no runtime behavior.
 *
 * [x] It contains no network behavior.
 *
 * [x] It contains no hardware behavior.
 *
 * [x] It contains no unsafe Rust.
 *
 * [x] It preserves source structure for AST construction.
 *
 * [x] It preserves source spans through ordinary ANTLR parse-tree contexts.
 *
 * [x] It is deterministic for a fixed token stream and grammar version.
 *
 * [x] It can scale syntactically with source size and available implementation
 *     resources.
 *
 * [x] It preserves the canonical quantum::ir boundary.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve at minimum:
 *
 *     stream declaration name
 *     optional stream type
 *     attributes
 *     ordered members
 *     property names
 *     property expressions
 *     nested blocks
 *     stream references
 *     source spans
 *
 * Conceptual representation:
 *
 *     NetworkStreamDecl
 *         name
 *         type
 *         attributes[]
 *         members[]
 *         source_span
 *
 *     NetworkStreamMember
 *         Property
 *         NestedBlock
 *
 *     NetworkStreamProperty
 *         name
 *         value
 *         source_span
 *
 *     NetworkStreamReference
 *         name
 *         source_span
 *
 * Exact Rust AST type names remain owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT create or require a networking-specific competing AST.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     - resolve the stream name;
 *     - resolve referenced types;
 *     - validate stream property names;
 *     - validate property value types;
 *     - resolve endpoint references;
 *     - resolve message references;
 *     - resolve protocol references;
 *     - resolve channel references;
 *     - resolve request/response references;
 *     - validate capability requirements;
 *     - validate resource requirements;
 *     - validate constraints;
 *     - validate preferences;
 *     - validate security requirements;
 *     - determine ordering semantics;
 *     - determine delivery semantics;
 *     - determine flow-control semantics;
 *     - determine backpressure semantics;
 *     - determine lifecycle semantics;
 *     - determine portability;
 *     - reject impossible or unsupported contracts.
 *
 * Parsing alone MUST NOT establish any of those semantic facts.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Streaming syntax lowers through:
 *
 *     parse tree
 *         ->
 *     frontend AST
 *         ->
 *     semantic networking model
 *         ->
 *     canonical semantic representation / networking IR
 *         ->
 *     optimization
 *         ->
 *     routing / placement / scheduling
 *         ->
 *     target realization
 *
 * This grammar MUST NOT create:
 *
 *     StreamIR
 *
 * merely as a parser-owned parallel IR.
 *
 * If a stream requires quantum semantic integration, the downstream semantic
 * representation may contribute metadata to the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * No second quantum IR is permitted.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime responsibilities include:
 *
 *     - stream creation;
 *     - stream opening;
 *     - stream closure;
 *     - buffering;
 *     - flow control;
 *     - backpressure;
 *     - transport realization;
 *     - serialization;
 *     - scheduling;
 *     - routing;
 *     - failure recovery;
 *     - resource allocation;
 *     - security enforcement.
 *
 * None of those behaviors occur during parsing.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * A source program may express:
 *
 *     requires: capacity >= required_capacity;
 *
 *     requires: capability("network.streaming");
 *
 *     requires: capability("network.backpressure");
 *
 *     constraint: latency <= acceptable_latency;
 *
 *     prefer: locality::near;
 *
 * These are semantic requirements/constraints/preferences.
 *
 * They MUST NOT be converted by the parser into physical resource allocation.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Classical:
 *
 *     Stream<T>
 *
 * may carry classical values.
 *
 * Quantum:
 *
 * streams may carry measurement results, control information or other
 * semantically valid quantum/classical communication data.
 *
 * Hybrid:
 *
 * streams may connect classical and quantum computation stages.
 *
 * HDL/hardware:
 *
 * streams may describe logical data-flow between hardware/software components.
 *
 * AI:
 *
 * streams may carry model inputs, inference results, training data or events.
 *
 * Data:
 *
 * streams may carry records, tensors, datasets and incremental transformations.
 *
 * Distributed:
 *
 * streams may span distributed participants.
 *
 * Security:
 *
 * stream properties may express security requirements.
 *
 * All cross-domain meaning remains downstream semantic responsibility.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify:
 *
 *     - malformed stream declaration;
 *     - missing stream name;
 *     - malformed type annotation;
 *     - malformed property;
 *     - malformed nested block;
 *     - missing property value;
 *     - malformed qualified name;
 *     - unexpected token.
 *
 * Semantic diagnostics should identify:
 *
 *     - unknown stream;
 *     - duplicate declaration;
 *     - unknown property;
 *     - invalid property value;
 *     - invalid type;
 *     - invalid endpoint;
 *     - invalid channel;
 *     - invalid protocol;
 *     - invalid message;
 *     - unsatisfied capability;
 *     - unsatisfied resource requirement;
 *     - invalid security requirement;
 *     - incompatible ordering/delivery contract.
 *
 * Syntax and semantic diagnostics MUST remain distinct.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Stream syntax is untrusted source input.
 *
 * Parsing MUST NOT:
 *
 *     - connect to a network;
 *     - open a socket;
 *     - access credentials;
 *     - inspect environment variables;
 *     - access files;
 *     - invoke commands;
 *     - select devices;
 *     - establish trust.
 *
 * A successfully parsed stream declaration does not grant:
 *
 *     - network capability;
 *     - endpoint access;
 *     - authorization;
 *     - credentials;
 *     - hardware access.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * For identical:
 *
 *     source;
 *     language version;
 *     lexer configuration;
 *     parser grammar;
 *     dialect configuration;
 *
 * the parse result MUST be deterministic.
 *
 * Parsing MUST NOT depend on:
 *
 *     - CPU availability;
 *     - GPU availability;
 *     - FPGA availability;
 *     - QPU availability;
 *     - network state;
 *     - filesystem state;
 *     - runtime state;
 *     - wall-clock time;
 *     - randomness;
 *     - hardware topology.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar uses ordinary ANTLR repetition and hierarchical parsing.
 *
 * It MUST NOT introduce:
 *
 *     - parser-time network access;
 *     - parser-time filesystem access;
 *     - parser-time hardware discovery;
 *     - parser-time semantic lookup;
 *     - parser-time resource negotiation.
 *
 * Practical parser memory/time usage is an implementation concern and must
 * not be represented as a Zamani language maximum.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * --------------------------------------------------------------------------
 *
 * stream telemetry;
 *
 * stream telemetry {
 *     type: Measurement;
 *     source: sensor;
 *     destination: analyzer;
 * };
 *
 * stream measurements: Stream<Measurement> {
 *     message: Measurement;
 *     protocol: network::stream;
 *     channel: telemetry;
 * };
 *
 * stream results: Stream<Result> {
 *     source: compute;
 *     destination: collector;
 *     requires: capability("network.streaming");
 * };
 *
 * stream data {
 *     buffering {
 *         policy: adaptive;
 *         capacity: required_capacity;
 *     }
 * };
 *
 * stream vendor::future {
 *     vendor::property: future_value;
 * };
 *
 * stream telemetry {
 *     requires: throughput >= required_throughput;
 *     constraint: latency <= acceptable_latency;
 *     prefer: locality::near;
 * };
 *
 *
 * NEGATIVE TESTS
 * --------------------------------------------------------------------------
 *
 * stream;
 *
 * stream {
 * };
 *
 * stream telemetry: ;
 *
 * stream telemetry {
 *     source:;
 * };
 *
 * stream telemetry {
 *     source
 * };
 *
 * stream telemetry {
 *     source: value
 * };
 *
 * malformed:: {
 * };
 *
 *
 * BOUNDARY TESTS
 * --------------------------------------------------------------------------
 *
 *     empty stream body;
 *     one property;
 *     many properties;
 *     nested blocks;
 *     deeply nested blocks;
 *     long qualified stream names;
 *     long property names;
 *     arbitrary expression values;
 *     typed stream;
 *     untyped stream;
 *     qualified stream name;
 *     stream reference.
 *
 *
 * SCALABILITY TESTS
 * --------------------------------------------------------------------------
 *
 * Test increasingly large source programs containing:
 *
 *     - many stream declarations;
 *     - many stream members;
 *     - many nested configuration blocks;
 *     - long qualified names;
 *     - large expressions;
 *     - large type expressions;
 *     - large mixed networking programs.
 *
 * Tests MUST NOT establish a language maximum.
 *
 * The purpose is to verify that no artificial grammar ceiling exists.
 *
 *
 * DETERMINISM TESTS
 * --------------------------------------------------------------------------
 *
 * Parse identical token streams repeatedly and verify equivalent parse-tree
 * structures and diagnostics.
 *
 *
 * COMPATIBILITY TESTS
 * --------------------------------------------------------------------------
 *
 * Verify coexistence with:
 *
 *     endpoints;
 *     addresses;
 *     channels;
 *     messages;
 *     protocols;
 *     requests;
 *     responses;
 *     services;
 *     sockets;
 *     network capabilities;
 *     classical constructs;
 *     quantum constructs;
 *     hybrid constructs;
 *     HDL constructs;
 *     distributed constructs;
 *     AI constructs;
 *     data constructs.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * The stream grammar describes:
 *
 *     WHAT a logical stream means.
 *
 * It does not describe:
 *
 *     WHERE the stream runs.
 *
 *     WHICH machine runs it.
 *
 *     WHICH network carries it.
 *
 *     WHICH transport realizes it.
 *
 *     WHICH hardware implements it.
 *
 *     HOW routing occurs.
 *
 *     HOW scheduling occurs.
 *
 *     HOW flow control is implemented.
 *
 *     HOW resources are allocated.
 *
 *     HOW security is enforced.
 *
 * Those decisions belong downstream.
 *
 * Therefore this grammar remains compatible with:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     accelerators
 *     HPC
 *     distributed systems
 *     cloud
 *     edge
 *     future computational substrates
 *
 * without introducing a language-level resource ceiling.
 *
 * ============================================================================
 */