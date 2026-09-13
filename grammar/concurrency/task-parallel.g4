/*
 * ============================================================================
 * Zamani Programming Language
 * Production Task-Parallel Grammar
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/task-parallel.g4
 *
 * Purpose:
 *     Parser-level grammar for structured task parallelism.
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *
 * Safety:
 *     This grammar contains no target-language actions.
 *     No unsafe Rust is required.
 *     The Zamani compiler/runtime MUST be implemented using safe Rust.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file specializes the generic parallel-computation boundary supplied by:
 *
 *     concurrency/parallel.g4
 *
 * and the task-concurrency primitives supplied by:
 *
 *     concurrency/tasks.g4
 *
 * It owns the syntax that combines those concepts into explicit
 * task-parallel computation.
 *
 *
 * THIS FILE OWNS
 * --------------
 *
 * - task-parallel regions;
 * - task-parallel task collections;
 * - structured task groups;
 * - task-parallel composition;
 * - task dependency declarations;
 * - task join/wait composition;
 * - task-parallel completion boundaries;
 * - task-parallel iteration over logical task domains;
 * - task-parallel dependency expressions;
 * - task-parallel structured nesting;
 * - task-parallel semantic intent.
 *
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 * - ordinary expressions;
 * - ordinary statements;
 * - function declarations;
 * - async function declarations;
 * - futures;
 * - generic spawn syntax;
 * - generic await syntax;
 * - generic parallel syntax;
 * - actors;
 * - channels;
 * - synchronization algorithms;
 * - cancellation;
 * - scheduling;
 * - resource allocation;
 * - worker counts;
 * - thread counts;
 * - CPU/core counts;
 * - GPU counts;
 * - accelerator counts;
 * - QPU counts;
 * - node counts;
 * - hardware topology;
 * - device IDs;
 * - placement;
 * - routing;
 * - hardware discovery;
 * - hardware calibration;
 * - quantum IR;
 * - classical IR;
 * - QEC;
 * - ZQN;
 * - resilience;
 * - runtime dispatch.
 *
 * Those concerns remain owned by their canonical repository subsystems.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Task parallelism describes LOGICAL CONCURRENCY.
 *
 * It does NOT describe physical execution width.
 *
 * Therefore this grammar MUST NOT encode:
 *
 *     MAX_TASKS
 *     MAX_PARALLEL_TASKS
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_TASK_GROUPS
 *     MAX_PARALLELISM
 *
 * It also MUST NOT imply:
 *
 *     one task      == one thread
 *     one task      == one core
 *     one task      == one GPU
 *     one task      == one QPU
 *     one task      == one node
 *     one task      == one hardware execution unit
 *
 * A task-parallel program may therefore be realized using:
 *
 *     one execution worker;
 *     multiple CPU workers;
 *     SIMD/vector execution;
 *     GPU execution;
 *     FPGA execution;
 *     accelerator execution;
 *     quantum/classical orchestration;
 *     distributed execution;
 *     heterogeneous execution;
 *     future computational substrates.
 *
 * If fewer physical resources are available than logical tasks, the scheduler
 * MAY serialize, batch, tile, queue, or otherwise transform the execution.
 *
 * If more resources are available, the scheduler MAY exploit additional
 * parallelism.
 *
 * The source semantics remain unchanged.
 *
 * ============================================================================
 * SEMANTIC PRINCIPLE
 * ============================================================================
 *
 * Task parallelism means:
 *
 *     "these logical computations are candidates for independent/concurrent
 *      execution subject to dependencies, effects, synchronization, resource
 *      constraints, target capabilities, and execution policy."
 *
 * It does NOT mean:
 *
 *     "all tasks must physically execute simultaneously."
 *
 * Consequently:
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
 *     semantic analysis
 *       |
 *       +--> type analysis
 *       +--> effect analysis
 *       +--> dependency analysis
 *       +--> resource analysis
 *       |
 *       v
 *     canonical program IR
 *       |
 *       +--> optimization
 *       +--> scheduling
 *       +--> routing
 *       +--> target lowering
 *       |
 *       v
 *     runtime
 *
 * This grammar participates only in the syntax stage.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Canonical task primitives are owned by:
 *
 *     tasks.g4
 *
 * Generic parallel constructs are owned by:
 *
 *     parallel.g4
 *
 * This file imports those grammar domains rather than redefining their rules.
 *
 * Expected canonical parser rules include:
 *
 *     expression
 *     blockExpression
 *     statement
 *
 * from the main Zamani parser composition.
 *
 * Expected task rules supplied by Tasks include:
 *
 *     taskSpawnExpression
 *     taskAwaitExpression
 *     taskSpawnStatement
 *     taskAwaitStatement
 *     taskParallelExpression
 *
 * Expected generic parallel rules supplied by Parallel include the generic
 * parallel boundary used by the repository's parser composition.
 *
 * This file MUST NOT redefine those rules.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * No lexer rules are declared here.
 *
 * Lexical ownership remains with Zamani's canonical lexer.
 *
 * This prevents task-parallel.g4 from silently introducing a second keyword
 * namespace.
 *
 * In particular, this file MUST NOT introduce new lexer tokens merely for:
 *
 *     TASK_GROUP
 *     TASK_JOIN
 *     TASK_DEPENDENCY
 *     TASK_PARALLEL
 *     WORKER
 *     THREAD
 *     CORE
 *     GPU
 *     QPU
 *     NODE
 *
 * The syntax is intentionally composed from the existing task and parallel
 * vocabulary.
 *
 * ============================================================================
 * RESOURCE INDEPENDENCE
 * ============================================================================
 *
 * There is no fixed task capacity.
 *
 * There is no fixed dependency count.
 *
 * There is no fixed group size.
 *
 * There is no fixed nesting depth represented by this grammar.
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * Therefore logical task-parallel structure is bounded only by:
 *
 *     source representation;
 *     parser implementation;
 *     compiler resources;
 *     runtime resources;
 *     target capabilities;
 *     physical resources.
 *
 * Those are implementation/environment constraints, not language-level
 * task-parallel limits.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve enough structure to distinguish:
 *
 *     TaskParallelRegion
 *     TaskParallelGroup
 *     TaskParallelItem
 *     TaskDependency
 *     TaskJoin
 *     TaskParallelIteration
 *     TaskParallelComposition
 *
 * The AST MUST preserve:
 *
 *     source order;
 *     source locations;
 *     nesting;
 *     dependency expressions;
 *     task operands;
 *     structured-region boundaries.
 *
 * The AST MUST NOT manufacture:
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
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     whether task operands are valid;
 *     whether dependencies refer to valid computations;
 *     whether task ordering is legal;
 *     whether effects permit concurrency;
 *     whether shared state creates conflicts;
 *     whether synchronization is required;
 *     whether cancellation semantics apply;
 *     whether resource requirements can be satisfied;
 *     whether a task-parallel transformation preserves observable semantics.
 *
 * This grammar does not perform those checks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * task-parallel.g4 MUST NOT define an IR.
 *
 * The frontend lowers parsed structures into the repository's canonical
 * semantic representation.
 *
 * The resulting semantic representation may subsequently be lowered into:
 *
 *     classical execution;
 *     quantum/classical execution;
 *     accelerator execution;
 *     distributed execution;
 *     hardware execution.
 *
 * Quantum computation MUST ultimately use the canonical quantum semantic
 * boundary rather than a task-parallel quantum representation.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * The runtime receives a compiled semantic/execution representation.
 *
 * The runtime decides:
 *
 *     how many workers to use;
 *     whether work is local or remote;
 *     whether tasks are queued;
 *     whether work is batched;
 *     whether execution is serialized;
 *     how resources are shared.
 *
 * None of those decisions belong to this grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST be deterministic.
 *
 * Task ordering expressed in source MUST remain observable to semantic analysis.
 *
 * The grammar MUST NOT use semantic predicates or target-language actions to
 * inspect runtime state.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 1. PUBLIC DOMAIN ENTRY POINT
 * ============================================================================
 *
 * Stable entry point for the task-parallel grammar.
 *
 * Parser composition should normally integrate this rule rather than depending
 * directly on internal productions.
 */

