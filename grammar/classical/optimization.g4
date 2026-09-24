/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/optimization.g4
 *
 * Grammar:
 *     ClassicalOptimization
 *
 * Status:
 *     Production classical-domain optimization composition grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the CLASSICAL-DOMAIN COMPOSITION BOUNDARY for
 * optimization intent.
 *
 * It does NOT implement optimization algorithms.
 *
 * It does NOT define a second optimization language.
 *
 * It does NOT define a classical IR.
 *
 * It does NOT define hardware-specific optimization.
 *
 * It composes the canonical optimization-intent grammar from:
 *
 *     grammar/compile/optimization.g4
 *
 * with classical-domain semantic categories.
 *
 * The resulting architecture is:
 *
 *     Zamani source
 *          |
 *          v
 *     lexical analysis
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +---------------------------+
 *          |                           |
 *          v                           v
 *     classical semantics       optimization intent
 *                                      |
 *                                      v
 *                         ClassicalOptimization
 *                                      |
 *                                      v
 *                             canonical IR / IR
 *                                      |
 *                                      v
 *                              optimization planner
 *                                      |
 *              +-----------------------+-----------------------+
 *              |                       |                       |
 *              v                       v                       v
 *         analysis                  passes                 verification
 *              |                       |                       |
 *              +-----------------------+-----------------------+
 *                                      |
 *                                      v
 *                              optimized IR
 *                                      |
 *                 +--------------------+--------------------+
 *                 |                    |                    |
 *                 v                    v                    v
 *              scheduler            routing              lowering
 *                 |                    |                    |
 *                 +--------------------+--------------------+
 *                                      |
 *                                      v
 *                                   runtime
 *
 * For hybrid programs:
 *
 *     classical optimization
 *              |
 *              v
 *       classical semantics
 *              |
 *              +------------------------------+
 *                                             |
 *                                             v
 *                                        quantum::ir
 *                                             |
 *                                             v
 *                                optimization / routing /
 *                                scheduling / resilience
 *                                             |
 *                                             v
 *                                            HAL
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Target implementation:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust edition 2021
 *
 * Generated Rust MUST remain safe Rust.
 *
 * No unsafe Rust is required or permitted by this grammar contract.
 *
 * This grammar contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no target-specific code;
 *     - no filesystem operations;
 *     - no network operations;
 *     - no device discovery;
 *     - no runtime execution.
 *
 * ============================================================================
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - classical optimization composition;
 *     - classical optimization semantic categories;
 *     - classical optimization scope boundaries;
 *     - classical optimization domain annotations;
 *     - classical numerical optimization intent;
 *     - classical algebraic optimization intent;
 *     - classical loop/data-flow optimization intent;
 *     - classical memory/locality optimization intent;
 *     - classical parallelization intent;
 *     - classical vector/data-parallel optimization intent;
 *     - classical symbolic optimization intent;
 *     - classical numerical-stability optimization intent;
 *     - classical approximation policy composition;
 *     - classical optimization interoperability with the canonical
 *       optimization-intent grammar.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - general optimization syntax;
 *     - optimizer implementation;
 *     - optimization algorithms;
 *     - optimization passes;
 *     - canonical IR;
 *     - classical IR implementation;
 *     - expressions;
 *     - types;
 *     - declarations;
 *     - statements;
 *     - resource discovery;
 *     - resource allocation;
 *     - hardware discovery;
 *     - target selection;
 *     - scheduling;
 *     - routing;
 *     - runtime execution;
 *     - SIMD implementation;
 *     - CPU implementation;
 *     - GPU implementation;
 *     - FPGA implementation;
 *     - accelerator implementation;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - calibration.
 *
 * ============================================================================
 * CANONICAL DEPENDENCY
 * ============================================================================
 *
 * General optimization intent is owned by:
 *
 *     grammar/compile/optimization.g4
 *
 * Therefore this grammar imports:
 *
 *     CompileOptimization
 *
 * It MUST NOT copy the rules from that grammar.
 *
 * In particular, this file deliberately does NOT redefine:
 *
 *     optimizationDeclaration
 *     optimizationObjective
 *     optimizationPolicy
 *     optimizationPipelineDeclaration
 *     optimizationPassDeclaration
 *     optimizationBudgetDeclaration
 *     optimizationVerificationDeclaration
 *     optimizationReproducibilityDeclaration
 *     optimizationProperty
 *     optimizationConstraint
 *     optimizationPreference
 *     optimizationHint
 *     optimizationScope
 *     optimizationTargetPolicy
 *     optimizationResourcePolicy
 *     optimizationApproximation
 *     optimizationStochastic
 *     optimizationProvenancePolicy
 *     identifier
 *     qualifiedIdentifier
 *     comparisonOperator
 *
 * The canonical compile optimization grammar remains the authority for
 * those constructs.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Classical optimization must preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Optimization intent may describe WHAT should be improved:
 *
 *     performance
 *     numerical stability
 *     memory locality
 *     communication
 *     parallelism
 *     energy
 *     latency
 *     throughput
 *     precision
 *     code size
 *     computational cost
 *
 * but it MUST NOT permanently encode HOW a particular machine achieves it.
 *
 * Valid semantic intent:
 *
 *     minimize latency
 *
 *     minimize memory traffic
 *
 *     maximize numerical stability
 *
 *     prefer parallel execution
 *
 *     prefer data locality
 *
 *     minimize communication
 *
 * Invalid universal machine assumptions:
 *
 *     use exactly 8 cores
 *
 *     use SIMD width 32
 *
 *     use GPU 0
 *
 *     use CPU 3
 *
 *     use exactly 64 threads
 *
 *     use register 17
 *
 *     use cache level 2
 *
 * unless such information is explicitly represented downstream as a
 * target-specific implementation decision rather than universal source
 * semantics.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT define:
 *
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ACCELERATORS
 *     MAX_VECTOR_WIDTH
 *     MAX_SIMD_WIDTH
 *     MAX_REGISTER_COUNT
 *     MAX_REGISTER_WIDTH
 *     MAX_CACHE_SIZE
 *     MAX_MEMORY
 *     MAX_MATRIX_ROWS
 *     MAX_MATRIX_COLUMNS
 *     MAX_TENSOR_RANK
 *     MAX_OPERATIONS
 *     MAX_PASSES
 *     MAX_PIPELINE_STAGES
 *
 * There are no fixed grammar-level limits on:
 *
 *     objectives
 *     passes
 *     pipeline stages
 *     expressions
 *     operands
 *     dimensions
 *     tensor rank
 *     vectors
 *     matrices
 *     collections
 *     optimization regions
 *     program size
 *     machine size.
 *
 * Repetition is represented structurally with ANTLR repetition operators.
 *
 * Actual implementation/resource limits belong to:
 *
 *     compiler policy
 *     resource management
 *     execution environment
 *     runtime
 *     deployment environment
 *
 * ============================================================================
 * SEMANTIC SEPARATION
 * ============================================================================
 *
 * This grammar preserves the following distinctions:
 *
 *     objective
 *         what should improve
 *
 *     requirement
 *         what must hold
 *
 *     constraint
 *         what optimization choices are forbidden
 *
 *     preference
 *         what is desirable but not mandatory
 *
 *     hint
 *         advisory information
 *
 *     capability
 *         what an execution environment can provide
 *
 *     resource
 *         an abstract computational resource
 *
 *     target
 *         an abstract compilation target
 *
 *     device
 *         a concrete realization selected downstream
 *
 * The grammar MUST NOT collapse these concepts.
 *
 * ============================================================================
 * CLASSICAL DOMAIN MODEL
 * ============================================================================
 *
 * Classical optimization may apply to:
 *
 *     scalar computation
 *     integer computation
 *     floating-point computation
 *     decimal computation
 *     rational computation
 *     complex computation
 *     vectors
 *     matrices
 *     tensors
 *     collections
 *     loops
 *     recursive computation
 *     dataflow
 *     symbolic computation
 *     numerical computation
 *     scientific computation
 *     signal processing
 *     control computation
 *     distributed classical computation
 *     accelerator-oriented classical computation
 *     hybrid classical/quantum computation
 *     software/hardware co-design.
 *
 * These are semantic categories.
 *
 * The grammar does NOT create separate grammars for each mathematical
 * algorithm.
 *
 * Operations such as:
 *
 *     fft
 *     svd
 *     matrix_multiply
 *     sort
 *     search
 *     gradient_descent
 *     factorization
 *
 * should normally remain operations, intrinsics, or library facilities.
 *
 * Dedicated grammar syntax is reserved for concepts with language-level
 * semantics.
 *
 * ============================================================================
 * GENERAL OPTIMIZATION REUSE
 * ============================================================================
 *
 * Classical optimization inherits the canonical optimization-intent surface.
 *
 * Therefore a classical optimization declaration may compose:
 *
 *     optimizationDeclaration
 *     optimizationObjectiveDeclaration
 *     optimizationPolicy
 *     optimizationPipelineDeclaration
 *     optimizationPassDeclaration
 *     optimizationBudgetDeclaration
 *     optimizationVerificationDeclaration
 *     optimizationReproducibilityDeclaration
 *     optimizationRequirement
 *     optimizationConstraint
 *     optimizationPreference
 *     optimizationHint
 *     optimizationScope
 *     optimizationResourcePolicy
 *     optimizationTargetPolicy
 *     optimizationApproximation
 *     optimizationStochastic
 *     optimizationProvenancePolicy
 *
 * without redefining them.
 *
 * ============================================================================
 */

