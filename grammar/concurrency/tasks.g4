/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/concurrency/tasks.g4
 *
 * STATUS
 * ------
 * PRODUCTION TASK-CONCURRENCY INTEGRATION GRAMMAR
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021 edition
 * Safe Rust only.
 *
 * This file contains ANTLR grammar only.
 *
 * It contains:
 *
 *     - no Rust actions;
 *     - no embedded Rust;
 *     - no unsafe code;
 *     - no semantic predicates;
 *     - no runtime execution;
 *     - no scheduler execution;
 *     - no hardware discovery;
 *     - no resource discovery;
 *     - no target selection.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the TASK-DOMAIN INTEGRATION GRAMMAR.
 *
 * It provides stable task-oriented parser entry points while delegating the
 * actual expression-level asynchronous syntax to its canonical owner:
 *
 *     grammar/expressions/async.g4
 *
 * Therefore this file does NOT redefine:
 *
 *     awaitExpression
 *     spawnExpression
 *     parallelExpression
 *
 * Those rules belong to:
 *
 *     grammar/expressions/async.g4
 *
 * whose grammar name is:
 *
 *     AsyncExpressions
 *
 * This separation is intentional.
 *
 * The architecture is:
 *
 *     lexer
 *       |
 *       v
 *     canonical expressions
 *       |
 *       v
 *     expressions/async.g4
 *       |
 *       +------------------+
 *       |                  |
 *       v                  v
 *     await              spawn/parallel
 *       |                  |
 *       +---------+--------+
 *                 |
 *                 v
 *          concurrency/tasks.g4
 *                 |
 *                 v
 *        concurrency composition
 *                 |
 *                 v
 *          statement composition
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Exactly one grammar owns each fundamental asynchronous expression.
 *
 * Canonical ownership:
 *
 *     awaitExpression
 *         -> grammar/expressions/async.g4
 *
 *     spawnExpression
 *         -> grammar/expressions/async.g4
 *
 *     parallelExpression
 *         -> grammar/expressions/async.g4
 *
 * Task-domain ownership:
 *
 *     taskExpression
 *         -> this file
 *
 *     taskConcurrencyExpression
 *         -> this file
 *
 *     taskSpawnExpression
 *         -> this file, as an adapter
 *
 *     taskAwaitExpression
 *         -> this file, as an adapter
 *
 *     taskParallelExpression
 *         -> this file, as an adapter
 *
 *     taskStatement
 *         -> this file, as a task-domain statement adapter
 *
 *     taskSpawnStatement
 *         -> this file
 *
 *     taskAwaitStatement
 *         -> this file
 *
 *     taskParallelStatement
 *         -> this file
 *
 * Adapter rules MUST NOT redefine the underlying syntax.
 *
 * ============================================================================
 * WHY THIS FILE MUST BE AN ADAPTER
 * ============================================================================
 *
 * The previous architecture duplicated asynchronous syntax in multiple places.
 *
 * That creates problems such as:
 *
 *     awaitExpression
 *         in expressions/async.g4
 *
 *     awaitExpression
 *         in functions/async.g4
 *
 *     awaitTaskExpression
 *         in tasks.g4
 *
 *     futureAwaitExpression
 *         in futures.g4
 *
 * Such duplication creates multiple grammar authorities.
 *
 * A future syntax change would then require synchronized modifications across
 * unrelated grammar files.
 *
 * The production architecture instead uses:
 *
 *     one canonical syntax owner
 *     +
 *     domain-specific adapters
 *
 * This file is therefore intentionally smaller than a standalone task
 * language.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - task-domain composition;
 *     - task-specific parser entry points;
 *     - task expression adapters;
 *     - task statement adapters;
 *     - task-oriented integration with concurrency.g4;
 *     - task-oriented integration with statements/concurrency.g4;
 *     - stable task grammar names used by existing consumers;
 *     - task-domain documentation and integration contracts.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - keywords;
 *     - ordinary expressions;
 *     - expression precedence;
 *     - postfix expressions;
 *     - blocks;
 *     - await syntax;
 *     - spawn syntax;
 *     - parallel syntax;
 *     - async function declarations;
 *     - futures as runtime objects;
 *     - promises;
 *     - executors;
 *     - schedulers;
 *     - worker pools;
 *     - threads;
 *     - CPUs;
 *     - GPUs;
 *     - FPGAs;
 *     - QPUs;
 *     - accelerators;
 *     - node topology;
 *     - network topology;
 *     - placement;
 *     - routing;
 *     - resource allocation;
 *     - hardware discovery;
 *     - quantum IR;
 *     - classical IR;
 *     - HDL IR;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime implementation.
 *
 * ============================================================================
 * CANONICAL OWNERS
 * ============================================================================
 *
 * Lexical vocabulary
 *     -> grammar/antlr/ZamaniLexer.g4
 *
 * General expression hierarchy
 *     -> grammar/expressions/
 *
 * Asynchronous expression syntax
 *     -> grammar/expressions/async.g4
 *
 * Async function declaration syntax
 *     -> grammar/functions/async.g4
 *
 * Task-domain composition
 *     -> this file
 *
 * Concurrency-domain composition
 *     -> grammar/concurrency/concurrency.g4
 *
 * Statement-layer concurrency integration
 *     -> grammar/statements/concurrency.g4
 *
 * Future integration
 *     -> grammar/concurrency/futures.g4
 *
 * Parallelism-specific grammar
 *     -> grammar/concurrency/parallel.g4
 *        grammar/concurrency/task-parallel.g4
 *        grammar/concurrency/data-parallel.g4
 *
 * Semantic task validity
 *     -> semantic/type/effect/ownership/resource analysis
 *
 * Canonical semantic representation
 *     -> canonical frontend/IR architecture
 *
 * Scheduling
 *     -> scheduler
 *
 * Runtime execution
 *     -> runtime
 *
 * Hardware capabilities
 *     -> hardware/HAL
 *
 * Quantum semantic representation
 *     -> quantum::ir
 *
 * Quantum resilience/error correction
 *     -> QEC / resilience / ZQN
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar imports:
 *
 *     AsyncExpressions
 *
 * because that grammar owns:
 *
 *     asyncExpression
 *     awaitExpression
 *     spawnExpression
 *     parallelExpression
 *
 * The canonical parser composition must also provide:
 *
 *     expression
 *     blockExpression
 *
 * through the normal Zamani expression/block grammar composition.
 *
 * This file MUST NOT create a second definition of those rules.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer owns all lexical spellings.
 *
 * Relevant existing vocabulary includes:
 *
 *     ASYNC
 *     AWAIT
 *     SPAWN
 *     PARALLEL
 *
 * This grammar consumes those tokens indirectly through the canonical async
 * expression grammar.
 *
 * It MUST NOT introduce:
 *
 *     TASK
 *     TASK_SCOPE
 *     TASK_GROUP
 *     WORKER
 *     THREAD
 *     EXECUTOR
 *     JOIN
 *     CANCEL
 *     ACTOR
 *     CHANNEL
 *
 * merely because those concepts may exist semantically.
 *
 * A new keyword requires the normal language evolution process:
 *
 *     specification
 *         ->
 *     lexical contract
 *         ->
 *     canonical lexer
 *         ->
 *     grammar
 *         ->
 *     AST
 *         ->
 *     semantics
 *         ->
 *     IR
 *         ->
 *     compiler/runtime
 *         ->
 *     conformance tests
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Task syntax expresses COMPUTATIONAL INTENT.
 *
 * It MUST NOT encode the physical realization of that intent.
 *
 * The same task-oriented source program may be realized on:
 *
 *     - a tiny embedded system;
 *     - a single-core CPU;
 *     - a multicore CPU;
 *     - a many-core system;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - an accelerator;
 *     - a quantum/classical hybrid system;
 *     - a quantum simulator;
 *     - a distributed system;
 *     - a cluster;
 *     - a cloud deployment;
 *     - a future computational substrate.
 *
 * No language-level ceiling is imposed by this grammar.
 *
 * In particular, this grammar MUST NOT encode:
 *
 *     MAX_TASKS
 *     MAX_CONCURRENT_TASKS
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CHANNELS
 *     MAX_QUEUES
 *     MAX_MEMORY
 *     MAX_PARALLELISM
 *     MAX_TASK_DEPTH
 *     MAX_TASK_GROUPS
 *
 * Resource availability is a downstream concern.
 *
 * The compiler/runtime may realize logical concurrency through:
 *
 *     - serialization;
 *     - cooperative scheduling;
 *     - batching;
 *     - streaming;
 *     - task parallelism;
 *     - data parallelism;
 *     - accelerator execution;
 *     - distributed execution;
 *     - heterogeneous execution.
 *
 * Such realization must preserve the program's defined semantics.
 *
 * ============================================================================
 * SEMANTIC PRINCIPLE
 * ============================================================================
 *
 * A task is NOT synonymous with:
 *
 *     thread
 *     core
 *     process
 *     GPU invocation
 *     FPGA execution unit
 *     QPU execution
 *     distributed node
 *     hardware queue
 *
 * Those are possible implementation mechanisms.
 *
 * The language describes logical computation and dependencies.
 *
 * ============================================================================
 * TASK CREATION
 * ============================================================================
 *
 * `spawnExpression` is defined by the canonical async expression grammar.
 *
 * This file exposes it through:
 *
 *     taskSpawnExpression
 *
 * The adapter does not change its meaning.
 *
 * Conceptually:
 *
 *     spawn computation()
 *
 * means:
 *
 *     create/submit a logical asynchronous computation
 *
 * and NOT:
 *
 *     create one OS thread
 *
 * or:
 *
 *     allocate one hardware execution unit.
 *
 * ============================================================================
 * TASK WAITING
 * ============================================================================
 *
 * `awaitExpression` is defined by the canonical async expression grammar.
 *
 * This file exposes it through:
 *
 *     taskAwaitExpression
 *
 * Whether the operand is actually awaitable is determined by semantic/type
 * analysis.
 *
 * This grammar does NOT introduce a closed runtime type such as:
 *
 *     Future<T>
 *     Promise<T>
 *     Task<T>
 *     JoinHandle<T>
 *
 * A semantic or library layer may provide such types without changing this
 * grammar.
 *
 * ============================================================================
 * TASK PARALLELISM
 * ============================================================================
 *
 * `parallelExpression` is defined by:
 *
 *     grammar/expressions/async.g4
 *
 * This file exposes it through:
 *
 *     taskParallelExpression
 *
 * The `parallel` keyword means that the computation may participate in
 * concurrent execution subject to semantic dependencies and constraints.
 *
 * It does NOT guarantee:
 *
 *     simultaneous execution;
 *     one worker per operation;
 *     one thread per operation;
 *     one core per operation;
 *     distributed execution;
 *     accelerator execution.
 *
 * ============================================================================
 * TASK EXPRESSION COMPOSITION
 * ============================================================================
 *
 * The public task expression boundary is:
 *
 *     taskExpression
 *
 * It delegates to:
 *
 *     taskConcurrencyExpression
 *
 * which delegates to the canonical async expression constructs.
 *
 * Therefore:
 *
 *     taskExpression
 *         |
 *         +--> taskSpawnExpression
 *         |
 *         +--> taskAwaitExpression
 *         |
 *         +--> taskParallelExpression
 *
 * and:
 *
 *     taskSpawnExpression
 *         -> spawnExpression
 *
 *     taskAwaitExpression
 *         -> awaitExpression
 *
 *     taskParallelExpression
 *         -> parallelExpression
 *
 * ============================================================================
 * TASK STATEMENT INTEGRATION
 * ============================================================================
 *
 * Existing statement-level consumers require task-specific statement names.
 *
 * Therefore this file preserves:
 *
 *     taskSpawnStatement
 *     taskAwaitStatement
 *     taskParallelStatement
 *
 * These are adapters around the canonical expression constructs.
 *
 * They do not create a second syntax.
 *
 * The universal statement layer may therefore integrate them without taking
 * ownership of their underlying concurrency semantics.
 *
 * ============================================================================
 * SEMICOLON OWNERSHIP
 * ============================================================================
 *
 * Expression rules do not own statement terminators.
 *
 * Statement adapters may consume:
 *
 *     SEMI?
 *
 * according to the repository's existing statement-composition conventions.
 *
 * The canonical universal statement grammar remains responsible for deciding
 * whether an expression statement requires a terminator in a particular
 * syntactic context.
 *
 * ============================================================================
 * TASK BODY
 * ============================================================================
 *
 * A task body is represented by an existing Zamani expression or block.
 *
 * This grammar does NOT define another block syntax.
 *
 * Canonical forms include:
 *
 *     spawn computation()
 *
 *     spawn value
 *
 *     spawn {
 *         computation()
 *     }
 *
 * Whether a computation is legally spawnable is a semantic question.
 *
 * ============================================================================
 * NESTING AND SCALABILITY
 * ============================================================================
 *
 * Task constructs may be nested structurally through canonical expressions and
 * blocks.
 *
 * Examples include:
 *
 *     await value
 *
 *     await spawn computation()
 *
 *     spawn spawn computation()
 *
 *     parallel {
 *         await work()
 *     }
 *
 *     spawn {
 *         await spawn {
 *             parallel {
 *                 work()
 *             }
 *         }
 *     }
 *
 * The grammar does not establish a finite task nesting depth.
 *
 * Any eventual limitation caused by:
 *
 *     parser stack;
 *     compiler memory;
 *     compiler time;
 *     runtime memory;
 *     runtime scheduling capacity;
 *     target capacity
 *
 * is an implementation/resource condition rather than a language-defined
 * grammar maximum.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing task syntax must depend only on:
 *
 *     - source token sequence;
 *     - grammar version;
 *     - canonical grammar composition.
 *
 * Parsing must NOT depend on:
 *
 *     - available CPU count;
 *     - worker count;
 *     - GPU availability;
 *     - QPU availability;
 *     - runtime scheduler state;
 *     - network state;
 *     - device state;
 *     - system time;
 *     - randomness;
 *     - environment state.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * These grammar adapters must preserve enough parse structure for the frontend
 * AST to represent:
 *
 *     - construct kind;
 *     - operand;
 *     - nested expressions;
 *     - nested blocks;
 *     - operand ordering;
 *     - source locations;
 *     - complete source span.
 *
 * The grammar must NOT require backend-specific AST nodes such as:
 *
 *     RuntimeTaskId
 *     WorkerId
 *     Thread
 *     Executor
 *     GPUHandle
 *     QPUHandle
 *     PhysicalDevice
 *
 * The domain-neutral frontend AST remains authoritative.
 *
 * If the AST represents these constructs using a generic operation/expression
 * model, this grammar must map to that existing model rather than creating a
 * competing task AST hierarchy.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis determines:
 *
 *     - whether a computation is spawnable;
 *     - whether a value is awaitable;
 *     - dependency relationships;
 *     - ownership;
 *     - borrowing;
 *     - lifetimes;
 *     - effects;
 *     - cancellation behavior;
 *     - synchronization;
 *     - determinism;
 *     - resource requirements;
 *     - capabilities;
 *     - domain-specific legality.
 *
 * None of these are parser responsibilities.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource analysis belongs downstream.
 *
 * Task syntax must not directly specify physical resource realization.
 *
 * Portable resource concepts may be expressed elsewhere through the canonical
 * resource/capability system, for example:
 *
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *
 * The task grammar itself does not decide:
 *
 *     how many workers;
 *     which CPU;
 *     which GPU;
 *     which accelerator;
 *     which QPU;
 *     which node;
 *     which memory bank.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Spawn, await, and parallel constructs may participate in the language's
 * effect system.
 *
 * Effects determine such properties as:
 *
 *     suspension;
 *     concurrency;
 *     synchronization;
 *     communication;
 *     mutation;
 *     resource access;
 *     cancellation.
 *
 * This grammar only exposes the source structure.
 *
 * ============================================================================
 * MEMORY / OWNERSHIP CONTRACT
 * ============================================================================
 *
 * A task may capture values according to the language's normal ownership,
 * borrowing, lifetime, closure, and memory rules.
 *
 * This grammar does not create task-specific ownership rules.
 *
 * In particular, it must not encode:
 *
 *     task-local memory size;
 *     stack size;
 *     heap size;
 *     fixed capture count;
 *     fixed argument count.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Task syntax is domain-neutral.
 *
 * A task operand may eventually represent:
 *
 *     classical computation;
 *     quantum computation;
 *     measurement;
 *     classical feed-forward;
 *     hybrid computation;
 *     accelerator work;
 *     distributed computation;
 *     HDL/co-design computation.
 *
 * If an operand becomes quantum semantic content, its canonical semantic path
 * remains:
 *
 *     source
 *       ->
 *     frontend AST
 *       ->
 *     semantic analysis
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     routing/scheduling
 *       ->
 *     QEC/resilience/ZQN
 *       ->
 *     HAL/target
 *
 * This file creates NO quantum-specific task IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A task may represent computation that eventually interacts with:
 *
 *     HDL;
 *     hardware/software co-design;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     heterogeneous hardware.
 *
 * This grammar does not select hardware or physical topology.
 *
 * Hardware capabilities belong to the hardware/resource/compile/execution
 * semantic layers.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * `spawn` does not imply locality.
 *
 * A spawned computation may ultimately execute:
 *
 *     locally;
 *     in another process;
 *     on another machine;
 *     on an accelerator;
 *     on a cluster;
 *     in a cloud deployment;
 *     on a future computational substrate.
 *
 * Placement is downstream.
 *
 * ============================================================================
 * FUTURE INTEGRATION
 * ============================================================================
 *
 * Future-oriented semantics belong to:
 *
 *     grammar/concurrency/futures.g4
 *
 * That grammar must consume:
 *
 *     awaitExpression
 *
 * rather than redefine it.
 *
 * This prevents:
 *
 *     task await
 *     future await
 *     async await
 *
 * from becoming three different languages.
 *
 * ============================================================================
 * CONCURRENCY-DOMAIN INTEGRATION
 * ============================================================================
 *
 * `grammar/concurrency/concurrency.g4` is the domain-level composition root.
 *
 * It may consume:
 *
 *     taskExpression
 *     taskStatement
 *
 * but must not redefine task syntax.
 *
 * The dependency direction is:
 *
 *     expressions/async.g4
 *             |
 *             v
 *         tasks.g4
 *             |
 *             v
 *      concurrency.g4
 *             |
 *             v
 *     canonical composition
 *
 * ============================================================================
 * STATEMENT-LAYER INTEGRATION
 * ============================================================================
 *
 * `grammar/statements/concurrency.g4` is the statement-layer adapter.
 *
 * It may consume:
 *
 *     taskSpawnStatement
 *     taskAwaitStatement
 *     taskParallelStatement
 *
 * but must not redefine them.
 *
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * `grammar/functions/async.g4` owns async function declarations.
 *
 * This file must NOT define:
 *
 *     asyncFunctionDeclaration
 *     asyncModifier
 *     functionDeclaration
 *
 * An async function body may contain task expressions because its body
 * eventually enters canonical expression/block grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar generates no IR.
 *
 * The intended path is:
 *
 *     task syntax
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     semantic analysis
 *         |
 *         +--------------------------+
 *         |                          |
 *         v                          v
 *     task/concurrency semantics   domain semantics
 *         |                          |
 *         +------------+-------------+
 *                      |
 *                      v
 *               canonical semantic IR
 *                      |
 *          +-----------+-----------+
 *          |           |           |
 *          v           v           v
 *      classical   quantum::ir   HDL/hardware
 *          |           |           |
 *          +-----------+-----------+
 *                      |
 *                      v
 *              optimization
 *                      |
 *                      v
 *              scheduling
 *                      |
 *                      v
 *                  runtime/HAL
 *
 * No second task IR is created by this grammar.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages downstream of this grammar may:
 *
 *     - construct dependency graphs;
 *     - analyze effects;
 *     - infer task properties;
 *     - perform task fusion;
 *     - serialize independent work;
 *     - expose parallelism;
 *     - lower to accelerator execution;
 *     - distribute computation;
 *     - schedule work according to available resources.
 *
 * Such transformations must preserve the language's semantic contract.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime owns:
 *
 *     - execution;
 *     - suspension;
 *     - resumption;
 *     - scheduling;
 *     - resource acquisition;
 *     - cancellation;
 *     - synchronization;
 *     - distributed execution;
 *     - recovery.
 *
 * Runtime must consume compiled semantic/IR representations.
 *
 * Runtime must NOT parse:
 *
 *     grammar/concurrency/tasks.g4
 *
 * directly.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * The presence of:
 *
 *     spawn
 *     await
 *     parallel
 *
 * does not grant capabilities.
 *
 * Semantic/security policy remains responsible for:
 *
 *     authorization;
 *     ownership;
 *     isolation;
 *     resource policy;
 *     capability checks;
 *     effect restrictions;
 *     deployment policy.
 *
 * Parser recovery must not create a valid task AST for malformed syntax in a
 * way that silently changes the intended program.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level diagnostics should cover structural failures such as:
 *
 *     - malformed task expression;
 *     - missing spawn operand;
 *     - missing await operand;
 *     - missing parallel operand;
 *     - malformed task statement;
 *     - malformed task block;
 *     - unexpected token following a task construct.
 *
 * Semantic diagnostics belong downstream, including:
 *
 *     - awaiting a non-awaitable value;
 *     - spawning a non-spawnable computation;
 *     - illegal concurrent access;
 *     - invalid ownership;
 *     - invalid lifetime;
 *     - unsatisfied capability;
 *     - unavailable resource;
 *     - invalid quantum execution requirement;
 *     - invalid hardware requirement.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing public task-oriented rule names are retained where possible:
 *
 *     taskExpression
 *     taskConcurrencyExpression
 *     taskSpawnExpression
 *     taskAwaitExpression
 *     taskParallelExpression
 *     taskSpawnStatement
 *     taskAwaitStatement
 *     taskParallelStatement
 *     taskStatement
 *
 * Their implementation is changed from duplicated syntax to adapters around
 * the canonical asynchronous expression grammar.
 *
 * This preserves parser-composition compatibility while eliminating competing
 * syntax authorities.
 *
 * The following must NOT be reintroduced as independent syntax:
 *
 *     awaitTaskExpression
 *     awaitTaskStatement
 *
 * if they duplicate the canonical await semantics.
 *
 * Likewise:
 *
 *     futureAwaitExpression
 *
 * must remain an adapter to canonical `awaitExpression`, not a second await
 * implementation.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     - hardware identifiers;
 *     - physical addresses;
 *     - device IDs;
 *     - worker IDs;
 *     - thread counts;
 *     - core counts;
 *     - CPU counts;
 *     - GPU counts;
 *     - FPGA counts;
 *     - QPU counts;
 *     - node counts;
 *     - memory capacities;
 *     - queue capacities;
 *     - task-count ceilings;
 *     - parallelism ceilings;
 *     - topology;
 *     - vendor-specific execution objects.
 *
 * Literal values appearing inside ordinary expressions remain valid program
 * data and are not considered language-level resource limits.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This grammar requires conformance coverage for:
 *
 * POSITIVE
 * --------
 *
 *     spawn computation()
 *     spawn value
 *     spawn { work() }
 *     await task
 *     await computation()
 *     await spawn computation()
 *     parallel computation()
 *     parallel { work() }
 *
 *     nested task constructs
 *     async-function bodies
 *     task expressions inside ordinary expressions
 *     task statements
 *
 * NEGATIVE
 * --------
 *
 *     spawn
 *     await
 *     parallel
 *
 *     malformed task blocks
 *     malformed nesting
 *     unexpected delimiters
 *     invalid token sequences
 *
 * BOUNDARY
 * --------
 *
 *     empty block
 *     single task
 *     nested tasks
 *     deeply nested tasks
 *     wide independent task sets
 *     mixed await/spawn/parallel constructs
 *
 * SCALABILITY
 * ----------
 *
 * Tests must vary logical task count and nesting without asserting a language
 * maximum.
 *
 * Any implementation-resource failure must be distinguished from a grammar
 * rejection caused by an artificial language limit.
 *
 * CROSS-DOMAIN
 * ------------
 *
 *     classical + task
 *     quantum + task
 *     hybrid + task
 *     HDL + task
 *     AI + task
 *     distributed + task
 *     networking + task
 *     accelerator + task
 *
 * DETERMINISM
 * -----------
 *
 * Identical source and grammar version must produce identical parse structure
 * independent of hardware/resource availability.
 *
 * COMPATIBILITY
 * -------------
 *
 * Existing task rule consumers must continue to resolve through the adapter
 * names defined here.
 *
 * ============================================================================
 * FEATURE-MANIFEST CONTRACT
 * ============================================================================
 *
 * The corresponding machine-readable feature contract should identify this
 * feature as something equivalent to:
 *
 *     task-concurrency
 *
 * and record:
 *
 *     syntax owner:
 *         grammar/expressions/async.g4
 *
 *     task adapter:
 *         grammar/concurrency/tasks.g4
 *
 *     lexer:
 *         grammar/antlr/ZamaniLexer.g4
 *
 *     AST:
 *         domain-neutral frontend AST
 *
 *     semantics:
 *         concurrency/type/effect/ownership/resource analysis
 *
 *     IR:
 *         canonical semantic IR
 *
 *     quantum:
 *         quantum::ir
 *
 *     scheduler:
 *         scheduling subsystem
 *
 *     runtime:
 *         concurrency/runtime subsystem
 *
 *     tests:
 *         grammar/tests/concurrency/
 *
 * No feature manifest may claim that this file itself implements scheduling,
 * resource allocation, runtime execution, or hardware support.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when all of the following are true:
 *
 * [x] Task-domain ownership is explicit.
 *
 * [x] Async expression ownership remains in expressions/async.g4.
 *
 * [x] awaitExpression is not duplicated.
 *
 * [x] spawnExpression is not duplicated.
 *
 * [x] parallelExpression is not duplicated.
 *
 * [x] Task-specific adapter rules are stable.
 *
 * [x] Existing task-oriented consumers can use taskExpression/taskStatement.
 *
 * [x] Statement integration is defined.
 *
 * [x] Concurrency-domain integration is defined.
 *
 * [x] Future integration is defined.
 *
 * [x] Async-function integration is defined.
 *
 * [x] AST contract is defined.
 *
 * [x] Semantic contract is defined.
 *
 * [x] Effect contract is defined.
 *
 * [x] Resource contract is defined.
 *
 * [x] IR contract is defined.
 *
 * [x] Compiler contract is defined.
 *
 * [x] Runtime contract is defined.
 *
 * [x] Quantum integration preserves quantum::ir.
 *
 * [x] HDL/hardware integration remains downstream.
 *
 * [x] Distributed integration remains target-independent.
 *
 * [x] No hardware limits are encoded.
 *
 * [x] No worker/thread/core counts are encoded.
 *
 * [x] No task-count ceiling is encoded.
 *
 * [x] No topology is encoded.
 *
 * [x] No unsafe Rust is introduced.
 *
 * [x] Rust 1.97 / 1.97.1 compatibility is documented.
 *
 * [x] Deterministic parsing is preserved.
 *
 * [x] Positive tests are specified.
 *
 * [x] Negative tests are specified.
 *
 * [x] Boundary tests are specified.
 *
 * [x] Scalability tests are specified.
 *
 * [x] Cross-domain tests are specified.
 *
 * [x] Compatibility tests are specified.
 *
 * [x] Hard-coding audit is specified.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar Tasks;

