/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/async.g4
 *
 * Status:
 *     Production asynchronous-expression grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only.
 *     No unsafe Rust is required or permitted by the compiler implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL ASYNCHRONOUS EXPRESSION SYNTAX.
 *
 * It is intentionally separate from:
 *
 *     grammar/functions/async.g4
 *
 * which owns asynchronous FUNCTION DECLARATION syntax.
 *
 * This file owns:
 *
 *     - await expressions;
 *     - spawn expressions;
 *     - parallel expressions;
 *     - the public async-expression composition point;
 *     - async expression operands;
 *     - expression-level integration boundaries.
 *
 * This file does NOT create a second general expression hierarchy.
 *
 * The canonical expression hierarchy remains:
 *
 *     grammar/expressions/expression.g4
 *
 * whose public entry point is:
 *
 *     expression
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical expression grammar
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     ordinary expressions        asyncExpression
 *                                        |
 *                              +---------+---------+
 *                              |         |         |
 *                              v         v         v
 *                            await     spawn    parallel
 *                              |         |         |
 *                              +---------+---------+
 *                                        |
 *                                        v
 *                               domain-neutral AST
 *                                        |
 *                                        v
 *                              structural validation
 *                                        |
 *                                        v
 *                              semantic analysis
 *                                        |
 *                         +--------------+--------------+
 *                         |              |              |
 *                         v              v              v
 *                       types         effects       resources
 *                                        |
 *                                        v
 *                              canonical semantic model
 *                                        |
 *                         +--------------+--------------+
 *                         |              |              |
 *                         v              v              v
 *                    classical      quantum::ir     HDL/hardware
 *                         |              |              |
 *                         +--------------+--------------+
 *                                        |
 *                                        v
 *                            optimization / lowering
 *                                        |
 *                                        v
 *                         scheduling / routing / resilience
 *                                        |
 *                                        v
 *                                    runtime/HAL
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - awaitExpression;
 *     - spawnExpression;
 *     - parallelExpression;
 *     - asyncExpression;
 *     - asyncExpressionOperand;
 *     - asyncBlockOperand;
 *     - async expression composition boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - keywords;
 *     - punctuation;
 *     - identifiers;
 *     - general expression precedence;
 *     - general expression hierarchy;
 *     - blocks;
 *     - function declarations;
 *     - function signatures;
 *     - parameter lists;
 *     - types;
 *     - statements generally;
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
 *     - task identifiers;
 *     - physical resources;
 *     - topology;
 *     - placement;
 *     - routing;
 *     - quantum scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime implementation.
 *
 * ============================================================================
 * NON-DUPLICATION CONTRACT
 * ============================================================================
 *
 * `grammar/functions/async.g4`
 *     owns:
 *         async function declarations
 *         async function signatures
 *         async function definitions
 *
 * `grammar/expressions/async.g4`
 *     owns:
 *         asynchronous expressions
 *
 * `grammar/expressions/expression.g4`
 *     owns:
 *         the complete expression hierarchy
 *         the public `expression` rule
 *
 * `grammar/concurrency/tasks.g4`
 *     owns:
 *         task-oriented concurrency composition.
 *
 * Therefore:
 *
 *     functions/async.g4
 *         MUST NOT duplicate await/spawn/parallel expression grammar.
 *
 *     concurrency/tasks.g4
 *         SHOULD delegate expression-level async constructs to this file
 *         rather than defining competing await/spawn/parallel productions.
 *
 *     expression.g4
 *         MUST expose `asyncExpression` at the appropriate expression
 *         precedence level.
 *
 * No second `expression` rule is defined here.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Lexer authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Existing canonical tokens used here:
 *
 *     ASYNC
 *     AWAIT
 *     SPAWN
 *     PARALLEL
 *
 * The lexer already reserves these asynchronous/concurrent keywords.
 *
 * This file MUST NOT redefine them.
 *
 * This file MUST NOT add lexer rules.
 *
 * ============================================================================
 * CANONICAL PARSER DEPENDENCIES
 * ============================================================================
 *
 * The surrounding canonical parser composition supplies:
 *
 *     expression
 *     blockExpression
 *
 * The lexer supplies:
 *
 *     ASYNC
 *     AWAIT
 *     SPAWN
 *     PARALLEL
 *
 * This file deliberately does not redefine:
 *
 *     expression
 *     blockExpression
 *
 * Doing so would create competing grammar ownership.
 *
 * ============================================================================
 * SOURCE SEMANTICS
 * ============================================================================
 *
 * `await expression`
 *
 * expresses a dependency on an asynchronous computation or value.
 *
 * `spawn expression`
 *
 * expresses creation/submission of an asynchronous computation.
 *
 * `spawn { ... }`
 *
 * expresses asynchronous execution of a structured computation body.
 *
 * `parallel expression`
 *
 * expresses permission/intent for a computation to participate in concurrent
 * execution.
 *
 * `parallel { ... }`
 *
 * expresses a structured region whose independent computations may be exposed
 * to downstream parallelization.
 *
 * IMPORTANT:
 *
 * These forms express SOURCE INTENT.
 *
 * They do NOT guarantee:
 *
 *     - simultaneous execution;
 *     - a dedicated worker;
 *     - a dedicated thread;
 *     - one CPU core per computation;
 *     - one accelerator per computation;
 *     - one device per computation;
 *     - distributed execution;
 *     - a particular execution order except where semantic dependencies require
 *       one.
 *
 * Downstream semantic analysis, scheduling and runtime realization determine
 * what execution strategy preserves the program's meaning.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Asynchronous syntax must support:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Therefore this grammar introduces NO universal limits for:
 *
 *     tasks
 *     concurrent tasks
 *     awaits
 *     async functions
 *     async expressions
 *     nested async expressions
 *     parallel regions
 *     parallel operations
 *     workers
 *     threads
 *     cores
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     accelerators
 *     nodes
 *     devices
 *     memory
 *     channels
 *     queues
 *     timelines
 *     distributed participants
 *
 * Repetition and nesting are represented structurally.
 *
 * There are no grammar-level constants such as:
 *
 *     MAX_TASKS
 *     MAX_ASYNC
 *     MAX_AWAIT_DEPTH
 *     MAX_PARALLELISM
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_CORES
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * "Infinity" means that the language introduces no artificial finite
 * implementation limit.
 *
 * Actual limits remain implementation/resource-policy concerns.
 *
 * ============================================================================
 * RESOURCE MODEL
 * ============================================================================
 *
 * Resource availability MUST NOT alter parsing.
 *
 * For example, the same source:
 *
 *     parallel {
 *         a();
 *         b();
 *         c();
 *     }
 *
 * may be lowered to:
 *
 *     - sequential execution;
 *     - cooperative concurrency;
 *     - multiple CPU workers;
 *     - GPU execution;
 *     - accelerator execution;
 *     - distributed execution;
 *     - heterogeneous execution;
 *     - future execution substrates.
 *
 * Semantic correctness must be preserved.
 *
 * Resource requirements and capabilities belong to semantic/resource analysis,
 * not to this grammar.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     hardware identifiers;
 *     physical addresses;
 *     CPU identifiers;
 *     GPU identifiers;
 *     QPU identifiers;
 *     physical qubit identifiers;
 *     worker counts;
 *     thread counts;
 *     core counts;
 *     memory capacities;
 *     queue capacities;
 *     task-count limits;
 *     parallelism limits;
 *     topology;
 *     backend names;
 *     vendor-specific runtime objects.
 *
 * Literal values appearing inside ordinary Zamani expressions remain ordinary
 * program semantics and are therefore not prohibited.
 *
 * ============================================================================
 * ASYNC FUNCTION INTEGRATION
 * ============================================================================
 *
 * The existing:
 *
 *     grammar/functions/async.g4
 *
 * owns:
 *
 *     async fn name(...) { ... }
 *
 * This file MUST NOT redefine that declaration.
 *
 * An async function body may contain:
 *
 *     awaitExpression
 *     spawnExpression
 *     parallelExpression
 *
 * because its body is ultimately parsed through the canonical expression /
 * block grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Async expressions are domain-neutral.
 *
 * They may contain:
 *
 *     classical computation;
 *     quantum computation;
 *     measurements;
 *     classical feed-forward;
 *     hybrid computation;
 *     accelerator work;
 *     distributed computation;
 *     HDL/co-design operations;
 *     future domains.
 *
 * Example:
 *
 *     async fn hybrid() {
 *         let result = await quantum_job();
 *         process(result);
 *     }
 *
 * The grammar does not decide whether `quantum_job()` is:
 *
 *     local;
 *     remote;
 *     simulated;
 *     physical;
 *     logical;
 *     distributed;
 *     accelerated.
 *
 * That is semantic/backend information.
 *
 * If the computation becomes quantum semantic content, lowering proceeds
 * through the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This grammar creates no quantum-specific asynchronous IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Async expressions may refer to computations that eventually lower into:
 *
 *     classical execution;
 *     HDL;
 *     hardware/software co-design;
 *     accelerators;
 *     heterogeneous systems.
 *
 * This grammar does not select hardware.
 *
 * Hardware capability requirements belong to:
 *
 *     resources/
 *     hardware/
 *     compile/
 *     execution/
 *
 * and their semantic consumers.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * `spawn` and `await` do not imply locality.
 *
 * A spawned computation may eventually execute:
 *
 *     locally;
 *     on another process;
 *     on another machine;
 *     on an accelerator;
 *     on a cluster;
 *     in a cloud deployment;
 *     on a future computational substrate.
 *
 * Location and placement are semantic/runtime concerns.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Asynchronous expressions may introduce effects.
 *
 * This grammar only establishes their syntactic structure.
 *
 * Effect analysis determines:
 *
 *     - whether suspension is legal;
 *     - whether spawning is legal;
 *     - whether parallel execution is legal;
 *     - what synchronization is required;
 *     - what capabilities are required;
 *     - what ownership/borrowing restrictions apply.
 *
 * This grammar must not encode those semantic decisions.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * The operand of:
 *
 *     await
 *     spawn
 *     parallel
 *
 * remains a canonical Zamani expression.
 *
 * Whether an operand is awaitable, spawnable, or safely parallelizable is a
 * semantic/type-system decision.
 *
 * This grammar therefore does NOT introduce:
 *
 *     Future<T>
 *     Promise<T>
 *     Task<T>
 *     JoinHandle<T>
 *     Executor<T>
 *
 * as syntax-level runtime types.
 *
 * A library or semantic type system may provide such abstractions without
 * changing this grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every accepted construct must map into the repository's domain-neutral
 * frontend AST.
 *
 * The parser must preserve:
 *
 *     - construct kind;
 *     - operand;
 *     - operand ordering;
 *     - complete source span;
 *     - nested structure;
 *     - block contents;
 *     - token/source locations.
 *
 * The grammar must NOT introduce backend/runtime AST concepts such as:
 *
 *     RuntimeTaskId
 *     WorkerId
 *     Executor
 *     Thread
 *     FutureObject
 *     QPUHandle
 *     PhysicalQubit
 *
 * If the existing AST lacks a dedicated asynchronous expression node, the
 * frontend AST integration should represent these constructs using the
 * repository's generic operation/expression representation rather than making
 * this grammar invent a second AST model.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only source structure.
 *
 * Semantic analysis determines:
 *
 *     - whether an expression is awaitable;
 *     - whether a computation may be spawned;
 *     - whether a parallel region is legal;
 *     - dependency relationships;
 *     - ownership;
 *     - borrowing;
 *     - lifetimes;
 *     - effects;
 *     - synchronization;
 *     - cancellation semantics;
 *     - resource requirements;
 *     - capabilities;
 *     - determinism;
 *     - domain-specific legality.
 *
 * None of these checks belong in the grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * The grammar produces no IR directly.
 *
 * The canonical pipeline is:
 *
 *     async source syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *     classical/control     quantum::ir
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *             optimization
 *                    |
 *                    v
 *             scheduling
 *                    |
 *                    v
 *          runtime/target realization
 *
 * Async syntax MUST NOT create a second concurrency IR that competes with the
 * repository's canonical semantic/IR architecture.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * The runtime owns:
 *
 *     execution;
 *     suspension/resumption;
 *     scheduling;
 *     task management;
 *     resource acquisition;
 *     cancellation;
 *     synchronization;
 *     distributed execution;
 *     recovery.
 *
 * None of those mechanisms are represented as grammar-level implementation
 * requirements.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source token sequence;
 *     grammar version.
 *
 * Parsing MUST NOT depend on:
 *
 *     system time;
 *     randomness;
 *     environment variables;
 *     hardware discovery;
 *     device state;
 *     network state;
 *     runtime scheduler state;
 *     available worker count.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics should identify structural errors such as:
 *
 *     - missing await operand;
 *     - missing spawn operand;
 *     - missing parallel operand;
 *     - malformed async expression;
 *     - malformed block;
 *     - unexpected token after an async keyword.
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples of downstream diagnostics:
 *
 *     await of a non-awaitable value;
 *     spawning a non-spawnable computation;
 *     illegal parallel access;
 *     conflicting mutable accesses;
 *     unsatisfied capability;
 *     unavailable resource;
 *     invalid quantum execution requirement.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing canonical keywords remain:
 *
 *     async
 *     await
 *     spawn
 *     parallel
 *
 * This file does NOT introduce alternate spellings such as:
 *
 *     asynchronously
 *     wait
 *     launch
 *     concurrently
 *
 * Such spellings require an explicit language-version decision.
 *
 * Existing:
 *
 *     async fn ...
 *
 * remains owned by:
 *
 *     grammar/functions/async.g4
 *
 * ============================================================================
 * EXPRESSION PRECEDENCE CONTRACT
 * ============================================================================
 *
 * `await` and `spawn` are prefix asynchronous operators.
 *
 * They consume one canonical expression operand.
 *
 * `parallel` consumes either:
 *
 *     - one canonical expression; or
 *     - one canonical block expression.
 *
 * The canonical expression grammar decides where `asyncExpression` enters its
 * precedence hierarchy.
 *
 * This file intentionally does not duplicate:
 *
 *     assignmentExpression;
 *     conditionalExpression;
 *     rangeExpression;
 *     logicalExpression;
 *     arithmeticExpression;
 *     postfixExpression;
 *     primaryExpression.
 *
 * Those remain owned by expression.g4 and its specialized delegates.
 *
 * ============================================================================
 * RECURSION / SCALABILITY
 * ============================================================================
 *
 * These constructs are structurally recursive through canonical expressions
 * and blocks.
 *
 * Examples such as:
 *
 *     await await value
 *
 *     spawn spawn computation
 *
 *     parallel parallel computation
 *
 *     spawn {
 *         await spawn {
 *             parallel {
 *                 work();
 *             }
 *         }
 *     }
 *
 * are not rejected merely because the grammar has a fixed conceptual nesting
 * depth.
 *
 * Any parser-stack, memory, or execution-resource limitation is an
 * implementation/resource-policy issue, not a language-level restriction.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar does not grant capabilities merely because source contains:
 *
 *     spawn
 *     await
 *     parallel
 *
 * Capability authorization remains semantic/runtime policy.
 *
 * An asynchronous computation MUST NOT bypass:
 *
 *     ownership;
 *     permissions;
 *     effect restrictions;
 *     resource policy;
 *     security policy;
 *     isolation;
 *     deployment policy.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] `async.g4` is the expression-level async grammar;
 *   [x] async function declarations remain in functions/async.g4;
 *   [x] `expression` remains owned by expression.g4;
 *   [x] `blockExpression` remains owned by the block grammar;
 *   [x] lexer rules are not duplicated;
 *   [x] await syntax is defined;
 *   [x] spawn syntax is defined;
 *   [x] parallel syntax is defined;
 *   [x] expression operands are canonical expressions;
 *   [x] block operands are canonical block expressions;
 *   [x] no Future/Promise/executor runtime type is introduced;
 *   [x] no scheduler is encoded;
 *   [x] no worker/thread count is encoded;
 *   [x] no hardware limit is encoded;
 *   [x] no topology is encoded;
 *   [x] no physical resource is encoded;
 *   [x] no quantum-specific async grammar is created;
 *   [x] quantum::ir remains downstream;
 *   [x] classical integration remains domain-neutral;
 *   [x] HDL integration remains domain-neutral;
 *   [x] distributed integration remains domain-neutral;
 *   [x] resource selection remains downstream;
 *   [x] semantic validation remains downstream;
 *   [x] source spans can be preserved;
 *   [x] diagnostics have defined ownership;
 *   [x] deterministic parsing is preserved;
 *   [x] arbitrary structural nesting is supported;
 *   [x] no artificial finite scalability limit exists.
 *
 * ============================================================================
 */

