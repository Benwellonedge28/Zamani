/*
 * ============================================================================
 * Zamani Programming Language
 * Production Actor-Concurrency Grammar
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/actors.g4
 *
 * Grammar:
 *     Actors
 *
 * Status:
 *     PRODUCTION SOURCE-GRAMMAR CONTRACT
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical parser grammar for Zamani actor-oriented
 * concurrency.
 *
 * Actors are logical concurrent computation boundaries.
 *
 * Actor syntax describes:
 *
 *     - actor declarations;
 *     - actor state;
 *     - message handlers;
 *     - actor construction;
 *     - actor references;
 *     - asynchronous message sending;
 *     - request/ask interactions;
 *     - forwarding;
 *     - lifecycle control;
 *     - supervision intent.
 *
 * It does NOT describe how actors are physically realized.
 *
 * An actor may ultimately be implemented by:
 *
 *     - cooperative execution;
 *     - an async task;
 *     - a thread;
 *     - a process;
 *     - an event-loop participant;
 *     - a local service;
 *     - a distributed service;
 *     - a heterogeneous execution context;
 *     - an accelerator-backed computation;
 *     - a future computational substrate.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     actorConstruct
 *     actorDeclaration
 *     actorMember
 *     actorStateField
 *     actorHandler
 *     actorLifecycleHandler
 *     actorSpawnExpression
 *     actorSpawnStatement
 *     actorSendExpression
 *     actorSendStatement
 *     actorAskExpression
 *     actorForwardExpression
 *     actorLifecycleExpression
 *     actorSupervisionConstruct
 *     actorTarget
 *     actorMessageName
 *     actorArguments
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
 *     parameters
 *     async/await/spawn/parallel expression semantics generally
 *     channels
 *     futures
 *     synchronization
 *     scheduling
 *     routing
 *     resource discovery
 *     hardware discovery
 *     distributed placement
 *     quantum::ir
 *     classical IR
 *     HDL/Hardware IR
 *     QEC
 *     ZQN
 *     HAL
 *     runtime implementation
 *
 * ============================================================================
 * CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * Canonical reusable syntax is imported rather than redefined.
 *
 *     Names
 *         identifier / qualifiedName
 *
 *     Types
 *         typeExpression
 *
 *     Expressions
 *         expression
 *
 *     ZamaniCoreBlocks
 *         block / blockExpression
 *
 *     Attributes
 *         attribute
 *
 *     Modifiers
 *         modifier
 *
 *     Visibility
 *         visibilityModifier
 *
 *     Parameters
 *         parameterList
 *
 * Actor syntax must not create competing definitions of these rules.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * Actor syntax contains NO universal implementation limits.
 *
 * It does not define:
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
 *
 * Actor count, message volume, execution parallelism, placement, mailbox
 * realization and resource consumption are downstream concerns.
 *
 * Consequently the same actor program can be lowered to:
 *
 *     tiny systems
 *     embedded systems
 *     single-core systems
 *     multicore CPUs
 *     GPUs
 *     FPGAs
 *     ASIC-backed systems
 *     distributed systems
 *     clusters
 *     cloud systems
 *     heterogeneous systems
 *     future computational substrates
 *
 * subject to actual semantic requirements and available resources.
 *
 * ============================================================================
 * HARD-CODING RULE
 * ============================================================================
 *
 * Numeric literals inside actor programs remain ordinary program values.
 *
 * For example:
 *
 *     let retries = 3;
 *
 * is valid program semantics.
 *
 * But the grammar MUST NOT interpret 3 as a universal actor-system limit.
 *
 * Likewise, actor syntax must never select:
 *
 *     CPU 0
 *     GPU 0
 *     QPU 0
 *     physical node 0
 *     physical core 0
 *     physical address
 *     fixed mailbox capacity
 *     fixed worker count
 *
 * ============================================================================
 * SEMANTIC PRINCIPLE
 * ============================================================================
 *
 * The following equivalences are NOT implied:
 *
 *     actor    == thread
 *     actor    == process
 *     actor    == machine
 *     actor    == node
 *     mailbox  == queue
 *     message  == network packet
 *
 * They are possible implementation choices.
 *
 * The semantic/runtime layers decide how the logical actor model is realized.
 *
 * ============================================================================
 * MESSAGE MODEL
 * ============================================================================
 *
 * Message names are identifiers.
 *
 * The grammar does not enumerate a fixed message vocabulary.
 *
 * Therefore:
 *
 *     send counter.increment(1)
 *
 * and:
 *
 *     send quantum_controller.measure(qubit)
 *
 * can share the same source-level communication model.
 *
 * Whether a message is local, remote, distributed, persistent, replicated,
 * secure, quantum/classical, or accelerator-backed is determined downstream.
 *
 * ============================================================================
 * ACTOR ISOLATION
 * ============================================================================
 *
 * Actor state is syntactically contained within the actor declaration.
 *
 * The grammar does not itself implement:
 *
 *     ownership;
 *     borrowing;
 *     isolation;
 *     race detection;
 *     effect checking;
 *     capability checking.
 *
 * Those are semantic-analysis responsibilities.
 *
 * ============================================================================
 * PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     Actors parser grammar
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *     +----+---------+-------------+-------------+
 *     |              |             |             |
 *     v              v             v             v
 * classical       quantum      distributed    hardware
 * semantics       semantics     semantics      semantics
 *     |              |             |             |
 *     +--------------+-------------+-------------+
 *                            |
 *                            v
 *                 canonical semantic IR
 *                            |
 *                 optimization / lowering
 *                            |
 *                 routing / scheduling
 *                            |
 *                      resilience
 *                            |
 *                         runtime
 *
 * If actor computation contains quantum semantics, the canonical quantum
 * lowering path remains:
 *
 *     semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * This grammar creates no quantum IR.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar must preserve:
 *
 *     - actor declaration structure;
 *     - actor name;
 *     - actor state declarations;
 *     - handler names;
 *     - handler parameters;
 *     - handler return type;
 *     - handler body;
 *     - message target;
 *     - message name;
 *     - message arguments;
 *     - lifecycle construct;
 *     - supervision construct;
 *     - complete source spans.
 *
 * The grammar must NOT create runtime objects such as:
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
 * Semantic analysis determines:
 *
 *     - actor identity;
 *     - actor type validity;
 *     - state isolation;
 *     - message compatibility;
 *     - handler resolution;
 *     - send legality;
 *     - ask/request result type;
 *     - forwarding legality;
 *     - lifecycle validity;
 *     - supervision policy;
 *     - effects;
 *     - capabilities;
 *     - ownership;
 *     - borrowing;
 *     - synchronization;
 *     - determinism;
 *     - resource requirements;
 *     - distributed placement;
 *     - failure/recovery semantics.
 *
 * None of those semantic decisions are encoded by parser actions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * actors.g4 produces parser structure only.
 *
 * There is no actor-specific competing IR introduced here.
 *
 * Actor semantics must lower into the repository's canonical semantic/IR
 * architecture.
 *
 * Message operations may ultimately participate in:
 *
 *     classical computation;
 *     quantum/classical orchestration;
 *     distributed computation;
 *     hardware/software co-design;
 *     AI/data pipelines;
 *     networking;
 *     security;
 *     future domains.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Actor syntax never means:
 *
 *     use N threads
 *     use N cores
 *     use N nodes
 *     use device N
 *
 * Resource requirements and capabilities belong to:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/distributed/
 *     grammar/compile/
 *     grammar/execution/
 *
 * and their semantic consumers.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Writing:
 *
 *     spawn actor ...
 *     send ...
 *     ask ...
 *     supervise ...
 *
 * does not itself grant any capability.
 *
 * Semantic/runtime authorization must still enforce:
 *
 *     isolation;
 *     ownership;
 *     permissions;
 *     capability policies;
 *     resource policies;
 *     security policies;
 *     deployment policies.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source tokens;
 *     grammar version.
 *
 * Parsing must NOT depend on:
 *
 *     hardware;
 *     available workers;
 *     system time;
 *     randomness;
 *     environment variables;
 *     network state;
 *     runtime state;
 *     scheduler state.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser-level diagnostics cover structural errors:
 *
 *     missing actor name;
 *     missing actor body;
 *     missing handler name;
 *     malformed parameter list;
 *     missing message target;
 *     missing message name;
 *     missing message arguments;
 *     malformed supervision body;
 *     malformed lifecycle construct.
 *
 * Semantic diagnostics remain downstream:
 *
 *     unknown actor;
 *     unknown message;
 *     invalid message payload;
 *     illegal state access;
 *     invalid supervision relationship;
 *     unavailable capability;
 *     unavailable resource;
 *     invalid quantum/hardware realization.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The existing concurrency composition boundary is:
 *
 *     grammar/concurrency/concurrency.g4
 *
 * This file therefore exports stable public rules:
 *
 *     actorConstruct
 *     actorDeclaration
 *     actorSpawnExpression
 *     actorSendExpression
 *     actorAskExpression
 *     actorForwardExpression
 *     actorLifecycleExpression
 *     actorSupervisionConstruct
 *     actorStatement
 *
 * `concurrency.g4` should consume `actorConstruct` rather than duplicate
 * actor syntax.
 *
 * ============================================================================
 * RUST / ANTLR CONTRACT
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
 *     - no unsafe code.
 *
 * Generated parser integration remains compatible with the repository's
 * Rust 1.97 / 1.97.1 safe-Rust implementation.
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
 * 1. PUBLIC ACTOR CONSTRUCT
 * ========================================================================== */

