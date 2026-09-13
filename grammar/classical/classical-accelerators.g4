/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/classical-accelerators.g4
 *
 * Status:
 *     Production-ready classical-accelerator domain parser grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No target-specific code.
 *     - No filesystem access.
 *     - No network access.
 *     - No device access.
 *     - No evaluation.
 *     - No allocation.
 *     - No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL CLASSICAL ACCELERATOR SYNTAX.
 *
 * It provides the parser boundary for expressing computations that may be
 * intended for, constrained toward, or described in terms of accelerator
 * execution.
 *
 * An accelerator is treated as a COMPUTATIONAL CAPABILITY, not as a fixed
 * physical object.
 *
 * The grammar therefore permits source programs to express concepts such as:
 *
 *     accelerator computations
 *     accelerator invocations
 *     accelerator-qualified operations
 *     accelerator execution regions
 *     accelerator implementation hints
 *     accelerator requirements
 *     accelerator constraints
 *     accelerator preferences
 *
 * without encoding:
 *
 *     a particular GPU;
 *     a particular FPGA;
 *     a particular ASIC;
 *     a particular TPU;
 *     a particular accelerator vendor;
 *     a fixed accelerator count;
 *     a fixed device count;
 *     a fixed memory size;
 *     a fixed SIMD width;
 *     a fixed vector width;
 *     a fixed work-group size;
 *     a fixed warp size;
 *     a fixed topology;
 *     a fixed address;
 *     a fixed deployment.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     ClassicalAccelerators
 *          |
 *          v
 *     Frontend AST
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     semantic analysis             capability/resource
 *          |                         analysis
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                  canonical semantic
 *                    representation
 *                        |
 *              +---------+----------+
 *              |                    |
 *              v                    v
 *        classical IR       resource/effect metadata
 *              |
 *              v
 *        optimization
 *              |
 *              v
 *        scheduling / placement / lowering
 *              |
 *              v
 *        target realization
 *              |
 *       +------+-------+----------------+
 *       |              |                |
 *       v              v                v
 *      CPU            GPU          FPGA / ASIC /
 *                                  future accelerator
 *
 * This grammar does NOT directly construct:
 *
 *     classical IR
 *     quantum::ir
 *     hardware IR
 *     accelerator runtime objects
 *     device state
 *     scheduling state
 *     placement state
 *     QEC state
 *     ZQN state
 *     runtime state
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - accelerator-domain parser composition;
 *     - accelerator invocation syntax;
 *     - accelerator-qualified computation syntax;
 *     - accelerator execution-region syntax;
 *     - accelerator semantic-intent wrappers;
 *     - accelerator requirement/constraint/preference syntactic wrappers;
 *     - accelerator execution-hint syntax where represented by existing
 *       language tokens;
 *     - stable parser entry points for classical accelerator consumers.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token spelling;
 *     - identifiers;
 *     - qualified-name syntax;
 *     - general expressions;
 *     - expression precedence;
 *     - general statements;
 *     - declarations;
 *     - function declarations;
 *     - generic type syntax;
 *     - vector type syntax;
 *     - matrix type syntax;
 *     - tensor type syntax;
 *     - accelerator type definitions;
 *     - accelerator implementation;
 *     - GPU architecture;
 *     - FPGA architecture;
 *     - ASIC architecture;
 *     - accelerator discovery;
 *     - device selection;
 *     - device identifiers;
 *     - device addresses;
 *     - memory allocation;
 *     - scheduling;
 *     - placement;
 *     - routing;
 *     - optimization;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - simulation;
 *     - runtime execution;
 *     - backend selection.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexical authority is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar MUST NOT declare lexer rules.
 *
 * In particular, this file MUST NOT introduce alternative spellings or
 * aliases for:
 *
 *     IDENTIFIER
 *     GPU
 *     SIM
 *     VECTORIZED
 *     PARALLEL
 *     REQUIRES
 *     WITH
 *     ASSIGN
 *     COLON
 *     COMMA
 *     DOT
 *     DOUBLE_COLON
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     SEMICOLON
 *
 * or any other lexer token.
 *
 * Accelerator implementations, vendors, devices, kernels, algorithms and
 * capabilities MUST NOT be converted into an exhaustive lexer keyword list.
 *
 * For example, names such as:
 *
 *     CUDA
 *     ROCm
 *     Metal
 *     Vulkan
 *     OpenCL
 *     TPU
 *     NPU
 *     DSP
 *     FPGA
 *     ASIC
 *     custom_accelerator
 *
 * remain semantic names unless the canonical language specification explicitly
 * reserves a spelling.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expression syntax is owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * Therefore this file MUST consume:
 *
 *     expression
 *
 * rather than redefine:
 *
 *     arithmetic precedence;
 *     comparison precedence;
 *     logical precedence;
 *     bitwise precedence;
 *     unary precedence;
 *     assignment;
 *     calls;
 *     indexing;
 *     member access;
 *     ranges;
 *     lambdas;
 *     comprehensions.
 *
 * This prevents accelerator syntax from becoming a second expression language.
 *
 * ============================================================================
 * NAME CONTRACT
 * ============================================================================
 *
 * Accelerator names are semantic names.
 *
 * The parser does not determine whether a name represents:
 *
 *     an accelerator;
 *     an accelerator family;
 *     a capability;
 *     a namespace;
 *     a library;
 *     a compiler intrinsic;
 *     a user-defined operation;
 *     a future accelerator abstraction.
 *
 * Name resolution belongs downstream.
 *
 * The grammar therefore accepts arbitrary identifier-based accelerator paths
 * where the surrounding syntax explicitly establishes accelerator context.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Accelerator syntax describes COMPUTATION and INTENT.
 *
 * It MUST NOT encode:
 *
 *     MAX_ACCELERATORS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_DEVICES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_LANES
 *     MAX_WARPS
 *     MAX_WORKGROUPS
 *     MAX_VECTOR_WIDTH
 *     MAX_MEMORY
 *     MAX_SHARED_MEMORY
 *     MAX_LOCAL_MEMORY
 *     MAX_DEVICE_MEMORY
 *     MAX_STREAMS
 *     MAX_NODES
 *
 * Nor may it encode:
 *
 *     GPU 0
 *     GPU 1
 *     device 0
 *     accelerator 0
 *     fixed PCI address
 *     fixed memory address
 *     fixed topology
 *     fixed vendor
 *
 * unless such information is explicitly part of source-level semantic intent
 * through another established language mechanism.
 *
 * A program can consequently remain semantically stable while being realized
 * on:
 *
 *     no accelerator;
 *     one accelerator;
 *     many accelerators;
 *     a CPU;
 *     a GPU;
 *     multiple GPUs;
 *     an FPGA;
 *     an ASIC;
 *     a specialized accelerator;
 *     a heterogeneous system;
 *     a distributed accelerator fabric;
 *     a future accelerator architecture.
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * Accelerator availability is NOT established by this grammar.
 *
 * Resource discovery belongs to the resource/hardware/compiler layers.
 *
 * For example:
 *
 *     requires accelerator
 *
 * must not implicitly mean:
 *
 *     requires GPU X
 *
 * and:
 *
 *     prefers GPU
 *
 * must not mean:
 *
 *     use exactly one GPU
 *
 * Likewise:
 *
 *     accelerator computation
 *
 * must not mean:
 *
 *     execute on a particular vendor device.
 *
 * Semantic resource requirements and implementation choices remain separate.
 *
 * ============================================================================
 * CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Capability checking occurs after parsing.
 *
 * Possible capabilities include:
 *
 *     vector execution
 *     tensor execution
 *     matrix acceleration
 *     parallel execution
 *     GPU execution
 *     FPGA execution
 *     specialized arithmetic
 *     low-precision arithmetic
 *     high-throughput execution
 *     accelerator memory
 *     asynchronous execution
 *     streaming
 *     other future capabilities
 *
 * This grammar does not maintain a finite capability vocabulary.
 *
 * ============================================================================
 * EXECUTION MODEL
 * ============================================================================
 *
 * Accelerator syntax may describe:
 *
 *     what computation is intended;
 *     what capability is preferred;
 *     what capability is required;
 *     what implementation family is acceptable;
 *     what constraints apply;
 *     what hints may improve realization.
 *
 * It does not describe:
 *
 *     how the accelerator executes;
 *     how work is scheduled;
 *     how memory is allocated;
 *     how kernels are compiled;
 *     how work is partitioned;
 *     how devices are selected;
 *     how communication occurs;
 *     how hardware is programmed.
 *
 * Those responsibilities belong downstream.
 *
 * ============================================================================
 * SYNTAX DESIGN
 * ============================================================================
 *
 * This grammar deliberately uses the already-established language vocabulary:
 *
 *     GPU
 *     SIM
 *     VECTORIZED
 *     PARALLEL
 *     REQUIRES
 *     WITH
 *
 * and ordinary identifiers.
 *
 * It does NOT introduce new lexer keywords such as:
 *
 *     accelerator
 *     kernel
 *     offload
 *     device
 *     launch
 *
 * because doing so would create an unnecessary lexical compatibility change.
 *
 * Accelerator-specific names may therefore be represented through qualified
 * semantic names such as:
 *
 *     accelerator::matrix
 *     accelerator::tensor
 *     accelerator::compute
 *     gpu::kernel
 *     fpga::pipeline
 *
 * where the relevant spelling is lexically available as an identifier.
 *
 * Reserved tokens such as GPU remain available for explicit execution-class
 * syntax.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `classicalAcceleratorConstruct` is the stable domain entry point.
 *
 * It is intentionally separate from `expression`.
 *
 * A parent grammar can therefore explicitly enter accelerator syntax without
 * changing the general expression grammar.
 *
 * ============================================================================
 */

