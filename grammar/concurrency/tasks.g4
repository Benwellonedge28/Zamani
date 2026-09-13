/*
 * ============================================================================
 * Zamani Programming Language
 * Production Task-Concurrency Grammar
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/tasks.g4
 *
 * Architectural role:
 *     Parser-level grammar for task-oriented concurrency.
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar introduces no Rust implementation code.
 *     The Zamani compiler/runtime MUST use safe Rust.
 *     Rust `unsafe` is neither required nor permitted.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - task spawning syntax;
 *   - asynchronous waiting syntax;
 *   - task-oriented parallel scopes;
 *   - task-oriented concurrency expressions;
 *   - reusable task statement productions;
 *   - the syntactic boundary between task intent and ordinary expressions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - task scheduling;
 *   - worker/thread counts;
 *   - CPU/core counts;
 *   - executor implementations;
 *   - runtime queues;
 *   - task identifiers;
 *   - task memory;
 *   - task stack sizes;
 *   - channel implementations;
 *   - actor implementations;
 *   - cancellation algorithms;
 *   - synchronization algorithms;
 *   - hardware topology;
 *   - CPU/GPU/QPU topology;
 *   - resource discovery;
 *   - resource limits;
 *   - placement;
 *   - routing;
 *   - scheduling policy;
 *   - optimization;
 *   - runtime dispatch;
 *   - classical IR;
 *   - quantum::ir;
 *   - hardware HAL;
 *   - ZQN;
 *   - QEC.
 *
 * Those concerns belong to their respective semantic, IR, scheduling,
 * resource, hardware, resilience, and runtime subsystems.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Task syntax expresses COMPUTATION INTENT.
 *
 * It MUST NOT encode assumptions about the machine on which the task executes.
 *
 * In particular, this grammar contains no language-level constants for:
 *
 *   - maximum tasks;
 *   - maximum concurrent tasks;
 *   - maximum threads;
 *   - maximum workers;
 *   - maximum cores;
 *   - maximum devices;
 *   - maximum nodes;
 *   - maximum queues;
 *   - maximum memory;
 *   - maximum task groups;
 *   - maximum parallelism.
 *
 * A task program may therefore be lowered according to the resources actually
 * available on the target:
 *
 *   tiny embedded system
 *   single-core CPU
 *   multicore CPU
 *   GPU
 *   accelerator
 *   FPGA
 *   quantum/classical hybrid system
 *   cluster
 *   distributed system
 *   cloud system
 *   future computational substrate
 *
 * Any practical resource limit MUST be introduced by:
 *
 *   semantic analysis
 *   resource analysis
 *   compilation policy
 *   scheduling
 *   deployment configuration
 *   runtime policy
 *   target capabilities
 *
 * and MUST NOT become a syntax-level maximum.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This file deliberately reuses canonical Zamani parser rules.
 *
 * Expected canonical lexer tokens:
 *
 *   ASYNC
 *   AWAIT
 *   SPAWN
 *   PARALLEL
 *
 * Expected canonical parser rules:
 *
 *   expression
 *   blockExpression
 *   statement
 *
 * These rules are owned elsewhere.
 *
 * DO NOT redefine them here.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Intended pipeline:
 *
 *   source
 *      |
 *      v
 *   ZamaniLexer
 *      |
 *      v
 *   ZamaniParser / parser composition
 *      |
 *      +--------------------+
 *      |                    |
 *      v                    v
 *   Core syntax          Tasks syntax
 *      |                    |
 *      +---------+----------+
 *                |
 *                v
 *            Frontend AST
 *                |
 *                v
 *        semantic/type/effect/
 *        resource analysis
 *                |
 *                v
 *        canonical program IR
 *                |
 *        +-------+--------+
 *        |                |
 *        v                v
 *   classical         quantum/hybrid
 *   execution         execution
 *        |                |
 *        +-------+--------+
 *                |
 *                v
 *          scheduling/
 *          placement/
 *          lowering
 *                |
 *                v
 *             runtime
 *
 * The grammar MUST NOT depend on the runtime to parse source code.
 *
 * The runtime MUST NOT be required to understand this grammar directly.
 *
 * ============================================================================
 * CANONICAL OWNERSHIP BOUNDARIES
 * ============================================================================
 *
 * `tasks.g4`
 *     owns syntax.
 *
 * Frontend AST
 *     owns structural representation.
 *
 * Semantic analysis
 *     owns validity and task semantics.
 *
 * Effect analysis
 *     owns concurrency/effect requirements.
 *
 * Resource analysis
 *     owns resource requirements and constraints.
 *
 * Canonical IR
 *     owns target-independent computation semantics.
 *
 * Scheduling
 *     owns ordering and resource-aware execution planning.
 *
 * Hardware abstraction
 *     owns target capabilities.
 *
 * Runtime
 *     owns actual execution.
 *
 * Resilience
 *     may decide how execution adapts to runtime faults.
 *
 * ============================================================================
 * IMPORTANT LEXICAL DESIGN DECISION
 * ============================================================================
 *
 * The current canonical lexer already reserves:
 *
 *     async
 *     await
 *     spawn
 *     parallel
 *
 * This file therefore uses those tokens directly.
 *
 * It intentionally DOES NOT introduce unsupported tokens such as:
 *
 *     TASK
 *     TASK_SCOPE
 *     TASK_GROUP
 *     CANCEL
 *     CHANNEL
 *     ACTOR
 *     RECEIVE
 *     SELECT
 *
 * merely to make this file appear more feature-complete.
 *
 * Those constructs require coordinated lexical, parser, AST, semantic and
 * documentation changes and therefore belong to their own independently
 * completable grammar work.
 *
 * ============================================================================
 * SEMANTIC PRINCIPLES
 * ============================================================================
 *
 * 1. `spawn` expresses concurrent task creation intent.
 *
 * 2. `await` expresses dependency on completion/value availability.
 *
 * 3. `parallel` expresses permission/intent for independent work to execute
 *    concurrently.
 *
 * 4. Actual parallelism is not guaranteed merely because the source contains
 *    `parallel`.
 *
 * 5. Actual execution resources are selected downstream.
 *
 * 6. A target with fewer resources may serialize or otherwise legally lower
 *    independent work while preserving program semantics.
 *
 * 7. A target with more resources may exploit greater parallelism where the
 *    semantic dependency graph permits it.
 *
 * 8. Task syntax must remain independent of machine scale.
 *
 * ============================================================================
 */