options {
    tokenVocab = ZamaniLexer;
}

import AsyncExpressions;


/*
 * ============================================================================
 * PUBLIC TASK EXPRESSION ENTRY
 * ============================================================================
 *
 * This is the stable task-domain expression boundary.
 *
 * It does not own the underlying async syntax.
 */
taskExpression
    : taskConcurrencyExpression
    ;


/*
 * ============================================================================
 * TASK CONCURRENCY EXPRESSION
 * ============================================================================
 *
 * Domain-specific adapter around the canonical asynchronous expression
 * constructs.
 */
taskConcurrencyExpression
    : taskSpawnExpression
    | taskAwaitExpression
    | taskParallelExpression
    ;


/*
 * ============================================================================
 * TASK SPAWN ADAPTER
 * ============================================================================
 *
 * Canonical syntax owner:
 *
 *     AsyncExpressions.spawnExpression
 *
 * This rule exists only for task-domain integration.
 */
taskSpawnExpression
    : spawnExpression
    ;


/*
 * ============================================================================
 * TASK AWAIT ADAPTER
 * ============================================================================
 *
 * Canonical syntax owner:
 *
 *     AsyncExpressions.awaitExpression
 */
taskAwaitExpression
    : awaitExpression
    ;


/*
 * ============================================================================
 * TASK PARALLEL ADAPTER
 * ============================================================================
 *
 * Canonical syntax owner:
 *
 *     AsyncExpressions.parallelExpression
 */
