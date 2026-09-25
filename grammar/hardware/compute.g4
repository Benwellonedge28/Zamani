/**
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hardware/compute.g4
 *
 * Grammar:
 *     ZamaniHardwareComputeParser
 *
 * Kind:
 *     ANTLR4 parser grammar
 *
 * Status:
 *     Production architecture / hardware-compute intent
 *
 * Rust target:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the GENERIC HARDWARE COMPUTATION INTENT layer.
 *
 * It describes computation requirements and implementation-independent
 * execution intent without selecting a physical processor, accelerator,
 * device, node, core, execution unit, register, memory bank, or vendor.
 *
 * It is deliberately broader than:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     NPU
 *
 * Those domains have their own specialized grammars.
 *
 * Generic computation syntax is what allows one Zamani program to describe
 * computation once and defer physical realization to later compilation and
 * runtime stages.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * This grammar therefore describes:
 *
 *     WHAT computation is required
 *     WHAT computation properties are required
 *     WHAT capabilities are required
 *     WHAT resources are required
 *     WHAT scaling behavior is desired
 *     WHAT performance properties matter
 *     WHAT portability properties matter
 *     WHAT implementation choices are preferred
 *
 * It does NOT prescribe:
 *
 *     WHICH CPU
 *     WHICH GPU
 *     WHICH FPGA
 *     WHICH ASIC
 *     WHICH QPU
 *     WHICH NPU
 *     WHICH core
 *     WHICH thread
 *     WHICH register
 *     WHICH physical memory bank
 *     WHICH physical node
 *     WHICH vendor device
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 *
 *     - generic compute declarations;
 *     - compute domains;
 *     - compute execution intent;
 *     - compute resource requirements;
 *     - compute capability requirements;
 *     - compute constraints;
 *     - compute preferences;
 *     - compute hints;
 *     - compute parallelism intent;
 *     - compute vectorization intent;
 *     - compute data-parallel intent;
 *     - compute task-parallel intent;
 *     - compute pipeline intent;
 *     - compute reduction intent;
 *     - compute synchronization intent;
 *     - compute precision intent;
 *     - compute memory-access intent;
 *     - compute scalability intent;
 *     - compute performance intent;
 *     - compute latency intent;
 *     - compute throughput intent;
 *     - compute energy/power intent;
 *     - compute reliability/resilience intent;
 *     - generic compute properties;
 *     - generic compute metadata;
 *     - symbolic compute composition.
 *
 * THIS FILE DOES NOT OWN
 *
 *     - lexical definitions;
 *     - identifiers;
 *     - literals;
 *     - expression precedence;
 *     - generic type syntax;
 *     - generic resource declarations;
 *     - generic capability declarations;
 *     - physical target discovery;
 *     - physical device selection;
 *     - CPU implementation details;
 *     - GPU implementation details;
 *     - FPGA implementation details;
 *     - ASIC implementation details;
 *     - QPU implementation details;
 *     - physical topology;
 *     - placement;
 *     - routing;
 *     - scheduling algorithms;
 *     - register allocation;
 *     - memory allocation;
 *     - machine-code generation;
 *     - kernel compilation;
 *     - driver interaction;
 *     - runtime dispatch;
 *     - calibration;
 *     - benchmarking;
 *     - QEC;
 *     - ZQN;
 *     - HAL implementation;
 *     - deployment.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
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
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic compute model
 *          |
 *          +--> resource analysis
 *          +--> capability resolution
 *          +--> type/effect analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic IR
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware IR
 *          |
 *          v
 *     optimization
 *          |
 *          +--> vectorization
 *          +--> parallelization
 *          +--> lowering
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> QEC/ZQN where applicable
 *          |
 *          v
 *     HAL / backend
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO parser-level universal capacity limits.
 *
 * In particular this grammar MUST NOT define:
 *
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_COMPUTE_UNITS
 *     MAX_ACCELERATORS
 *
 * Collections use ANTLR repetition.
 *
 * Quantities use expressions.
 *
 * Therefore:
 *
 *     count = n
 *     width = workload_width
 *     parallelism = available_parallelism
 *
 * are valid source-level concepts.
 *
 * Physical limits are discovered and validated downstream.
 *
 * ============================================================================
 * SEMANTIC DISTINCTIONS
 * ============================================================================
 *
 * REQUIREMENT
 *     Mandatory condition.
 *
 * CONSTRAINT
 *     Mandatory restriction on legal realization.
 *
 * CAPABILITY
 *     Required or described ability.
 *
 * PREFERENCE
 *     Non-mandatory optimization guidance.
 *
 * HINT
 *     Advisory information.
 *
 * RESOURCE
 *     Abstract resource relationship.
 *
 * TARGET
 *     Abstract execution/compilation context.
 *
 * This grammar does not collapse these concepts into one generic clause.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Expressions must use the canonical Zamani expression grammar.
 *
 * This file deliberately does not redefine:
 *
 *     arithmetic
 *     logical operators
 *     comparisons
 *     calls
 *     indexing
 *     member access
 *     literals
 *     assignment
 *     precedence
 *
 * ============================================================================
 * NAME INTEGRATION
 * ============================================================================
 *
 * Names must use the canonical Zamani name grammar.
 *
 * No vendor/device naming scheme is defined here.
 *
 * ============================================================================
 * ANTLR / RUST SAFETY
 * ============================================================================
 *
 * This grammar contains no target-language actions.
 *
 * No Rust `unsafe` is introduced by this file.
 *
 * Generated Rust must be compiled under the repository's existing:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     #![forbid(unsafe_code)]
 *
 * ============================================================================
 */

