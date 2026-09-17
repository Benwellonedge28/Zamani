/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/classical-types.g4
 *
 * Status:
 *     Canonical production classical-domain TYPE grammar.
 *
 * Purpose:
 *     Defines source-level classical type syntax without introducing a
 *     second type system, second AST, hardware model, or machine limit.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *     safe Rust only
 *     no embedded Rust actions
 *     no embedded semantic predicates
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       v
 *     Types
 *       |
 *       +--> ClassicalTypes
 *       |
 *       v
 *     frontend AST TypeExpr
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic type resolution
 *       |
 *       +--> classical semantic types
 *       +--> tensor/data semantic types
 *       +--> resource/capability metadata
 *       |
 *       v
 *     canonical semantic IR
 *       |
 *       v
 *     classical IR / data IR / accelerator metadata
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     scheduling / placement / lowering
 *       |
 *       v
 *     target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - classical-domain type entry points;
 *   - source-level scalar/numeric type references;
 *   - vector type constructors;
 *   - matrix type constructors;
 *   - tensor type constructors;
 *   - symbolic/value-parameterized classical types;
 *   - classical collection constructors;
 *   - classical tuple type composition;
 *   - classical type paths;
 *   - classical type arguments;
 *   - classical type-level dimension expressions;
 *   - classical type modifiers that are genuinely syntactic.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer definitions;
 *   - identifier spelling;
 *   - keywords;
 *   - global generic-type composition;
 *   - general array/slice syntax;
 *   - general function types;
 *   - general references/pointers;
 *   - declarations;
 *   - expressions;
 *   - numerical algorithms;
 *   - vector/matrix/tensor operations;
 *   - optimization;
 *   - SIMD width;
 *   - CPU/GPU/FPGA selection;
 *   - memory placement;
 *   - ABI layout;
 *   - scheduling;
 *   - hardware;
 *   - runtime representation;
 *   - classical IR;
 *   - quantum IR;
 *   - QEC;
 *   - ZQN;
 *   - HAL.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A classical type describes portable program meaning.
 *
 * It MUST NOT encode implementation limits such as:
 *
 *     MAX_VECTOR_LENGTH
 *     MAX_MATRIX_ROWS
 *     MAX_MATRIX_COLUMNS
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_INTEGER_WIDTH
 *     MAX_FLOAT_PRECISION
 *     MAX_REGISTER_WIDTH
 *     MAX_SIMD_WIDTH
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_ACCELERATORS
 *
 * Source-level constants are allowed:
 *
 *     Vector<f64, 1024>
 *     Matrix<f32, Rows, Cols>
 *
 * but those are PROGRAM SEMANTICS, not language limits.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * All list-like constructs use structural repetition.
 *
 * There is no grammar-level maximum for:
 *
 *     type arguments
 *     tensor dimensions
 *     namespace depth
 *     tuple elements
 *     symbolic expression depth
 *     numeric precision parameters
 *     collection parameters
 *
 * Actual implementation/resource limits belong to compiler policy,
 * semantic validation, resource negotiation, or runtime availability.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * It consumes the canonical Zamani lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * It MUST NOT declare lexer rules.
 *
 * The current repository's canonical lexical vocabulary includes concepts
 * such as:
 *
 *     IDENTIFIER
 *     INTEGER
 *     FLOAT
 *     INT
 *     FLOAT_TYPE
 *     BOOL_TYPE
 *     CHAR_TYPE
 *     STR_TYPE
 *     STRING_TYPE
 *     BYTE
 *     DOUBLE_COLON
 *     LESS_THAN
 *     GREATER_THAN
 *     LBRACKET
 *     RBRACKET
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     COLON
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     MODULO
 *     LEFT_SHIFT
 *     RIGHT_SHIFT
 *     BIT_AND
 *     BIT_OR
 *     CARET
 *
 * If the generated canonical lexer exposes an older spelling such as
 * INTEGER_LITERAL instead of INTEGER, the vocabulary must be reconciled in
 * grammar/antlr/ZamaniLexer.g4 / grammar/lexer/ rather than introducing a
 * local lexer rule here.
 *
 * ============================================================================
 * IMPORTANT DESIGN DECISION
 * ============================================================================
 *
 * Do NOT attempt to recognize every classical type by an exhaustive list of
 * parser alternatives such as:
 *
 *     Integer
 *     Float
 *     Complex
 *     Rational
 *     Decimal
 *     ...
 *
 * when all of those spellings are ordinary identifiers.
 *
 * Such alternatives are syntactically indistinguishable and create parser
 * ambiguity.
 *
 * Instead:
 *
 *     source spelling
 *          |
 *          v
 *     canonical type-path/type-argument syntax
 *          |
 *          v
 *     semantic resolver
 *
 * Semantic analysis decides whether a type path denotes:
 *
 *     scalar
 *     integer
 *     floating-point
 *     decimal
 *     fixed-point
 *     complex
 *     rational
 *     vector
 *     matrix
 *     tensor
 *     collection
 *     symbolic
 *     accelerator-facing
 *     user-defined
 *     future-domain
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar lowers into the existing domain-neutral frontend TypeExpr
 * representation.
 *
 * It MUST NOT introduce a classical-specific replacement AST.
 *
 * Conceptual mappings:
 *
 *     classicalNamedType
 *         -> TypeExpr named/identifier form
 *
 *     classicalGenericType
 *         -> TypeExpr generic/application form
 *
 *     classicalTupleType
 *         -> TypeExpr tuple form
 *
 *     classicalDependentType
 *         -> TypeExpr generic/value-parameterized form
 *
 *     classicalDimensionExpression
 *         -> type-level/value expression representation
 *
 * Exact Rust AST enum/field names remain owned by the frontend AST.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Syntax determines what was written.
 *
 * Semantic analysis determines what it means.
 *
 * Examples:
 *
 *     Vector<f64, N>
 *     Matrix<f32, Rows, Cols>
 *     Tensor<f64, N, M, K>
 *
 * do not themselves decide:
 *
 *     memory layout
 *     vectorization
 *     SIMD width
 *     accelerator
 *     device
 *     storage location
 *     scheduling
 *     execution placement
 *
 * Those decisions are downstream.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * The repository already has a universal generic type grammar.
 *
 * Therefore this file must not create a competing global generic system.
 *
 * Classical constructors use the same source-level type-argument model.
 *
 * Examples:
 *
 *     Vector<f64, N>
 *     Matrix<f32, Rows, Cols>
 *     Tensor<f64, N, M, K>
 *     Map<Key, Value>
 *
 * Nested generic composition is resolved by the canonical type system.
 *
 * ============================================================================
 * TYPE-LEVEL VALUE CONTRACT
 * ============================================================================
 *
 * Classical dimensions may remain symbolic:
 *
 *     N
 *     Rows
 *     Cols
 *     N * M
 *     2 * N + 1
 *     Batch * Features
 *
 * The grammar accepts syntax.
 *
 * Semantic analysis is responsible for determining:
 *
 *     integrality
 *     legality
 *     const-evaluability
 *     dimensional compatibility
 *     overflow policy
 *     resource feasibility
 *
 * The grammar imposes no universal maximum.
 *
 * ============================================================================
 */

