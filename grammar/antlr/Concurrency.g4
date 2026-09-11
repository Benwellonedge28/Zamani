/*
 * ============================================================================
 * Zamani Programming Language
 * Canonical Concurrency Parser Grammar
 * ============================================================================
 *
 * File:
 *     grammar/antlr/Concurrency.g4
 *
 * Role:
 *     Reusable ANTLR4 parser grammar for Zamani concurrency syntax.
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     The Zamani compiler MUST be implemented in safe Rust.
 *     No unsafe Rust is required or permitted.
 *
 * ============================================================================
 *
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * Concurrency.g4 defines SOURCE SYNTAX ONLY.
 *
 * It does not define:
 *
 *   - thread counts;
 *   - CPU counts;
 *   - worker counts;
 *   - scheduler implementation;
 *   - OS threads;
 *   - reactor implementations;
 *   - executor implementations;
 *   - queue capacities;
 *   - channel capacities;
 *   - stack sizes;
 *   - heap sizes;
 *   - NUMA topology;
 *   - CPU architecture;
 *   - GPU architecture;
 *   - QPU topology;
 *   - hardware-specific synchronization;
 *   - vendor APIs;
 *   - memory addresses;
 *   - machine word widths.
 *
 * Those concerns belong to semantic analysis, resource analysis, runtime
 * policy, scheduling, lowering, and target realization.
 *
 * ============================================================================
 *
 * PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer.g4
 *       |
 *       v
 *     ZamaniParser.g4
 *       |
 *       +----------------------------+
 *       |                            |
 *       v                            v
 *     Core.g4                  Concurrency.g4
 *       |                            |
 *       +-------------+--------------+
 *                     |
 *                     v
 *                Frontend AST
 *                     |
 *                     v
 *          name/type/effect/resource
 *                analysis
 *                     |
 *                     v
 *              canonical IR
 *                     |
 *              +------+------+
 *              |             |
 *              v             v
 *          classical      quantum
 *          execution      execution
 *              |             |
 *              +------+------+
 *                     |
 *                     v
 *                target lowering
 *
 * ============================================================================
 *
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * A concurrency construct describes intent rather than a machine topology.
 *
 * The grammar therefore contains no constants such as:
 *
 *     MAX_TASKS
 *     MAX_THREADS
 *     MAX_ACTORS
 *     MAX_CHANNELS
 *     MAX_WORKERS
 *     MAX_PARALLELISM
 *
 * The practical implementation may impose explicit resource limits, but such
 * limits MUST NOT become language-level grammar restrictions.
 *
 * A valid program can consequently be lowered to the resources available on
 * the target:
 *
 *     tiny device
 *     workstation
 *     multicore system
 *     accelerator
 *     cluster
 *     distributed system
 *     heterogeneous system
 *     future computational substrate
 *
 * ============================================================================
 *
 * CONCURRENCY MODELS
 * ============================================================================
 *
 * This grammar supports syntax for:
 *
 *     asynchronous functions
 *     awaiting
 *     task spawning
 *     parallel computation
 *     actor declarations
 *     actor spawning
 *     actor messaging
 *     channel construction
 *     send/receive
 *     select-style waiting
 *     join/wait
 *     synchronization scopes
 *     critical sections
 *     structured task scopes
 *
 * The grammar does NOT require a particular runtime concurrency model.
 *
 * ============================================================================
 *
 * SEMANTIC SEPARATION
 * ============================================================================
 *
 * Syntax:
 *
 *     "What did the programmer write?"
 *
 * AST:
 *
 *     "What structural construct did the programmer write?"
 *
 * Semantic analysis:
 *
 *     "Is the concurrent program valid?"
 *
 * Effect/resource analysis:
 *
 *     "What synchronization, capability, resource and effect requirements
 *      does it have?"
 *
 * IR:
 *
 *     "What target-independent computation does it represent?"
 *
 * Scheduler/runtime:
 *
 *     "How should it execute with currently available resources?"
 *
 * ============================================================================
 *
 * IMPORTANT
 * ============================================================================
 *
 * This grammar intentionally reuses canonical Zamani rules such as:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     pattern
 *     parameterList
 *     argumentList
 *     blockExpression
 *     statement
 *     attribute
 *     genericParameters
 *     whereClause
 *
 * Concurrency.g4 MUST NOT redefine those rules.
 *
 * ============================================================================
 */

