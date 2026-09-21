/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/await.g4
 *
 * Grammar:
 *     Await
 *
 * Status:
 *     CANONICAL AWAIT-SYNTAX GRAMMAR
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     Parser grammar only.
 *
 *     This file contains:
 *       - no embedded Rust;
 *       - no semantic predicates;
 *       - no actions;
 *       - no runtime execution;
 *       - no resource discovery;
 *       - no scheduling;
 *       - no hardware discovery;
 *       - no target selection;
 *       - no unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE SOURCE-LEVEL SYNTAX AUTHORITY for:
 *
 *     await
 *
 * The construct is intentionally domain-neutral.
 *
 * `await` expresses observation/dependency on an asynchronous or otherwise
 * suspendable computation.
 *
 * It does NOT specify how that computation is implemented.
 *
 * The awaited computation may ultimately be:
 *
 *     local;
 *     remote;
 *     distributed;
 *     CPU-executed;
 *     GPU-executed;
 *     FPGA/ASIC accelerated;
 *     accelerator-backed;
 *     quantum/classical;
 *     simulated;
 *     hardware-backed;
 *     implemented by a future computational substrate.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     awaitExpression
 *
 * This file is the canonical owner of the source-level `await` operator.
 *
 * The following files MUST NOT define another authoritative `awaitExpression`:
 *
 *     grammar/functions/async.g4
 *     grammar/concurrency/tasks.g4
 *     grammar/concurrency/futures.g4
 *     grammar/expressions/
 *     grammar/antlr/
 *     grammar/Zamani.g4
 *
 * Other grammar components may import and consume this rule.
 *
 * ============================================================================
 * OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - awaitExpression;
 *     - await operand grammar boundary;
 *     - await precedence boundary;
 *     - await-specific parser composition contract.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - async function declarations;
 *     - function declarations;
 *     - futures;
 *     - promises;
 *     - tasks;
 *     - task spawning;
 *     - parallelism;
 *     - channels;
 *     - actors;
 *     - cancellation;
 *     - synchronization;
 *     - scheduling;
 *     - worker allocation;
 *     - thread allocation;
 *     - resource allocation;
 *     - hardware;
 *     - quantum operations;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime implementation;
 *     - executor implementation;
 *     - ABI selection.
 *
 * Those responsibilities belong to their respective canonical owners.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
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
 *     awaitExpression
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     name/type/effect analysis
 *          |
 *          v
 *     awaitability analysis
 *          |
 *          v
 *     resource/capability analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *      classical          quantum::ir        distributed/HDL
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                              v
 *                     optimization/lowering
 *                              |
 *                              v
 *                         scheduling
 *                              |
 *                              v
 *                           runtime
 *
 * The grammar never skips directly from syntax to runtime.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexer owns the spelling of:
 *
 *     await
 *
 * This grammar therefore MUST NOT define:
 *
 *     AWAIT : 'await';
 *
 * or any other lexer rule.
 *
 * The token is expected to be supplied by the canonical Zamani lexer
 * vocabulary.
 *
 * ============================================================================
 * PARSER DEPENDENCY
 * ============================================================================
 *
 * This grammar imports:
 *
 *     Postfix
 *
 * because `await` intentionally consumes exactly one canonical postfix
 * expression.
 *
 * This gives the construct access to:
 *
 *     identifiers;
 *     calls;
 *     member access;
 *     indexing;
 *     chained postfix operations;
 *     other canonical postfix syntax.
 *
 * It deliberately does NOT import the complete expression grammar.
 *
 * This avoids the dependency cycle:
 *
 *     Expressions
 *         -> Await
 *         -> Expressions
 *
 * Instead the dependency is:
 *
 *     Expressions
 *         -> Await
 *             -> Postfix
 *
 * The canonical expression hierarchy remains owned by `Expressions`.
 *
 * ============================================================================
 */

parser grammar Await;

options {
    tokenVocab = ZamaniLexer;
}

import Postfix;


/*
 * ============================================================================
 * 1. CANONICAL AWAIT EXPRESSION
 * ============================================================================
 *
 * Canonical source form:
 *
 *     await computation
 *
 * Examples:
 *
 *     await task
 *
 *     await compute()
 *
 *     await service.request(value)
 *
 *     await stream.next()
 *
 *     await accelerator.run(kernel)
 *
 *     await quantum_operation()
 *
 * The operand is deliberately a postfix expression rather than an arbitrary
 * complete expression.
 *
 * This establishes the precedence boundary:
 *
 *     await compute()
 *
 * means:
 *
 *     await (compute())
 *
 * while:
 *
 *     await compute() + value
 *
 * is represented by the surrounding expression hierarchy as:
 *
 *     (await (compute())) + value
 *
 * The await grammar therefore cannot accidentally consume the entire
 * surrounding binary expression.
 */
