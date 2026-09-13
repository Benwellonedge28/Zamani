/*
 * ============================================================================
 * Zamani Programming Language
 * Production Actor-Concurrency Grammar
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/actors.g4
 *
 * Role:
 *     Parser-domain grammar for actor-oriented concurrent computation.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *
 * ============================================================================
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * This grammar defines the SOURCE-SYNTAX boundary for actor-oriented
 * computation.
 *
 * It describes actor intent and structure without describing a particular
 * runtime implementation.
 *
 * An actor may therefore ultimately be realized by:
 *
 *     - a local event-driven executor;
 *     - a lightweight task;
 *     - a thread;
 *     - a process;
 *     - a distributed service;
 *     - a remote execution context;
 *     - an accelerator;
 *     - a heterogeneous execution context;
 *     - a future computational substrate.
 *
 * The grammar does not choose among those implementations.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - actor declaration syntax;
 *     - actor member syntax;
 *     - actor state-field syntax;
 *     - actor message-handler syntax;
 *     - actor constructor/spawn syntax;
 *     - actor reference syntax;
 *     - actor message-send syntax;
 *     - actor receive-handler syntax;
 *     - actor ask/request syntax;
 *     - actor lifecycle syntax;
 *     - actor supervision syntax at the SOURCE syntax boundary;
 *     - actor-oriented extension points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - patterns;
 *     - ordinary functions;
 *     - ordinary blocks;
 *     - modules;
 *     - lexical token spelling;
 *     - actor runtime objects;
 *     - actor mailboxes;
 *     - queues;
 *     - queue capacities;
 *     - executors;
 *     - threads;
 *     - processes;
 *     - worker pools;
 *     - CPU topology;
 *     - GPU topology;
 *     - QPU topology;
 *     - machine topology;
 *     - resource discovery;
 *     - resource allocation;
 *     - scheduling;
 *     - routing;
 *     - hardware;
 *     - quantum::ir;
 *     - classical IR;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime dispatch.
 *
 * Those concepts belong to their canonical owners.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Actor syntax expresses COMPUTATION and COMMUNICATION INTENT.
 *
 * It MUST NOT impose machine-level limits.
 *
 * This file therefore contains no:
 *
 *     MAX_ACTORS
 *     MAX_ACTOR_MAILBOXES
 *     MAX_MESSAGES
 *     MAX_HANDLERS
 *     MAX_CHILDREN
 *     MAX_SUPERVISORS
 *     MAX_NODES
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_QUEUE_DEPTH
 *
 * Actor count, message volume, mailbox capacity, execution parallelism and
 * placement are determined by downstream semantic, resource, scheduling,
 * deployment and runtime systems.
 *
 * ============================================================================
 * SEMANTIC PRINCIPLE
 * ============================================================================
 *
 * An actor is a logical concurrent computation boundary.
 *
 * The grammar does NOT imply:
 *
 *     actor == thread
 *     actor == process
 *     actor == machine
 *     actor == node
 *     mailbox == queue
 *     message == network packet
 *
 * These are possible realizations, not language semantics.
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
 *     canonical parser
 *          |
 *          +----------------------+
 *          |                      |
 *          v                      v
 *        Core                 Actors
 *          |                      |
 *          +----------+-----------+
 *                     |
 *                     v
 *                Frontend AST
 *                     |
 *                     v
 *       name/type/effect/capability/
 *       resource analysis
 *                     |
 *                     v
 *             canonical semantic IR
 *                     |
 *          +----------+----------+
 *          |          |          |
 *          v          v          v
 *      classical    quantum    distributed
 *                     |
 *                     v
 *        optimization / routing /
 *        scheduling / resilience
 *                     |
 *                     v
 *                   runtime
 *
 * `actors.g4` stops at syntax.
 *
 * ============================================================================
 * CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * This grammar relies on canonical parser-domain rules for:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     parameterList
 *     parameter
 *     argumentList
 *     genericParameters
 *     genericArguments
 *     returnType
 *     whereClause
 *     block
 *     blockExpression
 *     pattern
 *     attribute
 *     visibility
 *     modifier
 *
 * It MUST NOT redefine those rules.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Actor-specific reserved vocabulary belongs to the canonical lexer.
 *
 * The lexer integration MUST provide the actor tokens used below:
 *
 *     ACTOR
 *     RECEIVE
 *     SEND
 *     ASK
 *     SPAWN_ACTOR
 *     SUPERVISE
 *     STOP_ACTOR
 *     RESTART_ACTOR
 *     FORWARD
 *     SELF_ACTOR
 *
 * If any of these are not yet present in the canonical lexer, that is a
 * separate lexical integration task.
 *
 * This file MUST NOT define lexer rules.
 *
 * This separation is intentional:
 *
 *     lexer        -> spelling/token identity
 *     actors.g4    -> actor syntax
 *     AST          -> structural representation
 *     semantics    -> meaning/validity
 *     IR           -> target-independent computation
 *     runtime      -> realization
 *
 * ============================================================================
 * NO BACKEND ASSUMPTIONS
 * ============================================================================
 *
 * Actor syntax must remain valid when lowered to:
 *
 *     tiny embedded systems
 *     single-core systems
 *     multicore CPUs
 *     GPUs
 *     FPGAs
 *     ASIC-backed systems
 *     quantum/classical systems
 *     clusters
 *     distributed systems
 *     cloud environments
 *     future computational substrates
 *
 * The grammar must never require a specific actor runtime.
 *
 * ============================================================================
 */