parser grammar AsyncExpressions;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ASYNCHRONOUS EXPRESSION ENTRY POINT
 * ========================================================================== */

/**
 * All expression-level asynchronous constructs enter through this rule.
 *
 * The canonical expression grammar should integrate this rule at the
 * appropriate prefix/primary-expression boundary.
 *
 * This is intentionally NOT named `expression` because expression.g4 owns
 * that public rule.
 */
asyncExpression
    : awaitExpression
    | spawnExpression
    | parallelExpression
    ;


/* ============================================================================
 * 2. AWAIT
 * ========================================================================== */

/**
 * Wait for the value/progress represented by an expression.
 *
 * Examples:
 *
 *     await task
 *     await computation()
 *     await future.value
 *     await spawn computation()
 *
 * Whether the operand is actually awaitable is a semantic/type-system
 * decision.
 */
awaitExpression
    : AWAIT asyncExpressionOperand
    ;


/**
 * Named operand boundary for `await`.
 *
 * The operand is an ordinary canonical expression.
 *
 * `asyncExpressionOperand` does not define expression syntax itself; it
 * delegates to the canonical `expression` rule.
 */
asyncExpressionOperand
    : expression
    ;


/* ============================================================================
 * 3. SPAWN
 * ========================================================================== */

/**
 * Start an asynchronous computation.
 *
 * Examples:
 *
 *     spawn computation()
 *     spawn value
 *     spawn {
 *         compute();
 *     }
 *
 * The spawned computation has no grammar-level location, worker, executor,
 * device, or resource assignment.
 */