parser grammar Concurrency;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. ASYNCHRONOUS FUNCTION DECLARATIONS
 * ========================================================================== */

/**
 * Async function declaration.
 *
 * The canonical function grammar may already accept:
 *
 *     async fn name(...) -> Type { ... }
 *
 * This named rule exists as a reusable concurrency-domain production.
 *
 * Semantic analysis determines whether the resulting function is:
 *
 *     - suspendable;
 *     - resumable;
 *     - executor-bound;
 *     - distributed;
 *     - effectful;
 *     - resource constrained.
 */
asyncFunctionDeclaration
    : ASYNC
      FN
      identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      whereClause?
      blockExpression
    ;


/* ============================================================================
 * 2. AWAIT
 * ========================================================================== */

/**
 * Await an asynchronous computation.
 *
 * Examples:
 *
 *     await task;
 *     await computation();
 *     await channel.receive();
 *
 * The operand remains an ordinary Zamani expression.
 */
awaitExpression
    : AWAIT expression
    ;


/**
 * Await as a statement.
 */
awaitStatement
    : awaitExpression
      SEMI?
    ;


/* ============================================================================
 * 3. TASK SPAWNING
 * ========================================================================== */

/**
 * Spawn a concurrent computation.
 *
 * Examples:
 *
 *     spawn computation();
 *
 *     spawn {
 *         work();
 *     }
 *
 * No executor, worker count, thread count or machine topology is encoded.
 */
spawnExpression
    : SPAWN
      (
          blockExpression
        | expression
      )
    ;


/**
 * Spawn as a statement.
 */
spawnStatement
    : spawnExpression
      SEMI?
    ;


/* ============================================================================
 * 4. PARALLEL COMPUTATION
 * ========================================================================== */

/**
 * Parallel computation construct.
 *
 * Examples:
 *
 *     parallel {
 *         a();
 *         b();
 *         c();
 *     }
 *
 *     parallel expression
 *
 * The number of actual execution resources is determined after semantic
 * analysis.
 */
parallelExpression
    : PARALLEL
      (
          blockExpression
        | expression
      )
    ;


/**
 * Parallel statement.
 */
parallelStatement
    : parallelExpression
      SEMI?
    ;


/* ============================================================================
 * 5. STRUCTURED CONCURRENCY
 * ========================================================================== */

/**
 * Structured concurrent scope.
 *
 * The scope provides a syntactic lifetime boundary for concurrent work.
 *
 * Example:
 *
 *     concurrent {
 *         spawn first();
 *         spawn second();
 *     }
 *
 * Lifetime, cancellation and join semantics belong to semantic analysis and
 * runtime policy.
 */
concurrentScope
    : CONCURRENT
      blockExpression
    ;


/**
 * Structured task scope.
 *
 * This construct allows implementations to establish a lexical ownership
 * boundary for child tasks.
 *
 * Example:
 *
 *     task_scope {
 *         spawn work();
 *     }
 */
taskScope
    : TASK_SCOPE
      blockExpression
    ;


/* ============================================================================
 * 6. JOIN / WAIT
 * ========================================================================== */

/**
 * Wait for one or more concurrent computations.
 *
 * Examples:
 *
 *     join task;
 *     join all;
 *
 * A task identifier is intentionally represented as an expression rather than
 * as a dedicated finite task-handle type.
 */
joinExpression
    : JOIN
      (
          ALL
        | ANY
        | expression
      )
    ;


/**
 * Join statement.
 */
joinStatement
    : joinExpression
      SEMI?
    ;


/* ============================================================================
 * 7. ACTORS
 * ========================================================================== */