parser grammar Actors;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. ACTOR DOMAIN ROOT
 * ============================================================================
 *
 * Stable public parser entry point for actor syntax.
 *
 * Parser composition layers should prefer `actorConstruct` rather than
 * depending directly on internal actor productions.
 * ============================================================================
 */

actorConstruct
    : actorDeclaration
    | actorSpawnExpression
    | actorSendExpression
    | actorAskExpression
    | actorLifecycleExpression
    | actorSupervisionConstruct
    ;


/*
 * ============================================================================
 * 2. ACTOR DECLARATION
 * ============================================================================
 *
 * Canonical conceptual form:
 *
 *     actor Counter {
 *         state: Int;
 *
 *         receive increment(value: Int) {
 *             ...
 *         }
 *     }
 *
 * The exact semantic meaning of fields and handlers is established after
 * parsing.
 *
 * Actor declarations are not classes merely because both have members.
 * The semantic layer must preserve actor isolation and message-driven
 * interaction semantics.
 * ============================================================================
 */

actorDeclaration
    : visibility?
      modifiers?
      ACTOR
      identifier
      genericParameters?
      actorInheritanceClause?
      whereClause?
      LBRACE
      actorMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 3. ACTOR INHERITANCE / CAPABILITY RELATION
 * ============================================================================
 *
 * Actor inheritance is deliberately expressed using canonical type expressions.
 *
 * This does not imply implementation inheritance, class inheritance, or a
 * specific runtime object model.
 * ============================================================================
 */

actorInheritanceClause
    : EXTENDS typeExpression
      (
          COMMA
          typeExpression
      )*
    ;


/*
 * ============================================================================
 * 4. ACTOR MEMBERS
 * ============================================================================
 *
 * Actor members are deliberately restricted to semantic actor members.
 *
 * A normal function declaration is not automatically an actor message handler.
 *
 * This distinction prevents ordinary object-oriented functions from silently
 * becoming concurrent message endpoints.
 * ============================================================================
 */

actorMember
    : attributes*
      visibility?
      actorMemberModifiers?
      actorStateField
    | attributes*
      visibility?
      actorMemberModifiers?
      actorHandler
    | attributes*
      visibility?
      actorMemberModifiers?
      actorLifecycleHandler
    ;


/*
 * ============================================================================
 * 5. ACTOR MEMBER MODIFIERS
 * ============================================================================
 *
 * Actor-specific modifiers are expressed through already-established lexical
 * vocabulary where possible.
 *
 * The grammar does not create a second modifier system.
 * ============================================================================
 */

