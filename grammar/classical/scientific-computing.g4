/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/scientific-computing.g4
 *
 * Status:
 *     PRODUCTION-READY SCIENTIFIC-COMPUTING DOMAIN BOUNDARY
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
 *     - No target-specific parser code.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime evaluation.
 *     - No unsafe Rust requirement.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the canonical SOURCE-SYNTAX INTEGRATION BOUNDARY for
 * scientific computing in Zamani.
 *
 * Scientific computing is a semantic domain of the single Zamani language.
 * It is NOT a separate language and MUST NOT create:
 *
 *     - a second expression grammar;
 *     - a second type system;
 *     - a second declaration system;
 *     - a scientific-specific parser hierarchy;
 *     - a scientific-specific AST;
 *     - a scientific-specific IR;
 *     - target-specific syntax;
 *     - a vendor-specific mathematical API;
 *     - artificial hardware limits.
 *
 * The scientific-computing domain covers computation such as:
 *
 *     - numerical simulation;
 *     - numerical analysis;
 *     - differential equations;
 *     - algebraic equations;
 *     - differential-algebraic equations;
 *     - discretization;
 *     - interpolation;
 *     - approximation;
 *     - integration;
 *     - differentiation;
 *     - optimization;
 *     - statistics;
 *     - probability;
 *     - stochastic computation;
 *     - signal and system computation;
 *     - scientific data transformation;
 *     - physical and mathematical modeling;
 *     - parameter studies;
 *     - uncertainty analysis;
 *     - sensitivity analysis;
 *     - scientific workflows;
 *     - reproducible computation;
 *     - simulation;
 *     - inverse problems;
 *     - forward models;
 *     - surrogate models;
 *     - numerical linear algebra;
 *     - tensor/scientific computation;
 *     - distributed scientific computation;
 *     - accelerator-oriented scientific computation;
 *     - quantum/classical scientific computation;
 *     - hardware/software co-design.
 *
 * The grammar deliberately does NOT enumerate these algorithms as keywords.
 *
 * For example:
 *
 *     solve(...)
 *     integrate(...)
 *     differentiate(...)
 *     interpolate(...)
 *     simulate(...)
 *     optimize(...)
 *     fft(...)
 *     eig(...)
 *     svd(...)
 *     monte_carlo(...)
 *
 * are normally ordinary Zamani expressions/calls.
 *
 * Their scientific meaning is determined by:
 *
 *     type analysis;
 *     operation resolution;
 *     semantic capabilities;
 *     library/intrinsic resolution;
 *     domain analysis;
 *     resource analysis;
 *     compiler lowering.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                         ZamaniParser
 *                              |
 *                              v
 *                  Domain-neutral frontend AST
 *                              |
 *                              v
 *                    Structural analysis
 *                              |
 *                              v
 *                     Semantic analysis
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *           Types           Effects         Resources
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                  Scientific semantic model
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *       Classical IR      Data/Tensor IR   Domain metadata
 *             |
 *             v
 *         Optimization
 *             |
 *             v
 *       Scheduling/lowering
 *             |
 *             +-------------------------------+
 *             |               |               |
 *             v               v               v
 *            CPU             GPU             FPGA
 *             |               |               |
 *             +---------------+---------------+
 *                             |
 *                             v
 *                     Other/future targets
 *
 * The scientific-computing grammar is strictly upstream of IR generation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - scientific-computing domain classification boundaries;
 *     - scientific-computing expression boundaries;
 *     - scientific-computing type classification boundaries;
 *     - scientific operation classification boundaries;
 *     - scientific model boundaries;
 *     - scientific simulation boundaries;
 *     - scientific numerical-method boundaries;
 *     - scientific constraint boundaries;
 *     - scientific resource/capability integration points;
 *     - scientific portability integration points;
 *     - scientific domain validation entry points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical syntax;
 *     - identifiers;
 *     - keywords;
 *     - numeric literal spelling;
 *     - operators;
 *     - precedence;
 *     - function calls;
 *     - indexing;
 *     - member access;
 *     - general declarations;
 *     - general statements;
 *     - general types;
 *     - matrix syntax;
 *     - vector syntax;
 *     - tensor syntax;
 *     - numerical literal semantics;
 *     - floating-point semantics;
 *     - integer semantics;
 *     - symbolic mathematics implementation;
 *     - linear algebra implementation;
 *     - optimization implementation;
 *     - solver implementation;
 *     - simulation implementation;
 *     - scientific library implementation;
 *     - BLAS/LAPACK/provider selection;
 *     - GPU selection;
 *     - FPGA selection;
 *     - CPU selection;
 *     - accelerator selection;
 *     - distributed placement;
 *     - scheduling;
 *     - routing;
 *     - runtime execution;
 *     - hardware discovery;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL.
 *
 * ============================================================================
 * RELATIONSHIP TO EXISTING CLASSICAL GRAMMARS
 * ============================================================================
 *
 * Scientific computing is intentionally composed over existing grammar
 * boundaries.
 *
 * Relevant existing contracts include:
 *
 *     grammar/classical/classical.g4
 *     grammar/classical/numeric.g4
 *     grammar/classical/numerical.g4
 *     grammar/classical/scalar.g4
 *     grammar/classical/vector.g4
 *     grammar/classical/matrix.g4
 *     grammar/classical/tensor.g4
 *     grammar/classical/linear-algebra.g4
 *     grammar/classical/symbolic.g4
 *     grammar/classical/statistics.g4
 *     grammar/classical/optimization.g4
 *     grammar/classical/signal-processing.g4
 *
 * This file does NOT replace those files.
 *
 * In particular:
 *
 *     scalar semantics
 *         -> scalar.g4
 *
 *     vectors
 *         -> vector.g4
 *
 *     matrices
 *         -> matrix.g4
 *
 *     tensors
 *         -> tensor.g4
 *
 *     linear algebra
 *         -> linear-algebra.g4
 *
 *     symbolic computation
 *         -> symbolic.g4
 *
 *     numerical computation
 *         -> numerical.g4 / numeric.g4
 *
 *     statistics
 *         -> statistics.g4
 *
 *     optimization
 *         -> optimization.g4
 *
 *     signal processing
 *         -> signal-processing.g4
 *
 * Scientific computing composes their semantics; it does not redefine them.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Scientific source describes:
 *
 *     WHAT is being computed
 *     WHAT mathematical/scientific properties are required
 *     WHAT numerical guarantees are required
 *     WHAT resources/capabilities are required
 *     WHAT constraints/preferences apply
 *
 * It does NOT prescribe:
 *
 *     WHICH CPU
 *     WHICH GPU
 *     WHICH FPGA
 *     WHICH accelerator
 *     WHICH memory bank
 *     WHICH node
 *     WHICH register
 *     WHICH vector lane
 *     WHICH physical device
 *     WHICH machine topology
 *
 * This separation is required for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * ABSOLUTE SCALABILITY RULE
 * ============================================================================
 *
 * This grammar contains NO artificial finite capacity.
 *
 * It MUST NOT encode:
 *
 *     MAX_ELEMENTS
 *     MAX_VECTOR_LENGTH
 *     MAX_MATRIX_ROWS
 *     MAX_MATRIX_COLUMNS
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ACCELERATORS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_SIMD_WIDTH
 *     MAX_DEVICES
 *     MAX_WORKERS
 *     MAX_OPERATIONS
 *     MAX_ITERATIONS
 *     MAX_TIMELINES
 *
 * It MUST NOT encode equivalent limits under another name.
 *
 * Repetition therefore remains structural:
 *
 *     item*
 *     item+
 *     item?
 *
 * rather than arbitrary grammar limits such as:
 *
 *     item{1,32}
 *
 * unless a finite cardinality is an actual part of the language semantics.
 *
 * ============================================================================
 * PROGRAM VALUE VS IMPLEMENTATION LIMIT
 * ============================================================================
 *
 * A value written by the programmer is valid program semantics.
 *
 * For example:
 *
 *     let n = 1000000;
 *
 * or:
 *
 *     let samples = expression;
 *
 * does not create a grammar-level resource limit.
 *
 * Likewise:
 *
 *     matrix(rows, columns)
 *
 * does not imply a maximum number of rows or columns.
 *
 * The grammar must distinguish:
 *
 *     program-defined value
 *
 * from:
 *
 *     implementation-defined capacity.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical lexical vocabulary is supplied by:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file MUST NOT declare lexer rules.
 *
 * Scientific algorithm names remain ordinary identifiers.
 *
 * Therefore names such as:
 *
 *     simulate
 *     solve
 *     integrate
 *     differentiate
 *     interpolate
 *     approximate
 *     optimize
 *     sample
 *     propagate
 *     calibrate
 *     analyze
 *     model
 *     residual
 *     sensitivity
 *     uncertainty
 *
 * are NOT hard-coded keywords by this grammar.
 *
 * This permits:
 *
 *     standard libraries;
 *     user libraries;
 *     dialects;
 *     compiler intrinsics;
 *     scientific frameworks;
 *     future algorithms;
 *     vendor-independent implementations.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expression syntax is owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * There is exactly one authoritative public expression rule:
 *
 *     expression
 *
 * Scientific expressions therefore use:
 *
 *     expression
 *
 * rather than defining:
 *
 *     scientificExpression
 *
 * as an independent precedence hierarchy.
 *
 * This file's `scientificComputingExpression` is a DOMAIN CLASSIFICATION
 * boundary only.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Type syntax is owned by:
 *
 *     grammar/types/
 *
 * Scientific computing may consume semantic types including:
 *
 *     scalar types
 *     numeric types
 *     vector types
 *     matrix types
 *     tensor types
 *     array types
 *     function types
 *     symbolic types
 *     data types
 *     physical quantity types
 *     unit-aware types
 *     resource-aware types
 *     probabilistic types
 *
 * This file MUST NOT redefine those type grammars.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar does not create a scientific-specific AST.
 *
 * Canonical mapping:
 *
 *     expression
 *         |
 *         v
 *     domain-neutral frontend AST
 *         |
 *         v
 *     semantic classification
 *         |
 *         v
 *     scientific semantic construct
 *
 * A scientific computation must therefore remain representable by the existing
 * domain-neutral frontend AST.
 *
 * No `ScientificExpression` AST type is required merely because an expression
 * belongs to this domain.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Scientific semantic analysis is responsible for determining:
 *
 *     - whether an expression is scientifically meaningful;
 *     - numeric domains;
 *     - units;
 *     - dimensions;
 *     - shapes;
 *     - physical quantities;
 *     - mathematical properties;
 *     - numerical method;
 *     - approximation requirements;
 *     - precision requirements;
 *     - stability requirements;
 *     - convergence requirements;
 *     - error bounds;
 *     - tolerances;
 *     - solver requirements;
 *     - stochastic properties;
 *     - deterministic properties;
 *     - reproducibility requirements;
 *     - resource requirements;
 *     - capability requirements;
 *     - portability constraints.
 *
 * None of these semantic decisions are encoded as parser-time hardware
 * decisions.
 *
 * ============================================================================
 * SCIENTIFIC OPERATION MODEL
 * ============================================================================
 *
 * A scientific operation is normally an ordinary Zamani expression.
 *
 * Examples:
 *
 *     solve(A, b)
 *     integrate(f, domain)
 *     simulate(model, parameters)
 *     optimize(objective, constraints)
 *     fft(signal)
 *     interpolate(data, points)
 *
 * The operation name is resolved semantically.
 *
 * This allows new scientific algorithms to be introduced without modifying
 * this grammar.
 *
 * Therefore adding:
 *
 *     new_solver(...)
 *
 * does NOT require a grammar update when it is expressible through canonical
 * function/application syntax.
 *
 * ============================================================================
 * SCIENTIFIC MODEL CONTRACT
 * ============================================================================
 *
 * A scientific model is represented by canonical Zamani syntax.
 *
 * The model may contain:
 *
 *     variables
 *     parameters
 *     equations
 *     functions
 *     constraints
 *     data
 *     tensors
 *     fields
 *     boundary conditions
 *     initial conditions
 *     observations
 *     objectives
 *     stochastic quantities
 *
 * The grammar provides a stable semantic boundary without prescribing how a
 * model is implemented.
 *
 * ============================================================================
 * SIMULATION CONTRACT
 * ============================================================================
 *
 * Simulation is a semantic concept.
 *
 * The grammar does not prescribe:
 *
 *     timestep implementation;
 *     numerical solver;
 *     integration algorithm;
 *     mesh representation;
 *     processor;
 *     accelerator;
 *     distributed topology;
 *     memory layout.
 *
 * A simulation expression remains an ordinary canonical expression.
 *
 * ============================================================================
 * NUMERICAL METHOD CONTRACT
 * ============================================================================
 *
 * Numerical methods are semantic/library/intrinsic constructs.
 *
 * The grammar does not enumerate:
 *
 *     Euler
 *     RungeKutta
 *     AdamsBashforth
 *     Newton
 *     GMRES
 *     CG
 *     BiCG
 *     MonteCarlo
 *     finite_difference
 *     finite_element
 *     finite_volume
 *
 * as universal parser keywords.
 *
 * If the language eventually gives one of these concepts special syntax,
 * that syntax must first obtain:
 *
 *     specification
 *     AST contract
 *     semantic contract
 *     IR contract
 *     compatibility contract
 *     tests
 *
 * before becoming stable language syntax.
 *
 * ============================================================================
 * UNITS AND PHYSICAL QUANTITIES
 * ============================================================================
 *
 * Scientific computing may use units and physical quantities.
 *
 * Their semantic representation belongs to the canonical type/semantic system.
 *
 * This grammar therefore does not introduce a second unit syntax.
 *
 * Expressions such as:
 *
 *     quantity
 *     convert(value, unit)
 *     normalize(value)
 *
 * remain canonical expressions unless the language specification explicitly
 * standardizes dedicated unit syntax.
 *
 * ============================================================================
 * DIMENSIONS AND SHAPES
 * ============================================================================
 *
 * Scientific dimensions and shapes are semantic values.
 *
 * They may be:
 *
 *     constants
 *     symbolic expressions
 *     generic parameters
 *     runtime-derived values
 *     dependent values
 *
 * The grammar imposes no finite dimension ceiling.
 *
 * Scientific dimensions must not be confused with:
 *
 *     machine word width
 *     SIMD width
 *     register width
 *     physical memory capacity
 *     device capacity.
 *
 * ============================================================================
 * PRECISION CONTRACT
 * ============================================================================
 *
 * Precision is semantic/type information.
 *
 * This grammar does not assume:
 *
 *     32-bit floating point
 *     64-bit floating point
 *     128-bit floating point
 *
 * as universal language limits.
 *
 * A particular numeric representation may be explicitly selected by a Zamani
 * type, profile, capability, or implementation contract where standardized.
 *
 * Target availability is resolved downstream.
 *
 * ============================================================================
 * REPRODUCIBILITY CONTRACT
 * ============================================================================
 *
 * Scientific programs may require reproducibility.
 *
 * The grammar provides an integration boundary for semantic constructs such as:
 *
 *     reproducibility requirements
 *     deterministic execution requirements
 *     provenance requirements
 *     numerical tolerance requirements
 *     algorithm/version constraints
 *
 * These are semantic requirements, not parser implementation behavior.
 *
 * ============================================================================
 * UNCERTAINTY CONTRACT
 * ============================================================================
 *
 * Scientific computation may represent uncertainty through canonical types,
 * expressions, probability models, or library operations.
 *
 * This file does not create a second probabilistic grammar.
 *
 * Semantic analysis determines whether an expression represents:
 *
 *     deterministic data
 *     random variables
 *     distributions
 *     uncertainty
 *     intervals
 *     confidence information
 *     stochastic processes
 *     samples
 *
 * ============================================================================
 * DIFFERENTIAL-EQUATION CONTRACT
 * ============================================================================
 *
 * Differential equations, algebraic equations, boundary conditions, and
 * initial conditions remain ordinary expressions and declarations.
 *
 * The grammar does not hard-code a finite number of:
 *
 *     variables
 *     equations
 *     dimensions
 *     boundary conditions
 *     domains
 *     time steps.
 *
 * ============================================================================
 * DATA CONTRACT
 * ============================================================================
 *
 * Scientific data may participate in:
 *
 *     arrays
 *     vectors
 *     matrices
 *     tensors
 *     streams
 *     records
 *     datasets
 *     distributed collections
 *
 * Existing data/container grammars remain authoritative.
 *
 * Scientific computing only provides the semantic domain boundary.
 *
 * ============================================================================
 * LINEAR-ALGEBRA CONTRACT
 * ============================================================================
 *
 * Linear algebra remains owned by:
 *
 *     grammar/classical/linear-algebra.g4
 *
 * This file may classify an expression as scientific and linear algebra
 * simultaneously.
 *
 * It must NOT redefine:
 *
 *     matrix syntax
 *     vector syntax
 *     tensor syntax
 *     indexing
 *     decomposition syntax
 *     solver syntax
 *
 * The semantic model may combine these domains.
 *
 * ============================================================================
 * NUMERICAL CONTRACT
 * ============================================================================
 *
 * Numerical syntax remains integrated with:
 *
 *     grammar/classical/numeric.g4
 *     grammar/classical/numerical.g4
 *
 * This file does not replace either existing numerical contract.
 *
 * The scientific domain is a higher-level semantic classification over
 * numerical computation.
 *
 * ============================================================================
 * SYMBOLIC CONTRACT
 * ============================================================================
 *
 * Symbolic computation remains integrated with:
 *
 *     grammar/classical/symbolic.g4
 *
 * Scientific expressions may therefore be symbolic, numeric, or hybrid.
 *
 * The parser does not choose between these interpretations.
 *
 * ============================================================================
 * OPTIMIZATION CONTRACT
 * ============================================================================
 *
 * Optimization semantics remain integrated with:
 *
 *     grammar/classical/optimization.g4
 *
 * Scientific optimization may use:
 *
 *     objective functions
 *     constraints
 *     gradients
 *     Hessians
 *     parameter spaces
 *     feasible regions
 *     stochastic objectives
 *
 * but the grammar does not enumerate optimization algorithms.
 *
 * ============================================================================
 * SIGNAL-PROCESSING CONTRACT
 * ============================================================================
 *
 * Signal-processing semantics remain integrated with:
 *
 *     grammar/classical/signal-processing.g4
 *
 * Scientific signal processing may therefore participate in:
 *
 *     numerical computation
 *     tensor computation
 *     statistics
 *     optimization
 *     control
 *     distributed computation
 *     AI/data processing.
 *
 * ============================================================================
 * STATISTICS CONTRACT
 * ============================================================================
 *
 * Statistical semantics remain integrated with:
 *
 *     grammar/classical/statistics.g4
 *
 * This grammar does not redefine:
 *
 *     mean
 *     variance
 *     covariance
 *     probability
 *     distributions
 *     sampling
 *     inference
 *
 * as parser keywords.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Scientific programs may express resource intent through the canonical
 * resource/capability system.
 *
 * Examples include concepts equivalent to:
 *
 *     requires memory >= required_memory
 *     requires capability("scientific.compute")
 *     requires capability("tensor.compute")
 *     requires capability("distributed.compute")
 *     requires capability("accelerated.compute")
 *
 * The exact resource syntax is owned by:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/compile/
 *     grammar/execution/
 *
 * This file only provides scientific semantic classification boundaries.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE SEPARATION
 * ============================================================================
 *
 * Scientific computing must distinguish:
 *
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *     implementation decision
 *
 * For example:
 *
 *     requires capability("distributed.compute")
 *
 * is different from:
 *
 *     prefer accelerator("gpu")
 *
 * which is different from:
 *
 *     target specific_device(...)
 *
 * The latter is target realization and does not belong to the portable
 * scientific grammar.
 *
 * ============================================================================
 * CLASSICAL / QUANTUM INTEGRATION
 * ============================================================================
 *
 * Scientific expressions may participate in hybrid programs.
 *
 * For example:
 *
 *     classical scientific model
 *         |
 *         v
 *     parameter computation
 *         |
 *         v
 *     quantum operation
 *         |
 *         v
 *     measurement
 *         |
 *         v
 *     classical scientific analysis
 *
 * This grammar does not introduce quantum syntax.
 *
 * Quantum syntax remains owned by:
 *
 *     grammar/quantum/
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Scientific computation may provide parameters or algorithms used by HDL or
 * hardware-oriented programs.
 *
 * This grammar does not define:
 *
 *     registers
 *     wires
 *     buses
 *     clocks
 *     physical addresses
 *     FPGA resources
 *     ASIC resources
 *     device topology.
 *
 * Those remain downstream or under:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/resources/
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Scientific computations may be distributed without source-level dependence
 * on a fixed number of nodes.
 *
 * The grammar does not encode:
 *
 *     node 0
 *     node 1
 *     fixed node counts
 *     fixed shard counts
 *     fixed worker counts
 *
 * Distribution semantics remain owned by:
 *
 *     grammar/distributed/
 *     grammar/resources/
 *     grammar/execution/
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Scientific computation may participate in:
 *
 *     AI
 *     machine learning
 *     data processing
 *     simulation
 *     optimization
 *     inference
 *     scientific discovery.
 *
 * AI/data grammar remains independently owned.
 *
 * Scientific computing supplies semantic interoperability rather than another
 * tensor/model language.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Scientific programs may interoperate with:
 *
 *     foreign functions
 *     scientific libraries
 *     numerical providers
 *     external data formats
 *     simulation systems
 *     accelerator APIs
 *     future scientific systems.
 *
 * Interoperability syntax belongs to:
 *
 *     grammar/interoperability/
 *
 * This grammar must remain provider-neutral.
 *
 * ============================================================================
 * DIALECT CONTRACT
 * ============================================================================
 *
 * Scientific domains may be extended through the canonical dialect mechanism.
 *
 * A dialect may introduce:
 *
 *     new operation names
 *     new semantic capabilities
 *     new domain metadata
 *     new library/intrinsic mappings
 *
 * without changing the base scientific grammar when ordinary expressions are
 * sufficient.
 *
 * A dialect MUST NOT silently create a second scientific language.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should report structural errors only.
 *
 * Examples:
 *
 *     malformed expression
 *     malformed type expression
 *     invalid token sequence
 *     incomplete delimiter sequence
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples:
 *
 *     incompatible scientific types
 *     invalid physical dimensions
 *     incompatible units
 *     impossible numerical requirement
 *     unsupported solver
 *     insufficient target capability
 *     unsatisfied resource requirement
 *     unsupported precision
 *
 * These MUST NOT be converted into arbitrary grammar-level restrictions.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     grammar version
 *     lexical vocabulary
 *     parser composition
 *     explicitly selected dialect configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     memory availability
 *     filesystem state
 *     network state
 *     wall-clock time
 *     randomness
 *     target availability.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no evaluation;
 *     - performs no I/O;
 *     - performs no network operations;
 *     - performs no hardware discovery;
 *     - performs no resource allocation;
 *     - performs no code execution;
 *     - contains no embedded Rust;
 *     - requires no unsafe Rust.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * The grammar deliberately delegates expression parsing to the canonical
 * expression hierarchy.
 *
 * This prevents the scientific domain from creating a second precedence
 * hierarchy or parser implementation.
 *
 * Repetition remains unbounded at the language level.
 *
 * Practical compiler resource limits remain implementation/environment
 * constraints and MUST NOT become scientific language semantics.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * The grammar imports only the canonical expression and type infrastructures.
 *
 * It deliberately does not import:
 *
 *     Matrix
 *     Vector
 *     Tensor
 *     LinearAlgebra
 *     Numerical
 *     Numeric
 *     Symbolic
 *     Optimization
 *     Statistics
 *     SignalProcessing
 *
 * merely to classify ordinary expressions.
 *
 * This prevents competing parse paths.
 *
 * Those domains remain independently owned and are combined semantically.
 *
 * ============================================================================
 */

