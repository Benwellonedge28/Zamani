/*
 * ============================================================================
 * Zamani Programming Language
 * Production Parallel-Computation Grammar
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/parallel.g4
 *
 * Purpose:
 *     Parser-level grammar for expressing target-independent parallel
 *     computation intent in Zamani.
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *
 * Safety:
 *     This grammar contains no target-language actions.
 *     No unsafe Rust is required.
 *     The Zamani compiler/runtime MUST be implemented without unsafe Rust.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - parallel computation syntax;
 *   - parallel regions;
 *   - parallel work-item syntax;
 *   - parallel composition syntax;
 *   - syntactic parallel iteration forms;
 *   - syntactic parallel mapping/reduction forms;
 *   - parallel intent modifiers that are genuinely language-level syntax;
 *   - explicit dependency/order expressions attached to parallel work;
 *   - integration boundaries for data-parallel and task-parallel grammars.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - ordinary expressions;
 *   - ordinary statements;
 *   - loops generally;
 *   - functions generally;
 *   - types;
 *   - collections;
 *   - tensor semantics;
 *   - task implementation;
 *   - actor implementation;
 *   - channels;
 *   - synchronization algorithms;
 *   - scheduling;
 *   - resource allocation;
 *   - worker counts;
 *   - thread counts;
 *   - CPU/core counts;
 *   - GPU counts;
 *   - accelerator counts;
 *   - QPU counts;
 *   - device identifiers;
 *   - topology;
 *   - placement;
 *   - routing;
 *   - hardware discovery;
 *   - hardware calibration;
 *   - quantum IR;
 *   - classical IR;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - runtime dispatch.
 *
 * Those concepts belong to their canonical repository subsystems.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Parallel syntax expresses WHAT MAY BE EXECUTED CONCURRENTLY.
 *
 * It does not prescribe HOW MANY execution resources must be used.
 *
 * Therefore this grammar MUST NOT contain:
 *
 *     MAX_PARALLELISM
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_QPUS
 *     MAX_DEVICES
 *     MAX_TASKS
 *     MAX_ITEMS
 *
 * Nor may it encode assumptions such as:
 *
 *     one task == one thread
 *     one item == one core
 *     one GPU == one kernel
 *     one qubit == one worker
 *     one node == one process
 *
 * A parallel program can therefore be lowered to:
 *
 *     - a single execution context;
 *     - several CPU cores;
 *     - vector/SIMD execution;
 *     - GPU execution;
 *     - FPGA/accelerator execution;
 *     - quantum/classical orchestration;
 *     - distributed execution;
 *     - heterogeneous execution;
 *     - a future computational substrate.
 *
 * If the target has fewer resources than the program exposes as legal
 * parallelism, downstream scheduling may serialize work while preserving
 * semantics.
 *
 * If the target has more resources, downstream scheduling may exploit them.
 *
 * ============================================================================
 * SEMANTIC PRINCIPLE
 * ============================================================================
 *
 * The meaning of:
 *
 *     parallel { A; B; C; }
 *
 * is NOT:
 *
 *     "A, B and C must execute simultaneously."
 *
 * It means:
 *
 *     "A, B and C are exposed as candidates for concurrent execution subject
 *      to semantic dependencies, effects, resource requirements, ordering
 *      constraints, synchronization requirements, and target capabilities."
 *
 * Consequently:
 *
 *     grammar
 *         ->
 *     AST
 *         ->
 *     semantic analysis
 *         ->
 *     dependency/effect/resource analysis
 *         ->
 *     canonical IR
 *         ->
 *     optimization
 *         ->
 *     scheduling
 *         ->
 *     target lowering
 *         ->
 *     runtime
 *
 * ============================================================================
 * OWNERSHIP RELATIONSHIPS
 * ============================================================================
 *
 * concurrency.g4
 *     owns the broad concurrency-domain composition.
 *
 * tasks.g4
 *     owns task-oriented spawn/await/parallel composition.
 *
 * futures.g4
 *     owns future/await-oriented integration.
 *
 * actors.g4
 *     owns actor-specific concurrency syntax.
 *
 * channels.g4
 *     owns channel communication syntax.
 *
 * synchronization.g4
 *     owns synchronization-specific syntax.
 *
 * data-parallel.g4
 *     owns data-parallel-specific extensions.
 *
 * task-parallel.g4
 *     owns task-parallel-specific extensions.
 *
 * parallel.g4
 *     owns the COMMON PARALLEL-COMPUTATION SYNTAX BOUNDARY.
 *
 * This file therefore provides reusable parallel productions without
 * stealing ownership from those more specialized grammars.
 *
 * ============================================================================
 * CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * The composed Zamani parser supplies canonical rules including:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     pattern
 *     blockExpression
 *     statement
 *     argumentList
 *     genericArguments
 *     attribute
 *     parameterList
 *     literal
 *
 * This file MUST NOT redefine those rules.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer owns lexical spelling.
 *
 * This grammar therefore does NOT declare lexer rules.
 *
 * The primary canonical token used here is:
 *
 *     PARALLEL
 *
 * Existing ordinary language tokens may also be consumed where they already
 * belong to the canonical lexical contract, but this file MUST NOT silently
 * introduce new reserved words.
 *
 * In particular, this file does NOT invent tokens such as:
 *
 *     DATA_PARALLEL
 *     TASK_PARALLEL
 *     PARALLEL_FOR
 *     PARALLEL_MAP
 *     REDUCE_PARALLEL
 *     WORKER
 *     THREAD
 *     CORE
 *
 * Specialized vocabulary must be coordinated through the lexer/specification
 * before becoming canonical language syntax.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Recommended composition:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     ordinary language             parallel.g4
 *                                        |
 *                                        v
 *                                Frontend AST
 *                                        |
 *                                        v
 *                           semantic/effect/resource analysis
 *                                        |
 *                                        v
 *                                  canonical IR
 *                                        |
 *                              +---------+---------+
 *                              |         |         |
 *                              v         v         v
 *                         classical   quantum   distributed
 *                              |         |         |
 *                              +---------+---------+
 *                                        |
 *                                        v
 *                                  optimization
 *                                        |
 *                                        v
 *                                    scheduling
 *                                        |
 *                                        v
 *                                target lowering
 *                                        |
 *                                        v
 *                                     runtime
 *
 * `parallel.g4` MUST NOT depend directly on:
 *
 *     runtime
 *     scheduler
 *     hardware
 *     QEC
 *     ZQN
 *     resilience
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser should preserve enough structure for the AST to distinguish:
 *
 *     ParallelRegion
 *     ParallelWorkItem
 *     ParallelIteration
 *     ParallelMap
 *     ParallelReduce
 *     ParallelComposition
 *     ParallelDependency
 *
 * where those concepts are accepted by the language specification.
 *
 * The AST MUST preserve source order and source locations.
 *
 * The AST MUST NOT contain runtime-assigned worker IDs, thread IDs, device IDs,
 * physical-core IDs, or other target-specific identities merely because the
 * source contains parallel syntax.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * A parallel construct may imply:
 *
 *     parallelism
 *     dependency analysis
 *     synchronization
 *     resource demand
 *     memory interaction
 *     communication
 *     ordering constraints
 *
 * These are semantic facts.
 *
 * They MUST be derived after parsing.
 *
 * This grammar never turns a resource preference into a fixed hardware count.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * 1. PUBLIC PARALLEL DOMAIN ENTRY POINT
 * ============================================================================
 *
 * Stable integration rule for parser composition.
 *
 * Specialized grammars may consume individual productions, but composed
 * parsers SHOULD use `parallelConstruct` as the domain entry point whenever
 * practical.
 */

