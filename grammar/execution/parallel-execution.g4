/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/parallel-execution.g4
 *
 * Grammar:
 *     ParallelExecution
 *
 * Purpose:
 *     Defines the execution-layer syntax for expressing parallel execution
 *     intent while preserving the separation between:
 *
 *         language semantics
 *         concurrency semantics
 *         scheduling
 *         placement
 *         resource allocation
 *         hardware realization
 *         runtime dispatch
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     concurrency grammar       execution/parallel-execution.g4
 *          |                              |
 *          +--------------+---------------+
 *                         |
 *                         v
 *                    frontend AST
 *                         |
 *                         v
 *                  semantic analysis
 *                         |
 *          +--------------+---------------+
 *          |              |               |
 *          v              v               v
 *       effects        resources       dependencies
 *          |              |               |
 *          +--------------+---------------+
 *                         |
 *                         v
 *                   canonical IR
 *                         |
 *                         v
 *                    optimization
 *                         |
 *                         v
 *                     routing
 *                         |
 *                         v
 *                    scheduling
 *                         |
 *                         v
 *                     placement
 *                         |
 *                         v
 *                 execution dispatch
 *                         |
 *                         v
 *                      runtime
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - execution-layer parallel intent;
 *   - parallel execution scopes;
 *   - execution-level parallel composition;
 *   - execution-level parallel expressions;
 *   - optional execution properties attached to a parallel construct;
 *   - the syntactic boundary between parallel intent and execution control.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - general concurrency;
 *   - task creation;
 *   - futures;
 *   - actors;
 *   - channels;
 *   - synchronization algorithms;
 *   - data-parallel semantics;
 *   - task-parallel semantics;
 *   - loops generally;
 *   - scheduling algorithms;
 *   - resource allocation;
 *   - placement;
 *   - routing;
 *   - hardware discovery;
 *   - target selection;
 *   - runtime dispatch;
 *   - classical IR;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - resilience.
 *
 * Those concepts remain owned by their canonical subsystems.
 *
 * ============================================================================
 *
 * RELATIONSHIP TO EXISTING CONCURRENCY GRAMMAR
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/concurrency/parallel.g4
 *
 * owns the language-level parallel-computation boundary.
 *
 * Existing:
 *
 *     grammar/concurrency/task-parallel.g4
 *
 * owns task-parallel-specific syntax.
 *
 * Existing:
 *
 *     grammar/concurrency/data-parallel.g4
 *
 * owns data-parallel-specific syntax.
 *
 * Existing:
 *
 *     grammar/concurrency/tasks.g4
 *
 * owns task-oriented concurrency.
 *
 * Existing:
 *
 *     grammar/concurrency/futures.g4
 *
 * owns future/await semantics.
 *
 * This file MUST NOT duplicate those grammars.
 *
 * Instead, it provides the execution-layer composition point.
 *
 * ============================================================================
 *
 * RELATIONSHIP TO EXECUTION GRAMMAR
 * ============================================================================
 *
 * `grammar/execution/execution.g4` owns execution constructs generally.
 *
 * This file provides:
 *
 *     parallelExecutionConstruct
 *
 * which `execution.g4` may include as one of its execution forms.
 *
 * Example composition:
 *
 *     execution
 *         |
 *         +-- sequential execution
 *         +-- asynchronous execution
 *         +-- parallel execution
 *         +-- distributed execution
 *         +-- scheduled execution
 *         +-- dispatched execution
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Parallel syntax expresses:
 *
 *     "these computations may participate in concurrent execution"
 *
 * It does NOT express:
 *
 *     "create exactly N workers"
 *
 *     "use exactly N CPU cores"
 *
 *     "use exactly N GPUs"
 *
 *     "use exactly N QPUs"
 *
 *     "assign one task to one core"
 *
 *     "execute on device X"
 *
 *     "use topology Y"
 *
 *     "reserve machine Z"
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * This grammar deliberately has no machine-scale constants.
 *
 * It contains no:
 *
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_QPUS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_TASKS
 *     MAX_ITEMS
 *     MAX_PARALLELISM
 *
 * The amount of parallel work is determined by program semantics.
 *
 * The amount of actual parallel execution is determined downstream by:
 *
 *     available resources
 *     capabilities
 *     dependencies
 *     effects
 *     memory constraints
 *     scheduling policy
 *     placement policy
 *     target capabilities
 *     runtime state
 *
 * ============================================================================
 *
 * IMPORTANT SEMANTIC RULE
 * ============================================================================
 *
 * Parallelism is an opportunity unless the language semantics explicitly
 * establish a stronger guarantee.
 *
 * Therefore:
 *
 *     parallel { A; B; C; }
 *
 * does NOT inherently require simultaneous execution.
 *
 * A valid target may execute:
 *
 *     A
 *     B
 *     C
 *
 * concurrently,
 *
 * or:
 *
 *     A -> B -> C
 *
 * sequentially,
 *
 * provided the resulting behavior satisfies the program's semantic
 * requirements.
 *
 * This permits one program to scale from a single execution resource to
 * arbitrarily many available resources.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * The grammar is declarative.
 *
 * It contains:
 *
 *     - no Rust actions;
 *     - no semantic predicates;
 *     - no runtime calls;
 *     - no resource discovery;
 *     - no hardware discovery;
 *     - no filesystem access;
 *     - no network access;
 *     - no mutable global state;
 *     - no randomness.
 *
 * ============================================================================
 *
 * RUST
 * ============================================================================
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * This grammar contains no Rust code.
 *
 * Zamani compiler/runtime implementation remains safe Rust.
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * ============================================================================
 *
 * LEXER CONTRACT
 * ============================================================================
 *
 * This parser consumes the canonical PARALLEL token.
 *
 * The canonical lexer owns the spelling of:
 *
 *     parallel
 *
 * This file does not declare lexer rules.
 *
 * No worker/thread/core/device keyword is introduced here.
 *
 * ============================================================================
 *
 * CORE GRAMMAR CONTRACT
 * ============================================================================
 *
 * The composed parser supplies canonical rules such as:
 *
 *     expression
 *     statement
 *     blockExpression
 *     pattern
 *     identifier
 *     qualifiedName
 *
 * This file MUST NOT redefine those rules.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve enough structure to distinguish:
 *
 *     ParallelExecution
 *     ParallelExecutionBody
 *     ParallelExecutionItem
 *     ParallelExecutionProperty
 *
 * Source locations must be preserved.
 *
 * The AST MUST NOT contain:
 *
 *     worker IDs
 *     thread IDs
 *     CPU IDs
 *     GPU IDs
 *     QPU IDs
 *     node IDs
 *     physical resource assignments
 *
 * merely because parallel syntax occurs.
 *
 * ============================================================================
 *
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether a construct is safely parallelizable;
 *     - whether dependencies permit concurrent execution;
 *     - whether effects permit concurrent execution;
 *     - whether memory accesses conflict;
 *     - whether synchronization is required;
 *     - whether the operation is deterministic;
 *     - whether a target can realize the requested parallelism;
 *     - whether serialization is permitted.
 *
 * ============================================================================
 *
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does not create a parallel IR.
 *
 * The semantic layer lowers parallel intent into the appropriate canonical
 * representation.
 *
 * For quantum computation:
 *
 *     source
 *       -> quantum syntax
 *       -> quantum::ir
 *       -> optimization/routing
 *       -> scheduling
 *
 * For classical computation:
 *
 *     source
 *       -> semantic representation
 *       -> classical IR
 *       -> optimization
 *       -> scheduling
 *
 * For heterogeneous computation:
 *
 *     source
 *       -> canonical semantic representation
 *       -> domain-specific lowering
 *       -> scheduling/placement
 *       -> execution
 *
 * ============================================================================
 *
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This grammar never allocates resources.
 *
 * Parallel intent may create resource demand during semantic analysis.
 *
 * Resource analysis determines actual demand.
 *
 * Resource management determines availability.
 *
 * Scheduling determines when work executes.
 *
 * Placement determines where work executes.
 *
 * Runtime dispatch determines how it is launched.
 *
 * ============================================================================
 *
 * HARDWARE CONTRACT
 * ============================================================================
 *
 * No hardware architecture is assumed.
 *
 * A parallel construct may eventually execute on:
 *
 *     CPU
 *     multicore CPU
 *     SIMD/vector hardware
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     quantum/classical hybrid hardware
 *     distributed systems
 *     cloud resources
 *     embedded systems
 *     future execution substrates
 *
 * The grammar does not know which one.
 *
 * ============================================================================
 *
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Parallel execution may surround quantum computation, but this grammar does
 * not represent:
 *
 *     qubits
 *     gates
 *     physical qubits
 *     coupling maps
 *     pulse schedules
 *     calibration
 *
 * Those remain owned by quantum::ir, routing, scheduling and hardware
 * subsystems.
 *
 * ============================================================================
 *
 * HDL CONTRACT
 * ============================================================================
 *
 * Parallel execution may represent execution intent around hardware-level
 * computation, but this grammar does not define:
 *
 *     clocks
 *     wires
 *     registers
 *     physical timing
 *     FPGA resources
 *     ASIC cells
 *
 * Those belong to HDL/hardware grammars and their semantic layers.
 *
 * ============================================================================
 *
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Parallel execution does not imply one process per node.
 *
 * Distributed placement is resolved downstream.
 *
 * ============================================================================
 *
 * RESILIENCE CONTRACT
 * ============================================================================
 *
 * Resilience may later adapt execution after failures.
 *
 * This grammar does not define:
 *
 *     retry;
 *     recovery;
 *     rollback;
 *     checkpointing;
 *     failover.
 *
 * Those remain resilience/execution-lifecycle concerns.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable execution-domain entry point.
 *
 * `execution.g4` should consume this rule rather than duplicate the complete
 * parallel execution syntax.
 *
 * ============================================================================
 */

