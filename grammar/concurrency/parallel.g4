/*
 * ============================================================================
 * Zamani Programming Language
 * Canonical Parallel-Computation Grammar
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/parallel.g4
 *
 * Status:
 *     Production-ready common parallel-computation parser component.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the COMMON SOURCE-LEVEL PARALLEL COMPUTATION BOUNDARY.
 *
 * It expresses that a computation or structured region is eligible for
 * parallel/concurrent realization while deliberately leaving the realization
 * strategy to semantic analysis, optimization, scheduling, and lowering.
 *
 * This file is intentionally smaller than the historical parallel grammar.
 *
 * Specialized parallel domains have their own owners:
 *
 *     data-parallel.g4
 *         data-parallel iteration, map, reduce, scan and partitioning
 *
 *     task-parallel.g4
 *         task-oriented parallelism and task dependencies
 *
 *     tasks.g4
 *         spawn/await/task syntax
 *
 *     channels.g4
 *         channel communication
 *
 *     synchronization.g4
 *         synchronization primitives
 *
 *     concurrency.g4
 *         concurrency-domain composition
 *
 * This file MUST NOT duplicate those grammars.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - common parallel computation syntax;
 *     - the `parallel` language construct;
 *     - parallel expression composition;
 *     - parallel structured-region boundaries;
 *     - statement-level integration for common parallel computation;
 *     - compatibility adapters for common parallel syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - ordinary expressions;
 *     - ordinary statements;
 *     - loops;
 *     - bindings;
 *     - types;
 *     - functions;
 *     - tasks;
 *     - futures;
 *     - actors;
 *     - channels;
 *     - barriers;
 *     - locks;
 *     - atomics;
 *     - data-parallel map;
 *     - data-parallel reduce;
 *     - data-parallel scan;
 *     - task dependencies;
 *     - worker creation;
 *     - worker counts;
 *     - thread counts;
 *     - CPU/core counts;
 *     - GPU counts;
 *     - FPGA counts;
 *     - accelerator counts;
 *     - QPU counts;
 *     - node counts;
 *     - machine topology;
 *     - device identifiers;
 *     - physical placement;
 *     - routing;
 *     - scheduling;
 *     - resource allocation;
 *     - hardware discovery;
 *     - hardware calibration;
 *     - quantum IR;
 *     - classical IR;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - HAL;
 *     - runtime implementation.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Parallel syntax expresses PORTABLE COMPUTATIONAL INTENT.
 *
 * It MUST NOT encode a fixed realization.
 *
 * In particular, this file contains no language-level limits for:
 *
 *     parallel regions
 *     parallel branches
 *     workers
 *     threads
 *     cores
 *     GPUs
 *     FPGAs
 *     accelerators
 *     QPUs
 *     nodes
 *     processes
 *     tasks
 *     channels
 *     vector lanes
 *     tensor dimensions
 *     memory
 *     execution contexts
 *
 * The following concepts are explicitly forbidden as universal grammar
 * restrictions:
 *
 *     MAX_PARALLELISM
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ACCELERATORS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_TASKS
 *     MAX_BRANCHES
 *
 * A single logical parallel computation may therefore be realized using:
 *
 *     - one execution context;
 *     - multiple CPU cores;
 *     - SIMD/vector execution;
 *     - GPU execution;
 *     - FPGA execution;
 *     - ASIC execution;
 *     - accelerator execution;
 *     - quantum/classical orchestration;
 *     - distributed execution;
 *     - heterogeneous execution;
 *     - a future computational substrate.
 *
 * If insufficient resources are available, downstream scheduling may serialize
 * logically parallel work while preserving the specified semantics.
 *
 * If additional resources are available, downstream scheduling may exploit
 * additional parallelism.
 *
 * Therefore:
 *
 *     logical parallelism != physical parallelism
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The source construct:
 *
 *     parallel {
 *         a();
 *         b();
 *         c();
 *     }
 *
 * means that the enclosed computation is exposed as a parallelizable region.
 *
 * It does NOT mean:
 *
 *     - three physical workers;
 *     - three threads;
 *     - three cores;
 *     - three devices;
 *     - simultaneous execution in wall-clock time.
 *
 * Semantic analysis determines:
 *
 *     - dependencies;
 *     - effects;
 *     - ownership;
 *     - aliasing;
 *     - memory conflicts;
 *     - synchronization requirements;
 *     - determinism;
 *     - resource requirements;
 *     - capability requirements;
 *     - legality of parallel realization.
 *
 * Scheduling determines the eventual execution order and resource assignment.
 *
 * ============================================================================
 * CANONICAL PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     parallel.g4
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +-----------------------------+
 *       |                             |
 *       v                             v
 *     effect analysis           resource/capability analysis
 *       |                             |
 *       +-------------+---------------+
 *                     |
 *                     v
 *             canonical semantic model
 *                     |
 *                     v
 *             canonical IR boundary
 *                     |
 *       +-------------+---------------+
 *       |             |               |
 *       v             v               v
 *   classical      quantum::ir      HDL/hardware
 *       |             |               |
 *       +-------------+---------------+
 *                     |
 *                     v
 *                 optimization
 *                     |
 *                     v
 *              routing/scheduling
 *                     |
 *                     v
 *             resilience/QEC/ZQN
 *                     |
 *                     v
 *                    HAL
 *                     |
 *                     v
 *             target realization
 *
 * This file participates only in the source/parser portion of this pipeline.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer owns all lexical spelling.
 *
 * Existing canonical concurrency vocabulary includes:
 *
 *     ASYNC
 *     AWAIT
 *     SPAWN
 *     PARALLEL
 *
 * This file uses:
 *
 *     PARALLEL
 *
 * and existing punctuation/tokens.
 *
 * No lexer rules are declared here.
 *
 * No new token is required for this file.
 *
 * IMPORTANT:
 *
 * `MAP`, `REDUCE`, `SCAN`, `PARTITION`, `WORKER`, `THREAD`, `CORE`,
 * `GPU`, `QPU`, etc. are deliberately NOT introduced here.
 *
 * Data-parallel operation names remain semantic names and are owned by
 * `data-parallel.g4`.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Canonical reusable grammar components:
 *
 *     Expressions
 *         -> expression
 *
 *     ZamaniExpressionBlocks
 *         -> expression-side block integration
 *
 * The final canonical block implementation remains owned by the repository's
 * canonical block composition.
 *
 * This file does not define:
 *
 *     expression
 *     blockExpression
 *     statement
 *     typeExpression
 *
 * ============================================================================
 */

