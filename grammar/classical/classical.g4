/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 *     grammar/classical/classical.g4
 *
 * STATUS
 *     CANONICAL CLASSICAL-DOMAIN COMPOSITION GRAMMAR
 *
 * PURPOSE
 *     Defines the single classical-domain parser boundary used by the
 *     canonical Zamani parser composition.
 *
 *     This grammar is intentionally NOT a second expression grammar, type
 *     system, declaration grammar, statement grammar, mathematical-library
 *     grammar, hardware grammar, or classical IR.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 *                    Zamani.g4
 *                         |
 *                         v
 *                  ZamaniParser.g4
 *                         |
 *          +--------------+----------------+
 *          |              |                |
 *       core/types/   expressions/      classical/
 *          |              |                |
 *          +--------------+----------------+
 *                         |
 *                         v
 *                 Domain-neutral AST
 *                         |
 *                         v
 *                  Semantic analysis
 *                         |
 *                         v
 *                 Canonical semantic IR
 *                         |
 *                         v
 *                  Classical lowering
 *                         |
 *          +--------------+----------------+
 *          |              |                |
 *      optimization    scheduling      placement
 *          |              |                |
 *          +--------------+----------------+
 *                         |
 *                         v
 *                 target realization
 *
 * Classical computation can subsequently be realized on CPUs, multicore
 * systems, GPUs, FPGAs, ASICs, accelerators, distributed systems, embedded
 * systems, future machines, or combinations of these.
 *
 * The source grammar does not select the realization.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Zamani supports the architectural goal:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Anywhere
 *     Forever
 *
 * This file therefore MUST NOT encode universal limits for:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     registers
 *     SIMD/vector widths
 *     memory
 *     cache
 *     NUMA nodes
 *     distributed nodes
 *     network links
 *     tensor rank
 *     tensor dimensions
 *     vector length
 *     matrix dimensions
 *     operation count
 *     function count
 *     process count
 *     task count
 *     device count
 *
 * Program literals such as:
 *
 *     1024
 *     4096
 *     n
 *     rows * columns
 *
 * remain ordinary program semantics.
 *
 * They MUST NOT be converted into compiler-wide capacity limits.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT introduce or depend on universal constants such as:
 *
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ACCELERATORS
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_LENGTH
 *     MAX_MATRIX_ROWS
 *     MAX_MATRIX_COLUMNS
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSIONS
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * It also MUST NOT enumerate:
 *
 *     CPU models
 *     GPU models
 *     FPGA families
 *     accelerator models
 *     instruction-set extensions
 *     vendor-specific mathematical libraries
 *
 * Those belong to target capability descriptions, backend lowering,
 * optimization, scheduling, resource analysis, and deployment.
 *
 * ============================================================================
 * DOMAIN MODEL
 * ============================================================================
 *
 * "Classical" is a semantic domain, not a separate language.
 *
 * The following can all have classical semantics:
 *
 *     scalar values
 *     integers
 *     floating-point values
 *     arbitrary-precision values
 *     vectors
 *     matrices
 *     tensors
 *     symbolic values
 *     numerical computations
 *     statistical computations
 *     signal-processing computations
 *     optimization computations
 *     scientific computations
 *     control computations
 *     data computations
 *     AI/ML computations
 *     distributed computations
 *     accelerator computations
 *
 * The parser does not decide which semantic category a value belongs to.
 *
 * Example:
 *
 *     x + y
 *
 * remains an ordinary Zamani expression.
 *
 * Semantic analysis determines whether x and y are integers, vectors,
 * matrices, tensors, symbolic values, distributed values, or another
 * supported semantic type.
 *
 * ============================================================================
 * CANONICAL SYNTAX OWNERSHIP
 * ============================================================================
 *
 * This grammar deliberately reuses canonical rules supplied by the parser
 * composition hierarchy.
 *
 * expression
 *     -> expressions/
 *
 * typeExpression
 *     -> types/
 *
 * statement
 *     -> statements/
 *
 * declaration
 *     -> declarations/
 *
 * functionDeclaration
 *     -> functions/
 *
 * identifier
 *     -> core/
 *
 * qualifiedName
 *     -> core/
 *
 * block
 *     -> core/statements/
 *
 * Assignment, indexing, calls, operators, literals, comprehensions, lambdas,
 * pattern matching, control flow, and generic syntax remain owned by their
 * canonical grammar domains.
 *
 * ============================================================================
 * CLASSICAL FEATURE FILES
 * ============================================================================
 *
 * The existing classical directory contains specialized grammar surfaces
 * including:
 *
 *     scalar.g4
 *     integer.g4
 *     floating-point.g4
 *     vector.g4
 *     matrix.g4
 *     tensor.g4
 *     arithmetic.g4
 *     numeric.g4
 *     numerical.g4
 *     linear-algebra.g4
 *     optimization.g4
 *     signal-processing.g4
 *     statistics.g4
 *     symbolic.g4
 *     scientific-computing.g4
 *     control.g4
 *     classical-accelerators.g4
 *
 * Those files remain specialized feature contracts.
 *
 * This file is their DOMAIN COMPOSITION BOUNDARY.
 *
 * They MUST NOT become competing root grammars.
 *
 * The canonical parser composition layer decides which specialized rules are
 * exposed from the complete Zamani parser.
 *
 * ============================================================================
 * OPEN-WORLD OPERATION MODEL
 * ============================================================================
 *
 * Classical computation MUST NOT be implemented as an exhaustive list such
 * as:
 *
 *     add
 *     subtract
 *     fft
 *     svd
 *     cholesky
 *     gradient_descent
 *     ...
 *
 * Such an enumeration would make the language depend on today's algorithm
 * inventory.
 *
 * Classical operations therefore use canonical names and expressions.
 *
 * Operation names remain open-world semantic identifiers.
 *
 * Examples:
 *
 *     add(x, y)
 *     linalg::matmul(A, B)
 *     tensor::contract(A, B)
 *     signal::transform(signal)
 *     optimize::solve(problem)
 *     statistics::mean(values)
 *     vendor::future_operation(x)
 *
 * The parser accepts the structure.
 *
 * Semantic analysis determines whether the named operation exists, what its
 * signature means, what effects it has, what capabilities it requires, and
 * how it lowers to IR.
 *
 * ============================================================================
 * NO LIBRARY EXECUTION
 * ============================================================================
 *
 * This grammar never:
 *
 *     loads a library
 *     executes an algorithm
 *     evaluates a numerical operation
 *     discovers hardware
 *     allocates memory
 *     contacts a device
 *     contacts a network
 *     accesses the filesystem
 *     reads environment variables
 *     accesses credentials
 *
 * Operation names are syntax/data at parse time.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces no classical-specific AST hierarchy.
 *
 * Classical constructs must map into the existing domain-neutral frontend AST.
 *
 * Conceptually:
 *
 *     classicalExpression
 *             |
 *             v
 *        generic AST
 *             |
 *             v
 *     semantic classification
 *             |
 *             v
 *       classical semantic model
 *
 * A classical operation invocation must preserve:
 *
 *     operation name
 *     qualified namespace
 *     ordered operands
 *     named arguments
 *     source spans
 *     syntactic attributes available to the frontend
 *
 * The AST MUST NOT become a list of every mathematical or scientific
 * algorithm.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     scalar/vector/matrix/tensor classification
 *     numeric type compatibility
 *     symbolic semantics
 *     dimensional compatibility
 *     shape compatibility
 *     broadcasting
 *     indexing legality
 *     numerical precision
 *     overflow semantics
 *     underflow semantics
 *     numerical stability
 *     purity
 *     effects
 *     ownership
 *     borrowing
 *     resource requirements
 *     capability requirements
 *     parallelism
 *     distributed execution
 *     accelerator eligibility
 *     determinism
 *     reproducibility
 *     domain interoperability
 *
 * Parser acceptance does not imply semantic validity.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT construct IR.
 *
 * The intended pipeline is:
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
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       v
 *     classical IR / appropriate canonical IR
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     scheduling / placement
 *       |
 *       v
 *     target lowering
 *       |
 *       v
 *     runtime / hardware
 *
 * If a classical computation participates in a quantum-classical program,
 * quantum semantics continue to use the established canonical quantum::ir
 * boundary. This file MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Classical syntax does not select hardware.
 *
 * Portable source may express requirements through the canonical resource
 * and capability systems, for example:
 *
 *     requires memory >= required_memory
 *     requires capability("tensor.compute")
 *     requires capability("vector.compute")
 *     requires capability("distributed.compute")
 *     requires capability("accelerator.compute")
 *
 * The exact resource/capability syntax belongs to resources/, hardware/,
 * compile/, and the semantic layer.
 *
 * This grammar only provides the classical computation boundary.
 *
 * ============================================================================
 * PARALLELISM CONTRACT
 * ============================================================================
 *
 * Classical programs may be parallel, concurrent, distributed, vectorized,
 * pipelined, or accelerator-backed.
 *
 * This grammar MUST NOT encode a universal execution width.
 *
 * Invalid architectural direction:
 *
 *     run_on_8_threads
 *     use_32_cores
 *     use_gpu_0
 *
 * as implicit universal language constructs.
 *
 * Portable intent belongs to the concurrency/resource/capability systems.
 *
 * ============================================================================
 * NUMERICAL SCALABILITY
 * ============================================================================
 *
 * The grammar permits program-defined values and dimensions.
 *
 * It does not impose:
 *
 *     finite vector lengths
 *     finite matrix dimensions
 *     finite tensor ranks
 *     finite tensor dimensions
 *     finite numerical precision choices
 *     finite operation inventories
 *
 * Any practical implementation limitation is a compiler/runtime/resource
 * limitation and MUST NOT be presented as a language-semantic maximum.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * All constructs parsed through this grammar are required to remain
 * source-locatable through the canonical ANTLR parse tree and subsequent AST.
 *
 * No grammar action is used to manufacture source locations.
 *
 * Rust-side AST construction is responsible for preserving source spans.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics belong to the canonical parser diagnostic system.
 *
 * Semantic diagnostics belong to semantic analysis.
 *
 * Resource diagnostics belong to resource/capability analysis.
 *
 * Backend diagnostics belong to target lowering.
 *
 * This grammar MUST NOT report hardware availability as a syntax error.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token stream
 *     grammar version
 *     active language version
 *     explicitly selected dialect configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     wall-clock time
 *     randomness
 *     hardware availability
 *     filesystem state
 *     network state
 *     environment variables
 *     runtime state
 *     compiler host
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This is a declarative ANTLR parser grammar.
 *
 * It contains:
 *
 *     no embedded Rust actions
 *     no unsafe code
 *     no semantic predicates
 *     no filesystem operations
 *     no network operations
 *     no hardware operations
 *     no process execution
 *     no secret access
 *
 * The Rust implementation consuming the grammar targets:
 *
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and must remain safe Rust.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing public grammar name is retained:
 *
 *     Classical
 *
 * Existing classical entry point is retained:
 *
 *     classicalConstruct
 *
 * Existing canonical rules remain authoritative:
 *
 *     expression
 *     typeExpression
 *     statement
 *     declaration
 *     identifier
 *     qualifiedName
 *
 * No lexer vocabulary is created here.
 *
 * ============================================================================
 */