parser grammar ScientificComputing;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions, Types;


/*
 * ============================================================================
 * 1. PUBLIC SCIENTIFIC-COMPUTING ENTRY POINT
 * ============================================================================
 *
 * This is a semantic-domain entry point.
 *
 * It is NOT an alternative universal expression hierarchy.
 *
 * ============================================================================
 */

scientificComputingConstruct
    : scientificComputingExpression
    | scientificComputingType
    | scientificComputingModel
    | scientificComputingSimulation
    | scientificComputingMethod
    | scientificComputingConstraint
    | scientificComputingRequirement
    ;


/*
 * ============================================================================
 * 2. SCIENTIFIC EXPRESSION
 * ============================================================================
 *
 * All concrete expression syntax is inherited from the canonical expression
 * grammar.
 *
 * ============================================================================
 */

scientificComputingExpression
    : expression
    ;


/*
 * ============================================================================
 * 3. SCIENTIFIC TYPE
 * ============================================================================
 *
 * Scientific type classification is semantic.
 *
 * Concrete type syntax remains owned by Types.
 *
 * ============================================================================
 */

scientificComputingType
    : typeExpression
    ;


/*
 * ============================================================================
 * 4. SCIENTIFIC VALUE
 * ============================================================================
 *
 * A scientific value is represented by the canonical expression system.
 *
 * ============================================================================
 */