/*
 * Single actor-domain parser boundary.
 *
 * This rule is what concurrency.g4 should consume.
 */
actorConstruct
    : actorDeclaration
    | actorSpawnExpression
    | actorSendExpression
    | actorAskExpression
    | actorForwardExpression
    | actorLifecycleExpression
    | actorSupervisionConstruct
    ;


/* ============================================================================
 * 2. ACTOR DECLARATION
 * ========================================================================== */

/*
 * Canonical form:
 *
 *     actor Counter {
 *         value: Int;
 *
 *         receive increment(amount: Int) {
 *             ...
 *         }
 *     }
 *
 * Actor generic declarations are deliberately not redefined here.
 *
 * Generic actor types should use the canonical language-wide type-parameter
 * contract once that contract is exposed by the central type/declaration
 * composition layer. This prevents actors.g4 from creating a second generic
 * parameter grammar.
 */
actorDeclaration
    : actorDeclarationPrefix*
      ACTOR
      identifier
      actorInheritanceClause?
      actorBody
    ;


/*
 * Declaration prefixes are limited to canonical constructs.
 *
 * They are syntax only. Legality is semantic.
 */
actorDeclarationPrefix
    : attribute
    | visibilityModifier
    | modifier
    ;


/* ============================================================================
 * 3. ACTOR INHERITANCE / CONTRACT RELATION
 * ========================================================================== */

