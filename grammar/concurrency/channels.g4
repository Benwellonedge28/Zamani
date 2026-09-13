/*
 * ============================================================================
 * Zamani Programming Language
 * Production Channel-Concurrency Grammar
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/channels.g4
 *
 * Role:
 *     Canonical parser-level grammar for typed communication channels and
 *     channel operations.
 *
 * Language model:
 *     Zamani describes computation and communication intent, not the physical
 *     resources used to realize that communication.
 *
 * Compiler/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust safety:
 *     This grammar contains no Rust implementation code.
 *
 *     The Zamani compiler/runtime MUST use safe Rust.
 *     Rust `unsafe` is neither required nor permitted for this feature.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - channel declaration syntax;
 *   - channel construction syntax;
 *   - channel type syntax at the parser boundary;
 *   - channel send syntax;
 *   - channel receive syntax;
 *   - channel close syntax;
 *   - channel try-send / try-receive syntax;
 *   - channel select syntax;
 *   - channel select arms;
 *   - channel communication expressions;
 *   - channel communication statements;
 *   - channel direction syntax;
 *   - channel communication modifiers that are genuinely syntactic;
 *   - stable parser integration points for channel concurrency.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - channel runtime implementation;
 *   - queue implementation;
 *   - queue capacity limits;
 *   - maximum channel count;
 *   - maximum sender count;
 *   - maximum receiver count;
 *   - worker count;
 *   - thread count;
 *   - CPU/core count;
 *   - machine topology;
 *   - network topology;
 *   - distributed placement;
 *   - scheduling;
 *   - backpressure algorithms;
 *   - buffering implementation;
 *   - memory allocation;
 *   - synchronization implementation;
 *   - fairness algorithms;
 *   - deadlock detection algorithms;
 *   - cancellation implementation;
 *   - timeout implementation;
 *   - hardware discovery;
 *   - hardware capabilities;
 *   - runtime resource limits;
 *   - classical IR;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - optimization;
 *   - resilience.
 *
 * Those concerns belong to semantic analysis, resource analysis, canonical IR,
 * scheduling, runtime, hardware abstraction, resilience, and other owning
 * subsystems.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Channel syntax describes communication semantics.
 *
 * It MUST NOT encode a fixed machine topology or finite implementation limit.
 *
 * In particular, this grammar contains no constants for:
 *
 *     MAX_CHANNELS
 *     MAX_MESSAGES
 *     MAX_SENDERS
 *     MAX_RECEIVERS
 *     MAX_QUEUE_SIZE
 *     MAX_BUFFER_SIZE
 *     MAX_CHANNELS_PER_TASK
 *     MAX_CHANNELS_PER_NODE
 *     MAX_NODES
 *     MAX_WORKERS
 *
 * A channel may therefore be lowered onto:
 *
 *     - an in-process queue;
 *     - shared memory;
 *     - an OS primitive;
 *     - an accelerator communication mechanism;
 *     - a local interconnect;
 *     - a network;
 *     - a distributed transport;
 *     - a quantum/classical orchestration layer;
 *     - a future communication substrate.
 *
 * The actual realization is determined after parsing.
 *
 * ============================================================================
 * SEMANTIC PRINCIPLE
 * ============================================================================
 *
 * A channel represents a communication relationship.
 *
 * It does NOT inherently represent:
 *
 *     thread
 *     process
 *     CPU
 *     core
 *     GPU
 *     FPGA
 *     QPU
 *     network socket
 *     memory address
 *     queue implementation
 *
 * A target with limited resources may serialize communication.
 *
 * A target with abundant resources may execute communication concurrently.
 *
 * Both are valid implementations if they preserve the program's semantic
 * behavior and declared communication guarantees.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar intentionally reuses canonical Zamani grammar rules.
 *
 * Expected canonical lexer tokens:
 *
 *     CHANNEL
 *     SEND
 *     RECEIVE
 *     CLOSE
 *     SELECT
 *     DEFAULT
 *     TRY
 *     TIMEOUT
 *
 * Optional advanced channel tokens:
 *
 *     TRY_SEND
 *     TRY_RECEIVE
 *     CASE
 *     WHEN
 *
 * The canonical lexer MUST own those lexical spellings.
 *
 * This file MUST NOT define lexer rules.
 *
 * Expected canonical parser rules:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     blockExpression
 *     statement
 *     typeExpression
 *     genericArguments
 *     argumentList
 *     pattern
 *
 * This file MUST NOT redefine those rules.
 *
 * ============================================================================
 * INTEGRATION PIPELINE
 * ============================================================================
 *
 *     source
 *        |
 *        v
 *     ZamaniLexer
 *        |
 *        v
 *     parser composition
 *        |
 *        +------------------------+
 *        |                        |
 *        v                        v
 *     Core grammar          channels.g4
 *        |                        |
 *        +-----------+------------+
 *                    |
 *                    v
 *                Frontend AST
 *                    |
 *                    v
 *          name/type/effect analysis
 *                    |
 *                    v
 *             resource analysis
 *                    |
 *                    v
 *              canonical IR
 *                    |
 *          +---------+---------+
 *          |                   |
 *          v                   v
 *      scheduling          distributed/
 *      /runtime            network/runtime
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *                  target
 *
 * The grammar MUST NOT depend on runtime implementation details.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser should produce channel-specific syntax nodes which preserve:
 *
 *     declaration
 *     channel type
 *     direction
 *     construction arguments
 *     sender
 *     receiver
 *     payload
 *     select arms
 *     default arm
 *     timeout expression
 *     close operation
 *     source locations
 *
 * Semantic lowering must subsequently convert those nodes into the canonical
 * program representation.
 *
 * This grammar MUST NOT define a second channel IR.
 *
 * ============================================================================
 * CHANNEL TYPE MODEL
 * ============================================================================
 *
 * Channel payload types are ordinary Zamani types.
 *
 * Examples:
 *
 *     Channel<Int>
 *     Channel<Message>
 *     Channel<QubitState>
 *     Channel<Tensor>
 *     Channel<Result<Value, Error>>
 *
 * The grammar does not impose a maximum payload size.
 *
 * Resource feasibility is determined later.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CHANNEL ROOT
 * ============================================================================
 *
 * Stable parser integration point.
 *
 * The main expression/statement grammar should integrate `channelExpression`
 * and `channelStatement` rather than importing every internal production.
 * ============================================================================
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
 * CHANNEL DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     channel values: Channel<Int>;
 *
 *     channel values: Channel<Message> = channel();
 *
 *     channel input: receive Channel<Request>;
 *
 *     channel output: send Channel<Response>;
 *
 * Direction is a semantic property of the channel endpoint and MUST NOT
 * imply a particular runtime implementation.
 * ============================================================================
 */

