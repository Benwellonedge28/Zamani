/*
 * ============================================================================
 * Zamani Universal Programming Language
 * Production Distributed Channel Grammar
 * ============================================================================
 *
 * File:
 *     grammar/distributed/channels.g4
 *
 * Grammar:
 *     DistributedChannels
 *
 * Status:
 *     PRODUCTION-READY DISTRIBUTED CHANNEL DOMAIN CONTRACT
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No randomness.
 *     - No unsafe Rust requirement.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL DISTRIBUTED CHANNEL ADAPTER.
 *
 * A distributed channel is a logical communication abstraction that may be
 * realized:
 *
 *     - locally;
 *     - within one process;
 *     - between processes;
 *     - between machines;
 *     - across clusters;
 *     - across clouds;
 *     - across heterogeneous systems;
 *     - between CPU/GPU/FPGA/ASIC execution domains;
 *     - between classical and quantum execution domains;
 *     - across distributed quantum systems;
 *     - by future communication substrates.
 *
 * This grammar describes communication intent.
 *
 * It does NOT select:
 *
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a node;
 *     - a machine;
 *     - a memory address;
 *     - a queue implementation;
 *     - a network interface;
 *     - a transport protocol;
 *     - a physical route;
 *     - a scheduler;
 *     - a placement;
 *     - a cloud provider;
 *     - a vendor runtime.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         ZAMANI SOURCE
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                       ZamaniParser
 *                              |
 *                              v
 *                   DistributedChannels
 *                              |
 *                              v
 *                      Frontend AST
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *        type analysis    effect analysis   name resolution
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                     capability analysis
 *                              |
 *                              v
 *                       resource analysis
 *                              |
 *                              v
 *                    distributed semantics
 *                              |
 *              +---------------+---------------+
 *              |               |               |
 *              v               v               v
 *         classical        quantum::ir       HDL/
 *           model             model        hardware model
 *              |               |               |
 *              +---------------+---------------+
 *                              |
 *                              v
 *                     optimization/lowering
 *                              |
 *                  +-----------+-----------+
 *                  |           |           |
 *                  v           v           v
 *              placement    routing    scheduling
 *                  |           |           |
 *                  +-----------+-----------+
 *                              |
 *                              v
 *                       networking/runtime
 *                              |
 *                              v
 *                         HAL/target
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     distributed channel declarations;
 *     distributed channel type/shape syntax;
 *     distributed channel endpoints;
 *     distributed channel operations;
 *     distributed channel selection;
 *     distributed channel policies;
 *     distributed channel requirements;
 *     distributed channel lifecycle intent;
 *     distributed channel semantic clauses;
 *     distributed channel parser adapters;
 *     distributed channel source-level integration boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers;
 *     qualified names;
 *     expression precedence;
 *     ordinary expressions;
 *     ordinary types;
 *     generic channels;
 *     generic concurrency;
 *     network protocols;
 *     sockets;
 *     network addresses;
 *     node discovery;
 *     service discovery;
 *     topology;
 *     physical placement;
 *     routing algorithms;
 *     scheduling algorithms;
 *     resource discovery;
 *     hardware discovery;
 *     transport implementation;
 *     replication implementation;
 *     consistency implementation;
 *     consensus algorithms;
 *     fault tolerance;
 *     QEC;
 *     ZQN;
 *     quantum gate semantics;
 *     quantum physical topology;
 *     quantum calibration;
 *     HAL;
 *     runtime implementation;
 *     classical IR;
 *     quantum::ir;
 *     HDL/hardware IR.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION
 * ============================================================================
 *
 * There are three related but distinct channel domains in Zamani:
 *
 *     grammar/concurrency/channels.g4
 *         Generic language-level concurrency channels.
 *
 *     grammar/distributed/channels.g4
 *         Distributed communication intent and distributed-channel semantics.
 *
 *     grammar/networking/channels.g4
 *         Networking/channel realization and network-specific channel intent.
 *
 * These MUST NOT become three competing definitions of one AST/IR concept.
 *
 * The relationship is:
 *
 *     generic channel
 *          |
 *          +--> local concurrency realization
 *          |
 *          +--> distributed realization
 *                    |
 *                    +--> networking realization
 *                    |
 *                    +--> other communication substrate
 *
 * Semantic analysis decides which interpretation applies.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Distributed channel operations are intentionally represented using
 * qualified names rather than a closed list of transport or vendor keywords.
 *
 * Examples:
 *
 *     distributed::send(...)
 *     distributed::receive(...)
 *     distributed::broadcast(...)
 *     distributed::scatter(...)
 *     distributed::gather(...)
 *     distributed::reduce(...)
 *
 * Future operations may be represented without changing this grammar:
 *
 *     distributed::future_operation(...)
 *     distributed::new_collective(...)
 *     vendor::distributed_extension(...)
 *
 * The parser recognizes structure.
 *
 * Semantic analysis determines whether the operation:
 *
 *     - exists;
 *     - is stable;
 *     - is experimental;
 *     - is deprecated;
 *     - is vendor-specific;
 *     - is supported by the selected target;
 *     - requires a capability;
 *     - requires a particular resource;
 *     - is valid for the channel's payload type.
 *
 * ============================================================================
 * NO NEW LEXER KEYWORDS
 * ============================================================================
 *
 * This file deliberately does NOT require new lexical tokens such as:
 *
 *     DISTRIBUTED_CHANNEL
 *     DISTRIBUTED_SEND
 *     DISTRIBUTED_RECEIVE
 *     DISTRIBUTED_SELECT
 *     DISTRIBUTED_BROADCAST
 *     DISTRIBUTED_ENDPOINT
 *     DISTRIBUTED_TRANSPORT
 *
 * Existing Zamani identifiers and qualified names are sufficient.
 *
 * This preserves forward compatibility.
 *
 * In particular:
 *
 *     distributed::channel
 *
 * is represented by the canonical qualifiedName structure.
 *
 * The semantic layer determines that the qualified name denotes the
 * distributed-channel declaration kind.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Canonical reusable parser grammars:
 *
 *     Names
 *         identifier
 *         qualifiedName
 *
 *     Types
 *         typeExpression
 *
 *     Expressions
 *         expression
 *         expressionList
 *         optionalExpressionList
 *
 * This file MUST NOT redefine those rules.
 *
 * ============================================================================
 * TOKEN POLICY
 * ============================================================================
 *
 * This file consumes the canonical Zamani lexer through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Stable tokens used by this grammar include:
 *
 *     IDENTIFIER
 *     DOUBLE_COLON
 *     COLON
 *     ASSIGN
 *     SEMICOLON
 *     COMMA
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     THIN_ARROW
 *     FAT_ARROW
 *
 * No lexical rules are defined here.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Distributed channels participate in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A channel describes a logical communication relationship.
 *
 * The same source program may be realized on:
 *
 *     one execution context;
 *     multiple CPU cores;
 *     multiple processes;
 *     multiple machines;
 *     GPU systems;
 *     FPGA systems;
 *     ASIC systems;
 *     quantum-classical systems;
 *     distributed quantum systems;
 *     HPC systems;
 *     clusters;
 *     clouds;
 *     edge systems;
 *     future computing substrates.
 *
 * Source semantics must not change merely because the realization changes.
 *
 * ============================================================================
 * ABSOLUTE SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO finite language-level limits for:
 *
 *     channels;
 *     endpoints;
 *     senders;
 *     receivers;
 *     participants;
 *     messages;
 *     channel operations;
 *     select arms;
 *     channel nesting;
 *     distributed scopes;
 *     channel declarations;
 *     channel relationships;
 *     payload expressions;
 *     channel policies;
 *     channel requirements.
 *
 * There is deliberately no:
 *
 *     MAX_CHANNELS
 *     MAX_ENDPOINTS
 *     MAX_SENDERS
 *     MAX_RECEIVERS
 *     MAX_MESSAGES
 *     MAX_MESSAGE_SIZE
 *     MAX_CHANNEL_DEPTH
 *     MAX_QUEUE_SIZE
 *     MAX_BUFFER_SIZE
 *     MAX_NODES
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_DEVICES
 *     MAX_NETWORK_SIZE
 *
 * ANTLR repetition operators `*` and `+` represent unbounded language
 * cardinality.
 *
 * Actual limits belong to implementation/resource layers.
 *
 * ============================================================================
 * "INFINITY" DEFINITION
 * ============================================================================
 *
 * "Scale to infinity" means:
 *
 *     the language does not impose an artificial finite machine-size ceiling.
 *
 * It does NOT claim that physical resources are infinite.
 *
 * Real execution remains constrained by available:
 *
 *     memory;
 *     compute;
 *     storage;
 *     bandwidth;
 *     latency;
 *     energy;
 *     hardware;
 *     compiler resources;
 *     runtime resources;
 *     operating-system resources;
 *     deployment policy.
 *
 * Those constraints MUST NOT be promoted into universal grammar constants.
 *
 * ============================================================================
 * REQUIREMENT / REALIZATION SEPARATION
 * ============================================================================
 *
 * A source program may describe:
 *
 *     requirement;
 *     constraint;
 *     capability;
 *     preference;
 *     hint;
 *     budget;
 *     policy.
 *
 * These are semantic categories.
 *
 * For example:
 *
 *     distributed::channel data: Message;
 *
 * may later require a communication capability.
 *
 * It does NOT mean:
 *
 *     use node 0;
 *     use network 0;
 *     use socket 0;
 *     use CPU 0;
 *     use GPU 0.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every accepted distributed-channel construct must map into the existing
 * domain-neutral frontend AST.
 *
 * At minimum, semantic AST information must preserve:
 *
 *     declaration source span;
 *     channel name;
 *     channel payload type;
 *     initializer;
 *     channel body;
 *     endpoint declarations;
 *     endpoint direction metadata;
 *     operation name;
 *     operation arguments;
 *     operation ordering;
 *     select arm ordering;
 *     select operation;
 *     select arm body;
 *     policy clauses;
 *     requirement clauses;
 *     relationship expressions;
 *     lifecycle intent;
 *     source ordering;
 *     source spans.
 *
 * This grammar MUST NOT introduce:
 *
 *     DistributedChannelAst;
 *     NetworkChannelAst;
 *     TcpChannelAst;
 *     MpiChannelAst;
 *     PhysicalChannelAst;
 *     QuantumChannelAst;
 *
 * merely because the source construct is distributed.
 *
 * The existing domain-neutral AST remains authoritative.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - resolution of distributed::channel;
 *     - payload type validation;
 *     - endpoint compatibility;
 *     - ownership and transfer rules;
 *     - borrowing/lifetime rules;
 *     - serializability/transferability;
 *     - operation validity;
 *     - send/receive compatibility;
 *     - select legality;
 *     - policy validity;
 *     - capability requirements;
 *     - resource requirements;
 *     - security requirements;
 *     - ordering semantics;
 *     - delivery semantics;
 *     - reliability semantics;
 *     - distributed execution legality;
 *     - local-vs-remote realization;
 *     - quantum/classical interoperability.
 *
 * Parsing does not determine any of these.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file defines NO channel-specific IR.
 *
 * The semantic channel operation must eventually lower into the existing
 * canonical semantic/IR architecture.
 *
 * Possible realizations include:
 *
 *     local channel;
 *     shared-memory communication;
 *     process IPC;
 *     distributed messaging;
 *     network transport;
 *     accelerator communication;
 *     classical/quantum orchestration;
 *     quantum communication;
 *     hardware communication fabric;
 *     future communication substrate.
 *
 * No source-level channel construct may require a second competing IR.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Generic channel semantics remain owned by:
 *
 *     grammar/concurrency/channels.g4
 *
 * DistributedChannels provides the distributed-domain interpretation.
 *
 * A semantic channel may therefore lower through:
 *
 *     concurrency semantics
 *          |
 *          +--> local realization
 *          |
 *          +--> distributed realization
 *
 * The grammar does not duplicate generic channel ownership.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Distributed channel syntax does not select a network transport.
 *
 * It may eventually lower through:
 *
 *     grammar/networking/channels.g4
 *
 * and related networking grammars.
 *
 * This file MUST NOT encode:
 *
 *     TCP;
 *     UDP;
 *     QUIC;
 *     MPI;
 *     RDMA;
 *     InfiniBand;
 *     Ethernet;
 *     vendor transport;
 *     socket APIs.
 *
 * Networking determines realization.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The parent distributed grammar:
 *
 *     grammar/distributed/distributed.g4
 *
 * must import:
 *
 *     DistributedChannels
 *
 * and use:
 *
 *     distributedChannelDeclaration
 *     distributedChannelStatement
 *     distributedChannelExpression
 *
 * as its channel-specific composition boundary.
 *
 * Existing generic rules named:
 *
 *     distributedChannel
 *     distributedCommunication
 *
 * must be treated as compatibility adapters, not as a second channel grammar.
 *
 * Their stable names may remain, but their implementation should delegate to
 * this file during integration.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A distributed channel may carry values whose semantic type participates in
 * quantum computation.
 *
 * Examples include:
 *
 *     measurement results;
 *     classical control values;
 *     logical-state metadata;
 *     distributed quantum-control information;
 *     domain-defined quantum communication values.
 *
 * This grammar does NOT define:
 *
 *     qubits;
 *     quantum gates;
 *     physical qubit IDs;
 *     quantum topology;
 *     entanglement routing;
 *     calibration;
 *     pulses;
 *     QEC;
 *     ZQN.
 *
 * When the payload is quantum-semantic, the downstream path remains:
 *
 *     frontend AST
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A distributed channel may connect:
 *
 *     software;
 *     hardware modules;
 *     accelerators;
 *     HDL-described components;
 *     classical compute;
 *     quantum compute.
 *
 * The channel grammar remains independent of:
 *
 *     bus width;
 *     register width;
 *     physical link count;
 *     pin count;
 *     device count;
 *     FPGA family;
 *     ASIC process;
 *     accelerator topology.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Security requirements may be represented through generic semantic policy
 * expressions.
 *
 * The grammar does not implement:
 *
 *     encryption;
 *     authentication;
 *     authorization;
 *     key management;
 *     secure transport;
 *     identity verification.
 *
 * Those belong to the security and runtime layers.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token stream;
 *     grammar version;
 *     parser configuration;
 *     explicitly selected dialect configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     time;
 *     randomness;
 *     filesystem state;
 *     network state;
 *     hardware availability;
 *     target availability;
 *     runtime state;
 *     scheduler state.
 *
 * ============================================================================
 * SECURITY OF PARSING
 * ============================================================================
 *
 * Parsing this grammar must never:
 *
 *     create a channel;
 *     allocate a queue;
 *     send a message;
 *     receive a message;
 *     open a socket;
 *     access a node;
 *     inspect hardware;
 *     contact a network;
 *     execute source code;
 *     allocate target resources.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CANONICAL PARSER GRAMMAR
 * ============================================================================
 */

