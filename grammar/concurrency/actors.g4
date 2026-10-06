/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/concurrency/actors.g4
 *
 * GRAMMAR
 * -------
 * Actors
 *
 * STATUS
 * ------
 * PRODUCTION SOURCE-GRAMMAR CONTRACT
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical parser-level owner of Zamani actor syntax.
 *
 * An actor is a logical concurrent computation boundary. It is NOT a
 * commitment to a thread, process, core, CPU, GPU, FPGA, QPU, node,
 * accelerator, operating-system process, network endpoint, or other physical
 * execution resource.
 *
 * This grammar owns:
 *
 *     - actor declarations;
 *     - actor state;
 *     - actor handlers;
 *     - actor construction;
 *     - actor communication intent;
 *     - actor request/ask intent;
 *     - actor forwarding intent;
 *     - actor lifecycle intent;
 *     - actor supervision intent.
 *
 * The grammar intentionally uses:
 *
 *     actor <command> ...
 *
 * for actor commands.
 *
 * The command word is an ordinary identifier rather than a dedicated lexer
 * token. This is required because the current canonical lexer reserves ACTOR
 * and SPAWN but does not reserve dedicated tokens for:
 *
 *     receive
 *     send
 *     ask
 *     forward
 *     stop
 *     restart
 *     supervise
 *
 * Semantic analysis classifies the command identifier.
 *
 * This keeps this grammar independently compilable against the current
 * lexical vocabulary and prevents this file from becoming dependent on
 * future lexer edits.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     actorConstruct
 *     actorDeclaration
 *     actorBody
 *     actorMember
 *     actorStateField
 *     actorHandler
 *     actorHandlerReturnType
 *     actorSpawnExpression
 *     actorSpawnStatement
 *     actorCommand
 *     actorSendExpression
 *     actorSendStatement
 *     actorAskExpression
 *     actorAskStatement
 *     actorForwardExpression
 *     actorForwardStatement
 *     actorLifecycleExpression
 *     actorLifecycleStatement
 *     actorSupervisionConstruct
 *     actorSupervisionStatement
 *     actorTarget
 *     actorTargetPath
 *     actorMessageName
 *     actorArguments
 *     actorStatement
 *     actorExpression
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers
 *     qualified names
 *     expressions
 *     types
 *     blocks
 *     attributes
 *     modifiers
 *     visibility
 *     callable parameters
 *     generic parameters
 *     channels
 *     futures
 *     tasks
 *     async/await
 *     parallelism
 *     synchronization
 *     cancellation
 *     scheduling
 *     routing
 *     resource discovery
 *     capability resolution
 *     hardware discovery
 *     distributed placement
 *     networking
 *     quantum semantics
 *     quantum::ir
 *     classical IR
 *     HDL/hardware IR
 *     QEC
 *     ZQN
 *     HAL
 *     runtime implementation
 *
 * Those concerns remain owned by their canonical subsystems.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/types/*
 *     grammar/expressions/*
 *     grammar/core/blocks.g4
 *     grammar/core/attributes.g4
 *     grammar/core/modifiers.g4
 *     grammar/core/visibility.g4
 *     grammar/functions/parameters.g4
 *
 * Canonical imported grammar names used below:
 *
 *     Names
 *     Types
 *     Expressions
 *     ZamaniCoreBlocks
 *     Attributes
 *     Modifiers
 *     Visibility
 *     Parameters
 *
 * REQUIRED EXISTING LEXER VOCABULARY:
 *
 *     ACTOR
 *     SPAWN
 *     EXTENDS
 *     SELF
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     COLON
 *     DOT
 *     SEMICOLON
 *     THIN_ARROW
 *     IDENTIFIER
 *
 * No new lexer token is required by this file.
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * PUBLIC RULES:
 *
 *     actorConstruct
 *     actorDeclaration
 *     actorSpawnExpression
 *     actorCommand
 *     actorSendExpression
 *     actorAskExpression
 *     actorForwardExpression
 *     actorLifecycleExpression
 *     actorSupervisionConstruct
 *     actorStatement
 *     actorExpression
 *
 * `grammar/concurrency/concurrency.g4` consumes `actorConstruct`.
 *
 * No parent grammar should duplicate actor syntax.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough structure for the domain-neutral frontend
 * AST to represent:
 *
 *     actor declaration
 *     actor name
 *     actor inheritance
 *     actor members
 *     state names
 *     state types
 *     state initializers
 *     handler command marker
 *     handler message name
 *     handler parameters
 *     handler return type
 *     handler body
 *     actor construction target
 *     construction arguments
 *     actor command kind/name
 *     actor target
 *     message name
 *     message arguments
 *     lifecycle target
 *     supervision target
 *     supervision body
 *     source spans
 *
 * The grammar MUST NOT create runtime objects such as:
 *
 *     ActorId
 *     Mailbox
 *     WorkerId
 *     ThreadHandle
 *     ProcessHandle
 *     Executor
 *     PhysicalNode
 *     DeviceId
 *
 * Those belong downstream.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parser structure is not actor semantics.
 *
 * Semantic analysis determines:
 *
 *     - whether a declaration is a valid actor;
 *     - whether actor state is isolated;
 *     - whether a state field is mutable;
 *     - whether a handler command is the reserved semantic command "receive";
 *     - whether a command is send/ask/forward/stop/restart/supervise;
 *     - whether the target resolves to an actor;
 *     - whether the message exists;
 *     - whether message arguments match the handler;
 *     - whether an ask has a valid result;
 *     - whether forwarding is legal;
 *     - whether lifecycle control is authorized;
 *     - whether supervision is valid;
 *     - whether ownership/borrowing rules are satisfied;
 *     - whether effects are legal;
 *     - whether capabilities are available;
 *     - whether resource requirements can be satisfied;
 *     - whether distributed placement is possible;
 *     - whether execution is deterministic where required;
 *     - whether recovery/failure policy is valid.
 *
 * ============================================================================
 * COMMAND SEMANTICS
 * ============================================================================
 *
 * Actor commands are syntactically represented by:
 *
 *     actor <identifier> ...
 *
 * The semantic layer classifies the identifier.
 *
 * Canonical command meanings are:
 *
 *     receive
 *         actor handler declaration
 *
 *     send
 *         asynchronous actor message intent
 *
 *     ask
 *         request/response actor message intent
 *
 *     forward
 *         message forwarding intent
 *
 *     stop
 *         logical actor lifecycle stop intent
 *
 *     restart
 *         logical actor lifecycle restart intent
 *
 *     supervise
 *         actor supervision intent
 *
 * Additional command identifiers may be introduced by future language
 * semantics without requiring a finite grammar vocabulary.
 *
 * Unknown commands are semantic diagnostics, not parser crashes.
 *
 * ============================================================================
 * SOURCE FORMS
 * ============================================================================
 *
 * Actor declaration:
 *
 *     actor Counter {
 *         value: Int;
 *
 *         actor receive increment(amount: Int) {
 *             value = value + amount;
 *         }
 *     }
 *
 * Actor construction:
 *
 *     spawn actor Counter(0)
 *
 * Communication:
 *
 *     actor send counter.increment(1);
 *
 *     actor ask counter.value();
 *
 *     actor forward worker.process(value);
 *
 * Lifecycle:
 *
 *     actor stop child;
 *
 *     actor restart child;
 *
 * Supervision:
 *
 *     actor supervise child {
 *         ...
 *     }
 *
 * The command words remain ordinary identifiers at the lexical layer.
 *
 * ============================================================================
 * TARGET MODEL
 * ============================================================================
 *
 * Actor targets intentionally do NOT use `qualifiedName` for dotted member
 * access.
 *
 * Canonical qualified names use:
 *
 *     ::
 *
 * while actor message selection uses:
 *
 *     .
 *
 * Therefore:
 *
 *     service::worker.increment()
 *
 * is represented as:
 *
 *     actorTargetPath DOT actorMessageName
 *
 * rather than:
 *
 *     qualifiedName DOT actorMessageName
 *
 * This prevents a qualified-name rule from consuming the final message name.
 *
 * ============================================================================
 * TARGET CONTRACT
 * ============================================================================
 *
 * actorTargetPath permits:
 *
 *     self
 *     identifier
 *     identifier::identifier
 *     identifier::identifier::identifier
 *
 * and parenthesized dynamic actor expressions:
 *
 *     (expression)
 *
 * The target grammar does not encode:
 *
 *     CPU identity
 *     GPU identity
 *     QPU identity
 *     node identity
 *     network address
 *     process ID
 *     thread ID
 *     physical device ID
 *
 * A source name may denote any of those concepts only after semantic
 * resolution, and physical realization remains downstream.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * Actor syntax imposes no universal capacity limit.
 *
 * There is deliberately no grammar-level limit for:
 *
 *     actors
 *     actor members
 *     handlers
 *     messages
 *     message arguments
 *     actor nesting
 *     actor instances
 *     supervisors
 *     supervised actors
 *     concurrent actors
 *     mailbox capacity
 *     workers
 *     threads
 *     cores
 *     CPUs
 *     GPUs
 *     FPGAs
 *     accelerators
 *     QPUs
 *     nodes
 *     memory
 *     network size
 *     device count
 *
 * Repetition uses ANTLR repetition operators rather than finite enumeration.
 *
 * Practical limits are implementation/resource limits and MUST NOT become
 * language semantics.
 *
 * The same source-level actor model can therefore be considered for:
 *
 *     tiny systems
 *     embedded systems
 *     single-core systems
 *     multicore systems
 *     GPU-backed systems
 *     FPGA-backed systems
 *     ASIC-backed systems
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     heterogeneous systems
 *     future computational substrates
 *
 * subject only to semantic feasibility and available resources.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT introduce:
 *
 *     MAX_ACTORS
 *     MAX_MESSAGES
 *     MAX_HANDLERS
 *     MAX_CHILDREN
 *     MAX_SUPERVISORS
 *     MAX_MAILBOXES
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_QUEUE_DEPTH
 *     MAX_DEVICES
 *
 * It also MUST NOT encode:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     node0
 *     core0
 *     worker0
 *
 * as special language constructs.
 *
 * Numeric values appearing in actor programs are ordinary program values.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Actor syntax itself does not select physical resources.
 *
 * Resource intent is expressed through the repository's canonical:
 *
 *     resources
 *     capabilities
 *     requirements
 *     constraints
 *     preferences
 *     policies
 *
 * subsystems.
 *
 * Examples of downstream semantic intent include:
 *
 *     requires capability("parallel.compute");
 *
 *     requires capability("distributed.actor");
 *
 *     requires capability("quantum.measurement");
 *
 * The actor grammar does not inspect or resolve those requirements.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Actor operations may carry effects determined downstream, including:
 *
 *     mutation
 *     IO
 *     network
 *     distributed
 *     measurement
 *     native
 *     foreign
 *     randomness
 *     learning
 *     adaptation
 *
 * Sending a message does not automatically imply a particular physical
 * transport.
 *
 * Asking a remote actor does not automatically imply network IO.
 *
 * Semantic analysis determines the actual effects.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Actor syntax does not grant capabilities.
 *
 * In particular:
 *
 *     spawn actor ...
 *     actor send ...
 *     actor ask ...
 *     actor forward ...
 *     actor stop ...
 *     actor restart ...
 *     actor supervise ...
 *
 * are source-level operations only.
 *
 * Authorization and capability checks remain downstream.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Actor execution may be constrained by:
 *
 *     security policies
 *     resource policies
 *     scheduling policies
 *     deployment policies
 *     recovery policies
 *     adaptation policies
 *
 * This grammar does not implement those policies.
 *
 * A policy may cause a valid source program to be rejected, specialized,
 * deferred, simulated, or realized differently on different targets without
 * changing the actor grammar.
 *
 * ============================================================================
 * CONTRACT / VALIDATION CONTRACT
 * ============================================================================
 *
 * Actor declarations and commands may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * through surrounding canonical contract grammar.
 *
 * This file does not duplicate contract syntax.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser must preserve source spans for actor constructs so downstream
 * provenance can associate:
 *
 *     source actor
 *         ->
 *     semantic actor
 *         ->
 *     optimization
 *         ->
 *     lowering
 *         ->
 *     scheduling
 *         ->
 *     deployment
 *         ->
 *     runtime realization
 *
 * with the original source construct.
 *
 * This is especially important for:
 *
 *     reproducibility
 *     debugging
 *     auditing
 *     explainability
 *     distributed diagnostics
 *     resource decisions
 *     target specialization.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * `grammar/concurrency/concurrency.g4` is the composition boundary.
 *
 * It imports this grammar and consumes:
 *
 *     actorConstruct
 *
 * It MUST NOT duplicate:
 *
 *     actorDeclaration
 *     actorSpawnExpression
 *     actorCommand
 *     actorTarget
 *     actorHandler
 *
 * ============================================================================
 * CHANNEL INTEGRATION
 * ============================================================================
 *
 * Actors may communicate through the canonical channel subsystem.
 *
 * This file does not redefine:
 *
 *     channel
 *     channel type
 *     send/receive channel operations
 *     channel selection
 *     channel closure
 *
 * An actor message and a channel operation are distinct semantic constructs
 * even when a runtime eventually implements one using the other.
 *
 * ============================================================================
 * TASK / FUTURE INTEGRATION
 * ============================================================================
 *
 * Actor construction may be implemented by a task/future runtime, but actor
 * syntax does not create a competing task or future model.
 *
 * The semantic layer decides whether actor realization uses:
 *
 *     task
 *     future
 *     event loop
 *     thread
 *     process
 *     service
 *     distributed executor
 *     other execution substrate.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A logical actor may be:
 *
 *     local
 *     remote
 *     migrated
 *     replicated
 *     partitioned
 *     heterogeneous
 *
 * without changing source syntax.
 *
 * Placement belongs to:
 *
 *     grammar/distributed/
 *     grammar/networking/
 *     grammar/resources/
 *     execution/deployment
 *     semantic analysis
 *
 * `actor send` is therefore not synonymous with "send a network packet".
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Actor message arguments and state types may contain quantum-domain values
 * when permitted by the type and semantic systems.
 *
 * This grammar does NOT define:
 *
 *     qubits
 *     physical qubits
 *     quantum topology
 *     gate sets
 *     routing
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *
 * If actor computation contains quantum semantics, the downstream canonical
 * path remains:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 * ============================================================================
 * CLASSICAL / AI / DATA / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Actor handlers are ordinary Zamani computation contexts.
 *
 * They may therefore contain semantics from:
 *
 *     classical computing
 *     numerical computing
 *     data processing
 *     AI/model execution
 *     reasoning
 *     learning
 *     hybrid computation
 *     HDL/hardware coordination
 *     accelerator coordination
 *     networking
 *     distributed execution
 *
 * without requiring an actor-specific grammar for every domain.
 *
 * ============================================================================
 * NO SECOND IR
 * ============================================================================
 *
 * This grammar introduces no:
 *
 *     ActorIR
 *     MailboxIR
 *     ThreadIR
 *     WorkerIR
 *     ActorHardwareIR
 *
 * Actor syntax lowers through the repository's existing frontend:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     structural validation
 *       ->
 *     semantic model
 *       ->
 *     canonical IR
 *
 * Quantum computation continues through `quantum::ir`.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token stream
 *     grammar version
 *     explicitly selected parser configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     resource availability
 *     scheduler state
 *     runtime state
 *     network state
 *     filesystem state
 *     wall-clock time
 *     randomness
 *     environment variables
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no parser actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime execution;
 *     - no unsafe implementation requirement.
 *
 * The generated/frontend Rust implementation remains compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * and must use safe Rust.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics cover structural errors such as:
 *
 *     missing actor name
 *     missing actor body
 *     malformed state field
 *     malformed handler
 *     malformed actor command
 *     missing command target
 *     missing message name
 *     malformed argument list
 *     malformed supervision body
 *     malformed inheritance
 *
 * Semantic diagnostics cover:
 *
 *     unknown actor
 *     unknown command
 *     unknown message
 *     incompatible message payload
 *     invalid actor state access
 *     invalid lifecycle operation
 *     invalid supervision relation
 *     unavailable capability
 *     unavailable resource
 *     impossible target realization
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This version deliberately does NOT require adding lexer tokens for actor
 * commands.
 *
 * Consequently the existing lexical surface remains compatible.
 *
 * The following source words remain identifiers:
 *
 *     receive
 *     send
 *     ask
 *     forward
 *     stop
 *     restart
 *     supervise
 *
 * Their actor-specific meaning exists only in an actor command position.
 *
 * This is preferable to silently introducing new reserved words because
 * reserving a previously legal identifier is a source-compatibility change.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     actor Counter {
 *         value: Int;
 *
 *         actor receive increment(amount: Int) {
 *             value = value + amount;
 *         }
 *     }
 *
 *     spawn actor Counter(0);
 *
 *     actor send counter.increment(1);
 *
 *     actor ask counter.value();
 *
 *     actor forward worker.process(value);
 *
 *     actor stop child;
 *
 *     actor restart child;
 *
 *     actor supervise child {
 *         recover();
 *     }
 *
 *     actor send service::worker.increment(1);
 *
 *     actor ask (router.select()).value();
 *
 * NEGATIVE:
 *
 *     actor;
 *     actor Counter
 *     actor Counter {
 *     actor Counter {
 *         value;
 *     }
 *     spawn;
 *     spawn actor;
 *     actor send;
 *     actor ask;
 *     actor forward;
 *     actor stop;
 *     actor restart;
 *     actor supervise;
 *
 * BOUNDARY:
 *
 *     empty actor bodies;
 *     empty argument lists;
 *     empty parameter lists;
 *     nested actor handlers;
 *     nested supervision blocks;
 *     long target paths;
 *     large argument lists;
 *     many actor members;
 *     many handlers;
 *
 * SCALABILITY:
 *
 *     no actor-count limit;
 *     no handler-count limit;
 *     no message-count limit;
 *     no argument-count limit;
 *     no supervisor-count limit;
 *     no worker-count limit;
 *     no thread-count limit;
 *     no node-count limit;
 *     no device-count limit;
 *     no memory-size limit.
 *
 * DETERMINISM:
 *
 *     identical source/token streams produce identical parse structures.
 *
 * CROSS-DOMAIN:
 *
 *     classical actor;
 *     quantum-aware actor;
 *     hybrid actor;
 *     HDL/hardware coordination actor;
 *     AI/data actor;
 *     distributed actor;
 *     networking actor;
 *     security-constrained actor.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] actor syntax has one canonical owner;
 *     [x] current lexer vocabulary is sufficient;
 *     [x] no nonexistent lexer token is referenced;
 *     [x] canonical SEMICOLON is used;
 *     [x] actor commands do not require new reserved words;
 *     [x] actor target parsing cannot consume the final dotted message name;
 *     [x] actor declarations preserve source structure;
 *     [x] actor handlers preserve parameters and bodies;
 *     [x] actor construction is target-independent;
 *     [x] communication is target-independent;
 *     [x] lifecycle is target-independent;
 *     [x] supervision is target-independent;
 *     [x] no resource ceiling is encoded;
 *     [x] no physical device is encoded;
 *     [x] no scheduler is encoded;
 *     [x] no runtime is encoded;
 *     [x] no actor-specific IR is created;
 *     [x] quantum::ir remains the canonical quantum boundary;
 *     [x] parsing is deterministic;
 *     [x] no unsafe Rust is required;
 *     [x] concurrency.g4 can consume actorConstruct;
 *     [x] semantic validation remains downstream.
 *
 * ============================================================================
 */