actorMemberModifiers
    : modifier+
    ;


/*
 * ============================================================================
 * 6. ACTOR STATE FIELD
 * ============================================================================
 *
 * Actor state is private to the actor's semantic isolation boundary.
 *
 * The grammar does not determine:
 *
 *     - memory location;
 *     - allocation strategy;
 *     - cache placement;
 *     - address;
 *     - NUMA node;
 *     - device memory;
 *     - serialization format.
 * ============================================================================
 */

actorStateField
    : identifier
      COLON
      typeExpression
      (
          ASSIGN
          expression
      )?
      SEMI?
    ;


/*
 * ============================================================================
 * 7. ACTOR HANDLER
 * ============================================================================
 *
 * A handler is a message endpoint.
 *
 * Example:
 *
 *     receive increment(value: Int) {
 *         ...
 *     }
 *
 * Message names are identifiers rather than an enumerated global keyword set.
 *
 * The semantic layer determines:
 *
 *     - message identity;
 *     - accepted payload;
 *     - effects;
 *     - state transitions;
 *     - reply behavior;
 *     - failure behavior.
 * ============================================================================
 */

actorHandler
    : RECEIVE
      identifier
      genericParameters?
      LPAREN
      parameterList?
      RPAREN
      returnType?
      whereClause?
      blockExpression
    ;


/*
 * ============================================================================
 * 8. ACTOR LIFECYCLE HANDLERS
 * ============================================================================
 *
 * Lifecycle handlers are explicitly separate from ordinary message handlers.
 *
 * This prevents startup/shutdown semantics from being confused with arbitrary
 * application messages.
 * ============================================================================
 */

actorLifecycleHandler
    : actorInitHandler
    | actorStartHandler
    | actorStopHandler
    ;


/*
 * ============================================================================
 * 9. INITIALIZATION HANDLER
 * ============================================================================
 */

actorInitHandler
    : actorLifecycleKeyword
      INIT
      LPAREN
      parameterList?
      RPAREN
      blockExpression
    ;


/*
 * ============================================================================
 * 10. START HANDLER
 * ============================================================================
 */

actorStartHandler
    : actorLifecycleKeyword
      START
      LPAREN
      RPAREN
      blockExpression
    ;


/*
 * ============================================================================
 * 11. STOP HANDLER
 * ============================================================================
 */

actorStopHandler
    : actorLifecycleKeyword
      STOP
      LPAREN
      RPAREN
      blockExpression
    ;


/*
 * ============================================================================
 * 12. LIFECYCLE KEYWORD
 * ============================================================================
 *
 * This rule exists as an explicit parser integration boundary.
 * ============================================================================
 */

actorLifecycleKeyword
    : RECEIVE
    ;


/*
 * ============================================================================
 * 13. ACTOR REFERENCE
 * ============================================================================
 *
 * Actor references are semantic values represented by ordinary expressions.
 *
 * This rule does not introduce a finite actor-handle type.
 * ============================================================================
 */

actorReference
    : expression
    ;


/*
 * ============================================================================
 * 14. ACTOR SPAWN
 * ============================================================================
 *
 * Conceptual form:
 *
 *     spawn_actor Counter(arguments...)
 *
 * Actor construction is a logical operation.
 *
 * It does not select:
 *
 *     CPU
 *     core
 *     thread
 *     process
 *     node
 *     device
 *     network endpoint
 *
 * Those choices belong downstream.
 * ============================================================================
 */

actorSpawnExpression
    : SPAWN_ACTOR
      qualifiedName
      genericArguments?
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 15. ACTOR SPAWN STATEMENT
 * ============================================================================
 */

actorSpawnStatement
    : actorSpawnExpression
      SEMI?
    ;


/*
 * ============================================================================
 * 16. ACTOR MESSAGE SEND
 * ============================================================================
 *
 * Conceptual forms may be represented as:
 *
 *     send target(message(...))
 *
 * or through a target/message/value structure defined by the canonical
 * language specification.
 *
 * The actor target is an expression, allowing:
 *
 *     local actor
 *     actor reference
 *     remote actor reference
 *     actor capability
 *     computed actor endpoint
 *
 * without changing the grammar.
 * ============================================================================
 */

