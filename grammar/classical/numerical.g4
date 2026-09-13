/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/numerical.g4
 *
 * Status:
 *     Production-ready numerical-domain parser grammar.
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
 *     - No evaluation.
 *     - No hardware access.
 *     - No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL NUMERICAL DOMAIN.
 *
 * Numerical syntax describes mathematical and computational intent without
 * selecting a particular numerical representation or execution architecture.
 *
 * The numerical domain includes syntax for:
 *
 *     - numeric values;
 *     - numeric literals;
 *     - numeric references;
 *     - numeric expressions;
 *     - numeric function/application forms;
 *     - symbolic numeric expressions;
 *     - numerical ranges;
 *     - numerical reductions;
 *     - numerical transformations;
 *     - numerical aggregations;
 *     - numerical predicates;
 *     - numerical construction;
 *     - numerical domain statements;
 *     - numerical value declarations;
 *     - numerical-domain integration points.
 *
 * The grammar deliberately does NOT implement numerical algorithms.
 *
 * For example, these are semantic/library operations rather than grammar
 * algorithms:
 *
 *     sin
 *     cos
 *     tan
 *     exp
 *     log
 *     sqrt
 *     abs
 *     norm
 *     determinant
 *     inverse
 *     solve
 *     eig
 *     fft
 *     convolution
 *
 * Their identifiers remain extensible and may be supplied by:
 *
 *     standard libraries;
 *     user libraries;
 *     dialects;
 *     compiler intrinsics;
 *     target-independent numerical implementations;
 *     accelerator implementations;
 *     future language extensions.
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
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     Expressions                    ClassicalTypes
 *          |                              |
 *          +---------------+--------------+
 *                          |
 *                          v
 *                    Numerical.g4
 *                          |
 *                          v
 *                     Frontend AST
 *                          |
 *                          v
 *                  Semantic analysis
 *                          |
 *          +---------------+----------------+
 *          |                                |
 *          v                                v
 *   numerical semantic model        resource/effect metadata
 *          |                                |
 *          +---------------+----------------+
 *                          |
 *                          v
 *                  canonical semantic IR
 *                          |
 *              +-----------+-----------+
 *              |                       |
 *              v                       v
 *        classical IR             data/tensor IR
 *              |
 *              v
 *         optimization
 *              |
 *              v
 *       lowering / scheduling
 *              |
 *              v
 *        target realization
 *
 * IMPORTANT:
 *
 * Numerical.g4 MUST NOT directly construct:
 *
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     hardware state
 *     scheduling state
 *     routing state
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
 *     - numerical-domain syntax;
 *     - numerical expression classification;
 *     - numeric literal classification;
 *     - numeric reference classification;
 *     - symbolic numeric expression boundaries;
 *     - numerical construction syntax;
 *     - numerical transformation syntax;
 *     - numerical reduction syntax;
 *     - numerical aggregation syntax;
 *     - numerical range syntax;
 *     - numerical predicate syntax;
 *     - numerical declaration composition;
 *     - numerical assignment composition;
 *     - numerical expression-statement composition;
 *     - numerical-domain parser integration points.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - token spelling;
 *     - identifier spelling;
 *     - numeric literal spelling;
 *     - arithmetic precedence;
 *     - comparison precedence;
 *     - logical precedence;
 *     - unary precedence;
 *     - assignment operator definitions;
 *     - general expression syntax;
 *     - general type syntax;
 *     - classical type definitions;
 *     - vector type definitions;
 *     - matrix type definitions;
 *     - tensor type definitions;
 *     - numerical algorithms;
 *     - floating-point semantics;
 *     - integer overflow semantics;
 *     - numeric promotion;
 *     - numeric coercion;
 *     - numerical precision selection;
 *     - numerical storage;
 *     - memory allocation;
 *     - SIMD selection;
 *     - GPU selection;
 *     - FPGA selection;
 *     - ASIC selection;
 *     - accelerator selection;
 *     - BLAS/LAPACK/provider selection;
 *     - execution placement;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Numerical source syntax describes mathematical/computational intent.
 *
 * It MUST NOT encode:
 *
 *     MAX_PRECISION
 *     MAX_SCALE
 *     MAX_BITS
 *     MAX_DIGITS
 *     MAX_VECTOR_LENGTH
 *     MAX_MATRIX_ROWS
 *     MAX_MATRIX_COLUMNS
 *     MAX_TENSOR_RANK
 *     MAX_ELEMENTS
 *     MAX_OPERATIONS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_ACCELERATORS
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_SIMD_WIDTH
 *     MAX_NODES
 *
 * No numerical construct in this file selects:
 *
 *     CPU count
 *     core count
 *     thread count
 *     GPU count
 *     accelerator count
 *     machine word width
 *     register width
 *     cache size
 *     memory capacity
 *     physical device
 *     device identifier
 *     physical address
 *     topology
 *     deployment size
 *
 * Numerical programs can therefore describe computations that may eventually
 * be realized on:
 *
 *     scalar processors;
 *     vector processors;
 *     multicore CPUs;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     numerical accelerators;
 *     distributed systems;
 *     heterogeneous systems;
 *     embedded systems;
 *     quantum-classical systems;
 *     future computational architectures.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Repetition is structural.
 *
 * Examples:
 *
 *     x + y
 *
 *     f(x, y, z)
 *
 *     sum(values)
 *
 *     reduce(values, operation)
 *
 *     matrix(...)
 *
 *     tensor(...)
 *
 *     expression over an arbitrary symbolic range
 *
 * There is deliberately no grammar-level limit on:
 *
 *     - expression count;
 *     - argument count;
 *     - symbolic dimensions;
 *     - range magnitude;
 *     - vector length;
 *     - matrix dimensions;
 *     - tensor rank;
 *     - numerical precision;
 *     - numeric width;
 *     - literal digit count.
 *
 * Practical parser/compiler limits belong to explicit implementation resource
 * policies rather than the language grammar.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file is a PARSER grammar.
 *
 * It consumes:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * This file MUST NOT declare lexer rules.
 *
 * Numeric lexical spelling is owned by the lexer.
 *
 * The canonical lexer provides tokens including:
 *
 *     INTEGER
 *     FLOAT
 *     TRUE
 *     FALSE
 *     NIL
 *     NULL
 *     STRING
 *     CHAR
 *     IDENTIFIER
 *
 * along with the canonical operator and punctuation vocabulary.
 *
 * This file therefore does NOT introduce aliases such as:
 *
 *     IDENT
 *     NUMBER
 *     NUMBER_LITERAL
 *     NUMERIC_LITERAL
 *     INT_LITERAL
 *     FLOAT_LITERAL
 *     PLUS_OPERATOR
 *     RANGE_OPERATOR
 *
 * ============================================================================
 * IDENTIFIER CONTRACT
 * ============================================================================
 *
 * The canonical lexical identifier token is:
 *
 *     IDENTIFIER
 *
 * Numerical operation names are intentionally identifiers unless the canonical
 * lexer explicitly reserves them.
 *
 * Therefore names such as:
 *
 *     sin
 *     cos
 *     exp
 *     log
 *     sqrt
 *     abs
 *     sum
 *     product
 *     min
 *     max
 *     mean
 *     variance
 *     solve
 *     inverse
 *     fft
 *
 * are not hard-coded into the lexer by this grammar.
 *
 * This permits:
 *
 *     user-defined numerical libraries;
 *     dialects;
 *     vendor-independent APIs;
 *     future numerical operations;
 *     symbolic systems;
 *     compiler intrinsics;
 *     accelerator implementations.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expression syntax belongs to:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file MUST NOT redefine:
 *
 *     additive precedence
 *     multiplicative precedence
 *     comparison precedence
 *     logical precedence
 *     bitwise precedence
 *     unary precedence
 *     assignment precedence
 *     general function-call syntax
 *     general indexing syntax
 *     general member access
 *
 * Numerical values participate in ordinary Zamani expressions.
 *
 * Semantic analysis determines whether an expression is numerically valid.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Numerical VALUE syntax is not numerical TYPE syntax.
 *
 * Numerical type syntax is owned by:
 *
 *     grammar/types/classical-types.g4
 *
 * Examples of type-level concepts include:
 *
 *     Integer
 *     SignedInteger
 *     UnsignedInteger
 *     Float
 *     Decimal
 *     FixedPoint
 *     Complex
 *     Rational
 *     Vector
 *     Matrix
 *     Tensor
 *
 * This file does not redefine those types.
 *
 * ============================================================================
 * NUMERICAL SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The parser does not determine:
 *
 *     - numeric type;
 *     - signedness;
 *     - precision;
 *     - scale;
 *     - rounding;
 *     - overflow behavior;
 *     - underflow behavior;
 *     - NaN semantics;
 *     - infinity semantics;
 *     - exactness;
 *     - symbolic evaluation;
 *     - automatic differentiation;
 *     - numerical stability;
 *     - algorithm selection;
 *     - implementation provider;
 *     - parallelization;
 *     - vectorization;
 *     - accelerator selection.
 *
 * Those are semantic/compiler/runtime concerns.
 *
 * ============================================================================
 * SYMBOLIC NUMERICAL COMPUTATION
 * ============================================================================
 *
 * Numerical expressions may contain symbolic values.
 *
 * Examples:
 *
 *     x + y
 *
 *     f(x)
 *
 *     n + 1
 *
 *     2 * pi
 *
 *     sum(f(i) for i in domain)
 *
 *     A[i, j]
 *
 * The grammar does not attempt to evaluate symbolic expressions.
 *
 * Symbolic computation is therefore compatible with:
 *
 *     compile-time evaluation;
 *     runtime evaluation;
 *     symbolic algebra;
 *     differentiation;
 *     constraint solving;
 *     dependent dimensions;
 *     numerical optimization;
 *     future mathematical systems.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Numerical values may participate in:
 *
 *     classical computation;
 *     quantum parameter expressions;
 *     quantum-classical control;
 *     HDL parameters;
 *     hardware parameters;
 *     timing expressions;
 *     resource expressions;
 *     distributed control;
 *     AI/data computation;
 *     compile-time computation.
 *
 * A numerical expression is therefore NOT inherently tied to a classical-only
 * execution target.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar uses no:
 *
 *     - semantic predicates;
 *     - embedded actions;
 *     - runtime evaluation;
 *     - target-dependent parser decisions.
 *
 * The same token sequence therefore has the same syntactic interpretation
 * independently of:
 *
 *     - available hardware;
 *     - CPU architecture;
 *     - GPU availability;
 *     - quantum backend;
 *     - runtime resources;
 *     - deployment topology.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no evaluation;
 *     - performs no allocation;
 *     - performs no I/O;
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - performs no device access;
 *     - performs no dynamic code execution.
 *
 * ============================================================================
 * IR BOUNDARY
 * ============================================================================
 *
 * The dependency direction is:
 *
 *     grammar
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
 *        +--> classical IR
 *        +--> numerical/data IR
 *        +--> tensor/data representations
 *        +--> resource/effect metadata
 *        |
 *        v
 *     optimization
 *        |
 *        v
 *     scheduling / lowering
 *        |
 *        v
 *     target realization
 *
 * Numerical.g4 MUST NOT import or depend on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     scheduler implementations
 *     hardware implementations
 *     runtime implementations
 *
 * ============================================================================
 */

