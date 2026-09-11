/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/antlr/Mathematics.g4
 *
 * Role:
 *     Mathematical syntax extension for the canonical Zamani parser.
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Lexer:
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Intended parser composition:
 *
 *     ZamaniParser
 *          |
 *          +--> Core
 *          |
 *          +--> Mathematics
 *          |
 *          +--> Quantum
 *          |
 *          +--> Types
 *          +--> Effects
 *          +--> ...
 *
 * Language:
 *     Zamani
 *
 * Compiler:
 *     ZUTC
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     The Zamani compiler implementation MUST use safe Rust only.
 *     No Rust `unsafe` is required or permitted.
 *
 * ============================================================================
 *
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * Mathematics is a LANGUAGE DOMAIN, not a second programming language.
 *
 * This grammar therefore does NOT attempt to enumerate every mathematical
 * algorithm or library operation.
 *
 * In particular, Mathematics.g4 does NOT make these into an exhaustive
 * keyword catalogue:
 *
 *     fft
 *     ifft
 *     svd
 *     qr
 *     lu
 *     cholesky
 *     determinant
 *     eigenvalues
 *     gradient
 *     jacobian
 *     hessian
 *     integrate
 *     differentiate
 *     minimize
 *     maximize
 *     regression
 *     pca
 *     ...
 *
 * Such names are ordinary identifiers and can therefore be:
 *
 *     built-in semantic intrinsics;
 *     standard-library functions;
 *     user-defined functions;
 *     imported functions;
 *     symbolic operations;
 *     accelerator operations;
 *     quantum-enhanced mathematical operations;
 *     future operations unknown when this grammar was written.
 *
 * This is essential for POCO-REAF:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Anywhere
 *     Forever
 *
 * The grammar must not establish a finite ceiling on mathematical capability.
 *
 * ============================================================================
 *
 * MATHEMATICAL MODEL
 * ============================================================================
 *
 * Mathematics in Zamani is represented primarily through:
 *
 *     values
 *     types
 *     generic types
 *     expressions
 *     function calls
 *     operators
 *     comprehensions / bindings
 *     symbolic values
 *     dimensions and shapes
 *     semantic capabilities
 *
 * For example, all of these can remain ordinary source-level names:
 *
 *     Vector
 *     Matrix
 *     Tensor
 *     Complex
 *     Rational
 *     Polynomial
 *     Symbol
 *     Distribution
 *     FFT
 *     SVD
 *     optimize
 *     differentiate
 *     integrate
 *
 * The semantic/type system determines what those names mean.
 *
 * ============================================================================
 *
 * NO HARD-CODED MACHINE ASSUMPTIONS
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     - fixed vector lengths;
 *     - fixed matrix dimensions;
 *     - fixed tensor rank;
 *     - fixed numeric precision;
 *     - fixed integer width;
 *     - fixed floating-point width;
 *     - fixed SIMD width;
 *     - fixed CPU;
 *     - fixed GPU;
 *     - fixed accelerator;
 *     - fixed memory capacity;
 *     - fixed quantum device;
 *     - fixed parallelism;
 *     - fixed backend;
 *     - fixed numerical algorithm.
 *
 * Dimensions are source-level values or type-level expressions.
 *
 * Resource validation belongs to semantic analysis and resource planning.
 *
 * ============================================================================
 *
 * PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     Frontend AST
 *       |
 *       v
 *     Name / symbol resolution
 *       |
 *       v
 *     Type inference / checking
 *       |
 *       v
 *     Mathematical semantic analysis
 *       |
 *       v
 *     Canonical IR
 *       |
 *       +--> mathematical semantics
 *       +--> classical/data semantics
 *       +--> quantum::ir where mathematics is used by quantum computation
 *       |
 *       v
 *     Optimization
 *       |
 *       +--> algebraic simplification
 *       +--> constant propagation
 *       +--> symbolic transformation
 *       +--> numerical optimization
 *       +--> vectorization
 *       +--> tensor lowering
 *       |
 *       v
 *     Target-independent lowering
 *       |
 *       v
 *     Target-specific realization
 *
 * ============================================================================
 *
 * IMPORTANT
 * ============================================================================
 *
 * Mathematics.g4 owns SYNTAX ONLY.
 *
 * It does not own:
 *
 *     - numerical algorithms;
 *     - numerical precision;
 *     - overflow policy;
 *     - floating-point semantics;
 *     - symbolic simplification;
 *     - differentiation implementation;
 *     - integration implementation;
 *     - optimization implementation;
 *     - linear algebra implementation;
 *     - tensor contraction implementation;
 *     - FFT implementation;
 *     - statistical implementation;
 *     - hardware vectorization;
 *     - GPU execution;
 *     - quantum execution.
 *
 * Those belong downstream.
 *
 * ============================================================================
 */