channelDeclaration
    : CHANNEL
      identifier
      channelTypeAnnotation?
      channelInitializer?
      SEMI?
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
 * CHANNEL TYPE
 * ============================================================================
 *
 * The canonical type grammar remains authoritative.
 *
 * The channel grammar merely provides the channel-specific wrapper.
 * ============================================================================
 */

channelType
    : channelTypeConstructor
    ;


channelTypeConstructor
    : CHANNEL
      LT
      typeExpression
      GT
    ;


/*
 * ============================================================================
 * CHANNEL DIRECTIONS
 * ============================================================================
 *
 * A direction restricts an endpoint's permitted communication operation.
 *
 * It does not determine implementation.
 *
 * Examples:
 *
 *     send Channel<T>
 *     receive Channel<T>
 *     Channel<T>
 *
 * Bidirectional channels remain valid unless semantic policy restricts them.
 * ============================================================================
 */

channelDirection
    : SEND
    | RECEIVE
    ;


directionalChannelType
    : channelDirection
      channelType
    ;


/*
 * ============================================================================
 * CHANNEL CONSTRUCTION
 * ============================================================================
 *
 * Capacity, buffering, allocation and transport are semantic/resource
 * concerns.
 *
 * Therefore:
 *
 *     channel<T>()
 *
 *     channel<T>(capacity)
 *
 * are syntactically representable without establishing a maximum.
 *
 * `capacity` is an ordinary expression.
 *
 * It may therefore be:
 *
 *     constant
 *     runtime-derived
 *     configuration-derived
 *     capability-derived
 *     symbolic
 *     target-negotiated
 *
 * The grammar does not decide which.
 * ============================================================================
 */