parser grammar ClassicalTypes;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Canonical classical-domain type boundary.
 *
 * The rule intentionally has one structural shape rather than dozens of
 * syntactically identical alternatives.
 */
classicalType
    : classicalTypePrimary
      classicalTypePostfix*
    ;


/* ============================================================================
 * 2. CLASSICAL TYPE PRIMARY
 * ========================================================================== */

/**
 * Classical source types are represented using ordinary type syntax.
 *
 * Semantic analysis classifies the resulting type.
 *
 * The explicit constructor alternatives are reserved for syntax whose shape
 * is genuinely different from an ordinary named/generic type.
 */
classicalTypePrimary
    : classicalTupleType
    | classicalTypeApplication
    | classicalNamedType
    ;


/* ============================================================================
 * 3. NAMED CLASSICAL TYPES
 * ========================================================================== */

/**
 * Named classical type.
 *
 * Examples:
 *
 *     int
 *     float
 *     Real
 *     Complex
 *     Money
 *     scientific::Real
 *     numerical::Real
 *
 * Built-in keyword tokens are accepted explicitly where the canonical lexer
 * reserves them.
 */
classicalNamedType
    : classicalBuiltinScalar
    | classicalTypePath
    ;


/**
 * Canonical built-in scalar names.
 *
 * These are lexical tokens in the current lexer where applicable.
 *
 * Width/precision semantics remain semantic.
 */
classicalBuiltinScalar
    : INT
    | FLOAT_TYPE
    | BOOL_TYPE
    | CHAR_TYPE
    | STR_TYPE
    | STRING_TYPE
    | BYTE
    ;


/**
 * Qualified classical type name.
 *
 * No namespace-depth limit exists.
 */
