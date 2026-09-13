/*
 * ============================================================================
 * Zamani Programming Language
 * Production Future/Async-Result Integration Grammar
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/futures.g4
 *
 * Role:
 *     Parser-level integration grammar for future-like asynchronous results.
 *
 * Language architecture:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic IR
 *       |
 *       +-----------------------------+
 *       |             |               |
 *       v             v               v
 *     classical    quantum/hybrid   distributed
 *       |             |               |
 *       +-------------+---------------+
 *                     |
 *                     v
 *                 scheduling
 *                     |
 *                     v
 *                   runtime
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Stable Rust
 *     No unsafe Rust
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the parser-level future integration boundary;
 *   - future-oriented composition of already-defined asynchronous syntax;
 *   - the stable future-operation parser entry point;
 *   - syntax-level future result observation through the canonical await rule;
 *   - integration contracts between asynchronous syntax and future semantics.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - the `await` keyword;
 *   - the `async` keyword;
 *   - the `spawn` keyword;
 *   - async function declarations;
 *   - function syntax;
 *   - task spawning syntax;
 *   - task scheduling;
 *   - task executors;
 *   - worker/thread counts;
 *   - queues;
 *   - polling;
 *   - wakeups;
 *   - promises;
 *   - runtime future objects;
 *   - cancellation;
 *   - synchronization;
 *   - channels;
 *   - actors;
 *   - parallel execution;
 *   - resource allocation;
 *   - placement;
 *   - hardware;
 *   - topology;
 *   - quantum resources;
 *   - quantum IR;
 *   - classical IR;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - runtime dispatch.
 *
 * Those concerns belong to their owning grammar, semantic, IR, compiler,
 * scheduler, resource, hardware, resilience, or runtime subsystem.
 *
 * ============================================================================
 * CRITICAL ARCHITECTURAL DECISION
 * ============================================================================
 *
 * A "future" is a semantic concept, not necessarily a dedicated lexical
 * construct.
 *
 * The current Zamani lexer already provides asynchronous vocabulary including:
 *
 *     ASYNC
 *     AWAIT
 *     SPAWN
 *     PARALLEL
 *
 * but this file MUST NOT invent a `FUTURE` lexer token merely because this
 * file is named futures.g4.
 *
 * A future may be represented by:
 *
 *     - a language-defined asynchronous value;
 *     - a promise/result abstraction;
 *     - a distributed operation;
 *     - an asynchronous I/O operation;
 *     - an accelerator operation;
 *     - a quantum/hybrid execution request;
 *     - a domain-specific asynchronous computation;
 *     - a future language extension.
 *
 * The semantic layer determines which interpretation applies.
 *
 * ============================================================================
 * WHY THIS FILE IS SMALL
 * ============================================================================
 *
 * This file intentionally does NOT reproduce:
 *
 *     awaitExpression
 *     spawnExpression
 *     asyncFunctionDeclaration
 *     parallelExpression
 *
 * Those productions already have or will have dedicated ownership.
 *
 * Duplicating them would create multiple grammar authorities and would make
 * changes to async semantics require synchronized edits across unrelated files.
 *
 * Instead, this grammar imports the canonical async grammar and exposes a
 * future-specific integration boundary around it.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Future syntax must remain independent of the machine executing the future.
 *
 * The source language must not encode:
 *
 *     MAX_FUTURES
 *     MAX_TASKS
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_CORES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_QUEUE_DEPTH
 *     MAX_MEMORY
 *     MAX_QUBITS
 *     MAX_CONCURRENT_OPERATIONS
 *
 * A future can therefore represent computation executed on:
 *
 *     - a tiny embedded target;
 *     - a single CPU;
 *     - a multicore CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an accelerator;
 *     - a quantum processor;
 *     - a quantum simulator;
 *     - a hybrid system;
 *     - a distributed cluster;
 *     - a cloud platform;
 *     - a future computational substrate.
 *
 * Actual resource availability is determined downstream.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * This grammar follows:
 *
 *     lexer
 *       |
 *       v
 *     core expression syntax
 *       |
 *       v
 *     functions/async.g4
 *       |
 *       v
 *     concurrency/futures.g4
 *       |
 *       v
 *     parser composition
 *
 * It MUST NOT create:
 *
 *     futures.g4 -> runtime
 *     futures.g4 -> scheduler
 *     futures.g4 -> hardware
 *     futures.g4 -> quantum::ir
 *     futures.g4 -> ZQN
 *     futures.g4 -> QEC
 *
 * The direction remains:
 *
 *     grammar
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical IR
 *       |
 *       v
 *     scheduling / lowering / runtime
 *
 * ============================================================================
 * CANONICAL INTEGRATION CONTRACT
 * ============================================================================
 *
 * The canonical async grammar owns the actual await production.
 *
 * Conceptually:
 *
 *     functions/async.g4
 *         |
 *         +--> awaitExpression
 *         |
 *         +--> async function syntax
 *
 * This file consumes `awaitExpression` rather than redefining it.
 *
 * Therefore there is exactly one parser authority for:
 *
 *     await <expression>
 *
 * This prevents divergence between:
 *
 *     functions/async.g4
 *     concurrency/futures.g4
 *     root parser
 *     AST construction
 *
 * ============================================================================
 */

