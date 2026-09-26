/*
 * ============================================================================
 * Zamani Universal Programming Language
 * Production Distributed-Actor Grammar
 * ============================================================================
 *
 * File:
 *     grammar/distributed/actors.g4
 *
 * Grammar:
 *     DistributedActors
 *
 * Status:
 *     PRODUCTION SOURCE-GRAMMAR CONTRACT
 *
 * Language baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL DISTRIBUTED-ACTOR SYNTAX.
 *
 * A distributed actor is a logical actor whose source-level intent permits
 * distributed realization.
 *
 * A distributed actor may ultimately be realized as:
 *
 *     - a local actor;
 *     - an asynchronous task;
 *     - a process;
 *     - a service;
 *     - a worker;
 *     - a remote actor;
 *     - a replicated actor;
 *     - a migrated actor;
 *     - an actor spanning heterogeneous resources;
 *     - a quantum/classical orchestration actor;
 *     - an accelerator-backed actor;
 *     - a future computational substrate.
 *
 * The grammar describes the PROGRAMMER'S SEMANTIC INTENT.
 *
 * It does NOT select:
 *
 *     - a physical machine;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - a QPU;
 *     - a physical node;
 *     - a network address;
 *     - a transport;
 *     - a scheduler;
 *     - a placement algorithm;
 *     - a replication algorithm;
 *     - a consensus algorithm;
 *     - a runtime executor.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 *     SOURCE
 *        |
 *        v
 *     ZamaniLexer
 *        |
 *        v
 *     DistributedActors
 *        |
 *        v
 *     domain-neutral frontend AST
 *        |
 *        v
 *     semantic analysis
 *        |
 *        +--> actor analysis
 *        +--> distributed analysis
 *        +--> type analysis
 *        +--> effect analysis
 *        +--> capability analysis
 *        +--> resource analysis
 *        +--> security analysis
 *        |
 *        v
 *     canonical semantic representation
 *        |
 *        +--> classical representation
 *        +--> quantum::ir
 *        +--> hardware/HDL representation
 *        +--> distributed execution metadata
 *        |
 *        v
 *     optimization
 *        |
 *        v
 *     placement / routing / scheduling
 *        |
 *        v
 *     resilience / recovery / deployment
 *        |
 *        v
 *     runtime
 *
 * This grammar never constructs or modifies an IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     distributedActorConstruct
 *     distributedActorDeclaration
 *     distributedActorMember
 *     distributedActorContract
 *     distributedActorRequirement
 *     distributedActorCapability
 *     distributedActorPlacementIntent
 *     distributedActorReplicationIntent
 *     distributedActorLifecycleIntent
 *     distributedActorCommunicationIntent
 *     distributedActorSupervisionIntent
 *     distributedActorReference
 *     distributedActorInvocation
 *     distributedActorSpawn
 *     distributedActorSend
 *     distributedActorAsk
 *     distributedActorForward
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers;
 *     qualified names;
 *     expressions;
 *     types;
 *     ordinary actor syntax;
 *     ordinary concurrency syntax;
 *     generic blocks;
 *     lexical tokens;
 *     networking protocols;
 *     physical topology;
 *     physical placement;
 *     resource allocation;
 *     scheduling;
 *     routing;
 *     deployment implementation;
 *     consensus algorithms;
 *     replication algorithms;
 *     QEC;
 *     ZQN;
 *     quantum::ir;
 *     classical IR;
 *     HDL IR;
 *     hardware discovery;
 *     runtime execution.
 *
 * ============================================================================
 * RELATIONSHIP WITH CONCURRENCY ACTORS
 * ============================================================================
 *
 * General actor syntax is owned by:
 *
 *     grammar/concurrency/actors.g4
 *
 * That grammar defines the general actor model:
 *
 *     actor declaration
 *     actor state
 *     actor handlers
 *     actor spawn
 *     actor send
 *     actor ask
 *     actor forwarding
 *     actor lifecycle
 *     actor supervision
 *
 * This grammar MUST NOT copy those rules into a second implementation.
 *
 * Instead, distributed actors add DISTRIBUTED INTENT around the canonical
 * actor model.
 *
 * The semantic distinction is:
 *
 *     actor
 *         =
 *     logical concurrent computation boundary
 *
 *     distributed actor
 *         =
 *     actor whose source contract permits or requires distributed realization
 *
 * A distributed actor is NOT inherently:
 *
 *     one node;
 *     one process;
 *     one machine;
 *     one thread;
 *     one network endpoint.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Distributed actor policies use qualified semantic names wherever possible.
 *
 * Examples include:
 *
 *     distributed::placement
 *     distributed::replication
 *     distributed::migration
 *     distributed::consistency
 *     distributed::fault_tolerance
 *     distributed::availability
 *     distributed::coordination
 *     distributed::supervision
 *
 * The grammar does not create one parser keyword for every possible future
 * policy.
 *
 * Semantic analysis determines whether a qualified construct is:
 *
 *     stable;
 *     experimental;
 *     proposed;
 *     deprecated;
 *     vendor-specific;
 *     dialect-specific;
 *     unknown.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Distributed actor syntax is designed for:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Anywhere
 *     Forever
 *
 * The same source-level actor may be realized on:
 *
 *     one execution context;
 *     multiple execution contexts;
 *     embedded hardware;
 *     CPUs;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     QPUs;
 *     clusters;
 *     HPC systems;
 *     cloud systems;
 *     heterogeneous systems;
 *     future computational architectures.
 *
 * The grammar therefore contains no artificial hardware ceiling.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains NO universal limits for:
 *
 *     actors;
 *     processes;
 *     nodes;
 *     workers;
 *     handlers;
 *     children;
 *     supervisors;
 *     messages;
 *     mailbox entries;
 *     replicas;
 *     partitions;
 *     regions;
 *     devices;
 *     threads;
 *     cores;
 *     CPUs;
 *     GPUs;
 *     FPGAs;
 *     QPUs;
 *     memory;
 *     storage;
 *     network capacity;
 *     topology size.
 *
 * There is deliberately no:
 *
 *     MAX_ACTORS
 *     MAX_PROCESSES
 *     MAX_NODES
 *     MAX_WORKERS
 *     MAX_MESSAGES
 *     MAX_REPLICAS
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_MAILBOX
 *
 * Repetition is represented structurally with `*` and `+`.
 *
 * Practical limitations belong to:
 *
 *     semantic resource analysis;
 *     compiler resource policy;
 *     deployment policy;
 *     scheduler;
 *     runtime;
 *     actual target resources.
 *
 * ============================================================================
 * HARD-CODING POLICY
 * ============================================================================
 *
 * Numeric literals remain ordinary source-level values.
 *
 * For example:
 *
 *     retry_count = 3
 *
 * is program data.
 *
 * It MUST NOT be interpreted as:
 *
 *     maximum three actors;
 *     maximum three replicas;
 *     maximum three nodes.
 *
 * The grammar must never encode:
 *
 *     actor0;
 *     node0;
 *     CPU0;
 *     GPU0;
 *     QPU0;
 *
 * as universal physical identities.
 *
 * A programmer-defined logical name such as `worker_a` is allowed.
 *
 * Logical identity is not physical identity.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Distributed actors may carry source-level requirements or capabilities.
 *
 * These express semantic intent.
 *
 * They do NOT allocate resources.
 *
 * Conceptually:
 *
 *     requirement
 *         !=
 *     capability
 *         !=
 *     preference
 *         !=
 *     physical placement
 *
 * Examples of semantic intent:
 *
 *     requires capability("distributed.actor");
 *     requires capability("quantum.measurement");
 *     requires capability("tensor.compute");
 *
 * Resource realization remains downstream.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Declaring or spawning a distributed actor does not grant authority.
 *
 * Authorization remains a semantic/runtime responsibility.
 *
 * The grammar does not provide:
 *
 *     filesystem access;
 *     network access;
 *     secret access;
 *     hardware access;
 *     arbitrary execution;
 *     privilege escalation.
 *
 * Security analysis must remain part of the canonical compilation pipeline.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A distributed actor may orchestrate quantum computation.
 *
 * For example, an actor may semantically:
 *
 *     request a quantum operation;
 *     receive a measurement;
 *     coordinate classical control;
 *     exchange quantum-related metadata;
 *     participate in distributed quantum execution.
 *
 * This grammar does NOT define:
 *
 *     gates;
 *     QubitId;
 *     physical qubits;
 *     pulse schedules;
 *     calibration;
 *     QEC algorithms;
 *     noise models;
 *     ZQN;
 *     quantum routing.
 *
 * Quantum semantics continue through:
 *
 *     quantum::ir
 *
 * as the repository's canonical quantum IR boundary.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Actor computation may contain or coordinate:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     AI computation;
 *     data processing;
 *     HDL/hardware intent;
 *     accelerator computation.
 *
 * This grammar does not duplicate those domain grammars.
 *
 * Cross-domain composition occurs through the canonical parser and semantic
 * model.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Distributed actor communication describes semantic communication intent.
 *
 * It does not select:
 *
 *     TCP;
 *     UDP;
 *     QUIC;
 *     MPI;
 *     RDMA;
 *     InfiniBand;
 *     a cloud transport;
 *     a vendor transport.
 *
 * Network realization belongs to:
 *
 *     grammar/networking/
 *
 * and downstream networking/runtime components.
 *
 * ============================================================================
 * REPLICATION
 * ============================================================================
 *
 * Distributed actors may express replication intent.
 *
 * The grammar does not create replicas.
 *
 * It does not determine:
 *
 *     replica count limits;
 *     replica placement;
 *     consensus;
 *     consistency implementation;
 *     failover algorithm.
 *
 * Those belong to:
 *
 *     grammar/distributed/replication.g4
 *     grammar/distributed/consistency.g4
 *     semantic analysis
 *     runtime
 *
 * where applicable.
 *
 * ============================================================================
 * PLACEMENT
 * ============================================================================
 *
 * Placement syntax expresses logical constraints or preferences only.
 *
 * It must never silently mean:
 *
 *     physical machine N;
 *     physical CPU N;
 *     physical GPU N;
 *     physical QPU N;
 *     physical node N.
 *
 * Placement algorithms remain downstream.
 *
 * ============================================================================
 * LIFECYCLE
 * ============================================================================
 *
 * Lifecycle syntax expresses logical intent such as:
 *
 *     start;
 *     stop;
 *     restart;
 *     recover;
 *     migrate;
 *
 * It does not directly invoke a runtime API.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     supplied token stream;
 *     grammar version;
 *     imported grammar definitions.
 *
 * It does NOT depend on:
 *
 *     hardware;
 *     available nodes;
 *     network state;
 *     wall clock;
 *     randomness;
 *     environment;
 *     runtime state;
 *     scheduler state.
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * AST construction must preserve:
 *
 *     - source order;
 *     - actor name;
 *     - actor contract clauses;
 *     - member order;
 *     - handler references;
 *     - expression structure;
 *     - argument order;
 *     - qualified-name segment order;
 *     - nested block structure;
 *     - lifecycle intent;
 *     - source spans.
 *
 * Semantic normalization occurs after parsing.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every public rule in this grammar maps to the repository's domain-neutral
 * frontend representation.
 *
 * The parser must NOT require a runtime-specific AST such as:
 *
 *     ActorHandle;
 *     Mailbox;
 *     WorkerHandle;
 *     ProcessHandle;
 *     PhysicalNode;
 *     DeviceHandle;
 *     NetworkSocket.
 *
 * Those are downstream implementation concepts.
 *
 * Conceptual mapping:
 *
 *     distributedActorDeclaration
 *         ->
 *     domain-neutral declaration / operation representation
 *         ->
 *     distributed semantic model
 *         ->
 *     canonical IR
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO distributed-actor IR.
 *
 * It must not create:
 *
 *     DistributedActorIR;
 *     ActorRuntimeIR;
 *     ActorNetworkIR;
 *
 * as competing intermediate representations.
 *
 * Distributed actor information must be preserved as semantic metadata in
 * the repository's canonical semantic/IR architecture.
 *
 * If an actor contains quantum computation:
 *
 *     actor semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * remains the canonical quantum boundary.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * `grammar/distributed/distributed.g4` is the distributed-domain composition
 * grammar.
 *
 * It should import this grammar and consume:
 *
 *     distributedActorConstruct
 *
 * rather than maintaining an independent `distributedActor` implementation.
 *
 * The existing:
 *
 *     grammar/concurrency/actors.g4
 *
 * remains the authority for ordinary actor syntax.
 *
 * This file adds only distributed specialization/intent.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It:
 *
 *     - reuses ZamaniLexer;
 *     - imports reusable parser grammars;
 *     - contains no lexer rules;
 *     - contains no embedded Rust;
 *     - contains no semantic predicates;
 *     - contains no parser actions;
 *     - contains no runtime callbacks;
 *     - contains no filesystem access;
 *     - contains no network access;
 *     - contains no hardware access.
 *
 * ============================================================================
 */