parallelConstruct
    : parallelExpression
    | parallelRegion
    | parallelIteration
    | parallelMap
    | parallelReduce
    | parallelComposition
    ;


/*
 * ============================================================================
 * 2. CORE PARALLEL EXPRESSION
 * ============================================================================
 *
 * This is the fundamental language-level parallel construct.
 *
 * Examples:
 *
 *     parallel {
 *         first()
 *         second()
 *         third()
 *     }
 *
 *     parallel compute()
 *
 * The operand remains a canonical expression or block expression.
 *
 * No number of workers is specified.
 */

parallelExpression
    : PARALLEL parallelBody
    ;


parallelBody
    : blockExpression
    | expression
    ;


/*
 * ============================================================================
 * 3. PARALLEL REGION
 * ============================================================================
 *
 * Named rule for a structured region of potentially concurrent computation.
 *
 * This rule is intentionally equivalent in basic syntax to the core parallel
 * expression but gives AST/semantic tooling a stable region boundary.
 */

parallelRegion
    : PARALLEL
      blockExpression
    ;


/*
 * ============================================================================
 * 4. PARALLEL STATEMENT ADAPTER
 * ============================================================================
 *
 * The canonical statement grammar may use this adapter when parallel intent
 * occurs in statement position.
 */

parallelStatement
    : parallelExpression
      SEMI?
    ;


