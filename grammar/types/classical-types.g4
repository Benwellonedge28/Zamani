/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/classical-types.g4
 *
 * Status:
 *     Production-ready classical-domain type syntax.
 *
 * Language:
 *     Zamani
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL CLASSICAL TYPE SYNTAX.
 *
 * It provides syntax for classical computational abstractions including:
 *
 *     scalar values
 *     signed/unsigned numeric families
 *     floating-point families
 *     arbitrary-precision numeric families
 *     decimal/fixed-point families
 *     complex values
 *     rational values
 *     bit/byte/word abstractions
 *     vectors
 *     matrices
 *     tensors
 *     symbolic/numerical arrays
 *     numeric containers
 *     classical accelerator-facing values
 *     compile-time symbolic dimensions
 *     value-parameterized classical types
 *
 * The grammar describes SOURCE SEMANTICS and COMPUTATIONAL INTENT.
 *
 * It does NOT describe:
 *
 *     CPU registers
 *     SIMD register widths
 *     GPU lanes
 *     GPU thread counts
 *     cache sizes
 *     physical memory
 *     device addresses
 *     accelerator IDs
 *     NUMA topology
 *     machine-specific vector widths
 *     machine-specific tensor limits
 *     physical storage layout
 *     ABI layout
 *     execution placement
 *     scheduling
 *     optimization
 *     hardware selection
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
 *          +---- Types.g4
 *          |
 *          +---- PrimitiveTypes.g4
 *          |
 *          +---- ClassicalTypes.g4
 *          |
 *          v
 *     frontend TypeExpr
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic type resolution
 *          |
 *          +---- classical semantic types
 *          |
 *          v
 *     canonical IR
 *          |
 *          +---- classical IR
 *          +---- tensor/data IR
 *          +---- accelerator metadata
 *          +---- resource metadata
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     scheduling / placement / lowering
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - classical-domain type syntax;
 *     - classical scalar-family syntax;
 *     - numeric-family constructor syntax;
 *     - vector type syntax;
 *     - matrix type syntax;
 *     - tensor type syntax;
 *     - symbolic-dimension syntax;
 *     - value-parameterized classical types;
 *     - classical container type syntax;
 *     - classical numeric-domain composition;
 *     - classical type modifiers that are genuinely source-level.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifier spelling;
 *     - primitive lexer rules;
 *     - generic type syntax globally;
 *     - type inference;
 *     - name resolution;
 *     - overload resolution;
 *     - numeric promotion;
 *     - numerical algorithms;
 *     - matrix algorithms;
 *     - tensor algorithms;
 *     - memory allocation;
 *     - hardware selection;
 *     - accelerator selection;
 *     - SIMD lowering;
 *     - GPU lowering;
 *     - CPU lowering;
 *     - classical IR;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - runtime representation.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Classical types describe WHAT a program means.
 *
 * They do not prescribe WHERE or HOW the value executes.
 *
 * For example:
 *
 *     Vector<f64, N>
 *
 * does not mean:
 *
 *     N CPU registers
 *     N SIMD lanes
 *     one particular GPU allocation
 *     one particular memory region
 *
 * Likewise:
 *
 *     Matrix<f32, Rows, Cols>
 *
 * does not prescribe:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     accelerator
 *     memory layout
 *     tiling
 *     thread count
 *     execution topology
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately NO grammar-level maximums for:
 *
 *     dimensions
 *     rank
 *     vector length
 *     matrix dimensions
 *     tensor dimensions
 *     generic arity
 *     parameter count
 *     nesting depth
 *     numeric precision
 *     numeric width
 *     container size
 *
 * A source program may therefore describe:
 *
 *     Vector<T, N>
 *     Matrix<T, Rows, Cols>
 *     Tensor<T, D0, D1, ...>
 *
 * without embedding a finite machine limit.
 *
 * Any implementation/resource limit must be represented by:
 *
 *     compiler policy
 *     semantic validation policy
 *     resource constraints
 *     capability negotiation
 *     target description
 *     runtime availability
 *
 * and MUST NOT be silently encoded here.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * It consumes the canonical Zamani lexer.
 *
 * It MUST NOT declare lexer rules.
 *
 * Classical constructor names are intentionally represented through the
 * canonical identifier token where they are not reserved lexical keywords.
 *
 * This permits future extensions without continually modifying the lexer.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * A construct such as:
 *
 *     Vector<f64, N>
 *
 * is syntactically a classical vector type.
 *
 * Semantic analysis decides:
 *
 *     - whether Vector is the canonical built-in;
 *     - whether N is a legal symbolic dimension;
 *     - whether f64 is a legal element type;
 *     - whether the resulting type is numeric;
 *     - whether operations are permitted;
 *     - whether the target can realize it;
 *     - whether resources are sufficient.
 *
 * The parser does not make those decisions.
 *
 * ============================================================================
 */