parser grammar Tasks;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. TASK CONCURRENCY ROOT
 * ============================================================================
 *
 * This rule is the stable entry point for task-specific expressions.
 *
 * It deliberately contains only constructs whose lexical tokens are already
 * part of the canonical Zamani lexer.
 *
 * Consumers can integrate this rule into the canonical expression grammar
 * without introducing another expression hierarchy.
 */
taskConcurrencyExpression
    : taskSpawnExpression
    | taskAwaitExpression
    | taskParallelExpression
    ;


/*
 * ============================================================================
 * 2. TASK SPAWN
 * ============================================================================
 *
 * `spawn` starts an asynchronous/concurrent computation.
 *
 * Valid structural forms include:
 *
 *     spawn computation()
 *
 *     spawn value
 *
 *     spawn {
 *         work()
 *     }
 *
 * The operand is deliberately an existing Zamani expression or block.
 *
 * This means task spawning can work with:
 *
 *     ordinary functions
 *     generic functions
 *     closures/lambdas
 *     quantum-classical computations
 *     accelerator work
 *     distributed operations
 *     future domain-specific computations
 *
 * without making those domains dependencies of this grammar file.
 */
taskSpawnExpression
    : SPAWN taskSpawnBody
    ;


/*
 * Task spawn body.
 *
 * A block is treated as a structured computation body.
 *
 * An ordinary expression allows an existing callable/expression model to
 * determine the computation.
 */
taskSpawnBody
    : blockExpression
    | expression
    ;


/*
 * ============================================================================
 * 3. TASK SPAWN STATEMENT
 * ============================================================================
 *
 * A spawn may appear directly as a statement.
 *
 * The optional semicolon is intentionally consistent with the existing
 * Zamani parser's statement conventions.
 */
taskSpawnStatement
    : taskSpawnExpression SEMI?
    ;


/*
 * ============================================================================
 * 4. AWAIT
 * ============================================================================
 *
 * `await` establishes a dependency on the result/progress of an asynchronous
 * computation.
 *
 * Examples:
 *
 *     await task
 *
 *     await computation()
 *
 *     await spawnable
 *
 * The operand remains an ordinary Zamani expression.
 *
 * This is critical for POCO-REAF because the grammar does not need to know
 * whether the awaited computation eventually executes:
 *
 *     locally
 *     remotely
 *     on a CPU
 *     on a GPU
 *     on an accelerator
 *     on a quantum/classical backend
 *     on a cluster
 *     on a future target.
 */