scientificComputingValue
    : expression
    ;


/*
 * ============================================================================
 * 5. SCIENTIFIC OPERAND
 * ============================================================================
 */

scientificComputingOperand
    : expression
    ;


/*
 * ============================================================================
 * 6. SCIENTIFIC OPERATION
 * ============================================================================
 *
 * Operations such as:
 *
 *     solve(...)
 *     simulate(...)
 *     integrate(...)
 *     differentiate(...)
 *     interpolate(...)
 *     optimize(...)
 *
 * are ordinary expressions.
 *
 * ============================================================================
 */

scientificComputingOperation
    : expression
    ;


/*
 * ============================================================================
 * 7. SCIENTIFIC MODEL
 * ============================================================================
 *
 * A model is represented through canonical Zamani syntax.
 *
 * Semantic analysis determines whether the resulting expression/type graph
 * represents a scientific model.
 *
 * ============================================================================
 */

scientificComputingModel
    : expression
    ;


/*
 * ============================================================================
 * 8. SCIENTIFIC SIMULATION
 * ============================================================================
 *
 * Simulation is a semantic classification boundary.
 *
 * The grammar does not select a solver or execution architecture.
 *
 * ============================================================================
 */

scientificComputingSimulation
    : expression
    ;


/*
 * ============================================================================
 * 9. SCIENTIFIC METHOD
 * ============================================================================
 *
 * Numerical/scientific methods are ordinary expressions or semantic operation
 * references.
 *
 * ============================================================================
 */