/**
 * Actor declaration.
 *
 * Example:
 *
 *     actor Counter {
 *         ...
 *     }
 *
 * Actor semantics belong to the semantic/runtime layers.
 */
actorDeclaration
    : visibilityModifier?
      ACTOR
      identifier
      genericParameters?
      inheritanceTypes?
      whereClause?
      LBRACE
      actorMember*
      RBRACE
    ;


/**
 * Actor members.
 *
 * Actors may contain fields and message handlers.
 */
actorMember
    : attribute*
      visibilityModifier?
      actorField
    | attribute*
      visibilityModifier?
      actorHandler
    | attribute*
      visibilityModifier?
      functionSignature
    ;


/**
 * Actor field.
 */
actorField
    : identifier
      COLON
      typeExpression
      SEMI?
    ;


/**
 * Actor message handler.
 *
 * Example:
 *
 *     receive Increment(value: Int) {
 *         ...
 *     }
 *
 * The message type is represented by a canonical type expression.
 */
actorHandler
    : RECEIVE
      identifier
      LPAREN
      parameterList?
      RPAREN
      blockExpression
    ;


/* ============================================================================
 * 8. ACTOR CREATION
 * ========================================================================== */

/**
 * Spawn an actor instance.
 *
 * Example:
 *
 *     spawn_actor Counter(...);
 *
 * The actor identity and constructor arguments are semantic values.
 */
actorSpawnExpression
    : SPAWN_ACTOR
      qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/**
 * Actor creation statement.
 */
actorSpawnStatement
    : actorSpawnExpression
      SEMI?
    ;


/* ============================================================================
 * 9. ACTOR MESSAGE SEND
 * ========================================================================== */

/**
 * Send a message to an actor.
 *
 * Example:
 *
 *     send actor <- Message(value);
 *
 * The target is an ordinary expression.
 *
 * Message identity is not a closed keyword set.
 */
sendExpression
    : SEND
      expression
      ACTOR_ARROW
      expression
    ;


/**
 * Send statement.
 */
sendStatement
    : sendExpression
      SEMI?
    ;


/* ============================================================================
 * 10. CHANNEL CREATION
 * ========================================================================== */

/**
 * Channel declaration.
 *
 * Examples:
 *
 *     channel values: Channel<Int>;
 *
 *     channel values;
 *
 * Capacity is intentionally NOT required at the grammar level.
 *
 * If an implementation supports an explicit capacity expression, it remains
 * an ordinary expression and is checked semantically.
 */
channelDeclaration
    : CHANNEL
      identifier
      (
          COLON
          typeExpression
      )?
      (
          ASSIGN
          channelExpression
      )?
      SEMI?
    ;


/**
 * Channel construction.
 *
 * Example:
 *
 *     channel<Type>()
 *
 *     channel<Type>(capacity)
 *
 * Resource feasibility is semantic/runtime responsibility.
 */
channelExpression
    : CHANNEL
      genericArguments?
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 11. CHANNEL SEND / RECEIVE
 * ========================================================================== */

/**
 * Explicit channel send.
 *
 * Example:
 *
 *     send channel value;
 *
 * The actual channel and payload remain expressions.
 */
channelSendExpression
    : SEND
      expression
      expression
    ;


/**
 * Explicit channel receive.
 *
 * Example:
 *
 *     receive channel
 *
 * This is an expression so it can participate in ordinary Zamani expressions.
 */
channelReceiveExpression
    : RECEIVE
      expression
    ;


/**
 * Receive statement.
 */
channelReceiveStatement
    : channelReceiveExpression
      SEMI?
    ;


/* ============================================================================
 * 12. SELECT
 * ========================================================================== */

/**
 * Select over multiple concurrent operations.
 *
 * Example:
 *
 *     select {
 *         case receive input => handle(input);
 *         case await task => finish(task);
 *     }
 *
 * Selection semantics are resolved downstream.
 */