spawnExpression
    : SPAWN spawnOperand
    ;


/**
 * Spawn accepts either:
 *
 *     ordinary expression
 *
 * or:
 *
 *     structured block expression
 *
 * This keeps task creation independent of the computation domain.
 */
spawnOperand
    : expression
    | blockExpression
    ;


/* ============================================================================
 * 4. PARALLEL
 * ========================================================================== */

/**
 * Express parallel-execution intent.
 *
 * Examples:
 *
 *     parallel computation()
 *
 *     parallel {
 *         a();
 *         b();
 *         c();
 *     }
 *
 * The keyword does not guarantee simultaneous execution.
 *
 * Dependency analysis, effect analysis, resource analysis and scheduling
 * determine the realizable execution plan.
 */
parallelExpression
    : PARALLEL parallelOperand
    ;


/**
 * Structured or expression-level parallel operand.
 */
parallelOperand
    : expression
    | blockExpression
    ;


/* ============================================================================
 * 5. STRUCTURED ASYNC EXPRESSION
 * ========================================================================== */

/**
 * Stable adapter for expression-composition grammars.
 *
 * This rule exists so the canonical expression grammar can import/use one
 * named async-expression boundary instead of depending on the internal
 * productions.
 */
asyncComputationExpression
    : asyncExpression
    ;