parallelExecutionConstruct
    : parallelExecutionRegion
    | parallelExecutionExpression
    ;


/*
 * ============================================================================
 * 2. PARALLEL EXECUTION REGION
 * ============================================================================
 *
 * Canonical structured form:
 *
 *     parallel {
 *         work_a();
 *         work_b();
 *         work_c();
 *     }
 *
 * The body is a normal language block.
 *
 * This is intentional:
 *
 *     - no duplicate statement grammar;
 *     - no duplicate expression grammar;
 *     - no duplicate task grammar;
 *     - no duplicate loop grammar.
 *
 * ============================================================================
 */

parallelExecutionRegion
    : PARALLEL blockExpression
    ;


/*
 * ============================================================================
 * 3. PARALLEL EXECUTION EXPRESSION
 * ============================================================================
 *
 * Allows parallel execution to operate on an ordinary expression.
 *
 * Example:
 *
 *     parallel compute(data)
 *
 * The expression's semantics determine what may actually execute in parallel.
 *
 * ============================================================================
 */

parallelExecutionExpression
    : PARALLEL expression
    ;


/*
 * ============================================================================
 * 4. OPTIONAL SUBJECT FORM
 * ============================================================================
 *
 * Provides a stable extension point for execution contexts where the program
 * identifies a semantic execution unit.
 *
 * Example conceptual form:
 *
 *     parallel task_group {
 *         ...
 *     }
 *
 * The identifier is not a hardware identifier.
 *
 * ============================================================================
 */