actorSendExpression
    : SEND
      actorReference
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 17. NAMED MESSAGE SEND
 * ============================================================================
 *
 * Canonical conceptual form:
 *
 *     send target.message(arguments...)
 *
 * The message name is a normal identifier.
 * ============================================================================
 */

actorNamedSendExpression
    : SEND
      actorReference
      DOT
      identifier
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 18. MESSAGE SEND STATEMENT
 * ============================================================================
 */

actorSendStatement
    : actorNamedSendExpression
      SEMI?
    | actorSendExpression
      SEMI?
    ;


/*
 * ============================================================================
 * 19. REQUEST / ASK
 * ============================================================================
 *
 * Request-style interaction returns a semantic asynchronous result.
 *
 * The grammar does not introduce a runtime-specific Promise/Future type.
 *
 * The returned value is interpreted by the type/effect system.
 * ============================================================================
 */

actorAskExpression
    : ASK
      actorReference
      DOT
      identifier
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 20. ACTOR RECEIVE OPERATION
 * ============================================================================
 *
 * A receive expression is deliberately distinct from a receive handler.
 *
 * Handler:
 *
 *     receive message(...) { ... }
 *
 * Operation:
 *
 *     receive target.message(...)
 *
 * The semantic layer determines whether receiving is:
 *
 *     blocking
 *     non-blocking
 *     asynchronous
 *     selective
 *     remotely mediated
 *
 * ============================================================================
 */

actorReceiveExpression
    : RECEIVE
      actorReference
      DOT
      identifier
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 21. ACTOR FORWARDING
 * ============================================================================
 *
 * Forwarding delegates a message without requiring the grammar to know whether
 * the destination is local or remote.
 * ============================================================================
 */

actorForwardExpression
    : FORWARD
      actorReference
      DOT
      identifier
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 22. ACTOR LIFECYCLE CONTROL
 * ============================================================================
 *
 * Lifecycle operations are logical requests.
 *
 * They do not guarantee physical termination, process destruction, thread
 * cancellation, or machine-level shutdown.
 * ============================================================================
 */

actorLifecycleExpression
    : actorStopExpression
    | actorRestartExpression
    ;


actorStopExpression
    : STOP_ACTOR
      actorReference
    ;


actorRestartExpression
    : RESTART_ACTOR
      actorReference
    ;


/*
 * ============================================================================
 * 23. SUPERVISION
 * ============================================================================
 *
 * Supervision describes a logical failure-management relationship.
 *
 * It does not prescribe a particular recovery implementation.
 * ============================================================================
 */

actorSupervisionConstruct
    : SUPERVISE
      actorReference
      actorSupervisionBody
    ;


actorSupervisionBody
    : blockExpression
    ;


/*
 * ============================================================================
 * 24. SUPERVISED ACTOR DECLARATION
 * ============================================================================
 *
 * Explicit syntax boundary for actor supervision.
 * ============================================================================
 */

supervisedActorDeclaration
    : SUPERVISE
      actorDeclaration
    ;


/*
 * ============================================================================
 * 25. ACTOR MESSAGE TARGET
 * ============================================================================
 *
 * Target identity remains an expression so actor addressing can evolve without
 * encoding physical deployment topology.
 * ============================================================================
 */

actorMessageTarget
    : actorReference
    ;


/*
 * ============================================================================
 * 26. ACTOR MESSAGE NAME
 * ============================================================================
 */

actorMessageName
    : identifier
    ;


/*
 * ============================================================================
 * 27. ACTOR MESSAGE INVOCATION
 * ============================================================================
 *
 * This production is intentionally target-independent.
 * ============================================================================
 */

actorMessageInvocation
    : actorMessageTarget
      DOT
      actorMessageName
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 28. ACTOR MESSAGE PAYLOAD
 * ============================================================================
 *
 * Payloads reuse the ordinary argument model.
 *
 * No maximum payload count or payload size is encoded.
 * ============================================================================
 */