/* ============================================================================
 * 6. AWAITABLE COMPUTATION BOUNDARY
 * ========================================================================== */

/**
 * Syntactic boundary only.
 *
 * Semantic analysis decides whether the expression can actually be awaited.
 *
 * This rule MUST NOT introduce a grammar-level Future/Promise/Task type.
 */
awaitableComputation
    : expression
    ;


/* ============================================================================
 * 7. SPAWNABLE COMPUTATION BOUNDARY
 * ========================================================================== */

/**
 * Syntactic boundary only.
 *
 * Semantic analysis decides whether the computation may be spawned.
 */
spawnableComputation
    : expression
    | blockExpression
    ;


/* ============================================================================
 * 8. PARALLELIZABLE COMPUTATION BOUNDARY
 * ========================================================================== */

/**
 * Syntactic boundary only.
 *
 * Semantic analysis decides whether the computation is safe/legal to expose
 * to concurrent execution.
 */
parallelizableComputation
    : expression
    | blockExpression
    ;


/* ============================================================================
 * 9. COMPOSITION ROOT
 * ========================================================================== */

/**
 * Stable public root for consumers that need the entire expression-level
 * asynchronous surface.
 */
asyncExpressionRoot
    : asyncExpression
    ;


/* ============================================================================
 * 10. INTEGRATION CONTRACTS
 * ========================================================================== */