parser grammar ClassicalTypes;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the primary integration point consumed by Types.g4.
 *
 * ============================================================================
 */

classicalType
    : classicalScalarType
    | classicalNumericType
    | classicalComplexType
    | classicalRationalType
    | classicalDecimalType
    | classicalBitType
    | classicalByteType
    | classicalWordType
    | classicalVectorType
    | classicalMatrixType
    | classicalTensorType
    | classicalArrayType
    | classicalSequenceType
    | classicalMapType
    | classicalSetType
    | classicalTupleType
    | classicalNamedType
    ;


/* ============================================================================
 * 2. SCALAR CLASSICAL TYPES
 * ============================================================================
 *
 * Scalar forms deliberately use identifiers where the lexer does not reserve
 * the corresponding spelling.
 *
 * Semantic analysis owns canonical-name recognition.
 *
 * ============================================================================
 */

classicalScalarType
    : classicalTypePath
    ;


/* ============================================================================
 * 3. NUMERIC CLASS
 * ============================================================================
 *
 * Canonical semantic families may include:
 *
 *     Integer
 *     SignedInteger
 *     UnsignedInteger
 *     Float
 *     Decimal
 *     FixedPoint
 *
 * These remain source-level constructors.
 *
 * ============================================================================
 */

classicalNumericType
    : numericTypeConstructor
      classicalTypeArguments?
    ;


numericTypeConstructor
    : classicalTypePath
    ;


/* ============================================================================
 * 4. COMPLEX TYPES
 * ============================================================================
 *
 * Examples:
 *
 *     Complex<f32>
 *     Complex<f64>
 *     Complex<Real>
 *
 * Precision is semantic.
 *
 * It does not imply a native hardware representation.
 *
 * ============================================================================
 */

classicalComplexType
    : complexTypeConstructor
      classicalTypeArguments?
    ;


complexTypeConstructor
    : classicalTypePath
    ;


/* ============================================================================
 * 5. RATIONAL TYPES
 * ============================================================================
 *
 * Examples:
 *
 *     Rational
 *     Rational<Int>
 *     Rational<Numerator, Denominator>
 *
 * Exactness and representation are semantic properties.
 *
 * ============================================================================
 */

classicalRationalType
    : rationalTypeConstructor
      classicalTypeArguments?
    ;


rationalTypeConstructor
    : classicalTypePath
    ;


/* ============================================================================
 * 6. DECIMAL / FIXED-POINT TYPES
 * ============================================================================
 *
 * Examples:
 *
 *     Decimal
 *     Decimal<P>
 *     Decimal<P, S>
 *
 *     FixedPoint<I, F>
 *
 * The grammar does not impose maximum precision or scale.
 *
 * ============================================================================
 */

classicalDecimalType
    : decimalTypeConstructor
      classicalTypeArguments?
    ;


decimalTypeConstructor
    : classicalTypePath
    ;


/* ============================================================================
 * 7. BIT TYPES
 * ============================================================================
 *
 * A bit is a semantic binary value.
 *
 * It is not defined as a particular hardware register.
 *
 * ============================================================================
 */

classicalBitType
    : bitTypeConstructor
      classicalTypeArguments?
    ;


bitTypeConstructor
    : classicalTypePath
    ;


/* ============================================================================
 * 8. BYTE TYPES
 * ============================================================================
 *
 * Byte is a source-level storage/data abstraction.
 *
 * Its physical representation belongs downstream.
 *
 * ============================================================================
 */