parser grammar Futures;

options {
    tokenVocab = ZamaniLexer;
}

import Async;


/*
 * ============================================================================
 * 1. FUTURE OPERATION
 * ============================================================================
 *
 * Stable public entry point for syntax involving a future-like asynchronous
 * result.
 *
 * At present the language's concrete observation syntax is the canonical
 * `awaitExpression` owned by functions/async.g4.
 *
 * Future creation is deliberately not duplicated here.
 *
 * A future may originate from:
 *
 *     - spawn;
 *     - async function invocation;
 *     - distributed execution;
 *     - accelerator execution;
 *     - quantum/hybrid execution;
 *     - an external asynchronous interface;
 *     - a future dialect.
 *
 * The origin is determined semantically.
 */
futureOperation
    : awaitExpression
    ;


/*
 * ============================================================================
 * 2. FUTURE EXPRESSION
 * ============================================================================
 *
 * Stable expression-level integration point.
 *
 * This rule intentionally delegates to the canonical asynchronous grammar.
 *
 * It does not create a second expression hierarchy.
 */
futureExpression
    : futureOperation
    ;


/*
 * ============================================================================
 * 3. FUTURE OBSERVATION
 * ============================================================================
 *
 * A future may be observed by awaiting it.
 *
 * The operand and awaitability rules remain owned by the canonical
 * awaitExpression implementation.
 *
 * Semantic analysis determines whether the awaited expression actually
 * denotes a future/awaitable computation.
 */
futureObservation
    : awaitExpression
    ;


/*
 * ============================================================================
 * 4. FUTURE RESULT BOUNDARY
 * ============================================================================
 *
 * This rule exists as a stable semantic integration point.
 *
 * It deliberately does not define a concrete Future<T> syntax because:
 *
 *     1. no dedicated FUTURE lexer token is currently required;
 *     2. future-like values may be domain-defined;
 *     3. concrete type syntax belongs to the type system;
 *     4. awaitability is a semantic property;
 *     5. introducing a grammar keyword would unnecessarily close the
 *        language's extension space.
 *
 * Therefore:
 *
 *     futureResult
 *         -> futureOperation
 *
 * and semantic analysis determines the resulting type/value.
 */
futureResult
    : futureOperation
    ;


/*
 * ============================================================================
 * 5. FUTURE-CONSUMING EXPRESSION
 * ============================================================================
 *
 * Generic parser-composition boundary.
 *
 * This allows future-aware portions of the root parser to depend on one stable
 * production rather than on internal implementation rules.
 */
futureConsumingExpression
    : futureExpression
    ;


/*
 * ============================================================================
 * 6. FUTURE SYNTAX ROOT
 * ============================================================================
 *
 * Stable public rule for consumers that specifically need future syntax.
 *
 * Consumers SHOULD prefer this rule over internal rules unless a more specific
 * production is required.
 */
future
    : futureOperation
    ;


/*
 * ============================================================================
 * 7. FUTURE OBSERVATION ROOT
 * ============================================================================
 *
 * Explicitly named root for semantic/frontend tooling that needs to distinguish
 * future observation from future creation.
 */
futureObservationRoot
    : futureObservation
    ;


/*
 * ============================================================================
 * 8. FUTURE RESULT ROOT
 * ============================================================================
 *
 * Stable integration point for semantic/type analysis.
 */
futureResultRoot
    : futureResult
    ;


/*
 * ============================================================================
 * 9. FUTURE EXPRESSION ROOT
 * ============================================================================
 *
 * Stable expression-oriented entry point for parser composition.
 */
futureExpressionRoot
    : futureExpression
    ;