/**
 * Intended expression integration:
 *
 *     primary/prefix expression composition
 *              |
 *              +--> asyncExpression
 *              |
 *              +--> other expression forms
 *
 * The canonical expression grammar remains the owner of precedence.
 *
 * This file must not redefine:
 *
 *     expression
 *     assignmentExpression
 *     conditionalExpression
 *     rangeExpression
 *     logicalOrExpression
 *     logicalAndExpression
 *     postfixExpression
 *     primaryExpression
 */


/**
 * Intended concurrency integration:
 *
 *     concurrency/tasks.g4
 *              |
 *              +--> awaitExpression
 *              +--> spawnExpression
 *              +--> parallelExpression
 *
 * `tasks.g4` should not create a competing second grammar for these exact
 * expression forms.
 *
 * Its task-level rules may wrap these expressions as statements or higher
 * level concurrency constructs.
 */


/**
 * Intended function integration:
 *
 *     functions/async.g4
 *              |
 *              +--> async fn declaration
 *                      |
 *                      v
 *                  blockExpression
 *                      |
 *                      v
 *                  asyncExpression
 *
 * Async function declaration syntax remains outside this file.
 */


/**
 * Intended frontend integration:
 *
 *     asyncExpression
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *
 * The AST must retain source structure and spans without embedding runtime
 * implementation objects.
 */