actorInheritanceClause
    : EXTENDS typeExpression
      (COMMA typeExpression)*
      COMMA?
    ;


/* ============================================================================
 * 4. ACTOR BODY
 * ========================================================================== */

actorBody
    : LBRACE
      actorMember*
      RBRACE
    ;


/* ============================================================================
 * 5. ACTOR MEMBER
 * ========================================================================== */

actorMember
    : actorMemberPrefix*
      actorStateField
    | actorMemberPrefix*
      actorHandler
    | actorMemberPrefix*
      actorLifecycleHandler
    ;


/*
 * Actor members may carry ordinary source-level attributes, visibility and
 * modifiers. Their semantic compatibility is checked downstream.
 */
actorMemberPrefix
    : attribute
    | visibilityModifier
    | modifier
    ;


/* ============================================================================
 * 6. ACTOR STATE
 * ========================================================================== */

/*
 * State is source-level actor-owned state.
 *
 * It does not specify physical memory placement.
 */
actorStateField
    : identifier
      COLON
      typeExpression
      (
          ASSIGN
          expression
      )?
      SEMI
    ;


/* ============================================================================
 * 7. MESSAGE HANDLER
 * ========================================================================== */

/*
 * Handler form:
 *
 *     receive increment(amount: Int) {
 *         ...
 *     }
 *
 * The message name remains ordinary identifier data.
 *
 * The language therefore does not need one keyword per message type.
 */