awaitExpression
    : AWAIT postfixExpression
    ;


/*
 * ============================================================================
 * 2. AWAIT OPERAND
 * ============================================================================
 *
 * Named adapter for semantic tooling and parser composition.
 *
 * This rule does not create a second expression hierarchy.
 */
awaitOperand
    : postfixExpression
    ;


/*
 * ============================================================================
 * 3. AWAIT OPERATION
 * ============================================================================
 *
 * Stable semantic/parser boundary.
 *
 * This name allows future grammar consumers to distinguish an await operation
 * from the generic expression hierarchy without duplicating its implementation.
 */
awaitOperation
    : awaitExpression
    ;


/*
 * ============================================================================
 * 4. AWAIT EXPRESSION ADAPTER
 * ============================================================================
 *
 * Stable expression-domain integration point.
 *
 * The canonical expression grammar may consume this rule at its prefix/unary
 * precedence level.
 */
awaitExpressionAdapter
    : awaitExpression
    ;


/*
 * ============================================================================
 * 5. AWAIT STATEMENT ADAPTER
 * ============================================================================
 *
 * Await remains fundamentally an expression.
 *
 * Statement ownership remains with the canonical statement grammar.
 *
 * This adapter exists only for parser compositions that need a named
 * await-specific statement boundary.
 *
 * It does NOT create a second statement hierarchy.
 *
 * The surrounding statement grammar remains responsible for deciding whether
 * an expression may appear as an expression statement and whether a semicolon
 * is required.
 */
awaitStatementExpression
    : awaitExpression
    ;


/*
 * ============================================================================
 * 6. AWAIT COMPOSITION ROOT
 * ============================================================================
 *
 * Stable public entry point for grammar consumers.
 *
 * Consumers that need specifically await syntax SHOULD use:
 *
 *     awaitConstruct
 *
 * rather than depending on internal adapters.
 */
awaitConstruct
    : awaitExpression
    ;


/*
 * ============================================================================
 * 7. SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The grammar deliberately accepts:
 *
 *     await expression
 *
 * without attempting to determine whether the operand is actually awaitable.
 *
 * For example:
 *
 *     await task
 *
 * may be semantically valid.
 *
 * But:
 *
 *     await 42
 *
 * may be syntactically valid while being semantically invalid.
 *
 * The semantic/type system must determine whether the operand supports the
 * language's awaitable contract.
 *
 * The grammar MUST NOT contain a closed list such as:
 *
 *     future
 *     task
 *     promise
 *     channel
 *     stream
 *     quantum_job
 *
 * because awaitability is a semantic property and because future computation
 * domains must remain extensible.
 */


/*
 * ============================================================================
 * 8. TYPE-SYSTEM CONTRACT
 * ============================================================================
 *
 * This grammar does not define:
 *
 *     Future<T>
 *     Task<T>
 *     Promise<T>
 *     Async<T>
 *     Stream<T>
 *
 * as special parser constructs.
 *
 * If these are valid Zamani types, their syntax belongs to the canonical
 * type grammar.
 *
 * Semantic analysis determines:
 *
 *     whether an operand is awaitable;
 *     the resulting value type;
 *     error/result propagation;
 *     suspension behavior;
 *     effect requirements.
 *
 * Therefore:
 *
 *     awaitExpression
 *         -> operand expression
 *         -> type checking
 *         -> awaitability checking
 *
 * rather than:
 *
 *     awaitExpression
 *         -> hard-coded Future/Task grammar.
 */


/*
 * ============================================================================
 * 9. EFFECT CONTRACT
 * ============================================================================
 *
 * Await may introduce or observe effects such as:
 *
 *     asynchronous execution;
 *     suspension;
 *     synchronization;
 *     communication;
 *     external interaction;
 *     distributed execution;
 *     cancellation interaction.
 *
 * This grammar does not encode those effects.
 *
 * Effect analysis owns their representation and validation.
 *
 * The AST/semantic layer must preserve enough source information to associate
 * the effect with this await operation.
 */


