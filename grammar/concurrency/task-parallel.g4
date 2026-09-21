/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/concurrency/task-parallel.g4
 *
 * STATUS
 * ------
 * CANONICAL TASK-PARALLEL PARSER COMPONENT
 *
 * PURPOSE
 * -------
 * Defines the source-level syntax for STRUCTURED TASK PARALLELISM.
 *
 * This file is deliberately a parser-domain grammar. It does not define:
 *
 *   - lexical vocabulary;
 *   - ordinary expressions;
 *   - ordinary statements;
 *   - spawn syntax;
 *   - await syntax;
 *   - generic parallel syntax;
 *   - data-parallel syntax;
 *   - scheduling;
 *   - resource allocation;
 *   - hardware topology;
 *   - worker/thread/core counts;
 *   - device selection;
 *   - runtime execution;
 *   - classical IR;
 *   - quantum IR;
 *   - HDL IR;
 *   - QEC;
 *   - ZQN;
 *   - HAL.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Compiler/runtime:
 *
 *   Rust 1.97 / Rust 1.97.1
 *   Rust 2021
 *   safe Rust only
 *
 * This grammar contains no target-language actions and therefore requires no
 * unsafe Rust.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *   - the task-parallel structured-group syntax;
 *   - task-parallel group boundaries;
 *   - task-parallel group items;
 *   - task-parallel task creation within a group;
 *   - task-parallel completion/dependency boundaries;
 *   - nested task-parallel groups;
 *   - the stable taskParallelConstruct entry point.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *   Tasks:
 *       grammar/concurrency/tasks.g4
 *
 *   Generic parallel computation:
 *       grammar/concurrency/parallel.g4
 *
 *   Data parallelism:
 *       grammar/concurrency/data-parallel.g4
 *
 *   Async expression syntax:
 *       grammar/expressions/async.g4
 *
 *   Ordinary expressions:
 *       grammar/expressions/
 *
 *   Ordinary blocks:
 *       grammar/core/ and expression/block grammar
 *
 *   Concurrency composition:
 *       grammar/concurrency/concurrency.g4
 *
 *   Statement integration:
 *       grammar/statements/concurrency.g4
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly one syntax owner for each construct.
 *
 * Spawn:
 *
 *     expressions/async.g4
 *             |
 *             v
 *     tasks.g4
 *             |
 *             v
 *     this file
 *
 * Await:
 *
 *     expressions/async.g4
 *             |
 *             v
 *     tasks.g4
 *             |
 *             v
 *     this file
 *
 * Generic parallel:
 *
 *     expressions/async.g4
 *             |
 *             v
 *     parallel.g4
 *
 * Task parallel:
 *
 *     this file
 *
 * This file MUST NOT redefine:
 *
 *     spawnExpression
 *     awaitExpression
 *     parallelExpression
 *
 * ============================================================================
 * TASK-PARALLEL SOURCE FORM
 * ============================================================================
 *
 * The task-parallel construct is deliberately distinct from generic:
 *
 *     parallel <expression>
 *
 * by using the existing lexical keyword:
 *
 *     group
 *
 * Therefore the canonical task-parallel form is:
 *
 *     parallel group {
 *         spawn computation_a();
 *         spawn computation_b();
 *     }
 *
 * The syntax does not require a new TASK_PARALLEL token.
 *
 * Existing lexical vocabulary is therefore preserved.
 *
 * ============================================================================
 * WHY `parallel group`
 * ============================================================================
 *
 * The repository already has:
 *
 *     PARALLEL
 *     GROUP
 *     SPAWN
 *     AWAIT
 *
 * Reusing these tokens avoids unnecessary lexical expansion while still giving
 * task-parallelism a syntactically identifiable construct.
 *
 * Generic:
 *
 *     parallel computation()
 *
 * remains owned by parallel.g4.
 *
 * Task parallel:
 *
 *     parallel group {
 *         ...
 *     }
 *
 * is owned here.
 *
 * This avoids making `task-parallel.g4` merely another spelling of
 * `parallelExpression`.
 *
 * ============================================================================
 * SEMANTIC MODEL
 * ============================================================================
 *
 * A task-parallel group describes LOGICAL TASK CONCURRENCY.
 *
 * It does NOT specify physical execution width.
 *
 * For example:
 *
 *     parallel group {
 *         spawn compute_a();
 *         spawn compute_b();
 *         spawn compute_c();
 *     }
 *
 * means that the three logical computations are candidates for concurrent
 * realization subject to:
 *
 *     - data dependencies;
 *     - ownership;
 *     - effects;
 *     - synchronization;
 *     - resource requirements;
 *     - capabilities;
 *     - correctness constraints;
 *     - target semantics.
 *
 * It does NOT mean:
 *
 *     three threads
 *     three cores
 *     three CPUs
 *     three GPUs
 *     three QPUs
 *     three nodes
 *     three hardware execution units
 *
 * The scheduler may realize the same semantic program using:
 *
 *     one worker;
 *     multiple workers;
 *     SIMD;
 *     GPU execution;
 *     FPGA execution;
 *     accelerator execution;
 *     distributed execution;
 *     quantum/classical orchestration;
 *     heterogeneous execution;
 *     future computational substrates.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar deliberately contains no universal physical limits.
 *
 * It does not encode:
 *
 *     MAX_TASKS
 *     MAX_PARALLEL_TASKS
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ACCELERATORS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_TASK_GROUPS
 *     MAX_DEPENDENCIES
 *     MAX_NESTING
 *
 * Repetition operators:
 *
 *     *
 *     +
 *
 * describe grammar cardinality, not hardware capacity.
 *
 * Consequently the logical task graph is limited only by the actual source,
 * compiler, runtime, and target resources available for a particular build or
 * execution.
 *
 * There is no artificial language-level machine ceiling.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * This grammar never selects:
 *
 *     worker;
 *     thread;
 *     CPU;
 *     GPU;
 *     FPGA;
 *     accelerator;
 *     QPU;
 *     node;
 *     memory bank;
 *     device;
 *     queue;
 *     physical address.
 *
 * Resource requirements and capabilities are evaluated downstream through the
 * canonical resource/capability architecture.
 *
 * The semantic distinction remains:
 *
 *     logical task
 *         !=
 *     physical execution resource
 *
 *     capability
 *         !=
 *     selected device
 *
 *     requirement
 *         !=
 *     placement
 *
 * ============================================================================
 * DEPENDENCY MODEL
 * ============================================================================
 *
 * This file intentionally does NOT introduce a `DEPENDS` keyword.
 *
 * No such canonical lexer token currently exists, and introducing a new
 * dependency keyword solely for this grammar would create unnecessary lexical
 * coupling.
 *
 * Task dependencies are already expressible using the canonical `await`
 * construct.
 *
 * Example:
 *
 *     parallel group {
 *         let first = spawn compute_a();
 *
 *         spawn {
 *             await first;
 *             compute_b();
 *         };
 *     }
 *
 * The grammar records the source structure.
 *
 * Semantic analysis determines that the second computation depends on the
 * completion of `first`.
 *
 * The resulting dependency graph is semantic data, not a second grammar-level
 * graph representation.
 *
 * ============================================================================
 * STRUCTURED COMPLETION
 * ============================================================================
 *
 * A task-parallel group is a structured lifetime boundary.
 *
 * Its logical child tasks remain within the group for semantic analysis.
 *
 * The group boundary therefore gives downstream analysis a stable region in
 * which to determine:
 *
 *     - task lifetime;
 *     - dependency closure;
 *     - effect closure;
 *     - ownership;
 *     - synchronization;
 *     - completion;
 *     - cancellation/recovery policy where applicable.
 *
 * The grammar does not itself implement joining or scheduling.
 *
 * ============================================================================
 * NESTING
 * ============================================================================
 *
 * Task-parallel groups may contain nested task-parallel groups.
 *
 * Example:
 *
 *     parallel group {
 *         spawn outer_work();
 *
 *         parallel group {
 *             spawn inner_a();
 *             spawn inner_b();
 *         };
 *     }
 *
 * No finite nesting depth is encoded.
 *
 * Any practical parser/compiler stack or memory limitation is an implementation
 * resource limitation, not a language-level task-parallel restriction.
 *
 * ============================================================================
 * CROSS-DOMAIN SUPPORT
 * ============================================================================
 *
 * A spawned computation is intentionally expressed through the canonical
 * `spawnExpression` grammar.
 *
 * Therefore its computation may eventually contain:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL/hardware-oriented computation;
 *     distributed computation;
 *     AI/ML computation;
 *     data processing;
 *     networking;
 *     cryptographic computation;
 *     accelerator computation;
 *     future computational domains.
 *
 * This file does not need to know the internal grammar of those domains.
 *
 * The dependency is:
 *
 *     task-parallel syntax
 *             |
 *             v
 *     domain-neutral AST
 *             |
 *             v
 *     semantic analysis
 *             |
 *             +-------------------+
 *             |                   |
 *             v                   v
 *       effect/dependency     resource/capability
 *           analysis              analysis
 *             |                   |
 *             +---------+---------+
 *                       |
 *                       v
 *               canonical semantic IR
 *                       |
 *          +------------+------------+
 *          |            |            |
 *          v            v            v
 *      classical     quantum       HDL/
 *         IR         quantum::ir   hardware
 *          |            |            |
 *          +------------+------------+
 *                       |
 *                       v
 *              optimization/lowering
 *                       |
 *                       v
 *                scheduling/routing
 *                       |
 *                       v
 *                  target/runtime
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * This grammar has no direct quantum IR.
 *
 * Quantum computation appearing inside a task is eventually lowered through:
 *
 *     quantum::ir
 *
 * and then through the existing quantum pipeline:
 *
 *     optimization
 *         ->
 *     decomposition
 *         ->
 *     routing
 *         ->
 *     scheduling
 *         ->
 *     QEC/resilience
 *         ->
 *     ZQN
 *         ->
 *     HAL
 *         ->
 *     target realization
 *
 * Task parallelism does not create:
 *
 *     TaskQuantumIR
 *     ParallelQuantumIR
 *     TaskQubitIR
 *
 * or any competing quantum representation.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * This file does not define:
 *
 *     clocks;
 *     wires;
 *     ports;
 *     physical cells;
 *     FPGA resources;
 *     ASIC resources;
 *     device addresses;
 *     hardware topology.
 *
 * A task may contain or coordinate hardware-oriented computation through the
 * normal Zamani domain composition.
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A logical task may eventually execute:
 *
 *     locally;
 *     remotely;
 *     across a distributed system;
 *     on an accelerator;
 *     on a heterogeneous target.
 *
 * This grammar does not encode:
 *
 *     node IDs;
 *     cluster sizes;
 *     network topology;
 *     placement;
 *     routing.
 *
 * Those are downstream realization decisions.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source token sequence;
 *     - grammar version;
 *     - imported canonical grammars.
 *
 * Parsing does NOT depend on:
 *
 *     - hardware availability;
 *     - worker count;
 *     - scheduler state;
 *     - network state;
 *     - runtime state;
 *     - system time;
 *     - randomness.
 *
 * The same source and grammar version therefore produce the same syntactic
 * structure.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar contains NO lexer rules.
 *
 * Existing tokens consumed here are:
 *
 *     PARALLEL
 *     GROUP
 *     LBRACE
 *     RBRACE
 *     SEMICOLON
 *
 * Task primitives are delegated through:
 *
 *     Tasks
 *
 * No new lexical token is required by this implementation.
 *
 * In particular, this file does NOT introduce:
 *
 *     TASK
 *     TASK_PARALLEL
 *     TASK_GROUP
 *     DEPENDS
 *     JOIN
 *     WORKER
 *     THREAD
 *     CORE
 *     GPU
 *     QPU
 *     NODE
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This grammar imports:
 *
 *     Tasks
 *     Parallel
 *
 * `Tasks` supplies:
 *
 *     taskSpawnExpression
 *     taskAwaitExpression
 *     taskParallelExpression
 *     taskSpawnStatement
 *     taskAwaitStatement
 *     taskStatement
 *
 * `Parallel` supplies the canonical generic parallel domain and its imported
 * expression/block vocabulary.
 *
 * This file does not import individual expression/statement leaf grammars
 * because those dependencies already belong to their canonical owners.
 *
 * ============================================================================
 * IMPORTANT: NO RULE DUPLICATION
 * ============================================================================
 *
 * The following rules MUST NOT be defined here:
 *
 *     expression
 *     blockExpression
 *     parallelExpression
 *     spawnExpression
 *     awaitExpression
 *     taskSpawnExpression
 *     taskAwaitExpression
 *     taskParallelExpression
 *
 * They already have canonical owners.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `taskParallelConstruct` is the only public domain dispatcher owned here.
 *
 * `taskParallel` is provided as a stable compatibility alias.
 *
 * ============================================================================
 */