parser grammar ClassicalOptimization;

import CompileOptimization;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC CLASSICAL OPTIMIZATION ENTRY POINT
 * ============================================================================
 *
 * This is the primary integration rule exposed by the classical domain.
 *
 * It provides an explicit classical semantic boundary while reusing the
 * canonical optimization grammar.
 *
 * The rule accepts:
 *
 *     - general optimization intent;
 *     - classical optimization categories;
 *     - classical optimization blocks;
 *     - classical optimization scopes.
 *
 * It does not create another optimizer language.
 *
 * ============================================================================
 */

classicalOptimization
    : classicalOptimizationItem+
    ;


/*
 * ============================================================================
 * 2. CLASSICAL OPTIMIZATION ITEM
 * ============================================================================
 *
 * Every item is either:
 *
 *     canonical optimization intent
 *
 * or:
 *
 *     classical-domain optimization intent.
 *
 * ============================================================================
 */

classicalOptimizationItem
    : optimizationDeclaration
    | optimizationObjectiveDeclaration
    | optimizationPolicy
    | optimizationPipelineDeclaration
    | optimizationPassDeclaration
    | optimizationBudgetDeclaration
    | optimizationVerificationDeclaration
    | optimizationReproducibilityDeclaration
    | optimizationRequirement
    | optimizationConstraint
    | optimizationPreference
    | optimizationHint
    | optimizationScope
    | optimizationResourcePolicy
    | optimizationTargetPolicy
    | optimizationApproximation
    | optimizationStochastic
    | optimizationProvenancePolicy
    | classicalOptimizationDirective
    | classicalOptimizationRegion
    | classicalOptimizationDomain
    | classicalOptimizationObjective
    ;


/*
 * ============================================================================
 * 3. CLASSICAL OPTIMIZATION DIRECTIVE
 * ============================================================================
 *
 * Generic classical optimization extension point.
 *
 * The directive name is symbolic.
 *
 * This intentionally does not enumerate:
 *
 *     compiler versions;
 *     CPU vendors;
 *     GPU vendors;
 *     instruction sets;
 *     accelerator models;
 *     machine identifiers.
 *
 * New classical optimization facilities can therefore be introduced without
 * turning every library operation into a language keyword.
 *
 * ============================================================================
 */