selectExpression
    : SELECT
      LBRACE
      selectArm*
      RBRACE
    ;


/**
 * One select arm.
 */
selectArm
    : CASE
      selectOperation
      FAT_ARROW
      (
          blockExpression
        | expression
      )
      COMMA?
    ;


/**
 * Select operation.
 *
 * The operation is deliberately open-ended so future concurrency primitives
 * can participate without changing the core grammar.
 */
selectOperation
    : awaitExpression
    | channelReceiveExpression
    | receiveExpression
    | expression
    ;


/* ============================================================================
 * 13. SYNCHRONIZATION
 * ========================================================================== */

/**
 * Synchronization scope.
 *
 * Example:
 *
 *     synchronize resource {
 *         ...
 *     }
 *
 * The resource expression identifies semantic synchronization state.
 */
synchronizeStatement
    : SYNCHRONIZE
      expression
      blockExpression
    ;


/**
 * Critical section.
 *
 * Example:
 *
 *     critical resource {
 *         ...
 *     }
 *
 * Lock acquisition, ordering, fairness and implementation are not parser
 * responsibilities.
 */
criticalStatement
    : CRITICAL
      expression?
      blockExpression
    ;


/* ============================================================================
 * 14. ATOMIC COMPUTATION
 * ========================================================================== */

/**
 * Atomic scope.
 *
 * Example:
 *
 *     atomic {
 *         ...
 *     }
 *
 * The parser records the lexical boundary.
 * The semantic layer determines which operations can legally occur there.
 */
atomicStatement
    : ATOMIC
      blockExpression
    ;


/* ============================================================================
 * 15. TASK CANCELLATION
 * ========================================================================== */

/**
 * Cancellation request.
 *
 * Example:
 *
 *     cancel task;
 *
 * Cancellation is cooperative/semantic unless a target runtime explicitly
 * defines another model.
 */
cancelStatement
    : CANCEL
      expression?
      SEMI?
    ;


/**
 * Cancellation-aware scope.
 *
 * Example:
 *
 *     cancellable {
 *         ...
 *     }
 */
cancellableScope
    : CANCELLABLE
      blockExpression
    ;


/* ============================================================================
 * 16. DEADLINE / TIME-BOUND CONCURRENCY
 * ========================================================================== */

/**
 * Deadline-aware computation.
 *
 * Example:
 *
 *     deadline expression {
 *         ...
 *     }
 *
 * The expression determines the semantic deadline.
 * Clock representation is not fixed by the grammar.
 */
deadlineScope
    : DEADLINE
      expression
      blockExpression
    ;


/**
 * Timeout-aware computation.
 *
 * Example:
 *
 *     timeout expression {
 *         ...
 *     }
 */
timeoutScope
    : TIMEOUT
      expression
      blockExpression
    ;


/* ============================================================================
 * 17. CONCURRENCY COMBINATORS
 * ========================================================================== */

/**
 * Wait for all computations.
 */
awaitAllExpression
    : AWAIT
      ALL
      LPAREN
      argumentList?
      RPAREN
    ;


/**
 * Wait for any computation.
 */
awaitAnyExpression
    : AWAIT
      ANY
      LPAREN
      argumentList?
      RPAREN
    ;


/**
 * Explicit task group.
 *
 * Example:
 *
 *     task_group {
 *         ...
 *     }
 */
taskGroup
    : TASK_GROUP
      blockExpression
    ;


/* ============================================================================
 * 18. CONCURRENCY PATTERNS
 * ========================================================================== */

/**
 * Pattern used by concurrent receive operations.
 *
 * It delegates value matching to canonical Zamani patterns.
 */
concurrencyPattern
    : pattern
    ;


/**
 * Concurrent binding.
 *
 * Example:
 *
 *     spawn |value| ...
 *
 * Binding semantics remain part of semantic analysis.
 */
concurrentBinding
    : identifier
    | pattern
    ;


/* ============================================================================
 * 19. CONCURRENCY ATTRIBUTES
 * ========================================================================== */