parser grammar TaskParallel;

options {
    tokenVocab = ZamaniLexer;
}

import
    Tasks,
    Parallel
;


/*
 * ============================================================================
 * PUBLIC TASK-PARALLEL ENTRY
 * ============================================================================
 *
 * Stable entry point used by:
 *
 *     grammar/concurrency/concurrency.g4
 *
 * and eventually:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * The rule intentionally has exactly one task-parallel syntax family.
 */

taskParallelConstruct
    : taskParallelGroup
    ;


/*
 * ============================================================================
 * TASK-PARALLEL GROUP
 * ============================================================================
 *
 * Canonical source form:
 *
 *     parallel group {
 *         spawn a();
 *         spawn b();
 *     }
 *
 * `parallel` is the existing generic parallel-intent keyword.
 *
 * `group` gives this construct its task-parallel-specific syntactic identity.
 *
 * This avoids redefining generic:
 *
 *     parallel <expression>
 *
 * from parallel.g4.
 */

taskParallelGroup
    : PARALLEL
      GROUP
      LBRACE
      taskParallelItem*
      RBRACE
    ;


/*
 * ============================================================================
 * TASK-PARALLEL ITEM
 * ============================================================================
 *
 * A group contains logical task creation, completion/dependency operations,
 * or nested task-parallel groups.
 *
 * Ordinary statements are deliberately not duplicated here.
 *
 * Computation belongs inside the expression or block supplied to `spawn`.
 */