parser grammar DistributedChannels;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Types,
    Expressions
;


/*
 * ============================================================================
 * 1. PUBLIC COMPOSITION ROOTS
 * ============================================================================
 *
 * These are the only public channel-specific entry points parent grammars
 * should consume.
 *
 * A parent parser should not need to know the internal rule structure.
 */

distributedChannelDeclaration
    : distributedChannelDesignator
      identifier
      COLON
      typeExpression
      distributedChannelInitializer?
      distributedChannelBody?
      SEMICOLON?
    ;


distributedChannelStatement
    : distributedChannelOperationStatement
    | distributedChannelSelectStatement
    | distributedChannelPolicyStatement
    | distributedChannelRequirementStatement
    | distributedChannelLifecycleStatement
    ;


distributedChannelExpression
    : distributedChannelInvocation
    | distributedChannelSelectExpression
    ;


distributedChannelMember
    : distributedChannelEndpointDeclaration
    | distributedChannelOperationStatement
    | distributedChannelPolicyStatement
    | distributedChannelRequirementStatement
    | distributedChannelLifecycleStatement
    | distributedChannelRelationshipStatement
    ;


/*
 * ============================================================================
 * 2. CHANNEL DESIGNATOR
 * ============================================================================
 *
 * The source-level canonical declaration shape is:
 *
 *     distributed::channel data: Message;
 *
 *     distributed::channel data: Message = initializer;
 *
 *     distributed::channel data: Message {
 *         ...
 *     }
 *
 * `qualifiedName` is intentionally reused.
 *
 * Semantic analysis MUST validate that the declaration designator denotes the
 * distributed-channel declaration kind.
 *
 * The grammar remains open-world and therefore does not hard-code a special
 * lexer token for "distributed channel".
 */

