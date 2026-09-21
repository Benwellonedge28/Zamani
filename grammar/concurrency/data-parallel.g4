/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/concurrency/data-parallel.g4
 *
 * STATUS
 * ------
 * CANONICAL DATA-PARALLEL SYNTAX COMPONENT
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust edition 2021
 * Safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the authoritative syntax owner for DATA-PARALLEL computation.
 *
 * Data parallelism expresses that a computation is applicable independently
 * across elements, partitions, logical lanes, records, tensor elements,
 * samples, measurements, generated domains, streams, or other semantic data
 * domains.
 *
 * This file describes SOURCE-LEVEL COMPUTATIONAL INTENT.
 *
 * It does NOT describe physical execution.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * This file owns:
 *
 *     dataParallelConstruct
 *     dataParallelIteration
 *     dataParallelMap
 *     dataParallelReduce
 *     dataParallelScan
 *     dataParallelPartition
 *     dataParallelOperation
 *     dataParallelPredicate
 *     dataParallelVectorizedIteration
 *
 * This file does NOT own:
 *
 *     ordinary expressions
 *     ordinary statements
 *     ordinary loops
 *     bindings
 *     blocks
 *     general concurrency
 *     general parallel regions
 *     tasks
 *     futures
 *     actors
 *     channels
 *     synchronization
 *     cancellation
 *     scheduling
 *     placement
 *     routing
 *     resource allocation
 *     hardware discovery
 *     target selection
 *     classical IR
 *     quantum IR
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * Canonical owners:
 *
 *     expressions/
 *         expression syntax and precedence
 *
 *     statements/
 *         statement syntax
 *
 *     statements/bindings.g4
 *         bindingPattern
 *
 *     core/blocks.g4
 *         blockExpression
 *
 *     concurrency/parallel.g4
 *         common parallel computation
 *
 *     concurrency/tasks.g4
 *         task-parallel primitives
 *
 *     concurrency/concurrency.g4
 *         concurrency-domain composition
 *
 *     resources/
 *         resource/capability/constraint semantics
 *
 *     hardware/
 *         hardware capability and target intent
 *
 *     execution/
 *         execution policy and realization
 *
 *     src/frontend/ast/
 *         domain-neutral AST
 *
 *     src/quantum/ir/
 *         canonical quantum semantic boundary
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The fundamental rule is:
 *
 *     LOGICAL DATA PARALLELISM != PHYSICAL EXECUTION WIDTH
 *
 * This grammar therefore imposes NO universal limits on:
 *
 *     workers
 *     threads
 *     CPU cores
 *     GPUs
 *     FPGAs
 *     ASIC resources
 *     accelerators
 *     QPUs
 *     nodes
 *     processes
 *     vector lanes
 *     tensor dimensions
 *     collection elements
 *     partitions
 *     iterations
 *     memory
 *     network resources
 *
 * The following must never appear as grammar-level limits:
 *
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ACCELERATORS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_ITEMS
 *     MAX_ELEMENTS
 *     MAX_PARTITIONS
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_DIMENSION
 *
 * A program describing one million logical elements is not a different
 * language program from one describing ten elements.
 *
 * The compiler/runtime may choose:
 *
 *     sequential execution
 *     CPU parallel execution
 *     SIMD/vector execution
 *     GPU execution
 *     FPGA execution
 *     accelerator execution
 *     distributed execution
 *     heterogeneous execution
 *     quantum/classical orchestration
 *     another future realization
 *
 * according to semantic legality, available resources and target capabilities.
 *
 * ============================================================================
 * RESOURCE SEPARATION
 * ============================================================================
 *
 * This grammar MUST NOT turn resource availability into source syntax.
 *
 * These are semantic concerns:
 *
 *     worker availability
 *     memory availability
 *     accelerator availability
 *     device topology
 *     placement
 *     scheduling
 *     routing
 *     bandwidth
 *     latency
 *     power
 *     thermal constraints
 *     reliability
 *
 * Data-parallel syntax says WHAT may be parallelized.
 *
 * Resource analysis determines WHAT CAN be realized.
 *
 * Scheduling determines WHEN and WHERE it is realized.
 *
 * Lowering determines HOW it is realized.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * The canonical lexical authority remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     grammar/lexer/tokens.g4
 *
 * Existing lexical tokens consumed here include:
 *
 *     PARALLEL
 *     FOR
 *     IN
 *     WHEN
 *     VECTORIZED
 *     DOUBLE_COLON
 *     LPAREN
 *     RPAREN
 *     COMMA
 *
 * and the canonical identifier token through the repository's name grammar.
 *
 * No MAP, REDUCE, SCAN, PARTITION, WORKER, THREAD, GPU, CORE or SIMD lexer
 * token is required by this file.
 *
 * Data-parallel operation names remain identifiers.
 *
 * ============================================================================
 * WHY `parallel::map` USES A SPECIAL RULE
 * ============================================================================
 *
 * `parallel` is already a reserved PARALLEL token.
 *
 * Therefore this is NOT equivalent to:
 *
 *     qualifiedName
 *
 * because:
 *
 *     qualifiedName
 *         ::= identifier ("::" identifier)*
 *
 * cannot consume PARALLEL as IDENTIFIER.
 *
 * The canonical data-parallel namespace therefore uses:
 *
 *     PARALLEL DOUBLE_COLON identifier
 *
 * which permits:
 *
 *     parallel::map
 *     parallel::reduce
 *     parallel::scan
 *     parallel::partition
 *     parallel::transform
 *     parallel::filter
 *     parallel::zip
 *     parallel::zip_with
 *     parallel::stencil
 *
 * without adding permanent lexer keywords for every operation.
 *
 * ============================================================================
 * OPERATION EXTENSIBILITY
 * ============================================================================
 *
 * `parallel::map`, `parallel::reduce`, `parallel::scan` and
 * `parallel::partition` are syntactic operation categories.
 *
 * The grammar intentionally does NOT enumerate every possible algorithm.
 *
 * Future library/dialect operations can use:
 *
 *     parallel::transform(...)
 *     parallel::filter(...)
 *     parallel::zip(...)
 *     parallel::zip_with(...)
 *     parallel::window(...)
 *     parallel::stencil(...)
 *     parallel::group(...)
 *     parallel::sort(...)
 *
 * without modifying the lexer.
 *
 * Whether a particular operation exists is a semantic/library/dialect
 * question, not a parser-level hardware question.
 *
 * ============================================================================
 * BINDING CONTRACT
 * ============================================================================
 *
 * Iteration uses the canonical:
 *
 *     bindingPattern
 *
 * from:
 *
 *     grammar/statements/bindings.g4
 *
 * This file MUST NOT define another pattern/binding grammar.
 *
 * ============================================================================
 * BLOCK CONTRACT
 * ============================================================================
 *
 * Bodies use:
 *
 *     blockExpression
 *
 * from:
 *
 *     grammar/core/blocks.g4
 *
 * This file MUST NOT define a second block grammar.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * All data domains, predicates, mapping functions, reducers, scan functions,
 * partitioners and optional semantic arguments use the canonical:
 *
 *     expression
 *
 * rule.
 *
 * This allows data-parallel computation over:
 *
 *     arrays
 *     slices
 *     collections
 *     ranges
 *     streams
 *     tensors
 *     matrices
 *     datasets
 *     generated sequences
 *     distributed data
 *     accelerator buffers
 *     measurement results
 *     symbolic domains
 *     future domain abstractions
 *
 * without embedding their implementation into this grammar.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The grammar establishes structure only.
 *
 * Semantic analysis MUST determine:
 *
 *     whether iterations are independent;
 *     whether writes conflict;
 *     whether reads/writes alias;
 *     whether effects permit parallel evaluation;
 *     whether ordering is observable;
 *     whether a mapper is valid;
 *     whether a reducer is valid;
 *     whether a scan operator is valid;
 *     whether a partitioner is valid;
 *     whether deterministic ordering is required;
 *     whether numerical semantics permit reassociation;
 *     whether resource/capability requirements can be satisfied.
 *
 * ============================================================================
 * REDUCTION SEMANTICS
 * ============================================================================
 *
 * The grammar deliberately does NOT assume that a reducer is associative.
 *
 * In particular:
 *
 *     floating-point addition
 *     floating-point subtraction
 *     arbitrary user functions
 *     stateful operations
 *
 * must not silently be reassociated.
 *
 * Semantic analysis owns:
 *
 *     identity
 *     associativity
 *     commutativity
 *     ordering
 *     determinism
 *     numerical stability
 *     overflow/underflow semantics
 *     effect legality
 *
 * The compiler may only transform reduction structure when semantic rules
 * prove the transformation legal.
 *
 * ============================================================================
 * SCAN SEMANTICS
 * ============================================================================
 *
 * A scan differs from a reduction because it produces values corresponding to
 * prefixes or another explicitly specified scan semantics.
 *
 * This grammar does not prescribe:
 *
 *     sequential scan
 *     tree scan
 *     segmented scan
 *     vector scan
 *     GPU scan
 *     FPGA pipeline
 *     distributed scan
 *
 * Those are lowering choices.
 *
 * ============================================================================
 * PARTITION SEMANTICS
 * ============================================================================
 *
 * `parallel::partition` describes semantic partitioning of a data domain.
 *
 * It does NOT mean:
 *
 *     assign partition 0 to CPU 0
 *     assign partition 1 to GPU 0
 *     assign partition 2 to node 0
 *
 * Physical placement is downstream.
 *
 * ============================================================================
 * VECTORIZATION
 * ============================================================================
 *
 * `vectorized` is a semantic/request modifier.
 *
 * It does NOT specify:
 *
 *     128-bit
 *     256-bit
 *     512-bit
 *     fixed SIMD lanes
 *     a particular instruction set
 *     a particular accelerator
 *
 * The compiler may realize vectorization through whatever target capability
 * satisfies the semantic contract.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Data-parallel constructs may surround or invoke quantum computation.
 *
 * Example:
 *
 *     parallel for item in data {
 *         quantum_operation(item);
 *     }
 *
 * Quantum semantics remain owned by the quantum subsystem.
 *
 * The lowering path is:
 *
 *     data-parallel AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target
 *
 * This file MUST NOT define quantum gates, qubits, QEC or another quantum IR.
 *
 * ============================================================================
 * CLASSICAL / AI / DATA / HDL / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * CLASSICAL
 *
 * Data-parallel computations may lower into classical computation IR.
 *
 * AI / ML
 *
 * Tensor/model/data-parallel operations remain owned by the AI/data/type
 * systems. This grammar supplies only parallel computation structure.
 *
 * HDL / HARDWARE
 *
 * Data parallelism may lower into replication, pipelining, vectorization or
 * another hardware realization. No physical width is encoded here.
 *
 * DISTRIBUTED
 *
 * A semantic data domain may be partitioned across logical execution
 * locations. Placement and communication remain distributed/runtime concerns.
 *
 * HYBRID
 *
 * Classical and quantum work may appear within the same data-parallel source
 * structure where the surrounding language permits it.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser/frontend must map these constructs into the existing domain-
 * neutral AST model.
 *
 * Required semantic shapes are:
 *
 *     DataParallelIteration
 *         binding
 *         domain
 *         predicate?
 *         body
 *         modifiers
 *
 *     DataParallelMap
 *         operation
 *         domain
 *         mapper
 *         arguments
 *
 *     DataParallelReduce
 *         operation
 *         domain
 *         reducer
 *         arguments
 *
 *     DataParallelScan
 *         operation
 *         domain
 *         scanner
 *         arguments
 *
 *     DataParallelPartition
 *         operation
 *         domain
 *         partitioner
 *         arguments
 *
 * Source spans MUST be preserved.
 *
 * Source ordering MUST be preserved.
 *
 * No AST node may contain:
 *
 *     worker ID
 *     thread ID
 *     CPU ID
 *     GPU ID
 *     QPU ID
 *     node ID
 *     physical address
 *     queue ID
 *     physical vector width
 *
 * unless such information is explicitly represented downstream as a
 * target-specific realization rather than source semantics.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * The parser produces AST.
 *
 * Semantic analysis produces canonical semantic information.
 *
 * Lowering then maps the semantic operation to the appropriate IR.
 *
 * Depending on the program, this may reach:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed representation
 *     accelerator representation
 *
 * The data-parallel grammar MUST NOT create another parallel IR merely to
 * represent these syntax forms.
 *
 * ============================================================================
 * SCHEDULING CONTRACT
 * ============================================================================
 *
 * Scheduling is downstream.
 *
 * It may choose:
 *
 *     sequential execution
 *     batching
 *     tiling
 *     vectorization
 *     task parallelism
 *     GPU execution
 *     accelerator execution
 *     distributed execution
 *     heterogeneous execution
 *
 * while preserving source semantics.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors are handled by the canonical ANTLR parser/error pipeline.
 *
 * This grammar:
 *
 *     does not print;
 *     does not panic;
 *     does not execute user code;
 *     does not inspect hardware;
 *     does not inspect runtime state;
 *     does not perform I/O;
 *     does not perform resource allocation.
 *
 * Semantic errors belong downstream.
 *
 * ============================================================================
 * VERSIONING CONTRACT
 * ============================================================================
 *
 * Adding a new operation under `parallel::` does not require a new lexer
 * keyword.
 *
 * Breaking changes require synchronized updates to:
 *
 *     language specification
 *     grammar
 *     AST contract
 *     semantic analysis
 *     compatibility documentation
 *     conformance tests
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This is ANTLR grammar source and contains no Rust implementation.
 *
 * Repository implementation requirements:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     no unsafe
 *
 * ============================================================================
 */