/*
 * ============================================================================
 * 10. MEMORY / OWNERSHIP CONTRACT
 * ============================================================================
 *
 * Await may create an important semantic boundary for:
 *
 *     ownership;
 *     borrowing;
 *     lifetimes;
 *     mutation;
 *     aliasing;
 *     shared state.
 *
 * None of those rules belong in this grammar.
 *
 * The canonical memory/type system remains authoritative.
 *
 * Semantic analysis may reject code where a value cannot legally survive
 * suspension.
 *
 * For example, the grammar may accept:
 *
 *     await operation()
 *
 * while semantic analysis determines whether values captured by the
 * surrounding computation are valid across the suspension point.
 */


/*
 * ============================================================================
 * 11. ERROR-PROPAGATION CONTRACT
 * ============================================================================
 *
 * The grammar does not decide how errors/results are represented.
 *
 * Depending on the surrounding type/effect system, awaiting a computation may
 * produce:
 *
 *     T
 *
 *     Result<T, E>
 *
 *     an effectful result;
 *
 *     a domain-specific completion value.
 *
 * Those meanings are semantic/type-system responsibilities.
 *
 * `await` MUST NOT force a particular runtime Future/Promise implementation.
 */


/*
 * ============================================================================
 * 12. CANCELLATION CONTRACT
 * ============================================================================
 *
 * Await may interact with cancellation, but cancellation is not implemented
 * here.
 *
 * This grammar MUST NOT define:
 *
 *     cancel thread
 *     cancel worker
 *     cancel executor
 *     cancel device
 *     cancel queue
 *
 * Cancellation semantics belong to the effect/execution/resilience systems.
 *
 * A future source-level cancellation construct, if introduced, must have its
 * own independent syntax and semantic contract.
 */


/*
 * ============================================================================
 * 13. CONCURRENCY CONTRACT
 * ============================================================================
 *
 * Await is one operation within the broader concurrency model.
 *
 * Other concurrency constructs remain independently owned:
 *
 *     tasks
 *     spawn
 *     parallel
 *     task parallelism
 *     data parallelism
 *     futures
 *     channels
 *     actors
 *     synchronization
 *
 * `await.g4` must not redefine them.
 *
 * In particular:
 *
 *     spawn
 *
 * is NOT an alias for:
 *
 *     await
 *
 * and:
 *
 *     await
 *
 * does NOT imply a particular executor or worker model.
 */


/*
 * ============================================================================
 * 14. TASK INTEGRATION
 * ============================================================================
 *
 * Task syntax may use:
 *
 *     awaitExpression
 *
 * to observe task completion.
 *
 * The task grammar must therefore consume:
 *
 *     awaitExpression
 *
 * rather than defining another await rule.
 *
 * Conceptually:
 *
 *     spawn computation
 *         |
 *         v
 *     task/future semantic value
 *         |
 *         v
 *     await expression
 *
 * The actual dependency relation is constructed during semantic/IR analysis.
 *
 * No task identifier or runtime handle is created by this grammar.
 */


/*
 * ============================================================================
 * 15. FUTURE INTEGRATION
 * ============================================================================
 *
 * The futures grammar must consume:
 *
 *     awaitExpression
 *
 * rather than redefine:
 *
 *     await <expression>
 *
 * A future can originate from:
 *
 *     local computation;
 *     async function invocation;
 *     task creation;
 *     remote execution;
 *     distributed computation;
 *     accelerator execution;
 *     quantum/classical execution;
 *     external services;
 *     future dialects.
 *
 * The grammar does not need to know which origin applies.
 */


/*
 * ============================================================================
 * 16. ASYNC FUNCTION INTEGRATION
 * ============================================================================
 *
 * Async function declaration syntax remains owned by:
 *
 *     grammar/functions/async.g4
 *
 * That file must continue to own:
 *
 *     asyncModifier
 *
 * but MUST consume:
 *
 *     awaitExpression
 *
 * from this grammar rather than defining it itself.
 *
 * Therefore:
 *
 *     async fn compute(...) -> T {
 *         await operation()
 *     }
 *
 * is assembled from:
 *
 *     AsyncFunctions
 *         |
 *         +--> asyncModifier
 *         |
 *         +--> Await.awaitExpression
 *
 * with function structure remaining owned by the Functions grammar.
 */