classicalByteType
    : byteTypeConstructor
      classicalTypeArguments?
    ;


byteTypeConstructor
    : classicalTypePath
    ;


/* ============================================================================
 * 9. WORD TYPES
 * ============================================================================
 *
 * Word<N> is intentionally symbolic.
 *
 * Examples:
 *
 *     Word<N>
 *     Word<Width>
 *
 * There is no fixed maximum width.
 *
 * ============================================================================
 */

classicalWordType
    : wordTypeConstructor
      classicalTypeArguments?
    ;


wordTypeConstructor
    : classicalTypePath
    ;


/* ============================================================================
 * 10. VECTOR TYPES
 * ============================================================================
 *
 * Canonical conceptual forms:
 *
 *     Vector<T>
 *     Vector<T, N>
 *
 * Examples:
 *
 *     Vector<f64>
 *     Vector<f64, N>
 *     Vector<Scalar, Size>
 *
 * N remains symbolic until semantic/compilation stages.
 *
 * ============================================================================
 */

classicalVectorType
    : vectorTypeConstructor
      classicalTypeArguments
    ;


vectorTypeConstructor
    : classicalTypePath
    ;


/* ============================================================================
 * 11. MATRIX TYPES
 * ============================================================================
 *
 * Canonical conceptual forms:
 *
 *     Matrix<T>
 *     Matrix<T, Rows, Cols>
 *
 * Dimensions are semantic expressions.
 *
 * No fixed matrix dimensions are permitted here.
 *
 * ============================================================================
 */

classicalMatrixType
    : matrixTypeConstructor
      classicalTypeArguments
    ;


matrixTypeConstructor
    : classicalTypePath
    ;


/* ============================================================================
 * 12. TENSOR TYPES
 * ============================================================================
 *
 * Canonical conceptual forms:
 *
 *     Tensor<T>
 *     Tensor<T, D0>
 *     Tensor<T, D0, D1>
 *     Tensor<T, D0, D1, D2>
 *     ...
 *
 * There is intentionally no fixed tensor rank.
 *
 * ============================================================================
 */

classicalTensorType
    : tensorTypeConstructor
      classicalTypeArguments
    ;


tensorTypeConstructor
    : classicalTypePath
    ;


/* ============================================================================
 * 13. CLASSICAL ARRAY TYPES
 * ============================================================================
 *
 * This is distinct from the general array grammar only at the semantic
 * classification level.
 *
 * Syntax:
 *
 *     Array<T>
 *     Array<T, N>
 *
 * The general [T] / [T; N] syntax remains owned by Types.g4.
 *
 * ============================================================================
 */

classicalArrayType
    : arrayTypeConstructor
      classicalTypeArguments
    ;


arrayTypeConstructor
    : classicalTypePath
    ;


/* ============================================================================
 * 14. SEQUENCE TYPES
 * ============================================================================
 *
 * Examples:
 *
 *     Sequence<T>
 *     Stream<T>
 *     Iterable<T>
 *
 * The grammar deliberately does not decide whether a sequence is:
 *
 *     lazy
 *     eager
 *     memory-backed
 *     distributed
 *     streamed
 *     accelerator-backed
 *
 * ============================================================================
 */

classicalSequenceType
    : sequenceTypeConstructor
      classicalTypeArguments
    ;


sequenceTypeConstructor
    : classicalTypePath
    ;


/* ============================================================================
 * 15. MAP TYPES
 * ============================================================================
 *
 * Classical associative containers:
 *
 *     Map<K, V>
 *
 *     Dictionary<K, V>
 *
 * Physical implementation is downstream.
 *
 * ============================================================================
 */

classicalMapType
    : mapTypeConstructor
      classicalTypeArguments
    ;


mapTypeConstructor
    : classicalTypePath
    ;


/* ============================================================================
 * 16. SET TYPES
 * ============================================================================
 */

classicalSetType
    : setTypeConstructor
      classicalTypeArguments
    ;


setTypeConstructor
    : classicalTypePath
    ;


/* ============================================================================
 * 17. TUPLE TYPES
 * ============================================================================
 *
 * This rule is provided as a classical-domain façade.
 *
 * Canonical tuple syntax remains owned by Types.g4.
 *
 * ============================================================================
 */