parser grammar DataParallel;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the only public dispatcher owned by this file.
 *
 * The canonical concurrency composition grammar should dispatch into this
 * rule rather than duplicating any of the alternatives below.
 */
dataParallelConstruct
    : dataParallelVectorizedIteration
    | dataParallelIteration
    | dataParallelMap
    | dataParallelReduce
    | dataParallelScan
    | dataParallelPartition
    ;


/*
 * ============================================================================
 * DATA-PARALLEL ITERATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     parallel for item in values {
 *         compute(item);
 *     }
 *
 * Optional predicate:
 *
 *     parallel for item in values when predicate(item) {
 *         compute(item);
 *     }
 *
 * IMPORTANT:
 *
 * The predicate appears BEFORE the body.
 *
 * This ordering is deliberate and unambiguous.
 */
dataParallelIteration
    : PARALLEL
      FOR
      bindingPattern
      IN
      expression
      dataParallelPredicateClause?
      blockExpression
    ;


/*
 * ============================================================================
 * DATA-PARALLEL PREDICATE
 * ============================================================================
 *
 * `when` is already a canonical lexical token.
 *
 * The predicate is an ordinary expression.
 */
dataParallelPredicateClause
    : WHEN
      expression
    ;


/*
 * ============================================================================
 * VECTOR-PARALLEL ITERATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     vectorized parallel for item in values {
 *         compute(item);
 *     }
 *
 * `VECTORIZED` expresses semantic intent.
 *
 * It does not select a vector width or instruction set.
 */