taskParallelItem
    : taskParallelTask
    | taskParallelDependency
    | taskParallelNestedGroup
    ;


/*
 * ============================================================================
 * TASK CREATION
 * ============================================================================
 *
 * `taskSpawnExpression` remains owned by Tasks.
 *
 * This adapter only establishes that a spawn operation is a member of a
 * task-parallel group.
 */

taskParallelTask
    : taskSpawnExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * TASK DEPENDENCY / COMPLETION
 * ============================================================================
 *
 * Dependencies are expressed using the canonical `await` operation.
 *
 * This does NOT mean that every await is automatically a dependency in all
 * semantic contexts. Semantic/type/effect analysis determines whether the
 * awaited value represents an asynchronous computation and whether the await
 * establishes a task dependency.
 *
 * This rule preserves that structure for downstream analysis.
 */

taskParallelDependency
    : taskAwaitExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * COMPLETION ALIAS
 * ============================================================================
 *
 * Named semantic boundary for tooling and diagnostics.
 *
 * No new syntax is introduced.
 */

taskParallelCompletion
    : taskParallelDependency
    ;


/*
 * ============================================================================
 * TASK-PARALLEL JOIN
 * ============================================================================
 *
 * A structured join is represented by one or more canonical await operations.
 *
 * No fixed number of tasks or dependencies is encoded.
 *
 * Semantic analysis determines the actual dependency relationships.
 */