/*
 * ============================================================================
 * 10. SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The grammar intentionally stops at:
 *
 *     awaitExpression
 *
 * The following questions MUST NOT be answered by this grammar:
 *
 *     Is the operand awaitable?
 *     What type does it produce?
 *     What executor runs it?
 *     Where does it run?
 *     How many workers are available?
 *     Is execution local or remote?
 *     Is the computation classical or quantum?
 *     Does it require a GPU?
 *     Does it require a QPU?
 *     Does it require distributed resources?
 *
 * Those are semantic/resource/target questions.
 */
futureSemanticBoundary
    : futureOperation
    ;


/*
 * ============================================================================
 * 11. DOMAIN-NEUTRAL FUTURE BOUNDARY
 * ============================================================================
 *
 * Future syntax must remain domain-neutral.
 *
 * The same future syntax may ultimately lower to:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     accelerator computation
 *     distributed computation
 *     HDL-related asynchronous tooling
 *     networking operations
 *     storage operations
 *     AI/ML operations
 *     future domain extensions
 *
 * The grammar does not enumerate those domains because doing so would make
 * futures.g4 depend on every domain grammar.
 */
domainNeutralFuture
    : futureOperation
    ;


/*
 * ============================================================================
 * 12. FUTURE COMPOSITION
 * ============================================================================
 *
 * This is the preferred stable rule for grammar composition layers.
 *
 * It provides a single future-oriented surface without defining new lexical
 * vocabulary.
 */
futureComposition
    : futureOperation
    ;


