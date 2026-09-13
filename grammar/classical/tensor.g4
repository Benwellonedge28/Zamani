/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/classical/tensor.g4
 *
 * Status:
 *     Production-ready tensor-domain parser grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust actions.
 *     This grammar contains no semantic predicates.
 *     This grammar contains no unsafe implementation.
 *     Runtime/compiler implementations MUST use safe Rust only.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL TENSOR COMPUTATION SYNTAX.
 *
 * It provides the parser boundary for tensor-domain constructs while
 * delegating general language concepts to their authoritative grammar owners.
 *
 * Tensor syntax expresses computational and mathematical intent.
 *
 * It MUST NOT encode:
 *
 *     - CPU count;
 *     - GPU count;
 *     - accelerator count;
 *     - thread count;
 *     - SIMD width;
 *     - register count;
 *     - cache size;
 *     - memory capacity;
 *     - device identity;
 *     - device topology;
 *     - physical tensor storage limits;
 *     - vendor library selection;
 *     - BLAS implementation;
 *     - accelerator implementation;
 *     - execution placement;
 *     - scheduling policy;
 *     - target architecture.
 *
 * Those concerns belong to semantic analysis, resource analysis,
 * optimization, scheduling, target lowering, deployment and runtime.
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
 *     ZamaniParser / grammar composition
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     Expressions                    Types
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                  Tensor syntax
 *                        |
 *                        v
 *                   Frontend AST
 *                        |
 *                        v
 *              Semantic / type analysis
 *                        |
 *          +-------------+-------------+
 *          |                           |
 *          v                           v
 *   Tensor semantic model       Resource/effect metadata
 *          |
 *          v
 *      Classical IR
 *          |
 *          v
 *     Optimization
 *          |
 *          v
 *     Scheduling / placement
 *          |
 *          v
 *   Target-independent lowering
 *          |
 *          v
 *    Target realization
 *
 * Tensor.g4 MUST NOT construct IR directly.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - tensor-domain syntax;
 *     - tensor declarations as a domain composition;
 *     - tensor literals;
 *     - tensor construction syntax;
 *     - tensor indexing syntax;
 *     - tensor slicing syntax;
 *     - tensor shape syntax;
 *     - tensor reshape syntax;
 *     - tensor transpose/permutation syntax;
 *     - tensor contraction syntax;
 *     - tensor reduction syntax;
 *     - tensor element-wise operation syntax;
 *     - tensor algebraic operation syntax;
 *     - tensor decomposition operation syntax;
 *     - tensor-domain expression composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical syntax;
 *     - identifier spelling;
 *     - numeric literal syntax;
 *     - general expressions;
 *     - general operators;
 *     - general type syntax;
 *     - generic type syntax;
 *     - type checking;
 *     - tensor shape validation;
 *     - rank validation;
 *     - dimension compatibility;
 *     - numerical precision;
 *     - tensor storage;
 *     - dense/sparse representation;
 *     - memory allocation;
 *     - memory layout;
 *     - tiling;
 *     - vectorization;
 *     - automatic differentiation;
 *     - device selection;
 *     - accelerator selection;
 *     - GPU selection;
 *     - distributed execution;
 *     - scheduling;
 *     - optimization;
 *     - classical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Tensor syntax describes mathematical/computational intent.
 *
 * Example:
 *
 *     A * B
 *
 * means tensor/matrix-compatible multiplication according to the semantic
 * types of A and B.
 *
 * It does NOT mean:
 *
 *     - execute on CPU;
 *     - execute on GPU;
 *     - use a particular accelerator;
 *     - use a particular BLAS implementation;
 *     - use a particular memory layout;
 *     - use a particular number of threads.
 *
 * Likewise:
 *
 *     reshape(A, shape)
 *
 * describes a semantic transformation.
 *
 * The compiler may realize it using:
 *
 *     - a view;
 *     - a copy;
 *     - a layout transformation;
 *     - a fused operation;
 *     - distributed storage;
 *     - accelerator-specific lowering;
 *
 * without changing the source program.
 *
 * ============================================================================
 *
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no finite tensor-size limit.
 *
 * It does not define:
 *
 *     MAX_RANK
 *     MAX_DIMENSIONS
 *     MAX_ELEMENTS
 *     MAX_TENSOR_SIZE
 *     MAX_SHAPE
 *     MAX_AXIS_COUNT
 *     MAX_OPERATIONS
 *
 * Tensor rank and dimensions are represented structurally.
 *
 * Dimensions may be:
 *
 *     - integer literals;
 *     - identifiers;
 *     - qualified identifiers;
 *     - expressions;
 *     - symbolic dimensions;
 *     - compile-time values;
 *     - dependent values;
 *     - values resolved from generic constraints.
 *
 * Practical limits belong to resource policy and execution environments,
 * not to the source grammar.
 *
 * ============================================================================
 *
 * TYPE CONTRACT
 * ============================================================================
 *
 * Tensor type syntax belongs to the canonical type system.
 *
 * This file MUST NOT redefine:
 *
 *     typeExpression
 *     typeArgument
 *     genericParameters
 *     generic constraints
 *
 * A semantic tensor type may ultimately be represented by the canonical
 * classical type system, for example:
 *
 *     Tensor<T>
 *     Tensor<T, Shape>
 *
 * but this file does not impose those semantic representations.
 *
 * ============================================================================
 *
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expressions are owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file reuses the canonical `expression` rule.
 *
 * It MUST NOT redefine:
 *
 *     arithmetic precedence;
 *     calls;
 *     identifiers;
 *     literals;
 *     unary operators;
 *     binary operators;
 *     assignment;
 *     conditionals;
 *     general indexing;
 *     member access.
 *
 * ============================================================================
 *
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * Canonical lexical tokens are provided by:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * No lexer rules are declared in this file.
 *
 * In particular, tensor operations such as:
 *
 *     reshape
 *     transpose
 *     permute
 *     contract
 *     einsum
 *     reduce
 *     broadcast
 *     concatenate
 *     stack
 *
 * are NOT made reserved lexical keywords by this grammar.
 *
 * They remain ordinary identifiers so that:
 *
 *     libraries;
 *     user-defined functions;
 *     dialects;
 *     future operations;
 *     domain extensions;
 *
 * do not require changes to the lexical grammar.
 *
 * ============================================================================
 *
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical dependencies:
 *
 *     ZamaniLexer
 *     Types
 *     Expressions
 *
 * Canonical consumers:
 *
 *     grammar/classical/classical.g4
 *     grammar/expressions/expressions.g4
 *     grammar/antlr/ZamaniParser.g4
 *     frontend AST
 *     semantic analysis
 *     classical IR lowering
 *
 * Tensor.g4 MUST remain below semantic analysis.
 *
 * It MUST NOT depend on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     scheduling
 *     routing
 *     hardware discovery
 *     runtime state.
 *
 * ============================================================================
 */

