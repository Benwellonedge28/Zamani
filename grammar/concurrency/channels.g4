/*
 * ============================================================================
 * Zamani Programming Language
 * Production Channel-Concurrency Grammar
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/channels.g4
 *
 * Grammar:
 *     Channels
 *
 * Status:
 *     PRODUCTION-READY MODULAR CHANNEL GRAMMAR
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the source-level syntax for logical communication channels.
 *
 * A Zamani channel expresses a communication relationship between computations.
 *
 * A channel does NOT inherently represent:
 *
 *     - a thread;
 *     - a CPU core;
 *     - a worker;
 *     - a process;
 *     - a GPU;
 *     - an FPGA;
 *     - a QPU;
 *     - a physical network socket;
 *     - a physical memory queue;
 *     - a node;
 *     - a machine;
 *     - a device;
 *     - a particular transport protocol.
 *
 * The language describes communication intent.
 *
 * Actual realization is determined downstream by:
 *
 *     semantic analysis
 *     resource/capability analysis
 *     canonical IR
 *     scheduling
 *     placement
 *     distributed execution
 *     networking
 *     runtime
 *     HAL
 *     target lowering
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     channelExpression
 *     channelStatement
 *     channelDeclaration
 *     channelType
 *     channelDirection
 *     channel construction
 *     send
 *     receive
 *     try-send
 *     try-receive
 *     close
 *     select
 *     select arms
 *     channel endpoints
 *     channel-specific parser adapters
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers
 *     names
 *     paths
 *     ordinary expressions
 *     expression precedence
 *     function calls
 *     ordinary types
 *     generic type semantics
 *     blocks
 *     general statements
 *     task scheduling
 *     synchronization implementation
 *     memory allocation
 *     buffering implementation
 *     queue implementation
 *     fairness
 *     deadlock detection
 *     cancellation implementation
 *     networking implementation
 *     distributed placement
 *     hardware discovery
 *     target selection
 *     classical IR
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     calibration
 *     HAL
 *     runtime implementation
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * The grammar deliberately imposes NO finite language-level limit on:
 *
 *     channels
 *     senders
 *     receivers
 *     messages
 *     channel operations
 *     select arms
 *     concurrent communication relationships
 *     payload sizes
 *     nesting
 *     channel declarations
 *     channel construction expressions
 *
 * There are no:
 *
 *     MAX_CHANNELS
 *     MAX_MESSAGES
 *     MAX_SENDERS
 *     MAX_RECEIVERS
 *     MAX_QUEUE_SIZE
 *     MAX_BUFFER_SIZE
 *     MAX_NODES
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *
 * A program may therefore express communication for:
 *
 *     a tiny embedded system
 *     a single execution context
 *     multicore CPUs
 *     GPUs
 *     accelerators
 *     FPGA/ASIC systems
 *     distributed systems
 *     HPC systems
 *     heterogeneous systems
 *     quantum/classical orchestration
 *     future computational substrates
 *
 * "Infinity" means that the language grammar introduces no artificial
 * hardware-scale ceiling. Actual execution remains subject to available
 * resources and semantic requirements.
 *
 * ============================================================================
 * REQUIREMENT / IMPLEMENTATION SEPARATION
 * ============================================================================
 *
 * A channel source program expresses:
 *
 *     WHAT communication means.
 *
 * It does not specify:
 *
 *     HOW MANY queues exist;
 *     WHICH thread owns a queue;
 *     WHICH CPU executes a sender;
 *     WHICH GPU executes a receiver;
 *     WHICH node transports a message;
 *     WHICH network link is selected;
 *     WHICH physical memory address is used.
 *
 * Those are downstream implementation decisions.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical Zamani lexer:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Parser dependencies:
 *
 *     Expressions
 *         -> expression
 *
 *     Types
 *         -> typeExpression
 *
 *     Calls
 *         -> argumentList
 *
 *     ZamaniExpressionBlocks
 *         -> blockExpression
 *
 * The channel grammar MUST NOT redefine any of these.
 *
 * The imports below make the channel grammar independently understandable
 * and provide its dependencies before it is composed into Concurrency.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Parsing produces syntax information only.
 *
 * The frontend AST must preserve at minimum:
 *
 *     - channel declaration source span;
 *     - channel type;
 *     - endpoint direction;
 *     - constructor arguments;
 *     - send channel expression;
 *     - send payload expression;
 *     - receive channel expression;
 *     - close channel expression;
 *     - try/non-blocking intent;
 *     - select arm ordering;
 *     - select operation;
 *     - select body;
 *     - default arm;
 *     - source ordering;
 *     - source spans.
 *
 * The AST must use the repository's existing domain-neutral AST architecture.
 *
 * This grammar MUST NOT introduce:
 *
 *     ChannelIR
 *     ChannelRuntimeNode
 *     PhysicalQueueNode
 *     NetworkChannelNode
 *     HardwareChannelNode
 *
 * or any second channel-specific IR.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - whether an expression is actually a channel;
 *     - payload type compatibility;
 *     - endpoint direction legality;
 *     - send/receive permissions;
 *     - channel ownership;
 *     - borrowing/lifetime rules;
 *     - close semantics;
 *     - select legality;
 *     - duplicate/default-arm validation;
 *     - blocking/non-blocking semantics;
 *     - cancellation interaction;
 *     - effect analysis;
 *     - resource requirements;
 *     - distributed communication legality;
 *     - capability requirements;
 *     - determinism guarantees.
 *
 * Parsing a channel construct does NOT imply semantic validity.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Channel syntax lowers through the existing semantic/IR architecture.
 *
 * No channel-specific universal IR is introduced here.
 *
 * The semantic representation may eventually lower communication onto:
 *
 *     local execution
 *     shared memory
 *     OS primitives
 *     accelerator communication
 *     interconnects
 *     networks
 *     distributed transports
 *     heterogeneous systems
 *     quantum/classical orchestration
 *     future communication mechanisms
 *
 * without changing the source grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Channel payloads may contain quantum-related semantic values where the
 * type/semantic system permits them.
 *
 * This grammar does NOT define quantum semantics.
 *
 * In particular it does not define:
 *
 *     qubit allocation
 *     physical qubits
 *     quantum routing
 *     QEC
 *     ZQN
 *     calibration
 *     quantum topology
 *
 * Quantum semantics continue through:
 *
 *     generic frontend AST
 *          ->
 *     semantic analysis
 *          ->
 *     quantum::ir
 *          ->
 *     optimization
 *          ->
 *     routing
 *          ->
 *     scheduling
 *          ->
 *     QEC / resilience / ZQN
 *          ->
 *     HAL
 *          ->
 *     target
 *
 * ============================================================================
 * DISTRIBUTED / NETWORK INTEGRATION
 * ============================================================================
 *
 * Channels may eventually be implemented locally or across a distributed
 * substrate.
 *
 * This grammar therefore does NOT encode:
 *
 *     node IDs
 *     IP addresses
 *     socket IDs
 *     physical links
 *     fixed topology
 *     transport implementation
 *     network protocol
 *
 * Those belong to networking/distributed/resource/runtime layers.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust actions.
 *
 * The generated Zamani frontend is maintained for:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * No unsafe Rust is required by this grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given the same:
 *
 *     source token stream
 *     language version
 *     parser configuration
 *
 * this grammar must produce the same parse structure.
 *
 * Parsing must not depend on:
 *
 *     time
 *     randomness
 *     filesystem state
 *     network state
 *     hardware availability
 *     scheduler state
 *     runtime state
 *     target selection
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a channel construct must never:
 *
 *     - create a channel;
 *     - send a message;
 *     - receive a message;
 *     - close a channel;
 *     - contact a network;
 *     - inspect hardware;
 *     - allocate runtime resources;
 *     - execute source code.
 *
 * ============================================================================
 */