distributedChannelDesignator
    : qualifiedName
    ;


/*
 * ============================================================================
 * 3. CHANNEL INITIALIZATION
 * ============================================================================
 */

distributedChannelInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 4. CHANNEL BODY
 * ============================================================================
 */

distributedChannelBody
    : LBRACE
      distributedChannelMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 5. ENDPOINT DECLARATIONS
 * ============================================================================
 *
 * Endpoint declarations remain semantic rather than transport-specific.
 *
 * Examples of semantic forms:
 *
 *     endpoint input: Message;
 *     endpoint output: Result;
 *
 * The first qualified name is deliberately open-world.
 *
 * Semantic analysis determines whether the endpoint is:
 *
 *     sender;
 *     receiver;
 *     bidirectional;
 *     request;
 *     response;
 *     stream;
 *     event;
 *     collective;
 *     another future channel role.
 */

distributedChannelEndpointDeclaration
    : qualifiedName
      identifier
      COLON
      typeExpression
      distributedChannelInitializer?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 6. CHANNEL OPERATION
 * ============================================================================
 *
 * General form:
 *
 *     distributed::send(channel, value, destination);
 *
 *     distributed::receive(channel);
 *
 *     distributed::broadcast(value, group);
 *
 *     distributed::scatter(value, group);
 *
 *     distributed::gather(group);
 *
 *     distributed::reduce(value, operation, group);
 *
 * The operation namespace is open-world.
 *
 * No fixed transport operation inventory is encoded.
 */

distributedChannelInvocation
    : distributedChannelOperationName
      LPAREN
      optionalExpressionList
      RPAREN
    ;


distributedChannelOperationName
    : qualifiedName
    ;