parser grammar ZamaniHardwareComputeParser;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Canonical parser composition.
 *
 * The repository is in the process of normalizing modular parser grammars
 * around ZamaniLexer. The expression and name grammars are therefore consumed
 * as shared syntax authorities rather than reimplemented here.
 */
import ZamaniExpressions, ZamaniNames;


/* ============================================================================
 * 1. PUBLIC COMPUTE ENTRY POINT
 * ============================================================================
 *
 * This is the only root entry point owned by this file.
 *
 * hardware.g4 should delegate generic compute constructs here rather than
 * reproducing these rules.
 */

computeDeclaration
    : computeHeader
      computeBody
      SEMICOLON?
    ;


/* ============================================================================
 * 2. COMPUTE HEADER
 * ========================================================================== */

computeHeader
    : COMPUTE
      IDENTIFIER
      computeGenericParameters?
    ;


/* ============================================================================
 * 3. GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic parameters allow compute descriptions to remain symbolic.
 *
 * Example:
 *
 *     compute Kernel<Width, Parallelism, Precision>
 *
 * The grammar does not impose a finite value range.
 */

computeGenericParameters
    : LT
      computeGenericParameter
      (
          COMMA
          computeGenericParameter
      )*
      GT
    ;

computeGenericParameter
    : IDENTIFIER
      (
          COLON
          qualifiedName
      )?
      (
          ASSIGN
          expression
      )?
    ;


/* ============================================================================
 * 4. COMPUTE BODY
 * ========================================================================== */

computeBody
    : LBRACE
      computeClause*
      RBRACE
    ;


/* ============================================================================
 * 5. COMPUTE CLAUSES
 * ========================================================================== */

computeClause
    : computeDomainClause
    | computeOperationClause
    | computeResourceClause
    | computeCapabilityClause
    | computeRequirementClause
    | computeConstraintClause
    | computePreferenceClause
    | computeHintClause
    | computeParallelismClause
    | computeVectorizationClause
    | computeDataParallelClause
    | computeTaskParallelClause
    | computePipelineClause
    | computeReductionClause
    | computeSynchronizationClause
    | computePrecisionClause
    | computeMemoryClause
    | computeScalabilityClause
    | computePerformanceClause
    | computeLatencyClause
    | computeThroughputClause
    | computeBandwidthClause
    | computePowerClause
    | computeEnergyClause
    | computeReliabilityClause
    | computeResilienceClause
    | computePortabilityClause
    | computeTargetClause
    | computePropertyClause
    | computeMetadataClause
    | computeCompositionClause
    ;


/* ============================================================================
 * 6. COMPUTE DOMAIN
 * ============================================================================
 *
 * A domain is symbolic.
 *
 * Examples:
 *
 *     domain = classical;
 *     domain = quantum;
 *     domain = tensor;
 *     domain = scientific;
 *     domain = heterogeneous;
 *     domain = custom::domain;
 *
 * The grammar does not enumerate domains.
 */

computeDomainClause
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 7. OPERATION
 * ============================================================================
 *
 * The operation name is open-ended.
 *
 * This prevents generic computation from becoming a fixed dictionary of
 * operations.
 *
 * Examples:
 *
 *     operation = matrix_multiply;
 *     operation = custom.compute;
 *     operation = tensor.contract;
 *     operation = user_defined_operation;
 */

computeOperationClause
    : IDENTIFIER
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 8. RESOURCE INTENT
 * ============================================================================
 *
 * This refers to an abstract resource.
 *
 * It does not allocate or select a physical resource.
 */

computeResourceClause
    : RESOURCE
      qualifiedName
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 9. CAPABILITY
 * ============================================================================
 *
 * Capability identities remain symbolic and extensible.
 */