parser grammar Numerical;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions;


/* ============================================================================
 * 1. PUBLIC NUMERICAL ENTRY POINT
 * ============================================================================
 *
 * `numericalConstruct` is the stable domain-level entry point.
 *
 * It is intentionally narrower than `expression`.
 *
 * Consumers that already parse general expressions should use:
 *
 *     expression
 *
 * Consumers that specifically need numerical-domain syntax should use:
 *
 *     numericalConstruct
 *
 * ============================================================================
 */

numericalConstruct
    : numericalDeclaration
    | numericalAssignment
    | numericalExpressionStatement
    | numericalReduction
    | numericalTransformation
    | numericalConstruction
    ;


/* ============================================================================
 * 2. NUMERICAL VALUE
 * ============================================================================
 *
 * This is the stable parser boundary for a source-level numerical value.
 *
 * A numerical value may be:
 *
 *     - a numeric literal;
 *     - a numeric reference;
 *     - a symbolic numerical expression;
 *     - a numerical callable invocation;
 *     - a parenthesized numerical expression.
 *
 * Semantic analysis determines whether the resulting expression is actually
 * numeric.
 *
 * ============================================================================
 */

numericalValue
    : numericalLiteral
    | numericalReference
    | numericalExpression
    ;


/* ============================================================================
 * 3. NUMERICAL EXPRESSION
 * ============================================================================
 *
 * Numerical expressions are ordinary Zamani expressions.
 *
 * This rule deliberately delegates expression precedence to Expressions.
 *
 * It is a classification boundary, not a second expression hierarchy.
 *
 * ============================================================================
 */