parser grammar Tensor;

options {
    tokenVocab = ZamaniLexer;
}

import Types, Expressions;


/* ============================================================================
 * 1. PUBLIC TENSOR ENTRY POINT
 * ============================================================================
 *
 * Consumers should integrate tensor syntax through `tensorConstruct`.
 *
 * ============================================================================
 */

tensorConstruct
    : tensorDeclaration
    | tensorAssignment
    | tensorExpressionStatement
    ;


/* ============================================================================
 * 2. TENSOR DECLARATION
 * ============================================================================
 *
 * Declaration syntax reuses the canonical type system.
 *
 * Examples:
 *
 *     let A: Tensor<f64> = tensor(...);
 *     let A: Tensor<f64, S> = tensor(...);
 *
 * The grammar does not validate the tensor type or shape.
 *
 * ============================================================================
 */

tensorDeclaration
    : LET identifier tensorTypeAnnotation? ASSIGN tensorExpression SEMICOLON
    | VAR identifier tensorTypeAnnotation? ASSIGN tensorExpression SEMICOLON
    | CONST identifier tensorTypeAnnotation? ASSIGN tensorExpression SEMICOLON
    | LET identifier tensorTypeAnnotation SEMICOLON
    | VAR identifier tensorTypeAnnotation SEMICOLON
    | CONST identifier tensorTypeAnnotation SEMICOLON
    ;


