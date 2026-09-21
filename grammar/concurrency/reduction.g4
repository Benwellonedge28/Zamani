/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/concurrency/reduction.g4
 *
 * STATUS
 * ------
 * CANONICAL REDUCTION DOMAIN / COMPOSITION COMPONENT
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
 * This file defines the canonical REDUCTION DOMAIN boundary.
 *
 * A reduction combines a logical data domain into one logical result using
 * an explicitly supplied reduction operation.
 *
 * IMPORTANT:
 *
 * This file does NOT redefine the concrete reduction call syntax.
 *
 * The existing canonical syntax:
 *
 *     parallel::reduce(values, reducer)
 *
 * is already owned by:
 *
 *     grammar/concurrency/data-parallel.g4
 *
 * This file therefore provides the reduction-specific composition boundary
 * around that canonical production.
 *
 * This separation is deliberate:
 *
 *     data-parallel.g4
 *         owns concrete data-parallel syntax
 *
 *     reduction.g4
 *         owns reduction-domain classification and integration
 *
 * This prevents two grammar files from independently defining:
 *
 *     reduce(...)
 *
 *     parallel::reduce(...)
 *
 * and therefore prevents duplicate syntax ownership.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * REDUCTION IS A SEMANTIC OPERATION.
 *
 * The grammar describes:
 *
 *     input domain
 *     reduction operation
 *     optional semantic arguments
 *
 * It does NOT describe:
 *
 *     worker count
 *     thread count
 *     CPU count
 *     GPU count
 *     vector width
 *     accelerator count
 *     node count
 *     memory capacity
 *     physical placement
 *     scheduling topology
 *     reduction tree shape
 *     physical communication topology
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     reductionConstruct
 *     reductionExpression
 *     reductionStatement
 *     reductionOperation
 *     reductionDomain
 *     reductionOperator
 *     reductionSemanticOperation
 *
 * THIS FILE DOES NOT OWN:
 *
 *     dataParallelReduce
 *     dataParallelReduceOperation
 *     ordinary expressions
 *     ordinary statements
 *     argument lists
 *     bindings
 *     blocks
 *     functions
 *     types
 *     tasks
 *     spawning
 *     awaiting
 *     scheduling
 *     routing
 *     resource allocation
 *     hardware discovery
 *     classical IR
 *     quantum IR
 *     HDL IR
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * CANONICAL OWNERS
 * ============================================================================
 *
 * Concrete reduction syntax:
 *
 *     grammar/concurrency/data-parallel.g4
 *
 * Common expressions:
 *
 *     grammar/expressions/
 *
 * Statements:
 *
 *     grammar/statements/
 *
 * Tasks:
 *
 *     grammar/concurrency/tasks.g4
 *
 * Task parallelism:
 *
 *     grammar/concurrency/task-parallel.g4
 *
 * Common parallelism:
 *
 *     grammar/concurrency/parallel.g4
 *
 * Concurrency composition:
 *
 *     grammar/concurrency/concurrency.g4
 *
 * Universal parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical root:
 *
 *     grammar/Zamani.g4
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Reduction is LOGICAL.
 *
 * For example:
 *
 *     parallel::reduce(values, combine)
 *
 * does not specify how many physical execution resources are used.
 *
 * The same source may therefore be realized using:
 *
 *     one execution context
 *     multiple CPU cores
 *     SIMD/vector execution
 *     GPU execution
 *     FPGA execution
 *     accelerator execution
 *     distributed execution
 *     heterogeneous execution
 *     quantum/classical orchestration
 *     another future computational substrate
 *
 * provided the target satisfies the semantic requirements.
 *
 * There is no grammar-level maximum for:
 *
 *     input elements
 *     reduction depth
 *     reduction groups
 *     logical parallelism
 *     data partitions
 *     workers
 *     threads
 *     cores
 *     GPUs
 *     FPGAs
 *     accelerators
 *     QPUs
 *     nodes
 *     memory
 *     vector width
 *     tensor dimensions
 *
 * "Infinity" means:
 *
 *     no artificial language-level finite maximum.
 *
 * Actual execution remains bounded by available resources and implementation
 * capabilities.
 *
 * ============================================================================
 * NO HARD-CODED MACHINE LIMITS
 * ============================================================================
 *
 * This file MUST NOT introduce constructs such as:
 *
 *     MAX_REDUCTION_SIZE
 *     MAX_ELEMENTS
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_DIMENSION
 *
 * A numeric literal appearing inside a Zamani expression remains program
 * semantics.
 *
 * For example:
 *
 *     parallel::reduce(values, combine, 1024)
 *
 * may be valid if the surrounding language semantics give 1024 meaning.
 *
 * It must never be interpreted by this grammar as a universal resource limit.
 *
 * ============================================================================
 * REDUCTION SEMANTICS
 * ============================================================================
 *
 * A reduction is not automatically associative.
 *
 * The following may have materially different semantics:
 *
 *     integer addition
 *     floating-point addition
 *     subtraction
 *     string concatenation
 *     matrix multiplication
 *     quantum observable aggregation
 *     probabilistic aggregation
 *     user-defined functions
 *     stateful operations
 *
 * Therefore this grammar does NOT declare:
 *
 *     associative
 *     commutative
 *     deterministic
 *     numerically stable
 *
 * as syntactic facts.
 *
 * Semantic analysis must determine these properties from:
 *
 *     types
 *     effects
 *     operation contracts
 *     library definitions
 *     dialect definitions
 *     attributes
 *     resource/capability contracts
 *     language semantics
 *
 * ============================================================================
 * REASSOCIATION
 * ============================================================================
 *
 * A compiler may only transform:
 *
 *     (((a op b) op c) op d)
 *
 * into another reduction tree when semantic analysis proves that the
 * transformation preserves the language-defined result.
 *
 * This is especially important for:
 *
 *     floating-point arithmetic
 *     effects
 *     stateful operations
 *     externally observable operations
 *     quantum measurements
 *     nondeterministic operations
 *
 * The grammar does not authorize reassociation merely because an operation
 * appears syntactically in a reduction.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Reduction syntax does not imply nondeterminism.
 *
 * The semantic layer determines whether:
 *
 *     ordering matters;
 *     reassociation is legal;
 *     a deterministic reduction tree is required;
 *     an implementation may choose any legal reduction tree.
 *
 * A deterministic source program must remain semantically deterministic after
 * lowering.
 *
 * Runtime scheduling choices must not silently alter observable program
 * semantics.
 *
 * ============================================================================
 * EMPTY INPUT
 * ============================================================================
 *
 * The grammar does not decide whether an empty reduction domain is legal.
 *
 * That decision belongs to semantic/type analysis because validity depends on
 * the reduction operation and its identity element.
 *
 * For example:
 *
 *     sum(empty)
 *
 * may have a defined identity,
 *
 * while:
 *
 *     max(empty)
 *
 * may not.
 *
 * Therefore:
 *
 *     grammar -> accepts structural form
 *     semantics -> determines whether the operation is valid
 *
 * ============================================================================
 * IDENTITY
 * ============================================================================
 *
 * An identity value is semantic information.
 *
 * If supplied by the canonical data-parallel syntax as an additional argument,
 * it remains an ordinary expression.
 *
 * This grammar does not introduce a special IDENTITY keyword.
 *
 * Semantic analysis determines whether an identity:
 *
 *     exists;
 *     has the correct type;
 *     is valid for the operation;
 *     preserves the operation's semantics.
 *
 * ============================================================================
 * ORDERING
 * ============================================================================
 *
 * Reduction ordering is not a physical topology.
 *
 * A source-level semantic requirement such as ordered reduction is distinct
 * from a backend implementation decision such as:
 *
 *     tree reduction
 *     pairwise reduction
 *     local reduction
 *     distributed tree
 *     GPU reduction
 *     vector reduction
 *
 * Physical ordering and communication topology remain downstream concerns.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Reduction may operate over data produced by quantum computation.
 *
 * Examples include:
 *
 *     measurement results
 *     expectation values
 *     sampled observables
 *     statistical aggregates
 *     classical values derived from quantum execution
 *
 * This grammar does NOT define quantum operations.
 *
 * It does NOT define:
 *
 *     qubits
 *     gates
 *     physical qubits
 *     QEC
 *     ZQN
 *     calibration
 *     pulse schedules
 *     routing
 *
 * The canonical path remains:
 *
 *     Zamani source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
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
 *     QEC / resilience
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target
 *
 * A reduction over quantum-produced data may therefore coexist with the
 * canonical quantum semantic boundary without introducing a second quantum IR.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Reduction may operate over:
 *
 *     scalars
 *     arrays
 *     slices
 *     vectors
 *     matrices
 *     tensors
 *     streams
 *     collections
 *     generated domains
 *     distributed datasets
 *     accelerator data
 *
 * The type system and semantic layer determine validity.
 *
 * This grammar does not enumerate numerical types.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Reduction may be used for:
 *
 *     tensor aggregation
 *     batch aggregation
 *     gradient accumulation
 *     statistics
 *     model metrics
 *     distributed training
 *     inference aggregation
 *
 * The grammar does not encode framework-specific operations.
 *
 * Framework semantics belong to libraries, dialects and downstream compiler
 * infrastructure.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Reduction may lower into:
 *
 *     combinational reduction trees
 *     pipelined reductions
 *     replicated reductions
 *     vector reductions
 *     streaming reductions
 *     hardware accelerators
 *
 * None of these physical structures is prescribed by this grammar.
 *
 * A source reduction describes computational meaning.
 *
 * Hardware synthesis decides the realization.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Reduction may be distributed over logical partitions.
 *
 * The grammar does not specify:
 *
 *     node IDs
 *     network addresses
 *     network topology
 *     number of nodes
 *     communication paths
 *     collective implementation
 *
 * Those belong to distributed semantic analysis, scheduling, routing and
 * deployment.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Reduction may require capabilities such as:
 *
 *     parallel reduction
 *     associative aggregation
 *     deterministic reduction
 *     distributed aggregation
 *     accelerator reduction
 *     tensor reduction
 *
 * Capability names remain semantic data.
 *
 * This file does not add capability keywords or hardware identifiers.
 *
 * Resource requirements belong downstream to:
 *
 *     resources/
 *     hardware/
 *     compile/
 *     execution/
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The reduction construct must map into the existing domain-neutral AST
 * architecture.
 *
 * The AST must preserve at minimum:
 *
 *     operation identity
 *     input/domain expression
 *     reducer expression
 *     optional semantic arguments
 *     source span
 *     source ordering
 *     attributes supplied by surrounding language constructs
 *
 * The grammar must not require a dedicated hardware-specific AST node.
 *
 * A suitable semantic shape is:
 *
 *     Reduction
 *       domain
 *       operation
 *       reducer
 *       arguments
 *       attributes
 *       source
 *
 * If the existing generic Operation model is used, the reduction may be
 * represented as:
 *
 *     Operation {
 *         name
 *         namespace
 *         operands
 *         parameters
 *         results
 *         attributes
 *         modifiers
 *         effects
 *         capabilities
 *         source
 *     }
 *
 * No worker/core/device information belongs in this AST representation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must validate:
 *
 *     domain existence
 *     reducer existence
 *     reducer arity
 *     operand/result types
 *     identity validity where supplied
 *     associativity requirements
 *     commutativity requirements
 *     ordering requirements
 *     effect compatibility
 *     aliasing
 *     ownership
 *     determinism
 *     overflow/underflow semantics
 *     numerical stability
 *     resource requirements
 *     capability requirements
 *     dialect/library availability
 *
 * The grammar performs none of these semantic checks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * There is no:
 *
 *     ReductionIR
 *     ParallelReductionIR
 *     QuantumReductionIR
 *     HardwareReductionIR
 *
 * introduced here.
 *
 * Instead:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic reduction
 *       |
 *       v
 *     canonical semantic/IR representation
 *
 * The resulting computation may then lower into:
 *
 *     classical IR
 *     quantum::ir where quantum computation is involved
 *     HDL/hardware representation
 *     distributed representation
 *     accelerator representation
 *
 * according to the actual program semantics.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages may:
 *
 *     validate reduction legality
 *     infer reduction properties
 *     choose a legal reduction strategy
 *     fuse reductions where semantically valid
 *     vectorize reductions
 *     parallelize reductions
 *     distribute reductions
 *     lower reductions to accelerators
 *     synthesize reductions into hardware
 *
 * provided semantic equivalence is preserved.
 *
 * The grammar itself does not make those implementation decisions.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime systems may realize reductions using:
 *
 *     sequential execution
 *     task execution
 *     vector execution
 *     GPU execution
 *     accelerator execution
 *     distributed execution
 *     heterogeneous execution
 *     future execution mechanisms
 *
 * Resource availability may affect realization without changing source
 * semantics.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to the canonical parser/error pipeline.
 *
 * Semantic errors belong to semantic analysis.
 *
 * Resource failures belong to resource/target analysis.
 *
 * Unsupported capabilities belong to capability negotiation.
 *
 * Runtime failures belong to runtime diagnostics.
 *
 * These must not be collapsed into one parser error category.
 *
 * This grammar:
 *
 *     does not print;
 *     does not panic;
 *     does not execute user code;
 *     does not inspect hardware;
 *     does not access files;
 *     does not access networks;
 *     does not allocate resources.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical syntax remains:
 *
 *     parallel::reduce(values, reducer)
 *
 * No new keyword is introduced by this file.
 *
 * Therefore existing lexical vocabulary remains unchanged.
 *
 * This file must not introduce:
 *
 *     REDUCE
 *     REDUCTION
 *     AGGREGATE
 *     FOLD
 *
 * as lexer tokens merely to support reduction.
 *
 * If a future language version introduces a new reduction syntax, that change
 * must proceed through:
 *
 *     specification
 *          |
 *          v
 *     lexical contract
 *          |
 *          v
 *     canonical grammar
 *          |
 *          v
 *     AST
 *          |
 *          v
 *     semantic model
 *          |
 *          v
 *     IR
 *          |
 *          v
 *     conformance tests
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar imports exactly one canonical concrete syntax owner:
 *
 *     DataParallel
 *
 * It does not import:
 *
 *     Tasks
 *     Parallel
 *     TaskParallel
 *
 * because reduction is already structurally represented by the data-parallel
 * reduction syntax and importing unrelated concurrency grammars would enlarge
 * the grammar's dependency surface unnecessarily.
 *
 * ============================================================================
 * PUBLIC COMPOSITION CONTRACT
 * ============================================================================
 *
 * Public entry:
 *
 *     reductionConstruct
 *
 * Expression boundary:
 *
 *     reductionExpression
 *
 * Statement boundary:
 *
 *     reductionStatement
 *
 * Semantic operation:
 *
 *     reductionSemanticOperation
 *
 * The concrete parser production remains:
 *
 *     dataParallelReduce
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * ANTLR imported grammars contribute their parser rules to the composed grammar.
 *
 * Therefore:
 *
 *     reduction.g4
 *         imports DataParallel
 *         |
 *         +--> dataParallelReduce
 *
 * and consumers may use:
 *
 *     reductionConstruct
 *
 * without redefining `dataParallelReduce`.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses no bounded repetition for reduction data.
 *
 * The number of elements in the reduced domain is determined by the program's
 * domain expression and its runtime semantics.
 *
 * The number of reduction operations is likewise unbounded by grammar-level
 * constants.
 *
 * Nested reductions remain possible through ordinary expressions where the
 * surrounding expression grammar permits them.
 *
 * Parser resource exhaustion is an implementation/resource condition, not a
 * language-level semantic limit.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     lexer version
 *     grammar version
 *     dialect configuration
 *
 * the parser must produce the same reduction parse structure.
 *
 * Parsing must not depend on:
 *
 *     hardware
 *     target resources
 *     runtime scheduling
 *     network state
 *     filesystem state
 *     randomness
 *     wall-clock time
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *     no resource maximums
 *     no hardware IDs
 *     no fixed reduction width
 *     no fixed tensor dimensions
 *     no fixed worker counts
 *     no target-specific syntax
 *     no vendor-specific syntax
 *     no physical topology
 *     no duplicate quantum IR
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     parallel::reduce(values, combine)
 *     parallel::reduce(values, combine, identity)
 *     parallel::reduce(values, combine, ordering)
 *     nested expressions containing reduction
 *     reduction over classical data
 *     reduction over tensor data
 *     reduction over distributed data
 *     reduction over quantum-derived data
 *     reduction used by AI/data pipelines
 *
 * NEGATIVE:
 *
 *     parallel::reduce()
 *     parallel::reduce(values)
 *     parallel::reduce(, combine)
 *     parallel::reduce(values, )
 *     parallel::reduce(values combine)
 *     parallel::reduce(values, combine
 *     parallel::reduce(values, combine))
 *
 * Note:
 *
 * Semantic invalidity such as an incompatible reducer type is NOT a parser
 * negative test. It belongs to semantic analysis.
 *
 * BOUNDARY:
 *
 *     deeply nested reduction expressions
 *     very large argument lists
 *     large source units
 *     nested reductions
 *     reductions over symbolic domains
 *     reductions over dynamically sized domains
 *
 * SCALABILITY:
 *
 *     no maximum input-domain size
 *     no maximum reduction depth
 *     no maximum logical parallelism
 *     no maximum worker count
 *     no maximum target size
 *
 * DETERMINISM:
 *
 *     identical source -> identical parse tree
 *
 * CROSS-DOMAIN:
 *
 *     classical + reduction
 *     quantum + reduction
 *     hybrid + reduction
 *     HDL + reduction
 *     AI + reduction
 *     distributed + reduction
 *     networking/data pipeline + reduction
 *
 * COMPATIBILITY:
 *
 *     existing parallel::reduce syntax remains accepted
 *     no new reduction keyword is required
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * [x] Concrete reduction syntax has one canonical owner.
 *
 * [x] This file does not duplicate dataParallelReduce.
 *
 * [x] No new lexer token is required.
 *
 * [x] No machine/resource limits are encoded.
 *
 * [x] No physical execution strategy is encoded.
 *
 * [x] No quantum IR is introduced.
 *
 * [x] AST destination is defined.
 *
 * [x] Semantic responsibilities are defined.
 *
 * [x] IR responsibilities are defined.
 *
 * [x] Compiler/runtime boundaries are defined.
 *
 * [x] Error categories are separated.
 *
 * [x] Cross-domain integration is defined.
 *
 * [x] Scalability requirements are defined.
 *
 * [x] Determinism requirements are defined.
 *
 * [x] Compatibility requirements are defined.
 *
 * [x] Test requirements are defined.
 *
 * [x] Rust implementation requirement remains safe Rust 1.97/1.97.1.
 *
 * ============================================================================
 */