numericalExpression
    : expression
    ;


/* ============================================================================
 * 4. NUMERICAL LITERAL
 * ============================================================================
 *
 * Numeric lexical spelling is owned by ZamaniLexer.
 *
 * This rule performs parser-level classification only.
 *
 * ============================================================================
 */

numericalLiteral
    : integerLiteral
    | floatingLiteral
    ;


/* ============================================================================
 * 5. INTEGER LITERAL
 * ============================================================================
 */

integerLiteral
    : INTEGER
    ;


/* ============================================================================
 * 6. FLOATING LITERAL
 * ============================================================================
 */

floatingLiteral
    : FLOAT
    ;


/* ============================================================================
 * 7. NUMERICAL REFERENCE
 * ============================================================================
 *
 * A numerical reference is an identifier whose semantic value is expected to
 * be numerical.
 *
 * Name resolution and type checking determine whether that is true.
 *
 * ============================================================================
 */

numericalReference
    : IDENTIFIER
    ;


/* ============================================================================
 * 8. NUMERICAL DECLARATION
 * ============================================================================
 *
 * Numerical declarations reuse the canonical general expression system.
 *
 * They do not define numerical types locally.
 *
 * Examples:
 *
 *     let x: Integer = 42;
 *
 *     let y: Float = 3.14;
 *
 *     let n = expression;
 *
 * The semantic layer determines whether the declared value is numerical.
 *
 * ============================================================================
 */