distributedChannelOperationStatement
    : distributedChannelInvocation
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 7. ARGUMENT ADAPTERS
 * ============================================================================
 *
 * The arguments remain ordinary Zamani expressions.
 *
 * This is important for:
 *
 *     symbolic destinations;
 *     dynamically computed payloads;
 *     capability expressions;
 *     resource expressions;
 *     generic data;
 *     quantum/classical values;
 *     future domain values.
 */

distributedChannelArguments
    : expressionList
    ;


optionalDistributedChannelArguments
    : optionalExpressionList
    ;


/*
 * ============================================================================
 * 8. SEND / RECEIVE COMPATIBILITY ADAPTERS
 * ============================================================================
 *
 * These rules provide stable semantic names without introducing new lexer
 * tokens.
 *
 * The operation itself remains a normal qualified-name invocation.
 *
 * Semantic validation determines that the operation is the corresponding
 * send/receive operation.
 */

distributedChannelSend
    : distributedChannelInvocation
    ;


distributedChannelReceive
    : distributedChannelInvocation
    ;


distributedChannelBroadcast
    : distributedChannelInvocation
    ;


distributedChannelScatter
    : distributedChannelInvocation
    ;


distributedChannelGather
    : distributedChannelInvocation
    ;


distributedChannelReduce
    : distributedChannelInvocation
    ;


/*
 * ============================================================================
 * 9. SELECT
 * ============================================================================
 *
 * Select is a source-level communication-choice construct.
 *
 * The selector operation remains a qualified-name invocation so future
 * communication mechanisms remain representable.
 *
 * Example:
 *
 *     distributed::select {
 *         distributed::receive(input) => process();
 *         distributed::send(output, value) => continue();
 *     }
 *
 * The parser does not determine which arm wins.
 */

distributedChannelSelectExpression
    : distributedChannelSelectHeader
      LBRACE
      distributedChannelSelectArm*
      RBRACE
    ;


distributedChannelSelectHeader
    : qualifiedName
    ;


distributedChannelSelectArm
    : distributedChannelInvocation
      FAT_ARROW
      distributedChannelSelectBody
      COMMA?
      SEMICOLON?
    ;


distributedChannelSelectBody
    : expression
    | distributedChannelSelectBlock
    ;


distributedChannelSelectBlock
    : LBRACE
      distributedChannelMember*
      RBRACE
    ;


distributedChannelSelectStatement
    : distributedChannelSelectExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 10. DEFAULT SELECT ARM
 * ============================================================================
 *
 * The default arm is represented structurally using a qualified operation
 * name rather than requiring a new DEFAULT token.
 *
 * Example:
 *
 *     distributed::default => fallback();
 *
 * Semantic analysis determines whether the operation name denotes the
 * channel-select default arm.
 */

distributedChannelDefaultSelectArm
    : qualifiedName
      FAT_ARROW
      distributedChannelSelectBody
      COMMA?
      SEMICOLON?
    ;


distributedChannelSelectWithDefaultExpression
    : distributedChannelSelectHeader
      LBRACE
      distributedChannelSelectArm*
      distributedChannelDefaultSelectArm?
      RBRACE
    ;


distributedChannelSelectWithDefaultStatement
    : distributedChannelSelectWithDefaultExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 11. CHANNEL POLICY
 * ============================================================================
 *
 * Policies are declarative semantic intent.
 *
 * Examples:
 *
 *     distributed::ordering = policy;
 *     distributed::delivery = policy;
 *     distributed::reliability = policy;
 *     distributed::consistency = policy;
 *     distributed::security = policy;
 *     distributed::locality = preference;
 *
 * The grammar does not implement the policy.
 */

distributedChannelPolicyStatement
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON?
    ;


distributedChannelPolicyBlock
    : LBRACE
      distributedChannelPolicyStatement*
      RBRACE
    ;


/*
 * ============================================================================
 * 12. REQUIREMENT
 * ============================================================================
 *
 * Requirements remain expressions.
 *
 * They may eventually describe:
 *
 *     capabilities;
 *     resources;
 *     reliability;
 *     latency;
 *     bandwidth;
 *     locality;
 *     security;
 *     ordering;
 *     availability;
 *     transferability.
 *
 * No physical machine limit is encoded here.
 */

distributedChannelRequirementStatement
    : qualifiedName
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON?
    ;


distributedChannelRequirementBlock
    : LBRACE
      distributedChannelRequirementStatement*
      RBRACE
    ;


/*
 * ============================================================================
 * 13. CHANNEL RELATIONSHIPS
 * ============================================================================
 *
 * Relationships express semantic relationships rather than physical links.
 *
 * Examples:
 *
 *     distributed::connect(channel, producer, consumer);
 *     distributed::bind(channel, endpoint);
 *     distributed::attach(channel, service);
 *     distributed::associate(channel, task);
 *
 * The grammar remains open-world.
 */

distributedChannelRelationshipStatement
    : qualifiedName
      LPAREN
      expressionList
      RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 14. CHANNEL LIFECYCLE
 * ============================================================================
 *
 * Lifecycle operations remain semantic operations.
 *
 * Examples:
 *
 *     distributed::open(channel);
 *     distributed::close(channel);
 *     distributed::flush(channel);
 *     distributed::drain(channel);
 *     distributed::suspend(channel);
 *     distributed::resume(channel);
 *
 * No runtime action occurs during parsing.
 */

distributedChannelLifecycleStatement
    : qualifiedName
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 15. CHANNEL CAPACITY / BUFFERING
 * ============================================================================
 *
 * Capacity is an expression, not a grammar constant.
 *
 * Examples:
 *
 *     distributed::capacity = expression;
 *     distributed::buffer = expression;
 *
 * A value may be:
 *
 *     literal;
 *     variable;
 *     symbolic;
 *     computed;
 *     negotiated;
 *     target-dependent.
 *
 * The grammar never turns a capacity into a universal maximum.
 */

distributedChannelCapacityClause
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON?
    ;


distributedChannelBufferClause
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 16. CHANNEL ENDPOINT RELATIONSHIP
 * ============================================================================
 *
 * Endpoint relationships are logical.
 *
 * They do not imply:
 *
 *     node adjacency;
 *     network adjacency;
 *     physical wiring;
 *     socket ownership;
 *     machine placement.
 */