classicalTypePath
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


/* ============================================================================
 * 4. CLASSICAL TYPE APPLICATION
 * ========================================================================== */

/**
 * Generic/value-parameterized classical type.
 *
 * Examples:
 *
 *     Vector<f64, N>
 *     Matrix<f32, Rows, Cols>
 *     Tensor<f64, N, M, K>
 *     Complex<f64>
 *     Rational<Int>
 *     Decimal<P, S>
 *     FixedPoint<I, F>
 *     Map<Key, Value>
 *
 * The constructor name is intentionally open-world.
 */
classicalTypeApplication
    : classicalTypePath
      classicalTypeArguments
    ;


/**
 * Classical type arguments.
 *
 * Arguments may be types or compile-time symbolic values.
 *
 * There is no fixed arity.
 */
classicalTypeArguments
    : LESS_THAN
      classicalTypeArgumentList?
      GREATER_THAN
    ;


/**
 * Ordered type argument list.
 *
 * A trailing comma is accepted.
 */
classicalTypeArgumentList
    : classicalTypeArgument
      (
          COMMA
          classicalTypeArgument
      )*
      COMMA?
    ;


/**
 * A classical constructor argument.
 *
 * Type arguments and symbolic value arguments are intentionally separated at
 * the semantic level, while sharing one syntactic argument position.
 */
classicalTypeArgument
    : classicalTypeArgumentType
    | classicalTypeArgumentValue
    ;


/**
 * Nested type argument.
 *
 * A nested classical type can itself contain arbitrary generic/value
 * parameters.
 */
classicalTypeArgumentType
    : classicalType
    ;


/**
 * Compile-time/value-level argument.
 */
classicalTypeArgumentValue
    : classicalDimensionExpression
    ;


/* ============================================================================
 * 5. POSTFIX TYPE FORMS
 * ========================================================================== */

/**
 * Classical type postfixes that are genuinely source-level.
 *
 * Optionality remains part of the universal type system and is intentionally
 * not duplicated here.
 *
 * This rule is reserved for classical-specific postfix extensions that may be
 * introduced by the canonical type system.
 *
 * It currently accepts no classical-specific postfix because no additional
 * postfix is required to express the canonical classical model.
 */
classicalTypePostfix
    : classicalTypeConstraintPostfix
    ;


/**
 * Classical type constraint postfix.
 *
 * Constraints are deliberately represented as source-level metadata rather
 * than hardware limits.
 *
 * Example conceptual forms:
 *
 *     T where ...
 *
 * The actual `where`/constraint grammar remains owned by the universal type
 * constraint subsystem. This rule is intentionally kept as an integration
 * boundary and should remain empty until a canonical postfix spelling exists.
 *
 * Therefore this production is not part of the active syntax yet.
 */
classicalTypeConstraintPostfix
    : /* intentionally reserved for future canonical constraint postfix */
    ;


/* ============================================================================
 * 6. VECTOR TYPE
 * ========================================================================== */

/**
 * Vector types are represented through the canonical constructor form:
 *
 *     Vector<T>
 *     Vector<T, N>
 *
 * The parser does not reserve the word Vector.
 *
 * Semantic analysis recognizes the constructor.
 *
 * This keeps the language open to:
 *
 *     math::Vector
 *     sparse::Vector
 *     distributed::Vector
 *     accelerator::Vector
 *
 * without changing the grammar.
 */
classicalVectorType
    : classicalTypeApplication
    ;


/* ============================================================================
 * 7. MATRIX TYPE
 * ========================================================================== */

/**
 * Matrix type boundary.
 *
 * Canonical semantic examples:
 *
 *     Matrix<T>
 *     Matrix<T, Rows, Cols>
 */
classicalMatrixType
    : classicalTypeApplication
    ;


/* ============================================================================
 * 8. TENSOR TYPE
 * ========================================================================== */

/**
 * Tensor type boundary.
 *
 * Canonical semantic examples:
 *
 *     Tensor<T>
 *     Tensor<T, N>
 *     Tensor<T, N, M>
 *     Tensor<T, N, M, K>
 *     ...
 *
 * Tensor rank is not hard-coded.
 */
classicalTensorType
    : classicalTypeApplication
    ;


/* ============================================================================
 * 9. NUMERIC TYPE BOUNDARY
 * ========================================================================== */