parser grammar Classical;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC CLASSICAL DOMAIN ENTRY POINT
 * ============================================================================
 *
 * This is the principal entry point for classical-domain consumers.
 *
 * It is deliberately structural rather than algorithm-specific.
 *
 * ============================================================================
 */

classicalConstruct
    : classicalExpression
    | classicalType
    | classicalInitializer
    | classicalComputationRegion
    | classicalOperationInvocation
    ;


/*
 * ============================================================================
 * GENERAL CLASSICAL EXPRESSION
 * ============================================================================
 *
 * Canonical expression syntax remains authoritative.
 *
 * This permits all existing and future Zamani expression forms without
 * requiring this grammar to be edited whenever a new expression form is
 * introduced.
 *
 * ============================================================================
 */

classicalExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL VALUE
 * ============================================================================
 *
 * Semantic-domain alias.
 *
 * No second value grammar is created.
 * ============================================================================
 */

classicalValue
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL TYPE
 * ============================================================================
 *
 * Type syntax remains owned by types/.
 *
 * Classical semantic classification is performed downstream.
 * ============================================================================
 */

classicalType
    : typeExpression
    ;


/*
 * ============================================================================
 * CLASSICAL INITIALIZER
 * ============================================================================
 *
 * This is a computation-domain initializer boundary, not a declaration
 * grammar.
 *
 * Declarations remain owned by declarations/.
 *
 * Examples structurally supported:
 *
 *     = expression
 *     : Type = expression
 *
 * ============================================================================
 */