classicalOptimizationDirective
    : optimizationKeyword
      classicalOptimizationDirectiveBody?
    ;


classicalOptimizationDirectiveBody
    : LPAREN optimizationArgumentList? RPAREN
    | LBRACE classicalOptimizationEntry* RBRACE
    | ASSIGN expression
    ;


classicalOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationPolicy
    | optimizationConstraint
    | optimizationPreference
    | optimizationHint
    | optimizationRequirement
    | optimizationPassDeclaration
    | optimizationPipelineDeclaration
    | classicalOptimizationDirective
    ;


/*
 * ============================================================================
 * 4. CLASSICAL OPTIMIZATION REGION
 * ============================================================================
 *
 * Associates optimization intent with a semantic program region.
 *
 * The region is identified symbolically.
 *
 * It may eventually correspond to:
 *
 *     function
 *     module
 *     loop
 *     expression
 *     dataflow region
 *     numerical kernel
 *     signal-processing region
 *     classical part of a hybrid computation
 *     implementation-defined semantic region.
 *
 * The actual region model belongs to the AST/semantic layer.
 *
 * ============================================================================
 */

classicalOptimizationRegion
    : optimizationScope
    | classicalOptimizationRegionBlock
    ;


classicalOptimizationRegionBlock
    : optimizationKeyword
      classicalOptimizationRegionReference?
      LBRACE classicalOptimizationItem* RBRACE
    ;


classicalOptimizationRegionReference
    : identifier
    | qualifiedIdentifier
    | expression
    ;


/*
 * ============================================================================
 * 5. CLASSICAL OPTIMIZATION DOMAIN
 * ============================================================================
 *
 * Allows a source program to identify a semantic classical optimization
 * domain without hard-coding implementation technology.
 *
 * Examples of semantic domain values include:
 *
 *     numerical
 *     symbolic
 *     linear_algebra
 *     signal_processing
 *     scientific
 *     control
 *     dataflow
 *     parallel
 *     distributed
 *     accelerator
 *
 * These names remain semantic identifiers.
 *
 * They are not parser-level hardware enumerations.
 *
 * ============================================================================
 */

classicalOptimizationDomain
    : optimizationKeyword
      classicalOptimizationDomainReference
      SEMICOLON?
    ;


classicalOptimizationDomainReference
    : identifier
    | qualifiedIdentifier
    | STRING
    ;


/*
 * ============================================================================
 * 6. CLASSICAL OPTIMIZATION OBJECTIVE
 * ============================================================================
 *
 * This rule provides a classical-domain semantic hook around the canonical
 * objective model.
 *
 * No finite objective enumeration is imposed.
 *
 * Therefore future objectives do not require grammar changes.
 *
 * ============================================================================
 */

classicalOptimizationObjective
    : classicalOptimizationObjectiveDirection
      classicalOptimizationObjectiveMetric
      classicalOptimizationObjectiveModifier*
      SEMICOLON?
    ;


classicalOptimizationObjectiveDirection
    : identifier
    ;


classicalOptimizationObjectiveMetric
    : expression
    ;


classicalOptimizationObjectiveModifier
    : optimizationWeight
    | optimizationPriority
    | optimizationTolerance
    | optimizationConstraintModifier
    | optimizationPropertyModifier
    ;


/*
 * ============================================================================
 * 7. NUMERICAL OPTIMIZATION
 * ============================================================================
 *
 * Numerical optimization is represented as semantic intent.
 *
 * This rule does NOT enumerate numerical algorithms.
 *
 * Examples of semantic concepts:
 *
 *     stability
 *     precision
 *     conditioning
 *     error
 *     convergence
 *     accuracy
 *     reproducibility
 *     range
 *     overflow behavior
 *     underflow behavior
 *
 * The actual algorithm remains outside the grammar.
 *
 * ============================================================================
 */

classicalNumericalOptimization
    : optimizationKeyword
      classicalNumericalOptimizationBody
    ;


classicalNumericalOptimizationBody
    : expression
    | LBRACE classicalNumericalOptimizationEntry* RBRACE
    ;


classicalNumericalOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    ;


/*
 * ============================================================================
 * 8. NUMERICAL STABILITY
 * ============================================================================
 *
 * Numerical stability is a semantic property.
 *
 * It may be expressed as an optimization objective, requirement, constraint,
 * preference, or policy.
 *
 * No machine floating-point format is hard-coded here.
 *
 * ============================================================================
 */

classicalNumericalStability
    : optimizationKeyword
      optimizationArgumentList?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 9. PRECISION POLICY
 * ============================================================================
 *
 * Precision is symbolic/expression-valued.
 *
 * This permits:
 *
 *     exact
 *     bounded
 *     approximate
 *     symbolic
 *     target-derived
 *     capability-derived
 *
 * semantics without making a particular bit width part of the grammar.
 *
 * ============================================================================
 */

classicalPrecisionPolicy
    : optimizationKeyword
      optimizationPrecisionValue
      SEMICOLON?
    ;


optimizationPrecisionValue
    : expression
    | identifier
    | qualifiedIdentifier
    | STRING
    ;


/*
 * ============================================================================
 * 10. LOOP OPTIMIZATION
 * ============================================================================
 *
 * Loop transformations are semantic/compiler transformations.
 *
 * This rule expresses intent only.
 *
 * It does not define:
 *
 *     unrolling factors;
 *     processor counts;
 *     thread counts;
 *     instruction widths.
 *
 * ============================================================================
 */

classicalLoopOptimization
    : optimizationKeyword
      classicalLoopOptimizationBody
    ;


classicalLoopOptimizationBody
    : expression
    | LBRACE classicalLoopOptimizationEntry* RBRACE
    ;


classicalLoopOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    | optimizationPipelineDeclaration
    ;


/*
 * ============================================================================
 * 11. DATAFLOW OPTIMIZATION
 * ============================================================================
 *
 * Dataflow optimization may cover:
 *
 *     dependency analysis;
 *     producer/consumer relationships;
 *     fusion;
 *     elimination;
 *     propagation;
 *     scheduling hints.
 *
 * Actual dataflow semantics remain owned by the semantic/IR layers.
 *
 * ============================================================================
 */

classicalDataflowOptimization
    : optimizationKeyword
      classicalDataflowOptimizationBody
    ;


classicalDataflowOptimizationBody
    : expression
    | LBRACE classicalDataflowOptimizationEntry* RBRACE
    ;


classicalDataflowOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    | optimizationPipelineDeclaration
    ;


/*
 * ============================================================================
 * 12. MEMORY LOCALITY OPTIMIZATION
 * ============================================================================
 *
 * Memory locality is an abstract optimization property.
 *
 * The grammar MUST NOT encode:
 *
 *     cache level;
 *     cache size;
 *     NUMA node number;
 *     memory-bank identifier;
 *     physical address.
 *
 * Actual placement belongs downstream.
 *
 * ============================================================================
 */

classicalMemoryLocalityOptimization
    : optimizationKeyword
      classicalMemoryLocalityBody
    ;


classicalMemoryLocalityBody
    : expression
    | LBRACE classicalMemoryLocalityEntry* RBRACE
    ;


classicalMemoryLocalityEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    ;


/*
 * ============================================================================
 * 13. PARALLEL OPTIMIZATION
 * ============================================================================
 *
 * Parallelism is expressed as semantic intent.
 *
 * It does NOT mean:
 *
 *     exactly N threads;
 *     exactly N cores;
 *     exactly N workers.
 *
 * The available execution environment determines the realizable degree of
 * parallelism.
 *
 * ============================================================================
 */

classicalParallelOptimization
    : optimizationKeyword
      classicalParallelOptimizationBody
    ;


classicalParallelOptimizationBody
    : expression
    | LBRACE classicalParallelOptimizationEntry* RBRACE
    ;


classicalParallelOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    | optimizationPipelineDeclaration
    ;


/*
 * ============================================================================
 * 14. DATA-PARALLEL OPTIMIZATION
 * ============================================================================
 *
 * Data parallelism is semantic.
 *
 * Vector length, SIMD width, lane count and accelerator width are target
 * realization properties.
 *
 * ============================================================================
 */

classicalDataParallelOptimization
    : optimizationKeyword
      classicalDataParallelOptimizationBody
    ;


classicalDataParallelOptimizationBody
    : expression
    | LBRACE classicalDataParallelOptimizationEntry* RBRACE
    ;


classicalDataParallelOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    ;


/*
 * ============================================================================
 * 15. VECTOR / MATRIX / TENSOR OPTIMIZATION
 * ============================================================================
 *
 * Vector, matrix and tensor dimensions remain program/type semantics.
 *
 * This grammar does not impose maximum dimensions.
 *
 * The optimizer may specialize these computations based on:
 *
 *     shape;
 *     sparsity;
 *     layout;
 *     precision;
 *     available capabilities;
 *     available resources.
 *
 * ============================================================================
 */

classicalLinearAlgebraOptimization
    : optimizationKeyword
      classicalLinearAlgebraOptimizationBody
    ;


classicalLinearAlgebraOptimizationBody
    : expression
    | LBRACE classicalLinearAlgebraOptimizationEntry* RBRACE
    ;


classicalLinearAlgebraOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    | optimizationPipelineDeclaration
    ;


/*
 * ============================================================================
 * 16. SYMBOLIC OPTIMIZATION
 * ============================================================================
 *
 * Symbolic transformations preserve symbolic semantics.
 *
 * The grammar does not enumerate algebra systems or symbolic engines.
 *
 * ============================================================================
 */

classicalSymbolicOptimization
    : optimizationKeyword
      classicalSymbolicOptimizationBody
    ;


classicalSymbolicOptimizationBody
    : expression
    | LBRACE classicalSymbolicOptimizationEntry* RBRACE
    ;


classicalSymbolicOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    ;


/*
 * ============================================================================
 * 17. ALGEBRAIC OPTIMIZATION
 * ============================================================================
 *
 * Algebraic simplification is semantic-preserving unless explicitly declared
 * otherwise.
 *
 * The grammar does not define a finite algebraic rewrite set.
 *
 * ============================================================================
 */

classicalAlgebraicOptimization
    : optimizationKeyword
      classicalAlgebraicOptimizationBody
    ;


classicalAlgebraicOptimizationBody
    : expression
    | LBRACE classicalAlgebraicOptimizationEntry* RBRACE
    ;


classicalAlgebraicOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    ;


/*
 * ============================================================================
 * 18. SIGNAL-PROCESSING OPTIMIZATION
 * ============================================================================
 *
 * Signal-processing algorithms remain library/intrinsic/semantic operations.
 *
 * This grammar only provides optimization intent around them.
 *
 * Examples of semantic properties include:
 *
 *     latency;
 *     throughput;
 *     numerical error;
 *     energy;
 *     communication;
 *     memory traffic.
 *
 * ============================================================================
 */

classicalSignalProcessingOptimization
    : optimizationKeyword
      classicalSignalProcessingOptimizationBody
    ;


classicalSignalProcessingOptimizationBody
    : expression
    | LBRACE classicalSignalProcessingOptimizationEntry* RBRACE
    ;


classicalSignalProcessingOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    ;


/*
 * ============================================================================
 * 19. SCIENTIFIC COMPUTING OPTIMIZATION
 * ============================================================================
 *
 * Scientific computation may have domain-specific numerical constraints.
 *
 * Those constraints remain expressions and semantic properties.
 *
 * No particular numerical library is embedded into the grammar.
 *
 * ============================================================================
 */