scientificComputingMethod
    : expression
    ;


/*
 * ============================================================================
 * 10. SCIENTIFIC ALGORITHM
 * ============================================================================
 *
 * No finite algorithm vocabulary is imposed.
 *
 * ============================================================================
 */

scientificComputingAlgorithm
    : expression
    ;


/*
 * ============================================================================
 * 11. SCIENTIFIC TRANSFORMATION
 * ============================================================================
 */

scientificComputingTransformation
    : expression
    ;


/*
 * ============================================================================
 * 12. SCIENTIFIC REDUCTION
 * ============================================================================
 */

scientificComputingReduction
    : expression
    ;


/*
 * ============================================================================
 * 13. SCIENTIFIC SOLVER
 * ============================================================================
 */

scientificComputingSolver
    : expression
    ;


/*
 * ============================================================================
 * 14. SCIENTIFIC INTEGRATION
 * ============================================================================
 */

scientificComputingIntegration
    : expression
    ;


/*
 * ============================================================================
 * 15. SCIENTIFIC DIFFERENTIATION
 * ============================================================================
 */

scientificComputingDifferentiation
    : expression
    ;


/*
 * ============================================================================
 * 16. SCIENTIFIC INTERPOLATION
 * ============================================================================
 */

scientificComputingInterpolation
    : expression
    ;


