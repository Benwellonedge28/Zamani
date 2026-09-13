/*
 * ============================================================================
 * Zamani Programming Language
 * Production Data-Parallel Computation Grammar
 * ============================================================================
 *
 * File:
 *     grammar/concurrency/data-parallel.g4
 *
 * Role:
 *     Reusable ANTLR4 parser-domain grammar for DATA-PARALLEL computation.
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
 *     The Zamani compiler/runtime MUST be implemented using safe Rust only.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - data-parallel source syntax;
 *     - parallel iteration syntax;
 *     - data-parallel mapping intent;
 *     - data-parallel reduction intent;
 *     - data-parallel scan/fold intent where represented by the language;
 *     - data-domain partitioning syntax;
 *     - data-parallel execution hints that are syntactically meaningful;
 *     - data-parallel composition boundaries;
 *     - integration points for classical, tensor, AI, accelerator and
 *       heterogeneous computation.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - general concurrency;
 *     - general parallel blocks;
 *     - task spawning;
 *     - futures;
 *     - actors;
 *     - channels;
 *     - synchronization;
 *     - scheduling;
 *     - resource allocation;
 *     - worker counts;
 *     - thread counts;
 *     - CPU counts;
 *     - GPU counts;
 *     - accelerator counts;
 *     - SIMD widths;
 *     - vector register widths;
 *     - machine topology;
 *     - device identifiers;
 *     - placement;
 *     - routing;
 *     - hardware discovery;
 *     - hardware calibration;
 *     - classical IR;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime dispatch.
 *
 * Those concerns belong to their canonical repository subsystems.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Data-parallel syntax expresses WHAT DATA MAY BE PROCESSED IN PARALLEL.
 *
 * It does not prescribe HOW MANY execution resources process that data.
 *
 * The following are therefore intentionally absent:
 *
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_ACCELERATORS
 *     MAX_LANES
 *     MAX_VECTOR_WIDTH
 *     MAX_ITEMS
 *     MAX_ELEMENTS
 *     MAX_PARTITIONS
 *     MAX_BLOCKS
 *
 * The source program describes a semantic computation over a domain.
 *
 * A target with one available execution resource may serialize the work.
 *
 * A target with many resources may execute independent portions concurrently.
 *
 * A GPU may map work to kernels.
 *
 * A CPU may use scalar, vector, task or thread execution.
 *
 * An FPGA may pipeline or replicate the computation.
 *
 * A distributed target may partition the domain across nodes.
 *
 * A future computational substrate may choose another realization.
 *
 * The source semantics remain unchanged.
 *
 * ============================================================================
 * CORE ARCHITECTURE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *      ordinary syntax              data-parallel syntax
 *                                          |
 *                                          v
 *                                      Frontend AST
 *                                          |
 *                                          v
 *                              semantic/type/effect analysis
 *                                          |
 *                                          v
 *                              dependency/resource analysis
 *                                          |
 *                                          v
 *                                   canonical semantic IR
 *                                          |
 *                              +-----------+-----------+
 *                              |           |           |
 *                              v           v           v
 *                          classical    quantum      other
 *                              |           |           |
 *                              +-----------+-----------+
 *                                          |
 *                                          v
 *                                      optimization
 *                                          |
 *                                          v
 *                                      scheduling
 *                                          |
 *                                          v
 *                                    target lowering
 *                                          |
 *                                          v
 *                                       runtime
 *
 * ============================================================================
 * IMPORTANT OWNERSHIP RULE
 * ============================================================================
 *
 * `grammar/concurrency/concurrency.g4` owns GENERAL parallel/concurrency
 * constructs.
 *
 * `grammar/concurrency/parallel.g4` owns the COMMON parallel-computation
 * boundary where applicable.
 *
 * THIS FILE owns DATA-PARALLEL-specific constructs.
 *
 * Therefore this file MUST NOT redefine:
 *
 *     parallelExpression
 *     parallelStatement
 *
 * when those are already owned by the common concurrency grammar.
 *
 * Instead, this file exposes:
 *
 *     dataParallelConstruct
 *     dataParallelIteration
 *     dataParallelMap
 *     dataParallelReduce
 *     dataParallelScan
 *     dataParallelPartition
 *
 * The composed parser decides where these productions are admitted.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file uses ONLY tokens already established by the canonical lexical
 * contract, including:
 *
 *     PARALLEL
 *     FOR
 *     IN
 *     WHEN
 *     WITH
 *     VECTORIZED
 *
 * and ordinary canonical identifiers/operators where appropriate.
 *
 * This file MUST NOT introduce lexer rules.
 *
 * In particular, it MUST NOT invent:
 *
 *     MAP
 *     REDUCE
 *     SCAN
 *     PARTITION
 *     CHUNK
 *     WORKER
 *     THREAD
 *     CORE
 *     GPU
 *     SIMD
 *
 * as new lexical tokens.
 *
 * Data-parallel operation names that are not reserved language keywords are
 * represented through ordinary identifiers/qualified names.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should be capable of representing:
 *
 *     DataParallelIteration
 *     DataParallelMap
 *     DataParallelReduce
 *     DataParallelScan
 *     DataParallelPartition
 *     DataParallelBody
 *     DataParallelModifier
 *
 * The AST MUST preserve:
 *
 *     - source locations;
 *     - source ordering;
 *     - binding patterns;
 *     - input expressions;
 *     - body expressions/statements;
 *     - optional semantic modifiers;
 *     - explicit reduction/combination expressions.
 *
 * The AST MUST NOT introduce:
 *
 *     worker IDs;
 *     thread IDs;
 *     CPU IDs;
 *     GPU IDs;
 *     device IDs;
 *     physical locations;
 *     runtime queue IDs.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Data-parallel syntax expresses a computation whose independent instances
 * operate over elements of a semantic data domain.
 *
 * Semantic analysis determines:
 *
 *     - whether iterations are independent;
 *     - whether memory accesses conflict;
 *     - whether effects permit parallel execution;
 *     - whether ordering is observable;
 *     - whether reduction is associative;
 *     - whether reduction requires a deterministic ordering;
 *     - whether a partitioning is legal;
 *     - whether a vectorized realization is possible;
 *     - whether a target capability supports the requested semantics.
 *
 * The grammar does not answer those questions.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * A data-parallel construct MUST NOT silently change observable semantics.
 *
 * In particular, a reduction must not automatically assume that floating-point
 * addition, subtraction or arbitrary user functions are associative.
 *
 * Determinism requirements belong to semantic analysis.
 *
 * A program may explicitly request deterministic semantics through the
 * canonical effect/resource/constraint system when such syntax is available.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * A data-parallel domain may be arbitrarily large subject only to:
 *
 *     - source representation;
 *     - semantic validity;
 *     - available memory;
 *     - available execution resources;
 *     - target capabilities;
 *     - runtime/resource policy.
 *
 * Those are NOT grammar limits.
 *
 * A range such as:
 *
 *     0..N
 *
 * remains a semantic range whose size is determined by N.
 *
 * The grammar does not impose a maximum value for N.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * DATA PARALLEL + CLASSICAL
 *
 *     Data-parallel operations can lower into classical IR.
 *
 * DATA PARALLEL + QUANTUM
 *
 *     Data-parallel orchestration may generate or control independent quantum
 *     computations, but quantum semantics remain owned by quantum IR.
 *
 * DATA PARALLEL + HDL
 *
 *     Data-parallel intent may lower into hardware replication, pipelining,
 *     vectorization or other hardware realization, but this grammar does not
 *     choose the implementation.
 *
 * DATA PARALLEL + AI
 *
 *     Tensor/model computations may use data-parallel semantics without this
 *     grammar owning tensor or model types.
 *
 * DATA PARALLEL + DISTRIBUTED
 *
 *     The semantic data domain may be partitioned across execution locations.
 *     Placement and communication remain distributed/runtime concerns.
 *
 * DATA PARALLEL + HARDWARE
 *
 *     Hardware capabilities can influence lowering, but hardware descriptions
 *     must not become embedded into source semantics.
 *
 * ============================================================================
 */