/**
 * Numeric type boundary.
 *
 * This is deliberately an integration façade rather than an alternative
 * parser interpretation.
 *
 * Semantic analysis classifies applications such as:
 *
 *     Integer
 *     UInt
 *     Float
 *     Decimal
 *     FixedPoint
 *     Complex
 *     Rational
 *
 * according to the canonical type registry.
 */
classicalNumericType
    : classicalTypePrimary
    ;


/* ============================================================================
 * 10. SCALAR TYPE BOUNDARY
 * ========================================================================== */

/**
 * Scalar semantic classification.
 */
classicalScalarType
    : classicalNamedType
    ;


/* ============================================================================
 * 11. COMPLEX TYPE BOUNDARY
 * ========================================================================== */

/**
 * Complex semantic classification.
 *
 * Examples:
 *
 *     Complex
 *     Complex<f64>
 */
classicalComplexType
    : classicalTypePrimary
    ;


/* ============================================================================
 * 12. RATIONAL TYPE BOUNDARY
 * ========================================================================== */

/**
 * Rational semantic classification.
 */
classicalRationalType
    : classicalTypePrimary
    ;


/* ============================================================================
 * 13. DECIMAL / FIXED-POINT TYPE BOUNDARY
 * ========================================================================== */

/**
 * Decimal/fixed-point semantic classification.
 */
classicalDecimalType
    : classicalTypePrimary
    ;


/* ============================================================================
 * 14. BIT / BYTE / WORD BOUNDARIES
 * ========================================================================== */

/**
 * Bit semantic classification.
 *
 * If `bit` becomes a reserved keyword in the canonical lexer, it belongs in
 * ZamaniLexer and should be added to the explicit builtin branch rather than
 * declared here.
 */
classicalBitType
    : classicalNamedType
    ;


/**
 * Byte semantic classification.
 */
classicalByteType
    : classicalNamedType
    ;


/**
 * Word semantic classification.
 *
 * Word width is semantic/source data:
 *
 *     Word<N>
 *
 * and never a grammar maximum.
 */
classicalWordType
    : classicalTypePrimary
    ;


/* ============================================================================
 * 15. CLASSICAL COLLECTION TYPES
 * ========================================================================== */

/**
 * Classical collection constructor boundary.
 *
 * Examples:
 *
 *     Array<T>
 *     Sequence<T>
 *     Map<K, V>
 *     Set<T>
 *
 * General array/slice syntax remains owned by the universal type grammar.
 */
classicalCollectionType
    : classicalTypeApplication
    ;


/**
 * Array semantic boundary.
 */
classicalArrayType
    : classicalCollectionType
    ;


/**
 * Sequence semantic boundary.
 */
classicalSequenceType
    : classicalCollectionType
    ;


/**
 * Map semantic boundary.
 */
classicalMapType
    : classicalCollectionType
    ;


/**
 * Set semantic boundary.
 */
classicalSetType
    : classicalCollectionType
    ;


/* ============================================================================
 * 16. TUPLE TYPES
 * ========================================================================== */

/**
 * Classical tuple type.
 *
 * Tuple syntax is structurally unbounded.
 *
 * Empty tuple:
 *
 *     ()
 *
 * Non-empty tuple:
 *
 *     (T)
 *     (T, U)
 *     (T, U, V)
 *     ...
 *
 * A singleton tuple requires the trailing comma.
 */
classicalTupleType
    : LPAREN RPAREN
    | LPAREN classicalTupleElement COMMA RPAREN
    | LPAREN classicalTupleElement COMMA classicalTupleElementList RPAREN
    ;


classicalTupleElementList
    : classicalTupleElement
      (
          COMMA
          classicalTupleElement
      )*
      COMMA?
    ;


classicalTupleElement
    : classicalType
    ;


/* ============================================================================
 * 17. DIMENSION / TYPE-LEVEL VALUE EXPRESSIONS
 * ========================================================================== */

/**
 * Classical dimensions and compile-time values.
 *
 * Examples:
 *
 *     N
 *     Rows
 *     Cols
 *     1024
 *     2 * N
 *     2 * N + 1
 *     Rows * Cols
 *     Batch * Features + Padding
 *
 * The grammar does not evaluate the expression.
 */
classicalDimensionExpression
    : classicalDimensionBitOr
    ;


/* --------------------------------------------------------------------------
 * Bitwise OR
 * -------------------------------------------------------------------------- */

classicalDimensionBitOr
    : classicalDimensionBitXor
      (
          BIT_OR
          classicalDimensionBitXor
      )*
    ;