/*
 * ============================================================================
 * 17. SCIENTIFIC APPROXIMATION
 * ============================================================================
 */

scientificComputingApproximation
    : expression
    ;


/*
 * ============================================================================
 * 18. SCIENTIFIC OPTIMIZATION
 * ============================================================================
 *
 * Optimization syntax remains owned by the classical optimization domain.
 *
 * ============================================================================
 */

scientificComputingOptimization
    : expression
    ;


/*
 * ============================================================================
 * 19. SCIENTIFIC STATISTICS
 * ============================================================================
 */

scientificComputingStatistics
    : expression
    ;


/*
 * ============================================================================
 * 20. SCIENTIFIC PROBABILITY
 * ============================================================================
 */

scientificComputingProbability
    : expression
    ;


/*
 * ============================================================================
 * 21. SCIENTIFIC STOCHASTIC COMPUTATION
 * ============================================================================
 */

scientificComputingStochastic
    : expression
    ;


/*
 * ============================================================================
 * 22. SCIENTIFIC DATA
 * ============================================================================
 */

scientificComputingData
    : expression
    ;


/*
 * ============================================================================
 * 23. SCIENTIFIC FIELD
 * ============================================================================
 */

scientificComputingField
    : expression
    ;


/*
 * ============================================================================
 * 24. SCIENTIFIC PARAMETER
 * ============================================================================
 */