taskAwaitExpression
    : AWAIT expression
    ;


/*
 * ============================================================================
 * 5. AWAIT STATEMENT
 * ============================================================================
 */
taskAwaitStatement
    : taskAwaitExpression SEMI?
    ;


/*
 * ============================================================================
 * 6. TASK PARALLEL SCOPE
 * ============================================================================
 *
 * `parallel { ... }` establishes a syntactic region in which independent
 * operations may be exposed to downstream parallelization.
 *
 * The grammar does NOT promise that every contained operation will execute
 * simultaneously.
 *
 * Semantic analysis must determine:
 *
 *     dependencies
 *     effects
 *     aliasing
 *     resource requirements
 *     ordering constraints
 *     synchronization requirements
 *
 * Scheduling then determines the realizable execution plan.
 */
taskParallelExpression
    : PARALLEL taskParallelBody
    ;


taskParallelBody
    : blockExpression
    | expression
    ;


/*
 * ============================================================================
 * 7. TASK PARALLEL STATEMENT
 * ============================================================================
 */
taskParallelStatement
    : taskParallelExpression SEMI?
    ;


/*
 * ============================================================================
 * 8. TASK EXPRESSION ADAPTER
 * ============================================================================
 *
 * This adapter gives the main expression grammar one stable integration point.
 *
 * It MUST NOT be recursively inserted into itself.
 *
 * A canonical expression grammar can integrate:
 *
 *     taskExpression
 *
 * at the appropriate precedence level.
 */
taskExpression
    : taskConcurrencyExpression
    ;


/*
 * ============================================================================
 * 9. TASK STATEMENT ADAPTER
 * ============================================================================
 *
 * This adapter provides a stable statement-level integration point.
 *
 * It intentionally contains only task constructs that are currently lexically
 * representable by Zamani's canonical lexer.
 */
taskStatement
    : taskSpawnStatement
    | taskAwaitStatement
    | taskParallelStatement
    ;


/*
 * ============================================================================
 * 10. STRUCTURED TASK BODY
 * ============================================================================
 *
 * This rule exists as a named integration boundary rather than defining a new
 * block syntax.
 *
 * A structured task body is simply a canonical Zamani block.
 *
 * Ownership of block semantics remains with the core expression/statement
 * grammar.
 */
taskBody
    : blockExpression
    ;


/*
 * ============================================================================
 * 11. NESTED TASK COMPUTATION
 * ============================================================================
 *
 * Tasks may contain ordinary expressions or blocks, and those expressions may
 * contain further task expressions after the main expression grammar integrates
 * `taskExpression`.
 *
 * No finite nesting depth is encoded here.
 *
 * Any parser/runtime stack limitation is an implementation concern and must
 * not be represented as a language-level task limit.
 */
nestedTaskBody
    : blockExpression
    | expression
    ;


/*
 * ============================================================================
 * 12. ASYNC FUNCTION REFERENCE
 * ============================================================================
 *
 * The canonical lexer already exposes `async`.
 *
 * Function declaration syntax belongs to the functions grammar, so this file
 * MUST NOT duplicate a complete function declaration.
 *
 * This rule exists only to identify the task-related async declaration prefix
 * when a composition layer needs it.
 *
 * The actual declaration must be completed by the canonical function grammar.
 */
asyncFunctionPrefix
    : ASYNC FN
    ;


/*
 * ============================================================================
 * 13. ASYNC TASK INVOCATION
 * ============================================================================
 *
 * An async invocation is intentionally represented by the normal expression
 * model rather than by a special callable type.
 *
 * Semantic analysis determines whether an expression denotes an awaitable
 * computation.
 *
 * This avoids baking a particular runtime Future/Promise representation into
 * the source grammar.
 */
asyncTaskComputation
    : expression
    ;


/*
 * ============================================================================
 * 14. SPAWNABLE COMPUTATION
 * ============================================================================
 *
 * A spawnable computation is intentionally open.
 *
 * The semantic layer determines whether the expression is actually spawnable.
 *
 * The grammar therefore does NOT hard-code a closed list such as:
 *
 *     functionCall
 *     closure
 *     actor
 *     kernel
 *     quantumJob
 *     gpuKernel
 *
 * because future Zamani computation domains must remain extensible.
 */
spawnableComputation
    : expression
    | blockExpression
    ;