numericalDeclaration
    : numericalBindingKeyword
      IDENTIFIER
      numericalTypeAnnotation?
      ASSIGN
      expression
      SEMICOLON
    | numericalBindingKeyword
      IDENTIFIER
      numericalTypeAnnotation
      SEMICOLON
    ;


/* ============================================================================
 * 9. NUMERICAL BINDING KEYWORD
 * ============================================================================
 */

numericalBindingKeyword
    : LET
    | VAR
    | CONST
    ;


/* ============================================================================
 * 10. NUMERICAL TYPE ANNOTATION
 * ============================================================================
 *
 * The actual type grammar remains authoritative.
 *
 * ============================================================================
 */

numericalTypeAnnotation
    : COLON typeExpression
    ;


/* ============================================================================
 * 11. NUMERICAL ASSIGNMENT
 * ============================================================================
 *
 * Assignment semantics remain outside this file.
 *
 * This rule provides a numerical-domain composition boundary only.
 *
 * ============================================================================
 */

numericalAssignment
    : expression
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 12. NUMERICAL EXPRESSION STATEMENT
 * ============================================================================
 *
 * A numerical expression can occur as a source-level computation statement.
 *
 * ============================================================================
 */

numericalExpressionStatement
    : numericalExpression
      SEMICOLON
    ;


/* ============================================================================
 * 13. NUMERICAL CONSTRUCTION
 * ============================================================================
 *
 * Construction is intentionally expressed as a generic callable form.
 *
 * The constructor name remains an identifier.
 *
 * This permits:
 *
 *     vector(...)
 *     matrix(...)
 *     tensor(...)
 *     range(...)
 *     zeros(...)
 *     ones(...)
 *     fill(...)
 *     random(...)
 *
 * and future library/dialect constructors without changing the lexer.
 *
 * Semantic resolution determines what the callable means.
 *
 * ============================================================================
 */

numericalConstruction
    : numericalCallable
    ;


/* ============================================================================
 * 14. NUMERICAL CALLABLE
 * ============================================================================
 *
 * A callable numerical operation is represented using an identifier followed
 * by an arbitrary expression argument list.
 *
 * No finite argument count is encoded.
 *
 * ============================================================================
 */

numericalCallable
    : IDENTIFIER
      LPAREN
      numericalArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 15. NUMERICAL ARGUMENT LIST
 * ============================================================================
 *
 * Arguments are ordinary expressions.
 *
 * This is essential for symbolic and cross-domain computation.
 *
 * Examples:
 *
 *     f(x)
 *
 *     f(x + 1)
 *
 *     f(n, precision)
 *
 *     f(matrix, tolerance, constraint)
 *
 * ============================================================================
 */

numericalArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 16. NUMERICAL RANGE
 * ============================================================================
 *
 * Numerical ranges use the canonical range operators from the lexer/expression
 * contract.
 *
 * No range magnitude limit is encoded.
 *
 * Examples:
 *
 *     0..n
 *
 *     start..end
 *
 *     start..=end
 *
 * Range semantics remain downstream.
 *
 * ============================================================================
 */