classicalTupleType
    : LPAREN
      classicalTupleElements?
      RPAREN
    ;


classicalTupleElements
    : typeLikeClassicalElement
      COMMA
      typeLikeClassicalElement*
      COMMA?
    ;


typeLikeClassicalElement
    : classicalType
    ;


/* ============================================================================
 * 18. NAMED CLASSICAL TYPES
 * ============================================================================
 *
 * User-defined classical types must remain possible.
 *
 * Examples:
 *
 *     MyNumber
 *     scientific::Real
 *     finance::Money
 *     simulation::State
 *
 * This rule intentionally does not resolve ownership.
 *
 * ============================================================================
 */

classicalNamedType
    : classicalTypePath
    ;


/* ============================================================================
 * 19. TYPE PATH
 * ============================================================================
 *
 * There is no maximum namespace depth.
 *
 * Examples:
 *
 *     Vector
 *     math::Vector
 *     numerical::linear_algebra::Matrix
 *     future::domain::numeric::Tensor
 *
 * ============================================================================
 */

classicalTypePath
    : IDENT
      (DOUBLE_COLON IDENT)*
    ;


/* ============================================================================
 * 20. CLASSICAL TYPE ARGUMENTS
 * ============================================================================
 *
 * Classical constructors can accept:
 *
 *     types
 *     symbolic dimensions
 *     compile-time values
 *
 * Examples:
 *
 *     Vector<f64, N>
 *     Matrix<f32, Rows, Cols>
 *     Tensor<f64, N, M, K>
 *
 * ============================================================================
 */

classicalTypeArguments
    : LESS_THAN
      classicalTypeArgumentList?
      GREATER_THAN
    ;


classicalTypeArgumentList
    : classicalTypeArgument
      (COMMA classicalTypeArgument)*
      COMMA?
    ;


classicalTypeArgument
    : classicalType
    | classicalValueArgument
    ;


/* ============================================================================
 * 21. CLASSICAL VALUE ARGUMENTS
 * ============================================================================
 *
 * These represent symbolic compile-time values.
 *
 * Examples:
 *
 *     N
 *     Rows
 *     Cols
 *     1024
 *     Rows * Cols
 *     N + M
 *
 * The parser does not evaluate them.
 *
 * ============================================================================
 */

classicalValueArgument
    : classicalValueExpression
    ;


classicalValueExpression
    : classicalValueUnary*
      classicalValuePrimary
      classicalValueBinaryPart*
    ;


classicalValueUnary
    : PLUS
    | MINUS
    ;


classicalValueBinaryPart
    : classicalValueOperator
      classicalValueUnary*
      classicalValuePrimary
    ;


classicalValueOperator
    : PLUS
    | MINUS
    | STAR
    | SLASH
    | MODULO
    | LEFT_SHIFT
    | RIGHT_SHIFT
    | BIT_AND
    | BIT_OR
    | CARET
    ;


classicalValuePrimary
    : INTEGER
    | FLOAT
    | IDENT
    | classicalQualifiedValuePath
    | classicalParenthesizedValue
    ;


classicalQualifiedValuePath
    : IDENT
      (DOUBLE_COLON IDENT)+
    ;


classicalParenthesizedValue
    : LPAREN
      classicalValueExpression
      RPAREN
    ;


/* ============================================================================
 * 22. NUMERIC DOMAIN TYPES
 * ============================================================================
 *
 * This rule gives semantic tooling a stable parser boundary for numerical
 * domains without closing the language to future numeric systems.
 *
 * Examples:
 *
 *     Integer
 *     SignedInteger
 *     UnsignedInteger
 *     Float
 *     Decimal
 *     FixedPoint
 *     Rational
 *     Complex
 *
 * The actual canonical vocabulary is resolved semantically.
 *
 * ============================================================================
 */

numericDomainType
    : classicalTypePath
      classicalTypeArguments?
    ;


/* ============================================================================
 * 23. LINEAR-ALGEBRA TYPES
 * ============================================================================
 *
 * These are syntax façades over the canonical classical type model.
 *
 * The grammar does not implement linear algebra.
 *
 * ============================================================================
 */

linearAlgebraType
    : vectorType
    | matrixType
    | tensorType
    ;