parser grammar Channels;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions,
    Types,
    Calls,
    ZamaniExpressionBlocks
;


/*
 * ============================================================================
 * 1. PUBLIC CHANNEL EXPRESSION
 * ============================================================================
 *
 * Exactly one expression-level channel entry point.
 *
 * Concrete operations remain independently owned below.
 */

channelExpression
    : channelConstructExpression
    | channelSendExpression
    | channelReceiveExpression
    | channelTrySendExpression
    | channelTryReceiveExpression
    | channelCloseExpression
    | channelSelectExpression
    ;


/*
 * ============================================================================
 * 2. PUBLIC CHANNEL STATEMENT
 * ============================================================================
 *
 * Channel declarations and communication operations are exposed here so the
 * concurrency composition grammar can consume them without duplicating the
 * implementation.
 */

channelStatement
    : channelDeclaration
    | channelSendStatement
    | channelReceiveStatement
    | channelCloseStatement
    | channelSelectStatement
    ;


/*
 * ============================================================================
 * 3. CHANNEL DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     channel values: channel<Int>;
 *     channel values: channel<Message> = channel<Message>();
 *
 * Direction is optional at the declaration boundary and is validated
 * semantically.
 */

channelDeclaration
    : CHANNEL
      identifier
      channelTypeAnnotation?
      channelInitializer?
      SEMICOLON?
    ;