parser grammar DataParallel;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC DATA-PARALLEL DOMAIN ENTRY POINT
 * ============================================================================
 *
 * This is the sole public entry point owned by this grammar.
 *
 * The composed Zamani parser should call `dataParallelConstruct` when entering
 * the data-parallel domain.
 *
 * ============================================================================
 */

dataParallelConstruct
    : dataParallelIteration
    | dataParallelMap
    | dataParallelReduce
    | dataParallelScan
    | dataParallelPartition
    | dataParallelForModifier
    ;


/* ============================================================================
 * 2. DATA-PARALLEL ITERATION
 * ============================================================================
 *
 * Canonical target-independent form:
 *
 *     parallel for item in values {
 *         compute(item);
 *     }
 *
 * This expresses:
 *
 *     for each item in values, compute(item) may be evaluated independently.
 *
 * It does NOT express:
 *
 *     one item == one thread
 *     one item == one core
 *     one item == one GPU lane
 *     one item == one node
 *
 * ============================================================================
 */

dataParallelIteration
    : PARALLEL
      FOR
      pattern
      IN
      expression
      dataParallelBody
      dataParallelWhenClause?
    ;


/* ============================================================================
 * 3. DATA-PARALLEL BODY
 * ============================================================================
 *
 * The body is owned structurally by the ordinary block grammar.
 *
 * This rule exists as a stable integration adapter.
 *
 * ============================================================================
 */

dataParallelBody
    : blockExpression
    ;


/* ============================================================================
 * 4. OPTIONAL ITERATION CONDITION
 * ============================================================================
 *
 * Example:
 *
 *     parallel for item in values
 *         when predicate(item)
 *     {
 *         compute(item);
 *     }
 *
 * `when` is already part of the canonical lexical vocabulary.
 *
 * The condition is an ordinary expression.
 *
 * ============================================================================
 */

