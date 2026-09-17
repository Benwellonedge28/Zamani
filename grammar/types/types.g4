/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/types.g4
 *
 * Role:
 *     CANONICAL MODULAR SOURCE-TYPE COMPOSITION GRAMMAR
 *
 * Grammar:
 *     Types
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file is the single public parser composition boundary for source-level
 * Zamani type syntax.
 *
 * It owns:
 *
 *   - typeExpression;
 *   - composition of source-level type forms;
 *   - named and qualified types;
 *   - generic type application;
 *   - primitive types;
 *   - tuple types;
 *   - array and slice syntax;
 *   - function types;
 *   - reference types;
 *   - pointer types;
 *   - optional postfix syntax;
 *   - Result types;
 *   - quantum source types;
 *   - temporal MTS types;
 *   - dependent/value-parameterized type syntax;
 *   - type-level value expressions;
 *   - parenthesized types;
 *   - source-level lifetime syntax.
 *
 * It does NOT own:
 *
 *   - lexer definitions;
 *   - keywords;
 *   - identifiers;
 *   - punctuation spelling;
 *   - operator spelling;
 *   - name resolution;
 *   - type inference;
 *   - unification;
 *   - generic substitution;
 *   - trait/interface resolution;
 *   - ownership checking;
 *   - borrow checking;
 *   - resource discovery;
 *   - hardware discovery;
 *   - physical qubit allocation;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - optimization;
 *   - backend selection;
 *   - runtime representation;
 *   - ABI layout.
 *
 * ============================================================================
 *
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     typeExpression
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
 *          +--------------------+---------------------+
 *          |                    |                     |
 *          v                    v                     v
 *      classical           quantum::ir          HDL/resource
 *       semantics           semantics             semantics
 *          |                    |                     |
 *          +--------------------+---------------------+
 *                               |
 *                               v
 *                         canonical IR
 *                               |
 *                    optimization / lowering
 *                               |
 *                    routing / scheduling
 *                               |
 *                         resilience / QEC
 *                               |
 *                             ZQN
 *                               |
 *                             HAL
 *                               |
 *                       target realization
 *
 * `TypeExpr` is the source-level AST boundary.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 *
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar contains NO implementation limits.
 *
 * It does not define:
 *
 *   MAX_QUBITS
 *   MAX_CPUS
 *   MAX_CORES
 *   MAX_THREADS
 *   MAX_GPUS
 *   MAX_FPGAS
 *   MAX_QPUS
 *   MAX_NODES
 *   MAX_DEVICES
 *   MAX_MEMORY
 *   MAX_REGISTER_WIDTH
 *   MAX_REGISTER_COUNT
 *   MAX_TENSOR_RANK
 *   MAX_TENSOR_DIMENSION
 *   MAX_ARRAY_LENGTH
 *   MAX_TUPLE_ARITY
 *   MAX_GENERIC_ARITY
 *   MAX_FUNCTION_PARAMETER_COUNT
 *   MAX_TYPE_DEPTH
 *   MAX_RESOURCE_COUNT
 *
 * Recursive grammar constructs are intentionally not bounded by language
 * constants.
 *
 * Practical parser/compiler limits, if required for hostile-input protection,
 * belong to explicit implementation policy and MUST NOT become language
 * semantics.
 *
 * ============================================================================
 *
 * PORTABILITY
 * ============================================================================
 *
 * A type describes source-level meaning.
 *
 * It must not select:
 *
 *   - CPU;
 *   - GPU;
 *   - FPGA;
 *   - ASIC;
 *   - QPU;
 *   - physical qubit;
 *   - memory bank;
 *   - network node;
 *   - accelerator instance;
 *   - vendor ABI.
 *
 * Target realization is downstream.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar lowers structurally into the existing frontend TypeExpr.
 *
 * Representative mappings:
 *
 *   namedType        -> TypeExpr::Identifier
 *   genericType      -> TypeExpr::Generic
 *   tupleType        -> TypeExpr::Tuple
 *   arrayType        -> TypeExpr::Array
 *   sliceType        -> TypeExpr::Slice
 *   functionType     -> TypeExpr::Function
 *   referenceType    -> TypeExpr::Reference
 *   pointerType      -> TypeExpr::Pointer
 *   optionalType     -> TypeExpr::Optional
 *   resultType       -> TypeExpr::Result
 *   neverType        -> TypeExpr::Never
 *   unitType         -> TypeExpr::Unit
 *   quantumType      -> TypeExpr::Quantum
 *   temporalType     -> TypeExpr::Temporal
 *
 * TypeValueExpr remains the source-level representation for symbolic
 * cardinality/value expressions where the existing AST already supports it.
 *
 * The grammar MUST NOT introduce a second TypeExpr representation.
 *
 * ============================================================================
 *
 * LEXER CONTRACT
 * ============================================================================
 *
 * All lexical tokens come from the canonical Zamani lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * whose vocabulary is assembled from:
 *
 *     grammar/lexer/
 *
 * This parser grammar therefore declares NO lexer rules.
 *
 * ============================================================================
 */

parser grammar Types;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC TYPE ENTRY POINT
 * ========================================================================== */

/**
 * Canonical source-level type expression.
 *
 * This is the ONLY public composition entry point for type syntax.
 */
typeExpression
    : typeQualifier* typeCore typePostfix*
    ;


/* ============================================================================
 * 2. TYPE QUALIFIERS
 * ========================================================================== */

/**
 * Source-level ownership/resource qualifiers.
 *
 * These are syntactic markers only.
 *
 * Their legality is determined by semantic analysis.
 */
typeQualifier
    : LINEAR
    | AFFINE
    ;


/* ============================================================================
 * 3. TYPE CORE
 * ========================================================================== */

/**
 * All canonical source-level type categories.
 *
 * Extensible domain types remain representable through namedType and
 * genericType rather than requiring a closed keyword inventory.
 */
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
    | resultType
    | quantumType
    | temporalType
    | dependentType
    | parenthesizedType
    ;


/* ============================================================================
 * 4. TYPE POSTFIXES
 * ========================================================================== */

/**
 * Postfix constructors.
 *
 * `T?` is the canonical compact optional spelling.
 */
typePostfix
    : QUESTION_MARK
    ;


/* ============================================================================
 * 5. PRIMITIVE TYPES
 * ============================================================================
 *
 * Only spellings already present in the canonical lexical vocabulary are
 * reserved here.
 *
 * Width-specific types such as:
 *
 *     i8
 *     i16
 *     i32
 *     i64
 *     i128
 *     u8
 *     u16
 *     f32
 *     f64
 *
 * remain ordinary identifiers unless the language specification explicitly
 * reserves them later.
 *
 * This avoids turning the grammar into a finite machine-width inventory.
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
 * 6. UNIT
 * ========================================================================== */

unitType
    : LPAREN RPAREN
    ;


/* ============================================================================
 * 7. NEVER
 * ========================================================================== */

neverType
    : NEVER
    ;


/* ============================================================================
 * 8. NAMED TYPES
 * ========================================================================== */

/**
 * Examples:
 *
 *     User
 *     Int32
 *     LogicalQubit
 *     Tensor
 *     Resource
 *     Capability
 *     std::collections::Map
 *
 * Name resolution is downstream.
 */
namedType
    : typePath
    ;


typePath
    : typePathSegment (DOUBLE_COLON typePathSegment)*
    ;


typePathSegment
    : IDENTIFIER
    ;


/* ============================================================================
 * 9. GENERIC TYPES
 * ========================================================================== */

/**
 * Examples:
 *
 *     Vec<int>
 *     Map<string, int>
 *     Tensor<float, Shape>
 *     Resource<Qubit>
 *     Capability<quantum::measurement>
 *
 * Generic arity is not bounded by the grammar.
 *
 * IMPORTANT:
 *
 * Generic arguments are source TypeExpr values because that is the existing
 * frontend TypeExpr::Generic contract.
 *
 * Value-dependent cardinalities use dependentType/typeValueExpression instead
 * of introducing an ambiguous "type-or-value" generic argument production.
 */
genericType
    : typePath typeArguments
    ;


typeArguments
    : LESS_THAN genericArgumentList GREATER_THAN
    ;


genericArgumentList
    : typeExpression (COMMA typeExpression)* COMMA?
    ;


/* ============================================================================
 * 10. TUPLE TYPES
 * ========================================================================== */

/**
 * Examples:
 *
 *     ()
 *     (int,)
 *     (int, bool)
 *     (int, bool, string)
 *
 * `()` is handled by unitType.
 *
 * Tuple arity is unbounded by grammar.
 */
tupleType
    : LPAREN
      typeExpression
      COMMA
      tupleTypeTail?
      RPAREN
    ;


tupleTypeTail
    : typeExpression (COMMA typeExpression)* COMMA?
    ;


/* ============================================================================
 * 11. ARRAY TYPES
 * ========================================================================== */

/**
 * Canonical forms:
 *
 *     [T]
 *     [T; N]
 *
 * `[T]` is a slice.
 *
 * `[T; N]` is an explicitly sized array.
 *
 * N is a source-level symbolic value and is NOT converted by the grammar to
 * a machine-sized integer.
 */
arrayType
    : LBRACKET
      typeExpression
      SEMI
      typeValueExpression
      RBRACKET
    ;


/* ============================================================================
 * 12. SLICE TYPES
 * ========================================================================== */

sliceType
    : LBRACKET
      typeExpression
      RBRACKET
    ;


/* ============================================================================
 * 13. FUNCTION TYPES
 * ========================================================================== */

/**
 * Canonical form:
 *
 *     fn() -> R
 *     fn(T) -> R
 *     fn(T, U) -> R
 *
 * Parameter count is not bounded by grammar.
 */
functionType
    : FN
      LPAREN
      functionTypeParameters?
      RPAREN
      functionTypeReturn?
    ;


functionTypeParameters
    : typeExpression (COMMA typeExpression)* COMMA?
    ;


functionTypeReturn
    : THIN_ARROW typeExpression
    ;


/* ============================================================================
 * 14. REFERENCE TYPES
 * ========================================================================== */

/**
 * Canonical forms:
 *
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *
 * Lifetime validity belongs to semantic analysis.
 */
referenceType
    : AMPERSAND
      lifetimeAnnotation?
      MUT?
      typeExpression
    ;


lifetimeAnnotation
    : APOSTROPHE IDENTIFIER
    ;


/* ============================================================================
 * 15. POINTER TYPES
 * ========================================================================== */

/**
 * Canonical forms:
 *
 *     *T
 *     *mut T
 *
 * Pointer width/address representation is downstream.
 */
pointerType
    : STAR
      MUT?
      typeExpression
    ;


/* ============================================================================
 * 16. RESULT TYPES
 * ========================================================================== */

/**
 * Canonical source form:
 *
 *     Result<T, E>
 *
 * `Result` is already a canonical lexical keyword.
 */
resultType
    : RESULT
      LESS_THAN
      typeExpression
      COMMA
      typeExpression
      GREATER_THAN
    ;


/* ============================================================================
 * 17. QUANTUM TYPES
 * ========================================================================== */

/**
 * The lexer reserves `Qubit` as a language-level quantum type name.
 *
 * Other quantum type names remain extensible ordinary identifiers/generic
 * types, for example:
 *
 *     LogicalQubit
 *     QRegister<N>
 *     QuantumState<T>
 *     QuantumResource<T>
 *
 * This deliberately avoids adding a closed hardware-specific quantum type
 * vocabulary.
 */
quantumType
    : QUBIT
    ;


/* ============================================================================
 * 18. TEMPORAL / MTS TYPES
 * ========================================================================== */

/**
 * Canonical temporal type:
 *
 *     MTS<T>
 *
 * The repository already has a dedicated temporal grammar component whose
 * independent lexical/parser boundary is the MTS constructor.
 *
 * This composition rule owns the complete type form.
 */
temporalType
    : MTS
      LESS_THAN
      typeExpression
      GREATER_THAN
    ;


/* ============================================================================
 * 19. DEPENDENT / VALUE-PARAMETERIZED TYPES
 * ========================================================================== */

/**
 * Canonical value-parameterized form:
 *
 *     Matrix<T>[Rows, Cols]
 *     Vector<T>[N]
 *     Tensor<T>[N, M, K]
 *
 * The base is a source type path and the dimensions are source-level values.
 *
 * This is deliberately open-ended and domain-neutral.
 */
dependentType
    : typePath
      LBRACKET
      typeValueExpression
      (COMMA typeValueExpression)*
      RBRACKET
    ;


/* ============================================================================
 * 20. TYPE-LEVEL VALUE EXPRESSIONS
 * ========================================================================== */

/**
 * Type-level values are structural source expressions.
 *
 * They are never evaluated by the grammar.
 *
 * Examples:
 *
 *     N
 *     Rows
 *     1024
 *     Rows * Cols
 *     2 * N
 *     N + Offset
 *
 * The resulting semantic TypeValueExpr remains symbolic until the semantic
 * and compilation layers decide whether evaluation is required.
 */
typeValueExpression
    : typeValueBitwiseOr
    ;


typeValueBitwiseOr
    : typeValueBitwiseAnd
      (PIPE typeValueBitwiseAnd)*
    ;


typeValueBitwiseAnd
    : typeValueShift
      (AMPERSAND typeValueShift)*
    ;


typeValueShift
    : typeValueAdditive
      ((LEFT_SHIFT | RIGHT_SHIFT) typeValueAdditive)*
    ;


typeValueAdditive
    : typeValueMultiplicative
      ((PLUS | MINUS) typeValueMultiplicative)*
    ;


typeValueMultiplicative
    : typeValueUnary
      ((STAR | SLASH | MODULO) typeValueUnary)*
    ;


typeValueUnary
    : (PLUS | MINUS | CARET | TILDE)*
      typeValuePrimary
    ;


typeValuePrimary
    : INTEGER
    | FLOAT
    | IDENTIFIER
    | qualifiedTypeValuePath
    | parenthesizedTypeValue
    ;


qualifiedTypeValuePath
    : IDENTIFIER
      DOUBLE_COLON
      IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


parenthesizedTypeValue
    : LPAREN
      typeValueExpression
      RPAREN
    ;


/* ============================================================================
 * 21. PARENTHESIZED TYPES
 * ========================================================================== */

parenthesizedType
    : LPAREN
      typeExpression
      RPAREN
    ;


/* ============================================================================
 * 22. STRUCTURAL CONTRACT
 * ============================================================================
 *
 * The parser preserves:
 *
 *   - source ordering;
 *   - type nesting;
 *   - generic argument ordering;
 *   - tuple ordering;
 *   - dependent-value ordering;
 *   - qualified-name ordering;
 *   - lifetime spelling;
 *   - source spans through the parser/AST integration layer.
 *
 * The parser does NOT:
 *
 *   - resolve aliases;
 *   - resolve generic parameters;
 *   - check trait bounds;
 *   - infer types;
 *   - check ownership;
 *   - check borrow validity;
 *   - allocate resources;
 *   - allocate qubits;
 *   - select hardware;
 *   - select a backend.
 *
 * ============================================================================
 *
 * DOMAIN INTEGRATION
 * ============================================================================
 *
 * Classical:
 *
 *     int
 *     float
 *     Tensor<T>[N, M]
 *     Matrix<T>[Rows, Cols]
 *
 * Quantum:
 *
 *     Qubit
 *     QuantumState
 *     LogicalQubit
 *     QRegister<N>
 *
 * HDL:
 *
 *     Signal<T>
 *     Register<T>
 *     Bus<T>
 *
 * Hardware/resource:
 *
 *     Resource<T>
 *     Capability<T>
 *     Memory<T>[N]
 *     Accelerator<T>
 *
 * Distributed:
 *
 *     Node<T>
 *     Channel<T>
 *     DistributedState<T>
 *
 * AI/data:
 *
 *     Tensor<T>[...]
 *     Dataset<T>
 *     Model<T>
 *
 * These remain source-level type abstractions. None of them imply physical
 * resources or machine-specific implementation.
 *
 * ============================================================================
 *
 * QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * Quantum type syntax does not create a quantum IR.
 *
 * The required lowering is:
 *
 *     source type
 *         |
 *         v
 *     frontend TypeExpr
 *         |
 *         v
 *     semantic quantum type
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     routing
 *         |
 *         v
 *     scheduling
 *         |
 *         v
 *     QEC / resilience
 *         |
 *         v
 *     ZQN
 *         |
 *         v
 *     HAL
 *         |
 *         v
 *     target
 *
 * No `QuantumTypeIR`, `PhysicalQubitIR`, or `GateIR` is introduced here.
 *
 * ============================================================================
 *
 * HARDWARE / RESOURCE SEPARATION
 * ============================================================================
 *
 * Valid source-level type:
 *
 *     Resource<QuantumResource>
 *
 * does NOT mean:
 *
 *     allocate physical device 0
 *
 * and:
 *
 *     Qubit
 *
 * does NOT mean:
 *
 *     physical qubit 17
 *
 * Hardware capability, topology, placement, routing, calibration, scheduling
 * and deployment remain downstream.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * The grammar is deterministic for a fixed token sequence and language
 * vocabulary.
 *
 * It contains no semantic predicates, actions, I/O, runtime calls or
 * implementation-specific decisions.
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *   - performs no I/O;
 *   - executes no source code;
 *   - evaluates no type values;
 *   - accesses no hardware;
 *   - requires no unsafe Rust;
 *   - performs no allocation based on source values.
 *
 * Hostile-input limits belong to explicit compiler/parser policy.
 *
 * ============================================================================
 *
 * CONFORMANCE EXAMPLES
 * ============================================================================
 *
 * VALID:
 *
 *     int
 *     float
 *     bool
 *     string
 *     char
 *     User
 *     module::User
 *     Vec<int>
 *     Map<string, int>
 *     (int, bool)
 *     (int,)
 *     ()
 *     [int]
 *     [int; N]
 *     fn(int) -> bool
 *     fn(int, string) -> Result
 *     &T
 *     &mut T
 *     &'a T
 *     &'a mut T
 *     *T
 *     *mut T
 *     Result<T, E>
 *     Qubit
 *     Qubit?
 *     Quantum<Q>
 *     MTS<int>
 *     MTS<QuantumState>
 *     Resource<Qubit>
 *     Capability<quantum::measurement>
 *     Matrix<T>[Rows, Cols]
 *     Tensor<T>[N, M, K]
 *
 * INVALID:
 *
 *     Result<T>
 *     Result<T, E, F>
 *     [T;]
 *     [T; N
 *     [T
 *     fn( -> T
 *     fn(T,) -> T
 *     &' T
 *     MTS<>
 *
 * BOUNDARY / SCALABILITY:
 *
 *     Vec<Vec<Vec<T>>>
 *     A::B::C::D::E
 *     Tuple-like types with arbitrarily many elements
 *     Functions with arbitrarily many parameters
 *     Tensor<T>[N1, N2, N3, ...]
 *     Qubit
 *     Qubit<N>
 *     Resource<Capability<T>>
 *
 * Tests must not introduce artificial maxima.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] There is one public typeExpression entry point.
 *   [x] It consumes the canonical ZamaniLexer vocabulary.
 *   [x] It does not define lexer rules.
 *   [x] Existing primitive keyword spellings are reused.
 *   [x] No nonexistent primitive tokens are referenced.
 *   [x] Named types remain open-ended.
 *   [x] Qualified names are unbounded by grammar.
 *   [x] Generic types are open-ended.
 *   [x] Tuple arity is unbounded by grammar.
 *   [x] Arrays support symbolic cardinalities.
 *   [x] Slices are syntactically distinct from sized arrays.
 *   [x] Function parameter count is unbounded by grammar.
 *   [x] References support lifetime and mutability syntax.
 *   [x] Pointer representation remains downstream.
 *   [x] Optionality uses one canonical postfix syntax.
 *   [x] Result uses the existing Result keyword.
 *   [x] Quantum syntax remains target-independent.
 *   [x] Temporal syntax uses the existing MTS vocabulary.
 *   [x] Resource/capability types remain extensible identifiers.
 *   [x] Dependent values remain symbolic.
 *   [x] No hardware limit is encoded.
 *   [x] No physical qubit mapping is encoded.
 *   [x] No second quantum IR is introduced.
 *   [x] No semantic evaluation occurs in the grammar.
 *   [x] No embedded Rust code occurs in the grammar.
 *   [x] No unsafe Rust is required.
 *
 * ============================================================================
 */