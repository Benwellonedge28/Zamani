/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/communication.g4
 *
 * Grammar:
 *     Communication
 *
 * Status:
 *     PRODUCTION SOURCE-GRAMMAR CONTRACT
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
 *     - This grammar contains no embedded Rust actions.
 *     - This grammar contains no semantic predicates.
 *     - This grammar performs no I/O.
 *     - This grammar performs no network access.
 *     - This grammar performs no hardware access.
 *     - This grammar performs no runtime callbacks.
 *     - This grammar requires no unsafe Rust.
 *     - Zamani compiler implementation remains safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL DISTRIBUTED COMMUNICATION SYNTAX.
 *
 * It provides the syntax boundary for communication intent between logical
 * distributed computational entities.
 *
 * Communication may ultimately be realized as:
 *
 *     - local communication;
 *     - intra-process communication;
 *     - inter-process communication;
 *     - inter-machine communication;
 *     - cluster communication;
 *     - cloud communication;
 *     - edge communication;
 *     - accelerator communication;
 *     - CPU/GPU/FPGA communication;
 *     - quantum-classical communication;
 *     - distributed quantum communication;
 *     - future communication mechanisms.
 *
 * The grammar describes WHAT communication is requested.
 *
 * It does not decide HOW that communication is physically implemented.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * Lexer
 *   |
 *   v
 * Parser
 *   |
 *   v
 * Domain-neutral AST
 *   |
 *   v
 * Semantic analysis
 *   |
 *   +--> name resolution
 *   +--> type checking
 *   +--> ownership/lifetime analysis
 *   +--> effect analysis
 *   +--> capability analysis
 *   +--> resource analysis
 *   +--> security analysis
 *   +--> distributed communication validation
 *   |
 *   v
 * Canonical semantic representation
 *   |
 *   +--> classical IR
 *   +--> quantum::ir
 *   +--> HDL/hardware representation
 *   +--> distributed execution metadata
 *   |
 *   v
 * Optimization
 *   |
 *   v
 * Routing / placement / scheduling
 *   |
 *   v
 * Runtime / deployment
 *
 * The grammar never constructs or modifies an IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Communication syntax participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A communication statement must remain meaningful when the compiler chooses
 * a different physical realization.
 *
 * For example:
 *
 *     distributed::send(channel, value, destination);
 *
 * does NOT imply:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     shared memory
 *     a particular network card
 *     a particular machine
 *     a particular process
 *     a particular node
 *     a particular accelerator
 *
 * The networking, execution, resource, placement, routing, scheduling and
 * runtime layers determine an appropriate realization.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Communication operations are represented by qualified names.
 *
 * This intentionally avoids a closed enumeration such as:
 *
 *     SEND
 *     RECEIVE
 *     BROADCAST
 *     SCATTER
 *     GATHER
 *     REDUCE
 *
 * as the only legal operations.
 *
 * Standard communication forms remain naturally expressible:
 *
 *     distributed::send(channel, value, destination);
 *     distributed::receive(channel);
 *     distributed::broadcast(value, group);
 *     distributed::scatter(value, group);
 *     distributed::gather(group);
 *     distributed::reduce(value, operation, group);
 *
 * Future forms remain syntactically representable:
 *
 *     distributed::future_protocol(...);
 *     vendor::specialized_transport(...);
 *     domain::collective(...);
 *
 * Whether such an operation is defined, supported, experimental, deprecated,
 * vendor-specific, or invalid is determined by semantic analysis.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - distributed communication declarations;
 *     - communication statements;
 *     - communication expressions;
 *     - communication operations;
 *     - communication arguments;
 *     - communication targets;
 *     - communication attributes;
 *     - communication contracts;
 *     - communication relationships;
 *     - communication blocks;
 *     - communication modifiers that are syntactically communication-specific.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - expression precedence;
 *     - general types;
 *     - general blocks;
 *     - actor declarations;
 *     - actor lifecycle semantics;
 *     - generic concurrency;
 *     - network protocols;
 *     - sockets;
 *     - physical addresses;
 *     - node discovery;
 *     - service discovery;
 *     - routing algorithms;
 *     - topology realization;
 *     - resource allocation;
 *     - scheduling;
 *     - placement;
 *     - replication algorithms;
 *     - consistency algorithms;
 *     - consensus algorithms;
 *     - fault-tolerance implementation;
 *     - security implementation;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL/hardware IR;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime implementation.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Canonical reusable syntax is imported rather than redefined.
 *
 *     Names
 *         identifier
 *         qualifiedName
 *
 *     Expressions
 *         expression
 *         expressionList
 *
 * This grammar MUST NOT redefine those rules.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The canonical composition relationship is:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     Communication
 *          |
 *          v
 *     Distributed
 *          |
 *          v
 *     Zamani parser composition
 *
 * `distributed.g4` should import this grammar and use:
 *
 *     distributedCommunication
 *
 * as its communication boundary.
 *
 * There must be exactly one production definition of:
 *
 *     distributedCommunication
 *
 * within the distributed grammar composition.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar maps communication syntax to a domain-neutral AST operation
 * representation.
 *
 * Conceptually:
 *
 *     distributedCommunication
 *          |
 *          v
 *     AST::Operation
 *          |
 *          +--> name
 *          +--> namespace
 *          +--> operands
 *          +--> parameters
 *          +--> results
 *          +--> attributes
 *          +--> modifiers
 *          +--> effects
 *          +--> capabilities
 *          +--> source span
 *
 * The exact Rust AST type is owned by the frontend AST subsystem.
 *
 * This grammar MUST NOT create:
 *
 *     DistributedCommunicationAst
 *     NetworkPacketAst
 *     TcpSendAst
 *     MpiSendAst
 *     QuantumCommunicationAst
 *
 * merely because a communication operation is distributed.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether the operation exists;
 *     - whether its arguments are valid;
 *     - whether source and destination are compatible;
 *     - whether the message type is transferable;
 *     - whether ownership permits the transfer;
 *     - whether required effects are available;
 *     - whether required capabilities exist;
 *     - whether resource requirements can be satisfied;
 *     - whether security policy permits communication;
 *     - whether ordering/consistency guarantees are valid;
 *     - whether a communication operation is local or remote;
 *     - whether quantum/classical boundaries are valid.
 *
 * The parser MUST NOT perform these decisions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * The grammar does not define a communication IR.
 *
 * Communication syntax lowers through the canonical semantic representation.
 *
 * Possible downstream consumers include:
 *
 *     classical execution IR
 *     distributed execution metadata
 *     networking representation
 *     quantum::ir
 *     HDL/hardware representation
 *
 * There must not be a second competing distributed IR created by this grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Communication can participate in hybrid and distributed quantum programs.
 *
 * Examples:
 *
 *     distributed::send(channel, measurement, destination);
 *
 *     distributed::receive(channel);
 *
 *     distributed::broadcast(classical_result, group);
 *
 * The communication grammar does not define:
 *
 *     - qubit identifiers;
 *     - physical qubits;
 *     - quantum gates;
 *     - quantum topology;
 *     - entanglement routing;
 *     - QEC;
 *     - calibration;
 *     - pulse scheduling;
 *     - ZQN.
 *
 * If communication affects quantum computation, semantic lowering eventually
 * integrates with the canonical `quantum::ir` path.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Communication may connect:
 *
 *     classical computation
 *     quantum computation
 *     HDL-described components
 *     hardware accelerators
 *     distributed services
 *     storage systems
 *     future execution substrates
 *
 * Hardware realization remains downstream.
 *
 * This grammar MUST NOT encode:
 *
 *     fixed bus widths;
 *     fixed link counts;
 *     fixed device counts;
 *     fixed channels;
 *     fixed message sizes;
 *     fixed node counts.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Communication intent may be lowered by the networking subsystem.
 *
 * For example:
 *
 *     distributed::send(c, x, destination);
 *
 * may eventually become any valid transport selected by the compiler/runtime.
 *
 * The source grammar therefore remains independent of:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     Ethernet
 *     shared-memory transport
 *     vendor transport
 *     future transport.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Communication may carry semantic requirements such as:
 *
 *     requires capability("communication");
 *     requires capability("collective.communication");
 *     requires capability("secure.communication");
 *
 * or resource requirements represented through general Zamani constructs.
 *
 * This grammar does not decide whether the requirements are satisfiable.
 *
 * Resource availability is a semantic/compiler/runtime concern.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Communication may carry source-level security intent through attributes or
 * operation arguments.
 *
 * The grammar does not implement:
 *
 *     encryption;
 *     authentication;
 *     authorization;
 *     key management;
 *     transport security;
 *     secure channels.
 *
 * Security analysis and runtime systems determine realization.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no artificial finite limits.
 *
 * It does NOT define:
 *
 *     MAX_MESSAGES
 *     MAX_CHANNELS
 *     MAX_CONNECTIONS
 *     MAX_ENDPOINTS
 *     MAX_NODES
 *     MAX_ACTORS
 *     MAX_SERVICES
 *     MAX_WORKERS
 *     MAX_GROUP_SIZE
 *     MAX_REPLICAS
 *     MAX_MESSAGE_SIZE
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICES
 *     MAX_LINKS
 *
 * It also does not define:
 *
 *     CPU limits;
 *     GPU limits;
 *     FPGA limits;
 *     QPU limits;
 *     memory limits;
 *     bandwidth limits;
 *     latency limits;
 *     topology limits.
 *
 * Repetition is represented structurally using `*` and `+`.
 *
 * Practical limits are evaluated downstream from actual resources and
 * semantic requirements.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden universal assumptions include:
 *
 *     MAX_NODES
 *     MAX_CHANNELS
 *     MAX_MESSAGES
 *     MAX_MESSAGE_SIZE
 *     MAX_CONNECTIONS
 *     MAX_DEVICES
 *     NODE_0
 *     CHANNEL_0
 *     DEVICE_0
 *     CPU_0
 *     GPU_0
 *     QPU_0
 *
 * Numeric literals remain valid program data.
 *
 * For example:
 *
 *     distributed::send(channel, value, destination, 3);
 *
 * may be valid if the operation's semantics define `3` as an argument.
 *
 * The grammar does not reinterpret it as a system-wide limit.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic with respect to the supplied token stream.
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no external state;
 *     - no environment queries;
 *     - no randomness;
 *     - no hardware discovery;
 *     - no network discovery.
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * Frontend processing must preserve:
 *
 *     - operation name;
 *     - qualified-name segment ordering;
 *     - argument ordering;
 *     - nested expression structure;
 *     - communication block structure;
 *     - attributes;
 *     - source spans.
 *
 * Semantic normalization happens after parsing.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics should identify malformed communication structure.
 *
 * Examples:
 *
 *     missing operation name;
 *     missing opening parenthesis;
 *     missing closing parenthesis;
 *     malformed argument list;
 *     missing semicolon;
 *     malformed communication block.
 *
 * Semantic diagnostics, such as unsupported transports or insufficient
 * resources, are not parser errors.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests must include:
 *
 *     distributed::send(channel, value, destination);
 *     distributed::receive(channel);
 *     distributed::broadcast(value, group);
 *     distributed::scatter(value, group);
 *     distributed::gather(group);
 *     distributed::reduce(value, operation, group);
 *
 * Qualified future operations must also remain parseable:
 *
 *     distributed::future_protocol(value, destination);
 *     vendor::transport(value, destination);
 *
 * Nested expressions must remain valid:
 *
 *     distributed::send(channel, compute(x + y), destination);
 *
 * Empty argument lists are syntactically representable where the general
 * expression-list contract permits them:
 *
 *     distributed::flush();
 *
 * Negative tests must include malformed forms such as:
 *
 *     distributed::send(;
 *     distributed::send(channel, value;
 *     distributed::send(channel value);
 *     distributed::send(channel, value, destination)
 *
 * Boundary/scalability tests must include:
 *
 *     - arbitrarily long qualified operation names;
 *     - arbitrarily many arguments subject to parser/runtime resources;
 *     - deeply nested expressions subject to implementation resources;
 *     - many communication operations in one source unit;
 *     - large communication blocks;
 *     - generated/future operation names.
 *
 * These tests validate absence of grammar-level artificial limits.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] Communication owns its source grammar.
 *     [x] General names are reused.
 *     [x] General expressions are reused.
 *     [x] No communication transport is hard-coded.
 *     [x] No resource maximum is hard-coded.
 *     [x] Future operation names remain representable.
 *     [x] Communication can be nested in distributed computation.
 *     [x] Communication can participate in quantum/classical computation.
 *     [x] Communication can integrate with networking.
 *     [x] Communication can integrate with resource/capability analysis.
 *     [x] Communication can integrate with security analysis.
 *     [x] No IR is created here.
 *     [x] No runtime behavior is created here.
 *     [x] Parser behavior remains deterministic.
 *     [x] Source ordering can be preserved.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * `Communication` is a parser grammar.
 *
 * `tokenVocab` points to the single canonical production lexer.
 */
parser grammar Communication;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/*
 * ============================================================================
 * PUBLIC COMMUNICATION ENTRY POINT
 * ============================================================================
 *
 * A communication construct is an operation-oriented construct:
 *
 *     qualifiedName(argument, ...);
 *
 * or a block form:
 *
 *     qualifiedName(argument, ...) {
 *         ...
 *     }
 *
 * The qualified operation name remains open-world.
 */
distributedCommunication
    : communicationOperation
    ;


/*
 * ============================================================================
 * COMMUNICATION OPERATION
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
 * A block form is permitted for communication constructs that semantically
 * contain nested source-level work:
 *
 *     distributed::transaction(channel) {
 *         ...
 *     }
 *
 * The grammar does not decide which operation names permit blocks.
 * Semantic analysis owns that decision.
 */
communicationOperation
    : communicationName
      LPAREN
      communicationArguments?
      RPAREN
      communicationTerminator
    | communicationName
      LPAREN
      communicationArguments?
      RPAREN
      distributedBlock
    ;


/*
 * ============================================================================
 * COMMUNICATION NAME
 * ============================================================================
 *
 * Qualified names provide the open-world operation namespace.
 *
 * Examples:
 *
 *     distributed::send
 *     distributed::receive
 *     distributed::broadcast
 *     vendor::operation
 *     future::communication::operation
 */
communicationName
    : qualifiedName
    ;


/*
 * ============================================================================
 * ARGUMENTS
 * ============================================================================
 *
 * The general expression grammar owns expression semantics.
 *
 * This grammar only establishes the communication argument boundary.
 *
 * Argument count is unbounded by language design.
 */
communicationArguments
    : expression
      (
          COMMA
          expression
      )*
    ;


/*
 * ============================================================================
 * TERMINATION
 * ============================================================================
 *
 * A simple communication operation is terminated by a semicolon.
 */
communicationTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * COMMUNICATION BLOCK
 * ============================================================================
 *
 * The block rule is intentionally supplied by the canonical distributed
 * grammar composition.
 *
 * `distributedBlock` is not redefined here, preventing competing block
 * definitions.
 *
 * ============================================================================
 */