taskParallelConstruct
    : taskParallelRegion
    | taskParallelGroup
    | taskParallelIteration
    | taskParallelComposition
    | taskParallelDependency
    | taskParallelJoin
    ;


/*
 * ============================================================================
 * 2. TASK-PARALLEL REGION
 * ============================================================================
 *
 * A task-parallel region establishes a structured scope in which task work may
 * be exposed for concurrent realization.
 *
 * Generic parallel ownership remains in parallel.g4.
 *
 * This rule specializes the contents of the region for task-oriented work.
 */

taskParallelRegion
    : PARALLEL
      LBRACE
      taskParallelElement*
      RBRACE
    ;


/*
 * ============================================================================
 * 3. TASK-PARALLEL ELEMENT
 * ============================================================================
 *
 * A task-parallel region may contain:
 *
 *     task creation;
 *     task completion dependencies;
 *     ordinary computation;
 *     explicit dependency relations;
 *     nested task-parallel structures.
 *
 * Ordinary statements remain owned by the canonical statement grammar.
 */

taskParallelElement
    : taskParallelWork
    | taskParallelDependency
    | taskParallelJoin
    | taskParallelComposition
    ;


/*
 * ============================================================================
 * 4. TASK-PARALLEL WORK
 * ============================================================================
 *
 * Generic task creation is imported from tasks.g4.
 *
 * This rule does not redefine spawn.
 */