/*
 * ============================================================================
 * 5. PARALLEL WORK ITEM
 * ============================================================================
 *
 * A work item is a syntactic unit that can participate in a parallel region.
 *
 * It is deliberately expressed through canonical statements/expressions.
 *
 * The grammar does not invent a new task or worker type.
 */

parallelWorkItem
    : parallelWorkExpression
    | parallelWorkStatement
    ;


parallelWorkExpression
    : expression
    ;


parallelWorkStatement
    : statement
    ;


/*
 * ============================================================================
 * 6. EXPLICIT PARALLEL WORK LIST
 * ============================================================================
 *
 * This provides a structured representation for parallel branches without
 * requiring the parser to interpret their execution resources.
 *
 * Example:
 *
 *     parallel {
 *         first();
 *         second();
 *         third();
 *     }
 *
 * Branch dependency analysis belongs downstream.
 */

parallelWorkList
    : parallelWorkItem+
    ;


parallelWorkListOptional
    : parallelWorkItem*
    ;


/*
 * ============================================================================
 * 7. PARALLEL COMPOSITION
 * ============================================================================
 *
 * A composition groups independent or partially independent computations.
 *
 * The parser preserves their lexical ordering.
 *
 * Semantic analysis determines which operations are actually independent.
 */

parallelComposition
    : PARALLEL
      LBRACE
      parallelWorkItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 8. PARALLEL ITERATION
 * ============================================================================
 *
 * Parallel iteration expresses the intent that iterations of a canonical
 * iteration construct may be evaluated concurrently.
 *
 * The actual iteration grammar remains owned by the statement/loop grammar.
 *
 * This rule intentionally uses an expression-based iteration source so the
 * grammar does not assume:
 *
 *     arrays
 *     vectors
 *     tensors
 *     fixed ranges
 *     fixed dimensions
 *     fixed collection sizes
 *
 * Example conceptual form:
 *
 *     parallel for item in values {
 *         compute(item)
 *     }
 *
 * The exact `for` syntax remains governed by the canonical loop grammar.
 *
 * If the canonical loop grammar provides a reusable loop production, the
 * integration layer SHOULD adapt that production rather than duplicate it.
 */

parallelIteration
    : PARALLEL
      parallelLoopBody
    ;


parallelLoopBody
    : FOR
      pattern
      IN
      expression
      blockExpression
    ;


/*
 * ============================================================================
 * 9. PARALLEL ITERATION EXPRESSION
 * ============================================================================
 *
 * Expression-form parallel iteration.
 *
 * This rule is intentionally separate from statement-form iteration so the
 * semantic layer can distinguish a value-producing parallel computation from
 * a side-effecting parallel statement.
 */

parallelIterationExpression
    : PARALLEL
      FOR
      pattern
      IN
      expression
      blockExpression
    ;


/*
 * ============================================================================
 * 10. PARALLEL MAP
 * ============================================================================
 *
 * Map expresses independent application of a computation over an input
 * collection/range/stream/value domain.
 *
 * No collection implementation is assumed.
 *
 * The input is an ordinary expression.
 *
 * Example conceptual form:
 *
 *     parallel map values with compute
 *
 * The concrete keyword form is deliberately represented through existing
 * expression syntax rather than introducing a new lexer keyword.
 *
 * This rule is reserved for integration with the canonical call/member
 * expression model.
 */

