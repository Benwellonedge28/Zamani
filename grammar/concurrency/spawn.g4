/*
 * ============================================================================
 * Zamani Universal Computing Language
 * Production Spawn-Concurrency Grammar
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/concurrency/spawn.g4
 *
 * GRAMMAR
 * -------
 * Spawn
 *
 * STATUS
 * ------
 * CANONICAL
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the single parser-level syntax authority for Zamani's
 * `spawn` construct.
 *
 * `spawn` expresses the creation/submission of a concurrent computation.
 *
 * It does NOT specify how that computation is executed.
 *
 * The same source construct may ultimately be realized as:
 *
 *     - a coroutine;
 *     - a cooperative task;
 *     - a scheduled task;
 *     - a thread;
 *     - an event-loop operation;
 *     - a GPU/accelerator operation;
 *     - an FPGA/ASIC operation;
 *     - a quantum-classical computation;
 *     - a distributed computation;
 *     - a remote computation;
 *     - another future execution substrate.
 *
 * The choice is made downstream of parsing.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Zamani frontend/compiler:
 *
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Safety:
 *
 *     This grammar contains no Rust implementation code.
 *     No unsafe Rust is required by this grammar.
 *     The Zamani compiler/runtime implementation remains safe Rust.
 *
 * This file contains ANTLR grammar only.
 *
 * It contains no:
 *
 *     - Rust actions;
 *     - semantic predicates;
 *     - filesystem access;
 *     - network access;
 *     - hardware discovery;
 *     - runtime execution;
 *     - scheduler execution;
 *     - executor invocation;
 *     - resource discovery;
 *     - target selection;
 *     - IR construction.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file is the ONLY grammar file that owns:
 *
 *     spawnExpression
 *     spawnBody
 *     spawnStatement
 *
 * Other grammar files may consume these rules.
 *
 * In particular:
 *
 *     grammar/concurrency/tasks.g4
 *     grammar/concurrency/concurrency.g4
 *
 * MUST NOT redefine the `spawn` productions.
 *
 * They may expose adapter/composition rules that reference them.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - spawnExpression;
 *     - spawnBody;
 *     - spawnStatement;
 *     - spawn syntax integration boundaries;
 *     - the syntactic distinction between spawning a computation and ordinary
 *       expression evaluation.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - task scheduling;
 *     - executors;
 *     - worker creation;
 *     - thread creation;
 *     - coroutine implementation;
 *     - futures;
 *     - promises;
 *     - channels;
 *     - actors;
 *     - parallel scopes;
 *     - synchronization;
 *     - cancellation;
 *     - resource allocation;
 *     - resource discovery;
 *     - target selection;
 *     - placement;
 *     - routing;
 *     - topology;
 *     - hardware;
 *     - GPU execution;
 *     - FPGA execution;
 *     - QPU execution;
 *     - quantum routing;
 *     - quantum scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime implementation;
 *     - canonical IR construction.
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
 * Canonical spawn token:
 *
 *     SPAWN
 *
 * Canonical ordinary expression hierarchy:
 *
 *     grammar/expressions/
 *
 * Canonical postfix/call/member/index syntax:
 *
 *     grammar/expressions/postfix.g4
 *
 * Canonical task composition:
 *
 *     grammar/concurrency/tasks.g4
 *
 * Canonical concurrency composition:
 *
 *     grammar/concurrency/concurrency.g4
 *
 * Canonical async syntax:
 *
 *     grammar/functions/async.g4
 *
 * Canonical future syntax:
 *
 *     grammar/concurrency/futures.g4
 *
 * Canonical frontend parser:
 *
 *     src/parser.rs
 *
 * Domain-neutral frontend AST:
 *
 *     src/ast/
 *
 * Existing AST spawn representation:
 *
 *     Expression::Spawn
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar uses the canonical Zamani expression and block-expression
 * rules.
 *
 * Expected upstream parser rules:
 *
 *     expression
 *     blockExpression
 *
 * Those rules are owned by the general expression/core grammar.
 *
 * This file does NOT redefine:
 *
 *     expression
 *     blockExpression
 *
 * This dependency direction is intentional:
 *
 *     core/expression syntax
 *              |
 *              v
 *         spawn.g4
 *              |
 *              v
 *       concurrency adapters
 *
 * `spawn.g4` MUST NOT import the concurrency composition root.
 *
 * It MUST NOT import:
 *
 *     concurrency.g4
 *     tasks.g4
 *     futures.g4
 *     parallel.g4
 *
 * merely to define `spawn`.
 *
 * This prevents circular dependencies.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The lexer owns recognition of:
 *
 *     SPAWN
 *
 * The parser owns recognition of:
 *
 *     SPAWN expression
 *
 * The lexer MUST NOT encode spawn-specific semantic behavior.
 *
 * The parser MUST NOT create a new token for every possible spawned
 * computation type.
 *
 * For example, this grammar intentionally does NOT require tokens such as:
 *
 *     SPAWN_TASK
 *     SPAWN_THREAD
 *     SPAWN_GPU
 *     SPAWN_QPU
 *     SPAWN_REMOTE
 *     SPAWN_ACTOR
 *
 * Such specialization would make the language target-dependent and would
 * violate the POCO-REAF architecture.
 *
 * ============================================================================
 * CORE SYNTAX
 * ============================================================================
 *
 * Canonical forms:
 *
 *     spawn expression
 *
 *     spawn {
 *         ...
 *     }
 *
 * The operand remains a normal Zamani expression or block expression.
 *
 * Examples:
 *
 *     spawn compute()
 *
 *     spawn worker()
 *
 *     spawn {
 *         compute()
 *     }
 *
 *     spawn async_compute()
 *
 *     spawn quantum_operation()
 *
 *     spawn remote_operation()
 *
 * The grammar deliberately does not distinguish those domains.
 *
 * ============================================================================
 * SPAWN EXPRESSION
 * ============================================================================
 *
 * `spawnExpression` is the canonical expression-level representation.
 *
 * It consists of:
 *
 *     SPAWN + spawnBody
 *
 * The body is either:
 *
 *     - a canonical block expression;
 *     - a canonical expression.
 *
 * The operand's semantic validity is checked later.
 *
 * ============================================================================
 */