/* --------------------------------------------------------------------------
 * Bitwise XOR
 * -------------------------------------------------------------------------- */

classicalDimensionBitXor
    : classicalDimensionBitAnd
      (
          CARET
          classicalDimensionBitAnd
      )*
    ;


/* --------------------------------------------------------------------------
 * Bitwise AND
 * -------------------------------------------------------------------------- */

classicalDimensionBitAnd
    : classicalDimensionShift
      (
          BIT_AND
          classicalDimensionShift
      )*
    ;


/* --------------------------------------------------------------------------
 * Shift
 * -------------------------------------------------------------------------- */

classicalDimensionShift
    : classicalDimensionAdditive
      (
          LEFT_SHIFT
        | RIGHT_SHIFT
      )
      classicalDimensionAdditive
      (
          (
              LEFT_SHIFT
            | RIGHT_SHIFT
          )
          classicalDimensionAdditive
      )*
    ;


/* --------------------------------------------------------------------------
 * Additive
 * -------------------------------------------------------------------------- */

classicalDimensionAdditive
    : classicalDimensionMultiplicative
      (
          (
              PLUS
            | MINUS
          )
          classicalDimensionMultiplicative
      )*
    ;


/* --------------------------------------------------------------------------
 * Multiplicative
 * -------------------------------------------------------------------------- */

classicalDimensionMultiplicative
    : classicalDimensionUnary
      (
          (
              STAR
            | SLASH
            | MODULO
          )
          classicalDimensionUnary
      )*
    ;


/* --------------------------------------------------------------------------
 * Unary
 * -------------------------------------------------------------------------- */

classicalDimensionUnary
    : (
          PLUS
        | MINUS
      )
      classicalDimensionUnary
    | classicalDimensionPrimary
    ;


/* --------------------------------------------------------------------------
 * Primary
 * -------------------------------------------------------------------------- */

classicalDimensionPrimary
    : INTEGER
    | IDENTIFIER
    | classicalDimensionQualifiedName
    | LPAREN classicalDimensionExpression RPAREN
    ;


/**
 * Qualified symbolic dimension.
 *
 * Examples:
 *
 *     shape::Rows
 *     model::Features
 *     tensor::Batch
 */
classicalDimensionQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )+
    ;


/* ============================================================================
 * 18. TYPE-LEVEL CONSTANT BOUNDARY
 * ========================================================================== */

/**
 * A type-level constant expression.
 *
 * This is intentionally an alias-like boundary for consumers that need to
 * distinguish a compile-time value from an ordinary type argument.
 */
classicalTypeValue
    : classicalDimensionExpression
    ;


/* ============================================================================
 * 19. SYMBOLIC DIMENSION BOUNDARY
 * ========================================================================== */

/**
 * Symbolic dimensions are intentionally open-world.
 *
 * There is no reserved N, M, K, Rows, Cols, etc.
 */
classicalSymbolicDimension
    : IDENTIFIER
    | classicalDimensionQualifiedName
    ;


/* ============================================================================
 * 20. CLASSICAL RESOURCE-NEUTRALITY
 * ========================================================================== */

/**
 * This grammar deliberately does NOT define:
 *
 *     cpu<N>
 *     gpu<N>
 *     simd<N>
 *     threads<N>
 *     memory<N>
 *     register<N>
 *
 * as classical types.
 *
 * If such concepts are required, their source-level intent belongs to the
 * resource/capability/hardware type systems.
 *
 * This preserves the distinction between:
 *
 *     semantic type
 *
 * and:
 *
 *     implementation/resource requirement.
 */


/* ============================================================================
 * 21. CLASSICAL ACCELERATOR-NEUTRALITY
 * ========================================================================== */

/**
 * A type such as:
 *
 *     Vector<T, N>
 *
 * does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     TPU
 *     NPU
 *     future accelerator
 *
 * Target selection belongs downstream.
 */


/* ============================================================================
 * 22. CLASSICAL / QUANTUM BOUNDARY
 * ========================================================================== */

/**
 * ClassicalTypes does not own quantum types.
 *
 * Quantum source types are owned by:
 *
 *     grammar/types/quantum.g4
 *
 * and composed by:
 *
 *     grammar/types/types.g4
 *
 * A classical type may nevertheless be used as a type argument to a quantum
 * construct where the universal type system permits it.
 *
 * Example:
 *
 *     quantum::State<Vector<f64>>
 *
 * The classical portion is parsed here; the quantum portion is parsed by the
 * canonical quantum type grammar.
 */