vectorType
    : vectorTypeConstructor
      classicalTypeArguments
    ;


matrixType
    : matrixTypeConstructor
      classicalTypeArguments
    ;


tensorType
    : tensorTypeConstructor
      classicalTypeArguments
    ;


/* ============================================================================
 * 24. SYMBOLIC DIMENSIONS
 * ============================================================================
 *
 * Dimensions may be:
 *
 *     literal
 *     named
 *     qualified
 *     arithmetic expression
 *     compile-time parameter
 *
 * They must not be converted to host-machine integers in the parser.
 *
 * ============================================================================
 */

symbolicDimension
    : classicalValueExpression
    ;


/* ============================================================================
 * 25. CLASSICAL TYPE PARAMETERS
 * ============================================================================
 *
 * A classical type may be parameterized by symbolic values.
 *
 * Examples:
 *
 *     Vector<T, N>
 *     Matrix<T, Rows, Cols>
 *
 * The grammar does not decide whether an argument is:
 *
 *     type
 *     dimension
 *     precision
 *     scale
 *     layout parameter
 *     semantic policy
 *
 * Semantic analysis decides this according to the constructor declaration.
 *
 * ============================================================================
 */

classicalParameterizedType
    : classicalTypePath
      classicalTypeArguments
    ;


/* ============================================================================
 * 26. CLASSICAL DATA TYPES
 * ============================================================================
 *
 * This façade allows semantic tooling to recognize common data-domain
 * constructors without embedding implementation details.
 *
 * ============================================================================
 */

classicalDataType
    : classicalMapType
    | classicalSetType
    | classicalSequenceType
    | classicalArrayType
    | classicalTupleType
    | classicalNamedType
    ;


/* ============================================================================
 * 27. CLASSICAL NUMERICAL TYPE
 * ============================================================================
 *
 * This intentionally accepts an open constructor vocabulary.
 *
 * Future forms can therefore be introduced without modifying the lexical
 * grammar.
 *
 * ============================================================================
 */

classicalNumericalType
    : numericDomainType
    | linearAlgebraType
    ;


/* ============================================================================
 * 28. CLASSICAL COMPUTATIONAL TYPE
 * ============================================================================
 *
 * Stable high-level façade for semantic analysis and AST tooling.
 *
 * ============================================================================
 */

classicalComputationalType
    : classicalNumericalType
    | classicalDataType
    | classicalParameterizedType
    ;


/* ============================================================================
 * 29. EXTENSIBLE CLASSICAL TYPE
 * ============================================================================
 *
 * Future classical computational domains must remain expressible without
 * changing this grammar merely because a new mathematical or computational
 * abstraction is introduced.
 *
 * Examples:
 *
 *     automatic_differentiation::Dual<T>
 *     interval::Interval<T>
 *     sparse::Vector<T, N>
 *     polynomial::Polynomial<T, Degree>
 *     symbolic::Expression<T>
 *     probability::Distribution<T>
 *
 * These are syntactically named/parameterized types.
 *
 * Semantic registration determines their meaning.
 *
 * ============================================================================
 */

extensibleClassicalType
    : classicalTypePath
    | classicalTypePath classicalTypeArguments
    ;


/* ============================================================================
 * 30. CLASSICAL TYPE EXPRESSION
 * ============================================================================
 *
 * Compatibility façade used by integrations that need to identify a
 * classical-domain expression without duplicating all alternatives.
 *
 * ============================================================================
 */

classicalTypeExpression
    : classicalType
    | classicalComputationalType
    | extensibleClassicalType
    ;