numericalRange
    : numericalRangeStart
      numericalRangeOperator
      numericalRangeEnd
    | numericalRangeOperator
      numericalRangeEnd
    | numericalRangeStart
      numericalRangeOperator
    ;


/* ============================================================================
 * 17. NUMERICAL RANGE START
 * ============================================================================
 */

numericalRangeStart
    : expression
    ;


/* ============================================================================
 * 18. NUMERICAL RANGE END
 * ============================================================================
 */

numericalRangeEnd
    : expression
    ;


/* ============================================================================
 * 19. NUMERICAL RANGE OPERATOR
 * ============================================================================
 *
 * Uses the canonical lexer tokens.
 *
 * ============================================================================
 */

numericalRangeOperator
    : DOT_DOT
    | DOT_DOT_EQ
    ;


/* ============================================================================
 * 20. NUMERICAL REDUCTION
 * ============================================================================
 *
 * Numerical reduction is represented as a generic semantic operation.
 *
 * Examples:
 *
 *     sum(values)
 *     product(values)
 *     min(values)
 *     max(values)
 *     reduce(values, op)
 *
 * The operation name is deliberately not hard-coded.
 *
 * ============================================================================
 */

numericalReduction
    : numericalReductionCall
    ;


/* ============================================================================
 * 21. NUMERICAL REDUCTION CALL
 * ============================================================================
 */

numericalReductionCall
    : IDENTIFIER
      LPAREN
      numericalReductionArguments
      RPAREN
    ;


/* ============================================================================
 * 22. NUMERICAL REDUCTION ARGUMENTS
 * ============================================================================
 *
 * At least one argument is required for this syntactic reduction boundary.
 *
 * The actual operation's arity is semantic.
 *
 * ============================================================================
 */

numericalReductionArguments
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 23. NUMERICAL TRANSFORMATION
 * ============================================================================
 *
 * Numerical transformations include mathematical operations represented by
 * ordinary callable syntax.
 *
 * Examples:
 *
 *     sin(x)
 *     cos(x)
 *     exp(x)
 *     log(x)
 *     sqrt(x)
 *     abs(x)
 *     normalize(x)
 *
 * The grammar deliberately does not enumerate these names.
 *
 * ============================================================================
 */

numericalTransformation
    : numericalTransformationCall
    ;


/* ============================================================================
 * 24. NUMERICAL TRANSFORMATION CALL
 * ============================================================================
 */

numericalTransformationCall
    : IDENTIFIER
      LPAREN
      numericalTransformationArguments?
      RPAREN
    ;


/* ============================================================================
 * 25. NUMERICAL TRANSFORMATION ARGUMENTS
 * ============================================================================
 */

numericalTransformationArguments
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 26. NUMERICAL PREDICATE
 * ============================================================================
 *
 * Numerical predicates are represented as general expressions.
 *
 * This rule exists as a semantic integration boundary rather than redefining
 * comparison syntax.
 *
 * Examples:
 *
 *     x < y
 *
 *     value == expected
 *
 *     norm(x) <= tolerance
 *
 * The comparison grammar remains authoritative.
 *
 * ============================================================================
 */

numericalPredicate
    : expression
    ;


/* ============================================================================
 * 27. NUMERICAL CONDITION
 * ============================================================================
 *
 * A numerical condition is syntactically an expression.
 *
 * Semantic analysis determines whether the expression produces an acceptable
 * predicate/boolean result.
 *
 * ============================================================================
 */

numericalCondition
    : expression
    ;


/* ============================================================================
 * 28. NUMERICAL SYMBOLIC VALUE
 * ============================================================================
 *
 * Symbolic values may be identifiers or arbitrary expressions.
 *
 * No symbolic evaluation occurs in the grammar.
 *
 * ============================================================================
 */

numericalSymbolicValue
    : IDENTIFIER
    | expression
    ;


/* ============================================================================
 * 29. NUMERICAL CONSTANT
 * ============================================================================
 *
 * A numerical constant is syntactically a declaration whose initializer is
 * an expression.
 *
 * Semantic analysis determines:
 *
 *     - constness;
 *     - numerical type;
 *     - exactness;
 *     - compile-time evaluability;
 *     - symbolic status.
 *
 * ============================================================================
 */