parser grammar Mathematics;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * MATHEMATICAL DOMAIN ENTRY POINT
 * ============================================================================
 *
 * This rule is intentionally a composition point.
 *
 * The root parser may include mathDeclaration as an item/declaration
 * alternative when the corresponding source form is enabled.
 *
 * Mathematical computation itself does not require a special declaration.
 * Ordinary Zamani expressions and functions are already sufficient for:
 *
 *     let x = sin(theta);
 *     let y = fft(signal);
 *     let z = A * x;
 *
 * Consequently, this grammar only owns explicit mathematical constructs
 * that cannot be represented cleanly as ordinary Core syntax.
 * ========================================================================== */

mathDeclaration
    : mathBindingDeclaration
    | symbolicBindingDeclaration
    | mathDefinitionDeclaration
    ;


/* ============================================================================
 * MATHEMATICAL BINDINGS
 * ============================================================================
 *
 * Mathematical values use the normal Zamani binding model.
 *
 * Examples:
 *
 *     let v: Vector<Real> = ...
 *     let A: Matrix<Real> = ...
 *     let T: Tensor<Real> = ...
 *
 * The actual semantic types are ordinary qualified/generic names.
 *
 * No dimension is hard-coded here.
 * ========================================================================== */

mathBindingDeclaration
    : LET MUT? identifier COLON mathType ASSIGN expression SEMI?
    | LET MUT? identifier COLON mathType SEMI?
    | VAR MUT? identifier COLON mathType ASSIGN expression SEMI?
    | VAR MUT? identifier COLON mathType SEMI?
    ;


/* ============================================================================
 * SYMBOLIC BINDINGS
 * ============================================================================
 *
 * Symbolic values are ordinary semantic values.
 *
 * Example:
 *
 *     symbolic x;
 *
 * is intentionally NOT made a keyword construct here.
 *
 * Instead, the canonical form is represented by a normal typed binding such
 * as:
 *
 *     let x: Symbol = ...
 *
 * or through an imported symbolic constructor.
 *
 * This rule exists only for an explicitly mathematical definition form.
 * ========================================================================== */

symbolicBindingDeclaration
    : identifier COLON symbolicType ASSIGN expression SEMI?
    ;


/* ============================================================================
 * MATHEMATICAL DEFINITIONS
 * ============================================================================
 *
 * A mathematical definition is a named function.
 *
 * It reuses the normal function declaration infrastructure semantically.
 *
 * The grammar deliberately does not create separate syntax for:
 *
 *     theorem
 *     lemma
 *     axiom
 *     proof
 *     derivative
 *     integral
 *     matrix algorithm
 *     numerical method
 *
 * unless and until those become formally specified language constructs.
 *
 * This prevents Mathematics.g4 from becoming an unbounded catalogue.
 * ========================================================================== */

mathDefinitionDeclaration
    : FN identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      effectClause?
      contractClause*
      block
    ;


/* ============================================================================
 * MATHEMATICAL TYPES
 * ============================================================================
 *
 * Mathematical type names are intentionally open.
 *
 * Examples:
 *
 *     Vector<Real>
 *     Matrix<Complex>
 *     Tensor<Real, Shape>
 *     Polynomial<Rational>
 *     Distribution<Real>
 *     Manifold<Real>
 *     HilbertSpace<Complex>
 *
 * Nothing in this grammar limits the number of generic parameters.
 *
 * The meaning of a mathematical type belongs to semantic/type analysis.
 * ========================================================================== */

mathType
    : qualifiedName
    | qualifiedName LT mathTypeArgumentList GT
    | tupleType
    | arrayType
    | sliceType
    | referenceType
    | parenthesizedMathType
    ;