parser grammar ClassicalAccelerators;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/*
 * ============================================================================
 * 1. PUBLIC DOMAIN ENTRY POINT
 * ============================================================================
 *
 * This rule recognizes syntactic constructs explicitly associated with
 * accelerator-oriented classical computation.
 */
classicalAcceleratorConstruct
    : acceleratorQualifiedInvocation
    | acceleratorQualifiedReference
    | acceleratorExecutionQualifier
    | acceleratorRequirement
    | acceleratorConstraint
    | acceleratorPreference
    | acceleratorHint
    ;


/*
 * ============================================================================
 * 2. ACCELERATOR QUALIFIED REFERENCE
 * ============================================================================
 *
 * A qualified accelerator reference provides an explicit semantic namespace
 * boundary without requiring accelerator names to become lexer keywords.
 *
 * Examples:
 *
 *     accelerator::matrix
 *     accelerator::tensor
 *     accelerator::compute
 *
 * The parser does not determine whether `accelerator` denotes a namespace,
 * capability family, library, resource class or user-defined entity.
 *
 * Semantic analysis performs that resolution.
 */
acceleratorQualifiedReference
    : acceleratorNamespace
      DOUBLE_COLON
      acceleratorName
      (
          DOUBLE_COLON acceleratorName
      )*
    ;