dataParallelVectorizedIteration
    : VECTORIZED
      dataParallelIteration
    ;


/*
 * ============================================================================
 * DATA-PARALLEL OPERATION NAMESPACE
 * ============================================================================
 *
 * Because `parallel` is a reserved PARALLEL token, it cannot be consumed by
 * the generic identifier-based qualifiedName rule.
 *
 * Therefore data-parallel library operations use:
 *
 *     parallel :: identifier
 *
 * Examples:
 *
 *     parallel::map
 *     parallel::reduce
 *     parallel::scan
 *     parallel::partition
 *
 * Additional namespace segments are allowed after the mandatory parallel
 * namespace:
 *
 *     parallel::experimental::transform
 *
 * Such names remain semantic/library/dialect names.
 */
dataParallelOperationName
    : PARALLEL
      DOUBLE_COLON
      identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


/*
 * ============================================================================
 * MAP
 * ============================================================================
 *
 * Canonical form:
 *
 *     parallel::map(values, mapper)
 *
 * Additional arguments are permitted for semantic/library-defined mapping
 * operations.
 */
dataParallelMap
    : dataParallelMapOperation
      LPAREN
      expression
      COMMA
      expression
      dataParallelAdditionalArgument*
      RPAREN
    ;

dataParallelMapOperation
    : dataParallelOperationName
    ;