computeCapabilityClause
    : CAPABILITY
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 10. REQUIREMENTS
 * ============================================================================
 *
 * Examples:
 *
 *     requires qubits >= n;
 *     requires memory >= required_memory;
 *     requires capability("tensor.compute");
 *     requires capability("gpu.compute");
 *
 * The expression grammar owns comparison and call syntax.
 */

computeRequirementClause
    : REQUIRES
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 11. CONSTRAINTS
 * ========================================================================== */

computeConstraintClause
    : CONSTRAINT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 12. PREFERENCES
 * ========================================================================== */

computePreferenceClause
    : PREFER
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 13. HINTS
 * ========================================================================== */

computeHintClause
    : HINT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 14. PARALLELISM
 * ============================================================================
 *
 * Parallelism is an intent.
 *
 * It is not a processor/thread count.
 */

computeParallelismClause
    : IDENTIFIER
      ASSIGN
      computeParallelismExpression
      SEMICOLON
    ;

computeParallelismExpression
    : expression
    ;


/* ============================================================================
 * 15. VECTORISATION
 * ============================================================================
 *
 * Vector width is an expression rather than a fixed machine width.
 */

computeVectorizationClause
    : IDENTIFIER
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 16. DATA PARALLELISM
 * ========================================================================== */

computeDataParallelClause
    : IDENTIFIER
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 17. TASK PARALLELISM
 * ========================================================================== */

computeTaskParallelClause
    : IDENTIFIER
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 18. PIPELINING
 * ========================================================================== */

computePipelineClause
    : IDENTIFIER
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 19. REDUCTION
 * ========================================================================== */

computeReductionClause
    : IDENTIFIER
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 20. SYNCHRONIZATION
 * ========================================================================== */

computeSynchronizationClause
    : IDENTIFIER
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 21. PRECISION
 * ========================================================================== */

computePrecisionClause
    : IDENTIFIER
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 22. MEMORY INTENT
 * ============================================================================
 *
 * This is an abstract memory relationship.
 *
 * It does not select a physical memory bank or impose a physical capacity.
 */

computeMemoryClause
    : MEMORY
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 23. SCALABILITY
 * ============================================================================
 *
 * Examples:
 *
 *     scalability = linear;
 *     scalability = workload_size;
 *     scalability = elastic;
 *     scalability = expression;
 *
 * The grammar does not define a maximum scale.
 */

computeScalabilityClause
    : SCALABILITY
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 24. PERFORMANCE
 * ========================================================================== */

computePerformanceClause
    : PERFORMANCE
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 25. LATENCY
 * ========================================================================== */

computeLatencyClause
    : LATENCY
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 26. THROUGHPUT
 * ========================================================================== */

computeThroughputClause
    : THROUGHPUT
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 27. BANDWIDTH
 * ========================================================================== */

computeBandwidthClause
    : BANDWIDTH
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 28. POWER
 * ========================================================================== */

computePowerClause
    : POWER
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 29. ENERGY
 * ========================================================================== */

computeEnergyClause
    : ENERGY
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 30. RELIABILITY
 * ========================================================================== */

computeReliabilityClause
    : RELIABILITY
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 31. RESILIENCE
 * ========================================================================== */

computeResilienceClause
    : RESILIENCE
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 32. PORTABILITY
 * ========================================================================== */

computePortabilityClause
    : PORTABILITY
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 33. ABSTRACT TARGET
 * ============================================================================
 *
 * TARGET identifies an abstract compilation/execution context.
 *
 * It must not be interpreted as a physical device identifier.
 */

computeTargetClause
    : TARGET
      ASSIGN
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 34. EXTENSIBLE PROPERTY
 * ============================================================================
 *
 * Property names are qualified names.
 *
 * This allows future domains to attach information without adding a new
 * keyword for every possible property.
 */

computePropertyClause
    : PROPERTY
      qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 35. METADATA
 * ============================================================================
 *
 * Metadata is descriptive and does not itself alter execution semantics.
 */

computeMetadataClause
    : METADATA
      LBRACE
      computeMetadataEntry*
      RBRACE
    ;

computeMetadataEntry
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 36. COMPOSITION
 * ============================================================================
 *
 * Generic compute units may refer to other abstract compute units.
 *
 * These relationships are resolved semantically.
 */

computeCompositionClause
    : qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 37. CANONICAL COMPUTE EXPRESSIONS
 * ============================================================================
 *
 * This rule exists only as an explicit integration boundary.
 *
 * It intentionally delegates all actual expression syntax.
 */

computeExpression
    : expression
    ;