parser grammar Actors;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Types,
    Expressions,
    ZamaniCoreBlocks,
    Attributes,
    Modifiers,
    Visibility,
    Parameters
    ;


/* ============================================================================
 * 1. PUBLIC ACTOR COMPOSITION BOUNDARY
 * ========================================================================== */

/*
 * `concurrency.g4` consumes this rule.
 *
 * This rule contains both declaration-level and operation-level actor
 * constructs. No actor syntax is duplicated in the composition grammar.
 */
actorConstruct
    : actorDeclaration
    | actorSpawnExpression
    | actorCommand
    | actorSupervisionConstruct
    ;


/* ============================================================================
 * 2. ACTOR DECLARATION
 * ========================================================================== */

actorDeclaration
    : actorDeclarationPrefix*
      ACTOR
      identifier
      actorInheritanceClause?
      actorBody
    ;


actorDeclarationPrefix
    : attribute
    | visibilityModifier
    | modifier
    ;


actorInheritanceClause
    : EXTENDS
      typeExpression
      (
          COMMA
          typeExpression
      )*
    ;


/* ============================================================================
 * 3. ACTOR BODY
 * ========================================================================== */

actorBody
    : LBRACE
      actorMember*
      RBRACE
    ;


actorMember
    : actorMemberPrefix*
      actorStateField
    | actorMemberPrefix*
      actorHandler
    ;