dataParallelAdditionalArgument
    : COMMA
      expression
    ;


/*
 * ============================================================================
 * REDUCE
 * ============================================================================
 *
 * Canonical form:
 *
 *     parallel::reduce(values, reducer)
 *
 * Optional arguments may represent semantic information such as an identity
 * value, ordering policy or domain-specific operation parameters.
 *
 * Their meaning is determined by semantic analysis/library contracts.
 */
dataParallelReduce
    : dataParallelReduceOperation
      LPAREN
      expression
      COMMA
      expression
      dataParallelAdditionalArgument*
      RPAREN
    ;

dataParallelReduceOperation
    : dataParallelOperationName
    ;


/*
 * ============================================================================
 * SCAN
 * ============================================================================
 *
 * Canonical form:
 *
 *     parallel::scan(values, scanner)
 *
 * The grammar does not prescribe the physical scan algorithm.
 */
dataParallelScan
    : dataParallelScanOperation
      LPAREN
      expression
      COMMA
      expression
      dataParallelAdditionalArgument*
      RPAREN
    ;

dataParallelScanOperation
    : dataParallelOperationName
    ;


/*
 * ============================================================================
 * PARTITION
 * ============================================================================
 *
 * Canonical form:
 *
 *     parallel::partition(values, partitioner)
 *
 * Partitioning remains logical.
 *
 * It does not select physical nodes, workers, devices or memory regions.
 */