/*
 * ============================================================================
 * 15. TASK DEPENDENCY
 * ============================================================================
 *
 * An await operation creates a dependency from the current computation to
 * another computation.
 *
 * The grammar records only the syntactic dependency operand.
 *
 * The semantic/IR layer must construct the actual dependency relation.
 */
taskDependencyExpression
    : AWAIT expression
    ;


/*
 * ============================================================================
 * 16. TASK PARALLELISM INTENT
 * ============================================================================
 *
 * This is deliberately a syntactic intent marker.
 *
 * It MUST NOT be interpreted as:
 *
 *     "create N threads"
 *     "use all cores"
 *     "use one worker per task"
 *     "allocate one hardware execution unit per task"
 *
 * Instead it means:
 *
 *     "these computations may be considered for concurrent execution,
 *      subject to semantic dependencies and target capabilities."
 */
taskParallelIntent
    : PARALLEL
      (
          blockExpression
        | expression
      )
    ;


/*
 * ============================================================================
 * 17. TASK COMPOSITION
 * ============================================================================
 *
 * This rule is useful to parser composition layers that need a single
 * concurrency-task production.
 */
taskComposition
    : taskSpawnExpression
    | taskAwaitExpression
    | taskParallelExpression
    ;


/*
 * ============================================================================
 * 18. TASK STATEMENT COMPOSITION
 * ============================================================================
 */
taskStatementComposition
    : taskSpawnStatement
    | taskAwaitStatement
    | taskParallelStatement
    ;


/*
 * ============================================================================
 * 19. TASK BLOCK
 * ============================================================================
 *
 * No new braces are defined here.
 *
 * The canonical blockExpression remains authoritative.
 */
taskBlock
    : blockExpression
    ;


/*
 * ============================================================================
 * 20. TASK-SAFE EXPRESSION BOUNDARY
 * ============================================================================
 *
 * This boundary allows semantic analysis to distinguish the syntactic task
 * operation from the arbitrary expression it contains.
 *
 * The grammar does not impose task-specific type restrictions.
 */
taskOperand
    : expression
    | blockExpression
    ;


/*
 * ============================================================================
 * 21. TASK ROOT
 * ============================================================================
 *
 * Stable public integration rule.
 *
 * Parser composition layers SHOULD prefer this rule rather than depending on
 * individual internal productions where practical.
 */
task
    : taskComposition
    ;


/*
 * ============================================================================
 * 22. TASK ROOT STATEMENT
 * ============================================================================
 */
taskStatementRoot
    : taskStatementComposition
    ;


/*
 * ============================================================================
 * 23. DOCUMENTED SEMANTIC CONTRACT
 * ============================================================================
 *
 * The following semantic distinctions are intentionally NOT represented as
 * parser alternatives:
 *
 *     spawn == thread creation
 *     parallel == physical parallel execution
 *     await == OS blocking
 *
 * None of those equations is valid at the language level.
 *
 * Instead:
 *
 *     spawn
 *         -> concurrent computation intent
 *
 *     await
 *         -> dependency/value synchronization intent
 *
 *     parallel
 *         -> permitted parallel execution intent
 *
 * The compiler may lower those intents differently depending on:
 *
 *     available resources
 *     dependency graph
 *     effect constraints
 *     memory model
 *     accelerator capabilities
 *     distributed capabilities
 *     quantum/classical execution model
 *     scheduling policy
 *     target constraints
 *
 * while preserving source-level semantics.
 */


/*
 * ============================================================================
 * 24. RESOURCE INDEPENDENCE
 * ============================================================================
 *
 * No task production accepts a machine resource count.
 *
 * Examples deliberately NOT encoded by this grammar:
 *
 *     spawn[8]
 *     parallel[64]
 *     task_scope[1024]
 *     threads = 16
 *     workers = 32
 *     cores = 128
 *
 * If a future Zamani language feature needs an explicit resource requirement,
 * it must use the canonical resource/requirements/constraints grammar and
 * semantic model rather than adding a task-specific hardware limit here.
 */


/*
 * ============================================================================
 * 25. CLASSICAL / QUANTUM / HDL INDEPENDENCE
 * ============================================================================
 *
 * Task syntax remains domain-neutral.
 *
 * A task operand may eventually denote:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL generation
 *     accelerator work
 *     distributed computation
 *     AI computation
 *     data processing
 *     networking
 *     future computation domains
 *
 * This file therefore MUST NOT import or redefine:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     scheduling
 *     hardware topology
 *     HDL IR
 *     classical IR
 *
 * Those systems consume semantic representations produced after parsing.
 */