/*
 * ============================================================================
 * 3. ACCELERATOR QUALIFIED INVOCATION
 * ============================================================================
 *
 * Examples:
 *
 *     accelerator::compute(data)
 *     accelerator::matrix::multiply(a, b)
 *     accelerator::tensor::transform(value)
 *
 * There is no fixed argument count.
 *
 * The meaning of the callable is resolved semantically.
 */
acceleratorQualifiedInvocation
    : acceleratorQualifiedReference
      LPAREN
      acceleratorArguments?
      RPAREN
    ;


/*
 * ============================================================================
 * 4. ACCELERATOR NAMESPACE
 * ============================================================================
 *
 * `accelerator` is intentionally represented by IDENTIFIER rather than a new
 * ACCELERATOR lexer token.
 *
 * This avoids expanding the reserved-word set and permits user-defined
 * accelerator namespaces.
 *
 * Reserved execution-class tokens are also accepted as namespace roots where
 * they already exist in the canonical lexer.
 *
 * Examples:
 *
 *     accelerator::compute
 *     gpu::kernel
 *     sim::compute
 *
 * The semantic layer determines the actual meaning.
 */
acceleratorNamespace
    : IDENTIFIER
    | GPU
    | SIM
    ;


/*
 * ============================================================================
 * 5. ACCELERATOR NAME
 * ============================================================================
 *
 * Accelerator operation names remain ordinary identifiers.
 *
 * This is important for future extensibility:
 *
 *     vendor::operation
 *     domain::operation
 *     custom_accelerator::operation
 *
 * do not require grammar modification merely because a new accelerator
 * operation is introduced.
 */
acceleratorName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 6. ACCELERATOR ARGUMENTS
 * ============================================================================
 *
 * Arguments are ordinary Zamani expressions.
 *
 * This allows accelerator operations to consume:
 *
 *     scalars
 *     vectors
 *     matrices
 *     tensors
 *     symbolic expressions
 *     function results
 *     references
 *     ranges
 *     classical values
 *     quantum-derived classical values
 *     future domain values
 *
 * No machine-specific argument representation is imposed.
 */