/* ============================================================================
 * 31. SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The parser produces structure only.
 *
 * The semantic/type layer is responsible for recognizing canonical
 * constructors.
 *
 * Conceptual mappings include:
 *
 *     Vector<T, N>
 *         -> canonical classical vector semantic type
 *
 *     Matrix<T, R, C>
 *         -> canonical classical matrix semantic type
 *
 *     Tensor<T, D0, ...>
 *         -> canonical classical tensor semantic type
 *
 *     Complex<T>
 *         -> canonical complex semantic type
 *
 *     Rational<T>
 *         -> canonical rational semantic type
 *
 *     Decimal<P, S>
 *         -> canonical decimal semantic type
 *
 *     FixedPoint<I, F>
 *         -> canonical fixed-point semantic type
 *
 *     Map<K, V>
 *         -> canonical map semantic type
 *
 *     Set<T>
 *         -> canonical set semantic type
 *
 *     Sequence<T>
 *         -> canonical sequence semantic type
 *
 * No semantic representation is defined in this grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 32. FRONTEND AST CONTRACT
 * ============================================================================
 *
 * The frontend TypeExpr representation already provides source-level
 * structures for named/generic/composite types.
 *
 * ClassicalTypes.g4 MUST therefore lower into those existing structures
 * rather than introducing a second ClassicalType AST.
 *
 * Conceptual lowering:
 *
 *     classicalTypePath
 *         -> TypeExpr::Identifier
 *
 *     classicalTypePath + arguments
 *         -> TypeExpr::Generic
 *
 *     Vector<T, N>
 *         -> TypeExpr::Generic
 *
 *     Matrix<T, R, C>
 *         -> TypeExpr::Generic
 *
 *     Tensor<T, D0, D1, ...>
 *         -> TypeExpr::Generic
 *
 * Value arguments that are not ordinary type expressions must be represented
 * through the canonical source-level type-value/dependent-type representation
 * owned by the frontend AST.
 *
 * This grammar MUST NOT create:
 *
 *     ClassicalType
 *     ClassicalIR
 *     MatrixIR
 *     TensorIR
 *
 * or equivalent duplicate semantic structures.
 *
 * ============================================================================
 */


/* ============================================================================
 * 33. CLASSICAL IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT import classical IR.
 *
 * The dependency direction is:
 *
 *     grammar
 *         ->
 *     frontend AST
 *         ->
 *     semantic classical types
 *         ->
 *     classical IR
 *
 * Never:
 *
 *     classical IR
 *         ->
 *     grammar
 *
 * This prevents a grammar/IR dependency cycle.
 *
 * ============================================================================
 */


/* ============================================================================
 * 34. QUANTUM INTEGRATION CONTRACT
 * ============================================================================
 *
 * Classical types may appear in quantum programs.
 *
 * Examples include:
 *
 *     measurement result types
 *     classical control values
 *     parameterized gate values
 *     loop/control expressions
 *     hybrid algorithm state
 *
 * This file MUST NOT define:
 *
 *     Qubit
 *     QuantumState
 *     QuantumGate
 *     Measurement
 *     QEC
 *     ZQN
 *
 * Those belong to the quantum grammar and semantic layers.
 *
 * ClassicalTypes.g4 is consumed by hybrid/quantum syntax through the common
 * type-expression boundary.
 *
 * ============================================================================
 */


/* ============================================================================
 * 35. HDL INTEGRATION CONTRACT
 * ============================================================================
 *
 * Classical types may appear in HDL/software co-design constructs for:
 *
 *     ports
 *     parameters
 *     control values
 *     configuration values
 *     algorithmic expressions
 *
 * However this grammar MUST NOT encode:
 *
 *     FPGA fabric size
 *     LUT count
 *     DSP count
 *     BRAM count
 *     clock frequency
 *     physical pin number
 *     device package
 *
 * HDL semantics own those concepts.
 *
 * ============================================================================
 */


/* ============================================================================
 * 36. HARDWARE INTEGRATION CONTRACT
 * ============================================================================
 *
 * A classical type can express computational requirements but must not select
 * hardware.
 *
 * For example:
 *
 *     Tensor<f64, N, M>
 *
 * may eventually lower to:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     distributed implementation
 *
 * according to downstream capabilities and policies.
 *
 * ============================================================================
 */


/* ============================================================================
 * 37. ACCELERATOR INTEGRATION CONTRACT
 * ============================================================================
 *
 * Types such as:
 *
 *     Vector<T, N>
 *     Matrix<T, R, C>
 *     Tensor<T, ...>
 *
 * are semantic data abstractions.
 *
 * They must not imply:
 *
 *     SIMD
 *     CUDA
 *     ROCm
 *     TPU
 *     NPU
 *     FPGA
 *
 * Backend lowering may select an accelerator when capabilities permit.
 *
 * ============================================================================
 */


