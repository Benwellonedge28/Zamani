/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/numeric.g4
 *
 * Grammar:
 *     Numeric
 *
 * Status:
 *     Production numerical-domain grammar boundary.
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
 *     - No runtime execution.
 *     - No hardware access.
 *     - No evaluation.
 *     - No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the CLASSICAL NUMERIC DOMAIN BOUNDARY.
 *
 * It does NOT define a second expression language.
 *
 * Numeric syntax is built on the canonical Zamani expression architecture:
 *
 *     source
 *        |
 *        v
 *     canonical lexer
 *        |
 *        v
 *     canonical expression grammar
 *        |
 *        v
 *     numeric semantic classification
 *        |
 *        v
 *     frontend AST
 *        |
 *        v
 *     semantic analysis
 *        |
 *        v
 *     canonical semantic representation
 *        |
 *        +-----------------------------+
 *        |                             |
 *        v                             v
 *   classical IR                 hybrid/other IR
 *        |
 *        v
 *   optimization
 *        |
 *        v
 *   resource/capability analysis
 *        |
 *        v
 *   scheduling / lowering
 *        |
 *        v
 *   target realization
 *
 * ============================================================================
 * CORE ARCHITECTURAL RULE
 * ============================================================================
 *
 * NUMERIC DOMAIN
 * !=
 * NUMERIC LEXER
 * !=
 * ARITHMETIC PRECEDENCE
 * !=
 * NUMERIC TYPE SYSTEM
 * !=
 * NUMERICAL ALGORITHM LIBRARY
 * !=
 * CLASSICAL IR
 *
 * Those responsibilities belong to different layers.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the classical numeric-domain parser boundary;
 *     - explicit numeric-domain entry points;
 *     - numeric-expression classification hooks;
 *     - numeric-literal classification hooks;
 *     - numeric-domain construction hooks where required by the language;
 *     - integration of numeric syntax with the canonical expression grammar.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - numeric literal spelling;
 *     - identifiers;
 *     - arithmetic operators;
 *     - arithmetic precedence;
 *     - arithmetic associativity;
 *     - unary operators;
 *     - assignment;
 *     - comparisons;
 *     - logical operators;
 *     - bitwise operators;
 *     - function calls;
 *     - indexing;
 *     - member access;
 *     - ranges;
 *     - general type syntax;
 *     - classical type definitions;
 *     - vector types;
 *     - matrix types;
 *     - tensor types;
 *     - numerical algorithms;
 *     - numerical libraries;
 *     - floating-point semantics;
 *     - integer semantics;
 *     - precision semantics;
 *     - overflow semantics;
 *     - rounding semantics;
 *     - memory allocation;
 *     - vectorization;
 *     - SIMD selection;
 *     - CPU selection;
 *     - GPU selection;
 *     - FPGA selection;
 *     - accelerator selection;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - hardware discovery;
 *     - resource discovery;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * CANONICAL LEXER CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * It consumes the production lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * The canonical lexical composition ultimately derives from:
 *
 *     grammar/lexer/tokens.g4
 *
 * and the lexical numeric implementation:
 *
 *     grammar/lexer/numeric-literals.g4
 *
 * Numeric lexical ownership remains there.
 *
 * This file MUST NOT declare lexer rules.
 *
 * ============================================================================
 * CANONICAL NUMERIC TOKENS
 * ============================================================================
 *
 * Numeric literal tokens currently provided by the canonical lexical system
 * include:
 *
 *     INTEGER
 *     FLOAT
 *
 * The parser MUST consume those existing tokens rather than inventing:
 *
 *     NUMBER
 *     NUMERIC_LITERAL
 *     INT_LITERAL
 *     FLOAT_LITERAL
 *     DECIMAL_LITERAL
 *
 * aliases.
 *
 * If additional numeric lexical forms are required in the future, they must
 * first be standardized in the canonical lexer architecture.
 *
 * ============================================================================
 * OPERATOR CONTRACT
 * ============================================================================
 *
 * This file MUST NOT define arithmetic precedence.
 *
 * Arithmetic precedence belongs to:
 *
 *     grammar/expressions/
 *
 * In particular, this file MUST NOT redefine:
 *
 *     additiveExpression
 *     multiplicativeExpression
 *     prefixExpression
 *     postfixExpression
 *     assignmentExpression
 *     conditionalExpression
 *     comparisonExpression
 *
 * Numeric expressions therefore use exactly the same expression semantics as
 * every other Zamani expression.
 *
 * Existing canonical arithmetic tokens include:
 *
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     MODULO
 *
 * This file does not redefine them.
 *
 * ============================================================================
 * EXPONENTIATION CONTRACT
 * ============================================================================
 *
 * This file deliberately does not introduce POWER or another exponentiation
 * token.
 *
 * The current canonical operator vocabulary does not establish POWER as a
 * production lexical token.
 *
 * If exponentiation becomes a standardized Zamani operator, it must be added
 * once to the canonical lexer/operator architecture and once to the canonical
 * expression precedence hierarchy.
 *
 * It must NOT be introduced independently here.
 *
 * ============================================================================
 * NUMERIC DOMAIN VS NUMERIC TYPE
 * ============================================================================
 *
 * Syntax alone does not determine the complete numeric type.
 *
 * For example:
 *
 *     42
 *
 * is a numeric literal.
 *
 * It is not automatically:
 *
 *     i8
 *     i16
 *     i32
 *     i64
 *     u32
 *     u64
 *     usize
 *
 * Likewise:
 *
 *     1.25
 *
 * is not automatically:
 *
 *     f32
 *     f64
 *     IEEE-754
 *
 * Type interpretation belongs to the type and semantic systems.
 *
 * ============================================================================
 * NUMERIC EXPRESSION CONTRACT
 * ============================================================================
 *
 * A numeric expression is syntactically an ordinary Zamani expression.
 *
 * Examples:
 *
 *     x + y
 *     x * y
 *     x / y
 *     x % y
 *     (x + y) * z
 *     f(x) + g(y)
 *     vector_value[i] * scale
 *     matrix_value[i, j] + offset
 *
 * Whether an expression is numerically valid is determined downstream by:
 *
 *     name resolution
 *     type inference
 *     type checking
 *     operator resolution
 *     overload resolution
 *     effect analysis
 *     domain analysis
 *     shape analysis
 *     capability analysis
 *
 * This grammar does not perform those operations.
 *
 * ============================================================================
 * PUBLIC NUMERIC ENTRY POINT
 * ============================================================================
 *
 * The public entry point is intentionally a semantic-domain boundary rather
 * than a duplicate expression hierarchy.
 *
 * A consumer requiring general expressions MUST use:
 *
 *     expression
 *
 * A consumer specifically requiring a numerical-domain expression MAY use:
 *
 *     numericExpression
 *
 * The rule delegates completely to the canonical expression grammar.
 *
 * ============================================================================
 * NUMERIC LITERAL BOUNDARY
 * ============================================================================
 *
 * Numeric literal syntax is owned by the canonical lexer.
 *
 * This grammar exposes a parser-level classification hook for consumers that
 * explicitly need to recognize a numeric literal.
 *
 * It does not redefine literal spelling.
 *
 * ============================================================================
 * NUMERIC CONSTRUCTION
 * ============================================================================
 *
 * Construction of vectors, matrices, tensors, arrays, ranges, maps, and other
 * data structures remains owned by their respective grammar domains.
 *
 * Numeric values can participate in those constructs through the universal
 * expression system.
 *
 * For example:
 *
 *     [1, 2, 3]
 *
 * is governed by collection/literal grammar.
 *
 * The numeric meaning of its elements is established semantically.
 *
 * ============================================================================
 * NUMERICAL ALGORITHMS
 * ============================================================================
 *
 * Numerical algorithms MUST NOT be encoded as an ever-growing collection of
 * grammar keywords.
 *
 * Examples that should normally remain ordinary operations, intrinsics, or
 * standard-library facilities include:
 *
 *     sin
 *     cos
 *     tan
 *     exp
 *     log
 *     sqrt
 *     abs
 *     min
 *     max
 *     sum
 *     product
 *     mean
 *     variance
 *     determinant
 *     inverse
 *     solve
 *     eig
 *     fft
 *     convolution
 *     interpolation
 *
 * These names remain extensible identifiers unless the canonical language
 * specification explicitly reserves one for a language-level reason.
 *
 * This allows:
 *
 *     standard libraries
 *     user libraries
 *     compiler intrinsics
 *     dialects
 *     accelerator implementations
 *     future numerical systems
 *
 * without modifying the core numerical grammar for every new operation.
 *
 * ============================================================================
 * NUMERIC TYPES
 * ============================================================================
 *
 * Numeric type syntax belongs to the canonical type system.
 *
 * This file MUST NOT redefine:
 *
 *     Integer
 *     Float
 *     Decimal
 *     FixedPoint
 *     Rational
 *     Complex
 *     Vector
 *     Matrix
 *     Tensor
 *
 * or any future numeric type.
 *
 * The semantic layer determines whether an expression has a numeric type.
 *
 * ============================================================================
 * PRECISION AND REPRESENTATION
 * ============================================================================
 *
 * Numeric source syntax MUST NOT imply a particular target representation.
 *
 * The parser must not assume:
 *
 *     machine word size
 *     pointer size
 *     register width
 *     SIMD width
 *     floating-point unit
 *     CPU instruction format
 *     GPU numeric format
 *     FPGA numeric format
 *     accelerator format
 *
 * Those are downstream realization properties.
 *
 * ============================================================================
 * ARBITRARY PRECISION
 * ============================================================================
 *
 * Lexical acceptance of a numeric literal MUST NOT depend on whether the
 * literal fits into the host compiler's native integer or floating-point
 * representation.
 *
 * For example, a syntactically valid INTEGER token may represent a value
 * larger than any native machine integer.
 *
 * Semantic analysis determines whether the selected numeric type and target
 * can represent or evaluate the value.
 *
 * A failure to represent a value on a particular target is a semantic,
 * compilation, or resource diagnostic—not a grammar-level language limit.
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * The grammar contains no artificial numeric scale limits.
 *
 * It MUST NOT encode:
 *
 *     MAX_BITS
 *     MAX_DIGITS
 *     MAX_PRECISION
 *     MAX_SCALE
 *     MAX_VECTOR_LENGTH
 *     MAX_MATRIX_ROWS
 *     MAX_MATRIX_COLUMNS
 *     MAX_TENSOR_RANK
 *     MAX_ELEMENTS
 *     MAX_OPERATIONS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ACCELERATORS
 *     MAX_MEMORY
 *     MAX_NODES
 *
 * It also MUST NOT encode:
 *
 *     fixed register widths
 *     fixed SIMD widths
 *     fixed processor counts
 *     fixed device counts
 *     physical addresses
 *     device identifiers
 *     hardware topology
 *
 * Structural grammar repetition is used instead of artificial language
 * limits.
 *
 * "Scale to infinity" means that the language does not impose an arbitrary
 * finite machine-scale ceiling on numeric program semantics.
 *
 * Actual execution remains bounded by:
 *
 *     available resources
 *     implementation capabilities
 *     explicit program requirements
 *     target capabilities
 *
 * ============================================================================
 * SYMBOLIC NUMERIC VALUES
 * ============================================================================
 *
 * Numeric expressions may contain symbolic values.
 *
 * Examples:
 *
 *     n + 1
 *     2 * x
 *     scale * parameter
 *     f(x)
 *     A[i, j]
 *
 * The parser does not determine whether a value is:
 *
 *     compile-time constant
 *     symbolic
 *     runtime-derived
 *     externally supplied
 *     dependent on another shape
 *
 * Those are semantic/compiler properties.
 *
 * ============================================================================
 * SHAPE INTEGRATION
 * ============================================================================
 *
 * Numeric expressions may describe or participate in shape expressions.
 *
 * Examples:
 *
 *     n
 *     n + 1
 *     rows * columns
 *     rank + offset
 *
 * Shape semantics belong to:
 *
 *     types
 *     vector
 *     matrix
 *     tensor
 *     data
 *
 * Numeric syntax itself does not impose a shape limit.
 *
 * ============================================================================
 * VECTOR / MATRIX / TENSOR INTEGRATION
 * ============================================================================
 *
 * Numeric expressions may occur in:
 *
 *     vector construction
 *     matrix construction
 *     tensor construction
 *     indexing
 *     slicing
 *     shape expressions
 *     reductions
 *     transformations
 *     resource expressions
 *
 * Their structural syntax remains owned by the appropriate grammar modules.
 *
 * The distinction is:
 *
 *     numeric value
 *          !=
 *     data structure
 *          !=
 *     shape
 *          !=
 *     storage layout
 *          !=
 *     hardware vector width
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical programs may use numeric expressions for:
 *
 *     scalar computation
 *     numerical algorithms
 *     scientific computing
 *     signal processing
 *     optimization
 *     vector computation
 *     matrix computation
 *     tensor computation
 *     symbolic computation
 *     control computation
 *     data processing
 *
 * No classical backend is selected by this grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Numeric expressions may occur inside quantum programs, especially for:
 *
 *     operation parameters
 *     angles
 *     probabilities
 *     thresholds
 *     timing expressions
 *     symbolic parameters
 *     compile-time expressions
 *     classical feed-forward
 *
 * Example:
 *
 *     theta + delta
 *
 * remains an ordinary expression.
 *
 * The quantum subsystem determines how the resulting semantic value is used.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT create:
 *
 *     numeric quantum IR
 *     quantum arithmetic IR
 *     a second quantum semantic representation
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Numeric expressions may occur in hardware-oriented contexts such as:
 *
 *     width expressions
 *     index expressions
 *     parameter expressions
 *     timing expressions
 *     address calculations
 *     resource constraints
 *
 * The numeric grammar does not encode:
 *
 *     wire width limits
 *     register widths
 *     bus widths
 *     clock frequencies
 *     physical addresses
 *     FPGA resource counts
 *     ASIC technology limits
 *
 * Such information belongs to semantic hardware/resource descriptions and
 * target realization.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Numeric expressions may participate in resource requirements.
 *
 * Conceptually:
 *
 *     required_memory
 *     required_compute
 *     required_bandwidth
 *     required_latency
 *     required_precision
 *
 * The resource subsystem interprets those expressions.
 *
 * This grammar does not decide whether a requirement is satisfiable.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Numeric expressions may describe:
 *
 *     partition sizes
 *     chunk sizes
 *     replication factors
 *     message sizes
 *     iteration ranges
 *     resource quantities
 *
 * No node count or topology limit is encoded.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Numeric expressions may participate in:
 *
 *     tensor shapes
 *     model parameters
 *     batch-size expressions
 *     learning-rate expressions
 *     optimization parameters
 *     dataset transformations
 *     dataflow quantities
 *
 * The grammar remains independent of a particular AI framework or backend.
 *
 * ============================================================================
 * EFFECT / MEMORY / CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Numeric expressions may occur within:
 *
 *     effect parameters
 *     memory sizes
 *     allocation expressions
 *     concurrency bounds
 *     task parameters
 *     synchronization conditions
 *
 * Semantic validation determines legality.
 *
 * No fixed worker/thread/core limit is encoded here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar constructs no AST directly.
 *
 * The frontend AST must preserve enough structure to represent:
 *
 *     numeric literal
 *     numeric expression
 *     operator identity
 *     operand ordering
 *     nesting/grouping
 *     source span
 *
 * The AST should remain domain-neutral.
 *
 * Do NOT introduce parser-only nodes such as:
 *
 *     IntegerAdd
 *     FloatAdd
 *     MatrixAdd
 *     TensorAdd
 *     GPUAdd
 *     FPGAAdd
 *
 * merely because semantic analysis later identifies an operand type.
 *
 * The same syntactic expression may acquire different semantic types while
 * preserving one structural AST representation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     numeric type resolution
 *     signedness
 *     precision
 *     scale
 *     exactness
 *     numeric promotion
 *     conversion
 *     overload resolution
 *     operator legality
 *     overflow policy
 *     underflow policy
 *     rounding
 *     division-by-zero behavior
 *     modulo semantics
 *     NaN semantics
 *     infinity semantics
 *     symbolic evaluation
 *     constant evaluation
 *     shape compatibility
 *     numerical stability
 *     domain legality
 *
 * The grammar does not perform any of these operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar has no direct IR dependency.
 *
 * Intended flow:
 *
 *     numeric syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     classical representation     hybrid/other representation
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     target lowering
 *
 * The grammar MUST NOT introduce a numeric-specific IR.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * Compiler stages consuming this grammar operate downstream:
 *
 *     parsing
 *     AST construction
 *     name resolution
 *     type checking
 *     constant evaluation
 *     semantic analysis
 *     canonical IR generation
 *     optimization
 *     target lowering
 *
 * Numeric optimization is not grammar behavior.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * This grammar has no direct runtime dependency.
 *
 * Runtime realization may choose representations based on:
 *
 *     target capabilities
 *     available resources
 *     semantic requirements
 *     numerical guarantees
 *     program constraints
 *
 * Source syntax remains unchanged.
 *
 * ============================================================================
 * TOOLING INTEGRATION
 * ============================================================================
 *
 * Formatters, syntax highlighters, IDE tooling, and language servers should
 * derive numeric syntax from:
 *
 *     canonical lexer tokens
 *     canonical expression rules
 *     this numeric-domain boundary
 *
 * Tooling MUST NOT create a second numeric grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing of numeric syntax must be deterministic.
 *
 * The result must not depend on:
 *
 *     CPU count
 *     GPU availability
 *     accelerator availability
 *     memory capacity
 *     runtime state
 *     network state
 *     hardware topology
 *     calibration
 *     scheduling
 *     randomness
 *
 * For identical source tokens and language/grammar versions, the syntactic
 * structure must be identical.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics belong to syntax.
 *
 * Examples:
 *
 *     malformed numeric token sequence
 *     incomplete expression
 *     invalid punctuation
 *     malformed grouping
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples:
 *
 *     numeric type mismatch
 *     overflow
 *     unsupported precision
 *     invalid conversion
 *     incompatible shapes
 *     division by zero
 *     unsupported target representation
 *     unsatisfied resource requirement
 *
 * These must not be converted into parser-level numeric restrictions.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no evaluation;
 *     - performs no I/O;
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - performs no device access;
 *     - performs no dynamic code execution;
 *     - contains no embedded Rust actions;
 *     - contains no unsafe code.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing canonical numeric literal syntax must remain accepted.
 *
 * Adding a new numerical library operation must not require changing this
 * grammar when that operation can be expressed as an ordinary function,
 * intrinsic, or operation.
 *
 * Changes to arithmetic precedence or operator spelling are NOT numerical
 * grammar changes; they belong to the canonical expression/lexer compatibility
 * process.
 *
 * A future language version may extend this grammar only through an explicit
 * compatibility decision.
 *
 * ============================================================================
 * LEGACY NUMERICAL GRAMMAR INTEGRATION
 * ============================================================================
 *
 * The repository currently contains:
 *
 *     grammar/classical/numerical.g4
 *
 * That file contains a substantially broader numerical-domain grammar.
 *
 * To prevent two competing numerical authorities, the production architecture
 * MUST establish ONE canonical ownership path.
 *
 * Recommended migration:
 *
 *     numeric.g4
 *         = narrow canonical numeric-domain boundary
 *
 *     numerical.g4
 *         = retained compatibility/design surface
 *
 * The canonical root should not import both files merely because both exist.
 *
 * If numerical.g4 contains constructs that are genuinely required as
 * language-level syntax, those constructs should be individually promoted
 * into the canonical modular architecture rather than copied wholesale.
 *
 * No existing valid feature should be silently discarded.
 *
 * ============================================================================
 * EXPRESSIONS/ARITHMETIC LEGACY INTEGRATION
 * ============================================================================
 *
 * The repository also contains:
 *
 *     grammar/expressions/arithmetic.g4
 *
 * It currently represents an older arithmetic hierarchy and uses a different
 * lexical vocabulary in parts of its historical implementation.
 *
 * This numeric grammar MUST NOT import that file.
 *
 * In particular, it must not inherit:
 *
 *     POWER
 *
 * merely because the legacy grammar references it.
 *
 * Arithmetic precedence remains a single-authority concern of the canonical
 * expressions architecture.
 *
 * ============================================================================
 * CLASSICAL ROOT INTEGRATION
 * ============================================================================
 *
 * The intended integration is:
 *
 *     grammar/classical/classical.g4
 *             |
 *             +--> Numeric
 *
 * where required by the canonical composition architecture.
 *
 * `classical.g4` remains responsible for classical-domain composition.
 *
 * It must not copy this file's rules into itself.
 *
 * ============================================================================
 * ROOT GRAMMAR INTEGRATION
 * ============================================================================
 *
 * The canonical:
 *
 *     grammar/Zamani.g4
 *
 * should compose classical grammar through:
 *
 *     Classical
 *
 * rather than importing Numeric directly as a second top-level domain.
 *
 * Therefore:
 *
 *     Zamani
 *        |
 *        v
 *     Classical
 *        |
 *        v
 *     Numeric
 *
 * is preferred over:
 *
 *     Zamani
 *        +--> Classical
 *        +--> Numeric
 *
 * because the latter creates unnecessary parallel domain composition.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests must include:
 *
 *     0
 *     1
 *     42
 *     1.0
 *     3.14159
 *
 *     x
 *     x + y
 *     x - y
 *     x * y
 *     x / y
 *     x % y
 *
 *     (x + y) * z
 *     f(x) + g(y)
 *     vector_value[i] * scale
 *     matrix_value[i, j] + offset
 *
 *     symbolic numeric expressions
 *     compile-time numeric expressions
 *     runtime-derived numeric expressions
 *     numeric expressions in classical contexts
 *     numeric expressions in quantum parameter contexts
 *     numeric expressions in HDL parameter contexts
 *     numeric expressions in resource expressions
 *     numeric expressions in distributed computations
 *     numeric expressions in AI/data contexts
 *
 * Negative tests must include malformed syntax such as:
 *
 *     +
 *     *
 *     /
 *     %
 *     x +
 *     x *
 *     x /
 *     x %
 *     (x + y
 *     x + * y
 *     x * / y
 *
 * Numeric semantic errors such as:
 *
 *     division by zero
 *     overflow
 *     incompatible numeric types
 *     incompatible shapes
 *
 * are semantic tests, not parser tests.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Boundary testing must include:
 *
 *     zero
 *     negative expressions where prefix syntax permits them
 *     very large integer literals
 *     very large floating literals
 *     digit-separated literals where supported
 *     deeply nested expressions
 *     very long arithmetic chains
 *     symbolic dimensions
 *     runtime-derived dimensions
 *
 * No boundary test may establish an artificial language maximum.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Test families must verify:
 *
 *     small numeric values
 *     large numeric values
 *     symbolic values
 *     runtime-derived values
 *     small expression trees
 *     large expression trees
 *     deeply nested expressions
 *     long operator chains
 *     large collection expressions
 *     large tensor-shape expressions
 *
 * The grammar must use the same rules regardless of scale.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Required cross-domain coverage:
 *
 *     classical + quantum
 *     classical + HDL
 *     classical + hardware
 *     classical + resources
 *     classical + distributed
 *     classical + AI
 *     classical + data
 *     classical + concurrency
 *
 * Representative semantic flow:
 *
 *     numeric value
 *         |
 *         v
 *     classical computation
 *         |
 *         v
 *     quantum parameter
 *         |
 *         v
 *     quantum operation
 *         |
 *         v
 *     measurement
 *         |
 *         v
 *     classical numeric value
 *
 * The grammar remains unchanged across these boundaries.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Repeated parsing of the same source under the same grammar and lexer version
 * must produce equivalent parse structures.
 *
 * Parsing must not depend on target hardware or runtime state.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_*
 *     fixed numeric width
 *     fixed precision
 *     fixed vector width
 *     fixed matrix dimensions
 *     fixed tensor rank
 *     fixed register width
 *     fixed SIMD width
 *     fixed processor count
 *     fixed accelerator count
 *     fixed memory capacity
 *     fixed node count
 *     device ID
 *     physical address
 *
 * Explicit numeric values written by a programmer remain program semantics.
 *
 * For example:
 *
 *     n = 1024
 *
 * may be meaningful program data.
 *
 * It does NOT establish:
 *
 *     MAX_N = 1024
 *
 * for the language.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * This grammar introduces no additional arithmetic precedence hierarchy.
 *
 * Numeric expressions delegate to the canonical expression parser.
 *
 * This avoids:
 *
 *     duplicate left recursion
 *     competing precedence rules
 *     unnecessary ambiguity
 *     duplicated operator trees
 *
 * Parser performance therefore remains governed by the canonical expression
 * architecture rather than a numerical-specific expression implementation.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar is compatible with the repository's:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * implementation baseline.
 *
 * The grammar contains no Rust code.
 *
 * Generated/runtime integration must remain safe Rust.
 *
 * `unsafe` is neither required nor introduced by this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] canonical ZamaniLexer vocabulary is consumed;
 * [ ] INTEGER and FLOAT remain owned by the canonical lexer;
 * [ ] no lexer rules are defined here;
 * [ ] no arithmetic operator definitions are duplicated;
 * [ ] no arithmetic precedence is duplicated;
 * [ ] no POWER token is invented;
 * [ ] no general expression hierarchy is duplicated;
 * [ ] numeric expression syntax delegates to `expression`;
 * [ ] numeric literal classification uses canonical tokens;
 * [ ] numeric types remain owned by the type system;
 * [ ] numerical algorithms remain library/intrinsic semantics;
 * [ ] no machine-specific representation is encoded;
 * [ ] no artificial numerical limits are encoded;
 * [ ] no hardware limits are encoded;
 * [ ] no resource limits are encoded;
 * [ ] no target selection is encoded;
 * [ ] AST mapping is defined;
 * [ ] semantic mapping is defined;
 * [ ] IR mapping is defined;
 * [ ] compiler integration is defined;
 * [ ] runtime integration is defined;
 * [ ] tooling integration is defined;
 * [ ] cross-domain integration is defined;
 * [ ] positive tests are defined;
 * [ ] negative tests are defined;
 * [ ] boundary tests are defined;
 * [ ] scalability tests are defined;
 * [ ] determinism tests are defined;
 * [ ] compatibility tests are defined;
 * [ ] hard-coding audit passes;
 * [ ] Rust 1.97/1.97.1 safe-Rust requirements remain satisfied.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * GRAMMAR DEFINITION
 * ============================================================================
 */