parser grammar Spawn;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. CANONICAL SPAWN EXPRESSION
 * ============================================================================
 *
 * Public expression-level entry point for spawn.
 *
 * Examples:
 *
 *     spawn compute()
 *
 *     spawn value
 *
 *     spawn async_compute()
 *
 *     spawn quantum_work()
 *
 *     spawn {
 *         compute()
 *     }
 *
 * The grammar deliberately accepts an existing Zamani expression rather than
 * enumerating callable/task/future/device/quantum forms.
 */
spawnExpression
    : SPAWN spawnBody
    ;


/*
 * ============================================================================
 * 2. SPAWN BODY
 * ============================================================================
 *
 * A spawn body is either:
 *
 *     - an ordinary Zamani expression;
 *     - a canonical block expression.
 *
 * No new block syntax is introduced here.
 *
 * Block ownership remains with the core/expression grammar.
 *
 * This permits future expression domains to become spawnable without editing
 * this grammar merely because a new computation domain is introduced.
 */
spawnBody
    : blockExpression
    | expression
    ;


/*
 * ============================================================================
 * 3. SPAWN STATEMENT
 * ============================================================================
 *
 * Statement-level adapter for canonical Zamani statement composition.
 *
 * The semicolon is optional here because the repository's current concurrency
 * grammar uses optional semicolon adapters for direct concurrency constructs.
 *
 * The canonical statement grammar remains responsible for determining the
 * final statement composition and formatting policy.
 *
 * Examples:
 *
 *     spawn compute();
 *
 *     spawn compute()
 *
 *     spawn {
 *         compute()
 *     }
 */
spawnStatement
    : spawnExpression SEMI?
    ;


/*
 * ============================================================================
 * 4. SPAWNABLE COMPUTATION BOUNDARY
 * ============================================================================
 *
 * This named rule exists as a semantic integration boundary.
 *
 * It intentionally does not enumerate:
 *
 *     function calls
 *     closures
 *     async functions
 *     futures
 *     tasks
 *     actors
 *     GPU kernels
 *     FPGA operations
 *     QPU jobs
 *     distributed services
 *     network requests
 *
 * All of those may be represented by ordinary Zamani expressions.
 *
 * Semantic analysis determines whether a particular expression is legally
 * spawnable.
 */
spawnableComputation
    : expression
    | blockExpression
    ;


/*
 * ============================================================================
 * 5. SPAWN ROOT
 * ============================================================================
 *
 * Stable composition rule for parser modules that need a single spawn entry
 * point.
 *
 * Other grammar files SHOULD consume `spawnExpression` or `spawnStatement`
 * directly when they need the exact syntactic category.
 *
 * `spawn` exists as a stable aggregate boundary for tooling and validation.
 */