dataParallelWhenClause
    : WHEN
      expression
    ;


/* ============================================================================
 * 5. DATA-PARALLEL MAP
 * ============================================================================
 *
 * Data mapping is represented using a qualified operation name rather than
 * introducing a permanently reserved MAP keyword.
 *
 * Canonical conceptual form:
 *
 *     parallel::map(values, operation)
 *
 * Example:
 *
 *     parallel::map(values, compute)
 *
 * The operation identity remains semantic.
 *
 * ============================================================================
 */

dataParallelMap
    : dataParallelOperationName
      LPAREN
      dataParallelMapArguments
      RPAREN
    ;


/* ============================================================================
 * 6. MAP ARGUMENTS
 * ============================================================================
 *
 * The first expression is the semantic input domain.
 *
 * The second expression is the mapping computation.
 *
 * Additional arguments remain possible through the canonical argument list.
 *
 * ============================================================================
 */

dataParallelMapArguments
    : expression
      COMMA
      expression
      (
          COMMA
          expression
      )*
    ;


/* ============================================================================
 * 7. DATA-PARALLEL REDUCE
 * ============================================================================
 *
 * Reduction combines partial results.
 *
 * Conceptual form:
 *
 *     parallel::reduce(values, combine)
 *
 * The grammar deliberately does NOT define:
 *
 *     a reduction tree;
 *     execution order;
 *     number of partial reductions;
 *     worker count;
 *     vector width;
 *     hardware implementation.
 *
 * ============================================================================
 */

dataParallelReduce
    : dataParallelReduceName
      LPAREN
      dataParallelReduceArguments
      RPAREN
    ;


/* ============================================================================
 * 8. REDUCE ARGUMENTS
 * ============================================================================
 *
 * First expression:
 *
 *     input data domain
 *
 * Second expression:
 *
 *     combination operation
 *
 * Optional third and subsequent expressions may carry semantic arguments,
 * such as an explicit identity supplied by the language/library contract.
 *
 * Their interpretation belongs to semantic analysis.
 *
 * ============================================================================
 */

dataParallelReduceArguments
    : expression
      COMMA
      expression
      (
          COMMA
          expression
      )*
    ;


/* ============================================================================
 * 9. DATA-PARALLEL SCAN
 * ============================================================================
 *
 * A scan/prefix computation is distinct from a reduction because it produces
 * an output corresponding to multiple prefixes of the input domain.
 *
 * Conceptual form:
 *
 *     parallel::scan(values, combine)
 *
 * The implementation may use sequential, tree-based, vectorized, pipelined,
 * distributed or other strategies.
 *
 * ============================================================================
 */

dataParallelScan
    : dataParallelScanName
      LPAREN
      dataParallelScanArguments
      RPAREN
    ;


/* ============================================================================
 * 10. SCAN ARGUMENTS
 * ============================================================================
 */

dataParallelScanArguments
    : expression
      COMMA
      expression
      (
          COMMA
          expression
      )*
    ;


/* ============================================================================
 * 11. DATA-PARALLEL PARTITION
 * ============================================================================
 *
 * Partitioning describes semantic decomposition of a data domain.
 *
 * It does NOT select:
 *
 *     nodes;
 *     workers;
 *     threads;
 *     GPUs;
 *     cores;
 *     devices.
 *
 * Conceptual form:
 *
 *     parallel::partition(values, partitioner)
 *
 * ============================================================================
 */

dataParallelPartition
    : dataParallelPartitionName
      LPAREN
      dataParallelPartitionArguments
      RPAREN
    ;


/* ============================================================================
 * 12. PARTITION ARGUMENTS
 * ============================================================================
 */

dataParallelPartitionArguments
    : expression
      COMMA
      expression
      (
          COMMA
          expression
      )*
    ;


/* ============================================================================
 * 13. OPERATION NAMES
 * ============================================================================
 *
 * These names intentionally remain qualified semantic names.
 *
 * They are NOT lexer keywords.
 *
 * This keeps the grammar open to:
 *
 *     parallel::map
 *     parallel::reduce
 *     parallel::scan
 *     parallel::partition
 *
 * and future library/dialect extensions without modifying the global lexer
 * for every new data-parallel algorithm.
 *
 * ============================================================================
 */

dataParallelOperationName
    : qualifiedName
    ;


dataParallelReduceName
    : qualifiedName
    ;


dataParallelScanName
    : qualifiedName
    ;


dataParallelPartitionName
    : qualifiedName
    ;


/* ============================================================================
 * 14. EXPLICIT DATA-PARALLEL FOR MODIFIER
 * ============================================================================
 *
 * This adapter supports integration with canonical loop constructs that expose
 * an explicit parallel modifier.
 *
 * Example conceptual form:
 *
 *     foreach item in values parallel {
 *         compute(item);
 *     }
 *
 * IMPORTANT:
 *
 * This rule does not redefine `foreach`.
 *
 * If the canonical loop grammar already owns that syntax, it should construct
 * the corresponding AST and use this rule only as an integration production.
 *
 * ============================================================================
 */