tensorTypeAnnotation
    : COLON typeExpression
    ;


/* ============================================================================
 * 3. TENSOR ASSIGNMENT
 * ============================================================================
 */

tensorAssignment
    : tensorAssignableTarget ASSIGN tensorExpression SEMICOLON
    ;


tensorAssignableTarget
    : identifier
    | tensorElementAccess
    | tensorSliceAccess
    ;


/* ============================================================================
 * 4. TENSOR EXPRESSION STATEMENT
 * ============================================================================
 */

tensorExpressionStatement
    : tensorExpression SEMICOLON
    ;


/* ============================================================================
 * 5. TENSOR EXPRESSION
 * ============================================================================
 *
 * Tensor expressions are intentionally compositional.
 *
 * No finite operation depth or tensor rank is encoded here.
 *
 * ============================================================================
 */

tensorExpression
    : tensorPrimary
    | tensorLiteral
    | tensorConstruction
    | tensorUnaryExpression
    | tensorBinaryExpression
    | tensorCallExpression
    | tensorIndexExpression
    | tensorSliceExpression
    | tensorShapeExpression
    | tensorReshapeExpression
    | tensorPermutationExpression
    | tensorContractionExpression
    | tensorReductionExpression
    | tensorElementwiseExpression
    | tensorDecompositionExpression
    | tensorExpressionInParentheses
    ;


/* ============================================================================
 * 6. TENSOR PRIMARY
 * ============================================================================
 *
 * The primary form delegates ordinary values to the canonical expression
 * system. This permits tensors to participate in larger Zamani expressions.
 *
 * ============================================================================
 */

tensorPrimary
    : identifier
    | literal
    ;


/* ============================================================================
 * 7. TENSOR PARENTHESES
 * ============================================================================
 */

tensorExpressionInParentheses
    : LPAREN tensorExpression RPAREN
    ;


/* ============================================================================
 * 8. TENSOR LITERALS
 * ============================================================================
 *
 * A tensor literal is recursively constructed from nested bracketed values.
 *
 * Examples:
 *
 *     [1, 2, 3]
 *
 *     [[1, 2], [3, 4]]
 *
 *     [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
 *
 * Rank is determined semantically from nesting.
 *
 * No maximum rank is imposed.
 *
 * ============================================================================
 */

tensorLiteral
    : LBRACKET tensorLiteralBody? RBRACKET
    ;


tensorLiteralBody
    : tensorLiteralElement
      (COMMA tensorLiteralElement)*
      COMMA?
    ;


tensorLiteralElement
    : expression
    | tensorLiteral
    ;


/* ============================================================================
 * 9. TENSOR CONSTRUCTION
 * ============================================================================
 *
 * Construction is intentionally identifier-based.
 *
 * Examples:
 *
 *     tensor(...)
 *     zeros(...)
 *     ones(...)
 *     fill(...)
 *     identity(...)
 *     random(...)
 *
 * The grammar does not reserve these names.
 *
 * Semantic resolution determines whether a call is a canonical tensor
 * constructor, a library function, or a user-defined operation.
 *
 * ============================================================================
 */

tensorConstruction
    : identifier
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


tensorArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 10. GENERAL TENSOR CALL
 * ============================================================================
 *
 * A tensor call remains a normal Zamani expression-level call composition.
 *
 * This rule exists as a domain boundary so tensor semantic analysis can
 * recognize tensor-producing calls without creating a second call grammar.
 *
 * ============================================================================
 */