classicalInitializer
    : classicalTypeAnnotation?
      ASSIGN
      expression
    ;


classicalTypeAnnotation
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * CLASSICAL COMPUTATION REGION
 * ============================================================================
 *
 * A classical region is a block of canonical statements.
 *
 * No statement grammar is duplicated.
 *
 * ============================================================================
 */

classicalComputationRegion
    : LBRACE
      classicalStatementSequence
      RBRACE
    ;


classicalStatementSequence
    : statement*
    ;


/*
 * ============================================================================
 * CLASSICAL OPERATION INVOCATION
 * ============================================================================
 *
 * Open-world operation model.
 *
 * The operation name is semantic data rather than a finite grammar
 * enumeration.
 *
 * Examples:
 *
 *     add(x, y)
 *     linalg::matmul(A, B)
 *     tensor::contract(A, B)
 *     statistics::mean(values)
 *     custom::future_operation(x)
 *
 * Existing canonical expression/call syntax remains authoritative for
 * ordinary source expressions. This rule is an explicit classical-domain
 * boundary for tools that need to recognize operation-shaped constructs.
 *
 * ============================================================================
 */

classicalOperationInvocation
    : qualifiedName
      LPAREN
      classicalOperationArgumentList?
      RPAREN
    ;


classicalOperationArgumentList
    : classicalOperationArgument
      (COMMA classicalOperationArgument)*
      COMMA?
    ;