parser grammar Parallel;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions,
    ZamaniExpressionBlocks
;


/*
 * ============================================================================
 * 1. PUBLIC PARALLEL DOMAIN ENTRY POINT
 * ============================================================================
 *
 * This is the stable public entry point for the common parallel grammar.
 *
 * Specialized grammars must NOT redefine this construct merely to obtain
 * access to the common `parallel` syntax.
 */

parallelConstruct
    : parallelExpression
    ;


/*
 * ============================================================================
 * 2. CANONICAL PARALLEL EXPRESSION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     parallel expression
 *
 *     parallel {
 *         ...
 *     }
 *
 * The enclosed expression remains an ordinary Zamani expression.
 *
 * The enclosed block remains the canonical Zamani block.
 *
 * No execution resource is specified.
 */

parallelExpression
    : PARALLEL parallelBody
    ;


/*
 * ============================================================================
 * 3. PARALLEL BODY
 * ============================================================================
 *
 * This adapter keeps the distinction between:
 *
 *     parallel expression
 *
 * and:
 *
 *     parallel block
 *
 * without defining another expression hierarchy or another block grammar.
 */

parallelBody
    : blockExpression
    | expression
    ;


/*
 * ============================================================================
 * 4. PARALLEL REGION
 * ============================================================================
 *
 * Stable semantic naming for a structured parallel region.
 *
 * This is an adapter to the canonical parallel expression rather than a
 * second implementation of the same source construct.
 */

parallelRegion
    : parallelBlock
    ;


parallelBlock
    : PARALLEL blockExpression
    ;


/*
 * ============================================================================
 * 5. PARALLEL STATEMENT
 * ============================================================================
 *
 * Statement-level integration.
 *
 * A block-form parallel construct does not require a trailing semicolon.
 *
 * An expression-form parallel statement uses the canonical statement
 * terminator.
 *
 * This avoids the historical/nonexistent `SEMI` token.
 *
 * Canonical punctuation is:
 *
 *     SEMICOLON
 */