tensorCallExpression
    : identifier
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 11. TENSOR UNARY OPERATIONS
 * ============================================================================
 *
 * Operation names are identifiers rather than reserved keywords.
 *
 * Typical semantic operations include:
 *
 *     transpose
 *     adjoint
 *     negate
 *     conjugate
 *     abs
 *     normalize
 *
 * ============================================================================
 */

tensorUnaryExpression
    : tensorUnaryOperator tensorExpression
    ;


tensorUnaryOperator
    : identifier
    ;


/* ============================================================================
 * 12. TENSOR BINARY OPERATIONS
 * ============================================================================
 *
 * Operators remain ordinary Zamani operators.
 *
 * Semantic analysis determines whether an operator represents:
 *
 *     element-wise arithmetic;
 *     tensor contraction;
 *     matrix multiplication;
 *     scalar-tensor multiplication;
 *     broadcasting;
 *     another domain-defined operation.
 *
 * ============================================================================
 */

tensorBinaryExpression
    : tensorExpression tensorBinaryOperator tensorExpression
    ;


tensorBinaryOperator
    : PLUS
    | MINUS
    | STAR
    | SLASH
    | CARET
    ;


/* ============================================================================
 * 13. TENSOR INDEXING
 * ============================================================================
 *
 * Index arity is deliberately unbounded.
 *
 * Examples:
 *
 *     A[i]
 *     A[i, j]
 *     A[i, j, k]
 *     A[i, j, k, l]
 *
 * The grammar imposes no fixed rank.
 *
 * ============================================================================
 */

tensorIndexExpression
    : identifier
      LBRACKET
      tensorIndexList
      RBRACKET
    ;


tensorIndexList
    : tensorIndex
      (COMMA tensorIndex)*
      COMMA?
    ;


tensorIndex
    : expression
    | tensorRange
    ;


/* ============================================================================
 * 14. TENSOR ELEMENT ACCESS
 * ============================================================================
 *
 * This is a semantic convenience rule for assignment and downstream AST
 * classification.
 *
 * ============================================================================
 */

tensorElementAccess
    : identifier
      LBRACKET
      tensorIndexList
      RBRACKET
    ;


/* ============================================================================
 * 15. TENSOR RANGES
 * ============================================================================
 */

tensorRange
    : expression DOT_DOT expression
    | expression DOT_DOT_EQ expression
    ;


/* ============================================================================
 * 16. TENSOR SLICING
 * ============================================================================
 *
 * Examples:
 *
 *     A[i..j]
 *     A[i..=j, k..l]
 *     A[:, j]
 *
 * Open-ended and wildcard-like slice syntax should be introduced only through
 * canonical expression/range rules when the language specification defines
 * them. This file therefore keeps the base form expression-based.
 *
 * ============================================================================
 */

tensorSliceExpression
    : identifier
      LBRACKET
      tensorSliceList
      RBRACKET
    ;


tensorSliceAccess
    : tensorSliceExpression
    ;


tensorSliceList
    : tensorSlice
      (COMMA tensorSlice)*
      COMMA?
    ;


tensorSlice
    : tensorRange
    | expression
    ;


/* ============================================================================
 * 17. TENSOR SHAPE
 * ============================================================================
 *
 * Shape expressions describe logical tensor dimensions.
 *
 * They do not describe physical memory.
 *
 * Examples:
 *
 *     shape(A)
 *     rank(A)
 *     dim(A, i)
 *
 * Operation identity is semantic, not lexical.
 *
 * ============================================================================
 */

tensorShapeExpression
    : identifier
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 18. TENSOR RESHAPE
 * ============================================================================
 *
 * Reshape syntax is represented as a normal identifier call:
 *
 *     reshape(A, shape)
 *
 * The grammar does not determine whether the operation is:
 *
 *     - view-preserving;
 *     - copying;
 *     - layout-changing;
 *     - distributed;
 *     - accelerator-backed.
 *
 * ============================================================================
 */

tensorReshapeExpression
    : identifier
      LPAREN
      tensorArgumentList
      RPAREN
    ;