distributedChannelEndpointRelationship
    : qualifiedName
      THIN_ARROW
      qualifiedName
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 17. CHANNEL TARGET LIST
 * ============================================================================
 *
 * A target list is an ordinary expression list.
 *
 * It may represent:
 *
 *     one participant;
 *     many participants;
 *     a dynamically computed group;
 *     a service;
 *     an actor;
 *     a region;
 *     a resource domain;
 *     a future distributed abstraction.
 */

distributedChannelTargetList
    : expressionList
    ;


optionalDistributedChannelTargetList
    : optionalExpressionList
    ;


/*
 * ============================================================================
 * 18. CHANNEL OPERATION WITH TARGETS
 * ============================================================================
 *
 * This adapter gives semantic tooling a stable operation boundary for
 * operations whose final argument or argument group represents participants.
 *
 * The grammar does not decide which argument is a target.
 */

distributedChannelTargetedOperation
    : distributedChannelOperationName
      LPAREN
      expressionList
      RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 19. CHANNEL REQUEST / RESPONSE
 * ============================================================================
 *
 * Request/response is a semantic communication pattern, not a transport.
 *
 * Example:
 *
 *     distributed::request(service, request_value);
 *     distributed::respond(request, response_value);
 */

distributedChannelRequest
    : qualifiedName
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON?
    ;


distributedChannelResponse
    : qualifiedName
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 20. STREAMING
 * ============================================================================
 *
 * Streaming remains semantic intent.
 *
 * The grammar does not define:
 *
 *     TCP streams;
 *     QUIC streams;
 *     GPU streams;
 *     DMA queues;
 *     physical FIFOs.
 */

distributedChannelStream
    : qualifiedName
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 21. COLLECTIVE COMMUNICATION
 * ============================================================================
 *
 * Collective forms remain open-world:
 *
 *     broadcast
 *     scatter
 *     gather
 *     reduce
 *     all_reduce
 *     all_gather
 *     barrier
 *     future_collective
 *
 * No finite collective inventory is imposed.
 */

distributedChannelCollective
    : qualifiedName
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 22. CHANNEL CONFIGURATION
 * ============================================================================
 *
 * Configuration is declarative.
 *
 * Example:
 *
 *     distributed::channel data: Message {
 *         distributed::ordering = ordering_policy;
 *         distributed::delivery = delivery_policy;
 *         distributed::security = security_policy;
 *         distributed::capacity = capacity_expression;
 *     }
 *
 * The grammar records structure only.
 */

distributedChannelConfiguration
    : distributedChannelPolicyBlock
    ;


distributedChannelConfiguredDeclaration
    : distributedChannelDesignator
      identifier
      COLON
      typeExpression
      distributedChannelInitializer?
      distributedChannelConfiguration
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 23. CHANNEL TYPE ADAPTER
 * ============================================================================
 *
 * The payload type remains a canonical Zamani type.
 *
 * This prevents the distributed grammar from creating a second type system.
 */

distributedChannelType
    : typeExpression
    ;


distributedChannelPayloadType
    : typeExpression
    ;


/*
 * ============================================================================
 * 24. CHANNEL REFERENCE
 * ============================================================================
 *
 * Channel references are ordinary expressions.
 *
 * Semantic analysis determines whether the referenced value denotes a
 * distributed channel.
 */

distributedChannelReference
    : expression
    ;


/*
 * ============================================================================
 * 25. CHANNEL NAME
 * ============================================================================
 *
 * No DistributedChannelName token is introduced.
 */

distributedChannelName
    : identifier
    ;


distributedChannelQualifiedName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 26. CHANNEL DECLARATION GROUP
 * ============================================================================
 *
 * No finite declaration count is imposed.
 */

distributedChannelDeclarationGroup
    : distributedChannelDeclaration+
    ;


optionalDistributedChannelDeclarationGroup
    : distributedChannelDeclaration*
    ;


/*
 * ============================================================================
 * 27. CHANNEL MEMBER GROUP
 * ============================================================================
 */

distributedChannelMemberGroup
    : distributedChannelMember*
    ;


distributedChannelNonEmptyMemberGroup
    : distributedChannelMember+
    ;


/*
 * ============================================================================
 * 28. CHANNEL OPERATION GROUP
 * ============================================================================
 */

distributedChannelOperationGroup
    : distributedChannelOperationStatement*
    ;


distributedChannelNonEmptyOperationGroup
    : distributedChannelOperationStatement+
    ;


/*
 * ============================================================================
 * 29. CHANNEL SELECT GROUP
 * ============================================================================
 */

distributedChannelSelectArmGroup
    : distributedChannelSelectArm*
    ;


distributedChannelNonEmptySelectArmGroup
    : distributedChannelSelectArm+
    ;


/*
 * ============================================================================
 * 30. DISTRIBUTED CHANNEL DOMAIN BLOCK
 * ============================================================================
 *
 * This is a reusable integration boundary for parent distributed grammars.
 */

distributedChannelDomainBlock
    : LBRACE
      distributedChannelMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 31. DISTRIBUTED CHANNEL DOMAIN
 * ============================================================================
 *
 * This is intentionally not a competing program root.
 */

distributedChannelDomain
    : distributedChannelDeclaration
    | distributedChannelStatement
    | distributedChannelExpression
    ;


/*
 * ============================================================================
 * 32. COMPATIBILITY ADAPTER: EXISTING distributedChannel
 * ============================================================================
 *
 * The existing grammar/distributed/distributed.g4 already exposes:
 *
 *     distributedChannel
 *
 * That public rule name should remain stable.
 *
 * During integration, its implementation should delegate to this grammar:
 *
 *     distributedChannel
 *         : distributedChannelDeclaration
 *         | distributedChannelStatement
 *         ;
 *
 * No second channel implementation should remain in distributed.g4.
 */

distributedChannelCompatibility
    : distributedChannelDeclaration
    | distributedChannelStatement
    | distributedChannelExpression
    ;


/*
 * ============================================================================
 * 33. COMPATIBILITY ADAPTER: COMMUNICATION
 * ============================================================================
 *
 * Existing distributed communication syntax is represented by:
 *
 *     distributed::send(...)
 *     distributed::receive(...)
 *     distributed::broadcast(...)
 *     distributed::scatter(...)
 *     distributed::gather(...)
 *     distributed::reduce(...)
 *
 * This file keeps those forms structurally compatible while moving channel
 * ownership into one distributed channel grammar.
 */