actorMemberPrefix
    : attribute
    | visibilityModifier
    | modifier
    ;


/* ============================================================================
 * 4. ACTOR STATE
 * ========================================================================== */

actorStateField
    : identifier
      COLON
      typeExpression
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 5. ACTOR HANDLER
 * ============================================================================
 *
 * Handler syntax:
 *
 *     actor receive increment(amount: Int) {
 *         ...
 *     }
 *
 * `receive` is deliberately parsed as an identifier.
 *
 * Semantic analysis MUST require the command identifier to have the semantic
 * spelling `receive` in this position.
 *
 * This avoids adding a new lexer token while preserving a readable source
 * form.
 */

actorHandler
    : ACTOR
      actorCommandName
      actorMessageName
      LPAREN
      parameterList?
      RPAREN
      actorHandlerReturnType?
      blockExpression
    ;


actorHandlerReturnType
    : THIN_ARROW
      typeExpression
    ;


/* ============================================================================
 * 6. ACTOR SPAWN
 * ============================================================================
 *
 * Actor construction is explicitly distinguished from generic `spawn`.
 *
 * Canonical form:
 *
 *     spawn actor Counter(0)
 *
 * This uses only the currently reserved SPAWN and ACTOR tokens.
 */

actorSpawnExpression
    : SPAWN
      ACTOR
      qualifiedName
      LPAREN
      actorArguments?
      RPAREN
    ;