/*
 * ============================================================================
 * 17. EXPRESSION INTEGRATION
 * ============================================================================
 *
 * The canonical expression hierarchy remains authoritative.
 *
 * Its prefix/unary layer should admit:
 *
 *     awaitExpression
 *
 * without redefining it.
 *
 * Conceptually:
 *
 *     prefixExpression
 *         : ...
 *         | awaitExpression
 *         | ...
 *         ;
 *
 * The exact precedence architecture remains owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file must not introduce another `expression` rule.
 */


/*
 * ============================================================================
 * 18. STATEMENT INTEGRATION
 * ============================================================================
 *
 * Await is an expression and should normally enter statements through the
 * canonical expression-statement mechanism.
 *
 * This avoids creating a duplicate statement hierarchy:
 *
 *     statement
 *         -> awaitStatement
 *
 * alongside:
 *
 *     statement
 *         -> expressionStatement
 *
 * unless the language specification explicitly requires a separate statement
 * form.
 *
 * The preferred production architecture is:
 *
 *     awaitExpression
 *         ->
 *     expression
 *         ->
 *     expressionStatement
 *
 * This ensures that future expression-level features automatically inherit
 * the same statement behavior.
 */


/*
 * ============================================================================
 * 19. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Await is domain-neutral.
 *
 * An awaited operation may represent:
 *
 *     classical work;
 *     quantum work;
 *     hybrid work;
 *     measurement;
 *     simulation;
 *     hardware execution;
 *     remote QPU execution;
 *     distributed quantum/classical orchestration.
 *
 * This grammar does NOT define:
 *
 *     Qubit;
 *     PhysicalQubitId;
 *     gate;
 *     circuit;
 *     quantum operation;
 *     measurement semantics;
 *     QEC;
 *     ZQN;
 *     routing;
 *     scheduling;
 *     pulse semantics;
 *     HAL.
 *
 * If an awaited operation is quantum, its semantic path remains:
 *
 *     await syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum operation
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
 *     QEC / resilience
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * No AsyncQuantumIR or AwaitQuantumIR is introduced.
 */


/*
 * ============================================================================
 * 20. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Await may observe:
 *
 *     scalar computation;
 *     vector computation;
 *     matrix computation;
 *     tensor computation;
 *     dataflow;
 *     scientific computation;
 *     accelerator computation;
 *     AI/ML computation.
 *
 * The grammar does not distinguish those domains.
 *
 * Their semantic types/effects/capabilities determine the meaning.
 */


/*
 * ============================================================================
 * 21. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Await may observe:
 *
 *     hardware simulation;
 *     accelerator execution;
 *     device interaction;
 *     hardware control;
 *     co-designed computation;
 *     asynchronous data movement.
 *
 * It does not define:
 *
 *     clock count;
 *     pipeline depth;
 *     device ID;
 *     register width;
 *     bus width;
 *     FPGA resource count;
 *     ASIC resource count;
 *     memory-bank count.
 *
 * Hardware realization remains downstream.
 */


/*
 * ============================================================================
 * 22. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Await does not imply that the awaited computation is remote.
 *
 * A target may implement:
 *
 *     await local_computation()
 *
 * locally.
 *
 * Another target may distribute that computation.
 *
 * The same source program remains valid because placement is not encoded in
 * the await syntax.
 *
 * Distributed semantics remain owned by:
 *
 *     grammar/distributed/
 *
 * and downstream distributed analysis/lowering.
 */


/*
 * ============================================================================
 * 23. RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     requires_1_thread
 *     requires_8_threads
 *     requires_16_cores
 *     requires_gpu_0
 *     requires_qpu_1
 *     requires_32_qubits
 *
 * or any equivalent fixed target selection.
 *
 * Await expresses a dependency on computation completion/availability.
 *
 * Resource analysis determines what is required to realize that computation.
 *
 * Capability analysis determines whether a target can provide the required
 * execution semantics.
 */


/*
 * ============================================================================
 * 24. POCO-REAF CONTRACT
 * ============================================================================
 *
 * `await` imposes no finite source-level limit on:
 *
 *     awaited operations;
 *     task count;
 *     future count;
 *     concurrent computations;
 *     distributed computations;
 *     execution contexts;
 *     nodes;
 *     devices;
 *     cores;
 *     threads;
 *     accelerators;
 *     qubits;
 *     memory capacity.
 *
 * No finite machine-scale property appears in this grammar.
 *
 * "Infinity" therefore means:
 *
 *     no artificial language-level finite ceiling.
 *
 * Actual execution remains bounded by available:
 *
 *     memory;
 *     compute;
 *     storage;
 *     communication;
 *     target capabilities;
 *     compiler resources;
 *     runtime resources;
 *     deployment policy.
 *
 * Such limits are not language semantics.
 */