parser grammar DistributedActors;

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


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the only public distributed-actor composition boundary.
 *
 * Other distributed grammar components should consume this rule rather than
 * reproducing distributed actor syntax.
 */
distributedActorConstruct
    : distributedActorDeclaration
    | distributedActorSpawn
    | distributedActorInvocation
    | distributedActorSend
    | distributedActorAsk
    | distributedActorForward
    | distributedActorLifecycleIntent
    ;


/*
 * ============================================================================
 * 2. DISTRIBUTED ACTOR DECLARATION
 * ============================================================================
 *
 * Canonical logical form:
 *
 *     distributed actor Counter {
 *         ...
 *     }
 *
 * The actor body remains an ordinary Zamani block/member structure.
 *
 * Distributed policy is expressed through optional semantic clauses.
 */
distributedActorDeclaration
    : distributedActorDeclarationPrefix*
      DISTRIBUTED
      ACTOR
      identifier
      distributedActorContract*
      actorInheritanceClause?
      distributedActorBody
    ;


/*
 * Declaration prefixes are canonical language constructs.
 */
distributedActorDeclarationPrefix
    : attribute
    | visibilityModifier
    | modifier
    ;


/*
 * Distributed actor body.
 *
 * Actor members are represented through the same structural member model used
 * by the canonical actor subsystem where possible.
 *
 * We intentionally avoid importing `Actors` here because doing so would make
 * the distributed actor grammar depend on the general actor grammar's private
 * rule surface and would encourage duplicate actor composition boundaries.
 *
 * The body is therefore structurally explicit while actor semantics remain
 * downstream.
 */