channelTypeAnnotation
    : COLON
      channelType
    ;


channelInitializer
    : ASSIGN
      channelConstructExpression
    ;


/*
 * ============================================================================
 * 4. CHANNEL TYPE
 * ============================================================================
 *
 * `channel<T>` is a language-level generic channel type.
 *
 * T is an ordinary Zamani type.
 *
 * No payload-size, queue-size, or hardware-size limit is encoded.
 */

channelType
    : CHANNEL
      LESS
      typeExpression
      GREATER
    ;


directionalChannelType
    : channelDirection
      channelType
    ;


channelDirection
    : SEND
    | RECEIVE
    ;


/*
 * ============================================================================
 * 5. CHANNEL CONSTRUCTION
 * ============================================================================
 *
 * Examples:
 *
 *     channel<Int>()
 *     channel<Int>(capacity)
 *     channel<Message>(capacity, policy)
 *
 * All constructor arguments remain ordinary Zamani expressions.
 *
 * The grammar does not decide which argument represents:
 *
 *     capacity
 *     buffering
 *     transport
 *     policy
 *     resource preference
 *
 * Those meanings belong to semantic analysis and resource/runtime layers.
 */

channelConstructExpression
    : CHANNEL
      LESS
      typeExpression
      GREATER
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 6. CHANNEL OPERAND
 * ============================================================================
 *
 * Any expression may syntactically occupy the channel position.
 *
 * Semantic analysis determines whether it evaluates to a channel.
 *
 * This prevents a closed finite namespace of channel identifiers.
 */

channelOperand
    : expression
    ;


/*
 * ============================================================================
 * 7. SEND
 * ============================================================================
 *
 * Canonical form:
 *
 *     send channel, value
 *
 * The payload may be any source-level expression whose semantic type is
 * compatible with the channel payload type.
 */

channelSendExpression
    : SEND
      channelOperand
      COMMA
      expression
    ;


channelSendStatement
    : channelSendExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 8. RECEIVE
 * ============================================================================
 *
 * Canonical form:
 *
 *     receive channel
 *
 * The resulting type is determined semantically from the channel type.
 */

channelReceiveExpression
    : RECEIVE
      channelOperand
    ;


channelReceiveStatement
    : channelReceiveExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 9. NON-BLOCKING SEND
 * ============================================================================
 *
 * Canonical form:
 *
 *     try send channel, value
 *
 * TRY and SEND remain independent existing/new lexical concepts rather than
 * introducing a compound TRY_SEND token.
 */

channelTrySendExpression
    : TRY
      SEND
      channelOperand
      COMMA
      expression
    ;


/*
 * ============================================================================
 * 10. NON-BLOCKING RECEIVE
 * ============================================================================
 *
 * Canonical form:
 *
 *     try receive channel
 */

channelTryReceiveExpression
    : TRY
      RECEIVE
      channelOperand
    ;


/*
 * ============================================================================
 * 11. CLOSE
 * ============================================================================
 *
 * Canonical form:
 *
 *     close channel
 *
 * Whether closing wakes receivers, rejects later sends, propagates
 * cancellation, or is idempotent is semantic/runtime behavior.
 */

channelCloseExpression
    : CLOSE
      channelOperand
    ;


channelCloseStatement
    : channelCloseExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 12. SELECT
 * ============================================================================
 *
 * A select contains zero or more communication alternatives plus at most one
 * default alternative.
 *
 * There is deliberately no finite arm count.
 */