dataParallelForModifier
    : FOR
      pattern
      IN
      expression
      PARALLEL
      dataParallelBody
    ;


/* ============================================================================
 * 15. DATA-PARALLEL EXPRESSION ADAPTER
 * ============================================================================
 *
 * This adapter provides a stable semantic entry point for expression-oriented
 * data-parallel constructs.
 *
 * ============================================================================
 */

dataParallelExpression
    : dataParallelMap
    | dataParallelReduce
    | dataParallelScan
    | dataParallelPartition
    ;


/* ============================================================================
 * 16. DATA-PARALLEL STATEMENT ADAPTER
 * ============================================================================
 *
 * This adapter allows the composed statement grammar to admit data-parallel
 * operations in statement position.
 *
 * It does not define ordinary statement syntax.
 *
 * ============================================================================
 */

dataParallelStatement
    : dataParallelIteration
      SEMI?
    | dataParallelExpression
      SEMI?
    ;


/* ============================================================================
 * 17. DATA-PARALLEL COMPOSITION
 * ============================================================================
 *
 * Multiple data-parallel operations may be composed.
 *
 * Source ordering remains significant until semantic analysis proves that
 * operations can be reordered.
 *
 * ============================================================================
 */

dataParallelComposition
    : PARALLEL
      LBRACE
      dataParallelElement*
      RBRACE
    ;


dataParallelElement
    : dataParallelIteration
    | dataParallelExpression
    | statement
    ;


/* ============================================================================
 * 18. OPTIONAL VECTORIZATION INTENT
 * ============================================================================
 *
 * `vectorized` is already a canonical lexer token.
 *
 * This syntax expresses a REQUEST/INTENT, not a machine vector width.
 *
 * Example:
 *
 *     vectorized parallel for item in values {
 *         compute(item);
 *     }
 *
 * The target may realize the computation through:
 *
 *     SIMD
 *     SVE
 *     GPU
 *     FPGA
 *     scalar execution
 *     another vector-like mechanism
 *
 * or may reject the requested semantic constraint if it cannot satisfy it.
 *
 * ============================================================================
 */

vectorizedDataParallelIteration
    : VECTORIZED
      dataParallelIteration
    ;


/* ============================================================================
 * 19. DATA-PARALLEL MODIFIER
 * ============================================================================
 *
 * Stable adapter for future data-parallel modifiers.
 *
 * The modifier itself is represented through canonical identifiers so the
 * grammar does not hard-code a growing list of backend-specific strategies.
 *
 * ============================================================================
 */

dataParallelModifier
    : qualifiedName
    ;


/* ============================================================================
 * 20. MODIFIED DATA-PARALLEL ITERATION
 * ============================================================================
 *
 * Example conceptual form:
 *
 *     parallel for item in values {
 *         compute(item);
 *     }
 *
 * followed by semantic attributes/modifiers supplied by the surrounding
 * grammar.
 *
 * This rule intentionally remains conservative so it cannot accidentally
 * consume arbitrary syntax as a data-parallel modifier.
 *
 * ============================================================================
 */

dataParallelModifiedConstruct
    : dataParallelModifier
      dataParallelConstruct
    ;


/* ============================================================================
 * 21. DATA DOMAIN
 * ============================================================================
 *
 * A data domain is represented by an ordinary expression.
 *
 * This means the grammar does not distinguish a fixed collection type from:
 *
 *     a lazy stream;
 *     a tensor;
 *     a matrix;
 *     a distributed dataset;
 *     a generated range;
 *     a quantum measurement result;
 *     a hardware buffer;
 *     an accelerator view;
 *     a future data structure.
 *
 * Type and effect analysis determines the actual semantics.
 *
 * ============================================================================
 */

dataParallelDomain
    : expression
    ;


/* ============================================================================
 * 22. DATA-PARALLEL BINDING
 * ============================================================================
 *
 * A binding uses the canonical pattern grammar.
 *
 * This permits future pattern systems without coupling this grammar to one
 * concrete element representation.
 *
 * ============================================================================
 */

dataParallelBinding
    : pattern
    ;


/* ============================================================================
 * 23. DATA-PARALLEL COMPUTATION
 * ============================================================================
 *
 * The computation remains an ordinary block/expression.
 *
 * No special callable type is invented here.
 *
 * ============================================================================
 */

dataParallelComputation
    : blockExpression
    | expression
    ;


/* ============================================================================
 * 24. DATA-PARALLEL ITERATION CORE
 * ============================================================================
 *
 * Reusable internal production.
 *
 * This is intentionally separated from the `parallel for` public syntax so
 * other grammar domains can integrate with the same semantic structure without
 * redefining the body or binding rules.
 *
 * ============================================================================
 */

dataParallelIterationCore
    : dataParallelBinding
      IN
      dataParallelDomain
      dataParallelComputation
    ;