mathTypeArgumentList
    : mathTypeArgument (COMMA mathTypeArgument)* COMMA?
    ;


mathTypeArgument
    : mathType
    | expression
    ;


parenthesizedMathType
    : LPAREN mathType RPAREN
    ;


symbolicType
    : qualifiedName
    | qualifiedName LT mathTypeArgumentList GT
    ;


/* ============================================================================
 * DIMENSION / SHAPE EXPRESSIONS
 * ============================================================================
 *
 * Mathematical dimensions are values, not fixed grammar constants.
 *
 * Examples:
 *
 *     Vector<Real, n>
 *     Matrix<Real, m, n>
 *     Tensor<Real, d0, d1, d2>
 *
 * The semantic layer decides whether dimensions are:
 *
 *     compile-time constants;
 *     runtime values;
 *     symbolic values;
 *     dependent values;
 *     inferred values;
 *     constrained values.
 *
 * There is no MAX_DIMENSION, MAX_RANK or MAX_SIZE here.
 * ========================================================================== */

shapeExpression
    : shapeAtom
    | shapeExpression COMMA shapeAtom
    ;


shapeAtom
    : expression
    ;


shapeList
    : LBRACKET shapeExpression? RBRACKET
    ;


/* ============================================================================
 * EXPLICIT MATHEMATICAL VALUE TYPES
 * ============================================================================
 *
 * These rules provide structural syntax for mathematical type descriptions
 * without reserving the type names as lexer keywords.
 *
 * Thus:
 *
 *     Vector
 *     Matrix
 *     Tensor
 *
 * remain identifiers at the lexical level.
 *
 * This is important because user libraries and future mathematical domains
 * must be able to introduce new mathematical types without changing the
 * lexer.
 * ========================================================================== */

vectorType
    : qualifiedName LT mathType (COMMA shapeExpression)? GT
    ;


matrixType
    : qualifiedName LT mathType
      (COMMA shapeExpression)*
      GT
    ;


tensorType
    : qualifiedName LT mathType
      (COMMA shapeExpression)*
      GT
    ;


/* ============================================================================
 * VECTOR / MATRIX / TENSOR LITERALS
 * ============================================================================
 *
 * Zamani Core already provides array expressions:
 *
 *     [a, b, c]
 *
 * and nested arrays:
 *
 *     [[a, b], [c, d]]
 *
 * Therefore Mathematics.g4 does NOT introduce competing literal syntax.
 *
 * The semantic layer can recognize these structures as:
 *
 *     vector values;
 *     matrix values;
 *     tensor values;
 *     ordinary arrays;
 *     other aggregate values.
 *
 * This keeps one source representation and avoids grammar duplication.
 * ========================================================================== */

mathAggregateExpression
    : arrayExpression
    ;


/* ============================================================================
 * MATHEMATICAL INDEXING
 * ============================================================================
 *
 * Core indexing already provides:
 *
 *     value[index]
 *
 * Mathematics may use arbitrary index expressions:
 *
 *     A[i]
 *     A[i, j]
 *     T[i, j, k]
 *
 * The existing Core grammar currently permits one expression inside brackets.
 *
 * This extension therefore provides a mathematical multi-index form.
 *
 * There is deliberately no fixed maximum rank.
 * ========================================================================== */

mathIndexExpression
    : expression LBRACKET mathIndexList RBRACKET
    ;


mathIndexList
    : expression (COMMA expression)* COMMA?
    ;


/* ============================================================================
 * MATHEMATICAL SLICES
 * ============================================================================
 *
 * Mathematical and numerical programs frequently need slices.
 *
 * A slice uses the existing range operators rather than introducing another
 * dimension language.
 *
 * Examples:
 *
 *     a[0..n]
 *     A[i, 0..m]
 *
 * ========================================================================== */

mathSlice
    : mathIndexAtom
    ;


mathIndexAtom
    : expression
    | rangeExpression
    ;