numericalConstant
    : CONST
      IDENTIFIER
      numericalTypeAnnotation?
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 30. NUMERICAL COMPREHENSION
 * ============================================================================
 *
 * Numerical comprehensions are represented structurally.
 *
 * Canonical form:
 *
 *     [ expression for identifier in expression ]
 *
 * The iteration source remains an ordinary expression.
 *
 * This keeps the construct compatible with:
 *
 *     finite iteration;
 *     symbolic domains;
 *     lazy computation;
 *     data-parallel execution;
 *     distributed execution;
 *     accelerator lowering;
 *     future execution models.
 *
 * ============================================================================
 */

numericalComprehension
    : LBRACKET
      expression
      FOR
      IDENTIFIER
      IN
      expression
      RBRACKET
    ;


/* ============================================================================
 * 31. NUMERICAL AGGREGATION
 * ============================================================================
 *
 * Aggregation is deliberately callable and identifier-based.
 *
 * Examples:
 *
 *     aggregate(values)
 *
 *     aggregate(values, operation)
 *
 *     aggregate(values, operation, initial)
 *
 * No fixed implementation is selected.
 *
 * ============================================================================
 */

numericalAggregation
    : IDENTIFIER
      LPAREN
      numericalAggregationArguments
      RPAREN
    ;


/* ============================================================================
 * 32. NUMERICAL AGGREGATION ARGUMENTS
 * ============================================================================
 */

numericalAggregationArguments
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 33. NUMERICAL WINDOW / DOMAIN
 * ============================================================================
 *
 * A numerical domain may be represented by an arbitrary expression.
 *
 * This is deliberately not tied to:
 *
 *     array length;
 *     machine vector width;
 *     processor count;
 *     thread count.
 *
 * ============================================================================
 */

numericalDomain
    : expression
    ;


/* ============================================================================
 * 34. NUMERICAL DOMAIN BINDING
 * ============================================================================
 *
 * Example:
 *
 *     for i in domain
 *
 * The complete control-flow grammar remains authoritative elsewhere.
 *
 * ============================================================================
 */

numericalDomainBinding
    : IDENTIFIER
      IN
      numericalDomain
    ;


/* ============================================================================
 * 35. NUMERICAL ITERATION
 * ============================================================================
 *
 * This is a domain integration boundary.
 *
 * The body remains an ordinary expression.
 *
 * ============================================================================
 */

numericalIteration
    : FOR
      numericalDomainBinding
      expression
    ;


/* ============================================================================
 * 36. NUMERICAL EXPRESSION GROUP
 * ============================================================================
 *
 * This provides a stable grouping boundary for downstream semantic consumers.
 *
 * ============================================================================
 */

numericalExpressionGroup
    : LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 37. NUMERICAL ARGUMENT EXPRESSION
 * ============================================================================
 *
 * Explicit integration point for APIs that distinguish numerical arguments
 * from other arguments at the semantic layer.
 *
 * ============================================================================
 */

numericalArgumentExpression
    : expression
    ;


/* ============================================================================
 * 38. NUMERICAL VALUE LIST
 * ============================================================================
 *
 * No fixed number of values is encoded.
 *
 * ============================================================================
 */

numericalValueList
    : numericalValue
      (COMMA numericalValue)*
      COMMA?
    ;


/* ============================================================================
 * 39. NUMERICAL RANGE LIST
 * ============================================================================
 *
 * Multiple numerical ranges may be composed without a finite count.
 *
 * ============================================================================
 */

numericalRangeList
    : numericalRange
      (COMMA numericalRange)*
      COMMA?
    ;


/* ============================================================================
 * 40. NUMERICAL SYMBOLIC LIST
 * ============================================================================
 */

numericalSymbolicList
    : numericalSymbolicValue
      (COMMA numericalSymbolicValue)*
      COMMA?
    ;


/* ============================================================================
 * 41. NUMERICAL FUNCTIONAL FORM
 * ============================================================================
 *
 * A numerical functional form consists of a callable numerical operation.
 *
 * This deliberately remains identifier-based.
 *
 * ============================================================================
 */

numericalFunctionalForm
    : IDENTIFIER
      LPAREN
      numericalArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 42. NUMERICAL COMPOSITION
 * ============================================================================
 *
 * This is the broad semantic integration point for numerical source
 * constructs.
 *
 * It intentionally does not recursively redefine every expression form.
 *
 * ============================================================================
 */