channelConstructExpression
    : CHANNEL
      genericArguments?
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * NAMED CHANNEL CONSTRUCTION
 * ============================================================================
 *
 * Allows explicit channel type information when generic inference is not
 * sufficient or desired.
 *
 * Examples:
 *
 *     channel<Payload>()
 *
 *     channel<Payload>(capacity)
 *
 *     channel<Payload>(capacity, policy)
 * ============================================================================
 */

typedChannelConstructExpression
    : CHANNEL
      LT
      typeExpression
      GT
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * SEND
 * ============================================================================
 *
 * Canonical form:
 *
 *     send channel, value
 *
 * The channel and value are ordinary expressions.
 *
 * This allows communication of arbitrary Zamani values without enumerating
 * every possible domain:
 *
 *     classical values
 *     quantum values
 *     hardware descriptors
 *     tensors
 *     messages
 *     distributed values
 *     future domain values
 * ============================================================================
 */

channelSendExpression
    : SEND
      channelOperand
      COMMA
      expression
    ;


/*
 * ============================================================================
 * SEND STATEMENT
 * ============================================================================
 */

channelSendStatement
    : channelSendExpression
      SEMI?
    ;


/*
 * ============================================================================
 * RECEIVE
 * ============================================================================
 *
 * Canonical expression form:
 *
 *     receive channel
 *
 * The result type is determined semantically from the channel's payload type.
 * ============================================================================
 */

channelReceiveExpression
    : RECEIVE
      channelOperand
    ;


/*
 * ============================================================================
 * RECEIVE STATEMENT
 * ============================================================================
 */

channelReceiveStatement
    : channelReceiveExpression
      SEMI?
    ;


/*
 * ============================================================================
 * TRY-SEND
 * ============================================================================
 *
 * A non-blocking communication request.
 *
 * The exact result representation is semantic/typed-system responsibility.
 *
 * The grammar does not hard-code:
 *
 *     bool
 *     Option<T>
 *     Result<T, E>
 *
 * because that would incorrectly impose one runtime model.
 * ============================================================================
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
 * TRY-RECEIVE
 * ============================================================================
 */

channelTryReceiveExpression
    : TRY
      RECEIVE
      channelOperand
    ;


/*
 * ============================================================================
 * CLOSE
 * ============================================================================
 *
 * Closing is a semantic communication operation.
 *
 * Whether closing:
 *
 *     wakes receivers
 *     rejects future sends
 *     propagates cancellation
 *     becomes idempotent
 *
 * is defined by the language semantic specification/runtime contract rather
 * than by this grammar.
 * ============================================================================
 */

channelCloseExpression
    : CLOSE
      channelOperand
    ;


channelCloseStatement
    : channelCloseExpression
      SEMI?
    ;


/*
 * ============================================================================
 * CHANNEL OPERAND
 * ============================================================================
 *
 * A channel is identified by an ordinary expression.
 *
 * This prevents the grammar from requiring a special finite namespace of
 * channel identifiers.
 * ============================================================================
 */

channelOperand
    : expression
    ;


/*
 * ============================================================================
 * SELECT
 * ============================================================================
 *
 * Select exposes multiple communication alternatives.
 *
 * It does not require a fixed number of arms.
 *
 * It does not require a fixed number of channels.
 *
 * It does not define fairness.
 *
 * It does not define scheduling.
 *
 * It does not define implementation-level polling.
 * ============================================================================
 */

channelSelectExpression
    : SELECT
      LBRACE
      channelSelectArm*
      channelSelectDefaultArm?
      RBRACE
    ;


/*
 * ============================================================================
 * SELECT ARM
 * ============================================================================
 *
 * Examples:
 *
 *     select {
 *         case receive input => process(input);
 *         case send output, value => continue();
 *     }
 *
 * Bindings are semantic values represented by ordinary patterns/identifiers.
 * ============================================================================
 */

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
 * SELECT OPERATION
 * ============================================================================
 *
 * A select arm may wait for:
 *
 *     receive
 *     send
 *     try-send
 *     try-receive
 *     ordinary asynchronous computation
 *
 * The communication primitives remain open to future extensions through the
 * semantic layer rather than through hard-coded machine concepts.
 * ============================================================================
 */

