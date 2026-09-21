/*
 * ============================================================================
 * Zamani Universal Computing Language
 * Production Asynchronous-Concurrency Composition Grammar
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/concurrency/async.g4
 *
 * GRAMMAR
 * -------
 * AsyncConcurrency
 *
 * STATUS
 * ------
 * CANONICAL CONCURRENCY-DOMAIN ASYNC COMPOSITION ADAPTER
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Zamani compiler/frontend:
 *
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Safety requirement:
 *
 *     safe Rust only
 *     no unsafe Rust required
 *
 * This file contains ANTLR grammar only.
 *
 * It contains no:
 *
 *     Rust actions
 *     semantic predicates
 *     filesystem access
 *     network access
 *     hardware discovery
 *     runtime execution
 *     scheduler execution
 *     executor invocation
 *     resource discovery
 *     target selection
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file establishes the asynchronous-concurrency integration boundary
 * inside grammar/concurrency/.
 *
 * IMPORTANT:
 *
 * The actual source-level async syntax already has a canonical owner:
 *
 *     grammar/functions/async.g4
 *
 * whose grammar name is:
 *
 *     AsyncFunctions
 *
 * That grammar owns:
 *
 *     asyncModifier
 *     awaitExpression
 *     asyncExpression
 *
 * Therefore this file MUST NOT redefine those rules.
 *
 * This file instead provides stable concurrency-domain adapter rules that
 * consume the canonical async grammar.
 *
 * This gives the repository a clean ownership model:
 *
 *     grammar/functions/async.g4
 *             |
 *             | owns async syntax
 *             v
 *     grammar/concurrency/async.g4
 *             |
 *             | exposes concurrency-domain integration
 *             v
 *     canonical parser composition
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * EXACTLY ONE FILE OWNS:
 *
 *     asyncModifier
 *     awaitExpression
 *     asyncExpression
 *
 * That owner is:
 *
 *     grammar/functions/async.g4
 *
 * This file MUST NOT define another production with any of those names.
 *
 * In particular, this file MUST NOT contain:
 *
 *     asyncModifier
 *     awaitExpression
 *     asyncExpression
 *
 * as definitions.
 *
 * It may consume them.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Async computation crosses multiple language domains:
 *
 *     functions
 *     expressions
 *     tasks
 *     futures
 *     parallelism
 *     distributed computation
 *     accelerators
 *     quantum/classical execution
 *     HDL/hardware co-design
 *
 * None of those domains should redefine the fundamental `async` or `await`
 * syntax.
 *
 * Instead:
 *
 *     functions/async.g4
 *             |
 *             +--> async function modifier
 *             |
 *             +--> await expression
 *             |
 *             +--> async expression boundary
 *             |
 *             v
 *     concurrency/async.g4
 *             |
 *             +--> concurrency integration boundary
 *             |
 *             v
 *     canonical parser
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - concurrency-domain exposure of canonical async syntax;
 *     - stable parser-facing async concurrency adapter rules;
 *     - explicit integration between functions/async.g4 and concurrency/*;
 *     - documentation of the concurrency boundary;
 *     - domain-neutral async composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - asyncModifier;
 *     - awaitExpression;
 *     - asyncExpression;
 *     - function declarations;
 *     - function definitions;
 *     - function parameters;
 *     - function generics;
 *     - return types;
 *     - ordinary expression precedence;
 *     - postfix expressions;
 *     - tasks;
 *     - spawn;
 *     - futures;
 *     - promises;
 *     - channels;
 *     - actors;
 *     - synchronization;
 *     - cancellation;
 *     - parallel regions;
 *     - data parallelism;
 *     - task parallelism;
 *     - schedulers;
 *     - executors;
 *     - workers;
 *     - threads;
 *     - CPU topology;
 *     - GPU topology;
 *     - FPGA topology;
 *     - QPU topology;
 *     - node topology;
 *     - network topology;
 *     - resource allocation;
 *     - hardware selection;
 *     - placement;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime implementation;
 *     - IR construction.
 *
 * ============================================================================
 * AUTHORITY MAP
 * ============================================================================
 *
 * Canonical lexical authority:
 *
 *     grammar/lexer/
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical async syntax:
 *
 *     grammar/functions/async.g4
 *
 * Canonical function declaration:
 *
 *     grammar/functions/functions.g4
 *
 * Canonical expression hierarchy:
 *
 *     grammar/expressions/
 *
 * Canonical task syntax:
 *
 *     grammar/concurrency/tasks.g4
 *
 * Canonical future syntax:
 *
 *     grammar/concurrency/futures.g4
 *
 * Canonical parallel syntax:
 *
 *     grammar/concurrency/parallel.g4
 *
 * Canonical concurrency composition:
 *
 *     grammar/concurrency/concurrency.g4
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical complete-program grammar:
 *
 *     grammar/Zamani.g4
 *
 * Handwritten frontend lexer:
 *
 *     src/lexer.rs
 *
 * Handwritten frontend parser:
 *
 *     src/parser.rs
 *
 * Domain-neutral frontend AST:
 *
 *     src/ast/
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar depends on:
 *
 *     AsyncFunctions
 *
 * and therefore consumes:
 *
 *     asyncModifier
 *     awaitExpression
 *     asyncExpression
 *
 * It does not import the complete expression grammar.
 *
 * It does not import the task grammar.
 *
 * It does not import the future grammar.
 *
 * It does not import the parallel grammar.
 *
 * It does not import quantum, HDL, hardware, resource, or runtime grammars.
 *
 * This intentional narrow dependency prevents composition cycles and preserves
 * single ownership.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * The canonical lexer remains:
 *
 *     ZamaniLexer
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniLexer
 *
 * is used here.
 *
 * The imported AsyncFunctions grammar already consumes ZamaniLexer and owns the
 * actual async lexical/parser constructs.
 *
 * This grammar imports AsyncFunctions solely to consume those canonical rules.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Async syntax represents portable computational intent.
 *
 * It does NOT identify:
 *
 *     a thread
 *     a worker
 *     a CPU
 *     a core
 *     a GPU
 *     an FPGA
 *     an ASIC
 *     a QPU
 *     an accelerator
 *     a device
 *     a node
 *     a process
 *     a queue
 *     an event loop
 *     an executor
 *     a scheduler
 *     a network endpoint
 *     a physical qubit
 *     a hardware address
 *
 * Consequently:
 *
 *     async
 *     await
 *
 * remain valid regardless of the eventual execution substrate.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The language must not impose an artificial finite machine-scale limit on:
 *
 *     async functions
 *     async expressions
 *     await operations
 *     concurrent computations
 *     task count
 *     future count
 *     execution contexts
 *     distributed participants
 *     devices
 *     nodes
 *     cores
 *     threads
 *     accelerators
 *     qubits
 *     memory
 *     storage
 *
 * "Infinity" means:
 *
 *     no artificial source-language ceiling imposed by this grammar.
 *
 * Actual execution remains constrained by resources available to:
 *
 *     lexer
 *     parser
 *     compiler
 *     optimizer
 *     semantic analyzer
 *     scheduler
 *     runtime
 *     deployment
 *     target hardware
 *
 * Those constraints are implementation/resource constraints rather than
 * language grammar limits.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT define:
 *
 *     MAX_ASYNC
 *     MAX_AWAIT
 *     MAX_TASKS
 *     MAX_FUTURES
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_ACCELERATORS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_CONCURRENCY
 *     MAX_PARALLELISM
 *
 * It must also not encode:
 *
 *     thread IDs
 *     worker IDs
 *     processor IDs
 *     device IDs
 *     node IDs
 *     hardware addresses
 *     physical qubit IDs
 *     topology coordinates
 *
 * No such information belongs in this syntax layer.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Async syntax and resource requirements are separate concepts.
 *
 * This:
 *
 *     async fn compute() { ... }
 *
 * expresses async function semantics.
 *
 * A resource requirement, if one is genuinely part of program semantics,
 * belongs to:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/compile/
 *     grammar/execution/
 *
 * Examples of downstream semantic intent include:
 *
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *     budget
 *     negotiation
 *
 * This file does not reinterpret async syntax as a resource request.
 *
 * ============================================================================
 * SEMANTIC SEPARATION
 * ============================================================================
 *
 * Parsing establishes syntax only.
 *
 * Semantic analysis determines:
 *
 *     whether a function is asynchronous;
 *     whether a suspension point is legal;
 *     whether an await operand is awaitable;
 *     what value await produces;
 *     what effects are introduced;
 *     what ownership/lifetime rules apply;
 *     what capabilities are required;
 *     what resource requirements exist;
 *     whether computation can cross a backend boundary;
 *     whether execution may be distributed;
 *     whether execution may be accelerated;
 *     whether computation interacts with quantum semantics;
 *     whether computation interacts with hardware semantics.
 *
 * None of these questions are answered by this grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does not define a new AST.
 *
 * The canonical frontend AST remains authoritative.
 *
 * Existing handwritten frontend support includes:
 *
 *     Expression::Async
 *     Expression::Await
 *     Expression::Spawn
 *
 * The existing AST also preserves source spans.
 *
 * Therefore the adapter rules in this file must lower through the same AST
 * representation as ordinary async syntax.
 *
 * Conceptually:
 *
 *     asyncConcurrencyModifier
 *             |
 *             v
 *     canonical asyncModifier
 *             |
 *             v
 *     function AST modifier
 *
 * and:
 *
 *     asyncConcurrencyExpression
 *             |
 *             v
 *     canonical asyncExpression
 *             |
 *             v
 *     Expression::Await
 *
 * No:
 *
 *     AsyncConcurrencyAst
 *     AsyncFutureAst
 *     AsyncTaskAst
 *     AsyncQuantumAst
 *     AsyncHardwareAst
 *
 * is introduced here.
 *
 * ============================================================================
 * SEMANTIC / IR CONTRACT
 * ============================================================================
 *
 * The lowering path is:
 *
 *     source
 *         |
 *         v
 *     lexer
 *         |
 *         v
 *     parser
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     canonical semantic representation
 *         |
 *         +----------------------+----------------------+
 *         |                      |                      |
 *         v                      v                      v
 *     classical             quantum::ir          HDL/hardware
 *         |                      |                      |
 *         +----------------------+----------------------+
 *                                |
 *                                v
 *                         optimization
 *                                |
 *                   +------------+-------------+
 *                   |            |             |
 *                   v            v             v
 *                routing     scheduling     resilience
 *                                             |
 *                                             v
 *                                            QEC
 *                                             |
 *                                             v
 *                                            ZQN
 *                                             |
 *                                             v
 *                                            HAL
 *                                             |
 *                                             v
 *                                      target realization
 *
 * This grammar creates NO IR.
 *
 * In particular, this file must never create:
 *
 *     AsyncIR
 *     AsyncConcurrencyIR
 *     FutureIR
 *     TaskIR
 *     AsyncQuantumIR
 *     AsyncHardwareIR
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Async syntax is domain-neutral.
 *
 * An awaited computation may semantically represent:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     quantum simulation
 *     remote quantum execution
 *     distributed quantum execution
 *     accelerator computation
 *
 * Example:
 *
 *     async fn measure_value(q: Qubit) -> Measurement {
 *         await measure(q)
 *     }
 *
 * This grammar does not determine whether `measure` is:
 *
 *     local
 *     simulated
 *     remote
 *     hardware-backed
 *     distributed
 *
 * Semantic analysis determines that.
 *
 * If the computation reaches the quantum domain, the canonical path remains:
 *
 *     frontend AST
 *         ->
 *     semantic quantum operation
 *         ->
 *     quantum::ir
 *         ->
 *     optimization
 *         ->
 *     routing
 *         ->
 *     scheduling
 *         ->
 *     QEC / resilience
 *         ->
 *     ZQN
 *         ->
 *     HAL
 *
 * No async-specific quantum IR is created.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Async computation may await:
 *
 *     hardware operations
 *     accelerator completion
 *     HDL simulation
 *     data movement
 *     device-independent services
 *
 * The grammar does not encode the realization.
 *
 * No:
 *
 *     gpu0
 *     fpga0
 *     qpu0
 *     device0
 *     core0
 *
 * syntax is introduced.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * An awaited operation may semantically represent:
 *
 *     local computation
 *     remote computation
 *     distributed service
 *     actor interaction
 *     network request
 *     cluster computation
 *     accelerator completion
 *     quantum-device completion
 *
 * This grammar does not encode:
 *
 *     node count
 *     endpoint identity
 *     network topology
 *     transport protocol
 *     machine identity
 *     deployment placement
 *
 * Those belong downstream.
 *
 * ============================================================================
 * TASK INTEGRATION
 * ============================================================================
 *
 * Task syntax remains owned by:
 *
 *     grammar/concurrency/tasks.g4
 *
 * In particular, this file does not define:
 *
 *     spawn
 *     task
 *     task group
 *     task scope
 *
 * Async and task syntax therefore remain separate ownership domains.
 *
 * Conceptually:
 *
 *     async syntax
 *          |
 *          +--> async function
 *          |
 *          +--> await
 *          |
 *          v
 *     concurrency semantics
 *          ^
 *          |
 *     task syntax
 *
 * If a task produces an awaitable computation, semantic analysis connects the
 * two concepts after parsing.
 *
 * ============================================================================
 * FUTURE INTEGRATION
 * ============================================================================
 *
 * Future/result abstractions remain owned by:
 *
 *     grammar/concurrency/futures.g4
 *
 * This file does not define:
 *
 *     future
 *     Future
 *     promise
 *     Promise
 *     Task<T>
 *     Future<T>
 *
 * as new syntax.
 *
 * If a future-related type exists in the canonical type system, this file
 * consumes it only through ordinary expressions/types and does not create a
 * second future grammar.
 *
 * ============================================================================
 * PARALLELISM INTEGRATION
 * ============================================================================
 *
 * Parallel execution syntax remains owned by:
 *
 *     grammar/concurrency/parallel.g4
 *
 * This file does not define:
 *
 *     parallel
 *     parallel scope
 *     worker count
 *     thread count
 *     core count
 *
 * Async computation may participate in parallel execution semantically, but
 * the scheduling model is downstream.
 *
 * ============================================================================
 * CANCELLATION INTEGRATION
 * ============================================================================
 *
 * Cancellation syntax remains owned by:
 *
 *     grammar/concurrency/cancellation.g4
 *
 * This file does not define:
 *
 *     cancel
 *     cancellation scope
 *     cancellation token
 *     thread cancellation
 *     executor cancellation
 *     queue cancellation
 *
 * Cancellation semantics are determined by the semantic/effect/runtime layers.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Async operations may introduce or require effects.
 *
 * This file does not define a second effect system.
 *
 * Effects remain owned by:
 *
 *     grammar/effects/
 *
 * Semantic analysis determines:
 *
 *     suspension effects
 *     concurrency effects
 *     I/O effects
 *     distributed effects
 *     hardware effects
 *     quantum effects
 *     cancellation effects
 *
 * ============================================================================
 * OWNERSHIP / LIFETIME INTEGRATION
 * ============================================================================
 *
 * Async suspension can affect ownership and lifetimes.
 *
 * The grammar does not decide:
 *
 *     whether a borrow crosses suspension;
 *     whether a value must be moved;
 *     whether a resource remains live;
 *     whether a capability remains valid;
 *     whether a reference can survive suspension.
 *
 * These are semantic/type-system questions.
 *
 * They belong to:
 *
 *     grammar/types/
 *     grammar/memory/
 *     semantic analysis
 *
 * The syntax remains independent of the eventual memory model.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar depends only on:
 *
 *     source token sequence
 *     selected grammar version
 *     explicitly selected dialect configuration
 *
 * It must not depend on:
 *
 *     wall-clock time
 *     randomness
 *     filesystem state
 *     network state
 *     environment variables
 *     hardware state
 *     scheduler state
 *     runtime state
 *     resource availability
 *     target availability
 *
 * The same source and grammar configuration must produce the same parse.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing async concurrency syntax performs no execution.
 *
 * The parser must not:
 *
 *     create tasks;
 *     create threads;
 *     start executors;
 *     access networks;
 *     inspect devices;
 *     access secrets;
 *     invoke QPUs;
 *     invoke FPGA/HDL hardware;
 *     allocate runtime resources;
 *     execute awaited expressions.
 *
 * `await` is syntax until semantic lowering and runtime execution occur.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics must preserve source locations through the canonical parser
 * and frontend source-span model.
 *
 * This adapter must not swallow or reinterpret syntax errors from the canonical
 * async grammar.
 *
 * Examples of malformed source that must remain malformed:
 *
 *     async
 *     await
 *     await ;
 *
 * Semantic invalidity is distinct from syntax invalidity.
 *
 * For example:
 *
 *     await nonAwaitableValue
 *
 * may be syntactically valid while being semantically invalid.
 *
 * That semantic diagnostic belongs downstream.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces NO new source keyword.
 *
 * It consumes the existing canonical vocabulary:
 *
 *     async
 *     await
 *
 * through AsyncFunctions.
 *
 * Therefore this file must not silently change the meaning of:
 *
 *     async
 *     await
 *
 * Existing valid async syntax remains governed by:
 *
 *     grammar/functions/async.g4
 *
 * Compatibility-sensitive changes must be handled through:
 *
 *     grammar/compatibility/
 *     grammar/spec/compatibility.md
 *
 * ============================================================================
 * HANDWRITTEN RUST FRONTEND CONFORMANCE
 * ============================================================================
 *
 * The repository contains:
 *
 *     src/lexer.rs
 *     src/parser.rs
 *
 * The handwritten frontend already represents:
 *
 *     async
 *     await
 *     spawn
 *
 * in its token/parser model.
 *
 * This grammar does not replace that implementation.
 *
 * Instead, both surfaces must converge on the same language contract.
 *
 * The handwritten AST currently contains:
 *
 *     Expression::Async
 *     Expression::Await
 *     Expression::Spawn
 *
 * Therefore no second async AST is permitted.
 *
 * ============================================================================
 * RUST SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * Its generated/consuming implementation must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * The compiler/frontend must not require `unsafe`.
 *
 * Where crate-level enforcement is appropriate, the implementation should use:
 *
 *     #![forbid(unsafe_code)]
 *
 * This grammar itself cannot introduce unsafe operations because it contains no
 * executable Rust actions.
 *
 * ============================================================================
 * COMPOSITION RULE
 * ============================================================================
 *
 * This file provides adapter rules only.
 *
 * It MUST NOT be used as an alternative parser root.
 *
 * The canonical composition remains:
 *
 *     grammar/Zamani.g4
 *          |
 *          v
 *     grammar/antlr/ZamaniParser.g4
 *          |
 *          v
 *     grammar/concurrency/async.g4
 *          |
 *          v
 *     AsyncFunctions
 *
 * The exact parser-composition wiring belongs to the canonical parser
 * composition layer.
 *
 * ============================================================================
 * PUBLIC INTEGRATION RULES
 * ============================================================================
 *
 * The following rules are intentionally stable public adapter names:
 *
 *     asyncConcurrencyModifier
 *     asyncConcurrencyExpression
 *     asyncConcurrencyAwait
 *
 * They are adapters.
 *
 * They do not become alternative owners of async syntax.
 *
 * Downstream parser composition may use:
 *
 *     asyncConcurrencyModifier
 *
 * where the concurrency layer needs to recognize the canonical async modifier.
 *
 * It may use:
 *
 *     asyncConcurrencyExpression
 *
 * where the concurrency layer needs an asynchronous expression boundary.
 *
 * It may use:
 *
 *     asyncConcurrencyAwait
 *
 * where tooling needs an explicitly named concurrency-domain await boundary.
 *
 * ============================================================================
 * IMPORTANT ADAPTER INVARIANT
 * ============================================================================
 *
 * The following equivalences are structural:
 *
 *     asyncConcurrencyModifier
 *         ==
 *     asyncModifier
 *
 *     asyncConcurrencyAwait
 *         ==
 *     awaitExpression
 *
 *     asyncConcurrencyExpression
 *         ==
 *     asyncExpression
 *
 * They must not acquire independent semantics.
 *
 * ============================================================================
 * FEATURE EXTENSIBILITY
 * ============================================================================
 *
 * Future async-related constructs must not automatically be added here.
 *
 * A new construct requires:
 *
 *     specification
 *         ->
 *     lexical contract if required
 *         ->
 *     AST contract
 *         ->
 *     semantic contract
 *         ->
 *     IR contract
 *         ->
 *     canonical grammar
 *         ->
 *     positive tests
 *         ->
 *     negative tests
 *         ->
 *     boundary tests
 *         ->
 *     scalability tests
 *         ->
 *     compatibility tests
 *
 * Examples of constructs that must NOT be added casually:
 *
 *     asyncBlock
 *     asyncClosure
 *     asyncLambda
 *     future
 *     promise
 *     executor
 *     scheduler
 *     worker
 *     thread
 *     taskScope
 *
 * Their existence in an implementation does not automatically make them
 * language syntax.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive parser-level integration tests MUST cover:
 *
 *     async function modifier
 *     await identifier
 *     await functionCall
 *     await memberCall
 *     await indexed computation
 *     await chained postfix computation
 *
 * Examples:
 *
 *     async fn compute() {
 *         await value
 *     }
 *
 *     async fn compute() {
 *         await work()
 *     }
 *
 *     async fn compute() {
 *         await service.request(value)
 *     }
 *
 *     async fn compute() {
 *         await stream.next()
 *     }
 *
 * Cross-domain semantic fixtures SHOULD include:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     accelerator computation
 *     distributed computation
 *     HDL/hardware-related computation
 *
 * The grammar tests syntax.
 *
 * Semantic tests determine whether those expressions are actually awaitable.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must not become valid merely because this adapter exists:
 *
 *     future
 *     Future
 *     Future<T>
 *     promise
 *     Promise<T>
 *     executor
 *     scheduler
 *     worker
 *     thread
 *     thread[8]
 *     worker[16]
 *     gpu[4]
 *     qpu[32]
 *
 * Those require independent language contracts.
 *
 * Malformed syntax must remain rejected:
 *
 *     async
 *     await
 *     await ;
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Boundary tests must verify:
 *
 *     await identifier
 *     await call()
 *     await member.call()
 *     await indexed.value
 *     await chained.call().result
 *
 * and verify that:
 *
 *     await computation() + value
 *
 * retains the canonical await precedence established by:
 *
 *     grammar/functions/async.g4
 *
 * This adapter must not alter expression precedence.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must verify that the grammar remains valid as source structure grows in:
 *
 *     async functions
 *     await operations
 *     nested computations
 *     call chains
 *     modules
 *     declarations
 *     tasks
 *     distributed computations
 *     quantum operations
 *     hardware descriptions
 *
 * No finite machine scale may be used as a grammar correctness criterion.
 *
 * A test environment may impose practical execution limits, but those limits
 * are test-resource limits rather than Zamani language limits.
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where AST/source serialization is supported:
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
 *     printer/serializer
 *       |
 *       v
 *     source
 *
 * must preserve:
 *
 *     async modifier
 *     await structure
 *     operand structure
 *     source meaning
 *
 * The adapter must not introduce information that cannot participate in the
 * canonical AST round trip.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Audit result:
 *
 *     PASS
 *
 * No machine/resource cardinality is encoded.
 *
 * No hardware identity is encoded.
 *
 * No device topology is encoded.
 *
 * No backend-specific executor is encoded.
 *
 * No quantum physical mapping is encoded.
 *
 * No fixed thread/core/worker count is encoded.
 *
 * No artificial finite concurrency limit is encoded.
 *
 * ============================================================================
 * INTEGRATION COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] asyncModifier has one canonical owner.
 * [x] awaitExpression has one canonical owner.
 * [x] asyncExpression has one canonical owner.
 * [x] This file only consumes those canonical rules.
 * [x] No duplicate async syntax is introduced.
 * [x] No duplicate await syntax is introduced.
 * [x] No duplicate task syntax is introduced.
 * [x] No future syntax is introduced.
 * [x] No parallel syntax is introduced.
 * [x] No cancellation syntax is introduced.
 * [x] No executor syntax is introduced.
 * [x] No scheduler syntax is introduced.
 * [x] No resource count is hard-coded.
 * [x] No hardware identity is hard-coded.
 * [x] No quantum physical resource is hard-coded.
 * [x] No new lexer token is required.
 * [x] Existing canonical AST remains authoritative.
 * [x] Existing Expression::Await remains authoritative.
 * [x] Existing Expression::Async remains authoritative.
 * [x] quantum::ir remains the canonical quantum semantic boundary.
 * [x] Safe Rust remains the implementation requirement.
 * [x] Rust 1.97/1.97.1 compatibility remains the implementation baseline.
 * [x] Parser behavior remains deterministic.
 *
 * Integration validation additionally requires:
 *
 * [ ] The canonical parser imports/consumes AsyncConcurrency.
 * [ ] The canonical expression hierarchy consumes the appropriate canonical
 *     async rule rather than this adapter as a replacement owner.
 * [ ] functions/functions.g4 consumes AsyncFunctions::asyncModifier.
 * [ ] expressions/ consumes AsyncFunctions::asyncExpression.
 * [ ] tasks.g4 does not remain an independent owner of awaitExpression.
 *
 * Those final repository-composition checks belong to their owning files and
 * must not be solved by duplicating syntax here.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Async syntax is a language-level semantic construct.
 *
 * Concurrency realization is a downstream implementation concern.
 *
 * Therefore:
 *
 *     async syntax
 *         !=
 *     thread creation
 *
 *     await syntax
 *         !=
 *     OS blocking
 *
 *     concurrency
 *         !=
 *     fixed hardware parallelism
 *
 *     parallelism
 *         !=
 *     fixed worker count
 *
 *     task
 *         !=
 *     thread
 *
 *     async computation
 *         !=
 *     particular executor
 *
 * The portable architecture remains:
 *
 *     Program
 *          ->
 *     Parse
 *          ->
 *     Domain-neutral AST
 *          ->
 *     Semantic analysis
 *          ->
 *     Canonical IR
 *          ->
 *     Resource/capability analysis
 *          ->
 *     Optimization
 *          ->
 *     Scheduling
 *          ->
 *     Placement/routing
 *          ->
 *     Runtime/backend realization
 *
 * This is the concurrency-level application of:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */

parser grammar AsyncConcurrency;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * CANONICAL ASYNC IMPORT
 * ============================================================================
 *
 * AsyncFunctions owns:
 *
 *     asyncModifier
 *     awaitExpression
 *     asyncExpression
 *
 * This grammar only consumes those rules.
 *
 * The import therefore establishes dependency without creating duplicate
 * ownership.
 */
import AsyncFunctions;


/*
 * ============================================================================
 * 1. ASYNC CONCURRENCY MODIFIER
 * ============================================================================
 *
 * Adapter around the canonical async modifier.
 *
 * DO NOT add ASYNC directly here.
 *
 * The lexical/parser ownership remains:
 *
 *     AsyncFunctions.asyncModifier
 */
asyncConcurrencyModifier
    : asyncModifier
    ;


/*
 * ============================================================================
 * 2. ASYNC CONCURRENCY AWAIT
 * ============================================================================
 *
 * Adapter around the canonical await expression.
 *
 * The actual await syntax remains owned by:
 *
 *     AsyncFunctions.awaitExpression
 */
asyncConcurrencyAwait
    : awaitExpression
    ;


/*
 * ============================================================================
 * 3. ASYNC CONCURRENCY EXPRESSION
 * ============================================================================
 *
 * Stable concurrency-domain expression boundary.
 *
 * The actual asynchronous expression remains owned by:
 *
 *     AsyncFunctions.asyncExpression
 */