/* ============================================================================
 * 38. RESOURCE INTEGRATION CONTRACT
 * ============================================================================
 *
 * Type syntax describes data and computational semantics.
 *
 * Resource requirements belong to the resource/capability grammar.
 *
 * Therefore:
 *
 *     Tensor<f64, N, M>
 *
 * MUST NOT itself mean:
 *
 *     allocate GPU
 *
 *     allocate N threads
 *
 *     reserve N cores
 *
 *     allocate fixed memory
 *
 * Resource analysis may derive resource requirements from the type and
 * operations, but those requirements are semantic/compiler metadata rather
 * than grammar-level machine assignments.
 *
 * ============================================================================
 */


/* ============================================================================
 * 39. DETERMINISM
 * ============================================================================
 *
 * The grammar preserves source ordering.
 *
 * No unordered structure is introduced here.
 *
 * Repeated type arguments retain their lexical order.
 *
 * Symbolic expressions retain their syntactic structure.
 *
 * Parsing the same source with the same grammar and lexer configuration must
 * produce the same parse structure.
 *
 * ============================================================================
 */


/* ============================================================================
 * 40. COMPATIBILITY
 * ============================================================================
 *
 * This file must remain compatible with:
 *
 *     grammar/types/types.g4
 *     grammar/types/primitive-types.g4
 *     grammar/types/generic-types.g4
 *     grammar/types/array-types.g4
 *     grammar/types/tuple-types.g4
 *     grammar/types/map-types.g4
 *     grammar/types/function-types.g4
 *     grammar/types/resource-types.g4
 *     grammar/types/type-constraints.g4
 *
 * It must not redefine their canonical ownership.
 *
 * In particular:
 *
 *     primitive syntax
 *         -> PrimitiveTypes.g4
 *
 *     general type syntax
 *         -> Types.g4
 *
 *     generic syntax
 *         -> GenericTypes.g4
 *
 *     classical domain specialization
 *         -> ClassicalTypes.g4
 *
 * ============================================================================
 */


/* ============================================================================
 * 41. BACKWARD COMPATIBILITY
 * ============================================================================
 *
 * Existing source-level forms such as:
 *
 *     int
 *     float
 *     bool
 *     str
 *     string
 *     char
 *
 * remain owned by PrimitiveTypes.g4.
 *
 * Existing general forms such as:
 *
 *     Vec<T>
 *     Matrix<T, ...>
 *     Tensor<T, ...>
 *
 * can be represented as named/parameterized classical types without forcing
 * the lexer to reserve every constructor name.
 *
 * ============================================================================
 */


/* ============================================================================
 * 42. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_VECTOR_SIZE
 *     MAX_MATRIX_ROWS
 *     MAX_MATRIX_COLUMNS
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_NUMERIC_WIDTH
 *     MAX_NUMERIC_PRECISION
 *     MAX_GENERIC_ARITY
 *     MAX_TYPE_DEPTH
 *     MAX_CONTAINER_SIZE
 *     MAX_ACCELERATOR_SIZE
 *
 * No fixed machine-resource quantity is represented here.
 *
 * ============================================================================
 */


/* ============================================================================
 * 43. SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains no executable Rust.
 *
 * Compiler/frontend code consuming it MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and MUST use safe Rust.
 *
 * Required crate-level policy in Rust consumers:
 *
 *     #![forbid(unsafe_code)]
 *     #![deny(unsafe_op_in_unsafe_fn)]
 *
 * No unsafe implementation is required by this grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 44. SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing classical types must not:
 *
 *     execute expressions;
 *     evaluate arbitrary code;
 *     access files;
 *     access networks;
 *     allocate target resources;
 *     inspect hardware;
 *     query devices;
 *     invoke external programs.
 *
 * Symbolic dimension expressions are syntax only.
 *
 * ============================================================================
 */


/* ============================================================================
 * 45. ERROR-RECOVERY CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to parser diagnostics.
 *
 * Semantic errors such as:
 *
 *     invalid dimension
 *     invalid precision
 *     invalid type argument
 *     unsupported numeric domain
 *     unavailable capability
 *
 * belong downstream.
 *
 * The grammar must not turn semantic failures into parser failures merely
 * because a backend cannot realize a type.
 *
 * ============================================================================
 */