dataParallelPartition
    : dataParallelPartitionOperation
      LPAREN
      expression
      COMMA
      expression
      dataParallelAdditionalArgument*
      RPAREN
    ;

dataParallelPartitionOperation
    : dataParallelOperationName
    ;


/*
 * ============================================================================
 * EXPRESSION ADAPTER
 * ============================================================================
 *
 * This rule is intentionally small.
 *
 * It exists so the canonical expression composition can recognize the
 * data-parallel value-producing forms without importing this grammar's entire
 * internal rule set.
 */
dataParallelExpression
    : dataParallelMap
    | dataParallelReduce
    | dataParallelScan
    | dataParallelPartition
    ;


/*
 * ============================================================================
 * STATEMENT ADAPTER
 * ============================================================================
 *
 * The canonical statement composition may use this adapter for constructs
 * whose primary semantic result is discarded or whose operation is admitted
 * in statement position.
 *
 * The adapter does not redefine ordinary statement syntax.
 */
dataParallelStatement
    : dataParallelIteration
    | dataParallelExpression SEMI
    ;


/*
 * ============================================================================
 * DATA-PARALLEL OPERATION EXTENSION
 * ============================================================================
 *
 * This rule provides a controlled extension point for library/dialect-defined
 * operations in the reserved `parallel::` namespace.
 *
 * It intentionally requires a call argument list so arbitrary ordinary
 * expressions cannot accidentally become data-parallel operations.
 *
 * Examples:
 *
 *     parallel::transform(values, f)
 *     parallel::filter(values, predicate)
 *     parallel::zip(a, b)
 *     parallel::zip_with(a, b, f)
 *     parallel::window(values, size)
 *     parallel::stencil(values, neighborhood, f)
 */