scientificComputingParameter
    : expression
    ;


/*
 * ============================================================================
 * 25. SCIENTIFIC DOMAIN
 * ============================================================================
 *
 * A domain may be continuous, discrete, symbolic, geometric, temporal,
 * spatial, stochastic, or another semantic domain.
 *
 * Its concrete representation remains an ordinary Zamani expression.
 *
 * ============================================================================
 */

scientificComputingDomain
    : expression
    ;


/*
 * ============================================================================
 * 26. SCIENTIFIC CONDITION
 * ============================================================================
 */

scientificComputingCondition
    : expression
    ;


/*
 * ============================================================================
 * 27. SCIENTIFIC CONSTRAINT
 * ============================================================================
 *
 * The expression is classified semantically as a scientific constraint.
 *
 * ============================================================================
 */

scientificComputingConstraint
    : expression
    ;


/*
 * ============================================================================
 * 28. SCIENTIFIC REQUIREMENT
 * ============================================================================
 *
 * Resource/capability requirement syntax remains owned by resources/.
 *
 * This rule is only a semantic integration point.
 *
 * ============================================================================
 */

scientificComputingRequirement
    : expression
    ;


/*
 * ============================================================================
 * 29. SCIENTIFIC CAPABILITY
 * ============================================================================
 */

scientificComputingCapability
    : expression
    ;