parser grammar Reduction;

options {
    tokenVocab = ZamaniLexer;
}

import DataParallel;


/*
 * ============================================================================
 * PUBLIC REDUCTION CONSTRUCT
 * ============================================================================
 *
 * Concrete syntax owner:
 *
 *     DataParallel.dataParallelReduce
 *
 * This adapter establishes reduction as a first-class semantic domain without
 * creating another syntax.
 */
reductionConstruct
    : dataParallelReduce
    ;


/*
 * ============================================================================
 * EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Reduction produces a logical value.
 *
 * The actual expression syntax is still owned by data-parallel.g4 and the
 * canonical expression grammar.
 */
reductionExpression
    : reductionConstruct
    ;


/*
 * ============================================================================
 * STATEMENT BOUNDARY
 * ============================================================================
 *
 * A reduction may be used as a statement when the surrounding statement
 * grammar permits an expression statement.
 *
 * This rule owns only the reduction-domain adapter.
 *
 * Canonical punctuation is SEMICOLON.
 *
 * IMPORTANT:
 *
 * Do not use the historical/noncanonical `SEMI` token here.
 */
reductionStatement
    : reductionExpression SEMICOLON
    ;


/*
 * ============================================================================
 * REDUCTION OPERATION
 * ============================================================================
 *
 * Semantic adapter to the canonical data-parallel operation.
 */
reductionOperation
    : dataParallelReduceOperation
    ;


/*
 * ============================================================================
 * REDUCTION DOMAIN
 * ============================================================================
 *
 * Semantic adapter for the reduced data domain.
 *
 * The domain remains an ordinary Zamani expression.
 */
reductionDomain
    : expression
    ;


/*
 * ============================================================================
 * REDUCTION OPERATOR
 * ============================================================================
 *
 * Semantic adapter for the reducer expression.
 */
reductionOperator
    : expression
    ;


/*
 * ============================================================================
 * REDUCTION SEMANTIC OPERATION
 * ============================================================================
 *
 * Tooling and downstream grammar composition may use this rule when they need
 * to identify reduction without depending on the concrete data-parallel rule.
 */
reductionSemanticOperation
    : reductionConstruct
    ;


/*
 * ============================================================================
 * REDUCTION ARGUMENT ADAPTER
 * ============================================================================
 *
 * Additional reduction arguments remain owned by data-parallel.g4.
 *
 * This rule deliberately does not recreate the argument syntax.
 */
reductionArguments
    : dataParallelAdditionalArgument*
    ;


/*
 * ============================================================================
 * END OF REDUCTION GRAMMAR
 * ============================================================================
 */