distributedActorBody
    : LBRACE
      distributedActorMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 3. ACTOR INHERITANCE
 * ============================================================================
 *
 * Kept structurally compatible with the general actor model.
 *
 * Semantic validation determines whether the referenced type is a valid actor
 * contract.
 */
actorInheritanceClause
    : EXTENDS
      typeExpression
      (
          COMMA
          typeExpression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 4. ACTOR MEMBERS
 * ============================================================================
 */

distributedActorMember
    : distributedActorMemberPrefix*
      distributedActorStateField
    | distributedActorMemberPrefix*
      distributedActorHandler
    | distributedActorMemberPrefix*
      distributedActorLifecycleHandler
    | distributedActorMemberPrefix*
      distributedActorContract
    ;


distributedActorMemberPrefix
    : attribute
    | visibilityModifier
    | modifier
    ;


/*
 * ============================================================================
 * 5. ACTOR STATE
 * ============================================================================
 *
 * State is logical actor-owned state.
 *
 * No physical memory location is specified.
 */
distributedActorStateField
    : identifier
      COLON
      typeExpression
      (
          ASSIGN
          expression
      )?
      SEMI
    ;


/*
 * ============================================================================
 * 6. MESSAGE HANDLERS
 * ============================================================================
 *
 * Message names remain ordinary identifiers.
 *
 * This avoids a closed vocabulary of messages.
 */
distributedActorHandler
    : RECEIVE
      identifier
      LPAREN
      parameterList?
      RPAREN
      distributedActorHandlerReturnType?
      blockExpression
    ;


distributedActorHandlerReturnType
    : THIN_ARROW
      typeExpression
    ;


/*
 * Lifecycle handlers are logical lifecycle definitions.
 */
distributedActorLifecycleHandler
    : START_ACTOR
      blockExpression
    | STOP_ACTOR
      blockExpression
    | RESTART_ACTOR
      blockExpression
    ;


/*
 * ============================================================================
 * 7. DISTRIBUTED ACTOR CONTRACT
 * ============================================================================
 *
 * Contract clauses describe semantic distributed intent.
 *
 * They do not perform runtime allocation.
 */
distributedActorContract
    : distributedActorRequirement
    | distributedActorCapability
    | distributedActorPlacementIntent
    | distributedActorReplicationIntent
    | distributedActorLifecycleIntent
    | distributedActorCommunicationIntent
    | distributedActorSupervisionIntent
    | distributedActorPolicyReference
    ;


/*
 * ============================================================================
 * 8. REQUIREMENTS
 * ============================================================================
 *
 * Requirements are expressions evaluated by semantic/resource analysis.
 *
 * This grammar deliberately does not enumerate resource types.
 */
distributedActorRequirement
    : REQUIRES
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 9. CAPABILITIES
 * ============================================================================
 *
 * Capability expressions are semantic requirements.
 */
distributedActorCapability
    : CAPABILITY
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 10. PLACEMENT INTENT
 * ============================================================================
 *
 * Placement remains abstract.
 *
 * The expression can describe locality, affinity, proximity, topology or
 * another semantic placement property without hard-coding a target.
 */
distributedActorPlacementIntent
    : PLACEMENT
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 11. REPLICATION INTENT
 * ============================================================================
 *
 * Replication is expressed semantically.
 *
 * The expression is not interpreted by the parser as a finite replica count.
 */
distributedActorReplicationIntent
    : REPLICATION
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 12. COMMUNICATION INTENT
 * ============================================================================
 *
 * Communication policy remains abstract.
 */
distributedActorCommunicationIntent
    : COMMUNICATION
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 13. SUPERVISION INTENT
 * ============================================================================
 *
 * Supervision is semantic policy, not runtime implementation.
 */
distributedActorSupervisionIntent
    : SUPERVISE
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 14. GENERIC POLICY REFERENCE
 * ============================================================================
 *
 * Future distributed actor policies remain syntactically representable
 * without requiring a new keyword for every policy.
 *
 * Example:
 *
 *     distributed::consistency eventual;
 *
 *     distributed::fault_tolerance resilient;
 *
 * The semantic layer determines whether the referenced policy is defined.
 */
distributedActorPolicyReference
    : qualifiedName
      expression?
      SEMI
    ;


/*
 * ============================================================================
 * 15. ACTOR REFERENCE
 * ============================================================================
 *
 * Logical references are qualified names.
 *
 * They do not imply physical addresses.
 */
distributedActorReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 16. ACTOR SPAWN
 * ============================================================================
 *
 * Canonical form:
 *
 *     spawn distributed actor Counter(...)
 *
 * The resulting semantic entity is logical.
 *
 * Runtime realization remains downstream.
 */
distributedActorSpawn
    : SPAWN
      DISTRIBUTED
      ACTOR
      qualifiedName
      LPAREN
      distributedActorArguments?
      RPAREN
    ;


distributedActorArguments
    : argumentList
    ;


distributedActorSpawnStatement
    : distributedActorSpawn
      SEMI
    ;


/*
 * ============================================================================
 * 17. INVOCATION
 * ============================================================================
 *
 * Logical distributed actor invocation.
 *
 * Example:
 *
 *     distributed actor_ref.message(value)
 *
 * The exact meaning of the member operation is semantic.
 */
distributedActorInvocation
    : distributedActorReference
      DOT
      identifier
      LPAREN
      optionalExpressionList
      RPAREN
    ;


distributedActorInvocationStatement
    : distributedActorInvocation
      SEMI
    ;


/*
 * ============================================================================
 * 18. SEND
 * ============================================================================
 *
 * Explicit asynchronous message intent.
 *
 * The transport remains outside the grammar.
 */
distributedActorSend
    : SEND
      distributedActorTarget
      DOT
      identifier
      LPAREN
      optionalExpressionList
      RPAREN
    ;


distributedActorSendStatement
    : distributedActorSend
      SEMI
    ;


/*
 * ============================================================================
 * 19. ASK / REQUEST
 * ============================================================================
 *
 * Request-response interaction.
 *
 * Result typing belongs to semantic analysis.
 */
distributedActorAsk
    : ASK
      distributedActorTarget
      DOT
      identifier
      LPAREN
      optionalExpressionList
      RPAREN
    ;


distributedActorAskStatement
    : distributedActorAsk
      SEMI
    ;


/*
 * ============================================================================
 * 20. FORWARDING
 * ============================================================================
 *
 * Forwarding remains logical communication intent.
 */
distributedActorForward
    : FORWARD
      distributedActorTarget
      DOT
      identifier
      LPAREN
      optionalExpressionList
      RPAREN
    ;


distributedActorForwardStatement
    : distributedActorForward
      SEMI
    ;


/*
 * ============================================================================
 * 21. ACTOR TARGET
 * ============================================================================
 *
 * A target is represented as a normal expression.
 *
 * This permits:
 *
 *     local actor reference;
 *     logical group;
 *     actor collection;
 *     dynamically resolved actor;
 *     future distributed actor abstraction.
 *
 * Physical addressing is not required.
 */
distributedActorTarget
    : expression
    ;


/*
 * ============================================================================
 * 22. LIFECYCLE INTENT
 * ============================================================================
 *
 * Lifecycle intent is open-ended through a qualified operation.
 *
 * Examples:
 *
 *     start;
 *     stop;
 *     restart;
 *     recover;
 *     migrate;
 *
 * The semantic layer determines legality.
 */
distributedActorLifecycleIntent
    : LIFECYCLE
      qualifiedName
      (
          LPAREN
          optionalExpressionList
          RPAREN
      )?
      SEMI
    ;


/*
 * ============================================================================
 * 23. DISTRIBUTED ACTOR SCOPE
 * ============================================================================
 *
 * Nested distributed actor declarations are legal.
 *
 * No finite nesting depth is encoded.
 */
distributedActorScope
    : LBRACE
      distributedActorMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 24. STATEMENT ADAPTERS
 * ============================================================================
 *
 * These adapters are intentionally small and do not redefine the universal
 * statement grammar.
 */
distributedActorStatement
    : distributedActorSpawnStatement
    | distributedActorInvocationStatement
    | distributedActorSendStatement
    | distributedActorAskStatement
    | distributedActorForwardStatement
    ;


/*
 * ============================================================================
 * 25. OPEN-WORLD POLICY OPERATION
 * ============================================================================
 *
 * This wrapper allows future distributed actor operations to remain
 * representable without adding a keyword for every operation.
 */
distributedActorOperation
    : qualifiedName
      LPAREN
      optionalExpressionList
      RPAREN
    ;


distributedActorOperationStatement
    : distributedActorOperation
      SEMI
    ;


/*
 * ============================================================================
 * 26. ARGUMENT CONTRACT
 * ============================================================================
 *
 * Expression syntax remains owned by Expressions.
 */
distributedActorArgumentList
    : argumentList
    ;


optionalDistributedActorArgumentList
    : distributedActorArgumentList?
    ;


/*
 * ============================================================================
 * 27. CROSS-DOMAIN EXPRESSION
 * ============================================================================
 *
 * Distributed actors may carry arbitrary valid Zamani expressions.
 *
 * This permits composition with:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     AI;
 *     data;
 *     networking;
 *     security;
 *     hardware;
 *     memory;
 *     effects.
 *
 * No domain-specific expression grammar is duplicated here.
 */
distributedActorExpression
    : expression
    ;


/*
 * ============================================================================
 * 28. RESOURCE INTEGRATION BOUNDARY
 * ============================================================================
 *
 * Resource syntax is intentionally represented through ordinary expressions
 * in this file.
 *
 * The resource subsystem remains authoritative for resource contracts.
 *
 * This prevents this grammar from becoming a second resource grammar.
 *
 * Examples of downstream semantic concepts:
 *
 *     resource requirement;
 *     capability;
 *     preference;
 *     constraint;
 *     placement;
 *     availability.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. CONCURRENCY INTEGRATION BOUNDARY
 * ============================================================================
 *
 * Distributed actors are concurrent entities, but this grammar does not
 * redefine:
 *
 *     async;
 *     await;
 *     spawn;
 *     parallel;
 *     channels;
 *     synchronization;
 *     cancellation;
 *     futures.
 *
 * General concurrency remains owned by:
 *
 *     grammar/concurrency/
 *
 * This file only adds distributed actor intent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. NETWORKING INTEGRATION BOUNDARY
 * ============================================================================
 *
 * Actor communication must not encode a transport.
 *
 * The following remain downstream:
 *
 *     protocol;
 *     endpoint;
 *     address;
 *     socket;
 *     route;
 *     transport;
 *     bandwidth;
 *     network topology.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. HARDWARE INTEGRATION BOUNDARY
 * ============================================================================
 *
 * This grammar never identifies:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     QPU;
 *     physical core;
 *     physical memory;
 *     physical accelerator.
 *
 * Hardware intent belongs to:
 *
 *     grammar/hardware/
 *
 * and resource/capability semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. DISTRIBUTED INTEGRATION BOUNDARY
 * ============================================================================
 *
 * This grammar composes with:
 *
 *     nodes.g4
 *     services.g4
 *     communication.g4
 *     messaging.g4
 *     replication.g4
 *     consistency.g4
 *     fault-tolerance.g4
 *     placement.g4
 *     topology.g4
 *
 * It must not duplicate their implementation responsibilities.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. FAILURE / RECOVERY INTEGRATION
 * ============================================================================
 *
 * Actor lifecycle may participate in distributed resilience.
 *
 * The grammar itself does not implement recovery.
 *
 * Downstream semantic/runtime layers may classify states such as:
 *
 *     Unknown
 *     Healthy
 *     Degraded
 *     Unstable
 *     Unavailable
 *     Recovering
 *     Quarantined
 *     Retired
 *
 * and outcomes such as:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * These are not parser-level execution decisions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. PROVENANCE
 * ============================================================================
 *
 * Source spans must survive:
 *
 *     distributedActorDeclaration
 *         ->
 *     AST
 *         ->
 *     semantic model
 *         ->
 *     canonical IR
 *         ->
 *     lowered realization
 *
 * This is required for diagnostics, tracing, debugging, optimization
 * provenance and distributed execution observability.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. COMPATIBILITY
 * ============================================================================
 *
 * This grammar establishes a stable distributed-actor boundary.
 *
 * Adding a new semantic policy should normally occur inside the policy
 * expression space rather than requiring a new parser keyword.
 *
 * Syntax changes affecting:
 *
 *     distributedActorDeclaration
 *     distributedActorSpawn
 *     distributedActorSend
 *     distributedActorAsk
 *
 * require explicit compatibility review.
 *
 * Deprecated syntax must be handled through the repository compatibility
 * system rather than silently changing semantic meaning.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. MACROS / METAPROGRAMMING
 * ============================================================================
 *
 * Macro expansion may produce distributed actor syntax.
 *
 * Expanded syntax must pass through the same:
 *
 *     lexer
 *     parser
 *     AST
 *     semantic analysis
 *     resource analysis
 *     capability analysis
 *     security analysis
 *
 * pipeline.
 *
 * This grammar must not become a macro escape hatch.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. DIALECT INTEGRATION
 * ============================================================================
 *
 * Vendor/provider-specific actor features belong in dialects.
 *
 * A dialect must not bypass:
 *
 *     AST validation;
 *     semantic validation;
 *     resource validation;
 *     capability validation;
 *     portability analysis.
 *
 * A dialect may extend semantic meaning through the repository's dialect
 * mechanism without turning this grammar into a provider-specific language.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. DETERMINISTIC PARSING
 * ============================================================================
 *
 * The grammar contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no runtime callbacks;
 *     no environment queries;
 *     no hardware queries;
 *     no network queries;
 *     no random state.
 *
 * Identical token streams must therefore produce equivalent parser structure.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. NEGATIVE SYNTAX BOUNDARY
 * ============================================================================
 *
 * This grammar must reject structurally malformed constructs such as:
 *
 *     distributed actor
 *     distributed actor Counter
 *
 * without a body;
 *
 *     spawn distributed
 *
 * without actor and target information;
 *
 * malformed parameter lists;
 *
 * malformed communication expressions;
 *
 * malformed contract clauses.
 *
 * It must NOT attempt to reject semantic failures such as:
 *
 *     no available node;
 *     insufficient memory;
 *     unavailable capability;
 *     unsupported transport;
 *     unsupported QPU;
 *     invalid placement;
 *     impossible replication policy.
 *
 * Those are downstream errors.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Conformance tests must verify that the grammar places no artificial
 * language-level limits on:
 *
 *     actor declarations;
 *     actor members;
 *     actor handlers;
 *     actor parameters;
 *     messages;
 *     nested distributed scopes;
 *     qualified-name depth;
 *     dependency relationships;
 *     replication intent;
 *     placement expressions;
 *     policy expressions;
 *     program size.
 *
 * Tests should be parameterized.
 *
 * They must NOT establish a universal upper bound.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Required integration coverage includes:
 *
 *     distributed + classical
 *     distributed + quantum
 *     distributed + hybrid
 *     distributed + HDL
 *     distributed + hardware
 *     distributed + AI
 *     distributed + data
 *     distributed + networking
 *     distributed + security
 *     distributed + memory
 *     distributed + concurrency
 *     distributed + effects
 *     distributed + resources
 *
 * Particularly important:
 *
 *     distributed + quantum + classical
 *     distributed + quantum + hardware
 *     distributed + quantum + networking
 *     distributed + quantum + QEC
 *     distributed + quantum + ZQN
 *
 * Quantum lowering must continue through:
 *
 *     quantum::ir
 *
 * without creating a distributed quantum IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     lexer configuration;
 *     grammar version;
 *     dialect configuration;
 *
 * repeated parsing must produce equivalent structural results.
 *
 * Parser behavior must not depend on:
 *
 *     current hardware;
 *     number of nodes;
 *     runtime state;
 *     system clock;
 *     scheduler state;
 *     network availability.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. ROUND-TRIP TEST CONTRACT
 * ============================================================================
 *
 * Where formatter support exists:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter
 *       -> parser
 *
 * must preserve the semantic structure of:
 *
 *     actor identity;
 *     contracts;
 *     state;
 *     handlers;
 *     targets;
 *     operations;
 *     arguments;
 *     lifecycle intent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no universal:
 *
 *     MAX_ACTORS
 *     MAX_PROCESSES
 *     MAX_NODES
 *     MAX_WORKERS
 *     MAX_MESSAGES
 *     MAX_REPLICAS
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_MAILBOX
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It contains no fixed:
 *
 *     CPU identity;
 *     GPU identity;
 *     FPGA identity;
 *     QPU identity;
 *     physical node identity;
 *     physical memory size;
 *     physical topology;
 *     provider;
 *     transport;
 *     machine address.
 *
 * Any numeric literal appearing in an expression is source-program data,
 * not a language-level implementation ceiling.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. SECURITY AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no executable actions;
 *     no filesystem operations;
 *     no network operations;
 *     no secret operations;
 *     no hardware operations;
 *     no runtime calls;
 *     no unsafe Rust;
 *     no dynamic code execution.
 *
 * Security authorization remains downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar uses:
 *
 *     structural repetition;
 *     reusable expression rules;
 *     qualified names;
 *     ordinary recursive blocks.
 *
 * It does not perform semantic searches or runtime operations while parsing.
 *
 * Any practical parser resource limits must be implementation-level controls
 * and must never be represented as language-level semantic limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. RUST CONTRACT
 * ============================================================================
 *
 * This grammar itself contains no Rust.
 *
 * The consuming Zamani compiler must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must not require unsafe Rust.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] Grammar name is unique.
 *
 *     [ ] File is the sole distributed-actor leaf grammar.
 *
 *     [ ] `distributedActorConstruct` is the public boundary.
 *
 *     [ ] General actor semantics remain owned by concurrency/actors.g4.
 *
 *     [ ] No duplicate actor IR exists.
 *
 *     [ ] No lexer rules are defined here.
 *
 *     [ ] Canonical names are reused.
 *
 *     [ ] Canonical expressions are reused.
 *
 *     [ ] Canonical types are reused.
 *
 *     [ ] Canonical blocks are reused where applicable.
 *
 *     [ ] Actor state is represented without physical-memory assumptions.
 *
 *     [ ] Actor communication is transport-independent.
 *
 *     [ ] Placement is target-independent.
 *
 *     [ ] Replication is target-independent.
 *
 *     [ ] Lifecycle is semantic intent.
 *
 *     [ ] Resource requirements remain separate from implementation.
 *
 *     [ ] Capabilities remain separate from allocation.
 *
 *     [ ] Security is not bypassed.
 *
 *     [ ] Quantum semantics do not create a second quantum IR.
 *
 *     [ ] quantum::ir remains canonical.
 *
 *     [ ] No hardware limits are encoded.
 *
 *     [ ] No fixed actor count is encoded.
 *
 *     [ ] No fixed node count is encoded.
 *
 *     [ ] No fixed message count is encoded.
 *
 *     [ ] No fixed replica count is encoded.
 *
 *     [ ] No fixed resource capacity is encoded.
 *
 *     [ ] No provider is embedded.
 *
 *     [ ] No transport is embedded.
 *
 *     [ ] Parsing is deterministic.
 *
 *     [ ] Source spans can be preserved.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Cross-domain tests exist.
 *
 *     [ ] Compatibility tests exist.
 *
 *     [ ] Rust 1.97/1.97.1 integration remains safe-Rust-only.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. FINAL INVARIANTS
 * ============================================================================
 *
 * INVARIANT 1
 * ----------
 * A distributed actor is a logical semantic entity.
 *
 * INVARIANT 2
 * ----------
 * A distributed actor is not inherently a process, thread, machine or node.
 *
 * INVARIANT 3
 * ----------
 * Actor syntax remains owned by the concurrency subsystem.
 *
 * INVARIANT 4
 * ----------
 * This grammar owns distributed specialization only.
 *
 * INVARIANT 5
 * ----------
 * Physical realization remains downstream.
 *
 * INVARIANT 6
 * ----------
 * Resource availability never becomes grammar syntax limits.
 *
 * INVARIANT 7
 * ----------
 * Capability requirements do not allocate resources.
 *
 * INVARIANT 8
 * ----------
 * Placement intent does not identify physical hardware.
 *
 * INVARIANT 9
 * ----------
 * Communication intent does not select a transport.
 *
 * INVARIANT 10
 * -----------
 * Replication intent does not implement replication.
 *
 * INVARIANT 11
 * -----------
 * Lifecycle intent does not invoke the runtime.
 *
 * INVARIANT 12
 * -----------
 * Quantum computation remains routed through canonical `quantum::ir`.
 *
 * INVARIANT 13
 * -----------
 * Distributed actors can participate in classical, quantum, hybrid, HDL,
 * hardware, AI, data, networking and future computation without requiring
 * separate actor languages.
 *
 * INVARIANT 14
 * -----------
 * No unsafe Rust is required.
 *
 * INVARIANT 15
 * -----------
 * The grammar remains deterministic and environment-independent.
 *
 * INVARIANT 16
 * -----------
 * Future semantic distributed actor policies remain extensible without
 * requiring a closed enumeration of every possible policy.
 *
 * INVARIANT 17
 * -----------
 * POCO-REAF is preserved:
 *
 *     Program Once
 *         ->
 *     Compile Once
 *         ->
 *     Run Everywhere
 *         ->
 *     Anywhere
 *         ->
 *     Forever
 *
 * ============================================================================
 * END OF grammar/distributed/actors.g4
 * ============================================================================
 */