/*
 * ============================================================================
 * 25. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_AWAITS
 *     MAX_TASKS
 *     MAX_FUTURES
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_CORES
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * Also forbidden:
 *
 *     CPU_0
 *     GPU_0
 *     FPGA_0
 *     QPU_0
 *     DEVICE_0
 *     THREAD_0
 *     WORKER_0
 *
 * No physical execution identity belongs in this grammar.
 */


/*
 * ============================================================================
 * 26. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * Given identical:
 *
 *     source;
 *     language version;
 *     lexer vocabulary;
 *     grammar version;
 *     parser configuration;
 *
 * `awaitExpression` must produce the same parse structure.
 *
 * Parsing must not depend on:
 *
 *     hardware discovery;
 *     scheduler state;
 *     runtime timing;
 *     thread scheduling;
 *     device availability;
 *     random values.
 *
 * Runtime execution may be nondeterministic where the language semantics
 * permit it; parser determinism remains mandatory.
 */


/*
 * ============================================================================
 * 27. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors are reported by the canonical parser/diagnostic subsystem.
 *
 * Examples of syntax failures include:
 *
 *     await
 *
 *     await )
 *
 *     await ,
 *
 *     await {
 *
 * when those tokens cannot form the required postfix operand in the canonical
 * expression context.
 *
 * This grammar MUST NOT:
 *
 *     print errors;
 *     panic;
 *     silently discard invalid input;
 *     execute user code;
 *     perform recovery with target-specific semantics.
 *
 * Semantic errors such as:
 *
 *     operand is not awaitable;
 *     suspension is forbidden here;
 *     ownership cannot cross suspension;
 *     required capability is unavailable;
 *
 * belong downstream.
 */


/*
 * ============================================================================
 * 28. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The AST construction layer must preserve source information for:
 *
 *     the `await` operator;
 *     the complete await expression;
 *     the awaited operand.
 *
 * The grammar itself does not construct spans.
 *
 * The parser/frontend layer is responsible for mapping ANTLR source positions
 * into Zamani's canonical Span representation.
 */


/*
 * ============================================================================
 * 29. AST CONTRACT
 * ============================================================================
 *
 * The preferred frontend representation is the existing domain-neutral:
 *
 *     Expression::Await(...)
 *
 * or its canonical successor if the frontend AST is being migrated to the
 * universal Operation representation.
 *
 * This grammar MUST NOT introduce:
 *
 *     AwaitThread
 *     AwaitGpu
 *     AwaitQpu
 *     AwaitDevice
 *     AwaitQuantumIR
 *
 * The AST must preserve:
 *
 *     source span;
 *     awaited expression;
 *     syntactic construct identity.
 *
 * Semantic information is added downstream.
 */


/*
 * ============================================================================
 * 30. IR CONTRACT
 * ============================================================================
 *
 * This file defines NO IR.
 *
 * Await semantics must lower into the repository's canonical semantic/IR
 * representation.
 *
 * There must be no:
 *
 *     AwaitIR
 *     TaskIR
 *     FutureIR
 *     AsyncQuantumIR
 *
 * created merely because `await` exists syntactically.
 *
 * If the canonical program IR has a suspension/dependency operation, semantic
 * lowering may use it.
 *
 * The exact IR representation is owned by the canonical IR subsystem.
 */


/*
 * ============================================================================
 * 31. SCHEDULING CONTRACT
 * ============================================================================
 *
 * `await` expresses a dependency/observation point.
 *
 * Scheduling determines how that dependency is realized.
 *
 * The scheduler may use:
 *
 *     cooperative execution;
 *     event-driven execution;
 *     coroutine/state-machine lowering;
 *     worker execution;
 *     thread execution;
 *     distributed execution;
 *     accelerator execution;
 *     heterogeneous execution.
 *
 * The grammar does not choose among them.
 */


/*
 * ============================================================================
 * 32. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime components consume compiled semantic representations.
 *
 * Runtime code MUST NOT depend directly on:
 *
 *     grammar/concurrency/await.g4
 *
 * The dependency direction is:
 *
 *     grammar
 *        |
 *        v
 *     AST
 *        |
 *        v
 *     semantic model / IR
 *        |
 *        v
 *     compiler
 *        |
 *        v
 *     runtime
 *
 * Never:
 *
 *     runtime
 *        |
 *        v
 *     grammar
 */