/* ============================================================================
 * 25. DATA-PARALLEL MAP CORE
 * ============================================================================
 *
 * Semantic decomposition:
 *
 *     domain
 *     mapper
 *
 * ============================================================================
 */

dataParallelMapCore
    : dataParallelDomain
      COMMA
      dataParallelComputation
    ;


/* ============================================================================
 * 26. DATA-PARALLEL REDUCE CORE
 * ============================================================================
 *
 * Semantic decomposition:
 *
 *     domain
 *     reducer
 *     optional semantic arguments
 *
 * ============================================================================
 */

dataParallelReduceCore
    : dataParallelDomain
      COMMA
      dataParallelComputation
      (
          COMMA
          expression
      )*
    ;


/* ============================================================================
 * 27. DATA-PARALLEL SCAN CORE
 * ============================================================================
 */

dataParallelScanCore
    : dataParallelDomain
      COMMA
      dataParallelComputation
      (
          COMMA
          expression
      )*
    ;


/* ============================================================================
 * 28. DATA-PARALLEL PARTITION CORE
 * ============================================================================
 */

dataParallelPartitionCore
    : dataParallelDomain
      COMMA
      dataParallelComputation
      (
          COMMA
          expression
      )*
    ;


/* ============================================================================
 * 29. DATA-PARALLEL PREDICATE
 * ============================================================================
 *
 * A predicate is an ordinary expression.
 *
 * It may be used by semantic analysis to determine whether an element
 * participates in a computation.
 *
 * ============================================================================
 */

dataParallelPredicate
    : expression
    ;


/* ============================================================================
 * 30. FILTERED DATA-PARALLEL ITERATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     parallel for item in values when predicate(item) {
 *         compute(item);
 *     }
 *
 * The predicate remains an ordinary expression.
 *
 * ============================================================================
 */

filteredDataParallelIteration
    : PARALLEL
      FOR
      dataParallelBinding
      IN
      dataParallelDomain
      WHEN
      dataParallelPredicate
      dataParallelBody
    ;


/* ============================================================================
 * 31. DATA-PARALLEL VALUE PRODUCTION
 * ============================================================================
 *
 * This adapter exists so semantic analysis can distinguish a data-parallel
 * construct intended to produce values from one intended only for effects.
 *
 * The grammar does not determine whether an operation is pure.
 *
 * ============================================================================
 */

dataParallelValueExpression
    : dataParallelMap
    | dataParallelReduce
    | dataParallelScan
    | dataParallelPartition
    ;


/* ============================================================================
 * 32. DATA-PARALLEL EFFECTFUL ITERATION
 * ============================================================================
 *
 * Effect legality belongs to semantic/effect analysis.
 *
 * This grammar merely records the structural form.
 *
 * ============================================================================
 */

dataParallelEffectfulIteration
    : PARALLEL
      FOR
      dataParallelBinding
      IN
      dataParallelDomain
      dataParallelBody
    ;


/* ============================================================================
 * 33. DATA-PARALLEL DOMAIN COMPOSITION
 * ============================================================================
 *
 * Domains can be represented by arbitrary canonical expressions.
 *
 * This prevents assumptions about:
 *
 *     collection size;
 *     dimensionality;
 *     storage;
 *     layout;
 *     memory location;
 *     machine representation.
 *
 * ============================================================================
 */

dataParallelDomainExpression
    : expression
    ;


/* ============================================================================
 * 34. DATA-PARALLEL REDUCTION SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This named rule exists specifically for semantic analysis.
 *
 * The semantic layer must determine:
 *
 *     - identity availability;
 *     - associativity;
 *     - commutativity;
 *     - determinism;
 *     - numerical stability;
 *     - side effects;
 *     - ordering requirements;
 *     - overflow/underflow semantics;
 *     - target support.
 *
 * ============================================================================
 */

dataParallelReductionBoundary
    : dataParallelReduce
    ;


/* ============================================================================
 * 35. DATA-PARALLEL ITERATION SEMANTIC BOUNDARY
 * ============================================================================
 */

dataParallelIterationBoundary
    : dataParallelIteration
    | filteredDataParallelIteration
    ;


/* ============================================================================
 * 36. DATA-PARALLEL VECTORIZATION BOUNDARY
 * ============================================================================
 */

dataParallelVectorizationBoundary
    : vectorizedDataParallelIteration
    ;


/* ============================================================================
 * 37. DATA-PARALLEL DOMAIN BOUNDARY
 * ============================================================================
 */

dataParallelDomainBoundary
    : dataParallelDomainExpression
    ;


/* ============================================================================
 * 38. DATA-PARALLEL COMBINER
 * ============================================================================
 *
 * A combiner is an ordinary computation expression.
 *
 * This intentionally avoids creating a special "Reducer" grammar type.
 *
 * ============================================================================
 */

dataParallelCombiner
    : expression
    ;


/* ============================================================================
 * 39. DATA-PARALLEL MAPPER
 * ============================================================================
 */

dataParallelMapper
    : expression
    ;