classicalScientificOptimization
    : optimizationKeyword
      classicalScientificOptimizationBody
    ;


classicalScientificOptimizationBody
    : expression
    | LBRACE classicalScientificOptimizationEntry* RBRACE
    ;


classicalScientificOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    ;


/*
 * ============================================================================
 * 20. CONTROL / REAL-TIME OPTIMIZATION
 * ============================================================================
 *
 * Classical control computation may have:
 *
 *     latency;
 *     deadline;
 *     jitter;
 *     throughput;
 *     stability;
 *     determinism;
 *     responsiveness.
 *
 * These remain semantic properties.
 *
 * The runtime/scheduler remains responsible for actual realization.
 *
 * ============================================================================
 */

classicalControlOptimization
    : optimizationKeyword
      classicalControlOptimizationBody
    ;


classicalControlOptimizationBody
    : expression
    | LBRACE classicalControlOptimizationEntry* RBRACE
    ;


classicalControlOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    ;


/*
 * ============================================================================
 * 21. DISTRIBUTED CLASSICAL OPTIMIZATION
 * ============================================================================
 *
 * Distributed optimization remains independent of a fixed number of nodes.
 *
 * The grammar MUST NOT encode:
 *
 *     node count;
 *     node identifiers;
 *     network topology;
 *     fixed worker count;
 *     physical network addresses.
 *
 * Those belong downstream to resource discovery, scheduling and deployment.
 *
 * ============================================================================
 */

classicalDistributedOptimization
    : optimizationKeyword
      classicalDistributedOptimizationBody
    ;


classicalDistributedOptimizationBody
    : expression
    | LBRACE classicalDistributedOptimizationEntry* RBRACE
    ;


classicalDistributedOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    | optimizationPipelineDeclaration
    ;


/*
 * ============================================================================
 * 22. ACCELERATOR-AWARE CLASSICAL OPTIMIZATION
 * ============================================================================
 *
 * This expresses semantic preference for capabilities, not physical devices.
 *
 * Examples:
 *
 *     vector capability;
 *     matrix capability;
 *     tensor capability;
 *     parallel capability;
 *     accelerator capability.
 *
 * The actual CPU/GPU/FPGA/ASIC/other realization is downstream.
 *
 * ============================================================================
 */

classicalAcceleratorOptimization
    : optimizationKeyword
      classicalAcceleratorOptimizationBody
    ;


classicalAcceleratorOptimizationBody
    : expression
    | LBRACE classicalAcceleratorOptimizationEntry* RBRACE
    ;


classicalAcceleratorOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    | optimizationResourcePolicy
    ;


/*
 * ============================================================================
 * 23. ENERGY-AWARE OPTIMIZATION
 * ============================================================================
 *
 * Energy is a semantic cost metric.
 *
 * The grammar does not assume a particular power-management system.
 *
 * ============================================================================
 */

classicalEnergyOptimization
    : optimizationKeyword
      classicalEnergyOptimizationBody
    ;


classicalEnergyOptimizationBody
    : expression
    | LBRACE classicalEnergyOptimizationEntry* RBRACE
    ;


classicalEnergyOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    ;


/*
 * ============================================================================
 * 24. COMMUNICATION-AWARE OPTIMIZATION
 * ============================================================================
 *
 * Communication cost may matter for:
 *
 *     distributed computation;
 *     NUMA computation;
 *     accelerator computation;
 *     heterogeneous computation;
 *     hybrid computation.
 *
 * The grammar does not encode a particular network or interconnect.
 *
 * ============================================================================
 */

classicalCommunicationOptimization
    : optimizationKeyword
      classicalCommunicationOptimizationBody
    ;


classicalCommunicationOptimizationBody
    : expression
    | LBRACE classicalCommunicationOptimizationEntry* RBRACE
    ;


classicalCommunicationOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    ;


/*
 * ============================================================================
 * 25. CODE-SIZE OPTIMIZATION
 * ============================================================================
 *
 * Code size is semantic compiler intent.
 *
 * It does not imply a particular instruction encoding or architecture.
 *
 * ============================================================================
 */

classicalCodeSizeOptimization
    : optimizationKeyword
      classicalCodeSizeOptimizationBody
    ;


classicalCodeSizeOptimizationBody
    : expression
    | LBRACE classicalCodeSizeOptimizationEntry* RBRACE
    ;


classicalCodeSizeOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    ;


/*
 * ============================================================================
 * 26. THROUGHPUT OPTIMIZATION
 * ============================================================================
 *
 * Throughput is a semantic performance property.
 *
 * Actual execution parallelism and scheduling remain downstream.
 *
 * ============================================================================
 */

classicalThroughputOptimization
    : optimizationKeyword
      classicalThroughputOptimizationBody
    ;


classicalThroughputOptimizationBody
    : expression
    | LBRACE classicalThroughputOptimizationEntry* RBRACE
    ;


classicalThroughputOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    ;


/*
 * ============================================================================
 * 27. LATENCY OPTIMIZATION
 * ============================================================================
 *
 * Latency is an abstract cost metric.
 *
 * No clock frequency or processor-specific timing assumption belongs here.
 *
 * ============================================================================
 */

classicalLatencyOptimization
    : optimizationKeyword
      classicalLatencyOptimizationBody
    ;


classicalLatencyOptimizationBody
    : expression
    | LBRACE classicalLatencyOptimizationEntry* RBRACE
    ;


classicalLatencyOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    ;


/*
 * ============================================================================
 * 28. DETERMINISM-AWARE OPTIMIZATION
 * ============================================================================
 *
 * Determinism is a semantic/compiler property.
 *
 * The grammar does not require a particular implementation strategy.
 *
 * This is especially important for:
 *
 *     numerical computation;
 *     distributed computation;
 *     parallel computation;
 *     reproducible builds;
 *     scientific workloads.
 *
 * ============================================================================
 */