/*
 * ============================================================================
 * 33. ERROR / RESULT INTEGRATION
 * ============================================================================
 *
 * Await does not force one result convention.
 *
 * The semantic/type system may support:
 *
 *     successful value;
 *     recoverable error;
 *     cancellation;
 *     failure;
 *     retry;
 *     recovery;
 *     domain-specific completion.
 *
 * These are represented by the canonical type/effect/resilience architecture.
 */


/*
 * ============================================================================
 * 34. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing valid Zamani syntax:
 *
 *     await expression
 *
 * must retain its meaning when this grammar becomes canonical.
 *
 * Moving ownership from:
 *
 *     grammar/functions/async.g4
 *
 * to:
 *
 *     grammar/concurrency/await.g4
 *
 * is an ownership refactoring, not a language syntax change.
 *
 * The source form remains:
 *
 *     await <postfix-expression>
 *
 * Existing AST/semantic behavior must be preserved.
 *
 * Compatibility must be checked at:
 *
 *     source syntax;
 *     AST;
 *     semantic model;
 *     IR;
 *     compiler;
 *     runtime.
 */


/*
 * ============================================================================
 * 35. CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Await must remain valid around computations from any supported Zamani
 * domain when their semantic types/effects permit it:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     accelerator
 *     scientific
 *     edge
 *     cloud
 *     future computational domains.
 *
 * The grammar does not add a domain-specific await syntax for any of these.
 */


/*
 * ============================================================================
 * 36. SCALABILITY CONTRACT
 * ============================================================================
 *
 * These forms have no grammar-level finite cardinality:
 *
 *     await a
 *
 *     await a()
 *
 *     await a(b, c, d, ...)
 *
 *     await chain.first().second().third()
 *
 * The number of surrounding operations is controlled by the canonical
 * expression grammar and available compiler resources, not by this file.
 *
 * No source-level syntax change is required when moving from:
 *
 *     tiny
 *
 * to:
 *
 *     embedded
 *
 * to:
 *
 *     multicore
 *
 * to:
 *
 *     accelerator
 *
 * to:
 *
 *     cluster
 *
 * to:
 *
 *     cloud
 *
 * to:
 *
 *     future computational substrate.
 */


/*
 * ============================================================================
 * 37. SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - executes no user code;
 *     - performs no hardware discovery;
 *     - performs no dynamic loading;
 *     - performs no resource allocation;
 *     - contains no target-language actions;
 *     - contains no unsafe Rust.
 *
 * Any capability/security checks belong to semantic analysis and later
 * execution layers.
 */


/*
 * ============================================================================
 * 38. EXTENSIBILITY CONTRACT
 * ============================================================================
 *
 * New asynchronous computation domains must NOT require changing this file.
 *
 * For example, adding a future domain such as:
 *
 *     distributed accelerator computation
 *
 * must not require:
 *
 *     awaitDistributedAccelerator
 *
 * because:
 *
 *     await <postfix-expression>
 *
 * already provides the domain-neutral syntax.
 *
 * The new domain instead supplies its own:
 *
 *     type;
 *     semantic model;
 *     capability contract;
 *     resource contract;
 *     IR/lowering.
 */


/*
 * ============================================================================
 * 39. NO KEYWORD EXPLOSION
 * ============================================================================
 *
 * This file intentionally does not add:
 *
 *     await_cpu
 *     await_gpu
 *     await_fpga
 *     await_qpu
 *     await_cluster
 *     await_remote
 *     await_device
 *     await_accelerator
 *
 * Such syntax would fragment the language and violate the universal
 * target-independent model.
 */