taskParallelWork
    : taskSpawnExpression
    | taskParallelExpression
    ;


/*
 * ============================================================================
 * 5. TASK-PARALLEL EXPRESSION
 * ============================================================================
 *
 * A task-parallel expression is a task-oriented computation exposed to the
 * parallel semantic layer.
 *
 * The underlying computation remains an ordinary Zamani expression.
 */

taskParallelExpression
    : PARALLEL
      taskParallelBody
    ;


taskParallelBody
    : blockExpression
    | expression
    ;


/*
 * ============================================================================
 * 6. TASK-PARALLEL GROUP
 * ============================================================================
 *
 * A group provides a structural boundary for a collection of task operations.
 *
 * It does not define a runtime task-group object.
 *
 * The AST may represent this as a logical TaskParallelGroup.
 */

taskParallelGroup
    : PARALLEL
      LBRACE
      taskParallelGroupElement*
      RBRACE
    ;


taskParallelGroupElement
    : taskParallelTask
    | taskParallelDependency
    | taskParallelJoin
    | taskParallelNestedGroup
    ;


taskParallelNestedGroup
    : taskParallelGroup
    ;


/*
 * ============================================================================
 * 7. TASK-PARALLEL TASK
 * ============================================================================
 *
 * Reuses the canonical spawn expression.
 *
 * This is important:
 *
 *     task-parallel.g4
 *         DOES NOT own spawn.
 *
 *     tasks.g4
 *         OWNS spawn.
 */

taskParallelTask
    : taskSpawnExpression
    ;


/*
 * ============================================================================
 * 8. TASK-PARALLEL TASK STATEMENT
 * ============================================================================
 *
 * Statement-level adapter for task creation.
 */

taskParallelTaskStatement
    : taskSpawnStatement
    ;


/*
 * ============================================================================
 * 9. TASK-PARALLEL AWAIT
 * ============================================================================
 *
 * Await remains owned by tasks.g4.
 *
 * This adapter allows it to participate in a task-parallel region.
 */

taskParallelAwait
    : taskAwaitExpression
    ;


taskParallelAwaitStatement
    : taskAwaitStatement
    ;


/*
 * ============================================================================
 * 10. TASK-PARALLEL JOIN
 * ============================================================================
 *
 * A join is represented through one or more existing await operations.
 *
 * The grammar deliberately does not create a fixed-size join construct.
 *
 * An arbitrary number of dependencies may be represented by repetition.
 *
 * Semantic analysis determines whether the resulting dependencies form a
 * valid completion boundary.
 */

taskParallelJoin
    : taskParallelAwait+
    ;