channelSelectExpression
    : SELECT
      LBRACE
      channelSelectArmComposition*
      channelSelectDefaultArm?
      RBRACE
    ;


channelSelectArmComposition
    : channelSelectArm
    ;


channelSelectArm
    : CASE
      channelSelectOperation
      FAT_ARROW
      channelSelectBody
      COMMA?
    ;


channelSelectBody
    : blockExpression
    | expression
    ;


/*
 * ============================================================================
 * 13. SELECT OPERATION
 * ============================================================================
 *
 * Select is intentionally restricted to communication operations.
 *
 * This avoids the previous overly broad:
 *
 *     | expression
 *
 * alternative, which could make arbitrary expressions appear to be
 * communication alternatives.
 */

channelSelectOperation
    : channelReceiveExpression
    | channelSendExpression
    | channelTryReceiveExpression
    | channelTrySendExpression
    ;


channelSelectDefaultArm
    : DEFAULT
      FAT_ARROW
      channelSelectBody
      COMMA?
    ;


channelSelectStatement
    : channelSelectExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 14. ENDPOINTS
 * ============================================================================
 *
 * Endpoints are logical channel views.
 *
 * They do not identify physical communication resources.
 */

channelEndpoint
    : channelOperand
    ;


directionalChannelEndpoint
    : channelDirection
      channelOperand
    ;


channelEndpointDeclaration
    : CHANNEL
      identifier
      COLON
      directionalChannelType
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 15. CHANNEL OPERATION ROOT
 * ============================================================================
 *
 * Stable integration point for semantic tooling.
 */

channelOperation
    : channelSendExpression
    | channelReceiveExpression
    | channelTrySendExpression
    | channelTryReceiveExpression
    | channelCloseExpression
    ;


channelOperationStatement
    : channelOperation
      SEMICOLON?
    ;


channelSelectableOperation
    : channelSendExpression
    | channelReceiveExpression
    | channelTrySendExpression
    | channelTryReceiveExpression
    ;


/*
 * ============================================================================
 * 16. CHANNEL CONCURRENCY ADAPTERS
 * ============================================================================
 *
 * These stable names let Concurrency and downstream tooling refer to the
 * channel domain without knowing its internal productions.
 */

channelConcurrencyExpression
    : channelExpression
    ;


channelConcurrencyStatement
    : channelStatement
    ;


/*
 * ============================================================================
 * 17. CHANNEL DECLARATION GROUP
 * ============================================================================
 *
 * No fixed number of declarations is imposed.
 */

channelDeclarationGroup
    : channelDeclaration+
    ;


/*
 * ============================================================================
 * 18. SEMANTIC / RESOURCE BOUNDARY
 * ============================================================================
 *
 * Capacity and other constructor arguments are source expressions.
 *
 * These named adapters exist for semantic tooling without imposing a
 * machine-specific representation.
 */

channelCapacityExpression
    : expression
    ;


channelConfiguredConstructExpression
    : channelConstructExpression
    ;


channelBufferedConstructExpression
    : channelConstructExpression
    ;