parallelStatement
    : PARALLEL blockExpression
    | PARALLEL expression SEMICOLON
    ;


/*
 * ============================================================================
 * 6. PARALLEL WORK ITEM
 * ============================================================================
 *
 * Compatibility adapter for tooling that wants to classify an element of
 * a parallel region.
 *
 * The underlying syntax remains owned by the ordinary expression/block
 * grammars.
 *
 * This rule does not introduce a worker abstraction.
 */

parallelWorkItem
    : parallelWorkExpression
    | parallelWorkStatement
    ;


parallelWorkExpression
    : expression
    ;


parallelWorkStatement
    : parallelStatement
    ;


/*
 * ============================================================================
 * 7. PARALLEL WORK LIST
 * ============================================================================
 *
 * A logical collection of source-level work elements.
 *
 * The grammar imposes no finite number of elements.
 *
 * Resource decomposition is downstream.
 */

parallelWorkList
    : parallelWorkItem+
    ;


parallelWorkListOptional
    : parallelWorkItem*
    ;


/*
 * ============================================================================
 * 8. PARALLEL COMPOSITION
 * ============================================================================
 *
 * Compatibility/composition adapter.
 *
 * The actual structured region is represented by `parallelBlock`.
 *
 * This rule does not introduce another parallel-region implementation.
 */

parallelComposition
    : parallelBlock
    ;


/*
 * ============================================================================
 * 9. PARALLEL INVOCATION
 * ============================================================================
 *
 * Canonical callable form:
 *
 *     parallel(callable)
 *
 * The callable is an ordinary Zamani expression.
 *
 * This syntax does not select:
 *
 *     a worker;
 *     a thread;
 *     a core;
 *     a device;
 *     a backend.
 */

parallelInvocation
    : PARALLEL
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 10. PARALLEL ARGUMENT LIST ADAPTER
 * ============================================================================
 *
 * The common argument syntax remains owned by the expression/call grammar.
 *
 * This adapter is retained for tooling compatibility.
 */

parallelArgumentList
    : argumentList
    ;


/*
 * ============================================================================
 * 11. PARALLEL CAPABILITY REFERENCE
 * ============================================================================
 *
 * Capability identity is semantic data.
 *
 * Examples:
 *
 *     vectorized
 *     asynchronous
 *     distributed
 *     accelerator
 *     quantum-compatible
 *
 * No finite capability inventory is encoded here.
 *
 * Hardware realization belongs downstream.
 */

parallelCapabilityReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 12. PARALLEL POLICY REFERENCE
 * ============================================================================
 *
 * Policies remain semantic names.
 *
 * The grammar does not implement scheduling policies.
 */

parallelPolicyReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 13. PARALLEL REQUIREMENT
 * ============================================================================
 *
 * A requirement identifies semantic intent.
 *
 * It does not select a physical resource.
 *
 * For example, a downstream semantic layer may interpret:
 *
 *     parallel.compute
 *
 * as a capability requirement.
 *
 * The grammar does not determine whether that capability is implemented by
 * CPUs, GPUs, FPGAs, QPUs, distributed nodes, or another target.
 */

parallelRequirement
    : qualifiedName
    | expression
    ;


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
 * 14. PARALLEL ATTRIBUTE ADAPTER
 * ============================================================================
 *
 * Attributes remain owned by the canonical attribute grammar.
 *
 * No duplicate attribute syntax is introduced.
 */

parallelAttribute
    : attribute
    ;


/*
 * ============================================================================
 * 15. PARALLEL GENERIC ARGUMENT ADAPTER
 * ============================================================================
 *
 * Generic syntax remains owned by the canonical generic/type grammar.
 *
 * This rule exists only as an integration adapter where existing consumers
 * require a parallel-domain name.
 */

parallelGenericArguments
    : genericArguments
    ;


/*
 * ============================================================================
 * 16. PARALLEL COMPUTATION EXPRESSION
 * ============================================================================
 *
 * Stable expression-facing composition boundary.
 *
 * Common parallel syntax belongs here.
 *
 * Data-parallel and task-parallel specializations are intentionally NOT
 * duplicated here.
 */