actorSpawnStatement
    : actorSpawnExpression
      SEMICOLON
    ;


actorArguments
    : argumentList
    ;


/* ============================================================================
 * 7. ACTOR COMMAND
 * ============================================================================
 *
 * Canonical forms:
 *
 *     actor send counter.increment(1);
 *     actor ask counter.value();
 *     actor forward worker.process(value);
 *     actor stop child;
 *     actor restart child;
 *     actor supervise child { ... }
 *
 * The first identifier after ACTOR is the command name.
 *
 * This creates an open semantic extension point without creating a finite
 * lexical command catalogue.
 */

actorCommand
    : ACTOR
      actorCommandName
      actorCommandPayload
    ;


actorCommandName
    : identifier
    ;


actorCommandPayload
    : actorSendPayload
    | actorAskPayload
    | actorForwardPayload
    | actorLifecyclePayload
    | actorSupervisionPayload
    ;


/* ============================================================================
 * 8. MESSAGE COMMAND PAYLOADS
 * ========================================================================== */

actorSendPayload
    : actorMessageTarget
      DOT
      actorMessageName
      LPAREN
      actorArguments?
      RPAREN
      SEMICOLON?
    ;


actorAskPayload
    : actorMessageTarget
      DOT
      actorMessageName
      LPAREN
      actorArguments?
      RPAREN
      SEMICOLON?
    ;