distributedChannelCommunication
    : distributedChannelInvocation
    ;


distributedChannelCommunicationStatement
    : distributedChannelCommunication
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 34. RESOURCE / CAPABILITY ADAPTER
 * ============================================================================
 *
 * Resource and capability semantics remain owned by grammar/resources/.
 *
 * This grammar only carries their source-level expression boundary.
 */

distributedChannelCapabilityRequirement
    : qualifiedName
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON?
    ;


distributedChannelResourceRequirement
    : qualifiedName
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON?
    ;


distributedChannelConstraint
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON?
    ;


distributedChannelPreference
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON?
    ;


distributedChannelHint
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 35. SECURITY ADAPTER
 * ============================================================================
 *
 * Security semantics remain owned by grammar/security/.
 */

distributedChannelSecurityClause
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 36. RELIABILITY ADAPTER
 * ============================================================================
 *
 * Reliability is intent, not an implementation algorithm.
 */

distributedChannelReliabilityClause
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 37. ORDERING ADAPTER
 * ============================================================================
 */

distributedChannelOrderingClause
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 38. DELIVERY ADAPTER
 * ============================================================================
 */

distributedChannelDeliveryClause
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 39. LOCALITY ADAPTER
 * ============================================================================
 */

distributedChannelLocalityClause
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 40. AVAILABILITY ADAPTER
 * ============================================================================
 */

distributedChannelAvailabilityClause
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 41. FINAL SEMANTIC MEMBER UNION
 * ============================================================================
 *
 * This is the stable internal union.
 *
 * New semantic channel features should normally be added here first rather
 * than duplicated across several parent grammars.
 */

distributedChannelSemanticMember
    : distributedChannelEndpointDeclaration
    | distributedChannelOperationStatement
    | distributedChannelPolicyStatement
    | distributedChannelRequirementStatement
    | distributedChannelLifecycleStatement
    | distributedChannelRelationshipStatement
    | distributedChannelEndpointRelationship
    | distributedChannelCapacityClause
    | distributedChannelBufferClause
    | distributedChannelCapabilityRequirement
    | distributedChannelResourceRequirement
    | distributedChannelConstraint
    | distributedChannelPreference
    | distributedChannelHint
    | distributedChannelSecurityClause
    | distributedChannelReliabilityClause
    | distributedChannelOrderingClause
    | distributedChannelDeliveryClause
    | distributedChannelLocalityClause
    | distributedChannelAvailabilityClause
    ;


/*
 * ============================================================================
 * 42. SEMANTIC CHANNEL BODY
 * ============================================================================
 */

distributedChannelSemanticBody
    : LBRACE
      distributedChannelSemanticMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 43. SEMANTIC CHANNEL DECLARATION
 * ============================================================================
 */

distributedChannelSemanticDeclaration
    : distributedChannelDesignator
      distributedChannelName
      COLON
      distributedChannelPayloadType
      distributedChannelInitializer?
      distributedChannelSemanticBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 44. COMPLETE CHANNEL CONSTRUCT
 * ============================================================================
 *
 * This is the preferred stable leaf-level integration rule.
 */

distributedChannelConstruct
    : distributedChannelSemanticDeclaration
    | distributedChannelStatement
    | distributedChannelExpression
    ;


/*
 * ============================================================================
 * 45. COMPLETE CHANNEL PROGRAM FRAGMENT
 * ============================================================================
 *
 * This is deliberately a fragment, not a second program root.
 */

distributedChannelFragment
    : distributedChannelConstruct*
    ;