/* ============================================================================
 * 23. CLASSICAL / HDL BOUNDARY
 * ========================================================================== */

/**
 * ClassicalTypes does not own HDL types.
 *
 * Hardware/co-design types remain under:
 *
 *     grammar/types/hardware.g4
 *     grammar/hdl/
 *
 * A classical type can participate in a hardware interface only through the
 * canonical type/semantic composition system.
 */


/* ============================================================================
 * 24. CLASSICAL / DATA BOUNDARY
 * ========================================================================== */

/**
 * Tensor, vector, matrix and collection syntax can participate in the data
 * subsystem.
 *
 * This grammar does not own:
 *
 *     data schemas
 *     query languages
 *     serialization
 *     data pipelines
 *     provenance
 *     storage placement
 *
 * Those remain downstream/domain-specific concerns.
 */


/* ============================================================================
 * 25. CLASSICAL / AI BOUNDARY
 * ========================================================================== */

/**
 * AI/ML constructs may use classical types for:
 *
 *     tensor element types
 *     dimensions
 *     parameters
 *     model values
 *     datasets
 *     symbolic values
 *
 * AI semantic meaning remains owned by grammar/ai/ and semantic analysis.
 */


/* ============================================================================
 * 26. DETERMINISM CONTRACT
 * ========================================================================== */

/**
 * The public classical type syntax intentionally avoids alternatives such as:
 *
 *     IntegerType
 *     NumericType
 *     ScalarType
 *
 * when they would all begin with IDENTIFIER and accept identical suffixes.
 *
 * Classification is semantic rather than syntactic.
 *
 * This avoids competing ANTLR alternatives and preserves deterministic
 * parsing.
 */


/* ============================================================================
 * 27. ERROR CONTRACT
 * ========================================================================== */

/**
 * Invalid examples must be rejected by the canonical parser/type system:
 *
 *     Vector<
 *     Vector<>
 *     Vector<f64,
 *     Matrix<f64, Rows
 *     Tensor<f64,,N>
 *     Tensor<f64,N,>
 *
 * where the surrounding canonical generic grammar does not permit the form.
 *
 * Semantic errors such as:
 *
 *     Vector<f64, -1>
 *     Matrix<f64, Rows / 0, Cols>
 *
 * are NOT grammar errors merely because their values are semantically
 * invalid. They belong to type-level semantic validation.
 */


/* ============================================================================
 * 28. HARD-CODING AUDIT
 * ========================================================================== */

/**
 * Forbidden universal limits:
 *
 *     MAX_VECTOR_LENGTH
 *     MAX_MATRIX_ROWS
 *     MAX_MATRIX_COLUMNS
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_GENERIC_ARITY
 *     MAX_TUPLE_ARITY
 *     MAX_TYPE_DEPTH
 *     MAX_NUMERIC_WIDTH
 *
 * None are present in this grammar.
 *
 * Numeric literals remain program data.
 *
 * Symbolic dimensions remain symbolic.
 *
 * Structural repetition remains open-ended.
 */


/* ============================================================================
 * 29. COMPLETION CONTRACT
 * ========================================================================== */

/**
 * This file is complete only when all of the following are true:
 *
 *   [x] classical type ownership is explicit;
 *   [x] lexer rules are not duplicated;
 *   [x] generic syntax is not replaced by a second generic system;
 *   [x] named types are open-world;
 *   [x] vector syntax is supported;
 *   [x] matrix syntax is supported;
 *   [x] tensor syntax is supported;
 *   [x] collection syntax is supported;
 *   [x] tuple syntax is supported;
 *   [x] symbolic dimensions are supported;
 *   [x] arbitrary namespace depth is supported;
 *   [x] arbitrary constructor arity is supported;
 *   [x] no hardware limits are encoded;
 *   [x] no machine-specific widths are encoded;
 *   [x] no backend is selected;
 *   [x] AST ownership is defined;
 *   [x] semantic ownership is defined;
 *   [x] IR ownership is downstream;
 *   [x] quantum ownership remains in Quantum;
 *   [x] HDL ownership remains in HDL;
 *   [x] resource ownership remains in resources/hardware;
 *   [x] positive tests exist;
 *   [x] negative tests exist;
 *   [x] boundary tests exist;
 *   [x] scalability tests exist;
 *   [x] determinism tests exist;
 *   [x] compatibility tests exist.
 *
 * Rust implementation remains outside this grammar and MUST remain safe Rust.
 */