parallelMap
    : PARALLEL
      MAP
      LPAREN
      expression
      COMMA
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 11. PARALLEL REDUCE
 * ============================================================================
 *
 * Reduction combines independently computed partial results.
 *
 * The reduction operator/function remains an ordinary expression.
 *
 * Example conceptual form:
 *
 *     parallel reduce(values, combine)
 *
 * Associativity, identity, determinism and numerical semantics belong to
 * semantic analysis.
 *
 * The grammar MUST NOT assume a particular reduction tree or hardware.
 */

parallelReduce
    : PARALLEL
      REDUCE
      LPAREN
      expression
      COMMA
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 12. PARALLEL MAP/REDUCE ADAPTER
 * ============================================================================
 *
 * Stable expression-level integration point for data-parallel grammar.
 */

parallelDataOperation
    : parallelMap
    | parallelReduce
    ;


/*
 * ============================================================================
 * 13. PARALLEL DEPENDENCY
 * ============================================================================
 *
 * A parallel computation may explicitly identify a semantic dependency.
 *
 * The dependency operand remains an ordinary expression.
 *
 * This does not create a runtime dependency object.
 */

parallelDependency
    : dependencyKeyword
      expression
    ;


dependencyKeyword
    : DEPENDS
    | AFTER
    ;


/*
 * ============================================================================
 * 14. PARALLEL ORDERING
 * ============================================================================
 *
 * Explicit ordering is useful when parallel work is otherwise independent
 * but must preserve a particular semantic relationship.
 *
 * The grammar represents the relation.
 *
 * Semantic validation determines whether the relation is meaningful.
 */

parallelOrdering
    : parallelOrderingOperator
      expression
    ;


parallelOrderingOperator
    : BEFORE
    | AFTER
    ;


/*
 * ============================================================================
 * 15. PARALLEL BARRIER
 * ============================================================================
 *
 * A barrier establishes a semantic synchronization boundary.
 *
 * This rule does not implement synchronization.
 *
 * The synchronization grammar/runtime owns its implementation.
 */

parallelBarrier
    : BARRIER
    ;


/*
 * ============================================================================
 * 16. PARALLEL REGION ELEMENT
 * ============================================================================
 *
 * This adapter lets semantic tooling classify the contents of a parallel
 * region without redefining ordinary statements.
 */

parallelRegionElement
    : parallelWorkItem
    | parallelDependency
    | parallelOrdering
    | parallelBarrier
    ;


/*
 * ============================================================================
 * 17. EXPLICIT PARALLEL REGION WITH CONTROL ELEMENTS
 * ============================================================================
 *
 * This is the richer region form.
 *
 * Source order is preserved exactly.
 *
 * Dependency and ordering relations are interpreted downstream.
 */

parallelStructuredRegion
    : PARALLEL
      LBRACE
      parallelRegionElement*
      RBRACE
    ;


/*
 * ============================================================================
 * 18. PARALLEL COMPOSITION EXPRESSION
 * ============================================================================
 *
 * Parenthesized parallel composition remains expression-compatible.
 *
 * This rule does not create a second expression hierarchy.
 */

parallelCompositionExpression
    : PARALLEL
      LPAREN
      parallelExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 19. PARALLEL GROUP
 * ============================================================================
 *
 * A group is a syntactic grouping mechanism only.
 *
 * It must not be interpreted as:
 *
 *     thread group
 *     worker group
 *     hardware group
 *     machine group
 *
 * unless semantic analysis explicitly assigns such meaning.
 */

parallelGroup
    : PARALLEL
      LBRACE
      parallelWorkItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 20. PARALLEL INVOCATION
 * ============================================================================
 *
 * Provides an explicit integration point for an existing callable expression.
 *
 * Example conceptual form:
 *
 *     parallel(compute)
 *
 * The callable remains a normal expression.
 */