/*
 * ============================================================================
 * 30. SCIENTIFIC PREFERENCE
 * ============================================================================
 */

scientificComputingPreference
    : expression
    ;


/*
 * ============================================================================
 * 31. SCIENTIFIC RESOURCE
 * ============================================================================
 */

scientificComputingResource
    : expression
    ;


/*
 * ============================================================================
 * 32. SCIENTIFIC PRECISION REQUIREMENT
 * ============================================================================
 *
 * Precision is semantic/type information, not a parser-level machine limit.
 *
 * ============================================================================
 */

scientificComputingPrecision
    : expression
    ;


/*
 * ============================================================================
 * 33. SCIENTIFIC TOLERANCE
 * ============================================================================
 */

scientificComputingTolerance
    : expression
    ;


/*
 * ============================================================================
 * 34. SCIENTIFIC ERROR BOUND
 * ============================================================================
 */

scientificComputingErrorBound
    : expression
    ;


/*
 * ============================================================================
 * 35. SCIENTIFIC CONVERGENCE CONDITION
 * ============================================================================
 */

scientificComputingConvergence
    : expression
    ;


/*
 * ============================================================================
 * 36. SCIENTIFIC STABILITY CONDITION
 * ============================================================================
 */

scientificComputingStability
    : expression
    ;


/*
 * ============================================================================
 * 37. SCIENTIFIC REPRODUCIBILITY REQUIREMENT
 * ============================================================================
 */

scientificComputingReproducibility
    : expression
    ;


/*
 * ============================================================================
 * 38. SCIENTIFIC PROVENANCE
 * ============================================================================
 */

scientificComputingProvenance
    : expression
    ;


/*
 * ============================================================================
 * 39. SCIENTIFIC UNCERTAINTY
 * ============================================================================
 */

scientificComputingUncertainty
    : expression
    ;


/*
 * ============================================================================
 * 40. SCIENTIFIC SENSITIVITY
 * ============================================================================
 */

scientificComputingSensitivity
    : expression
    ;


/*
 * ============================================================================
 * 41. SCIENTIFIC PARAMETER STUDY
 * ============================================================================
 */

scientificComputingParameterStudy
    : expression
    ;


/*
 * ============================================================================
 * 42. SCIENTIFIC FORWARD MODEL
 * ============================================================================
 */

scientificComputingForwardModel
    : expression
    ;


/*
 * ============================================================================
 * 43. SCIENTIFIC INVERSE MODEL
 * ============================================================================
 */

scientificComputingInverseModel
    : expression
    ;


/*
 * ============================================================================
 * 44. SCIENTIFIC SURROGATE MODEL
 * ============================================================================
 */

scientificComputingSurrogateModel
    : expression
    ;


/*
 * ============================================================================
 * 45. SCIENTIFIC EQUATION
 * ============================================================================
 *
 * Equations are represented by canonical expressions.
 *
 * ============================================================================
 */

scientificComputingEquation
    : expression
    ;


/*
 * ============================================================================
 * 46. SCIENTIFIC DIFFERENTIAL EQUATION
 * ============================================================================
 */

scientificComputingDifferentialEquation
    : expression
    ;


/*
 * ============================================================================
 * 47. SCIENTIFIC ALGEBRAIC EQUATION
 * ============================================================================
 */

scientificComputingAlgebraicEquation
    : expression
    ;


/*
 * ============================================================================
 * 48. SCIENTIFIC INITIAL CONDITION
 * ============================================================================
 */

scientificComputingInitialCondition
    : expression
    ;


/*
 * ============================================================================
 * 49. SCIENTIFIC BOUNDARY CONDITION
 * ============================================================================
 */

scientificComputingBoundaryCondition
    : expression
    ;


/*
 * ============================================================================
 * 50. SCIENTIFIC OBSERVATION
 * ============================================================================
 */

scientificComputingObservation
    : expression
    ;


/*
 * ============================================================================
 * 51. SCIENTIFIC DATASET
 * ============================================================================
 */