numericalComposition
    : numericalLiteral
    | numericalReference
    | numericalCallable
    | numericalRange
    | numericalComprehension
    | numericalExpressionGroup
    ;


/* ============================================================================
 * 43. NUMERICAL VALUE OR EXPRESSION
 * ============================================================================
 *
 * Stable integration point for declarations, resource expressions, compile-
 * time expressions, hardware parameters, quantum parameters, and other
 * cross-domain consumers.
 *
 * ============================================================================
 */

numericalValueOrExpression
    : numericalValue
    | expression
    ;


/* ============================================================================
 * 44. NUMERICAL DIMENSION EXPRESSION
 * ============================================================================
 *
 * Dimensions remain expressions.
 *
 * This allows:
 *
 *     N
 *     N + 1
 *     rows
 *     cols * factor
 *     symbolic_dimension
 *
 * without imposing a machine-oriented bound.
 *
 * Semantic validation determines whether the expression is a legal dimension.
 *
 * ============================================================================
 */

numericalDimensionExpression
    : expression
    ;


/* ============================================================================
 * 45. NUMERICAL PRECISION EXPRESSION
 * ============================================================================
 *
 * Precision is represented as a source expression where the language surface
 * requires one.
 *
 * The grammar does not prescribe a maximum precision.
 *
 * ============================================================================
 */

numericalPrecisionExpression
    : expression
    ;


/* ============================================================================
 * 46. NUMERICAL SCALE EXPRESSION
 * ============================================================================
 *
 * Scale is likewise semantic.
 *
 * ============================================================================
 */

numericalScaleExpression
    : expression
    ;


/* ============================================================================
 * 47. NUMERICAL TOLERANCE EXPRESSION
 * ============================================================================
 *
 * A tolerance is an ordinary expression.
 *
 * This permits:
 *
 *     literal tolerances;
 *     named tolerances;
 *     symbolic tolerances;
 *     configuration-derived values;
 *     compile-time expressions.
 *
 * ============================================================================
 */

numericalToleranceExpression
    : expression
    ;


/* ============================================================================
 * 48. NUMERICAL DOMAIN EXPRESSION
 * ============================================================================
 *
 * This rule exists specifically for semantic consumers that need to classify
 * an expression as a numerical-domain expression.
 *
 * ============================================================================
 */

numericalDomainExpression
    : expression
    ;


/* ============================================================================
 * 49. NUMERICAL CALLABLE NAME
 * ============================================================================
 *
 * Operation identity is semantic.
 *
 * ============================================================================
 */

numericalCallableName
    : IDENTIFIER
    ;


/* ============================================================================
 * 50. NUMERICAL OPERATION INVOCATION
 * ============================================================================
 *
 * This is the generic numerical operation boundary.
 *
 * It intentionally permits future numerical operations without grammar
 * modification.
 *
 * ============================================================================
 */

numericalOperationInvocation
    : numericalCallableName
      LPAREN
      numericalArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 51. NUMERICAL OPERATION ARGUMENT
 * ============================================================================
 */

numericalOperationArgument
    : expression
    ;


/* ============================================================================
 * 52. NUMERICAL OPERATION ARGUMENT LIST
 * ============================================================================
 */

numericalOperationArgumentList
    : numericalOperationArgument
      (COMMA numericalOperationArgument)*
      COMMA?
    ;


/* ============================================================================
 * 53. NUMERICAL SYMBOLIC OPERATION
 * ============================================================================
 *
 * Symbolic operations are ordinary callable forms.
 *
 * The semantic system determines whether the operation is:
 *
 *     numerical;
 *     symbolic;
 *     differentiable;
 *     evaluable;
 *     compile-time evaluable;
 *     runtime evaluable.
 *
 * ============================================================================
 */

numericalSymbolicOperation
    : numericalCallableName
      LPAREN
      numericalSymbolicOperationArguments?
      RPAREN
    ;


/* ============================================================================
 * 54. NUMERICAL SYMBOLIC OPERATION ARGUMENTS
 * ============================================================================
 */

numericalSymbolicOperationArguments
    : numericalSymbolicValue
      (COMMA numericalSymbolicValue)*
      COMMA?
    ;


/* ============================================================================
 * 55. NUMERICAL DOMAIN STATEMENT
 * ============================================================================
 *
 * This rule deliberately composes existing numerical forms rather than
 * creating another statement hierarchy.
 *
 * ============================================================================
 */