parallelComputationExpression
    : parallelExpression
    | parallelInvocation
    ;


/*
 * ============================================================================
 * 17. PARALLEL COMPUTATION STATEMENT
 * ============================================================================
 *
 * Stable statement-facing composition boundary.
 */

parallelComputationStatement
    : parallelStatement
    | parallelBlock
    ;


/*
 * ============================================================================
 * 18. DATA-PARALLEL INTEGRATION BOUNDARY
 * ============================================================================
 *
 * This adapter allows the concurrency composition layer to identify the
 * common relationship between parallel computation and data-parallel
 * specialization.
 *
 * IMPORTANT:
 *
 * This file does not define map/reduce/scan/partition syntax.
 *
 * Those are owned by:
 *
 *     grammar/concurrency/data-parallel.g4
 *
 * The composed parser integrates that grammar separately.
 */

parallelDataBoundary
    : parallelComputationExpression
    ;


/*
 * ============================================================================
 * 19. TASK-PARALLEL INTEGRATION BOUNDARY
 * ============================================================================
 *
 * Task-specific semantics remain owned by:
 *
 *     grammar/concurrency/task-parallel.g4
 *
 * This file only exposes the common parallel boundary.
 */

parallelTaskBoundary
    : parallelComputationExpression
    | parallelComputationStatement
    ;


/*
 * ============================================================================
 * 20. CONCURRENCY INTEGRATION BOUNDARY
 * ============================================================================
 *
 * `concurrency.g4` uses this adapter to expose common parallel syntax.
 */

parallelConcurrencyBoundary
    : parallelComputationExpression
    | parallelComputationStatement
    ;


/*
 * ============================================================================
 * 21. CANONICAL PUBLIC ALIAS
 * ============================================================================
 *
 * Stable public alias retained for callers that use the shorter `parallel`
 * entry point.
 */