scientificComputingDataset
    : expression
    ;


/*
 * ============================================================================
 * 52. SCIENTIFIC SAMPLING
 * ============================================================================
 */

scientificComputingSampling
    : expression
    ;


/*
 * ============================================================================
 * 53. SCIENTIFIC STATISTICAL INFERENCE
 * ============================================================================
 */

scientificComputingInference
    : expression
    ;


/*
 * ============================================================================
 * 54. SCIENTIFIC TRANSFORM
 * ============================================================================
 */

scientificComputingTransform
    : expression
    ;


/*
 * ============================================================================
 * 55. SCIENTIFIC SIMULATION STEP
 * ============================================================================
 */

scientificComputingStep
    : expression
    ;


/*
 * ============================================================================
 * 56. SCIENTIFIC WORKFLOW
 * ============================================================================
 */

scientificComputingWorkflow
    : expression
    ;


/*
 * ============================================================================
 * 57. SCIENTIFIC PARALLEL COMPUTATION
 * ============================================================================
 *
 * Parallelization is semantic/execution intent.
 *
 * No thread/core/device count is encoded.
 *
 * ============================================================================
 */

scientificComputingParallel
    : expression
    ;


/*
 * ============================================================================
 * 58. SCIENTIFIC DISTRIBUTED COMPUTATION
 * ============================================================================
 *
 * Distributed realization belongs downstream.
 *
 * ============================================================================
 */

scientificComputingDistributed
    : expression
    ;


/*
 * ============================================================================
 * 59. SCIENTIFIC ACCELERATED COMPUTATION
 * ============================================================================
 *
 * Accelerator realization is downstream.
 * ============================================================================
 */

scientificComputingAccelerated
    : expression
    ;


/*
 * ============================================================================
 * 60. SCIENTIFIC QUANTUM BOUNDARY
 * ============================================================================
 *
 * Scientific computation may participate in quantum-classical programs.
 *
 * No quantum syntax is defined here.
 *
 * ============================================================================
 */

scientificComputingQuantumBoundary
    : expression
    ;


/*
 * ============================================================================
 * 61. SCIENTIFIC HDL/HARDWARE BOUNDARY
 * ============================================================================
 */

scientificComputingHardwareBoundary
    : expression
    ;


/*
 * ============================================================================
 * 62. SCIENTIFIC DATA/AI BOUNDARY
 * ============================================================================
 */

scientificComputingDataAIBoundary
    : expression
    ;


/*
 * ============================================================================
 * 63. SCIENTIFIC SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Stable entry point for downstream semantic consumers.
 *
 * ============================================================================
 */

scientificComputingSemanticBoundary
    : expression
    | typeExpression
    ;


/*
 * ============================================================================
 * 64. SCIENTIFIC VALIDATION INPUT
 * ============================================================================
 *
 * This entry point is intended for grammar conformance tooling.
 *
 * Semantic validation remains downstream.
 *
 * ============================================================================
 */

scientificComputingValidationInput
    : scientificComputingConstruct EOF?
    ;


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is a parser grammar.
 * [x] It uses the canonical Zamani lexer vocabulary.
 * [x] It imports canonical Expressions.
 * [x] It imports canonical Types.
 * [x] It defines no lexer rules.
 * [x] It defines no embedded Rust.
 * [x] It defines no semantic predicates.
 * [x] It defines no hardware-dependent parsing.
 * [x] It defines no artificial resource limits.
 * [x] It defines no MAX_* capacity constants.
 * [x] It does not redefine expression precedence.
 * [x] It does not redefine function-call syntax.
 * [x] It does not redefine indexing.
 * [x] It does not redefine operators.
 * [x] It does not redefine numeric literals.
 * [x] It does not redefine vector syntax.
 * [x] It does not redefine matrix syntax.
 * [x] It does not redefine tensor syntax.
 * [x] It does not redefine linear algebra.
 * [x] It does not redefine numerical algorithms.
 * [x] It does not enumerate scientific algorithms as keywords.
 * [x] It does not create a scientific-specific AST.
 * [x] It does not create a scientific-specific IR.
 * [x] It does not create a quantum IR.
 * [x] It does not select a CPU/GPU/FPGA/QPU.
 * [x] It does not select a numerical provider.
 * [x] It preserves POCO-REAF.
 *
 * ============================================================================
 * REQUIRED DOWNSTREAM INTEGRATION
 * ============================================================================
 *
 * A stable feature using this grammar must have:
 *
 *     source syntax
 *         ->
 *     canonical frontend AST
 *         ->
 *     scientific semantic classification
 *         ->
 *     canonical semantic model
 *         ->
 *     canonical IR
 *         ->
 *     compiler lowering
 *         ->
 *     runtime/backend
 *
 * Tests must cover:
 *
 *     - positive parsing;
 *     - negative parsing;
 *     - boundary syntax;
 *     - scalability;
 *     - deterministic parsing;
 *     - compatibility;
 *     - cross-domain use.
 *
 * ============================================================================
 */