actorMessagePayload
    : argumentList
    ;


/*
 * ============================================================================
 * 29. ACTOR MESSAGE PATTERN
 * ============================================================================
 *
 * Message handlers may later integrate with the canonical pattern system.
 *
 * This rule deliberately delegates pattern ownership.
 * ============================================================================
 */

actorMessagePattern
    : pattern
    ;


/*
 * ============================================================================
 * 30. ACTOR HANDLER SIGNATURE
 * ============================================================================
 *
 * Signature-only representation for interfaces/traits/contracts.
 * ============================================================================
 */

actorHandlerSignature
    : RECEIVE
      identifier
      genericParameters?
      LPAREN
      parameterList?
      RPAREN
      returnType?
      whereClause?
      SEMI?
    ;


/*
 * ============================================================================
 * 31. ACTOR CONSTRUCTOR ARGUMENTS
 * ============================================================================
 */

actorConstructorArguments
    : argumentList
    ;


/*
 * ============================================================================
 * 32. ACTOR TYPE REFERENCE
 * ============================================================================
 *
 * Actor types are represented by canonical type expressions.
 *
 * A dedicated `Actor<T>` type must not be invented here.
 * ============================================================================
 */

actorTypeReference
    : typeExpression
    ;


/*
 * ============================================================================
 * 33. ACTOR GENERIC REFERENCE
 * ============================================================================
 */

actorGenericReference
    : qualifiedName
      genericArguments?
    ;


/*
 * ============================================================================
 * 34. ACTOR EXTENSION CONSTRUCT
 * ============================================================================
 *
 * Future actor models must be extensible without adding an exhaustive list of
 * runtime implementations.
 *
 * The extension namespace is represented by an ordinary qualified name.
 * ============================================================================
 */

actorExtensionConstruct
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 35. ACTOR EXPRESSION
 * ============================================================================
 *
 * Stable expression-level integration point.
 * ============================================================================
 */

actorExpression
    : actorSpawnExpression
    | actorSendExpression
    | actorNamedSendExpression
    | actorAskExpression
    | actorReceiveExpression
    | actorForwardExpression
    | actorLifecycleExpression
    | actorSupervisionConstruct
    ;


/*
 * ============================================================================
 * 36. ACTOR STATEMENT
 * ============================================================================
 *
 * Stable statement-level integration point.
 * ============================================================================
 */

actorStatement
    : actorSpawnStatement
    | actorSendStatement
    | actorLifecycleExpression SEMI?
    | actorSupervisionConstruct
    ;


/*
 * ============================================================================
 * 37. ACTOR MEMBER ROOT
 * ============================================================================
 */

actorMemberRoot
    : actorMember
    ;


/*
 * ============================================================================
 * 38. ACTOR DECLARATION ROOT
 * ============================================================================
 */

actorRoot
    : actorDeclaration
    ;


/*
 * ============================================================================
 * 39. ACTOR SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Everything below this boundary belongs outside the grammar:
 *
 *     actor identity
 *     actor lifecycle semantics
 *     isolation semantics
 *     mailbox semantics
 *     ordering guarantees
 *     delivery guarantees
 *     failure semantics
 *     supervision strategy
 *     resource requirements
 *     placement
 *     routing
 *     scheduling
 *     persistence
 *     replication
 *     serialization
 *     security
 *     distributed execution
 *
 * The parser records structure only.
 * ============================================================================
 */

actorSemanticBoundary
    : actorConstruct
    ;


/*
 * ============================================================================
 * 40. RESOURCE-NEUTRAL ACTOR BOUNDARY
 * ============================================================================
 *
 * Actor syntax does not encode physical resources.
 *
 * This prevents source-level coupling such as:
 *
 *     actor Counter on cpu0
 *     actor Counter on node7
 *     actor Counter on gpu2
 *     actor Counter on qpu0
 *
 * from becoming mandatory language semantics.
 *
 * If placement is semantically meaningful, it must be represented by the
 * canonical resource/target/capability system.
 * ============================================================================
 */