dataParallelExtension
    : dataParallelOperationName
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * SEMANTIC OPERATION ADAPTER
 * ============================================================================
 *
 * Tooling may use this rule to identify a data-parallel operation without
 * duplicating its individual forms.
 */
dataParallelSemanticOperation
    : dataParallelIteration
    | dataParallelMap
    | dataParallelReduce
    | dataParallelScan
    | dataParallelPartition
    | dataParallelExtension
    ;


/*
 * ============================================================================
 * DOMAIN ADAPTER
 * ============================================================================
 *
 * A data domain is intentionally just an ordinary expression.
 *
 * This keeps the grammar independent of:
 *
 *     collection representation;
 *     tensor representation;
 *     stream implementation;
 *     distributed representation;
 *     accelerator buffers;
 *     quantum measurement storage;
 *     memory layout.
 */
dataParallelDomain
    : expression
    ;


/*
 * ============================================================================
 * MAPPER / REDUCER / SCANNER / PARTITIONER ADAPTERS
 * ============================================================================
 *
 * These are semantic names, not new types.
 *
 * The actual callable/type/effect rules belong downstream.
 */
dataParallelMapper
    : expression
    ;

dataParallelReducer
    : expression
    ;

dataParallelScanner
    : expression
    ;

dataParallelPartitioner
    : expression
    ;