/* ============================================================================
 * MATHEMATICAL COMPREHENSIONS
 * ============================================================================
 *
 * Comprehensions are useful for:
 *
 *     vectors;
 *     matrices;
 *     tensors;
 *     symbolic collections;
 *     probability spaces;
 *     generated numerical domains.
 *
 * They are intentionally generic.
 *
 * No fixed number of dimensions is encoded.
 *
 * Example conceptual forms:
 *
 *     [f(i) for i in domain]
 *
 *     [f(i, j) for i in rows, j in columns]
 *
 * The semantic layer determines whether the result is an array, vector,
 * matrix, tensor, symbolic object, lazy sequence, distributed collection,
 * or another mathematical value.
 * ========================================================================== */

mathComprehension
    : LBRACKET
      expression
      comprehensionClause+
      RBRACKET
    ;


comprehensionClause
    : FOR pattern IN expression
    | WHEN expression
    ;


/* ============================================================================
 * SET / DOMAIN COMPREHENSIONS
 * ============================================================================
 *
 * Mathematical domains can be represented without introducing a finite list
 * of mathematical set constructors into the lexer.
 *
 * Example:
 *
 *     { x | x in domain, condition }
 *
 * This is syntax only. The semantic type may be:
 *
 *     Set
 *     Relation
 *     Predicate
 *     SymbolicDomain
 *     MeasureSpace
 *     ConstraintDomain
 *     another semantic object.
 * ========================================================================== */

mathSetComprehension
    : LBRACE expression PIPE mathSetPredicate RBRACE
    ;


mathSetPredicate
    : expression
    ;


/* ============================================================================
 * EQUATION / CONSTRAINT EXPRESSIONS
 * ============================================================================
 *
 * Equality and comparison already belong to Core expressions.
 *
 * Mathematics does not introduce another equality token.
 *
 * A mathematical constraint is therefore simply an expression whose semantic
 * type/effect is validated later.
 * ========================================================================== */

mathConstraint
    : expression
    ;


mathConstraintList
    : mathConstraint (COMMA mathConstraint)* COMMA?
    ;


/* ============================================================================
 * MATHEMATICAL RELATIONS
 * ============================================================================
 *
 * This is a semantic grouping rule rather than a new operator vocabulary.
 *
 * Supported source operators come from the canonical lexer/Core grammar:
 *
 *     ==
 *     !=
 *     <
 *     >
 *     <=
 *     >=
 *
 * ========================================================================== */

mathRelation
    : expression
    ;


/* ============================================================================
 * OPTIMIZATION PROBLEM SURFACE
 * ============================================================================
 *
 * Optimization is intentionally represented as a generic mathematical
 * expression plus constraints.
 *
 * It does NOT hard-code:
 *
 *     gradient descent;
 *     Newton;
 *     simplex;
 *     interior point;
 *     simulated annealing;
 *     quantum optimization;
 *     a particular solver.
 *
 * The semantic layer chooses or resolves the requested optimization
 * capability.
 *
 * Canonical conceptual form:
 *
 *     minimize objective subject to constraints
 *
 *     maximize objective subject to constraints
 *
 * The words `minimize` and `maximize` are intentionally NOT lexer keywords.
 * They are identifiers resolved as mathematical semantic operations.
 *
 * ========================================================================== */

optimizationExpression
    : identifier
      expression
    ;


/* ============================================================================
 * FUNCTION APPLICATION FOR MATHEMATICAL OPERATIONS
 * ============================================================================
 *
 * Mathematics uses the same Core call syntax:
 *
 *     fft(signal)
 *     svd(A)
 *     determinant(A)
 *     gradient(f, x)
 *     integrate(f, x)
 *     solve(A, b)
 *
 * The operation name is an identifier.
 *
 * This rule exists to provide a stable semantic hook without adding an
 * exhaustive keyword list.
 * ========================================================================== */

mathCallExpression
    : qualifiedName LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * OPERATOR-BASED MATHEMATICS
 * ============================================================================
 *
 * Core owns the canonical operator precedence.
 *
 * Mathematics MUST NOT define a second precedence hierarchy.
 *
 * Consequently mathematical expressions reuse:
 *
 *     +
 *     -
 *     *
 *     /
 *     %
 *     **
 *     comparisons
 *     logical operators
 *     bitwise operators
 *     ranges
 *
 * according to the canonical Core grammar and semantic operator table.
 *
 * This rule is a composition point only.
 * ========================================================================== */