classicalDeterminismOptimization
    : optimizationKeyword
      classicalDeterminismOptimizationBody
    ;


classicalDeterminismOptimizationBody
    : expression
    | LBRACE classicalDeterminismOptimizationEntry* RBRACE
    ;


classicalDeterminismOptimizationEntry
    : optimizationProperty
    | optimizationObjectiveDeclaration
    | optimizationConstraint
    | optimizationPreference
    | optimizationRequirement
    | optimizationHint
    | optimizationReproducibilityDeclaration
    ;


/*
 * ============================================================================
 * 29. SEMANTIC-PRESERVATION BOUNDARY
 * ============================================================================
 *
 * Classical optimization is semantic-preserving by default.
 *
 * If a transformation changes observable semantics, the semantic layer must
 * reject it unless the language contract explicitly permits the change.
 *
 * The canonical optimization grammar owns the general preservation policy.
 *
 * This rule provides a classical-domain entry point.
 *
 * ============================================================================
 */

classicalSemanticPreservation
    : optimizationSemanticPreservation
    ;


/*
 * ============================================================================
 * 30. APPROXIMATE CLASSICAL OPTIMIZATION
 * ============================================================================
 *
 * Approximation MUST be explicit.
 *
 * The optimizer may not silently replace exact semantics with an approximation.
 *
 * ============================================================================
 */

classicalApproximateOptimization
    : optimizationApproximation
    ;


/*
 * ============================================================================
 * 31. STOCHASTIC CLASSICAL OPTIMIZATION
 * ============================================================================
 *
 * Stochastic behavior is explicitly represented through the canonical
 * optimization grammar.
 *
 * Reproducibility may be requested through the canonical reproducibility
 * constructs.
 *
 * ============================================================================
 */

classicalStochasticOptimization
    : optimizationStochastic
    ;


/*
 * ============================================================================
 * 32. CLASSICAL RESOURCE-AWARE OPTIMIZATION
 * ============================================================================
 *
 * Resource-aware optimization uses abstract resource intent.
 *
 * It may consider:
 *
 *     compute;
 *     memory;
 *     bandwidth;
 *     storage;
 *     communication;
 *     energy;
 *     latency;
 *     parallelism;
 *     reliability;
 *     precision.
 *
 * It must not encode fixed physical capacity.
 *
 * ============================================================================
 */

classicalResourceAwareOptimization
    : optimizationResourcePolicy
    ;


/*
 * ============================================================================
 * 33. CLASSICAL TARGET-AWARE OPTIMIZATION
 * ============================================================================
 *
 * Target-aware optimization may use abstract target/capability information.
 *
 * It MUST NOT directly select a physical device.
 *
 * Target resolution belongs to compile/target and downstream compiler
 * infrastructure.
 *
 * ============================================================================
 */

classicalTargetAwareOptimization
    : optimizationTargetPolicy
    ;


/*
 * ============================================================================
 * 34. CLASSICAL VERIFICATION
 * ============================================================================
 *
 * Optimization verification may include:
 *
 *     semantic equivalence;
 *     numerical equivalence;
 *     shape preservation;
 *     type preservation;
 *     effect preservation;
 *     resource-contract preservation;
 *     determinism;
 *     approximation bounds.
 *
 * The implementation belongs to the optimizer/verification subsystem.
 *
 * ============================================================================
 */

classicalOptimizationVerification
    : optimizationVerificationDeclaration
    ;


/*
 * ============================================================================
 * 35. CLASSICAL REPRODUCIBILITY
 * ============================================================================
 *
 * Reproducibility is important for POCO-REAF.
 *
 * The grammar delegates the actual reproducibility policy to the canonical
 * optimization grammar.
 *
 * ============================================================================
 */

classicalOptimizationReproducibility
    : optimizationReproducibilityDeclaration
    ;


/*
 * ============================================================================
 * 36. CLASSICAL PROVENANCE
 * ============================================================================
 *
 * Optimization provenance allows the compiler/tooling stack to record:
 *
 *     selected policy;
 *     selected passes;
 *     semantic assumptions;
 *     target capabilities;
 *     resource realization;
 *     verification results;
 *     transformation history.
 *
 * This is metadata, not optimizer implementation.
 *
 * ============================================================================
 */

classicalOptimizationProvenance
    : optimizationProvenancePolicy
    ;


/*
 * ============================================================================
 * 37. CLASSICAL OPTIMIZATION COMPOSITION
 * ============================================================================
 *
 * This is the recommended high-level integration point for semantic analysis.
 *
 * An arbitrary number of optimization constructs may be composed.
 *
 * No fixed number of passes, objectives or regions is encoded.
 *
 * ============================================================================
 */

classicalOptimizationComposition
    : LBRACE
      classicalOptimizationItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 38. CLASSICAL OPTIMIZATION LIST
 * ============================================================================
 *
 * Explicit list form for tooling and AST builders that need a standalone
 * optimization sequence.
 *
 * ============================================================================
 */

classicalOptimizationList
    : classicalOptimizationItem
      (COMMA classicalOptimizationItem)*
    ;


/*
 * ============================================================================
 * 39. CLASSICAL OPTIMIZATION PROPERTY
 * ============================================================================
 *
 * Reuses the canonical optimization property model.
 *
 * No duplicate property syntax is introduced.
 *
 * ============================================================================
 */

classicalOptimizationProperty
    : optimizationProperty
    ;


/*
 * ============================================================================
 * 40. CLASSICAL OPTIMIZATION CONSTRAINT
 * ============================================================================
 *
 * Reuses the canonical optimization constraint model.
 *
 * ============================================================================
 */

classicalOptimizationConstraint
    : optimizationConstraint
    ;