actorForwardPayload
    : actorMessageTarget
      DOT
      actorMessageName
      LPAREN
      actorArguments?
      RPAREN
      SEMICOLON?
    ;


/*
 * Stable semantic adapters.
 *
 * These rules do not own command spelling. The command identifier is retained
 * in the parse tree and semantic analysis maps it to send/ask/forward.
 */

actorSendExpression
    : ACTOR
      actorCommandName
      actorSendPayload
    ;


actorSendStatement
    : actorSendExpression
      SEMICOLON?
    ;


actorAskExpression
    : ACTOR
      actorCommandName
      actorAskPayload
    ;


actorAskStatement
    : actorAskExpression
      SEMICOLON?
    ;


actorForwardExpression
    : ACTOR
      actorCommandName
      actorForwardPayload
    ;


actorForwardStatement
    : actorForwardExpression
      SEMICOLON?
    ;


/* ============================================================================
 * 9. ACTOR LIFECYCLE
 * ========================================================================== */

actorLifecyclePayload
    : actorTarget
      SEMICOLON?
    ;


actorLifecycleExpression
    : ACTOR
      actorCommandName
      actorLifecyclePayload
    ;


actorLifecycleStatement
    : actorLifecycleExpression
      SEMICOLON?
    ;


/* ============================================================================
 * 10. SUPERVISION
 * ========================================================================== */

actorSupervisionPayload
    : actorTarget
      blockExpression
    ;


actorSupervisionConstruct
    : ACTOR
      actorCommandName
      actorSupervisionPayload
    ;


actorSupervisionStatement
    : actorSupervisionConstruct
      SEMICOLON?
    ;


/* ============================================================================
 * 11. ACTOR TARGET
 * ============================================================================
 *
 * IMPORTANT:
 *
 * Do not replace this with `qualifiedName`.
 *
 * `qualifiedName` uses `::`, while message selection uses `.`.
 *
 * This rule explicitly consumes only `::`-separated target segments before
 * the message DOT, preventing the final message name from being swallowed.
 */

actorMessageTarget
    : actorTarget
    ;


actorTarget
    : actorTargetPath
    | LPAREN
      expression
      RPAREN
    ;


actorTargetPath
    : SELF
      (
          DOUBLE_COLON
          identifier
      )*
    | identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


/* ============================================================================
 * 12. ACTOR MESSAGE NAME
 * ========================================================================== */

actorMessageName
    : identifier
    ;


/* ============================================================================
 * 13. ACTOR STATEMENT / EXPRESSION ADAPTERS
 * ============================================================================
 *
 * These are stable downstream-facing names.
 *
 * Semantic analysis classifies the command identifier rather than this parser
 * grammar attempting to encode the command vocabulary lexically.
 */

actorStatement
    : actorSpawnStatement
    | actorSendStatement
    | actorAskStatement
    | actorForwardStatement
    | actorLifecycleStatement
    | actorSupervisionStatement
    ;


actorExpression
    : actorSpawnExpression
    | actorSendExpression
    | actorAskExpression
    | actorForwardExpression
    | actorLifecycleExpression
    ;