/* ============================================================================
 * 40. DATA-PARALLEL PARTITIONER
 * ============================================================================
 */

dataParallelPartitioner
    : expression
    ;


/* ============================================================================
 * 41. DATA-PARALLEL SEMANTIC OPERATION
 * ============================================================================
 *
 * Common adapter for semantic tooling.
 * ============================================================================
 */

dataParallelSemanticOperation
    : dataParallelMap
    | dataParallelReduce
    | dataParallelScan
    | dataParallelPartition
    | dataParallelIteration
    ;


/* ============================================================================
 * 42. DATA-PARALLEL EXTENSION
 * ============================================================================
 *
 * Future data-parallel operations may be introduced through qualified names
 * without changing the global lexer.
 *
 * Example conceptual forms:
 *
 *     parallel::transform(...)
 *     parallel::zip(...)
 *     parallel::zip_with(...)
 *     parallel::filter(...)
 *     parallel::group(...)
 *     parallel::window(...)
 *     parallel::stencil(...)
 *
 * These names are resolved by the active semantic/library/dialect registry.
 *
 * ============================================================================
 */

dataParallelExtension
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 43. DATA-PARALLEL EXTENSION STATEMENT
 * ============================================================================
 */

dataParallelExtensionStatement
    : dataParallelExtension
      SEMI?
    ;


/* ============================================================================
 * 44. DATA-PARALLEL EXTENSION EXPRESSION
 * ============================================================================
 */

dataParallelExtensionExpression
    : dataParallelExtension
    ;


/* ============================================================================
 * 45. DATA-PARALLEL DOMAIN ELEMENT
 * ============================================================================
 *
 * This production is intentionally generic.
 *
 * It does not impose an element type.
 *
 * Type checking determines whether the element can participate in the
 * requested computation.
 *
 * ============================================================================
 */

dataParallelElementExpression
    : expression
    ;


/* ============================================================================
 * 46. DATA-PARALLEL INDEXING BOUNDARY
 * ============================================================================
 *
 * Indexing is deliberately delegated to the ordinary expression grammar.
 *
 * This rule exists only as a semantic adapter.
 * ============================================================================
 */

dataParallelIndexExpression
    : expression
    ;


/* ============================================================================
 * 47. DATA-PARALLEL RESULT
 * ============================================================================
 *
 * Result structure is determined by semantic typing.
 *
 * No fixed result container is imposed.
 *
 * ============================================================================
 */

dataParallelResultExpression
    : expression
    ;


/* ============================================================================
 * 48. DATA-PARALLEL CONSTRAINT
 * ============================================================================
 *
 * Constraints belong to the canonical resource/constraint system.
 *
 * This grammar intentionally does not encode:
 *
 *     worker count;
 *     memory capacity;
 *     device count;
 *     vector width;
 *     node count.
 *
 * ============================================================================
 */

dataParallelConstraintExpression
    : expression
    ;


/* ============================================================================
 * 49. DATA-PARALLEL INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical parser integration:
 *
 *     ZamaniParser
 *          |
 *          +--> concurrencyConstruct
 *          |
 *          +--> dataParallelConstruct
 *
 * OR, where the root parser owns ordering:
 *
 *     concurrencyConstruct
 *          |
 *          +--> common parallel syntax
 *          |
 *          +--> dataParallelConstruct
 *
 * The exact dispatch point belongs to the composed parser.
 *
 * This file MUST NOT modify the ownership of `concurrencyConstruct`.
 *
 * ============================================================================
 */


/* ============================================================================
 * 50. AST LOWERING CONTRACT
 * ============================================================================
 *
 * Parser:
 *
 *     dataParallelIteration
 *          -> AST DataParallelIteration
 *
 *     dataParallelMap
 *          -> AST DataParallelMap
 *
 *     dataParallelReduce
 *          -> AST DataParallelReduce
 *
 *     dataParallelScan
 *          -> AST DataParallelScan
 *
 *     dataParallelPartition
 *          -> AST DataParallelPartition
 *
 * Semantic analysis then determines:
 *
 *     independence
 *     effects
 *     dependencies
 *     resource requirements
 *     determinism
 *     legal transformations
 *
 * ============================================================================
 */


/* ============================================================================
 * 51. IR INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT define an IR.
 *
 * Lowering is responsible for translating data-parallel AST constructs into
 * the canonical IR used by the relevant repository subsystem.
 *
 * Classical computations:
 *
 *     -> classical/canonical computation IR
 *
 * Quantum computations:
 *
 *     -> quantum::ir
 *
 * Hardware computations:
 *
 *     -> hardware/HDL lowering
 *
 * Distributed computations:
 *
 *     -> distributed execution representation
 *
 * The grammar never directly emits any of those representations.
 *
 * ============================================================================
 */