parallelExecutionSubject
    : qualifiedName
    ;


/*
 * ============================================================================
 * 5. NAMED PARALLEL EXECUTION REGION
 * ============================================================================
 *
 * This rule is intentionally separate from `parallelExecutionRegion` so AST
 * and semantic tooling can distinguish a named execution scope from an
 * anonymous one.
 *
 * ============================================================================
 */

namedParallelExecutionRegion
    : PARALLEL
      parallelExecutionSubject
      blockExpression
    ;


/*
 * ============================================================================
 * 6. PARALLEL EXECUTION ITEM
 * ============================================================================
 *
 * A parallel region contains ordinary language constructs.
 *
 * This adapter exists only as an explicit semantic boundary.
 *
 * ============================================================================
 */

parallelExecutionItem
    : statement
    | expression
    ;


/*
 * ============================================================================
 * 7. EXPLICIT PARALLEL ITEM LIST
 * ============================================================================
 *
 * Useful to downstream AST tooling that needs a normalized collection of
 * execution candidates.
 *
 * The parser does not infer independence.
 *
 * ============================================================================
 */

parallelExecutionItems
    : parallelExecutionItem+
    ;


/*
 * ============================================================================
 * 8. OPTIONAL PARALLEL ITEM LIST
 * ============================================================================
 *
 * Empty parallel scopes are syntactically valid.
 *
 * This supports:
 *
 *     incremental compilation;
 *     macros;
 *     generated source;
 *     tooling;
 *     transformations.
 *
 * ============================================================================
 */