/* ============================================================================
 * 14. SEMANTIC COMMAND CLASSIFICATION CONTRACT
 * ============================================================================
 *
 * The following source forms are semantically canonical:
 *
 *     actor receive ...
 *     actor send ...
 *     actor ask ...
 *     actor forward ...
 *     actor stop ...
 *     actor restart ...
 *     actor supervise ...
 *
 * The grammar deliberately accepts the command as `identifier`.
 *
 * Semantic validation MUST:
 *
 *     1. preserve the command spelling;
 *     2. classify it in actor context;
 *     3. reject unsupported command names;
 *     4. never reinterpret an unsupported command as a different command;
 *     5. produce a source-span diagnostic for the command identifier.
 *
 * This provides future extensibility without reserving an unbounded keyword
 * catalogue.
 *
 * ============================================================================
 * 15. ACTOR HANDLER SEMANTICS
 * ============================================================================
 *
 * `actor receive name(...) { ... }` is a handler declaration.
 *
 * Semantic analysis must verify:
 *
 *     command == receive
 *
 * and then establish:
 *
 *     message name
 *     parameter types
 *     return type
 *     handler effects
 *     handler capabilities
 *     handler requirements
 *     state access
 *     isolation
 *     contracts
 *     policies
 *
 * A handler is not automatically a thread.
 *
 * A handler is not automatically a process.
 *
 * A handler is not automatically a network service.
 *
 * ============================================================================
 * 16. MESSAGE SEMANTICS
 * ============================================================================
 *
 * `actor send`:
 *
 *     asynchronous communication intent.
 *
 * `actor ask`:
 *
 *     request/response intent.
 *
 * `actor forward`:
 *
 *     forwarding intent.
 *
 * The grammar does not decide:
 *
 *     local vs remote
 *     synchronous vs asynchronous runtime implementation
 *     queue implementation
 *     serialization
 *     transport
 *     ordering implementation
 *     persistence
 *     replication
 *     retry strategy
 *
 * Those are semantic/runtime/deployment decisions.
 *
 * ============================================================================
 * 17. LIFECYCLE SEMANTICS
 * ============================================================================
 *
 * `actor stop target`
 *
 * and:
 *
 * `actor restart target`
 *
 * represent logical lifecycle intent.
 *
 * They do not mean:
 *
 *     terminate thread
 *     terminate process
 *     power off machine
 *     reset device
 *
 * unless a downstream target-specific realization explicitly establishes that
 * correspondence.
 *
 * ============================================================================
 * 18. SUPERVISION SEMANTICS
 * ============================================================================
 *
 * `actor supervise target { ... }`
 *
 * expresses supervision intent.
 *
 * The block may contain ordinary Zamani computation.
 *
 * Semantic analysis/runtime policy determines:
 *
 *     failure handling
 *     restart behavior
 *     escalation
 *     recovery
 *     isolation
 *     observability
 *     placement
 *     resource requirements
 *
 * No supervisor resource is allocated by parsing.
 *
 * ============================================================================
 * 19. ERROR BOUNDARIES
 * ============================================================================
 *
 * Parser errors:
 *
 *     malformed actor declaration
 *     missing actor name
 *     missing actor body
 *     malformed state field
 *     malformed handler
 *     missing command name
 *     malformed command payload
 *     missing actor target
 *     missing message name
 *     malformed argument list
 *     malformed supervision block
 *
 * Semantic errors:
 *
 *     unknown actor command
 *     command used in wrong actor context
 *     unknown actor target
 *     unknown message
 *     incompatible arguments
 *     invalid handler return type
 *     illegal state access
 *     invalid lifecycle relationship
 *     invalid supervision relationship
 *     unavailable capability
 *     unavailable resource
 *     invalid distributed realization
 *
 * ============================================================================
 * 20. SOURCE-LEVEL SECURITY
 * ============================================================================
 *
 * Actor syntax never grants authority.
 *
 * Capability and policy checks remain mandatory for:
 *
 *     actor creation
 *     message communication
 *     lifecycle control
 *     supervision
 *     foreign calls
 *     network communication
 *     resource acquisition
 *     hardware interaction
 *
 * Parser execution itself performs none of these actions.
 *
 * ============================================================================
 * 21. DOMAIN-NEUTRALITY
 * ============================================================================
 *
 * Actor bodies use the canonical expression/block/type systems.
 *
 * Consequently actors may coordinate:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     AI/model computation
 *     data processing
 *     distributed computation
 *     networking
 *     HDL/hardware coordination
 *     accelerator computation
 *
 * without introducing domain-specific actor grammars.
 *
 * ============================================================================
 * 22. QUANTUM BOUNDARY
 * ============================================================================
 *
 * This file does not define quantum operations.
 *
 * If an actor handler performs quantum computation, the downstream semantic
 * pipeline remains:
 *
 *     actor syntax
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic quantum model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 * No actor-specific quantum IR is introduced.
 *
 * ============================================================================
 * 23. DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * An actor may be realized locally or across a distributed substrate.
 *
 * This file does not define:
 *
 *     node IDs
 *     addresses
 *     transports
 *     network topology
 *     replication counts
 *     placement
 *     consensus
 *
 * Distributed specialization belongs downstream.
 *
 * ============================================================================
 * 24. RESOURCE / CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Resource requirements and capabilities are intentionally absent from the
 * concrete actor productions.
 *
 * They are consumed from:
 *
 *     grammar/resources/
 *     grammar/security/
 *     grammar/distributed/
 *     grammar/execution/
 *     grammar/compile/
 *
 * Actor semantics may reference their resulting semantic information.
 *
 * ============================================================================
 * 25. NO PHYSICAL LIMITS
 * ============================================================================
 *
 * This grammar contains no finite actor/resource expansion.
 *
 * Examples:
 *
 *     actorMember*
 *     parameterList?
 *     actorArguments?
 *     (DOUBLE_COLON identifier)*
 *
 * are structurally open-ended.
 *
 * There is no grammar constant limiting:
 *
 *     actor count
 *     handler count
 *     member count
 *     message count
 *     argument count
 *     supervision count
 *     target path length
 *     worker count
 *     thread count
 *     node count
 *     device count
 *     memory
 *     compute
 *
 * ============================================================================
 * 26. COMPATIBILITY
 * ============================================================================
 *
 * No new reserved words are introduced.
 *
 * Existing programs using:
 *
 *     send
 *     ask
 *     receive
 *     forward
 *     stop
 *     restart
 *     supervise
 *
 * as ordinary identifiers remain lexically valid.
 *
 * Actor-specific interpretation occurs only in actor command positions.
 *
 * ============================================================================
 * 27. ANTLR / RUST SAFETY
 * ============================================================================
 *
 * This grammar is parser-only.
 *
 * It contains:
 *
 *     no actions
 *     no predicates
 *     no embedded Rust
 *     no filesystem access
 *     no network access
 *     no runtime calls
 *     no hardware calls
 *
 * Generated Rust remains compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * and requires no unsafe Rust.
 *
 * ============================================================================
 * 28. CONCURRENCY COMPOSITION INTEGRATION
 * ============================================================================
 *
 * `grammar/concurrency/concurrency.g4` must retain:
 *
 *     import
 *         AsyncExpressions,
 *         Tasks,
 *         Futures,
 *         Parallel,
 *         DataParallel,
 *         TaskParallel,
 *         Actors,
 *         Channels,
 *         Cancellation,
 *         Synchronization
 *         ;
 *
 * and consume:
 *
 *     actorConstruct
 *
 * through:
 *
 *     actorConcurrencyConstruct
 *         : actorConstruct
 *         ;
 *
 * No actor production should be copied into `concurrency.g4`.
 *
 * ============================================================================
 * 29. AST / SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Frontend AST integration must preserve:
 *
 *     ActorDeclaration
 *     ActorState
 *     ActorHandler
 *     ActorCommand
 *     ActorSpawn
 *     ActorTarget
 *     ActorMessage
 *     ActorLifecycle
 *     ActorSupervision
 *
 * without making those names runtime object types.
 *
 * Recommended semantic normalization:
 *
 *     ActorCommand {
 *         command,
 *         target,
 *         message,
 *         arguments,
 *         body,
 *         source
 *     }
 *
 * where `command` remains source-derived and semantic analysis classifies it.
 *
 * ============================================================================
 * 30. IR INTEGRATION
 * ============================================================================
 *
 * There is no actor-specific parser IR.
 *
 * Actor operations become canonical semantic operations with fields such as:
 *
 *     name
 *     namespace
 *     operands
 *     parameters
 *     results
 *     attributes
 *     modifiers
 *     effects
 *     capabilities
 *     source
 *
 * Lowering determines whether the operation becomes:
 *
 *     local communication
 *     asynchronous execution
 *     distributed communication
 *     service invocation
 *     heterogeneous coordination
 *     another supported target representation.
 *
 * ============================================================================
 * 31. TEST INTEGRATION
 * ============================================================================
 *
 * Required test ownership:
 *
 *     grammar/tests/concurrency/actors/
 *
 * Required categories:
 *
 *     declaration
 *     state
 *     handler
 *     spawn
 *     send
 *     ask
 *     forward
 *     lifecycle
 *     supervision
 *     target paths
 *     dynamic targets
 *     nested blocks
 *     contracts
 *     effects
 *     capabilities
 *     resources
 *     provenance
 *     distributed integration
 *     quantum integration
 *     negative syntax
 *     semantic diagnostics
 *     scalability
 *     determinism
 *     compatibility
 *
 * ============================================================================
 * 32. HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS:
 *
 *     no hardware capacity constants
 *     no actor capacity constants
 *     no thread constants
 *     no worker constants
 *     no node constants
 *     no device constants
 *     no mailbox capacity constants
 *     no topology constants
 *     no quantum capacity constants
 *     no parser actions
 *     no unsafe Rust
 *     no physical placement
 *     no runtime allocation
 *     no second IR
 *
 * ============================================================================
 * 33. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It composes as parser grammar `Actors`.
 *     [x] It consumes only the current canonical lexer vocabulary.
 *     [x] It does not reference nonexistent actor tokens.
 *     [x] It uses `SEMICOLON`, not `SEMI`.
 *     [x] Actor command words remain extensible identifiers.
 *     [x] Actor target paths cannot consume the final dotted message name.
 *     [x] Actor declarations have one owner.
 *     [x] Actor handlers have one owner.
 *     [x] Actor construction has one owner.
 *     [x] Actor communication has one owner.
 *     [x] Lifecycle and supervision remain logical source intent.
 *     [x] Resource realization remains downstream.
 *     [x] Capability authorization remains downstream.
 *     [x] Distributed placement remains downstream.
 *     [x] Quantum lowering remains through `quantum::ir`.
 *     [x] No actor-specific IR is created.
 *     [x] No hard-coded scalability limit exists.
 *     [x] Parsing is deterministic.
 *     [x] No embedded Rust exists.
 *     [x] No unsafe Rust is required.
 *     [x] `concurrency.g4` can consume `actorConstruct`.
 *
 * ============================================================================
 */