/* ============================================================================
 * 19. TENSOR PERMUTATION
 * ============================================================================
 *
 * Supports semantic constructs such as:
 *
 *     transpose(A)
 *     permute(A, axes)
 *
 * Axis validation belongs to semantic analysis.
 *
 * ============================================================================
 */

tensorPermutationExpression
    : identifier
      LPAREN
      tensorArgumentList
      RPAREN
    ;


/* ============================================================================
 * 20. TENSOR CONTRACTION
 * ============================================================================
 *
 * Supports semantic tensor contractions such as:
 *
 *     contract(A, B, ...)
 *     einsum(...)
 *
 * The grammar does not impose a fixed contraction rank.
 *
 * ============================================================================
 */

tensorContractionExpression
    : identifier
      LPAREN
      tensorArgumentList
      RPAREN
    ;


/* ============================================================================
 * 21. TENSOR REDUCTION
 * ============================================================================
 *
 * Examples include semantic operations such as:
 *
 *     sum(A)
 *     product(A)
 *     min(A)
 *     max(A)
 *     mean(A)
 *
 * These remain ordinary identifiers and are resolved semantically.
 *
 * ============================================================================
 */

tensorReductionExpression
    : identifier
      LPAREN
      tensorArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 22. TENSOR ELEMENT-WISE OPERATIONS
 * ============================================================================
 *
 * Element-wise semantics are determined by type and broadcasting rules.
 *
 * This grammar only provides a syntactic composition point.
 *
 * ============================================================================
 */

tensorElementwiseExpression
    : tensorExpression tensorElementwiseOperator tensorExpression
    ;


tensorElementwiseOperator
    : PLUS
    | MINUS
    | STAR
    | SLASH
    ;


/* ============================================================================
 * 23. TENSOR DECOMPOSITION
 * ============================================================================
 *
 * Decompositions such as:
 *
 *     SVD
 *     QR
 *     LU
 *     eigendecomposition
 *
 * are semantic/library operations rather than fixed language keywords.
 *
 * ============================================================================
 */

tensorDecompositionExpression
    : identifier
      LPAREN
      tensorArgumentList
      RPAREN
    ;


/* ============================================================================
 * 24. TENSOR SHAPE SPECIFICATION
 * ============================================================================
 *
 * Shape specifications are structural and can contain symbolic expressions.
 *
 * Examples:
 *
 *     <M, N>
 *     <Batch, Height, Width, Channels>
 *
 * This rule is intentionally generic.
 *
 * ============================================================================
 */

tensorShapeSpecification
    : LESS_THAN
      tensorDimensionList?
      GREATER_THAN
    ;


tensorDimensionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 25. TENSOR TYPE PARAMETERS
 * ============================================================================
 *
 * This rule provides a syntax boundary for tensor-related generic parameters
 * without redefining the canonical generic type system.
 *
 * ============================================================================
 */

tensorTypeParameters
    : LESS_THAN
      tensorTypeParameterList?
      GREATER_THAN
    ;


tensorTypeParameterList
    : tensorTypeParameter
      (COMMA tensorTypeParameter)*
      COMMA?
    ;


tensorTypeParameter
    : typeExpression
    | expression
    ;


/* ============================================================================
 * 26. TENSOR AXIS EXPRESSIONS
 * ============================================================================
 *
 * Axis identity is semantic.
 *
 * It may be:
 *
 *     - positional;
 *     - symbolic;
 *     - named;
 *     - compile-time computed.
 *
 * No finite axis limit is encoded.
 *
 * ============================================================================
 */

tensorAxisExpression
    : expression
    ;


tensorAxisList
    : tensorAxisExpression
      (COMMA tensorAxisExpression)*
      COMMA?
    ;


/* ============================================================================
 * 27. TENSOR NAMED ARGUMENT
 * ============================================================================
 *
 * This supports future extensible tensor operations without introducing
 * operation-specific grammar.
 *
 * ============================================================================
 */

tensorNamedArgument
    : identifier COLON expression
    ;


/* ============================================================================
 * 28. TENSOR OPERATION ARGUMENT
 * ============================================================================
 */

tensorOperationArgument
    : tensorNamedArgument
    | expression
    ;


tensorOperationArgumentList
    : tensorOperationArgument
      (COMMA tensorOperationArgument)*
      COMMA?
    ;


/* ============================================================================
 * 29. TENSOR OPERATION INVOCATION
 * ============================================================================
 *
 * This is the principal extension point for tensor libraries and dialects.
 *
 * Example:
 *
 *     operation(A, B)
 *
 * or:
 *
 *     operation(input: A, axes: axes)
 *
 * No operation inventory is hard-coded here.
 *
 * ============================================================================
 */

tensorOperationInvocation
    : identifier
      LPAREN
      tensorOperationArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 30. TENSOR DOMAIN COMPOSITION
 * ============================================================================
 *
 * This rule intentionally accepts canonical expressions around tensor
 * constructs rather than forcing all tensor computation into a closed grammar.
 *
 * This is essential for POCO-REAF because tensor functionality can evolve
 * without requiring the language grammar to be rewritten for every future
 * tensor algorithm.
 *
 * ============================================================================
 */

tensorDomainExpression
    : tensorExpression
    | expression
    ;


/* ============================================================================
 * 31. TENSOR RESOURCE-NEUTRALITY CONTRACT
 * ============================================================================
 *
 * The grammar contains no syntax for:
 *
 *     device = GPU0
 *     gpu_count = ...
 *     threads = ...
 *     cores = ...
 *     memory = ...
 *     vector_width = ...
 *     accelerator_id = ...
 *
 * If a tensor program needs resource requirements, it must use the canonical
 * Zamani resource/capability grammar.
 *
 * Tensor semantics therefore remain portable across:
 *
 *     embedded systems;
 *     CPUs;
 *     multicore CPUs;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     clusters;
 *     distributed systems;
 *     cloud systems;
 *     future architectures.
 *
 * ============================================================================
 */


/* ============================================================================
 * 32. SEMANTIC BOUNDARY CONTRACT
 * ============================================================================
 *
 * This grammar emits syntax.
 *
 * Semantic analysis MUST subsequently determine:
 *
 *     - tensor rank;
 *     - tensor element type;
 *     - shape;
 *     - dimension compatibility;
 *     - broadcasting;
 *     - contraction compatibility;
 *     - reduction legality;
 *     - indexing legality;
 *     - slice legality;
 *     - reshape legality;
 *     - aliasing;
 *     - ownership;
 *     - memory effects;
 *     - numerical semantics;
 *     - resource requirements.
 *
 * The grammar must never silently encode those semantic decisions.
 *
 * ============================================================================
 */


/* ============================================================================
 * 33. IR BOUNDARY
 * ============================================================================
 *
 * Tensor.g4 MUST NOT directly reference or construct:
 *
 *     classical IR;
 *     quantum::ir;
 *     QEC;
 *     ZQN;
 *     scheduling IR;
 *     hardware IR;
 *     runtime objects.
 *
 * The expected pipeline is:
 *
 *     tensor syntax
 *         |
 *         v
 *     frontend AST
 *         |
 *         v
 *     semantic tensor model
 *         |
 *         v
 *     canonical classical/data IR
 *         |
 *         +--> optimization
 *         +--> scheduling
 *         +--> resource realization
 *         +--> target lowering
 *
 * Quantum IR remains a separate canonical semantic boundary.
 *
 * ============================================================================
 */


/* ============================================================================
 * 34. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded actions;
 *     - no random behavior;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment inspection;
 *     - no target discovery;
 *     - no mutable global state.
 *
 * Parsing the same source with the same grammar and lexer must therefore
 * produce the same parse structure.
 *
 * ============================================================================
 */


/* ============================================================================
 * 35. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     - fixed tensor rank limits;
 *     - fixed dimension limits;
 *     - fixed element counts;
 *     - fixed device counts;
 *     - fixed accelerator counts;
 *     - fixed thread counts;
 *     - fixed memory sizes;
 *     - fixed hardware topology;
 *     - fixed vendor APIs.
 *
 * Numeric literals appearing in source programs are DATA, not grammar limits.
 *
 * ============================================================================
 */