channelSelectOperation
    : channelReceiveExpression
    | channelSendExpression
    | channelTryReceiveExpression
    | channelTrySendExpression
    | expression
    ;


/*
 * ============================================================================
 * DEFAULT SELECT ARM
 * ============================================================================
 *
 * A default arm expresses non-blocking fallback behavior.
 *
 * It does not guarantee that the implementation performs polling.
 * ============================================================================
 */

channelSelectDefaultArm
    : DEFAULT
      FAT_ARROW
      channelSelectBody
      COMMA?
    ;


/*
 * ============================================================================
 * SELECT WITH BINDING
 * ============================================================================
 *
 * Optional semantic binding form.
 *
 * Example:
 *
 *     select {
 *         case value = receive input => process(value);
 *     }
 *
 * The left-hand side remains a canonical pattern.
 * ============================================================================
 */

channelSelectBindingArm
    : CASE
      pattern
      ASSIGN
      channelSelectOperation
      FAT_ARROW
      channelSelectBody
      COMMA?
    ;


/*
 * ============================================================================
 * SELECT ARM COMPOSITION
 * ============================================================================
 *
 * Stable integration rule allowing future semantic expansion without changing
 * the public `channelSelectExpression` entry point.
 * ============================================================================
 */

channelSelectArmComposition
    : channelSelectArm
    | channelSelectBindingArm
    ;


/*
 * ============================================================================
 * SELECT BODY
 * ============================================================================
 */

channelSelect
    : SELECT
      LBRACE
      channelSelectArmComposition*
      channelSelectDefaultArm?
      RBRACE
    ;


/*
 * ============================================================================
 * CHANNEL COMMUNICATION STATEMENT
 * ============================================================================
 *
 * Stable statement-level integration point.
 * ============================================================================
 */

channelStatement
    : channelDeclaration
    | channelSendStatement
    | channelReceiveStatement
    | channelCloseStatement
    | channelSelectStatement
    ;


channelSelectStatement
    : channelSelectExpression
      SEMI?
    ;


/*
 * ============================================================================
 * CHANNEL COMMUNICATION ROOT
 * ============================================================================
 *
 * This is the preferred public integration boundary for the concurrency
 * statement grammar.
 * ============================================================================
 */

channelConcurrencyStatement
    : channelStatement
    ;


/*
 * ============================================================================
 * CHANNEL COMMUNICATION EXPRESSION ROOT
 * ============================================================================
 */

channelConcurrencyExpression
    : channelExpression
    ;


/*
 * ============================================================================
 * CHANNEL ENDPOINT
 * ============================================================================
 *
 * Endpoint direction is represented explicitly at syntax level where the
 * language requires it.
 *
 * It remains independent of:
 *
 *     process
 *     thread
 *     actor
 *     node
 *     device
 *     hardware queue
 * ============================================================================
 */

channelEndpoint
    : channelOperand
    | directionalChannelEndpoint
    ;


directionalChannelEndpoint
    : channelDirection
      channelOperand
    ;


/*
 * ============================================================================
 * CHANNEL ENDPOINT DECLARATION
 * ============================================================================
 */

channelEndpointDeclaration
    : CHANNEL
      identifier
      COLON
      directionalChannelType
      SEMI?
    ;


/*
 * ============================================================================
 * CHANNEL CAPACITY EXPRESSION
 * ============================================================================
 *
 * This named boundary exists so semantic analysis can identify capacity
 * arguments without forcing a grammar-level numeric limit.
 *
 * Examples:
 *
 *     channel<T>(0)
 *     channel<T>(capacity)
 *     channel<T>(available_capacity())
 *     channel<T>(resource.capacity)
 *
 * The grammar accepts the expression.
 *
 * Semantic analysis determines:
 *
 *     validity
 *     units
 *     representability
 *     target feasibility
 *     overflow behavior
 *     resource policy
 * ============================================================================
 */

channelCapacityExpression
    : expression
    ;


/*
 * ============================================================================
 * CHANNEL CONSTRUCTION WITH CAPACITY
 * ============================================================================
 */

channelBufferedConstructExpression
    : CHANNEL
      genericArguments?
      LPAREN
      channelCapacityExpression
      RPAREN
    ;