/*
 * ============================================================================
 * 41. CLASSICAL OPTIMIZATION PREFERENCE
 * ============================================================================
 *
 * Reuses the canonical optimization preference model.
 *
 * ============================================================================
 */

classicalOptimizationPreference
    : optimizationPreference
    ;


/*
 * ============================================================================
 * 42. CLASSICAL OPTIMIZATION HINT
 * ============================================================================
 *
 * Reuses the canonical optimization hint model.
 *
 * ============================================================================
 */

classicalOptimizationHint
    : optimizationHint
    ;


/*
 * ============================================================================
 * 43. CLASSICAL OPTIMIZATION REQUIREMENT
 * ============================================================================
 *
 * Reuses the canonical optimization requirement model.
 *
 * ============================================================================
 */

classicalOptimizationRequirement
    : optimizationRequirement
    ;


/*
 * ============================================================================
 * 44. CLASSICAL OPTIMIZATION PIPELINE
 * ============================================================================
 *
 * Reuses the canonical optimization pipeline.
 *
 * Pass implementations are resolved downstream through the optimizer
 * registry/planner.
 *
 * ============================================================================
 */

classicalOptimizationPipeline
    : optimizationPipelineDeclaration
    ;


/*
 * ============================================================================
 * 45. CLASSICAL OPTIMIZATION PASS
 * ============================================================================
 *
 * Reuses the canonical pass-selection contract.
 *
 * ============================================================================
 */

classicalOptimizationPass
    : optimizationPassDeclaration
    ;


/*
 * ============================================================================
 * 46. CLASSICAL OPTIMIZATION BUDGET
 * ============================================================================
 *
 * A budget is an optimizer/compiler resource policy.
 *
 * It is NOT a machine capacity limit.
 *
 * ============================================================================
 */

classicalOptimizationBudget
    : optimizationBudgetDeclaration
    ;


/*
 * ============================================================================
 * 47. CLASSICAL OPTIMIZATION SCOPE
 * ============================================================================
 *
 * Scope is delegated to the canonical optimization scope model.
 *
 * ============================================================================
 */

classicalOptimizationScope
    : optimizationScope
    ;


/*
 * ============================================================================
 * 48. CLASSICAL OPTIMIZATION RESOURCE POLICY
 * ============================================================================
 *
 * Resource policy remains abstract and portable.
 *
 * ============================================================================
 */

classicalOptimizationResourcePolicy
    : optimizationResourcePolicy
    ;


/*
 * ============================================================================
 * 49. CLASSICAL OPTIMIZATION TARGET POLICY
 * ============================================================================
 *
 * Target policy remains abstract.
 *
 * ============================================================================
 */

classicalOptimizationTargetPolicy
    : optimizationTargetPolicy
    ;