parallelInvocation
    : PARALLEL
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 21. PARALLEL ARGUMENT LIST
 * ============================================================================
 *
 * Named adapter for tooling.
 *
 * Canonical argument syntax remains owned by the expression/function grammar.
 */

parallelArgumentList
    : argumentList
    ;


/*
 * ============================================================================
 * 22. PARALLEL CAPABILITY REFERENCE
 * ============================================================================
 *
 * Parallel capability names are semantic identifiers.
 *
 * This rule MUST NOT encode physical hardware names as a closed set.
 *
 * Examples of possible semantic capabilities include:
 *
 *     vectorized
 *     asynchronous
 *     distributed
 *     accelerator
 *     quantum-compatible
 *
 * Such meanings are resolved by capability analysis.
 */

parallelCapabilityReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 23. PARALLEL POLICY REFERENCE
 * ============================================================================
 *
 * Policies are semantic references.
 *
 * They are not scheduler implementations.
 */

parallelPolicyReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 24. PARALLEL REQUIREMENT
 * ============================================================================
 *
 * Requirements express semantic needs without selecting hardware.
 */

parallelRequirement
    : qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 25. PARALLEL REQUIREMENT LIST
 * ============================================================================
 */

parallelRequirementList
    : parallelRequirement
      (
          COMMA
          parallelRequirement
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 26. PARALLEL ATTRIBUTE ADAPTER
 * ============================================================================
 *
 * Attributes remain owned by the canonical attribute grammar.
 */

parallelAttribute
    : attribute
    ;


/*
 * ============================================================================
 * 27. PARALLEL GENERIC ARGUMENT ADAPTER
 * ============================================================================
 *
 * Generic syntax remains owned by the canonical type/generic grammar.
 */

parallelGenericArguments
    : genericArguments
    ;


/*
 * ============================================================================
 * 28. PARALLEL EXPRESSION COMPOSITION
 * ============================================================================
 *
 * Stable expression integration point.
 *
 * A canonical expression grammar can integrate this production at the
 * concurrency/parallel precedence boundary.
 */

parallelComputationExpression
    : parallelExpression
    | parallelIterationExpression
    | parallelDataOperation
    | parallelInvocation
    | parallelCompositionExpression
    ;


/*
 * ============================================================================
 * 29. PARALLEL STATEMENT COMPOSITION
 * ============================================================================
 *
 * Stable statement integration point.
 */

parallelComputationStatement
    : parallelStatement
    | parallelIteration
    | parallelStructuredRegion
    | parallelBarrier
    ;


/*
 * ============================================================================
 * 30. DATA-PARALLEL INTEGRATION BOUNDARY
 * ============================================================================
 *
 * `data-parallel.g4` may reuse this boundary.
 *
 * This file deliberately does not own:
 *
 *     tensor dimensions
 *     array semantics
 *     vector semantics
 *     matrix semantics
 *     stream semantics
 *     SIMD lowering
 *     accelerator kernels
 *
 * Those concepts belong to their respective domains.
 */

parallelDataBoundary
    : parallelDataOperation
    | parallelIterationExpression
    ;


/*
 * ============================================================================
 * 31. TASK-PARALLEL INTEGRATION BOUNDARY
 * ============================================================================
 *
 * `task-parallel.g4` may reuse this boundary.
 *
 * This file does not redefine:
 *
 *     spawn
 *     await
 *     task groups
 *     task handles
 *     cancellation
 *
 * Those belong to task/future/cancellation grammars.
 */

parallelTaskBoundary
    : parallelExpression
    | parallelStructuredRegion
    ;


/*
 * ============================================================================
 * 32. CONCURRENCY INTEGRATION BOUNDARY
 * ============================================================================
 *
 * `concurrency.g4` can expose parallel syntax through this single adapter.
 */

parallelConcurrencyBoundary
    : parallelComputationExpression
    | parallelComputationStatement
    ;


/*
 * ============================================================================
 * 33. CANONICAL PUBLIC ROOT
 * ============================================================================
 *
 * Stable root for external parser composition.
 *
 * Consumers should prefer this production over depending on internal helper
 * rules.
 */

parallel
    : parallelConstruct
    ;