taskParallelJoin
    : taskParallelDependency+
    ;


/*
 * ============================================================================
 * NESTED TASK-PARALLEL GROUP
 * ============================================================================
 *
 * Nested structured parallelism is legal.
 */

taskParallelNestedGroup
    : taskParallelGroup
    ;


/*
 * ============================================================================
 * NAMED ITEM LIST
 * ============================================================================
 *
 * Stable list boundary for tooling and semantic analysis.
 *
 * The list is unbounded by language-level resource constants.
 */

taskParallelItemList
    : taskParallelItem*
    ;


/*
 * ============================================================================
 * NON-EMPTY ITEM LIST
 * ============================================================================
 *
 * Useful where a downstream grammar wants to require at least one logical
 * operation without modifying the core group rule.
 */

taskParallelNonEmptyItemList
    : taskParallelItem+
    ;


/*
 * ============================================================================
 * STABLE COMPATIBILITY ALIAS
 * ============================================================================
 *
 * Existing consumers may use `taskParallel`.
 *
 * It remains a pure adapter.
 */

taskParallel
    : taskParallelConstruct
    ;


/*
 * ============================================================================
 * EXPRESSION-LEVEL DOMAIN BOUNDARY
 * ============================================================================
 *
 * The task-parallel group is a structured computation.
 *
 * It is intentionally NOT named `taskParallelExpression`, because that rule is
 * already owned by tasks.g4 as the adapter for generic `parallelExpression`.
 *
 * Keeping these names distinct prevents two grammar files from claiming the
 * same rule.
 */