taskParallelExpression
    : parallelExpression
    ;


/*
 * ============================================================================
 * TASK SPAWN STATEMENT
 * ============================================================================
 *
 * Statement-level adapter.
 *
 * The actual spawn syntax remains owned by AsyncExpressions.
 */
taskSpawnStatement
    : taskSpawnExpression SEMI?
    ;


/*
 * ============================================================================
 * TASK AWAIT STATEMENT
 * ============================================================================
 *
 * Statement-level adapter.
 */
taskAwaitStatement
    : taskAwaitExpression SEMI?
    ;


/*
 * ============================================================================
 * TASK PARALLEL STATEMENT
 * ============================================================================
 *
 * Statement-level adapter.
 */
taskParallelStatement
    : taskParallelExpression SEMI?
    ;


/*
 * ============================================================================
 * PUBLIC TASK STATEMENT ENTRY
 * ============================================================================
 *
 * This is the stable task-domain statement boundary.
 */
taskStatement
    : taskSpawnStatement
    | taskAwaitStatement
    | taskParallelStatement
    ;


/*
 * ============================================================================
 * TASK COMPOSITION
 * ============================================================================
 *
 * Stable domain-level expression composition point for concurrency.g4.
 */
taskComposition
    : taskExpression
    ;


/*
 * ============================================================================
 * TASK STATEMENT COMPOSITION
 * ============================================================================
 *
 * Stable domain-level statement composition point for concurrency.g4 and
 * statements/concurrency.g4.
 */
taskStatementComposition
    : taskStatement
    ;