parser grammar Numeric;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/*
 * ============================================================================
 * 1. NUMERIC EXPRESSION
 * ============================================================================
 *
 * This is the primary public numerical-domain parser boundary.
 *
 * It intentionally delegates all expression syntax to the canonical
 * expression grammar.
 *
 * Therefore:
 *
 *     x + y
 *
 * has exactly the same syntactic precedence and associativity whether it is
 * encountered through the universal expression entry point or through this
 * numerical-domain boundary.
 *
 * Semantic analysis determines whether the expression is numeric.
 *
 * ============================================================================
 */

numericExpression
    : expression
    ;


/*
 * ============================================================================
 * 2. NUMERIC LITERAL
 * ============================================================================
 *
 * Numeric literal spelling is owned by the canonical lexer.
 *
 * This rule exists only for parser consumers that explicitly need the
 * lexical numeric-literal category.
 *
 * It does not define numeric semantics.
 *
 * ============================================================================
 */

numericLiteral
    : INTEGER
    | FLOAT
    ;


/*
 * ============================================================================
 * 3. NUMERIC VALUE
 * ============================================================================
 *
 * A numeric value is represented through the canonical expression system.
 *
 * The public numeric expression rule remains the semantic entry point.
 *
 * This rule is intentionally equivalent to numericExpression rather than
 * creating a second precedence hierarchy.
 *
 * ============================================================================
 */

numericValue
    : numericExpression
    ;