/* ============================================================================
 * 46. FUTURE EXTENSION CONTRACT
 * ============================================================================
 *
 * New classical domains must not require this file to be rewritten merely
 * because a new semantic constructor appears.
 *
 * Examples:
 *
 *     Interval<T>
 *     Dual<T>
 *     Polynomial<T, N>
 *     RationalFunction<T>
 *     SparseVector<T, N>
 *     SparseMatrix<T, R, C>
 *     DistributedTensor<T, ...>
 *     Symbolic<T>
 *     AutomaticDifferentiation<T>
 *     Probability<T>
 *     FixedPoint<I, F>
 *     Posit<P>
 *     BFloat<P>
 *     Decimal<P, S>
 *
 * These can enter through the open named/parameterized type mechanism.
 *
 * ============================================================================
 */


/* ============================================================================
 * 47. NO HARDWARE-LOCK-IN
 * ============================================================================
 *
 * This file must remain independent of:
 *
 *     x86
 *     ARM
 *     RISC-V
 *     GPU vendor
 *     FPGA vendor
 *     ASIC vendor
 *     TPU
 *     NPU
 *     quantum hardware
 *     cloud provider
 *     operating system
 *     ABI
 *
 * ============================================================================
 */


/* ============================================================================
 * 48. TEST CONTRACT
 * ============================================================================
 *
 * The grammar is complete only when tests cover at least:
 *
 * POSITIVE:
 *
 *     Vector<f64>
 *     Vector<f64, N>
 *     Matrix<f32, Rows, Cols>
 *     Tensor<f64, N, M, K>
 *     Complex<f64>
 *     Rational
 *     Decimal<P, S>
 *     FixedPoint<I, F>
 *     Map<String, Int>
 *     Set<Int>
 *     Sequence<Float>
 *     scientific::Real
 *     numerical::Matrix<f64, R, C>
 *
 * NEGATIVE:
 *
 *     malformed generic arguments
 *     missing dimensions
 *     malformed qualified names
 *     malformed arithmetic expressions
 *     unmatched delimiters
 *     empty constructor arguments where forbidden by the surrounding grammar
 *
 * SCALABILITY:
 *
 *     arbitrary symbolic vector size
 *     arbitrary symbolic matrix dimensions
 *     arbitrary tensor rank
 *     nested classical types
 *     deeply qualified classical types
 *     large generic argument lists
 *
 * CROSS-DOMAIN:
 *
 *     classical + quantum
 *     classical + HDL
 *     classical + hardware
 *     classical + distributed
 *     classical + AI
 *     classical + quantum + distributed
 *
 * DETERMINISM:
 *
 *     identical source -> identical parse structure
 *
 * ROUND TRIP:
 *
 *     source -> lexer -> parser -> AST -> printer -> parser
 *
 * must preserve intended type semantics.
 *
 * ============================================================================
 */


/* ============================================================================
 * 49. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 * [ ] It compiles as an ANTLR parser grammar.
 *
 * [ ] It consumes only canonical ZamaniLexer tokens.
 *
 * [ ] It does not define lexer rules.
 *
 * [ ] It does not define a second AST.
 *
 * [ ] It does not define a classical IR.
 *
 * [ ] It does not depend on quantum IR.
 *
 * [ ] It does not depend on hardware implementation.
 *
 * [ ] It does not contain fixed machine-resource limits.
 *
 * [ ] It supports symbolic dimensions.
 *
 * [ ] It supports arbitrarily extensible classical constructors.
 *
 * [ ] It integrates with Types.g4.
 *
 * [ ] It integrates with PrimitiveTypes.g4.
 *
 * [ ] It does not duplicate generic-type ownership.
 *
 * [ ] It does not force target-specific representation.
 *
 * [ ] It supports classical/quantum/hardware interoperability through the
 *     common TypeExpr boundary.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Round-trip tests pass.
 *
 * [ ] Rust consumers compile on Rust 1.97 and Rust 1.97.1.
 *
 * [ ] Rust consumers contain no unsafe code.
 *
 * ============================================================================
 */