/*
 * ============================================================================
 * 40. TEST CONTRACT
 * ============================================================================
 *
 * REQUIRED POSITIVE TESTS
 *
 *     await task
 *
 *     await compute()
 *
 *     await service.request(value)
 *
 *     await stream.next()
 *
 *     await accelerator.run(kernel)
 *
 *     await quantum_operation()
 *
 *     await distributed_operation()
 *
 *     await tensor.compute()
 *
 *
 * REQUIRED PRECEDENCE TESTS
 *
 *     await compute() + value
 *
 *     value + await compute()
 *
 *     await service.request(value).result()
 *
 *     await values[index]
 *
 *     await values[index].compute()
 *
 *
 * REQUIRED NESTING TESTS
 *
 *     await awaitable()
 *
 *     await outer(inner())
 *
 *     await service.request(await dependency())
 *
 * where nested-await semantics are supported by the surrounding expression
 * grammar.
 *
 *
 * REQUIRED NEGATIVE SYNTAX TESTS
 *
 *     await
 *
 *     await )
 *
 *     await ,
 *
 *     await ;
 *
 *     await }
 *
 * The exact diagnostic text belongs to the canonical diagnostic subsystem.
 *
 *
 * REQUIRED SEMANTIC NEGATIVE TESTS
 *
 *     await 42
 *
 * when the semantic type system does not define integers as awaitable.
 *
 *     await non_awaitable_value
 *
 * when the referenced value is not awaitable.
 *
 * These must parse if syntactically valid and then be rejected by semantic
 * analysis rather than incorrectly reported as syntax failures.
 *
 *
 * REQUIRED CROSS-DOMAIN TESTS
 *
 *     classical await
 *     quantum await
 *     hybrid await
 *     HDL/hardware await
 *     distributed await
 *     AI/data await
 *     accelerator await
 *
 * where corresponding semantic contracts exist.
 *
 *
 * REQUIRED SCALABILITY TESTS
 *
 *     tiny source program;
 *     large source program;
 *     deeply composed postfix computation;
 *     large argument lists;
 *     generated asynchronous computations;
 *     many independent await sites.
 *
 * Tests must verify that no artificial machine-size limit is introduced by
 * this grammar.
 *
 *
 * REQUIRED DETERMINISM TEST
 *
 * Parse identical source repeatedly with identical language/grammar
 * configuration and verify identical parse structure.
 */


/*
 * ============================================================================
 * 41. HARD-CODING TEST CONTRACT
 * ============================================================================
 *
 * Repository validation should reject additions to this file that introduce
 * universal machine constants or target identities.
 *
 * At minimum, validation should scan for:
 *
 *     MAX_
 *     CPU_
 *     GPU_
 *     FPGA_
 *     QPU_
 *     DEVICE_
 *     THREAD_
 *     WORKER_
 *
 * when used to define language limits or target identities.
 *
 * A generic semantic identifier containing one of those strings is not
 * automatically invalid; validation must distinguish actual language
 * hard-coding from ordinary prose/examples.
 */


/*
 * ============================================================================
 * 42. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when all of the following are true:
 *
 * [x] It has exactly one canonical `awaitExpression` rule.
 *
 * [x] The rule consumes canonical `postfixExpression`.
 *
 * [x] It owns no lexer definitions.
 *
 * [x] It owns no runtime behavior.
 *
 * [x] It owns no scheduling behavior.
 *
 * [x] It owns no resource allocation.
 *
 * [x] It owns no hardware topology.
 *
 * [x] It owns no physical device identity.
 *
 * [x] It owns no quantum IR.
 *
 * [x] It owns no QEC.
 *
 * [x] It owns no ZQN.
 *
 * [x] It contains no machine-size ceiling.
 *
 * [x] It contains no unsafe Rust.
 *
 * [x] It has a declared parser dependency on `Postfix`.
 *
 * [x] It has an explicit AST contract.
 *
 * [x] It has an explicit semantic contract.
 *
 * [x] It has an explicit IR contract.
 *
 * [x] It has an explicit scheduling contract.
 *
 * [x] It has an explicit runtime contract.
 *
 * [x] It has an explicit compatibility contract.
 *
 * [x] It has an explicit scalability contract.
 *
 * [x] It has an explicit cross-domain contract.
 *
 * [x] It has an explicit testing contract.
 *
 * [ ] `grammar/functions/async.g4` has removed its duplicate
 *     `awaitExpression`.
 *
 * [ ] `grammar/concurrency/tasks.g4` has removed its duplicate
 *     `taskAwaitExpression` implementation and consumes this rule.
 *
 * [ ] `grammar/concurrency/futures.g4` consumes this rule through its
 *     canonical import chain.
 *
 * [ ] `grammar/expressions/expressions.g4` admits this rule at the canonical
 *     prefix-expression position.
 *
 * [ ] The canonical parser composition imports this grammar exactly once.
 *
 * [ ] Existing source compatibility tests pass.
 *
 * [ ] AST/semantic/IR conformance tests pass.
 *
 * ============================================================================
 */