/*
 * ============================================================================
 * CHANNEL CONSTRUCTION WITH OPTIONS
 * ============================================================================
 *
 * Options remain ordinary expressions so the language can evolve without
 * repeatedly modifying this grammar for every future runtime policy.
 * ============================================================================
 */

channelConfiguredConstructExpression
    : CHANNEL
      genericArguments?
      LPAREN
      argumentList
      RPAREN
    ;


/*
 * ============================================================================
 * CHANNEL TYPE ALIAS INTEGRATION
 * ============================================================================
 *
 * This production intentionally delegates alias semantics to the canonical
 * type system.
 * ============================================================================
 */

channelTypeReference
    : typeExpression
    ;


/*
 * ============================================================================
 * CHANNEL VALUE BINDING
 * ============================================================================
 *
 * Receiving into a binding is expressed using the canonical pattern system.
 *
 * Example:
 *
 *     receive channel -> value
 *
 * The exact surface form may be selected by the canonical statement grammar;
 * this production exists as an explicit parser integration boundary.
 * ============================================================================
 */

channelReceiveBinding
    : RECEIVE
      channelOperand
      RECEIVE_BIND
      pattern
    ;


/*
 * ============================================================================
 * CHANNEL SEND WITH NAMED PAYLOAD
 * ============================================================================
 *
 * Optional structured form for message-oriented communication.
 *
 * Example:
 *
 *     send channel {
 *         value: payload
 *     }
 *
 * The actual message schema remains a normal Zamani value.
 * ============================================================================
 */

channelStructuredSend
    : SEND
      channelOperand
      blockExpression
    ;


/*
 * ============================================================================
 * CHANNEL MESSAGE EXPRESSION
 * ============================================================================
 *
 * This adapter keeps message representation independent from transport.
 * ============================================================================
 */

channelMessage
    : expression
    | blockExpression
    ;


/*
 * ============================================================================
 * CHANNEL OPERATION
 * ============================================================================
 */

channelOperation
    : channelSendExpression
    | channelReceiveExpression
    | channelTrySendExpression
    | channelTryReceiveExpression
    | channelCloseExpression
    ;


/*
 * ============================================================================
 * CHANNEL OPERATION STATEMENT
 * ============================================================================
 */

channelOperationStatement
    : channelOperation
      SEMI?
    ;


/*
 * ============================================================================
 * CHANNEL SELECTABLE OPERATION
 * ============================================================================
 */

channelSelectableOperation
    : channelOperation
    | expression
    ;


/*
 * ============================================================================
 * CHANNEL SELECT ARM BODY
 * ============================================================================
 */

channelArmBody
    : blockExpression
    | expression
    ;


/*
 * ============================================================================
 * CHANNEL DECLARATION GROUP
 * ============================================================================
 *
 * Repetition is deliberately unbounded by grammar.
 *
 * Practical resource limits belong to semantic/resource/runtime layers.
 * ============================================================================
 */

channelDeclarationGroup
    : channelDeclaration+
    ;


/*
 * ============================================================================
 * CHANNEL OPERATION GROUP
 * ============================================================================
 */

channelOperationGroup
    : channelOperationStatement+
    ;


/*
 * ============================================================================
 * CHANNEL ROOT ADAPTER
 * ============================================================================
 *
 * Parser composition should use these stable boundaries instead of coupling
 * itself to internal implementation productions.
 * ============================================================================
 */

channelRootExpression
    : channelConcurrencyExpression
    ;