parallelExecutionItemsOptional
    : parallelExecutionItem*
    ;


/*
 * ============================================================================
 * 9. PARALLEL EXECUTION PROPERTY BLOCK
 * ============================================================================
 *
 * Execution properties are intentionally generic.
 *
 * This permits future execution semantics without embedding a closed list of
 * hardware concepts into the grammar.
 *
 * Example:
 *
 *     parallel {
 *         ...
 *     } with {
 *         property: value;
 *     }
 *
 * The enclosing execution grammar may choose whether this form is exposed
 * publicly.
 *
 * ============================================================================
 */

parallelExecutionPropertyBlock
    : LBRACE
      parallelExecutionPropertyEntry*
      RBRACE
    ;


/*
 * ============================================================================
 * 10. PROPERTY ENTRY
 * ============================================================================
 *
 * Property names are qualified names.
 *
 * This enables:
 *
 *     standard properties
 *     domain properties
 *     future properties
 *     dialect properties
 *     organization-specific extensions
 *
 * without requiring a new parser keyword for every addition.
 *
 * ============================================================================
 */

parallelExecutionPropertyEntry
    : qualifiedName
      parallelExecutionPropertyAssignment
      SEMI
    ;


/*
 * ============================================================================
 * 11. PROPERTY ASSIGNMENT
 * ============================================================================
 *
 * Both forms are supported:
 *
 *     property: value;
 *
 *     property = value;
 *
 * The value is a canonical expression.
 *
 * ============================================================================
 */

parallelExecutionPropertyAssignment
    : COLON expression
    | ASSIGN expression
    ;


/*
 * ============================================================================
 * 12. PARALLEL EXECUTION WITH PROPERTIES
 * ============================================================================
 *
 * This is an explicit execution-level extension form.
 *
 * The properties remain metadata/intent.
 *
 * They do not directly control a runtime scheduler.
 *
 * ============================================================================
 */

parallelExecutionWithProperties
    : PARALLEL
      blockExpression
      parallelExecutionPropertyBlock
    ;


/*
 * ============================================================================
 * 13. PARALLEL EXECUTION SUBJECT WITH PROPERTIES
 * ============================================================================
 *
 * Named semantic execution scope plus execution properties.
 *
 * ============================================================================
 */

namedParallelExecutionWithProperties
    : PARALLEL
      parallelExecutionSubject
      blockExpression
      parallelExecutionPropertyBlock
    ;


/*
 * ============================================================================
 * 14. EXECUTION PARALLEL FORM
 * ============================================================================
 *
 * Combined public form.
 *
 * The alternatives are intentionally explicit so the AST can retain the
 * difference between:
 *
 *     anonymous region;
 *     named region;
 *     region with properties;
 *     named region with properties;
 *     expression form.
 *
 * ============================================================================
 */

parallelExecutionForm
    : parallelExecutionRegion
    | namedParallelExecutionRegion
    | parallelExecutionWithProperties
    | namedParallelExecutionWithProperties
    | parallelExecutionExpression
    ;