/* ============================================================================
 * 52. OPTIMIZATION CONTRACT
 * ============================================================================
 *
 * Optimization may transform:
 *
 *     map
 *     reduce
 *     scan
 *     iteration
 *     partition
 *
 * into:
 *
 *     vectorized computation
 *     tiled computation
 *     fused computation
 *     pipelined computation
 *     distributed computation
 *     accelerator computation
 *
 * only where semantic equivalence is proven.
 *
 * This grammar does not choose those transformations.
 *
 * ============================================================================
 */


/* ============================================================================
 * 53. SCHEDULING CONTRACT
 * ============================================================================
 *
 * Scheduling determines actual realization.
 *
 * Examples:
 *
 *     one worker
 *     many workers
 *     vector execution
 *     GPU execution
 *     accelerator execution
 *     distributed execution
 *
 * No scheduler rule belongs here.
 *
 * ============================================================================
 */


/* ============================================================================
 * 54. HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware capability information is supplied by the hardware/target system.
 *
 * This grammar MUST NOT contain:
 *
 *     CPU count
 *     GPU count
 *     core count
 *     thread count
 *     SIMD width
 *     device ID
 *     topology
 *     memory capacity
 *     accelerator count
 *
 * ============================================================================
 */


/* ============================================================================
 * 55. QUANTUM CONTRACT
 * ============================================================================
 *
 * Data-parallel execution may orchestrate multiple quantum computations.
 *
 * However:
 *
 *     quantum operations
 *     qubits
 *     logical qubits
 *     physical qubits
 *     gates
 *     measurements
 *     QEC
 *     ZQN
 *
 * remain owned by the quantum subsystem.
 *
 * If a data-parallel operation produces quantum work, semantic lowering is
 * responsible for producing the appropriate canonical quantum representation.
 *
 * This grammar never creates a quantum IR.
 *
 * ============================================================================
 */


/* ============================================================================
 * 56. MEMORY CONTRACT
 * ============================================================================
 *
 * Data-parallel syntax does not imply:
 *
 *     contiguous memory;
 *     shared memory;
 *     distributed memory;
 *     coherent memory;
 *     cache locality;
 *     NUMA locality.
 *
 * Those properties are determined by types, effects, resources, target
 * capabilities and lowering.
 *
 * ============================================================================
 */


/* ============================================================================
 * 57. EFFECT CONTRACT
 * ============================================================================
 *
 * A data-parallel body may be:
 *
 *     pure
 *     read-only
 *     stateful
 *     effectful
 *     synchronized
 *     externally observable
 *
 * The grammar does not classify those properties.
 *
 * Effect analysis MUST determine whether parallel evaluation preserves
 * semantics.
 *
 * ============================================================================
 */


/* ============================================================================
 * 58. DETERMINISM CONTRACT
 * ============================================================================
 *
 * For operations where execution order can affect observable results,
 * semantic analysis MUST NOT silently assume arbitrary reordering is legal.
 *
 * In particular:
 *
 *     reduction
 *     scan
 *     floating-point computation
 *     side-effectful operations
 *
 * require appropriate semantic validation.
 *
 * ============================================================================
 */


/* ============================================================================
 * 59. SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no source-level finite limit on:
 *
 *     data-domain size;
 *     iteration count;
 *     number of independent operations;
 *     nesting depth;
 *     tensor extent;
 *     collection extent;
 *     distributed partition count.
 *
 * Actual limits, when they exist, are supplied by:
 *
 *     compiler resource policies;
 *     host resources;
 *     target capabilities;
 *     runtime policies;
 *     memory availability;
 *     deployment constraints.
 *
 * They MUST NOT be encoded as grammar constants.
 *
 * ============================================================================
 */


/* ============================================================================
 * 60. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_ACCELERATORS
 *     MAX_ELEMENTS
 *     MAX_ITEMS
 *     MAX_PARTITIONS
 *     MAX_VECTOR_WIDTH
 *     DEVICE_0
 *     GPU_0
 *     CORE_0
 *     THREAD_0
 *
 * No physical resource identity is permitted.
 *
 * ============================================================================
 */


/* ============================================================================
 * 61. SECURITY CONTRACT
 * ============================================================================
 *
 * The grammar performs no I/O.
 *
 * It performs no filesystem access.
 *
 * It performs no network access.
 *
 * It executes no user code.
 *
 * It contains no target-language actions.
 *
 * It does not dynamically load libraries.
 *
 * It does not resolve hardware.
 *
 * ============================================================================
 */


/* ============================================================================
 * 62. ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors are emitted through the canonical ANTLR parser error
 * mechanism.
 *
 * This grammar MUST NOT:
 *
 *     print to stdout;
 *     panic;
 *     silently recover invalid data-parallel constructs;
 *     convert invalid syntax into comments;
 *     silently reinterpret invalid syntax as ordinary computation.
 *
 * Semantic errors such as:
 *
 *     reduction not associative where required;
 *     conflicting writes;
 *     invalid effect combination;
 *     unsupported capability;
 *     impossible resource requirement;
 *
 * belong to semantic diagnostics rather than this grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 63. VERSIONING CONTRACT
 * ============================================================================
 *
 * The grammar follows the active Zamani language version.
 *
 * Adding a new data-parallel operation does not automatically require a new
 * reserved keyword.
 *
 * Qualified semantic operation names permit compatible extension.
 *
 * Breaking changes require:
 *
 *     language specification update;
 *     grammar update;
 *     AST update;
 *     semantic update;
 *     compatibility documentation;
 *     migration guidance;
 *     regression tests.
 *
 * ============================================================================
 */