/*
 * ============================================================================
 * 11. TASK-PARALLEL DEPENDENCY
 * ============================================================================
 *
 * A dependency expresses that one task computation depends upon another
 * computation.
 *
 * The operands remain ordinary expressions.
 *
 * No runtime task identifier is embedded in the grammar.
 */

taskParallelDependency
    : taskParallelDependencyExpression
    ;


taskParallelDependencyExpression
    : taskParallelDependencySource
      taskParallelDependencyOperator
      taskParallelDependencyTarget
    ;


taskParallelDependencySource
    : expression
    ;


taskParallelDependencyTarget
    : expression
    ;


/*
 * ============================================================================
 * 12. DEPENDENCY OPERATOR
 * ============================================================================
 *
 * Dependency syntax is deliberately represented using the existing comparison/
 * relation vocabulary rather than introducing target-specific dependency
 * objects.
 *
 * The semantic layer determines the exact dependency meaning.
 */

taskParallelDependencyOperator
    : DEPENDS
    ;


/*
 * ============================================================================
 * 13. TASK-PARALLEL COMPOSITION
 * ============================================================================
 *
 * Composition permits multiple logical task computations to participate in
 * one structured task-parallel expression.
 *
 * The number of tasks is unbounded by the grammar.
 */

taskParallelComposition
    : PARALLEL
      LBRACE
      taskParallelCompositionElement+
      RBRACE
    ;


taskParallelCompositionElement
    : taskParallelTask
    | taskParallelAwait
    | taskParallelDependency
    | taskParallelGroup
    | taskParallelIteration
    ;


/*
 * ============================================================================
 * 14. TASK-PARALLEL ITERATION
 * ============================================================================
 *
 * Task parallelism may be expressed over a logical iteration domain.
 *
 * The domain remains an ordinary expression.
 *
 * Therefore the grammar does not assume:
 *
 *     array size;
 *     task count;
 *     worker count;
 *     partition count;
 *     thread count;
 *     processor count.
 *
 * The actual decomposition is downstream.
 */

taskParallelIteration
    : PARALLEL
      FOR
      pattern
      IN
      expression
      taskParallelIterationBody
    ;


taskParallelIterationBody
    : blockExpression
    ;


/*
 * ============================================================================
 * 15. TASK-PARALLEL ITERATION TASK
 * ============================================================================
 *
 * Each logical iteration may expose work to the task-parallel semantic layer.
 *
 * This does not mean one physical runtime task must be created per iteration.
 */

taskParallelIterationTask
    : taskParallelIteration
    ;


/*
 * ============================================================================
 * 16. TASK-PARALLEL NESTING
 * ============================================================================
 *
 * Nested task parallelism is legal.
 *
 * No finite nesting depth is encoded.
 *
 * Resource realization remains a scheduler/runtime concern.
 */

taskParallelNested
    : taskParallelConstruct
    ;


/*
 * ============================================================================
 * 17. TASK-PARALLEL REGION ELEMENT WITH NESTING
 * ============================================================================
 */

taskParallelNestedElement
    : taskParallelTask
    | taskParallelAwait
    | taskParallelDependency
    | taskParallelNested
    ;


/*
 * ============================================================================
 * 18. TASK-PARALLEL SEQUENCE
 * ============================================================================
 *
 * Source order is preserved.
 *
 * Whether two operations can actually execute concurrently is determined by
 * semantic dependency/effect analysis.
 */

taskParallelSequence
    : taskParallelElement+
    ;


/*
 * ============================================================================
 * 19. TASK-PARALLEL WORK SET
 * ============================================================================
 *
 * An arbitrary number of logical tasks can be represented.
 *
 * There is deliberately no fixed maximum.
 */

taskParallelWorkSet
    : taskParallelTask+
    ;


/*
 * ============================================================================
 * 20. TASK-PARALLEL DEPENDENCY SET
 * ============================================================================
 *
 * Dependency expressions are repeatable and therefore scalable.
 */

taskParallelDependencySet
    : taskParallelDependency+
    ;


/*
 * ============================================================================
 * 21. TASK-PARALLEL COMPLETION BOUNDARY
 * ============================================================================
 *
 * A structured completion boundary consists of zero or more await operations.
 *
 * The semantic layer determines whether an empty boundary is meaningful.
 */