numericalDomainStatement
    : numericalExpressionStatement
    | numericalDeclaration
    | numericalAssignment
    ;


/* ============================================================================
 * 56. NUMERICAL BLOCK
 * ============================================================================
 *
 * A numerical block is a structural grouping boundary.
 *
 * Statement ownership remains in the statement subsystem.
 *
 * ============================================================================
 */

numericalBlock
    : LBRACE
      numericalDomainStatement*
      RBRACE
    ;


/* ============================================================================
 * 57. NUMERICAL VALUE INITIALIZER
 * ============================================================================
 */

numericalValueInitializer
    : ASSIGN
      expression
    ;


/* ============================================================================
 * 58. NUMERICAL DECLARATION NAME
 * ============================================================================
 */

numericalDeclarationName
    : IDENTIFIER
    ;


/* ============================================================================
 * 59. NUMERICAL REFERENCE PATH
 * ============================================================================
 *
 * Qualified numerical names are deliberately kept identifier-based.
 *
 * ============================================================================
 */

numericalReferencePath
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


/* ============================================================================
 * 60. NUMERICAL QUALIFIED REFERENCE
 * ============================================================================
 */

numericalQualifiedReference
    : numericalReferencePath
    ;


/*
 * ============================================================================
 * COMPLETION / INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * 1. It compiles as an ANTLR4 parser grammar against the canonical
 *    ZamaniLexer vocabulary.
 *
 * 2. It does not declare lexer rules.
 *
 * 3. It does not introduce duplicate lexical token aliases.
 *
 * 4. It uses IDENTIFIER consistently with the canonical lexical contract.
 *
 * 5. It does not redefine general arithmetic/operator precedence.
 *
 * 6. It does not construct or depend directly on classical IR.
 *
 * 7. It does not construct or depend directly on quantum::ir.
 *
 * 8. It does not depend on QEC, ZQN, scheduling, routing, hardware discovery,
 *    or runtime implementation.
 *
 * 9. Numeric precision, width, scale, overflow, promotion, representation,
 *    storage, and execution strategy remain semantic/compiler concerns.
 *
 * 10. No machine-oriented maximum exists in the grammar.
 *
 * 11. Numerical operation names remain extensible identifiers.
 *
 * 12. Symbolic numerical expressions remain syntactically representable.
 *
 * 13. Numerical ranges do not have a fixed size.
 *
 * 14. Numerical argument lists have no artificial finite maximum.
 *
 * 15. The same numerical source syntax remains valid independently of target
 *     architecture.
 *
 * 16. Positive tests cover:
 *
 *         integer literals
 *         floating literals
 *         numeric references
 *         symbolic expressions
 *         numerical calls
 *         reductions
 *         transformations
 *         ranges
 *         comprehensions
 *         declarations
 *         assignments
 *         qualified references
 *
 * 17. Negative tests cover:
 *
 *         malformed numeric expressions
 *         malformed ranges
 *         missing call delimiters
 *         malformed argument lists
 *         malformed declarations
 *         invalid punctuation
 *
 * 18. Boundary tests cover:
 *
 *         empty argument lists where legal
 *         one argument
 *         many arguments
 *         deeply composed expressions
 *         arbitrarily long symbolic ranges
 *         very large literal spellings
 *
 * 19. Cross-domain tests verify numerical expressions can participate in:
 *
 *         classical
 *         quantum
 *         hybrid
 *         HDL
 *         hardware
 *         distributed
 *         AI
 *         resource
 *         compile-time
 *
 * 20. Determinism tests verify identical token sequences produce identical
 *     parse structures.
 *
 * 21. Round-trip tests verify supported numerical syntax survives:
 *
 *         source
 *           -> lexer
 *           -> parser
 *           -> AST
 *           -> serializer/printer
 *           -> parser
 *
 *     without semantic loss.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_*
 *     fixed numerical widths
 *     fixed precision
 *     fixed vector size
 *     fixed matrix size
 *     fixed tensor rank
 *     fixed argument count
 *     fixed operation count
 *     CPU assumptions
 *     GPU assumptions
 *     FPGA assumptions
 *     ASIC assumptions
 *     accelerator assumptions
 *     device identifiers
 *     physical addresses
 *     machine topology
 *     runtime resource counts
 *
 * Any practical implementation limit belongs to an explicit parser/compiler/
 * resource/runtime policy and must never become a hidden language rule.
 *
 * ============================================================================
 */