acceleratorArguments
    : expression
      (
          COMMA expression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 7. ACCELERATOR EXECUTION QUALIFIER
 * ============================================================================
 *
 * These forms provide syntactic intent without selecting a concrete device.
 *
 * Examples:
 *
 *     gpu expression
 *     sim expression
 *     vectorized expression
 *     parallel expression
 *
 * The meaning of the qualifier is resolved downstream.
 *
 * In particular:
 *
 *     GPU
 *
 * does NOT mean:
 *
 *     one GPU
 *
 * or:
 *
 *     a particular GPU.
 *
 * Likewise:
 *
 *     PARALLEL
 *
 * does not establish a number of execution units.
 */
acceleratorExecutionQualifier
    : acceleratorExecutionMode
      expression
    ;


/*
 * ============================================================================
 * 8. EXECUTION MODE
 * ============================================================================
 *
 * These tokens already belong to the canonical lexical vocabulary.
 */
acceleratorExecutionMode
    : GPU
    | SIM
    | VECTORIZED
    | PARALLEL
    ;


/*
 * ============================================================================
 * 9. ACCELERATOR REQUIREMENT
 * ============================================================================
 *
 * Requirements describe semantic necessity.
 *
 * Examples:
 *
 *     requires accelerator::tensor
 *     requires accelerator::matrix
 *     requires gpu::tensor
 *
 * The referenced capability/resource is resolved by semantic analysis.
 *
 * This does NOT select a physical device.
 */
acceleratorRequirement
    : REQUIRES
      acceleratorRequirementTarget
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 10. REQUIREMENT TARGET
 * ============================================================================
 *
 * A requirement target is represented as an accelerator-qualified semantic
 * name.
 *
 * No physical resource identifier is required or implied.
 */
acceleratorRequirementTarget
    : acceleratorQualifiedReference
    ;


/*
 * ============================================================================
 * 11. ACCELERATOR CONSTRAINT
 * ============================================================================
 *
 * `WITH` provides a syntactic composition boundary for accelerator-related
 * constraints or policies when the surrounding language construct establishes
 * that interpretation.
 *
 * Example:
 *
 *     with accelerator::memory expression
 *
 * The grammar does not interpret the constraint.
 */
acceleratorConstraint
    : WITH
      acceleratorConstraintTarget
      expression
    ;


/*
 * ============================================================================
 * 12. ACCELERATOR CONSTRAINT TARGET
 * ============================================================================
 */
acceleratorConstraintTarget
    : acceleratorQualifiedReference
    ;


/*
 * ============================================================================
 * 13. ACCELERATOR PREFERENCE
 * ============================================================================
 *
 * Preferences are deliberately distinct from requirements.
 *
 * A preference may be ignored or replaced by the compiler when necessary to
 * preserve correctness, portability, resource feasibility or other stronger
 * constraints.
 *
 * The parser only establishes the syntax.
 */
acceleratorPreference
    : acceleratorPreferenceMode
      acceleratorQualifiedReference
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 14. ACCELERATOR PREFERENCE MODE
 * ============================================================================
 *
 * `VECTORIZED` and `GPU` are existing lexical forms.
 *
 * They express implementation preference/classification rather than a fixed
 * resource allocation.
 */
acceleratorPreferenceMode
    : VECTORIZED
    | GPU
    | PARALLEL
    ;


/*
 * ============================================================================
 * 15. ACCELERATOR HINT
 * ============================================================================
 *
 * A hint is weaker than a requirement.
 *
 * This grammar deliberately represents a hint as:
 *
 *     execution qualifier + semantic target
 *
 * without prescribing how the compiler must realize it.
 */
acceleratorHint
    : acceleratorHintMode
      acceleratorHintTarget
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 16. ACCELERATOR HINT MODE
 * ============================================================================
 */
acceleratorHintMode
    : SIM
    | VECTORIZED
    | GPU
    | PARALLEL
    ;


/*
 * ============================================================================
 * 17. ACCELERATOR HINT TARGET
 * ============================================================================
 */
acceleratorHintTarget
    : acceleratorQualifiedReference
    ;


/*
 * ============================================================================
 * 18. EXPRESSION-LEVEL ACCELERATOR CALL
 * ============================================================================
 *
 * This stable wrapper permits semantic consumers to explicitly classify a
 * general accelerator-qualified invocation without redefining call syntax.
 *
 * The actual invocation syntax remains ordinary expression composition.
 */
acceleratorCallExpression
    : acceleratorQualifiedInvocation
    ;


/*
 * ============================================================================
 * 19. ACCELERATOR COMPUTATION
 * ============================================================================
 *
 * An accelerator computation is represented as an expression in an
 * accelerator-qualified context.
 *
 * This is intentionally not an independent expression hierarchy.
 */
acceleratorComputation
    : acceleratorExecutionQualifier
    ;


/*
 * ============================================================================
 * 20. ACCELERATOR RESOURCE REFERENCE
 * ============================================================================
 *
 * This rule provides a stable syntactic classification point for semantic
 * resource analysis.
 *
 * It does not allocate or select a resource.
 */
acceleratorResourceReference
    : acceleratorQualifiedReference
    ;


/*
 * ============================================================================
 * 21. ACCELERATOR CAPABILITY REFERENCE
 * ============================================================================
 *
 * Capabilities remain semantic names.
 *
 * Examples:
 *
 *     accelerator::tensor
 *     accelerator::matrix
 *     accelerator::vector
 *     accelerator::stream
 *
 * The grammar does not maintain a finite list.
 */
acceleratorCapabilityReference
    : acceleratorQualifiedReference
    ;


/*
 * ============================================================================
 * 22. ACCELERATOR OPERATION REFERENCE
 * ============================================================================
 *
 * Operations are intentionally open-ended.
 */
acceleratorOperationReference
    : acceleratorQualifiedReference
    ;


/*
 * ============================================================================
 * 23. ACCELERATOR OPERATION INVOCATION
 * ============================================================================
 *
 * This is an explicit stable entry point for accelerator operation calls.
 */
acceleratorOperationInvocation
    : acceleratorQualifiedInvocation
    ;


/*
 * ============================================================================
 * 24. OPTIONAL SEMANTIC CLASSIFICATION WRAPPER
 * ============================================================================
 *
 * The following rule allows downstream parser compositions to consume a
 * general expression as an accelerator-domain expression without redefining
 * expression syntax.
 *
 * It intentionally contains only `expression`.
 *
 * Semantic analysis determines whether the expression actually denotes an
 * accelerator computation.
 */
acceleratorValueExpression
    : expression
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Parser consumers should preserve:
 *
 *     - source locations;
 *     - token boundaries;
 *     - accelerator namespace segments;
 *     - operation name;
 *     - argument ordering;
 *     - qualifier ordering;
 *     - requirement/preference/hint distinction;
 *     - original source spelling.
 *
 * The frontend AST may represent these as equivalent structures such as:
 *
 *     AcceleratorReference
 *     AcceleratorInvocation
 *     AcceleratorExecutionQualifier
 *     AcceleratorRequirement
 *     AcceleratorConstraint
 *     AcceleratorPreference
 *     AcceleratorHint
 *
 * Exact AST structure belongs to the frontend AST layer, not this grammar.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - whether the referenced entity exists;
 *     - whether it is an accelerator capability;
 *     - whether it is an operation;
 *     - whether arguments have valid types;
 *     - whether argument shapes are compatible;
 *     - whether the operation is pure or effectful;
 *     - whether the operation can execute on an available target;
 *     - whether requirements can be satisfied;
 *     - whether constraints are satisfiable;
 *     - whether preferences can be honored;
 *     - whether the computation can be vectorized;
 *     - whether parallelization is legal;
 *     - whether offloading is profitable;
 *     - whether fallback execution exists;
 *     - whether resource requirements are feasible.
 *
 * None of these decisions are made by this parser.
 *
 * ============================================================================
 * CLASSICAL IR INTEGRATION
 * ============================================================================
 *
 * The grammar MUST NOT directly construct classical IR.
 *
 * The intended pipeline is:
 *
 *     parser context
 *         ->
 *     frontend AST
 *         ->
 *     semantic analysis
 *         ->
 *     classical semantic representation
 *         ->
 *     classical IR
 *
 * Accelerator-specific information may become:
 *
 *     operation metadata;
 *     capability requirements;
 *     resource requirements;
 *     execution preferences;
 *     effect information;
 *     lowering constraints;
 *     provenance.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Accelerator syntax may appear around expressions that ultimately participate
 * in hybrid quantum-classical computation.
 *
 * This file does NOT:
 *
 *     - create qubits;
 *     - select QPUs;
 *     - create quantum operations;
 *     - construct quantum::ir;
 *     - perform routing;
 *     - perform scheduling;
 *     - invoke QEC;
 *     - interpret ZQN faults.
 *
 * If an accelerator computation consumes a value derived from quantum
 * computation, semantic analysis determines whether that interaction is
 * valid.
 *
 * The quantum semantic boundary remains:
 *
 *     frontend AST
 *         ->
 *     semantic analysis
 *         ->
 *     quantum::ir
 *
 * ============================================================================
 * VECTOR / MATRIX / TENSOR INTEGRATION
 * ============================================================================
 *
 * Vector, matrix and tensor values remain ordinary expressions.
 *
 * Their syntax belongs to their respective classical domain grammars and the
 * general expression layer.
 *
 * This file does not redefine:
 *
 *     vector literals;
 *     matrix literals;
 *     tensor literals;
 *     indexing;
 *     slicing;
 *     shape syntax;
 *     arithmetic;
 *     broadcasting.
 *
 * Accelerator realization of those values is downstream.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Accelerator requirements MUST eventually integrate with the universal
 * resource model.
 *
 * The conceptual distinction is:
 *
 *     requirement
 *         !=
 *     constraint
 *         !=
 *     capability
 *         !=
 *     preference
 *         !=
 *     hint
 *         !=
 *     placement
 *         !=
 *     selected device
 *
 * This distinction is essential for POCO-REAF.
 *
 * A source program may require:
 *
 *     accelerator::tensor
 *
 * without specifying:
 *
 *     GPU 0
 *     GPU 1
 *     device X
 *     vendor Y
 *     memory address Z
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler may use parsed accelerator constructs for:
 *
 *     - capability checking;
 *     - resource analysis;
 *     - target-independent lowering;
 *     - accelerator eligibility;
 *     - vectorization;
 *     - parallelization;
 *     - offload analysis;
 *     - kernel formation;
 *     - scheduling constraints;
 *     - code generation;
 *     - fallback generation;
 *     - provenance.
 *
 * Compiler implementation policy MUST remain outside this grammar.
 *
 * ============================================================================
 * OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Optimization owns:
 *
 *     - vectorization;
 *     - fusion;
 *     - tiling;
 *     - layout transformations;
 *     - kernel optimization;
 *     - target-specific optimization;
 *     - accelerator-specific lowering.
 *
 * This grammar does not implement any of these.
 *
 * ============================================================================
 * SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Scheduling owns:
 *
 *     - execution order;
 *     - dependency ordering;
 *     - resource allocation;
 *     - temporal placement;
 *     - synchronization;
 *     - overlap;
 *     - pipeline scheduling;
 *     - accelerator dispatch timing.
 *
 * This grammar contains no timing grid, work-group size, warp size, stream
 * count, or device queue model.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware abstraction owns:
 *
 *     - actual devices;
 *     - capabilities;
 *     - supported operations;
 *     - memory;
 *     - topology;
 *     - interfaces;
 *     - calibration;
 *     - target-specific properties.
 *
 * This grammar only describes source-level intent.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime consumes compiler-produced representations.
 *
 * Runtime MUST NOT parse source-level accelerator grammar in order to discover
 * hardware.
 *
 * Runtime resource identity must remain distinct from source-level semantic
 * identity.
 *
 * ============================================================================
 * TOOLING INTEGRATION
 * ============================================================================
 *
 * IDEs, language servers, formatters, linters and documentation tools may use
 * these rules to identify accelerator-oriented constructs.
 *
 * Tooling should preserve:
 *
 *     source spelling;
 *     qualified-name structure;
 *     argument order;
 *     qualifier structure;
 *     requirement/preference/hint distinctions.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar performs:
 *
 *     no I/O;
 *     no filesystem access;
 *     no network access;
 *     no device access;
 *     no code execution;
 *     no allocation;
 *     no dynamic loading.
 *
 * Accelerator names and targets must never be interpreted as commands by the
 * parser.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no target-dependent branches;
 *     - no environment-dependent behavior;
 *     - no randomness.
 *
 * The same token stream therefore receives the same syntactic interpretation.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar imposes no finite limit on:
 *
 *     - accelerator namespace depth;
 *     - number of accelerator operations;
 *     - number of arguments;
 *     - number of requirements;
 *     - number of constraints;
 *     - number of hints;
 *     - number of expressions;
 *     - number of accelerator constructs.
 *
 * Repetition is structural.
 *
 * Practical limits, if required for denial-of-service protection or compiler
 * resource management, MUST be explicit parser/compiler resource policy and
 * MUST NOT become language semantics.
 *
 * "Infinity" therefore means:
 *
 *     no artificial finite machine-scale ceiling is encoded by this grammar.
 *
 * Actual compilation and execution remain bounded by available computational
 * resources and explicit implementation policies.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_ACCELERATORS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_THREADS
 *     MAX_LANES
 *     MAX_WARPS
 *     MAX_WORKGROUPS
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * Forbidden:
 *
 *     fixed device IDs;
 *     fixed addresses;
 *     fixed vendor names as required syntax;
 *     fixed accelerator counts;
 *     fixed GPU counts;
 *     fixed work-group sizes;
 *     fixed vector widths;
 *     fixed memory sizes;
 *     fixed accelerator topology.
 *
 * Any such limitation belongs downstream to explicit resource, capability,
 * compiler, scheduling, deployment or runtime policy.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file is additive.
 *
 * It does not redefine the canonical lexer.
 *
 * It does not replace general expression syntax.
 *
 * It does not introduce an `ACCELERATOR` lexer keyword.
 *
 * This preserves compatibility with existing source names and the current
 * lexical architecture.
 *
 * If future language evolution decides that `accelerator` itself must become a
 * reserved keyword, that is a language/lexer compatibility change and must be
 * handled through the canonical lexer/versioning policy rather than silently
 * changing this file.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST cover:
 *
 *     accelerator::compute(data)
 *     accelerator::matrix::multiply(a, b)
 *     accelerator::tensor::transform(value)
 *     gpu::kernel(data)
 *     sim::compute(value)
 *
 *     gpu expression
 *     sim expression
 *     vectorized expression
 *     parallel expression
 *
 *     requires accelerator::tensor;
 *     requires accelerator::matrix;
 *
 *     with accelerator::memory expression
 *
 *     vectorized accelerator::tensor;
 *     gpu accelerator::compute;
 *     parallel accelerator::compute;
 *
 * Negative tests MUST cover:
 *
 *     malformed qualified names;
 *     missing `::`;
 *     missing operation name;
 *     malformed argument lists;
 *     missing closing parenthesis;
 *     malformed requirement targets;
 *     malformed constraint targets;
 *     malformed qualifier expressions.
 *
 * Boundary tests MUST cover:
 *
 *     empty argument list;
 *     one argument;
 *     many arguments;
 *     deeply qualified accelerator names;
 *     deeply nested expressions;
 *     very large source programs;
 *     very large accelerator construct sequences.
 *
 * Scalability tests MUST verify that the grammar contains no artificial limit
 * on:
 *
 *     accelerator count;
 *     device count;
 *     argument count;
 *     namespace depth;
 *     data size;
 *     vector size;
 *     tensor dimensions;
 *     execution scale.
 *
 * Cross-domain tests MUST include:
 *
 *     classical + accelerator;
 *     vector + accelerator;
 *     matrix + accelerator;
 *     tensor + accelerator;
 *     classical + quantum-derived value + accelerator;
 *     classical + accelerator + distributed;
 *     classical + accelerator + HDL/hardware.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     1. It compiles as an ANTLR4 parser grammar against the canonical lexer.
 *
 *     2. It imports only established grammar composition boundaries.
 *
 *     3. It introduces no duplicate expression hierarchy.
 *
 *     4. It introduces no lexer aliases.
 *
 *     5. It introduces no machine-size limit.
 *
 *     6. It does not select a physical accelerator.
 *
 *     7. It does not perform semantic evaluation.
 *
 *     8. It preserves source-level accelerator intent.
 *
 *     9. It provides stable parser entry points for compiler integration.
 *
 *    10. Positive, negative, boundary, scalability and cross-domain tests
 *        pass.
 *
 *    11. Frontend AST mappings are documented and verified.
 *
 *    12. Classical IR lowering consumes semantic analysis output rather than
 *        parser-specific accelerator objects.
 *
 *    13. Resource/capability checking remains downstream.
 *
 *    14. Optimization, scheduling, hardware and runtime remain downstream.
 *
 *    15. No `unsafe` implementation is introduced anywhere in the Rust
 *        integration.
 *
 * ============================================================================
 */