classicalOperationArgument
    : classicalNamedArgument
    | expression
    ;


classicalNamedArgument
    : identifier
      COLON
      expression
    ;


/*
 * ============================================================================
 * CLASSICAL NUMERICAL BOUNDARY
 * ============================================================================
 *
 * Numerical classification is semantic.
 * ============================================================================
 */

classicalNumericalExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL SYMBOLIC BOUNDARY
 * ============================================================================
 */

classicalSymbolicExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL COLLECTION BOUNDARY
 * ============================================================================
 *
 * Covers semantic values such as:
 *
 *     vectors
 *     matrices
 *     tensors
 *     arrays
 *     sequences
 *     maps
 *     user-defined collections
 *
 * Their actual syntax belongs to expressions/types and the existing
 * specialized classical grammars.
 * ============================================================================
 */

classicalCollectionExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL SCALAR BOUNDARY
 * ============================================================================
 */

classicalScalarExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL INTEGER BOUNDARY
 * ============================================================================
 */

classicalIntegerExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL FLOATING-POINT BOUNDARY
 * ============================================================================
 */

classicalFloatingPointExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL VECTOR BOUNDARY
 * ============================================================================
 */

classicalVectorExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL MATRIX BOUNDARY
 * ============================================================================
 */

classicalMatrixExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL TENSOR BOUNDARY
 * ============================================================================
 */

classicalTensorExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL LINEAR-ALGEBRA BOUNDARY
 * ============================================================================
 */

classicalLinearAlgebraExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL NUMERIC BOUNDARY
 * ============================================================================
 */

classicalNumericExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL NUMERICAL-METHOD BOUNDARY
 * ============================================================================
 */

classicalNumericalMethodExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL STATISTICS BOUNDARY
 * ============================================================================
 */

classicalStatisticsExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL SIGNAL-PROCESSING BOUNDARY
 * ============================================================================
 */

classicalSignalProcessingExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL OPTIMIZATION BOUNDARY
 * ============================================================================
 */

classicalOptimizationExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL SCIENTIFIC-COMPUTING BOUNDARY
 * ============================================================================
 */

classicalScientificComputingExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL CONTROL-COMPUTATION BOUNDARY
 * ============================================================================
 */

classicalControlExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL ACCELERATOR BOUNDARY
 * ============================================================================
 *
 * This identifies an accelerator-oriented semantic boundary without selecting
 * a particular physical accelerator.
 * ============================================================================
 */

classicalAcceleratorExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL PURE COMPUTATION BOUNDARY
 * ============================================================================
 *
 * Purity is determined by effects/semantic analysis.
 * ============================================================================
 */

classicalPureExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL EFFECTFUL COMPUTATION BOUNDARY
 * ============================================================================
 */

classicalEffectfulExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL COMPILE-TIME BOUNDARY
 * ============================================================================
 *
 * Compile-time evaluability is a compiler/semantic property.
 * ============================================================================
 */

classicalCompileTimeExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL CONSTANT-EVALUATION BOUNDARY
 * ============================================================================
 */

classicalConstantExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL MEMORY BOUNDARY
 * ============================================================================
 *
 * Ownership, borrowing, allocation, lifetime, persistence and address-space
 * semantics remain owned by memory/ and semantic analysis.
 * ============================================================================
 */

classicalMemoryExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL PARALLELISM BOUNDARY
 * ============================================================================
 *
 * No fixed number of workers is encoded.
 * ============================================================================
 */

classicalParallelExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL DISTRIBUTED-COMPUTATION BOUNDARY
 * ============================================================================
 *
 * No fixed node count or topology is encoded.
 * ============================================================================
 */

classicalDistributedExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL DATA BOUNDARY
 * ============================================================================
 */

classicalDataExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL AI/ML BOUNDARY
 * ============================================================================
 */

classicalAIExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL NETWORKING BOUNDARY
 * ============================================================================
 */

classicalNetworkExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL SECURITY BOUNDARY
 * ============================================================================
 */

classicalSecurityExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL RESOURCE/CAPABILITY BOUNDARY
 * ============================================================================
 *
 * This does not define resource syntax. The canonical resources/hardware
 * grammars remain authoritative.
 * ============================================================================
 */

classicalResourceExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL PORTABILITY BOUNDARY
 * ============================================================================
 */

classicalPortableExpression
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL QUANTUM-INTERACTION BOUNDARY
 * ============================================================================
 *
 * Classical values may participate in hybrid quantum-classical computation.
 *
 * This grammar does not define:
 *
 *     qubits
 *     gates
 *     quantum states
 *     physical qubits
 *     topology
 *     QEC
 *     ZQN
 *
 * Those remain owned by the quantum subsystem.
 * ============================================================================
 */

classicalQuantumInteraction
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL HARDWARE-INTERACTION BOUNDARY
 * ============================================================================
 */

classicalHardwareInteraction
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL HDL-INTERACTION BOUNDARY
 * ============================================================================
 */

classicalHDLInteraction
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL RESULT BOUNDARY
 * ============================================================================
 */

classicalResult
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL FAILURE/ERROR VALUE BOUNDARY
 * ============================================================================
 *
 * Error semantics remain owned by the canonical result/effect/error systems.
 * ============================================================================
 */

classicalFailureValue
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL CONSTRAINT VALUE BOUNDARY
 * ============================================================================
 */

classicalConstraintValue
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL CAPABILITY VALUE BOUNDARY
 * ============================================================================
 */

classicalCapabilityValue
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL PORTABILITY VALUE BOUNDARY
 * ============================================================================
 */

classicalPortabilityValue
    : expression
    ;


/*
 * ============================================================================
 * CLASSICAL DOMAIN IDENTIFIER
 * ============================================================================
 *
 * Open-world semantic identifier.
 *
 * It is deliberately NOT:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator-0
 *     core-0
 *
 * as a parser-level machine enumeration.
 * ============================================================================
 */

classicalDomainIdentifier
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * CLASSICAL SEMANTIC ANCHOR
 * ============================================================================
 *
 * Stable hook for semantic tooling.
 * ============================================================================
 */

classicalSemanticAnchor
    : classicalConstruct
    ;


/*
 * ============================================================================
 * CLASSICAL PROGRAM FRAGMENT
 * ============================================================================
 *
 * Arbitrarily many classical constructs are structurally accepted.
 * ============================================================================
 */

classicalProgramFragment
    : classicalConstruct*
    ;


/*
 * ============================================================================
 * CLASSICAL BLOCK CONTENT
 * ============================================================================
 */

classicalBlockContent
    : statement*
    ;


/*
 * ============================================================================
 * CLASSICAL VALIDATION ENTRY POINT
 * ============================================================================
 *
 * Tooling may use this rule to validate a single classical construct.
 *
 * EOF is optional so the rule can also be embedded into larger parser
 * compositions.
 * ============================================================================
 */

classicalValidationInput
    : classicalConstruct EOF?
    ;