/*
 * ============================================================================
 * 50. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] classical optimization has an explicit grammar boundary;
 * [x] canonical compile optimization is reused;
 * [x] no second optimization language is created;
 * [x] no optimization IR is created;
 * [x] no hardware limits are encoded;
 * [x] no CPU/core/thread counts are encoded;
 * [x] no GPU/FPGA/device identifiers are encoded;
 * [x] no SIMD/vector width is encoded;
 * [x] no cache/memory size is encoded;
 * [x] numerical optimization is represented semantically;
 * [x] loop optimization is represented semantically;
 * [x] dataflow optimization is represented semantically;
 * [x] locality optimization is represented semantically;
 * [x] parallel optimization is represented semantically;
 * [x] linear algebra optimization is represented semantically;
 * [x] symbolic optimization is represented semantically;
 * [x] signal-processing optimization is represented semantically;
 * [x] scientific optimization is represented semantically;
 * [x] distributed optimization is represented semantically;
 * [x] accelerator optimization is capability-based;
 * [x] approximation is explicit;
 * [x] stochastic optimization is explicit;
 * [x] verification is explicit;
 * [x] reproducibility is explicit;
 * [x] provenance is explicit;
 * [x] classical optimization remains open-ended;
 * [x] Rust implementation remains compatible with Rust 1.97/1.97.1;
 * [x] no unsafe Rust is required.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM:
 *
 *     grammar/lexer/
 *         supplies ZamaniLexer tokens.
 *
 *     grammar/core/
 *         supplies canonical names and identifiers.
 *
 *     grammar/expressions/
 *         supplies expression syntax.
 *
 *     grammar/types/
 *         supplies type syntax.
 *
 *     grammar/resources/
 *         supplies resource/capability semantics.
 *
 *     grammar/compile/optimization.g4
 *         supplies canonical optimization-intent syntax.
 *
 *     grammar/spec/classical.md
 *         supplies the normative classical semantic contract.
 *
 * DOWNSTREAM:
 *
 *     frontend AST
 *         receives generic optimization intent plus classical-domain
 *         classification.
 *
 *     semantic analysis
 *         resolves the meaning of optimization objectives, requirements,
 *         constraints, preferences and capabilities.
 *
 *     canonical IR
 *         receives machine-independent semantic computation.
 *
 *     classical optimization subsystem
 *         resolves optimization policy and pass selection.
 *
 *     optimizer
 *         implements actual transformations.
 *
 *     scheduler
 *         resolves execution ordering and available parallelism.
 *
 *     resource manager
 *         resolves actual resource availability.
 *
 *     compiler
 *         resolves target realization.
 *
 *     runtime
 *         executes the resulting program.
 *
 *     hardware/HAL
 *         provides actual execution capabilities.
 *
 * HYBRID INTEGRATION:
 *
 *     classical optimization
 *             |
 *             v
 *     classical semantic analysis
 *             |
 *             +-----------------------+
 *                                     |
 *                                     v
 *                                quantum::ir
 *                                     |
 *                                     v
 *                           optimization / routing /
 *                           scheduling / resilience
 *                                     |
 *                                     v
 *                                    ZQN
 *                                     |
 *                                     v
 *                                    HAL
 *
 * The classical optimization grammar MUST NOT create or modify a competing
 * quantum IR.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every successful parse of a classical optimization construct MUST map to
 * the repository's generic/domain-neutral AST representation.
 *
 * The AST should preserve, where applicable:
 *
 *     source span
 *     optimization kind
 *     objective
 *     operands
 *     parameters
 *     properties
 *     modifiers
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     resource intent
 *     capability intent
 *     target intent
 *     verification policy
 *     reproducibility policy
 *     provenance intent
 *     semantic domain.
 *
 * This grammar MUST NOT require a `ClassicalOptimizationIR` AST type.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST:
 *
 *     - resolve symbolic optimization names;
 *     - distinguish objective/requirement/constraint/preference/hint;
 *     - verify type compatibility;
 *     - verify resource/capability references;
 *     - determine classical applicability;
 *     - validate semantic preservation;
 *     - validate approximation policy;
 *     - validate stochastic/reproducibility requirements;
 *     - reject impossible mandatory requirements;
 *     - preserve source provenance;
 *     - preserve target independence.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file does not define an IR.
 *
 * Optimization intent is metadata/policy attached to the canonical semantic
 * compilation pipeline.
 *
 * Classical computation must lower through the repository's canonical
 * machine-independent semantic representation.
 *
 * No separate:
 *
 *     ClassicalOptimizationIR
 *     ClassicalOptimizerIR
 *     HardwareOptimizationIR
 *
 * is introduced by this grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required tests under:
 *
 *     grammar/tests/classical/
 *
 * At minimum:
 *
 *     optimization/
 *         positive/
 *         negative/
 *         boundary/
 *         scalability/
 *         determinism/
 *         compatibility/
 *
 * POSITIVE TESTS:
 *
 *     classical optimization objective;
 *     multiple objectives;
 *     objective priority;
 *     optimization profile;
 *     optimization pipeline;
 *     pass selection;
 *     pass configuration;
 *     numerical optimization;
 *     loop optimization;
 *     dataflow optimization;
 *     memory locality;
 *     parallel optimization;
 *     vector/data-parallel optimization;
 *     linear algebra optimization;
 *     symbolic optimization;
 *     signal processing;
 *     scientific computing;
 *     distributed optimization;
 *     accelerator-aware optimization;
 *     resource-aware optimization;
 *     reproducibility;
 *     provenance;
 *     explicit approximation;
 *     stochastic optimization.
 *
 * NEGATIVE TESTS:
 *
 *     invalid optimization structure;
 *     malformed objective;
 *     malformed pass configuration;
 *     invalid resource expression;
 *     invalid capability reference;
 *     invalid semantic-preservation policy;
 *     invalid approximation contract;
 *     invalid deterministic requirement;
 *     target/device confusion where semantic validation catches it.
 *
 * BOUNDARY TESTS:
 *
 *     zero optimization entries where legal;
 *     one objective;
 *     many objectives;
 *     nested optimization regions;
 *     nested pipelines;
 *     large symbolic expressions;
 *     symbolic dimensions;
 *     symbolic resource requirements;
 *     runtime-derived values;
 *     deeply nested optimization composition.
 *
 * SCALABILITY TESTS:
 *
 *     small computation;
 *     large computation;
 *     symbolic-size computation;
 *     runtime-derived computation;
 *     arbitrarily many optimization objectives;
 *     arbitrarily many optimization passes;
 *     arbitrarily many pipeline stages;
 *     arbitrarily large classical data shapes.
 *
 * No test may establish an artificial grammar-level maximum.
 *
 * DETERMINISM TESTS:
 *
 *     identical source + identical semantic environment
 *         => equivalent parse/AST structure.
 *
 * Compatibility tests MUST ensure that additions to the classical optimization
 * domain do not break canonical optimization syntax.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file has intentionally avoided:
 *
 *     CPU counts
 *     core counts
 *     thread counts
 *     GPU counts
 *     FPGA counts
 *     accelerator counts
 *     register counts
 *     register widths
 *     SIMD widths
 *     cache sizes
 *     memory capacities
 *     device identifiers
 *     physical addresses
 *     network node counts
 *     fixed tensor dimensions
 *     fixed vector dimensions
 *     fixed matrix dimensions
 *     fixed optimization-pass counts.
 *
 * Numeric expressions remain ordinary program/semantic expressions.
 *
 * A numeric value appearing in source is not automatically a hardware limit.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * The classical optimization architecture is:
 *
 *     CLASSICAL SOURCE
 *             |
 *             v
 *     CLASSICAL SEMANTICS
 *             |
 *             v
 *     OPTIMIZATION INTENT
 *             |
 *             v
 *     CANONICAL MACHINE-INDEPENDENT IR
 *             |
 *             v
 *     RESOURCE / CAPABILITY ANALYSIS
 *             |
 *             v
 *     OPTIMIZATION PLANNER
 *             |
 *             v
 *     OPTIMIZED IR
 *             |
 *             v
 *     SCHEDULING / ROUTING / LOWERING
 *             |
 *             v
 *     TARGET REALIZATION
 *             |
 *             v
 *     RUNTIME
 *
 * Never:
 *
 *     CLASSICAL SOURCE
 *             |
 *             v
 *     FIXED MACHINE ASSUMPTIONS
 *             |
 *             v
 *     TARGET LOCK-IN
 *
 * The purpose of this file is therefore to make classical optimization a
 * first-class Zamani domain while preserving:
 *
 *     one language;
 *     one semantic architecture;
 *     one canonical IR architecture;
 *     capability/resource-based realization;
 *     no artificial machine limits;
 *     deterministic parsing;
 *     safe Rust;
 *     POCO-REAF.
 *
 * ============================================================================
 */