taskParallelGroupExpression
    : taskParallelGroup
    ;


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The parser establishes:
 *
 *     group
 *       |
 *       +--> logical task creation
 *       |
 *       +--> logical completion/dependency
 *       |
 *       +--> nested group
 *
 * Semantic analysis establishes:
 *
 *     - task identity;
 *     - dependency graph;
 *     - effect compatibility;
 *     - ownership/borrowing legality;
 *     - lifetime relationships;
 *     - cancellation semantics;
 *     - determinism requirements;
 *     - resource requirements;
 *     - capability requirements;
 *     - cross-domain legality.
 *
 * The grammar performs none of these semantic operations.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * No resource quantity is represented by this grammar.
 *
 * In particular, this grammar does not decide:
 *
 *     number of workers;
 *     number of threads;
 *     number of cores;
 *     number of CPUs;
 *     number of GPUs;
 *     number of FPGAs;
 *     number of accelerators;
 *     number of QPUs;
 *     number of nodes;
 *     amount of memory;
 *     topology;
 *     placement.
 *
 * Those decisions belong to:
 *
 *     semantic resource analysis
 *     compiler target selection
 *     scheduler
 *     runtime
 *     HAL
 *     deployment
 *
 * ============================================================================
 * SCHEDULING CONTRACT
 * ============================================================================
 *
 * This grammar does not select a scheduler.
 *
 * It does not specify:
 *
 *     worker assignment;
 *     work stealing;
 *     queue policy;
 *     priority policy;
 *     affinity;
 *     placement;
 *     time slots;
 *     physical execution order.
 *
 * It merely preserves the logical task structure from which those decisions
 * can later be derived.
 *
 * ============================================================================
 * MEMORY / OWNERSHIP CONTRACT
 * ============================================================================
 *
 * Task captures follow the canonical Zamani type, ownership, borrowing,
 * lifetime, and memory systems.
 *
 * This grammar introduces no second ownership model.
 *
 * Example:
 *
 *     parallel group {
 *         spawn {
 *             use_value();
 *         };
 *     }
 *
 * Whether `use_value()` may legally capture surrounding state is a semantic
 * question, not a grammar question.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Spawn and await participate in the existing effect system.
 *
 * The semantic layer determines effects such as:
 *
 *     asynchronous execution;
 *     suspension;
 *     synchronization;
 *     communication;
 *     mutation;
 *     external resource access;
 *     cancellation;
 *     distributed execution.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * A task body may contain quantum computation where permitted by the canonical
 * language composition.
 *
 * This grammar does not define:
 *
 *     gates;
 *     qubits;
 *     circuits;
 *     physical qubits;
 *     QEC;
 *     noise;
 *     calibration;
 *     routing.
 *
 * The quantum pipeline remains:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     quantum semantic analysis
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     QEC / resilience / ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Task bodies may coordinate hardware-oriented computations, but this grammar
 * does not encode physical hardware.
 *
 * No:
 *
 *     core IDs;
 *     accelerator IDs;
 *     FPGA resources;
 *     ASIC cells;
 *     device addresses;
 *     physical topology
 *
 * are introduced here.
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * A logical task may eventually be realized locally or remotely.
 *
 * The source grammar remains unchanged when deployment changes from:
 *
 *     embedded
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     accelerator
 *     cluster
 *     cloud
 *     heterogeneous system
 *     future target.
 *
 * Placement and topology remain downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser output must retain enough information for the domain-neutral
 * frontend AST to represent:
 *
 *     TaskParallelGroup
 *     TaskParallelTask
 *     TaskParallelDependency
 *     TaskParallelNestedGroup
 *
 * where those are the semantic categories selected by the frontend AST
 * architecture.
 *
 * The AST must preserve:
 *
 *     source span;
 *     source ordering;
 *     nesting;
 *     child parse structure;
 *     task operands;
 *     await operands.
 *
 * The AST must NOT invent:
 *
 *     worker IDs;
 *     thread IDs;
 *     CPU IDs;
 *     GPU IDs;
 *     QPU IDs;
 *     node IDs;
 *     physical addresses;
 *     scheduler IDs.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar defines NO IR.
 *
 * Task-parallel semantics must lower into the repository's existing canonical
 * semantic/IR architecture.
 *
 * There must not be:
 *
 *     TaskParallelIR
 *     ParallelTaskIR
 *     QuantumTaskIR
 *
 * as competing universal intermediate representations created by this file.
 *
 * If the canonical compiler IR already has a representation for structured
 * concurrency/dependencies, that representation is the sole downstream owner.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler consumes semantic information derived from this grammar for:
 *
 *     type checking;
 *     effect checking;
 *     dependency analysis;
 *     ownership analysis;
 *     resource analysis;
 *     capability checking;
 *     optimization;
 *     scheduling;
 *     target lowering.
 *
 * This grammar itself does none of those operations.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * The runtime receives compiled semantic/execution artifacts.
 *
 * It may realize a task-parallel group using:
 *
 *     serialization;
 *     cooperative execution;
 *     worker pools;
 *     OS threads;
 *     accelerator queues;
 *     distributed workers;
 *     heterogeneous resources;
 *     other future mechanisms.
 *
 * None of those mechanisms affect this grammar.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing lexical tokens are preserved.
 *
 * No new token is required for this file.
 *
 * The following existing tokens are consumed:
 *
 *     PARALLEL
 *     GROUP
 *     LBRACE
 *     RBRACE
 *     SEMICOLON
 *
 * Existing task syntax remains owned by Tasks.
 *
 * Existing generic parallel syntax remains owned by Parallel.
 *
 * The file therefore does not require a token rename or lexer migration.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors remain parser errors.
 *
 * Examples:
 *
 *     parallel group
 *     parallel group {
 *     parallel group { spawn }
 *     parallel group { await }
 *
 * are syntactically incomplete.
 *
 * Semantic errors remain downstream.
 *
 * Examples:
 *
 *     awaiting a non-awaitable value
 *     spawning an invalid computation
 *     illegal ownership capture
 *     unsatisfied capability
 *     conflicting effects
 *
 * must NOT be converted into grammar rules merely to improve diagnostics.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses repetition rather than finite enumeration.
 *
 * Therefore:
 *
 *     taskParallelItem*
 *
 * supports arbitrary source-level group cardinality.
 *
 * Nested:
 *
 *     taskParallelGroup
 *
 * supports arbitrary structural nesting subject only to actual parser/compiler
 * resources.
 *
 * There is no language-level task or worker ceiling.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_TASKS
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ACCELERATORS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_PARALLELISM
 *
 * It contains no:
 *
 *     worker IDs;
 *     device IDs;
 *     hardware addresses;
 *     physical topology.
 *
 * Any finite limitation encountered during compilation or execution must be
 * reported by the subsystem that owns that limitation.
 *
 * ============================================================================
 * VALIDATION CONTRACT
 * ============================================================================
 *
 * Validation for this grammar must cover:
 *
 * POSITIVE
 * --------
 *
 *     parallel group {}
 *     parallel group {
 *         spawn work();
 *     }
 *     parallel group {
 *         spawn work_a();
 *         spawn work_b();
 *     }
 *     parallel group {
 *         spawn {
 *             compute();
 *         };
 *         await task_handle;
 *     }
 *
 * NESTED
 * ------
 *
 *     parallel group {
 *         spawn outer();
 *         parallel group {
 *             spawn inner_a();
 *             spawn inner_b();
 *         };
 *     }
 *
 * NEGATIVE
 * --------
 *
 *     parallel group
 *     parallel group {
 *     parallel group { spawn }
 *     parallel group { await }
 *     parallel group ( ... )
 *     parallel group [ ... ]
 *
 * NON-TASK PARALLEL
 * -----------------
 *
 *     parallel compute()
 *
 * must remain owned by the generic parallel grammar and must not be silently
 * reclassified as a task-parallel group.
 *
 * SCALABILITY
 * -----------
 *
 * Tests must generate task-parallel groups containing varying numbers of
 * logical task items without establishing a language-level maximum.
 *
 * CROSS-DOMAIN
 * ------------
 *
 * Task bodies must be tested with:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL/hardware-related computation;
 *     distributed computation;
 *     AI/data computation;
 *     accelerator-oriented computation.
 *
 * These tests verify composition, not hardware allocation.
 *
 * DETERMINISM
 * -----------
 *
 * Identical source and grammar version must yield equivalent parse structure.
 *
 * ============================================================================
 * INTEGRATION CHECKLIST
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Existing filename preserved.
 * [x] Existing canonical tokens preserved.
 * [x] No new lexer token required.
 * [x] No Rust actions.
 * [x] No unsafe Rust dependency.
 * [x] No fixed hardware limits.
 * [x] No fixed worker/thread/core/device counts.
 * [x] No duplicate spawn grammar.
 * [x] No duplicate await grammar.
 * [x] No duplicate generic parallel grammar.
 * [x] No DEPENDS pseudo-token.
 * [x] No SEMI pseudo-token.
 * [x] No duplicate task-parallel expression rule.
 * [x] Task-parallel has a unique syntactic boundary.
 * [x] Dependencies remain semantic data derived from await/task structure.
 * [x] Nested task groups are supported.
 * [x] Logical task count is unbounded by grammar constants.
 * [x] Quantum remains downstream through `quantum::ir`.
 * [x] Resource/capability analysis remains downstream.
 * [x] Scheduling remains downstream.
 * [x] Runtime realization remains downstream.
 *
 * Remaining repository integration requirement:
 *
 *     grammar/concurrency/concurrency.g4
 *
 * must import `TaskParallel` as a delegate grammar so that its existing
 * `concurrencyTaskParallelConstruct` rule resolves:
 *
 *     taskParallelConstruct
 *
 * Likewise, the canonical parser composition must reach the `Concurrency`
 * domain through its normal domain-dispatch path.
 *
 * No change to the lexical vocabulary is required.
 *
 * ============================================================================
 * END
 * ============================================================================
 */