channelRootStatement
    : channelConcurrencyStatement
    ;


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The following are semantic requirements, not grammar requirements:
 *
 *   1. A send must target a send-capable endpoint.
 *
 *   2. A receive must target a receive-capable endpoint.
 *
 *   3. The payload type must be compatible with the channel payload type.
 *
 *   4. A closed channel must obey the language's defined close semantics.
 *
 *   5. Select arms must be type/effect compatible where required.
 *
 *   6. Blocking behavior must be represented in the semantic/effect model.
 *
 *   7. Communication effects must be visible to effect analysis.
 *
 *   8. Resource requirements must be visible to resource analysis.
 *
 *   9. Distributed communication must be lowered through the appropriate
 *      distributed/network subsystem rather than implemented by the grammar.
 *
 *  10. Quantum communication must be interpreted through the appropriate
 *      quantum semantic/IR layers rather than represented by channel grammar
 *      as a physical QPU operation.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * The grammar MUST NOT decide:
 *
 *     how many channels can exist;
 *     how many messages can be buffered;
 *     how many senders can connect;
 *     how many receivers can connect;
 *     how many channels execute simultaneously;
 *     how much memory a channel consumes;
 *     whether communication is local or remote.
 *
 * Resource analysis may derive requirements from the program.
 *
 * Scheduling may derive execution order.
 *
 * Hardware abstraction may provide capabilities.
 *
 * Runtime may provide actual resources.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST be deterministic.
 *
 * The grammar MUST NOT depend on:
 *
 *     current time;
 *     runtime scheduling;
 *     thread interleaving;
 *     hardware discovery;
 *     random selection;
 *     network state.
 *
 * Select execution order is a runtime/semantic concern and must not affect
 * parsing.
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * Syntax errors belong to the parser diagnostics layer.
 *
 * Semantic errors such as:
 *
 *     send on receive-only endpoint;
 *     receive on send-only endpoint;
 *     incompatible payload;
 *     invalid close operation;
 *     impossible select arm;
 *
 * belong to semantic analysis.
 *
 * This grammar MUST NOT encode semantic diagnostics as parser hacks.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There is deliberately:
 *
 *     no MAX_CHANNELS;
 *     no MAX_MESSAGES;
 *     no MAX_BUFFER;
 *     no MAX_SENDERS;
 *     no MAX_RECEIVERS;
 *     no MAX_SELECT_ARMS;
 *     no MAX_CHANNEL_TYPE_DEPTH;
 *     no MAX_CHANNEL_NESTING;
 *     no fixed machine topology.
 *
 * Any actual implementation limit MUST be represented by the appropriate
 * compiler/runtime/resource mechanism rather than by this grammar.
 *
 * ============================================================================
 * FUTURE EXTENSIBILITY
 * ============================================================================
 *
 * Future communication mechanisms may be introduced through:
 *
 *     dialects;
 *     effects;
 *     capabilities;
 *     resource constraints;
 *     distributed execution;
 *     networking;
 *     hardware-specific lowering;
 *
 * without changing the fundamental channel model.
 *
 * Examples include:
 *
 *     local channels
 *     distributed channels
 *     streaming channels
 *     hardware FIFOs
 *     accelerator queues
 *     event channels
 *     actor mailboxes
 *     telemetry streams
 *     quantum/classical coordination channels
 *     future communication substrates
 *
 * These are implementation/semantic realizations, not separate physical
 * channel grammars.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [ ] all referenced lexer tokens exist in the canonical Zamani lexer;
 *   [ ] all referenced parser rules exist in their owning grammar;
 *   [ ] this file contains no duplicate canonical type/expression/block rules;
 *   [ ] channel declarations parse;
 *   [ ] typed channels parse;
 *   [ ] directional endpoints parse;
 *   [ ] construction parses;
 *   [ ] send parses;
 *   [ ] receive parses;
 *   [ ] try-send parses;
 *   [ ] try-receive parses;
 *   [ ] close parses;
 *   [ ] select parses;
 *   [ ] default select arms parse;
 *   [ ] nested communication parses;
 *   [ ] arbitrary valid payload types parse;
 *   [ ] arbitrary valid expressions parse;
 *   [ ] no fixed resource limit is encoded;
 *   [ ] no machine topology is encoded;
 *   [ ] no runtime implementation is encoded;
 *   [ ] no Rust code is embedded;
 *   [ ] no `unsafe` implementation is required;
 *   [ ] positive tests exist;
 *   [ ] negative syntax tests exist;
 *   [ ] semantic boundary tests exist;
 *   [ ] scalability tests exist;
 *   [ ] cross-domain tests exist;
 *   [ ] deterministic parsing tests exist;
 *   [ ] canonical IR lowering tests exist outside the grammar layer.
 *
 * ============================================================================
 */
 
parser grammar Channels;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ROOTS
 * ============================================================================
 */

channelsExpression
    : channelExpression
    ;


channelsStatement
    : channelStatement
    ;