spawn
    : spawnExpression
    ;


/*
 * ============================================================================
 * 6. SPAWN STATEMENT ROOT
 * ============================================================================
 *
 * Stable statement-level composition boundary.
 */
spawnStatementRoot
    : spawnStatement
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does NOT introduce a Spawn-specific AST.
 *
 * The repository already has:
 *
 *     Expression::Spawn(Span, Box<Expression>)
 *
 * in the domain-neutral frontend AST.
 *
 * Therefore the intended lowering is:
 *
 *     spawnExpression
 *          |
 *          v
 *     Expression::Spawn
 *
 * The operand is lowered through the ordinary expression AST pipeline.
 *
 * Conceptually:
 *
 *     spawn compute()
 *
 * becomes:
 *
 *     Expression::Spawn(
 *         span,
 *         Expression::Call(...)
 *     )
 *
 * The exact AST construction remains owned by the parser/frontend
 * implementation.
 *
 * This grammar must not introduce:
 *
 *     SpawnTask
 *     SpawnThread
 *     SpawnGpu
 *     SpawnQpu
 *     SpawnRemote
 *     SpawnHardware
 *
 * AST variants.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The resulting AST node must preserve the source span covering the complete
 * spawn construct.
 *
 * For:
 *
 *     spawn compute()
 *
 * the span must be sufficient for diagnostics, formatting, IDE tooling,
 * source mapping, and later semantic analysis.
 *
 * This grammar itself does not manufacture source spans.
 *
 * The parser/AST layer owns span construction.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only:
 *
 *     this is a spawn expression.
 *
 * Semantic analysis must determine:
 *
 *     - whether the operand is spawnable;
 *     - the resulting type;
 *     - whether the spawned computation is awaitable;
 *     - ownership/borrowing requirements;
 *     - effect requirements;
 *     - resource requirements;
 *     - capability requirements;
 *     - lifetime requirements;
 *     - cancellation semantics;
 *     - failure propagation;
 *     - synchronization requirements;
 *     - whether execution may be local or distributed;
 *     - whether execution may use an accelerator;
 *     - whether execution interacts with quantum computation;
 *     - whether execution crosses a hardware boundary.
 *
 * None of these are parser decisions.
 *
 * ============================================================================
 * SPAWN DOES NOT MEAN THREAD CREATION
 * ============================================================================
 *
 * The language meaning MUST NOT be defined as:
 *
 *     spawn == create a thread
 *
 * It means:
 *
 *     spawn == establish concurrent-computation intent
 *
 * The implementation may realize that intent using any valid execution model.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A source program containing:
 *
 *     spawn compute()
 *
 * does not specify:
 *
 *     - CPU;
 *     - core;
 *     - thread;
 *     - worker;
 *     - GPU;
 *     - FPGA;
 *     - ASIC;
 *     - QPU;
 *     - accelerator;
 *     - node;
 *     - process;
 *     - executor;
 *     - queue;
 *     - event loop;
 *     - physical device;
 *     - hardware address.
 *
 * Therefore the same source construct remains valid across:
 *
 *     tiny systems
 *     embedded systems
 *     single-core systems
 *     multicore systems
 *     heterogeneous systems
 *     accelerator systems
 *     quantum-classical systems
 *     distributed systems
 *     clusters
 *     cloud systems
 *     future computational substrates.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * No finite source-language limit is imposed on:
 *
 *     - number of spawn expressions;
 *     - number of spawned computations;
 *     - nesting depth;
 *     - number of concurrent computations;
 *     - number of devices;
 *     - number of execution contexts;
 *     - number of nodes;
 *     - number of workers;
 *     - number of cores;
 *     - amount of memory;
 *     - amount of storage;
 *     - number of accelerators;
 *     - number of quantum resources.
 *
 * "Infinity" in the POCO-REAF requirement means:
 *
 *     the language grammar introduces no artificial machine-scale ceiling.
 *
 * Actual execution is necessarily bounded by resources available to the
 * compiler, runtime, scheduler, deployment environment, and target.
 *
 * Such limits are external resource constraints, not syntax limits.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT define or imply:
 *
 *     MAX_SPAWN
 *     MAX_TASKS
 *     MAX_CONCURRENT_TASKS
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
 *     MAX_SPAWN_DEPTH
 *
 * It must also never encode:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     fpga0
 *     node0
 *     thread0
 *     worker0
 *
 * as universal source-language execution targets.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Spawn syntax describes computation intent.
 *
 * It does not describe the resources used to realize that computation.
 *
 * These concepts remain separate:
 *
 *     semantic requirement
 *     capability requirement
 *     constraint
 *     preference
 *     hint
 *     implementation decision
 *
 * For example:
 *
 *     spawn compute()
 *
 * may later require a capability such as:
 *
 *     capability("parallel.compute")
 *
 * but that capability belongs to the resource/capability model.
 *
 * It must not be encoded as:
 *
 *     spawn_on_gpu()
 *
 * merely to expose a particular implementation.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * `spawn` introduces concurrency-related semantic effects.
 *
 * Effect analysis, rather than parsing, determines:
 *
 *     - whether the operation is permitted in the current effect context;
 *     - whether it requires asynchronous execution;
 *     - whether synchronization is necessary;
 *     - whether ownership crosses an execution boundary;
 *     - whether the spawned operation may outlive the current scope;
 *     - whether cancellation/recovery semantics apply.
 *
 * The grammar does not encode these effect rules.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The type of a spawn expression is determined by semantic/type analysis.
 *
 * The grammar MUST NOT hard-code:
 *
 *     Task<T>
 *     Future<T>
 *     Promise<T>
 *
 * as the universal result type.
 *
 * Zamani's type system may evolve independently.
 *
 * The parser records the syntax.
 *
 * Semantic/type analysis determines the resulting computation abstraction.
 *
 * ============================================================================
 * MEMORY / OWNERSHIP CONTRACT
 * ============================================================================
 *
 * A spawned computation may create a new execution/lifetime boundary.
 *
 * Ownership and borrowing rules therefore belong to semantic analysis.
 *
 * The grammar must not attempt to decide whether:
 *
 *     a captured value is copied;
 *     a value is moved;
 *     a reference is borrowed;
 *     a value is shared;
 *     a value is transferred;
 *     a value is remotely serialized.
 *
 * Those decisions belong to the type, ownership, memory, and execution
 * semantics.
 *
 * ============================================================================
 * ASYNC INTEGRATION
 * ============================================================================
 *
 * Async syntax is owned by:
 *
 *     grammar/functions/async.g4
 *
 * Spawn MUST NOT redefine:
 *
 *     async
 *     await
 *     async function declarations.
 *
 * An async computation may be spawned because it is represented by an ordinary
 * Zamani expression.
 *
 * Example:
 *
 *     spawn async_compute()
 *
 * The semantic layer determines the relationship between:
 *
 *     spawn
 *     async
 *     await
 *
 * ============================================================================
 * AWAIT INTEGRATION
 * ============================================================================
 *
 * Await syntax has its own canonical ownership.
 *
 * A typical dependency relationship is:
 *
 *     spawn computation
 *          |
 *          v
 *     await result
 *
 * `spawn.g4` does not define `await`.
 *
 * `await` must therefore remain independently owned by the canonical await
 * grammar rather than being duplicated here.
 *
 * ============================================================================
 * TASK INTEGRATION
 * ============================================================================
 *
 * The current repository's `grammar/concurrency/tasks.g4` historically
 * contains:
 *
 *     taskSpawnExpression
 *     taskSpawnStatement
 *
 * Those rules duplicate the fundamental spawn syntax.
 *
 * Production ownership must be migrated to this file.
 *
 * After migration, `tasks.g4` should consume:
 *
 *     spawnExpression
 *     spawnStatement
 *
 * rather than redefine them.
 *
 * Recommended adapter:
 *
 *     taskSpawnExpression
 *         : spawnExpression
 *         ;
 *
 *     taskSpawnStatement
 *         : spawnStatement
 *         ;
 *
 * Alternatively, if the adapter names are no longer required by external
 * consumers, they should be removed after repository-wide reference analysis.
 *
 * The important invariant is:
 *
 *     one syntax owner
 *     many consumers
 *
 * ============================================================================
 * FUTURES INTEGRATION
 * ============================================================================
 *
 * A spawned computation may produce a future-like semantic value.
 *
 * Future syntax remains owned by:
 *
 *     grammar/concurrency/futures.g4
 *
 * This grammar must not introduce future syntax merely because spawn may
 * produce an asynchronous result.
 *
 * ============================================================================
 * PARALLELISM INTEGRATION
 * ============================================================================
 *
 * `spawn` establishes concurrent computation intent.
 *
 * `parallel` establishes a separate parallelism intent.
 *
 * These concepts must not be collapsed.
 *
 * For example:
 *
 *     spawn compute()
 *
 * does not mean:
 *
 *     execute physically in parallel immediately.
 *
 * Scheduling determines whether, where, and when the spawned computation is
 * actually executed.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A spawned computation may eventually execute remotely.
 *
 * This grammar does not encode:
 *
 *     node identifiers;
 *     endpoint identifiers;
 *     network addresses;
 *     topology;
 *     transport protocols;
 *     machine identities.
 *
 * Distributed semantics belong to:
 *
 *     grammar/distributed/
 *     grammar/networking/
 *     grammar/resources/
 *     grammar/hardware/
 *
 * as appropriate.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A spawned expression may semantically invoke quantum computation.
 *
 * Example:
 *
 *     spawn quantum_work()
 *
 * or a computation that eventually lowers to quantum operations.
 *
 * This grammar does not define:
 *
 *     quantum operations;
 *     physical qubits;
 *     QPU identifiers;
 *     quantum routing;
 *     quantum scheduling;
 *     QEC;
 *     ZQN;
 *     calibration.
 *
 * If the spawned computation contains quantum semantics, the established
 * pipeline remains:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * `spawn.g4` creates no alternative quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Spawn may surround or invoke computations whose semantic target is:
 *
 *     HDL simulation;
 *     hardware acceleration;
 *     accelerator execution;
 *     hardware/software co-design.
 *
 * This file remains target-independent.
 *
 * It does not define:
 *
 *     device IDs;
 *     physical ports;
 *     clock resources;
 *     FPGA coordinates;
 *     ASIC regions;
 *     accelerator counts.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Spawn can apply to:
 *
 *     model inference;
 *     training work;
 *     tensor computation;
 *     data processing;
 *     streaming;
 *     pipeline stages;
 *     distributed learning.
 *
 * No AI framework or data-processing framework becomes a grammar dependency.
 *
 * ============================================================================
 * CANONICAL LOWERING
 * ============================================================================
 *
 * The intended architectural flow is:
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
 *     Expression::Spawn
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +-------------------------------+
 *       |                               |
 *       v                               v
 *     concurrency semantics       resource/capability analysis
 *       |                               |
 *       +---------------+---------------+
 *                       |
 *                       v
 *                 canonical program IR
 *                       |
 *             +---------+---------+
 *             |         |         |
 *             v         v         v
 *          classical quantum    HDL/hardware
 *                       |
 *                       v
 *                  optimization
 *                       |
 *                       v
 *             routing / placement
 *                       |
 *                       v
 *                   scheduling
 *                       |
 *                       v
 *                    runtime
 *
 * No SpawnIR is created by this grammar.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler must consume the semantic representation generated from
 * `Expression::Spawn`.
 *
 * The compiler must not depend directly on this `.g4` file.
 *
 * Correct dependency direction:
 *
 *     grammar
 *       ->
 *     AST
 *       ->
 *     semantic model
 *       ->
 *     IR
 *       ->
 *     compiler
 *       ->
 *     runtime
 *
 * Forbidden:
 *
 *     compiler -> grammar
 *     runtime -> grammar
 *     scheduler -> grammar
 *     hardware -> grammar
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime implementation may choose any valid realization.
 *
 * Examples include:
 *
 *     cooperative tasks
 *     coroutines
 *     worker pools
 *     event loops
 *     operating-system threads
 *     accelerator queues
 *     distributed workers
 *     remote execution
 *
 * The source syntax remains unchanged.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing of a given source program must not depend on:
 *
 *     - runtime scheduling;
 *     - hardware discovery;
 *     - available worker count;
 *     - device availability;
 *     - network state;
 *     - timing;
 *     - hash iteration order.
 *
 * The parse structure must be deterministic for a fixed language/grammar
 * version and lexer configuration.
 *
 * Runtime scheduling may remain nondeterministic where the language semantics
 * permit it.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to the parser layer.
 *
 * Examples:
 *
 *     spawn
 *
 *     spawn {
 *         ...
 *     // missing closing delimiter
 *
 * Semantic errors belong to semantic analysis.
 *
 * Examples:
 *
 *     spawn non_spawnable_value
 *
 * when the type/effect system determines that the operand cannot be spawned.
 *
 * Resource failures belong to resource analysis or runtime.
 *
 * Hardware failures belong to the hardware/runtime/resilience layers.
 *
 * These categories must not be collapsed into parser errors.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * `spawn` must not bypass:
 *
 *     - capability checks;
 *     - authorization;
 *     - isolation;
 *     - ownership;
 *     - resource policies;
 *     - execution boundaries;
 *     - security policy.
 *
 * The grammar provides syntax only.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing valid spawn syntax must retain its meaning unless an explicit
 * language-version migration changes it.
 *
 * In particular, migration from:
 *
 *     tasks.g4::taskSpawnExpression
 *
 * to:
 *
 *     spawn.g4::spawnExpression
 *
 * is an ownership migration, not a source-language semantic change.
 *
 * Existing source forms:
 *
 *     spawn expression
 *     spawn block
 *
 * remain source-compatible.
 *
 * ============================================================================
 * VALIDATION CONTRACT
 * ============================================================================
 *
 * This file must be validated for:
 *
 *     - duplicate rule names;
 *     - ambiguous alternatives;
 *     - unreachable alternatives;
 *     - undefined parser rules;
 *     - undefined lexer tokens;
 *     - import cycles;
 *     - left recursion;
 *     - precedence conflicts;
 *     - source-span preservation;
 *     - AST coverage;
 *     - semantic coverage;
 *     - hard-coded machine limits.
 *
 * ============================================================================
 * REQUIRED TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     spawn compute()
 *     spawn value
 *     spawn {
 *         compute()
 *     }
 *     spawn async_compute()
 *     spawn quantum_work()
 *     spawn remote_work()
 *
 * Negative:
 *
 *     spawn
 *
 *     spawn {
 *         ...
 *     // malformed delimiter
 *
 *     spawn ;
 *
 *     spawn )
 *
 * Boundary:
 *
 *     deeply nested spawn expressions;
 *     large spawn dependency structures;
 *     many syntactically independent spawn expressions;
 *     nested blocks;
 *     nested expressions.
 *
 * Cross-domain:
 *
 *     classical + spawn
 *     quantum + spawn
 *     hybrid + spawn
 *     HDL + spawn
 *     hardware + spawn
 *     distributed + spawn
 *     AI + spawn
 *     data + spawn
 *     networking + spawn
 *
 * Scalability:
 *
 *     no test establishes a maximum number of spawn operations.
 *
 * Determinism:
 *
 *     repeated parsing of identical source produces equivalent syntax/AST.
 *
 * Round-trip:
 *
 *     parse -> AST -> format -> parse
 *
 * must preserve spawn semantics where formatter support exists.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     - no machine count;
 *     - no processor count;
 *     - no thread count;
 *     - no worker count;
 *     - no GPU count;
 *     - no FPGA count;
 *     - no QPU count;
 *     - no node count;
 *     - no memory capacity;
 *     - no topology;
 *     - no device identifier;
 *     - no physical address;
 *     - no execution-unit identifier.
 *
 * Numeric values inside expressions remain ordinary program semantics.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The spawn grammar must remain simple enough that parser complexity is driven
 * by source structure rather than target-resource cardinality.
 *
 * It must not perform:
 *
 *     resource discovery;
 *     scheduling;
 *     semantic type inference;
 *     hardware probing;
 *     IR optimization.
 *
 * This keeps parsing independent of machine scale.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * `spawn.g4` is complete when:
 *
 *     [x] one canonical spawn syntax owner exists;
 *     [x] SPAWN comes from the canonical lexer;
 *     [x] ordinary expression syntax is reused;
 *     [x] block syntax is reused;
 *     [x] no runtime semantics are embedded;
 *     [x] no hardware assumptions are embedded;
 *     [x] no finite concurrency limits are embedded;
 *     [x] AST mapping to Expression::Spawn is defined;
 *     [x] semantic ownership is defined;
 *     [x] compiler boundary is defined;
 *     [x] runtime boundary is defined;
 *     [x] quantum integration is defined;
 *     [x] HDL/hardware integration is defined;
 *     [x] distributed integration is defined;
 *     [x] resource/capability separation is defined;
 *     [x] compatibility migration from tasks.g4 is defined;
 *     [x] validation requirements are defined;
 *     [x] positive/negative/boundary/scalability tests are defined;
 *     [x] hard-coding audit is defined.
 *
 * The repository-level integration is complete only after `tasks.g4`,
 * `concurrency.g4`, and the canonical parser composition stop independently
 * defining the same spawn syntax.
 *
 * ============================================================================
 */