/*
 * ============================================================================
 * 13. FUTURE INTEGRATION CONTRACT
 * ============================================================================
 *
 * ROOT PARSER
 * -----------
 *
 * The root parser/composition layer MAY integrate:
 *
 *     futureExpression
 *
 * wherever the canonical expression architecture permits a domain expression
 * extension.
 *
 * It MUST NOT redefine:
 *
 *     awaitExpression
 *
 * merely to integrate futures.
 *
 *
 * FUNCTIONS
 * ---------
 *
 * `functions/async.g4` remains authoritative for:
 *
 *     async function declarations
 *     await expressions
 *     async-specific function syntax
 *
 *
 * TASKS
 * -----
 *
 * `concurrency/tasks.g4` remains responsible for task-oriented syntax such as:
 *
 *     spawn
 *
 * but MUST NOT redefine the future grammar.
 *
 *
 * PARALLELISM
 * -----------
 *
 * `concurrency/parallel.g4` owns parallel execution syntax.
 *
 * This file does not define:
 *
 *     parallel
 *
 *
 * CHANNELS
 * --------
 *
 * `concurrency/channels.g4` owns channel syntax.
 *
 * This file does not define:
 *
 *     channel
 *     send
 *     receive
 *     select
 *
 *
 * CANCELLATION
 * ------------
 *
 * `concurrency/cancellation.g4` owns cancellation syntax.
 *
 * This file does not define:
 *
 *     cancel
 *     cancellation scopes
 *
 *
 * TYPES
 * -----
 *
 * The type system owns concrete future-related type semantics.
 *
 * If Zamani eventually introduces explicit syntax such as:
 *
 *     Future<T>
 *
 * that syntax MUST be introduced through the canonical type grammar and lexer
 * ownership process, not silently added here.
 *
 *
 * RUNTIME
 * -------
 *
 * Runtime futures/promises/executors are downstream implementations.
 *
 * This grammar does not depend on them.
 *
 *
 * ============================================================================
 * 14. AST CONTRACT
 * ============================================================================
 *
 * This grammar must lower to the existing frontend AST representation.
 *
 * In particular, the repository already provides a dedicated Await expression
 * representation.
 *
 * The parser MUST NOT create:
 *
 *     RuntimeFuture
 *     FutureHandle
 *     Executor
 *     PromiseObject
 *     TaskHandle
 *     ThreadHandle
 *     DeviceFuture
 *     QuantumFuture
 *
 * as grammar-level AST objects.
 *
 * The AST represents source semantics.
 *
 * Runtime objects are created only downstream.
 *
 *
 * ============================================================================
 * 15. QUANTUM INTEGRATION
 * ============================================================================
 *
 * A quantum computation may be asynchronous without requiring a special
 * quantum-future grammar.
 *
 * For example:
 *
 *     let result = await submit_quantum_computation();
 *
 * remains structurally:
 *
 *     AwaitExpression
 *         |
 *         +--> ordinary expression
 *
 * Semantic analysis may later determine that the expression represents:
 *
 *     QPU execution
 *     quantum simulation
 *     hybrid execution
 *     remote quantum execution
 *     distributed quantum execution
 *
 * No quantum grammar dependency is introduced here.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 *
 * ============================================================================
 * 16. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A distributed operation may also be awaited:
 *
 *     let result = await remote_compute();
 *
 * This grammar does not encode:
 *
 *     node count
 *     network topology
 *     endpoint
 *     machine identity
 *     placement
 *     communication protocol
 *
 * Those concerns belong to distributed/network/resource/target layers.
 *
 *
 * ============================================================================
 * 17. ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * An accelerator operation may produce an awaitable result.
 *
 * This grammar does not distinguish:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     TPU-like accelerator
 *     quantum accelerator
 *     future accelerator
 *
 * Such distinctions are target/capability semantics.
 *
 *
 * ============================================================================
 * 18. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Future syntax does not express physical resource counts.
 *
 * For example, this grammar does NOT encode:
 *
 *     await on 8 workers
 *     await on 16 cores
 *     await on 4 GPUs
 *     await on 32 qubits
 *
 * If a program has a genuine resource requirement, that requirement belongs
 * to the canonical resource/requirement/constraint system.
 *
 * Requirement, constraint, capability, preference, hint, resource and target
 * remain distinct concepts.
 *
 *
 * ============================================================================
 * 19. SEMANTIC ANALYSIS CONTRACT
 * ============================================================================
 *
 * Semantic analysis consumes this grammar's AST and determines:
 *
 *     - whether the operand is awaitable;
 *     - the resulting value type;
 *     - whether awaiting is legal in the current context;
 *     - effects;
 *     - capabilities;
 *     - resource requirements;
 *     - dependency relationships;
 *     - control-flow consequences;
 *     - domain-specific semantics.
 *
 * The semantic analyzer MUST NOT modify this grammar.
 *
 *
 * ============================================================================
 * 20. IR CONTRACT
 * ============================================================================
 *
 * This grammar has NO direct dependency on:
 *
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     distributed IR
 *     accelerator IR
 *
 * Semantic lowering decides which canonical IR representation is appropriate.
 *
 * Conceptually:
 *
 *     Future/Await syntax
 *             |
 *             v
 *       frontend AST
 *             |
 *             v
 *      semantic await
 *             |
 *             v
 *       canonical program IR
 *             |
 *       +-----+-----+
 *       |           |
 *       v           v
 *   classical   quantum/hybrid
 *
 * The grammar never becomes a second IR.
 *
 *
 * ============================================================================
 * 21. SCHEDULING CONTRACT
 * ============================================================================
 *
 * Scheduling consumes semantic dependencies and execution requirements.
 *
 * This file does not determine:
 *
 *     ordering
 *     start time
 *     duration
 *     worker allocation
 *     resource allocation
 *     placement
 *     routing
 *     retry
 *     recovery
 *
 * The scheduler may serialize or parallelize execution according to available
 * resources while preserving semantic dependencies.
 *
 *
 * ============================================================================
 * 22. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime implementations may represent an awaited computation using:
 *
 *     futures
 *     promises
 *     event loops
 *     task systems
 *     distributed coordination
 *     device queues
 *     accelerator queues
 *     quantum job handles
 *
 * Those are implementation details.
 *
 * They MUST NOT leak backward into this grammar.
 *
 *
 * ============================================================================
 * 23. SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file introduces no finite limit on:
 *
 *     futures
 *     await expressions
 *     tasks
 *     concurrent operations
 *     machines
 *     devices
 *     nodes
 *     qubits
 *     cores
 *     threads
 *     memory
 *
 * The only fixed cardinality in the grammar is the number of syntactic
 * operands required by the language construct itself.
 *
 * This is fundamentally different from imposing a machine capacity.
 *
 *
 * ============================================================================
 * 24. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_FUTURES
 *     MAX_TASKS
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_CORES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_QUBITS
 *     MAX_CONCURRENCY
 *     MAX_PARALLELISM
 *
 * Also forbidden:
 *
 *     device IDs
 *     hardware addresses
 *     processor IDs
 *     topology coordinates
 *     vendor-specific executor names
 *     backend-specific queue names
 *
 * No such value is present in this grammar.
 *
 *
 * ============================================================================
 * 25. DETERMINISM
 * ============================================================================
 *
 * The grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no target-language code
 *     no mutable parser-global state
 *     no runtime callbacks
 *     no I/O
 *     no randomness
 *
 * Therefore parsing remains a deterministic syntax operation.
 *
 *
 * ============================================================================
 * 26. SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - executes no user computation;
 *     - contains no unsafe Rust;
 *     - contains no Rust implementation code;
 *     - does not access hardware;
 *     - does not access network resources;
 *     - does not create runtime objects;
 *     - does not select privileged resources.
 *
 * Any compiler/runtime resource limits belong to explicit downstream policy
 * layers rather than hidden grammar constants.
 *
 *
 * ============================================================================
 * 27. ERROR HANDLING
 * ============================================================================
 *
 * Syntax errors are handled by the canonical ANTLR parser/error strategy.
 *
 * This grammar MUST NOT:
 *
 *     - silently reinterpret malformed future syntax;
 *     - emit runtime diagnostics;
 *     - recover by inventing a future object;
 *     - convert invalid syntax into comments;
 *     - select a backend to resolve syntax ambiguity.
 *
 * Semantic errors such as:
 *
 *     "expression is not awaitable"
 *
 * belong to semantic diagnostics rather than parser syntax errors.
 *
 *
 * ============================================================================
 * 28. COMPATIBILITY
 * ============================================================================
 *
 * Existing valid syntax:
 *
 *     await expression
 *
 * remains owned by the canonical async grammar.
 *
 * This file does not change the meaning of existing await syntax.
 *
 * Consequently, future grammar evolution can occur without changing the
 * canonical await production.
 *
 * If Zamani later introduces explicit future syntax, that change must go
 * through:
 *
 *     lexer authority
 *         |
 *         v
 *     syntax specification
 *         |
 *         v
 *     type/semantic model
 *         |
 *         v
 *     AST
 *         |
 *         v
 *     compatibility policy
 *         |
 *         v
 *     tests
 *
 * rather than being added as an isolated keyword here.
 *
 *
 * ============================================================================
 * 29. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST cover:
 *
 *     await expression
 *     await functionCall()
 *     await ordinaryExpression
 *     futureExpression composition
 *     futureOperation composition
 *     futureObservation
 *
 * Cross-domain semantic fixtures SHOULD include:
 *
 *     classical async computation
 *     distributed computation
 *     accelerator computation
 *     quantum/hybrid computation
 *
 * The parser tests should verify syntax only.
 *
 *
 * ============================================================================
 * 30. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Negative tests MUST verify that this grammar does not accidentally introduce
 * unsupported syntax such as:
 *
 *     future
 *     Future
 *     Future<T>
 *     promise
 *     Promise<T>
 *
 * merely because those names appear in documentation or semantic concepts.
 *
 * Such syntax should become valid only after a deliberate language-design
 * change establishes lexical, type, AST and semantic ownership.
 *
 * Tests must also reject malformed await syntax through the canonical async
 * grammar.
 *
 *
 * ============================================================================
 * 31. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Boundary tests should cover:
 *
 *     very small source files;
 *     large numbers of future observations;
 *     deeply nested expressions;
 *     nested asynchronous computations;
 *     repeated await operations;
 *     large programs containing many independent async operations.
 *
 * No test may use a finite machine limit as the definition of language
 * correctness.
 *
 *
 * ============================================================================
 * 32. ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where the repository's AST printer/serializer supports the construct:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     printer
 *       |
 *       v
 *     source
 *
 * must preserve the semantic meaning of the future/await construct.
 *
 *
 * ============================================================================
 * 33. IMPLEMENTATION COMPLETION CRITERIA
 * ============================================================================
 *
 * futures.g4 is COMPLETE when:
 *
 *   [ ] It compiles with the repository's ANTLR grammar composition.
 *   [ ] It uses the canonical ZamaniLexer.
 *   [ ] It imports the canonical async grammar.
 *   [ ] It does not redefine awaitExpression.
 *   [ ] It does not define a duplicate expression hierarchy.
 *   [ ] It does not define async function syntax.
 *   [ ] It does not define spawn syntax.
 *   [ ] It does not define parallel syntax.
 *   [ ] It introduces no speculative FUTURE token.
 *   [ ] It introduces no machine-size limit.
 *   [ ] It introduces no hardware dependency.
 *   [ ] It introduces no IR dependency.
 *   [ ] It introduces no runtime dependency.
 *   [ ] It has positive parser tests.
 *   [ ] It has negative parser tests.
 *   [ ] It has boundary tests.
 *   [ ] It has cross-domain semantic fixtures.
 *   [ ] Its AST mapping is documented.
 *   [ ] Its semantic ownership is documented.
 *   [ ] Its POCO-REAF behavior is documented.
 *   [ ] Its integration with functions/async.g4 is tested.
 *   [ ] Its integration with concurrency/concurrency.g4 is tested.
 *   [ ] Its integration with the root parser is tested.
 *   [ ] No duplicate grammar authority remains.
 *
 * ============================================================================
 */