/*
 * ============================================================================
 * CLASSICAL DOMAIN CONTRACT SUMMARY
 * ============================================================================
 *
 * This grammar owns:
 *
 *     - classical-domain composition;
 *     - stable classical semantic boundaries;
 *     - open-world classical operation invocation;
 *     - classical computation regions;
 *     - integration hooks for existing classical feature grammars.
 *
 * This grammar does NOT own:
 *
 *     - lexical rules;
 *     - token definitions;
 *     - general expressions;
 *     - operators;
 *     - general calls;
 *     - indexing;
 *     - slicing;
 *     - declarations;
 *     - functions;
 *     - modules;
 *     - effects;
 *     - memory semantics;
 *     - concurrency semantics;
 *     - resource semantics;
 *     - hardware selection;
 *     - target selection;
 *     - numerical execution;
 *     - mathematical library implementation;
 *     - classical IR;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - scheduling;
 *     - routing;
 *     - calibration;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden universal capacity constants:
 *
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ACCELERATORS
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_LENGTH
 *     MAX_MATRIX_ROWS
 *     MAX_MATRIX_COLUMNS
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSIONS
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * None are used as grammar limits.
 *
 * No physical device identifiers are encoded.
 *
 * No vendor-specific algorithm inventory is encoded.
 *
 * No finite operation inventory is encoded.
 *
 * No finite vector/matrix/tensor size is encoded.
 *
 * ============================================================================
 * ANTLR / RUST CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     - is parser-only;
 *     - uses the canonical ZamaniLexer vocabulary;
 *     - contains no lexer rules;
 *     - contains no embedded target-language actions;
 *     - contains no unsafe code;
 *     - contains no semantic predicates;
 *     - does not require unsafe Rust;
 *     - is intended for the repository's Rust 2021 frontend;
 *     - remains compatible with the requested Rust 1.97 / 1.97.1 baseline.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The following classes of tests must exist in grammar/tests/classical/.
 *
 * POSITIVE
 *
 *     scalar expressions
 *     integer expressions
 *     floating-point expressions
 *     vector expressions
 *     matrix expressions
 *     tensor expressions
 *     symbolic expressions
 *     numerical expressions
 *     statistical expressions
 *     signal-processing expressions
 *     optimization expressions
 *     scientific-computing expressions
 *     control computations
 *     generic function calls
 *     qualified classical operations
 *     named operation arguments
 *     classical computation blocks
 *
 * Examples:
 *
 *     x + y
 *     linalg::matmul(A, B)
 *     tensor::contract(A, B)
 *     statistics::mean(values)
 *     optimize::solve(problem)
 *     custom::future_operation(x)
 *
 * NEGATIVE / SEMANTIC
 *
 * Syntactically valid but semantically invalid examples must be rejected by
 * semantic analysis rather than this parser, for example:
 *
 *     linalg::matmul(scalar, incompatible_value)
 *
 *     tensor::contract(incompatible_a, incompatible_b)
 *
 *     statistics::mean(non_numeric_value)
 *
 *     optimize::solve(invalid_problem)
 *
 * BOUNDARY
 *
 *     empty classical region
 *     deeply nested expressions
 *     large expression trees
 *     many operation arguments
 *     many chained operations
 *     symbolic dimensions
 *     generated large vectors
 *     generated large matrices
 *     generated high-rank tensors
 *
 * SCALABILITY
 *
 * Verify that parsing imposes no artificial:
 *
 *     vector-length limit
 *     matrix-size limit
 *     tensor-rank limit
 *     tensor-dimension limit
 *     argument-count limit
 *     operation-count limit
 *     function-count limit
 *     worker-count limit
 *     device-count limit
 *     node-count limit
 *
 * CROSS-DOMAIN
 *
 * Test interaction with:
 *
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     resources
 *     concurrency
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     interoperability
 *
 * DETERMINISM
 *
 * Identical source/token streams under identical grammar/language versions
 * must produce equivalent parse structures.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * classical.g4 is complete when:
 *
 *     [ ] It remains the sole classical-domain composition boundary.
 *     [ ] It is imported through the canonical ZamaniParser hierarchy.
 *     [ ] It uses the canonical ZamaniLexer vocabulary.
 *     [ ] It contains no lexer rules.
 *     [ ] It contains no competing expression grammar.
 *     [ ] It contains no competing type grammar.
 *     [ ] It contains no competing declaration grammar.
 *     [ ] It contains no competing statement grammar.
 *     [ ] It contains no finite algorithm inventory.
 *     [ ] It contains no hardware inventory.
 *     [ ] It contains no universal resource limits.
 *     [ ] It contains no target-specific identifiers.
 *     [ ] Existing scalar/vector/matrix/tensor/etc. grammar ownership remains
 *         in their existing files.
 *     [ ] Classical operations remain open-world.
 *     [ ] AST mapping remains domain-neutral.
 *     [ ] Semantic validation remains downstream.
 *     [ ] Classical IR remains downstream.
 *     [ ] quantum::ir remains the canonical quantum boundary.
 *     [ ] QEC remains downstream.
 *     [ ] ZQN remains downstream.
 *     [ ] Routing remains downstream.
 *     [ ] Scheduling remains downstream.
 *     [ ] HAL remains downstream.
 *     [ ] Runtime execution remains downstream.
 *     [ ] Positive tests exist.
 *     [ ] Negative tests exist.
 *     [ ] Boundary tests exist.
 *     [ ] Scalability tests exist.
 *     [ ] Determinism tests exist.
 *     [ ] Cross-domain tests exist.
 *     [ ] Rust integration requires no unsafe code.
 *
 * ============================================================================
 */