/**
 * Concurrency attributes use the canonical Zamani attribute syntax.
 *
 * Examples may include:
 *
 *     #[async]
 *     #[parallel]
 *     #[actor]
 *     #[nonblocking]
 *
 * Their meaning is semantic.
 */
concurrencyAttribute
    : attribute
    ;


/* ============================================================================
 * 20. CONCURRENCY EFFECT INTEGRATION
 * ========================================================================== */

/**
 * A concurrent operation may explicitly carry an effect requirement.
 *
 * Example:
 *
 *     with effects { IO, Network }
 *
 * Effects.g4 owns the effect syntax.
 */
concurrencyEffectClause
    : effectClause
    ;


/* ============================================================================
 * 21. CONCURRENT FUNCTION SIGNATURE
 * ========================================================================== */

/**
 * Signature-only async operation.
 */
asyncFunctionSignature
    : ASYNC?
      FN
      identifier
      genericParameters?
      LPAREN
      parameterList?
      RPAREN
      returnType?
      whereClause?
      SEMI?
    ;


/* ============================================================================
 * 22. DISTRIBUTED CONCURRENCY
 * ========================================================================== */

/**
 * A placement-neutral distributed computation.
 *
 * Example:
 *
 *     distributed {
 *         ...
 *     }
 *
 * No node count, network topology or transport protocol is specified here.
 */
distributedScope
    : DISTRIBUTED
      blockExpression
    ;


/**
 * Explicit remote computation.
 *
 * Example:
 *
 *     remote expression
 *
 * Placement and transport belong to semantic/lowering layers.
 */
remoteExpression
    : REMOTE
      expression
    ;


/* ============================================================================
 * 23. CONCURRENCY REGION
 * ========================================================================== */

/**
 * Generic concurrency region.
 *
 * This is intentionally more abstract than threads/tasks/actors.
 *
 * Example:
 *
 *     concurrent_region {
 *         ...
 *     }
 *
 * This lets future execution substrates map the same source intent onto
 * whatever execution model is available.
 */
concurrencyRegion
    : CONCURRENT_REGION
      blockExpression
    ;


/* ============================================================================
 * 24. CONCURRENCY DECLARATION
 * ========================================================================== */

/**
 * General concurrency declaration.
 *
 * This provides a semantic declaration boundary without forcing one runtime
 * model into the language.
 */
concurrencyDeclaration
    : visibilityModifier?
      CONCURRENCY
      identifier
      genericParameters?
      whereClause?
      (
          SEMI
        | blockExpression
      )
    ;


/* ============================================================================
 * 25. CONCURRENCY BLOCK
 * ========================================================================== */

/**
 * Reusable concurrency block.
 *
 * All contained statements are ordinary Zamani statements or concurrency
 * constructs recognized by the composed parser.
 */
concurrencyBlock
    : LBRACE
      concurrencyElement*
      RBRACE
    ;


concurrencyElement
    : attribute
    | statement
    ;


/* ============================================================================
 * 26. CONCURRENCY EXPRESSION
 * ========================================================================== */

/**
 * Domain-dispatch rule for concurrency expressions.
 *
 * The root parser may use this rule when it needs to distinguish concurrency
 * constructs from ordinary expressions.
 */
concurrencyExpression
    : awaitExpression
    | spawnExpression
    | parallelExpression
    | joinExpression
    | actorSpawnExpression
    | sendExpression
    | channelExpression
    | channelReceiveExpression
    | selectExpression
    | awaitAllExpression
    | awaitAnyExpression
    | remoteExpression
    ;


/* ============================================================================
 * 27. CONCURRENCY STATEMENT
 * ========================================================================== */

/**
 * Domain-dispatch rule for concurrency statements.
 */