/**
 * Intended IR integration:
 *
 *     semantic async computation
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical/control semantics
 *          +--> quantum::ir where applicable
 *          +--> HDL/hardware semantics where applicable
 *
 * No async-specific competing IR is created by this grammar.
 */


/* ============================================================================
 * 11. NEGATIVE DESIGN CONTRACT
 * ========================================================================== */

/**
 * The following MUST remain invalid as grammar-owned concepts:
 *
 *     spawn_on_cpu(0, ...)
 *     spawn_on_gpu(0, ...)
 *     spawn_on_qpu(0, ...)
 *     spawn_on_core(3, ...)
 *     spawn_with_threads(8, ...)
 *     spawn_with_workers(16, ...)
 *
 * unless a future explicitly approved Zamani language feature defines such
 * target-specific syntax in an appropriate target/deployment grammar.
 *
 * Even then, it must not become a universal hardware assumption.
 */


/**
 * Likewise, this grammar must never add:
 *
 *     MAX_TASKS
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_PARALLEL
 *     MAX_AWAIT
 *
 * or equivalent parser-level limits.
 */


/* ============================================================================
 * 12. SOURCE-LEVEL EXAMPLES
 * ========================================================================== */

/*
 * Valid structural forms:
 *
 *     await value
 *
 *     await compute()
 *
 *     spawn compute()
 *
 *     spawn {
 *         compute();
 *     }
 *
 *     parallel compute()
 *
 *     parallel {
 *         first();
 *         second();
 *     }
 *
 *     async fn process() {
 *         let a = await first();
 *         let b = await second();
 *         return combine(a, b);
 *     }
 *
 * The semantic layer decides whether `first()` and `second()` can actually
 * execute concurrently and what resources are available.
 *
 * These examples intentionally do not prescribe:
 *
 *     CPU count
 *     GPU count
 *     QPU count
 *     thread count
 *     worker count
 *     node count
 *     memory size
 *     topology
 *     physical device IDs
 */


/* ============================================================================
 * 13. FINAL OWNERSHIP SUMMARY
 * ========================================================================== */

/*
 * expression.g4
 *     -> complete expression precedence and public `expression`.
 *
 * expressions/async.g4
 *     -> await/spawn/parallel expression syntax.
 *
 * functions/async.g4
 *     -> async function declarations and signatures.
 *
 * concurrency/tasks.g4
 *     -> task-oriented statement/composition layer.
 *
 * core/blocks.g4
 *     -> canonical blockExpression.
 *
 * lexer/ZamaniLexer.g4
 *     -> ASYNC/AWAIT/SPAWN/PARALLEL token definitions.
 *
 * semantic analysis
 *     -> awaitability, spawnability, parallel legality, effects, ownership,
 *        capabilities and resource requirements.
 *
 * canonical IR
 *     -> target-independent execution semantics.
 *
 * scheduling
 *     -> actual ordering/resource-aware execution plan.
 *
 * runtime
 *     -> actual execution.
 *
 * quantum::ir
 *     -> canonical quantum semantic boundary whenever async computation
 *        contains quantum semantics.
 *
 * QEC/ZQN/HAL/routing/calibration
 *     -> their existing downstream responsibilities remain unchanged.
 */