taskParallelCompletion
    : taskParallelAwait*
    ;


/*
 * ============================================================================
 * 22. TASK-PARALLEL BODY
 * ============================================================================
 *
 * This is the canonical body integration boundary.
 */

taskParallelTaskBody
    : blockExpression
    ;


/*
 * ============================================================================
 * 23. TASK-PARALLEL OPERAND
 * ============================================================================
 *
 * The operand remains open-ended so future Zamani computation domains can
 * participate without modifying this grammar.
 *
 * Possible semantic operands include:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     hardware operation;
 *     accelerator computation;
 *     distributed computation;
 *     AI computation;
 *     data transformation.
 *
 * None of those domains become grammar dependencies here.
 */

taskParallelOperand
    : expression
    | blockExpression
    ;


/*
 * ============================================================================
 * 24. TASK-PARALLEL NESTED BODY
 * ============================================================================
 */

taskParallelNestedBody
    : taskParallelOperand
    | taskParallelGroup
    | taskParallelIteration
    ;


/*
 * ============================================================================
 * 25. TASK-PARALLEL ROOT EXPRESSION
 * ============================================================================
 *
 * Stable expression-level integration point.
 */

taskParallelRootExpression
    : taskParallelConstruct
    ;


/*
 * ============================================================================
 * 26. TASK-PARALLEL ROOT STATEMENT
 * ============================================================================
 *
 * Stable statement-level integration point.
 *
 * The optional semicolon follows the existing Zamani statement convention.
 */

taskParallelRootStatement
    : taskParallelConstruct
      SEMI?
    ;


/*
 * ============================================================================
 * 27. TASK-PARALLEL TASK COLLECTION
 * ============================================================================
 *
 * A collection is logical.
 *
 * It does not imply a particular runtime collection implementation.
 */

taskParallelTaskCollection
    : taskParallelTask+
    ;


/*
 * ============================================================================
 * 28. TASK-PARALLEL DEPENDENCY COLLECTION
 * ============================================================================
 */

taskParallelDependencyCollection
    : taskParallelDependency+
    ;


/*
 * ============================================================================
 * 29. TASK-PARALLEL STRUCTURED REGION
 * ============================================================================
 *
 * Named boundary for AST and semantic tooling.
 */

taskParallelStructuredRegion
    : PARALLEL
      LBRACE
      taskParallelStructuredElement*
      RBRACE
    ;


taskParallelStructuredElement
    : taskParallelTask
    | taskParallelAwait
    | taskParallelDependency
    | taskParallelGroup
    | taskParallelIteration
    | taskParallelStructuredRegion
    ;


/*
 * ============================================================================
 * 30. TASK-PARALLEL COMPLETION GROUP
 * ============================================================================
 *
 * A completion group can contain arbitrary completion dependencies.
 */

taskParallelCompletionGroup
    : taskParallelAwait+
    ;


/*
 * ============================================================================
 * 31. TASK-PARALLEL DEPENDENCY CHAIN
 * ============================================================================
 *
 * A dependency chain is structural syntax only.
 *
 * The semantic layer determines whether the chain is acyclic, meaningful, and
 * compatible with the effects of the referenced computations.
 */

taskParallelDependencyChain
    : taskParallelDependency
      (
          taskParallelDependency
      )*
    ;


/*
 * ============================================================================
 * 32. TASK-PARALLEL DAG INPUT
 * ============================================================================
 *
 * The grammar does not construct a graph directly.
 *
 * It merely preserves enough structure for semantic analysis to construct the
 * canonical dependency graph.
 */

taskParallelDagInput
    : taskParallelTaskCollection
      taskParallelDependencyCollection?
    ;


/*
 * ============================================================================
 * 33. TASK-PARALLEL SCOPE
 * ============================================================================
 *
 * Named semantic boundary for scope analysis.
 */

taskParallelScope
    : PARALLEL
      blockExpression
    ;


/*
 * ============================================================================
 * 34. TASK-PARALLEL REGION ITEM
 * ============================================================================
 */