concurrencyStatement
    : awaitStatement
    | spawnStatement
    | parallelStatement
    | joinStatement
    | actorSpawnStatement
    | sendStatement
    | channelDeclaration
    | channelReceiveStatement
    | synchronizeStatement
    | criticalStatement
    | atomicStatement
    | cancelStatement
    | concurrentScope
    | taskScope
    | cancellableScope
    | deadlineScope
    | timeoutScope
    | taskGroup
    | distributedScope
    | concurrencyRegion
    ;


/* ============================================================================
 * 28. ACTOR MESSAGE EXPRESSION
 * ========================================================================== */

/**
 * Generic actor message construction.
 *
 * Example:
 *
 *     Message(value)
 *
 * Message names remain ordinary qualified names.
 */
actorMessageExpression
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 29. MESSAGE PATTERN
 * ========================================================================== */

/**
 * Generic actor message pattern.
 *
 * Example:
 *
 *     Message(value)
 *
 * No closed message vocabulary is defined.
 */
actorMessagePattern
    : qualifiedName
      (
          LPAREN
          argumentPatternList?
          RPAREN
      )?
    ;


argumentPatternList
    : concurrencyPattern
      (COMMA concurrencyPattern)*
      COMMA?
    ;


/* ============================================================================
 * 30. MAILBOX / RECEIVER SCOPE
 * ========================================================================== */

/**
 * Receiver scope.
 *
 * Example:
 *
 *     receive {
 *         case Message(value) => ...
 *     }
 */
receiverScope
    : RECEIVE
      LBRACE
      receiverArm*
      RBRACE
    ;


receiverArm
    : CASE
      actorMessagePattern
      FAT_ARROW
      (
          blockExpression
        | expression
      )
      COMMA?
    ;


/* ============================================================================
 * 31. ACTOR BEHAVIOR
 * ========================================================================== */

/**
 * Actor behavior block.
 */
actorBehavior
    : BEHAVIOR
      blockExpression
    ;


/* ============================================================================
 * 32. TASK HANDLE OPERATIONS
 * ========================================================================== */

/**
 * Explicit task result retrieval.
 *
 * Example:
 *
 *     task_result task
 */
taskResultExpression
    : TASK_RESULT
      expression
    ;


/**
 * Explicit task completion query.
 *
 * Example:
 *
 *     task_done task
 */
taskDoneExpression
    : TASK_DONE
      expression
    ;


/* ============================================================================
 * 33. CONCURRENCY RESOURCE REFERENCES
 * ========================================================================== */

/**
 * Resource references remain semantic identifiers.
 *
 * Examples:
 *
 *     executor
 *     scheduler
 *     mailbox
 *     lock
 *     semaphore
 *
 * The grammar does not define a finite resource catalogue.
 */
concurrencyResource
    : qualifiedName
    ;


/* ============================================================================
 * 34. CONCURRENCY POLICY REFERENCE
 * ========================================================================== */

/**
 * Optional named policy reference.
 *
 * Example:
 *
 *     policy::deterministic
 *
 * The grammar does not prescribe policy semantics.
 */
concurrencyPolicy
    : qualifiedName
    ;


/* ============================================================================
 * 35. CONCURRENCY CONFIGURATION
 * ========================================================================== */

/**
 * Generic concurrency configuration.
 *
 * Example:
 *
 *     concurrency_options {
 *         ...
 *     }
 *
 * Configuration fields are intentionally parsed structurally.
 * Resource validation belongs downstream.
 */
concurrencyOptions
    : CONCURRENCY_OPTIONS
      blockExpression
    ;


/* ============================================================================
 * 36. SEMANTICALLY NEUTRAL EXECUTION PLACEMENT
 * ========================================================================== */

/**
 * Placement annotation.
 *
 * Example:
 *
 *     placement expression
 *
 * The expression may represent a logical placement, policy, capability,
 * region, node class, or future execution substrate.
 */
placementExpression
    : PLACEMENT
      expression
    ;


/* ============================================================================
 * 37. CONCURRENCY COMPOSITION
 * ========================================================================== */

/**
 * Composition of concurrent computations.
 *
 * The actual algebra is defined by semantic analysis.
 */