/*
 * ============================================================================
 * 15. CONCURRENCY ADAPTER
 * ============================================================================
 *
 * The existing concurrency subsystem owns the detailed parallel-computation
 * semantics.
 *
 * This rule provides a semantic integration point rather than redefining
 * those constructs.
 *
 * The enclosing parser should import/use the concurrency grammar when the
 * concrete parser architecture requires it.
 *
 * ============================================================================
 */

parallelConcurrencyAdapter
    : parallelExecutionForm
    ;


/*
 * ============================================================================
 * 16. DATA-PARALLEL ADAPTER
 * ============================================================================
 *
 * Data-parallel constructs remain owned by:
 *
 *     grammar/concurrency/data-parallel.g4
 *
 * This rule intentionally accepts an ordinary expression so the execution
 * layer does not duplicate data-parallel syntax.
 *
 * ============================================================================
 */

parallelDataExecution
    : PARALLEL expression
    ;


/*
 * ============================================================================
 * 17. TASK-PARALLEL ADAPTER
 * ============================================================================
 *
 * Task-parallel constructs remain owned by:
 *
 *     grammar/concurrency/task-parallel.g4
 *
 * This adapter permits task-parallel execution to enter the execution layer
 * without defining another task language.
 *
 * ============================================================================
 */

parallelTaskExecution
    : PARALLEL expression
    ;


/*
 * ============================================================================
 * 18. ASYNCHRONOUS PARALLEL ADAPTER
 * ============================================================================
 *
 * Asynchronous semantics remain owned by the concurrency/futures subsystem.
 *
 * This grammar only establishes an execution-layer composition point.
 *
 * ============================================================================
 */

parallelAsyncExecution
    : ASYNC
      parallelExecutionForm
    ;


/*
 * ============================================================================
 * 19. PARALLEL EXECUTION CANDIDATE
 * ============================================================================
 *
 * A candidate represents work exposed to downstream dependency/effect/resource
 * analysis.
 *
 * It is not a scheduler object.
 *
 * ============================================================================
 */

parallelExecutionCandidate
    : parallelExecutionItem
    ;


/*
 * ============================================================================
 * 20. PARALLEL EXECUTION GROUP
 * ============================================================================
 *
 * Structured grouping of execution candidates.
 *
 * No number of candidates is fixed.
 *
 * ============================================================================
 */

parallelExecutionGroup
    : PARALLEL
      LBRACE
      parallelExecutionCandidate*
      RBRACE
    ;


/*
 * ============================================================================
 * 21. PARALLEL EXECUTION GROUP WITH SUBJECT
 * ============================================================================
 */

parallelExecutionNamedGroup
    : PARALLEL
      parallelExecutionSubject
      LBRACE
      parallelExecutionCandidate*
      RBRACE
    ;


/*
 * ============================================================================
 * 22. PARALLEL EXECUTION BLOCK
 * ============================================================================
 *
 * Stable parser-composition alias.
 *
 * ============================================================================
 */

parallelExecutionBlock
    : parallelExecutionRegion
    ;


/*
 * ============================================================================
 * 23. PARALLEL EXECUTION VALUE
 * ============================================================================
 *
 * Stable expression-level adapter for consumers that need a single execution
 * value.
 *
 * ============================================================================
 */

parallelExecutionValue
    : parallelExecutionExpression
    ;


/*
 * ============================================================================
 * 24. PARALLEL EXECUTION SCOPE
 * ============================================================================
 *
 * A scope is a semantic boundary.
 *
 * Scope lifetime is resolved downstream.
 *
 * ============================================================================
 */

parallelExecutionScope
    : parallelExecutionForm
    ;


/*
 * ============================================================================
 * 25. PARALLEL EXECUTION ROOT
 * ============================================================================
 *
 * Canonical entry point for this grammar module.
 *
 * ============================================================================
 */

parallelExecution
    : parallelExecutionForm
    ;