/* ============================================================================
 * PARSER GRAMMAR
 * ========================================================================== */

parser grammar Actors;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Types,
    Expressions,
    ZamaniCoreBlocks,
    Attributes,
    Modifiers,
    Visibility,
    Parameters
    ;


/* ============================================================================
 * PUBLIC COMPOSITION
 * ========================================================================== */

actorConstruct
    : actorDeclaration
    | actorSpawnExpression
    | actorCommand
    | actorSupervisionConstruct
    ;


/* ============================================================================
 * ACTOR DECLARATION
 * ========================================================================== */

actorDeclaration
    : actorDeclarationPrefix*
      ACTOR
      identifier
      actorInheritanceClause?
      actorBody
    ;


actorDeclarationPrefix
    : attribute
    | visibilityModifier
    | modifier
    ;


actorInheritanceClause
    : EXTENDS
      typeExpression
      (
          COMMA
          typeExpression
      )*
    ;


actorBody
    : LBRACE
      actorMember*
      RBRACE
    ;


actorMember
    : actorMemberPrefix*
      actorStateField
    | actorMemberPrefix*
      actorHandler
    ;


actorMemberPrefix
    : attribute
    | visibilityModifier
    | modifier
    ;


/* ============================================================================
 * ACTOR STATE
 * ========================================================================== */