asyncConcurrencyExpression
    : asyncExpression
    ;


/*
 * ============================================================================
 * 4. ASYNC CONCURRENCY CONSTRUCT
 * ============================================================================
 *
 * Single stable adapter for consumers that need to identify either:
 *
 *     async modifier
 *     async expression
 *
 * without taking ownership of either construct.
 *
 * This rule is intentionally structural.
 *
 * It creates no new semantics.
 */
asyncConcurrencyConstruct
    : asyncConcurrencyModifier
    | asyncConcurrencyExpression
    ;


/*
 * ============================================================================
 * 5. ASYNC CONCURRENCY AWAIT CONSTRUCT
 * ============================================================================
 *
 * Explicit await-facing adapter for tooling and parser composition.
 */
asyncConcurrencyAwaitConstruct
    : asyncConcurrencyAwait
    ;


/*
 * ============================================================================
 * 6. ASYNC CONCURRENCY ROOT
 * ============================================================================
 *
 * Stable parser-facing entry point for the concurrency/async domain.
 *
 * This rule deliberately does not include:
 *
 *     spawn
 *     parallel
 *     task
 *     future
 *     channel
 *     actor
 *     cancellation
 *
 * Those remain owned by their respective concurrency grammar files.
 */
asyncConcurrency
    : asyncConcurrencyConstruct
    | asyncConcurrencyAwaitConstruct
    ;


/*
 * ============================================================================
 * 7. ASYNC CONCURRENCY EXPRESSION ROOT
 * ============================================================================
 *
 * Expression-only integration point.
 */
asyncConcurrencyExpressionRoot
    : asyncConcurrencyExpression
    ;


/*
 * ============================================================================
 * 8. ASYNC CONCURRENCY MODIFIER ROOT
 * ============================================================================
 *
 * Function-declaration integration point.
 *
 * The complete function declaration remains owned by functions/functions.g4.
 */
asyncConcurrencyModifierRoot
    : asyncConcurrencyModifier
    ;


/*
 * ============================================================================
 * 9. ASYNC CONCURRENCY AWAIT ROOT
 * ============================================================================
 *
 * Await-only integration point.
 */
asyncConcurrencyAwaitRoot
    : asyncConcurrencyAwait
    ;


/*
 * ============================================================================
 * 10. SEMANTIC BOUNDARY MARKER
 * ============================================================================
 *
 * This rule exists solely as a stable parser composition boundary.
 *
 * It intentionally delegates completely to canonical async syntax.
 */
asyncConcurrencySemanticBoundary
    : asyncConcurrency
    ;