taskParallelRegionItem
    : taskParallelTask
    | taskParallelAwait
    | taskParallelDependency
    | taskParallelScope
    | taskParallelIteration
    ;


/*
 * ============================================================================
 * 35. FINAL PUBLIC COMPOSITION RULE
 * ============================================================================
 *
 * Parser composition should use this rule when a single stable task-parallel
 * entry point is required.
 */

taskParallel
    : taskParallelConstruct
    ;


/*
 * ============================================================================
 * ARCHITECTURAL INVARIANTS
 * ============================================================================
 *
 * The following invariants are mandatory:
 *
 * 1. This grammar contains no Rust actions.
 *
 * 2. This grammar contains no unsafe code.
 *
 * 3. This grammar contains no fixed resource counts.
 *
 * 4. This grammar contains no hardware IDs.
 *
 * 5. This grammar contains no physical topology.
 *
 * 6. This grammar contains no scheduler implementation.
 *
 * 7. This grammar contains no runtime implementation.
 *
 * 8. This grammar contains no quantum IR.
 *
 * 9. This grammar contains no QEC implementation.
 *
 * 10. This grammar contains no ZQN implementation.
 *
 * 11. This grammar contains no resource discovery.
 *
 * 12. This grammar reuses task syntax from tasks.g4.
 *
 * 13. This grammar specializes, rather than replaces, parallel.g4.
 *
 * 14. Ordinary expressions remain owned by the canonical expression grammar.
 *
 * 15. Ordinary blocks remain owned by the canonical block grammar.
 *
 * 16. Function declarations remain owned by the functions grammar.
 *
 * 17. Async/future semantics remain owned by functions/futures grammar layers.
 *
 * 18. Scheduling remains outside grammar/.
 *
 * 19. Hardware realization remains outside grammar/.
 *
 * 20. The semantic meaning of a program remains independent of the physical
 *     number of execution resources.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     numeric resource limits;
 *     worker-count literals used as language limits;
 *     thread-count limits;
 *     processor-count limits;
 *     GPU-count limits;
 *     QPU-count limits;
 *     node-count limits;
 *     fixed task capacities;
 *     fixed dependency capacities;
 *     fixed nesting limits;
 *     fixed topology;
 *     physical device identifiers.
 *
 * Repetition operators:
 *
 *     *
 *     +
 *
 * are semantic grammar repetition and are NOT resource limits.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Classical:
 *     Task operands may contain ordinary classical computation.
 *
 * Quantum:
 *     Task operands may eventually contain quantum constructs, but this file
 *     does not define quantum semantics. Quantum lowering remains owned by the
 *     canonical quantum semantic boundary.
 *
 * Hybrid:
 *     A task may contain hybrid computation without requiring this grammar to
 *     understand the internal hybrid representation.
 *
 * HDL:
 *     Hardware-oriented computations may participate through ordinary semantic
 *     expressions/blocks where the language permits them.
 *
 * Distributed:
 *     Distributed realization is selected downstream from the logical task
 *     representation.
 *
 * AI/data:
 *     AI and data computations may become task operands without special
 *     task-parallel grammar dependencies.
 *
 * Hardware:
 *     Hardware capability selection belongs to resources/hardware/target
 *     layers, not this file.
 *
 * Resilience:
 *     Runtime recovery may adapt execution after compilation; resilience does
 *     not become a grammar dependency.
 *
 * ============================================================================
 * EXPECTED COMPILER PIPELINE
 * ============================================================================
 *
 *     task-parallel.g4
 *             |
 *             v
 *     Frontend AST
 *             |
 *             v
 *     Name/type/effect analysis
 *             |
 *             v
 *     Dependency analysis
 *             |
 *             v
 *     Resource/capability analysis
 *             |
 *             v
 *     Canonical semantic IR
 *             |
 *       +-----+------+----------------+
 *       |            |                |
 *       v            v                v
 *   optimize     schedule          route
 *       |            |                |
 *       +------------+----------------+
 *                    |
 *                    v
 *              target lowering
 *                    |
 *                    v
 *                 runtime
 *
 * ============================================================================
 */