actorHandler
    : RECEIVE
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
 * 8. LIFECYCLE HANDLERS
 * ========================================================================== */

/*
 * Lifecycle operations use dedicated actor-prefixed lexical tokens so that:
 *
 *     start_actor
 *     stop_actor
 *     restart_actor
 *
 * cannot accidentally become ordinary message names in actor declarations.
 *
 * They remain logical lifecycle operations; they do not identify a runtime
 * thread/process/device.
 */
actorLifecycleHandler
    : START_ACTOR
      blockExpression
    | STOP_ACTOR
      blockExpression
    | RESTART_ACTOR
      blockExpression
    ;


/* ============================================================================
 * 9. ACTOR SPAWN
 * ========================================================================== */

/*
 * Actor construction is deliberately distinguished from generic:
 *
 *     spawn expression
 *
 * by the explicit:
 *
 *     spawn actor Name(...)
 *
 * form.
 *
 * This avoids an ambiguity between the generic async grammar and the actor
 * grammar.
 *
 * Example:
 *
 *     spawn actor Counter(0)
 *
 * The runtime decides how the actor is realized.
 */
actorSpawnExpression
    : SPAWN
      ACTOR
      qualifiedName
      LPAREN
      actorArguments?
      RPAREN
    ;


actorArguments
    : argumentList
    ;


/*
 * Statement adapter.
 */
actorSpawnStatement
    : actorSpawnExpression
      SEMI
    ;


/* ============================================================================
 * 10. ACTOR TARGET
 * ========================================================================== */

/*
 * The target is intentionally narrower than `expression`.
 *
 * This prevents:
 *
 *     send a.b(...)
 *
 * from becoming ambiguous because `expression` could consume `a.b` before
 * the actor message name is recognized.
 *
 * Parenthesized expressions permit dynamic actor references without making
 * arbitrary expressions part of the message-name decision.
 */
actorTarget
    : SELF
    | qualifiedName
    | LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 11. MESSAGE NAME
 * ========================================================================== */

actorMessageName
    : identifier
    ;


/* ============================================================================
 * 12. SEND
 * ========================================================================== */

/*
 * Canonical form:
 *
 *     send counter.increment(1)
 *
 *     send self.update(value)
 *
 *     send (router.select()).dispatch(message)
 *
 * The actual delivery mechanism is semantic/runtime behavior.
 */
actorSendExpression
    : SEND
      actorTarget
      DOT
      actorMessageName
      LPAREN
      actorArguments?
      RPAREN
    ;


actorSendStatement
    : actorSendExpression
      SEMI
    ;


/* ============================================================================
 * 13. ASK / REQUEST
 * ========================================================================== */

/*
 * Ask is request-style actor communication.
 *
 * Example:
 *
 *     ask counter.value()
 *
 * The returned expression is typed semantically.
 *
 * This grammar does not introduce Future<T>, Promise<T>, Task<T>, or any
 * runtime-specific future type.
 */
actorAskExpression
    : ASK
      actorTarget
      DOT
      actorMessageName
      LPAREN
      actorArguments?
      RPAREN
    ;


/* ============================================================================
 * 14. FORWARD
 * ========================================================================== */

/*
 * Forwarding passes a message to another actor.
 *
 * Example:
 *
 *     forward worker.process(value)
 *
 * Forwarding semantics, ordering and delivery guarantees are downstream.
 */
actorForwardExpression
    : FORWARD
      actorTarget
      DOT
      actorMessageName
      LPAREN
      actorArguments?
      RPAREN
    ;


actorForwardStatement
    : actorForwardExpression
      SEMI
    ;


/* ============================================================================
 * 15. LIFECYCLE CONTROL
 * ========================================================================== */

/*
 * These are logical actor lifecycle operations.
 *
 * They do not identify physical execution resources.
 */
actorLifecycleExpression
    : STOP_ACTOR
      actorTarget
    | RESTART_ACTOR
      actorTarget
    ;


actorLifecycleStatement
    : actorLifecycleExpression
      SEMI
    ;


/* ============================================================================
 * 16. SUPERVISION
 * ========================================================================== */

/*
 * Structured supervision intent:
 *
 *     supervise child {
 *         ...
 *     }
 *
 * The body describes supervision policy/behavior at source level.
 *
 * It does not allocate a supervisor thread, process or machine.
 */
actorSupervisionConstruct
    : SUPERVISE
      actorTarget
      blockExpression
    ;


actorSupervisionStatement
    : actorSupervisionConstruct
      SEMI?
    ;


/* ============================================================================
 * 17. ACTOR STATEMENT COMPOSITION
 * ========================================================================== */

/*
 * Stable statement-level actor boundary.
 *
 * Ordinary expression statements remain owned by statements/.
 */
actorStatement
    : actorSpawnStatement
    | actorSendStatement
    | actorForwardStatement
    | actorLifecycleStatement
    | actorSupervisionStatement
    ;


/* ============================================================================
 * 18. ACTOR EXPRESSION COMPOSITION
 * ========================================================================== */

actorExpression
    : actorSpawnExpression
    | actorSendExpression
    | actorAskExpression
    | actorForwardExpression
    | actorLifecycleExpression
    ;


/* ============================================================================
 * 19. RESOURCE-NEUTRALITY
 * ========================================================================== */

/*
 * No production in this file accepts:
 *
 *     worker count
 *     thread count
 *     core count
 *     GPU count
 *     QPU count
 *     node count
 *     physical actor ID
 *     physical address
 *     mailbox capacity
 *     device selection
 *     topology
 *
 * Resource and placement information must be expressed through the canonical
 * resource/hardware/deployment mechanisms and resolved downstream.
 */


/* ============================================================================
 * 20. CROSS-DOMAIN INTEGRATION
 * ========================================================================== */

/*
 * CLASSICAL
 *
 * Actor handler bodies can contain ordinary Zamani expressions/statements.
 *
 * QUANTUM
 *
 * Actor messages may carry quantum-domain values or request quantum
 * computation. If quantum semantics are present, lowering eventually crosses
 * the canonical quantum::ir boundary.
 *
 * HYBRID
 *
 * An actor may coordinate classical computation, quantum operations and
 * measurement/feed-forward without introducing another language.
 *
 * HDL / HARDWARE
 *
 * Actor messages may coordinate hardware/software co-design operations.
 * Physical realization remains outside this grammar.
 *
 * DISTRIBUTED
 *
 * An actor reference may resolve to a local or remote logical actor.
 * Location is semantic/deployment information.
 *
 * AI / DATA
 *
 * Actor messages may carry models, tensors, datasets, streams or results.
 * No framework-specific syntax is required here.
 *
 * NETWORKING
 *
 * Actor communication may eventually cross network boundaries, but `send`
 * does not itself mean "send a network packet".
 *
 * SECURITY
 *
 * Actor access remains subject to semantic capability and authorization
 * analysis.
 */


/* ============================================================================
 * 21. NO SECOND IR
 * ========================================================================== */

/*
 * This grammar never creates:
 *
 *     ActorIR
 *     MailboxIR
 *     ThreadIR
 *     WorkerIR
 *     ActorHardwareIR
 *
 * Actor constructs are lowered through the repository's existing canonical
 * semantic/IR architecture.
 *
 * If a future actor-specific semantic representation is needed, it must be
 * established outside this parser grammar and integrated through the existing
 * AST -> semantic model -> IR contract.
 */


/* ============================================================================
 * 22. DETERMINISTIC PARSING
 * ========================================================================== */