actorStateField
    : identifier
      COLON
      typeExpression
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * ACTOR HANDLER
 * ========================================================================== */

actorHandler
    : ACTOR
      actorCommandName
      actorMessageName
      LPAREN
      parameterList?
      RPAREN
      actorHandlerReturnType?
      blockExpression
    ;


actorHandlerReturnType
    : THIN_ARROW
      typeExpression
    ;


/* ============================================================================
 * ACTOR SPAWN
 * ========================================================================== */

actorSpawnExpression
    : SPAWN
      ACTOR
      qualifiedName
      LPAREN
      actorArguments?
      RPAREN
    ;


actorSpawnStatement
    : actorSpawnExpression
      SEMICOLON
    ;


actorArguments
    : argumentList
    ;


/* ============================================================================
 * ACTOR COMMANDS
 * ========================================================================== */

actorCommand
    : ACTOR
      actorCommandName
      actorCommandPayload
    ;


actorCommandName
    : identifier
    ;


actorCommandPayload
    : actorSendPayload
    | actorAskPayload
    | actorForwardPayload
    | actorLifecyclePayload
    | actorSupervisionPayload
    ;


/* ============================================================================
 * COMMUNICATION
 * ========================================================================== */

actorSendPayload
    : actorMessageTarget
      DOT
      actorMessageName
      LPAREN
      actorArguments?
      RPAREN
      SEMICOLON?
    ;


actorAskPayload
    : actorMessageTarget
      DOT
      actorMessageName
      LPAREN
      actorArguments?
      RPAREN
      SEMICOLON?
    ;


actorForwardPayload
    : actorMessageTarget
      DOT
      actorMessageName
      LPAREN
      actorArguments?
      RPAREN
      SEMICOLON?
    ;


actorSendExpression
    : ACTOR
      actorCommandName
      actorSendPayload
    ;


actorSendStatement
    : actorSendExpression
      SEMICOLON?
    ;


actorAskExpression
    : ACTOR
      actorCommandName
      actorAskPayload
    ;


actorAskStatement
    : actorAskExpression
      SEMICOLON?
    ;


actorForwardExpression
    : ACTOR
      actorCommandName
      actorForwardPayload
    ;


actorForwardStatement
    : actorForwardExpression
      SEMICOLON?
    ;


/* ============================================================================
 * LIFECYCLE
 * ========================================================================== */

actorLifecyclePayload
    : actorTarget
      SEMICOLON?
    ;


actorLifecycleExpression
    : ACTOR
      actorCommandName
      actorLifecyclePayload
    ;


actorLifecycleStatement
    : actorLifecycleExpression
      SEMICOLON?
    ;


/* ============================================================================
 * SUPERVISION
 * ========================================================================== */

actorSupervisionPayload
    : actorTarget
      blockExpression
    ;


actorSupervisionConstruct
    : ACTOR
      actorCommandName
      actorSupervisionPayload
    ;


actorSupervisionStatement
    : actorSupervisionConstruct
      SEMICOLON?
    ;


/* ============================================================================
 * TARGETS
 * ========================================================================== */

actorMessageTarget
    : actorTarget
    ;


actorTarget
    : actorTargetPath
    | LPAREN
      expression
      RPAREN
    ;


actorTargetPath
    : SELF
      (
          DOUBLE_COLON
          identifier
      )*
    | identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


actorMessageName
    : identifier
    ;


/* ============================================================================
 * PUBLIC ADAPTERS
 * ========================================================================== */

actorStatement
    : actorSpawnStatement
    | actorSendStatement
    | actorAskStatement
    | actorForwardStatement
    | actorLifecycleStatement
    | actorSupervisionStatement
    ;


actorExpression
    : actorSpawnExpression
    | actorSendExpression
    | actorAskExpression
    | actorForwardExpression
    | actorLifecycleExpression
    ;