parallel
    : parallelConstruct
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve:
 *
 *     - construct kind;
 *     - source span;
 *     - source ordering;
 *     - nesting;
 *     - enclosed expression;
 *     - enclosed block;
 *     - source-level attributes where supplied by the surrounding grammar.
 *
 * Suggested semantic classification:
 *
 *     ParallelExpression
 *     ParallelRegion
 *     ParallelInvocation
 *
 * The AST MUST NOT manufacture:
 *
 *     worker IDs;
 *     thread IDs;
 *     core IDs;
 *     GPU IDs;
 *     FPGA IDs;
 *     QPU IDs;
 *     node IDs;
 *     scheduler IDs;
 *     physical addresses;
 *     physical topology.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis determines:
 *
 *     - whether the enclosed computation is parallelizable;
 *     - whether dependencies permit concurrent realization;
 *     - whether effects permit concurrent realization;
 *     - whether ownership/borrowing rules permit concurrent access;
 *     - whether memory conflicts exist;
 *     - whether synchronization is required;
 *     - whether ordering is observable;
 *     - whether deterministic semantics are required;
 *     - what capabilities are required;
 *     - what resources are required.
 *
 * Syntax acceptance does not imply semantic parallelizability.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Parallel syntax may result in derived resource requirements.
 *
 * These requirements belong to semantic/resource analysis.
 *
 * Examples:
 *
 *     execution capacity
 *     memory bandwidth
 *     communication
 *     accelerator capability
 *     vector capability
 *     quantum/classical orchestration
 *     distributed communication
 *
 * The grammar does not resolve those requirements against a machine.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic for:
 *
 *     identical source;
 *     identical tokenization;
 *     identical language version;
 *     identical grammar configuration.
 *
 * Parsing must not depend upon:
 *
 *     CPU availability;
 *     GPU availability;
 *     QPU availability;
 *     machine topology;
 *     runtime state;
 *     scheduler state;
 *     calibration;
 *     network state;
 *     filesystem state;
 *     randomness;
 *     wall-clock time.
 *
 * Parallel execution itself may be nondeterministic only where permitted by
 * the semantic/effect model.
 *
 * The grammar does not silently introduce nondeterminism.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A parallel region may contain quantum source constructs once those constructs
 * are admitted through the canonical expression/block/statement composition.
 *
 * This file does NOT:
 *
 *     - define quantum operations;
 *     - define quantum gates;
 *     - define qubit counts;
 *     - define physical qubits;
 *     - define QPU topology;
 *     - define routing;
 *     - define scheduling;
 *     - define QEC;
 *     - define ZQN;
 *     - define calibration.
 *
 * Quantum lowering remains:
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
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     QEC/resilience
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 * No parallel-specific quantum IR is introduced.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Parallel syntax may contain:
 *
 *     scalar computation;
 *     vector computation;
 *     matrix computation;
 *     tensor computation;
 *     symbolic computation;
 *     numerical computation;
 *     scientific computation;
 *     AI/ML computation.
 *
 * Their semantics remain owned by their respective domains.
 *
 * The common parallel grammar does not create:
 *
 *     CPUParallelIR
 *     GPUParallelIR
 *     SIMDParallelIR
 *
 * or equivalent duplicate intermediate representations.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Parallel source intent may eventually lower into:
 *
 *     pipelines;
 *     replicated logic;
 *     vector hardware;
 *     accelerator execution;
 *     distributed hardware;
 *     other target implementations.
 *
 * This grammar does not select any such realization.
 *
 * It MUST NOT encode:
 *
 *     register width;
 *     bus width;
 *     number of processing elements;
 *     FPGA family;
 *     ASIC family;
 *     clock frequency;
 *     physical placement.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A parallel region may eventually be distributed across:
 *
 *     processes;
 *     nodes;
 *     services;
 *     accelerators;
 *     heterogeneous resources.
 *
 * This grammar does not equate:
 *
 *     parallel branch == node
 *
 * or:
 *
 *     worker == process.
 *
 * Distribution, partitioning, communication, replication and placement belong
 * to their canonical downstream subsystems.
 *
 * ============================================================================
 * DATA-PARALLEL OWNERSHIP
 * ============================================================================
 *
 * Do NOT add:
 *
 *     parallelMap
 *     parallelReduce
 *     parallelScan
 *     parallelPartition
 *
 * implementations here.
 *
 * Those are owned by:
 *
 *     grammar/concurrency/data-parallel.g4
 *
 * Its operation names remain semantic qualified names rather than permanently
 * reserved lexer keywords.
 *
 * This avoids:
 *
 *     MAP
 *     REDUCE
 *     SCAN
 *     PARTITION
 *
 * becoming unnecessary global keywords.
 *
 * ============================================================================
 * TASK-PARALLEL OWNERSHIP
 * ============================================================================
 *
 * Do NOT define:
 *
 *     spawn
 *     await
 *     task dependency
 *     task join
 *     task group
 *
 * here.
 *
 * Those belong to:
 *
 *     tasks.g4
 *     futures.g4
 *     task-parallel.g4
 *
 * The common parallel grammar remains independent of task implementation.
 *
 * ============================================================================
 * SYNCHRONIZATION OWNERSHIP
 * ============================================================================
 *
 * Do NOT define:
 *
 *     barrier
 *     mutex
 *     lock
 *     semaphore
 *     condition variable
 *     atomic protocol
 *
 * here.
 *
 * Synchronization grammar owns synchronization syntax.
 *
 * A parallel computation can contain synchronization constructs only through
 * the canonical concurrency composition.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_PARALLELISM
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ACCELERATORS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_TASKS
 *     MAX_BRANCHES
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_DIMENSIONS
 *     MAX_MEMORY
 *
 * It contains no:
 *
 *     physical device IDs;
 *     physical qubit IDs;
 *     fixed topology;
 *     fixed placement;
 *     fixed worker-to-core mapping;
 *     fixed execution width.
 *
 * Program values remain program values.
 *
 * For example:
 *
 *     parallel compute(1024)
 *
 * does not establish a universal parallelism limit of 1024.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - performs no network access;
 *     - executes no source code;
 *     - accesses no hardware;
 *     - performs no scheduling;
 *     - performs no resource discovery;
 *     - contains no embedded Rust;
 *     - requires no unsafe Rust.
 *
 * Malformed source is handled by the canonical parser/diagnostic layer.
 *
 * Parser denial-of-service protection, if required, belongs to configurable
 * implementation policy rather than language semantics.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors include forms such as:
 *
 *     parallel
 *     parallel (
 *     parallel {
 *     parallel expression extra
 *     parallel expression ;
 *
 * where the final form is invalid or valid according to the surrounding
 * statement context rather than this grammar inventing an alternative
 * terminator.
 *
 * Semantic errors include:
 *
 *     parallel computation with conflicting mutable accesses;
 *     parallel computation with invalid ownership;
 *     parallel computation with incompatible effects;
 *     unavailable capability;
 *     unsatisfied resource requirement;
 *     nondeterministic operation where deterministic semantics are required.
 *
 * Semantic diagnostics remain downstream.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The existing PARALLEL token remains unchanged:
 *
 *     PARALLEL : 'parallel' ;
 *
 * No token rename is required.
 *
 * The historical `SEMI` spelling is NOT retained because the canonical lexer
 * uses:
 *
 *     SEMICOLON
 *
 * This file therefore corrects the existing token mismatch rather than
 * creating a compatibility alias that would produce multiple punctuation
 * authorities.
 *
 * `parallel.g4` no longer owns MAP/REDUCE syntax because those constructs are
 * correctly delegated to data-parallel.g4.
 *
 * ============================================================================
 * INTEGRATION WITH CONCURRENCY.G4
 * ============================================================================
 *
 * `grammar/concurrency/concurrency.g4` is the composition owner.
 *
 * It should import:
 *
 *     Parallel
 *
 * together with the other concurrency grammars:
 *
 *     Tasks
 *     Futures
 *     DataParallel
 *     TaskParallel
 *     Channels
 *     Synchronization
 *
 * where those grammar components are part of the canonical composition.
 *
 * The composition layer should expose:
 *
 *     concurrencyParallelConstruct
 *         : parallelConstruct
 *         ;
 *
 * and should NOT duplicate:
 *
 *     parallelExpression
 *     parallelStatement
 *     parallelRegion
 *     parallelBlock
 *
 * ============================================================================
 * INTEGRATION WITH DATA-PARALLEL.G4
 * ============================================================================
 *
 * `data-parallel.g4` owns:
 *
 *     dataParallelConstruct
 *     dataParallelIteration
 *     dataParallelMap
 *     dataParallelReduce
 *     dataParallelScan
 *     dataParallelPartition
 *
 * It MUST NOT redefine:
 *
 *     parallelExpression
 *     parallelStatement
 *
 * The composition layer admits data-parallel syntax separately.
 *
 * ============================================================================
 * INTEGRATION WITH TASK-PARALLEL.G4
 * ============================================================================
 *
 * `task-parallel.g4` owns task-parallel-specific structures.
 *
 * It may consume:
 *
 *     parallelExpression
 *
 * or the stable:
 *
 *     parallelTaskBoundary
 *
 * adapter.
 *
 * It must not redefine the common `parallel` syntax.
 *
 * ============================================================================
 * INTEGRATION WITH EXPRESSIONS
 * ============================================================================
 *
 * The canonical expression hierarchy remains owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file consumes:
 *
 *     expression
 *
 * but does not redefine it.
 *
 * If `parallel` becomes legal at a specific expression-precedence location,
 * the expression composition grammar should integrate `parallelExpression`
 * at that location rather than modifying this file to recreate expression
 * precedence.
 *
 * ============================================================================
 * INTEGRATION WITH BLOCKS
 * ============================================================================
 *
 * The canonical block implementation remains outside this grammar.
 *
 * This file consumes:
 *
 *     blockExpression
 *
 * through the existing block composition architecture.
 *
 * It does not define:
 *
 *     LBRACE ... RBRACE
 *
 * independently.
 *
 * ============================================================================
 * INTEGRATION WITH SEMANTICS
 * ============================================================================
 *
 * Semantic analysis consumes the parsed parallel structure and derives:
 *
 *     dependency graph;
 *     effect information;
 *     ownership constraints;
 *     synchronization requirements;
 *     resource requirements;
 *     capability requirements;
 *     determinism requirements;
 *     parallelization legality.
 *
 * ============================================================================
 * INTEGRATION WITH IR
 * ============================================================================
 *
 * This grammar introduces NO parallel-specific IR.
 *
 * Parallel semantics must lower into the repository's canonical semantic/IR
 * infrastructure.
 *
 * Quantum operations continue through:
 *
 *     quantum::ir
 *
 * Classical computation continues through its canonical representation.
 *
 * Hardware/software co-design continues through the canonical hardware/HDL
 * semantic path.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     parallel compute()
 *
 *     parallel {
 *         first();
 *         second();
 *     }
 *
 *     parallel {
 *         let x = compute();
 *         let y = transform();
 *     }
 *
 *     parallel(callable)
 *
 *     parallel {
 *         quantum_work();
 *     }
 *
 *     parallel {
 *         accelerator_work();
 *     }
 *
 *     parallel {
 *         distributed_work();
 *     }
 *
 * NEGATIVE:
 *
 *     parallel
 *
 *     parallel {
 *
 *     parallel (
 *
 *     parallel )
 *
 *     parallel { extra
 *
 *     parallel expression extra unexpected
 *
 * BOUNDARY:
 *
 *     parallel { }
 *
 *     parallel { one(); }
 *
 *     parallel { one(); two(); }
 *
 *     deeply nested parallel regions;
 *
 *     many parallel regions;
 *
 *     large expressions inside a parallel region;
 *
 *     large source programs containing parallel constructs.
 *
 * SCALABILITY:
 *
 *     - no test establishes a maximum branch count;
 *     - no test establishes a maximum nesting depth as a language limit;
 *     - no test establishes a worker count;
 *     - no test establishes a machine size;
 *     - no test establishes a device count.
 *
 * CROSS-DOMAIN:
 *
 *     classical + parallel
 *     quantum + parallel
 *     hybrid + parallel
 *     HDL/hardware + parallel
 *     AI/data + parallel
 *     distributed + parallel
 *     networking + parallel
 *     accelerator + parallel
 *
 * DETERMINISM:
 *
 *     identical token streams must produce equivalent parse structures.
 *
 * COMPATIBILITY:
 *
 *     existing `parallel` source must continue to recognize the stable
 *     PARALLEL keyword.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * [x] Uses parser grammar syntax.
 *
 * [x] Uses the canonical ZamaniLexer vocabulary.
 *
 * [x] Preserves the existing PARALLEL token.
 *
 * [x] Does not introduce new lexer tokens.
 *
 * [x] Does not use the nonexistent SEMI token.
 *
 * [x] Uses canonical SEMICOLON.
 *
 * [x] Does not invent MAP/REDUCE lexer tokens.
 *
 * [x] Does not duplicate data-parallel grammar.
 *
 * [x] Does not duplicate task-parallel grammar.
 *
 * [x] Does not duplicate synchronization grammar.
 *
 * [x] Does not duplicate expression precedence.
 *
 * [x] Does not duplicate block syntax.
 *
 * [x] Does not define an independent statement grammar.
 *
 * [x] Has a stable public parallelConstruct entry point.
 *
 * [x] Has a stable parallel expression boundary.
 *
 * [x] Has a stable parallel statement boundary.
 *
 * [x] Has explicit AST ownership.
 *
 * [x] Has explicit semantic ownership.
 *
 * [x] Has explicit IR ownership.
 *
 * [x] Has explicit compiler/runtime boundaries.
 *
 * [x] Has quantum::ir integration without creating a second quantum IR.
 *
 * [x] Has resource/capability separation.
 *
 * [x] Has deterministic parsing requirements.
 *
 * [x] Has positive/negative/boundary/scalability tests specified.
 *
 * [x] Contains no hardware limits.
 *
 * [x] Contains no physical topology.
 *
 * [x] Contains no worker/thread/core/device identifiers.
 *
 * [x] Contains no Rust actions.
 *
 * [x] Requires safe Rust only.
 *
 * A future change to scheduling, routing, hardware, QEC, ZQN, HAL, runtime,
 * compiler optimization, CPU/GPU/FPGA/QPU realization, or resource discovery
 * MUST NOT require editing this file unless the source-level parallel syntax
 * itself changes.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */