/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/antlr/Types.g4
 *
 * Role:
 *     Canonical source-level type grammar for Zamani.
 *
 * Architecture:
 *
 *     source
 *        |
 *        v
 *     ZamaniLexer
 *        |
 *        v
 *     ZamaniParser
 *        |
 *        +---- imports Types.g4
 *        |
 *        v
 *     frontend AST
 *        |
 *        v
 *     structural validation
 *        |
 *        v
 *     semantic type resolution
 *        |
 *        v
 *     canonical IR / ZUIR
 *        |
 *        v
 *     target-independent lowering
 *        |
 *        v
 *     target realization
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust safety:
 *     No unsafe Rust.
 *
 * ============================================================================
 *
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * This grammar defines TYPE SYNTAX ONLY.
 *
 * It does not:
 *
 *   - resolve names;
 *   - infer types;
 *   - perform generic substitution;
 *   - perform trait resolution;
 *   - perform ownership analysis;
 *   - perform borrow checking;
 *   - allocate memory;
 *   - allocate registers;
 *   - allocate qubits;
 *   - select hardware;
 *   - select a backend;
 *   - select a CPU/GPU/QPU;
 *   - select a quantum topology;
 *   - select a gate set;
 *   - determine ABI layout;
 *   - determine pointer width;
 *   - determine integer width;
 *   - determine array storage;
 *   - determine runtime representation.
 *
 * Those responsibilities belong to semantic analysis, resource analysis,
 * canonical IR, optimization, lowering and target realization.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Types describe portable computational intent.
 *
 * They MUST NOT contain language-level assumptions such as:
 *
 *     MAX_TYPE_DEPTH
 *     MAX_GENERIC_ARITY
 *     MAX_ARRAY_LENGTH
 *     MAX_TUPLE_ARITY
 *     MAX_FUNCTION_PARAMETERS
 *     MAX_QUBITS
 *     MAX_REGISTER_COUNT
 *     MAX_MEMORY
 *
 * Any implementation safety limit must be an explicit compiler policy.
 *
 * The grammar itself contains no artificial finite resource limit.
 *
 * ============================================================================
 *
 * TYPE MODEL
 * ============================================================================
 *
 * The canonical frontend representation is TypeExpr.
 *
 * The source grammar maps to concepts including:
 *
 *     Primitive
 *     Named
 *     Generic
 *     Tuple
 *     Array
 *     Slice
 *     Function
 *     Reference
 *     Pointer
 *     Optional
 *     Result
 *     Never
 *     Unit
 *     Linear
 *     Affine
 *     Temporal
 *     Quantum
 *     dependent/value-parameterized forms
 *
 * The grammar intentionally does not enumerate every possible semantic type.
 *
 * User-defined and future types remain representable through names,
 * qualification and generic application.
 *
 * ============================================================================
 *
 * IMPORTANT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * All tokens come from ZamaniLexer.g4.
 *
 * The root parser should import this grammar:
 *
 *     import Types;
 *
 * The root parser remains responsible for deciding WHERE a type expression
 * may occur:
 *
 *     function parameters
 *     return types
 *     fields
 *     generic bounds
 *     aliases
 *     casts
 *     annotations
 *     declarations
 *
 * Types.g4 does not own those contexts.
 *
 * ============================================================================
 */

parser grammar Types;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. CANONICAL TYPE ENTRY POINT
 * ========================================================================== */

/**
 * TypeExpression is the sole public entry point for source-level types.
 *
 * Recursive structure is intentionally expressed through grammar recursion
 * rather than finite enumerations.
 */
typeExpression
    : typeQualifier* typeCore typePostfix*
    ;


/* ============================================================================
 * 2. TYPE QUALIFIERS
 *
 * Qualifiers modify the source-level type abstraction.
 *
 * Their legality and composition rules belong to semantic analysis.
 * ========================================================================== */

typeQualifier
    : LINEAR
    | AFFINE
    ;


/* ============================================================================
 * 3. TYPE CORE
 * ========================================================================== */

typeCore
    : primitiveType
    | unitType
    | neverType
    | namedType
    | genericType
    | tupleType
    | arrayType
    | sliceType
    | functionType
    | referenceType
    | pointerType
    | optionalType
    | resultType
    | quantumType
    | temporalType
    | dependentType
    | parenthesizedType
    ;


/* ============================================================================
 * 4. POSTFIX TYPE FORMS
 *
 * Postfix forms are deliberately extensible through semantic type
 * construction rather than a closed hardware-oriented vocabulary.
 * ========================================================================== */

typePostfix
    : optionalPostfix
    ;


/**
 * T?
 *
 * This is the compact optional-type spelling.
 *
 * The semantic layer decides whether it is equivalent to Optional<T>.
 */
optionalPostfix
    : QUESTION_MARK
    ;


/* ============================================================================
 * 5. PRIMITIVE TYPES
 *
 * IMPORTANT:
 *
 * The lexer currently exposes broad primitive keywords such as `int`,
 * `float`, `bool`, `str`, `string`, `char` and `void`.
 *
 * Width-specific integer/floating representations should remain available
 * through named or semantic type resolution unless and until the canonical
 * lexical specification reserves additional spellings.
 *
 * The grammar therefore does NOT invent a fixed machine-width inventory.
 * ========================================================================== */

primitiveType
    : VOID
    | INT
    | FLOAT_TYPE
    | BOOL_TYPE
    | STR_TYPE
    | STRING_TYPE
    | CHAR_TYPE
    ;


/* ============================================================================
 * 6. UNIT TYPE
 *
 * Canonical source spelling:
 *
 *     ()
 *
 * It maps to the frontend UnitType / TypeExpr::Unit representation.
 * ========================================================================== */

unitType
    : LPAREN RPAREN
    ;


/* ============================================================================
 * 7. NEVER TYPE
 *
 * Canonical lexical spelling:
 *
 *     Never
 *
 * This represents the uninhabited type.
 * ========================================================================== */

neverType
    : NEVER
    ;


/* ============================================================================
 * 8. NAMED TYPES
 *
 * Examples:
 *
 *     Int
 *     User
 *     quantum::State
 *     std::collections::Map
 *
 * Name resolution is deliberately outside the grammar.
 * ========================================================================== */

namedType
    : typePath
    ;


/**
 * Source-level type path.
 *
 * Qualification remains purely syntactic.
 *
 * The semantic resolver determines whether the path names:
 *
 *     - a primitive alias;
 *     - a user type;
 *     - a module type;
 *     - a generic constructor;
 *     - a type parameter;
 *     - a future domain type.
 */
typePath
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 9. GENERIC TYPES
 *
 * Examples:
 *
 *     Vec<Int>
 *     Map<String, Int>
 *     Quantum<Q>
 *     Result<Value, Error>
 *
 * No fixed generic arity is imposed.
 * ========================================================================== */

genericType
    : typePath
      LESS_THAN
      typeArgumentList
      GREATER_THAN
    ;


typeArgumentList
    : typeArgument
      (COMMA typeArgument)*
      COMMA?
    ;


typeArgument
    : typeExpression
    | typeValueArgument
    ;


/* ============================================================================
 * 10. TYPE-LEVEL VALUE ARGUMENTS
 *
 * These allow source-level symbolic cardinalities and dependent-style
 * parameters without forcing the parser to evaluate them.
 *
 * Examples:
 *
 *     Array<T, N>
 *     Tensor<T, N, M>
 *     Matrix<T, Rows, Cols>
 *
 * The parser preserves the expression structurally.
 *
 * Semantic analysis determines whether the value is legal as a type-level
 * argument.
 *
 * IMPORTANT:
 *
 * The grammar does not convert the value to a machine-sized integer.
 * ========================================================================== */

typeValueArgument
    : typeValueExpression
    ;


typeValueExpression
    : typeValuePrimary
      typeValueBinaryPart*
    ;


typeValueBinaryPart
    : typeValueOperator typeValuePrimary
    ;


typeValueOperator
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


typeValuePrimary
    : INTEGER
    | FLOAT
    | identifier
    | qualifiedTypeValueName
    | parenthesizedTypeValue
    ;


qualifiedTypeValueName
    : identifier
      (DOUBLE_COLON identifier)+
    ;


parenthesizedTypeValue
    : LPAREN typeValueExpression RPAREN
    ;


/* ============================================================================
 * 11. TUPLE TYPES
 *
 * Examples:
 *
 *     (Int, Bool)
 *     (Int, String, QuantumState)
 *
 * Unit:
 *
 *     ()
 *
 * is handled separately by unitType.
 *
 * No tuple-size ceiling is encoded.
 * ========================================================================== */

tupleType
    : LPAREN
      typeExpression
      COMMA
      typeTupleTail?
      RPAREN
    ;


typeTupleTail
    : typeExpression
      (COMMA typeExpression)*
      COMMA?
    ;


/* ============================================================================
 * 12. ARRAY TYPES
 *
 * Canonical source-level forms supported:
 *
 *     [T]
 *     [T; N]
 *
 * The element type is semantic.
 *
 * N may be:
 *
 *     - a literal;
 *     - a symbolic identifier;
 *     - a qualified symbolic name;
 *     - a compile-time expression.
 *
 * No machine-sized array limit is encoded.
 * ========================================================================== */

arrayType
    : LBRACKET
      typeExpression
      arrayLength?
      RBRACKET
    ;


arrayLength
    : SEMI typeValueExpression
    ;


/* ============================================================================
 * 13. SLICE TYPES
 *
 * Canonical source form:
 *
 *     [T]
 *
 * without an explicit length may semantically represent a slice depending on
 * the active type-system policy.
 *
 * A dedicated `slice<T>` form is also supported because it is unambiguous and
 * maps naturally to the existing SliceType façade.
 * ========================================================================== */

sliceType
    : identifier
      LESS_THAN
      typeExpression
      GREATER_THAN
    ;


/* ============================================================================
 * 14. FUNCTION TYPES
 *
 * Canonical function-type shape:
 *
 *     fn(T1, T2) -> R
 *
 * Parameter and return types remain source-level types.
 *
 * There is no fixed parameter count.
 * ========================================================================== */

functionType
    : FN
      LPAREN
      functionTypeParameters?
      RPAREN
      functionTypeReturn?
    ;


functionTypeParameters
    : typeExpression
      (COMMA typeExpression)*
      COMMA?
    ;


functionTypeReturn
    : THIN_ARROW
      typeExpression
    ;


/* ============================================================================
 * 15. REFERENCE TYPES
 *
 * Canonical forms:
 *
 *     &T
 *     &mut T
 *
 * Optional lifetime syntax may be added through lifetimeAnnotation.
 *
 * Reference validity is semantic.
 *
 * The grammar does not model a Rust-specific ownership system.
 * ========================================================================== */

referenceType
    : AMPERSAND
      lifetimeAnnotation?
      MUT?
      typeExpression
    ;


lifetimeAnnotation
    : APOSTROPHE identifier
    ;


/* ============================================================================
 * 16. POINTER TYPES
 *
 * Canonical source-level forms:
 *
 *     *T
 *     *mut T
 *
 * Pointer safety and representation are semantic/backend concerns.
 *
 * The grammar does not imply:
 *
 *     32-bit pointers
 *     64-bit pointers
 *     physical addresses
 *     virtual-address layouts
 * ========================================================================== */

pointerType
    : STAR
      MUT?
      typeExpression
    ;


/* ============================================================================
 * 17. EXPLICIT OPTIONAL TYPE
 *
 * Canonical generic form:
 *
 *     Optional<T>
 *
 * The lexer does not reserve `Optional`, so it is represented as a normal
 * generic type constructor through genericType.
 *
 * This rule exists as a named grammar façade for tools/documentation.
 * ========================================================================== */

optionalType
    : OPTIONAL_TYPE_NAME
      LESS_THAN
      typeExpression
      GREATER_THAN
    ;


/**
 * Contextual type constructor spelling.
 *
 * IMPORTANT:
 *
 * `Optional` is intentionally represented by IDENT rather than a new lexer
 * keyword. This prevents unnecessary lexical reservation.
 */
OPTIONAL_TYPE_NAME
    : identifier
    ;


/* ============================================================================
 * 18. RESULT TYPES
 *
 * Canonical generic form:
 *
 *     Result<T, E>
 *
 * Like Optional, Result is already represented lexically in the current
 * language design as RESULT (`Result`), so it receives an explicit production.
 * ========================================================================== */

resultType
    : RESULT
      LESS_THAN
      typeExpression
      COMMA
      typeExpression
      GREATER_THAN
    ;


/* ============================================================================
 * 19. QUANTUM TYPES
 *
 * Quantum type syntax is intentionally abstraction-oriented.
 *
 * The grammar does NOT enumerate:
 *
 *     Qubit0
 *     Qubit1
 *     physical qubit IDs
 *     hardware topology
 *     vendor gate sets
 *     fixed register widths
 *
 * `Qubit` remains a lexical type name in the current lexer.
 *
 * Parameterization allows future/general semantic quantum abstractions:
 *
 *     Qubit
 *     Quantum<Q>
 *     Quantum<State>
 *
 * without making hardware cardinality part of syntax.
 * ========================================================================== */

quantumType
    : QUBIT
    ;


/* ============================================================================
 * 20. TEMPORAL TYPES
 *
 * Temporal is a semantic type constructor.
 *
 * The lexer already reserves the `zamani`/temporal vocabulary, but the
 * canonical type constructor remains a named type so future temporal domains
 * do not require a closed grammar inventory.
 *
 * Example:
 *
 *     zamani::Instant
 *     zamani::Duration
 *
 * An explicit temporal wrapper is retained for AST compatibility.
 * ========================================================================== */

temporalType
    : ZAMANI
      LESS_THAN
      typeExpression
      GREATER_THAN
    ;


/* ============================================================================
 * 21. DEPENDENT TYPE FORMS
 *
 * The frontend type representation has a TypeValueExpr abstraction and the
 * language specification contemplates dependent/value-parameterized types.
 *
 * The grammar intentionally keeps the constructor open:
 *
 *     TypeName[Value]
 *
 * or:
 *
 *     TypeName<Value>
 *
 * The actual dependent-type legality belongs to semantic analysis.
 *
 * This rule does NOT attempt to implement dependent type theory in ANTLR.
 * ========================================================================== */

dependentType
    : typePath
      LBRACKET
      typeValueExpression
      (COMMA typeValueExpression)*
      RBRACKET
    ;


/* ============================================================================
 * 22. PARENTHESIZED TYPES
 *
 * Parentheses provide grouping for recursive type expressions:
 *
 *     (T)
 *     &(T)
 *     fn((A, B)) -> C
 *
 * Unit `()` is recognized separately.
 * ========================================================================== */

parenthesizedType
    : LPAREN
      typeExpression
      RPAREN
    ;


/* ============================================================================
 * 23. TYPE IDENTIFIERS
 *
 * All ordinary identifiers are delegated to the canonical lexer.
 *
 * This rule is intentionally small so the lexer remains the single lexical
 * authority.
 * ========================================================================== */

identifier
    : IDENT
    ;


/* ============================================================================
 * 24. STRUCTURAL TYPE HELPERS
 *
 * These named rules provide stable grammar-node boundaries for tooling.
 * They do not introduce additional semantic types.
 * ========================================================================== */

typeList
    : typeExpression
      (COMMA typeExpression)*
      COMMA?
    ;


typeBound
    : typeExpression
    ;


typeBoundList
    : typeBound
      (PLUS typeBound)*
    ;


/* ============================================================================
 * 25. TYPE-LEVEL QUALIFIED VALUES
 *
 * Example:
 *
 *     Matrix<T, math::Rows>
 *
 * Qualification is preserved for later semantic resolution.
 * ========================================================================== */

typeValuePath
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * END OF TYPES.G4
 * ============================================================================
 *
 * Ownership:
 *
 *     Types.g4
 *         -> source-level type syntax
 *
 * Does NOT own:
 *
 *     lexer
 *     declarations
 *     statements
 *     expressions
 *     semantic types
 *     type checking
 *     inference
 *     resource analysis
 *     quantum allocation
 *     hardware
 *     ABI
 *     backend
 *     runtime
 *
 * Integration:
 *
 *     ZamaniLexer.g4
 *         |
 *         v
 *     ZamaniParser.g4
 *         |
 *         +---- import Types;
 *         |
 *         v
 *     TypeExpression
 *         |
 *         v
 *     frontend::ast::node::types::TypeExpr
 *
 * ============================================================================
 */