/*
 * ============================================================================
 * 26. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should represent these constructs structurally, for example:
 *
 *     TaskSpawn
 *         body
 *
 *     TaskAwait
 *         operand
 *
 *     TaskParallel
 *         body
 *
 * The exact AST type names belong to the frontend AST implementation.
 *
 * This grammar MUST NOT prescribe Rust struct definitions.
 *
 * The AST MUST preserve:
 *
 *     source span
 *     source ordering
 *     nested structure
 *     operand structure
 *
 * The AST MUST NOT invent:
 *
 *     thread IDs
 *     worker IDs
 *     CPU IDs
 *     device IDs
 *     hardware addresses
 *
 * merely because the source contains a task construct.
 */


/*
 * ============================================================================
 * 27. SEMANTIC ANALYSIS CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine, outside this grammar:
 *
 *     whether a spawn operand is spawnable;
 *     whether await is applied to an awaitable computation;
 *     whether parallel execution is legal;
 *     whether effects permit concurrent execution;
 *     whether shared state introduces dependencies;
 *     whether synchronization is required;
 *     whether resource requirements can be satisfied;
 *     whether cancellation/failure semantics are valid;
 *     whether a computation may cross a backend boundary.
 *
 * Syntax acceptance MUST NOT be confused with semantic validity.
 */


/*
 * ============================================================================
 * 28. EFFECT SYSTEM CONTRACT
 * ============================================================================
 *
 * Task operations may interact with:
 *
 *     IO
 *     memory
 *     networking
 *     hardware
 *     quantum execution
 *     distributed execution
 *     security
 *     external effects
 *
 * Effect ownership belongs to the effects subsystem.
 *
 * This grammar only records task syntax.
 */


/*
 * ============================================================================
 * 29. RESOURCE SYSTEM CONTRACT
 * ============================================================================
 *
 * Resource requirements belong to the canonical resource grammar.
 *
 * Task syntax may cause semantic analysis to derive resource requirements,
 * but `tasks.g4` must not encode the resource model itself.
 *
 * This preserves the distinction between:
 *
 *     semantic requirement
 *     resource constraint
 *     capability
 *     preference
 *     hint
 *     actual allocation
 *
 * and prevents task syntax from becoming hardware-specific.
 */


/*
 * ============================================================================
 * 30. SCHEDULING CONTRACT
 * ============================================================================
 *
 * A task dependency graph may later be consumed by the scheduling subsystem.
 *
 * The scheduler determines:
 *
 *     ordering
 *     concurrency degree
 *     resource allocation
 *     placement
 *     timing
 *     synchronization
 *
 * The grammar does not select:
 *
 *     ASAP
 *     ALAP
 *     list scheduling
 *     critical path scheduling
 *     RCPSP
 *     event scheduling
 *     distributed scheduling
 *
 * Those are downstream policies.
 */


/*
 * ============================================================================
 * 31. QUANTUM INTEGRATION CONTRACT
 * ============================================================================
 *
 * Task syntax may surround or invoke quantum computations.
 *
 * Example conceptual form:
 *
 *     spawn quantum_computation(...)
 *
 *     await quantum_result
 *
 *     parallel {
 *         classical_work()
 *         quantum_work()
 *     }
 *
 * The grammar does not create a quantum representation.
 *
 * Quantum semantics must lower through the repository's canonical
 * `quantum::ir` boundary.
 */


/*
 * ============================================================================
 * 32. HARDWARE INTEGRATION CONTRACT
 * ============================================================================
 *
 * Hardware realization belongs downstream.
 *
 * A task may ultimately map to:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     distributed node
 *     heterogeneous execution resource
 *
 * This grammar remains unchanged as those targets evolve.
 */


/*
 * ============================================================================
 * 33. DISTRIBUTED INTEGRATION CONTRACT
 * ============================================================================
 *
 * Nothing in this file assumes that a task is local.
 *
 * A task may eventually be:
 *
 *     local
 *     remote
 *     replicated
 *     migrated
 *     scheduled across a cluster
 *
 * Distributed placement belongs to the distributed execution/resource layers.
 */


/*
 * ============================================================================
 * 34. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given identical source text and identical lexer/parser configuration,
 * parsing must produce the same parse structure.
 *
 * This grammar contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no mutable parser state;
 *     no runtime callbacks;
 *     no environment queries;
 *     no hardware queries;
 *     no time queries;
 *     no randomness.
 *
 * This is intentional.
 */