/* ============================================================================
 * 64. TEST CONTRACT
 * ============================================================================
 *
 * REQUIRED POSITIVE TESTS
 *
 *     parallel for item in values {
 *         compute(item);
 *     }
 *
 *     parallel for item in values when predicate(item) {
 *         compute(item);
 *     }
 *
 *     parallel::map(values, compute)
 *
 *     parallel::reduce(values, combine)
 *
 *     parallel::scan(values, combine)
 *
 *     parallel::partition(values, partitioner)
 *
 * REQUIRED VECTOR TEST
 *
 *     vectorized parallel for item in values {
 *         compute(item);
 *     }
 *
 * REQUIRED CROSS-DOMAIN TESTS
 *
 *     classical + data parallel
 *     tensor + data parallel
 *     AI + data parallel
 *     quantum orchestration + data parallel
 *     HDL/hardware + data parallel
 *     distributed + data parallel
 *     accelerator + data parallel
 *
 * ============================================================================
 */


/* ============================================================================
 * 65. REQUIRED NEGATIVE TESTS
 * ============================================================================
 *
 * The following MUST NOT become valid merely because this grammar is
 * extensible:
 *
 *     parallel for
 *
 *     parallel for item
 *
 *     parallel for item in
 *
 *     parallel for item in values
 *
 * when the required body is absent.
 *
 * Also reject malformed calls such as:
 *
 *     parallel::map()
 *
 * when the semantic operation requires the required argument structure.
 *
 * Syntax/semantic responsibility must remain clearly separated.
 *
 * ============================================================================
 */


/* ============================================================================
 * 66. SCALABILITY TESTS
 * ============================================================================
 *
 * Tests must demonstrate that the grammar does not impose a machine-size
 * ceiling.
 *
 * Examples should vary:
 *
 *     tiny data domains;
 *     large data domains;
 *     dynamically determined domains;
 *     nested domains;
 *     generated ranges;
 *     tensors;
 *     streams;
 *     distributed datasets.
 *
 * The grammar MUST NOT require changing syntax as the target scales.
 *
 * ============================================================================
 */


/* ============================================================================
 * 67. DETERMINISM TESTS
 * ============================================================================
 *
 * Parsing the same source with the same language version MUST produce the same
 * parse structure.
 *
 * Source ordering MUST be preserved.
 *
 * The parser MUST NOT depend on:
 *
 *     hardware;
 *     thread scheduling;
 *     runtime availability;
 *     random values;
 *     device discovery.
 *
 * ============================================================================
 */


/* ============================================================================
 * 68. ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where a canonical AST printer/serializer exists:
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
 *     printer
 *       |
 *       v
 *     parser
 *
 * must preserve the intended data-parallel semantic structure.
 *
 * ============================================================================
 */


/* ============================================================================
 * 69. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 * [ ] It is a valid ANTLR4 parser grammar.
 *
 * [ ] It uses the canonical Zamani lexer vocabulary.
 *
 * [ ] It introduces no undocumented lexer tokens.
 *
 * [ ] It does not redefine `parallelExpression`.
 *
 * [ ] It does not redefine `parallelStatement`.
 *
 * [ ] It does not duplicate general concurrency ownership.
 *
 * [ ] It supports scalable data-parallel iteration.
 *
 * [ ] It supports data-parallel map semantics.
 *
 * [ ] It supports reduction semantics.
 *
 * [ ] It supports scan semantics.
 *
 * [ ] It supports semantic partitioning.
 *
 * [ ] It supports optional predicates.
 *
 * [ ] It supports vectorization intent without vector-width assumptions.
 *
 * [ ] It does not contain machine-size constants.
 *
 * [ ] It does not identify physical workers.
 *
 * [ ] It does not identify physical devices.
 *
 * [ ] It does not define scheduling.
 *
 * [ ] It does not define runtime execution.
 *
 * [ ] It does not define classical IR.
 *
 * [ ] It does not define quantum IR.
 *
 * [ ] It does not define QEC.
 *
 * [ ] It does not define ZQN.
 *
 * [ ] It does not define hardware topology.
 *
 * [ ] It does not require unsafe Rust.
 *
 * [ ] Its AST integration contract is documented.
 *
 * [ ] Its semantic integration contract is documented.
 *
 * [ ] Its IR integration contract is documented.
 *
 * [ ] Its scheduling integration contract is documented.
 *
 * [ ] Its hardware integration contract is documented.
 *
 * [ ] Its quantum integration contract is documented.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] No downstream file needs to redefine the fundamental data-parallel
 *     semantics established here.
 *
 * ============================================================================
 */