/*
 * ============================================================================
 * 46. AST / SEMANTIC TRACEABILITY
 * ============================================================================
 *
 * The intended traceability is:
 *
 *     distributedChannelSemanticDeclaration
 *          |
 *          v
 *     domain-neutral declaration AST
 *          |
 *          v
 *     semantic channel model
 *          |
 *          +--> payload type
 *          +--> endpoints
 *          +--> operations
 *          +--> policies
 *          +--> requirements
 *          +--> capabilities
 *          +--> source span
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical execution
 *          +--> distributed execution
 *          +--> networking
 *          +--> quantum::ir when quantum semantics participate
 *          +--> HDL/hardware realization when applicable
 *
 * This grammar MUST NOT require a channel-specific IR.
 *
 * ============================================================================
 * 47. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level diagnostics include:
 *
 *     missing channel designator;
 *     missing channel name;
 *     missing colon;
 *     malformed type;
 *     malformed initializer;
 *     malformed body;
 *     missing parenthesis;
 *     missing argument separator;
 *     malformed select arm;
 *     missing FAT_ARROW;
 *     malformed operation;
 *     malformed policy;
 *     malformed requirement;
 *     malformed relationship.
 *
 * Semantic diagnostics include:
 *
 *     unresolved channel;
 *     invalid channel kind;
 *     invalid payload type;
 *     invalid endpoint;
 *     incompatible endpoint;
 *     invalid communication operation;
 *     unsupported operation;
 *     invalid policy;
 *     unsatisfied capability;
 *     unsatisfied resource requirement;
 *     security violation;
 *     invalid ownership transfer;
 *     invalid quantum/classical transfer;
 *     invalid distributed realization.
 *
 * Resource exhaustion MUST NOT be reported as a syntax error.
 *
 * ============================================================================
 * 48. ERROR BOUNDARY
 * ============================================================================
 *
 * The grammar must distinguish:
 *
 *     syntactically invalid
 *
 * from:
 *
 *     syntactically valid but semantically invalid
 *
 * from:
 *
 *     semantically valid but resource-infeasible
 *
 * from:
 *
 *     resource-feasible but target-unsupported.
 *
 * This distinction is mandatory for POCO-REAF.
 *
 * ============================================================================
 * 49. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Identical:
 *
 *     source;
 *     token stream;
 *     grammar version;
 *     parser configuration;
 *
 * must yield the same parse structure.
 *
 * The grammar does not depend on:
 *
 *     resource availability;
 *     target hardware;
 *     network state;
 *     runtime state;
 *     scheduler state;
 *     time;
 *     randomness.
 *
 * ============================================================================
 * 50. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST remain free of:
 *
 *     MAX_CHANNELS
 *     MAX_ENDPOINTS
 *     MAX_SENDERS
 *     MAX_RECEIVERS
 *     MAX_MESSAGES
 *     MAX_QUEUE_SIZE
 *     MAX_BUFFER_SIZE
 *     MAX_NODES
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_DEVICES
 *     MAX_NETWORK_SIZE
 *     MAX_TOPOLOGY_SIZE
 *
 * It also MUST NOT encode:
 *
 *     physical node IDs;
 *     socket IDs;
 *     device IDs;
 *     memory addresses;
 *     fixed topology;
 *     fixed transport.
 *
 * Program values remain valid:
 *
 *     let capacity = derive_capacity();
 *
 * The existence of a source value does not create a language-level resource
 * limit.
 *
 * ============================================================================
 * 51. TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * --------------------------------------------------------------------------
 *
 *     distributed::channel input: Message;
 *
 *     distributed::channel output: Result = initial_channel();
 *
 *     distributed::channel data: Tensor<Message> {
 *         distributed::capacity = capacity_expression;
 *         distributed::delivery = delivery_policy;
 *     }
 *
 *     distributed::send(data, value, destination);
 *
 *     distributed::receive(data);
 *
 *     distributed::broadcast(value, group);
 *
 *     distributed::scatter(value, group);
 *
 *     distributed::gather(group);
 *
 *     distributed::reduce(value, operation, group);
 *
 *
 * SELECT
 * --------------------------------------------------------------------------
 *
 *     distributed::select {
 *         distributed::receive(input) => process(input);
 *         distributed::send(output, value) => continue_work();
 *     }
 *
 *
 * SELECT WITH DEFAULT
 * --------------------------------------------------------------------------
 *
 *     distributed::select {
 *         distributed::receive(input) => process(input);
 *         distributed::default => fallback();
 *     }
 *
 *
 * NESTED BODY
 * --------------------------------------------------------------------------
 *
 *     distributed::channel messages: Message {
 *         distributed::ordering = ordering_policy;
 *         distributed::reliability = reliability_policy;
 *         distributed::security = security_policy;
 *
 *         distributed::send(messages, value, destination);
 *         distributed::receive(messages);
 *     }
 *
 *
 * SYMBOLIC VALUES
 * --------------------------------------------------------------------------
 *
 *     distributed::channel data: Payload {
 *         distributed::capacity = compute_capacity();
 *     }
 *
 *
 * QUANTUM/CLASSICAL
 * --------------------------------------------------------------------------
 *
 *     distributed::channel results: Measurement;
 *
 *     distributed::send(results, measurement, classical_destination);
 *
 *
 * FUTURE EXTENSIONS
 * --------------------------------------------------------------------------
 *
 *     vendor::distributed_extension(channel, payload);
 *
 *     distributed::future_collective(channel, group);
 *
 * These remain syntactically representable.
 *
 *
 * NEGATIVE TESTS
 * --------------------------------------------------------------------------
 *
 *     distributed::channel;
 *
 *     distributed::channel data;
 *
 *     distributed::channel data:;
 *
 *     distributed::channel data: ;
 *
 *     distributed::channel data: Message =
 *
 *     distributed::send(
 *
 *     distributed::receive(
 *
 *     distributed::select {
 *         distributed::receive(input)
 *     }
 *
 *
 * BOUNDARY TESTS
 * --------------------------------------------------------------------------
 *
 *     one channel;
 *     many channels;
 *     one endpoint;
 *     many endpoints;
 *     one operation;
 *     many operations;
 *     one select arm;
 *     many select arms;
 *     nested channel bodies;
 *     deeply qualified operation names;
 *     deeply qualified channel names;
 *     symbolic capacity;
 *     computed target lists;
 *     empty optional argument lists;
 *     large expression lists.
 *
 *
 * SCALABILITY TESTS
 * --------------------------------------------------------------------------
 *
 * Verify that the grammar imposes no fixed limits on:
 *
 *     channel declarations;
 *     endpoints;
 *     operations;
 *     select arms;
 *     participants;
 *     nested scopes;
 *     payload expressions;
 *     qualified-name depth.
 *
 *
 * DETERMINISM TESTS
 * --------------------------------------------------------------------------
 *
 * Same source + same grammar version + same token configuration
 *     =>
 * same parse tree.
 *
 *
 * HARDWARE-INDEPENDENCE TESTS
 * --------------------------------------------------------------------------
 *
 * The same source syntax must remain parseable independently of:
 *
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     QPU count;
 *     node count;
 *     memory capacity;
 *     network topology;
 *     device availability.
 *
 * ============================================================================
 * 52. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing distributed source forms such as:
 *
 *     distributed::send(...);
 *     distributed::receive(...);
 *     distributed::broadcast(...);
 *     distributed::scatter(...);
 *     distributed::gather(...);
 *     distributed::reduce(...);
 *
 * remain structurally representable.
 *
 * Existing public rule:
 *
 *     distributedChannel
 *
 * remains available through the parent Distributed grammar as a compatibility
 * adapter.
 *
 * Existing generic channel grammar:
 *
 *     grammar/concurrency/channels.g4
 *
 * remains independently usable.
 *
 * Existing networking channel grammar:
 *
 *     grammar/networking/channels.g4
 *
 * remains independently usable.
 *
 * No source-level rename is required merely to introduce this file.
 *
 * ============================================================================
 * 53. INTEGRATION CHECKLIST
 * ============================================================================
 *
 * PARENT GRAMMAR
 * --------------------------------------------------------------------------
 *
 * grammar/distributed/distributed.g4 must:
 *
 *     [ ] import DistributedChannels;
 *     [ ] retain public rule distributedChannel;
 *     [ ] delegate channel declarations to this file;
 *     [ ] delegate channel statements to this file;
 *     [ ] delegate channel expressions to this file;
 *     [ ] stop maintaining a second independent channel implementation.
 *
 *
 * DISTRIBUTED COMMUNICATION
 * --------------------------------------------------------------------------
 *
 * grammar/distributed/communication.g4 must:
 *
 *     [ ] remain the generic distributed communication contract;
 *     [ ] use distributedChannelInvocation where appropriate;
 *     [ ] not duplicate channel declaration syntax;
 *     [ ] not create a second distributed channel AST.
 *
 *
 * MESSAGING
 * --------------------------------------------------------------------------
 *
 * grammar/distributed/messaging.g4 must:
 *
 *     [ ] remain responsible for message structure/schema;
 *     [ ] treat channel payloads as ordinary type/expression references;
 *     [ ] not define another channel type.
 *
 *
 * CONCURRENCY
 * --------------------------------------------------------------------------
 *
 * grammar/concurrency/channels.g4 must:
 *
 *     [ ] remain the generic channel grammar;
 *     [ ] not be changed merely to introduce distributed-channel semantics;
 *     [ ] lower into the same semantic channel model when applicable.
 *
 *
 * NETWORKING
 * --------------------------------------------------------------------------
 *
 * grammar/networking/channels.g4 must:
 *
 *     [ ] remain responsible for network-specific channel realization;
 *     [ ] not become the owner of distributed semantic channel declarations;
 *     [ ] receive distributed channel intent only through semantic lowering.
 *
 *
 * RESOURCES
 * --------------------------------------------------------------------------
 *
 * grammar/resources/ must:
 *
 *     [ ] own resource/capability semantics;
 *     [ ] determine resource feasibility;
 *     [ ] never inherit fixed channel limits from this grammar.
 *
 *
 * EXECUTION
 * --------------------------------------------------------------------------
 *
 * grammar/execution/ must:
 *
 *     [ ] own runtime/execution intent;
 *     [ ] consume channel semantics after analysis;
 *     [ ] never require source-level physical placement.
 *
 *
 * QUANTUM
 * --------------------------------------------------------------------------
 *
 * grammar/quantum/ must:
 *
 *     [ ] retain quantum::ir as canonical quantum semantic boundary;
 *     [ ] never create DistributedChannelIR;
 *     [ ] validate quantum payload semantics downstream.
 *
 * ============================================================================
 * 54. REQUIRED PARENT INTEGRATION
 * ============================================================================
 *
 * The parent grammar should contain the following composition:
 *
 *     parser grammar Distributed;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 *     import
 *         Names,
 *         Expressions,
 *         DistributedChannels
 *     ;
 *
 * Then the existing distributed channel adapter should become:
 *
 *     distributedChannel
 *         : distributedChannelDeclaration
 *         | distributedChannelStatement
 *         ;
 *
 * and channel-specific expression integration should use:
 *
 *     distributedChannelExpression
 *
 * rather than duplicating operation syntax.
 *
 * The exact parent-rule placement is deliberately left to Distributed's
 * composition layer; this leaf grammar must not import its parent, which would
 * create a dependency cycle.
 *
 * ============================================================================
 * 55. NO CIRCULAR DEPENDENCY
 * ============================================================================
 *
 * Dependency direction is:
 *
 *     Names
 *       ^
 *     Types
 *       ^
 *     Expressions
 *       ^
 *     DistributedChannels
 *       ^
 *     Distributed
 *       ^
 *     ZamaniParser
 *
 * Never:
 *
 *     DistributedChannels -> Distributed
 *
 * Such an import would create a grammar cycle.
 *
 * ============================================================================
 * 56. RUST INTEGRATION
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * Generated parser consumers remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The implementation must remain:
 *
 *     safe Rust;
 *     no unsafe blocks;
 *     no unsafe functions;
 *     no unsafe traits;
 *     no hidden unsafe requirement introduced by this grammar.
 *
 * The grammar itself cannot guarantee implementation-wide absence of unsafe;
 * that remains a repository build/CI invariant.
 *
 * ============================================================================
 * 57. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The frontend must preserve source spans for:
 *
 *     channel declaration;
 *     channel name;
 *     channel payload type;
 *     initializer;
 *     body;
 *     endpoint;
 *     operation;
 *     operation arguments;
 *     select;
 *     select arm;
 *     policy;
 *     requirement;
 *     lifecycle construct.
 *
 * The grammar must not discard syntactic structure required for diagnostics,
 * formatting, IDE/LSP tooling, provenance, or compatibility analysis.
 *
 * ============================================================================
 * 58. PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar must avoid unnecessary semantic lookups during parsing.
 *
 * In particular:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no target probing;
 *     - no filesystem probing;
 *     - no network probing;
 *     - no hardware probing.
 *
 * Open-world operation names are represented as ordinary qualified names.
 *
 * Parent grammars should invoke the narrowest applicable public rule to avoid
 * unnecessary ambiguity between generic distributed operations and channel
 * constructs.
 *
 * ============================================================================
 * 59. SECURITY CONTRACT
 * ============================================================================
 *
 * The parser must never execute:
 *
 *     operation names;
 *     resource expressions;
 *     policy expressions;
 *     capability expressions;
 *     target expressions.
 *
 * All expressions are syntax only at this stage.
 *
 * ============================================================================
 * 60. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] file owns distributed channel source syntax;
 *     [x] no new lexer keyword is required;
 *     [x] canonical Names grammar is reused;
 *     [x] canonical Types grammar is reused;
 *     [x] canonical Expressions grammar is reused;
 *     [x] no generic identifier grammar is duplicated;
 *     [x] no second channel IR is created;
 *     [x] no network transport is hard-coded;
 *     [x] no hardware topology is hard-coded;
 *     [x] no finite channel/resource limit exists;
 *     [x] select has unbounded arm cardinality;
 *     [x] operation names remain open-world;
 *     [x] channel payloads remain ordinary Zamani types;
 *     [x] policies remain expressions;
 *     [x] requirements remain expressions;
 *     [x] quantum integration preserves quantum::ir;
 *     [x] networking remains downstream;
 *     [x] concurrency remains separately owned;
 *     [x] AST mapping is defined;
 *     [x] semantic ownership is defined;
 *     [x] IR integration is defined;
 *     [x] diagnostics are defined;
 *     [x] scalability tests are defined;
 *     [x] compatibility adapters are defined;
 *     [x] Rust 1.97/1.97.1 integration is defined;
 *     [x] no unsafe Rust is required.
 *
 * ============================================================================
 * END OF grammar/distributed/channels.g4
 * ============================================================================
 */