/* ============================================================================
 * 36. COMPATIBILITY
 * ============================================================================
 *
 * Tensor.g4 is additive.
 *
 * It does not modify:
 *
 *     - lexical token definitions;
 *     - general expression precedence;
 *     - canonical type syntax;
 *     - quantum syntax;
 *     - hardware syntax;
 *     - runtime semantics.
 *
 * Existing general expressions remain valid.
 *
 * Tensor-specific semantic names remain ordinary identifiers so future
 * libraries and dialects can extend tensor computation without changing the
 * lexical contract.
 *
 * ============================================================================
 */


/* ============================================================================
 * 37. TEST CONTRACT
 * ============================================================================
 *
 * Positive examples:
 *
 *     let x = [1, 2, 3];
 *     let A = [[1, 2], [3, 4]];
 *     let T = [[[1], [2]], [[3], [4]]];
 *
 *     let y = reshape(A, shape);
 *     let z = transpose(A);
 *     let c = contract(A, B);
 *     let r = reduce(A, axis);
 *
 * Indexing:
 *
 *     A[i];
 *     A[i, j];
 *     A[i, j, k];
 *
 * Slicing:
 *
 *     A[i..j];
 *     A[i..=j, k..l];
 *
 * Symbolic dimensions:
 *
 *     let A: Tensor<f64> = tensor(M, N);
 *
 * Nested tensor construction:
 *
 *     let T = tensor(shape, values);
 *
 * Negative tests must include:
 *
 *     - malformed brackets;
 *     - malformed commas;
 *     - malformed ranges;
 *     - malformed calls;
 *     - incomplete index expressions;
 *     - incomplete declarations;
 *     - incomplete assignments.
 *
 * Boundary tests must include:
 *
 *     - scalar tensors;
 *     - vectors;
 *     - matrices;
 *     - deeply nested tensors;
 *     - symbolic dimensions;
 *     - very large source representations generated by tests.
 *
 * Scalability tests must verify that no source grammar rule imposes:
 *
 *     - fixed rank;
 *     - fixed dimension;
 *     - fixed element count;
 *     - fixed accelerator size.
 *
 * Integration tests must combine tensor syntax with:
 *
 *     - classical expressions;
 *     - functions;
 *     - generics;
 *     - concurrency;
 *     - resource requirements;
 *     - quantum/classical hybrid programs;
 *     - hardware-facing programs.
 *
 * ============================================================================
 */


/* ============================================================================
 * 38. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] ANTLR accepts the grammar.
 *
 * [ ] No lexer rules are declared here.
 *
 * [ ] Canonical ZamaniLexer tokens are used.
 *
 * [ ] Canonical Expressions grammar is reused.
 *
 * [ ] Canonical Types grammar is reused.
 *
 * [ ] Tensor syntax is represented without finite machine limits.
 *
 * [ ] Tensor rank is not hard-coded.
 *
 * [ ] Tensor dimensions are not hard-coded.
 *
 * [ ] Tensor element counts are not hard-coded.
 *
 * [ ] Hardware is not selected by tensor syntax.
 *
 * [ ] Tensor operations remain extensible.
 *
 * [ ] Semantic validation remains outside the grammar.
 *
 * [ ] Classical IR lowering remains outside the grammar.
 *
 * [ ] quantum::ir remains outside the grammar.
 *
 * [ ] QEC remains outside the grammar.
 *
 * [ ] ZQN remains outside the grammar.
 *
 * [ ] Scheduling remains outside the grammar.
 *
 * [ ] Runtime execution remains outside the grammar.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Round-trip tests pass where applicable.
 *
 * [ ] Rust compiler integration remains safe Rust with no unsafe code.
 *
 * [ ] No subsequent grammar file is required to redefine the tensor
 *     semantic contract established here.
 *
 * ============================================================================
 */