mathExpression
    : expression
    | mathComprehension
    | mathSetComprehension
    | mathIndexExpression
    | mathCallExpression
    ;


/* ============================================================================
 * SYMBOLIC EXPRESSION
 * ============================================================================
 *
 * A symbolic expression is structurally an ordinary expression.
 *
 * The semantic system determines whether an expression is:
 *
 *     numeric;
 *     symbolic;
 *     automatic-differentiation capable;
 *     differentiable;
 *     integrable;
 *     probabilistic;
 *     tensor-valued;
 *     quantum-valued;
 *     etc.
 * ========================================================================== */

symbolicExpression
    : expression
    ;


/* ============================================================================
 * SYMBOLIC REPLACEMENT
 * ============================================================================
 *
 * Generic substitution syntax:
 *
 *     substitute(expression, substitutions...)
 *
 * is intentionally an ordinary function call.
 *
 * No keyword is required.
 *
 * This rule exists only as an explicit semantic composition point.
 * ========================================================================== */

substitutionExpression
    : qualifiedName LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * CALCULUS EXPRESSIONS
 * ============================================================================
 *
 * Calculus operations remain ordinary callable operations.
 *
 * Examples:
 *
 *     differentiate(f, x)
 *     derivative(f, x)
 *     integrate(f, x)
 *     integral(f, x)
 *     limit(f, x, a)
 *     gradient(f, x)
 *     jacobian(f, x)
 *     hessian(f, x)
 *
 * There is deliberately no finite calculus keyword catalogue.
 * ========================================================================== */

calculusExpression
    : qualifiedName LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * LINEAR ALGEBRA EXPRESSIONS
 * ============================================================================
 *
 * Linear algebra operations are ordinary semantic calls.
 *
 * Examples:
 *
 *     transpose(A)
 *     inverse(A)
 *     determinant(A)
 *     trace(A)
 *     rank(A)
 *     eigenvalues(A)
 *     eigenvectors(A)
 *     svd(A)
 *     qr(A)
 *     solve(A, b)
 *
 * The grammar does not distinguish these names lexically.
 * ========================================================================== */

linearAlgebraExpression
    : qualifiedName LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * TENSOR EXPRESSIONS
 * ============================================================================
 *
 * Tensor operations may have arbitrary rank.
 *
 * Examples:
 *
 *     contract(T, ...)
 *     einsum(...)
 *     reshape(T, shape)
 *     transpose(T, ...)
 *     permute(T, ...)
 *     kron(A, B)
 *
 * No rank or dimension limit exists in this grammar.
 * ========================================================================== */

tensorExpression
    : qualifiedName LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * PROBABILITY / STATISTICS EXPRESSIONS
 * ============================================================================
 *
 * Statistical operations are ordinary mathematical calls.
 *
 * Examples:
 *
 *     mean(x)
 *     variance(x)
 *     covariance(x, y)
 *     correlation(x, y)
 *     sample(distribution, n)
 *     pdf(distribution, x)
 *     cdf(distribution, x)
 *
 * Distribution families are semantic names, not grammar keywords.
 * ========================================================================== */

probabilityExpression
    : qualifiedName LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * NUMERICAL / SIGNAL EXPRESSIONS
 * ============================================================================
 *
 * Numerical algorithms and signal-processing operations remain ordinary
 * callable semantic operations.
 *
 * Examples:
 *
 *     fft(x)
 *     ifft(x)
 *     convolve(x, kernel)
 *     interpolate(x, ...)
 *     filter(signal, ...)
 *
 * No finite operation inventory is encoded.
 * ========================================================================== */

numericalExpression
    : qualifiedName LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * MATHEMATICAL OBJECT EXPRESSION
 * ============================================================================
 *
 * This is the principal semantic composition rule.
 *
 * The parser can recognize a mathematical expression without deciding which
 * mathematical domain it belongs to.
 *
 * Semantic analysis determines whether it represents:
 *
 *     scalar;
 *     vector;
 *     matrix;
 *     tensor;
 *     polynomial;
 *     symbolic expression;
 *     probability distribution;
 *     function;
 *     operator;
 *     manifold;
 *     graph;
 *     optimization problem;
 *     quantum mathematical object;
 *     another supported mathematical abstraction.
 * ========================================================================== */