dataParallelPredicate
    : expression
    ;


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical composition:
 *
 *     ZamaniParser
 *          |
 *          v
 *     Concurrency
 *          |
 *          +--> common parallel
 *          |
 *          +--> task parallel
 *          |
 *          +--> data parallel
 *                    |
 *                    v
 *             dataParallelConstruct
 *
 * `concurrency.g4` owns the concurrency-domain composition boundary.
 *
 * It MUST NOT copy the individual rules from this file.
 *
 * `parallel.g4` owns common:
 *
 *     parallelExpression
 *     parallelStatement
 *     parallelBody
 *
 * where those constructs are part of the canonical architecture.
 *
 * This file MUST NOT redefine those rules.
 *
 * `statements/loops.g4` owns ordinary:
 *
 *     forStatement
 *
 * This file does NOT replace or redefine it.
 *
 * The `parallel for` construct is a distinct data-parallel construct because
 * the PARALLEL token occurs before FOR.
 *
 * ============================================================================
 * GRAMMAR DEPENDENCY CONTRACT
 * ============================================================================
 *
 * The surrounding canonical grammar composition supplies:
 *
 *     identifier
 *     expression
 *     argumentList
 *     bindingPattern
 *     blockExpression
 *
 * This component deliberately does not redefine those universal rules.
 *
 * When the repository's ANTLR build composes modular parser grammars, the
 * composition layer MUST resolve those references against their canonical
 * owners.
 *
 * The dependency direction is:
 *
 *     DataParallel
 *          |
 *          +--> Expressions
 *          +--> Bindings
 *          +--> CoreBlocks
 *          +--> canonical lexer vocabulary
 *
 * It MUST NOT depend on:
 *
 *     runtime
 *     compiler backend
 *     hardware
 *     quantum IR
 *     scheduler
 *     resource allocator
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * The parser/frontend must preserve:
 *
 *     source span
 *     binding
 *     domain
 *     predicate
 *     operation
 *     mapper/reducer/scanner/partitioner
 *     additional arguments
 *     body
 *     source order
 *
 * A suitable generic operation representation is preferred over introducing
 * one AST type for every possible parallel library operation.
 *
 * For example:
 *
 *     parallel::map
 *
 * may become a generic operation:
 *
 *     name       = "map"
 *     namespace  = "parallel"
 *
 * while:
 *
 *     parallel::experimental::transform
 *
 * becomes:
 *
 *     namespace  = "parallel::experimental"
 *     name       = "transform"
 *
 * This is semantic data, not a new parser keyword.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Semantic analysis determines whether a data-parallel body is legal.
 *
 * Examples requiring analysis:
 *
 *     parallel writes to disjoint locations
 *     parallel writes to aliased locations
 *     atomic updates
 *     reduction
 *     I/O
 *     external effects
 *     synchronization
 *     mutation
 *
 * The grammar does not classify an operation as pure, deterministic or
 * associative.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource/capability requirements belong to the canonical resource system.
 *
 * Examples of downstream semantic information include:
 *
 *     capability("parallel.compute")
 *     capability("vector.compute")
 *     capability("tensor.compute")
 *     capability("accelerator.compute")
 *
 * Those capabilities must not become physical device identifiers.
 *
 * The distinction remains:
 *
 *     semantic requirement
 *          !=
 *     capability
 *          !=
 *     allocation
 *          !=
 *     placement
 *          !=
 *     scheduling
 *
 * ============================================================================
 * CLASSICAL IR INTEGRATION
 * ============================================================================
 *
 * A classical data-parallel operation may lower into the repository's
 * canonical classical semantic representation.
 *
 * This grammar does not define that representation.
 *
 * ============================================================================
 * QUANTUM IR INTEGRATION
 * ============================================================================
 *
 * If a data-parallel body contains quantum computation:
 *
 *     DataParallel AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar must never introduce:
 *
 *     DataParallelQuantumIR
 *     ParallelQuantumIR
 *     QuantumParallelIR
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A backend may realize data parallelism as:
 *
 *     replication
 *     pipelining
 *     vectorization
 *     spatial parallelism
 *     temporal parallelism
 *     accelerator invocation
 *
 * but none of those physical decisions belong to this grammar.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A semantic partition may eventually be mapped onto:
 *
 *     one process
 *     many processes
 *     one machine
 *     many machines
 *     a cluster
 *     a future distributed substrate
 *
 * The grammar contains no node count or topology.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * Given the same:
 *
 *     source
 *     language version
 *     lexical contract
 *     grammar version
 *
 * the parser must produce the same syntactic structure.
 *
 * Data-parallel execution determinism is a semantic/runtime property.
 *
 * In particular, the parser does not decide whether reduction reordering is
 * permitted.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     fixed worker count
 *     fixed thread count
 *     fixed core count
 *     fixed GPU count
 *     fixed FPGA count
 *     fixed QPU count
 *     fixed node count
 *     fixed vector width
 *     fixed tensor dimension
 *     fixed memory size
 *     fixed topology
 *     physical device identifier
 *     physical address
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     performs no I/O;
 *     executes no user code;
 *     performs no hardware discovery;
 *     performs no network access;
 *     performs no filesystem access;
 *     performs no runtime allocation;
 *     contains no target-language actions.
 *
 * The Rust compiler/frontend implementation remains safe Rust.
 *
 * ============================================================================
 * REQUIRED CONFORMANCE TESTS
 * ============================================================================
 *
 * POSITIVE:
 *
 *     parallel for item in values {
 *         compute(item);
 *     }
 *
 *     parallel for item in values when predicate(item) {
 *         compute(item);
 *     }
 *
 *     vectorized parallel for item in values {
 *         compute(item);
 *     }
 *
 *     parallel::map(values, compute);
 *
 *     parallel::reduce(values, combine);
 *
 *     parallel::scan(values, combine);
 *
 *     parallel::partition(values, partitioner);
 *
 *     parallel::transform(values, transform);
 *
 *     parallel::zip(left, right);
 *
 *     parallel::zip_with(left, right, combine);
 *
 *     parallel::experimental::operation(values, f);
 *
 * NEGATIVE:
 *
 *     parallel for
 *
 *     parallel for item
 *
 *     parallel for item in
 *
 *     parallel for item in values
 *
 *     parallel::map()
 *
 *     parallel::map(values)
 *
 *     parallel::reduce()
 *
 *     vectorized
 *
 *     vectorized for item in values { ... }
 *
 * The last form is invalid because `vectorized` modifies a data-parallel
 * construct, not an ordinary sequential loop.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     empty domains
 *     singleton domains
 *     dynamically sized domains
 *     symbolic domains
 *     nested data-parallel constructs
 *     nested parallel operations
 *     very large logical ranges
 *     streams
 *     tensors
 *     distributed datasets
 *     quantum-derived datasets
 *
 * No test may establish a finite hardware-size ceiling.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Required:
 *
 *     classical + data parallel
 *     tensor + data parallel
 *     AI + data parallel
 *     quantum + data parallel
 *     hybrid + data parallel
 *     HDL/hardware + data parallel
 *     distributed + data parallel
 *     accelerator + data parallel
 *
 * ============================================================================
 * COMPATIBILITY TESTS
 * ============================================================================
 *
 * Verify that this grammar does not break:
 *
 *     ordinary for
 *     ordinary function calls
 *     ordinary qualified names
 *     common parallel blocks
 *     task parallelism
 *     async/await
 *     resource requirements
 *     quantum operation syntax
 *     HDL syntax
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] It uses the canonical Zamani lexer.
 * [ ] It introduces no lexer rules.
 * [ ] It introduces no machine-size limits.
 * [ ] It does not redefine ordinary loops.
 * [ ] It does not redefine common parallel syntax.
 * [ ] It does not redefine task parallelism.
 * [ ] It uses bindingPattern.
 * [ ] It uses canonical expression syntax.
 * [ ] It uses canonical block syntax.
 * [ ] It supports scalable parallel iteration.
 * [ ] It supports predicates.
 * [ ] It supports vectorization intent.
 * [ ] It supports map.
 * [ ] It supports reduce.
 * [ ] It supports scan.
 * [ ] It supports partition.
 * [ ] It supports extensible parallel:: operations.
 * [ ] It does not require MAP/REDUCE/SCAN/PARTITION lexer tokens.
 * [ ] It handles the reserved PARALLEL token correctly.
 * [ ] It preserves source structure for AST construction.
 * [ ] It preserves deterministic parsing.
 * [ ] It has explicit semantic integration.
 * [ ] It has explicit resource integration.
 * [ ] It has explicit compiler integration.
 * [ ] It has explicit runtime integration.
 * [ ] It has explicit quantum::ir integration.
 * [ ] It has explicit HDL/hardware integration.
 * [ ] It has positive tests.
 * [ ] It has negative tests.
 * [ ] It has boundary tests.
 * [ ] It has scalability tests.
 * [ ] It has determinism tests.
 * [ ] It has cross-domain tests.
 * [ ] It has compatibility tests.
 * [ ] It contains no unsafe Rust.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Data-parallel syntax describes:
 *
 *     WHAT CAN BE COMPUTED IN PARALLEL
 *
 * while downstream compilation determines:
 *
 *     WHETHER
 *     WHERE
 *     WHEN
 *     HOW
 *     WITH WHICH AVAILABLE RESOURCES
 *
 * the computation is realized.
 *
 * Therefore:
 *
 *     PROGRAM
 *        |
 *        v
 *     DATA-PARALLEL SEMANTIC INTENT
 *        |
 *        v
 *     AST
 *        |
 *        v
 *     SEMANTIC ANALYSIS
 *        |
 *        +--> effects
 *        +--> dependencies
 *        +--> resources
 *        +--> capabilities
 *        +--> determinism
 *        |
 *        v
 *     CANONICAL IR
 *        |
 *        +--> classical
 *        +--> quantum::ir
 *        +--> HDL/hardware
 *        +--> distributed
 *        +--> accelerator
 *        |
 *        v
 *     OPTIMIZATION
 *        |
 *        v
 *     ROUTING / SCHEDULING / RESILIENCE
 *        |
 *        v
 *     HAL
 *        |
 *        v
 *     TARGET
 *
 * This is the required data-parallel contribution to:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */