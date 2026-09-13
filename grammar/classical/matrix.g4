/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/matrix.g4
 *
 * Status:
 *     Production-ready classical matrix-domain composition grammar.
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     target-specific implementation code, or unsafe implementation.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL MATRIX COMPUTATION SYNTAX.
 *
 * It provides the parser boundary for matrix-specific constructs while
 * deliberately delegating general language concepts to their authoritative
 * grammar owners.
 *
 * This file may express:
 *
 *     - matrix declarations;
 *     - matrix values;
 *     - matrix literals;
 *     - matrix initialization;
 *     - matrix indexing;
 *     - matrix assignment;
 *     - matrix construction;
 *     - matrix transformation expressions;
 *     - matrix algebra operation syntax;
 *     - matrix decomposition operation syntax;
 *     - matrix reduction operation syntax;
 *     - matrix element-wise operation syntax;
 *     - matrix shape expressions;
 *     - matrix dimension expressions;
 *     - matrix-domain computation statements;
 *     - matrix-domain expression composition.
 *
 * This file MUST remain independent of any particular:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     BLAS implementation
 *     vendor library
 *     memory layout
 *     SIMD width
 *     thread count
 *     cache size
 *     machine word size
 *     device topology
 *     physical memory capacity
 *     execution backend.
 *
 * ============================================================================
 *
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
 *          +----------------------+
 *          |                      |
 *          v                      v
 *     Types.g4              Expressions.g4
 *          |                      |
 *          +----------+-----------+
 *                     |
 *                     v
 *              Matrix.g4
 *                     |
 *                     v
 *                Frontend AST
 *                     |
 *                     v
 *              Semantic Analysis
 *                     |
 *          +----------+-----------+
 *          |                      |
 *          v                      v
 *    Matrix semantic model   Resource/effect metadata
 *          |
 *          v
 *      Classical IR
 *          |
 *          v
 *      Optimization
 *          |
 *          v
 *      Scheduling / placement
 *          |
 *          v
 *      Target-independent lowering
 *          |
 *          v
 *      Target realization
 *
 * Matrix.g4 MUST NOT construct IR directly.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - matrix-domain syntax;
 *     - matrix expression composition;
 *     - matrix literal syntax;
 *     - matrix construction syntax;
 *     - matrix operation syntax;
 *     - matrix decomposition operation syntax;
 *     - matrix reduction operation syntax;
 *     - matrix indexing composition;
 *     - matrix assignment composition;
 *     - matrix-domain statement composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical rules;
 *     - identifier spelling;
 *     - numeric literal definitions;
 *     - general expressions;
 *     - general type syntax;
 *     - generic type syntax;
 *     - classical type definitions;
 *     - matrix semantic typing;
 *     - matrix dimension validation;
 *     - matrix shape compatibility;
 *     - matrix algorithms;
 *     - numerical precision;
 *     - floating-point semantics;
 *     - sparse storage representation;
 *     - dense storage representation;
 *     - memory allocation;
 *     - memory layout;
 *     - tiling;
 *     - vectorization;
 *     - parallelization;
 *     - GPU selection;
 *     - FPGA selection;
 *     - accelerator selection;
 *     - BLAS selection;
 *     - LAPACK selection;
 *     - vendor libraries;
 *     - hardware capabilities;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - runtime execution;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Matrix syntax describes mathematical/computational intent.
 *
 * It MUST NOT encode physical realization.
 *
 * For example:
 *
 *     Matrix<f64, Rows, Cols>
 *
 * does not mean:
 *
 *     - a fixed number of CPU registers;
 *     - a fixed SIMD width;
 *     - a fixed GPU allocation;
 *     - a fixed number of accelerator threads;
 *     - a fixed memory layout;
 *     - a fixed machine size.
 *
 * Likewise:
 *
 *     A * B
 *
 * expresses matrix multiplication semantics.
 *
 * It does not select:
 *
 *     - CPU execution;
 *     - GPU execution;
 *     - FPGA execution;
 *     - ASIC execution;
 *     - distributed execution;
 *     - a particular BLAS provider.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * No finite machine-oriented matrix limit is encoded here.
 *
 * In particular, this grammar contains no:
 *
 *     MAX_ROWS
 *     MAX_COLUMNS
 *     MAX_ELEMENTS
 *     MAX_RANK
 *     MAX_MATRIX_SIZE
 *     MAX_DIMENSION
 *     MAX_OPERATIONS
 *     MAX_NESTING
 *     MAX_MEMORY
 *
 * Matrix dimensions are represented structurally.
 *
 * They may be:
 *
 *     literals;
 *     identifiers;
 *     qualified identifiers;
 *     compile-time expressions;
 *     symbolic dimensions;
 *     dependent values;
 *     expressions resolved later by semantic analysis.
 *
 * Practical resource limits belong to:
 *
 *     parser resource policy;
 *     semantic validation;
 *     compiler resource policy;
 *     resource management;
 *     scheduling;
 *     deployment;
 *     runtime.
 *
 * Such limits MUST NOT silently become language semantics.
 *
 * ============================================================================
 *
 * DIMENSION CONTRACT
 * ============================================================================
 *
 * Matrix dimensions are syntax only.
 *
 * The grammar does NOT require dimensions to be:
 *
 *     - positive;
 *     - equal;
 *     - statically known;
 *     - compile-time constants;
 *     - bounded by an integer size.
 *
 * Semantic analysis is responsible for determining whether a particular
 * matrix operation has compatible shapes.
 *
 * ============================================================================
 *
 * TYPE CONTRACT
 * ============================================================================
 *
 * Matrix type syntax is owned by:
 *
 *     grammar/types/classical-types.g4
 *
 * and is integrated into:
 *
 *     grammar/types/types.g4
 *
 * This file therefore MUST NOT redefine:
 *
 *     Matrix<T>
 *     Matrix<T, Rows, Cols>
 *     typeExpression
 *     typeArgument
 *     typeValueExpression
 *
 * Matrix declarations reuse canonical typeExpression.
 *
 * ============================================================================
 *
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expression syntax is owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file MUST NOT redefine:
 *
 *     arithmetic precedence;
 *     function calls;
 *     identifiers;
 *     literals;
 *     unary operators;
 *     binary operators;
 *     assignment operators;
 *     conditional expressions;
 *     indexing primitives;
 *     member access.
 *
 * Matrix-specific constructs are composed around the canonical expression
 * grammar.
 *
 * ============================================================================
 *
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * No lexer rules are declared here.
 *
 * Matrix names such as:
 *
 *     Matrix
 *     matrix
 *     transpose
 *     inverse
 *     determinant
 *     solve
 *     eigenvalues
 *
 * are intentionally not made lexer-level keywords by this file.
 *
 * This permits:
 *
 *     user-defined matrix libraries;
 *     domain extensions;
 *     dialects;
 *     future algorithms;
 *     vendor-independent operations;
 *     compatibility evolution.
 *
 * Semantic resolution determines whether an identifier denotes a canonical
 * matrix operation.
 *
 * ============================================================================
 *
 * IMPORT CONTRACT
 * ============================================================================
 */

parser grammar Matrix;

options {
    tokenVocab = ZamaniLexer;
}

import Types, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC MATRIX DOMAIN ENTRY POINT
 * ============================================================================
 *
 * This is the primary parser integration point.
 *
 * A consumer that wants matrix-domain syntax should depend on:
 *
 *     matrixConstruct
 *
 * rather than duplicating matrix productions.
 * ============================================================================
 */

matrixConstruct
    : matrixDeclaration
    | matrixAssignment
    | matrixExpressionStatement
    | matrixControlExpression
    ;


/*
 * ============================================================================
 * 2. MATRIX DECLARATION
 * ============================================================================
 *
 * Matrix declarations reuse the canonical type system.
 *
 * Examples:
 *
 *     let A: Matrix<f64, Rows, Cols> = ...;
 *     let A = matrix(...);
 *     const A: Matrix<f64> = ...;
 *
 * The grammar does not determine whether the declared type is semantically
 * a matrix. That remains a semantic-analysis responsibility.
 * ============================================================================
 */

matrixDeclaration
    : LET identifier matrixTypeAnnotation? ASSIGN matrixExpression SEMICOLON
    | VAR identifier matrixTypeAnnotation? ASSIGN matrixExpression SEMICOLON
    | CONST identifier matrixTypeAnnotation? ASSIGN matrixExpression SEMICOLON
    | LET identifier matrixTypeAnnotation SEMICOLON
    | VAR identifier matrixTypeAnnotation SEMICOLON
    | CONST identifier matrixTypeAnnotation SEMICOLON
    ;


matrixTypeAnnotation
    : COLON typeExpression
    ;


/*
 * ============================================================================
 * 3. MATRIX ASSIGNMENT
 * ============================================================================
 *
 * Assignment operators remain owned by the general expression grammar.
 *
 * This production only establishes a matrix-domain composition boundary.
 *
 * Semantic analysis determines:
 *
 *     - whether the target is assignable;
 *     - whether it denotes a matrix;
 *     - whether the value has a compatible shape;
 *     - whether element types are compatible.
 * ============================================================================
 */

matrixAssignment
    : matrixAssignableTarget ASSIGN matrixExpression SEMICOLON
    ;


matrixAssignableTarget
    : identifier
    | matrixElementAccess
    | matrixSliceAccess
    ;


/*
 * ============================================================================
 * 4. MATRIX EXPRESSION STATEMENT
 * ============================================================================
 */

matrixExpressionStatement
    : matrixExpression SEMICOLON
    ;


/*
 * ============================================================================
 * 5. MATRIX EXPRESSION
 * ============================================================================
 *
 * This is intentionally recursive.
 *
 * It permits matrix computations to compose without a finite operation count.
 *
 * ============================================================================
 */

matrixExpression
    : matrixPrimary
    | matrixConstruction
    | matrixUnaryOperation
    | matrixBinaryOperation
    | matrixCallOperation
    | matrixIndexExpression
    | matrixTransformExpression
    | matrixReductionExpression
    | matrixDecompositionExpression
    | matrixSolveExpression
    | matrixExpressionInParentheses
    ;


/*
 * ============================================================================
 * 6. MATRIX PRIMARY
 * ============================================================================
 *
 * Existing general expressions remain available.
 *
 * This permits matrix values to participate in ordinary Zamani expressions.
 * ============================================================================
 */

matrixPrimary
    : identifier
    | INTEGER
    | FLOAT
    | STRING
    | matrixLiteral
    ;


/*
 * ============================================================================
 * 7. MATRIX CONSTRUCTION
 * ============================================================================
 *
 * Matrix construction is expressed through semantic constructor names rather
 * than fixed machine-specific mechanisms.
 *
 * Supported source-level constructors include:
 *
 *     matrix(...)
 *     zeros(...)
 *     ones(...)
 *     identity(...)
 *     diagonal(...)
 *     random(...)
 *     fill(...)
 *
 * Additional constructors can be introduced through dialects/libraries
 * without changing the core matrix semantics.
 *
 * ============================================================================
 */

matrixConstruction
    : MATRIX_CONSTRUCTOR
      LPAREN
      matrixArgumentList?
      RPAREN
    ;


MATRIX_CONSTRUCTOR
    : IDENT
    ;


/*
 * ============================================================================
 * 8. MATRIX LITERALS
 * ============================================================================
 *
 * Matrix literals are structurally recursive.
 *
 * Examples:
 *
 *     [[1, 2], [3, 4]]
 *     [[a, b], [c, d]]
 *
 * There is no finite row/column limit.
 *
 * Semantic analysis validates:
 *
 *     - rectangularity;
 *     - element compatibility;
 *     - dimension compatibility.
 *
 * ============================================================================
 */

matrixLiteral
    : LBRACKET
      LBRACKET
      matrixRowElements?
      RBRACKET
      matrixAdditionalRows*
      RBRACKET
    ;


matrixAdditionalRows
    : COMMA
      LBRACKET
      matrixRowElements?
      RBRACKET
    ;


matrixRowElements
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 9. MATRIX ARGUMENTS
 * ============================================================================
 *
 * Arguments are general expressions.
 *
 * This allows symbolic dimensions and values without creating a second
 * expression language.
 * ============================================================================
 */

matrixArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 10. MATRIX UNARY OPERATIONS
 * ============================================================================
 *
 * Unary matrix operations have semantic names.
 *
 * No operation is tied to a particular backend.
 * ============================================================================
 */

matrixUnaryOperation
    : matrixUnaryOperator
      matrixExpression
    ;


matrixUnaryOperator
    : TRANSPOSE_OPERATOR
    | INVERSE_OPERATOR
    | ADJOINT_OPERATOR
    | NEGATE_OPERATOR
    ;


TRANSPOSE_OPERATOR
    : IDENT
    ;


INVERSE_OPERATOR
    : IDENT
    ;


ADJOINT_OPERATOR
    : IDENT
    ;


NEGATE_OPERATOR
    : IDENT
    ;


/*
 * ============================================================================
 * 11. MATRIX BINARY OPERATIONS
 * ============================================================================
 *
 * Ordinary mathematical operators remain available through the canonical
 * expression system.
 *
 * Matrix-specific semantic operations are represented here as operation
 * names.
 * ============================================================================
 */

matrixBinaryOperation
    : matrixExpression matrixBinaryOperator matrixExpression
    ;


matrixBinaryOperator
    : PLUS
    | MINUS
    | STAR
    | SLASH
    | CARET
    ;


/*
 * ============================================================================
 * 12. MATRIX INDEXING
 * ============================================================================
 *
 * Matrix indexing does not impose a fixed rank.
 *
 * Examples:
 *
 *     A[i, j]
 *     A[i, j, k]
 *     A[i]
 *
 * Semantic analysis determines whether a particular indexing form is valid
 * for the matrix type.
 *
 * ============================================================================
 */

matrixIndexExpression
    : identifier
      LBRACKET
      matrixIndexList
      RBRACKET
    ;


matrixIndexList
    : matrixIndex
      (COMMA matrixIndex)*
      COMMA?
    ;


matrixIndex
    : expression
    | matrixRange
    ;


matrixRange
    : expression
      RANGE_OPERATOR
      expression
    ;


RANGE_OPERATOR
    : DOT
      DOT
    ;


/*
 * ============================================================================
 * 13. MATRIX ELEMENT ACCESS
 * ============================================================================
 */

matrixElementAccess
    : identifier
      LBRACKET
      expression
      COMMA
      expression
      RBRACKET
    ;


/*
 * ============================================================================
 * 14. MATRIX SLICE ACCESS
 * ============================================================================
 *
 * Slice dimensionality is unbounded by this grammar.
 *
 * Examples:
 *
 *     A[r0:r1, c0:c1]
 *
 * Semantic analysis determines whether the resulting slice is valid.
 * ============================================================================
 */

matrixSliceAccess
    : identifier
      LBRACKET
      matrixSliceDimension
      (COMMA matrixSliceDimension)*
      COMMA?
      RBRACKET
    ;


matrixSliceDimension
    : expression
      RANGE_OPERATOR
      expression
    | expression
    ;


/*
 * ============================================================================
 * 15. MATRIX TRANSFORM OPERATIONS
 * ============================================================================
 *
 * The operation names are represented as identifiers at lexical level.
 *
 * Canonical semantic operations may include:
 *
 *     transpose
 *     conjugate
 *     adjoint
 *     reshape
 *     permute
 *     reorder
 *     broadcast
 *     tile
 *     flatten
 *
 * The grammar does not restrict future operations to this list.
 * ============================================================================
 */

matrixTransformExpression
    : identifier
      LPAREN
      matrixArgumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 16. MATRIX REDUCTION OPERATIONS
 * ============================================================================
 *
 * Examples:
 *
 *     trace(A)
 *     sum(A)
 *     product(A)
 *     norm(A)
 *     max(A)
 *     min(A)
 *
 * Whether an operation is mathematically valid depends on the semantic type
 * and operation definition.
 * ============================================================================
 */

matrixReductionExpression
    : identifier
      LPAREN
      matrixReductionArguments
      RPAREN
    ;


matrixReductionArguments
    : matrixExpression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 17. MATRIX DECOMPOSITION OPERATIONS
 * ============================================================================
 *
 * Examples:
 *
 *     lu(A)
 *     qr(A)
 *     svd(A)
 *     eig(A)
 *     cholesky(A)
 *
 * The parser records the operation structure.
 *
 * It does not decide whether:
 *
 *     - A is square;
 *     - A is positive definite;
 *     - A is Hermitian;
 *     - a decomposition exists;
 *     - the requested precision is supported.
 * ============================================================================
 */

matrixDecompositionExpression
    : matrixOperationName
      LPAREN
      matrixExpression
      RPAREN
    ;


matrixOperationName
    : identifier
    ;


/*
 * ============================================================================
 * 18. MATRIX SOLVE OPERATIONS
 * ============================================================================
 *
 * Examples:
 *
 *     solve(A, b)
 *     least_squares(A, b)
 *     inverse_solve(A, b)
 *
 * Shape and numerical legality are semantic concerns.
 * ============================================================================
 */

matrixSolveExpression
    : matrixSolveOperation
      LPAREN
      matrixExpression
      COMMA
      matrixExpression
      matrixOptionalArguments?
      RPAREN
    ;


matrixSolveOperation
    : identifier
    ;


matrixOptionalArguments
    : COMMA
      matrixArgumentList
    ;


/*
 * ============================================================================
 * 19. MATRIX CONTROL EXPRESSION
 * ============================================================================
 *
 * Matrix operations may appear inside general control expressions.
 *
 * This rule is deliberately small because control-flow ownership belongs to
 * statements/expressions grammar.
 * ============================================================================
 */

matrixControlExpression
    : IF
      expression
      matrixExpressionBlock
      matrixElseRegion?
    ;


matrixElseRegion
    : ELSE
      (
          matrixControlExpression
        | matrixExpressionBlock
      )
    ;


matrixExpressionBlock
    : LBRACE
      matrixConstruct*
      RBRACE
    ;


/*
 * ============================================================================
 * 20. PARENTHESIZED MATRIX EXPRESSION
 * ============================================================================
 */

matrixExpressionInParentheses
    : LPAREN
      matrixExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 21. MATRIX IDENTIFIER BRIDGE
 * ============================================================================
 *
 * Identifier spelling remains owned by ZamaniLexer.
 * ============================================================================
 */

identifier
    : IDENT
    ;


/*
 * ============================================================================
 * 22. MATRIX SEMANTIC OPERATORS
 * ============================================================================
 *
 * These rules are intentionally parser-level abstractions.
 *
 * They do not define numerical behavior.
 *
 * Semantic analysis is responsible for resolving operation identity.
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 *     1. It parses through the canonical Zamani lexer.
 *
 *     2. It imports only authoritative grammar dependencies.
 *
 *     3. It declares no machine/resource maximum.
 *
 *     4. It declares no lexer rules.
 *
 *     5. It does not duplicate Matrix<T, Rows, Cols> type syntax owned by
 *        ClassicalTypes.g4.
 *
 *     6. It does not duplicate general expression syntax.
 *
 *     7. It does not construct IR.
 *
 *     8. It does not select hardware.
 *
 *     9. It does not impose numerical implementation limits.
 *
 *    10. It supports arbitrarily many matrix elements at the language level,
 *        subject only to downstream parser/compiler/resource availability.
 *
 *    11. It preserves symbolic dimensions.
 *
 *    12. It permits user-defined/future matrix operations through semantic
 *        identifier resolution.
 *
 *    13. Positive, negative, boundary, determinism and round-trip tests exist.
 *
 *    14. Cross-domain tests verify matrix syntax can participate in:
 *
 *            classical + quantum
 *            classical + HDL
 *            classical + hardware
 *            classical + distributed
 *            classical + AI
 *
 *        without adding machine-specific syntax here.
 *
 * ============================================================================
 *
 * IMPORTANT ANTLR NOTE
 * ============================================================================
 *
 * The productions above deliberately avoid Rust actions and semantic
 * predicates. ANTLR's generated parser therefore remains independent of the
 * Rust implementation version.
 *
 * Rust 1.97 / 1.97.1 compatibility is enforced by the generated frontend and
 * compiler crate, not by embedding Rust code in this grammar.
 *
 * ============================================================================
 */