mathematicalObjectExpression
    : mathExpression
    ;


/* ============================================================================
 * MATHEMATICAL FUNCTION SIGNATURE
 * ============================================================================
 *
 * Mathematical functions use the normal Zamani function model.
 *
 * This rule exists so semantic tooling can identify a function as a
 * mathematical definition without introducing a separate function language.
 * ========================================================================== */

mathematicalFunctionSignature
    : FN identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      effectClause?
    ;


/* ============================================================================
 * MATHEMATICAL CONTRACT
 * ============================================================================
 *
 * Mathematical contracts reuse Core contracts.
 *
 * Examples of semantic properties that may eventually be expressed here:
 *
 *     domain constraints;
 *     codomain constraints;
 *     shape constraints;
 *     dimensional equality;
 *     positivity;
 *     invertibility;
 *     differentiability;
 *     continuity;
 *     conservation;
 *     numerical stability.
 *
 * The actual predicates remain ordinary expressions.
 * ========================================================================== */

mathematicalContract
    : contractClause
    ;


/* ============================================================================
 * DIMENSIONAL / SHAPE CONSTRAINTS
 * ============================================================================
 *
 * Shape relationships are semantic predicates.
 *
 * Example conceptual constraint:
 *
 *     shape(A, 0) == shape(B, 1)
 *
 * The grammar does not know what `shape`, `rows`, `columns`, `rank`, etc.
 * mean. They are ordinary semantic operations.
 * ========================================================================== */

shapeConstraint
    : expression
    ;


/* ============================================================================
 * UNIT / DIMENSION EXPRESSIONS
 * ============================================================================
 *
 * Physical units and dimensional analysis must not be confused with machine
 * widths.
 *
 * Units may therefore be represented as ordinary semantic type/value
 * expressions:
 *
 *     Quantity<Real, Meter>
 *     Quantity<Real, Second>
 *     Quantity<Real, Meter / Second>
 *
 * The semantic system owns dimensional consistency.
 * ========================================================================== */

unitExpression
    : mathType
    ;


/* ============================================================================
 * EXACT / SYMBOLIC NUMERICAL EXPRESSIONS
 * ============================================================================
 *
 * Exact arithmetic is a semantic property.
 *
 * The grammar does not force:
 *
 *     f32;
 *     f64;
 *     arbitrary precision;
 *     fixed precision;
 *     software floating point;
 *     hardware floating point.
 *
 * Numeric literals continue to use the canonical lexer.
 * ========================================================================== */

exactExpression
    : expression
    ;


/* ============================================================================
 * MATHEMATICAL DOMAIN QUALIFIER
 * ============================================================================
 *
 * This permits tooling and semantic analysis to attach a domain interpretation
 * to an expression without introducing domain-specific keywords.
 *
 * Example:
 *
 *     @symbolic
 *     @numerical
 *     @exact
 *     @probabilistic
 *
 * The annotation syntax itself belongs to Core/lexer.
 *
 * This grammar only provides the mathematical semantic hook.
 * ========================================================================== */

mathDomainExpression
    : expression
    ;


/* ============================================================================
 * MATHEMATICAL STATEMENT
 * ============================================================================
 *
 * A mathematical statement is intentionally expression-oriented.
 *
 * This prevents Mathematics.g4 from competing with Core.statement.
 *
 * Mathematical operations such as:
 *
 *     solve(...)
 *     optimize(...)
 *     fft(...)
 *     differentiate(...)
 *
 * can therefore occur anywhere an expression is valid.
 * ========================================================================== */

mathStatement
    : expression SEMI?
    ;


/* ============================================================================
 * MATHEMATICAL MODULE CONTENT
 * ============================================================================
 *
 * Mathematical modules use ordinary Zamani items.
 *
 * No separate mathematical namespace model is introduced here.
 * ========================================================================== */

mathItem
    : mathDeclaration
    | mathStatement
    ;


/* ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The root Zamani parser SHOULD import this parser grammar and expose only
 * the mathematical rules that are part of the canonical language version.
 *
 * Conceptually:
 *
 *     parser grammar ZamaniParser;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 *     import Core, Mathematics, Quantum, Types, Effects, ...;
 *
 * The exact import set belongs to the root parser.
 *
 * Mathematics.g4 MUST NOT define:
 *
 *     program
 *     compilationUnit
 *     item
 *     statement
 *     expression
 *     typeExpression
 *     identifier
 *     literal
 *
 * when those rules are owned by Core.
 *
 * This prevents duplicate parser authorities.
 * ========================================================================== */


/* ============================================================================
 * AST / SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * Mathematics.g4 produces parser contexts only.
 *
 * The frontend AST should normalize mathematical syntax into the canonical
 * mathematical semantic representation.
 *
 * It must NOT create backend-specific mathematical instructions at parse time.
 *
 * Conceptually:
 *
 *     Mathematics.g4
 *           |
 *           v
 *     Mathematical AST
 *           |
 *           v
 *     semantic resolution
 *           |
 *           v
 *     canonical IR
 *
 * Mathematical expressions that participate in quantum computation must be
 * lowered through the same canonical semantic boundary used by the rest of
 * Zamani. In particular, quantum-related mathematics must not bypass
 * `quantum::ir`.
 *
 * ============================================================================
 */


/* ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar intentionally contains NO:
 *
 *     MAX_VECTOR_SIZE
 *     MAX_MATRIX_ROWS
 *     MAX_MATRIX_COLUMNS
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_SYMBOLS
 *     MAX_EXPRESSION_DEPTH
 *     MAX_POLYNOMIAL_DEGREE
 *     MAX_VARIABLES
 *     MAX_OPERANDS
 *     MAX_OPTIMIZATION_CONSTRAINTS
 *     MAX_ITERATIONS
 *
 * Resource exhaustion is not a language grammar rule.
 *
 * Resource requirements are determined by:
 *
 *     semantic analysis;
 *     type checking;
 *     resource analysis;
 *     compiler configuration;
 *     execution environment;
 *     target capabilities;
 *     runtime policy.
 *
 * A program may therefore scale from tiny systems to arbitrarily large
 * supported systems subject only to the resources and semantics of the
 * selected realization.
 *
 * ============================================================================
 */


/* ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Mathematics.g4 must remain deterministic under the canonical lexer.
 *
 * Mathematical names are identifiers.
 *
 * There is no parser dependence on:
 *
 *     a library being installed;
 *     a backend being available;
 *     a hardware device existing;
 *     a particular numerical implementation;
 *     a particular quantum processor;
 *     a particular floating-point format.
 *
 * Name resolution and capability checking happen after parsing.
 *
 * ============================================================================
 */


/* ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The historical monolithic grammar contained dedicated productions for
 * vectors, matrices, tensors, symbolic mathematics, calculus, statistics,
 * FFT, optimization and many named algorithms.
 *
 * Those historical forms are NOT automatically canonical.
 *
 * Compatibility should be implemented through one of:
 *
 *     SPEC_CHANGE
 *     IMPLEMENTATION_FIX
 *     EXPLICIT_COMPATIBILITY_RULE
 *
 * and tested explicitly.
 *
 * A historical spelling must never silently acquire new semantics merely
 * because it appears in an older grammar document.
 *
 * ============================================================================
 */


/* ============================================================================
 * IMPLEMENTATION STATUS
 * ============================================================================
 *
 * SPECIFIED:
 *
 *     - open mathematical naming model
 *     - expression-oriented mathematics
 *     - generic mathematical types
 *     - arbitrary shape expressions
 *     - mathematical comprehensions
 *     - mathematical indexing
 *     - symbolic expression hooks
 *     - mathematical contracts
 *     - optimization expression hook
 *     - probability/statistics expression hooks
 *     - numerical expression hooks
 *     - tensor expression hooks
 *
 * IMPLEMENTATION:
 *
 *     Must be verified against:
 *
 *       src/lexer.rs
 *       src/parser.rs
 *       src/ast/
 *       src/ir_gen.rs
 *       src/ir_verify.rs
 *       canonical mathematical semantic/IR modules
 *
 * TESTED:
 *
 *     Must include valid and invalid parser cases before conformance is
 *     declared complete.
 *
 * STABLE:
 *
 *     Only after canonical specification, parser, AST, semantic analysis,
 *     diagnostics and tests agree.
 *
 * ============================================================================
 */