/*
 * Actor parsing depends only on the token stream.
 *
 * It must not inspect:
 *
 *     hardware;
 *     resources;
 *     runtime state;
 *     scheduler state;
 *     network state;
 *     environment variables;
 *     system time;
 *     randomness.
 */


/* ============================================================================
 * 23. NEGATIVE CONTRACT
 * ========================================================================== */

/*
 * The following are intentionally NOT actor grammar:
 *
 *     actor_on_cpu(0)
 *     actor_on_gpu(0)
 *     actor_on_qpu(0)
 *     actor_on_core(3)
 *     actor_with_threads(8)
 *     actor_with_workers(16)
 *     actor_on_node(2)
 *
 * Such target realization belongs downstream and must never become an
 * accidental universal actor API.
 *
 * Likewise, this grammar must not add:
 *
 *     MAX_ACTORS
 *     MAX_MESSAGES
 *     MAX_MAILBOXES
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_NODES
 */


/* ============================================================================
 * 24. TEST CONTRACT
 * ========================================================================== */

/*
 * POSITIVE
 *
 *     actor Counter {
 *         value: Int;
 *
 *         receive increment(amount: Int) {
 *             value = value + amount;
 *         }
 *     }
 *
 *     spawn actor Counter(0);
 *
 *     send counter.increment(1);
 *
 *     ask counter.value();
 *
 *     forward worker.process(value);
 *
 *     supervise child {
 *         recover();
 *     }
 *
 *     stop_actor child;
 *
 *     restart_actor child;
 *
 *
 * NEGATIVE
 *
 *     actor;
 *     actor Counter
 *     actor Counter {
 *     receive;
 *     send;
 *     send counter;
 *     send counter.increment;
 *     ask;
 *     forward;
 *     supervise;
 *
 *
 * BOUNDARY
 *
 *     empty actor bodies;
 *     empty parameter lists;
 *     empty argument lists;
 *     deeply nested handler blocks;
 *     deeply nested actor references;
 *     arbitrarily large message argument lists;
 *     arbitrarily many actor members;
 *     arbitrarily many handlers;
 *
 *
 * SCALABILITY
 *
 *     no grammar-level actor-count limit;
 *     no grammar-level message-count limit;
 *     no grammar-level handler-count limit;
 *     no grammar-level actor-nesting limit;
 *     no grammar-level node-count limit;
 *     no grammar-level worker-count limit;
 *     no grammar-level hardware limit.
 *
 *
 * DETERMINISM
 *
 *     identical token streams must produce identical parse structures.
 *
 *
 * CROSS-DOMAIN
 *
 *     classical actor;
 *     quantum actor;
 *     hybrid actor;
 *     HDL/hardware actor;
 *     distributed actor;
 *     AI/data actor;
 *     networking actor;
 *     security-aware actor.
 */


/* ============================================================================
 * 25. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * actors.g4 is complete when:
 *
 * [x] actor syntax has one canonical owner;
 * [x] canonical shared grammar rules are imported;
 * [x] no generic expression hierarchy is duplicated;
 * [x] actor spawn is distinguishable from generic async spawn;
 * [x] actor targets do not greedily consume message names;
 * [x] actor messages use identifier-as-data;
 * [x] lifecycle syntax has explicit ownership;
 * [x] supervision is source intent only;
 * [x] no mailbox implementation is encoded;
 * [x] no scheduler is encoded;
 * [x] no worker/thread/core count is encoded;
 * [x] no hardware topology is encoded;
 * [x] no physical device is selected;
 * [x] no resource limit is encoded;
 * [x] no actor-specific competing IR is created;
 * [x] quantum::ir remains the canonical quantum boundary;
 * [x] AST structure can preserve all actor source information;
 * [x] semantic validation remains downstream;
 * [x] runtime realization remains downstream;
 * [x] parsing is deterministic;
 * [x] syntax remains independent of available resources;
 * [x] Rust integration requires no unsafe code;
 * [x] public actor entry points are documented;
 * [x] negative/boundary/scalability/cross-domain tests are defined.
 *
 * ============================================================================
 */