concurrencyComposition
    : concurrencyExpression
      (
          AND
          concurrencyExpression
      )*
    ;


/* ============================================================================
 * 38. INTEGRATION CONTRACT
 * ============================================================================
 *
 * Concurrency.g4 is NOT a standalone Zamani language.
 *
 * It is a parser-domain grammar composed with the canonical Zamani parser.
 *
 * It depends on canonical rules supplied by the root/core parser:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     pattern
 *     parameterList
 *     argumentList
 *     blockExpression
 *     statement
 *     attribute
 *     genericParameters
 *     genericArguments
 *     whereClause
 *     returnType
 *     functionSignature
 *     visibilityModifier
 *     inheritanceTypes
 *     effectClause
 *
 * The root parser MUST be the only authority that decides where concurrency
 * productions can occur in the complete Zamani language.
 *
 * ============================================================================
 *
 * LEXER INTEGRATION
 * ============================================================================
 *
 * Existing canonical lexical tokens include:
 *
 *     ASYNC
 *     AWAIT
 *     SPAWN
 *     PARALLEL
 *
 * Existing concurrency-related lexical concepts also include:
 *
 *     ACTOR
 *     RECEIVE
 *     SEND
 *     CHANNEL
 *     SELECT
 *     JOIN
 *     CANCEL
 *     ATOMIC
 *     CRITICAL
 *     SYNCHRONIZE
 *     DISTRIBUTED
 *     REMOTE
 *
 * If any of these tokens are not yet present in the canonical
 * ZamaniLexer.g4, they MUST be added there before this grammar is generated.
 *
 * They MUST NOT be declared as lexer rules inside this file.
 *
 * ============================================================================
 *
 * NO DUPLICATED EXPRESSION GRAMMAR
 * ============================================================================
 *
 * This file intentionally does not define:
 *
 *     expression
 *     assignmentExpression
 *     logicalExpression
 *     primaryExpression
 *     typeExpression
 *     pattern
 *
 * The canonical parser owns those constructs.
 *
 * ============================================================================
 *
 * NO HARD-CODED RESOURCE MODEL
 * ============================================================================
 *
 * This grammar never assumes:
 *
 *     one thread per task;
 *     one task per core;
 *     one actor per process;
 *     one channel per queue;
 *     one scheduler per machine;
 *     one node per actor;
 *     one executor per runtime.
 *
 * Such mappings are implementation choices.
 *
 * ============================================================================
 *
 * NO QUANTUM-SPECIFIC CONCURRENCY ASSUMPTIONS
 * ============================================================================
 *
 * Concurrency syntax remains independent from quantum hardware.
 *
 * A concurrent computation may eventually contain:
 *
 *     classical computation
 *     quantum computation
 *     distributed computation
 *     simulation
 *     AI computation
 *     I/O
 *     temporal computation
 *
 * without requiring this grammar to know the target hardware.
 *
 * ============================================================================
 *
 * SAFETY
 * ============================================================================
 *
 * The language keyword `unsafe`, if supported elsewhere in Zamani, does not
 * imply Rust `unsafe`.
 *
 * The Zamani compiler itself remains safe Rust.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * Concurrency may introduce nondeterminism.
 *
 * The grammar does not hide this fact.
 *
 * Semantic analysis MUST preserve the language rule that nondeterminism is
 * explicit through concurrency, effects, randomness, quantum measurement,
 * external systems, or other declared mechanisms.
 *
 * ============================================================================
 *
 * FUTURE EXTENSION
 * ============================================================================
 *
 * New concurrency models MUST prefer adding semantic abstractions rather than
 * embedding hardware assumptions.
 *
 * Examples include:
 *
 *     dataflow
 *     actors
 *     CSP-style communication
 *     futures
 *     structured concurrency
 *     distributed actors
 *     transactional concurrency
 *     speculative execution
 *     heterogeneous execution
 *     accelerator task graphs
 *     quantum-classical orchestration
 *     fault-tolerant distributed execution
 *
 * ============================================================================
 */