actorResourceNeutralBoundary
    : actorConstruct
    ;


/*
 * ============================================================================
 * 41. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Actors may orchestrate quantum/classical computation.
 *
 * The actor grammar does not define quantum operations.
 *
 * A handler may contain ordinary Zamani expressions that eventually lower into
 * quantum::ir through the normal semantic pipeline.
 *
 * Therefore:
 *
 *     actor
 *       |
 *       v
 *     handler
 *       |
 *       v
 *     expression
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *
 * `actors.g4` never becomes a second quantum IR.
 * ============================================================================
 */

actorQuantumIntegrationBoundary
    : actorDeclaration
    ;


/*
 * ============================================================================
 * 42. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Actor references may represent local or distributed identities.
 *
 * This grammar deliberately does not encode:
 *
 *     node IDs
 *     IP addresses
 *     ports
 *     machine IDs
 *     cluster sizes
 *     network topology
 *
 * Those belong to networking/distributed/resource/target semantics.
 * ============================================================================
 */

actorDistributedIntegrationBoundary
    : actorMessageTarget
    ;


/*
 * ============================================================================
 * 43. HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware realization is downstream.
 *
 * An actor may ultimately be implemented using CPU, GPU, FPGA, ASIC,
 * accelerator, quantum/classical or other resources without changing its
 * source-level actor semantics.
 * ============================================================================
 */

actorHardwareIntegrationBoundary
    : actorConstruct
    ;


/*
 * ============================================================================
 * 44. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Actor resource requirements, if any, are semantic resource requirements.
 *
 * This grammar does not define resource quantities.
 * ============================================================================
 */

actorResourceIntegrationBoundary
    : actorConstruct
    ;


/*
 * ============================================================================
 * 45. EFFECT INTEGRATION
 * ============================================================================
 *
 * Actor operations may introduce effects.
 *
 * Effect ownership remains in the canonical effect system.
 * ============================================================================
 */

actorEffectIntegrationBoundary
    : actorHandler
    ;


/*
 * ============================================================================
 * 46. CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Actor operations are one concurrency model.
 *
 * They must coexist with:
 *
 *     tasks
 *     futures
 *     channels
 *     parallel execution
 *     structured concurrency
 *
 * without redefining those domains.
 * ============================================================================
 */

actorConcurrencyBoundary
    : actorConstruct
    ;


/*
 * ============================================================================
 * 47. DETERMINISM BOUNDARY
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * Actor runtime ordering is NOT necessarily deterministic.
 *
 * These are deliberately different properties.
 * ============================================================================
 */

actorDeterminismBoundary
    : actorConstruct
    ;


/*
 * ============================================================================
 * 48. EXTENSIBILITY
 * ============================================================================
 *
 * Future actor features must be introduced through coordinated:
 *
 *     lexer
 *     parser
 *     AST
 *     semantic
 *     diagnostics
 *     IR
 *     runtime
 *     documentation
 *     tests
 *
 * changes.
 *
 * This grammar must not silently reinterpret unknown actor syntax.
 * ============================================================================
 */

actorExtensionBoundary
    : actorExtensionConstruct
    ;


/*
 * ============================================================================
 * 49. PUBLIC EXPRESSION ROOT
 * ============================================================================
 */

actorExpressionRoot
    : actorExpression
    ;


/*
 * ============================================================================
 * 50. PUBLIC STATEMENT ROOT
 * ============================================================================
 */

actorStatementRoot
    : actorStatement
    ;


/*
 * ============================================================================
 * 51. PUBLIC DECLARATION ROOT
 * ============================================================================
 */

actorDeclarationRoot
    : actorDeclaration
    ;


/*
 * ============================================================================
 * 52. COMPLETENESS BOUNDARY
 * ============================================================================
 *
 * This is the final stable actor-domain parser entry point.
 * ============================================================================
 */

actor
    : actorConstruct
    ;