/*
 * ============================================================================
 * 19. INTEGRATION CONTRACT
 * ============================================================================
 *
 * Upstream:
 *
 *     ZamaniLexer
 *     Expressions
 *     Types
 *     Calls
 *     ZamaniExpressionBlocks
 *
 * Composition:
 *
 *     Channels
 *          |
 *          v
 *     Concurrency
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     domain-neutral AST
 *
 * Downstream:
 *
 *     semantic analysis
 *          |
 *          +--> effects
 *          +--> ownership/lifetimes
 *          +--> resources/capabilities
 *          +--> distributed/network semantics
 *          +--> canonical IR
 *          |
 *          +--> scheduling
 *          +--> placement
 *          +--> runtime
 *
 * No downstream component may interpret this grammar as selecting a physical
 * communication resource during parsing.
 *
 * ============================================================================
 * 20. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify structural errors such as:
 *
 *     missing channel name
 *     missing channel type
 *     malformed generic type
 *     missing constructor parenthesis
 *     missing constructor argument delimiter
 *     missing send payload
 *     missing receive operand
 *     malformed select arm
 *     missing FAT_ARROW
 *     malformed default arm
 *     missing closing brace
 *
 * Semantic diagnostics belong downstream:
 *
 *     non-channel operand
 *     invalid payload type
 *     invalid endpoint direction
 *     send on receive-only endpoint
 *     receive on send-only endpoint
 *     invalid close
 *     duplicate default arm
 *     impossible communication requirement
 *     unsatisfied capability
 *     unsatisfied resource requirement
 *
 * ============================================================================
 * 21. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_CHANNELS
 *     MAX_MESSAGES
 *     MAX_SENDERS
 *     MAX_RECEIVERS
 *     MAX_QUEUE_SIZE
 *     MAX_BUFFER_SIZE
 *     MAX_NODES
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     physical channel IDs
 *     physical queue IDs
 *     fixed network topology
 *
 * None are represented here.
 *
 * Program constants remain legal source semantics. For example:
 *
 *     let capacity = derive_capacity();
 *
 * may be passed to a channel constructor without turning that value into a
 * universal language limit.
 *
 * ============================================================================
 * 22. TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     channel values: channel<Int>;
 *     channel values: channel<Int> = channel<Int>();
 *     channel values: channel<Message> = channel<Message>(capacity);
 *     send values, message;
 *     receive values;
 *     try send values, message;
 *     try receive values;
 *     close values;
 *
 * Select:
 *
 *     select {
 *         case receive input => process();
 *         case send output, value => continue();
 *         default => fallback();
 *     }
 *
 * Blocks:
 *
 *     select {
 *         case receive input => {
 *             process(input);
 *         }
 *     }
 *
 * Nested expressions:
 *
 *     send channels[index], compute(value);
 *     receive services.lookup();
 *
 * Domain-neutral payloads:
 *
 *     send classical, tensor;
 *     send quantum, state;
 *     send accelerator, result;
 *     send distributed, message;
 *
 * Negative:
 *
 *     channel;
 *     channel<>;
 *     channel<Int>(
 *     send;
 *     send channel;
 *     receive;
 *     try send channel;
 *     select {
 *     select {
 *         default => value;
 *         default => other;
 *     }
 *
 * Boundary:
 *
 *     empty select;
 *     one select arm;
 *     many select arms;
 *     symbolic channel capacity;
 *     nested channel expressions;
 *     nested select blocks;
 *     deeply composed payload expressions.
 *
 * Scalability:
 *
 *     no fixed channel count;
 *     no fixed arm count;
 *     no fixed payload count;
 *     no fixed sender/receiver count;
 *     no fixed topology;
 *     no fixed execution width.
 *
 * Determinism:
 *
 *     identical token stream -> identical parse structure.
 *
 * Compatibility:
 *
 *     existing channelExpression;
 *     existing channelStatement;
 *     existing channelDeclaration;
 *     existing channelSendExpression;
 *     existing channelReceiveExpression;
 *     existing channelSelectExpression;
 *
 * remain stable parser integration names.
 *
 * ============================================================================
 * 23. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] one canonical channel grammar exists;
 *     [x] existing filename is retained;
 *     [x] existing useful public rule names are retained;
 *     [x] canonical lexer is consumed;
 *     [x] no lexer rules exist here;
 *     [x] canonical expression grammar is reused;
 *     [x] canonical type grammar is reused;
 *     [x] canonical call argument grammar is reused;
 *     [x] canonical block-expression grammar is reused;
 *     [x] no duplicate select root exists;
 *     [x] select arm composition is reachable;
 *     [x] no undefined SEMI token is used;
 *     [x] no undefined RECEIVE_BIND token is used;
 *     [x] no undefined TRY_SEND/TRY_RECEIVE token is required;
 *     [x] no fixed resource limit exists;
 *     [x] no physical topology exists;
 *     [x] no quantum gate inventory exists;
 *     [x] no quantum IR exists;
 *     [x] no runtime execution exists;
 *     [x] no unsafe Rust is required;
 *     [x] Rust 1.97/1.97.1 compatibility is documented;
 *     [x] AST ownership is predefined;
 *     [x] semantic ownership is predefined;
 *     [x] IR integration is predefined;
 *     [x] compiler/runtime integration is predefined;
 *     [x] scalability requirements are predefined;
 *     [x] diagnostics are predefined;
 *     [x] test categories are predefined.
 *
 * ============================================================================
 */