/*
 * ============================================================================
 * 35. ERROR-RECOVERY CONTRACT
 * ============================================================================
 *
 * Error reporting belongs to the parser/frontend diagnostic layer.
 *
 * This grammar must not:
 *
 *     print diagnostics;
 *     write files;
 *     access the network;
 *     query hardware;
 *     invoke runtime code.
 *
 * Generated parser recovery must remain deterministic and must preserve
 * source-location information for frontend diagnostics.
 */


/*
 * ============================================================================
 * 36. SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is no task-count limit in this grammar.
 *
 * There is no nesting-count constant.
 *
 * There is no thread-count constant.
 *
 * There is no worker-count constant.
 *
 * There is no machine-size constant.
 *
 * There is no device-count constant.
 *
 * Therefore:
 *
 *     source-scale
 *     task-count
 *     concurrency-degree
 *     machine-scale
 *
 * remain independent concepts.
 *
 * Any implementation resource limit must be explicit and external to syntax.
 */


/*
 * ============================================================================
 * 37. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical lexical spellings:
 *
 *     async
 *     await
 *     spawn
 *     parallel
 *
 * are reused rather than replaced.
 *
 * This file must therefore not introduce aliases that silently change the
 * meaning of existing source programs.
 *
 * Future concurrency keywords should be added through the canonical lexer
 * and language-version compatibility process before being referenced here.
 */


/*
 * ============================================================================
 * 38. EXTENSIBILITY CONTRACT
 * ============================================================================
 *
 * Future task features should be added as independently owned grammar files
 * rather than making this file a monolithic concurrency grammar.
 *
 * Examples of future independently owned domains include:
 *
 *     task cancellation
 *     task groups
 *     channels
 *     actors
 *     synchronization
 *     task selection
 *     deadlines
 *     timeouts
 *     distributed tasks
 *     task placement
 *     task resilience
 *
 * Such features must first establish their lexical, AST, semantic and
 * integration contracts before being added to the task root.
 */


/*
 * ============================================================================
 * 39. TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     spawn work()
 *     spawn { work() }
 *     await task
 *     await computation()
 *     parallel { work() }
 *     parallel computation()
 *
 * Nested:
 *
 *     spawn {
 *         await task
 *     }
 *
 *     parallel {
 *         spawn first()
 *         spawn second()
 *     }
 *
 * Cross-domain:
 *
 *     parallel {
 *         classical_work()
 *         quantum_work()
 *     }
 *
 *     spawn hardware_operation()
 *
 *     await distributed_result
 *
 * Negative:
 *
 *     spawn
 *     await
 *     parallel
 *
 *     spawn ;
 *     await ;
 *     parallel ;
 *
 *     malformed braces
 *     malformed expressions
 *
 * Boundary:
 *
 *     deeply nested task expressions;
 *     very large task bodies;
 *     many sibling spawn operations;
 *     many nested parallel scopes;
 *     large expressions used as task operands.
 *
 * Scalability:
 *
 *     no test may assert an arbitrary maximum number of tasks;
 *     no test may encode a maximum worker/thread/core count.
 *
 * Determinism:
 *
 *     parse the same source repeatedly and compare parse structure.
 */


/*
 * ============================================================================
 * 40. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] It generates successfully with the canonical Zamani lexer vocabulary.
 *
 * [ ] It contains no undefined lexer token references.
 *
 * [ ] It contains no Rust actions.
 *
 * [ ] It contains no semantic predicates.
 *
 * [ ] It contains no runtime dependencies.
 *
 * [ ] It contains no hardware assumptions.
 *
 * [ ] It contains no fixed task/resource limits.
 *
 * [ ] It does not redefine canonical expression/block rules.
 *
 * [ ] It does not define a second AST.
 *
 * [ ] It does not define an IR.
 *
 * [ ] It does not define scheduling policy.
 *
 * [ ] It does not define executor policy.
 *
 * [ ] It does not define hardware topology.
 *
 * [ ] It remains domain-neutral.
 *
 * [ ] It supports spawn, await and parallel task intent.
 *
 * [ ] It has positive, negative, boundary and scalability tests.
 *
 * [ ] It can be composed into the canonical Zamani parser.
 *
 * [ ] Its semantic contract is documented before downstream implementation.
 *
 * [ ] Rust 1.97 / 1.97.1 compatibility is preserved by the generated
 *     frontend implementation.
 *
 * [ ] No unsafe Rust is required anywhere in the implementation path.
 *
 * ============================================================================
 */